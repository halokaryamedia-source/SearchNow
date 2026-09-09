# Workspace Agent Routing

SearchNow is repository-backed system memory. Current explicit user intent and current repository/project authority outrank chat history. Use the smallest owner and proof set that can settle the task.

## Canonical workflow names

Use these names everywhere in current policy, routing, handoff, and user-facing development reporting:

```text
Context Recovery
→ Product Requirements
→ Architecture & UX
→ Implementation
→ Verification
→ Local Promotion
→ Stable Promotion
```

Do not invent numbered stage aliases.

## Core routing

| Intent | Mode | Start |
|---|---|---|
| Inspect, understand, recover, compare, decide | Plan | current state + smallest relevant owner; read-only unless change requested |
| Add/change SearchNow capability, architecture, UX, policy, tooling, contracts | Development | `development-brief` + exact owner |
| Fix bounded bug, regression, stale routing/docs, behavior-preserving defect | Maintenance | concrete failure/target → first wrong owner |
| Promote verified work between repository baselines | Promotion | branch policy + matching gate |

## Boot

### Observe / recover

```text
AGENTS.md
→ CONTEXT.md
→ docs/knowledge/next-action.md
→ smallest owner needed
→ report current understanding
→ STOP
```

Read `GITHUB_RULES.md` only when GitHub mutation/history/CI/promotion behavior can affect the task.

### Development

```text
AGENTS.md
→ CONTEXT.md
→ docs/knowledge/next-action.md
→ .agents/skills/development-brief/SKILL.md
→ exact owner
→ GITHUB_RULES.md before material GitHub mutation
```

### Maintenance

```text
concrete failure or explicit target
→ first wrong owner
→ exact owner
→ GITHUB_RULES.md before material GitHub mutation
→ smallest falsifiable proof
→ STOP
```

Do not broad-read the repository or replay the entire product history for bounded work.

## Authority

Use the nearest authority for each claim:

1. current explicit user instruction;
2. approved SearchNow decisions;
3. authoritative current SearchNow requirement/source;
4. recovered BlueCoin evidence when the question is about legacy behavior;
5. durable SearchNow foundation/architecture contract;
6. current implementation source;
7. tests, CI, runtime observations, and Git history as evidence.

Recovered legacy behavior does not outrank explicit SearchNow product decisions. Generated/build output never outranks canonical source.

## First wrong owner

```text
product goal / allowed behavior wrong          → Product Requirements
legacy behavior understanding wrong            → docs/legacy/
UX / component boundary / architecture wrong   → Architecture & UX owner
implementation wrong                            → exact source owner
implementation correct, test stale              → test
implementation/test correct, CI wrong           → workflow / repository policy
release/promotion state wrong                    → branch/promotion policy
derived artifact wrong                          → upstream canonical owner
```

Do not repair upstream defects with UI workarounds, compatibility layers, or generated-file patches.

## Canonical and derived state

```text
user requirement / approved decision / evidence
→ canonical requirement + architecture
→ implementation source
→ build/generated artifact
→ verification evidence
```

Fix the first wrong canonical owner and regenerate/rebuild downstream state.

## Repository continuity

- `CONTEXT.md` → stable product/repository orientation.
- `docs/knowledge/next-action.md` → active continuation only.
- `docs/knowledge/decisions/` → durable rationale.
- `docs/foundation/` → durable product/development policy.
- `docs/legacy/` → recovered BlueCoin 2.4 evidence only.
- source/tests → implementation and executable proof.
- reviews/history → supporting evidence only when needed.

## Branches

```text
develop  → active repository development
Local    → verified integration baseline; develop→Local uses squash
main     → stable history; Local→main requires explicit stable promotion
```

After an approved `develop → Local` squash, synchronize `develop` to the resulting `Local` HEAD before new development.

## Product boundaries

SearchNow may modernize legitimate catalog browsing, owned/local content discovery, download management through permitted paths, package inspection, validation, organization, diagnostics, and authorized export/conversion.

Do not implement or optimize:

- Marketplace DRM/protection bypass;
- converting paid/restricted content into free/unrestricted content;
- extraction, pooling, upload, or distribution of protected-content decryption keys;
- hidden collection/transmission of entitlement-derived secrets or credentials.

Legacy routines related to those behaviors remain forensic documentation only.

## User-facing communication

Default to concise, result-first reporting. Distinguish `implemented`, `repository/static verified`, `runtime verified`, and `not verified`.

Stop when requested scope is complete and the cheapest sufficient evidence supports the claim.
