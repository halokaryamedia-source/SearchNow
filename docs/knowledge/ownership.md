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

## Product/foundation owners

| Boundary | Owner |
|---|---|
| allowed/disallowed behavior + privacy/safety | `docs/foundation/00-product-boundaries.md` |
| canonical development lifecycle | `docs/foundation/01-development-flow.md` |
| target user workflow | `docs/foundation/02-target-product-flow.md` |
| staged implementation direction | `docs/foundation/03-implementation-roadmap.md` |
| verification/promotion semantics | `docs/foundation/04-verification-promotion.md` |
| current source/runtime architecture | `docs/foundation/05-application-architecture.md` |

## Legacy evidence owners

| Boundary | Owner |
|---|---|
| legacy behavioral baseline | `docs/legacy/01-current-state.md` |
| recovered CLR/source architecture | `docs/legacy/04-recovered-source-architecture.md` |
| type/method/field symbol map | `docs/legacy/05-recovered-symbol-map.md` |
| runtime/filesystem/network/data contracts | `docs/legacy/06-runtime-data-contracts.md` |
| reconstruction evidence/confidence | `docs/legacy/07-reconstruction-evidence.md` |

## Implementation owners

| Boundary | Owner |
|---|---|
| desktop package/build/frontend root | `EngineData/Frontend/RustApp/` |
| top-level Svelte composition/navigation | `EngineData/Frontend/RustApp/src/App.svelte` |
| product pages | `EngineData/Frontend/RustApp/src/pages/` |
| reusable visual components | `EngineData/Frontend/RustApp/src/components/` |
| product-readable runtime facade | `EngineData/Frontend/RustApp/src/app/bridge/runtimeProductFacade.ts` |
| raw Tauri frontend command client | `EngineData/Frontend/RustApp/src/app/bridge/runtimeApi.ts` |
| frontend shared DTO/view types | `EngineData/Frontend/RustApp/src/app/shared/` |
| design tokens/base desktop CSS | `EngineData/Frontend/RustApp/src/styles/` |
| Tauri process entry | `EngineData/Frontend/RustApp/src-tauri/src/main.rs` |
| desktop bootstrap/window behavior | `EngineData/Frontend/RustApp/src-tauri/src/app_bootstrap.rs` |
| Tauri command registration/wrappers | `EngineData/Frontend/RustApp/src-tauri/src/commands/` |
| reusable runtime/domain truth | `EngineData/Frontend/RustApp/src-tauri/src/engine/` |
| runtime-data ownership contract | `UserData/README.md` |

Future Minecraft/library/catalog/download/package/storage modules belong under the Rust engine unless a revised architecture decision assigns a different owner.
