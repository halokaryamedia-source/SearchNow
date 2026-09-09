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
| backend local/package/download/transport/resolver contract | `docs/foundation/06-backend-architecture.md` |
| provider-neutral catalog/query/domain contract | `docs/foundation/07-catalog-architecture.md` |
| provider session/credential runtime contract | `docs/foundation/08-provider-session-architecture.md` |

## Current implementation owners

| Boundary | Owner |
|---|---|
| typed settings + persistence | `EngineData/Backend/RustCore/src/settings.rs` |
| platform/AppData context | `EngineData/Backend/RustCore/src/platform.rs` |
| Minecraft storage discovery | `EngineData/Backend/RustCore/src/minecraft.rs` |
| local content indexing | `EngineData/Backend/RustCore/src/library.rs` |
| read-only package inspection + archive safety + BP/RP relationships | `EngineData/Backend/RustCore/src/package/` |
| catalog query/item/page/download-reference DTOs | `EngineData/Backend/RustCore/src/catalog/model.rs` |
| catalog provider registry + query validation + provider-result normalization | `EngineData/Backend/RustCore/src/catalog/provider.rs` |
| safe provider-session status/error DTOs | `EngineData/Backend/RustCore/src/provider_session/model.rs` |
| runtime-only provider session material/lease + acquire/reuse/refresh coordination | `EngineData/Backend/RustCore/src/provider_session/runtime.rs` |
| persisted download DTO/state contract | `EngineData/Backend/RustCore/src/download/model.rs` |
| download lifecycle/state machine/concurrency | `EngineData/Backend/RustCore/src/download/manager.rs` |
| download persistence/recovery | `EngineData/Backend/RustCore/src/download/store.rs` |
| download workspace + atomic finalization | `EngineData/Backend/RustCore/src/download/workspace.rs` |
| generic transport registry + local-file transport | `EngineData/Backend/RustCore/src/download/transport.rs` |
| scheduler/executor/progress checkpointing | `EngineData/Backend/RustCore/src/download/executor.rs` |
| public HTTPS + ephemeral HTTP request mechanics/redirect/timeout/stream bounds | `EngineData/Backend/RustCore/src/download/http.rs` |
| stable provider reference + runtime resolver registry + expiry/re-resolution + secret isolation | `EngineData/Backend/RustCore/src/download/resolver.rs` |
| backend orchestration snapshot | `EngineData/Backend/RustCore/src/lib.rs` |
| backend error semantics | `EngineData/Backend/RustCore/src/error.rs` |
| Tauri IPC registration | `EngineData/Frontend/RustApp/src-tauri/src/commands/registry.rs` |
| Tauri application/runtime transport registration | `EngineData/Frontend/RustApp/src-tauri/src/app_bootstrap.rs` |
| Tauri settings adaptation | `.../commands/settings.rs` |
| Tauri Minecraft adaptation | `.../commands/minecraft.rs` |
| Tauri library adaptation | `.../commands/library.rs` |
| Tauri package-inspection adaptation | `.../commands/package.rs` |
| Tauri download adaptation | `.../commands/download.rs` |
| frontend raw invoke boundary | `EngineData/Frontend/RustApp/src/app/bridge/runtimeApi.ts` |

Tauri commands must remain adapters. Queue lifecycle, network policy, catalog validation, provider resolution/session state, persistence, filesystem, package, and Minecraft behavior belong in RustCore rather than IPC wrappers or Svelte pages.

`provider_session/runtime.rs` owns generic acquire/reuse/refresh coordination and secret-bearing runtime leases, **not provider-specific login endpoints**. `catalog/provider.rs` owns catalog coordination, `resolver.rs` owns resource resolution, and `http.rs` owns HTTP mechanics. A real provider adapter should compose these existing interfaces around one shared session manager rather than duplicating state machines or pushing credential values into persisted DTOs.

## Legacy evidence

Legacy BlueCoin behavior remains under `docs/legacy/` and is evidence only. It does not own SearchNow implementation behavior.
