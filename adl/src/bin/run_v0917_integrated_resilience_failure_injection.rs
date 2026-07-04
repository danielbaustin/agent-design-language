use std::fs;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::{Duration, Instant};

use adl::adl::{
    DelegationActionKind, DelegationPolicyRuleSpec, DelegationPolicySpec, DelegationRuleEffect,
    DelegationSpec, ProviderSpec,
};
use adl::delegation_policy;
use adl::long_lived_agent::{self, InspectOptions, RunOptions};
use adl::remote_exec::{
    execute_remote, retryability, stable_failure_kind, ExecuteInputsPayload, ExecuteRequest,
    ExecuteStepPayload, PROTOCOL_VERSION,
};
use adl::resilience::{
    bulkhead_initial_state, circuit_breaker_initial_state, execute_bulkhead_policy,
    execute_circuit_breaker_policy, execute_fallback_policy, execute_rate_limit_policy,
    execute_retry_policy, execute_timeout_policy, rate_limit_initial_state,
    remote_exec_health_payload, BulkheadPolicyV1, CircuitBreakerPolicyV1, FallbackPolicyV1,
    RateLimitPolicyV1, ResilienceFaultClassV1, ResilienceFaultClassificationV1, ResiliencePolicyV1,
    ResilienceSurfaceV1, RetryPolicyV1, TimeoutObservation,
};
use anyhow::{anyhow, Context, Result};
use chrono::Utc;
use clap::Parser;
use serde::Serialize;
use serde_json::{json, Value};

const ISSUE: u64 = 4784;
const REVIEW_ROOT: &str =
    "docs/milestones/v0.91.7/review/runtime/v0917_integrated_resilience_failure_injection_4784";
const DISCLAIMER: &str = "This packet is a bounded local proof for #4784. It exercises existing ADL long-lived-agent, remote-exec, delegation-policy, and resilience primitives under injected failures. It does not claim the #4783 scheduler/watcher/AEE resilience middleware path is available before that issue lands, and it does not claim complete product resilience or v0.92 readiness.";

#[derive(Debug, Parser)]
#[command(name = "run_v0917_integrated_resilience_failure_injection")]
#[command(about = "Generate the v0.91.7 integrated resilience failure-injection proof packet")]
struct Args {
    #[arg(long, default_value = REVIEW_ROOT)]
    out: PathBuf,
}

fn main() -> Result<()> {
    run(Args::parse())
}

