# Current Validation

Reviewed: **2026-09-10**  
Scope: `develop` remote backend foundation + provider-independent frontend wiring  
Status: **remote code/static/hosted compile verification complete; target-Windows installed/runtime smoke pending**

## Latest frontend code proof

Frontend code baseline:

- commit: `c2ffdad5fef9b03df13d61335ee929ffe43f92be`;
- GitHub Actions: `Repository Verify` run **#168**;
- Linux repository/backend/frontend verification: **PASS**;
- hosted Windows verification: **PASS on rerun of the same commit SHA**.

The first Windows attempt hit one transient timeout in the local HTTP fixture test `stalled_response_body_times_out_as_retryable`. No Rust/source change was made. Re-running the Windows job for the identical commit passed the 71-test RustCore suite, frontend build, and native Tauri compile gate, which classifies the first result as runner/test-fixture timing noise rather than a frontend regression.

## Verified remotely

The current baseline proves:

- repository contracts pass;
- Rust formatting passes;
- **71 RustCore tests pass**;
- strict Clippy passes with warnings denied;
- Tauri adapter formatting passes;
- frontend dependencies install from committed `package-lock.json` via `npm ci`;
- frontend architecture and source-size contracts pass;
- Svelte/TypeScript checking passes without frontend compile errors;
- production frontend build passes;
- Rust dependency resolution remains locked for RustCore and Tauri;
- hosted Windows runs the locked RustCore suite;
- hosted Windows builds the frontend and native Tauri crate successfully.

## Frontend coverage in this baseline

- application boot/runtime-health presentation uses the consolidated backend snapshot;
- Library is wired to the Rust-owned local scan and exposes search/filter/status presentation only;
- Downloads is wired to runtime queue/progress plus cancel/retry/remove behavior and scheduler-error state;
- Settings loads/saves the existing Minecraft discovery settings, tracks unsaved changes, and avoids stale save feedback;
- Minecraft rescan/detected-root state is surfaced without moving discovery logic into Svelte;
- safe diagnostics are visible from Settings while secret/path redaction remains owned by RustCore;
- Discover has provider-neutral catalog search/filter/sort/pagination wiring and no fabricated real provider;
- `runtimeApi.ts` remains the sole normal raw Tauri invoke owner and pages use `runtimeProductFacade`;
- download transport selection and credential/session material remain outside the frontend boundary.

## Not yet proven

Remote verification does **not** prove:

- installed SearchNow launch/interaction on the owner's target Windows machine;
- target-machine AppData/Minecraft discovery against the owner's installation;
- representative real local libraries at user scale;
- actual settings/download interaction through a running installed Tauri window;
- production provider authentication, Marketplace/PlayFab endpoints, or TLS/network behavior;
- live Discover result/download behavior without a real provider;
- installer/branding/clean-machine release acceptance.

## Deferred proof

`TARGET_WINDOWS_RUNTIME_SMOKE` remains required and is intentionally deferred until the owner can perform local testing. See `../next-action.md`.
