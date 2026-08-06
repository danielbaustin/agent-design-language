use csdlc_v2::{
    classify_readiness, CheckConclusion, CheckObservation, CheckRequirement, ConflictState,
    PostPublicationFinding, ReadinessRequest, RemoteReviewState,
};

fn request() -> ReadinessRequest {
    ReadinessRequest {
        schema: "csdlc.readiness_request.v1".into(),
        issue: 7,
        expected_generation: 4,
        expected_digest: "digest".into(),
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
fn public_schemas_keep_readiness_but_drop_terminal_mutation_contracts() {
    let bundle = csdlc_v2::public_schema_bundle();
    assert!(bundle.get("readiness_request").is_some());
    assert!(bundle.get("readiness_report").is_some());
    assert!(bundle.get("terminal_receipt").is_some());
    assert!(bundle.get("terminal_observation").is_none());
    assert!(bundle.get("terminal_reconciliation_request").is_none());
}
