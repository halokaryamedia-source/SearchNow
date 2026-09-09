use crate::{
    build_local_backend_snapshot,
    catalog::{CatalogError, CatalogPage, CatalogRequest},
    diagnostics::{
        BackendDiagnosticsSnapshot, DiagnosticComponent, DiagnosticSeverity, DiagnosticsBuffer,
    },
    download::{
        default_download_paths, DownloadExecutionRuntime, DownloadJob, DownloadManagerSnapshot,
        DownloadPolicy, DownloadRequest, DownloadStore, DownloadTransportRegistry, HttpTransport,
        HttpTransportPolicy, ProviderResolvedTransport,
    },
    error::BackendResult,
    minecraft::{discover_minecraft_storage, MinecraftDiscoverySnapshot},
    package::{inspect_package, PackageInspection},
    platform::PlatformContext,
    provider_adapter::{IntegratedProvider, ProviderAdapterRuntime, ProviderRuntimeStatus},
    runtime::{runtime_status, RuntimeStatus},
    settings::{AppSettings, SettingsStore},
    LocalBackendSnapshot,
};
use serde::Serialize;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
    time::Instant,
};

#[derive(Debug, Clone)]
pub struct SearchNowBackendPaths {
    pub settings_path: PathBuf,
    pub download_state_path: PathBuf,
    pub download_workspace_root: PathBuf,
    pub download_destination_root: PathBuf,
}

