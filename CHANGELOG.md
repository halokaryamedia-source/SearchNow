# Changelog

All notable SearchNow repository/product changes will be recorded here.

## Unreleased

### Added

- Recovered BlueCoin 2.4 architecture and runtime documentation.
- PRD-Creator-style development routing and branch model adapted for SearchNow.
- Tauri 2 + Svelte 5 + TypeScript/Vite + Rust desktop scaffold.
- In-process `EngineData/Backend/RustCore` backend library separated from Tauri IPC.
- Typed/versioned settings with staged persistence.
- Minecraft Bedrock GDK/account-scoped storage discovery with Preview opt-in and legacy UWP fallback.
- Bounded read-only indexing for local behavior packs, resource packs, skin packs, and worlds.
- Read-only folder / `.mcpack` / `.mcaddon` package inspection with typed manifest classification.
- UUID dependency-based BP/RP relationship detection.
- ZIP archive safety checks for traversal, symlinks, duplicate paths, size bounds, and extreme compression ratios without extraction.
- Transport-agnostic persistent download manager with typed job states, bounded concurrency, cancellation/retry semantics, progress validation, restart recovery, and terminal-job cleanup.
- Download workspace planning with safe destination file names and destination-local staged atomic no-overwrite publication.
- Provider-neutral download transport registry and execution runtime.
- Deterministic `local-file` transport for end-to-end queue → transfer → progress → final publication testing.
- Native bounded worker scheduling with cooperative cancellation and queued-job pumping.
- Progress persistence checkpointing at 1 MiB plus lifecycle boundaries to reduce state-write overhead.
- Tauri-managed `DownloadExecutionRuntime` created during application bootstrap; download IPC commands delegate to RustCore runtime ownership.
- Provider-neutral `https-public` transport using pinned `ureq`/rustls with explicit timeout, redirect, HTTPS-only, response-size, and content-length safeguards.
- Manual redirect handling revalidates every destination and prevents production HTTPS downloads from downgrading to plain HTTP.
- Public HTTPS queue entries reject persisted query strings and embedded credentials.
- Credential-safe `provider-resolved` transport plus `ResourceResolver` registry for converting stable provider identity into ephemeral URL/query/header transfer material only at execution time.
- Stable `ProviderResourceRef` validation that rejects URL/query/fragment runtime material from persisted provider resource ids.
- Resolver expiry refresh and explicit retry re-resolution, so stale signed material is never persisted/reused.
- Runtime HTTP header bounds, cross-origin credential redirect rejection, and sanitized sensitive request/read errors.
- Repository guards preventing authorization headers, bearer-token/signed-URL/cookie/header fields from entering persisted download DTO/store ownership.
- Deterministic resolver fixtures proving missing-provider behavior, expiry refresh, retry re-resolution, and signed-query/Authorization use without state persistence.
- Provider-neutral catalog domain with typed query/filter/sort/page, bounded item/page contracts, and explicit public-vs-provider download identity.
- `CatalogProvider` registry/service with request-before-provider validation, provider-output normalization, duplicate/malformed-result rejection, and safe provider error mapping.
- Fake catalog-provider fixtures for search/filter/sort/pagination, missing providers, malformed provider output, secret-safe failures, and catalog-item → existing download-source mapping.
- Catalog-domain repository guards preventing runtime credential fields from becoming catalog DTO ownership.
- Shared runtime-only `ProviderSessionManager`/registry/source boundary for catalog and resource-resolver adapters.
- Opaque non-serializable/non-Debug provider session material and lease carriers with safe serializable status only.
- Provider session reuse, expiry refresh, successful concurrent refresh deduplication, and failed-refresh-wave deduplication to avoid provider refresh storms.
- Provider-session failure sanitization plus deterministic fixtures proving CatalogProvider and ResourceResolver share one session owner without secret persistence.
- Provider-neutral `IntegratedProvider` / `ProviderAdapterRuntime` composition with canonical component-key validation and safe capability metadata.
- Deterministic integrated-provider fixture proving catalog → stable provider download identity → shared session → resolver → authenticated runtime HTTP → completed atomic file publication.
- Provider adapter fixture verifies catalog and resolver reuse one session acquisition and no runtime secret enters catalog JSON, provider status JSON, or persisted download state.
- Backend unit-test, clippy, architecture, source-size, and legacy-protected-content guards.
