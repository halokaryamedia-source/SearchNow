use tauri::Manager;

pub fn configure_main_window<R: tauri::Runtime>(
    app: &mut tauri::App<R>,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(window) = app.get_webview_window("main") {
        window.set_title("SearchNow")?;
    }
    Ok(())
}
