# SearchNow Rust Core

`RustCore` is the in-process backend/domain library for SearchNow. It is linked into the Tauri application; it is **not** a second process or local server.

## Ownership

```text
RustCore
├─ app_runtime       single application backend composition root
├─ settings/storage typed settings + shared atomic persistence/recovery
├─ platform          process/platform path context
├─ minecraft/library Windows Bedrock discovery + bounded local indexing
├─ package/          read-only package inspection + archive safety
├─ identity          canonical provider/resource identity validation
├─ catalog/          provider-neutral catalog domain
├─ provider_session/ runtime-only provider session ownership
├─ provider_adapter/ integrated provider composition
├─ download/         persistent recoverable execution + transports/resolution
├─ diagnostics       bounded events + current component health
└─ runtime           backend runtime identity
```

The Tauri `commands/` layer owns IPC adaptation only. Frontend code must not own filesystem discovery, provider/session truth, download transports, local library truth, or settings persistence.

## Current rules

- Windows is the supported runtime target.
- Detect current GDK storage first; support legacy UWP as an optional compatibility fallback.
- Local discovery is read-only and performs no hidden network calls.
- Do not inspect entitlement/decryption-key material.
- Library scans are shallow and bounded; do not recursively size/hash entire packs during normal discovery.
- Invalid content becomes a truthful item/warning rather than crashing the whole scan.
- Settings and download state share one crash-recoverable persistence primitive.
- Persisted download jobs fail closed when malformed and reconcile interrupted finalization on startup.
- Production download transports are selected inside the backend from product/provider intent.
- Provider/session runtime credentials remain non-persisted and secret-safe.
- Historical diagnostics and current component health are distinct concepts.

## Verification

The standalone dependency graph is committed in `Cargo.lock`.

```bash
cargo test --locked --manifest-path EngineData/Backend/RustCore/Cargo.toml
cargo clippy --locked --manifest-path EngineData/Backend/RustCore/Cargo.toml --all-targets -- -D warnings
```

The current remote foundation baseline contains 71 passing RustCore tests. Hosted CI repeats the suite on Windows before compiling the Tauri application with its own locked dependency graph.
