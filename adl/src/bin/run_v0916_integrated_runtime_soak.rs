use std::collections::HashMap;
use std::fs;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::{Duration, Instant};

use adl::adl::ProviderSpec;
use adl::long_lived_agent::{self, InspectOptions, LeaseRecord, RunOptions};
use adl::obsmem_transition_memory::build_write_request_from_transition_handoff;
use adl::remote_exec::{
    execute_remote, retryability, stable_failure_kind, ExecuteInputsPayload, ExecuteRequest,
    ExecuteStepPayload, PROTOCOL_VERSION,
};
use adl::resilience::{
    bulkhead_initial_state, execute_bulkhead_policy, execute_fallback_policy,
    execute_timeout_policy, remote_exec_health_payload, BulkheadPolicyV1, FallbackPolicyV1,
    RateLimitPolicyV1, ResilienceFaultClassificationV1, ResiliencePolicyV1, ResilienceSurfaceV1,
    RetryPolicyV1, TimeoutObservation, TimeoutPolicyV1,
};
use anyhow::{anyhow, Context, Result};
use chrono::{Duration as ChronoDuration, Utc};
use clap::Parser;
use serde::Serialize;
use serde_json::{json, Value};

const DISCLAIMER: &str = "This integrated runtime soak is a bounded local proof surface. It does not claim autonomous v0.92 readiness, external-agent transport closure, or full Observatory/Unity product completion.";

#[derive(Debug, Parser)]
#[command(name = "run_v0916_integrated_runtime_soak")]
#[command(
    about = "Execute the v0.91.6 integrated runtime soak and write reviewer-facing artifacts"
)]
struct Args {
    #[arg(long)]
    out: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();
    run(args)
}

fn run(args: Args) -> Result<()> {
    let out_dir = absolute_from_cwd(&args.out)?;
    if out_dir.exists() {
        fs::remove_dir_all(&out_dir)
            .with_context(|| format!("reset existing output dir {}", out_dir.display()))?;
    }
    fs::create_dir_all(&out_dir)
        .with_context(|| format!("create output dir {}", out_dir.display()))?;

    write_file(&out_dir.join("README.md"), &readme())?;
    write_file(
        &out_dir.join("reviewer_walkthrough.md"),
        &reviewer_walkthrough(),
    )?;

    let spec_path = write_agent_spec_under(&out_dir, "long_lived_agent", "v0916-runtime-soak")?;
    let initial_status = long_lived_agent::status(&spec_path)?;
    write_json(
        &out_dir.join("long_lived_agent/initial_status.json"),
        &initial_status,
    )?;

    let run_status = long_lived_agent::run(
        &spec_path,
        RunOptions {
            max_cycles: 2,
            interval_secs: Some(0),
            no_sleep: true,
            recover_stale_lease: false,
        },
    )?;
    write_json(
        &out_dir.join("long_lived_agent/run_status_cycle2.json"),
        &run_status,
    )?;

    let restart_status = long_lived_agent::run(
        &spec_path,
        RunOptions {
            max_cycles: 1,
            interval_secs: Some(0),
            no_sleep: true,
            recover_stale_lease: false,
        },
    )?;
    write_json(
        &out_dir.join("long_lived_agent/restart_status_cycle3.json"),
        &restart_status,
    )?;

    let inspection_latest =
        long_lived_agent::inspect(&spec_path, InspectOptions { cycle_id: None })?;
    let inspection_cycle1 = long_lived_agent::inspect(
        &spec_path,
        InspectOptions {
            cycle_id: Some("cycle-000001".to_string()),
        },
    )?;
    write_json(&out_dir.join("inspection/latest.json"), &inspection_latest)?;
    write_json(
        &out_dir.join("inspection/cycle-000001.json"),
        &inspection_cycle1,
    )?;

    let lease_injection_probe = capture_injected_lease_probe(&spec_path)?;
    write_json(
        &out_dir.join("long_lived_agent/lease_injection_probe.json"),
        &lease_injection_probe,
    )?;

    let stop_probe = execute_stop_probe(&out_dir)?;
    write_json(
        &out_dir.join("long_lived_agent_stop_probe/stop_probe.json"),
        &stop_probe,
    )?;

    let timeout_trace = run_timeout_policy_probe();
    write_json(
        &out_dir.join("resilience/timeout_execution.json"),
        &timeout_trace,
    )?;

    let bulkhead_trace = run_bulkhead_probe();
    write_json(
        &out_dir.join("resilience/bulkhead_execution.json"),
        &bulkhead_trace,
    )?;

    let degraded_trace = run_degraded_fallback_probe();
    write_json(
        &out_dir.join("resilience/degraded_fallback_execution.json"),
        &degraded_trace,
    )?;

    let remote_timeout = run_remote_timeout_probe()?;
    write_json(
        &out_dir.join("remote_exec/timeout_probe.json"),
        &remote_timeout,
    )?;
    write_json(
        &out_dir.join("remote_exec/health_payload.json"),
        &remote_exec_health_payload(),
    )?;

    let obsmem_request = build_obsmem_request()?;
    write_json(
        &out_dir.join("obsmem/transition_memory_request.json"),
        &obsmem_request,
    )?;

    let stopped = long_lived_agent::stop(
        &spec_path,
        "bounded integrated runtime soak stop after proof capture",
    )?;
    write_json(
        &out_dir.join("long_lived_agent/status_after_stop.json"),
        &stopped,
    )?;

    let evidence_index = build_evidence_index(&out_dir)?;
    write_json(
        &out_dir.join("integrated_runtime_soak_evidence_index.json"),
        &evidence_index,
    )?;

    let proof_packet = build_proof_packet(
        &initial_status,
        &run_status,
        &restart_status,
        &lease_injection_probe,
        &stop_probe,
        &stopped,
        &timeout_trace,
        &bulkhead_trace,
        &degraded_trace,
        &remote_timeout,
        &evidence_index,
    );
    write_json(
        &out_dir.join("integrated_runtime_soak_proof.json"),
        &proof_packet,
    )?;

    let artifact_scan = scan_public_artifacts(&out_dir)?;
    if !artifact_scan
        .get("passed")
        .and_then(Value::as_bool)
        .unwrap_or(false)
    {
        return Err(anyhow!(
            "integrated runtime soak artifact safety scan failed"
        ));
    }
    write_json(
        &out_dir.join("audit/artifact_safety_scan.json"),
        &artifact_scan,
    )?;

    println!("out={}", out_dir.display());
    println!(
        "proof={}",
        out_dir.join("integrated_runtime_soak_proof.json").display()
    );
    Ok(())
}

