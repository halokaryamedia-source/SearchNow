use super::error::CommandError;
use searchnow_core::download::{
    DownloadJob, DownloadManager, DownloadManagerSnapshot, DownloadRequest, DownloadStore,
};
use std::sync::Mutex;
use tauri::State;

pub struct DownloadCommandState {
    manager: Mutex<DownloadManager>,
    store: DownloadStore,
}

impl DownloadCommandState {
    pub fn new(manager: DownloadManager, store: DownloadStore) -> Self {
        Self {
            manager: Mutex::new(manager),
            store,
        }
    }

    fn snapshot(&self) -> Result<DownloadManagerSnapshot, CommandError> {
        let manager = self.manager.lock().map_err(|_| {
            CommandError::new(
                "download_state_lock_failed",
                "Download manager state is unavailable.",
            )
        })?;
        Ok(manager.snapshot())
    }

    fn mutate<T>(
        &self,
        action: impl FnOnce(&mut DownloadManager) -> Result<T, searchnow_core::error::BackendError>,
    ) -> Result<T, CommandError> {
        let mut manager = self.manager.lock().map_err(|_| {
            CommandError::new(
                "download_state_lock_failed",
                "Download manager state is unavailable.",
            )
        })?;
        let mut candidate = manager.clone();
        let output = action(&mut candidate).map_err(CommandError::from)?;
        self.store
            .save(&candidate.persisted_state())
            .map_err(CommandError::from)?;
        *manager = candidate;
        Ok(output)
    }
}

#[tauri::command]
pub fn get_download_snapshot(
    state: State<'_, DownloadCommandState>,
) -> Result<DownloadManagerSnapshot, CommandError> {
    state.snapshot()
}

#[tauri::command]
pub fn queue_download(
    state: State<'_, DownloadCommandState>,
    request: DownloadRequest,
) -> Result<DownloadJob, CommandError> {
    state.mutate(|manager| manager.enqueue(request))
}

#[tauri::command]
pub fn cancel_download(
    state: State<'_, DownloadCommandState>,
    job_id: String,
) -> Result<DownloadJob, CommandError> {
    state.mutate(|manager| manager.request_cancel(&job_id))
}

#[tauri::command]
pub fn retry_download(
    state: State<'_, DownloadCommandState>,
    job_id: String,
) -> Result<DownloadJob, CommandError> {
    state.mutate(|manager| manager.retry(&job_id))
}

#[tauri::command]
pub fn remove_download(
    state: State<'_, DownloadCommandState>,
    job_id: String,
) -> Result<DownloadManagerSnapshot, CommandError> {
    state.mutate(|manager| {
        manager.remove_terminal(&job_id)?;
        Ok(manager.snapshot())
    })
}
