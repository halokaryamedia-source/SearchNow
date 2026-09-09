# 03 — Implementation Roadmap

## Objective

Build SearchNow as a maintainable Minecraft Bedrock content-management application with a clear separation between UI, application logic, Minecraft-specific logic, networking, storage, and packaging.

Do not begin by recreating the old WinForms form structure 1:1. First establish the domain/services and then attach the UI to them.

## Proposed architecture

```text
SearchNow.UI
    │
    ▼
SearchNow.Application
    │
    ├── MinecraftService
    ├── LibraryService
    ├── CatalogService
    ├── DownloadService
    ├── PackageService
    ├── ExportService
    └── DiagnosticsService
    │
    ▼
SearchNow.Infrastructure
    ├── Minecraft filesystem adapter
    ├── HTTP / catalog adapter
    ├── local storage
    ├── archive I/O
    └── logging
```

Suggested project boundaries:

```text
src/
  SearchNow.UI/
  SearchNow.Application/
  SearchNow.Core/
  SearchNow.Infrastructure/

tests/
  SearchNow.Core.Tests/
  SearchNow.Application.Tests/
  SearchNow.IntegrationTests/

docs/
```

The exact UI technology can be chosen later. Core/application projects should not depend on WinForms/WPF/WinUI-specific types.

## Core domain models

Start with explicit models rather than passing JSON/dictionaries through the entire application.

Candidate models:

```text
MinecraftInstallation
ContentItem
ContentSource
ContentType
ContentStatus
PackageManifest
DownloadJob
ExportJob
ValidationResult
AppSettings
```

### ContentItem

Should represent what the UI needs without exposing transport details.

Conceptual fields:

```text
Id
Title
Description
Type
Thumbnail
Source
Status
InstalledPath?
RemoteReference?
TechnicalMetadata?
```

Transport-specific DTOs should remain in infrastructure adapters.

## Service responsibilities

### MinecraftService

Responsible for:

- discovering Minecraft installation/data folders;
- validating configured paths;
- enumerating legitimate local content;
- watching/rescanning local content when requested.

It must not perform unrelated catalog/network work during discovery.

### LibraryService

Responsible for combining local content sources into one normalized library model.

Should handle:

- deduplication;
- status calculation;
- sorting/filtering support;
- local metadata cache.

### CatalogService

Responsible for remote catalog discovery through permitted endpoints.

Requirements:

- isolated authentication/session management;
- cancellation support;
- request timeout;
- retry policy only for transient errors;
- normalized responses;
- no catalog request on every search keystroke.

### DownloadService

Responsible for a persistent or resumable queue where feasible.

State machine:

```text
Queued
→ Downloading
→ Validating
→ Processing
→ Completed

or

→ Failed / Cancelled
```

UI must observe job state rather than implement download logic itself.

### PackageService

Responsible for legitimate package operations:

- inspect archives/folders;
- read manifests;
- detect content type;
- validate structure;
- pair BP/RP where applicable;
- stage files for export;
- package supported Minecraft formats.

Source input must not be silently overwritten.

### ExportService

Responsible for:

- selecting the correct package format;
- output naming;
- duplicate policy;
- atomic/staged writes;
- post-write verification;
- returning the final output path to the UI.

### DiagnosticsService

Responsible for structured logs and troubleshooting information.

Logs should never casually include account tokens, secrets, entitlement material, or other sensitive values.

## Configuration

Use one typed settings model.

Suggested categories:

```text
Minecraft
Downloads
Export
Privacy
Diagnostics
UI
```

Settings must have safe defaults. Invalid configuration should fall back gracefully or produce a recoverable validation error.

## Privacy requirements

Non-negotiable defaults:

1. Local Minecraft/account-derived data remains local unless the user explicitly enables a documented external feature.
2. No automatic upload of content keys or entitlement-derived secrets.
3. Do not store access/session tokens in plaintext logs.
4. Network operations must have an identifiable feature/user action that triggered them.
5. Diagnostics sharing, if ever added, is opt-in.
6. Temporary processing data is cleaned predictably.

## Product boundary

SearchNow modernization should not implement or optimize mechanisms whose purpose is to bypass Marketplace DRM, make paid content free, or distribute decryption keys.

