use super::*;

#[test]
fn identity_adversarial_runtime_writes_contract_json() {
    let _guard = TEST_MUTEX
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let repo = temp_repo("identity-adversarial-runtime");
    let out_path = repo.join(".adl/state/adversarial_runtime_model_v1.json");

    real_identity_in_repo(
        &[
            "adversarial-runtime".to_string(),
            "--out".to_string(),
            ".adl/state/adversarial_runtime_model_v1.json".to_string(),
        ],
        &repo,
    )
    .expect("identity adversarial-runtime");

    let json: Value =
        serde_json::from_slice(&fs::read(&out_path).expect("read out")).expect("parse json");
    assert_eq!(json["schema_version"], "adversarial_runtime_model.v1");
    assert_eq!(
        json["proof_hook_output_path"],
        ".adl/state/adversarial_runtime_model_v1.json"
    );
    assert!(json["adversarial_pressure"]["operating_assumptions"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value
            == "systems are probed continuously rather than only during scheduled review windows"));
    assert!(json["review_surface"]["downstream_boundaries"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value.as_str().expect("string").contains("WP-04")));
    assert!(json["owned_runtime_surfaces"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value == "adl identity adversarial-runtime"));
}

#[test]
fn identity_adversarial_runtime_validates_unknown_args_and_missing_out_value() {
    let repo = temp_repo("identity-adversarial-runtime-errors");

    let err = real_identity_in_repo(
        &["adversarial-runtime".to_string(), "--bogus".to_string()],
        &repo,
    )
    .expect_err("unknown arg should fail");
    assert!(err
        .to_string()
        .contains("unknown arg for identity adversarial-runtime: --bogus"));

    let err = real_identity_in_repo(
        &["adversarial-runtime".to_string(), "--out".to_string()],
        &repo,
    )
    .expect_err("out flag without value should fail");
    assert!(err.to_string().contains("--out requires a value"));
}

#[test]
fn identity_red_blue_architecture_writes_contract_json() {
    let _guard = TEST_MUTEX
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let repo = temp_repo("identity-red-blue-architecture");
    let out_path = repo.join(".adl/state/red_blue_agent_architecture_v1.json");

    real_identity_in_repo(
        &[
            "red-blue-architecture".to_string(),
            "--out".to_string(),
            ".adl/state/red_blue_agent_architecture_v1.json".to_string(),
        ],
        &repo,
    )
    .expect("identity red-blue-architecture");

    let json: Value =
        serde_json::from_slice(&fs::read(&out_path).expect("read out")).expect("parse json");
    assert_eq!(json["schema_version"], "red_blue_agent_architecture.v1");
    assert_eq!(
        json["proof_hook_output_path"],
        ".adl/state/red_blue_agent_architecture_v1.json"
    );
    assert_eq!(json["red_role"]["role"], "red");
    assert!(json["purple_coordination"]["governance_responsibilities"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value == "govern replay and escalation order"));
    assert!(json["interaction_model"]["stage_order"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value == "blue risk evaluation"));
    assert!(json["review_surface"]["downstream_boundaries"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value.as_str().expect("string").contains("WP-04")));
    assert!(json["owned_runtime_surfaces"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value == "adl identity red-blue-architecture"));
}

#[test]
fn identity_red_blue_architecture_validates_unknown_args_and_missing_out_value() {
    let repo = temp_repo("identity-red-blue-architecture-errors");

    let err = real_identity_in_repo(
        &["red-blue-architecture".to_string(), "--bogus".to_string()],
        &repo,
    )
    .expect_err("unknown arg should fail");
    assert!(err
        .to_string()
        .contains("unknown arg for identity red-blue-architecture: --bogus"));

    let err = real_identity_in_repo(
        &["red-blue-architecture".to_string(), "--out".to_string()],
        &repo,
    )
    .expect_err("out flag without value should fail");
    assert!(err.to_string().contains("--out requires a value"));
}

#[test]
fn identity_adversarial_runner_writes_contract_json() {
    let _guard = TEST_MUTEX
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let repo = temp_repo("identity-adversarial-runner");
    let out_path = repo.join(".adl/state/adversarial_execution_runner_v1.json");

    real_identity_in_repo(
        &[
            "adversarial-runner".to_string(),
            "--out".to_string(),
            ".adl/state/adversarial_execution_runner_v1.json".to_string(),
        ],
        &repo,
    )
    .expect("identity adversarial-runner");

    let json: Value =
        serde_json::from_slice(&fs::read(&out_path).expect("read out")).expect("parse json");
    assert_eq!(json["schema_version"], "adversarial_execution_runner.v1");
    assert_eq!(
        json["proof_hook_output_path"],
        ".adl/state/adversarial_execution_runner_v1.json"
    );
    assert!(json["canonical_stages"]
        .as_array()
        .expect("array")
        .iter()
        .any(|stage| stage["stage_id"] == "attempt_bounded_exploit"
            && stage["blocked_in_postures"]
                .as_array()
                .expect("array")
                .iter()
                .any(|posture| posture == "audit")));
    assert!(json["posture_policy"]["enforcement_rules"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value == "limit exhaustion produces an explicit defer record"));
    assert!(json["evidence_capture"]["linkage_rules"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value
            .as_str()
            .expect("string")
            .contains("mitigation decisions must cite exploit evidence")));
    assert!(json["review_surface"]["downstream_boundaries"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value.as_str().expect("string").contains("WP-05")));
    assert!(json["owned_runtime_surfaces"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value == "adl identity adversarial-runner"));
}

