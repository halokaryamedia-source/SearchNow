pub mod app_runtime;
pub mod catalog;
pub mod diagnostics;
pub mod download;
pub mod error;
pub mod library;
pub mod minecraft;
pub mod package;
mod persistence;
pub mod platform;
pub mod provider_adapter;
pub mod provider_session;
pub mod runtime;
pub mod settings;

#[cfg(test)]
mod app_runtime_tests;

use library::LibrarySnapshot;
use minecraft::MinecraftDiscoverySnapshot;
use platform::PlatformContext;
use serde::Serialize;
use settings::AppSettings;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalBackendSnapshot {
    pub minecraft: MinecraftDiscoverySnapshot,
    pub library: LibrarySnapshot,
}

pub fn build_local_backend_snapshot(
    settings: &AppSettings,
    platform: &PlatformContext,
) -> LocalBackendSnapshot {
    let minecraft = minecraft::discover_minecraft_storage(&settings.minecraft, platform);
    let library = library::scan_library(
        &minecraft.roots,
        settings.minecraft.include_development_content,
    );
    LocalBackendSnapshot { minecraft, library }
}
