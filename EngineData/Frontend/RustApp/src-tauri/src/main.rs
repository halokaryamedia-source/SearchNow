mod app_bootstrap;
mod commands;

fn main() {
    let builder =
        tauri::Builder::default().setup(|app| app_bootstrap::configure_application(app));

    commands::registry::register(builder)
        .run(tauri::generate_context!())
        .expect("SearchNow app failed to run");
}
