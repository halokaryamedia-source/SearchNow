use super::error::CommandError;
use searchnow_core::download::{
    DownloadExecutionRuntime, DownloadJob, DownloadManagerSnapshot, DownloadRequest,
};
use tauri::State;

#[tauri::command]
pub fn get_download_snapshot(
    state: State<'_, DownloadExecutionRuntime>,
) -> Result<DownloadManagerSnapshot, CommandError> {
    state.snapshot().map_err(CommandError::from)
}

#[tauri::command]
pub fn queue_download(
    state: State<'_, DownloadExecutionRuntime>,
    request: DownloadRequest,
) -> Result<DownloadJob, CommandError> {
    state.queue(request).map_err(CommandError::from)
}

#[tauri::command]
pub fn cancel_download(
    state: State<'_, DownloadExecutionRuntime>,
    job_id: String,
) -> Result<DownloadJob, CommandError> {
    state.cancel(&job_id).map_err(CommandError::from)
}

#[tauri::command]
pub fn retry_download(
    state: State<'_, DownloadExecutionRuntime>,
    job_id: String,
) -> Result<DownloadJob, CommandError> {
    state.retry(&job_id).map_err(CommandError::from)
}

#[tauri::command]
pub fn remove_download(
    state: State<'_, DownloadExecutionRuntime>,
    job_id: String,
) -> Result<DownloadManagerSnapshot, CommandError> {
    state.remove_terminal(&job_id).map_err(CommandError::from)
}
