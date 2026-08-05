use std::fs;

use csdlc_v2::cards::{digest, PlanStep, ResourceProfile, StepStatus, ValidationLane};
use csdlc_v2::doctor::DoctorStatus;
use csdlc_v2::test_support::{initialize_native_json, BootstrapRequest};
use csdlc_v2::{
    create_issue_draft, diagnose, load_binding_intent, load_preparation_manifest,
    migrate_legacy_preparation, release_derived_bind, repair_legacy_preparation, run_derived_bind,
    run_preparation, run_preparation_batch, seal_preparation, sync_preparation, BatchChildOutcome,
    BindReleaseRequest, BindingIntent, Claim, DerivedBindRequest, ErrorCode, InitialCardInput,
    IssueCreateRequest, LegacyPreparationDisposition, LegacyPreparationMigrationRequest,
    LegacyPreparationRepairDisposition, LegacyPreparationRepairRequest, PlanningProfile,
    PreparationState, PrepareBatchRequest, PrepareRunRequest, PrepareSealRequest,
    PrepareSyncRequest, Store,
};

fn initial(goal: &str) -> InitialCardInput {
    InitialCardInput {
        title: "Preparation fixture".into(),
        slug: "preparation-fixture".into(),
        version: "v0.92".into(),
        goal: goal.into(),
        required_outcome: "Produce claim-free execution readiness.".into(),
        declared_scope: vec!["csdlc-v2".into()],
        authority_boundary: vec!["no execution claim during preparation".into()],
        operator_constraints: vec!["none".into()],
        task_boundary: "Prepare only the fixture.".into(),
        deliverables: vec!["immutable generation".into(), "readiness receipt".into()],
        acceptance_criteria: vec!["AC-1: generation is complete".into()],
        dependencies: vec!["none".into()],
        repo_inputs: vec!["docs/design.md".into()],
        non_goals: vec!["publication".into()],
        plan_summary: "Sync and seal one generation.".into(),
        steps: vec![PlanStep {
            id: "step-1".into(),
            action: "sync and seal".into(),
            acceptance_ids: vec!["AC-1".into()],
            status: StepStatus::Pending,
        }],
        invariants: vec!["preparation is claim-free".into()],
        risks: vec!["stale inputs".into()],
        planning_profile: PlanningProfile::Small,
        stop_conditions: vec!["digest drift".into()],
        validation_lanes: vec![ValidationLane {
            lane: "focused".into(),
            proof_role: "preparation behavior".into(),
            acceptance_ids: vec!["AC-1".into()],
            deterministic: true,
            resource_profile: ResourceProfile::Small,
            budget_seconds: 60,
            budget_tokens: 1_000,
            argv: vec!["cargo".into(), "test".into()],
            parallel_group: "local".into(),
            defer_reason: None,
        }],
        failure_policy: "Fail closed.".into(),
        review_prompts: vec!["Review claim-free truth.".into()],
        review_scope: "fixture".into(),
    }
}

fn fixture() -> (tempfile::TempDir, Store) {
    let temp = tempfile::tempdir().expect("tempdir");
    git(temp.path(), &["init", "-b", "main"]);
    git(temp.path(), &["config", "user.email", "test@example.com"]);
    git(temp.path(), &["config", "user.name", "C-SDLC Test"]);
    fs::create_dir_all(temp.path().join("docs")).expect("docs");
    fs::write(temp.path().join("docs/design.md"), "# Approved design\n").expect("design");
    fs::write(
        temp.path().join("docs/diagram.mmd"),
        "flowchart LR\n  Draft --> Ready\n",
    )
    .expect("diagram");
    git(temp.path(), &["add", "."]);
    git(temp.path(), &["commit", "-m", "baseline"]);
    let store = Store::new(temp.path());
    create_issue_draft(
        &store,
        IssueCreateRequest {
            issue: 5861,
            repository: "example/repo".into(),
            title: "Preparation fixture".into(),
            slug: "preparation-fixture".into(),
            version: "v0.92".into(),
        },
    )
    .expect("draft");
    (temp, store)
}

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

fn git_branch_exists_for_test(root: &std::path::Path, branch: &str) -> bool {
    std::process::Command::new("git")
        .current_dir(root)
        .args(["show-ref", "--verify", "--quiet"])
        .arg(format!("refs/heads/{branch}"))
        .status()
        .unwrap()
        .success()
}

fn write_session_ledger(root: &std::path::Path, claims: &[(u64, String)]) {
    let path = root.join(".adl/session-ledger/ledger.json");
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    let claims = claims
        .iter()
        .map(|(issue, session_id)| {
            serde_json::json!({
                "claim_id": format!("ledger-{issue}"),
                "session_id": session_id,
                "owner": "operator",
                "resource": {"kind": "issue", "id": issue.to_string()},
                "purpose": "test binding",
                "mode": "active",
                "lifecycle_phase": "binding",
                "policy_ref": null,
                "github": {"issue": issue},
                "branch": null,
                "worktree_path": null,
                "do_not_touch_paths": [],
                "blockers": [],
                "created_at": "2026-01-01T00:00:00Z",
                "heartbeat_at": "2026-01-01T00:00:00Z",
                "expires_at": "2999-01-01T00:00:00Z",
                "released_at": null,
                "release_reason": null
            })
        })
        .collect::<Vec<_>>();
    fs::write(
        path,
        serde_json::to_vec_pretty(&serde_json::json!({
            "schema": "adl.session_ledger.v1",
            "updated_at": "2026-01-01T00:00:00Z",
            "global_freeze": null,
            "claims": claims
        }))
        .unwrap(),
    )
    .unwrap();
}

fn current_revision(store: &Store) -> String {
    let output = std::process::Command::new("git")
        .current_dir(store.root())
        .args(["rev-parse", "HEAD"])
        .output()
        .unwrap();
    String::from_utf8(output.stdout).unwrap().trim().to_string()
}

