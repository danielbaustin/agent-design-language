use std::path::{Path, PathBuf};

use anyhow::{anyhow, Context, Result};
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
pub const RUNTIME_V2_FOUNDATION_PROOF_PACKET_SCHEMA: &str = "runtime_v2.foundation_proof_packet.v1";
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2FoundationArtifactRef {
    pub artifact_id: String,
    pub artifact_kind: String,
    pub schema_version: String,
    pub path: String,
    pub source_wp: String,
    pub proves: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2FoundationProofCheck {
    pub check_id: String,
    pub status: String,
    pub evidence_ref: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2FoundationProofPacket {
    pub schema_version: String,
    pub proof_id: String,
    pub demo_id: String,
    pub demo_name: String,
    pub classification: String,
    pub manifold_id: String,
    pub artifact_path: String,
    pub generated_at_utc: String,
    pub integrated_artifacts: Vec<RuntimeV2FoundationArtifactRef>,
    pub proof_claims: Vec<String>,
    pub non_claims: Vec<String>,
    pub checks: Vec<RuntimeV2FoundationProofCheck>,
    pub reviewer_entrypoint: String,
    pub replay_command: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeV2FoundationPrototypeArtifacts {
    pub manifold: RuntimeV2ManifoldRoot,
    pub kernel: RuntimeV2KernelLoopArtifacts,
    pub citizens: RuntimeV2CitizenLifecycleArtifacts,
    pub snapshot: RuntimeV2SnapshotAndRehydrationArtifacts,
    pub invariant_violation: RuntimeV2InvariantViolationArtifact,
    pub operator_report: RuntimeV2OperatorControlReport,
    pub security_boundary: RuntimeV2SecurityBoundaryProofPacket,
    pub proof_packet: RuntimeV2FoundationProofPacket,
}

impl RuntimeV2ManifoldRoot {
    pub fn prototype(manifold_id: impl Into<String>) -> Result<Self> {
        let manifold_id = normalize_id(manifold_id.into(), "manifold_id")?;
        Ok(Self {
            schema_version: RUNTIME_V2_MANIFOLD_SCHEMA.to_string(),
            artifact_path: DEFAULT_MANIFOLD_ARTIFACT_PATH.to_string(),
            lifecycle_state: "initialized".to_string(),
            clock_anchor: ManifoldClockAnchor {
                anchor_id: "clock_anchor_0000".to_string(),
                clock_kind: "monotonic_logical".to_string(),
                monotonic_tick: 0,
                observed_at_utc: "not_started".to_string(),
            },
            citizen_registry_refs: CitizenRegistryRefs {
                registry_root: "runtime_v2/citizens".to_string(),
                active_index: "runtime_v2/citizens/active_index.json".to_string(),
                pending_index: "runtime_v2/citizens/pending_index.json".to_string(),
            },
            kernel_service_refs: KernelServiceRefs {
                registry_path: "runtime_v2/kernel/service_registry.json".to_string(),
                service_loop_path: "runtime_v2/kernel/service_loop.jsonl".to_string(),
                service_state_path: "runtime_v2/kernel/service_state.json".to_string(),
            },
            trace_root: TraceRootRef {
                trace_root: "runtime_v2/traces".to_string(),
                event_log_path: "runtime_v2/traces/events.jsonl".to_string(),
                next_event_sequence: 1,
            },
            snapshot_root: SnapshotRootRef {
                snapshot_root: "runtime_v2/snapshots".to_string(),
                latest_snapshot_id: None,
                rehydration_report_path: "runtime_v2/rehydration_report.json".to_string(),
            },
            resource_ledger: ResourceLedgerRef {
                ledger_path: "runtime_v2/resource_ledger.json".to_string(),
                accounting_mode: "bounded_prototype".to_string(),
            },
            invariant_policy_refs: InvariantPolicyRefs {
                policy_path: "runtime_v2/invariants/policy.json".to_string(),
                enforcement_mode: "fail_closed_before_activation".to_string(),
                blocking_invariants: vec![
                    "single_active_manifold_instance".to_string(),
                    "no_duplicate_active_citizen_instance".to_string(),
                    "trace_sequence_must_advance_monotonically".to_string(),
                    "snapshot_restore_must_validate_before_active_state".to_string(),
                ],
            },
            review_surface: RuntimeV2ManifoldReviewSurface {
                required_artifacts: vec![
                    DEFAULT_MANIFOLD_ARTIFACT_PATH.to_string(),
                    "runtime_v2/citizens/active_index.json".to_string(),
                    "runtime_v2/kernel/service_registry.json".to_string(),
                    "runtime_v2/traces/events.jsonl".to_string(),
                    "runtime_v2/snapshots".to_string(),
                    "runtime_v2/invariants/policy.json".to_string(),
                ],
                proof_hook_command: "cargo test --manifest-path adl/Cargo.toml runtime_v2::tests::runtime_v2_manifold_root_contract_is_stable".to_string(),
                proof_hook_output_path: DEFAULT_MANIFOLD_ARTIFACT_PATH.to_string(),
                downstream_boundaries: vec![
                    "WP-06 owns the bounded kernel service loop behavior".to_string(),
                    "WP-07 owns provisional citizen record materialization".to_string(),
                    "WP-08 owns snapshot writing, sealing, and rehydration".to_string(),
                    "WP-09 owns invariant violation artifacts".to_string(),
                ],
                non_goals: vec![
                    "no true Godel-agent birthday or identity rebinding".to_string(),
                    "no full moral, emotional, or polis governance layer".to_string(),
                    "no cross-machine migration or cross-polis state transfer".to_string(),
                    "no live kernel scheduling behavior in WP-05".to_string(),
                ],
            },
            manifold_id,
        })
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_MANIFOLD_SCHEMA {
            return Err(anyhow!(
                "unsupported Runtime v2 manifold schema '{}'",
                self.schema_version
            ));
        }
        normalize_id(self.manifold_id.clone(), "manifold_id")?;
        validate_lifecycle_state(&self.lifecycle_state)?;
        validate_relative_path(&self.artifact_path, "artifact_path")?;
        validate_clock_anchor(&self.clock_anchor)?;
        validate_registry_refs(&self.citizen_registry_refs)?;
        validate_kernel_refs(&self.kernel_service_refs)?;
        validate_trace_root(&self.trace_root)?;
        validate_snapshot_root(&self.snapshot_root)?;
        validate_relative_path(
            &self.resource_ledger.ledger_path,
            "resource_ledger.ledger_path",
        )?;
        normalize_id(
            self.resource_ledger.accounting_mode.clone(),
            "resource_ledger.accounting_mode",
        )?;
        validate_invariant_policy_refs(&self.invariant_policy_refs)?;
        validate_review_surface(&self.review_surface)?;
        Ok(())
    }

    pub fn to_pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.validate()?;
        serde_json::to_vec_pretty(self).context("serialize Runtime v2 manifold root")
    }

    pub fn write_to_path(&self, path: impl AsRef<Path>) -> Result<()> {
        let path = path.as_ref();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).with_context(|| {
                format!(
                    "failed to create Runtime v2 manifold parent '{}'",
                    parent.display()
                )
            })?;
        }
        std::fs::write(path, self.to_pretty_json_bytes()?).with_context(|| {
            format!(
                "failed to write Runtime v2 manifold root '{}'",
                path.display()
            )
        })
    }

    pub fn read_from_path(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let bytes = std::fs::read(path).with_context(|| {
            format!(
                "failed to read Runtime v2 manifold root '{}'",
                path.display()
            )
        })?;
        let root: Self =
            serde_json::from_slice(&bytes).context("parse Runtime v2 manifold root")?;
        root.validate()?;
        Ok(root)
    }

    pub fn artifact_path_buf(&self) -> PathBuf {
        PathBuf::from(&self.artifact_path)
    }
}

impl RuntimeV2KernelLoopArtifacts {
    pub fn prototype(manifold: &RuntimeV2ManifoldRoot) -> Result<Self> {
        manifold.validate()?;
        let services = prototype_kernel_services();
        let events = services
            .iter()
            .enumerate()
            .map(|(index, service)| RuntimeV2KernelLoopEvent {
                schema_version: RUNTIME_V2_KERNEL_LOOP_EVENT_SCHEMA.to_string(),
                event_sequence: manifold.trace_root.next_event_sequence + index as u64,
                manifold_id: manifold.manifold_id.clone(),
                service_id: service.service_id.clone(),
                action: "service_tick".to_string(),
                outcome: "observed_ready".to_string(),
                artifact_ref: service.owns_artifact_path.clone(),
            })
            .collect::<Vec<_>>();
        let completed_through_event_sequence = events
            .last()
            .map(|event| event.event_sequence)
            .unwrap_or(manifold.trace_root.next_event_sequence);
        let state_services = events
            .iter()
            .map(|event| RuntimeV2KernelServiceStatus {
                service_id: event.service_id.clone(),
                lifecycle_state: "ready".to_string(),
                last_event_sequence: event.event_sequence,
                blocked_reason: None,
            })
            .collect::<Vec<_>>();
        let artifacts = Self {
            registry: RuntimeV2KernelServiceRegistry {
                schema_version: RUNTIME_V2_KERNEL_SERVICE_REGISTRY_SCHEMA.to_string(),
                manifold_id: manifold.manifold_id.clone(),
                registry_path: manifold.kernel_service_refs.registry_path.clone(),
                services,
            },
            state: RuntimeV2KernelServiceState {
                schema_version: RUNTIME_V2_KERNEL_SERVICE_STATE_SCHEMA.to_string(),
                manifold_id: manifold.manifold_id.clone(),
                service_state_path: manifold.kernel_service_refs.service_state_path.clone(),
                loop_status: "bounded_tick_complete".to_string(),
                completed_through_event_sequence,
                services: state_services,
            },
            events,
            service_loop_path: manifold.kernel_service_refs.service_loop_path.clone(),
        };
        artifacts.validate()?;
        Ok(artifacts)
    }

    pub fn validate(&self) -> Result<()> {
        self.registry.validate()?;
        self.state.validate()?;
        validate_relative_path(&self.service_loop_path, "kernel_loop.service_loop_path")?;
        if self.events.is_empty() {
            return Err(anyhow!("kernel_loop.events must not be empty"));
        }
        if self.registry.manifold_id != self.state.manifold_id {
            return Err(anyhow!(
                "kernel registry and service state manifold ids must match"
            ));
        }
        let mut seen_services = Vec::new();
        for (expected_sequence, event) in (self.events[0].event_sequence..).zip(self.events.iter())
        {
            event.validate()?;
            if event.manifold_id != self.registry.manifold_id {
                return Err(anyhow!("kernel loop event manifold id must match registry"));
            }
            if event.event_sequence != expected_sequence {
                return Err(anyhow!(
                    "kernel loop events must be contiguous and monotonically ordered"
                ));
            }
            if !self
                .registry
                .services
                .iter()
                .any(|service| service.service_id == event.service_id)
            {
                return Err(anyhow!(
                    "kernel loop event references unknown service '{}'",
                    event.service_id
                ));
            }
            seen_services.push(event.service_id.clone());
        }
        let registry_ids = self
            .registry
            .services
            .iter()
            .map(|service| service.service_id.clone())
            .collect::<Vec<_>>();
        if seen_services != registry_ids {
            return Err(anyhow!(
                "kernel loop event order must match service activation order"
            ));
        }
        if self.state.completed_through_event_sequence
            != self
                .events
                .last()
                .expect("events checked non-empty")
                .event_sequence
        {
            return Err(anyhow!(
                "kernel service state must record the last loop event sequence"
            ));
        }
        Ok(())
    }

    pub fn registry_pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.validate()?;
        serde_json::to_vec_pretty(&self.registry).context("serialize kernel service registry")
    }

    pub fn state_pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.validate()?;
        serde_json::to_vec_pretty(&self.state).context("serialize kernel service state")
    }

    pub fn service_loop_jsonl_bytes(&self) -> Result<Vec<u8>> {
        self.validate()?;
        let mut out = Vec::new();
        for event in &self.events {
            serde_json::to_writer(&mut out, event).context("serialize kernel loop event")?;
            out.push(b'\n');
        }
        Ok(out)
    }

    pub fn write_to_root(&self, root: impl AsRef<Path>) -> Result<()> {
        let root = root.as_ref();
        write_relative(
            root,
            &self.registry.registry_path,
            self.registry_pretty_json_bytes()?,
        )?;
        write_relative(
            root,
            &self.state.service_state_path,
            self.state_pretty_json_bytes()?,
        )?;
        write_relative(
            root,
            &self.service_loop_path,
            self.service_loop_jsonl_bytes()?,
        )?;
        Ok(())
    }
}

impl RuntimeV2CitizenLifecycleArtifacts {
    pub fn prototype(manifold: &RuntimeV2ManifoldRoot) -> Result<Self> {
        manifold.validate()?;
        let records = vec![
            RuntimeV2ProvisionalCitizenRecord {
                schema_version: RUNTIME_V2_PROVISIONAL_CITIZEN_SCHEMA.to_string(),
                citizen_id: "proto-citizen-alpha".to_string(),
                display_name: "Prototype Citizen Alpha".to_string(),
                provisional_status: "provisional".to_string(),
                lifecycle_state: "active".to_string(),
                manifold_id: manifold.manifold_id.clone(),
                record_path: "runtime_v2/citizens/proto-citizen-alpha.json".to_string(),
                created_at_utc: "not_started".to_string(),
                last_wake_at_utc: None,
                memory_identity_refs: RuntimeV2CitizenMemoryIdentityRefs {
                    memory_root_ref: "runtime_v2/citizens/proto-citizen-alpha/memory".to_string(),
                    identity_profile_ref: "runtime_v2/citizens/proto-citizen-alpha/identity.json"
                        .to_string(),
                },
                policy_boundary_refs: RuntimeV2CitizenPolicyBoundaryRefs {
                    policy_ref: "runtime_v2/citizens/proto-citizen-alpha/policy.json".to_string(),
                    admission_trace_ref: "runtime_v2/traces/admission/proto-citizen-alpha.json"
                        .to_string(),
                },
                rehydration_validation_ref: None,
                termination_event_ref: None,
                resources_released: false,
                can_execute_episodes: true,
            },
            RuntimeV2ProvisionalCitizenRecord {
                schema_version: RUNTIME_V2_PROVISIONAL_CITIZEN_SCHEMA.to_string(),
                citizen_id: "proto-citizen-beta".to_string(),
                display_name: "Prototype Citizen Beta".to_string(),
                provisional_status: "provisional".to_string(),
                lifecycle_state: "proposed".to_string(),
                manifold_id: manifold.manifold_id.clone(),
                record_path: "runtime_v2/citizens/proto-citizen-beta.json".to_string(),
                created_at_utc: "not_started".to_string(),
                last_wake_at_utc: None,
                memory_identity_refs: RuntimeV2CitizenMemoryIdentityRefs {
                    memory_root_ref: "runtime_v2/citizens/proto-citizen-beta/memory".to_string(),
                    identity_profile_ref: "runtime_v2/citizens/proto-citizen-beta/identity.json"
                        .to_string(),
                },
                policy_boundary_refs: RuntimeV2CitizenPolicyBoundaryRefs {
                    policy_ref: "runtime_v2/citizens/proto-citizen-beta/policy.json".to_string(),
                    admission_trace_ref: "runtime_v2/traces/admission/proto-citizen-beta.json"
                        .to_string(),
                },
                rehydration_validation_ref: None,
                termination_event_ref: None,
                resources_released: false,
                can_execute_episodes: false,
            },
        ];
        let active_citizens = records
            .iter()
            .filter(|record| record.lifecycle_state == "active")
            .map(RuntimeV2CitizenRegistryEntry::from_record)
            .collect();
        let pending_citizens = records
            .iter()
            .filter(|record| record.lifecycle_state != "active")
            .map(RuntimeV2CitizenRegistryEntry::from_record)
            .collect();
        let active_index = RuntimeV2CitizenRegistryIndex {
            schema_version: RUNTIME_V2_CITIZEN_REGISTRY_INDEX_SCHEMA.to_string(),
            manifold_id: manifold.manifold_id.clone(),
            registry_root: manifold.citizen_registry_refs.registry_root.clone(),
            index_kind: "active".to_string(),
            index_path: manifold.citizen_registry_refs.active_index.clone(),
            citizens: active_citizens,
        };
        let pending_index = RuntimeV2CitizenRegistryIndex {
            schema_version: RUNTIME_V2_CITIZEN_REGISTRY_INDEX_SCHEMA.to_string(),
            manifold_id: manifold.manifold_id.clone(),
            registry_root: manifold.citizen_registry_refs.registry_root.clone(),
            index_kind: "pending".to_string(),
            index_path: manifold.citizen_registry_refs.pending_index.clone(),
            citizens: pending_citizens,
        };
        let artifacts = Self {
            records,
            active_index,
            pending_index,
        };
        artifacts.validate()?;
        Ok(artifacts)
    }

