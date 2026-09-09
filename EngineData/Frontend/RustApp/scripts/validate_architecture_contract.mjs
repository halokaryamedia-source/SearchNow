import { access, readdir, readFile } from "node:fs/promises";
import { extname, relative, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const appRoot = resolve(fileURLToPath(new URL("..", import.meta.url)));
const backendRoot = resolve(appRoot, "../../Backend/RustCore");
const required = [
  "src/App.svelte",
  "src/app/bridge/runtimeApi.ts",
  "src/app/bridge/runtimeProductFacade.ts",
  "src/pages/Library.svelte",
  "src/pages/Discover.svelte",
  "src/pages/Downloads.svelte",
  "src/pages/Settings.svelte",
  "src-tauri/src/main.rs",
  "src-tauri/src/app_bootstrap.rs",
  "src-tauri/src/commands/registry.rs",
  "src-tauri/src/commands/runtime.rs",
  "src-tauri/src/commands/settings.rs",
  "src-tauri/src/commands/minecraft.rs",
  "src-tauri/src/commands/library.rs",
  "src-tauri/src/commands/package.rs",
  "src-tauri/src/commands/download.rs",
];
const backendRequired = [
  "Cargo.toml",
  "src/lib.rs",
  "src/app_runtime.rs",
  "src/settings.rs",
  "src/minecraft.rs",
  "src/library.rs",
  "src/package/mod.rs",
  "src/package/model.rs",
  "src/package/manifest.rs",
  "src/package/archive.rs",
  "src/package/folder.rs",
  "src/download/mod.rs",
  "src/download/model.rs",
  "src/download/manager.rs",
  "src/download/store.rs",
  "src/download/workspace.rs",
  "src/download/transport.rs",
  "src/download/executor.rs",
  "src/download/http.rs",
  "src/download/resolver.rs",
  "src/catalog/mod.rs",
  "src/catalog/model.rs",
  "src/catalog/provider.rs",
  "src/provider_session/mod.rs",
  "src/provider_session/model.rs",
  "src/provider_session/runtime.rs",
  "src/provider_adapter/mod.rs",
  "src/provider_adapter/model.rs",
  "src/provider_adapter/runtime.rs",
];
const errors = [];

for (const path of required) {
  try { await access(resolve(appRoot, path)); } catch { errors.push(`missing required architecture path: ${path}`); }
}
for (const path of backendRequired) {
  try { await access(resolve(backendRoot, path)); } catch { errors.push(`missing backend core path: EngineData/Backend/RustCore/${path}`); }
}

async function collect(directory) {
  const entries = await readdir(directory, { withFileTypes: true });
  const files = [];
  for (const entry of entries) {
    const path = resolve(directory, entry.name);
    if (entry.isDirectory()) files.push(...await collect(path));
    else if (entry.isFile() && [".ts", ".svelte"].includes(extname(entry.name))) files.push(path);
  }
  return files;
}

for (const path of await collect(resolve(appRoot, "src"))) {
  const rel = relative(appRoot, path).replaceAll("\\", "/");
  const text = await readFile(path, "utf8");
  if (text.includes("@tauri-apps/api/core") && rel !== "src/app/bridge/runtimeApi.ts") errors.push(`${rel}: direct Tauri invoke import is reserved for runtimeApi.ts`);
  if (rel.startsWith("src/pages/") && text.includes("runtimeApi")) errors.push(`${rel}: pages must not call runtimeApi directly`);
}

const tauriManifest = await readFile(resolve(appRoot, "src-tauri/Cargo.toml"), "utf8");
if (!tauriManifest.includes('searchnow-core = { path = "../../../Backend/RustCore" }')) errors.push("Tauri runtime must link the in-process RustCore backend");

const commandPaths = [
  "runtime.rs",
  "settings.rs",
  "minecraft.rs",
  "library.rs",
  "package.rs",
  "download.rs",
];
const forbiddenCommandOwners = [
  "SettingsStore",
  "PlatformContext",
  "DownloadExecutionRuntime",
  "DownloadTransportRegistry",
  "ResourceResolverRegistry",
  "HttpTransport",
  "ProviderAdapterRuntime",
  "IntegratedProvider",
];
for (const file of commandPaths) {
  const text = await readFile(resolve(appRoot, `src-tauri/src/commands/${file}`), "utf8");
  if (!text.includes("SearchNowBackendRuntime")) errors.push(`${file}: Tauri command must delegate through SearchNowBackendRuntime`);
  for (const forbidden of forbiddenCommandOwners) {
    if (text.includes(forbidden)) errors.push(`${file}: Tauri command must not own or construct backend component ${forbidden}`);
  }
}

const bootstrap = await readFile(resolve(appRoot, "src-tauri/src/app_bootstrap.rs"), "utf8");
if (!bootstrap.includes("SearchNowBackendRuntime::new")) errors.push("Tauri bootstrap must construct the consolidated SearchNowBackendRuntime");
if (!bootstrap.includes("app.manage(runtime)")) errors.push("Tauri bootstrap must manage one consolidated backend runtime");
for (const forbidden of ["DownloadExecutionRuntime", "DownloadTransportRegistry", "ResourceResolverRegistry", "HttpTransport", "ProviderAdapterRuntime"]) {
  if (bootstrap.includes(forbidden)) errors.push(`Tauri bootstrap must not construct backend sub-runtime ${forbidden}`);
}

const lib = await readFile(resolve(backendRoot, "src/lib.rs"), "utf8");
if (!lib.includes("pub mod app_runtime")) errors.push("RustCore must expose the application backend runtime");
if (!lib.includes("pub mod catalog")) errors.push("RustCore must expose the provider-neutral catalog domain");
if (!lib.includes("pub mod provider_session")) errors.push("RustCore must expose the shared provider-session runtime boundary");
if (!lib.includes("pub mod provider_adapter")) errors.push("RustCore must expose the integrated provider adapter boundary");

const appRuntime = await readFile(resolve(backendRoot, "src/app_runtime.rs"), "utf8");
for (const needle of [
  "SearchNowBackendRuntime",
  "SettingsStore::new",
  "ProviderAdapterRuntime::compose",
  "providers.resolvers()",
  "ProviderResolvedTransport::new",
  "DownloadExecutionRuntime::new",
  "BackendRuntimeSnapshot",
]) {
  if (!appRuntime.includes(needle)) errors.push(`application backend runtime is missing composition contract ${needle}`);
}

if (errors.length) {
  for (const error of errors) console.error(`ERROR: ${error}`);
  process.exit(1);
}
console.log("SearchNow architecture contract: PASS");
