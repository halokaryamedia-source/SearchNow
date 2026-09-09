use searchnow_core::runtime::{runtime_status, RuntimeStatus};

#[tauri::command]
pub fn get_runtime_status() -> RuntimeStatus {
    runtime_status()
}
