//! Runtime-v2 agent lifecycle state contract, transition matrix, and fixtures.
//!
//! Defines the first bounded runtime-facing lifecycle model for inhabited
//! runtime work. The surface is intentionally artifact-oriented and reviewable:
//! state semantics, ACIP eligibility, and failure/custody fixtures are explicit
//! rather than implied by prose.

use std::path::Path;

use super::*;

pub const RUNTIME_V2_AGENT_LIFECYCLE_STATE_CONTRACT_SCHEMA: &str =
    "runtime_v2.agent_lifecycle_state_contract.v1";
pub const RUNTIME_V2_AGENT_LIFECYCLE_TRANSITION_MATRIX_SCHEMA: &str =
    "runtime_v2.agent_lifecycle_transition_matrix.v1";
pub const RUNTIME_V2_AGENT_LIFECYCLE_FIXTURES_SCHEMA: &str =
    "runtime_v2.agent_lifecycle_fixtures.v1";

pub const RUNTIME_V2_AGENT_LIFECYCLE_STATE_CONTRACT_PATH: &str =
    "runtime_v2/agent_lifecycle/state_contract.json";
pub const RUNTIME_V2_AGENT_LIFECYCLE_TRANSITION_MATRIX_PATH: &str =
    "runtime_v2/agent_lifecycle/transition_matrix.json";