fn sync_request(store: &Store, goal: &str) -> PrepareSyncRequest {
    PrepareSyncRequest {
        issue: 5861,
        repository: "example/repo".into(),
        design_path: "docs/design.md".into(),
        diagram_path: "docs/diagram.mmd".into(),
        design_reviewer: "reviewer".into(),
        design_approved: true,
        owned_paths: vec!["csdlc-v2".into()],
        dependencies: Vec::new(),
        base_revision: current_revision(store),
        initial: initial(goal),
        expected_manifest_digest: None,
    }
}

fn batch_child(
    store: &Store,
    issue: u64,
    slug: &str,
    goal: &str,
    owned_path: &str,
    dependencies: Vec<csdlc_v2::DependencyRevision>,
) -> PrepareRunRequest {
    create_issue_draft(
        store,
        IssueCreateRequest {
            issue,
            repository: "example/repo".into(),
            title: format!("Batch child {issue}"),
            slug: slug.into(),
            version: "v0.92".into(),
        },
    )
    .expect("child draft");
    let mut initial = initial(goal);
    initial.slug = slug.into();
    initial.title = format!("Batch child {issue}");
    PrepareRunRequest {
        sync: PrepareSyncRequest {
            issue,
            repository: "example/repo".into(),
            design_path: "docs/design.md".into(),
            diagram_path: "docs/diagram.mmd".into(),
            design_reviewer: "reviewer".into(),
            design_approved: true,
            owned_paths: vec![owned_path.into()],
            dependencies,
            base_revision: current_revision(store),
            initial,
            expected_manifest_digest: None,
        },
    }
}

#[test]
fn draft_and_preparation_are_claim_free() {
    let (temp, store) = fixture();
    let draft = load_preparation_manifest(&store, 5861).expect("manifest");
    assert_eq!(draft.state, PreparationState::Draft);
    let doctor = diagnose(&store, 5861);
    assert_eq!(doctor.preparation_state, Some(PreparationState::Draft));
    assert_eq!(doctor.next_operation.as_deref(), Some("csdlc-prepare sync"));
    assert!(!temp.path().join(".csdlc/issues/5861/index.json").exists());

    let generation =
        sync_preparation(&store, sync_request(&store, "Prepare the issue.")).expect("sync");
    assert_eq!(generation.sequence, 1);
    assert_eq!(
        load_preparation_manifest(&store, 5861).unwrap().state,
        PreparationState::Prepared
    );
    assert_eq!(
        diagnose(&store, 5861).next_operation.as_deref(),
        Some("csdlc-prepare seal")
    );
    assert!(!temp.path().join(".csdlc/issues/5861/index.json").exists());
}

#[test]
fn manifest_tampering_fails_closed() {
    let (temp, store) = fixture();
    let path = temp
        .path()
        .join(".csdlc/preparation/issues/5861/manifest.json");
    let mut value: serde_json::Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
    value["state"] = serde_json::json!("execution_ready");
    fs::write(&path, serde_json::to_vec_pretty(&value).unwrap()).unwrap();
    let error = load_preparation_manifest(&store, 5861).expect_err("tamper must fail");
    assert_eq!(error.code, ErrorCode::CorruptRecord);
    assert_eq!(diagnose(&store, 5861).status, DoctorStatus::Corrupt);
}

#[cfg(unix)]
#[test]
fn preparation_namespace_symlink_fails_before_external_write() {
    use std::os::unix::fs::symlink;

    let temp = tempfile::tempdir().unwrap();
    let external = tempfile::tempdir().unwrap();
    fs::create_dir_all(temp.path().join(".csdlc")).unwrap();
    symlink(external.path(), temp.path().join(".csdlc/preparation")).unwrap();
    let error = create_issue_draft(
        &Store::new(temp.path()),
        IssueCreateRequest {
            issue: 5861,
            repository: "example/repo".into(),
            title: "Symlink escape".into(),
            slug: "symlink-escape".into(),
            version: "v0.92".into(),
        },
    )
    .expect_err("symlinked namespace must fail closed");
    assert_eq!(error.code, ErrorCode::UnsafeCheckout);
    assert!(fs::read_dir(external.path()).unwrap().next().is_none());
}

#[test]
fn forged_design_approval_invalidates_the_generation_digest() {
    let (temp, store) = fixture();
    let mut request = sync_request(&store, "Prepare without approval.");
    request.design_approved = false;
    request.design_reviewer.clear();
    let generation = sync_preparation(&store, request).unwrap();
    let path = temp
        .path()
        .join(".csdlc/preparation/issues/5861/generations")
        .join(&generation.generation_id)
        .join("generation.json");
    let mut value: serde_json::Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
    value["design_approved"] = serde_json::json!(true);
    value["design_reviewer"] = serde_json::json!("forged-reviewer");
    fs::write(&path, serde_json::to_vec_pretty(&value).unwrap()).unwrap();
    let manifest = load_preparation_manifest(&store, 5861).unwrap();
    let error = seal_preparation(
        &store,
        PrepareSealRequest {
            issue: 5861,
            expected_generation: generation.generation_id,
            expected_semantic_digest: generation.semantic_digest,
            expected_manifest_digest: manifest.digest,
            dependencies: Vec::new(),
        },
    )
    .expect_err("forged approval must fail");
    assert_eq!(error.code, ErrorCode::CorruptRecord);
}

#[test]
fn run_preserves_failed_generation_then_seals_a_successor() {
    let (_temp, store) = fixture();
    let failed = run_preparation(
        &store,
        PrepareRunRequest {
            sync: sync_request(&store, "[TODO] replace this goal"),
        },
    )
    .expect("run result");
    assert!(failed.receipt.is_none());
    assert_eq!(failed.next_operation, "csdlc-prepare seal");
    assert_eq!(
        load_preparation_manifest(&store, 5861).unwrap().state,
        PreparationState::Prepared
    );

    let mut request = sync_request(&store, "Prepare the issue completely.");
    request.expected_manifest_digest =
        Some(load_preparation_manifest(&store, 5861).unwrap().digest);
    let ready = run_preparation(&store, PrepareRunRequest { sync: request }).expect("ready");
    assert_eq!(ready.generation.sequence, 2);
    assert!(ready.receipt.is_some());
    assert_eq!(
        load_preparation_manifest(&store, 5861).unwrap().state,
        PreparationState::ExecutionReady
    );
}