    pub fn validate(&self) -> Result<()> {
        self.active_index.validate()?;
        self.pending_index.validate()?;
        if self.active_index.index_kind != "active" {
            return Err(anyhow!("citizen active index must use index_kind active"));
        }
        if self.pending_index.index_kind != "pending" {
            return Err(anyhow!("citizen pending index must use index_kind pending"));
        }
        if self.records.is_empty() {
            return Err(anyhow!("citizen_lifecycle.records must not be empty"));
        }
        let mut all_ids = std::collections::BTreeSet::new();
        let mut active_ids = std::collections::BTreeSet::new();
        for record in &self.records {
            record.validate()?;
            if record.manifold_id != self.active_index.manifold_id
                || record.manifold_id != self.pending_index.manifold_id
            {
                return Err(anyhow!(
                    "citizen record manifold id must match registry index"
                ));
            }
            if !all_ids.insert(record.citizen_id.clone()) {
                return Err(anyhow!(
                    "citizen_lifecycle.records contains duplicate citizen '{}'",
                    record.citizen_id
                ));
            }
            if record.lifecycle_state == "active" && !active_ids.insert(record.citizen_id.clone()) {
                return Err(anyhow!(
                    "citizen_lifecycle.records contains duplicate active citizen '{}'",
                    record.citizen_id
                ));
            }
        }
        let active_entries = self
            .records
            .iter()
            .filter(|record| record.lifecycle_state == "active")
            .map(RuntimeV2CitizenRegistryEntry::from_record)
            .collect::<Vec<_>>();
        let pending_entries = self
            .records
            .iter()
            .filter(|record| record.lifecycle_state != "active")
            .map(RuntimeV2CitizenRegistryEntry::from_record)
            .collect::<Vec<_>>();
        if self.active_index.citizens != active_entries {
            return Err(anyhow!(
                "citizen active index must match active lifecycle records"
            ));
        }
        if self.pending_index.citizens != pending_entries {
            return Err(anyhow!(
                "citizen pending index must match non-active lifecycle records"
            ));
        }
        Ok(())
    }

    pub fn record_pretty_json_bytes(record: &RuntimeV2ProvisionalCitizenRecord) -> Result<Vec<u8>> {
        record.validate()?;
        serde_json::to_vec_pretty(record).context("serialize provisional citizen record")
    }

    pub fn index_pretty_json_bytes(index: &RuntimeV2CitizenRegistryIndex) -> Result<Vec<u8>> {
        index.validate()?;
        serde_json::to_vec_pretty(index).context("serialize citizen registry index")
    }

    pub fn write_to_root(&self, root: impl AsRef<Path>) -> Result<()> {
        let root = root.as_ref();
        self.validate()?;
        for record in &self.records {
            write_relative(
                root,
                &record.record_path,
                Self::record_pretty_json_bytes(record)?,
            )?;
        }
        write_relative(
            root,
            &self.active_index.index_path,
            Self::index_pretty_json_bytes(&self.active_index)?,
        )?;
        write_relative(
            root,
            &self.pending_index.index_path,
            Self::index_pretty_json_bytes(&self.pending_index)?,
        )?;
        Ok(())
    }
}

impl RuntimeV2SnapshotAndRehydrationArtifacts {
    pub fn prototype(
        manifold: &RuntimeV2ManifoldRoot,
        kernel: &RuntimeV2KernelLoopArtifacts,
        citizens: &RuntimeV2CitizenLifecycleArtifacts,
    ) -> Result<Self> {
        manifold.validate()?;
        kernel.validate()?;
        citizens.validate()?;
        if kernel.state.manifold_id != manifold.manifold_id {
            return Err(anyhow!(
                "snapshot kernel state manifold id must match manifold"
            ));
        }
        if citizens.active_index.manifold_id != manifold.manifold_id {
            return Err(anyhow!("snapshot citizen manifold id must match manifold"));
        }

        let snapshot_id = "snapshot-0001".to_string();
        let mut manifold_state = manifold.clone();
        manifold_state.lifecycle_state = "snapshotting".to_string();
        manifold_state.snapshot_root.latest_snapshot_id = Some(snapshot_id.clone());
        let invariant_status = manifold
            .invariant_policy_refs
            .blocking_invariants
            .iter()
            .map(|invariant_id| RuntimeV2SnapshotInvariantStatus {
                invariant_id: invariant_id.clone(),
                status: "passed".to_string(),
                checked_before_snapshot: true,
            })
            .collect::<Vec<_>>();
        let mut snapshot = RuntimeV2SnapshotManifest {
            schema_version: RUNTIME_V2_SNAPSHOT_MANIFEST_SCHEMA.to_string(),
            snapshot_id: snapshot_id.clone(),
            manifold_id: manifold.manifold_id.clone(),
            snapshot_path: "runtime_v2/snapshots/snapshot-0001.json".to_string(),
            created_at_utc: "not_started".to_string(),
            manifold_state,
            citizen_records: citizens.records.clone(),
            active_index: citizens.active_index.clone(),
            pending_index: citizens.pending_index.clone(),
            kernel_service_state: kernel.state.clone(),
            last_trace_cursor: kernel.state.completed_through_event_sequence,
            invariant_status,
            structural_checksum: String::new(),
        };
        snapshot.structural_checksum = snapshot.compute_structural_checksum()?;
        snapshot.validate()?;

        let rehydration_report = RuntimeV2RehydrationReport {
            schema_version: RUNTIME_V2_REHYDRATION_REPORT_SCHEMA.to_string(),
            snapshot_id: snapshot.snapshot_id.clone(),
            manifold_id: snapshot.manifold_id.clone(),
            report_path: snapshot
                .manifold_state
                .snapshot_root
                .rehydration_report_path
                .clone(),
            restored_manifold_id: snapshot.manifold_id.clone(),
            restored_lifecycle_state: "active".to_string(),
            trace_resume_sequence: snapshot.last_trace_cursor + 1,
            invariant_checks_ran_before_resume: true,
            duplicate_active_citizen_detected: false,
            restored_active_citizens: snapshot
                .active_index
                .citizens
                .iter()
                .map(|entry| entry.citizen_id.clone())
                .collect(),
            wake_allowed: true,
            wake_refused_reason: None,
            snapshot_checksum: snapshot.structural_checksum.clone(),
            rehydrated_at_utc: "not_started".to_string(),
        };
        let artifacts = Self {
            snapshot,
            rehydration_report,
        };
        artifacts.validate()?;
        Ok(artifacts)
    }

    pub fn validate(&self) -> Result<()> {
        self.snapshot.validate()?;
        self.rehydration_report
            .validate_against_snapshot(&self.snapshot)
    }

    pub fn snapshot_pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.validate()?;
        serde_json::to_vec_pretty(&self.snapshot).context("serialize Runtime v2 snapshot manifest")
    }

    pub fn rehydration_report_pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.validate()?;
        serde_json::to_vec_pretty(&self.rehydration_report)
            .context("serialize Runtime v2 rehydration report")
    }

    pub fn write_to_root(&self, root: impl AsRef<Path>) -> Result<()> {
        let root = root.as_ref();
        write_relative(
            root,
            &self.snapshot.snapshot_path,
            self.snapshot_pretty_json_bytes()?,
        )?;
        write_relative(
            root,
            &self.rehydration_report.report_path,
            self.rehydration_report_pretty_json_bytes()?,
        )?;
        Ok(())
    }
}

impl RuntimeV2SnapshotManifest {
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_SNAPSHOT_MANIFEST_SCHEMA {
            return Err(anyhow!(
                "unsupported Runtime v2 snapshot schema '{}'",
                self.schema_version
            ));
        }
        normalize_id(self.snapshot_id.clone(), "snapshot.snapshot_id")?;
        normalize_id(self.manifold_id.clone(), "snapshot.manifold_id")?;
        validate_relative_path(&self.snapshot_path, "snapshot.snapshot_path")?;
        validate_timestamp_marker(&self.created_at_utc, "snapshot.created_at_utc")?;
        self.manifold_state.validate()?;
        if self.manifold_state.manifold_id != self.manifold_id {
            return Err(anyhow!("snapshot manifold id must match manifold state"));
        }
        if self.manifold_state.lifecycle_state != "snapshotting" {
            return Err(anyhow!(
                "snapshot manifold state must be captured while snapshotting"
            ));
        }
        if self
            .manifold_state
            .snapshot_root
            .latest_snapshot_id
            .as_ref()
            != Some(&self.snapshot_id)
        {
            return Err(anyhow!(
                "snapshot manifold state must record the latest snapshot id"
            ));
        }
        self.kernel_service_state.validate()?;
        if self.kernel_service_state.manifold_id != self.manifold_id {
            return Err(anyhow!(
                "snapshot kernel service state manifold id must match snapshot"
            ));
        }
        let lifecycle = RuntimeV2CitizenLifecycleArtifacts {
            records: self.citizen_records.clone(),
            active_index: self.active_index.clone(),
            pending_index: self.pending_index.clone(),
        };
        lifecycle.validate()?;
        if self.active_index.manifold_id != self.manifold_id
            || self.pending_index.manifold_id != self.manifold_id
        {
            return Err(anyhow!(
                "snapshot citizen indexes must match snapshot manifold"
            ));
        }
        if self.last_trace_cursor != self.kernel_service_state.completed_through_event_sequence {
            return Err(anyhow!(
                "snapshot last_trace_cursor must match completed kernel event sequence"
            ));
        }
        validate_snapshot_invariant_statuses(&self.invariant_status)?;
        if !self
            .invariant_status
            .iter()
            .all(|status| status.status == "passed" && status.checked_before_snapshot)
        {
            return Err(anyhow!(
                "snapshot invariant checks must pass before rehydration can be allowed"
            ));
        }
        let expected_checksum = self.compute_structural_checksum()?;
        if self.structural_checksum != expected_checksum {
            return Err(anyhow!("snapshot structural checksum mismatch"));
        }
        Ok(())
    }

    fn compute_structural_checksum(&self) -> Result<String> {
        checksum_for_serialize(&(
            &self.schema_version,
            &self.snapshot_id,
            &self.manifold_id,
            &self.snapshot_path,
            &self.created_at_utc,
            &self.manifold_state,
            &self.citizen_records,
            &self.active_index,
            &self.pending_index,
            &self.kernel_service_state,
            &self.last_trace_cursor,
            &self.invariant_status,
        ))
    }
}

impl RuntimeV2RehydrationReport {
    pub fn validate_against_snapshot(&self, snapshot: &RuntimeV2SnapshotManifest) -> Result<()> {
        if self.schema_version != RUNTIME_V2_REHYDRATION_REPORT_SCHEMA {
            return Err(anyhow!(
                "unsupported Runtime v2 rehydration report schema '{}'",
                self.schema_version
            ));
        }
        normalize_id(self.snapshot_id.clone(), "rehydration.snapshot_id")?;
        normalize_id(self.manifold_id.clone(), "rehydration.manifold_id")?;
        validate_relative_path(&self.report_path, "rehydration.report_path")?;
        normalize_id(
            self.restored_manifold_id.clone(),
            "rehydration.restored_manifold_id",
        )?;
        validate_lifecycle_state(&self.restored_lifecycle_state)?;
        validate_timestamp_marker(&self.rehydrated_at_utc, "rehydration.rehydrated_at_utc")?;
        if self.snapshot_id != snapshot.snapshot_id {
            return Err(anyhow!(
                "rehydration report snapshot id must match snapshot"
            ));
        }
        if self.manifold_id != snapshot.manifold_id
            || self.restored_manifold_id != snapshot.manifold_id
        {
            return Err(anyhow!(
                "rehydration restored manifold id must match snapshot manifold id"
            ));
        }
        if self.trace_resume_sequence <= snapshot.last_trace_cursor {
            return Err(anyhow!(
                "rehydration trace must resume after the snapshot cursor"
            ));
        }
        if !self.invariant_checks_ran_before_resume {
            return Err(anyhow!(
                "rehydration invariants must run before active state resumes"
            ));
        }
        let mut restored_ids = std::collections::BTreeSet::new();
        for citizen_id in &self.restored_active_citizens {
            normalize_id(citizen_id.clone(), "rehydration.restored_active_citizens")?;
            if !restored_ids.insert(citizen_id.clone()) {
                return Err(anyhow!(
                    "rehydration restored active citizens contain duplicate '{}'",
                    citizen_id
                ));
            }
        }
        let snapshot_active_ids = snapshot
            .active_index
            .citizens
            .iter()
            .map(|entry| entry.citizen_id.clone())
            .collect::<Vec<_>>();
        if self.restored_active_citizens != snapshot_active_ids {
            return Err(anyhow!(
                "rehydration restored active citizens must match snapshot active index"
            ));
        }
        if self.duplicate_active_citizen_detected {
            return Err(anyhow!(
                "rehydration must refuse duplicate active citizen instances"
            ));
        }
        if self.snapshot_checksum != snapshot.structural_checksum {
            return Err(anyhow!("rehydration snapshot checksum must match snapshot"));
        }
        let expected_wake_allowed = self.invariant_checks_ran_before_resume
            && !self.duplicate_active_citizen_detected
            && self.trace_resume_sequence > snapshot.last_trace_cursor;
        if self.wake_allowed != expected_wake_allowed {
            return Err(anyhow!(
                "rehydration wake_allowed must reflect invariant, duplicate, and trace checks"
            ));
        }
        if self.wake_allowed && self.wake_refused_reason.is_some() {
            return Err(anyhow!(
                "rehydration wake_refused_reason must be absent when wake is allowed"
            ));
        }
        if !self.wake_allowed
            && self
                .wake_refused_reason
                .as_ref()
                .map(|reason| reason.trim().is_empty())
                .unwrap_or(true)
        {
            return Err(anyhow!(
                "rehydration wake_refused_reason must explain refused wake"
            ));
        }
        Ok(())
    }
}

