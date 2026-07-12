use csdlc_v2::cards::{
    EvidenceOutcome, PlanStep, ResourceProfile, StepStatus, ValidationLane, ValidationResult,
};
use csdlc_v2::{
    assign_review, closeout_issue, edit_issue, initialize_issue, record_publication,
    record_readiness, record_review, BootstrapRequest, CardKind, Claim, EditRequest,
    InitialCardInput, LifecyclePhase, PlanningProfile, PublicationIntent, PublicationRequest,
    ReadinessRequest, RemotePullRequest, ReviewAssignmentRequest, ReviewEvidence,
    ReviewRecordRequest, SemanticOperation, Store, TerminalDisposition, TerminalObservation,
};

fn git(root: &std::path::Path, args: &[&str]) {
    let output = std::process::Command::new("git")
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

fn edit(
    store: &Store,
    record: &csdlc_v2::IssueRecord,
    card: CardKind,
    operation: SemanticOperation,
) -> csdlc_v2::IssueRecord {
    edit_issue(
        store,
        EditRequest {
            issue: 7,
            card,
            expected_generation: record.generation,
            expected_digest: record.digest.clone(),
            claim_id: "claim".into(),
            actor: "agent".into(),
            reason: "fixture".into(),
            operation,
            fail_after_backup: false,
        },
    )
    .unwrap()
}

fn fixture() -> (tempfile::TempDir, Store, csdlc_v2::IssueRecord, String) {
    let temp = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(temp.path().join("docs")).unwrap();
    std::fs::write(temp.path().join("docs/design.md"), "# design\n").unwrap();
    std::fs::write(
        temp.path().join("docs/diagram.mmd"),
        "flowchart LR\n A-->B\n",
    )
    .unwrap();
    git(temp.path(), &["init", "-b", "issue-7"]);
    git(
        temp.path(),
        &["config", "user.email", "test@example.invalid"],
    );
    git(temp.path(), &["config", "user.name", "C-SDLC Test"]);
    git(temp.path(), &["add", "docs"]);
    git(temp.path(), &["commit", "-m", "fixture"]);
    let sha = csdlc_v2::git::run(temp.path(), &["rev-parse", "HEAD"])
        .unwrap()
        .stdout;
    let store = Store::new(temp.path());
    let mut record = initialize_issue(
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
                worktree: temp.path().to_string_lossy().into_owned(),
                protected_paths: vec!["src".into()],
                purpose: "gate7 fixture".into(),
            },
            initial: InitialCardInput {
                title: "Gate 7 fixture".into(),
                slug: "gate-7-fixture".into(),
                version: "v0.91.7".into(),
                goal: "prove terminal lifecycle".into(),
                required_outcome: "truthful closeout".into(),
                declared_scope: vec!["docs".into()],
                authority_boundary: vec!["no merge".into()],
                task_boundary: "fixture".into(),
                deliverables: vec!["record".into()],
                acceptance_criteria: vec!["terminal truth".into()],
                dependencies: vec!["none".into()],
                repo_inputs: vec!["docs".into()],
                non_goals: vec!["network".into()],
                plan_summary: "advance lifecycle".into(),
                steps: vec![PlanStep {
                    id: "one".into(),
                    action: "advance".into(),
                    acceptance_ids: vec!["AC-1".into()],
                    status: StepStatus::Pending,
                }],
                invariants: vec!["exact SHA".into()],
                risks: vec!["stale remote".into()],
                planning_profile: PlanningProfile::Small,
                stop_conditions: vec!["mismatch".into()],
                validation_lanes: vec![ValidationLane {
                    lane: "focused".into(),
                    proof_role: "lifecycle".into(),
                    acceptance_ids: vec!["AC-1".into()],
                    deterministic: true,
                    resource_profile: ResourceProfile::Small,
                    budget_seconds: 30,
                    budget_tokens: 100,
                    argv: vec!["cargo".into(), "test".into()],
                    parallel_group: "local".into(),
                    defer_reason: None,
                }],
                failure_policy: "fail closed".into(),
                review_prompts: vec!["review".into()],
            },
        },
    )
    .unwrap();
    record = edit(
        &store,
        &record,
        CardKind::Sip,
        SemanticOperation::AdvancePhase {
            phase: LifecyclePhase::Ready,
        },
    );
    record = edit(
        &store,
        &record,
        CardKind::Sip,
        SemanticOperation::AdvancePhase {
            phase: LifecyclePhase::Bound,
        },
    );
    record = edit(
        &store,
        &record,
        CardKind::Sor,
        SemanticOperation::RecordExecution {
            summary: "done".into(),
            changes: vec!["docs".into()],
            artifacts: vec!["artifact".into()],
        },
    );
    record = edit(
        &store,
        &record,
        CardKind::Sor,
        SemanticOperation::RecordValidation {
            result: ValidationResult {
                command: vec!["cargo".into(), "test".into()],
                purpose: "proof".into(),
                outcome: EvidenceOutcome::Passed,
                evidence_ref: "evidence.json".into(),
            },
        },
    );
    record = edit(
        &store,
        &record,
        CardKind::Sip,
        SemanticOperation::AdvancePhase {
            phase: LifecyclePhase::Implemented,
        },
    );
    let assigned = assign_review(
        &store,
        ReviewAssignmentRequest {
            issue: 7,
            expected_generation: record.generation,
            expected_digest: record.digest,
            claim_id: "claim".into(),
            reviewer: "reviewer".into(),
            assigned_by: "agent".into(),
            scope: vec!["docs".into()],
        },
    )
    .unwrap();
    let revision = assigned
        .review_assignment
        .as_ref()
        .unwrap()
        .revision
        .clone();
    record = record_review(
        &store,
        ReviewRecordRequest {
            issue: 7,
            expected_generation: assigned.generation,
            expected_digest: assigned.digest,
            claim_id: "claim".into(),
            actor: "reviewer".into(),
            evidence: ReviewEvidence {
                reviewer: "reviewer".into(),
                scope: vec!["docs".into()],
                reviewed_revision: revision.clone(),
                findings: vec![],
                residual_risks: vec![],
                completed: true,
                non_substantive_proof: None,
            },
        },
    )
    .unwrap();
    record = edit(
        &store,
        &record,
        CardKind::Sip,
        SemanticOperation::AdvancePhase {
            phase: LifecyclePhase::Reviewed,
        },
    );
    let request = PublicationRequest {
        schema: "csdlc.publication_request.v1".into(),
        issue: 7,
        expected_generation: record.generation,
        expected_digest: record.digest.clone(),
        claim_id: "claim".into(),
        actor: "publisher".into(),
        repository: "example/repo".into(),
        base: "main".into(),
        head: "issue-7".into(),
        title: "Fixture".into(),
        body: "Closes #7".into(),
        draft: true,
        remote: "origin".into(),
        token_file: None,
    };
    let intent = PublicationIntent {
        schema: "csdlc.publication_intent.v1".into(),
        issue: 7,
        repository: "example/repo".into(),
        base: "main".into(),
        head: "issue-7".into(),
        title: "Fixture".into(),
        body: "Closes #7".into(),
        draft: true,
        revision: revision.clone(),
        commit_sha: sha.clone(),
    };
    record = record_publication(
        &store,
        &request,
        &intent,
        RemotePullRequest {
            number: 70,
            url: "https://example.invalid/70".into(),
            repository: "example/repo".into(),
            base: "main".into(),
            head: "issue-7".into(),
            title: "Fixture".into(),
            body: "Closes #7".into(),
            draft: true,
            state: "open".into(),
            head_sha: sha.clone(),
        },
    )
    .unwrap();
    (temp, store, record, sha)
}

