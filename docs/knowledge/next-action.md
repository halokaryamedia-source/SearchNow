# Next Action

Status: `FRONTEND_REMOTE_REFINEMENT_READY`

## Sequencing note

The target-Windows runtime smoke remains required, but it is intentionally deferred because the owner is not currently available to run local tests. Frontend development therefore continues only within behavior that can be implemented and verified remotely without inventing installed-runtime or real-provider behavior.

## Current checkpoint

The `develop` branch contains the closed remote backend foundation plus a provider-independent frontend product shell using the same runtime boundaries.

Current remote frontend coverage:

- one application bootstrap using the safe backend/runtime snapshot;
- persistent mounted page sessions for Library, Discover, Downloads, and Settings so page-local search/filter/sort state survives navigation;
- active-page execution gating so local scans, catalog queries, download polling, settings loads, and diagnostics reads do not continue unnecessarily while their page is inactive;
- shared `PageState` and `Notice` primitives instead of independent loading/error/empty-state implementations per page;
- responsive desktop/narrow-window layouts, focus-visible treatment, navigation current-page semantics, and reduced-motion handling;
- Library scan, metrics, search, content filter, sorting, warning states, and technical details;
- Downloads queue/history, progress, adaptive active-page polling, search/filter, cancel/retry/remove actions, and scheduler-error visibility;
- Settings load/save, dirty-state protection, discovery rescan, detected-root presentation, save feedback, and safe diagnostics;
- Discover search/filter/sort/pagination through the provider-neutral catalog boundary;
- typed frontend `QueueCatalogDownloadRequest` bridge/facade support while raw transport selection remains inaccessible to product UI;
- one `runtimeProductFacade` over the single raw Tauri `runtimeApi` bridge;
- no frontend-owned persistence, filesystem scan, credential handling, provider endpoint ownership, or second runtime client.

## Deliberately not claimed

The following remain outside this checkpoint:

- installed application behavior on the owner's target Windows machine;
- target-machine AppData/Minecraft discovery and representative real local libraries;
- real provider login or Marketplace/PlayFab network behavior;
- live catalog results without a registered real provider;
- Discover download-button behavior requiring package/output filename metadata that the current catalog contract does not provide;
- package import/file-picker interaction without an approved target-runtime interaction path;
- installer/branding/clean-machine release acceptance.

The typed download intent is intentionally prepared in the frontend bridge without exposing an unsupported product button or guessing filenames.

## Remaining paths

```text
FRONTEND_REMOTE_REFINEMENT_READY
├── TARGET_WINDOWS_RUNTIME_SMOKE        deferred by owner, still required for local-runtime proof
├── REAL_PROVIDER_INTEGRATION          functional dependency for live Discover behavior
└── VISUAL_PRODUCT_REFINEMENT          may continue remotely without changing runtime ownership
```

No backend-foundation rewrite or second frontend state/runtime system is required.

When real provider work begins, preserve the existing provider/session/resource and product-intent boundaries. The frontend should receive normalized catalog/product state, not provider secrets, raw endpoints, or internal transport controls.

When local testing becomes available, use `tools/windows_smoke_readiness.ps1` plus the actual Tauri application for `TARGET_WINDOWS_RUNTIME_SMOKE`; hosted compilation must not be treated as installed-runtime proof.
