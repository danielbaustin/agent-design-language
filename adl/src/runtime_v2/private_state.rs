//! Runtime-v2 private-state contract primitives.
//!
//! Describes private-state references, lifecycle metadata, and serialization-safe
//! payloads for privacy-sensitive execution flow.

use super::*;
use sha2::{Digest, Sha256};

pub const RUNTIME_V2_PRIVATE_STATE_FORMAT_DECISION_SCHEMA: &str =
    "runtime_v2.private_state_format_decision.v1";
pub const RUNTIME_V2_PRIVATE_CITIZEN_STATE_SCHEMA: &str = "runtime_v2.private_citizen_state.v1";
pub const RUNTIME_V2_PRIVATE_STATE_PROJECTION_SCHEMA: &str =
    "runtime_v2.private_state_projection.v1";
pub const RUNTIME_V2_PRIVATE_STATE_FORMAT_DECISION_PATH: &str =
    "runtime_v2/private_state/format_decision.json";
pub const RUNTIME_V2_PRIVATE_STATE_CANONICAL_PATH: &str =
    "runtime_v2/private_state/proto-citizen-alpha.private-state.bin";
pub const RUNTIME_V2_PRIVATE_STATE_PROJECTION_PATH: &str =
    "runtime_v2/private_state/proto-citizen-alpha.projection.json";

