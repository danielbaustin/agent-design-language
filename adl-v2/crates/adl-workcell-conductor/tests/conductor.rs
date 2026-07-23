use adl_workcell_conductor::{
    plan, CardKind, ClaimSnapshot, ConductorInput, ExecutionPlanSnapshot, IssueSnapshot, Lane,
    RefusalCode, SerializedGate, ValidationLane, CONDUCTOR_CONTRACT_VERSION,
};
use std::collections::BTreeSet;

fn cards() -> BTreeSet<CardKind> {
    [
        CardKind::Sip,
        CardKind::Stp,
        CardKind::Spp,
        CardKind::Vpp,
        CardKind::Srp,
        CardKind::Sor,
    ]
    .into_iter()
    .collect()
}

fn issue(id: u64, dependencies: Vec<u64>, path: &str) -> IssueSnapshot {
    IssueSnapshot {
        issue: id,
        source_revision: format!("revision-{id}"),
        ready: true,
        cards: cards(),
        claim: Some(ClaimSnapshot {
            id: format!("claim-{id}"),
            owner: format!("owner-{id}"),
            branch: format!("codex/{id}"),
            worktree: format!("/worktrees/{id}"),
            purpose: String::from("bounded implementation"),
            expires_unix_seconds: 2_000,
            protected_paths: vec![path.into()],
        }),
        dependencies,
        write_paths: vec![format!("{path}/src")],
        validation_lanes: vec![ValidationLane {
            name: String::from("focused"),
            argv: vec![String::from("cargo"), String::from("test")],
        }],
        expected_outputs: vec![format!("evidence/{id}.json")],
    }
}

fn input(issues: Vec<IssueSnapshot>) -> ConductorInput {
    ConductorInput {
        contract: String::from(CONDUCTOR_CONTRACT_VERSION),
        source_revision: String::from("source-revision"),
        observed_unix_seconds: 1_000,
        correlation_seed: String::from("seed"),
        max_writable_assignments: 8,
        active_writable_assignments: 0,
        known_validation_lanes: [String::from("focused")].into_iter().collect(),
        resolved_dependencies: [9].into_iter().collect(),
        execution_plan: ExecutionPlanSnapshot {
            contract: String::from("adl.execution-plan.v1"),
            source_digest: String::from("plan-digest"),
            node_ids: vec![String::from("node-a")],
        },
        issues,
    }
}

#[test]
fn emits_canonical_parallel_and_serial_waves() {
    let decision = plan(input(vec![
        issue(3, vec![1, 2], "crates/three"),
        issue(2, vec![9], "crates/two"),
        issue(1, vec![9], "crates/one"),
    ]))
    .unwrap();
    let assignments = decision.plan.assignments;
    assert_eq!(
        assignments.iter().map(|a| a.issue).collect::<Vec<_>>(),
        [1, 2, 3]
    );
    assert_eq!(assignments[0].lane, Lane::Parallel);
    assert_eq!(assignments[1].lane, Lane::Parallel);
    assert_eq!(assignments[2].lane, Lane::Serial);
    assert_eq!(assignments[2].wave, 1);
    assert_eq!(
        decision.plan.serialized_gates,
        [
            SerializedGate::Review,
            SerializedGate::Publication,
            SerializedGate::Merge,
            SerializedGate::PostMergeValidation,
            SerializedGate::Closeout,
        ]
    );
}

#[test]
fn ordering_and_identity_are_deterministic() {
    let left = plan(input(vec![
        issue(2, vec![9], "crates/two"),
        issue(1, vec![9], "crates/one"),
    ]))
    .unwrap();
    let right = plan(input(vec![
        issue(1, vec![9], "crates/one"),
        issue(2, vec![9], "crates/two"),
    ]))
    .unwrap();
    assert_eq!(left, right);
    assert_eq!(
        serde_json::to_vec(&left).unwrap(),
        serde_json::to_vec(&right).unwrap()
    );
}

#[test]
fn rejects_dependency_cycles() {
    let refusal = plan(input(vec![
        issue(1, vec![2], "crates/one"),
        issue(2, vec![1], "crates/two"),
    ]))
    .unwrap_err();
    assert_eq!(refusal.code, RefusalCode::DependencyCycle);
}

