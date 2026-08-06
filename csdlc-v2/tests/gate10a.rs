use csdlc_v2::operator::validate_external_cargo_target;
use csdlc_v2::{
    bind_issue, build_and_install_binaries, edit_issue, install_binaries,
    resolve_operator_generation, verify_coexistence, BindRequest, BootstrapRequest, CardKind,
    CoexistenceInventory, EditRequest, Generation, LifecyclePhase, SemanticOperation,
    SkillManifest, Store,
};
use std::fs;
use std::path::Path;
use std::process::Command;

#[test]
fn eleven_skills_are_typed_and_bind_the_generation_selector() {
    let manifest = SkillManifest::load().unwrap();
    assert_eq!(manifest.skills.len(), 11);
    assert_eq!(
        manifest.generation_selector,
        "csdlc-v2/operator/generation-selector.json"
    );
    assert!(manifest
        .skills
        .iter()
        .all(|r| r.binary.starts_with("csdlc-") && !r.binary.contains("python")));
    assert!(manifest
        .skills
        .iter()
        .any(|route| route.name == "csdlc-v2-clean" && route.binary == "csdlc-clean"));
}

#[test]
fn current_operator_guidance_has_no_sunset_v1_route() {
    let repo = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
    let manifest = SkillManifest::load().unwrap();
    for skill in &manifest.skills {
        let path = repo
            .join("csdlc-v2/operator/skills")
            .join(&skill.name)
            .join("SKILL.md");
        let text = fs::read_to_string(&path).unwrap();
        assert!(
            current_guidance_is_v2_only(&text, &["v1_sunset"]),
            "current operational skill retains sunset guidance: {}",
            path.display()
        );
    }

    let workflow = fs::read_to_string(repo.join("docs/default_workflow.md")).unwrap();
    assert!(workflow.starts_with("# Default C-SDLC v2 workflow"));
    assert!(workflow.contains("csdlc-init"));
    assert!(workflow.contains("csdlc-finish"));
    assert!(!workflow.contains("csdlc-closeout"));
    assert!(current_guidance_is_v2_only(
        &workflow,
        &["docs/legacy/default_workflow_v1.md"]
    ));
}

#[test]
fn current_bootstrap_guidance_does_not_call_deleted_prompt_wrapper() {
    let repo = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
    let current_guidance = [
        "docs/default_workflow.md",
        "docs/tooling/README.md",
        "docs/tooling/structured-prompt-validator-binary-resolution.md",
        "csdlc-v2/AGENTS.md",
        "csdlc-v2/operator/skills/csdlc-v2-init/SKILL.md",
        "csdlc-v2/operator/skills/csdlc-v2-validate/SKILL.md",
        "docs/templates/prompts/1.0.3/schemas/sip.structure.json",
        "docs/templates/prompts/1.0.3/schemas/stp.structure.json",
        "docs/templates/prompts/1.0.3/schemas/spp.structure.json",
        "docs/templates/prompts/1.0.3/schemas/vpp.structure.json",
        "docs/templates/prompts/1.0.3/schemas/srp.structure.json",
        "docs/templates/prompts/1.0.3/schemas/sor.structure.json",
    ];
    for relative in current_guidance {
        let text = fs::read_to_string(repo.join(relative)).unwrap();
        assert!(
            !text.contains("bash adl/tools/validate_structured_prompt.sh")
                && !text.contains(
                    "adl/tools/validate_structured_prompt.sh` is a compatibility wrapper"
                ),
            "current bootstrap guidance calls deleted prompt wrapper: {relative}"
        );
    }
}

#[test]
fn current_guidance_guard_rejects_exact_former_wrapper_command() {
    let former = "Run `bash ./adl/tools/pr.sh run 42`; pr.sh remains the default.";
    assert!(!current_guidance_is_v2_only(former, &[]));
}

#[test]
fn repo_wide_active_command_scan_enforces_final_authority() {
    let repo = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
    let status = Command::new("python3")
        .arg("adl/tools/generate_active_command_reference_scan.py")
        .arg("--check")
        .current_dir(repo)
        .status()
        .expect("run active command reference scan");
    assert!(status.success(), "repo-wide active command scan failed");
}