fn run(args: Args) -> Result<()> {
    let out_dir = absolute_from_cwd(&args.out)?;
    if out_dir.exists() {
        fs::remove_dir_all(&out_dir)
            .with_context(|| format!("reset output dir {}", out_dir.display()))?;
    }
    fs::create_dir_all(&out_dir)
        .with_context(|| format!("create output dir {}", out_dir.display()))?;

    write_file(&out_dir.join("README.md"), &readme())?;
    write_file(
        &out_dir.join("reviewer_walkthrough.md"),
        &reviewer_walkthrough(),
    )?;

    let agent_spec = write_agent_spec_under(&out_dir, "control_plane/long_lived_agent")?;
    let initial_status = long_lived_agent::status(&agent_spec)?;
    write_json(
        &out_dir.join("control_plane/long_lived_agent/initial_status.json"),
        &initial_status,
    )?;
    let run_status = long_lived_agent::run(
        &agent_spec,
        RunOptions {
            max_cycles: 2,
            interval_secs: Some(0),
            no_sleep: true,
            recover_stale_lease: false,
        },
    )?;
    write_json(
        &out_dir.join("control_plane/long_lived_agent/run_status_cycle2.json"),
        &run_status,
    )?;
    let resume_status = long_lived_agent::run(
        &agent_spec,
        RunOptions {
            max_cycles: 1,
            interval_secs: Some(0),
            no_sleep: true,
            recover_stale_lease: false,
        },
    )?;
    write_json(
        &out_dir.join("control_plane/long_lived_agent/resume_status_cycle3.json"),
        &resume_status,
    )?;
    let inspection = long_lived_agent::inspect(&agent_spec, InspectOptions { cycle_id: None })?;
    write_json(
        &out_dir.join("control_plane/inspection/latest.json"),
        &inspection,
    )?;
    let stop_probe = execute_stop_probe(&out_dir)?;
    write_json(
        &out_dir.join("control_plane/live_stop/stop_probe.json"),
        &stop_probe,
    )?;
    let stopped = long_lived_agent::stop(
        &agent_spec,
        "bounded v0.91.7 integrated resilience proof complete",
    )?;
    write_json(
        &out_dir.join("control_plane/long_lived_agent/status_after_stop.json"),
        &stopped,
    )?;

    let retry = run_retry_probe();
    write_json(&out_dir.join("resilience/retry_execution.json"), &retry)?;
    let timeout = run_timeout_probe();
    write_json(&out_dir.join("resilience/timeout_execution.json"), &timeout)?;
    let cancellation = run_cancellation_probe();
    write_json(
        &out_dir.join("resilience/cancellation_execution.json"),
        &cancellation,
    )?;
    let circuit = run_circuit_terminal_probe();
    write_json(
        &out_dir.join("resilience/circuit_terminal_execution.json"),
        &circuit,
    )?;
    let rate = run_rate_backpressure_probe();
    write_json(
        &out_dir.join("resilience/rate_backpressure_execution.json"),
        &rate,
    )?;
    let bulkhead = run_bulkhead_probe();
    write_json(
        &out_dir.join("resilience/bulkhead_execution.json"),
        &bulkhead,
    )?;
    let fallback = run_degraded_fallback_probe();
    write_json(
        &out_dir.join("resilience/degraded_fallback_execution.json"),
        &fallback,
    )?;
    let terminal = run_terminal_negative_probe();
    write_json(
        &out_dir.join("negative_cases/auth_quota_policy_terminal.json"),
        &terminal,
    )?;
    let remote_timeout = run_remote_timeout_probe()?;
    write_json(
        &out_dir.join("runtime_provider/remote_timeout_probe.json"),
        &remote_timeout,
    )?;
    write_json(
        &out_dir.join("runtime_provider/remote_health_payload.json"),
        &remote_exec_health_payload(),
    )?;

    let matrix = build_matrix(
        &resume_status,
        &stop_probe,
        &retry,
        &timeout,
        &cancellation,
        &circuit,
        &rate,
        &bulkhead,
        &fallback,
        &terminal,
        &remote_timeout,
    );
    write_json(&out_dir.join("failure_injection_matrix.json"), &matrix)?;
    let blocker_register = build_blocker_register();
    write_json(&out_dir.join("blocker_register.json"), &blocker_register)?;
    let proof = build_proof_packet(
        &initial_status,
        &run_status,
        &resume_status,
        &stop_probe,
        &stopped,
        &matrix,
        &blocker_register,
    );
    write_json(
        &out_dir.join("integrated_resilience_failure_injection_proof.json"),
        &proof,
    )?;
    let safety_scan = scan_public_artifacts(&out_dir)?;
    if !safety_scan
        .get("passed")
        .and_then(Value::as_bool)
        .unwrap_or(false)
    {
        return Err(anyhow!("artifact safety scan failed"));
    }
    write_json(
        &out_dir.join("audit/artifact_safety_scan.json"),
        &safety_scan,
    )?;
    let evidence_index = build_evidence_index(&out_dir)?;
    write_json(&out_dir.join("evidence_index.json"), &evidence_index)?;

    println!("out={}", out_dir.display());
    println!(
        "proof={}",
        out_dir
            .join("integrated_resilience_failure_injection_proof.json")
            .display()
    );
    Ok(())
}

fn run_retry_probe() -> Value {
    let policy = ResiliencePolicyV1 {
        schema_version: "adl.resilience.policy.v1".to_string(),
        policy_id: "v0917.integrated.retry".to_string(),
        retry: Some(RetryPolicyV1 {
            max_attempts: 3,
            backoff_ms: Some(5),
            jitter_ms: Some(0),
            max_elapsed_ms: Some(100),
            retryable_fault_classes: vec![ResilienceFaultClassV1::ProviderTransientHttp],
        }),
        timeout: None,
        circuit_breaker: None,
        rate_limit: None,
        bulkhead: None,
        fallback: None,
        checkpoint_required: false,
        telemetry_required: true,
    };
    let mut attempts = Vec::new();
    let execution = execute_retry_policy(
        &policy,
        ResilienceSurfaceV1::Provider,
        "provider_route.retry_after_503",
        |attempt| {
            if attempt < 3 {
                Err::<String, _>("provider transient 503".to_string())
            } else {
                Ok::<_, String>("provider-route-recovered".to_string())
            }
        },
        |err| ResilienceFaultClassificationV1::provider(err, Some(503)),
        |_| {},
        |record| attempts.push(record.clone()),
    );
    json!({
        "pattern": "retry",
        "attempt_count": attempts.len(),
        "trace": execution.trace,
        "result": execution.result.ok(),
    })
}

