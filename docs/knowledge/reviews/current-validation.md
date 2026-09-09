# Current Validation

## Backend local-core + package + download-manager target

Target claim:

> SearchNow has a bounded, local-first Rust backend core for settings, Minecraft storage discovery, local library indexing, read-only package inspection, and a transport-agnostic persistent download lifecycle manager separated from Tauri IPC.

Repository/CI evidence on `develop`:

- repository contract: PASS;
- RustCore format: PASS;
- RustCore compile/tests: PASS — **25 tests, 0 failures**;
- RustCore clippy with warnings denied: PASS;
- Tauri adapter format: PASS;
- frontend architecture/source-size/type/build gates: PASS;
- package fixtures cover folder, `.mcpack`, `.mcaddon`, BP→RP UUID dependency, duplicate UUID, and archive path-traversal rejection;
- download fixtures cover bounded concurrency, monotonic progress, cooperative cancellation, retryability, interrupted-job recovery, staged persistence, destination traversal rejection, and no-overwrite atomic publication.

Package inspection remains read-only. The download manager does not perform network I/O and does not persist auth headers, tokens, signed URLs, or provider credentials; persisted source identity is limited to a transport key plus non-sensitive resource id.

Runtime claims **not** established by hosted CI:

- real Windows AppData discovery;
- real Minecraft account-scoped directory behavior;
- Tauri IPC execution on an installed Windows app;
- scan performance against a large real library;
- compatibility across a representative set of real-world `.mcpack` / `.mcaddon` files;
- download persistence/finalization behavior on representative Windows filesystems and user destination folders;
- any real remote transport/network behavior.

These remain TARGET_WINDOWS/REAL_FIXTURE evidence.
