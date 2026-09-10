use crate::error::{BackendError, BackendResult};
use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
    process,
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Debug, Clone)]
pub struct AtomicFileStore {
    path: PathBuf,
    backup_name: &'static str,
    temp_prefix: &'static str,
}

impl AtomicFileStore {
    pub fn new(
        path: impl Into<PathBuf>,
        backup_name: &'static str,
        temp_prefix: &'static str,
    ) -> Self {
        Self {
            path: path.into(),
            backup_name,
            temp_prefix,
        }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn read_to_string(&self, max_bytes: u64) -> BackendResult<Option<String>> {
        self.recover_primary()?;
        if !self.path.exists() {
            return Ok(None);
        }
        let metadata = fs::metadata(&self.path).map_err(|error| {
            BackendError::from_io(
                "storage_metadata_failed",
                "SearchNow could not inspect a persisted state file.",
                error,
            )
        })?;
        if metadata.len() > max_bytes {
            return Err(BackendError::new(
                "storage_too_large",
                "A SearchNow persisted state file exceeds its configured size limit.",
            ));
        }
        fs::read_to_string(&self.path).map(Some).map_err(|error| {
            BackendError::from_io(
                "storage_read_failed",
                "SearchNow could not read a persisted state file.",
                error,
            )
        })
    }

    pub fn replace(&self, bytes: &[u8], max_bytes: u64) -> BackendResult<()> {
        if bytes.len() as u64 > max_bytes {
            return Err(BackendError::new(
                "storage_too_large",
                "SearchNow refused to persist state larger than its configured size limit.",
            ));
        }
        let parent = self.path.parent().ok_or_else(|| {
            BackendError::new(
                "storage_path_invalid",
                "A SearchNow persisted state path has no parent directory.",
            )
        })?;
        fs::create_dir_all(parent).map_err(|error| {
            BackendError::from_io(
                "storage_directory_failed",
                "SearchNow could not create a persisted state directory.",
                error,
            )
        })?;
        self.cleanup_stale_temps(parent);

        let token = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        let temp = parent.join(format!(
            "{}.{}.{}.tmp",
            self.temp_prefix,
            process::id(),
            token
        ));
        let backup = parent.join(self.backup_name);

        let mut file = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&temp)
            .map_err(|error| {
                BackendError::from_io(
                    "storage_stage_failed",
                    "SearchNow could not stage persisted state.",
                    error,
                )
            })?;
        file.write_all(bytes)
            .and_then(|_| file.sync_all())
            .map_err(|error| {
                let _ = fs::remove_file(&temp);
                BackendError::from_io(
                    "storage_stage_failed",
                    "SearchNow could not finish staging persisted state.",
                    error,
                )
            })?;
        drop(file);

        if self.path.exists() {
            let _ = fs::remove_file(&backup);
            fs::rename(&self.path, &backup).map_err(|error| {
                let _ = fs::remove_file(&temp);
                BackendError::from_io(
                    "storage_replace_failed",
                    "SearchNow could not prepare existing persisted state for replacement.",
                    error,
                )
            })?;
        }

        if let Err(error) = fs::rename(&temp, &self.path) {
            let _ = fs::rename(&backup, &self.path);
            let _ = fs::remove_file(&temp);
            return Err(BackendError::from_io(
                "storage_replace_failed",
                "SearchNow could not replace persisted state.",
                error,
            ));
        }

        let _ = fs::remove_file(&backup);
        Ok(())
    }

    fn recover_primary(&self) -> BackendResult<()> {
        if self.path.exists() {
            return Ok(());
        }
        let Some(parent) = self.path.parent() else {
            return Err(BackendError::new(
                "storage_path_invalid",
                "A SearchNow persisted state path has no parent directory.",
            ));
        };
        let backup = parent.join(self.backup_name);
        if backup.exists() {
            fs::rename(&backup, &self.path).map_err(|error| {
                BackendError::from_io(
                    "storage_recovery_failed",
                    "SearchNow could not recover the last valid persisted state.",
                    error,
                )
            })?;
        }
        Ok(())
    }

    fn cleanup_stale_temps(&self, parent: &Path) {
        let Ok(entries) = fs::read_dir(parent) else {
            return;
        };
        let expected_prefix = format!("{}.", self.temp_prefix);
        for entry in entries.filter_map(Result::ok) {
            let Ok(file_type) = entry.file_type() else {
                continue;
            };
            if !file_type.is_file() || file_type.is_symlink() {
                continue;
            }
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if name.starts_with(&expected_prefix) && name.ends_with(".tmp") {
                let _ = fs::remove_file(entry.path());
            }
        }
    }
}
