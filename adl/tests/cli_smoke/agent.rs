use super::*;
use std::io::Write;

fn spawn_loopback_otlp_collector() -> (
    String,
    std::sync::mpsc::Receiver<String>,
    std::thread::JoinHandle<()>,
) {
    let listener =
        std::net::TcpListener::bind("127.0.0.1:0").expect("bind loopback otlp collector");
    listener
        .set_nonblocking(true)
        .expect("set collector nonblocking");
    let addr = listener.local_addr().expect("collector addr");
    let (tx, rx) = std::sync::mpsc::channel();
    let handle = std::thread::spawn(move || {
        let started = std::time::Instant::now();
        let mut last_request_at: Option<std::time::Instant> = None;
        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let body = read_http_body(&mut stream);
                    tx.send(body).expect("send otlp body");
                    stream
                        .write_all(b"HTTP/1.1 200 OK\r\ncontent-length: 2\r\n\r\nOK")
                        .expect("write collector response");
                    last_request_at = Some(std::time::Instant::now());
                }
                Err(err) if err.kind() == std::io::ErrorKind::WouldBlock => {
                    let idle_done = last_request_at
                        .map(|instant| instant.elapsed() > std::time::Duration::from_millis(750))
                        .unwrap_or(false);
                    if idle_done || started.elapsed() > std::time::Duration::from_secs(8) {
                        break;
                    }
                    std::thread::sleep(std::time::Duration::from_millis(25));
                }
                Err(err) => panic!("collector accept failed: {err}"),
            }
        }
    });
    (format!("http://{addr}/v1/traces"), rx, handle)
}

fn read_http_body(stream: &mut std::net::TcpStream) -> String {
    use std::io::Read;
    let mut buf = Vec::new();
    let mut temp = [0_u8; 4096];
    loop {
        let n = stream.read(&mut temp).expect("read request");
        if n == 0 {
            break;
        }
        buf.extend_from_slice(&temp[..n]);
        if let Some(header_end) = find_header_end(&buf) {
            let content_length = content_length(&buf[..header_end]).unwrap_or(0);
            if buf.len() >= header_end + 4 + content_length {
                break;
            }
        }
    }
    if let Some(header_end) = find_header_end(&buf) {
        String::from_utf8_lossy(&buf[header_end + 4..]).to_string()
    } else {
        String::new()
    }
}

fn find_header_end(buf: &[u8]) -> Option<usize> {
    buf.windows(4).position(|window| window == b"\r\n\r\n")
}

fn content_length(headers: &[u8]) -> Option<usize> {
    String::from_utf8_lossy(headers).lines().find_map(|line| {
        let (name, value) = line.split_once(':')?;
        name.eq_ignore_ascii_case("content-length")
            .then(|| value.trim().parse::<usize>().ok())
            .flatten()
    })
}

