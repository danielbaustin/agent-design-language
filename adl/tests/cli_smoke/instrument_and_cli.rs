use super::*;

#[test]
fn instrument_graph_output_is_stable() {
    let path = fixture_path("examples/v0-5-pattern-fork-join.adl.yaml");
    let out1 = run_adl(&[
        "instrument",
        "graph",
        path.to_str().unwrap(),
        "--format",
        "json",
    ]);
    let out2 = run_adl(&[
        "instrument",
        "graph",
        path.to_str().unwrap(),
        "--format",
        "json",
    ]);

    assert!(
        out1.status.success() && out2.status.success(),
        "expected success, stderr1:\n{}\nstderr2:\n{}",
        String::from_utf8_lossy(&out1.stderr),
        String::from_utf8_lossy(&out2.stderr)
    );
    assert_eq!(
        out1.stdout, out2.stdout,
        "expected deterministic graph export output"
    );
}

#[test]
fn instrument_replay_and_diff_trace_outputs_are_stable() {
    let d = unique_test_temp_dir("instrument-replay-diff");
    let trace_a = d.join("trace-a.json");
    let trace_b = d.join("trace-b.json");

    let trace_json = r#"[
  {
    "kind": "StepStarted",
    "step_id": "s1",
    "agent_id": "a",
    "provider_id": "p",
    "task_id": "t",
    "delegation_json": null
  },
  {
    "kind": "StepOutputChunk",
    "step_id": "s1",
    "chunk_bytes": 12
  },
  {
    "kind": "StepFinished",
    "step_id": "s1",
    "success": true
  }
]"#;

    fs::write(&trace_a, trace_json).expect("write trace_a");
    fs::write(&trace_b, trace_json).expect("write trace_b");

    let replay1 = run_adl(&["instrument", "replay", trace_a.to_str().unwrap()]);
    let replay2 = run_adl(&["instrument", "replay", trace_a.to_str().unwrap()]);
    assert!(
        replay1.status.success() && replay2.status.success(),
        "expected success, stderr1:\n{}\nstderr2:\n{}",
        String::from_utf8_lossy(&replay1.stderr),
        String::from_utf8_lossy(&replay2.stderr)
    );
    assert_eq!(
        replay1.stdout, replay2.stdout,
        "expected stable replay output"
    );

    let diff1 = run_adl(&[
        "instrument",
        "diff-trace",
        trace_a.to_str().unwrap(),
        trace_b.to_str().unwrap(),
    ]);
    let diff2 = run_adl(&[
        "instrument",
        "diff-trace",
        trace_a.to_str().unwrap(),
        trace_b.to_str().unwrap(),
    ]);
    assert!(
        diff1.status.success() && diff2.status.success(),
        "expected success, stderr1:\n{}\nstderr2:\n{}",
        String::from_utf8_lossy(&diff1.stderr),
        String::from_utf8_lossy(&diff2.stderr)
    );
    assert_eq!(
        diff1.stdout, diff2.stdout,
        "expected stable trace diff output"
    );
}

