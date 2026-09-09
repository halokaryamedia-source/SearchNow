use super::*;
use crate::{
    catalog::{
        CatalogProvider, CatalogProviderFailure, CatalogProviderPage, CatalogProviderRegistry,
        CatalogQuery, CatalogService,
    },
    download::{ProviderResolveFailure, ResolvedResource, ResourceResolver},
};
use std::{
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        Arc, Barrier,
    },
    thread,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

struct SecretSession {
    value: String,
}

struct FakeSessionSource {
    acquire_calls: AtomicUsize,
    refresh_calls: AtomicUsize,
    fail_refresh: AtomicBool,
    block_refresh: AtomicBool,
    short_first_expiry: bool,
}

impl FakeSessionSource {
    fn stable() -> Self {
        Self {
            acquire_calls: AtomicUsize::new(0),
            refresh_calls: AtomicUsize::new(0),
            fail_refresh: AtomicBool::new(false),
            block_refresh: AtomicBool::new(false),
            short_first_expiry: false,
        }
    }

    fn expiring() -> Self {
        Self {
            short_first_expiry: true,
            ..Self::stable()
        }
    }
}

impl ProviderSessionSource for FakeSessionSource {
    fn provider_key(&self) -> &str {
        "fake"
    }

    fn acquire(&self) -> Result<ProviderSessionMaterial, ProviderSessionFailure> {
        self.acquire_calls.fetch_add(1, Ordering::SeqCst);
        let expires_at_ms = self
            .short_first_expiry
            .then(|| now_ms().saturating_add(30))
            .or(Some(u64::MAX));
        Ok(ProviderSessionMaterial::new(
            SecretSession {
                value: "runtime-secret-first".into(),
            },
            expires_at_ms,
        ))
    }

    fn refresh(
        &self,
        current: &ProviderSessionLease,
    ) -> Result<ProviderSessionMaterial, ProviderSessionFailure> {
        self.refresh_calls.fetch_add(1, Ordering::SeqCst);
        let previous = current
            .downcast::<SecretSession>()
            .map_err(|_| ProviderSessionFailure::new("fake_type_mismatch", false))?;
        assert!(previous.value.starts_with("runtime-secret"));
        while self.block_refresh.load(Ordering::SeqCst) {
            thread::sleep(Duration::from_millis(1));
        }
        thread::sleep(Duration::from_millis(40));
        if self.fail_refresh.load(Ordering::SeqCst) {
            return Err(ProviderSessionFailure::new(
                "token=runtime-secret-must-not-leak",
                true,
            ));
        }
        Ok(ProviderSessionMaterial::new(
            SecretSession {
                value: "runtime-secret-refreshed".into(),
            },
            Some(u64::MAX),
        ))
    }
}

fn session_manager(source: Arc<dyn ProviderSessionSource>) -> Arc<ProviderSessionManager> {
    let mut registry = ProviderSessionRegistry::new();
    registry.register(source).expect("register session source");
    Arc::new(ProviderSessionManager::new(registry))
}

#[test]
fn session_is_acquired_once_and_reused_without_public_secret_state() {
    let source = Arc::new(FakeSessionSource::stable());
    let manager = session_manager(source.clone());

    let first = manager.acquire("fake").expect("first session");
    let second = manager.acquire("fake").expect("reused session");
    assert_eq!(
        first.downcast::<SecretSession>().expect("secret").value,
        "runtime-secret-first"
    );
    assert_eq!(
        second.downcast::<SecretSession>().expect("secret").value,
        "runtime-secret-first"
    );
    assert_eq!(source.acquire_calls.load(Ordering::SeqCst), 1);

    let status = manager.status("fake");
    assert_eq!(status.state, ProviderSessionState::Available);
    let serialized = serde_json::to_string(&status).expect("safe status json");
    assert!(!serialized.contains("runtime-secret"));
}

#[test]
fn concurrent_expired_session_refresh_is_deduplicated() {
    let source = Arc::new(FakeSessionSource::expiring());
    let manager = session_manager(source.clone());
    manager.acquire("fake").expect("initial session");
    thread::sleep(Duration::from_millis(45));

    let mut workers = Vec::new();
    for _ in 0..8 {
        let manager = manager.clone();
        workers.push(thread::spawn(move || {
            manager
                .acquire("fake")
                .expect("refreshed session")
                .downcast::<SecretSession>()
                .expect("secret")
                .value
                .clone()
        }));
    }
    for worker in workers {
        assert_eq!(worker.join().expect("worker"), "runtime-secret-refreshed");
    }
    assert_eq!(source.acquire_calls.load(Ordering::SeqCst), 1);
    assert_eq!(source.refresh_calls.load(Ordering::SeqCst), 1);
}

