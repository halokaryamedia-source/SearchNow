# Current Validation

## Backend local-core target

Target claim:

> SearchNow has a bounded, local-first Rust backend core for settings, Minecraft storage discovery, and read-only local library indexing, separated from Tauri IPC.

Repository/CI proof required:

- repository and architecture contracts pass;
- RustCore format passes;
- RustCore unit tests pass;
- RustCore clippy passes with warnings denied;
- frontend static/type/build gates remain green;
- Tauri adapter format passes.

Runtime claims **not** established by hosted CI:

- real Windows AppData discovery;
- real Minecraft account-scoped directory behavior;
- Tauri IPC execution on installed Windows app;
- scan performance against a large real library.

These remain TARGET_WINDOWS evidence.
