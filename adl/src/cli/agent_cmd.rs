use anyhow::{anyhow, Result};
use std::path::PathBuf;

use ::adl::long_lived_agent::{self, InspectOptions, RunOptions, TickOptions};

pub(crate) fn real_agent(args: &[String]) -> Result<()> {
    let Some(subcommand) = args.first().map(|arg| arg.as_str()) else {
        return Err(anyhow!(
            "agent requires a subcommand: tick | run | status | inspect | stop"
        ));
    };

    match subcommand {
        "tick" => real_tick(&args[1..]),
        "run" => real_run(&args[1..]),
        "status" => real_status(&args[1..]),
        "inspect" => real_inspect(&args[1..]),
        "stop" => real_stop(&args[1..]),
        "--help" | "-h" | "help" => {
            println!("{}", super::usage::usage());
            Ok(())
        }
        other => Err(anyhow!(
            "unknown agent subcommand '{other}' (expected tick, run, status, inspect, stop)"
        )),
    }
}

fn real_tick(args: &[String]) -> Result<()> {
    let mut parsed = AgentArgs::default();
    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--spec" => {
                parsed.spec = Some(PathBuf::from(required_value(args, i, "--spec")?));
                i += 1;
            }
            "--recover-stale-lease" => parsed.recover_stale_lease = true,
            "--json" => parsed.json_output = true,
            "--help" | "-h" => {
                println!("{}", super::usage::usage());
                return Ok(());
            }
            other => return Err(anyhow!("unknown arg for agent tick: {other}")),
        }
        i += 1;
    }
    let status = long_lived_agent::tick(
        &parsed.spec()?,
        TickOptions {
            recover_stale_lease: parsed.recover_stale_lease,
        },
    )?;
    print_status(&status, parsed.json_output)
}

fn real_run(args: &[String]) -> Result<()> {
    let mut parsed = AgentArgs::default();
    let mut max_cycles: Option<u64> = None;
    let mut interval_secs: Option<u64> = None;
    let mut no_sleep = false;

    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--spec" => {
                parsed.spec = Some(PathBuf::from(required_value(args, i, "--spec")?));
                i += 1;
            }
            "--max-cycles" => {
                max_cycles = Some(parse_u64(required_value(args, i, "--max-cycles")?)?);
                i += 1;
            }
            "--interval-secs" => {
                interval_secs = Some(parse_u64(required_value(args, i, "--interval-secs")?)?);
                i += 1;
            }
            "--no-sleep" => no_sleep = true,
            "--recover-stale-lease" => parsed.recover_stale_lease = true,
            "--json" => parsed.json_output = true,
            "--help" | "-h" => {
                println!("{}", super::usage::usage());
                return Ok(());
            }
            other => return Err(anyhow!("unknown arg for agent run: {other}")),
        }
        i += 1;
    }
    let max_cycles = max_cycles.ok_or_else(|| anyhow!("agent run requires --max-cycles <n>"))?;
    let status = long_lived_agent::run(
        &parsed.spec()?,
        RunOptions {
            max_cycles,
            interval_secs,
            no_sleep,
            recover_stale_lease: parsed.recover_stale_lease,
        },
    )?;
    print_status(&status, parsed.json_output)
}

fn real_status(args: &[String]) -> Result<()> {
    let mut parsed = AgentArgs::default();
    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--spec" => {
                parsed.spec = Some(PathBuf::from(required_value(args, i, "--spec")?));
                i += 1;
            }
            "--json" => parsed.json_output = true,
            "--help" | "-h" => {
                println!("{}", super::usage::usage());
                return Ok(());
            }
            other => return Err(anyhow!("unknown arg for agent status: {other}")),
        }
        i += 1;
    }
    let status = long_lived_agent::status(&parsed.spec()?)?;
    print_status(&status, parsed.json_output)
}

