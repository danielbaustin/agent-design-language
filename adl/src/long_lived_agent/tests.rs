//! Integration tests for long-lived agent execution and artifact invariants.
use super::*;
use crate::observability::test_env_lock;
use crate::runtime_aws_signal::mock_signal_artifact_path;
use std::env;
use std::ffi::OsString;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::MutexGuard;
use std::time::Instant;

static TEMP_SEQ: AtomicU64 = AtomicU64::new(0);

fn temp_dir(prefix: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "adl-long-lived-agent-{prefix}-{}-{}",
        std::process::id(),
        TEMP_SEQ.fetch_add(1, Ordering::Relaxed)
    ));
    fs::create_dir_all(&dir).expect("create temp dir");
    dir
}

fn write_spec(root: &Path) -> PathBuf {
    write_spec_with_workflow_kind(root, "demo_adapter")
}

fn write_spec_with_workflow_kind(root: &Path, workflow_kind: &str) -> PathBuf {
    write_spec_with_safety(root, workflow_kind, false, false)
}

fn write_spec_with_safety(
    root: &Path,
    workflow_kind: &str,
    allow_broker: bool,
    financial_advice: bool,
) -> PathBuf {
    write_spec_with_safety_and_run_args(
        root,
        workflow_kind,
        allow_broker,
        financial_advice,
        "    provider_id: local_ollama\n    model: gemma4:latest\n",
    )
}

fn write_spec_with_safety_and_run_args(
    root: &Path,
    workflow_kind: &str,
    allow_broker: bool,
    financial_advice: bool,
    run_args: &str,
) -> PathBuf {
    let spec = root.join("agent.yaml");
    fs::write(
        &spec,
        format!(
            r#"schema: adl.long_lived_agent_spec.v1
agent_instance_id: test-agent
display_name: Test Agent
state_root: state
workflow:
  kind: {workflow_kind}
  name: wp02_heartbeat_probe
  run_args:
{run_args}heartbeat:
  interval_secs: 1
  max_cycles: 3
  stale_lease_after_secs: 60
safety:
  allow_network: false
  allow_broker: {allow_broker}
  allow_filesystem_writes_outside_state_root: false
  allow_real_world_side_effects: false
  require_public_artifact_sanitization: true
  financial_advice: {financial_advice}
  max_cycle_runtime_secs: 120
  max_consecutive_failures: 2
memory:
  namespace: tests/test-agent
  write_policy: append_only
"#,
        ),
    )
    .expect("write spec");
    spec
}

fn required_state_files(root: &Path) -> Vec<PathBuf> {
    [
        "agent_spec.locked.json",
        "continuity.json",
        "cycle_ledger.jsonl",
        "status.json",
        "provider_binding_history.jsonl",
        "memory_index.json",
    ]
    .into_iter()
    .map(|name| root.join("state").join(name))
    .collect()
}

fn required_cycle_files(root: &Path, cycle_id: &str) -> Vec<PathBuf> {
    let dir = root.join("state/cycles").join(cycle_id);
    [
        "cycle_manifest.json",
        "observations.json",
        "decision_request.json",
        "decision_result.json",
        "run_ref.json",
        "memory_writes.jsonl",
        "guardrail_report.json",
        "cycle_summary.md",
    ]
    .into_iter()
    .map(|name| dir.join(name))
    .collect()
}

fn guardrail_check_result<'a>(guardrails: &'a Value, check_id: &str) -> &'a str {
    guardrails["checks"]
        .as_array()
        .expect("checks")
        .iter()
        .find(|check| check["check_id"] == check_id)
        .and_then(|check| check["result"].as_str())
        .unwrap_or_else(|| panic!("missing check {check_id}"))
}

struct MultiEnvGuard {
    saved: Vec<(String, Option<OsString>)>,
    _lock: MutexGuard<'static, ()>,
}

impl MultiEnvGuard {
    fn set_all(values: &[(&str, &str)]) -> Self {
        let lock = test_env_lock();
        let mut saved = Vec::with_capacity(values.len());
        for (key, value) in values {
            saved.push(((*key).to_string(), env::var_os(key)));
            unsafe {
                env::set_var(key, value);
            }
        }
        Self { saved, _lock: lock }
    }
}

impl Drop for MultiEnvGuard {
    fn drop(&mut self) {
        unsafe {
            for (key, old) in self.saved.iter().rev() {
                match old {
                    Some(value) => env::set_var(key, value),
                    None => env::remove_var(key),
                }
            }
        }
    }
}

#[test]
fn status_initializes_required_continuity_files_without_running_cycle() {
    let root = temp_dir("init");
    let spec = write_spec(&root);

    let initialized = status(&spec).expect("status initializes continuity");

    assert_eq!(initialized.state, AgentStatusState::NotStarted);
    assert_eq!(initialized.completed_cycle_count, 0);
    for path in required_state_files(&root) {
        assert!(path.exists(), "missing {}", path.display());
    }
    let ledger = fs::read_to_string(root.join("state/cycle_ledger.jsonl")).expect("read ledger");
    assert_eq!(ledger.lines().count(), 0);
    let continuity: Value =
        serde_json::from_str(&fs::read_to_string(root.join("state/continuity.json")).unwrap())
            .expect("parse continuity");
    assert_eq!(continuity["continuity_kind"], "pre_v0_92_handle");
    assert_eq!(continuity["future_identity_ref"], Value::Null);
    assert_eq!(continuity["latest_cycle_id"], Value::Null);
}

