//! Resume steering model for execution pause and resume control state.

use std::collections::{HashMap, HashSet};

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

/// Snapshot of a paused execution state and deferred continuation plan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PauseState {
    pub paused_step_id: String,
    pub reason: Option<String>,
    pub completed_step_ids: Vec<String>,
    pub remaining_step_ids: Vec<String>,
    pub saved_state: HashMap<String, String>,
    pub completed_outputs: HashMap<String, String>,
}

/// Mutable state persisted across resume attempts.
#[derive(Debug, Clone, Default)]
pub struct ResumeState {
    pub completed_step_ids: HashSet<String>,
    pub saved_state: HashMap<String, String>,
    pub completed_outputs: HashMap<String, String>,
    pub steering_history: Vec<SteeringRecord>,
}

/// Declarative steering patch applied at the configured resume boundary.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SteeringPatch {
    pub schema_version: String,
    pub apply_at: String,
    #[serde(default)]
    pub reason: Option<String>,
    #[serde(default)]
    pub set_state: HashMap<String, String>,
    #[serde(default)]
    pub remove_state: Vec<String>,
}

/// Applied steering operation record for deterministic audit trails.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SteeringRecord {
    pub sequence: u32,
    pub apply_at: String,
    #[serde(default)]
    pub reason: Option<String>,
    pub payload_fingerprint: String,
    #[serde(default)]
    pub set_state_keys: Vec<String>,
    #[serde(default)]
    pub removed_state_keys: Vec<String>,
}

pub const STEERING_PATCH_SCHEMA_VERSION: &str = "steering_patch.v1";
pub const STEERING_APPLY_AT_RESUME_BOUNDARY: &str = "resume_boundary";

/// Validate a steering patch for schema, application boundary, and basic key rules.
pub fn validate_steering_patch(patch: &SteeringPatch) -> Result<()> {
    if patch.schema_version != STEERING_PATCH_SCHEMA_VERSION {
        return Err(anyhow!(
            "steering patch schema_version mismatch: patch='{}' expected='{}'",
            patch.schema_version,
            STEERING_PATCH_SCHEMA_VERSION
        ));
    }
    if patch.apply_at != STEERING_APPLY_AT_RESUME_BOUNDARY {
        return Err(anyhow!(
            "steering patch apply_at must be '{}' (found '{}')",
            STEERING_APPLY_AT_RESUME_BOUNDARY,
            patch.apply_at
        ));
    }

    let mut remove_set = HashSet::new();
    for key in &patch.remove_state {
        let trimmed = key.trim();
        if trimmed.is_empty() {
            return Err(anyhow!("steering patch remove_state contains an empty key"));
        }
        if !remove_set.insert(trimmed.to_string()) {
            return Err(anyhow!(
                "steering patch remove_state contains duplicate key '{}'",
                trimmed
            ));
        }
    }

    for key in patch.set_state.keys() {
        let trimmed = key.trim();
        if trimmed.is_empty() {
            return Err(anyhow!("steering patch set_state contains an empty key"));
        }
        if remove_set.contains(trimmed) {
            return Err(anyhow!(
                "steering patch key '{}' cannot appear in both set_state and remove_state",
                trimmed
            ));
        }
    }

    if patch.set_state.is_empty() && patch.remove_state.is_empty() {
        return Err(anyhow!(
            "steering patch must set or remove at least one saved-state key"
        ));
    }

    Ok(())
}

/// Build a persisted steering record from an applied steering patch.
pub fn steering_record_from_patch(
    sequence: u32,
    payload_fingerprint: String,
    patch: &SteeringPatch,
) -> SteeringRecord {
    let mut set_state_keys: Vec<String> = patch.set_state.keys().cloned().collect();
    set_state_keys.sort();
    let mut removed_state_keys = patch.remove_state.clone();
    removed_state_keys.sort();
    removed_state_keys.dedup();

    SteeringRecord {
        sequence,
        apply_at: patch.apply_at.clone(),
        reason: patch.reason.clone(),
        payload_fingerprint,
        set_state_keys,
        removed_state_keys,
    }
}

/// Apply a steering patch to resume state and record the resulting deterministic patch history.
pub fn apply_steering_patch(
    resume: &mut ResumeState,
    patch: &SteeringPatch,
    payload_fingerprint: String,
) -> Result<SteeringRecord> {
    validate_steering_patch(patch)?;

    for key in &patch.remove_state {
        resume.saved_state.remove(key.trim());
    }
    for (key, value) in &patch.set_state {
        resume
            .saved_state
            .insert(key.trim().to_string(), value.clone());
    }

    let sequence = u32::try_from(resume.steering_history.len())
        .unwrap_or(u32::MAX)
        .saturating_add(1);
    let record = steering_record_from_patch(sequence, payload_fingerprint, patch);
    resume.steering_history.push(record.clone());
    Ok(record)
}
