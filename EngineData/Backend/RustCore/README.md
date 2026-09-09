# SearchNow Rust Core

`RustCore` is the in-process backend/domain library for SearchNow. It is linked into the Tauri application; it is **not** a second process or local server.

## Ownership

```text
RustCore
├─ settings      typed/versioned local settings + staged persistence
├─ platform      process/platform path context
├─ minecraft     Windows Bedrock storage discovery
├─ library       bounded read-only local content indexing
└─ runtime       backend runtime identity
```

The Tauri `commands/` layer owns IPC adaptation only. Frontend code must not own filesystem discovery, local library truth, or settings persistence.

## Current rules

- Windows is the supported runtime target.
- Detect current GDK storage first; support legacy UWP as an optional compatibility fallback.
- Local discovery is read-only and performs no network calls.
- Do not inspect entitlement/decryption-key material.
- Library scans are shallow and bounded; do not recursively size/hash entire packs during normal discovery.
- Invalid content becomes a truthful item/warning rather than crashing the whole scan.

## Verification

```bash
cargo test --manifest-path EngineData/Backend/RustCore/Cargo.toml
cargo clippy --manifest-path EngineData/Backend/RustCore/Cargo.toml --all-targets -- -D warnings
```
