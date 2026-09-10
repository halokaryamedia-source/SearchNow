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
- Normal committed Tauri application icon path used by standard `tauri_build::build()`.

### Changed

- SettingsStore and DownloadStore no longer maintain separate file-replacement/recovery implementations; both delegate to the shared atomic persistence primitive.
- Windows build readiness no longer depends on generated `OUT_DIR` placeholder icons or `window_icon_path` build-script injection.
- Repository validation now guards the standard icon/build contract and shared persistence ownership instead of freezing temporary workaround details.
- README, implementation roadmap, next-action, validation, and observability documentation now describe the implemented backend rather than the initial scaffold state.

### Safety / Architecture

- Runtime credential material remains excluded from persisted download/catalog DTOs and public provider status.
- Provider-session material remains opaque, runtime-only, non-serializable, and non-Debug.
- Protected-content bypass, key distribution, and hidden entitlement-data transmission remain outside the product boundary.
