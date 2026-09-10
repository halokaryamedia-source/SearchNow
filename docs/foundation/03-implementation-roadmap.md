# 03 — Implementation Roadmap

## Objective

Build SearchNow as a small, maintainable Minecraft Bedrock content-management desktop application with one native backend owner and thin UI/IPC boundaries.

The old BlueCoin WinForms structure remains evidence only; it is not the source architecture to recreate.

## Approved implementation shape

```text
EngineData/
├── Backend/RustCore/
│   └── src/
│       ├── app_runtime.rs
│       ├── settings.rs + storage.rs
│       ├── minecraft.rs + library.rs
│       ├── package/
│       ├── download/
│       ├── catalog/
│       ├── provider_session/
│       ├── provider_adapter/
│       └── diagnostics.rs
└── Frontend/RustApp/
    ├── src/                 Svelte product UI + facade/bridge
    └── src-tauri/src/       thin Tauri bootstrap + commands
```

## Runtime rule

```text
Svelte UI
→ product facade
→ thin runtime API
→ Tauri command
→ SearchNowBackendRuntime
→ RustCore domain/runtime owner
```

Do not add Python, a local HTTP server, or another backend process unless a concrete capability cannot be served cleanly by the Rust process and an explicit architecture decision revises this rule.

## Phase status

### Phase 0 — Evidence / Development System

**Complete.** Legacy architecture, product/safety boundaries, branch model, and development routing are documented.

### Phase 1 — Application Architecture Scaffold

**Complete at repository/static level.** Tauri/Svelte/Rust stack, four product surfaces, thin bridge, source-size contracts, and runtime-status vertical slice are established.

### Phase 2 — Minecraft Discovery + Local Library

**Backend complete; frontend product wiring still partial.** RustCore discovers current Bedrock GDK/account roots, optional Preview, and legacy UWP fallback, then performs bounded local indexing.

### Phase 3 — Package Inspection

**Backend complete for read-only inspection.** Folder / `.mcpack` / `.mcaddon` metadata inspection, manifest classification, BP/RP relationship detection, and archive safety checks are implemented without extraction/mutation.

### Phase 4 — Download Runtime

**Backend complete for generic execution boundary.** Persistent bounded queue, cancellation/retry, restart recovery, local deterministic fixture transport, public HTTPS transport, provider-resolved transport, and staged no-overwrite finalization are implemented.

Further hardening remains part of foundation closure where crash recovery or platform behavior requires it.

### Phase 5 — Provider-Neutral Runtime

**Backend foundation complete.** Catalog domain, provider-session manager, resolver registry, integrated-provider composition, and shared application ownership exist with secret-safe public state.

No real provider endpoint/login is implemented yet.

### Phase 6 — Application Runtime + Observability

**Implemented; foundation closure in progress.** Tauri manages one `SearchNowBackendRuntime`; bounded diagnostics/health and hosted Windows compile gates exist. Current closure work removes the old Tauri icon workaround and consolidates file persistence/recovery.

### Phase 7 — Real Provider Preparation

**Next after closure is green.** Before integrating a real provider, define canonical provider/resource identity types, retry/timeout policy, credential-storage requirements, and product-intent APIs that prevent frontend transport leakage.

### Phase 8 — Discover / Downloads / Settings Product Wiring

Wire implemented backend behavior into the product facade/UI without moving runtime truth into Svelte.

### Phase 9 — Target-Windows Acceptance / Release

Installed Windows smoke testing, representative large-library/network fixtures, installer/bundle/branding, clean-machine verification, and Local → main promotion.

## Quality rules

1. Fix the first wrong owner.
2. No runtime truth duplicated in Svelte.
3. No Tauri `invoke` calls from pages/components.
4. No business logic inside command wrappers.
5. Keep source-size budgets green; split ownership before god-objects form.
6. Network operations must map to identifiable product behavior.
7. Repository/static/hosted compile proof never upgrades itself to installed Windows runtime proof.
8. Do not freeze temporary workarounds into repository contracts.
