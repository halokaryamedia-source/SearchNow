# 05 — Application Architecture

Status: **approved current architecture**

## Architecture goal

SearchNow should remain fast to start, light on user hardware, easy to reason about, and difficult to accidentally grow into the legacy BlueCoin god-object model.

The implementation therefore uses the smallest runtime topology that serves the current product:

```text
one Tauri desktop application
├─ Svelte frontend
└─ Rust backend/runtime
```

No Python/local HTTP/service process is part of the current architecture.

## Technology stack

```text
Tauri 2
Svelte 5
Vite
TypeScript
Tailwind CSS 4
CSS custom-property design tokens
Lucide Svelte
Rust 2021
```

Use UI dependencies selectively. Do not add a router/global-state/UI framework merely because one exists.

## Source topology

```text
EngineData/Frontend/RustApp/
├── src/
│   ├── main.ts
│   ├── App.svelte
│   ├── pages/
│   │   ├── Library.svelte
│   │   ├── Discover.svelte
│   │   ├── Downloads.svelte
│   │   └── Settings.svelte
│   ├── components/
│   ├── app/
│   │   ├── bridge/
│   │   │   ├── runtimeProductFacade.ts
│   │   │   └── runtimeApi.ts
│   │   └── shared/
│   └── styles/
└── src-tauri/
    └── src/
        ├── main.rs
        ├── app_bootstrap.rs
        ├── commands/
        │   ├── registry.rs
        │   └── ...
        └── engine/
            ├── runtime.rs
            └── ...
```

## Frontend contract

Frontend state is presentation/application state only:

```text
allowed frontend truth
→ current page
→ search/filter input
→ open/closed dialogs
→ temporary selection/busy state
```

Runtime/persistent truth belongs to Rust:

```text
Minecraft installation/location
local library/index
catalog session/results source
settings persistence
download queue/jobs
package validation
filesystem writes
cache/diagnostics
```

The UI may hold snapshots for rendering but does not become the authority for these states.

## Bridge contract

Normal call path:

```text
page/component
→ product-facing facade
→ runtimeApi
→ Tauri invoke
```

Rules:

- `runtimeApi.ts` is the only normal frontend owner of raw Tauri `invoke` calls;
- pages should consume product-facing props/actions rather than transport DTOs;
- the product facade converts technical failures/status into stable product-readable state;
- do not create parallel command clients for the same runtime capability.

## Rust contract

```text
main.rs
→ process entry only

app_bootstrap.rs
→ desktop/window/bootstrap behavior

commands/
→ thin Tauri IPC wrappers + command registration

engine/
→ reusable runtime/domain logic and state ownership
```

A command should validate/translate its Tauri boundary and delegate. Filesystem/network/domain processing belongs in the engine or a bounded engine-owned adapter.

## Future engine growth

Create modules only as real responsibilities arrive:

```text
engine/minecraft/
engine/library/
engine/catalog/
engine/downloads/
engine/package/
engine/storage/
engine/diagnostics/
```

Do not pre-create a deep clean-architecture hierarchy with empty interfaces. SearchNow favors clear ownership over abstraction count.

## Runtime topology rule

Default:

```text
Tauri process only
```

A second process/runtime requires all of:

1. concrete product capability that materially benefits from/needs it;
2. evidence the capability is not cleanly served in Rust;
3. explicit lifecycle/IPC/resource ownership;
4. packaging and failure behavior defined;
5. durable architecture decision recorded.

## Source size / god-object prevention

Initial budgets:

```text
.rs      20 KB
.ts      16 KB
.svelte  18 KB
```

Budgets are architecture tripwires, not quality scores. When exceeded, split current responsibility; do not raise the limit by default.

## Security

- keep webview CSP restrictive;
- perform privileged filesystem/network work in Rust;
- do not expose generic filesystem/network commands to the frontend;
- do not log credentials, entitlement secrets, protected-content keys, or tokens;
- network behavior must map to an explicit feature/user intent.

## Proof boundaries

```text
repository/static
→ architecture validator
→ source-size validator
→ Svelte typecheck/build
→ Rust format/static source checks

local Windows
→ Tauri compile/run
→ webview/native bridge behavior

feature runtime
→ actual Minecraft filesystem/catalog/download/package behavior
```

Claim only the strongest boundary actually proven.