fn real_inspect(args: &[String]) -> Result<()> {
    let mut parsed = AgentArgs::default();
    let mut cycle_id: Option<String> = None;
    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--spec" => {
                parsed.spec = Some(PathBuf::from(required_value(args, i, "--spec")?));
                i += 1;
            }
            "--cycle" => {
                cycle_id = Some(required_value(args, i, "--cycle")?.to_string());
                i += 1;
            }
            "--json" => parsed.json_output = true,
            "--help" | "-h" => {
                println!("{}", super::usage::usage());
                return Ok(());
            }
            other => return Err(anyhow!("unknown arg for agent inspect: {other}")),
        }
        i += 1;
    }
    let packet = long_lived_agent::inspect(&parsed.spec()?, InspectOptions { cycle_id })?;
    print_inspection(&packet, parsed.json_output)
}

fn real_stop(args: &[String]) -> Result<()> {
    let mut parsed = AgentArgs::default();
    let mut reason: Option<String> = None;
    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--spec" => {
                parsed.spec = Some(PathBuf::from(required_value(args, i, "--spec")?));
                i += 1;
            }
            "--reason" => {
                reason = Some(required_value(args, i, "--reason")?.to_string());
                i += 1;
            }
            "--json" => parsed.json_output = true,
            "--help" | "-h" => {
                println!("{}", super::usage::usage());
                return Ok(());
            }
            other => return Err(anyhow!("unknown arg for agent stop: {other}")),
        }
        i += 1;
    }
    let reason = reason.ok_or_else(|| anyhow!("agent stop requires --reason <text>"))?;
    let status = long_lived_agent::stop(&parsed.spec()?, &reason)?;
    print_status(&status, parsed.json_output)
}

#[derive(Default)]
struct AgentArgs {
    spec: Option<PathBuf>,
    recover_stale_lease: bool,
    json_output: bool,
}

impl AgentArgs {
    fn spec(&self) -> Result<PathBuf> {
        self.spec
            .clone()
            .ok_or_else(|| anyhow!("agent command requires --spec <agent-spec.yaml>"))
    }
}

fn required_value<'a>(args: &'a [String], index: usize, flag: &str) -> Result<&'a str> {
    args.get(index + 1)
        .map(|s| s.as_str())
        .ok_or_else(|| anyhow!("{flag} requires a value"))
}

fn parse_u64(raw: &str) -> Result<u64> {
    raw.parse::<u64>()
        .map_err(|_| anyhow!("expected unsigned integer, got '{raw}'"))
}

fn print_status(status: &long_lived_agent::StatusRecord, json_output: bool) -> Result<()> {
    if json_output {
        println!("{}", serde_json::to_string_pretty(status)?);
        return Ok(());
    }
    println!("agent: {}", status.agent_instance_id);
    println!("state: {}", status_state_label(&status.state));
    println!("completed cycles: {}", status.completed_cycle_count);
    println!(
        "last cycle: {}",
        match (&status.last_cycle_id, &status.last_cycle_status) {
            (Some(cycle_id), Some(cycle_status)) => format!("{cycle_id} {cycle_status}"),
            (Some(cycle_id), None) => cycle_id.clone(),
            _ => "none".to_string(),
        }
    );
    println!(
        "active lease: {}",
        status
            .active_lease
            .as_ref()
            .map(|lease| lease.lease_id.as_str())
            .unwrap_or("none")
    );
    println!(
        "stop requested: {}",
        if status.stop_requested { "yes" } else { "no" }
    );
    println!("consecutive failures: {}", status.consecutive_failure_count);
    println!(
        "last error: {}",
        status
            .last_error
            .as_ref()
            .map(|error| format!("{}: {}", error.class, error.message))
            .unwrap_or_else(|| "none".to_string())
    );
    Ok(())
}