#[test]
fn failed_refresh_wave_is_shared_without_refresh_storm() {
    let source = Arc::new(FakeSessionSource::expiring());
    let manager = session_manager(source.clone());
    manager.acquire("fake").expect("initial session");
    source.fail_refresh.store(true, Ordering::SeqCst);
    source.block_refresh.store(true, Ordering::SeqCst);
    thread::sleep(Duration::from_millis(45));

    let start = Arc::new(Barrier::new(9));
    let mut workers = Vec::new();
    for _ in 0..8 {
        let manager = manager.clone();
        let start = start.clone();
        workers.push(thread::spawn(move || {
            start.wait();
            match manager.acquire("fake") {
                Ok(_) => panic!("expected shared refresh failure"),
                Err(error) => error.code,
            }
        }));
    }
    start.wait();

    let deadline = Instant::now() + Duration::from_secs(1);
    while source.refresh_calls.load(Ordering::SeqCst) == 0 {
        assert!(Instant::now() < deadline, "refresh did not start");
        thread::sleep(Duration::from_millis(1));
    }
    thread::sleep(Duration::from_millis(50));
    source.block_refresh.store(false, Ordering::SeqCst);

    for worker in workers {
        assert_eq!(worker.join().expect("worker"), "provider_session_failed");
    }
    assert_eq!(source.refresh_calls.load(Ordering::SeqCst), 1);
}

#[test]
fn refresh_failure_is_sanitized_and_status_remains_secret_free() {
    let source = Arc::new(FakeSessionSource::expiring());
    let manager = session_manager(source.clone());
    manager.acquire("fake").expect("initial session");
    source.fail_refresh.store(true, Ordering::SeqCst);
    thread::sleep(Duration::from_millis(45));

    let error = match manager.acquire("fake") {
        Ok(_) => panic!("expected refresh failure"),
        Err(error) => error,
    };
    assert_eq!(error.code, "provider_session_failed");
    assert!(error.retryable);
    assert!(!error.message.contains("runtime-secret"));

    let status = manager.status("fake");
    assert_eq!(status.state, ProviderSessionState::Failed);
    assert_eq!(
        status.failure_code.as_deref(),
        Some("provider_session_failed")
    );
    let serialized = serde_json::to_string(&status).expect("safe status json");
    assert!(!serialized.contains("runtime-secret"));
}

#[test]
fn missing_provider_session_is_explicitly_unavailable() {
    let manager = ProviderSessionManager::new(ProviderSessionRegistry::new());
    let error = match manager.acquire("missing") {
        Ok(_) => panic!("expected missing session source"),
        Err(error) => error,
    };
    assert_eq!(error.code, "provider_session_unavailable");
    assert!(!error.retryable);
    assert_eq!(
        manager.status("missing").state,
        ProviderSessionState::Unavailable
    );
}

struct SessionCatalogProvider {
    sessions: Arc<ProviderSessionManager>,
}

impl CatalogProvider for SessionCatalogProvider {
    fn key(&self) -> &str {
        "fake"
    }

    fn query(&self, _query: &CatalogQuery) -> Result<CatalogProviderPage, CatalogProviderFailure> {
        let lease = self
            .sessions
            .acquire("fake")
            .map_err(|error| CatalogProviderFailure::new(error.code, error.retryable))?;
        let secret = lease
            .downcast::<SecretSession>()
            .map_err(|error| CatalogProviderFailure::new(error.code, error.retryable))?;
        assert!(!secret.value.is_empty());
        Ok(CatalogProviderPage {
            items: Vec::new(),
            next_cursor: None,
        })
    }
}

struct SessionResolver {
    sessions: Arc<ProviderSessionManager>,
}

impl ResourceResolver for SessionResolver {
    fn provider_key(&self) -> &str {
        "fake"
    }

    fn resolve(&self, _resource_id: &str) -> Result<ResolvedResource, ProviderResolveFailure> {
        let lease = self
            .sessions
            .acquire("fake")
            .map_err(|error| ProviderResolveFailure::new(error.code, error.retryable))?;
        let secret = lease
            .downcast::<SecretSession>()
            .map_err(|error| ProviderResolveFailure::new(error.code, error.retryable))?;
        assert!(!secret.value.is_empty());
        Ok(ResolvedResource::new("https://example.com/resource"))
    }
}

#[test]
fn catalog_provider_and_resource_resolver_share_one_session_owner() {
    let source = Arc::new(FakeSessionSource::stable());
    let sessions = session_manager(source.clone());

    let mut catalog_registry = CatalogProviderRegistry::new();
    catalog_registry
        .register(Arc::new(SessionCatalogProvider {
            sessions: sessions.clone(),
        }))
        .expect("catalog provider");
    let catalog = CatalogService::new(Arc::new(catalog_registry));
    catalog
        .query(&crate::catalog::CatalogRequest {
            provider: "fake".into(),
            query: CatalogQuery::default(),
        })
        .expect("catalog query");

    let resolver = SessionResolver {
        sessions: sessions.clone(),
    };
    resolver.resolve("item-42").expect("resource resolve");

    assert_eq!(source.acquire_calls.load(Ordering::SeqCst), 1);
    assert_eq!(source.refresh_calls.load(Ordering::SeqCst), 0);
}

fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
        .min(u64::MAX as u128) as u64
}
