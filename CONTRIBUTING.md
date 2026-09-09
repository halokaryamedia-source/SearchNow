# Contributing

SearchNow currently uses the same development-history model as PRD-Creator, adapted for this application.

## Branch model

```text
develop
→ active repository development
→ working commits may be granular

Local
→ verified integration / stable working baseline
→ exactly one squash commit per approved promoted update

main
→ stable repository history
→ stable promotions use merge commits
```

Routine work happens on `develop`. Do not make routine changes directly on `Local` or `main`.

## Promote `develop` → `Local`

Use a dedicated pull request when one coherent development outcome is ready for the verified baseline.

The PR must:

- come from `develop`;
- pass `Local Promotion Verify`;
- represent one approved logical update;
- be merged with **squash merge**.

After merge, synchronize `develop` to the resulting `Local` HEAD before starting the next development cycle.

```text
one approved develop → Local promotion
= exactly +1 logical milestone commit on Local
```

## Promote `Local` → `main`

Use a dedicated stable pull request only when explicitly approved.

The PR must:

- come from `Local`;
- pass `Stable Release Verify`;
- use a normal **merge commit**.

Do not synchronize main-only stable marker commits back into `Local` merely for ancestry.

Tags/releases are separate publishing actions and are never automatic.

## Before committing

Run the cheapest proof for the changed claim. Repository policy/documentation changes use:

```bash
python tools/verify_repository.py
```

Once application source exists, module-specific build/test checks become the primary proof and promotion gates will run the broader integration suite.

## Commit discipline

```text
feat:      new capability
fix:       behavior correction
refactor:  internal restructuring without intended behavior change
docs:      documentation/policy-only change
test:      test-only change
ci:        workflow/CI change
build:     dependency/toolchain change
release:   explicit release/publish state
chore:     bounded maintenance when no better category fits
```

## Pull requests

Use `.github/PULL_REQUEST_TEMPLATE.md`. Keep one PR scoped to one logical delivery.

## Data/security boundary

Do not commit credentials, private user/account material, entitlement-derived secrets, content decryption keys, or temporary transfer payloads.

See `SECURITY.md` and `docs/foundation/00-product-boundaries.md`.