fn print_inspection(packet: &serde_json::Value, json_output: bool) -> Result<()> {
    if json_output {
        println!("{}", serde_json::to_string_pretty(packet)?);
        return Ok(());
    }
    println!(
        "agent: {}",
        packet
            .get("agent_instance_id")
            .and_then(serde_json::Value::as_str)
            .unwrap_or("unknown")
    );
    println!(
        "state: {}",
        packet
            .get("status")
            .and_then(|status| status.get("state"))
            .and_then(serde_json::Value::as_str)
            .unwrap_or("unknown")
    );
    println!(
        "status ref: {}",
        packet
            .get("status_ref")
            .and_then(serde_json::Value::as_str)
            .unwrap_or("status.json")
    );
    if let Some(cycle) = packet.get("selected_cycle") {
        println!(
            "cycle: {} {}",
            cycle
                .get("cycle_id")
                .and_then(serde_json::Value::as_str)
                .unwrap_or("unknown"),
            cycle
                .get("status")
                .and_then(serde_json::Value::as_str)
                .unwrap_or("unknown")
        );
        let refs = cycle.get("refs").unwrap_or(&serde_json::Value::Null);
        println!(
            "manifest: {}",
            refs.get("manifest")
                .and_then(serde_json::Value::as_str)
                .unwrap_or("missing")
        );
        println!(
            "guardrails: {} {}",
            refs.get("guardrail_report")
                .and_then(serde_json::Value::as_str)
                .unwrap_or("missing"),
            cycle
                .get("guardrails")
                .and_then(|guardrails| guardrails.get("status"))
                .and_then(serde_json::Value::as_str)
                .unwrap_or("unknown")
        );
        println!(
            "summary: {}",
            refs.get("cycle_summary")
                .and_then(serde_json::Value::as_str)
                .unwrap_or("missing")
        );
        println!(
            "run ref: {}",
            refs.get("run_ref")
                .and_then(serde_json::Value::as_str)
                .unwrap_or("missing")
        );
    } else {
        println!("cycle: none");
    }
    println!(
        "trace/query: {}",
        packet
            .get("trace_query_decision")
            .and_then(|decision| decision.get("status"))
            .and_then(serde_json::Value::as_str)
            .unwrap_or("unknown")
    );
    println!(
        "proof: {}",
        packet
            .get("reviewer_proof")
            .and_then(|proof| proof.get("status"))
            .and_then(serde_json::Value::as_str)
            .unwrap_or("unknown")
    );
    Ok(())
}

