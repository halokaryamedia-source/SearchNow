from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[1]
APP = ROOT / "EngineData" / "Frontend" / "RustApp"

REQUIRED = [
    "README.md",
    "AGENTS.md",
    "CONTEXT.md",
    "GITHUB_RULES.md",
    "CONTRIBUTING.md",
    "SECURITY.md",
    ".agents/skills/development-brief/SKILL.md",
    "docs/README.md",
    "docs/foundation/00-product-boundaries.md",
    "docs/foundation/01-development-flow.md",
    "docs/foundation/02-target-product-flow.md",
    "docs/foundation/03-implementation-roadmap.md",
    "docs/foundation/04-verification-promotion.md",
    "docs/foundation/05-application-architecture.md",
    "docs/knowledge/next-action.md",
    "docs/knowledge/ownership.md",
    "docs/knowledge/source-authority.md",
    "docs/knowledge/work-routing.md",
    "docs/knowledge/work-modes/development.md",
    "docs/knowledge/work-modes/maintenance.md",
    "docs/knowledge/decisions/D-001-application-stack.md",
    "docs/knowledge/reviews/current-validation.md",
    "docs/legacy/01-current-state.md",
    "docs/legacy/04-recovered-source-architecture.md",
    "docs/legacy/05-recovered-symbol-map.md",
    "docs/legacy/06-runtime-data-contracts.md",
    "docs/legacy/07-reconstruction-evidence.md",
    "EngineData/README.md",
    "EngineData/Frontend/RustApp/README.md",
    "EngineData/Frontend/RustApp/package.json",
    "EngineData/Frontend/RustApp/src/App.svelte",
    "EngineData/Frontend/RustApp/src/app/bridge/runtimeApi.ts",
    "EngineData/Frontend/RustApp/src/app/bridge/runtimeProductFacade.ts",
    "EngineData/Frontend/RustApp/src-tauri/Cargo.toml",
    "EngineData/Frontend/RustApp/src-tauri/src/main.rs",
    "EngineData/Frontend/RustApp/src-tauri/src/commands/registry.rs",
    "EngineData/Frontend/RustApp/src-tauri/src/engine/runtime.rs",
    "UserData/README.md",
    ".github/PULL_REQUEST_TEMPLATE.md",
    ".github/workflows/repository-verify.yml",
    ".github/workflows/local-promotion-verify.yml",
    ".github/workflows/release-verify.yml",
]

errors = []
for rel in REQUIRED:
    if not (ROOT / rel).exists():
        errors.append(f"missing required path: {rel}")

checks = {
    "README.md": ["develop", "Local", "main", "Tauri 2", "Svelte 5", "Rust"],
    "AGENTS.md": ["first wrong owner", "develop", "Local", "main"],
    "CONTEXT.md": ["Development branch: `develop`", "Verified integration baseline: `Local`", "Tauri 2", "Svelte 5", "Rust"],
    "GITHUB_RULES.md": ["PIN", "READ MINIMUM", "WRITE ONCE", "STOP"],
    "docs/foundation/05-application-architecture.md": ["runtimeApi.ts", "commands/", "engine/", "20 KB", "16 KB", "18 KB"],
}

for rel, needles in checks.items():
    path = ROOT / rel
    if not path.exists():
        continue
    text = path.read_text(encoding="utf-8")
    for needle in needles:
        if needle not in text:
            errors.append(f"{rel}: missing contract text {needle!r}")

for legacy_old in [
    "docs/01-current-state.md",
    "docs/04-recovered-source-architecture.md",
    "docs/05-recovered-symbol-map.md",
    "docs/06-runtime-data-contracts.md",
    "docs/07-reconstruction-evidence.md",
]:
    if (ROOT / legacy_old).exists():
        errors.append(f"legacy document not normalized into docs/legacy/: {legacy_old}")

if (ROOT / "EngineData" / "Backend").exists():
    errors.append("EngineData/Backend exists without a revised architecture decision; current runtime is Tauri/Svelte/Rust only")

if (ROOT / "src").exists():
    errors.append("root src/ is not the approved source root; use EngineData/Frontend/RustApp")

runtime_api = APP / "src" / "app" / "bridge" / "runtimeApi.ts"
if runtime_api.exists() and "@tauri-apps/api/core" not in runtime_api.read_text(encoding="utf-8"):
    errors.append("runtimeApi.ts must own the raw Tauri invoke boundary")

if errors:
    for error in errors:
        print(f"ERROR: {error}")
    sys.exit(1)

print("SearchNow repository contracts: PASS")
