# SearchNow

SearchNow is a clean modernization of the inspected BlueCoin 2.4 desktop application into a maintainable Minecraft Bedrock content-management client.

The legacy executable is preserved as behavioral/architectural evidence only. SearchNow does **not** treat DRM bypass, paid-to-free conversion, protected-content key distribution, or hidden credential/data behavior as product requirements.

## Canonical Development Workflow

```text
Context Recovery
→ Product Requirements
→ Architecture & UX
→ Implementation
→ Verification
→ Local Promotion
→ Stable Promotion
```

## Branch Model

```text
develop  → active repository development
Local    → verified integration baseline; one squash commit per approved promotion
main     → stable repository history
```

Routine work happens on `develop`.

## Application Architecture

```text
Tauri 2 desktop shell
├─ Svelte 5 + Vite + TypeScript frontend
│  ├─ pages/components
│  ├─ product facade
│  └─ thin Tauri API bridge
└─ EngineData/Backend/RustCore
   ├─ SearchNowBackendRuntime = single application backend owner
   ├─ settings + shared atomic persistence
   ├─ Minecraft discovery + local library
   ├─ read-only package inspection
   ├─ provider-neutral catalog/session/resolver composition
   ├─ persistent recoverable download execution
   └─ bounded safe diagnostics + current component health
```

No Python worker, local HTTP backend, or second backend process exists in the current architecture.

Current product surfaces:

```text
Library
Discover
Downloads
Settings
```

## Current Backend Status

Implemented at repository/runtime-core level:

- typed settings and shared crash-recoverable persistence;
- Minecraft Bedrock GDK/account-scoped discovery with Preview opt-in and legacy UWP fallback;
- bounded local content indexing;
- read-only folder / `.mcpack` / `.mcaddon` inspection with archive safety limits;
- provider-neutral catalog, provider-session, adapter, and runtime resource-resolution boundaries;
- canonical provider/resource identity validation;
- bounded persistent download queue with startup reconciliation, Windows-safe destination naming, HTTPS/provider-resolved production transports, and atomic no-overwrite finalization;
- provider-neutral catalog download intent at the Tauri IPC boundary; raw transport selection is internal-only;
- one consolidated `SearchNowBackendRuntime` managed by Tauri;
- bounded secret-safe diagnostics with current component health;
- committed npm/Rust lockfiles and deterministic `npm ci` / Cargo `--locked` verification;
- hosted Linux verification and native Windows RustCore/Tauri compile gates.

Not implemented yet:

- real provider login/credentials/endpoints;
- Marketplace/PlayFab-specific integration;
- production Discover/provider frontend wiring;
- installer/release branding and clean-machine acceptance.

## Source Map

```text
EngineData/
├── Backend/RustCore/        reusable backend/runtime truth
└── Frontend/RustApp/
    ├── src/                 Svelte product UI + bridge
    └── src-tauri/src/       thin Tauri bootstrap + commands

UserData/                    runtime-data ownership contract
docs/foundation/             durable product/architecture policy
docs/knowledge/              continuation, ownership, decisions, evidence
docs/legacy/                 recovered BlueCoin 2.4 evidence
tools/                       repository and Windows readiness checks
```

## Developer Quick Start

Prerequisites: Node.js 22+, Rust toolchain, and Tauri Windows prerequisites for local desktop execution.

```bash
cd EngineData/Frontend/RustApp
npm ci
npm run validate:quick
npm run dev:app
```

Repository contract check:

```bash
python tools/verify_repository.py
```

Repository/static and hosted compile checks do not prove installed Windows runtime behavior. See `docs/knowledge/reviews/current-validation.md`.
