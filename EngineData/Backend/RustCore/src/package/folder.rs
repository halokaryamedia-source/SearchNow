use super::{manifest::ManifestCandidate, model::PackageIssue, MAX_MANIFEST_BYTES};
use std::{collections::VecDeque, fs, path::Path};

const MAX_SEARCH_DEPTH: usize = 2;
const MAX_DIRECTORIES: usize = 512;

pub(crate) struct FolderScan {
    pub candidates: Vec<ManifestCandidate>,
    pub issues: Vec<PackageIssue>,
}

pub(crate) fn scan_folder(root: &Path) -> FolderScan {
    let mut candidates = Vec::new();
    let mut issues = Vec::new();
    let mut queue = VecDeque::from([(root.to_path_buf(), 0usize)]);
    let mut visited = 0usize;

    while let Some((directory, depth)) = queue.pop_front() {
        visited += 1;
        if visited > MAX_DIRECTORIES {
            issues.push(PackageIssue::warning(
                "package_folder_limit",
                format!("Package folder inspection stopped after {MAX_DIRECTORIES} directories."),
                Some(display_relative(root, &directory)),
            ));
            break;
        }

        let manifest = directory.join("manifest.json");
        if manifest.is_file() {
            if let Some(candidate) =
                read_manifest_candidate(root, &directory, &manifest, &mut issues)
            {
                candidates.push(candidate);
            }
            continue;
        }

        if depth >= MAX_SEARCH_DEPTH {
            continue;
        }
        let entries = match fs::read_dir(&directory) {
            Ok(entries) => entries,
            Err(error) => {
                issues.push(PackageIssue::warning(
                    "package_folder_read_failed",
                    format!("Could not inspect package folder: {error}"),
                    Some(display_relative(root, &directory)),
                ));
                continue;
            }
        };
        let mut directories: Vec<_> = entries.filter_map(Result::ok).collect();
        directories.sort_by_key(|entry| entry.file_name());
        for entry in directories {
            let metadata = match fs::symlink_metadata(entry.path()) {
                Ok(metadata) => metadata,
                Err(_) => continue,
            };
            if metadata.file_type().is_symlink() || !metadata.is_dir() {
                continue;
            }
            queue.push_back((entry.path(), depth + 1));
        }
    }

    FolderScan { candidates, issues }
}

fn read_manifest_candidate(
    root: &Path,
    pack_root_path: &Path,
    manifest_path: &Path,
    issues: &mut Vec<PackageIssue>,
) -> Option<ManifestCandidate> {
    let metadata = match fs::symlink_metadata(manifest_path) {
        Ok(metadata) => metadata,
        Err(error) => {
            issues.push(PackageIssue::error(
                "manifest_metadata_failed",
                format!("Could not inspect manifest.json: {error}"),
                Some(display_relative(root, manifest_path)),
            ));
            return None;
        }
    };
    if metadata.file_type().is_symlink() {
        issues.push(PackageIssue::error(
            "manifest_symlink_rejected",
            "Symlinked manifest.json files are not inspected.",
            Some(display_relative(root, manifest_path)),
        ));
        return None;
    }
    if metadata.len() > MAX_MANIFEST_BYTES {
        issues.push(PackageIssue::error(
            "manifest_too_large",
            "manifest.json is larger than the inspection limit.",
            Some(display_relative(root, manifest_path)),
        ));
        return None;
    }

    let bytes = match fs::read(manifest_path) {
        Ok(bytes) if bytes.len() as u64 <= MAX_MANIFEST_BYTES => bytes,
        Ok(_) => {
            issues.push(PackageIssue::error(
                "manifest_too_large",
                "manifest.json grew beyond the inspection limit while being read.",
                Some(display_relative(root, manifest_path)),
            ));
            return None;
        }
        Err(error) => {
            issues.push(PackageIssue::error(
                "manifest_read_failed",
                format!("Could not read manifest.json: {error}"),
                Some(display_relative(root, manifest_path)),
            ));
            return None;
        }
    };

    let pack_root = display_relative(root, pack_root_path);
    let pack_root = if pack_root.is_empty() {
        ".".into()
    } else {
        pack_root
    };
    Some(ManifestCandidate {
        manifest_path: display_relative(root, manifest_path),
        pack_root,
        bytes,
    })
}

fn display_relative(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}
