use super::*;
use std::{
    io::{Cursor, Read},
    sync::Arc,
    thread,
    time::{Duration, Instant},
};

#[derive(Clone)]
struct FixtureTransport {
    key: String,
    payload: Vec<u8>,
    delay: Duration,
}

impl FixtureTransport {
    fn new(key: &str, payload: Vec<u8>, delay: Duration) -> Self {
        Self {
            key: key.to_string(),
            payload,
            delay,
        }
    }
}

impl DownloadTransport for FixtureTransport {
    fn key(&self) -> &str {
        &self.key
    }

    fn open(
        &self,
        _source: &DownloadSourceRef,
    ) -> Result<DownloadTransportStream, DownloadTransportFailure> {
        Ok(DownloadTransportStream {
            reader: Box::new(SlowReader {
                cursor: Cursor::new(self.payload.clone()),
                delay: self.delay,
                max_chunk: 32 * 1024,
            }),
            total_bytes: Some(self.payload.len() as u64),
        })
    }
}

struct SlowReader {
    cursor: Cursor<Vec<u8>>,
    delay: Duration,
    max_chunk: usize,
}

impl Read for SlowReader {
    fn read(&mut self, buffer: &mut [u8]) -> std::io::Result<usize> {
        if self.delay > Duration::ZERO {
            thread::sleep(self.delay);
        }
        let len = buffer.len().min(self.max_chunk);
        self.cursor.read(&mut buffer[..len])
    }
}

fn request(transport: &str, resource_id: String, name: &str, bytes: usize) -> DownloadRequest {
    DownloadRequest {
        source: DownloadSourceRef {
            transport: transport.to_string(),
            resource_id,
        },
        display_name: name.to_string(),
        destination_file_name: format!("{name}.mcpack"),
        expected_bytes: Some(bytes as u64),
    }
}

fn wait_for(
    runtime: &DownloadExecutionRuntime,
    predicate: impl Fn(&DownloadManagerSnapshot) -> bool,
) -> DownloadManagerSnapshot {
    let started = Instant::now();
    loop {
        let snapshot = runtime.snapshot().expect("snapshot");
        if predicate(&snapshot) {
            return snapshot;
        }
        assert!(
            started.elapsed() < Duration::from_secs(5),
            "download runtime condition timed out: {snapshot:?}"
        );
        thread::sleep(Duration::from_millis(10));
    }
}

fn fixture_runtime(
    root: &std::path::Path,
    policy: DownloadPolicy,
    transport: Option<FixtureTransport>,
) -> DownloadExecutionRuntime {
    let mut registry = DownloadTransportRegistry::new();
    if let Some(transport) = transport {
        registry.register(Arc::new(transport)).expect("transport");
    }
    DownloadExecutionRuntime::new(
        policy,
        DownloadStore::new(root.join("state.json")),
        root.join("workspace"),
        root.join("files"),
        registry,
    )
    .expect("runtime")
}

#[test]
fn local_file_transport_completes_end_to_end() {
    let directory = tempfile::tempdir().expect("tempdir");
    let source = directory.path().join("source.mcpack");
    let payload = vec![7_u8; 700_000];
    std::fs::write(&source, &payload).expect("source");

    let runtime = DownloadExecutionRuntime::new(
        DownloadPolicy::default(),
        DownloadStore::new(directory.path().join("state.json")),
        directory.path().join("workspace"),
        directory.path().join("files"),
        DownloadTransportRegistry::with_local_file().expect("registry"),
    )
    .expect("runtime");

    let job = runtime
        .queue(request(
            "local-file",
            source.to_string_lossy().into_owned(),
            "local-copy",
            payload.len(),
        ))
        .expect("queue");

    let snapshot = wait_for(&runtime, |snapshot| {
        snapshot.jobs.iter().any(|candidate| {
            candidate.id == job.id && candidate.state == DownloadJobState::Completed
        })
    });
    let completed = snapshot
        .jobs
        .iter()
        .find(|candidate| candidate.id == job.id)
        .expect("completed job");
    assert_eq!(completed.progress.downloaded_bytes, payload.len() as u64);
    assert_eq!(
        std::fs::read(directory.path().join("files/local-copy.mcpack")).expect("final payload"),
        payload
    );
}

#[test]
fn scheduler_respects_active_concurrency() {
    let directory = tempfile::tempdir().expect("tempdir");
    let payload = vec![3_u8; 512 * 1024];
    let runtime = fixture_runtime(
        directory.path(),
        DownloadPolicy {
            max_active: 2,
            max_jobs: 10,
        },
        Some(FixtureTransport::new(
            "slow",
            payload.clone(),
            Duration::from_millis(15),
        )),
    );

    for name in ["one", "two", "three"] {
        runtime
            .queue(request("slow", name.to_string(), name, payload.len()))
            .expect("queue");
    }

    wait_for(&runtime, |snapshot| {
        snapshot.active_jobs == 2 && snapshot.queued_jobs == 1
    });
    let finished = wait_for(&runtime, |snapshot| {
        snapshot
            .jobs
            .iter()
            .all(|job| job.state == DownloadJobState::Completed)
    });
    assert_eq!(finished.jobs.len(), 3);
}

#[test]
fn active_cancellation_is_cooperative() {
    let directory = tempfile::tempdir().expect("tempdir");
    let payload = vec![4_u8; 1024 * 1024];
    let runtime = fixture_runtime(
        directory.path(),
        DownloadPolicy::default(),
        Some(FixtureTransport::new(
            "slow-cancel",
            payload.clone(),
            Duration::from_millis(20),
        )),
    );

    let job = runtime
        .queue(request(
            "slow-cancel",
            "fixture".into(),
            "cancel-me",
            payload.len(),
        ))
        .expect("queue");

    wait_for(&runtime, |snapshot| {
        snapshot.jobs.iter().any(|candidate| {
            candidate.id == job.id && candidate.state == DownloadJobState::Transferring
        })
    });
    runtime.cancel(&job.id).expect("cancel");

    let snapshot = wait_for(&runtime, |snapshot| {
        snapshot.jobs.iter().any(|candidate| {
            candidate.id == job.id && candidate.state == DownloadJobState::Cancelled
        })
    });
    assert_eq!(
        snapshot
            .jobs
            .iter()
            .find(|candidate| candidate.id == job.id)
            .expect("cancelled")
            .state,
        DownloadJobState::Cancelled
    );
    assert!(!directory.path().join("files/cancel-me.mcpack").exists());
}

#[test]
fn unavailable_transport_fails_without_retry_loop() {
    let directory = tempfile::tempdir().expect("tempdir");
    let runtime = fixture_runtime(directory.path(), DownloadPolicy::default(), None);
    let job = runtime
        .queue(request("missing", "resource".into(), "missing", 10))
        .expect("queue");

    let snapshot = wait_for(&runtime, |snapshot| {
        snapshot
            .jobs
            .iter()
            .any(|candidate| candidate.id == job.id && candidate.state == DownloadJobState::Failed)
    });
    let failed = snapshot
        .jobs
        .iter()
        .find(|candidate| candidate.id == job.id)
        .expect("failed");
    assert_eq!(
        failed.last_error.as_ref().expect("failure").code,
        "download_transport_unavailable"
    );
    assert!(!failed.last_error.as_ref().expect("failure").retryable);
}
