use super::shared::{
    discover_run_ids, resolve_export_ids, stable_fingerprint_hex, validate_bundle_rel_path,
};
use super::*;

#[test]
fn export_jsonl_deterministic_for_fixture_runs() {
    let base = std::env::temp_dir().join(format!("learn-export-{}", std::process::id()));
    let runs_root = base.join("runs");
    let run_dir = runs_root.join("r1");
    std::fs::create_dir_all(run_dir.join("learning")).unwrap();
    std::fs::write(
        run_dir.join("run_summary.json"),
        r#"{"workflow_id":"wf","adl_version":"0.7","swarm_version":"0.6.0","status":"success"}"#,
    )
    .unwrap();
    std::fs::write(
        run_dir.join("steps.json"),
        r#"[{"step_id":"s1","provider_id":"p1","status":"success","output_artifact_path":"/tmp/out.txt"}]"#,
    )
    .unwrap();
    std::fs::write(
        run_dir.join("learning").join("scores.json"),
        r#"{"summary":{"success_ratio":1.0,"retry_count":0}}"#,
    )
    .unwrap();
    std::fs::write(
        run_dir.join("learning").join("suggestions.json"),
        r#"{"suggestions":[{"id":"sug-002","category":"b"},{"id":"sug-001","category":"a"}]}"#,
    )
    .unwrap();

    let out1 = base.join("one.jsonl");
    let out2 = base.join("two.jsonl");
    export_jsonl(&runs_root, &[], &out1).unwrap();
    export_jsonl(&runs_root, &[], &out2).unwrap();
    let a = std::fs::read(&out1).unwrap();
    let b = std::fs::read(&out2).unwrap();
    assert_eq!(a, b, "export jsonl must be byte-stable");
}

#[test]
fn export_bundle_v1_is_deterministic_and_path_safe() {
    let base = std::env::temp_dir().join(format!("learn-bundle-{}", std::process::id()));
    let runs_root = base.join("runs");
    let run_dir = runs_root.join("r1");
    std::fs::create_dir_all(run_dir.join("learning")).unwrap();
    std::fs::write(
        run_dir.join("run_summary.json"),
        r#"{"workflow_id":"wf","adl_version":"0.7","swarm_version":"0.6.0","status":"success"}"#,
    )
    .unwrap();
    std::fs::write(
        run_dir.join("steps.json"),
        r#"[{"step_id":"s1","provider_id":"p1","status":"success","output_artifact_path":"/Users/redacted/path.txt"}]"#,
    )
    .unwrap();
    std::fs::write(
        run_dir.join("learning").join("suggestions.json"),
        r#"{"suggestions":[{"id":"sug-002","category":"security"},{"id":"sug-001","category":"retry"}]}"#,
    )
    .unwrap();

    let out1 = base.join("bundle-a");
    let out2 = base.join("bundle-b");
    export_bundle_v1(&runs_root, &[], &out1).unwrap();
    export_bundle_v1(&runs_root, &[], &out2).unwrap();

    let manifest_a = std::fs::read(out1.join("learning_export_v1").join("manifest.json")).unwrap();
    let manifest_b = std::fs::read(out2.join("learning_export_v1").join("manifest.json")).unwrap();
    assert_eq!(
        manifest_a, manifest_b,
        "bundle manifest must be byte-stable"
    );

    let manifest_json: serde_json::Value = serde_json::from_slice(&manifest_a).unwrap();
    for entry in manifest_json
        .get("files")
        .and_then(|v| v.as_array())
        .unwrap()
    {
        let rel = entry.get("path").and_then(|v| v.as_str()).unwrap();
        let expected_hash = entry.get("hash").and_then(|v| v.as_str()).unwrap();
        let bytes = std::fs::read(out1.join("learning_export_v1").join(rel)).unwrap();
        assert_eq!(
            expected_hash,
            stable_fingerprint_hex(&bytes),
            "manifest hash must match file content for {rel}"
        );
    }

    let steps = std::fs::read_to_string(
        out1.join("learning_export_v1")
            .join("runs")
            .join("r1")
            .join("step_records.json"),
    )
    .unwrap();
    assert!(
        !steps.contains("/Users/") && !steps.contains("/home/"),
        "bundle must not leak host paths: {steps}"
    );
    assert!(
        !steps.contains("gho_"),
        "bundle must not leak token-like secrets: {steps}"
    );
}

