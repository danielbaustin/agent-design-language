use super::*;

#[test]
fn identity_foundation_writes_bounded_foundation_json() {
    let _guard = TEST_MUTEX
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let repo = temp_repo("identity-foundation");
    let out_path = repo.join(".adl/state/chronosense_foundation.v1.json");

    real_identity_in_repo(
        &[
            "foundation".to_string(),
            "--out".to_string(),
            ".adl/state/chronosense_foundation.v1.json".to_string(),
        ],
        &repo,
    )
    .expect("identity foundation");

    let json: Value =
        serde_json::from_slice(&fs::read(&out_path).expect("read out")).expect("parse json");
    assert_eq!(json["schema_version"], "chronosense_foundation.v1");
    assert_eq!(
        json["proof_hook_output_path"],
        ".adl/state/chronosense_foundation.v1.json"
    );
    assert!(json["owned_runtime_surfaces"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value == "adl identity foundation"));
}

#[test]
fn identity_foundation_validates_unknown_args_and_missing_out_value() {
    let repo = temp_repo("identity-foundation-errors");

    let err = real_identity_in_repo(&["foundation".to_string(), "--bogus".to_string()], &repo)
        .expect_err("unknown arg should fail");
    assert!(err
        .to_string()
        .contains("unknown arg for identity foundation: --bogus"));

    let err = real_identity_in_repo(&["foundation".to_string(), "--out".to_string()], &repo)
        .expect_err("out flag without value should fail");
    assert!(err.to_string().contains("--out requires a value"));
}

#[test]
fn identity_schema_writes_temporal_schema_contract_json() {
    let _guard = TEST_MUTEX
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let repo = temp_repo("identity-schema");
    let out_path = repo.join(".adl/state/temporal_schema_v01.json");

    real_identity_in_repo(
        &[
            "schema".to_string(),
            "--out".to_string(),
            ".adl/state/temporal_schema_v01.json".to_string(),
        ],
        &repo,
    )
    .expect("identity schema");

    let json: Value =
        serde_json::from_slice(&fs::read(&out_path).expect("read out")).expect("parse json");
    assert_eq!(json["schema_version"], "temporal_schema.v0_1");
    assert_eq!(
        json["proof_hook_output_path"],
        ".adl/state/temporal_schema_v01.json"
    );
    assert_eq!(
        json["primary_temporal_anchor"]["monotonic_order"],
        "required strictly increasing order token"
    );
    assert!(json["reference_frames"]["internal_reasoning"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value == "monotonic"));
    assert!(json["execution_policy_trace_hooks"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value == "run_state.v1.duration_ms"));
}

#[test]
fn identity_schema_validates_unknown_args_and_missing_out_value() {
    let repo = temp_repo("identity-schema-errors");

    let err = real_identity_in_repo(&["schema".to_string(), "--bogus".to_string()], &repo)
        .expect_err("unknown arg should fail");
    assert!(err
        .to_string()
        .contains("unknown arg for identity schema: --bogus"));

    let err = real_identity_in_repo(&["schema".to_string(), "--out".to_string()], &repo)
        .expect_err("out flag without value should fail");
    assert!(err.to_string().contains("--out requires a value"));
}

#[test]
fn identity_continuity_writes_continuity_semantics_contract_json() {
    let _guard = TEST_MUTEX
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let repo = temp_repo("identity-continuity");
    let out_path = repo.join(".adl/state/continuity_semantics_v1.json");

    real_identity_in_repo(
        &[
            "continuity".to_string(),
            "--out".to_string(),
            ".adl/state/continuity_semantics_v1.json".to_string(),
        ],
        &repo,
    )
    .expect("identity continuity");

    let json: Value =
        serde_json::from_slice(&fs::read(&out_path).expect("read out")).expect("parse json");
    assert_eq!(json["schema_version"], "continuity_semantics.v1");
    assert_eq!(
        json["proof_hook_output_path"],
        ".adl/state/continuity_semantics_v1.json"
    );
    assert!(json["continuity_state_contract"]["continuity_status"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value == "resume_ready"));
    assert!(json["resumption_rules"]
        .as_array()
        .expect("array")
        .iter()
        .any(|rule| {
            rule["continuity_status"] == "continuity_refused"
                && rule["resume_permitted"] == Value::Bool(false)
        }));
}

