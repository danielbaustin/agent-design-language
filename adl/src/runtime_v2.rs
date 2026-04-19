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
}
