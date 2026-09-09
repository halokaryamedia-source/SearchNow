# 01 — Development Flow

SearchNow adopts the PRD-Creator development discipline with application-specific owners.

## Canonical lifecycle

```text
Context Recovery
→ Product Requirements
→ Architecture & UX
→ Implementation
→ Verification
→ Local Promotion
→ Stable Promotion
```

These are human workflow names. Internal implementation states may exist later but do not create alternate workflow vocabulary.

## Context Recovery

Recover only the context needed for the task:

```text
AGENTS.md
→ CONTEXT.md
→ docs/knowledge/next-action.md
→ smallest relevant owner
```

Read-only inspection stops after reporting unless change is explicitly requested.

## Product Requirements

Owns what SearchNow must do, must not do, user-facing behavior, privacy/safety boundary, and acceptance intent.

## Architecture & UX

Owns component boundaries, source organization, UI navigation/state model, data ownership, service boundaries, and technical decisions required before implementation.

## Implementation

Owns executable source. Implement only approved requirements/contracts and avoid broad cleanup outside the affected owner.

## Verification

Use the cheapest falsifiable proof during iteration. Promotion boundaries run broader available checks.

Verification categories:

```text
repository/static
unit/module
integration
runtime/UI
```

Do not claim a stronger category than actually executed.

## Local Promotion

`develop → Local` means one coherent update is ready to become the verified working baseline.

- source: `develop`
- gate: Local Promotion Verify
- merge: squash
- result: one new logical milestone commit on `Local`
- then synchronize `develop` to resulting `Local` HEAD

## Stable Promotion

`Local → main` is explicit stable promotion.

- source: `Local`
- gate: Stable Release Verify
- merge: normal merge commit
- stable marker is not synchronized back into lower branches merely for ancestry

Tags/releases are separate explicit actions.
