use super::super::*;

#[test]
fn cli_artifact_validate_control_path_accepts_demo_fixture() {
    let out_dir = unique_temp_dir("adl-control-path-validate-pass");
    real_demo(&[
        "demo-g-v086-control-path".to_string(),
        "--run".to_string(),
        "--no-open".to_string(),
        "--out".to_string(),
        out_dir.to_string_lossy().to_string(),
    ])
    .expect("control-path demo should succeed");

    let control_path_root = out_dir.join("demo-g-v086-control-path");
    real_artifact(&[
        "validate-control-path".to_string(),
        "--root".to_string(),
        control_path_root.to_string_lossy().to_string(),
    ])
    .expect("validator should accept canonical control-path fixture");

    let _ = std::fs::remove_dir_all(out_dir);
}

fn materialize_control_path_demo(name: &str) -> (std::path::PathBuf, std::path::PathBuf) {
    let out_dir = unique_temp_dir(name);
    real_demo(&[
        "demo-g-v086-control-path".to_string(),
        "--run".to_string(),
        "--no-open".to_string(),
        "--out".to_string(),
        out_dir.to_string_lossy().to_string(),
    ])
    .expect("control-path demo should succeed");

    let control_path_root = out_dir.join("demo-g-v086-control-path");

    (out_dir, control_path_root)
}

fn rewrite_json_artifact(
    control_path_root: &std::path::Path,
    name: &str,
    value: &serde_json::Value,
) {
    std::fs::write(
        control_path_root.join(name),
        serde_json::to_vec_pretty(value).expect("serialize artifact rewrite"),
    )
    .expect("rewrite control-path artifact");
}

fn expect_control_path_artifact_validation_error<F>(
    control_path_root: &std::path::Path,
    artifact_name: &str,
    expected_substring: &str,
    mutate: F,
) where
    F: FnOnce(&mut serde_json::Value),
{
    let artifact_path = control_path_root.join(artifact_name);
    let original_artifact =
        std::fs::read_to_string(&artifact_path).expect("read control-path artifact");
    let mut artifact: serde_json::Value =
        serde_json::from_str(&original_artifact).expect("parse control-path artifact");
    mutate(&mut artifact);
    rewrite_json_artifact(control_path_root, artifact_name, &artifact);

    let err = real_artifact(&[
        "validate-control-path".to_string(),
        "--root".to_string(),
        control_path_root.to_string_lossy().to_string(),
    ])
    .expect_err("validator should reject mutated control-path artifact");
    assert!(
        err.to_string().contains(expected_substring),
        "expected '{expected_substring}' in error, got: {err}"
    );

    std::fs::write(&artifact_path, original_artifact).expect("restore control-path artifact");
}

fn expect_two_control_path_artifacts_validation_error<F, G>(
    control_path_root: &std::path::Path,
    first_artifact_name: &str,
    second_artifact_name: &str,
    expected_substring: &str,
    mutate_first: F,
    mutate_second: G,
) where
    F: FnOnce(&mut serde_json::Value),
    G: FnOnce(&mut serde_json::Value),
{
    let first_artifact_path = control_path_root.join(first_artifact_name);
    let second_artifact_path = control_path_root.join(second_artifact_name);
    let original_first_artifact =
        std::fs::read_to_string(&first_artifact_path).expect("read first control-path artifact");
    let original_second_artifact =
        std::fs::read_to_string(&second_artifact_path).expect("read second control-path artifact");
    let mut first_artifact: serde_json::Value =
        serde_json::from_str(&original_first_artifact).expect("parse first control-path artifact");
    let mut second_artifact: serde_json::Value = serde_json::from_str(&original_second_artifact)
        .expect("parse second control-path artifact");
    mutate_first(&mut first_artifact);
    mutate_second(&mut second_artifact);
    rewrite_json_artifact(control_path_root, first_artifact_name, &first_artifact);
    rewrite_json_artifact(control_path_root, second_artifact_name, &second_artifact);

    let err = real_artifact(&[
        "validate-control-path".to_string(),
        "--root".to_string(),
        control_path_root.to_string_lossy().to_string(),
    ])
    .expect_err("validator should reject mutated control-path artifacts");
    assert!(
        err.to_string().contains(expected_substring),
        "expected '{expected_substring}' in error, got: {err}"
    );

    std::fs::write(&first_artifact_path, original_first_artifact)
        .expect("restore first control-path artifact");
    std::fs::write(&second_artifact_path, original_second_artifact)
        .expect("restore second control-path artifact");
}

