#[cfg(feature = "slow-proof-tests")]
use super::common::unique_temp_path;
use super::*;
#[cfg(feature = "slow-proof-tests")]
use std::fs;

#[test]
fn runtime_v2_invariant_and_violation_contract_is_stable() {
    let artifacts =
        runtime_v2_invariant_and_violation_contract().expect("invariant and violation artifacts");
    artifacts.validate().expect("valid WP-04 artifacts");

    assert_eq!(
        artifacts.invariant_map.schema_version,
        RUNTIME_V2_CSM_RUN_INVARIANT_MAP_SCHEMA
    );
    assert_eq!(artifacts.invariant_map.demo_id, "D2");
    assert_eq!(artifacts.invariant_map.manifold_id, "proto-csm-01");
    assert_eq!(
        artifacts.invariant_map.artifact_path,
        "runtime_v2/invariants/csm_run_invariant_map.json"
    );
    assert_eq!(artifacts.invariant_map.coverage_entries.len(), 5);
    assert!(artifacts
        .invariant_map
        .coverage_entries
        .iter()
        .any(
            |entry| entry.invariant_id == "invalid_action_must_be_refused_before_commit"
                && entry.coverage_status == "negative_fixture_backed"
        ));
    assert_eq!(
        artifacts.violation_schema.schema_version,
        RUNTIME_V2_VIOLATION_ARTIFACT_SCHEMA_CONTRACT
    );
    assert_eq!(
        artifacts.violation_schema.artifact_schema_version,
        RUNTIME_V2_INVARIANT_VIOLATION_SCHEMA
    );
    assert_eq!(
        artifacts.violation_schema.negative_fixture_ref,
        "runtime_v2/invariants/violation-0001.json"
    );
}

#[test]
fn runtime_v2_invariant_and_violation_contract_matches_golden_fixtures() {
    let artifacts =
        runtime_v2_invariant_and_violation_contract().expect("invariant and violation artifacts");
    let invariant_map = String::from_utf8(
        artifacts
            .invariant_map
            .to_pretty_json_bytes()
            .expect("json"),
    )
    .expect("utf8");
    let violation_schema = String::from_utf8(
        artifacts
            .violation_schema
            .to_pretty_json_bytes()
            .expect("json"),
    )
    .expect("utf8");

    assert_eq!(
        invariant_map,
        include_str!("../../../tests/fixtures/runtime_v2/invariants/csm_run_invariant_map.json")
            .trim_end()
    );
    assert_eq!(
        violation_schema,
        include_str!(
            "../../../tests/fixtures/runtime_v2/violations/violation_artifact_schema.json"
        )
        .trim_end()
    );
}

#[cfg(feature = "slow-proof-tests")]
#[test]
fn runtime_v2_invariant_and_violation_contract_writes_without_path_leakage() {
    let temp_root = unique_temp_path("wp04-contract");
    let artifacts =
        runtime_v2_invariant_and_violation_contract().expect("invariant and violation artifacts");

    artifacts
        .write_to_root(&temp_root)
        .expect("write WP-04 artifacts");

    let invariant_path = temp_root.join(&artifacts.invariant_map.artifact_path);
    let violation_schema_path = temp_root.join(&artifacts.violation_schema.artifact_path);
    assert!(invariant_path.is_file());
    assert!(violation_schema_path.is_file());
    let invariant_text = fs::read_to_string(invariant_path).expect("invariant map text");
    let violation_text = fs::read_to_string(violation_schema_path).expect("violation schema text");
    let temp_root_text = temp_root.to_string_lossy();
    assert!(!invariant_text.contains(temp_root_text.as_ref()));
    assert!(!violation_text.contains(temp_root_text.as_ref()));
    assert!(invariant_text.contains("\"schema_version\": \"runtime_v2.csm_run_invariant_map.v1\""));
    assert!(violation_text
        .contains("\"schema_version\": \"runtime_v2.violation_artifact_schema_contract.v1\""));

    fs::remove_dir_all(temp_root).ok();
}

#[test]
fn runtime_v2_invariant_and_violation_validation_rejects_unsafe_or_ambiguous_state() {
    let mut artifacts =
        runtime_v2_invariant_and_violation_contract().expect("invariant and violation artifacts");
    artifacts.invariant_map.artifact_path =
        "/tmp/runtime_v2/invariants/csm_run_invariant_map.json".to_string();
    assert!(artifacts
        .validate()
        .expect_err("absolute invariant path should fail")
        .to_string()
        .contains("repository-relative path"));

    let mut artifacts =
        runtime_v2_invariant_and_violation_contract().expect("invariant and violation artifacts");
    artifacts
        .invariant_map
        .coverage_entries
        .retain(|entry| entry.invariant_id != "invalid_action_must_be_refused_before_commit");
    assert!(artifacts
        .validate()
        .expect_err("missing invariant should fail")
        .to_string()
        .contains("D2 invariant set"));

    let mut artifacts =
        runtime_v2_invariant_and_violation_contract().expect("invariant and violation artifacts");
    artifacts
        .violation_schema
        .required_fields
        .retain(|field| field.field_name != "result");
    assert!(artifacts
        .validate()
        .expect_err("missing result field should fail")
        .to_string()
        .contains("every required field"));

    let mut artifacts =
        runtime_v2_invariant_and_violation_contract().expect("invariant and violation artifacts");
    artifacts.invariant_map.claim_boundary = "live Runtime v2 run executed".to_string();
    assert!(artifacts
        .validate()
        .expect_err("overclaim should fail")
        .to_string()
        .contains("non-live claim boundary"));
}

#[test]
fn runtime_v2_invariant_and_violation_fixtures_are_positive_and_negative_pair() {
    let positive: serde_json::Value = serde_json::from_str(include_str!(
        "../../../tests/fixtures/runtime_v2/csm_run/run_packet_contract.json"
    ))
    .expect("parse positive fixture");
    let negative: serde_json::Value = serde_json::from_str(include_str!(
        "../../../tests/fixtures/runtime_v2/invariants/violation-0001.json"
    ))
    .expect("parse negative fixture");

    assert_eq!(
        positive["schema_version"],
        "runtime_v2.csm_run_packet_contract.v1"
    );
    assert_eq!(
        negative["schema_version"],
        "runtime_v2.invariant_violation.v1"
    );
    assert_eq!(
        negative["result"]["blocked_before_commit"],
        serde_json::Value::Bool(true)
    );
    assert_eq!(
        negative["result"]["resulting_state"],
        "transition_refused_state_unchanged"
    );
}
