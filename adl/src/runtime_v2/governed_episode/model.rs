use super::*;

pub const RUNTIME_V2_CSM_RESOURCE_PRESSURE_SCHEMA: &str =
    "runtime_v2.csm_resource_pressure_fixture.v1";
pub const RUNTIME_V2_CSM_SCHEDULING_DECISION_SCHEMA: &str = "runtime_v2.csm_scheduling_decision.v1";
pub const RUNTIME_V2_CSM_FIRST_RUN_TRACE_EVENT_SCHEMA: &str =
    "runtime_v2.csm_first_run_trace_event.v1";
pub const RUNTIME_V2_CSM_CITIZEN_ACTION_FIXTURE_SCHEMA: &str =
    "runtime_v2.csm_citizen_action_fixture.v1";
pub const RUNTIME_V2_CSM_FREEDOM_GATE_DECISION_SCHEMA: &str =
    "runtime_v2.csm_freedom_gate_decision.v1";
pub const RUNTIME_V2_CSM_INVALID_ACTION_FIXTURE_SCHEMA: &str =
    "runtime_v2.csm_invalid_action_fixture.v1";
pub const RUNTIME_V2_CSM_WAKE_CONTINUITY_PROOF_SCHEMA: &str =
    "runtime_v2.csm_wake_continuity_proof.v1";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2CsmEpisodeCandidate {
    pub episode_id: String,
    pub citizen_id: String,
    pub identity_handle: String,
    pub requested_action: String,
    pub priority: u64,
    pub estimated_compute_tokens: u64,
    pub estimated_wall_clock_ms: u64,
    pub safety_class: String,
    pub admission_ref: String,
    pub can_execute_episodes: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2CsmResourcePressureFixture {
    pub schema_version: String,
    pub fixture_id: String,
    pub demo_id: String,
    pub manifold_id: String,
    pub artifact_path: String,
    pub boot_manifest_ref: String,
    pub citizen_roster_ref: String,
    pub pressure_kind: String,
    pub available_compute_tokens: u64,
    pub requested_compute_tokens: u64,
    pub available_wall_clock_ms: u64,
    pub requested_wall_clock_ms: u64,
    pub scheduler_policy: String,
    pub candidates: Vec<RuntimeV2CsmEpisodeCandidate>,
    pub claim_boundary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2CsmSchedulingDecision {
    pub schema_version: String,
    pub decision_id: String,
    pub demo_id: String,
    pub manifold_id: String,
    pub artifact_path: String,
    pub resource_pressure_ref: String,
    pub selected_episode_id: String,
    pub selected_citizen_id: String,
    pub scheduling_outcome: String,
    pub scheduler_reason: String,
    pub deferred_episode_ids: Vec<String>,
    pub trace_ref: String,
    pub required_invariants: Vec<String>,
    pub claim_boundary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2CsmFirstRunTraceEvent {
    pub schema_version: String,
    pub event_sequence: u64,
    pub event_id: String,
    pub manifold_id: String,
    pub episode_id: String,
    pub citizen_id: String,
    pub service_id: String,
    pub action: String,
    pub outcome: String,
    pub artifact_ref: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2CsmCitizenActionFixture {
    pub schema_version: String,
    pub action_id: String,
    pub demo_id: String,
    pub manifold_id: String,
    pub artifact_path: String,
    pub scheduling_decision_ref: String,
    pub episode_id: String,
    pub citizen_id: String,
    pub requested_action: String,
    pub action_payload_summary: String,
    pub resource_budget_tokens: u64,
    pub wall_clock_budget_ms: u64,
    pub safety_class: String,
    pub required_gate: String,
    pub claim_boundary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2CsmFreedomGateDecision {
    pub schema_version: String,
    pub decision_id: String,
    pub demo_id: String,
    pub manifold_id: String,
    pub artifact_path: String,
    pub citizen_action_ref: String,
    pub scheduling_decision_ref: String,
    pub episode_id: String,
    pub citizen_id: String,
    pub gate_id: String,
    pub gate_policy: String,
    pub decision_outcome: String,
    pub mediated_action: String,
    pub decision_reason: String,
    pub checked_invariants: Vec<String>,
    pub trace_ref: String,
    pub downstream_boundary: String,
    pub claim_boundary: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeV2CsmGovernedEpisodeArtifacts {
    pub resource_pressure: RuntimeV2CsmResourcePressureFixture,
    pub scheduling_decision: RuntimeV2CsmSchedulingDecision,
    pub first_run_trace: Vec<RuntimeV2CsmFirstRunTraceEvent>,
    pub first_run_trace_path: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeV2CsmFreedomGateMediationArtifacts {
    pub citizen_action: RuntimeV2CsmCitizenActionFixture,
    pub freedom_gate_decision: RuntimeV2CsmFreedomGateDecision,
    pub first_run_trace: Vec<RuntimeV2CsmFirstRunTraceEvent>,
    pub first_run_trace_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2CsmInvalidActionFixture {
    pub schema_version: String,
    pub invalid_action_id: String,
    pub demo_id: String,
    pub manifold_id: String,
    pub artifact_path: String,
    pub freedom_gate_decision_ref: String,
    pub episode_id: String,
    pub citizen_id: String,
    pub actor: String,
    pub attempted_action: String,
    pub attempted_state: String,
    pub invalid_reason: String,
    pub required_invariant: String,
    pub expected_result: String,
    pub claim_boundary: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeV2CsmInvalidActionRejectionArtifacts {
    pub invalid_action: RuntimeV2CsmInvalidActionFixture,
    pub violation_packet: RuntimeV2InvariantViolationArtifact,
    pub first_run_trace: Vec<RuntimeV2CsmFirstRunTraceEvent>,
    pub first_run_trace_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2CsmWakeContinuityCheck {
    pub invariant_id: String,
    pub status: String,
    pub checked_before_wake: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2CsmCitizenWakeContinuity {
    pub citizen_id: String,
    pub snapshot_record_ref: String,
    pub restored_record_ref: String,
    pub predecessor_snapshot_id: String,
    pub successor_trace_sequence: u64,
    pub continuity_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2CsmDuplicateActivationGuard {
    pub invariant_id: String,
    pub attempted_duplicate_active_heads: bool,
    pub duplicate_active_citizen_detected: bool,
    pub quarantine_required: bool,
    pub guard_result: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2CsmWakeContinuityProof {
    pub schema_version: String,
    pub proof_id: String,
    pub demo_id: String,
    pub manifold_id: String,
    pub artifact_path: String,
    pub snapshot_ref: String,
    pub rehydration_report_ref: String,
    pub source_trace_ref: String,
    pub wake_trace_sequence: u64,
    pub restored_active_citizens: Vec<String>,
    pub continuity_checks: Vec<RuntimeV2CsmWakeContinuityCheck>,
    pub citizen_continuity: Vec<RuntimeV2CsmCitizenWakeContinuity>,
    pub duplicate_activation_guard: RuntimeV2CsmDuplicateActivationGuard,
    pub proof_outcome: String,
    pub claim_boundary: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeV2CsmWakeContinuityArtifacts {
    pub snapshot_rehydration: RuntimeV2SnapshotAndRehydrationArtifacts,
    pub wake_continuity_proof: RuntimeV2CsmWakeContinuityProof,
    pub first_run_trace: Vec<RuntimeV2CsmFirstRunTraceEvent>,
    pub first_run_trace_path: String,
}
