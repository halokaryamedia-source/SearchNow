# 05 — Recovered CLR Symbol Map

> Exact symbol inventory recovered from `BlueCoin2.dll` CLR metadata. This document complements [`04-recovered-source-architecture.md`](04-recovered-source-architecture.md).

## 1. Metadata summary

```text
TypeDef rows   60
MethodDef rows 186
Field rows     391
```

The assembly also contains standard compiler metadata tables, embedded resources, dynamic call sites, closures, and WinForms generated code.

---

# 2. Source-authored / meaningful types

## `NewProgressBar : System.Windows.Forms.ProgressBar`

Methods:

```text
.ctor
OnPaintBackground
OnPaint
```

---

## `McCrypt.Crypto`

Fields: none observed.

Methods:

```text
Sha256
Aes256CfbEncrypt
Aes256CfbDecrypt
.ctor
```

---

## `McCrypt.Keys`

Fields:

```text
rng
KeyDbFile
lastTitleAccountId
lastMinecraftId
lastDeviceId
contentList
```

Methods:

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

---

## `McCrypt.Manifest`

Fields: none observed.

Methods:

```text
SignManifestString
SignManifest
ReadType
ReadName
ReadUUID
ChangeUUID
.ctor
```

---

## `McCrypt.Marketplace`

Fields:

```text
dontEncrypt
```

Methods:

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

Note: two MethodDef rows are named `EncryptContents`, therefore the original source contains overloads.

---

## `McCrypt.Utils`

Fields: none observed.

Methods:

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

---

## `BlackMarketplace.ColorScheme`

Fields:

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

Methods:

```text
.ctor
```

---

## `BlackMarketplace.Config`

Fields:

```text
DataFolder
ConfigFile
```

Methods:

```text
initConfig
replaceConfValue
GetConfValue
WriteConfValue
.ctor
.cctor
```

---

## `BlackMarketplace.DownloadForm : System.Windows.Forms.Form`

Fields:

```text
logger
_searchFor
_lastSearch
_skip
_searching
_filterChanged
filteredItems
components
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

Methods:

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

Compiler-generated methods attached to this workflow:

```text
<InitFilterBox>b__15_0
<DownloadForm_Load>b__17_0
<DownloadForm_Load>b__17_1
<DownloadForm_Load>b__17_2
<DownloadForm_Load>b__17_3
<dlE_Click>b__26_0
<dl_Click>b__27_0
```

---

## `BlackMarketplace.FileDownloader`

Fields:

```text
Finished
DownloadedBytes
TotalBytes
Percentage
```

Methods:

```text
.ctor
download
Wc_DownloadFileCompleted
Wc_DownloadProgressChanged
```

---

## `BlackMarketplace.KeysTsv`

Fields:

```text
marketplaceItems
ContentId
KeyId
KeyType
KeyData
```

Methods:

```text
get_Items
LookupKey
ClearKeysList
.ctor
.cctor
```

---

## `BlackMarketplace.Logger`

Fields:

```text
LoggerFile
LOGGER_ENABLED
```

Methods:

```text
InitLogger
.ctor
.cctor
```

---

## `BlackMarketplace.McClient : System.Net.WebClient`

Methods:

```text
GetWebRequest
.ctor
```

---

## `BlackMarketplace.PlayFab`

Fields:

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

Methods:

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

The `TITLE_SECRET` value itself is intentionally excluded from repository documentation.

---

## `BlackMarketplace.Program`

Methods:

```text
Main
```

---

## `BlackMarketplace.Themes`

Fields:

```text
Dark
Light
DWMWA_USE_IMMERSIVE_DARK_MODE_BEFORE_20H1
DWMWA_USE_IMMERSIVE_DARK_MODE
```

Methods:

```text
ChangeTheme
DwmSetWindowAttribute
UseImmersiveDarkMode
IsWindows10OrGreater
.cctor
```

---

## `BlackMarketplace.Url`

Fields:

```text
marketplaceItems
ContentId
UrlId
UrlType
UrlData
```

Methods:

```text
get_Items
.ctor
.cctor
```

---

## `BlackMarketplace.ApplicationConfiguration`

Methods:

```text
Initialize
```

---

# 3. Legacy value types / data shapes

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

These generic names should not be copied into SearchNow.

---

# 4. `Filter` enum

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

---

# 5. Compiler / generated metadata types

The following are not architectural modules and should not be manually recreated.

## Compiler attributes

```text
Microsoft.CodeAnalysis.EmbeddedAttribute
System.Runtime.CompilerServices.NullableAttribute
System.Runtime.CompilerServices.NullableContextAttribute
System.Runtime.CompilerServices.RefSafetyRulesAttribute
```

## Private implementation data

```text
<PrivateImplementationDetails>
__StaticArrayInitTypeSize=28
```

## Dynamic call-site containers

Observed `dynamic` binder helper types:

```text
<>o__3
<>o__4
<>o__5
<>o__6
<>o__10
<>o__11
<>o__12
<>o__13
<>o__15
<>o__16
<>o__17
<>o__18
<>o__19
<>o__20
<>o__21
<>o__25
```

Some are associated with `DownloadForm`, others with `PlayFab` and McCrypt workflows. Their high field counts (`<>p__*`) indicate extensive runtime `dynamic` binding, primarily around JSON/object access.

## Closure/display classes

```text
<>c
<>c__DisplayClass12_0
<>c__DisplayClass12_1
<>c__DisplayClass13_0
<>c__DisplayClass13_1
<>c__DisplayClass19_0
<>c__DisplayClass25_0
<>c__DisplayClass25_1
<>c__DisplayClass25_2
<>c__DisplayClass25_3
```

Observed closure ownership from method names/call graph:

```text
DisplayClass12_* → Marketplace.decryptContentsJsonFiles
DisplayClass13_* → Marketplace.DecryptContents
DisplayClass19_0 → DownloadForm.DoSearch
DisplayClass25_* → DownloadForm.DownloadSelected
<>c             → LoadDb / MoveDirectory helpers
```

These types are generated by the C# compiler for lambdas and captured local state.

---

# 6. High-value internal call graph

## Startup

```text
Program.Main
  → ApplicationConfiguration.Initialize
  → DownloadForm..ctor