const PRIVATE_STATE_MAGIC: &[u8] = b"ADLPSv1\0";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2PrivateStateFormatDecision {
    pub schema_version: String,
    pub decision_id: String,
    pub milestone: String,
    pub selected_format: String,
    pub canonical_state_schema: String,
    pub canonical_artifact_path: String,
    pub projection_schema: String,
    pub projection_artifact_path: String,
    pub authority_rule: String,
    pub projection_rule: String,
    pub serialization_rules: Vec<String>,
    pub schema_evolution_rules: Vec<String>,
    pub compatibility_notes: Vec<String>,
    pub validation_commands: Vec<String>,
    pub source_audit_refs: Vec<String>,
    pub non_claims: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeV2PrivateCitizenState {
    pub schema_version: String,
    pub artifact_kind: String,
    pub citizen_id: String,
    pub manifold_id: String,
    pub lineage_id: String,
    pub state_sequence: u64,
    pub predecessor_state_hash: String,
    pub projection_schema_version: String,
    pub projection_artifact_path: String,
    pub continuity_ledger_ref: String,
    pub private_sections: Vec<RuntimeV2PrivateStateSection>,
    pub projection_policy: RuntimeV2PrivateStateProjectionPolicy,
    pub schema_evolution: RuntimeV2PrivateStateSchemaEvolution,
    pub claim_boundary: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeV2PrivateStateSection {
    pub section_id: String,
    pub section_kind: String,
    pub classification: String,
    pub payload_digest: String,
    pub provenance_ref: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeV2PrivateStateProjectionPolicy {
    pub allowed_projection_schema: String,
    pub redacted_fields: Vec<String>,
    pub projection_purpose: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeV2PrivateStateSchemaEvolution {
    pub compatibility_family: String,
    pub minimum_reader_version: String,
    pub reserved_field_numbers: Vec<u32>,
    pub unknown_field_policy: String,
    pub migration_policy: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2PrivateStateProjection {
    pub schema_version: String,
    pub projection_id: String,
    pub projection_kind: String,
    pub citizen_id: String,
    pub manifold_id: String,
    pub lineage_id: String,
    pub state_sequence: u64,
    pub source_state_schema: String,
    pub source_state_ref: String,
    pub source_state_hash: String,
    pub visible_summary: Vec<String>,
    pub redacted_fields: Vec<String>,
    pub authority_status: String,
    pub validation_status: String,
    pub claim_boundary: String,
    pub non_claims: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeV2PrivateStateArtifacts {
    pub format_decision: RuntimeV2PrivateStateFormatDecision,
    pub canonical_state: RuntimeV2PrivateCitizenState,
    pub projection: RuntimeV2PrivateStateProjection,
}

impl RuntimeV2PrivateStateArtifacts {
    pub fn prototype() -> Result<Self> {
        let manifold = RuntimeV2ManifoldRoot::prototype("proto-csm-01")?;
        let citizens = RuntimeV2CitizenLifecycleArtifacts::prototype(&manifold)?;
        let active_citizen = citizens
            .records
            .iter()
            .find(|record| record.citizen_id == "proto-citizen-alpha")
            .ok_or_else(|| anyhow!("private-state prototype requires proto-citizen-alpha"))?;
        active_citizen.validate()?;

        let format_decision = RuntimeV2PrivateStateFormatDecision::prototype();
        let canonical_state = RuntimeV2PrivateCitizenState::prototype(&manifold, active_citizen)?;
        let projection =
            canonical_state.projection(RUNTIME_V2_PRIVATE_STATE_CANONICAL_PATH.to_string())?;
        let artifacts = Self {
            format_decision,
            canonical_state,
            projection,
        };
        artifacts.validate()?;
        Ok(artifacts)
    }

    pub fn validate(&self) -> Result<()> {
        self.format_decision.validate()?;
        self.canonical_state.validate()?;
        self.projection
            .validate_against_state(&self.canonical_state)?;
        if self.format_decision.canonical_state_schema != self.canonical_state.schema_version {
            return Err(anyhow!(
                "private-state decision schema must match canonical state schema"
            ));
        }
        if self.format_decision.projection_schema != self.projection.schema_version {
            return Err(anyhow!(
                "private-state decision projection schema must match projection"
            ));
        }
        Ok(())
    }

    pub fn write_to_root(&self, root: impl AsRef<Path>) -> Result<()> {
        self.validate()?;
        let root = root.as_ref();
        write_relative(
            root,
            RUNTIME_V2_PRIVATE_STATE_FORMAT_DECISION_PATH,
            self.format_decision.pretty_json_bytes()?,
        )?;
        write_relative(
            root,
            RUNTIME_V2_PRIVATE_STATE_CANONICAL_PATH,
            self.canonical_state.canonical_bytes()?,
        )?;
        write_relative(
            root,
            RUNTIME_V2_PRIVATE_STATE_PROJECTION_PATH,
            self.projection.pretty_json_bytes()?,
        )?;
        Ok(())
    }
}

impl RuntimeV2PrivateStateFormatDecision {
    pub fn prototype() -> Self {
        Self {
            schema_version: RUNTIME_V2_PRIVATE_STATE_FORMAT_DECISION_SCHEMA.to_string(),
            decision_id: "v0-90-3-wp-03-private-state-format".to_string(),
            milestone: "v0.90.3".to_string(),
            selected_format: "deterministic_tagged_binary_v1_with_protobuf_compatible_field_numbers".to_string(),
            canonical_state_schema: RUNTIME_V2_PRIVATE_CITIZEN_STATE_SCHEMA.to_string(),
            canonical_artifact_path: RUNTIME_V2_PRIVATE_STATE_CANONICAL_PATH.to_string(),
            projection_schema: RUNTIME_V2_PRIVATE_STATE_PROJECTION_SCHEMA.to_string(),
            projection_artifact_path: RUNTIME_V2_PRIVATE_STATE_PROJECTION_PATH.to_string(),
            authority_rule:
                "The canonical binary private-state artifact is authority; JSON is only a redacted projection for review surfaces."
                    .to_string(),
            projection_rule:
                "Projection must be derived from canonical bytes, must carry the source hash, and must never be accepted as private-state authority."
                    .to_string(),
            serialization_rules: vec![
                "field numbers are encoded in ascending order".to_string(),
                "integers are encoded big-endian".to_string(),
                "strings are UTF-8 with u32 length prefixes".to_string(),
                "repeated sections preserve declared order after validation".to_string(),
                "content hashes are sha256 over the exact canonical byte stream".to_string(),
            ],
            schema_evolution_rules: vec![
                "field numbers 1 through 15 are reserved for v1 identity, lineage, projection, ledger, and policy fields".to_string(),
                "unknown required fields fail closed until a migration policy accepts them".to_string(),
                "future protobuf/prost bindings must preserve the same field meanings and hash input boundaries".to_string(),
                "schema migration must emit a continuity witness before later WPs may treat the successor as active".to_string(),
            ],
            compatibility_notes: vec![
                "v0.90.2 JSON citizen, wake, quarantine, and Observatory artifacts are inheritance evidence, not durable private-state authority".to_string(),
                "v0.90.3 WP-03 records the canonical format boundary before signed envelopes, local sealing, and append-only ledger work add stronger authority layers".to_string(),
                "JSON projections remain useful for operators only when they hash-link back to canonical bytes and preserve redaction boundaries".to_string(),
            ],
            validation_commands: vec![
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_private_state -- --nocapture".to_string(),
                "git diff --check".to_string(),
            ],
            source_audit_refs: vec![
                "docs/milestones/v0.90.3/CITIZEN_STATE_INHERITANCE_AUDIT_v0.90.3.md".to_string(),
                "docs/milestones/v0.90.3/features/CITIZEN_STATE_SECURITY_AND_FORMAT.md".to_string(),
            ],
            non_claims: vec![
                "does not implement signed envelopes or trust-root validation".to_string(),
                "does not implement encryption, sealed checkpoint storage, or cloud enclaves".to_string(),
                "does not implement append-only lineage replay or duplicate-head quarantine".to_string(),
                "does not claim first true Godel-agent birth".to_string(),
            ],
        }
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_PRIVATE_STATE_FORMAT_DECISION_SCHEMA {
            return Err(anyhow!(
                "unsupported private-state format decision schema '{}'",
                self.schema_version
            ));
        }
        normalize_id(self.decision_id.clone(), "private_state.decision_id")?;
        if self.milestone != "v0.90.3" {
            return Err(anyhow!(
                "private-state format decision must target milestone v0.90.3"
            ));
        }
        if !self.selected_format.contains("binary") {
            return Err(anyhow!(
                "private-state format decision must select a binary canonical format"
            ));
        }
        if self.canonical_state_schema != RUNTIME_V2_PRIVATE_CITIZEN_STATE_SCHEMA {
            return Err(anyhow!(
                "private-state decision must name the canonical state schema"
            ));
        }
        if self.projection_schema != RUNTIME_V2_PRIVATE_STATE_PROJECTION_SCHEMA {
            return Err(anyhow!(
                "private-state decision must name the projection schema"
            ));
        }
        validate_relative_path(
            &self.canonical_artifact_path,
            "private_state.canonical_artifact_path",
        )?;
        validate_relative_path(
            &self.projection_artifact_path,
            "private_state.projection_artifact_path",
        )?;
        if !self.authority_rule.contains("canonical binary")
            || !self.authority_rule.contains("JSON is only")
        {
            return Err(anyhow!(
                "private-state decision must state that JSON is not authority"
            ));
        }
        if !self
            .projection_rule
            .contains("must never be accepted as private-state authority")
        {
            return Err(anyhow!(
                "private-state projection rule must reject JSON authority"
            ));
        }
        require_text_list(
            &self.serialization_rules,
            "private_state.serialization_rules",
            4,
        )?;
        require_text_list(
            &self.schema_evolution_rules,
            "private_state.schema_evolution_rules",
            3,
        )?;
        require_text_list(
            &self.compatibility_notes,
            "private_state.compatibility_notes",
            2,
        )?;
        require_text_list(
            &self.source_audit_refs,
            "private_state.source_audit_refs",
            1,
        )?;
        for path in &self.source_audit_refs {
            validate_relative_path(path, "private_state.source_audit_refs")?;
        }
        if !self
            .validation_commands
            .iter()
            .any(|command| command.contains("runtime_v2_private_state"))
        {
            return Err(anyhow!(
                "private-state decision must include focused validation command"
            ));
        }
        if !self
            .non_claims
            .iter()
            .any(|claim| claim.contains("signed envelopes"))
        {
            return Err(anyhow!(
                "private-state decision must preserve signed-envelope non-claim"
            ));
        }
        Ok(())
    }

    pub fn pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.validate()?;
        serde_json::to_vec_pretty(self).context("serialize private-state format decision")
    }
}

impl RuntimeV2PrivateCitizenState {
    pub fn prototype(
        manifold: &RuntimeV2ManifoldRoot,
        citizen: &RuntimeV2ProvisionalCitizenRecord,
    ) -> Result<Self> {
        manifold.validate()?;
        citizen.validate()?;
        if citizen.manifold_id != manifold.manifold_id {
            return Err(anyhow!(
                "private-state citizen manifold id must match manifold"
            ));
        }
        if !citizen.can_execute_episodes {
            return Err(anyhow!(
                "private-state prototype requires an active executable citizen"
            ));
        }

        let state = Self {
            schema_version: RUNTIME_V2_PRIVATE_CITIZEN_STATE_SCHEMA.to_string(),
            artifact_kind: "canonical_private_citizen_state".to_string(),
            citizen_id: citizen.citizen_id.clone(),
            manifold_id: manifold.manifold_id.clone(),
            lineage_id: format!("lineage-{}", citizen.citizen_id),
            state_sequence: 1,
            predecessor_state_hash: "genesis".to_string(),
            projection_schema_version: RUNTIME_V2_PRIVATE_STATE_PROJECTION_SCHEMA.to_string(),
            projection_artifact_path: RUNTIME_V2_PRIVATE_STATE_PROJECTION_PATH.to_string(),
            continuity_ledger_ref: "runtime_v2/private_state/lineage_ledger.jsonl".to_string(),
            private_sections: vec![
                RuntimeV2PrivateStateSection {
                    section_id: "identity-core".to_string(),
                    section_kind: "identity".to_string(),
                    classification: "private_citizen_state".to_string(),
                    payload_digest: "sha256:8a86f6e05f10dfd4f49d6e40e612b87bb4a4ab73672cb2a79b398c39b1d903d5".to_string(),
                    provenance_ref: citizen.memory_identity_refs.identity_profile_ref.clone(),
                },
                RuntimeV2PrivateStateSection {
                    section_id: "continuity-memory".to_string(),
                    section_kind: "memory".to_string(),
                    classification: "private_citizen_state".to_string(),
                    payload_digest: "sha256:7238cf13bdaf86f7c287f199c26f83080f615f3bb473cf119f072cb9f4aebbb3".to_string(),
                    provenance_ref: citizen.memory_identity_refs.memory_root_ref.clone(),
                },
            ],
            projection_policy: RuntimeV2PrivateStateProjectionPolicy {
                allowed_projection_schema: RUNTIME_V2_PRIVATE_STATE_PROJECTION_SCHEMA.to_string(),
                redacted_fields: vec![
                    "private_sections.payload_bytes".to_string(),
                    "private_sections.payload_digest".to_string(),
                    "identity_core.contents".to_string(),
                    "continuity_memory.contents".to_string(),
                ],
                projection_purpose:
                    "operator review of continuity status without raw private-state inspection"
                        .to_string(),
            },
            schema_evolution: RuntimeV2PrivateStateSchemaEvolution {
                compatibility_family: "runtime_v2_private_state".to_string(),
                minimum_reader_version: "v0.90.3".to_string(),
                reserved_field_numbers: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
                unknown_field_policy: "fail_closed_until_migration_witness".to_string(),
                migration_policy:
                    "successor schemas must preserve citizen_id, lineage_id, sequence, predecessor hash, and projection hash-link semantics"
                        .to_string(),
            },
            claim_boundary:
                "This binary fixture is canonical private state for WP-03; JSON projection is not authority and cannot wake or migrate a citizen."
                    .to_string(),
        };
        state.validate()?;
        Ok(state)
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_PRIVATE_CITIZEN_STATE_SCHEMA {
            return Err(anyhow!(
                "unsupported private citizen state schema '{}'",
                self.schema_version
            ));
        }
        normalize_id(self.artifact_kind.clone(), "private_state.artifact_kind")?;
        normalize_id(self.citizen_id.clone(), "private_state.citizen_id")?;
        normalize_id(self.manifold_id.clone(), "private_state.manifold_id")?;
        normalize_id(self.lineage_id.clone(), "private_state.lineage_id")?;
        if self.state_sequence == 0 {
            return Err(anyhow!("private-state sequence must be greater than zero"));
        }
        validate_hash_or_genesis(
            &self.predecessor_state_hash,
            "private_state.predecessor_state_hash",
        )?;
        if self.projection_schema_version != RUNTIME_V2_PRIVATE_STATE_PROJECTION_SCHEMA {
            return Err(anyhow!(
                "private-state projection schema version must match projection schema"
            ));
        }
        validate_relative_path(
            &self.projection_artifact_path,
            "private_state.projection_artifact_path",
        )?;
        validate_relative_path(
            &self.continuity_ledger_ref,
            "private_state.continuity_ledger_ref",
        )?;
        if self.private_sections.is_empty() {
            return Err(anyhow!(
                "private-state must include at least one private section"
            ));
        }
        for section in &self.private_sections {
            section.validate()?;
        }
        self.projection_policy.validate()?;
        self.schema_evolution.validate()?;
        if !self
            .claim_boundary
            .contains("JSON projection is not authority")
        {
            return Err(anyhow!(
                "private-state claim boundary must reject JSON authority"
            ));
        }
        Ok(())
    }

    pub fn canonical_bytes(&self) -> Result<Vec<u8>> {
        self.validate()?;
        let mut bytes = Vec::new();
        bytes.extend_from_slice(PRIVATE_STATE_MAGIC);
        push_string_field(&mut bytes, 1, &self.schema_version)?;
        push_string_field(&mut bytes, 2, &self.artifact_kind)?;
        push_string_field(&mut bytes, 3, &self.citizen_id)?;
        push_string_field(&mut bytes, 4, &self.manifold_id)?;
        push_string_field(&mut bytes, 5, &self.lineage_id)?;
        push_u64_field(&mut bytes, 6, self.state_sequence);
        push_string_field(&mut bytes, 7, &self.predecessor_state_hash)?;
        push_string_field(&mut bytes, 8, &self.projection_schema_version)?;
        push_string_field(&mut bytes, 9, &self.projection_artifact_path)?;
        push_string_field(&mut bytes, 10, &self.continuity_ledger_ref)?;
        push_sections_field(&mut bytes, 11, &self.private_sections)?;
        push_projection_policy_field(&mut bytes, 12, &self.projection_policy)?;
        push_schema_evolution_field(&mut bytes, 13, &self.schema_evolution)?;
        push_string_field(&mut bytes, 14, &self.claim_boundary)?;
        Ok(bytes)
    }

    pub fn content_hash(&self) -> Result<String> {
        Ok(format!(
            "sha256:{:x}",
            Sha256::digest(self.canonical_bytes()?)
        ))
    }

    pub fn projection(&self, source_state_ref: String) -> Result<RuntimeV2PrivateStateProjection> {
        self.validate()?;
        validate_relative_path(&source_state_ref, "private_state.source_state_ref")?;
        let projection = RuntimeV2PrivateStateProjection {
            schema_version: RUNTIME_V2_PRIVATE_STATE_PROJECTION_SCHEMA.to_string(),
            projection_id: format!("projection-{}-{:04}", self.citizen_id, self.state_sequence),
            projection_kind: "redacted_operator_projection".to_string(),
            citizen_id: self.citizen_id.clone(),
            manifold_id: self.manifold_id.clone(),
            lineage_id: self.lineage_id.clone(),
            state_sequence: self.state_sequence,
            source_state_schema: self.schema_version.clone(),
            source_state_ref,
            source_state_hash: self.content_hash()?,
            visible_summary: vec![
                "canonical private state exists for proto-citizen-alpha".to_string(),
                "state sequence 1 continues from genesis".to_string(),
                "private payload bytes and section digests are redacted from this projection".to_string(),
                "projection can support operator review but cannot authorize wake, restore, migration, or inspection".to_string(),
            ],
            redacted_fields: self.projection_policy.redacted_fields.clone(),
            authority_status: "projection_not_authority".to_string(),
            validation_status: "hash_linked_to_canonical_binary_state".to_string(),
            claim_boundary:
                "This JSON projection is review evidence only; canonical binary private-state bytes remain the authority."
                    .to_string(),
            non_claims: vec![
                "does not expose raw private-state payloads".to_string(),
                "does not authorize wake or migration".to_string(),
                "does not replace signed envelope validation planned for WP-04".to_string(),
            ],
        };
        projection.validate_against_state(self)?;
        Ok(projection)
    }
}

impl RuntimeV2PrivateStateSection {
    pub fn validate(&self) -> Result<()> {
        normalize_id(self.section_id.clone(), "private_state.section_id")?;
        normalize_id(self.section_kind.clone(), "private_state.section_kind")?;
        if self.classification != "private_citizen_state" {
            return Err(anyhow!(
                "private-state section classification must remain private"
            ));
        }
        validate_sha256_hash(&self.payload_digest, "private_state.payload_digest")?;
        validate_relative_path(&self.provenance_ref, "private_state.provenance_ref")?;
        Ok(())
    }
}

impl RuntimeV2PrivateStateProjectionPolicy {
    pub fn validate(&self) -> Result<()> {
        if self.allowed_projection_schema != RUNTIME_V2_PRIVATE_STATE_PROJECTION_SCHEMA {
            return Err(anyhow!(
                "private-state projection policy must name the allowed projection schema"
            ));
        }
        require_text_list(&self.redacted_fields, "private_state.redacted_fields", 1)?;
        if !self
            .redacted_fields
            .iter()
            .any(|field| field.contains("payload"))
        {
            return Err(anyhow!(
                "private-state projection policy must redact payload fields"
            ));
        }
        validate_nonempty_text(&self.projection_purpose, "private_state.projection_purpose")?;
        Ok(())
    }
}

impl RuntimeV2PrivateStateSchemaEvolution {
    pub fn validate(&self) -> Result<()> {
        normalize_id(
            self.compatibility_family.clone(),
            "private_state.compatibility_family",
        )?;
        validate_nonempty_text(
            &self.minimum_reader_version,
            "private_state.minimum_reader_version",
        )?;
        if self.reserved_field_numbers.len() < 8 {
            return Err(anyhow!(
                "private-state schema evolution must reserve v1 field numbers"
            ));
        }
        if self.unknown_field_policy != "fail_closed_until_migration_witness" {
            return Err(anyhow!(
                "private-state unknown fields must fail closed until migration witness"
            ));
        }
        if !self.migration_policy.contains("predecessor hash") {
            return Err(anyhow!(
                "private-state migration policy must preserve predecessor linkage"
            ));
        }
        Ok(())
    }
}

impl RuntimeV2PrivateStateProjection {
    pub fn validate_shape(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_PRIVATE_STATE_PROJECTION_SCHEMA {
            return Err(anyhow!(
                "unsupported private-state projection schema '{}'",
                self.schema_version
            ));
        }
        normalize_id(self.projection_id.clone(), "private_state.projection_id")?;
        normalize_id(
            self.projection_kind.clone(),
            "private_state.projection_kind",
        )?;
        normalize_id(self.citizen_id.clone(), "private_state.citizen_id")?;
        normalize_id(self.manifold_id.clone(), "private_state.manifold_id")?;
        normalize_id(self.lineage_id.clone(), "private_state.lineage_id")?;
        if self.state_sequence == 0 {
            return Err(anyhow!(
                "private-state projection sequence must be greater than zero"
            ));
        }
        if self.source_state_schema != RUNTIME_V2_PRIVATE_CITIZEN_STATE_SCHEMA {
            return Err(anyhow!(
                "private-state projection source schema must name canonical state"
            ));
        }
        validate_relative_path(&self.source_state_ref, "private_state.source_state_ref")?;
        validate_sha256_hash(&self.source_state_hash, "private_state.source_state_hash")?;
        require_text_list(&self.visible_summary, "private_state.visible_summary", 1)?;
        require_text_list(&self.redacted_fields, "private_state.redacted_fields", 1)?;
        if self.authority_status != "projection_not_authority" {
            return Err(anyhow!(
                "private-state projection must declare non-authority status"
            ));
        }
        if !self.validation_status.contains("canonical_binary") {
            return Err(anyhow!(
                "private-state projection must be hash-linked to canonical binary state"
            ));
        }
        if !self.claim_boundary.contains("review evidence only")
            || !self.claim_boundary.contains("canonical binary")
        {
            return Err(anyhow!(
                "private-state projection claim boundary must preserve authority separation"
            ));
        }
        if !self
            .non_claims
            .iter()
            .any(|claim| claim.contains("raw private-state payloads"))
        {
            return Err(anyhow!(
                "private-state projection must preserve raw-payload non-claim"
            ));
        }
        Ok(())
    }

    pub fn validate_against_state(&self, state: &RuntimeV2PrivateCitizenState) -> Result<()> {
        state.validate()?;
        self.validate_shape()?;
        if self.citizen_id != state.citizen_id
            || self.manifold_id != state.manifold_id
            || self.lineage_id != state.lineage_id
        {
            return Err(anyhow!(
                "private-state projection identity and lineage must match canonical state"
            ));
        }
        if self.state_sequence != state.state_sequence {
            return Err(anyhow!(
                "private-state projection sequence must match canonical state"
            ));
        }
        if self.source_state_schema != state.schema_version {
            return Err(anyhow!(
                "private-state projection source schema must match canonical state"
            ));
        }
        if self.source_state_hash != state.content_hash()? {
            return Err(anyhow!(
                "private-state projection source hash must match canonical bytes"
            ));
        }
        Ok(())
    }

    pub fn pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.validate_shape()?;
        serde_json::to_vec_pretty(self).context("serialize private-state projection")
    }
}

fn require_text_list(values: &[String], field: &str, min_len: usize) -> Result<()> {
    if values.len() < min_len {
        return Err(anyhow!("{field} must include at least {min_len} entries"));
    }
    for value in values {
        validate_nonempty_text(value, field)?;
    }
    Ok(())
}

fn validate_hash_or_genesis(value: &str, field: &str) -> Result<()> {
    if value == "genesis" {
        return Ok(());
    }
    validate_sha256_hash(value, field)
}

fn validate_sha256_hash(value: &str, field: &str) -> Result<()> {
    let hex = value
        .strip_prefix("sha256:")
        .ok_or_else(|| anyhow!("{field} must be a sha256 hash"))?;
    if hex.len() != 64 || !hex.chars().all(|ch| ch.is_ascii_hexdigit()) {
        return Err(anyhow!("{field} must be a 64-character sha256 digest"));
    }
    Ok(())
}

fn push_string_field(bytes: &mut Vec<u8>, tag: u16, value: &str) -> Result<()> {
    validate_nonempty_text(value, "private_state.canonical_string")?;
    bytes.extend_from_slice(&tag.to_be_bytes());
    let value_bytes = value.as_bytes();
    let len = u32::try_from(value_bytes.len()).context("private-state string too long")?;
    bytes.extend_from_slice(&len.to_be_bytes());
    bytes.extend_from_slice(value_bytes);
    Ok(())
}

fn push_u64_field(bytes: &mut Vec<u8>, tag: u16, value: u64) {
    bytes.extend_from_slice(&tag.to_be_bytes());
    bytes.extend_from_slice(&8_u32.to_be_bytes());
    bytes.extend_from_slice(&value.to_be_bytes());
}

fn push_u32_field(bytes: &mut Vec<u8>, tag: u16, value: u32) {
    bytes.extend_from_slice(&tag.to_be_bytes());
    bytes.extend_from_slice(&4_u32.to_be_bytes());
    bytes.extend_from_slice(&value.to_be_bytes());
}

fn push_sections_field(
    bytes: &mut Vec<u8>,
    tag: u16,
    sections: &[RuntimeV2PrivateStateSection],
) -> Result<()> {
    let mut nested = Vec::new();
    let len = u32::try_from(sections.len()).context("too many private-state sections")?;
    nested.extend_from_slice(&len.to_be_bytes());
    for section in sections {
        push_string_field(&mut nested, 1, &section.section_id)?;
        push_string_field(&mut nested, 2, &section.section_kind)?;
        push_string_field(&mut nested, 3, &section.classification)?;
        push_string_field(&mut nested, 4, &section.payload_digest)?;
        push_string_field(&mut nested, 5, &section.provenance_ref)?;
    }
    push_bytes_field(bytes, tag, &nested)
}

fn push_projection_policy_field(
    bytes: &mut Vec<u8>,
    tag: u16,
    policy: &RuntimeV2PrivateStateProjectionPolicy,
) -> Result<()> {
    let mut nested = Vec::new();
    push_string_field(&mut nested, 1, &policy.allowed_projection_schema)?;
    push_string_vec_field(&mut nested, 2, &policy.redacted_fields)?;
    push_string_field(&mut nested, 3, &policy.projection_purpose)?;
    push_bytes_field(bytes, tag, &nested)
}

fn push_schema_evolution_field(
    bytes: &mut Vec<u8>,
    tag: u16,
    evolution: &RuntimeV2PrivateStateSchemaEvolution,
) -> Result<()> {
    let mut nested = Vec::new();
    push_string_field(&mut nested, 1, &evolution.compatibility_family)?;
    push_string_field(&mut nested, 2, &evolution.minimum_reader_version)?;
    let len = u32::try_from(evolution.reserved_field_numbers.len())
        .context("too many reserved private-state field numbers")?;
    nested.extend_from_slice(&len.to_be_bytes());
    for number in &evolution.reserved_field_numbers {
        push_u32_field(&mut nested, 3, *number);
    }
    push_string_field(&mut nested, 4, &evolution.unknown_field_policy)?;
    push_string_field(&mut nested, 5, &evolution.migration_policy)?;
    push_bytes_field(bytes, tag, &nested)
}

fn push_string_vec_field(bytes: &mut Vec<u8>, tag: u16, values: &[String]) -> Result<()> {
    let mut nested = Vec::new();
    let len = u32::try_from(values.len()).context("too many private-state string values")?;
    nested.extend_from_slice(&len.to_be_bytes());
    for value in values {
        push_string_field(&mut nested, 1, value)?;
    }
    push_bytes_field(bytes, tag, &nested)
}

fn push_bytes_field(bytes: &mut Vec<u8>, tag: u16, value: &[u8]) -> Result<()> {
    bytes.extend_from_slice(&tag.to_be_bytes());
    let len = u32::try_from(value.len()).context("private-state bytes too long")?;
    bytes.extend_from_slice(&len.to_be_bytes());
    bytes.extend_from_slice(value);
    Ok(())
}
