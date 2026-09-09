# 01 — Current-State Baseline

## Source of this document

This baseline is reconstructed from static analysis of the supplied `BlueCoin_2.4.exe`. The executable was inspected without executing it. The SearchNow repository itself was empty when this branch was created.

Because this is reverse-engineered behavioral documentation, every implementation assumption must later be verified against source code or controlled runtime tests.

## Application profile

The inspected application is a Windows x64 .NET 7 WinForms application distributed as a single-file bundle. Its embedded application assembly uses Newtonsoft.Json and NLog.

The application is strongly coupled to Minecraft Bedrock / Minecraft for Windows data, Marketplace metadata, local package content, and PlayFab-backed catalog operations.

## Reconstructed startup flow

High-level behavior observed from the executable:

```text
Launch
  ↓
Read local configuration
  ↓
Locate Minecraft Bedrock data
  ↓
Read local Minecraft/account-content metadata
  ↓
Initialize local/remote content-key database behavior
  ↓
Authenticate catalog session
  ↓
Load/search Marketplace catalog
  ↓
Present content in WinForms UI
```

Several network and local-data operations appear to happen automatically during startup. This is a usability and privacy problem because the application does not clearly separate user intent from background initialization.

## Main functional areas found

### 1. Minecraft installation discovery

The application attempts to locate Minecraft Bedrock data under the Windows UWP package directory and reads Minecraft configuration/content files.

Observed responsibilities include:

- detecting Minecraft installation data;
- reading `options.txt`;
- discovering local content metadata;
- reading entitlement-related files;
- identifying locally available content.

### 2. Catalog / Marketplace browser

The application creates a PlayFab-backed session and uses catalog endpoints to search and inspect Minecraft Marketplace entries.

The current UI appears technically oriented, exposing identifiers such as Marketplace UUIDs and other internal metadata that are not useful to normal users.

### 3. Content download

The application can retrieve catalog metadata, resolve downloadable assets, download packages, and process them into Minecraft-compatible package formats.

The inspected build distinguishes between a normal processed download flow and an encrypted/raw download mode.

### 4. Package inspection and conversion

The application understands common Minecraft Bedrock package structures and metadata such as:

- `manifest.json`;
- resource packs;
- behavior packs;
- worlds/templates;
- skin/persona content;
- add-on combinations.

It can package processed content into formats such as `.mcpack`, `.mcaddon`, `.mctemplate`, `.mcpersona`, or `.zip` depending on detected content type.

### 5. Logging / local database behavior

The build contains NLog and maintains a local key/database workflow. It can retrieve a remote `keys.tsv` database and maintain a backup copy.

The existing design mixes infrastructure details directly into the application flow instead of isolating them behind explicit services.

## Important privacy finding

Static analysis indicates that entitlement-derived content identifiers and content-key material can be transmitted to a third-party backend automatically during startup.

This behavior must **not** be preserved as an implicit action in SearchNow.

Target rule:

> Local/account-derived data stays local by default. Any external data transfer must be explicit, documented, optional, and visible to the user.

## Important product/safety boundary

The old application also contains protected-content decryption and restriction-bypass behavior.

SearchNow documentation and implementation should not treat bypassing Marketplace DRM, converting paid content into free content, or distributing decryption keys as product requirements.

Supported modernization scope should instead focus on legitimate content-management capabilities such as:

- catalog browsing;
- owned/local content discovery;
- downloading content through permitted paths;
- package inspection;
- manifest validation;
- local pack management;
- BP/RP pairing;
- package export/conversion where the user has legitimate access to the content;
- diagnostics and recovery.

## Current UX problems

The existing application appears to expose implementation details rather than user tasks.

Primary issues:

1. Too much happens automatically during startup.
2. Privacy-sensitive operations are not clearly surfaced.
3. Internal IDs and keys are presented as first-class UI concepts.
4. Download/process stages are not explained as a coherent workflow.
5. Technical errors can leak directly into the user experience.
6. Catalog, download, processing, and export responsibilities are mixed.
7. Code naming and responsibilities suggest an experimental rather than maintainable architecture.
8. There is no clear separation between normal user mode and developer/diagnostic information.

## Baseline conclusion

The useful idea is not the existing UI. The useful core concept is a Minecraft Bedrock content-management client that can discover content, browse a catalog, manage downloads, validate packages, and export legitimate local content through a clear workflow.

SearchNow should therefore be treated as a structured redesign rather than a cosmetic reskin of the old executable.
