# D-001 — Tauri + Svelte + Rust Application Stack

## Context

BlueCoin 2.4 mixes WinForms UI, network behavior, local Minecraft discovery, processing, storage, and package behavior in one experimental application assembly. TranslateIT demonstrates a cleaner desktop pattern using Tauri/Svelte with Rust runtime ownership and a separate worker only where its AI domain requires Python.

SearchNow does not currently have an AI/runtime requirement that justifies a second process.

## Decision

Use:

```text
Tauri 2
+ Svelte 5 / TypeScript / Vite
+ Tailwind CSS + small semantic design tokens
+ Rust commands + Rust engine
```

Start with one desktop process. Do not create a Python worker/backend service.

## Why

- lower runtime/process overhead;
- clear UI/native boundary;
- modern productive UI development;
- Rust is suitable for filesystem, archive, HTTP, queue, validation, and local persistence work required by SearchNow;
- fewer packaging/dependency/failure surfaces;
- avoids recreating BlueCoin's UI/runtime coupling.

## Not chosen

- **Patch/rebuild WinForms/.NET legacy shape:** keeps the coupling we are trying to remove.
- **Electron:** unnecessary runtime weight for this product.
- **Python local worker:** no current capability requires it.
- **deep multi-project clean architecture from day one:** too much abstraction before real owners/callers exist.

## Guardrails

- Svelte owns presentation/transient state only.
- Raw Tauri invoke is centralized in `runtimeApi.ts`.
- Tauri commands remain thin.
- reusable truth lives in Rust `engine/`.
- initial source-size budgets prevent new god-objects.
- a second process requires a new durable architecture decision.

## Evidence / proof boundary

Repository CI may prove structure, source contracts, typecheck/build, and formatting. Actual Tauri/Windows runtime behavior requires local/target Windows execution.

## Follow-up owner

`docs/foundation/05-application-architecture.md` + `EngineData/Frontend/RustApp/`.
