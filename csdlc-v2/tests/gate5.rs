use csdlc_v2::cards::{FindingDisposition, FindingSeverity};
use csdlc_v2::{
    assign_review, edit_issue, evaluate_publication_review, evaluate_publication_review_in_repo,
    record_review, BootstrapRequest, CardKind, Claim, EditRequest, ErrorCode, InitialCardInput,
    LifecyclePhase, NonSubstantiveProof, PlanningProfile, ReviewAssignmentRequest, ReviewEvidence,
    ReviewFindingEvidence, ReviewRecordRequest, ReviewRecoveryRequest, SemanticOperation, Store,
};

fn install_native_authority(root: &std::path::Path) {
    let registry = root.join("docs/templates/prompts/current.json");
    let manifest = root.join("csdlc-v2/operator/native-card-shape.json");
    std::fs::create_dir_all(registry.parent().unwrap()).unwrap();
    std::fs::create_dir_all(manifest.parent().unwrap()).unwrap();
    std::fs::write(
        registry,
        include_bytes!("../../docs/templates/prompts/current.json"),
    )
    .unwrap();
    std::fs::write(
        manifest,
        include_bytes!("../operator/native-card-shape.json"),
    )
    .unwrap();
}

fn bootstrap_issue(
    store: &Store,
    request: BootstrapRequest,
) -> csdlc_v2::Result<csdlc_v2::IssueRecord> {
    csdlc_v2::initialize_native_json(store, &serde_json::to_vec(&request).unwrap())
}

fn finding(id: &str) -> ReviewFindingEvidence {
    ReviewFindingEvidence {
        id: id.into(),
        severity: FindingSeverity::P1,
        summary: "fix correctness".into(),
        actionable: true,
        in_scope: true,
        disposition: FindingDisposition::Fixed,
        fix_revision: Some("rev-2".into()),
        route: None,
    }
}

