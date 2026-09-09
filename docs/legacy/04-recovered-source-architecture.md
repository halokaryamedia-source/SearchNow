# 04 — Recovered Source Architecture

> Status: **observed from static analysis** of `BlueCoin_2.4.exe` / embedded `BlueCoin2.dll`.
>
> This is not the original source tree. It is a source-level reconstruction based on CLR metadata, embedded bundle metadata, symbols, constants, resources, and internal call relationships. Names shown below are names that exist in the inspected assembly unless explicitly marked as inferred.

## 1. Purpose

This document freezes the architecture of the legacy application before SearchNow is redesigned. It exists so later refactoring does not accidentally lose useful behavior or preserve legacy coupling without understanding it.

The legacy application combines five concerns in one desktop process:

1. WinForms UI and settings.
2. Minecraft Bedrock local-data discovery.
3. Marketplace / PlayFab catalog communication.
4. Download and temporary-file management.
5. Legacy protected-content/key-processing functionality.

SearchNow should use this document as a compatibility reference, **not** as an instruction to reproduce protected-content bypass behavior.

---

## 2. Binary / runtime architecture

Inspected file:

```text
BlueCoin_2.4.exe
SHA-256: 1ff4cca4208e051819479f293c2c474d1a0638a11ff206b64e7bdf2580f2c365
Size: 4,702,510 bytes
Platform: Windows x64 GUI
Runtime: .NET 7
Target: net7.0 / win-x64
UI framework: Windows Forms
Distribution: .NET single-file bundle, bundle format 6.0
```

Recovered single-file bundle manifest:

```text
BlueCoin_2.4.exe
├── BlueCoin2.deps.json            1,600 bytes
├── BlueCoin2.runtimeconfig.json     372 bytes
├── Newtonsoft.Json.dll        1,830,912 bytes
├── NLog.dll                   2,441,216 bytes
└── BlueCoin2.dll                266,240 bytes
```

Explicit NuGet/runtime dependencies:

```text
Newtonsoft.Json 13.0.1
NLog            5.3.2
Microsoft.NETCore.App         7.0.0
Microsoft.WindowsDesktop.App 7.0.0
```

Native interop observed:

```text
dwmapi.dll
└── DwmSetWindowAttribute(...)
```

The P/Invoke is used by the theme layer for immersive dark-mode handling.

---

## 3. CLR metadata inventory

Recovered from `BlueCoin2.dll`:

```text
TypeDef   : 60
MethodDef : 186
Fields    : 391
```

The raw counts include compiler-generated closures/dynamic call-site helpers. The meaningful source-authored architecture is concentrated in these groups:

```text
McCrypt.*
BlackMarketplace.*
NewProgressBar
Filter
legacy data structs / DTO-like types
```

Compiler-generated types such as `<>c`, `<>o__*`, and `<>c__DisplayClass*` are implementation artifacts and should not become first-class SearchNow architecture.

---

## 4. Reconstructed source tree

The following layout is the closest source-level representation of the assembly structure.

```text
BlueCoin2/
├── Program.cs
├── ApplicationConfiguration.cs
├── DownloadForm.cs
├── DownloadForm.Designer.cs          # inferred split; InitializeComponent is present
├── NewProgressBar.cs
├── FileDownloader.cs
├── Config.cs
├── Logger.cs
├── Themes.cs
├── ColorScheme.cs
├── McClient.cs
├── PlayFab.cs
├── KeysTsv.cs
├── Url.cs
├── Filter.cs
│
├── McCrypt/
│   ├── Crypto.cs
│   ├── Keys.cs
│   ├── Manifest.cs
│   ├── Marketplace.cs
│   └── Utils.cs
│
└── LegacyModels/
    ├── content                  # FriendlyId, ContentKey
    ├── keysJsonStruct           # id, contentKey
    ├── signatureBlock           # hash, path
    ├── contentsJson             # version, content
    ├── contentKeys              # key, path
    └── content                  # path
```

`DownloadForm.Designer.cs` is an inferred source split because the compiled class contains a standard WinForms `InitializeComponent()` method and generated component fields. The original project may have used a different physical file layout.

---

# 5. Namespace: `McCrypt`

This namespace is the legacy content-processing subsystem. It is highly coupled to key derivation, entitlement data, package metadata, cryptography, and Marketplace package processing.

