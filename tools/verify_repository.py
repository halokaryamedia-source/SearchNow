from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[1]

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
    "docs/knowledge/next-action.md",
    "docs/knowledge/ownership.md",
    "docs/knowledge/source-authority.md",
    "docs/knowledge/work-routing.md",
    "docs/knowledge/work-modes/development.md",
    "docs/knowledge/work-modes/maintenance.md",
    "docs/legacy/01-current-state.md",
    "docs/legacy/04-recovered-source-architecture.md",
    "docs/legacy/05-recovered-symbol-map.md",
    "docs/legacy/06-runtime-data-contracts.md",
    "docs/legacy/07-reconstruction-evidence.md",
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
    "README.md": ["develop", "Local", "main", "Context Recovery", "Stable Promotion"],
    "AGENTS.md": ["first wrong owner", "develop", "Local", "main"],
    "CONTEXT.md": ["Development branch: `develop`", "Verified integration baseline: `Local`", "Stable branch: `main`"],
    "GITHUB_RULES.md": ["PIN", "READ MINIMUM", "WRITE ONCE", "STOP"],
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

if errors:
    for error in errors:
        print(f"ERROR: {error}")
    sys.exit(1)

print("SearchNow repository contracts: PASS")
