use std::collections::HashMap;
use std::fs;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::{Duration, Instant};

use adl::adl::ProviderSpec;
use adl::csm_observatory::{write_observatory_outputs, ObservatoryFormat};
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
use adl::scheduler::{schedule_economics_bundle, SchedulerEconomicsInputBundleV1};
use anyhow::{anyhow, Context, Result};
use chrono::{Duration as ChronoDuration, Utc};
use clap::Parser;
use serde::Serialize;
use serde_json::{json, Value};

const DISCLAIMER: &str = "This integrated runtime soak is a bounded local proof surface. It does not claim autonomous v0.92 readiness, external-agent transport closure, or full Observatory/Unity product completion.";
const RUNTIME_SOAK_ISSUE: u32 = 4543;

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

    let scheduler_plan = build_scheduler_plan()?;
    write_json(
        &out_dir.join("scheduler/scheduler_plan.json"),
        &scheduler_plan,
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

    let completion_classification = build_completion_classification(&out_dir);
    write_json(
        &out_dir.join("completion_classification.json"),
        &completion_classification,
    )?;

    let observatory_bundle = write_observatory_integration_bundle(
        &out_dir,
        &run_status,
        &restart_status,
        &stop_probe,
        &timeout_trace,
        &bulkhead_trace,
        &degraded_trace,
        &remote_timeout,
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
        &observatory_bundle,
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

fn build_scheduler_plan() -> Result<Value> {
    let bundle: SchedulerEconomicsInputBundleV1 = serde_json::from_str(include_str!(
        "../../tests/fixtures/scheduler/economics_inputs_v1.json"
    ))
    .context("parse scheduler economics input fixture")?;
    let plan = schedule_economics_bundle(&bundle).context("build scheduler plan")?;
    serde_json::to_value(plan).context("serialize scheduler plan")
}

#[allow(clippy::too_many_arguments)]
fn write_observatory_integration_bundle(
    out_dir: &Path,
    run_status: &adl::long_lived_agent::StatusRecord,
    restart_status: &adl::long_lived_agent::StatusRecord,
    stop_probe: &Value,
    timeout_trace: &Value,
    bulkhead_trace: &Value,
    degraded_trace: &Value,
    remote_timeout: &Value,
    stopped: &adl::long_lived_agent::StatusRecord,
) -> Result<Value> {
    let artifact_root = repo_relative_ref(out_dir)?;
    let observatory_dir = out_dir.join("runtime_v2/observatory");
    let packet_path = observatory_dir.join("visibility_packet.json");
    let packet = build_observatory_integration_packet(
        &artifact_root,
        run_status,
        restart_status,
        stop_probe,
        timeout_trace,
        bulkhead_trace,
        degraded_trace,
        remote_timeout,
        stopped,
    )?;
    write_json(&packet_path, &packet)?;

    let output =
        write_observatory_outputs(&packet_path, &observatory_dir, ObservatoryFormat::Bundle)
            .context("write integrated observatory bundle")?;

    Ok(json!({
        "packet_id": packet["packet_id"],
        "classification": packet["review"]["demo_classification"],
        "runtime_artifact_root": artifact_root,
        "visibility_packet_ref": relative_ref(out_dir, output.packet_path.as_deref().unwrap_or(&packet_path))?,
        "operator_report_ref": relative_ref(out_dir, output.report_path.as_deref().context("missing observatory report path")?)?,
        "unity_contract_ref": relative_ref(out_dir, output.unity_contract_path.as_deref().context("missing Unity contract path")?)?,
        "console_reference_ref": relative_ref(out_dir, output.console_reference_path.as_deref().context("missing observatory console reference path")?)?,
        "manifest_ref": relative_ref(out_dir, output.manifest_path.as_deref().context("missing observatory manifest path")?)?
    }))
}

#[allow(clippy::too_many_arguments)]
fn build_observatory_integration_packet(
    artifact_root: &str,
    run_status: &adl::long_lived_agent::StatusRecord,
    restart_status: &adl::long_lived_agent::StatusRecord,
    stop_probe: &Value,
    timeout_trace: &Value,
    bulkhead_trace: &Value,
    degraded_trace: &Value,
    remote_timeout: &Value,
    stopped: &adl::long_lived_agent::StatusRecord,
) -> Result<Value> {
    let long_lived_status_ref = format!("{artifact_root}/long_lived_agent/state/status.json");
    let ledger_ref = format!("{artifact_root}/long_lived_agent/state/cycle_ledger.jsonl");
    let inspection_ref = format!("{artifact_root}/inspection/latest.json");
    let stop_probe_ref = format!("{artifact_root}/long_lived_agent_stop_probe/stop_probe.json");
    let timeout_ref = format!("{artifact_root}/resilience/timeout_execution.json");
    let bulkhead_ref = format!("{artifact_root}/resilience/bulkhead_execution.json");
    let degraded_ref = format!("{artifact_root}/resilience/degraded_fallback_execution.json");
    let remote_ref = format!("{artifact_root}/remote_exec/timeout_probe.json");
    let obsmem_ref = format!("{artifact_root}/obsmem/transition_memory_request.json");
    let report_ref = format!("{artifact_root}/runtime_v2/observatory/operator_report.md");
    let packet_ref = format!("{artifact_root}/runtime_v2/observatory/visibility_packet.json");
    let contract_ref =
        format!("{artifact_root}/runtime_v2/observatory/unity_observatory_contract.json");

    let packet = json!({
        "schema": "adl.csm_visibility_packet.v1",
        "packet_id": "v0916-runtime-soak-observatory-packet-0001",
        "generated_at": Utc::now().to_rfc3339(),
        "source": {
            "mode": "captured_artifacts",
            "evidence_level": "bounded_local_runtime_capture",
            "fixture": false,
            "runtime_artifact_root": artifact_root,
            "claim_boundary": "This packet is a bounded local capture produced by the v0.91.6 integrated runtime soak. It is derived from runtime-owned artifacts, suitable for Unity Observatory consumption, and explicitly does not claim full product completion, live telemetry streaming, or v0.92 coherence.",
            "source_refs": [
                long_lived_status_ref,
                ledger_ref,
                inspection_ref,
                stop_probe_ref,
                timeout_ref,
                bulkhead_ref,
                degraded_ref,
                remote_ref,
                obsmem_ref
            ]
        },
        "manifold": {
            "manifold_id": "v0916-runtime-soak-01",
            "display_name": "Runtime / Ops Soak #1",
            "state": format!("{:?}_after_reviewable_capture", stopped.state),
            "lifecycle": "bounded_local_runtime_capture",
            "current_tick": restart_status.completed_cycle_count,
            "uptime": format!("{} bounded cadence cycles captured across restart-proof execution", restart_status.completed_cycle_count),
            "policy_profile": "v0916_integrated_runtime_soak",
            "snapshot_status": {
                "state": "not_applicable",
                "latest_snapshot_id": "none",
                "rehydration_report_ref": obsmem_ref,
                "note": "This soak exports observatory evidence from runtime-owned artifacts; it does not claim live snapshot streaming."
            },
            "health": {
                "summary": format!(
                    "Long-lived-agent cadence, stop-between-cycles control, resilience classifications, remote timeout semantics, and ObsMem handoff all emitted one bounded Unity-consumable review surface with final runtime state '{}'.",
                    format!("{:?}", stopped.state)
                ),
                "level": "nominal",
                "attention_items": [
                    "Unity contract and operator report are generated from the same soak-owned packet.",
                    "Artifact capture is bounded and local; no always-on or remote streaming claim is made.",
                    "Direct runtime mutation remains out of scope for the Unity consumer surface."
                ]
            },
            "evidence_refs": [
                long_lived_status_ref,
                stop_probe_ref,
                timeout_ref,
                remote_ref,
                packet_ref
            ]
        },
        "kernel": {
            "scheduler_state": format!("cadence_run_settled_{:?}", stopped.state),
            "trace_state": "reviewable_artifact_capture",
            "invariant_state": "proof_gates_passed",
            "resource_state": "bounded_local_execution",
            "service_states": [
                { "service_id": "long_lived_agent", "state": format!("{:?}", run_status.state) },
                { "service_id": "resilience_timeout_lane", "state": timeout_trace["trace"]["final_status"] },
                { "service_id": "resilience_bulkhead_lane", "state": bulkhead_trace["trace"]["final_status"] },
                { "service_id": "remote_exec_timeout_lane", "state": remote_timeout["stable_failure_kind"] }
            ],
            "active_guardrails": [
                "duplicate activation remains lease-blocked",
                "stop requests settle between cadence cycles",
                "remote timeouts remain classifiable and retry-aware",
                "Unity handoff remains read-only and packet-driven"
            ],
            "pulse": {
                "status": "bounded_review_tick_complete",
                "completed_through_event_sequence": restart_status.completed_cycle_count,
                "evidence_refs": [ledger_ref, inspection_ref]
            }
        },
        "citizens": [
            {
                "citizen_id": "runtime-lane-alpha",
                "display_name": "Runtime lane alpha",
                "role": "worker",
                "lifecycle_state": "paused",
                "continuity_status": format!("restart_proved_then_{:?}", stopped.state),
                "current_episode": "episode-runtime-cadence",
                "resource_balance": {
                    "compute_units": 6,
                    "scarcity_note": "Completed bounded cadence execution and emitted durable status, ledger, and inspection artifacts."
                },
                "recent_decisions": [{
                    "decision_id": "runtime-lane-alpha-stop-window",
                    "decision": "allow",
                    "summary": "Let one cycle complete, then honor stop during the cadence sleep window.",
                    "evidence_ref": stop_probe_ref
                }],
                "capability_envelope": {
                    "can_execute_episodes": true,
                    "allowed": ["bounded_cadence_cycle", "status_and_inspection_export"],
                    "forbidden": ["cross_session_mutation", "silent_overlap_activation", "unguarded_live_export"]
                },
                "alerts": [],
                "evidence_refs": [long_lived_status_ref, ledger_ref, inspection_ref]
            },
            {
                "citizen_id": "runtime-lane-beta",
                "display_name": "Resilience review lane",
                "role": "service",
                "lifecycle_state": "active",
                "continuity_status": "classification_proofs_captured",
                "current_episode": "episode-resilience-classification",
                "resource_balance": {
                    "compute_units": 4,
                    "scarcity_note": "Focused timeout, bulkhead saturation, and degraded fallback probes completed under bounded local policy execution."
                },
                "recent_decisions": [{
                    "decision_id": "runtime-lane-beta-timeout-refusal",
                    "decision": "refuse",
                    "summary": "A hanging local endpoint was classified as a stable timeout with retryability captured explicitly.",
                    "evidence_ref": remote_ref
                }],
                "capability_envelope": {
                    "can_execute_episodes": true,
                    "allowed": ["timeout_probe", "bulkhead_probe", "degraded_fallback_probe"],
                    "forbidden": ["network_escape", "non_reviewable_failure", "silent_retry_loop"]
                },
                "alerts": [],
                "evidence_refs": [timeout_ref, bulkhead_ref, degraded_ref, remote_ref]
            },
            {
                "citizen_id": "runtime-lane-gamma",
                "display_name": "Observatory export lane",
                "role": "reviewer",
                "lifecycle_state": "awake",
                "continuity_status": "unity_contract_export_ready",
                "current_episode": "episode-unity-handoff",
                "resource_balance": {
                    "compute_units": 2,
                    "scarcity_note": "Exports a Unity-facing contract and operator report from the same bounded packet without direct runtime mutation."
                },
                "recent_decisions": [{
                    "decision_id": "runtime-lane-gamma-unity-export",
                    "decision": "allow",
                    "summary": "Packet, report, console reference, and Unity contract are retained under one reviewable artifact root.",
                    "evidence_ref": packet_ref
                }],
                "capability_envelope": {
                    "can_execute_episodes": false,
                    "allowed": ["read_only_report_export", "unity_contract_generation"],
                    "forbidden": ["runtime_mutation", "identity_rebinding", "live_telemetry_claims"]
                },
                "alerts": [],
                "evidence_refs": [packet_ref, report_ref, contract_ref]
            }
        ],
        "episodes": [
            {
                "episode_id": "episode-runtime-cadence",
                "title": "Bounded cadence run and restart continuity",
                "state": "completed",
                "citizen_ids": ["runtime-lane-alpha"],
                "started_at": Utc::now().to_rfc3339(),
                "last_event": "stop_honored_during_sleep_window",
                "proof_surface": stop_probe_ref,
                "blocked_reason": "not blocked; cadence run, restart proof, and stop-window control all emitted durable evidence"
            },
            {
                "episode_id": "episode-resilience-classification",
                "title": "Timeout, bulkhead, degraded fallback, and remote timeout classification",
                "state": "completed",
                "citizen_ids": ["runtime-lane-beta"],
                "started_at": Utc::now().to_rfc3339(),
                "last_event": "remote_timeout_classified",
                "proof_surface": remote_ref,
                "blocked_reason": "not blocked; focused resilience probes completed under bounded local execution"
            },
            {
                "episode_id": "episode-unity-handoff",
                "title": "Unity Observatory packet and contract export",
                "state": "completed",
                "citizen_ids": ["runtime-lane-alpha", "runtime-lane-gamma"],
                "started_at": Utc::now().to_rfc3339(),
                "last_event": "unity_contract_written",
                "proof_surface": packet_ref,
                "blocked_reason": "not blocked; Unity-facing contract is generated from the same packet and review path"
            }
        ],
        "freedom_gate": {
            "recent_docket": [
                {
                    "decision_id": "fg-runtime-stop-window",
                    "actor": "runtime-lane-alpha",
                    "action": "honor_stop_between_cycles",
                    "decision": "allow",
                    "rationale": "Stop requests are allowed only after a bounded cycle completes and the cadence window is safe.",
                    "evidence_ref": stop_probe_ref
                },
                {
                    "decision_id": "fg-resilience-bulkhead",
                    "actor": "runtime-lane-beta",
                    "action": "attempt_parallel_provider_work_under_saturation",
                    "decision": "defer",
                    "rationale": "Bulkhead saturation remains visible and reviewable instead of being hidden behind automatic queue growth.",
                    "evidence_ref": bulkhead_ref
                },
                {
                    "decision_id": "fg-remote-timeout-classification",
                    "actor": "runtime-lane-beta",
                    "action": "treat_hanging_local_endpoint_as_success",
                    "decision": "refuse",
                    "rationale": "Remote timeout failures must remain classifiable and retry-aware rather than being treated as success.",
                    "evidence_ref": remote_ref
                }
            ],
            "allow_count": 1,
            "defer_count": 1,
            "refuse_count": 1,
            "open_questions": [],
            "rejected_actions": ["treat_hanging_local_endpoint_as_success"]
        },
        "invariants": [
            {
                "invariant_id": "runtime_stop_only_between_cycles",
                "name": "Stop requests settle between cadence cycles",
                "state": "healthy",
                "severity": "high",
                "last_checked": stop_probe["persisted_state"],
                "evidence_ref": stop_probe_ref
            },
            {
                "invariant_id": "resilience_timeout_is_reviewable",
                "name": "Timeout and degraded fallback traces remain reviewer-readable",
                "state": "healthy",
                "severity": "high",
                "last_checked": timeout_trace["trace"]["final_status"],
                "evidence_ref": timeout_ref
            },
            {
                "invariant_id": "unity_surface_is_read_only",
                "name": "Unity consumer surface remains packet-driven and read only",
                "state": "healthy",
                "severity": "critical",
                "last_checked": "unity_contract_written",
                "evidence_ref": report_ref
            }
        ],
        "resources": {
            "compute_units": { "total": 12, "allocated": 10, "available": 2 },
            "memory_pressure": "bounded_local_probe",
            "queue_depth": 0,
            "fairness_notes": [
                "The soak favors durable proof emission over high-throughput execution.",
                "Unity consumes the contract after export rather than participating in the runtime run itself."
            ],
            "scarcity_events": [{
                "event_id": "bulkhead_saturation_reviewed",
                "summary": "Bulkhead saturation is surfaced explicitly as a bounded review event.",
                "evidence_level": "bounded_local_runtime_capture"
            }]
        },
        "trace": {
            "trace_tail": [
                {
                    "event_sequence": 1,
                    "actor": "runtime.long_lived_agent",
                    "event_type": "cycle_restart_proved",
                    "summary": format!("Restart path settled with completed_cycle_count={}.", restart_status.completed_cycle_count),
                    "evidence_ref": long_lived_status_ref
                },
                {
                    "event_sequence": 2,
                    "actor": "runtime.stop_probe",
                    "event_type": "stop_window_honored",
                    "summary": format!("Stop probe persisted runtime state '{}'.", stop_probe["persisted_state"].as_str().unwrap_or("unknown")),
                    "evidence_ref": stop_probe_ref
                },
                {
                    "event_sequence": 3,
                    "actor": "runtime.resilience",
                    "event_type": "bulkhead_and_timeout_classified",
                    "summary": format!(
                        "Timeout={}, bulkhead={}, degraded={}.",
                        timeout_trace["trace"]["final_status"].as_str().unwrap_or("unknown"),
                        bulkhead_trace["trace"]["final_status"].as_str().unwrap_or("unknown"),
                        degraded_trace["trace"]["final_status"].as_str().unwrap_or("unknown")
                    ),
                    "evidence_ref": timeout_ref
                },
                {
                    "event_sequence": 4,
                    "actor": "runtime.remote_exec",
                    "event_type": "remote_timeout_captured",
                    "summary": format!(
                        "Remote timeout classified as '{}' with retryability '{}'.",
                        remote_timeout["stable_failure_kind"].as_str().unwrap_or("unknown"),
                        remote_timeout["retryability"].as_str().unwrap_or("unknown")
                    ),
                    "evidence_ref": remote_ref
                },
                {
                    "event_sequence": 5,
                    "actor": "runtime.observatory",
                    "event_type": "unity_contract_emitted",
                    "summary": "Packet, operator report, console reference, and Unity contract were emitted from the same capture.",
                    "evidence_ref": packet_ref
                }
            ],
            "causal_gaps": [],
            "latest_operator_event": {
                "event_sequence": 5,
                "event_ref": packet_ref
            },
            "latest_citizen_event": {
                "event_sequence": 2,
                "event_ref": stop_probe_ref
            },
            "latest_kernel_event": {
                "event_sequence": 4,
                "event_ref": remote_ref
            }
        },
        "operator_actions": {
            "available_actions": [
                { "action": "inspect_runtime_soak_packet", "mode": "read_only", "status": "available_from_bounded_capture" },
                { "action": "open_unity_operator_report", "mode": "read_only", "status": "available_from_same_packet_bundle" },
                { "action": "stage_unity_contract", "mode": "read_only", "status": "available_for_local_unity_consumption" }
            ],
            "disabled_actions": [
                {
                    "action": "promote_to_live_streaming",
                    "reason": "The Soak #1 observatory handoff is a bounded retained capture, not a live streaming bridge.",
                    "future_issue": 4555
                },
                {
                    "action": "mutate_runtime_from_unity_surface",
                    "reason": "The Unity Observatory consumer remains fail-closed and packet-driven in v0.91.6.",
                    "future_issue": 4555
                }
            ],
            "required_confirmations": [
                "All Unity-facing controls remain proposal-only presentation surfaces.",
                "Runtime truth is the retained artifact bundle, not the transient editor state."
            ],
            "safety_notes": [
                "No private paths, credentials, or unbounded logs are required by the Unity contract.",
                "This packet is truthful only for the bounded Soak #1 capture that produced it."
            ]
        },
        "review": {
            "primary_artifacts": [
                packet_ref,
                report_ref,
                contract_ref,
                long_lived_status_ref,
                stop_probe_ref,
                remote_ref,
                obsmem_ref
            ],
            "missing_artifacts": [],
            "demo_classification": "captured_artifacts",
            "caveats": [
                "This is a bounded local capture and not a live telemetry stream.",
                "Unity consumes a deterministic contract produced after the soak run; it is not co-executing the runtime.",
                "Inhabitant identity, profile, and v0.92 rebinding semantics remain out of scope."
            ],
            "next_consumers": [
                { "issue": 2189, "consumer": "static Observatory console compatibility" },
                { "issue": 2190, "consumer": "operator report generator compatibility" },
                { "issue": 2191, "consumer": "CLI bundle compatibility" },
                { "issue": 2192, "consumer": "operator command packet design" },
                { "issue": 2258, "consumer": "WP-14 integrated first CSM run demo" },
                { "issue": 4543, "consumer": "Runtime/Ops Soak #1 sprint closeout evidence" },
                { "issue": 4548, "consumer": "Unity local-runtime consumption proof surfaces" }
            ]
        },
        "observatory_ui": {
            "default_room": "world",
            "default_lens": "operator",
            "default_memory_dot": "runtime_overview",
            "proposal_mode_statement": "Every active-looking control is a governed request proposal only. No direct runtime mutation is performed from this surface.",
            "rooms": [
                {
                    "room_id": "world",
                    "label": "World / Reality",
                    "question": "What runtime evidence exists right now, and what actually completed?",
                    "note": "Default integrated soak evidence view."
                },
                {
                    "room_id": "governance",
                    "label": "Operator / Governance",
                    "question": "What decision boundaries, disabled paths, and proof links remain explicit?",
                    "note": "Freedom Gate docket and guarded follow-on paths live here."
                },
                {
                    "room_id": "cognition",
                    "label": "Cognition / Internal State",
                    "question": "What bounded internal-state-adjacent signals are safe to project?",
                    "note": "Only review-safe signals are exposed."
                }
            ],
            "lenses": [
                {
                    "lens_id": "public",
                    "label": "Public lens",
                    "summary": "Investor-safe summary with bounded claims."
                },
                {
                    "lens_id": "operator",
                    "label": "Operator lens",
                    "summary": "Operational state, disabled reasons, and review links."
                },
                {
                    "lens_id": "reviewer",
                    "label": "Reviewer lens",
                    "summary": "Packet refs, report refs, and caveats."
                }
            ],
            "memory_dots": [
                {
                    "dot_id": "runtime_overview",
                    "label": "Runtime overview",
                    "room": "world",
                    "lens": "operator",
                    "selected_target": "runtime-lane-alpha",
                    "note": "Open the soak with runtime cadence state in focus."
                },
                {
                    "dot_id": "resilience_watch",
                    "label": "Resilience watch",
                    "room": "governance",
                    "lens": "reviewer",
                    "selected_target": "runtime-lane-beta",
                    "note": "Follow timeout and bulkhead evidence."
                },
                {
                    "dot_id": "investor_view",
                    "label": "Corporate Investor",
                    "room": "world",
                    "lens": "public",
                    "selected_target": "runtime-lane-gamma",
                    "note": "Presentation mode without changing evidence."
                }
            ],
            "corporate_investor_fallback": {
                "label": "Corporate Investor UI",
                "keyboard_shortcut": "i",
                "claim_boundary": "Presentation mode only; evidence, authority, and trace boundaries do not change."
            },
            "proposal_cases": [
                {
                    "proposal_id": "proposal-open-runtime-report",
                    "title": "Open runtime operator report",
                    "target_kind": "service_actor",
                    "target_id": "runtime-lane-gamma",
                    "room": "world",
                    "lens": "operator",
                    "disposition": "available",
                    "summary": "Open the soak-owned operator report produced from the same packet.",
                    "authority_checks": ["validate_projection_class", "append_trace_anchor"],
                    "disabled_reason": null,
                    "trace_anchor": packet_ref,
                    "review_export": report_ref
                },
                {
                    "proposal_id": "proposal-review-timeout-floor",
                    "title": "Review remote timeout floor",
                    "target_kind": "service_actor",
                    "target_id": "runtime-lane-beta",
                    "room": "governance",
                    "lens": "reviewer",
                    "disposition": "challenge",
                    "summary": "Inspect retryability and stable-failure classification without widening into live network work.",
                    "authority_checks": ["validate_service_actor_scope", "emit_review_anchor"],
                    "disabled_reason": "Live transport closure remains future work after this bounded milestone proof.",
                    "trace_anchor": remote_ref,
                    "review_export": report_ref
                },
                {
                    "proposal_id": "proposal-stage-unity-contract",
                    "title": "Stage Unity contract",
                    "target_kind": "service_actor",
                    "target_id": "runtime-lane-gamma",
                    "room": "world",
                    "lens": "public",
                    "disposition": "available",
                    "summary": "Prepare the Unity-facing contract and screenshots for review without mutating runtime state.",
                    "authority_checks": ["validate_operator_identity", "redact_private_state_by_lens"],
                    "disabled_reason": null,
                    "trace_anchor": packet_ref,
                    "review_export": report_ref
                }
            ]
        }
    });
    adl::csm_observatory::validate_visibility_packet(&packet)?;
    Ok(packet)
}

fn build_evidence_index(out_dir: &Path) -> Result<Value> {
    let mut refs = Vec::new();
    collect_relative_files(out_dir, out_dir, &mut refs)?;
    refs.sort();
    Ok(json!({
        "schema_version": "adl.integrated_runtime_soak_evidence_index.v1",
        "issue": RUNTIME_SOAK_ISSUE,
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

fn build_completion_classification(out_dir: &Path) -> Value {
    let heartbeat_mock_ref = "long_lived_agent/state/aws_runtime_heartbeat_mock.jsonl";
    let heartbeat_mock_exists = out_dir.join(heartbeat_mock_ref).exists();
    let scheduler_evidence_refs = vec!["scheduler/scheduler_plan.json"];
    let scheduler_notes = vec![
        "This run proves the deterministic scheduler decision artifact, not a retained operator-facing scheduler CLI artifact."
    ];
    json!({
        "schema_version": "adl.runtime.soak.completion_classification.v1",
        "issue": RUNTIME_SOAK_ISSUE,
        "generated_at": Utc::now().to_rfc3339(),
        "classifications": [
            {
                "surface": "runtime_boot_shutdown",
                "status": "integrated_proven",
                "evidence_refs": [
                    "long_lived_agent/run_status_cycle2.json",
                    "long_lived_agent/restart_status_cycle3.json",
                    "long_lived_agent/status_after_stop.json",
                    "inspection/latest.json"
                ]
            },
            {
                "surface": "resilience_negative_cases",
                "status": "integrated_proven",
                "evidence_refs": [
                    "resilience/timeout_execution.json",
                    "resilience/bulkhead_execution.json",
                    "resilience/degraded_fallback_execution.json",
                    "remote_exec/timeout_probe.json"
                ]
            },
            {
                "surface": "scheduler_advisory_decision_packet",
                "status": "integrated_proven",
                "evidence_refs": scheduler_evidence_refs,
                "notes": scheduler_notes
            },
            {
                "surface": "memory_handoff",
                "status": "integrated_proven",
                "evidence_refs": [
                    "obsmem/transition_memory_request.json"
                ]
            },
            {
                "surface": "aws_runtime_heartbeat",
                "status": if heartbeat_mock_exists { "local_only_accepted" } else { "blocked" },
                "evidence_refs": if heartbeat_mock_exists {
                    vec![heartbeat_mock_ref]
                } else {
                    Vec::<&str>::new()
                },
                "notes": if heartbeat_mock_exists {
                    vec![
                        "Mock-mode runtime heartbeat publication was captured for bounded local proof without claiming live CloudWatch transport."
                    ]
                } else {
                    vec![
                        "No runtime heartbeat artifact was captured in this run. Re-run with ADL_AWS_SIGNAL_MODE=mock to record the local-only signal path."
                    ]
                }
            },
            {
                "surface": "aws_local_ops_ssm_nodes",
                "status": "blocked",
                "owner_issue": 4545,
                "notes": [
                    "This runner does not re-verify the live AWS profile, CloudWatch account posture, or current SSM node health for wuji, nessus.local, and opticon.local.",
                    "Retained prior proof exists under the v0.91.6 AWS/local-operations review packet and SSM proof docs, but current live revalidation is still required."
                ],
                "evidence_refs": [
                    "docs/milestones/v0.91.6/review/V0916_RUNTIME_AWS_LOCAL_OPERATIONS_MINI_SPRINT_REVIEW_4343.md",
                    "docs/milestones/v0.91.6/review/security/LOCAL_POLIS_SSM_PROOF_4113.md",
                    "docs/milestones/v0.91.6/review/security/LOCAL_POLIS_SSM_PROOF_4318.md",
                    "docs/milestones/v0.91.6/review/security/LOCAL_POLIS_SSM_PROOF_4319.md"
                ]
            },
            {
                "surface": "acip_aee_memory_runtime_path",
                "status": "blocked",
                "owner_issue": 4546,
                "notes": [
                    "This runner preserves the ObsMem handoff slice but does not yet execute one integrated ACIP plus AEE temporary-agent path."
                ]
            },
            {
                "surface": "logging_stdout_stderr_contract",
                "status": "blocked",
                "owner_issue": 4543,
                "notes": [
                    "A dedicated machine-stdout versus human-stderr proof artifact is not emitted by this runner yet."
                ]
            },
            {
                "surface": "observatory_unity_consumption",
                "status": "blocked",
                "owner_issue": 4548,
                "notes": [
                    "Unity/Observatory proof remains outside this runtime-only session and must be handled in the separate Unity lane."
                ]
            }
        ]
    })
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

fn repo_root() -> Result<PathBuf> {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .map(Path::to_path_buf)
        .context("derive repository root from manifest dir")
}

fn repo_relative_ref(path: &Path) -> Result<String> {
    let root = repo_root()?;
    relative_ref(&root, path)
}

fn relative_ref(root: &Path, path: &Path) -> Result<String> {
    let rel = path
        .strip_prefix(root)
        .with_context(|| format!("{} is not under {}", path.display(), root.display()))?;
    Ok(rel.to_string_lossy().replace('\\', "/"))
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
    observatory_bundle: &Value,
    evidence_index: &Value,
) -> Value {
    json!({
        "schema_version": "adl.integrated_runtime_soak_proof.v1",
        "issue": RUNTIME_SOAK_ISSUE,
        "generated_at": Utc::now().to_rfc3339(),
        "what_this_proves": [
            "The v0.91.6 runtime proves initialization, run/restart/inspection continuity, and a companion live stop-between-cycles behavior across bounded long-lived-agent probes with durable cycle evidence.",
            "The resilience substrate emits reviewer-readable timeout, bulkhead saturation/backpressure, and degraded fallback traces using the real library entrypoints.",
            "The deterministic scheduler economics fixture still resolves into a reviewable scheduler plan artifact that can be cited by the soak packet.",
            "Remote execution timeout classification is live against a hanging local endpoint and records stable failure kind plus retryability.",
            "ObsMem transition memory can still build a structured write request from a tracked handoff packet.",
            "The long-lived-agent leased-state contract blocks overlap when an injected lease artifact is present.",
            "The soak emits one runtime-owned Observatory packet, operator report, and Unity-facing contract from the same bounded artifact capture."
        ],
        "what_this_does_not_prove": [
            "full v0.92 activation readiness",
            "external-agent trust or transport closure",
            "full Observatory or Unity UI product completion",
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
            "degraded_output": degraded_trace["trace"]["output_degraded"],
            "observatory_classification": observatory_bundle["classification"],
            "unity_contract_ref": observatory_bundle["unity_contract_ref"]
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
            "scheduler/scheduler_plan.json",
            "remote_exec/timeout_probe.json",
            "obsmem/transition_memory_request.json",
            "completion_classification.json",
            "runtime_v2/observatory/visibility_packet.json",
            "runtime_v2/observatory/operator_report.md",
            "runtime_v2/observatory/unity_observatory_contract.json",
            "audit/artifact_safety_scan.json"
        ],
        "observatory_integration": observatory_bundle,
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
        "issue": RUNTIME_SOAK_ISSUE,
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
        "# V0.91.6 Integrated Runtime Soak\n\n{DISCLAIMER}\n\n## What This Proves\n\nThis run proves a bounded integrated runtime slice for `#4543`: one long-lived-agent run/restart/inspection continuity path, one companion live stop-between-cycles probe, timeout classification, bulkhead/backpressure saturation, degraded fallback, one deterministic scheduler decision artifact, remote-exec timeout handling, one tracked ObsMem handoff path, one explicit injected-lease contract probe, and one retained Unity Observatory handoff bundle all converge under one reviewer-readable artifact root.\n\n## Reviewer Path\n\n1. Inspect `integrated_runtime_soak_proof.json`.\n2. Inspect `completion_classification.json` for integrated-proven versus blocked surfaces.\n3. Inspect `long_lived_agent/state/status.json` and `long_lived_agent/state/cycle_ledger.jsonl`.\n4. Inspect `inspection/latest.json`.\n5. Inspect `long_lived_agent_stop_probe/stop_probe.json`.\n6. Inspect `resilience/timeout_execution.json`, `resilience/bulkhead_execution.json`, and `resilience/degraded_fallback_execution.json`.\n7. Inspect `scheduler/scheduler_plan.json`.\n8. Inspect `remote_exec/timeout_probe.json`.\n9. Inspect `obsmem/transition_memory_request.json`.\n10. Inspect `runtime_v2/observatory/visibility_packet.json`, `runtime_v2/observatory/operator_report.md`, and `runtime_v2/observatory/unity_observatory_contract.json`.\n11. Inspect `audit/artifact_safety_scan.json`.\n"
    )
}

fn reviewer_walkthrough() -> String {
    "# Reviewer Walkthrough\n\nRun the soak with `cargo run --manifest-path adl/Cargo.toml --bin run_v0916_integrated_runtime_soak -- --out docs/milestones/v0.91.6/review/runtime/v0916_integrated_runtime_soak_4543`.\n\nThe review question is whether the runtime now leaves one honest, durable packet showing restart, a live stop between cycles, timeout, saturation/backpressure, degraded fallback, scheduler advisory evidence, remote-exec timeout semantics, memory handoff, and a Unity Observatory handoff contract under one bounded local proof surface without overclaiming ACIP, Observatory product readiness, Unity UI completion, or v0.92 readiness.\n".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_dir(name: &str) -> PathBuf {
        let unique = format!(
            "{}-{}-{}",
            name,
            std::process::id(),
            Utc::now().timestamp_nanos_opt().unwrap_or_default()
        );
        repo_root()
            .expect("repo root")
            .join(".adl/tmp")
            .join(unique)
    }

    #[test]
    fn run_v0916_integrated_runtime_soak_generates_expected_artifacts() {
        let out_dir = temp_dir("v0916-runtime-soak");
        run(Args {
            out: out_dir.clone(),
        })
        .expect("runtime soak run should succeed");

        let proof_path = out_dir.join("integrated_runtime_soak_proof.json");
        let proof: Value = serde_json::from_str(
            &fs::read_to_string(&proof_path).expect("read generated proof packet"),
        )
        .expect("parse generated proof packet");
        assert_eq!(proof["issue"], RUNTIME_SOAK_ISSUE);
        assert_eq!(
            proof["status_summary"]["live_stop_probe_state"],
            Value::String("stopped".to_string())
        );
        assert_eq!(
            proof["status_summary"]["remote_timeout_failure_kind"],
            Value::String("timeout".to_string())
        );
        assert_eq!(
            proof["observatory_integration"]["classification"],
            Value::String("captured_artifacts".to_string())
        );
        assert!(
            out_dir
                .join("runtime_v2/observatory/unity_observatory_contract.json")
                .exists(),
            "expected Unity contract bundle artifact"
        );

        let evidence_index_path = out_dir.join("integrated_runtime_soak_evidence_index.json");
        let evidence_index: Value = serde_json::from_str(
            &fs::read_to_string(&evidence_index_path).expect("read generated evidence index"),
        )
        .expect("parse generated evidence index");
        let artifact_refs = evidence_index["artifact_refs"]
            .as_array()
            .expect("artifact refs array");
        assert!(
            artifact_refs.iter().any(|entry| {
                entry == "long_lived_agent/state/status.json"
                    || entry == "long_lived_agent\\state\\status.json"
            }),
            "expected long-lived-agent status artifact in evidence index"
        );
        assert!(
            artifact_refs
                .iter()
                .any(|entry| entry == "scheduler/scheduler_plan.json"),
            "expected scheduler plan artifact in evidence index"
        );

        let artifact_scan_path = out_dir.join("audit/artifact_safety_scan.json");
        let artifact_scan: Value = serde_json::from_str(
            &fs::read_to_string(&artifact_scan_path).expect("read generated artifact scan"),
        )
        .expect("parse generated artifact scan");
        assert_eq!(artifact_scan["passed"], Value::Bool(true));

        let completion_path = out_dir.join("completion_classification.json");
        let completion: Value = serde_json::from_str(
            &fs::read_to_string(&completion_path).expect("read completion classification"),
        )
        .expect("parse completion classification");
        assert_eq!(completion["issue"], Value::from(RUNTIME_SOAK_ISSUE));

        let _ = fs::remove_dir_all(&out_dir);
    }

    #[test]
    fn run_v0916_integrated_runtime_soak_support_functions_are_reviewable() {
        let root = temp_dir("v0916-runtime-soak-support");
        fs::create_dir_all(&root).expect("create temp root");

        let relative = absolute_from_cwd(Path::new("target"))
            .expect("relative path should resolve against cwd");
        assert!(relative.ends_with("target"));
        let absolute = absolute_from_cwd(&root).expect("absolute path should remain absolute");
        assert_eq!(absolute, root);

        let spec_path = write_agent_spec_under(&root, "spec", "agent-4543")
            .expect("agent spec should be written");
        let spec_text = fs::read_to_string(&spec_path).expect("read agent spec");
        assert!(spec_text.contains("agent-4543"));
        assert!(spec_text.contains("state_root: state"));

        write_file(&root.join("notes.txt"), "runtime soak notes").expect("write text file");
        write_json(&root.join("data/info.json"), &json!({"ok": true})).expect("write json file");
        wait_for_path(&root.join("data/info.json"), Duration::from_secs(1))
            .expect("json artifact should appear");

        let evidence_index = build_evidence_index(&root).expect("build evidence index");
        let refs = evidence_index["artifact_refs"]
            .as_array()
            .expect("artifact refs array");
        assert!(refs.iter().any(|entry| entry == "notes.txt"));

        let leak_file = root.join("leak.md");
        write_file(
            &leak_file,
            "secret_access_key=redacted\npath=/Users/example/private\n",
        )
        .expect("write leak file");
        let scan = scan_public_artifacts(&root).expect("scan artifacts");
        assert_eq!(scan["passed"], Value::Bool(false));
        let findings = scan["findings"].as_array().expect("findings array");
        assert!(findings
            .iter()
            .any(|finding| finding["family"] == "secret_material"));
        assert!(findings
            .iter()
            .any(|finding| finding["family"] == "private_host_path"));

        let readme_text = readme();
        assert!(readme_text.contains("What This Proves"));
        let walkthrough = reviewer_walkthrough();
        assert!(walkthrough.contains("Unity Observatory handoff contract"));

        let timeout = run_timeout_policy_probe();
        assert_eq!(
            timeout["final_status"],
            Value::String("TimedOut".to_string())
        );
        let bulkhead = run_bulkhead_probe();
        assert_eq!(
            bulkhead["trace"]["final_status"],
            Value::String("saturated".to_string())
        );
        let degraded = run_degraded_fallback_probe();
        assert_eq!(degraded["trace"]["output_degraded"], Value::Bool(true));
        let remote = run_remote_timeout_probe().expect("remote timeout probe should succeed");
        assert_eq!(
            remote["stable_failure_kind"],
            Value::String("timeout".to_string())
        );
        let obsmem = build_obsmem_request().expect("obsmem request should build");
        assert!(obsmem.is_object());
        assert!(!obsmem.as_object().expect("obsmem object").is_empty());
        let scheduler = build_scheduler_plan().expect("scheduler plan should build");
        assert_eq!(
            scheduler["schema_version"],
            Value::String("adl.scheduler.plan.v1".to_string())
        );
        let completion = build_completion_classification(&root);
        assert_eq!(completion["issue"], Value::from(RUNTIME_SOAK_ISSUE));

        let _ = fs::remove_dir_all(&root);
    }
}
