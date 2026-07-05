use super::*;

#[test]
fn agent_run_writes_bounded_cycles_and_status() {
    let root = unique_test_temp_dir("agent-smoke");
    let spec = root.join("agent.yaml");
    fs::write(
        &spec,
        r#"schema: adl.long_lived_agent_spec.v1
agent_instance_id: smoke-agent
display_name: Smoke Agent
state_root: state
workflow:
  kind: demo_adapter
  name: wp02_smoke_probe
  run_args:
    provider_id: local_ollama
    model: gemma4:latest
heartbeat:
  interval_secs: 1
  max_cycles: 3
  stale_lease_after_secs: 60
safety:
  allow_network: false
  allow_broker: false
  allow_filesystem_writes_outside_state_root: false
  allow_real_world_side_effects: false
  require_public_artifact_sanitization: true
  financial_advice: false
  max_cycle_runtime_secs: 120
  max_consecutive_failures: 2
memory:
  namespace: smoke/agent
  write_policy: append_only
"#,
    )
    .expect("write agent spec");

    let spec_str = spec.to_str().expect("utf8 path");
    let out = run_adl(&[
        "agent",
        "run",
        "--spec",
        spec_str,
        "--max-cycles",
        "3",
        "--no-sleep",
        "--json",
    ]);
    assert!(
        out.status.success(),
        "expected agent run success, stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("\"state\": \"completed\""),
        "stdout:\n{stdout}"
    );
    assert!(
        stdout.contains("\"completed_cycle_count\": 3"),
        "stdout:\n{stdout}"
    );
    assert!(root.join("state/status.json").exists());
    assert!(root.join("state/agent_spec.locked.json").exists());
    assert!(root.join("state/continuity.json").exists());
    assert!(root.join("state/continuity_checkpoint.json").exists());
    assert!(root.join("state/continuity_replay_manifest.json").exists());
    assert!(root.join("state/cycle_ledger.jsonl").exists());
    assert!(root.join("state/provider_binding_history.jsonl").exists());
    assert!(root.join("state/memory_index.json").exists());
    for cycle_id in ["cycle-000001", "cycle-000002", "cycle-000003"] {
        let cycle_dir = root.join("state/cycles").join(cycle_id);
        for artifact in [
            "cycle_manifest.json",
            "observations.json",
            "decision_request.json",
            "decision_result.json",
            "run_ref.json",
            "memory_writes.jsonl",
            "guardrail_report.json",
            "cycle_summary.md",
        ] {
            assert!(
                cycle_dir.join(artifact).exists(),
                "missing {artifact} for {cycle_id}"
            );
        }
    }
    let ledger =
        fs::read_to_string(root.join("state/cycle_ledger.jsonl")).expect("read cycle ledger");
    assert_eq!(ledger.lines().count(), 3);
    let continuity =
        fs::read_to_string(root.join("state/continuity.json")).expect("read continuity");
    assert!(continuity.contains(r#""continuity_kind": "pre_v0_92_handle""#));
    assert!(continuity.contains(r#""latest_cycle_id": "cycle-000003""#));

    let human_status = run_adl(&["agent", "status", "--spec", spec_str]);
    assert!(
        human_status.status.success(),
        "expected agent status success, stderr:\n{}",
        String::from_utf8_lossy(&human_status.stderr)
    );
    let human_stdout = String::from_utf8_lossy(&human_status.stdout);
    assert!(human_stdout.contains("agent: smoke-agent"));
    assert!(human_stdout.contains("state: completed"));

    let status = run_adl(&["agent", "status", "--spec", spec_str, "--json"]);
    assert!(
        status.status.success(),
        "expected agent status success, stderr:\n{}",
        String::from_utf8_lossy(&status.stderr)
    );
    let status_stdout = String::from_utf8_lossy(&status.stdout);
    assert!(
        status_stdout.contains("\"state\": \"completed\""),
        "stdout:\n{status_stdout}"
    );

    let inspect = run_adl(&["agent", "inspect", "--spec", spec_str, "--json"]);
    assert!(
        inspect.status.success(),
        "expected agent inspect success, stderr:\n{}",
        String::from_utf8_lossy(&inspect.stderr)
    );
    let inspect_stdout = String::from_utf8_lossy(&inspect.stdout);
    assert!(
        inspect_stdout.contains("\"schema\": \"adl.long_lived_agent_inspection_packet.v1\""),
        "stdout:\n{inspect_stdout}"
    );
    assert!(
        inspect_stdout.contains("\"manifest\": \"cycles/cycle-000003/cycle_manifest.json\""),
        "stdout:\n{inspect_stdout}"
    );
    assert!(
        inspect_stdout
            .contains("\"guardrail_report\": \"cycles/cycle-000003/guardrail_report.json\""),
        "stdout:\n{inspect_stdout}"
    );
    assert!(
        inspect_stdout.contains("\"path\": \"continuity_checkpoint.json\""),
        "stdout:\n{inspect_stdout}"
    );

    let human_inspect = run_adl(&["agent", "inspect", "--spec", spec_str]);
    assert!(
        human_inspect.status.success(),
        "expected human agent inspect success, stderr:\n{}",
        String::from_utf8_lossy(&human_inspect.stderr)
    );
    let human_inspect_stdout = String::from_utf8_lossy(&human_inspect.stdout);
    assert!(human_inspect_stdout.contains("agent: smoke-agent"));
    assert!(human_inspect_stdout.contains("cycle: cycle-000003 success"));
    assert!(human_inspect_stdout.contains("proof: pass"));
}

#[test]
fn agent_restart_restores_checkpoint_and_reuses_next_cycle_id_without_duplicates() {
    let root = unique_test_temp_dir("agent-restart");
    let spec = root.join("agent.yaml");
    fs::write(
        &spec,
        r#"schema: adl.long_lived_agent_spec.v1
agent_instance_id: restart-agent
display_name: Restart Agent
state_root: state
workflow:
  kind: demo_adapter
  name: wp02_smoke_probe
  run_args:
    provider_id: local_ollama
    model: gemma4:latest
heartbeat:
  interval_secs: 1
  max_cycles: 2
  stale_lease_after_secs: 60
safety:
  allow_network: false
  allow_broker: false
  allow_filesystem_writes_outside_state_root: false
  allow_real_world_side_effects: false
  require_public_artifact_sanitization: true
  financial_advice: false
  max_cycle_runtime_secs: 120
  max_consecutive_failures: 2
memory:
  namespace: smoke/agent
  write_policy: append_only
"#,
    )
    .expect("write agent spec");

    let spec_str = spec.to_str().expect("utf8 path");
    let first = run_adl(&[
        "agent",
        "run",
        "--spec",
        spec_str,
        "--max-cycles",
        "2",
        "--no-sleep",
        "--json",
    ]);
    assert!(
        first.status.success(),
        "expected first run success, stderr:\n{}",
        String::from_utf8_lossy(&first.stderr)
    );

    fs::remove_file(root.join("state/status.json")).expect("remove status to force restore");

    let restored = run_adl(&["agent", "status", "--spec", spec_str, "--json"]);
    assert!(
        restored.status.success(),
        "expected restored status success, stderr:\n{}",
        String::from_utf8_lossy(&restored.stderr)
    );
    let restored_stdout = String::from_utf8_lossy(&restored.stdout);
    assert!(
        restored_stdout.contains("\"last_cycle_id\": \"cycle-000002\""),
        "stdout:\n{restored_stdout}"
    );

    let replay_manifest: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(root.join("state/continuity_replay_manifest.json"))
            .expect("read continuity replay manifest"),
    )
    .expect("parse continuity replay manifest");
    assert_eq!(
        replay_manifest["expected_resume"]["next_cycle_id"],
        "cycle-000003"
    );

    let second = run_adl(&[
        "agent",
        "run",
        "--spec",
        spec_str,
        "--max-cycles",
        "1",
        "--no-sleep",
        "--json",
    ]);
    assert!(
        second.status.success(),
        "expected resumed run success, stderr:\n{}",
        String::from_utf8_lossy(&second.stderr)
    );

    let ledger =
        fs::read_to_string(root.join("state/cycle_ledger.jsonl")).expect("read cycle ledger");
    assert_eq!(ledger.lines().count(), 3, "ledger:\n{ledger}");
    assert!(ledger.contains("\"cycle_id\":\"cycle-000001\""));
    assert!(ledger.contains("\"cycle_id\":\"cycle-000002\""));
    assert!(ledger.contains("\"cycle_id\":\"cycle-000003\""));

    let checkpoint: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(root.join("state/continuity_checkpoint.json"))
            .expect("read continuity checkpoint"),
    )
    .expect("parse continuity checkpoint");
    assert_eq!(checkpoint["latest_cycle_id"], "cycle-000003");
}

#[test]
fn csm_daemon_writes_status_checkpoints_and_otel_observability() {
    let root = unique_test_temp_dir("csm-daemon");
    let spec = root.join("agent.yaml");
    fs::write(
        &spec,
        r#"schema: adl.long_lived_agent_spec.v1
agent_instance_id: daemon-agent
display_name: Daemon Agent
state_root: state
workflow:
  kind: demo_adapter
  name: wp02_smoke_probe
  run_args:
    provider_id: local_ollama
    model: gemma4:latest
heartbeat:
  interval_secs: 1
  max_cycles: 3
  stale_lease_after_secs: 60
safety:
  allow_network: false
  allow_broker: false
  allow_filesystem_writes_outside_state_root: false
  allow_real_world_side_effects: false
  require_public_artifact_sanitization: true
  financial_advice: false
  max_cycle_runtime_secs: 120
  max_consecutive_failures: 2
memory:
  namespace: smoke/daemon-agent
  write_policy: append_only
"#,
    )
    .expect("write agent spec");

    let observability_log = root.join("observability.log");
    let otel_log = root.join("otel.jsonl");
    let otel_status = root.join("otel-status.json");
    let spec_str = spec.to_str().expect("utf8 path");
    let log_str = observability_log.to_str().expect("utf8 log path");
    let otel_log_str = otel_log.to_str().expect("utf8 otel log path");
    let otel_status_str = otel_status.to_str().expect("utf8 otel status path");
    let out = run_csm_with_env(
        &[
            "daemon",
            "--spec",
            spec_str,
            "--max-restarts",
            "1",
            "--checkpoint-interval-secs",
            "1",
            "--no-sleep",
            "--json",
        ],
        &[
            ("ADL_OBSERVABILITY_STDERR", "0"),
            ("ADL_OBSERVABILITY_LOG", log_str),
            ("ADL_OBSERVABILITY_HEARTBEAT_MS", "25"),
            ("ADL_OTEL_LOG", otel_log_str),
            ("ADL_OTEL_STATUS", otel_status_str),
        ],
    );
    assert!(
        out.status.success(),
        "expected daemon success, stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("\"schema\": \"adl.long_lived_agent_daemon_status.v1\""),
        "stdout:\n{stdout}"
    );
    assert!(
        stdout.contains("\"state\": \"completed\""),
        "stdout:\n{stdout}"
    );
    assert!(root.join("state/daemon_status.json").exists());
    assert!(root.join("state/continuity_checkpoint.json").exists());
    assert!(root.join("state/continuity_replay_manifest.json").exists());

    let daemon_status: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(root.join("state/daemon_status.json")).expect("read daemon status"),
    )
    .expect("parse daemon status");
    assert_eq!(
        daemon_status["unsupported_permanence_claims"][0],
        "not_os_boot_persistent"
    );
    assert_eq!(daemon_status["trace_id"], "agent.daemon-agent.daemon");

    let operator_events =
        fs::read_to_string(root.join("state/operator_events.jsonl")).expect("operator events");
    assert!(operator_events.contains("\"event\":\"daemon_started\""));
    assert!(operator_events.contains("\"event\":\"child_spawn\""));
    assert!(operator_events.contains("\"event\":\"checkpoint_write\""));
    assert!(operator_events.contains("\"trace_id\":\"agent.daemon-agent.daemon\""));
    assert!(operator_events.contains("\"otel\""));

    let observability = fs::read_to_string(&observability_log).expect("read observability log");
    assert!(observability.contains("command=csm"));
    assert!(observability.contains("stage=csm_daemon"));
    assert!(observability.contains("stage=daemon_started"));
    assert!(observability.contains("stage=checkpoint_write"));
    assert!(observability.contains("otel_service_name=csm-runtime-daemon"));
    assert!(observability.contains("trace_id=agent.daemon-agent.daemon"));

    let otel_events = fs::read_to_string(&otel_log).expect("read otel jsonl");
    assert!(otel_events.contains("\"schema\":\"adl.otel.event.v1\""));
    assert!(otel_events.contains("\"name\":\"csm.daemon_started\""));
    assert!(otel_events.contains("\"name\":\"csm.csm_daemon\""));
    assert!(otel_events.contains("\"trace_id\":\"agent.daemon-agent.daemon\""));
    assert!(otel_events.contains("\"service.name\":\"csm-runtime-daemon\""));

    let otel_status: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&otel_status).expect("read otel status"))
            .expect("parse otel status");
    assert_eq!(otel_status["schema"], "adl.otel.monitor_status.v1");
    assert!(otel_status["event_count"].as_u64().expect("event count") >= 4);
    assert_eq!(otel_status["last_trace_id"], "agent.daemon-agent.daemon");
}

