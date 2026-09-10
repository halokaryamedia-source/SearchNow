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
│       ├── identity.rs
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

**Backend foundation complete.** Persistent bounded queue, cancellation/retry, fail-closed persisted-state validation, startup crash reconciliation, Windows-safe destination naming, staged no-overwrite finalization, public HTTPS transport, and provider-resolved transport are implemented.

The deterministic `local-file` transport remains test/development-only and is not registered by the production application runtime.

### Phase 5 — Provider-Neutral Runtime

**Backend foundation complete.** Catalog domain, provider-session manager, resolver registry, integrated-provider composition, shared canonical provider/resource identity validation, and secret-safe public state are implemented.

No real provider endpoint/login is implemented yet.

### Phase 6 — Application Runtime + Observability

**Remote foundation complete.** Tauri manages one `SearchNowBackendRuntime`; Settings/Downloads share one atomic persistence owner; current health is separated from historical diagnostics; scheduler continuation failures are surfaced; raw transport selection is kept behind the backend boundary; hosted Linux and Windows verification are green.

### Phase 7 — Deterministic Build / Verification

**Complete for repository verification.** npm and both Rust application scopes have committed lockfiles. Repository, Local-promotion, and stable-release workflows use `npm ci` and Cargo `--locked`, with current GitHub Actions runtime versions.

### Phase 8 — Target-Windows Runtime Smoke

**Next validation boundary.** Run non-destructive local Windows evidence for AppData resolution, current GDK/UWP discovery, settings save/reload, package inspection, download finalization, diagnostics, and actual Tauri application launch.

Hosted Windows compilation is prerequisite evidence, not a substitute for this phase.

### Phase 9 — Real Provider Integration

After local foundation smoke is accepted, define and implement the concrete provider credential/session acquisition, endpoint contracts, timeout/retry/backoff behavior, and Marketplace/PlayFab-specific adapters without changing the established product/transport boundary.

### Phase 10 — Discover / Downloads / Settings Product Wiring

Wire implemented backend/provider behavior into the product facade/UI without moving runtime truth into Svelte.

### Phase 11 — Release Acceptance

Representative large-library/network fixtures, installer/bundle/branding, clean-machine verification, and Local → main promotion.

## Quality rules

1. Fix the first wrong owner.
2. No runtime truth duplicated in Svelte.
3. No Tauri `invoke` calls from pages/components.
4. No business logic inside command wrappers.
5. Keep source-size budgets green; split ownership before god-objects form.
6. Network operations must map to identifiable product behavior.
7. Repository/static/hosted compile proof never upgrades itself to installed Windows runtime proof.
8. Do not freeze temporary workarounds into repository contracts.
9. Verification must use committed dependency graphs; update lockfiles only as an explicit dependency change.
