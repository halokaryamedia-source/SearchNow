use crate::error::{BackendError, BackendResult};
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
    process,
    time::{SystemTime, UNIX_EPOCH},
};

pub const CURRENT_SCHEMA_VERSION: u32 = 1;
const MAX_SETTINGS_BYTES: u64 = 512 * 1024;

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
    path: PathBuf,
}

impl SettingsStore {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn load(&self) -> BackendResult<AppSettings> {
        if !self.path.exists() {
            return Ok(AppSettings::default());
        }
        let metadata = fs::metadata(&self.path).map_err(|error| {
            BackendError::from_io(
                "settings_metadata_failed",
                "SearchNow could not inspect its settings file.",
                error,
            )
        })?;
        if metadata.len() > MAX_SETTINGS_BYTES {
            return Err(BackendError::new(
                "settings_too_large",
                "SearchNow settings are unexpectedly large and were not loaded.",
            ));
        }
        let text = fs::read_to_string(&self.path).map_err(|error| {
            BackendError::from_io(
                "settings_read_failed",
                "SearchNow could not read its settings file.",
                error,
            )
        })?;
        let settings: AppSettings = serde_json::from_str(&text).map_err(|error| {
            BackendError::new(
                "settings_invalid_json",
                format!("SearchNow settings are invalid: {error}"),
            )
        })?;
        settings.validate()?;
        Ok(settings)
    }

    pub fn save(&self, settings: &AppSettings) -> BackendResult<()> {
        settings.validate()?;
        let parent = self.path.parent().ok_or_else(|| {
            BackendError::new(
                "settings_path_invalid",
                "SearchNow settings path has no parent directory.",
            )
        })?;
        fs::create_dir_all(parent).map_err(|error| {
            BackendError::from_io(
                "settings_directory_failed",
                "SearchNow could not create its settings directory.",
                error,
            )
        })?;
        let bytes = serde_json::to_vec_pretty(settings).map_err(|error| {
            BackendError::new(
                "settings_serialize_failed",
                format!("SearchNow could not serialize settings: {error}"),
            )
        })?;
        let token = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        let temp = parent.join(format!(".settings.{}.{}.tmp", process::id(), token));
        let backup = parent.join(".settings.backup");

        let mut file = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&temp)
            .map_err(|error| {
                BackendError::from_io(
                    "settings_stage_failed",
                    "SearchNow could not stage settings for saving.",
                    error,
                )
            })?;
        file.write_all(&bytes).and_then(|_| file.sync_all()).map_err(|error| {
            let _ = fs::remove_file(&temp);
            BackendError::from_io(
                "settings_stage_failed",
                "SearchNow could not finish staging settings.",
                error,
            )
        })?;

        if self.path.exists() {
            let _ = fs::remove_file(&backup);
            fs::rename(&self.path, &backup).map_err(|error| {
                let _ = fs::remove_file(&temp);
                BackendError::from_io(
                    "settings_replace_failed",
                    "SearchNow could not prepare the existing settings for replacement.",
                    error,
                )
            })?;
        }

        if let Err(error) = fs::rename(&temp, &self.path) {
            let _ = fs::rename(&backup, &self.path);
            let _ = fs::remove_file(&temp);
            return Err(BackendError::from_io(
                "settings_replace_failed",
                "SearchNow could not replace its settings file.",
                error,
            ));
        }
        let _ = fs::remove_file(&backup);
        Ok(())
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
    fn future_schema_fails_closed() {
        let mut settings = AppSettings::default();
        settings.schema_version = CURRENT_SCHEMA_VERSION + 1;
        let error = settings.validate().expect_err("future schema must fail");
        assert_eq!(error.code(), "settings_schema_unsupported");
    }
}