fn current_guidance_is_v2_only(text: &str, allowed_v1_references: &[&str]) -> bool {
    let mut normalized = text.to_ascii_lowercase().replace("./", "");
    for allowed in allowed_v1_references {
        normalized = normalized.replace(&allowed.to_ascii_lowercase(), "");
    }
    !normalized.contains("pr.sh")
        && !normalized.contains("workflow-conductor")
        && !normalized.contains("v1")
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
    let repo = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
    let destination_parent = tempfile::tempdir().unwrap();
    let destination = destination_parent.path().join("csdlc-v2");
    fs::write(destination_parent.path().join("v1-stays"), b"v1").unwrap();
    let receipt = install_binaries(prebuilt_binaries(), &destination).unwrap();
    let manifest = SkillManifest::load().unwrap();
    assert_eq!(receipt.binaries.len(), manifest.required_binaries().len());
    assert_eq!(
        fs::read(destination_parent.path().join("v1-stays")).unwrap(),
        b"v1"
    );
    assert!(destination.join("install-receipt.json").is_file());
    assert!(destination.join("csdlc-github").is_file());
    assert!(destination.join("csdlc-github-issue").is_file());
    assert!(destination.join("csdlc-github-pr").is_file());
    assert!(destination.join("csdlc-issue").is_file());
    assert!(destination.join("csdlc-install").is_file());
    assert!(!destination.join("csdlc-merge").exists());
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
    stamp_current_revision(&repo, &destination);
    let inventory = CoexistenceInventory::load().unwrap();
    assert!(
        verify_coexistence(&repo, &destination, &inventory)
            .unwrap()
            .pass
    );
    fs::create_dir_all(repo.join("adl/tools")).unwrap();
    fs::write(repo.join("adl/tools/pr.sh"), b"legacy").unwrap();
    let forbidden = verify_coexistence(&repo, &destination, &inventory).unwrap();
    assert!(!forbidden.pass);
    assert!(forbidden
        .present_forbidden_v1_paths
        .contains(&"adl/tools/pr.sh".into()));
    fs::remove_file(repo.join("adl/tools/pr.sh")).unwrap();
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
fn external_cargo_target_is_exact_existing_and_outside_checkout() {
    let repo = tempfile::tempdir().unwrap();
    let external = tempfile::tempdir().unwrap();
    let external = std::fs::canonicalize(external.path()).unwrap();
    assert_eq!(
        validate_external_cargo_target(repo.path(), &external).unwrap(),
        external
    );

    assert!(validate_external_cargo_target(repo.path(), Path::new("relative-target")).is_err());
    assert!(validate_external_cargo_target(repo.path(), &repo.path().join("missing")).is_err());
    std::fs::create_dir(repo.path().join("inside")).unwrap();
    assert!(validate_external_cargo_target(repo.path(), &repo.path().join("inside")).is_err());

    #[cfg(unix)]
    {
        use std::os::unix::fs::symlink;
        let alias = repo.path().join("external-alias");
        symlink(&external, &alias).unwrap();
        assert!(validate_external_cargo_target(repo.path(), &alias).is_err());
    }
}

#[test]
fn stale_owner_binary_provenance_fails_closed() {
    let repo = tempfile::tempdir().unwrap();
    git(repo.path(), &["init", "-b", "main"]);
    git(
        repo.path(),
        &["config", "user.email", "test@example.invalid"],
    );
    git(repo.path(), &["config", "user.name", "C-SDLC Test"]);
    fs::create_dir_all(repo.path().join("csdlc-v2/operator")).unwrap();
    fs::write(
        repo.path()
            .join("csdlc-v2/operator/generation-selector.json"),
        fs::read(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("operator/generation-selector.json"),
        )
        .unwrap(),
    )
    .unwrap();
    let source = repo.path().join("csdlc-v2/target/debug");
    fs::create_dir_all(&source).unwrap();
    let parent = tempfile::tempdir().unwrap();
    let bins = parent.path().join("csdlc-v2");
    for name in SkillManifest::load().unwrap().required_binaries() {
        fs::write(source.join(&name), name.as_bytes()).unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(source.join(name), fs::Permissions::from_mode(0o755)).unwrap();
        }
    }
    fs::write(repo.path().join("source-revision"), b"one").unwrap();
    git(repo.path(), &["add", "."]);
    git(repo.path(), &["commit", "-m", "first revision"]);
    install_binaries(&source, &bins).unwrap();
    fs::write(repo.path().join("source-revision"), b"two").unwrap();
    git(repo.path(), &["add", "source-revision"]);
    git(repo.path(), &["commit", "-m", "advance source revision"]);
    let receipt_path = bins.join("install-receipt.json");
    let mut receipt: serde_json::Value =
        serde_json::from_slice(&fs::read(&receipt_path).unwrap()).unwrap();
    assert!(receipt["source_revision"]
        .as_str()
        .unwrap()
        .starts_with("content:"));
    let error =
        verify_coexistence(repo.path(), &bins, &CoexistenceInventory::load().unwrap()).unwrap_err();
    assert!(
        error.message.contains("stale owner-binary provenance"),
        "{}",
        error.message
    );

    receipt["source_revision"] = serde_json::Value::String("git:stale-revision".into());
    fs::write(&receipt_path, serde_json::to_vec_pretty(&receipt).unwrap()).unwrap();
    let error =
        verify_coexistence(repo.path(), &bins, &CoexistenceInventory::load().unwrap()).unwrap_err();
    assert!(error.message.contains("stale owner-binary provenance"));
}

