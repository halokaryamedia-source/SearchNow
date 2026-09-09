# Current Validation

## Backend local-core + package + transport-execution target

Target claim:

> SearchNow has a bounded, local-first Rust backend core for settings, Minecraft storage discovery, local library indexing, read-only package inspection, persistent download lifecycle management, and transport execution separated from Tauri IPC.

Repository/CI evidence on `develop`:

- repository contract: PASS;
- RustCore format: PASS;
- RustCore compile/tests: PASS — **29 tests, 0 failures**;
- RustCore clippy with warnings denied: PASS;
- Tauri adapter format: PASS;
- frontend architecture/source-size/type/build gates: PASS;
- package fixtures cover folder, `.mcpack`, `.mcaddon`, BP→RP UUID dependency, duplicate UUID, and archive path-traversal rejection;
- download-manager fixtures cover bounded concurrency, monotonic progress, cooperative cancellation, retryability, interrupted-job recovery, staged persistence, destination traversal rejection, and no-overwrite atomic publication;
- transport-execution fixtures cover local-file end-to-end publication, scheduler concurrency, cooperative active cancellation, and unavailable-transport failure;
- Tauri bootstrap now creates and manages one `DownloadExecutionRuntime`; download commands delegate to that RustCore owner.

Transport execution remains provider-neutral. The only concrete transport currently registered by the desktop bootstrap is `local-file`; no HTTP/provider authentication is implemented in this slice.

Queue persistence still stores transport/resource identity only. It does not persist auth headers, bearer tokens, cookies, signed URLs, or provider secrets. Progress is persisted at lifecycle boundaries plus bounded checkpoints rather than on every transfer chunk.

Runtime claims **not** established by hosted CI:

- real Windows AppData discovery;
- real Minecraft account-scoped directory behavior;
- Tauri IPC execution on an installed Windows app;
- scan performance against a large real library;
- compatibility across a representative set of real-world `.mcpack` / `.mcaddon` files;
- download persistence/finalization behavior on representative Windows filesystems and user destination folders;
- cancellation timing under real slow/unstable remote I/O;
- any real HTTP/provider transport, authentication, catalog, or remote endpoint behavior.

These remain TARGET_WINDOWS/REAL_FIXTURE/NETWORK evidence.
