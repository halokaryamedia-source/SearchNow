# Next Action

## Current Status

`DOWNLOAD_MANAGER_CORE_READY`

Completed:

1. Tauri/Svelte/Rust application scaffold established;
2. backend core isolated as one in-process `EngineData/Backend/RustCore` library;
3. typed/versioned settings with staged persistence implemented;
4. current Minecraft GDK/account storage discovery plus Preview opt-in and legacy UWP fallback implemented;
5. bounded read-only local library indexing implemented;
6. read-only folder / `.mcpack` / `.mcaddon` package inspection implemented;
7. typed package classification, UUID validation, BP↔RP dependency mapping, and archive safety implemented;
8. transport-agnostic `DownloadJob` lifecycle/state machine implemented;
9. default download concurrency is bounded to 3 active jobs with hard validation caps;
10. queued cancellation is immediate, active cancellation is cooperative, and finalizing jobs cannot be cancelled unsafely;
11. retry semantics cover retryable failure, cancelled jobs, and interrupted recovery;
12. download state is schema-versioned and staged to disk with rollback behavior;
13. active jobs recovered after restart become explicit `Interrupted` state rather than silently resuming;
14. persisted jobs contain transport/resource identity only — no auth tokens, headers, or signed URLs;
15. workspace planning rejects destination traversal and final publication is staged in the destination directory before atomic no-overwrite commit;
16. Tauri download queue mutations clone → persist → commit so memory and persisted state stay aligned;
17. repository CI passes with **25 RustCore tests** and strict clippy.

## Active Boundary

Keep work on `develop`. `Local` and `main` remain untouched until explicit promotion.

The manager owns lifecycle, not transport. Catalog/auth, real remote transfer, protected-content processing, package mutation/export, and frontend download-page wiring remain outside the completed slice.

Hosted CI does not replace real Windows/filesystem/network testing.

## Next Step

Continue backend-first with **Transport Execution Core**.

Build the execution boundary before any provider-specific API:

- define a transport adapter contract that consumes claimed jobs without owning queue state;
- add a scheduler/executor that respects `max_active` and cooperative `CancelRequested` semantics;
- integrate workspace creation, transfer payload ownership, progress reporting, finalization, completion/failure, and persisted state updates;
- implement an offline/mock or local-file transport first so the full lifecycle is testable without network;
- test executor recovery/failure/cancellation paths with deterministic fixtures;
- keep auth, catalog lookup, Marketplace endpoints, and frontend wiring out of the same slice.

Do not implement DRM/key-sharing/protected-content bypass paths.
