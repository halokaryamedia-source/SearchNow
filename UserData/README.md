# UserData

This directory documents SearchNow runtime-data ownership. Runtime-generated data itself is not committed.

Target application data classes:

```text
CacheData/     disposable cache, thumbnails, temporary download/process data
LogData/       minimal/redacted diagnostics
AppData/       persistent local settings and application-owned indexes/state
```

Minecraft-owned files remain in Minecraft's own directories and must not be silently copied into repository source.

Rules:

- local-first by default;
- no credentials, entitlement-derived secrets, content keys, or private user material in Git;
- cache is replaceable;
- logs are minimal and redacted;
- persistent application data is versioned/migrated by the runtime once those contracts are implemented.