fn expect_security_review_validation_error<F>(
    control_path_root: &std::path::Path,
    original_security_review: &str,
    expected_substring: &str,
    mutate: F,
) where
    F: FnOnce(&mut serde_json::Value),
{
    std::fs::write(
        control_path_root.join("security_review.json"),
        original_security_review,
    )
    .expect("restore security review artifact");
    let mut security_review: serde_json::Value =
        serde_json::from_str(original_security_review).expect("parse security review");
    mutate(&mut security_review);
    rewrite_json_artifact(control_path_root, "security_review.json", &security_review);

    let err = real_artifact(&[
        "validate-control-path".to_string(),
        "--root".to_string(),
        control_path_root.to_string_lossy().to_string(),
    ])
    .expect_err("validator should reject mutated security review");
    assert!(
        err.to_string().contains(expected_substring),
        "expected '{expected_substring}' in error, got: {err}"
    );
}

#[test]
fn cli_artifact_requires_subcommand() {
    let err = real_artifact(&[]).expect_err("artifact should require a subcommand");
    assert!(err
        .to_string()
        .contains("artifact requires a subcommand: validate-control-path"));
}

#[test]
fn cli_artifact_rejects_unknown_subcommand() {
    let err = real_artifact(&["unknown".to_string()]).expect_err("unknown subcommand");
    assert!(err
        .to_string()
        .contains("unknown artifact subcommand 'unknown'"));
}

#[test]
fn cli_artifact_validate_control_path_requires_root_flag() {
    let err = real_artifact(&["validate-control-path".to_string()])
        .expect_err("validate-control-path should require --root");
    assert!(err
        .to_string()
        .contains("artifact validate-control-path requires --root <dir>"));
}

#[test]
fn cli_artifact_validate_control_path_rejects_unknown_arg() {
    let err = real_artifact(&["validate-control-path".to_string(), "--bogus".to_string()])
        .expect_err("validate-control-path should reject unknown args");
    assert!(err
        .to_string()
        .contains("unknown arg for artifact validate-control-path: --bogus"));
}

#[test]
fn cli_artifact_validate_control_path_rejects_missing_root() {
    let missing_root = unique_temp_dir("adl-control-path-validate-missing-root");
    std::fs::remove_dir_all(&missing_root).expect("remove temporary root");

    let err = real_artifact(&[
        "validate-control-path".to_string(),
        "--root".to_string(),
        missing_root.to_string_lossy().to_string(),
    ])
    .expect_err("validator should reject missing artifact root");
    assert!(err
        .to_string()
        .contains("control-path artifact root does not exist"));
}

#[test]
fn cli_artifact_validate_control_path_rejects_missing_required_artifact() {
    let out_dir = unique_temp_dir("adl-control-path-validate-fail");
    real_demo(&[
        "demo-g-v086-control-path".to_string(),
        "--run".to_string(),
        "--no-open".to_string(),
        "--out".to_string(),
        out_dir.to_string_lossy().to_string(),
    ])
    .expect("control-path demo should succeed");

    let control_path_root = out_dir.join("demo-g-v086-control-path");
    std::fs::remove_file(control_path_root.join("memory.json")).expect("remove memory artifact");

    let err = real_artifact(&[
        "validate-control-path".to_string(),
        "--root".to_string(),
        control_path_root.to_string_lossy().to_string(),
    ])
    .expect_err("validator should reject missing required artifact");
    assert!(err
        .to_string()
        .contains("missing required control-path artifact"));

    let _ = std::fs::remove_dir_all(out_dir);
}

#[test]
fn cli_artifact_validate_control_path_rejects_missing_decisions_artifact() {
    let out_dir = unique_temp_dir("adl-control-path-validate-missing-decisions");
    real_demo(&[
        "demo-g-v086-control-path".to_string(),
        "--run".to_string(),
        "--no-open".to_string(),
        "--out".to_string(),
        out_dir.to_string_lossy().to_string(),
    ])
    .expect("control-path demo should succeed");

    let control_path_root = out_dir.join("demo-g-v086-control-path");
    std::fs::remove_file(control_path_root.join("decisions.json"))
        .expect("remove decisions artifact");

    let err = real_artifact(&[
        "validate-control-path".to_string(),
        "--root".to_string(),
        control_path_root.to_string_lossy().to_string(),
    ])
    .expect_err("validator should reject missing decisions artifact");
    assert!(err
        .to_string()
        .contains("missing required control-path artifact"));

    let _ = std::fs::remove_dir_all(out_dir);
}

#[test]
fn cli_artifact_validate_control_path_rejects_missing_action_proposals_artifact() {
    let out_dir = unique_temp_dir("adl-control-path-validate-missing-proposals");
    real_demo(&[
        "demo-g-v086-control-path".to_string(),
        "--run".to_string(),
        "--no-open".to_string(),
        "--out".to_string(),
        out_dir.to_string_lossy().to_string(),
    ])
    .expect("control-path demo should succeed");

    let control_path_root = out_dir.join("demo-g-v086-control-path");
    std::fs::remove_file(control_path_root.join("action_proposals.json"))
        .expect("remove action proposal artifact");

    let err = real_artifact(&[
        "validate-control-path".to_string(),
        "--root".to_string(),
        control_path_root.to_string_lossy().to_string(),
    ])
    .expect_err("validator should reject missing action proposal artifact");
    assert!(err
        .to_string()
        .contains("missing required control-path artifact"));

    let _ = std::fs::remove_dir_all(out_dir);
}

