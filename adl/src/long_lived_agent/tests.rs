//! Integration tests for long-lived agent execution and artifact invariants.
use super::*;
use crate::observability::test_env_lock;
use crate::runtime_aws_signal::mock_signal_artifact_path;
use adl_runtime::determinism::verify_retained_cycle_record;
use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;
use ed25519_dalek::{Signer, SigningKey};
use std::env;
use std::ffi::OsString;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::MutexGuard;
use std::time::{Duration, Instant};

static TEMP_SEQ: AtomicU64 = AtomicU64::new(0);

#[test]
fn governed_stop_requires_spec_bound_ed25519_authorization() {
    let signing_key = SigningKey::from_bytes(&[7_u8; 32]);
    let state_root = temp_dir("signed-stop-policy");
    let operator = env::var("USER")
        .or_else(|_| env::var("USERNAME"))
        .expect("test process OS identity");
    let loaded = LoadedAgentSpec {
        spec: AgentSpec {
            schema: SPEC_SCHEMA.to_string(),
            agent_instance_id: "signed-stop-agent".to_string(),
            display_name: "Signed stop agent".to_string(),
            state_root: state_root.clone(),
            workflow: WorkflowSpec {
                kind: "sequential".to_string(),
                name: None,
                path: None,
                run_args: Value::Null,
            },
            heartbeat: HeartbeatSpec {
                interval_secs: Some(1),
                max_cycles: Some(1),
                stale_lease_after_secs: Some(60),
            },
            checkpoint: AgentCheckpointSpec::default(),
            safety: json!({
                "governed_stop_authority": {
                    "public_key_b64": BASE64.encode(signing_key.verifying_key().to_bytes()),
                    "operators": [operator]
                }
            }),
            memory: Value::Null,
        },
        spec_path: PathBuf::from("agent.yaml"),
        state_root,
    };
    let request = GovernedStopRequest {
        reason: "test signed stop".to_string(),
        operator_identity: operator,
        authorization: String::new(),
        intent: "recoverability_drill".to_string(),
        requested_at: chrono::Utc::now(),
    };
    let payload = governed_stop_authorization_payload(&loaded.spec.agent_instance_id, &request);
    let signature = signing_key.sign(payload.as_bytes());
    let authorized = GovernedStopRequest {
        authorization: BASE64.encode(signature.to_bytes()),
        ..request.clone()
    };
    assert!(validate_governed_stop_request(&loaded, &authorized).is_ok());
    consume_governed_stop_authorization(&loaded, &authorized).unwrap();
    assert!(consume_governed_stop_authorization(&loaded, &authorized).is_err());
    let forged = GovernedStopRequest {
        authorization: BASE64.encode(signing_key.sign(b"forged").to_bytes()),
        ..authorized.clone()
    };
    assert!(validate_governed_stop_request(&loaded, &forged).is_err());

    let wrong_operator = GovernedStopRequest {
        operator_identity: "unlisted-operator".to_string(),
        ..authorized.clone()
    };
    assert!(validate_governed_stop_request(&loaded, &wrong_operator).is_err());

    let mismatch_operator = "different-os-identity".to_string();
    let mismatch_unsigned = GovernedStopRequest {
        operator_identity: mismatch_operator.clone(),
        authorization: String::new(),
        ..authorized
    };
    let mismatch_payload =
        governed_stop_authorization_payload(&loaded.spec.agent_instance_id, &mismatch_unsigned);
    let mismatch_request = GovernedStopRequest {
        authorization: BASE64.encode(signing_key.sign(mismatch_payload.as_bytes()).to_bytes()),
        ..mismatch_unsigned
    };
    let mut mismatch_loaded = loaded.clone();
    mismatch_loaded.spec.safety = json!({
        "governed_stop_authority": {
            "public_key_b64": BASE64.encode(signing_key.verifying_key().to_bytes()),
            "operators": [mismatch_operator]
        }
    });
    assert!(validate_governed_stop_request(&mismatch_loaded, &mismatch_request).is_err());
}

#[test]
fn observability_replay_preserves_every_typed_priority_label() {
    let cases = [
        (
            ChannelPriority::LowPriorityObservability,
            "low_priority_observability",
        ),
        (ChannelPriority::Audit, "audit"),
        (ChannelPriority::Evidence, "evidence"),
        (ChannelPriority::GovernedExecution, "governed_execution"),
        (ChannelPriority::CriticalContinuity, "critical_continuity"),
        (ChannelPriority::ControlPlane, "control_plane"),
    ];
    for (priority, expected) in cases {
        assert_eq!(observability_priority_label(priority), expected);
    }
}

fn temp_dir(prefix: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "adl-long-lived-agent-{prefix}-{}-{}",
        std::process::id(),
        TEMP_SEQ.fetch_add(1, Ordering::Relaxed)
    ));
    // A rerun can reuse a process id and sequence after a prior test process;
    // start from a clean fixture so cycle numbering is deterministic.
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("create temp dir");
    dir
}

fn issue_5169_scratch_dir(prefix: &str) -> PathBuf {
    let dir = std::env::current_dir()
        .expect("current dir")
        .join(".adl/scratch/issue-5169")
        .join(format!(
            "{prefix}-{}-{}",
            std::process::id(),
            TEMP_SEQ.fetch_add(1, Ordering::Relaxed)
        ));
    fs::create_dir_all(&dir).expect("create issue 5169 scratch dir");
    dir
}

