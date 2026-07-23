use adl_workcell_conductor::{Lane, TaskAssignment, ValidationLane};
use adl_workcell_convergence::*;

const REV: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
const OTHER_REV: &str = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";
const DIGEST: &str = "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc";

fn assignment(issue: u64, wave: usize) -> TaskAssignment {
    TaskAssignment {
        issue,
        claim_id: format!("claim-{issue}"),
        branch: format!("codex/{issue}"),
        worktree: format!(".worktrees/{issue}"),
        source_revision: REV.into(),
        execution_plan_digest: DIGEST.into(),
        dependencies: Vec::new(),
        protected_paths: vec![format!("work/{issue}")],
        write_paths: vec![format!("work/{issue}/out")],
        validation_lanes: vec![ValidationLane {
            name: "convergence-contract".into(),
            argv: vec!["cargo".into(), "test".into()],
        }],
        expected_outputs: vec![format!("work/{issue}/out/result.json")],
        lane: Lane::Parallel,
        wave,
        correlation_id: format!("corr-{issue}"),
    }
}

fn output(issue: u64) -> TaskOutput {
    TaskOutput {
        issue,
        claim_id: format!("claim-{issue}"),
        branch: format!("codex/{issue}"),
        worktree: format!(".worktrees/{issue}"),
        source_revision: REV.into(),
        assignment_digest: DIGEST.into(),
        protected_paths: vec![format!("work/{issue}")],
        write_paths: vec![format!("work/{issue}/out")],
        artifacts: vec![ArtifactRef {
            path: format!("work/{issue}/out/result.json"),
            digest: DIGEST.into(),
        }],
        validation_refs: vec![format!("evidence/{issue}/validation.json")],
        review_refs: vec![format!("evidence/{issue}/review.json")],
        status: OutputStatus::Succeeded,
        changed_assumptions: Vec::new(),
        blockers: Vec::new(),
    }
}

fn input(assignments: Vec<TaskAssignment>, outputs: Vec<TaskOutput>) -> ConvergenceInput {
    ConvergenceInput {
        contract: CONVERGENCE_CONTRACT_VERSION.into(),
        source_revision: REV.into(),
        correlation_seed: "seed".into(),
        authority: ConvergenceAuthority {
            subject: "codex:5502".into(),
            may_decide: true,
            may_create_task: false,
            may_mutate_github: false,
            may_write_filesystem: false,
            may_mutate_lifecycle: false,
            declared_integration_authority: "operator-approved-serial-integration".into(),
        },
        assignments,
        outputs,
        active_claims: Vec::new(),
    }
}

#[test]
fn integrates_successful_outputs_in_conductor_order_with_stable_identity() {
    let first = converge(input(
        vec![assignment(11, 1), assignment(10, 0)],
        vec![output(11), output(10)],
    ))
    .unwrap();
    let second = converge(input(
        vec![assignment(10, 0), assignment(11, 1)],
        vec![output(10), output(11)],
    ))
    .unwrap();

    assert_eq!(first.decision_id, second.decision_id);
    let ConvergenceDecision::Integrate(plan) = first.decision else {
        panic!("expected integration plan");
    };
    assert_eq!(
        plan.steps.iter().map(|step| step.issue).collect::<Vec<_>>(),
        vec![10, 11]
    );
    assert_eq!(first.projection.residual_blockers, Vec::new());
}

#[test]
fn blocks_missing_and_unreviewed_outputs_but_keeps_partial_success_projection() {
    let mut unreviewed = output(12);
    unreviewed.review_refs.clear();
    let decision = converge(input(
        vec![assignment(10, 0), assignment(11, 0), assignment(12, 1)],
        vec![output(10), unreviewed],
    ))
    .unwrap();

    let ConvergenceDecision::Blocked(blocked) = decision.decision else {
        panic!("expected blocked decision");
    };
    assert_eq!(decision.projection.integrated[0].issue, 10);
    assert_eq!(decision.projection.remaining_issues, vec![11, 12]);
    assert!(blocked
        .blockers
        .iter()
        .any(|blocker| blocker.code == BlockerCode::MissingOutput && blocker.issue == 11));
    assert!(blocked
        .blockers
        .iter()
        .any(|blocker| blocker.code == BlockerCode::AmbiguousReview && blocker.issue == 12));
}

