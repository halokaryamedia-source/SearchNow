# Work Routing

Compact human reference only. Root `AGENTS.md` is canonical.

## Mode map

| User intent | Mode | Primary route |
|---|---|---|
| inspect, understand, compare, recover context | Plan | current authority + smallest relevant owner |
| change product behavior, architecture, UX, source, tooling, policy | Development | `development-brief` + first wrong owner |
| fix bounded defect/regression/stale behavior | Maintenance | concrete failure/target + first wrong owner |
| move verified state between branches | Promotion | branch policy + matching verification gate |

## Owner rule

Route by what is wrong, not by file type:

```text
product behavior/scope wrong
→ product boundary / target-flow owner

legacy understanding wrong
→ exact docs/legacy owner

architecture/UX meaning wrong
→ exact architecture/UX owner

meaning correct; code behavior wrong
→ exact implementation owner

implementation correct; test/CI wrong
→ test or repository engineering owner
```

## Continuity

```text
current continuation → next-action.md
future/non-active work → operations/backlog.md
validation evidence → reviews/
durable rationale → decisions/
```

Historical notes do not become active work unless current intent promotes them.
