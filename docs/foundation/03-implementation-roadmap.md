# 03 — Implementation Roadmap

## Objective

Build SearchNow as a small, maintainable Minecraft Bedrock content-management desktop application with clear ownership between presentation, native command boundaries, reusable runtime logic, local data, and network/filesystem adapters.

The old BlueCoin WinForms structure is evidence only; it is not the source architecture to recreate.

## Approved implementation shape

```text
EngineData/Frontend/RustApp/
├── src/
│   ├── App.svelte
│   ├── pages/
│   ├── components/
│   ├── app/bridge/
│   ├── app/shared/
│   └── styles/
└── src-tauri/src/
    ├── main.rs
    ├── app_bootstrap.rs
    ├── commands/
    └── engine/
```

Architecture authority: `05-application-architecture.md`.

## Runtime rule

```text
Svelte UI
→ product facade
→ thin runtime API
→ Tauri command
→ Rust engine
```

Keep SearchNow in one desktop process whenever practical. Do not add Python, a local HTTP server, or another worker process unless a concrete capability requires an ecosystem/runtime that Rust cannot serve cleanly and a new architecture decision is approved.

## Target engine ownership

As features arrive, grow the Rust engine by responsibility rather than by legacy class names:

```text
engine/
├── runtime.rs
├── minecraft/      installation discovery + local Minecraft boundaries
├── library/        normalized local content model/index
├── catalog/        permitted remote catalog/session behavior
├── downloads/      runtime-owned transfer/process queue
├── package/        manifest/archive inspection + validation
├── storage/        typed settings/cache/index persistence
└── diagnostics/    redacted structured diagnostics
```

Add modules only when a caller/current responsibility exists; do not pre-create empty abstraction trees.

## Candidate domain models

```text
MinecraftInstallation
ContentItem
ContentSource
ContentType
ContentStatus
PackageManifest
DownloadJob
ValidationResult
AppSettings
```

Transport-specific DTOs stay at their adapter/boundary and must not leak through the whole application.

## Development phases

### Phase 0 — Evidence / Development System

Status: **complete**

- legacy executable architecture recovered;
- product/safety boundaries documented;
- PRD-Creator-style repository workflow established.

### Phase 1 — Application Architecture Scaffold

Status: **complete at repository/static level**

- Tauri/Svelte/Rust stack chosen;
- four product surfaces scaffolded;
- frontend bridge boundary established;
- Rust commands/engine separation established;
- runtime-status end-to-end slice added;
- source-size and architecture contracts added.

Target-Windows execution remains a separate proof boundary.

### Phase 2 — Minecraft Discovery + Library

Next implementation target.

Deliver one thin vertical slice:

```text
user opens Library
→ Rust discovers configured/default Minecraft Bedrock location
→ local result normalized
→ product facade receives library/install snapshot
→ Library renders truthful detected/not-detected state
```

Constraints:

- no network call;
- no entitlement/key upload;
- no hidden background sharing;
- filesystem failure must be recoverable;
- UI does not own scan truth.

### Phase 3 — Local Content Index

- enumerate legitimate local packs/world/templates;
- normalized `ContentItem` model;
- deduplicate and categorize;
- cache/index only after the direct scan contract is correct.

### Phase 4 — Catalog / Discover

- permitted catalog adapter;
- explicit session/network ownership;
- debounced search;
- normalized catalog DTO → product model;
- clear offline/error state.

### Phase 5 — Download Queue

Runtime-owned state machine:

```text
Queued
→ Downloading
→ Validating
→ Processing
→ Completed

or Failed / Cancelled
```

The frontend observes jobs; it does not perform transfer/process logic.

### Phase 6 — Package / Export

- manifest/archive inspection;
- content-type detection;
- BP/RP pairing where legitimate/applicable;
- staged/atomic export;
- post-write validation;
- never silently overwrite source input.

### Phase 7 — Diagnostics / Release

- redacted diagnostics;
- cache cleanup;
- Windows target acceptance;
- installer/bundle contract;
- clean-machine verification.

## Quality rules

1. Fix the first wrong owner.
2. No runtime truth duplicated in Svelte.
3. No Tauri `invoke` calls from pages/components.
4. No business logic inside command wrappers.
5. Keep source-size budgets green; split ownership before god-objects form.
6. Network operations must map to identifiable product behavior.
7. Repository/static proof never upgrades itself to target-Windows runtime proof.