#[test]
fn run_flag_executes_fixture_with_mock_provider_and_writes_outputs() {
    let out_dir = unique_test_temp_dir("cli-run-mock").join("out");
    let runs_root = unique_test_temp_dir("cli-run-mock-runs");
    let fixture = fixture_path("examples/v0-6-hitl-no-pause.adl.yaml");
    let mock = fixture_path("tools/mock_ollama_v0_4.sh");
    let out = run_adl_with_env(
        &[
            fixture.to_str().unwrap(),
            "--run",
            "--allow-unsigned",
            "--out",
            out_dir.to_str().unwrap(),
        ],
        &[
            ("ADL_OLLAMA_BIN", mock.to_str().unwrap()),
            ("ADL_RUNS_ROOT", runs_root.to_str().unwrap()),
        ],
    );
    assert!(
        out.status.success(),
        "stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(out_dir.join("s1.txt").is_file(), "missing s1.txt");
    assert!(out_dir.join("s2.txt").is_file(), "missing s2.txt");
    assert!(out_dir.join("s3.txt").is_file(), "missing s3.txt");
}

#[test]
fn run_flag_honors_no_step_output_alias() {
    let out_dir = unique_test_temp_dir("cli-run-no-step-output").join("out");
    let runs_root = unique_test_temp_dir("cli-run-no-step-output-runs");
    let fixture = fixture_path("examples/v0-6-hitl-no-pause.adl.yaml");
    let mock = fixture_path("tools/mock_ollama_v0_4.sh");
    let out = run_adl_with_env(
        &[
            fixture.to_str().unwrap(),
            "--run",
            "--allow-unsigned",
            "--no-step-output",
            "--out",
            out_dir.to_str().unwrap(),
        ],
        &[
            ("ADL_OLLAMA_BIN", mock.to_str().unwrap()),
            ("ADL_RUNS_ROOT", runs_root.to_str().unwrap()),
        ],
    );
    assert!(
        out.status.success(),
        "stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(out_dir.join("s1.txt").is_file(), "missing s1.txt");
}

#[test]
fn demo_command_accepts_no_open_flag_for_print_plan() {
    let out = run_adl(&["demo", "demo-b-one-command", "--print-plan", "--no-open"]);
    assert!(
        out.status.success(),
        "stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("Demo: demo-b-one-command"),
        "stdout:\n{stdout}"
    );
}

#[test]
fn resume_unknown_run_id_fails_with_pause_state_message() {
    let fixture = fixture_path("examples/v0-5-pattern-linear.adl.yaml");
    let out = run_adl(&[
        "resume",
        "does-not-exist-475",
        "--adl",
        fixture.to_str().unwrap(),
    ]);
    assert!(!out.status.success(), "resume should fail");
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("pause_state.json") || stderr.contains("missing"),
        "stderr:\n{stderr}"
    );
}

#[test]
fn instrument_graph_dot_and_invalid_format_branches_are_covered() {
    let path = fixture_path("examples/v0-5-pattern-linear.adl.yaml");
    let dot = run_adl(&[
        "instrument",
        "graph",
        path.to_str().unwrap(),
        "--format",
        "dot",
    ]);
    assert!(
        dot.status.success(),
        "dot stderr:\n{}",
        String::from_utf8_lossy(&dot.stderr)
    );
    let dot_stdout = String::from_utf8_lossy(&dot.stdout);
    assert!(dot_stdout.contains("digraph"), "stdout:\n{dot_stdout}");

    let bad = run_adl(&[
        "instrument",
        "graph",
        path.to_str().unwrap(),
        "--format",
        "xml",
    ]);
    assert!(!bad.status.success(), "invalid format should fail");
    let bad_stderr = String::from_utf8_lossy(&bad.stderr);
    assert!(
        bad_stderr.contains("unsupported --format"),
        "stderr:\n{bad_stderr}"
    );
}

#[test]
fn instrument_replay_rejects_extra_argument() {
    let d = unique_test_temp_dir("instrument-replay-extra");
    let trace = d.join("trace.json");
    fs::write(&trace, "[]").expect("write trace");
    let out = run_adl(&["instrument", "replay", trace.to_str().unwrap(), "extra"]);
    assert!(!out.status.success(), "replay with extra arg should fail");
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("accepts exactly one <trace.json>"),
        "stderr:\n{stderr}"
    );
}

#[test]
fn instrument_replay_bundle_rejects_invalid_arguments() {
    assert_failure_contains(
        &run_adl(&["instrument", "replay-bundle"]),
        "instrument replay-bundle requires <bundle_dir> <run_id>",
    );
    assert_failure_contains(
        &run_adl(&["instrument", "replay-bundle", "/tmp/trace_bundle_v2"]),
        "instrument replay-bundle requires <bundle_dir> <run_id>",
    );
    assert_failure_contains(
        &run_adl(&[
            "instrument",
            "replay-bundle",
            "/tmp/trace_bundle_v2",
            "run1",
            "extra",
        ]),
        "instrument replay-bundle accepts exactly <bundle_dir> <run_id>",
    );
}

