use super::*;

#[test]
fn identity_operational_skills_writes_contract_json() {
    let _guard = TEST_MUTEX
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let repo = temp_repo("identity-operational-skills");
    let out_path = repo.join(".adl/state/operational_skills_substrate_v1.json");

    real_identity_in_repo(
        &[
            "operational-skills".to_string(),
            "--out".to_string(),
            ".adl/state/operational_skills_substrate_v1.json".to_string(),
        ],
        &repo,
    )
    .expect("identity operational-skills");

    let json: Value =
        serde_json::from_slice(&fs::read(&out_path).expect("read out")).expect("parse json");
    assert_eq!(json["schema_version"], "operational_skills_substrate.v1");
    assert_eq!(
        json["proof_hook_output_path"],
        ".adl/state/operational_skills_substrate_v1.json"
    );
    assert_eq!(json["execution_phases"][0]["phase_id"], "plan");
    assert_eq!(json["execution_phases"][4]["phase_id"], "commit");
    assert!(json["invocation_boundary"]["required_fields"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value == "trace_correlation_id"));
    assert!(json["bounded_arxiv_paper_writer"]["prohibited_actions"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value == "submit_to_arxiv"));
    assert!(json["review_surface"]["downstream_boundaries"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value.as_str().expect("string").contains("WP-09")));
    assert!(json["owned_runtime_surfaces"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value == "adl identity operational-skills"));
}

#[test]
fn identity_operational_skills_validates_unknown_args_and_missing_out_value() {
    let repo = temp_repo("identity-operational-skills-errors");

    let err = real_identity_in_repo(
        &["operational-skills".to_string(), "--bogus".to_string()],
        &repo,
    )
    .expect_err("unknown arg should fail");
    assert!(err
        .to_string()
        .contains("unknown arg for identity operational-skills: --bogus"));

    let err = real_identity_in_repo(
        &["operational-skills".to_string(), "--out".to_string()],
        &repo,
    )
    .expect_err("out flag without value should fail");
    assert!(err.to_string().contains("--out requires a value"));
}

#[test]
fn identity_skill_composition_writes_contract_json() {
    let _guard = TEST_MUTEX
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let repo = temp_repo("identity-skill-composition");
    let out_path = repo.join(".adl/state/skill_composition_model_v1.json");

    real_identity_in_repo(
        &[
            "skill-composition".to_string(),
            "--out".to_string(),
            ".adl/state/skill_composition_model_v1.json".to_string(),
        ],
        &repo,
    )
    .expect("identity skill-composition");

    let json: Value =
        serde_json::from_slice(&fs::read(&out_path).expect("read out")).expect("parse json");
    assert_eq!(json["schema_version"], "skill_composition_model.v1");
    assert_eq!(
        json["proof_hook_output_path"],
        ".adl/state/skill_composition_model_v1.json"
    );
    assert!(json["primitive_order"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value == "adjudication"));
    assert!(json["graph_contract"]["prohibited_shapes"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value == "dynamic_graph_mutation_after_plan_phase"));
    assert!(json["bounded_arxiv_writer_composition"]["gates"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value
            .as_str()
            .expect("string")
            .contains("human_publication_gate")));
    assert!(json["review_surface"]["downstream_boundaries"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value.as_str().expect("string").contains("WP-13")));
    assert!(json["owned_runtime_surfaces"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value == "adl identity skill-composition"));
}

#[test]
fn identity_skill_composition_validates_unknown_args_and_missing_out_value() {
    let repo = temp_repo("identity-skill-composition-errors");

    let err = real_identity_in_repo(
        &["skill-composition".to_string(), "--bogus".to_string()],
        &repo,
    )
    .expect_err("unknown arg should fail");
    assert!(err
        .to_string()
        .contains("unknown arg for identity skill-composition: --bogus"));

    let err = real_identity_in_repo(
        &["skill-composition".to_string(), "--out".to_string()],
        &repo,
    )
    .expect_err("out flag without value should fail");
    assert!(err.to_string().contains("--out requires a value"));
}

#[test]
fn identity_delegation_refusal_coordination_writes_contract_json() {
    let _guard = TEST_MUTEX
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let repo = temp_repo("identity-delegation-refusal-coordination");
    let out_path = repo.join(".adl/state/delegation_refusal_coordination_v1.json");

    real_identity_in_repo(
        &[
            "delegation-refusal-coordination".to_string(),
            "--out".to_string(),
            ".adl/state/delegation_refusal_coordination_v1.json".to_string(),
        ],
        &repo,
    )
    .expect("identity delegation-refusal-coordination");

    let json: Value =
        serde_json::from_slice(&fs::read(&out_path).expect("read out")).expect("parse json");
    assert_eq!(json["schema_version"], "delegation_refusal_coordination.v1");
    assert_eq!(
        json["proof_hook_output_path"],
        ".adl/state/delegation_refusal_coordination_v1.json"
    );
    assert!(json["outcome_taxonomy"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value["outcome_kind"] == "governed_refusal"));
    assert!(
        json["delegation_refusal_boundary"]["failure_separation_rules"]
            .as_array()
            .expect("array")
            .iter()
            .any(|value| value.as_str().expect("string").contains("governed refusal"))
    );
    assert!(json["coordination_negotiation"]["allowed_outcomes"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value == "bounded_dissent"));
    assert!(json["owned_runtime_surfaces"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value == "adl identity delegation-refusal-coordination"));
}

