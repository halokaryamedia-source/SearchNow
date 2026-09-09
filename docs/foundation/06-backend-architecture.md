# 06 — Backend Architecture

Status: **current backend contract**

## Goal

Keep SearchNow backend local-first, testable, bounded, and cheap to run. The backend is split by source ownership, not by process count.

```text
one SearchNow executable
├─ Tauri adapter
└─ RustCore (in-process library)
```

`RustCore` is not a server, worker, daemon, or second runtime.

## Source ownership

```text
EngineData/Backend/RustCore/
├─ settings.rs    typed/versioned settings and staged persistence
├─ platform.rs    Windows process path context
├─ minecraft.rs   Bedrock storage discovery
├─ library.rs     bounded read-only local index
├─ runtime.rs     backend identity
└─ error.rs       internal structured failures

EngineData/Frontend/RustApp/src-tauri/src/commands/
→ Tauri IPC adaptation only
```

The Tauri command boundary must delegate to RustCore. Business/filesystem logic does not accumulate in commands.

## Minecraft storage discovery

Current Minecraft for Windows creator storage moved with the GDK migration from Minecraft 1.21.120. SearchNow therefore checks current storage first:

```text
%APPDATA%/Minecraft Bedrock/users/shared/games/com.mojang
%APPDATA%/Minecraft Bedrock/users/<account>/games/com.mojang
```

Preview is opt-in:

```text
%APPDATA%/Minecraft Bedrock Preview/users/shared/games/com.mojang
```

Legacy UWP remains an optional compatibility fallback:

```text
%LOCALAPPDATA%/Packages/Microsoft.MinecraftUWP_8wekyb3d8bbwe/LocalState/games/com.mojang
```

Discovery means **local storage detected**, not proof that a particular game executable/version is healthy.

Reference: Microsoft Learn, `GDK Migration on Windows (from Minecraft version 1.21.120)`.

## Local library scan

Normal scan is deliberately shallow and read-only:

```text
behavior_packs/
resource_packs/
skin_packs/
minecraftWorlds/
```

Development folders are opt-in. Each container scan is bounded and only reads direct child directories plus small metadata such as `manifest.json` or `levelname.txt`.

Normal discovery does **not**:

- recursively calculate pack sizes;
- hash every content file;
- open entitlement files;
- derive or upload decryption keys;
- contact catalog/network services;
- modify Minecraft data.

Invalid manifests become `invalidMetadata` items instead of aborting the whole library.

## Settings

Settings are typed and schema-versioned. Defaults are local-safe:

```text
Preview content            off
Legacy UWP fallback        on
Development content        off
Custom root                none
```

Persistence uses staged write + replacement/rollback behavior. Future unsupported schema versions fail closed rather than being silently interpreted.

## Performance guardrails

- at most 64 account-scoped GDK roots per product are inspected;
- at most 5,000 direct entries per content container are indexed;
- manifest reads are capped at 1 MiB;
- world name reads are capped at 4 KiB;
- symlink directories are skipped;
- filesystem scans run behind Tauri `spawn_blocking`, never as frontend logic.

These are safety/performance bounds, not product limits. Raising them requires evidence from real libraries.

## Error model

Expected absence is state, not exception:

```text
Found | NotFound | UnsupportedPlatform
```

Corrupt individual content becomes an item-level status/warning. Hard settings/storage failures use stable error codes; Tauri maps internal errors to a small serializable command error without exposing internal implementation state as product truth.

## Verification boundary

REMOTE_GITHUB can prove:

- RustCore compiles and unit tests pass;
- clippy/format/static architecture gates pass;
- fixture-based GDK/UWP discovery works;
- bounded pack/world indexing works.

TARGET_WINDOWS is still required to prove real AppData paths, permissions, actual Minecraft content, Tauri IPC runtime behavior, and performance on a representative library.
