# Next Action

Status: `FRONTEND_VISUAL_REFINEMENT_READY`

## Sequencing note

The target-Windows runtime smoke remains required, but it is intentionally deferred because the owner is not currently available to run local tests. Remote frontend work is now limited to behavior that does not invent installed-runtime or real-provider capabilities.

## Current checkpoint

The `develop` branch contains the closed remote backend foundation plus a provider-independent frontend product shell with its remote visual/product refinement pass completed.

Current remote frontend coverage:

- one application bootstrap using the safe backend/runtime snapshot;
- persistent mounted Library, Discover, Downloads, and Settings pages with active-page execution gating;
- session-only last-route restoration;
- centralized route metadata shared by Sidebar and the route-aware topbar;
- shared `PageState`, `Notice`, `MetricCard`, `ResultsBar`, `ContentTypeMark`, `ContentDetails`, `TechnicalDetails`, and `StatePill` primitives;
- responsive narrow-window behavior, keyboard focus visibility, reduced-motion handling, and semantic download progress;
- Library scan, metrics, search/filter/sort/reset, readable details, content-type marks, and separate technical metadata;
- Discover provider-neutral search/filter/sort/pagination, duplicate-result suppression, retry/reset flows, readable catalog details, and no fabricated provider/download behavior;
- Downloads queue/history, adaptive active-page polling, search/filter/reset, cancel/retry/remove actions, recovery refresh, scheduler-error visibility, semantic progress, and technical metadata;
- Settings load/save, dirty/revert protection, discovery rescan, friendly Minecraft storage labels, detected-root presentation, and safe diagnostics;
- typed frontend `QueueCatalogDownloadRequest` bridge/facade support while raw transport selection remains inaccessible to product UI;
- final visual polish for spacing/density, card balance, metric readability, hover/focus feedback, form affordance, mobile spacing, and distinct loading-state presentation;
- one `runtimeProductFacade` over the single raw Tauri `runtimeApi` bridge;
- no frontend-owned persistence, filesystem scan, credential handling, provider endpoint ownership, or second runtime client.

## Verification state

Latest visual-refinement code endpoint:

- commit: `bd4f8426a77638fc3da54103876560390932fd9d`;
- GitHub Actions: `Repository Verify` run **#241**;
- Linux/static/frontend verification: **PASS**;
- hosted Windows gate for this exact SHA: **queued at this checkpoint**.

The Linux gate proves repository contracts, 71 RustCore tests, Tauri formatting, locked dependency installation, frontend architecture/source-size validation, Svelte/TypeScript checking, and production frontend build for the visual-final code.

The immediately preceding information-hierarchy documentation baseline has already passed the complete hosted Windows RustCore/frontend/Tauri compile gate. That remains compile evidence only, not installed-runtime proof.

## Deliberately not claimed

The following remain outside this checkpoint:

- installed application behavior on the owner's target Windows machine;
- target-machine AppData/Minecraft discovery and representative real local libraries;
- real provider login or Marketplace/PlayFab network behavior;
- live catalog results without a registered real provider;
- Discover download-button behavior requiring package/output filename metadata that the current catalog contract does not provide;
- package import/file-picker interaction without an approved target-runtime interaction path;
- installer/branding/clean-machine release acceptance.

## Remaining paths

```text
FRONTEND_VISUAL_REFINEMENT_READY
├── TARGET_WINDOWS_RUNTIME_SMOKE        deferred by owner, still required for local-runtime proof
└── REAL_PROVIDER_INTEGRATION          next functional dependency for live Discover behavior
```

No further frontend logic should be invented solely to avoid those dependencies. Preserve the current provider/session/resource and product-intent boundaries when real provider work begins.