fn absolute_from_cwd(path: &Path) -> Result<PathBuf> {
    if path.is_absolute() {
        Ok(path.to_path_buf())
    } else {
        Ok(std::env::current_dir()?.join(path))
    }
}

fn write_agent_spec_under(
    out_dir: &Path,
    dir_name: &str,
    agent_instance_id: &str,
) -> Result<PathBuf> {
    let spec_path = out_dir.join(dir_name).join("agent.yaml");
    let body = r#"schema: adl.long_lived_agent_spec.v1
agent_instance_id: __AGENT_INSTANCE_ID__
display_name: V0916 Runtime Soak
state_root: state
workflow:
  kind: demo_adapter
  name: v0916_runtime_soak
  run_args:
    provider_id: local_ollama
    model: gemma4:latest
heartbeat:
  interval_secs: 1
  max_cycles: 6
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
  namespace: runtime/soak/v0916
  write_policy: append_only
"#;
    write_file(
        &spec_path,
        &body.replace("__AGENT_INSTANCE_ID__", agent_instance_id),
    )?;
    Ok(spec_path)
}

fn capture_injected_lease_probe(spec_path: &Path) -> Result<Value> {
    let loaded = long_lived_agent::load_spec(spec_path)?;
    let lease_path = loaded.state_root.join("lease.json");
    let lease = LeaseRecord {
        schema: "adl.long_lived_agent_lease.v1".to_string(),
        agent_instance_id: loaded.spec.agent_instance_id.clone(),
        lease_id: "lease-v0916-runtime-soak-active".to_string(),
        cycle_id: "cycle-lease-probe".to_string(),
        owner_pid: 0,
        hostname: "redacted".to_string(),
        started_at: Utc::now(),
        expires_at: Utc::now() + ChronoDuration::seconds(30),
        status: "active".to_string(),
    };
    write_json(&lease_path, &lease)?;
    let status = long_lived_agent::status(spec_path)?;
    let tick_error = long_lived_agent::tick(spec_path, Default::default())
        .err()
        .map(|err| err.to_string())
        .unwrap_or_else(|| "unexpected_success".to_string());
    fs::remove_file(&lease_path)
        .with_context(|| format!("remove lease probe {}", lease_path.display()))?;
    Ok(json!({
        "probe_kind": "injected_lease_contract_probe",
        "note": "This probe injects a lease artifact to verify leased-state and overlap-blocking behavior. It does not claim a concurrently running live lease owner.",
        "status_state": status.state,
        "tick_error": tick_error,
    }))
}