impl RuntimeV2InvariantViolationArtifact {
    pub fn duplicate_active_citizen_prototype(
        manifold: &RuntimeV2ManifoldRoot,
        kernel: &RuntimeV2KernelLoopArtifacts,
        citizens: &RuntimeV2CitizenLifecycleArtifacts,
    ) -> Result<Self> {
        manifold.validate()?;
        kernel.validate()?;
        citizens.validate()?;
        if kernel.state.manifold_id != manifold.manifold_id
            || citizens.active_index.manifold_id != manifold.manifold_id
        {
            return Err(anyhow!(
                "invariant violation inputs must share the same manifold id"
            ));
        }
        let invariant_id = "no_duplicate_active_citizen_instance".to_string();
        if !manifold
            .invariant_policy_refs
            .blocking_invariants
            .contains(&invariant_id)
        {
            return Err(anyhow!(
                "manifold policy must declare no_duplicate_active_citizen_instance"
            ));
        }
        let active_citizen = citizens.active_index.citizens.first().ok_or_else(|| {
            anyhow!("duplicate active citizen prototype requires an active citizen")
        })?;
        let mut illegal = citizens.clone();
        let duplicate_record = citizens
            .records
            .iter()
            .find(|record| record.citizen_id == active_citizen.citizen_id)
            .ok_or_else(|| anyhow!("active citizen record missing from lifecycle records"))?
            .clone();
        illegal.records.push(duplicate_record);
        let source_error = illegal
            .validate()
            .expect_err("duplicate active citizen input must be rejected")
            .to_string();
        let artifact = Self {
            schema_version: RUNTIME_V2_INVARIANT_VIOLATION_SCHEMA.to_string(),
            violation_id: "violation-0001".to_string(),
            manifold_id: manifold.manifold_id.clone(),
            artifact_path: "runtime_v2/invariants/violation-0001.json".to_string(),
            detected_at_utc: "not_started".to_string(),
            severity: "blocking".to_string(),
            invariant_id,
            invariant_owner_service_id: "invariant_checker".to_string(),
            policy_enforcement_mode: manifold.invariant_policy_refs.enforcement_mode.clone(),
            attempted_transition: RuntimeV2InvariantViolationAttempt {
                actor: "kernel.identity_admission_guard".to_string(),
                attempted_action: "duplicate_active_citizen_activation".to_string(),
                attempted_state: "active_index_with_duplicate_proto_citizen_alpha".to_string(),
                source_artifact_ref: active_citizen.record_path.clone(),
            },
            evaluated_refs: vec![
                RuntimeV2InvariantViolationEvaluatedRef {
                    ref_kind: "active_index".to_string(),
                    artifact_ref: citizens.active_index.index_path.clone(),
                },
                RuntimeV2InvariantViolationEvaluatedRef {
                    ref_kind: "kernel_state".to_string(),
                    artifact_ref: kernel.state.service_state_path.clone(),
                },
                RuntimeV2InvariantViolationEvaluatedRef {
                    ref_kind: "invariant_policy".to_string(),
                    artifact_ref: manifold.invariant_policy_refs.policy_path.clone(),
                },
            ],
            affected_citizens: vec![active_citizen.citizen_id.clone()],
            refusal_reason: "duplicate active citizen instance would violate identity continuity"
                .to_string(),
            source_error,
            result: RuntimeV2InvariantViolationResult {
                resulting_state: "transition_refused_state_unchanged".to_string(),
                blocked_before_commit: true,
                recovery_action: "retain_existing_active_index_and_record_violation".to_string(),
                trace_ref: "runtime_v2/traces/invariants/violation-0001.json".to_string(),
            },
        };
        artifact.validate()?;
        Ok(artifact)
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_INVARIANT_VIOLATION_SCHEMA {
            return Err(anyhow!(
                "unsupported Runtime v2 invariant violation schema '{}'",
                self.schema_version
            ));
        }
        normalize_id(
            self.violation_id.clone(),
            "invariant_violation.violation_id",
        )?;
        normalize_id(self.manifold_id.clone(), "invariant_violation.manifold_id")?;
        validate_relative_path(&self.artifact_path, "invariant_violation.artifact_path")?;
        validate_timestamp_marker(&self.detected_at_utc, "invariant_violation.detected_at_utc")?;
        validate_invariant_violation_severity(&self.severity)?;
        normalize_id(
            self.invariant_id.clone(),
            "invariant_violation.invariant_id",
        )?;
        normalize_id(
            self.invariant_owner_service_id.clone(),
            "invariant_violation.invariant_owner_service_id",
        )?;
        match self.policy_enforcement_mode.as_str() {
            "fail_closed_before_activation" | "report_only" => {}
            other => {
                return Err(anyhow!(
                    "unsupported invariant_violation.policy_enforcement_mode '{other}'"
                ))
            }
        }
        self.attempted_transition.validate()?;
        validate_invariant_violation_evaluated_refs(&self.evaluated_refs)?;
        if self.affected_citizens.is_empty() {
            return Err(anyhow!(
                "invariant_violation.affected_citizens must not be empty"
            ));
        }
        let mut seen = std::collections::BTreeSet::new();
        for citizen_id in &self.affected_citizens {
            normalize_id(citizen_id.clone(), "invariant_violation.affected_citizens")?;
            if !seen.insert(citizen_id.clone()) {
                return Err(anyhow!(
                    "invariant_violation.affected_citizens contains duplicate '{}'",
                    citizen_id
                ));
            }
        }
        validate_nonempty_text(&self.refusal_reason, "invariant_violation.refusal_reason")?;
        validate_nonempty_text(&self.source_error, "invariant_violation.source_error")?;
        self.result.validate()?;
        if !self.result.blocked_before_commit {
            return Err(anyhow!(
                "invariant violation artifacts must prove rejection before commit"
            ));
        }
        Ok(())
    }

    pub fn to_pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.validate()?;
        serde_json::to_vec_pretty(self).context("serialize Runtime v2 invariant violation")
    }

    pub fn write_to_root(&self, root: impl AsRef<Path>) -> Result<()> {
        write_relative(
            root.as_ref(),
            &self.artifact_path,
            self.to_pretty_json_bytes()?,
        )
    }
}

impl RuntimeV2InvariantViolationAttempt {
    pub fn validate(&self) -> Result<()> {
        normalize_id(self.actor.clone(), "invariant_violation.actor")?;
        normalize_id(
            self.attempted_action.clone(),
            "invariant_violation.attempted_action",
        )?;
        normalize_id(
            self.attempted_state.clone(),
            "invariant_violation.attempted_state",
        )?;
        validate_relative_path(
            &self.source_artifact_ref,
            "invariant_violation.source_artifact_ref",
        )
    }
}

impl RuntimeV2InvariantViolationEvaluatedRef {
    pub fn validate(&self) -> Result<()> {
        normalize_id(
            self.ref_kind.clone(),
            "invariant_violation.evaluated_ref_kind",
        )?;
        validate_relative_path(
            &self.artifact_ref,
            "invariant_violation.evaluated_artifact_ref",
        )
    }
}

impl RuntimeV2InvariantViolationResult {
    pub fn validate(&self) -> Result<()> {
        normalize_id(
            self.resulting_state.clone(),
            "invariant_violation.resulting_state",
        )?;
        normalize_id(
            self.recovery_action.clone(),
            "invariant_violation.recovery_action",
        )?;
        validate_relative_path(&self.trace_ref, "invariant_violation.trace_ref")
    }
}

impl RuntimeV2OperatorControlReport {
    pub fn prototype(
        manifold: &RuntimeV2ManifoldRoot,
        kernel: &RuntimeV2KernelLoopArtifacts,
        citizens: &RuntimeV2CitizenLifecycleArtifacts,
        snapshot: &RuntimeV2SnapshotAndRehydrationArtifacts,
        violation: &RuntimeV2InvariantViolationArtifact,
    ) -> Result<Self> {
        manifold.validate()?;
        kernel.validate()?;
        citizens.validate()?;
        snapshot.validate()?;
        violation.validate()?;
        if kernel.state.manifold_id != manifold.manifold_id
            || citizens.active_index.manifold_id != manifold.manifold_id
            || snapshot.snapshot.manifold_id != manifold.manifold_id
            || violation.manifold_id != manifold.manifold_id
        {
            return Err(anyhow!(
                "operator control inputs must share the same manifold id"
            ));
        }
        let active_state = RuntimeV2OperatorControlState::from_parts(
            "active",
            &kernel.state,
            citizens,
            manifold.snapshot_root.latest_snapshot_id.clone(),
        );
        let paused_state = RuntimeV2OperatorControlState {
            manifold_lifecycle_state: "paused".to_string(),
            kernel_loop_status: "operator_paused".to_string(),
            ..active_state.clone()
        };
        let snapshotting_state = RuntimeV2OperatorControlState::from_parts(
            "snapshotting",
            &kernel.state,
            citizens,
            Some(snapshot.snapshot.snapshot_id.clone()),
        );
        let terminated_state = RuntimeV2OperatorControlState {
            manifold_lifecycle_state: "terminated".to_string(),
            kernel_loop_status: "operator_terminated".to_string(),
            active_citizen_count: 0,
            pending_citizen_count: citizens.pending_index.citizens.len(),
            latest_snapshot_id: Some(snapshot.snapshot.snapshot_id.clone()),
            completed_through_event_sequence: kernel.state.completed_through_event_sequence + 6,
        };
        let report = Self {
            schema_version: RUNTIME_V2_OPERATOR_CONTROL_REPORT_SCHEMA.to_string(),
            report_id: "operator-report-0001".to_string(),
            manifold_id: manifold.manifold_id.clone(),
            artifact_path: "runtime_v2/operator/control_report.json".to_string(),
            generated_at_utc: "not_started".to_string(),
            control_interface_service_id: "operator_control_interface".to_string(),
            commands: vec![
                RuntimeV2OperatorCommandReport {
                    command: "inspect_manifold".to_string(),
                    requested_by: "operator.cli".to_string(),
                    affected_service: "operator_control_interface".to_string(),
                    pre_state: active_state.clone(),
                    post_state: active_state.clone(),
                    outcome: "allowed".to_string(),
                    trace_event_ref: "runtime_v2/traces/operator/inspect-manifold.json".to_string(),
                    reason: "reported bounded manifold lifecycle and kernel status".to_string(),
                },
                RuntimeV2OperatorCommandReport {
                    command: "inspect_citizens".to_string(),
                    requested_by: "operator.cli".to_string(),
                    affected_service: "operator_control_interface".to_string(),
                    pre_state: active_state.clone(),
                    post_state: active_state.clone(),
                    outcome: "allowed".to_string(),
                    trace_event_ref: "runtime_v2/traces/operator/inspect-citizens.json".to_string(),
                    reason: "reported active and pending provisional citizen counts".to_string(),
                },
                RuntimeV2OperatorCommandReport {
                    command: "pause_manifold".to_string(),
                    requested_by: "operator.cli".to_string(),
                    affected_service: "scheduler".to_string(),
                    pre_state: active_state.clone(),
                    post_state: paused_state.clone(),
                    outcome: "allowed".to_string(),
                    trace_event_ref: "runtime_v2/traces/operator/pause-manifold.json".to_string(),
                    reason: "scheduler accepts a bounded operator pause before new episodes"
                        .to_string(),
                },
                RuntimeV2OperatorCommandReport {
                    command: "resume_manifold".to_string(),
                    requested_by: "operator.cli".to_string(),
                    affected_service: "scheduler".to_string(),
                    pre_state: paused_state,
                    post_state: active_state.clone(),
                    outcome: "allowed".to_string(),
                    trace_event_ref: "runtime_v2/traces/operator/resume-manifold.json".to_string(),
                    reason: "invariant checks passed and the paused manifold may resume"
                        .to_string(),
                },
                RuntimeV2OperatorCommandReport {
                    command: "request_snapshot".to_string(),
                    requested_by: "operator.cli".to_string(),
                    affected_service: "snapshot_manager".to_string(),
                    pre_state: active_state.clone(),
                    post_state: snapshotting_state,
                    outcome: "allowed".to_string(),
                    trace_event_ref: "runtime_v2/traces/operator/request-snapshot.json".to_string(),
                    reason: "snapshot manager can seal a bounded snapshot after invariants pass"
                        .to_string(),
                },
                RuntimeV2OperatorCommandReport {
                    command: "inspect_last_failures".to_string(),
                    requested_by: "operator.cli".to_string(),
                    affected_service: "invariant_checker".to_string(),
                    pre_state: active_state.clone(),
                    post_state: active_state.clone(),
                    outcome: "allowed".to_string(),
                    trace_event_ref: violation.result.trace_ref.clone(),
                    reason: format!(
                        "latest blocking invariant failure is {}",
                        violation.violation_id
                    ),
                },
                RuntimeV2OperatorCommandReport {
                    command: "terminate_manifold".to_string(),
                    requested_by: "operator.cli".to_string(),
                    affected_service: "resource_ledger".to_string(),
                    pre_state: active_state,
                    post_state: terminated_state,
                    outcome: "allowed".to_string(),
                    trace_event_ref: "runtime_v2/traces/operator/terminate-manifold.json"
                        .to_string(),
                    reason: "resource ledger records bounded termination and release intent"
                        .to_string(),
                },
            ],
        };
        report.validate()?;
        Ok(report)
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_OPERATOR_CONTROL_REPORT_SCHEMA {
            return Err(anyhow!(
                "unsupported Runtime v2 operator control report schema '{}'",
                self.schema_version
            ));
        }
        normalize_id(self.report_id.clone(), "operator_control.report_id")?;
        normalize_id(self.manifold_id.clone(), "operator_control.manifold_id")?;
        validate_relative_path(&self.artifact_path, "operator_control.artifact_path")?;
        validate_timestamp_marker(&self.generated_at_utc, "operator_control.generated_at_utc")?;
        normalize_id(
            self.control_interface_service_id.clone(),
            "operator_control.control_interface_service_id",
        )?;
        if self.control_interface_service_id != "operator_control_interface" {
            return Err(anyhow!(
                "operator control report must be owned by operator_control_interface"
            ));
        }
        validate_operator_commands(&self.commands)?;
        Ok(())
    }

    pub fn to_pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.validate()?;
        serde_json::to_vec_pretty(self).context("serialize Runtime v2 operator control report")
    }

    pub fn write_to_root(&self, root: impl AsRef<Path>) -> Result<()> {
        write_relative(
            root.as_ref(),
            &self.artifact_path,
            self.to_pretty_json_bytes()?,
        )
    }
}

impl RuntimeV2OperatorControlState {
    fn from_parts(
        manifold_lifecycle_state: &str,
        kernel_state: &RuntimeV2KernelServiceState,
        citizens: &RuntimeV2CitizenLifecycleArtifacts,
        latest_snapshot_id: Option<String>,
    ) -> Self {
        Self {
            manifold_lifecycle_state: manifold_lifecycle_state.to_string(),
            kernel_loop_status: kernel_state.loop_status.clone(),
            active_citizen_count: citizens.active_index.citizens.len(),
            pending_citizen_count: citizens.pending_index.citizens.len(),
            latest_snapshot_id,
            completed_through_event_sequence: kernel_state.completed_through_event_sequence,
        }
    }

    pub fn validate(&self) -> Result<()> {
        validate_lifecycle_state(&self.manifold_lifecycle_state)?;
        normalize_id(
            self.kernel_loop_status.clone(),
            "operator_control.kernel_loop_status",
        )?;
        if let Some(snapshot_id) = &self.latest_snapshot_id {
            normalize_id(snapshot_id.clone(), "operator_control.latest_snapshot_id")?;
        }
        if self.completed_through_event_sequence == 0 {
            return Err(anyhow!(
                "operator_control.completed_through_event_sequence must be positive"
            ));
        }
        if self.manifold_lifecycle_state == "terminated" && self.active_citizen_count != 0 {
            return Err(anyhow!(
                "operator_control terminated state must not retain active citizens"
            ));
        }
        Ok(())
    }
}

impl RuntimeV2OperatorCommandReport {
    pub fn validate(&self) -> Result<()> {
        validate_operator_command(&self.command)?;
        normalize_id(self.requested_by.clone(), "operator_control.requested_by")?;
        normalize_id(
            self.affected_service.clone(),
            "operator_control.affected_service",
        )?;
        self.pre_state.validate()?;
        self.post_state.validate()?;
        validate_operator_outcome(&self.outcome)?;
        validate_relative_path(&self.trace_event_ref, "operator_control.trace_event_ref")?;
        validate_nonempty_text(&self.reason, "operator_control.reason")?;
        if self.outcome == "allowed" && self.pre_state == self.post_state {
            match self.command.as_str() {
                "inspect_manifold" | "inspect_citizens" | "inspect_last_failures" => {}
                _ => {
                    return Err(anyhow!(
                        "operator mutating control commands must change post_state when allowed"
                    ))
                }
            }
        }
        Ok(())
    }
}