#[test]
fn cli_artifact_validate_control_path_rejects_missing_mediation_artifact() {
    let out_dir = unique_temp_dir("adl-control-path-validate-missing-mediation");
    real_demo(&[
        "demo-g-v086-control-path".to_string(),
        "--run".to_string(),
        "--no-open".to_string(),
        "--out".to_string(),
        out_dir.to_string_lossy().to_string(),
    ])
    .expect("control-path demo should succeed");

    let control_path_root = out_dir.join("demo-g-v086-control-path");
    std::fs::remove_file(control_path_root.join("mediation.json"))
        .expect("remove mediation artifact");

    let err = real_artifact(&[
        "validate-control-path".to_string(),
        "--root".to_string(),
        control_path_root.to_string_lossy().to_string(),
    ])
    .expect_err("validator should reject missing mediation artifact");
    assert!(err
        .to_string()
        .contains("missing required control-path artifact"));

    let _ = std::fs::remove_dir_all(out_dir);
}

#[test]
fn cli_artifact_validate_control_path_rejects_missing_skill_model_artifact() {
    let out_dir = unique_temp_dir("adl-control-path-validate-missing-skill-model");
    real_demo(&[
        "demo-g-v086-control-path".to_string(),
        "--run".to_string(),
        "--no-open".to_string(),
        "--out".to_string(),
        out_dir.to_string_lossy().to_string(),
    ])
    .expect("control-path demo should succeed");

    let control_path_root = out_dir.join("demo-g-v086-control-path");
    std::fs::remove_file(control_path_root.join("skill_model.json"))
        .expect("remove skill model artifact");

    let err = real_artifact(&[
        "validate-control-path".to_string(),
        "--root".to_string(),
        control_path_root.to_string_lossy().to_string(),
    ])
    .expect_err("validator should reject missing skill model artifact");
    assert!(err
        .to_string()
        .contains("missing required control-path artifact"));

    let _ = std::fs::remove_dir_all(out_dir);
}

#[test]
fn cli_artifact_validate_control_path_rejects_missing_skill_execution_protocol_artifact() {
    let out_dir = unique_temp_dir("adl-control-path-validate-missing-skill-protocol");
    real_demo(&[
        "demo-g-v086-control-path".to_string(),
        "--run".to_string(),
        "--no-open".to_string(),
        "--out".to_string(),
        out_dir.to_string_lossy().to_string(),
    ])
    .expect("control-path demo should succeed");

    let control_path_root = out_dir.join("demo-g-v086-control-path");
    std::fs::remove_file(control_path_root.join("skill_execution_protocol.json"))
        .expect("remove skill execution protocol artifact");

    let err = real_artifact(&[
        "validate-control-path".to_string(),
        "--root".to_string(),
        control_path_root.to_string_lossy().to_string(),
    ])
    .expect_err("validator should reject missing skill execution protocol artifact");
    assert!(err
        .to_string()
        .contains("missing required control-path artifact"));

    let _ = std::fs::remove_dir_all(out_dir);
}

#[test]
fn cli_artifact_validate_control_path_rejects_missing_security_review_artifact() {
    let out_dir = unique_temp_dir("adl-control-path-validate-missing-security-review");
    real_demo(&[
        "demo-g-v086-control-path".to_string(),
        "--run".to_string(),
        "--no-open".to_string(),
        "--out".to_string(),
        out_dir.to_string_lossy().to_string(),
    ])
    .expect("control-path demo should succeed");

    let control_path_root = out_dir.join("demo-g-v086-control-path");
    std::fs::remove_file(control_path_root.join("security_review.json"))
        .expect("remove security review artifact");

    let err = real_artifact(&[
        "validate-control-path".to_string(),
        "--root".to_string(),
        control_path_root.to_string_lossy().to_string(),
    ])
    .expect_err("validator should reject missing security review artifact");
    assert!(err
        .to_string()
        .contains("missing required control-path artifact"));

    let _ = std::fs::remove_dir_all(out_dir);
}

#[test]
fn cli_artifact_validate_control_path_rejects_missing_run_summary_sibling() {
    let (out_dir, control_path_root) =
        materialize_control_path_demo("adl-control-path-validate-missing-run-summary");
    std::fs::remove_file(control_path_root.join("run_summary.json"))
        .expect("remove run summary sibling artifact");

    let err = real_artifact(&[
        "validate-control-path".to_string(),
        "--root".to_string(),
        control_path_root.to_string_lossy().to_string(),
    ])
    .expect_err("validator should reject missing run summary sibling");
    assert!(err
        .to_string()
        .contains("missing required control-path sibling artifact 'run_summary.json'"));

    let _ = std::fs::remove_dir_all(out_dir);
}

