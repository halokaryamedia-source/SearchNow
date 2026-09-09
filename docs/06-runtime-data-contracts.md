# 06 — Legacy Runtime Data Contracts

> Reconstructed from static analysis of `BlueCoin_2.4.exe` / `BlueCoin2.dll`. This file documents **what data crosses component boundaries** in the legacy application so SearchNow can redesign those boundaries deliberately.

## 1. Scope

This document records:

- local filesystem inputs;
- application configuration;
- in-memory state;
- catalog/network interfaces;
- temporary/download/output files;
- package metadata structures;
- legacy sensitive-data paths;
- ownership boundaries that SearchNow should introduce.

Secret values found in the legacy executable are intentionally not reproduced.

---

# 2. Local Minecraft inputs

Legacy Minecraft discovery is rooted at:

```text
%LOCALAPPDATA%/Packages/
└── Microsoft.MinecraftUWP_8wekyb3d8bbwe/
    └── LocalState/
        └── games/
            └── com.mojang/
                └── minecraftpe/
```

Observed direct inputs:

```text
options.txt
*.ent
```

## 2.1 `options.txt`

Observed keys read by `McCrypt.Keys.ReadOptionsTxt`:

```text
last_minecraft_id
last_title_account_id
```

Associated in-memory fields:

```text
lastMinecraftId
lastTitleAccountId
lastDeviceId
```

The legacy implementation mixes identity/account-derived data with content-key processing. SearchNow should isolate local installation/account metadata behind a read-only discovery interface.

Suggested target boundary:

```text
IMinecraftInstallationLocator
IMinecraftLocalMetadataReader
```

---

## 2.2 entitlement files (`*.ent`)

Observed JSON/data field names inside the legacy entitlement path:

```text
EntityId
ReceiptData
DeviceId
Entitlements
EntitlementReceipts
Receipt
FriendlyId
PackId
ContentKey
```

Observed processing methods:

```text
ReadEntitlementFile
decryptEntitlementFile
readReceipt
readInnerReceipt
handleEntitlements
deriveUserKey
deriveEntKey
deriveContentKey
```

This path produces/updates legacy content ID/key state.

SearchNow rule:

> Do not treat entitlement-derived decryption keys as a normal product data contract. Owned/local-content discovery must be designed around supported metadata and permitted access paths.

---

# 3. Legacy application storage

Observed application data root:

```text
%APPDATA%/BlackMarketplace/
```

Observed files:

```text
noire.conf
keys.tsv
keys.tsv.bak
logs.txt
```

## 3.1 `noire.conf`

Observed default values embedded in the binary:

```text
UiMode:Dark
Logging:False
ExpertMode:False
FetchKeysTsv:True
```

Observed access API:

```text
Config.initConfig
Config.GetConfValue
Config.WriteConfValue
Config.replaceConfValue
```

Observed settings consumed by UI/infrastructure:

```text
UiMode
Logging
ExpertMode
FetchKeysTsv
```

Potential additional connection/auth configuration is read by PlayFab code.

### Target SearchNow contract

Replace stringly typed configuration with a typed, versioned settings model:

```text
AppSettings
├── Appearance
├── Paths
├── Downloads
├── Privacy
├── Diagnostics
└── Advanced
```

Required properties of the new settings layer:

- schema version;
- safe defaults;
- atomic save;
- validation;
- migration support;
- no secrets stored as plain app constants;
- no hidden network opt-ins.

---

## 3.2 `keys.tsv`

Legacy role:

```text
remote shared key database
  ↓
FetchKeysTsv
  ↓
keys.tsv
  ↓
LoadDb
  ↓
KeysTsv.Items
  ↓
LookupKey(ContentId)
```

Observed `KeysTsv` field/schema concepts:

```text
ContentId
KeyId
KeyType
KeyData
```

Observed backup behavior:

```text
keys.tsv
  ↓ update
keys.tsv.bak
```

This database is part of the protected-content legacy architecture and should not become a SearchNow core dependency.

---

## 3.3 logs

Observed logger filename:

```text
logs.txt
```

Logging backend:

```text
NLog 5.3.2
```

Observed enable flag:

```text
Logging
```

Target logging rules:

