use std::{error::Error, fmt};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackendError {
    code: &'static str,
    message: String,
    detail: Option<String>,
}

impl BackendError {
    pub fn new(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            detail: None,
        }
    }

    pub fn from_io(
        code: &'static str,
        message: impl Into<String>,
        error: std::io::Error,
    ) -> Self {
        Self {
            code,
            message: message.into(),
            detail: Some(error.to_string()),
        }
    }

    pub fn code(&self) -> &'static str {
        self.code
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn detail(&self) -> Option<&str> {
        self.detail.as_deref()
    }
}

impl fmt::Display for BackendError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.message)
    }
}

impl Error for BackendError {}

pub type BackendResult<T> = Result<T, BackendError>;
