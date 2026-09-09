# Next Action

## Current Status

`TRANSPORT_EXECUTION_CORE_READY`

Completed:

1. Tauri/Svelte/Rust application scaffold established;
2. backend core isolated as one in-process `EngineData/Backend/RustCore` library;
3. typed/versioned settings with staged persistence implemented;
4. current Minecraft GDK/account storage discovery plus Preview opt-in and legacy UWP fallback implemented;
5. bounded read-only local library indexing implemented;
6. read-only folder / `.mcpack` / `.mcaddon` package inspection implemented;
7. typed package classification, UUID validation, BP↔RP dependency mapping, and archive safety implemented;
8. transport-agnostic `DownloadJob` lifecycle/state machine, bounded concurrency, retry/cancellation, persistence/recovery, and atomic finalization implemented;
9. provider-neutral `DownloadTransport` contract and registry implemented;
10. `DownloadExecutionRuntime` now claims available jobs, runs bounded worker threads, writes app-owned payload workspaces, reports progress, finalizes, persists lifecycle transitions, and pumps queued work;
11. transfer buffer is fixed at 256 KiB and progress persistence is checkpointed at 1 MiB plus lifecycle boundaries rather than rewriting state every chunk;
12. cooperative `CancelRequested` handling is integrated into the worker loop and cancellation wins over concurrent transfer errors once recorded;
13. `local-file` transport provides deterministic end-to-end execution without network;
14. download workspaces/payloads reject unsafe symlink/non-file shapes and are cleaned after terminal execution attempts;
15. Tauri startup creates and `manage`s exactly one `DownloadExecutionRuntime`; download IPC commands now delegate directly to this RustCore owner;
16. architecture validation requires executor/transport/bootstrap ownership;
17. repository CI passes with **29 RustCore tests**, strict clippy, Tauri format, architecture/source-size validation, Svelte typecheck, and frontend build.

## Active Boundary

Keep work on `develop`. `Local` and `main` remain untouched until explicit promotion.

The completed executor is still provider-neutral. There is no real HTTP transport, provider authentication, catalog API, protected-content processing, package mutation/export, or frontend download-page wiring in this slice.

Hosted CI does not replace real Windows/filesystem/network testing.

## Next Step

Continue backend-first with **HTTP Transport Foundation (provider-neutral)**.

Build only generic network transport behavior first:

- add a Rust HTTP client boundary without coupling it to Marketplace/PlayFab/catalog code;
- support ordinary public HTTPS resources through a dedicated transport key;
- enforce bounded connect/read/overall timeout behavior so cooperative cancellation cannot block indefinitely on a stalled socket;
- apply conservative redirect limits and reject unsupported/non-HTTPS schemes by default;
- validate declared/observed content length against download progress and configurable safety limits;
- keep auth headers/tokens/signed URLs out of persisted `DownloadJob` state;
- define the later resolver boundary for authenticated/provider resources instead of embedding credentials in `resourceId`;
- test success, redirects, timeout/failure, oversized response, length mismatch, and cancellation using deterministic local HTTP fixtures where possible;
- keep catalog search, provider login, Marketplace-specific endpoints, and frontend wiring out of the same slice.

Do not implement DRM/key-sharing/protected-content bypass paths.
