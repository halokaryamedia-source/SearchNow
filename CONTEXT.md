# SearchNow Context

Status: backend foundation / application hardening  
Development branch: `develop`  
Verified integration baseline: `Local`  
Stable branch: `main`

SearchNow is a clean modernization of the inspected BlueCoin 2.4 Windows desktop application. Legacy behavior is preserved as evidence while the new application is built around explicit product boundaries, local-first privacy, deterministic builds, and a small maintainable desktop architecture.

## Canonical workflow

```text
Context Recovery
→ Product Requirements
→ Architecture & UX
→ Implementation
→ Verification
→ Local Promotion
→ Stable Promotion
```

## Branch model

```text
develop → active Development
Local   → verified integration baseline; one squash commit per approved update
main    → stable repository history
```

## Current product surfaces

```text
Library
Discover
Downloads
Settings
```

## Current architecture

```text
Tauri 2
├─ Svelte 5 + Vite + TypeScript frontend
│  ├─ transient presentation/application state
│  ├─ product-facing facade
│  └─ thin Tauri command API
└─ Rust
   ├─ src-tauri/ = desktop bootstrap + IPC boundary
   └─ EngineData/Backend/RustCore/ = reusable runtime/domain truth
      ├─ one SearchNowBackendRuntime composition root
      ├─ local settings/Minecraft/library/package boundaries
      ├─ catalog/provider/session/resolver boundaries
      ├─ persistent download execution + recovery
      ├─ shared atomic JSON persistence
      └─ bounded diagnostics + current health
```

There is no Python worker/current `EngineData/Backend` process. `RustCore` is an in-process library linked by Tauri. A separate runtime may be added only when a concrete requirement cannot be served cleanly by Rust and the architecture decision is explicitly revised.

## Current source roots

```text
Cargo.toml / Cargo.lock
EngineData/Backend/RustCore/
EngineData/Frontend/RustApp/
UserData/
```

## Runtime ownership

Svelte owns UI/transient state only. `SearchNowBackendRuntime` owns application runtime composition. RustCore owns persistent/runtime truth such as Minecraft discovery, library state, catalog sessions, resource resolution, download jobs, package validation, filesystem I/O, settings persistence, recovery, and diagnostics.

Production Tauri IPC does not expose raw transport selection. The deterministic `local-file` transport remains a RustCore test fixture. Future frontend download actions must express product intent and let RustCore select the transport.

## Deterministic dependency boundary

```text
Cargo.lock                              Rust workspace dependency truth
EngineData/Frontend/RustApp/package-lock.json   frontend dependency truth
```

Verification uses `npm ci` and Cargo `--locked`.

## Evidence boundary

```text
legacy binary evidence
→ docs/legacy/
→ approved SearchNow requirements
→ docs/foundation/ architecture
→ implementation source
→ repository/static proof
→ hosted target compile proof
→ installed target-runtime proof
```

Source/build success is not proof of installed Windows behavior.

## Safety/privacy boundary

SearchNow is local-first by default. External transmission of user/account-derived data must be explicit and feature-bound.

Protected-content bypass, paid-to-free conversion, content-key pooling/distribution, and hidden entitlement-derived data upload remain outside the product.

## Repository map

```text
AGENTS.md            routing, authority, continuity, branch kernel
GITHUB_RULES.md      GitHub execution and mutation discipline
CONTEXT.md           this stable orientation
docs/foundation/     durable SearchNow policy + architecture
docs/knowledge/      next action, ownership, decisions, evidence
docs/legacy/         recovered BlueCoin 2.4 evidence
.agents/skills/      reusable development judgment
EngineData/          current implementation source
UserData/            runtime-data ownership contract
tools/               repository/target readiness utilities
.github/             CI and promotion gates
```

For a new Development session, read `docs/knowledge/next-action.md` after this file.
