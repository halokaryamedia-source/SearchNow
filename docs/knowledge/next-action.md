# Next Action

## Current Status

`CATALOG_DOMAIN_READY`

Completed:

1. Tauri/Svelte/Rust desktop scaffold and PRD-Creator-style development workflow are established;
2. backend logic lives in one in-process `EngineData/Backend/RustCore` library, with Tauri kept as an IPC/bootstrap adapter;
3. typed settings, Minecraft storage discovery, bounded local library indexing, and read-only package inspection are implemented;
4. package classification, UUID dependency mapping, and bounded archive safety are implemented;
5. persistent download lifecycle, bounded concurrency, cancellation/retry, restart recovery, progress checkpoints, workspace safety, and atomic no-overwrite publication are implemented;
6. provider-neutral transport execution supports deterministic `local-file`, bounded `https-public`, and credential-safe `provider-resolved` runtime resolution;
7. stable provider resource identity is separated from ephemeral signed URL/query/header transfer material;
8. provider resolver expiry refresh, explicit retry re-resolution, sensitive error sanitization, and credential-safe redirect behavior are implemented;
9. provider-neutral catalog DTOs now define typed query/filter/sort/page requests, item/page outputs, content types, and explicit public-vs-provider download references;
10. `CatalogProvider` + `CatalogProviderRegistry` + `CatalogService` provide a narrow backend query boundary without owning UI/download lifecycle;
11. catalog request bounds cover provider/query/filter/page/cursor input before provider execution;
12. provider output is validated for page count, cursor, duplicate ids, title/description/tags, total item text size, and downloadable identity before reaching public `CatalogPage`;
13. catalog provider failures are normalized to stable safe errors without provider exception/secret text;
14. catalog items map into the existing `https-public` or `provider-resolved` download boundaries rather than creating a parallel download model;
15. catalog DTOs are covered by credential-field repository guards;
16. deterministic fake catalog fixtures cover filtering, sorting, pagination, invalid query, missing provider, malformed provider data, safe failure normalization, and download-source mapping;
17. repository CI passes with **48 RustCore tests**, strict clippy, Tauri formatting, architecture/source-size validation, Svelte typecheck, and frontend build.

## Active Boundary

Keep work on `develop`. `Local` and `main` remain untouched until explicit promotion.

No real catalog provider, provider login, PlayFab/Marketplace endpoint, protected-content bypass, package mutation/export, or Discover-page wiring is implemented in this completed slice.

Hosted fixtures do not replace real provider/network/Windows evidence.

## Next Step

Continue backend-first with **Provider Session / Credential Runtime Boundary**.

Build the final generic session boundary before any real provider adapter:

- define one provider-session interface that catalog providers and resource resolvers can share instead of implementing parallel login/refresh logic;
- keep session/credential material runtime-only: do not derive `Serialize`, do not persist it into app settings/catalog DTOs/download state, and do not expose secret-bearing `Debug` output;
- represent only safe provider/session status publicly (available, unavailable, expired/refreshing, failure code) without token/session values;
- define expiry + refresh semantics and ensure concurrent provider work does not trigger duplicate refresh/login attempts unnecessarily;
- normalize provider-session failures to stable safe code + retryability without exception/body/token leakage;
- implement deterministic fake session-provider fixtures for acquire, reuse, expiry refresh, refresh failure, concurrent refresh deduplication, and secret-non-persistence checks;
- keep catalog query retries/download retries owned by their existing runtimes; the session boundary only supplies current runtime session context;
- keep real PlayFab login, Marketplace endpoints, title secrets, provider credentials, and frontend authentication flows out of this slice.

Do not implement DRM/key-sharing/protected-content bypass paths.
