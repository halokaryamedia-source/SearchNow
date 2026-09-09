# SearchNow Documentation

This branch is the planning and development workspace for SearchNow.

The repository was initially empty. Documentation is being established before implementation so product flow, architecture, scope, privacy, and development priorities remain explicit.

## Documentation Index

1. [`01-current-state.md`](01-current-state.md) — baseline reconstructed from static analysis of the supplied BlueCoin 2.4 executable.
2. [`02-target-product-flow.md`](02-target-product-flow.md) — proposed user-facing workflow and UX model.
3. [`03-implementation-roadmap.md`](03-implementation-roadmap.md) — phased development plan and technical boundaries.
4. [`04-recovered-source-architecture.md`](04-recovered-source-architecture.md) — detailed source-level reconstruction: modules, classes, responsibilities, startup/search/download call chains, filesystem/network architecture, and migration map.
5. [`05-recovered-symbol-map.md`](05-recovered-symbol-map.md) — CLR symbol inventory for meaningful types, fields, methods, generated helpers, event bindings, and high-value internal call relationships.
6. [`06-runtime-data-contracts.md`](06-runtime-data-contracts.md) — local files, configuration, runtime state, package metadata, network contracts, download state, and target data-ownership boundaries.
7. [`07-reconstruction-evidence.md`](07-reconstruction-evidence.md) — artifact hash, .NET bundle manifest, CLR evidence, analysis method, and confidence matrix.

## Current Status

- Repository initialized.
- `Local` branch created for planning and development.
- No SearchNow source implementation has been added yet.
- Legacy BlueCoin architecture has been statically mapped before redesign begins.
- `BlueCoin_2.4.exe` was **not executed** during the architecture pass.
- Current-state knowledge comes from static analysis of the supplied executable, not from original BlueCoin source code.

## Legacy Architecture Freeze

The recovered BlueCoin architecture is now documented at four levels:

```text
Behavior baseline
    ↓
Source/module architecture
    ↓
CLR symbol map
    ↓
Runtime data/network/filesystem contracts
    ↓
Evidence + confidence boundary
```

Normal SearchNow development should use these documents as the legacy reference instead of repeatedly reverse-engineering the executable.

Additional legacy analysis should only be performed when a specific unresolved behavior blocks a documented requirement.

## Security / Product Boundary

Sensitive credential values discovered in the executable are intentionally not copied into this repository. Legacy protected-content bypass and shared decryption-key behavior are documented only as architectural facts; they are not SearchNow product requirements.

## Working Principle

Documentation is authoritative before implementation. Significant behavior should first be represented in the product flow or architecture documentation, then implemented and tested against that documented behavior.