#[test]
fn tick_creates_state_status_full_cycle_bundle_and_removes_lease() {
    let root = temp_dir("tick");
    let spec = write_spec(&root);

    let status = tick(&spec, TickOptions::default()).expect("tick");

    assert_eq!(status.state, AgentStatusState::Idle);
    assert_eq!(status.completed_cycle_count, 1);
    assert_eq!(status.last_cycle_id.as_deref(), Some("cycle-000001"));
    for path in required_state_files(&root) {
        assert!(path.exists(), "missing {}", path.display());
    }
    for path in required_cycle_files(&root, "cycle-000001") {
        assert!(path.exists(), "missing {}", path.display());
    }
    assert!(!root
        .join("state/cycles/cycle-000001/heartbeat.json")
        .exists());
    let manifest: Value = serde_json::from_str(
        &fs::read_to_string(root.join("state/cycles/cycle-000001/cycle_manifest.json"))
            .expect("read manifest"),
    )
    .expect("parse manifest");
    assert_eq!(manifest["schema"], CYCLE_MANIFEST_SCHEMA);
    assert_eq!(manifest["status"], "success");
    assert_eq!(manifest["previous_cycle_id"], Value::Null);
    assert!(manifest["input_hash"]
        .as_str()
        .expect("input hash")
        .starts_with("sha256:"));
    let decision_request: Value = serde_json::from_str(
        &fs::read_to_string(root.join("state/cycles/cycle-000001/decision_request.json"))
            .expect("read request"),
    )
    .expect("parse request");
    assert_eq!(decision_request["forbidden_actions"][0], "execute_order");
    let memory_writes =
        fs::read_to_string(root.join("state/cycles/cycle-000001/memory_writes.jsonl"))
            .expect("read memory writes");
    assert_eq!(memory_writes.lines().count(), 1);
    let continuity: Value =
        serde_json::from_str(&fs::read_to_string(root.join("state/continuity.json")).unwrap())
            .expect("parse continuity");
    assert_eq!(continuity["schema"], CONTINUITY_SCHEMA);
    assert_eq!(continuity["continuity_kind"], "pre_v0_92_handle");
    assert_eq!(continuity["latest_cycle_id"], "cycle-000001");
    assert!(continuity["non_claims"]
        .as_array()
        .expect("non claims")
        .contains(&json!("not_v0_92_identity_tuple")));
    let ledger =
        fs::read_to_string(root.join("state/cycle_ledger.jsonl")).expect("read cycle ledger");
    assert_eq!(ledger.lines().count(), 1);
    let ledger_entry: Value = serde_json::from_str(ledger.lines().next().expect("ledger line"))
        .expect("parse ledger entry");
    assert_eq!(ledger_entry["schema"], CYCLE_LEDGER_ENTRY_SCHEMA);
    assert_eq!(ledger_entry["continuity_kind"], "pre_v0_92_handle");
    let provider_history = fs::read_to_string(root.join("state/provider_binding_history.jsonl"))
        .expect("read provider history");
    let provider_entry: Value =
        serde_json::from_str(provider_history.lines().next().expect("provider line"))
            .expect("parse provider binding");
    assert_eq!(provider_entry["schema"], PROVIDER_BINDING_SCHEMA);
    assert_eq!(provider_entry["provider_id"], "local_ollama");
    assert_eq!(provider_entry["model"], "gemma4:latest");
    assert_eq!(provider_entry["binding_status"], "available");
    let memory_index: Value =
        serde_json::from_str(&fs::read_to_string(root.join("state/memory_index.json")).unwrap())
            .expect("parse memory index");
    assert_eq!(memory_index["schema"], MEMORY_INDEX_SCHEMA);
    assert_eq!(
        memory_index["local_memory_refs"][0],
        "cycles/cycle-000001/memory_writes.jsonl"
    );
    assert!(!root.join("state/lease.json").exists());
}

