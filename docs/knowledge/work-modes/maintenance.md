# Maintenance Workflow

Use for bounded defects, regressions, stale routing, or behavior-preserving corrections.

## Entry

```text
concrete failure / explicit target
→ first wrong owner
→ smallest affected source
→ cheapest falsifiable proof
```

Do not automatically reopen architecture or rewrite adjacent modules when the current contract is correct.

## Rules

1. Reproduce or establish the failure from current evidence.
2. Fix the first wrong owner.
3. Preserve valid behavior outside the affected scope.
4. Add/update the smallest regression proof that would have caught the defect when executable tests exist.
5. Do not bundle unrelated cleanup, dependency upgrades, or abstractions.
6. Update `next-action.md` only when active continuation materially changes.
7. Stop when the bounded defect and sufficient proof are complete.

`No change required` is a valid maintenance result when current state already satisfies the contract.
