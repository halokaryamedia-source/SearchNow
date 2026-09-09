# Next Action

## Current Status

`RESOURCE_RESOLVER_BOUNDARY_READY`

Completed:

1. Tauri/Svelte/Rust desktop scaffold and PRD-Creator-style development workflow are established;
2. backend logic lives in one in-process `EngineData/Backend/RustCore` library, with Tauri kept as an IPC/bootstrap adapter;
3. typed settings, Minecraft storage discovery, bounded local library indexing, and read-only package inspection are implemented;
4. package classification, UUID dependency mapping, and bounded archive safety are implemented;
5. persistent `DownloadJob` lifecycle, bounded concurrency, cancellation/retry, restart recovery, progress checkpoints, safe workspace ownership, and atomic no-overwrite publication are implemented;
6. provider-neutral `DownloadTransport` and `DownloadExecutionRuntime` execute queue jobs on bounded native worker threads;
7. deterministic `local-file` transport proves the full queue → transfer → publication path;
8. provider-neutral `https-public` uses pinned blocking `ureq`/rustls with HTTPS-only production policy, manual redirect validation, socket timeouts, SearchNow-owned overall deadline, response-size bounds, and Content-Length validation;
9. public persisted URLs reject query strings, embedded credentials, and fragments;
10. `provider-resolved` now provides a separate runtime-only path for authenticated/signed resources without weakening `https-public` persistence rules;
11. persisted provider references contain only stable `provider:opaque-resource-id` identity; URL/query/header runtime material is rejected from that reference;
12. `ResourceResolver` + `ResourceResolverRegistry` resolve stable identity into ephemeral in-memory `ResolvedResource` immediately before transport execution;
13. resolved URLs may contain temporary signed query material and bounded runtime headers such as `Authorization`, but resolved objects are not serialized into download state;
14. expired resolved material is refreshed before transfer, and an explicit job retry resolves again instead of reusing stale transfer material;
15. sensitive runtime HTTP request errors and stream-read errors are sanitized before they can become persisted `lastError` text;
16. authenticated runtime requests cannot forward credential headers across origins through redirects;
17. Tauri bootstrap registers `local-file`, `https-public`, and `provider-resolved` into one managed `DownloadExecutionRuntime`; the provider resolver registry is intentionally empty until a real provider adapter is added;
18. repository/architecture guards reject runtime credential fields from persisted download DTO/store ownership;
19. repository CI passes with **42 RustCore tests**, strict clippy, Tauri formatting, architecture/source-size validation, Svelte typecheck, and frontend build.

## Active Boundary

Keep work on `develop`. `Local` and `main` remain untouched until explicit promotion.

The resolver boundary is generic only. SearchNow still has no real provider login, PlayFab/Marketplace-specific catalog implementation, protected-content bypass, package mutation/export, or frontend download-page wiring.

Hosted loopback fixtures do not replace real Windows/network/TLS/provider evidence.

## Next Step

Continue backend-first with **Catalog Domain / Provider Query Boundary**.

Build the provider-neutral discovery/catalog model before any provider-specific endpoint work:

- define typed `CatalogQuery`, filter/sort/page request, `CatalogItem`, `CatalogPage`, and stable provider identity contracts;
- define a `CatalogProvider` interface and registry without owning UI or download lifecycle;
- make catalog items that are legitimately downloadable expose only stable provider/resource identity compatible with `ProviderResourceRef`;
- enforce bounded page size, pagination/cursor length, text/filter lengths, result count, and metadata sizes;
- distinguish ordinary public resource identity from provider-resolved identity explicitly;
- normalize provider failures into stable, non-secret catalog errors;
- implement deterministic fake catalog-provider fixtures first for search, pagination, filtering, missing provider, malformed provider data, and catalog-item → download-source mapping;
- keep provider credentials/session material out of catalog DTOs, persisted download state, and logs;
- keep PlayFab login, Marketplace endpoints, provider-specific authentication, and frontend wiring out of this slice.

Do not implement DRM/key-sharing/protected-content bypass paths.
