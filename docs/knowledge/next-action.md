# Next Action

## Current Status

`PACKAGE_INSPECTION_CORE_READY`

Completed:

1. Tauri/Svelte/Rust application scaffold established;
2. backend core isolated as one in-process `EngineData/Backend/RustCore` library;
3. typed/versioned settings with staged persistence implemented;
4. current Minecraft GDK/account storage discovery plus Preview opt-in and legacy UWP fallback implemented;
5. bounded read-only local library indexing implemented;
6. read-only folder / `.mcpack` / `.mcaddon` package inspection implemented;
7. manifest module classification implemented for behavior/resource/skin/world-template packs;
8. BP↔RP relationships resolve from dependency UUIDs, not naming heuristics;
9. ZIP inspection rejects traversal, symlink, duplicate-path, excessive-size, and extreme-ratio conditions without extracting archives;
10. package inspection is exposed through a thin Tauri `spawn_blocking` adapter;
11. repository CI passes with 15 RustCore tests and strict clippy.

## Active Boundary

Keep work on `develop`. `Local` and `main` remain untouched until explicit promotion.

Catalog/auth, remote downloading, protected-content processing, package mutation/export, and frontend wiring to the package command remain outside the completed slice.

Hosted CI does not replace a real Windows/Minecraft/package test.

## Next Step

Continue backend-first with **Download Manager Core (transport-agnostic)**.

Build only queue/job lifecycle first:

- typed `DownloadJob` / state machine;
- bounded concurrent-job policy;
- cancellation and retry semantics;
- progress snapshot model;
- persistence/recovery contract for queued/incomplete jobs;
- destination/workspace ownership and atomic finalization boundary;
- unit tests for legal/illegal state transitions;
- no Marketplace-specific authentication/catalog logic in the same slice.

Do not implement DRM/key-sharing/protected-content bypass paths.
