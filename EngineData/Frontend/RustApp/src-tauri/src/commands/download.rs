use super::error::CommandError;
use searchnow_core::{
    app_runtime::SearchNowBackendRuntime,
    download::{DownloadJob, DownloadManagerSnapshot, DownloadRequest},
};
use tauri::State;

#[tauri::command]
pub fn get_download_snapshot(
    state: State<'_, SearchNowBackendRuntime>,
) -> Result<DownloadManagerSnapshot, CommandError> {
    state.download_snapshot().map_err(CommandError::from)
}

#[tauri::command]
pub fn queue_download(
    state: State<'_, SearchNowBackendRuntime>,
    request: DownloadRequest,
) -> Result<DownloadJob, CommandError> {
    state.queue_download(request).map_err(CommandError::from)
}

#[tauri::command]
pub fn cancel_download(
    state: State<'_, SearchNowBackendRuntime>,
    job_id: String,
) -> Result<DownloadJob, CommandError> {
    state.cancel_download(&job_id).map_err(CommandError::from)
}

#[tauri::command]
pub fn retry_download(
    state: State<'_, SearchNowBackendRuntime>,
    job_id: String,
) -> Result<DownloadJob, CommandError> {
    state.retry_download(&job_id).map_err(CommandError::from)
}

#[tauri::command]
pub fn remove_download(
    state: State<'_, SearchNowBackendRuntime>,
    job_id: String,
) -> Result<DownloadManagerSnapshot, CommandError> {
    state.remove_download(&job_id).map_err(CommandError::from)
}