#[test]
fn identity_adversarial_runner_validates_unknown_args_and_missing_out_value() {
    let repo = temp_repo("identity-adversarial-runner-errors");

    let err = real_identity_in_repo(
        &["adversarial-runner".to_string(), "--bogus".to_string()],
        &repo,
    )
    .expect_err("unknown arg should fail");
    assert!(err
        .to_string()
        .contains("unknown arg for identity adversarial-runner: --bogus"));

    let err = real_identity_in_repo(
        &["adversarial-runner".to_string(), "--out".to_string()],
        &repo,
    )
    .expect_err("out flag without value should fail");
    assert!(err.to_string().contains("--out requires a value"));
}

#[test]
fn identity_exploit_replay_writes_contract_json() {
    let _guard = TEST_MUTEX
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let repo = temp_repo("identity-exploit-replay");
    let out_path = repo.join(".adl/state/exploit_artifact_replay_v1.json");

    real_identity_in_repo(
        &[
            "exploit-replay".to_string(),
            "--out".to_string(),
            ".adl/state/exploit_artifact_replay_v1.json".to_string(),
        ],
        &repo,
    )
    .expect("identity exploit-replay");

    let json: Value =
        serde_json::from_slice(&fs::read(&out_path).expect("read out")).expect("parse json");
    assert_eq!(json["schema_version"], "exploit_artifact_replay.v1");
    assert_eq!(
        json["proof_hook_output_path"],
        ".adl/state/exploit_artifact_replay_v1.json"
    );
    assert!(json["lifecycle_order"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value == "AdversarialReplayManifest"));
    assert!(json["replay_manifest"]["replay_modes"]
        .as_array()
        .expect("array")
        .iter()
        .any(|mode| mode["mode"] == "bounded_variance"));
    assert!(json["integrity"]["rules"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value == "no mitigation without exploit evidence linkage"));
    assert!(json["runner_integration"]["upstream_contracts"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value == "adversarial_execution_runner.v1"));
    assert!(json["owned_runtime_surfaces"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value == "adl identity exploit-replay"));
}

#[test]
fn identity_exploit_replay_validates_unknown_args_and_missing_out_value() {
    let repo = temp_repo("identity-exploit-replay-errors");

    let err = real_identity_in_repo(
        &["exploit-replay".to_string(), "--bogus".to_string()],
        &repo,
    )
    .expect_err("unknown arg should fail");
    assert!(err
        .to_string()
        .contains("unknown arg for identity exploit-replay: --bogus"));

    let err = real_identity_in_repo(&["exploit-replay".to_string(), "--out".to_string()], &repo)
        .expect_err("out flag without value should fail");
    assert!(err.to_string().contains("--out requires a value"));
}

#[test]
fn identity_continuous_verification_writes_contract_json() {
    let _guard = TEST_MUTEX
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let repo = temp_repo("identity-continuous-verification");
    let out_path = repo.join(".adl/state/continuous_verification_self_attack_v1.json");

    real_identity_in_repo(
        &[
            "continuous-verification".to_string(),
            "--out".to_string(),
            ".adl/state/continuous_verification_self_attack_v1.json".to_string(),
        ],
        &repo,
    )
    .expect("identity continuous-verification");

    let json: Value =
        serde_json::from_slice(&fs::read(&out_path).expect("read out")).expect("parse json");
    assert_eq!(
        json["schema_version"],
        "continuous_verification_self_attack.v1"
    );
    assert_eq!(
        json["proof_hook_output_path"],
        ".adl/state/continuous_verification_self_attack_v1.json"
    );
    assert!(json["cadence"]["supported_modes"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value == "continuous_bounded"));
    assert!(json["lifecycle"]
        .as_array()
        .expect("array")
        .iter()
        .any(|stage| stage["stage_id"] == "validate_replay"));
    assert!(json["self_attack_layers"]
        .as_array()
        .expect("array")
        .iter()
        .any(|layer| layer["layer_id"] == "learning_promotion"));
    assert!(json["policy"]["prohibited_shortcuts"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value == "self-attack without target allowlist"));
    assert!(json["upstream_contracts"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value == "exploit_artifact_replay.v1"));
    assert!(json["owned_runtime_surfaces"]
        .as_array()
        .expect("array")
        .iter()
        .any(|value| value == "adl identity continuous-verification"));
}

#[test]
fn identity_continuous_verification_validates_unknown_args_and_missing_out_value() {
    let repo = temp_repo("identity-continuous-verification-errors");

    let err = real_identity_in_repo(
        &["continuous-verification".to_string(), "--bogus".to_string()],
        &repo,
    )
    .expect_err("unknown arg should fail");
    assert!(err
        .to_string()
        .contains("unknown arg for identity continuous-verification: --bogus"));

    let err = real_identity_in_repo(
        &["continuous-verification".to_string(), "--out".to_string()],
        &repo,
    )
    .expect_err("out flag without value should fail");
    assert!(err.to_string().contains("--out requires a value"));
}
