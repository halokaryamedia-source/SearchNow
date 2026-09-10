use super::*;
use std::fs;

fn request(name: &str) -> DownloadRequest {
    DownloadRequest {
        source: DownloadSourceRef {
            transport: "fixture".into(),
            resource_id: format!("resource-{name}"),
        },
        display_name: name.into(),
        destination_file_name: format!("{name}.mcaddon"),
        expected_bytes: Some(7),
    }
}

#[test]
fn unicode_file_name_survives_keep_both_finalization() {
    let directory = tempfile::tempdir().expect("tempdir");
    let workspace = directory.path().join("workspace");
    let destination = directory.path().join("downloads");
    let plan = plan_workspace(
        &workspace,
        &destination,
        "download-000001",
        "Peta_日本.mcaddon",
    )
    .expect("plan");

    ensure_workspace(&plan).expect("workspace");
    fs::write(&plan.payload_path, b"payload").expect("payload");

    let first = finalize_payload(&plan).expect("first finalization");
    let second = finalize_payload(&plan).expect("keep both finalization");

    assert_eq!(
        first.file_name().and_then(|value| value.to_str()),
        Some("Peta_日本.mcaddon")
    );
    assert_eq!(
        second.file_name().and_then(|value| value.to_str()),
        Some("Peta_日本 (2).mcaddon")
    );
    assert_eq!(fs::read(first).expect("first payload"), b"payload");
    assert_eq!(fs::read(second).expect("second payload"), b"payload");
}

#[test]
fn custom_destination_must_be_absolute() {
    let mut manager = DownloadManager::new(DownloadPolicy::default()).expect("manager");
    let error = manager
        .enqueue_to(request("pack"), Some("relative/downloads".into()))
        .expect_err("relative destination must be rejected");

    assert_eq!(error.code(), "download_destination_directory_invalid");
}

#[test]
fn retry_preserves_the_user_selected_destination() {
    let directory = tempfile::tempdir().expect("tempdir");
    let destination = directory.path().join("chosen");
    let mut manager = DownloadManager::new(DownloadPolicy::default()).expect("manager");
    let job = manager
        .enqueue_to(request("pack"), Some(destination.clone()))
        .expect("queue");

    manager.claim_ready_jobs();
    manager
        .mark_failed(&job.id, "fixture_failure", "retry me", true)
        .expect("failed");
    let retried = manager.retry(&job.id).expect("retry");

    assert_eq!(retried.destination_directory, Some(destination));
    assert_eq!(retried.state, DownloadJobState::Queued);
    assert_eq!(retried.progress.downloaded_bytes, 0);
}
