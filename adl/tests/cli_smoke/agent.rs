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
heartbeat:
  interval_secs: 1
  max_cycles: 3
  stale_lease_after_secs: 60
safety:
  allow_network: false
  allow_broker: false
  financial_advice: false
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
    assert!(root
        .join("state/cycles/cycle-000001/heartbeat.json")
        .exists());
    assert!(root
        .join("state/cycles/cycle-000002/heartbeat.json")
        .exists());
    assert!(root
        .join("state/cycles/cycle-000003/heartbeat.json")
        .exists());

    let status = run_adl(&["agent", "status", "--spec", spec_str]);
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
}
