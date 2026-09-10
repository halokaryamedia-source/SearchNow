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

Routine work happens on `develop`. `Local` and `main` move only through their documented promotion gates.

## Application Architecture

```text
Tauri 2 desktop shell
├─ Svelte 5 + Vite + TypeScript frontend
│  ├─ pages/components
│  ├─ product facade
│  └─ thin Tauri API bridge
└─ Rust
   ├─ src-tauri/                     desktop bootstrap + thin IPC commands
   └─ EngineData/Backend/RustCore/  reusable application/runtime truth
      ├─ app runtime composition
      ├─ Minecraft discovery + local library
      ├─ package inspection
      ├─ catalog/provider/session/resolver boundaries
      ├─ persistent download execution
      ├─ atomic persistence + crash recovery
      └─ bounded diagnostics/current health
```

There is one in-process application backend owner (`SearchNowBackendRuntime`). No Python worker, local HTTP server, or second backend process exists in the current architecture.

Current product surfaces:

```text
Library
Discover
Downloads
Settings
```

## Current Backend Foundation

Implemented backend capabilities include:

- typed local settings with bounded, crash-recoverable atomic JSON persistence;
- Windows Minecraft Bedrock GDK/account discovery with Preview opt-in and legacy UWP fallback;
- bounded read-only indexing of local packs and worlds;
- read-only folder / `.mcpack` / `.mcaddon` inspection with archive traversal, symlink, duplicate-path, size, and compression-ratio safeguards;
- provider-neutral catalog, shared runtime-only provider sessions, integrated provider composition, and runtime resource resolution;
- persistent bounded download execution with cancellation/retry, safe staging/finalization, restart validation, and crash reconciliation;
- production HTTPS and provider-resolved transports; the deterministic `local-file` transport is test-only and is not exposed through production Tauri IPC;
- bounded structured diagnostics with current component health separated from retained event history;
- hosted Linux verification plus a Windows RustCore/Tauri compile gate.

Real provider login/endpoints, production credentials, and frontend Discover/provider wiring are intentionally outside this foundation slice.

## Source Map

```text
Cargo.toml / Cargo.lock                one Rust workspace + deterministic lock
EngineData/
├── Backend/RustCore/                  reusable backend/runtime core
└── Frontend/RustApp/
    ├── package-lock.json              deterministic frontend dependency lock
    ├── src/                           Svelte product UI + bridge
    └── src-tauri/                     Tauri bootstrap + thin commands

UserData/                              runtime-data ownership contract
docs/foundation/                       durable product/architecture policy
docs/knowledge/                        continuation, ownership, decisions, evidence
docs/legacy/                           recovered BlueCoin 2.4 evidence
tools/                                 repository + Windows readiness verification
```

## Developer Quick Start

Prerequisites: Node.js 22+, Rust toolchain, and Tauri Windows prerequisites for local desktop execution.

```bash
cd EngineData/Frontend/RustApp
npm ci
npm run validate:quick
npm run dev:app
```

Repository checks from the repository root:

```bash
python tools/verify_repository.py
cargo fmt --all -- --check
cargo test -p searchnow-core --locked
cargo clippy -p searchnow-core --all-targets --locked -- -D warnings
```

Optional non-destructive Windows readiness check:

```powershell
./tools/windows_smoke_readiness.ps1 -CompileChecks
```

Hosted/static checks prove repository and compile contracts only. They do not prove installed Windows behavior, representative production-network behavior, or future real-provider compatibility. See `docs/knowledge/reviews/current-validation.md`.
