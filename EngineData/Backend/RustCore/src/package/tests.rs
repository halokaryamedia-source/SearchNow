use super::*;
use std::{fs, io::Write, path::Path};
use zip::{write::SimpleFileOptions, CompressionMethod, ZipWriter};

const BP_UUID: &str = "11111111-1111-4111-8111-111111111111";
const RP_UUID: &str = "22222222-2222-4222-8222-222222222222";

#[test]
fn folder_detects_behavior_resource_relationship() {
    let directory = tempfile::tempdir().expect("tempdir");
    let behavior = directory.path().join("BP");
    let resource = directory.path().join("RP");
    fs::create_dir_all(&behavior).expect("behavior folder");
    fs::create_dir_all(&resource).expect("resource folder");
    fs::write(
        behavior.join("manifest.json"),
        behavior_manifest(BP_UUID, RP_UUID),
    )
    .expect("behavior manifest");
    fs::write(resource.join("manifest.json"), resource_manifest(RP_UUID))
        .expect("resource manifest");

    let inspection = inspect_package(directory.path()).expect("inspection");
    assert_eq!(inspection.status, PackageInspectionStatus::Ready);
    assert_eq!(inspection.packs.len(), 2);
    assert_eq!(inspection.relationships.len(), 1);
    assert_eq!(
        inspection.relationships[0].kind,
        PackageRelationshipKind::BehaviorRequiresResource
    );
}

#[test]
fn mcpack_is_read_only_and_classified() {
    let directory = tempfile::tempdir().expect("tempdir");
    let archive = directory.path().join("sample.mcpack");
    write_archive(
        &archive,
        &[("manifest.json", resource_manifest(RP_UUID).as_bytes())],
    );

    let inspection = inspect_package(&archive).expect("inspection");
    assert_eq!(inspection.input_kind, PackageInputKind::McPack);
    assert_eq!(inspection.safety, PackageSafety::Safe);
    assert_eq!(inspection.status, PackageInspectionStatus::Ready);
    assert_eq!(inspection.packs[0].kind, PackKind::ResourcePack);
    assert!(archive.is_file());
}

#[test]
fn mcaddon_detects_two_pack_manifests() {
    let directory = tempfile::tempdir().expect("tempdir");
    let archive = directory.path().join("sample.mcaddon");
    let behavior = behavior_manifest(BP_UUID, RP_UUID);
    let resource = resource_manifest(RP_UUID);
    write_archive(
        &archive,
        &[
            ("Behavior/manifest.json", behavior.as_bytes()),
            ("Resource/manifest.json", resource.as_bytes()),
        ],
    );

    let inspection = inspect_package(&archive).expect("inspection");
    assert_eq!(inspection.input_kind, PackageInputKind::McAddon);
    assert_eq!(inspection.packs.len(), 2);
    assert_eq!(inspection.relationships.len(), 1);
}

#[test]
fn archive_path_traversal_is_rejected() {
    let directory = tempfile::tempdir().expect("tempdir");
    let archive = directory.path().join("unsafe.mcpack");
    let manifest = resource_manifest(RP_UUID);
    write_archive(
        &archive,
        &[
            ("manifest.json", manifest.as_bytes()),
            ("../../escape.txt", b"unsafe"),
        ],
    );

    let inspection = inspect_package(&archive).expect("inspection");
    assert_eq!(inspection.safety, PackageSafety::Rejected);
    assert_eq!(inspection.status, PackageInspectionStatus::Rejected);
    assert!(inspection
        .issues
        .iter()
        .any(|issue| issue.code == "archive_path_rejected"));
}

#[test]
fn duplicate_pack_uuid_is_reported() {
    let directory = tempfile::tempdir().expect("tempdir");
    for name in ["A", "B"] {
        let pack = directory.path().join(name);
        fs::create_dir_all(&pack).expect("pack folder");
        fs::write(pack.join("manifest.json"), resource_manifest(RP_UUID)).expect("manifest");
    }

    let inspection = inspect_package(directory.path()).expect("inspection");
    assert_eq!(inspection.status, PackageInspectionStatus::Issues);
    assert!(inspection
        .issues
        .iter()
        .any(|issue| issue.code == "package_duplicate_uuid"));
}

fn write_archive(path: &Path, entries: &[(&str, &[u8])]) {
    let file = fs::File::create(path).expect("archive file");
    let mut writer = ZipWriter::new(file);
    let options = SimpleFileOptions::default().compression_method(CompressionMethod::Stored);
    for (name, bytes) in entries {
        writer
            .start_file(*name, options)
            .expect("start archive file");
        writer.write_all(bytes).expect("write archive file");
    }
    writer.finish().expect("finish archive");
}

fn behavior_manifest(uuid: &str, resource_uuid: &str) -> String {
    format!(
        r#"{{
  "format_version": 2,
  "header": {{"name":"Behavior","uuid":"{uuid}","version":[1,0,0]}},
  "modules": [{{"type":"data","uuid":"33333333-3333-4333-8333-333333333333","version":[1,0,0]}}],
  "dependencies": [{{"uuid":"{resource_uuid}","version":[1,0,0]}}]
}}"#
    )
}

fn resource_manifest(uuid: &str) -> String {
    format!(
        r#"{{
  "format_version": 2,
  "header": {{"name":"Resource","uuid":"{uuid}","version":[1,0,0]}},
  "modules": [{{"type":"resources","uuid":"44444444-4444-4444-8444-444444444444","version":[1,0,0]}}]
}}"#
    )
}
