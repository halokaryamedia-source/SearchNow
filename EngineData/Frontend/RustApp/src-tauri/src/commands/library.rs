use super::{context::settings_store, error::CommandError};
use searchnow_core::{
    build_local_backend_snapshot, platform::PlatformContext, LocalBackendSnapshot,
};
use tauri::AppHandle;

#[tauri::command]
pub async fn scan_local_library(app: AppHandle) -> Result<LocalBackendSnapshot, CommandError> {
    let settings = settings_store(&app)?.load()?;
    let platform = PlatformContext::from_process();
    tauri::async_runtime::spawn_blocking(move || build_local_backend_snapshot(&settings, &platform))
        .await
        .map_err(|error| {
            CommandError::new(
                "library_scan_task_failed",
                format!("Local library scan task failed: {error}"),
            )
        })
}
