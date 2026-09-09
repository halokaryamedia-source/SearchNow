from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[1]

REQUIRED = [
    "README.md", "AGENTS.md", "CONTEXT.md", "GITHUB_RULES.md", "CONTRIBUTING.md", "SECURITY.md",
    ".agents/skills/development-brief/SKILL.md",
    "docs/README.md",
    "docs/foundation/00-product-boundaries.md",
    "docs/foundation/01-development-flow.md",
    "docs/foundation/02-target-product-flow.md",
    "docs/foundation/03-implementation-roadmap.md",
    "docs/foundation/04-verification-promotion.md",
    "docs/foundation/05-application-architecture.md",
    "docs/foundation/06-backend-architecture.md",
    "docs/foundation/07-catalog-architecture.md",
    "docs/foundation/08-provider-session-architecture.md",
    "docs/foundation/09-provider-adapter-architecture.md",
    "docs/foundation/10-application-runtime-architecture.md",
    "docs/foundation/11-observability-windows-readiness.md",
    "docs/knowledge/next-action.md", "docs/knowledge/ownership.md", "docs/knowledge/source-authority.md",
    "docs/knowledge/work-routing.md", "docs/knowledge/work-modes/development.md", "docs/knowledge/work-modes/maintenance.md",
    "docs/legacy/01-current-state.md", "docs/legacy/04-recovered-source-architecture.md",
    "docs/legacy/05-recovered-symbol-map.md", "docs/legacy/06-runtime-data-contracts.md", "docs/legacy/07-reconstruction-evidence.md",
    "EngineData/Backend/RustCore/Cargo.toml", "EngineData/Backend/RustCore/src/lib.rs",
    "EngineData/Backend/RustCore/src/app_runtime.rs", "EngineData/Backend/RustCore/src/diagnostics.rs",
    "EngineData/Backend/RustCore/src/settings.rs", "EngineData/Backend/RustCore/src/minecraft.rs",
    "EngineData/Backend/RustCore/src/library.rs", "EngineData/Backend/RustCore/src/download/resolver.rs",
    "EngineData/Backend/RustCore/src/catalog/mod.rs", "EngineData/Backend/RustCore/src/catalog/model.rs",
    "EngineData/Backend/RustCore/src/catalog/provider.rs",
    "EngineData/Backend/RustCore/src/provider_session/mod.rs",
    "EngineData/Backend/RustCore/src/provider_session/model.rs",
    "EngineData/Backend/RustCore/src/provider_session/runtime.rs",
    "EngineData/Backend/RustCore/src/provider_adapter/mod.rs",
    "EngineData/Backend/RustCore/src/provider_adapter/model.rs",
    "EngineData/Backend/RustCore/src/provider_adapter/runtime.rs",
    "EngineData/Frontend/RustApp/src-tauri/build.rs",
    "EngineData/Frontend/RustApp/src-tauri/src/app_bootstrap.rs",
    "EngineData/Frontend/RustApp/src-tauri/src/commands/registry.rs",
    "tools/windows_smoke_readiness.ps1",
    ".github/PULL_REQUEST_TEMPLATE.md", ".github/workflows/repository-verify.yml",
    ".github/workflows/local-promotion-verify.yml", ".github/workflows/release-verify.yml",
]

errors = [f"missing required path: {rel}" for rel in REQUIRED if not (ROOT / rel).exists()]

checks = {
    "README.md": ["develop", "Local", "main", "Context Recovery", "Stable Promotion"],
    "AGENTS.md": ["first wrong owner", "develop", "Local", "main"],
    "CONTEXT.md": ["Development branch: `develop`", "Verified integration baseline: `Local`", "Stable branch: `main`"],
    "GITHUB_RULES.md": ["PIN", "READ MINIMUM", "WRITE ONCE", "STOP"],
    "docs/foundation/06-backend-architecture.md": ["RustCore", "GDK", "read-only", "Tauri"],
    "docs/foundation/07-catalog-architecture.md": ["CatalogProvider", "CatalogService", "ProviderResourceRef"],
    "docs/foundation/08-provider-session-architecture.md": ["ProviderSessionSource", "ProviderSessionManager", "non-serializable", "refresh storm"],
    "docs/foundation/09-provider-adapter-architecture.md": ["IntegratedProvider", "ProviderAdapterRuntime", "CatalogProvider", "ResourceResolver"],
    "docs/foundation/10-application-runtime-architecture.md": ["SearchNowBackendRuntime", "one managed state", "ProviderResolvedTransport", "BackendRuntimeSnapshot"],
    "docs/foundation/11-observability-windows-readiness.md": ["DiagnosticsBuffer", "bounded", "Windows RustCore + Tauri compile gate", "OUT_DIR"],
}

