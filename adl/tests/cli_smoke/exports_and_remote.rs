use super::*;

#[test]
fn learn_export_jsonl_is_deterministic() {
    let d = unique_test_temp_dir("learn-export");
    let runs = d.join("runs");
    let run = runs.join("r1");
    fs::create_dir_all(run.join("learning")).unwrap();

    fs::write(
        run.join("run_summary.json"),
        r#"{"workflow_id":"wf","adl_version":"0.7","swarm_version":"0.6.0","status":"success"}"#,
    )
    .unwrap();
    fs::write(
        run.join("steps.json"),
        r#"[{"step_id":"s1","provider_id":"p1","status":"success","output_artifact_path":"/tmp/o1"},{"step_id":"s2","provider_id":"p2","status":"failure","output_artifact_path":"/tmp/o2"}]"#,
    )
    .unwrap();
    fs::write(
        run.join("learning").join("scores.json"),
        r#"{"summary":{"success_ratio":0.5,"retry_count":1,"failure_count":1}}"#,
    )
    .unwrap();
    fs::write(
        run.join("learning").join("suggestions.json"),
        r#"{"suggestions":[{"id":"sug-002","category":"security"},{"id":"sug-001","category":"retry"}]}"#,
    )
    .unwrap();

    let out1 = d.join("export-1.jsonl");
    let out2 = d.join("export-2.jsonl");
    let one = run_adl(&[
        "learn",
        "export",
        "--format",
        "jsonl",
        "--runs-dir",
        runs.to_str().unwrap(),
        "--out",
        out1.to_str().unwrap(),
    ]);
    let two = run_adl(&[
        "learn",
        "export",
        "--format",
        "jsonl",
        "--runs-dir",
        runs.to_str().unwrap(),
        "--out",
        out2.to_str().unwrap(),
    ]);

    assert!(
        one.status.success(),
        "stderr:\n{}",
        String::from_utf8_lossy(&one.stderr)
    );
    assert!(
        two.status.success(),
        "stderr:\n{}",
        String::from_utf8_lossy(&two.stderr)
    );
    assert_eq!(
        fs::read(&out1).unwrap(),
        fs::read(&out2).unwrap(),
        "learn export jsonl should be byte-identical across repeated exports"
    );
}

#[test]
fn learn_export_jsonl_has_no_secrets_or_absolute_paths() {
    let d = unique_test_temp_dir("learn-export-redact");
    let runs = d.join("runs");
    let run = runs.join("r1");
    fs::create_dir_all(run.join("learning")).unwrap();

    fs::write(
        run.join("run_summary.json"),
        r#"{"workflow_id":"wf","adl_version":"0.7","swarm_version":"0.6.0","status":"success"}"#,
    )
    .unwrap();
    fs::write(
        run.join("steps.json"),
        r#"[{"step_id":"s1","provider_id":"p1","status":"success","output_artifact_path":"/Users/name/private/path.txt"}]"#,
    )
    .unwrap();

    let out = d.join("export.jsonl");
    let cmd = run_adl(&[
        "learn",
        "export",
        "--format",
        "jsonl",
        "--runs-dir",
        runs.to_str().unwrap(),
        "--out",
        out.to_str().unwrap(),
    ]);
    assert!(
        cmd.status.success(),
        "stderr:\n{}",
        String::from_utf8_lossy(&cmd.stderr)
    );

    let body = fs::read_to_string(out).unwrap();
    assert!(
        !body.contains("/Users/"),
        "export must not leak absolute host paths: {body}"
    );
    assert!(
        !body.contains("gho_"),
        "export must not leak token-like secrets: {body}"
    );
}