#[test]
fn identity_continuity_validates_unknown_args_and_missing_out_value() {
    let repo = temp_repo("identity-continuity-errors");

    let err = real_identity_in_repo(&["continuity".to_string(), "--bogus".to_string()], &repo)
        .expect_err("unknown arg should fail");
    assert!(err
        .to_string()
        .contains("unknown arg for identity continuity: --bogus"));

    let err = real_identity_in_repo(&["continuity".to_string(), "--out".to_string()], &repo)
        .expect_err("out flag without value should fail");
    assert!(err.to_string().contains("--out requires a value"));
}

#[test]
fn identity_retrieval_writes_temporal_query_retrieval_contract_json() {
    let _guard = TEST_MUTEX
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let repo = temp_repo("identity-retrieval");
    let out_path = repo.join(".adl/state/temporal_query_retrieval_v1.json");

    real_identity_in_repo(
        &[
            "retrieval".to_string(),
            "--out".to_string(),
            ".adl/state/temporal_query_retrieval_v1.json".to_string(),
        ],
        &repo,
    )
    .expect("identity retrieval");

    let json: Value =
        serde_json::from_slice(&fs::read(&out_path).expect("read out")).expect("parse json");
    assert_eq!(json["schema_version"], "temporal_query_retrieval.v1");
    assert_eq!(
        json["proof_hook_output_path"],
        ".adl/state/temporal_query_retrieval_v1.json"
    );
    assert!(json["owned_runtime_surfaces"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value == "adl identity retrieval"));
}

#[test]
fn identity_retrieval_validates_unknown_args_and_missing_out_value() {
    let repo = temp_repo("identity-retrieval-errors");

    let err = real_identity_in_repo(&["retrieval".to_string(), "--bogus".to_string()], &repo)
        .expect_err("unknown arg should fail");
    assert!(err
        .to_string()
        .contains("unknown arg for identity retrieval: --bogus"));

    let err = real_identity_in_repo(&["retrieval".to_string(), "--out".to_string()], &repo)
        .expect_err("out flag without value should fail");
    assert!(err.to_string().contains("--out requires a value"));
}

#[test]
fn identity_commitments_writes_commitment_deadline_contract_json() {
    let _guard = TEST_MUTEX
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let repo = temp_repo("identity-commitments");
    let out_path = repo.join(".adl/state/commitment_deadline_semantics_v1.json");

    real_identity_in_repo(
        &[
            "commitments".to_string(),
            "--out".to_string(),
            ".adl/state/commitment_deadline_semantics_v1.json".to_string(),
        ],
        &repo,
    )
    .expect("identity commitments");

    let json: Value =
        serde_json::from_slice(&fs::read(&out_path).expect("read out")).expect("parse json");
    assert_eq!(json["schema_version"], "commitment_deadline_semantics.v1");
    assert_eq!(
        json["proof_hook_output_path"],
        ".adl/state/commitment_deadline_semantics_v1.json"
    );
    assert!(json["lifecycle"]["states"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value == "missed"));
    assert!(json["deadline_semantics"]["supported_frames"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value == "continuity_relative"));
    assert!(json["owned_runtime_surfaces"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value == "adl identity commitments"));
}

#[test]
fn identity_commitments_validates_unknown_args_and_missing_out_value() {
    let repo = temp_repo("identity-commitments-errors");

    let err = real_identity_in_repo(&["commitments".to_string(), "--bogus".to_string()], &repo)
        .expect_err("unknown arg should fail");
    assert!(err
        .to_string()
        .contains("unknown arg for identity commitments: --bogus"));

    let err = real_identity_in_repo(&["commitments".to_string(), "--out".to_string()], &repo)
        .expect_err("out flag without value should fail");
    assert!(err.to_string().contains("--out requires a value"));
}

