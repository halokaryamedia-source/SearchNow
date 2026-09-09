# Current Validation

## Backend local-core + package + transport + resolver + catalog + session + provider-composition target

Target claim:

> SearchNow has a bounded local-first Rust backend plus a provider-neutral integrated-provider composition that shares runtime-only session state across catalog and resolved downloads, while keeping provider credentials out of public/persisted DTOs and Tauri IPC ownership.

Repository/CI evidence on `develop`:

- repository contract: PASS;
- RustCore format: PASS;
- RustCore compile/tests: PASS — **56 tests, 0 failures**;
- RustCore clippy with warnings denied: PASS;
- Tauri adapter format: PASS;
- frontend architecture/source-size/type/build gates: PASS;
- package/download/HTTP/resolver/catalog/session regression fixtures remain passing;
- provider composition rejects mismatched component keys;
- integrated fake provider proves catalog → stable provider resource → shared session → resource resolver → authenticated runtime HTTP → completed atomic file publication;
- catalog + resolver reuse one provider-session acquisition in the integrated flow;
- integrated fixture verifies provider runtime secret material is absent from catalog JSON, provider status JSON, and persisted download state.

Provider-adapter boundary established by CI:

- `IntegratedProvider` is the single provider composition contract;
- one canonical adapter key must match any contributed session source, catalog provider and resource resolver key;
- duplicate/mismatched provider identities fail closed during composition;
- one `ProviderSessionManager` is constructed before catalog/resolver components and injected into both;
- provider capability metadata exposes only provider/session/catalog/resolved-download availability plus safe session status;
- provider component construction failures are normalized by SearchNow rather than exposing provider details;
- catalog items still map to the existing `provider-resolved` download model rather than creating a parallel provider download lifecycle;
- ephemeral Authorization material is generated only during runtime resolution/HTTP and is absent from persisted/public provider DTOs;
- final file publication remains owned by `DownloadExecutionRuntime`.

Claims **not** established by hosted CI:

- real Windows AppData/Minecraft account-scoped behavior;
- Tauri IPC execution in an installed Windows build;
- performance against large real libraries/packages;
- production HTTPS/TLS reliability against representative servers/CDNs;
- any real provider login/acquire/refresh endpoint;
- provider-specific token expiry/invalidation semantics;
- any real remote catalog provider/API compatibility;
- provider-specific terms/permissions and production download behavior;
- secure OS credential storage, if a future provider requires durable user credentials.

These remain TARGET_WINDOWS / REAL_FIXTURE / NETWORK / PROVIDER evidence.
