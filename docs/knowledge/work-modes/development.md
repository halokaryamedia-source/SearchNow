# Development Workflow

This is the human-readable lifecycle overview for non-trivial SearchNow repository/application Development. The executable procedure is `.agents/skills/development-brief/SKILL.md`.

## Entry

```text
AGENTS.md
→ CONTEXT.md
→ docs/knowledge/next-action.md
→ development-brief
→ exact owner
```

A read-only inspect/understand request stops after context recovery/reporting.

## Lifecycle

```text
recover current context
→ ground goal / scope / acceptance / proof
→ development needed?
   ├─ no → explain + minimum proof → STOP
   └─ yes
      → smallest relevant owner
      → smallest complete implementation
      → cheapest relevant proof
      → original-scope check
      → update only continuity/decision owner whose state changed
      → STOP
```

## Execution channel

```text
bounded repository text change
→ GitHub-capable channel

coordinated multi-file / local build-runtime need
→ real Git/Codex-style workspace

runtime/UI acceptance claim
→ actual runtime/UI capability
```

Do not create temporary Actions/helper architecture to emulate missing capability.

## Completion

Before reporting complete, re-check:

- original goal and out-of-scope boundary;
- 2–5 acceptance criteria;
- actual proof category;
- whether continuation/decision state truly changed.

Distinguish implementation from runtime verification.
