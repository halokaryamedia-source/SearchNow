# Next Action

Status: `FRONTEND_INFORMATION_HIERARCHY_READY`

## Sequencing note

The target-Windows runtime smoke remains required, but it is intentionally deferred because the owner is not currently available to run local tests. Frontend development may continue only where behavior can be implemented and verified remotely without inventing installed-runtime or real-provider behavior.

## Current checkpoint

The `develop` branch contains the closed remote backend foundation plus a provider-independent frontend product shell using the same runtime boundaries.

Current remote frontend coverage:

- one application bootstrap using the safe backend/runtime snapshot;
- persistent mounted page sessions for Library, Discover, Downloads, and Settings with active-page execution gating;
- session-only last-route restoration without introducing another persistent frontend state owner;
- centralized route metadata shared by Sidebar and a route-aware topbar;
- shared `PageState`, `Notice`, `MetricCard`, `ResultsBar`, `ContentTypeMark`, `ContentDetails`, `TechnicalDetails`, and `StatePill` primitives;
- responsive narrow-window behavior, focus-visible treatment, reduced-motion handling, and current-route semantics;
- Library scan, metrics, search/filter/sort/reset, content-type marks, readable product details, and collapsed technical metadata;
- Discover provider-neutral search/filter/sort/pagination, duplicate-result suppression, retry/reset flows, readable catalog details, and no fabricated provider/download behavior;
- Downloads queue/history, semantic progress bars, adaptive active-page polling, search/filter/reset, cancel/retry/remove actions, recovery refresh, scheduler-error visibility, and technical metadata;
- Settings load/save, dirty/revert protection, discovery rescan, friendly Minecraft channel/storage labels, detected-root presentation, and safe diagnostics;
- typed frontend `QueueCatalogDownloadRequest` bridge/facade support while raw transport selection remains inaccessible to product UI;
- one `runtimeProductFacade` over the single raw Tauri `runtimeApi` bridge;
- no frontend-owned persistence, filesystem scan, credential handling, provider endpoint ownership, or second runtime client.

## Verification state

The latest information-hierarchy code endpoint is `fbbf1c247869a4ac95038d3e1c135f883a34dbac`.

Repository Verify run **#234** has passed the Linux/static/frontend gate for that SHA, including repository contracts, 71 RustCore tests, Tauri formatting, locked dependency installation, frontend architecture/source-size checks, Svelte/TypeScript checking, and production frontend build.

The native hosted-Windows gate for that exact SHA is still pending/queued at this checkpoint. Earlier frontend baselines have already demonstrated Windows RustCore/frontend/Tauri compilation, but no claim is made here that `fbbf1c2` has completed that native Windows gate yet.

## Deliberately not claimed

The following remain outside this checkpoint:

- installed application behavior on the owner's target Windows machine;
- target-machine AppData/Minecraft discovery and representative real local libraries;
- real provider login or Marketplace/PlayFab network behavior;
- live catalog results without a registered real provider;
- Discover download-button behavior requiring package/output filename metadata that the current catalog contract does not provide;
- package import/file-picker interaction without an approved target-runtime interaction path;
- installer/branding/clean-machine release acceptance.

The typed download intent remains prepared in the frontend bridge without exposing an unsupported product button or guessing filenames.

## Remaining paths

```text
FRONTEND_INFORMATION_HIERARCHY_READY
├── TARGET_WINDOWS_RUNTIME_SMOKE        deferred by owner, still required for local-runtime proof
├── REAL_PROVIDER_INTEGRATION          functional dependency for live Discover behavior
└── VISUAL_PRODUCT_REFINEMENT          can continue remotely without changing runtime ownership
```

No backend-foundation rewrite or second frontend state/runtime system is required.

When real provider work begins, preserve the existing provider/session/resource and product-intent boundaries. The frontend should receive normalized catalog/product state, not provider secrets, raw endpoints, or internal transport controls.