- redact tokens and account identifiers;
- do not log secret/key material;
- bounded retention;
- distinguish user-facing errors from diagnostic details;
- diagnostics opt-in where appropriate.

---

# 4. Temporary workspace / output

Observed temporary root concept:

```text
%TEMP%
```

Observed helper methods:

```text
CreateTemporaryFile
CreateTemporaryFolder
DeleteTempFiles
MoveDirectory
EscapeFilename
```

Observed final output root concept:

```text
output/
```

Observed output formats:

```text
.zip
.mcpack
.mcaddon
.mctemplate
.mcpersona
```

Observed classification-to-output concepts:

```text
resource → .mcpack
data     → .mcpack
persona  → .mcpersona
world    → .mctemplate
addon    → .mcaddon
unknown  → .zip
```

Add-on composition recognizes:

```text
BP
RP
```

Target SearchNow boundary:

```text
WorkspaceService
PackageInspector
PackageExporter
```

No UI event handler should own temp directory lifecycle directly.

---

# 5. Minecraft package inputs / metadata

Observed package metadata/files:

```text
manifest.json
signatures.json
contents.json
content.zipe
skins.json
level.dat
pack_icon.png
```

Observed directory concepts:

```text
subpacks/
behavior_packs/
resource_packs/
db/
```

Observed manifest concepts:

```text
modules
type
header
name
uuid
```

Observed language-name fallback concepts:

```text
texts/en_US.lang
pack.name
```

Observed helper path:

```text
Manifest.ReadType
Manifest.ReadName
Manifest.ReadUUID
Manifest.ChangeUUID
Manifest.SignManifest
```

SearchNow should treat package inspection as a read-only operation by default and make any transformation/export operation explicit.

---

# 6. Legacy content JSON structures

Minimal structs recovered from metadata:

## 6.1 content/key record

```text
content
├── FriendlyId
└── ContentKey
```

## 6.2 outbound key JSON record

```text
keysJsonStruct
├── id
└── contentKey
```

## 6.3 signature entry

```text
signatureBlock
├── hash
└── path
```

## 6.4 contents manifest

```text
contentsJson
├── version
└── content
```

## 6.5 file key mapping

```text
contentKeys
├── key
└── path
```

## 6.6 content path entry

```text
content
└── path
```

The naming is too generic for a maintained codebase. SearchNow should introduce explicit DTOs and domain models.

---

# 7. PlayFab network contract

Legacy PlayFab host construction:

```text
https://<title-id>.playfabapi.com
```

Observed routes:

```text
/Catalog/Search
/Catalog/GetPublishedItem
/Authentication/GetEntityToken
/Client/GetTitlePublicKey
/Client/LoginWithCustomID
```

Observed request/header concepts:

```text
POST
Content-Type: application/json
Accept: */*
Accept-Language: en-US
Accept-Encoding: gzip, deflate, br
x-playfab-signature
x-playfab-timestamp
x-entitytoken
User-Agent
```

Observed catalog request fields/concepts:

```text
count
filter
orderBy
creationDate DESC
top
skip
ItemId
ETag
```

Observed login/session concepts:

```text
TitleId
TitleSharedSecret
CustomId
PlayerSecret
EncryptedRequest
CreateAccount
ClientSecret
PlayFabId
EntityToken
Entity
master_player_account
```

The binary also contains an embedded title secret. Its value is intentionally not retained here.

### SearchNow rule

If a catalog integration is retained, it must be implemented as a typed adapter with supported authentication and **no client-embedded shared secret**.

Suggested boundary:

```text
ICatalogClient
IAuthSessionProvider
```

---

# 8. Catalog search model

Observed content tags/types used to build catalog filters:

```text
MarketplaceDurableCatalog_V1.2
PersonaDurable
hidden_offer
worldtemplate
resourcepack
mashup
skinpack
has_skinpack
addon
earth_achievement
```

Observed legacy `Filter` enum:

```text
None
FriendlyId
UUID
Template
ResourcePacks
MashUp
Skins
Persona
Addon
HiddenOffer
McEarth
HaveKey
```

Observed result/display concepts:

```text
Title
Description
Market UUID
UUID
Type
Key
```