#[test]
fn successor_generation_demotes_readiness_and_rejects_stale_seal() {
    let (_temp, store) = fixture();
    let first =
        sync_preparation(&store, sync_request(&store, "Prepare generation one.")).expect("sync");
    let prepared = load_preparation_manifest(&store, 5861).unwrap();
    let old_seal = PrepareSealRequest {
        issue: 5861,
        expected_generation: first.generation_id.clone(),
        expected_semantic_digest: first.semantic_digest.clone(),
        expected_manifest_digest: prepared.digest,
        dependencies: Vec::new(),
    };
    seal_preparation(&store, old_seal.clone()).expect("seal");

    let mut successor = sync_request(&store, "Prepare generation two.");
    successor.expected_manifest_digest =
        Some(load_preparation_manifest(&store, 5861).unwrap().digest);
    let second = sync_preparation(&store, successor).expect("successor");
    assert_eq!(second.sequence, 2);
    assert_eq!(
        load_preparation_manifest(&store, 5861).unwrap().state,
        PreparationState::Prepared
    );

    let error = seal_preparation(&store, old_seal).expect_err("stale receipt must fail");
    assert_eq!(error.code, ErrorCode::StaleDigest);
}

#[test]
fn batch_retains_successful_children_and_reports_overlap_and_failure() {
    let (_temp, store) = fixture();
    let child_one = batch_child(
        &store,
        6001,
        "child-one",
        "Ready child.",
        "src/one",
        Vec::new(),
    );
    let child_two = batch_child(
        &store,
        6002,
        "child-two",
        "Ready overlap.",
        "src/shared",
        Vec::new(),
    );
    let child_three = batch_child(
        &store,
        6003,
        "child-three",
        "[TODO] blocked overlap.",
        "src/shared/nested",
        Vec::new(),
    );
    let result = run_preparation_batch(
        &store,
        PrepareBatchRequest {
            batch_id: "batch-1".into(),
            children: vec![child_one, child_two, child_three],
        },
    )
    .expect("batch");
    assert!(!result.ready);
    assert_eq!(result.overlap_issues, vec![6002, 6003]);
    assert_eq!(
        result.children[0].outcome,
        BatchChildOutcome::ExecutionReady
    );
    assert_eq!(
        result.children[1].outcome,
        BatchChildOutcome::ExecutionReady
    );
    assert_eq!(result.children[2].outcome, BatchChildOutcome::Prepared);
    assert!(result.children[2].error.is_some());
    assert_eq!(
        load_preparation_manifest(&store, 6001).unwrap().state,
        PreparationState::ExecutionReady
    );
}

#[test]
fn batch_reports_dependency_cycles_without_overstating_child_readiness() {
    let (_temp, store) = fixture();
    let left = batch_child(
        &store,
        6101,
        "cycle-left",
        "Ready left.",
        "src/left",
        vec![csdlc_v2::DependencyRevision {
            issue: 6102,
            revision: "rev-right".into(),
        }],
    );
    let right = batch_child(
        &store,
        6102,
        "cycle-right",
        "Ready right.",
        "src/right",
        vec![csdlc_v2::DependencyRevision {
            issue: 6101,
            revision: "rev-left".into(),
        }],
    );
    let result = run_preparation_batch(
        &store,
        PrepareBatchRequest {
            batch_id: "cycle".into(),
            children: vec![left, right],
        },
    )
    .unwrap();
    assert!(!result.ready);
    assert_eq!(result.cycle_issues, vec![6101, 6102]);
    assert!(result
        .children
        .iter()
        .all(|child| child.outcome == BatchChildOutcome::Prepared));
}

#[test]
fn dependency_drift_blocks_seal() {
    let (_temp, store) = fixture();
    let dependency = run_preparation(
        &store,
        batch_child(
            &store,
            6201,
            "dependency",
            "Ready dependency.",
            "src/dependency",
            Vec::new(),
        ),
    )
    .unwrap();
    let dependency_revision = dependency.receipt.unwrap().digest;
    let child = batch_child(
        &store,
        6202,
        "dependent",
        "Ready dependent.",
        "src/dependent",
        vec![csdlc_v2::DependencyRevision {
            issue: 6201,
            revision: dependency_revision.clone(),
        }],
    );
    let generation = sync_preparation(&store, child.sync).unwrap();
    let manifest = load_preparation_manifest(&store, 6202).unwrap();

    let mut successor = batch_child(
        &store,
        6201,
        "dependency",
        "Changed dependency.",
        "src/dependency",
        Vec::new(),
    );
    successor.sync.expected_manifest_digest =
        Some(load_preparation_manifest(&store, 6201).unwrap().digest);
    sync_preparation(&store, successor.sync).unwrap();

    let error = seal_preparation(
        &store,
        PrepareSealRequest {
            issue: 6202,
            expected_generation: generation.generation_id,
            expected_semantic_digest: generation.semantic_digest,
            expected_manifest_digest: manifest.digest,
            dependencies: vec![csdlc_v2::DependencyRevision {
                issue: 6201,
                revision: dependency_revision,
            }],
        },
    )
    .expect_err("stale dependency must block seal");
    assert_eq!(error.code, ErrorCode::StaleDigest);
}

#[test]
fn seal_rejects_an_unavailable_validation_executable() {
    let (_temp, store) = fixture();
    let mut request = sync_request(&store, "Reject a non-proving validation lane.");
    request.initial.validation_lanes[0].argv = vec!["definitely-not-installed-5861".into()];
    let result = run_preparation(&store, PrepareRunRequest { sync: request }).unwrap();
    assert!(result.receipt.is_none());
    assert_eq!(result.error_code.as_deref(), Some("invalid_input"));
}

