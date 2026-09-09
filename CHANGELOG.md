# Changelog

All notable SearchNow repository/product changes are recorded here.

## Unreleased

### Added

- Recovered BlueCoin 2.4 architecture and runtime documentation.
- PRD-Creator-style development routing and `develop → Local → main` promotion model.
- Repository continuity, ownership, source-authority, verification, and promotion policy.
- Finalized Tauri 2 + Svelte 5 + TypeScript/Vite + Rust application architecture.
- `EngineData/Frontend/RustApp` application scaffold with Library, Discover, Downloads, and Settings surfaces.
- Product-facade → Tauri API → Rust command → Rust engine runtime-status vertical slice.
- Architecture/source-size validation gates designed to prevent direct bridge leakage and new god-objects.

### Changed

- Replaced the earlier technology-neutral/.NET-shaped implementation roadmap with the approved Tauri/Svelte/Rust source model.
- Updated repository CI to validate the frontend architecture, typecheck/build output, repository contracts, and Rust formatting.
