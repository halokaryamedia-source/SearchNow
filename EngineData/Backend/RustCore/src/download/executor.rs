use super::{
    cleanup_workspace, ensure_workspace, finalize_payload, plan_workspace, prepare_payload_file,
    DownloadJob, DownloadJobState, DownloadManager, DownloadManagerSnapshot, DownloadPolicy,
    DownloadRequest, DownloadStore, DownloadTransportRegistry,
};
use crate::error::{BackendError, BackendResult};
use std::{
    io::{ErrorKind, Read, Write},
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
    thread,
};

const TRANSFER_BUFFER_BYTES: usize = 256 * 1024;
const PROGRESS_CHECKPOINT_BYTES: u64 = 1024 * 1024;

#[derive(Clone)]
pub struct DownloadExecutionRuntime {
    inner: Arc<DownloadExecutionInner>,
}

struct DownloadExecutionInner {
    manager: Mutex<DownloadManager>,
    store: DownloadStore,
    workspace_root: PathBuf,
    destination_root: PathBuf,
    transports: DownloadTransportRegistry,
}

impl DownloadExecutionRuntime {
    pub fn new(
        policy: DownloadPolicy,
        store: DownloadStore,
        workspace_root: impl Into<PathBuf>,
        destination_root: impl Into<PathBuf>,
        transports: DownloadTransportRegistry,
    ) -> BackendResult<Self> {
        let manager = DownloadManager::recover(policy, store.load()?)?;
        store.save(&manager.persisted_state())?;

        let runtime = Self {
            inner: Arc::new(DownloadExecutionInner {
                manager: Mutex::new(manager),
                store,
                workspace_root: workspace_root.into(),
                destination_root: destination_root.into(),
                transports,
            }),
        };
        runtime.pump()?;
        Ok(runtime)
    }

    pub fn snapshot(&self) -> BackendResult<DownloadManagerSnapshot> {
        let manager = self.lock_manager()?;
        Ok(manager.snapshot())
    }

    pub fn queue(&self, request: DownloadRequest) -> BackendResult<DownloadJob> {
        let job = self.mutate_persist(|manager| manager.enqueue(request))?;
        let _ = self.pump();
        Ok(job)
    }

    pub fn cancel(&self, job_id: &str) -> BackendResult<DownloadJob> {
        self.mutate_persist(|manager| manager.request_cancel(job_id))
    }

    pub fn retry(&self, job_id: &str) -> BackendResult<DownloadJob> {
        let job = self.mutate_persist(|manager| manager.retry(job_id))?;
        let _ = self.pump();
        Ok(job)
    }

    pub fn remove_terminal(&self, job_id: &str) -> BackendResult<DownloadManagerSnapshot> {
        self.mutate_persist(|manager| {
            manager.remove_terminal(job_id)?;
            Ok(manager.snapshot())
        })
    }

    pub fn pump(&self) -> BackendResult<usize> {
        let claimed = {
            let mut manager = self.lock_manager()?;
            let mut candidate = manager.clone();
            let claimed = candidate.claim_ready_jobs();
            if claimed.is_empty() {
                return Ok(0);
            }
            self.inner.store.save(&candidate.persisted_state())?;
            *manager = candidate;
            claimed
        };
        let count = claimed.len();

        for job in claimed {
            let runtime = self.clone();
            thread::spawn(move || {
                if let Err(error) = runtime.execute_claimed_job(&job.id) {
                    runtime.fail_from_backend_error(&job.id, error);
                }
                let _ = runtime.pump();
            });
        }
        Ok(count)
    }

    fn execute_claimed_job(&self, job_id: &str) -> BackendResult<()> {
        let job = self.current_job(job_id)?;
        let plan = plan_workspace(
            &self.inner.workspace_root,
            &self.inner.destination_root,
            &job.id,
            &job.destination_file_name,
        )?;

        if self.acknowledge_cancel_if_requested(job_id, &plan)? {
            return Ok(());
        }

        let mut stream = match self.inner.transports.open(&job.source) {
            Ok(stream) => stream,
            Err(failure) => {
                self.fail_job(
                    job_id,
                    &plan,
                    &failure.code,
                    &failure.message,
                    failure.retryable,
                )?;
                return Ok(());
            }
        };

        if self.acknowledge_cancel_if_requested(job_id, &plan)? {
            return Ok(());
        }

        ensure_workspace(&plan)?;
        let mut payload = prepare_payload_file(&plan)?;

        let transition = self.mutate_persist(|manager| {
            manager.mark_transferring(job_id)?;
            manager.report_progress(job_id, 0, stream.total_bytes)
        });
        if let Err(error) = transition {
            if self.acknowledge_cancel_if_requested(job_id, &plan)? {
                return Ok(());
            }
            self.fail_job(job_id, &plan, error.code(), error.message(), false)?;
            return Ok(());
        }

        let mut buffer = vec![0_u8; TRANSFER_BUFFER_BYTES];
        let mut downloaded = 0_u64;

        loop {
            if self.acknowledge_cancel_if_requested(job_id, &plan)? {
                return Ok(());
            }

            let read = match stream.reader.read(&mut buffer) {
                Ok(read) => read,
                Err(error) => {
                    let (code, retryable) = transfer_read_failure(error.kind());
                    self.fail_job(
                        job_id,
                        &plan,
                        code,
                        &format!("Download transport read failed: {error}"),
                        retryable,
                    )?;
                    return Ok(());
                }
            };
            if read == 0 {
                break;
            }

            if let Err(error) = payload.write_all(&buffer[..read]) {
                self.fail_job(
                    job_id,
                    &plan,
                    "download_payload_write_failed",
                    &format!("SearchNow could not write the download payload: {error}"),
                    true,
                )?;
                return Ok(());
            }

            downloaded = downloaded.saturating_add(read as u64);
            if let Err(error) = self.report_progress(job_id, downloaded, stream.total_bytes) {
                self.fail_job(job_id, &plan, error.code(), error.message(), false)?;
                return Ok(());
            }
        }

        if let Err(error) = payload.sync_all() {
            self.fail_job(
                job_id,
                &plan,
                "download_payload_sync_failed",
                &format!("SearchNow could not sync the completed download payload: {error}"),
                true,
            )?;
            return Ok(());
        }
        drop(payload);

        if stream
            .total_bytes
            .is_some_and(|total_bytes| downloaded != total_bytes)
        {
            self.fail_job(
                job_id,
                &plan,
                "download_transfer_incomplete",
                "Download transport ended before its declared byte count was received.",
                true,
            )?;
            return Ok(());
        }

        self.mutate_persist(|manager| manager.begin_finalizing(job_id))?;

        if let Err(error) = finalize_payload(&plan) {
            let retryable = error.code() != "download_destination_exists";
            self.fail_job(job_id, &plan, error.code(), error.message(), retryable)?;
            return Ok(());
        }

        self.mutate_persist(|manager| manager.mark_completed(job_id))?;
        let _ = cleanup_workspace(&plan);
        Ok(())
    }

