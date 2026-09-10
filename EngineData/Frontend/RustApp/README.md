# SearchNow Tauri Desktop Application

This directory is the active SearchNow desktop application package.

## Stack

```text
Tauri 2
Svelte 5
Vite
TypeScript
Tailwind CSS 4
CSS custom-property design tokens
Lucide Svelte
Rust
```

No SvelteKit, frontend router, Redux-like state library, Python worker, or heavy UI framework is required for the current product.

## Frontend ownership

```text
index.html
→ src/main.ts
→ src/App.svelte
   ├─ pages/
   ├─ components/
   ├─ app/bridge/
   ├─ app/shared/
   └─ styles/
```

The frontend owns presentation and transient application state only. Persistent/runtime truth belongs to RustCore.

## Runtime boundary

```text
Svelte page/component
→ runtimeProductFacade.ts
→ runtimeApi.ts
→ Tauri command
→ src-tauri/src/commands/
→ SearchNowBackendRuntime
→ ../../Backend/RustCore/
```

`runtimeApi.ts` is the only normal frontend file allowed to import Tauri `invoke` directly. Tauri commands remain adapters; they do not own backend sub-runtimes or business logic.

Download IPC accepts provider-neutral catalog download intent. The frontend does not select internal transport keys.

## Current backend-connected surfaces

The repository foundation exposes typed boundaries for:

```text
runtime/diagnostics
settings
Minecraft discovery
local library
package inspection
download lifecycle
provider-neutral catalog/download intent
```

Real provider login/endpoints and production Discover wiring remain future product work.

## Development

Committed `package-lock.json` is the dependency baseline:

```bash
npm ci
npm run validate:quick
npm run dev:app
```

Do not replace `npm ci` with unconstrained dependency resolution during normal verification. Dependency graph changes should be explicit and reviewed together with the updated lockfile.

Hosted CI also verifies the native Tauri crate on Windows with its committed Cargo lock. Installed-app behavior still requires target-Windows runtime smoke evidence.
