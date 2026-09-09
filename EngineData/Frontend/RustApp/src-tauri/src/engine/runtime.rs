use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeStatus {
    pub app_ready: bool,
    pub app_version: String,
    pub platform: String,
    pub architecture: String,
    pub backend: String,
}

pub fn runtime_status() -> RuntimeStatus {
    RuntimeStatus {
        app_ready: true,
        app_version: env!("CARGO_PKG_VERSION").to_string(),
        platform: std::env::consts::OS.to_string(),
        architecture: std::env::consts::ARCH.to_string(),
        backend: "Rust".to_string(),
    }
}