    fn report_progress(
        &self,
        job_id: &str,
        downloaded_bytes: u64,
        total_bytes: Option<u64>,
    ) -> BackendResult<DownloadJob> {
        let mut manager = self.lock_manager()?;
        let previous = find_job(&manager.snapshot(), job_id)?
            .progress
            .downloaded_bytes;
        let mut candidate = manager.clone();
        let output = candidate.report_progress(job_id, downloaded_bytes, total_bytes)?;

        let crossed_checkpoint =
            previous / PROGRESS_CHECKPOINT_BYTES != downloaded_bytes / PROGRESS_CHECKPOINT_BYTES;
        let completed_declared_size = total_bytes.is_some_and(|total| downloaded_bytes == total);
        if crossed_checkpoint || completed_declared_size {
            self.inner.store.save(&candidate.persisted_state())?;
        }
        *manager = candidate;
        Ok(output)
    }

    fn acknowledge_cancel_if_requested(
        &self,
        job_id: &str,
        plan: &super::DownloadWorkspacePlan,
    ) -> BackendResult<bool> {
        if self.current_job(job_id)?.state != DownloadJobState::CancelRequested {
            return Ok(false);
        }

        self.mutate_persist(|manager| manager.acknowledge_cancel(job_id))?;
        let _ = cleanup_workspace(plan);
        Ok(true)
    }

    fn fail_job(
        &self,
        job_id: &str,
        plan: &super::DownloadWorkspacePlan,
        code: &str,
        message: &str,
        retryable: bool,
    ) -> BackendResult<()> {
        if self.acknowledge_cancel_if_requested(job_id, plan)? {
            return Ok(());
        }

        self.mutate_persist(|manager| {
            manager.mark_failed(job_id, code.to_string(), message.to_string(), retryable)
        })?;
        let _ = cleanup_workspace(plan);
        Ok(())
    }

    fn fail_from_backend_error(&self, job_id: &str, error: BackendError) {
        let Ok(job) = self.current_job(job_id) else {
            return;
        };
        if !job.state.is_active() {
            return;
        }

        let Ok(plan) = plan_workspace(
            &self.inner.workspace_root,
            &self.inner.destination_root,
            &job.id,
            &job.destination_file_name,
        ) else {
            return;
        };
        let _ = self.fail_job(job_id, &plan, error.code(), error.message(), true);
    }

    fn current_job(&self, job_id: &str) -> BackendResult<DownloadJob> {
        let manager = self.lock_manager()?;
        find_job(&manager.snapshot(), job_id)
    }

    fn mutate_persist<T>(
        &self,
        action: impl FnOnce(&mut DownloadManager) -> BackendResult<T>,
    ) -> BackendResult<T> {
        let mut manager = self.lock_manager()?;
        let mut candidate = manager.clone();
        let output = action(&mut candidate)?;
        self.inner.store.save(&candidate.persisted_state())?;
        *manager = candidate;
        Ok(output)
    }

    fn lock_manager(&self) -> BackendResult<std::sync::MutexGuard<'_, DownloadManager>> {
        self.inner.manager.lock().map_err(|_| {
            BackendError::new(
                "download_state_lock_failed",
                "Download manager state is unavailable.",
            )
        })
    }
}

fn transfer_read_failure(kind: ErrorKind) -> (&'static str, bool) {
    match kind {
        ErrorKind::TimedOut | ErrorKind::WouldBlock => ("download_transfer_timeout", true),
        ErrorKind::InvalidData => ("download_transfer_invalid_data", false),
        _ => ("download_transfer_read_failed", true),
    }
}

fn find_job(snapshot: &DownloadManagerSnapshot, job_id: &str) -> BackendResult<DownloadJob> {
    snapshot
        .jobs
        .iter()
        .find(|job| job.id == job_id)
        .cloned()
        .ok_or_else(|| BackendError::new("download_job_not_found", "Download job was not found."))
}

pub fn default_download_paths(app_data_root: &Path) -> (PathBuf, PathBuf, PathBuf) {
    let root = app_data_root.join("downloads");
    (
        root.join("state.json"),
        root.join("workspace"),
        root.join("files"),
    )
}