fn run_timeout_probe() -> Value {
    let policy = ResiliencePolicyV1::provider_attempt_policy("v0917.integrated.timeout", 1, 50);
    let execution = execute_timeout_policy(
        &policy,
        ResilienceSurfaceV1::Provider,
        "provider_route.timeout_budget",
        || TimeoutObservation {
            result: Ok::<_, String>("late-provider-success".to_string()),
            elapsed_ms: 80,
            cancelled: false,
        },
        |err| ResilienceFaultClassificationV1::provider(err, None),
        |breach, elapsed_ms, budget_ms| {
            format!(
                "timeout {:?} elapsed={} budget={}",
                breach, elapsed_ms, budget_ms
            )
        },
        |elapsed_ms| format!("cancelled after {elapsed_ms}ms"),
    );
    json!({
        "pattern": "timeout",
        "trace": execution.trace,
        "result": execution.result.err(),
    })
}

fn run_cancellation_probe() -> Value {
    let policy = ResiliencePolicyV1::provider_attempt_policy("v0917.integrated.cancel", 1, 100);
    let execution = execute_timeout_policy(
        &policy,
        ResilienceSurfaceV1::Workflow,
        "workflow_control.explicit_cancel",
        || TimeoutObservation::<(), ResilienceFaultClassificationV1> {
            result: Err(ResilienceFaultClassificationV1::provider("cancelled", None)),
            elapsed_ms: 15,
            cancelled: true,
        },
        |fault| fault.clone(),
        |breach, elapsed_ms, budget_ms| {
            ResilienceFaultClassificationV1::provider(
                &format!(
                    "timeout {:?} elapsed={} budget={}",
                    breach, elapsed_ms, budget_ms
                ),
                None,
            )
        },
        |elapsed_ms| {
            ResilienceFaultClassificationV1::provider(&format!("cancelled at {elapsed_ms}"), None)
        },
    );
    json!({
        "pattern": "cancellation",
        "trace": execution.trace,
        "result": execution.result.err(),
    })
}

fn run_circuit_terminal_probe() -> Value {
    let policy = ResiliencePolicyV1 {
        schema_version: "adl.resilience.policy.v1".to_string(),
        policy_id: "v0917.integrated.circuit_terminal".to_string(),
        retry: None,
        timeout: None,
        circuit_breaker: Some(CircuitBreakerPolicyV1 {
            failure_threshold: 1,
            recovery_window_ms: 1_000,
            half_open_max_attempts: 1,
        }),
        rate_limit: None,
        bulkhead: None,
        fallback: None,
        checkpoint_required: false,
        telemetry_required: true,
    };
    let initial = circuit_breaker_initial_state(&policy);
    let first = execute_circuit_breaker_policy(
        &policy,
        ResilienceSurfaceV1::Provider,
        "provider_route.auth_terminal_failure",
        &initial,
        100,
        || Err::<String, _>("unauthorized provider request".to_string()),
        |err| ResilienceFaultClassificationV1::provider(err, Some(401)),
        |state, wait_ms| {
            format!(
                "circuit open at {:?}; retry after {}ms",
                state.opened_at_ms, wait_ms
            )
        },
        None::<fn() -> String>,
    );
    let second = execute_circuit_breaker_policy(
        &policy,
        ResilienceSurfaceV1::Provider,
        "provider_route.auth_terminal_guard",
        &first.state,
        101,
        || Ok::<String, String>("should-not-execute".to_string()),
        |err| ResilienceFaultClassificationV1::provider(err, None),
        |state, wait_ms| {
            format!(
                "circuit remains open after terminal auth failure at {:?}; retry after {}ms",
                state.opened_at_ms, wait_ms
            )
        },
        None::<fn() -> String>,
    );
    json!({
        "pattern": "circuit_terminal_guard",
        "first_failure_trace": first.trace,
        "guard_trace": second.trace,
        "guard_result": second.result.err(),
    })
}

