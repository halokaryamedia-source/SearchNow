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
| allowed/disallowed product behavior + privacy/safety | `docs/foundation/00-product-boundaries.md` |
| canonical development lifecycle | `docs/foundation/01-development-flow.md` |
| target user workflow | `docs/foundation/02-target-product-flow.md` |
| staged implementation direction | `docs/foundation/03-implementation-roadmap.md` |
| verification/promotion semantics | `docs/foundation/04-verification-promotion.md` |

## Legacy evidence owners

| Boundary | Owner |
|---|---|
| legacy application behavioral baseline | `docs/legacy/01-current-state.md` |
| recovered CLR/source architecture | `docs/legacy/04-recovered-source-architecture.md` |
| type/method/field symbol map | `docs/legacy/05-recovered-symbol-map.md` |
| runtime/filesystem/network/data contracts | `docs/legacy/06-runtime-data-contracts.md` |
| reconstruction evidence/confidence | `docs/legacy/07-reconstruction-evidence.md` |

## Future implementation owners

Once source exists, ownership should remain layered:

```text
UI               → presentation/navigation/user state
Application      → use cases/orchestration
Core/Domain      → stable product models/rules
Infrastructure   → filesystem/network/platform adapters
Tests            → executable contracts for corresponding owner
```

Exact project/file ownership must be updated here when the source tree is finalized.
