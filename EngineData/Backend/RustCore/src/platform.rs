use std::{env, path::PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlatformKind {
    Windows,
    Unsupported,
}

#[derive(Debug, Clone)]
pub struct PlatformContext {
    pub kind: PlatformKind,
    pub roaming_app_data: Option<PathBuf>,
    pub local_app_data: Option<PathBuf>,
}

impl PlatformContext {
    pub fn from_process() -> Self {
        Self {
            kind: if cfg!(target_os = "windows") {
                PlatformKind::Windows
            } else {
                PlatformKind::Unsupported
            },
            roaming_app_data: env::var_os("APPDATA").map(PathBuf::from),
            local_app_data: env::var_os("LOCALAPPDATA").map(PathBuf::from),
        }
    }

    #[cfg(test)]
    pub(crate) fn windows(roaming_app_data: PathBuf, local_app_data: PathBuf) -> Self {
        Self {
            kind: PlatformKind::Windows,
            roaming_app_data: Some(roaming_app_data),
            local_app_data: Some(local_app_data),
        }
    }
}
