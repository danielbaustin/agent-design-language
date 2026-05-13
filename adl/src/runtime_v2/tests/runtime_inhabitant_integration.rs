use super::*;
use crate::capability_aptitude_testing::{
    build_capability_aptitude_artifact_bundle, CAPABILITY_APTITUDE_TESTING_ARTIFACT_ROOT,
};

#[test]
fn runtime_v2_runtime_inhabitant_integration_contract_is_stable() {
    let artifacts = runtime_v2_runtime_inhabitant_integration_contract()
        .expect("runtime inhabitant integration artifacts");
    artifacts
        .packet
        .validate()
        .expect("valid runtime inhabitant integration packet");

    assert_eq!(
        artifacts.packet.schema_version,
        RUNTIME_V2_RUNTIME_INHABITANT_INTEGRATION_PACKET_SCHEMA
    );
    assert_eq!(artifacts.packet.milestone, "v0.91.1");
    assert_eq!(artifacts.packet.wp, "WP-15");
    assert_eq!(
        artifacts.packet.capability_dependency_ref,
        CAPABILITY_APTITUDE_TESTING_ARTIFACT_ROOT
    );
    assert_eq!(artifacts.packet.integration_stage_refs.len(), 12);
    assert_eq!(artifacts.packet.trace_refs.len(), 4);
    assert!(artifacts
        .packet
        .non_claims
        .iter()
        .any(|claim| claim.contains("WP-16")));
}

#[test]
fn runtime_v2_runtime_inhabitant_integration_contract_registry_smoke_covers_accessors() {
    runtime_v2_csm_integrated_run_contract().expect("integrated run");
    runtime_v2_standing_contract().expect("standing");
    runtime_v2_citizen_state_substrate_contract().expect("citizen state");
    runtime_v2_agent_lifecycle_state_contract().expect("lifecycle");
    runtime_v2_memory_identity_architecture_contract().expect("memory");
    runtime_v2_theory_of_mind_foundation_contract().expect("theory of mind");
    build_capability_aptitude_artifact_bundle();
    runtime_v2_intelligence_metric_architecture_contract().expect("intelligence");
    runtime_v2_governed_learning_substrate_contract().expect("learning");
    runtime_v2_access_control_contract().expect("access control");
    runtime_v2_acip_hardening_contract().expect("ACIP hardening");
    runtime_v2_a2a_adapter_boundary_contract().expect("A2A boundary");
    runtime_v2_runtime_inhabitant_integration_contract().expect("runtime inhabitant integration");
}

#[test]
fn runtime_v2_runtime_inhabitant_integration_matches_golden_fixture_and_report() {
    let artifacts = runtime_v2_runtime_inhabitant_integration_contract()
        .expect("runtime inhabitant integration artifacts");
    let rendered = serde_json::to_value(&artifacts.packet).expect("serialize packet");
    let fixture: serde_json::Value = serde_json::from_str(include_str!(
        "../../../tests/fixtures/runtime_v2/inhabitant/runtime_inhabitant_integration.json"
    ))
    .expect("parse fixture");
    assert_eq!(rendered, fixture);

    let expected_report = include_str!(
        "../../../tests/fixtures/runtime_v2/inhabitant/runtime_inhabitant_operator_report.md"
    );
    assert_eq!(
        artifacts.operator_report_markdown.trim_end(),
        expected_report.trim_end()
    );
}

#[test]
fn runtime_v2_runtime_inhabitant_integration_validation_rejects_metadata_drift() {
    let mut artifacts = runtime_v2_runtime_inhabitant_integration_contract()
        .expect("runtime inhabitant integration artifacts");
    artifacts.packet.schema_version =
        "runtime_v2.runtime_inhabitant_integration_packet.v0".to_string();
    assert!(artifacts
        .packet
        .validate()
        .expect_err("schema drift should fail")
        .to_string()
        .contains("unsupported runtime inhabitant integration schema"));

    let mut artifacts = runtime_v2_runtime_inhabitant_integration_contract()
        .expect("runtime inhabitant integration artifacts");
    artifacts.packet.validation_commands.pop();
    assert!(artifacts
        .packet
        .validate()
        .expect_err("validation command drift should fail")
        .to_string()
        .contains("validation command set"));

    let mut artifacts = runtime_v2_runtime_inhabitant_integration_contract()
        .expect("runtime inhabitant integration artifacts");
    artifacts.packet.claim_boundary = "bounded integration".to_string();
    assert!(artifacts
        .packet
        .validate()
        .expect_err("claim boundary drift should fail")
        .to_string()
        .contains("birthday/autonomy claim boundary"));
}

#[test]
fn runtime_v2_runtime_inhabitant_integration_validation_rejects_stage_and_trace_gaps() {
    let mut artifacts = runtime_v2_runtime_inhabitant_integration_contract()
        .expect("runtime inhabitant integration artifacts");
    artifacts.packet.trace_refs.clear();
    assert!(artifacts
        .packet
        .validate()
        .expect_err("missing trace refs should fail")
        .to_string()
        .contains("exactly four reviewed trace refs"));

    let mut artifacts = runtime_v2_runtime_inhabitant_integration_contract()
        .expect("runtime inhabitant integration artifacts");
    artifacts.packet.integration_stage_refs[0].stage_id.clear();
    assert!(artifacts
        .packet
        .validate()
        .expect_err("bad stage id should fail")
        .to_string()
        .contains("stage_id"));

    let mut artifacts = runtime_v2_runtime_inhabitant_integration_contract()
        .expect("runtime inhabitant integration artifacts");
    artifacts.packet.integration_stage_refs[3].sequence = 99;
    assert!(artifacts
        .packet
        .validate()
        .expect_err("sequence drift should fail")
        .to_string()
        .contains("reviewed sequence order"));
}

