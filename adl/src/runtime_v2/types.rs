use serde::{Deserialize, Serialize};

pub const RUNTIME_V2_MANIFOLD_SCHEMA: &str = "runtime_v2.manifold.v1";
pub const RUNTIME_V2_KERNEL_SERVICE_REGISTRY_SCHEMA: &str = "runtime_v2.kernel.service_registry.v1";
pub const RUNTIME_V2_KERNEL_SERVICE_STATE_SCHEMA: &str = "runtime_v2.kernel.service_state.v1";
pub const RUNTIME_V2_KERNEL_LOOP_EVENT_SCHEMA: &str = "runtime_v2.kernel.service_loop_event.v1";
pub const RUNTIME_V2_PROVISIONAL_CITIZEN_SCHEMA: &str = "runtime_v2.provisional_citizen.v1";
pub const RUNTIME_V2_CITIZEN_REGISTRY_INDEX_SCHEMA: &str = "runtime_v2.citizen_registry_index.v1";
pub const RUNTIME_V2_SNAPSHOT_MANIFEST_SCHEMA: &str = "runtime_v2.snapshot_manifest.v1";
pub const RUNTIME_V2_REHYDRATION_REPORT_SCHEMA: &str = "runtime_v2.rehydration_report.v1";
pub const RUNTIME_V2_INVARIANT_VIOLATION_SCHEMA: &str = "runtime_v2.invariant_violation.v1";
pub const RUNTIME_V2_OPERATOR_CONTROL_REPORT_SCHEMA: &str = "runtime_v2.operator_control_report.v1";
pub const RUNTIME_V2_SECURITY_BOUNDARY_PROOF_SCHEMA: &str = "runtime_v2.security_boundary_proof.v1";
pub const DEFAULT_MANIFOLD_ARTIFACT_PATH: &str = "runtime_v2/manifold.json";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ManifoldClockAnchor {
    pub anchor_id: String,
    pub clock_kind: String,
    pub monotonic_tick: u64,
    pub observed_at_utc: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CitizenRegistryRefs {
    pub registry_root: String,
    pub active_index: String,
    pub pending_index: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct KernelServiceRefs {
    pub registry_path: String,
    pub service_loop_path: String,
    pub service_state_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TraceRootRef {
    pub trace_root: String,
    pub event_log_path: String,
    pub next_event_sequence: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SnapshotRootRef {
    pub snapshot_root: String,
    pub latest_snapshot_id: Option<String>,
    pub rehydration_report_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ResourceLedgerRef {
    pub ledger_path: String,
    pub accounting_mode: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InvariantPolicyRefs {
    pub policy_path: String,
    pub enforcement_mode: String,
    pub blocking_invariants: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2ManifoldReviewSurface {
    pub required_artifacts: Vec<String>,
    pub proof_hook_command: String,
    pub proof_hook_output_path: String,
    pub downstream_boundaries: Vec<String>,
    pub non_goals: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2ManifoldRoot {
    pub schema_version: String,
    pub manifold_id: String,
    pub lifecycle_state: String,
    pub artifact_path: String,
    pub clock_anchor: ManifoldClockAnchor,
    pub citizen_registry_refs: CitizenRegistryRefs,
    pub kernel_service_refs: KernelServiceRefs,
    pub trace_root: TraceRootRef,
    pub snapshot_root: SnapshotRootRef,
    pub resource_ledger: ResourceLedgerRef,
    pub invariant_policy_refs: InvariantPolicyRefs,
    pub review_surface: RuntimeV2ManifoldReviewSurface,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2KernelServiceRegistration {
    pub service_id: String,
    pub service_kind: String,
    pub lifecycle_state: String,
    pub activation_order: u64,
    pub owns_artifact_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2KernelServiceRegistry {
    pub schema_version: String,
    pub manifold_id: String,
    pub registry_path: String,
    pub services: Vec<RuntimeV2KernelServiceRegistration>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2KernelServiceStatus {
    pub service_id: String,
    pub lifecycle_state: String,
    pub last_event_sequence: u64,
    pub blocked_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2KernelServiceState {
    pub schema_version: String,
    pub manifold_id: String,
    pub service_state_path: String,
    pub loop_status: String,
    pub completed_through_event_sequence: u64,
    pub services: Vec<RuntimeV2KernelServiceStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2KernelLoopEvent {
    pub schema_version: String,
    pub event_sequence: u64,
    pub manifold_id: String,
    pub service_id: String,
    pub action: String,
    pub outcome: String,
    pub artifact_ref: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeV2KernelLoopArtifacts {
    pub registry: RuntimeV2KernelServiceRegistry,
    pub state: RuntimeV2KernelServiceState,
    pub events: Vec<RuntimeV2KernelLoopEvent>,
    pub service_loop_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2CitizenMemoryIdentityRefs {
    pub memory_root_ref: String,
    pub identity_profile_ref: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2CitizenPolicyBoundaryRefs {
    pub policy_ref: String,
    pub admission_trace_ref: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2ProvisionalCitizenRecord {
    pub schema_version: String,
    pub citizen_id: String,
    pub display_name: String,
    pub provisional_status: String,
    pub lifecycle_state: String,
    pub manifold_id: String,
    pub record_path: String,
    pub created_at_utc: String,
    pub last_wake_at_utc: Option<String>,
    pub memory_identity_refs: RuntimeV2CitizenMemoryIdentityRefs,
    pub policy_boundary_refs: RuntimeV2CitizenPolicyBoundaryRefs,
    pub rehydration_validation_ref: Option<String>,
    pub termination_event_ref: Option<String>,
    pub resources_released: bool,
    pub can_execute_episodes: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2CitizenRegistryEntry {
    pub citizen_id: String,
    pub lifecycle_state: String,
    pub record_path: String,
    pub can_execute_episodes: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2CitizenRegistryIndex {
    pub schema_version: String,
    pub manifold_id: String,
    pub registry_root: String,
    pub index_kind: String,
    pub index_path: String,
    pub citizens: Vec<RuntimeV2CitizenRegistryEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeV2CitizenLifecycleArtifacts {
    pub records: Vec<RuntimeV2ProvisionalCitizenRecord>,
    pub active_index: RuntimeV2CitizenRegistryIndex,
    pub pending_index: RuntimeV2CitizenRegistryIndex,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2SnapshotInvariantStatus {
    pub invariant_id: String,
    pub status: String,
    pub checked_before_snapshot: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2SnapshotManifest {
    pub schema_version: String,
    pub snapshot_id: String,
    pub manifold_id: String,
    pub snapshot_path: String,
    pub created_at_utc: String,
    pub manifold_state: RuntimeV2ManifoldRoot,
    pub citizen_records: Vec<RuntimeV2ProvisionalCitizenRecord>,
    pub active_index: RuntimeV2CitizenRegistryIndex,
    pub pending_index: RuntimeV2CitizenRegistryIndex,
    pub kernel_service_state: RuntimeV2KernelServiceState,
    pub last_trace_cursor: u64,
    pub invariant_status: Vec<RuntimeV2SnapshotInvariantStatus>,
    pub structural_checksum: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2RehydrationReport {
    pub schema_version: String,
    pub snapshot_id: String,
    pub manifold_id: String,
    pub report_path: String,
    pub restored_manifold_id: String,
    pub restored_lifecycle_state: String,
    pub trace_resume_sequence: u64,
    pub invariant_checks_ran_before_resume: bool,
    pub duplicate_active_citizen_detected: bool,
    pub restored_active_citizens: Vec<String>,
    pub wake_allowed: bool,
    pub wake_refused_reason: Option<String>,
    pub snapshot_checksum: String,
    pub rehydrated_at_utc: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeV2SnapshotAndRehydrationArtifacts {
    pub snapshot: RuntimeV2SnapshotManifest,
    pub rehydration_report: RuntimeV2RehydrationReport,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2InvariantViolationAttempt {
    pub actor: String,
    pub attempted_action: String,
    pub attempted_state: String,
    pub source_artifact_ref: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2InvariantViolationEvaluatedRef {
    pub ref_kind: String,
    pub artifact_ref: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2InvariantViolationResult {
    pub resulting_state: String,
    pub blocked_before_commit: bool,
    pub recovery_action: String,
    pub trace_ref: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2InvariantViolationArtifact {
    pub schema_version: String,
    pub violation_id: String,
    pub manifold_id: String,
    pub artifact_path: String,
    pub detected_at_utc: String,
    pub severity: String,
    pub invariant_id: String,
    pub invariant_owner_service_id: String,
    pub policy_enforcement_mode: String,
    pub attempted_transition: RuntimeV2InvariantViolationAttempt,
    pub evaluated_refs: Vec<RuntimeV2InvariantViolationEvaluatedRef>,
    pub affected_citizens: Vec<String>,
    pub refusal_reason: String,
    pub source_error: String,
    pub result: RuntimeV2InvariantViolationResult,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2OperatorControlState {
    pub manifold_lifecycle_state: String,
    pub kernel_loop_status: String,
    pub active_citizen_count: usize,
    pub pending_citizen_count: usize,
    pub latest_snapshot_id: Option<String>,
    pub completed_through_event_sequence: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2OperatorCommandReport {
    pub command: String,
    pub requested_by: String,
    pub affected_service: String,
    pub pre_state: RuntimeV2OperatorControlState,
    pub post_state: RuntimeV2OperatorControlState,
    pub outcome: String,
    pub trace_event_ref: String,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2OperatorControlReport {
    pub schema_version: String,
    pub report_id: String,
    pub manifold_id: String,
    pub artifact_path: String,
    pub generated_at_utc: String,
    pub control_interface_service_id: String,
    pub commands: Vec<RuntimeV2OperatorCommandReport>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2SecurityBoundaryAttempt {
    pub actor: String,
    pub attempted_action: String,
    pub requested_state: String,
    pub source_command_ref: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2SecurityBoundaryEvaluatedRule {
    pub rule_id: String,
    pub rule_kind: String,
    pub source_ref: String,
    pub decision: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2SecurityBoundaryResult {
    pub allowed: bool,
    pub refusal_reason: String,
    pub resulting_state: RuntimeV2OperatorControlState,
    pub trace_ref: String,
    pub recovery_action: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2SecurityBoundaryProofPacket {
    pub schema_version: String,
    pub proof_id: String,
    pub manifold_id: String,
    pub artifact_path: String,
    pub generated_at_utc: String,
    pub boundary_service_id: String,
    pub attempt: RuntimeV2SecurityBoundaryAttempt,
    pub evaluated_rules: Vec<RuntimeV2SecurityBoundaryEvaluatedRule>,
    pub related_artifacts: Vec<String>,
    pub result: RuntimeV2SecurityBoundaryResult,
}
