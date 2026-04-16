use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freedom_gate;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RuntimeControlState {
    pub signals: CognitiveSignalsState,
    pub arbitration: CognitiveArbitrationState,
    pub fast_slow: FastSlowPathState,
    pub agency: AgencySelectionState,
    pub bounded_execution: BoundedExecutionState,
    pub evaluation: EvaluationControlState,
    pub reframing: ReframingControlState,
    pub freedom_gate: FreedomGateState,
    pub memory: MemoryParticipationState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DominantInstinct {
    Integrity,
    Completion,
    Curiosity,
    Coherence,
}

impl DominantInstinct {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Integrity => "integrity",
            Self::Completion => "completion",
            Self::Curiosity => "curiosity",
            Self::Coherence => "coherence",
        }
    }
}

impl fmt::Display for DominantInstinct {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Route {
    Fast,
    Hybrid,
    Slow,
}

impl Route {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Fast => "fast",
            Self::Hybrid => "hybrid",
            Self::Slow => "slow",
        }
    }
}

impl fmt::Display for Route {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SelectedPath {
    FastPath,
    SlowPath,
}

impl SelectedPath {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FastPath => "fast_path",
            Self::SlowPath => "slow_path",
        }
    }
}

impl fmt::Display for SelectedPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CognitiveSignalsState {
    pub dominant_instinct: DominantInstinct,
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
    pub route_selected: Route,
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
    pub selected_path: SelectedPath,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AgencySelectionDecisionTemplate {
    pub selection_mode: &'static str,
    pub candidate_id: &'static str,
    pub candidate_kind: &'static str,
    pub candidate_action: &'static str,
    pub candidate_reason: &'static str,
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReframingControlState {
    pub frame_adequacy_score: u32,
    pub reframing_trigger: String,
    pub reframing_reason: String,
    pub prior_frame: String,
    pub new_frame: String,
    pub reexecution_choice: String,
    pub post_reframe_state: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FreedomGateState {
    pub input: FreedomGateInputState,
    pub gate_decision: String,
    pub reason_code: String,
    pub decision_reason: String,
    pub selected_action_or_none: Option<String>,
    pub commitment_blocked: bool,
    pub judgment_boundary: String,
    pub required_follow_up: String,
    pub decision_record_kind: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FreedomGateInputState {
    pub candidate_id: String,
    pub candidate_action: String,
    pub candidate_rationale: String,
    pub risk_class: String,
    pub policy_context: FreedomGatePolicyContextState,
    pub evaluation_signals: FreedomGateEvaluationSignalsState,
    pub consequence_context: FreedomGateConsequenceContextState,
    pub frame_state: String,
}

fn parse_route(value: &str) -> Route {
    match value {
        "fast" => Route::Fast,
        "hybrid" => Route::Hybrid,
        "slow" => Route::Slow,
        other => panic!("unexpected route_selected value: {other}"),
    }
}

impl From<freedom_gate::FreedomGatePolicyContext> for FreedomGatePolicyContextState {
    fn from(value: freedom_gate::FreedomGatePolicyContext) -> Self {
        Self {
            route_selected: parse_route(&value.route_selected),
            selected_candidate_kind: value.selected_candidate_kind,
            requires_review: value.requires_review,
            policy_blocked: value.policy_blocked,
        }
    }
}

impl From<freedom_gate::FreedomGateEvaluationSignals> for FreedomGateEvaluationSignalsState {
    fn from(value: freedom_gate::FreedomGateEvaluationSignals) -> Self {
        Self {
            progress_signal: value.progress_signal,
            contradiction_signal: value.contradiction_signal,
            failure_signal: value.failure_signal,
            termination_reason: value.termination_reason,
        }
    }
}

impl From<freedom_gate::FreedomGateConsequenceContext> for FreedomGateConsequenceContextState {
    fn from(value: freedom_gate::FreedomGateConsequenceContext) -> Self {
        Self {
            impact_scope: value.impact_scope,
            recovery_cost: value.recovery_cost,
            operator_visibility: value.operator_visibility,
            escalation_available: value.escalation_available,
        }
    }
}

impl From<freedom_gate::FreedomGateInput> for FreedomGateInputState {
    fn from(value: freedom_gate::FreedomGateInput) -> Self {
        Self {
            candidate_id: value.candidate_id,
            candidate_action: value.candidate_action,
            candidate_rationale: value.candidate_rationale,
            risk_class: value.risk_class,
            policy_context: value.policy_context.into(),
            evaluation_signals: value.evaluation_signals.into(),
            consequence_context: value.consequence_context.into(),
            frame_state: value.frame_state,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FreedomGatePolicyContextState {
    pub route_selected: Route,
    pub selected_candidate_kind: String,
    pub requires_review: bool,
    pub policy_blocked: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FreedomGateEvaluationSignalsState {
    pub progress_signal: String,
    pub contradiction_signal: String,
    pub failure_signal: String,
    pub termination_reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FreedomGateConsequenceContextState {
    pub impact_scope: String,
    pub recovery_cost: String,
    pub operator_visibility: String,
    pub escalation_available: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MemoryParticipationState {
    pub read: MemoryReadState,
    pub write: MemoryWriteState,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MemoryReadState {
    pub query: MemoryQueryState,
    pub entries: Vec<MemoryReadEntry>,
    pub retrieval_order: String,
    pub influence_summary: String,
    pub influenced_stage: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MemoryQueryState {
    pub workflow_id: String,
    pub status_filter: String,
    pub limit: u32,
    pub source: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MemoryReadEntry {
    pub memory_entry_id: String,
    pub run_id: String,
    pub workflow_id: String,
    pub summary: String,
    pub tags: Vec<String>,
    pub source: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MemoryWriteState {
    pub entry_id: String,
    pub content: String,
    pub tags: Vec<String>,
    pub logical_timestamp: String,
    pub write_reason: String,
    pub source: String,
}
