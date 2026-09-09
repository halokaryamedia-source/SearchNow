# Next Action

## Current Status

`PROVIDER_ADAPTER_COMPOSITION_READY`

Completed:

1. Tauri/Svelte/Rust desktop scaffold and PRD-Creator-style development workflow are established;
2. local settings, Minecraft discovery, bounded local library indexing, and read-only package inspection are implemented in RustCore;
3. persistent download lifecycle, bounded execution, safe workspace/finalization, `local-file`, `https-public`, and `provider-resolved` transport boundaries are implemented;
4. provider-neutral catalog query/filter/sort/page/item contracts and bounded provider registry/service are implemented;
5. runtime-only provider session management provides shared acquire/reuse/expiry-refresh with successful and failed refresh-wave deduplication;
6. `IntegratedProvider` + `ProviderAdapterRuntime` now compose a provider's session source, catalog provider and resource resolver around one canonical provider key and one shared `ProviderSessionManager`;
7. provider component key mismatches and duplicates fail closed before a usable composed runtime is returned;
8. public provider capability/status metadata contains only safe availability/session metadata and no credential material;
9. deterministic integrated-provider fixture proves catalog query → `CatalogItem` → stable provider-resolved `DownloadSourceRef` → resource resolution → authenticated runtime HTTP → completed final file;
10. catalog and resolver operations in that flow reuse the same valid provider session, with exactly one session acquisition;
11. integrated fixture confirms runtime secret material is absent from catalog output, provider public status and persisted download state;
12. repository CI passes with **56 RustCore tests**, strict clippy, Tauri formatting, architecture/source-size validation, Svelte typecheck, and frontend build.

## Active Boundary

Keep work on `develop`. `Local` and `main` remain untouched until explicit promotion.

No real provider login, PlayFab/Marketplace endpoint, hardcoded title secret, real remote catalog adapter, protected-content bypass, package mutation/export, or frontend provider/Discover wiring is implemented in this completed slice.

Hosted loopback fixtures do not replace real provider/network/Windows evidence.

## Next Step

Continue backend-first with **Application Backend Runtime Composition / Tauri State Consolidation**.

Create one application-level runtime owner before adding a real provider:

- define one `SearchNowBackendRuntime` (or equivalent) in RustCore that composes local platform/settings context, `ProviderAdapterRuntime`, catalog/session/resolver access, and `DownloadExecutionRuntime`;
- make Tauri manage one backend application state object instead of accumulating separate runtime/state owners as features grow;
- keep blocking filesystem/network work delegated to existing worker/thread boundaries rather than running it on the UI thread;
- expose one safe runtime snapshot/status contract for Tauri that can report local discovery, provider capabilities/session status, and download summary without credential material;
- ensure provider resolvers from the composed provider runtime are the same registry used by the application's provider-resolved download transport;
- preserve settings/download persistence ownership in their existing stores rather than introducing a second application state database;
- add deterministic startup/composition tests proving one backend runtime wires local core + provider runtime + download runtime consistently;
- test startup failure behavior so partial provider/download initialization does not leave a misleading healthy runtime;
- add architecture guards so Tauri commands depend on the consolidated backend runtime rather than constructing provider/catalog/download engines themselves;
- keep real PlayFab login, Marketplace endpoints, hardcoded provider secrets, provider credentials, and frontend feature wiring out of this slice.

Do not implement DRM/key-sharing/protected-content bypass paths.
