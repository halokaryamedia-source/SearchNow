# SearchNow Documentation

SearchNow follows the same repository-memory and development-routing pattern as PRD-Creator, adapted to a Windows desktop application.

## Documentation Map

```text
docs/
├── foundation/   durable product, architecture, development and promotion policy
├── knowledge/    active continuation, ownership, decisions, reviews and work modes
└── legacy/       recovered BlueCoin 2.4 evidence
```

### Foundation

- [`foundation/00-product-boundaries.md`](foundation/00-product-boundaries.md)
- [`foundation/01-development-flow.md`](foundation/01-development-flow.md)
- [`foundation/02-target-product-flow.md`](foundation/02-target-product-flow.md)
- [`foundation/03-implementation-roadmap.md`](foundation/03-implementation-roadmap.md)
- [`foundation/04-verification-promotion.md`](foundation/04-verification-promotion.md)

### Knowledge / continuity

- [`knowledge/next-action.md`](knowledge/next-action.md) — active continuation only.
- [`knowledge/ownership.md`](knowledge/ownership.md) — who owns what.
- [`knowledge/source-authority.md`](knowledge/source-authority.md) — evidence/decision precedence.
- [`knowledge/work-routing.md`](knowledge/work-routing.md) — compact mode routing.
- [`knowledge/work-modes/development.md`](knowledge/work-modes/development.md)
- [`knowledge/work-modes/maintenance.md`](knowledge/work-modes/maintenance.md)

### Legacy evidence

- [`legacy/01-current-state.md`](legacy/01-current-state.md)
- [`legacy/04-recovered-source-architecture.md`](legacy/04-recovered-source-architecture.md)
- [`legacy/05-recovered-symbol-map.md`](legacy/05-recovered-symbol-map.md)
- [`legacy/06-runtime-data-contracts.md`](legacy/06-runtime-data-contracts.md)
- [`legacy/07-reconstruction-evidence.md`](legacy/07-reconstruction-evidence.md)

## Authority rule

```text
current explicit user instruction
→ approved SearchNow decisions
→ current SearchNow requirements/foundation
→ legacy evidence when legacy behavior is the question
→ implementation
→ tests/runtime/generated evidence
```

Do not use generated/build output to repair upstream product or architecture meaning.
