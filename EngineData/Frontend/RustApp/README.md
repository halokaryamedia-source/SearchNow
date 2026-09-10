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

## Connected product surfaces

The remote frontend implementation now uses the existing Rust runtime for:

- application/runtime health bootstrap and refresh;
- Library scan, summary, search/filter, warnings, and technical details;
- Downloads queue/progress plus cancel, retry, remove, scheduler-error visibility, and adaptive refresh while the page is mounted;
- Settings load/save for the current Minecraft discovery preferences;
- Minecraft storage rescan and detected-root presentation;
- safe runtime diagnostics and recent event presentation;
- provider-neutral Discover search/filter/sort/pagination when a catalog-capable provider is registered.

The bridge has no second API client and pages do not own persistent/runtime truth.

## Deliberately deferred

The frontend does not fabricate behavior that the backend/provider cannot truthfully support yet:

- no real provider login or Marketplace/PlayFab network adapter is present;
- Discover does not guess destination filenames/package types for downloads before provider metadata defines a safe output contract;
- package import/file-picker actions are not claimed as working until the target runtime interaction is implemented and tested;
- installed Windows behavior is not considered verified by hosted CI.

## Development

Committed `package-lock.json` is the dependency baseline:

```bash
npm ci
npm run validate:quick
npm run dev:app
```

`validate:quick` checks the architecture contract, source-size budget, Svelte/TypeScript types, and production frontend build.

Do not replace `npm ci` with unconstrained dependency resolution during normal verification. Dependency graph changes should be explicit and reviewed together with the updated lockfile.

Hosted CI also verifies the native Tauri crate on Windows with its committed Cargo lock. Installed-app behavior still requires target-Windows runtime smoke evidence.
