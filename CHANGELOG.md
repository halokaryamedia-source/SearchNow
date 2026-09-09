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
- Provider-neutral download transport registry and execution runtime.
- Deterministic `local-file` transport for end-to-end queue → transfer → progress → final publication testing.
- Native bounded worker scheduling with cooperative cancellation and queued-job pumping.
- Progress persistence checkpointing at 1 MiB plus lifecycle boundaries to reduce state-write overhead.
- Tauri-managed `DownloadExecutionRuntime` created during application bootstrap; download IPC commands delegate to RustCore runtime ownership.
- Provider-neutral `https-public` transport using pinned `ureq`/rustls with explicit timeout, redirect, HTTPS-only, response-size, and content-length safeguards.
- Manual redirect handling revalidates every destination and prevents production HTTPS downloads from downgrading to plain HTTP.
- Public HTTPS queue entries reject persisted query strings and embedded credentials so signed/authenticated request material remains a future runtime-resolver concern rather than download-state data.
- Deterministic local HTTP fixtures for success, redirects, timeout/failure, oversized responses, declared-length mismatch, and cooperative cancellation.
- Backend unit-test, clippy, architecture, source-size, and legacy-protected-content guards.
