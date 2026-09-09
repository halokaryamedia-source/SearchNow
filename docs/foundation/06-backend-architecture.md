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
├─ download/      transport-agnostic job lifecycle/persistence/workspace
├─ runtime.rs     backend identity
└─ error.rs       internal structured failures

EngineData/Frontend/RustApp/src-tauri/src/commands/
→ Tauri IPC adaptation only
```

The Tauri command boundary must delegate to RustCore. Business/filesystem/archive/download lifecycle logic does not accumulate in commands.

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

Unknown or mixed primary module categories are reported explicitly rather than silently coerced. BP↔RP relationships are resolved from manifest dependency UUIDs against pack header UUIDs; folder names are not relationship truth.

Archive inspection never extracts a package. It rejects traversal/absolute escape shapes, symlinks, duplicate normalized paths, excessive sizes, and extreme compression ratios while reading only bounded metadata/manifests.

## Download manager

`RustCore/src/download/` owns download lifecycle without owning a network provider.

Persisted source identity is intentionally narrow:

```text
transport key
resource id
```

Do **not** persist authorization headers, bearer tokens, signed URLs, cookies, provider secrets, or other short-lived credentials in download queue state. A future transport adapter resolves a resource id into a live transfer request at execution time.

### State machine

```text
Queued
  ├─ cancel → Cancelled
  └─ claim  → Preparing
                ├─ failure → Failed
                ├─ cancel  → CancelRequested → Cancelled
                └─ start   → Transferring
                               ├─ failure → Failed
                               ├─ cancel  → CancelRequested → Cancelled
                               └─ complete bytes → Finalizing → Completed

Failed(retryable) / Cancelled / Interrupted
  └─ retry → Queued
```

`Finalizing` cannot be cancelled through the ordinary command because publication must not be interrupted halfway through its atomic boundary.

### Concurrency and queue bounds

Defaults:

```text
max active jobs   3
max retained jobs 1000
```

Validation hard caps:

```text
max active jobs   16
max retained jobs 5000
```

`claim_ready_jobs()` is the only scheduler-facing path that moves queued jobs into active ownership and it never exceeds available active slots.

### Progress semantics

Progress bytes are monotonic within an attempt. If a total size is known it cannot change mid-attempt and downloaded bytes cannot exceed it. A known-size job cannot enter `Finalizing` until all declared bytes have arrived.

Retry resets downloaded bytes to zero because resume/range semantics are not yet part of the transport contract.

### Persistence and restart recovery

Download state uses a dedicated schema-versioned JSON store with a 4 MiB state cap and staged write + replacement/rollback behavior.

On restart:

- `Queued` remains queued;
- terminal states remain truthful;
- `Preparing`, `Transferring`, `Finalizing`, and `CancelRequested` become `Interrupted` with a retryable interruption reason;
- no active job silently resumes without an explicit transport/resume contract.

Tauri queue mutations use clone → mutate → persist → replace-memory semantics, so a failed persistence write does not leave in-memory state claiming a mutation that was not stored.

### Workspace and final publication

A job receives an app-owned workspace containing `payload.part`. Destination input accepts one safe filename only; path separators, `..`, control characters, and unsafe job ids are rejected.

Final publication follows this boundary:

```text
workspace payload.part
        ↓ copy
<destination>/.searchnow-<job>.part
        ↓ fsync
atomic hard-link publish, no overwrite
        ↓
final destination file
```

The staging file lives in the destination directory, so final publication is on the destination filesystem and the final name never becomes visible as a partial file. Existing destinations are not overwritten automatically.

This atomic-publication strategy still requires TARGET_WINDOWS/filesystem evidence on representative user volumes before release claims are made.

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
- package/archive inspection is bounded and read-only;
- download concurrency is bounded and queue state has a persistence size cap;
- symlink directories/package inputs are skipped or rejected at their boundaries;
- filesystem/archive scans run behind appropriate native boundaries, never as frontend logic.

These are safety/performance bounds, not product limits. Raising them requires evidence from real workloads.

## Error model

Expected absence is state, not exception:

```text
Found | NotFound | UnsupportedPlatform
```

Corrupt individual content/package metadata becomes an item/inspection issue. Download lifecycle uses explicit job state plus stable failure codes. Hard settings/storage/input failures use stable backend errors. Tauri maps internal errors to a small serializable command error without making IPC the domain owner.

## Verification boundary

REMOTE_GITHUB can prove:

- RustCore compiles and unit tests pass;
- clippy/format/static architecture gates pass;
- fixture-based GDK/UWP discovery works;
- bounded pack/world indexing works;
- folder/`.mcpack`/`.mcaddon` inspection works on fixtures;
- dependency-based BP↔RP relationship detection works;
- path-traversal archive fixtures are rejected;
- download state transitions/concurrency/progress/recovery/persistence behave on deterministic fixtures;
- destination traversal is rejected and atomic no-overwrite publication works on the CI filesystem.

TARGET_WINDOWS is still required to prove real AppData paths, permissions, actual Minecraft content, Tauri IPC runtime behavior, destination-filesystem behavior, and performance on representative real packages/libraries/download workloads.