Observed dynamic JSON fields:

```text
Id
DisplayProperties
packIdentity
Tags
```

Target SearchNow result model should not expose implementation-only IDs/keys as primary UI data.

Suggested model:

```text
CatalogItem
├── Id
├── Title
├── Description
├── ContentType
├── Thumbnail
├── Ownership/Availability state
└── TechnicalMetadata (advanced only)
```

---

# 9. Product/download metadata contract

Observed product/content concepts:

```text
Contents
Item
Type
Url
```

Observed asset type labels:

```text
skinbinary
personabinary
resourcebinary
```

Legacy pipeline:

```text
Catalog Item ID
  ↓
PlayFab.GetProductInformation
  ↓
content list / asset metadata
  ↓
URL resolution
  ↓
FileDownloader
```

Target model:

```text
DownloadAsset
├── AssetId
├── Kind
├── Uri
├── ExpectedSize? 
└── IntegrityMetadata?
```

`Uri` should be infrastructure-layer data, not persisted as long-lived domain state unless needed.

---

# 10. Download progress contract

Recovered `FileDownloader` state:

```text
Finished
DownloadedBytes
TotalBytes
Percentage
```

Events/callbacks:

```text
Wc_DownloadProgressChanged
Wc_DownloadFileCompleted
```

Target SearchNow job state should be explicit:

```text
Queued
Resolving
Downloading
Validating
Extracting
Inspecting
Packaging
Completed
Failed
Cancelled
```

With progress payload:

```text
DownloadedBytes
TotalBytes?
Percent?
CurrentStage
CurrentFile?
Error?
```

---

# 11. Third-party BlueCoin backend contract

Observed host:

```text
mc-prod.vercel.app
```

Two legacy opaque endpoints are embedded as Base64 strings.

Observed roles:

```text
endpoint A
  ← UploadString(JSON produced by ExportKeysJson2)

endpoint B
  → DownloadString/File request for keys.tsv
```

Observed outbound JSON shape includes:

```text
id
contentKey
```

This is a sensitive legacy integration. Exact opaque paths are intentionally omitted because SearchNow should not depend on or recreate the shared protected-content key service.

Target behavior:

```text
local account/content metadata
  X no implicit third-party upload
```

Any future telemetry or sync capability must be a separate, explicit feature with its own documented schema and consent model.

---

# 12. In-memory global/static state

The legacy architecture holds substantial process-wide mutable state.

Examples:

```text
PlayFab:
  PUBLIC_KEY
  ENTITY_TOKEN
  ENTITY_ID
  ENTITY_TYPE

Keys:
  lastTitleAccountId
  lastMinecraftId
  lastDeviceId
  contentList

KeysTsv:
  marketplaceItems

Url:
  marketplaceItems

DownloadForm:
  _searchFor
  _lastSearch
  _skip
  _searching
  _filterChanged
  filteredItems
```

Consequences:

- difficult unit testing;
- implicit lifecycle;
- race/concurrency risk;
- hard cancellation;
- hidden coupling;
- difficult multi-window/multi-job support.

SearchNow rule: state ownership must be explicit and scoped.

---

# 13. Target ownership map

```text
AppSettingsService
  owns persistent user settings

MinecraftDiscoveryService
  owns installation/local metadata discovery

CatalogSession
  owns auth/session lifetime

CatalogClient
  owns catalog requests

DownloadManager
  owns queue and job lifecycle

DownloadJob
  owns per-item progress/state

WorkspaceService
  owns temp files/directories

PackageInspector
  owns read-only pack interpretation

PackageExporter
  owns explicit output creation

DiagnosticsService
  owns logs/redaction

UI state
  owns selection/filter/navigation only
```

This ownership map is the minimum separation needed to prevent the legacy `DownloadForm` god-object architecture from reappearing.

---

# 14. Data-boundary rule for the rewrite

Every SearchNow feature should answer four questions in documentation before implementation:

1. **Input:** what exact data enters this component?
2. **Output:** what exact data leaves it?
3. **Owner:** which component owns lifecycle/persistence?
4. **Side effects:** which filesystem/network actions can occur?

If those four points are unclear, the component is not ready to implement.