#[test]
fn export_trace_bundle_v2_requires_activation_log() {
    let base = std::env::temp_dir().join(format!("trace-bundle-missing-{}", std::process::id()));
    let runs_root = base.join("runs");
    let run_dir = runs_root.join("r1");
    std::fs::create_dir_all(run_dir.join("logs")).unwrap();
    std::fs::create_dir_all(run_dir.join("learning")).unwrap();
    std::fs::write(
        run_dir.join("run.json"),
        r#"{"schema_version":"run_state.v1","run_id":"r1","workflow_id":"wf","version":"0.75","status":"success","error_message":null,"start_time_ms":1,"end_time_ms":2,"duration_ms":1,"execution_plan_hash":"abc","scheduler_max_concurrency":null,"scheduler_policy_source":null,"pause":null}"#,
    )
    .unwrap();
    std::fs::write(
        run_dir.join("steps.json"),
        r#"[{"step_id":"s1","agent_id":"a","provider_id":"p","status":"success","output_artifact_path":"outputs/s1.txt"}]"#,
    )
    .unwrap();
    std::fs::write(
        run_dir.join("run_summary.json"),
        r#"{"run_summary_version":1,"artifact_model_version":1,"run_id":"r1","workflow_id":"wf","adl_version":"0.75","swarm_version":"0.7.0","status":"success","counts":{"total_steps":1,"completed_steps":1,"failed_steps":0,"provider_call_count":1,"delegation_steps":0,"delegation_requires_verification_steps":0},"policy":{"security_envelope_enabled":false,"signing_required":false,"key_id_required":false,"verify_allowed_algs":[],"verify_allowed_key_sources":[],"sandbox_policy":"centralized_path_resolver_v1","security_denials_by_code":{}},"links":{"run_json":"run.json","steps_json":"steps.json","outputs_dir":"outputs","logs_dir":"logs","learning_dir":"learning","overlays_dir":"learning/overlays"}}"#,
    )
    .unwrap();
    std::fs::write(
        run_dir.join("run_status.json"),
        r#"{"run_status_version":1,"run_id":"r1","workflow_id":"wf","overall_status":"succeeded","completed_steps":["s1"],"pending_steps":[],"attempt_counts_by_step":{"s1":1}}"#,
    )
    .unwrap();

    let out = base.join("out");
    let err = export_trace_bundle_v2(&runs_root, &[], &out)
        .expect_err("missing activation log should fail");
    assert!(
        err.to_string().contains("missing required file"),
        "unexpected error: {err}"
    );
}