#[test]
fn keeps_declared_partial_outputs_visible_with_residual_blockers() {
    let mut partial = output(11);
    partial.status = OutputStatus::Partial;
    partial.blockers.push(Blocker {
        code: BlockerCode::ResidualBlocker,
        issue: 11,
        message: "review evidence still pending".into(),
        evidence_refs: vec!["evidence/11/blocker.json".into()],
    });

    let decision = converge(input(
        vec![assignment(10, 0), assignment(11, 1)],
        vec![output(10), partial],
    ))
    .unwrap();

    let ConvergenceDecision::Blocked(record) = decision.decision else {
        panic!("expected blocked decision");
    };
    assert_eq!(
        decision
            .projection
            .partial_successes
            .iter()
            .map(|step| step.issue)
            .collect::<Vec<_>>(),
        vec![10, 11]
    );
    assert_eq!(decision.projection.integrated[0].issue, 10);
    assert_eq!(decision.projection.remaining_issues, vec![11]);
    assert!(record
        .blockers
        .iter()
        .any(|blocker| blocker.code == BlockerCode::ResidualBlocker && blocker.issue == 11));
}

#[test]
fn blocks_unexplained_non_success_outputs_instead_of_integrating_with_remaining_work() {
    let mut partial_assignment = assignment(11, 1);
    partial_assignment.expected_outputs.clear();
    let mut partial = output(11);
    partial.status = OutputStatus::Partial;
    partial.artifacts.clear();

    let decision = converge(input(
        vec![assignment(10, 0), partial_assignment],
        vec![output(10), partial],
    ))
    .unwrap();

    let ConvergenceDecision::Blocked(record) = decision.decision else {
        panic!("expected unexplained partial output to block");
    };
    assert_eq!(decision.projection.integrated[0].issue, 10);
    assert_eq!(decision.projection.remaining_issues, vec![11]);
    assert!(record
        .blockers
        .iter()
        .any(|blocker| blocker.code == BlockerCode::ResidualBlocker && blocker.issue == 11));
}

#[test]
fn emits_replan_for_changed_assumptions_without_silent_scope_expansion() {
    let mut changed = output(11);
    changed.changed_assumptions.push(ChangedAssumption {
        key: "interface-freeze".into(),
        expected: "digest-a".into(),
        observed: "digest-b".into(),
    });

    let decision = converge(input(
        vec![assignment(10, 0), assignment(11, 1)],
        vec![output(10), changed],
    ))
    .unwrap();

    let ConvergenceDecision::Replan(record) = decision.decision else {
        panic!("expected replan");
    };
    assert_eq!(record.integrated_issues, vec![10]);
    assert_eq!(record.admissible_remaining_work, vec![11]);
    assert_eq!(record.changed_assumptions[0].key, "interface-freeze");
}

#[test]
fn rejects_secret_bearing_changed_assumptions_before_decision() {
    let mut changed = output(11);
    changed.changed_assumptions.push(ChangedAssumption {
        key: "interface-freeze".into(),
        expected: "digest-a".into(),
        observed: "token-value".into(),
    });

    let error = converge(input(vec![assignment(11, 0)], vec![changed])).unwrap_err();

    assert_eq!(error.code, ConvergenceErrorCode::InvalidInput);
}

#[test]
fn rejects_conflicting_duplicate_output_identity() {
    let mut forged = output(10);
    forged.source_revision = OTHER_REV.into();

    let error = converge(input(vec![assignment(10, 0)], vec![output(10), forged])).unwrap_err();

    assert_eq!(error.code, ConvergenceErrorCode::InvalidInput);
}

