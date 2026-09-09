use super::error::CommandError;
use searchnow_core::{
    app_runtime::{BackendRuntimeSnapshot, SearchNowBackendRuntime},
    runtime::RuntimeStatus,
};
use tauri::State;

#[tauri::command]
pub fn get_runtime_status(state: State<'_, SearchNowBackendRuntime>) -> RuntimeStatus {
    state.runtime_status()
}

#[tauri::command]
pub async fn get_backend_runtime_snapshot(
    state: State<'_, SearchNowBackendRuntime>,
) -> Result<BackendRuntimeSnapshot, CommandError> {
    let runtime = state.inner().clone();
    tauri::async_runtime::spawn_blocking(move || runtime.snapshot())
        .await
        .map_err(|error| {
            CommandError::new(
                "backend_snapshot_task_failed",
                format!("Backend snapshot task failed: {error}"),
            )
        })?
        .map_err(CommandError::from)
}