#[cfg(unix)]
#[test]
fn seal_rejects_a_validation_file_without_an_executable_bit() {
    let (_temp, store) = fixture();
    let mut request = sync_request(&store, "Reject a non-executable validation lane.");
    request.initial.validation_lanes[0].argv = vec!["docs/design.md".into()];
    let result = run_preparation(&store, PrepareRunRequest { sync: request }).unwrap();
    assert!(result.receipt.is_none());
    assert_eq!(result.error_code.as_deref(), Some("invalid_input"));
}

#[cfg(unix)]
#[test]
fn sync_rejects_symlinked_design_input() {
    use std::os::unix::fs::symlink;

    let (temp, store) = fixture();
    let outside = temp.path().join("outside-design.md");
    fs::write(&outside, "outside\n").unwrap();
    fs::remove_file(temp.path().join("docs/design.md")).unwrap();
    symlink(&outside, temp.path().join("docs/design.md")).unwrap();
    let error = sync_preparation(&store, sync_request(&store, "Reject symlink input."))
        .expect_err("symlinked design must fail closed");
    assert_eq!(error.code, ErrorCode::UnsafeCheckout);
}

#[test]
fn batch_id_is_immutable_across_different_results() {
    let (_temp, store) = fixture();
    let request = PrepareBatchRequest {
        batch_id: "immutable-batch".into(),
        children: vec![batch_child(
            &store,
            6301,
            "immutable-child",
            "First result.",
            "src/immutable",
            Vec::new(),
        )],
    };
    run_preparation_batch(&store, request).unwrap();
    let changed = PrepareBatchRequest {
        batch_id: "immutable-batch".into(),
        children: vec![batch_child(
            &store,
            6302,
            "different-child",
            "Different result.",
            "src/different",
            Vec::new(),
        )],
    };
    let error = run_preparation_batch(&store, changed)
        .expect_err("same batch id cannot overwrite immutable truth");
    assert_eq!(error.code, ErrorCode::CorruptRecord);
}

#[test]
fn concurrent_writers_cannot_replace_an_immutable_batch_result() {
    let (_temp, store) = fixture();
    let requests = [
        PrepareBatchRequest {
            batch_id: "contended-batch".into(),
            children: vec![batch_child(
                &store,
                6311,
                "contended-left",
                "Left result.",
                "src/left",
                Vec::new(),
            )],
        },
        PrepareBatchRequest {
            batch_id: "contended-batch".into(),
            children: vec![batch_child(
                &store,
                6312,
                "contended-right",
                "Right result.",
                "src/right",
                Vec::new(),
            )],
        },
    ];
    let barrier = std::sync::Arc::new(std::sync::Barrier::new(3));
    let results = std::thread::scope(|scope| {
        let handles = requests.map(|request| {
            let store = store.clone();
            let barrier = barrier.clone();
            scope.spawn(move || {
                barrier.wait();
                run_preparation_batch(&store, request)
            })
        });
        barrier.wait();
        handles.map(|handle| handle.join().unwrap())
    });
    assert_eq!(results.iter().filter(|result| result.is_ok()).count(), 1);
    assert_eq!(
        results
            .iter()
            .filter_map(|result| result.as_ref().err())
            .filter(|error| error.code == ErrorCode::CorruptRecord)
            .count(),
        1
    );
}

#[test]
fn migration_releases_preparation_only_claim_and_preserves_snapshot() {
    let temp = tempfile::tempdir().expect("tempdir");
    git(temp.path(), &["init", "-b", "main"]);
    fs::create_dir_all(temp.path().join("docs/templates/prompts")).unwrap();
    fs::create_dir_all(temp.path().join("csdlc-v2/operator")).unwrap();
    fs::write(
        temp.path().join("docs/templates/prompts/current.json"),
        include_bytes!("../../docs/templates/prompts/current.json"),
    )
    .unwrap();
    fs::write(
        temp.path().join("csdlc-v2/operator/native-card-shape.json"),
        include_bytes!("../operator/native-card-shape.json"),
    )
    .unwrap();
    fs::write(temp.path().join("docs/design.md"), "# Approved design\n").unwrap();
    fs::write(
        temp.path().join("docs/diagram.mmd"),
        "flowchart LR\n  Legacy --> Prepared\n",
    )
    .unwrap();
    let store = Store::new(temp.path());
    let mut migrated_initial = initial("Migrate preparation truth.");
    migrated_initial.slug = "legacy-preparation".into();
    let record = initialize_native_json(
        &store,
        &serde_json::to_vec(&BootstrapRequest {
            issue: 7001,
            repository: "example/repo".into(),
            design_path: "docs/design.md".into(),
            diagram_path: "docs/diagram.mmd".into(),
            design_reviewer: "reviewer".into(),
            design_approved: true,
            claim: Claim {
                id: "legacy-preparation-claim".into(),
                owner: "legacy-preparer".into(),
                generation: 0,
                acquired_unix_seconds: 1,
                expires_unix_seconds: u64::MAX,
                heartbeat_unix_seconds: 1,
                branch: "codex/7001-legacy-preparation".into(),
                worktree: ".worktrees/7001-legacy-preparation".into(),
                protected_paths: vec!["csdlc-v2".into()],
                purpose: "legacy preparation".into(),
            },
            initial: migrated_initial,
            prepared_cards: None,
        })
        .unwrap(),
    )
    .expect("legacy init");
    create_issue_draft(
        &store,
        IssueCreateRequest {
            issue: 7001,
            repository: "example/repo".into(),
            title: "Preparation fixture".into(),
            slug: "legacy-preparation".into(),
            version: "v0.92".into(),
        },
    )
    .unwrap();
    let snapshot = temp
        .path()
        .join(".csdlc/preparation/issues/7001/migration")
        .join(format!("legacy-{}.json", record.digest));
    fs::create_dir_all(snapshot.parent().unwrap()).unwrap();
    fs::write(&snapshot, serde_json::to_vec_pretty(&record).unwrap()).unwrap();
    let interrupted = diagnose(&store, 7001);
    assert_eq!(interrupted.status, DoctorStatus::Corrupt);
    assert_eq!(
        interrupted.next_operation.as_deref(),
        Some("csdlc-migrate preparation")
    );
    let result = migrate_legacy_preparation(
        &store,
        LegacyPreparationMigrationRequest {
            issue: 7001,
            expected_legacy_digest: record.digest,
            actor: "operator".into(),
            reason: "move claim-free preparation to v0.92".into(),
            base_revision: "abc123".into(),
        },
    )
    .expect("migration");
    assert_eq!(
        result.disposition,
        LegacyPreparationDisposition::ImportedPrepared
    );
    assert!(!store.issue_dir(7001).exists());
    assert_eq!(
        load_preparation_manifest(&store, 7001).unwrap().state,
        PreparationState::Prepared
    );
    assert!(temp.path().join(result.snapshot_path.unwrap()).is_file());
}