fn implemented_fixture() -> (tempfile::TempDir, Store, csdlc_v2::IssueRecord) {
    let temp = tempfile::tempdir().expect("temp");
    std::fs::create_dir_all(temp.path().join("docs")).expect("docs");
    std::fs::write(temp.path().join("docs/design.md"), "# reviewed design\n").expect("design");
    std::fs::write(
        temp.path().join("docs/diagram.mmd"),
        "flowchart LR\n A-->B\n",
    )
    .expect("diagram");
    install_native_authority(temp.path());
    git(temp.path(), &["init", "-b", "main"]);
    git(
        temp.path(),
        &["config", "user.email", "test@example.invalid"],
    );
    git(temp.path(), &["config", "user.name", "C-SDLC Test"]);
    git(temp.path(), &["add", "."]);
    git(temp.path(), &["commit", "-m", "fixture"]);
    let store = Store::new(temp.path());
    let mut record = bootstrap_issue(
        &store,
        BootstrapRequest {
            issue: 7,
            repository: "example/repo".into(),
            design_path: "docs/design.md".into(),
            diagram_path: "docs/diagram.mmd".into(),
            design_reviewer: "architect".into(),
            design_approved: true,
            claim: Claim {
                id: "claim".into(),
                owner: "agent".into(),
                generation: 0,
                acquired_unix_seconds: 1,
                expires_unix_seconds: u64::MAX,
                heartbeat_unix_seconds: 1,
                branch: "issue-7".into(),
                worktree: ".worktrees/issue-7".into(),
                protected_paths: vec!["src".into()],
                purpose: "review test".into(),
            },
            initial: InitialCardInput {
                title: "review fixture".into(),
                slug: "review-fixture".into(),
                version: "v0.91.7".into(),
                goal: "prove review".into(),
                required_outcome: "review truth".into(),
                declared_scope: vec!["src".into()],
                authority_boundary: vec!["no network".into()],
                operator_constraints: vec!["none".into()],
                task_boundary: "review only".into(),
                deliverables: vec!["review".into()],
                acceptance_criteria: vec!["review current".into()],
                dependencies: vec!["none".into()],
                repo_inputs: vec!["src".into()],
                non_goals: vec!["publish".into()],
                plan_summary: "implement then review".into(),
                steps: vec![csdlc_v2::cards::PlanStep {
                    id: "one".into(),
                    action: "review".into(),
                    acceptance_ids: vec!["AC-1".into()],
                    status: csdlc_v2::cards::StepStatus::Pending,
                }],
                invariants: vec!["exact revision".into()],
                risks: vec!["stale".into()],
                planning_profile: PlanningProfile::Small,
                stop_conditions: vec!["stale".into()],
                validation_lanes: vec![csdlc_v2::cards::ValidationLane {
                    lane: "focused".into(),
                    proof_role: "review".into(),
                    acceptance_ids: vec!["AC-1".into()],
                    deterministic: true,
                    resource_profile: csdlc_v2::cards::ResourceProfile::Small,
                    budget_seconds: 60,
                    budget_tokens: 100,
                    argv: vec!["cargo".into(), "test".into()],
                    parallel_group: "local".into(),
                    defer_reason: None,
                }],
                failure_policy: "fail closed".into(),
                review_prompts: vec!["review correctness".into()],
                review_scope: "fixture".into(),
            },
            prepared_cards: None,
        },
    )
    .expect("init");
    for operation in [
        SemanticOperation::AdvancePhase {
            phase: LifecyclePhase::Ready,
        },
        SemanticOperation::AdvancePhase {
            phase: LifecyclePhase::Bound,
        },
        SemanticOperation::RecordExecution {
            summary: "implemented".into(),
            changes: vec!["src".into()],
            artifacts: vec!["artifact".into()],
        },
        SemanticOperation::AdvancePhase {
            phase: LifecyclePhase::Implemented,
        },
    ] {
        let card = if matches!(operation, SemanticOperation::RecordExecution { .. }) {
            CardKind::Sor
        } else {
            CardKind::Sip
        };
        record = edit_issue(
            &store,
            EditRequest {
                issue: 7,
                card,
                expected_generation: record.generation,
                expected_digest: record.digest.clone(),
                claim_id: "claim".into(),
                actor: "agent".into(),
                reason: "fixture transition".into(),
                operation,
                fail_after_backup: false,
            },
        )
        .expect("transition");
    }
    (temp, store, record)
}

#[test]
fn substantive_revision_honors_review_scope_pathspecs() {
    let temp = tempfile::tempdir().expect("temp");
    std::fs::create_dir_all(temp.path().join("docs")).expect("docs");
    std::fs::create_dir_all(temp.path().join("src")).expect("src");
    std::fs::write(temp.path().join("docs/review.md"), "reviewed\n").expect("doc");
    std::fs::write(temp.path().join("src/outside.rs"), "outside\n").expect("src");
    git(temp.path(), &["init", "-b", "main"]);
    git(
        temp.path(),
        &["config", "user.email", "test@example.invalid"],
    );
    git(temp.path(), &["config", "user.name", "C-SDLC Test"]);
    git(temp.path(), &["add", "docs", "src"]);
    git(temp.path(), &["commit", "-m", "fixture"]);

    let clean = csdlc_v2::git::substantive_revision(temp.path(), &["docs".into()])
        .expect("clean scoped revision");
    let head = git_out(temp.path(), &["rev-parse", "HEAD"]);
    assert_eq!(clean, csdlc_v2::git::clean_commit_revision(&head));

    std::fs::write(temp.path().join("src/outside.rs"), "outside dirty\n").expect("dirty src");
    std::fs::write(temp.path().join("src/untracked.rs"), "new outside\n").expect("outside new");
    let outside_dirty = csdlc_v2::git::substantive_revision(temp.path(), &["docs".into()])
        .expect("outside dirty scoped revision");
    assert_eq!(outside_dirty, clean);

    std::fs::write(temp.path().join("docs/new.md"), "new reviewed file\n").expect("new doc");
    let inside_untracked = csdlc_v2::git::substantive_revision(temp.path(), &["docs".into()])
        .expect("inside untracked scoped revision");
    assert_ne!(inside_untracked, clean);

    std::fs::write(temp.path().join("docs/review.md"), "reviewed dirty\n").expect("dirty doc");
    let inside_dirty = csdlc_v2::git::substantive_revision(temp.path(), &["docs".into()])
        .expect("inside dirty scoped revision");
    assert_ne!(inside_dirty, clean);
}

