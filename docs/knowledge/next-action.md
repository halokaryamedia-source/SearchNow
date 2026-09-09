# Next Action

## Current Status

`ARCHITECTURE_SCAFFOLD_READY`

SearchNow now has an approved application architecture and initial source scaffold.

Completed:

1. BlueCoin 2.4 legacy behavior/source architecture recovered and preserved;
2. PRD-Creator-style repository development model established;
3. TranslateIT architecture reviewed as implementation reference;
4. Tauri 2 + Svelte 5 + TypeScript/Vite + Rust selected;
5. Python/second backend process intentionally excluded from current architecture;
6. Library / Discover / Downloads / Settings UI surfaces scaffolded;
7. product facade → Tauri API → Rust command → Rust engine boundary implemented;
8. runtime-status vertical slice added;
9. architecture/source-size/static build gates added.

## Active Boundary

Keep work on `develop`.

`Local` and `main` remain untouched until explicit promotion.

The application source now exists, but Minecraft/catalog/download/package behavior has **not** been implemented. Do not mistake scaffold UI for functional content-management behavior.

## Next Step

Implement the first real product vertical slice:

**Minecraft Installation Discovery → Library Snapshot**

Acceptance criteria:

1. Rust detects the supported/default Minecraft Bedrock data location or returns a truthful not-found state.
2. Discovery is local-only and performs no catalog/network/key-sharing behavior.
3. Tauri command delegates discovery to a bounded Rust engine owner.
4. Product facade exposes a user-readable installation/library snapshot without leaking internal transport details.
5. Library page renders detected/not-detected/error states from runtime truth.

Do not add catalog or download behavior in the same slice.
