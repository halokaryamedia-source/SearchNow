use super::model::{PersistedDownloadState, DOWNLOAD_SCHEMA_VERSION};
use crate::error::{BackendError, BackendResult};
use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
    process,
    time::{SystemTime, UNIX_EPOCH},
};

const MAX_DOWNLOAD_STATE_BYTES: u64 = 4 * 1024 * 1024;

#[derive(Debug, Clone)]
pub struct DownloadStore {
    path: PathBuf,
}

impl DownloadStore {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn load(&self) -> BackendResult<PersistedDownloadState> {
        if !self.path.exists() {
            return Ok(PersistedDownloadState::default());
        }
        let metadata = fs::metadata(&self.path).map_err(|error| {
            BackendError::from_io(
                "download_state_metadata_failed",
                "SearchNow could not inspect its download state.",
                error,
            )
        })?;
        if metadata.len() > MAX_DOWNLOAD_STATE_BYTES {
            return Err(BackendError::new(
                "download_state_too_large",
                "SearchNow download state is unexpectedly large and was not loaded.",
            ));
        }
        let text = fs::read_to_string(&self.path).map_err(|error| {
            BackendError::from_io(
                "download_state_read_failed",
                "SearchNow could not read its download state.",
                error,
            )
        })?;
        let state: PersistedDownloadState = serde_json::from_str(&text).map_err(|error| {
            BackendError::new(
                "download_state_invalid_json",
                format!("SearchNow download state is invalid: {error}"),
            )
        })?;
        if state.schema_version != DOWNLOAD_SCHEMA_VERSION {
            return Err(BackendError::new(
                "download_state_schema_unsupported",
                format!(
                    "Download state schema {} is not supported by this SearchNow build.",
                    state.schema_version
                ),
            ));
        }
        Ok(state)
    }

    pub fn save(&self, state: &PersistedDownloadState) -> BackendResult<()> {
        if state.schema_version != DOWNLOAD_SCHEMA_VERSION {
            return Err(BackendError::new(
                "download_state_schema_unsupported",
                "SearchNow refused to save an unsupported download state schema.",
            ));
        }
        let parent = self.path.parent().ok_or_else(|| {
            BackendError::new(
                "download_state_path_invalid",
                "Download state path has no parent directory.",
            )
        })?;
        fs::create_dir_all(parent).map_err(|error| {
            BackendError::from_io(
                "download_state_directory_failed",
                "SearchNow could not create its download state directory.",
                error,
            )
        })?;
        let bytes = serde_json::to_vec_pretty(state).map_err(|error| {
            BackendError::new(
                "download_state_serialize_failed",
                format!("SearchNow could not serialize download state: {error}"),
            )
        })?;
        if bytes.len() as u64 > MAX_DOWNLOAD_STATE_BYTES {
            return Err(BackendError::new(
                "download_state_too_large",
                "SearchNow refused to persist an oversized download state.",
            ));
        }

        let token = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        let temp = parent.join(format!(".downloads.{}.{}.tmp", process::id(), token));
        let backup = parent.join(".downloads.backup");
        let mut file = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&temp)
            .map_err(|error| {
                BackendError::from_io(
                    "download_state_stage_failed",
                    "SearchNow could not stage download state.",
                    error,
                )
            })?;
        file.write_all(&bytes)
            .and_then(|_| file.sync_all())
            .map_err(|error| {
                let _ = fs::remove_file(&temp);
                BackendError::from_io(
                    "download_state_stage_failed",
                    "SearchNow could not finish staging download state.",
                    error,
                )
            })?;

        if self.path.exists() {
            let _ = fs::remove_file(&backup);
            fs::rename(&self.path, &backup).map_err(|error| {
                let _ = fs::remove_file(&temp);
                BackendError::from_io(
                    "download_state_replace_failed",
                    "SearchNow could not prepare existing download state for replacement.",
                    error,
                )
            })?;
        }
        if let Err(error) = fs::rename(&temp, &self.path) {
            let _ = fs::rename(&backup, &self.path);
            let _ = fs::remove_file(&temp);
            return Err(BackendError::from_io(
                "download_state_replace_failed",
                "SearchNow could not replace its download state.",
                error,
            ));
        }
        let _ = fs::remove_file(&backup);
        Ok(())
    }
}