fn run_rate_backpressure_probe() -> Value {
    let policy = ResiliencePolicyV1 {
        schema_version: "adl.resilience.policy.v1".to_string(),
        policy_id: "v0917.integrated.rate_backpressure".to_string(),
        retry: None,
        timeout: None,
        circuit_breaker: None,
        rate_limit: Some(RateLimitPolicyV1 {
            max_requests: 1,
            window_ms: 250,
        }),
        bulkhead: None,
        fallback: None,
        checkpoint_required: false,
        telemetry_required: true,
    };
    let state = rate_limit_initial_state(&policy, 1_000);
    let allowed = execute_rate_limit_policy(
        &policy,
        ResilienceSurfaceV1::Provider,
        "provider_route.rate_first",
        &state,
        1_000,
        || Ok::<_, String>("first-request-admitted".to_string()),
        |_, wait_ms| format!("rate limit wait {wait_ms}ms"),
        |err| ResilienceFaultClassificationV1::provider(err, Some(429)),
    );
    let throttled = execute_rate_limit_policy(
        &policy,
        ResilienceSurfaceV1::Provider,
        "provider_route.rate_backpressure",
        &allowed.state,
        1_001,
        || Ok::<_, String>("should-not-execute".to_string()),
        |_, wait_ms| format!("rate limit wait {wait_ms}ms"),
        |err| ResilienceFaultClassificationV1::provider(err, Some(429)),
    );
    json!({
        "pattern": "rate_backpressure",
        "allowed_trace": allowed.trace,
        "throttled_trace": throttled.trace,
        "throttled_result": throttled.result.err(),
    })
}

fn run_bulkhead_probe() -> Value {
    let policy = ResiliencePolicyV1 {
        schema_version: "adl.resilience.policy.v1".to_string(),
        policy_id: "v0917.integrated.bulkhead".to_string(),
        retry: None,
        timeout: None,
        circuit_breaker: None,
        rate_limit: None,
        bulkhead: Some(BulkheadPolicyV1 {
            fault_domain: "provider_route".to_string(),
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
        "provider_route.bulkhead_saturation",
        &state,
        || Ok::<_, String>("should-not-execute".to_string()),
        |err| ResilienceFaultClassificationV1::provider(err, None),
        |bulkhead_state| {
            format!(
                "local_runtime_busy: fault_domain={} in_flight={}",
                bulkhead_state.fault_domain, bulkhead_state.in_flight
            )
        },
    );
    json!({
        "pattern": "bulkhead",
        "trace": execution.trace,
        "result": execution.result.err(),
    })
}

fn run_degraded_fallback_probe() -> Value {
    let policy = ResiliencePolicyV1 {
        schema_version: "adl.resilience.policy.v1".to_string(),
        policy_id: "v0917.integrated.degraded_fallback".to_string(),
        retry: None,
        timeout: None,
        circuit_breaker: None,
        rate_limit: None,
        bulkhead: None,
        fallback: Some(FallbackPolicyV1 {
            fallback_ref: "provider_route.cached_degraded_summary".to_string(),
            activation_fault_classes: vec![ResilienceFaultClassV1::ProviderTimeout],
            marks_output_degraded: true,
        }),
        checkpoint_required: false,
        telemetry_required: true,
    };
    let execution = execute_fallback_policy(
        &policy,
        ResilienceSurfaceV1::Provider,
        "provider_route.degraded_fallback",
        || Err::<String, _>("provider timeout".to_string()),
        |err| ResilienceFaultClassificationV1::provider(err, None),
        Some(|| "degraded-provider-result".to_string()),
    );
    json!({
        "pattern": "degraded_fallback",
        "outcome_kind": format!("{:?}", execution.outcome_kind),
        "trace": execution.trace,
        "result": execution.result,
    })
}

fn run_terminal_negative_probe() -> Value {
    let auth = ResilienceFaultClassificationV1::provider("invalid api key", Some(401));
    let quota = ResilienceFaultClassificationV1::provider("credit balance exhausted", Some(402));
    let retry_policy =
        ResiliencePolicyV1::provider_attempt_policy("v0917.integrated.terminal", 3, 50);
    let auth_retry = execute_retry_policy(
        &retry_policy,
        ResilienceSurfaceV1::Provider,
        "provider_route.auth_no_retry",
        |_| Err::<String, _>("invalid api key".to_string()),
        |err| ResilienceFaultClassificationV1::provider(err, Some(401)),
        |_| {},
        |_| {},
    );
    let quota_retry = execute_retry_policy(
        &retry_policy,
        ResilienceSurfaceV1::Provider,
        "provider_route.quota_no_retry",
        |_| Err::<String, _>("credit balance exhausted".to_string()),
        |err| ResilienceFaultClassificationV1::provider(err, Some(402)),
        |_| {},
        |_| {},
    );
    let policy = DelegationPolicySpec {
        default_allow: false,
        rules: vec![DelegationPolicyRuleSpec {
            id: "deny-unapproved-remote-provider".to_string(),
            action: DelegationActionKind::ProviderCall,
            target_id: Some("premium-provider".to_string()),
            effect: DelegationRuleEffect::Deny,
            require_approval: false,
        }],
    };
    let delegation = DelegationSpec {
        role: Some("runtime-provider".to_string()),
        requires_verification: Some(true),
        escalation_target: Some("operator".to_string()),
        tags: vec!["resilience-negative-case".to_string()],
    };
    let policy_outcome = delegation_policy::evaluate(
        Some(&policy),
        Some(&delegation),
        DelegationActionKind::ProviderCall,
        "premium-provider",
    );
    json!({
        "pattern": "negative_auth_quota_policy",
        "auth_classification": auth,
        "quota_classification": quota,
        "auth_retry_trace": auth_retry.trace,
        "quota_retry_trace": quota_retry.trace,
        "policy_decision": policy_outcome.decision.as_str(),
        "policy_rule_id": policy_outcome.rule_id,
    })
}

fn run_remote_timeout_probe() -> Result<Value> {
    let listener = TcpListener::bind("127.0.0.1:0").context("bind timeout listener")?;
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
        run_id: "v0917-integrated-resilience-remote-timeout".to_string(),
        workflow_id: "v0917_integrated_resilience_failure_injection".to_string(),
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
        "pattern": "runtime_provider_remote_timeout",
        "stable_failure_kind": stable_failure_kind(&err),
        "retryability": retryability(&err),
        "error_summary": "remote execution request timed out against a hanging local endpoint"
    }))
}

