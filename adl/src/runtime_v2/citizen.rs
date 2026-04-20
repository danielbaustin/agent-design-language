use super::*;
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