#[test]
fn export_trace_bundle_v2_is_deterministic_and_manifest_hashes_match() {
    let base = std::env::temp_dir().join(format!("trace-bundle-{}", std::process::id()));
    let runs_root = base.join("runs");
    let run_dir = runs_root.join("r1");
    std::fs::create_dir_all(run_dir.join("logs")).unwrap();
    std::fs::create_dir_all(run_dir.join("learning")).unwrap();
    std::fs::write(
        run_dir.join("run.json"),
        r#"{"schema_version":"run_state.v1","run_id":"r1","workflow_id":"wf","version":"0.75","status":"success","error_message":null,"start_time_ms":1,"end_time_ms":2,"duration_ms":1,"execution_plan_hash":"abc","scheduler_max_concurrency":null,"scheduler_policy_source":null,"pause":null}"#,
    )
    .unwrap();
    std::fs::write(
        run_dir.join("steps.json"),
        r#"[{"step_id":"s1","agent_id":"a","provider_id":"p","status":"success","output_artifact_path":"outputs/s1.txt"}]"#,
    )
    .unwrap();
    std::fs::write(
        run_dir.join("run_summary.json"),
        r#"{"run_summary_version":1,"artifact_model_version":1,"run_id":"r1","workflow_id":"wf","adl_version":"0.75","swarm_version":"0.7.0","status":"success","counts":{"total_steps":1,"completed_steps":1,"failed_steps":0,"provider_call_count":1,"delegation_steps":0,"delegation_requires_verification_steps":0},"policy":{"security_envelope_enabled":false,"signing_required":false,"key_id_required":false,"verify_allowed_algs":[],"verify_allowed_key_sources":[],"sandbox_policy":"centralized_path_resolver_v1","security_denials_by_code":{}},"links":{"run_json":"run.json","steps_json":"steps.json","outputs_dir":"outputs","logs_dir":"logs","learning_dir":"learning","overlays_dir":"learning/overlays"}}"#,
    )
    .unwrap();
    std::fs::write(
        run_dir.join("run_status.json"),
        r#"{"run_status_version":1,"run_id":"r1","workflow_id":"wf","overall_status":"succeeded","failure_kind":null,"completed_steps":["s1"],"pending_steps":[],"attempt_counts_by_step":{"s1":1}}"#,
    )
    .unwrap();
    std::fs::write(
        run_dir.join("logs").join("activation_log.json"),
        r#"{"activation_log_version":1,"ordering":"append_only_emission_order","stable_ids":{"step_id":"replay_stable_with_same_plan","delegation_id":"replay_stable_with_same_activation_log","run_id":"run_scoped_not_cross_run_stable"},"events":[{"RunStarted":{"ts":"2026-03-01T00:00:00.000Z","run_id":"r1","workflow_id":"wf","version":"0.75"}}]}"#,
    )
    .unwrap();
    std::fs::write(
        run_dir.join("learning").join("scores.json"),
        r#"{"scores_version":1,"run_id":"r1","generated_from":{"artifact_model_version":1,"run_summary_version":1},"summary":{"success_ratio":1.0,"failure_count":0,"retry_count":0,"delegation_denied_count":0,"security_denied_count":0},"metrics":{"scheduler_max_parallel_observed":1}}"#,
    )
    .unwrap();

    let out1 = base.join("bundle-a");
    let out2 = base.join("bundle-b");
    export_trace_bundle_v2(&runs_root, &[], &out1).unwrap();
    export_trace_bundle_v2(&runs_root, &[], &out2).unwrap();

    let manifest_a = std::fs::read(out1.join("trace_bundle_v2").join("manifest.json")).unwrap();
    let manifest_b = std::fs::read(out2.join("trace_bundle_v2").join("manifest.json")).unwrap();
    assert_eq!(
        manifest_a, manifest_b,
        "trace bundle manifest must be byte-stable"
    );

    let manifest_json: serde_json::Value = serde_json::from_slice(&manifest_a).unwrap();
    for entry in manifest_json
        .get("files")
        .and_then(|v| v.as_array())
        .unwrap()
    {
        let rel = entry.get("path").and_then(|v| v.as_str()).unwrap();
        let expected_hash = entry.get("hash").and_then(|v| v.as_str()).unwrap();
        let bytes = std::fs::read(out1.join("trace_bundle_v2").join(rel)).unwrap();
        assert_eq!(
            expected_hash,
            stable_fingerprint_hex(&bytes),
            "manifest hash must match file content for {rel}"
        );
    }
}

