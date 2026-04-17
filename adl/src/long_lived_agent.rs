use anyhow::{anyhow, Context, Result};
use chrono::{DateTime, Duration as ChronoDuration, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;

const SPEC_SCHEMA: &str = "adl.long_lived_agent_spec.v1";
const LEASE_SCHEMA: &str = "adl.long_lived_agent_lease.v1";
const STATUS_SCHEMA: &str = "adl.long_lived_agent_status.v1";
const STOP_SCHEMA: &str = "adl.long_lived_agent_stop.v1";
const CYCLE_MANIFEST_SCHEMA: &str = "adl.long_lived_agent_cycle_manifest.v1";
const OBSERVATIONS_SCHEMA: &str = "adl.long_lived_agent_observations.v1";
const DECISION_REQUEST_SCHEMA: &str = "adl.long_lived_agent_decision_request.v1";
const DECISION_RESULT_SCHEMA: &str = "adl.long_lived_agent_decision_result.v1";
const RUN_REF_SCHEMA: &str = "adl.long_lived_agent_run_ref.v1";
const MEMORY_WRITE_SCHEMA: &str = "adl.long_lived_agent_memory_write.v1";
const GUARDRAIL_REPORT_SCHEMA: &str = "adl.long_lived_agent_guardrail_report.v1";
const CONTINUITY_SCHEMA: &str = "adl.long_lived_agent_continuity.v1";
const CYCLE_LEDGER_ENTRY_SCHEMA: &str = "adl.long_lived_agent_cycle_ledger_entry.v1";
const PROVIDER_BINDING_SCHEMA: &str = "adl.long_lived_agent_provider_binding.v1";
const MEMORY_INDEX_SCHEMA: &str = "adl.long_lived_agent_memory_index.v1";
const OPERATOR_EVENT_SCHEMA: &str = "adl.long_lived_agent_operator_event.v1";
const INSPECTION_PACKET_SCHEMA: &str = "adl.long_lived_agent_inspection_packet.v1";
const DEFAULT_MAX_CYCLE_RUNTIME_SECS: u64 = 120;
const DEFAULT_MAX_CONSECUTIVE_FAILURES: u64 = 2;
const STOP_MODE_BEFORE_NEXT_CYCLE: &str = "stop_before_next_cycle";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentSpec {
    pub schema: String,
    pub agent_instance_id: String,
    pub display_name: String,
    pub state_root: PathBuf,
    pub workflow: WorkflowSpec,
    pub heartbeat: HeartbeatSpec,
    #[serde(default)]
    pub safety: Value,
    #[serde(default)]
    pub memory: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowSpec {
    pub kind: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub path: Option<PathBuf>,
    #[serde(default)]
    pub run_args: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeartbeatSpec {
    #[serde(default)]
    pub interval_secs: Option<u64>,
    #[serde(default)]
    pub max_cycles: Option<u64>,
    #[serde(default)]
    pub stale_lease_after_secs: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AgentStatusState {
    NotStarted,
    Idle,
    Leased,
    RunningCycle,
    Stopped,
    Failed,
    Completed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaseRecord {
    pub schema: String,
    pub agent_instance_id: String,
    pub lease_id: String,
    pub cycle_id: String,
    pub owner_pid: u32,
    pub hostname: String,
    pub started_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusError {
    pub class: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusRecord {
    pub schema: String,
    pub agent_instance_id: String,
    pub state: AgentStatusState,
    pub last_cycle_id: Option<String>,
    pub last_cycle_status: Option<String>,
    pub completed_cycle_count: u64,
    #[serde(default)]
    pub consecutive_failure_count: u64,
    pub active_lease: Option<LeaseRecord>,
    pub stop_requested: bool,
    pub last_error: Option<StatusError>,
    #[serde(default)]
    pub safety_policy: Value,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StopRecord {
    pub schema: String,
    pub agent_instance_id: String,
    pub reason: String,
    #[serde(default = "default_requested_by")]
    pub requested_by: String,
    #[serde(default = "default_stop_mode")]
    pub mode: String,
    pub requested_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct LoadedAgentSpec {
    pub spec: AgentSpec,
    pub spec_path: PathBuf,
    pub state_root: PathBuf,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct TickOptions {
    pub recover_stale_lease: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct RunOptions {
    pub max_cycles: u64,
    pub interval_secs: Option<u64>,
    pub no_sleep: bool,
    pub recover_stale_lease: bool,
}

#[derive(Debug, Clone, Default)]
pub struct InspectOptions {
    pub cycle_id: Option<String>,
}

#[derive(Debug, Clone, Default)]
struct LedgerCursor {
    latest_cycle_id: Option<String>,
    latest_status: Option<String>,
    count: u64,
    max_cycle_number: u64,
}

pub fn load_spec(spec_path: &Path) -> Result<LoadedAgentSpec> {
    let raw = fs::read_to_string(spec_path)
        .with_context(|| format!("failed reading agent spec {}", spec_path.display()))?;
    let spec: AgentSpec = serde_yaml::from_str(&raw)
        .with_context(|| format!("failed parsing agent spec {}", spec_path.display()))?;
    validate_spec(&spec)?;
    let base = spec_path.parent().unwrap_or_else(|| Path::new("."));
    let state_root = if spec.state_root.is_absolute() {
        spec.state_root.clone()
    } else {
        base.join(&spec.state_root)
    };
    Ok(LoadedAgentSpec {
        spec,
        spec_path: spec_path.to_path_buf(),
        state_root,
    })
}

pub fn tick(spec_path: &Path, options: TickOptions) -> Result<StatusRecord> {
    let loaded = load_spec(spec_path)?;
    ensure_state_root(&loaded)?;
    if let Some(stop) = read_stop(&loaded)? {
        let status = stopped_status(&loaded, stop.reason);
        write_status(&loaded, &status)?;
        return Ok(status);
    }

    let cycle_id = next_cycle_id(&loaded)?;
    let lease = acquire_lease(&loaded, &cycle_id, options.recover_stale_lease)?;
    let running = status_with_state(
        &loaded,
        AgentStatusState::RunningCycle,
        None,
        None,
        Some(lease.clone()),
        false,
        None,
    );
    write_status(&loaded, &running)?;

    let result = write_cycle_artifacts(&loaded, &cycle_id);
    remove_lease(&loaded)?;

    match result {
        Ok(()) => {
            let status = status_with_state(
                &loaded,
                AgentStatusState::Idle,
                Some(cycle_id),
                Some("success".to_string()),
                None,
                false,
                None,
            );
            write_status(&loaded, &status)?;
            Ok(status)
        }
        Err(err) => {
            let cursor = ledger_cursor(&loaded).unwrap_or_default();
            let status = status_with_state(
                &loaded,
                AgentStatusState::Failed,
                cursor.latest_cycle_id.or(Some(cycle_id)),
                cursor.latest_status.or_else(|| Some("failed".to_string())),
                None,
                false,
                Some(StatusError {
                    class: "workflow_failed".to_string(),
                    message: err.to_string(),
                }),
            );
            write_status(&loaded, &status)?;
            Err(err)
        }
    }
}

pub fn run(spec_path: &Path, options: RunOptions) -> Result<StatusRecord> {
    if options.max_cycles == 0 {
        return Err(anyhow!("agent run requires --max-cycles greater than zero"));
    }
    let loaded = load_spec(spec_path)?;
    let sleep_secs = options
        .interval_secs
        .or(loaded.spec.heartbeat.interval_secs)
        .unwrap_or(0);

    let mut last_status = status(spec_path)?;
    for index in 0..options.max_cycles {
        if read_stop(&loaded)?.is_some() {
            last_status = tick(
                spec_path,
                TickOptions {
                    recover_stale_lease: options.recover_stale_lease,
                },
            )?;
            break;
        }
        match tick(
            spec_path,
            TickOptions {
                recover_stale_lease: options.recover_stale_lease,
            },
        ) {
            Ok(status) => {
                last_status = status;
            }
            Err(err) => {
                last_status = status(spec_path)?;
                let failures = consecutive_failure_count(&loaded)?;
                if failures >= max_consecutive_failures(&loaded) {
                    last_status = write_stop_record(
                        &loaded,
                        &format!(
                            "max_consecutive_failures reached after {failures} blocked or failed cycles"
                        ),
                        "supervisor",
                        "max_consecutive_failures",
                    )?;
                    break;
                }
                if index + 1 >= options.max_cycles {
                    return Err(err);
                }
            }
        }
        if last_status.state == AgentStatusState::Stopped {
            break;
        }
        if index + 1 < options.max_cycles && !options.no_sleep && sleep_secs > 0 {
            thread::sleep(Duration::from_secs(sleep_secs));
        }
    }

    if last_status.state != AgentStatusState::Stopped {
        last_status.state = AgentStatusState::Completed;
        last_status.updated_at = Utc::now();
        write_status(&loaded, &last_status)?;
    }
    Ok(last_status)
}

pub fn status(spec_path: &Path) -> Result<StatusRecord> {
    let loaded = load_spec(spec_path)?;
    ensure_state_root(&loaded)?;
    let ledger = ledger_cursor(&loaded)?;
    let mut current = read_status(&loaded)?.unwrap_or_else(|| {
        if ledger.latest_cycle_id.is_some() {
            status_with_state(
                &loaded,
                AgentStatusState::Idle,
                ledger.latest_cycle_id.clone(),
                ledger.latest_status.clone(),
                None,
                false,
                None,
            )
        } else {
            status_with_state(
                &loaded,
                AgentStatusState::NotStarted,
                None,
                None,
                None,
                false,
                None,
            )
        }
    });
    if let Some(latest_cycle_id) = ledger.latest_cycle_id.clone() {
        current.last_cycle_id = Some(latest_cycle_id);
        current.last_cycle_status = ledger.latest_status.clone();
        if current.state == AgentStatusState::NotStarted {
            current.state = AgentStatusState::Idle;
        }
    }
    current.completed_cycle_count = completed_cycle_count(&loaded)?;

    if let Some(stop) = read_stop(&loaded)? {
        current.state = AgentStatusState::Stopped;
        current.stop_requested = true;
        current.last_error = Some(StatusError {
            class: "operator_stop_requested".to_string(),
            message: stop.reason,
        });
    } else if let Some(lease) = read_lease(&loaded)? {
        if lease_is_stale(&lease) {
            current.state = AgentStatusState::Failed;
            current.active_lease = Some(lease);
            current.last_error = Some(StatusError {
                class: "lease_stale".to_string(),
                message: "active lease is stale and requires explicit recovery".to_string(),
            });
        } else {
            current.state = AgentStatusState::Leased;
            current.active_lease = Some(lease);
            current.last_error = None;
        }
    }
    current.updated_at = Utc::now();
    write_status(&loaded, &current)?;
    Ok(current)
}

pub fn stop(spec_path: &Path, reason: &str) -> Result<StatusRecord> {
    if reason.trim().is_empty() {
        return Err(anyhow!("agent stop requires a non-empty --reason"));
    }
    let loaded = load_spec(spec_path)?;
    ensure_state_root(&loaded)?;
    write_stop_record(
        &loaded,
        reason.trim(),
        "operator",
        "operator_stop_requested",
    )
}

pub fn inspect(spec_path: &Path, options: InspectOptions) -> Result<Value> {
    let status = status(spec_path)?;
    let loaded = load_spec(spec_path)?;
    let ledger = ledger_cursor(&loaded)?;
    let selected_cycle_id = options
        .cycle_id
        .clone()
        .or_else(|| status.last_cycle_id.clone())
        .or(ledger.latest_cycle_id);
    let selected_cycle = selected_cycle_id
        .as_deref()
        .map(|cycle_id| inspect_cycle(&loaded, cycle_id))
        .transpose()?;
    let proof_status = if selected_cycle.is_some() {
        "pass"
    } else {
        "no_cycle_available"
    };

    Ok(json!({
        "schema": INSPECTION_PACKET_SCHEMA,
        "agent_instance_id": loaded.spec.agent_instance_id.clone(),
        "generated_at": Utc::now(),
        "state_root_ref": path_artifact_ref(&loaded.spec.state_root),
        "status_ref": "status.json",
        "status": status,
        "cycle_count": ledger.count,
        "selected_cycle_id": selected_cycle_id,
        "selected_cycle": selected_cycle,
        "reviewer_proof": {
            "status": proof_status,
            "status_ref": {
                "path": "status.json",
                "exists": status_path(&loaded).exists()
            },
            "cycle_ref_status": if proof_status == "pass" {
                "selected_cycle_artifacts_present"
            } else {
                "no_cycle_selected"
            }
        },
        "trace_query_decision": {
            "status": "deferred_full_platform",
            "minimal_v0_90_boundary": "inspection packet over status, cycle manifest, guardrail report, run_ref, and cycle summary artifacts",
            "full_tql_platform": "deferred",
            "full_signed_trace_architecture": "deferred",
            "reason": "WP-06 accepts the narrow reviewer inspection slice and does not widen into TQL or signed-trace architecture."
        }
    }))
}

fn validate_spec(spec: &AgentSpec) -> Result<()> {
    if spec.schema != SPEC_SCHEMA {
        return Err(anyhow!(
            "unsupported agent spec schema '{}' (expected {SPEC_SCHEMA})",
            spec.schema
        ));
    }
    if spec.agent_instance_id.trim().is_empty() {
        return Err(anyhow!("agent spec requires agent_instance_id"));
    }
    if spec.display_name.trim().is_empty() {
        return Err(anyhow!("agent spec requires display_name"));
    }
    if spec.workflow.kind.trim().is_empty() {
        return Err(anyhow!("agent spec requires workflow.kind"));
    }
    let stale = spec.heartbeat.stale_lease_after_secs.unwrap_or(900);
    if stale == 0 {
        return Err(anyhow!(
            "agent spec heartbeat.stale_lease_after_secs must be greater than zero"
        ));
    }
    if safety_u64(
        &spec.safety,
        "max_cycle_runtime_secs",
        DEFAULT_MAX_CYCLE_RUNTIME_SECS,
    ) == 0
    {
        return Err(anyhow!(
            "agent spec safety.max_cycle_runtime_secs must be greater than zero"
        ));
    }
    if safety_u64(
        &spec.safety,
        "max_consecutive_failures",
        DEFAULT_MAX_CONSECUTIVE_FAILURES,
    ) == 0
    {
        return Err(anyhow!(
            "agent spec safety.max_consecutive_failures must be greater than zero"
        ));
    }
    Ok(())
}

fn ensure_state_root(loaded: &LoadedAgentSpec) -> Result<()> {
    fs::create_dir_all(cycles_dir(loaded))
        .with_context(|| format!("failed creating {}", cycles_dir(loaded).display()))?;
    ensure_locked_spec(loaded)?;
    ensure_jsonl_file(&cycle_ledger_path(loaded))?;
    ensure_jsonl_file(&provider_binding_history_path(loaded))?;
    ensure_continuity(loaded)?;
    ensure_memory_index(loaded)?;
    if !status_path(loaded).exists() {
        let status = status_with_state(
            loaded,
            AgentStatusState::NotStarted,
            None,
            None,
            None,
            false,
            None,
        );
        write_status(loaded, &status)?;
    }
    Ok(())
}

fn ensure_locked_spec(loaded: &LoadedAgentSpec) -> Result<()> {
    let locked = locked_spec_path(loaded);
    let current = serde_json::to_value(&loaded.spec)?;
    if locked.exists() {
        let locked_value: Value = read_json_required(&locked)?;
        if locked_value != current {
            append_operator_event(
                loaded,
                "spec_revision_requested",
                json!({
                    "reason": "operator spec changed after lock creation",
                    "locked_spec_ref": "agent_spec.locked.json"
                }),
            )?;
            return Err(anyhow!(
                "spec_revision_required: {} differs from the locked continuity spec",
                loaded.spec_path.display()
            ));
        }
    } else {
        write_json_pretty(&locked, &loaded.spec)?;
        append_operator_event(
            loaded,
            "created",
            json!({
                "locked_spec_ref": "agent_spec.locked.json",
                "continuity_kind": "pre_v0_92_handle"
            }),
        )?;
    }
    Ok(())
}

fn ensure_jsonl_file(path: &Path) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed creating {}", parent.display()))?;
    }
    OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .with_context(|| format!("failed ensuring jsonl file {}", path.display()))?;
    Ok(())
}

fn ensure_continuity(loaded: &LoadedAgentSpec) -> Result<()> {
    let path = continuity_path(loaded);
    if path.exists() {
        return Ok(());
    }
    let continuity = json!({
        "schema": CONTINUITY_SCHEMA,
        "agent_instance_id": loaded.spec.agent_instance_id.clone(),
        "display_name": loaded.spec.display_name.clone(),
        "created_at": Utc::now(),
        "created_by": "operator",
        "continuity_kind": "pre_v0_92_handle",
        "status": "active",
        "state_root": path_artifact_ref(&loaded.spec.state_root),
        "memory_namespace": memory_namespace(loaded),
        "cycle_ledger_ref": "cycle_ledger.jsonl",
        "latest_cycle_id": Value::Null,
        "future_identity_ref": Value::Null,
        "non_claims": [
            "not_v0_92_identity_tuple",
            "not_capability_governance",
            "not_autonomous_legal_personhood"
        ]
    });
    write_json_pretty(&path, &continuity)
}

fn ensure_memory_index(loaded: &LoadedAgentSpec) -> Result<()> {
    let path = memory_index_path(loaded);
    if path.exists() {
        return Ok(());
    }
    let memory_index = json!({
        "schema": MEMORY_INDEX_SCHEMA,
        "memory_namespace": memory_namespace(loaded),
        "append_only": true,
        "local_memory_refs": [],
        "obsmem_export_status": "not_exported"
    });
    write_json_pretty(&path, &memory_index)
}

fn acquire_lease(
    loaded: &LoadedAgentSpec,
    cycle_id: &str,
    recover_stale_lease: bool,
) -> Result<LeaseRecord> {
    let path = lease_path(loaded);
    if let Some(existing) = read_lease(loaded)? {
        if lease_is_stale(&existing) {
            if !recover_stale_lease {
                let status = status_with_state(
                    loaded,
                    AgentStatusState::Failed,
                    None,
                    None,
                    Some(existing),
                    false,
                    Some(StatusError {
                        class: "lease_stale".to_string(),
                        message: "active lease is stale; rerun with --recover-stale-lease"
                            .to_string(),
                    }),
                );
                write_status(loaded, &status)?;
                return Err(anyhow!(
                    "lease_stale: active lease is stale; rerun with --recover-stale-lease"
                ));
            }
            append_operator_event(
                loaded,
                "stale_lease_recovered",
                json!({
                    "lease_id": existing.lease_id,
                    "stale_cycle_id": existing.cycle_id,
                    "recovered_for_cycle_id": cycle_id
                }),
            )?;
            remove_lease(loaded)?;
        } else {
            let status = status_with_state(
                loaded,
                AgentStatusState::Leased,
                None,
                None,
                Some(existing),
                false,
                Some(StatusError {
                    class: "lease_active".to_string(),
                    message: "another cycle already holds the agent lease".to_string(),
                }),
            );
            write_status(loaded, &status)?;
            return Err(anyhow!(
                "lease_active: another cycle already holds the agent lease"
            ));
        }
    }

    let now = Utc::now();
    let expires_at = now
        + ChronoDuration::seconds(
            loaded
                .spec
                .heartbeat
                .stale_lease_after_secs
                .unwrap_or(900)
                .try_into()
                .unwrap_or(i64::MAX),
        );
    let lease = LeaseRecord {
        schema: LEASE_SCHEMA.to_string(),
        agent_instance_id: loaded.spec.agent_instance_id.clone(),
        lease_id: format!(
            "lease-{}-{}",
            loaded.spec.agent_instance_id,
            cycle_id.trim_start_matches("cycle-")
        ),
        cycle_id: cycle_id.to_string(),
        owner_pid: std::process::id(),
        hostname: hostname(),
        started_at: now,
        expires_at,
        status: "active".to_string(),
    };
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&path)
        .with_context(|| format!("failed creating lease {}", path.display()))?;
    let raw = serde_json::to_vec_pretty(&lease)?;
    file.write_all(&raw)
        .with_context(|| format!("failed writing lease {}", path.display()))?;
    file.write_all(b"\n")
        .with_context(|| format!("failed finalizing lease {}", path.display()))?;
    Ok(lease)
}

fn write_cycle_artifacts(loaded: &LoadedAgentSpec, cycle_id: &str) -> Result<()> {
    let cycle_dir = cycles_dir(loaded).join(cycle_id);
    fs::create_dir_all(&cycle_dir)
        .with_context(|| format!("failed creating cycle dir {}", cycle_dir.display()))?;
    let started_at = Utc::now();
    let previous_cycle_id = latest_cycle_id(loaded)?;
    let workflow_ref = workflow_ref(&loaded.spec.workflow);
    let provider_binding = provider_binding(loaded, cycle_id, started_at);
    let safety_policy = effective_safety_policy(loaded);
    let workflow_supported = workflow_kind_supported(&loaded.spec.workflow.kind);
    let broker_allowed = safety_bool_default(&loaded.spec.safety, "allow_broker", false);
    let financial_advice_allowed =
        safety_bool_default(&loaded.spec.safety, "financial_advice", false);
    let outside_writes_allowed = safety_bool_default(
        &loaded.spec.safety,
        "allow_filesystem_writes_outside_state_root",
        false,
    );
    let real_world_side_effects_allowed =
        safety_bool_default(&loaded.spec.safety, "allow_real_world_side_effects", false);
    let require_sanitization = safety_bool_default(
        &loaded.spec.safety,
        "require_public_artifact_sanitization",
        true,
    );
    let mut rejected_actions = rejected_actions_for_policy(loaded);
    if !workflow_supported {
        rejected_actions.push("unsupported_workflow_kind".to_string());
    }
    dedup_strings(&mut rejected_actions);

    let observations = json!({
        "schema": OBSERVATIONS_SCHEMA,
        "agent_instance_id": loaded.spec.agent_instance_id.clone(),
        "cycle_id": cycle_id,
        "observed_at": started_at,
        "sources": [
            {
                "source_id": "agent_spec",
                "kind": "locked_supervisor_spec",
                "trust_level": "operator_configured",
                "artifact_ref": "../../agent_spec.locked.json"
            }
        ],
        "facts": [
            {
                "key": "workflow.kind",
                "value": loaded.spec.workflow.kind.clone(),
                "as_of": cycle_id
            },
            {
                "key": "workflow.ref",
                "value": workflow_ref.clone(),
                "as_of": cycle_id
            }
        ]
    });
    write_json_pretty(&cycle_dir.join("observations.json"), &observations)?;

    let decision_request = json!({
        "schema": DECISION_REQUEST_SCHEMA,
        "agent_instance_id": loaded.spec.agent_instance_id.clone(),
        "cycle_id": cycle_id,
        "agent_context_ref": "../../agent_spec.locked.json",
        "observations_ref": "observations.json",
        "memory_refs": [],
        "allowed_actions": ["record_cycle", "explain"],
        "forbidden_actions": ["execute_order", "connect_broker", "personalized_advice"],
        "not_financial_advice": true
    });
    write_json_pretty(&cycle_dir.join("decision_request.json"), &decision_request)?;

    let sanitization = if require_sanitization {
        sanitize_public_artifacts(&[
            ("observations.json", &observations),
            ("decision_request.json", &decision_request),
            ("provider_binding", &provider_binding),
        ])?
    } else {
        SanitizationResult::skipped()
    };
    if !sanitization.passed {
        rejected_actions.push("artifact_sanitization".to_string());
        dedup_strings(&mut rejected_actions);
    }

    let max_runtime_not_exceeded =
        Utc::now() <= started_at + ChronoDuration::seconds(max_cycle_runtime_secs(loaded) as i64);
    if !max_runtime_not_exceeded {
        rejected_actions.push("max_cycle_runtime_exceeded".to_string());
        dedup_strings(&mut rejected_actions);
    }

    let guardrail_pass = workflow_supported
        && rejected_actions.is_empty()
        && sanitization.passed
        && max_runtime_not_exceeded;
    let cycle_status = if guardrail_pass { "success" } else { "blocked" };
    let decision_status = if guardrail_pass {
        "accepted"
    } else {
        "rejected"
    };
    let guardrail_status = if guardrail_pass { "pass" } else { "fail" };

    let decision = if guardrail_pass {
        json!({
            "action": "record_cycle",
            "summary": "Bounded long-lived agent cycle completed under the v0.90 artifact contract.",
            "workflow_ref": workflow_ref,
            "paper_only": true
        })
    } else {
        json!({
            "action": "blocked",
            "summary": "Cycle blocked before workflow execution because the configured contract failed required guardrails.",
            "workflow_ref": workflow_ref,
            "paper_only": true
        })
    };
    let decision_result = json!({
        "schema": DECISION_RESULT_SCHEMA,
        "agent_instance_id": loaded.spec.agent_instance_id.clone(),
        "cycle_id": cycle_id,
        "status": decision_status,
        "decision": decision,
        "provider": {
            "source": provider_binding["provider_id"].clone(),
            "model": provider_binding["model"].clone()
        },
        "not_financial_advice": true
    });
    write_json_pretty(&cycle_dir.join("decision_result.json"), &decision_result)?;

    let run_ref = if loaded.spec.workflow.kind == "adl_workflow" {
        json!({
            "schema": RUN_REF_SCHEMA,
            "workflow_kind": "adl_workflow",
            "workflow_ref": workflow_ref,
            "run_status_ref": null,
            "trace_ref": null,
            "execution_note": "WP-03 records the cycle artifact contract; full workflow invocation remains bounded by the configured supervisor cycle."
        })
    } else {
        json!({
            "schema": RUN_REF_SCHEMA,
            "workflow_kind": loaded.spec.workflow.kind.clone(),
            "adapter": workflow_ref,
            "adapter_artifact_ref": "decision_result.json"
        })
    };
    write_json_pretty(&cycle_dir.join("run_ref.json"), &run_ref)?;

    let memory_write = json!({
        "schema": MEMORY_WRITE_SCHEMA,
        "cycle_id": cycle_id,
        "memory_id": format!("mem-{}", cycle_id.trim_start_matches("cycle-")),
        "summary": if guardrail_pass {
            "Recorded a bounded cycle artifact bundle."
        } else {
            "Recorded a blocked cycle with machine-readable guardrail evidence."
        },
        "tags": [
            format!("agent:{}", loaded.spec.agent_instance_id),
            format!("cycle:{cycle_id}"),
            "long-lived-agent",
            "paper-only"
        ],
        "source_refs": if guardrail_pass {
            json!(["decision_result.json", "cycle_manifest.json"])
        } else {
            json!(["guardrail_report.json", "cycle_manifest.json"])
        },
        "write_policy": "append_only"
    });
    write_jsonl(&cycle_dir.join("memory_writes.jsonl"), &[memory_write])?;
    update_memory_index(loaded, cycle_id)?;
    append_jsonl(&provider_binding_history_path(loaded), &provider_binding)?;

    let checks = vec![
        json!({
            "check_id": "spec_policy_loaded",
            "result": "pass",
            "policy": safety_policy
        }),
        json!({
            "check_id": "lease_valid",
            "result": "pass"
        }),
        json!({
            "check_id": "stop_not_requested",
            "result": "pass"
        }),
        json!({
            "check_id": "workflow_kind_supported",
            "result": if workflow_supported { "pass" } else { "fail" },
            "details": loaded.spec.workflow.kind.clone()
        }),
        json!({
            "check_id": "no_forbidden_action",
            "result": if rejected_actions.is_empty() { "pass" } else { "fail" },
            "rejected_actions": rejected_actions.clone()
        }),
        json!({
            "check_id": "artifact_sanitization",
            "result": if sanitization.passed { "pass" } else { "fail" },
            "findings": sanitization.findings
        }),
        json!({
            "check_id": "max_runtime_not_exceeded",
            "result": if max_runtime_not_exceeded { "pass" } else { "fail" },
            "max_cycle_runtime_secs": max_cycle_runtime_secs(loaded)
        }),
        json!({
            "check_id": "no_real_trading",
            "result": if rejected_actions.iter().any(|action| action == "execute_order" || action == "place_order") {
                "fail"
            } else {
                "pass"
            }
        }),
        json!({
            "check_id": "no_broker_integration",
            "result": if broker_allowed || rejected_actions.iter().any(|action| action == "connect_broker") {
                "fail"
            } else {
                "pass"
            }
        }),
        json!({
            "check_id": "not_financial_advice",
            "result": if financial_advice_allowed || rejected_actions.iter().any(|action| action == "personalized_advice") {
                "fail"
            } else {
                "pass"
            }
        }),
        json!({
            "check_id": "no_real_world_side_effects",
            "result": if real_world_side_effects_allowed { "fail" } else { "pass" }
        }),
        json!({
            "check_id": "writes_within_allowed_roots",
            "result": if outside_writes_allowed { "fail" } else { "pass" }
        }),
        json!({
            "check_id": "paper_only_ledger",
            "result": if rejected_actions.iter().any(|action| action == "execute_order" || action == "place_order" || action == "connect_broker") {
                "fail"
            } else {
                "pass"
            }
        }),
    ];
    let guardrail_report = json!({
        "schema": GUARDRAIL_REPORT_SCHEMA,
        "agent_instance_id": loaded.spec.agent_instance_id.clone(),
        "cycle_id": cycle_id,
        "status": guardrail_status,
        "checks": checks,
        "rejected_actions": rejected_actions.clone(),
        "policy_defaults": effective_safety_policy(loaded)
    });
    write_json_pretty(&cycle_dir.join("guardrail_report.json"), &guardrail_report)?;

    let completed_at = Utc::now();
    let manifest_input = json!({
        "observations_ref": "observations.json",
        "decision_request_ref": "decision_request.json",
        "previous_cycle_id": previous_cycle_id,
        "workflow_kind": loaded.spec.workflow.kind.clone(),
        "workflow_ref": workflow_ref
    });
    let manifest_output = json!({
        "decision_result_ref": "decision_result.json",
        "run_ref": "run_ref.json",
        "memory_writes_ref": "memory_writes.jsonl",
        "guardrail_report_ref": "guardrail_report.json",
        "status": cycle_status
    });
    let manifest = json!({
        "schema": CYCLE_MANIFEST_SCHEMA,
        "agent_instance_id": loaded.spec.agent_instance_id.clone(),
        "cycle_id": cycle_id,
        "status": cycle_status,
        "started_at": started_at,
        "completed_at": completed_at,
        "workflow_kind": loaded.spec.workflow.kind.clone(),
        "workflow_ref": manifest_input["workflow_ref"].clone(),
        "input_hash": sha256_json(&manifest_input)?,
        "output_hash": sha256_json(&manifest_output)?,
        "previous_cycle_id": manifest_input["previous_cycle_id"].clone(),
        "next_cycle_hint": "sleep_until_next_heartbeat",
        "artifacts": {
            "observations": "observations.json",
            "decision_request": "decision_request.json",
            "decision_result": "decision_result.json",
            "run_ref": "run_ref.json",
            "memory_writes": "memory_writes.jsonl",
            "guardrail_report": "guardrail_report.json",
            "cycle_summary": "cycle_summary.md"
        },
        "not_financial_advice": true
    });
    write_json_pretty(&cycle_dir.join("cycle_manifest.json"), &manifest)?;

    fs::write(
        cycle_dir.join("cycle_summary.md"),
        format!(
            "# Long-Lived Agent Cycle {cycle_id}\n\n- Agent: `{}`\n- Workflow kind: `{}`\n- Cycle status: `{cycle_status}`\n- Observations: `observations.json`\n- Decision request: `decision_request.json`\n- Decision result: `decision_result.json`\n- Guardrail result: `{guardrail_status}`\n- Memory writes: `memory_writes.jsonl`\n- Next-cycle note: `sleep_until_next_heartbeat`\n- Safety: paper-only; not financial advice; no broker execution\n",
            loaded.spec.agent_instance_id, loaded.spec.workflow.kind
        ),
    )
    .with_context(|| format!("failed writing cycle summary for {cycle_id}"))?;
    append_cycle_ledger_entry(
        loaded,
        cycle_id,
        cycle_status,
        started_at,
        completed_at,
        previous_cycle_id.as_deref(),
    )?;
    update_continuity_after_cycle(loaded, cycle_id, cycle_status)?;

    if !guardrail_pass {
        return Err(anyhow!(
            "cycle_blocked: cycle {cycle_id} failed required guardrails; see {}",
            cycle_dir.join("guardrail_report.json").display()
        ));
    }

    Ok(())
}

fn status_with_state(
    loaded: &LoadedAgentSpec,
    state: AgentStatusState,
    last_cycle_id: Option<String>,
    last_cycle_status: Option<String>,
    active_lease: Option<LeaseRecord>,
    stop_requested: bool,
    last_error: Option<StatusError>,
) -> StatusRecord {
    StatusRecord {
        schema: STATUS_SCHEMA.to_string(),
        agent_instance_id: loaded.spec.agent_instance_id.clone(),
        state,
        last_cycle_id,
        last_cycle_status,
        completed_cycle_count: completed_cycle_count(loaded).unwrap_or(0),
        consecutive_failure_count: consecutive_failure_count(loaded).unwrap_or(0),
        active_lease,
        stop_requested,
        last_error,
        safety_policy: effective_safety_policy(loaded),
        updated_at: Utc::now(),
    }
}

fn write_stop_record(
    loaded: &LoadedAgentSpec,
    reason: &str,
    requested_by: &str,
    event: &str,
) -> Result<StatusRecord> {
    let stop = StopRecord {
        schema: STOP_SCHEMA.to_string(),
        agent_instance_id: loaded.spec.agent_instance_id.clone(),
        reason: reason.to_string(),
        requested_by: requested_by.to_string(),
        mode: STOP_MODE_BEFORE_NEXT_CYCLE.to_string(),
        requested_at: Utc::now(),
    };
    write_json_pretty(&stop_path(loaded), &stop)?;
    append_operator_event(
        loaded,
        event,
        json!({
            "reason": reason,
            "mode": STOP_MODE_BEFORE_NEXT_CYCLE
        }),
    )?;
    let status = stopped_status(loaded, stop.reason);
    write_status(loaded, &status)?;
    Ok(status)
}

fn stopped_status(loaded: &LoadedAgentSpec, reason: String) -> StatusRecord {
    status_with_state(
        loaded,
        AgentStatusState::Stopped,
        read_status(loaded)
            .ok()
            .flatten()
            .and_then(|s| s.last_cycle_id),
        read_status(loaded)
            .ok()
            .flatten()
            .and_then(|s| s.last_cycle_status),
        None,
        true,
        Some(StatusError {
            class: "operator_stop_requested".to_string(),
            message: reason,
        }),
    )
}

fn next_cycle_id(loaded: &LoadedAgentSpec) -> Result<String> {
    let latest = ledger_cursor(loaded)?.max_cycle_number;
    let directory_latest = completed_cycle_count_from_dirs(loaded)?;
    let next = latest.max(directory_latest) + 1;
    Ok(format!("cycle-{number:06}", number = next))
}

fn latest_cycle_id(loaded: &LoadedAgentSpec) -> Result<Option<String>> {
    Ok(ledger_cursor(loaded)?.latest_cycle_id)
}

fn completed_cycle_count(loaded: &LoadedAgentSpec) -> Result<u64> {
    let ledger = ledger_cursor(loaded)?;
    if ledger.count > 0 {
        return Ok(ledger.count);
    }
    completed_cycle_count_from_dirs(loaded)
}

fn consecutive_failure_count(loaded: &LoadedAgentSpec) -> Result<u64> {
    let path = cycle_ledger_path(loaded);
    if !path.exists() {
        return Ok(0);
    }
    let file = File::open(&path)
        .with_context(|| format!("failed opening cycle ledger {}", path.display()))?;
    let mut statuses = Vec::new();
    for line in BufReader::new(file).lines() {
        let line =
            line.with_context(|| format!("failed reading cycle ledger {}", path.display()))?;
        if line.trim().is_empty() {
            continue;
        }
        let value: Value = serde_json::from_str(&line)
            .with_context(|| format!("failed parsing cycle ledger {}", path.display()))?;
        if let Some(status) = value.get("status").and_then(Value::as_str) {
            statuses.push(status.to_string());
        }
    }
    Ok(statuses
        .iter()
        .rev()
        .take_while(|status| status.as_str() != "success")
        .count() as u64)
}

fn completed_cycle_count_from_dirs(loaded: &LoadedAgentSpec) -> Result<u64> {
    let dir = cycles_dir(loaded);
    if !dir.exists() {
        return Ok(0);
    }
    let mut max_seen = 0u64;
    for entry in fs::read_dir(&dir).with_context(|| format!("failed reading {}", dir.display()))? {
        let entry = entry?;
        let Some(name) = entry.file_name().to_str().map(|s| s.to_string()) else {
            continue;
        };
        if let Some(number) = name.strip_prefix("cycle-") {
            if let Ok(parsed) = number.parse::<u64>() {
                max_seen = max_seen.max(parsed);
            }
        }
    }
    Ok(max_seen)
}

fn ledger_cursor(loaded: &LoadedAgentSpec) -> Result<LedgerCursor> {
    let path = cycle_ledger_path(loaded);
    if !path.exists() {
        return Ok(LedgerCursor::default());
    }
    let file = File::open(&path)
        .with_context(|| format!("failed opening cycle ledger {}", path.display()))?;
    let mut cursor = LedgerCursor::default();
    for line in BufReader::new(file).lines() {
        let line =
            line.with_context(|| format!("failed reading cycle ledger {}", path.display()))?;
        if line.trim().is_empty() {
            continue;
        }
        let value: Value = serde_json::from_str(&line)
            .with_context(|| format!("failed parsing cycle ledger {}", path.display()))?;
        cursor.count += 1;
        let Some(cycle_id) = value.get("cycle_id").and_then(Value::as_str) else {
            continue;
        };
        let Some(number) = cycle_number(cycle_id) else {
            continue;
        };
        if number >= cursor.max_cycle_number {
            cursor.max_cycle_number = number;
            cursor.latest_cycle_id = Some(cycle_id.to_string());
            cursor.latest_status = value
                .get("status")
                .and_then(Value::as_str)
                .map(str::to_string);
        }
    }
    Ok(cursor)
}

fn cycle_number(cycle_id: &str) -> Option<u64> {
    cycle_id.strip_prefix("cycle-")?.parse::<u64>().ok()
}

fn append_cycle_ledger_entry(
    loaded: &LoadedAgentSpec,
    cycle_id: &str,
    status: &str,
    started_at: DateTime<Utc>,
    completed_at: DateTime<Utc>,
    previous_cycle_id: Option<&str>,
) -> Result<()> {
    let entry = json!({
        "schema": CYCLE_LEDGER_ENTRY_SCHEMA,
        "agent_instance_id": loaded.spec.agent_instance_id.clone(),
        "cycle_id": cycle_id,
        "status": status,
        "started_at": started_at,
        "completed_at": completed_at,
        "previous_cycle_id": previous_cycle_id,
        "manifest_ref": format!("cycles/{cycle_id}/cycle_manifest.json"),
        "summary_ref": format!("cycles/{cycle_id}/cycle_summary.md"),
        "memory_writes_ref": format!("cycles/{cycle_id}/memory_writes.jsonl"),
        "guardrail_report_ref": format!("cycles/{cycle_id}/guardrail_report.json"),
        "continuity_kind": "pre_v0_92_handle"
    });
    append_jsonl(&cycle_ledger_path(loaded), &entry)
}

fn update_continuity_after_cycle(
    loaded: &LoadedAgentSpec,
    cycle_id: &str,
    cycle_status: &str,
) -> Result<()> {
    ensure_continuity(loaded)?;
    let path = continuity_path(loaded);
    let mut continuity: Value = read_json_required(&path)?;
    continuity["latest_cycle_id"] = json!(cycle_id);
    continuity["latest_cycle_status"] = json!(cycle_status);
    continuity["status"] = json!("active");
    continuity["updated_at"] = json!(Utc::now());
    write_json_pretty(&path, &continuity)
}

fn update_memory_index(loaded: &LoadedAgentSpec, cycle_id: &str) -> Result<()> {
    ensure_memory_index(loaded)?;
    let path = memory_index_path(loaded);
    let mut index: Value = read_json_required(&path)?;
    let memory_ref = format!("cycles/{cycle_id}/memory_writes.jsonl");
    let refs = index
        .get_mut("local_memory_refs")
        .and_then(Value::as_array_mut)
        .ok_or_else(|| anyhow!("memory index local_memory_refs must be an array"))?;
    if !refs.iter().any(|value| value.as_str() == Some(&memory_ref)) {
        refs.push(json!(memory_ref));
    }
    write_json_pretty(&path, &index)
}

fn provider_binding(loaded: &LoadedAgentSpec, cycle_id: &str, bound_at: DateTime<Utc>) -> Value {
    let provider_id = loaded
        .spec
        .workflow
        .run_args
        .get("provider_id")
        .or_else(|| loaded.spec.workflow.run_args.get("provider"))
        .and_then(Value::as_str)
        .map(str::to_string);
    let model = loaded
        .spec
        .workflow
        .run_args
        .get("model")
        .and_then(Value::as_str)
        .map(str::to_string);
    let binding_status = if provider_id.is_some() || model.is_some() {
        "available"
    } else {
        "not_available"
    };
    json!({
        "schema": PROVIDER_BINDING_SCHEMA,
        "agent_instance_id": loaded.spec.agent_instance_id.clone(),
        "cycle_id": cycle_id,
        "provider_id": provider_id.unwrap_or_else(|| loaded.spec.workflow.kind.clone()),
        "model": model,
        "binding_status": binding_status,
        "source": if binding_status == "available" {
            "workflow_run_args"
        } else {
            "workflow_kind_fallback"
        },
        "bound_at": bound_at
    })
}

fn read_status(loaded: &LoadedAgentSpec) -> Result<Option<StatusRecord>> {
    read_json_optional(&status_path(loaded))
}

fn write_status(loaded: &LoadedAgentSpec, status: &StatusRecord) -> Result<()> {
    write_json_pretty(&status_path(loaded), status)
}

fn read_lease(loaded: &LoadedAgentSpec) -> Result<Option<LeaseRecord>> {
    read_json_optional(&lease_path(loaded))
}

fn read_stop(loaded: &LoadedAgentSpec) -> Result<Option<StopRecord>> {
    read_json_optional(&stop_path(loaded))
}

fn lease_is_stale(lease: &LeaseRecord) -> bool {
    lease.status == "active" && lease.expires_at <= Utc::now()
}

fn remove_lease(loaded: &LoadedAgentSpec) -> Result<()> {
    let path = lease_path(loaded);
    match fs::remove_file(&path) {
        Ok(()) => Ok(()),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(err) => Err(err).with_context(|| format!("failed removing lease {}", path.display())),
    }
}

fn read_json_optional<T>(path: &Path) -> Result<Option<T>>
where
    T: for<'de> Deserialize<'de>,
{
    if !path.exists() {
        return Ok(None);
    }
    let raw = fs::read_to_string(path)
        .with_context(|| format!("failed reading json artifact {}", path.display()))?;
    let value = serde_json::from_str(&raw)
        .with_context(|| format!("failed parsing json artifact {}", path.display()))?;
    Ok(Some(value))
}

fn read_json_required(path: &Path) -> Result<Value> {
    let raw = fs::read_to_string(path)
        .with_context(|| format!("failed reading json artifact {}", path.display()))?;
    serde_json::from_str(&raw)
        .with_context(|| format!("failed parsing json artifact {}", path.display()))
}

fn write_json_pretty<T>(path: &Path, value: &T) -> Result<()>
where
    T: Serialize,
{
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed creating {}", parent.display()))?;
    }
    let file = File::create(path).with_context(|| format!("failed creating {}", path.display()))?;
    serde_json::to_writer_pretty(&file, value)
        .with_context(|| format!("failed writing {}", path.display()))?;
    fs::OpenOptions::new()
        .append(true)
        .open(path)?
        .write_all(b"\n")
        .with_context(|| format!("failed finalizing {}", path.display()))?;
    Ok(())
}

fn write_jsonl(path: &Path, values: &[Value]) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed creating {}", parent.display()))?;
    }
    let mut file =
        File::create(path).with_context(|| format!("failed creating {}", path.display()))?;
    for value in values {
        serde_json::to_writer(&mut file, value)
            .with_context(|| format!("failed writing {}", path.display()))?;
        file.write_all(b"\n")
            .with_context(|| format!("failed finalizing {}", path.display()))?;
    }
    Ok(())
}

fn append_jsonl(path: &Path, value: &Value) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed creating {}", parent.display()))?;
    }
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .with_context(|| format!("failed opening jsonl file {}", path.display()))?;
    serde_json::to_writer(&mut file, value)
        .with_context(|| format!("failed writing jsonl file {}", path.display()))?;
    file.write_all(b"\n")
        .with_context(|| format!("failed finalizing jsonl file {}", path.display()))?;
    Ok(())
}

fn append_operator_event(loaded: &LoadedAgentSpec, event: &str, details: Value) -> Result<()> {
    let record = json!({
        "schema": OPERATOR_EVENT_SCHEMA,
        "agent_instance_id": loaded.spec.agent_instance_id.clone(),
        "event": event,
        "at": Utc::now(),
        "operator": "local",
        "details": details
    });
    append_jsonl(&operator_events_path(loaded), &record)
}

fn inspect_cycle(loaded: &LoadedAgentSpec, cycle_id: &str) -> Result<Value> {
    validate_cycle_ref(cycle_id)?;
    let manifest_ref = format!("cycles/{cycle_id}/cycle_manifest.json");
    let guardrail_ref = format!("cycles/{cycle_id}/guardrail_report.json");
    let summary_ref = format!("cycles/{cycle_id}/cycle_summary.md");
    let decision_result_ref = format!("cycles/{cycle_id}/decision_result.json");
    let run_ref_ref = format!("cycles/{cycle_id}/run_ref.json");
    let memory_writes_ref = format!("cycles/{cycle_id}/memory_writes.jsonl");

    let manifest = read_state_json_artifact(loaded, &manifest_ref)?;
    let guardrail_report = read_state_json_artifact(loaded, &guardrail_ref)?;
    let run_ref = read_state_json_artifact(loaded, &run_ref_ref)?;
    let summary = read_state_text_artifact(loaded, &summary_ref)?;
    let guardrail_checks = guardrail_check_summary(&guardrail_report);
    let failed_guardrail_checks = guardrail_checks
        .iter()
        .filter_map(|check| {
            if check.get("result").and_then(Value::as_str) == Some("fail") {
                check
                    .get("check_id")
                    .and_then(Value::as_str)
                    .map(str::to_string)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    Ok(json!({
        "cycle_id": cycle_id,
        "status": manifest.get("status").cloned().unwrap_or(Value::Null),
        "workflow_kind": manifest.get("workflow_kind").cloned().unwrap_or(Value::Null),
        "workflow_ref": manifest.get("workflow_ref").cloned().unwrap_or(Value::Null),
        "refs": {
            "manifest": manifest_ref,
            "guardrail_report": guardrail_ref,
            "cycle_summary": summary_ref,
            "decision_result": decision_result_ref,
            "run_ref": run_ref_ref,
            "memory_writes": memory_writes_ref
        },
        "manifest": {
            "input_hash": manifest.get("input_hash").cloned().unwrap_or(Value::Null),
            "output_hash": manifest.get("output_hash").cloned().unwrap_or(Value::Null),
            "previous_cycle_id": manifest.get("previous_cycle_id").cloned().unwrap_or(Value::Null),
            "not_financial_advice": manifest.get("not_financial_advice").cloned().unwrap_or(Value::Null)
        },
        "guardrails": {
            "status": guardrail_report.get("status").cloned().unwrap_or(Value::Null),
            "checks": guardrail_checks,
            "failed_checks": failed_guardrail_checks,
            "rejected_actions": guardrail_report
                .get("rejected_actions")
                .cloned()
                .unwrap_or_else(|| json!([]))
        },
        "summary_preview": summary_preview(&summary),
        "trace_boundary": {
            "run_ref": run_ref_ref,
            "trace_ref": run_ref.get("trace_ref").cloned().unwrap_or(Value::Null),
            "status": if run_ref.get("trace_ref").is_some_and(|value| !value.is_null()) {
                "trace_ref_available"
            } else {
                "cycle_artifact_only"
            }
        }
    }))
}

fn validate_cycle_ref(cycle_id: &str) -> Result<()> {
    let Some(suffix) = cycle_id.strip_prefix("cycle-") else {
        return Err(anyhow!(
            "agent inspect --cycle must use a generated cycle id like cycle-000001"
        ));
    };
    if suffix.len() != 6 || !suffix.chars().all(|ch| ch.is_ascii_digit()) {
        return Err(anyhow!(
            "agent inspect --cycle must use a generated cycle id like cycle-000001"
        ));
    }
    Ok(())
}

fn read_state_json_artifact(loaded: &LoadedAgentSpec, artifact_ref: &str) -> Result<Value> {
    let path = loaded.state_root.join(artifact_ref);
    if !path.exists() {
        return Err(anyhow!("inspection artifact missing: {artifact_ref}"));
    }
    let raw = fs::read_to_string(&path)
        .with_context(|| format!("failed reading inspection artifact {artifact_ref}"))?;
    serde_json::from_str(&raw)
        .with_context(|| format!("failed parsing inspection artifact {artifact_ref}"))
}

fn read_state_text_artifact(loaded: &LoadedAgentSpec, artifact_ref: &str) -> Result<String> {
    let path = loaded.state_root.join(artifact_ref);
    if !path.exists() {
        return Err(anyhow!("inspection artifact missing: {artifact_ref}"));
    }
    fs::read_to_string(&path)
        .with_context(|| format!("failed reading inspection artifact {artifact_ref}"))
}

fn guardrail_check_summary(guardrail_report: &Value) -> Vec<Value> {
    guardrail_report
        .get("checks")
        .and_then(Value::as_array)
        .map(|checks| {
            checks
                .iter()
                .map(|check| {
                    json!({
                        "check_id": check.get("check_id").cloned().unwrap_or(Value::Null),
                        "result": check.get("result").cloned().unwrap_or(Value::Null)
                    })
                })
                .collect()
        })
        .unwrap_or_default()
}

fn summary_preview(summary: &str) -> Vec<String> {
    summary
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .take(6)
        .map(str::to_string)
        .collect()
}

fn sha256_json(value: &Value) -> Result<String> {
    let bytes = serde_json::to_vec(value)?;
    let digest = Sha256::digest(bytes);
    Ok(format!("sha256:{digest:x}"))
}

fn workflow_kind_supported(kind: &str) -> bool {
    matches!(kind, "demo_adapter" | "adl_workflow")
}

fn workflow_ref(workflow: &WorkflowSpec) -> String {
    workflow
        .name
        .clone()
        .or_else(|| workflow.path.as_deref().map(path_artifact_ref))
        .unwrap_or_else(|| workflow.kind.clone())
}

#[derive(Debug, Clone)]
struct SanitizationResult {
    passed: bool,
    findings: Vec<Value>,
}

impl SanitizationResult {
    fn skipped() -> Self {
        Self {
            passed: true,
            findings: vec![json!({
                "status": "skipped",
                "reason": "require_public_artifact_sanitization is false"
            })],
        }
    }
}

fn sanitize_public_artifacts(artifacts: &[(&str, &Value)]) -> Result<SanitizationResult> {
    let mut findings = Vec::new();
    let banned = [
        ("host_path", "/users/"),
        ("bearer_token", "bearer "),
        ("private_key", "private key"),
        ("api_key", "api_key"),
        ("api_key", "api key"),
        ("broker_account", "broker_account"),
        ("broker_token", "broker_token"),
        ("personal_portfolio", "personal_portfolio"),
        ("personal_risk_profile", "personal_risk_profile"),
    ];
    for (artifact, value) in artifacts {
        let raw = serde_json::to_string(value)
            .with_context(|| format!("failed serializing public artifact {artifact}"))?;
        let lower = raw.to_ascii_lowercase();
        for (finding, needle) in &banned {
            if lower.contains(needle) {
                findings.push(json!({
                    "artifact": artifact,
                    "finding": finding,
                    "pattern": needle
                }));
            }
        }
    }
    Ok(SanitizationResult {
        passed: findings.is_empty(),
        findings,
    })
}

fn rejected_actions_for_policy(loaded: &LoadedAgentSpec) -> Vec<String> {
    let mut rejected = Vec::new();
    let run_args = &loaded.spec.workflow.run_args;
    for action in requested_actions(run_args) {
        match action.to_ascii_lowercase().as_str() {
            "execute_order" | "place_order" | "trade" | "buy" | "sell" => {
                rejected.push("execute_order".to_string());
            }
            "connect_broker" | "broker_connect" => {
                rejected.push("connect_broker".to_string());
            }
            "personalized_advice" | "financial_advice" | "recommend_to_user" => {
                rejected.push("personalized_advice".to_string());
            }
            _ => {}
        }
    }
    if safety_bool_default(&loaded.spec.safety, "allow_broker", false)
        || contains_any_key(
            run_args,
            &[
                "broker_url",
                "broker_account_id",
                "broker_token",
                "broker_api_key",
                "broker_credentials",
            ],
        )
    {
        rejected.push("connect_broker".to_string());
    }
    if safety_bool_default(&loaded.spec.safety, "financial_advice", false)
        || contains_any_key(
            run_args,
            &[
                "personal_portfolio",
                "personal_risk_profile",
                "personal_assets",
                "private_portfolio_data",
            ],
        )
    {
        rejected.push("personalized_advice".to_string());
    }
    if safety_bool_default(&loaded.spec.safety, "allow_real_world_side_effects", false) {
        rejected.push("real_world_side_effect".to_string());
    }
    if safety_bool_default(
        &loaded.spec.safety,
        "allow_filesystem_writes_outside_state_root",
        false,
    ) {
        rejected.push("writes_outside_allowed_roots".to_string());
    }
    dedup_strings(&mut rejected);
    rejected
}

fn requested_actions(value: &Value) -> Vec<String> {
    let mut actions = Vec::new();
    collect_requested_actions(value, &mut actions);
    dedup_strings(&mut actions);
    actions
}

fn collect_requested_actions(value: &Value, actions: &mut Vec<String>) {
    match value {
        Value::Object(map) => {
            for (key, value) in map {
                if matches!(
                    key.as_str(),
                    "action" | "requested_action" | "tool" | "tool_name"
                ) {
                    if let Some(action) = value.as_str() {
                        actions.push(action.to_string());
                    }
                }
                if key == "actions" {
                    if let Some(items) = value.as_array() {
                        for item in items {
                            if let Some(action) = item.as_str() {
                                actions.push(action.to_string());
                            }
                        }
                    }
                }
                collect_requested_actions(value, actions);
            }
        }
        Value::Array(items) => {
            for item in items {
                collect_requested_actions(item, actions);
            }
        }
        _ => {}
    }
}

fn contains_any_key(value: &Value, keys: &[&str]) -> bool {
    match value {
        Value::Object(map) => map.iter().any(|(key, value)| {
            keys.iter().any(|candidate| key == candidate) || contains_any_key(value, keys)
        }),
        Value::Array(items) => items.iter().any(|item| contains_any_key(item, keys)),
        _ => false,
    }
}

fn dedup_strings(values: &mut Vec<String>) {
    let mut seen = Vec::new();
    values.retain(|value| {
        if seen.contains(value) {
            false
        } else {
            seen.push(value.clone());
            true
        }
    });
}

fn path_artifact_ref(path: &Path) -> String {
    if path.is_absolute() {
        return path
            .file_name()
            .and_then(|name| name.to_str())
            .map(|name| format!("absolute-path-redacted/{name}"))
            .unwrap_or_else(|| "absolute-path-redacted/workflow".to_string());
    }
    path.to_string_lossy().to_string()
}

fn safety_bool_default(safety: &Value, key: &str, default: bool) -> bool {
    safety.get(key).and_then(Value::as_bool).unwrap_or(default)
}

fn safety_u64(safety: &Value, key: &str, default: u64) -> u64 {
    safety.get(key).and_then(Value::as_u64).unwrap_or(default)
}

fn effective_safety_policy(loaded: &LoadedAgentSpec) -> Value {
    json!({
        "allow_network": safety_bool_default(&loaded.spec.safety, "allow_network", false),
        "allow_broker": safety_bool_default(&loaded.spec.safety, "allow_broker", false),
        "allow_filesystem_writes_outside_state_root": safety_bool_default(
            &loaded.spec.safety,
            "allow_filesystem_writes_outside_state_root",
            false,
        ),
        "allow_real_world_side_effects": safety_bool_default(
            &loaded.spec.safety,
            "allow_real_world_side_effects",
            false,
        ),
        "require_public_artifact_sanitization": safety_bool_default(
            &loaded.spec.safety,
            "require_public_artifact_sanitization",
            true,
        ),
        "financial_advice": safety_bool_default(&loaded.spec.safety, "financial_advice", false),
        "max_cycle_runtime_secs": safety_u64(
            &loaded.spec.safety,
            "max_cycle_runtime_secs",
            DEFAULT_MAX_CYCLE_RUNTIME_SECS,
        ),
        "max_consecutive_failures": max_consecutive_failures(loaded)
    })
}

fn max_cycle_runtime_secs(loaded: &LoadedAgentSpec) -> u64 {
    safety_u64(
        &loaded.spec.safety,
        "max_cycle_runtime_secs",
        DEFAULT_MAX_CYCLE_RUNTIME_SECS,
    )
}

fn max_consecutive_failures(loaded: &LoadedAgentSpec) -> u64 {
    safety_u64(
        &loaded.spec.safety,
        "max_consecutive_failures",
        DEFAULT_MAX_CONSECUTIVE_FAILURES,
    )
}

fn default_requested_by() -> String {
    "operator".to_string()
}

fn default_stop_mode() -> String {
    STOP_MODE_BEFORE_NEXT_CYCLE.to_string()
}

fn memory_namespace(loaded: &LoadedAgentSpec) -> String {
    loaded
        .spec
        .memory
        .get("namespace")
        .and_then(Value::as_str)
        .filter(|value| !value.trim().is_empty())
        .map(str::to_string)
        .unwrap_or_else(|| loaded.spec.agent_instance_id.clone())
}

fn cycles_dir(loaded: &LoadedAgentSpec) -> PathBuf {
    loaded.state_root.join("cycles")
}

fn locked_spec_path(loaded: &LoadedAgentSpec) -> PathBuf {
    loaded.state_root.join("agent_spec.locked.json")
}

fn continuity_path(loaded: &LoadedAgentSpec) -> PathBuf {
    loaded.state_root.join("continuity.json")
}

fn cycle_ledger_path(loaded: &LoadedAgentSpec) -> PathBuf {
    loaded.state_root.join("cycle_ledger.jsonl")
}

fn provider_binding_history_path(loaded: &LoadedAgentSpec) -> PathBuf {
    loaded.state_root.join("provider_binding_history.jsonl")
}

fn memory_index_path(loaded: &LoadedAgentSpec) -> PathBuf {
    loaded.state_root.join("memory_index.json")
}

fn operator_events_path(loaded: &LoadedAgentSpec) -> PathBuf {
    loaded.state_root.join("operator_events.jsonl")
}

fn status_path(loaded: &LoadedAgentSpec) -> PathBuf {
    loaded.state_root.join("status.json")
}

fn lease_path(loaded: &LoadedAgentSpec) -> PathBuf {
    loaded.state_root.join("lease.json")
}

fn stop_path(loaded: &LoadedAgentSpec) -> PathBuf {
    loaded.state_root.join("stop.json")
}

fn hostname() -> String {
    std::env::var("HOSTNAME")
        .or_else(|_| std::env::var("COMPUTERNAME"))
        .unwrap_or_else(|_| "local".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};

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
        let ledger =
            fs::read_to_string(root.join("state/cycle_ledger.jsonl")).expect("read ledger");
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
        let provider_history =
            fs::read_to_string(root.join("state/provider_binding_history.jsonl"))
                .expect("read provider history");
        let provider_entry: Value =
            serde_json::from_str(provider_history.lines().next().expect("provider line"))
                .expect("parse provider binding");
        assert_eq!(provider_entry["schema"], PROVIDER_BINDING_SCHEMA);
        assert_eq!(provider_entry["provider_id"], "local_ollama");
        assert_eq!(provider_entry["model"], "gemma4:latest");
        assert_eq!(provider_entry["binding_status"], "available");
        let memory_index: Value = serde_json::from_str(
            &fs::read_to_string(root.join("state/memory_index.json")).unwrap(),
        )
        .expect("parse memory index");
        assert_eq!(memory_index["schema"], MEMORY_INDEX_SCHEMA);
        assert_eq!(
            memory_index["local_memory_refs"][0],
            "cycles/cycle-000001/memory_writes.jsonl"
        );
        assert!(!root.join("state/lease.json").exists());
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
        let provider_history =
            fs::read_to_string(root.join("state/provider_binding_history.jsonl"))
                .expect("read provider history");
        assert_eq!(provider_history.lines().count(), 3);
        let continuity: Value =
            serde_json::from_str(&fs::read_to_string(root.join("state/continuity.json")).unwrap())
                .expect("parse continuity");
        assert_eq!(continuity["latest_cycle_id"], "cycle-000003");
        let memory_index: Value = serde_json::from_str(
            &fs::read_to_string(root.join("state/memory_index.json")).unwrap(),
        )
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

        let recovered = status(&spec).expect("status recovers from ledger");

        assert_eq!(recovered.state, AgentStatusState::Idle);
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
        let ledger =
            fs::read_to_string(root.join("state/cycle_ledger.jsonl")).expect("read ledger");
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
}
