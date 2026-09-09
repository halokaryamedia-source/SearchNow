use super::{model::*, MAX_MANIFEST_BYTES};
use serde::Deserialize;
use serde_json::Value;

#[derive(Debug)]
pub(crate) struct ManifestCandidate {
    pub manifest_path: String,
    pub pack_root: String,
    pub bytes: Vec<u8>,
}

#[derive(Debug, Deserialize)]
struct ManifestDocument {
    #[serde(default)]
    format_version: Option<Value>,
    #[serde(default)]
    header: Option<ManifestHeader>,
    #[serde(default)]
    modules: Vec<ManifestModule>,
    #[serde(default)]
    dependencies: Vec<ManifestDependency>,
}

#[derive(Debug, Deserialize)]
struct ManifestHeader {
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    uuid: Option<String>,
    #[serde(default)]
    version: Option<Value>,
}

#[derive(Debug, Deserialize)]
struct ManifestModule {
    #[serde(default, rename = "type")]
    module_type: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ManifestDependency {
    #[serde(default)]
    uuid: Option<String>,
    #[serde(default)]
    module_name: Option<String>,
    #[serde(default)]
    version: Option<Value>,
}

pub(crate) fn inspect_manifest(
    candidate: ManifestCandidate,
) -> Result<(PackManifestSummary, Vec<PackageIssue>), String> {
    if candidate.bytes.len() as u64 > MAX_MANIFEST_BYTES {
        return Err("manifest.json is larger than the inspection limit.".into());
    }

    let document: ManifestDocument = serde_json::from_slice(&candidate.bytes)
        .map_err(|error| format!("manifest.json is invalid JSON: {error}"))?;
    let header = document
        .header
        .ok_or_else(|| "manifest.json does not contain a header object.".to_string())?;

    let mut issues = Vec::new();
    let name = header
        .name
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| {
            issues.push(PackageIssue::warning(
                "manifest_name_missing",
                "Pack manifest has no display name; the folder name is used instead.",
                Some(candidate.manifest_path.clone()),
            ));
            fallback_pack_name(&candidate.pack_root)
        });

    let uuid = header.uuid.filter(|value| !value.trim().is_empty());
    match uuid.as_deref() {
        None => issues.push(PackageIssue::error(
            "manifest_uuid_missing",
            "Pack manifest header has no UUID.",
            Some(candidate.manifest_path.clone()),
        )),
        Some(value) if !looks_like_uuid(value) => issues.push(PackageIssue::warning(
            "manifest_uuid_malformed",
            "Pack manifest UUID is not in canonical UUID form.",
            Some(candidate.manifest_path.clone()),
        )),
        Some(_) => {}
    }

    let mut module_types: Vec<String> = document
        .modules
        .into_iter()
        .filter_map(|module| module.module_type)
        .map(|value| value.trim().to_ascii_lowercase())
        .filter(|value| !value.is_empty())
        .collect();
    module_types.sort();
    module_types.dedup();

    if module_types.is_empty() {
        issues.push(PackageIssue::error(
            "manifest_modules_missing",
            "Pack manifest declares no module types.",
            Some(candidate.manifest_path.clone()),
        ));
    }

    let kind = classify_modules(&module_types);
    if kind == PackKind::Unknown {
        issues.push(PackageIssue::warning(
            "manifest_module_unknown",
            "Pack module types are not recognized by the current inspector.",
            Some(candidate.manifest_path.clone()),
        ));
    } else if kind == PackKind::Mixed {
        issues.push(PackageIssue::warning(
            "manifest_module_mixed",
            "Pack manifest mixes multiple primary pack categories.",
            Some(candidate.manifest_path.clone()),
        ));
    }

    let dependencies = document
        .dependencies
        .into_iter()
        .map(|dependency| PackDependency {
            uuid: dependency.uuid.filter(|value| !value.trim().is_empty()),
            module_name: dependency
                .module_name
                .filter(|value| !value.trim().is_empty()),
            version: dependency.version.as_ref().and_then(version_text),
        })
        .collect();

    let has_scripts = module_types.iter().any(|value| value == "script");
    Ok((
        PackManifestSummary {
            manifest_path: candidate.manifest_path,
            pack_root: candidate.pack_root,
            name,
            description: header.description,
            uuid,
            version: header.version.as_ref().and_then(version_text),
            format_version: document.format_version.as_ref().and_then(version_text),
            kind,
            module_types,
            dependencies,
            has_scripts,
        },
        issues,
    ))
}

fn classify_modules(module_types: &[String]) -> PackKind {
    let behavior = module_types
        .iter()
        .any(|value| value == "data" || value == "script");
    let resource = module_types.iter().any(|value| value == "resources");
    let skin = module_types.iter().any(|value| value == "skin_pack");
    let world = module_types.iter().any(|value| value == "world_template");
    let categories = [behavior, resource, skin, world]
        .into_iter()
        .filter(|present| *present)
        .count();

    match categories {
        0 => PackKind::Unknown,
        1 if behavior => PackKind::BehaviorPack,
        1 if resource => PackKind::ResourcePack,
        1 if skin => PackKind::SkinPack,
        1 if world => PackKind::WorldTemplate,
        _ => PackKind::Mixed,
    }
}

fn version_text(value: &Value) -> Option<String> {
    match value {
        Value::String(text) if !text.trim().is_empty() => Some(text.clone()),
        Value::Number(number) => Some(number.to_string()),
        Value::Array(parts) => {
            let parts: Option<Vec<String>> = parts
                .iter()
                .map(|part| part.as_u64().map(|number| number.to_string()))
                .collect();
            parts.map(|parts| parts.join("."))
        }
        _ => None,
    }
}

fn looks_like_uuid(value: &str) -> bool {
    if value.len() != 36 {
        return false;
    }
    value.chars().enumerate().all(|(index, character)| {
        if matches!(index, 8 | 13 | 18 | 23) {
            character == '-'
        } else {
            character.is_ascii_hexdigit()
        }
    })
}

fn fallback_pack_name(pack_root: &str) -> String {
    if pack_root == "." {
        return "Unnamed pack".into();
    }
    pack_root
        .rsplit('/')
        .find(|part| !part.is_empty())
        .unwrap_or("Unnamed pack")
        .to_string()
}
