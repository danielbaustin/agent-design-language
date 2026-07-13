use csdlc_v2::{
    install_binaries, resolve_operator_generation, verify_coexistence, CoexistenceInventory,
    Generation, SkillManifest,
};
use std::fs;

#[test]
fn nine_skills_are_typed_and_bind_the_generation_selector() {
    let manifest = SkillManifest::load().unwrap();
    assert_eq!(manifest.skills.len(), 9);
    assert_eq!(
        manifest.generation_selector,
        "csdlc-v2/operator/generation-selector.json"
    );
    assert!(manifest
        .skills
        .iter()
        .all(|r| r.binary.starts_with("csdlc-") && !r.binary.contains("python")));
}
#[test]
fn coexistence_fails_closed_when_v1_or_v2_is_missing() {
    let repo = tempfile::tempdir().unwrap();
    let bins = tempfile::tempdir().unwrap();
    let inventory = CoexistenceInventory::load().unwrap();
    assert!(verify_coexistence(repo.path(), bins.path(), &inventory).is_err());
    let mut altered = inventory.clone();
    altered.required_v1_paths.clear();
    assert!(verify_coexistence(repo.path(), bins.path(), &altered).is_err());
}
#[test]
fn installer_records_provenance_without_replacing_other_files() {
    let source = tempfile::tempdir().unwrap();
    let destination_parent = tempfile::tempdir().unwrap();
    let destination = destination_parent.path().join("csdlc-v2");
    let manifest = SkillManifest::load().unwrap();
    for name in manifest.required_binaries() {
        fs::write(source.path().join(&name), name.as_bytes()).unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(source.path().join(name), fs::Permissions::from_mode(0o755))
                .unwrap();
        }
    }
    fs::write(destination_parent.path().join("v1-stays"), b"v1").unwrap();
    let receipt = install_binaries(source.path(), &destination).unwrap();
    assert_eq!(receipt.binaries.len(), 11);
    assert_eq!(
        fs::read(destination_parent.path().join("v1-stays")).unwrap(),
        b"v1"
    );
    assert!(destination.join("install-receipt.json").is_file());
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        assert_ne!(
            fs::metadata(destination.join("csdlc-init"))
                .unwrap()
                .permissions()
                .mode()
                & 0o111,
            0
        );
    }
    let repo = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
    let inventory = CoexistenceInventory::load().unwrap();
    assert!(
        verify_coexistence(&repo, &destination, &inventory)
            .unwrap()
            .pass
    );
    fs::write(destination.join("csdlc-init"), b"tampered").unwrap();
    let tampered = verify_coexistence(&repo, &destination, &inventory).unwrap();
    assert!(!tampered.pass);
    assert!(tampered.missing_v2_binaries.contains(&"csdlc-init".into()));

    #[cfg(unix)]
    {
        use std::os::unix::fs::symlink;
        fs::remove_file(destination.join("install-receipt.json")).unwrap();
        symlink("/bin/true", destination.join("install-receipt.json")).unwrap();
        assert!(verify_coexistence(&repo, &destination, &inventory).is_err());
    }
}

#[test]
fn operator_guidance_is_bound_to_manifest_and_coexistence_contract() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let root_agents = fs::read_to_string(root.join("../AGENTS.md")).unwrap();
    let nested_agents = fs::read_to_string(root.join("AGENTS.md")).unwrap();
    let manifest = SkillManifest::load().unwrap();
    let selector: csdlc_v2::GenerationSelector =
        serde_json::from_slice(&fs::read(root.join("operator/generation-selector.json")).unwrap())
            .unwrap();
    assert_eq!(manifest.skills.len(), 9);
    assert_eq!(
        resolve_operator_generation(&root.join(".."), 5294, None).unwrap(),
        selector.default_generation
    );
    assert_eq!(
        resolve_operator_generation(&root.join(".."), 5294, Some(Generation::V1)).unwrap(),
        Generation::V1
    );
    for text in [&root_agents, &nested_agents] {
        assert!(text.contains("v1"));
        assert!(text.contains("csdlc-install"));
        assert!(text.contains("nine"));
    }
}

#[test]
fn missing_late_source_leaves_prior_generation_untouched() {
    let source = tempfile::tempdir().unwrap();
    let destination_parent = tempfile::tempdir().unwrap();
    let destination = destination_parent.path().join("csdlc-v2");
    fs::create_dir(&destination).unwrap();
    fs::write(destination.join("previous"), b"known-good").unwrap();
    let manifest = SkillManifest::load().unwrap();
    for name in manifest.required_binaries().iter().take(9) {
        fs::write(source.path().join(name), name.as_bytes()).unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(source.path().join(name), fs::Permissions::from_mode(0o755))
                .unwrap();
        }
    }
    assert!(install_binaries(source.path(), &destination).is_err());
    assert_eq!(
        fs::read(destination.join("previous")).unwrap(),
        b"known-good"
    );
    assert_eq!(fs::read_dir(&destination).unwrap().count(), 1);
}

#[test]
fn shared_destination_and_non_executable_sources_are_rejected_without_mutation() {
    let source = tempfile::tempdir().unwrap();
    let parent = tempfile::tempdir().unwrap();
    let shared = parent.path().join("bin");
    fs::create_dir(&shared).unwrap();
    fs::write(shared.join("v1-owner"), b"v1").unwrap();
    assert!(install_binaries(source.path(), &shared).is_err());
    assert_eq!(fs::read(shared.join("v1-owner")).unwrap(), b"v1");

    let dedicated = parent.path().join("csdlc-v2");
    let manifest = SkillManifest::load().unwrap();
    for name in manifest.required_binaries() {
        fs::write(source.path().join(name), b"not executable").unwrap();
    }
    assert!(install_binaries(source.path(), &dedicated).is_err());
    assert!(!dedicated.exists());
}

#[cfg(unix)]
#[test]
fn symlinked_installed_binaries_fail_coexistence() {
    use std::os::unix::fs::symlink;
    let repo = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
    let source = tempfile::tempdir().unwrap();
    let parent = tempfile::tempdir().unwrap();
    let bins = parent.path().join("csdlc-v2");
    for name in SkillManifest::load().unwrap().required_binaries() {
        use std::os::unix::fs::PermissionsExt;
        fs::write(source.path().join(&name), name.as_bytes()).unwrap();
        fs::set_permissions(source.path().join(name), fs::Permissions::from_mode(0o755)).unwrap();
    }
    install_binaries(source.path(), &bins).unwrap();
    fs::remove_file(bins.join("csdlc-init")).unwrap();
    symlink("/bin/true", bins.join("csdlc-init")).unwrap();
    let report = verify_coexistence(&repo, &bins, &CoexistenceInventory::load().unwrap()).unwrap();
    assert!(!report.pass);
    assert_eq!(report.missing_v2_binaries, vec!["csdlc-init"]);
}
