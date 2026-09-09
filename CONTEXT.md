# SearchNow Context

Status: documentation and development-system foundation  
Development branch: `develop`  
Verified integration baseline: `Local`  
Stable branch: `main`

SearchNow is a clean modernization of the inspected BlueCoin 2.4 Windows desktop application. The repository currently preserves recovered legacy architecture and defines the target product/development model before source implementation.

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

`develop → Local` requires the Local promotion gate and squash merge. After promotion, synchronize `develop` to the resulting `Local` HEAD. `Local → main` requires explicit stable promotion and the stable gate.

## Current product direction

Target user-facing surfaces are intentionally simple:

```text
Library
Discover
Downloads
Settings
```

Normal users should not need to understand internal UUIDs, PlayFab details, entitlement internals, encryption keys, or package-processing internals.

## Evidence boundary

The inspected BlueCoin 2.4 executable is a legacy evidence source, not SearchNow source of truth.

```text
legacy binary evidence
→ recovered documentation
→ approved SearchNow requirements
→ target architecture/UX
→ implementation
→ verification
```

Explicit SearchNow decisions may preserve, replace, or reject legacy behavior.

## Safety/privacy boundary

SearchNow is local-first by default. External transmission of user/account-derived data must be explicit, documented, optional where appropriate, and visible.

Protected-content bypass, paid-to-free conversion, content-key pooling/distribution, and hidden entitlement-derived data upload are outside the target product.

## Repository map

```text
AGENTS.md            routing, authority, continuity, branch kernel
GITHUB_RULES.md      GitHub execution and mutation discipline
CONTEXT.md           this stable orientation
docs/foundation/     durable SearchNow policy and target contracts
docs/knowledge/      next action, ownership, decisions, evidence
docs/legacy/         recovered BlueCoin 2.4 baseline
.agents/skills/      reusable development judgment
tools/               repository verification/operator utilities
.github/             CI and promotion gates
src/ + tests/        implementation/proof once source work begins
```

For a new Development session, read `docs/knowledge/next-action.md` after this file. For bounded work, route directly to the smallest owner.
