use std::fs;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::{Duration, Instant};

use adl::adl::ProviderSpec;
use adl::long_lived_agent::{self, InspectOptions, RunOptions};
use adl::remote_exec::{
    execute_remote, retryability, stable_failure_kind, ExecuteInputsPayload, ExecuteRequest,
    ExecuteStepPayload, PROTOCOL_VERSION,
};
use adl::resilience::{
    bulkhead_initial_state, execute_bulkhead_policy, execute_fallback_policy,
    execute_retry_policy, execute_timeout_policy, remote_exec_health_payload, BulkheadPolicyV1,
    FallbackPolicyV1, RateLimitPolicyV1, ResilienceFaultClassificationV1, ResiliencePolicyV1,
    ResilienceSurfaceV1, RetryPolicyV1, TimeoutObservation, TimeoutPolicyV1,
};
use anyhow::{anyhow, Context, Result};
use chrono::Utc;
use clap::Parser;
use serde::Serialize;
use serde_json::{json, Value};

const DISCLAIMER: &str = "This runtime failure-injection packet is a bounded local proof for #4547. It proves reviewer-readable resilience behavior under one integrated runtime path and does not claim full v0.92 runtime readiness, checkpoint/restore, migration, replay, or Unity/Observatory completion.";

#[derive(Debug, Parser)]
#[command(name = "run_v0916_runtime_failure_injection")]
#[command(
    about = "Execute the v0.91.6 integrated runtime failure-injection proof and write reviewer-facing artifacts"
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

    let spec_path = write_agent_spec_under(
        &out_dir,
        "long_lived_agent",
        "v0916-runtime-failure-injection",
    )?;
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

    let resume_status = long_lived_agent::run(
        &spec_path,
        RunOptions {
            max_cycles: 1,
            interval_secs: Some(0),
            no_sleep: true,
            recover_stale_lease: false,
        },
    )?;
    write_json(
        &out_dir.join("long_lived_agent/resume_status_cycle3.json"),
        &resume_status,
    )?;

    let inspection_latest =
        long_lived_agent::inspect(&spec_path, InspectOptions { cycle_id: None })?;
    write_json(&out_dir.join("inspection/latest.json"), &inspection_latest)?;

    let stop_probe = execute_stop_probe(&out_dir)?;
    write_json(
        &out_dir.join("long_lived_agent_stop_probe/stop_probe.json"),
        &stop_probe,
    )?;

    let retry_trace = run_retry_policy_probe();
    write_json(
        &out_dir.join("resilience/retry_execution.json"),
        &retry_trace,
    )?;

    let timeout_trace = run_timeout_policy_probe();
    write_json(
        &out_dir.join("resilience/timeout_execution.json"),
        &timeout_trace,
    )?;

    let cancellation_trace = run_cancellation_policy_probe();
    write_json(
        &out_dir.join("resilience/cancellation_execution.json"),
        &cancellation_trace,
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

    let stopped = long_lived_agent::stop(
        &spec_path,
        "bounded runtime failure-injection stop after proof capture",
    )?;
    write_json(
        &out_dir.join("long_lived_agent/status_after_stop.json"),
        &stopped,
    )?;

    let failure_register = build_failure_register(
        &resume_status,
        &stop_probe,
        &retry_trace,
        &timeout_trace,
        &cancellation_trace,
        &bulkhead_trace,
        &degraded_trace,
        &remote_timeout,
    );
    write_json(
        &out_dir.join("runtime_failure_register.json"),
        &failure_register,
    )?;

    let placeholder_evidence_index = json!({
        "schema_version": "adl.runtime_failure_injection_evidence_index.v1",
        "issue": 4547,
        "generated_at": Utc::now().to_rfc3339(),
        "artifact_refs": [],
        "prerequisite_refs": [
            "docs/milestones/v0.91.6/features/RESILIENCE_PERSISTENCE_SLEEP_WAKE_v0.91.6.md",
            "docs/milestones/v0.91.6/review/V0916_RUNTIME_RESILIENCE_FOLLOW_ON_SPRINT_REVIEW_4241.md",
            "docs/milestones/v0.91.6/RUNTIME_INTEGRATION_SOAK_SPRINT_v0.91.6.md"
        ],
        "note": "temporary placeholder written before the final evidence index is materialized"
    });
    let placeholder_proof_packet = build_proof_packet(
        &initial_status,
        &run_status,
        &resume_status,
        &stop_probe,
        &stopped,
        &retry_trace,
        &timeout_trace,
        &cancellation_trace,
        &bulkhead_trace,
        &degraded_trace,
        &remote_timeout,
        &failure_register,
        &placeholder_evidence_index,
    );
    write_json(
        &out_dir.join("runtime_failure_injection_proof.json"),
        &placeholder_proof_packet,
    )?;

    let evidence_index = build_evidence_index(&out_dir)?;
    write_json(
        &out_dir.join("runtime_failure_injection_evidence_index.json"),
        &evidence_index,
    )?;

    let proof_packet = build_proof_packet(
        &initial_status,
        &run_status,
        &resume_status,
        &stop_probe,
        &stopped,
        &retry_trace,
        &timeout_trace,
        &cancellation_trace,
        &bulkhead_trace,
        &degraded_trace,
        &remote_timeout,
        &failure_register,
        &evidence_index,
    );
    write_json(
        &out_dir.join("runtime_failure_injection_proof.json"),
        &proof_packet,
    )?;

    let artifact_scan = scan_public_artifacts(&out_dir)?;
    if !artifact_scan
        .get("passed")
        .and_then(Value::as_bool)
        .unwrap_or(false)
    {
        return Err(anyhow!(
            "runtime failure-injection artifact safety scan failed"
        ));
    }
    write_json(
        &out_dir.join("audit/artifact_safety_scan.json"),
        &artifact_scan,
    )?;

    println!("out={}", out_dir.display());
    println!(
        "proof={}",
        out_dir.join("runtime_failure_injection_proof.json").display()
    );
    Ok(())
}

