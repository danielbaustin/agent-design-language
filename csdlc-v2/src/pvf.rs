use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io::Read;
use std::path::{Component, Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};
use std::thread;
use std::time::{Duration, Instant};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumString};

use crate::cards::{EvidenceOutcome, ValidationResult};
use crate::store::ImplementationCommit;
use crate::{ErrorCode, IssueRecord, Result, Store, V2Error};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    JsonSchema,
    Display,
    EnumString,
    AsRefStr,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum Determinism {
    Deterministic,
    ControlledExternal,
    Live,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    JsonSchema,
    Display,
    EnumString,
    AsRefStr,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum NetworkPolicy {
    Denied,
    Loopback,
    External,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    JsonSchema,
    Display,
    EnumString,
    AsRefStr,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum ReleaseGate {
    Required,
    Optional,
    NonGoal,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    JsonSchema,
    Display,
    EnumString,
    AsRefStr,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum ExecutionMode {
    Local,
    DeferredCi,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ResourceCost {
    pub cpu_units: u32,
    pub memory_mib: u32,
    pub tokens: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct EvidencePolicy {
    pub max_log_bytes: usize,
    pub redact_values: Vec<String>,
    pub require_relative_paths: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PvfLane {
    pub id: String,
    pub proof_role: String,
    pub purpose: String,
    pub determinism: Determinism,
    pub resources: ResourceCost,
    pub credentials: Vec<String>,
    pub network: NetworkPolicy,
    pub dependencies: Vec<String>,
    pub parallel_group: String,
    pub release_gate: ReleaseGate,
    pub execution: ExecutionMode,
    pub timeout_seconds: u64,
    pub executable: String,
    pub argv: Vec<String>,
    pub evidence: EvidencePolicy,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PvfManifest {
    pub schema: String,
    pub lanes: Vec<PvfLane>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SelectionRequest {
    pub requested_lanes: Vec<String>,
    pub allow_network: bool,
    pub available_credentials: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SelectedDag {
    pub schema: String,
    pub waves: Vec<Vec<String>>,
    pub lanes: BTreeMap<String, PvfLane>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ExecutionBudget {
    pub max_parallel: usize,
    pub cpu_units: u32,
    pub memory_mib: u32,
    pub tokens: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ExecutionRequest {
    pub manifest: PvfManifest,
    pub selection: SelectionRequest,
    pub budget: ExecutionBudget,
    pub root: PathBuf,
    pub evidence_dir: PathBuf,
    pub cancellation_file: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FinalizeRequest {
    pub schema: String,
    pub issue: u64,
    pub expected_generation: u64,
    pub expected_digest: String,
    pub claim_id: String,
    pub actor: String,
    pub summary: String,
    pub changes: Vec<String>,
    pub artifacts: Vec<String>,
    pub execution: ExecutionRequest,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    JsonSchema,
    Display,
    EnumString,
    AsRefStr,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum LaneStatus {
    Passed,
    DeferredCi,
    Failed,
    TimedOut,
    Blocked,
    AcceptedNonGoal,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct LaneEvidence {
    pub lane: String,
    pub command: Vec<String>,
    pub purpose: String,
    pub status: LaneStatus,
    pub duration_ms: u128,
    pub log_ref: Option<String>,
    pub redaction_ok: bool,
    pub redactions_applied: usize,
    pub path_hygiene_ok: bool,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    JsonSchema,
    Display,
    EnumString,
    AsRefStr,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum ValidationDisposition {
    LocalPass,
    DeferredCi,
    Waiting,
    Failed,
    Blocked,
    AcceptedNonGoal,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ExecutionReport {
    pub schema: String,
    pub disposition: ValidationDisposition,
    pub selected_waves: Vec<Vec<String>>,
    pub evidence: Vec<LaneEvidence>,
}

pub fn select(manifest: &PvfManifest, request: &SelectionRequest) -> Result<SelectedDag> {
    validate_manifest(manifest)?;
    let all: BTreeMap<_, _> = manifest
        .lanes
        .iter()
        .map(|lane| (lane.id.clone(), lane.clone()))
        .collect();
    let mut selected: BTreeSet<String> = request.requested_lanes.iter().cloned().collect();
    for lane in &manifest.lanes {
        if lane.release_gate == ReleaseGate::Required {
            selected.insert(lane.id.clone());
        }
    }
    let mut pending: Vec<_> = selected.iter().cloned().collect();
    while let Some(id) = pending.pop() {
        let lane = all.get(&id).ok_or_else(|| {
            V2Error::new(
                ErrorCode::InvalidManifest,
                format!("unknown selected lane {id}"),
            )
        })?;
        for dep in &lane.dependencies {
            if selected.insert(dep.clone()) {
                pending.push(dep.clone());
            }
        }
    }
    for id in &selected {
        let lane = &all[id];
        if lane.network == NetworkPolicy::External && !request.allow_network {
            return Err(V2Error::new(
                ErrorCode::InvalidManifest,
                format!("lane {id} requires external network"),
            ));
        }
        if lane
            .credentials
            .iter()
            .any(|name| !request.available_credentials.contains(name))
        {
            return Err(V2Error::new(
                ErrorCode::InvalidManifest,
                format!("lane {id} requires unavailable credentials"),
            ));
        }
    }
    let mut remaining = selected.clone();
    let mut done = BTreeSet::new();
    let mut waves = Vec::new();
    while !remaining.is_empty() {
        let wave: Vec<_> = remaining
            .iter()
            .filter(|id| all[*id].dependencies.iter().all(|dep| done.contains(dep)))
            .cloned()
            .collect();
        if wave.is_empty() {
            return Err(V2Error::new(
                ErrorCode::InvalidManifest,
                "validation dependency cycle",
            ));
        }
        for id in &wave {
            remaining.remove(id);
            done.insert(id.clone());
        }
        waves.push(wave);
    }
    Ok(SelectedDag {
        schema: "csdlc.pvf.selected.v1".into(),
        waves,
        lanes: all
            .into_iter()
            .filter(|(id, _)| selected.contains(id))
            .collect(),
    })
}

fn validate_manifest(manifest: &PvfManifest) -> Result<()> {
    if manifest.schema != "csdlc.pvf.manifest.v1" || manifest.lanes.is_empty() {
        return Err(V2Error::new(
            ErrorCode::InvalidManifest,
            "invalid PVF schema or empty lanes",
        ));
    }
    let ids: BTreeSet<_> = manifest.lanes.iter().map(|lane| lane.id.as_str()).collect();
    if ids.len() != manifest.lanes.len() {
        return Err(V2Error::new(
            ErrorCode::InvalidManifest,
            "duplicate lane id",
        ));
    }
    for lane in &manifest.lanes {
        if lane.id.trim().is_empty()
            || !lane
                .id
                .chars()
                .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '_' | '-' | '.'))
            || lane.proof_role.trim().is_empty()
            || lane.purpose.trim().is_empty()
            || lane.executable.trim().is_empty()
            || lane.parallel_group.trim().is_empty()
            || lane.timeout_seconds == 0
            || lane.evidence.max_log_bytes == 0
            || lane
                .dependencies
                .iter()
                .any(|dep| !ids.contains(dep.as_str()) || dep == &lane.id)
        {
            return Err(V2Error::new(
                ErrorCode::InvalidManifest,
                format!("lane {} is incomplete", lane.id),
            ));
        }
    }
    Ok(())
}

pub fn execute(request: ExecutionRequest) -> Result<ExecutionReport> {
    if request.budget.max_parallel == 0 {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "max_parallel must be positive",
        ));
    }
    let dag = select(&request.manifest, &request.selection)?;
    let batches = plan_batches(&dag, &request.budget)?;
    fs::create_dir_all(&request.evidence_dir)?;
    let evidence = Arc::new(Mutex::new(Vec::new()));
    let outcomes = Arc::new(Mutex::new(BTreeMap::<String, LaneStatus>::new()));
    let cancelled = Arc::new(AtomicBool::new(false));
    for (wave_index, wave) in dag.waves.iter().enumerate() {
        if request
            .cancellation_file
            .as_ref()
            .is_some_and(|path| path.exists())
        {
            cancelled.store(true, Ordering::SeqCst);
            break;
        }
        for id in wave {
            let lane = &dag.lanes[id];
            let dependency_statuses: Vec<_> = lane
                .dependencies
                .iter()
                .filter_map(|dep| outcomes.lock().expect("outcomes").get(dep).copied())
                .collect();
            let inherited = if dependency_statuses.iter().any(|s| {
                matches!(
                    s,
                    LaneStatus::Failed | LaneStatus::TimedOut | LaneStatus::Blocked
                )
            }) {
                Some(LaneStatus::Blocked)
            } else if dependency_statuses.contains(&LaneStatus::DeferredCi) {
                Some(LaneStatus::DeferredCi)
            } else if dependency_statuses.contains(&LaneStatus::AcceptedNonGoal) {
                Some(LaneStatus::Blocked)
            } else {
                None
            };
            let status = inherited.or_else(|| {
                if lane.release_gate == ReleaseGate::NonGoal {
                    Some(LaneStatus::AcceptedNonGoal)
                } else if lane.execution == ExecutionMode::DeferredCi {
                    Some(LaneStatus::DeferredCi)
                } else {
                    None
                }
            });
            if let Some(status) = status {
                outcomes
                    .lock()
                    .expect("outcomes")
                    .insert(id.clone(), status);
                evidence
                    .lock()
                    .expect("evidence")
                    .push(skipped_evidence(lane, status));
            }
        }
        for batch in &batches[wave_index] {
            if cancelled.load(Ordering::SeqCst) {
                break;
            }
            let runnable: Vec<_> = batch
                .iter()
                .filter(|id| !outcomes.lock().expect("outcomes").contains_key(*id))
                .cloned()
                .collect();
            let mut handles = Vec::new();
            for id in runnable {
                let lane = dag.lanes[&id].clone();
                let root = request.root.clone();
                let dir = request.evidence_dir.clone();
                let cancel = Arc::clone(&cancelled);
                let cancel_path = request.cancellation_file.clone();
                handles.push(thread::spawn(move || {
                    run_lane(&root, &dir, &lane, &cancel, cancel_path.as_deref())
                }));
            }
            for handle in handles {
                let item = handle.join().map_err(|_| {
                    V2Error::new(ErrorCode::ValidationFailed, "validation worker panicked")
                })?;
                if matches!(
                    item.status,
                    LaneStatus::Failed | LaneStatus::TimedOut | LaneStatus::Blocked
                ) {
                    cancelled.store(true, Ordering::SeqCst);
                }
                outcomes
                    .lock()
                    .expect("outcomes")
                    .insert(item.lane.clone(), item.status);
                evidence.lock().expect("evidence").push(item);
            }
        }
        if cancelled.load(Ordering::SeqCst) {
            break;
        }
    }
    let mut evidence = Arc::try_unwrap(evidence)
        .expect("workers complete")
        .into_inner()
        .expect("evidence lock");
    evidence.sort_by(|a, b| a.lane.cmp(&b.lane));
    let disposition = converge(
        &evidence,
        cancelled.load(Ordering::SeqCst),
        request
            .cancellation_file
            .as_ref()
            .is_some_and(|p| p.exists()),
    );
    Ok(ExecutionReport {
        schema: "csdlc.pvf.report.v1".into(),
        disposition,
        selected_waves: dag.waves,
        evidence,
    })
}

pub fn finalize(store: &Store, request: FinalizeRequest) -> Result<IssueRecord> {
    if request.schema != "csdlc.finalize_request.v1"
        || request.actor.trim().is_empty()
        || request.summary.trim().is_empty()
        || request.changes.is_empty()
        || request.execution.root != store.root()
    {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "finalize request identity, execution root, or execution summary is invalid",
        ));
    }
    let before = store.load_record(request.issue)?;
    if before.generation != request.expected_generation || before.digest != request.expected_digest
    {
        return Err(V2Error::new(
            ErrorCode::StaleDigest,
            "finalize request does not match canonical record",
        ));
    }
    before
        .claim
        .as_ref()
        .ok_or_else(|| V2Error::new(ErrorCode::MissingClaim, "claim missing"))?
        .validate(&request.claim_id, crate::store::now_seconds()?)?;

    let evidence_dir = request.execution.evidence_dir.clone();
    let expected_evidence_dir = store
        .root()
        .join(".csdlc")
        .join("evidence")
        .join(request.issue.to_string());
    if evidence_dir != expected_evidence_dir
        || [
            store.root().join(".csdlc"),
            store.root().join(".csdlc/evidence"),
        ]
        .iter()
        .any(|path| fs::symlink_metadata(path).is_ok_and(|metadata| metadata.is_symlink()))
    {
        return Err(V2Error::new(
            ErrorCode::UnsafeCheckout,
            "finalize evidence must use the issue-owned repository evidence directory",
        ));
    }
    let evidence_parent = evidence_dir.parent().ok_or_else(|| {
        V2Error::new(
            ErrorCode::InvalidInput,
            "finalize evidence directory must have a parent",
        )
    })?;
    fs::create_dir_all(evidence_parent)?;
    let nonce = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|_| V2Error::new(ErrorCode::Io, "system clock is before the Unix epoch"))?
        .as_nanos();
    let staging_dir = evidence_parent.join(format!(
        ".csdlc-finalize-{}-{}-{nonce}",
        request.issue,
        std::process::id()
    ));
    let mut execution = request.execution.clone();
    execution.evidence_dir = staging_dir.clone();
    let report = match execute(execution) {
        Ok(report) => report,
        Err(error) => {
            let _ = remove_path_no_follow(&staging_dir);
            return Err(error);
        }
    };
    if report.disposition != ValidationDisposition::LocalPass {
        let _ = remove_path_no_follow(&staging_dir);
        return Err(V2Error::new(
            ErrorCode::ValidationFailed,
            "finalize requires a complete local validation pass",
        ));
    }
    let validation = report
        .evidence
        .iter()
        .map(|lane| ValidationResult {
            command: lane.command.clone(),
            purpose: lane.purpose.clone(),
            outcome: match lane.status {
                LaneStatus::Passed => EvidenceOutcome::Passed,
                LaneStatus::AcceptedNonGoal => EvidenceOutcome::SkippedNonGoal,
                LaneStatus::DeferredCi => EvidenceOutcome::Deferred,
                LaneStatus::Failed | LaneStatus::TimedOut => EvidenceOutcome::Failed,
                LaneStatus::Blocked => EvidenceOutcome::Blocked,
            },
            evidence_ref: lane
                .log_ref
                .clone()
                .unwrap_or_else(|| format!("pvf:{}", lane.lane)),
        })
        .collect();
    let committed = store.commit_implementation(
        ImplementationCommit {
            issue: request.issue,
            expected_generation: request.expected_generation,
            expected_digest: request.expected_digest,
            claim_id: request.claim_id,
            actor: request.actor,
            summary: request.summary,
            changes: request.changes,
            artifacts: request.artifacts,
            validation,
        },
        &staging_dir,
        &evidence_dir,
    );
    if committed.is_err() && staging_dir.exists() {
        remove_path_no_follow(&staging_dir).map_err(|_| {
            V2Error::new(
                ErrorCode::ReconciliationRequired,
                "failed to remove staged evidence after finalize rejection",
            )
        })?;
    }
    committed
}

fn remove_path_no_follow(path: &Path) -> std::io::Result<()> {
    let metadata = match fs::symlink_metadata(path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(error) => return Err(error),
    };
    if metadata.file_type().is_symlink() || metadata.is_file() {
        fs::remove_file(path)
    } else {
        fs::remove_dir_all(path)
    }
}

fn skipped_evidence(lane: &PvfLane, status: LaneStatus) -> LaneEvidence {
    LaneEvidence {
        lane: lane.id.clone(),
        command: redacted_command(lane),
        purpose: lane.purpose.clone(),
        status,
        duration_ms: 0,
        log_ref: None,
        redaction_ok: true,
        redactions_applied: 0,
        path_hygiene_ok: true,
    }
}

fn plan_batches(dag: &SelectedDag, budget: &ExecutionBudget) -> Result<Vec<Vec<Vec<String>>>> {
    let total: u64 = dag
        .lanes
        .values()
        .filter(|l| l.execution == ExecutionMode::Local && l.release_gate != ReleaseGate::NonGoal)
        .map(|l| l.resources.tokens)
        .sum();
    if total > budget.tokens {
        return Err(V2Error::new(
            ErrorCode::InvalidManifest,
            "selected local lanes exceed total token budget",
        ));
    }
    let mut result = Vec::new();
    for wave in &dag.waves {
        let mut groups: BTreeMap<String, Vec<String>> = BTreeMap::new();
        for id in wave {
            let lane = &dag.lanes[id];
            if lane.execution != ExecutionMode::Local || lane.release_gate == ReleaseGate::NonGoal {
                continue;
            }
            if lane.resources.cpu_units > budget.cpu_units
                || lane.resources.memory_mib > budget.memory_mib
            {
                return Err(V2Error::new(
                    ErrorCode::InvalidManifest,
                    format!("lane {id} exceeds resource budget"),
                ));
            }
            groups
                .entry(lane.parallel_group.clone())
                .or_default()
                .push(id.clone());
        }
        let mut batches = Vec::new();
        for (_, ids) in groups {
            let mut batch = Vec::new();
            let (mut cpu, mut mem) = (0, 0);
            for id in ids {
                let c = &dag.lanes[&id].resources;
                if batch.len() == budget.max_parallel
                    || cpu + c.cpu_units > budget.cpu_units
                    || mem + c.memory_mib > budget.memory_mib
                {
                    batches.push(batch);
                    batch = Vec::new();
                    cpu = 0;
                    mem = 0;
                }
                cpu += c.cpu_units;
                mem += c.memory_mib;
                batch.push(id);
            }
            if !batch.is_empty() {
                batches.push(batch);
            }
        }
        result.push(batches);
    }
    Ok(result)
}

fn converge(
    evidence: &[LaneEvidence],
    cancelled: bool,
    external_cancel: bool,
) -> ValidationDisposition {
    if external_cancel {
        return ValidationDisposition::Waiting;
    }
    if evidence
        .iter()
        .any(|e| matches!(e.status, LaneStatus::Failed | LaneStatus::TimedOut))
    {
        return ValidationDisposition::Failed;
    }
    if evidence.iter().any(|e| e.status == LaneStatus::Blocked) {
        return ValidationDisposition::Blocked;
    }
    if cancelled {
        return ValidationDisposition::Blocked;
    }
    if evidence.iter().any(|e| e.status == LaneStatus::DeferredCi) {
        return ValidationDisposition::DeferredCi;
    }
    if !evidence.is_empty()
        && evidence
            .iter()
            .all(|e| e.status == LaneStatus::AcceptedNonGoal)
    {
        return ValidationDisposition::AcceptedNonGoal;
    }
    if evidence.is_empty() {
        ValidationDisposition::Waiting
    } else {
        ValidationDisposition::LocalPass
    }
}

fn redacted_command(lane: &PvfLane) -> Vec<String> {
    std::iter::once(lane.executable.clone())
        .chain(lane.argv.clone())
        .map(|mut value| {
            for secret in &lane.evidence.redact_values {
                if !secret.is_empty() {
                    value = value.replace(secret, "[REDACTED]");
                }
            }
            value
        })
        .collect()
}

fn run_lane(
    root: &Path,
    evidence_dir: &Path,
    lane: &PvfLane,
    cancelled: &AtomicBool,
    cancellation_file: Option<&Path>,
) -> LaneEvidence {
    let started = Instant::now();
    let command = redacted_command(lane);
    let result = (|| -> Result<(LaneStatus, Vec<u8>)> {
        use std::os::unix::process::CommandExt;
        let mut child = Command::new(&lane.executable)
            .args(&lane.argv)
            .current_dir(root)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .process_group(0)
            .spawn()?;
        let stdout = child.stdout.take().expect("stdout");
        let stderr = child.stderr.take().expect("stderr");
        let limit = lane.evidence.max_log_bytes;
        let out = thread::spawn(move || drain(stdout, limit));
        let err = thread::spawn(move || drain(stderr, limit));
        let deadline = Instant::now() + Duration::from_secs(lane.timeout_seconds);
        let timed_out = loop {
            if let Some(status) = child.try_wait()? {
                let mut bytes = out.join().unwrap_or_default();
                bytes.extend(err.join().unwrap_or_default());
                return Ok((
                    if status.success() {
                        LaneStatus::Passed
                    } else {
                        LaneStatus::Failed
                    },
                    bytes,
                ));
            }
            if cancellation_file.is_some_and(|path| path.exists()) {
                cancelled.store(true, Ordering::SeqCst);
            }
            if cancelled.load(Ordering::SeqCst) || Instant::now() >= deadline {
                let timed_out = Instant::now() >= deadline;
                unsafe {
                    libc::kill(-(child.id() as i32), libc::SIGKILL);
                }
                let _ = child.wait();
                break timed_out;
            }
            thread::sleep(Duration::from_millis(10));
        };
        let mut bytes = out.join().unwrap_or_default();
        bytes.extend(err.join().unwrap_or_default());
        Ok((
            if timed_out {
                LaneStatus::TimedOut
            } else {
                LaneStatus::Blocked
            },
            bytes,
        ))
    })();
    let (status, mut bytes) =
        result.unwrap_or_else(|error| (LaneStatus::Blocked, error.message.into_bytes()));
    if matches!(
        status,
        LaneStatus::Failed | LaneStatus::TimedOut | LaneStatus::Blocked
    ) {
        cancelled.store(true, Ordering::SeqCst);
    }
    let mut redactions_applied = 0;
    let mut text = String::from_utf8_lossy(&bytes).into_owned();
    for secret in &lane.evidence.redact_values {
        if !secret.is_empty() && text.contains(secret) {
            text = text.replace(secret, "[REDACTED]");
            redactions_applied += 1;
        }
    }
    bytes = text.into_bytes();
    bytes.truncate(lane.evidence.max_log_bytes);
    let log_name = format!("{}.log", lane.id);
    let path = evidence_dir.join(&log_name);
    let path_ok = clean_relative(Path::new(&log_name));
    let log_ref = if fs::write(&path, &bytes).is_ok() {
        Some(log_name)
    } else {
        None
    };
    LaneEvidence {
        lane: lane.id.clone(),
        command,
        purpose: lane.purpose.clone(),
        status,
        duration_ms: started.elapsed().as_millis(),
        log_ref,
        redaction_ok: true,
        redactions_applied,
        path_hygiene_ok: path_ok,
    }
}

fn drain(mut reader: impl Read, limit: usize) -> Vec<u8> {
    let mut kept = Vec::new();
    let mut buf = [0u8; 8192];
    while let Ok(n) = reader.read(&mut buf) {
        if n == 0 {
            break;
        }
        let remaining = limit.saturating_sub(kept.len());
        kept.extend_from_slice(&buf[..n.min(remaining)]);
    }
    kept
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ScheduleInput {
    pub phase_ready: bool,
    pub cards_ready: bool,
    pub design_ready: bool,
    pub dependencies_ready: bool,
    pub claim_live: bool,
    pub paths_clear: bool,
    pub budget_available: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ScheduleReport {
    pub schema: String,
    pub eligible_operations: Vec<String>,
    pub blockers: Vec<String>,
    pub authority: String,
}
pub fn classify_schedule(input: &ScheduleInput) -> ScheduleReport {
    let checks = [
        ("phase", input.phase_ready),
        ("cards", input.cards_ready),
        ("design", input.design_ready),
        ("dependencies", input.dependencies_ready),
        ("claim", input.claim_live),
        ("paths", input.paths_clear),
        ("budget", input.budget_available),
    ];
    let blockers = checks
        .iter()
        .filter(|(_, ok)| !*ok)
        .map(|(name, _)| name.to_string())
        .collect::<Vec<_>>();
    ScheduleReport {
        schema: "csdlc.scheduler.report.v1".into(),
        eligible_operations: if blockers.is_empty() {
            vec!["validate".into()]
        } else {
            vec![]
        },
        blockers,
        authority: "read_only; cannot claim, execute, publish, merge, or close".into(),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ShepherdInput {
    pub validation: Option<ValidationDisposition>,
    pub dependency_wait: bool,
    pub retryable_failure: bool,
    pub repair_needed: bool,
    pub operator_decision_needed: bool,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Display)]
#[serde(rename_all = "snake_case")]
pub enum ShepherdState {
    Ready,
    Waiting,
    Retryable,
    RepairRequired,
    OperatorRequired,
}
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ShepherdReport {
    pub schema: String,
    pub state: ShepherdState,
    pub authority: String,
}
pub fn classify_shepherd(input: &ShepherdInput) -> ShepherdReport {
    let state = if input.operator_decision_needed {
        ShepherdState::OperatorRequired
    } else if input.repair_needed {
        ShepherdState::RepairRequired
    } else if input.retryable_failure {
        ShepherdState::Retryable
    } else if input.dependency_wait || input.validation.is_none() {
        ShepherdState::Waiting
    } else {
        ShepherdState::Ready
    };
    ShepherdReport {
        schema: "csdlc.shepherd.report.v1".into(),
        state,
        authority: "observe only; cannot edit, execute, publish, merge, or close".into(),
    }
}

pub fn clean_relative(path: &Path) -> bool {
    path.components()
        .all(|part| matches!(part, Component::Normal(_)))
}
