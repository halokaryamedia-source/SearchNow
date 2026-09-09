# SearchNow

SearchNow is a documentation-first modernization of the inspected BlueCoin 2.4 desktop application into a clearer, maintainable Minecraft Bedrock content-management client.

The legacy executable is preserved only as behavioral and architectural evidence. SearchNow does **not** treat DRM bypass, paid-to-free conversion, protected-content key distribution, or hidden credential behavior as product requirements.

## Canonical Development Workflow

```text
Context Recovery
→ Product Requirements
→ Architecture & UX
→ Implementation
→ Verification
→ Local Promotion
→ Stable Promotion
```

These names are used consistently in repository policy, continuation notes, development reports, and handoff.

## Branch Model

```text
develop  → active repository development
Local    → verified integration baseline; one squash commit per approved promotion
main     → stable repository history
```

Routine work happens on `develop`.

- `develop → Local`: dedicated PR, promotion gate, **squash merge**.
- after promotion: synchronize `develop` to the resulting `Local` HEAD before new work.
- `Local → main`: explicit stable PR, stable gate, normal **merge commit**.
- tags/releases are separate publishing actions.

## Repository Map

```text
AGENTS.md            work routing, authority, boot, continuity, branch kernel
GITHUB_RULES.md      GitHub mutation, proof, CI, promotion discipline
CONTEXT.md           stable product/repository orientation
docs/foundation/     durable SearchNow product/development policy
docs/knowledge/      active continuation, ownership, decisions, evidence
docs/legacy/         recovered BlueCoin 2.4 evidence and architecture
.agents/skills/      reusable development judgment
tools/               repository verification/operator utilities
.github/             PR template and promotion gates
src/                 application source once implementation begins
tests/               executable regression contracts once implementation begins
```

## Development Boot

For non-trivial development:

```text
AGENTS.md
→ CONTEXT.md
→ docs/knowledge/next-action.md
→ .agents/skills/development-brief/SKILL.md
→ exact owner
→ GITHUB_RULES.md before material GitHub mutation
```

Do not broad-read the repository when the smallest owner can settle the task.

## Current Stage

The recovered legacy architecture is documented. The repository workflow is being standardized before application source implementation begins.

Repository contract check:

```bash
python tools/verify_repository.py
```

See [docs/README.md](docs/README.md) for the documentation map.
