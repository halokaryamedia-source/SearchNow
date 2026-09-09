import { access, readdir, readFile } from "node:fs/promises";
import { extname, relative, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const appRoot = resolve(fileURLToPath(new URL("..", import.meta.url)));
const required = [
  "src/App.svelte",
  "src/app/bridge/runtimeApi.ts",
  "src/app/bridge/runtimeProductFacade.ts",
  "src/app/shared/types.ts",
  "src/pages/Library.svelte",
  "src/pages/Discover.svelte",
  "src/pages/Downloads.svelte",
  "src/pages/Settings.svelte",
  "src-tauri/src/main.rs",
  "src-tauri/src/commands/registry.rs",
  "src-tauri/src/commands/runtime.rs",
  "src-tauri/src/engine/runtime.rs",
];

const errors = [];
for (const path of required) {
  try {
    await access(resolve(appRoot, path));
  } catch {
    errors.push(`missing required architecture path: ${path}`);
  }
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
  if (text.includes("@tauri-apps/api/core") && rel !== "src/app/bridge/runtimeApi.ts") {
    errors.push(`${rel}: direct Tauri invoke import is reserved for runtimeApi.ts`);
  }
  if (rel.startsWith("src/pages/") && text.includes("runtimeApi")) {
    errors.push(`${rel}: pages must use product-facing props/actions, not runtimeApi directly`);
  }
}

const runtimeCommand = await readFile(resolve(appRoot, "src-tauri/src/commands/runtime.rs"), "utf8");
if (!runtimeCommand.includes("crate::engine::runtime")) {
  errors.push("runtime command must delegate to the Rust engine");
}

if (errors.length) {
  for (const error of errors) console.error(`ERROR: ${error}`);
  process.exit(1);
}

console.log("SearchNow architecture contract: PASS");