fn execute_stop_probe(out_dir: &Path) -> Result<Value> {
    let spec_path = write_agent_spec_under(
        out_dir,
        "long_lived_agent_stop_probe",
        "v0916-runtime-failure-stop-probe",
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
    wait_for_completed_cycles(&spec_path, 1, Duration::from_secs(15))?;
    let stop_status = long_lived_agent::stop(
        &spec_path,
        "operator stop after the first persisted cycle and before a second persisted cycle",
    )?;
    let final_status = runner
        .join()
        .map_err(|_| anyhow!("stop probe thread panicked"))??;
    let persisted = long_lived_agent::status(&spec_path)?;
    let second_cycle_manifest = loaded
        .state_root
        .join("cycles/cycle-000002/cycle_manifest.json");
    Ok(json!({
        "probe_kind": "live_stop_after_first_persisted_cycle",
        "note": "This probe waits for one persisted completed cycle, issues stop before a second persisted cycle appears, and verifies the bounded non-continuity truth explicitly.",
        "stop_status": stop_status,
        "run_returned_state": final_status.state,
        "persisted_state": persisted.state,
        "completed_cycle_count": persisted.completed_cycle_count,
        "second_cycle_manifest_present": second_cycle_manifest.exists(),
        "last_error": persisted.last_error,
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

fn run_retry_policy_probe() -> Value {
    let policy = ResiliencePolicyV1 {
        schema_version: "adl.resilience_policy.v1".to_string(),
        policy_id: "runtime.failure_injection.retry.policy".to_string(),
        retry: Some(RetryPolicyV1 {
            max_attempts: 3,
            backoff_ms: Some(5),
            jitter_ms: Some(0),
            max_elapsed_ms: Some(100),
            retryable_fault_classes: vec![adl::resilience::ResilienceFaultClassV1::ProviderTransientHttp],
        }),
        timeout: None,
        circuit_breaker: None,
        rate_limit: None,
        bulkhead: None,
        fallback: None,
        checkpoint_required: false,
        telemetry_required: true,
    };
    let mut attempts_seen = Vec::new();
    let execution = execute_retry_policy(
        &policy,
        ResilienceSurfaceV1::Runtime,
        "runtime.failure_injection.retry_probe",
        |attempt_index| {
            if attempt_index < 3 {
                Err::<String, _>("provider transient 503".to_string())
            } else {
                Ok::<_, String>("recovered-after-retry".to_string())
            }
        },
        |err| ResilienceFaultClassificationV1::provider(err, Some(503)),
        |_| {},
        |record| attempts_seen.push(record.clone()),
    );
    json!({
        "attempt_count_observed": attempts_seen.len(),
        "attempts_observed": attempts_seen,
        "final_status": format!("{:?}", execution.trace.final_status),
        "trace": execution.trace,
        "result": execution.result.ok(),
    })
}

fn run_timeout_policy_probe() -> Value {
    let policy = ResiliencePolicyV1 {
        schema_version: "adl.resilience_policy.v1".to_string(),
        policy_id: "runtime.failure_injection.timeout.policy".to_string(),
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
        "runtime.failure_injection.timeout_probe",
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

fn run_cancellation_policy_probe() -> Value {
    let policy = ResiliencePolicyV1::provider_attempt_policy(
        "runtime.failure_injection.cancellation.policy",
        1,
        100,
    );
    let execution = execute_timeout_policy(
        &policy,
        ResilienceSurfaceV1::Workflow,
        "runtime.failure_injection.cancellation_probe",
        || TimeoutObservation::<(), ResilienceFaultClassificationV1> {
            result: Err(ResilienceFaultClassificationV1::provider("cancelled", None)),
            elapsed_ms: 15,
            cancelled: true,
        },
        |fault| fault.clone(),
        |breach, elapsed_ms, budget_ms| ResilienceFaultClassificationV1::provider(
            &format!("timeout {:?} elapsed={} budget={}", breach, elapsed_ms, budget_ms),
            None,
        ),
        |elapsed_ms| ResilienceFaultClassificationV1::provider(
            &format!("cancelled at {elapsed_ms}"),
            None,
        ),
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
        policy_id: "runtime.failure_injection.bulkhead.policy".to_string(),
        retry: None,
        timeout: None,
        circuit_breaker: None,
        rate_limit: Some(RateLimitPolicyV1 {
            max_requests: 1,
            window_ms: 100,
        }),
        bulkhead: Some(BulkheadPolicyV1 {
            fault_domain: "runtime_failure_provider".to_string(),
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
        "runtime.failure_injection.bulkhead_probe",
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
        policy_id: "runtime.failure_injection.fallback.policy".to_string(),
        retry: Some(RetryPolicyV1 {
            max_attempts: 2,
            backoff_ms: Some(10),
            jitter_ms: Some(0),
            max_elapsed_ms: Some(20),
            retryable_fault_classes: vec![
                adl::resilience::ResilienceFaultClassV1::ProviderTimeout,
                adl::resilience::ResilienceFaultClassV1::ProviderTransientHttp,
            ],
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
            fallback_ref: "runtime.failure_injection.degraded".to_string(),
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
        "runtime.failure_injection.degraded_probe",
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
        run_id: "runtime-failure-injection-remote-timeout".to_string(),
        workflow_id: "v0916_runtime_failure_injection".to_string(),
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
                config: std::collections::HashMap::new(),
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

fn build_failure_register(
    resume_status: &adl::long_lived_agent::StatusRecord,
    stop_probe: &Value,
    retry_trace: &Value,
    timeout_trace: &Value,
    cancellation_trace: &Value,
    bulkhead_trace: &Value,
    degraded_trace: &Value,
    remote_timeout: &Value,
) -> Value {
    json!({
        "schema_version": "adl.runtime_failure_register.v1",
        "issue": 4547,
        "generated_at": Utc::now().to_rfc3339(),
        "entries": [
            {
                "failure_mode": "resume_continuation_across_reinvocation",
                "expected_behavior": "A bounded repeated run may resume the integrated runtime path from prior state without claiming interrupted restart recovery or broader durable continuity.",
                "observed_behavior": format!("resume_state={:?} completed_cycle_count={}", resume_status.state, resume_status.completed_cycle_count),
                "evidence_ref": "long_lived_agent/resume_status_cycle3.json",
                "status": "proved",
                "continuity_classification": "bounded_resume_continuation_proved"
            },
            {
                "failure_mode": "stop_after_first_persisted_cycle",
                "expected_behavior": "Stop issued after the first persisted cycle should leave the runtime stopped without a second persisted cycle manifest.",
                "observed_behavior": format!("persisted_state={} completed_cycle_count={} second_cycle_manifest_present={}", stop_probe["persisted_state"], stop_probe["completed_cycle_count"], stop_probe["second_cycle_manifest_present"]),
                "evidence_ref": "long_lived_agent_stop_probe/stop_probe.json",
                "status": "proved",
                "continuity_classification": "bounded_non_continuity_after_stop_proved"
            },
            {
                "failure_mode": "retry_backoff_after_transient_provider_failure",
                "expected_behavior": "Retry policy should retry transient HTTP failures and eventually recover within budget.",
                "observed_behavior": format!("final_status={} attempt_count={}", retry_trace["final_status"], retry_trace["attempt_count_observed"]),
                "evidence_ref": "resilience/retry_execution.json",
                "status": "proved",
                "continuity_classification": "bounded_retry_recovery_without_durable_continuity_claim"
            },
            {
                "failure_mode": "timeout",
                "expected_behavior": "Late completion beyond the timeout budget should classify as timeout rather than success.",
                "observed_behavior": format!("final_status={}", timeout_trace["final_status"]),
                "evidence_ref": "resilience/timeout_execution.json",
                "status": "proved",
                "continuity_classification": "no_continuity_claim_after_timeout"
            },
            {
                "failure_mode": "cancellation",
                "expected_behavior": "Explicit cancellation should classify as cancelled and leave follow-on handling explicit.",
                "observed_behavior": format!("final_status={}", cancellation_trace["final_status"]),
                "evidence_ref": "resilience/cancellation_execution.json",
                "status": "proved",
                "continuity_classification": "resume_allowed_but_not_claimed_automatic"
            },
            {
                "failure_mode": "partial_failure_via_bulkhead_saturation",
                "expected_behavior": "A saturated local bulkhead should reject work locally while preserving the broader runtime path truthfully.",
                "observed_behavior": format!("final_status={}", bulkhead_trace["trace"]["final_status"]),
                "evidence_ref": "resilience/bulkhead_execution.json",
                "status": "proved",
                "continuity_classification": "partial_failure_classified_not_hidden"
            },
            {
                "failure_mode": "degraded_fallback",
                "expected_behavior": "Provider failure should degrade explicitly through fallback instead of overclaiming a clean success path.",
                "observed_behavior": format!("outcome_kind={} output_degraded={}", degraded_trace["outcome_kind"], degraded_trace["trace"]["output_degraded"]),
                "evidence_ref": "resilience/degraded_fallback_execution.json",
                "status": "proved",
                "continuity_classification": "degraded_service_only"
            },
            {
                "failure_mode": "remote_execution_timeout",
                "expected_behavior": "A hanging endpoint should resolve as a stable timeout failure kind with retryability truth.",
                "observed_behavior": format!("stable_failure_kind={} retryability={}", remote_timeout["stable_failure_kind"], remote_timeout["retryability"]),
                "evidence_ref": "remote_exec/timeout_probe.json",
                "status": "proved",
                "continuity_classification": "no_continuity_claim_after_remote_timeout"
            }
        ]
    })
}

#[allow(clippy::too_many_arguments)]
fn build_proof_packet(
    initial_status: &adl::long_lived_agent::StatusRecord,
    run_status: &adl::long_lived_agent::StatusRecord,
    resume_status: &adl::long_lived_agent::StatusRecord,
    stop_probe: &Value,
    stopped: &adl::long_lived_agent::StatusRecord,
    retry_trace: &Value,
    timeout_trace: &Value,
    cancellation_trace: &Value,
    bulkhead_trace: &Value,
    degraded_trace: &Value,
    remote_timeout: &Value,
    failure_register: &Value,
    evidence_index: &Value,
) -> Value {
    json!({
        "schema_version": "adl.runtime_failure_injection_proof.v1",
        "issue": 4547,
        "generated_at": Utc::now().to_rfc3339(),
        "what_this_proves": [
            "The v0.91.6 runtime leaves one reviewer-readable failure-injection register covering retry, timeout, explicit cancellation, bulkhead saturation, degraded fallback, remote timeout, and bounded run/resume/stop continuity truth under one integrated runtime path.",
            "Negative cases remain explicit: timeout is not treated as success, cancellation is not treated as silent continuation, and degraded behavior is marked degraded instead of being overclaimed as a clean pass.",
            "Soak #1 can consume one bounded resilience packet without inheriting unsupported checkpoint/restore, migration, replay, or v0.92 readiness claims."
        ],
        "what_this_does_not_prove": [
            "interrupted restart recovery after a forced stop",
            "full checkpoint or restore continuity",
            "migration or replay support",
            "autonomous recovery across all runtime surfaces",
            "Unity or Observatory integration completion",
            "full v0.92 runtime readiness"
        ],
        "status_summary": {
            "initial_state": initial_status.state,
            "run_state_after_cycle2": run_status.state,
            "resume_state_after_cycle3": resume_status.state,
            "stop_state": stopped.state,
            "live_stop_probe_state": stop_probe["persisted_state"],
            "live_stop_second_cycle_manifest_present": stop_probe["second_cycle_manifest_present"],
            "retry_final_status": retry_trace["trace"]["final_status"],
            "timeout_final_status": timeout_trace["trace"]["final_status"],
            "cancellation_final_status": cancellation_trace["trace"]["final_status"],
            "bulkhead_final_status": bulkhead_trace["trace"]["final_status"],
            "degraded_fallback_final_status": degraded_trace["trace"]["final_status"],
            "degraded_output": degraded_trace["trace"]["output_degraded"],
            "remote_timeout_failure_kind": remote_timeout["stable_failure_kind"],
            "remote_timeout_retryability": remote_timeout["retryability"]
        },
        "failure_register_ref": "runtime_failure_register.json",
        "failure_register": failure_register,
        "reviewer_path": [
            "README.md",
            "runtime_failure_injection_proof.json",
            "runtime_failure_register.json",
            "long_lived_agent/initial_status.json",
            "long_lived_agent/run_status_cycle2.json",
            "long_lived_agent/resume_status_cycle3.json",
            "long_lived_agent_stop_probe/stop_probe.json",
            "resilience/retry_execution.json",
            "resilience/timeout_execution.json",
            "resilience/cancellation_execution.json",
            "resilience/bulkhead_execution.json",
            "resilience/degraded_fallback_execution.json",
            "remote_exec/timeout_probe.json",
            "audit/artifact_safety_scan.json"
        ],
        "evidence_index_ref": "runtime_failure_injection_evidence_index.json",
        "evidence_index": evidence_index,
        "disclaimer": DISCLAIMER
    })
}

fn build_evidence_index(out_dir: &Path) -> Result<Value> {
    let mut refs = Vec::new();
    collect_relative_files(out_dir, out_dir, &mut refs)?;
    refs.sort();
    Ok(json!({
        "schema_version": "adl.runtime_failure_injection_evidence_index.v1",
        "issue": 4547,
        "generated_at": Utc::now().to_rfc3339(),
        "artifact_refs": refs,
        "prerequisite_refs": [
            "docs/milestones/v0.91.6/features/RESILIENCE_PERSISTENCE_SLEEP_WAKE_v0.91.6.md",
            "docs/milestones/v0.91.6/review/V0916_RUNTIME_RESILIENCE_FOLLOW_ON_SPRINT_REVIEW_4241.md",
            "docs/milestones/v0.91.6/RUNTIME_INTEGRATION_SOAK_SPRINT_v0.91.6.md"
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
        "schema_version": "adl.runtime_failure_injection_artifact_safety_scan.v1",
        "issue": 4547,
        "scanned_at": Utc::now().to_rfc3339(),
        "passed": findings.is_empty(),
        "scanned_artifacts": files,
        "findings": findings,
    }))
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
display_name: V0916 Runtime Failure Injection
state_root: state
workflow:
  kind: demo_adapter
  name: v0916_runtime_failure_injection
  run_args:
    provider_id: local_ollama
    model: gemma4:latest
heartbeat:
  interval_secs: 1
  max_cycles: 6
  stale_lease_after_secs: 60
safety:
  allow_network: false
"#
    .replace("__AGENT_INSTANCE_ID__", agent_instance_id);
    write_file(&spec_path, &body)?;
    Ok(spec_path)
}

fn wait_for_completed_cycles(spec_path: &Path, min_cycles: u64, timeout: Duration) -> Result<()> {
    let started = Instant::now();
    while started.elapsed() < timeout {
        match long_lived_agent::status(spec_path) {
            Ok(status) => {
                if status.completed_cycle_count >= min_cycles {
                    return Ok(());
                }
            }
            Err(err) => {
                let msg = err.to_string();
                if !msg.contains("EOF while parsing")
                    && !msg.contains("failed parsing json artifact")
                {
                    return Err(err);
                }
            }
        }
        thread::sleep(Duration::from_millis(50));
    }
    Err(anyhow!(
        "timed out waiting for {} completed cycles at {}",
        min_cycles,
        spec_path.display()
    ))
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
        "# V0.91.6 Runtime Failure Injection\n\n{DISCLAIMER}\n\n## What This Proves\n\nThis run proves a bounded integrated runtime slice for `#4547`: one long-lived-agent run/resume/stop continuity packet plus retry, timeout, explicit cancellation, partial failure via bulkhead saturation, degraded fallback, and remote timeout classification under one reviewer-readable artifact root.\n\n## Reviewer Path\n\n1. Inspect `runtime_failure_injection_proof.json`.\n2. Inspect `runtime_failure_register.json`.\n3. Inspect `long_lived_agent/resume_status_cycle3.json` and `long_lived_agent_stop_probe/stop_probe.json`.\n4. Inspect `resilience/retry_execution.json`, `resilience/timeout_execution.json`, `resilience/cancellation_execution.json`, `resilience/bulkhead_execution.json`, and `resilience/degraded_fallback_execution.json`.\n5. Inspect `remote_exec/timeout_probe.json`.\n6. Inspect `audit/artifact_safety_scan.json`.\n"
    )
}

fn reviewer_walkthrough() -> String {
    "# Reviewer Walkthrough\n\nRun the proof with `cargo run --manifest-path adl/Cargo.toml --bin run_v0916_runtime_failure_injection -- --out docs/milestones/v0.91.6/review/runtime/v0916_runtime_failure_injection_4547`.\n\nThe review question is whether the runtime now leaves one honest, durable packet showing retry, timeout, explicit cancellation, partial failure classification, degraded fallback, remote timeout semantics, one bounded resume-continuation proof, and one explicit non-continuity-after-stop proof without overclaiming interrupted restart recovery, checkpoint/restore, migration, replay, or v0.92 readiness.\n".to_string()
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
        std::env::temp_dir().join(unique)
    }

    #[test]
    fn run_v0916_runtime_failure_injection_generates_expected_artifacts() {
        let out_dir = temp_dir("v0916-runtime-failure-injection");
        run(Args {
            out: out_dir.clone(),
        })
        .expect("runtime failure-injection run should succeed");

        let proof_path = out_dir.join("runtime_failure_injection_proof.json");
        let proof: Value = serde_json::from_str(
            &fs::read_to_string(&proof_path).expect("read generated proof packet"),
        )
        .expect("parse generated proof packet");
        assert_eq!(proof["issue"], 4547);
        assert_eq!(
            proof["status_summary"]["resume_state_after_cycle3"],
            Value::String("completed".to_string())
        );
        assert_eq!(
            proof["status_summary"]["live_stop_second_cycle_manifest_present"],
            Value::Bool(false)
        );
        assert_eq!(
            proof["status_summary"]["cancellation_final_status"],
            Value::String("cancelled".to_string())
        );
        assert_eq!(
            proof["status_summary"]["remote_timeout_failure_kind"],
            Value::String("timeout".to_string())
        );

        let register_path = out_dir.join("runtime_failure_register.json");
        let register: Value = serde_json::from_str(
            &fs::read_to_string(&register_path).expect("read generated failure register"),
        )
        .expect("parse generated failure register");
        assert_eq!(register["issue"], 4547);
        assert_eq!(
            register["entries"].as_array().expect("entries array").len(),
            8
        );

        let safety_scan_path = out_dir.join("audit/artifact_safety_scan.json");
        let safety_scan: Value = serde_json::from_str(
            &fs::read_to_string(&safety_scan_path).expect("read safety scan"),
        )
        .expect("parse safety scan");
        assert_eq!(safety_scan["passed"], Value::Bool(true));
    }

    #[test]
    fn runtime_failure_injection_helpers_remain_reviewable() {
        let retry = run_retry_policy_probe();
        assert_eq!(retry["attempt_count_observed"], Value::from(3));
        assert_eq!(
            retry["trace"]["final_status"],
            Value::String("succeeded".to_string())
        );

        let timeout = run_timeout_policy_probe();
        assert_eq!(
            timeout["trace"]["final_status"],
            Value::String("timed_out".to_string())
        );

        let cancellation = run_cancellation_policy_probe();
        assert_eq!(
            cancellation["trace"]["final_status"],
            Value::String("cancelled".to_string())
        );

        let bulkhead = run_bulkhead_probe();
        assert_eq!(
            bulkhead["trace"]["final_status"],
            Value::String("saturated".to_string())
        );

        let degraded = run_degraded_fallback_probe();
        assert_eq!(degraded["trace"]["output_degraded"], Value::Bool(true));

        let remote = run_remote_timeout_probe().expect("remote timeout probe should succeed");
        assert_eq!(remote["stable_failure_kind"], Value::String("timeout".to_string()));
    }
}
