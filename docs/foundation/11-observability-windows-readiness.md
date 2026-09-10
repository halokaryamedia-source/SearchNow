# 11 — Backend Observability & Windows Readiness

Status: **remote foundation closure in progress**

## Goal

SearchNow must remain diagnosable, recoverable, and Windows-buildable without introducing a second logging/runtime system, raw transport control at the UI boundary, or build-only workaround paths.

```text
SearchNowBackendRuntime
├── bounded DiagnosticsBuffer
├── current BackendHealthSnapshot
├── coarse operation timings
├── stable result codes
├── provider-neutral download intent
└── recoverable persisted download state

GitHub Actions
├── Linux repository/backend/frontend gates
└── Windows RustCore + Tauri compile gate
```

## Diagnostic ownership

`EngineData/Backend/RustCore/src/diagnostics.rs` owns bounded in-memory diagnostic events, current per-component health, startup phase, and safe health snapshots. Diagnostics remain best-effort and must never break product operations.

Events contain only stable SearchNow-owned fields: timestamp, component, severity, code, static message, and optional duration. Absolute user paths, filenames supplied by users, query strings, Authorization/header values, cookies, access/refresh/session tokens, signed URLs, provider bodies, opaque provider-session payloads, and entitlement/protected-content material remain forbidden.

Historical event retention and current health are intentionally separate. A recovered component may report healthy while an older failure event remains available for diagnosis.

## Instrumentation level

Instrument operation boundaries only. Current events cover backend startup/readiness, provider/runtime composition, settings load/save, Minecraft discovery, library scan, package inspection, catalog query, download lifecycle commands, and aggregate runtime snapshots.

Do not emit per-file scan events, per-download-chunk events, headers, or provider payload dumps.

## Windows build contract

Repository verification runs RustCore tests and a native Tauri compile gate on `windows-latest` after the normal Linux verification gate.

Tauri uses normal committed application icon resources at:

```text
EngineData/Frontend/RustApp/src-tauri/icons/icon.png
EngineData/Frontend/RustApp/src-tauri/icons/icon.ico
```

`src-tauri/build.rs` uses the standard `tauri_build::build()` path. Build-time generated ICOs, `OUT_DIR` icon injection, and other implicit fallback workarounds are not part of the architecture. Final branding may replace the temporary icons later without changing build mechanics.

A green hosted Windows compile gate proves Windows compilation only. It does not prove installed-app runtime behavior, production provider compatibility, production TLS behavior, or user-machine performance.

## Persistence and recovery closure

Settings and download state use the shared `AtomicFileStore` persistence primitive. It stages and syncs writes, preserves a recoverable backup during replacement, recovers that backup if the primary is missing after an interrupted replacement, and removes stale temporary files opportunistically.

Download startup validates persisted job identities and state invariants before use. Duplicate or malformed job identities fail closed. The retained sequence is reconciled against existing job ids so a stale `next_sequence` value cannot reuse a prior identity.

Finalization uses a destination-local stage file. Startup removes stale stage files and reconciles a job left in `Finalizing` when the final file was already published with the expected size before the state commit completed. This prevents a successful publish from becoming a false retry conflict after a crash.

Download destination names are validated against Windows file-name restrictions, including reserved DOS device names, forbidden characters, path separators, and trailing dot/space rules.

## Download and provider boundary

Tauri accepts a provider-neutral `QueueCatalogDownloadRequest`; it does not accept a raw `DownloadRequest` or a caller-selected transport key. `SearchNowBackendRuntime` converts the catalog download reference to the internal transport identity.

The production runtime registers only the public HTTPS and provider-resolved transports. The deterministic `local-file` transport remains a test/development fixture and is not reachable from the normal product command surface.

Provider keys and stable provider resource ids use one canonical validation owner in `identity.rs`. Catalog, session, adapter, and resolved-download layers reuse that contract instead of carrying independent limits.

Scheduler continuation failures are retained as sanitized `scheduler_error` state in download snapshots rather than being silently discarded.

## Future local smoke evidence

`tools/windows_smoke_readiness.ps1` remains a later non-destructive TARGET_WINDOWS step. Local runtime claims must not be made until that smoke phase is actually performed.

The smoke phase should cover AppData resolution, current GDK/UWP Minecraft discovery, settings save/reload, package inspection, local/provider download finalization, safe diagnostics, and application launch.

## Security boundary

Observability/readiness work must not reintroduce legacy protected-content/key behavior, secret sharing, provider-specific credential persistence, raw transport selection from the UI, or hidden user/account-data transmission.
