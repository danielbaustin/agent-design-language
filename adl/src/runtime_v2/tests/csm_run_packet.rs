#[cfg(feature = "slow-proof-tests")]
use super::common::unique_temp_path;
use super::*;
#[cfg(feature = "slow-proof-tests")]
use std::fs;

#[test]
fn runtime_v2_csm_run_packet_contract_is_stable() {
    let contract = runtime_v2_csm_run_packet_contract().expect("csm run contract");
    contract.validate().expect("valid CSM run contract");

    assert_eq!(
        contract.schema_version,
        RUNTIME_V2_CSM_RUN_PACKET_CONTRACT_SCHEMA
    );
    assert_eq!(contract.demo_id, "D2");
    assert_eq!(contract.manifold_id, "proto-csm-01");
    assert_eq!(
        contract.artifact_path,
        "runtime_v2/csm_run/run_packet_contract.json"
    );
    assert!(contract
        .artifact_requirements
        .iter()
        .any(|artifact| artifact.artifact_id == "run_packet_fixture"
            && artifact.must_exist_before_live_run));
    assert!(contract
        .artifact_requirements
        .iter()
        .any(|artifact| artifact.artifact_id == "observatory_packet"
            && !artifact.must_exist_before_live_run));
    assert!(contract
        .artifact_requirements
        .iter()
        .any(
            |artifact| artifact.artifact_id == "recovery_eligibility_model"
                && artifact.owner_wp == "WP-11"
                && !artifact.must_exist_before_live_run
        ));
    assert!(contract
        .artifact_requirements
        .iter()
        .any(|artifact| artifact.artifact_id == "quarantine_artifact"
            && artifact.owner_wp == "WP-12"
            && !artifact.must_exist_before_live_run));
    assert!(contract
        .artifact_requirements
        .iter()
        .any(|artifact| artifact.artifact_id == "hardening_proof_packet"
            && artifact.owner_wp == "WP-13"
            && !artifact.must_exist_before_live_run));
    assert!(contract
        .artifact_requirements
        .iter()
        .any(
            |artifact| artifact.artifact_id == "integrated_first_run_transcript"
                && artifact.owner_wp == "WP-14"
                && !artifact.must_exist_before_live_run
        ));
    assert!(contract
        .artifact_requirements
        .iter()
        .any(
            |artifact| artifact.artifact_id == "integrated_first_run_proof_packet"
                && artifact.owner_wp == "WP-14"
                && !artifact.must_exist_before_live_run
        ));
    assert_eq!(contract.stages.len(), 7);
    assert_eq!(contract.stages[0].owner_wp, "WP-03");
    assert_eq!(contract.stages[4].owner_wp, "WP-09-WP-10");
    assert_eq!(contract.stages[5].owner_wp, "WP-11-WP-13");
    assert_eq!(contract.stages[6].owner_wp, "WP-14");
    assert!(contract
        .claim_boundary
        .contains("not a live Runtime v2 execution artifact"));
}

#[test]
fn runtime_v2_csm_run_packet_contract_matches_golden_fixture() {
    let contract = runtime_v2_csm_run_packet_contract().expect("csm run contract");
    let generated =
        String::from_utf8(contract.to_pretty_json_bytes().expect("json")).expect("utf8");

    assert_eq!(
        generated,
        include_str!("../../../tests/fixtures/runtime_v2/csm_run/run_packet_contract.json")
            .trim_end()
    );
}

#[cfg(feature = "slow-proof-tests")]
#[test]
fn runtime_v2_csm_run_packet_contract_writes_without_path_leakage() {
    let temp_root = unique_temp_path("csm-run-contract");
    let contract = runtime_v2_csm_run_packet_contract().expect("csm run contract");

    contract
        .write_to_root(&temp_root)
        .expect("write CSM run contract");

    let contract_path = temp_root.join(&contract.artifact_path);
    assert!(contract_path.is_file());
    let text = fs::read_to_string(contract_path).expect("contract text");
    assert!(!text.contains(temp_root.to_string_lossy().as_ref()));
    assert!(text.contains("\"schema_version\": \"runtime_v2.csm_run_packet_contract.v1\""));
    assert!(text.contains("\"demo_id\": \"D2\""));
    assert!(text.contains("not a live Runtime v2 execution artifact"));

    fs::remove_dir_all(temp_root).ok();
}

