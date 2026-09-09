# 07 — Reconstruction Evidence & Confidence

## 1. Why this file exists

The SearchNow repository does not contain BlueCoin's original source code. The legacy architecture documentation is reconstructed from the supplied executable. This file records the evidence boundary so future contributors know exactly what is verified and what remains inferred.

---

## 2. Analyzed artifact

```text
File: BlueCoin_2.4.exe
Size: 4,702,510 bytes
SHA-256: 1ff4cca4208e051819479f293c2c474d1a0638a11ff206b64e7bdf2580f2c365
Format: PE32+ Windows GUI, x86-64
```

Analysis mode:

```text
STATIC ONLY
```

The executable was not launched during this architecture pass.

---

## 3. .NET single-file evidence

The executable contains the standard .NET single-file bundle marker and a bundle v6.0 manifest.

Recovered manifest:

| Embedded file | Offset | Original size |
|---|---:|---:|
| `BlueCoin2.deps.json` | 158720 | 1600 |
| `BlueCoin2.runtimeconfig.json` | 160320 | 372 |
| `Newtonsoft.Json.dll` | 163840 | 1830912 |
| `NLog.dll` | 1994752 | 2441216 |
| `BlueCoin2.dll` | 4435968 | 266240 |

Bundle ID observed:

```text
NWNh5EQWmqfHFfqdy+Fks3hF+phhz+o=
```

The bundle manifest ends at the physical end of the EXE, matching the inspected file size.

---

## 4. Runtime configuration evidence

Recovered runtime configuration identifies:

```text
Target framework: net7.0
Runtime target: .NETCoreApp,Version=v7.0/win-x64
Frameworks:
  Microsoft.NETCore.App 7.0.0
  Microsoft.WindowsDesktop.App 7.0.0
```

Recovered dependency manifest identifies:

```text
Newtonsoft.Json 13.0.1
NLog 5.3.2
```

---

## 5. CLR metadata evidence

`BlueCoin2.dll` contains normal CLR metadata streams:

```text
#~
#Strings
#US
#GUID
#Blob
```

Observed table counts relevant to this audit:

```text
TypeRef    231
TypeDef     60
Field      391
MethodDef  186
Param      149
MemberRef  524
```

This makes class/method/field names in the architecture documentation directly evidence-backed rather than guessed from UI behavior.

---

## 6. Internal-call evidence

Method bodies were inspected statically for CLR method-call relationships. Examples directly observed include:

```text
Program.Main
  → ApplicationConfiguration.Initialize
  → DownloadForm..ctor

DownloadForm.GetCatPictures
  → McCrypt.Keys.ReadOptionsTxt
  → McCrypt.Keys.ReadEntitlementFile
  → McCrypt.Keys.ExportKeysJson2

DownloadForm.DoSearch
  → PlayFab.Search

DownloadForm.DownloadSelected
  → PlayFab.GetProductInformation
  → FileDownloader
  → Marketplace.DecryptContents
  → Marketplace.CrackLevelDat
  → Marketplace.CrackSkinsJson
  → Manifest.ReadType
  → Manifest.ReadName

PlayFab.PullEntityTokenOutOfMyAss
  → Config.GetConfValue
  → GetPublicKeyAndMicrosoftTakesABigL
  → LoginWithCustomId
  → RefreshEntityTokenBullshit
```

This is why the documented startup/search/download dependency chains are labeled **Observed** rather than purely inferred.

---

## 7. String/resource evidence

The assembly contains strings directly identifying:

- Minecraft UWP package path;
- `options.txt` and `*.ent` discovery;
- `manifest.json`, `contents.json`, `skins.json`, `level.dat`;
- `keys.tsv` and backup behavior;
- PlayFab routes;
- Marketplace search tags/types;
- output extensions;
- configuration defaults;
- WinForms control labels;
- third-party backend host;
- an embedded authentication/title secret.

The secret itself is intentionally excluded from SearchNow documentation.

---

## 8. Native interop evidence

Observed native module reference:

```text
dwmapi.dll
```

Observed imported method:

```text
DwmSetWindowAttribute
```

Its call path is attached to the theme/dark-mode implementation.

---

# 9. Confidence matrix

| Area | Confidence | Basis |
|---|---|---|
| Runtime / framework | Very high | bundle + runtimeconfig |
| Embedded dependencies | Very high | bundle + deps.json |
| Type/class names | Very high | CLR TypeDef |
| Field/method names | Very high | CLR metadata |
| Internal method relationships | High | static IL call analysis |
| Local paths/config keys | High | embedded user strings + call context |
| Network routes | High | embedded strings + PlayFab method context |
| Physical original `.cs` file split | Medium | reconstructed from type/compiler conventions |
| Exact runtime timing/concurrency behavior | Medium | static analysis only |
| Error behavior in real network conditions | Unknown until test | runtime test required |
| UI pixel/layout accuracy | Not frozen here | requires visual/runtime inspection |

---

# 10. Documentation rule

Future SearchNow documents must not say “the old source does X” unless one of these is true:

1. it is directly observed in this static evidence;
2. it is verified later by controlled runtime testing;
3. original BlueCoin source becomes available.

Otherwise use `reconstructed` or `inferred` explicitly.

---

# 11. Architecture freeze status

The static source architecture is now considered **documented enough to stop rediscovering the same binary structure during normal SearchNow development**.

Remaining legacy investigation should only be done when a specific unresolved behavior blocks a documented SearchNow requirement.