## 5.1 `McCrypt.Crypto`

Base type: `System.Object`

Observed methods:

```text
Sha256
Aes256CfbEncrypt
Aes256CfbDecrypt
.ctor
```

Responsibilities:

- SHA-256 hashing.
- AES-256 CFB encryption/decryption primitives.
- Low-level crypto utility used by entitlement, manifest, and package-processing code.

Observed internal consumers:

```text
McCrypt.Manifest
McCrypt.Marketplace
McCrypt.Keys
```

SearchNow rule: cryptographic helpers, where legitimately needed, should live behind narrow interfaces and must not be coupled directly to UI code.

---

## 5.2 `McCrypt.Keys`

Base type: `System.Object`

Observed fields:

```text
rng
KeyDbFile
lastTitleAccountId
lastMinecraftId
lastDeviceId
contentList
```

Observed methods:

```text
ExportKeysJson
ExportKeysJson2
deriveUserKey
GenerateKey
deriveEntKey
deriveContentKey
AddKey
handleEntitlements
readInnerReceipt
readReceipt
ReadOptionsTxt
padLastTileAccountId
decryptEntitlementFile
ReadEntitlementFile
ReadKeysDb
LookupKey
.ctor
.cctor
```

Observed responsibility chain:

```text
ReadOptionsTxt
  └─ reads Minecraft identity-related values from options.txt

ReadEntitlementFile
  ├─ decryptEntitlementFile
  │   ├─ padLastTileAccountId
  │   ├─ deriveEntKey
  │   ├─ ForceDecodeBase64
  │   └─ Marketplace.decryptEntitlementBuffer
  ├─ JsonDecodeCloserToMinecraft
  ├─ ForceDecodeBase64
  └─ readReceipt
      ├─ JsonDecodeCloserToMinecraft
      └─ readInnerReceipt
          └─ deriveUserKey

handleEntitlements
  ├─ deriveContentKey
  └─ AddKey
      └─ LookupKey
```

This is a major legacy coupling point: identity parsing, entitlement parsing, key derivation, local key database access, and export are all combined in one class.

SearchNow migration rule: do **not** carry this class across as one unit. Separate local Minecraft discovery/owned-content metadata from any legacy protected-content mechanism.

---

## 5.3 `McCrypt.Manifest`

Observed methods:

```text
SignManifestString
SignManifest
ReadType
ReadName
ReadUUID
ChangeUUID
.ctor
```

Observed internal relationships:

```text
SignManifestString -> Crypto.Sha256
SignManifest       -> Crypto.Sha256
ReadName           -> Utils.TrimName
```

Responsibilities inferred from symbols/constants:

- reading Minecraft `manifest.json` metadata;
- determining pack/content type;
- reading display name and UUID;
- changing UUID values;
- computing/signing manifest-related hashes.

SearchNow target replacement:

```text
ManifestReader
ManifestValidator
PackIdentityService
```

These should be separate from protected-content processing.

---

## 5.4 `McCrypt.Marketplace`

Observed field:

```text
dontEncrypt
```

Observed methods:

```text
shouldEncrypt
CrackLevelDat
CrackSkinsJson
CrackZipe
EncryptContents
EncryptContents
decryptEntitlementBuffer
worldOrContentsJsonDecrypt
decryptContentsJsonFiles
DecryptContents
.ctor
.cctor
```

Important internal relationships:

```text
EncryptContents
  ├─ Keys.GenerateKey
  ├─ EncryptContents       # recursive
  ├─ shouldEncrypt
  ├─ Utils.IsDirectory
  ├─ Crypto.Aes256CfbEncrypt
  └─ Utils.WriteString

decryptEntitlementBuffer
  └─ Crypto.Aes256CfbDecrypt

worldOrContentsJsonDecrypt
  ├─ Utils.ReadString
  ├─ Keys.LookupKey
  └─ Crypto.Aes256CfbDecrypt

decryptContentsJsonFiles
  ├─ Utils.JsonDecodeCloserToMinecraft
  └─ Crypto.Aes256CfbDecrypt via generated closure

DecryptContents
  ├─ worldOrContentsJsonDecrypt
  ├─ decryptContentsJsonFiles
  └─ DecryptContents       # recursive

CrackLevelDat
  └─ Utils.FindData
```

Observed package/file concepts:

