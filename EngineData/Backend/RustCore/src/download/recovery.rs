use super::{
    cleanup_destination_stage, cleanup_workspace, plan_workspace, validate_destination_file_name,
    validate_job_id, DownloadJob, DownloadJobState, DownloadPolicy, PersistedDownloadState,
    DOWNLOAD_SCHEMA_VERSION,
};
use crate::error::{BackendError, BackendResult};
use std::{collections::HashSet, fs, path::Path};

const MAX_TRANSPORT_KEY_BYTES: usize = 64;
const MAX_RESOURCE_ID_BYTES: usize = 512;
const MAX_DISPLAY_NAME_BYTES: usize = 256;
const MAX_FAILURE_CODE_BYTES: usize = 128;
const MAX_FAILURE_MESSAGE_BYTES: usize = 4 * 1024;

pub(crate) fn validate_and_reconcile_persisted_state(
    mut persisted: PersistedDownloadState,
    policy: DownloadPolicy,
    workspace_root: &Path,
    destination_root: &Path,
) -> BackendResult<PersistedDownloadState> {
    if persisted.schema_version != DOWNLOAD_SCHEMA_VERSION {
        return Err(BackendError::new(
            "download_state_schema_unsupported",
            "Persisted download state schema is not supported.",
        ));
    }
    if persisted.jobs.len() > policy.max_jobs {
        return Err(BackendError::new(
            "download_state_job_limit",
            "Persisted download state exceeds the configured job limit.",
        ));
    }

    let mut seen = HashSet::with_capacity(persisted.jobs.len());
    for job in &mut persisted.jobs {
        validate_persisted_job(job)?;
        if !seen.insert(job.id.clone()) {
            return Err(invalid_state(
                "Persisted download state contains duplicate job ids.",
            ));
        }

        if job.state == DownloadJobState::Finalizing
            && reconcile_completed_final_file(job, workspace_root, destination_root)
        {
            continue;
        }

        if job.state.is_active() {
            if let Ok(plan) = plan_workspace(
                workspace_root,
                destination_root,
                &job.id,
                &job.destination_file_name,
            ) {
                let _ = cleanup_destination_stage(&plan);
                let _ = cleanup_workspace(&plan);
            }
        }
    }

    Ok(persisted)
}

fn validate_persisted_job(job: &DownloadJob) -> BackendResult<()> {
    validate_job_id(&job.id).map_err(|_| invalid_state("Persisted download job id is invalid."))?;
    validate_destination_file_name(&job.destination_file_name)
        .map_err(|_| invalid_state("Persisted download destination is invalid."))?;

    let display_name = job.display_name.trim();
    if display_name.is_empty()
        || display_name != job.display_name
        || display_name.len() > MAX_DISPLAY_NAME_BYTES
        || display_name.chars().any(char::is_control)
    {
        return Err(invalid_state("Persisted download display name is invalid."));
    }
    if job.source.transport.is_empty()
        || job.source.transport.len() > MAX_TRANSPORT_KEY_BYTES
        || !job
            .source
            .transport
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.'))
    {
        return Err(invalid_state(
            "Persisted download transport identity is invalid.",
        ));
    }
    if job.source.resource_id.is_empty()
        || job.source.resource_id.len() > MAX_RESOURCE_ID_BYTES
        || job.source.resource_id.chars().any(char::is_control)
    {
        return Err(invalid_state(
            "Persisted download resource identity is invalid.",
        ));
    }
    if job
        .progress
        .total_bytes
        .is_some_and(|total| job.progress.downloaded_bytes > total)
    {
        return Err(invalid_state(
            "Persisted download progress exceeds its total size.",
        ));
    }
    if job.updated_at_ms < job.created_at_ms {
        return Err(invalid_state(
            "Persisted download timestamps are inconsistent.",
        ));
    }
    if job.state == DownloadJobState::Completed {
        if job.last_error.is_some()
            || job
                .progress
                .total_bytes
                .is_some_and(|total| job.progress.downloaded_bytes != total)
        {
            return Err(invalid_state(
                "Persisted completed download state is inconsistent.",
            ));
        }
    }
    if job.state == DownloadJobState::Failed && job.last_error.is_none() {
        return Err(invalid_state(
            "Persisted failed download is missing failure state.",
        ));
    }
    if let Some(error) = &job.last_error {
        if error.code.is_empty()
            || error.code.len() > MAX_FAILURE_CODE_BYTES
            || error.code.chars().any(char::is_control)
            || error.message.is_empty()
            || error.message.len() > MAX_FAILURE_MESSAGE_BYTES
            || error
                .message
                .chars()
                .any(|character| character.is_control() && !matches!(character, '\n' | '\r' | '\t'))
        {
            return Err(invalid_state(
                "Persisted download failure metadata is invalid.",
            ));
        }
    }
    Ok(())
}