fn wait_for_json_state(path: &Path, pointer: &str, expected: &str) -> Value {
    let deadline = Instant::now() + Duration::from_secs(20);
    loop {
        if let Ok(raw) = fs::read_to_string(path) {
            if let Ok(value) = serde_json::from_str::<Value>(&raw) {
                if value.pointer(pointer).and_then(Value::as_str) == Some(expected) {
                    return value;
                }
            }
        }
        assert!(
            Instant::now() < deadline,
            "timed out waiting for {} {}={expected}",
            path.display(),
            pointer
        );
        thread::sleep(Duration::from_millis(25));
    }
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
        "determinism_boundary.json",
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

fn record_notice_for_http_failure(
    prefix: &str,
    response_status: u16,
    response_delay: Duration,
    timeout_ms: u64,
) -> Value {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("bind failure route");
    let address = listener.local_addr().expect("failure route address");
    let server = tiny_http::Server::from_listener(listener, None).expect("failure route server");
    let receiver = thread::spawn(move || {
        let request = server.recv().expect("receive failure notice");
        thread::sleep(response_delay);
        let _ = request.respond(tiny_http::Response::empty(response_status));
    });
    let endpoint = format!("http://{address}/{prefix}");
    let timeout_ms = timeout_ms.to_string();
    let _env = MultiEnvGuard::set_all(&[
        ("ADL_CSM_NOTICE_REQUIRED_CHANNEL", "control_plane"),
        ("ADL_CSM_NOTICE_CONTROL_PLANE_MODE", "live"),
        ("ADL_CSM_NOTICE_CONTROL_PLANE_APPROVED", "1"),
        ("ADL_CSM_NOTICE_CONTROL_PLANE_TARGET", "https"),
        ("ADL_CSM_NOTICE_CONTROL_PLANE_URL", endpoint.as_str()),
        ("ADL_AWS_SIGNAL_MODE", "mock"),
        ("ADL_AWS_HEARTBEAT_TARGET", "cloudwatch_logs"),
        (
            "ADL_AWS_SNS_TOPIC_ARN",
            "arn:aws:sns:us-west-2:000000000000:unselected-route",
        ),
        ("ADL_CSM_NOTICE_HTTP_TIMEOUT_MS", timeout_ms.as_str()),
        ("ADL_CSM_DISK_FLOOR_BYTES", "0"),
        ("ADL_CSM_TEST_AVAILABLE_BYTES", "1073741824"),
    ]);
    let root = temp_dir(prefix);
    let spec = write_spec(&root);
    let loaded = load_spec(&spec).expect("load spec");
    ensure_state_root(&loaded).expect("state root");
    let runtime_context = CsmRuntimeContext::new(&loaded).expect("csm runtime context");
    let status = status_with_state(
        &loaded,
        AgentStatusState::Failed,
        None,
        None,
        None,
        false,
        None,
    );
    record_governed_runtime_notice(
        &runtime_context,
        &loaded,
        GovernedNoticeInput {
            notice_kind: "runtime_degraded",
            severity: "critical",
            trigger: prefix,
            status: &status,
            restart_count: 0,
            bounded_test_restart_limit: None,
            last_child_exit: None,
            safe_fail: json!({"status": "serialized"}),
            details: json!({"proof": prefix}),
        },
    )
    .expect("retain failed notice");
    receiver.join().expect("failure route join");
    let notice = read_json_required(&csm_notice_latest_path(&loaded))
        .expect("latest failed governed notice");
    let channel_state: Value =
        read_json_required(&loaded.state_root.join("csm_typed_channel_state.json"))
            .expect("failed typed channel state");
    assert_eq!(channel_state["summary"]["durable_spool_depth"], 1);
    assert!(!loaded
        .state_root
        .join("aws_csm_governed_notice_sns_mock.jsonl")
        .exists());
    notice
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

    let cycle_dir = root.join("state/cycles/cycle-000001");
    let boundary: CsmCycleDeterminismBoundaryRecord = serde_json::from_str(
        &fs::read_to_string(cycle_dir.join("determinism_boundary.json"))
            .expect("read determinism boundary"),
    )
    .expect("parse determinism boundary");
    assert_eq!(boundary.cycle_id, "cycle-000001");
    assert_eq!(boundary.decisions.len(), 5);
    assert_eq!(boundary.decision_requests.len(), 5);
    boundary.replay().expect("replay retained cycle boundary");
    let decision_components = boundary
        .decisions
        .iter()
        .map(|decision| decision.component)
        .collect::<Vec<_>>();
    assert!(decision_components.contains(&DeterministicCoreComponent::SchedulerAdmission));
    assert!(decision_components.contains(&DeterministicCoreComponent::ReasoningRuntime));
    assert!(decision_components.contains(&DeterministicCoreComponent::AeeGovernedExecution));
    assert!(decision_components.contains(&DeterministicCoreComponent::CheckpointVersionTransition));
    assert!(decision_components.contains(&DeterministicCoreComponent::LifelogOrdering));
    let aee_decision = boundary
        .decisions
        .iter()
        .find(|decision| decision.component == DeterministicCoreComponent::AeeGovernedExecution)
        .expect("AEE decision");
    assert!(aee_decision
        .cited_shell_events
        .iter()
        .any(|event_id| event_id.ends_with("provider_model_io-initial")));
    let reasoning_decision = boundary
        .decisions
        .iter()
        .find(|decision| decision.component == DeterministicCoreComponent::ReasoningRuntime)
        .expect("reasoning decision");
    assert!(reasoning_decision
        .cited_shell_events
        .iter()
        .any(|event_id| event_id.ends_with("provider_model_io-result")));
    for shell_class in NondeterministicShellClass::ALL {
        assert!(
            boundary
                .captured_shell_events
                .iter()
                .any(|event| event.shell_class == shell_class),
            "missing retained {} event",
            shell_class.as_str()
        );
    }
    assert!(boundary
        .captured_shell_events
        .iter()
        .all(|event| !event.payload.is_null()));
    assert!(boundary.captured_shell_events.iter().any(|event| {
        event.shell_class == NondeterministicShellClass::WallClock
            && event.payload["phase"] == "completion"
    }));
    assert!(!cycle_dir.join("shell_inputs").exists());
    let determinism_artifacts = fs::read_dir(&cycle_dir)
        .expect("read cycle directory")
        .filter_map(Result::ok)
        .filter_map(|entry| entry.file_name().into_string().ok())
        .filter(|name| {
            name.contains("determinism")
                || name.starts_with("core_decision_")
                || name.starts_with("nondeterminism_quarantine_")
        })
        .collect::<Vec<_>>();
    assert_eq!(determinism_artifacts, vec!["determinism_boundary.json"]);
    assert_eq!(
        manifest["artifacts"]["determinism_boundary"],
        "determinism_boundary.json"
    );
    assert_eq!(
        manifest["csm_runtime"]["determinism_boundary"],
        "typed_capture_and_fail_closed_quarantine"
    );
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
fn assembled_cycle_fail_closes_tampered_and_uncaptured_shell_influence() {
    let root = temp_dir("determinism-boundary-quarantine");
    let spec = write_spec(&root);
    tick(&spec, TickOptions::default()).expect("tick creates retained shell evidence");

    let cycle_dir = root.join("state/cycles/cycle-000001");
    let boundary_path = cycle_dir.join("determinism_boundary.json");
    let boundary_bytes = fs::read(&boundary_path).expect("read determinism boundary");
    let boundary: CsmCycleDeterminismBoundaryRecord =
        serde_json::from_slice(&boundary_bytes).expect("parse determinism boundary");
    let expected_fingerprint = cycle_record_fingerprint(&boundary).expect("boundary fingerprint");
    verify_retained_cycle_record(&boundary, &expected_fingerprint)
        .expect("verify retained cycle boundary");

    let provider_index = boundary
        .captured_shell_events
        .iter()
        .rposition(|event| event.shell_class == NondeterministicShellClass::ProviderModelIo)
        .expect("provider result event");

    let mut payload_tampered = boundary.clone();
    payload_tampered.captured_shell_events[provider_index].payload["status"] = json!("tampered");
    assert!(payload_tampered.replay().is_err());

    let old_event = &boundary.captured_shell_events[provider_index];
    let replacement = CapturedShellInputEvent::new(
        old_event.event_id.clone(),
        old_event.shell_class,
        old_event.source.clone(),
        old_event.observed_time.clone(),
        old_event.confidence,
        old_event.retention_location.clone(),
        json!({"schema": "adl.csm.provider_result_summary.v1", "status": "tampered"}),
    )
    .expect("create coordinated tamper event");
    let mut coordinated_tamper = boundary.clone();
    coordinated_tamper.captured_shell_events[provider_index] = replacement.clone();
    for request in &mut coordinated_tamper.decision_requests {
        for input in &mut request.inputs {
            if let CoreDecisionInput::CapturedShell {
                event_id,
                value_fingerprint,
                ..
            } = input
            {
                if event_id == &replacement.event_id {
                    *value_fingerprint = replacement.value_fingerprint.clone();
                }
            }
        }
    }
    assert!(verify_retained_cycle_record(&coordinated_tamper, &expected_fingerprint).is_err());

    let mut uncaptured = boundary.clone();
    let aee_request = uncaptured
        .decision_requests
        .iter_mut()
        .find(|request| request.component == DeterministicCoreComponent::AeeGovernedExecution)
        .expect("AEE request");
    aee_request.inputs.push(CoreDecisionInput::CapturedShell {
        shell_class: NondeterministicShellClass::AwsCloud,
        event_id: "cycle-000001-aws-cloud-not-retained".to_string(),
        value_fingerprint: old_event.value_fingerprint.clone(),
    });
    assert!(uncaptured.replay().is_err());
    assert_eq!(fs::read(&boundary_path).unwrap(), boundary_bytes);
}

#[test]
fn pre_execution_admission_blocks_unsupported_workflow_before_provider_work() {
    let root = temp_dir("determinism-pre-execution-admission");
    let spec = write_spec_with_workflow_kind(&root, "unsupported_external_workflow");

    let error = tick(&spec, TickOptions::default()).expect_err("unsupported workflow is blocked");
    assert!(error.to_string().contains("cycle_blocked"));
    let cycle_dir = root.join("state/cycles/cycle-000001");
    assert!(!cycle_dir.join("csm_adl_run_status.json").exists());
    let boundary: CsmCycleDeterminismBoundaryRecord = serde_json::from_slice(
        &fs::read(cycle_dir.join("determinism_boundary.json")).expect("read boundary"),
    )
    .expect("parse boundary");
    let aee_request = boundary
        .decision_requests
        .iter()
        .find(|request| request.component == DeterministicCoreComponent::AeeGovernedExecution)
        .expect("AEE request");
    assert!(aee_request.inputs.iter().any(|input| matches!(
        input,
        CoreDecisionInput::Deterministic {
            kind: DeterministicCoreInputKind::PolicyDecision,
            value
        } if value == "deny"
    )));
}

#[test]
fn provider_result_summary_does_not_retain_sensitive_fields() {
    let raw = json!({
        "status": "success",
        "workflow_kind": "adl_workflow",
        "secret": "do-not-retain",
        "token": "token-do-not-retain",
        "prompt": "prompt-do-not-retain",
        "output": "output-do-not-retain",
        "tool_output": {"authorization": "also-do-not-retain"},
        "trace": {"events": [{"sensitive": "value"}]}
    });
    let summary = safe_provider_result_summary(&raw);
    let retained = serde_json::to_string(&summary).expect("serialize summary");
    assert_eq!(summary["status"], "success");
    assert_eq!(summary["trace_event_count"], 1);
    assert!(!retained.contains("do-not-retain"));
    assert!(!retained.contains("token-do-not-retain"));
    assert!(!retained.contains("prompt-do-not-retain"));
    assert!(!retained.contains("output-do-not-retain"));
    assert!(!retained.contains("authorization"));
}

#[test]
fn status_tolerates_partial_trailing_cycle_ledger_record() {
    let root = temp_dir("partial-ledger-tail");
    let spec = write_spec(&root);
    tick(&spec, TickOptions::default()).expect("tick");

    let ledger_path = root.join("state/cycle_ledger.jsonl");
    let mut ledger = fs::OpenOptions::new()
        .append(true)
        .open(&ledger_path)
        .expect("open ledger for partial append");
    write!(
        ledger,
        "{{\"schema\":\"{}\",\"cycle_id\":\"cycle-000002\",\"status\"",
        CYCLE_LEDGER_ENTRY_SCHEMA
    )
    .expect("write partial trailing ledger record");

    let recovered = status(&spec).expect("status skips partial trailing cycle ledger record");
    assert_eq!(recovered.completed_cycle_count, 1);
    assert_eq!(recovered.last_cycle_id.as_deref(), Some("cycle-000001"));
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
    let _env = MultiEnvGuard::set_all(&[
        ("ADL_CSM_DISK_FLOOR_BYTES", "0"),
        ("ADL_CSM_TEST_AVAILABLE_BYTES", "1073741824"),
    ]);
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
fn status_refuses_checkpoint_recovery_when_godel_chain_is_corrupt() {
    let _env = MultiEnvGuard::set_all(&[
        ("ADL_CSM_DISK_FLOOR_BYTES", "0"),
        ("ADL_CSM_TEST_AVAILABLE_BYTES", "1073741824"),
    ]);
    let root = temp_dir("godel-corrupt-recovery");
    let spec = write_spec(&root);
    tick(&spec, TickOptions::default()).expect("tick writes checkpoint and Godel chain");
    fs::remove_file(root.join("state/status.json")).expect("remove status to force recovery");
    let chain_path = root
        .join("state")
        .join("godel_snapshots/godel_agent_snapshot_chain.json");
    let mut chain: Value =
        serde_json::from_str(&fs::read_to_string(&chain_path).expect("read chain"))
            .expect("parse chain");
    chain["chain_length"] = json!(99);
    fs::write(
        &chain_path,
        serde_json::to_vec_pretty(&chain).expect("encode chain"),
    )
    .expect("corrupt chain");

    let err = status(&spec).expect_err("corrupt Godel chain must block checkpoint recovery");

    assert!(err
        .to_string()
        .contains("Godel last-known-good pointer did not validate"));
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
fn freedom_gate_denial_blocks_adl_workflow_before_executor() {
    let root = temp_dir("freedom-gate-denied-adl-workflow");
    let workflow = root.join("workflow.adl.yaml");
    fs::write(
        &workflow,
        r#"version: "0.3"
providers: {}
agents: {}
tasks: {}
run:
  name: denied-before-executor
  workflow:
    kind: "sequential"
    steps: []
"#,
    )
    .expect("write workflow");
    let spec = root.join("agent.yaml");
    fs::write(
        &spec,
        r#"schema: adl.long_lived_agent_spec.v1
agent_instance_id: test-agent
display_name: Test Agent
state_root: state
workflow:
  kind: adl_workflow
  name: denied_before_executor
  path: workflow.adl.yaml
  run_args:
    provider_id: local_ollama
    model: gemma4:latest
    freedom_gate_policy_decision: denied
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
  namespace: tests/test-agent
  write_policy: append_only
"#,
    )
    .expect("write spec");

    let err = tick(&spec, TickOptions::default()).expect_err("freedom gate denial blocks");

    assert!(err.to_string().contains("cycle_blocked"));
    let cycle_dir = root.join("state/cycles/cycle-000001");
    let decision: Value = serde_json::from_str(
        &fs::read_to_string(cycle_dir.join("freedom_gate_decision.json"))
            .expect("read freedom gate decision"),
    )
    .expect("parse freedom gate decision");
    assert_eq!(decision["decision"], "denied");
    assert_eq!(decision["reason_code"], "policy_denied");
    assert_eq!(decision["stopped_before_executor"], true);
    assert_eq!(decision["executor_invocation_ref"], Value::Null);
    assert!(
        !cycle_dir.join("adl_runtime").exists(),
        "Freedom Gate denial must stop before ADL executor creates runtime artifacts"
    );
    let guardrails: Value = serde_json::from_str(
        &fs::read_to_string(cycle_dir.join("guardrail_report.json")).expect("read guardrails"),
    )
    .expect("parse guardrails");
    assert!(guardrails["rejected_actions"]
        .as_array()
        .unwrap()
        .contains(&json!("freedom_gate_policy_denied")));
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
fn daemon_partial_checkpoint_preserves_recoverable_failure_reason() {
    let _env = MultiEnvGuard::set_all(&[
        ("ADL_CSM_DISK_FLOOR_BYTES", "0"),
        ("ADL_CSM_TEST_AVAILABLE_BYTES", "1073741824"),
    ]);
    let root = temp_dir("daemon-partial-failure-reason");
    let spec = write_spec_with_workflow_kind(&root, "unsupported_adapter");
    let loaded = load_spec(&spec).expect("load spec");
    ensure_state_root(&loaded).expect("state root");
    let runtime_context = CsmRuntimeContext::new(&loaded).expect("csm runtime context");
    let error = StatusError {
        class: "daemon_child_failed".to_string(),
        message: "cycle failed before restart".to_string(),
    };
    let failed = status_with_state(
        &loaded,
        AgentStatusState::Failed,
        None,
        None,
        None,
        false,
        Some(error.clone()),
    );
    persist_status(&loaded, &failed, "daemon_child_failed_recoverable").expect("persist failed");
    let mut daemon_status = write_daemon_status(
        &runtime_context,
        &loaded,
        DaemonStatusInput {
            state: "restarting",
            bounded_test_mode: true,
            restart_count: 1,
            bounded_test_restart_limit: Some(1),
            checkpoint_interval_secs: 1,
            last_event: "restart_scheduled",
            last_child_exit: Some("error:cycle failed".to_string()),
            next_backoff_secs: 0,
        },
    )
    .expect("daemon status");

    let stop_observed = sleep_with_partial_checkpoints(
        &runtime_context,
        &loaded,
        &mut daemon_status,
        PartialCheckpointSleep {
            total_sleep_secs: 0,
            checkpoint_interval_secs: 1,
            restart_count: 1,
            bounded_test_restart_limit: Some(1),
            last_child_exit: Some("error:cycle failed".to_string()),
            recoverable_error: Some(error),
            event: "restart_backoff",
            no_sleep: true,
        },
    )
    .expect("partial checkpoint");
    assert!(!stop_observed);

    let status = read_status(&loaded)
        .expect("read status")
        .expect("status exists");
    assert_eq!(status.state, AgentStatusState::Failed);
    assert_eq!(
        status.last_error.as_ref().map(|error| error.class.as_str()),
        Some("daemon_child_failed")
    );
    let checkpoint: serde_json::Value =
        read_json_required(&continuity_checkpoint_path(&loaded)).expect("checkpoint");
    assert_eq!(checkpoint["state"], "failed");
}

#[test]
fn live_daemon_recovers_storage_and_runtime_api_without_process_restart() {
    let _env = MultiEnvGuard::set_all(&[
        ("ADL_CSM_DISK_FLOOR_BYTES", "4096"),
        ("ADL_CSM_TEST_AVAILABLE_BYTES", "1024"),
        ("ADL_AWS_SIGNAL_MODE", "disabled"),
    ]);
    let root = issue_5169_scratch_dir("same-process");
    let spec = write_spec(&root);
    let daemon_spec = spec.clone();
    let daemon_thread = thread::spawn(move || {
        daemon(
            &daemon_spec,
            DaemonOptions {
                bounded_test_restart_limit: Some(3),
                checkpoint_interval_secs: 1,
                interval_secs: Some(3),
                api_bind: None,
                no_sleep: false,
                recover_stale_lease: true,
                api_otel_status_path: None,
                api_otel_log_path: None,
            },
        )
    });
    let state_root = root.join("state");
    let pressure_path = state_root.join("csm_backpressure_state.json");
    let low_disk = wait_for_json_state(&pressure_path, "/storage_pressure/state", "low_disk");
    assert!(low_disk["updated_at"].is_string());
    let daemon_before =
        wait_for_json_state(&state_root.join("daemon_status.json"), "/state", "running");
    let process_before = daemon_before["supervisor_pid"].clone();
    let checkpoint_path = state_root.join("continuity_checkpoint.json");
    assert!(
        !checkpoint_path.exists(),
        "low disk must not manufacture a continuity checkpoint"
    );

    unsafe {
        env::set_var("ADL_CSM_TEST_AVAILABLE_BYTES", "1073745920");
    }
    let recovered = wait_for_json_state(&pressure_path, "/storage_pressure/state", "recovered");
    let checkpoint_after = wait_for_json_state(
        &checkpoint_path,
        "/schema",
        "adl.long_lived_agent_continuity_checkpoint.v1",
    );
    let daemon_after =
        wait_for_json_state(&state_root.join("daemon_status.json"), "/state", "running");
    assert_eq!(daemon_after["supervisor_pid"], process_before);
    assert_eq!(process_before, std::process::id());
    assert!(recovered["storage_pressure"]["low_disk_captured_at"].is_string());
    assert!(checkpoint_after["captured_at"].is_string());

    let api_options = crate::csm_runtime_api::CsmRuntimeApiOptions {
        spec_path: spec.clone(),
        bind: "127.0.0.1:19969".to_string(),
        test_max_requests: Some(1),
        idle_timeout_ms: None,
        shutdown_file: None,
        otel_status_path: None,
        otel_log_path: None,
    };
    let status = crate::csm_runtime_api::runtime_api_response(&api_options, "/status")
        .expect("status response");
    let health = crate::csm_runtime_api::runtime_api_response(&api_options, "/health")
        .expect("health response");
    let ready = crate::csm_runtime_api::runtime_api_response(&api_options, "/ready")
        .expect("ready response");
    let metrics = crate::csm_runtime_api::runtime_api_response(&api_options, "/metrics")
        .expect("metrics response");
    assert_eq!(
        status["backpressure"]["storage_pressure"]["state"],
        "recovered"
    );
    assert_eq!(
        health["backpressure"]["storage_pressure"]["state"],
        "recovered"
    );
    assert!(!ready["blocking_reasons"]
        .as_array()
        .expect("blocking reasons")
        .contains(&json!("storage_low_disk")));
    assert_eq!(metrics["states"]["storage_pressure"], "recovered");
    assert_eq!(metrics["states"]["backpressure_health"], "healthy");

    let events = fs::read_to_string(state_root.join("operator_events.jsonl"))
        .expect("retained operator events");
    assert!(events.contains("\"event\":\"low_disk_preflight\""));
    assert!(events.contains("\"event\":\"storage_recovered\""));
    assert!(events.contains("\"event_name\":\"storage_recovered\""));

    write_json_pretty(
        &stop_path(&load_spec(&spec).expect("load spec for stop")),
        &StopRecord {
            schema: "adl.long_lived_agent_stop.v1".to_string(),
            agent_instance_id: "test-agent".to_string(),
            reason: "integrated_storage_recovery_proven".to_string(),
            requested_by: "issue-5169-test".to_string(),
            classification: "operator_stop_requested".to_string(),
            mode: "stop_before_next_cycle".to_string(),
            requested_at: Utc::now(),
        },
    )
    .expect("request daemon stop");
    daemon_thread
        .join()
        .expect("daemon thread join")
        .expect("daemon stop cleanly");
}

#[test]
fn governed_notice_blocks_before_cloud_sequence_reservation_without_publishable_route() {
    let _env = MultiEnvGuard::set_all(&[
        ("ADL_CSM_NOTICE_REQUIRED_CHANNEL", "eventbridge"),
        ("ADL_CSM_DISK_FLOOR_BYTES", "0"),
        ("ADL_CSM_TEST_AVAILABLE_BYTES", "1073741824"),
    ]);
    let root = temp_dir("governed-notice-preflight-blocked");
    let spec = write_spec(&root);
    let loaded = load_spec(&spec).expect("load spec");
    ensure_state_root(&loaded).expect("state root");
    let runtime_context = CsmRuntimeContext::new(&loaded).expect("csm runtime context");
    let status = status_with_state(
        &loaded,
        AgentStatusState::Failed,
        None,
        None,
        None,
        false,
        Some(StatusError {
            class: "cloud_route_unavailable".to_string(),
            message: "publish route is not configured".to_string(),
        }),
    );

    record_governed_runtime_notice(
        &runtime_context,
        &loaded,
        GovernedNoticeInput {
            notice_kind: "runtime_degraded",
            severity: "critical",
            trigger: "cloud_route_unavailable",
            status: &status,
            restart_count: 0,
            bounded_test_restart_limit: None,
            last_child_exit: None,
            safe_fail: json!({"status": "serialized"}),
            details: json!({"authorization_policy": {"decision": "allow"}}),
        },
    )
    .expect("record blocked notice");
    let notice: Value =
        read_json_required(&csm_notice_latest_path(&loaded)).expect("latest governed notice");

    assert_eq!(notice["publish_preflight"]["status"], "blocked");
    assert_eq!(
        notice["typed_channel_delivery"]["status"],
        "blocked_before_sequence_reservation"
    );
    assert_eq!(
        notice["typed_channel_delivery"]["spool_sequence"],
        Value::Null
    );
    assert_eq!(notice["typed_channel_delivery"]["cursor_advanced"], false);
    assert_eq!(
        notice["publish_transaction"]["status"],
        "blocked_before_sequence_reservation"
    );
    let channel_state: Value =
        read_json_required(&loaded.state_root.join("csm_typed_channel_state.json"))
            .expect("typed channel state");
    assert_eq!(channel_state["summary"]["durable_spool_depth"], 0);
}

#[test]
fn governed_notice_rejects_secret_material_before_any_durable_payload_write() {
    let _env = MultiEnvGuard::set_all(&[
        ("ADL_CSM_NOTICE_REQUIRED_CHANNEL", "eventbridge"),
        ("ADL_CSM_DISK_FLOOR_BYTES", "0"),
        ("ADL_CSM_TEST_AVAILABLE_BYTES", "1073741824"),
    ]);
    let root = temp_dir("governed-notice-redaction-rejected");
    let spec = write_spec(&root);
    let loaded = load_spec(&spec).expect("load spec");
    ensure_state_root(&loaded).expect("state root");
    let runtime_context = CsmRuntimeContext::new(&loaded).expect("csm runtime context");
    let status = status_with_state(
        &loaded,
        AgentStatusState::Failed,
        None,
        None,
        None,
        false,
        None,
    );
    let forbidden = "Bearer must-never-be-retained-5115";
    record_governed_runtime_notice(
        &runtime_context,
        &loaded,
        GovernedNoticeInput {
            notice_kind: "runtime_degraded",
            severity: "critical",
            trigger: "redaction_rejection_proof",
            status: &status,
            restart_count: 0,
            bounded_test_restart_limit: None,
            last_child_exit: None,
            safe_fail: json!({"status": "serialized"}),
            details: json!({"authorization": forbidden}),
        },
    )
    .expect("record redacted rejection");

    let latest = fs::read_to_string(csm_notice_latest_path(&loaded)).expect("latest notice");
    let ledger = fs::read_to_string(csm_notice_ledger_path(&loaded)).expect("notice ledger");
    assert!(!latest.contains(forbidden));
    assert!(!ledger.contains(forbidden));
    let notice: Value = serde_json::from_str(&latest).expect("parse latest notice");
    assert_eq!(notice["redaction"]["status"], "rejected_before_persistence");
    assert_eq!(notice["redaction"]["retained_payload"], false);
    assert_eq!(notice["publish_preflight"]["status"], "blocked");
    assert_eq!(
        notice["publish_preflight"]["failure_class"],
        "csm_notice_redaction_failed"
    );
    let channel_state: Value =
        read_json_required(&loaded.state_root.join("csm_typed_channel_state.json"))
            .expect("typed channel state");
    assert_eq!(channel_state["summary"]["durable_spool_depth"], 0);
}

#[test]
fn cloud_publish_acknowledgement_requires_the_preflight_selected_route() {
    let attempts = vec![
        json!({
            "channel": "cloudwatch_logs",
            "status": "published_live",
            "provider_message_id": "cloudwatch-receipt"
        }),
        json!({
            "channel": "cloudfront_control_plane",
            "status": "blocked",
            "provider_message_id": Value::Null
        }),
    ];
    assert!(verified_route_attempt(&attempts, "cloudfront_control_plane").is_none());
    assert_eq!(
        verified_route_attempt(&attempts, "cloudwatch_logs")
            .and_then(|attempt| attempt["provider_message_id"].as_str()),
        Some("cloudwatch-receipt")
    );
}

#[test]
fn governed_notice_advances_once_after_selected_live_route_confirms_publication() {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("bind receipt server");
    let address = listener.local_addr().expect("receipt server address");
    let server = tiny_http::Server::from_listener(listener, None).expect("receipt server");
    let receiver = thread::spawn(move || {
        let request = server.recv().expect("receive governed notice");
        let idempotency_key = request
            .headers()
            .iter()
            .find(|header| header.field.equiv("Idempotency-Key"))
            .map(|header| header.value.as_str().to_string())
            .expect("idempotency key");
        let response = tiny_http::Response::empty(202).with_header(
            tiny_http::Header::from_bytes("x-request-id", "receipt-5115").expect("receipt header"),
        );
        request.respond(response).expect("respond to notice");
        idempotency_key
    });
    let endpoint = format!("http://{address}/polis/runtime-events");
    let _env = MultiEnvGuard::set_all(&[
        ("ADL_CSM_NOTICE_REQUIRED_CHANNEL", "control_plane"),
        ("ADL_CSM_NOTICE_CONTROL_PLANE_MODE", "live"),
        ("ADL_CSM_NOTICE_CONTROL_PLANE_APPROVED", "1"),
        ("ADL_CSM_NOTICE_CONTROL_PLANE_TARGET", "https"),
        ("ADL_CSM_NOTICE_CONTROL_PLANE_URL", endpoint.as_str()),
        ("ADL_AWS_SIGNAL_MODE", "mock"),
        ("ADL_AWS_HEARTBEAT_TARGET", "cloudwatch_logs"),
        (
            "ADL_AWS_SNS_TOPIC_ARN",
            "arn:aws:sns:us-west-2:000000000000:unselected-route",
        ),
        ("ADL_CSM_DISK_FLOOR_BYTES", "0"),
        ("ADL_CSM_TEST_AVAILABLE_BYTES", "1073741824"),
    ]);
    let root = temp_dir("governed-notice-live-receipt");
    let spec = write_spec(&root);
    let loaded = load_spec(&spec).expect("load spec");
    ensure_state_root(&loaded).expect("state root");
    let runtime_context = CsmRuntimeContext::new(&loaded).expect("csm runtime context");
    let status = status_with_state(
        &loaded,
        AgentStatusState::Failed,
        None,
        None,
        None,
        false,
        None,
    );

    record_governed_runtime_notice(
        &runtime_context,
        &loaded,
        GovernedNoticeInput {
            notice_kind: "runtime_degraded",
            severity: "critical",
            trigger: "live_route_receipt_proof",
            status: &status,
            restart_count: 0,
            bounded_test_restart_limit: None,
            last_child_exit: None,
            safe_fail: json!({"status": "serialized"}),
            details: json!({"proof": "selected_route_receipt"}),
        },
    )
    .expect("publish governed notice");
    let received_idempotency_key = receiver.join().expect("receipt server join");
    let notice: Value =
        read_json_required(&csm_notice_latest_path(&loaded)).expect("latest governed notice");

    assert_eq!(notice["publish_preflight"]["status"], "publishable");
    assert_eq!(
        notice["publish_preflight"]["idempotency_key"],
        received_idempotency_key
    );
    assert_eq!(
        notice["typed_channel_delivery"]["status"],
        "published_and_atomically_acknowledged"
    );
    assert_eq!(
        notice["typed_channel_delivery"]["provider_receipt_id"],
        "receipt-5115"
    );
    assert_eq!(notice["typed_channel_delivery"]["cursor_advanced"], true);
    assert_eq!(
        notice["publish_transaction"]["status"],
        "published_and_atomically_acknowledged"
    );
    let channel_state: Value =
        read_json_required(&loaded.state_root.join("csm_typed_channel_state.json"))
            .expect("typed channel state");
    assert_eq!(channel_state["summary"]["durable_spool_depth"], 0);
}

#[test]
fn governed_notice_retains_spool_and_cursor_when_selected_live_route_is_unreachable() {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("reserve unreachable port");
    let address = listener.local_addr().expect("unreachable address");
    drop(listener);
    let endpoint = format!("http://{address}/unreachable");
    let _env = MultiEnvGuard::set_all(&[
        ("ADL_CSM_NOTICE_REQUIRED_CHANNEL", "control_plane"),
        ("ADL_CSM_NOTICE_CONTROL_PLANE_MODE", "live"),
        ("ADL_CSM_NOTICE_CONTROL_PLANE_APPROVED", "1"),
        ("ADL_CSM_NOTICE_CONTROL_PLANE_TARGET", "https"),
        ("ADL_CSM_NOTICE_CONTROL_PLANE_URL", endpoint.as_str()),
        ("ADL_AWS_SIGNAL_MODE", "mock"),
        ("ADL_AWS_HEARTBEAT_TARGET", "cloudwatch_logs"),
        (
            "ADL_AWS_SNS_TOPIC_ARN",
            "arn:aws:sns:us-west-2:000000000000:unselected-route",
        ),
        ("ADL_CSM_DISK_FLOOR_BYTES", "0"),
        ("ADL_CSM_TEST_AVAILABLE_BYTES", "1073741824"),
    ]);
    let root = temp_dir("governed-notice-unreachable");
    let spec = write_spec(&root);
    let loaded = load_spec(&spec).expect("load spec");
    ensure_state_root(&loaded).expect("state root");
    let runtime_context = CsmRuntimeContext::new(&loaded).expect("csm runtime context");
    let status = status_with_state(
        &loaded,
        AgentStatusState::Failed,
        None,
        None,
        None,
        false,
        None,
    );

    record_governed_runtime_notice(
        &runtime_context,
        &loaded,
        GovernedNoticeInput {
            notice_kind: "runtime_degraded",
            severity: "critical",
            trigger: "unreachable_route_proof",
            status: &status,
            restart_count: 0,
            bounded_test_restart_limit: None,
            last_child_exit: None,
            safe_fail: json!({"status": "serialized"}),
            details: json!({"proof": "unreachable_selected_route"}),
        },
    )
    .expect("retain unreachable notice");
    let notice: Value =
        read_json_required(&csm_notice_latest_path(&loaded)).expect("latest governed notice");

    assert_eq!(notice["publish_preflight"]["status"], "publishable");
    assert_eq!(
        notice["typed_channel_delivery"]["status"],
        "durably_spooled_waiting_for_verified_transport_receipt"
    );
    assert_eq!(notice["typed_channel_delivery"]["cursor_advanced"], false);
    assert_eq!(
        notice["publish_transaction"]["status"],
        "durably_spooled_waiting_for_verified_transport_receipt"
    );
    assert!(notice["typed_channel_delivery"]["spool_sequence"].is_number());
    assert!(notice["delivery_attempts"]
        .as_array()
        .expect("delivery attempts")
        .iter()
        .any(|attempt| {
            attempt["channel"] == "cloudfront_control_plane"
                && attempt["status"] == "failed"
                && attempt["failure_class"] == "control_plane_http_unreachable"
        }));
    let attempted_channels: Vec<_> = notice["delivery_attempts"]
        .as_array()
        .expect("delivery attempts")
        .iter()
        .filter_map(|attempt| attempt["channel"].as_str())
        .collect();
    assert_eq!(
        attempted_channels,
        vec!["local_notice_ledger", "cloudfront_control_plane"]
    );
    let channel_state: Value =
        read_json_required(&loaded.state_root.join("csm_typed_channel_state.json"))
            .expect("typed channel state");
    assert_eq!(channel_state["summary"]["durable_spool_depth"], 1);
    assert!(!loaded
        .state_root
        .join("aws_csm_governed_notice_sns_mock.jsonl")
        .exists());

    unsafe {
        env::set_var(
            "ADL_CSM_NOTICE_CONTROL_PLANE_URL",
            "http://127.0.0.1:1/configuration-changed",
        );
    }
    let mismatched = drain_pending_cloud_notices(&runtime_context, &loaded, None)
        .expect("fail closed on changed route contract");
    assert_eq!(mismatched.acknowledged_count, 0);
    let mismatched_channel_state: Value =
        read_json_required(&loaded.state_root.join("csm_typed_channel_state.json"))
            .expect("mismatched typed channel state");
    assert_eq!(
        mismatched_channel_state["summary"]["durable_spool_depth"],
        1
    );
    assert_eq!(
        mismatched_channel_state["last_receipt"]["status"],
        "durably_spooled_route_contract_mismatch"
    );
    assert_eq!(
        mismatched_channel_state["last_receipt"]["cursor_advanced"],
        false
    );
    unsafe {
        env::set_var("ADL_CSM_NOTICE_CONTROL_PLANE_URL", endpoint.as_str());
    }

    let listener = std::net::TcpListener::bind(address).expect("restore selected route");
    let server = tiny_http::Server::from_listener(listener, None).expect("recovery server");
    let receiver = thread::spawn(move || {
        let request = server.recv().expect("receive replayed notice");
        let idempotency_key = request
            .headers()
            .iter()
            .find(|header| header.field.equiv("Idempotency-Key"))
            .map(|header| header.value.as_str().to_string())
            .expect("replay idempotency key");
        request
            .respond(
                tiny_http::Response::empty(202).with_header(
                    tiny_http::Header::from_bytes("x-request-id", "recovery-receipt-5115")
                        .expect("recovery receipt header"),
                ),
            )
            .expect("respond to replay");
        idempotency_key
    });
    let recovered = drain_pending_cloud_notices(&runtime_context, &loaded, None)
        .expect("replay retained notice");
    let replay_idempotency_key = receiver.join().expect("recovery server join");
    assert_eq!(recovered.acknowledged_count, 1);
    assert_eq!(
        notice["publish_preflight"]["idempotency_key"],
        replay_idempotency_key
    );
    let duplicate_drain =
        drain_pending_cloud_notices(&runtime_context, &loaded, None).expect("second replay drain");
    assert_eq!(duplicate_drain.acknowledged_count, 0);
    let recovered_channel_state: Value =
        read_json_required(&loaded.state_root.join("csm_typed_channel_state.json"))
            .expect("recovered typed channel state");
    assert_eq!(recovered_channel_state["summary"]["durable_spool_depth"], 0);
}

#[test]
fn governed_notice_retains_spool_and_cursor_when_selected_route_denies_access() {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("bind denied route");
    let address = listener.local_addr().expect("denied route address");
    let server = tiny_http::Server::from_listener(listener, None).expect("denied route server");
    let receiver = thread::spawn(move || {
        let request = server.recv().expect("receive denied notice");
        request
            .respond(tiny_http::Response::empty(403))
            .expect("deny notice");
    });
    let endpoint = format!("http://{address}/denied");
    let _env = MultiEnvGuard::set_all(&[
        ("ADL_CSM_NOTICE_REQUIRED_CHANNEL", "control_plane"),
        ("ADL_CSM_NOTICE_CONTROL_PLANE_MODE", "live"),
        ("ADL_CSM_NOTICE_CONTROL_PLANE_APPROVED", "1"),
        ("ADL_CSM_NOTICE_CONTROL_PLANE_TARGET", "https"),
        ("ADL_CSM_NOTICE_CONTROL_PLANE_URL", endpoint.as_str()),
        ("ADL_CSM_DISK_FLOOR_BYTES", "0"),
        ("ADL_CSM_TEST_AVAILABLE_BYTES", "1073741824"),
    ]);
    let root = temp_dir("governed-notice-denied");
    let spec = write_spec(&root);
    let loaded = load_spec(&spec).expect("load spec");
    ensure_state_root(&loaded).expect("state root");
    let runtime_context = CsmRuntimeContext::new(&loaded).expect("csm runtime context");
    let status = status_with_state(
        &loaded,
        AgentStatusState::Failed,
        None,
        None,
        None,
        false,
        None,
    );

    record_governed_runtime_notice(
        &runtime_context,
        &loaded,
        GovernedNoticeInput {
            notice_kind: "runtime_degraded",
            severity: "critical",
            trigger: "denied_route_proof",
            status: &status,
            restart_count: 0,
            bounded_test_restart_limit: None,
            last_child_exit: None,
            safe_fail: json!({"status": "serialized"}),
            details: json!({"proof": "authorization_denial"}),
        },
    )
    .expect("retain denied notice");
    receiver.join().expect("denied route join");
    let notice: Value =
        read_json_required(&csm_notice_latest_path(&loaded)).expect("latest governed notice");
    assert_eq!(
        notice["typed_channel_delivery"]["status"],
        "durably_spooled_waiting_for_verified_transport_receipt"
    );
    assert_eq!(notice["typed_channel_delivery"]["cursor_advanced"], false);
    assert!(notice["delivery_attempts"]
        .as_array()
        .expect("delivery attempts")
        .iter()
        .any(|attempt| {
            attempt["channel"] == "cloudfront_control_plane"
                && attempt["status"] == "failed"
                && attempt["failure_class"] == "control_plane_http_access_denied_403"
        }));
    let channel_state: Value =
        read_json_required(&loaded.state_root.join("csm_typed_channel_state.json"))
            .expect("typed channel state");
    assert_eq!(channel_state["summary"]["durable_spool_depth"], 1);
}

#[test]
fn governed_notice_retains_spool_and_cursor_when_selected_route_throttles() {
    let notice =
        record_notice_for_http_failure("governed-notice-throttled", 429, Duration::ZERO, 1_000);
    assert_eq!(
        notice["typed_channel_delivery"]["status"],
        "durably_spooled_waiting_for_verified_transport_receipt"
    );
    assert_eq!(notice["typed_channel_delivery"]["cursor_advanced"], false);
    assert!(notice["delivery_attempts"]
        .as_array()
        .expect("delivery attempts")
        .iter()
        .any(|attempt| attempt["failure_class"] == "control_plane_http_throttled_429"));
}

#[test]
fn governed_notice_retains_spool_and_cursor_for_ambiguous_timeout() {
    let notice = record_notice_for_http_failure(
        "governed-notice-timeout-ambiguous",
        202,
        Duration::from_millis(100),
        10,
    );
    assert_eq!(
        notice["typed_channel_delivery"]["status"],
        "durably_spooled_waiting_for_verified_transport_receipt"
    );
    assert_eq!(notice["typed_channel_delivery"]["cursor_advanced"], false);
    assert!(notice["delivery_attempts"]
        .as_array()
        .expect("delivery attempts")
        .iter()
        .any(|attempt| attempt["failure_class"] == "control_plane_http_timeout_ambiguous"));
}

#[test]
fn safe_fail_bundle_preserves_malformed_artifacts_and_quarantines_active_lease() {
    let _env = MultiEnvGuard::set_all(&[
        ("ADL_CSM_DISK_FLOOR_BYTES", "0"),
        ("ADL_CSM_TEST_AVAILABLE_BYTES", "1073741824"),
    ]);
    let root = temp_dir("safe-fail-malformed-quarantine");
    let spec = write_spec(&root);
    let loaded = load_spec(&spec).expect("load spec");
    ensure_state_root(&loaded).expect("state root");
    let runtime_context = CsmRuntimeContext::new(&loaded).expect("csm runtime context");
    let lease = LeaseRecord {
        schema: LEASE_SCHEMA.to_string(),
        agent_instance_id: loaded.spec.agent_instance_id.clone(),
        lease_id: "lease-test".to_string(),
        cycle_id: "cycle-000001".to_string(),
        owner_pid: 123,
        hostname: "test-host".to_string(),
        started_at: Utc::now(),
        expires_at: Utc::now() + ChronoDuration::seconds(60),
        status: "active".to_string(),
    };
    let status = status_with_state(
        &loaded,
        AgentStatusState::Failed,
        Some("cycle-000001".to_string()),
        Some("failed".to_string()),
        Some(lease),
        false,
        Some(StatusError {
            class: "daemon_child_failed".to_string(),
            message: "failed with active lease".to_string(),
        }),
    );
    persist_status(&loaded, &status, "daemon_child_failed_recoverable")
        .expect("persist failed status");
    fs::write(continuity_replay_manifest_path(&loaded), "{")
        .expect("write malformed replay manifest");

    let summary = record_safe_fail_bundle(
        &runtime_context,
        &loaded,
        &SafeFailRecord {
            status: &status,
            trigger: "daemon_child_failed",
            restart_count: 0,
            bounded_test_restart_limit: Some(1),
            last_child_exit: Some("error:failed".to_string()),
            details: json!({"test_case": "malformed_replay_manifest"}),
        },
    )
    .expect("record safe fail");
    assert_eq!(summary["status"], "serialized");

    let bundle: serde_json::Value =
        read_json_required(&safe_fail_bundle_path(&loaded)).expect("safe fail bundle");
    assert_eq!(bundle["agent_outcome"]["state"], "quarantined");
    assert_eq!(bundle["recoverability"]["class"], "quarantine_required");
    assert_eq!(
        bundle["serialized_state"]["continuity_replay_manifest"]["status"],
        "unreadable"
    );
    assert_eq!(
        bundle["monotonicity"]["does_not_rewrite_cycle_ledger"],
        true
    );
    assert!(bundle["negative_case_boundaries"]
        .as_array()
        .expect("negative boundaries")
        .iter()
        .any(|boundary| boundary
            == "malformed_prior_state_is_retained_as_unreadable_artifact_evidence"));
}

#[test]
fn safe_fail_bundle_suppresses_sequence_artifact_under_low_disk() {
    let _env = MultiEnvGuard::set_all(&[
        ("ADL_CSM_DISK_FLOOR_BYTES", "4096"),
        ("ADL_CSM_TEST_AVAILABLE_BYTES", "1024"),
    ]);
    let root = temp_dir("safe-fail-low-disk");
    let spec = write_spec(&root);
    let loaded = load_spec(&spec).expect("load spec");
    ensure_state_root(&loaded).expect("state root");
    let runtime_context = CsmRuntimeContext::new(&loaded).expect("csm runtime context");
    let status = status_with_state(
        &loaded,
        AgentStatusState::Failed,
        Some("cycle-000001".to_string()),
        Some("failed".to_string()),
        None,
        false,
        Some(StatusError {
            class: "daemon_child_failed".to_string(),
            message: "failed under low disk".to_string(),
        }),
    );
    persist_status(&loaded, &status, "daemon_child_failed_recoverable")
        .expect("persist failed status");

    let summary = record_safe_fail_bundle(
        &runtime_context,
        &loaded,
        &SafeFailRecord {
            status: &status,
            trigger: "daemon_child_failed",
            restart_count: 0,
            bounded_test_restart_limit: None,
            last_child_exit: Some("error:failed".to_string()),
            details: json!({"test_case": "low_disk"}),
        },
    )
    .expect("record low-disk safe fail");

    assert_eq!(summary["status"], "serialized_degraded");
    assert_eq!(summary["storage_pressure"], "low_disk");
    assert_eq!(summary["sequence_ref"], Value::Null);
    assert!(safe_fail_bundle_path(&loaded).exists());
    assert!(!safe_fail_artifacts_dir(&loaded)
        .join("safe-fail-000001.json")
        .exists());

    let bundle = read_json_required(&safe_fail_bundle_path(&loaded)).expect("safe fail bundle");
    assert_eq!(
        bundle["monotonicity"]["policy"],
        "low_disk_latest_pointer_only_no_new_sequence_artifact"
    );
    assert_eq!(
        bundle["monotonicity"]["sequence_artifact_suppressed"],
        "storage_low_disk"
    );
}

#[test]
fn continuity_checkpoint_low_disk_does_not_advance_godel_chain() {
    let root = temp_dir("godel-low-disk");
    let spec = write_spec(&root);
    let loaded = load_spec(&spec).expect("load spec");
    ensure_state_root(&loaded).expect("state root");
    match fs::remove_dir_all(root.join("state/godel_snapshots")) {
        Ok(()) => {}
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {}
        Err(err) => panic!("remove setup Godel chain: {err}"),
    }
    assert!(!root.join("state/godel_snapshots").exists());
    let _env = MultiEnvGuard::set_all(&[
        ("ADL_CSM_DISK_FLOOR_BYTES", "4096"),
        ("ADL_CSM_TEST_AVAILABLE_BYTES", "1024"),
    ]);
    let status = status_with_state(
        &loaded,
        AgentStatusState::Idle,
        Some("cycle-000001".to_string()),
        Some("success".to_string()),
        None,
        false,
        None,
    );

    let retained = write_continuity_restore_artifacts(&loaded, &status, "low_disk_checkpoint")
        .expect("low disk degrades without advancing chain");

    assert!(!retained);
    assert!(!root.join("state/godel_snapshots").exists());
    assert!(root
        .join("state/csm_low_disk_recovery_manifest.json")
        .exists());
}

#[test]
fn daemon_interval_defaults_positive_and_rejects_zero_cadence() {
    let root = temp_dir("daemon-positive-cadence");
    let spec = write_spec(&root);
    let loaded = load_spec(&spec).expect("load spec");
    assert_eq!(
        daemon_interval_secs(&loaded, None).expect("spec interval"),
        1
    );
    assert_eq!(
        daemon_interval_secs(&loaded, Some(2)).expect("override interval"),
        2
    );
    assert!(daemon_interval_secs(&loaded, Some(0))
        .expect_err("zero override must fail")
        .to_string()
        .contains("greater than zero"));

    let no_interval_spec = root.join("agent-no-interval.yaml");
    let body = fs::read_to_string(&spec).expect("read spec");
    fs::write(&no_interval_spec, body.replace("  interval_secs: 1\n", ""))
        .expect("write no-interval spec");
    let no_interval_loaded = load_spec(&no_interval_spec).expect("load no-interval spec");
    assert_eq!(
        daemon_interval_secs(&no_interval_loaded, None).expect("default interval"),
        3
    );

    let zero_interval_spec = root.join("agent-zero-interval.yaml");
    fs::write(
        &zero_interval_spec,
        fs::read_to_string(&spec)
            .expect("read spec")
            .replace("  interval_secs: 1\n", "  interval_secs: 0\n"),
    )
    .expect("write zero-interval spec");
    let zero_interval_loaded = load_spec(&zero_interval_spec).expect("load zero-interval spec");
    assert!(daemon_interval_secs(&zero_interval_loaded, None)
        .expect_err("zero spec interval must fail")
        .to_string()
        .contains("greater than zero"));
}

#[test]
fn daemon_status_records_restart_always_permanent_service_contract() {
    let root = temp_dir("daemon-restart-always-contract");
    let spec = write_spec(&root);
    let loaded = load_spec(&spec).expect("load spec");
    ensure_state_root(&loaded).expect("state root");
    let runtime_context = CsmRuntimeContext::new(&loaded).expect("csm runtime context");

    let running = write_daemon_status(
        &runtime_context,
        &loaded,
        DaemonStatusInput {
            state: "running",
            bounded_test_mode: false,
            restart_count: 0,
            bounded_test_restart_limit: None,
            checkpoint_interval_secs: 1,
            last_event: "daemon_started",
            last_child_exit: None,
            next_backoff_secs: 0,
        },
    )
    .expect("running daemon status");

    assert_eq!(running.restart_policy, "always");
    assert_eq!(running.service_mode, "permanent");
    assert!(!running.bounded_test_mode);
    assert_eq!(
        running.runtime_capabilities["supervisor"]["restart_policy"],
        "always"
    );
    assert_eq!(
        running.runtime_capabilities["supervisor"]["lifetime_boundary"],
        "operator_stop_or_fatal_supervisor_failure_only"
    );
    assert_eq!(
        running.runtime_capabilities["freedom_gate"]["status"],
        "integrated"
    );
    assert_eq!(
        running.runtime_capabilities["freedom_gate"]["executor_requires_gate_decision"],
        true
    );
    assert!(
        loaded
            .state_root
            .join(crate::csm_freedom_gate::CSM_FREEDOM_GATE_STATUS_REF)
            .exists(),
        "daemon status write must retain CSM Freedom Gate status"
    );

    let bounded = write_daemon_status(
        &runtime_context,
        &loaded,
        DaemonStatusInput {
            state: "completed",
            bounded_test_mode: true,
            restart_count: 0,
            bounded_test_restart_limit: None,
            checkpoint_interval_secs: 1,
            last_event: "daemon_completed",
            last_child_exit: Some("success".to_string()),
            next_backoff_secs: 0,
        },
    )
    .expect("bounded daemon status");

    assert_eq!(bounded.restart_policy, "always");
    assert_eq!(bounded.service_mode, "bounded_test_only");
    assert!(bounded.bounded_test_mode);
}

#[test]
fn daemon_status_records_nonfatal_cav_snapshot_write_failure() {
    let root = temp_dir("daemon-cav-write-failure");
    let spec = write_spec(&root);
    let loaded = load_spec(&spec).expect("load spec");
    ensure_state_root(&loaded).expect("state root");
    fs::create_dir(loaded.state_root.join(csm_cav::CSM_CAV_STATUS_REF))
        .expect("block CAV status file with directory");
    let runtime_context = CsmRuntimeContext::new(&loaded).expect("csm runtime context");

    let status = write_daemon_status(
        &runtime_context,
        &loaded,
        DaemonStatusInput {
            state: "running",
            bounded_test_mode: true,
            restart_count: 0,
            bounded_test_restart_limit: None,
            checkpoint_interval_secs: 1,
            last_event: "daemon_started",
            last_child_exit: None,
            next_backoff_secs: 0,
        },
    )
    .expect("CAV snapshot failure remains nonfatal");

    assert_eq!(status.state, "running");
    let events = fs::read_to_string(operator_events_path(&loaded)).expect("operator events");
    assert!(events.contains("\"event\":\"csm_cav_status_write_failed\""));
    assert!(events.contains("\"status\":\"blocked_nonfatal\""));
}

#[test]
fn agent_checkpoint_policy_clamps_cadence_and_governs_requests() {
    let root = temp_dir("agent-checkpoint-policy");
    let spec = root.join("agent.yaml");
    fs::write(
        &spec,
        r#"schema: adl.long_lived_agent_spec.v1
agent_instance_id: test-agent
display_name: Test Agent
state_root: state
workflow:
  kind: demo_adapter
  name: wp02_heartbeat_probe
  run_args:
    provider_id: local_ollama
checkpoint:
  interval_secs: 2
  allow_agent_requested: true
  min_request_interval_secs: 5
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
  namespace: tests/test-agent
  write_policy: append_only
"#,
    )
    .expect("write spec");
    let loaded = load_spec(&spec).expect("load spec");
    ensure_state_root(&loaded).expect("state root");
    assert_eq!(
        effective_checkpoint_interval_secs(&loaded, 10).expect("effective interval"),
        2
    );
    fs::write(
        checkpoint_request_path(&loaded),
        r#"{"schema":"adl.csm.agent_checkpoint_request.v1","reason":"agent-local state changed","requested_at":"2026-07-06T00:00:00Z"}"#,
    )
    .expect("write checkpoint request");
    let runtime_context = CsmRuntimeContext::new(&loaded).expect("runtime context");
    let accepted = observe_agent_checkpoint_request(
        &runtime_context,
        &loaded,
        0,
        Utc::now() - ChronoDuration::seconds(60),
    )
    .expect("observe request")
    .expect("request outcome");
    assert_eq!(accepted["decision"], "accepted");
    assert!(!checkpoint_request_path(&loaded).exists());

    fs::write(
        checkpoint_request_path(&loaded),
        r#"{"schema":"adl.csm.agent_checkpoint_request.v1","reason":"second request","requested_at":"2026-07-06T00:00:01Z"}"#,
    )
    .expect("write second checkpoint request");
    let rate_limited = observe_agent_checkpoint_request(&runtime_context, &loaded, 0, Utc::now())
        .expect("observe rate limited")
        .expect("request outcome");
    assert_eq!(rate_limited["decision"], "blocked_rate_limited");

    for (case, body) in [
        ("malformed", "{"),
        (
            "wrong_schema",
            r#"{"schema":"adl.csm.agent_checkpoint_request.v0","reason":"state changed","requested_at":"2026-07-06T00:00:02Z"}"#,
        ),
        (
            "missing_reason",
            r#"{"schema":"adl.csm.agent_checkpoint_request.v1","requested_at":"2026-07-06T00:00:03Z"}"#,
        ),
    ] {
        fs::write(checkpoint_request_path(&loaded), body)
            .unwrap_or_else(|err| panic!("write {case} checkpoint request: {err}"));
        let blocked = observe_agent_checkpoint_request(
            &runtime_context,
            &loaded,
            0,
            Utc::now() - ChronoDuration::seconds(60),
        )
        .unwrap_or_else(|err| panic!("observe {case} checkpoint request: {err}"))
        .unwrap_or_else(|| panic!("{case} checkpoint request outcome"));
        assert_eq!(blocked["decision"], "blocked_malformed", "{case}");
        assert_eq!(blocked["request_validation"]["status"], "failed", "{case}");
        assert!(!checkpoint_request_path(&loaded).exists(), "{case}");
    }
}

#[test]
fn daemon_heartbeat_partial_checkpoint_does_not_report_backoff() {
    let root = temp_dir("daemon-heartbeat-no-backoff");
    let spec = write_spec(&root);
    let loaded = load_spec(&spec).expect("load spec");
    ensure_state_root(&loaded).expect("state root");
    let runtime_context = CsmRuntimeContext::new(&loaded).expect("csm runtime context");
    let mut daemon_status = write_daemon_status(
        &runtime_context,
        &loaded,
        DaemonStatusInput {
            state: "running",
            bounded_test_mode: false,
            restart_count: 0,
            bounded_test_restart_limit: Some(1),
            checkpoint_interval_secs: 1,
            last_event: "daemon_started",
            last_child_exit: None,
            next_backoff_secs: 0,
        },
    )
    .expect("daemon status");

    let stop_observed = sleep_with_partial_checkpoints(
        &runtime_context,
        &loaded,
        &mut daemon_status,
        PartialCheckpointSleep {
            total_sleep_secs: 1,
            checkpoint_interval_secs: 1,
            restart_count: 0,
            bounded_test_restart_limit: Some(1),
            last_child_exit: None,
            recoverable_error: None,
            event: "daemon_heartbeat",
            no_sleep: false,
        },
    )
    .expect("partial checkpoint");

    assert!(!stop_observed);
    assert_eq!(daemon_status.next_backoff_secs, 0);
}

#[test]
fn daemon_healthy_partial_checkpoint_does_not_emit_safe_fail_bundle() {
    let _env = MultiEnvGuard::set_all(&[
        ("ADL_CSM_DISK_FLOOR_BYTES", "0"),
        ("ADL_CSM_TEST_AVAILABLE_BYTES", "1073741824"),
    ]);
    let root = temp_dir("daemon-healthy-partial-no-safe-fail");
    let spec = write_spec(&root);
    let loaded = load_spec(&spec).expect("load spec");
    ensure_state_root(&loaded).expect("state root");
    let runtime_context = CsmRuntimeContext::new(&loaded).expect("csm runtime context");
    let mut daemon_status = write_daemon_status(
        &runtime_context,
        &loaded,
        DaemonStatusInput {
            state: "running",
            bounded_test_mode: true,
            restart_count: 0,
            bounded_test_restart_limit: Some(1),
            checkpoint_interval_secs: 1,
            last_event: "daemon_started",
            last_child_exit: None,
            next_backoff_secs: 0,
        },
    )
    .expect("daemon status");

    let stop_observed = sleep_with_partial_checkpoints(
        &runtime_context,
        &loaded,
        &mut daemon_status,
        PartialCheckpointSleep {
            total_sleep_secs: 0,
            checkpoint_interval_secs: 1,
            restart_count: 0,
            bounded_test_restart_limit: Some(1),
            last_child_exit: None,
            recoverable_error: None,
            event: "daemon_heartbeat",
            no_sleep: true,
        },
    )
    .expect("healthy partial checkpoint");

    assert!(!stop_observed);
    assert!(continuity_checkpoint_path(&loaded).exists());
    assert!(status_path(&loaded).exists());
    assert!(!safe_fail_bundle_path(&loaded).exists());
    assert!(!safe_fail_artifacts_dir(&loaded).exists());
    let operator_events =
        fs::read_to_string(operator_events_path(&loaded)).expect("operator events");
    assert!(!operator_events.contains("\"event\":\"safe_fail_serialization\""));
}

#[test]
fn daemon_partial_checkpoint_reports_stop_observed_before_restart_attempt() {
    let root = temp_dir("daemon-stop-during-backoff");
    let spec = write_spec(&root);
    let loaded = load_spec(&spec).expect("load spec");
    ensure_state_root(&loaded).expect("state root");
    write_stop_record(
        &loaded,
        "operator pause before restart",
        "operator",
        "operator_stop_requested",
    )
    .expect("write stop");
    let runtime_context = CsmRuntimeContext::new(&loaded).expect("csm runtime context");
    let mut daemon_status = write_daemon_status(
        &runtime_context,
        &loaded,
        DaemonStatusInput {
            state: "restarting",
            bounded_test_mode: false,
            restart_count: 1,
            bounded_test_restart_limit: Some(1),
            checkpoint_interval_secs: 1,
            last_event: "restart_scheduled",
            last_child_exit: Some("error:cycle failed".to_string()),
            next_backoff_secs: 1,
        },
    )
    .expect("daemon status");

    let stop_observed = sleep_with_partial_checkpoints(
        &runtime_context,
        &loaded,
        &mut daemon_status,
        PartialCheckpointSleep {
            total_sleep_secs: 1,
            checkpoint_interval_secs: 1,
            restart_count: 1,
            bounded_test_restart_limit: Some(1),
            last_child_exit: Some("error:cycle failed".to_string()),
            recoverable_error: None,
            event: "restart_backoff",
            no_sleep: false,
        },
    )
    .expect("partial checkpoint");
    assert!(stop_observed);

    let events = fs::read_to_string(operator_events_path(&loaded)).expect("operator events");
    assert!(events.contains("\"event\":\"graceful_shutdown_requested\""));
    assert!(!events.contains("\"event\":\"restart_attempted\""));
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
