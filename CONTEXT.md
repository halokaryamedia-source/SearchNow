# SearchNow Context

Status: architecture scaffold / implementation foundation  
Development branch: `develop`  
Verified integration baseline: `Local`  
Stable branch: `main`

SearchNow is a clean modernization of the inspected BlueCoin 2.4 Windows desktop application. Legacy behavior is preserved as evidence while the new application is built around explicit product boundaries, local-first privacy, and a small maintainable desktop architecture.

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
└─ Rust desktop/runtime backend
   ├─ commands/ = IPC boundary
   └─ engine/   = reusable runtime/domain truth
```

There is no Python worker/current `EngineData/Backend` process. A separate runtime may be added only when a concrete requirement cannot be served cleanly by Rust and the architecture decision is explicitly revised.

## Current source roots

```text
EngineData/Frontend/RustApp/
UserData/
```

## Runtime ownership

Svelte owns UI/transient state only. Rust owns persistent/runtime truth such as Minecraft discovery, library state, catalog sessions, download jobs, package validation, filesystem I/O, settings persistence, and diagnostics as those capabilities are implemented.

## Evidence boundary

```text
legacy binary evidence
→ docs/legacy/
→ approved SearchNow requirements
→ docs/foundation/ architecture
→ implementation source
→ repository/static proof
→ target-Windows runtime proof
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
tools/               repository verification utilities
.github/             CI and promotion gates
```

For a new Development session, read `docs/knowledge/next-action.md` after this file.
