//! Runtime-v2 snapshot and rehydration contracts.
//!
//! Captures canonical snapshot manifests and rehydration reports consumed by
//! wake/resume workflows and snapshot validation.

use super::*;
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

    pub(crate) fn compute_structural_checksum(&self) -> Result<String> {
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
