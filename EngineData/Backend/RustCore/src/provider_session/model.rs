use serde::Serialize;

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum ProviderSessionState {
    Unavailable,
    Available,
    Expired,
    Refreshing,
    Failed,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ProviderSessionStatus {
    pub provider: String,
    pub state: ProviderSessionState,
    pub expires_at_ms: Option<u64>,
    pub failure_code: Option<String>,
    pub retryable: bool,
}

impl ProviderSessionStatus {
    pub(crate) fn unavailable(provider: impl Into<String>) -> Self {
        Self {
            provider: provider.into(),
            state: ProviderSessionState::Unavailable,
            expires_at_ms: None,
            failure_code: None,
            retryable: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ProviderSessionError {
    pub code: String,
    pub message: String,
    pub retryable: bool,
}

impl ProviderSessionError {
    pub(crate) fn new(
        code: impl Into<String>,
        message: impl Into<String>,
        retryable: bool,
    ) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            retryable,
        }
    }
}