#[test]
fn instrument_replay_bundle_from_trace_bundle_v2_is_stable() {
    let d = unique_test_temp_dir("instrument-replay-bundle");
    let runs = d.join("runs");
    let run = runs.join("r1");
    fs::create_dir_all(run.join("logs")).unwrap();
    fs::create_dir_all(run.join("learning")).unwrap();

    fs::write(
        run.join("run.json"),
        r#"{"schema_version":"run_state.v1","run_id":"r1","workflow_id":"wf","version":"0.75","status":"success","error_message":null,"start_time_ms":1,"end_time_ms":2,"duration_ms":1,"execution_plan_hash":"abc","scheduler_max_concurrency":null,"scheduler_policy_source":null,"pause":null}"#,
    )
    .unwrap();
    fs::write(
        run.join("steps.json"),
        r#"[{"step_id":"s1","agent_id":"a","provider_id":"p","status":"success","output_artifact_path":"outputs/s1.txt"}]"#,
    )
    .unwrap();
    fs::write(
        run.join("run_summary.json"),
        r#"{"run_summary_version":1,"artifact_model_version":1,"run_id":"r1","workflow_id":"wf","adl_version":"0.75","swarm_version":"0.7.0","status":"success","counts":{"total_steps":1,"completed_steps":1,"failed_steps":0,"provider_call_count":1,"delegation_steps":0,"delegation_requires_verification_steps":0},"policy":{"security_envelope_enabled":false,"signing_required":false,"key_id_required":false,"verify_allowed_algs":[],"verify_allowed_key_sources":[],"sandbox_policy":"centralized_path_resolver_v1","security_denials_by_code":{}},"links":{"run_json":"run.json","steps_json":"steps.json","outputs_dir":"outputs","logs_dir":"logs","learning_dir":"learning","overlays_dir":"learning/overlays"}}"#,
    )
    .unwrap();
    fs::write(
        run.join("run_status.json"),
        r#"{"run_status_version":1,"run_id":"r1","workflow_id":"wf","overall_status":"succeeded","failure_kind":null,"completed_steps":["s1"],"pending_steps":[],"attempt_counts_by_step":{"s1":1}}"#,
    )
    .unwrap();
    fs::write(
        run.join("logs").join("activation_log.json"),
        r#"{"activation_log_version":1,"ordering":"append_only_emission_order","stable_ids":{"step_id":"replay_stable_with_same_plan","delegation_id":"replay_stable_with_same_activation_log","run_id":"run_scoped_not_cross_run_stable"},"events":[{"kind":"StepStarted","step_id":"s1","agent_id":"a","provider_id":"p","task_id":"t","delegation_json":null},{"kind":"StepFinished","step_id":"s1","success":true}]}"#,
    )
    .unwrap();

    let bundle_out = d.join("bundle");
    let export = run_adl(&[
        "learn",
        "export",
        "--format",
        "trace-bundle-v2",
        "--runs-dir",
        runs.to_str().unwrap(),
        "--out",
        bundle_out.to_str().unwrap(),
    ]);
    assert!(
        export.status.success(),
        "export stderr:\n{}",
        String::from_utf8_lossy(&export.stderr)
    );

    let bundle_dir = bundle_out.join("trace_bundle_v2");
    let replay1 = run_adl(&[
        "instrument",
        "replay-bundle",
        bundle_dir.to_str().unwrap(),
        "r1",
    ]);
    let replay2 = run_adl(&[
        "instrument",
        "replay-bundle",
        bundle_dir.to_str().unwrap(),
        "r1",
    ]);
    assert!(
        replay1.status.success() && replay2.status.success(),
        "stderr1:\n{}\nstderr2:\n{}",
        String::from_utf8_lossy(&replay1.stderr),
        String::from_utf8_lossy(&replay2.stderr)
    );
    assert_eq!(
        replay1.stdout, replay2.stdout,
        "replay-from-bundle output should be deterministic"
    );
}

