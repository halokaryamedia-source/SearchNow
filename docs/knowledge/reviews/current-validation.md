# Current Validation

Reviewed: **2026-09-10**  
Scope: `develop` remote backend foundation + provider-independent frontend information hierarchy  
Status: **latest Linux/static/frontend verification complete; target-Windows installed/runtime smoke pending**

## Latest frontend code proof

Current information-hierarchy code endpoint:

- commit: `fbbf1c247869a4ac95038d3e1c135f883a34dbac`;
- GitHub Actions: `Repository Verify` run **#234**;
- Linux repository/backend/frontend verification: **PASS**;
- hosted Windows verification for this exact SHA: **not yet completed at the recorded checkpoint**.

The Linux gate for this endpoint proves repository contracts, 71 RustCore tests, Tauri formatting, locked dependency installation, frontend architecture/source-size contracts, Svelte/TypeScript checking, and production frontend build.

Earlier frontend baselines have passed native hosted-Windows RustCore/frontend/Tauri compilation. Several later Windows runs were cancelled when a newer `develop` commit superseded them because the workflow intentionally uses `cancel-in-progress`. A cancelled superseded run is therefore not classified as a source failure.

## Frontend coverage in this baseline

- application boot/runtime-health presentation uses the consolidated backend snapshot;
- Sidebar and topbar share centralized route metadata;
- the last route is restored for the current browser/app session only;
- product pages remain mounted so search/filter/sort state survives navigation while runtime work is gated to the active page;
- Library exposes metrics, search/filter/sort/reset, content-type marks, readable content disclosure, and separate technical metadata;
- Discover exposes provider-neutral search/filter/sort/pagination, duplicate-page-item suppression, retry/reset behavior, readable catalog details, and no fabricated real-provider action;
- Downloads exposes queue/history, adaptive polling, search/filter/reset, semantic progressbar state, cancel/retry/remove, scheduler-error/recovery feedback, and technical metadata;
- Settings loads/saves current discovery settings, tracks and reverts unsaved changes, uses friendly Minecraft storage labels, and surfaces safe diagnostics;
- shared presentation primitives include `PageState`, `Notice`, `MetricCard`, `ResultsBar`, `ContentTypeMark`, `ContentDetails`, `TechnicalDetails`, and `StatePill`;
- `runtimeApi.ts` remains the sole normal raw Tauri invoke owner and pages use `runtimeProductFacade`;
- raw transport selection, credentials/session material, filesystem ownership, and provider endpoint behavior remain outside Svelte.

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

`TARGET_WINDOWS_RUNTIME_SMOKE` remains required and is intentionally deferred until the owner can perform local testing. Hosted compilation must not be presented as installed-runtime proof.
