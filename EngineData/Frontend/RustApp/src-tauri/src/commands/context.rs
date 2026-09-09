use super::error::CommandError;
use searchnow_core::settings::SettingsStore;
use tauri::{AppHandle, Manager};

pub fn settings_store(app: &AppHandle) -> Result<SettingsStore, CommandError> {
    let directory = app.path().app_config_dir().map_err(|error| {
        CommandError::new(
            "app_config_path_unavailable",
            format!("SearchNow could not resolve its settings directory: {error}"),
        )
    })?;
    Ok(SettingsStore::new(directory.join("settings.json")))
}
