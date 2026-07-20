use csdlc_v2::cards::{
    digest, CardContent, EvidenceOutcome, IntegrationState, MergeState, PlanStep, ResourceProfile,
    StepStatus, ValidationLane, ValidationResult,
};
use csdlc_v2::{
    assign_review, closeout_issue, edit_issue, initialize_issue, record_merged_publication,
    record_publication, record_readiness, record_review, BootstrapRequest, CardKind, Claim,
    EditRequest, InitialCardInput, LifecyclePhase, PlanningProfile, PublicationIntent,
    PublicationRequest, ReadinessRequest, ReconcileTerminalRequest, RemotePullRequest,
    ReviewAssignmentRequest, ReviewEvidence, ReviewRecordRequest, SemanticOperation, Store,
    TerminalDesignRepairRequest, TerminalDisposition, TerminalObservation,
    TerminalPlanStepRepairRequest, TerminalSorArtifactRepairRequest,
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
    let reopened = Store::new(store.root());
    let edited = edit_issue(
        &reopened,
        EditRequest {
            issue: record.issue,
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
    .unwrap();
    Store::new(store.root())
        .load_record(record.issue)
        .inspect(|record| assert_eq!(record.digest, edited.digest))
        .unwrap()
}

fn fixture_with_validation_history(
    issue: u64,
    title: &str,
    scenario: &str,
    validation_history: Vec<ValidationResult>,
) -> (tempfile::TempDir, Store, csdlc_v2::IssueRecord, String) {
    fixture_with_validation_history_and_publication(
        issue,
        title,
        scenario,
        validation_history,
        true,
    )
}

fn fixture_with_validation_history_and_publication(
    issue: u64,
    title: &str,
    scenario: &str,
    validation_history: Vec<ValidationResult>,
    publish: bool,
) -> (tempfile::TempDir, Store, csdlc_v2::IssueRecord, String) {
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
            issue,
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
                title: title.into(),
                slug: scenario.into(),
                version: "v0.91.7".into(),
                goal: format!("prove {scenario} terminal lifecycle"),
                required_outcome: "truthful closeout".into(),
                declared_scope: vec![scenario.into()],
                authority_boundary: vec!["no merge".into()],
                task_boundary: format!("execute {scenario} fixture"),
                deliverables: vec!["record".into()],
                acceptance_criteria: vec!["terminal truth".into()],
                dependencies: vec!["none".into()],
                repo_inputs: vec![scenario.into()],
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
                    proof_role: scenario.into(),
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
    for result in validation_history {
        record = edit(
            &store,
            &record,
            CardKind::Sor,
            SemanticOperation::RecordValidation { result },
        );
    }
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
            issue,
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
            issue,
            expected_generation: assigned.generation,
            expected_digest: assigned.digest,
            claim_id: "claim".into(),
            actor: "reviewer".into(),
            evidence: ReviewEvidence {
                reviewer: "reviewer".into(),
                scope: vec!["docs".into()],
                reviewed_revision: revision.clone(),
                findings: vec![],
                residual_risks: vec!["#5411 follow-up".into()],
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
    if !publish {
        return (temp, store, record, sha);
    }
    let publication_body = format!("Closes #{issue}");
    let request = PublicationRequest {
        schema: "csdlc.publication_request.v1".into(),
        issue,
        expected_generation: record.generation,
        expected_digest: record.digest.clone(),
        claim_id: "claim".into(),
        actor: "publisher".into(),
        repository: "example/repo".into(),
        base: "main".into(),
        head: "issue-7".into(),
        title: "Fixture".into(),
        body: publication_body.clone(),
        draft: true,
        remote: "origin".into(),
        token_file: None,
    };
    let intent = PublicationIntent {
        schema: "csdlc.publication_intent.v1".into(),
        issue,
        repository: "example/repo".into(),
        base: "main".into(),
        head: "issue-7".into(),
        title: "Fixture".into(),
        body: publication_body.clone(),
        draft: true,
        revision: revision.clone(),
        commit_sha: sha.clone(),
    };
    record_publication(
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
            body: publication_body,
            draft: true,
            state: "open".into(),
            head_sha: sha.clone(),
        },
    )
    .unwrap();
    record = Store::new(store.root()).load_record(issue).unwrap();
    assert_eq!(record.phase, LifecyclePhase::Published);
    (temp, store, record, sha)
}

#[test]
fn readiness_regression_and_exact_terminal_closeout_are_atomic_and_idempotent() {
    run_complete_lifecycle(7, "Gate 7 fixture", "gate7", true);
}

#[test]
fn merged_publication_reconciliation_projects_truth_before_closeout() {
    let (_temp, store, record, sha) = fixture_with_validation_history_and_publication(
        74,
        "Gate 6 merged reconciliation fixture",
        "merged-publication-reconciliation",
        vec![],
        false,
    );
    let reviewed_revision = record.review.as_ref().unwrap().reviewed_revision.clone();
    let body = "Closes #74".to_string();
    let request = PublicationRequest {
        schema: "csdlc.publication_request.v1".into(),
        issue: 74,
        expected_generation: record.generation,
        expected_digest: record.digest,
        claim_id: "claim".into(),
        actor: "publisher".into(),
        repository: "example/repo".into(),
        base: "main".into(),
        head: "issue-7".into(),
        title: "Fixture".into(),
        body: body.clone(),
        draft: true,
        remote: "origin".into(),
        token_file: None,
    };
    let intent = PublicationIntent {
        schema: "csdlc.publication_intent.v1".into(),
        issue: 74,
        repository: "example/repo".into(),
        base: "main".into(),
        head: "issue-7".into(),
        title: "Fixture".into(),
        body: body.clone(),
        draft: false,
        revision: reviewed_revision,
        commit_sha: sha.clone(),
    };
    let published = record_merged_publication(
        &store,
        &request,
        &intent,
        RemotePullRequest {
            number: 74,
            url: "https://example.invalid/74".into(),
            repository: "example/repo".into(),
            base: "main".into(),
            head: "issue-7".into(),
            title: "Fixture".into(),
            body,
            draft: false,
            state: "merged".into(),
            head_sha: sha,
        },
    )
    .unwrap();

    assert_eq!(published.phase, LifecyclePhase::Published);
    assert_eq!(
        published.publication.as_ref().unwrap().observed_state,
        "merged"
    );
    assert_eq!(
        published.transitions.last().unwrap().reason,
        "observed exact merged PR after current review"
    );
    let cards = store.load_cards(74).unwrap();
    let CardContent::Sor(sor) = &cards[&CardKind::Sor].content else {
        panic!("expected SOR card")
    };
    assert_eq!(sor.integration_state, IntegrationState::Merged);
    assert_eq!(sor.merge_state, MergeState::Merged);
    assert_eq!(
        published.audit.last().unwrap().operation,
        "record_merged_publication"
    );
}

#[test]
fn terminal_projection_and_receipt_recover_at_each_durable_boundary() {
    for (offset, stage) in [
        "before_journal",
        "after_journal",
        "after_projection",
        "after_projection_journal",
        "after_receipt_write",
        "after_receipt_rename",
        "after_receipt_journal",
    ]
    .into_iter()
    .enumerate()
    {
        let issue = 5_470 + offset as u64;
        let (temp, store, record, sha) = fixture_with_validation_history_and_publication(
            issue,
            "Terminal durability fixture",
            "terminal-durability",
            vec![ValidationResult {
                command: vec!["cargo".into(), "test".into()],
                purpose: "terminal durability proof".into(),
                outcome: EvidenceOutcome::Passed,
                evidence_ref: "durability-proof.json".into(),
            }],
            true,
        );
        let readiness = record_readiness(
            &store,
            ReadinessRequest {
                schema: "csdlc.readiness_request.v1".into(),
                issue,
                expected_generation: record.generation,
                expected_digest: record.digest,
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
            },
        )
        .unwrap();
        let publication = PublicationRequest {
            schema: "csdlc.publication_request.v1".into(),
            issue,
            expected_generation: readiness.generation,
            expected_digest: readiness.digest.clone(),
            claim_id: "claim".into(),
            actor: "publisher".into(),
            repository: "example/repo".into(),
            base: "main".into(),
            head: "issue-7".into(),
            title: "Fixture".into(),
            body: format!("Closes #{issue}"),
            draft: true,
            remote: "origin".into(),
            token_file: None,
        };
        let reviewed_revision = readiness.review.as_ref().unwrap().reviewed_revision.clone();
        record_merged_publication(
            &store,
            &publication,
            &PublicationIntent {
                schema: "csdlc.publication_intent.v1".into(),
                issue,
                repository: "example/repo".into(),
                base: "main".into(),
                head: "issue-7".into(),
                title: "Fixture".into(),
                body: format!("Closes #{issue}"),
                draft: false,
                revision: reviewed_revision,
                commit_sha: sha.clone(),
            },
            RemotePullRequest {
                number: 70,
                url: "https://example.invalid/70".into(),
                repository: "example/repo".into(),
                base: "main".into(),
                head: "issue-7".into(),
                title: "Fixture".into(),
                body: format!("Closes #{issue}"),
                draft: false,
                state: "merged".into(),
                head_sha: sha.clone(),
            },
        )
        .unwrap();
        let current = store.load_record(issue).unwrap();
        closeout_issue(
            &store,
            TerminalObservation {
                schema: "csdlc.terminal_observation.v1".into(),
                issue,
                expected_generation: current.generation,
                expected_digest: current.digest.clone(),
                claim_id: "claim".into(),
                actor: "closer".into(),
                pull_request: Some(70),
                disposition: TerminalDisposition::Merged,
                observed_sha: Some(sha),
                observed_state: "merged".into(),
                approved_no_pr_reason: None,
                receipt_path: format!("csdlc-v2/closeout/{issue}.json"),
            },
        )
        .unwrap();
        let terminal_start = store.load_record(issue).unwrap();
        store.retain_terminal_receipt(issue).unwrap();
        let original_record_digest = store.load_record(issue).unwrap().digest;
        let original_receipt_path = store.terminal_receipt_path(issue).unwrap();
        let original_receipt = std::fs::read(&original_receipt_path).unwrap();
        let request = ReconcileTerminalRequest {
            issue,
            expected_initialization_digest: terminal_start.initialization_digest.clone(),
            expected_branch: "issue-7".into(),
            expected_worktree: temp.path().to_string_lossy().into_owned(),
            actor: "durability-test".into(),
            reason: format!("fault boundary {stage}"),
            follow_ups: vec![],
        };
        std::env::set_var("CSDLC_V2_TEST_INTERRUPT_ISSUE", issue.to_string());
        std::env::set_var("CSDLC_V2_TEST_INTERRUPT_STAGE", stage);
        let interrupted = store.reconcile_terminal(request.clone()).unwrap_err();
        assert!(matches!(
            interrupted.code,
            csdlc_v2::ErrorCode::InterruptedTransaction
        ));
        std::env::remove_var("CSDLC_V2_TEST_INTERRUPT_ISSUE");
        std::env::remove_var("CSDLC_V2_TEST_INTERRUPT_STAGE");
        let journal = temp
            .path()
            .join(".git/csdlc-v2/terminal-transactions")
            .join(format!("{issue}.json"));
        if stage == "before_journal" {
            assert!(!journal.exists(), "journal unexpectedly written at {stage}");
            assert_eq!(
                store.load_record(issue).unwrap().digest,
                terminal_start.digest
            );
        } else {
            assert!(journal.is_file(), "journal missing at {stage}");
            if stage == "after_journal" {
                assert_eq!(
                    store.load_record(issue).unwrap().digest,
                    original_record_digest
                );
                assert_eq!(
                    std::fs::read(&original_receipt_path).unwrap(),
                    original_receipt
                );
            }
            let journal_before_uncoordinated_edit = std::fs::read(&journal).unwrap();
            let receipt_path = store.terminal_receipt_path(issue).unwrap();
            let receipt_before_uncoordinated_edit = std::fs::read(&receipt_path).unwrap();
            let current = store.load_record(issue).unwrap();
            let uncoordinated_edit = edit_issue(
                &store,
                EditRequest {
                    issue,
                    card: CardKind::Spp,
                    expected_generation: current.generation,
                    expected_digest: current.digest,
                    claim_id: "claim".into(),
                    actor: "uncoordinated-editor".into(),
                    reason: "prove shared terminal recovery fails closed".into(),
                    operation: SemanticOperation::UpdatePlanStep {
                        step_id: "one".into(),
                        status: StepStatus::Completed,
                    },
                    fail_after_backup: false,
                },
            )
            .unwrap_err();
            assert_eq!(
                uncoordinated_edit.code,
                csdlc_v2::ErrorCode::ReconciliationRequired
            );
            assert_eq!(
                std::fs::read(&journal).unwrap(),
                journal_before_uncoordinated_edit
            );
            assert_eq!(
                std::fs::read(&receipt_path).unwrap(),
                receipt_before_uncoordinated_edit
            );
        }
        let recovered = store.reconcile_terminal(request).unwrap();
        assert_eq!(recovered.phase, LifecyclePhase::ClosedOut);
        assert!(!journal.exists(), "journal retained after {stage}");
        let receipt = store.load_terminal_receipt(issue).unwrap().unwrap();
        assert_eq!(receipt.record.digest, recovered.digest);
    }
}

#[test]
fn terminal_design_repair_after_journal_interruption_preserves_recoverable_journal() {
    let issue = 5_467;
    let (temp, store, mut target, sha) = fixture_with_validation_history_and_publication(
        issue,
        "Terminal repair target fixture",
        "terminal-repair-target",
        vec![ValidationResult {
            command: vec!["cargo".into(), "test".into()],
            purpose: "terminal repair proof".into(),
            outcome: EvidenceOutcome::Passed,
            evidence_ref: "terminal-repair-proof.json".into(),
        }],
        true,
    );
    let authority_issue = 5_487;
    let authority = initialize_issue(
        &store,
        BootstrapRequest {
            issue: authority_issue,
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
                protected_paths: vec!["authority".into()],
                purpose: "terminal repair authority".into(),
            },
            initial: InitialCardInput {
                title: "Terminal repair authority fixture".into(),
                slug: "terminal-repair-authority".into(),
                version: "v0.91.7".into(),
                goal: "authorize terminal design repair".into(),
                required_outcome: "repair authority".into(),
                declared_scope: vec!["terminal repair authority".into()],
                authority_boundary: vec!["no merge".into()],
                task_boundary: "authorize fixture repair".into(),
                deliverables: vec!["authority record".into()],
                acceptance_criteria: vec!["authority exists".into()],
                dependencies: vec!["none".into()],
                repo_inputs: vec!["docs/design.md".into()],
                non_goals: vec!["network".into()],
                plan_summary: "authorize terminal repair".into(),
                steps: vec![PlanStep {
                    id: "one".into(),
                    action: "authorize".into(),
                    acceptance_ids: vec!["AC-1".into()],
                    status: StepStatus::Pending,
                }],
                invariants: vec!["same store".into()],
                risks: vec!["stale target".into()],
                planning_profile: PlanningProfile::Small,
                stop_conditions: vec!["mismatch".into()],
                validation_lanes: vec![ValidationLane {
                    lane: "focused".into(),
                    proof_role: "terminal repair authority".into(),
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
    target = record_readiness(
        &store,
        ReadinessRequest {
            schema: "csdlc.readiness_request.v1".into(),
            issue,
            expected_generation: target.generation,
            expected_digest: target.digest.clone(),
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
        },
    )
    .unwrap();
    record_merged_publication(
        &store,
        &PublicationRequest {
            schema: "csdlc.publication_request.v1".into(),
            issue,
            expected_generation: target.generation,
            expected_digest: target.digest.clone(),
            claim_id: "claim".into(),
            actor: "publisher".into(),
            repository: "example/repo".into(),
            base: "main".into(),
            head: "issue-7".into(),
            title: "Fixture".into(),
            body: format!("Closes #{issue}"),
            draft: true,
            remote: "origin".into(),
            token_file: None,
        },
        &PublicationIntent {
            schema: "csdlc.publication_intent.v1".into(),
            issue,
            repository: "example/repo".into(),
            base: "main".into(),
            head: "issue-7".into(),
            title: "Fixture".into(),
            body: format!("Closes #{issue}"),
            draft: false,
            revision: target.review.as_ref().unwrap().reviewed_revision.clone(),
            commit_sha: sha.clone(),
        },
        RemotePullRequest {
            number: 70,
            url: "https://example.invalid/70".into(),
            repository: "example/repo".into(),
            base: "main".into(),
            head: "issue-7".into(),
            title: "Fixture".into(),
            body: format!("Closes #{issue}"),
            draft: false,
            state: "merged".into(),
            head_sha: sha.clone(),
        },
    )
    .unwrap();
    let current = store.load_record(issue).unwrap();
    closeout_issue(
        &store,
        TerminalObservation {
            schema: "csdlc.terminal_observation.v1".into(),
            issue,
            expected_generation: current.generation,
            expected_digest: current.digest,
            claim_id: "claim".into(),
            actor: "closer".into(),
            pull_request: Some(70),
            disposition: TerminalDisposition::Merged,
            observed_sha: Some(sha),
            observed_state: "merged".into(),
            approved_no_pr_reason: None,
            receipt_path: format!("csdlc-v2/closeout/{issue}.json"),
        },
    )
    .unwrap();
    let target = store.load_record(issue).unwrap();
    let receipt = store.retain_terminal_receipt(issue).unwrap();
    let design = std::fs::read(temp.path().join("docs/design.md")).unwrap();
    let diagram = std::fs::read(temp.path().join("docs/diagram.mmd")).unwrap();
    let mut repair_request = TerminalDesignRepairRequest {
        authority_issue: authority.issue,
        target_issue: issue,
        expected_authority_generation: authority.generation,
        expected_authority_digest: authority.digest,
        expected_target_generation: target.generation,
        expected_target_digest: target.digest.clone(),
        expected_receipt_digest: receipt.digest,
        authority_claim_id: "claim".into(),
        actor: "codex".into(),
        reviewer: "reviewer".into(),
        source_design_path: "docs/design.md".into(),
        source_diagram_path: "docs/diagram.mmd".into(),
        expected_design_digest: digest(&design),
        expected_diagram_digest: digest(&diagram),
        fail_after_stage: Some("after_journal".into()),
    };
    let interrupted = store
        .repair_terminal_design(repair_request.clone())
        .unwrap_err();
    assert!(matches!(
        interrupted.code,
        csdlc_v2::ErrorCode::InterruptedTransaction
    ));
    let journal = temp
        .path()
        .join(".git/csdlc-v2/terminal-transactions")
        .join(format!("{issue}.json"));
    assert!(journal.is_file(), "repair journal must remain recoverable");
    repair_request.fail_after_stage = None;
    let recovered = store.repair_terminal_design(repair_request).unwrap();
    assert!(
        !journal.exists(),
        "repair journal should clear after recovery"
    );
    assert_eq!(recovered.phase, LifecyclePhase::ClosedOut);
    assert_ne!(recovered.digest, target.digest);
    let recovered_receipt = store.load_terminal_receipt(issue).unwrap().unwrap();
    assert_eq!(recovered_receipt.record.digest, recovered.digest);
}

#[test]
fn later_pass_supersedes_waiting_validation_through_terminal_closeout() {
    let identity = || ValidationResult {
        command: vec!["cargo".into(), "test".into()],
        purpose: "proof".into(),
        outcome: EvidenceOutcome::Waiting,
        evidence_ref: "evidence.json".into(),
    };
    let mut passed = identity();
    passed.outcome = EvidenceOutcome::Passed;
    run_complete_lifecycle_with_validation_history(
        71,
        "Gate 7 supersession fixture",
        "validation-supersession",
        false,
        vec![identity(), passed],
    );
}
#[test]
fn later_failure_blocks_merged_and_closed_unmerged_terminal_closeout() {
    for (issue, disposition) in [
        (72, TerminalDisposition::Merged),
        (73, TerminalDisposition::ClosedUnmerged),
    ] {
        let (temp, store, mut record, sha) = fixture_with_validation_history(
            issue,
            "Gate 7 validation regression fixture",
            "validation-regression",
            vec![ValidationResult {
                command: vec!["cargo".into(), "test".into()],
                purpose: "proof".into(),
                outcome: EvidenceOutcome::Passed,
                evidence_ref: "evidence.json".into(),
            }],
        );
        record = record_readiness(
            &store,
            ReadinessRequest {
                schema: "csdlc.readiness_request.v1".into(),
                issue,
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
            },
        )
        .unwrap();
        record = edit(
            &store,
            &record,
            CardKind::Sor,
            SemanticOperation::RecordValidation {
                result: ValidationResult {
                    command: vec!["cargo".into(), "test".into()],
                    purpose: "proof".into(),
                    outcome: EvidenceOutcome::Failed,
                    evidence_ref: "evidence.json".into(),
                },
            },
        );
        let observed_state = match disposition {
            TerminalDisposition::Merged => "merged",
            TerminalDisposition::ClosedUnmerged => "closed",
            TerminalDisposition::ClosedNoPr => unreachable!(),
        };
        let terminal = TerminalObservation {
            schema: "csdlc.terminal_observation.v1".into(),
            issue,
            expected_generation: record.generation,
            expected_digest: record.digest.clone(),
            claim_id: "claim".into(),
            actor: "closer".into(),
            pull_request: Some(70),
            disposition,
            observed_sha: Some(sha),
            observed_state: observed_state.into(),
            approved_no_pr_reason: None,
            receipt_path: format!("csdlc-v2/closeout/{issue}.json"),
        };
        assert!(closeout_issue(&store, terminal).is_err());
        assert_eq!(
            Store::new(temp.path()).load_record(issue).unwrap().phase,
            LifecyclePhase::MergeReady
        );
    }

    let issue = 74;
    let (temp, store, mut record, _) = fixture_with_validation_history_and_publication(
        issue,
        "Gate 7 no-PR validation regression fixture",
        "validation-regression-no-pr",
        vec![ValidationResult {
            command: vec!["cargo".into(), "test".into()],
            purpose: "proof".into(),
            outcome: EvidenceOutcome::Passed,
            evidence_ref: "evidence.json".into(),
        }],
        false,
    );
    record = edit(
        &store,
        &record,
        CardKind::Sor,
        SemanticOperation::RecordValidation {
            result: ValidationResult {
                command: vec!["cargo".into(), "test".into()],
                purpose: "proof".into(),
                outcome: EvidenceOutcome::Failed,
                evidence_ref: "evidence.json".into(),
            },
        },
    );
    let terminal = TerminalObservation {
        schema: "csdlc.terminal_observation.v1".into(),
        issue,
        expected_generation: record.generation,
        expected_digest: record.digest.clone(),
        claim_id: "claim".into(),
        actor: "closer".into(),
        pull_request: None,
        disposition: TerminalDisposition::ClosedNoPr,
        observed_sha: None,
        observed_state: "closed_no_pr".into(),
        approved_no_pr_reason: Some("operator-approved no-PR closeout".into()),
        receipt_path: format!("csdlc-v2/closeout/{issue}.json"),
    };
    assert!(closeout_issue(&store, terminal).is_err());
    assert_eq!(
        Store::new(temp.path()).load_record(issue).unwrap().phase,
        LifecyclePhase::Reviewed
    );
}

#[test]
fn unresolved_post_review_finding_is_not_projected_as_complete() {
    let issue = 74;
    let (temp, store, record, sha) = fixture_with_validation_history(
        issue,
        "Gate 7 unresolved review fixture",
        "unresolved-review",
        vec![ValidationResult {
            command: vec!["cargo".into(), "test".into()],
            purpose: "proof".into(),
            outcome: EvidenceOutcome::Passed,
            evidence_ref: "evidence.json".into(),
        }],
    );
    let record = record_readiness(
        &store,
        ReadinessRequest {
            schema: "csdlc.readiness_request.v1".into(),
            issue,
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
        },
    )
    .unwrap();
    let record = edit(
        &store,
        &record,
        CardKind::Srp,
        SemanticOperation::RecordFinding {
            finding: csdlc_v2::cards::ReviewFinding {
                id: "late-finding".into(),
                severity: csdlc_v2::cards::FindingSeverity::P1,
                summary: "late unresolved finding".into(),
                actionable: true,
                in_scope: true,
                disposition: csdlc_v2::cards::FindingDisposition::Open,
                fix_revision: None,
                route: None,
            },
        },
    );
    closeout_issue(
        &store,
        TerminalObservation {
            schema: "csdlc.terminal_observation.v1".into(),
            issue,
            expected_generation: record.generation,
            expected_digest: record.digest.clone(),
            claim_id: "claim".into(),
            actor: "closer".into(),
            pull_request: Some(70),
            disposition: TerminalDisposition::Merged,
            observed_sha: Some(sha),
            observed_state: "merged".into(),
            approved_no_pr_reason: None,
            receipt_path: format!("csdlc-v2/closeout/{issue}.json"),
        },
    )
    .unwrap();
    let receipt = store.retain_terminal_receipt(issue).unwrap();
    let reconciled = store
        .reconcile_terminal(csdlc_v2::ReconcileTerminalRequest {
            issue,
            expected_initialization_digest: receipt.initialization_digest,
            expected_branch: "issue-7".into(),
            expected_worktree: temp.path().to_string_lossy().into_owned(),
            actor: "closeout-retainer".into(),
            reason: "preserve unresolved review truth".into(),
            follow_ups: vec![],
        })
        .unwrap();
    assert_ne!(
        store.load_cards(issue).unwrap()[&CardKind::Srp].status,
        csdlc_v2::cards::CardStatus::Complete
    );
    assert_ne!(
        store.load_terminal_receipt(issue).unwrap().unwrap().cards[&CardKind::Srp].status,
        csdlc_v2::cards::CardStatus::Complete
    );
    assert_eq!(reconciled.phase, LifecyclePhase::ClosedOut);
}

#[test]
fn terminal_reconcile_materializes_missing_projection_from_retained_receipt() {
    let issue = 75;
    let (temp, store, record, sha) = fixture_with_validation_history(
        issue,
        "Gate 7 retained receipt fixture",
        "missing-projection-reconcile",
        vec![ValidationResult {
            command: vec!["cargo".into(), "test".into()],
            purpose: "proof".into(),
            outcome: EvidenceOutcome::Passed,
            evidence_ref: "evidence.json".into(),
        }],
    );
    let record = record_readiness(
        &store,
        ReadinessRequest {
            schema: "csdlc.readiness_request.v1".into(),
            issue,
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
        },
    )
    .unwrap();
    closeout_issue(
        &store,
        TerminalObservation {
            schema: "csdlc.terminal_observation.v1".into(),
            issue,
            expected_generation: record.generation,
            expected_digest: record.digest.clone(),
            claim_id: "claim".into(),
            actor: "closer".into(),
            pull_request: Some(70),
            disposition: TerminalDisposition::Merged,
            observed_sha: Some(sha),
            observed_state: "merged".into(),
            approved_no_pr_reason: None,
            receipt_path: format!("csdlc-v2/closeout/{issue}.json"),
        },
    )
    .unwrap();
    let receipt = store.retain_terminal_receipt(issue).unwrap();
    fs::remove_dir_all(store.issue_dir(issue)).unwrap();

    let reconciled = store
        .reconcile_terminal(csdlc_v2::ReconcileTerminalRequest {
            issue,
            expected_initialization_digest: receipt.initialization_digest,
            expected_branch: "issue-7".into(),
            expected_worktree: temp.path().to_string_lossy().into_owned(),
            actor: "closeout-retainer".into(),
            reason: "materialize retained terminal authority without local projection".into(),
            follow_ups: vec![],
        })
        .unwrap();

    assert_eq!(reconciled.phase, LifecyclePhase::ClosedOut);
    assert!(store.issue_dir(issue).join("index.json").exists());
    assert_eq!(
        fs::read_to_string(temp.path().join(&reconciled.design_path)).unwrap(),
        receipt.authored_artifacts["docs/design.md"]
    );
    assert_eq!(
        store
            .load_terminal_receipt(issue)
            .unwrap()
            .unwrap()
            .record
            .digest,
        reconciled.digest
    );
}

#[test]
fn terminal_reconcile_rejects_partial_projection_loss() {
    let (temp, store, issue, receipt) =
        retained_receipt_fixture(76, "partial-projection-reconcile");
    fs::remove_file(store.issue_dir(issue).join("cards/sor.values.json")).unwrap();

    let error = store
        .reconcile_terminal(csdlc_v2::ReconcileTerminalRequest {
            issue,
            expected_initialization_digest: receipt.initialization_digest,
            expected_branch: "issue-7".into(),
            expected_worktree: temp.path().to_string_lossy().into_owned(),
            actor: "closeout-retainer".into(),
            reason: "must reject partial projection loss".into(),
            follow_ups: vec![],
        })
        .unwrap_err();

    assert!(matches!(error.code, csdlc_v2::ErrorCode::Io));
}

#[test]
fn terminal_reconcile_rejects_missing_index_inside_existing_projection_dir() {
    let (temp, store, issue, receipt) = retained_receipt_fixture(77, "missing-index-reconcile");
    fs::remove_file(store.issue_dir(issue).join("index.json")).unwrap();

    let error = store
        .reconcile_terminal(csdlc_v2::ReconcileTerminalRequest {
            issue,
            expected_initialization_digest: receipt.initialization_digest,
            expected_branch: "issue-7".into(),
            expected_worktree: temp.path().to_string_lossy().into_owned(),
            actor: "closeout-retainer".into(),
            reason: "must reject missing index inside existing projection".into(),
            follow_ups: vec![],
        })
        .unwrap_err();

    assert!(matches!(error.code, csdlc_v2::ErrorCode::Io));
}

#[test]
fn terminal_reconcile_rejects_misplaced_receipt_for_missing_projection() {
    let source_issue = 78;
    let target_issue = 79;
    let (temp, store, _, receipt) =
        retained_receipt_fixture(source_issue, "misplaced-receipt-reconcile");
    let source_receipt = fs::read(store.terminal_receipt_path(source_issue).unwrap()).unwrap();
    fs::create_dir_all(
        store
            .terminal_receipt_path(target_issue)
            .unwrap()
            .parent()
            .unwrap(),
    )
    .unwrap();
    fs::write(
        store.terminal_receipt_path(target_issue).unwrap(),
        source_receipt,
    )
    .unwrap();
    assert!(!store.issue_dir(target_issue).exists());

    let error = store
        .reconcile_terminal(csdlc_v2::ReconcileTerminalRequest {
            issue: target_issue,
            expected_initialization_digest: receipt.initialization_digest,
            expected_branch: "issue-7".into(),
            expected_worktree: temp.path().to_string_lossy().into_owned(),
            actor: "closeout-retainer".into(),
            reason: "must reject misplaced terminal receipt".into(),
            follow_ups: vec![],
        })
        .unwrap_err();

    assert!(matches!(
        error.code,
        csdlc_v2::ErrorCode::ReconciliationRequired
    ));
    assert!(!store.issue_dir(target_issue).exists());
}

fn retained_receipt_fixture(
    issue: u64,
    scenario: &str,
) -> (tempfile::TempDir, Store, u64, csdlc_v2::TerminalReceipt) {
    let (temp, store, record, sha) = fixture_with_validation_history(
        issue,
        "Gate 7 retained receipt fixture",
        scenario,
        vec![ValidationResult {
            command: vec!["cargo".into(), "test".into()],
            purpose: "proof".into(),
            outcome: EvidenceOutcome::Passed,
            evidence_ref: "evidence.json".into(),
        }],
    );
    let record = record_readiness(
        &store,
        ReadinessRequest {
            schema: "csdlc.readiness_request.v1".into(),
            issue,
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
        },
    )
    .unwrap();
    closeout_issue(
        &store,
        TerminalObservation {
            schema: "csdlc.terminal_observation.v1".into(),
            issue,
            expected_generation: record.generation,
            expected_digest: record.digest.clone(),
            claim_id: "claim".into(),
            actor: "closer".into(),
            pull_request: Some(70),
            disposition: TerminalDisposition::Merged,
            observed_sha: Some(sha),
            observed_state: "merged".into(),
            approved_no_pr_reason: None,
            receipt_path: format!("csdlc-v2/closeout/{issue}.json"),
        },
    )
    .unwrap();
    let receipt = store.retain_terminal_receipt(issue).unwrap();
    (temp, store, issue, receipt)
}

fn terminal_plan_repair_authority(
    store: &Store,
    issue: u64,
    worktree: &std::path::Path,
    protected_paths: Vec<String>,
) -> csdlc_v2::IssueRecord {
    initialize_issue(
        store,
        BootstrapRequest {
            issue,
            repository: "example/repo".into(),
            design_path: "docs/design.md".into(),
            diagram_path: "docs/diagram.mmd".into(),
            design_reviewer: "architect".into(),
            design_approved: true,
            claim: Claim {
                id: format!("authority-{issue}"),
                owner: "agent".into(),
                generation: 0,
                acquired_unix_seconds: 1,
                expires_unix_seconds: u64::MAX,
                heartbeat_unix_seconds: 1,
                branch: "issue-7".into(),
                worktree: worktree.to_string_lossy().into_owned(),
                protected_paths,
                purpose: "terminal plan repair authority".into(),
            },
            initial: InitialCardInput {
                title: "Terminal plan repair authority".into(),
                slug: format!("terminal-plan-authority-{issue}"),
                version: "v0.91.7".into(),
                goal: "repair one stale terminal plan step".into(),
                required_outcome: "atomic terminal plan truth".into(),
                declared_scope: vec!["terminal plan".into()],
                authority_boundary: vec!["one target".into()],
                task_boundary: "authorize terminal plan repair".into(),
                deliverables: vec!["repair".into()],
                acceptance_criteria: vec!["atomic parity".into()],
                dependencies: vec!["closed target".into()],
                repo_inputs: vec!["receipt".into()],
                non_goals: vec!["general edits".into()],
                plan_summary: "repair target".into(),
                steps: vec![PlanStep {
                    id: "authority".into(),
                    action: "authorize".into(),
                    acceptance_ids: vec!["AC-1".into()],
                    status: StepStatus::Pending,
                }],
                invariants: vec!["fail closed".into()],
                risks: vec!["stale receipt".into()],
                planning_profile: PlanningProfile::Small,
                stop_conditions: vec!["mismatch".into()],
                validation_lanes: vec![ValidationLane {
                    lane: "focused".into(),
                    proof_role: "terminal repair".into(),
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
                review_prompts: vec!["review atomicity".into()],
            },
        },
    )
    .unwrap()
}

#[test]
fn terminal_plan_repair_is_scoped_atomic_and_receipt_bound() {
    let target_issue = 90;
    let (temp, store, _, receipt) = retained_receipt_fixture(target_issue, "terminal-plan-repair");
    let original_record = store.load_record(target_issue).unwrap();
    let original_cards = store.load_cards(target_issue).unwrap();
    let original_receipt =
        std::fs::read(store.terminal_receipt_path(target_issue).unwrap()).unwrap();

    let unscoped =
        terminal_plan_repair_authority(&store, 91, temp.path(), vec![".csdlc/issues/89".into()]);
    let scoped = terminal_plan_repair_authority(
        &store,
        92,
        temp.path(),
        vec![
            format!(".csdlc/issues/{target_issue}"),
            ".csdlc/issues/91".into(),
        ],
    );
    let request = |authority: &csdlc_v2::IssueRecord| TerminalPlanStepRepairRequest {
        authority_issue: authority.issue,
        target_issue,
        expected_authority_generation: authority.generation,
        expected_authority_digest: authority.digest.clone(),
        expected_target_generation: original_record.generation,
        expected_target_digest: original_record.digest.clone(),
        expected_receipt_digest: receipt.digest.clone(),
        authority_claim_id: format!("authority-{}", authority.issue),
        actor: "repairer".into(),
        step_id: "one".into(),
        fail_after_stage: None,
    };

    let scope_error = store
        .repair_terminal_plan_step(request(&unscoped))
        .unwrap_err();
    assert_eq!(scope_error.code, csdlc_v2::ErrorCode::InvalidInput);

    let mut interrupted = request(&scoped);
    interrupted.fail_after_stage = Some("after_projection".into());
    let interruption = store.repair_terminal_plan_step(interrupted).unwrap_err();
    assert_eq!(
        interruption.code,
        csdlc_v2::ErrorCode::InterruptedTransaction
    );
    assert_eq!(store.load_record(target_issue).unwrap(), original_record);
    assert_eq!(store.load_cards(target_issue).unwrap(), original_cards);
    assert_eq!(
        std::fs::read(store.terminal_receipt_path(target_issue).unwrap()).unwrap(),
        original_receipt
    );

    let repaired = store.repair_terminal_plan_step(request(&scoped)).unwrap();
    assert_eq!(repaired.generation, original_record.generation + 1);
    let repaired_cards = store.load_cards(target_issue).unwrap();
    let CardContent::Spp(spp) = &repaired_cards[&CardKind::Spp].content else {
        panic!("SPP content");
    };
    assert_eq!(spp.steps[0].status, StepStatus::Completed);
    let repaired_receipt = store.load_terminal_receipt(target_issue).unwrap().unwrap();
    assert_eq!(repaired_receipt.record.digest, repaired.digest);
    assert_eq!(repaired_receipt.cards, repaired_cards);

    let stale_target = store
        .repair_terminal_plan_step(request(&scoped))
        .unwrap_err();
    assert_eq!(stale_target.code, csdlc_v2::ErrorCode::StaleDigest);

    let mut stale_receipt = request(&scoped);
    stale_receipt.expected_target_generation = repaired.generation;
    stale_receipt.expected_target_digest = repaired.digest;
    let stale_receipt = store.repair_terminal_plan_step(stale_receipt).unwrap_err();
    assert_eq!(stale_receipt.code, csdlc_v2::ErrorCode::StaleDigest);

    let mut nonterminal = request(&scoped);
    nonterminal.target_issue = unscoped.issue;
    nonterminal.expected_target_generation = unscoped.generation;
    nonterminal.expected_target_digest = unscoped.digest;
    let phase_error = store.repair_terminal_plan_step(nonterminal).unwrap_err();
    assert_eq!(phase_error.code, csdlc_v2::ErrorCode::InvalidTransition);
}

#[test]
fn terminal_sor_artifact_repair_is_scoped_atomic_and_receipt_bound() {
    let target_issue = 93;
    let (temp, store, _, receipt) =
        retained_receipt_fixture(target_issue, "terminal-sor-artifact-repair");
    let reconciled = store
        .reconcile_terminal(ReconcileTerminalRequest {
            issue: target_issue,
            expected_initialization_digest: receipt.initialization_digest,
            expected_branch: "issue-7".into(),
            expected_worktree: temp.path().to_string_lossy().into_owned(),
            actor: "closeout-retainer".into(),
            reason: "materialize retained paths before SOR repair".into(),
            follow_ups: vec![],
        })
        .unwrap();
    let receipt = store.load_terminal_receipt(target_issue).unwrap().unwrap();
    let original_cards = store.load_cards(target_issue).unwrap();
    let original_receipt =
        std::fs::read(store.terminal_receipt_path(target_issue).unwrap()).unwrap();
    let retained_ref = reconciled.diagram_path.clone();
    let artifact_digest = digest(receipt.authored_artifacts[&retained_ref].as_bytes());

    let unscoped =
        terminal_plan_repair_authority(&store, 94, temp.path(), vec![".csdlc/issues/92".into()]);
    let scoped = terminal_plan_repair_authority(
        &store,
        95,
        temp.path(),
        vec![format!(".csdlc/issues/{target_issue}")],
    );
    let request = |authority: &csdlc_v2::IssueRecord| TerminalSorArtifactRepairRequest {
        authority_issue: authority.issue,
        target_issue,
        expected_authority_generation: authority.generation,
        expected_authority_digest: authority.digest.clone(),
        expected_target_generation: reconciled.generation,
        expected_target_digest: reconciled.digest.clone(),
        expected_receipt_digest: receipt.digest.clone(),
        authority_claim_id: format!("authority-{}", authority.issue),
        actor: "repairer".into(),
        stale_ref: "artifact".into(),
        retained_ref: retained_ref.clone(),
        expected_artifact_digest: artifact_digest.clone(),
        fail_after_stage: None,
    };

    let scope_error = store
        .repair_terminal_sor_artifact(request(&unscoped))
        .unwrap_err();
    assert_eq!(scope_error.code, csdlc_v2::ErrorCode::InvalidInput);

    let mut wrong_bytes = request(&scoped);
    wrong_bytes.expected_artifact_digest = "wrong".into();
    let digest_error = store.repair_terminal_sor_artifact(wrong_bytes).unwrap_err();
    assert_eq!(digest_error.code, csdlc_v2::ErrorCode::StaleDigest);

    let mut interrupted = request(&scoped);
    interrupted.fail_after_stage = Some("after_projection".into());
    let interruption = store.repair_terminal_sor_artifact(interrupted).unwrap_err();
    assert_eq!(
        interruption.code,
        csdlc_v2::ErrorCode::InterruptedTransaction
    );
    assert_eq!(store.load_record(target_issue).unwrap(), reconciled);
    assert_eq!(store.load_cards(target_issue).unwrap(), original_cards);
    assert_eq!(
        std::fs::read(store.terminal_receipt_path(target_issue).unwrap()).unwrap(),
        original_receipt
    );

    let repaired = store
        .repair_terminal_sor_artifact(request(&scoped))
        .unwrap();
    let repaired_cards = store.load_cards(target_issue).unwrap();
    let CardContent::Sor(sor) = &repaired_cards[&CardKind::Sor].content else {
        panic!("SOR content");
    };
    assert_eq!(sor.artifacts, vec![retained_ref.clone()]);
    let repaired_receipt = store.load_terminal_receipt(target_issue).unwrap().unwrap();
    assert_eq!(repaired_receipt.record.digest, repaired.digest);
    assert_eq!(repaired_receipt.cards, repaired_cards);

    let stale_target = store
        .repair_terminal_sor_artifact(request(&scoped))
        .unwrap_err();
    assert_eq!(stale_target.code, csdlc_v2::ErrorCode::StaleDigest);
}

pub(crate) fn run_complete_lifecycle(
    issue: u64,
    title: &str,
    scenario: &str,
    hostile: bool,
) -> csdlc_v2::NormalizedOutcome {
    run_complete_lifecycle_with_validation_history(
        issue,
        title,
        scenario,
        hostile,
        vec![ValidationResult {
            command: vec!["cargo".into(), "test".into()],
            purpose: "proof".into(),
            outcome: EvidenceOutcome::Passed,
            evidence_ref: "evidence.json".into(),
        }],
    )
}

fn run_complete_lifecycle_with_validation_history(
    issue: u64,
    title: &str,
    scenario: &str,
    hostile: bool,
    validation_history: Vec<ValidationResult>,
) -> csdlc_v2::NormalizedOutcome {
    let (temp, store, mut record, sha) =
        fixture_with_validation_history(issue, title, scenario, validation_history);
    let mut request = ReadinessRequest {
        schema: "csdlc.readiness_request.v1".into(),
        issue,
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
    record_readiness(&store, request.clone()).unwrap();
    record = Store::new(store.root()).load_record(issue).unwrap();
    assert_eq!(record.phase, LifecyclePhase::MergeReady);
    if hostile {
        request.expected_generation = record.generation;
        request.expected_digest = record.digest.clone();
        request.checks[0].conclusion = csdlc_v2::CheckConclusion::Failure;
        record = record_readiness(&store, request).unwrap();
        assert_eq!(record.phase, LifecyclePhase::Published);
    }

    let wrong = TerminalObservation {
        schema: "csdlc.terminal_observation.v1".into(),
        issue,
        expected_generation: record.generation,
        expected_digest: record.digest.clone(),
        claim_id: "claim".into(),
        actor: "closer".into(),
        pull_request: Some(70),
        disposition: TerminalDisposition::Merged,
        observed_sha: Some("wrong".into()),
        observed_state: "merged".into(),
        approved_no_pr_reason: None,
        receipt_path: format!("csdlc-v2/closeout/{issue}.json"),
    };
    if hostile {
        assert!(closeout_issue(&store, wrong).is_err());
    }
    let current = store.load_record(issue).unwrap();
    assert_eq!(
        current.phase,
        if hostile {
            LifecyclePhase::Published
        } else {
            LifecyclePhase::MergeReady
        }
    );

    let mut green = ReadinessRequest {
        schema: "csdlc.readiness_request.v1".into(),
        issue,
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
    let stale = temp.path().join("stale-issue-record");
    copy_dir_all(&store.issue_dir(issue), &stale);
    let terminal = TerminalObservation {
        schema: "csdlc.terminal_observation.v1".into(),
        issue,
        expected_generation: record.generation,
        expected_digest: record.digest.clone(),
        claim_id: "claim".into(),
        actor: "closer".into(),
        pull_request: Some(70),
        disposition: TerminalDisposition::Merged,
        observed_sha: Some(sha),
        observed_state: "merged".into(),
        approved_no_pr_reason: None,
        receipt_path: format!("/legacy/absolute/closeout/{issue}.json"),
    };
    closeout_issue(&store, terminal.clone()).unwrap();
    let closed = Store::new(store.root()).load_record(issue).unwrap();
    assert_eq!(closed.phase, LifecyclePhase::ClosedOut);
    assert!(closed.claim.is_none());
    let mut retry = terminal;
    retry.receipt_path = format!("csdlc-v2/closeout/{issue}.json");
    assert_eq!(closeout_issue(&store, retry).unwrap(), closed);
    let receipt = store.retain_terminal_receipt(issue).unwrap();
    assert_eq!(receipt.record.generation, closed.generation + 1);
    assert_eq!(
        receipt.cards[&CardKind::Srp].status,
        csdlc_v2::cards::CardStatus::PrePhase
    );
    assert_eq!(
        receipt.record.terminal.as_ref().unwrap().receipt_path,
        format!("csdlc-v2/closeout/{issue}.json")
    );
    assert_eq!(receipt.cards.len(), 6);
    assert_eq!(receipt.authored_artifacts.len(), 2);
    let receipt_path = store.terminal_receipt_path(issue).unwrap();
    let retained = fs::read(&receipt_path).unwrap();
    let mut tampered: serde_json::Value = serde_json::from_slice(&retained).unwrap();
    tampered["cards"]["sor"]["status"] = serde_json::json!("draft");
    fs::write(&receipt_path, serde_json::to_vec_pretty(&tampered).unwrap()).unwrap();
    assert!(store.load_terminal_receipt(issue).is_err());
    fs::write(&receipt_path, retained).unwrap();
    let terminal_index_path = store.issue_dir(issue).join("index.json");
    let terminal_index = fs::read(&terminal_index_path).unwrap();
    let mut divergent: serde_json::Value = serde_json::from_slice(&terminal_index).unwrap();
    divergent["terminal"]["released_branch"] = serde_json::json!("different-branch");
    let divergent = serde_json::to_vec_pretty(&divergent).unwrap();
    fs::write(&terminal_index_path, &divergent).unwrap();
    let conflict = store.retain_terminal_receipt(issue).unwrap_err();
    assert!(matches!(
        conflict.code,
        csdlc_v2::ErrorCode::ReconciliationRequired
    ));
    assert_eq!(fs::read(&terminal_index_path).unwrap(), divergent);
    fs::write(&terminal_index_path, terminal_index).unwrap();
    fs::remove_dir_all(store.issue_dir(issue)).unwrap();
    fs::rename(&stale, store.issue_dir(issue)).unwrap();
    assert!(store.load_record(issue).unwrap().claim.is_some());
    let stale_index = fs::read(store.issue_dir(issue).join("index.json")).unwrap();
    let conflict = store.retain_terminal_receipt(issue).unwrap_err();
    assert!(matches!(
        conflict.code,
        csdlc_v2::ErrorCode::ReconciliationRequired
    ));
    assert_eq!(
        fs::read(store.issue_dir(issue).join("index.json")).unwrap(),
        stale_index
    );
    git(temp.path(), &["branch", "-m", "main"]);
    let unsafe_checkout = store
        .reconcile_terminal(csdlc_v2::ReconcileTerminalRequest {
            issue,
            expected_initialization_digest: receipt.initialization_digest.clone(),
            expected_branch: "issue-7".into(),
            expected_worktree: temp.path().to_string_lossy().into_owned(),
            actor: "closeout-retainer".into(),
            reason: "must not mutate primary checkout".into(),
            follow_ups: vec!["#5411 follow-up".into()],
        })
        .unwrap_err();
    assert!(matches!(
        unsafe_checkout.code,
        csdlc_v2::ErrorCode::UnsafeCheckout
    ));
    git(temp.path(), &["branch", "-m", "issue-7"]);
    let design_path = temp.path().join("docs/design.md");
    fs::write(&design_path, "# stale design\n").unwrap();
    let reconciled = store
        .reconcile_terminal(csdlc_v2::ReconcileTerminalRequest {
            issue,
            expected_initialization_digest: receipt.initialization_digest.clone(),
            expected_branch: "issue-7".into(),
            expected_worktree: temp.path().to_string_lossy().into_owned(),
            actor: "closeout-retainer".into(),
            reason: "materialize shared terminal authority".into(),
            follow_ups: vec!["#5411 follow-up".into()],
        })
        .unwrap();
    assert_eq!(reconciled.phase, LifecyclePhase::ClosedOut);
    assert_eq!(reconciled.generation, receipt.record.generation + 1);
    assert_eq!(
        reconciled.audit.last().unwrap().operation,
        "reconcile_terminal"
    );
    assert_eq!(
        fs::read_to_string(temp.path().join(&reconciled.design_path)).unwrap(),
        receipt.authored_artifacts["docs/design.md"]
    );
    assert_eq!(
        fs::read_to_string(&design_path).unwrap(),
        "# stale design\n"
    );
    assert_eq!(
        Store::new(store.root()).load_cards(issue).unwrap()[&CardKind::Srp].status,
        csdlc_v2::cards::CardStatus::Complete
    );
    let repeated = store
        .reconcile_terminal(csdlc_v2::ReconcileTerminalRequest {
            issue,
            expected_initialization_digest: receipt.initialization_digest.clone(),
            expected_branch: "issue-7".into(),
            expected_worktree: temp.path().to_string_lossy().into_owned(),
            actor: "closeout-retainer".into(),
            reason: "materialize shared terminal authority".into(),
            follow_ups: vec!["#5411 follow-up".into()],
        })
        .unwrap();
    assert_eq!(repeated, reconciled);
    let reconciled_receipt = store.load_terminal_receipt(issue).unwrap().unwrap();
    assert_eq!(
        reconciled_receipt.cards[&CardKind::Srp].status,
        csdlc_v2::cards::CardStatus::Complete
    );
    let sor = match &reconciled_receipt.cards[&CardKind::Sor].content {
        csdlc_v2::cards::CardContent::Sor(values) => values,
        _ => panic!("expected SOR card"),
    };
    assert_eq!(sor.follow_ups, vec!["#5411 follow-up"]);
    assert!(store.load_record(issue).unwrap().claim.is_none());
    let doctor = csdlc_v2::diagnose(&store, issue);
    assert_eq!(doctor.phase, Some(LifecyclePhase::ClosedOut));
    assert!(doctor.findings.is_empty());

    let baseline_projection = temp.path().join("baseline-terminal-projection");
    copy_dir_all(&store.issue_dir(issue), &baseline_projection);
    let baseline_receipt = fs::read(store.terminal_receipt_path(issue).unwrap()).unwrap();
    let reconcile = |actor: &str, reason: &str| ReconcileTerminalRequest {
        issue,
        expected_initialization_digest: receipt.initialization_digest.clone(),
        expected_branch: "issue-7".into(),
        expected_worktree: temp.path().to_string_lossy().into_owned(),
        actor: actor.into(),
        reason: reason.into(),
        follow_ups: vec!["#5411 follow-up".into()],
    };

    store
        .reconcile_terminal(reconcile("receipt-writer", "receipt-side attribution"))
        .unwrap();
    let divergent_receipt = fs::read(store.terminal_receipt_path(issue).unwrap()).unwrap();
    fs::remove_dir_all(store.issue_dir(issue)).unwrap();
    copy_dir_all(&baseline_projection, &store.issue_dir(issue));
    fs::write(
        store.terminal_receipt_path(issue).unwrap(),
        &baseline_receipt,
    )
    .unwrap();

    let tracked = store
        .reconcile_terminal(reconcile("tracked-writer", "tracked attribution"))
        .unwrap();
    fs::write(
        store.terminal_receipt_path(issue).unwrap(),
        divergent_receipt,
    )
    .unwrap();
    let preserved = store
        .reconcile_terminal(reconcile("final-writer", "refresh divergent receipt"))
        .unwrap();
    assert_eq!(
        &preserved.audit[..tracked.audit.len()],
        tracked.audit.as_slice()
    );
    assert_eq!(
        store.load_terminal_receipt(issue).unwrap().unwrap().record,
        preserved
    );
    csdlc_v2::NormalizedOutcome::from_v2(&store, issue).unwrap()
}

fn copy_dir_all(source: &std::path::Path, destination: &std::path::Path) {
    fs::create_dir_all(destination).unwrap();
    for entry in fs::read_dir(source).unwrap() {
        let entry = entry.unwrap();
        let target = destination.join(entry.file_name());
        if entry.file_type().unwrap().is_dir() {
            copy_dir_all(&entry.path(), &target);
        } else {
            fs::copy(entry.path(), target).unwrap();
        }
    }
}
use std::fs;