#[test]
fn tick_mock_mode_writes_runtime_aws_heartbeat_envelopes() {
    let root = temp_dir("aws-heartbeat-mock");
    let spec = write_spec(&root);
    let _env = MultiEnvGuard::set_all(&[
        ("ADL_AWS_SIGNAL_MODE", "mock"),
        ("ADL_AWS_REGION", "us-west-2"),
    ]);

    let status = tick(&spec, TickOptions::default()).expect("tick");

    assert_eq!(status.state, AgentStatusState::Idle);
    let loaded = load_spec(&spec).expect("load");
    let mock_path = mock_signal_artifact_path(&loaded);
    assert!(mock_path.exists(), "missing {}", mock_path.display());
    let lines = fs::read_to_string(&mock_path).expect("read heartbeat mock log");
    let events = lines
        .lines()
        .map(|line| serde_json::from_str::<Value>(line).expect("parse heartbeat envelope"))
        .collect::<Vec<_>>();
    assert!(
        events.len() >= 3,
        "expected initialization, running, and completed heartbeat events"
    );
    let last = events.last().expect("last envelope");
    assert_eq!(last["schema_version"], "adl.runtime.aws_signal.v1");
    assert_eq!(last["signal_kind"], "heartbeat");
    assert_eq!(last["runtime_id"], "test-agent");
    assert_eq!(last["cycle_id"], "cycle-000001");
    assert_eq!(last["projection_level"], "operations_safe");
    assert_eq!(last["transport"]["mode"], "mock");
    assert_eq!(last["transport"]["target_kind"], "cloudwatch_logs");
    assert_eq!(last["transport"]["region"], "us-west-2");
    assert_eq!(last["payload"]["state"], "idle");
    assert_eq!(
        last["payload"]["next_cycle_hint"],
        "sleep_until_next_heartbeat"
    );
    assert_eq!(last["payload"]["lease_state"], "clear");
}

#[test]
fn repeated_mock_heartbeats_advance_sequence_and_correlation_id() {
    let root = temp_dir("aws-heartbeat-sequence");
    let spec = write_spec(&root);
    let _env = MultiEnvGuard::set_all(&[
        ("ADL_AWS_SIGNAL_MODE", "mock"),
        ("ADL_AWS_REGION", "us-west-2"),
    ]);

    tick(&spec, TickOptions::default()).expect("tick");
    let status_after_tick = status(&spec).expect("status");
    assert_eq!(status_after_tick.state, AgentStatusState::Idle);

    let loaded = load_spec(&spec).expect("load");
    let lines = fs::read_to_string(mock_signal_artifact_path(&loaded)).expect("read mock log");
    let events = lines
        .lines()
        .map(|line| serde_json::from_str::<Value>(line).expect("parse heartbeat envelope"))
        .collect::<Vec<_>>();
    let mut completed_cycle_events = events
        .iter()
        .filter(|value| {
            value["cycle_id"] == "cycle-000001"
                && value["payload"]["state"] == "idle"
                && value["status"] == "completed"
        })
        .collect::<Vec<_>>();
    completed_cycle_events.sort_by_key(|value| value["heartbeat_seq"].as_u64().unwrap_or(0));
    assert!(
        completed_cycle_events.len() >= 2,
        "expected repeated completed heartbeats for the same cycle"
    );
    let first = completed_cycle_events[0];
    let later = completed_cycle_events[1];
    assert!(
        later["heartbeat_seq"].as_u64().expect("later seq")
            > first["heartbeat_seq"].as_u64().expect("first seq")
    );
    assert_ne!(later["correlation_id"], first["correlation_id"]);
}

#[test]
fn mock_mode_rejects_unsupported_heartbeat_target() {
    let root = temp_dir("aws-heartbeat-unsupported-target");
    let spec = write_spec(&root);
    let observability_log = root.join("aws-heartbeat-events.log");
    let _env = MultiEnvGuard::set_all(&[
        ("ADL_AWS_SIGNAL_MODE", "mock"),
        ("ADL_AWS_REGION", "us-west-2"),
        ("ADL_AWS_HEARTBEAT_TARGET", "sns"),
        (
            "ADL_OBSERVABILITY_LOG",
            observability_log.to_str().expect("observability path utf8"),
        ),
        ("ADL_OBSERVABILITY_STDERR", "0"),
    ]);

    let status = tick(&spec, TickOptions::default()).expect("tick should still succeed");

    assert_eq!(status.state, AgentStatusState::Idle);
    let logged = fs::read_to_string(&observability_log).expect("read observability log");
    assert!(logged.contains("stage=aws_runtime_heartbeat"));
    assert!(logged.contains("result=failed"));
    assert!(logged.contains("failure_class=aws_signal_unsupported_target"));

    let loaded = load_spec(&spec).expect("load");
    assert!(
        !mock_signal_artifact_path(&loaded).exists(),
        "unsupported target should not write mock heartbeat envelopes"
    );
}

#[test]
fn disabled_mode_skips_without_writing_mock_or_cursor_artifacts() {
    let root = temp_dir("aws-heartbeat-disabled");
    let spec = write_spec(&root);
    let observability_log = root.join("aws-heartbeat-events.log");
    let _env = MultiEnvGuard::set_all(&[
        ("ADL_AWS_SIGNAL_MODE", "disabled"),
        (
            "ADL_OBSERVABILITY_LOG",
            observability_log.to_str().expect("observability path utf8"),
        ),
        ("ADL_OBSERVABILITY_STDERR", "0"),
    ]);

    let status = tick(&spec, TickOptions::default()).expect("tick should still succeed");
    assert_eq!(status.state, AgentStatusState::Idle);

    let logged = fs::read_to_string(&observability_log).expect("read observability log");
    assert!(logged.contains("stage=aws_runtime_heartbeat"));
    assert!(logged.contains("result=skipped"));

    let loaded = load_spec(&spec).expect("load");
    assert!(
        !mock_signal_artifact_path(&loaded).exists(),
        "disabled mode should not write mock heartbeat envelopes"
    );
    assert!(
        !loaded
            .state_root
            .join("aws_runtime_heartbeat_cursor.json")
            .exists(),
        "disabled mode should not allocate heartbeat cursor state"
    );
}

