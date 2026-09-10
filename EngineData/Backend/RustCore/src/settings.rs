use crate::{
    error::{BackendError, BackendResult},
    persistence::{AtomicJsonContract, AtomicJsonStore},
};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

pub const CURRENT_SCHEMA_VERSION: u32 = 1;
const MAX_SETTINGS_BYTES: u64 = 512 * 1024;
const SETTINGS_STORE_CONTRACT: AtomicJsonContract = AtomicJsonContract {
    max_bytes: MAX_SETTINGS_BYTES,
    temp_prefix: ".settings.",
    backup_file_name: ".settings.backup",
    subject: "settings",
    metadata_failed: "settings_metadata_failed",
    read_failed: "settings_read_failed",
    too_large: "settings_too_large",
    invalid_json: "settings_invalid_json",
    serialize_failed: "settings_serialize_failed",
    directory_failed: "settings_directory_failed",
    stage_failed: "settings_stage_failed",
    replace_failed: "settings_replace_failed",
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AppSettings {
    #[serde(default = "current_schema_version")]
    pub schema_version: u32,
    #[serde(default)]
    pub minecraft: MinecraftSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MinecraftSettings {
    #[serde(default)]
    pub root_override: Option<PathBuf>,
    #[serde(default)]
    pub include_preview: bool,
    #[serde(default = "default_true")]
    pub include_legacy_uwp: bool,
    #[serde(default)]
    pub include_development_content: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            schema_version: CURRENT_SCHEMA_VERSION,
            minecraft: MinecraftSettings::default(),
        }
    }
}

impl Default for MinecraftSettings {
    fn default() -> Self {
        Self {
            root_override: None,
            include_preview: false,
            include_legacy_uwp: true,
            include_development_content: false,
        }
    }
}

impl AppSettings {
    pub fn validate(&self) -> BackendResult<()> {
        if self.schema_version != CURRENT_SCHEMA_VERSION {
            return Err(BackendError::new(
                "settings_schema_unsupported",
                format!(
                    "Settings schema {} is not supported by this SearchNow build.",
                    self.schema_version
                ),
            ));
        }
        if self
            .minecraft
            .root_override
            .as_ref()
            .is_some_and(|path| path.as_os_str().is_empty())
        {
            return Err(BackendError::new(
                "settings_minecraft_root_invalid",
                "Minecraft root override cannot be empty.",
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct SettingsStore {
    store: AtomicJsonStore,
}

impl SettingsStore {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self {
            store: AtomicJsonStore::new(path, SETTINGS_STORE_CONTRACT),
        }
    }

    pub fn path(&self) -> &Path {
        self.store.path()
    }

    pub fn load(&self) -> BackendResult<AppSettings> {
        Ok(self.store.load(AppSettings::validate)?.unwrap_or_default())
    }

    pub fn save(&self, settings: &AppSettings) -> BackendResult<()> {
        self.store.save(settings, AppSettings::validate)
    }
}

fn current_schema_version() -> u32 {
    CURRENT_SCHEMA_VERSION
}

fn default_true() -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn defaults_are_local_safe() {
        let settings = AppSettings::default();
        assert!(!settings.minecraft.include_preview);
        assert!(settings.minecraft.include_legacy_uwp);
        assert!(!settings.minecraft.include_development_content);
        assert!(settings.minecraft.root_override.is_none());
    }

    #[test]
    fn settings_round_trip() {
        let directory = tempfile::tempdir().expect("tempdir");
        let store = SettingsStore::new(directory.path().join("settings.json"));
        let mut settings = AppSettings::default();
        settings.minecraft.include_preview = true;
        store.save(&settings).expect("save");
        assert_eq!(store.load().expect("load"), settings);
    }

    #[test]
    fn settings_backup_recovers_interrupted_replace_gap() {
        let directory = tempfile::tempdir().expect("tempdir");
        let path = directory.path().join("settings.json");
        let store = SettingsStore::new(&path);
        let first = AppSettings::default();
        let mut second = AppSettings::default();
        second.minecraft.include_preview = true;
        store.save(&first).expect("first save");
        store.save(&second).expect("second save");
        fs::remove_file(path).expect("simulate replace gap");
        assert_eq!(store.load().expect("recover backup"), first);
    }

    #[test]
    fn future_schema_fails_closed() {
        let settings = AppSettings {
            schema_version: CURRENT_SCHEMA_VERSION + 1,
            ..AppSettings::default()
        };
        let error = settings.validate().expect_err("future schema must fail");
        assert_eq!(error.code(), "settings_schema_unsupported");
    }
}