DownloadForm..ctor
  → OldConfigCheck
  → InitializeComponent
  → SetKeysFetchCheckbox
  → SetUiTheme
  → SetExpertMode

DownloadForm_Load worker
  → GetCatPictures
  → InitFilterBox
  → FetchKeysTsv
  → LoadDb
  → PlayFab.PullEntityTokenOutOfMyAss
  → DoSearch
```

## Local Minecraft / entitlement path

```text
GetCatPictures
  → Keys.ReadOptionsTxt
  → Keys.ReadEntitlementFile
  → Keys.ExportKeysJson2
  → McClient

Keys.ReadEntitlementFile
  → Keys.decryptEntitlementFile
  → Utils.JsonDecodeCloserToMinecraft
  → Utils.ForceDecodeBase64
  → Keys.readReceipt

Keys.handleEntitlements
  → Keys.deriveContentKey
  → Keys.AddKey
```

## Catalog path

```text
DownloadForm.DoSearch
  → PlayFab.Search

PlayFab.POST
  → McClient
  → PlayFab.GetPlayfabApiUrl
  → PlayFab.SdkMsg
```

## Authentication/session path

```text
PlayFab.PullEntityTokenOutOfMyAss
  → Config.GetConfValue
  → GetPublicKeyAndMicrosoftTakesABigL
  → LoginWithCustomId
  → RefreshEntityTokenBullshit

LoginWithCustomId
  → GenerateClientSecret
  → GenerateCustomId
  → EncryptCustomIdLoginToken
  → Config.WriteConfValue
```

## Download/process path

```text
DownloadForm.DownloadSelected
  → KeysTsv.LookupKey
  → Keys.AddKey
  → PlayFab.GetProductInformation
  → CreateTemporaryFolder
  → CreateTemporaryFile
  → FileDownloader
  → Marketplace.DecryptContents
  → Marketplace.CrackLevelDat
  → Marketplace.CrackSkinsJson
  → Manifest.ReadType
  → Manifest.ReadName
  → EscapeFilename
  → MoveDirectory
  → DeleteTempFiles
```

## Package processing path

```text
Marketplace.DecryptContents
  → Marketplace.worldOrContentsJsonDecrypt
  → Marketplace.decryptContentsJsonFiles
  → Marketplace.DecryptContents (recursive)

Marketplace.worldOrContentsJsonDecrypt
  → Utils.ReadString
  → Keys.LookupKey
  → Crypto.Aes256CfbDecrypt

Marketplace.EncryptContents
  → Keys.GenerateKey
  → Marketplace.EncryptContents (recursive)
  → Marketplace.shouldEncrypt
  → Utils.IsDirectory
  → Crypto.Aes256CfbEncrypt
  → Utils.WriteString
```

---

# 7. UI event bindings recovered

`InitializeComponent()` attaches at least these handlers:

```text
dl                  → dl_Click
progress            → progress_Click
search.TextChanged  → search_TextChanged
search.KeyPress     → search_KeyPress
options.CellContentClick → options_CellContentClick
options.Scroll      → options_Scroll
options.SelectionChanged → options_SelectionChanged
comboBox1.SelectedIndexChanged → comboBox1_SelectedIndexChanged
checkBox1.CheckedChanged → checkBox1_CheckedChanged
dlE                 → dlE_Click
button1             → button1_Click_1
KeysUpdateCheckBox.CheckedChanged → KeysUpdateCheckBox_CheckedChanged
Form.Load           → DownloadForm_Load
```

This is useful when reproducing behavior because it shows which user actions currently initiate search, configuration updates, and downloads.

---

# 8. Native interoperability

One native module reference was observed:

```text
dwmapi.dll
```

One P/Invoke was observed:

```text
DwmSetWindowAttribute
```

It belongs to the theme/dark-mode path, not the core content workflow.

---

# 9. Reimplementation warning

This symbol map is a **legacy compatibility map**. It should be used to answer:

- where a behavior lived;
- what depended on what;
- which responsibilities need equivalents;
- which coupling should be removed.

It should **not** be used as a requirement to preserve methods related to DRM/protected-content bypass, shared decryption-key distribution, or embedded credentials.
