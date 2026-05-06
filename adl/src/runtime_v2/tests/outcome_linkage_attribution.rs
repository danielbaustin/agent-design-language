use super::*;

#[test]
fn runtime_v2_outcome_linkage_accepts_required_examples() {
    let examples = outcome_linkage_required_examples();

    validate_outcome_linkage_examples(&examples).expect("required examples should validate");
}

#[test]
fn runtime_v2_outcome_linkage_canonical_materialization_is_stable() {
    let mut record = outcome_linkage_required_examples()
        .into_iter()
        .find(|example| example.example_kind == OutcomeLinkageExampleKind::Contested)
        .expect("contested example")
        .record;

    record.attribution.policy_contribution_refs = vec![
        "policy:z-contested".to_string(),
        "policy:a-contested".to_string(),
    ];
    record.linked_outcomes[0].uncertainty_refs = vec![
        "review:z-contested".to_string(),
        "review:a-contested".to_string(),
    ];

    let first = canonical_outcome_linkage_json_bytes(&record).expect("first canonical bytes");
    let second = canonical_outcome_linkage_json_bytes(&record).expect("second canonical bytes");

    assert_eq!(first, second);

    let canonical = String::from_utf8(first).expect("utf8");
    let a_policy = canonical.find("policy:a-contested").expect("a policy");
    let z_policy = canonical.find("policy:z-contested").expect("z policy");
    assert!(a_policy < z_policy);
}

#[test]
fn runtime_v2_outcome_linkage_rejects_false_certainty_for_unknown_outcomes() {
    let mut record = outcome_linkage_required_examples()
        .into_iter()
        .find(|example| example.example_kind == OutcomeLinkageExampleKind::Unknown)
        .expect("unknown example")
        .record;

    record.linked_outcomes[0].causal_posture = "evidenced".to_string();
    record.linked_outcomes[0].evidence_refs = vec!["artifact:spoofed-proof".to_string()];

    let err = validate_outcome_linkage_record(&record)
        .expect_err("unknown outcomes must not claim certainty")
        .to_string();

    assert!(err.contains("must not collapse uncertainty"));
}

#[test]
fn runtime_v2_outcome_linkage_requires_delegation_lineage_for_delegated_outcomes() {
    let mut record = outcome_linkage_required_examples()
        .into_iter()
        .find(|example| example.example_kind == OutcomeLinkageExampleKind::Contested)
        .expect("contested example")
        .record;

    record.attribution.delegate_trace_ref = Some("trace:wrong-child".to_string());

    let err = validate_outcome_linkage_record(&record)
        .expect_err("delegated linkage must preserve lineage")
        .to_string();

    assert!(err.contains("must retain the source trace delegation lineage"));
}

#[test]
fn runtime_v2_outcome_linkage_rejects_spoofed_delegation_refs_on_non_delegated_trace() {
    let mut record = outcome_linkage_required_examples()
        .into_iter()
        .find(|example| example.example_kind == OutcomeLinkageExampleKind::Known)
        .expect("known example")
        .record;

    record.attribution.delegated_by_trace_ref = Some("trace:spoofed-parent".to_string());
    record.attribution.delegate_trace_ref = Some("trace:spoofed-child".to_string());

    let err = validate_outcome_linkage_record(&record)
        .expect_err("non-delegated linkage must reject spoofed delegation refs")
        .to_string();

    assert!(err.contains("delegation refs require a delegated source trace"));
}

#[test]
fn runtime_v2_outcome_linkage_rejects_contested_outcomes_without_rebuttal_refs() {
    let mut record = outcome_linkage_required_examples()
        .into_iter()
        .find(|example| example.example_kind == OutcomeLinkageExampleKind::Contested)
        .expect("contested example")
        .record;

    record.linked_outcomes[0].rebuttal_refs.clear();

    let err = validate_outcome_linkage_record(&record)
        .expect_err("contested outcomes require rebuttals")
        .to_string();

    assert!(err.contains("contested outcomes require rebuttal_refs"));
}

#[test]
fn runtime_v2_outcome_linkage_requires_review_path() {
    let mut record = outcome_linkage_required_examples()
        .into_iter()
        .find(|example| example.example_kind == OutcomeLinkageExampleKind::Known)
        .expect("known example")
        .record;

    record.review_refs.review_packet_refs.clear();
    record.review_refs.trajectory_review_refs.clear();
    record.review_refs.metric_refs.clear();
    record.review_refs.challenge_ref = None;

    let err = validate_outcome_linkage_record(&record)
        .expect_err("missing review path should fail")
        .to_string();

    assert!(err.contains("must preserve a review packet, trajectory, metric, or challenge path"));
}

#[test]
fn runtime_v2_outcome_linkage_rejects_host_paths_in_refs() {
    let mut record = outcome_linkage_required_examples()
        .into_iter()
        .find(|example| example.example_kind == OutcomeLinkageExampleKind::Known)
        .expect("known example")
        .record;

    record.linked_outcomes[0].evidence_refs = vec!["/Users/daniel/private.txt".to_string()];

    let err = validate_outcome_linkage_record(&record)
        .expect_err("host paths must be rejected")
        .to_string();

    assert!(err.contains("evidence_refs"));
    assert!(!err.contains("/Users/daniel/private.txt"));
}

#[test]
fn runtime_v2_outcome_linkage_rejects_windows_and_file_uri_host_paths_in_refs() {
    let mut windows_record = outcome_linkage_required_examples()
        .into_iter()
        .find(|example| example.example_kind == OutcomeLinkageExampleKind::Known)
        .expect("known example")
        .record;
    windows_record.linked_outcomes[0].evidence_refs =
        vec!["C:/Users/daniel/private.txt".to_string()];

    let windows_err = validate_outcome_linkage_record(&windows_record)
        .expect_err("windows-style host paths must be rejected")
        .to_string();
    assert!(windows_err.contains("evidence_refs"));
    assert!(!windows_err.contains("C:/Users/daniel/private.txt"));

    let mut uri_record = outcome_linkage_required_examples()
        .into_iter()
        .find(|example| example.example_kind == OutcomeLinkageExampleKind::Known)
        .expect("known example")
        .record;
    uri_record.linked_outcomes[0].evidence_refs =
        vec!["file:///Users/daniel/private.txt".to_string()];

    let uri_err = validate_outcome_linkage_record(&uri_record)
        .expect_err("file uris must be rejected")
        .to_string();
    assert!(uri_err.contains("evidence_refs"));
    assert!(!uri_err.contains("file:///Users/daniel/private.txt"));
}