#[test]
fn assignment_and_recording_update_index_and_srp_without_publication_side_effect() {
    let (temp, store, record) = implemented_fixture();
    let assigned = assign_review(
        &store,
        ReviewAssignmentRequest {
            issue: 7,
            expected_generation: record.generation,
            expected_digest: record.digest,
            claim_id: "claim".into(),
            reviewer: "subagent".into(),
            assigned_by: "agent".into(),
            scope: vec!["src".into()],
        },
    )
    .expect("assign");
    let cards = store.load_cards(7).expect("assigned cards");
    let csdlc_v2::cards::CardContent::Srp(srp) = &cards[&CardKind::Srp].content else {
        panic!("SRP");
    };
    assert_eq!(srp.review_scope, "src");
    assert!(assigned.review.is_none());
    let revision = assigned
        .review_assignment
        .as_ref()
        .expect("assignment")
        .revision
        .clone();
    let mut fixed = finding("F-1");
    fixed.fix_revision = Some(revision.clone());
    let value = ReviewEvidence {
        reviewer: "subagent".into(),
        scope: vec!["src".into()],
        reviewed_revision: revision.clone(),
        findings: vec![fixed],
        residual_risks: vec!["none".into()],
        completed: true,
        non_substantive_proof: None,
    };
    let reviewed = record_review(
        &store,
        ReviewRecordRequest {
            issue: 7,
            expected_generation: assigned.generation,
            expected_digest: assigned.digest,
            claim_id: "claim".into(),
            actor: "agent".into(),
            evidence: value,
        },
    )
    .expect("record");
    assert!(evaluate_publication_review(reviewed.review.as_ref(), &revision).ready);
    let cards = store.load_cards(7).expect("cards");
    match &cards[&CardKind::Srp].content {
        csdlc_v2::cards::CardContent::Srp(srp) => {
            assert_eq!(srp.reviewer.as_deref(), Some("subagent"));
            assert_eq!(srp.findings.len(), 1);
        }
        _ => unreachable!(),
    };
    assert_eq!(git_out(temp.path(), &["branch", "--show-current"]), "main");
    assert!(
        !temp.path().join(".git/refs/remotes").exists(),
        "review created remote state"
    );
    assert_eq!(reviewed.phase, LifecyclePhase::Reviewed);
}

#[test]
fn direct_exact_review_records_and_advances_without_assignment() {
    let (_temp, store, record) = implemented_fixture();
    assert!(record.review_assignment.is_none());
    let revision = csdlc_v2::git::substantive_revision(store.root(), &["src".into()])
        .expect("exact scoped revision");
    let before = std::fs::read(store.issue_dir(7).join("index.json")).expect("before");
    let mut stale = ReviewRecordRequest {
        issue: 7,
        expected_generation: record.generation,
        expected_digest: record.digest.clone(),
        claim_id: "claim".into(),
        actor: "reviewer".into(),
        evidence: ReviewEvidence {
            reviewer: "reviewer".into(),
            scope: vec!["src".into()],
            reviewed_revision: "git-blake3:stale:stale".into(),
            findings: vec![],
            residual_risks: vec![],
            completed: true,
            non_substantive_proof: None,
        },
    };
    assert_eq!(
        record_review(&store, stale.clone()).unwrap_err().code,
        ErrorCode::UnsafeCheckout
    );
    assert_eq!(
        std::fs::read(store.issue_dir(7).join("index.json")).expect("unchanged"),
        before
    );
    stale.evidence.reviewed_revision = revision;
    let reviewed = record_review(&store, stale).expect("direct exact review");
    assert_eq!(reviewed.phase, LifecyclePhase::Reviewed);
    assert!(reviewed.review_assignment.is_none());
    assert_eq!(
        reviewed.audit.last().expect("audit").operation,
        "record_review"
    );
}

