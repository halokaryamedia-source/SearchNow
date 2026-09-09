# SearchNow

SearchNow is currently in its documentation-first planning stage.

Development planning is maintained on the `Local` branch before implementation begins.

## Start Here

- [`docs/README.md`](docs/README.md) — full documentation index and current status.
- [`docs/01-current-state.md`](docs/01-current-state.md) — behavior baseline reconstructed from BlueCoin 2.4.
- [`docs/04-recovered-source-architecture.md`](docs/04-recovered-source-architecture.md) — **primary legacy source architecture reference**.
- [`docs/05-recovered-symbol-map.md`](docs/05-recovered-symbol-map.md) — recovered CLR types, fields, methods, generated helpers, events, and call relationships.
- [`docs/06-runtime-data-contracts.md`](docs/06-runtime-data-contracts.md) — filesystem, settings, network, package, runtime-state, and data-boundary map.
- [`docs/07-reconstruction-evidence.md`](docs/07-reconstruction-evidence.md) — evidence, bundle manifest, hashes, metadata counts, and confidence levels.
- [`docs/02-target-product-flow.md`](docs/02-target-product-flow.md) — proposed SearchNow user workflow and UX model.
- [`docs/03-implementation-roadmap.md`](docs/03-implementation-roadmap.md) — architecture and staged development plan.

## Current Stage

```text
Legacy architecture capture       ✓
        ↓
Product / UX specification
        ↓
Technical decisions
        ↓
Repository foundation
        ↓
Implementation
        ↓
Testing / release
```

## Legacy Reference Status

The supplied `BlueCoin_2.4.exe` has been statically decomposed to its .NET single-file manifest and embedded application assembly. The documentation now records the meaningful source types, responsibilities, internal call chains, local files, configuration, network boundaries, and runtime data flow.

The executable was not run during this architecture pass, and the repository does not contain BlueCoin's original source code. Reconstructed physical `.cs` file layout is therefore explicitly distinguished from directly observed CLR metadata.

Sensitive credential values found in the legacy executable are intentionally not copied here. Legacy protected-content bypass/key-sharing behavior is documented only as historical architecture and is not a SearchNow implementation requirement.

## Working Rule

Do not begin implementation by rediscovering or copying the legacy `DownloadForm` architecture. Use the recovered documents as the baseline, then implement the separated SearchNow architecture defined by the product and technical specifications.
