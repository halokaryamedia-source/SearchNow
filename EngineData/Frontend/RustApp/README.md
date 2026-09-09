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

The frontend owns presentation and transient application state only. Persistent/runtime truth belongs to Rust.

## Runtime boundary

```text
Svelte page/component
→ runtimeProductFacade.ts
→ runtimeApi.ts
→ Tauri command
→ src-tauri/src/commands/
→ src-tauri/src/engine/
```

`runtimeApi.ts` is the only normal frontend file allowed to import Tauri `invoke` directly.

## Current vertical slice

The first executable slice is intentionally small:

```text
App startup
→ loadProductRuntimeSnapshot()
→ get_runtime_status
→ Rust engine runtime status
→ UI status badge/settings diagnostics
```

This proves the architecture without prematurely implementing Minecraft/catalog/download behavior.

## Development

```bash
npm install
npm run validate:quick
npm run dev:app
```

A dependency lockfile should be generated and reviewed from the first local dependency resolution before release-grade dependency reproducibility is claimed.
