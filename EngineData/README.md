# EngineData

`EngineData` is the implementation boundary for SearchNow.

## Current architecture

```text
Frontend/RustApp
├─ Svelte 5 + Vite + TypeScript presentation/application layer
└─ Tauri 2 shell + thin native command adapters
        ↓
Backend/RustCore
└─ SearchNowBackendRuntime + reusable Rust domain/runtime truth
```

SearchNow has no Python/local-worker backend. Filesystem access, Minecraft discovery, catalog/provider runtime, download management, package inspection, validation, settings, persistence, and diagnostics remain inside Rust unless a future requirement proves that a separate runtime is necessary and the architecture decision is explicitly revised.

## Ownership rule

```text
Frontend/RustApp/src/pages + components
→ presentation and transient UI state

Frontend/RustApp/src/app/bridge
→ product-facing facade + thin Tauri command API

Frontend/RustApp/src-tauri/src/commands
→ native IPC adaptation only

Backend/RustCore
→ reusable application/domain/runtime truth
```

`SearchNowBackendRuntime` is the single application backend owner. Do not create parallel APIs, duplicate runtime state in Svelte/Tauri commands, or a second backend process merely for convenience.

## Dependency rule

The frontend, standalone RustCore, and Tauri application each have committed dependency locks. Normal verification uses `npm ci` and Cargo `--locked`; dependency resolution changes must be explicit rather than incidental.

Hosted Windows compilation is repository evidence only. Actual target-machine behavior remains a separate local smoke phase.
