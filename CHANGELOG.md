# Changelog

All notable SearchNow repository/product changes will be recorded here.

## Unreleased

### Added

- Recovered BlueCoin 2.4 architecture and runtime documentation.
- PRD-Creator-style development routing and branch model adapted for SearchNow.
- Tauri 2 + Svelte 5 + TypeScript/Vite + Rust desktop scaffold.
- In-process `EngineData/Backend/RustCore` backend library separated from Tauri IPC.
- Typed/versioned settings with staged persistence.
- Minecraft Bedrock GDK/account-scoped storage discovery with Preview opt-in and legacy UWP fallback.
- Bounded read-only indexing for local behavior packs, resource packs, skin packs, and worlds.
- Read-only folder / `.mcpack` / `.mcaddon` package inspection with typed manifest classification.
- UUID dependency-based BP/RP relationship detection.
- ZIP archive safety checks for traversal, symlinks, duplicate paths, size bounds, and extreme compression ratios without extraction.
- Transport-agnostic persistent download manager with typed job states, bounded concurrency, cancellation/retry semantics, progress validation, restart recovery, and terminal-job cleanup.
- Download workspace planning with safe destination file names and destination-local staged atomic no-overwrite publication.
- Tauri-managed download queue state with transactional clone → persist → commit mutations.
- Backend unit-test, clippy, architecture, source-size, and legacy-protected-content guards.