#[test]
fn csm_daemon_executes_adl_workflow_dag_with_aee_runtime_trace() {
    let root = unique_test_temp_dir("csm-adl-workflow");
    let spec = root.join("agent.yaml");
    let workflow = fixture_path("examples/v0-3-scheduler-max-concurrency.adl.yaml");
    fs::write(
        &spec,
        format!(
            r#"schema: adl.long_lived_agent_spec.v1
agent_instance_id: csm-dag-agent
display_name: CSM DAG Agent
state_root: state
workflow:
  kind: adl_workflow
  name: scheduler_max_concurrency
  path: {}
  run_args: {{}}
heartbeat:
  interval_secs: 1
  max_cycles: 3
  stale_lease_after_secs: 60
safety:
  allow_network: false
  allow_broker: false
  allow_filesystem_writes_outside_state_root: false
  allow_real_world_side_effects: false
  require_public_artifact_sanitization: true
  financial_advice: false
  max_cycle_runtime_secs: 120
  max_consecutive_failures: 2
memory:
  namespace: smoke/csm-dag-agent
  write_policy: append_only
"#,
            workflow.display()
        ),
    )
    .expect("write agent spec");

    let observability_log = root.join("observability.log");
    let otel_log = root.join("otel.jsonl");
    let otel_status = root.join("otel-status.json");
    let mock = fixture_path("tools/mock_ollama_v0_4.sh");
    let spec_str = spec.to_str().expect("utf8 path");
    let out = run_csm_with_env(
        &[
            "daemon",
            "--spec",
            spec_str,
            "--max-restarts",
            "1",
            "--checkpoint-interval-secs",
            "1",
            "--no-sleep",
            "--json",
        ],
        &[
            ("ADL_OBSERVABILITY_STDERR", "0"),
            (
                "ADL_OBSERVABILITY_LOG",
                observability_log.to_str().expect("utf8 observability path"),
            ),
            ("ADL_OTEL_LOG", otel_log.to_str().expect("utf8 otel path")),
            (
                "ADL_OTEL_STATUS",
                otel_status.to_str().expect("utf8 otel status path"),
            ),
            ("ADL_OLLAMA_BIN", mock.to_str().expect("utf8 mock path")),
        ],
    );
    assert!(
        out.status.success(),
        "expected csm DAG runtime success, stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );

    let cycle_root = root.join("state/cycles/cycle-000001");
    let run_status_path = cycle_root.join("csm_adl_run_status.json");
    assert!(run_status_path.exists());
    assert!(cycle_root.join("adl_runtime").exists());
    let run_status: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&run_status_path).expect("read run status"))
            .expect("parse run status");
    assert_eq!(run_status["schema"], "adl.csm.adl_workflow_run_status.v1");
    assert_eq!(run_status["runtime_owner"], "csm");
    assert_eq!(run_status["adl_role"], "tooling_control_plane");
    assert_eq!(run_status["status"], "success");
    assert_eq!(run_status["step_count"], 4);
    assert_eq!(run_status["scheduler_policy"]["max_concurrency"], 2);
    assert_eq!(run_status["scheduler_policy"]["source"], "run_default");
    assert_eq!(run_status["records"][0]["step_id"], "fork.a");
    assert_eq!(run_status["records"][0]["status"], "success");
    assert!(run_status["trace_events"]
        .as_array()
        .expect("trace events array")
        .iter()
        .any(|event| event
            .as_str()
            .expect("trace event")
            .contains("SchedulerPolicy max_concurrency=2 source=run_default")));
    assert!(
        run_status["trace_events"]
            .as_array()
            .expect("trace events array")
            .iter()
            .any(|event| event
                .as_str()
                .expect("trace event")
                .contains("RuntimeResilienceDecision")),
        "expected retained AEE/runtime resilience trace: {run_status}"
    );
    assert_eq!(
        run_status["aee_resilience_trace"],
        "retained_in_trace_events"
    );

    let run_ref: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(cycle_root.join("run_ref.json")).unwrap())
            .expect("parse run ref");
    assert_eq!(run_ref["run_status_ref"], "csm_adl_run_status.json");
    assert!(run_ref["execution_note"]
        .as_str()
        .expect("execution note")
        .contains("CSM executed the configured ADL DAG"));

    let daemon_status: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(root.join("state/daemon_status.json")).expect("read daemon status"),
    )
    .expect("parse daemon status");
    assert_eq!(
        daemon_status["runtime_capabilities"]["chronosense"]["status"],
        "integrated"
    );
    assert_eq!(
        daemon_status["runtime_capabilities"]["aee"]["status"],
        "integrated"
    );
    assert_eq!(
        daemon_status["runtime_capabilities"]["scheduler_watcher"]["status"],
        "integrated"
    );
    assert_eq!(
        daemon_status["runtime_capabilities"]["resilience_middleware"]["status"],
        "integrated"
    );
}

