use super::*;

#[test]
fn runtime_v2_moral_trace_schema_accepts_required_examples() {
    let examples = moral_trace_required_examples();

    validate_moral_trace_examples(&examples).expect("required examples should validate");
}

#[test]
fn runtime_v2_moral_trace_schema_canonical_materialization_is_stable() {
    let mut trace = moral_trace_required_examples()
        .into_iter()
        .find(|example| example.example_kind == MoralTraceExampleKind::Delegation)
        .expect("delegation example")
        .trace;

    trace.visibility.reviewer_evidence_refs = vec![
        "artifact:z-review".to_string(),
        "artifact:a-review".to_string(),
    ];
    trace.outcome.outcome_evidence_refs = vec![
        "artifact:z-outcome".to_string(),
        "artifact:a-outcome".to_string(),
    ];

    let first = canonical_moral_trace_json_bytes(&trace).expect("first canonical bytes");
    let second = canonical_moral_trace_json_bytes(&trace).expect("second canonical bytes");

    assert_eq!(first, second);

    let canonical = String::from_utf8(first).expect("utf8");
    let z_index = canonical
        .find("artifact:z-review")
        .expect("z review present");
    let a_index = canonical
        .find("artifact:a-review")
        .expect("a review present");
    assert!(a_index < z_index);
}

#[test]
fn runtime_v2_moral_trace_schema_rejects_public_private_state_exposure() {
    let mut trace = moral_trace_required_examples()
        .into_iter()
        .find(|example| example.example_kind == MoralTraceExampleKind::Refusal)
        .expect("refusal example")
        .trace;

    trace.visibility.public_summary =
        Some("This summary reveals private_state diagnostic details.".to_string());
    trace.visibility.public_evidence_refs =
        vec!["private_state:diagnostic-classification".to_string()];

    let err = validate_moral_trace_record(&trace)
        .expect_err("public exposure of private-state markers should fail")
        .to_string();

    assert!(err.contains("moral_trace.visibility.public_summary"));
    assert!(!err.contains("private_state:diagnostic-classification"));
}

#[test]
fn runtime_v2_moral_trace_schema_rejects_outcome_and_attribution_drift() {
    let mut trace = moral_trace_required_examples()
        .into_iter()
        .find(|example| example.example_kind == MoralTraceExampleKind::Delegation)
        .expect("delegation example")
        .trace;

    trace.attribution.delegate_trace_ref = Some("trace:wrong-child".to_string());

    let err = validate_moral_trace_record(&trace)
        .expect_err("delegation drift should fail")
        .to_string();

    assert!(err.contains("moral_trace.attribution delegate_trace_ref"));
}

#[test]
fn runtime_v2_moral_trace_schema_rejects_spoofed_delegation_refs_on_non_delegated_trace() {
    let mut trace = moral_trace_required_examples()
        .into_iter()
        .find(|example| example.example_kind == MoralTraceExampleKind::OrdinaryAction)
        .expect("ordinary action example")
        .trace;

    trace.attribution.delegated_by_trace_ref = Some("trace:spoofed-parent".to_string());
    trace.attribution.delegate_trace_ref = Some("trace:spoofed-child".to_string());

    let err = validate_moral_trace_record(&trace)
        .expect_err("non-delegated traces must reject spoofed delegation refs")
        .to_string();

    assert!(err.contains("delegation refs require an active delegation context"));
}

#[test]
fn runtime_v2_moral_trace_schema_requires_a_reviewer_path() {
    let mut trace = moral_trace_required_examples()
        .into_iter()
        .find(|example| example.example_kind == MoralTraceExampleKind::OrdinaryAction)
        .expect("ordinary action example")
        .trace;

    trace.visibility.reviewer_evidence_refs.clear();
    trace.visibility.governance_evidence_refs.clear();
    trace.review_refs.review_packet_refs.clear();
    trace.review_refs.challenge_ref = None;

    let err = validate_moral_trace_record(&trace)
        .expect_err("missing reviewer path should fail")
        .to_string();

    assert!(err.contains("reviewer or governance review path"));
}

#[test]
fn runtime_v2_moral_trace_schema_rejects_challenged_event_without_trace_challenge_ref() {
    let mut trace = moral_trace_required_examples()
        .into_iter()
        .find(|example| example.example_kind == MoralTraceExampleKind::DeferredDecision)
        .expect("deferred example")
        .trace;

    trace.moral_event.event_kind = "challenged".to_string();
    trace.outcome.outcome_kind = "challenged".to_string();
    trace.review_refs.challenge_ref = None;

    let err = validate_moral_trace_record(&trace)
        .expect_err("challenged traces must keep a challenge ref")
        .to_string();

    assert!(err.contains("challenge_ref"));
}

#[test]
fn runtime_v2_moral_trace_schema_rejects_windows_host_paths_in_public_surfaces() {
    let mut trace = moral_trace_required_examples()
        .into_iter()
        .find(|example| example.example_kind == MoralTraceExampleKind::OrdinaryAction)
        .expect("ordinary action example")
        .trace;

    trace.visibility.public_evidence_refs = vec!["C:\\Users\\daniel\\secret.txt".to_string()];

    let err = validate_moral_trace_record(&trace)
        .expect_err("windows host paths must be rejected from public refs")
        .to_string();

    assert!(err.contains("public_evidence_refs"));
    assert!(!err.contains("C:\\Users\\daniel\\secret.txt"));
}
