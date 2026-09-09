use super::error::CommandError;
use searchnow_core::{app_runtime::SearchNowBackendRuntime, minecraft::MinecraftDiscoverySnapshot};
use tauri::State;

#[tauri::command]
pub async fn discover_minecraft_storage_command(
    state: State<'_, SearchNowBackendRuntime>,
) -> Result<MinecraftDiscoverySnapshot, CommandError> {
    let runtime = state.inner().clone();
    tauri::async_runtime::spawn_blocking(move || runtime.discover_minecraft())
        .await
        .map_err(|error| {
            CommandError::new(
                "minecraft_discovery_task_failed",
                format!("Minecraft discovery task failed: {error}"),
            )
        })?
        .map_err(CommandError::from)
}
