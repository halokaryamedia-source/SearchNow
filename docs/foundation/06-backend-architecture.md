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
├─ download/
│  ├─ model.rs       download DTOs/state contract
│  ├─ manager.rs     job lifecycle/state machine/concurrency
│  ├─ store.rs       schema-versioned persistence/recovery
│  ├─ workspace.rs   payload workspace + atomic publication
│  ├─ transport.rs   provider-neutral transport contract + local-file
│  ├─ executor.rs    scheduler/execution/progress checkpoints
│  └─ http.rs        provider-neutral public HTTPS transport
├─ runtime.rs     backend identity
└─ error.rs       internal structured failures

EngineData/Frontend/RustApp/src-tauri/src/
├─ app_bootstrap.rs          construct/manage application runtime owners
└─ commands/                 Tauri IPC adaptation only
```

The Tauri command boundary must delegate to RustCore. Business/filesystem/archive/download lifecycle/transport execution logic does not accumulate in commands.

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

`RustCore/src/download/manager.rs` owns download lifecycle without owning a network provider.

Persisted source identity is intentionally narrow:

```text
transport key
resource id
```

Do **not** persist authorization headers, bearer tokens, signed URLs, cookies, provider secrets, or other short-lived credentials in download queue state. Provider/authenticated resources must later resolve their opaque resource id into ephemeral runtime request material outside persisted job state.

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

Lifecycle mutations use clone → mutate → persist → replace-memory semantics. Transfer progress is cheaper: memory is updated every chunk, but disk persistence is checkpointed at 1 MiB boundaries and completion/lifecycle transitions. A crash therefore never silently resumes an active transfer; recovered active work becomes `Interrupted` even if the last persisted byte counter trails memory.

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

Workspace reuse is guarded: existing workspace/payload paths must be regular non-symlink filesystem objects. Completed/cancelled/failed execution attempts clean their job workspace best-effort because range/resume is not yet supported.

## Transport execution

`RustCore/src/download/transport.rs` defines the transport adapter contract. A transport owns only how a `DownloadSourceRef` becomes a readable byte stream plus optional declared total size. It does **not** own queue state, retries, persistence, finalization, destination paths, or UI state.

`RustCore/src/download/executor.rs` owns scheduling and execution:

```text
claim queued jobs
      ↓
Preparing
      ↓ open transport
create/reset app workspace payload
      ↓
Transferring
      ↓ read bounded chunks
write payload + report progress
      ↓
Finalizing
      ↓ atomic publication
Completed / Failed / Cancelled
      ↓
pump next queued jobs
```

The executor uses native worker threads inside the same SearchNow process. It does not create a daemon, local HTTP service, or second runtime. Multiple `pump()` calls are safe at the scheduling boundary because claims happen while the manager is locked and active-slot accounting is authoritative.

Cancellation is cooperative. A worker checks `CancelRequested` between transport reads; cancellation wins over a concurrent transfer/progress error when the cancellation state is already recorded. `Finalizing` remains non-cancellable.

Transport read errors are normalized by the executor: socket timeout/would-block failures become retryable `download_transfer_timeout`; bounded-stream invalid-data failures become non-retryable transfer-data failures; other read failures remain retryable unless a transport has already provided a more specific pre-stream failure.

### Concrete transport: `local-file`

`local-file` reads a regular non-symlink local file and reports its metadata size. It exists to prove the complete lifecycle without network variability. Missing/temporarily inaccessible files can be marked retryable; invalid local source shapes are non-retryable.

### Concrete transport: `https-public`

`RustCore/src/download/http.rs` owns ordinary unauthenticated public HTTPS transfer behavior. It uses pinned blocking `ureq` with rustls so it fits the existing worker-thread executor without introducing Tokio or another runtime.

Production `https-public` accepts only ordinary HTTPS URLs and rejects:

- non-HTTPS schemes;
- missing hosts;
- URL username/password user-info;
- fragments;
- persisted query strings.

Persisted query strings are deliberately rejected because signed URLs and provider/authentication parameters can contain short-lived credentials. Those belong in a future runtime resolver, not `DownloadJob.resourceId`.

Redirects are handled manually rather than delegated to the HTTP client. Supported redirect statuses are 301/302/303/307/308. Every redirect target is resolved and revalidated before use, so production HTTPS work cannot silently downgrade to plain HTTP. Default redirect limit is 5; the hard validated ceiling is 10.

Default HTTP policy:

```text
connect timeout      10 seconds
read timeout         15 seconds
overall deadline     30 minutes
redirect limit       5
response limit       8 GiB
```

Hard response-size validation ceiling is 32 GiB. A declared `Content-Length` above the configured response limit is rejected before streaming. Unknown-length responses are wrapped in a byte-counting reader so they cannot grow past the same configured limit. A body shorter than its declared Content-Length surfaces as a transfer read failure rather than being finalized as complete.

Socket connect/read timeouts remain owned by `ureq`. SearchNow owns the overall deadline separately with a monotonic `Instant` wrapper. This split is intentional: `ureq 2.x`'s request-level `timeout()` takes precedence over the dedicated read timeout, so SearchNow does **not** use it for the overall deadline. The separate wrapper preserves both a short stalled-read bound and a longer total-transfer deadline.

HTTP status failures are retryable only for the bounded transient set currently recognized by the transport (`408`, `425`, `429`, `500`, `502`, `503`, `504`). Other HTTP status failures are treated as non-retryable by default.

Local test fixtures may explicitly allow plain HTTP only inside the RustCore test build. This is not a production transport mode.

### Runtime bootstrap

Tauri startup constructs exactly one `DownloadExecutionRuntime` using app-owned paths:

```text
<AppData>/downloads/state.json
<AppData>/downloads/workspace/
<AppData>/downloads/files/
```

The runtime registry currently includes:

```text
local-file
https-public
```

The runtime is registered through `app.manage(...)`. Tauri download commands resolve that state and delegate queue/snapshot/cancel/retry/remove behavior to RustCore. Tauri does not duplicate manager/executor truth.

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
- transfer buffers are fixed at 256 KiB;
- progress persistence is checkpointed at 1 MiB instead of rewriting queue state for every chunk;
- public HTTPS responses have bounded redirects, socket timeouts, overall deadline, and byte limits;
- symlink directories/package inputs/download workspaces are skipped or rejected at their boundaries;
- filesystem/archive/transfer work stays in native backend boundaries, never in frontend logic.

These are safety/performance bounds, not product limits. Raising them requires evidence from real workloads.

## Error model

Expected absence is state, not exception:

```text
Found | NotFound | UnsupportedPlatform
```

Corrupt individual content/package metadata becomes an item/inspection issue. Download lifecycle uses explicit job state plus stable failure codes. Transport failures additionally declare retryability. Hard settings/storage/input failures use stable backend errors. Tauri maps internal errors to a small serializable command error without making IPC the domain owner.

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
- destination traversal is rejected and atomic no-overwrite publication works on the CI filesystem;
- local-file transport runs end-to-end through queue → executor → progress → final publication;
- executor respects bounded active concurrency and cooperative cancellation;
- public-HTTPS policy rejects plain HTTP/query credentials and enforces redirect/size/content-length rules;
- deterministic loopback HTTP fixtures prove success, redirect, stalled-read timeout, length mismatch, oversized-response rejection, and executor cancellation behavior.

TARGET_WINDOWS is still required to prove real AppData paths, permissions, actual Minecraft content, Tauri IPC runtime behavior, destination-filesystem behavior, and performance on representative real packages/libraries/download workloads. NETWORK evidence is still required before claiming real public HTTPS/TLS reliability or any provider-specific integration.