#[test]
fn tick_live_mode_without_approval_stays_fail_closed_and_observable() {
    let root = temp_dir("aws-heartbeat-live-blocked");
    let spec = write_spec(&root);
    let observability_log = root.join("aws-heartbeat-events.log");
    let _env = MultiEnvGuard::set_all(&[
        ("ADL_AWS_SIGNAL_MODE", "live"),
        ("ADL_AWS_REGION", "us-west-2"),
        ("ADL_AWS_HEARTBEAT_TARGET", "cloudwatch_logs"),
        (
            "ADL_AWS_HEARTBEAT_LOG_GROUP",
            "arn:aws:logs:us-west-2:123456789012:log-group/private",
        ),
        (
            "ADL_AWS_HEARTBEAT_LOG_STREAM",
            "arn:aws:logs:us-west-2:123456789012:log-stream/private",
        ),
        (
            "ADL_OBSERVABILITY_LOG",
            observability_log.to_str().expect("observability path utf8"),
        ),
        ("ADL_OBSERVABILITY_STDERR", "0"),
    ]);

    let status = tick(&spec, TickOptions::default()).expect("tick should still succeed");

    assert_eq!(status.state, AgentStatusState::Idle);
    let logged = fs::read_to_string(&observability_log).expect("read observability log");
    assert!(logged.contains("stage=aws_runtime_heartbeat"));
    assert!(logged.contains("result=failed"));
    assert!(logged.contains("failure_class=aws_signal_live_not_approved"));
    assert!(!logged.contains("123456789012"));
    assert!(!logged.contains("arn:aws:logs"));

    let loaded = load_spec(&spec).expect("load");
    assert!(
        !mock_signal_artifact_path(&loaded).exists(),
        "live-blocked mode should not write mock heartbeat envelopes"
    );
    assert!(
        !loaded
            .state_root
            .join("aws_runtime_heartbeat_cursor.json")
            .exists(),
        "live-blocked mode should not allocate heartbeat cursor state"
    );
}

#[test]
fn run_max_cycles_no_sleep_writes_exactly_three_cycles_and_completed_status() {
    let root = temp_dir("run");
    let spec = write_spec(&root);

    let status = run(
        &spec,
        RunOptions {
            max_cycles: 3,
            interval_secs: None,
            no_sleep: true,
            recover_stale_lease: false,
        },
    )
    .expect("run");

    assert_eq!(status.state, AgentStatusState::Completed);
    assert_eq!(status.completed_cycle_count, 3);
    assert!(root.join("state/cycles/cycle-000001").exists());
    assert!(root.join("state/cycles/cycle-000002").exists());
    assert!(root.join("state/cycles/cycle-000003").exists());
    assert!(!root.join("state/cycles/cycle-000004").exists());
    let ledger =
        fs::read_to_string(root.join("state/cycle_ledger.jsonl")).expect("read cycle ledger");
    assert_eq!(ledger.lines().count(), 3);
    let provider_history = fs::read_to_string(root.join("state/provider_binding_history.jsonl"))
        .expect("read provider history");
    assert_eq!(provider_history.lines().count(), 3);
    let continuity: Value =
        serde_json::from_str(&fs::read_to_string(root.join("state/continuity.json")).unwrap())
            .expect("parse continuity");
    assert_eq!(continuity["latest_cycle_id"], "cycle-000003");
    let memory_index: Value =
        serde_json::from_str(&fs::read_to_string(root.join("state/memory_index.json")).unwrap())
            .expect("parse memory index");
    assert_eq!(
        memory_index["local_memory_refs"]
            .as_array()
            .expect("memory refs")
            .len(),
        3
    );
    let manifest: Value = serde_json::from_str(
        &fs::read_to_string(root.join("state/cycles/cycle-000002/cycle_manifest.json"))
            .expect("read manifest"),
    )
    .expect("parse manifest");
    assert_eq!(manifest["previous_cycle_id"], "cycle-000001");
}

#[test]
fn run_with_interval_sleep_preserves_cycle_count_and_waits_between_cycles() {
    let root = temp_dir("run-with-sleep");
    let spec = write_spec(&root);
    let started = Instant::now();

    let status = run(
        &spec,
        RunOptions {
            max_cycles: 2,
            interval_secs: Some(1),
            no_sleep: false,
            recover_stale_lease: false,
        },
    )
    .expect("run");

    assert_eq!(status.state, AgentStatusState::Completed);
    assert_eq!(status.completed_cycle_count, 2);
    assert!(
        started.elapsed() >= Duration::from_secs(1),
        "expected Tokio-backed cadence wait between cycles"
    );
    let ledger =
        fs::read_to_string(root.join("state/cycle_ledger.jsonl")).expect("read cycle ledger");
    assert_eq!(ledger.lines().count(), 2);
}

