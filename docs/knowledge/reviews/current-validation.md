# Current Validation

## Backend local-core + package + transport + resolver + catalog target

Target claim:

> SearchNow has a bounded local-first Rust backend for settings, Minecraft storage discovery, local library indexing, read-only package inspection, persistent download execution, public HTTPS, credential-safe runtime resolution, and a provider-neutral bounded catalog domain separated from Tauri IPC.

Repository/CI evidence on `develop`:

- repository contract: PASS;
- RustCore format: PASS;
- RustCore compile/tests: PASS — **48 tests, 0 failures**;
- RustCore clippy with warnings denied: PASS;
- Tauri adapter format: PASS;
- frontend architecture/source-size/type/build gates: PASS;
- package fixtures cover folder, `.mcpack`, `.mcaddon`, BP→RP UUID dependency, duplicate UUID, and archive path-traversal rejection;
- download fixtures cover bounded concurrency, monotonic progress, cooperative cancellation, retryability, interrupted-job recovery, staged persistence, destination traversal rejection, and no-overwrite atomic publication;
- HTTP/resolver fixtures cover public transport bounds, expired-material refresh, explicit retry re-resolution, signed query/header use without persistence, and secret-safe failures;
- catalog fixtures prove request validation before provider invocation, filter/sort/cursor pagination, missing-provider handling, malformed provider data rejection, provider-failure sanitization, and catalog-item → existing download-source mapping;
- catalog domain DTO ownership is included in repository credential-field guards;
- architecture validation requires catalog model/provider source boundaries;
- Tauri bootstrap still owns one `DownloadExecutionRuntime`; no catalog provider or catalog IPC is registered yet because this slice establishes backend domain contracts first.

Catalog boundary established by CI:

- default page size is 30 and hard page bound is 100;
- query/provider/cursor/filter fields are bounded before provider calls;
- provider result count, identifiers, descriptive text and tags are bounded after provider calls;
- duplicate item ids and malformed downloadable identities fail closed;
- provider failures reduce to stable safe code + generic message;
- public downloadable items reuse `https-public` rules and reject signed query material;
- provider downloadable items reuse stable `ProviderResourceRef` / `provider-resolved` identity;
- catalog DTOs do not own auth headers, access/bearer tokens, cookies, signed URLs, or runtime header collections.

Claims **not** established by hosted CI:

- real Windows AppData/Minecraft account-scoped behavior;
- Tauri IPC execution in an installed Windows build;
- performance against large real libraries/packages;
- publication behavior across representative Windows destination filesystems;
- production HTTPS/TLS reliability against representative servers/CDNs;
- any real provider authentication/session lifecycle;
- any real remote catalog provider/API compatibility;
- provider-specific query semantics or resolved-resource behavior.

These remain TARGET_WINDOWS / REAL_FIXTURE / NETWORK / PROVIDER evidence.