#[test]
fn csm_owns_daemon_and_adl_agent_daemon_is_removed() {
    let help = run_csm(&["--help"]);
    assert!(
        help.status.success(),
        "expected csm help success, stderr:\n{}",
        String::from_utf8_lossy(&help.stderr)
    );
    let help_stdout = String::from_utf8_lossy(&help.stdout);
    assert!(help_stdout.contains("csm daemon --spec"));
    assert!(help_stdout.contains("dedicated runtime owner binary"));

    let removed_agent = run_adl(&["agent", "daemon", "--help"]);
    assert!(
        !removed_agent.status.success(),
        "expected adl agent daemon removal, stdout:\n{}",
        String::from_utf8_lossy(&removed_agent.stdout)
    );
    let stderr = String::from_utf8_lossy(&removed_agent.stderr);
    assert!(
        stderr.contains("unknown agent subcommand 'daemon'"),
        "stderr:\n{stderr}"
    );

    let removed_adl_csm = run_adl(&["csm", "daemon", "--help"]);
    assert!(
        !removed_adl_csm.status.success(),
        "expected adl csm daemon removal, stdout:\n{}",
        String::from_utf8_lossy(&removed_adl_csm.stdout)
    );
    let stderr = String::from_utf8_lossy(&removed_adl_csm.stderr);
    assert!(
        stderr.contains("csm daemon is owned by the standalone csm runtime binary"),
        "stderr:\n{stderr}"
    );
}