fn status_state_label(state: &long_lived_agent::AgentStatusState) -> &'static str {
    match state {
        long_lived_agent::AgentStatusState::NotStarted => "not_started",
        long_lived_agent::AgentStatusState::Idle => "idle",
        long_lived_agent::AgentStatusState::Leased => "leased",
        long_lived_agent::AgentStatusState::RunningCycle => "running_cycle",
        long_lived_agent::AgentStatusState::Stopped => "stopped",
        long_lived_agent::AgentStatusState::Failed => "failed",
        long_lived_agent::AgentStatusState::Completed => "completed",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;
    use std::sync::atomic::{AtomicU64, Ordering};

    static TEMP_SEQ: AtomicU64 = AtomicU64::new(0);

    fn args(values: &[&str]) -> Vec<String> {
        values.iter().map(|value| value.to_string()).collect()
    }

    fn temp_dir(prefix: &str) -> PathBuf {
        let path = std::env::temp_dir().join(format!(
            "adl-agent-cmd-{prefix}-{}-{}",
            std::process::id(),
            TEMP_SEQ.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir_all(&path).expect("create temp dir");
        path
    }

    fn write_spec(root: &Path) -> PathBuf {
        let spec = root.join("agent.yaml");
        fs::write(
            &spec,
            r#"schema: adl.long_lived_agent_spec.v1
agent_instance_id: agent-cmd-test
display_name: Agent Command Test
state_root: state
workflow:
  kind: demo_adapter
  name: agent_cmd_probe
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
  namespace: tests/agent-cmd
  write_policy: append_only
"#,
        )
        .expect("write spec");
        spec
    }

    fn assert_err_contains(result: Result<()>, needle: &str) {
        let err = result.expect_err("expected error");
        assert!(
            err.to_string().contains(needle),
            "expected '{needle}' in error '{err}'"
        );
    }

    fn assert_cycle_contract_exists(root: &Path, cycle_id: &str) {
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

    fn assert_continuity_contract_exists(root: &Path) {
        for artifact in [
            "agent_spec.locked.json",
            "continuity.json",
            "cycle_ledger.jsonl",
            "provider_binding_history.jsonl",
            "memory_index.json",
        ] {
            assert!(
                root.join("state").join(artifact).exists(),
                "missing {artifact}"
            );
        }
    }

    #[test]
    fn agent_dispatch_help_and_unknown_paths_are_reported() {
        assert!(real_agent(&args(&["help"])).is_ok());
        assert!(real_agent(&args(&["tick", "--help"])).is_ok());
        assert!(real_agent(&args(&["run", "--help"])).is_ok());
        assert!(real_agent(&args(&["status", "--help"])).is_ok());
        assert!(real_agent(&args(&["inspect", "--help"])).is_ok());
        assert!(real_agent(&args(&["stop", "--help"])).is_ok());

        assert_err_contains(real_agent(&args(&[])), "agent requires a subcommand");
        assert_err_contains(real_agent(&args(&["bogus"])), "unknown agent subcommand");
    }

    #[test]
    fn agent_argument_validation_reports_missing_values_and_unknown_args() {
        assert_err_contains(real_agent(&args(&["tick"])), "requires --spec");
        assert_err_contains(
            real_agent(&args(&["tick", "--spec"])),
            "--spec requires a value",
        );
        assert_err_contains(real_agent(&args(&["tick", "--bogus"])), "unknown arg");
        assert_err_contains(real_agent(&args(&["run"])), "requires --max-cycles");
        assert_err_contains(
            real_agent(&args(&["run", "--max-cycles", "not-a-number"])),
            "expected unsigned integer",
        );
        assert_err_contains(
            real_agent(&args(&[
                "run",
                "--max-cycles",
                "1",
                "--interval-secs",
                "nope",
            ])),
            "expected unsigned integer",
        );
        assert_err_contains(real_agent(&args(&["run", "--bogus"])), "unknown arg");
        assert_err_contains(real_agent(&args(&["status", "--bogus"])), "unknown arg");
        assert_err_contains(real_agent(&args(&["inspect", "--bogus"])), "unknown arg");
        assert_err_contains(
            real_agent(&args(&["inspect", "--cycle"])),
            "--cycle requires a value",
        );
        assert_err_contains(
            real_agent(&args(&["stop", "--reason"])),
            "--reason requires a value",
        );
        assert_err_contains(real_agent(&args(&["stop"])), "requires --reason");
        assert_err_contains(real_agent(&args(&["stop", "--bogus"])), "unknown arg");
    }

    #[test]
    fn agent_commands_execute_success_paths() {
        let root = temp_dir("success");
        let spec = write_spec(&root);
        let spec = spec.to_str().expect("utf8 spec path");

        assert!(real_agent(&args(&["tick", "--spec", spec])).is_ok());
        assert_continuity_contract_exists(&root);
        assert_cycle_contract_exists(&root, "cycle-000001");

        assert!(real_agent(&args(&[
            "run",
            "--spec",
            spec,
            "--max-cycles",
            "2",
            "--interval-secs",
            "1",
            "--no-sleep",
            "--recover-stale-lease",
        ]))
        .is_ok());
        assert_cycle_contract_exists(&root, "cycle-000003");
        let ledger =
            fs::read_to_string(root.join("state/cycle_ledger.jsonl")).expect("cycle ledger");
        assert_eq!(ledger.lines().count(), 3);

        assert!(real_agent(&args(&["status", "--spec", spec])).is_ok());
        assert!(real_agent(&args(&["status", "--spec", spec, "--json"])).is_ok());
        assert!(real_agent(&args(&["inspect", "--spec", spec])).is_ok());
        assert!(real_agent(&args(&[
            "inspect",
            "--spec",
            spec,
            "--cycle",
            "cycle-000002",
            "--json",
        ]))
        .is_ok());
        assert!(real_agent(&args(&[
            "stop",
            "--spec",
            spec,
            "--reason",
            "operator requested pause",
            "--json",
        ]))
        .is_ok());
        assert!(root.join("state/stop.json").exists());
    }
}
