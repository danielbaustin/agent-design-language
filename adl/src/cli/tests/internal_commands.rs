use super::*;

#[test]
fn real_learn_validates_subcommand_and_export_args() {
    let err = real_learn(&[]).expect_err("missing subcommand");
    assert!(err.to_string().contains("supported: export"));

    let err = real_learn(&["unknown".to_string()]).expect_err("unknown subcommand");
    assert!(err.to_string().contains("unknown learn subcommand"));

    let err = real_learn_export(&[
        "--format".to_string(),
        "csv".to_string(),
        "--out".to_string(),
        "/tmp/out".to_string(),
    ])
    .expect_err("unsupported format");
    assert!(err.to_string().contains("unsupported learn export format"));

    let err =
        real_learn_export(&["--format".to_string(), "jsonl".to_string()]).expect_err("missing out");
    assert!(err.to_string().contains("requires --out"));

    let err =
        real_learn_export(&["--bogus".to_string(), "x".to_string()]).expect_err("unknown arg");
    assert!(err.to_string().contains("unknown learn export arg"));
}

#[test]
fn cli_internal_keygen_sign_verify_roundtrip_succeeds() {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time")
        .as_nanos();
    let base = std::env::temp_dir().join(format!("adl-main-keygen-{now}"));
    let key_dir = base.join("keys");
    std::fs::create_dir_all(&base).expect("create base dir");
    real_keygen(&[
        "--out-dir".to_string(),
        key_dir.to_string_lossy().to_string(),
    ])
    .expect("keygen should succeed");

    let source =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/v0-5-pattern-linear.adl.yaml");
    let signed = base.join("signed.adl.yaml");
    real_sign(&[
        source.to_string_lossy().to_string(),
        "--key".to_string(),
        key_dir
            .join("ed25519-private.b64")
            .to_string_lossy()
            .to_string(),
        "--key-id".to_string(),
        "test-main".to_string(),
        "--out".to_string(),
        signed.to_string_lossy().to_string(),
    ])
    .expect("sign should succeed");

    real_verify(&[
        signed.to_string_lossy().to_string(),
        "--key".to_string(),
        key_dir
            .join("ed25519-public.b64")
            .to_string_lossy()
            .to_string(),
    ])
    .expect("verify should succeed");

    let _ = std::fs::remove_dir_all(base);
}

#[test]
fn cli_internal_instrument_variants_succeed() {
    let fixture =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/v0-5-pattern-fork-join.adl.yaml");
    real_instrument(&[
        "graph".to_string(),
        fixture.to_string_lossy().to_string(),
        "--format".to_string(),
        "json".to_string(),
    ])
    .expect("graph json");
    real_instrument(&[
        "graph".to_string(),
        fixture.to_string_lossy().to_string(),
        "--format".to_string(),
        "dot".to_string(),
    ])
    .expect("graph dot");
    real_instrument(&[
        "diff-plan".to_string(),
        fixture.to_string_lossy().to_string(),
        fixture.to_string_lossy().to_string(),
    ])
    .expect("diff-plan");
    let provider_fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("examples/v0-7-provider-portability-http-profile.adl.yaml");
    real_instrument(&[
        "provider-substrate".to_string(),
        provider_fixture.to_string_lossy().to_string(),
    ])
    .expect("provider-substrate");
    real_instrument(&["provider-substrate-schema".to_string()]).expect("provider-substrate-schema");

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time")
        .as_nanos();
    let base = std::env::temp_dir().join(format!("adl-main-instrument-{now}"));
    std::fs::create_dir_all(&base).expect("create base dir");
    let left = base.join("left.trace.json");
    let right = base.join("right.trace.json");
    std::fs::write(&left, "[]").expect("write left trace");
    std::fs::write(&right, "[]").expect("write right trace");
    real_instrument(&["replay".to_string(), left.to_string_lossy().to_string()]).expect("replay");
    real_instrument(&[
        "diff-trace".to_string(),
        left.to_string_lossy().to_string(),
        right.to_string_lossy().to_string(),
    ])
    .expect("diff-trace");
    real_instrument(&["trace-schema".to_string()]).expect("trace-schema");
    let trace_v1 = base.join("trace-v1.json");
    std::fs::write(
        &trace_v1,
        serde_json::to_string_pretty(&serde_json::json!({
            "schema_version": "trace.v1",
            "events": [
                {
                    "event_id": "run-start-1",
                    "timestamp": "2026-04-03T12:00:00Z",
                    "event_type": "RUN_START",
                    "trace_id": "trace-1",
                    "run_id": "run-1",
                    "span_id": "span-root",
                    "parent_span_id": null,
                    "actor": {"type": "agent", "id": "agent.main"},
                    "scope": {"level": "run", "name": "run"},
                    "inputs_ref": "artifacts/run-1/inputs.json",
                    "outputs_ref": null,
                    "artifact_ref": null,
                    "decision_context": null,
                    "provider": null,
                    "error": null,
                    "contract_validation": null
                },
                {
                    "event_id": "run-end-1",
                    "timestamp": "2026-04-03T12:00:01Z",
                    "event_type": "RUN_END",
                    "trace_id": "trace-1",
                    "run_id": "run-1",
                    "span_id": "span-root",
                    "parent_span_id": null,
                    "actor": {"type": "agent", "id": "agent.main"},
                    "scope": {"level": "run", "name": "run"},
                    "inputs_ref": null,
                    "outputs_ref": "artifacts/run-1/outputs.json",
                    "artifact_ref": null,
                    "decision_context": null,
                    "provider": null,
                    "error": null,
                    "contract_validation": null
                }
            ]
        }))
        .expect("serialize trace v1 fixture"),
    )
    .expect("write trace v1 fixture");
    real_instrument(&[
        "validate-trace-v1".to_string(),
        trace_v1.to_string_lossy().to_string(),
    ])
    .expect("validate-trace-v1");
    let _ = std::fs::remove_dir_all(base);
}

