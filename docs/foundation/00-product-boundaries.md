# 00 — Product Boundaries

## Product intent

SearchNow is a maintainable, user-friendly Minecraft Bedrock content-management desktop application derived from useful concepts observed in BlueCoin 2.4.

Target normal-user surfaces:

```text
Library
Discover
Downloads
Settings
```

The product should hide implementation details unless the user explicitly enters an advanced/diagnostic view.

## In scope

- Minecraft installation/local-content discovery;
- catalog browsing/search where access is permitted;
- owned/local content organization;
- download queue and progress through permitted paths;
- package inspection and manifest validation;
- BP/RP pairing and package organization;
- authorized package export/conversion;
- diagnostics, recovery, logging, and clear error handling;
- transparent settings and privacy controls.

## Explicitly out of scope

- Marketplace DRM/protection bypass;
- converting paid/restricted Marketplace content into free/unrestricted content;
- extraction, pooling, upload, download, or distribution of protected-content decryption keys;
- hidden entitlement/account-derived data transmission;
- embedded reusable credentials or secrets.

Legacy evidence documenting these behaviors remains preserved for understanding the old application, not as a target requirement.

## UX principles

1. One primary action per content state.
2. Human-readable errors first; technical details are secondary.
3. Network/account-sensitive actions are visible.
4. Local-first default for user-derived data.
5. Normal UI does not expose internal identifiers/keys unnecessarily.
6. Progress reflects real processing stages.
