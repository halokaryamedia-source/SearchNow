use searchnow_core::{
    app_runtime::{SearchNowBackendPaths, SearchNowBackendRuntime},
    platform::PlatformContext,
};
use tauri::Manager;

pub fn configure_application<R: tauri::Runtime>(
    app: &mut tauri::App<R>,
) -> Result<(), Box<dyn std::error::Error>> {
    let paths =
        SearchNowBackendPaths::from_roots(app.path().app_config_dir()?, app.path().app_data_dir()?);
    let runtime = SearchNowBackendRuntime::new(paths, PlatformContext::from_process(), Vec::new())?;
    app.manage(runtime);

    if let Some(window) = app.get_webview_window("main") {
        window.set_title("SearchNow")?;
    }
    Ok(())
}
