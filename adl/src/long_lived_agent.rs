use anyhow::{anyhow, Context, Result};
use chrono::{DateTime, Duration as ChronoDuration, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::fs::{self, File, OpenOptions};
use std::io::Write;
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
    pub active_lease: Option<LeaseRecord>,
    pub stop_requested: bool,
    pub last_error: Option<StatusError>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StopRecord {
    pub schema: String,
    pub agent_instance_id: String,
    pub reason: String,
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
            let status = status_with_state(
                &loaded,
                AgentStatusState::Failed,
                Some(cycle_id),
                Some("failed".to_string()),
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
        last_status = tick(
            spec_path,
            TickOptions {
                recover_stale_lease: options.recover_stale_lease,
            },
        )?;
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
    let mut current = read_status(&loaded)?.unwrap_or_else(|| {
        status_with_state(
            &loaded,
            AgentStatusState::NotStarted,
            None,
            None,
            None,
            false,
            None,
        )
    });

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
    let stop = StopRecord {
        schema: STOP_SCHEMA.to_string(),
        agent_instance_id: loaded.spec.agent_instance_id.clone(),
        reason: reason.trim().to_string(),
        requested_at: Utc::now(),
    };
    write_json_pretty(&stop_path(&loaded), &stop)?;
    let status = stopped_status(&loaded, stop.reason);
    write_status(&loaded, &status)?;
    Ok(status)
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
    Ok(())
}

fn ensure_state_root(loaded: &LoadedAgentSpec) -> Result<()> {
    fs::create_dir_all(cycles_dir(loaded))
        .with_context(|| format!("failed creating {}", cycles_dir(loaded).display()))?;
    let locked = loaded.state_root.join("agent_spec.locked.json");
    if !locked.exists() {
        write_json_pretty(&locked, &loaded.spec)?;
    }
    Ok(())
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
    let previous_cycle_id = previous_cycle_id(cycle_id)?;
    let workflow_ref = workflow_ref(&loaded.spec.workflow);
    let workflow_supported = workflow_kind_supported(&loaded.spec.workflow.kind);
    let broker_allowed = safety_bool(&loaded.spec.safety, "allow_broker");
    let financial_advice_allowed = safety_bool(&loaded.spec.safety, "financial_advice");
    let guardrail_pass = workflow_supported && !broker_allowed && !financial_advice_allowed;
    let cycle_status = if guardrail_pass { "success" } else { "blocked" };
    let decision_status = if guardrail_pass {
        "accepted"
    } else {
        "rejected"
    };
    let guardrail_status = if guardrail_pass { "pass" } else { "fail" };

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
            "source": loaded.spec.workflow.kind.clone(),
            "model": null
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

    let mut checks = Vec::new();
    checks.push(json!({
        "check_id": "workflow_kind_supported",
        "result": if workflow_supported { "pass" } else { "fail" },
        "details": loaded.spec.workflow.kind.clone()
    }));
    checks.push(json!({
        "check_id": "no_real_trading",
        "result": if broker_allowed { "fail" } else { "pass" }
    }));
    checks.push(json!({
        "check_id": "no_personalized_financial_advice",
        "result": if financial_advice_allowed { "fail" } else { "pass" }
    }));
    let mut rejected_actions = Vec::new();
    if !workflow_supported {
        rejected_actions.push("unsupported_workflow_kind");
    }
    if broker_allowed {
        rejected_actions.push("connect_broker");
    }
    if financial_advice_allowed {
        rejected_actions.push("personalized_advice");
    }
    let guardrail_report = json!({
        "schema": GUARDRAIL_REPORT_SCHEMA,
        "agent_instance_id": loaded.spec.agent_instance_id.clone(),
        "cycle_id": cycle_id,
        "status": guardrail_status,
        "checks": checks,
        "rejected_actions": rejected_actions
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
        active_lease,
        stop_requested,
        last_error,
        updated_at: Utc::now(),
    }
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
    Ok(format!(
        "cycle-{number:06}",
        number = completed_cycle_count(loaded)? + 1
    ))
}

fn previous_cycle_id(cycle_id: &str) -> Result<Option<String>> {
    let current = cycle_id
        .trim_start_matches("cycle-")
        .parse::<u64>()
        .with_context(|| format!("invalid cycle id {cycle_id}"))?;
    if current <= 1 {
        Ok(None)
    } else {
        Ok(Some(format!("cycle-{number:06}", number = current - 1)))
    }
}

fn completed_cycle_count(loaded: &LoadedAgentSpec) -> Result<u64> {
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

fn safety_bool(safety: &Value, key: &str) -> bool {
    safety.get(key).and_then(Value::as_bool).unwrap_or(false)
}

fn cycles_dir(loaded: &LoadedAgentSpec) -> PathBuf {
    loaded.state_root.join("cycles")
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
heartbeat:
  interval_secs: 1
  max_cycles: 3
  stale_lease_after_secs: 60
safety:
  allow_network: false
  allow_broker: {allow_broker}
  financial_advice: {financial_advice}
memory:
  namespace: tests/test-agent
  write_policy: append_only
"#,
            ),
        )
        .expect("write spec");
        spec
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

    #[test]
    fn tick_creates_state_status_full_cycle_bundle_and_removes_lease() {
        let root = temp_dir("tick");
        let spec = write_spec(&root);

        let status = tick(&spec, TickOptions::default()).expect("tick");

        assert_eq!(status.state, AgentStatusState::Idle);
        assert_eq!(status.completed_cycle_count, 1);
        assert_eq!(status.last_cycle_id.as_deref(), Some("cycle-000001"));
        assert!(root.join("state/agent_spec.locked.json").exists());
        assert!(root.join("state/status.json").exists());
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
        let manifest: Value = serde_json::from_str(
            &fs::read_to_string(root.join("state/cycles/cycle-000002/cycle_manifest.json"))
                .expect("read manifest"),
        )
        .expect("parse manifest");
        assert_eq!(manifest["previous_cycle_id"], "cycle-000001");
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
        assert_eq!(guardrails["checks"][1]["check_id"], "no_real_trading");
        assert_eq!(guardrails["checks"][1]["result"], "fail");
        assert_eq!(
            guardrails["checks"][2]["check_id"],
            "no_personalized_financial_advice"
        );
        assert_eq!(guardrails["checks"][2]["result"], "fail");
        assert_eq!(guardrails["rejected_actions"][0], "connect_broker");
        assert_eq!(guardrails["rejected_actions"][1], "personalized_advice");
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
    }
}
