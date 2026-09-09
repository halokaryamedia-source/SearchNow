use super::error::CommandError;
use searchnow_core::package::{inspect_package, PackageInspection};
use std::path::PathBuf;

#[tauri::command]
pub async fn inspect_local_package(path: PathBuf) -> Result<PackageInspection, CommandError> {
    tauri::async_runtime::spawn_blocking(move || inspect_package(&path))
        .await
        .map_err(|error| {
            CommandError::new(
                "package_inspection_task_failed",
                format!("Package inspection task failed: {error}"),
            )
        })?
        .map_err(CommandError::from)
}