#[test]
fn csm_daemon_restart_budget_failure_leaves_recoverable_checkpoint() {
    let root = unique_test_temp_dir("csm-daemon-failure");
    let spec = root.join("agent.yaml");
    fs::write(
        &spec,
        r#"schema: adl.long_lived_agent_spec.v1
agent_instance_id: daemon-failure-agent
display_name: Daemon Failure Agent
state_root: state
workflow:
  kind: unsupported_adapter
  name: failing_probe
  run_args: {}
heartbeat:
  interval_secs: 1
  max_cycles: 3
  stale_lease_after_secs: 60
safety:
  allow_network: false
  allow_broker: false
  allow_filesystem_writes_outside_state_root: false
  allow_real_world_side_effects: false
  require_public_artifact_sanitization: true
  financial_advice: false
  max_cycle_runtime_secs: 120
  max_consecutive_failures: 5
memory:
  namespace: smoke/daemon-failure-agent
  write_policy: append_only
"#,
    )
    .expect("write agent spec");

    let spec_str = spec.to_str().expect("utf8 path");
    let out = run_csm(&[
        "daemon",
        "--spec",
        spec_str,
        "--max-restarts",
        "1",
        "--checkpoint-interval-secs",
        "1",
        "--no-sleep",
        "--json",
    ]);
    assert!(
        !out.status.success(),
        "expected daemon failure, stdout:\n{}",
        String::from_utf8_lossy(&out.stdout)
    );

    let daemon_status: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(root.join("state/daemon_status.json")).expect("read daemon status"),
    )
    .expect("parse daemon status");
    assert_eq!(daemon_status["state"], "failed");
    assert_eq!(daemon_status["restart_count"], 1);
    assert_eq!(daemon_status["last_event"], "restart_budget_exhausted");
    assert!(root.join("state/continuity_checkpoint.json").exists());
    assert!(root.join("state/continuity_replay_manifest.json").exists());

    let status: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(root.join("state/status.json")).expect("read status"),
    )
    .expect("parse status");
    assert_eq!(status["state"], "failed");
    assert_eq!(status["last_error"]["class"], "daemon_child_failed");

    let operator_events =
        fs::read_to_string(root.join("state/operator_events.jsonl")).expect("operator events");
    assert!(operator_events.contains("\"event\":\"restart_scheduled\""));
    assert!(operator_events.contains("\"event\":\"restart_attempted\""));
    assert!(operator_events.contains("\"event\":\"restart_budget_exhausted\""));
    assert!(operator_events.contains("\"checkpoint_ref\":\"continuity_checkpoint.json\""));
}
