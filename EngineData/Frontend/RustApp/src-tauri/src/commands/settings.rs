use super::error::CommandError;
use searchnow_core::{app_runtime::SearchNowBackendRuntime, settings::AppSettings};
use tauri::State;

#[tauri::command]
pub fn load_app_settings(
    state: State<'_, SearchNowBackendRuntime>,
) -> Result<AppSettings, CommandError> {
    state.load_settings().map_err(CommandError::from)
}

#[tauri::command]
pub fn save_app_settings(
    state: State<'_, SearchNowBackendRuntime>,
    settings: AppSettings,
) -> Result<AppSettings, CommandError> {
    state.save_settings(&settings).map_err(CommandError::from)
}
