# EngineData

`EngineData` is the implementation boundary for SearchNow.

## Current architecture

```text
Frontend/RustApp
→ Tauri 2 desktop application
→ Svelte 5 + Vite + TypeScript presentation/application layer
→ Rust Tauri commands + reusable Rust engine
```

SearchNow intentionally starts without a Python/local-worker backend. Filesystem, Minecraft discovery, catalog communication, download management, package inspection, validation, settings, and local persistence should stay in Rust unless a future requirement proves that a separate runtime is necessary.

## Ownership rule

```text
Svelte pages/components
→ presentation and transient UI state

src/app/bridge
→ product-facing facade + thin Tauri command API

src-tauri/src/commands
→ native IPC boundary only

src-tauri/src/engine
→ reusable application/runtime truth
```

Do not create parallel APIs, duplicate runtime state, or a second backend process merely for convenience.
