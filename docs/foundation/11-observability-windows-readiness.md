# 11 — Backend Observability & Windows Readiness

Status: **current observability/readiness contract**

## Goal

SearchNow must be diagnosable without turning diagnostics into a second logging system, a credential leak surface, or a reason core operations fail. Build verification must also be deterministic enough that a green or red result can be reproduced from committed source and lockfiles.

```text
SearchNowBackendRuntime
├── bounded DiagnosticsBuffer
├── current component health
├── safe BackendHealthSnapshot
├── coarse operation timings
└── stable result codes

Repository
├── Cargo.lock
└── Frontend/package-lock.json

GitHub Actions
├── Linux repository/backend/frontend gates
└── Windows RustCore + Tauri compile gate
```

Local smoke testing remains a later TARGET_WINDOWS evidence step; it is not silently substituted by hosted CI.

## Diagnostic ownership

`EngineData/Backend/RustCore/src/diagnostics.rs` owns:

- `DiagnosticEvent`;
- `DiagnosticSeverity`;
- `DiagnosticComponent`;
- startup phase;
- current degraded-component state;
- backend health snapshot;
- bounded in-memory event retention.

The default event buffer retains 128 events and is hard-capped at 512. Older events are dropped rather than allowing unbounded memory growth.

Historical events and current health have separate semantics:

```text
retained events     = what happened recently
component health    = what is currently degraded
```

A successful operation clears the current error degradation for its component without deleting historical error events. A transient error therefore does not keep health degraded merely because the old event remains in the ring buffer.

Diagnostic failures remain best-effort: poisoned/unavailable diagnostic state returns an unavailable/unknown snapshot and must not break core product operations.

## Safe event contract

Events contain only:

```text
timestamp
a coarse component
severity
stable static code
safe static message
optional duration
```

Do not put into diagnostics:

- absolute user paths;
- filenames supplied by users;
- query strings;
- Authorization/header values;
- cookies;
- access/refresh/session tokens;
- signed URLs;
- provider response bodies;
- opaque provider-session payloads;
- entitlement/protected-content material.

Detailed errors stay in their typed boundary and must be sanitized before any future persistent logging is considered.

## Instrumentation level

Instrument operation boundaries, not inner loops. Current coarse events cover backend startup, provider/download runtime composition, settings, Minecraft discovery, library scan, package inspection, catalog query, download lifecycle commands, and aggregate runtime snapshot.

Do not emit per-file scan events, per-download-chunk events, HTTP header events, or provider payload dumps.

## Download scheduler visibility

A scheduler/persistence failure after a worker completes must not disappear silently. `DownloadManagerSnapshot` may expose one safe scheduler failure code/message so the product can distinguish a healthy idle queue from a queue that cannot currently advance. The failure contains no runtime credential material.

## Deterministic dependency verification

The repository owns one Rust workspace lock and one frontend npm lock:

```text
Cargo.toml
Cargo.lock
EngineData/Frontend/RustApp/package-lock.json
```

CI uses locked resolution:

```text
npm ci
cargo ... --locked
```

Do not return to floating CI installs unless the dependency policy is explicitly revised.

## Windows remote readiness

Repository verification runs a hosted Windows gate after the normal Linux gate:

```text
Windows runner
├── cargo test -p searchnow-core --locked
├── npm ci
├── build Svelte/Vite frontend
└── cargo check -p searchnow --locked
```

The committed `src-tauri/icons/icon.png` satisfies Tauri context generation. Windows resource compilation may continue using the build-only ICO generated under Cargo `OUT_DIR` until final branding assets are approved. Build placeholders are not product branding.

A green hosted Windows compile proves compilation and deterministic Windows-compatible RustCore behavior only. It does not prove installed-app behavior.

## Future local smoke evidence

`tools/windows_smoke_readiness.ps1` is the non-destructive readiness entrypoint. It checks current GDK and legacy UWP candidate locations and can run the same locked frontend/Rust compilation prerequisites.

When local testing becomes convenient, manual smoke evidence should cover:

1. Windows/AppData environment resolution;
2. actual Bedrock GDK/UWP account discovery;
3. settings save/reload/recovery;
4. local package inspection;
5. download state startup/reconciliation and stale staging cleanup;
6. safe diagnostics inspection.

Local runtime claims must not be made until those checks are actually performed.

## Security boundary

Observability/readiness must not reintroduce legacy protected-content/key behavior, hidden upload, secret sharing, or provider-specific credential persistence. Production Tauri IPC also does not expose the deterministic `local-file` fixture or raw transport-selecting enqueue requests.
