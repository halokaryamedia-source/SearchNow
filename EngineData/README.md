# EngineData

`EngineData` is the implementation boundary for SearchNow.

## Current architecture

```text
EngineData/
├── Backend/RustCore/
│   └── reusable application/runtime truth
└── Frontend/RustApp/
    ├── src/             Svelte presentation + product bridge
    └── src-tauri/src/   Tauri bootstrap + thin native IPC
```

`RustCore` is linked in-process by the Tauri crate. There is no Python/local-worker backend or second application runtime.

## Ownership rule

```text
Svelte pages/components
→ presentation and transient UI state

Frontend/RustApp/src/app/bridge
→ product-facing facade + thin Tauri command API

Frontend/RustApp/src-tauri/src/commands
→ native IPC boundary only

Backend/RustCore/src/app_runtime.rs
→ one application composition root

Backend/RustCore domain modules
→ Minecraft discovery, library/package inspection,
  provider/catalog/session/resolver behavior,
  downloads, persistence/recovery, diagnostics
```

Settings and download state share one internal atomic persistence implementation. Provider identities share one internal validation contract. Production download transport selection remains inside RustCore; the `local-file` transport is a deterministic test fixture, not a product IPC surface.

Do not create parallel APIs, duplicate runtime state, duplicate storage replacement logic, or a second backend process merely for convenience.
