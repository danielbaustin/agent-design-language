use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::path::{Path, PathBuf};

use ::adl::{artifacts, execute, failure_taxonomy, instrumentation, resolve, trace};

pub(crate) const RUN_STATE_SCHEMA_VERSION: &str = "run_state.v1";
pub(crate) const PAUSE_STATE_SCHEMA_VERSION: &str = "pause_state.v1";
pub(crate) const RUN_STATUS_VERSION: u32 = 1;
pub(crate) const RUN_SUMMARY_VERSION: u32 = 1;
pub(crate) const SCORES_VERSION: u32 = 1;
pub(crate) const SUGGESTIONS_VERSION: u32 = 1;
pub(crate) const AEE_DECISION_VERSION: u32 = 1;
pub(crate) const COGNITIVE_SIGNALS_VERSION: u32 = 1;
pub(crate) const COGNITIVE_ARBITRATION_VERSION: u32 = 1;
pub(crate) const FAST_SLOW_PATH_VERSION: u32 = 1;
pub(crate) const AGENCY_SELECTION_VERSION: u32 = 1;
pub(crate) const BOUNDED_EXECUTION_VERSION: u32 = 1;
pub(crate) const EVALUATION_SIGNALS_VERSION: u32 = 1;
pub(crate) const REASONING_GRAPH_VERSION: u32 = 1;
pub(crate) const CLUSTER_GROUNDWORK_VERSION: u32 = 1;

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct RunStateArtifact {
    pub(crate) schema_version: String,
    pub(crate) run_id: String,
    pub(crate) workflow_id: String,
    pub(crate) version: String,
    pub(crate) status: String,
    pub(crate) error_message: Option<String>,
    pub(crate) start_time_ms: u128,
    pub(crate) end_time_ms: u128,
    pub(crate) duration_ms: u128,
    pub(crate) execution_plan_hash: String,
    #[serde(default)]
    pub(crate) scheduler_max_concurrency: Option<usize>,
    #[serde(default)]
    pub(crate) scheduler_policy_source: Option<String>,
    #[serde(default)]
    pub(crate) steering_history: Vec<execute::SteeringRecord>,
    pub(crate) pause: Option<execute::PauseState>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PauseStateArtifact {
    pub(crate) schema_version: String,
    pub(crate) run_id: String,
    pub(crate) workflow_id: String,
    pub(crate) version: String,
    pub(crate) status: String,
    pub(crate) adl_path: String,
    pub(crate) execution_plan_hash: String,
    #[serde(default)]
    pub(crate) steering_history: Vec<execute::SteeringRecord>,
    pub(crate) pause: execute::PauseState,
}

#[derive(Debug, Serialize)]
pub(crate) struct StepStateArtifact {
    pub(crate) step_id: String,
    pub(crate) agent_id: String,
    pub(crate) provider_id: String,
    pub(crate) status: String,
    pub(crate) output_artifact_path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct RunStatusArtifact {
    pub(crate) run_status_version: u32,
    pub(crate) run_id: String,
    pub(crate) workflow_id: String,
    pub(crate) overall_status: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) failure_kind: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) failed_step_id: Option<String>,
    pub(crate) completed_steps: Vec<String>,
    pub(crate) pending_steps: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) started_steps: Option<Vec<String>>,
    pub(crate) attempt_counts_by_step: BTreeMap<String, u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) effective_max_concurrency: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) effective_max_concurrency_source: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct RunSummaryArtifact {
    pub(crate) run_summary_version: u32,
    pub(crate) artifact_model_version: u32,
    pub(crate) run_id: String,
    pub(crate) workflow_id: String,
    pub(crate) adl_version: String,
    pub(crate) swarm_version: String,
    pub(crate) status: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) error_kind: Option<String>,
    pub(crate) counts: RunSummaryCounts,
    pub(crate) policy: RunSummaryPolicy,
    pub(crate) links: RunSummaryLinks,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct RunSummaryCounts {
    pub(crate) total_steps: usize,
    pub(crate) completed_steps: usize,
    pub(crate) failed_steps: usize,
    pub(crate) provider_call_count: usize,
    pub(crate) delegation_steps: usize,
    pub(crate) delegation_requires_verification_steps: usize,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct RunSummaryPolicy {
    pub(crate) security_envelope_enabled: bool,
    pub(crate) signing_required: bool,
    pub(crate) key_id_required: bool,
    pub(crate) verify_allowed_algs: Vec<String>,
    pub(crate) verify_allowed_key_sources: Vec<String>,
    pub(crate) sandbox_policy: String,
    pub(crate) security_denials_by_code: BTreeMap<String, usize>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct RunSummaryLinks {
    pub(crate) run_json: String,
    pub(crate) steps_json: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) pause_state_json: Option<String>,
    pub(crate) outputs_dir: String,
    pub(crate) logs_dir: String,
    pub(crate) learning_dir: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) scores_json: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) suggestions_json: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) aee_decision_json: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) cognitive_signals_json: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) fast_slow_path_json: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) agency_selection_json: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) bounded_execution_json: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) evaluation_signals_json: Option<String>,
    pub(crate) cognitive_arbitration_json: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) affect_state_json: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) reasoning_graph_json: Option<String>,
    pub(crate) overlays_dir: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) cluster_groundwork_json: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) trace_json: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ClusterGroundworkArtifact {
    pub(crate) cluster_groundwork_version: u32,
    pub(crate) run_id: String,
    pub(crate) workflow_id: String,
    pub(crate) coordinator_id: String,
    pub(crate) worker_id: String,
    pub(crate) canonical_ordering_key: String,
    pub(crate) frontier_ordering: String,
    pub(crate) readiness_frontiers: Vec<ClusterReadyFrontier>,
    pub(crate) lease_records: Vec<ClusterLeaseRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ClusterReadyFrontier {
    pub(crate) frontier_index: u32,
    pub(crate) ready_step_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ClusterLeaseRecord {
    pub(crate) issued_sequence: u32,
    pub(crate) lease_id: String,
    pub(crate) step_id: String,
    pub(crate) depends_on: Vec<String>,
    pub(crate) observed_attempts: u32,
    pub(crate) claim_owner: String,
    pub(crate) worker_id: String,
    pub(crate) lease_state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ScoresArtifact {
    pub(crate) scores_version: u32,
    pub(crate) run_id: String,
    pub(crate) generated_from: ScoresGeneratedFrom,
    pub(crate) summary: ScoresSummary,
    pub(crate) metrics: ScoresMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ScoresGeneratedFrom {
    pub(crate) artifact_model_version: u32,
    pub(crate) run_summary_version: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ScoresSummary {
    pub(crate) success_ratio: f64,
    pub(crate) failure_count: usize,
    pub(crate) retry_count: usize,
    pub(crate) delegation_denied_count: usize,
    pub(crate) security_denied_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ScoresMetrics {
    pub(crate) scheduler_max_parallel_observed: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct SuggestionsArtifact {
    pub(crate) suggestions_version: u32,
    pub(crate) run_id: String,
    pub(crate) generated_from: SuggestionsGeneratedFrom,
    pub(crate) suggestions: Vec<SuggestionItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct SuggestionsGeneratedFrom {
    pub(crate) artifact_model_version: u32,
    pub(crate) run_summary_version: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) scores_version: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct SuggestionItem {
    pub(crate) id: String,
    pub(crate) category: String,
    pub(crate) severity: String,
    pub(crate) rationale: String,
    pub(crate) evidence: SuggestionEvidence,
    pub(crate) proposed_change: SuggestedChangeIntent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct SuggestionEvidence {
    pub(crate) failure_count: usize,
    pub(crate) retry_count: usize,
    pub(crate) delegation_denied_count: usize,
    pub(crate) security_denied_count: usize,
    pub(crate) success_ratio: f64,
    pub(crate) scheduler_max_parallel_observed: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct SuggestedChangeIntent {
    pub(crate) intent: String,
    pub(crate) target: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct AeeDecisionArtifact {
    pub(crate) aee_decision_version: u32,
    pub(crate) run_id: String,
    pub(crate) generated_from: AeeDecisionGeneratedFrom,
    pub(crate) affect_state: AffectStateRef,
    pub(crate) decision: AeeDecisionRecord,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct AeeDecisionGeneratedFrom {
    pub(crate) artifact_model_version: u32,
    pub(crate) run_summary_version: u32,
    pub(crate) suggestions_version: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) scores_version: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct AffectStateArtifact {
    pub(crate) affect_state_version: u32,
    pub(crate) run_id: String,
    pub(crate) generated_from: AeeDecisionGeneratedFrom,
    pub(crate) affect: AffectStateRecord,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct CognitiveSignalsArtifact {
    pub(crate) cognitive_signals_version: u32,
    pub(crate) run_id: String,
    pub(crate) generated_from: AeeDecisionGeneratedFrom,
    pub(crate) instinct: CognitiveInstinctRecord,
    pub(crate) affect: CognitiveAffectSignalRecord,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct CognitiveInstinctRecord {
    pub(crate) instinct_profile_id: String,
    pub(crate) dominant_instinct: String,
    pub(crate) completion_pressure: String,
    pub(crate) integrity_bias: String,
    pub(crate) curiosity_bias: String,
    pub(crate) candidate_selection_bias: String,
    pub(crate) deterministic_update_rule: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct CognitiveAffectSignalRecord {
    pub(crate) affect_state_id: String,
    pub(crate) urgency_level: String,
    pub(crate) salience_level: String,
    pub(crate) persistence_pressure: String,
    pub(crate) confidence_shift: String,
    pub(crate) downstream_influence: String,
    pub(crate) deterministic_update_rule: String,
}

#[cfg(test)]
pub(crate) type CognitiveSignalsState = execute::CognitiveSignalsState;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct AffectStateRecord {
    pub(crate) affect_state_id: String,
    pub(crate) affect_mode: String,
    pub(crate) urgency_level: String,
    pub(crate) frustration_level: String,
    pub(crate) confidence_shift: String,
    pub(crate) recovery_bias: u32,
    pub(crate) downstream_priority: String,
    pub(crate) update_reason: String,
    pub(crate) deterministic_update_rule: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct AffectStateRef {
    pub(crate) affect_state_id: String,
    pub(crate) affect_mode: String,
    pub(crate) downstream_priority: String,
    pub(crate) recovery_bias: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct CognitiveArbitrationArtifact {
    pub(crate) cognitive_arbitration_version: u32,
    pub(crate) run_id: String,
    pub(crate) generated_from: AeeDecisionGeneratedFrom,
    pub(crate) route_selected: String,
    pub(crate) reasoning_mode: String,
    pub(crate) confidence: String,
    pub(crate) risk_class: String,
    pub(crate) applied_constraints: Vec<String>,
    pub(crate) cost_latency_assumption: String,
    pub(crate) route_reason: String,
    pub(crate) deterministic_selection_rule: String,
}

#[cfg(test)]
pub(crate) type CognitiveArbitrationState = execute::CognitiveArbitrationState;

#[cfg(test)]
pub(crate) type FastSlowPathState = execute::FastSlowPathState;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct FastSlowPathArtifact {
    pub(crate) fast_slow_path_version: u32,
    pub(crate) run_id: String,
    pub(crate) generated_from: AeeDecisionGeneratedFrom,
    pub(crate) arbitration_route: String,
    pub(crate) selected_path: String,
    pub(crate) path_family: String,
    pub(crate) runtime_branch_taken: String,
    pub(crate) handoff_state: String,
    pub(crate) candidate_strategy: String,
    pub(crate) review_depth: String,
    pub(crate) execution_profile: String,
    pub(crate) termination_expectation: String,
    pub(crate) path_difference_summary: String,
    pub(crate) deterministic_handoff_rule: String,
}

#[cfg(test)]
pub(crate) type AgencySelectionState = execute::AgencySelectionState;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct AgencySelectionArtifact {
    pub(crate) agency_selection_version: u32,
    pub(crate) run_id: String,
    pub(crate) generated_from: AeeDecisionGeneratedFrom,
    pub(crate) candidate_generation_basis: String,
    pub(crate) selection_mode: String,
    pub(crate) candidate_set: Vec<AgencyCandidateRecord>,
    pub(crate) selected_candidate_id: String,
    pub(crate) selected_candidate_reason: String,
    pub(crate) deterministic_selection_rule: String,
}

pub(crate) type AgencyCandidateRecord = execute::AgencyCandidateRecord;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct BoundedExecutionArtifact {
    pub(crate) bounded_execution_version: u32,
    pub(crate) run_id: String,
    pub(crate) generated_from: AeeDecisionGeneratedFrom,
    pub(crate) selected_candidate_id: String,
    pub(crate) selected_path: String,
    pub(crate) execution_status: String,
    pub(crate) continuation_state: String,
    pub(crate) provisional_termination_state: String,
    pub(crate) iteration_count: u32,
    pub(crate) iterations: Vec<BoundedExecutionIteration>,
    pub(crate) deterministic_execution_rule: String,
}

pub(crate) type BoundedExecutionState = execute::BoundedExecutionState;

pub(crate) type BoundedExecutionIteration = execute::BoundedExecutionIteration;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct EvaluationSignalsArtifact {
    pub(crate) evaluation_signals_version: u32,
    pub(crate) run_id: String,
    pub(crate) generated_from: AeeDecisionGeneratedFrom,
    pub(crate) selected_candidate_id: String,
    pub(crate) selected_path: String,
    pub(crate) progress_signal: String,
    pub(crate) contradiction_signal: String,
    pub(crate) failure_signal: String,
    pub(crate) termination_reason: String,
    pub(crate) behavior_effect: String,
    pub(crate) next_control_action: String,
    pub(crate) deterministic_evaluation_rule: String,
}

pub(crate) type EvaluationControlState = execute::EvaluationControlState;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ReasoningGraphArtifact {
    pub(crate) reasoning_graph_version: u32,
    pub(crate) run_id: String,
    pub(crate) generated_from: AeeDecisionGeneratedFrom,
    pub(crate) graph: ReasoningGraphRecord,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ReasoningGraphRecord {
    pub(crate) graph_id: String,
    pub(crate) dominant_affect_mode: String,
    pub(crate) ranking_rule: String,
    pub(crate) selected_path: ReasoningGraphSelection,
    pub(crate) nodes: Vec<ReasoningGraphNode>,
    pub(crate) edges: Vec<ReasoningGraphEdge>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ReasoningGraphSelection {
    pub(crate) selected_node_id: String,
    pub(crate) selected_intent: String,
    pub(crate) selected_target: String,
    pub(crate) graph_derived_output: String,
    pub(crate) affect_changed_ranking: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ReasoningGraphNode {
    pub(crate) node_id: String,
    pub(crate) node_kind: String,
    pub(crate) label: String,
    pub(crate) rank: u32,
    pub(crate) priority_score: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) affect_mode: Option<String>,
    pub(crate) rationale: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ReasoningGraphEdge {
    pub(crate) edge_id: String,
    pub(crate) from: String,
    pub(crate) to: String,
    pub(crate) relation: String,
    pub(crate) rationale: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct AeeDecisionRecord {
    pub(crate) decision_id: String,
    pub(crate) decision_kind: String,
    pub(crate) selected_suggestion_id: String,
    pub(crate) category: String,
    pub(crate) intent: String,
    pub(crate) target: String,
    pub(crate) rationale: String,
    pub(crate) expected_downstream_effect: String,
    pub(crate) deterministic_selection_rule: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) recommended_retry_budget: Option<u32>,
    pub(crate) evidence: SuggestionEvidence,
}

pub(crate) fn stable_fingerprint_hex(bytes: &[u8]) -> String {
    let mut hash: u64 = 0xcbf29ce484222325;
    for b in bytes {
        hash ^= u64::from(*b);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("{hash:016x}")
}

pub(crate) fn execution_plan_hash<T: Serialize>(plan: &T) -> Result<String> {
    let plan_json = serde_json::to_vec(plan).context("serialize execution plan for hashing")?;
    Ok(stable_fingerprint_hex(&plan_json))
}

pub(crate) fn classify_failure_kind(err: &anyhow::Error) -> Option<&'static str> {
    failure_taxonomy::classify(err)
}

pub(crate) fn build_run_summary(
    resolved: &resolve::AdlResolved,
    status: &str,
    pause: Option<&execute::PauseState>,
    steps: &[StepStateArtifact],
    records: usize,
    failure: Option<&anyhow::Error>,
    run_paths: &artifacts::RunArtifactPaths,
) -> RunSummaryArtifact {
    let failed_steps = steps.iter().filter(|s| s.status == "failure").count();
    let completed_steps = steps
        .iter()
        .filter(|s| s.status == "success" || s.status == "failure")
        .count();
    let delegation_steps = resolved
        .steps
        .iter()
        .filter(|s| {
            s.delegation
                .as_ref()
                .map(|d| !d.is_effectively_empty())
                .unwrap_or(false)
        })
        .count();
    let delegation_requires_verification_steps = resolved
        .steps
        .iter()
        .filter(|s| {
            s.delegation
                .as_ref()
                .and_then(|d| d.requires_verification)
                .unwrap_or(false)
        })
        .count();
    let mut security_denials_by_code = BTreeMap::new();
    if let Some(code) = failure.and_then(classify_failure_kind) {
        *security_denials_by_code
            .entry(code.to_string())
            .or_insert(0) += 1;
    }

    let (
        security_envelope_enabled,
        signing_required,
        key_id_required,
        mut allowed_algs,
        mut allowed_key_sources,
    ) = if let Some(remote) = resolved.doc.run.remote.as_ref() {
        (
            true,
            remote.require_signed_requests,
            remote.require_key_id,
            remote.verify_allowed_algs.clone(),
            remote.verify_allowed_key_sources.clone(),
        )
    } else {
        (false, false, false, Vec::new(), Vec::new())
    };
    allowed_algs.sort();
    allowed_algs.dedup();
    allowed_key_sources.sort();
    allowed_key_sources.dedup();
    let scores_rel = run_paths
        .scores_json()
        .strip_prefix(run_paths.run_dir())
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| "learning/scores.json".to_string());
    let suggestions_rel = run_paths
        .suggestions_json()
        .strip_prefix(run_paths.run_dir())
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| "learning/suggestions.json".to_string());
    let aee_decision_rel = run_paths
        .aee_decision_json()
        .strip_prefix(run_paths.run_dir())
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| "learning/aee_decision.json".to_string());
    let cognitive_signals_rel = run_paths
        .cognitive_signals_json()
        .strip_prefix(run_paths.run_dir())
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| "learning/cognitive_signals.v1.json".to_string());
    let fast_slow_path_rel = run_paths
        .fast_slow_path_json()
        .strip_prefix(run_paths.run_dir())
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| "learning/fast_slow_path.v1.json".to_string());
    let agency_selection_rel = run_paths
        .agency_selection_json()
        .strip_prefix(run_paths.run_dir())
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| "learning/agency_selection.v1.json".to_string());
    let bounded_execution_rel = run_paths
        .bounded_execution_json()
        .strip_prefix(run_paths.run_dir())
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| "learning/bounded_execution.v1.json".to_string());
    let evaluation_signals_rel = run_paths
        .evaluation_signals_json()
        .strip_prefix(run_paths.run_dir())
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| "learning/evaluation_signals.v1.json".to_string());
    let cognitive_arbitration_rel = run_paths
        .cognitive_arbitration_json()
        .strip_prefix(run_paths.run_dir())
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| "learning/cognitive_arbitration.v1.json".to_string());
    let affect_state_rel = run_paths
        .affect_state_json()
        .strip_prefix(run_paths.run_dir())
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| "learning/affect_state.v1.json".to_string());
    let reasoning_graph_rel = run_paths
        .reasoning_graph_json()
        .strip_prefix(run_paths.run_dir())
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| "learning/reasoning_graph.v1.json".to_string());
    let cluster_groundwork_rel = run_paths
        .cluster_groundwork_json()
        .strip_prefix(run_paths.run_dir())
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| "meta/cluster_groundwork.json".to_string());

    RunSummaryArtifact {
        run_summary_version: RUN_SUMMARY_VERSION,
        artifact_model_version: artifacts::ARTIFACT_MODEL_VERSION,
        run_id: resolved.run_id.clone(),
        workflow_id: resolved.workflow_id.clone(),
        adl_version: resolved.doc.version.clone(),
        swarm_version: env!("CARGO_PKG_VERSION").to_string(),
        status: status.to_string(),
        error_kind: failure.and_then(classify_failure_kind).map(str::to_string),
        counts: RunSummaryCounts {
            total_steps: resolved.steps.len(),
            completed_steps,
            failed_steps,
            provider_call_count: records,
            delegation_steps,
            delegation_requires_verification_steps,
        },
        policy: RunSummaryPolicy {
            security_envelope_enabled,
            signing_required,
            key_id_required,
            verify_allowed_algs: allowed_algs,
            verify_allowed_key_sources: allowed_key_sources,
            sandbox_policy: "centralized_path_resolver_v1".to_string(),
            security_denials_by_code,
        },
        links: RunSummaryLinks {
            run_json: "run.json".to_string(),
            steps_json: "steps.json".to_string(),
            pause_state_json: pause.map(|_| "pause_state.json".to_string()),
            outputs_dir: run_paths
                .outputs_dir()
                .strip_prefix(run_paths.run_dir())
                .map(|p| p.display().to_string())
                .unwrap_or_else(|_| "outputs".to_string()),
            logs_dir: run_paths
                .logs_dir()
                .strip_prefix(run_paths.run_dir())
                .map(|p| p.display().to_string())
                .unwrap_or_else(|_| "logs".to_string()),
            learning_dir: run_paths
                .learning_dir()
                .strip_prefix(run_paths.run_dir())
                .map(|p| p.display().to_string())
                .unwrap_or_else(|_| "learning".to_string()),
            scores_json: Some(scores_rel),
            suggestions_json: Some(suggestions_rel),
            aee_decision_json: Some(aee_decision_rel),
            cognitive_signals_json: Some(cognitive_signals_rel),
            fast_slow_path_json: Some(fast_slow_path_rel),
            agency_selection_json: Some(agency_selection_rel),
            bounded_execution_json: Some(bounded_execution_rel),
            evaluation_signals_json: Some(evaluation_signals_rel),
            cognitive_arbitration_json: Some(cognitive_arbitration_rel),
            affect_state_json: Some(affect_state_rel),
            reasoning_graph_json: Some(reasoning_graph_rel),
            overlays_dir: run_paths
                .overlays_dir()
                .strip_prefix(run_paths.run_dir())
                .map(|p| p.display().to_string())
                .unwrap_or_else(|_| "learning/overlays".to_string()),
            cluster_groundwork_json: run_paths
                .cluster_groundwork_json()
                .is_file()
                .then_some(cluster_groundwork_rel),
            trace_json: None,
        },
    }
}

pub(crate) fn build_cluster_groundwork_artifact(
    resolved: &resolve::AdlResolved,
    steps: &[StepStateArtifact],
    tr: &trace::Trace,
) -> ClusterGroundworkArtifact {
    let mut remaining_deps: BTreeMap<String, BTreeSet<String>> = resolved
        .execution_plan
        .nodes
        .iter()
        .map(|node| {
            (
                node.step_id.clone(),
                node.depends_on.iter().cloned().collect::<BTreeSet<_>>(),
            )
        })
        .collect();
    let mut remaining_nodes: BTreeSet<String> =
        remaining_deps.keys().cloned().collect::<BTreeSet<_>>();
    let mut readiness_frontiers = Vec::new();
    while !remaining_nodes.is_empty() {
        let ready = remaining_nodes
            .iter()
            .filter(|step_id| {
                remaining_deps
                    .get(step_id.as_str())
                    .map(|deps| deps.is_empty())
                    .unwrap_or(false)
            })
            .cloned()
            .collect::<Vec<_>>();
        if ready.is_empty() {
            break;
        }
        readiness_frontiers.push(ClusterReadyFrontier {
            frontier_index: readiness_frontiers.len() as u32,
            ready_step_ids: ready.clone(),
        });
        for step_id in &ready {
            remaining_nodes.remove(step_id);
        }
        for deps in remaining_deps.values_mut() {
            for step_id in &ready {
                deps.remove(step_id);
            }
        }
    }

    let mut attempts_by_step: BTreeMap<String, u32> = BTreeMap::new();
    for event in &tr.events {
        if let trace::TraceEvent::StepStarted { step_id, .. } = event {
            *attempts_by_step.entry(step_id.clone()).or_insert(0) += 1;
        }
    }
    let status_by_step = steps
        .iter()
        .map(|step| (step.step_id.clone(), step.status.clone()))
        .collect::<BTreeMap<_, _>>();
    let depends_on_by_step = resolved
        .execution_plan
        .nodes
        .iter()
        .map(|node| (node.step_id.clone(), node.depends_on.clone()))
        .collect::<BTreeMap<_, _>>();

    let mut lease_records = Vec::new();
    let mut issued_sequence: u32 = 0;
    for frontier in &readiness_frontiers {
        for step_id in &frontier.ready_step_ids {
            issued_sequence = issued_sequence.saturating_add(1);
            let status = status_by_step
                .get(step_id)
                .map(|value| value.as_str())
                .unwrap_or("not_run");
            let lease_state = match status {
                "success" => "completed",
                "failure" => "failed",
                _ => "planned",
            };
            lease_records.push(ClusterLeaseRecord {
                issued_sequence,
                lease_id: format!("lease:{}:{}:1", resolved.run_id, step_id),
                step_id: step_id.clone(),
                depends_on: depends_on_by_step.get(step_id).cloned().unwrap_or_default(),
                observed_attempts: attempts_by_step.get(step_id).copied().unwrap_or(0),
                claim_owner: "adl-coordinator-local".to_string(),
                worker_id: "adl-worker-local".to_string(),
                lease_state: lease_state.to_string(),
            });
        }
    }

    ClusterGroundworkArtifact {
        cluster_groundwork_version: CLUSTER_GROUNDWORK_VERSION,
        run_id: resolved.run_id.clone(),
        workflow_id: resolved.workflow_id.clone(),
        coordinator_id: "adl-coordinator-local".to_string(),
        worker_id: "adl-worker-local".to_string(),
        canonical_ordering_key: "(run_id, step_id, attempt)".to_string(),
        frontier_ordering: "topological_frontier_then_step_id".to_string(),
        readiness_frontiers,
        lease_records,
    }
}

pub(crate) fn build_run_status(
    resolved: &resolve::AdlResolved,
    tr: &trace::Trace,
    overall_status: &str,
    steps: &[StepStateArtifact],
    failure: Option<&anyhow::Error>,
    resume_completed_step_ids: &BTreeSet<String>,
) -> RunStatusArtifact {
    let mut completed_steps: BTreeSet<String> = resume_completed_step_ids.clone();
    let mut pending_steps: BTreeSet<String> = BTreeSet::new();
    let mut failed_step_id: Option<String> = None;

    for step in steps {
        match step.status.as_str() {
            "success" => {
                completed_steps.insert(step.step_id.clone());
            }
            "failure" => {
                if failed_step_id.is_none() {
                    failed_step_id = Some(step.step_id.clone());
                }
                pending_steps.insert(step.step_id.clone());
            }
            _ => {
                pending_steps.insert(step.step_id.clone());
            }
        }
    }

    let mut attempts_by_step: BTreeMap<String, u32> = resume_completed_step_ids
        .iter()
        .map(|step_id| (step_id.clone(), 0))
        .collect();
    let mut started_set = BTreeSet::new();
    for event in &tr.events {
        if let trace::TraceEvent::StepStarted { step_id, .. } = event {
            started_set.insert(step_id.clone());
            *attempts_by_step.entry(step_id.clone()).or_insert(0) += 1;
        }
    }

    let scheduler_policy = execute::scheduler_policy_for_run(resolved).ok().flatten();

    RunStatusArtifact {
        run_status_version: RUN_STATUS_VERSION,
        run_id: resolved.run_id.clone(),
        workflow_id: resolved.workflow_id.clone(),
        overall_status: overall_status.to_string(),
        failure_kind: failure.and_then(classify_failure_kind).map(str::to_string),
        failed_step_id,
        completed_steps: completed_steps.into_iter().collect(),
        pending_steps: pending_steps.into_iter().collect(),
        started_steps: if started_set.is_empty() {
            None
        } else {
            Some(started_set.into_iter().collect())
        },
        attempt_counts_by_step: attempts_by_step,
        effective_max_concurrency: scheduler_policy.map(|(value, _)| value),
        effective_max_concurrency_source: scheduler_policy
            .map(|(_, source)| source.as_str().to_string()),
    }
}

pub(crate) fn compute_retry_count(tr: &trace::Trace) -> usize {
    let mut started_by_step: BTreeMap<&str, usize> = BTreeMap::new();
    for event in &tr.events {
        if let trace::TraceEvent::StepStarted { step_id, .. } = event {
            *started_by_step.entry(step_id.as_str()).or_insert(0) += 1;
        }
    }
    started_by_step
        .values()
        .map(|count| count.saturating_sub(1))
        .sum()
}

pub(crate) fn compute_max_parallel_observed(tr: &trace::Trace) -> usize {
    let mut active: BTreeSet<&str> = BTreeSet::new();
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
    max_parallel
}

pub(crate) fn build_scores_artifact(
    run_summary: &RunSummaryArtifact,
    tr: &trace::Trace,
) -> ScoresArtifact {
    let success_steps = run_summary
        .counts
        .completed_steps
        .saturating_sub(run_summary.counts.failed_steps);
    let success_ratio = if run_summary.counts.total_steps == 0 {
        1.0
    } else {
        let permille = (success_steps * 1000) / run_summary.counts.total_steps;
        (permille as f64) / 1000.0
    };
    let security_denied_count: usize = run_summary.policy.security_denials_by_code.values().sum();
    let delegation_denied_count: usize = run_summary
        .policy
        .security_denials_by_code
        .iter()
        .filter_map(|(code, count)| {
            if code.starts_with("DELEGATION_") {
                Some(*count)
            } else {
                None
            }
        })
        .sum();

    ScoresArtifact {
        scores_version: SCORES_VERSION,
        run_id: run_summary.run_id.clone(),
        generated_from: ScoresGeneratedFrom {
            artifact_model_version: run_summary.artifact_model_version,
            run_summary_version: run_summary.run_summary_version,
        },
        summary: ScoresSummary {
            success_ratio,
            failure_count: run_summary.counts.failed_steps,
            retry_count: compute_retry_count(tr),
            delegation_denied_count,
            security_denied_count,
        },
        metrics: ScoresMetrics {
            scheduler_max_parallel_observed: compute_max_parallel_observed(tr),
        },
    }
}

pub(crate) fn read_scores_if_present(
    run_paths: &artifacts::RunArtifactPaths,
) -> Option<ScoresArtifact> {
    let path = run_paths.scores_json();
    let raw = std::fs::read_to_string(path).ok()?;
    serde_json::from_str::<ScoresArtifact>(&raw).ok()
}

pub(crate) fn build_suggestions_artifact(
    run_summary: &RunSummaryArtifact,
    scores: Option<&ScoresArtifact>,
) -> SuggestionsArtifact {
    let fallback_summary;
    let fallback_metrics;
    let (score_summary, score_metrics, score_version) = if let Some(scores) = scores {
        (
            &scores.summary,
            &scores.metrics,
            Some(scores.scores_version),
        )
    } else {
        let failed_steps = run_summary.counts.failed_steps;
        let success_steps = run_summary
            .counts
            .completed_steps
            .saturating_sub(failed_steps);
        let success_ratio = if run_summary.counts.total_steps == 0 {
            1.0
        } else {
            let permille = (success_steps * 1000) / run_summary.counts.total_steps;
            (permille as f64) / 1000.0
        };
        let security_denied_count: usize =
            run_summary.policy.security_denials_by_code.values().sum();
        let delegation_denied_count: usize = run_summary
            .policy
            .security_denials_by_code
            .iter()
            .filter_map(|(code, count)| {
                if code.starts_with("DELEGATION_") {
                    Some(*count)
                } else {
                    None
                }
            })
            .sum();
        fallback_summary = ScoresSummary {
            success_ratio,
            failure_count: failed_steps,
            retry_count: 0,
            delegation_denied_count,
            security_denied_count,
        };
        fallback_metrics = ScoresMetrics {
            scheduler_max_parallel_observed: 1,
        };
        (&fallback_summary, &fallback_metrics, None)
    };

    let base_evidence = SuggestionEvidence {
        failure_count: score_summary.failure_count,
        retry_count: score_summary.retry_count,
        delegation_denied_count: score_summary.delegation_denied_count,
        security_denied_count: score_summary.security_denied_count,
        success_ratio: score_summary.success_ratio,
        scheduler_max_parallel_observed: score_metrics.scheduler_max_parallel_observed,
    };

    let mut suggestions = Vec::new();

    if score_summary.failure_count > 0 {
        suggestions.push(SuggestionItem {
            id: String::new(),
            category: "retry".to_string(),
            severity: "improvement".to_string(),
            rationale: "One or more steps failed; consider safer retry policy for transient paths."
                .to_string(),
            evidence: base_evidence.clone(),
            proposed_change: SuggestedChangeIntent {
                intent: "increase_step_retry_budget".to_string(),
                target: "failed-step-set".to_string(),
            },
        });
    }
    if score_summary.delegation_denied_count > 0 {
        suggestions.push(SuggestionItem {
            id: String::new(),
            category: "delegation".to_string(),
            severity: "warning".to_string(),
            rationale: "Delegation-denied signals detected; review delegation policy scope."
                .to_string(),
            evidence: base_evidence.clone(),
            proposed_change: SuggestedChangeIntent {
                intent: "review_delegation_policy_scope".to_string(),
                target: "delegation-boundary".to_string(),
            },
        });
    }
    if score_summary.security_denied_count > 0 {
        suggestions.push(SuggestionItem {
            id: String::new(),
            category: "security".to_string(),
            severity: "warning".to_string(),
            rationale: "Security denials observed; align expected capabilities with trust policy."
                .to_string(),
            evidence: base_evidence.clone(),
            proposed_change: SuggestedChangeIntent {
                intent: "review_security_policy_expectations".to_string(),
                target: "security-envelope".to_string(),
            },
        });
    }
    if score_summary.success_ratio < 1.0 {
        suggestions.push(SuggestionItem {
            id: String::new(),
            category: "general".to_string(),
            severity: "improvement".to_string(),
            rationale: "Success ratio is below 1.0; review failing steps and dependency shape."
                .to_string(),
            evidence: base_evidence.clone(),
            proposed_change: SuggestedChangeIntent {
                intent: "review_failure_hotspots".to_string(),
                target: "workflow-step-dependencies".to_string(),
            },
        });
    }
    if run_summary.counts.total_steps > 1 && score_metrics.scheduler_max_parallel_observed <= 1 {
        suggestions.push(SuggestionItem {
            id: String::new(),
            category: "scheduler".to_string(),
            severity: "info".to_string(),
            rationale: "Observed parallelism is low; evaluate opportunities for safe concurrency."
                .to_string(),
            evidence: base_evidence,
            proposed_change: SuggestedChangeIntent {
                intent: "evaluate_parallelizable_dependencies".to_string(),
                target: "workflow-structure".to_string(),
            },
        });
    }

    for (idx, suggestion) in suggestions.iter_mut().enumerate() {
        suggestion.id = format!("sug-{:03}", idx + 1);
    }

    SuggestionsArtifact {
        suggestions_version: SUGGESTIONS_VERSION,
        run_id: run_summary.run_id.clone(),
        generated_from: SuggestionsGeneratedFrom {
            artifact_model_version: run_summary.artifact_model_version,
            run_summary_version: run_summary.run_summary_version,
            scores_version: score_version,
        },
        suggestions,
    }
}

fn aee_decision_kind_for_intent(intent: &str) -> (&'static str, &'static str) {
    match intent {
        "increase_step_retry_budget" => (
            "bounded_retry_recovery",
            "raise retry budget for the failed step set on the next bounded run",
        ),
        "evaluate_parallelizable_dependencies" => (
            "bounded_scheduler_review",
            "review whether the workflow can safely increase bounded parallelism",
        ),
        "review_delegation_policy_scope" => (
            "bounded_delegation_review",
            "review delegation boundaries before the next bounded run",
        ),
        "review_security_policy_expectations" => (
            "bounded_security_review",
            "review trust-policy expectations before the next bounded run",
        ),
        "review_failure_hotspots" => (
            "bounded_failure_review",
            "review failing dependency hotspots before the next bounded run",
        ),
        _ => (
            "bounded_runtime_review",
            "review bounded runtime signals before the next run",
        ),
    }
}

pub(crate) fn build_affect_state_artifact(
    run_summary: &RunSummaryArtifact,
    suggestions: &SuggestionsArtifact,
    scores: Option<&ScoresArtifact>,
) -> AffectStateArtifact {
    let selected = suggestions
        .suggestions
        .first()
        .cloned()
        .unwrap_or_else(|| SuggestionItem {
            id: "sug-000".to_string(),
            category: "stability".to_string(),
            severity: "info".to_string(),
            rationale: "No bounded adaptation signals fired; keep current policy state."
                .to_string(),
            evidence: SuggestionEvidence {
                failure_count: 0,
                retry_count: 0,
                delegation_denied_count: 0,
                security_denied_count: 0,
                success_ratio: 1.0,
                scheduler_max_parallel_observed: 1,
            },
            proposed_change: SuggestedChangeIntent {
                intent: "maintain_current_policy".to_string(),
                target: "workflow-runtime".to_string(),
            },
        });

    let recovery_bias = if selected.evidence.failure_count > 0
        || selected.proposed_change.intent == "increase_step_retry_budget"
    {
        2
    } else if selected.evidence.retry_count > 0 {
        1
    } else {
        0
    };

    let (affect_mode, urgency_level, frustration_level, confidence_shift, downstream_priority) =
        if recovery_bias >= 2 {
            (
                "recovery_focus",
                "elevated",
                "high",
                "reduced",
                "prefer bounded recovery before broader runtime review",
            )
        } else if recovery_bias == 1 {
            (
                "watchful_adjustment",
                "guarded",
                "moderate",
                "stable",
                "stabilize retries before expanding scope",
            )
        } else {
            (
                "steady_state",
                "low",
                "low",
                "stable",
                "maintain current bounded runtime policy",
            )
        };

    let update_reason = format!(
        "failure_count={} retry_count={} success_ratio={} selected_intent={}",
        selected.evidence.failure_count,
        selected.evidence.retry_count,
        selected.evidence.success_ratio,
        selected.proposed_change.intent
    );

    AffectStateArtifact {
        affect_state_version: 1,
        run_id: run_summary.run_id.clone(),
        generated_from: AeeDecisionGeneratedFrom {
            artifact_model_version: run_summary.artifact_model_version,
            run_summary_version: run_summary.run_summary_version,
            suggestions_version: suggestions.suggestions_version,
            scores_version: scores.map(|value| value.scores_version),
        },
        affect: AffectStateRecord {
            affect_state_id: "affect-001".to_string(),
            affect_mode: affect_mode.to_string(),
            urgency_level: urgency_level.to_string(),
            frustration_level: frustration_level.to_string(),
            confidence_shift: confidence_shift.to_string(),
            recovery_bias,
            downstream_priority: downstream_priority.to_string(),
            update_reason,
            deterministic_update_rule:
                "derive affect mode and recovery bias from the first stable suggestion plus bounded failure and retry evidence"
                    .to_string(),
        },
    }
}

#[cfg(test)]
pub(crate) fn build_cognitive_signals_state(
    run_summary: &RunSummaryArtifact,
    suggestions: &SuggestionsArtifact,
    _scores: Option<&ScoresArtifact>,
) -> CognitiveSignalsState {
    let selected = suggestions
        .suggestions
        .first()
        .cloned()
        .unwrap_or_else(|| SuggestionItem {
            id: "sug-000".to_string(),
            category: "stability".to_string(),
            severity: "info".to_string(),
            rationale: "No bounded adaptation signals fired; keep current policy state."
                .to_string(),
            evidence: SuggestionEvidence {
                failure_count: 0,
                retry_count: 0,
                delegation_denied_count: 0,
                security_denied_count: 0,
                success_ratio: 1.0,
                scheduler_max_parallel_observed: 1,
            },
            proposed_change: SuggestedChangeIntent {
                intent: "maintain_current_policy".to_string(),
                target: "workflow-runtime".to_string(),
            },
        });

    let completion_pressure =
        if selected.evidence.failure_count > 0 || run_summary.status == "failure" {
            "elevated"
        } else if selected.evidence.retry_count > 0 {
            "guarded"
        } else {
            "steady"
        };
    let integrity_bias = if selected.evidence.security_denied_count > 0 {
        "high"
    } else {
        "bounded"
    };
    let curiosity_bias = if selected.evidence.success_ratio < 1.0 {
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
    let salience_level = if selected.severity == "high" || selected.evidence.failure_count > 0 {
        "high"
    } else if selected.evidence.retry_count > 0 {
        "moderate"
    } else {
        "low"
    };
    let persistence_pressure = if completion_pressure == "elevated" {
        "retry_biased"
    } else if selected.evidence.retry_count > 0 {
        "stabilize_then_retry"
    } else {
        "bounded_once"
    };
    let confidence_shift = if selected.evidence.failure_count > 0 {
        "reduced"
    } else {
        "stable"
    };
    let downstream_influence = format!(
        "dominant_instinct={} selected_intent={} failure_count={} retry_count={}",
        dominant_instinct,
        selected.proposed_change.intent,
        selected.evidence.failure_count,
        selected.evidence.retry_count
    );

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
        downstream_influence,
    }
}

#[cfg(test)]
pub(crate) fn build_cognitive_signals_artifact(
    run_summary: &RunSummaryArtifact,
    suggestions: &SuggestionsArtifact,
    scores: Option<&ScoresArtifact>,
) -> CognitiveSignalsArtifact {
    let state = build_cognitive_signals_state(run_summary, suggestions, scores);
    build_cognitive_signals_artifact_from_state(run_summary, &state, suggestions, scores)
}

pub(crate) fn build_cognitive_signals_artifact_from_state(
    run_summary: &RunSummaryArtifact,
    state: &execute::CognitiveSignalsState,
    suggestions: &SuggestionsArtifact,
    scores: Option<&ScoresArtifact>,
) -> CognitiveSignalsArtifact {
    CognitiveSignalsArtifact {
        cognitive_signals_version: COGNITIVE_SIGNALS_VERSION,
        run_id: run_summary.run_id.clone(),
        generated_from: AeeDecisionGeneratedFrom {
            artifact_model_version: run_summary.artifact_model_version,
            run_summary_version: run_summary.run_summary_version,
            suggestions_version: suggestions.suggestions_version,
            scores_version: scores.map(|value| value.scores_version),
        },
        instinct: CognitiveInstinctRecord {
            instinct_profile_id: "instinct-001".to_string(),
            dominant_instinct: state.dominant_instinct.clone(),
            completion_pressure: state.completion_pressure.clone(),
            integrity_bias: state.integrity_bias.clone(),
            curiosity_bias: state.curiosity_bias.clone(),
            candidate_selection_bias: state.candidate_selection_bias.clone(),
            deterministic_update_rule:
                "derive bounded instinct profile from stable failure, retry, security, and success evidence ordering"
                    .to_string(),
        },
        affect: CognitiveAffectSignalRecord {
            affect_state_id: "signal-affect-001".to_string(),
            urgency_level: state.urgency_level.clone(),
            salience_level: state.salience_level.clone(),
            persistence_pressure: state.persistence_pressure.clone(),
            confidence_shift: state.confidence_shift.clone(),
            downstream_influence: state.downstream_influence.clone(),
            deterministic_update_rule:
                "derive bounded affect signals from the first stable suggestion plus bounded run summary evidence"
                    .to_string(),
        },
    }
}

#[cfg(test)]
pub(crate) fn build_cognitive_arbitration_state(
    _run_summary: &RunSummaryArtifact,
    suggestions: &SuggestionsArtifact,
    signals: &CognitiveSignalsArtifact,
    affect_state: &AffectStateArtifact,
) -> CognitiveArbitrationState {
    let selected = suggestions
        .suggestions
        .first()
        .cloned()
        .unwrap_or_else(|| SuggestionItem {
            id: "sug-000".to_string(),
            category: "stability".to_string(),
            severity: "info".to_string(),
            rationale: "No bounded adaptation signals fired; keep current policy state."
                .to_string(),
            evidence: SuggestionEvidence {
                failure_count: 0,
                retry_count: 0,
                delegation_denied_count: 0,
                security_denied_count: 0,
                success_ratio: 1.0,
                scheduler_max_parallel_observed: 1,
            },
            proposed_change: SuggestedChangeIntent {
                intent: "maintain_current_policy".to_string(),
                target: "workflow-runtime".to_string(),
            },
        });

    let (route_selected, reasoning_mode) = if selected.evidence.security_denied_count > 0
        || selected.evidence.failure_count > 0
        || signals.instinct.integrity_bias == "reinforced"
    {
        ("slow", "review_heavy")
    } else if affect_state.affect.recovery_bias >= 2
        || selected.evidence.retry_count > 0
        || signals.affect.confidence_shift == "reduced"
        || signals.affect.persistence_pressure == "sustained"
    {
        ("hybrid", "bounded_recovery")
    } else {
        ("fast", "direct_execution")
    };
    let risk_class = if selected.evidence.security_denied_count > 0 {
        "high"
    } else if selected.evidence.failure_count > 0 || affect_state.affect.recovery_bias >= 2 {
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
    if selected.evidence.security_denied_count > 0 {
        applied_constraints.push("security_denial_present".to_string());
    }
    if selected.evidence.failure_count > 0 {
        applied_constraints.push("failure_recovery_bias".to_string());
    }
    if selected.evidence.retry_count > 0 {
        applied_constraints.push("retry_budget_pressure".to_string());
    }
    if applied_constraints.is_empty() {
        applied_constraints.push("bounded_default_path".to_string());
    }

    let cost_latency_assumption = match route_selected {
        "fast" => "prefer lower-cost low-latency execution when bounded evidence is stable",
        "hybrid" => "allow bounded extra review when retry or recovery pressure is present",
        _ => "spend bounded additional cognition when failure or policy risk is present",
    };
    let route_reason = format!(
        "route={} dominant_instinct={} confidence_shift={} affect_mode={} failure_count={} retry_count={} security_denied_count={} selected_intent={}",
        route_selected,
        signals.instinct.dominant_instinct,
        signals.affect.confidence_shift,
        affect_state.affect.affect_mode,
        selected.evidence.failure_count,
        selected.evidence.retry_count,
        selected.evidence.security_denied_count,
        selected.proposed_change.intent
    );

    CognitiveArbitrationState {
        route_selected: route_selected.to_string(),
        reasoning_mode: reasoning_mode.to_string(),
        confidence: confidence.to_string(),
        risk_class: risk_class.to_string(),
        applied_constraints,
        cost_latency_assumption: cost_latency_assumption.to_string(),
        route_reason,
    }
}

#[cfg(test)]
pub(crate) fn build_cognitive_arbitration_artifact(
    run_summary: &RunSummaryArtifact,
    suggestions: &SuggestionsArtifact,
    signals: &CognitiveSignalsArtifact,
    affect_state: &AffectStateArtifact,
    scores: Option<&ScoresArtifact>,
) -> CognitiveArbitrationArtifact {
    let state = build_cognitive_arbitration_state(run_summary, suggestions, signals, affect_state);
    build_cognitive_arbitration_artifact_from_state(run_summary, suggestions, &state, scores)
}

pub(crate) fn build_cognitive_arbitration_artifact_from_state(
    run_summary: &RunSummaryArtifact,
    suggestions: &SuggestionsArtifact,
    state: &execute::CognitiveArbitrationState,
    scores: Option<&ScoresArtifact>,
) -> CognitiveArbitrationArtifact {
    CognitiveArbitrationArtifact {
        cognitive_arbitration_version: COGNITIVE_ARBITRATION_VERSION,
        run_id: run_summary.run_id.clone(),
        generated_from: AeeDecisionGeneratedFrom {
            artifact_model_version: run_summary.artifact_model_version,
            run_summary_version: run_summary.run_summary_version,
            suggestions_version: suggestions.suggestions_version,
            scores_version: scores.map(|value| value.scores_version),
        },
        route_selected: state.route_selected.clone(),
        reasoning_mode: state.reasoning_mode.clone(),
        confidence: state.confidence.clone(),
        risk_class: state.risk_class.clone(),
        applied_constraints: state.applied_constraints.clone(),
        cost_latency_assumption: state.cost_latency_assumption.clone(),
        route_reason: state.route_reason.clone(),
        deterministic_selection_rule:
            "derive route from runtime signal state, bounded affect recovery bias, and stable failure/security/retry evidence ordering"
                .to_string(),
    }
}

#[cfg(test)]
pub(crate) fn build_fast_slow_path_state(
    arbitration: &CognitiveArbitrationArtifact,
) -> FastSlowPathState {
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

pub(crate) fn build_fast_slow_path_artifact(
    run_summary: &RunSummaryArtifact,
    arbitration: &CognitiveArbitrationArtifact,
    state: &execute::FastSlowPathState,
    scores: Option<&ScoresArtifact>,
) -> FastSlowPathArtifact {
    FastSlowPathArtifact {
        fast_slow_path_version: FAST_SLOW_PATH_VERSION,
        run_id: run_summary.run_id.clone(),
        generated_from: AeeDecisionGeneratedFrom {
            artifact_model_version: run_summary.artifact_model_version,
            run_summary_version: run_summary.run_summary_version,
            suggestions_version: arbitration.generated_from.suggestions_version,
            scores_version: scores.map(|value| value.scores_version),
        },
        arbitration_route: arbitration.route_selected.clone(),
        selected_path: state.selected_path.clone(),
        path_family: state.path_family.clone(),
        runtime_branch_taken: state.runtime_branch_taken.clone(),
        handoff_state: state.handoff_state.clone(),
        candidate_strategy: state.candidate_strategy.clone(),
        review_depth: state.review_depth.clone(),
        execution_profile: state.execution_profile.clone(),
        termination_expectation: state.termination_expectation.clone(),
        path_difference_summary: state.path_difference_summary.clone(),
        deterministic_handoff_rule:
            "derive fast/slow path handoff and branch selection directly from bounded arbitration route selection before downstream candidate generation"
                .to_string(),
    }
}

#[cfg(test)]
pub(crate) fn build_agency_selection_state(
    signals: &CognitiveSignalsArtifact,
    arbitration: &CognitiveArbitrationArtifact,
    fast_slow_state: &FastSlowPathState,
    fast_slow_path: &FastSlowPathArtifact,
) -> AgencySelectionState {
    let (
        selection_mode,
        candidate_set,
        selected_candidate_id,
        selected_candidate_kind,
        selected_candidate_action,
        selected_candidate_reason,
    ) = match fast_slow_path.selected_path.as_str() {
        "fast_path" => {
            let candidate_set = vec![
                    AgencyCandidateRecord {
                        candidate_id: "cand-fast-execute".to_string(),
                        candidate_kind: "direct_execution".to_string(),
                        bounded_action: "execute selected candidate directly under bounded once semantics".to_string(),
                        review_requirement: "minimal".to_string(),
                        execution_priority: 1,
                        rationale: format!(
                            "route={} dominant_instinct={} confidence={}",
                            arbitration.route_selected, signals.instinct.dominant_instinct, arbitration.confidence
                        ),
                    },
                    AgencyCandidateRecord {
                        candidate_id: "cand-fast-verify".to_string(),
                        candidate_kind: "bounded_verification".to_string(),
                        bounded_action: "perform one bounded verification pass before execution".to_string(),
                        review_requirement: "light".to_string(),
                        execution_priority: 2,
                        rationale: "keep a fallback candidate available without changing the primary fast-path commitment".to_string(),
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
                        bounded_action: "review, refine, or veto the current candidate before execution".to_string(),
                        review_requirement: "verification_required".to_string(),
                        execution_priority: 1,
                        rationale: format!(
                            "route={} dominant_instinct={} risk_class={}",
                            arbitration.route_selected, signals.instinct.dominant_instinct, arbitration.risk_class
                        ),
                    },
                    AgencyCandidateRecord {
                        candidate_id: "cand-slow-direct".to_string(),
                        candidate_kind: "direct_execution".to_string(),
                        bounded_action: "execute the current candidate without additional refinement".to_string(),
                        review_requirement: "minimal".to_string(),
                        execution_priority: 2,
                        rationale: "retain the direct-execution alternative as a bounded comparator candidate".to_string(),
                    },
                    AgencyCandidateRecord {
                        candidate_id: "cand-slow-defer".to_string(),
                        candidate_kind: "bounded_deferral".to_string(),
                        bounded_action: "defer execution and surface the candidate set for later gate/review stages".to_string(),
                        review_requirement: "review_required".to_string(),
                        execution_priority: 3,
                        rationale: "preserve a bounded non-execution option when policy or review pressure remains elevated".to_string(),
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
            fast_slow_path.selected_path,
            fast_slow_state.runtime_branch_taken,
            arbitration.route_selected,
            signals.instinct.candidate_selection_bias
        ),
        selection_mode: selection_mode.to_string(),
        candidate_set,
        selected_candidate_id,
        selected_candidate_kind,
        selected_candidate_action,
        selected_candidate_reason,
    }
}

pub(crate) fn build_agency_selection_artifact(
    run_summary: &RunSummaryArtifact,
    arbitration: &CognitiveArbitrationArtifact,
    state: &execute::AgencySelectionState,
    scores: Option<&ScoresArtifact>,
) -> AgencySelectionArtifact {
    AgencySelectionArtifact {
        agency_selection_version: AGENCY_SELECTION_VERSION,
        run_id: run_summary.run_id.clone(),
        generated_from: AeeDecisionGeneratedFrom {
            artifact_model_version: run_summary.artifact_model_version,
            run_summary_version: run_summary.run_summary_version,
            suggestions_version: arbitration.generated_from.suggestions_version,
            scores_version: scores.map(|value| value.scores_version),
        },
        candidate_generation_basis: state.candidate_generation_basis.clone(),
        selection_mode: state.selection_mode.clone(),
        candidate_set: state.candidate_set.clone(),
        selected_candidate_id: state.selected_candidate_id.clone(),
        selected_candidate_reason: state.selected_candidate_reason.clone(),
        deterministic_selection_rule:
            "derive the bounded candidate set and selected candidate from the already-selected fast/slow runtime branch, arbitration route, and instinct bias without hidden initiative state"
                .to_string(),
    }
}

#[cfg(test)]
pub(crate) fn build_bounded_execution_state(
    run_summary: &RunSummaryArtifact,
    _fast_slow_path: &FastSlowPathArtifact,
    _agency_selection: &AgencySelectionArtifact,
    agency_state: &AgencySelectionState,
) -> BoundedExecutionState {
    let (execution_status, continuation_state, provisional_termination_state, iterations) =
        match agency_state.selected_candidate_kind.as_str() {
            "direct_execution" => (
                "completed",
                "stop_after_one",
                "ready_for_evaluation",
                vec![BoundedExecutionIteration {
                    iteration_index: 1,
                    stage: "execute".to_string(),
                    action: agency_state.selected_candidate_action.clone(),
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
                        action: agency_state.selected_candidate_action.clone(),
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
                    action: agency_state.selected_candidate_action.clone(),
                    outcome: "bounded_deferral_recorded".to_string(),
                }],
            ),
        };

    let execution_status = if run_summary.status == "failure" {
        "completed_with_failure_signal"
    } else {
        execution_status
    };
    let continuation_state = if run_summary.status == "failure" && iterations.len() > 1 {
        "bounded_review_complete_with_failure_signal"
    } else {
        continuation_state
    };
    let provisional_termination_state = if run_summary.status == "failure" {
        "ready_for_runtime_evaluation"
    } else {
        provisional_termination_state
    };

    BoundedExecutionState {
        execution_status: execution_status.to_string(),
        continuation_state: continuation_state.to_string(),
        provisional_termination_state: provisional_termination_state.to_string(),
        iterations,
    }
}

pub(crate) fn build_bounded_execution_artifact(
    run_summary: &RunSummaryArtifact,
    fast_slow_path: &FastSlowPathArtifact,
    agency_selection: &AgencySelectionArtifact,
    state: &BoundedExecutionState,
    scores: Option<&ScoresArtifact>,
) -> BoundedExecutionArtifact {
    BoundedExecutionArtifact {
        bounded_execution_version: BOUNDED_EXECUTION_VERSION,
        run_id: run_summary.run_id.clone(),
        generated_from: AeeDecisionGeneratedFrom {
            artifact_model_version: run_summary.artifact_model_version,
            run_summary_version: run_summary.run_summary_version,
            suggestions_version: agency_selection.generated_from.suggestions_version,
            scores_version: scores.map(|value| value.scores_version),
        },
        selected_candidate_id: agency_selection.selected_candidate_id.clone(),
        selected_path: fast_slow_path.selected_path.clone(),
        execution_status: state.execution_status.clone(),
        continuation_state: state.continuation_state.clone(),
        provisional_termination_state: state.provisional_termination_state.clone(),
        iteration_count: state.iterations.len() as u32,
        iterations: state.iterations.clone(),
        deterministic_execution_rule:
            "derive bounded iteration shape, continuation state, and provisional termination from runtime loop state without hidden retry state"
                .to_string(),
    }
}

#[cfg(test)]
pub(crate) fn build_evaluation_control_state(
    run_summary: &RunSummaryArtifact,
    bounded_execution: &BoundedExecutionArtifact,
) -> EvaluationControlState {
    let (
        progress_signal,
        contradiction_signal,
        failure_signal,
        termination_reason,
        behavior_effect,
        next_control_action,
    ) = if run_summary.status == "failure" {
        (
            "stalled_progress",
            "present",
            "bounded_failure_detected",
            if bounded_execution.iteration_count > 1 {
                "bounded_failure"
            } else {
                "no_progress"
            },
            "emit bounded failure/termination signals for later reframing or policy handling",
            if bounded_execution.iteration_count > 1 {
                "handoff_to_reframing"
            } else {
                "terminate_with_failure"
            },
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

pub(crate) fn build_evaluation_signals_artifact(
    run_summary: &RunSummaryArtifact,
    fast_slow_path: &FastSlowPathArtifact,
    agency_selection: &AgencySelectionArtifact,
    state: &EvaluationControlState,
    scores: Option<&ScoresArtifact>,
) -> EvaluationSignalsArtifact {
    EvaluationSignalsArtifact {
        evaluation_signals_version: EVALUATION_SIGNALS_VERSION,
        run_id: run_summary.run_id.clone(),
        generated_from: AeeDecisionGeneratedFrom {
            artifact_model_version: run_summary.artifact_model_version,
            run_summary_version: run_summary.run_summary_version,
            suggestions_version: agency_selection.generated_from.suggestions_version,
            scores_version: scores.map(|value| value.scores_version),
        },
        selected_candidate_id: agency_selection.selected_candidate_id.clone(),
        selected_path: fast_slow_path.selected_path.clone(),
        progress_signal: state.progress_signal.clone(),
        contradiction_signal: state.contradiction_signal.clone(),
        failure_signal: state.failure_signal.clone(),
        termination_reason: state.termination_reason.clone(),
        behavior_effect: state.behavior_effect.clone(),
        next_control_action: state.next_control_action.clone(),
        deterministic_evaluation_rule:
            "derive bounded evaluation, termination, and next control action from runtime execution evidence without hidden continuation state"
                .to_string(),
    }
}

pub(crate) fn build_aee_decision_artifact(
    run_summary: &RunSummaryArtifact,
    suggestions: &SuggestionsArtifact,
    affect_state: &AffectStateArtifact,
    scores: Option<&ScoresArtifact>,
) -> AeeDecisionArtifact {
    let selected = suggestions
        .suggestions
        .first()
        .cloned()
        .unwrap_or_else(|| SuggestionItem {
            id: "sug-000".to_string(),
            category: "stability".to_string(),
            severity: "info".to_string(),
            rationale: "No bounded adaptation signals fired; keep current policy state."
                .to_string(),
            evidence: SuggestionEvidence {
                failure_count: 0,
                retry_count: 0,
                delegation_denied_count: 0,
                security_denied_count: 0,
                success_ratio: 1.0,
                scheduler_max_parallel_observed: 1,
            },
            proposed_change: SuggestedChangeIntent {
                intent: "maintain_current_policy".to_string(),
                target: "workflow-runtime".to_string(),
            },
        });
    let (decision_kind, expected_downstream_effect) =
        aee_decision_kind_for_intent(&selected.proposed_change.intent);
    let recommended_retry_budget = (selected.proposed_change.intent
        == "increase_step_retry_budget"
        && affect_state.affect.recovery_bias >= 2)
        .then_some(2);
    let expected_downstream_effect = if let Some(budget) = recommended_retry_budget {
        format!(
            "{expected_downstream_effect}; affect-guided recovery bias recommends retry budget max_attempts={budget}"
        )
    } else {
        expected_downstream_effect.to_string()
    };

    AeeDecisionArtifact {
        aee_decision_version: AEE_DECISION_VERSION,
        run_id: run_summary.run_id.clone(),
        generated_from: AeeDecisionGeneratedFrom {
            artifact_model_version: run_summary.artifact_model_version,
            run_summary_version: run_summary.run_summary_version,
            suggestions_version: suggestions.suggestions_version,
            scores_version: scores.map(|value| value.scores_version),
        },
        affect_state: AffectStateRef {
            affect_state_id: affect_state.affect.affect_state_id.clone(),
            affect_mode: affect_state.affect.affect_mode.clone(),
            downstream_priority: affect_state.affect.downstream_priority.clone(),
            recovery_bias: affect_state.affect.recovery_bias,
        },
        decision: AeeDecisionRecord {
            decision_id: "aee-001".to_string(),
            decision_kind: decision_kind.to_string(),
            selected_suggestion_id: selected.id,
            category: selected.category,
            intent: selected.proposed_change.intent,
            target: selected.proposed_change.target,
            rationale: selected.rationale,
            expected_downstream_effect,
            deterministic_selection_rule:
                "select the first suggestion emitted by build_suggestions_artifact after stable category ordering, then apply the deterministic affect-state recovery bias"
                    .to_string(),
            recommended_retry_budget,
            evidence: selected.evidence,
        },
    }
}

pub(crate) fn build_reasoning_graph_artifact(
    run_summary: &RunSummaryArtifact,
    affect_state: &AffectStateArtifact,
    aee_decision: &AeeDecisionArtifact,
    scores: Option<&ScoresArtifact>,
) -> ReasoningGraphArtifact {
    let retry_score = if affect_state.affect.recovery_bias >= 2 {
        92
    } else if affect_state.affect.recovery_bias == 1 {
        68
    } else {
        22
    };
    let maintain_score = if affect_state.affect.recovery_bias == 0 {
        88
    } else {
        36
    };
    let failure_signal_score = if aee_decision.decision.evidence.failure_count > 0 {
        80
    } else {
        24
    };

    let mut action_nodes = [
        ReasoningGraphNode {
            node_id: "action.retry_budget".to_string(),
            node_kind: "action".to_string(),
            label: "increase retry budget".to_string(),
            rank: 0,
            priority_score: retry_score,
            affect_mode: Some(affect_state.affect.affect_mode.clone()),
            rationale: format!(
                "Affect-guided recovery bias {} favors bounded retry recovery.",
                affect_state.affect.recovery_bias
            ),
        },
        ReasoningGraphNode {
            node_id: "action.maintain_policy".to_string(),
            node_kind: "action".to_string(),
            label: "maintain current policy".to_string(),
            rank: 0,
            priority_score: maintain_score,
            affect_mode: Some(affect_state.affect.affect_mode.clone()),
            rationale: "Steady-state or low-bias runs preserve the current bounded runtime policy."
                .to_string(),
        },
    ];
    action_nodes.sort_by(|a, b| {
        b.priority_score
            .cmp(&a.priority_score)
            .then_with(|| a.node_id.cmp(&b.node_id))
    });
    for (idx, node) in action_nodes.iter_mut().enumerate() {
        node.rank = (idx + 1) as u32;
    }

    let selected_node = action_nodes
        .iter()
        .find(|node| {
            (aee_decision.decision.intent == "increase_step_retry_budget"
                && node.node_id == "action.retry_budget")
                || (aee_decision.decision.intent != "increase_step_retry_budget"
                    && node.node_id == "action.maintain_policy")
        })
        .cloned()
        .unwrap_or_else(|| action_nodes[0].clone());

    let nodes = vec![
        ReasoningGraphNode {
            node_id: "evidence.runtime".to_string(),
            node_kind: "evidence".to_string(),
            label: "bounded runtime evidence".to_string(),
            rank: 1,
            priority_score: failure_signal_score,
            affect_mode: None,
            rationale: format!(
                "failure_count={} retry_count={} success_ratio={}",
                aee_decision.decision.evidence.failure_count,
                aee_decision.decision.evidence.retry_count,
                aee_decision.decision.evidence.success_ratio
            ),
        },
        ReasoningGraphNode {
            node_id: "affect.current".to_string(),
            node_kind: "affect".to_string(),
            label: affect_state.affect.affect_mode.replace('_', " "),
            rank: 1,
            priority_score: 100,
            affect_mode: Some(affect_state.affect.affect_mode.clone()),
            rationale: affect_state.affect.downstream_priority.clone(),
        },
        action_nodes[0].clone(),
        action_nodes[1].clone(),
    ];

    let edges = vec![
        ReasoningGraphEdge {
            edge_id: "edge-001".to_string(),
            from: "evidence.runtime".to_string(),
            to: "affect.current".to_string(),
            relation: "updates".to_string(),
            rationale: affect_state.affect.update_reason.clone(),
        },
        ReasoningGraphEdge {
            edge_id: "edge-002".to_string(),
            from: "affect.current".to_string(),
            to: "action.retry_budget".to_string(),
            relation: "prioritizes".to_string(),
            rationale: "High recovery bias increases retry-budget priority.".to_string(),
        },
        ReasoningGraphEdge {
            edge_id: "edge-003".to_string(),
            from: "affect.current".to_string(),
            to: "action.maintain_policy".to_string(),
            relation: "prioritizes".to_string(),
            rationale: "Low recovery bias preserves the maintain-current-policy path.".to_string(),
        },
    ];

    ReasoningGraphArtifact {
        reasoning_graph_version: REASONING_GRAPH_VERSION,
        run_id: run_summary.run_id.clone(),
        generated_from: AeeDecisionGeneratedFrom {
            artifact_model_version: run_summary.artifact_model_version,
            run_summary_version: run_summary.run_summary_version,
            suggestions_version: affect_state.generated_from.suggestions_version,
            scores_version: scores.map(|value| value.scores_version),
        },
        graph: ReasoningGraphRecord {
            graph_id: "reasoning-graph-001".to_string(),
            dominant_affect_mode: affect_state.affect.affect_mode.clone(),
            ranking_rule:
                "sort action nodes by descending priority_score, then lexicographic node_id"
                    .to_string(),
            selected_path: ReasoningGraphSelection {
                selected_node_id: selected_node.node_id,
                selected_intent: aee_decision.decision.intent.clone(),
                selected_target: aee_decision.decision.target.clone(),
                graph_derived_output: aee_decision.decision.expected_downstream_effect.clone(),
                affect_changed_ranking: affect_state.affect.recovery_bias > 0,
            },
            nodes,
            edges,
        },
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn write_run_state_artifacts(
    resolved: &resolve::AdlResolved,
    tr: &trace::Trace,
    adl_path: &Path,
    _out_dir: &Path,
    start_ms: u128,
    end_ms: u128,
    status: &str,
    pause: Option<&execute::PauseState>,
    steering_history: &[execute::SteeringRecord],
    runtime_control: &execute::RuntimeControlState,
    resume_completed_step_ids: Option<&HashSet<String>>,
    failure: Option<&anyhow::Error>,
) -> Result<PathBuf> {
    let run_paths = artifacts::RunArtifactPaths::for_run(&resolved.run_id)?;
    run_paths.ensure_layout()?;
    run_paths.write_model_marker()?;
    let run_dir = run_paths.run_dir();
    let resume_completed: BTreeSet<String> = resume_completed_step_ids
        .map(|ids| ids.iter().cloned().collect())
        .unwrap_or_default();

    let mut status_by_step: HashMap<String, String> = HashMap::new();
    for ev in &tr.events {
        if let trace::TraceEvent::StepFinished {
            step_id, success, ..
        } = ev
        {
            let status = if *success { "success" } else { "failure" };
            status_by_step.insert(step_id.clone(), status.to_string());
        }
    }

    let mut steps = Vec::with_capacity(resolved.steps.len());
    for step in &resolved.steps {
        let status = status_by_step
            .get(&step.id)
            .cloned()
            .or_else(|| {
                resume_completed
                    .contains(&step.id)
                    .then(|| "success".to_string())
            })
            .unwrap_or_else(|| "not_run".to_string());
        let output_artifact_path = match (status.as_str(), step.write_to.as_deref()) {
            ("success", Some(write_to)) => Some(write_to.to_string()),
            _ => None,
        };

        let agent_id = step
            .agent
            .as_deref()
            .unwrap_or("<unresolved-agent>")
            .to_string();
        let provider_id = step
            .provider
            .as_deref()
            .unwrap_or("<unresolved-provider>")
            .to_string();

        steps.push(StepStateArtifact {
            step_id: step.id.clone(),
            agent_id,
            provider_id,
            status,
            output_artifact_path,
        });
    }

    let scheduler_policy = execute::scheduler_policy_for_run(resolved)?;
    let error_message = tr.events.iter().rev().find_map(|ev| match ev {
        trace::TraceEvent::RunFailed { message, .. } => Some(message.clone()),
        _ => None,
    });
    let run_artifact = RunStateArtifact {
        schema_version: RUN_STATE_SCHEMA_VERSION.to_string(),
        run_id: resolved.run_id.clone(),
        workflow_id: resolved.workflow_id.clone(),
        version: resolved.doc.version.clone(),
        status: status.to_string(),
        error_message: error_message.clone(),
        start_time_ms: start_ms,
        end_time_ms: end_ms,
        duration_ms: end_ms.saturating_sub(start_ms),
        execution_plan_hash: execution_plan_hash(&resolved.execution_plan)?,
        scheduler_max_concurrency: scheduler_policy.map(|(v, _)| v),
        scheduler_policy_source: scheduler_policy.map(|(_, source)| source.as_str().to_string()),
        steering_history: steering_history.to_vec(),
        pause: pause.cloned(),
    };

    let run_json = serde_json::to_vec_pretty(&run_artifact).context("serialize run.json")?;
    let steps_json = serde_json::to_vec_pretty(&steps).context("serialize steps.json")?;
    let activation_log_path = run_paths.activation_log_json();
    instrumentation::write_trace_artifact(&activation_log_path, &tr.events)?;
    let cluster_groundwork = build_cluster_groundwork_artifact(resolved, &steps, tr);
    let cluster_groundwork_json = serde_json::to_vec_pretty(&cluster_groundwork)
        .context("serialize cluster_groundwork.json")?;
    artifacts::atomic_write(
        &run_paths.cluster_groundwork_json(),
        &cluster_groundwork_json,
    )?;
    let run_summary = build_run_summary(
        resolved,
        status,
        pause,
        &steps,
        tr.events
            .iter()
            .filter(|ev| matches!(ev, trace::TraceEvent::StepFinished { .. }))
            .count(),
        failure,
        &run_paths,
    );
    let overall_status = match status {
        "success" => "succeeded",
        "failure" => "failed",
        "paused" => "running",
        other => other,
    };
    let run_status = build_run_status(
        resolved,
        tr,
        overall_status,
        &steps,
        failure,
        &resume_completed,
    );
    let run_summary_json =
        serde_json::to_vec_pretty(&run_summary).context("serialize run_summary.json")?;
    let run_status_json =
        serde_json::to_vec_pretty(&run_status).context("serialize run_status.json")?;
    let scores = build_scores_artifact(&run_summary, tr);
    let scores_json = serde_json::to_vec_pretty(&scores).context("serialize scores.json")?;
    let scores_for_suggestions = read_scores_if_present(&run_paths).unwrap_or(scores.clone());
    let suggestions = build_suggestions_artifact(&run_summary, Some(&scores_for_suggestions));
    let suggestions_json =
        serde_json::to_vec_pretty(&suggestions).context("serialize suggestions.json")?;
    let cognitive_signals = build_cognitive_signals_artifact_from_state(
        &run_summary,
        &runtime_control.signals,
        &suggestions,
        Some(&scores_for_suggestions),
    );
    let cognitive_signals_json = serde_json::to_vec_pretty(&cognitive_signals)
        .context("serialize cognitive_signals.v1.json")?;
    let affect_state =
        build_affect_state_artifact(&run_summary, &suggestions, Some(&scores_for_suggestions));
    let affect_state_json =
        serde_json::to_vec_pretty(&affect_state).context("serialize affect_state.v1.json")?;
    let cognitive_arbitration = build_cognitive_arbitration_artifact_from_state(
        &run_summary,
        &suggestions,
        &runtime_control.arbitration,
        Some(&scores_for_suggestions),
    );
    let fast_slow_path = build_fast_slow_path_artifact(
        &run_summary,
        &cognitive_arbitration,
        &runtime_control.fast_slow,
        Some(&scores_for_suggestions),
    );
    let agency_selection = build_agency_selection_artifact(
        &run_summary,
        &cognitive_arbitration,
        &runtime_control.agency,
        Some(&scores_for_suggestions),
    );
    let bounded_execution = build_bounded_execution_artifact(
        &run_summary,
        &fast_slow_path,
        &agency_selection,
        &runtime_control.bounded_execution,
        Some(&scores_for_suggestions),
    );
    let evaluation_signals = build_evaluation_signals_artifact(
        &run_summary,
        &fast_slow_path,
        &agency_selection,
        &runtime_control.evaluation,
        Some(&scores_for_suggestions),
    );
    let cognitive_arbitration_json = serde_json::to_vec_pretty(&cognitive_arbitration)
        .context("serialize cognitive_arbitration.v1.json")?;
    let fast_slow_path_json =
        serde_json::to_vec_pretty(&fast_slow_path).context("serialize fast_slow_path.v1.json")?;
    let agency_selection_json = serde_json::to_vec_pretty(&agency_selection)
        .context("serialize agency_selection.v1.json")?;
    let bounded_execution_json = serde_json::to_vec_pretty(&bounded_execution)
        .context("serialize bounded_execution.v1.json")?;
    let evaluation_signals_json = serde_json::to_vec_pretty(&evaluation_signals)
        .context("serialize evaluation_signals.v1.json")?;
    let aee_decision = build_aee_decision_artifact(
        &run_summary,
        &suggestions,
        &affect_state,
        Some(&scores_for_suggestions),
    );
    let reasoning_graph = build_reasoning_graph_artifact(
        &run_summary,
        &affect_state,
        &aee_decision,
        Some(&scores_for_suggestions),
    );
    let aee_decision_json =
        serde_json::to_vec_pretty(&aee_decision).context("serialize aee_decision.json")?;
    let reasoning_graph_json =
        serde_json::to_vec_pretty(&reasoning_graph).context("serialize reasoning_graph.v1.json")?;

    artifacts::atomic_write(&run_paths.run_json(), &run_json)?;
    artifacts::atomic_write(&run_paths.steps_json(), &steps_json)?;
    artifacts::atomic_write(&run_paths.run_status_json(), &run_status_json)?;
    artifacts::atomic_write(&run_paths.run_summary_json(), &run_summary_json)?;
    artifacts::atomic_write(&run_paths.scores_json(), &scores_json)?;
    artifacts::atomic_write(&run_paths.suggestions_json(), &suggestions_json)?;
    artifacts::atomic_write(&run_paths.cognitive_signals_json(), &cognitive_signals_json)?;
    artifacts::atomic_write(
        &run_paths.cognitive_arbitration_json(),
        &cognitive_arbitration_json,
    )?;
    artifacts::atomic_write(&run_paths.fast_slow_path_json(), &fast_slow_path_json)?;
    artifacts::atomic_write(&run_paths.agency_selection_json(), &agency_selection_json)?;
    artifacts::atomic_write(&run_paths.bounded_execution_json(), &bounded_execution_json)?;
    artifacts::atomic_write(
        &run_paths.evaluation_signals_json(),
        &evaluation_signals_json,
    )?;
    artifacts::atomic_write(&run_paths.cognitive_signals_json(), &cognitive_signals_json)?;
    artifacts::atomic_write(
        &run_paths.cognitive_arbitration_json(),
        &cognitive_arbitration_json,
    )?;
    artifacts::atomic_write(&run_paths.fast_slow_path_json(), &fast_slow_path_json)?;
    artifacts::atomic_write(&run_paths.agency_selection_json(), &agency_selection_json)?;
    artifacts::atomic_write(&run_paths.bounded_execution_json(), &bounded_execution_json)?;
    artifacts::atomic_write(
        &run_paths.evaluation_signals_json(),
        &evaluation_signals_json,
    )?;
    artifacts::atomic_write(&run_paths.affect_state_json(), &affect_state_json)?;
    artifacts::atomic_write(&run_paths.aee_decision_json(), &aee_decision_json)?;
    artifacts::atomic_write(&run_paths.reasoning_graph_json(), &reasoning_graph_json)?;
    if let Some(pause_payload) = pause {
        let pause_artifact = PauseStateArtifact {
            schema_version: PAUSE_STATE_SCHEMA_VERSION.to_string(),
            run_id: resolved.run_id.clone(),
            workflow_id: resolved.workflow_id.clone(),
            version: resolved.doc.version.clone(),
            status: "paused".to_string(),
            adl_path: adl_path.display().to_string(),
            execution_plan_hash: execution_plan_hash(&resolved.execution_plan)?,
            steering_history: steering_history.to_vec(),
            pause: pause_payload.clone(),
        };
        let pause_json =
            serde_json::to_vec_pretty(&pause_artifact).context("serialize pause_state.json")?;
        artifacts::atomic_write(&run_paths.pause_state_json(), &pause_json)?;
    }

    Ok(run_dir)
}

pub(crate) fn load_resume_state(
    path: &Path,
    resolved: &resolve::AdlResolved,
) -> Result<execute::ResumeState> {
    let raw = std::fs::read_to_string(path)
        .with_context(|| format!("failed to read resume state '{}'", path.display()))?;
    let artifact: RunStateArtifact = serde_json::from_str(&raw).with_context(|| {
        format!(
            "failed to parse resume state '{}' as run_state artifact",
            path.display()
        )
    })?;

    if artifact.schema_version != RUN_STATE_SCHEMA_VERSION {
        return Err(anyhow::anyhow!(
            "resume state schema_version mismatch: state='{}' expected='{}'",
            artifact.schema_version,
            RUN_STATE_SCHEMA_VERSION
        ));
    }

    if artifact.schema_version != RUN_STATE_SCHEMA_VERSION {
        return Err(anyhow::anyhow!(
            "resume state schema_version mismatch: state='{}' expected='{}'",
            artifact.schema_version,
            RUN_STATE_SCHEMA_VERSION
        ));
    }

    if artifact.status != "paused" {
        return Err(anyhow::anyhow!(
            "resume state must have status='paused' (found='{}' for run_id='{}' in '{}')",
            artifact.status,
            artifact.run_id,
            path.display()
        ));
    }
    if artifact.run_id != resolved.run_id {
        return Err(anyhow::anyhow!(
            "resume run_id mismatch: state='{}' current='{}'",
            artifact.run_id,
            resolved.run_id
        ));
    }
    if artifact.workflow_id != resolved.workflow_id {
        return Err(anyhow::anyhow!(
            "resume workflow_id mismatch for run_id='{}' in '{}': state='{}' current='{}'",
            artifact.run_id,
            path.display(),
            artifact.workflow_id,
            resolved.workflow_id
        ));
    }
    if artifact.version != resolved.doc.version {
        return Err(anyhow::anyhow!(
            "resume version mismatch for run_id='{}' in '{}': state='{}' current='{}'",
            artifact.run_id,
            path.display(),
            artifact.version,
            resolved.doc.version
        ));
    }
    let plan_hash = execution_plan_hash(&resolved.execution_plan)?;
    if artifact.execution_plan_hash != plan_hash {
        return Err(anyhow::anyhow!(
            "resume execution plan mismatch for run_id='{}' in '{}'; state plan != current plan (resume requires identical plan + ordering)",
            artifact.run_id,
            path.display()
        ));
    }
    let pause = artifact
        .pause
        .ok_or_else(|| anyhow::anyhow!("resume state missing pause payload"))?;

    let completed_step_ids = pause.completed_step_ids.into_iter().collect();
    Ok(execute::ResumeState {
        completed_step_ids,
        saved_state: pause.saved_state,
        completed_outputs: pause.completed_outputs,
        steering_history: artifact.steering_history,
    })
}

pub(crate) fn resume_state_path_for_run_id(run_id: &str) -> Result<PathBuf> {
    Ok(artifacts::RunArtifactPaths::for_run(run_id)?.pause_state_json())
}

pub(crate) fn load_pause_state_artifact(path: &Path) -> Result<PauseStateArtifact> {
    let raw = std::fs::read_to_string(path)
        .with_context(|| format!("failed to read pause state '{}'", path.display()))?;
    let artifact: PauseStateArtifact =
        serde_json::from_str(&raw).with_context(|| "failed to parse pause_state.json")?;
    Ok(artifact)
}

pub(crate) fn load_steering_patch(path: &Path) -> Result<(execute::SteeringPatch, String)> {
    let raw = std::fs::read(path)
        .with_context(|| format!("failed to read steering patch '{}'", path.display()))?;
    let fingerprint = stable_fingerprint_hex(&raw);
    let patch: execute::SteeringPatch =
        serde_json::from_slice(&raw).with_context(|| "failed to parse steering patch JSON")?;
    execute::validate_steering_patch(&patch)?;
    Ok((patch, fingerprint))
}

pub(crate) fn validate_pause_artifact_basic(
    artifact: &PauseStateArtifact,
    run_id: &str,
) -> Result<()> {
    if artifact.schema_version != PAUSE_STATE_SCHEMA_VERSION {
        return Err(anyhow::anyhow!(
            "pause state schema_version mismatch: state='{}' expected='{}'",
            artifact.schema_version,
            PAUSE_STATE_SCHEMA_VERSION
        ));
    }
    if artifact.status != "paused" {
        return Err(anyhow::anyhow!(
            "pause state must have status='paused' (found '{}')",
            artifact.status
        ));
    }
    if artifact.run_id != run_id {
        return Err(anyhow::anyhow!(
            "pause state run_id mismatch: state='{}' requested='{}'",
            artifact.run_id,
            run_id
        ));
    }
    Ok(())
}

pub(crate) fn validate_pause_artifact_for_resume(
    artifact: &PauseStateArtifact,
    run_id: &str,
    resolved: &resolve::AdlResolved,
) -> Result<()> {
    validate_pause_artifact_basic(artifact, run_id)?;
    if artifact.run_id != resolved.run_id {
        return Err(anyhow::anyhow!(
            "resume run_id mismatch: state='{}' current='{}'",
            artifact.run_id,
            resolved.run_id
        ));
    }
    if artifact.workflow_id != resolved.workflow_id {
        return Err(anyhow::anyhow!(
            "resume workflow_id mismatch: state='{}' current='{}'",
            artifact.workflow_id,
            resolved.workflow_id
        ));
    }
    if artifact.version != resolved.doc.version {
        return Err(anyhow::anyhow!(
            "resume version mismatch: state='{}' current='{}'",
            artifact.version,
            resolved.doc.version
        ));
    }
    let plan_hash = execution_plan_hash(&resolved.execution_plan)?;
    if artifact.execution_plan_hash != plan_hash {
        return Err(anyhow::anyhow!(
            "resume execution plan hash mismatch; resume requires identical plan and ordering"
        ));
    }
    Ok(())
}