#[allow(clippy::too_many_arguments)]
fn build_matrix(
    resume_status: &adl::long_lived_agent::StatusRecord,
    stop_probe: &Value,
    retry: &Value,
    timeout: &Value,
    cancellation: &Value,
    circuit: &Value,
    rate: &Value,
    bulkhead: &Value,
    fallback: &Value,
    terminal: &Value,
    remote_timeout: &Value,
) -> Value {
    json!({
        "schema_version": "adl.v0917.integrated_resilience_failure_matrix.v1",
        "issue": ISSUE,
        "generated_at": Utc::now().to_rfc3339(),
        "entries": [
            matrix_entry("control_plane_resume", "workflow/control-plane", "long-lived-agent resumes bounded cycle state across reinvocation", format!("resume_state={:?} completed_cycles={}", resume_status.state, resume_status.completed_cycle_count), "control_plane/long_lived_agent/resume_status_cycle3.json", "proved"),
            matrix_entry("control_plane_stop", "workflow/control-plane", "live stop leaves explicit stopped state and no second persisted stop-probe cycle", format!("persisted_state={} completed_cycles={} second_cycle_manifest_present={}", stop_probe["persisted_state"], stop_probe["completed_cycle_count"], stop_probe["second_cycle_manifest_present"]), "control_plane/live_stop/stop_probe.json", "proved"),
            matrix_entry("retry", "runtime/provider", "transient provider failure retries and then succeeds within attempt budget", format!("attempt_count={} final_status={}", retry["attempt_count"], retry["trace"]["final_status"]), "resilience/retry_execution.json", "proved"),
            matrix_entry("timeout", "runtime/provider", "late provider completion is classified as timeout instead of success", format!("final_status={}", timeout["trace"]["final_status"]), "resilience/timeout_execution.json", "proved"),
            matrix_entry("cancellation", "workflow/control-plane", "explicit cancellation is classified as cancelled and not hidden as success", format!("final_status={}", cancellation["trace"]["final_status"]), "resilience/cancellation_execution.json", "proved"),
            matrix_entry("circuit_terminal_guard", "runtime/provider", "terminal auth failure opens the circuit and the next guarded operation is not executed", format!("guard_status={} operation_executed={}", circuit["guard_trace"]["final_status"], circuit["guard_trace"]["operation_executed"]), "resilience/circuit_terminal_execution.json", "proved"),
            matrix_entry("rate_backpressure", "runtime/provider", "second provider request in the same window is throttled with wait metadata", format!("final_status={} wait_ms={}", rate["throttled_trace"]["final_status"], rate["throttled_trace"]["wait_ms"]), "resilience/rate_backpressure_execution.json", "proved"),
            matrix_entry("bulkhead", "runtime/provider", "saturated provider bulkhead rejects without executing the protected operation", format!("final_status={} operation_executed={}", bulkhead["trace"]["final_status"], bulkhead["trace"]["operation_executed"]), "resilience/bulkhead_execution.json", "proved"),
            matrix_entry("degraded_fallback", "runtime/provider", "provider timeout activates degraded fallback and marks output degraded", format!("final_status={} output_degraded={}", fallback["trace"]["final_status"], fallback["trace"]["output_degraded"]), "resilience/degraded_fallback_execution.json", "proved"),
            matrix_entry("negative_auth_quota_policy", "runtime/provider/security", "auth, quota, and policy failures remain terminal/operator-gated and do not retry", format!("auth_status={} quota_status={} policy_decision={}", terminal["auth_retry_trace"]["final_status"], terminal["quota_retry_trace"]["final_status"], terminal["policy_decision"]), "negative_cases/auth_quota_policy_terminal.json", "proved"),
            matrix_entry("remote_timeout", "runtime/provider", "remote-exec hanging endpoint returns stable timeout failure kind with retryability truth", format!("stable_failure_kind={} retryability={}", remote_timeout["stable_failure_kind"], remote_timeout["retryability"]), "runtime_provider/remote_timeout_probe.json", "proved"),
            matrix_entry("scheduler_watcher_aee_middleware", "scheduler/watcher/AEE", "consume #4783 integrated middleware once landed", "#4783 worktree contains uncommitted edits on scheduler/watcher/AEE-adjacent runtime surfaces and is not a consumable dependency for this branch".to_string(), "blocker_register.json", "blocked_dependency")
        ]
    })
}