#[test]
fn untracked_build_input_is_rejected_before_cargo_runs() {
    let repo = tempfile::tempdir().unwrap();
    git(repo.path(), &["init", "-b", "main"]);
    git(
        repo.path(),
        &["config", "user.email", "test@example.invalid"],
    );
    git(repo.path(), &["config", "user.name", "C-SDLC Test"]);
    fs::create_dir_all(repo.path().join("csdlc-v2/src")).unwrap();
    fs::write(
        repo.path().join("csdlc-v2/Cargo.toml"),
        "[package]\nname='fixture'\nversion='0.1.0'\nedition='2021'\n",
    )
    .unwrap();
    fs::write(repo.path().join("csdlc-v2/src/main.rs"), "fn main() {}\n").unwrap();
    git(repo.path(), &["add", "."]);
    git(repo.path(), &["commit", "-m", "tracked source"]);
    fs::write(
        repo.path().join("csdlc-v2/build.rs"),
        "fn main() { std::fs::write(\"cargo-ran\", \"bad\").unwrap(); }\n",
    )
    .unwrap();
    let destination = tempfile::tempdir().unwrap().path().join("csdlc-v2");
    let error = build_and_install_binaries(repo.path(), &destination).unwrap_err();
    assert!(error.message.contains("dirty C-SDLC owner sources"));
    assert!(!repo.path().join("cargo-ran").exists());
    assert!(!destination.exists());
}

#[test]
fn dirty_shared_owner_dependency_is_rejected_before_cargo_runs() {
    let repo = tempfile::tempdir().unwrap();
    git(repo.path(), &["init", "-b", "main"]);
    git(
        repo.path(),
        &["config", "user.email", "test@example.invalid"],
    );
    git(repo.path(), &["config", "user.name", "C-SDLC Test"]);
    fs::create_dir_all(repo.path().join("csdlc-v2/src")).unwrap();
    fs::create_dir_all(repo.path().join("adl-resilience/src")).unwrap();
    fs::write(
        repo.path().join("csdlc-v2/Cargo.toml"),
        "[package]\nname='fixture'\nversion='0.1.0'\nedition='2021'\n",
    )
    .unwrap();
    fs::write(repo.path().join("csdlc-v2/src/main.rs"), "fn main() {}\n").unwrap();
    fs::write(
        repo.path().join("adl-resilience/Cargo.toml"),
        "[package]\nname='adl-resilience'\nversion='0.1.0'\nedition='2021'\n",
    )
    .unwrap();
    fs::write(
        repo.path().join("adl-resilience/src/lib.rs"),
        "pub fn clean() {}\n",
    )
    .unwrap();
    git(repo.path(), &["add", "."]);
    git(repo.path(), &["commit", "-m", "tracked source"]);
    fs::write(
        repo.path().join("adl-resilience/src/lib.rs"),
        "pub fn dirty_dependency() {}\n",
    )
    .unwrap();
    let destination = tempfile::tempdir().unwrap().path().join("csdlc-v2");
    let error = build_and_install_binaries(repo.path(), &destination).unwrap_err();
    assert!(error.message.contains("dirty C-SDLC owner sources"));
    assert!(!destination.exists());
}

