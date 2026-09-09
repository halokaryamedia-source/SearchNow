# 04 — Verification and Promotion

## Iteration proof

During normal `develop` work, run the cheapest check that can falsify the changed claim.

Current repository-only baseline:

```bash
python tools/verify_repository.py
```

When executable source is introduced, add module-specific formatting, static analysis, build, unit, integration, and runtime/UI tests according to the selected stack. Do not invent gates before the underlying implementation exists.

## Promotion boundary

### develop → Local

Requires:

- dedicated PR from `develop`;
- Local Promotion Verify PASS;
- one coherent approved logical update;
- squash merge;
- post-merge synchronization of `develop` to resulting `Local` HEAD.

### Local → main

Requires:

- explicit stable PR from `Local`;
- Stable Release Verify PASS;
- normal merge commit.

## Proof language

Use only:

```text
implemented
repository/static verified
integration verified
runtime/UI verified
not verified
```

A passing repository structure check does not prove application runtime behavior.