#[test]
fn learn_export_bundle_v1_is_deterministic() {
    let d = unique_test_temp_dir("learn-export-bundle");
    let runs = d.join("runs");
    let run = runs.join("r1");
    fs::create_dir_all(run.join("learning")).unwrap();

    fs::write(
        run.join("run_summary.json"),
        r#"{"workflow_id":"wf","adl_version":"0.7","swarm_version":"0.6.0","status":"success"}"#,
    )
    .unwrap();
    fs::write(
        run.join("steps.json"),
        r#"[{"step_id":"s2","provider_id":"p2","status":"failure","output_artifact_path":"/tmp/o2"},{"step_id":"s1","provider_id":"p1","status":"success","output_artifact_path":"/tmp/o1"}]"#,
    )
    .unwrap();
    fs::write(
        run.join("learning").join("scores.json"),
        r#"{"summary":{"success_ratio":0.5,"retry_count":1,"failure_count":1}}"#,
    )
    .unwrap();
    fs::write(
        run.join("learning").join("suggestions.json"),
        r#"{"suggestions":[{"id":"sug-002","category":"security"},{"id":"sug-001","category":"retry"}]}"#,
    )
    .unwrap();

    let out1 = d.join("bundle-1");
    let out2 = d.join("bundle-2");

    let one = run_adl(&[
        "learn",
        "export",
        "--format",
        "bundle-v1",
        "--runs-dir",
        runs.to_str().unwrap(),
        "--out",
        out1.to_str().unwrap(),
    ]);
    let two = run_adl(&[
        "learn",
        "export",
        "--format",
        "bundle-v1",
        "--runs-dir",
        runs.to_str().unwrap(),
        "--out",
        out2.to_str().unwrap(),
    ]);

    assert!(
        one.status.success(),
        "stderr:\n{}",
        String::from_utf8_lossy(&one.stderr)
    );
    assert!(
        two.status.success(),
        "stderr:\n{}",
        String::from_utf8_lossy(&two.stderr)
    );

    let manifest1 = fs::read(out1.join("learning_export_v1").join("manifest.json")).unwrap();
    let manifest2 = fs::read(out2.join("learning_export_v1").join("manifest.json")).unwrap();
    assert_eq!(
        manifest1, manifest2,
        "bundle manifest should be deterministic"
    );

    assert!(out1
        .join("learning_export_v1")
        .join("runs")
        .join("r1")
        .join("metadata.json")
        .is_file());
    assert!(out1
        .join("learning_export_v1")
        .join("runs")
        .join("r1")
        .join("step_records.json")
        .is_file());
    assert!(out1
        .join("learning_export_v1")
        .join("runs")
        .join("r1")
        .join("scores_summary.json")
        .is_file());
    assert!(out1
        .join("learning_export_v1")
        .join("runs")
        .join("r1")
        .join("suggestions_summary.json")
        .is_file());
}

#[test]
fn learn_export_bundle_v1_has_no_secrets_or_absolute_paths() {
    let d = unique_test_temp_dir("learn-export-bundle-redact");
    let runs = d.join("runs");
    let run = runs.join("r1");
    fs::create_dir_all(run.join("learning")).unwrap();

    fs::write(
        run.join("run_summary.json"),
        r#"{"workflow_id":"wf","adl_version":"0.7","swarm_version":"0.6.0","status":"success"}"#,
    )
    .unwrap();
    fs::write(
        run.join("steps.json"),
        r#"[{"step_id":"s1","provider_id":"p1","status":"success","output_artifact_path":"/Users/name/private/path.txt"}]"#,
    )
    .unwrap();
    fs::write(
        run.join("learning").join("suggestions.json"),
        r#"{"suggestions":[{"id":"sug-001","category":"retry"}]}"#,
    )
    .unwrap();

    let out = d.join("bundle");
    let cmd = run_adl(&[
        "learn",
        "export",
        "--format",
        "bundle-v1",
        "--runs-dir",
        runs.to_str().unwrap(),
        "--out",
        out.to_str().unwrap(),
    ]);
    assert!(
        cmd.status.success(),
        "stderr:\n{}",
        String::from_utf8_lossy(&cmd.stderr)
    );

    let mut all_json = String::new();
    let bundle_root = out.join("learning_export_v1");
    for rel in [
        "manifest.json",
        "runs/r1/metadata.json",
        "runs/r1/step_records.json",
        "runs/r1/suggestions_summary.json",
    ] {
        all_json.push_str(&fs::read_to_string(bundle_root.join(rel)).unwrap());
        all_json.push('\n');
    }

    assert!(
        !all_json.contains("/Users/") && !all_json.contains("/home/"),
        "bundle export must not leak absolute host paths: {all_json}"
    );
    assert!(
        !all_json.contains("gho_"),
        "bundle export must not leak token-like secrets: {all_json}"
    );
}