#[test]
fn inspect_latest_cycle_emits_reviewer_proof_packet() {
    let root = temp_dir("inspect-latest");
    let spec = write_spec(&root);
    run(
        &spec,
        RunOptions {
            max_cycles: 2,
            interval_secs: None,
            no_sleep: true,
            recover_stale_lease: false,
        },
    )
    .expect("run");

    let packet = inspect(&spec, InspectOptions::default()).expect("inspect latest");

    assert_eq!(packet["schema"], INSPECTION_PACKET_SCHEMA);
    assert_eq!(packet["agent_instance_id"], "test-agent");
    assert_eq!(packet["reviewer_proof"]["status"], "pass");
    assert_eq!(
        packet["selected_cycle"]["refs"]["manifest"],
        "cycles/cycle-000002/cycle_manifest.json"
    );
    assert_eq!(
        packet["selected_cycle"]["refs"]["guardrail_report"],
        "cycles/cycle-000002/guardrail_report.json"
    );
    assert_eq!(
        packet["selected_cycle"]["refs"]["cycle_summary"],
        "cycles/cycle-000002/cycle_summary.md"
    );
    assert_eq!(packet["selected_cycle"]["guardrails"]["status"], "pass");
    assert_eq!(
        packet["selected_cycle"]["trace_boundary"]["status"],
        "cycle_artifact_only"
    );
    assert_eq!(
        packet["trace_query_decision"]["full_tql_platform"],
        "deferred"
    );
    assert_eq!(
        packet["trace_query_decision"]["full_signed_trace_architecture"],
        "deferred"
    );
    let raw = serde_json::to_string(&packet).expect("serialize packet");
    assert!(!raw.contains(root.to_string_lossy().as_ref()));
}

#[test]
fn inspect_specific_cycle_and_rejects_unsafe_cycle_refs() {
    let root = temp_dir("inspect-specific");
    let spec = write_spec(&root);
    run(
        &spec,
        RunOptions {
            max_cycles: 2,
            interval_secs: None,
            no_sleep: true,
            recover_stale_lease: false,
        },
    )
    .expect("run");

    let packet = inspect(
        &spec,
        InspectOptions {
            cycle_id: Some("cycle-000001".to_string()),
        },
    )
    .expect("inspect selected cycle");

    assert_eq!(packet["selected_cycle"]["cycle_id"], "cycle-000001");
    assert_eq!(
        packet["selected_cycle"]["refs"]["run_ref"],
        "cycles/cycle-000001/run_ref.json"
    );
    let err = inspect(
        &spec,
        InspectOptions {
            cycle_id: Some("../cycle-000001".to_string()),
        },
    )
    .expect_err("unsafe cycle ref rejected");
    assert!(err.to_string().contains("generated cycle id"));
}

#[test]
fn status_recovers_latest_cycle_from_ledger_when_status_file_is_missing() {
    let root = temp_dir("ledger-restart");
    let spec = write_spec(&root);
    run(
        &spec,
        RunOptions {
            max_cycles: 2,
            interval_secs: None,
            no_sleep: true,
            recover_stale_lease: false,
        },
    )
    .expect("run");
    fs::remove_file(root.join("state/status.json")).expect("remove status to simulate restart");
    fs::remove_file(root.join("state/continuity_checkpoint.json"))
        .expect("remove checkpoint to force ledger restore");

    let recovered = status(&spec).expect("status recovers from ledger");

    assert_eq!(recovered.state, AgentStatusState::Completed);
    assert_eq!(recovered.completed_cycle_count, 2);
    assert_eq!(recovered.last_cycle_id.as_deref(), Some("cycle-000002"));
    assert_eq!(recovered.last_cycle_status.as_deref(), Some("success"));
}

#[test]
fn locked_spec_refuses_silent_revision_and_records_operator_event() {
    let root = temp_dir("spec-revision");
    let spec = write_spec(&root);
    tick(&spec, TickOptions::default()).expect("initial tick locks spec");
    let locked_before =
        fs::read_to_string(root.join("state/agent_spec.locked.json")).expect("locked spec");
    let changed = fs::read_to_string(&spec)
        .expect("read spec")
        .replace("display_name: Test Agent", "display_name: Different Agent");
    fs::write(&spec, changed).expect("write changed spec");

    let err = status(&spec).expect_err("changed spec should require revision");

    assert!(err.to_string().contains("spec_revision_required"));
    let locked_after =
        fs::read_to_string(root.join("state/agent_spec.locked.json")).expect("locked spec");
    assert_eq!(locked_after, locked_before);
    let events = fs::read_to_string(root.join("state/operator_events.jsonl")).expect("events");
    assert!(events.contains("\"event\":\"spec_revision_requested\""));
}

