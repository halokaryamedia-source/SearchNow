use super::*;
use std::fs;

fn request(name: &str) -> DownloadRequest {
    DownloadRequest {
        source: DownloadSourceRef {
            transport: "fixture".into(),
            resource_id: format!("resource-{name}"),
        },
        display_name: name.into(),
        destination_file_name: format!("{name}.mcpack"),
        expected_bytes: Some(10),
    }
}

#[test]
fn concurrency_claims_only_available_slots() {
    let mut manager = DownloadManager::new(DownloadPolicy {
        max_active: 2,
        max_jobs: 10,
    })
    .expect("manager");
    manager.enqueue(request("one")).expect("queue");
    manager.enqueue(request("two")).expect("queue");
    manager.enqueue(request("three")).expect("queue");
    let claimed = manager.claim_ready_jobs();
    assert_eq!(claimed.len(), 2);
    assert_eq!(manager.snapshot().active_jobs, 2);
    assert_eq!(manager.snapshot().queued_jobs, 1);
}

#[test]
fn progress_is_monotonic_and_bounded() {
    let mut manager = DownloadManager::new(DownloadPolicy::default()).expect("manager");
    let job = manager.enqueue(request("pack")).expect("queue");
    manager.claim_ready_jobs();
    manager.mark_transferring(&job.id).expect("transfer");
    manager
        .report_progress(&job.id, 5, Some(10))
        .expect("progress");
    let error = manager
        .report_progress(&job.id, 4, Some(10))
        .expect_err("regression must fail");
    assert_eq!(error.code(), "download_progress_regressed");
    let error = manager
        .report_progress(&job.id, 11, Some(10))
        .expect_err("overflow must fail");
    assert_eq!(error.code(), "download_progress_exceeds_total");
}

#[test]
fn queued_cancel_is_immediate_and_retryable() {
    let mut manager = DownloadManager::new(DownloadPolicy::default()).expect("manager");
    let job = manager.enqueue(request("pack")).expect("queue");
    let cancelled = manager.request_cancel(&job.id).expect("cancel");
    assert_eq!(cancelled.state, DownloadJobState::Cancelled);
    let retried = manager.retry(&job.id).expect("retry");
    assert_eq!(retried.state, DownloadJobState::Queued);
}

#[test]
fn active_cancel_requires_transport_acknowledgement() {
    let mut manager = DownloadManager::new(DownloadPolicy::default()).expect("manager");
    let job = manager.enqueue(request("pack")).expect("queue");
    manager.claim_ready_jobs();
    manager.mark_transferring(&job.id).expect("transfer");
    let cancelling = manager.request_cancel(&job.id).expect("request cancel");
    assert_eq!(cancelling.state, DownloadJobState::CancelRequested);
    let cancelled = manager.acknowledge_cancel(&job.id).expect("ack cancel");
    assert_eq!(cancelled.state, DownloadJobState::Cancelled);
}

#[test]
fn finalization_requires_complete_declared_size() {
    let mut manager = DownloadManager::new(DownloadPolicy::default()).expect("manager");
    let job = manager.enqueue(request("pack")).expect("queue");
    manager.claim_ready_jobs();
    manager.mark_transferring(&job.id).expect("transfer");
    manager
        .report_progress(&job.id, 9, Some(10))
        .expect("progress");
    let error = manager
        .begin_finalizing(&job.id)
        .expect_err("incomplete download must not finalize");
    assert_eq!(error.code(), "download_finalize_incomplete");
    manager
        .report_progress(&job.id, 10, Some(10))
        .expect("progress");
    manager.begin_finalizing(&job.id).expect("finalizing");
    let completed = manager.mark_completed(&job.id).expect("complete");
    assert_eq!(completed.state, DownloadJobState::Completed);
}

#[test]
fn non_retryable_failure_stays_failed() {
    let mut manager = DownloadManager::new(DownloadPolicy::default()).expect("manager");
    let job = manager.enqueue(request("pack")).expect("queue");
    manager.claim_ready_jobs();
    manager
        .mark_failed(&job.id, "fixture", "failed", false)
        .expect("failed");
    let error = manager.retry(&job.id).expect_err("retry must fail");
    assert_eq!(error.code(), "download_failure_not_retryable");
}

#[test]
fn recovery_marks_active_jobs_interrupted() {
    let mut manager = DownloadManager::new(DownloadPolicy::default()).expect("manager");
    manager.enqueue(request("pack")).expect("queue");
    manager.claim_ready_jobs();
    let persisted = manager.persisted_state();
    let recovered =
        DownloadManager::recover(DownloadPolicy::default(), persisted).expect("recover");
    assert_eq!(
        recovered.snapshot().jobs[0].state,
        DownloadJobState::Interrupted
    );
}

#[test]
fn store_round_trip_preserves_queue() {
    let directory = tempfile::tempdir().expect("tempdir");
    let store = DownloadStore::new(directory.path().join("downloads.json"));
    let mut manager = DownloadManager::new(DownloadPolicy::default()).expect("manager");
    manager.enqueue(request("pack")).expect("queue");
    store.save(&manager.persisted_state()).expect("save");
    let recovered =
        DownloadManager::recover(DownloadPolicy::default(), store.load().expect("load"))
            .expect("recover");
    assert_eq!(recovered.snapshot().queued_jobs, 1);
}

#[test]
fn destination_file_name_cannot_escape_root() {
    let error = plan_workspace(
        std::path::Path::new("workspace"),
        std::path::Path::new("downloads"),
        "download-000001",
        "../escape.mcpack",
    )
    .expect_err("unsafe destination must fail");
    assert_eq!(error.code(), "download_destination_name_invalid");
}

#[test]
fn finalization_publishes_complete_file_without_overwrite() {
    let directory = tempfile::tempdir().expect("tempdir");
    let workspace = directory.path().join("workspace");
    let destination = directory.path().join("downloads");
    let plan =
        plan_workspace(&workspace, &destination, "download-000001", "pack.mcpack").expect("plan");
    ensure_workspace(&plan).expect("workspace");
    fs::write(&plan.payload_path, b"complete payload").expect("payload");
    let final_path = finalize_payload(&plan).expect("finalize");
    assert_eq!(
        fs::read(&final_path).expect("final file"),
        b"complete payload"
    );
    let error = finalize_payload(&plan).expect_err("overwrite must be rejected");
    assert_eq!(error.code(), "download_destination_exists");
}
