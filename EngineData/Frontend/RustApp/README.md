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
RustCore (linked in-process)
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

The frontend owns presentation and transient application state only. Persistent/runtime truth belongs to `EngineData/Backend/RustCore` through one managed `SearchNowBackendRuntime`.

## Runtime boundary

```text
Svelte page/component
→ product facade
→ runtimeApi.ts
→ Tauri command
→ src-tauri/src/commands/
→ SearchNowBackendRuntime
→ EngineData/Backend/RustCore
```

`runtimeApi.ts` is the only normal frontend file allowed to import Tauri `invoke` directly. Pages/components must not select raw download transports or construct backend runtime state.

## Current executable boundary

The UI currently proves desktop/runtime availability through the product facade. The backend already owns Minecraft discovery, local library/package inspection, provider-neutral catalog/session/resolver composition, persistent downloads, recovery, and diagnostics. Those capabilities are intentionally wired into product UI only as their feature slices are approved.

The Tauri download command surface currently exposes snapshot/cancel/retry/remove only. A future enqueue command must represent a product action; it must not accept a raw transport-selecting `DownloadRequest` from the UI.

## Deterministic development

The committed `package-lock.json` is dependency truth for the frontend package.

```bash
npm ci
npm run validate:quick
npm run dev:app
```

For Windows compile readiness, build the frontend before Tauri compilation or use the repository-level `tools/windows_smoke_readiness.ps1 -CompileChecks` entrypoint.
