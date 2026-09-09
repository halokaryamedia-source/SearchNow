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
| backend topology/data/performance/package/download contract | `docs/foundation/06-backend-architecture.md` |

## Current implementation owners

| Boundary | Owner |
|---|---|
| typed settings + persistence | `EngineData/Backend/RustCore/src/settings.rs` |
| platform/AppData context | `EngineData/Backend/RustCore/src/platform.rs` |
| Minecraft storage discovery | `EngineData/Backend/RustCore/src/minecraft.rs` |
| local content indexing | `EngineData/Backend/RustCore/src/library.rs` |
| read-only package inspection + archive safety + BP/RP relationships | `EngineData/Backend/RustCore/src/package/` |
| download lifecycle/state machine/concurrency | `EngineData/Backend/RustCore/src/download/manager.rs` |
| download persistence/recovery | `EngineData/Backend/RustCore/src/download/store.rs` |
| download workspace + atomic finalization | `EngineData/Backend/RustCore/src/download/workspace.rs` |
| download DTOs/state contract | `EngineData/Backend/RustCore/src/download/model.rs` |
| backend orchestration snapshot | `EngineData/Backend/RustCore/src/lib.rs` |
| backend error semantics | `EngineData/Backend/RustCore/src/error.rs` |
| Tauri IPC registration | `EngineData/Frontend/RustApp/src-tauri/src/commands/registry.rs` |
| Tauri settings adaptation | `.../commands/settings.rs` |
| Tauri Minecraft adaptation | `.../commands/minecraft.rs` |
| Tauri library adaptation | `.../commands/library.rs` |
| Tauri package-inspection adaptation | `.../commands/package.rs` |
| Tauri download queue adaptation + transactional persistence | `.../commands/download.rs` |
| frontend raw invoke boundary | `EngineData/Frontend/RustApp/src/app/bridge/runtimeApi.ts` |

Tauri commands must remain adapters. Fix filesystem/domain/archive/download behavior in RustCore, not in IPC wrappers or Svelte pages.

## Legacy evidence

Legacy BlueCoin behavior remains under `docs/legacy/` and is evidence only. It does not own SearchNow implementation behavior.