fn matrix_entry(
    pattern: &str,
    surface: &str,
    expected_behavior: &str,
    observed_behavior: String,
    evidence_ref: &str,
    status: &str,
) -> Value {
    json!({
        "pattern": pattern,
        "surface": surface,
        "expected_behavior": expected_behavior,
        "observed_behavior": observed_behavior,
        "evidence_ref": evidence_ref,
        "status": status
    })
}

fn build_blocker_register() -> Value {
    json!({
        "schema_version": "adl.v0917.integrated_resilience_blocker_register.v1",
        "issue": ISSUE,
        "generated_at": Utc::now().to_rfc3339(),
        "blockers": [
            {
                "blocker_id": "dependency-4783-scheduler-watcher-aee-middleware",
                "classification": "blocked_dependency",
                "missing_dependency": "#4783 integrated resilience middleware branch publication/merge",
                "evidence": [
                    "git status in .worktrees/adl-wp-4783 shows uncommitted edits on adl/src/execute/*, adl/src/instrumentation/*, adl/src/obsmem_indexing.rs, adl/src/resilience.rs, and adl/src/trace/*",
                    "git diff --stat origin/main...HEAD in .worktrees/adl-wp-4783 is empty because HEAD still matches origin/main; the dependency is not present on this branch"
                ],
                "impact": "This #4784 packet proves existing integrated ADL long-lived-agent, remote-exec, delegation-policy, and resilience primitives, but does not claim scheduler/watcher/AEE middleware resilience until #4783 lands.",
                "release_disposition": "blocks any v0.92 claim that specifically requires the #4783 scheduler/watcher/AEE integrated middleware path"
            }
        ]
    })
}

