# Changelog

All notable SearchNow repository/product changes will be recorded here.

## Unreleased

### Added

- Recovered BlueCoin 2.4 architecture and runtime documentation.
- PRD-Creator-style development routing and branch model adapted for SearchNow.
- Tauri 2 + Svelte 5 + TypeScript/Vite + Rust desktop scaffold.
- In-process `EngineData/Backend/RustCore` backend library separated from Tauri IPC.
- Typed/versioned settings and Minecraft Bedrock GDK/account-scoped discovery.
- Bounded read-only local behavior/resource/skin-pack and world indexing.
- Read-only folder / `.mcpack` / `.mcaddon` package inspection with manifest classification, BP/RP dependency detection, and ZIP traversal/symlink/size/compression-ratio safeguards.
- Persistent bounded download manager with typed lifecycle, cancellation/retry, restart recovery, progress checkpointing, safe workspace planning, and atomic no-overwrite publication.
- Provider-neutral `https-public` and credential-safe `provider-resolved` transports with timeout, redirect, response-size, resolver-expiry, and sensitive-runtime-material safeguards.
- Provider-neutral catalog, shared runtime-only provider-session manager, resource resolver registry, and integrated-provider composition.
- Consolidated `SearchNowBackendRuntime` as the single application backend owner managed by Tauri.
- Bounded secret-safe diagnostics and hosted Windows RustCore/Tauri compile gates.
- Shared `AtomicFileStore` for staged persistence, backup recovery, and stale temporary-file cleanup.
- Normal committed Tauri PNG/ICO application resources used by standard `tauri_build::build()`.
- Canonical provider/resource identity validation shared by catalog, session, adapter, and resolved-download layers.
- Persisted download-state validation, duplicate-id rejection, stale-sequence reconciliation, destination-stage cleanup, and interrupted-finalization recovery.
- Windows-safe destination filename validation, including reserved DOS device names and trailing dot/space rules.
- Provider-neutral `QueueCatalogDownloadRequest` product intent at the Tauri IPC boundary.
- Sanitized download scheduler continuation-error state exposed through download snapshots.
- Committed `package-lock.json` plus standalone RustCore and Tauri `Cargo.lock` dependency graphs.

### Changed

- SettingsStore and DownloadStore no longer maintain separate file-replacement/recovery implementations; both delegate to the shared atomic persistence primitive.
- Production application runtime no longer registers the deterministic `local-file` fixture transport.
- Tauri download commands no longer accept raw `DownloadRequest`/transport selection from the frontend.
- Provider/resource validation no longer carries independent size/key rules across runtime layers.
- Diagnostics current component health is independent from retained historical failure events.
- Download scheduler continuation errors are retained instead of silently discarded.
- Windows build readiness no longer depends on generated `OUT_DIR` placeholder icons or `window_icon_path` build-script injection.
- Repository, Local-promotion, and stable-release verification now use current GitHub Actions v7 releases, `npm ci`, Cargo `--locked`, and read-only repository permissions.
- Repository validation now requires committed dependency locks and guards the product-intent/production-transport boundary.
- README, context, implementation roadmap, next-action, validation, backlog, and observability documentation describe the closed remote foundation rather than the initial scaffold state.

### Verified

- Remote foundation verification passes 71 RustCore tests with strict Clippy, frontend architecture/size/typecheck/build checks, and native hosted-Windows RustCore/Tauri locked compilation.

### Safety / Architecture

- Runtime credential material remains excluded from persisted download/catalog DTOs and public provider status.
- Provider-session material remains opaque, runtime-only, non-serializable, and non-Debug.
- Product UI/IPC cannot select arbitrary internal download transports.
- Protected-content bypass, key distribution, and hidden entitlement-data transmission remain outside the product boundary.
