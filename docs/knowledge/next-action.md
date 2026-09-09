# Next Action

## Current Status

`BACKEND_LOCAL_CORE_READY`

Completed:

1. Tauri/Svelte/Rust application scaffold established;
2. backend core extracted into one in-process `EngineData/Backend/RustCore` library;
3. current Minecraft GDK storage discovery implemented with account-scoped roots;
4. Preview discovery remains opt-in;
5. legacy UWP discovery retained as compatibility fallback;
6. typed/versioned settings with staged persistence implemented;
7. bounded read-only local library indexing implemented for behavior/resource/skin packs and worlds;
8. invalid metadata is isolated per item instead of failing the scan;
9. Tauri commands are thin adapters and filesystem scans run through `spawn_blocking`;
10. backend unit/clippy/architecture gates added.

## Active Boundary

Keep work on `develop`. `Local` and `main` remain untouched until explicit promotion.

Backend work remains local-only. Catalog, remote authentication, downloads, protected-content processing, package mutation/export, and frontend wiring to the new backend commands are not part of this completed slice.

## Next Step

Continue backend-first with **Package Inspection Core**:

- read-only `.mcpack` / `.mcaddon` / folder inspection;
- typed manifest/content classification;
- BP/RP relationship detection;
- bounded archive safety rules;
- fixture-based tests;
- no export/mutation in the same slice.

Before that slice is considered runtime-verified, run current local core against a real Windows Minecraft Bedrock installation.