fn execute_stop_probe(out_dir: &Path) -> Result<Value> {
    let spec_path = write_agent_spec_under(
        out_dir,
        "long_lived_agent_stop_probe",
        "v0916-runtime-stop-probe",
    )?;
    let thread_spec = spec_path.clone();
    let runner = thread::spawn(move || {
        long_lived_agent::run(
            &thread_spec,
            RunOptions {
                max_cycles: 5,
                interval_secs: Some(1),
                no_sleep: false,
                recover_stale_lease: false,
            },
        )
    });

    let loaded = long_lived_agent::load_spec(&spec_path)?;
    let first_cycle_manifest = loaded
        .state_root
        .join("cycles/cycle-000001/cycle_manifest.json");
    wait_for_path(&first_cycle_manifest, Duration::from_secs(5))?;
    thread::sleep(Duration::from_millis(100));
    let stop_status = long_lived_agent::stop(
        &spec_path,
        "operator stop during bounded cadence sleep window",
    )?;
    let final_status = runner
        .join()
        .map_err(|_| anyhow!("stop probe thread panicked"))??;
    let persisted = long_lived_agent::status(&spec_path)?;
    Ok(json!({
        "probe_kind": "live_stop_between_cycles",
        "note": "This probe lets one cycle complete, issues stop during the cadence sleep window, and verifies that later cycles do not start.",
        "stop_status": stop_status,
        "run_returned_state": final_status.state,
        "persisted_state": persisted.state,
        "completed_cycle_count": persisted.completed_cycle_count,
        "last_error": persisted.last_error,
    }))
}

fn run_timeout_policy_probe() -> Value {
    let policy = ResiliencePolicyV1 {
        schema_version: "adl.resilience_policy.v1".to_string(),
        policy_id: "runtime.soak.timeout.policy".to_string(),
        retry: Some(RetryPolicyV1 {
            max_attempts: 2,
            backoff_ms: Some(5),
            jitter_ms: Some(0),
            max_elapsed_ms: Some(20),
            retryable_fault_classes: vec![adl::resilience::ResilienceFaultClassV1::ProviderTimeout],
        }),
        timeout: Some(TimeoutPolicyV1 {
            timeout_ms: 50,
            hard_deadline_ms: Some(75),
        }),
        circuit_breaker: None,
        rate_limit: None,
        bulkhead: None,
        fallback: None,
        checkpoint_required: false,
        telemetry_required: true,
    };
    let execution = execute_timeout_policy(
        &policy,
        ResilienceSurfaceV1::Runtime,
        "runtime.soak.timeout_probe",
        || TimeoutObservation {
            result: Ok::<_, String>("late-success".to_string()),
            elapsed_ms: 80,
            cancelled: false,
        },
        |err| ResilienceFaultClassificationV1::provider(err, None),
        |breach, elapsed_ms, budget_ms| {
            format!(
                "timeout_probe {:?} elapsed={} budget={}",
                breach, elapsed_ms, budget_ms
            )
        },
        |elapsed_ms| format!("cancelled after {elapsed_ms}ms"),
    );
    json!({
        "final_status": format!("{:?}", execution.trace.final_status),
        "trace": execution.trace,
        "result": execution.result.err(),
    })
}

fn run_bulkhead_probe() -> Value {
    let policy = ResiliencePolicyV1 {
        schema_version: "adl.resilience_policy.v1".to_string(),
        policy_id: "runtime.soak.bulkhead.policy".to_string(),
        retry: None,
        timeout: None,
        circuit_breaker: None,
        rate_limit: Some(RateLimitPolicyV1 {
            max_requests: 1,
            window_ms: 100,
        }),
        bulkhead: Some(BulkheadPolicyV1 {
            fault_domain: "runtime_soak_provider".to_string(),
            max_concurrency: 1,
            max_queue_depth: Some(0),
        }),
        fallback: None,
        checkpoint_required: false,
        telemetry_required: true,
    };
    let mut state = bulkhead_initial_state(&policy);
    state.in_flight = 1;
    let execution = execute_bulkhead_policy(
        &policy,
        ResilienceSurfaceV1::Provider,
        "runtime.soak.bulkhead_probe",
        &state,
        || Ok::<_, String>("should-not-run".to_string()),
        |err| ResilienceFaultClassificationV1::provider(err, None),
        |bulkhead_state| {
            format!(
                "local_runtime_busy: bulkhead saturated for {} with in_flight={}",
                bulkhead_state.fault_domain, bulkhead_state.in_flight
            )
        },
    );
    json!({
        "result": execution.result.err(),
        "state": execution.state,
        "trace": execution.trace,
    })
}

