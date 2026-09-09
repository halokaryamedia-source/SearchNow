use super::{context::settings_store, error::CommandError};
use searchnow_core::settings::AppSettings;
use tauri::AppHandle;

#[tauri::command]
pub fn load_app_settings(app: AppHandle) -> Result<AppSettings, CommandError> {
    Ok(settings_store(&app)?.load()?)
}

#[tauri::command]
pub fn save_app_settings(
    app: AppHandle,
    settings: AppSettings,
) -> Result<AppSettings, CommandError> {
    settings_store(&app)?.save(&settings)?;
    Ok(settings)
}