#[test]
fn ambiguous_migration_requires_digest_pinned_typed_repair() {
    let temp = tempfile::tempdir().expect("tempdir");
    git(temp.path(), &["init", "-b", "main"]);
    fs::create_dir_all(temp.path().join("docs/templates/prompts")).unwrap();
    fs::create_dir_all(temp.path().join("csdlc-v2/operator")).unwrap();
    fs::write(
        temp.path().join("docs/templates/prompts/current.json"),
        include_bytes!("../../docs/templates/prompts/current.json"),
    )
    .unwrap();
    fs::write(
        temp.path().join("csdlc-v2/operator/native-card-shape.json"),
        include_bytes!("../operator/native-card-shape.json"),
    )
    .unwrap();
    fs::write(temp.path().join("docs/design.md"), "# Original design\n").unwrap();
    fs::write(
        temp.path().join("docs/diagram.mmd"),
        "flowchart LR\n  Legacy --> Prepared\n",
    )
    .unwrap();
    let store = Store::new(temp.path());
    let mut legacy_initial = initial("Repair ambiguous preparation truth.");
    legacy_initial.slug = "legacy-repair".into();
    let record = initialize_native_json(
        &store,
        &serde_json::to_vec(&BootstrapRequest {
            issue: 7002,
            repository: "example/repo".into(),
            design_path: "docs/design.md".into(),
            diagram_path: "docs/diagram.mmd".into(),
            design_reviewer: "reviewer".into(),
            design_approved: true,
            claim: Claim {
                id: "legacy-repair-claim".into(),
                owner: "legacy-preparer".into(),
                generation: 0,
                acquired_unix_seconds: 1,
                expires_unix_seconds: u64::MAX,
                heartbeat_unix_seconds: 1,
                branch: "codex/7002-legacy-repair".into(),
                worktree: ".worktrees/7002-legacy-repair".into(),
                protected_paths: vec!["csdlc-v2".into()],
                purpose: "legacy preparation".into(),
            },
            initial: legacy_initial,
            prepared_cards: None,
        })
        .unwrap(),
    )
    .expect("legacy init");
    fs::write(temp.path().join("docs/design.md"), "# Drifted design\n").unwrap();

    let quarantine = migrate_legacy_preparation(
        &store,
        LegacyPreparationMigrationRequest {
            issue: 7002,
            expected_legacy_digest: record.digest.clone(),
            actor: "operator".into(),
            reason: "classify ambiguous preparation".into(),
            base_revision: "abc123".into(),
        },
    )
    .expect("quarantine");
    assert_eq!(
        quarantine.disposition,
        LegacyPreparationDisposition::Quarantined
    );
    assert_eq!(quarantine.next_operation, "csdlc-migrate repair");

    let error = migrate_legacy_preparation(
        &store,
        LegacyPreparationMigrationRequest {
            issue: 7002,
            expected_legacy_digest: record.digest.clone(),
            actor: "operator".into(),
            reason: "attempt to replace immutable quarantine evidence".into(),
            base_revision: "abc123".into(),
        },
    )
    .expect_err("classified migration evidence cannot be replaced");
    assert_eq!(error.code, ErrorCode::ReconciliationRequired);

    #[cfg(unix)]
    {
        use std::os::unix::fs::symlink;

        let original = quarantine.snapshot_path.as_ref().unwrap();
        let alias = ".csdlc/preparation/issues/7002/migration/quarantine-alias.json";
        symlink(
            temp.path().join(original).file_name().unwrap(),
            temp.path().join(alias),
        )
        .unwrap();
        let error = repair_legacy_preparation(
            &store,
            LegacyPreparationRepairRequest {
                issue: 7002,
                expected_legacy_digest: record.digest.clone(),
                expected_quarantine_digest: quarantine.resulting_digest.clone().unwrap(),
                expected_preparation_digest: None,
                quarantine_path: alias.into(),
                disposition: LegacyPreparationRepairDisposition::RetainLegacyAuthority,
                actor: "operator".into(),
                reason: "reject an aliased quarantine packet".into(),
            },
        )
        .expect_err("symlinked quarantine packet must fail closed");
        assert_eq!(error.code, ErrorCode::UnsafeCheckout);
    }

    let repair = repair_legacy_preparation(
        &store,
        LegacyPreparationRepairRequest {
            issue: 7002,
            expected_legacy_digest: record.digest,
            expected_quarantine_digest: quarantine.resulting_digest.unwrap(),
            expected_preparation_digest: None,
            quarantine_path: quarantine.snapshot_path.unwrap(),
            disposition: LegacyPreparationRepairDisposition::RetainLegacyAuthority,
            actor: "operator".into(),
            reason: "retain unchanged legacy authority for explicit follow-up".into(),
        },
    )
    .expect("repair");
    assert!(repair.repaired);
    assert_eq!(repair.next_operation, "continue_existing_lifecycle");
    assert!(temp.path().join(repair.audit_path).is_file());
    assert_eq!(
        store.load_record(7002).unwrap().claim.unwrap().id,
        "legacy-repair-claim"
    );
}

