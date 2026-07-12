//! Per-Godel-agent snapshot and diff protocol for CSM continuity.

use crate::long_lived_agent::{
    load_spec, AgentCheckpointSpec, AgentSpec, AgentStatusState, LoadedAgentSpec, StatusRecord,
};
use anyhow::{anyhow, bail, Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::fs::{self, OpenOptions};
use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;

pub const GODEL_AGENT_SNAPSHOT_SCHEMA: &str = "adl.csm.godel_agent_snapshot.v1";
pub const GODEL_AGENT_DIFF_SCHEMA: &str = "adl.csm.godel_agent_diff.v1";
pub const GODEL_AGENT_CHAIN_SCHEMA: &str = "adl.csm.godel_agent_snapshot_chain.v1";
pub const GODEL_AGENT_PROOF_SCHEMA: &str = "adl.csm.godel_agent_snapshot_diff_proof.v1";

const FORMAT_VERSION: &str = "godel-agent-state.v1";
const POINTER_REF: &str = "godel_snapshots/godel_agent_snapshot_chain.json";
const SNAPSHOT_DIR_REF: &str = "godel_snapshots/snapshots";
const DIFF_DIR_REF: &str = "godel_snapshots/diffs";
const CHAIN_LOCK_RETRY_COUNT: usize = 400;
const CHAIN_LOCK_RETRY_SLEEP_MS: u64 = 25;
const CHAIN_LOCK_STALE_AFTER_SECS: u64 = 120;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GodelSnapshotProtocol {
    pub format_version: String,
    pub compatible_read_versions: Vec<String>,
    pub migration_policy: String,
    pub cadence_policy: GodelSnapshotCadencePolicy,
    pub backpressure_policy: GodelSnapshotBackpressurePolicy,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GodelSnapshotCadencePolicy {
    pub trigger: String,
    pub checkpoint_interval_secs: u64,
    pub min_agent_request_interval_secs: u64,
    pub writes_base_snapshot_when_chain_missing: bool,
    pub writes_diff_after_valid_base: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GodelSnapshotBackpressurePolicy {
    pub csm_queue: String,
    pub policy: String,
    pub failure_mode: String,
    pub low_disk_behavior: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GodelAgentStateProjection {
    pub agent_instance_id: String,
    pub display_name: String,
    pub workflow_kind: String,
    pub workflow_name: Option<String>,
    pub workflow_path_ref: Option<String>,
    pub declared_capabilities: Vec<String>,
    pub active_run_position: GodelRunPosition,
    pub memory_refs: Vec<String>,
    pub local_deltas: Vec<String>,
    pub recovery_hints: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GodelRunPosition {
    pub state: String,
    pub latest_cycle_id: Option<String>,
    pub latest_cycle_status: Option<String>,
    pub completed_cycle_count: u64,
    pub consecutive_failure_count: u64,
    pub stop_requested: bool,
    pub lease_state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GodelAgentSnapshot {
    pub schema: String,
    pub format_version: String,
    pub agent_instance_id: String,
    pub snapshot_id: String,
    pub snapshot_ref: String,
    pub captured_at: DateTime<Utc>,
    pub protocol: GodelSnapshotProtocol,
    pub state_projection: GodelAgentStateProjection,
    pub continuity_checkpoint_ref: String,
    pub continuity_replay_manifest_ref: String,
    pub content_sha256: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GodelAgentDiff {
    pub schema: String,
    pub format_version: String,
    pub agent_instance_id: String,
    pub diff_id: String,
    pub diff_ref: String,
    pub base_snapshot_ref: String,
    pub base_snapshot_sha256: String,
    pub previous_diff_ref: Option<String>,
    pub previous_diff_sha256: Option<String>,
    pub captured_at: DateTime<Utc>,
    pub protocol: GodelSnapshotProtocol,
    pub state_projection: GodelAgentStateProjection,
    pub delta_summary: Vec<String>,
    pub continuity_checkpoint_ref: String,
    pub continuity_replay_manifest_ref: String,
    pub content_sha256: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GodelAgentSnapshotChain {
    pub schema: String,
    pub format_version: String,
    pub agent_instance_id: String,
    pub updated_at: DateTime<Utc>,
    pub base_snapshot_ref: String,
    pub last_diff_ref: Option<String>,
    pub last_known_good_ref: String,
    pub last_known_good_sha256: String,
    pub chain_length: u64,
    pub compaction_policy: String,
    pub compacted_through_diff_ref: Option<String>,
    pub recovery_relevance: String,
    pub observability: GodelSnapshotObservability,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GodelSnapshotObservability {
    pub cadence_state: String,
    pub snapshot_lag_secs: u64,
    pub failure_count: u64,
    pub last_write_kind: String,
    pub last_write_ref: String,
    pub backpressure_queue: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GodelSnapshotWriteResult {
    pub write_kind: String,
    pub artifact_ref: String,
    pub chain_ref: String,
    pub last_known_good_ref: String,
    pub last_known_good_sha256: String,
    pub validation_status: String,
}

#[derive(Debug, Clone)]
pub struct GodelSnapshotProofOptions {
    pub out_dir: PathBuf,
    pub spec_path: Option<PathBuf>,
    pub run_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GodelSnapshotProofReport {
    pub schema: String,
    pub run_id: String,
    pub generated_at: DateTime<Utc>,
    pub integrated_surfaces: Vec<String>,
    pub follow_on_surfaces: Vec<String>,
    pub positive_case: GodelSnapshotPositiveCase,
    pub negative_cases: Vec<GodelSnapshotNegativeCase>,
    pub validation_commands: Vec<String>,
    pub non_claims: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GodelSnapshotPositiveCase {
    pub base_snapshot_ref: String,
    pub diff_ref: String,
    pub chain_ref: String,
    pub recovery_read_status: String,
    pub last_known_good_ref: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GodelSnapshotNegativeCase {
    pub case_id: String,
    pub status: String,
    pub expected_error_contains: String,
    pub observed_error: String,
}

pub fn write_checkpoint_snapshot_diff(
    loaded: &LoadedAgentSpec,
    status: &StatusRecord,
    checkpoint_reason: &str,
    checkpoint_interval_secs: u64,
) -> Result<GodelSnapshotWriteResult> {
    let _lock = acquire_chain_lock(&loaded.state_root)?;
    fs::create_dir_all(snapshot_root(loaded))?;
    fs::create_dir_all(snapshot_dir(loaded))?;
    fs::create_dir_all(diff_dir(loaded))?;

    let chain_path = pointer_path(loaded);
    let result = if chain_path.exists() {
        match write_diff(loaded, status, checkpoint_reason, checkpoint_interval_secs) {
            Ok(result) => result,
            Err(err) if is_godel_chain_integrity_error(&err) => {
                quarantine_corrupt_chain(loaded, &err)?;
                write_base_snapshot(loaded, status, checkpoint_reason, checkpoint_interval_secs)?
            }
            Err(err) => return Err(err),
        }
    } else {
        write_base_snapshot(loaded, status, checkpoint_reason, checkpoint_interval_secs)?
    };
    validate_chain_at(&loaded.state_root)?;
    Ok(result)
}

pub fn prove_godel_snapshot_diff(
    options: GodelSnapshotProofOptions,
) -> Result<GodelSnapshotProofReport> {
    fs::create_dir_all(&options.out_dir)?;
    let spec_path = proof_spec_path(&options)?;
    let loaded = load_spec(&spec_path)?;
    fs::create_dir_all(&loaded.state_root)?;

    let first = status_record(
        &loaded,
        AgentStatusState::RunningCycle,
        Some("cycle-000001"),
        1,
    );
    let base = write_checkpoint_snapshot_diff(&loaded, &first, "proof_base_snapshot", 3)?;
    let second = status_record(&loaded, AgentStatusState::Idle, Some("cycle-000002"), 2);
    let diff = write_checkpoint_snapshot_diff(&loaded, &second, "proof_diff", 3)?;
    let chain = read_chain(&loaded.state_root)?;
    validate_recovery_read(&loaded.state_root)?;

    let negative_cases = run_negative_cases(&options.out_dir, &spec_path)?;
    let report = GodelSnapshotProofReport {
        schema: GODEL_AGENT_PROOF_SCHEMA.to_string(),
        run_id: options.run_id,
        generated_at: Utc::now(),
        integrated_surfaces: vec![
            "csm continuity checkpoint writer".to_string(),
            "continuity_checkpoint.json godel_agent_snapshot_diff pointer".to_string(),
            "continuity_replay_manifest.json recovery reviewer pointer".to_string(),
            "csm backpressure snapshot_diff queue policy".to_string(),
            "standalone csm godel-snapshot proof command".to_string(),
        ],
        follow_on_surfaces: vec![
            "future non-CSM Godel agent types must opt into this protocol before coverage is claimed"
                .to_string(),
            "live distributed consensus and kill -9 mid-write durability remain outside this local atomic-write proof"
                .to_string(),
        ],
        positive_case: GodelSnapshotPositiveCase {
            base_snapshot_ref: base.artifact_ref,
            diff_ref: diff.artifact_ref,
            chain_ref: POINTER_REF.to_string(),
            recovery_read_status: "validated_last_known_good".to_string(),
            last_known_good_ref: chain.last_known_good_ref,
        },
        negative_cases,
        validation_commands: vec![
            "cargo test --manifest-path adl/Cargo.toml csm_godel_snapshot -- --nocapture"
                .to_string(),
            "csm godel-snapshot proof --out <proof-dir> --json".to_string(),
            "git diff --check".to_string(),
        ],
        non_claims: vec![
            "not_secret_material_capture".to_string(),
            "not_full_prompt_persistence".to_string(),
            "not_all_future_agent_types".to_string(),
            "not_distributed_consensus_checkpointing".to_string(),
        ],
    };
    write_json_atomic(
        &options.out_dir.join("godel_snapshot_diff_proof.json"),
        &report,
    )?;
    Ok(report)
}

pub fn validate_chain_at(state_root: &Path) -> Result<GodelAgentSnapshotChain> {
    let chain = read_chain(state_root)?;
    validate_ref(&chain.base_snapshot_ref, "chain.base_snapshot_ref")?;
    validate_ref(&chain.last_known_good_ref, "chain.last_known_good_ref")?;
    if chain.schema != GODEL_AGENT_CHAIN_SCHEMA {
        bail!("unsupported Godel snapshot chain schema '{}'", chain.schema);
    }
    if chain.format_version != FORMAT_VERSION {
        bail!(
            "unsupported Godel snapshot chain format_version '{}'",
            chain.format_version
        );
    }
    let base: GodelAgentSnapshot = read_ref_json(state_root, &chain.base_snapshot_ref)?;
    validate_snapshot(&base)?;
    if base.agent_instance_id != chain.agent_instance_id {
        bail!("Godel snapshot chain agent id must match base snapshot");
    }
    let base_hash = stable_hash_without_content_sha256(&base)?;
    if base.content_sha256 != base_hash {
        bail!("Godel base snapshot content hash mismatch");
    }

    let mut last_ref = chain.base_snapshot_ref.clone();
    let mut last_hash = base.content_sha256.clone();
    let mut length = 1u64;
    let mut seen = BTreeSet::new();
    while let Some(diff_ref) = next_diff_ref(state_root, &chain.agent_instance_id, length)? {
        if !seen.insert(diff_ref.clone()) {
            bail!("Godel diff chain contains a repeated diff ref");
        }
        let diff: GodelAgentDiff = read_ref_json(state_root, &diff_ref)?;
        validate_diff(&diff)?;
        if diff.agent_instance_id != chain.agent_instance_id {
            bail!("Godel diff chain agent id mismatch");
        }
        if diff.base_snapshot_ref != chain.base_snapshot_ref {
            bail!("Godel diff base snapshot ref mismatch");
        }
        if diff.base_snapshot_sha256 != base.content_sha256 {
            bail!("Godel diff base snapshot hash mismatch");
        }
        if diff
            .previous_diff_ref
            .as_deref()
            .unwrap_or(&chain.base_snapshot_ref)
            != last_ref
        {
            bail!("Godel diff previous ref does not match chain predecessor");
        }
        if diff
            .previous_diff_sha256
            .as_deref()
            .unwrap_or(&base.content_sha256)
            != last_hash
        {
            bail!("Godel diff previous hash does not match chain predecessor");
        }
        let diff_hash = stable_hash_without_content_sha256(&diff)?;
        if diff.content_sha256 != diff_hash {
            bail!("Godel diff content hash mismatch");
        }
        last_ref = diff_ref;
        last_hash = diff.content_sha256;
        length += 1;
    }
    if chain.chain_length != length {
        bail!("Godel snapshot chain length mismatch");
    }
    if chain.last_known_good_ref != last_ref || chain.last_known_good_sha256 != last_hash {
        bail!("Godel last-known-good pointer mismatch");
    }
    if let Some(last_diff_ref) = &chain.last_diff_ref {
        if last_diff_ref != &last_ref {
            bail!("Godel chain last diff ref mismatch");
        }
    } else if chain.chain_length != 1 {
        bail!("Godel chain missing last diff ref");
    }
    Ok(chain)
}

pub fn validate_recovery_read(state_root: &Path) -> Result<Value> {
    let _lock = acquire_chain_lock(state_root)?;
    let chain = validate_chain_at(state_root)?;
    let artifact: Value = read_ref_json(state_root, &chain.last_known_good_ref)?;
    Ok(json!({
        "schema": "adl.csm.godel_agent_recovery_read.v1",
        "status": "validated_last_known_good",
        "last_known_good_ref": chain.last_known_good_ref,
        "last_known_good_sha256": chain.last_known_good_sha256,
        "artifact": artifact
    }))
}

struct GodelChainLock {
    path: PathBuf,
}

impl Drop for GodelChainLock {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.path);
    }
}

fn acquire_chain_lock(state_root: &Path) -> Result<GodelChainLock> {
    let root = state_root.join("godel_snapshots");
    fs::create_dir_all(&root)?;
    let path = root.join(".godel_snapshot_chain.lock");
    for _ in 0..CHAIN_LOCK_RETRY_COUNT {
        match OpenOptions::new().write(true).create_new(true).open(&path) {
            Ok(mut file) => {
                use std::io::Write as _;
                writeln!(file, "pid={}", std::process::id())?;
                writeln!(file, "acquired_at={}", Utc::now().to_rfc3339())?;
                return Ok(GodelChainLock { path });
            }
            Err(err) if err.kind() == std::io::ErrorKind::AlreadyExists => {
                if remove_stale_chain_lock(&path)? {
                    continue;
                }
                thread::sleep(Duration::from_millis(CHAIN_LOCK_RETRY_SLEEP_MS));
            }
            Err(err) => {
                return Err(err).with_context(|| {
                    format!("acquire Godel snapshot chain lock {}", path.display())
                });
            }
        }
    }
    Err(anyhow!(
        "timed out acquiring Godel snapshot chain lock {}",
        path.display()
    ))
}

fn remove_stale_chain_lock(path: &Path) -> Result<bool> {
    let Ok(metadata) = fs::metadata(path) else {
        return Ok(false);
    };
    let Ok(modified) = metadata.modified() else {
        return Ok(false);
    };
    let Ok(age) = modified.elapsed() else {
        return Ok(false);
    };
    if age < Duration::from_secs(CHAIN_LOCK_STALE_AFTER_SECS) {
        return Ok(false);
    }
    match fs::remove_file(path) {
        Ok(()) => Ok(true),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(true),
        Err(err) => Err(err)
            .with_context(|| format!("remove stale Godel snapshot chain lock {}", path.display())),
    }
}

fn write_base_snapshot(
    loaded: &LoadedAgentSpec,
    status: &StatusRecord,
    checkpoint_reason: &str,
    checkpoint_interval_secs: u64,
) -> Result<GodelSnapshotWriteResult> {
    let snapshot_ref = format!(
        "{SNAPSHOT_DIR_REF}/{}-snapshot-000001.json",
        loaded.spec.agent_instance_id
    );
    let mut snapshot = GodelAgentSnapshot {
        schema: GODEL_AGENT_SNAPSHOT_SCHEMA.to_string(),
        format_version: FORMAT_VERSION.to_string(),
        agent_instance_id: loaded.spec.agent_instance_id.clone(),
        snapshot_id: format!("{}-snapshot-000001", loaded.spec.agent_instance_id),
        snapshot_ref: snapshot_ref.clone(),
        captured_at: Utc::now(),
        protocol: protocol(&loaded.spec.checkpoint, checkpoint_interval_secs),
        state_projection: state_projection(&loaded.spec, status, checkpoint_reason),
        continuity_checkpoint_ref: "continuity_checkpoint.json".to_string(),
        continuity_replay_manifest_ref: "continuity_replay_manifest.json".to_string(),
        content_sha256: String::new(),
    };
    validate_snapshot(&snapshot)?;
    snapshot.content_sha256 = stable_hash_without_content_sha256(&snapshot)?;
    write_json_atomic(&loaded.state_root.join(&snapshot_ref), &snapshot)?;
    let chain = chain_for(
        loaded,
        None,
        snapshot_ref.clone(),
        snapshot.content_sha256.clone(),
        1,
        "base_snapshot",
    );
    write_json_atomic(&pointer_path(loaded), &chain)?;
    Ok(write_result("base_snapshot", snapshot_ref, &chain))
}

fn is_godel_chain_integrity_error(err: &anyhow::Error) -> bool {
    let message = format!("{err:#}");
    (message.contains("Godel ")
        && (message.contains("mismatch")
            || message.contains("unsupported")
            || message.contains("missing")
            || message.contains("parse")))
        || ((message.contains("read ") || message.contains("parse "))
            && (message.contains(POINTER_REF)
                || message.contains(SNAPSHOT_DIR_REF)
                || message.contains(DIFF_DIR_REF)))
}

fn quarantine_corrupt_chain(loaded: &LoadedAgentSpec, err: &anyhow::Error) -> Result<()> {
    let root = snapshot_root(loaded);
    fs::create_dir_all(&root)?;
    let nonce = Utc::now()
        .to_rfc3339_opts(chrono::SecondsFormat::Micros, true)
        .replace([':', '.'], "-");
    let quarantine_ref = format!("quarantine/recovered-{nonce}");
    let quarantine_dir = root.join(&quarantine_ref);
    fs::create_dir_all(&quarantine_dir)?;

    let mut preserved_refs = Vec::new();
    for name in ["godel_agent_snapshot_chain.json", "snapshots", "diffs"] {
        let src = root.join(name);
        if src.exists() {
            let dst = quarantine_dir.join(name);
            fs::rename(&src, &dst).with_context(|| {
                format!(
                    "quarantine corrupt Godel snapshot chain {} -> {}",
                    src.display(),
                    dst.display()
                )
            })?;
            preserved_refs.push(format!("godel_snapshots/{quarantine_ref}/{name}"));
        }
    }

    let manifest = json!({
        "schema": "adl.csm.godel_snapshot_quarantine.v1",
        "agent_instance_id": loaded.spec.agent_instance_id,
        "quarantined_at": Utc::now(),
        "reason": format!("{err:#}"),
        "action": "preserve_corrupt_chain_and_start_fresh_base_snapshot",
        "preserved_refs": preserved_refs,
        "new_chain_policy": "next_checkpoint_writes_base_snapshot",
        "non_claims": [
            "does_not_delete_corrupt_chain_evidence",
            "does_not_treat_corrupt_snapshot_as_recovered_state"
        ]
    });
    write_json_atomic(&quarantine_dir.join("quarantine_manifest.json"), &manifest)?;
    Ok(())
}

fn write_diff(
    loaded: &LoadedAgentSpec,
    status: &StatusRecord,
    checkpoint_reason: &str,
    checkpoint_interval_secs: u64,
) -> Result<GodelSnapshotWriteResult> {
    let chain = validate_chain_at(&loaded.state_root)?;
    let sequence = chain.chain_length;
    let diff_ref = format!(
        "{DIFF_DIR_REF}/{}-diff-{sequence:06}.json",
        loaded.spec.agent_instance_id
    );
    let base: GodelAgentSnapshot = read_ref_json(&loaded.state_root, &chain.base_snapshot_ref)?;
    let mut diff = GodelAgentDiff {
        schema: GODEL_AGENT_DIFF_SCHEMA.to_string(),
        format_version: FORMAT_VERSION.to_string(),
        agent_instance_id: loaded.spec.agent_instance_id.clone(),
        diff_id: format!("{}-diff-{sequence:06}", loaded.spec.agent_instance_id),
        diff_ref: diff_ref.clone(),
        base_snapshot_ref: chain.base_snapshot_ref.clone(),
        base_snapshot_sha256: base.content_sha256.clone(),
        previous_diff_ref: chain
            .last_diff_ref
            .clone()
            .or(Some(chain.base_snapshot_ref.clone())),
        previous_diff_sha256: Some(chain.last_known_good_sha256.clone()),
        captured_at: Utc::now(),
        protocol: protocol(&loaded.spec.checkpoint, checkpoint_interval_secs),
        state_projection: state_projection(&loaded.spec, status, checkpoint_reason),
        delta_summary: delta_summary(status, checkpoint_reason),
        continuity_checkpoint_ref: "continuity_checkpoint.json".to_string(),
        continuity_replay_manifest_ref: "continuity_replay_manifest.json".to_string(),
        content_sha256: String::new(),
    };
    validate_diff(&diff)?;
    diff.content_sha256 = stable_hash_without_content_sha256(&diff)?;
    write_json_atomic(&loaded.state_root.join(&diff_ref), &diff)?;
    let updated = chain_for(
        loaded,
        Some(diff_ref.clone()),
        diff_ref.clone(),
        diff.content_sha256.clone(),
        chain.chain_length + 1,
        "diff",
    )
    .with_base(chain.base_snapshot_ref, chain.compacted_through_diff_ref);
    write_json_atomic(&pointer_path(loaded), &updated)?;
    Ok(write_result("diff", diff_ref, &updated))
}

fn chain_for(
    loaded: &LoadedAgentSpec,
    last_diff_ref: Option<String>,
    last_known_good_ref: String,
    last_known_good_sha256: String,
    chain_length: u64,
    last_write_kind: &str,
) -> GodelAgentSnapshotChain {
    let base_snapshot_ref = if chain_length == 1 {
        last_known_good_ref.clone()
    } else {
        String::new()
    };
    GodelAgentSnapshotChain {
        schema: GODEL_AGENT_CHAIN_SCHEMA.to_string(),
        format_version: FORMAT_VERSION.to_string(),
        agent_instance_id: loaded.spec.agent_instance_id.clone(),
        updated_at: Utc::now(),
        base_snapshot_ref,
        last_diff_ref,
        last_known_good_ref: last_known_good_ref.clone(),
        last_known_good_sha256,
        chain_length,
        compaction_policy:
            "compact by writing a fresh base snapshot after 64 diffs or before schema migration"
                .to_string(),
        compacted_through_diff_ref: None,
        recovery_relevance:
            "CSM recovery readers use last_known_good_ref after validating the hash chain"
                .to_string(),
        observability: GodelSnapshotObservability {
            cadence_state: "on_checkpoint_cadence".to_string(),
            snapshot_lag_secs: 0,
            failure_count: 0,
            last_write_kind: last_write_kind.to_string(),
            last_write_ref: last_known_good_ref,
            backpressure_queue: "snapshot_diff".to_string(),
        },
    }
}

impl GodelAgentSnapshotChain {
    fn with_base(
        mut self,
        base_snapshot_ref: String,
        compacted_through_diff_ref: Option<String>,
    ) -> Self {
        self.base_snapshot_ref = base_snapshot_ref;
        self.compacted_through_diff_ref = compacted_through_diff_ref;
        self
    }
}

fn write_result(
    write_kind: &str,
    artifact_ref: String,
    chain: &GodelAgentSnapshotChain,
) -> GodelSnapshotWriteResult {
    GodelSnapshotWriteResult {
        write_kind: write_kind.to_string(),
        artifact_ref,
        chain_ref: POINTER_REF.to_string(),
        last_known_good_ref: chain.last_known_good_ref.clone(),
        last_known_good_sha256: chain.last_known_good_sha256.clone(),
        validation_status: "passed".to_string(),
    }
}

fn protocol(
    checkpoint: &AgentCheckpointSpec,
    checkpoint_interval_secs: u64,
) -> GodelSnapshotProtocol {
    GodelSnapshotProtocol {
        format_version: FORMAT_VERSION.to_string(),
        compatible_read_versions: vec![FORMAT_VERSION.to_string()],
        migration_policy:
            "same-major v1 readers are compatible; v2+ requires explicit migration before recovery"
                .to_string(),
        cadence_policy: GodelSnapshotCadencePolicy {
            trigger: "csm_continuity_checkpoint".to_string(),
            checkpoint_interval_secs,
            min_agent_request_interval_secs: checkpoint.min_request_interval_secs.unwrap_or(30),
            writes_base_snapshot_when_chain_missing: true,
            writes_diff_after_valid_base: true,
        },
        backpressure_policy: GodelSnapshotBackpressurePolicy {
            csm_queue: "snapshot_diff".to_string(),
            policy: "defer_latest_only_under_backpressure".to_string(),
            failure_mode: "fail_checkpoint_validation_if_chain_cannot_validate".to_string(),
            low_disk_behavior:
                "preserve last-known-good pointer; do not advance chain on interrupted write"
                    .to_string(),
        },
    }
}

fn state_projection(
    spec: &AgentSpec,
    status: &StatusRecord,
    checkpoint_reason: &str,
) -> GodelAgentStateProjection {
    GodelAgentStateProjection {
        agent_instance_id: spec.agent_instance_id.clone(),
        display_name: spec.display_name.clone(),
        workflow_kind: spec.workflow.kind.clone(),
        workflow_name: spec.workflow.name.clone(),
        workflow_path_ref: spec
            .workflow
            .path
            .as_ref()
            .map(|path| path.to_string_lossy().to_string())
            .filter(|value| !value.contains("..") && !Path::new(value).is_absolute()),
        declared_capabilities: declared_capabilities(spec),
        active_run_position: GodelRunPosition {
            state: serde_json::to_string(&status.state)
                .unwrap_or_else(|_| "\"unknown\"".to_string())
                .trim_matches('"')
                .to_string(),
            latest_cycle_id: status.last_cycle_id.clone(),
            latest_cycle_status: status.last_cycle_status.clone(),
            completed_cycle_count: status.completed_cycle_count,
            consecutive_failure_count: status.consecutive_failure_count,
            stop_requested: status.stop_requested,
            lease_state: if status.active_lease.is_some() {
                "active".to_string()
            } else {
                "clear".to_string()
            },
        },
        memory_refs: memory_refs(&spec.memory),
        local_deltas: vec![
            format!("checkpoint_reason:{checkpoint_reason}"),
            format!("status_state:{:?}", status.state),
        ],
        recovery_hints: recovery_hints(status),
    }
}

fn declared_capabilities(spec: &AgentSpec) -> Vec<String> {
    let mut capabilities = vec![
        "csm_continuity_checkpoint".to_string(),
        "godel_snapshot_diff_v1".to_string(),
        format!("workflow_kind:{}", spec.workflow.kind),
    ];
    if spec.checkpoint.allow_agent_requested {
        capabilities.push("agent_requested_checkpoint".to_string());
    }
    if !spec.memory.is_null() {
        capabilities.push("memory_refs_projected_without_payloads".to_string());
    }
    capabilities
}

fn memory_refs(memory: &Value) -> Vec<String> {
    let mut refs = Vec::new();
    collect_memory_refs(memory, "$", &mut refs);
    refs.sort();
    refs.dedup();
    refs.truncate(32);
    refs
}

fn collect_memory_refs(value: &Value, path: &str, refs: &mut Vec<String>) {
    match value {
        Value::Object(map) => {
            for (key, child) in map {
                let lowered = key.to_ascii_lowercase();
                if looks_like_secret_key(&lowered) {
                    refs.push(format!("{path}.{key}:redacted_key"));
                } else {
                    collect_memory_refs(child, &format!("{path}.{key}"), refs);
                }
            }
        }
        Value::Array(items) => {
            for (index, child) in items.iter().enumerate() {
                collect_memory_refs(child, &format!("{path}[{index}]"), refs);
            }
        }
        Value::String(raw) if looks_like_ref(raw) => refs.push(raw.clone()),
        _ => {}
    }
}

fn looks_like_ref(raw: &str) -> bool {
    !raw.contains('\n')
        && !raw.contains("://")
        && !raw.starts_with('/')
        && !looks_like_secret_value(raw)
        && (raw.ends_with(".json")
            || raw.ends_with(".jsonl")
            || raw.ends_with(".yaml")
            || raw.contains('/'))
}

fn looks_like_secret_key(key: &str) -> bool {
    [
        "secret",
        "token",
        "password",
        "prompt",
        "api_key",
        "apikey",
        "authorization",
        "credential",
        "credentials",
        "bearer",
        "private_key",
    ]
    .iter()
    .any(|marker| key.contains(marker))
}

fn looks_like_secret_value(raw: &str) -> bool {
    let lowered = raw.to_ascii_lowercase();
    lowered.contains("bearer ")
        || lowered.contains("api_key=")
        || lowered.contains("apikey=")
        || lowered.contains("authorization:")
        || lowered.contains("private_key")
        || lowered.contains("credential")
        || lowered.contains("secret")
        || lowered.contains("token=")
        || lowered.contains("password")
}

fn recovery_hints(status: &StatusRecord) -> Vec<String> {
    match status.state {
        AgentStatusState::Failed if status.active_lease.is_none() => vec![
            "recover_from_last_known_good_after_operator_review".to_string(),
            "inspect_continuity_replay_manifest_before_resume".to_string(),
        ],
        AgentStatusState::RunningCycle => {
            vec!["resume_requires_lease_review_before_duplicate_active_cycle".to_string()]
        }
        _ => vec!["resume_from_validated_last_known_good_pointer".to_string()],
    }
}

fn delta_summary(status: &StatusRecord, checkpoint_reason: &str) -> Vec<String> {
    vec![
        format!("checkpoint_reason:{checkpoint_reason}"),
        format!(
            "latest_cycle_id:{}",
            status.last_cycle_id.as_deref().unwrap_or("none")
        ),
        format!(
            "latest_cycle_status:{}",
            status.last_cycle_status.as_deref().unwrap_or("none")
        ),
        format!("completed_cycle_count:{}", status.completed_cycle_count),
    ]
}

fn validate_snapshot(snapshot: &GodelAgentSnapshot) -> Result<()> {
    if snapshot.schema != GODEL_AGENT_SNAPSHOT_SCHEMA {
        bail!("unsupported Godel snapshot schema '{}'", snapshot.schema);
    }
    validate_common(
        &snapshot.format_version,
        &snapshot.agent_instance_id,
        &snapshot.snapshot_ref,
        &snapshot.protocol,
        &snapshot.state_projection,
    )
}

fn validate_diff(diff: &GodelAgentDiff) -> Result<()> {
    if diff.schema != GODEL_AGENT_DIFF_SCHEMA {
        bail!("unsupported Godel diff schema '{}'", diff.schema);
    }
    validate_ref(&diff.base_snapshot_ref, "diff.base_snapshot_ref")?;
    validate_ref(&diff.diff_ref, "diff.diff_ref")?;
    if let Some(previous) = &diff.previous_diff_ref {
        validate_ref(previous, "diff.previous_diff_ref")?;
    }
    if diff.base_snapshot_sha256.trim().is_empty() {
        bail!("Godel diff requires base snapshot hash");
    }
    validate_common(
        &diff.format_version,
        &diff.agent_instance_id,
        &diff.diff_ref,
        &diff.protocol,
        &diff.state_projection,
    )
}

fn validate_common(
    format_version: &str,
    agent_instance_id: &str,
    artifact_ref: &str,
    protocol: &GodelSnapshotProtocol,
    projection: &GodelAgentStateProjection,
) -> Result<()> {
    if format_version != FORMAT_VERSION {
        bail!("unsupported Godel snapshot format_version '{format_version}'");
    }
    validate_ref(artifact_ref, "artifact_ref")?;
    if agent_instance_id.trim().is_empty() || projection.agent_instance_id != agent_instance_id {
        bail!("Godel snapshot projection agent id mismatch");
    }
    if protocol.format_version != FORMAT_VERSION
        || !protocol
            .compatible_read_versions
            .iter()
            .any(|version| version == FORMAT_VERSION)
    {
        bail!("Godel snapshot protocol compatibility set does not include v1");
    }
    if protocol.cadence_policy.checkpoint_interval_secs == 0 {
        bail!("Godel snapshot cadence interval must be greater than zero");
    }
    if protocol.backpressure_policy.csm_queue != "snapshot_diff" {
        bail!("Godel snapshot backpressure queue must be snapshot_diff");
    }
    let encoded = serde_json::to_string(projection)?;
    for forbidden in ["BEGIN ", "PRIVATE KEY", "password", "token=", "secret="] {
        if encoded.contains(forbidden) {
            bail!("Godel snapshot projection contains unsafe sensitive material marker");
        }
    }
    Ok(())
}

fn validate_ref(value: &str, field: &str) -> Result<()> {
    if value.trim().is_empty() {
        bail!("{field} must not be empty");
    }
    let path = Path::new(value);
    if path.is_absolute() || value.contains("..") || value.contains('\\') {
        bail!("{field} must be a safe repository-relative artifact ref");
    }
    Ok(())
}

fn stable_hash_without_content_sha256<T>(value: &T) -> Result<String>
where
    T: Serialize,
{
    let mut value = serde_json::to_value(value)?;
    if let Some(object) = value.as_object_mut() {
        object.insert("content_sha256".to_string(), Value::String(String::new()));
    }
    let bytes = serde_json::to_vec(&value)?;
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    Ok(format!("{:x}", hasher.finalize()))
}

fn read_chain(state_root: &Path) -> Result<GodelAgentSnapshotChain> {
    read_json(&state_root.join(POINTER_REF))
}

fn read_ref_json<T>(state_root: &Path, artifact_ref: &str) -> Result<T>
where
    T: for<'de> Deserialize<'de>,
{
    validate_ref(artifact_ref, "artifact_ref")?;
    read_json(&state_root.join(artifact_ref))
}

fn read_json<T>(path: &Path) -> Result<T>
where
    T: for<'de> Deserialize<'de>,
{
    let raw = fs::read_to_string(path).with_context(|| format!("read {}", path.display()))?;
    serde_json::from_str(&raw).with_context(|| format!("parse {}", path.display()))
}

fn write_json_atomic<T>(path: &Path, value: &T) -> Result<()>
where
    T: Serialize,
{
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let tmp = path.with_extension(format!("tmp-{}", std::process::id()));
    fs::write(&tmp, serde_json::to_vec_pretty(value)?)
        .with_context(|| format!("write temp {}", tmp.display()))?;
    fs::rename(&tmp, path).with_context(|| {
        format!(
            "commit atomic Godel snapshot write {} -> {}",
            tmp.display(),
            path.display()
        )
    })
}

fn next_diff_ref(
    state_root: &Path,
    agent_instance_id: &str,
    sequence: u64,
) -> Result<Option<String>> {
    let candidate = format!("{DIFF_DIR_REF}/{agent_instance_id}-diff-{sequence:06}.json");
    if state_root.join(&candidate).exists() {
        Ok(Some(candidate))
    } else {
        Ok(None)
    }
}

fn snapshot_root(loaded: &LoadedAgentSpec) -> PathBuf {
    loaded.state_root.join("godel_snapshots")
}

fn snapshot_dir(loaded: &LoadedAgentSpec) -> PathBuf {
    loaded.state_root.join(SNAPSHOT_DIR_REF)
}

fn diff_dir(loaded: &LoadedAgentSpec) -> PathBuf {
    loaded.state_root.join(DIFF_DIR_REF)
}

fn pointer_path(loaded: &LoadedAgentSpec) -> PathBuf {
    loaded.state_root.join(POINTER_REF)
}

fn write_fixture_spec(out_dir: &Path) -> Result<PathBuf> {
    let spec = out_dir.join("agent.yaml");
    let state_root = "state";
    fs::write(
        &spec,
        format!(
            r#"schema: adl.long_lived_agent_spec.v1
agent_instance_id: godel-agent-4912
display_name: Godel Agent 4912
state_root: {state_root}
workflow:
  kind: adl_workflow
  name: godel_snapshot_diff_fixture
  path: workflow.adl.yaml
heartbeat:
  interval_secs: 3
  max_cycles: 2
checkpoint:
  interval_secs: 3
  allow_agent_requested: true
  min_request_interval_secs: 3
memory:
  refs:
    - memory/index.json
safety:
  prompt_capture: forbidden
"#
        ),
    )?;
    fs::write(
        out_dir.join("workflow.adl.yaml"),
        "schema: adl.v1\nsteps: []\n",
    )?;
    Ok(spec)
}

fn proof_spec_path(options: &GodelSnapshotProofOptions) -> Result<PathBuf> {
    match &options.spec_path {
        Some(input) => write_isolated_spec_copy(input, &options.out_dir),
        None => write_fixture_spec(&options.out_dir),
    }
}

fn write_isolated_spec_copy(input: &Path, out_dir: &Path) -> Result<PathBuf> {
    let mut loaded = load_spec(input)?;
    loaded.spec.state_root = PathBuf::from("state");
    let isolated = out_dir.join("agent.yaml");
    fs::write(&isolated, serde_yaml::to_string(&loaded.spec)?)
        .with_context(|| format!("write isolated proof spec {}", isolated.display()))?;
    Ok(isolated)
}

fn status_record(
    loaded: &LoadedAgentSpec,
    state: AgentStatusState,
    cycle_id: Option<&str>,
    count: u64,
) -> StatusRecord {
    StatusRecord {
        schema: "adl.long_lived_agent_status.v1".to_string(),
        agent_instance_id: loaded.spec.agent_instance_id.clone(),
        state,
        last_cycle_id: cycle_id.map(str::to_string),
        last_cycle_status: Some("completed".to_string()),
        completed_cycle_count: count,
        consecutive_failure_count: 0,
        active_lease: None,
        stop_requested: false,
        last_error: None,
        safety_policy: json!({"prompt_capture": "forbidden"}),
        updated_at: Utc::now(),
    }
}

fn run_negative_cases(out_dir: &Path, spec_path: &Path) -> Result<Vec<GodelSnapshotNegativeCase>> {
    let mut cases = Vec::new();
    for case_id in [
        "malformed_diff",
        "missing_base_snapshot",
        "schema_upgrade",
        "stale_agent",
        "interrupted_write",
    ] {
        let case_dir = out_dir.join("negative").join(case_id);
        fs::create_dir_all(&case_dir)?;
        let case_spec = case_dir.join("agent.yaml");
        fs::copy(spec_path, &case_spec)?;
        let loaded = load_spec(&case_spec)?;
        fs::create_dir_all(&loaded.state_root)?;
        let status = status_record(&loaded, AgentStatusState::Idle, Some("cycle-000001"), 1);
        let _ = write_checkpoint_snapshot_diff(&loaded, &status, "negative_seed", 3)?;
        let expected = mutate_negative_case(&loaded, case_id)?;
        let observed = match validate_chain_at(&loaded.state_root) {
            Ok(_) => "unexpected success".to_string(),
            Err(err) => sanitize_observed_error(&err.to_string(), &case_dir),
        };
        cases.push(GodelSnapshotNegativeCase {
            case_id: case_id.to_string(),
            status: if observed.contains(&expected) {
                "passed".to_string()
            } else {
                "failed".to_string()
            },
            expected_error_contains: expected,
            observed_error: observed,
        });
    }
    Ok(cases)
}

fn mutate_negative_case(loaded: &LoadedAgentSpec, case_id: &str) -> Result<String> {
    let chain = read_chain(&loaded.state_root)?;
    match case_id {
        "malformed_diff" => {
            let status = status_record(loaded, AgentStatusState::Idle, Some("cycle-000002"), 2);
            let diff = write_diff(loaded, &status, "negative_malformed_diff", 3)?;
            fs::write(loaded.state_root.join(diff.artifact_ref), "{")?;
            Ok("parse".to_string())
        }
        "missing_base_snapshot" => {
            fs::remove_file(loaded.state_root.join(chain.base_snapshot_ref))?;
            Ok("read".to_string())
        }
        "schema_upgrade" => {
            let path = loaded.state_root.join(chain.base_snapshot_ref);
            let mut snapshot: Value = read_json(&path)?;
            snapshot["schema"] = json!("adl.csm.godel_agent_snapshot.v2");
            write_json_atomic(&path, &snapshot)?;
            Ok("unsupported Godel snapshot schema".to_string())
        }
        "stale_agent" => {
            let path = loaded.state_root.join(POINTER_REF);
            let mut stale: Value = read_json(&path)?;
            stale["agent_instance_id"] = json!("different-agent");
            write_json_atomic(&path, &stale)?;
            Ok("agent id must match".to_string())
        }
        "interrupted_write" => {
            let tmp = loaded.state_root.join(format!(
                "{DIFF_DIR_REF}/{}-diff-000001.json.tmp",
                loaded.spec.agent_instance_id
            ));
            fs::write(tmp, r#"{"schema":"adl.csm.godel_agent_diff.v1""#)?;
            let path = loaded.state_root.join(POINTER_REF);
            let mut stale: Value = read_json(&path)?;
            stale["chain_length"] = json!(2);
            write_json_atomic(&path, &stale)?;
            Ok("length mismatch".to_string())
        }
        other => Err(anyhow!("unknown negative case {other}")),
    }
}

fn sanitize_observed_error(error: &str, proof_case_dir: &Path) -> String {
    let mut sanitized = error.replace(&proof_case_dir.to_string_lossy().to_string(), "<case-dir>");
    let temp_dir = std::env::temp_dir();
    sanitized = sanitized.replace(&temp_dir.to_string_lossy().to_string(), "<tmp>");
    sanitized
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_dir(name: &str) -> PathBuf {
        let path = std::env::temp_dir().join(format!(
            "adl-csm-godel-snapshot-{name}-{}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&path);
        fs::create_dir_all(&path).expect("create temp dir");
        path
    }

    #[test]
    fn csm_godel_snapshot_writes_base_then_diff_and_validates_recovery() {
        let out = temp_dir("positive");
        let spec = write_fixture_spec(&out).expect("fixture spec");
        let loaded = load_spec(&spec).expect("load spec");
        fs::create_dir_all(&loaded.state_root).expect("state root");
        let first = status_record(
            &loaded,
            AgentStatusState::RunningCycle,
            Some("cycle-000001"),
            1,
        );
        let base = write_checkpoint_snapshot_diff(&loaded, &first, "test_base", 3).expect("base");
        assert_eq!(base.write_kind, "base_snapshot");
        let second = status_record(&loaded, AgentStatusState::Idle, Some("cycle-000002"), 2);
        let diff = write_checkpoint_snapshot_diff(&loaded, &second, "test_diff", 3).expect("diff");
        assert_eq!(diff.write_kind, "diff");
        let recovery = validate_recovery_read(&loaded.state_root).expect("recovery");
        assert_eq!(recovery["status"], "validated_last_known_good");
    }

    #[test]
    fn csm_godel_snapshot_quarantines_corrupt_chain_before_fresh_base() {
        let out = temp_dir("quarantine-corrupt");
        let spec = write_fixture_spec(&out).expect("fixture spec");
        let loaded = load_spec(&spec).expect("load spec");
        fs::create_dir_all(&loaded.state_root).expect("state root");
        let first = status_record(
            &loaded,
            AgentStatusState::RunningCycle,
            Some("cycle-000001"),
            1,
        );
        write_checkpoint_snapshot_diff(&loaded, &first, "test_base", 3).expect("base");
        let snapshot_path = loaded
            .state_root
            .join("godel_snapshots/snapshots")
            .join(format!(
                "{}-snapshot-000001.json",
                loaded.spec.agent_instance_id
            ));
        let mut snapshot: Value =
            serde_json::from_str(&fs::read_to_string(&snapshot_path).expect("read snapshot"))
                .expect("parse snapshot");
        snapshot["state_projection"]["local_deltas"] = json!(["tampered_after_hash"]);
        fs::write(
            &snapshot_path,
            serde_json::to_vec_pretty(&snapshot).expect("encode snapshot"),
        )
        .expect("tamper snapshot");

        let second = status_record(&loaded, AgentStatusState::Idle, Some("cycle-000002"), 2);
        let recovered =
            write_checkpoint_snapshot_diff(&loaded, &second, "recover_after_corruption", 3)
                .expect("quarantine and fresh base");

        assert_eq!(recovered.write_kind, "base_snapshot");
        let chain = read_chain(&loaded.state_root).expect("fresh chain");
        assert_eq!(chain.chain_length, 1);
        assert!(loaded
            .state_root
            .join("godel_snapshots/quarantine")
            .read_dir()
            .expect("quarantine dir exists")
            .any(|entry| entry
                .expect("quarantine entry")
                .path()
                .join("quarantine_manifest.json")
                .exists()));
        validate_recovery_read(&loaded.state_root).expect("fresh chain validates");
    }

    #[test]
    fn csm_godel_snapshot_quarantines_malformed_pointer_before_fresh_base() {
        let out = temp_dir("quarantine-malformed-pointer");
        let spec = write_fixture_spec(&out).expect("fixture spec");
        let loaded = load_spec(&spec).expect("load spec");
        fs::create_dir_all(&loaded.state_root).expect("state root");
        let first = status_record(
            &loaded,
            AgentStatusState::RunningCycle,
            Some("cycle-000001"),
            1,
        );
        write_checkpoint_snapshot_diff(&loaded, &first, "test_base", 3).expect("base");
        fs::write(pointer_path(&loaded), "{").expect("malform pointer");

        let second = status_record(&loaded, AgentStatusState::Idle, Some("cycle-000002"), 2);
        let recovered =
            write_checkpoint_snapshot_diff(&loaded, &second, "recover_after_malformed_pointer", 3)
                .expect("quarantine and fresh base");

        assert_eq!(recovered.write_kind, "base_snapshot");
        let chain = read_chain(&loaded.state_root).expect("fresh chain");
        assert_eq!(chain.chain_length, 1);
        validate_recovery_read(&loaded.state_root).expect("fresh chain validates");
    }

    #[test]
    fn csm_godel_snapshot_quarantines_missing_base_before_fresh_base() {
        let out = temp_dir("quarantine-missing-base");
        let spec = write_fixture_spec(&out).expect("fixture spec");
        let loaded = load_spec(&spec).expect("load spec");
        fs::create_dir_all(&loaded.state_root).expect("state root");
        let first = status_record(
            &loaded,
            AgentStatusState::RunningCycle,
            Some("cycle-000001"),
            1,
        );
        write_checkpoint_snapshot_diff(&loaded, &first, "test_base", 3).expect("base");
        let chain = read_chain(&loaded.state_root).expect("seed chain");
        fs::remove_file(loaded.state_root.join(chain.base_snapshot_ref)).expect("remove base");

        let second = status_record(&loaded, AgentStatusState::Idle, Some("cycle-000002"), 2);
        let recovered =
            write_checkpoint_snapshot_diff(&loaded, &second, "recover_after_missing_base", 3)
                .expect("quarantine and fresh base");

        assert_eq!(recovered.write_kind, "base_snapshot");
        let chain = read_chain(&loaded.state_root).expect("fresh chain");
        assert_eq!(chain.chain_length, 1);
        validate_recovery_read(&loaded.state_root).expect("fresh chain validates");
    }

    #[test]
    fn csm_godel_snapshot_negative_cases_are_proven() {
        let out = temp_dir("negative");
        let out_display = out.to_string_lossy().to_string();
        let report = prove_godel_snapshot_diff(GodelSnapshotProofOptions {
            out_dir: out,
            spec_path: None,
            run_id: "test-run".to_string(),
        })
        .expect("proof");
        assert_eq!(report.negative_cases.len(), 5);
        assert!(report
            .negative_cases
            .iter()
            .all(|case| case.status == "passed"));
        assert!(report
            .negative_cases
            .iter()
            .all(|case| !case.observed_error.contains(&out_display)));
    }

    #[test]
    fn csm_godel_snapshot_proof_spec_is_isolated_from_absolute_state_root() {
        let out = temp_dir("isolated-proof");
        let live_state = out.join("live-state");
        fs::create_dir_all(&live_state).expect("live state");
        let live_marker = live_state.join("must-not-delete.txt");
        fs::write(&live_marker, "live").expect("live marker");
        let spec = out.join("live-agent.yaml");
        fs::write(
            &spec,
            format!(
                r#"schema: adl.long_lived_agent_spec.v1
agent_instance_id: live-godel-agent
display_name: Live Godel Agent
state_root: {}
workflow:
  kind: adl_workflow
  name: live
heartbeat: {{}}
checkpoint: {{}}
safety: {{}}
memory: {{}}
"#,
                live_state.display()
            ),
        )
        .expect("live spec");
        let proof_dir = out.join("proof");
        let report = prove_godel_snapshot_diff(GodelSnapshotProofOptions {
            out_dir: proof_dir.clone(),
            spec_path: Some(spec),
            run_id: "isolated-proof".to_string(),
        })
        .expect("proof");
        assert_eq!(
            report.positive_case.recovery_read_status,
            "validated_last_known_good"
        );
        assert!(live_marker.exists());
        assert!(!live_state.join("godel_snapshots").exists());
        assert!(proof_dir.join("state/godel_snapshots").exists());
    }

    #[test]
    fn csm_godel_snapshot_memory_projection_redacts_secret_like_refs() {
        let out = temp_dir("secret-redaction");
        let spec = out.join("agent.yaml");
        fs::write(
            &spec,
            r#"schema: adl.long_lived_agent_spec.v1
agent_instance_id: redaction-agent
display_name: Redaction Agent
state_root: state
workflow:
  kind: adl_workflow
  name: redaction
heartbeat: {}
checkpoint: {}
safety: {}
memory:
  api_key: path/that/looks/like/ref
  authorization: Bearer abc/def
  safe_ref: memory/index.json
"#,
        )
        .expect("spec");
        let loaded = load_spec(&spec).expect("load");
        fs::create_dir_all(&loaded.state_root).expect("state");
        let status = status_record(&loaded, AgentStatusState::Idle, Some("cycle-000001"), 1);
        write_checkpoint_snapshot_diff(&loaded, &status, "redaction", 1).expect("snapshot");
        let snapshot: GodelAgentSnapshot = read_ref_json(
            &loaded.state_root,
            "godel_snapshots/snapshots/redaction-agent-snapshot-000001.json",
        )
        .expect("snapshot read");
        assert!(snapshot
            .state_projection
            .memory_refs
            .iter()
            .any(|value| value.ends_with("api_key:redacted_key")));
        assert!(snapshot
            .state_projection
            .memory_refs
            .iter()
            .any(|value| value.ends_with("authorization:redacted_key")));
        assert!(snapshot
            .state_projection
            .memory_refs
            .iter()
            .any(|value| value == "memory/index.json"));
        assert!(!snapshot
            .state_projection
            .memory_refs
            .iter()
            .any(|value| value.contains("abc/def")));
    }
}
