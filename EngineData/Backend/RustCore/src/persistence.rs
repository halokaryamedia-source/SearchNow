use crate::error::{BackendError, BackendResult};
use serde::{de::DeserializeOwned, Serialize};
use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
    process,
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Debug, Clone, Copy)]
pub(crate) struct AtomicJsonContract {
    pub max_bytes: u64,
    pub temp_prefix: &'static str,
    pub backup_file_name: &'static str,
    pub subject: &'static str,
    pub metadata_failed: &'static str,
    pub read_failed: &'static str,
    pub too_large: &'static str,
    pub invalid_json: &'static str,
    pub serialize_failed: &'static str,
    pub directory_failed: &'static str,
    pub stage_failed: &'static str,
    pub replace_failed: &'static str,
}

#[derive(Debug, Clone)]
pub(crate) struct AtomicJsonStore {
    path: PathBuf,
    contract: AtomicJsonContract,
}

impl AtomicJsonStore {
    pub(crate) fn new(path: impl Into<PathBuf>, contract: AtomicJsonContract) -> Self {
        Self {
            path: path.into(),
            contract,
        }
    }

    pub(crate) fn path(&self) -> &Path {
        &self.path
    }

    pub(crate) fn load<T, F>(&self, validate: F) -> BackendResult<Option<T>>
    where
        T: DeserializeOwned,
        F: Fn(&T) -> BackendResult<()>,
    {
        let backup = self.backup_path()?;
        if self.path.exists() {
            match self.load_candidate(&self.path, &validate) {
                Ok(value) => return Ok(Some(value)),
                Err(primary_error) => {
                    if backup.exists() {
                        if let Ok(value) = self.load_candidate(&backup, &validate) {
                            return Ok(Some(value));
                        }
                    }
                    return Err(primary_error);
                }
            }
        }

        if backup.exists() {
            return self.load_candidate(&backup, &validate).map(Some);
        }

        Ok(None)
    }

    pub(crate) fn save<T, F>(&self, value: &T, validate: F) -> BackendResult<()>
    where
        T: Serialize,
        F: Fn(&T) -> BackendResult<()>,
    {
        validate(value)?;
        let parent = self.parent()?;
        fs::create_dir_all(parent).map_err(|error| {
            BackendError::from_io(
                self.contract.directory_failed,
                format!(
                    "SearchNow could not create the {} directory.",
                    self.contract.subject
                ),
                error,
            )
        })?;

        let bytes = serde_json::to_vec_pretty(value).map_err(|error| {
            BackendError::new(
                self.contract.serialize_failed,
                format!(
                    "SearchNow could not serialize {}: {error}",
                    self.contract.subject
                ),
            )
        })?;
        if bytes.len() as u64 > self.contract.max_bytes {
            return Err(BackendError::new(
                self.contract.too_large,
                format!(
                    "SearchNow refused to persist oversized {}.",
                    self.contract.subject
                ),
            ));
        }

        self.cleanup_stale_temps(parent);
        let temp = self.temp_path(parent);
        let mut file = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&temp)
            .map_err(|error| {
                BackendError::from_io(
                    self.contract.stage_failed,
                    format!("SearchNow could not stage {}.", self.contract.subject),
                    error,
                )
            })?;
        file.write_all(&bytes)
            .and_then(|_| file.sync_all())
            .map_err(|error| {
                let _ = fs::remove_file(&temp);
                BackendError::from_io(
                    self.contract.stage_failed,
                    format!(
                        "SearchNow could not finish staging {}.",
                        self.contract.subject
                    ),
                    error,
                )
            })?;
        drop(file);

        let backup = self.backup_path()?;
        if self.path.exists() {
            self.ensure_regular_file(&self.path)?;
            if backup.exists() {
                self.ensure_regular_file(&backup)?;
                fs::remove_file(&backup).map_err(|error| {
                    let _ = fs::remove_file(&temp);
                    BackendError::from_io(
                        self.contract.replace_failed,
                        format!(
                            "SearchNow could not rotate the previous {} backup.",
                            self.contract.subject
                        ),
                        error,
                    )
                })?;
            }
            fs::rename(&self.path, &backup).map_err(|error| {
                let _ = fs::remove_file(&temp);
                BackendError::from_io(
                    self.contract.replace_failed,
                    format!(
                        "SearchNow could not prepare existing {} for replacement.",
                        self.contract.subject
                    ),
                    error,
                )
            })?;
        }

        if let Err(error) = fs::rename(&temp, &self.path) {
            if backup.exists() && !self.path.exists() {
                let _ = fs::rename(&backup, &self.path);
            }
            let _ = fs::remove_file(&temp);
            return Err(BackendError::from_io(
                self.contract.replace_failed,
                format!("SearchNow could not replace {}.", self.contract.subject),
                error,
            ));
        }