#[test]
fn identity_causality_writes_temporal_causality_explanation_contract_json() {
    let _guard = TEST_MUTEX
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let repo = temp_repo("identity-causality");
    let out_path = repo.join(".adl/state/temporal_causality_explanation_v1.json");

    real_identity_in_repo(
        &[
            "causality".to_string(),
            "--out".to_string(),
            ".adl/state/temporal_causality_explanation_v1.json".to_string(),
        ],
        &repo,
    )
    .expect("identity causality");

    let json: Value =
        serde_json::from_slice(&fs::read(&out_path).expect("read out")).expect("parse json");
    assert_eq!(json["schema_version"], "temporal_causality_explanation.v1");
    assert_eq!(
        json["proof_hook_output_path"],
        ".adl/state/temporal_causality_explanation_v1.json"
    );
    assert_eq!(
        json["causal_relations"]["sequence_boundary_rule"],
        "sequence alone is insufficient evidence for causality"
    );
    assert!(json["causal_relations"]["relation_types"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value == "unknown_relation"));
    assert!(json["owned_runtime_surfaces"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value == "adl identity causality"));
}

#[test]
fn identity_causality_validates_unknown_args_and_missing_out_value() {
    let repo = temp_repo("identity-causality-errors");

    let err = real_identity_in_repo(&["causality".to_string(), "--bogus".to_string()], &repo)
        .expect_err("unknown arg should fail");
    assert!(err
        .to_string()
        .contains("unknown arg for identity causality: --bogus"));

    let err = real_identity_in_repo(&["causality".to_string(), "--out".to_string()], &repo)
        .expect_err("out flag without value should fail");
    assert!(err.to_string().contains("--out requires a value"));
}

#[test]
fn identity_cost_writes_execution_policy_cost_model_contract_json() {
    let _guard = TEST_MUTEX
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let repo = temp_repo("identity-cost");
    let out_path = repo.join(".adl/state/execution_policy_cost_model_v1.json");

    real_identity_in_repo(
        &[
            "cost".to_string(),
            "--out".to_string(),
            ".adl/state/execution_policy_cost_model_v1.json".to_string(),
        ],
        &repo,
    )
    .expect("identity cost");

    let json: Value =
        serde_json::from_slice(&fs::read(&out_path).expect("read out")).expect("parse json");
    assert_eq!(json["schema_version"], "execution_policy_cost_model.v1");
    assert_eq!(
        json["proof_hook_output_path"],
        ".adl/state/execution_policy_cost_model_v1.json"
    );
    assert!(json["owned_runtime_surfaces"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value == "adl identity cost"));
}

#[test]
fn identity_cost_validates_unknown_args_and_missing_out_value() {
    let repo = temp_repo("identity-cost-errors");

    let err = real_identity_in_repo(&["cost".to_string(), "--bogus".to_string()], &repo)
        .expect_err("unknown arg should fail");
    assert!(err
        .to_string()
        .contains("unknown arg for identity cost: --bogus"));

    let err = real_identity_in_repo(&["cost".to_string(), "--out".to_string()], &repo)
        .expect_err("out flag without value should fail");
    assert!(err.to_string().contains("--out requires a value"));
}

#[test]
fn identity_phi_writes_phi_integration_metrics_contract_json() {
    let _guard = TEST_MUTEX
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let repo = temp_repo("identity-phi");
    let out_path = repo.join(".adl/state/phi_integration_metrics_v1.json");

    real_identity_in_repo(
        &[
            "phi".to_string(),
            "--out".to_string(),
            ".adl/state/phi_integration_metrics_v1.json".to_string(),
        ],
        &repo,
    )
    .expect("identity phi");

    let json: Value =
        serde_json::from_slice(&fs::read(&out_path).expect("read out")).expect("parse json");
    assert_eq!(json["schema_version"], "phi_integration_metrics.v1");
    assert_eq!(
        json["proof_hook_output_path"],
        ".adl/state/phi_integration_metrics_v1.json"
    );
    assert_eq!(
        json["comparison_profiles"].as_array().expect("array").len(),
        3
    );
    assert!(json["review_surface"]["non_goals"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value == "formal IIT phi calculation"));
    assert!(json["owned_runtime_surfaces"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value == "adl identity phi"));
}

