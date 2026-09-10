# Next Action

Status: `FRONTEND_PROVIDER_INDEPENDENT_READY`

## Sequencing note

The target-Windows runtime smoke remains required, but it is intentionally deferred because the owner is not currently available to run local tests. Development therefore continued with frontend work that can be implemented and verified remotely without inventing local-runtime or real-provider behavior.

## Verified checkpoint

The `develop` branch now contains the closed remote backend foundation plus a provider-independent frontend workspace built on the same runtime boundaries.

Frontend surfaces now implemented remotely:

- one application bootstrap using the safe backend/runtime snapshot;
- Library scan, summary metrics, search/filter, warning/empty/error states, and collapsed technical details;
- Downloads queue/history, actual runtime progress, adaptive polling while mounted, cancel/retry/remove actions, and scheduler-error visibility;
- Settings load/save for current Minecraft discovery preferences with dirty-state protection;
- Minecraft storage rescan and detected-root presentation;
- safe runtime health and recent diagnostics in Settings;
- Discover search/filter/sort/pagination UI through the provider-neutral catalog command when a catalog-capable provider exists;
- one `runtimeProductFacade` over the single raw Tauri `runtimeApi` bridge;
- no frontend-owned persistence, filesystem scan, download transport selection, credential handling, or second runtime client.

The current RustCore suite remains **71 passing tests**. The frontend architecture/source-size/Svelte typecheck/production-build gates and hosted Windows RustCore/Tauri compile gate pass for the current remote frontend code baseline.

## Deliberately not claimed

The following remain outside the verified frontend checkpoint:

- real provider login or Marketplace/PlayFab network behavior;
- live catalog results without a registered real provider;
- Discover download-button behavior that would require guessing package/output filename metadata not yet supplied by the provider contract;
- package import/file-picker interaction not yet backed by an approved target-runtime interaction path;
- installed application behavior on the owner's Windows machine;
- real local Minecraft libraries, AppData paths, and user-machine performance.

## Remaining paths

```text
FRONTEND_PROVIDER_INDEPENDENT_READY
├── TARGET_WINDOWS_RUNTIME_SMOKE        deferred by owner, still required for local-runtime proof
├── REAL_PROVIDER_INTEGRATION          future functional dependency for live Discover
└── VISUAL_PRODUCT_REFINEMENT          may continue remotely when a concrete visual direction is requested
```

No backend-foundation rewrite is required before any of those paths.

When real provider work begins, preserve the existing provider/session/resource and product-intent boundaries. The frontend should receive normalized catalog/product state, not provider secrets, raw endpoints, or internal transport controls.

When local testing becomes available, use `tools/windows_smoke_readiness.ps1` plus the actual Tauri application for `TARGET_WINDOWS_RUNTIME_SMOKE`; do not treat hosted compilation as installed-runtime proof.