#[test]
fn dirty_substantive_tree_is_rejected_before_review_assignment() {
    let (temp, store, record) = implemented_fixture();
    std::fs::write(temp.path().join("docs/design.md"), "# changed design\n").expect("dirty");
    let error = assign_review(
        &store,
        ReviewAssignmentRequest {
            issue: 7,
            expected_generation: record.generation,
            expected_digest: record.digest,
            claim_id: "claim".into(),
            reviewer: "subagent".into(),
            assigned_by: "agent".into(),
            scope: vec!["docs".into()],
        },
    )
    .expect_err("dirty review assignment must fail closed");
    assert!(matches!(error.code, ErrorCode::UnsafeCheckout));
}

#[test]
fn metadata_only_changes_do_not_stale_a_clean_review() {
    let (temp, store, record) = implemented_fixture();
    let assigned = assign_review(
        &store,
        ReviewAssignmentRequest {
            issue: 7,
            expected_generation: record.generation,
            expected_digest: record.digest,
            claim_id: "claim".into(),
            reviewer: "subagent".into(),
            assigned_by: "agent".into(),
            scope: vec!["docs".into()],
        },
    )
    .expect("clean assignment");
    let revision = assigned
        .review_assignment
        .as_ref()
        .expect("assignment")
        .revision
        .clone();
    let reviewed = record_review(
        &store,
        ReviewRecordRequest {
            issue: 7,
            expected_generation: assigned.generation,
            expected_digest: assigned.digest,
            claim_id: "claim".into(),
            actor: "agent".into(),
            evidence: ReviewEvidence {
                reviewer: "subagent".into(),
                scope: vec!["docs".into()],
                reviewed_revision: revision.clone(),
                findings: vec![],
                residual_risks: vec![],
                completed: true,
                non_substantive_proof: None,
            },
        },
    )
    .expect("record review");
    std::fs::create_dir_all(temp.path().join(".csdlc/review")).expect("metadata dir");
    std::fs::write(temp.path().join(".csdlc/review/observation.json"), "{}\n").expect("metadata");
    let current = csdlc_v2::git::substantive_revision(temp.path(), &["docs".into()])
        .expect("current revision");
    assert_eq!(current, revision);
    assert!(
        evaluate_publication_review_in_repo(temp.path(), reviewed.review.as_ref(), &current).ready
    );
    let report = csdlc_v2::diagnose(&store, 7);
    assert!(!report
        .findings
        .iter()
        .any(|finding| finding.code == "review_publication_dead_end"));
}

