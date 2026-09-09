# Current Validation

## Architecture scaffold

Target claim:

> SearchNow has a coherent, lightweight Tauri/Svelte/Rust source scaffold with an enforced frontend/native/runtime ownership boundary.

Repository/static evidence expected for the current `develop` HEAD:

- repository contract verifier passes;
- architecture validator passes;
- source-size budget passes;
- Svelte typecheck passes;
- Vite frontend build passes;
- Rust formatting check passes;
- `get_runtime_status` follows product facade → runtime API → Tauri command → Rust engine ownership.

## Not proven by hosted/static checks

- actual Tauri desktop launch on Windows;
- WebView2/native IPC behavior on target Windows;
- installer/bundle behavior;
- Minecraft installation discovery;
- catalog/network behavior;
- download/package/export behavior.

These claims require their matching local/target runtime proof after implementation.
