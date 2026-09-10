use super::model::{PersistedDownloadState, DOWNLOAD_SCHEMA_VERSION};
use crate::{
    error::{BackendError, BackendResult},
    persistence::{AtomicJsonContract, AtomicJsonStore},
};
use std::path::{Path, PathBuf};

const MAX_DOWNLOAD_STATE_BYTES: u64 = 4 * 1024 * 1024;
const DOWNLOAD_STORE_CONTRACT: AtomicJsonContract = AtomicJsonContract {
    max_bytes: MAX_DOWNLOAD_STATE_BYTES,
    temp_prefix: ".downloads.",
    backup_file_name: ".downloads.backup",
    subject: "download state",
    metadata_failed: "download_state_metadata_failed",
    read_failed: "download_state_read_failed",
    too_large: "download_state_too_large",
    invalid_json: "download_state_invalid_json",
    serialize_failed: "download_state_serialize_failed",
    directory_failed: "download_state_directory_failed",
    stage_failed: "download_state_stage_failed",
    replace_failed: "download_state_replace_failed",
};

#[derive(Debug, Clone)]
pub struct DownloadStore {
    store: AtomicJsonStore,
}

impl DownloadStore {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self {
            store: AtomicJsonStore::new(path, DOWNLOAD_STORE_CONTRACT),
        }
    }

    pub fn path(&self) -> &Path {
        self.store.path()
    }

    pub fn load(&self) -> BackendResult<PersistedDownloadState> {
        Ok(self
            .store
            .load(validate_download_schema)?
            .unwrap_or_default())
    }

    pub fn save(&self, state: &PersistedDownloadState) -> BackendResult<()> {
        self.store.save(state, validate_download_schema)
    }
}

fn validate_download_schema(state: &PersistedDownloadState) -> BackendResult<()> {
    if state.schema_version != DOWNLOAD_SCHEMA_VERSION {
        return Err(BackendError::new(
            "download_state_schema_unsupported",
            format!(
                "Download state schema {} is not supported by this SearchNow build.",
                state.schema_version
            ),
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::download::{DownloadJob, DownloadJobState, DownloadProgress, DownloadSourceRef};
    use std::fs;

    fn state(sequence: u64) -> PersistedDownloadState {
        PersistedDownloadState {
            schema_version: DOWNLOAD_SCHEMA_VERSION,
            next_sequence: sequence + 1,
            jobs: vec![DownloadJob {
                id: format!("download-{sequence:06}"),
                source: DownloadSourceRef {
                    transport: "fixture".into(),
                    resource_id: "asset".into(),
                },
                display_name: "Fixture".into(),
                destination_file_name: "fixture.mcpack".into(),
                state: DownloadJobState::Completed,
                progress: DownloadProgress {
                    downloaded_bytes: 1,
                    total_bytes: Some(1),
                },
                attempt: 1,
                last_error: None,
                created_at_ms: 1,
                updated_at_ms: 2,
            }],
        }
    }

    #[test]
    fn backup_recovers_download_state_when_primary_is_missing() {
        let directory = tempfile::tempdir().expect("tempdir");
        let path = directory.path().join("state.json");
        let store = DownloadStore::new(&path);
        let first = state(1);
        let second = state(2);
        store.save(&first).expect("first save");
        store.save(&second).expect("second save");
        fs::remove_file(path).expect("simulate replace gap");
        assert_eq!(store.load().expect("recover backup"), first);
    }
}
