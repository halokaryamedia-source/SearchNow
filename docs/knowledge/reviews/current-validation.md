# Current Validation

## Backend local-core + package + HTTP transport target

Target claim:

> SearchNow has a bounded, local-first Rust backend core for settings, Minecraft storage discovery, local library indexing, read-only package inspection, persistent download lifecycle management, provider-neutral transport execution, and a bounded public-HTTPS transport separated from Tauri IPC.

Repository/CI evidence on `develop`:

- repository contract: PASS;
- RustCore format: PASS;
- RustCore compile/tests: PASS — **37 tests, 0 failures**;
- RustCore clippy with warnings denied: PASS;
- Tauri adapter format: PASS;
- frontend architecture/source-size/type/build gates: PASS;
- package fixtures cover folder, `.mcpack`, `.mcaddon`, BP→RP UUID dependency, duplicate UUID, and archive path-traversal rejection;
- download-manager fixtures cover bounded concurrency, monotonic progress, cooperative cancellation, retryability, interrupted-job recovery, staged persistence, destination traversal rejection, and no-overwrite atomic publication;
- transport-execution fixtures cover local-file end-to-end publication, scheduler concurrency, cooperative active cancellation, and unavailable-transport failure;
- HTTP fixtures cover public policy rejection of plain HTTP/query-bearing persisted URLs, ordinary streaming, relative redirects, declared oversize rejection, Content-Length truncation, stalled body-read timeout through the executor, and cooperative active cancellation;
- Tauri bootstrap creates and manages one `DownloadExecutionRuntime` and registers both `local-file` and `https-public`; download commands delegate to that RustCore owner.

HTTP timeout behavior has an explicit ownership split: the client retains connect/read socket timeouts while SearchNow enforces the overall transfer deadline in its own stream wrapper. This avoids `ureq 2.x` request-level timeout precedence from weakening the shorter stalled-read timeout.

Queue persistence still stores transport/resource identity only. It does not persist auth headers, bearer tokens, cookies, signed URLs, or provider secrets. `https-public` additionally rejects query-bearing persisted URLs and embedded URL credentials so authenticated/signed request material must later be resolved only at runtime.

Runtime claims **not** established by hosted CI:

- real Windows AppData discovery;
- real Minecraft account-scoped directory behavior;
- Tauri IPC execution on an installed Windows app;
- scan performance against a large real library;
- compatibility across a representative set of real-world `.mcpack` / `.mcaddon` files;
- download persistence/finalization behavior on representative Windows filesystems and user destination folders;
- cancellation timing under real slow/unstable internet I/O;
- production HTTPS/TLS behavior against representative public remote servers/CDNs;
- provider authentication, runtime resource resolution, catalog APIs, or provider-specific endpoint behavior.

These remain TARGET_WINDOWS/REAL_FIXTURE/NETWORK evidence.
