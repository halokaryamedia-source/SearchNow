---
name: development-brief
description: Front door for non-trivial SearchNow repository/application Development. Recover continuity, identify the first wrong owner, bound the smallest complete change, define falsifiable acceptance criteria and cheapest sufficient proof, then execute.
---

# Development Brief

Use when changing how SearchNow works: product behavior, UI/UX, architecture, application source, policy, tooling, machine contracts, validation, or repository structure.

Root `AGENTS.md` owns work mode, authority, continuity, canonical workflow naming, and branch behavior. `GITHUB_RULES.md` owns GitHub mutation and verification mechanics.

## Development contract

Before writing, establish only:

```text
Goal / actual requirement
First wrong owner
In scope / out of scope
Acceptance criteria: 2–5
Cheapest sufficient proof
Unresolved material decision, only if one remains
```

A user-proposed implementation is evidence about the requirement, not automatically the required architecture.

## Procedure

1. **Recover actual state** — follow Development boot and reconcile continuation against current repository state.
2. **Diagnose** — separate product requirement, legacy understanding, architecture/UX, implementation, test, and CI failures; fix the first wrong owner.
3. **Bound** — preserve valid behavior outside scope and define falsifiable acceptance criteria.
4. **Use legacy evidence correctly** — `docs/legacy/` may establish observed BlueCoin behavior, but never forces SearchNow to preserve unsafe/undesired behavior.
5. **Execute through completion** — implement reversible scope, update only invalidated downstream owners, use `GITHUB_RULES.md` for repository mechanics, and update `next-action.md` only when active continuation materially changes.
6. **Verify** — use the cheapest proof that can falsify the changed claim; distinguish static/repository verification from runtime/UI verification.

## User-facing form

```text
Tujuan:
Hasil yang dituju:
Tidak diubah:
Cara memastikan benar:
```

Stop when requested scope is complete and evidence supports the claim.