#[test]
fn instrument_replay_bundle_rejects_tampered_bundle() {
    let d = unique_test_temp_dir("instrument-replay-bundle-tamper");
    let runs = d.join("runs");
    let run = runs.join("r1");
    fs::create_dir_all(run.join("logs")).unwrap();
    fs::write(
        run.join("run.json"),
        r#"{"schema_version":"run_state.v1","run_id":"r1","workflow_id":"wf","version":"0.75","status":"success","error_message":null,"start_time_ms":1,"end_time_ms":2,"duration_ms":1,"execution_plan_hash":"abc","scheduler_max_concurrency":null,"scheduler_policy_source":null,"pause":null}"#,
    )
    .unwrap();
    fs::write(run.join("steps.json"), r#"[]"#).unwrap();
    fs::write(
        run.join("run_summary.json"),
        r#"{"run_summary_version":1,"artifact_model_version":1,"run_id":"r1","workflow_id":"wf","adl_version":"0.75","swarm_version":"0.7.0","status":"success","counts":{"total_steps":0,"completed_steps":0,"failed_steps":0,"provider_call_count":0,"delegation_steps":0,"delegation_requires_verification_steps":0},"policy":{"security_envelope_enabled":false,"signing_required":false,"key_id_required":false,"verify_allowed_algs":[],"verify_allowed_key_sources":[],"sandbox_policy":"centralized_path_resolver_v1","security_denials_by_code":{}},"links":{"run_json":"run.json","steps_json":"steps.json","outputs_dir":"outputs","logs_dir":"logs","learning_dir":"learning","overlays_dir":"learning/overlays"}}"#,
    )
    .unwrap();
    fs::write(
        run.join("run_status.json"),
        r#"{"run_status_version":1,"run_id":"r1","workflow_id":"wf","overall_status":"succeeded","failure_kind":null,"completed_steps":[],"pending_steps":[],"attempt_counts_by_step":{}}"#,
    )
    .unwrap();
    fs::write(
        run.join("logs").join("activation_log.json"),
        r#"{"activation_log_version":1,"ordering":"append_only_emission_order","stable_ids":{"step_id":"x","delegation_id":"x","run_id":"x"},"events":[]}"#,
    )
    .unwrap();

    let bundle_out = d.join("bundle");
    let export = run_adl(&[
        "learn",
        "export",
        "--format",
        "trace-bundle-v2",
        "--runs-dir",
        runs.to_str().unwrap(),
        "--out",
        bundle_out.to_str().unwrap(),
    ]);
    assert!(
        export.status.success(),
        "export stderr:\n{}",
        String::from_utf8_lossy(&export.stderr)
    );
    let activation = bundle_out
        .join("trace_bundle_v2")
        .join("runs")
        .join("r1")
        .join("logs")
        .join("activation_log.json");
    fs::write(&activation, b"{\"tampered\":true}").unwrap();

    let out = run_adl(&[
        "instrument",
        "replay-bundle",
        bundle_out.join("trace_bundle_v2").to_str().unwrap(),
        "r1",
    ]);
    assert!(!out.status.success(), "tampered bundle should fail");
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("hash mismatch") || stderr.contains("size mismatch"),
        "stderr:\n{stderr}"
    );
}

#[test]
fn instrument_replay_schema_mismatch_emits_stable_replay_failure_code() {
    let d = unique_test_temp_dir("instrument-replay-schema-mismatch");
    let trace = d.join("trace.json");
    fs::write(
        &trace,
        r#"{
  "activation_log_version": 1,
  "ordering": "unordered",
  "stable_ids": {
    "step_id": "stable within resolved execution plan",
    "delegation_id": "deterministic per run: del-<counter>",
    "run_id": "run-scoped identifier; not replay-stable across independent runs"
  },
  "events": []
}"#,
    )
    .expect("write invalid ordering trace");
    let out = run_adl(&["instrument", "replay", trace.to_str().unwrap()]);
    assert!(
        !out.status.success(),
        "replay with invalid ordering should fail"
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("REPLAY_INVARIANT_VIOLATION"),
        "stderr should contain stable replay failure code; stderr:\n{stderr}"
    );
}

#[test]
fn learn_subcommand_requires_supported_export_only() {
    let missing = run_adl(&["learn"]);
    assert!(
        !missing.status.success(),
        "learn without subcommand should fail"
    );
    let missing_stderr = String::from_utf8_lossy(&missing.stderr);
    assert!(
        missing_stderr.contains("learn subcommand required"),
        "stderr:\n{missing_stderr}"
    );

    let unsupported = run_adl(&["learn", "export", "--format", "csv", "--out", "x.jsonl"]);
    assert!(
        !unsupported.status.success(),
        "unsupported format should fail"
    );
    let unsupported_stderr = String::from_utf8_lossy(&unsupported.stderr);
    assert!(
        unsupported_stderr.contains("unsupported learn export format"),
        "stderr:\n{unsupported_stderr}"
    );
}