#[test]
fn reviewed_dirty_state_is_diagnosed_and_recoverable_for_clean_rereview() {
    let (temp, store, implemented) = implemented_fixture();
    let before = std::fs::read(store.issue_dir(7).join("index.json")).unwrap();
    let premature = edit_issue(
        &store,
        EditRequest {
            issue: 7,
            card: CardKind::Srp,
            expected_generation: implemented.generation,
            expected_digest: implemented.digest.clone(),
            claim_id: "claim".into(),
            actor: "operator".into(),
            reason: "not actually recovered".into(),
            operation: SemanticOperation::CorrectReviewPromptsAfterRecovery {
                values: vec!["truthful prompt".into()],
            },
            fail_after_backup: false,
        },
    )
    .unwrap_err();
    assert_eq!(premature.code, ErrorCode::InvalidTransition);
    assert_eq!(
        std::fs::read(store.issue_dir(7).join("index.json")).unwrap(),
        before
    );
    let assigned = assign_review(
        &store,
        ReviewAssignmentRequest {
            issue: 7,
            expected_generation: implemented.generation,
            expected_digest: implemented.digest,
            claim_id: "claim".into(),
            reviewer: "subagent".into(),
            assigned_by: "agent".into(),
            scope: vec!["docs".into()],
        },
    )
    .expect("assign clean review");
    let revision = assigned
        .review_assignment
        .as_ref()
        .expect("assignment")
        .revision
        .clone();
    let reviewed = record_review(
        &store,
        ReviewRecordRequest {
            issue: 7,
            expected_generation: assigned.generation,
            expected_digest: assigned.digest,
            claim_id: "claim".into(),
            actor: "agent".into(),
            evidence: ReviewEvidence {
                reviewer: "subagent".into(),
                scope: vec!["docs".into()],
                reviewed_revision: revision,
                findings: vec![],
                residual_risks: vec![],
                completed: true,
                non_substantive_proof: None,
            },
        },
    )
    .expect("record review");
    std::fs::write(temp.path().join("docs/new-proof.md"), "proof\n").expect("dirty change");
    let report = csdlc_v2::diagnose(&store, 7);
    assert!(matches!(
        report.status,
        csdlc_v2::doctor::DoctorStatus::Block
    ));
    assert!(report
        .findings
        .iter()
        .any(|finding| finding.code == "review_publication_dead_end"));
    assert_eq!(report.next_operation.as_deref(), Some("recover_review"));

    let recovered = csdlc_v2::recover_review(
        &store,
        ReviewRecoveryRequest {
            issue: 7,
            expected_generation: reviewed.generation,
            expected_digest: reviewed.digest,
            claim_id: "claim".into(),
            actor: "operator".into(),
            reason: "re-review after finalizing substantive changes".into(),
        },
    )
    .expect("recover reviewed state");
    assert_eq!(recovered.phase, LifecyclePhase::Implemented);
    assert!(recovered.review.is_none());
    assert!(recovered.review_assignment.is_none());
    assert!(recovered
        .audit
        .iter()
        .any(|event| event.operation == "recover_review"));

    let corrected = edit_issue(
        &store,
        EditRequest {
            issue: 7,
            card: CardKind::Srp,
            expected_generation: recovered.generation,
            expected_digest: recovered.digest,
            claim_id: "claim".into(),
            actor: "operator".into(),
            reason: "correct stale review question after recovery".into(),
            operation: SemanticOperation::CorrectReviewPromptsAfterRecovery {
                values: vec!["Does the final hosted mode match current truth?".into()],
            },
            fail_after_backup: false,
        },
    )
    .expect("correct prompts after recovery");
    let cards = store.load_cards(7).unwrap();
    let csdlc_v2::cards::CardContent::Srp(srp) = &cards[&CardKind::Srp].content else {
        panic!("SRP")
    };
    assert_eq!(
        srp.review_prompts,
        vec!["Does the final hosted mode match current truth?"]
    );

    git(temp.path(), &["add", "docs/new-proof.md"]);
    git(temp.path(), &["commit", "-m", "finalize reviewed changes"]);
    let reassigned = assign_review(
        &store,
        ReviewAssignmentRequest {
            issue: 7,
            expected_generation: corrected.generation,
            expected_digest: corrected.digest,
            claim_id: "claim".into(),
            reviewer: "reviewer".into(),
            assigned_by: "operator".into(),
            scope: vec!["docs".into()],
        },
    )
    .expect("reassign after clean finalize");
    assert!(reassigned.review_assignment.is_some());
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
fn evidence() -> ReviewEvidence {
    ReviewEvidence {
        reviewer: "bounded-subagent".into(),
        scope: vec!["csdlc-v2/".into()],
        reviewed_revision: "rev-2".into(),
        findings: vec![finding("F-1")],
        residual_risks: vec!["none known".into()],
        completed: true,
        non_substantive_proof: None,
    }
}

#[test]
fn exact_completed_review_with_resolved_findings_is_publishable() {
    let report = evaluate_publication_review(Some(&evidence()), "rev-2");
    assert!(report.ready);
    assert!(report.blocker_codes.is_empty());
}

#[test]
fn missing_incomplete_stale_and_unresolved_review_fail_closed() {
    assert_eq!(
        evaluate_publication_review(None, "rev").blocker_codes,
        vec!["review_missing"]
    );
    let mut value = evidence();
    value.completed = false;
    assert!(evaluate_publication_review(Some(&value), "rev-2")
        .blocker_codes
        .contains(&"review_incomplete".into()));
    value.completed = true;
    assert!(evaluate_publication_review(Some(&value), "rev-3")
        .blocker_codes
        .contains(&"review_stale".into()));
    value.findings[0].disposition = FindingDisposition::Open;
    assert!(evaluate_publication_review(Some(&value), "rev-2")
        .blocker_codes
        .contains(&"actionable_finding_unresolved".into()));
}

#[test]
fn guard_rejects_malformed_fixed_and_accepted_risk_evidence() {
    let mut value = evidence();
    value.findings[0].fix_revision = Some("wrong".into());
    assert!(evaluate_publication_review(Some(&value), "rev-2")
        .blocker_codes
        .contains(&"review_evidence_invalid".into()));
    value.findings[0].disposition = FindingDisposition::AcceptedRisk;
    value.findings[0].fix_revision = None;
    value.residual_risks.clear();
    assert!(evaluate_publication_review(Some(&value), "rev-2")
        .blocker_codes
        .contains(&"review_evidence_invalid".into()));
}

#[test]
fn out_of_scope_finding_must_remain_visible_and_routed() {
    let mut value = evidence();
    value.findings[0].in_scope = false;
    value.findings[0].disposition = FindingDisposition::OutOfScope;
    value.findings[0].fix_revision = None;
    value.findings[0].route = None;
    assert!(evaluate_publication_review(Some(&value), "rev-2")
        .blocker_codes
        .contains(&"out_of_scope_finding_unrouted".into()));
    value.findings[0].route = Some("follow-up:#999".into());
    assert!(evaluate_publication_review(Some(&value), "rev-2").ready);
}

#[test]
fn non_substantive_exception_is_narrow_and_machine_proven() {
    let temp = tempfile::tempdir().expect("temp");
    std::fs::create_dir_all(temp.path().join(".csdlc/review")).expect("dir");
    std::fs::write(temp.path().join(".csdlc/review/result.json"), "one").expect("one");
    git(temp.path(), &["init", "-b", "main"]);
    git(
        temp.path(),
        &["config", "user.email", "test@example.invalid"],
    );
    git(temp.path(), &["config", "user.name", "C-SDLC Test"]);
    git(temp.path(), &["add", "."]);
    git(temp.path(), &["commit", "-m", "one"]);
    let from = git_out(temp.path(), &["rev-parse", "HEAD"]);
    std::fs::write(temp.path().join(".csdlc/review/result.json"), "two").expect("two");
    git(temp.path(), &["add", "."]);
    git(temp.path(), &["commit", "-m", "two"]);
    let to = git_out(temp.path(), &["rev-parse", "HEAD"]);
    let from_revision = csdlc_v2::git::clean_commit_revision(&from);
    let to_revision = csdlc_v2::git::clean_commit_revision(&to);
    let mut value = evidence();
    value.reviewed_revision = from_revision.clone();
    value.findings[0].fix_revision = Some(from_revision.clone());
    value.non_substantive_proof = Some(NonSubstantiveProof {
        policy: "review_metadata_only_v1".into(),
        from_revision,
        to_revision: to_revision.clone(),
        from_commit: from,
        to_commit: to,
        changed_paths: vec![".csdlc/review/result.json".into()],
    });
    assert!(evaluate_publication_review_in_repo(temp.path(), Some(&value), &to_revision).ready);
    value
        .non_substantive_proof
        .as_mut()
        .expect("proof")
        .changed_paths = vec!["src/lib.rs".into()];
    assert!(!evaluate_publication_review_in_repo(temp.path(), Some(&value), &to_revision).ready);
}

#[test]
fn typed_publication_metadata_commit_does_not_stale_review_but_source_drift_does() {
    let temp = tempfile::tempdir().expect("temp");
    std::fs::create_dir_all(temp.path().join("docs")).expect("docs");
    std::fs::create_dir_all(temp.path().join(".csdlc/issues/7/cards")).expect("cards");
    std::fs::create_dir_all(temp.path().join(".csdlc/prepared/issues/7")).expect("prepared");
    std::fs::create_dir_all(temp.path().join(".csdlc/requests")).expect("requests");
    std::fs::create_dir_all(temp.path().join(".csdlc/publication")).expect("publication");
    std::fs::write(temp.path().join("docs/design.md"), "reviewed\n").expect("design");
    git(temp.path(), &["init", "-b", "main"]);
    git(
        temp.path(),
        &["config", "user.email", "test@example.invalid"],
    );
    git(temp.path(), &["config", "user.name", "C-SDLC Test"]);
    git(temp.path(), &["add", "."]);
    git(temp.path(), &["commit", "-m", "reviewed source"]);
    let from = git_out(temp.path(), &["rev-parse", "HEAD"]);
    let from_revision = csdlc_v2::git::clean_commit_revision(&from);
    let evidence = ReviewEvidence {
        reviewer: "subagent".into(),
        scope: vec!["docs".into()],
        reviewed_revision: from_revision,
        findings: vec![],
        residual_risks: vec![],
        completed: true,
        non_substantive_proof: None,
    };
    for (path, body) in [
        (".csdlc/issues/7/index.json", "{}\n"),
        (".csdlc/issues/7/audit.jsonl", "{}\n"),
        (".csdlc/issues/7/cards/sor.md", "card\n"),
        (".csdlc/issues/7/cards/sor.values.json", "{}\n"),
        (".csdlc/prepared/issues/7/publication.json", "{}\n"),
        (".csdlc/publication/7.intent.json", "{}\n"),
    ] {
        let target = temp.path().join(path);
        std::fs::write(target, body).expect("metadata");
    }
    git(temp.path(), &["add", "."]);
    git(temp.path(), &["commit", "-m", "typed publication metadata"]);
    let to = git_out(temp.path(), &["rev-parse", "HEAD"]);
    let current = csdlc_v2::git::clean_commit_revision(&to);
    assert!(evaluate_publication_review_in_repo(temp.path(), Some(&evidence), &current).ready);

    std::fs::write(temp.path().join(".csdlc/requests/7-publish.json"), "{}\n")
        .expect("obsolete tracked request");
    git(temp.path(), &["add", ".csdlc/requests/7-publish.json"]);
    git(
        temp.path(),
        &["commit", "-m", "obsolete tracked request drift"],
    );
    let request_drift = git_out(temp.path(), &["rev-parse", "HEAD"]);
    let request_drift_revision = csdlc_v2::git::clean_commit_revision(&request_drift);
    let request_report =
        evaluate_publication_review_in_repo(temp.path(), Some(&evidence), &request_drift_revision);
    assert!(request_report
        .blocker_codes
        .contains(&"review_stale".into()));

    std::fs::write(
        temp.path().join(".csdlc/issues/7/cards/sor.md"),
        "hand-edited substantive card\n",
    )
    .expect("card drift");
    git(temp.path(), &["add", ".csdlc/issues/7/cards/sor.md"]);
    git(temp.path(), &["commit", "-m", "substantive card drift"]);
    let card_drift = git_out(temp.path(), &["rev-parse", "HEAD"]);
    let card_drift_revision = csdlc_v2::git::clean_commit_revision(&card_drift);
    let card_report =
        evaluate_publication_review_in_repo(temp.path(), Some(&evidence), &card_drift_revision);
    assert!(card_report.blocker_codes.contains(&"review_stale".into()));

    std::fs::write(temp.path().join("docs/new-source.md"), "substantive\n").expect("source");
    git(temp.path(), &["add", "docs/new-source.md"]);
    git(temp.path(), &["commit", "-m", "substantive drift"]);
    let drift = git_out(temp.path(), &["rev-parse", "HEAD"]);
    let drift_revision = csdlc_v2::git::clean_commit_revision(&drift);
    let report = evaluate_publication_review_in_repo(temp.path(), Some(&evidence), &drift_revision);
    assert!(report.blocker_codes.contains(&"review_stale".into()));
}

#[test]
fn doctor_accepts_committed_typed_metadata_after_review() {
    let (temp, store, record) = implemented_fixture();
    let assigned = assign_review(
        &store,
        ReviewAssignmentRequest {
            issue: 7,
            expected_generation: record.generation,
            expected_digest: record.digest,
            claim_id: "claim".into(),
            reviewer: "subagent".into(),
            assigned_by: "agent".into(),
            scope: vec!["docs".into()],
        },
    )
    .expect("assignment");
    let revision = assigned
        .review_assignment
        .as_ref()
        .unwrap()
        .revision
        .clone();
    let reviewed = record_review(
        &store,
        ReviewRecordRequest {
            issue: 7,
            expected_generation: assigned.generation,
            expected_digest: assigned.digest,
            claim_id: "claim".into(),
            actor: "subagent".into(),
            evidence: ReviewEvidence {
                reviewer: "subagent".into(),
                scope: vec!["docs".into()],
                reviewed_revision: revision,
                findings: vec![],
                residual_risks: vec![],
                completed: true,
                non_substantive_proof: None,
            },
        },
    )
    .expect("review");
    assert_eq!(reviewed.phase, LifecyclePhase::Reviewed);
    std::fs::create_dir_all(temp.path().join(".csdlc/publication")).expect("publication");
    std::fs::write(temp.path().join(".csdlc/publication/7.intent.json"), "{}\n").expect("intent");
    git(temp.path(), &["add", ".csdlc/publication/7.intent.json"]);
    git(temp.path(), &["commit", "-m", "typed publication metadata"]);
    let report = csdlc_v2::diagnose(&store, 7);
    assert!(!report
        .findings
        .iter()
        .any(|finding| finding.code == "review_publication_dead_end"));
}

#[test]
fn guard_cli_is_read_only_and_returns_typed_truth() {
    let temp = tempfile::tempdir().expect("temp");
    std::fs::create_dir_all(temp.path().join("docs")).expect("docs");
    std::fs::write(temp.path().join("docs/review.md"), "review").expect("doc");
    git(temp.path(), &["init", "-b", "main"]);
    git(
        temp.path(),
        &["config", "user.email", "test@example.invalid"],
    );
    git(temp.path(), &["config", "user.name", "C-SDLC Test"]);
    git(temp.path(), &["add", "docs"]);
    git(temp.path(), &["commit", "-m", "review"]);
    let revision =
        csdlc_v2::git::substantive_revision(temp.path(), &["docs".into()]).expect("revision");
    let mut reviewed = evidence();
    reviewed.reviewed_revision = revision.clone();
    reviewed.findings[0].fix_revision = Some(revision);
    let request_dir = tempfile::tempdir().expect("request dir");
    let path = request_dir.path().join("guard.json");
    std::fs::write(
        &path,
        serde_json::json!({"evidence":reviewed,"scope":["docs"]}).to_string(),
    )
    .expect("request");
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_csdlc-review"))
        .args([
            "--root",
            temp.path().to_str().expect("root"),
            "guard",
            "--request",
            path.to_str().expect("request"),
        ])
        .output()
        .expect("CLI");
    assert!(output.status.success());
    let report: String = String::from_utf8(output.stdout).expect("UTF-8");
    assert!(report.contains("\"ready\":true"));
    assert!(
        !temp.path().join(".csdlc").exists(),
        "guard mutated repository"
    );
}
fn git_out(root: &std::path::Path, args: &[&str]) -> String {
    let output = std::process::Command::new("git")
        .current_dir(root)
        .args(args)
        .output()
        .expect("git");
    assert!(output.status.success());
    String::from_utf8(output.stdout)
        .expect("UTF-8")
        .trim()
        .into()
}
