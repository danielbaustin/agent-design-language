use std::fs;

use csdlc_v2::{
    diagnose, edit_issue, BootstrapRequest, CardKind, Claim, EditRequest, ErrorCode,
    SemanticOperation, Store,
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
        },
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
    let record = csdlc_v2::initialize_issue(&store, request.clone()).expect("initialize");
    assert!(temp.path().join("docs/design.md").exists());
    assert!(temp.path().join("docs/diagram.mmd").exists());
    assert_eq!(
        csdlc_v2::initialize_issue(&store, request).expect("idempotent init"),
        record
    );
    (temp, store, record)
}

#[test]
fn bind_creates_and_idempotently_reuses_typed_worktree() {
    let (temp, store, record) = fixture();
    git(temp.path(), &["init", "-b", "main"]);
    git(
        temp.path(),
        &["config", "user.email", "test@example.invalid"],
    );
    git(temp.path(), &["config", "user.name", "C-SDLC Test"]);
    git(temp.path(), &["add", "docs"]);
    git(temp.path(), &["commit", "-m", "fixture"]);
    let claim = record.claim.clone().expect("claim");
    let request = csdlc_v2::BindRequest {
        issue: 42,
        base_branch: "main".into(),
        branch: claim.branch.clone(),
        worktree: claim.worktree.clone(),
        claim,
    };
    let first = csdlc_v2::bind_issue(&store, request.clone()).expect("bind");
    assert!(first.created);
    let bound_digest = store.load_record(42).expect("bound record").digest;
    assert_eq!(
        store.load_record(42).expect("bound record").phase,
        csdlc_v2::LifecyclePhase::Bound
    );
    let second = csdlc_v2::bind_issue(&store, request).expect("rebind");
    assert!(!second.created);
    assert_eq!(
        store.load_record(42).expect("reused record").digest,
        bound_digest
    );
}

#[test]
fn bind_refuses_primary_checkout_and_worktree_mismatch() {
    let (temp, store, record) = fixture();
    git(temp.path(), &["init", "-b", "wrong"]);
    git(
        temp.path(),
        &["config", "user.email", "test@example.invalid"],
    );
    git(temp.path(), &["config", "user.name", "C-SDLC Test"]);
    git(temp.path(), &["add", "docs"]);
    git(temp.path(), &["commit", "-m", "fixture"]);
    let claim = record.claim.clone().expect("claim");
    let error = csdlc_v2::bind_issue(
        &store,
        csdlc_v2::BindRequest {
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
    let error = csdlc_v2::bind_issue(
        &store,
        csdlc_v2::BindRequest {
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
    other.claim.as_mut().expect("claim").protected_paths = vec!["src/nested".into()];
    fs::create_dir_all(store.issue_dir(43)).expect("other issue");
    fs::write(
        store.issue_dir(43).join("index.json"),
        serde_json::to_vec(&other).expect("json"),
    )
    .expect("other record");
    let claim = record.claim.clone().expect("claim");
    let error = csdlc_v2::bind_issue(
        &store,
        csdlc_v2::BindRequest {
            issue: 42,
            base_branch: "main".into(),
            branch: claim.branch.clone(),
            worktree: claim.worktree.clone(),
            claim,
        },
    )
    .expect_err("path overlap");
    assert!(matches!(error.code, ErrorCode::ClaimCollision));
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
    let (_temp, store, record) = fixture();
    csdlc_v2::heartbeat_claim(&store, 42, "claim-1", 0, 2, 60).expect("heartbeat");
    let replacement = Claim {
        id: "replacement".into(),
        owner: "next".into(),
        generation: 0,
        acquired_unix_seconds: 62,
        expires_unix_seconds: u64::MAX,
        heartbeat_unix_seconds: 62,
        branch: "issue-42".into(),
        worktree: ".worktrees/issue-42".into(),
        protected_paths: vec!["src".into()],
        purpose: "explicit recovery".into(),
    };
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

fn edit(record: &csdlc_v2::IssueRecord, operation: SemanticOperation) -> EditRequest {
    EditRequest {
        issue: 42,
        card: CardKind::Sip,
        expected_generation: record.generation,
        expected_digest: record.digest.clone(),
        claim_id: "claim-1".into(),
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
    assert_eq!(store.load_cards(42).expect("cards").len(), 6);
    assert_eq!(
        sip_goal(&store.load_cards(42).expect("cards")),
        "Prove Gate 2."
    );
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
        "bootstrap_request",
        "approve_design_request",
        "edit_request",
        "bind_request",
        "bind_result",
        "recover_claim_request",
        "issue_record",
        "doctor_report",
    ] {
        assert!(schema[key].is_object(), "missing schema for {key}");
        assert!(
            schema[key]["properties"].is_object(),
            "missing root properties for {key}"
        );
    }
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
    let record = csdlc_v2::initialize_issue(&store, request()).expect("placeholder init");
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
        csdlc_v2::cards::digest(&fs::read(temp.path().join("docs/design.md")).expect("design"));
    let diagram_digest =
        csdlc_v2::cards::digest(&fs::read(temp.path().join("docs/diagram.mmd")).expect("diagram"));
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
    csdlc_v2::bind_issue(
        &store,
        csdlc_v2::BindRequest {
            issue: 42,
            base_branch: "main".into(),
            branch: claim.branch.clone(),
            worktree: claim.worktree.clone(),
            claim,
        },
    )
    .expect("bind");
    assert_eq!(
        store.load_record(42).expect("record").phase,
        csdlc_v2::LifecyclePhase::Bound
    );
}

#[test]
fn issue_local_design_paths_do_not_look_like_existing_records() {
    let temp = tempfile::tempdir().expect("tempdir");
    let store = Store::new(temp.path());
    let mut bootstrap = request();
    bootstrap.design_path = ".csdlc/issues/42/design.md".into();
    bootstrap.diagram_path = ".csdlc/issues/42/diagram.mmd".into();
    bootstrap.design_approved = false;

    let record = csdlc_v2::initialize_issue(&store, bootstrap).expect("issue-local init");
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

    let error = csdlc_v2::initialize_issue(&store, bootstrap).expect_err("invalid claim");
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
        csdlc_v2::cards::digest(&fs::read(temp.path().join("docs/design.md")).expect("design"));
    let diagram_digest =
        csdlc_v2::cards::digest(&fs::read(temp.path().join("docs/diagram.mmd")).expect("diagram"));
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