#[test]
fn cli_artifact_validate_control_path_rejects_malformed_artifact() {
    let out_dir = unique_temp_dir("adl-control-path-validate-malformed");
    real_demo(&[
        "demo-g-v086-control-path".to_string(),
        "--run".to_string(),
        "--no-open".to_string(),
        "--out".to_string(),
        out_dir.to_string_lossy().to_string(),
    ])
    .expect("control-path demo should succeed");

    let control_path_root = out_dir.join("demo-g-v086-control-path");
    std::fs::write(
        control_path_root.join("final_result.json"),
        "{\"run_id\":\"broken\"}",
    )
    .expect("rewrite malformed final_result artifact");

    let err = real_artifact(&[
        "validate-control-path".to_string(),
        "--root".to_string(),
        control_path_root.to_string_lossy().to_string(),
    ])
    .expect_err("validator should reject malformed artifact");
    assert!(err.to_string().contains("invalid control-path artifact"));

    let _ = std::fs::remove_dir_all(out_dir);
}

#[test]
fn cli_artifact_validate_control_path_rejects_empty_summary() {
    let (out_dir, control_path_root) =
        materialize_control_path_demo("adl-control-path-validate-empty-summary");
    std::fs::write(control_path_root.join("summary.txt"), "\n\n")
        .expect("empty control-path summary");

    let err = real_artifact(&[
        "validate-control-path".to_string(),
        "--root".to_string(),
        control_path_root.to_string_lossy().to_string(),
    ])
    .expect_err("validator should reject empty summary");
    assert!(err.to_string().contains("control-path summary is empty"));

    let _ = std::fs::remove_dir_all(out_dir);
}

