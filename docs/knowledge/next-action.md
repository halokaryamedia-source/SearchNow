# Next Action

## Current Status

`APPLICATION_BACKEND_RUNTIME_READY`

Completed:

1. Tauri/Svelte/Rust desktop scaffold and PRD-Creator-style development workflow are established;
2. local settings, Minecraft discovery, bounded library indexing, and read-only package inspection are implemented in RustCore;
3. persistent download lifecycle, bounded execution, safe workspace/finalization, `local-file`, `https-public`, and `provider-resolved` transports are implemented;
4. provider-neutral catalog, runtime resource resolver, shared provider-session manager, and integrated-provider composition are implemented;
5. `SearchNowBackendRuntime` now composes SettingsStore/PlatformContext, ProviderAdapterRuntime, and DownloadExecutionRuntime into one application backend owner;
6. provider-resolved application downloads use the exact `ResourceResolverRegistry` created by the composed provider runtime;
7. `BackendRuntimeSnapshot` exposes only safe runtime/Minecraft/provider/download aggregate state;
8. Tauri bootstrap manages one `SearchNowBackendRuntime`; feature commands no longer construct separate settings/download/provider runtime owners;
9. filesystem-heavy local operations remain on `spawn_blocking`, while download execution keeps its bounded worker/thread model;
10. settings and download persistence remain in their existing stores; no second application database was introduced;
11. deterministic application-runtime tests cover safe startup, provider-resolver → download integration, and fail-closed provider construction;
12. repository CI passes with **59 RustCore tests**, strict clippy, Tauri formatting, architecture/source-size validation, Svelte typecheck, and frontend build.

## Active Boundary

Keep work on `develop`. `Local` and `main` remain untouched until explicit promotion.

No real provider login, PlayFab/Marketplace endpoint, hardcoded provider secret, protected-content bypass, package mutation/export, or frontend provider/Discover wiring is implemented.

Hosted CI still does not replace installed Windows/runtime/network evidence.

## Next Step

Continue backend-first with **Backend Observability / Health / Windows Readiness Boundary**.

Prepare the generic backend for reliable local testing before any real provider is added:

- define safe structured diagnostic events with component, stable code, severity, timestamp/duration, and redacted message fields;
- add a bounded in-memory diagnostic/health buffer rather than unbounded logging;
- expose safe application health/startup phase information through `SearchNowBackendRuntime` without credentials, request headers, signed URLs, opaque provider session values, or raw provider bodies;
- instrument important backend boundaries (startup, local discovery/library/package, catalog/session/resolver, download lifecycle) with coarse timings and stable result codes while avoiding per-chunk/per-file spam;
- define redaction rules for user paths and network/provider error details before any persistent diagnostic log is enabled;
- add TARGET_WINDOWS readiness gates, prioritizing a real Windows Tauri `cargo check`/build compile gate because hosted Linux currently only formats the Tauri crate;
- add a compact Windows smoke-test checklist/script for AppData path resolution, Minecraft GDK/UWP discovery, settings persistence, package inspection, and download finalization;
- keep diagnostics best-effort so diagnostic failures cannot break core product operations;
- keep real PlayFab/Marketplace/provider credentials and frontend feature wiring out of this slice.

Do not implement DRM/key-sharing/protected-content bypass paths.