#[test]
fn runtime_v2_csm_run_packet_validation_rejects_unsafe_or_ambiguous_state() {
    let mut contract = runtime_v2_csm_run_packet_contract().expect("csm run contract");
    contract.artifact_path = "/tmp/runtime_v2/csm_run/run_packet_contract.json".to_string();
    assert!(contract
        .validate()
        .expect_err("absolute path should fail")
        .to_string()
        .contains("repository-relative path"));

    let mut contract = runtime_v2_csm_run_packet_contract().expect("csm run contract");
    contract.stages[1].sequence = 9;
    assert!(contract
        .validate()
        .expect_err("non-contiguous stage order should fail")
        .to_string()
        .contains("contiguous sequence order"));

    let mut contract = runtime_v2_csm_run_packet_contract().expect("csm run contract");
    contract
        .artifact_requirements
        .retain(|artifact| artifact.artifact_id != "violation_schema");
    assert!(contract
        .validate()
        .expect_err("missing violation schema should fail")
        .to_string()
        .contains("violation_schema"));

    let mut contract = runtime_v2_csm_run_packet_contract().expect("csm run contract");
    contract.claim_boundary = "live run succeeded".to_string();
    assert!(contract
        .validate()
        .expect_err("overclaim should fail")
        .to_string()
        .contains("non-live claim boundary"));
}

#[test]
fn runtime_v2_csm_run_packet_fixture_is_reviewable_and_bounded() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!(
        "../../../../demos/fixtures/csm_run/proto-csm-01-run-packet.json"
    ))
    .expect("parse CSM run packet fixture");

    assert_eq!(fixture["schema"], "adl.csm_run_packet_fixture.v1");
    assert_eq!(fixture["manifold_id"], "proto-csm-01");
    assert_eq!(fixture["demo_id"], "D2");
    assert_eq!(
        fixture["contract_ref"],
        "runtime_v2/csm_run/run_packet_contract.json"
    );
    assert!(fixture["claim_boundary"]
        .as_str()
        .expect("claim boundary")
        .contains("does not prove that Runtime v2 has executed the run"));
    assert_eq!(
        fixture["stage_plan"]
            .as_array()
            .expect("stage plan array")
            .len(),
        9
    );
    assert!(fixture["required_before_live_run"]
        .as_array()
        .expect("required artifacts")
        .iter()
        .any(|value| value == "runtime_v2/invariants/csm_run_invariant_map.json"));
    assert!(fixture["required_before_live_run"]
        .as_array()
        .expect("required artifacts")
        .iter()
        .any(|value| value == "runtime_v2/recovery/eligibility_model.json"));
    assert!(fixture["required_before_live_run"]
        .as_array()
        .expect("required artifacts")
        .iter()
        .any(|value| value == "runtime_v2/quarantine/quarantine_artifact.json"));
    assert!(fixture["required_before_live_run"]
        .as_array()
        .expect("required artifacts")
        .iter()
        .any(|value| value == "runtime_v2/hardening/hardening_proof_packet.json"));
    assert!(fixture["required_before_live_run"]
        .as_array()
        .expect("required artifacts")
        .iter()
        .any(|value| value == "runtime_v2/csm_run/integrated_first_run_transcript.jsonl"));
    assert!(fixture["required_before_live_run"]
        .as_array()
        .expect("required artifacts")
        .iter()
        .any(|value| value == "runtime_v2/csm_run/integrated_first_run_proof_packet.json"));

    let text = include_str!("../../../../demos/fixtures/csm_run/proto-csm-01-run-packet.json");
    assert!(!text.contains(&["/", "Users/"].concat()));
    assert!(!text.contains(&["/", "private/"].concat()));
    assert!(!text.contains("BEGIN "));
}