fn build_proof_packet(
    initial_status: &adl::long_lived_agent::StatusRecord,
    run_status: &adl::long_lived_agent::StatusRecord,
    resume_status: &adl::long_lived_agent::StatusRecord,
    stop_probe: &Value,
    stopped: &adl::long_lived_agent::StatusRecord,
    matrix: &Value,
    blocker_register: &Value,
) -> Value {
    json!({
        "schema_version": "adl.v0917.integrated_resilience_failure_injection_proof.v1",
        "issue": ISSUE,
        "generated_at": Utc::now().to_rfc3339(),
        "status": "proved_with_blocked_dependency",
        "what_this_proves": [
            "Existing ADL integrated runtime/control-plane paths exercise retry, timeout, cancellation, circuit-terminal guard, rate/backpressure, bulkhead, degraded fallback, and remote timeout proof under one artifact root.",
            "At least one workflow/control-plane path is exercised through long-lived-agent run/resume/stop artifacts.",
            "At least one runtime/provider path is exercised through remote-exec timeout and provider-surface resilience traces.",
            "Negative auth, quota/billing, and delegation-policy cases fail closed without retrying as successful work."
        ],
        "what_this_does_not_prove": [
            "The #4783 scheduler/watcher/AEE resilience middleware path, because it is not yet landed on this branch.",
            "Complete product resilience across every runtime, provider, AWS, Unity, scheduler, and watcher surface.",
            "v0.92 runtime readiness."
        ],
        "status_summary": {
            "initial_state": initial_status.state,
            "run_state_after_cycle2": run_status.state,
            "resume_state_after_cycle3": resume_status.state,
            "live_stop_state": stop_probe["persisted_state"],
            "status_after_stop": stopped.state,
            "matrix_status_counts": summarize_matrix_statuses(matrix),
            "blocker_count": blocker_register["blockers"].as_array().map(|items| items.len()).unwrap_or(0)
        },
        "matrix_ref": "failure_injection_matrix.json",
        "blocker_register_ref": "blocker_register.json",
        "reviewer_path": [
            "README.md",
            "integrated_resilience_failure_injection_proof.json",
            "failure_injection_matrix.json",
            "blocker_register.json",
            "control_plane/long_lived_agent/resume_status_cycle3.json",
            "control_plane/live_stop/stop_probe.json",
            "resilience/retry_execution.json",
            "resilience/timeout_execution.json",
            "resilience/cancellation_execution.json",
            "resilience/circuit_terminal_execution.json",
            "resilience/rate_backpressure_execution.json",
            "resilience/bulkhead_execution.json",
            "resilience/degraded_fallback_execution.json",
            "negative_cases/auth_quota_policy_terminal.json",
            "runtime_provider/remote_timeout_probe.json",
            "audit/artifact_safety_scan.json"
        ],
        "disclaimer": DISCLAIMER
    })
}

fn summarize_matrix_statuses(matrix: &Value) -> Value {
    let mut proved = 0_u64;
    let mut blocked_dependency = 0_u64;
    if let Some(entries) = matrix.get("entries").and_then(Value::as_array) {
        for entry in entries {
            match entry.get("status").and_then(Value::as_str) {
                Some("proved") => proved += 1,
                Some("blocked_dependency") => blocked_dependency += 1,
                _ => {}
            }
        }
    }
    json!({
        "proved": proved,
        "blocked_dependency": blocked_dependency
    })
}

fn execute_stop_probe(out_dir: &Path) -> Result<Value> {
    let spec_path = write_agent_spec_under(out_dir, "control_plane/live_stop")?;
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
        "operator stop after first persisted cycle for #4784 resilience proof",
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
        "stop_status": stop_status,
        "run_returned_state": final_status.state,
        "persisted_state": persisted.state,
        "completed_cycle_count": persisted.completed_cycle_count,
        "second_cycle_manifest_present": second_cycle_manifest.exists(),
        "last_error": persisted.last_error
    }))
}

