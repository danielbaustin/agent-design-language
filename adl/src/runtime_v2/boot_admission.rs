use std::path::Path;

use super::*;

pub const RUNTIME_V2_CSM_BOOT_MANIFEST_SCHEMA: &str = "runtime_v2.csm_boot_manifest.v1";
pub const RUNTIME_V2_CSM_CITIZEN_ROSTER_SCHEMA: &str = "runtime_v2.csm_citizen_roster.v1";
pub const RUNTIME_V2_CSM_ADMISSION_TRACE_EVENT_SCHEMA: &str =
    "runtime_v2.csm_admission_trace_event.v1";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2CsmAdmissionReceipt {
    pub citizen_id: String,
    pub role: String,
    pub identity_handle: String,
    pub source_record_ref: String,
    pub admission_trace_ref: String,
    pub admission_status: String,
    pub can_execute_episodes: bool,
    pub boundary_note: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2CsmBootManifest {
    pub schema_version: String,
    pub boot_id: String,
    pub demo_id: String,
    pub manifold_id: String,
    pub artifact_path: String,
    pub generated_at_utc: String,
    pub run_packet_ref: String,
    pub invariant_map_ref: String,
    pub violation_schema_ref: String,
    pub citizen_roster_ref: String,
    pub admission_trace_ref: String,
    pub boot_state: String,
    pub admitted_citizens: Vec<RuntimeV2CsmAdmissionReceipt>,
    pub required_before_next_stage: Vec<String>,
    pub claim_boundary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2CsmCitizenRosterEntry {
    pub citizen_id: String,
    pub display_name: String,
    pub role: String,
    pub lifecycle_state_at_boot: String,
    pub identity_handle: String,
    pub record_ref: String,
    pub memory_root_ref: String,
    pub policy_ref: String,
    pub admission_trace_ref: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2CsmCitizenRoster {
    pub schema_version: String,
    pub roster_id: String,
    pub demo_id: String,
    pub manifold_id: String,
    pub artifact_path: String,
    pub boot_manifest_ref: String,
    pub entries: Vec<RuntimeV2CsmCitizenRosterEntry>,
    pub provisional_boundary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2CsmAdmissionTraceEvent {
    pub schema_version: String,
    pub event_sequence: u64,
    pub event_id: String,
    pub manifold_id: String,
    pub service_id: String,
    pub action: String,
    pub outcome: String,
    pub citizen_id: String,
    pub artifact_ref: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeV2CsmBootAdmissionArtifacts {
    pub boot_manifest: RuntimeV2CsmBootManifest,
    pub citizen_roster: RuntimeV2CsmCitizenRoster,
    pub admission_trace: Vec<RuntimeV2CsmAdmissionTraceEvent>,
    pub admission_trace_path: String,
}

impl RuntimeV2CsmBootAdmissionArtifacts {
    pub fn prototype() -> Result<Self> {
        let manifold = runtime_v2_manifold_contract()?;
        let csm_run = runtime_v2_csm_run_packet_contract()?;
        let invariant_contract = runtime_v2_invariant_and_violation_contract()?;
        let citizens = RuntimeV2CitizenLifecycleArtifacts::prototype(&manifold)?;
        Self::from_contracts(&manifold, &csm_run, &invariant_contract, &citizens)
    }

    pub fn from_contracts(
        manifold: &RuntimeV2ManifoldRoot,
        csm_run: &RuntimeV2CsmRunPacketContract,
        invariant_contract: &RuntimeV2InvariantAndViolationContractArtifacts,
        citizens: &RuntimeV2CitizenLifecycleArtifacts,
    ) -> Result<Self> {
        manifold.validate()?;
        csm_run.validate()?;
        invariant_contract.validate()?;
        citizens.validate()?;
        if csm_run.manifold_id != manifold.manifold_id
            || invariant_contract.invariant_map.manifold_id != manifold.manifold_id
            || citizens.active_index.manifold_id != manifold.manifold_id
        {
            return Err(anyhow!(
                "CSM boot/admission inputs must share the same manifold id"
            ));
        }

        let admission_trace_path = "runtime_v2/csm_run/boot_admission_trace.jsonl".to_string();
        let worker_records = citizens
            .records
            .iter()
            .filter(|record| {
                record.citizen_id == "proto-citizen-alpha"
                    || record.citizen_id == "proto-citizen-beta"
            })
            .collect::<Vec<_>>();
        if worker_records.len() != 2 {
            return Err(anyhow!(
                "CSM boot/admission prototype requires exactly two worker citizens"
            ));
        }

        let admitted_citizens = worker_records
            .iter()
            .map(|record| RuntimeV2CsmAdmissionReceipt {
                citizen_id: record.citizen_id.clone(),
                role: "worker".to_string(),
                identity_handle: format!(
                    "runtime-v2://{}/citizens/{}",
                    manifold.manifold_id, record.citizen_id
                ),
                source_record_ref: record.record_path.clone(),
                admission_trace_ref: record.policy_boundary_refs.admission_trace_ref.clone(),
                admission_status: "admitted_for_bounded_run".to_string(),
                can_execute_episodes: record.citizen_id == "proto-citizen-alpha",
                boundary_note:
                    "provisional admission for v0.90.2 D3; not a true Godel-agent birthday"
                        .to_string(),
            })
            .collect::<Vec<_>>();

        let boot_manifest = RuntimeV2CsmBootManifest {
            schema_version: RUNTIME_V2_CSM_BOOT_MANIFEST_SCHEMA.to_string(),
            boot_id: "proto-csm-01-boot-0001".to_string(),
            demo_id: "D3".to_string(),
            manifold_id: manifold.manifold_id.clone(),
            artifact_path: "runtime_v2/csm_run/boot_manifest.json".to_string(),
            generated_at_utc: "2026-04-20T00:00:00Z".to_string(),
            run_packet_ref: csm_run.artifact_path.clone(),
            invariant_map_ref: invariant_contract.invariant_map.artifact_path.clone(),
            violation_schema_ref: invariant_contract.violation_schema.artifact_path.clone(),
            citizen_roster_ref: "runtime_v2/csm_run/citizen_roster.json".to_string(),
            admission_trace_ref: admission_trace_path.clone(),
            boot_state: "booted_bounded".to_string(),
            admitted_citizens,
            required_before_next_stage: vec![
                "runtime_v2/csm_run/boot_manifest.json".to_string(),
                "runtime_v2/csm_run/citizen_roster.json".to_string(),
                admission_trace_path.clone(),
            ],
            claim_boundary:
                "This boot manifest proves bounded D3 boot/admission evidence; it is not a live governed episode and not a true Godel-agent birthday."
                    .to_string(),
        };

        let citizen_roster = RuntimeV2CsmCitizenRoster {
            schema_version: RUNTIME_V2_CSM_CITIZEN_ROSTER_SCHEMA.to_string(),
            roster_id: "proto-csm-01-worker-roster-0001".to_string(),
            demo_id: "D3".to_string(),
            manifold_id: manifold.manifold_id.clone(),
            artifact_path: "runtime_v2/csm_run/citizen_roster.json".to_string(),
            boot_manifest_ref: boot_manifest.artifact_path.clone(),
            entries: worker_records
                .iter()
                .map(|record| RuntimeV2CsmCitizenRosterEntry {
                    citizen_id: record.citizen_id.clone(),
                    display_name: record.display_name.clone(),
                    role: "worker".to_string(),
                    lifecycle_state_at_boot: record.lifecycle_state.clone(),
                    identity_handle: format!(
                        "runtime-v2://{}/citizens/{}",
                        manifold.manifold_id, record.citizen_id
                    ),
                    record_ref: record.record_path.clone(),
                    memory_root_ref: record.memory_identity_refs.memory_root_ref.clone(),
                    policy_ref: record.policy_boundary_refs.policy_ref.clone(),
                    admission_trace_ref: record
                        .policy_boundary_refs
                        .admission_trace_ref
                        .clone(),
                })
                .collect(),
            provisional_boundary:
                "workers are admitted for the bounded v0.90.2 CSM run; this does not promote them to v0.92 citizen continuity or birthday semantics"
                    .to_string(),
        };

        let admission_trace = vec![
            trace_event(TraceEventSpec {
                sequence: 1,
                event_id: "boot_contracts_verified",
                manifold_id: &manifold.manifold_id,
                service_id: "kernel_runtime",
                action: "verify_csm_run_packet_and_invariant_contracts",
                outcome: "passed",
                citizen_id: "proto-csm-01",
                artifact_ref: &csm_run.artifact_path,
            }),
            trace_event(TraceEventSpec {
                sequence: 2,
                event_id: "admit_proto_citizen_alpha",
                manifold_id: &manifold.manifold_id,
                service_id: "identity_admission_guard",
                action: "admit_worker_citizen",
                outcome: "admitted",
                citizen_id: "proto-citizen-alpha",
                artifact_ref: "runtime_v2/citizens/proto-citizen-alpha.json",
            }),
            trace_event(TraceEventSpec {
                sequence: 3,
                event_id: "admit_proto_citizen_beta",
                manifold_id: &manifold.manifold_id,
                service_id: "identity_admission_guard",
                action: "admit_worker_citizen",
                outcome: "admitted",
                citizen_id: "proto-citizen-beta",
                artifact_ref: "runtime_v2/citizens/proto-citizen-beta.json",
            }),
            trace_event(TraceEventSpec {
                sequence: 4,
                event_id: "boot_manifest_written",
                manifold_id: &manifold.manifold_id,
                service_id: "trace_writer",
                action: "record_boot_manifest",
                outcome: "written",
                citizen_id: "proto-csm-01",
                artifact_ref: &boot_manifest.artifact_path,
            }),
        ];

        let artifacts = Self {
            boot_manifest,
            citizen_roster,
            admission_trace,
            admission_trace_path,
        };
        artifacts.validate()?;
        Ok(artifacts)
    }

    pub fn validate(&self) -> Result<()> {
        self.boot_manifest.validate()?;
        self.citizen_roster.validate()?;
        validate_relative_path(&self.admission_trace_path, "csm_boot.admission_trace_path")?;
        if self.boot_manifest.admission_trace_ref != self.admission_trace_path {
            return Err(anyhow!(
                "CSM boot manifest admission_trace_ref must match trace artifact path"
            ));
        }
        if self.boot_manifest.citizen_roster_ref != self.citizen_roster.artifact_path {
            return Err(anyhow!(
                "CSM boot manifest must point at the generated citizen roster"
            ));
        }
        if self.admission_trace.len() != 4 {
            return Err(anyhow!(
                "CSM boot/admission trace must contain the bounded D3 event set"
            ));
        }
        for (index, event) in self.admission_trace.iter().enumerate() {
            event.validate()?;
            if event.event_sequence != index as u64 + 1 {
                return Err(anyhow!(
                    "CSM boot/admission trace events must be contiguous"
                ));
            }
            if event.manifold_id != self.boot_manifest.manifold_id {
                return Err(anyhow!(
                    "CSM boot/admission trace manifold id must match manifest"
                ));
            }
        }
        Ok(())
    }

    pub fn admission_trace_jsonl_bytes(&self) -> Result<Vec<u8>> {
        self.validate()?;
        let mut bytes = Vec::new();
        for event in &self.admission_trace {
            bytes.extend(serde_json::to_vec(event).context("serialize admission trace event")?);
            bytes.push(b'\n');
        }
        Ok(bytes)
    }

    pub fn write_to_root(&self, root: impl AsRef<Path>) -> Result<()> {
        let root = root.as_ref();
        self.boot_manifest.write_to_root(root)?;
        self.citizen_roster.write_to_root(root)?;
        write_relative(
            root,
            &self.admission_trace_path,
            self.admission_trace_jsonl_bytes()?,
        )
    }
}

impl RuntimeV2CsmBootManifest {
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_CSM_BOOT_MANIFEST_SCHEMA {
            return Err(anyhow!(
                "unsupported Runtime v2 CSM boot manifest schema '{}'",
                self.schema_version
            ));
        }
        if self.demo_id != "D3" {
            return Err(anyhow!("CSM boot manifest must map to D3"));
        }
        normalize_id(self.boot_id.clone(), "csm_boot.boot_id")?;
        normalize_id(self.manifold_id.clone(), "csm_boot.manifold_id")?;
        validate_relative_path(&self.artifact_path, "csm_boot.artifact_path")?;
        validate_timestamp_marker(&self.generated_at_utc, "csm_boot.generated_at_utc")?;
        validate_relative_path(&self.run_packet_ref, "csm_boot.run_packet_ref")?;
        validate_relative_path(&self.invariant_map_ref, "csm_boot.invariant_map_ref")?;
        validate_relative_path(&self.violation_schema_ref, "csm_boot.violation_schema_ref")?;
        validate_relative_path(&self.citizen_roster_ref, "csm_boot.citizen_roster_ref")?;
        validate_relative_path(&self.admission_trace_ref, "csm_boot.admission_trace_ref")?;
        match self.boot_state.as_str() {
            "booted_bounded" => {}
            other => return Err(anyhow!("unsupported csm_boot.boot_state '{other}'")),
        }
        validate_admission_receipts(&self.admitted_citizens)?;
        validate_relative_refs(
            &self.required_before_next_stage,
            "csm_boot.required_before_next_stage",
        )?;
        if !self.claim_boundary.contains("not a live governed episode")
            || !self
                .claim_boundary
                .contains("not a true Godel-agent birthday")
        {
            return Err(anyhow!(
                "CSM boot manifest must preserve governed-episode and birthday non-claims"
            ));
        }
        Ok(())
    }

    pub fn to_pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.validate()?;
        serde_json::to_vec_pretty(self).context("serialize Runtime v2 CSM boot manifest")
    }

    pub fn write_to_root(&self, root: impl AsRef<Path>) -> Result<()> {
        write_relative(
            root.as_ref(),
            &self.artifact_path,
            self.to_pretty_json_bytes()?,
        )
    }
}

impl RuntimeV2CsmAdmissionReceipt {
    pub fn validate(&self) -> Result<()> {
        normalize_id(self.citizen_id.clone(), "csm_boot.admitted_citizen")?;
        normalize_id(self.role.clone(), "csm_boot.admitted_role")?;
        validate_nonempty_text(&self.identity_handle, "csm_boot.identity_handle")?;
        if !self.identity_handle.starts_with("runtime-v2://") {
            return Err(anyhow!(
                "CSM boot identity handles must use the runtime-v2 scheme"
            ));
        }
        validate_relative_path(&self.source_record_ref, "csm_boot.source_record_ref")?;
        validate_relative_path(&self.admission_trace_ref, "csm_boot.admission_trace_ref")?;
        match self.admission_status.as_str() {
            "admitted_for_bounded_run" => {}
            other => return Err(anyhow!("unsupported csm_boot.admission_status '{other}'")),
        }
        validate_nonempty_text(&self.boundary_note, "csm_boot.boundary_note")
    }
}

impl RuntimeV2CsmCitizenRoster {
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_CSM_CITIZEN_ROSTER_SCHEMA {
            return Err(anyhow!(
                "unsupported Runtime v2 CSM citizen roster schema '{}'",
                self.schema_version
            ));
        }
        if self.demo_id != "D3" {
            return Err(anyhow!("CSM citizen roster must map to D3"));
        }
        normalize_id(self.roster_id.clone(), "csm_roster.roster_id")?;
        normalize_id(self.manifold_id.clone(), "csm_roster.manifold_id")?;
        validate_relative_path(&self.artifact_path, "csm_roster.artifact_path")?;
        validate_relative_path(&self.boot_manifest_ref, "csm_roster.boot_manifest_ref")?;
        if self.entries.len() != 2 {
            return Err(anyhow!(
                "CSM citizen roster must contain exactly two worker citizens"
            ));
        }
        let mut seen = std::collections::BTreeSet::new();
        for entry in &self.entries {
            entry.validate()?;
            if !seen.insert(entry.citizen_id.clone()) {
                return Err(anyhow!(
                    "CSM citizen roster contains duplicate citizen '{}'",
                    entry.citizen_id
                ));
            }
        }
        for required in ["proto-citizen-alpha", "proto-citizen-beta"] {
            if !seen.contains(required) {
                return Err(anyhow!(
                    "CSM citizen roster missing required worker '{required}'"
                ));
            }
        }
        if !self
            .provisional_boundary
            .contains("does not promote them to v0.92")
        {
            return Err(anyhow!(
                "CSM citizen roster must preserve the provisional boundary"
            ));
        }
        Ok(())
    }

    pub fn to_pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.validate()?;
        serde_json::to_vec_pretty(self).context("serialize Runtime v2 CSM citizen roster")
    }

    pub fn write_to_root(&self, root: impl AsRef<Path>) -> Result<()> {
        write_relative(
            root.as_ref(),
            &self.artifact_path,
            self.to_pretty_json_bytes()?,
        )
    }
}

impl RuntimeV2CsmCitizenRosterEntry {
    pub fn validate(&self) -> Result<()> {
        normalize_id(self.citizen_id.clone(), "csm_roster.citizen_id")?;
        validate_display_name(&self.display_name, "csm_roster.display_name")?;
        normalize_id(self.role.clone(), "csm_roster.role")?;
        validate_citizen_lifecycle_state(&self.lifecycle_state_at_boot)?;
        validate_nonempty_text(&self.identity_handle, "csm_roster.identity_handle")?;
        if !self.identity_handle.starts_with("runtime-v2://") {
            return Err(anyhow!(
                "CSM roster identity handles must use the runtime-v2 scheme"
            ));
        }
        validate_relative_path(&self.record_ref, "csm_roster.record_ref")?;
        validate_relative_path(&self.memory_root_ref, "csm_roster.memory_root_ref")?;
        validate_relative_path(&self.policy_ref, "csm_roster.policy_ref")?;
        validate_relative_path(&self.admission_trace_ref, "csm_roster.admission_trace_ref")
    }
}

impl RuntimeV2CsmAdmissionTraceEvent {
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_CSM_ADMISSION_TRACE_EVENT_SCHEMA {
            return Err(anyhow!(
                "unsupported Runtime v2 CSM admission trace event schema '{}'",
                self.schema_version
            ));
        }
        if self.event_sequence == 0 {
            return Err(anyhow!(
                "CSM admission trace event sequence must be positive"
            ));
        }
        normalize_id(self.event_id.clone(), "csm_trace.event_id")?;
        normalize_id(self.manifold_id.clone(), "csm_trace.manifold_id")?;
        normalize_id(self.service_id.clone(), "csm_trace.service_id")?;
        normalize_id(self.action.clone(), "csm_trace.action")?;
        match self.outcome.as_str() {
            "passed" | "admitted" | "written" => {}
            other => return Err(anyhow!("unsupported csm_trace.outcome '{other}'")),
        }
        normalize_id(self.citizen_id.clone(), "csm_trace.citizen_id")?;
        validate_relative_path(&self.artifact_ref, "csm_trace.artifact_ref")
    }
}

