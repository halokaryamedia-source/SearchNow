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
└─ Rust backend/runtime
   ├─ commands/  thin IPC boundary
   └─ engine/    application/runtime truth
```

No Python worker or second backend process exists in the current architecture.

Current product surfaces:

```text
Library
Discover
Downloads
Settings
```

## Source Map

```text
EngineData/
└── Frontend/RustApp/
    ├── src/                 Svelte product UI + bridge
    └── src-tauri/src/       Rust commands + engine

UserData/                    runtime-data ownership contract
docs/foundation/             durable product/architecture policy
docs/knowledge/              continuation, ownership, decisions, evidence
docs/legacy/                 recovered BlueCoin 2.4 evidence
```

## Current Executable Slice

The first implemented vertical slice proves the architecture itself:

```text
Svelte App
→ runtimeProductFacade
→ runtimeApi
→ Tauri get_runtime_status
→ Rust command
→ Rust engine
→ runtime status returned to UI
```

Minecraft discovery/catalog/download logic is intentionally not implemented yet.

## Developer Quick Start

Prerequisites: Node.js 22+, Rust toolchain, and Tauri Windows prerequisites for local desktop execution.

```bash
cd EngineData/Frontend/RustApp
npm install
npm run validate:quick
npm run dev:app
```

Repository contract check:

```bash
python tools/verify_repository.py
```

Repository/static checks do not prove installed Windows runtime behavior. See `docs/knowledge/reviews/current-validation.md`.