The engineering target is legitimate discovery, management, validation, downloading through permitted access paths, import/export of accessible content, and package tooling.

## Development phases

### Phase 0 — Documentation / Decisions

Status: **in progress**

Deliverables:

- current-state baseline;
- target user flow;
- implementation roadmap;
- later: UI wireframe and technical decision record.

No implementation should be considered stable until these documents agree.

### Phase 1 — Repository foundation

Create solution/project structure and baseline tooling.

Deliverables:

- solution and projects;
- formatter/analyzers;
- nullable reference types enabled;
- dependency injection/bootstrap;
- structured logging;
- unit-test projects;
- CI build/test workflow.

Exit criteria:

```text
restore → build → test
```

works on a clean checkout.

### Phase 2 — Minecraft local discovery

Implement only local functionality first.

Deliverables:

- automatic Minecraft path discovery;
- manual path override;
- validation;
- local content enumeration;
- normalized `ContentItem` results;
- rescan/cancellation behavior.

This phase provides a useful application even without remote services.

### Phase 3 — Library UI

Implement the first complete user-facing vertical slice.

```text
Launch
→ detect Minecraft
→ scan local content
→ show Library
→ open item details
```

Focus on responsiveness, loading states, empty states, and errors.

### Phase 4 — Import / Package inspection

Deliverables:

- drag-and-drop/import picker;
- package type detection;
- manifest inspection;
- validation results;
- BP/RP recognition;
- safe staging.

Unit tests should use synthetic/test-owned fixtures.

### Phase 5 — Catalog integration

Only after the local product flow is stable.

Deliverables:

- isolated catalog adapter;
- session lifecycle;
- search/filter API;
- content detail model;
- rate/debounce behavior;
- network error/retry handling.

### Phase 6 — Download manager

Deliverables:

- queue;
- cancellation;
- accurate progress;
- retry;
- staging;
- validation;
- clear completion output.

### Phase 7 — Export

Deliverables:

- supported Minecraft package output;
- safe duplicate handling;
- BP/RP combination where valid;
- post-export validation;
- open-output action.

### Phase 8 — UX hardening

Audit:

- startup performance;
- memory usage;
- long-running operation responsiveness;
- keyboard/navigation behavior;
- error language;
- empty/loading states;
- DPI/scaling;
- cancellation;
- network-offline behavior.

### Phase 9 — Packaging and release

Deliverables:

- reproducible release build;
- version metadata;
- application signing strategy;
- portable/installable packaging decision;
- release notes;
- clean-machine smoke test.

## Testing strategy

### Unit tests

Prioritize deterministic logic:

- manifest parsing;
- type detection;
- deduplication;
- status transitions;
- output-name generation;
- validation;
- settings parsing.

### Integration tests

Use adapters/interfaces so filesystem and HTTP behavior can be tested without depending on the UI.

Test scenarios:

```text
Minecraft missing
Minecraft path invalid
empty library
malformed manifest
corrupt archive
duplicate item
download cancelled
network timeout
HTTP rejection
insufficient disk space
export path unavailable
```

### Manual smoke tests

Keep a short reproducible checklist rather than relying on ad-hoc exploration.

## Performance rules

1. No heavy filesystem scan on the UI thread.
2. No unbounded recursive scan.
3. Avoid loading full archives into memory when streaming is possible.
4. Paginate or virtualize large catalog/library lists.
5. Cache normalized metadata, but invalidate safely.
6. Debounce user search input.
7. Cancel obsolete requests when the query changes.
8. Keep network timeouts finite.

## Naming / code quality

Use intent-based names. Experimental/joke method names from the inspected binary are not to be reproduced.

Examples:

```text
InitializeMinecraftInstallationAsync
DiscoverLocalContentAsync
SearchCatalogAsync
QueueDownloadAsync
InspectPackageAsync
ValidateManifest
ExportPackageAsync
```

A method should generally do one domain-level job. UI event handlers should delegate to application commands/services rather than contain business logic.

## Immediate next work after documentation

Before implementation, add two small decision documents:

1. `04-ui-structure.md` — concrete screen/state layout.
2. `05-technical-decisions.md` — target .NET version, UI framework, persistence approach, HTTP stack, packaging strategy.

Once those are approved, begin Phase 1 repository foundation.