impl RuntimeV2SecurityBoundaryProofPacket {
    pub fn refused_resume_without_invariant_prototype(
        manifold: &RuntimeV2ManifoldRoot,
        kernel: &RuntimeV2KernelLoopArtifacts,
        violation: &RuntimeV2InvariantViolationArtifact,
        operator_report: &RuntimeV2OperatorControlReport,
    ) -> Result<Self> {
        manifold.validate()?;
        kernel.validate()?;
        violation.validate()?;
        operator_report.validate()?;
        if kernel.state.manifold_id != manifold.manifold_id
            || violation.manifold_id != manifold.manifold_id
            || operator_report.manifold_id != manifold.manifold_id
        {
            return Err(anyhow!(
                "security boundary inputs must share the same manifold id"
            ));
        }
        let resume_command = operator_report
            .commands
            .iter()
            .find(|command| command.command == "resume_manifold")
            .ok_or_else(|| anyhow!("security boundary proof requires resume_manifold command"))?;
        let inspect_failures_command = operator_report
            .commands
            .iter()
            .find(|command| command.command == "inspect_last_failures")
            .ok_or_else(|| {
                anyhow!("security boundary proof requires inspect_last_failures command")
            })?;
        if inspect_failures_command.trace_event_ref != violation.result.trace_ref {
            return Err(anyhow!(
                "security boundary proof must use the latest invariant violation trace"
            ));
        }
        let proof = Self {
            schema_version: RUNTIME_V2_SECURITY_BOUNDARY_PROOF_SCHEMA.to_string(),
            proof_id: "security-boundary-proof-0001".to_string(),
            manifold_id: manifold.manifold_id.clone(),
            artifact_path: "runtime_v2/security_boundary/proof_packet.json".to_string(),
            generated_at_utc: "not_started".to_string(),
            boundary_service_id: operator_report.control_interface_service_id.clone(),
            attempt: RuntimeV2SecurityBoundaryAttempt {
                actor: resume_command.requested_by.clone(),
                attempted_action: "resume_manifold_without_fresh_invariant_pass".to_string(),
                requested_state: "active".to_string(),
                source_command_ref: operator_report.artifact_path.clone(),
            },
            evaluated_rules: vec![
                RuntimeV2SecurityBoundaryEvaluatedRule {
                    rule_id: "require_fresh_invariant_pass_before_resume".to_string(),
                    rule_kind: "operator_policy".to_string(),
                    source_ref: manifold.invariant_policy_refs.policy_path.clone(),
                    decision: "refuse".to_string(),
                },
                RuntimeV2SecurityBoundaryEvaluatedRule {
                    rule_id: violation.invariant_id.clone(),
                    rule_kind: "blocking_invariant".to_string(),
                    source_ref: violation.artifact_path.clone(),
                    decision: "blocking_failure_present".to_string(),
                },
                RuntimeV2SecurityBoundaryEvaluatedRule {
                    rule_id: "scheduler_resume_gate".to_string(),
                    rule_kind: "kernel_service_policy".to_string(),
                    source_ref: kernel.state.service_state_path.clone(),
                    decision: "keep_paused".to_string(),
                },
            ],
            related_artifacts: vec![
                manifold.artifact_path.clone(),
                kernel.state.service_state_path.clone(),
                violation.artifact_path.clone(),
                operator_report.artifact_path.clone(),
            ],
            result: RuntimeV2SecurityBoundaryResult {
                allowed: false,
                refusal_reason:
                    "resume refused because latest invariant evidence is blocking and no fresh pass is recorded"
                        .to_string(),
                resulting_state: resume_command.pre_state.clone(),
                trace_ref:
                    "runtime_v2/traces/security_boundary/refused-resume-without-invariant.json"
                        .to_string(),
                recovery_action: "remain_paused_and_require_invariant_checker_pass_before_resume"
                    .to_string(),
            },
        };
        proof.validate()?;
        Ok(proof)
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_SECURITY_BOUNDARY_PROOF_SCHEMA {
            return Err(anyhow!(
                "unsupported Runtime v2 security boundary proof schema '{}'",
                self.schema_version
            ));
        }
        normalize_id(self.proof_id.clone(), "security_boundary.proof_id")?;
        normalize_id(self.manifold_id.clone(), "security_boundary.manifold_id")?;
        validate_relative_path(&self.artifact_path, "security_boundary.artifact_path")?;
        validate_timestamp_marker(&self.generated_at_utc, "security_boundary.generated_at_utc")?;
        normalize_id(
            self.boundary_service_id.clone(),
            "security_boundary.boundary_service_id",
        )?;
        if self.boundary_service_id != "operator_control_interface" {
            return Err(anyhow!(
                "security boundary proof must pass through operator_control_interface"
            ));
        }
        self.attempt.validate()?;
        validate_security_boundary_rules(&self.evaluated_rules)?;
        validate_security_boundary_related_artifacts(&self.related_artifacts)?;
        self.result.validate()?;
        if self.result.allowed {
            return Err(anyhow!(
                "security boundary proof must demonstrate a refused invalid action"
            ));
        }
        Ok(())
    }

    pub fn to_pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.validate()?;
        serde_json::to_vec_pretty(self).context("serialize Runtime v2 security boundary proof")
    }

    pub fn write_to_root(&self, root: impl AsRef<Path>) -> Result<()> {
        write_relative(
            root.as_ref(),
            &self.artifact_path,
            self.to_pretty_json_bytes()?,
        )
    }
}

impl RuntimeV2SecurityBoundaryAttempt {
    pub fn validate(&self) -> Result<()> {
        normalize_id(self.actor.clone(), "security_boundary.actor")?;
        normalize_id(
            self.attempted_action.clone(),
            "security_boundary.attempted_action",
        )?;
        validate_lifecycle_state(&self.requested_state)?;
        validate_relative_path(
            &self.source_command_ref,
            "security_boundary.source_command_ref",
        )
    }
}

impl RuntimeV2SecurityBoundaryEvaluatedRule {
    pub fn validate(&self) -> Result<()> {
        normalize_id(self.rule_id.clone(), "security_boundary.rule_id")?;
        validate_security_boundary_rule_kind(&self.rule_kind)?;
        validate_relative_path(&self.source_ref, "security_boundary.source_ref")?;
        validate_security_boundary_decision(&self.decision)
    }
}

impl RuntimeV2SecurityBoundaryResult {
    pub fn validate(&self) -> Result<()> {
        validate_nonempty_text(&self.refusal_reason, "security_boundary.refusal_reason")?;
        self.resulting_state.validate()?;
        validate_relative_path(&self.trace_ref, "security_boundary.trace_ref")?;
        normalize_id(
            self.recovery_action.clone(),
            "security_boundary.recovery_action",
        )?;
        if self.allowed {
            return Err(anyhow!(
                "security boundary result must be refused for this proof"
            ));
        }
        if self.resulting_state.manifold_lifecycle_state != "paused" {
            return Err(anyhow!(
                "security boundary refused resume must leave the manifold paused"
            ));
        }
        Ok(())
    }
}

impl RuntimeV2FoundationPrototypeArtifacts {
    pub fn prototype() -> Result<Self> {
        let manifold = runtime_v2_manifold_contract()?;
        let kernel = RuntimeV2KernelLoopArtifacts::prototype(&manifold)?;
        let citizens = RuntimeV2CitizenLifecycleArtifacts::prototype(&manifold)?;
        let snapshot =
            RuntimeV2SnapshotAndRehydrationArtifacts::prototype(&manifold, &kernel, &citizens)?;
        let invariant_violation =
            RuntimeV2InvariantViolationArtifact::duplicate_active_citizen_prototype(
                &manifold, &kernel, &citizens,
            )?;
        let operator_report = RuntimeV2OperatorControlReport::prototype(
            &manifold,
            &kernel,
            &citizens,
            &snapshot,
            &invariant_violation,
        )?;
        let security_boundary =
            RuntimeV2SecurityBoundaryProofPacket::refused_resume_without_invariant_prototype(
                &manifold,
                &kernel,
                &invariant_violation,
                &operator_report,
            )?;
        let proof_packet = RuntimeV2FoundationProofPacket::prototype(
            &manifold,
            &kernel,
            &citizens,
            &snapshot,
            &invariant_violation,
            &operator_report,
            &security_boundary,
        )?;

        let artifacts = Self {
            manifold,
            kernel,
            citizens,
            snapshot,
            invariant_violation,
            operator_report,
            security_boundary,
            proof_packet,
        };
        artifacts.validate()?;
        Ok(artifacts)
    }

    pub fn validate(&self) -> Result<()> {
        self.manifold.validate()?;
        self.kernel.validate()?;
        self.citizens.validate()?;
        self.snapshot.validate()?;
        self.invariant_violation.validate()?;
        self.operator_report.validate()?;
        self.security_boundary.validate()?;
        self.proof_packet.validate()?;

        let manifold_id = &self.manifold.manifold_id;
        for (name, actual) in [
            ("kernel", &self.kernel.registry.manifold_id),
            ("citizens", &self.citizens.active_index.manifold_id),
            ("snapshot", &self.snapshot.snapshot.manifold_id),
            ("invariant_violation", &self.invariant_violation.manifold_id),
            ("operator_report", &self.operator_report.manifold_id),
            ("security_boundary", &self.security_boundary.manifold_id),
            ("proof_packet", &self.proof_packet.manifold_id),
        ] {
            if actual != manifold_id {
                return Err(anyhow!(
                    "Runtime v2 foundation artifact '{name}' is bound to manifold '{actual}' instead of '{manifold_id}'"
                ));
            }
        }
        Ok(())
    }

    pub fn write_to_root(&self, root: impl AsRef<Path>) -> Result<()> {
        let root = root.as_ref();
        self.validate()?;
        self.manifold
            .write_to_path(root.join(&self.manifold.artifact_path))?;
        self.kernel.write_to_root(root)?;
        self.citizens.write_to_root(root)?;
        self.snapshot.write_to_root(root)?;
        self.invariant_violation.write_to_root(root)?;
        self.operator_report.write_to_root(root)?;
        self.security_boundary.write_to_root(root)?;
        self.proof_packet.write_to_root(root)
    }
}

impl RuntimeV2FoundationProofPacket {
    pub fn prototype(
        manifold: &RuntimeV2ManifoldRoot,
        kernel: &RuntimeV2KernelLoopArtifacts,
        citizens: &RuntimeV2CitizenLifecycleArtifacts,
        snapshot: &RuntimeV2SnapshotAndRehydrationArtifacts,
        invariant_violation: &RuntimeV2InvariantViolationArtifact,
        operator_report: &RuntimeV2OperatorControlReport,
        security_boundary: &RuntimeV2SecurityBoundaryProofPacket,
    ) -> Result<Self> {
        let active_record = citizens
            .records
            .iter()
            .find(|record| record.lifecycle_state == "active")
            .ok_or_else(|| anyhow!("foundation prototype requires one active citizen record"))?;
        let pending_record = citizens
            .records
            .iter()
            .find(|record| record.lifecycle_state != "active")
            .ok_or_else(|| anyhow!("foundation prototype requires one pending citizen record"))?;

        let packet = Self {
            schema_version: RUNTIME_V2_FOUNDATION_PROOF_PACKET_SCHEMA.to_string(),
            proof_id: "runtime-v2-foundation-proof-0001".to_string(),
            demo_id: "D7".to_string(),
            demo_name: "runtime_v2_foundation_prototype".to_string(),
            classification: "proving".to_string(),
            manifold_id: manifold.manifold_id.clone(),
            artifact_path: "runtime_v2/proof_packet.json".to_string(),
            generated_at_utc: "not_started".to_string(),
            integrated_artifacts: vec![
                RuntimeV2FoundationArtifactRef {
                    artifact_id: "manifold_root".to_string(),
                    artifact_kind: "manifold".to_string(),
                    schema_version: manifold.schema_version.clone(),
                    path: manifold.artifact_path.clone(),
                    source_wp: "WP-05".to_string(),
                    proves: "bounded persistent manifold root exists".to_string(),
                },
                RuntimeV2FoundationArtifactRef {
                    artifact_id: "kernel_service_registry".to_string(),
                    artifact_kind: "kernel_registry".to_string(),
                    schema_version: kernel.registry.schema_version.clone(),
                    path: kernel.registry.registry_path.clone(),
                    source_wp: "WP-06".to_string(),
                    proves: "kernel services are registered in deterministic activation order"
                        .to_string(),
                },
                RuntimeV2FoundationArtifactRef {
                    artifact_id: "kernel_service_state".to_string(),
                    artifact_kind: "kernel_state".to_string(),
                    schema_version: kernel.state.schema_version.clone(),
                    path: kernel.state.service_state_path.clone(),
                    source_wp: "WP-06".to_string(),
                    proves: "bounded kernel loop state is reviewable".to_string(),
                },
                RuntimeV2FoundationArtifactRef {
                    artifact_id: "active_citizen_record".to_string(),
                    artifact_kind: "citizen_record".to_string(),
                    schema_version: active_record.schema_version.clone(),
                    path: active_record.record_path.clone(),
                    source_wp: "WP-07".to_string(),
                    proves: "one provisional citizen can be active without claiming true birth"
                        .to_string(),
                },
                RuntimeV2FoundationArtifactRef {
                    artifact_id: "pending_citizen_record".to_string(),
                    artifact_kind: "citizen_record".to_string(),
                    schema_version: pending_record.schema_version.clone(),
                    path: pending_record.record_path.clone(),
                    source_wp: "WP-07".to_string(),
                    proves: "one provisional citizen can remain pending without duplicate activation"
                        .to_string(),
                },
                RuntimeV2FoundationArtifactRef {
                    artifact_id: "snapshot_manifest".to_string(),
                    artifact_kind: "snapshot".to_string(),
                    schema_version: snapshot.snapshot.schema_version.clone(),
                    path: snapshot.snapshot.snapshot_path.clone(),
                    source_wp: "WP-08".to_string(),
                    proves: "manifold state can be snapshotted with invariant status".to_string(),
                },
                RuntimeV2FoundationArtifactRef {
                    artifact_id: "rehydration_report".to_string(),
                    artifact_kind: "rehydration".to_string(),
                    schema_version: snapshot.rehydration_report.schema_version.clone(),
                    path: snapshot.rehydration_report.report_path.clone(),
                    source_wp: "WP-08".to_string(),
                    proves: "wake can be refused when invariant preconditions are not satisfied"
                        .to_string(),
                },
                RuntimeV2FoundationArtifactRef {
                    artifact_id: "invariant_violation".to_string(),
                    artifact_kind: "invariant_violation".to_string(),
                    schema_version: invariant_violation.schema_version.clone(),
                    path: invariant_violation.artifact_path.clone(),
                    source_wp: "WP-09".to_string(),
                    proves: "illegal duplicate active citizen transition is rejected before commit"
                        .to_string(),
                },
                RuntimeV2FoundationArtifactRef {
                    artifact_id: "operator_control_report".to_string(),
                    artifact_kind: "operator_control".to_string(),
                    schema_version: operator_report.schema_version.clone(),
                    path: operator_report.artifact_path.clone(),
                    source_wp: "WP-10".to_string(),
                    proves: "inspect, pause, resume, snapshot, and termination commands are bounded"
                        .to_string(),
                },
                RuntimeV2FoundationArtifactRef {
                    artifact_id: "security_boundary_proof".to_string(),
                    artifact_kind: "security_boundary".to_string(),
                    schema_version: security_boundary.schema_version.clone(),
                    path: security_boundary.artifact_path.clone(),
                    source_wp: "WP-11".to_string(),
                    proves: "invalid resume is refused through operator and scheduler policy"
                        .to_string(),
                },
            ],
            proof_claims: vec![
                "Runtime v2 foundation prototype can be inspected end to end".to_string(),
                "All WP-05 through WP-11 proof surfaces are bound to one manifold id".to_string(),
                "The prototype records both happy-path continuity handles and blocked invalid action evidence".to_string(),
            ],
            non_claims: vec![
                "does not prove first true Godel-agent birth".to_string(),
                "does not prove full moral, emotional, or polis governance".to_string(),
                "does not prove live scheduling, cross-machine migration, or full security ecology"
                    .to_string(),
            ],
            checks: vec![
                RuntimeV2FoundationProofCheck {
                    check_id: "same_manifold_id".to_string(),
                    status: "pass".to_string(),
                    evidence_ref: manifold.artifact_path.clone(),
                },
                RuntimeV2FoundationProofCheck {
                    check_id: "one_active_one_pending_citizen".to_string(),
                    status: "pass".to_string(),
                    evidence_ref: citizens.active_index.index_path.clone(),
                },
                RuntimeV2FoundationProofCheck {
                    check_id: "snapshot_rehydration_linked".to_string(),
                    status: "pass".to_string(),
                    evidence_ref: snapshot.rehydration_report.report_path.clone(),
                },
                RuntimeV2FoundationProofCheck {
                    check_id: "invalid_action_refused".to_string(),
                    status: "pass".to_string(),
                    evidence_ref: security_boundary.artifact_path.clone(),
                },
            ],
            reviewer_entrypoint:
                "cargo run --manifest-path adl/Cargo.toml -- demo demo-l-v0901-runtime-v2-foundation --run --out artifacts/v0901 --no-open"
                    .to_string(),
            replay_command:
                "cargo run --manifest-path adl/Cargo.toml -- runtime-v2 foundation-demo --out artifacts/v0901/demo-l-v0901-runtime-v2-foundation"
                    .to_string(),
        };
        packet.validate()?;
        Ok(packet)
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_FOUNDATION_PROOF_PACKET_SCHEMA {
            return Err(anyhow!(
                "unsupported Runtime v2 foundation proof schema '{}'",
                self.schema_version
            ));
        }
        normalize_id(self.proof_id.clone(), "foundation.proof_id")?;
        if self.demo_id != "D7" {
            return Err(anyhow!(
                "Runtime v2 foundation demo must map to demo matrix row D7"
            ));
        }
        if self.demo_name != "runtime_v2_foundation_prototype" {
            return Err(anyhow!(
                "Runtime v2 foundation demo name must remain runtime_v2_foundation_prototype"
            ));
        }
        if self.classification != "proving" {
            return Err(anyhow!(
                "Runtime v2 foundation proof packet must classify the demo as proving"
            ));
        }
        normalize_id(self.manifold_id.clone(), "foundation.manifold_id")?;
        validate_relative_path(&self.artifact_path, "foundation.artifact_path")?;
        validate_timestamp_marker(&self.generated_at_utc, "foundation.generated_at_utc")?;
        validate_foundation_artifact_refs(&self.integrated_artifacts)?;
        validate_foundation_checks(&self.checks)?;
        if self.proof_claims.len() < 3 || self.non_claims.len() < 3 {
            return Err(anyhow!(
                "Runtime v2 foundation proof packet must include explicit claims and non-claims"
            ));
        }
        validate_nonempty_text(&self.reviewer_entrypoint, "foundation.reviewer_entrypoint")?;
        validate_nonempty_text(&self.replay_command, "foundation.replay_command")?;
        Ok(())
    }

