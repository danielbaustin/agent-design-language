use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};

use super::DELEGATION_POLICY_DENY_CODE;
use crate::sandbox;
use crate::trace;

pub fn materialize_inputs(
    mut inputs: HashMap<String, String>,
    base_dir: &Path,
) -> Result<HashMap<String, String>> {
    for (k, v) in inputs.iter_mut() {
        let Some(raw) = v.strip_prefix("@file:") else {
            continue;
        };

        let mut path_str = raw.trim();
        if path_str.is_empty() {
            return Err(anyhow!("input '{k}' uses @file: with an empty path"));
        }

        // Allow simple quoting in YAML values: "@file:..." or '@file:...'
        if (path_str.starts_with('"') && path_str.ends_with('"'))
            || (path_str.starts_with('\'') && path_str.ends_with('\''))
        {
            path_str = &path_str[1..path_str.len() - 1];
            path_str = path_str.trim();
        }

        let candidate = PathBuf::from(path_str);
        let path_for_stat = if candidate.is_absolute() {
            candidate.clone()
        } else {
            base_dir.join(&candidate)
        };

        let meta = std::fs::metadata(&path_for_stat).with_context(|| {
            format!(
                "failed to stat input file for '{k}': '{}' (base_dir='{}')",
                path_for_stat.display(),
                base_dir.display()
            )
        })?;
        if !meta.is_file() {
            return Err(anyhow!(
                "input '{k}' references a non-file path: '{}'",
                path_for_stat.display()
            ));
        }
        if meta.len() > MATERIALIZE_INPUT_MAX_FILE_BYTES {
            return Err(anyhow!(
                "input '{k}' file is too large ({} bytes > {} bytes): '{}'",
                meta.len(),
                MATERIALIZE_INPUT_MAX_FILE_BYTES,
                path_for_stat.display()
            ));
        }

        let canon =
            sandbox::resolve_existing_path_within_root(base_dir, &candidate).map_err(|err| {
                let requested = err.requested_path().unwrap_or("sandbox:/<unknown>");
                let resolved = err
                    .resolved_path()
                    .map(|value| format!(" resolved_path={value}"))
                    .unwrap_or_default();
                anyhow!(
                    "input '{k}' file rejected by sandbox resolver: code={} message={} requested_path={}{}",
                    err.code(),
                    err.message(),
                    requested,
                    resolved
                )
            })?;

        let bytes = std::fs::read(&canon).with_context(|| {
            format!("failed to read input file for '{k}': '{}'", canon.display())
        })?;
        let mut text = String::from_utf8(bytes).with_context(|| {
            format!("input '{k}' file is not valid UTF-8: '{}'", canon.display())
        })?;

        // Normalize newlines for stable hashing / traces.
        if text.contains("\r\n") {
            text = text.replace("\r\n", "\n");
        }

        *v = text;
    }

    Ok(inputs)
}

/// Maximum allowed bytes per `@file:` materialized input.
pub const MATERIALIZE_INPUT_MAX_FILE_BYTES: u64 = 512 * 1024;

/// Default concurrency cap for concurrent workflow runs when no override is provided.
pub(crate) const DEFAULT_MAX_CONCURRENCY: usize = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchedulerPolicySource {
    WorkflowOverride,
    RunDefault,
    EngineDefault,
}

impl SchedulerPolicySource {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::WorkflowOverride => "workflow_override",
            Self::RunDefault => "run_default",
            Self::EngineDefault => "engine_default",
        }
    }
}

/// Result of executing one step.
#[allow(dead_code)] // v0.1: returned for callers / future use; not all fields are read yet
#[derive(Debug, Clone)]
pub struct StepOutput {
    pub step_id: String,
    pub provider_id: String,
    pub model_output: String,
}

