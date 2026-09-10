import { access, readdir, readFile } from "node:fs/promises";
import { extname, relative, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const appRoot = resolve(fileURLToPath(new URL("..", import.meta.url)));
const backendRoot = resolve(appRoot, "../../Backend/RustCore");
const workspaceRoot = resolve(appRoot, "../../..");
const required = [
  "package-lock.json",
  "src/App.svelte",
  "src/app/bridge/runtimeApi.ts",
  "src/app/bridge/runtimeProductFacade.ts",
  "src/pages/Library.svelte",
  "src/pages/Discover.svelte",
  "src/pages/Downloads.svelte",
  "src/pages/Settings.svelte",
  "src-tauri/icons/icon.png",
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
  "src/diagnostics.rs",
  "src/persistence.rs",
  "src/provider_identity.rs",
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
  "src/download/recovery.rs",
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
for (const path of ["Cargo.toml", "Cargo.lock"]) {
  try { await access(resolve(workspaceRoot, path)); } catch { errors.push(`missing deterministic Rust workspace path: ${path}`); }
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

const downloadCommand = await readFile(resolve(appRoot, "src-tauri/src/commands/download.rs"), "utf8");
for (const forbidden of ["DownloadRequest", "queue_download"]) {
  if (downloadCommand.includes(forbidden)) errors.push(`download.rs: raw transport-selecting enqueue API must not be exposed through Tauri: ${forbidden}`);
}
const registry = await readFile(resolve(appRoot, "src-tauri/src/commands/registry.rs"), "utf8");
if (registry.includes("queue_download")) errors.push("command registry must not expose raw queue_download before a product-intent enqueue API exists");

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
if (!lib.includes("mod persistence")) errors.push("RustCore must keep one internal atomic persistence owner");
if (!lib.includes("mod provider_identity")) errors.push("RustCore must keep one internal provider identity contract");

const appRuntime = await readFile(resolve(backendRoot, "src/app_runtime.rs"), "utf8");
for (const needle of [
  "SearchNowBackendRuntime",
  "SettingsStore::new",
  "ProviderAdapterRuntime::compose",
  "providers.resolvers()",
  "ProviderResolvedTransport::new",
  "DownloadExecutionRuntime::new",
  "BackendRuntimeSnapshot",
  "DiagnosticsBuffer",
]) {
  if (!appRuntime.includes(needle)) errors.push(`application backend runtime is missing composition contract ${needle}`);
}
if (appRuntime.includes("DownloadTransportRegistry::with_local_file")) errors.push("production SearchNowBackendRuntime must not register the local-file test transport");

const diagnostics = await readFile(resolve(backendRoot, "src/diagnostics.rs"), "utf8");
for (const needle of ["degraded_components", "HashSet<DiagnosticComponent>", "record_outcome"]) {
  if (!diagnostics.includes(needle)) errors.push(`diagnostics current-health contract is missing ${needle}`);
}

const persistence = await readFile(resolve(backendRoot, "src/persistence.rs"), "utf8");
for (const needle of ["AtomicJsonStore", "backup_path", "cleanup_stale_temps", "sync_all"]) {
  if (!persistence.includes(needle)) errors.push(`atomic persistence contract is missing ${needle}`);
}

const recovery = await readFile(resolve(backendRoot, "src/download/recovery.rs"), "utf8");
for (const needle of ["validate_and_reconcile_persisted_state", "duplicate job ids", "Finalizing"]) {
  if (!recovery.includes(needle)) errors.push(`download recovery contract is missing ${needle}`);
}

if (errors.length) {
  for (const error of errors) console.error(`ERROR: ${error}`);
  process.exit(1);
}
console.log("SearchNow architecture contract: PASS");