#[test]
fn rejects_unresolved_dependencies() {
    let refusal = plan(input(vec![issue(1, vec![99], "crates/one")])).unwrap_err();
    assert_eq!(refusal.code, RefusalCode::UnresolvedDependency);
}

#[test]
fn rejects_missing_cards_claims_and_stale_claims() {
    let mut missing_card = issue(1, vec![9], "crates/one");
    missing_card.cards.remove(&CardKind::Sor);
    assert_eq!(
        plan(input(vec![missing_card])).unwrap_err().code,
        RefusalCode::MissingCards
    );

    let mut missing_claim = issue(1, vec![9], "crates/one");
    missing_claim.claim = None;
    assert_eq!(
        plan(input(vec![missing_claim])).unwrap_err().code,
        RefusalCode::MissingClaim
    );

    let mut stale = issue(1, vec![9], "crates/one");
    stale.claim.as_mut().unwrap().expires_unix_seconds = 1_000;
    assert_eq!(
        plan(input(vec![stale])).unwrap_err().code,
        RefusalCode::StaleClaim
    );
}

#[test]
fn rejects_unknown_or_malformed_validation_lanes() {
    let mut unknown = issue(1, vec![9], "crates/one");
    unknown.validation_lanes[0].name = String::from("mystery");
    assert_eq!(
        plan(input(vec![unknown])).unwrap_err().code,
        RefusalCode::UnknownValidationLane
    );
}

#[test]
fn rejects_exact_and_segment_prefix_path_collisions() {
    let exact = plan(input(vec![
        issue(1, vec![9], "crates/shared"),
        issue(2, vec![9], "crates/shared"),
    ]))
    .unwrap_err();
    assert_eq!(exact.code, RefusalCode::PathCollision);

    let prefix = plan(input(vec![
        issue(1, vec![9], "crates/shared"),
        issue(2, vec![9], "crates/shared/nested"),
    ]))
    .unwrap_err();
    assert_eq!(prefix.code, RefusalCode::PathCollision);

    plan(input(vec![
        issue(1, vec![9], "crates/app"),
        issue(2, vec![9], "crates/apple"),
    ]))
    .unwrap();
}

#[test]
fn rejects_write_paths_outside_the_active_claim() {
    let mut escaped = issue(1, vec![9], "crates/owned");
    escaped.write_paths = vec![String::from("crates/unclaimed/src")];
    let refusal = plan(input(vec![escaped])).unwrap_err();
    assert_eq!(refusal.code, RefusalCode::AmbiguousAuthority);
    assert!(refusal.message.contains("outside the active claim"));
}

#[test]
fn rejects_absolute_parent_and_dot_paths() {
    for path in ["/absolute", "../escape", "crates/../escape", "./crates"] {
        let refusal = plan(input(vec![issue(1, vec![9], path)])).unwrap_err();
        assert_eq!(refusal.code, RefusalCode::InvalidPath, "{path}");
    }
}

#[test]
fn rejects_wip_overflow_and_ambiguous_claim_authority() {
    let mut limited = input(vec![issue(1, vec![9], "crates/one")]);
    limited.max_writable_assignments = 1;
    limited.active_writable_assignments = 1;
    assert_eq!(plan(limited).unwrap_err().code, RefusalCode::WipOverflow);

    let mut ambiguous = issue(1, vec![9], "crates/one");
    ambiguous.claim.as_mut().unwrap().owner.clear();
    assert_eq!(
        plan(input(vec![ambiguous])).unwrap_err().code,
        RefusalCode::AmbiguousAuthority
    );
}

#[test]
fn correlation_changes_with_seed_or_content() {
    let first = plan(input(vec![issue(1, vec![9], "crates/one")])).unwrap();
    let mut changed_seed = input(vec![issue(1, vec![9], "crates/one")]);
    changed_seed.correlation_seed = String::from("other-seed");
    let second = plan(changed_seed).unwrap();
    assert_ne!(
        first.plan.assignments[0].correlation_id,
        second.plan.assignments[0].correlation_id
    );
}
