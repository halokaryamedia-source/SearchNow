# Repository Ownership

Use only to answer **who owns what**. Exact procedures remain in the named owners.

## Root owners

| Boundary | Owner |
|---|---|
| work modes / boot / authority / continuity / branch kernel | `AGENTS.md` |
| GitHub mutation/history/CI/promotion safety | `GITHUB_RULES.md` |
| stable product/repository orientation | `CONTEXT.md` |
| active continuation | `docs/knowledge/next-action.md` |
| source/evidence precedence | `docs/knowledge/source-authority.md` |
| durable decisions/rationale | `docs/knowledge/decisions/` |
| current validation evidence | `docs/knowledge/reviews/current-validation.md` |

## Foundation owners

| Boundary | Owner |
|---|---|
| allowed/disallowed product behavior | `docs/foundation/00-product-boundaries.md` |
| development lifecycle | `docs/foundation/01-development-flow.md` |
| target user workflow | `docs/foundation/02-target-product-flow.md` |
| implementation direction | `docs/foundation/03-implementation-roadmap.md` |
| verification/promotion | `docs/foundation/04-verification-promotion.md` |
| desktop application topology | `docs/foundation/05-application-architecture.md` |
| backend topology/data/performance/package/download/transport contract | `docs/foundation/06-backend-architecture.md` |

## Current implementation owners

| Boundary | Owner |
|---|---|
| typed settings + persistence | `EngineData/Backend/RustCore/src/settings.rs` |
| platform/AppData context | `EngineData/Backend/RustCore/src/platform.rs` |
| Minecraft storage discovery | `EngineData/Backend/RustCore/src/minecraft.rs` |
| local content indexing | `EngineData/Backend/RustCore/src/library.rs` |
| read-only package inspection + archive safety + BP/RP relationships | `EngineData/Backend/RustCore/src/package/` |
| download DTOs/state contract | `EngineData/Backend/RustCore/src/download/model.rs` |
| download lifecycle/state machine/concurrency | `EngineData/Backend/RustCore/src/download/manager.rs` |
| download persistence/recovery | `EngineData/Backend/RustCore/src/download/store.rs` |
| download workspace + atomic finalization | `EngineData/Backend/RustCore/src/download/workspace.rs` |
| transport adapter contract + local-file transport | `EngineData/Backend/RustCore/src/download/transport.rs` |
| scheduler/executor/progress checkpointing | `EngineData/Backend/RustCore/src/download/executor.rs` |
| public HTTPS transport policy/request/redirect/stream bounds | `EngineData/Backend/RustCore/src/download/http.rs` |
| backend orchestration snapshot | `EngineData/Backend/RustCore/src/lib.rs` |
| backend error semantics | `EngineData/Backend/RustCore/src/error.rs` |
| Tauri IPC registration | `EngineData/Frontend/RustApp/src-tauri/src/commands/registry.rs` |
| Tauri application/runtime + transport registration | `EngineData/Frontend/RustApp/src-tauri/src/app_bootstrap.rs` |
| Tauri settings adaptation | `.../commands/settings.rs` |
| Tauri Minecraft adaptation | `.../commands/minecraft.rs` |
| Tauri library adaptation | `.../commands/library.rs` |
| Tauri package-inspection adaptation | `.../commands/package.rs` |
| Tauri download adaptation | `.../commands/download.rs` |
| frontend raw invoke boundary | `EngineData/Frontend/RustApp/src/app/bridge/runtimeApi.ts` |

Tauri commands must remain adapters. Queue lifecycle, transport execution, network policy, persistence, filesystem, package, and Minecraft behavior belong in RustCore rather than IPC wrappers or Svelte pages.

Future provider authentication must not be pushed into `http.rs` or persisted queue models. The next owner is a separate runtime resolver/provider-adapter boundary that turns opaque provider resource ids into ephemeral in-memory transfer material.

## Legacy evidence

Legacy BlueCoin behavior remains under `docs/legacy/` and is evidence only. It does not own SearchNow implementation behavior.
