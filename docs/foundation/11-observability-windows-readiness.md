# 11 — Backend Observability & Windows Readiness

Status: **foundation closure in progress**

## Goal

SearchNow must remain diagnosable and Windows-buildable without introducing a second logging/runtime system or build-only workaround paths.

```text
SearchNowBackendRuntime
├── bounded DiagnosticsBuffer
├── safe BackendHealthSnapshot
├── coarse operation timings
└── stable result codes

GitHub Actions
├── Linux repository/backend/frontend gates
└── Windows RustCore + Tauri compile gate
```

## Diagnostic ownership

`EngineData/Backend/RustCore/src/diagnostics.rs` owns bounded in-memory diagnostic events, severity/component metadata, startup phase, and safe health snapshots. Diagnostics remain best-effort and must never break product operations.

Events contain only stable SearchNow-owned fields: timestamp, component, severity, code, static message, and optional duration. Absolute user paths, filenames supplied by users, query strings, Authorization/header values, cookies, access/refresh/session tokens, signed URLs, provider bodies, opaque provider-session payloads, and entitlement/protected-content material remain forbidden.

## Instrumentation level

Instrument operation boundaries only. Current events cover backend startup/readiness, provider/runtime composition, settings load/save, Minecraft discovery, library scan, package inspection, catalog query, download lifecycle commands, and aggregate runtime snapshots.

Do not emit per-file scan events, per-download-chunk events, headers, or provider payload dumps.

## Windows build contract

Repository verification runs RustCore tests and a native Tauri compile gate on `windows-latest` after the normal Linux verification gate.

Tauri uses a normal committed application icon at:

```text
EngineData/Frontend/RustApp/src-tauri/icons/icon.png
```

`src-tauri/build.rs` uses the standard `tauri_build::build()` path. Build-time generated ICOs, `OUT_DIR` icon injection, and other implicit fallback workarounds are not part of the architecture. Final branding may replace the temporary icon later without changing build mechanics.

A green hosted Windows compile gate proves Windows compilation only. It does not prove installed-app runtime behavior, production provider compatibility, production TLS behavior, or user-machine performance.

## Persistence/readiness closure

Settings and download state use the shared `AtomicFileStore` persistence primitive. It stages and syncs writes, preserves a recoverable backup during replacement, recovers that backup if the primary is missing after an interrupted replacement, and removes stale temporary files opportunistically.

This keeps recovery mechanics in one owner instead of duplicating replacement logic across feature stores.

## Future local smoke evidence

`tools/windows_smoke_readiness.ps1` remains a later non-destructive TARGET_WINDOWS step. Local runtime claims must not be made until that smoke phase is actually performed.

The smoke phase should cover AppData resolution, current GDK/UWP Minecraft discovery, settings save/reload, package inspection, local download finalization, safe diagnostics, and application launch.

## Security boundary

Observability/readiness work must not reintroduce legacy protected-content/key behavior, secret sharing, provider-specific credential persistence, or hidden user/account-data transmission.
