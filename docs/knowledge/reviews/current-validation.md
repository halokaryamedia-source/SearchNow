# Current Validation

## Backend local-core + package + transport + resolver + catalog + session target

Target claim:

> SearchNow has a bounded local-first Rust backend for local/package workflows, persistent download execution, public HTTPS, credential-safe resource resolution, provider-neutral catalog queries, and a shared runtime-only provider session owner separated from Tauri IPC.

Repository/CI evidence on `develop`:

- repository contract: PASS;
- RustCore format: PASS;
- RustCore compile/tests: PASS — **54 tests, 0 failures**;
- RustCore clippy with warnings denied: PASS;
- Tauri adapter format: PASS;
- frontend architecture/source-size/type/build gates: PASS;
- package/download/HTTP/resolver/catalog regression fixtures remain passing;
- provider-session fixtures prove initial acquire, valid-session reuse, expiry refresh, concurrent successful refresh deduplication, shared failed-refresh waves, sanitized refresh failure, missing-provider state, and shared CatalogProvider/ResourceResolver session ownership;
- public `ProviderSessionStatus` JSON contains no runtime secret material;
- architecture/repository guards require provider-session source ownership and reject Serialize/Deserialize/Debug on secret-bearing runtime carriers.

Provider-session boundary established by CI:

- `ProviderSessionMaterial` and `ProviderSessionLease` are runtime-only and non-serializable;
- secret-bearing runtime carriers intentionally do not implement Debug;
- session payload is opaque provider-defined `Any + Send + Sync` state;
- valid material is reused instead of repeatedly acquiring credentials;
- expired material refreshes through one source-owned refresh path;
- concurrent callers wait on one refresh instead of launching duplicate refresh work;
- a failed refresh result is shared with callers already waiting on that wave, preventing retry storms; a later independent acquire may retry when the failure is retryable;
- provider failure details are reduced to stable code + generic message;
- only safe status state/expiry/failure-code/retryability metadata is serializable;
- CatalogProvider and ResourceResolver can share one `Arc<ProviderSessionManager>`.

Claims **not** established by hosted CI:

- real Windows AppData/Minecraft account-scoped behavior;
- Tauri IPC execution in an installed Windows build;
- performance against large real libraries/packages;
- production HTTPS/TLS reliability against representative servers/CDNs;
- any real provider login/acquire/refresh endpoint;
- provider-specific token expiry/invalidation semantics;
- any real remote catalog provider/API compatibility;
- secure OS credential storage, if a future provider requires durable user credentials.

These remain TARGET_WINDOWS / REAL_FIXTURE / NETWORK / PROVIDER evidence.