#[test]
fn cli_artifact_validate_control_path_rejects_security_review_mismatches() {
    let (out_dir, control_path_root) =
        materialize_control_path_demo("adl-control-path-validate-security-review-mismatch");
    let original_security_review =
        std::fs::read_to_string(control_path_root.join("security_review.json"))
            .expect("read security review artifact");

    expect_security_review_validation_error(
        &control_path_root,
        &original_security_review,
        "control-path security review posture",
        |security_review| {
            security_review["posture"]["declared_posture"] = serde_json::json!("unsafe");
        },
    );
    expect_security_review_validation_error(
        &control_path_root,
        &original_security_review,
        "control-path security review attacker_pressure",
        |security_review| {
            security_review["threat_model"]["attacker_pressure"] = serde_json::json!("benign");
        },
    );
    expect_security_review_validation_error(
        &control_path_root,
        &original_security_review,
        "control-path security review accepted_risk_level",
        |security_review| {
            security_review["posture"]["accepted_risk_level"] = serde_json::json!("low");
        },
    );
    expect_security_review_validation_error(
        &control_path_root,
        &original_security_review,
        "control-path security review commitment_policy",
        |security_review| {
            security_review["posture"]["commitment_policy"] = serde_json::json!("allow");
        },
    );
    expect_security_review_validation_error(
        &control_path_root,
        &original_security_review,
        "control-path security review mitigation_authority",
        |security_review| {
            security_review["posture"]["mitigation_authority"] = serde_json::json!("operator");
        },
    );
    expect_security_review_validation_error(
        &control_path_root,
        &original_security_review,
        "control-path security review trust_state",
        |security_review| {
            security_review["trust_under_adversary"]["trust_state"] =
                serde_json::json!("fully_trusted");
        },
    );
    expect_security_review_validation_error(
        &control_path_root,
        &original_security_review,
        "control-path security review boundaries mismatch",
        |security_review| {
            security_review["threat_model"]["active_trust_boundaries"] =
                serde_json::json!(["operator_only"]);
        },
    );
    expect_security_review_validation_error(
        &control_path_root,
        &original_security_review,
        "control-path security review threat classes mismatch",
        |security_review| {
            security_review["threat_model"]["canonical_threat_classes"] =
                serde_json::json!(["tampering_only"]);
        },
    );
    expect_security_review_validation_error(
        &control_path_root,
        &original_security_review,
        "control-path security review mitigations mismatch",
        |security_review| {
            security_review["threat_model"]["required_mitigations"] =
                serde_json::json!(["manual_review"]);
        },
    );
    expect_security_review_validation_error(
        &control_path_root,
        &original_security_review,
        "control-path security review proof surfaces mismatch",
        |security_review| {
            security_review["threat_model"]["reviewer_visible_surfaces"] =
                serde_json::json!(["control_path/final_result.json"]);
        },
    );
    expect_security_review_validation_error(
        &control_path_root,
        &original_security_review,
        "control-path security review trusted surfaces mismatch",
        |security_review| {
            security_review["trust_under_adversary"]["trusted_surfaces"] =
                serde_json::json!(["control_path/final_result.json"]);
        },
    );
    expect_security_review_validation_error(
        &control_path_root,
        &original_security_review,
        "control-path security review reduced trust surfaces mismatch",
        |security_review| {
            security_review["trust_under_adversary"]["reduced_trust_surfaces"] =
                serde_json::json!(["control_path/final_result.json"]);
        },
    );
    expect_security_review_validation_error(
        &control_path_root,
        &original_security_review,
        "control-path security review revalidation requirements mismatch",
        |security_review| {
            security_review["trust_under_adversary"]["revalidation_requirements"] =
                serde_json::json!(["manual_recheck"]);
        },
    );
    expect_security_review_validation_error(
        &control_path_root,
        &original_security_review,
        "control-path security review escalation_path",
        |security_review| {
            security_review["trust_under_adversary"]["escalation_path"] =
                serde_json::json!("approve_immediately");
        },
    );
    expect_security_review_validation_error(
        &control_path_root,
        &original_security_review,
        "control-path security review evidence route",
        |security_review| {
            security_review["evidence"]["route_selected"] = serde_json::json!("fast");
        },
    );
    expect_security_review_validation_error(
        &control_path_root,
        &original_security_review,
        "control-path security review evidence risk_class",
        |security_review| {
            security_review["evidence"]["risk_class"] = serde_json::json!("low");
        },
    );
    expect_security_review_validation_error(
        &control_path_root,
        &original_security_review,
        "control-path security review evidence mediation_outcome",
        |security_review| {
            security_review["evidence"]["mediation_outcome"] = serde_json::json!("approved");
        },
    );
    expect_security_review_validation_error(
        &control_path_root,
        &original_security_review,
        "control-path security review evidence gate_decision",
        |security_review| {
            security_review["evidence"]["gate_decision"] = serde_json::json!("allow");
        },
    );
    expect_security_review_validation_error(
        &control_path_root,
        &original_security_review,
        "control-path security review evidence final_result",
        |security_review| {
            security_review["evidence"]["final_result"] = serde_json::json!("allow");
        },
    );
    expect_security_review_validation_error(
        &control_path_root,
        &original_security_review,
        "control-path security review evidence security_denied_count",
        |security_review| {
            security_review["evidence"]["security_denied_count"] = serde_json::json!(99);
        },
    );
    expect_security_review_validation_error(
        &control_path_root,
        &original_security_review,
        "control-path security review evidence security_envelope_enabled",
        |security_review| {
            security_review["evidence"]["security_envelope_enabled"] = serde_json::json!(false);
        },
    );
    expect_security_review_validation_error(
        &control_path_root,
        &original_security_review,
        "control-path security review evidence signing_required",
        |security_review| {
            security_review["evidence"]["signing_required"] = serde_json::json!(false);
        },
    );
    expect_security_review_validation_error(
        &control_path_root,
        &original_security_review,
        "control-path security review evidence key_id_required",
        |security_review| {
            security_review["evidence"]["key_id_required"] = serde_json::json!(false);
        },
    );
    expect_security_review_validation_error(
        &control_path_root,
        &original_security_review,
        "control-path security review evidence verify_allowed_algs mismatch",
        |security_review| {
            security_review["evidence"]["verify_allowed_algs"] = serde_json::json!(["rsa"]);
        },
    );
    expect_security_review_validation_error(
        &control_path_root,
        &original_security_review,
        "control-path security review evidence verify_allowed_key_sources mismatch",
        |security_review| {
            security_review["evidence"]["verify_allowed_key_sources"] =
                serde_json::json!(["local"]);
        },
    );
    expect_security_review_validation_error(
        &control_path_root,
        &original_security_review,
        "control-path security review evidence sandbox_policy",
        |security_review| {
            security_review["evidence"]["sandbox_policy"] = serde_json::json!("none");
        },
    );
    expect_security_review_validation_error(
        &control_path_root,
        &original_security_review,
        "control-path security review evidence trace_visibility_expectation",
        |security_review| {
            security_review["evidence"]["trace_visibility_expectation"] =
                serde_json::json!("hidden");
        },
    );

    let _ = std::fs::write(
        control_path_root.join("security_review.json"),
        original_security_review,
    );
    let _ = std::fs::remove_dir_all(out_dir);
}

