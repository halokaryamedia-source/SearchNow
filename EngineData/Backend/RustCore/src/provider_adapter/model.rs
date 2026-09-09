use crate::provider_session::ProviderSessionStatus;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ProviderCapabilities {
    pub provider: String,
    pub session: bool,
    pub catalog: bool,
    pub resolved_download: bool,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ProviderRuntimeStatus {
    pub capabilities: ProviderCapabilities,
    pub session: Option<ProviderSessionStatus>,
}
