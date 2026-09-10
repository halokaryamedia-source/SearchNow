use super::{ProviderSessionError, ProviderSessionState, ProviderSessionStatus};
use crate::{
    error::{BackendError, BackendResult},
    provider_identity::valid_provider_key,
};
use std::{
    any::Any,
    collections::HashMap,
    sync::{Arc, Condvar, Mutex, MutexGuard},
    time::{SystemTime, UNIX_EPOCH},
};

const REFRESH_SKEW_MS: u64 = 30_000;
const RETRY_BACKOFF_MS: u64 = 1_000;

pub struct ProviderSessionFailure {
    pub code: String,
    pub retryable: bool,
}

impl ProviderSessionFailure {
    pub fn new(code: impl Into<String>, retryable: bool) -> Self {
        Self {
            code: code.into(),
            retryable,
        }
    }
}

pub struct ProviderSessionMaterial {
    value: Arc<dyn Any + Send + Sync>,
    expires_at_ms: Option<u64>,
}

impl ProviderSessionMaterial {
    pub fn new<T>(value: T, expires_at_ms: Option<u64>) -> Self
    where
        T: Any + Send + Sync,
    {
        Self {
            value: Arc::new(value),
            expires_at_ms,
        }
    }

    pub fn expires_at_ms(&self) -> Option<u64> {
        self.expires_at_ms
    }

    fn is_expired_at(&self, now_ms: u64) -> bool {
        self.expires_at_ms
            .is_some_and(|expires_at_ms| expires_at_ms <= now_ms)
    }

    fn needs_refresh_at(&self, now_ms: u64) -> bool {
        self.expires_at_ms
            .is_some_and(|expires_at_ms| expires_at_ms <= now_ms.saturating_add(REFRESH_SKEW_MS))
    }
}

#[derive(Clone)]
pub struct ProviderSessionLease {
    material: Arc<ProviderSessionMaterial>,
}

impl ProviderSessionLease {
    pub fn expires_at_ms(&self) -> Option<u64> {
        self.material.expires_at_ms()
    }

    pub fn downcast<T>(&self) -> Result<Arc<T>, ProviderSessionError>
    where
        T: Any + Send + Sync,
    {
        self.material.value.clone().downcast::<T>().map_err(|_| {
            ProviderSessionError::new(
                "provider_session_type_mismatch",
                "Provider session runtime material has an unexpected internal type.",
                false,
            )
        })
    }
}

pub trait ProviderSessionSource: Send + Sync {
    fn provider_key(&self) -> &str;

    fn acquire(&self) -> Result<ProviderSessionMaterial, ProviderSessionFailure>;

    fn refresh(
        &self,
        current: &ProviderSessionLease,
    ) -> Result<ProviderSessionMaterial, ProviderSessionFailure>;
}

#[derive(Default)]
pub struct ProviderSessionRegistry {
    sources: HashMap<String, Arc<dyn ProviderSessionSource>>,
}

impl ProviderSessionRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, source: Arc<dyn ProviderSessionSource>) -> BackendResult<()> {
        let key = source.provider_key().trim();
        if !valid_provider_key(key) {
            return Err(BackendError::new(
                "provider_session_key_invalid",
                "Provider session key is empty or unsupported.",
            ));
        }
        if self.sources.contains_key(key) {
            return Err(BackendError::new(
                "provider_session_duplicate",
                "A provider session source with the same key is already registered.",
            ));
        }
        self.sources.insert(key.to_string(), source);
        Ok(())
    }
}

pub struct ProviderSessionManager {
    entries: HashMap<String, Arc<ProviderSessionEntry>>,
}

impl ProviderSessionManager {
    pub fn new(registry: ProviderSessionRegistry) -> Self {
        let entries = registry
            .sources
            .into_iter()
            .map(|(key, source)| (key, Arc::new(ProviderSessionEntry::new(source))))
            .collect();
        Self { entries }
    }

    pub fn acquire(&self, provider: &str) -> Result<ProviderSessionLease, ProviderSessionError> {
        let Some(entry) = self.entries.get(provider) else {
            return Err(ProviderSessionError::new(
                "provider_session_unavailable",
                "No runtime session source is registered for this provider.",
                false,
            ));
        };
        entry.acquire()
    }

    pub fn status(&self, provider: &str) -> ProviderSessionStatus {
        if !valid_provider_key(provider) {
            return ProviderSessionStatus::unavailable("invalid");
        }
        self.entries.get(provider).map_or_else(
            || ProviderSessionStatus::unavailable(provider),
            |entry| entry.status(provider),
        )
    }
}

struct ProviderSessionEntry {
    source: Arc<dyn ProviderSessionSource>,
    state: Mutex<ProviderSessionEntryState>,
    changed: Condvar,
}