```text
contents.json
content.zipe
level.dat
skins.json
subpacks/
behavior_packs/
resource_packs/
db/
```

Legacy methods named `Crack*`, protected-content decryption, restriction changes, and key use are recorded here only to preserve an accurate architecture history. They are **not SearchNow implementation requirements**.

Safe functionality worth extracting conceptually:

- recursive package traversal;
- content-type recognition;
- manifest inspection;
- path handling;
- validation;
- package assembly for legitimately accessible files.

---

## 5.5 `McCrypt.Utils`

Observed methods:

```text
JsonDecodeCloserToMinecraft
IsDirectory
FindData
ReadString
WriteString
ForceDecodeBase64
TrimName
.ctor
```

Responsibilities:

- Minecraft-oriented JSON decoding helper;
- file/directory discrimination;
- binary search/helper behavior;
- binary string read/write;
- Base64 normalization/decoding;
- display-name trimming.

This class is a generic utility bucket. SearchNow should avoid reproducing a catch-all `Utils` class; helpers should move to the component that owns the behavior.

---

# 6. Namespace: `BlackMarketplace`

This namespace contains the WinForms shell, application configuration, catalog communication, download orchestration, and UI infrastructure.

## 6.1 `BlackMarketplace.Program`

Observed method:

```text
Main
```

Observed startup chain:

```text
Program.Main
  ├─ ApplicationConfiguration.Initialize
  └─ new DownloadForm
```

This is the application entry point.

---

## 6.2 `BlackMarketplace.ApplicationConfiguration`

Observed method:

```text
Initialize
```

Role: standard WinForms process/UI initialization generated by modern .NET WinForms templates.

---

## 6.3 `BlackMarketplace.DownloadForm`

Base type: `System.Windows.Forms.Form`

This is the dominant legacy class and the main architectural problem: UI, startup sequencing, catalog logic, local file handling, download orchestration, processing, and export are concentrated here.

Observed state/data fields:

```text
logger
_searchFor
_lastSearch
_skip
_searching
_filterChanged
filteredItems
components
```

Observed UI fields:

```text
dl
progress
search
options
status
comboBox1
checkBox1
dlE
Download
Title
Description
UUID
Type
Key
tableLayoutPanel1
button1
KeysUpdateCheckBox
```

Observed methods:

```text
SetUiTheme
SetExpertMode
SetKeysFetchCheckbox
.ctor
LoadDb
GetCatPictures
FetchKeysTsv
InitFilterBox
OldConfigCheck
DownloadForm_Load
ClearUnchecked
DoSearch
DeleteTempFiles
CreateTemporaryFile
CreateTemporaryFolder
EscapeFilename
MoveDirectory
DownloadSelected
dlE_Click
dl_Click
search_KeyPress
options_Scroll
options_SelectionChanged
options_CellContentClick
comboBox1_SelectedIndexChanged
search_TextChanged
checkBox1_CheckedChanged
progress_Click
tableLayoutPanel1_Paint
tableLayoutPanel1_Paint_1
button1_Click_1
KeysUpdateCheckBox_CheckedChanged
Dispose
InitializeComponent
.cctor
```

### Constructor chain

```text
DownloadForm..ctor
  ├─ OldConfigCheck
  ├─ InitializeComponent
  ├─ SetKeysFetchCheckbox
  ├─ SetUiTheme
  └─ SetExpertMode
```

### Form-load startup chain

Observed generated load worker calls:

```text
DownloadForm_Load
  └─ startup worker
      ├─ GetCatPictures
      │   ├─ McCrypt.Keys.ReadOptionsTxt
      │   ├─ McCrypt.Keys.ReadEntitlementFile
      │   ├─ McCrypt.Keys.ExportKeysJson2
      │   └─ McClient
      ├─ InitFilterBox
      ├─ FetchKeysTsv
      ├─ LoadDb
      ├─ PlayFab.PullEntityTokenOutOfMyAss
      └─ DoSearch
```

This confirms that several local-data and network operations are embedded directly in UI startup.

### Search chain

```text
DoSearch
  ├─ build/filter query
  ├─ PlayFab.Search
  ├─ project dynamic response objects
  ├─ apply local filtering/key state
  └─ populate DataGridView
```

UI events that trigger `DoSearch` include:

```text
search_KeyPress
options_Scroll
comboBox1_SelectedIndexChanged
```