fn reconcile_completed_final_file(
    job: &mut DownloadJob,
    workspace_root: &Path,
    destination_root: &Path,
) -> bool {
    let Ok(plan) = plan_workspace(
        workspace_root,
        destination_root,
        &job.id,
        &job.destination_file_name,
    ) else {
        return false;
    };
    let Ok(metadata) = fs::symlink_metadata(&plan.final_path) else {
        return false;
    };
    if metadata.file_type().is_symlink() || !metadata.file_type().is_file() {
        return false;
    }
    if metadata.len() != job.progress.downloaded_bytes {
        return false;
    }
    if job
        .progress
        .total_bytes
        .is_some_and(|total| metadata.len() != total)
    {
        return false;
    }

    job.state = DownloadJobState::Completed;
    job.last_error = None;
    let _ = cleanup_destination_stage(&plan);
    let _ = cleanup_workspace(&plan);
    true
}

fn invalid_state(message: &'static str) -> BackendError {
    BackendError::new("download_state_job_invalid", message)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::download::{DownloadProgress, DownloadSourceRef};

    fn job(state: DownloadJobState) -> DownloadJob {
        DownloadJob {
            id: "download-000001".into(),
            source: DownloadSourceRef {
                transport: "fixture".into(),
                resource_id: "asset".into(),
            },
            display_name: "Fixture".into(),
            destination_file_name: "fixture.mcpack".into(),
            state,
            progress: DownloadProgress {
                downloaded_bytes: 4,
                total_bytes: Some(4),
            },
            attempt: 1,
            last_error: None,
            created_at_ms: 1,
            updated_at_ms: 2,
        }
    }

    #[test]
    fn duplicate_persisted_job_ids_fail_closed() {
        let directory = tempfile::tempdir().expect("tempdir");
        let persisted = PersistedDownloadState {
            schema_version: DOWNLOAD_SCHEMA_VERSION,
            next_sequence: 2,
            jobs: vec![job(DownloadJobState::Queued), job(DownloadJobState::Queued)],
        };
        let error = validate_and_reconcile_persisted_state(
            persisted,
            DownloadPolicy::default(),
            &directory.path().join("workspace"),
            &directory.path().join("files"),
        )
        .expect_err("duplicates must fail");
        assert_eq!(error.code(), "download_state_job_invalid");
    }

    #[test]
    fn published_finalizing_job_recovers_as_completed() {
        let directory = tempfile::tempdir().expect("tempdir");
        let workspace = directory.path().join("workspace");
        let files = directory.path().join("files");
        fs::create_dir_all(&files).expect("destination");
        fs::write(files.join("fixture.mcpack"), b"data").expect("final file");
        let persisted = PersistedDownloadState {
            schema_version: DOWNLOAD_SCHEMA_VERSION,
            next_sequence: 2,
            jobs: vec![job(DownloadJobState::Finalizing)],
        };
        let reconciled = validate_and_reconcile_persisted_state(
            persisted,
            DownloadPolicy::default(),
            &workspace,
            &files,
        )
        .expect("reconcile");
        assert_eq!(reconciled.jobs[0].state, DownloadJobState::Completed);
    }
}