#[test]
fn cli_artifact_validate_control_path_rejects_final_result_and_convergence_mismatches() {
    let (out_dir, control_path_root) =
        materialize_control_path_demo("adl-control-path-validate-final-result-mismatch");

    expect_control_path_artifact_validation_error(
        &control_path_root,
        "final_result.json",
        "control-path final_result stage_order mismatch",
        |final_result| {
            final_result["stage_order"] = serde_json::json!(["signals", "final_result"]);
        },
    );
    expect_control_path_artifact_validation_error(
        &control_path_root,
        "final_result.json",
        "control-path artifact run_id mismatch",
        |final_result| {
            final_result["run_id"] = serde_json::json!("other-run");
        },
    );
    expect_control_path_artifact_validation_error(
        &control_path_root,
        "final_result.json",
        "control-path final_result route",
        |final_result| {
            final_result["route_selected"] = serde_json::json!("fast");
        },
    );
    expect_control_path_artifact_validation_error(
        &control_path_root,
        "final_result.json",
        "control-path final_result selected_candidate",
        |final_result| {
            final_result["selected_candidate"] = serde_json::json!("cand-other");
        },
    );
    expect_control_path_artifact_validation_error(
        &control_path_root,
        "final_result.json",
        "control-path final_result termination_reason",
        |final_result| {
            final_result["termination_reason"] = serde_json::json!("max_iterations");
        },
    );
    expect_control_path_artifact_validation_error(
        &control_path_root,
        "final_result.json",
        "control-path final_result gate_decision",
        |final_result| {
            final_result["gate_decision"] = serde_json::json!("defer");
        },
    );
    expect_control_path_artifact_validation_error(
        &control_path_root,
        "final_result.json",
        "control-path final_result next_control_action",
        |final_result| {
            final_result["next_control_action"] = serde_json::json!("continue");
        },
    );
    expect_control_path_artifact_validation_error(
        &control_path_root,
        "convergence.json",
        "control-path convergence selected_candidate_id",
        |convergence| {
            convergence["selected_candidate_id"] = serde_json::json!("cand-other");
        },
    );
    expect_control_path_artifact_validation_error(
        &control_path_root,
        "convergence.json",
        "control-path convergence termination_reason",
        |convergence| {
            convergence["termination_reason"] = serde_json::json!("max_iterations");
        },
    );
    expect_control_path_artifact_validation_error(
        &control_path_root,
        "convergence.json",
        "control-path convergence gate_decision",
        |convergence| {
            convergence["gate_decision"] = serde_json::json!("defer");
        },
    );
    expect_control_path_artifact_validation_error(
        &control_path_root,
        "convergence.json",
        "control-path convergence next_control_action",
        |convergence| {
            convergence["next_control_action"] = serde_json::json!("continue");
        },
    );

    let _ = std::fs::remove_dir_all(out_dir);
}

#[test]
fn cli_artifact_validate_control_path_rejects_decision_and_proposal_mismatches() {
    let (out_dir, control_path_root) =
        materialize_control_path_demo("adl-control-path-validate-decision-proposal-mismatch");

    expect_control_path_artifact_validation_error(
        &control_path_root,
        "decisions.json",
        "control-path decisions schema fields mismatch",
        |decisions| {
            decisions["decision_schema_fields"] = serde_json::json!(["decision_id"]);
        },
    );
    expect_control_path_artifact_validation_error(
        &control_path_root,
        "decisions.json",
        "control-path decisions outcome vocabulary mismatch",
        |decisions| {
            decisions["outcome_class_vocabulary"] = serde_json::json!(["accept"]);
        },
    );
    expect_control_path_artifact_validation_error(
        &control_path_root,
        "decisions.json",
        "control-path decisions artifact must contain exactly 3 surfaces and 3 records",
        |decisions| {
            decisions["surfaces"] = serde_json::json!([]);
        },
    );
    expect_control_path_artifact_validation_error(
        &control_path_root,
        "action_proposals.json",
        "control-path action proposal schema fields mismatch",
        |action_proposals| {
            action_proposals["proposal_schema_fields"] = serde_json::json!(["proposal_id"]);
        },
    );
    expect_control_path_artifact_validation_error(
        &control_path_root,
        "action_proposals.json",
        "control-path action proposal vocabulary mismatch",
        |action_proposals| {
            action_proposals["proposal_kind_vocabulary"] = serde_json::json!(["tool_call"]);
        },
    );
    expect_control_path_artifact_validation_error(
        &control_path_root,
        "action_proposals.json",
        "control-path action proposals artifact must contain exactly 1 bounded proposal",
        |action_proposals| {
            let proposal = action_proposals["proposals"][0].clone();
            action_proposals["proposals"] = serde_json::json!([proposal.clone(), proposal]);
        },
    );
    expect_control_path_artifact_validation_error(
        &control_path_root,
        "action_proposals.json",
        "must remain non-authoritative",
        |action_proposals| {
            action_proposals["proposals"][0]["non_authoritative"] = serde_json::json!(false);
        },
    );
    expect_control_path_artifact_validation_error(
        &control_path_root,
        "action_proposals.json",
        "is not in the declared vocabulary",
        |action_proposals| {
            action_proposals["proposals"][0]["kind"] = serde_json::json!("unknown_kind");
        },
    );

    let _ = std::fs::remove_dir_all(out_dir);
}