#[test]
fn identity_delegation_refusal_coordination_validates_unknown_args_and_missing_out_value() {
    let repo = temp_repo("identity-delegation-refusal-coordination-errors");

    let err = real_identity_in_repo(
        &[
            "delegation-refusal-coordination".to_string(),
            "--bogus".to_string(),
        ],
        &repo,
    )
    .expect_err("unknown arg should fail");
    assert!(err
        .to_string()
        .contains("unknown arg for identity delegation-refusal-coordination: --bogus"));

    let err = real_identity_in_repo(
        &[
            "delegation-refusal-coordination".to_string(),
            "--out".to_string(),
        ],
        &repo,
    )
    .expect_err("out flag without value should fail");
    assert!(err.to_string().contains("--out requires a value"));
}

#[test]
fn identity_provider_extension_packaging_writes_contract_json() {
    let _guard = TEST_MUTEX
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let repo = temp_repo("identity-provider-extension-packaging");
    let out_path = repo.join(".adl/state/provider_extension_packaging_v1.json");

    real_identity_in_repo(
        &[
            "provider-extension-packaging".to_string(),
            "--out".to_string(),
            ".adl/state/provider_extension_packaging_v1.json".to_string(),
        ],
        &repo,
    )
    .expect("identity provider-extension-packaging");

    let json: Value =
        serde_json::from_slice(&fs::read(&out_path).expect("read out")).expect("parse json");
    assert_eq!(json["schema_version"], "provider_extension_packaging.v1");
    assert_eq!(
        json["proof_hook_output_path"],
        ".adl/state/provider_extension_packaging_v1.json"
    );
    assert!(json["scope_decision"]["non_promoted_inputs"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value == "PROVIDER_SECURITY_CAPABILITIES_EXTENSION.md"));
    assert!(
        json["capability_boundary"]["excluded_security_capabilities"]
            .as_array()
            .expect("array")
            .iter()
            .any(|value| value == "provider attestation")
    );
    assert!(json["owned_runtime_surfaces"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value == "adl identity provider-extension-packaging"));
}

#[test]
fn identity_provider_extension_packaging_validates_unknown_args_and_missing_out_value() {
    let repo = temp_repo("identity-provider-extension-packaging-errors");

    let err = real_identity_in_repo(
        &[
            "provider-extension-packaging".to_string(),
            "--bogus".to_string(),
        ],
        &repo,
    )
    .expect_err("unknown arg should fail");
    assert!(err
        .to_string()
        .contains("unknown arg for identity provider-extension-packaging: --bogus"));

    let err = real_identity_in_repo(
        &[
            "provider-extension-packaging".to_string(),
            "--out".to_string(),
        ],
        &repo,
    )
    .expect_err("out flag without value should fail");
    assert!(err.to_string().contains("--out requires a value"));
}

#[test]
fn identity_demo_proof_entry_points_writes_contract_json() {
    let _guard = TEST_MUTEX
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let repo = temp_repo("identity-demo-proof-entry-points");
    let out_path = repo.join(".adl/state/demo_proof_entry_points_v1.json");

    real_identity_in_repo(
        &[
            "demo-proof-entry-points".to_string(),
            "--out".to_string(),
            ".adl/state/demo_proof_entry_points_v1.json".to_string(),
        ],
        &repo,
    )
    .expect("identity demo-proof-entry-points");

    let json: Value =
        serde_json::from_slice(&fs::read(&out_path).expect("read out")).expect("parse json");
    assert_eq!(json["schema_version"], "demo_proof_entry_points.v1");
    assert_eq!(
        json["proof_hook_output_path"],
        ".adl/state/demo_proof_entry_points_v1.json"
    );
    assert!(json["owned_runtime_surfaces"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value == "adl identity demo-proof-entry-points"));
    assert!(
        json["package"]["rows"]
            .as_array()
            .expect("rows")
            .iter()
            .any(|row| row["demo_id"] == "D1"
                && row["entry_commands"]
                    .as_array()
                    .expect("entry commands")
                    .iter()
                    .any(|command| command
                        == "adl identity adversarial-runtime --out .adl/state/adversarial_runtime_model_v1.json"))
    );
    assert!(json["package"]["rows"]
        .as_array()
        .expect("rows")
        .iter()
        .any(|row| row["demo_id"] == "D8"
            && row["status"] == "LANDED"
            && row["entry_commands"]
                .as_array()
                .expect("entry commands")
                .iter()
                .any(|command| command == "bash adl/tools/demo_v0891_five_agent_hey_jude.sh")));
    assert!(json["package"]["rows"]
        .as_array()
        .expect("rows")
        .iter()
        .any(|row| row["demo_id"] == "D9"
            && row["status"] == "LANDED"
            && row["primary_proof_surfaces"]
                .as_array()
                .expect("proof surfaces")
                .iter()
                .any(|surface| surface
                    == "artifacts/v0891/arxiv_manuscript_workflow/manuscript_status/three_paper_status.json")));
}

#[test]
fn identity_demo_proof_entry_points_validates_unknown_args_and_missing_out_value() {
    let repo = temp_repo("identity-demo-proof-entry-points-errors");

    let err = real_identity_in_repo(
        &["demo-proof-entry-points".to_string(), "--bogus".to_string()],
        &repo,
    )
    .expect_err("unknown arg should fail");
    assert!(err
        .to_string()
        .contains("unknown arg for identity demo-proof-entry-points: --bogus"));

    let err = real_identity_in_repo(
        &["demo-proof-entry-points".to_string(), "--out".to_string()],
        &repo,
    )
    .expect_err("out flag without value should fail");
    assert!(err.to_string().contains("--out requires a value"));
}