#[test]
fn runtime_v2_runtime_inhabitant_integration_validate_against_rejects_dependency_drift() {
    let integrated_run = runtime_v2_csm_integrated_run_contract().expect("integrated run");
    let standing = runtime_v2_standing_contract().expect("standing");
    let citizen_state = runtime_v2_citizen_state_substrate_contract().expect("state");
    let lifecycle = runtime_v2_agent_lifecycle_state_contract().expect("lifecycle");
    let memory = runtime_v2_memory_identity_architecture_contract().expect("memory");
    let tom = runtime_v2_theory_of_mind_foundation_contract().expect("tom");
    let capability = build_capability_aptitude_artifact_bundle();
    let intelligence =
        runtime_v2_intelligence_metric_architecture_contract().expect("intelligence");
    let learning = runtime_v2_governed_learning_substrate_contract().expect("learning");
    let access = runtime_v2_access_control_contract().expect("access");
    let acip = runtime_v2_acip_hardening_contract().expect("acip");
    let a2a = runtime_v2_a2a_adapter_boundary_contract().expect("a2a");

    let mut artifacts = runtime_v2_runtime_inhabitant_integration_contract()
        .expect("runtime inhabitant integration artifacts");
    artifacts.packet.integrated_run_ref = "runtime_v2/csm_run/drifted.json".to_string();
    assert!(artifacts
        .packet
        .validate_against(
            &integrated_run,
            &standing,
            &citizen_state,
            &lifecycle,
            &memory,
            &tom,
            &capability,
            &intelligence,
            &learning,
            &access,
            &acip,
            &a2a,
        )
        .expect_err("integrated-run drift should fail")
        .to_string()
        .contains("integrated CSM run proof packet"));

    let mut artifacts = runtime_v2_runtime_inhabitant_integration_contract()
        .expect("runtime inhabitant integration artifacts");
    artifacts.packet.capability_dependency_ref =
        "docs/milestones/v0.91.1/review/drifted".to_string();
    assert!(artifacts
        .packet
        .validate_against(
            &integrated_run,
            &standing,
            &citizen_state,
            &lifecycle,
            &memory,
            &tom,
            &capability,
            &intelligence,
            &learning,
            &access,
            &acip,
            &a2a,
        )
        .expect_err("capability drift should fail")
        .to_string()
        .contains("capability harness artifact root"));

    let mut artifacts = runtime_v2_runtime_inhabitant_integration_contract()
        .expect("runtime inhabitant integration artifacts");
    artifacts.packet.trace_refs[1] = "runtime_v2/trace/drifted.jsonl".to_string();
    assert!(artifacts
        .packet
        .validate_against(
            &integrated_run,
            &standing,
            &citizen_state,
            &lifecycle,
            &memory,
            &tom,
            &capability,
            &intelligence,
            &learning,
            &access,
            &acip,
            &a2a,
        )
        .expect_err("trace drift should fail")
        .to_string()
        .contains("trace refs drifted"));
}

#[test]
fn runtime_v2_runtime_inhabitant_integration_proof_route_paths_exist() {
    let artifacts = runtime_v2_runtime_inhabitant_integration_contract()
        .expect("runtime inhabitant integration artifacts");
    let repo_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("repo root");

    let proof_paths = vec![
        artifacts.packet.source_feature_doc.clone(),
        "adl/src/runtime_v2/runtime_inhabitant_integration.rs".to_string(),
        "adl/src/runtime_v2/tests/runtime_inhabitant_integration.rs".to_string(),
        "adl/tests/fixtures/runtime_v2/inhabitant/runtime_inhabitant_integration.json".to_string(),
        "adl/tests/fixtures/runtime_v2/inhabitant/runtime_inhabitant_operator_report.md"
            .to_string(),
    ];
    for proof_path in proof_paths {
        assert!(
            repo_root.join(&proof_path).exists(),
            "expected proof-route path to exist: {proof_path}"
        );
    }

    let temp_root = common::unique_temp_path("runtime-inhabitant-integration-write");
    artifacts
        .write_to_root(&temp_root)
        .expect("write runtime inhabitant integration artifacts");
    let rendered = std::fs::read_to_string(
        temp_root.join(RUNTIME_V2_RUNTIME_INHABITANT_INTEGRATION_PACKET_PATH),
    )
    .expect("read written packet");
    assert!(rendered.contains("\"wp\": \"WP-15\""));

    let report =
        std::fs::read_to_string(temp_root.join(RUNTIME_V2_RUNTIME_INHABITANT_OPERATOR_REPORT_PATH))
            .expect("read written report");
    assert!(report.contains("Runtime Inhabitant Integration Operator Report"));
}
