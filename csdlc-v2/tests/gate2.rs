use std::fs;

use csdlc_v2::cards::CardContent;
use csdlc_v2::doctor::DoctorStatus;
use csdlc_v2::test_support::{
    bind_issue, initialize_native_json, reacquire_claim, BindRequest, BootstrapRequest,
    ReacquireClaimRequest,
};
use csdlc_v2::{
    amend_claim_scope, diagnose, edit_issue, AmendClaimScopeRequest, CardKind, Claim, EditRequest,
    ErrorCode, LifecyclePhase, PlanningCollectionField, SemanticOperation, Store,
};
use tempfile::TempDir;

fn git(root: &std::path::Path, args: &[&str]) {
    let output = std::process::Command::new("git")
        .current_dir(root)
        .args(args)
        .output()
        .expect("git");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

fn initialize_issue(
    store: &Store,
    request: BootstrapRequest,
) -> csdlc_v2::Result<csdlc_v2::IssueRecord> {
    if !store.root().join(".git").exists() {
        git(store.root(), &["init", "-b", "main"]);
    }
    let registry = store.root().join("docs/templates/prompts/current.json");
    let manifest = store
        .root()
        .join("csdlc-v2/operator/native-card-shape.json");
    fs::create_dir_all(registry.parent().expect("registry parent")).expect("registry dir");
    fs::create_dir_all(manifest.parent().expect("manifest parent")).expect("manifest dir");
    fs::write(
        &registry,
        include_bytes!("../../docs/templates/prompts/current.json"),
    )
    .expect("registry fixture");
    fs::write(
        &manifest,
        include_bytes!("../operator/native-card-shape.json"),
    )
    .expect("manifest fixture");
    let bytes = serde_json::to_vec(&request).expect("native request bytes");
    initialize_native_json(store, &bytes)
}

fn request() -> BootstrapRequest {
    BootstrapRequest {
        issue: 42,
        repository: "example/repo".into(),
        design_path: "docs/design.md".into(),
        diagram_path: "docs/diagram.mmd".into(),
        design_reviewer: "reviewer".into(),
        design_approved: true,
        claim: Claim {
            id: "claim-1".into(),
            owner: "agent".into(),
            generation: 0,
            acquired_unix_seconds: 1,
            expires_unix_seconds: u64::MAX,
            heartbeat_unix_seconds: 1,
            branch: "issue-42".into(),
            worktree: ".worktrees/issue-42".into(),
            protected_paths: vec!["src".into()],
            purpose: "test".into(),
        },
        initial: csdlc_v2::InitialCardInput {
            title: "Gate 2 fixture".into(),
            slug: "gate-2-fixture".into(),
            version: "v0.91.7".into(),
            goal: "Prove Gate 2.".into(),
            required_outcome: "Construct and validate six typed cards.".into(),
            declared_scope: vec!["fixture record".into()],
            authority_boundary: vec!["no network".into()],
            operator_constraints: vec!["none".into()],
            task_boundary: "Implement only the fixture.".into(),
            deliverables: vec!["record".into()],
            acceptance_criteria: vec!["six cards exist".into(), "doctor is ready".into()],
            dependencies: vec!["none".into()],
            repo_inputs: vec!["docs/design.md".into()],
            non_goals: vec!["GitHub".into()],
            plan_summary: "Build then diagnose.".into(),
            steps: vec![csdlc_v2::cards::PlanStep {
                id: "step-1".into(),
                action: "construct and diagnose".into(),
                acceptance_ids: vec!["AC-1".into(), "AC-2".into()],
                status: csdlc_v2::cards::StepStatus::Pending,
            }],
            invariants: vec!["atomic record".into()],
            risks: vec!["interruption".into()],
            planning_profile: csdlc_v2::PlanningProfile::Small,
            stop_conditions: vec!["invariant failure".into()],
            validation_lanes: vec![csdlc_v2::cards::ValidationLane {
                lane: "focused".into(),
                proof_role: "Gate 2 behavior".into(),
                acceptance_ids: vec!["AC-1".into(), "AC-2".into()],
                deterministic: true,
                resource_profile: csdlc_v2::cards::ResourceProfile::Small,
                budget_seconds: 120,
                budget_tokens: 1_000,
                argv: vec!["cargo".into(), "test".into()],
                parallel_group: "local".into(),
                defer_reason: None,
            }],
            failure_policy: "Fail closed.".into(),
            review_prompts: vec!["Review correctness.".into()],
            review_scope: "fixture".into(),
        },
        prepared_cards: None,
    }
}

fn fixture() -> (TempDir, Store, csdlc_v2::IssueRecord) {
    let temp = tempfile::tempdir().expect("tempdir");
    fs::create_dir_all(temp.path().join("docs")).expect("docs");
    fs::write(temp.path().join("docs/design.md"), "# Reviewed design\n").expect("design");
    fs::write(
        temp.path().join("docs/diagram.mmd"),
        "flowchart LR\n  A --> B\n",
    )
    .expect("diagram");
    let store = Store::new(temp.path());
    let request = request();
    let record = initialize_issue(&store, request.clone()).expect("initialize");
    assert!(temp.path().join("docs/design.md").exists());
    assert!(temp.path().join("docs/diagram.mmd").exists());
    assert_eq!(
        initialize_issue(&store, request).expect("idempotent init"),
        record
    );
    (temp, store, record)
}

fn bind_fixture() -> (TempDir, Store, csdlc_v2::IssueRecord) {
    let (temp, store, record) = fixture();
    git(temp.path(), &["add", "."]);
    git(temp.path(), &["commit", "-m", "fixture"]);
    let claim = record.claim.clone().expect("claim");
    bind_issue(
        &store,
        BindRequest {
            issue: 42,
            base_branch: "main".into(),
            branch: claim.branch.clone(),
            worktree: claim.worktree.clone(),
            claim,
        },
    )
    .expect("bind fixture");
    let bound_store = Store::new(temp.path().join(".worktrees/issue-42"));
    let bound = bound_store.load_record(42).expect("bound record");
    (temp, bound_store, bound)
}

fn bind_issue_5337_fixture() -> (TempDir, Store, csdlc_v2::IssueRecord) {
    let temp = tempfile::tempdir().expect("tempdir");
    fs::create_dir_all(temp.path().join("docs")).expect("docs");
    fs::write(temp.path().join("docs/design.md"), "# Prepared design\n").expect("design");
    fs::write(
        temp.path().join("docs/diagram.mmd"),
        "flowchart LR\n  Prepare --> Implement\n",
    )
    .expect("diagram");
    let store = Store::new(temp.path());
    let mut request = request();
    request.issue = 5_337;
    request.claim.id = "claim-5337".into();
    request.claim.branch = "issue-5337".into();
    request.claim.worktree = ".worktrees/issue-5337".into();
    request.initial.title = "[v0.91.8][WP-03] Prepared characterization corpus".into();
    request.initial.slug = "prepared-characterization-corpus".into();
    request.initial.goal =
        "Prepare the characterization issue without implementation claims.".into();
    let record = initialize_issue(&store, request).expect("initialize #5337 fixture");
    git(temp.path(), &["add", "."]);
    git(temp.path(), &["commit", "-m", "prepared #5337 fixture"]);
    let claim = record.claim.clone().expect("claim");
    bind_issue(
        &store,
        BindRequest {
            issue: 5_337,
            base_branch: "main".into(),
            branch: claim.branch.clone(),
            worktree: claim.worktree.clone(),
            claim,
        },
    )
    .expect("bind #5337 fixture");
    let bound_store = Store::new(temp.path().join(".worktrees/issue-5337"));
    let bound = bound_store.load_record(5_337).expect("bound #5337 record");
    (temp, bound_store, bound)
}

fn edit_current(
    store: &Store,
    record: &csdlc_v2::IssueRecord,
    card: CardKind,
    operation: SemanticOperation,
) -> csdlc_v2::IssueRecord {
    edit_issue(
        store,
        EditRequest {
            issue: record.issue,
            card,
            expected_generation: record.generation,
            expected_digest: record.digest.clone(),
            claim_id: record.claim.as_ref().expect("claim").id.clone(),
            actor: "agent".into(),
            reason: "typed preparation-to-implementation replan".into(),
            operation,
            fail_after_backup: false,
        },
    )
    .expect("typed edit")
}

fn cli_edit_current(
    root: &std::path::Path,
    store: &Store,
    record: &csdlc_v2::IssueRecord,
    card: CardKind,
    operation: SemanticOperation,
) -> csdlc_v2::IssueRecord {
    let request = EditRequest {
        issue: record.issue,
        card,
        expected_generation: record.generation,
        expected_digest: record.digest.clone(),
        claim_id: record.claim.as_ref().expect("claim").id.clone(),
        actor: "agent".into(),
        reason: "typed preparation-to-implementation CLI replan".into(),
        operation,
        fail_after_backup: false,
    };
    let request_path = root.join("typed-replan.json");
    fs::write(
        &request_path,
        serde_json::to_vec(&request).expect("CLI request JSON"),
    )
    .expect("write CLI request");
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_csdlc-edit"))
        .args([
            "--repo",
            root.to_str().expect("repo path"),
            "apply",
            "--request",
            request_path.to_str().expect("request path"),
        ])
        .output()
        .expect("run csdlc-edit");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    store.load_record(record.issue).expect("CLI-updated record")
}

fn implemented_fixture() -> (TempDir, Store, csdlc_v2::IssueRecord) {
    let (temp, store, mut record) = bind_fixture();
    record = edit_current(
        &store,
        &record,
        CardKind::Sor,
        SemanticOperation::RecordExecution {
            summary: "implemented".into(),
            changes: vec!["csdlc-v2".into()],
            artifacts: vec!["focused tests".into()],
        },
    );
    record = edit_current(
        &store,
        &record,
        CardKind::Sip,
        SemanticOperation::AdvancePhase {
            phase: LifecyclePhase::Implemented,
        },
    );
    assert_eq!(record.phase, LifecyclePhase::Implemented);
    (temp, store, record)
}

fn spp_replacement_request(
    record: &csdlc_v2::IssueRecord,
    operation: SemanticOperation,
) -> EditRequest {
    EditRequest {
        issue: 42,
        card: CardKind::Spp,
        expected_generation: record.generation,
        expected_digest: record.digest.clone(),
        claim_id: "claim-1".into(),
        actor: "agent".into(),
        reason: "bounded implemented review remediation".into(),
        operation,
        fail_after_backup: false,
    }
}

fn replacement_steps() -> Vec<csdlc_v2::cards::PlanStep> {
    vec![csdlc_v2::cards::PlanStep {
        id: "correct-review-finding".into(),
        action: "correct the SPP contradiction found during exact review".into(),
        acceptance_ids: vec!["AC-1".into(), "AC-2".into()],
        status: csdlc_v2::cards::StepStatus::Pending,
    }]
}

#[test]
fn bootstrap_rejects_missing_vpp_command_before_authoring_files() {
    let temp = tempfile::tempdir().expect("tempdir");
    let store = Store::new(temp.path());
    let mut request = request();
    request.initial.validation_lanes[0].argv = vec![
        "bash".into(),
        "adl/tools/validate_planning_templates.sh".into(),
    ];
    let error = initialize_issue(&store, request).expect_err("missing command");
    assert!(matches!(error.code, ErrorCode::InvalidInput));
    assert!(!temp.path().join("docs/design.md").exists());
    assert!(!temp.path().join(".csdlc/issues/42").exists());
}

#[test]
fn native_registry_is_required_and_shape_checked_before_issue_authoring() {
    for registry in [
        None,
        Some(b"not-json".as_slice()),
        Some(br#"{"generations":{}}"#.as_slice()),
        Some(include_bytes!("../../docs/templates/prompts/current.json").as_slice()),
    ] {
        let temp = tempfile::tempdir().expect("tempdir");
        fs::create_dir_all(temp.path().join("docs")).expect("docs");
        fs::write(temp.path().join("docs/design.md"), "# Design\n").expect("design");
        fs::write(
            temp.path().join("docs/diagram.mmd"),
            "flowchart LR\n A-->B\n",
        )
        .expect("diagram");
        if let Some(bytes) = registry {
            let path = temp.path().join("docs/templates/prompts/current.json");
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            fs::write(path, bytes).unwrap();
        }
        let bytes = serde_json::to_vec(&request()).expect("native request bytes");
        let error = initialize_native_json(&Store::new(temp.path()), &bytes)
            .expect_err("invalid registry must fail closed");
        assert!(matches!(error.code, ErrorCode::InvalidManifest));
        assert!(!temp.path().join(".csdlc/issues/42").exists());
    }
}

#[cfg(unix)]
#[test]
fn native_registry_and_manifest_symlink_escapes_fail_before_authoring() {
    use std::os::unix::fs::symlink;

    let bytes = serde_json::to_vec(&request()).expect("native request bytes");
    let escaped_registry = tempfile::tempdir().expect("registry root");
    let outside = tempfile::tempdir().expect("outside registry");
    let outside_registry = outside.path().join("current.json");
    fs::write(
        &outside_registry,
        include_bytes!("../../docs/templates/prompts/current.json"),
    )
    .unwrap();
    let registry_path = escaped_registry
        .path()
        .join("docs/templates/prompts/current.json");
    fs::create_dir_all(registry_path.parent().unwrap()).unwrap();
    symlink(&outside_registry, &registry_path).unwrap();
    let error = initialize_native_json(&Store::new(escaped_registry.path()), &bytes)
        .expect_err("registry symlink escape");
    assert!(matches!(error.code, ErrorCode::InvalidManifest));
    assert!(!escaped_registry.path().join(".csdlc/issues/42").exists());

    let escaped_manifest = tempfile::tempdir().expect("manifest root");
    let registry_path = escaped_manifest
        .path()
        .join("docs/templates/prompts/current.json");
    fs::create_dir_all(registry_path.parent().unwrap()).unwrap();
    fs::write(
        &registry_path,
        include_bytes!("../../docs/templates/prompts/current.json"),
    )
    .unwrap();
    let outside_manifest = outside.path().join("native-card-shape.json");
    fs::write(
        &outside_manifest,
        include_bytes!("../operator/native-card-shape.json"),
    )
    .unwrap();
    let manifest_path = escaped_manifest
        .path()
        .join("csdlc-v2/operator/native-card-shape.json");
    fs::create_dir_all(manifest_path.parent().unwrap()).unwrap();
    symlink(&outside_manifest, &manifest_path).unwrap();
    let error = initialize_native_json(&Store::new(escaped_manifest.path()), &bytes)
        .expect_err("manifest symlink escape");
    assert!(matches!(error.code, ErrorCode::InvalidManifest));
    assert!(!escaped_manifest.path().join(".csdlc/issues/42").exists());
}

#[test]
fn retained_bootstrap_without_new_fields_loads_as_explicit_none() {
    let mut value = serde_json::to_value(request()).expect("request JSON");
    let initial = value["initial"].as_object_mut().expect("initial object");
    initial.remove("operator_constraints");
    initial.remove("review_scope");
    let retained: BootstrapRequest =
        serde_json::from_value(value.clone()).expect("retained request");
    assert_eq!(retained.initial.operator_constraints, vec!["none"]);
    assert_eq!(retained.initial.review_scope, "none");
    let temp = tempfile::tempdir().expect("tempdir");
    let bytes = serde_json::to_vec(&value).expect("retained bytes");
    let error = initialize_native_json(&Store::new(temp.path()), &bytes)
        .expect_err("native entrypoint requires explicit fields");
    assert!(matches!(error.code, ErrorCode::InvalidInput));
    assert!(!temp.path().join(".csdlc/issues/42").exists());
}

#[test]
fn bootstrap_rejects_one_path_for_both_authored_artifact_roles() {
    let temp = tempfile::tempdir().expect("tempdir");
    let store = Store::new(temp.path());
    let mut request = request();
    request.diagram_path = request.design_path.clone();
    let error = initialize_issue(&store, request).expect_err("shared authored path");
    assert!(matches!(error.code, ErrorCode::InvalidInput));
    assert!(!temp.path().join("docs/design.md").exists());
    assert!(!temp.path().join(".csdlc/issues/42").exists());
}

#[test]
fn bind_creates_and_idempotently_reuses_typed_worktree() {
    let (temp, store, record) = fixture();
    git(temp.path(), &["init", "-b", "issue-43-terminal"]);
    git(
        temp.path(),
        &["config", "user.email", "test@example.invalid"],
    );
    git(temp.path(), &["config", "user.name", "C-SDLC Test"]);
    git(temp.path(), &["add", "docs"]);
    git(temp.path(), &["commit", "-m", "fixture"]);
    let claim = record.claim.clone().expect("claim");
    let request = BindRequest {
        issue: 42,
        base_branch: "main".into(),
        branch: claim.branch.clone(),
        worktree: claim.worktree.clone(),
        claim,
    };
    let first = bind_issue(&store, request.clone()).expect("bind");
    assert!(first.created);
    let bound_store = Store::new(temp.path().join(".worktrees/issue-42"));
    let bound_digest = bound_store.load_record(42).expect("bound record").digest;
    assert_eq!(
        bound_store.load_record(42).expect("bound record").phase,
        csdlc_v2::LifecyclePhase::Bound
    );
    let second = bind_issue(&store, request).expect("rebind");
    assert!(!second.created);
    assert_eq!(
        bound_store.load_record(42).expect("reused record").digest,
        bound_digest
    );
}

#[test]
fn bind_supports_issue_local_state_without_touching_primary_checkout() {
    let temp = tempfile::tempdir().unwrap();
    fs::create_dir_all(temp.path().join("docs")).unwrap();
    fs::write(temp.path().join("docs/design.md"), "# design\n").unwrap();
    fs::write(
        temp.path().join("docs/diagram.mmd"),
        "flowchart LR\n A-->B\n",
    )
    .unwrap();
    let mut initial = request();
    initial.claim.worktree = ".".into();
    let store = Store::new(temp.path());
    let record = initialize_issue(&store, initial).unwrap();
    git(temp.path(), &["init", "-b", "main"]);
    git(
        temp.path(),
        &["config", "user.email", "test@example.invalid"],
    );
    git(temp.path(), &["config", "user.name", "C-SDLC Test"]);
    git(temp.path(), &["add", "docs"]);
    git(temp.path(), &["commit", "-m", "fixture"]);
    git(temp.path(), &["switch", "-c", "issue-42"]);
    let claim = record.claim.clone().expect("claim");
    let request = BindRequest {
        issue: 42,
        base_branch: "main".into(),
        branch: "issue-42".into(),
        worktree: ".".into(),
        claim,
    };
    let result = bind_issue(&store, request).expect("issue-local bind");
    assert!(!result.created);
    assert_eq!(
        store.load_record(42).unwrap().phase,
        csdlc_v2::LifecyclePhase::Bound
    );
    assert_eq!(git_branch(temp.path()), "issue-42");
}

#[test]
fn bind_activates_exact_reserved_claim_from_existing_worktree() {
    let temp = tempfile::tempdir().unwrap();
    fs::create_dir_all(temp.path().join("docs")).unwrap();
    fs::write(temp.path().join("docs/design.md"), "# design\n").unwrap();
    fs::write(
        temp.path().join("docs/diagram.mmd"),
        "flowchart LR\n A-->B\n",
    )
    .unwrap();
    git(temp.path(), &["init", "-b", "main"]);
    git(
        temp.path(),
        &["config", "user.email", "test@example.invalid"],
    );
    git(temp.path(), &["config", "user.name", "C-SDLC Test"]);
    let store = Store::new(temp.path());
    initialize_issue(&store, request()).unwrap();
    git(temp.path(), &["add", "."]);
    git(temp.path(), &["commit", "-m", "prepared issue"]);
    git(
        temp.path(),
        &[
            "worktree",
            "add",
            "-b",
            "issue-42",
            ".worktrees/issue-42",
            "main",
        ],
    );

    let issue_root = temp.path().join(".worktrees/issue-42");
    let issue_store = Store::new(&issue_root);
    let claim = issue_store.load_record(42).unwrap().claim.unwrap();
    let result = bind_issue(
        &issue_store,
        BindRequest {
            issue: 42,
            base_branch: "main".into(),
            branch: claim.branch.clone(),
            worktree: claim.worktree.clone(),
            claim,
        },
    )
    .expect("activate exact reserved claim from its existing worktree");

    assert!(!result.created);
    assert_eq!(
        issue_store.load_record(42).unwrap().phase,
        csdlc_v2::LifecyclePhase::Bound
    );
}

#[test]
fn bind_rejects_reserved_worktree_that_does_not_match_current_checkout() {
    let temp = tempfile::tempdir().unwrap();
    fs::create_dir_all(temp.path().join("docs")).unwrap();
    fs::write(temp.path().join("docs/design.md"), "# design\n").unwrap();
    fs::write(
        temp.path().join("docs/diagram.mmd"),
        "flowchart LR\n A-->B\n",
    )
    .unwrap();
    let mut initial = request();
    initial.claim.worktree = ".worktrees/not-this-worktree".into();
    git(temp.path(), &["init", "-b", "main"]);
    git(
        temp.path(),
        &["config", "user.email", "test@example.invalid"],
    );
    git(temp.path(), &["config", "user.name", "C-SDLC Test"]);
    let store = Store::new(temp.path());
    initialize_issue(&store, initial).unwrap();
    git(temp.path(), &["add", "."]);
    git(temp.path(), &["commit", "-m", "prepared issue"]);
    git(
        temp.path(),
        &[
            "worktree",
            "add",
            "-b",
            "issue-42",
            ".worktrees/issue-42",
            "main",
        ],
    );

    let issue_root = temp.path().join(".worktrees/issue-42");
    let issue_store = Store::new(&issue_root);
    let claim = issue_store.load_record(42).unwrap().claim.unwrap();
    let error = bind_issue(
        &issue_store,
        BindRequest {
            issue: 42,
            base_branch: "main".into(),
            branch: claim.branch.clone(),
            worktree: claim.worktree.clone(),
            claim,
        },
    )
    .expect_err("mismatched reserved worktree must fail closed");

    assert_eq!(error.code, ErrorCode::UnsafeCheckout);
    assert_eq!(
        issue_store.load_record(42).unwrap().phase,
        csdlc_v2::LifecyclePhase::Initialized
    );
}

#[test]
fn bind_rejects_standalone_repository_with_matching_worktree_suffix() {
    let temp = tempfile::tempdir().unwrap();
    let issue_root = temp.path().join("unrelated/.worktrees/issue-42");
    fs::create_dir_all(issue_root.join("docs")).unwrap();
    fs::write(issue_root.join("docs/design.md"), "# design\n").unwrap();
    fs::write(
        issue_root.join("docs/diagram.mmd"),
        "flowchart LR\n A-->B\n",
    )
    .unwrap();
    git(&issue_root, &["init", "-b", "issue-42"]);
    git(
        &issue_root,
        &["config", "user.email", "test@example.invalid"],
    );
    git(&issue_root, &["config", "user.name", "C-SDLC Test"]);
    let store = Store::new(&issue_root);
    let record = initialize_issue(&store, request()).unwrap();
    git(&issue_root, &["add", "."]);
    git(&issue_root, &["commit", "-m", "copied prepared issue"]);
    let claim = record.claim.unwrap();

    let error = bind_issue(
        &store,
        BindRequest {
            issue: 42,
            base_branch: "main".into(),
            branch: claim.branch.clone(),
            worktree: claim.worktree.clone(),
            claim,
        },
    )
    .expect_err("matching path suffix in an unrelated repository must fail closed");

    assert_eq!(error.code, ErrorCode::UnsafeCheckout);
    assert_eq!(
        store.load_record(42).unwrap().phase,
        csdlc_v2::LifecyclePhase::Initialized
    );
}

#[test]
fn closed_issue_claim_release_is_typed_and_compare_and_swap_guarded() {
    let (temp, store, mut record) = fixture();
    git(temp.path(), &["init", "-b", "main"]);
    git(
        temp.path(),
        &["config", "user.email", "test@example.invalid"],
    );
    git(temp.path(), &["config", "user.name", "C-SDLC Test"]);
    git(temp.path(), &["add", "docs"]);
    git(temp.path(), &["commit", "-m", "fixture"]);
    record = csdlc_v2::edit_issue(
        &store,
        edit(
            &record,
            SemanticOperation::AdvancePhase {
                phase: csdlc_v2::LifecyclePhase::Ready,
            },
        ),
    )
    .unwrap();
    record = csdlc_v2::edit_issue(
        &store,
        edit(
            &record,
            SemanticOperation::AdvancePhase {
                phase: csdlc_v2::LifecyclePhase::Bound,
            },
        ),
    )
    .unwrap();
    record = csdlc_v2::edit_issue(
        &store,
        EditRequest {
            issue: 42,
            card: CardKind::Sor,
            expected_generation: record.generation,
            expected_digest: record.digest.clone(),
            claim_id: "claim-1".into(),
            actor: "agent".into(),
            reason: "test edit".into(),
            operation: SemanticOperation::RecordExecution {
                summary: "done".into(),
                changes: vec!["claim".into()],
                artifacts: vec![],
            },
            fail_after_backup: false,
        },
    )
    .unwrap();
    record = csdlc_v2::edit_issue(
        &store,
        edit(
            &record,
            SemanticOperation::AdvancePhase {
                phase: csdlc_v2::LifecyclePhase::Implemented,
            },
        ),
    )
    .unwrap();
    let claim_id = record.claim.as_ref().unwrap().id.clone();
    let evidence = csdlc_v2::release_closed_claim(
        &store,
        csdlc_v2::ReleaseClosedClaimRequest {
            issue: 42,
            repository: "example/repo".into(),
            expected_claim_id: claim_id,
            expected_generation: record.generation,
            expected_digest: record.digest.clone(),
            actor: "operator".into(),
            reason: "GitHub issue is closed; release stale broad claim for follow-on setup".into(),
            observed_issue_state: "closed".into(),
            observed_issue: 42,
            observation_source: "github://example/repo/issues/42".into(),
        },
    )
    .unwrap();
    assert_eq!(evidence.previous_owner, "agent");
    let released = store.load_record(42).unwrap();
    assert!(released.claim.is_none());
    assert!(released
        .audit
        .last()
        .unwrap()
        .operation
        .contains("release_closed_claim"));
    assert_eq!(csdlc_v2::diagnose(&store, 42).status, DoctorStatus::Pass);
}

#[test]
fn active_claim_transition_atomically_updates_purpose_and_scope() {
    let (_temp, store, mut record) = fixture();
    record = csdlc_v2::edit_issue(
        &store,
        edit(
            &record,
            SemanticOperation::AdvancePhase {
                phase: csdlc_v2::LifecyclePhase::Ready,
            },
        ),
    )
    .unwrap();
    record = csdlc_v2::edit_issue(
        &store,
        edit(
            &record,
            SemanticOperation::AdvancePhase {
                phase: csdlc_v2::LifecyclePhase::Bound,
            },
        ),
    )
    .unwrap();
    let before_audit = record.audit.len();
    let transitioned = csdlc_v2::transition_active_claim(
        &store,
        csdlc_v2::TransitionActiveClaimRequest {
            issue: 42,
            claim_id: "claim-1".into(),
            expected_owner: "agent".into(),
            expected_generation: record.generation,
            expected_digest: record.digest.clone(),
            now_unix_seconds: 2,
            actor: "agent".into(),
            reason: "begin implementation".into(),
            expected_purpose: "test".into(),
            purpose: "Implement the accepted issue contract".into(),
            add_protected_paths: vec!["src".into(), "tests".into()],
        },
    )
    .unwrap();
    assert_eq!(
        transitioned.purpose,
        "Implement the accepted issue contract"
    );
    assert!(transitioned.protected_paths.contains(&"src".into()));
    let updated = store.load_record(42).unwrap();
    assert_eq!(updated.audit.len(), before_audit + 1);
    assert!(updated
        .audit
        .last()
        .unwrap()
        .operation
        .contains("transition_active_claim"));
    let audit: serde_json::Value =
        serde_json::from_str(&updated.audit.last().unwrap().operation).unwrap();
    assert_eq!(audit["expected_purpose"], "test");
    assert_eq!(audit["purpose"], "Implement the accepted issue contract");
    assert_eq!(
        audit["add_protected_paths"],
        serde_json::json!(["src", "tests"])
    );
}

#[test]
fn claim_revoke_clears_unexpired_claim_with_operator_cas_audit() {
    let (_temp, store, record) = fixture();
    let claim_id = record.claim.as_ref().expect("claim").id.clone();
    let result = csdlc_v2::revoke_active_claim(
        &store,
        csdlc_v2::RevokeActiveClaimRequest {
            issue: 42,
            repository: "example/repo".into(),
            expected_claim_id: claim_id.clone(),
            expected_generation: record.generation,
            expected_digest: record.digest.clone(),
            now_unix_seconds: 2,
            actor: "operator".into(),
            operator_authority: "operator-authorized:5648".into(),
            reason: "release abandoned setup claim before lease expiry".into(),
        },
    )
    .expect("revoke");
    assert_eq!(result.claim_id, claim_id);
    assert_eq!(result.previous_owner, "agent");
    assert!(result.released);
    assert_eq!(result.generation, record.generation);
    let released = store.load_record(42).expect("record");
    assert!(released.claim.is_none());
    assert_eq!(released.phase, record.phase);
    assert_eq!(released.digest, result.digest);
    assert!(!released
        .claim
        .as_ref()
        .is_some_and(|claim| claim.protected_paths.iter().any(|path| path == "src")));
    assert!(released
        .audit
        .last()
        .expect("audit")
        .operation
        .contains("revoke_active_claim"));
    assert!(released
        .audit
        .last()
        .expect("audit")
        .operation
        .contains("operator-authorized:5648"));
}

#[test]
fn claim_revoke_fails_closed_for_stale_digest_and_missing_authority() {
    let (_temp, store, record) = fixture();
    let claim_id = record.claim.as_ref().expect("claim").id.clone();
    let stale = csdlc_v2::revoke_active_claim(
        &store,
        csdlc_v2::RevokeActiveClaimRequest {
            issue: 42,
            repository: "example/repo".into(),
            expected_claim_id: claim_id.clone(),
            expected_generation: record.generation,
            expected_digest: "stale".into(),
            now_unix_seconds: 2,
            actor: "operator".into(),
            operator_authority: "operator-authorized:5648".into(),
            reason: "stale request".into(),
        },
    )
    .expect_err("stale digest");
    assert!(matches!(stale.code, ErrorCode::StaleDigest));
    let missing_authority = csdlc_v2::revoke_active_claim(
        &store,
        csdlc_v2::RevokeActiveClaimRequest {
            issue: 42,
            repository: "example/repo".into(),
            expected_claim_id: claim_id,
            expected_generation: record.generation,
            expected_digest: record.digest.clone(),
            now_unix_seconds: 2,
            actor: "operator".into(),
            operator_authority: " ".into(),
            reason: "missing authority".into(),
        },
    )
    .expect_err("authority required");
    assert!(matches!(missing_authority.code, ErrorCode::InvalidInput));

    let stale_generation = csdlc_v2::revoke_active_claim(
        &store,
        csdlc_v2::RevokeActiveClaimRequest {
            issue: 42,
            repository: "example/repo".into(),
            expected_claim_id: "claim-1".into(),
            expected_generation: record.generation + 1,
            expected_digest: record.digest.clone(),
            now_unix_seconds: 2,
            actor: "operator".into(),
            operator_authority: "operator-authorized:5648".into(),
            reason: "stale generation".into(),
        },
    )
    .expect_err("stale generation");
    assert!(matches!(stale_generation.code, ErrorCode::StaleGeneration));

    let claim_mismatch = csdlc_v2::revoke_active_claim(
        &store,
        csdlc_v2::RevokeActiveClaimRequest {
            issue: 42,
            repository: "example/repo".into(),
            expected_claim_id: "wrong-claim".into(),
            expected_generation: record.generation,
            expected_digest: record.digest,
            now_unix_seconds: 2,
            actor: "operator".into(),
            operator_authority: "operator-authorized:5648".into(),
            reason: "claim mismatch".into(),
        },
    )
    .expect_err("claim mismatch");
    assert!(matches!(claim_mismatch.code, ErrorCode::InvalidClaim));
}

#[test]
fn claim_revoke_requires_unexpired_claim() {
    let (_temp, store, record) = fixture();
    let error = csdlc_v2::revoke_active_claim(
        &store,
        csdlc_v2::RevokeActiveClaimRequest {
            issue: 42,
            repository: "example/repo".into(),
            expected_claim_id: "claim-1".into(),
            expected_generation: record.generation,
            expected_digest: record.digest,
            now_unix_seconds: u64::MAX,
            actor: "operator".into(),
            operator_authority: "operator-authorized:5648".into(),
            reason: "expired request must route to recovery".into(),
        },
    )
    .expect_err("expired claim");
    assert!(matches!(error.code, ErrorCode::ExpiredClaim));
}

#[test]
fn active_claim_transition_rejects_stale_owner_without_any_write() {
    let (_temp, store, mut record) = fixture();
    record = csdlc_v2::edit_issue(
        &store,
        edit(
            &record,
            SemanticOperation::AdvancePhase {
                phase: csdlc_v2::LifecyclePhase::Ready,
            },
        ),
    )
    .unwrap();
    record = csdlc_v2::edit_issue(
        &store,
        edit(
            &record,
            SemanticOperation::AdvancePhase {
                phase: csdlc_v2::LifecyclePhase::Bound,
            },
        ),
    )
    .unwrap();
    let before = std::fs::read(store.root().join(".csdlc/issues/42/index.json")).unwrap();
    let error = csdlc_v2::transition_active_claim(
        &store,
        csdlc_v2::TransitionActiveClaimRequest {
            issue: 42,
            claim_id: "claim-1".into(),
            expected_owner: "stale-owner".into(),
            expected_generation: record.generation,
            expected_digest: record.digest,
            now_unix_seconds: 2,
            actor: "agent".into(),
            reason: "begin implementation".into(),
            expected_purpose: "test".into(),
            purpose: "Implement the accepted issue contract".into(),
            add_protected_paths: vec!["src".into()],
        },
    )
    .unwrap_err();
    assert_eq!(error.code, ErrorCode::InvalidClaim);
    assert_eq!(
        std::fs::read(store.root().join(".csdlc/issues/42/index.json")).unwrap(),
        before
    );
}

#[test]
fn active_claim_transition_guards_cas_expiry_collision_and_real_cli() {
    let (temp, store, mut record) = fixture();
    record = csdlc_v2::edit_issue(
        &store,
        edit(
            &record,
            SemanticOperation::AdvancePhase {
                phase: csdlc_v2::LifecyclePhase::Ready,
            },
        ),
    )
    .unwrap();
    record = csdlc_v2::edit_issue(
        &store,
        edit(
            &record,
            SemanticOperation::AdvancePhase {
                phase: csdlc_v2::LifecyclePhase::Bound,
            },
        ),
    )
    .unwrap();
    let base = csdlc_v2::TransitionActiveClaimRequest {
        issue: 42,
        claim_id: "claim-1".into(),
        expected_owner: "agent".into(),
        expected_generation: record.generation,
        expected_digest: record.digest.clone(),
        now_unix_seconds: 2,
        actor: "agent".into(),
        reason: "begin implementation".into(),
        expected_purpose: "test".into(),
        purpose: "implementation".into(),
        add_protected_paths: vec!["product".into()],
    };
    let index = store.issue_dir(42).join("index.json");
    let before = fs::read(&index).unwrap();
    let mut stale_generation = base.clone();
    stale_generation.expected_generation += 1;
    assert_eq!(
        csdlc_v2::transition_active_claim(&store, stale_generation)
            .unwrap_err()
            .code,
        ErrorCode::StaleGeneration
    );
    assert_eq!(fs::read(&index).unwrap(), before);
    let mut stale_digest = base.clone();
    stale_digest.expected_digest = "stale".into();
    assert_eq!(
        csdlc_v2::transition_active_claim(&store, stale_digest)
            .unwrap_err()
            .code,
        ErrorCode::StaleDigest
    );
    let mut wrong_source = base.clone();
    wrong_source.expected_purpose = "already implementation".into();
    assert_eq!(
        csdlc_v2::transition_active_claim(&store, wrong_source)
            .unwrap_err()
            .code,
        ErrorCode::InvalidClaim
    );
    let mut expired = base.clone();
    expired.now_unix_seconds = u64::MAX;
    assert_eq!(
        csdlc_v2::transition_active_claim(&store, expired)
            .unwrap_err()
            .code,
        ErrorCode::ExpiredClaim
    );
    assert_eq!(fs::read(&index).unwrap(), before);

    let mut other = record.clone();
    other.issue = 43;
    let other_claim = other.claim.as_mut().unwrap();
    other_claim.id = "claim-43".into();
    other_claim.branch = "main".into();
    other_claim.worktree = ".".into();
    other_claim.protected_paths = vec!["product/nested".into()];
    fs::create_dir_all(store.issue_dir(43)).unwrap();
    fs::write(
        store.issue_dir(43).join("index.json"),
        serde_json::to_vec(&other).unwrap(),
    )
    .unwrap();
    assert_eq!(
        csdlc_v2::transition_active_claim(&store, base.clone())
            .unwrap_err()
            .code,
        ErrorCode::ClaimCollision
    );
    assert_eq!(fs::read(&index).unwrap(), before);
    fs::remove_dir_all(store.issue_dir(43)).unwrap();

    let request_path = temp.path().join("transition.json");
    fs::write(&request_path, serde_json::to_vec(&base).unwrap()).unwrap();
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_csdlc-bind"))
        .args([
            "--root",
            temp.path().to_str().unwrap(),
            "--transition-request",
            request_path.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let updated = store.load_record(42).unwrap();
    assert_eq!(updated.claim.unwrap().purpose, "implementation");
}

fn git_branch(root: &std::path::Path) -> String {
    let output = std::process::Command::new("git")
        .current_dir(root)
        .args(["branch", "--show-current"])
        .output()
        .unwrap();
    String::from_utf8(output.stdout).unwrap().trim().into()
}

#[test]
fn bind_refuses_primary_checkout_and_worktree_mismatch() {
    let (temp, store, record) = fixture();
    git(temp.path(), &["checkout", "-b", "wrong"]);
    git(
        temp.path(),
        &["config", "user.email", "test@example.invalid"],
    );
    git(temp.path(), &["config", "user.name", "C-SDLC Test"]);
    git(temp.path(), &["add", "docs"]);
    git(temp.path(), &["commit", "-m", "fixture"]);
    let claim = record.claim.clone().expect("claim");
    let error = bind_issue(
        &store,
        BindRequest {
            issue: 42,
            base_branch: "main".into(),
            branch: claim.branch.clone(),
            worktree: claim.worktree.clone(),
            claim,
        },
    )
    .expect_err("unsafe checkout");
    assert!(matches!(error.code, ErrorCode::UnsafeCheckout));
}

#[test]
fn bind_refuses_branch_at_a_different_worktree() {
    let (temp, store, record) = fixture();
    git(temp.path(), &["init", "-b", "main"]);
    git(
        temp.path(),
        &["config", "user.email", "test@example.invalid"],
    );
    git(temp.path(), &["config", "user.name", "C-SDLC Test"]);
    git(temp.path(), &["add", "docs"]);
    git(temp.path(), &["commit", "-m", "fixture"]);
    git(
        temp.path(),
        &[
            "worktree",
            "add",
            "-b",
            "issue-42",
            ".worktrees/other",
            "main",
        ],
    );
    let claim = record.claim.clone().expect("claim");
    let error = bind_issue(
        &store,
        BindRequest {
            issue: 42,
            base_branch: "main".into(),
            branch: claim.branch.clone(),
            worktree: claim.worktree.clone(),
            claim,
        },
    )
    .expect_err("topology mismatch");
    assert!(matches!(error.code, ErrorCode::ClaimCollision));
}

#[test]
fn bind_refuses_overlapping_protected_path_reserved_by_another_issue() {
    let (temp, store, record) = fixture();
    git(temp.path(), &["init", "-b", "main"]);
    git(
        temp.path(),
        &["config", "user.email", "test@example.invalid"],
    );
    git(temp.path(), &["config", "user.name", "C-SDLC Test"]);
    git(temp.path(), &["add", "docs"]);
    git(temp.path(), &["commit", "-m", "fixture"]);
    let mut other = record.clone();
    other.issue = 43;
    let other_claim = other.claim.as_mut().expect("claim");
    other_claim.branch = "main".into();
    other_claim.worktree = ".".into();
    other_claim.protected_paths = vec!["src/nested".into()];
    fs::create_dir_all(store.issue_dir(43)).expect("other issue");
    fs::write(
        store.issue_dir(43).join("index.json"),
        serde_json::to_vec(&other).expect("json"),
    )
    .expect("other record");
    let claim = record.claim.clone().expect("claim");
    let error = bind_issue(
        &store,
        BindRequest {
            issue: 42,
            base_branch: "main".into(),
            branch: claim.branch.clone(),
            worktree: claim.worktree.clone(),
            claim,
        },
    )
    .expect_err("path overlap");
    assert!(matches!(error.code, ErrorCode::ClaimCollision));
    assert!(error.message.contains("in phase"));
    assert!(error.message.contains("protected"));
}

#[test]
fn heartbeat_is_compare_and_swap_and_missed_heartbeat_does_not_enable_recovery() {
    let (_temp, store, record) = fixture();
    let error = csdlc_v2::heartbeat_claim(&store, 42, "wrong", 0, 2, 60).expect_err("wrong owner");
    assert!(matches!(error.code, ErrorCode::InvalidClaim));
    let replacement = Claim {
        id: "replacement".into(),
        owner: "next".into(),
        generation: 0,
        acquired_unix_seconds: 2,
        expires_unix_seconds: u64::MAX,
        heartbeat_unix_seconds: 2,
        branch: "issue-42".into(),
        worktree: ".worktrees/issue-42".into(),
        protected_paths: vec!["src".into()],
        purpose: "recover".into(),
    };
    let error = csdlc_v2::recover_claim(
        &store,
        csdlc_v2::RecoverClaimRequest {
            issue: 42,
            expected_claim_id: record.claim.expect("claim").id,
            expected_generation: 0,
            now_unix_seconds: 3,
            replacement,
            recovery_actor: "operator".into(),
            reason: "explicit recovery".into(),
        },
    )
    .expect_err("not expired");
    assert!(matches!(error.code, ErrorCode::InvalidClaim));
}

#[test]
fn heartbeat_and_expired_recovery_record_positive_evidence() {
    let (temp, store, record) = fixture();
    csdlc_v2::heartbeat_claim(&store, 42, "claim-1", 0, 2, 60).expect("heartbeat");
    let replacement = Claim {
        id: "replacement".into(),
        owner: "next".into(),
        generation: 0,
        acquired_unix_seconds: 62,
        expires_unix_seconds: u64::MAX,
        heartbeat_unix_seconds: 62,
        branch: "issue-42".into(),
        worktree: ".".into(),
        protected_paths: vec!["src".into()],
        purpose: "explicit recovery".into(),
    };
    let wrong_checkout = csdlc_v2::recover_claim(
        &store,
        csdlc_v2::RecoverClaimRequest {
            issue: 42,
            expected_claim_id: record.claim.as_ref().expect("claim").id.clone(),
            expected_generation: 0,
            now_unix_seconds: 62,
            replacement: replacement.clone(),
            recovery_actor: "operator".into(),
            reason: "lease expired".into(),
        },
    )
    .expect_err("wrong checkout");
    assert_eq!(wrong_checkout.code, ErrorCode::UnsafeCheckout);
    git(temp.path(), &["branch", "-m", "issue-42"]);
    let evidence = csdlc_v2::recover_claim(
        &store,
        csdlc_v2::RecoverClaimRequest {
            issue: 42,
            expected_claim_id: record.claim.expect("claim").id,
            expected_generation: 0,
            now_unix_seconds: 62,
            replacement,
            recovery_actor: "operator".into(),
            reason: "lease expired".into(),
        },
    )
    .expect("recover");
    assert_eq!(evidence.previous_owner, "agent");
    assert_eq!(evidence.observed_expiry_unix_seconds, 62);
    assert_eq!(
        store
            .load_record(42)
            .expect("record")
            .claim
            .expect("claim")
            .owner,
        "next"
    );
}

fn reacquired_claim(generation: u64) -> Claim {
    Claim {
        id: "claim-reacquired".into(),
        owner: "next-owner".into(),
        generation,
        acquired_unix_seconds: 10,
        expires_unix_seconds: u64::MAX,
        heartbeat_unix_seconds: 10,
        branch: "issue-42".into(),
        worktree: ".".into(),
        protected_paths: vec!["src".into()],
        purpose: "resume dormant issue".into(),
    }
}

#[test]
fn released_claim_reacquires_without_phase_or_audit_rewind() {
    let (temp, store, record) = fixture();
    git(temp.path(), &["branch", "-m", "issue-42"]);
    let released = csdlc_v2::revoke_active_claim(
        &store,
        csdlc_v2::RevokeActiveClaimRequest {
            issue: 42,
            repository: "example/repo".into(),
            expected_claim_id: "claim-1".into(),
            expected_generation: record.generation,
            expected_digest: record.digest,
            now_unix_seconds: 2,
            actor: "operator".into(),
            operator_authority: "operator-authorized:5727".into(),
            reason: "deliberately release dormant preparation".into(),
        },
    )
    .expect("release");
    let dormant = store.load_record(42).expect("dormant");
    let dormant_audit_len = dormant.audit.len();
    let doctor = diagnose(&store, 42);
    assert_eq!(doctor.status, DoctorStatus::Block);
    assert_eq!(
        doctor.next_operation.as_deref(),
        Some("csdlc-bind --recover-request <request.json>")
    );

    let result = reacquire_claim(
        &store,
        ReacquireClaimRequest {
            issue: 42,
            expected_generation: dormant.generation,
            expected_digest: released.digest,
            now_unix_seconds: 10,
            actor: "next-owner".into(),
            reason: "resume accepted work".into(),
            replacement: reacquired_claim(dormant.generation),
        },
    )
    .expect("historical authority repair remains library-only");
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_csdlc-bind"))
        .args([
            "--root",
            temp.path().to_str().expect("root"),
            "--reacquire-request",
            "removed.json",
        ])
        .output()
        .expect("run csdlc-bind");
    assert!(!output.status.success());
    let resumed = store.load_record(42).expect("resumed");
    assert_eq!(resumed.phase, dormant.phase);
    assert_eq!(resumed.audit.len(), dormant_audit_len + 1);
    assert!(resumed.audit[dormant_audit_len - 1]
        .operation
        .contains("revoke_active_claim"));
    assert!(resumed.audit[dormant_audit_len]
        .operation
        .contains("reacquire_claim"));
    assert_eq!(result.previous_claim_id, None);
    assert_eq!(diagnose(&store, 42).status, DoctorStatus::Pass);
}

#[test]
fn expired_claim_reacquires_and_preserves_previous_owner_evidence() {
    let temp = tempfile::tempdir().expect("tempdir");
    fs::create_dir_all(temp.path().join("docs")).expect("docs");
    fs::write(temp.path().join("docs/design.md"), "# Reviewed design\n").expect("design");
    fs::write(
        temp.path().join("docs/diagram.mmd"),
        "flowchart LR\n  A --> B\n",
    )
    .expect("diagram");
    let store = Store::new(temp.path());
    let mut bootstrap = request();
    bootstrap.claim.expires_unix_seconds = u64::MAX - 2;
    let record = initialize_issue(&store, bootstrap).expect("initialize");
    git(temp.path(), &["branch", "-m", "issue-42"]);
    let result = reacquire_claim(
        &store,
        ReacquireClaimRequest {
            issue: 42,
            expected_generation: record.generation,
            expected_digest: record.digest,
            now_unix_seconds: u64::MAX - 1,
            actor: "next-owner".into(),
            reason: "replace expired lease".into(),
            replacement: Claim {
                acquired_unix_seconds: u64::MAX - 1,
                heartbeat_unix_seconds: u64::MAX - 1,
                ..reacquired_claim(record.generation)
            },
        },
    )
    .expect("expired reacquire");
    assert_eq!(result.previous_claim_id.as_deref(), Some("claim-1"));
    assert_eq!(result.previous_owner.as_deref(), Some("agent"));
}

#[test]
fn reacquire_fails_closed_for_stale_binding_and_live_overlap() {
    let (temp, store, record) = fixture();
    git(temp.path(), &["branch", "-m", "issue-42"]);
    csdlc_v2::revoke_active_claim(
        &store,
        csdlc_v2::RevokeActiveClaimRequest {
            issue: 42,
            repository: "example/repo".into(),
            expected_claim_id: "claim-1".into(),
            expected_generation: record.generation,
            expected_digest: record.digest,
            now_unix_seconds: 2,
            actor: "operator".into(),
            operator_authority: "operator-authorized:5727".into(),
            reason: "release".into(),
        },
    )
    .expect("release");
    let dormant = store.load_record(42).expect("dormant");
    let stale = reacquire_claim(
        &store,
        ReacquireClaimRequest {
            issue: 42,
            expected_generation: dormant.generation + 1,
            expected_digest: dormant.digest.clone(),
            now_unix_seconds: 10,
            actor: "next-owner".into(),
            reason: "stale".into(),
            replacement: reacquired_claim(dormant.generation),
        },
    )
    .expect_err("stale generation");
    assert_eq!(stale.code, ErrorCode::StaleGeneration);

    let stale_digest = reacquire_claim(
        &store,
        ReacquireClaimRequest {
            issue: 42,
            expected_generation: dormant.generation,
            expected_digest: "stale".into(),
            now_unix_seconds: 10,
            actor: "next-owner".into(),
            reason: "stale digest".into(),
            replacement: reacquired_claim(dormant.generation),
        },
    )
    .expect_err("stale digest");
    assert_eq!(stale_digest.code, ErrorCode::StaleDigest);

    let mut invalid_binding = reacquired_claim(dormant.generation);
    invalid_binding.branch = "other-branch".into();
    let invalid = reacquire_claim(
        &store,
        ReacquireClaimRequest {
            issue: 42,
            expected_generation: dormant.generation,
            expected_digest: dormant.digest.clone(),
            now_unix_seconds: 10,
            actor: "next-owner".into(),
            reason: "wrong branch".into(),
            replacement: invalid_binding,
        },
    )
    .expect_err("invalid binding");
    assert_eq!(invalid.code, ErrorCode::UnsafeCheckout);

    let mut invalid_worktree = reacquired_claim(dormant.generation);
    invalid_worktree.worktree = ".worktrees/not-this-one".into();
    let invalid = reacquire_claim(
        &store,
        ReacquireClaimRequest {
            issue: 42,
            expected_generation: dormant.generation,
            expected_digest: dormant.digest.clone(),
            now_unix_seconds: 10,
            actor: "next-owner".into(),
            reason: "wrong worktree".into(),
            replacement: invalid_worktree,
        },
    )
    .expect_err("invalid worktree");
    assert_eq!(invalid.code, ErrorCode::UnsafeCheckout);

    let mut other_request = request();
    other_request.issue = 43;
    other_request.claim.id = "claim-43".into();
    other_request.claim.branch = "issue-42".into();
    other_request.claim.worktree = ".".into();
    other_request.claim.protected_paths = vec!["src/nested".into()];
    initialize_issue(&store, other_request).expect("other issue");
    let collision = reacquire_claim(
        &store,
        ReacquireClaimRequest {
            issue: 42,
            expected_generation: dormant.generation,
            expected_digest: dormant.digest,
            now_unix_seconds: 10,
            actor: "next-owner".into(),
            reason: "colliding resume".into(),
            replacement: reacquired_claim(dormant.generation),
        },
    )
    .expect_err("live overlap");
    assert_eq!(collision.code, ErrorCode::ClaimCollision);
}

#[test]
fn fresh_initialization_accepts_overlap_released_by_metadata_advanced_merged_terminal() {
    let temp = tempfile::tempdir().expect("tempdir");
    git(temp.path(), &["init", "-b", "main"]);
    git(
        temp.path(),
        &["config", "user.email", "test@example.invalid"],
    );
    git(temp.path(), &["config", "user.name", "C-SDLC Test"]);
    fs::create_dir_all(temp.path().join("docs")).expect("docs");
    fs::create_dir_all(temp.path().join("csdlc-v2/src")).expect("csdlc source");
    fs::create_dir_all(temp.path().join("docs/templates/prompts")).expect("registry directory");
    fs::create_dir_all(temp.path().join("csdlc-v2/operator")).expect("manifest directory");
    fs::write(
        temp.path().join("docs/templates/prompts/current.json"),
        include_bytes!("../../docs/templates/prompts/current.json"),
    )
    .expect("registry fixture");
    fs::write(
        temp.path().join("csdlc-v2/operator/native-card-shape.json"),
        include_bytes!("../operator/native-card-shape.json"),
    )
    .expect("manifest fixture");
    fs::write(temp.path().join("docs/design.md"), "# Reviewed design\n").expect("design");
    fs::write(
        temp.path().join("docs/diagram.mmd"),
        "flowchart LR\n  A --> B\n",
    )
    .expect("diagram");
    fs::write(
        temp.path().join("csdlc-v2/src/lib.rs"),
        "pub fn stable() {}\n",
    )
    .expect("source");
    git(temp.path(), &["add", "."]);
    git(temp.path(), &["commit", "-m", "reviewed source"]);
    let reviewed_revision = csdlc_v2::git::substantive_revision(temp.path(), &["csdlc-v2".into()])
        .expect("reviewed revision");

    let review = csdlc_v2::ReviewEvidence {
        reviewer: "independent-reviewer".into(),
        scope: vec!["csdlc-v2".into()],
        reviewed_revision,
        findings: vec![],
        residual_risks: vec![],
        completed: true,
        non_substantive_proof: None,
    };
    let mut finished = csdlc_v2::IssueRecord {
        schema: "csdlc.issue.v2".into(),
        issue: 5_778,
        repository: "example/repo".into(),
        initialization_digest: "initialization-5778".into(),
        phase: LifecyclePhase::Reviewed,
        generation: 25,
        digest: "canonical-5778".into(),
        claim: Some(Claim {
            id: "claim-5778".into(),
            owner: "finished-session".into(),
            generation: 25,
            acquired_unix_seconds: 1,
            expires_unix_seconds: u64::MAX,
            heartbeat_unix_seconds: 1,
            branch: "main".into(),
            worktree: ".".into(),
            protected_paths: vec!["csdlc-v2".into()],
            purpose: "implementation".into(),
        }),
        review_assignment: None,
        review: Some(review),
        publication: None,
        readiness: None,
        terminal: None,
        migration: None,
        design_path: "docs/design.md".into(),
        diagram_path: "docs/diagram.mmd".into(),
        design_review: csdlc_v2::DesignReview::Approved {
            reviewer: "reviewer".into(),
            revision: "reviewed".into(),
        },
        cards: std::collections::BTreeMap::new(),
        transitions: vec![],
        audit: vec![],
    };
    fs::create_dir_all(temp.path().join(".csdlc/issues/5778")).expect("issue directory");
    fs::write(
        temp.path().join(".csdlc/issues/5778/index.json"),
        serde_json::to_vec_pretty(&finished).expect("historical record"),
    )
    .expect("historical projection");
    git(temp.path(), &["add", ".csdlc/issues/5778/index.json"]);
    git(temp.path(), &["commit", "-m", "review metadata"]);
    let published = csdlc_v2::git::run(temp.path(), &["rev-parse", "HEAD"])
        .expect("published head")
        .stdout;

    finished.phase = LifecyclePhase::Published;
    finished.publication = Some(csdlc_v2::PublicationEvidence {
        repository: "example/repo".into(),
        issue: 5_778,
        pull_request: 5_782,
        url: "https://example.test/pull/5782".into(),
        base: "main".into(),
        head: "codex/5778".into(),
        revision: csdlc_v2::git::clean_commit_revision(&published),
        draft: false,
        observed_state: "open".into(),
    });
    fs::write(
        temp.path().join(".csdlc/issues/5778/index.json"),
        serde_json::to_vec_pretty(&finished).expect("published record"),
    )
    .expect("published projection");
    git(temp.path(), &["add", ".csdlc/issues/5778/index.json"]);
    git(temp.path(), &["commit", "-m", "publication metadata"]);
    let final_head = csdlc_v2::git::run(temp.path(), &["rev-parse", "HEAD"])
        .expect("final head")
        .stdout;

    let finish_request = csdlc_v2::FinishRequest {
        schema: "csdlc.finish_request.v1".into(),
        issue: 5_778,
        expected_generation: 25,
        expected_digest: "canonical-5778".into(),
        claim_id: "claim-5778".into(),
        actor: "finished-session".into(),
        repository: "example/repo".into(),
        pull_request: Some(5_782),
        base: Some("main".into()),
        head: Some("codex/5778".into()),
        expected_head_sha: Some(final_head.clone()),
        merge_method: csdlc_v2::MergeMethod::Squash,
        required_checks: vec![],
        require_review: true,
        approved_no_pr_reason: None,
        token_file: None,
    };
    let packet = csdlc_v2::github::PrStatePacket {
        schema: "csdlc.github_pr_state.v1".into(),
        repository: "example/repo".into(),
        pull_request: 5_782,
        linked_issue: Some(5_778),
        linkage_source: Some("github".into()),
        state: "closed".into(),
        draft: false,
        merge_state: "unknown".into(),
        review_decision: "approved".into(),
        base_ref: Some("main".into()),
        head_ref: Some("codex/5778".into()),
        head_sha: final_head,
        url: Some("https://example.test/pull/5782".into()),
        body: Some("Closes #5778".into()),
        merged: true,
        merge_commit_sha: Some("1111111111111111111111111111111111111111".into()),
        checks: vec![],
        required_check_names: vec![],
        classification: "merged".into(),
    };
    let envelope = csdlc_v2::finish::derive_terminal(
        &finished,
        &finish_request,
        &csdlc_v2::IssueTerminalObservation {
            state: "closed".into(),
            labels: vec![],
            observed_unix_seconds: 100,
        },
        Some(&packet),
    )
    .expect("derive merged terminal")
    .expect("merged terminal");
    csdlc_v2::finish::retain_cached_terminal(temp.path(), &envelope)
        .expect("retain derived terminal");

    fs::write(
        temp.path().join("csdlc-v2/src/later.rs"),
        "pub fn later_unrelated_change() {}\n",
    )
    .expect("later source");
    git(temp.path(), &["add", "csdlc-v2/src/later.rs"]);
    git(
        temp.path(),
        &["commit", "-m", "later unrelated main change"],
    );

    let store = Store::new(temp.path());
    let mut next = request();
    next.claim.protected_paths = vec!["csdlc-v2/src/finish.rs".into()];
    initialize_issue(&store, next)
        .expect("strictly validated merged terminal releases overlapping finished claim");
}

#[test]
fn reacquire_rejects_direct_rendered_card_drift() {
    let (temp, store, record) = fixture();
    git(temp.path(), &["branch", "-m", "issue-42"]);
    csdlc_v2::revoke_active_claim(
        &store,
        csdlc_v2::RevokeActiveClaimRequest {
            issue: 42,
            repository: "example/repo".into(),
            expected_claim_id: "claim-1".into(),
            expected_generation: record.generation,
            expected_digest: record.digest,
            now_unix_seconds: 2,
            actor: "operator".into(),
            operator_authority: "operator-authorized:test".into(),
            reason: "prepare direct drift regression".into(),
        },
    )
    .expect("release");
    let dormant = store.load_record(42).expect("dormant");
    fs::write(store.issue_dir(42).join("cards/sip.md"), "# direct drift\n")
        .expect("write direct drift");
    let error = reacquire_claim(
        &store,
        ReacquireClaimRequest {
            issue: 42,
            expected_generation: dormant.generation,
            expected_digest: dormant.digest,
            now_unix_seconds: 10,
            actor: "next-owner".into(),
            reason: "direct drift must fail closed".into(),
            replacement: reacquired_claim(dormant.generation),
        },
    )
    .expect_err("direct rendered-card drift");
    assert_eq!(error.code, ErrorCode::CorruptRecord);
}

#[test]
fn concurrent_reacquisition_across_worktrees_allows_only_one_overlapping_writer() {
    let (temp, store, record_42) = fixture();
    git(
        temp.path(),
        &["config", "user.email", "test@example.invalid"],
    );
    git(temp.path(), &["config", "user.name", "test"]);
    csdlc_v2::revoke_active_claim(
        &store,
        csdlc_v2::RevokeActiveClaimRequest {
            issue: 42,
            repository: "example/repo".into(),
            expected_claim_id: "claim-1".into(),
            expected_generation: record_42.generation,
            expected_digest: record_42.digest,
            now_unix_seconds: 2,
            actor: "operator".into(),
            operator_authority: "operator-authorized:5727".into(),
            reason: "prepare dormant issue 42".into(),
        },
    )
    .expect("release issue 42");

    let mut request_43 = request();
    request_43.issue = 43;
    request_43.design_path = "docs/design-43.md".into();
    request_43.diagram_path = "docs/diagram-43.mmd".into();
    request_43.claim.id = "claim-43".into();
    request_43.claim.branch = "issue-43".into();
    request_43.claim.protected_paths = vec!["src/nested".into()];
    fs::write(temp.path().join("docs/design-43.md"), "# Reviewed design\n").expect("design");
    fs::write(
        temp.path().join("docs/diagram-43.mmd"),
        "flowchart LR\n  A --> B\n",
    )
    .expect("diagram");
    let record_43 = initialize_issue(&store, request_43).expect("initialize issue 43");
    csdlc_v2::revoke_active_claim(
        &store,
        csdlc_v2::RevokeActiveClaimRequest {
            issue: 43,
            repository: "example/repo".into(),
            expected_claim_id: "claim-43".into(),
            expected_generation: record_43.generation,
            expected_digest: record_43.digest,
            now_unix_seconds: 2,
            actor: "operator".into(),
            operator_authority: "operator-authorized:5727".into(),
            reason: "prepare dormant issue 43".into(),
        },
    )
    .expect("release issue 43");
    git(temp.path(), &["add", "."]);
    git(temp.path(), &["commit", "-m", "two dormant issues"]);

    let worktree_42 = temp.path().join("worktree-42");
    let worktree_43 = temp.path().join("worktree-43");
    git(
        temp.path(),
        &[
            "worktree",
            "add",
            "-b",
            "issue-42",
            worktree_42.to_str().expect("worktree 42"),
        ],
    );
    git(
        temp.path(),
        &[
            "worktree",
            "add",
            "-b",
            "issue-43",
            worktree_43.to_str().expect("worktree 43"),
        ],
    );
    let dormant_42 = Store::new(&worktree_42).load_record(42).expect("issue 42");
    let dormant_43 = Store::new(&worktree_43).load_record(43).expect("issue 43");
    let barrier = std::sync::Arc::new(std::sync::Barrier::new(2));

    let run = |issue: u64,
               root: std::path::PathBuf,
               dormant: csdlc_v2::IssueRecord,
               branch: &str,
               claim_id: &str,
               path: &str,
               barrier: std::sync::Arc<std::sync::Barrier>| {
        let branch = branch.to_owned();
        let claim_id = claim_id.to_owned();
        let path = path.to_owned();
        std::thread::spawn(move || {
            barrier.wait();
            reacquire_claim(
                &Store::new(root),
                ReacquireClaimRequest {
                    issue,
                    expected_generation: dormant.generation,
                    expected_digest: dormant.digest,
                    now_unix_seconds: 10,
                    actor: format!("owner-{issue}"),
                    reason: "concurrent cross-worktree reacquisition".into(),
                    replacement: Claim {
                        id: claim_id,
                        owner: format!("owner-{issue}"),
                        generation: dormant.generation,
                        acquired_unix_seconds: 10,
                        expires_unix_seconds: u64::MAX,
                        heartbeat_unix_seconds: 10,
                        branch,
                        worktree: ".".into(),
                        protected_paths: vec![path],
                        purpose: "prove one overlapping writer".into(),
                    },
                },
            )
        })
    };
    let thread_42 = run(
        42,
        worktree_42,
        dormant_42,
        "issue-42",
        "claim-reacquired-42",
        "src",
        barrier.clone(),
    );
    let thread_43 = run(
        43,
        worktree_43,
        dormant_43,
        "issue-43",
        "claim-reacquired-43",
        "src/nested",
        barrier,
    );
    let result_42 = thread_42.join().expect("issue 42 thread");
    let result_43 = thread_43.join().expect("issue 43 thread");
    let outcomes = [result_42, result_43];
    assert_eq!(outcomes.iter().filter(|result| result.is_ok()).count(), 1);
    assert_eq!(
        outcomes
            .iter()
            .filter(|result| {
                result
                    .as_ref()
                    .is_err_and(|error| error.code == ErrorCode::ClaimCollision)
            })
            .count(),
        1
    );
    assert!(temp.path().join(".git/csdlc-v2/bindings.lock").exists());
}

#[test]
fn expired_recovery_cannot_bypass_cross_worktree_overlap_authority() {
    let temp = tempfile::tempdir().expect("tempdir");
    let store = Store::new(temp.path());
    let mut request_42 = request();
    request_42.claim.protected_paths = vec!["old/42".into()];
    request_42.claim.expires_unix_seconds = u64::MAX - 2;
    fs::create_dir_all(temp.path().join("docs")).expect("docs");
    fs::write(temp.path().join("docs/design.md"), "# Reviewed design\n").expect("design");
    fs::write(
        temp.path().join("docs/diagram.mmd"),
        "flowchart LR\n  A --> B\n",
    )
    .expect("diagram");
    let record_42 = initialize_issue(&store, request_42).expect("initialize issue 42");

    let mut request_43 = request();
    request_43.issue = 43;
    request_43.design_path = "docs/design-43.md".into();
    request_43.diagram_path = "docs/diagram-43.mmd".into();
    request_43.claim.id = "claim-43".into();
    request_43.claim.branch = "issue-43".into();
    request_43.claim.worktree = ".worktrees/issue-43".into();
    request_43.claim.protected_paths = vec!["old/43".into()];
    request_43.claim.expires_unix_seconds = u64::MAX - 2;
    fs::write(temp.path().join("docs/design-43.md"), "# Reviewed design\n").expect("design");
    fs::write(
        temp.path().join("docs/diagram-43.mmd"),
        "flowchart LR\n  A --> B\n",
    )
    .expect("diagram");
    let record_43 = initialize_issue(&store, request_43).expect("initialize issue 43");
    git(
        temp.path(),
        &["config", "user.email", "test@example.invalid"],
    );
    git(temp.path(), &["config", "user.name", "test"]);
    git(temp.path(), &["add", "."]);
    git(temp.path(), &["commit", "-m", "two expiring issues"]);

    let worktree_42 = temp.path().join("worktree-42");
    let worktree_43 = temp.path().join("worktree-43");
    git(
        temp.path(),
        &[
            "worktree",
            "add",
            "-b",
            "issue-42",
            worktree_42.to_str().expect("worktree 42"),
        ],
    );
    git(
        temp.path(),
        &[
            "worktree",
            "add",
            "-b",
            "issue-43",
            worktree_43.to_str().expect("worktree 43"),
        ],
    );
    let barrier = std::sync::Arc::new(std::sync::Barrier::new(2));
    let run = |issue: u64,
               root: std::path::PathBuf,
               record: csdlc_v2::IssueRecord,
               claim_id: &str,
               branch: &str,
               worktree: &str,
               path: &str,
               barrier: std::sync::Arc<std::sync::Barrier>| {
        let claim_id = claim_id.to_owned();
        let branch = branch.to_owned();
        let worktree = worktree.to_owned();
        let path = path.to_owned();
        std::thread::spawn(move || {
            barrier.wait();
            csdlc_v2::recover_claim(
                &Store::new(root),
                csdlc_v2::RecoverClaimRequest {
                    issue,
                    expected_claim_id: record.claim.expect("expired claim").id,
                    expected_generation: record.generation,
                    now_unix_seconds: u64::MAX - 1,
                    replacement: Claim {
                        id: claim_id,
                        owner: format!("owner-{issue}"),
                        generation: record.generation,
                        acquired_unix_seconds: u64::MAX - 1,
                        expires_unix_seconds: u64::MAX,
                        heartbeat_unix_seconds: u64::MAX - 1,
                        branch,
                        worktree,
                        protected_paths: vec![path],
                        purpose: "recover through shared authority".into(),
                    },
                    recovery_actor: format!("operator-{issue}"),
                    reason: "expired cross-worktree recovery".into(),
                },
            )
        })
    };
    let thread_42 = run(
        42,
        worktree_42,
        record_42,
        "replacement-42",
        "issue-42",
        ".",
        "src",
        barrier.clone(),
    );
    let thread_43 = run(
        43,
        worktree_43,
        record_43,
        "replacement-43",
        "issue-43",
        ".",
        "src/nested",
        barrier,
    );
    let outcomes = [
        thread_42.join().expect("issue 42 thread"),
        thread_43.join().expect("issue 43 thread"),
    ];
    assert_eq!(outcomes.iter().filter(|result| result.is_ok()).count(), 1);
    assert_eq!(
        outcomes
            .iter()
            .filter(|result| {
                result
                    .as_ref()
                    .is_err_and(|error| error.code == ErrorCode::ClaimCollision)
            })
            .count(),
        1
    );
}

#[test]
fn bound_claim_scope_amendment_is_collision_checked_and_audited() {
    let (temp, store, record) = fixture();
    git(temp.path(), &["init", "-b", "main"]);
    git(
        temp.path(),
        &["config", "user.email", "test@example.invalid"],
    );
    git(temp.path(), &["config", "user.name", "test"]);
    git(temp.path(), &["add", "."]);
    git(temp.path(), &["commit", "-m", "fixture"]);
    let claim = record.claim.clone().expect("claim");
    bind_issue(
        &store,
        BindRequest {
            issue: 42,
            base_branch: "main".into(),
            branch: claim.branch.clone(),
            worktree: claim.worktree.clone(),
            claim,
        },
    )
    .expect("bind");
    let bound_store = Store::new(temp.path().join(".worktrees/issue-42"));
    let bound = bound_store.load_record(42).expect("bound");
    let amended = amend_claim_scope(
        &bound_store,
        AmendClaimScopeRequest {
            issue: 42,
            claim_id: "claim-1".into(),
            expected_generation: bound.generation,
            expected_digest: bound.digest,
            now_unix_seconds: 2,
            actor: "agent".into(),
            reason: "review found one additional owned path".into(),
            add_protected_paths: vec!["docs/review".into(), "docs/review".into()],
        },
    )
    .expect("amend");
    assert_eq!(amended.protected_paths, vec!["docs/review", "src"]);
    let current = bound_store.load_record(42).expect("current");
    assert_eq!(current.generation, bound.generation);
    assert!(current
        .audit
        .last()
        .expect("audit")
        .operation
        .contains("amend_claim_scope"));

    let mut other = current.clone();
    other.issue = 43;
    other.claim.as_mut().expect("claim").id = "other".into();
    other.claim.as_mut().expect("claim").protected_paths = vec!["docs/owned".into()];
    other.claim.as_mut().expect("claim").expires_unix_seconds = 2;
    fs::create_dir_all(bound_store.issue_dir(43)).expect("other issue");
    fs::write(
        bound_store.issue_dir(43).join("index.json"),
        serde_json::to_vec(&other).expect("json"),
    )
    .expect("other record");
    let error = amend_claim_scope(
        &bound_store,
        AmendClaimScopeRequest {
            issue: 42,
            claim_id: "claim-1".into(),
            expected_generation: current.generation,
            expected_digest: current.digest,
            now_unix_seconds: 3,
            actor: "agent".into(),
            reason: "attempt overlap".into(),
            add_protected_paths: vec!["docs/owned/nested".into()],
        },
    )
    .expect_err("collision");
    assert!(matches!(error.code, ErrorCode::ClaimCollision));
}

fn edit(record: &csdlc_v2::IssueRecord, operation: SemanticOperation) -> EditRequest {
    edit_for(42, "claim-1", record, CardKind::Sip, operation)
}

fn edit_for(
    issue: u64,
    claim_id: &str,
    record: &csdlc_v2::IssueRecord,
    card: CardKind,
    operation: SemanticOperation,
) -> EditRequest {
    EditRequest {
        issue,
        card,
        expected_generation: record.generation,
        expected_digest: record.digest.clone(),
        claim_id: claim_id.into(),
        actor: "agent".into(),
        reason: "test edit".into(),
        operation,
        fail_after_backup: false,
    }
}

#[test]
fn bootstrap_constructs_all_six_cards_and_ready_doctor() {
    let (_temp, store, record) = fixture();
    assert_eq!(record.cards.len(), 6);
    let cards = store.load_cards(42).expect("cards");
    assert_eq!(cards.len(), 6);
    assert_eq!(sip_goal(&cards), "Prove Gate 2.");
    let csdlc_v2::cards::CardContent::Sip(sip) = &cards[&CardKind::Sip].content else {
        panic!("SIP");
    };
    assert_eq!(sip.operator_constraints, vec!["none"]);
    let csdlc_v2::cards::CardContent::Srp(srp) = &cards[&CardKind::Srp].content else {
        panic!("SRP");
    };
    assert_eq!(srp.review_scope, "fixture");
    let report = diagnose(&store, 42);
    assert!(report.ready, "{report:?}");
    assert!(report.findings.is_empty());
}

#[test]
fn semantic_edit_updates_one_owned_projection_atomically() {
    let (_temp, store, record) = fixture();
    let next = edit_issue(
        &store,
        edit(
            &record,
            SemanticOperation::SetField {
                field: csdlc_v2::cards::TextField::Goal,
                value: "Ship the small state engine.".into(),
            },
        ),
    )
    .expect("edit");
    assert_eq!(next.generation, 1);
    let cards = store.load_cards(42).expect("cards");
    assert_eq!(sip_goal(&cards), "Ship the small state engine.");
    let audit = fs::read_to_string(store.issue_dir(42).join("audit.jsonl")).expect("audit");
    let events: Vec<serde_json::Value> = audit
        .lines()
        .map(|line| serde_json::from_str(line).expect("audit event"))
        .collect();
    assert_eq!(events.len(), 2);
    assert_eq!(events[1]["generation"], 1);
    assert!(events[1]["operation"]
        .as_str()
        .expect("operation")
        .contains("set_field"));
    assert!(diagnose(&store, 42).findings.is_empty());
}

#[test]
fn bound_replan_is_typed_claimed_and_limited_to_planning_cards() {
    let (temp, store, record) = fixture();
    git(temp.path(), &["init", "-b", "main"]);
    git(
        temp.path(),
        &["config", "user.email", "test@example.invalid"],
    );
    git(temp.path(), &["config", "user.name", "test"]);
    git(temp.path(), &["add", "."]);
    git(temp.path(), &["commit", "-m", "fixture"]);
    let claim = record.claim.clone().expect("claim");
    bind_issue(
        &store,
        BindRequest {
            issue: 42,
            base_branch: "main".into(),
            branch: "issue-42".into(),
            worktree: ".worktrees/issue-42".into(),
            claim,
        },
    )
    .expect("bind");
    let bound_store = Store::new(temp.path().join(".worktrees/issue-42"));
    let bound_record = bound_store.load_record(42).expect("bound record");
    let updated = edit_issue(
        &bound_store,
        EditRequest {
            issue: 42,
            card: CardKind::Spp,
            expected_generation: bound_record.generation,
            expected_digest: bound_record.digest.clone(),
            claim_id: "claim-1".into(),
            actor: "agent".into(),
            reason: "revise bounded plan after scope clarification".into(),
            operation: SemanticOperation::Replan {
                field: csdlc_v2::cards::TextField::PlanSummary,
                value: "Replanned bounded execution.".into(),
            },
            fail_after_backup: false,
        },
    )
    .expect("replan");
    assert!(updated.generation > bound_record.generation);
    let replan_audit = updated.audit.last().expect("audit").operation.clone();
    assert!(replan_audit.contains("replan"));
    assert!(replan_audit.contains("previous_value"));
    assert!(replan_audit.contains("Build then diagnose."));
    let sip = edit_issue(
        &bound_store,
        EditRequest {
            issue: 42,
            card: CardKind::Sip,
            expected_generation: updated.generation,
            expected_digest: updated.digest,
            claim_id: "claim-1".into(),
            actor: "agent".into(),
            reason: "revise intent".into(),
            operation: SemanticOperation::Replan {
                field: csdlc_v2::cards::TextField::Goal,
                value: "Replanned goal.".into(),
            },
            fail_after_backup: false,
        },
    )
    .expect("SIP replan");
    let stp = edit_issue(
        &bound_store,
        EditRequest {
            issue: 42,
            card: CardKind::Stp,
            expected_generation: sip.generation,
            expected_digest: sip.digest,
            claim_id: "claim-1".into(),
            actor: "agent".into(),
            reason: "revise task boundary".into(),
            operation: SemanticOperation::Replan {
                field: csdlc_v2::cards::TextField::TaskBoundary,
                value: "Replanned task boundary.".into(),
            },
            fail_after_backup: false,
        },
    )
    .expect("STP replan");
    let sip_constraints = edit_issue(
        &bound_store,
        EditRequest {
            issue: 42,
            card: CardKind::Sip,
            expected_generation: stp.generation,
            expected_digest: stp.digest,
            claim_id: "claim-1".into(),
            actor: "agent".into(),
            reason: "replace preparation constraints".into(),
            operation: SemanticOperation::ReplaceOperatorConstraints {
                values: vec!["owner binaries only".into()],
            },
            fail_after_backup: false,
        },
    )
    .expect("SIP constraint replacement");
    let srp_scope = edit_issue(
        &bound_store,
        EditRequest {
            issue: 42,
            card: CardKind::Srp,
            expected_generation: sip_constraints.generation,
            expected_digest: sip_constraints.digest,
            claim_id: "claim-1".into(),
            actor: "agent".into(),
            reason: "correct preparation review scope".into(),
            operation: SemanticOperation::Replan {
                field: csdlc_v2::cards::TextField::ReviewScope,
                value: "exact compact-card repair".into(),
            },
            fail_after_backup: false,
        },
    )
    .expect("bound SRP scope replan");
    let before_invalid = bound_store
        .load_cards(42)
        .expect("cards before invalid edit");
    let invalid = edit_issue(
        &bound_store,
        EditRequest {
            issue: 42,
            card: CardKind::Stp,
            expected_generation: srp_scope.generation,
            expected_digest: srp_scope.digest.clone(),
            claim_id: "claim-1".into(),
            actor: "agent".into(),
            reason: "try uncovered criterion".into(),
            operation: SemanticOperation::ReplaceAcceptanceCriteria {
                values: vec!["one".into(), "two".into(), "uncovered".into()],
            },
            fail_after_backup: false,
        },
    )
    .expect_err("SPP and VPP must cover every replacement criterion");
    assert!(matches!(invalid.code, ErrorCode::CardInvalid));
    assert_eq!(bound_store.load_cards(42).unwrap(), before_invalid);
    let stale = edit_issue(
        &bound_store,
        EditRequest {
            issue: 42,
            card: CardKind::Stp,
            expected_generation: srp_scope.generation,
            expected_digest: srp_scope.digest.clone(),
            claim_id: "claim-1".into(),
            actor: "agent".into(),
            reason: "try stale removed criterion mapping".into(),
            operation: SemanticOperation::ReplaceAcceptanceCriteria {
                values: vec!["one".into()],
            },
            fail_after_backup: false,
        },
    )
    .expect_err("removed criteria cannot leave stale SPP or VPP mappings");
    assert!(matches!(stale.code, ErrorCode::CardInvalid));
    assert_eq!(bound_store.load_cards(42).unwrap(), before_invalid);
    let prepared = edit_issue(
        &bound_store,
        EditRequest {
            issue: 42,
            card: CardKind::Stp,
            expected_generation: srp_scope.generation,
            expected_digest: srp_scope.digest,
            claim_id: "claim-1".into(),
            actor: "agent".into(),
            reason: "replace covered criteria".into(),
            operation: SemanticOperation::ReplaceAcceptanceCriteria {
                values: vec!["one".into(), "two".into()],
            },
            fail_after_backup: false,
        },
    )
    .expect("covered STP replacement");
    let sor = edit_issue(
        &bound_store,
        EditRequest {
            issue: 42,
            card: CardKind::Sor,
            expected_generation: prepared.generation,
            expected_digest: prepared.digest,
            claim_id: "claim-1".into(),
            actor: "agent".into(),
            reason: "unauthorized bound replan".into(),
            operation: SemanticOperation::Replan {
                field: csdlc_v2::cards::TextField::SorSummary,
                value: "nope".into(),
            },
            fail_after_backup: false,
        },
    )
    .expect_err("SOR replan must be rejected");
    assert!(matches!(sor.code, ErrorCode::InvalidTransition));
}

#[test]
fn bound_plan_progress_and_validation_lane_replacement_are_typed() {
    let (temp, store, record) = fixture();
    git(temp.path(), &["init", "-b", "main"]);
    git(
        temp.path(),
        &["config", "user.email", "test@example.invalid"],
    );
    git(temp.path(), &["config", "user.name", "test"]);
    git(temp.path(), &["add", "."]);
    git(temp.path(), &["commit", "-m", "fixture"]);
    let claim = record.claim.clone().expect("claim");
    bind_issue(
        &store,
        BindRequest {
            issue: 42,
            base_branch: "main".into(),
            branch: claim.branch.clone(),
            worktree: claim.worktree.clone(),
            claim,
        },
    )
    .expect("bind");
    let bound_store = Store::new(temp.path().join(".worktrees/issue-42"));
    let bound = bound_store.load_record(42).expect("bound");
    let progressed = edit_issue(
        &bound_store,
        EditRequest {
            issue: 42,
            card: CardKind::Spp,
            expected_generation: bound.generation,
            expected_digest: bound.digest,
            claim_id: "claim-1".into(),
            actor: "agent".into(),
            reason: "step proof completed".into(),
            operation: SemanticOperation::UpdatePlanStep {
                step_id: "step-1".into(),
                status: csdlc_v2::cards::StepStatus::Completed,
            },
            fail_after_backup: false,
        },
    )
    .expect("update plan step");
    let lanes = vec![csdlc_v2::cards::ValidationLane {
        lane: "focused-replacement".into(),
        proof_role: "Focused replacement proof".into(),
        acceptance_ids: vec!["AC-1".into(), "AC-2".into()],
        deterministic: true,
        resource_profile: csdlc_v2::cards::ResourceProfile::Small,
        budget_seconds: 120,
        budget_tokens: 1_000,
        argv: vec!["cargo".into(), "test".into()],
        parallel_group: "local".into(),
        defer_reason: None,
    }];
    edit_issue(
        &bound_store,
        EditRequest {
            issue: 42,
            card: CardKind::Vpp,
            expected_generation: progressed.generation,
            expected_digest: progressed.digest,
            claim_id: "claim-1".into(),
            actor: "agent".into(),
            reason: "correct proof role".into(),
            operation: SemanticOperation::ReplaceValidationLanes {
                lanes: lanes.clone(),
            },
            fail_after_backup: false,
        },
    )
    .expect("replace lanes");
    let cards = bound_store.load_cards(42).expect("cards");
    match &cards[&CardKind::Spp].content {
        csdlc_v2::cards::CardContent::Spp(spp) => {
            assert_eq!(spp.steps[0].status, csdlc_v2::cards::StepStatus::Completed)
        }
        _ => panic!("SPP"),
    }
    match &cards[&CardKind::Vpp].content {
        csdlc_v2::cards::CardContent::Vpp(vpp) => assert_eq!(vpp.lanes, lanes),
        _ => panic!("VPP"),
    }
}

#[test]
fn field_ownership_violation_fails_without_generation_change() {
    let (_temp, store, record) = fixture();
    let error = edit_issue(
        &store,
        edit(
            &record,
            SemanticOperation::SetField {
                field: csdlc_v2::cards::TextField::PlanSummary,
                value: "wrong owner".into(),
            },
        ),
    )
    .expect_err("ownership failure");
    assert!(matches!(error.code, ErrorCode::FieldOwnership));
    assert_eq!(store.load_record(42).expect("record").generation, 0);
}

#[test]
fn stale_generation_and_digest_fail_closed() {
    let (_temp, store, record) = fixture();
    let mut stale_generation = edit(
        &record,
        SemanticOperation::SetField {
            field: csdlc_v2::cards::TextField::Goal,
            value: "x".into(),
        },
    );
    stale_generation.expected_generation = 9;
    assert!(matches!(
        edit_issue(&store, stale_generation)
            .expect_err("stale generation")
            .code,
        ErrorCode::StaleGeneration
    ));

    let mut stale_digest = edit(
        &record,
        SemanticOperation::SetField {
            field: csdlc_v2::cards::TextField::Goal,
            value: "x".into(),
        },
    );
    stale_digest.expected_digest = "bad".into();
    assert!(matches!(
        edit_issue(&store, stale_digest)
            .expect_err("stale digest")
            .code,
        ErrorCode::StaleDigest
    ));
}

#[test]
fn illegal_transition_fails_closed() {
    let (_temp, store, record) = fixture();
    let error = edit_issue(
        &store,
        edit(
            &record,
            SemanticOperation::AdvancePhase {
                phase: csdlc_v2::LifecyclePhase::Bound,
            },
        ),
    )
    .expect_err("skip ready");
    assert!(matches!(error.code, ErrorCode::InvalidTransition));
    assert_eq!(
        store.load_record(42).expect("record").phase,
        csdlc_v2::LifecyclePhase::Initialized
    );
}

#[test]
fn issue_5337_preparation_converts_to_complete_implementation_truth_with_typed_edits() {
    let (_temp, store, mut record) = bind_issue_5337_fixture();

    for (card, field, replacement) in [
        (
            CardKind::Sip,
            PlanningCollectionField::DeclaredScope,
            vec!["versioned black-box characterization corpus".into()],
        ),
        (
            CardKind::Sip,
            PlanningCollectionField::AuthorityBoundary,
            vec!["pinned v1 binary is observed only through process I/O".into()],
        ),
        (
            CardKind::Sip,
            PlanningCollectionField::InitialAssumptions,
            vec!["the pinned v1 binary is available locally".into()],
        ),
        (
            CardKind::Stp,
            PlanningCollectionField::Deliverables,
            vec!["corpus harness".into(), "retained observations".into()],
        ),
        (
            CardKind::Stp,
            PlanningCollectionField::Dependencies,
            vec!["integrated runtime parity plan".into()],
        ),
        (
            CardKind::Stp,
            PlanningCollectionField::RepoInputs,
            vec!["adl-characterization".into(), "pinned v1 executable".into()],
        ),
        (
            CardKind::Stp,
            PlanningCollectionField::NonGoals,
            vec!["no incumbent ADL Rust dependency".into()],
        ),
        (
            CardKind::Spp,
            PlanningCollectionField::AffectedAreas,
            vec!["adl-characterization crate".into()],
        ),
        (
            CardKind::Spp,
            PlanningCollectionField::Invariants,
            vec!["raw observations remain immutable".into()],
        ),
        (
            CardKind::Spp,
            PlanningCollectionField::Risks,
            vec!["normalization could erase semantic differences".into()],
        ),
        (
            CardKind::Spp,
            PlanningCollectionField::StopConditions,
            vec!["unexplained repeated-run divergence".into()],
        ),
        (
            CardKind::Spp,
            PlanningCollectionField::ReplanTriggers,
            vec!["pinned v1 behavior changes".into()],
        ),
    ] {
        record = cli_edit_current(
            store.root(),
            &store,
            &record,
            card,
            SemanticOperation::ReplacePlanningCollection {
                field,
                values: replacement,
            },
        );
    }
    record = cli_edit_current(
        store.root(),
        &store,
        &record,
        CardKind::Sip,
        SemanticOperation::ReplaceOperatorConstraints {
            values: vec![
                "typed lifecycle only".into(),
                "no deferred corpus work".into(),
            ],
        },
    );
    record = cli_edit_current(
        store.root(),
        &store,
        &record,
        CardKind::Srp,
        SemanticOperation::ReplacePlanningCollection {
            field: PlanningCollectionField::ReviewPrompts,
            values: vec!["Can normalization erase a semantic difference?".into()],
        },
    );
    record = cli_edit_current(
        store.root(),
        &store,
        &record,
        CardKind::Spp,
        SemanticOperation::ReplaceAcceptancePlan {
            acceptance_criteria: vec![
                "every corpus case has repeated raw observations".into(),
                "coverage and nondeterminism classifications validate".into(),
                "the retained manifest records every scenario digest".into(),
            ],
            steps: vec![
                csdlc_v2::cards::PlanStep {
                    id: "capture".into(),
                    action: "capture every pinned v1 case repeatedly".into(),
                    acceptance_ids: vec!["AC-1".into()],
                    status: csdlc_v2::cards::StepStatus::Pending,
                },
                csdlc_v2::cards::PlanStep {
                    id: "verify".into(),
                    action: "verify normalization, coverage, and reproducibility".into(),
                    acceptance_ids: vec!["AC-2".into()],
                    status: csdlc_v2::cards::StepStatus::Pending,
                },
                csdlc_v2::cards::PlanStep {
                    id: "retain".into(),
                    action: "retain the complete scenario manifest".into(),
                    acceptance_ids: vec!["AC-3".into()],
                    status: csdlc_v2::cards::StepStatus::Pending,
                },
            ],
            validation_lanes: vec![csdlc_v2::cards::ValidationLane {
                lane: "characterization-corpus".into(),
                proof_role: "verify repeated outcomes and complete coverage".into(),
                acceptance_ids: vec!["AC-1".into(), "AC-2".into(), "AC-3".into()],
                deterministic: true,
                resource_profile: csdlc_v2::cards::ResourceProfile::Small,
                budget_seconds: 120,
                budget_tokens: 1_000,
                argv: vec!["cargo".into(), "test".into()],
                parallel_group: "local".into(),
                defer_reason: None,
            }],
        },
    );
    record = cli_edit_current(
        store.root(),
        &store,
        &record,
        CardKind::Srp,
        SemanticOperation::Replan {
            field: csdlc_v2::cards::TextField::ReviewScope,
            value: "exact characterization corpus implementation revision".into(),
        },
    );

    let cards = store.load_cards(5_337).expect("converted cards");
    let design_digest =
        csdlc_v2::cards::digest(&fs::read(store.root().join("docs/design.md")).expect("design"));
    let diagram_digest =
        csdlc_v2::cards::digest(&fs::read(store.root().join("docs/diagram.mmd")).expect("diagram"));
    csdlc_v2::cards::validate_cross_card(
        &cards,
        "docs/design.md",
        &design_digest,
        "docs/diagram.mmd",
        &diagram_digest,
    )
    .expect("typed cross-card validation");
    let report = diagnose(&store, 5_337);
    assert!(matches!(report.status, DoctorStatus::Pass), "{report:?}");
    assert!(report.findings.is_empty());
    assert_eq!(record.generation, 16);
    assert_eq!(record.audit.len(), 18);
    assert!(record
        .audit
        .iter()
        .any(|event| event.operation.contains("replace_planning_collection")));
    assert!(record
        .audit
        .iter()
        .any(|event| event.operation.contains("replace_acceptance_plan")));
}

#[test]
fn planning_replacements_reject_invalid_requests_without_mutation() {
    let (_temp, store, record) = bind_fixture();
    let before_record = store.load_record(42).expect("record snapshot");
    let before_cards = store.load_cards(42).expect("card snapshot");

    let cases = [
        EditRequest {
            issue: 42,
            card: CardKind::Sip,
            expected_generation: record.generation,
            expected_digest: record.digest.clone(),
            claim_id: "claim-1".into(),
            actor: "agent".into(),
            reason: "empty replacement".into(),
            operation: SemanticOperation::ReplacePlanningCollection {
                field: PlanningCollectionField::DeclaredScope,
                values: Vec::new(),
            },
            fail_after_backup: false,
        },
        EditRequest {
            issue: 42,
            card: CardKind::Sip,
            expected_generation: record.generation,
            expected_digest: record.digest.clone(),
            claim_id: "claim-1".into(),
            actor: "agent".into(),
            reason: "wrong card".into(),
            operation: SemanticOperation::ReplacePlanningCollection {
                field: PlanningCollectionField::Deliverables,
                values: vec!["wrong owner".into()],
            },
            fail_after_backup: false,
        },
        EditRequest {
            issue: 42,
            card: CardKind::Sip,
            expected_generation: record.generation + 1,
            expected_digest: record.digest.clone(),
            claim_id: "claim-1".into(),
            actor: "agent".into(),
            reason: "stale generation".into(),
            operation: SemanticOperation::ReplacePlanningCollection {
                field: PlanningCollectionField::DeclaredScope,
                values: vec!["scope".into()],
            },
            fail_after_backup: false,
        },
        EditRequest {
            issue: 42,
            card: CardKind::Sip,
            expected_generation: record.generation,
            expected_digest: "stale".into(),
            claim_id: "claim-1".into(),
            actor: "agent".into(),
            reason: "stale digest".into(),
            operation: SemanticOperation::ReplacePlanningCollection {
                field: PlanningCollectionField::DeclaredScope,
                values: vec!["scope".into()],
            },
            fail_after_backup: false,
        },
        EditRequest {
            issue: 42,
            card: CardKind::Sip,
            expected_generation: record.generation,
            expected_digest: record.digest.clone(),
            claim_id: "not-the-claim".into(),
            actor: "agent".into(),
            reason: "invalid claim".into(),
            operation: SemanticOperation::ReplacePlanningCollection {
                field: PlanningCollectionField::DeclaredScope,
                values: vec!["scope".into()],
            },
            fail_after_backup: false,
        },
        EditRequest {
            issue: 42,
            card: CardKind::Spp,
            expected_generation: record.generation,
            expected_digest: record.digest.clone(),
            claim_id: "claim-1".into(),
            actor: "agent".into(),
            reason: "stale acceptance mapping".into(),
            operation: SemanticOperation::ReplacePlanSteps {
                steps: vec![csdlc_v2::cards::PlanStep {
                    id: "bad".into(),
                    action: "map an unknown acceptance identifier".into(),
                    acceptance_ids: vec!["AC-1".into(), "AC-3".into()],
                    status: csdlc_v2::cards::StepStatus::Pending,
                }],
            },
            fail_after_backup: false,
        },
        EditRequest {
            issue: 42,
            card: CardKind::Spp,
            expected_generation: record.generation,
            expected_digest: record.digest.clone(),
            claim_id: "claim-1".into(),
            actor: "agent".into(),
            reason: "incoherent atomic acceptance plan".into(),
            operation: SemanticOperation::ReplaceAcceptancePlan {
                acceptance_criteria: vec!["one".into(), "two".into(), "three".into()],
                steps: vec![csdlc_v2::cards::PlanStep {
                    id: "incomplete".into(),
                    action: "omit the new criterion".into(),
                    acceptance_ids: vec!["AC-1".into(), "AC-2".into()],
                    status: csdlc_v2::cards::StepStatus::Pending,
                }],
                validation_lanes: vec![csdlc_v2::cards::ValidationLane {
                    lane: "incomplete".into(),
                    proof_role: "also omit the new criterion".into(),
                    acceptance_ids: vec!["AC-1".into(), "AC-2".into()],
                    deterministic: true,
                    resource_profile: csdlc_v2::cards::ResourceProfile::Small,
                    budget_seconds: 60,
                    budget_tokens: 100,
                    argv: vec!["cargo".into(), "test".into()],
                    parallel_group: "local".into(),
                    defer_reason: None,
                }],
            },
            fail_after_backup: false,
        },
    ];

    let expected_codes = [
        ErrorCode::CardInvalid,
        ErrorCode::FieldOwnership,
        ErrorCode::StaleGeneration,
        ErrorCode::StaleDigest,
        ErrorCode::MissingClaim,
        ErrorCode::CardInvalid,
        ErrorCode::CardInvalid,
    ];
    for (request, expected) in cases.into_iter().zip(expected_codes) {
        let error = edit_issue(&store, request).expect_err("invalid replacement");
        assert_eq!(error.code, expected);
        assert_eq!(store.load_record(42).unwrap(), before_record);
        assert_eq!(store.load_cards(42).unwrap(), before_cards);
    }
}

#[test]
fn planning_replacements_are_phase_bounded_and_allow_narrow_implemented_corrections() {
    let (_temp, initialized_store, initialized) = fixture();
    for (card, operation) in [
        (
            CardKind::Sip,
            SemanticOperation::ReplacePlanningCollection {
                field: PlanningCollectionField::DeclaredScope,
                values: vec!["scope".into()],
            },
        ),
        (
            CardKind::Sip,
            SemanticOperation::ReplaceOperatorConstraints {
                values: vec!["constraint".into()],
            },
        ),
        (
            CardKind::Stp,
            SemanticOperation::ReplaceAcceptanceCriteria {
                values: vec!["one".into(), "two".into()],
            },
        ),
    ] {
        let error = edit_issue(
            &initialized_store,
            EditRequest {
                issue: 42,
                card,
                expected_generation: initialized.generation,
                expected_digest: initialized.digest.clone(),
                claim_id: "claim-1".into(),
                actor: "agent".into(),
                reason: "too early".into(),
                operation,
                fail_after_backup: false,
            },
        )
        .expect_err("initialized replacement must fail");
        assert_eq!(error.code, ErrorCode::InvalidTransition);
    }

    let (_temp, store, mut record) = bind_fixture();
    let smuggled = edit_issue(
        &store,
        EditRequest {
            issue: 42,
            card: CardKind::Spp,
            expected_generation: record.generation,
            expected_digest: record.digest.clone(),
            claim_id: "claim-1".into(),
            actor: "agent".into(),
            reason: "smuggle completion".into(),
            operation: SemanticOperation::ReplacePlanSteps {
                steps: vec![csdlc_v2::cards::PlanStep {
                    id: "done".into(),
                    action: "claim completion".into(),
                    acceptance_ids: vec!["AC-1".into(), "AC-2".into()],
                    status: csdlc_v2::cards::StepStatus::Completed,
                }],
            },
            fail_after_backup: false,
        },
    )
    .expect_err("replacement cannot smuggle completed work");
    assert_eq!(smuggled.code, ErrorCode::CardInvalid);

    record = edit_current(
        &store,
        &record,
        CardKind::Sor,
        SemanticOperation::RecordExecution {
            summary: "implemented".into(),
            changes: vec!["csdlc-v2".into()],
            artifacts: vec!["tests".into()],
        },
    );
    record = edit_current(
        &store,
        &record,
        CardKind::Sip,
        SemanticOperation::AdvancePhase {
            phase: csdlc_v2::LifecyclePhase::Implemented,
        },
    );
    record = edit_current(
        &store,
        &record,
        CardKind::Spp,
        SemanticOperation::ReplacePlanningCollection {
            field: PlanningCollectionField::AffectedAreas,
            values: vec!["implementation-discovered surface".into()],
        },
    );
    let cards = store.load_cards(42).expect("cards");
    let CardContent::Spp(spp) = &cards[&CardKind::Spp].content else {
        panic!("SPP")
    };
    assert_eq!(
        spp.affected_areas,
        vec!["implementation-discovered surface"]
    );
    record = edit_current(
        &store,
        &record,
        CardKind::Sip,
        SemanticOperation::ReplaceOperatorConstraints {
            values: vec!["corrected implementation boundary".into()],
        },
    );
    record = edit_current(
        &store,
        &record,
        CardKind::Stp,
        SemanticOperation::ReplaceAcceptanceCriteria {
            values: vec!["one".into(), "two".into()],
        },
    );
    record = edit_current(
        &store,
        &record,
        CardKind::Srp,
        SemanticOperation::ReplacePlanningCollection {
            field: PlanningCollectionField::ReviewPrompts,
            values: vec!["corrected exact-head prompt".into()],
        },
    );
    let corrected_cards = store.load_cards(42).expect("corrected cards");
    let CardContent::Sip(sip) = &corrected_cards[&CardKind::Sip].content else {
        panic!("SIP")
    };
    assert_eq!(
        sip.operator_constraints,
        vec!["corrected implementation boundary"]
    );
    record = edit_current(
        &store,
        &record,
        CardKind::Sip,
        SemanticOperation::ReplacePlanningCollection {
            field: PlanningCollectionField::AuthorityBoundary,
            values: vec!["corrected implementation authority".into()],
        },
    );
    let corrected_cards = store.load_cards(42).expect("authority corrected cards");
    let CardContent::Sip(sip) = &corrected_cards[&CardKind::Sip].content else {
        panic!("SIP")
    };
    assert_eq!(
        sip.authority_boundary,
        vec!["corrected implementation authority"]
    );
    let CardContent::Srp(srp) = &corrected_cards[&CardKind::Srp].content else {
        panic!("SRP")
    };
    assert_eq!(srp.review_prompts, vec!["corrected exact-head prompt"]);

    let error = edit_issue(
        &store,
        EditRequest {
            issue: 42,
            card: CardKind::Sip,
            expected_generation: record.generation,
            expected_digest: record.digest,
            claim_id: "claim-1".into(),
            actor: "agent".into(),
            reason: "too late".into(),
            operation: SemanticOperation::ReplacePlanningCollection {
                field: PlanningCollectionField::DeclaredScope,
                values: vec!["late scope widening".into()],
            },
            fail_after_backup: false,
        },
    )
    .expect_err("unrelated post-implementation replan must fail");
    assert_eq!(error.code, ErrorCode::InvalidTransition);
}

#[test]
fn implemented_spp_review_remediation_allows_guarded_plan_and_stop_condition_corrections() {
    let (_temp, store, mut record) = implemented_fixture();

    record = edit_issue(
        &store,
        spp_replacement_request(
            &record,
            SemanticOperation::ReplacePlanSteps {
                steps: replacement_steps(),
            },
        ),
    )
    .expect("implemented plan-step correction");
    let cards = store.load_cards(42).expect("plan-step cards");
    let CardContent::Spp(spp) = &cards[&CardKind::Spp].content else {
        panic!("SPP");
    };
    assert_eq!(spp.steps, replacement_steps());
    assert!(record
        .audit
        .last()
        .expect("audit")
        .operation
        .contains("replace_plan_steps"));

    record = edit_issue(
        &store,
        spp_replacement_request(
            &record,
            SemanticOperation::ReplacePlanningCollection {
                field: PlanningCollectionField::Invariants,
                values: vec!["review-remediated invariant".into()],
            },
        ),
    )
    .expect("implemented invariant correction");
    record = edit_issue(
        &store,
        spp_replacement_request(
            &record,
            SemanticOperation::ReplacePlanningCollection {
                field: PlanningCollectionField::StopConditions,
                values: vec!["review-remediated stop condition".into()],
            },
        ),
    )
    .expect("implemented stop-condition correction");

    let cards = store.load_cards(42).expect("corrected cards");
    let CardContent::Spp(spp) = &cards[&CardKind::Spp].content else {
        panic!("SPP");
    };
    assert_eq!(spp.invariants, vec!["review-remediated invariant"]);
    assert_eq!(
        spp.stop_conditions,
        vec!["review-remediated stop condition"]
    );
    assert_eq!(record.phase, LifecyclePhase::Implemented);
    assert_eq!(
        record.claim.as_ref().expect("claim").generation,
        record.generation
    );
}

#[test]
fn implemented_spp_replacements_remain_generation_digest_and_claim_guarded() {
    let (_temp, store, record) = implemented_fixture();
    let before_record = store.load_record(42).expect("record snapshot");
    let before_cards = store.load_cards(42).expect("card snapshot");

    let mut stale_generation = spp_replacement_request(
        &record,
        SemanticOperation::ReplacePlanSteps {
            steps: replacement_steps(),
        },
    );
    stale_generation.expected_generation += 1;
    let error = edit_issue(&store, stale_generation).expect_err("stale generation");
    assert_eq!(error.code, ErrorCode::StaleGeneration);

    let mut stale_digest = spp_replacement_request(
        &record,
        SemanticOperation::ReplacePlanningCollection {
            field: PlanningCollectionField::Invariants,
            values: vec!["guarded".into()],
        },
    );
    stale_digest.expected_digest = "stale".into();
    let error = edit_issue(&store, stale_digest).expect_err("stale digest");
    assert_eq!(error.code, ErrorCode::StaleDigest);

    let mut stale_claim = spp_replacement_request(
        &record,
        SemanticOperation::ReplacePlanningCollection {
            field: PlanningCollectionField::StopConditions,
            values: vec!["guarded".into()],
        },
    );
    stale_claim.claim_id = "not-the-claim".into();
    let error = edit_issue(&store, stale_claim).expect_err("stale claim");
    assert_eq!(error.code, ErrorCode::MissingClaim);

    assert_eq!(store.load_record(42).expect("record"), before_record);
    assert_eq!(store.load_cards(42).expect("cards"), before_cards);
}

#[test]
fn implemented_spp_review_remediation_rejects_unbounded_collections() {
    let (_temp, store, record) = implemented_fixture();
    let error = edit_issue(
        &store,
        spp_replacement_request(
            &record,
            SemanticOperation::ReplacePlanningCollection {
                field: PlanningCollectionField::Risks,
                values: vec!["not bounded review remediation".into()],
            },
        ),
    )
    .expect_err("implemented risks replacement remains rejected");
    assert_eq!(error.code, ErrorCode::InvalidTransition);
}

#[test]
fn implemented_review_remediation_allows_guarded_sip_authority_and_stp_acceptance() {
    let (_temp, store, mut record) = implemented_fixture();

    record = edit_issue(
        &store,
        edit_for(
            42,
            "claim-1",
            &record,
            CardKind::Sip,
            SemanticOperation::ReplacePlanningCollection {
                field: PlanningCollectionField::AuthorityBoundary,
                values: vec!["release the successor after merge".into()],
            },
        ),
    )
    .expect("implemented SIP authority correction");
    record = edit_issue(
        &store,
        edit_for(
            42,
            "claim-1",
            &record,
            CardKind::Stp,
            SemanticOperation::ReplaceAcceptanceCriteria {
                values: vec![
                    "merge releases the successor".into(),
                    "closeout remains asynchronous".into(),
                ],
            },
        ),
    )
    .expect("implemented STP acceptance correction");

    let cards = store.load_cards(42).expect("corrected cards");
    let CardContent::Sip(sip) = &cards[&CardKind::Sip].content else {
        panic!("SIP");
    };
    assert_eq!(
        sip.authority_boundary,
        vec!["release the successor after merge"]
    );
    let CardContent::Stp(stp) = &cards[&CardKind::Stp].content else {
        panic!("STP");
    };
    assert_eq!(
        stp.acceptance_criteria,
        vec![
            "merge releases the successor",
            "closeout remains asynchronous"
        ]
    );
    assert_eq!(record.phase, LifecyclePhase::Implemented);
}

#[test]
fn implemented_sip_review_remediation_rejects_non_authority_collections() {
    let (_temp, store, record) = implemented_fixture();
    let error = edit_issue(
        &store,
        edit_for(
            42,
            "claim-1",
            &record,
            CardKind::Sip,
            SemanticOperation::ReplacePlanningCollection {
                field: PlanningCollectionField::DeclaredScope,
                values: vec!["scope widening remains forbidden".into()],
            },
        ),
    )
    .expect_err("implemented SIP scope replacement remains rejected");
    assert_eq!(error.code, ErrorCode::InvalidTransition);
}

#[test]
fn execution_replacement_is_sor_only_and_implemented_only() {
    let (_temp, store, mut record) = bind_fixture();
    let too_early = edit_issue(
        &store,
        EditRequest {
            issue: 42,
            card: CardKind::Sor,
            expected_generation: record.generation,
            expected_digest: record.digest.clone(),
            claim_id: "claim-1".into(),
            actor: "agent".into(),
            reason: "replacement requires observed implementation".into(),
            operation: SemanticOperation::ReplaceExecution {
                summary: "not yet implemented".into(),
                changes: vec![],
                artifacts: vec![],
                validation: vec![],
            },
            fail_after_backup: false,
        },
    )
    .expect_err("bound execution replacement must fail");
    assert_eq!(too_early.code, ErrorCode::InvalidTransition);

    record = edit_current(
        &store,
        &record,
        CardKind::Sor,
        SemanticOperation::RecordExecution {
            summary: "interim execution".into(),
            changes: vec!["stale change".into()],
            artifacts: vec!["stale-artifact.json".into()],
        },
    );
    record = edit_current(
        &store,
        &record,
        CardKind::Sip,
        SemanticOperation::AdvancePhase {
            phase: csdlc_v2::LifecyclePhase::Implemented,
        },
    );
    record = edit_current(
        &store,
        &record,
        CardKind::Sor,
        SemanticOperation::ReplaceExecution {
            summary: "final truthful execution".into(),
            changes: vec!["final change".into()],
            artifacts: vec!["final-evidence.json".into()],
            validation: vec![csdlc_v2::cards::ValidationResult {
                command: vec!["cargo".into(), "test".into()],
                purpose: "focused exact proof".into(),
                outcome: csdlc_v2::cards::EvidenceOutcome::Passed,
                evidence_ref: "final-evidence.json".into(),
            }],
        },
    );

    let cards = store.load_cards(42).expect("cards");
    let CardContent::Sor(sor) = &cards[&CardKind::Sor].content else {
        panic!("SOR")
    };
    assert_eq!(sor.summary, "final truthful execution");
    assert_eq!(sor.actual_changes, vec!["final change"]);
    assert_eq!(sor.artifacts, vec!["final-evidence.json"]);
    assert_eq!(sor.actual_validation.len(), 1);
    assert_eq!(sor.actual_validation[0].purpose, "focused exact proof");
    assert_eq!(record.phase, csdlc_v2::LifecyclePhase::Implemented);

    let invalid = edit_issue(
        &store,
        EditRequest {
            issue: 42,
            card: CardKind::Sor,
            expected_generation: record.generation,
            expected_digest: record.digest.clone(),
            claim_id: "claim-1".into(),
            actor: "agent".into(),
            reason: "cannot erase execution truth".into(),
            operation: SemanticOperation::ReplaceExecution {
                summary: "".into(),
                changes: vec![],
                artifacts: vec![],
                validation: vec![],
            },
            fail_after_backup: false,
        },
    )
    .expect_err("replacement cannot erase execution truth");
    assert_eq!(invalid.code, ErrorCode::CardInvalid);
    assert_eq!(store.load_record(42).expect("unchanged record"), record);
}

#[test]
fn direct_markdown_drift_is_corruption() {
    let (_temp, store, _record) = fixture();
    fs::write(
        store.issue_dir(42).join("cards/sip.md"),
        "# edited by hand\n",
    )
    .expect("drift");
    let report = diagnose(&store, 42);
    assert!(matches!(
        report.status,
        csdlc_v2::doctor::DoctorStatus::Corrupt
    ));
    assert_eq!(report.findings[0].code, "corrupt_record");
}

#[test]
fn missing_design_or_diagram_blocks_readiness() {
    let (temp, store, _record) = fixture();
    fs::remove_file(temp.path().join("docs/diagram.mmd")).expect("remove diagram");
    let report = diagnose(&store, 42);
    assert!(!report.ready);
    assert!(report
        .findings
        .iter()
        .any(|finding| finding.code == "diagram_missing"));
}

#[test]
fn interrupted_commit_keeps_complete_backup_and_next_writer_recovers() {
    let (_temp, store, record) = fixture();
    let mut interrupted = edit(
        &record,
        SemanticOperation::SetField {
            field: csdlc_v2::cards::TextField::Goal,
            value: "interrupted".into(),
        },
    );
    interrupted.fail_after_backup = true;
    assert!(matches!(
        edit_issue(&store, interrupted)
            .expect_err("injected failure")
            .code,
        ErrorCode::InterruptedTransaction
    ));
    assert!(!store.issue_dir(42).exists());
    assert!(store.interrupted_backup(42).exists());
    assert!(matches!(
        diagnose(&store, 42).status,
        csdlc_v2::doctor::DoctorStatus::Interrupted
    ));

    let recovered = edit_issue(
        &store,
        edit(
            &record,
            SemanticOperation::SetField {
                field: csdlc_v2::cards::TextField::Goal,
                value: "recovered".into(),
            },
        ),
    )
    .expect("recover and edit");
    assert_eq!(recovered.generation, 1);
    assert!(!store.interrupted_backup(42).exists());
    assert!(diagnose(&store, 42).findings.is_empty());
}

fn sip_goal(cards: &std::collections::BTreeMap<CardKind, csdlc_v2::CardValues>) -> &str {
    match &cards[&CardKind::Sip].content {
        csdlc_v2::cards::CardContent::Sip(values) => &values.goal,
        _ => unreachable!("SIP content"),
    }
}

#[test]
fn public_schema_bundle_covers_requests_state_and_doctor_output() {
    let schema = csdlc_v2::public_schema_bundle();
    assert_eq!(schema["schema"], "csdlc.public_schema_bundle.v1");
    for key in [
        "approve_design_request",
        "edit_request",
        "recover_claim_request",
        "release_closed_claim_request",
        "revoke_active_claim_request",
        "revoke_active_claim_result",
        "amend_claim_scope_request",
        "issue_record",
        "terminal_receipt",
        "doctor_report",
    ] {
        assert!(schema[key].is_object(), "missing schema for {key}");
        assert!(
            schema[key]["properties"].is_object(),
            "missing root properties for {key}"
        );
    }
    assert!(schema.get("reconcile_terminal_request").is_none());
}

#[test]
fn lifecycle_binaries_share_stable_typed_exit_classes() {
    assert_eq!(ErrorCode::InvalidInput.exit_code(), 64);
    assert_eq!(ErrorCode::ClaimCollision.exit_code(), 73);
    assert_eq!(ErrorCode::GitFailure.exit_code(), 74);
    assert_eq!(ErrorCode::ReconciliationRequired.exit_code(), 75);
}

#[test]
fn placeholder_design_is_pending_then_can_be_completed_approved_and_bound() {
    let temp = tempfile::tempdir().expect("tempdir");
    let store = Store::new(temp.path());
    let record = initialize_issue(&store, request()).expect("placeholder init");
    assert!(matches!(
        record.design_review,
        csdlc_v2::DesignReview::Pending
    ));
    assert!(!diagnose(&store, 42).ready);
    fs::write(
        temp.path().join("docs/design.md"),
        "# Completed design\n\nReviewed.\n",
    )
    .expect("design edit");
    let approved = csdlc_v2::approve_design(
        &store,
        csdlc_v2::ApproveDesignRequest {
            issue: 42,
            expected_generation: record.generation,
            expected_digest: record.digest,
            claim_id: "claim-1".into(),
            reviewer: "architect".into(),
        },
    )
    .expect("approve design");
    assert!(
        matches!(approved.design_review, csdlc_v2::DesignReview::Approved { reviewer, .. } if reviewer == "architect")
    );
    let cards = store.load_cards(42).expect("approved cards");
    let design_digest =
        csdlc_v2::cards::digest(&fs::read(store.root().join("docs/design.md")).expect("design"));
    let diagram_digest =
        csdlc_v2::cards::digest(&fs::read(store.root().join("docs/diagram.mmd")).expect("diagram"));
    for kind in [CardKind::Spp, CardKind::Vpp] {
        match &cards[&kind].content {
            csdlc_v2::cards::CardContent::Spp(values) => {
                assert_eq!(values.design_digest, design_digest);
                assert_eq!(values.diagram_digest, diagram_digest);
            }
            csdlc_v2::cards::CardContent::Vpp(values) => {
                assert_eq!(values.design_digest, design_digest);
                assert_eq!(values.diagram_digest, diagram_digest);
            }
            _ => unreachable!("design-bearing card"),
        }
    }
    assert!(diagnose(&store, 42).ready);
    git(temp.path(), &["init", "-b", "main"]);
    git(
        temp.path(),
        &["config", "user.email", "test@example.invalid"],
    );
    git(temp.path(), &["config", "user.name", "C-SDLC Test"]);
    git(temp.path(), &["add", "."]);
    git(temp.path(), &["commit", "-m", "fixture"]);
    let claim = approved.claim.clone().expect("claim");
    bind_issue(
        &store,
        BindRequest {
            issue: 42,
            base_branch: "main".into(),
            branch: claim.branch.clone(),
            worktree: claim.worktree.clone(),
            claim,
        },
    )
    .expect("bind");
    assert_eq!(
        Store::new(temp.path().join(".worktrees/issue-42"))
            .load_record(42)
            .expect("record")
            .phase,
        csdlc_v2::LifecyclePhase::Bound
    );
}

#[test]
fn initialized_approved_stale_design_can_be_reapproved_before_readiness() {
    let temp = tempfile::tempdir().expect("tempdir");
    git(temp.path(), &["init", "-b", "main"]);
    git(
        temp.path(),
        &["config", "user.email", "test@example.invalid"],
    );
    git(temp.path(), &["config", "user.name", "C-SDLC Test"]);
    fs::create_dir_all(temp.path().join("docs")).expect("docs");
    fs::write(temp.path().join("docs/design.md"), "# Reviewed design\n").expect("design");
    fs::write(
        temp.path().join("docs/diagram.mmd"),
        "flowchart LR\n  A --> B\n",
    )
    .expect("diagram");
    let store = Store::new(temp.path());
    let record = initialize_issue(&store, request()).expect("initialize");
    let redundant = csdlc_v2::approve_design(
        &store,
        csdlc_v2::ApproveDesignRequest {
            issue: 42,
            expected_generation: record.generation,
            expected_digest: record.digest.clone(),
            claim_id: "claim-1".into(),
            reviewer: "architect".into(),
        },
    )
    .expect_err("unchanged initialized approval must not churn state");
    assert!(matches!(redundant.code, ErrorCode::InvalidTransition));

    fs::write(
        temp.path().join("docs/design.md"),
        "# Approved design changed before readiness\n",
    )
    .expect("stale design edit");
    assert!(!diagnose(&store, 42).ready);

    let recovered = csdlc_v2::approve_design(
        &store,
        csdlc_v2::ApproveDesignRequest {
            issue: 42,
            expected_generation: record.generation,
            expected_digest: record.digest,
            claim_id: "claim-1".into(),
            reviewer: "recovery-reviewer".into(),
        },
    )
    .expect("typed initialized design recovery");
    assert_eq!(recovered.phase, csdlc_v2::LifecyclePhase::Initialized);
    assert_eq!(recovered.generation, 1);
    assert!(matches!(
        recovered.design_review,
        csdlc_v2::DesignReview::Approved { reviewer, .. }
            if reviewer == "recovery-reviewer"
    ));
    assert_eq!(
        recovered.audit.last().expect("audit").reason,
        "reapprove stale initialized issue design"
    );
    assert!(diagnose(&store, 42).ready);
}

#[test]
fn bound_and_implemented_design_reapproval_refreshes_truth_and_reviewed_rejects() {
    let (temp, store, record) = fixture();
    let ready = edit_issue(
        &store,
        edit(
            &record,
            SemanticOperation::AdvancePhase {
                phase: csdlc_v2::LifecyclePhase::Ready,
            },
        ),
    )
    .expect("ready");
    let bound = edit_issue(
        &store,
        edit(
            &ready,
            SemanticOperation::AdvancePhase {
                phase: csdlc_v2::LifecyclePhase::Bound,
            },
        ),
    )
    .expect("bound");
    let bound_transitions = bound.transitions.clone();
    fs::write(
        temp.path().join("docs/design.md"),
        "# Bound design revision\n",
    )
    .expect("bound design edit");
    fs::write(
        temp.path().join("docs/diagram.mmd"),
        "flowchart LR\n  Bound --> Reapproved\n",
    )
    .expect("bound diagram edit");
    let reapproved_bound = csdlc_v2::approve_design(
        &store,
        csdlc_v2::ApproveDesignRequest {
            issue: 42,
            expected_generation: bound.generation,
            expected_digest: bound.digest,
            claim_id: "claim-1".into(),
            reviewer: "architect".into(),
        },
    )
    .expect("bound reapproval");
    assert_eq!(reapproved_bound.phase, csdlc_v2::LifecyclePhase::Bound);
    assert_eq!(reapproved_bound.transitions, bound_transitions);
    assert_eq!(reapproved_bound.generation, bound.generation + 1);
    assert_eq!(
        reapproved_bound.audit.last().expect("audit").operation,
        "approve_design"
    );

    let mut execution = edit(
        &reapproved_bound,
        SemanticOperation::RecordExecution {
            summary: "implemented".into(),
            changes: vec!["csdlc-v2/src/store.rs".into()],
            artifacts: vec!["focused tests".into()],
        },
    );
    execution.card = CardKind::Sor;
    let implemented_evidence = edit_issue(&store, execution).expect("execution evidence");
    let implemented = edit_issue(
        &store,
        edit(
            &implemented_evidence,
            SemanticOperation::AdvancePhase {
                phase: csdlc_v2::LifecyclePhase::Implemented,
            },
        ),
    )
    .expect("implemented");
    let implemented_transitions = implemented.transitions.clone();
    fs::write(
        temp.path().join("docs/design.md"),
        "# Implemented design correction\n",
    )
    .expect("implemented design edit");
    fs::write(
        temp.path().join("docs/diagram.mmd"),
        "flowchart LR\n  Implemented --> Corrected\n",
    )
    .expect("implemented diagram edit");

    let mut stale_mutation = edit(
        &implemented,
        SemanticOperation::UpdatePlanStep {
            step_id: "step-1".into(),
            status: csdlc_v2::cards::StepStatus::Completed,
        },
    );
    stale_mutation.card = CardKind::Spp;
    assert!(matches!(
        edit_issue(&store, stale_mutation)
            .expect_err("stale design blocks typed mutation")
            .code,
        ErrorCode::CardInvalid
    ));

    let reapproved = csdlc_v2::approve_design(
        &store,
        csdlc_v2::ApproveDesignRequest {
            issue: 42,
            expected_generation: implemented.generation,
            expected_digest: implemented.digest,
            claim_id: "claim-1".into(),
            reviewer: "architect".into(),
        },
    )
    .expect("implemented reapproval");
    assert_eq!(reapproved.phase, csdlc_v2::LifecyclePhase::Implemented);
    assert_eq!(reapproved.transitions, implemented_transitions);
    assert_eq!(reapproved.generation, implemented.generation + 1);
    let design_digest =
        csdlc_v2::cards::digest(&fs::read(store.root().join("docs/design.md")).expect("design"));
    let diagram_digest =
        csdlc_v2::cards::digest(&fs::read(store.root().join("docs/diagram.mmd")).expect("diagram"));
    assert!(matches!(
        &reapproved.design_review,
        csdlc_v2::DesignReview::Approved { revision, .. } if revision == &design_digest
    ));
    let cards = store.load_cards(42).expect("reapproved cards");
    for kind in [CardKind::Spp, CardKind::Vpp] {
        let (actual_design, actual_diagram) = match &cards[&kind].content {
            csdlc_v2::cards::CardContent::Spp(values) => {
                (&values.design_digest, &values.diagram_digest)
            }
            csdlc_v2::cards::CardContent::Vpp(values) => {
                (&values.design_digest, &values.diagram_digest)
            }
            _ => unreachable!("design-bearing card"),
        };
        assert_eq!(actual_design, &design_digest);
        assert_eq!(actual_diagram, &diagram_digest);
    }

    let mut plan_mutation = edit(
        &reapproved,
        SemanticOperation::UpdatePlanStep {
            step_id: "step-1".into(),
            status: csdlc_v2::cards::StepStatus::Completed,
        },
    );
    plan_mutation.card = CardKind::Spp;
    let mutated = edit_issue(&store, plan_mutation).expect("typed mutation after reapproval");
    let mut review = edit(
        &mutated,
        SemanticOperation::RecordReview {
            reviewer: "independent-reviewer".into(),
            revision: "reviewed-revision".into(),
            result: csdlc_v2::cards::ReviewResult::Pass,
            residual_risk: Vec::new(),
        },
    );
    review.card = CardKind::Srp;
    let reviewed_evidence = edit_issue(&store, review).expect("review evidence");
    let reviewed = edit_issue(
        &store,
        edit(
            &reviewed_evidence,
            SemanticOperation::AdvancePhase {
                phase: csdlc_v2::LifecyclePhase::Reviewed,
            },
        ),
    )
    .expect("reviewed");
    let error = csdlc_v2::approve_design(
        &store,
        csdlc_v2::ApproveDesignRequest {
            issue: 42,
            expected_generation: reviewed.generation,
            expected_digest: reviewed.digest,
            claim_id: "claim-1".into(),
            reviewer: "architect".into(),
        },
    )
    .expect_err("reviewed reapproval must fail closed");
    assert!(matches!(error.code, ErrorCode::InvalidTransition));
}

#[test]
fn design_reapproval_rejects_unrelated_card_projection_drift() {
    let (temp, store, record) = fixture();
    let ready = edit_issue(
        &store,
        edit(
            &record,
            SemanticOperation::AdvancePhase {
                phase: csdlc_v2::LifecyclePhase::Ready,
            },
        ),
    )
    .expect("ready");
    let bound = edit_issue(
        &store,
        edit(
            &ready,
            SemanticOperation::AdvancePhase {
                phase: csdlc_v2::LifecyclePhase::Bound,
            },
        ),
    )
    .expect("bound");
    fs::write(
        temp.path().join("docs/design.md"),
        "# Legitimate bound design revision\n",
    )
    .expect("design edit");
    let sip_path = store.issue_dir(42).join("cards/sip.values.json");
    let mut sip: serde_json::Value =
        serde_json::from_slice(&fs::read(&sip_path).expect("SIP values")).expect("SIP JSON");
    sip["content"]["values"]["goal"] = "unauthorized direct edit".into();
    fs::write(
        &sip_path,
        serde_json::to_vec_pretty(&sip).expect("serialize SIP"),
    )
    .expect("tamper SIP values");

    let error = csdlc_v2::approve_design(
        &store,
        csdlc_v2::ApproveDesignRequest {
            issue: 42,
            expected_generation: bound.generation,
            expected_digest: bound.digest,
            claim_id: "claim-1".into(),
            reviewer: "architect".into(),
        },
    )
    .expect_err("unrelated card drift must not be canonicalized");
    assert!(matches!(error.code, ErrorCode::CorruptRecord));
}

#[test]
fn issue_local_design_paths_do_not_look_like_existing_records() {
    let temp = tempfile::tempdir().expect("tempdir");
    let store = Store::new(temp.path());
    let mut bootstrap = request();
    bootstrap.design_path = ".csdlc/issues/42/design.md".into();
    bootstrap.diagram_path = ".csdlc/issues/42/diagram.mmd".into();
    bootstrap.design_approved = false;

    let record = initialize_issue(&store, bootstrap).expect("issue-local init");
    assert_eq!(record.issue, 42);
    assert!(store.issue_dir(42).join("index.json").exists());
    assert!(store.issue_dir(42).join("design.md").exists());
    assert!(store.issue_dir(42).join("diagram.mmd").exists());
    assert!(!matches!(
        diagnose(&store, 42).status,
        csdlc_v2::doctor::DoctorStatus::Corrupt
    ));
}

#[test]
fn invalid_issue_local_init_fails_before_creating_artifacts() {
    let temp = tempfile::tempdir().expect("tempdir");
    let store = Store::new(temp.path());
    let mut bootstrap = request();
    bootstrap.issue = 43;
    bootstrap.claim.owner.clear();
    bootstrap.design_path = ".csdlc/issues/43/design.md".into();
    bootstrap.diagram_path = ".csdlc/issues/43/diagram.mmd".into();

    let error = initialize_issue(&store, bootstrap).expect_err("invalid claim");
    assert!(matches!(error.code, ErrorCode::InvalidInput));
    assert!(!store.issue_dir(43).exists());
}

#[test]
fn doctor_rejects_index_digest_tampering() {
    let (_temp, store, _record) = fixture();
    let path = store.issue_dir(42).join("index.json");
    let mut index: serde_json::Value =
        serde_json::from_slice(&fs::read(&path).expect("index")).expect("index JSON");
    index["generation"] = 99.into();
    fs::write(&path, serde_json::to_vec_pretty(&index).expect("serialize")).expect("tamper");
    let report = diagnose(&store, 42);
    assert!(matches!(
        report.status,
        csdlc_v2::doctor::DoctorStatus::Corrupt
    ));
    assert!(report.findings[0].message.contains("digest"));
}

#[test]
fn ready_transition_requires_current_design_and_automatic_budgets() {
    let (temp, store, record) = fixture();
    let cards = store.load_cards(42).expect("cards");
    match (
        &cards[&CardKind::Spp].content,
        &cards[&CardKind::Vpp].content,
    ) {
        (csdlc_v2::cards::CardContent::Spp(spp), csdlc_v2::cards::CardContent::Vpp(vpp)) => {
            assert_eq!(spp.execution_estimates.elapsed_seconds, 7_200);
            assert_eq!(spp.execution_estimates.total_tokens, 40_000);
            assert_eq!(vpp.planned_validation_seconds, 1_200);
            assert_eq!(vpp.planned_validation_tokens, 10_000);
        }
        _ => unreachable!("planning cards"),
    }
    let mut over_budget = cards.clone();
    if let csdlc_v2::cards::CardContent::Vpp(vpp) =
        &mut over_budget.get_mut(&CardKind::Vpp).expect("VPP").content
    {
        vpp.lanes[0].budget_tokens = vpp.planned_validation_tokens + 1;
    }
    let design_digest =
        csdlc_v2::cards::digest(&fs::read(store.root().join("docs/design.md")).expect("design"));
    let diagram_digest =
        csdlc_v2::cards::digest(&fs::read(store.root().join("docs/diagram.mmd")).expect("diagram"));
    assert!(csdlc_v2::cards::validate_cross_card(
        &over_budget,
        "docs/design.md",
        &design_digest,
        "docs/diagram.mmd",
        &diagram_digest,
    )
    .is_err());
    fs::remove_file(temp.path().join("docs/design.md")).expect("remove design");
    let error = edit_issue(
        &store,
        edit(
            &record,
            SemanticOperation::AdvancePhase {
                phase: csdlc_v2::LifecyclePhase::Ready,
            },
        ),
    )
    .expect_err("stale design must block ready");
    assert!(matches!(error.code, ErrorCode::Io | ErrorCode::CardInvalid));

    let (_other_temp, other_store, other_record) = fixture();
    let ready = edit_issue(
        &other_store,
        edit(
            &other_record,
            SemanticOperation::AdvancePhase {
                phase: csdlc_v2::LifecyclePhase::Ready,
            },
        ),
    )
    .expect("valid readiness transition");
    assert_eq!(ready.phase, csdlc_v2::LifecyclePhase::Ready);
}

#[test]
fn sor_status_cannot_fabricate_terminal_truth() {
    let (_temp, store, record) = fixture();
    let mut sor = store
        .load_cards(42)
        .expect("cards")
        .remove(&CardKind::Sor)
        .expect("SOR");
    csdlc_v2::cards::apply(
        &mut sor,
        &SemanticOperation::AdvanceStatus {
            status: csdlc_v2::CardStatus::Ready,
        },
    )
    .expect("activate SOR values");
    let error = csdlc_v2::cards::apply(
        &mut sor,
        &SemanticOperation::AdvanceStatus {
            status: csdlc_v2::CardStatus::Complete,
        },
    )
    .expect_err("premature SOR completion");
    assert!(matches!(error.code, ErrorCode::InvalidTransition));

    let empty_evidence = csdlc_v2::cards::apply(
        &mut sor,
        &SemanticOperation::RecordValidation {
            result: csdlc_v2::cards::ValidationResult {
                command: Vec::new(),
                purpose: String::new(),
                outcome: csdlc_v2::cards::EvidenceOutcome::Passed,
                evidence_ref: String::new(),
            },
        },
    )
    .expect_err("empty evidence");
    assert!(matches!(empty_evidence.code, ErrorCode::CardInvalid));

    let ready = edit_issue(
        &store,
        edit(
            &record,
            SemanticOperation::AdvancePhase {
                phase: csdlc_v2::LifecyclePhase::Ready,
            },
        ),
    )
    .expect("ready");
    let bound = edit_issue(
        &store,
        edit(
            &ready,
            SemanticOperation::AdvancePhase {
                phase: csdlc_v2::LifecyclePhase::Bound,
            },
        ),
    )
    .expect("bound");
    let mut premature_closeout = edit(
        &bound,
        SemanticOperation::RecordCloseout {
            integration_state: csdlc_v2::cards::IntegrationState::Merged,
            publication_state: csdlc_v2::cards::PublicationState::Closed,
            merge_state: csdlc_v2::cards::MergeState::Merged,
            closeout_state: csdlc_v2::cards::CloseoutState::Complete,
        },
    );
    premature_closeout.card = CardKind::Sor;
    assert!(matches!(
        edit_issue(&store, premature_closeout)
            .expect_err("closeout while bound")
            .code,
        ErrorCode::InvalidTransition
    ));
}
