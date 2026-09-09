pub fn register<R: tauri::Runtime>(builder: tauri::Builder<R>) -> tauri::Builder<R> {
    builder.invoke_handler(tauri::generate_handler![
        crate::commands::runtime::get_runtime_status,
        crate::commands::settings::load_app_settings,
        crate::commands::settings::save_app_settings,
        crate::commands::minecraft::discover_minecraft_storage_command,
        crate::commands::library::scan_local_library,
        crate::commands::package::inspect_local_package,
        crate::commands::download::get_download_snapshot,
        crate::commands::download::queue_download,
        crate::commands::download::cancel_download,
        crate::commands::download::retry_download,
        crate::commands::download::remove_download,
    ])
}
