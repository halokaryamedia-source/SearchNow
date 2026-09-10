# Current Validation

## Foundation-closure target

Target claim:

> SearchNow has one in-process application backend runtime, shared crash-recoverable file persistence, provider-neutral catalog/session/resolver/download composition, secret-safe diagnostics, and a native Windows Tauri compile gate.

## Last verified baseline before foundation-closure commit

The last `develop` workflow before this closure work established:

- Linux repository contract: PASS;
- RustCore format/tests/clippy: PASS — 62 tests, 0 failures on that commit;
- frontend architecture/source-size/type/build gates: PASS;
- Windows RustCore tests: PASS;
- Windows frontend build: PASS;
- Windows Tauri compile: **FAIL**, because `tauri::generate_context!()` still attempted to resolve the missing default `src-tauri/icons/icon.png`.

Therefore that commit is **not** a promotable clean baseline.

## Closure changes now expected to verify

- standard Tauri icon path exists at `src-tauri/icons/icon.png`;
- `src-tauri/build.rs` no longer generates an `OUT_DIR` placeholder icon;
- settings and download state use one `AtomicFileStore` replacement/recovery mechanism;
- settings backup recovery has deterministic regression coverage;
- repository contracts require the normal Tauri icon/build path rather than the obsolete workaround;
- documentation reflects the implemented backend instead of the old scaffold-only state.

Do not upgrade these expected results to PASS until the workflow for the closure commit is green.

## Existing backend evidence

The existing test suite already covers safe runtime startup, provider-resolver → download integration, fail-closed provider construction, bounded diagnostics, catalog validation, session refresh deduplication, HTTPS limits/redirect rules, download cancellation/recovery, package/archive safety, Minecraft discovery, library indexing, and settings persistence.

All active Tauri feature commands delegate through `State<SearchNowBackendRuntime>`; feature commands do not construct separate settings/download/provider engines.

## Claims not established by hosted CI

- installed Windows Tauri execution;
- real Windows AppData/Minecraft account-scoped behavior on a user machine;
- large-library/package performance on representative Windows machines;
- production HTTPS/TLS reliability against representative servers/CDNs;
- any real provider login/catalog/resource endpoint or provider-specific auth semantics;
- secure OS credential storage if a future provider requires durable user credentials;
- installer/clean-machine release behavior.

These remain TARGET_WINDOWS / REAL_FIXTURE / NETWORK / PROVIDER evidence.
