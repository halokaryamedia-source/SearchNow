# 09 — Provider Adapter Architecture

Status: **current integrated-provider composition contract**

## Goal

SearchNow composes each remote provider through one provider adapter boundary. A provider may contribute session, catalog, and resolved-download capabilities, but those components must share one canonical provider identity and one session owner.

```text
IntegratedProvider
│
├── ProviderSessionSource
├── CatalogProvider
└── ResourceResolver
        │
        ▼
ProviderAdapterRuntime::compose
        │
        ├── one ProviderSessionManager
        ├── one CatalogService
        └── one ResourceResolverRegistry
```

This boundary is provider-neutral. It does not implement PlayFab, Marketplace, or any real provider endpoint.

## Source ownership

```text
EngineData/Backend/RustCore/src/provider_adapter/
├── model.rs    safe serializable provider capability/status metadata
├── runtime.rs  composition, identity validation and registry wiring
└── tests.rs    deterministic composition and end-to-end provider fixtures
```

`provider_adapter/runtime.rs` owns composition only. It does not own provider login protocol, catalog retry policy, download retry policy, HTTP mechanics, filesystem persistence, or frontend state.

## Canonical provider identity

`IntegratedProvider::provider_key()` is the canonical provider identity for a composition.

Every contributed component must use exactly the same key:

```text
IntegratedProvider key
      =
ProviderSessionSource key
      =
CatalogProvider key
      =
ResourceResolver key
```

A mismatch fails composition with `provider_adapter_component_key_mismatch`.

Duplicate integrated-provider keys fail with `provider_adapter_duplicate`.

Invalid keys fail before provider components become available.

## Composition semantics

Composition occurs in two stages:

1. validate all integrated-provider identities and register session sources;
2. build one shared `ProviderSessionManager`, then construct catalog providers and resource resolvers using that same manager.

Only after every component validates does `ProviderAdapterRuntime::compose` return a usable runtime. A failed composition does not expose a half-registered catalog/resolver set to callers.

Provider component construction failures are normalized into a SearchNow-owned safe error rather than forwarding provider exception/body/credential text.

## Safe capability metadata

Public provider metadata contains only capability and safe session status information:

```text
provider
session: bool
catalog: bool
resolvedDownload: bool
safe ProviderSessionStatus
```

It must not contain:

```text
token
cookie
Authorization
signed URL
runtime headers
provider secret
session payload
```

Capability metadata answers what a provider can do, not how it authenticates.

## Responsibility boundaries

```text
ProviderAdapterRuntime
→ composition and key consistency

ProviderSessionManager
→ session acquire/reuse/refresh

CatalogService / CatalogProvider
→ catalog query and result normalization

ResourceResolver
→ stable provider resource → ephemeral transfer material

HttpTransport
→ HTTP mechanics

DownloadExecutionRuntime
→ queue/retry/progress/workspace/finalization
```

Do not move retry, transport, persistence, or session state into the adapter composition layer.

## End-to-end contract

The deterministic integrated-provider fixture proves the complete generic backend path:

```text
Catalog query
    ↓
CatalogItem
    ↓
CatalogDownloadRef::ProviderResolved
    ↓
stable DownloadSourceRef
    ↓
ResourceResolver
    ↓
shared ProviderSessionManager
    ↓
ephemeral Authorization header
    ↓
HTTP fixture
    ↓
DownloadExecutionRuntime
    ↓
atomic final file
```

The same valid session is used by both the catalog provider and resource resolver. The fixture asserts that the session source is acquired exactly once across the catalog + download flow.

It also verifies that the runtime secret is absent from:

- serialized catalog output;
- public provider status/capability output;
- persisted download state.

## Integration rule for a future real provider

A real provider should implement one `IntegratedProvider` owner and create its three components through that composition.

```text
providers/<provider>/
├── session
├── catalog
├── resolver
└── adapter/composition
```

Provider-specific code belongs in the provider implementation area in RustCore. It must not be placed in:

- Tauri command modules;
- `download/http.rs`;
- persisted download/catalog DTOs;
- frontend pages.

Tauri may expose safe provider operations/status through thin commands later, but it must not own provider protocols.

## Security boundary

Do not:

- hardcode legacy BlueCoin title/provider secrets;
- persist runtime provider credential/session material;
- expose credential values through capability/status metadata;
- create a second provider-specific download manager;
- let CatalogProvider and ResourceResolver acquire separate independent sessions for the same provider;
- implement provider login inside Tauri commands or public HTTP transport;
- use provider integration to reproduce entitlement-key sharing, DRM bypass, or protected-content decryption.

## Verification boundary

REMOTE_GITHUB proves with deterministic fixtures:

- mismatched provider component keys fail closed;
- one integrated provider composes session + catalog + resolver successfully;
- catalog output maps into the existing provider-resolved download boundary;
- catalog and resolver reuse one session acquisition;
- ephemeral Authorization material reaches the HTTP request but not persisted/public DTOs;
- the resolved HTTP payload reaches `DownloadExecutionRuntime` and is atomically published as the final file;
- all prior local/package/download/catalog/session regressions remain passing.

Hosted CI does **not** prove a real provider endpoint, real authentication/session semantics, production TLS/CDN behavior, provider terms/permissions, or Windows runtime integration. Those require PROVIDER / NETWORK / TARGET_WINDOWS evidence.
