# Next Action

## Current Status

`FOUNDATION_CLOSURE_IN_PROGRESS`

Completed before this closure slice:

1. Tauri/Svelte/Rust desktop scaffold and PRD-Creator-style development workflow;
2. local settings, Minecraft discovery, bounded library indexing, and read-only package inspection in RustCore;
3. persistent bounded download lifecycle with local fixture, public HTTPS, and provider-resolved execution boundaries;
4. provider-neutral catalog, shared provider-session manager, resource resolver, and integrated-provider composition;
5. one `SearchNowBackendRuntime` owning settings/platform/provider/download/diagnostics composition;
6. thin Tauri commands delegating through that one runtime;
7. bounded secret-safe diagnostics and hosted Linux/Windows verification structure.

## Foundation closure being applied

- replace the Windows build-only icon workaround with the standard committed Tauri icon path;
- consolidate SettingsStore and DownloadStore replacement/recovery mechanics into shared `AtomicFileStore`;
- preserve backup recovery when an interrupted replacement leaves the primary state file missing;
- update repository verification so workaround-specific implementation details are not frozen as architecture;
- synchronize README, validation, roadmap/continuity documentation with the actual implemented backend.

## Active Boundary

Keep work on `develop`. `Local` and `main` remain untouched until explicit promotion.

No real provider login, PlayFab/Marketplace endpoint, durable provider credential storage, protected-content bypass, package mutation/export, or production Discover/provider frontend wiring belongs in this closure slice.

## Required proof before moving to the next feature

1. repository contract PASS;
2. RustCore format/tests/clippy PASS;
3. frontend architecture/type/build PASS;
4. Windows RustCore PASS;
5. Windows Tauri `cargo check` PASS using the normal committed icon path;
6. no regression to duplicate backend ownership or secret-bearing public/persisted DTOs.

If any closure gate fails, fix the first wrong owner/root cause before adding feature work.

## Next feature boundary after closure

Once the closure commit is green, continue backend-first with a **real-provider preparation boundary**, not direct provider endpoint implementation. That next slice should first define canonical provider identity/resource types, provider retry/timeout semantics, production credential-storage requirements, and product-intent APIs that keep transport details out of the frontend.
