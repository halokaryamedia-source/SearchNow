mod executor;
mod http;
mod manager;
mod model;
mod resolver;
mod store;
mod transport;
mod workspace;

pub use executor::{default_download_paths, DownloadExecutionRuntime};
pub use http::{HttpTransport, HttpTransportPolicy, PUBLIC_HTTPS_TRANSPORT_KEY};
pub use manager::DownloadManager;
pub use model::*;
pub use resolver::{
    provider_download_source, ProviderResolveFailure, ProviderResolvedTransport,
    ProviderResourceRef, ResolvedResource, ResourceResolver, ResourceResolverRegistry,
    RESOLVED_PROVIDER_TRANSPORT_KEY,
};
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
mod http_tests;
#[cfg(test)]
mod resolver_tests;
#[cfg(test)]
mod tests;