#[test]
fn derived_bind_is_retryable_and_release_removes_only_created_git_artifacts() {
    let temp = tempfile::tempdir().expect("tempdir");
    git(temp.path(), &["init", "-b", "main"]);
    git(temp.path(), &["config", "user.email", "test@example.com"]);
    git(temp.path(), &["config", "user.name", "C-SDLC Test"]);
    fs::create_dir_all(temp.path().join("docs/templates/prompts")).unwrap();
    fs::create_dir_all(temp.path().join("csdlc-v2/operator")).unwrap();
    fs::write(
        temp.path().join("docs/templates/prompts/current.json"),
        include_bytes!("../../docs/templates/prompts/current.json"),
    )
    .unwrap();
    fs::write(
        temp.path().join("csdlc-v2/operator/native-card-shape.json"),
        include_bytes!("../operator/native-card-shape.json"),
    )
    .unwrap();
    fs::write(temp.path().join("docs/design.md"), "# Approved design\n").unwrap();
    fs::write(
        temp.path().join("docs/diagram.mmd"),
        "flowchart LR\n  Ready --> Bound\n",
    )
    .unwrap();
    git(temp.path(), &["add", "."]);
    git(temp.path(), &["commit", "-m", "baseline"]);
    let base_revision = std::process::Command::new("git")
        .current_dir(temp.path())
        .args(["rev-parse", "HEAD"])
        .output()
        .unwrap();
    let base_revision = String::from_utf8(base_revision.stdout)
        .unwrap()
        .trim()
        .to_string();
    let store = Store::new(temp.path());
    create_issue_draft(
        &store,
        IssueCreateRequest {
            issue: 8001,
            repository: "example/repo".into(),
            title: "Derived bind".into(),
            slug: "derived-bind".into(),
            version: "v0.92".into(),
        },
    )
    .unwrap();
    let mut bind_initial = initial("Bind without copied claims.");
    bind_initial.slug = "derived-bind".into();
    let prepared = run_preparation(
        &store,
        PrepareRunRequest {
            sync: PrepareSyncRequest {
                issue: 8001,
                repository: "example/repo".into(),
                design_path: "docs/design.md".into(),
                diagram_path: "docs/diagram.mmd".into(),
                design_reviewer: "reviewer".into(),
                design_approved: true,
                owned_paths: vec!["src/owned".into()],
                dependencies: Vec::new(),
                base_revision: base_revision.clone(),
                initial: bind_initial,
                expected_manifest_digest: None,
            },
        },
    )
    .unwrap();
    assert!(prepared.receipt.is_some());
    let request = DerivedBindRequest {
        issue: 8001,
        session_id: "session-8001".into(),
        base_branch: "main".into(),
        expected_base_revision: base_revision,
        lease_seconds: 3_600,
    };
    write_session_ledger(temp.path(), &[(8001, "session-8001".into())]);
    fs::write(temp.path().join("dirty.txt"), "unrelated\n").unwrap();
    let error = run_derived_bind(&store, request.clone()).expect_err("dirty bind must fail");
    assert_eq!(error.code, ErrorCode::UnsafeCheckout);
    fs::remove_file(temp.path().join("dirty.txt")).unwrap();
    let receipt_path = temp
        .path()
        .join(".csdlc/preparation/issues/8001/receipt.json");
    let original_receipt = fs::read(&receipt_path).unwrap();
    let mut forged: serde_json::Value = serde_json::from_slice(&original_receipt).unwrap();
    forged["base_revision"] = serde_json::json!("forged");
    fs::write(&receipt_path, serde_json::to_vec_pretty(&forged).unwrap()).unwrap();
    let error = run_derived_bind(&store, request.clone()).expect_err("forged receipt must fail");
    assert_eq!(error.code, ErrorCode::CorruptRecord);
    fs::write(&receipt_path, original_receipt).unwrap();
    let ledger_path = temp.path().join(".adl/session-ledger/ledger.json");
    let ledger = fs::read(&ledger_path).unwrap();
    fs::remove_file(&ledger_path).unwrap();
    let error = run_derived_bind(&store, request.clone()).expect_err("ledger is required");
    assert_eq!(error.code, ErrorCode::MissingClaim);
    fs::write(&ledger_path, ledger).unwrap();
    write_session_ledger(
        temp.path(),
        &[
            (8001, "session-8001".into()),
            (8001, "competing-session".into()),
        ],
    );
    let error = run_derived_bind(&store, request.clone()).expect_err("competing claim blocks bind");
    assert_eq!(error.code, ErrorCode::ClaimCollision);
    write_session_ledger(temp.path(), &[(8001, "session-8001".into())]);
    git(temp.path(), &["branch", "codex/8001-derived-bind"]);
    let error = run_derived_bind(&store, request.clone()).expect_err("existing branch blocks bind");
    assert_eq!(error.code, ErrorCode::ClaimCollision);
    assert!(git_branch_exists_for_test(
        temp.path(),
        "codex/8001-derived-bind"
    ));
    git(temp.path(), &["branch", "-D", "codex/8001-derived-bind"]);
    let first = run_derived_bind(&store, request.clone()).expect("bind");
    assert!(first.bind.created);
    assert_eq!(first.owner, "operator");
    assert!(temp.path().join(&first.worktree).is_dir());
    let retry = run_derived_bind(&store, request).expect("retry");
    assert!(!retry.bind.created);

    let common = std::process::Command::new("git")
        .current_dir(temp.path())
        .args(["rev-parse", "--path-format=absolute", "--git-common-dir"])
        .output()
        .unwrap();
    let intent_path =
        std::path::PathBuf::from(String::from_utf8(common.stdout).unwrap().trim().to_string())
            .join("csdlc-v2/binding-intents/8001.json");
    let mut intent: BindingIntent =
        serde_json::from_slice(&fs::read(&intent_path).unwrap()).unwrap();
    let original_intent = fs::read(&intent_path).unwrap();
    intent.owner.clear();
    intent.digest.clear();
    intent.digest = digest(&serde_json::to_vec(&intent).unwrap());
    fs::write(&intent_path, serde_json::to_vec_pretty(&intent).unwrap()).unwrap();
    let error = load_binding_intent(&store, 8001).expect_err("invalid intent identity must fail");
    assert_eq!(error.code, ErrorCode::CorruptRecord);
    fs::write(&intent_path, original_intent).unwrap();
    let intent: BindingIntent = serde_json::from_slice(&fs::read(&intent_path).unwrap()).unwrap();
    let materialized_card = temp
        .path()
        .join(&first.worktree)
        .join(".csdlc/preparation/issues/8001/generations")
        .join(&intent.generation_id)
        .join("design.snapshot");
    let original_card = fs::read(&materialized_card).unwrap();
    fs::write(&materialized_card, "# drifted lifecycle card\n").unwrap();
    let error = release_derived_bind(
        &store,
        BindReleaseRequest {
            issue: 8001,
            session_id: "session-8001".into(),
            expected_intent_digest: None,
        },
    )
    .expect_err("dirty lifecycle drift must fail closed");
    assert_eq!(error.code, ErrorCode::ReconciliationRequired);
    fs::write(&materialized_card, original_card).unwrap();

    let mut intent: BindingIntent =
        serde_json::from_slice(&fs::read(&intent_path).unwrap()).unwrap();
    // Simulate process loss after Git mutation but before artifact-ledger update.
    intent.created_artifacts.clear();
    intent.state = csdlc_v2::BindingIntentState::Bound;
    intent.acquired_unix_seconds = 0;
    intent.expires_unix_seconds = 0;
    intent.digest.clear();
    intent.digest = digest(&serde_json::to_vec(&intent).unwrap());
    fs::write(&intent_path, serde_json::to_vec_pretty(&intent).unwrap()).unwrap();
    write_session_ledger(temp.path(), &[(8001, "replacement-session".into())]);
    let released = release_derived_bind(
        &store,
        BindReleaseRequest {
            issue: 8001,
            session_id: "replacement-session".into(),
            expected_intent_digest: Some(intent.digest),
        },
    )
    .expect("digest-pinned takeover recovers interrupted artifact evidence");
    assert!(released.released);
    assert!(!temp.path().join(first.worktree).exists());
    assert_eq!(
        load_preparation_manifest(&store, 8001).unwrap().state,
        PreparationState::ExecutionReady
    );
}