    pub fn to_pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.validate()?;
        serde_json::to_vec_pretty(self).context("serialize Runtime v2 foundation proof packet")
    }

    pub fn write_to_root(&self, root: impl AsRef<Path>) -> Result<()> {
        write_relative(
            root.as_ref(),
            &self.artifact_path,
            self.to_pretty_json_bytes()?,
        )
    }
}

impl RuntimeV2ProvisionalCitizenRecord {
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_PROVISIONAL_CITIZEN_SCHEMA {
            return Err(anyhow!(
                "unsupported Runtime v2 provisional citizen schema '{}'",
                self.schema_version
            ));
        }
        normalize_id(self.citizen_id.clone(), "citizen.citizen_id")?;
        validate_display_name(&self.display_name, "citizen.display_name")?;
        validate_provisional_status(&self.provisional_status)?;
        validate_citizen_lifecycle_state(&self.lifecycle_state)?;
        normalize_id(self.manifold_id.clone(), "citizen.manifold_id")?;
        validate_relative_path(&self.record_path, "citizen.record_path")?;
        validate_timestamp_marker(&self.created_at_utc, "citizen.created_at_utc")?;
        if let Some(last_wake) = &self.last_wake_at_utc {
            validate_timestamp_marker(last_wake, "citizen.last_wake_at_utc")?;
        }
        self.memory_identity_refs.validate()?;
        self.policy_boundary_refs.validate()?;
        if let Some(rehydration_ref) = &self.rehydration_validation_ref {
            validate_relative_path(rehydration_ref, "citizen.rehydration_validation_ref")?;
        }
        if let Some(termination_ref) = &self.termination_event_ref {
            validate_relative_path(termination_ref, "citizen.termination_event_ref")?;
        }
        let lifecycle_can_execute = self.lifecycle_state == "active";
        if self.can_execute_episodes != lifecycle_can_execute {
            return Err(anyhow!(
                "citizen.can_execute_episodes must be true only for active citizens"
            ));
        }
        if self.lifecycle_state == "waking" && self.rehydration_validation_ref.is_none() {
            return Err(anyhow!(
                "waking citizens must record rehydration validation before execution"
            ));
        }
        if self.resources_released && self.termination_event_ref.is_none() {
            return Err(anyhow!(
                "citizen resources cannot be released before termination is recorded"
            ));
        }
        if self.lifecycle_state == "rejected" && self.provisional_status != "rejected" {
            return Err(anyhow!(
                "rejected citizen lifecycle must use rejected provisional_status"
            ));
        }
        Ok(())
    }
}

impl RuntimeV2CitizenMemoryIdentityRefs {
    pub fn validate(&self) -> Result<()> {
        validate_relative_path(&self.memory_root_ref, "citizen.memory_root_ref")?;
        validate_relative_path(&self.identity_profile_ref, "citizen.identity_profile_ref")
    }
}

impl RuntimeV2CitizenPolicyBoundaryRefs {
    pub fn validate(&self) -> Result<()> {
        validate_relative_path(&self.policy_ref, "citizen.policy_ref")?;
        validate_relative_path(&self.admission_trace_ref, "citizen.admission_trace_ref")
    }
}

impl RuntimeV2CitizenRegistryEntry {
    fn from_record(record: &RuntimeV2ProvisionalCitizenRecord) -> Self {
        Self {
            citizen_id: record.citizen_id.clone(),
            lifecycle_state: record.lifecycle_state.clone(),
            record_path: record.record_path.clone(),
            can_execute_episodes: record.can_execute_episodes,
        }
    }

    pub fn validate(&self) -> Result<()> {
        normalize_id(self.citizen_id.clone(), "citizen_index.citizen_id")?;
        validate_citizen_lifecycle_state(&self.lifecycle_state)?;
        validate_relative_path(&self.record_path, "citizen_index.record_path")?;
        if self.can_execute_episodes != (self.lifecycle_state == "active") {
            return Err(anyhow!(
                "citizen index can_execute_episodes must match lifecycle state"
            ));
        }
        Ok(())
    }
}

impl RuntimeV2CitizenRegistryIndex {
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_CITIZEN_REGISTRY_INDEX_SCHEMA {
            return Err(anyhow!(
                "unsupported Runtime v2 citizen registry index schema '{}'",
                self.schema_version
            ));
        }
        normalize_id(self.manifold_id.clone(), "citizen_index.manifold_id")?;
        validate_relative_path(&self.registry_root, "citizen_index.registry_root")?;
        validate_citizen_index_kind(&self.index_kind)?;
        validate_relative_path(&self.index_path, "citizen_index.index_path")?;
        if self.index_kind == "active" && self.citizens.is_empty() {
            return Err(anyhow!(
                "citizen_index.citizens must not be empty for active index"
            ));
        }
        let mut ids = std::collections::BTreeSet::new();
        for entry in &self.citizens {
            entry.validate()?;
            if !ids.insert(entry.citizen_id.clone()) {
                return Err(anyhow!(
                    "citizen_index.citizens contains duplicate citizen '{}'",
                    entry.citizen_id
                ));
            }
            if self.index_kind == "active" && entry.lifecycle_state != "active" {
                return Err(anyhow!(
                    "citizen active index must contain only active citizens"
                ));
            }
            if self.index_kind == "pending" && entry.lifecycle_state == "active" {
                return Err(anyhow!(
                    "citizen pending index must not contain active citizens"
                ));
            }
        }
        Ok(())
    }
}

impl RuntimeV2KernelServiceRegistry {
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_KERNEL_SERVICE_REGISTRY_SCHEMA {
            return Err(anyhow!(
                "unsupported Runtime v2 kernel service registry schema '{}'",
                self.schema_version
            ));
        }
        normalize_id(self.manifold_id.clone(), "kernel_registry.manifold_id")?;
        validate_relative_path(&self.registry_path, "kernel_registry.registry_path")?;
        if self.services.is_empty() {
            return Err(anyhow!("kernel_registry.services must not be empty"));
        }
        let mut seen = std::collections::BTreeSet::new();
        for (index, service) in self.services.iter().enumerate() {
            service.validate()?;
            if !seen.insert(service.service_id.clone()) {
                return Err(anyhow!(
                    "kernel_registry.services contains duplicate service '{}'",
                    service.service_id
                ));
            }
            if service.activation_order != index as u64 + 1 {
                return Err(anyhow!(
                    "kernel_registry.services activation_order must be contiguous"
                ));
            }
        }
        validate_required_kernel_services(&self.services)
    }
}

impl RuntimeV2KernelServiceRegistration {
    pub fn validate(&self) -> Result<()> {
        normalize_id(self.service_id.clone(), "kernel_service.service_id")?;
        normalize_id(self.service_kind.clone(), "kernel_service.service_kind")?;
        validate_service_lifecycle_state(&self.lifecycle_state, "kernel_service.lifecycle_state")?;
        if self.activation_order == 0 {
            return Err(anyhow!("kernel_service.activation_order must be positive"));
        }
        validate_relative_path(
            &self.owns_artifact_path,
            "kernel_service.owns_artifact_path",
        )
    }
}

impl RuntimeV2KernelServiceState {
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_KERNEL_SERVICE_STATE_SCHEMA {
            return Err(anyhow!(
                "unsupported Runtime v2 kernel service state schema '{}'",
                self.schema_version
            ));
        }
        normalize_id(self.manifold_id.clone(), "kernel_state.manifold_id")?;
        validate_relative_path(&self.service_state_path, "kernel_state.service_state_path")?;
        normalize_id(self.loop_status.clone(), "kernel_state.loop_status")?;
        if self.completed_through_event_sequence == 0 {
            return Err(anyhow!(
                "kernel_state.completed_through_event_sequence must be positive"
            ));
        }
        if self.services.is_empty() {
            return Err(anyhow!("kernel_state.services must not be empty"));
        }
        for service in &self.services {
            service.validate()?;
        }
        Ok(())
    }
}

impl RuntimeV2KernelServiceStatus {
    pub fn validate(&self) -> Result<()> {
        normalize_id(self.service_id.clone(), "kernel_service_status.service_id")?;
        validate_service_lifecycle_state(
            &self.lifecycle_state,
            "kernel_service_status.lifecycle_state",
        )?;
        if self.last_event_sequence == 0 {
            return Err(anyhow!(
                "kernel_service_status.last_event_sequence must be positive"
            ));
        }
        if let Some(reason) = &self.blocked_reason {
            if reason.trim().is_empty() {
                return Err(anyhow!(
                    "kernel_service_status.blocked_reason must not be empty when present"
                ));
            }
        }
        Ok(())
    }
}

impl RuntimeV2KernelLoopEvent {
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_KERNEL_LOOP_EVENT_SCHEMA {
            return Err(anyhow!(
                "unsupported Runtime v2 kernel loop event schema '{}'",
                self.schema_version
            ));
        }
        if self.event_sequence == 0 {
            return Err(anyhow!("kernel_loop_event.event_sequence must be positive"));
        }
        normalize_id(self.manifold_id.clone(), "kernel_loop_event.manifold_id")?;
        normalize_id(self.service_id.clone(), "kernel_loop_event.service_id")?;
        normalize_id(self.action.clone(), "kernel_loop_event.action")?;
        match self.outcome.as_str() {
            "observed_ready" | "deferred" | "refused" | "blocked" => {}
            other => return Err(anyhow!("unsupported kernel_loop_event.outcome '{other}'")),
        }
        validate_relative_path(&self.artifact_ref, "kernel_loop_event.artifact_ref")
    }
}

fn normalize_id(value: String, field: &str) -> Result<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(anyhow!("{field} must not be empty"));
    }
    if trimmed.contains('/') || trimmed.contains('\\') || trimmed.contains(':') {
        return Err(anyhow!("{field} must be a stable identifier, not a path"));
    }
    Ok(trimmed.to_string())
}

fn validate_lifecycle_state(value: &str) -> Result<()> {
    match value {
        "initialized" | "active" | "paused" | "snapshotting" | "rehydrating" | "terminated" => {
            Ok(())
        }
        other => Err(anyhow!("unsupported manifold lifecycle_state '{other}'")),
    }
}

fn validate_clock_anchor(anchor: &ManifoldClockAnchor) -> Result<()> {
    normalize_id(anchor.anchor_id.clone(), "clock_anchor.anchor_id")?;
    match anchor.clock_kind.as_str() {
        "monotonic_logical" | "wall_clock_bound" => {}
        other => return Err(anyhow!("unsupported clock_anchor.clock_kind '{other}'")),
    }
    let observed = anchor.observed_at_utc.trim();
    if observed.is_empty() {
        return Err(anyhow!("clock_anchor.observed_at_utc must not be empty"));
    }
    Ok(())
}

fn validate_registry_refs(refs: &CitizenRegistryRefs) -> Result<()> {
    validate_relative_path(&refs.registry_root, "citizen_registry_refs.registry_root")?;
    validate_relative_path(&refs.active_index, "citizen_registry_refs.active_index")?;
    validate_relative_path(&refs.pending_index, "citizen_registry_refs.pending_index")
}

fn validate_kernel_refs(refs: &KernelServiceRefs) -> Result<()> {
    validate_relative_path(&refs.registry_path, "kernel_service_refs.registry_path")?;
    validate_relative_path(
        &refs.service_loop_path,
        "kernel_service_refs.service_loop_path",
    )?;
    validate_relative_path(
        &refs.service_state_path,
        "kernel_service_refs.service_state_path",
    )
}

fn validate_trace_root(trace_root: &TraceRootRef) -> Result<()> {
    validate_relative_path(&trace_root.trace_root, "trace_root.trace_root")?;
    validate_relative_path(&trace_root.event_log_path, "trace_root.event_log_path")?;
    if trace_root.next_event_sequence == 0 {
        return Err(anyhow!("trace_root.next_event_sequence must be positive"));
    }
    Ok(())
}

fn validate_snapshot_root(snapshot_root: &SnapshotRootRef) -> Result<()> {
    validate_relative_path(&snapshot_root.snapshot_root, "snapshot_root.snapshot_root")?;
    if let Some(id) = &snapshot_root.latest_snapshot_id {
        normalize_id(id.clone(), "snapshot_root.latest_snapshot_id")?;
    }
    validate_relative_path(
        &snapshot_root.rehydration_report_path,
        "snapshot_root.rehydration_report_path",
    )
}

fn validate_invariant_policy_refs(refs: &InvariantPolicyRefs) -> Result<()> {
    validate_relative_path(&refs.policy_path, "invariant_policy_refs.policy_path")?;
    match refs.enforcement_mode.as_str() {
        "fail_closed_before_activation" | "report_only" => {}
        other => {
            return Err(anyhow!(
                "unsupported invariant_policy_refs.enforcement_mode '{other}'"
            ))
        }
    }
    if refs.blocking_invariants.is_empty() {
        return Err(anyhow!(
            "invariant_policy_refs.blocking_invariants must not be empty"
        ));
    }
    for invariant in &refs.blocking_invariants {
        normalize_id(
            invariant.clone(),
            "invariant_policy_refs.blocking_invariants",
        )?;
    }
    Ok(())
}

fn validate_review_surface(surface: &RuntimeV2ManifoldReviewSurface) -> Result<()> {
    if surface.required_artifacts.is_empty() {
        return Err(anyhow!(
            "review_surface.required_artifacts must not be empty"
        ));
    }
    for path in &surface.required_artifacts {
        validate_relative_path(path, "review_surface.required_artifacts")?;
    }
    if surface.proof_hook_command.trim().is_empty() {
        return Err(anyhow!(
            "review_surface.proof_hook_command must not be empty"
        ));
    }
    validate_relative_path(
        &surface.proof_hook_output_path,
        "review_surface.proof_hook_output_path",
    )?;
    if surface.downstream_boundaries.is_empty() {
        return Err(anyhow!(
            "review_surface.downstream_boundaries must name later WP boundaries"
        ));
    }
    if surface.non_goals.is_empty() {
        return Err(anyhow!("review_surface.non_goals must not be empty"));
    }
    Ok(())
}

fn validate_relative_path(value: &str, field: &str) -> Result<()> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(anyhow!("{field} must not be empty"));
    }
    if trimmed.starts_with('/')
        || trimmed.starts_with('\\')
        || trimmed.contains('\\')
        || trimmed.contains(':')
    {
        return Err(anyhow!("{field} must be a repository-relative path"));
    }
    for component in Path::new(trimmed).components() {
        use std::path::Component;
        match component {
            Component::Normal(_) => {}
            Component::CurDir => {}
            _ => return Err(anyhow!("{field} must not traverse outside the repo")),
        }
    }
    Ok(())
}

fn validate_service_lifecycle_state(value: &str, field: &str) -> Result<()> {
    match value {
        "registered" | "ready" | "paused" | "blocked" | "terminated" => Ok(()),
        other => Err(anyhow!("unsupported {field} '{other}'")),
    }
}

fn validate_provisional_status(value: &str) -> Result<()> {
    match value {
        "provisional" | "admitted" | "rejected" => Ok(()),
        other => Err(anyhow!("unsupported citizen.provisional_status '{other}'")),
    }
}

fn validate_citizen_lifecycle_state(value: &str) -> Result<()> {
    match value {
        "proposed" | "admitted" | "active" | "paused" | "sleeping" | "waking" | "terminated"
        | "rejected" => Ok(()),
        other => Err(anyhow!("unsupported citizen.lifecycle_state '{other}'")),
    }
}

fn validate_citizen_index_kind(value: &str) -> Result<()> {
    match value {
        "active" | "pending" => Ok(()),
        other => Err(anyhow!("unsupported citizen_index.index_kind '{other}'")),
    }
}