        Ok(())
    }

    fn load_candidate<T, F>(&self, path: &Path, validate: &F) -> BackendResult<T>
    where
        T: DeserializeOwned,
        F: Fn(&T) -> BackendResult<()>,
    {
        self.ensure_regular_file(path)?;
        let metadata = fs::metadata(path).map_err(|error| {
            BackendError::from_io(
                self.contract.metadata_failed,
                format!("SearchNow could not inspect {}.", self.contract.subject),
                error,
            )
        })?;
        if metadata.len() > self.contract.max_bytes {
            return Err(BackendError::new(
                self.contract.too_large,
                format!(
                    "SearchNow {} is unexpectedly large and was not loaded.",
                    self.contract.subject
                ),
            ));
        }
        let text = fs::read_to_string(path).map_err(|error| {
            BackendError::from_io(
                self.contract.read_failed,
                format!("SearchNow could not read {}.", self.contract.subject),
                error,
            )
        })?;
        let value = serde_json::from_str(&text).map_err(|error| {
            BackendError::new(
                self.contract.invalid_json,
                format!(
                    "SearchNow {} contains invalid JSON: {error}",
                    self.contract.subject
                ),
            )
        })?;
        validate(&value)?;
        Ok(value)
    }

    fn ensure_regular_file(&self, path: &Path) -> BackendResult<()> {
        let metadata = fs::symlink_metadata(path).map_err(|error| {
            BackendError::from_io(
                self.contract.metadata_failed,
                format!("SearchNow could not inspect {}.", self.contract.subject),
                error,
            )
        })?;
        if metadata.file_type().is_symlink() || !metadata.file_type().is_file() {
            return Err(BackendError::new(
                self.contract.read_failed,
                format!(
                    "SearchNow {} must be a regular non-symlink file.",
                    self.contract.subject
                ),
            ));
        }
        Ok(())
    }

    fn parent(&self) -> BackendResult<&Path> {
        self.path.parent().ok_or_else(|| {
            BackendError::new(
                self.contract.replace_failed,
                format!(
                    "SearchNow {} path has no parent directory.",
                    self.contract.subject
                ),
            )
        })
    }

    fn backup_path(&self) -> BackendResult<PathBuf> {
        Ok(self.parent()?.join(self.contract.backup_file_name))
    }

    fn temp_path(&self, parent: &Path) -> PathBuf {
        let token = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        parent.join(format!(
            "{}{}.{}.tmp",
            self.contract.temp_prefix,
            process::id(),
            token
        ))
    }

    fn cleanup_stale_temps(&self, parent: &Path) {
        let Ok(entries) = fs::read_dir(parent) else {
            return;
        };
        for entry in entries.filter_map(Result::ok) {
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if !name.starts_with(self.contract.temp_prefix) || !name.ends_with(".tmp") {
                continue;
            }
            let path = entry.path();
            let Ok(metadata) = fs::symlink_metadata(&path) else {
                continue;
            };
            if metadata.file_type().is_file() && !metadata.file_type().is_symlink() {
                let _ = fs::remove_file(path);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    const FIXTURE: AtomicJsonContract = AtomicJsonContract {
        max_bytes: 1024,
        temp_prefix: ".fixture.",
        backup_file_name: ".fixture.backup",
        subject: "fixture state",
        metadata_failed: "fixture_metadata_failed",
        read_failed: "fixture_read_failed",
        too_large: "fixture_too_large",
        invalid_json: "fixture_invalid_json",
        serialize_failed: "fixture_serialize_failed",
        directory_failed: "fixture_directory_failed",
        stage_failed: "fixture_stage_failed",
        replace_failed: "fixture_replace_failed",
    };

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
    struct FixtureState {
        value: u32,
    }

    fn valid(_: &FixtureState) -> BackendResult<()> {
        Ok(())
    }

    #[test]
    fn backup_recovers_missing_primary_after_interrupted_replace() {
        let directory = tempfile::tempdir().expect("tempdir");
        let path = directory.path().join("state.json");
        let store = AtomicJsonStore::new(&path, FIXTURE);
        store
            .save(&FixtureState { value: 1 }, valid)
            .expect("first save");
        store
            .save(&FixtureState { value: 2 }, valid)
            .expect("second save");
        fs::remove_file(&path).expect("simulate crash gap");

        let recovered = store
            .load::<FixtureState, _>(valid)
            .expect("load")
            .expect("backup");
        assert_eq!(recovered.value, 1);
    }

    #[test]
    fn backup_recovers_invalid_primary() {
        let directory = tempfile::tempdir().expect("tempdir");
        let path = directory.path().join("state.json");
        let store = AtomicJsonStore::new(&path, FIXTURE);
        store
            .save(&FixtureState { value: 7 }, valid)
            .expect("first save");
        store
            .save(&FixtureState { value: 8 }, valid)
            .expect("second save");
        fs::write(&path, "not-json").expect("corrupt primary");

        let recovered = store
            .load::<FixtureState, _>(valid)
            .expect("load")
            .expect("backup");
        assert_eq!(recovered.value, 7);
    }
}