#[test]
fn import_trace_bundle_v2_accepts_valid_bundle_and_returns_activation_log_path() {
    let base = std::env::temp_dir().join(format!("trace-bundle-import-ok-{}", std::process::id()));
    let runs_root = base.join("runs");
    let run_dir = runs_root.join("r1");
    std::fs::create_dir_all(run_dir.join("logs")).unwrap();
    std::fs::create_dir_all(run_dir.join("learning")).unwrap();
    std::fs::write(
        run_dir.join("run.json"),
        r#"{"schema_version":"run_state.v1","run_id":"r1","workflow_id":"wf","version":"0.75","status":"success","error_message":null,"start_time_ms":1,"end_time_ms":2,"duration_ms":1,"execution_plan_hash":"abc","scheduler_max_concurrency":null,"scheduler_policy_source":null,"pause":null}"#,
    )
    .unwrap();
    std::fs::write(
        run_dir.join("steps.json"),
        r#"[{"step_id":"s1","agent_id":"a","provider_id":"p","status":"success","output_artifact_path":"outputs/s1.txt"}]"#,
    )
    .unwrap();
    std::fs::write(
        run_dir.join("run_summary.json"),
        r#"{"run_summary_version":1,"artifact_model_version":1,"run_id":"r1","workflow_id":"wf","adl_version":"0.75","swarm_version":"0.7.0","status":"success","counts":{"total_steps":1,"completed_steps":1,"failed_steps":0,"provider_call_count":1,"delegation_steps":0,"delegation_requires_verification_steps":0},"policy":{"security_envelope_enabled":false,"signing_required":false,"key_id_required":false,"verify_allowed_algs":[],"verify_allowed_key_sources":[],"sandbox_policy":"centralized_path_resolver_v1","security_denials_by_code":{}},"links":{"run_json":"run.json","steps_json":"steps.json","outputs_dir":"outputs","logs_dir":"logs","learning_dir":"learning","overlays_dir":"learning/overlays"}}"#,
    )
    .unwrap();
    std::fs::write(
        run_dir.join("run_status.json"),
        r#"{"run_status_version":1,"run_id":"r1","workflow_id":"wf","overall_status":"succeeded","failure_kind":null,"completed_steps":["s1"],"pending_steps":[],"attempt_counts_by_step":{"s1":1}}"#,
    )
    .unwrap();
    std::fs::write(
        run_dir.join("logs").join("activation_log.json"),
        r#"{"activation_log_version":1,"ordering":"append_only_emission_order","stable_ids":{"step_id":"replay_stable_with_same_plan","delegation_id":"replay_stable_with_same_activation_log","run_id":"run_scoped_not_cross_run_stable"},"events":[{"RunStarted":{"ts":"2026-03-01T00:00:00.000Z","run_id":"r1","workflow_id":"wf","version":"0.75"}}]}"#,
    )
    .unwrap();
    let out = base.join("bundle");
    export_trace_bundle_v2(&runs_root, &[], &out).unwrap();

    let imported = import_trace_bundle_v2(&out.join("trace_bundle_v2"), "r1").unwrap();
    assert_eq!(imported.run_id, "r1");
    assert!(imported.activation_log_path.is_file());
    assert!(imported
        .activation_log_path
        .ends_with("trace_bundle_v2/runs/r1/logs/activation_log.json"));
}