### Download chain

`DownloadSelected` is the central orchestration method.

Observed internal calls:

```text
DownloadSelected
  ├─ KeysTsv.LookupKey
  ├─ McCrypt.Keys.AddKey
  ├─ PlayFab.GetProductInformation
  ├─ CreateTemporaryFolder
  ├─ CreateTemporaryFile
  ├─ FileDownloader
  ├─ McCrypt.Marketplace.DecryptContents
  ├─ McCrypt.Marketplace.CrackLevelDat
  ├─ McCrypt.Marketplace.CrackSkinsJson
  ├─ McCrypt.Manifest.ReadType
  ├─ McCrypt.Manifest.ReadName
  ├─ EscapeFilename
  ├─ MoveDirectory
  └─ DeleteTempFiles
```

The method also has many compiler-generated lambda helpers (`<DownloadSelected>b__*`), confirming that a large multi-stage workflow is implemented inside this single UI class.

### Download button distinction

Observed event mapping:

```text
dl_Click  -> DownloadSelected(...)
dlE_Click -> DownloadSelected(...)
```

The UI labels indicate two flows:

```text
Download
Download Encrypted
```

The exact boolean direction has already been captured in the current-state investigation, but SearchNow should replace this with explicit user-facing states rather than exposing encryption terminology as a primary action.

---

## 6.4 `BlackMarketplace.FileDownloader`

Observed fields:

```text
Finished
DownloadedBytes
TotalBytes
Percentage
```

Observed methods:

```text
.ctor
download
Wc_DownloadFileCompleted
Wc_DownloadProgressChanged
```

Observed chain:

```text
FileDownloader..ctor
  └─ download
      ├─ McClient
      ├─ Wc_DownloadProgressChanged
      └─ Wc_DownloadFileCompleted
```

This is a thin asynchronous download wrapper around `WebClient`-style infrastructure.

SearchNow target: replace with `HttpClient`, cancellation tokens, a queue, explicit state transitions, retries, and testable progress reporting.

---

## 6.5 `BlackMarketplace.KeysTsv`

Observed static/data fields:

```text
marketplaceItems
ContentId
KeyId
KeyType
KeyData
```

Observed methods:

```text
get_Items
LookupKey
ClearKeysList
.ctor
.cctor
```

Role: parses/holds the legacy `keys.tsv` dataset and resolves content IDs to stored key material.

SearchNow rule: do not make this legacy shared-key database part of the modern product core.

---

## 6.6 `BlackMarketplace.PlayFab`

Observed fields:

```text
logger
PLAYFAB_VERSION
CPP_REST_VERSION
LIB_HTTP_CLIENT_VERSION
TITLE_ID
TITLE_SECRET
PUBLIC_KEY
ENTITY_TOKEN
ENTITY_ID
ENTITY_TYPE
```

Observed methods:

```text
GetProductInformation
Search
POST
RefreshEntityTokenBullshit
SdkMsg
GenerateClientSecret
GenerateCustomId
GetPlayfabApiUrl
GetPublicKeyAndMicrosoftTakesABigL
EncryptCustomIdLoginToken
LoginWithCustomId
EncryptRequest
PullEntityTokenOutOfMyAss
.ctor
.cctor
```

Names are recorded verbatim because they exist in the assembly. They should be replaced with professional names if behavior is reimplemented.

Observed high-level authentication chain:

```text
PullEntityTokenOutOfMyAss
  ├─ Config.GetConfValue
  ├─ GetPublicKeyAndMicrosoftTakesABigL
  ├─ LoginWithCustomId
  │   ├─ GenerateClientSecret
  │   ├─ GenerateCustomId
  │   ├─ EncryptCustomIdLoginToken
  │   └─ Config.WriteConfValue
  └─ RefreshEntityTokenBullshit
```

Observed API responsibilities:

```text
POST
  ├─ McClient
  ├─ GetPlayfabApiUrl
  └─ SdkMsg

Search
  └─ Marketplace catalog search

GetProductInformation
  └─ published item metadata / asset metadata
```

Observed PlayFab routes/constants include catalog search, published-item lookup, entity-token retrieval, public-key retrieval, and custom-ID login.

### Security finding

The legacy binary contains title/authentication constants, including a title secret, directly in executable strings. **The secret value is intentionally not copied into this repository.**

SearchNow rule:

- no shared secret embedded in desktop binaries;
- treat any credential found in BlueCoin as compromised;
- rotate/revoke legacy credentials if they are still active;
- use supported authentication flows and server-side secrets only where appropriate.

---

## 6.7 `BlackMarketplace.McClient`

Base type: `System.Net.WebClient`

Observed methods:

```text
GetWebRequest
.ctor
```

Role: customizes outgoing WebClient requests, including headers/decompression behavior.

SearchNow target: replace with a centrally configured `HttpClient` / typed client implementation.

---

## 6.8 `BlackMarketplace.Config`

Observed fields:

```text
DataFolder
ConfigFile
```

Observed methods:

```text
initConfig
replaceConfValue
GetConfValue
WriteConfValue
.ctor
.cctor
```

Observed call relationships:

```text
replaceConfValue -> initConfig
GetConfValue      -> initConfig
WriteConfValue    -> initConfig
WriteConfValue    -> GetConfValue
WriteConfValue    -> replaceConfValue
```

Observed legacy storage concepts:

```text
%APPDATA%/BlackMarketplace/
noire.conf
```

Observed defaults include:

```text
UiMode = Dark
Logging = False
ExpertMode = False
FetchKeysTsv = True
```

SearchNow target: typed settings model, versioned schema, atomic writes, explicit privacy defaults.

---

## 6.9 `BlackMarketplace.Logger`

Observed fields:

```text
LoggerFile
LOGGER_ENABLED
```

Observed methods:

```text
InitLogger
.ctor
.cctor
```

NLog is the logging backend. Logging enablement is read from legacy configuration.

Observed filename:

```text
logs.txt
```

SearchNow target: structured diagnostic logging, automatic redaction of identifiers/tokens, bounded log retention.

---

## 6.10 `BlackMarketplace.Themes`

Observed fields:

```text
Dark
Light
DWMWA_USE_IMMERSIVE_DARK_MODE_BEFORE_20H1
DWMWA_USE_IMMERSIVE_DARK_MODE
```

Observed methods:

```text
ChangeTheme
DwmSetWindowAttribute
UseImmersiveDarkMode
IsWindows10OrGreater
.cctor
```

Observed chain:

```text
UseImmersiveDarkMode
  ├─ IsWindows10OrGreater
  └─ DwmSetWindowAttribute (dwmapi.dll)
```

Theme definitions are represented by `ColorScheme` objects.

---

## 6.11 `BlackMarketplace.ColorScheme`

Observed fields:

```text
PanelBG
PanelFG
ButtonBG
ButtonFG
TextBoxBG
TextBoxFG
ProgressBarBG
ProgressBarFG
ComboBoxBG
ComboBoxFG
CheckBoxBG
CheckBoxFG
DataGridViewBG
DataGridViewFG
DataGridHeadBG
DataGridHeadFG
```

Role: color palette container for WinForms controls.

---

## 6.12 `BlackMarketplace.Url`

Observed fields:

```text
marketplaceItems
ContentId
UrlId
UrlType
UrlData
```

Observed methods:

```text
get_Items
.ctor
.cctor
```

Role inferred from symbols: dynamic/static mapping of item IDs, asset URL IDs/types, and URL data returned from product information.

---

# 7. Global UI helper: `NewProgressBar`

Base type: `System.Windows.Forms.ProgressBar`

Observed methods:

```text
.ctor
OnPaintBackground
OnPaint
```

Role: custom-drawn WinForms progress bar.

This behavior is purely presentation-layer logic and should not contain download state in SearchNow.

---

# 8. Enum: `Filter`

Observed values:

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

This enum reveals the legacy search/filter model. Some names correspond directly to Marketplace content tags/types; `HaveKey` exposes a legacy implementation detail that should not survive as a normal-user filter.

---

# 9. Legacy data structs / DTO-like types

The assembly contains several minimal value types used for dynamic JSON/binary processing.

## `content`

```text
FriendlyId
ContentKey
```

## `keysJsonStruct`

```text
id
contentKey
```

## `signatureBlock`

```text
hash
path
```

## `contentsJson`

```text
version
content
```

## `contentKeys`

```text
key
path
```

## second `content` shape

```text
path
```

The duplicate generic naming is another indicator that the original source evolved experimentally. SearchNow should replace these with explicit immutable models such as `PackIdentity`, `ContentEntry`, `DownloadAsset`, and `ManifestEntry`.

