use std::path::{Path, PathBuf};

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};

pub const RUNTIME_V2_MANIFOLD_SCHEMA: &str = "runtime_v2.manifold.v1";
pub const RUNTIME_V2_KERNEL_SERVICE_REGISTRY_SCHEMA: &str = "runtime_v2.kernel.service_registry.v1";
pub const RUNTIME_V2_KERNEL_SERVICE_STATE_SCHEMA: &str = "runtime_v2.kernel.service_state.v1";
pub const RUNTIME_V2_KERNEL_LOOP_EVENT_SCHEMA: &str = "runtime_v2.kernel.service_loop_event.v1";
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
}
