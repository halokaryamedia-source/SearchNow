# 08 — Provider Session Architecture

Status: **current provider-session contract**

## Goal

SearchNow uses one runtime-only provider-session owner for provider authentication/session context. Catalog providers and resource resolvers may share that owner; they must not implement parallel token/login/refresh state machines.

```text
ProviderSessionSource
        ↓
ProviderSessionManager
        ↓ shared runtime lease
   ┌────┴────┐
   ↓         ↓
Catalog    Resource
Provider   Resolver
```

This boundary is generic. It does not implement PlayFab, Marketplace, or any provider-specific login endpoint.

## Source ownership

```text
EngineData/Backend/RustCore/src/provider_session/
├── model.rs    public safe status/error DTOs only
├── runtime.rs  secret-bearing runtime material, registry, lease and refresh coordination
└── tests.rs    deterministic session/concurrency/secret-isolation fixtures
```

`provider_session/runtime.rs` owns provider session acquisition/reuse/refresh coordination. It does not own catalog queries, download retry policy, HTTP transport, filesystem state, or Tauri UI state.

## Runtime-only credential material

`ProviderSessionMaterial` stores provider-defined runtime state as an opaque `Any + Send + Sync` payload plus optional expiry metadata. `ProviderSessionLease` grants provider adapters typed access through explicit downcast.

Secret-bearing runtime objects intentionally do **not** implement:

```text
Serialize
Deserialize
Debug
```

They must never be written to settings, catalog DTOs, download state, logs, crash-oriented debug strings, or frontend IPC payloads.

Only safe status metadata is serializable:

```text
provider
state
expiresAtMs
failureCode
retryable
```

Public status states are:

```text
Unavailable
Available
Expired
Refreshing
Failed
```

No token/session/cookie/header value is part of public status.

## Acquisition and reuse

For one registered provider:

```text
acquire()
  ↓
valid material exists?
  ├─ yes → reuse existing lease
  └─ no
       ↓
     expired material exists?
       ├─ yes → refresh(current lease)
       └─ no  → acquire new material
```

A valid session is reused by catalog and resolver work instead of repeatedly acquiring provider credentials.

## Refresh coordination

Each provider entry owns one mutex + condition variable. Only one thread may perform acquire/refresh work for that provider at a time.

```text
first caller
  ↓
Refreshing
  ↓
provider acquire/refresh

concurrent callers
  ↓
wait on same refresh
  ↓
consume same result
```

Successful refresh wakes all waiters and they reuse the newly stored material.

A failed refresh wave is also shared. Waiters that were already blocked on that refresh receive the same sanitized failure instead of immediately launching sequential duplicate refresh attempts. A later, separate `acquire()` call may retry when the failure is marked retryable.

This prevents both success-path and failure-path refresh storms.

## Failure semantics

Provider-session failures expose only:

```text
safe code
retryable flag
generic SearchNow message
```

Malformed/provider-secret-bearing codes are replaced with:

```text
provider_session_failed
```

Provider exception bodies, response payloads, tokens, cookies, and session material must not cross this boundary.

Missing providers are explicit and non-retryable:

```text
provider_session_unavailable
```

Already-expired material returned by a source is rejected as retryable `provider_session_material_expired` rather than being treated as valid.

## Integration rule

A real provider adapter should build one shared `Arc<ProviderSessionManager>` and inject it into both its `CatalogProvider` and `ResourceResolver` implementations.

```text
Provider adapter
   │
   ├─ SessionSource ──→ ProviderSessionManager
   │                         │
   ├─ CatalogProvider ←──────┤
   │                         │
   └─ ResourceResolver ←─────┘
```

Catalog query retry remains owned by the catalog caller/provider policy. Download retry remains owned by `DownloadExecutionRuntime`. Provider session only supplies current runtime session context.

## Security boundary

Do not:

- derive `Serialize`, `Deserialize`, or `Debug` for `ProviderSessionMaterial` or `ProviderSessionLease`;
- persist provider session values into settings or queue state;
- expose tokens through public provider status;
- duplicate login/refresh state inside CatalogProvider and ResourceResolver;
- hardcode legacy BlueCoin title/provider secrets;
- use this boundary for entitlement-key sharing, DRM bypass, or protected-content decryption.

## Verification boundary

REMOTE_GITHUB can prove with deterministic fixtures:

- session acquisition occurs once and valid material is reused;
- public serialized status contains no secret payload;
- expired session refresh works;
- eight concurrent consumers share one successful refresh;
- concurrent consumers share one failed refresh wave without a refresh storm;
- refresh failure code/message is sanitized;
- missing provider status/error is explicit;
- a CatalogProvider and ResourceResolver can share the same session manager and session instance;
- static repository guards keep runtime session carriers non-serializable and non-Debug.

Hosted CI does **not** prove any real provider login, remote token refresh, network/TLS behavior, Windows credential storage, or provider-specific session semantics. Those require PROVIDER/NETWORK/TARGET_WINDOWS evidence.
