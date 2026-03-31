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
