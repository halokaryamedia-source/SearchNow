# Next Action

Status: `FRONTEND_VISUAL_REFINEMENT_READY`

## Sequencing note

The remote cleanup pass is complete at the verified code baseline. The target-Windows installed/runtime smoke remains required, but it is intentionally deferred because the owner is not currently available to run local tests. Real-provider work also remains a separate functional dependency and must not be simulated in product code.

## Current checkpoint

The `develop` branch contains the closed remote backend foundation plus the provider-independent frontend product shell. The latest cleanup removed the remaining static/compile issues found by the full audit without adding a second runtime or new architecture layer.

Current remote coverage includes:

- one application bootstrap using the safe backend/runtime snapshot;
- `SearchNowBackendRuntime` as the application authority over settings, local discovery, catalog/provider composition, downloads, and diagnostics;
- crate-internal local snapshot composition rather than a competing public backend entry path;
- bounded and regression-tested persisted-state temp cleanup;
- persistent mounted Library, Discover, Downloads, and Settings pages with active-page execution gating;
- one `runtimeProductFacade` over the single raw Tauri `runtimeApi` bridge;
- native accessible Library card controls and valid dialog semantics;
- Settings discovery state synchronized from runtime snapshots without stale initial-value capture;
- desktop folder selection bound correctly to the generic Tauri runtime;
- no frontend-owned persistence, filesystem scan, credential handling, provider endpoint ownership, or second runtime client.

## Verification state

Latest verified code endpoint:

- commit: `2e5b708baaaa31090c9258563221b7b639ec1208`;
- GitHub Actions: `Repository Verify` run **#341**;
- repository/backend/frontend gate: **PASS**;
- hosted Windows RustCore/frontend/Tauri compile gate: **PASS**;
- RustCore: **81 tests passed**;
- Svelte check: **0 errors, 0 warnings**.

This is remote/static/integration evidence only. It does not replace running the installed application on the owner's Windows machine.

## Deliberately not claimed

The following remain outside this checkpoint:

- installed application behavior on the owner's target Windows machine;
- target-machine AppData/Minecraft discovery and representative real local libraries;
- real provider login or Marketplace/PlayFab network behavior;
- live catalog results without a registered real provider;
- provider-backed Discover download behavior until a real provider supplies deterministic safe output metadata;
- package import/file-picker interaction beyond the currently approved paths;
- installer/branding/clean-machine release acceptance.

## Remaining paths

```text
FRONTEND_VISUAL_REFINEMENT_READY
├── TARGET_WINDOWS_RUNTIME_SMOKE        deferred by owner; required for installed-runtime proof
└── REAL_PROVIDER_INTEGRATION          next functional dependency for live Discover behavior
```

Do not add speculative abstractions to avoid these dependencies. Continue from the smallest concrete owner when either path becomes active.