/// Stable execution telemetry record for one step.
///
/// Records are emitted in deterministic step completion order and are intended
/// for run summaries and machine-readable artifact generation.
#[derive(Debug, Clone)]
pub struct StepExecutionRecord {
    pub step_id: String,
    pub provider_id: String,
    pub status: String,
    pub attempts: u32,
    pub output_bytes: usize,
}

/// Aggregate result from executing a resolved workflow.
///
/// Contains step outputs, generated artifact paths, and per-step records.
#[derive(Debug, Clone)]
pub struct ExecutionResult {
    pub outputs: Vec<StepOutput>,
    pub artifacts: Vec<PathBuf>,
    pub records: Vec<StepExecutionRecord>,
    pub pause: Option<PauseState>,
    pub steering_history: Vec<SteeringRecord>,
    pub runtime_control: RuntimeControlState,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RuntimeControlState {
    pub signals: CognitiveSignalsState,
    pub arbitration: CognitiveArbitrationState,
    pub fast_slow: FastSlowPathState,
    pub agency: AgencySelectionState,
    pub bounded_execution: BoundedExecutionState,
    pub evaluation: EvaluationControlState,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CognitiveSignalsState {
    pub dominant_instinct: String,
    pub completion_pressure: String,
    pub integrity_bias: String,
    pub curiosity_bias: String,
    pub candidate_selection_bias: String,
    pub urgency_level: String,
    pub salience_level: String,
    pub persistence_pressure: String,
    pub confidence_shift: String,
    pub downstream_influence: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CognitiveArbitrationState {
    pub route_selected: String,
    pub reasoning_mode: String,
    pub confidence: String,
    pub risk_class: String,
    pub applied_constraints: Vec<String>,
    pub cost_latency_assumption: String,
    pub route_reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FastSlowPathState {
    pub selected_path: String,
    pub path_family: String,
    pub runtime_branch_taken: String,
    pub handoff_state: String,
    pub candidate_strategy: String,
    pub review_depth: String,
    pub execution_profile: String,
    pub termination_expectation: String,
    pub path_difference_summary: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AgencySelectionState {
    pub candidate_generation_basis: String,
    pub selection_mode: String,
    pub candidate_set: Vec<AgencyCandidateRecord>,
    pub selected_candidate_id: String,
    pub selected_candidate_kind: String,
    pub selected_candidate_action: String,
    pub selected_candidate_reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AgencyCandidateRecord {
    pub candidate_id: String,
    pub candidate_kind: String,
    pub bounded_action: String,
    pub review_requirement: String,
    pub execution_priority: u32,
    pub rationale: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BoundedExecutionState {
    pub execution_status: String,
    pub continuation_state: String,
    pub provisional_termination_state: String,
    pub iterations: Vec<BoundedExecutionIteration>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BoundedExecutionIteration {
    pub iteration_index: u32,
    pub stage: String,
    pub action: String,
    pub outcome: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EvaluationControlState {
    pub progress_signal: String,
    pub contradiction_signal: String,
    pub failure_signal: String,
    pub termination_reason: String,
    pub behavior_effect: String,
    pub next_control_action: String,
}

#[derive(Debug, Clone, Copy)]
struct RuntimeSignalEvidence {
    failure_count: usize,
    retry_count: usize,
    delegation_denied_count: usize,
    security_denied_count: usize,
    success_ratio_permille: usize,
    scheduler_max_parallel_observed: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PauseState {
    pub paused_step_id: String,
    pub reason: Option<String>,
    pub completed_step_ids: Vec<String>,
    pub remaining_step_ids: Vec<String>,
    pub saved_state: HashMap<String, String>,
    pub completed_outputs: HashMap<String, String>,
}

#[derive(Debug, Clone, Default)]
pub struct ResumeState {
    pub completed_step_ids: HashSet<String>,
    pub saved_state: HashMap<String, String>,
    pub completed_outputs: HashMap<String, String>,
    pub steering_history: Vec<SteeringRecord>,
}

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

pub fn derive_runtime_control_state(
    overall_status: &str,
    records: &[StepExecutionRecord],
    tr: &trace::Trace,
) -> RuntimeControlState {
    let evidence = collect_runtime_signal_evidence(overall_status, records, tr);
    let signals = derive_cognitive_signals_state(overall_status, evidence);
    let arbitration = derive_cognitive_arbitration_state(overall_status, evidence, &signals);
    let fast_slow = derive_fast_slow_path_state(&arbitration);
    let agency = derive_agency_selection_state(&signals, &arbitration, &fast_slow);
    let bounded_execution = derive_bounded_execution_state(overall_status, &agency);
    let evaluation = derive_evaluation_control_state(overall_status, &bounded_execution);

    RuntimeControlState {
        signals,
        arbitration,
        fast_slow,
        agency,
        bounded_execution,
        evaluation,
    }
}

fn collect_runtime_signal_evidence(
    overall_status: &str,
    records: &[StepExecutionRecord],
    tr: &trace::Trace,
) -> RuntimeSignalEvidence {
    let total_steps = records.len();
    let failure_count = records
        .iter()
        .filter(|record| record.status != "success")
        .count();
    let success_count = total_steps.saturating_sub(failure_count);
    let retry_count: usize = records
        .iter()
        .map(|record| record.attempts.saturating_sub(1) as usize)
        .sum();
    let delegation_denied_count = tr
        .events
        .iter()
        .filter(|event| matches!(event, trace::TraceEvent::DelegationDenied { .. }))
        .count();
    let security_denied_count = delegation_denied_count;
    let success_ratio_permille = if total_steps == 0 {
        if overall_status == "success" {
            1000
        } else {
            0
        }
    } else {
        (success_count * 1000) / total_steps
    };
    let scheduler_max_parallel_observed = compute_max_parallel_observed_from_trace(tr);

    RuntimeSignalEvidence {
        failure_count,
        retry_count,
        delegation_denied_count,
        security_denied_count,
        success_ratio_permille,
        scheduler_max_parallel_observed,
    }
}

fn compute_max_parallel_observed_from_trace(tr: &trace::Trace) -> usize {
    let mut active: HashSet<&str> = HashSet::new();
    let mut max_parallel = 0usize;
    for event in &tr.events {
        match event {
            trace::TraceEvent::StepStarted { step_id, .. } => {
                active.insert(step_id.as_str());
                max_parallel = max_parallel.max(active.len());
            }
            trace::TraceEvent::StepFinished { step_id, .. } => {
                active.remove(step_id.as_str());
            }
            _ => {}
        }
    }
    max_parallel.max(1)
}

fn derive_cognitive_signals_state(
    overall_status: &str,
    evidence: RuntimeSignalEvidence,
) -> CognitiveSignalsState {
    let completion_pressure = if evidence.failure_count > 0 || overall_status == "failure" {
        "elevated"
    } else if evidence.retry_count > 0 || overall_status == "paused" {
        "guarded"
    } else {
        "steady"
    };
    let integrity_bias = if evidence.security_denied_count > 0 {
        "high"
    } else {
        "bounded"
    };
    let curiosity_bias = if evidence.success_ratio_permille < 1000 && evidence.failure_count == 0 {
        "active"
    } else {
        "low"
    };
    let dominant_instinct = if integrity_bias == "high" {
        "integrity"
    } else if completion_pressure == "elevated" {
        "completion"
    } else if curiosity_bias == "active" {
        "curiosity"
    } else {
        "coherence"
    };
    let candidate_selection_bias = match dominant_instinct {
        "integrity" => "prefer lower-risk constrained candidates",
        "completion" => "prefer candidates that reduce unfinished work quickly",
        "curiosity" => "prefer candidates that reduce uncertainty",
        _ => "prefer candidates that preserve bounded coherence",
    };
    let salience_level = if evidence.failure_count > 0 || evidence.delegation_denied_count > 0 {
        "high"
    } else if evidence.retry_count > 0 {
        "moderate"
    } else {
        "low"
    };
    let persistence_pressure = if evidence.failure_count > 0 {
        "retry_biased"
    } else if evidence.retry_count > 0 {
        "stabilize_then_retry"
    } else {
        "bounded_once"
    };
    let confidence_shift = if evidence.failure_count > 0 || evidence.delegation_denied_count > 0 {
        "reduced"
    } else {
        "stable"
    };

    CognitiveSignalsState {
        dominant_instinct: dominant_instinct.to_string(),
        completion_pressure: completion_pressure.to_string(),
        integrity_bias: integrity_bias.to_string(),
        curiosity_bias: curiosity_bias.to_string(),
        candidate_selection_bias: candidate_selection_bias.to_string(),
        urgency_level: completion_pressure.to_string(),
        salience_level: salience_level.to_string(),
        persistence_pressure: persistence_pressure.to_string(),
        confidence_shift: confidence_shift.to_string(),
        downstream_influence: format!(
            "dominant_instinct={} failure_count={} retry_count={} delegation_denied_count={} max_parallel={}",
            dominant_instinct,
            evidence.failure_count,
            evidence.retry_count,
            evidence.delegation_denied_count,
            evidence.scheduler_max_parallel_observed
        ),
    }
}

fn derive_cognitive_arbitration_state(
    overall_status: &str,
    evidence: RuntimeSignalEvidence,
    signals: &CognitiveSignalsState,
) -> CognitiveArbitrationState {
    let (route_selected, reasoning_mode) = if evidence.security_denied_count > 0
        || evidence.failure_count > 0
        || signals.dominant_instinct == "integrity"
    {
        ("slow", "review_heavy")
    } else if evidence.retry_count > 0
        || overall_status == "paused"
        || signals.confidence_shift == "reduced"
    {
        ("hybrid", "bounded_recovery")
    } else {
        ("fast", "direct_execution")
    };
    let risk_class = if evidence.security_denied_count > 0 {
        "high"
    } else if evidence.failure_count > 0 || evidence.retry_count > 0 {
        "medium"
    } else {
        "low"
    };
    let confidence = if route_selected == "fast" {
        "high"
    } else if route_selected == "hybrid" {
        "guarded"
    } else {
        "review_required"
    };
    let mut applied_constraints = Vec::new();
    if evidence.security_denied_count > 0 {
        applied_constraints.push("security_denial_present".to_string());
    }
    if evidence.failure_count > 0 {
        applied_constraints.push("failure_recovery_bias".to_string());
    }
    if evidence.retry_count > 0 {
        applied_constraints.push("retry_budget_pressure".to_string());
    }
    if overall_status == "paused" {
        applied_constraints.push("pause_boundary_present".to_string());
    }
    if applied_constraints.is_empty() {
        applied_constraints.push("bounded_default_path".to_string());
    }
    let cost_latency_assumption = match route_selected {
        "fast" => "prefer lower-cost low-latency execution when bounded evidence is stable",
        "hybrid" => "allow bounded extra review when retry or pause pressure is present",
        _ => "spend bounded additional cognition when failure or policy risk is present",
    };

    CognitiveArbitrationState {
        route_selected: route_selected.to_string(),
        reasoning_mode: reasoning_mode.to_string(),
        confidence: confidence.to_string(),
        risk_class: risk_class.to_string(),
        applied_constraints,
        cost_latency_assumption: cost_latency_assumption.to_string(),
        route_reason: format!(
            "route={} dominant_instinct={} overall_status={} failure_count={} retry_count={} delegation_denied_count={}",
            route_selected,
            signals.dominant_instinct,
            overall_status,
            evidence.failure_count,
            evidence.retry_count,
            evidence.delegation_denied_count
        ),
    }
}

fn derive_fast_slow_path_state(arbitration: &CognitiveArbitrationState) -> FastSlowPathState {
    let (
        selected_path,
        path_family,
        runtime_branch_taken,
        handoff_state,
        candidate_strategy,
        review_depth,
        execution_profile,
        termination_expectation,
    ) = match arbitration.route_selected.as_str() {
        "fast" => (
            "fast_path",
            "fast",
            "fast_direct_execution_branch",
            "direct_handoff",
            "accept first bounded candidate",
            "minimal",
            "single_pass_direct_execution",
            "terminate_on_first_bounded_success_or_policy_block",
        ),
        "hybrid" => (
            "slow_path",
            "slow",
            "slow_bounded_recovery_branch",
            "bounded_recovery_handoff",
            "compare current candidate against one bounded refinement",
            "bounded_recovery_review",
            "review_then_execute_once",
            "terminate_after_bounded_review_cycle_or_policy_block",
        ),
        _ => (
            "slow_path",
            "slow",
            "slow_review_refine_branch",
            "review_handoff",
            "validate, refine, or veto the current bounded candidate",
            "verification_required",
            "review_and_refine_before_execution",
            "terminate_after_bounded_review_cycle_or_policy_block",
        ),
    };
    let path_difference_summary = match selected_path {
        "fast_path" => {
            "fast_path favors direct execution with minimal review and a single bounded candidate handoff"
        }
        _ => {
            "slow_path requires bounded review/refinement before execution and can revise or veto the current candidate"
        }
    };

    FastSlowPathState {
        selected_path: selected_path.to_string(),
        path_family: path_family.to_string(),
        runtime_branch_taken: runtime_branch_taken.to_string(),
        handoff_state: handoff_state.to_string(),
        candidate_strategy: candidate_strategy.to_string(),
        review_depth: review_depth.to_string(),
        execution_profile: execution_profile.to_string(),
        termination_expectation: termination_expectation.to_string(),
        path_difference_summary: path_difference_summary.to_string(),
    }
}

fn derive_agency_selection_state(
    signals: &CognitiveSignalsState,
    arbitration: &CognitiveArbitrationState,
    fast_slow: &FastSlowPathState,
) -> AgencySelectionState {
    let (
        selection_mode,
        candidate_set,
        selected_candidate_id,
        selected_candidate_kind,
        selected_candidate_action,
        selected_candidate_reason,
    ) = match fast_slow.selected_path.as_str() {
        "fast_path" => {
            let candidate_set = vec![
                AgencyCandidateRecord {
                    candidate_id: "cand-fast-execute".to_string(),
                    candidate_kind: "direct_execution".to_string(),
                    bounded_action:
                        "execute selected candidate directly under bounded once semantics"
                            .to_string(),
                    review_requirement: "minimal".to_string(),
                    execution_priority: 1,
                    rationale: format!(
                        "route={} dominant_instinct={} confidence={}",
                        arbitration.route_selected, signals.dominant_instinct, arbitration.confidence
                    ),
                },
                AgencyCandidateRecord {
                    candidate_id: "cand-fast-verify".to_string(),
                    candidate_kind: "bounded_verification".to_string(),
                    bounded_action:
                        "perform one bounded verification pass before execution".to_string(),
                    review_requirement: "light".to_string(),
                    execution_priority: 2,
                    rationale:
                        "keep a fallback candidate available without changing the primary fast-path commitment"
                            .to_string(),
                },
            ];
            (
                "fast_candidate_commitment",
                candidate_set,
                "cand-fast-execute".to_string(),
                "direct_execution".to_string(),
                "execute selected candidate directly under bounded once semantics".to_string(),
                "fast path prioritizes direct bounded execution when arbitration confidence is high and failure pressure is absent".to_string(),
            )
        }
        _ => {
            let candidate_set = vec![
                AgencyCandidateRecord {
                    candidate_id: "cand-slow-review".to_string(),
                    candidate_kind: "review_and_refine".to_string(),
                    bounded_action:
                        "review, refine, or veto the current candidate before execution"
                            .to_string(),
                    review_requirement: "verification_required".to_string(),
                    execution_priority: 1,
                    rationale: format!(
                        "route={} dominant_instinct={} risk_class={}",
                        arbitration.route_selected, signals.dominant_instinct, arbitration.risk_class
                    ),
                },
                AgencyCandidateRecord {
                    candidate_id: "cand-slow-direct".to_string(),
                    candidate_kind: "direct_execution".to_string(),
                    bounded_action:
                        "execute the current candidate without additional refinement".to_string(),
                    review_requirement: "minimal".to_string(),
                    execution_priority: 2,
                    rationale:
                        "retain the direct-execution alternative as a bounded comparator candidate"
                            .to_string(),
                },
                AgencyCandidateRecord {
                    candidate_id: "cand-slow-defer".to_string(),
                    candidate_kind: "bounded_deferral".to_string(),
                    bounded_action:
                        "defer execution and surface the candidate set for later gate/review stages"
                            .to_string(),
                    review_requirement: "review_required".to_string(),
                    execution_priority: 3,
                    rationale:
                        "preserve a bounded non-execution option when policy or review pressure remains elevated"
                            .to_string(),
                },
            ];
            (
                "slow_candidate_comparison",
                candidate_set,
                "cand-slow-review".to_string(),
                "review_and_refine".to_string(),
                "review, refine, or veto the current candidate before execution".to_string(),
                "slow path makes review/refinement the selected candidate when arbitration requires bounded caution".to_string(),
            )
        }
    };

    AgencySelectionState {
        candidate_generation_basis: format!(
            "path={} runtime_branch={} route={} candidate_selection_bias={}",
            fast_slow.selected_path,
            fast_slow.runtime_branch_taken,
            arbitration.route_selected,
            signals.candidate_selection_bias
        ),
        selection_mode: selection_mode.to_string(),
        candidate_set,
        selected_candidate_id,
        selected_candidate_kind,
        selected_candidate_action,
        selected_candidate_reason,
    }
}

fn derive_bounded_execution_state(
    overall_status: &str,
    agency: &AgencySelectionState,
) -> BoundedExecutionState {
    let (execution_status, continuation_state, provisional_termination_state, iterations) =
        match agency.selected_candidate_kind.as_str() {
            "direct_execution" => (
                "completed",
                "stop_after_one",
                "ready_for_evaluation",
                vec![BoundedExecutionIteration {
                    iteration_index: 1,
                    stage: "execute".to_string(),
                    action: agency.selected_candidate_action.clone(),
                    outcome: "bounded_direct_execution_complete".to_string(),
                }],
            ),
            "review_and_refine" => (
                "completed",
                "bounded_review_complete",
                "ready_for_evaluation",
                vec![
                    BoundedExecutionIteration {
                        iteration_index: 1,
                        stage: "review".to_string(),
                        action: agency.selected_candidate_action.clone(),
                        outcome: "bounded_review_pass_complete".to_string(),
                    },
                    BoundedExecutionIteration {
                        iteration_index: 2,
                        stage: "execute".to_string(),
                        action: "execute the reviewed bounded candidate".to_string(),
                        outcome: "bounded_reviewed_execution_complete".to_string(),
                    },
                ],
            ),
            _ => (
                "completed",
                "deferred",
                "ready_for_evaluation",
                vec![BoundedExecutionIteration {
                    iteration_index: 1,
                    stage: "defer".to_string(),
                    action: agency.selected_candidate_action.clone(),
                    outcome: "bounded_deferral_recorded".to_string(),
                }],
            ),
        };

    let execution_status = if overall_status == "failure" {
        "completed_with_failure_signal"
    } else if overall_status == "paused" {
        "paused_for_review"
    } else {
        execution_status
    };
    let continuation_state = if overall_status == "failure" && iterations.len() > 1 {
        "bounded_review_complete_with_failure_signal"
    } else if overall_status == "paused" {
        "paused_before_termination"
    } else {
        continuation_state
    };

    BoundedExecutionState {
        execution_status: execution_status.to_string(),
        continuation_state: continuation_state.to_string(),
        provisional_termination_state: provisional_termination_state.to_string(),
        iterations,
    }
}

fn derive_evaluation_control_state(
    overall_status: &str,
    bounded_execution: &BoundedExecutionState,
) -> EvaluationControlState {
    let (
        progress_signal,
        contradiction_signal,
        failure_signal,
        termination_reason,
        behavior_effect,
        next_control_action,
    ) = if overall_status == "failure" {
        (
            "stalled_progress",
            "present",
            "bounded_failure_detected",
            if bounded_execution.iterations.len() > 1 {
                "bounded_failure"
            } else {
                "no_progress"
            },
            "emit bounded failure/termination signals for later reframing or policy handling",
            if bounded_execution.iterations.len() > 1 {
                "handoff_to_reframing"
            } else {
                "terminate_with_failure"
            },
        )
    } else if overall_status == "paused" {
        (
            "guarded_progress",
            "none",
            "none",
            "pause_boundary",
            "preserve bounded state for resume or explicit review handling",
            "await_resume",
        )
    } else {
        (
            "steady_progress",
            "none",
            "none",
            "success",
            "allow bounded execution to terminate cleanly after evaluation confirms progress",
            "complete_run",
        )
    };

    EvaluationControlState {
        progress_signal: progress_signal.to_string(),
        contradiction_signal: contradiction_signal.to_string(),
        failure_signal: failure_signal.to_string(),
        termination_reason: termination_reason.to_string(),
        behavior_effect: behavior_effect.to_string(),
        next_control_action: next_control_action.to_string(),
    }
}

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

#[derive(Debug, Clone, PartialEq, Eq)]
/// Stable policy rejection kinds emitted by execution-time delegation checks.
pub enum ExecutionPolicyErrorKind {
    /// Action was denied by policy and execution must fail/stop that step.
    Denied,
    /// Action requires approval and cannot proceed automatically.
    ApprovalRequired,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionPolicyError {
    pub kind: ExecutionPolicyErrorKind,
    pub step_id: String,
    pub action_kind: String,
    pub target_id: String,
    pub rule_id: Option<String>,
}

impl ExecutionPolicyError {
    pub fn code(&self) -> &'static str {
        match self.kind {
            ExecutionPolicyErrorKind::Denied => DELEGATION_POLICY_DENY_CODE,
            ExecutionPolicyErrorKind::ApprovalRequired => "DELEGATION_POLICY_APPROVAL_REQUIRED",
        }
    }
}

impl std::fmt::Display for ExecutionPolicyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let suffix = self
            .rule_id
            .as_ref()
            .map(|id| format!(" (rule_id={id})"))
            .unwrap_or_default();
        match self.kind {
            ExecutionPolicyErrorKind::Denied => write!(
                f,
                "{}: step '{}' action '{}' target '{}' denied{}",
                self.code(),
                self.step_id,
                self.action_kind,
                self.target_id,
                suffix
            ),
            ExecutionPolicyErrorKind::ApprovalRequired => write!(
                f,
                "{}: step '{}' action '{}' target '{}' requires approval{}",
                self.code(),
                self.step_id,
                self.action_kind,
                self.target_id,
                suffix
            ),
        }
    }
}

impl std::error::Error for ExecutionPolicyError {}

pub fn stable_failure_kind(err: &anyhow::Error) -> Option<&'static str> {
    for cause in err.chain() {
        if cause.downcast_ref::<ExecutionPolicyError>().is_some() {
            return Some("policy_denied");
        }
    }
    None
}