#[test]
fn derived_bind_uses_the_existing_issue_worktree_in_place() {
    let temp = tempfile::tempdir().expect("tempdir");
    git(temp.path(), &["init", "-b", "main"]);
    git(temp.path(), &["config", "user.email", "test@example.com"]);
    git(temp.path(), &["config", "user.name", "C-SDLC Test"]);
    fs::create_dir_all(temp.path().join("docs/templates/prompts")).unwrap();
    fs::create_dir_all(temp.path().join("csdlc-v2/operator")).unwrap();
    fs::write(
        temp.path().join("docs/templates/prompts/current.json"),
        include_bytes!("../../docs/templates/prompts/current.json"),
    )
    .unwrap();
    fs::write(
        temp.path().join("csdlc-v2/operator/native-card-shape.json"),
        include_bytes!("../operator/native-card-shape.json"),
    )
    .unwrap();
    fs::write(temp.path().join("docs/design.md"), "# Approved design\n").unwrap();
    fs::write(
        temp.path().join("docs/diagram.mmd"),
        "flowchart LR\n  Prepared --> Bound\n",
    )
    .unwrap();
    git(temp.path(), &["add", "."]);
    git(temp.path(), &["commit", "-m", "baseline"]);
    git(temp.path(), &["switch", "-c", "codex/9001-issue-local"]);
    let base_revision = std::process::Command::new("git")
        .current_dir(temp.path())
        .args(["rev-parse", "HEAD"])
        .output()
        .unwrap();
    let base_revision = String::from_utf8(base_revision.stdout)
        .unwrap()
        .trim()
        .to_string();
    let store = Store::new(temp.path());
    create_issue_draft(
        &store,
        IssueCreateRequest {
            issue: 9001,
            repository: "example/repo".into(),
            title: "Issue-local bind".into(),
            slug: "issue-local".into(),
            version: "v0.92".into(),
        },
    )
    .unwrap();
    let mut bind_initial = initial("Bind this worktree in place.");
    bind_initial.slug = "issue-local".into();
    run_preparation(
        &store,
        PrepareRunRequest {
            sync: PrepareSyncRequest {
                issue: 9001,
                repository: "example/repo".into(),
                design_path: "docs/design.md".into(),
                diagram_path: "docs/diagram.mmd".into(),
                design_reviewer: "reviewer".into(),
                design_approved: true,
                owned_paths: vec!["src/issue-local".into()],
                dependencies: Vec::new(),
                base_revision: base_revision.clone(),
                initial: bind_initial,
                expected_manifest_digest: None,
            },
        },
    )
    .unwrap();
    write_session_ledger(temp.path(), &[(9001, "session-9001".into())]);
    let result = run_derived_bind(
        &store,
        DerivedBindRequest {
            issue: 9001,
            session_id: "session-9001".into(),
            base_branch: "main".into(),
            expected_base_revision: base_revision.clone(),
            lease_seconds: 3_600,
        },
    )
    .expect("issue-local bind");
    assert!(!result.bind.created);
    assert_eq!(result.worktree, ".");
    let mut post_bind_initial = initial("Do not edit claim-free state after bind.");
    post_bind_initial.slug = "issue-local".into();
    let error = sync_preparation(
        &store,
        PrepareSyncRequest {
            issue: 9001,
            repository: "example/repo".into(),
            design_path: "docs/design.md".into(),
            diagram_path: "docs/diagram.mmd".into(),
            design_reviewer: "reviewer".into(),
            design_approved: true,
            owned_paths: vec!["src/issue-local".into()],
            dependencies: Vec::new(),
            base_revision,
            initial: post_bind_initial,
            expected_manifest_digest: None,
        },
    )
    .expect_err("bound preparation edit must fail");
    assert_eq!(error.code, ErrorCode::InvalidTransition);
    fs::create_dir_all(temp.path().join("src")).unwrap();
    fs::write(temp.path().join("src/issue-local.rs"), "fn started() {}\n").unwrap();
    let error = release_derived_bind(
        &store,
        BindReleaseRequest {
            issue: 9001,
            session_id: "session-9001".into(),
            expected_intent_digest: None,
        },
    )
    .expect_err("issue-local implementation work must block release");
    assert_eq!(error.code, ErrorCode::UnsafeCheckout);
    fs::remove_file(temp.path().join("src/issue-local.rs")).unwrap();
    release_derived_bind(
        &store,
        BindReleaseRequest {
            issue: 9001,
            session_id: "session-9001".into(),
            expected_intent_digest: None,
        },
    )
    .expect("issue-local release");
    assert_eq!(
        std::process::Command::new("git")
            .current_dir(temp.path())
            .args(["branch", "--show-current"])
            .output()
            .unwrap()
            .stdout,
        b"codex/9001-issue-local\n"
    );
}

