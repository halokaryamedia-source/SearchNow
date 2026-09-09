# Current Validation

## Consolidated application backend runtime target

Target claim:

> SearchNow has one in-process application backend runtime that composes local settings/platform behavior, provider catalog/session/resolver composition, and persistent download execution while Tauri manages only that consolidated backend owner.

Repository/CI evidence on `develop`:

- repository contract: PASS;
- RustCore format: PASS;
- RustCore compile/tests: PASS — **59 tests, 0 failures**;
- RustCore clippy with warnings denied: PASS;
- Tauri adapter format: PASS;
- frontend architecture/source-size/type/build gates: PASS;
- package/download/HTTP/resolver/catalog/session/provider-adapter regression fixtures remain passing;
- `SearchNowBackendRuntime` startup succeeds with zero providers and emits a safe aggregate snapshot;
- invalid provider composition prevents application runtime construction;
- application-runtime fixture proves the resolver registry produced by provider composition is the registry used by actual provider-resolved download execution;
- all active Tauri feature commands delegate through `State<SearchNowBackendRuntime>`;
- Tauri bootstrap manages one consolidated backend state instead of separate download/provider/settings engines.

Application-runtime boundary established by CI:

- canonical paths for settings/download state/workspace/final files are assembled once for application construction;
- `ProviderAdapterRuntime` is composed before provider-resolved transport registration;
- `ProviderResolvedTransport` receives `providers.resolvers()` from that exact composed runtime;
- `DownloadExecutionRuntime` remains the only download lifecycle/persistence/finalization owner;
- `SettingsStore` and `DownloadStore` remain their existing persistence authorities; no second application database was introduced;
- safe `BackendRuntimeSnapshot` contains runtime status, Minecraft discovery, safe provider status/capabilities, and download summary only;
- filesystem-heavy local commands continue to use Tauri `spawn_blocking`; download transfer remains on the existing bounded worker threads;
- architecture validator rejects old per-command/sub-runtime ownership assumptions.

Claims **not** established by hosted CI:

- installed Windows Tauri execution;
- Tauri Rust compile/link on the actual Windows target;
- real Windows AppData/Minecraft account-scoped behavior;
- large-library/package performance on representative Windows machines;
- production HTTPS/TLS reliability against representative servers/CDNs;
- any real provider login/catalog/resource endpoint or provider-specific auth semantics;
- secure OS credential storage if a future provider requires durable user credentials.

These remain TARGET_WINDOWS / REAL_FIXTURE / NETWORK / PROVIDER evidence.