for rel, needles in checks.items():
    path = ROOT / rel
    if not path.exists():
        continue
    text = path.read_text(encoding="utf-8")
    for needle in needles:
        if needle not in text:
            errors.append(f"{rel}: missing contract text {needle!r}")

active_backend = ROOT / "EngineData" / "Backend" / "RustCore" / "src"
for path in active_backend.rglob("*.rs") if active_backend.exists() else []:
    text = path.read_text(encoding="utf-8", errors="replace")
    for forbidden in ["mc-prod.vercel.app", "ContentKey", "keys.tsv", "decryptEntitlementFile"]:
        if forbidden in text:
            errors.append(f"{path.relative_to(ROOT)}: forbidden legacy protected-content dependency {forbidden!r}")

for rel in [
    "EngineData/Backend/RustCore/src/download/model.rs",
    "EngineData/Backend/RustCore/src/download/store.rs",
    "EngineData/Backend/RustCore/src/catalog/model.rs",
    "EngineData/Backend/RustCore/src/provider_adapter/model.rs",
]:
    path = ROOT / rel
    if not path.exists():
        continue
    lowered = path.read_text(encoding="utf-8", errors="replace").lower()
    for forbidden in ["authorization", "bearer_token", "signed_url", "cookie", "headers", "access_token", "refresh_token"]:
        if forbidden in lowered:
            errors.append(f"{rel}: runtime credential field must not enter persisted/public DTO state: {forbidden!r}")

session_runtime = ROOT / "EngineData" / "Backend" / "RustCore" / "src" / "provider_session" / "runtime.rs"
if session_runtime.exists():
    text = session_runtime.read_text(encoding="utf-8", errors="replace")
    for forbidden in ["Serialize", "Deserialize", "#[derive(Debug", "impl std::fmt::Debug for ProviderSessionMaterial", "impl std::fmt::Debug for ProviderSessionLease"]:
        if forbidden in text:
            errors.append(f"{session_runtime.relative_to(ROOT)}: runtime session material must remain non-serializable/non-debug: {forbidden!r}")

provider_boundary_names = ["IntegratedProvider", "ProviderSessionSource", "CatalogProvider", "ResourceResolver", "ProviderAdapterRuntime"]
commands_root = ROOT / "EngineData" / "Frontend" / "RustApp" / "src-tauri" / "src" / "commands"
active_command_files = ["runtime.rs", "settings.rs", "minecraft.rs", "library.rs", "package.rs", "download.rs"]
for name in active_command_files:
    path = commands_root / name
    if not path.exists():
        continue
    text = path.read_text(encoding="utf-8", errors="replace")
    if "SearchNowBackendRuntime" not in text:
        errors.append(f"{path.relative_to(ROOT)}: active Tauri feature command must delegate through SearchNowBackendRuntime")
    for forbidden in ["SettingsStore", "PlatformContext", "DownloadExecutionRuntime", "DownloadTransportRegistry", "ResourceResolverRegistry", "HttpTransport", "ProviderAdapterRuntime", "IntegratedProvider"]:
        if forbidden in text:
            errors.append(f"{path.relative_to(ROOT)}: backend sub-runtime ownership belongs in SearchNowBackendRuntime, not Tauri command: {forbidden!r}")

commands_mod = commands_root / "mod.rs"
if commands_mod.exists() and "mod context" in commands_mod.read_text(encoding="utf-8", errors="replace"):
    errors.append("Tauri commands must not reactivate the obsolete per-command backend context helper")

bootstrap = ROOT / "EngineData" / "Frontend" / "RustApp" / "src-tauri" / "src" / "app_bootstrap.rs"
if bootstrap.exists():
    text = bootstrap.read_text(encoding="utf-8", errors="replace")
    for needle in ["SearchNowBackendRuntime::new", "app.manage(runtime)"]:
        if needle not in text:
            errors.append(f"{bootstrap.relative_to(ROOT)}: missing consolidated backend bootstrap contract {needle!r}")
    for forbidden in ["DownloadExecutionRuntime", "DownloadTransportRegistry", "ResourceResolverRegistry", "HttpTransport", "ProviderAdapterRuntime"]:
        if forbidden in text:
            errors.append(f"{bootstrap.relative_to(ROOT)}: Tauri bootstrap must not construct backend sub-runtime {forbidden!r}")