#[test]
fn cli_internal_learn_export_writes_jsonl() {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time")
        .as_nanos();
    let base = std::env::temp_dir().join(format!("adl-main-learn-{now}"));
    let runs_dir = base.join("runs");
    std::fs::create_dir_all(&runs_dir).expect("create runs dir");
    let out = base.join("learning.jsonl");
    real_learn_export(&[
        "--format".to_string(),
        "jsonl".to_string(),
        "--runs-dir".to_string(),
        runs_dir.to_string_lossy().to_string(),
        "--out".to_string(),
        out.to_string_lossy().to_string(),
    ])
    .expect("learn export");
    assert!(out.exists(), "learn export should emit output file");
    let tool_result = base.join("learning.jsonl.tool_result.v1.json");
    assert!(
        tool_result.exists(),
        "learn export should emit tool_result sidecar"
    );
    let tool_result_json: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&tool_result).expect("read tool_result"))
            .expect("parse tool_result");
    assert_eq!(
        tool_result_json
            .get("schema_version")
            .and_then(|v| v.as_str()),
        Some("tool_result.v1")
    );
    assert_eq!(
        tool_result_json.get("tool_name").and_then(|v| v.as_str()),
        Some("adl.learn.export")
    );
    assert_eq!(
        tool_result_json.get("status").and_then(|v| v.as_str()),
        Some("success")
    );
    let _ = std::fs::remove_dir_all(base);
}

#[test]
fn cli_internal_demo_print_plan_path_succeeds() {
    real_demo(&["demo-a-say-mcp".to_string(), "--print-plan".to_string()])
        .expect("known demo should succeed");
}

#[test]
fn cli_internal_demo_trace_only_path_succeeds() {
    real_demo(&["demo-a-say-mcp".to_string(), "--trace".to_string()])
        .expect("trace-only dry run should succeed");
}

#[test]
fn cli_internal_demo_run_no_open_path_succeeds() {
    let out_dir = unique_temp_dir("adl-demo-no-open");
    real_demo(&[
        "demo-b-one-command".to_string(),
        "--run".to_string(),
        "--no-open".to_string(),
        "--out".to_string(),
        out_dir.to_string_lossy().to_string(),
    ])
    .expect("demo run with explicit no-open should succeed");
    let _ = std::fs::remove_dir_all(out_dir);
}

#[test]
fn cli_internal_demo_help_path_succeeds() {
    real_demo(&["demo-a-say-mcp".to_string(), "--help".to_string()])
        .expect("help path should succeed");
}

#[test]
fn cli_internal_demo_defaults_to_run_when_no_mode_flag_is_given() {
    let out_dir = unique_temp_dir("adl-demo-default-run");
    real_demo(&[
        "demo-a-say-mcp".to_string(),
        "--no-open".to_string(),
        "--out".to_string(),
        out_dir.to_string_lossy().to_string(),
    ])
    .expect("default demo invocation should run");
    let _ = std::fs::remove_dir_all(out_dir);
}

#[test]
fn cli_internal_demo_run_trace_and_out_path_succeeds() {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time")
        .as_nanos();
    let out_dir = std::env::temp_dir().join(format!("adl-demo-out-{now}"));
    real_demo(&[
        "demo-a-say-mcp".to_string(),
        "--run".to_string(),
        "--trace".to_string(),
        "--no-open".to_string(),
        "--out".to_string(),
        out_dir.to_string_lossy().to_string(),
    ])
    .expect("demo run with trace and explicit out dir should succeed");
    assert!(out_dir.join("demo-a-say-mcp").exists());
    let _ = std::fs::remove_dir_all(out_dir);
}

#[test]
fn validate_pause_artifact_basic_rejects_mismatches() {
    let mk = || PauseStateArtifact {
        schema_version: PAUSE_STATE_SCHEMA_VERSION.to_string(),
        run_id: "run-1".to_string(),
        workflow_id: "wf".to_string(),
        version: "0.5".to_string(),
        status: "paused".to_string(),
        adl_path: "adl/examples/v0-6-hitl-pause-resume.adl.yaml".to_string(),
        execution_plan_hash: "abc".to_string(),
        steering_history: Vec::new(),
        pause: execute::PauseState {
            paused_step_id: "s1".to_string(),
            reason: None,
            completed_step_ids: vec!["s1".to_string()],
            remaining_step_ids: vec![],
            saved_state: HashMap::new(),
            completed_outputs: HashMap::new(),
        },
    };

    let mut wrong_schema = mk();
    wrong_schema.schema_version = "pause_state.v0".to_string();
    assert!(validate_pause_artifact_basic(&wrong_schema, "run-1").is_err());

    let mut wrong_status = mk();
    wrong_status.status = "success".to_string();
    assert!(validate_pause_artifact_basic(&wrong_status, "run-1").is_err());

    let mut wrong_run = mk();
    wrong_run.run_id = "run-2".to_string();
    assert!(validate_pause_artifact_basic(&wrong_run, "run-1").is_err());
}

#[test]
fn resume_state_path_for_run_id_targets_pause_state_json() {
    let _runs_guard = EnvGuard::set("ADL_RUNS_ROOT", "");
    let path = resume_state_path_for_run_id("demo-run").expect("path");
    let s = path.to_string_lossy();
    assert!(
        s.ends_with(".adl/runs/demo-run/pause_state.json"),
        "path={s}"
    );
}

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
