mod executor;
mod manager;
mod model;
mod store;
mod transport;
mod workspace;

pub use executor::{default_download_paths, DownloadExecutionRuntime};
pub use manager::DownloadManager;
pub use model::*;
pub use store::DownloadStore;
pub use transport::{
    DownloadTransport, DownloadTransportFailure, DownloadTransportRegistry,
    DownloadTransportStream, LocalFileTransport,
};
pub use workspace::{
    cleanup_workspace, ensure_workspace, finalize_payload, plan_workspace, prepare_payload_file,
    validate_destination_file_name, DownloadWorkspacePlan,
};

#[cfg(test)]
mod executor_tests;
#[cfg(test)]
mod tests;