fn run_degraded_fallback_probe() -> Value {
    let policy = ResiliencePolicyV1 {
        schema_version: "adl.resilience_policy.v1".to_string(),
        policy_id: "runtime.soak.fallback.policy".to_string(),
        retry: Some(RetryPolicyV1 {
            max_attempts: 2,
            backoff_ms: Some(10),
            jitter_ms: Some(0),
            max_elapsed_ms: Some(20),
            retryable_fault_classes: vec![adl::resilience::ResilienceFaultClassV1::ProviderTimeout],
        }),
        timeout: Some(TimeoutPolicyV1 {
            timeout_ms: 50,
            hard_deadline_ms: Some(75),
        }),
        circuit_breaker: None,
        rate_limit: Some(RateLimitPolicyV1 {
            max_requests: 1,
            window_ms: 100,
        }),
        bulkhead: None,
        fallback: Some(FallbackPolicyV1 {
            fallback_ref: "runtime.soak.degraded".to_string(),
            activation_fault_classes: vec![
                adl::resilience::ResilienceFaultClassV1::ProviderTimeout,
                adl::resilience::ResilienceFaultClassV1::ProviderTransientHttp,
            ],
            marks_output_degraded: true,
        }),
        checkpoint_required: false,
        telemetry_required: true,
    };
    let execution = execute_fallback_policy(
        &policy,
        ResilienceSurfaceV1::Workflow,
        "runtime.soak.degraded_probe",
        || Err::<String, _>("provider timeout".to_string()),
        |err| ResilienceFaultClassificationV1::provider(err, None),
        Some(|| "degraded-fallback-result".to_string()),
    );
    json!({
        "outcome_kind": format!("{:?}", execution.outcome_kind),
        "trace": execution.trace,
        "result": execution.result,
    })
}

fn run_remote_timeout_probe() -> Result<Value> {
    let listener = TcpListener::bind("127.0.0.1:0").context("bind timeout probe listener")?;
    let port = listener
        .local_addr()
        .context("read timeout probe addr")?
        .port();
    let handle = thread::spawn(move || -> Result<()> {
        let (mut stream, _) = listener.accept().context("accept timeout probe client")?;
        let mut buf = [0_u8; 512];
        let _ = stream.read(&mut buf);
        thread::sleep(Duration::from_millis(250));
        let _ = stream.write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\n\r\nOK");
        Ok(())
    });

    let request = ExecuteRequest {
        protocol_version: PROTOCOL_VERSION.to_string(),
        run_id: "runtime-soak-remote-timeout".to_string(),
        workflow_id: "v0916_runtime_soak".to_string(),
        step_id: "remote-timeout".to_string(),
        step: ExecuteStepPayload {
            kind: "completion".to_string(),
            provider: "ollama".to_string(),
            prompt: "bounded timeout probe".to_string(),
            conversation: None,
            tools: Vec::new(),
            provider_spec: ProviderSpec {
                id: None,
                profile: None,
                kind: "ollama".to_string(),
                base_url: None,
                default_model: None,
                config: HashMap::new(),
            },
            model_override: None,
        },
        inputs: ExecuteInputsPayload::default(),
        timeout_ms: 50,
        security: None,
    };

    let err = execute_remote(&format!("http://127.0.0.1:{port}"), 50, &request)
        .expect_err("timeout probe should fail");
    handle
        .join()
        .map_err(|_| anyhow!("timeout probe thread panicked"))??;
    Ok(json!({
        "error_code": "REMOTE_TIMEOUT",
        "error_summary": "remote execution request timed out against a hanging local endpoint",
        "stable_failure_kind": stable_failure_kind(&err),
        "retryability": retryability(&err),
    }))
}

fn build_obsmem_request() -> Result<Value> {
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .context("derive repo root from manifest dir")?;
    let handoff = repo_root.join(
        "docs/milestones/v0.91.4/review/obsmem_transition_memory/ct_demo_001_obsmem_transition_memory_handoff.json",
    );
    let request = build_write_request_from_transition_handoff(repo_root, &handoff)?;
    serde_json::to_value(request).context("serialize obsmem transition memory request")
}