#[test]
fn import_trace_bundle_v2_rejects_manifest_hash_mismatch() {
    let base = std::env::temp_dir().join(format!(
        "trace-bundle-import-bad-hash-{}",
        std::process::id()
    ));
    let runs_root = base.join("runs");
    let run_dir = runs_root.join("r1");
    std::fs::create_dir_all(run_dir.join("logs")).unwrap();
    std::fs::write(
        run_dir.join("run.json"),
        r#"{"schema_version":"run_state.v1","run_id":"r1","workflow_id":"wf","version":"0.75","status":"success","error_message":null,"start_time_ms":1,"end_time_ms":2,"duration_ms":1,"execution_plan_hash":"abc","scheduler_max_concurrency":null,"scheduler_policy_source":null,"pause":null}"#,
    )
    .unwrap();
    std::fs::write(run_dir.join("steps.json"), r#"[]"#).unwrap();
    std::fs::write(
        run_dir.join("run_summary.json"),
        r#"{"run_summary_version":1,"artifact_model_version":1,"run_id":"r1","workflow_id":"wf","adl_version":"0.75","swarm_version":"0.7.0","status":"success","counts":{"total_steps":0,"completed_steps":0,"failed_steps":0,"provider_call_count":0,"delegation_steps":0,"delegation_requires_verification_steps":0},"policy":{"security_envelope_enabled":false,"signing_required":false,"key_id_required":false,"verify_allowed_algs":[],"verify_allowed_key_sources":[],"sandbox_policy":"centralized_path_resolver_v1","security_denials_by_code":{}},"links":{"run_json":"run.json","steps_json":"steps.json","outputs_dir":"outputs","logs_dir":"logs","learning_dir":"learning","overlays_dir":"learning/overlays"}}"#,
    )
    .unwrap();
    std::fs::write(
        run_dir.join("run_status.json"),
        r#"{"run_status_version":1,"run_id":"r1","workflow_id":"wf","overall_status":"succeeded","failure_kind":null,"completed_steps":[],"pending_steps":[],"attempt_counts_by_step":{}}"#,
    )
    .unwrap();
    std::fs::write(
        run_dir.join("logs").join("activation_log.json"),
        r#"{"activation_log_version":1,"ordering":"append_only_emission_order","stable_ids":{"step_id":"x","delegation_id":"x","run_id":"x"},"events":[]}"#,
    )
    .unwrap();
    let out = base.join("bundle");
    export_trace_bundle_v2(&runs_root, &[], &out).unwrap();
    let activation = out
        .join("trace_bundle_v2")
        .join("runs")
        .join("r1")
        .join("logs")
        .join("activation_log.json");
    std::fs::write(&activation, b"{\"tampered\":true}").unwrap();

    let err = import_trace_bundle_v2(&out.join("trace_bundle_v2"), "r1")
        .expect_err("tampered bundle should fail hash check");
    assert!(
        err.to_string().contains("hash mismatch") || err.to_string().contains("size mismatch"),
        "unexpected: {err}"
    );
}

#[test]
fn validate_bundle_rel_path_rejects_absolute_windows_and_traversal_paths() {
    let err = validate_bundle_rel_path("/abs/path.json").expect_err("absolute must fail");
    assert!(err.to_string().contains("absolute path"));

    let err = validate_bundle_rel_path("runs\\r1\\run.json").expect_err("windows separator fails");
    assert!(err.to_string().contains("non-canonical path separator"));

    let err = validate_bundle_rel_path("runs/../secret.json").expect_err("traversal fails");
    assert!(err.to_string().contains("traversal or prefix"));

    validate_bundle_rel_path("runs/r1/run.json").expect("relative path is valid");
}

#[test]
fn resolve_export_ids_sorts_and_dedupes_explicit_ids() {
    let root = std::env::temp_dir().join(format!("learn-ids-{}", std::process::id()));
    std::fs::create_dir_all(&root).unwrap();
    let ids = resolve_export_ids(
        &root,
        &["r2".to_string(), "r1".to_string(), "r2".to_string()],
    )
    .expect("resolve ids");
    assert_eq!(ids, vec!["r1".to_string(), "r2".to_string()]);
}

#[test]
fn export_jsonl_rejects_unsafe_explicit_run_id_path_segment() {
    let root = std::env::temp_dir().join(format!("learn-unsafe-{}", std::process::id()));
    let out = root.join("out.jsonl");
    std::fs::create_dir_all(&root).unwrap();
    let err = export_jsonl(&root, &["../escape".to_string()], &out)
        .expect_err("unsafe explicit run_id must fail");
    assert!(err.to_string().contains("safe path segment"));
}

#[test]
fn discover_run_ids_ignores_non_dirs_and_missing_summary() {
    let root = std::env::temp_dir().join(format!("learn-discover-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(root.join("not-a-dir"), "x").unwrap();
    std::fs::create_dir_all(root.join("r-no-summary")).unwrap();
    std::fs::create_dir_all(root.join("r-ok")).unwrap();
    std::fs::write(root.join("r-ok").join("run_summary.json"), "{}").unwrap();

    let ids = discover_run_ids(&root).expect("discover run ids");
    assert_eq!(ids, vec!["r-ok".to_string()]);
}

#[test]
fn import_trace_bundle_v2_rejects_missing_or_unsorted_manifest_surfaces() {
    let root = std::env::temp_dir().join(format!("trace-bundle-errors-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(root.join("trace_bundle_v2")).unwrap();

    let err = import_trace_bundle_v2(&root, "r1").expect_err("manifest missing must fail");
    assert!(err.to_string().contains("manifest not found"));

    let manifest_path = root.join("trace_bundle_v2").join("manifest.json");
    std::fs::write(
        &manifest_path,
        serde_json::to_vec_pretty(&serde_json::json!({
            "trace_bundle_version": 999,
            "run_count": 1,
            "runs": ["r1"],
            "files": []
        }))
        .unwrap(),
    )
    .unwrap();
    let err = import_trace_bundle_v2(&root, "r1").expect_err("bad version must fail");
    assert!(err.to_string().contains("unsupported trace_bundle_version"));

    std::fs::write(
        &manifest_path,
        serde_json::to_vec_pretty(&serde_json::json!({
            "trace_bundle_version": TRACE_BUNDLE_VERSION,
            "run_count": 2,
            "runs": ["r2", "r1"],
            "files": []
        }))
        .unwrap(),
    )
    .unwrap();
    let err = import_trace_bundle_v2(&root, "r1").expect_err("unsorted runs must fail");
    assert!(err
        .to_string()
        .contains("runs list is not canonically sorted"));
}
