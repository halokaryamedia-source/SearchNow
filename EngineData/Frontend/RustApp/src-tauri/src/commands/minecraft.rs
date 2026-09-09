use super::{context::settings_store, error::CommandError};
use searchnow_core::{minecraft::{discover_minecraft_storage, MinecraftDiscoverySnapshot}, platform::PlatformContext};
use tauri::AppHandle;

#[tauri::command]
pub async fn discover_minecraft_storage_command(
    app: AppHandle,
) -> Result<MinecraftDiscoverySnapshot, CommandError> {
    let settings = settings_store(&app)?.load()?;
    let platform = PlatformContext::from_process();
    tauri::async_runtime::spawn_blocking(move || {
        discover_minecraft_storage(&settings.minecraft, &platform)
    })
    .await
    .map_err(|error| {
        CommandError::new(
            "minecraft_discovery_task_failed",
            format!("Minecraft discovery task failed: {error}"),
        )
    })
}