struct TraceEventSpec<'a> {
    sequence: u64,
    event_id: &'a str,
    manifold_id: &'a str,
    service_id: &'a str,
    action: &'a str,
    outcome: &'a str,
    citizen_id: &'a str,
    artifact_ref: &'a str,
}

fn trace_event(spec: TraceEventSpec<'_>) -> RuntimeV2CsmAdmissionTraceEvent {
    RuntimeV2CsmAdmissionTraceEvent {
        schema_version: RUNTIME_V2_CSM_ADMISSION_TRACE_EVENT_SCHEMA.to_string(),
        event_sequence: spec.sequence,
        event_id: spec.event_id.to_string(),
        manifold_id: spec.manifold_id.to_string(),
        service_id: spec.service_id.to_string(),
        action: spec.action.to_string(),
        outcome: spec.outcome.to_string(),
        citizen_id: spec.citizen_id.to_string(),
        artifact_ref: spec.artifact_ref.to_string(),
    }
}

fn validate_admission_receipts(receipts: &[RuntimeV2CsmAdmissionReceipt]) -> Result<()> {
    if receipts.len() != 2 {
        return Err(anyhow!(
            "CSM boot manifest must admit exactly two worker citizens"
        ));
    }
    let mut seen = std::collections::BTreeSet::new();
    let mut executable_count = 0;
    for receipt in receipts {
        receipt.validate()?;
        if !seen.insert(receipt.citizen_id.clone()) {
            return Err(anyhow!(
                "CSM boot manifest contains duplicate admission receipt '{}'",
                receipt.citizen_id
            ));
        }
        if receipt.can_execute_episodes {
            executable_count += 1;
        }
    }
    for required in ["proto-citizen-alpha", "proto-citizen-beta"] {
        if !seen.contains(required) {
            return Err(anyhow!(
                "CSM boot manifest missing required worker '{required}'"
            ));
        }
    }
    if executable_count != 1 {
        return Err(anyhow!(
            "CSM boot manifest must keep only one worker executable before WP-06 scheduling"
        ));
    }
    Ok(())
}

fn validate_relative_refs(refs: &[String], field: &str) -> Result<()> {
    if refs.is_empty() {
        return Err(anyhow!("{field} must not be empty"));
    }
    let mut seen = std::collections::BTreeSet::new();
    for reference in refs {
        validate_relative_path(reference, field)?;
        if !seen.insert(reference.clone()) {
            return Err(anyhow!("{field} contains duplicate reference"));
        }
    }
    Ok(())
}
