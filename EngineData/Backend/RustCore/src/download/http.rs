use super::{
    DownloadSourceRef, DownloadTransport, DownloadTransportFailure, DownloadTransportStream,
};
use crate::error::{BackendError, BackendResult};
use std::{
    io::{self, Read},
    time::Duration,
};
use url::Url;

pub const PUBLIC_HTTPS_TRANSPORT_KEY: &str = "https-public";
const HARD_MAX_RESPONSE_BYTES: u64 = 32 * 1024 * 1024 * 1024;
const HARD_MAX_REDIRECTS: u32 = 10;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HttpTransportPolicy {
    pub connect_timeout: Duration,
    pub read_timeout: Duration,
    pub overall_timeout: Duration,
    pub max_redirects: u32,
    pub max_response_bytes: u64,
}

impl Default for HttpTransportPolicy {
    fn default() -> Self {
        Self {
            connect_timeout: Duration::from_secs(10),
            read_timeout: Duration::from_secs(15),
            overall_timeout: Duration::from_secs(30 * 60),
            max_redirects: 5,
            max_response_bytes: 8 * 1024 * 1024 * 1024,
        }
    }
}

impl HttpTransportPolicy {
    pub fn validate(self) -> BackendResult<()> {
        if self.connect_timeout.is_zero()
            || self.read_timeout.is_zero()
            || self.overall_timeout.is_zero()
            || self.connect_timeout > self.overall_timeout
            || self.read_timeout > self.overall_timeout
            || self.max_redirects > HARD_MAX_REDIRECTS
            || self.max_response_bytes == 0
            || self.max_response_bytes > HARD_MAX_RESPONSE_BYTES
        {
            return Err(BackendError::new(
                "download_http_policy_invalid",
                "HTTP transport timeout, redirect, or response-size policy is invalid.",
            ));
        }
        Ok(())
    }
}

#[derive(Clone)]
pub struct HttpTransport {
    agent: ureq::Agent,
    policy: HttpTransportPolicy,
    allow_plain_http: bool,
}

impl HttpTransport {
    pub fn new(policy: HttpTransportPolicy) -> BackendResult<Self> {
        Self::build(policy, false)
    }

    fn build(policy: HttpTransportPolicy, allow_plain_http: bool) -> BackendResult<Self> {
        policy.validate()?;
        let agent = ureq::AgentBuilder::new()
            .redirects(0)
            .timeout_connect(policy.connect_timeout)
            .timeout_read(policy.read_timeout)
            .timeout_write(policy.connect_timeout)
            .timeout(policy.overall_timeout)
            .build();
        Ok(Self {
            agent,
            policy,
            allow_plain_http,
        })
    }

    #[cfg(test)]
    pub(crate) fn new_test_http(policy: HttpTransportPolicy) -> BackendResult<Self> {
        Self::build(policy, true)
    }

    fn open_url(
        &self,
        source: &DownloadSourceRef,
    ) -> Result<DownloadTransportStream, DownloadTransportFailure> {
        if source.transport != PUBLIC_HTTPS_TRANSPORT_KEY {
            return Err(DownloadTransportFailure::new(
                "download_http_transport_key_invalid",
                "HTTP transport received a job for a different transport key.",
                false,
            ));
        }

        let mut current = parse_source_url(&source.resource_id, self.allow_plain_http)?;
        let mut redirects = 0_u32;

        loop {
            let response = match self.agent.get(current.as_str()).call() {
                Ok(response) => response,
                Err(error) => return Err(map_request_error(error)),
            };
            let status = response.status();

            if matches!(status, 301 | 302 | 303 | 307 | 308) {
                if redirects >= self.policy.max_redirects {
                    return Err(DownloadTransportFailure::new(
                        "download_http_redirect_limit",
                        "HTTP response exceeded SearchNow's redirect limit.",
                        false,
                    ));
                }
                let location = response.header("Location").ok_or_else(|| {
                    DownloadTransportFailure::new(
                        "download_http_redirect_missing_location",
                        "HTTP redirect did not provide a Location header.",
                        false,
                    )
                })?;
                let next = current.join(location).map_err(|error| {
                    DownloadTransportFailure::new(
                        "download_http_redirect_invalid",
                        format!("HTTP redirect target is invalid: {error}"),
                        false,
                    )
                })?;
                validate_runtime_url(&next, self.allow_plain_http)?;
                current = next;
                redirects += 1;
                continue;
            }

            if !(200..300).contains(&status) {
                return Err(status_failure(status));
            }

            let total_bytes = parse_content_length(&response)?;
            if total_bytes.is_some_and(|value| value > self.policy.max_response_bytes) {
                return Err(DownloadTransportFailure::new(
                    "download_http_response_too_large",
                    format!(
                        "HTTP response exceeds the configured {} byte safety limit.",
                        self.policy.max_response_bytes
                    ),
                    false,
                ));
            }

            let reader =
                ResponseLimitReader::new(response.into_reader(), self.policy.max_response_bytes);
            return Ok(DownloadTransportStream {
                reader: Box::new(reader),
                total_bytes,
            });
        }
    }
}