---

# 10. WinForms UI composition recovered from fields

The legacy primary window contains at least the following user-facing concepts:

```text
Search box
Filter ComboBox
DataGridView results
Dark Mode checkbox
Auto update keys checkbox
Download button
Download Encrypted button
Progress bar
Status text
Open keys folder button
Expert Mode-dependent UI
```

Observed result columns/labels:

```text
Title
Description
Market UUID
UUID
Type
Key
```

This confirms that the legacy interface is organized around technical implementation data rather than user tasks.

---

# 11. Local filesystem architecture

Observed Minecraft discovery root:

```text
%LOCALAPPDATA%/
└── Packages/
    └── Microsoft.MinecraftUWP_8wekyb3d8bbwe/
        └── LocalState/
            └── games/
                └── com.mojang/
                    └── minecraftpe/
                        ├── options.txt
                        └── *.ent
```

Observed application storage concepts:

```text
%APPDATA%/BlackMarketplace/
├── noire.conf
├── keys.tsv
├── keys.tsv.bak
└── logs.txt
```

Observed processing/output concepts:

```text
%TEMP%/...      temporary extraction/download work
output/...      final package output
```

Observed package extensions:

```text
.zip
.mcpack
.mcaddon
.mctemplate
.mcpersona
```

Observed package classification concepts:

```text
resource
data
world
persona
addon
BP
RP
```

---

# 12. Network architecture

The legacy client communicates with two broad external systems.

## 12.1 Microsoft PlayFab / Minecraft catalog path

Observed high-level routes:

```text
/Catalog/Search
/Catalog/GetPublishedItem
/Authentication/GetEntityToken
/Client/GetTitlePublicKey
/Client/LoginWithCustomID
```

Observed dynamic API host pattern:

```text
https://<title-id>.playfabapi.com
```

The legacy client constructs and sends PlayFab requests itself and stores session/entity state in static fields on `PlayFab`.

## 12.2 Third-party BlueCoin backend

Observed host:

```text
mc-prod.vercel.app
```

Two opaque HTTPS endpoints are embedded in Base64 strings and used for:

1. uploading entitlement-derived content ID/key data;
2. retrieving the shared `keys.tsv` database.

The exact opaque paths and legacy credential material are intentionally not copied into this repository because they are not required for SearchNow architecture and would preserve a sensitive legacy integration.

---

# 13. Reconstructed runtime dependency graph

```text
Program
  ↓
DownloadForm
  ├──────────────→ Config
  ├──────────────→ Themes → ColorScheme / dwmapi
  ├──────────────→ Logger → NLog
  ├──────────────→ PlayFab → McClient → WebClient
  ├──────────────→ FileDownloader → McClient
  ├──────────────→ KeysTsv
  └──────────────→ McCrypt
                    ├── Keys
                    │   ├── Utils
                    │   └── Marketplace → Crypto
                    ├── Manifest → Crypto / Utils
                    ├── Marketplace → Crypto / Keys / Utils
                    └── Crypto
```

The key architectural defect is obvious from this graph: **`DownloadForm` is simultaneously UI, application service, workflow coordinator, file manager, network coordinator, and legacy content processor.**

---

# 14. Reconstructed startup sequence

```text
Program.Main
  ↓
ApplicationConfiguration.Initialize
  ↓
DownloadForm constructor
  ├─ legacy-config migration/check
  ├─ WinForms controls
  ├─ keys-update checkbox state
  ├─ theme
  └─ expert mode
  ↓
DownloadForm_Load
  ↓
Logger.InitLogger
  ↓
startup worker
  ├─ GetCatPictures
  │   ├─ ReadOptionsTxt
  │   ├─ ReadEntitlementFile
  │   └─ ExportKeysJson2 → third-party network path
  ├─ InitFilterBox
  ├─ FetchKeysTsv → third-party network path
  ├─ LoadDb
  ├─ PlayFab authentication/session initialization
  └─ DoSearch
  ↓
Main UI ready
```

The oddly named `GetCatPictures()` is therefore not a cosmetic method. It is part of local Minecraft metadata/key collection and network startup behavior.

---

# 15. Reconstructed catalog flow