fn otlp_attr_string<'a>(attrs: &'a serde_json::Value, key: &str) -> Option<&'a str> {
    attrs.as_array()?.iter().find_map(|attr| {
        (attr.get("key")?.as_str()? == key)
            .then(|| attr.get("value")?.get("stringValue")?.as_str())
            .flatten()
    })
}

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
fn csm_daemon_exports_otlp_http_json_to_loopback_collector() {
    let root = unique_test_temp_dir("csm-daemon-otlp");
    let spec = root.join("agent.yaml");
    fs::write(
        &spec,
        r#"schema: adl.long_lived_agent_spec.v1
agent_instance_id: daemon-otlp-agent
display_name: Daemon OTLP Agent
state_root: state
workflow:
  kind: demo_adapter
  name: otlp_probe
  run_args: {}
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
  namespace: smoke/daemon-otlp-agent
  write_policy: append_only
"#,
    )
    .expect("write agent spec");
    let (endpoint, captured, collector) = spawn_loopback_otlp_collector();
    let observability_log = root.join("observability.log");
    let otel_log = root.join("otel.jsonl");
    let otel_status = root.join("otel-status.json");
    let out = run_csm_with_env(
        &[
            "daemon",
            "--spec",
            spec.to_str().expect("utf8 spec"),
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
            ("ADL_OTEL_EXPORTER_OTLP_ENDPOINT", endpoint.as_str()),
            ("ADL_OTEL_EXPORTER_TIMEOUT_MS", "2000"),
        ],
    );
    assert!(
        out.status.success(),
        "expected daemon OTLP success, stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    collector.join().expect("collector joined");
    let exported = captured.try_iter().collect::<Vec<_>>();
    let mut span_names = std::collections::BTreeSet::new();
    let mut service_names = std::collections::BTreeSet::new();
    let mut trace_id_lengths = std::collections::BTreeSet::new();
    for body in &exported {
        let payload: serde_json::Value = serde_json::from_str(body).expect("parse otlp payload");
        for resource_span in payload["resourceSpans"].as_array().expect("resource spans") {
            if let Some(service_name) =
                otlp_attr_string(&resource_span["resource"]["attributes"], "service.name")
            {
                service_names.insert(service_name.to_string());
            }
            for scope_span in resource_span["scopeSpans"].as_array().expect("scope spans") {
                for span in scope_span["spans"].as_array().expect("spans") {
                    span_names.insert(span["name"].as_str().expect("span name").to_string());
                    trace_id_lengths.insert(span["traceId"].as_str().expect("trace id").len());
                    assert_eq!(span["kind"], 1);
                    assert!(span["startTimeUnixNano"].as_str().is_some());
                    assert!(span["endTimeUnixNano"].as_str().is_some());
                    assert_eq!(span["spanId"].as_str().expect("span id").len(), 16);
                }
            }
        }
    }
    assert!(service_names.contains("csm-runtime-daemon"));
    assert!(span_names.contains("csm.daemon_started"));
    assert!(span_names.contains("csm.checkpoint_write"));
    assert!(trace_id_lengths.contains(&32));
    let exported_text = exported.join("\n");
    assert!(!exported_text.contains("adl.otlp_http_json.export.v1"));
    assert!(!exported_text.contains(root.to_str().expect("root utf8")));

    let status: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&otel_status).expect("read otel status"))
            .expect("parse otel status");
    assert_eq!(status["schema"], "adl.otel.monitor_status.v1");
    assert_eq!(status["exporter"]["schema"], "adl.otel.exporter_status.v1");
    assert_eq!(status["exporter"]["protocol"], "otlp_http_json");
    assert_eq!(status["exporter"]["status"], "success");
    assert_eq!(status["exporter"]["endpoint"], "<configured>");
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

    let removed_adl_csm_service = run_adl(&["csm", "service", "install", "--help"]);
    assert!(
        !removed_adl_csm_service.status.success(),
        "expected adl csm service removal, stdout:\n{}",
        String::from_utf8_lossy(&removed_adl_csm_service.stdout)
    );
    let stderr = String::from_utf8_lossy(&removed_adl_csm_service.stderr);
    assert!(
        stderr.contains("csm service is owned by the standalone csm runtime binary"),
        "stderr:\n{stderr}"
    );
}

#[test]
fn csm_service_install_writes_launchd_envelope_without_adl_runtime_owner() {
    let root = unique_test_temp_dir("csm-service-install");
    let spec = root.join("agent.yaml");
    fs::write(
        &spec,
        r#"schema: adl.long_lived_agent_spec.v1
agent_instance_id: service-install-agent
display_name: Service Install Agent
state_root: runtime-state
workflow:
  kind: demo_adapter
  name: service_install_probe
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
memory:
  namespace: smoke/service-install-agent
  write_policy: append_only
"#,
    )
    .expect("write agent spec");
    let service_root = root.join("service");
    let csm_bin = resolve_csm_exe();
    let out = run_csm(&[
        "service",
        "install",
        "--spec",
        spec.to_str().expect("utf8 spec"),
        "--service-root",
        service_root.to_str().expect("utf8 service root"),
        "--manager",
        "launchd",
        "--label",
        "com.agentlogic.csm.test-install",
        "--csm-bin",
        csm_bin.to_str().expect("utf8 csm bin"),
        "--checkpoint-interval-secs",
        "1",
        "--otlp-endpoint",
        "http://127.0.0.1:4318/v1/traces",
        "--otlp-timeout-ms",
        "750",
        "--json",
    ]);
    assert!(
        out.status.success(),
        "expected service install success, stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );

    let manifest: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(service_root.join("service_manifest.json")).expect("manifest"),
    )
    .expect("parse manifest");
    assert_eq!(manifest["schema"], "adl.csm.service_manifest.v1");
    assert_eq!(manifest["runtime_owner"], "csm");
    assert_eq!(manifest["manager"], "launchd");
    assert_eq!(manifest["checkpoint_interval_secs"], 1);
    assert_eq!(manifest["otlp_endpoint"], "http://127.0.0.1:4318/v1/traces");
    assert_eq!(manifest["otlp_timeout_ms"], 750);
    assert!(manifest["daemon_status"]
        .as_str()
        .expect("daemon status path")
        .ends_with("runtime-state/daemon_status.json"));
    assert!(manifest["continuity_checkpoint"]
        .as_str()
        .expect("checkpoint path")
        .ends_with("runtime-state/continuity_checkpoint.json"));
    assert!(manifest["unsupported_permanence_claims"]
        .as_array()
        .expect("nonclaims")
        .iter()
        .any(|value| value == "host_reboot_survival_not_proven"));

    let plist = fs::read_to_string(service_root.join("csm.launchd.plist")).expect("plist");
    assert!(plist.contains("<key>KeepAlive</key>"));
    assert!(plist.contains("<string>daemon</string>"));
    assert!(plist.contains("ADL_OTEL_STATUS"));
    assert!(plist.contains("ADL_OTEL_EXPORTER_OTLP_ENDPOINT"));
    assert!(plist.contains("http://127.0.0.1:4318/v1/traces"));
    assert!(plist.contains("ADL_OTEL_EXPORTER_TIMEOUT_MS"));
    assert!(!plist.contains("adl agent daemon"));

    let status: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(service_root.join("service_status.json")).expect("service status"),
    )
    .expect("parse service status");
    assert_eq!(status["schema"], "adl.csm.service_status.v1");
    assert_eq!(status["service_state"], "installed");
    assert_eq!(status["broad_process_scan"], false);
    assert_eq!(status["uses_ps"], false);
    assert_eq!(status["otlp_exporter_configured"], true);
    assert_eq!(status["otlp_endpoint_ref"], "<configured>");
}

