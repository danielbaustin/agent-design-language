use super::*;

#[test]
fn runtime_v2_anti_harm_trajectory_constraint_packet_validates() {
    let packet = anti_harm_trajectory_constraint_packet().expect("packet");

    validate_anti_harm_trajectory_constraint_packet(&packet).expect("packet should validate");

    assert_eq!(packet.constraints.len(), 4);
    assert_eq!(packet.synthetic_scenarios.len(), 1);
    assert_eq!(packet.decisions.len(), 2);
}

#[test]
fn runtime_v2_anti_harm_trajectory_constraint_packet_has_required_modes_and_decisions() {
    let packet = anti_harm_trajectory_constraint_packet().expect("packet");

    assert!(packet
        .constraints
        .iter()
        .any(|constraint| constraint.harm_mode == "decomposed"));
    assert!(packet
        .constraints
        .iter()
        .any(|constraint| constraint.harm_mode == "delegated"));
    assert!(packet
        .constraints
        .iter()
        .any(|constraint| constraint.harm_mode == "delayed"));
    assert!(packet
        .constraints
        .iter()
        .any(|constraint| constraint.harm_mode == "disguised"));
    assert!(packet
        .decisions
        .iter()
        .any(|decision| decision.decision_kind == "deny"));
    assert!(packet
        .decisions
        .iter()
        .any(|decision| decision.decision_kind == "escalate"));
}

#[test]
fn runtime_v2_anti_harm_trajectory_constraint_json_materialization_is_stable() {
    let mut packet = anti_harm_trajectory_constraint_packet().expect("packet");
    packet.constraints.reverse();
    packet.synthetic_scenarios.reverse();
    packet.decisions.reverse();
    packet.constraints[0].evidence_field_refs.reverse();
    packet.decisions[0].trace_evidence_refs.reverse();

    let first = anti_harm_trajectory_constraint_json_bytes(&packet).expect("first bytes");
    let second = anti_harm_trajectory_constraint_json_bytes(&packet).expect("second bytes");

    assert_eq!(first, second);

    let json = String::from_utf8(first).expect("utf8");
    let decomposed_index = json.find("constraint-decomposed-harm").expect("decomposed");
    let disguised_index = json.find("constraint-disguised-harm").expect("disguised");
    assert!(decomposed_index < disguised_index);
}

#[test]
fn runtime_v2_anti_harm_trajectory_constraint_rejects_single_step_scenario() {
    let mut packet = anti_harm_trajectory_constraint_packet().expect("packet");
    packet.synthetic_scenarios[0].individually_benign_trace_refs =
        vec![packet.synthetic_scenarios[0].individually_benign_trace_refs[0].clone()];

    let err = validate_anti_harm_trajectory_constraint_packet(&packet)
        .expect_err("single-step scenario should fail")
        .to_string();

    assert!(err.contains("cross-step trajectory"));
}

#[test]
fn runtime_v2_anti_harm_trajectory_constraint_rejects_duplicate_cross_step_trace_refs() {
    let mut packet = anti_harm_trajectory_constraint_packet().expect("packet");
    let duplicated = packet.synthetic_scenarios[0].individually_benign_trace_refs[0].clone();
    packet.synthetic_scenarios[0].individually_benign_trace_refs =
        vec![duplicated.clone(), duplicated.clone(), duplicated];

    let err = validate_anti_harm_trajectory_constraint_packet(&packet)
        .expect_err("duplicate trace refs should not satisfy cross-step proof")
        .to_string();

    assert!(err.contains("distinct cross-step trace refs"));
}

#[test]
fn runtime_v2_anti_harm_trajectory_constraint_rejects_missing_required_harm_mode() {
    let mut packet = anti_harm_trajectory_constraint_packet().expect("packet");
    packet
        .constraints
        .retain(|constraint| constraint.harm_mode != "disguised");

    let err = validate_anti_harm_trajectory_constraint_packet(&packet)
        .expect_err("missing required harm mode should fail")
        .to_string();

    assert!(err.contains("required harm mode"));
}

#[test]
fn runtime_v2_anti_harm_trajectory_constraint_rejects_missing_deny_or_escalate_record() {
    let mut packet = anti_harm_trajectory_constraint_packet().expect("packet");
    packet
        .decisions
        .retain(|decision| decision.decision_kind != "deny");

    let err = validate_anti_harm_trajectory_constraint_packet(&packet)
        .expect_err("both deny and escalate records are required")
        .to_string();

    assert!(err.contains("both deny and escalate records"));
}

#[test]
fn runtime_v2_anti_harm_trajectory_constraint_rejects_unknown_trajectory_finding_refs() {
    let mut packet = anti_harm_trajectory_constraint_packet().expect("packet");
    packet.decisions[0].trajectory_finding_refs = vec!["unknown-finding".to_string()];

    let err = validate_anti_harm_trajectory_constraint_packet(&packet)
        .expect_err("unknown trajectory finding refs should fail")
        .to_string();

    assert!(err.contains("trajectory_finding_refs"));
}

#[test]
fn runtime_v2_anti_harm_trajectory_constraint_rejects_unknown_upstream_refs() {
    let mut packet = anti_harm_trajectory_constraint_packet().expect("packet");
    packet.synthetic_scenarios[0].supporting_outcome_linkage_refs =
        vec!["outcome-linkage:unknown-outcome".to_string()];

    let scenario_err = validate_anti_harm_trajectory_constraint_packet(&packet)
        .expect_err("synthetic scenario must cite known outcome-linkage refs")
        .to_string();
    assert!(scenario_err.contains("known WP-05 outcome-linkage examples"));

    let mut packet = anti_harm_trajectory_constraint_packet().expect("packet");
    packet.decisions[0].trace_evidence_refs = vec!["trace:unknown-trace".to_string()];

    let decision_err = validate_anti_harm_trajectory_constraint_packet(&packet)
        .expect_err("decision records must cite known trace refs")
        .to_string();
    assert!(decision_err.contains("known WP-04 trace examples"));
}

#[test]
fn runtime_v2_anti_harm_trajectory_constraint_rejects_non_operational_boundary_drift() {
    let mut packet = anti_harm_trajectory_constraint_packet().expect("packet");
    packet.synthetic_scenarios[0].claim_boundary =
        "Proof only; operator can improvise the rest.".to_string();

    let err = validate_anti_harm_trajectory_constraint_packet(&packet)
        .expect_err("synthetic scenario boundary must stay non-operational")
        .to_string();

    assert!(err.contains("synthetic and non-operational"));
}
