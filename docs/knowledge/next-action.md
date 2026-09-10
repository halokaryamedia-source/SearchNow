# Next Action

Status: `REMOTE_GITHUB_FOUNDATION_COMPLETE`

## Verified checkpoint

The `develop` repository foundation is closed for the current backend-first scope.

Completed remotely:

- one `SearchNowBackendRuntime` application owner;
- settings/Minecraft/library/package/catalog/provider/download runtime foundations;
- shared crash-recoverable persistence;
- fail-closed persisted download recovery and crash reconciliation;
- Windows-safe destination naming;
- production transport isolation and provider-neutral download intent at Tauri IPC;
- canonical provider/resource identity validation;
- secret-safe diagnostics with current component health separated from event history;
- observable scheduler continuation failure state;
- standard committed Tauri PNG/ICO resources;
- committed npm + Rust dependency locks;
- read-only deterministic CI using `npm ci` and Cargo `--locked`;
- hosted Linux repository/backend/frontend verification;
- hosted Windows RustCore tests and Tauri compile verification.

The current RustCore suite contains **71 passing tests** in the remote verification baseline.

## Next boundary

```text
REMOTE_GITHUB_FOUNDATION_COMPLETE
→ TARGET_WINDOWS_RUNTIME_SMOKE
```

The next canonical action is **local Windows runtime validation**, not another repository-foundation rewrite.

Use the existing non-destructive readiness tooling and actual Tauri app runtime to validate:

1. application/AppData path resolution;
2. current Minecraft Bedrock GDK/account-scoped discovery and legacy UWP fallback;
3. settings save/reload recovery behavior;
4. local library/package inspection against representative content;
5. download finalization/recovery behavior with representative fixtures;
6. safe diagnostics/health snapshots;
7. actual Tauri window startup and command invocation on the target Windows machine.

## Boundary after local smoke

Only after the target-Windows foundation smoke is accepted should active development move to the next product slice, expected to be real provider integration and then Discover/Downloads/Settings product wiring.

Real-provider work must preserve the existing runtime, credential, provider-resource, and product-intent boundaries; it is an extension of this foundation, not a second system.

## Proof rule

Hosted Linux/Windows CI proves repository correctness and native Windows compilation. It does **not** prove installed-app behavior, user-machine Minecraft discovery, production provider compatibility, or clean-machine release readiness.
