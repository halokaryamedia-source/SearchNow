use super::{ProviderCapabilities, ProviderRuntimeStatus};
use crate::{
    catalog::{CatalogProvider, CatalogProviderRegistry, CatalogService},
    download::{ResourceResolver, ResourceResolverRegistry},
    error::{BackendError, BackendResult},
    provider_identity::valid_provider_key,
    provider_session::{ProviderSessionManager, ProviderSessionRegistry, ProviderSessionSource},
};
use std::{collections::HashSet, sync::Arc};

pub trait IntegratedProvider: Send + Sync {
    fn provider_key(&self) -> &str;

    fn session_source(&self) -> Option<Arc<dyn ProviderSessionSource>> {
        None
    }

    fn catalog_provider(
        &self,
        _sessions: Arc<ProviderSessionManager>,
    ) -> BackendResult<Option<Arc<dyn CatalogProvider>>> {
        Ok(None)
    }

    fn resource_resolver(
        &self,
        _sessions: Arc<ProviderSessionManager>,
    ) -> BackendResult<Option<Arc<dyn ResourceResolver>>> {
        Ok(None)
    }
}

pub struct ProviderAdapterRuntime {
    sessions: Arc<ProviderSessionManager>,
    catalog: Arc<CatalogService>,
    resolvers: Arc<ResourceResolverRegistry>,
    capabilities: Vec<ProviderCapabilities>,
}

impl ProviderAdapterRuntime {
    pub fn compose(providers: Vec<Arc<dyn IntegratedProvider>>) -> BackendResult<Self> {
        let mut seen = HashSet::with_capacity(providers.len());
        let mut session_registry = ProviderSessionRegistry::new();
        let mut session_flags = Vec::with_capacity(providers.len());

        for provider in &providers {
            let key = validate_key(provider.provider_key())?;
            if !seen.insert(key.to_string()) {
                return Err(BackendError::new(
                    "provider_adapter_duplicate",
                    "A provider adapter with the same key is already registered.",
                ));
            }

            let has_session = match provider.session_source() {
                Some(source) => {
                    ensure_component_key(key, source.provider_key(), "session")?;
                    session_registry.register(source)?;
                    true
                }
                None => false,
            };
            session_flags.push((key.to_string(), has_session));
        }

        let sessions = Arc::new(ProviderSessionManager::new(session_registry));
        let mut catalog_registry = CatalogProviderRegistry::new();
        let mut resolver_registry = ResourceResolverRegistry::new();
        let mut capabilities = Vec::with_capacity(providers.len());

        for (provider, (key, has_session)) in providers.iter().zip(session_flags) {
            let catalog = provider
                .catalog_provider(sessions.clone())
                .map_err(|_| component_build_error("catalog"))?;
            let has_catalog = match catalog {
                Some(component) => {
                    ensure_component_key(&key, component.key(), "catalog")?;
                    catalog_registry.register(component)?;
                    true
                }
                None => false,
            };

            let resolver = provider
                .resource_resolver(sessions.clone())
                .map_err(|_| component_build_error("resolver"))?;
            let has_resolver = match resolver {
                Some(component) => {
                    ensure_component_key(&key, component.provider_key(), "resolver")?;
                    resolver_registry.register(component)?;
                    true
                }
                None => false,
            };

            capabilities.push(ProviderCapabilities {
                provider: key,
                session: has_session,
                catalog: has_catalog,
                resolved_download: has_resolver,
            });
        }

        Ok(Self {
            sessions,
            catalog: Arc::new(CatalogService::new(Arc::new(catalog_registry))),
            resolvers: Arc::new(resolver_registry),
            capabilities,
        })
    }

    pub fn sessions(&self) -> Arc<ProviderSessionManager> {
        self.sessions.clone()
    }

    pub fn catalog(&self) -> Arc<CatalogService> {
        self.catalog.clone()
    }

    pub fn resolvers(&self) -> Arc<ResourceResolverRegistry> {
        self.resolvers.clone()
    }

    pub fn status(&self) -> Vec<ProviderRuntimeStatus> {
        self.capabilities
            .iter()
            .cloned()
            .map(|capabilities| ProviderRuntimeStatus {
                session: capabilities
                    .session
                    .then(|| self.sessions.status(&capabilities.provider)),
                capabilities,
            })
            .collect()
    }
}

fn validate_key(key: &str) -> BackendResult<&str> {
    let key = key.trim();
    if !valid_provider_key(key) {
        return Err(BackendError::new(
            "provider_adapter_key_invalid",
            "Provider adapter key is empty or unsupported.",
        ));
    }
    Ok(key)
}

fn ensure_component_key(expected: &str, actual: &str, component: &str) -> BackendResult<()> {
    if expected == actual.trim() {
        return Ok(());
    }
    Err(BackendError::new(
        "provider_adapter_component_key_mismatch",
        format!("Provider {component} component key does not match its adapter key."),
    ))
}

fn component_build_error(component: &str) -> BackendError {
    BackendError::new(
        "provider_adapter_component_build_failed",
        format!("Provider {component} component could not be constructed."),
    )
}
