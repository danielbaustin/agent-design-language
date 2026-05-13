use super::*;

#[test]
fn runtime_v2_theory_of_mind_foundation_contract_is_stable() {
    let packet =
        runtime_v2_theory_of_mind_foundation_contract().expect("theory-of-mind foundation packet");
    packet.validate().expect("valid theory-of-mind packet");

    assert_eq!(
        packet.schema_version,
        RUNTIME_V2_THEORY_OF_MIND_FOUNDATION_SCHEMA
    );
    assert_eq!(packet.demo_id, "memory_tom_evidence_demo");
    assert_eq!(packet.milestone, "v0.91.1");
    assert_eq!(packet.wp, "WP-08");
    assert_eq!(packet.agent_models.len(), 4);
    assert_eq!(packet.update_events.len(), 4);
    assert_eq!(packet.fixture_matrix.len(), 4);
    assert!(packet
        .non_claims
        .iter()
        .any(|claim| claim.contains("mind-reading")));
}

#[test]
fn runtime_v2_theory_of_mind_foundation_matches_golden_fixture() {
    let packet =
        runtime_v2_theory_of_mind_foundation_contract().expect("theory-of-mind foundation packet");
    let json =
        String::from_utf8(packet.pretty_json_bytes().expect("theory-of-mind json")).expect("utf8");
    let expected = include_str!(
        "../../../tests/fixtures/runtime_v2/theory_of_mind/theory_of_mind_foundation.json"
    )
    .trim_end()
    .to_string();

    if json != expected {
        let dump_path = common::unique_temp_path("theory-of-mind-foundation-fixture");
        std::fs::write(&dump_path, &json).expect("write fixture dump");
        panic!(
            "theory-of-mind golden fixture drifted; actual packet dumped to {}",
            dump_path.display()
        );
    }
}

#[test]
fn runtime_v2_theory_of_mind_foundation_validation_rejects_shape_drift() {
    let mut packet =
        runtime_v2_theory_of_mind_foundation_contract().expect("theory-of-mind foundation packet");
    packet.demo_id = "D99".to_string();
    assert!(packet
        .validate()
        .expect_err("changing demo route should fail")
        .to_string()
        .contains("memory/ToM evidence demo route"));

    let mut packet =
        runtime_v2_theory_of_mind_foundation_contract().expect("theory-of-mind foundation packet");
    packet.evidence_requirements = vec!["observable evidence only".to_string()];
    assert!(packet
        .validate()
        .expect_err("missing policy-authorized requirement should fail")
        .to_string()
        .contains("policy-authorized-state"));

    let mut packet =
        runtime_v2_theory_of_mind_foundation_contract().expect("theory-of-mind foundation packet");
    packet.agent_models[0].hypothesis_summary =
        "Alex's hidden motive is known through mind-reading.".to_string();
    assert!(packet
        .validate()
        .expect_err("mind-reading claim should fail")
        .to_string()
        .contains("mind-reading"));

    let mut packet =
        runtime_v2_theory_of_mind_foundation_contract().expect("theory-of-mind foundation packet");
    packet.update_events[1].correction_of_event_id = None;
    assert!(packet
        .validate()
        .expect_err("correction event without corrected event should fail")
        .to_string()
        .contains("correction_update"));
}

#[test]
fn runtime_v2_theory_of_mind_foundation_validate_against_rejects_dependency_drift() {
    let citizen_state =
        runtime_v2_citizen_state_substrate_contract().expect("citizen-state substrate");
    let memory =
        runtime_v2_memory_identity_architecture_contract().expect("memory identity architecture");

    let mut packet =
        runtime_v2_theory_of_mind_foundation_contract().expect("theory-of-mind foundation packet");
    packet.memory_dependency_ref = "runtime_v2/memory_identity/drifted.json".to_string();
    assert!(packet
        .validate_against(&citizen_state, &memory)
        .expect_err("memory dependency drift should fail")
        .to_string()
        .contains("must depend on the landed memory/identity architecture"));

    let mut packet =
        runtime_v2_theory_of_mind_foundation_contract().expect("theory-of-mind foundation packet");
    packet.agent_models[0].evidence_ref = "runtime_v2/private_state/forbidden.json".to_string();
    assert!(packet
        .validate_against(&citizen_state, &memory)
        .expect_err("forbidden evidence ref should fail")
        .to_string()
        .contains("allowed citizen-state or memory evidence ref"));

    let mut packet =
        runtime_v2_theory_of_mind_foundation_contract().expect("theory-of-mind foundation packet");
    packet.update_events[0].evidence_refs = vec![memory.identity_evidence_refs[0].clone()];
    assert!(packet
        .validate_against(&citizen_state, &memory)
        .expect_err("observable update without observatory evidence should fail")
        .to_string()
        .contains("observatory projection evidence"));
}

#[test]
fn runtime_v2_theory_of_mind_foundation_proof_route_paths_exist() {
    let packet =
        runtime_v2_theory_of_mind_foundation_contract().expect("theory-of-mind foundation packet");
    let repo_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("repo root");
    let mut proof_paths = vec![
        packet.source_feature_doc.clone(),
        "adl/src/runtime_v2/theory_of_mind_foundation.rs".to_string(),
        "adl/src/runtime_v2/tests/theory_of_mind_foundation.rs".to_string(),
        "adl/tests/fixtures/runtime_v2/citizen_state/citizen_state_substrate.json".to_string(),
        "adl/tests/fixtures/runtime_v2/memory_identity/memory_identity_architecture.json"
            .to_string(),
    ];
    proof_paths.extend(
        packet
            .fixture_matrix
            .iter()
            .map(|fixture| fixture.artifact_ref.clone()),
    );

    for proof_path in proof_paths {
        assert!(
            repo_root.join(&proof_path).exists(),
            "expected proof-route path to exist: {proof_path}"
        );
    }
}

#[test]
fn runtime_v2_theory_of_mind_foundation_write_to_path_materializes_fixture() {
    let packet =
        runtime_v2_theory_of_mind_foundation_contract().expect("theory-of-mind foundation packet");
    let output_path = common::unique_temp_path("theory-of-mind-foundation-write")
        .join("runtime_v2/theory_of_mind/theory_of_mind_foundation.json");

    packet
        .write_to_path(&output_path)
        .expect("write theory-of-mind packet to explicit path");

    let written = std::fs::read_to_string(&output_path).expect("read written theory-of-mind json");
    assert_eq!(
        written.trim_end(),
        String::from_utf8(packet.pretty_json_bytes().expect("theory-of-mind json"))
            .expect("utf8")
            .trim_end()
    );
}

#[test]
fn runtime_v2_theory_of_mind_foundation_write_to_root_materializes_fixture() {
    let packet =
        runtime_v2_theory_of_mind_foundation_contract().expect("theory-of-mind foundation packet");
    let root = common::unique_temp_path("theory-of-mind-foundation-root");

    packet
        .write_to_root(&root)
        .expect("write theory-of-mind packet to temp root");

    let written = std::fs::read_to_string(root.join(RUNTIME_V2_THEORY_OF_MIND_FOUNDATION_PATH))
        .expect("read rooted theory-of-mind json");
    assert_eq!(
        written.trim_end(),
        String::from_utf8(packet.pretty_json_bytes().expect("theory-of-mind json"))
            .expect("utf8")
            .trim_end()
    );
}