#[test]
fn identity_phi_validates_unknown_args_and_missing_out_value() {
    let repo = temp_repo("identity-phi-errors");

    let err = real_identity_in_repo(&["phi".to_string(), "--bogus".to_string()], &repo)
        .expect_err("unknown arg should fail");
    assert!(err
        .to_string()
        .contains("unknown arg for identity phi: --bogus"));

    let err = real_identity_in_repo(&["phi".to_string(), "--out".to_string()], &repo)
        .expect_err("out flag without value should fail");
    assert!(err.to_string().contains("--out requires a value"));
}

#[test]
fn identity_instinct_writes_instinct_model_contract_json() {
    let _guard = TEST_MUTEX
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let repo = temp_repo("identity-instinct");
    let out_path = repo.join(".adl/state/instinct_model_v1.json");

    real_identity_in_repo(
        &[
            "instinct".to_string(),
            "--out".to_string(),
            ".adl/state/instinct_model_v1.json".to_string(),
        ],
        &repo,
    )
    .expect("identity instinct");

    let json: Value =
        serde_json::from_slice(&fs::read(&out_path).expect("read out")).expect("parse json");
    assert_eq!(json["schema_version"], "instinct_model.v1");
    assert_eq!(
        json["proof_hook_output_path"],
        ".adl/state/instinct_model_v1.json"
    );
    assert_eq!(json["instinct_set"].as_array().expect("array").len(), 4);
    assert!(json["instinct_set"]
        .as_array()
        .expect("array")
        .iter()
        .any(|entry| {
            entry["instinct_id"] == "integrity"
                && entry["subordinate_to"]
                    .as_array()
                    .expect("array")
                    .iter()
                    .any(|value| value == "policy")
        }));
    assert!(json["owned_runtime_surfaces"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value == "adl identity instinct"));
}

#[test]
fn identity_instinct_validates_unknown_args_and_missing_out_value() {
    let repo = temp_repo("identity-instinct-errors");

    let err = real_identity_in_repo(&["instinct".to_string(), "--bogus".to_string()], &repo)
        .expect_err("unknown arg should fail");
    assert!(err
        .to_string()
        .contains("unknown arg for identity instinct: --bogus"));

    let err = real_identity_in_repo(&["instinct".to_string(), "--out".to_string()], &repo)
        .expect_err("out flag without value should fail");
    assert!(err.to_string().contains("--out requires a value"));
}

#[test]
fn identity_instinct_runtime_writes_instinct_runtime_surface_contract_json() {
    let _guard = TEST_MUTEX
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let repo = temp_repo("identity-instinct-runtime");
    let out_path = repo.join(".adl/state/instinct_runtime_surface_v1.json");

    real_identity_in_repo(
        &[
            "instinct-runtime".to_string(),
            "--out".to_string(),
            ".adl/state/instinct_runtime_surface_v1.json".to_string(),
        ],
        &repo,
    )
    .expect("identity instinct-runtime");

    let json: Value =
        serde_json::from_slice(&fs::read(&out_path).expect("read out")).expect("parse json");
    assert_eq!(json["schema_version"], "instinct_runtime_surface.v1");
    assert_eq!(
        json["proof_hook_output_path"],
        ".adl/state/instinct_runtime_surface_v1.json"
    );
    assert!(json["proof_cases"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value["expected_candidate_id"] == "cand-fast-verify"));
    assert!(json["review_surface"]["policy_override_rule"]
        .as_str()
        .expect("string")
        .contains("high-risk slow-path review remains mandatory"));
    assert!(json["owned_runtime_surfaces"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value == "adl identity instinct-runtime"));
}

#[test]
fn identity_instinct_runtime_validates_unknown_args_and_missing_out_value() {
    let repo = temp_repo("identity-instinct-runtime-errors");

    let err = real_identity_in_repo(
        &["instinct-runtime".to_string(), "--bogus".to_string()],
        &repo,
    )
    .expect_err("unknown arg should fail");
    assert!(err
        .to_string()
        .contains("unknown arg for identity instinct-runtime: --bogus"));

    let err = real_identity_in_repo(
        &["instinct-runtime".to_string(), "--out".to_string()],
        &repo,
    )
    .expect_err("out flag without value should fail");
    assert!(err.to_string().contains("--out requires a value"));
}