#[test]
fn keygen_sign_verify_argument_errors_are_deterministic() {
    assert_failure_contains(&run_adl(&["keygen", "--bogus"]), "Unknown arg for keygen");
    assert_failure_contains(&run_adl(&["sign"]), "sign requires <adl.yaml>");
    assert_failure_contains(
        &run_adl(&["sign", "examples/v0-5-pattern-linear.adl.yaml"]),
        "sign requires --key <private_key_path>",
    );
    assert_failure_contains(
        &run_adl(&["verify", "examples/v0-5-pattern-linear.adl.yaml", "--bogus"]),
        "Unknown arg for verify",
    );
    let help_keygen = run_adl(&["keygen", "--help"]);
    assert!(help_keygen.status.success(), "keygen --help should succeed");
}

#[test]
fn instrument_diff_subcommands_validate_required_args() {
    assert_failure_contains(
        &run_adl(&[
            "instrument",
            "diff-plan",
            "examples/v0-5-pattern-linear.adl.yaml",
        ]),
        "instrument diff-plan requires <left.adl.yaml> <right.adl.yaml>",
    );
    assert_failure_contains(
        &run_adl(&["instrument", "diff-trace", "/tmp/a.trace.json"]),
        "instrument diff-trace requires <left.trace.json> <right.trace.json>",
    );
    assert_failure_contains(
        &run_adl(&[
            "instrument",
            "graph",
            "examples/v0-5-pattern-linear.adl.yaml",
            "--format",
        ]),
        "instrument graph requires --format <json|dot>",
    );
}

#[test]
fn learn_export_value_validation_covers_missing_values() {
    assert_failure_contains(
        &run_adl(&["learn", "export", "--format"]),
        "--format requires a value",
    );
    assert_failure_contains(
        &run_adl(&["learn", "export", "--runs-dir"]),
        "--runs-dir requires a directory path",
    );
    assert_failure_contains(
        &run_adl(&["learn", "export", "--out"]),
        "--out requires a path",
    );
    assert_failure_contains(
        &run_adl(&["learn", "export", "--run-id"]),
        "--run-id requires a value",
    );
}

#[test]
fn sign_and_verify_missing_option_values_are_reported() {
    let src = "examples/v0-5-pattern-linear.adl.yaml";
    assert_failure_contains(
        &run_adl(&["sign", src, "--key"]),
        "sign requires --key <private_key_path>",
    );
    assert_failure_contains(
        &run_adl(&["sign", src, "--out"]),
        "sign requires --out <signed_file>",
    );
    assert_failure_contains(
        &run_adl(&["sign", src, "--key-id"]),
        "sign requires --key-id <id>",
    );
    assert_failure_contains(
        &run_adl(&["verify", src, "--key"]),
        "verify requires --key <public_key_path>",
    );
}

#[test]
fn legacy_resume_flag_path_validation_fails_deterministically() {
    assert_failure_contains(
        &run_adl(&["examples/v0-5-pattern-linear.adl.yaml", "--run", "--resume"]),
        "--resume requires a run.json path",
    );
    assert_failure_contains(
        &run_adl(&[
            "examples/v0-5-pattern-linear.adl.yaml",
            "--run",
            "--overlay",
            "/tmp/does-not-exist-overlay.json",
        ]),
        "failed to read overlay file",
    );
}

#[test]
fn demo_subcommand_validates_name_and_unknown_flag() {
    assert_failure_contains(&run_adl(&["demo"]), "missing demo name");
    assert_failure_contains(
        &run_adl(&["demo", "demo-a-say-mcp", "--bogus"]),
        "Unknown arg: --bogus",
    );
}

#[test]
fn demo_subcommand_requires_out_value() {
    assert_failure_contains(
        &run_adl(&["demo", "demo-a-say-mcp", "--out"]),
        "--out requires a directory path",
    );
}
