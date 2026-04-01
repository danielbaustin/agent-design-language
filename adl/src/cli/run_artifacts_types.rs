use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::Path;

use ::adl::execute;

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
pub(crate) const REFRAMING_VERSION: u32 = 1;
pub(crate) const FREEDOM_GATE_VERSION: u32 = 1;
pub(crate) const MEMORY_READ_VERSION: u32 = 1;
pub(crate) const MEMORY_WRITE_VERSION: u32 = 1;
pub(crate) const CONTROL_PATH_MEMORY_VERSION: u32 = 1;
pub(crate) const CONTROL_PATH_FINAL_RESULT_VERSION: u32 = 1;
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

fn normalize_pause_adl_ref(path: &Path) -> String {
    let mut parts = Vec::new();
    for component in path.components() {
        match component {
            std::path::Component::CurDir => {}
            std::path::Component::Normal(part) => parts.push(part.to_string_lossy().to_string()),
            std::path::Component::ParentDir => parts.push("..".to_string()),
            std::path::Component::RootDir | std::path::Component::Prefix(_) => {}
        }
    }
    if parts.is_empty() {
        "<unknown>".to_string()
    } else {
        parts.join("/")
    }
}

pub(crate) fn sanitize_pause_adl_path(adl_path: &Path) -> String {
    if !adl_path.is_absolute() {
        return normalize_pause_adl_ref(adl_path);
    }
    if let Ok(cwd) = std::env::current_dir() {
        if let Ok(rel) = adl_path.strip_prefix(&cwd) {
            return normalize_pause_adl_ref(rel);
        }
    }
    if let Some(file_name) = adl_path.file_name() {
        return format!("external:/{}", file_name.to_string_lossy());
    }
    "external:/<unknown>".to_string()
}

#[cfg(test)]
mod tests {
    use super::sanitize_pause_adl_path;
    use std::path::Path;

    #[test]
    fn sanitize_pause_adl_path_normalizes_relative_paths() {
        let rel = Path::new("./fixtures/../demo/example.adl");
        assert_eq!(sanitize_pause_adl_path(rel), "fixtures/../demo/example.adl");
    }

    #[test]
    fn sanitize_pause_adl_path_relativizes_absolute_paths_inside_cwd() {
        let cwd = std::env::current_dir().expect("cwd");
        let path = cwd.join("fixtures").join("resume").join("example.adl");
        assert_eq!(
            sanitize_pause_adl_path(&path),
            "fixtures/resume/example.adl"
        );
    }

    #[test]
    fn sanitize_pause_adl_path_redacts_external_absolute_paths_to_filename() {
        let path = Path::new("/tmp/external/example.adl");
        assert_eq!(sanitize_pause_adl_path(path), "external:/example.adl");
    }

    #[test]
    fn sanitize_pause_adl_path_handles_root_without_filename() {
        let path = Path::new("/");
        assert_eq!(sanitize_pause_adl_path(path), "external:/<unknown>");
    }
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
pub(crate) struct ReframingArtifact {
    pub(crate) reframing_version: u32,
    pub(crate) run_id: String,
    pub(crate) generated_from: AeeDecisionGeneratedFrom,
    pub(crate) selected_candidate_id: String,
    pub(crate) selected_path: String,
    pub(crate) frame_adequacy_score: u32,
    pub(crate) reframing_trigger: String,
    pub(crate) reframing_reason: String,
    pub(crate) prior_frame: String,
    pub(crate) new_frame: String,
    pub(crate) reexecution_choice: String,
    pub(crate) post_reframe_state: String,
    pub(crate) deterministic_reframing_rule: String,
}

pub(crate) type ReframingControlState = execute::ReframingControlState;

pub(crate) type FreedomGateState = execute::FreedomGateState;
pub(crate) type FreedomGateInputState = execute::FreedomGateInputState;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct FreedomGateArtifact {
    pub(crate) freedom_gate_version: u32,
    pub(crate) run_id: String,
    pub(crate) generated_from: AeeDecisionGeneratedFrom,
    pub(crate) input: FreedomGateInputState,
    pub(crate) gate_decision: String,
    pub(crate) reason_code: String,
    pub(crate) decision_reason: String,
    pub(crate) selected_action_or_none: Option<String>,
    pub(crate) commitment_blocked: bool,
    pub(crate) deterministic_gate_rule: String,
}

pub(crate) type MemoryReadState = execute::MemoryReadState;
pub(crate) type MemoryQueryState = execute::MemoryQueryState;
pub(crate) type MemoryReadEntry = execute::MemoryReadEntry;
pub(crate) type MemoryWriteState = execute::MemoryWriteState;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct MemoryReadArtifact {
    pub(crate) memory_read_version: u32,
    pub(crate) run_id: String,
    pub(crate) generated_from: AeeDecisionGeneratedFrom,
    pub(crate) query: MemoryQueryState,
    pub(crate) read_count: u32,
    pub(crate) entries: Vec<MemoryReadEntry>,
    pub(crate) retrieval_order: String,
    pub(crate) influence_summary: String,
    pub(crate) influenced_stage: String,
    pub(crate) deterministic_read_rule: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct MemoryWriteArtifact {
    pub(crate) memory_write_version: u32,
    pub(crate) run_id: String,
    pub(crate) generated_from: AeeDecisionGeneratedFrom,
    pub(crate) entry_id: String,
    pub(crate) content: String,
    pub(crate) tags: Vec<String>,
    pub(crate) logical_timestamp: String,
    pub(crate) write_reason: String,
    pub(crate) source: String,
    pub(crate) deterministic_write_rule: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ControlPathMemoryArtifact {
    pub(crate) control_path_memory_version: u32,
    pub(crate) run_id: String,
    pub(crate) generated_from: AeeDecisionGeneratedFrom,
    pub(crate) read: MemoryReadArtifact,
    pub(crate) write: MemoryWriteArtifact,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ControlPathFinalResultArtifact {
    pub(crate) control_path_final_result_version: u32,
    pub(crate) run_id: String,
    pub(crate) route_selected: String,
    pub(crate) selected_candidate: String,
    pub(crate) termination_reason: String,
    pub(crate) gate_decision: String,
    pub(crate) final_result: String,
    pub(crate) commitment_blocked: bool,
    pub(crate) next_control_action: String,
    pub(crate) stage_order: Vec<String>,
}

pub(crate) struct ControlPathSummaryContext<'a> {
    pub(crate) signals: &'a CognitiveSignalsArtifact,
    pub(crate) agency: &'a AgencySelectionArtifact,
    pub(crate) arbitration: &'a CognitiveArbitrationArtifact,
    pub(crate) execution: &'a BoundedExecutionArtifact,
    pub(crate) evaluation: &'a EvaluationSignalsArtifact,
    pub(crate) reframing: &'a ReframingArtifact,
    pub(crate) memory: &'a ControlPathMemoryArtifact,
    pub(crate) freedom_gate: &'a FreedomGateArtifact,
    pub(crate) final_result: &'a ControlPathFinalResultArtifact,
}

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