#[test]
fn learn_export_trace_bundle_v2_is_deterministic_and_sanitized() {
    let d = unique_test_temp_dir("trace-bundle-v2");
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
        r#"{"activation_log_version":1,"ordering":"append_only_emission_order","stable_ids":{"step_id":"replay_stable_with_same_plan","delegation_id":"replay_stable_with_same_activation_log","run_id":"run_scoped_not_cross_run_stable"},"events":[{"RunStarted":{"ts":"2026-03-01T00:00:00.000Z","run_id":"r1","workflow_id":"wf","version":"0.75"}}]}"#,
    )
    .unwrap();
    fs::write(
        run.join("logs").join("trace_v1.json"),
        r#"{"schema_version":"trace.v1","events":[{"event_id":"trace-v1-0001","timestamp":"2026-03-01T00:00:00.000Z","event_type":"RUN_START","trace_id":"r1","run_id":"r1","span_id":"run:r1","parent_span_id":null,"actor":{"type":"agent","id":"wf"},"scope":{"level":"run","name":"wf"},"inputs_ref":"artifacts/r1/run.json","outputs_ref":"artifacts/r1/logs/trace_v1.json","artifact_ref":"artifacts/r1/run.json","decision_context":null,"provider":null,"error":null,"contract_validation":null},{"event_id":"trace-v1-0002","timestamp":"2026-03-01T00:00:01.000Z","event_type":"RUN_END","trace_id":"r1","run_id":"r1","span_id":"run:r1","parent_span_id":null,"actor":{"type":"agent","id":"wf"},"scope":{"level":"run","name":"wf"},"inputs_ref":"artifacts/r1/run.json","outputs_ref":"artifacts/r1/steps.json","artifact_ref":"artifacts/r1/logs/trace_v1.json","decision_context":{"context":"run completion","outcome":"success","rationale":null},"provider":null,"error":null,"contract_validation":null}]}"#,
    )
    .unwrap();

    let out1 = d.join("trace-bundle-1");
    let out2 = d.join("trace-bundle-2");
    let one = run_adl(&[
        "learn",
        "export",
        "--format",
        "trace-bundle-v2",
        "--runs-dir",
        runs.to_str().unwrap(),
        "--out",
        out1.to_str().unwrap(),
    ]);
    let two = run_adl(&[
        "learn",
        "export",
        "--format",
        "trace-bundle-v2",
        "--runs-dir",
        runs.to_str().unwrap(),
        "--out",
        out2.to_str().unwrap(),
    ]);
    assert!(
        one.status.success(),
        "stderr:\n{}",
        String::from_utf8_lossy(&one.stderr)
    );
    assert!(
        two.status.success(),
        "stderr:\n{}",
        String::from_utf8_lossy(&two.stderr)
    );

    let manifest1 = fs::read(out1.join("trace_bundle_v2").join("manifest.json")).unwrap();
    let manifest2 = fs::read(out2.join("trace_bundle_v2").join("manifest.json")).unwrap();
    assert_eq!(
        manifest1, manifest2,
        "trace bundle v2 manifest should be deterministic"
    );

    let mut all_json = String::new();
    for rel in [
        "manifest.json",
        "runs/r1/metadata.json",
        "runs/r1/run_summary.json",
        "runs/r1/logs/activation_log.json",
        "runs/r1/logs/trace_v1.json",
    ] {
        all_json.push_str(&fs::read_to_string(out1.join("trace_bundle_v2").join(rel)).unwrap());
        all_json.push('\n');
    }
    assert!(
        !all_json.contains("/Users/") && !all_json.contains("/home/"),
        "trace bundle v2 must not leak host paths: {all_json}"
    );
    assert!(
        !all_json.contains("gho_") && !all_json.contains("sk-"),
        "trace bundle v2 must not leak secret-like tokens: {all_json}"
    );
}

#[test]
fn adl_remote_rejects_invalid_bind_deterministically() {
    let Ok(adl_remote) = std::env::var("CARGO_BIN_EXE_adl_remote") else {
        return;
    };
    let adl_out = Command::new(adl_remote)
        .arg("127.0.0.1:not-a-port")
        .output()
        .expect("run adl-remote");
    assert!(
        !adl_out.status.success(),
        "adl-remote should fail on invalid bind"
    );
    let adl_stderr = String::from_utf8_lossy(&adl_out.stderr);
    assert!(
        adl_stderr.contains("failed to bind remote server"),
        "stderr:\n{adl_stderr}"
    );
}
