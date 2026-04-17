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
