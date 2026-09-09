mod manager;
mod model;
mod store;
mod workspace;

pub use manager::DownloadManager;
pub use model::*;
pub use store::DownloadStore;
pub use workspace::{
    ensure_workspace, finalize_payload, plan_workspace, validate_destination_file_name,
    DownloadWorkspacePlan,
};

#[cfg(test)]
mod tests;