#[test]
fn blocked_cycle_still_writes_reviewable_artifacts_before_returning_error() {
    let root = temp_dir("blocked-cycle");
    let spec = write_spec_with_workflow_kind(&root, "unsupported_probe");

    let err = tick(&spec, TickOptions::default()).expect_err("unsupported workflow blocks");

    assert!(err.to_string().contains("cycle_blocked"));
    for path in required_cycle_files(&root, "cycle-000001") {
        assert!(path.exists(), "missing {}", path.display());
    }
    let manifest: Value = serde_json::from_str(
        &fs::read_to_string(root.join("state/cycles/cycle-000001/cycle_manifest.json"))
            .expect("read manifest"),
    )
    .expect("parse manifest");
    assert_eq!(manifest["status"], "blocked");
    let guardrails: Value = serde_json::from_str(
        &fs::read_to_string(root.join("state/cycles/cycle-000001/guardrail_report.json"))
            .expect("read guardrails"),
    )
    .expect("parse guardrails");
    assert_eq!(guardrails["status"], "fail");
    assert_eq!(
        guardrail_check_result(&guardrails, "spec_policy_loaded"),
        "pass"
    );
    assert_eq!(
        guardrail_check_result(&guardrails, "artifact_sanitization"),
        "pass"
    );
    assert_eq!(
        guardrails["rejected_actions"][0],
        "unsupported_workflow_kind"
    );
    let decision: Value = serde_json::from_str(
        &fs::read_to_string(root.join("state/cycles/cycle-000001/decision_result.json"))
            .expect("read decision"),
    )
    .expect("parse decision");
    assert_eq!(decision["status"], "rejected");
}

#[test]
fn forbidden_action_guardrails_block_cycle_with_specific_rejections() {
    let root = temp_dir("forbidden-actions");
    let spec = write_spec_with_safety(&root, "demo_adapter", true, true);

    let err = tick(&spec, TickOptions::default()).expect_err("unsafe workflow blocks");

    assert!(err.to_string().contains("cycle_blocked"));
    for path in required_cycle_files(&root, "cycle-000001") {
        assert!(path.exists(), "missing {}", path.display());
    }
    let guardrails: Value = serde_json::from_str(
        &fs::read_to_string(root.join("state/cycles/cycle-000001/guardrail_report.json"))
            .expect("read guardrails"),
    )
    .expect("parse guardrails");
    assert_eq!(guardrails["status"], "fail");
    assert_eq!(
        guardrail_check_result(&guardrails, "no_broker_integration"),
        "fail"
    );
    assert_eq!(
        guardrail_check_result(&guardrails, "not_financial_advice"),
        "fail"
    );
    assert_eq!(
        guardrail_check_result(&guardrails, "artifact_sanitization"),
        "pass"
    );
    assert_eq!(guardrails["rejected_actions"][0], "connect_broker");
    assert_eq!(guardrails["rejected_actions"][1], "personalized_advice");
}

#[test]
fn stock_league_execute_order_request_is_rejected_as_paper_only() {
    let root = temp_dir("stock-illegal-order");
    let spec = write_spec_with_safety_and_run_args(
            &root,
            "demo_adapter",
            false,
            false,
            "    provider_id: local_ollama\n    model: gemma4:latest\n    requested_action: execute_order\n",
        );

    let err = tick(&spec, TickOptions::default()).expect_err("execute_order blocks");

    assert!(err.to_string().contains("cycle_blocked"));
    let guardrails: Value = serde_json::from_str(
        &fs::read_to_string(root.join("state/cycles/cycle-000001/guardrail_report.json"))
            .expect("read guardrails"),
    )
    .expect("parse guardrails");
    assert_eq!(guardrails["status"], "fail");
    assert_eq!(
        guardrail_check_result(&guardrails, "no_forbidden_action"),
        "fail"
    );
    assert_eq!(
        guardrail_check_result(&guardrails, "no_real_trading"),
        "fail"
    );
    assert_eq!(
        guardrail_check_result(&guardrails, "paper_only_ledger"),
        "fail"
    );
    assert_eq!(guardrails["rejected_actions"][0], "execute_order");
}

#[test]
fn sanitizer_blocks_public_artifact_host_path_leakage() {
    let root = temp_dir("sanitize-host-path");
    let spec = write_spec_with_safety_and_run_args(
        &root,
        "demo_adapter",
        false,
        false,
        "    provider_id: local_ollama\n    model: /Users/daniel/private-model\n",
    );

    let err = tick(&spec, TickOptions::default()).expect_err("sanitizer blocks");

    assert!(err.to_string().contains("cycle_blocked"));
    let guardrails: Value = serde_json::from_str(
        &fs::read_to_string(root.join("state/cycles/cycle-000001/guardrail_report.json"))
            .expect("read guardrails"),
    )
    .expect("parse guardrails");
    assert_eq!(
        guardrail_check_result(&guardrails, "artifact_sanitization"),
        "fail"
    );
    assert_eq!(guardrails["rejected_actions"][0], "artifact_sanitization");
}

#[test]
fn consecutive_failure_threshold_requests_supervisor_stop() {
    let root = temp_dir("consecutive-failures");
    let spec = write_spec_with_workflow_kind(&root, "unsupported_probe");

    let stopped = run(
        &spec,
        RunOptions {
            max_cycles: 3,
            interval_secs: None,
            no_sleep: true,
            recover_stale_lease: false,
        },
    )
    .expect("run stops after threshold");

    assert_eq!(stopped.state, AgentStatusState::Stopped);
    assert_eq!(stopped.completed_cycle_count, 2);
    assert_eq!(stopped.consecutive_failure_count, 2);
    assert!(root.join("state/stop.json").exists());
    assert!(!root.join("state/cycles/cycle-000003").exists());
    let ledger = fs::read_to_string(root.join("state/cycle_ledger.jsonl")).expect("read ledger");
    assert_eq!(ledger.lines().count(), 2);
    let events = fs::read_to_string(root.join("state/operator_events.jsonl")).expect("events");
    assert!(events.contains("\"event\":\"max_consecutive_failures\""));
}

