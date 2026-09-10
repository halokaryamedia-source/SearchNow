use super::error::CommandError;
use tauri::AppHandle;
use tauri_plugin_dialog::DialogExt;

#[tauri::command]
pub async fn choose_download_directory(app: AppHandle) -> Result<Option<String>, CommandError> {
    let selected = app.dialog().file().blocking_pick_folder();
    let Some(selected) = selected else {
        return Ok(None);
    };
    let path = selected.into_path().map_err(|_| {
        CommandError::new(
            "dialog_path_invalid",
            "The selected folder could not be used.",
        )
    })?;
    Ok(Some(path.to_string_lossy().into_owned()))
}