#[test]
fn csm_service_install_rejects_secret_bearing_otlp_endpoint() {
    let root = unique_test_temp_dir("csm-service-secret-otlp");
    let spec = root.join("agent.yaml");
    fs::write(
        &spec,
        r#"schema: adl.long_lived_agent_spec.v1
agent_instance_id: service-secret-otlp-agent
display_name: Service Secret OTLP Agent
state_root: state
workflow:
  kind: demo_adapter
  name: service_secret_otlp_probe
  run_args: {}
heartbeat:
  interval_secs: 1
  max_cycles: 1
  stale_lease_after_secs: 60
safety:
  allow_network: false
  allow_broker: false
  allow_filesystem_writes_outside_state_root: false
  allow_real_world_side_effects: false
  require_public_artifact_sanitization: true
  financial_advice: false
  max_cycle_runtime_secs: 120
memory:
  namespace: smoke/service-secret-otlp-agent
  write_policy: append_only
"#,
    )
    .expect("write agent spec");
    let out = run_csm(&[
        "service",
        "install",
        "--spec",
        spec.to_str().expect("utf8 spec"),
        "--service-root",
        root.join("service").to_str().expect("utf8 service root"),
        "--otlp-endpoint",
        "https://collector.example.invalid/v1/traces?token=secret",
        "--json",
    ]);
    assert!(
        !out.status.success(),
        "expected secret-bearing endpoint rejection, stdout:\n{}",
        String::from_utf8_lossy(&out.stdout)
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("--otlp-endpoint must not contain credentials"),
        "stderr:\n{stderr}"
    );
}

#[test]
fn csm_service_install_rejects_secret_bearing_otlp_endpoint_from_env() {
    let root = unique_test_temp_dir("csm-service-secret-otlp-env");
    let spec = root.join("agent.yaml");
    fs::write(
        &spec,
        r#"schema: adl.long_lived_agent_spec.v1
agent_instance_id: service-secret-otlp-env-agent
display_name: Service Secret OTLP Env Agent
state_root: state
workflow:
  kind: demo_adapter
  name: service_secret_otlp_env_probe
  run_args: {}
heartbeat:
  interval_secs: 1
  max_cycles: 1
  stale_lease_after_secs: 60
safety:
  allow_network: false
  allow_broker: false
  allow_filesystem_writes_outside_state_root: false
  allow_real_world_side_effects: false
  require_public_artifact_sanitization: true
  financial_advice: false
  max_cycle_runtime_secs: 120
memory:
  namespace: smoke/service-secret-otlp-env-agent
  write_policy: append_only
"#,
    )
    .expect("write agent spec");
    let service_root = root.join("service");
    let out = run_csm_with_env(
        &[
            "service",
            "install",
            "--spec",
            spec.to_str().expect("utf8 spec"),
            "--service-root",
            service_root.to_str().expect("utf8 service root"),
            "--json",
        ],
        &[(
            "ADL_OTEL_EXPORTER_OTLP_ENDPOINT",
            "https://user:secret@collector.example.invalid/v1/traces",
        )],
    );
    assert!(
        !out.status.success(),
        "expected env endpoint rejection, stdout:\n{}",
        String::from_utf8_lossy(&out.stdout)
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("--otlp-endpoint must not contain credentials"),
        "stderr:\n{stderr}"
    );
    assert!(!service_root.join("service_manifest.json").exists());
    assert!(!service_root.join("csm.launchd.plist").exists());
}