fn validate_snapshot_invariant_statuses(
    statuses: &[RuntimeV2SnapshotInvariantStatus],
) -> Result<()> {
    if statuses.is_empty() {
        return Err(anyhow!("snapshot invariant_status must not be empty"));
    }
    let mut seen = std::collections::BTreeSet::new();
    for status in statuses {
        normalize_id(status.invariant_id.clone(), "snapshot.invariant_id")?;
        match status.status.as_str() {
            "passed" | "failed" | "not_checked" => {}
            other => return Err(anyhow!("unsupported snapshot invariant status '{other}'")),
        }
        if !seen.insert(status.invariant_id.clone()) {
            return Err(anyhow!(
                "snapshot invariant_status contains duplicate invariant '{}'",
                status.invariant_id
            ));
        }
    }
    Ok(())
}

fn validate_invariant_violation_severity(value: &str) -> Result<()> {
    match value {
        "blocking" | "warning" | "audit" => Ok(()),
        other => Err(anyhow!(
            "unsupported invariant_violation.severity '{other}'"
        )),
    }
}

fn validate_invariant_violation_evaluated_refs(
    refs: &[RuntimeV2InvariantViolationEvaluatedRef],
) -> Result<()> {
    if refs.is_empty() {
        return Err(anyhow!(
            "invariant_violation.evaluated_refs must not be empty"
        ));
    }
    let mut seen = std::collections::BTreeSet::new();
    for evaluated_ref in refs {
        evaluated_ref.validate()?;
        let key = format!("{}:{}", evaluated_ref.ref_kind, evaluated_ref.artifact_ref);
        if !seen.insert(key) {
            return Err(anyhow!(
                "invariant_violation.evaluated_refs contains duplicate ref"
            ));
        }
    }
    Ok(())
}

fn validate_operator_commands(commands: &[RuntimeV2OperatorCommandReport]) -> Result<()> {
    let required = [
        "inspect_manifold",
        "inspect_citizens",
        "pause_manifold",
        "resume_manifold",
        "request_snapshot",
        "inspect_last_failures",
        "terminate_manifold",
    ];
    if commands.len() != required.len() {
        return Err(anyhow!(
            "operator_control.commands must cover each bounded operator command exactly once"
        ));
    }
    let mut seen = std::collections::BTreeSet::new();
    for (expected, command) in required.iter().zip(commands.iter()) {
        command.validate()?;
        if command.command != *expected {
            return Err(anyhow!(
                "operator_control.commands must preserve deterministic command order"
            ));
        }
        if !seen.insert(command.command.clone()) {
            return Err(anyhow!(
                "operator_control.commands contains duplicate command '{}'",
                command.command
            ));
        }
    }
    Ok(())
}

fn validate_operator_command(value: &str) -> Result<()> {
    match value {
        "inspect_manifold"
        | "inspect_citizens"
        | "pause_manifold"
        | "resume_manifold"
        | "request_snapshot"
        | "inspect_last_failures"
        | "terminate_manifold" => Ok(()),
        other => Err(anyhow!("unsupported operator_control.command '{other}'")),
    }
}

fn validate_operator_outcome(value: &str) -> Result<()> {
    match value {
        "allowed" | "refused" | "deferred" => Ok(()),
        other => Err(anyhow!("unsupported operator_control.outcome '{other}'")),
    }
}

fn validate_security_boundary_rules(
    rules: &[RuntimeV2SecurityBoundaryEvaluatedRule],
) -> Result<()> {
    if rules.len() < 3 {
        return Err(anyhow!(
            "security_boundary.evaluated_rules must include operator, invariant, and kernel checks"
        ));
    }
    let mut seen = std::collections::BTreeSet::new();
    let mut has_operator = false;
    let mut has_invariant = false;
    let mut has_kernel = false;
    for rule in rules {
        rule.validate()?;
        if !seen.insert(rule.rule_id.clone()) {
            return Err(anyhow!(
                "security_boundary.evaluated_rules contains duplicate rule '{}'",
                rule.rule_id
            ));
        }
        match rule.rule_kind.as_str() {
            "operator_policy" => has_operator = true,
            "blocking_invariant" => has_invariant = true,
            "kernel_service_policy" => has_kernel = true,
            _ => {}
        }
    }
    if !(has_operator && has_invariant && has_kernel) {
        return Err(anyhow!(
            "security_boundary.evaluated_rules missing required policy/invariant/kernel coverage"
        ));
    }
    Ok(())
}

fn validate_security_boundary_rule_kind(value: &str) -> Result<()> {
    match value {
        "operator_policy" | "blocking_invariant" | "kernel_service_policy" => Ok(()),
        other => Err(anyhow!("unsupported security_boundary.rule_kind '{other}'")),
    }
}

fn validate_security_boundary_decision(value: &str) -> Result<()> {
    match value {
        "refuse" | "blocking_failure_present" | "keep_paused" => Ok(()),
        other => Err(anyhow!("unsupported security_boundary.decision '{other}'")),
    }
}

fn validate_security_boundary_related_artifacts(artifacts: &[String]) -> Result<()> {
    if artifacts.is_empty() {
        return Err(anyhow!(
            "security_boundary.related_artifacts must not be empty"
        ));
    }
    let mut seen = std::collections::BTreeSet::new();
    for artifact in artifacts {
        validate_relative_path(artifact, "security_boundary.related_artifacts")?;
        if !seen.insert(artifact.clone()) {
            return Err(anyhow!(
                "security_boundary.related_artifacts contains duplicate artifact"
            ));
        }
    }
    if !artifacts
        .iter()
        .any(|artifact| artifact == "runtime_v2/invariants/violation-0001.json")
    {
        return Err(anyhow!(
            "security_boundary.related_artifacts must include invariant violation evidence"
        ));
    }
    if !artifacts
        .iter()
        .any(|artifact| artifact == "runtime_v2/operator/control_report.json")
    {
        return Err(anyhow!(
            "security_boundary.related_artifacts must include operator control evidence"
        ));
    }
    Ok(())
}

fn validate_foundation_artifact_refs(artifacts: &[RuntimeV2FoundationArtifactRef]) -> Result<()> {
    if artifacts.len() < 10 {
        return Err(anyhow!(
            "Runtime v2 foundation proof packet must integrate at least ten artifact refs"
        ));
    }
    let mut seen_ids = std::collections::BTreeSet::new();
    let mut seen_wps = std::collections::BTreeSet::new();
    for artifact in artifacts {
        normalize_id(artifact.artifact_id.clone(), "foundation.artifact_id")?;
        validate_foundation_artifact_kind(&artifact.artifact_kind)?;
        validate_relative_path(&artifact.path, "foundation.artifact_path")?;
        validate_nonempty_text(&artifact.schema_version, "foundation.artifact_schema")?;
        validate_nonempty_text(&artifact.source_wp, "foundation.source_wp")?;
        validate_nonempty_text(&artifact.proves, "foundation.proves")?;
        if !seen_ids.insert(artifact.artifact_id.clone()) {
            return Err(anyhow!(
                "Runtime v2 foundation proof packet contains duplicate artifact id '{}'",
                artifact.artifact_id
            ));
        }
        seen_wps.insert(artifact.source_wp.clone());
    }

    for required_wp in [
        "WP-05", "WP-06", "WP-07", "WP-08", "WP-09", "WP-10", "WP-11",
    ] {
        if !seen_wps.contains(required_wp) {
            return Err(anyhow!(
                "Runtime v2 foundation proof packet missing required source {required_wp}"
            ));
        }
    }
    Ok(())
}

fn validate_foundation_artifact_kind(value: &str) -> Result<()> {
    match value {
        "manifold"
        | "kernel_registry"
        | "kernel_state"
        | "citizen_record"
        | "snapshot"
        | "rehydration"
        | "invariant_violation"
        | "operator_control"
        | "security_boundary" => Ok(()),
        other => Err(anyhow!("unsupported foundation artifact kind '{other}'")),
    }
}

fn validate_foundation_checks(checks: &[RuntimeV2FoundationProofCheck]) -> Result<()> {
    let mut seen = std::collections::BTreeSet::new();
    for check in checks {
        normalize_id(check.check_id.clone(), "foundation.check_id")?;
        if check.status != "pass" {
            return Err(anyhow!(
                "Runtime v2 foundation proof check '{}' must pass",
                check.check_id
            ));
        }
        validate_relative_path(&check.evidence_ref, "foundation.check_evidence_ref")?;
        if !seen.insert(check.check_id.clone()) {
            return Err(anyhow!(
                "Runtime v2 foundation proof packet contains duplicate check '{}'",
                check.check_id
            ));
        }
    }
    for required_check in [
        "same_manifold_id",
        "one_active_one_pending_citizen",
        "snapshot_rehydration_linked",
        "invalid_action_refused",
    ] {
        if !seen.contains(required_check) {
            return Err(anyhow!(
                "Runtime v2 foundation proof packet missing required check {required_check}"
            ));
        }
    }
    Ok(())
}

fn validate_nonempty_text(value: &str, field: &str) -> Result<()> {
    if value.trim().is_empty() {
        return Err(anyhow!("{field} must not be empty"));
    }
    Ok(())
}

fn validate_display_name(value: &str, field: &str) -> Result<()> {
    validate_nonempty_text(value, field)
}

fn validate_timestamp_marker(value: &str, field: &str) -> Result<()> {
    validate_nonempty_text(value, field)
}

fn validate_required_kernel_services(
    services: &[RuntimeV2KernelServiceRegistration],
) -> Result<()> {
    let required = [
        "clock_service",
        "identity_admission_guard",
        "scheduler",
        "resource_ledger",
        "trace_writer",
        "snapshot_manager",
        "invariant_checker",
        "operator_control_interface",
    ];
    for required_service in required {
        if !services
            .iter()
            .any(|service| service.service_id == required_service)
        {
            return Err(anyhow!(
                "kernel_registry.services missing required service '{required_service}'"
            ));
        }
    }
    Ok(())
}

fn write_relative(root: &Path, rel_path: &str, bytes: Vec<u8>) -> Result<()> {
    validate_relative_path(rel_path, "runtime_v2.write_relative")?;
    let path = root.join(rel_path);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("failed to create '{}'", parent.display()))?;
    }
    std::fs::write(&path, bytes).with_context(|| format!("failed to write '{}'", path.display()))
}

fn checksum_for_serialize(value: &impl Serialize) -> Result<String> {
    let bytes = serde_json::to_vec(value).context("serialize Runtime v2 checksum input")?;
    let mut checksum = 0xcbf29ce484222325_u64;
    for byte in bytes {
        checksum ^= u64::from(byte);
        checksum = checksum.wrapping_mul(0x100000001b3);
    }
    Ok(format!("fnv1a64:{checksum:016x}"))
}

fn prototype_kernel_services() -> Vec<RuntimeV2KernelServiceRegistration> {
    [
        (
            "clock_service",
            "clock",
            "runtime_v2/kernel/clock_service.json",
        ),
        (
            "identity_admission_guard",
            "admission",
            "runtime_v2/kernel/admission_guard.json",
        ),
        ("scheduler", "scheduler", "runtime_v2/kernel/scheduler.json"),
        (
            "resource_ledger",
            "resource",
            "runtime_v2/resource_ledger.json",
        ),
        ("trace_writer", "trace", "runtime_v2/traces/events.jsonl"),
        ("snapshot_manager", "snapshot", "runtime_v2/snapshots"),
        (
            "invariant_checker",
            "invariant",
            "runtime_v2/invariants/policy.json",
        ),
        (
            "operator_control_interface",
            "operator",
            "runtime_v2/operator/control_report.json",
        ),
    ]
    .into_iter()
    .enumerate()
    .map(|(index, (service_id, service_kind, owns_artifact_path))| {
        RuntimeV2KernelServiceRegistration {
            service_id: service_id.to_string(),
            service_kind: service_kind.to_string(),
            lifecycle_state: "registered".to_string(),
            activation_order: index as u64 + 1,
            owns_artifact_path: owns_artifact_path.to_string(),
        }
    })
    .collect()
}

pub fn runtime_v2_manifold_contract() -> Result<RuntimeV2ManifoldRoot> {
    RuntimeV2ManifoldRoot::prototype("proto-csm-01")
}

pub fn runtime_v2_kernel_loop_contract() -> Result<RuntimeV2KernelLoopArtifacts> {
    RuntimeV2KernelLoopArtifacts::prototype(&runtime_v2_manifold_contract()?)
}

pub fn runtime_v2_citizen_lifecycle_contract() -> Result<RuntimeV2CitizenLifecycleArtifacts> {
    RuntimeV2CitizenLifecycleArtifacts::prototype(&runtime_v2_manifold_contract()?)
}

pub fn runtime_v2_snapshot_rehydration_contract() -> Result<RuntimeV2SnapshotAndRehydrationArtifacts>
{
    let manifold = runtime_v2_manifold_contract()?;
    let kernel = RuntimeV2KernelLoopArtifacts::prototype(&manifold)?;
    let citizens = RuntimeV2CitizenLifecycleArtifacts::prototype(&manifold)?;
    RuntimeV2SnapshotAndRehydrationArtifacts::prototype(&manifold, &kernel, &citizens)
}

pub fn runtime_v2_invariant_violation_contract() -> Result<RuntimeV2InvariantViolationArtifact> {
    let manifold = runtime_v2_manifold_contract()?;
    let kernel = RuntimeV2KernelLoopArtifacts::prototype(&manifold)?;
    let citizens = RuntimeV2CitizenLifecycleArtifacts::prototype(&manifold)?;
    RuntimeV2InvariantViolationArtifact::duplicate_active_citizen_prototype(
        &manifold, &kernel, &citizens,
    )
}

pub fn runtime_v2_operator_control_report_contract() -> Result<RuntimeV2OperatorControlReport> {
    let manifold = runtime_v2_manifold_contract()?;
    let kernel = RuntimeV2KernelLoopArtifacts::prototype(&manifold)?;
    let citizens = RuntimeV2CitizenLifecycleArtifacts::prototype(&manifold)?;
    let snapshot =
        RuntimeV2SnapshotAndRehydrationArtifacts::prototype(&manifold, &kernel, &citizens)?;
    let violation = RuntimeV2InvariantViolationArtifact::duplicate_active_citizen_prototype(
        &manifold, &kernel, &citizens,
    )?;
    RuntimeV2OperatorControlReport::prototype(&manifold, &kernel, &citizens, &snapshot, &violation)
}

pub fn runtime_v2_security_boundary_proof_contract() -> Result<RuntimeV2SecurityBoundaryProofPacket>
{
    let manifold = runtime_v2_manifold_contract()?;
    let kernel = RuntimeV2KernelLoopArtifacts::prototype(&manifold)?;
    let citizens = RuntimeV2CitizenLifecycleArtifacts::prototype(&manifold)?;
    let snapshot =
        RuntimeV2SnapshotAndRehydrationArtifacts::prototype(&manifold, &kernel, &citizens)?;
    let violation = RuntimeV2InvariantViolationArtifact::duplicate_active_citizen_prototype(
        &manifold, &kernel, &citizens,
    )?;
    let operator_report = RuntimeV2OperatorControlReport::prototype(
        &manifold, &kernel, &citizens, &snapshot, &violation,
    )?;
    RuntimeV2SecurityBoundaryProofPacket::refused_resume_without_invariant_prototype(
        &manifold,
        &kernel,
        &violation,
        &operator_report,
    )
}

