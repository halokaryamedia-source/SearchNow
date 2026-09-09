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
EngineData/Backend/RustCore/src/
├─ settings.rs    typed/versioned settings and staged persistence
├─ platform.rs    Windows process path context
├─ minecraft.rs   Bedrock storage discovery
├─ library.rs     bounded read-only local index
├─ package/       bounded read-only package inspection
├─ runtime.rs     backend identity
└─ error.rs       internal structured failures

EngineData/Frontend/RustApp/src-tauri/src/commands/
→ Tauri IPC adaptation only
```

The Tauri command boundary must delegate to RustCore. Business/filesystem/archive logic does not accumulate in commands.

## Minecraft storage discovery

Current Minecraft for Windows creator storage moved with the GDK migration from Minecraft 1.21.120. SearchNow checks current storage first:

```text
%APPDATA%/Minecraft Bedrock/users/shared/games/com.mojang
%APPDATA%/Minecraft Bedrock/users/<account>/games/com.mojang
```

Preview is opt-in. Legacy UWP remains an optional compatibility fallback. Discovery means **local storage detected**, not proof that a particular game executable/version is healthy.

## Local library scan

Normal scan is deliberately shallow and read-only:

```text
behavior_packs/
resource_packs/
skin_packs/
minecraftWorlds/
```

Development folders are opt-in. Each container scan is bounded and only reads direct child directories plus small metadata such as `manifest.json` or `levelname.txt`.

Normal discovery does **not** recursively calculate pack sizes, hash every content file, open entitlement files, derive/upload decryption keys, contact catalog services, or modify Minecraft data.

## Package inspection

Package inspection is a separate read-only capability owned by `RustCore/src/package/`.

Accepted inputs:

```text
folder
.mcpack
.mcaddon
```

Current manifest classification recognizes primary Bedrock module categories:

```text
data / script   → BehaviorPack
resources       → ResourcePack
skin_pack       → SkinPack
world_template  → WorldTemplate
```

Unknown or mixed primary module categories are reported explicitly rather than silently coerced.

BP↔RP relationships are resolved from manifest dependency UUIDs against pack header UUIDs. Folder names such as `BP` or `RP` are not treated as relationship truth.

### Archive safety

Inspection never extracts an archive. It reads ZIP metadata and bounded `manifest.json` payloads only.

Current tripwires:

```text
archive file                  ≤ 4 GiB
archive entries               ≤ 20,000
reported total uncompressed   ≤ 16 GiB
single entry                  ≤ 4 GiB
manifest.json                 ≤ 1 MiB
manifest search depth         ≤ 3 archive path components
folder search depth           ≤ 2
folder directories inspected  ≤ 512
```

The inspector rejects unsafe/ambiguous archive shapes such as:

- paths that are not enclosed in the archive namespace (`../`, absolute escape patterns);
- symlink entries;
- duplicate normalized paths;
- extreme compression ratio after the large-uncompressed threshold;
- archive/entry/uncompressed-size limit violations.

Nested `.mcpack` / `.mcaddon` entries are counted and reported but are not recursively expanded in the current slice. Inspection never mutates the source package.

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
- filesystem/archive scans run behind Tauri `spawn_blocking`, never as frontend logic.

These are safety/performance bounds, not product limits. Raising them requires evidence from real workloads.

## Error model

Expected absence is state, not exception:

```text
Found | NotFound | UnsupportedPlatform
```

Corrupt individual content/package metadata becomes an item/inspection issue. Hard settings/storage/input failures use stable error codes. Tauri maps internal errors to a small serializable command error without making IPC the domain owner.

## Verification boundary

REMOTE_GITHUB can prove:

- RustCore compiles and unit tests pass;
- clippy/format/static architecture gates pass;
- fixture-based GDK/UWP discovery works;
- bounded pack/world indexing works;
- folder/`.mcpack`/`.mcaddon` inspection works on fixtures;
- dependency-based BP↔RP relationship detection works;
- path-traversal archive fixtures are rejected.

TARGET_WINDOWS is still required to prove real AppData paths, permissions, actual Minecraft content, Tauri IPC runtime behavior, and performance on representative real packages/libraries.
