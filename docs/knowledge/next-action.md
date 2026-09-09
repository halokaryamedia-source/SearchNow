# Next Action

## Current Status

`HTTP_TRANSPORT_FOUNDATION_READY`

Completed:

1. Tauri/Svelte/Rust application scaffold established;
2. backend core isolated as one in-process `EngineData/Backend/RustCore` library;
3. typed/versioned settings with staged persistence implemented;
4. current Minecraft GDK/account storage discovery plus Preview opt-in and legacy UWP fallback implemented;
5. bounded read-only local library indexing implemented;
6. read-only folder / `.mcpack` / `.mcaddon` package inspection implemented;
7. typed package classification, UUID validation, BP↔RP dependency mapping, and archive safety implemented;
8. transport-agnostic `DownloadJob` lifecycle/state machine, bounded concurrency, retry/cancellation, persistence/recovery, and atomic finalization implemented;
9. provider-neutral `DownloadTransport` registry and `DownloadExecutionRuntime` implemented with bounded worker threads and progress checkpoints;
10. deterministic `local-file` transport proves the complete queue → transfer → publication lifecycle without network;
11. provider-neutral `https-public` transport is implemented using pinned blocking `ureq` with rustls and a dedicated RustCore owner;
12. production public transport is HTTPS-only and rejects embedded URL credentials, fragments, and persisted query strings;
13. redirects are manual, bounded, and every target is revalidated, preventing production HTTPS→HTTP downgrade;
14. connect/read socket timeouts remain separate from a SearchNow-owned overall transfer deadline so stalled reads stay bounded without sacrificing the total deadline;
15. declared and observed response sizes are bounded; Content-Length mismatch/short bodies cannot finalize as successful downloads;
16. HTTP status retryability is explicit and conservative;
17. Tauri bootstrap registers `local-file` and `https-public` into exactly one managed execution runtime;
18. architecture validation requires HTTP transport ownership and bootstrap registration;
19. repository CI passes with **37 RustCore tests**, strict clippy, Tauri format, architecture/source-size validation, Svelte typecheck, and frontend build.

## Active Boundary

Keep work on `develop`. `Local` and `main` remain untouched until explicit promotion.

The public HTTPS transport is intentionally unauthenticated/provider-neutral. There is still no provider login, PlayFab/Marketplace-specific catalog logic, protected-content processing, package mutation/export, or frontend download-page wiring in this completed slice.

Hosted CI loopback fixtures do not replace real Windows/network/TLS testing.

## Next Step

Continue backend-first with **Runtime Resource Resolver / Provider Adapter Boundary**.

Build the generic credential-safe boundary before any provider-specific API:

- define an opaque persisted provider resource reference that contains stable non-secret identity only;
- define a resolver interface that converts that identity into ephemeral in-memory transfer material immediately before transport execution;
- allow resolved runtime material to carry temporary URL/query/header/auth context without serializing it back into `DownloadJob` or logs;
- define clear expiry/re-resolution semantics so retries do not reuse stale signed URLs or tokens;
- keep the existing `https-public` path for truly public resources and introduce a separate resolved/provider transport path rather than weakening its persistence rules;
- make cancellation and retry continue to be owned by `DownloadExecutionRuntime`, not by resolvers;
- implement deterministic fake resolver/provider fixtures first, including success, expired/refresh, resolver failure, missing provider, and secret-non-persistence assertions;
- add architecture guards preventing provider credentials from entering queue DTOs/persisted state;
- keep PlayFab login, Marketplace endpoints, catalog search, and any provider-specific credential implementation out of the same slice.

Do not implement DRM/key-sharing/protected-content bypass paths.