pub fn runtime_v2_foundation_demo_contract() -> Result<RuntimeV2FoundationPrototypeArtifacts> {
    RuntimeV2FoundationPrototypeArtifacts::prototype()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        env, fs,
        time::{SystemTime, UNIX_EPOCH},
    };

    fn unique_temp_path(label: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time went backwards")
            .as_nanos();
        env::temp_dir().join(format!("runtime-v2-{label}-{}-{nanos}", std::process::id()))
    }

    #[test]
    fn runtime_v2_manifold_root_contract_is_stable() {
        let root = runtime_v2_manifold_contract().expect("contract");
        root.validate().expect("valid manifold root");

        assert_eq!(root.schema_version, RUNTIME_V2_MANIFOLD_SCHEMA);
        assert_eq!(root.manifold_id, "proto-csm-01");
        assert_eq!(root.lifecycle_state, "initialized");
        assert_eq!(root.artifact_path, DEFAULT_MANIFOLD_ARTIFACT_PATH);
        assert_eq!(root.clock_anchor.monotonic_tick, 0);
        assert_eq!(root.trace_root.next_event_sequence, 1);
        assert_eq!(root.snapshot_root.latest_snapshot_id, None);
        assert!(root
            .invariant_policy_refs
            .blocking_invariants
            .contains(&"single_active_manifold_instance".to_string()));
        assert!(root
            .review_surface
            .downstream_boundaries
            .iter()
            .any(|boundary| boundary.contains("WP-06")));
    }

    #[test]
    fn runtime_v2_manifold_root_round_trips_without_path_leakage() {
        let temp_root = unique_temp_path("roundtrip");
        let path = temp_root.join(DEFAULT_MANIFOLD_ARTIFACT_PATH);
        let root = runtime_v2_manifold_contract().expect("contract");

        root.write_to_path(&path).expect("write manifest");
        let loaded = RuntimeV2ManifoldRoot::read_from_path(&path).expect("read manifest");
        assert_eq!(loaded, root);

        let text = fs::read_to_string(&path).expect("manifest text");
        assert!(text.contains("\"schema_version\": \"runtime_v2.manifold.v1\""));
        assert!(text.contains("\"artifact_path\": \"runtime_v2/manifold.json\""));
        assert!(!text.contains(temp_root.to_string_lossy().as_ref()));

        fs::remove_dir_all(temp_root).ok();
    }

    #[test]
    fn runtime_v2_manifold_root_matches_golden_manifest_fixture() {
        let root = runtime_v2_manifold_contract().expect("contract");
        let generated =
            String::from_utf8(root.to_pretty_json_bytes().expect("json")).expect("utf8");
        let expected = include_str!("../tests/fixtures/runtime_v2/manifold.json");

        assert_eq!(generated, expected.trim_end());
    }

    #[test]
    fn runtime_v2_manifold_validation_rejects_unsafe_or_ambiguous_roots() {
        let mut root = runtime_v2_manifold_contract().expect("contract");
        root.manifold_id = " ".to_string();
        assert!(root
            .validate()
            .expect_err("empty id should fail")
            .to_string()
            .contains("manifold_id must not be empty"));

        let mut root = runtime_v2_manifold_contract().expect("contract");
        root.artifact_path = "/tmp/runtime_v2/manifold.json".to_string();
        assert!(root
            .validate()
            .expect_err("absolute path should fail")
            .to_string()
            .contains("artifact_path must be a repository-relative path"));

        let mut root = runtime_v2_manifold_contract().expect("contract");
        root.trace_root.next_event_sequence = 0;
        assert!(root
            .validate()
            .expect_err("zero sequence should fail")
            .to_string()
            .contains("trace_root.next_event_sequence must be positive"));
    }

    #[test]
    fn runtime_v2_manifold_root_does_not_claim_later_wp_outputs() {
        let root = runtime_v2_manifold_contract().expect("contract");
        let json = String::from_utf8(root.to_pretty_json_bytes().expect("json")).expect("utf8");

        assert!(json.contains("WP-07 owns provisional citizen record materialization"));
        assert!(json.contains("WP-08 owns snapshot writing, sealing, and rehydration"));
        assert!(json.contains("no true Godel-agent birthday"));
        assert!(!json.contains("citizen_id"));
        assert!(!json.contains("snapshot_hash"));
        assert!(!json.contains("kernel_tick_completed"));
    }

    #[test]
    fn runtime_v2_kernel_loop_contract_matches_manifold_refs() {
        let manifold = runtime_v2_manifold_contract().expect("manifold");
        let loop_artifacts = RuntimeV2KernelLoopArtifacts::prototype(&manifold).expect("loop");

        assert_eq!(
            loop_artifacts.registry.registry_path,
            manifold.kernel_service_refs.registry_path
        );
        assert_eq!(
            loop_artifacts.service_loop_path,
            manifold.kernel_service_refs.service_loop_path
        );
        assert_eq!(
            loop_artifacts.state.service_state_path,
            manifold.kernel_service_refs.service_state_path
        );
        assert_eq!(loop_artifacts.registry.services.len(), 8);
        assert_eq!(loop_artifacts.events.len(), 8);
        assert_eq!(
            loop_artifacts.events[0].event_sequence,
            manifold.trace_root.next_event_sequence
        );
        assert_eq!(loop_artifacts.state.loop_status, "bounded_tick_complete");
        assert!(loop_artifacts
            .registry
            .services
            .iter()
            .any(|service| service.service_id == "operator_control_interface"));
    }

    #[test]
    fn runtime_v2_kernel_loop_artifacts_match_golden_fixtures() {
        let loop_artifacts = runtime_v2_kernel_loop_contract().expect("loop");
        let registry = String::from_utf8(
            loop_artifacts
                .registry_pretty_json_bytes()
                .expect("registry json"),
        )
        .expect("utf8 registry");
        let state = String::from_utf8(
            loop_artifacts
                .state_pretty_json_bytes()
                .expect("state json"),
        )
        .expect("utf8 state");
        let loop_jsonl = String::from_utf8(
            loop_artifacts
                .service_loop_jsonl_bytes()
                .expect("loop jsonl"),
        )
        .expect("utf8 loop jsonl");

        assert_eq!(
            registry,
            include_str!("../tests/fixtures/runtime_v2/kernel/service_registry.json").trim_end()
        );
        assert_eq!(
            state,
            include_str!("../tests/fixtures/runtime_v2/kernel/service_state.json").trim_end()
        );
        assert_eq!(
            loop_jsonl,
            include_str!("../tests/fixtures/runtime_v2/kernel/service_loop.jsonl")
        );
    }

    #[test]
    fn runtime_v2_kernel_loop_writes_artifacts_without_path_leakage() {
        let temp_root = unique_temp_path("kernel-loop");
        let loop_artifacts = runtime_v2_kernel_loop_contract().expect("loop");

        loop_artifacts
            .write_to_root(&temp_root)
            .expect("write loop artifacts");

        let registry_path = temp_root.join(&loop_artifacts.registry.registry_path);
        let state_path = temp_root.join(&loop_artifacts.state.service_state_path);
        let loop_path = temp_root.join(&loop_artifacts.service_loop_path);
        assert!(registry_path.is_file());
        assert!(state_path.is_file());
        assert!(loop_path.is_file());

        let registry = fs::read_to_string(registry_path).expect("registry text");
        let state = fs::read_to_string(state_path).expect("state text");
        let loop_jsonl = fs::read_to_string(loop_path).expect("loop text");
        let temp_root_text = temp_root.to_string_lossy();
        assert!(!registry.contains(temp_root_text.as_ref()));
        assert!(!state.contains(temp_root_text.as_ref()));
        assert!(!loop_jsonl.contains(temp_root_text.as_ref()));
        assert_eq!(loop_jsonl.lines().count(), 8);

        fs::remove_dir_all(temp_root).ok();
    }

    #[test]
    fn runtime_v2_kernel_loop_validation_rejects_unsafe_or_ambiguous_state() {
        let mut loop_artifacts = runtime_v2_kernel_loop_contract().expect("loop");
        loop_artifacts.registry.services[1].service_id = "clock_service".to_string();
        assert!(loop_artifacts
            .validate()
            .expect_err("duplicate service should fail")
            .to_string()
            .contains("duplicate service"));

        let mut loop_artifacts = runtime_v2_kernel_loop_contract().expect("loop");
        loop_artifacts.events[1].event_sequence = 10;
        assert!(loop_artifacts
            .validate()
            .expect_err("non-contiguous event order should fail")
            .to_string()
            .contains("contiguous"));

        let mut loop_artifacts = runtime_v2_kernel_loop_contract().expect("loop");
        loop_artifacts.service_loop_path = "/tmp/service_loop.jsonl".to_string();
        assert!(loop_artifacts
            .validate()
            .expect_err("absolute path should fail")
            .to_string()
            .contains("repository-relative path"));
    }

    #[test]
    fn runtime_v2_citizen_lifecycle_contract_matches_manifold_refs() {
        let manifold = runtime_v2_manifold_contract().expect("manifold");
        let citizens = RuntimeV2CitizenLifecycleArtifacts::prototype(&manifold).expect("citizens");

        assert_eq!(citizens.active_index.manifold_id, manifold.manifold_id);
        assert_eq!(citizens.pending_index.manifold_id, manifold.manifold_id);
        assert_eq!(
            citizens.active_index.registry_root,
            manifold.citizen_registry_refs.registry_root
        );
        assert_eq!(
            citizens.active_index.index_path,
            manifold.citizen_registry_refs.active_index
        );
        assert_eq!(
            citizens.pending_index.index_path,
            manifold.citizen_registry_refs.pending_index
        );
        assert_eq!(citizens.records.len(), 2);
        assert_eq!(citizens.active_index.citizens.len(), 1);
        assert_eq!(citizens.pending_index.citizens.len(), 1);
        assert!(citizens
            .records
            .iter()
            .any(|record| record.lifecycle_state == "active" && record.can_execute_episodes));
        assert!(citizens
            .records
            .iter()
            .any(|record| record.lifecycle_state == "proposed" && !record.can_execute_episodes));
    }

    #[test]
    fn runtime_v2_citizen_lifecycle_artifacts_match_golden_fixtures() {
        let citizens = runtime_v2_citizen_lifecycle_contract().expect("citizens");
        let alpha = citizens
            .records
            .iter()
            .find(|record| record.citizen_id == "proto-citizen-alpha")
            .expect("alpha");
        let beta = citizens
            .records
            .iter()
            .find(|record| record.citizen_id == "proto-citizen-beta")
            .expect("beta");
        let alpha_json = String::from_utf8(
            RuntimeV2CitizenLifecycleArtifacts::record_pretty_json_bytes(alpha)
                .expect("alpha json"),
        )
        .expect("utf8 alpha");
        let beta_json = String::from_utf8(
            RuntimeV2CitizenLifecycleArtifacts::record_pretty_json_bytes(beta).expect("beta json"),
        )
        .expect("utf8 beta");
        let active_index_json = String::from_utf8(
            RuntimeV2CitizenLifecycleArtifacts::index_pretty_json_bytes(&citizens.active_index)
                .expect("active index json"),
        )
        .expect("utf8 active index");
        let pending_index_json = String::from_utf8(
            RuntimeV2CitizenLifecycleArtifacts::index_pretty_json_bytes(&citizens.pending_index)
                .expect("pending index json"),
        )
        .expect("utf8 pending index");

        assert_eq!(
            alpha_json,
            include_str!("../tests/fixtures/runtime_v2/citizens/proto-citizen-alpha.json")
                .trim_end()
        );
        assert_eq!(
            beta_json,
            include_str!("../tests/fixtures/runtime_v2/citizens/proto-citizen-beta.json")
                .trim_end()
        );
        assert_eq!(
            active_index_json,
            include_str!("../tests/fixtures/runtime_v2/citizens/active_index.json").trim_end()
        );
        assert_eq!(
            pending_index_json,
            include_str!("../tests/fixtures/runtime_v2/citizens/pending_index.json").trim_end()
        );
    }

    #[test]
    fn runtime_v2_citizen_lifecycle_writes_artifacts_without_path_leakage() {
        let temp_root = unique_temp_path("citizens");
        let citizens = runtime_v2_citizen_lifecycle_contract().expect("citizens");

        citizens
            .write_to_root(&temp_root)
            .expect("write citizen artifacts");

        let alpha_path = temp_root.join(&citizens.records[0].record_path);
        let beta_path = temp_root.join(&citizens.records[1].record_path);
        let active_index_path = temp_root.join(&citizens.active_index.index_path);
        let pending_index_path = temp_root.join(&citizens.pending_index.index_path);
        assert!(alpha_path.is_file());
        assert!(beta_path.is_file());
        assert!(active_index_path.is_file());
        assert!(pending_index_path.is_file());

        let alpha = fs::read_to_string(alpha_path).expect("alpha text");
        let index = fs::read_to_string(active_index_path).expect("index text");
        let temp_root_text = temp_root.to_string_lossy();
        assert!(!alpha.contains(temp_root_text.as_ref()));
        assert!(!index.contains(temp_root_text.as_ref()));
        assert!(index.contains("\"index_kind\": \"active\""));
        assert!(index.contains("\"citizens\""));

        fs::remove_dir_all(temp_root).ok();
    }

    #[test]
    fn runtime_v2_citizen_lifecycle_validation_rejects_unsafe_or_ambiguous_state() {
        let mut citizens = runtime_v2_citizen_lifecycle_contract().expect("citizens");
        citizens.records[1].citizen_id = "proto-citizen-alpha".to_string();
        assert!(citizens
            .validate()
            .expect_err("duplicate citizen should fail")
            .to_string()
            .contains("duplicate citizen"));

        let mut citizens = runtime_v2_citizen_lifecycle_contract().expect("citizens");
        citizens.records[1].lifecycle_state = "paused".to_string();
        citizens.records[1].can_execute_episodes = true;
        assert!(citizens
            .validate()
            .expect_err("inactive executor should fail")
            .to_string()
            .contains("true only for active citizens"));

        let mut citizens = runtime_v2_citizen_lifecycle_contract().expect("citizens");
        citizens.records[1].lifecycle_state = "waking".to_string();
        assert!(citizens
            .validate()
            .expect_err("waking without rehydration proof should fail")
            .to_string()
            .contains("rehydration validation"));

        let mut citizens = runtime_v2_citizen_lifecycle_contract().expect("citizens");
        citizens.records[1].resources_released = true;
        assert!(citizens
            .validate()
            .expect_err("resource release without termination proof should fail")
            .to_string()
            .contains("before termination is recorded"));
    }

    #[test]
    fn runtime_v2_snapshot_rehydration_contract_matches_upstream_refs() {
        let manifold = runtime_v2_manifold_contract().expect("manifold");
        let kernel = RuntimeV2KernelLoopArtifacts::prototype(&manifold).expect("kernel");
        let citizens = RuntimeV2CitizenLifecycleArtifacts::prototype(&manifold).expect("citizens");
        let artifacts =
            RuntimeV2SnapshotAndRehydrationArtifacts::prototype(&manifold, &kernel, &citizens)
                .expect("snapshot");

        assert_eq!(
            artifacts.snapshot.schema_version,
            RUNTIME_V2_SNAPSHOT_MANIFEST_SCHEMA
        );
        assert_eq!(
            artifacts.rehydration_report.schema_version,
            RUNTIME_V2_REHYDRATION_REPORT_SCHEMA
        );
        assert_eq!(artifacts.snapshot.manifold_id, manifold.manifold_id);
        assert_eq!(
            artifacts.snapshot.last_trace_cursor,
            kernel.state.completed_through_event_sequence
        );
        assert_eq!(artifacts.snapshot.citizen_records, citizens.records);
        assert_eq!(artifacts.snapshot.active_index, citizens.active_index);
        assert_eq!(
            artifacts.rehydration_report.trace_resume_sequence,
            artifacts.snapshot.last_trace_cursor + 1
        );
        assert!(artifacts.rehydration_report.wake_allowed);
    }

    #[test]
    fn runtime_v2_snapshot_rehydration_artifacts_match_golden_fixtures() {
        let artifacts = runtime_v2_snapshot_rehydration_contract().expect("snapshot");
        let snapshot = String::from_utf8(
            artifacts
                .snapshot_pretty_json_bytes()
                .expect("snapshot json"),
        )
        .expect("utf8 snapshot");
        let rehydration = String::from_utf8(
            artifacts
                .rehydration_report_pretty_json_bytes()
                .expect("rehydration json"),
        )
        .expect("utf8 rehydration");

        assert_eq!(
            snapshot,
            include_str!("../tests/fixtures/runtime_v2/snapshots/snapshot-0001.json").trim_end()
        );
        assert_eq!(
            rehydration,
            include_str!("../tests/fixtures/runtime_v2/rehydration_report.json").trim_end()
        );
    }

    #[test]
    fn runtime_v2_snapshot_rehydration_writes_artifacts_without_path_leakage() {
        let temp_root = unique_temp_path("snapshot");
        let artifacts = runtime_v2_snapshot_rehydration_contract().expect("snapshot");

        artifacts
            .write_to_root(&temp_root)
            .expect("write snapshot artifacts");

        let snapshot_path = temp_root.join(&artifacts.snapshot.snapshot_path);
        let report_path = temp_root.join(&artifacts.rehydration_report.report_path);
        assert!(snapshot_path.is_file());
        assert!(report_path.is_file());

        let snapshot = fs::read_to_string(snapshot_path).expect("snapshot text");
        let report = fs::read_to_string(report_path).expect("report text");
        let temp_root_text = temp_root.to_string_lossy();
        assert!(!snapshot.contains(temp_root_text.as_ref()));
        assert!(!report.contains(temp_root_text.as_ref()));
        assert!(snapshot.contains("\"structural_checksum\": \"fnv1a64:"));
        assert!(report.contains("\"wake_allowed\": true"));

        fs::remove_dir_all(temp_root).ok();
    }

    #[test]
    fn runtime_v2_snapshot_rehydration_validation_rejects_unsafe_or_ambiguous_state() {
        let mut artifacts = runtime_v2_snapshot_rehydration_contract().expect("snapshot");
        artifacts.snapshot.structural_checksum = "fnv1a64:0000000000000000".to_string();
        assert!(artifacts
            .validate()
            .expect_err("checksum drift should fail")
            .to_string()
            .contains("checksum mismatch"));

        let mut artifacts = runtime_v2_snapshot_rehydration_contract().expect("snapshot");
        artifacts.rehydration_report.restored_manifold_id = "other-manifold".to_string();
        assert!(artifacts
            .validate()
            .expect_err("wrong restored manifold should fail")
            .to_string()
            .contains("restored manifold id"));

        let mut artifacts = runtime_v2_snapshot_rehydration_contract().expect("snapshot");
        artifacts.rehydration_report.trace_resume_sequence = artifacts.snapshot.last_trace_cursor;
        assert!(artifacts
            .validate()
            .expect_err("non-advancing trace should fail")
            .to_string()
            .contains("resume after the snapshot cursor"));

        let mut artifacts = runtime_v2_snapshot_rehydration_contract().expect("snapshot");
        artifacts
            .rehydration_report
            .restored_active_citizens
            .push("proto-citizen-alpha".to_string());
        assert!(artifacts
            .validate()
            .expect_err("duplicate active citizen should fail")
            .to_string()
            .contains("duplicate"));

        let mut artifacts = runtime_v2_snapshot_rehydration_contract().expect("snapshot");
        artifacts.snapshot.invariant_status[0].status = "failed".to_string();
        artifacts.snapshot.structural_checksum = artifacts
            .snapshot
            .compute_structural_checksum()
            .expect("checksum");
        assert!(artifacts
            .validate()
            .expect_err("failed invariant should fail")
            .to_string()
            .contains("invariant checks must pass"));
    }

    #[test]
    fn runtime_v2_invariant_violation_contract_records_rejected_transition() {
        let manifold = runtime_v2_manifold_contract().expect("manifold");
        let kernel = RuntimeV2KernelLoopArtifacts::prototype(&manifold).expect("kernel");
        let citizens = RuntimeV2CitizenLifecycleArtifacts::prototype(&manifold).expect("citizens");
        let violation = RuntimeV2InvariantViolationArtifact::duplicate_active_citizen_prototype(
            &manifold, &kernel, &citizens,
        )
        .expect("violation");

        assert_eq!(
            violation.schema_version,
            RUNTIME_V2_INVARIANT_VIOLATION_SCHEMA
        );
        assert_eq!(violation.manifold_id, manifold.manifold_id);
        assert_eq!(
            violation.invariant_id,
            "no_duplicate_active_citizen_instance"
        );
        assert_eq!(violation.invariant_owner_service_id, "invariant_checker");
        assert_eq!(violation.severity, "blocking");
        assert_eq!(
            violation.policy_enforcement_mode,
            "fail_closed_before_activation"
        );
        assert_eq!(violation.affected_citizens, vec!["proto-citizen-alpha"]);
        assert!(violation.result.blocked_before_commit);
        assert!(violation.source_error.contains("duplicate citizen"));
        assert!(violation
            .evaluated_refs
            .iter()
            .any(|evaluated_ref| evaluated_ref.ref_kind == "kernel_state"));
    }

    #[test]
    fn runtime_v2_invariant_violation_artifact_matches_golden_fixture() {
        let violation = runtime_v2_invariant_violation_contract().expect("violation");
        let generated =
            String::from_utf8(violation.to_pretty_json_bytes().expect("json")).expect("utf8");

        assert_eq!(
            generated,
            include_str!("../tests/fixtures/runtime_v2/invariants/violation-0001.json").trim_end()
        );
    }

    #[test]
    fn runtime_v2_invariant_violation_writes_artifact_without_path_leakage() {
        let temp_root = unique_temp_path("invariant");
        let violation = runtime_v2_invariant_violation_contract().expect("violation");

        violation
            .write_to_root(&temp_root)
            .expect("write violation artifact");

        let violation_path = temp_root.join(&violation.artifact_path);
        assert!(violation_path.is_file());
        let text = fs::read_to_string(violation_path).expect("violation text");
        assert!(!text.contains(temp_root.to_string_lossy().as_ref()));
        assert!(text.contains("\"schema_version\": \"runtime_v2.invariant_violation.v1\""));
        assert!(text.contains("\"blocked_before_commit\": true"));

        fs::remove_dir_all(temp_root).ok();
    }

    #[test]
    fn runtime_v2_invariant_violation_validation_rejects_unsafe_or_ambiguous_state() {
        let mut violation = runtime_v2_invariant_violation_contract().expect("violation");
        violation.artifact_path = "/tmp/violation.json".to_string();
        assert!(violation
            .validate()
            .expect_err("absolute path should fail")
            .to_string()
            .contains("repository-relative path"));

        let mut violation = runtime_v2_invariant_violation_contract().expect("violation");
        violation.result.blocked_before_commit = false;
        assert!(violation
            .validate()
            .expect_err("unblocked violation should fail")
            .to_string()
            .contains("before commit"));

        let mut violation = runtime_v2_invariant_violation_contract().expect("violation");
        violation
            .evaluated_refs
            .push(violation.evaluated_refs[0].clone());
        assert!(violation
            .validate()
            .expect_err("duplicate evaluated refs should fail")
            .to_string()
            .contains("duplicate ref"));

        let mut violation = runtime_v2_invariant_violation_contract().expect("violation");
        violation.refusal_reason = " ".to_string();
        assert!(violation
            .validate()
            .expect_err("empty refusal reason should fail")
            .to_string()
            .contains("refusal_reason must not be empty"));
    }

    #[test]
    fn runtime_v2_operator_control_report_records_bounded_controls() {
        let report = runtime_v2_operator_control_report_contract().expect("operator report");

        assert_eq!(
            report.schema_version,
            RUNTIME_V2_OPERATOR_CONTROL_REPORT_SCHEMA
        );
        assert_eq!(report.manifold_id, "proto-csm-01");
        assert_eq!(
            report.control_interface_service_id,
            "operator_control_interface"
        );
        assert_eq!(report.commands.len(), 7);
        assert_eq!(report.commands[0].command, "inspect_manifold");
        assert_eq!(report.commands[2].command, "pause_manifold");
        assert_eq!(
            report.commands[2].post_state.manifold_lifecycle_state,
            "paused"
        );
        assert_eq!(report.commands[3].command, "resume_manifold");
        assert_eq!(
            report.commands[4].post_state.latest_snapshot_id.as_deref(),
            Some("snapshot-0001")
        );
        assert_eq!(report.commands[5].command, "inspect_last_failures");
        assert!(report.commands[5]
            .trace_event_ref
            .contains("violation-0001"));
        assert_eq!(
            report.commands[6].post_state.manifold_lifecycle_state,
            "terminated"
        );
        assert_eq!(report.commands[6].post_state.active_citizen_count, 0);
    }

    #[test]
    fn runtime_v2_operator_control_report_matches_golden_fixture() {
        let report = runtime_v2_operator_control_report_contract().expect("operator report");
        let generated =
            String::from_utf8(report.to_pretty_json_bytes().expect("json")).expect("utf8");

        assert_eq!(
            generated,
            include_str!("../tests/fixtures/runtime_v2/operator/control_report.json").trim_end()
        );
    }

    #[test]
    fn runtime_v2_operator_control_report_writes_without_path_leakage() {
        let temp_root = unique_temp_path("operator-controls");
        let report = runtime_v2_operator_control_report_contract().expect("operator report");

        report
            .write_to_root(&temp_root)
            .expect("write operator report");

        let report_path = temp_root.join(&report.artifact_path);
        assert!(report_path.is_file());
        let text = fs::read_to_string(report_path).expect("operator report text");
        assert!(!text.contains(temp_root.to_string_lossy().as_ref()));
        assert!(text.contains("\"schema_version\": \"runtime_v2.operator_control_report.v1\""));
        assert!(text.contains("\"command\": \"pause_manifold\""));
        assert!(text.contains("\"command\": \"terminate_manifold\""));

        fs::remove_dir_all(temp_root).ok();
    }

    #[test]
    fn runtime_v2_operator_control_validation_rejects_unsafe_or_ambiguous_state() {
        let mut report = runtime_v2_operator_control_report_contract().expect("operator report");
        report.artifact_path = "/tmp/operator/control_report.json".to_string();
        assert!(report
            .validate()
            .expect_err("absolute path should fail")
            .to_string()
            .contains("repository-relative path"));

        let mut report = runtime_v2_operator_control_report_contract().expect("operator report");
        report.commands[0].command = "inspect_citizens".to_string();
        assert!(report
            .validate()
            .expect_err("command order should fail")
            .to_string()
            .contains("deterministic command order"));

        let mut report = runtime_v2_operator_control_report_contract().expect("operator report");
        report.commands[2].post_state = report.commands[2].pre_state.clone();
        assert!(report
            .validate()
            .expect_err("mutating command with unchanged state should fail")
            .to_string()
            .contains("mutating control commands must change post_state"));

        let mut report = runtime_v2_operator_control_report_contract().expect("operator report");
        report.commands[6].post_state.active_citizen_count = 1;
        assert!(report
            .validate()
            .expect_err("terminated state with active citizens should fail")
            .to_string()
            .contains("terminated state must not retain active citizens"));
    }

    #[test]
    fn runtime_v2_security_boundary_proof_records_refused_invalid_action() {
        let proof = runtime_v2_security_boundary_proof_contract().expect("security proof");

        assert_eq!(
            proof.schema_version,
            RUNTIME_V2_SECURITY_BOUNDARY_PROOF_SCHEMA
        );
        assert_eq!(proof.manifold_id, "proto-csm-01");
        assert_eq!(proof.boundary_service_id, "operator_control_interface");
        assert_eq!(
            proof.attempt.attempted_action,
            "resume_manifold_without_fresh_invariant_pass"
        );
        assert!(!proof.result.allowed);
        assert_eq!(
            proof.result.resulting_state.manifold_lifecycle_state,
            "paused"
        );
        assert!(proof
            .evaluated_rules
            .iter()
            .any(|rule| rule.rule_kind == "blocking_invariant"));
        assert!(proof
            .related_artifacts
            .contains(&"runtime_v2/operator/control_report.json".to_string()));
        assert!(proof
            .related_artifacts
            .contains(&"runtime_v2/invariants/violation-0001.json".to_string()));
    }

    #[test]
    fn runtime_v2_security_boundary_proof_matches_golden_fixture() {
        let proof = runtime_v2_security_boundary_proof_contract().expect("security proof");
        let generated =
            String::from_utf8(proof.to_pretty_json_bytes().expect("json")).expect("utf8");

        assert_eq!(
            generated,
            include_str!("../tests/fixtures/runtime_v2/security_boundary/proof_packet.json")
                .trim_end()
        );
    }

    #[test]
    fn runtime_v2_security_boundary_proof_writes_without_path_leakage() {
        let temp_root = unique_temp_path("security-boundary");
        let proof = runtime_v2_security_boundary_proof_contract().expect("security proof");

        proof
            .write_to_root(&temp_root)
            .expect("write security proof");

        let proof_path = temp_root.join(&proof.artifact_path);
        assert!(proof_path.is_file());
        let text = fs::read_to_string(proof_path).expect("security proof text");
        assert!(!text.contains(temp_root.to_string_lossy().as_ref()));
        assert!(text.contains("\"schema_version\": \"runtime_v2.security_boundary_proof.v1\""));
        assert!(text.contains("\"allowed\": false"));
        assert!(text.contains("resume_manifold_without_fresh_invariant_pass"));

        fs::remove_dir_all(temp_root).ok();
    }

    #[test]
    fn runtime_v2_security_boundary_validation_rejects_unsafe_or_ambiguous_state() {
        let mut proof = runtime_v2_security_boundary_proof_contract().expect("security proof");
        proof.artifact_path = "/tmp/security/proof.json".to_string();
        assert!(proof
            .validate()
            .expect_err("absolute path should fail")
            .to_string()
            .contains("repository-relative path"));

        let mut proof = runtime_v2_security_boundary_proof_contract().expect("security proof");
        proof.result.allowed = true;
        assert!(proof
            .validate()
            .expect_err("allowed invalid action should fail")
            .to_string()
            .contains("must be refused"));

        let mut proof = runtime_v2_security_boundary_proof_contract().expect("security proof");
        proof.evaluated_rules.remove(1);
        assert!(proof
            .validate()
            .expect_err("missing invariant coverage should fail")
            .to_string()
            .contains("must include operator, invariant, and kernel checks"));

        let mut proof = runtime_v2_security_boundary_proof_contract().expect("security proof");
        proof
            .related_artifacts
            .retain(|artifact| artifact != "runtime_v2/operator/control_report.json");
        assert!(proof
            .validate()
            .expect_err("missing operator evidence should fail")
            .to_string()
            .contains("operator control evidence"));
    }

    #[test]
    fn runtime_v2_foundation_demo_contract_integrates_wp05_through_wp11() {
        let artifacts = runtime_v2_foundation_demo_contract().expect("foundation demo");
        assert_eq!(
            artifacts.proof_packet.schema_version,
            RUNTIME_V2_FOUNDATION_PROOF_PACKET_SCHEMA
        );
        assert_eq!(artifacts.proof_packet.demo_id, "D7");
        assert_eq!(artifacts.proof_packet.classification, "proving");
        assert_eq!(
            artifacts.proof_packet.manifold_id,
            artifacts.manifold.manifold_id
        );
        assert!(artifacts
            .proof_packet
            .integrated_artifacts
            .iter()
            .any(|artifact| artifact.source_wp == "WP-11"
                && artifact.path == "runtime_v2/security_boundary/proof_packet.json"));
        assert_eq!(artifacts.citizens.records.len(), 2);
        assert!(!artifacts.security_boundary.result.allowed);
    }

    #[test]
    fn runtime_v2_foundation_demo_matches_golden_fixture() {
        let artifacts = runtime_v2_foundation_demo_contract().expect("foundation demo");
        let json = String::from_utf8(artifacts.proof_packet.to_pretty_json_bytes().expect("json"))
            .expect("utf8");
        assert_eq!(
            json,
            include_str!("../tests/fixtures/runtime_v2/foundation/proof_packet.json").trim_end()
        );
    }

    #[test]
    fn runtime_v2_foundation_demo_writes_integrated_artifacts_without_path_leakage() {
        let temp_root = unique_temp_path("runtime-v2-foundation");
        let artifacts = runtime_v2_foundation_demo_contract().expect("foundation demo");
        artifacts
            .write_to_root(&temp_root)
            .expect("write foundation demo");

        for rel_path in [
            "runtime_v2/proof_packet.json",
            "runtime_v2/manifold.json",
            "runtime_v2/kernel/service_registry.json",
            "runtime_v2/kernel/service_state.json",
            "runtime_v2/kernel/service_loop.jsonl",
            "runtime_v2/citizens/proto-citizen-alpha.json",
            "runtime_v2/citizens/proto-citizen-beta.json",
            "runtime_v2/snapshots/snapshot-0001.json",
            "runtime_v2/rehydration_report.json",
            "runtime_v2/invariants/violation-0001.json",
            "runtime_v2/operator/control_report.json",
            "runtime_v2/security_boundary/proof_packet.json",
        ] {
            assert!(temp_root.join(rel_path).is_file(), "missing {rel_path}");
        }

        let proof_text =
            fs::read_to_string(temp_root.join("runtime_v2/proof_packet.json")).expect("proof");
        assert!(!proof_text.contains("/Users/"), "proof leaked host path");
        assert!(proof_text.contains("\"classification\": \"proving\""));
        fs::remove_dir_all(temp_root).ok();
    }

    #[test]
    fn runtime_v2_foundation_demo_validation_rejects_incomplete_or_ambiguous_packet() {
        let mut artifacts = runtime_v2_foundation_demo_contract().expect("foundation demo");
        artifacts.proof_packet.classification = "non_proving".to_string();
        assert!(artifacts.validate().is_err());

        let mut artifacts = runtime_v2_foundation_demo_contract().expect("foundation demo");
        artifacts
            .proof_packet
            .integrated_artifacts
            .retain(|artifact| artifact.source_wp != "WP-11");
        assert!(artifacts.validate().is_err());

        let mut artifacts = runtime_v2_foundation_demo_contract().expect("foundation demo");
        artifacts.proof_packet.checks[0].status = "fail".to_string();
        assert!(artifacts.validate().is_err());
    }
}