#[test]
fn cli_artifact_validate_control_path_rejects_mediation_mismatches() {
    let (out_dir, control_path_root) =
        materialize_control_path_demo("adl-control-path-validate-mediation-mismatch");

    expect_control_path_artifact_validation_error(
        &control_path_root,
        "mediation.json",
        "control-path mediation outcome vocabulary mismatch",
        |mediation| {
            mediation["mediation_outcome_vocabulary"] = serde_json::json!(["approved"]);
        },
    );
    expect_control_path_artifact_validation_error(
        &control_path_root,
        "mediation.json",
        "control-path mediation authority boundary mismatch",
        |mediation| {
            mediation["authority_boundary"] = serde_json::json!("model_decides");
        },
    );
    expect_control_path_artifact_validation_error(
        &control_path_root,
        "mediation.json",
        "control-path mediation proposal",
        |mediation| {
            mediation["mediation"]["proposal_id"] = serde_json::json!("proposal.other");
        },
    );
    expect_two_control_path_artifacts_validation_error(
        &control_path_root,
        "mediation.json",
        "security_review.json",
        "control-path mediation runtime authority",
        |mediation| {
            mediation["mediation"]["runtime_authority"] = serde_json::json!("model");
        },
        |security_review| {
            security_review["posture"]["mitigation_authority"] = serde_json::json!("model");
        },
    );
    expect_two_control_path_artifacts_validation_error(
        &control_path_root,
        "mediation.json",
        "security_review.json",
        "control-path mediation outcome",
        |mediation| {
            mediation["mediation"]["mediation_outcome"] = serde_json::json!("deferred");
        },
        |security_review| {
            security_review["evidence"]["mediation_outcome"] = serde_json::json!("deferred");
        },
    );
    expect_control_path_artifact_validation_error(
        &control_path_root,
        "mediation.json",
        "control-path mediation decision_id",
        |mediation| {
            mediation["mediation"]["decision_id"] = serde_json::json!("decision.other");
        },
    );
    expect_control_path_artifact_validation_error(
        &control_path_root,
        "mediation.json",
        "control-path mediation temporal anchor",
        |mediation| {
            mediation["mediation"]["temporal_anchor"] =
                serde_json::json!("control_path/mediation.json");
        },
    );
    expect_control_path_artifact_validation_error(
        &control_path_root,
        "mediation.json",
        "control-path mediation judgment_boundary",
        |mediation| {
            mediation["mediation"]["judgment_boundary"] = serde_json::json!("other_boundary");
        },
    );
    expect_control_path_artifact_validation_error(
        &control_path_root,
        "mediation.json",
        "control-path mediation required_follow_up",
        |mediation| {
            mediation["mediation"]["required_follow_up"] = serde_json::json!("none");
        },
    );
    expect_control_path_artifact_validation_error(
        &control_path_root,
        "mediation.json",
        "must not carry approved_action_or_none when outcome is not approved",
        |mediation| {
            mediation["mediation"]["approved_action_or_none"] =
                serde_json::json!("candidate.review_and_refine");
        },
    );

    let _ = std::fs::remove_dir_all(out_dir);
}

#[test]
fn cli_artifact_validate_control_path_rejects_skill_model_schema_field_mismatch() {
    let (out_dir, control_path_root) =
        materialize_control_path_demo("adl-control-path-validate-skill-model-schema");
    let mut skill_model: serde_json::Value = serde_json::from_slice(
        &std::fs::read(control_path_root.join("skill_model.json")).expect("read skill model"),
    )
    .expect("parse skill model");
    skill_model["skill_schema_fields"] = serde_json::json!(["wrong_field"]);
    rewrite_json_artifact(&control_path_root, "skill_model.json", &skill_model);

    let err = real_artifact(&[
        "validate-control-path".to_string(),
        "--root".to_string(),
        control_path_root.to_string_lossy().to_string(),
    ])
    .expect_err("validator should reject skill model schema mismatch");
    assert!(err
        .to_string()
        .contains("control-path skill model schema fields mismatch"));

    let _ = std::fs::remove_dir_all(out_dir);
}

#[test]
fn cli_artifact_validate_control_path_rejects_skill_model_selection_status_mismatch() {
    let (out_dir, control_path_root) =
        materialize_control_path_demo("adl-control-path-validate-skill-model-selection");
    let mut skill_model: serde_json::Value = serde_json::from_slice(
        &std::fs::read(control_path_root.join("skill_model.json")).expect("read skill model"),
    )
    .expect("parse skill model");
    skill_model["skill"]["selection_status"] = serde_json::json!("not_selected");
    rewrite_json_artifact(&control_path_root, "skill_model.json", &skill_model);

    let err = real_artifact(&[
        "validate-control-path".to_string(),
        "--root".to_string(),
        control_path_root.to_string_lossy().to_string(),
    ])
    .expect_err("validator should reject mismatched skill selection status");
    assert!(err
        .to_string()
        .contains("control-path skill model selection_status"));

    let _ = std::fs::remove_dir_all(out_dir);
}