#[test]
fn active_lease_blocks_overlapping_tick_and_status_reports_leased() {
    let root = temp_dir("active-lease");
    let spec = write_spec(&root);
    let loaded = load_spec(&spec).expect("load");
    ensure_state_root(&loaded).expect("state");
    let now = Utc::now();
    let lease = LeaseRecord {
        schema: LEASE_SCHEMA.to_string(),
        agent_instance_id: "test-agent".to_string(),
        lease_id: "lease-test-agent-000001".to_string(),
        cycle_id: "cycle-000001".to_string(),
        owner_pid: 999,
        hostname: "local".to_string(),
        started_at: now,
        expires_at: now + ChronoDuration::seconds(60),
        status: "active".to_string(),
    };
    write_json_pretty(&root.join("state/lease.json"), &lease).expect("lease");

    let err = tick(&spec, TickOptions::default()).expect_err("active lease should block");
    assert!(err.to_string().contains("lease_active"));
    let status = status(&spec).expect("status");
    assert_eq!(status.state, AgentStatusState::Leased);
    assert!(status.active_lease.is_some());
}

#[test]
fn running_status_artifact_is_reviewable_with_active_lease_context() {
    let root = temp_dir("running-status");
    let spec = write_spec(&root);
    let loaded = load_spec(&spec).expect("load");
    ensure_state_root(&loaded).expect("state");
    let now = Utc::now();
    let lease = LeaseRecord {
        schema: LEASE_SCHEMA.to_string(),
        agent_instance_id: "test-agent".to_string(),
        lease_id: "lease-test-agent-000001".to_string(),
        cycle_id: "cycle-000001".to_string(),
        owner_pid: 999,
        hostname: "local".to_string(),
        started_at: now,
        expires_at: now + ChronoDuration::seconds(60),
        status: "active".to_string(),
    };
    let running = status_with_state(
        &loaded,
        AgentStatusState::RunningCycle,
        None,
        None,
        Some(lease),
        false,
        None,
    );

    write_status(&loaded, &running).expect("write running status");
    let persisted = read_status(&loaded)
        .expect("read running status")
        .expect("status exists");

    assert_eq!(persisted.state, AgentStatusState::RunningCycle);
    assert_eq!(
        persisted.active_lease.as_ref().expect("lease").cycle_id,
        "cycle-000001"
    );
    assert_eq!(persisted.completed_cycle_count, 0);
}

#[test]
fn stale_lease_requires_recovery_then_allows_tick() {
    let root = temp_dir("stale-lease");
    let spec = write_spec(&root);
    let loaded = load_spec(&spec).expect("load");
    ensure_state_root(&loaded).expect("state");
    let now = Utc::now();
    let lease = LeaseRecord {
        schema: LEASE_SCHEMA.to_string(),
        agent_instance_id: "test-agent".to_string(),
        lease_id: "lease-test-agent-000001".to_string(),
        cycle_id: "cycle-000001".to_string(),
        owner_pid: 999,
        hostname: "local".to_string(),
        started_at: now - ChronoDuration::seconds(120),
        expires_at: now - ChronoDuration::seconds(60),
        status: "active".to_string(),
    };
    write_json_pretty(&root.join("state/lease.json"), &lease).expect("lease");

    let err = tick(&spec, TickOptions::default()).expect_err("stale lease should block");
    assert!(err.to_string().contains("lease_stale"));
    let blocked_status = status(&spec).expect("blocked status");
    assert_eq!(blocked_status.state, AgentStatusState::Failed);
    assert_eq!(
        blocked_status
            .last_error
            .as_ref()
            .expect("stale lease error")
            .class,
        "lease_stale"
    );
    let recovered = tick(
        &spec,
        TickOptions {
            recover_stale_lease: true,
        },
    )
    .expect("recovered tick");
    assert_eq!(recovered.state, AgentStatusState::Idle);
    assert_eq!(recovered.completed_cycle_count, 1);
    let events = fs::read_to_string(root.join("state/operator_events.jsonl")).expect("events");
    assert!(events.contains("\"event\":\"stale_lease_recovered\""));
}

#[test]
fn stop_prevents_next_tick_and_records_reason() {
    let root = temp_dir("stop");
    let spec = write_spec(&root);

    let stopped = stop(&spec, "operator requested pause").expect("stop");
    assert_eq!(stopped.state, AgentStatusState::Stopped);
    let after_tick = tick(&spec, TickOptions::default()).expect("tick sees stop");
    assert_eq!(after_tick.state, AgentStatusState::Stopped);
    assert_eq!(after_tick.completed_cycle_count, 0);
    assert!(after_tick
        .last_error
        .as_ref()
        .expect("error")
        .message
        .contains("operator requested pause"));
    let stop_record: Value =
        serde_json::from_str(&fs::read_to_string(root.join("state/stop.json")).unwrap())
            .expect("parse stop");
    assert_eq!(stop_record["schema"], STOP_SCHEMA);
    assert_eq!(stop_record["requested_by"], "operator");
    assert_eq!(stop_record["mode"], STOP_MODE_BEFORE_NEXT_CYCLE);
    let events = fs::read_to_string(root.join("state/operator_events.jsonl")).expect("events");
    assert!(events.contains("\"event\":\"operator_stop_requested\""));
}