#[test]
fn csm_service_local_start_stop_retains_status_checkpoint_and_observability() {
    let root = unique_test_temp_dir("csm-service-local");
    let spec = root.join("agent.yaml");
    fs::write(
        &spec,
        r#"schema: adl.long_lived_agent_spec.v1
agent_instance_id: service-local-agent
display_name: Service Local Agent
state_root: state
workflow:
  kind: demo_adapter
  name: service_local_probe
  run_args: {}
heartbeat:
  interval_secs: 10
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
memory:
  namespace: smoke/service-local-agent
  write_policy: append_only
"#,
    )
    .expect("write agent spec");
    let service_root = root.join("service");
    let csm_bin = resolve_csm_exe();
    let install = run_csm(&[
        "service",
        "install",
        "--spec",
        spec.to_str().expect("utf8 spec"),
        "--service-root",
        service_root.to_str().expect("utf8 service root"),
        "--manager",
        "local",
        "--label",
        "com.agentlogic.csm.test-local",
        "--csm-bin",
        csm_bin.to_str().expect("utf8 csm bin"),
        "--checkpoint-interval-secs",
        "1",
        "--json",
    ]);
    assert!(
        install.status.success(),
        "install stderr:\n{}",
        String::from_utf8_lossy(&install.stderr)
    );

    let start = run_csm(&[
        "service",
        "start",
        "--service-root",
        service_root.to_str().expect("utf8 service root"),
        "--json",
    ]);
    assert!(
        start.status.success(),
        "start stderr:\n{}",
        String::from_utf8_lossy(&start.stderr)
    );
    std::thread::sleep(std::time::Duration::from_millis(1500));

    let status = run_csm(&[
        "service",
        "status",
        "--service-root",
        service_root.to_str().expect("utf8 service root"),
        "--json",
    ]);
    assert!(
        status.status.success(),
        "status stderr:\n{}",
        String::from_utf8_lossy(&status.stderr)
    );
    let service_status: serde_json::Value =
        serde_json::from_slice(&status.stdout).expect("parse service status stdout");
    assert_eq!(service_status["runtime_owner"], "csm");
    assert_eq!(service_status["broad_process_scan"], false);
    assert_eq!(service_status["uses_ps"], false);
    assert!(root.join("state/daemon_status.json").exists());
    assert!(root.join("state/continuity_checkpoint.json").exists());
    assert!(service_root.join("logs/observability.log").exists());
    assert!(service_root.join("logs/otel_status.json").exists());

    let stop = run_csm(&[
        "service",
        "stop",
        "--service-root",
        service_root.to_str().expect("utf8 service root"),
        "--json",
    ]);
    assert!(
        stop.status.success(),
        "stop stderr:\n{}",
        String::from_utf8_lossy(&stop.stderr)
    );
    let service_status: serde_json::Value =
        serde_json::from_slice(&stop.stdout).expect("parse stop status stdout");
    assert_eq!(service_status["service_state"], "stopped_or_requested");
    let agent_status: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(root.join("state/status.json")).unwrap())
            .expect("parse agent status");
    assert_eq!(agent_status["state"], "stopped");
    assert_eq!(
        agent_status["last_error"]["class"],
        "operator_stop_requested"
    );
}

#[test]
fn csm_service_local_start_refuses_unverified_live_pid_metadata() {
    let root = unique_test_temp_dir("csm-service-unverified-pid");
    let spec = root.join("agent.yaml");
    fs::write(
        &spec,
        r#"schema: adl.long_lived_agent_spec.v1
agent_instance_id: service-unverified-pid-agent
display_name: Service Unverified PID Agent
state_root: state
workflow:
  kind: demo_adapter
  name: service_unverified_pid_probe
  run_args: {}
heartbeat:
  interval_secs: 10
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
memory:
  namespace: smoke/service-unverified-pid-agent
  write_policy: append_only
"#,
    )
    .expect("write agent spec");
    let service_root = root.join("service");
    let csm_bin = resolve_csm_exe();
    let install = run_csm(&[
        "service",
        "install",
        "--spec",
        spec.to_str().expect("utf8 spec"),
        "--service-root",
        service_root.to_str().expect("utf8 service root"),
        "--manager",
        "local",
        "--label",
        "com.agentlogic.csm.test-unverified-pid",
        "--csm-bin",
        csm_bin.to_str().expect("utf8 csm bin"),
        "--checkpoint-interval-secs",
        "1",
        "--json",
    ]);
    assert!(
        install.status.success(),
        "install stderr:\n{}",
        String::from_utf8_lossy(&install.stderr)
    );
    fs::write(
        service_root.join("csm-service.pid"),
        std::process::id().to_string(),
    )
    .expect("write live but unowned pid");

    let start = run_csm(&[
        "service",
        "start",
        "--service-root",
        service_root.to_str().expect("utf8 service root"),
        "--json",
    ]);
    assert!(
        !start.status.success(),
        "expected unverified live pid refusal, stdout:\n{}",
        String::from_utf8_lossy(&start.stdout)
    );
    let stderr = String::from_utf8_lossy(&start.stderr);
    assert!(
        stderr.contains("refused live but unverified pid metadata"),
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
