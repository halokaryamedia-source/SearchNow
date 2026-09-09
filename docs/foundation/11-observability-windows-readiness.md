# 11 — Backend Observability & Windows Readiness

Status: **current observability/readiness contract**

## Goal

SearchNow must be diagnosable without turning diagnostics into a second logging system, a credential leak surface, or a reason core operations fail.

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

No local-PC testing is required to maintain this contract. Local smoke testing remains a later TARGET_WINDOWS evidence step.

## Diagnostic ownership

`EngineData/Backend/RustCore/src/diagnostics.rs` owns:

- `DiagnosticEvent`;
- `DiagnosticSeverity`;
- `DiagnosticComponent`;
- startup phase;
- backend health snapshot;
- bounded in-memory retention.

The default buffer retains 128 events and is hard-capped at 512. Older events are dropped rather than allowing unbounded memory growth.

Diagnostic failures are best-effort: poisoned/unavailable diagnostic state returns an unavailable/unknown snapshot and must not fail product operations.

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

Messages in the generic diagnostic buffer remain static SearchNow-owned text. Detailed errors stay in the normal typed error boundary and must be sanitized before any future persistent logging is considered.

## Instrumentation level

Instrument operation boundaries, not inner loops.

Current coarse events cover:

- backend startup and readiness;
- provider-runtime composition;
- download-runtime startup;
- settings load/save;
- Minecraft discovery;
- local library scan;
- package inspection;
- catalog query;
- download queue/cancel/retry/remove;
- aggregate runtime snapshot.

Do not emit per-file scan events, per-download-chunk events, HTTP header events, or provider payload dumps.

## Health semantics

`BackendHealthSnapshot` is safe public metadata. It may report:

- startup phase;
- healthy/degraded/unknown state;
- retained/dropped event counts;
- warning/error counts;
- last stable diagnostic code.

It is not a persistence database and does not expose secret-bearing session material.

## Windows remote readiness

Repository verification includes a hosted Windows gate after the normal Linux gate:

```text
Windows runner
├── cargo test RustCore
└── cargo check Tauri crate
```

This proves Windows compilation and deterministic Windows-compatible RustCore behavior without requiring the user's local PC.

Tauri 2 requires a Windows resource icon during `tauri-build`. Until final branding assets exist, `src-tauri/build.rs` generates a tiny build-only ICO under Cargo `OUT_DIR` on Windows and passes that path explicitly to `tauri-build`. The placeholder is never treated as a product branding asset and does not need to be committed as a binary file.

Final product branding must replace this build-only fallback later.

## Future local smoke evidence

`tools/windows_smoke_readiness.ps1` is prepared for later use. It is not automatically run on the user's PC.

When local testing becomes convenient, the smoke phase should cover:

1. Windows/AppData environment resolution;
2. RustCore + Tauri compile check;
3. Minecraft GDK/UWP candidate detection;
4. settings save/reload;
5. local package inspection;
6. local download finalization;
7. safe diagnostics inspection.

Local runtime claims must not be made until that smoke phase is actually performed.

## Security boundary

Observability and readiness must not reintroduce legacy BlueCoin protected-content/key behavior, secret sharing, or provider-specific credential persistence.

The Windows compile gate proves compilation only. It does not prove real Marketplace/provider compatibility, installed-app behavior, production TLS behavior, or user-machine performance.