app_runtime = ROOT / "EngineData" / "Backend" / "RustCore" / "src" / "app_runtime.rs"
if app_runtime.exists():
    text = app_runtime.read_text(encoding="utf-8", errors="replace")
    for needle in ["SearchNowBackendRuntime", "ProviderAdapterRuntime::compose", "providers.resolvers()", "ProviderResolvedTransport::new", "DownloadExecutionRuntime::new", "SettingsStore::new", "BackendRuntimeSnapshot", "DiagnosticsBuffer"]:
        if needle not in text:
            errors.append(f"{app_runtime.relative_to(ROOT)}: missing application composition/observability contract {needle!r}")

# Diagnostics are intentionally bounded and static-message only. Guard ownership patterns,
# not anti-leak assertion literals used by tests.
diagnostics = ROOT / "EngineData" / "Backend" / "RustCore" / "src" / "diagnostics.rs"
if diagnostics.exists():
    text = diagnostics.read_text(encoding="utf-8", errors="replace")
    for needle in ["DEFAULT_DIAGNOSTIC_CAPACITY", "MAX_DIAGNOSTIC_CAPACITY", "VecDeque", "pub message: &'static str", "BackendHealthSnapshot"]:
        if needle not in text:
            errors.append(f"{diagnostics.relative_to(ROOT)}: missing bounded/static diagnostic contract {needle!r}")
    lowered = text.lower()
    for forbidden in [
        "pub authorization:", "authorization: string", "pub bearer_token:", "bearer_token: string",
        "pub signed_url:", "signed_url: string", "pub access_token:", "access_token: string",
        "pub refresh_token:", "refresh_token: string", "pub cookie:", "cookie: string",
        "pub headers:", "headers: hashmap", "headers: vec<",
    ]:
        if forbidden in lowered:
            errors.append(f"{diagnostics.relative_to(ROOT)}: diagnostic implementation must not own credential field {forbidden!r}")

build_rs = ROOT / "EngineData" / "Frontend" / "RustApp" / "src-tauri" / "build.rs"
if build_rs.exists():
    text = build_rs.read_text(encoding="utf-8", errors="replace")
    for needle in ["OUT_DIR", "window_icon_path", "build_placeholder_ico", "tauri_build::try_build"]:
        if needle not in text:
            errors.append(f"{build_rs.relative_to(ROOT)}: missing Windows build-readiness fallback contract {needle!r}")

for workflow_rel in [
    ".github/workflows/repository-verify.yml",
    ".github/workflows/local-promotion-verify.yml",
    ".github/workflows/release-verify.yml",
]:
    workflow = ROOT / workflow_rel
    if workflow.exists():
        text = workflow.read_text(encoding="utf-8", errors="replace")
        for needle in ["windows-latest", "cargo check --manifest-path EngineData/Frontend/RustApp/src-tauri/Cargo.toml"]:
            if needle not in text:
                errors.append(f"{workflow_rel}: missing hosted Windows Tauri compile gate {needle!r}")

public_http = ROOT / "EngineData" / "Backend" / "RustCore" / "src" / "download" / "http.rs"
if public_http.exists():
    text = public_http.read_text(encoding="utf-8", errors="replace")
    for forbidden in provider_boundary_names:
        if forbidden in text:
            errors.append(f"{public_http.relative_to(ROOT)}: public HTTP transport must remain provider-neutral: {forbidden!r}")

for legacy_old in [
    "docs/01-current-state.md", "docs/04-recovered-source-architecture.md", "docs/05-recovered-symbol-map.md",
    "docs/06-runtime-data-contracts.md", "docs/07-reconstruction-evidence.md",
]:
    if (ROOT / legacy_old).exists():
        errors.append(f"legacy document not normalized into docs/legacy/: {legacy_old}")

if errors:
    for error in errors:
        print(f"ERROR: {error}")
    sys.exit(1)

print("SearchNow repository contracts: PASS")