#[test]
fn blocks_assignment_and_output_that_are_stale_relative_to_envelope_head() {
    let mut request = input(vec![assignment(10, 0)], vec![output(10)]);
    request.source_revision = OTHER_REV.into();

    let decision = converge(request).unwrap();

    let ConvergenceDecision::Blocked(record) = decision.decision else {
        panic!("expected stale output to block integration");
    };
    assert!(decision.projection.integrated.is_empty());
    assert_eq!(decision.projection.remaining_issues, vec![10]);
    assert!(record
        .blockers
        .iter()
        .any(|blocker| blocker.code == BlockerCode::StaleOutput && blocker.issue == 10));
}

#[test]
fn rejects_path_traversal_before_decision() {
    let mut assignment = assignment(10, 0);
    assignment.write_paths = vec!["../outside".into()];

    let error = converge(input(vec![assignment], vec![output(10)])).unwrap_err();

    assert_eq!(error.code, ConvergenceErrorCode::InvalidPath);
}

#[test]
fn rejects_non_repository_local_evidence_refs_before_decision() {
    let mut absolute_ref = output(10);
    absolute_ref.validation_refs = vec!["/tmp/proof.json".into()];
    let error = converge(input(vec![assignment(10, 0)], vec![absolute_ref])).unwrap_err();
    assert_eq!(error.code, ConvergenceErrorCode::InvalidPath);

    let mut traversal_ref = output(10);
    traversal_ref.review_refs = vec!["../outside/review.json".into()];
    let error = converge(input(vec![assignment(10, 0)], vec![traversal_ref])).unwrap_err();
    assert_eq!(error.code, ConvergenceErrorCode::InvalidPath);
}

#[test]
fn rejects_non_repository_local_task_blocker_evidence_refs_before_decision() {
    let mut task_output = output(10);
    task_output.blockers.push(Blocker {
        code: BlockerCode::ResidualBlocker,
        issue: 10,
        message: "manual inspection required".into(),
        evidence_refs: vec!["https://example.test/proof.json".into()],
    });

    let error = converge(input(vec![assignment(10, 0)], vec![task_output])).unwrap_err();

    assert_eq!(error.code, ConvergenceErrorCode::InvalidInput);
}

#[test]
fn blocks_overlapping_active_claims() {
    let mut request = input(vec![assignment(10, 0)], vec![output(10)]);
    request.active_claims.push(ActiveClaim {
        issue: 5500,
        claim_id: "claim-5500".into(),
        protected_paths: vec!["work/10/out/dashboard".into()],
    });

    let decision = converge(request).unwrap();

    let ConvergenceDecision::Blocked(record) = decision.decision else {
        panic!("expected blocked decision");
    };
    assert!(record
        .blockers
        .iter()
        .any(|blocker| blocker.code == BlockerCode::PathOverlap));
}

#[test]
fn blocks_hidden_mutation_authority() {
    let mut request = input(vec![assignment(10, 0)], vec![output(10)]);
    request.authority.may_mutate_github = true;

    let decision = converge(request).unwrap();

    let ConvergenceDecision::Blocked(record) = decision.decision else {
        panic!("expected blocked decision");
    };
    assert!(record
        .blockers
        .iter()
        .any(|blocker| blocker.code == BlockerCode::HiddenMutationAuthority));
}

#[test]
fn blocks_out_of_scope_artifacts() {
    let mut task_output = output(10);
    task_output.artifacts.push(ArtifactRef {
        path: "work/10/extra.json".into(),
        digest: DIGEST.into(),
    });

    let decision = converge(input(vec![assignment(10, 0)], vec![task_output])).unwrap();

    let ConvergenceDecision::Blocked(record) = decision.decision else {
        panic!("expected blocked decision");
    };
    assert!(record
        .blockers
        .iter()
        .any(|blocker| blocker.code == BlockerCode::OutOfScopeArtifact));
}