#[test]
fn ten_overlapping_binds_have_one_git_winner() {
    let temp = tempfile::tempdir().expect("tempdir");
    git(temp.path(), &["init", "-b", "main"]);
    git(temp.path(), &["config", "user.email", "test@example.com"]);
    git(temp.path(), &["config", "user.name", "C-SDLC Test"]);
    fs::create_dir_all(temp.path().join("docs/templates/prompts")).unwrap();
    fs::create_dir_all(temp.path().join("csdlc-v2/operator")).unwrap();
    fs::write(
        temp.path().join("docs/templates/prompts/current.json"),
        include_bytes!("../../docs/templates/prompts/current.json"),
    )
    .unwrap();
    fs::write(
        temp.path().join("csdlc-v2/operator/native-card-shape.json"),
        include_bytes!("../operator/native-card-shape.json"),
    )
    .unwrap();
    fs::write(temp.path().join("docs/design.md"), "# Approved design\n").unwrap();
    fs::write(
        temp.path().join("docs/diagram.mmd"),
        "flowchart LR\n  Ready --> Winner\n",
    )
    .unwrap();
    git(temp.path(), &["add", "."]);
    git(temp.path(), &["commit", "-m", "baseline"]);
    let base_revision = std::process::Command::new("git")
        .current_dir(temp.path())
        .args(["rev-parse", "HEAD"])
        .output()
        .unwrap();
    let base_revision = String::from_utf8(base_revision.stdout)
        .unwrap()
        .trim()
        .to_string();
    let store = Store::new(temp.path());
    let mut requests = Vec::new();
    for offset in 0..10_u64 {
        let issue = 10_000 + offset;
        let slug = format!("overlap-{offset}");
        create_issue_draft(
            &store,
            IssueCreateRequest {
                issue,
                repository: "example/repo".into(),
                title: format!("Overlap {offset}"),
                slug: slug.clone(),
                version: "v0.92".into(),
            },
        )
        .unwrap();
        let mut child_initial = initial("Compete for one owned path.");
        child_initial.slug = slug;
        run_preparation(
            &store,
            PrepareRunRequest {
                sync: PrepareSyncRequest {
                    issue,
                    repository: "example/repo".into(),
                    design_path: "docs/design.md".into(),
                    diagram_path: "docs/diagram.mmd".into(),
                    design_reviewer: "reviewer".into(),
                    design_approved: true,
                    owned_paths: vec!["src/exclusive".into()],
                    dependencies: Vec::new(),
                    base_revision: base_revision.clone(),
                    initial: child_initial,
                    expected_manifest_digest: None,
                },
            },
        )
        .unwrap();
        requests.push(DerivedBindRequest {
            issue,
            session_id: format!("session-{issue}"),
            base_branch: "main".into(),
            expected_base_revision: base_revision.clone(),
            lease_seconds: 3_600,
        });
    }
    write_session_ledger(
        temp.path(),
        &requests
            .iter()
            .map(|request| (request.issue, request.session_id.clone()))
            .collect::<Vec<_>>(),
    );
    let barrier = std::sync::Arc::new(std::sync::Barrier::new(requests.len()));
    let handles = requests
        .into_iter()
        .map(|request| {
            let barrier = barrier.clone();
            let store = store.clone();
            std::thread::spawn(move || {
                barrier.wait();
                (request.issue, run_derived_bind(&store, request))
            })
        })
        .collect::<Vec<_>>();
    let results = handles
        .into_iter()
        .map(|handle| handle.join().expect("thread"))
        .collect::<Vec<_>>();
    let winners = results
        .iter()
        .filter_map(|(issue, result)| result.as_ref().ok().map(|value| (*issue, value)))
        .collect::<Vec<_>>();
    assert_eq!(winners.len(), 1, "{results:#?}");
    assert!(
        results
            .iter()
            .filter_map(|(_, result)| result.as_ref().err())
            .all(|error| error.code == ErrorCode::ClaimCollision),
        "{results:#?}"
    );
    let worktrees = std::process::Command::new("git")
        .current_dir(temp.path())
        .args(["worktree", "list", "--porcelain"])
        .output()
        .unwrap();
    let worktrees = String::from_utf8(worktrees.stdout).unwrap();
    assert_eq!(worktrees.matches("branch refs/heads/codex/10").count(), 1);
}