#[test]
fn readiness_regression_and_exact_terminal_closeout_are_atomic_and_idempotent() {
    let (_temp, store, mut record, sha) = fixture();
    let mut request = ReadinessRequest {
        schema: "csdlc.readiness_request.v1".into(),
        issue: 7,
        expected_generation: record.generation,
        expected_digest: record.digest.clone(),
        claim_id: "claim".into(),
        actor: "shepherd".into(),
        pull_request: 70,
        head_sha: sha.clone(),
        required_checks: vec!["fast".into()],
        require_review: true,
        checks: vec![csdlc_v2::CheckObservation {
            name: "fast".into(),
            requirement: csdlc_v2::CheckRequirement::Required,
            conclusion: csdlc_v2::CheckConclusion::Success,
            details_url: None,
        }],
        review_state: csdlc_v2::RemoteReviewState::Approved,
        conflict_state: csdlc_v2::ConflictState::Clean,
        post_publication_findings: vec![],
    };
    record = record_readiness(&store, request.clone()).unwrap();
    assert_eq!(record.phase, LifecyclePhase::MergeReady);
    request.expected_generation = record.generation;
    request.expected_digest = record.digest.clone();
    request.checks[0].conclusion = csdlc_v2::CheckConclusion::Failure;
    record = record_readiness(&store, request).unwrap();
    assert_eq!(record.phase, LifecyclePhase::Published);

    let wrong = TerminalObservation {
        schema: "csdlc.terminal_observation.v1".into(),
        issue: 7,
        expected_generation: record.generation,
        expected_digest: record.digest.clone(),
        claim_id: "claim".into(),
        actor: "closer".into(),
        pull_request: Some(70),
        disposition: TerminalDisposition::Merged,
        observed_sha: Some("wrong".into()),
        observed_state: "merged".into(),
        approved_no_pr_reason: None,
        receipt_path: "/tmp/gate7-receipt.json".into(),
    };
    assert!(closeout_issue(&store, wrong).is_err());
    let current = store.load_record(7).unwrap();
    assert_eq!(current.phase, LifecyclePhase::Published);

    let mut green = ReadinessRequest {
        schema: "csdlc.readiness_request.v1".into(),
        issue: 7,
        expected_generation: current.generation,
        expected_digest: current.digest.clone(),
        claim_id: "claim".into(),
        actor: "shepherd".into(),
        pull_request: 70,
        head_sha: sha.clone(),
        required_checks: vec!["fast".into()],
        require_review: true,
        checks: vec![csdlc_v2::CheckObservation {
            name: "fast".into(),
            requirement: csdlc_v2::CheckRequirement::Required,
            conclusion: csdlc_v2::CheckConclusion::Success,
            details_url: None,
        }],
        review_state: csdlc_v2::RemoteReviewState::Approved,
        conflict_state: csdlc_v2::ConflictState::Clean,
        post_publication_findings: vec![],
    };
    record = record_readiness(&store, green.clone()).unwrap();
    green.expected_generation = record.generation;
    green.expected_digest = record.digest.clone();
    let terminal = TerminalObservation {
        schema: "csdlc.terminal_observation.v1".into(),
        issue: 7,
        expected_generation: record.generation,
        expected_digest: record.digest.clone(),
        claim_id: "claim".into(),
        actor: "closer".into(),
        pull_request: Some(70),
        disposition: TerminalDisposition::Merged,
        observed_sha: Some(sha),
        observed_state: "merged".into(),
        approved_no_pr_reason: None,
        receipt_path: "/tmp/gate7-receipt.json".into(),
    };
    let closed = closeout_issue(&store, terminal.clone()).unwrap();
    assert_eq!(closed.phase, LifecyclePhase::ClosedOut);
    assert!(closed.claim.is_none());
    assert_eq!(closeout_issue(&store, terminal).unwrap(), closed);
    let doctor = csdlc_v2::diagnose(&store, 7);
    assert_eq!(doctor.phase, Some(LifecyclePhase::ClosedOut));
    assert!(doctor.findings.is_empty());
}
