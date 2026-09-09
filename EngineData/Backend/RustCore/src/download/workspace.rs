use crate::error::{BackendError, BackendResult};
use serde::Serialize;
use std::{
    fs::{self, File, OpenOptions},
    io,
    path::{Path, PathBuf},
};

const MAX_DESTINATION_FILE_NAME_BYTES: usize = 240;
const MAX_JOB_ID_BYTES: usize = 128;

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DownloadWorkspacePlan {
    pub job_id: String,
    pub workspace_dir: PathBuf,
    pub payload_path: PathBuf,
    pub final_path: PathBuf,
}

pub fn validate_destination_file_name(value: &str) -> BackendResult<()> {
    if value.is_empty()
        || value == "."
        || value == ".."
        || value.len() > MAX_DESTINATION_FILE_NAME_BYTES
        || value.contains('/')
        || value.contains('\\')
        || value.chars().any(char::is_control)
    {
        return Err(BackendError::new(
            "download_destination_name_invalid",
            "Download destination must be one safe file name without path separators.",
        ));
    }
    Ok(())
}

fn validate_job_id(value: &str) -> BackendResult<()> {
    if value.is_empty()
        || value.len() > MAX_JOB_ID_BYTES
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_'))
    {
        return Err(BackendError::new(
            "download_job_id_invalid",
            "Download job id contains unsupported characters.",
        ));
    }
    Ok(())
}

pub fn plan_workspace(
    workspace_root: &Path,
    destination_root: &Path,
    job_id: &str,
    destination_file_name: &str,
) -> BackendResult<DownloadWorkspacePlan> {
    validate_job_id(job_id)?;
    validate_destination_file_name(destination_file_name)?;
    let workspace_dir = workspace_root.join(job_id);
    Ok(DownloadWorkspacePlan {
        job_id: job_id.to_string(),
        payload_path: workspace_dir.join("payload.part"),
        workspace_dir,
        final_path: destination_root.join(destination_file_name),
    })
}

pub fn ensure_workspace(plan: &DownloadWorkspacePlan) -> BackendResult<()> {
    fs::create_dir_all(&plan.workspace_dir).map_err(|error| {
        BackendError::from_io(
            "download_workspace_create_failed",
            "SearchNow could not create the download workspace.",
            error,
        )
    })
}

pub fn finalize_payload(plan: &DownloadWorkspacePlan) -> BackendResult<PathBuf> {
    validate_destination_file_name(
        plan.final_path
            .file_name()
            .and_then(|value| value.to_str())
            .unwrap_or_default(),
    )?;
    let payload_metadata = fs::symlink_metadata(&plan.payload_path).map_err(|error| {
        BackendError::from_io(
            "download_payload_missing",
            "Download payload is not available for finalization.",
            error,
        )
    })?;
    if !payload_metadata.file_type().is_file() || payload_metadata.file_type().is_symlink() {
        return Err(BackendError::new(
            "download_payload_invalid",
            "Download payload must be a regular non-symlink file.",
        ));
    }

    let destination_dir = plan.final_path.parent().ok_or_else(|| {
        BackendError::new(
            "download_destination_path_invalid",
            "Download destination has no parent directory.",
        )
    })?;
    fs::create_dir_all(destination_dir).map_err(|error| {
        BackendError::from_io(
            "download_destination_create_failed",
            "SearchNow could not create the download destination directory.",
            error,
        )
    })?;
    if plan.final_path.exists() {
        return Err(BackendError::new(
            "download_destination_exists",
            "Download destination already exists; automatic overwrite is disabled.",
        ));
    }

    let stage_path = destination_dir.join(format!(".searchnow-{}.part", plan.job_id));
    let mut source = File::open(&plan.payload_path).map_err(|error| {
        BackendError::from_io(
            "download_payload_open_failed",
            "SearchNow could not open the completed download payload.",
            error,
        )
    })?;
    let mut stage = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&stage_path)
        .map_err(|error| {
            BackendError::from_io(
                "download_finalize_stage_failed",
                "SearchNow could not create its destination staging file.",
                error,
            )
        })?;

    if let Err(error) = io::copy(&mut source, &mut stage).and_then(|_| stage.sync_all()) {
        let _ = fs::remove_file(&stage_path);
        return Err(BackendError::from_io(
            "download_finalize_stage_failed",
            "SearchNow could not finish staging the downloaded file.",
            error,
        ));
    }
    drop(stage);

    if let Err(error) = fs::hard_link(&stage_path, &plan.final_path) {
        let _ = fs::remove_file(&stage_path);
        return Err(BackendError::from_io(
            "download_finalize_commit_failed",
            "SearchNow could not atomically publish the downloaded file.",
            error,
        ));
    }
    let _ = fs::remove_file(&stage_path);
    Ok(plan.final_path.clone())
}
