use super::error::CommandError;
use searchnow_core::{
    app_runtime::SearchNowBackendRuntime, package::PackageInspection,
};
use std::path::PathBuf;
use tauri::State;

#[tauri::command]
pub async fn inspect_local_package(
    state: State<'_, SearchNowBackendRuntime>,
    path: PathBuf,
) -> Result<PackageInspection, CommandError> {
    let runtime = state.inner().clone();
    tauri::async_runtime::spawn_blocking(move || runtime.inspect_package(&path))
        .await
        .map_err(|error| {
            CommandError::new(
                "package_inspection_task_failed",
                format!("Package inspection task failed: {error}"),
            )
        })?
        .map_err(CommandError::from)
}