impl DownloadTransport for HttpTransport {
    fn key(&self) -> &str {
        PUBLIC_HTTPS_TRANSPORT_KEY
    }

    fn open(
        &self,
        source: &DownloadSourceRef,
    ) -> Result<DownloadTransportStream, DownloadTransportFailure> {
        self.open_url(source)
    }
}

fn parse_source_url(value: &str, allow_plain_http: bool) -> Result<Url, DownloadTransportFailure> {
    let url = Url::parse(value).map_err(|error| {
        DownloadTransportFailure::new(
            "download_http_url_invalid",
            format!("Public HTTP resource URL is invalid: {error}"),
            false,
        )
    })?;
    validate_runtime_url(&url, allow_plain_http)?;
    if url.query().is_some() {
        return Err(DownloadTransportFailure::new(
            "download_http_query_not_persistable",
            "Public HTTPS download jobs cannot persist query strings; use a runtime resolver for signed or authenticated URLs.",
            false,
        ));
    }
    Ok(url)
}

fn validate_runtime_url(url: &Url, allow_plain_http: bool) -> Result<(), DownloadTransportFailure> {
    let scheme_allowed = url.scheme() == "https" || (allow_plain_http && url.scheme() == "http");
    if !scheme_allowed {
        return Err(DownloadTransportFailure::new(
            "download_http_scheme_rejected",
            "Public HTTP transport accepts HTTPS URLs only.",
            false,
        ));
    }
    if url.host_str().is_none() {
        return Err(DownloadTransportFailure::new(
            "download_http_host_missing",
            "HTTP resource URL must include a host.",
            false,
        ));
    }
    if !url.username().is_empty() || url.password().is_some() {
        return Err(DownloadTransportFailure::new(
            "download_http_userinfo_rejected",
            "HTTP resource URLs cannot embed usernames or passwords.",
            false,
        ));
    }
    if url.fragment().is_some() {
        return Err(DownloadTransportFailure::new(
            "download_http_fragment_rejected",
            "HTTP resource URLs cannot contain fragments.",
            false,
        ));
    }
    Ok(())
}

fn parse_content_length(
    response: &ureq::Response,
) -> Result<Option<u64>, DownloadTransportFailure> {
    let Some(value) = response.header("Content-Length") else {
        return Ok(None);
    };
    value.parse::<u64>().map(Some).map_err(|error| {
        DownloadTransportFailure::new(
            "download_http_content_length_invalid",
            format!("HTTP Content-Length is invalid: {error}"),
            false,
        )
    })
}

fn map_request_error(error: ureq::Error) -> DownloadTransportFailure {
    match error {
        ureq::Error::Status(status, _) => status_failure(status),
        ureq::Error::Transport(error) => DownloadTransportFailure::new(
            "download_http_request_failed",
            format!("HTTP request failed: {error}"),
            true,
        ),
    }
}

fn status_failure(status: u16) -> DownloadTransportFailure {
    let retryable = matches!(status, 408 | 425 | 429 | 500 | 502 | 503 | 504);
    DownloadTransportFailure::new(
        "download_http_status_error",
        format!("HTTP server returned status {status}."),
        retryable,
    )
}

struct ResponseLimitReader<R> {
    inner: R,
    remaining: u64,
}

impl<R> ResponseLimitReader<R> {
    fn new(inner: R, limit: u64) -> Self {
        Self {
            inner,
            remaining: limit,
        }
    }
}

impl<R: Read> Read for ResponseLimitReader<R> {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        if buffer.is_empty() {
            return Ok(0);
        }
        if self.remaining == 0 {
            let mut probe = [0_u8; 1];
            return match self.inner.read(&mut probe) {
                Ok(0) => Ok(0),
                Ok(_) => Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "HTTP response exceeded the configured download safety limit.",
                )),
                Err(error) => Err(error),
            };
        }

        let allowed = buffer.len().min(self.remaining as usize);
        let read = self.inner.read(&mut buffer[..allowed])?;
        self.remaining = self.remaining.saturating_sub(read as u64);
        Ok(read)
    }
}