impl ProviderSessionEntry {
    fn new(source: Arc<dyn ProviderSessionSource>) -> Self {
        Self {
            source,
            state: Mutex::new(ProviderSessionEntryState::default()),
            changed: Condvar::new(),
        }
    }

    fn acquire(&self) -> Result<ProviderSessionLease, ProviderSessionError> {
        let mut waited_for_refresh = false;
        loop {
            let mut state = self.lock_state();
            let now = now_ms();
            if let Some(material) = state.material.as_ref() {
                if !material.needs_refresh_at(now) {
                    return Ok(ProviderSessionLease {
                        material: material.clone(),
                    });
                }
            }

            if matches!(state.phase, ProviderSessionPhase::Refreshing) {
                state = self
                    .changed
                    .wait(state)
                    .unwrap_or_else(|poisoned| poisoned.into_inner());
                waited_for_refresh = true;
                drop(state);
                continue;
            }

            if let ProviderSessionPhase::Failed(error) = &state.phase {
                let cooling_down = state.retry_after_ms.is_some_and(|retry_at| now < retry_at);
                if waited_for_refresh || !error.retryable || cooling_down {
                    return Err(error.clone());
                }
            }

            let previous = state
                .material
                .as_ref()
                .map(|material| ProviderSessionLease {
                    material: material.clone(),
                });
            state.phase = ProviderSessionPhase::Refreshing;
            drop(state);

            let result = match previous.as_ref() {
                Some(current) => self.source.refresh(current),
                None => self.source.acquire(),
            };

            let mut state = self.lock_state();
            match result {
                Ok(material) => {
                    let material = Arc::new(material);
                    if material.is_expired_at(now_ms()) {
                        let error = ProviderSessionError::new(
                            "provider_session_material_expired",
                            "Provider session source returned already-expired runtime material.",
                            true,
                        );
                        state.phase = ProviderSessionPhase::Failed(error.clone());
                        state.retry_after_ms = Some(now_ms().saturating_add(RETRY_BACKOFF_MS));
                        self.changed.notify_all();
                        return Err(error);
                    }
                    state.material = Some(material.clone());
                    state.phase = ProviderSessionPhase::Idle;
                    state.retry_after_ms = None;
                    self.changed.notify_all();
                    return Ok(ProviderSessionLease { material });
                }
                Err(failure) => {
                    let error = sanitize_failure(failure);
                    state.retry_after_ms = error
                        .retryable
                        .then(|| now_ms().saturating_add(RETRY_BACKOFF_MS));
                    state.phase = ProviderSessionPhase::Failed(error.clone());
                    self.changed.notify_all();
                    return Err(error);
                }
            }
        }
    }

    fn status(&self, provider: &str) -> ProviderSessionStatus {
        let state = self.lock_state();
        let expires_at_ms = state
            .material
            .as_ref()
            .and_then(|material| material.expires_at_ms());
        match &state.phase {
            ProviderSessionPhase::Refreshing => ProviderSessionStatus {
                provider: provider.to_string(),
                state: ProviderSessionState::Refreshing,
                expires_at_ms,
                failure_code: None,
                retryable: false,
            },
            ProviderSessionPhase::Failed(error) => ProviderSessionStatus {
                provider: provider.to_string(),
                state: ProviderSessionState::Failed,
                expires_at_ms,
                failure_code: Some(error.code.clone()),
                retryable: error.retryable,
            },
            ProviderSessionPhase::Idle => {
                let state_value = match state.material.as_ref() {
                    None => ProviderSessionState::Unavailable,
                    Some(material) if material.is_expired_at(now_ms()) => {
                        ProviderSessionState::Expired
                    }
                    Some(_) => ProviderSessionState::Available,
                };
                ProviderSessionStatus {
                    provider: provider.to_string(),
                    state: state_value,
                    expires_at_ms,
                    failure_code: None,
                    retryable: false,
                }
            }
        }
    }

    fn lock_state(&self) -> MutexGuard<'_, ProviderSessionEntryState> {
        self.state
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }
}

#[derive(Default)]
struct ProviderSessionEntryState {
    material: Option<Arc<ProviderSessionMaterial>>,
    phase: ProviderSessionPhase,
    retry_after_ms: Option<u64>,
}

#[derive(Default)]
enum ProviderSessionPhase {
    #[default]
    Idle,
    Refreshing,
    Failed(ProviderSessionError),
}

fn sanitize_failure(failure: ProviderSessionFailure) -> ProviderSessionError {
    let code = if valid_provider_key(&failure.code) {
        failure.code
    } else {
        "provider_session_failed".to_string()
    };
    ProviderSessionError::new(
        code,
        "Provider session could not be acquired or refreshed.",
        failure.retryable,
    )
}

fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
        .min(u64::MAX as u128) as u64
}
