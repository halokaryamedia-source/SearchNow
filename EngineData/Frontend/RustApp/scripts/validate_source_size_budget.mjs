import { readdir, stat } from "node:fs/promises";
import { extname, relative, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const appRoot = resolve(fileURLToPath(new URL("..", import.meta.url)));
const roots = [
  resolve(appRoot, "src"),
  resolve(appRoot, "src-tauri", "src"),
  resolve(appRoot, "../../../Backend/RustCore/src"),
];
const tracked = new Set([".rs", ".svelte", ".ts"]);
const budgets = { ".rs": 20_000, ".svelte": 18_000, ".ts": 16_000 };

async function collect(directory) {
  const entries = await readdir(directory, { withFileTypes: true });
  const files = [];
  for (const entry of entries) {
    const path = resolve(directory, entry.name);
    if (entry.isDirectory()) files.push(...await collect(path));
    else if (entry.isFile() && tracked.has(extname(entry.name))) files.push(path);
  }
  return files;
}

const violations = [];
for (const root of roots) {
  for (const path of await collect(root)) {
    const extension = extname(path);
    const { size } = await stat(path);
    const budget = budgets[extension];
    if (size > budget) violations.push({ path: relative(appRoot, path).replaceAll("\\", "/"), size, budget });
  }
}

if (violations.length) {
  console.error("Source size budget exceeded:");
  for (const item of violations) console.error(`- ${item.path}: ${item.size} > ${item.budget} bytes`);
  console.error("Split ownership before adding responsibility; do not raise a budget by default.");
  process.exit(1);
}
console.log("SearchNow source size budget: PASS");
