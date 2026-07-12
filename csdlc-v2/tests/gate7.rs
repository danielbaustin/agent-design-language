use csdlc_v2::{
    classify_readiness, closeout_issue, CheckConclusion, CheckObservation, CheckRequirement,
    ConflictState, PostPublicationFinding, ReadinessRequest, RemoteReviewState, Store,
    TerminalDisposition, TerminalObservation,
};

fn request() -> ReadinessRequest {
    ReadinessRequest {
        schema: "csdlc.readiness_request.v1".into(),
        issue: 7,
        expected_generation: 4,
        expected_digest: "digest".into(),
        claim_id: "claim".into(),
        actor: "shepherd".into(),
        pull_request: 70,
        head_sha: "abc".into(),
        required_checks: vec!["fast".into(), "contract".into()],
        require_review: true,
        checks: vec![
            CheckObservation {
                name: "fast".into(),
                requirement: CheckRequirement::Required,
                conclusion: CheckConclusion::Success,
                details_url: None,
            },
            CheckObservation {
                name: "contract".into(),
                requirement: CheckRequirement::Required,
                conclusion: CheckConclusion::Success,
                details_url: None,
            },
            CheckObservation {
                name: "optional-soak".into(),
                requirement: CheckRequirement::Optional,
                conclusion: CheckConclusion::Skipped,
                details_url: None,
            },
        ],
        review_state: RemoteReviewState::Approved,
        conflict_state: ConflictState::Clean,
        post_publication_findings: vec![],
    }
}

#[test]
fn green_required_truth_is_ready_without_promoting_optional_skips() {
    let report = classify_readiness(&request()).unwrap();
    assert!(report.ready);
    assert_eq!(report.optional_non_success, vec!["optional-soak"]);
}

#[test]
fn pending_failed_and_unobserved_required_checks_are_distinct() {
    let mut value = request();
    value.checks[0].conclusion = CheckConclusion::Pending;
    value.checks[1].conclusion = CheckConclusion::Failure;
    value.required_checks.push("missing".into());
    let report = classify_readiness(&value).unwrap();
    assert_eq!(report.required_pending, vec!["fast", "missing:unobserved"]);
    assert_eq!(report.required_failed, vec!["contract"]);
    assert!(!report.ready);
}

#[test]
fn requested_changes_remain_routed_and_block_readiness() {
    let mut value = request();
    value.review_state = RemoteReviewState::ChangesRequested;
    value
        .post_publication_findings
        .push(PostPublicationFinding {
            id: "review-1".into(),
            reviewer: "reviewer".into(),
            summary: "repair edge case".into(),
            changes_requested: true,
            active: true,
            route: "pull_request:70".into(),
        });
    let report = classify_readiness(&value).unwrap();
    assert!(report
        .blockers
        .contains(&"post_publication_changes_requested".into()));
    assert!(report
        .blockers
        .contains(&"required_review_not_approved".into()));
}

#[test]
fn unknown_conflict_or_review_truth_never_becomes_green() {
    let mut value = request();
    value.review_state = RemoteReviewState::Unknown;
    value.conflict_state = ConflictState::Unknown;
    let report = classify_readiness(&value).unwrap();
    assert!(!report.ready);
    assert_eq!(report.blockers.len(), 2);
}

#[test]
fn false_terminal_observations_fail_before_store_mutation() {
    let temp = tempfile::tempdir().unwrap();
    let store = Store::new(temp.path());
    let invalid = TerminalObservation {
        schema: "csdlc.terminal_observation.v1".into(),
        issue: 7,
        expected_generation: 1,
        expected_digest: "digest".into(),
        claim_id: "claim".into(),
        actor: "closer".into(),
        pull_request: Some(70),
        disposition: TerminalDisposition::Merged,
        observed_sha: None,
        observed_state: "open".into(),
        approved_no_pr_reason: None,
        receipt_path: "receipt.json".into(),
    };
    assert!(closeout_issue(&store, invalid).is_err());
    assert!(!temp.path().join(".csdlc").exists());
}

#[test]
fn no_pr_terminal_requires_explicit_approval() {
    let temp = tempfile::tempdir().unwrap();
    let store = Store::new(temp.path());
    let invalid = TerminalObservation {
        schema: "csdlc.terminal_observation.v1".into(),
        issue: 7,
        expected_generation: 1,
        expected_digest: "digest".into(),
        claim_id: "claim".into(),
        actor: "closer".into(),
        pull_request: None,
        disposition: TerminalDisposition::ClosedNoPr,
        observed_sha: None,
        observed_state: "closed_no_pr".into(),
        approved_no_pr_reason: None,
        receipt_path: "receipt.json".into(),
    };
    assert!(closeout_issue(&store, invalid).is_err());
}

#[test]
fn closed_unmerged_terminal_requires_exact_head_sha() {
    let temp = tempfile::tempdir().unwrap();
    let store = Store::new(temp.path());
    let invalid = TerminalObservation {
        schema: "csdlc.terminal_observation.v1".into(),
        issue: 7,
        expected_generation: 1,
        expected_digest: "digest".into(),
        claim_id: "claim".into(),
        actor: "closer".into(),
        pull_request: Some(70),
        disposition: TerminalDisposition::ClosedUnmerged,
        observed_sha: None,
        observed_state: "closed".into(),
        approved_no_pr_reason: None,
        receipt_path: "receipt.json".into(),
    };
    assert!(closeout_issue(&store, invalid).is_err());
    assert!(!temp.path().join(".csdlc").exists());
}

#[test]
fn schemas_cover_readiness_and_terminal_truth() {
    let bundle = csdlc_v2::public_schema_bundle();
    assert!(bundle.get("readiness_request").is_some());
    assert!(bundle.get("readiness_report").is_some());
    assert!(bundle.get("terminal_observation").is_some());
}

#[test]
fn prune_guard_requires_exact_topology_and_clean_worktree() {
    let temp = tempfile::tempdir().unwrap();
    let git = |args: &[&str]| {
        let output = std::process::Command::new("git")
            .current_dir(temp.path())
            .args(args)
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
    };
    git(&["init", "-b", "terminal-7"]);
    git(&["config", "user.email", "test@example.invalid"]);
    git(&["config", "user.name", "C-SDLC Test"]);
    std::fs::write(temp.path().join("tracked"), "clean").unwrap();
    git(&["add", "tracked"]);
    git(&["commit", "-m", "fixture"]);
    let path = temp.path().to_string_lossy().into_owned();
    csdlc_v2::readiness::validate_prune_surface(temp.path(), "terminal-7", &path).unwrap();
    assert!(csdlc_v2::readiness::validate_prune_surface(temp.path(), "wrong", &path).is_err());
    std::fs::write(temp.path().join("dirty"), "dirty").unwrap();
    assert!(csdlc_v2::readiness::validate_prune_surface(temp.path(), "terminal-7", &path).is_err());
}