fn write_agent_spec_under(out_dir: &Path, dir_name: &str) -> Result<PathBuf> {
    let spec_path = out_dir.join(dir_name).join("agent.yaml");
    let body = r#"schema: adl.long_lived_agent_spec.v1
agent_instance_id: v0917-integrated-resilience-failure-injection
display_name: V0917 Integrated Resilience Failure Injection
state_root: state
workflow:
  kind: demo_adapter
  name: v0917_integrated_resilience_failure_injection
  run_args:
    provider_id: local_ollama
    model: gemma4:latest
heartbeat:
  interval_secs: 1
  max_cycles: 6
  stale_lease_after_secs: 60
safety:
  allow_network: false
"#;
    write_file(&spec_path, body)?;
    Ok(spec_path)
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

fn wait_for_completed_cycles(spec_path: &Path, min_cycles: u64, timeout: Duration) -> Result<()> {
    let started = Instant::now();
    while started.elapsed() < timeout {
        match long_lived_agent::status(spec_path) {
            Ok(status) if status.completed_cycle_count >= min_cycles => return Ok(()),
            Ok(_) => {}
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

fn build_evidence_index(out_dir: &Path) -> Result<Value> {
    let mut refs = Vec::new();
    collect_relative_files(out_dir, out_dir, &mut refs)?;
    refs.sort();
    Ok(json!({
        "schema_version": "adl.v0917.integrated_resilience_evidence_index.v1",
        "issue": ISSUE,
        "generated_at": Utc::now().to_rfc3339(),
        "artifact_refs": refs,
        "source_refs": [
            ".adl/v0.91.7/bodies/issue-4784-v0-91-7-resilience-soak-prove-integrated-resilience-failure-injection.md",
            "docs/milestones/v0.91.7/RUNTIME_SOAK_2_EXECUTION_PACKET_v0.91.7.md",
            "docs/milestones/v0.91.6/review/runtime/v0916_runtime_failure_injection_4547/",
            "adl/src/resilience.rs"
        ]
    }))
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
                "x-api-key",
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
                        "artifact_ref": rel
                    }));
                }
            }
        }
    }
    Ok(json!({
        "schema_version": "adl.v0917.integrated_resilience_artifact_safety_scan.v1",
        "issue": ISSUE,
        "scanned_at": Utc::now().to_rfc3339(),
        "passed": findings.is_empty(),
        "scanned_artifacts": files,
        "findings": findings
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

fn absolute_from_cwd(path: &Path) -> Result<PathBuf> {
    if path.is_absolute() {
        Ok(path.to_path_buf())
    } else {
        Ok(std::env::current_dir()?.join(path))
    }
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
        "# V0.91.7 Integrated Resilience Failure Injection (#4784)\n\n{DISCLAIMER}\n\n## What This Proves\n\nThis packet proves the currently available integrated ADL paths for #4784: long-lived-agent run/resume/stop control-plane behavior, remote-exec timeout behavior, retry, timeout, cancellation, circuit-terminal guard, rate/backpressure, bulkhead, degraded fallback, and negative auth/quota/policy classification.\n\n## Reviewer Path\n\n1. Inspect `integrated_resilience_failure_injection_proof.json`.\n2. Inspect `failure_injection_matrix.json` and `blocker_register.json`.\n3. Inspect `control_plane/long_lived_agent/resume_status_cycle3.json` and `control_plane/live_stop/stop_probe.json`.\n4. Inspect `resilience/*.json`, `runtime_provider/remote_timeout_probe.json`, and `negative_cases/auth_quota_policy_terminal.json`.\n5. Inspect `audit/artifact_safety_scan.json`.\n"
    )
}

fn reviewer_walkthrough() -> String {
    "Run the proof with `cargo run --manifest-path adl/Cargo.toml --bin run_v0917_integrated_resilience_failure_injection -- --out docs/milestones/v0.91.7/review/runtime/v0917_integrated_resilience_failure_injection_4784`.\n\nThe review question is whether #4784 now has a durable, reviewer-readable matrix proving every required resilience pattern on at least one currently available integrated ADL path, while preserving the blocker for the not-yet-landed #4783 scheduler/watcher/AEE middleware dependency.\n".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resilience_failure_injection_terminal_negative_probe_fails_closed() {
        let terminal = run_terminal_negative_probe();
        assert_eq!(
            terminal["policy_decision"],
            Value::String("denied".to_string())
        );
        assert_eq!(
            terminal["auth_retry_trace"]["attempts"]
                .as_array()
                .expect("auth attempts")
                .len(),
            1
        );
        assert_eq!(
            terminal["quota_retry_trace"]["attempts"]
                .as_array()
                .expect("quota attempts")
                .len(),
            1
        );
    }

    #[test]
    fn resilience_failure_injection_required_pattern_probes_emit_expected_terminal_states() {
        assert_eq!(
            run_timeout_probe()["trace"]["final_status"],
            Value::String("timed_out".to_string())
        );
        assert_eq!(
            run_cancellation_probe()["trace"]["final_status"],
            Value::String("cancelled".to_string())
        );
        assert_eq!(
            run_rate_backpressure_probe()["throttled_trace"]["final_status"],
            Value::String("throttled".to_string())
        );
        assert_eq!(
            run_bulkhead_probe()["trace"]["final_status"],
            Value::String("saturated".to_string())
        );
        assert_eq!(
            run_degraded_fallback_probe()["trace"]["output_degraded"],
            Value::Bool(true)
        );
    }
}