impl SearchNowBackendPaths {
    pub fn from_roots(config_root: impl AsRef<Path>, data_root: impl AsRef<Path>) -> Self {
        let (download_state_path, download_workspace_root, download_destination_root) =
            default_download_paths(data_root.as_ref());
        Self {
            settings_path: config_root.as_ref().join("settings.json"),
            download_state_path,
            download_workspace_root,
            download_destination_root,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BackendRuntimeSnapshot {
    pub runtime: RuntimeStatus,
    pub minecraft: MinecraftDiscoverySnapshot,
    pub providers: Vec<ProviderRuntimeStatus>,
    pub downloads: DownloadManagerSnapshot,
    pub diagnostics: BackendDiagnosticsSnapshot,
}

#[derive(Clone)]
pub struct SearchNowBackendRuntime {
    settings: SettingsStore,
    platform: PlatformContext,
    providers: Arc<ProviderAdapterRuntime>,
    downloads: DownloadExecutionRuntime,
    diagnostics: DiagnosticsBuffer,
}

impl SearchNowBackendRuntime {
    pub fn new(
        paths: SearchNowBackendPaths,
        platform: PlatformContext,
        integrated_providers: Vec<Arc<dyn IntegratedProvider>>,
    ) -> BackendResult<Self> {
        let http = HttpTransport::new(HttpTransportPolicy::default())?;
        Self::compose(paths, platform, integrated_providers, http)
    }

    fn compose(
        paths: SearchNowBackendPaths,
        platform: PlatformContext,
        integrated_providers: Vec<Arc<dyn IntegratedProvider>>,
        http: HttpTransport,
    ) -> BackendResult<Self> {
        let diagnostics = DiagnosticsBuffer::default();
        let startup_started = Instant::now();
        diagnostics.record(
            DiagnosticComponent::Runtime,
            DiagnosticSeverity::Info,
            "backend_runtime_starting",
            "SearchNow backend runtime is starting.",
            None,
        );

        let provider_started = Instant::now();
        let providers = Arc::new(ProviderAdapterRuntime::compose(integrated_providers)?);
        diagnostics.record(
            DiagnosticComponent::Provider,
            DiagnosticSeverity::Info,
            "provider_runtime_ready",
            "Provider runtime composition is ready.",
            Some(provider_started.elapsed()),
        );

        let mut transports = DownloadTransportRegistry::with_local_file()?;
        transports.register(Arc::new(http.clone()))?;
        transports.register(Arc::new(ProviderResolvedTransport::new(
            providers.resolvers(),
            http,
        )))?;

        let download_started = Instant::now();
        let downloads = DownloadExecutionRuntime::new(
            DownloadPolicy::default(),
            DownloadStore::new(&paths.download_state_path),
            &paths.download_workspace_root,
            &paths.download_destination_root,
            transports,
        )?;
        diagnostics.record(
            DiagnosticComponent::Download,
            DiagnosticSeverity::Info,
            "download_runtime_ready",
            "Download runtime is ready.",
            Some(download_started.elapsed()),
        );

        diagnostics.mark_ready();
        diagnostics.record(
            DiagnosticComponent::Runtime,
            DiagnosticSeverity::Info,
            "backend_runtime_ready",
            "SearchNow backend runtime is ready.",
            Some(startup_started.elapsed()),
        );

        Ok(Self {
            settings: SettingsStore::new(paths.settings_path),
            platform,
            providers,
            downloads,
            diagnostics,
        })
    }

    pub fn load_settings(&self) -> BackendResult<AppSettings> {
        let started = Instant::now();
        let result = self.settings.load();
        self.diagnostics.record_outcome(
            DiagnosticComponent::Settings,
            started,
            result.is_ok(),
            "settings_load_ok",
            "Settings loaded successfully.",
            "settings_load_failed",
            "Settings could not be loaded.",
            DiagnosticSeverity::Error,
        );
        result
    }

    pub fn save_settings(&self, settings: &AppSettings) -> BackendResult<AppSettings> {
        let started = Instant::now();
        let result = self.settings.save(settings).map(|()| settings.clone());
        self.diagnostics.record_outcome(
            DiagnosticComponent::Settings,
            started,
            result.is_ok(),
            "settings_save_ok",
            "Settings saved successfully.",
            "settings_save_failed",
            "Settings could not be saved.",
            DiagnosticSeverity::Error,
        );
        result
    }

    pub fn discover_minecraft(&self) -> BackendResult<MinecraftDiscoverySnapshot> {
        let started = Instant::now();
        let result = self.discover_minecraft_raw();
        self.diagnostics.record_outcome(
            DiagnosticComponent::Minecraft,
            started,
            result.is_ok(),
            "minecraft_discovery_ok",
            "Minecraft storage discovery completed.",
            "minecraft_discovery_failed",
            "Minecraft storage discovery could not complete.",
            DiagnosticSeverity::Error,
        );
        result
    }

    pub fn scan_local_library(&self) -> BackendResult<LocalBackendSnapshot> {
        let started = Instant::now();
        let result = self.scan_local_library_raw();
        self.diagnostics.record_outcome(
            DiagnosticComponent::Library,
            started,
            result.is_ok(),
            "library_scan_ok",
            "Local library scan completed.",
            "library_scan_failed",
            "Local library scan could not complete.",
            DiagnosticSeverity::Error,
        );
        result
    }

    pub fn inspect_package(&self, path: &Path) -> BackendResult<PackageInspection> {
        let started = Instant::now();
        let result = inspect_package(path);
        self.diagnostics.record_outcome(
            DiagnosticComponent::Package,
            started,
            result.is_ok(),
            "package_inspection_ok",
            "Package inspection completed.",
            "package_inspection_failed",
            "Package inspection could not complete.",
            DiagnosticSeverity::Warning,
        );
        result
    }

    pub fn query_catalog(&self, request: &CatalogRequest) -> Result<CatalogPage, CatalogError> {
        let started = Instant::now();
        let result = self.providers.catalog().query(request);
        self.diagnostics.record_outcome(
            DiagnosticComponent::Catalog,
            started,
            result.is_ok(),
            "catalog_query_ok",
            "Catalog query completed.",
            "catalog_query_failed",
            "Catalog query could not complete.",
            DiagnosticSeverity::Warning,
        );
        result
    }

    pub fn provider_status(&self) -> Vec<ProviderRuntimeStatus> {
        self.providers.status()
    }

    pub fn runtime_status(&self) -> RuntimeStatus {
        runtime_status()
    }

    pub fn diagnostics_snapshot(&self) -> BackendDiagnosticsSnapshot {
        self.diagnostics.snapshot()
    }

    pub fn snapshot(&self) -> BackendResult<BackendRuntimeSnapshot> {
        let started = Instant::now();
        let result = (|| {
            Ok(BackendRuntimeSnapshot {
                runtime: self.runtime_status(),
                minecraft: self.discover_minecraft_raw()?,
                providers: self.provider_status(),
                downloads: self.downloads.snapshot()?,
                diagnostics: self.diagnostics_snapshot(),
            })
        })();
        self.diagnostics.record_outcome(
            DiagnosticComponent::Runtime,
            started,
            result.is_ok(),
            "backend_snapshot_ok",
            "Backend runtime snapshot completed.",
            "backend_snapshot_failed",
            "Backend runtime snapshot could not complete.",
            DiagnosticSeverity::Error,
        );
        result
    }

    pub fn download_snapshot(&self) -> BackendResult<DownloadManagerSnapshot> {
        self.downloads.snapshot()
    }

    pub fn queue_download(&self, request: DownloadRequest) -> BackendResult<DownloadJob> {
        let started = Instant::now();
        let result = self.downloads.queue(request);
        self.diagnostics.record_outcome(
            DiagnosticComponent::Download,
            started,
            result.is_ok(),
            "download_queue_ok",
            "Download job queued.",
            "download_queue_failed",
            "Download job could not be queued.",
            DiagnosticSeverity::Warning,
        );
        result
    }

    pub fn cancel_download(&self, job_id: &str) -> BackendResult<DownloadJob> {
        let started = Instant::now();
        let result = self.downloads.cancel(job_id);
        self.diagnostics.record_outcome(
            DiagnosticComponent::Download,
            started,
            result.is_ok(),
            "download_cancel_ok",
            "Download cancellation requested.",
            "download_cancel_failed",
            "Download cancellation could not be requested.",
            DiagnosticSeverity::Warning,
        );
        result
    }

    pub fn retry_download(&self, job_id: &str) -> BackendResult<DownloadJob> {
        let started = Instant::now();
        let result = self.downloads.retry(job_id);
        self.diagnostics.record_outcome(
            DiagnosticComponent::Download,
            started,
            result.is_ok(),
            "download_retry_ok",
            "Download retry queued.",
            "download_retry_failed",
            "Download retry could not be queued.",
            DiagnosticSeverity::Warning,
        );
        result
    }

    pub fn remove_download(&self, job_id: &str) -> BackendResult<DownloadManagerSnapshot> {
        let started = Instant::now();
        let result = self.downloads.remove_terminal(job_id);
        self.diagnostics.record_outcome(
            DiagnosticComponent::Download,
            started,
            result.is_ok(),
            "download_remove_ok",
            "Terminal download job removed.",
            "download_remove_failed",
            "Terminal download job could not be removed.",
            DiagnosticSeverity::Warning,
        );
        result
    }

    fn discover_minecraft_raw(&self) -> BackendResult<MinecraftDiscoverySnapshot> {
        let settings = self.settings.load()?;
        Ok(discover_minecraft_storage(
            &settings.minecraft,
            &self.platform,
        ))
    }

    fn scan_local_library_raw(&self) -> BackendResult<LocalBackendSnapshot> {
        let settings = self.settings.load()?;
        Ok(build_local_backend_snapshot(&settings, &self.platform))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        diagnostics::{BackendStartupPhase, DiagnosticSeverity},
        download::{
            provider_download_source, DownloadJobState, HttpTransportPolicy,
            ProviderResolveFailure, ResolvedResource, ResourceResolver,
        },
        error::BackendResult,
    };
    use std::{
        io::{Read, Write},
        net::{TcpListener, TcpStream},
        sync::Arc,
        thread,
        time::{Duration, Instant},
    };

    struct InvalidProvider;

    impl IntegratedProvider for InvalidProvider {
        fn provider_key(&self) -> &str {
            "invalid provider key"
        }
    }

    struct ResolverProvider {
        base_url: String,
    }

    impl IntegratedProvider for ResolverProvider {
        fn provider_key(&self) -> &str {
            "runtime-fixture"
        }

        fn resource_resolver(
            &self,
            _sessions: Arc<crate::provider_session::ProviderSessionManager>,
        ) -> BackendResult<Option<Arc<dyn ResourceResolver>>> {
            Ok(Some(Arc::new(LoopbackResolver {
                base_url: self.base_url.clone(),
            })))
        }
    }

    struct LoopbackResolver {
        base_url: String,
    }

    impl ResourceResolver for LoopbackResolver {
        fn provider_key(&self) -> &str {
            "runtime-fixture"
        }

        fn resolve(&self, resource_id: &str) -> Result<ResolvedResource, ProviderResolveFailure> {
            if resource_id != "asset" {
                return Err(ProviderResolveFailure::new(
                    "fixture_resource_missing",
                    false,
                ));
            }
            Ok(ResolvedResource::new(format!("{}/asset", self.base_url)))
        }
    }

    #[test]
    fn runtime_snapshot_is_safe_and_consistent_without_providers() {
        let temp = tempfile::tempdir().expect("tempdir");
        let paths =
            SearchNowBackendPaths::from_roots(temp.path().join("config"), temp.path().join("data"));
        let runtime = SearchNowBackendRuntime::compose(
            paths,
            PlatformContext::windows(temp.path().join("roaming"), temp.path().join("local")),
            Vec::new(),
            HttpTransport::new_test_http(test_http_policy()).expect("test HTTP"),
        )
        .expect("runtime");

        let snapshot = runtime.snapshot().expect("snapshot");
        assert!(snapshot.runtime.app_ready);
        assert!(snapshot.providers.is_empty());
        assert_eq!(snapshot.downloads.active_jobs, 0);
        assert_eq!(snapshot.downloads.queued_jobs, 0);
        assert_eq!(
            snapshot.diagnostics.health.startup_phase,
            BackendStartupPhase::Ready
        );
    }

    #[test]
    fn composed_provider_resolver_is_used_by_application_download_runtime() {
        let payload = b"searchnow-backend-runtime".repeat(1024);
        let base_url = spawn_server(payload.clone());
        let temp = tempfile::tempdir().expect("tempdir");
        let paths =
            SearchNowBackendPaths::from_roots(temp.path().join("config"), temp.path().join("data"));
        let destination = paths.download_destination_root.clone();
        let runtime = SearchNowBackendRuntime::compose(
            paths,
            PlatformContext::windows(temp.path().join("roaming"), temp.path().join("local")),
            vec![Arc::new(ResolverProvider { base_url })],
            HttpTransport::new_test_http(test_http_policy()).expect("test HTTP"),
        )
        .expect("runtime");

        let status = runtime.provider_status();
        assert_eq!(status.len(), 1);
        assert!(status[0].capabilities.resolved_download);

        runtime
            .queue_download(DownloadRequest {
                source: provider_download_source("runtime-fixture", "asset")
                    .expect("provider source"),
                display_name: "Runtime Fixture".into(),
                destination_file_name: "runtime.mcpack".into(),
                expected_bytes: Some(payload.len() as u64),
            })
            .expect("queue");

        let snapshot = wait_for_terminal(&runtime);
        assert_eq!(snapshot.jobs[0].state, DownloadJobState::Completed);
        assert_eq!(
            std::fs::read(destination.join("runtime.mcpack")).expect("final file"),
            payload
        );
    }

    #[test]
    fn invalid_provider_prevents_application_runtime_construction() {
        let temp = tempfile::tempdir().expect("tempdir");
        let paths =
            SearchNowBackendPaths::from_roots(temp.path().join("config"), temp.path().join("data"));
        let result = SearchNowBackendRuntime::compose(
            paths,
            PlatformContext::windows(temp.path().join("roaming"), temp.path().join("local")),
            vec![Arc::new(InvalidProvider)],
            HttpTransport::new_test_http(test_http_policy()).expect("test HTTP"),
        );
        let error = match result {
            Err(error) => error,
            Ok(_) => panic!("invalid provider must prevent runtime construction"),
        };
        assert_eq!(error.code(), "provider_adapter_key_invalid");
    }

    #[test]
    fn failed_package_diagnostic_does_not_expose_input_path() {
        let temp = tempfile::tempdir().expect("tempdir");
        let paths =
            SearchNowBackendPaths::from_roots(temp.path().join("config"), temp.path().join("data"));
        let runtime = SearchNowBackendRuntime::compose(
            paths,
            PlatformContext::windows(temp.path().join("roaming"), temp.path().join("local")),
            Vec::new(),
            HttpTransport::new_test_http(test_http_policy()).expect("test HTTP"),
        )
        .expect("runtime");
        let secret_path = temp.path().join("private-user-path-secret.mcaddon");
        assert!(runtime.inspect_package(&secret_path).is_err());
        let diagnostics = runtime.diagnostics_snapshot();
        assert!(diagnostics.events.iter().any(|event| {
            event.code == "package_inspection_failed"
                && event.severity == DiagnosticSeverity::Warning
        }));
        let json = serde_json::to_string(&diagnostics).expect("diagnostics json");
        assert!(!json.contains("private-user-path-secret"));
    }

    fn spawn_server(payload: Vec<u8>) -> String {
        let listener = TcpListener::bind("127.0.0.1:0").expect("bind fixture");
        let address = listener.local_addr().expect("fixture address");
        thread::spawn(move || {
            let Ok((mut stream, _)) = listener.accept() else {
                return;
            };
            let _ = read_request(&mut stream);
            let _ = write!(
                stream,
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                payload.len()
            );
            let _ = stream.write_all(&payload);
            let _ = stream.flush();
        });
        format!("http://{address}")
    }

    fn read_request(stream: &mut TcpStream) -> std::io::Result<()> {
        stream.set_read_timeout(Some(Duration::from_secs(1)))?;
        let mut buffer = [0_u8; 4096];
        let _ = stream.read(&mut buffer)?;
        Ok(())
    }

    fn test_http_policy() -> HttpTransportPolicy {
        HttpTransportPolicy {
            connect_timeout: Duration::from_secs(1),
            read_timeout: Duration::from_secs(1),
            overall_timeout: Duration::from_secs(5),
            max_redirects: 2,
            max_response_bytes: 2 * 1024 * 1024,
        }
    }

    fn wait_for_terminal(runtime: &SearchNowBackendRuntime) -> DownloadManagerSnapshot {
        let started = Instant::now();
        loop {
            let snapshot = runtime.download_snapshot().expect("download snapshot");
            if snapshot
                .jobs
                .first()
                .is_some_and(|job| job.state.is_terminal())
            {
                return snapshot;
            }
            assert!(
                started.elapsed() < Duration::from_secs(5),
                "application runtime download timed out"
            );
            thread::sleep(Duration::from_millis(10));
        }
    }
}
