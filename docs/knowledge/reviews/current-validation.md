# Current Validation

## Backend local-core + package-inspection target

Target claim:

> SearchNow has a bounded, local-first Rust backend core for settings, Minecraft storage discovery, local library indexing, and read-only package inspection, separated from Tauri IPC.

Repository/CI evidence on `develop`:

- repository contract: PASS;
- RustCore format: PASS;
- RustCore compile/tests: PASS — **15 tests, 0 failures**;
- RustCore clippy with warnings denied: PASS;
- Tauri adapter format: PASS;
- frontend architecture/source-size/type/build gates: PASS;
- package fixtures cover folder, `.mcpack`, `.mcaddon`, BP→RP UUID dependency, duplicate UUID, and archive path-traversal rejection.

Package inspection is read-only: the archive path is inspected through ZIP metadata/manifest reads and is not extracted or rewritten.

Runtime claims **not** established by hosted CI:

- real Windows AppData discovery;
- real Minecraft account-scoped directory behavior;
- Tauri IPC execution on an installed Windows app;
- scan performance against a large real library;
- compatibility across a representative set of real-world `.mcpack` / `.mcaddon` files.

These remain TARGET_WINDOWS/REAL_FIXTURE evidence.