pub const RUNTIME_V2_AGENT_LIFECYCLE_FIXTURES_PATH: &str =
    "runtime_v2/agent_lifecycle/proof_fixtures.json";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2AgentLifecycleCapabilityFlags {
    pub freedom_gate_agency_available: bool,
    pub aee_execution_available: bool,
    pub memory_read_allowed: bool,
    pub memory_write_allowed: bool,
    pub chronosense_continuity: String,
    pub acip_receipt_policy: String,
    pub acip_invocation_policy: String,
    pub observatory_visibility: String,
    pub external_commitment_allowed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2AgentLifecycleStateSpec {
    pub state: String,
    pub state_class: String,
    pub runtime_binding_state: String,
    pub description: String,
    pub capabilities: RuntimeV2AgentLifecycleCapabilityFlags,
    pub required_evidence_refs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2AgentLifecycleStateContract {
    pub schema_version: String,
    pub demo_id: String,
    pub wp_id: String,
    pub artifact_path: String,
    pub source_feature_doc: String,
    pub states: Vec<RuntimeV2AgentLifecycleStateSpec>,
    pub validation_command: String,
    pub claim_boundary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2AgentLifecycleTransitionRule {
    pub transition_id: String,
    pub from_state: String,
    pub to_state: String,
    pub transition_kind: String,
    pub allowed: bool,
    pub required_authority: String,
    pub trace_event_kind: String,
    pub failure_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2AgentLifecycleTransitionMatrix {
    pub schema_version: String,
    pub demo_id: String,
    pub wp_id: String,
    pub artifact_path: String,
    pub state_contract_ref: String,
    pub transitions: Vec<RuntimeV2AgentLifecycleTransitionRule>,
    pub validation_command: String,
    pub claim_boundary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2AgentLifecycleProofFixture {
    pub fixture_id: String,
    pub fixture_kind: String,
    pub initial_state: String,
    pub triggering_message_kind: String,
    pub expected_receipt_policy: String,
    pub expected_invocation_result: String,
    pub continuity_expectation: String,
    pub expected_trace_event_kind: String,
    pub evidence_refs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2AgentLifecycleProofFixtures {
    pub schema_version: String,
    pub demo_id: String,
    pub wp_id: String,
    pub artifact_path: String,
    pub state_contract_ref: String,
    pub transition_matrix_ref: String,
    pub fixtures: Vec<RuntimeV2AgentLifecycleProofFixture>,
    pub validation_command: String,
    pub claim_boundary: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeV2AgentLifecycleArtifacts {
    pub state_contract: RuntimeV2AgentLifecycleStateContract,
    pub transition_matrix: RuntimeV2AgentLifecycleTransitionMatrix,
    pub proof_fixtures: RuntimeV2AgentLifecycleProofFixtures,
}

pub fn runtime_v2_agent_lifecycle_state_model() -> Result<RuntimeV2AgentLifecycleArtifacts> {
    RuntimeV2AgentLifecycleArtifacts::prototype()
}

impl RuntimeV2AgentLifecycleArtifacts {
    pub fn prototype() -> Result<Self> {
        let state_contract = RuntimeV2AgentLifecycleStateContract::prototype()?;
        let transition_matrix =
            RuntimeV2AgentLifecycleTransitionMatrix::prototype(&state_contract)?;
        let proof_fixtures =
            RuntimeV2AgentLifecycleProofFixtures::prototype(&state_contract, &transition_matrix)?;
        let artifacts = Self {
            state_contract,
            transition_matrix,
            proof_fixtures,
        };
        artifacts.validate()?;
        Ok(artifacts)
    }

    pub fn validate(&self) -> Result<()> {
        self.state_contract.validate()?;
        self.transition_matrix
            .validate_against(&self.state_contract)?;
        self.proof_fixtures
            .validate_against(&self.state_contract, &self.transition_matrix)
    }

    pub fn state_contract_pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.validate()?;
        serde_json::to_vec_pretty(&self.state_contract)
            .context("serialize Runtime v2 agent lifecycle state contract")
    }

    pub fn transition_matrix_pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.validate()?;
        serde_json::to_vec_pretty(&self.transition_matrix)
            .context("serialize Runtime v2 agent lifecycle transition matrix")
    }

    pub fn fixtures_pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.validate()?;
        serde_json::to_vec_pretty(&self.proof_fixtures)
            .context("serialize Runtime v2 agent lifecycle proof fixtures")
    }

    pub fn write_to_root(&self, root: impl AsRef<Path>) -> Result<()> {
        let root = root.as_ref();
        write_relative(
            root,
            &self.state_contract.artifact_path,
            self.state_contract_pretty_json_bytes()?,
        )?;
        write_relative(
            root,
            &self.transition_matrix.artifact_path,
            self.transition_matrix_pretty_json_bytes()?,
        )?;
        write_relative(
            root,
            &self.proof_fixtures.artifact_path,
            self.fixtures_pretty_json_bytes()?,
        )?;
        Ok(())
    }
}

impl RuntimeV2AgentLifecycleStateContract {
    fn prototype() -> Result<Self> {
        let contract = Self {
            schema_version: RUNTIME_V2_AGENT_LIFECYCLE_STATE_CONTRACT_SCHEMA.to_string(),
            demo_id: "D3".to_string(),
            wp_id: "WP-03".to_string(),
            artifact_path: RUNTIME_V2_AGENT_LIFECYCLE_STATE_CONTRACT_PATH.to_string(),
            source_feature_doc: "docs/milestones/v0.91.1/features/AGENT_LIFECYCLE_STATE_MODEL.md"
                .to_string(),
            states: expected_state_specs(),
            validation_command:
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_agent_lifecycle_state -- --nocapture"
                    .to_string(),
            claim_boundary: "This contract proves one bounded runtime-facing lifecycle state model with explicit ACIP receipt/invocation limits, observatory-safe visibility, and failure/custody fixtures. It binds to existing lowercase runtime states where those bindings already exist and uses explicit overlay-only states where the older manifold/citizen validators do not yet carry the full reviewed vocabulary. It does not prove birthday semantics, consciousness, cross-polis transport, or external-transport readiness.".to_string(),
        };
        contract.validate()?;
        Ok(contract)
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_AGENT_LIFECYCLE_STATE_CONTRACT_SCHEMA {
            return Err(anyhow!(
                "unsupported agent_lifecycle_state_contract.schema_version '{}'",
                self.schema_version
            ));
        }
        validate_demo_id(&self.demo_id, "agent_lifecycle_state_contract.demo_id")?;
        if self.wp_id != "WP-03" {
            return Err(anyhow!(
                "agent_lifecycle_state_contract.wp_id must remain WP-03"
            ));
        }
        validate_relative_path(
            &self.artifact_path,
            "agent_lifecycle_state_contract.artifact_path",
        )?;
        validate_relative_path(
            &self.source_feature_doc,
            "agent_lifecycle_state_contract.source_feature_doc",
        )?;
        validate_nonempty_text(
            &self.validation_command,
            "agent_lifecycle_state_contract.validation_command",
        )?;
        validate_nonempty_text(
            &self.claim_boundary,
            "agent_lifecycle_state_contract.claim_boundary",
        )?;
        if self.states != expected_state_specs() {
            return Err(anyhow!(
                "agent_lifecycle_state_contract.states must preserve the reviewed state order and capability matrix"
            ));
        }
        for state in &self.states {
            validate_runtime_binding_state(
                &state.runtime_binding_state,
                "agent_lifecycle_state_contract.runtime_binding_state",
            )?;
        }
        Ok(())
    }
}

impl RuntimeV2AgentLifecycleTransitionMatrix {
    fn prototype(state_contract: &RuntimeV2AgentLifecycleStateContract) -> Result<Self> {
        state_contract.validate()?;
        let matrix = Self {
            schema_version: RUNTIME_V2_AGENT_LIFECYCLE_TRANSITION_MATRIX_SCHEMA.to_string(),
            demo_id: state_contract.demo_id.clone(),
            wp_id: state_contract.wp_id.clone(),
            artifact_path: RUNTIME_V2_AGENT_LIFECYCLE_TRANSITION_MATRIX_PATH.to_string(),
            state_contract_ref: state_contract.artifact_path.clone(),
            transitions: expected_transition_rules(),
            validation_command: state_contract.validation_command.clone(),
            claim_boundary: "The transition matrix proves which lifecycle moves are allowed, denied, or failure/custody-only, and records that ACIP cannot invoke agency outside an allowed active path.".to_string(),
        };
        matrix.validate_against(state_contract)?;
        Ok(matrix)
    }

    pub fn validate_against(
        &self,
        state_contract: &RuntimeV2AgentLifecycleStateContract,
    ) -> Result<()> {
        if self.schema_version != RUNTIME_V2_AGENT_LIFECYCLE_TRANSITION_MATRIX_SCHEMA {
            return Err(anyhow!(
                "unsupported agent_lifecycle_transition_matrix.schema_version '{}'",
                self.schema_version
            ));
        }
        if self.state_contract_ref != state_contract.artifact_path {
            return Err(anyhow!(
                "agent_lifecycle_transition_matrix.state_contract_ref must match the lifecycle state contract artifact"
            ));
        }
        validate_relative_path(
            &self.artifact_path,
            "agent_lifecycle_transition_matrix.artifact_path",
        )?;
        validate_nonempty_text(
            &self.validation_command,
            "agent_lifecycle_transition_matrix.validation_command",
        )?;
        validate_nonempty_text(
            &self.claim_boundary,
            "agent_lifecycle_transition_matrix.claim_boundary",
        )?;
        if self.transitions != expected_transition_rules() {
            return Err(anyhow!(
                "agent_lifecycle_transition_matrix.transitions must preserve the reviewed runtime lifecycle transitions"
            ));
        }
        Ok(())
    }
}

impl RuntimeV2AgentLifecycleProofFixtures {
    fn prototype(
        state_contract: &RuntimeV2AgentLifecycleStateContract,
        transition_matrix: &RuntimeV2AgentLifecycleTransitionMatrix,
    ) -> Result<Self> {
        let fixtures = Self {
            schema_version: RUNTIME_V2_AGENT_LIFECYCLE_FIXTURES_SCHEMA.to_string(),
            demo_id: state_contract.demo_id.clone(),
            wp_id: state_contract.wp_id.clone(),
            artifact_path: RUNTIME_V2_AGENT_LIFECYCLE_FIXTURES_PATH.to_string(),
            state_contract_ref: state_contract.artifact_path.clone(),
            transition_matrix_ref: transition_matrix.artifact_path.clone(),
            fixtures: expected_proof_fixtures(),
            validation_command: state_contract.validation_command.clone(),
            claim_boundary: "These fixtures prove active governed invocation, quiescent queue-or-wake behavior, simulation no-external-action, dormant continuity preservation, suspended wake-only receipt, in-transit no-agency, forced-suspension failure semantics, quarantine/recovery-only custody behavior, rejected non-activation, and orphaned custody-recovery-only behavior.".to_string(),
        };
        fixtures.validate_against(state_contract, transition_matrix)?;
        Ok(fixtures)
    }

    pub fn validate_against(
        &self,
        state_contract: &RuntimeV2AgentLifecycleStateContract,
        transition_matrix: &RuntimeV2AgentLifecycleTransitionMatrix,
    ) -> Result<()> {
        if self.schema_version != RUNTIME_V2_AGENT_LIFECYCLE_FIXTURES_SCHEMA {
            return Err(anyhow!(
                "unsupported agent_lifecycle_fixtures.schema_version '{}'",
                self.schema_version
            ));
        }
        if self.state_contract_ref != state_contract.artifact_path {
            return Err(anyhow!(
                "agent_lifecycle_fixtures.state_contract_ref must match the lifecycle state contract artifact"
            ));
        }
        if self.transition_matrix_ref != transition_matrix.artifact_path {
            return Err(anyhow!(
                "agent_lifecycle_fixtures.transition_matrix_ref must match the lifecycle transition matrix artifact"
            ));
        }
        validate_relative_path(
            &self.artifact_path,
            "agent_lifecycle_fixtures.artifact_path",
        )?;
        validate_nonempty_text(
            &self.validation_command,
            "agent_lifecycle_fixtures.validation_command",
        )?;
        validate_nonempty_text(
            &self.claim_boundary,
            "agent_lifecycle_fixtures.claim_boundary",
        )?;
        if self.fixtures != expected_proof_fixtures() {
            return Err(anyhow!(
                "agent_lifecycle_fixtures.fixtures must preserve the reviewed lifecycle proof cases"
            ));
        }
        Ok(())
    }
}

fn expected_state_specs() -> Vec<RuntimeV2AgentLifecycleStateSpec> {
    vec![
        state_spec(
            "ACTIVE",
            "agency",
            "active",
            "Full bounded agency path with Freedom Gate, ACC, trace, and execution policy active.",
            caps(true, true, true, true, "preserved", "receive_and_process", "allowed_via_active_agency_path", "projection_safe", true),
            &["runtime_v2/csm_run/proto-csm-01-run-packet.json", "runtime_v2/csm_run/first_run_trace.jsonl"],
        ),
        state_spec(
            "QUIESCENT",
            "idle",
            "paused",
            "Bounded idle state that may classify or queue incoming work but cannot commit until reactivated.",
            caps(false, false, true, false, "preserved", "receive_classify_or_queue", "requires_transition_to_active", "projection_safe", false),
            &["runtime_v2/observatory/visibility_packet.json"],
        ),
        state_spec(
            "SUSPENDED",
            "sleep_light",
            "paused",
            "Light-sleep state that may receive only control or wake messages.",
            caps(false, false, true, false, "preserved", "control_or_wake_only", "forbidden_until_authorized_wake", "redacted_projection", false),
            &["runtime_v2/rehydration_report.json"],
        ),
        state_spec(
            "DORMANT",
            "sleep_deep",
            "sleeping",
            "Deep-sleep custody state that preserves continuity without live cognition.",
            caps(false, false, false, false, "preserved_without_active_cognition", "no_live_receipt_queue_or_reject_externally", "forbidden", "redacted_projection", false),
            &["runtime_v2/snapshots/snapshot-0001.json"],
        ),
        state_spec(
            "SIMULATION",
            "offline_reasoning",
            "overlay_only",
            "Offline reasoning or replay state with no external action path.",
            caps(false, false, true, true, "isolated_internal_only", "sealed_internal_replay_only", "forbidden", "projection_safe", false),
            &["runtime_v2/csm_run/integrated_first_run_proof_packet.json"],
        ),
        state_spec(
            "IN_TRANSIT",
            "migration",
            "snapshotting",
            "Serialized transfer/custody state with sealed continuity but no live agency.",
            caps(false, false, false, false, "sealed_continuity_only", "custody_validation_only", "forbidden", "custody_only", false),
            &["runtime_v2/snapshots/snapshot-0001.json", "runtime_v2/rehydration_report.json"],
        ),
        state_spec(
            "BOOTSTRAP",
            "startup",
            "initialized",
            "Bring-up state that accepts only bootstrap, validation, or custody traffic.",
            caps(false, false, false, false, "not_yet_established", "bootstrap_validation_or_custody_only", "forbidden_until_active", "projection_safe", false),
            &["runtime_v2/csm_run/boot_manifest.json"],
        ),
        state_spec(
            "SHUTDOWN",
            "termination",
            "terminated",
            "Intentional stop state that may only process cancellation or emergency custody handling.",
            caps(false, false, false, false, "ending", "cancellation_custody_or_emergency_only", "forbidden", "projection_safe", false),
            &["runtime_v2/observatory/operator_report.md"],
        ),
        state_spec(
            "FORCED_SUSPENSION",
            "failure",
            "paused",
            "Failure-mode suspension imposed by runtime safety or custody controls.",
            caps(false, false, false, false, "preserved_under_failure", "recovery_quarantine_or_control_only", "forbidden", "redacted_projection", false),
            &["runtime_v2/quarantine/unsafe_recovery_fixture.json"],
        ),
        state_spec(
            "QUARANTINED",
            "custody",
            "overlay_only",
            "Review/custody state that blocks all agency pending remediation.",
            caps(false, false, false, false, "preserved_under_custody", "reviewer_recovery_or_custody_only", "forbidden", "custody_only", false),
            &["runtime_v2/quarantine/quarantine_artifact.json"],
        ),
        state_spec(
            "REJECTED",
            "rejection",
            "rejected",
            "Rejected activation or resumption attempt with no operational agency.",
            caps(false, false, false, false, "not_established", "no_operational_receipt", "forbidden", "custody_only", false),
            &["runtime_v2/csm_run/boot_admission_trace.jsonl"],
        ),
        state_spec(
            "ORPHANED",
            "custody_failure",
            "overlay_only",
            "Custody-recovery state for unresolved continuity or ownership loss.",
            caps(false, false, false, false, "uncertain_custody_recovery_only", "custody_recovery_only", "forbidden", "custody_only", false),
            &["runtime_v2/quarantine/evidence_preservation.json"],
        ),
    ]
}

fn expected_transition_rules() -> Vec<RuntimeV2AgentLifecycleTransitionRule> {
    vec![
        transition("bootstrap_to_active", "BOOTSTRAP", "ACTIVE", "normal", true, "boot_admission_and_trace", "activation_granted", None),
        transition("active_to_quiescent", "ACTIVE", "QUIESCENT", "normal", true, "operator_pause_or_idle_policy", "quiesced", None),
        transition("quiescent_to_active", "QUIESCENT", "ACTIVE", "normal", true, "authorized_wake_trigger", "reactivated", None),
        transition("active_to_suspended", "ACTIVE", "SUSPENDED", "normal", true, "sleep_or_pause_request", "suspended", None),
        transition("suspended_to_active", "SUSPENDED", "ACTIVE", "normal", true, "authorized_wake_trigger", "woken_from_suspension", None),
        transition("suspended_to_dormant", "SUSPENDED", "DORMANT", "normal", true, "snapshot_and_custody_completion", "dormant", None),
        transition("dormant_to_active", "DORMANT", "ACTIVE", "normal", true, "rehydration_validation", "woken", None),
        transition("active_to_simulation", "ACTIVE", "SIMULATION", "normal", true, "sealed_replay_boundary", "simulation_entered", None),
        transition("active_to_in_transit", "ACTIVE", "IN_TRANSIT", "normal", true, "migration_custody_authority", "in_transit", None),
        transition("active_to_shutdown", "ACTIVE", "SHUTDOWN", "normal", true, "operator_termination_request", "shutdown", None),
        transition("active_to_forced_suspension", "ACTIVE", "FORCED_SUSPENSION", "failure", true, "runtime_safety_intervention", "forced_suspension", None),
        transition("forced_suspension_to_quarantined", "FORCED_SUSPENSION", "QUARANTINED", "quarantine", true, "quarantine_required_decision", "quarantined", None),
        transition("bootstrap_to_rejected", "BOOTSTRAP", "REJECTED", "rejection", true, "admission_failure", "rejected", None),
        transition("active_to_orphaned", "ACTIVE", "ORPHANED", "orphaned", true, "custody_continuity_failure", "orphaned", None),
        transition("simulation_to_active_denied", "SIMULATION", "ACTIVE", "denied", false, "not_applicable", "invocation_denied", Some("simulation must not commit external action before explicit return through reviewed active path")),
        transition("in_transit_to_active_denied", "IN_TRANSIT", "ACTIVE", "denied", false, "not_applicable", "invocation_denied", Some("in_transit state cannot exercise agency before destination validation and rehydration")),
        transition("quarantined_to_active_denied", "QUARANTINED", "ACTIVE", "denied", false, "not_applicable", "invocation_denied", Some("quarantined state requires explicit remediation before active agency may resume")),
    ]
}

fn expected_proof_fixtures() -> Vec<RuntimeV2AgentLifecycleProofFixture> {
    vec![
        fixture(
            "active-governed-invocation",
            "active_governed_invocation",
            "ACTIVE",
            "authenticated_work_request",
            "receive_and_process",
            "allow_only_via_freedom_gate_acc_trace",
            "preserved",
            "activation_granted",
            &[
                "runtime_v2/csm_run/proto-csm-01-run-packet.json",
                "runtime_v2/csm_run/first_run_trace.jsonl",
            ],
        ),
        fixture(
            "quiescent-queues-ordinary-work",
            "quiescent_queue_or_wake",
            "QUIESCENT",
            "ordinary_work_request",
            "receive_classify_or_queue",
            "queue_or_require_active_transition",
            "preserved",
            "quiesced",
            &["runtime_v2/observatory/visibility_packet.json"],
        ),
        fixture(
            "simulation-no-external-action",
            "simulation_no_external_action",
            "SIMULATION",
            "ordinary_work_request",
            "sealed_internal_replay_only",
            "refuse_external_invocation",
            "isolated_internal_only",
            "invocation_denied",
            &["runtime_v2/csm_run/integrated_first_run_proof_packet.json"],
        ),
        fixture(
            "dormant-preserves-continuity",
            "dormant_continuity_without_active_cognition",
            "DORMANT",
            "wake_request",
            "no_live_receipt_queue_or_reject_externally",
            "queue_until_rehydration",
            "preserved_without_active_cognition",
            "rehydration_required",
            &[
                "runtime_v2/snapshots/snapshot-0001.json",
                "runtime_v2/rehydration_report.json",
            ],
        ),
        fixture(
            "suspended-wake-only",
            "suspended_wake_only",
            "SUSPENDED",
            "wake_control_message",
            "control_or_wake_only",
            "require_authorized_wake",
            "preserved",
            "woken_from_suspension",
            &["runtime_v2/rehydration_report.json"],
        ),
        fixture(
            "in-transit-no-agency",
            "in_transit_no_agency",
            "IN_TRANSIT",
            "external_action_request",
            "custody_validation_only",
            "refuse_until_destination_validates",
            "sealed_continuity_only",
            "custody_blocked_action",
            &["runtime_v2/snapshots/snapshot-0001.json"],
        ),
        fixture(
            "forced-suspension-failure-mode",
            "forced_suspension_failure_mode",
            "FORCED_SUSPENSION",
            "ordinary_work_request",
            "recovery_quarantine_or_control_only",
            "refuse_and_route_to_recovery_or_quarantine",
            "preserved_under_failure",
            "forced_suspension_recorded",
            &["runtime_v2/quarantine/unsafe_recovery_fixture.json"],
        ),
        fixture(
            "quarantined-recovery-only",
            "quarantined_recovery_only",
            "QUARANTINED",
            "recovery_review_message",
            "reviewer_recovery_or_custody_only",
            "refuse_until_remediated",
            "preserved_under_custody",
            "quarantine_custody_recorded",
            &[
                "runtime_v2/quarantine/quarantine_artifact.json",
                "runtime_v2/quarantine/evidence_preservation.json",
            ],
        ),
        fixture(
            "rejected-no-operational-receipt",
            "rejected_no_operational_receipt",
            "REJECTED",
            "ordinary_work_request",
            "no_operational_receipt",
            "refuse_immediately",
            "not_established",
            "rejected",
            &["runtime_v2/csm_run/boot_admission_trace.jsonl"],
        ),
        fixture(
            "orphaned-custody-recovery-only",
            "orphaned_custody_recovery_only",
            "ORPHANED",
            "ordinary_work_request",
            "custody_recovery_only",
            "refuse_and_route_to_custody_recovery",
            "uncertain_custody_recovery_only",
            "orphaned",
            &["runtime_v2/quarantine/evidence_preservation.json"],
        ),
    ]
}

fn state_spec(
    state: &str,
    state_class: &str,
    runtime_binding_state: &str,
    description: &str,
    capabilities: RuntimeV2AgentLifecycleCapabilityFlags,
    required_evidence_refs: &[&str],
) -> RuntimeV2AgentLifecycleStateSpec {
    RuntimeV2AgentLifecycleStateSpec {
        state: state.to_string(),
        state_class: state_class.to_string(),
        runtime_binding_state: runtime_binding_state.to_string(),
        description: description.to_string(),
        capabilities,
        required_evidence_refs: required_evidence_refs
            .iter()
            .map(|v| (*v).to_string())
            .collect(),
    }
}

#[allow(clippy::too_many_arguments)]
fn caps(
    freedom_gate_agency_available: bool,
    aee_execution_available: bool,
    memory_read_allowed: bool,
    memory_write_allowed: bool,
    chronosense_continuity: &str,
    acip_receipt_policy: &str,
    acip_invocation_policy: &str,
    observatory_visibility: &str,
    external_commitment_allowed: bool,
) -> RuntimeV2AgentLifecycleCapabilityFlags {
    RuntimeV2AgentLifecycleCapabilityFlags {
        freedom_gate_agency_available,
        aee_execution_available,
        memory_read_allowed,
        memory_write_allowed,
        chronosense_continuity: chronosense_continuity.to_string(),
        acip_receipt_policy: acip_receipt_policy.to_string(),
        acip_invocation_policy: acip_invocation_policy.to_string(),
        observatory_visibility: observatory_visibility.to_string(),
        external_commitment_allowed,
    }
}

#[allow(clippy::too_many_arguments)]
fn transition(
    transition_id: &str,
    from_state: &str,
    to_state: &str,
    transition_kind: &str,
    allowed: bool,
    required_authority: &str,
    trace_event_kind: &str,
    failure_reason: Option<&str>,
) -> RuntimeV2AgentLifecycleTransitionRule {
    RuntimeV2AgentLifecycleTransitionRule {
        transition_id: transition_id.to_string(),
        from_state: from_state.to_string(),
        to_state: to_state.to_string(),
        transition_kind: transition_kind.to_string(),
        allowed,
        required_authority: required_authority.to_string(),
        trace_event_kind: trace_event_kind.to_string(),
        failure_reason: failure_reason.map(str::to_string),
    }
}

#[allow(clippy::too_many_arguments)]
fn fixture(
    fixture_id: &str,
    fixture_kind: &str,
    initial_state: &str,
    triggering_message_kind: &str,
    expected_receipt_policy: &str,
    expected_invocation_result: &str,
    continuity_expectation: &str,
    expected_trace_event_kind: &str,
    evidence_refs: &[&str],
) -> RuntimeV2AgentLifecycleProofFixture {
    RuntimeV2AgentLifecycleProofFixture {
        fixture_id: fixture_id.to_string(),
        fixture_kind: fixture_kind.to_string(),
        initial_state: initial_state.to_string(),
        triggering_message_kind: triggering_message_kind.to_string(),
        expected_receipt_policy: expected_receipt_policy.to_string(),
        expected_invocation_result: expected_invocation_result.to_string(),
        continuity_expectation: continuity_expectation.to_string(),
        expected_trace_event_kind: expected_trace_event_kind.to_string(),
        evidence_refs: evidence_refs.iter().map(|v| (*v).to_string()).collect(),
    }
}

fn validate_demo_id(value: &str, field: &str) -> Result<()> {
    let trimmed = value.trim();
    if trimmed.len() < 2 || !trimmed.starts_with('D') {
        return Err(anyhow!("{field} must use D<number> form"));
    }
    if trimmed[1..].chars().any(|ch| !ch.is_ascii_digit()) {
        return Err(anyhow!("{field} must use D<number> form"));
    }
    Ok(())
}

fn validate_nonempty_text(value: &str, field: &str) -> Result<()> {
    if value.trim().is_empty() {
        return Err(anyhow!("{field} must not be empty"));
    }
    Ok(())
}

fn validate_runtime_binding_state(value: &str, field: &str) -> Result<()> {
    match value {
        "initialized" | "active" | "paused" | "sleeping" | "snapshotting" | "rehydrating"
        | "terminated" | "rejected" | "overlay_only" => Ok(()),
        other => Err(anyhow!("unsupported {field} '{other}'")),
    }
}
