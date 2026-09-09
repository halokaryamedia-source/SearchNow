# Next Action

## Current Status

`DEVELOPMENT_SYSTEM_READY`

SearchNow now uses the PRD-Creator-style repository development model adapted to application development.

Completed:

1. recovered and documented BlueCoin 2.4 legacy architecture;
2. established `develop` as active development branch;
3. retained `Local` as verified integration baseline and `main` as stable history;
4. established canonical development workflow and agent routing;
5. separated durable foundation policy, active knowledge/continuation, and legacy evidence;
6. established source authority, ownership, GitHub execution, verification, and promotion contracts;
7. added repository verification and promotion CI gates.

## Active Boundary

Keep new development on `develop`.

`Local` and `main` remain untouched until an explicit promotion is requested and the corresponding gate passes.

No application source implementation exists yet.

## Next Step

Finalize **Architecture & UX** before scaffolding source:

- choose the concrete Windows UI/runtime stack;
- freeze the target source/project tree;
- define core/service/UI boundaries and state ownership;
- define the first thin vertical slice and its falsifiable acceptance criteria.

Do not start broad implementation before these decisions are recorded.