#[test]
fn cli_artifact_validate_control_path_rejects_skill_model_temporal_anchor_mismatch() {
    let (out_dir, control_path_root) =
        materialize_control_path_demo("adl-control-path-validate-skill-model-anchor");
    let mut skill_model: serde_json::Value = serde_json::from_slice(
        &std::fs::read(control_path_root.join("skill_model.json")).expect("read skill model"),
    )
    .expect("parse skill model");
    skill_model["skill"]["temporal_anchor"] = serde_json::json!("control_path/mediation.json");
    rewrite_json_artifact(&control_path_root, "skill_model.json", &skill_model);

    let err = real_artifact(&[
        "validate-control-path".to_string(),
        "--root".to_string(),
        control_path_root.to_string_lossy().to_string(),
    ])
    .expect_err("validator should reject mismatched skill model temporal anchor");
    assert!(err
        .to_string()
        .contains("control-path skill model temporal anchor"));

    let _ = std::fs::remove_dir_all(out_dir);
}

#[test]
fn cli_artifact_validate_control_path_rejects_skill_execution_protocol_stage_mismatch() {
    let (out_dir, control_path_root) =
        materialize_control_path_demo("adl-control-path-validate-skill-protocol-stages");
    let mut skill_protocol: serde_json::Value = serde_json::from_slice(
        &std::fs::read(control_path_root.join("skill_execution_protocol.json"))
            .expect("read skill execution protocol"),
    )
    .expect("parse skill execution protocol");
    skill_protocol["lifecycle_stages"] = serde_json::json!(["proposed", "authorized"]);
    rewrite_json_artifact(
        &control_path_root,
        "skill_execution_protocol.json",
        &skill_protocol,
    );

    let err = real_artifact(&[
        "validate-control-path".to_string(),
        "--root".to_string(),
        control_path_root.to_string_lossy().to_string(),
    ])
    .expect_err("validator should reject skill execution protocol stage mismatch");
    assert!(err
        .to_string()
        .contains("control-path skill execution protocol stages mismatch"));

    let _ = std::fs::remove_dir_all(out_dir);
}

#[test]
fn cli_artifact_validate_control_path_rejects_skill_execution_protocol_authorization_mismatch() {
    let (out_dir, control_path_root) =
        materialize_control_path_demo("adl-control-path-validate-skill-protocol-authorization");
    let mut skill_protocol: serde_json::Value = serde_json::from_slice(
        &std::fs::read(control_path_root.join("skill_execution_protocol.json"))
            .expect("read skill execution protocol"),
    )
    .expect("parse skill execution protocol");
    skill_protocol["invocation"]["authorization_decision"] = serde_json::json!("rejected");
    rewrite_json_artifact(
        &control_path_root,
        "skill_execution_protocol.json",
        &skill_protocol,
    );

    let err = real_artifact(&[
        "validate-control-path".to_string(),
        "--root".to_string(),
        control_path_root.to_string_lossy().to_string(),
    ])
    .expect_err("validator should reject skill execution protocol authorization mismatch");
    assert!(err
        .to_string()
        .contains("control-path skill execution protocol authorization_decision"));

    let _ = std::fs::remove_dir_all(out_dir);
}

#[test]
fn cli_artifact_validate_control_path_rejects_skill_execution_protocol_trace_expectation_mismatch()
{
    let (out_dir, control_path_root) =
        materialize_control_path_demo("adl-control-path-validate-skill-protocol-trace");
    let mut skill_protocol: serde_json::Value = serde_json::from_slice(
        &std::fs::read(control_path_root.join("skill_execution_protocol.json"))
            .expect("read skill execution protocol"),
    )
    .expect("parse skill execution protocol");
    skill_protocol["invocation"]["trace_expectation"] = serde_json::json!("not_visible");
    rewrite_json_artifact(
        &control_path_root,
        "skill_execution_protocol.json",
        &skill_protocol,
    );

    let err = real_artifact(&[
        "validate-control-path".to_string(),
        "--root".to_string(),
        control_path_root.to_string_lossy().to_string(),
    ])
    .expect_err("validator should reject skill execution protocol trace expectation mismatch");
    assert!(err
        .to_string()
        .contains("control-path skill execution protocol trace expectation"));

    let _ = std::fs::remove_dir_all(out_dir);
}

#[test]
fn cli_artifact_validate_control_path_rejects_skill_execution_protocol_temporal_anchor_mismatch() {
    let (out_dir, control_path_root) =
        materialize_control_path_demo("adl-control-path-validate-skill-protocol-anchor");
    let mut skill_protocol: serde_json::Value = serde_json::from_slice(
        &std::fs::read(control_path_root.join("skill_execution_protocol.json"))
            .expect("read skill execution protocol"),
    )
    .expect("parse skill execution protocol");
    skill_protocol["invocation"]["temporal_anchor"] =
        serde_json::json!("control_path/action_proposals.json");
    rewrite_json_artifact(
        &control_path_root,
        "skill_execution_protocol.json",
        &skill_protocol,
    );

    let err = real_artifact(&[
        "validate-control-path".to_string(),
        "--root".to_string(),
        control_path_root.to_string_lossy().to_string(),
    ])
    .expect_err("validator should reject skill execution protocol temporal anchor mismatch");
    assert!(err
        .to_string()
        .contains("control-path skill execution protocol temporal anchor"));

    let _ = std::fs::remove_dir_all(out_dir);
}