#[test]
fn loom_duplicate_activation_allows_only_one_cycle_start() {
    loom::model(|| {
        use loom::sync::{Arc, Mutex};
        use loom::thread;

        struct CoordinationModel {
            stop_requested: bool,
            lease_state: CoordinationLeaseState,
            visible_state: AgentStatusState,
        }

        impl CoordinationModel {
            fn new() -> Self {
                Self {
                    stop_requested: false,
                    lease_state: CoordinationLeaseState::Clear,
                    visible_state: AgentStatusState::Idle,
                }
            }

            fn try_start_cycle(&mut self) -> ActivationDecision {
                let decision = activation_decision(self.stop_requested, self.lease_state, false);
                if decision == ActivationDecision::Start {
                    self.lease_state = CoordinationLeaseState::Active;
                    self.visible_state = AgentStatusState::RunningCycle;
                }
                decision
            }

            fn snapshot(&self) -> AgentStatusState {
                derive_visible_status_state(
                    self.visible_state.clone(),
                    self.stop_requested,
                    self.lease_state,
                )
            }
        }

        let model = Arc::new(Mutex::new(CoordinationModel::new()));
        let left_model = Arc::clone(&model);
        let right_model = Arc::clone(&model);

        let left = thread::spawn(move || {
            left_model
                .lock()
                .expect("coordination lock")
                .try_start_cycle()
        });
        let right = thread::spawn(move || {
            right_model
                .lock()
                .expect("coordination lock")
                .try_start_cycle()
        });

        let left_result = left.join().expect("left join");
        let right_result = right.join().expect("right join");
        let started = [left_result, right_result]
            .into_iter()
            .filter(|result| *result == ActivationDecision::Start)
            .count();
        let leased = [left_result, right_result]
            .into_iter()
            .filter(|result| *result == ActivationDecision::LeaseActive)
            .count();

        assert_eq!(started, 1, "duplicate activation must yield one starter");
        assert_eq!(
            leased, 1,
            "duplicate activation must yield one lease denial"
        );
        assert_eq!(
            model.lock().expect("coordination lock").snapshot(),
            AgentStatusState::Leased
        );
    });
}

#[test]
fn loom_stop_request_wins_over_concurrent_activation_and_follow_up_status() {
    loom::model(|| {
        use loom::sync::{Arc, Mutex};
        use loom::thread;

        struct CoordinationModel {
            stop_requested: bool,
            lease_state: CoordinationLeaseState,
            visible_state: AgentStatusState,
        }

        impl CoordinationModel {
            fn new() -> Self {
                Self {
                    stop_requested: false,
                    lease_state: CoordinationLeaseState::Clear,
                    visible_state: AgentStatusState::Idle,
                }
            }

            fn try_start_cycle(&mut self) -> ActivationDecision {
                let decision = activation_decision(self.stop_requested, self.lease_state, false);
                if decision == ActivationDecision::Start {
                    self.lease_state = CoordinationLeaseState::Active;
                    self.visible_state = AgentStatusState::RunningCycle;
                }
                decision
            }

            fn request_stop(&mut self) {
                self.stop_requested = true;
            }

            fn snapshot(&self) -> AgentStatusState {
                derive_visible_status_state(
                    self.visible_state.clone(),
                    self.stop_requested,
                    self.lease_state,
                )
            }
        }

        let model = Arc::new(Mutex::new(CoordinationModel::new()));
        let activation_model = Arc::clone(&model);
        let stop_model = Arc::clone(&model);
        let observed_model = Arc::clone(&model);

        let activation = thread::spawn(move || {
            activation_model
                .lock()
                .expect("coordination lock")
                .try_start_cycle()
        });
        let stop =
            thread::spawn(move || stop_model.lock().expect("coordination lock").request_stop());
        let observed =
            thread::spawn(move || observed_model.lock().expect("coordination lock").snapshot());

        let activation_result = activation.join().expect("activation join");
        stop.join().expect("stop join");
        let observed_state = observed.join().expect("observed join");
        let final_state = model.lock().expect("coordination lock").snapshot();

        assert!(
            matches!(
                observed_state,
                AgentStatusState::Idle | AgentStatusState::Leased | AgentStatusState::Stopped
            ),
            "status observations may see any truthful in-flight state"
        );
        assert!(
            matches!(
                activation_result,
                ActivationDecision::Start | ActivationDecision::StopRequested
            ),
            "activation must either win the race or observe the stop boundary"
        );
        assert_eq!(
            final_state,
            AgentStatusState::Stopped,
            "once stop wins, follow-up status must settle on stopped"
        );
    });
}