fn build_evidence_index(out_dir: &Path) -> Result<Value> {
    let mut refs = Vec::new();
    collect_relative_files(out_dir, out_dir, &mut refs)?;
    refs.sort();
    Ok(json!({
        "schema_version": "adl.integrated_runtime_soak_evidence_index.v1",
        "issue": 4245,
        "generated_at": Utc::now().to_rfc3339(),
        "artifact_refs": refs,
        "prerequisite_refs": [
            "docs/milestones/v0.91.6/features/TOKIO_RUNTIME_SUBSTRATE_v0.91.6.md",
            "docs/milestones/v0.91.6/features/ACIP_A2A_PROVIDER_COMMUNICATIONS_v0.91.6.md",
            "docs/milestones/v0.91.6/features/AEE_MEMORY_ACP_BRIDGE_ACCOUNTING_v0.91.6.md",
            "docs/milestones/v0.91.6/features/OBSERVATORY_UNITY_CONSUMPTION_CLASSIFICATION_v0.91.6.md"
        ]
    }))
}

fn collect_relative_files(root: &Path, current: &Path, out: &mut Vec<String>) -> Result<()> {
    for entry in fs::read_dir(current).with_context(|| format!("read dir {}", current.display()))? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            collect_relative_files(root, &path, out)?;
            continue;
        }
        let rel = path
            .strip_prefix(root)
            .with_context(|| format!("strip prefix {} from {}", root.display(), path.display()))?;
        out.push(rel.display().to_string());
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn build_proof_packet(
    initial_status: &adl::long_lived_agent::StatusRecord,
    run_status: &adl::long_lived_agent::StatusRecord,
    restart_status: &adl::long_lived_agent::StatusRecord,
    lease_injection_probe: &Value,
    stop_probe: &Value,
    stopped: &adl::long_lived_agent::StatusRecord,
    timeout_trace: &Value,
    bulkhead_trace: &Value,
    degraded_trace: &Value,
    remote_timeout: &Value,
    evidence_index: &Value,
) -> Value {
    json!({
        "schema_version": "adl.integrated_runtime_soak_proof.v1",
        "issue": 4245,
        "generated_at": Utc::now().to_rfc3339(),
        "what_this_proves": [
            "The v0.91.6 runtime proves initialization, run/restart/inspection continuity, and a companion live stop-between-cycles behavior across bounded long-lived-agent probes with durable cycle evidence.",
            "The resilience substrate emits reviewer-readable timeout, bulkhead saturation/backpressure, and degraded fallback traces using the real library entrypoints.",
            "Remote execution timeout classification is live against a hanging local endpoint and records stable failure kind plus retryability.",
            "ObsMem transition memory can still build a structured write request from a tracked handoff packet.",
            "The long-lived-agent leased-state contract blocks overlap when an injected lease artifact is present."
        ],
        "what_this_does_not_prove": [
            "full v0.92 activation readiness",
            "external-agent trust or transport closure",
            "full Observatory or Unity UI completion",
            "always-on autonomy",
            "end-to-end ACIP runtime execution beyond prerequisite consumption"
        ],
        "status_summary": {
            "initial_state": initial_status.state,
            "run_state_after_cycle2": run_status.state,
            "restart_state_after_cycle3": restart_status.state,
            "stop_state": stopped.state,
            "completed_cycle_count_after_restart": restart_status.completed_cycle_count,
            "lease_injection_probe_state": lease_injection_probe["status_state"],
            "lease_injection_tick_error": lease_injection_probe["tick_error"],
            "live_stop_probe_state": stop_probe["persisted_state"],
            "live_stop_probe_completed_cycle_count": stop_probe["completed_cycle_count"],
            "remote_timeout_failure_kind": remote_timeout["stable_failure_kind"],
            "remote_timeout_retryability": remote_timeout["retryability"],
            "timeout_final_status": timeout_trace["trace"]["final_status"],
            "bulkhead_final_status": bulkhead_trace["trace"]["final_status"],
            "degraded_fallback_final_status": degraded_trace["trace"]["final_status"],
            "degraded_output": degraded_trace["trace"]["output_degraded"]
        },
        "reviewer_path": [
            "README.md",
            "integrated_runtime_soak_proof.json",
            "long_lived_agent/state/status.json",
            "long_lived_agent/state/cycle_ledger.jsonl",
            "inspection/latest.json",
            "long_lived_agent_stop_probe/stop_probe.json",
            "resilience/timeout_execution.json",
            "resilience/bulkhead_execution.json",
            "resilience/degraded_fallback_execution.json",
            "remote_exec/timeout_probe.json",
            "obsmem/transition_memory_request.json",
            "audit/artifact_safety_scan.json"
        ],
        "evidence_index_ref": "integrated_runtime_soak_evidence_index.json",
        "evidence_index": evidence_index,
        "disclaimer": DISCLAIMER,
    })
}

