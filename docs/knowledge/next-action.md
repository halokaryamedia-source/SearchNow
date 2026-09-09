# Next Action

## Current Status

`PROVIDER_SESSION_BOUNDARY_READY`

Completed:

1. Tauri/Svelte/Rust desktop scaffold and PRD-Creator-style development workflow are established;
2. local settings, Minecraft discovery, bounded local library indexing, and read-only package inspection are implemented in RustCore;
3. persistent download lifecycle, bounded execution, safe workspace/finalization, `local-file`, `https-public`, and `provider-resolved` transport boundaries are implemented;
4. stable provider resource identity is separated from ephemeral signed URL/query/header transfer material;
5. provider-neutral catalog query/filter/sort/page/item contracts and provider registry/service are implemented with bounded validation and safe errors;
6. `ProviderSessionSource` + registry + manager now provide one shared runtime session owner for catalog providers and resource resolvers;
7. secret-bearing `ProviderSessionMaterial` / `ProviderSessionLease` remain runtime-only, non-Serialize/non-Deserialize, and non-Debug;
8. public provider-session status exposes only safe state, expiry, failure code, and retryability metadata;
9. valid sessions are reused, expired sessions refresh, and concurrent consumers share one successful refresh;
10. failed concurrent refresh waves are also deduplicated so waiters consume one sanitized failure rather than triggering a refresh storm;
11. provider-session failure code/message normalization prevents provider exception/body/token leakage;
12. deterministic fixtures prove one shared session owner can serve both CatalogProvider and ResourceResolver;
13. repository CI passes with **54 RustCore tests**, strict clippy, Tauri formatting, architecture/source-size validation, Svelte typecheck, and frontend build.

## Active Boundary

Keep work on `develop`. `Local` and `main` remain untouched until explicit promotion.

No real provider login, PlayFab/Marketplace endpoint, title secret, real catalog adapter, protected-content bypass, package mutation/export, or frontend authentication/Discover wiring is implemented in this completed slice.

Hosted fixtures do not replace real provider/network/Windows evidence.

## Next Step

Continue backend-first with **Provider Adapter Composition / Integration Boundary**.

Build a generic integrated-provider composition before any provider-specific endpoint:

- define one provider adapter composition/factory that wires a provider's `ProviderSessionSource`, `CatalogProvider`, and `ResourceResolver` around the same `ProviderSessionManager`;
- define safe provider capability/availability metadata so SearchNow can know whether catalog, resolved download, and session support are registered without exposing credentials;
- make provider registration consistent/atomic enough that catalog and resolver keys cannot silently disagree;
- keep catalog retry, download retry, transport, and persistence owned by their existing runtimes;
- implement one deterministic fake integrated provider proving catalog query → CatalogItem → stable DownloadSourceRef → runtime resolve → HTTP fixture → completed download using the shared session owner;
- verify catalog and download operations reuse the same valid session rather than acquiring separate credentials;
- verify provider adapter failures remain safe and no session/request secret enters catalog DTOs, download state, logs, or public provider metadata;
- add architecture guards for provider adapter ownership and prevent provider-specific code from being placed in Tauri commands or public HTTPS transport;
- keep real PlayFab login, Marketplace endpoints, hardcoded title secrets, provider credentials, and frontend provider flows out of this slice.

Do not implement DRM/key-sharing/protected-content bypass paths.