```text
User search/filter action
  ↓
DownloadForm.DoSearch
  ↓
build PlayFab catalog filter
  ↓
PlayFab.Search
  ↓
POST /Catalog/Search
  ↓
dynamic JSON result
  ↓
filter/project items
  ↓
DataGridView rows
```

Observed catalog classifications include:

```text
worldtemplate
resourcepack
mashup
skinpack
PersonaDurable
addon
hidden_offer
earth_achievement
```

---

# 16. Reconstructed download/process flow

```text
Selected DataGridView rows
  ↓
DownloadSelected
  ↓
resolve item/key state
  ↓
PlayFab.GetProductInformation
  ↓
resolve downloadable content entries
  ↓
create temporary folders/files
  ↓
FileDownloader
  ↓
extract/process package
  ↓
legacy decrypt/restriction-processing path when enabled
  ↓
Manifest.ReadType / ReadName
  ↓
select output extension
  ↓
move/package output
  ↓
DeleteTempFiles
```

Observed downloadable asset labels:

```text
skinbinary
personabinary
resourcebinary
```

SearchNow should preserve the clean portions of this concept as a staged job pipeline while excluding protected-content bypass behavior.

---

# 17. Legacy architecture liabilities

## 17.1 God object

`DownloadForm` owns far too many responsibilities.

## 17.2 Static mutable state

`PlayFab`, `KeysTsv`, configuration, and key-related code rely heavily on static fields/state. This makes testing, concurrency, cancellation, and multiple sessions difficult.

## 17.3 Dynamic JSON coupling

Compiler-generated dynamic call-site types (`<>o__*`) are numerous, indicating heavy `dynamic` access to JSON objects. This makes schema changes fail at runtime rather than compile time.

## 17.4 Synchronous/legacy networking

`WebClient` and explicit event callbacks are used instead of modern `HttpClient` + task/cancellation patterns.

## 17.5 UI-driven orchestration

Workflow state is encoded in button event handlers and form methods rather than an application layer.

## 17.6 Hidden startup side effects

Local-data collection, network upload, key-database refresh, authentication, and first catalog search occur as part of form startup.

## 17.7 Embedded credential material

Authentication-related constants include secret material in the client binary. Treat it as compromised.

## 17.8 Weak naming

Examples retained verbatim from the binary:

```text
GetCatPictures
GetPublicKeyAndMicrosoftTakesABigL
RefreshEntityTokenBullshit
PullEntityTokenOutOfMyAss
CrackZipe
```

These names are evidence of an experimental codebase and should not propagate to SearchNow.

---

# 18. SearchNow architectural extraction map

Legacy component → modern responsibility:

| Legacy | SearchNow direction |
|---|---|
| `DownloadForm` | Views + ViewModels/Presenters + application use cases |
| `Config` | typed `SettingsService` |
| `McClient` | typed `HttpClient` infrastructure |
| `PlayFab` | `CatalogClient` / supported auth adapter |
| `FileDownloader` | `DownloadManager` + `DownloadJob` |
| `Manifest` | `ManifestReader` + `ManifestValidator` |
| `Utils` | eliminate bucket; move helpers to owning modules |
| `Logger` | structured diagnostics service |
| `Themes` / `ColorScheme` | UI theme service/design tokens |
| temporary-file helpers | `WorkspaceService` / `TempFileService` |
| package output logic | `PackageExporter` |
| legacy key/decrypt/crack subsystem | **do not port as product core** |

Recommended target dependency direction:

```text
UI
 ↓
Application
 ↓
Domain
 ↑
Infrastructure adapters
```

No UI class should directly call filesystem, PlayFab, cryptographic, or packaging implementation details.

---

# 19. Confidence labels

Use these labels in all later documentation:

- **Observed** — directly present in CLR metadata, strings, bundle metadata, or internal IL call relationships.
- **Reconstructed** — strongly derived from multiple observed facts, but physical source-file organization is unknown.
- **Inferred** — reasonable interpretation that must be checked in controlled runtime tests or original source if obtained.
- **Target** — proposed SearchNow architecture; not legacy behavior.

This document deliberately separates these categories so reverse-engineered facts do not become accidental requirements.

---

# 20. Freeze point

As of this documentation pass, the legacy architecture is sufficiently mapped to begin designing SearchNow without treating the binary as a black box.

Before implementation begins, the remaining documentation should reference this file rather than rediscovering the legacy architecture repeatedly.