#[test]
fn freshly_installed_stable_edit_binary_is_executable() {
    let parent = tempfile::tempdir().unwrap();
    let destination = parent.path().join("csdlc-v2");
    install_binaries(prebuilt_binaries(), &destination).unwrap();

    let fixture = tempfile::tempdir().unwrap();
    git(fixture.path(), &["init", "-b", "main"]);
    git(
        fixture.path(),
        &["config", "user.email", "test@example.invalid"],
    );
    git(fixture.path(), &["config", "user.name", "C-SDLC Test"]);
    fs::create_dir_all(fixture.path().join("docs")).unwrap();
    fs::write(fixture.path().join("docs/design.md"), "# Reviewed design\n").unwrap();
    fs::write(
        fixture.path().join("docs/diagram.mmd"),
        "flowchart LR\n  A --> B\n",
    )
    .unwrap();
    install_native_authority(fixture.path());
    let store = Store::new(fixture.path());
    csdlc_v2::initialize_native_json(&store, &serde_json::to_vec(&bootstrap_request()).unwrap())
        .unwrap();
    git(fixture.path(), &["switch", "-c", "issue-42"]);
    bind_issue(
        &store,
        BindRequest {
            issue: 42,
            base_branch: "main".into(),
            branch: "issue-42".into(),
            worktree: ".".into(),
        },
    )
    .unwrap();
    let bound = store.load_record(42).unwrap();
    let mut execution = edit(
        &bound,
        SemanticOperation::RecordExecution {
            summary: "implemented".into(),
            changes: vec!["docs/design.md".into()],
            artifacts: vec!["focused test".into()],
        },
    );
    execution.card = CardKind::Sor;
    let evidenced = edit_issue(&store, execution).unwrap();
    let implemented = edit_issue(
        &store,
        edit(
            &evidenced,
            SemanticOperation::AdvancePhase {
                phase: LifecyclePhase::Implemented,
            },
        ),
    )
    .unwrap();
    fs::write(
        fixture.path().join("docs/design.md"),
        "# Implemented correction\n",
    )
    .unwrap();
    let request = csdlc_v2::ApproveDesignRequest {
        issue: 42,
        expected_generation: implemented.generation,
        expected_digest: implemented.digest,
        reviewer: "architect".into(),
    };
    let request_path = fixture.path().join("approve.json");
    fs::write(&request_path, serde_json::to_vec_pretty(&request).unwrap()).unwrap();
    let result = Command::new(destination.join("csdlc-edit"))
        .args([
            "--repo",
            fixture.path().to_str().unwrap(),
            "approve-design",
            "--request",
            request_path.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "stable typed editor failed to reapprove implemented design: {}",
        String::from_utf8_lossy(&result.stderr)
    );
    let reapproved = store.load_record(42).unwrap();
    assert_eq!(reapproved.phase, LifecyclePhase::Implemented);
    assert_eq!(reapproved.generation, implemented.generation + 1);
}

fn install_native_authority(root: &std::path::Path) {
    let registry = root.join("docs/templates/prompts/current.json");
    let manifest = root.join("csdlc-v2/operator/native-card-shape.json");
    fs::create_dir_all(registry.parent().unwrap()).unwrap();
    fs::create_dir_all(manifest.parent().unwrap()).unwrap();
    fs::write(
        registry,
        include_bytes!("../../docs/templates/prompts/current.json"),
    )
    .unwrap();
    fs::write(
        manifest,
        include_bytes!("../operator/native-card-shape.json"),
    )
    .unwrap();
}

fn git(root: &std::path::Path, args: &[&str]) {
    let output = Command::new("git")
        .current_dir(root)
        .args(args)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

fn edit(record: &csdlc_v2::IssueRecord, operation: SemanticOperation) -> EditRequest {
    EditRequest {
        issue: 42,
        card: CardKind::Sip,
        expected_generation: record.generation,
        expected_digest: record.digest.clone(),
        actor: "agent".into(),
        reason: "test edit".into(),
        operation,
        fail_after_backup: false,
    }
}

fn bootstrap_request() -> BootstrapRequest {
    BootstrapRequest {
        issue: 42,
        repository: "example/repo".into(),
        actor: "agent".into(),
        design_path: "docs/design.md".into(),
        diagram_path: "docs/diagram.mmd".into(),
        design_reviewer: "reviewer".into(),
        design_approved: true,
        initial: csdlc_v2::InitialCardInput {
            title: "Gate 10A fixture".into(),
            slug: "gate-10a-fixture".into(),
            version: "v0.91.7".into(),
            goal: "Prove installed editor behavior.".into(),
            required_outcome: "Reapprove implemented design.".into(),
            declared_scope: vec!["fixture".into()],
            authority_boundary: vec!["no network".into()],
            operator_constraints: vec!["none".into()],
            task_boundary: "Fixture only.".into(),
            deliverables: vec!["record".into()],
            acceptance_criteria: vec!["editor reapproves".into()],
            dependencies: vec!["none".into()],
            repo_inputs: vec!["docs/design.md".into()],
            non_goals: vec!["GitHub".into()],
            plan_summary: "Construct and reapprove.".into(),
            steps: vec![csdlc_v2::cards::PlanStep {
                id: "step-1".into(),
                action: "exercise editor".into(),
                acceptance_ids: vec!["AC-1".into()],
                status: csdlc_v2::cards::StepStatus::Pending,
            }],
            invariants: vec!["typed mutation".into()],
            risks: vec!["binary drift".into()],
            planning_profile: csdlc_v2::PlanningProfile::Small,
            stop_conditions: vec!["failure".into()],
            validation_lanes: vec![csdlc_v2::cards::ValidationLane {
                lane: "focused".into(),
                proof_role: "Gate 10A".into(),
                acceptance_ids: vec!["AC-1".into()],
                deterministic: true,
                resource_profile: csdlc_v2::cards::ResourceProfile::Small,
                budget_seconds: 120,
                budget_tokens: 1000,
                argv: vec!["cargo".into(), "test".into()],
                parallel_group: "local".into(),
                defer_reason: None,
            }],
            failure_policy: "Fail closed.".into(),
            review_prompts: vec!["Review correctness.".into()],
            review_scope: "fixture".into(),
        },
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
    assert_eq!(manifest.skills.len(), 11);
    assert_eq!(
        resolve_operator_generation(&root.join(".."), 5294, None).unwrap(),
        selector.default_generation
    );
    assert!(resolve_operator_generation(&root.join(".."), 5294, Some(Generation::V1)).is_err());
    for text in [&root_agents, &nested_agents] {
        assert!(text.contains("v1"));
        assert!(text.contains("csdlc-install"));
        assert!(text.contains("eleven"));
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
    let required = manifest.required_binaries();
    for name in required.iter().take(required.len() - 1) {
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
    let parent = tempfile::tempdir().unwrap();
    let bins = parent.path().join("csdlc-v2");
    install_binaries(prebuilt_binaries(), &bins).unwrap();
    stamp_current_revision(&repo, &bins);
    fs::remove_file(bins.join("csdlc-init")).unwrap();
    symlink("/bin/true", bins.join("csdlc-init")).unwrap();
    let report = verify_coexistence(&repo, &bins, &CoexistenceInventory::load().unwrap()).unwrap();
    assert!(!report.pass);
    assert_eq!(report.missing_v2_binaries, vec!["csdlc-init"]);
}

fn prebuilt_binaries() -> &'static std::path::Path {
    std::path::Path::new(env!("CARGO_BIN_EXE_csdlc-install"))
        .parent()
        .expect("Cargo binary directory")
}

fn stamp_current_revision(repo: &std::path::Path, bins: &std::path::Path) {
    let output = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .current_dir(repo)
        .output()
        .unwrap();
    assert!(output.status.success());
    let revision = String::from_utf8(output.stdout).unwrap();
    let receipt_path = bins.join("install-receipt.json");
    let mut receipt: serde_json::Value =
        serde_json::from_slice(&fs::read(&receipt_path).unwrap()).unwrap();
    receipt["source_revision"] = serde_json::Value::String(format!("git:{}", revision.trim()));
    fs::write(receipt_path, serde_json::to_vec_pretty(&receipt).unwrap()).unwrap();
}
