use searchnow_core::download::{DownloadManager, DownloadPolicy, DownloadStore};
use tauri::Manager;

pub fn configure_main_window<R: tauri::Runtime>(
    app: &mut tauri::App<R>,
) -> Result<(), Box<dyn std::error::Error>> {
    let data_dir = app.path().app_data_dir()?;
    let store = DownloadStore::new(data_dir.join("downloads").join("state.json"));
    let manager = DownloadManager::recover(DownloadPolicy::default(), store.load()?)?;
    app.manage(crate::commands::download::DownloadCommandState::new(
        manager, store,
    ));

    if let Some(window) = app.get_webview_window("main") {
        window.set_title("SearchNow")?;
    }
    Ok(())
}
