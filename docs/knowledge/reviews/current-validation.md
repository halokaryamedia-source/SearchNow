# Current Validation

## Backend local-core + package + HTTP + resource-resolver target

Target claim:

> SearchNow has a bounded local-first Rust backend for settings, Minecraft storage discovery, local library indexing, read-only package inspection, persistent download lifecycle management, provider-neutral execution, public HTTPS transport, and a credential-safe runtime provider resolver separated from Tauri IPC.

Repository/CI evidence on `develop`:

- repository contract: PASS;
- RustCore format: PASS;
- RustCore compile/tests: PASS — **42 tests, 0 failures**;
- RustCore clippy with warnings denied: PASS;
- Tauri adapter format: PASS;
- frontend architecture/source-size/type/build gates: PASS;
- package fixtures cover folder, `.mcpack`, `.mcaddon`, BP→RP UUID dependency, duplicate UUID, and archive path-traversal rejection;
- download fixtures cover bounded concurrency, monotonic progress, cooperative cancellation, retryability, interrupted-job recovery, staged persistence, destination traversal rejection, and no-overwrite atomic publication;
- transport fixtures cover local-file end-to-end publication, scheduler concurrency, cooperative cancellation, and unavailable transport;
- HTTP fixtures cover public HTTPS policy, ordinary streaming, relative redirects, oversize rejection, Content-Length truncation, stalled read timeout, and cancellation;
- resolver fixtures prove stable provider reference validation, missing-provider failure, expired-material refresh, explicit retry re-resolution, and use of ephemeral signed query + Authorization header without persistence;
- state persistence retains stable identity such as `fake:catalog-item-42` while fixture secrets are absent from `state.json`;
- architecture/repository guards prevent runtime credential fields from becoming persisted download DTO/store owners;
- Tauri bootstrap manages one `DownloadExecutionRuntime` and registers `local-file`, `https-public`, and the generic `provider-resolved` boundary.

Credential boundary established by CI:

- queue state persists transport + stable resource identity only;
- provider resource ids reject URL/query/fragment shapes;
- signed query/header material exists only in runtime resolver output;
- resolver errors are reduced to stable safe code + generic message;
- sensitive HTTP transport/read errors are sanitized;
- credential-bearing headers are rejected on cross-origin redirects;
- retry performs a new resolution rather than persisting/reusing previous ephemeral transfer material.

Claims **not** established by hosted CI:

- real Windows AppData/Minecraft account-scoped behavior;
- Tauri IPC execution in an installed Windows build;
- performance against large real libraries/packages;
- publication behavior across representative Windows destination filesystems;
- production HTTPS/TLS reliability against representative servers/CDNs;
- any real provider authentication/session lifecycle;
- real catalog API/provider endpoints;
- provider-specific resolved-resource behavior.

These remain TARGET_WINDOWS / REAL_FIXTURE / NETWORK / PROVIDER evidence.
