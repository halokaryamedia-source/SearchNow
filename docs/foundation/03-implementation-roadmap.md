# 03 — Implementation Roadmap

## Objective

Build SearchNow as a small, maintainable Minecraft Bedrock content-management desktop application with clear ownership between presentation, native command boundaries, reusable runtime logic, local data, and network/filesystem adapters.

The old BlueCoin WinForms structure is evidence only; it is not the source architecture to recreate.

## Approved implementation shape

```text
SearchNow/
├── Cargo.toml / Cargo.lock
└── EngineData/
    ├── Backend/RustCore/
    │   └── src/
    │       ├── app_runtime.rs
    │       ├── persistence.rs
    │       ├── diagnostics.rs
    │       ├── minecraft.rs / library.rs / package/
    │       ├── catalog/
    │       ├── provider_session/
    │       ├── provider_adapter/
    │       └── download/
    └── Frontend/RustApp/
        ├── package-lock.json
        ├── src/
        │   ├── pages/
        │   ├── components/
        │   └── app/bridge/
        └── src-tauri/src/
            ├── main.rs
            ├── app_bootstrap.rs
            └── commands/
```

Architecture authority: `05-application-architecture.md`, `06-backend-architecture.md`, and the later focused foundation documents.

## Runtime rule

```text
Svelte UI
→ product facade
→ thin runtime API
→ Tauri command
→ one SearchNowBackendRuntime
→ RustCore domain/runtime owner
```

Keep SearchNow in one desktop process whenever practical. Do not add Python, a local HTTP server, or another worker process unless a concrete capability cannot be served cleanly by Rust and a new architecture decision is explicitly approved.

## Current phase status

### Phase 0 — Evidence / Development System

Status: **complete**

- legacy executable architecture recovered;
- product/safety boundaries documented;
- PRD-Creator-style repository workflow established.

### Phase 1 — Application Architecture Scaffold

Status: **complete at repository/static level**

- Tauri/Svelte/Rust stack established;
- four product surfaces scaffolded;
- frontend bridge boundary established;
- RustCore separated from thin Tauri IPC;
- one `SearchNowBackendRuntime` is the application backend owner.

Installed Windows execution remains a separate evidence boundary.

### Phase 2 — Minecraft Discovery + Library

Status: **backend foundation complete; product wiring intentionally partial**

Implemented:

- Windows Bedrock GDK shared/account discovery;
- Preview opt-in;
- legacy UWP fallback;
- bounded local behavior/resource/skin pack and world indexing;
- settings-owned discovery policy;
- filesystem work kept outside Svelte.

Future UI work must consume these existing backend boundaries rather than recreate scan truth in the frontend.

### Phase 3 — Local Content Index

Status: **direct bounded index foundation complete**

Implemented direct scan and normalized local content metadata. Persistent cache/indexing is intentionally not introduced until a product requirement demonstrates that direct scan performance is insufficient. Cross-root duplicate/precedence semantics remain a product decision, not a foundation blocker.

### Phase 4 — Catalog / Discover

Status: **provider-neutral backend foundation complete; real provider not implemented**

Implemented:

- typed catalog query/filter/sort/page contracts;
- provider registry/service;
- provider-output normalization and bounded validation;
- shared runtime-only provider sessions;
- integrated provider composition;
- stable provider resource identity → runtime resolver boundary;
- proactive session refresh and retry cooldown.

Next feature work may add one explicitly approved real provider and then wire Discover through the product facade. Provider-specific credentials/endpoints do not belong in generic foundation code.

### Phase 5 — Download Queue

Status: **backend execution foundation complete; product enqueue API intentionally deferred**

Implemented:

```text
Queued
→ Preparing
→ Transferring
→ Finalizing
→ Completed

or Failed / Cancelled / Interrupted
```

The runtime owns persistence, bounded scheduling, retries, cooperative cancellation, progress, crash recovery, and no-overwrite finalization. Production runtime exposes HTTPS/provider-resolved transports. `local-file` remains a deterministic RustCore test fixture.

Raw transport selection is not exposed through Tauri. A future frontend enqueue command must represent product intent, not infrastructure transport details.

### Phase 6 — Package / Export

Status: **read-only package inspection complete; mutation/export not implemented**

Implemented:

- folder / `.mcpack` / `.mcaddon` inspection;
- manifest/content classification;
- dependency relationship detection;
- archive traversal/symlink/duplicate path protections;
- bounded archive size, entry count, expanded size, and compression-ratio checks.

Package mutation/export remains outside the current approved slice and must never silently overwrite user source input.

### Phase 7 — Diagnostics / Foundation Release Readiness

Status: **remote foundation implemented; final evidence depends on verification gates**

Implemented:

- bounded redacted diagnostics;
- current component health separated from historical events;
- shared crash-recoverable JSON persistence;
- download restart validation/reconciliation;
- Windows-safe destination naming;
- deterministic npm/Cargo lockfiles;
- hosted Linux repository/backend/frontend checks;
- hosted Windows RustCore + Tauri compile gate;
- non-destructive local Windows readiness script.

Installed Windows behavior, representative large-library performance, production network behavior, and real-provider behavior require their own later evidence. Repository/static proof never upgrades itself to target-runtime proof.

## Quality rules

1. Fix the first wrong owner.
2. No runtime truth duplicated in Svelte.
3. No Tauri `invoke` calls from pages/components.
4. No business logic inside command wrappers.
5. One persistence primitive; do not duplicate file-replacement/recovery logic.
6. Raw transport selection stays inside RustCore; product IPC expresses product intent.
7. Keep provider identities and stable resource identities under canonical shared validation.
8. Keep source-size budgets green; split ownership before god-objects form.
9. Network operations must map to identifiable product behavior and use bounded timeouts in their adapter.
10. Repository/static proof never upgrades itself to target-Windows runtime proof.