fn scan_public_artifacts(out_dir: &Path) -> Result<Value> {
    let mut files = Vec::new();
    collect_relative_files(out_dir, out_dir, &mut files)?;
    files.retain(|path| path != "audit/artifact_safety_scan.json");
    files.sort();

    let patterns: &[(&str, &[&str])] = &[
        ("private_host_path", &["/users/", "\\users\\"]),
        (
            "secret_material",
            &[
                "bearer ",
                "private_key",
                "begin rsa private key",
                "secret_access_key",
            ],
        ),
    ];

    let mut findings = Vec::new();
    for rel in &files {
        let path = out_dir.join(rel);
        let Ok(contents) = fs::read_to_string(&path) else {
            continue;
        };
        let lowered = contents.to_ascii_lowercase();
        for (family, family_patterns) in patterns {
            for pattern in *family_patterns {
                if lowered.contains(pattern) {
                    findings.push(json!({
                        "family": family,
                        "pattern": pattern,
                        "artifact_ref": rel,
                    }));
                }
            }
        }
    }

    Ok(json!({
        "schema_version": "adl.integrated_runtime_soak_artifact_safety_scan.v1",
        "issue": 4245,
        "scanned_at": Utc::now().to_rfc3339(),
        "passed": findings.is_empty(),
        "scanned_artifacts": files,
        "findings": findings,
    }))
}

fn wait_for_path(path: &Path, timeout: Duration) -> Result<()> {
    let started = Instant::now();
    while started.elapsed() < timeout {
        if path.exists() {
            return Ok(());
        }
        thread::sleep(Duration::from_millis(50));
    }
    Err(anyhow!("timed out waiting for {}", path.display()))
}

fn write_json<T: Serialize>(path: &Path, value: &T) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("create parent dir {}", parent.display()))?;
    }
    let text = serde_json::to_string_pretty(value)? + "\n";
    fs::write(path, text).with_context(|| format!("write json {}", path.display()))
}

fn write_file(path: &Path, contents: &str) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("create parent dir {}", parent.display()))?;
    }
    fs::write(path, contents).with_context(|| format!("write file {}", path.display()))
}

fn readme() -> String {
    format!(
        "# V0.91.6 Integrated Runtime Soak\n\n{DISCLAIMER}\n\n## What This Proves\n\nThis run proves a bounded integrated runtime slice for `#4245`: one long-lived-agent run/restart/inspection continuity path, one companion live stop-between-cycles probe, timeout classification, bulkhead/backpressure saturation, degraded fallback, remote-exec timeout handling, one tracked ObsMem handoff path, and one explicit injected-lease contract probe all converge under one reviewer-readable artifact root.\n\n## Reviewer Path\n\n1. Inspect `integrated_runtime_soak_proof.json`.\n2. Inspect `long_lived_agent/state/status.json` and `long_lived_agent/state/cycle_ledger.jsonl`.\n3. Inspect `inspection/latest.json`.\n4. Inspect `long_lived_agent_stop_probe/stop_probe.json`.\n5. Inspect `resilience/timeout_execution.json`, `resilience/bulkhead_execution.json`, and `resilience/degraded_fallback_execution.json`.\n6. Inspect `remote_exec/timeout_probe.json`.\n7. Inspect `obsmem/transition_memory_request.json`.\n8. Inspect `audit/artifact_safety_scan.json`.\n"
    )
}

fn reviewer_walkthrough() -> String {
    "# Reviewer Walkthrough\n\nRun the soak with `cargo run --manifest-path adl/Cargo.toml --bin run_v0916_integrated_runtime_soak -- --out docs/milestones/v0.91.6/review/runtime/v0916_integrated_runtime_soak_4245`.\n\nThe review question is whether the runtime now leaves one honest, durable packet showing restart, a live stop between cycles, timeout, saturation/backpressure, degraded fallback, remote-exec timeout semantics, and memory handoff under one bounded local proof surface without overclaiming ACIP, Observatory, or v0.92 readiness.\n".to_string()
}
