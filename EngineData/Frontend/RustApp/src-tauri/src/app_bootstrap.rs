use searchnow_core::download::{
    default_download_paths, DownloadExecutionRuntime, DownloadPolicy, DownloadStore, HttpTransport,
    HttpTransportPolicy, DownloadTransportRegistry,
};
use std::sync::Arc;
use tauri::Manager;

pub fn configure_application<R: tauri::Runtime>(
    app: &mut tauri::App<R>,
) -> Result<(), Box<dyn std::error::Error>> {
    let app_data_root = app.path().app_data_dir()?;
    let (state_path, workspace_root, destination_root) = default_download_paths(&app_data_root);
    let mut transports = DownloadTransportRegistry::with_local_file()?;
    transports.register(Arc::new(HttpTransport::new(HttpTransportPolicy::default())?))?;
    let runtime = DownloadExecutionRuntime::new(
        DownloadPolicy::default(),
        DownloadStore::new(state_path),
        workspace_root,
        destination_root,
        transports,
    )?;
    app.manage(runtime);

    if let Some(window) = app.get_webview_window("main") {
        window.set_title("SearchNow")?;
    }
    Ok(())
}
