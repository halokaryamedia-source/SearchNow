use crate::{platform::{PlatformContext, PlatformKind}, settings::MinecraftSettings};
use serde::Serialize;
use std::{collections::HashSet, fs, path::{Path, PathBuf}};

const MAX_ACCOUNT_ROOTS: usize = 64;

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum MinecraftChannel {
    Stable,
    Preview,
    Custom,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum MinecraftStorageKind {
    GdkShared,
    GdkUser,
    LegacyUwp,
    CustomOverride,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum MinecraftDiscoveryState {
    Found,
    NotFound,
    UnsupportedPlatform,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MinecraftStorageRoot {
    pub id: String,
    pub channel: MinecraftChannel,
    pub storage_kind: MinecraftStorageKind,
    pub root: PathBuf,
    pub account_hint: Option<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DiscoveryWarning {
    pub code: String,
    pub message: String,
    pub path: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MinecraftDiscoverySnapshot {
    pub state: MinecraftDiscoveryState,
    pub roots: Vec<MinecraftStorageRoot>,
    pub warnings: Vec<DiscoveryWarning>,
    pub checked_candidates: usize,
    pub message: String,
}

pub fn discover_minecraft_storage(
    settings: &MinecraftSettings,
    platform: &PlatformContext,
) -> MinecraftDiscoverySnapshot {
    if platform.kind != PlatformKind::Windows {
        return MinecraftDiscoverySnapshot {
            state: MinecraftDiscoveryState::UnsupportedPlatform,
            roots: Vec::new(),
            warnings: Vec::new(),
            checked_candidates: 0,
            message: "SearchNow currently supports Minecraft Bedrock discovery on Windows only.".into(),
        };
    }

    let mut collector = Collector::default();

    if let Some(root) = &settings.root_override {
        collector.add_candidate(
            root.clone(),
            MinecraftChannel::Custom,
            MinecraftStorageKind::CustomOverride,
            None,
            true,
        );
    }

    if let Some(roaming) = &platform.roaming_app_data {
        collect_gdk_product(
            &mut collector,
            &roaming.join("Minecraft Bedrock"),
            MinecraftChannel::Stable,
        );
        if settings.include_preview {
            collect_gdk_product(
                &mut collector,
                &roaming.join("Minecraft Bedrock Preview"),
                MinecraftChannel::Preview,
            );
        }
    }

    if settings.include_legacy_uwp {
        if let Some(local) = &platform.local_app_data {
            collector.add_candidate(
                local
                    .join("Packages")
                    .join("Microsoft.MinecraftUWP_8wekyb3d8bbwe")
                    .join("LocalState")
                    .join("games")
                    .join("com.mojang"),
                MinecraftChannel::Stable,
                MinecraftStorageKind::LegacyUwp,
                None,
                false,
            );
            if settings.include_preview {
                collector.add_candidate(
                    local
                        .join("Packages")
                        .join("Microsoft.MinecraftWindowsBeta_8wekyb3d8bbwe")
                        .join("LocalState")
                        .join("games")
                        .join("com.mojang"),
                    MinecraftChannel::Preview,
                    MinecraftStorageKind::LegacyUwp,
                    None,
                    false,
                );
            }
        }
    }

    collector.finish()
}

fn collect_gdk_product(
    collector: &mut Collector,
    product_root: &Path,
    channel: MinecraftChannel,
) {
    collector.add_candidate(
        product_root.join("users").join("shared").join("games").join("com.mojang"),
        channel,
        MinecraftStorageKind::GdkShared,
        None,
        false,
    );

    let user_root = product_root.join("users");
    let entries = match fs::read_dir(&user_root) {
        Ok(entries) => entries,
        Err(_) => return,
    };
    let mut directories: Vec<_> = entries.filter_map(Result::ok).collect();
    directories.sort_by_key(|entry| entry.file_name());
    if directories.len() > MAX_ACCOUNT_ROOTS {
        collector.warnings.push(DiscoveryWarning {
            code: "minecraft_user_root_limit".into(),
            message: format!("Only the first {MAX_ACCOUNT_ROOTS} Minecraft user roots were inspected."),
            path: Some(user_root.clone()),
        });
        directories.truncate(MAX_ACCOUNT_ROOTS);
    }
    for entry in directories {
        let Ok(file_type) = entry.file_type() else { continue };
        if !file_type.is_dir() || file_type.is_symlink() {
            continue;
        }
        let name = entry.file_name().to_string_lossy().to_string();
        if name.eq_ignore_ascii_case("shared") {
            continue;
        }
        collector.add_candidate(
            entry.path().join("games").join("com.mojang"),
            channel,
            MinecraftStorageKind::GdkUser,
            Some(name),
            false,
        );
    }
}

#[derive(Default)]
struct Collector {
    roots: Vec<MinecraftStorageRoot>,
    warnings: Vec<DiscoveryWarning>,
    seen: HashSet<String>,
    checked: usize,
}

impl Collector {
    fn add_candidate(
        &mut self,
        path: PathBuf,
        channel: MinecraftChannel,
        storage_kind: MinecraftStorageKind,
        account_hint: Option<String>,
        warn_missing: bool,
    ) {
        self.checked += 1;
        if !path.is_dir() {
            if warn_missing {
                self.warnings.push(DiscoveryWarning {
                    code: "minecraft_override_missing".into(),
                    message: "The configured Minecraft root does not exist or is not a directory.".into(),
                    path: Some(path),
                });
            }
            return;
        }
        let normalized = normalized_path_key(&path);
        if !self.seen.insert(normalized.clone()) {
            return;
        }
        self.roots.push(MinecraftStorageRoot {
            id: format!("root-{:016x}", fnv1a(normalized.as_bytes())),
            channel,
            storage_kind,
            root: path,
            account_hint,
        });
    }

    fn finish(mut self) -> MinecraftDiscoverySnapshot {
        self.roots.sort_by(|left, right| left.root.cmp(&right.root));
        let found = !self.roots.is_empty();
        MinecraftDiscoverySnapshot {
            state: if found { MinecraftDiscoveryState::Found } else { MinecraftDiscoveryState::NotFound },
            roots: self.roots,
            warnings: self.warnings,
            checked_candidates: self.checked,
            message: if found {
                "Minecraft Bedrock local storage was detected.".into()
            } else {
                "Minecraft Bedrock local storage was not detected.".into()
            },
        }
    }
}

fn normalized_path_key(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/").to_lowercase()
}

fn fnv1a(bytes: &[u8]) -> u64 {
    let mut hash = 0xcbf29ce484222325u64;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::platform::PlatformContext;

    #[test]
    fn finds_current_gdk_shared_root() {
        let directory = tempfile::tempdir().expect("tempdir");
        let roaming = directory.path().join("Roaming");
        let local = directory.path().join("Local");
        let root = roaming.join("Minecraft Bedrock/users/shared/games/com.mojang");
        fs::create_dir_all(&root).expect("fixture");
        let snapshot = discover_minecraft_storage(
            &MinecraftSettings::default(),
            &PlatformContext::windows(roaming, local),
        );
        assert_eq!(snapshot.state, MinecraftDiscoveryState::Found);
        assert!(snapshot.roots.iter().any(|item| item.root == root));
    }

    #[test]
    fn finds_account_scoped_gdk_root() {
        let directory = tempfile::tempdir().expect("tempdir");
        let roaming = directory.path().join("Roaming");
        let local = directory.path().join("Local");
        let root = roaming.join("Minecraft Bedrock/users/12345/games/com.mojang");
        fs::create_dir_all(&root).expect("fixture");
        let snapshot = discover_minecraft_storage(
            &MinecraftSettings::default(),
            &PlatformContext::windows(roaming, local),
        );
        assert!(snapshot.roots.iter().any(|item| item.root == root));
    }

    #[test]
    fn legacy_uwp_is_a_fallback_candidate() {
        let directory = tempfile::tempdir().expect("tempdir");
        let roaming = directory.path().join("Roaming");
        let local = directory.path().join("Local");
        let root = local.join("Packages/Microsoft.MinecraftUWP_8wekyb3d8bbwe/LocalState/games/com.mojang");
        fs::create_dir_all(&root).expect("fixture");
        let snapshot = discover_minecraft_storage(
            &MinecraftSettings::default(),
            &PlatformContext::windows(roaming, local),
        );
        assert!(snapshot.roots.iter().any(|item| item.root == root));
    }

    #[test]
    fn preview_is_opt_in() {
        let directory = tempfile::tempdir().expect("tempdir");
        let roaming = directory.path().join("Roaming");
        let local = directory.path().join("Local");
        let preview = roaming.join("Minecraft Bedrock Preview/users/shared/games/com.mojang");
        fs::create_dir_all(&preview).expect("fixture");
        let snapshot = discover_minecraft_storage(
            &MinecraftSettings::default(),
            &PlatformContext::windows(roaming, local),
        );
        assert!(!snapshot.roots.iter().any(|item| item.root == preview));
    }
}
