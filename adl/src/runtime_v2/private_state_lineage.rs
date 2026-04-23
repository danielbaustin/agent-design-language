//! Runtime-v2 private-state lineage tracking.
//!
//! Documents lineage references and continuity chain metadata used to prove
//! deterministic private-state evolution across state transitions.

use super::*;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;

pub const RUNTIME_V2_PRIVATE_STATE_LINEAGE_LEDGER_SCHEMA: &str =
    "runtime_v2.private_state_lineage_ledger.v1";
pub const RUNTIME_V2_PRIVATE_STATE_MATERIALIZED_HEAD_SCHEMA: &str =
    "runtime_v2.private_state_materialized_head.v1";
pub const RUNTIME_V2_PRIVATE_STATE_LINEAGE_PROOF_SCHEMA: &str =
    "runtime_v2.private_state_lineage_proof.v1";
pub const RUNTIME_V2_PRIVATE_STATE_LINEAGE_DISPOSITION_SCHEMA: &str =
    "runtime_v2.private_state_lineage_disposition.v1";
pub const RUNTIME_V2_PRIVATE_STATE_LINEAGE_LEDGER_PATH: &str =
    "runtime_v2/private_state/lineage_ledger.json";
pub const RUNTIME_V2_PRIVATE_STATE_MATERIALIZED_HEAD_PATH: &str =
    "runtime_v2/private_state/materialized_head.json";
pub const RUNTIME_V2_PRIVATE_STATE_LINEAGE_PROOF_PATH: &str =
    "runtime_v2/private_state/lineage_negative_cases.json";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2PrivateStateLineageLedger {
    pub schema_version: String,
    pub ledger_id: String,
    pub artifact_path: String,
    pub citizen_id: String,
    pub manifold_id: String,
    pub lineage_id: String,
    pub authority_rule: String,
    pub append_only_rule: String,
    pub entries: Vec<RuntimeV2PrivateStateLineageEntry>,
    pub accepted_head_entry_hash: String,
    pub ledger_root_hash: String,
    pub recovery_policy: Vec<String>,
    pub non_claims: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2PrivateStateLineageEntry {
    pub entry_id: String,
    pub entry_hash: String,
    pub previous_entry_hash: String,
    pub transition_type: String,
    pub citizen_id: String,
    pub manifold_id: String,
    pub lineage_id: String,
    pub state_sequence: u64,
    pub predecessor_state_hash: String,
    pub envelope_ref: String,
    pub envelope_hash: String,
    pub sealed_checkpoint_ref: String,
    pub sealed_checkpoint_hash: String,
    pub canonical_state_hash: String,
    pub writer_identity: String,
    pub witness_ref: Option<String>,
    pub receipt_ref: Option<String>,
    pub recorded_at_logical_tick: u64,
    pub disposition: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2PrivateStateMaterializedHead {
    pub schema_version: String,
    pub head_id: String,
    pub artifact_path: String,
    pub citizen_id: String,
    pub manifold_id: String,
    pub lineage_id: String,
    pub state_sequence: u64,
    pub head_entry_hash: String,
    pub ledger_ref: String,
    pub sealed_checkpoint_ref: String,
    pub sealed_checkpoint_hash: String,
    pub canonical_state_hash: String,
    pub status: String,
    pub authority_rule: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2PrivateStateLineageNegativeCase {
    pub case_id: String,
    pub mutation: String,
    pub expected_error_fragment: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2PrivateStateLineageProof {
    pub schema_version: String,
    pub proof_id: String,
    pub demo_id: String,
    pub ledger_ref: String,
    pub materialized_head_ref: String,
    pub sealed_checkpoint_ref: String,
    pub required_negative_cases: Vec<RuntimeV2PrivateStateLineageNegativeCase>,
    pub validation_command: String,
    pub claim_boundary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2PrivateStateLineageDisposition {
    pub schema_version: String,
    pub disposition_id: String,
    pub citizen_id: String,
    pub lineage_id: String,
    pub disposition: String,
    pub reason: String,
    pub evidence_refs: Vec<String>,
    pub required_next_step: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeV2PrivateStateLineageArtifacts {
    pub sealing_artifacts: RuntimeV2PrivateStateSealingArtifacts,
    pub ledger: RuntimeV2PrivateStateLineageLedger,
    pub materialized_head: RuntimeV2PrivateStateMaterializedHead,
    pub negative_cases: RuntimeV2PrivateStateLineageProof,
}

impl RuntimeV2PrivateStateLineageArtifacts {
    pub fn prototype() -> Result<Self> {
        let sealing_artifacts = RuntimeV2PrivateStateSealingArtifacts::prototype()?;
        let ledger = RuntimeV2PrivateStateLineageLedger::prototype(&sealing_artifacts)?;
        let materialized_head =
            RuntimeV2PrivateStateMaterializedHead::from_ledger_head(&ledger, &sealing_artifacts)?;
        let negative_cases = RuntimeV2PrivateStateLineageProof::prototype();
        let artifacts = Self {
            sealing_artifacts,
            ledger,
            materialized_head,
            negative_cases,
        };
        artifacts.validate()?;
        Ok(artifacts)
    }

    pub fn validate(&self) -> Result<()> {
        self.sealing_artifacts.validate()?;
        self.ledger.validate_against(&self.sealing_artifacts)?;
        self.materialized_head
            .validate_against_ledger(&self.ledger, &self.sealing_artifacts)?;
        self.negative_cases.validate()?;
        Ok(())
    }

    pub fn write_to_root(&self, root: impl AsRef<Path>) -> Result<()> {
        self.validate()?;
        let root = root.as_ref();
        write_relative(
            root,
            RUNTIME_V2_PRIVATE_STATE_LINEAGE_LEDGER_PATH,
            self.ledger.pretty_json_bytes()?,
        )?;
        write_relative(
            root,
            RUNTIME_V2_PRIVATE_STATE_MATERIALIZED_HEAD_PATH,
            self.materialized_head.pretty_json_bytes()?,
        )?;
        write_relative(
            root,
            RUNTIME_V2_PRIVATE_STATE_LINEAGE_PROOF_PATH,
            self.negative_cases.pretty_json_bytes()?,
        )?;
        Ok(())
    }
}

impl RuntimeV2PrivateStateLineageLedger {
    pub fn prototype(sealing: &RuntimeV2PrivateStateSealingArtifacts) -> Result<Self> {
        sealing.validate()?;
        let state = &sealing.envelope_artifacts.private_state.canonical_state;
        let checkpoint_hash = sha256_json(&sealing.sealed_checkpoint.pretty_json_bytes()?)?;
        let entry = RuntimeV2PrivateStateLineageEntry::new_accepted(
            "lineage-entry-proto-citizen-alpha-0001".to_string(),
            "genesis".to_string(),
            "admission".to_string(),
            state.citizen_id.clone(),
            state.manifold_id.clone(),
            state.lineage_id.clone(),
            state.state_sequence,
            state.predecessor_state_hash.clone(),
            RUNTIME_V2_PRIVATE_STATE_ENVELOPE_PATH.to_string(),
            sealing.sealed_checkpoint.envelope_hash.clone(),
            RUNTIME_V2_PRIVATE_STATE_SEALED_CHECKPOINT_PATH.to_string(),
            checkpoint_hash,
            state.content_hash()?,
            sealing.envelope_artifacts.envelope.writer_identity.clone(),
            None,
            None,
            1,
        )?;
        let entries = vec![entry];
        let accepted_head_entry_hash = entries[0].entry_hash.clone();
        let ledger_root_hash = ledger_root_hash(&entries)?;
        let ledger = Self {
            schema_version: RUNTIME_V2_PRIVATE_STATE_LINEAGE_LEDGER_SCHEMA.to_string(),
            ledger_id: "private-state-lineage-proto-citizen-alpha".to_string(),
            artifact_path: RUNTIME_V2_PRIVATE_STATE_LINEAGE_LEDGER_PATH.to_string(),
            citizen_id: state.citizen_id.clone(),
            manifold_id: state.manifold_id.clone(),
            lineage_id: state.lineage_id.clone(),
            authority_rule:
                "The append-only ledger is the authoritative continuity record; materialized head files are projections that must match the accepted head."
                    .to_string(),
            append_only_rule:
                "Accepted entries form one sequence-linked chain by previous_entry_hash, state_sequence, and predecessor_state_hash; silent truncation, replay, or fork is invalid."
                    .to_string(),
            entries,
            accepted_head_entry_hash,
            ledger_root_hash,
            recovery_policy: vec![
                "materialized head must equal accepted ledger head before activation".to_string(),
                "ledger/head disagreement enters recovery_or_quarantine instead of trusting the convenient copy".to_string(),
                "truncated, replayed, tampered, or forked ledgers fail closed".to_string(),
            ],
            non_claims: vec![
                "does not emit continuity witnesses or citizen-facing receipts".to_string(),
                "does not implement anti-equivocation across multiple signed successors".to_string(),
                "does not claim first true Godel-agent birth".to_string(),
            ],
        };
        ledger.validate_against(sealing)?;
        Ok(ledger)
    }

    pub fn validate_against(&self, sealing: &RuntimeV2PrivateStateSealingArtifacts) -> Result<()> {
        self.validate_shape()?;
        sealing.validate()?;
        let state = &sealing.envelope_artifacts.private_state.canonical_state;
        if self.citizen_id != state.citizen_id
            || self.manifold_id != state.manifold_id
            || self.lineage_id != state.lineage_id
        {
            return Err(anyhow!(
                "private-state lineage ledger identity must match sealed state"
            ));
        }
        let head = self.accepted_head()?;
        if head.canonical_state_hash != state.content_hash()? {
            return Err(anyhow!(
                "private-state lineage ledger accepted head canonical state hash mismatch"
            ));
        }
        if head.sealed_checkpoint_ref != RUNTIME_V2_PRIVATE_STATE_SEALED_CHECKPOINT_PATH {
            return Err(anyhow!(
                "private-state lineage ledger accepted head checkpoint ref mismatch"
            ));
        }
        if head.sealed_checkpoint_hash
            != sha256_json(&sealing.sealed_checkpoint.pretty_json_bytes()?)?
        {
            return Err(anyhow!(
                "private-state lineage ledger accepted head checkpoint hash mismatch"
            ));
        }
        Ok(())
    }

    pub fn validate_shape(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_PRIVATE_STATE_LINEAGE_LEDGER_SCHEMA {
            return Err(anyhow!(
                "unsupported private-state lineage ledger schema '{}'",
                self.schema_version
            ));
        }
        normalize_id(self.ledger_id.clone(), "private_lineage.ledger_id")?;
        validate_relative_path(&self.artifact_path, "private_lineage.artifact_path")?;
        normalize_id(self.citizen_id.clone(), "private_lineage.citizen_id")?;
        normalize_id(self.manifold_id.clone(), "private_lineage.manifold_id")?;
        normalize_id(self.lineage_id.clone(), "private_lineage.lineage_id")?;
        if !self
            .authority_rule
            .contains("authoritative continuity record")
            || !self.authority_rule.contains("materialized head")
        {
            return Err(anyhow!(
                "private-state lineage ledger must define ledger authority over materialized head"
            ));
        }
        if !self.append_only_rule.contains("silent truncation")
            || !self.append_only_rule.contains("fork")
        {
            return Err(anyhow!(
                "private-state lineage ledger must state append-only fork/truncation rule"
            ));
        }
        if self.entries.is_empty() {
            return Err(anyhow!(
                "private-state lineage ledger must contain at least one accepted entry"
            ));
        }
        let mut seen_entry_hashes = BTreeSet::new();
        let mut seen_sequences = BTreeSet::new();
        let mut previous_hash = "genesis".to_string();
        let mut previous_state_hash = "genesis".to_string();
        for (expected_sequence, entry) in (1_u64..).zip(self.entries.iter()) {
            entry.validate()?;
            if entry.citizen_id != self.citizen_id
                || entry.manifold_id != self.manifold_id
                || entry.lineage_id != self.lineage_id
            {
                return Err(anyhow!(
                    "private-state lineage entry identity must match ledger"
                ));
            }
            if entry.entry_hash != entry.computed_hash()? {
                return Err(anyhow!("private-state lineage entry hash mismatch"));
            }
            if entry.previous_entry_hash != previous_hash {
                return Err(anyhow!(
                    "private-state lineage entry previous hash mismatch"
                ));
            }
            if entry.state_sequence != expected_sequence {
                return Err(anyhow!("private-state lineage sequence must be contiguous"));
            }
            if entry.predecessor_state_hash != previous_state_hash {
                return Err(anyhow!(
                    "private-state lineage predecessor state hash mismatch"
                ));
            }
            if !seen_entry_hashes.insert(entry.entry_hash.clone()) {
                return Err(anyhow!(
                    "private-state lineage replayed entry hash detected"
                ));
            }
            if !seen_sequences.insert(entry.state_sequence) {
                return Err(anyhow!("private-state lineage replayed sequence detected"));
            }
            previous_hash = entry.entry_hash.clone();
            previous_state_hash = entry.canonical_state_hash.clone();
        }
        let head = self
            .entries
            .last()
            .ok_or_else(|| anyhow!("private-state lineage ledger missing head"))?;
        if self.accepted_head_entry_hash != head.entry_hash {
            return Err(anyhow!(
                "private-state lineage accepted head must match final append-only entry"
            ));
        }
        if self.ledger_root_hash != ledger_root_hash(&self.entries)? {
            return Err(anyhow!("private-state lineage ledger root hash mismatch"));
        }
        require_text_list(&self.recovery_policy, "private_lineage.recovery_policy", 2)?;
        if !self
            .recovery_policy
            .iter()
            .any(|policy| policy.contains("recovery_or_quarantine"))
        {
            return Err(anyhow!(
                "private-state lineage recovery policy must route disagreement to recovery_or_quarantine"
            ));
        }
        require_text_list(&self.non_claims, "private_lineage.non_claims", 1)
    }

    pub fn accepted_head(&self) -> Result<&RuntimeV2PrivateStateLineageEntry> {
        self.validate_shape()?;
        self.entries
            .last()
            .ok_or_else(|| anyhow!("private-state lineage ledger missing accepted head"))
    }

    pub fn fork_candidates(
        &self,
        candidate: RuntimeV2PrivateStateLineageEntry,
    ) -> Result<Vec<RuntimeV2PrivateStateLineageEntry>> {
        self.validate_shape()?;
        let head = self.accepted_head()?;
        if candidate.previous_entry_hash == head.previous_entry_hash
            && candidate.state_sequence == head.state_sequence
            && candidate.entry_hash != head.entry_hash
        {
            return Err(anyhow!(
                "private-state lineage forked successor for same sequence detected"
            ));
        }
        let mut entries = self.entries.clone();
        entries.push(candidate);
        Ok(entries)
    }

    pub fn pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.validate_shape()?;
        serde_json::to_vec_pretty(self).context("serialize private-state lineage ledger")
    }
}

impl RuntimeV2PrivateStateLineageEntry {
    #[allow(clippy::too_many_arguments)]
    pub fn new_accepted(
        entry_id: String,
        previous_entry_hash: String,
        transition_type: String,
        citizen_id: String,
        manifold_id: String,
        lineage_id: String,
        state_sequence: u64,
        predecessor_state_hash: String,
        envelope_ref: String,
        envelope_hash: String,
        sealed_checkpoint_ref: String,
        sealed_checkpoint_hash: String,
        canonical_state_hash: String,
        writer_identity: String,
        witness_ref: Option<String>,
        receipt_ref: Option<String>,
        recorded_at_logical_tick: u64,
    ) -> Result<Self> {
        let mut entry = Self {
            entry_id,
            entry_hash: String::new(),
            previous_entry_hash,
            transition_type,
            citizen_id,
            manifold_id,
            lineage_id,
            state_sequence,
            predecessor_state_hash,
            envelope_ref,
            envelope_hash,
            sealed_checkpoint_ref,
            sealed_checkpoint_hash,
            canonical_state_hash,
            writer_identity,
            witness_ref,
            receipt_ref,
            recorded_at_logical_tick,
            disposition: "accepted".to_string(),
        };
        entry.entry_hash = entry.computed_hash()?;
        entry.validate()?;
        Ok(entry)
    }

    pub fn computed_hash(&self) -> Result<String> {
        normalize_id(self.entry_id.clone(), "private_lineage.entry_id")?;
        validate_hash_or_genesis(
            &self.previous_entry_hash,
            "private_lineage.previous_entry_hash",
        )?;
        validate_nonempty_text(&self.transition_type, "private_lineage.transition_type")?;
        normalize_id(self.citizen_id.clone(), "private_lineage.citizen_id")?;
        normalize_id(self.manifold_id.clone(), "private_lineage.manifold_id")?;
        normalize_id(self.lineage_id.clone(), "private_lineage.lineage_id")?;
        validate_hash_or_genesis(
            &self.predecessor_state_hash,
            "private_lineage.predecessor_state_hash",
        )?;
        validate_relative_path(&self.envelope_ref, "private_lineage.envelope_ref")?;
        validate_sha256_hash(&self.envelope_hash, "private_lineage.envelope_hash")?;
        validate_relative_path(
            &self.sealed_checkpoint_ref,
            "private_lineage.sealed_checkpoint_ref",
        )?;
        validate_sha256_hash(
            &self.sealed_checkpoint_hash,
            "private_lineage.sealed_checkpoint_hash",
        )?;
        validate_sha256_hash(
            &self.canonical_state_hash,
            "private_lineage.canonical_state_hash",
        )?;
        normalize_id(
            self.writer_identity.clone(),
            "private_lineage.writer_identity",
        )?;
        if let Some(witness_ref) = &self.witness_ref {
            validate_relative_path(witness_ref, "private_lineage.witness_ref")?;
        }
        if let Some(receipt_ref) = &self.receipt_ref {
            validate_relative_path(receipt_ref, "private_lineage.receipt_ref")?;
        }
        if self.recorded_at_logical_tick == 0 {
            return Err(anyhow!(
                "private-state lineage entry logical tick must be positive"
            ));
        }
        if self.disposition != "accepted" {
            return Err(anyhow!(
                "private-state lineage fixture entries must be accepted before becoming head candidates"
            ));
        }
        Ok(sha256_bytes(self.hash_payload()?.as_bytes()))
    }

    pub fn validate(&self) -> Result<()> {
        if self.state_sequence == 0 {
            return Err(anyhow!(
                "private-state lineage entry sequence must be greater than zero"
            ));
        }
        validate_sha256_hash(&self.entry_hash, "private_lineage.entry_hash")?;
        if self.entry_hash != self.computed_hash()? {
            return Err(anyhow!("private-state lineage entry hash mismatch"));
        }
        Ok(())
    }

    fn hash_payload(&self) -> Result<String> {
        Ok(format!(
            "schema={}\nentry_id={}\nprevious_entry_hash={}\ntransition_type={}\ncitizen_id={}\nmanifold_id={}\nlineage_id={}\nstate_sequence={}\npredecessor_state_hash={}\nenvelope_ref={}\nenvelope_hash={}\nsealed_checkpoint_ref={}\nsealed_checkpoint_hash={}\ncanonical_state_hash={}\nwriter_identity={}\nwitness_ref={}\nreceipt_ref={}\nrecorded_at_logical_tick={}\ndisposition={}\n",
            RUNTIME_V2_PRIVATE_STATE_LINEAGE_LEDGER_SCHEMA,
            self.entry_id,
            self.previous_entry_hash,
            self.transition_type,
            self.citizen_id,
            self.manifold_id,
            self.lineage_id,
            self.state_sequence,
            self.predecessor_state_hash,
            self.envelope_ref,
            self.envelope_hash,
            self.sealed_checkpoint_ref,
            self.sealed_checkpoint_hash,
            self.canonical_state_hash,
            self.writer_identity,
            self.witness_ref.clone().unwrap_or_else(|| "deferred_to_wp_07".to_string()),
            self.receipt_ref.clone().unwrap_or_else(|| "deferred_to_wp_07".to_string()),
            self.recorded_at_logical_tick,
            self.disposition,
        ))
    }
}

impl RuntimeV2PrivateStateMaterializedHead {
    pub fn from_ledger_head(
        ledger: &RuntimeV2PrivateStateLineageLedger,
        sealing: &RuntimeV2PrivateStateSealingArtifacts,
    ) -> Result<Self> {
        ledger.validate_against(sealing)?;
        let head = ledger.accepted_head()?;
        let materialized = Self {
            schema_version: RUNTIME_V2_PRIVATE_STATE_MATERIALIZED_HEAD_SCHEMA.to_string(),
            head_id: "materialized-head-proto-citizen-alpha".to_string(),
            artifact_path: RUNTIME_V2_PRIVATE_STATE_MATERIALIZED_HEAD_PATH.to_string(),
            citizen_id: ledger.citizen_id.clone(),
            manifold_id: ledger.manifold_id.clone(),
            lineage_id: ledger.lineage_id.clone(),
            state_sequence: head.state_sequence,
            head_entry_hash: head.entry_hash.clone(),
            ledger_ref: ledger.artifact_path.clone(),
            sealed_checkpoint_ref: head.sealed_checkpoint_ref.clone(),
            sealed_checkpoint_hash: head.sealed_checkpoint_hash.clone(),
            canonical_state_hash: head.canonical_state_hash.clone(),
            status: "materialized_projection_of_accepted_ledger_head".to_string(),
            authority_rule:
                "This file is valid only if it matches the append-only ledger accepted head; disagreement requires recovery_or_quarantine."
                    .to_string(),
        };
        materialized.validate_against_ledger(ledger, sealing)?;
        Ok(materialized)
    }

    pub fn validate_shape(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_PRIVATE_STATE_MATERIALIZED_HEAD_SCHEMA {
            return Err(anyhow!(
                "unsupported private-state materialized head schema '{}'",
                self.schema_version
            ));
        }
        normalize_id(self.head_id.clone(), "private_lineage.head_id")?;
        validate_relative_path(&self.artifact_path, "private_lineage.head_artifact_path")?;
        normalize_id(self.citizen_id.clone(), "private_lineage.head_citizen_id")?;
        normalize_id(self.manifold_id.clone(), "private_lineage.head_manifold_id")?;
        normalize_id(self.lineage_id.clone(), "private_lineage.head_lineage_id")?;
        if self.state_sequence == 0 {
            return Err(anyhow!(
                "private-state materialized head sequence must be greater than zero"
            ));
        }
        validate_sha256_hash(&self.head_entry_hash, "private_lineage.head_entry_hash")?;
        validate_relative_path(&self.ledger_ref, "private_lineage.head_ledger_ref")?;
        validate_relative_path(
            &self.sealed_checkpoint_ref,
            "private_lineage.head_sealed_checkpoint_ref",
        )?;
        validate_sha256_hash(
            &self.sealed_checkpoint_hash,
            "private_lineage.head_sealed_checkpoint_hash",
        )?;
        validate_sha256_hash(
            &self.canonical_state_hash,
            "private_lineage.head_canonical_state_hash",
        )?;
        if self.status != "materialized_projection_of_accepted_ledger_head" {
            return Err(anyhow!(
                "private-state materialized head must declare projection status"
            ));
        }
        if !self.authority_rule.contains("valid only if it matches")
            || !self.authority_rule.contains("recovery_or_quarantine")
        {
            return Err(anyhow!(
                "private-state materialized head must route disagreement to recovery_or_quarantine"
            ));
        }
        Ok(())
    }

    pub fn validate_against_ledger(
        &self,
        ledger: &RuntimeV2PrivateStateLineageLedger,
        sealing: &RuntimeV2PrivateStateSealingArtifacts,
    ) -> Result<()> {
        self.validate_shape()?;
        ledger.validate_against(sealing)?;
        let head = ledger.accepted_head()?;
        if self.citizen_id != ledger.citizen_id
            || self.manifold_id != ledger.manifold_id
            || self.lineage_id != ledger.lineage_id
        {
            return Err(anyhow!(
                "private-state materialized head identity must match ledger"
            ));
        }
        if self.ledger_ref != ledger.artifact_path {
            return Err(anyhow!(
                "private-state materialized head ledger ref mismatch"
            ));
        }
        if self.state_sequence != head.state_sequence
            || self.head_entry_hash != ledger.accepted_head_entry_hash
            || self.head_entry_hash != head.entry_hash
            || self.sealed_checkpoint_ref != head.sealed_checkpoint_ref
            || self.sealed_checkpoint_hash != head.sealed_checkpoint_hash
            || self.canonical_state_hash != head.canonical_state_hash
        {
            return Err(anyhow!(
                "private-state ledger/head disagreement requires recovery_or_quarantine"
            ));
        }
        Ok(())
    }

    pub fn disposition_against_ledger(
        &self,
        ledger: &RuntimeV2PrivateStateLineageLedger,
        sealing: &RuntimeV2PrivateStateSealingArtifacts,
    ) -> Result<RuntimeV2PrivateStateLineageDisposition> {
        if self.validate_against_ledger(ledger, sealing).is_ok() {
            return RuntimeV2PrivateStateLineageDisposition::accepted(self, ledger);
        }
        RuntimeV2PrivateStateLineageDisposition::recovery_or_quarantine(self, ledger)
    }

    pub fn pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.validate_shape()?;
        serde_json::to_vec_pretty(self).context("serialize private-state materialized head")
    }
}

impl RuntimeV2PrivateStateLineageProof {
    pub fn prototype() -> Self {
        Self {
            schema_version: RUNTIME_V2_PRIVATE_STATE_LINEAGE_PROOF_SCHEMA.to_string(),
            proof_id: "v0-90-3-wp-06-lineage-negative-cases".to_string(),
            demo_id: "D5".to_string(),
            ledger_ref: RUNTIME_V2_PRIVATE_STATE_LINEAGE_LEDGER_PATH.to_string(),
            materialized_head_ref: RUNTIME_V2_PRIVATE_STATE_MATERIALIZED_HEAD_PATH.to_string(),
            sealed_checkpoint_ref: RUNTIME_V2_PRIVATE_STATE_SEALED_CHECKPOINT_PATH.to_string(),
            required_negative_cases: vec![
                lineage_negative_case(
                    "tampered_entry",
                    "mutate accepted entry canonical state hash without recomputing entry hash",
                    "entry hash mismatch",
                ),
                lineage_negative_case(
                    "truncated_ledger",
                    "remove the only accepted head while retaining accepted_head_entry_hash",
                    "at least one accepted entry",
                ),
                lineage_negative_case(
                    "forked_successor",
                    "present conflicting successor for the same sequence position",
                    "forked successor",
                ),
                lineage_negative_case(
                    "replayed_entry",
                    "append an already accepted entry again",
                    "previous hash mismatch",
                ),
                lineage_negative_case(
                    "head_disagreement",
                    "change materialized head hash away from ledger accepted head",
                    "recovery_or_quarantine",
                ),
            ],
            validation_command:
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_private_state_lineage -- --nocapture"
                    .to_string(),
            claim_boundary:
                "D5 proves append-only lineage replay, accepted-head calculation, and recovery_or_quarantine disposition for head disagreement; witnesses, receipts, and anti-equivocation are later WPs."
                    .to_string(),
        }
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_PRIVATE_STATE_LINEAGE_PROOF_SCHEMA {
            return Err(anyhow!(
                "unsupported private-state lineage proof schema '{}'",
                self.schema_version
            ));
        }
        normalize_id(self.proof_id.clone(), "private_lineage_proof.proof_id")?;
        if self.demo_id != "D5" {
            return Err(anyhow!(
                "private-state lineage proof must map to demo matrix row D5"
            ));
        }
        validate_relative_path(&self.ledger_ref, "private_lineage_proof.ledger_ref")?;
        validate_relative_path(
            &self.materialized_head_ref,
            "private_lineage_proof.materialized_head_ref",
        )?;
        validate_relative_path(
            &self.sealed_checkpoint_ref,
            "private_lineage_proof.sealed_checkpoint_ref",
        )?;
        let required = [
            "tampered_entry",
            "truncated_ledger",
            "forked_successor",
            "replayed_entry",
            "head_disagreement",
        ];
        for required_case in required {
            if !self
                .required_negative_cases
                .iter()
                .any(|case| case.case_id == required_case)
            {
                return Err(anyhow!(
                    "private-state lineage proof missing negative case '{required_case}'"
                ));
            }
        }
        for case in &self.required_negative_cases {
            case.validate()?;
        }
        if !self
            .validation_command
            .contains("runtime_v2_private_state_lineage")
        {
            return Err(anyhow!(
                "private-state lineage proof must include focused validation command"
            ));
        }
        if !self.claim_boundary.contains("append-only lineage replay")
            || !self.claim_boundary.contains("later WPs")
        {
            return Err(anyhow!(
                "private-state lineage proof must preserve WP boundary"
            ));
        }
        Ok(())
    }

    pub fn pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.validate()?;
        serde_json::to_vec_pretty(self).context("serialize private-state lineage proof")
    }
}

impl RuntimeV2PrivateStateLineageNegativeCase {
    pub fn validate(&self) -> Result<()> {
        normalize_id(self.case_id.clone(), "private_lineage_proof.case_id")?;
        validate_nonempty_text(&self.mutation, "private_lineage_proof.mutation")?;
        validate_nonempty_text(
            &self.expected_error_fragment,
            "private_lineage_proof.expected_error_fragment",
        )
    }
}

impl RuntimeV2PrivateStateLineageDisposition {
    fn accepted(
        head: &RuntimeV2PrivateStateMaterializedHead,
        ledger: &RuntimeV2PrivateStateLineageLedger,
    ) -> Result<Self> {
        let disposition = Self {
            schema_version: RUNTIME_V2_PRIVATE_STATE_LINEAGE_DISPOSITION_SCHEMA.to_string(),
            disposition_id: "lineage-head-accepted".to_string(),
            citizen_id: head.citizen_id.clone(),
            lineage_id: head.lineage_id.clone(),
            disposition: "accepted".to_string(),
            reason: "materialized head matches append-only ledger accepted head".to_string(),
            evidence_refs: vec![ledger.artifact_path.clone(), head.artifact_path.clone()],
            required_next_step: "activation_allowed_for_this_bounded_fixture".to_string(),
        };
        disposition.validate()?;
        Ok(disposition)
    }

    fn recovery_or_quarantine(
        head: &RuntimeV2PrivateStateMaterializedHead,
        ledger: &RuntimeV2PrivateStateLineageLedger,
    ) -> Result<Self> {
        let disposition = Self {
            schema_version: RUNTIME_V2_PRIVATE_STATE_LINEAGE_DISPOSITION_SCHEMA.to_string(),
            disposition_id: "lineage-head-disagreement".to_string(),
            citizen_id: head.citizen_id.clone(),
            lineage_id: head.lineage_id.clone(),
            disposition: "recovery_or_quarantine".to_string(),
            reason:
                "materialized head does not match append-only ledger accepted head; do not trust the convenient copy"
                    .to_string(),
            evidence_refs: vec![ledger.artifact_path.clone(), head.artifact_path.clone()],
            required_next_step:
                "reconstruct_from_ledger_or_quarantine_before_any_wake_or_activation"
                    .to_string(),
        };
        disposition.validate()?;
        Ok(disposition)
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_PRIVATE_STATE_LINEAGE_DISPOSITION_SCHEMA {
            return Err(anyhow!(
                "unsupported private-state lineage disposition schema '{}'",
                self.schema_version
            ));
        }
        normalize_id(
            self.disposition_id.clone(),
            "private_lineage_disposition.disposition_id",
        )?;
        normalize_id(
            self.citizen_id.clone(),
            "private_lineage_disposition.citizen_id",
        )?;
        normalize_id(
            self.lineage_id.clone(),
            "private_lineage_disposition.lineage_id",
        )?;
        match self.disposition.as_str() {
            "accepted" | "recovery_or_quarantine" => {}
            other => {
                return Err(anyhow!(
                    "unsupported private-state lineage disposition '{other}'"
                ))
            }
        }
        validate_nonempty_text(&self.reason, "private_lineage_disposition.reason")?;
        require_text_list(
            &self.evidence_refs,
            "private_lineage_disposition.evidence_refs",
            2,
        )?;
        for evidence_ref in &self.evidence_refs {
            validate_relative_path(evidence_ref, "private_lineage_disposition.evidence_ref")?;
        }
        validate_nonempty_text(
            &self.required_next_step,
            "private_lineage_disposition.required_next_step",
        )
    }
}

fn lineage_negative_case(
    case_id: &str,
    mutation: &str,
    expected_error_fragment: &str,
) -> RuntimeV2PrivateStateLineageNegativeCase {
    RuntimeV2PrivateStateLineageNegativeCase {
        case_id: case_id.to_string(),
        mutation: mutation.to_string(),
        expected_error_fragment: expected_error_fragment.to_string(),
    }
}

fn ledger_root_hash(entries: &[RuntimeV2PrivateStateLineageEntry]) -> Result<String> {
    if entries.is_empty() {
        return Err(anyhow!(
            "private-state lineage ledger root requires entries"
        ));
    }
    let mut payload = String::new();
    for entry in entries {
        payload.push_str(&entry.entry_hash);
        payload.push('\n');
    }
    Ok(sha256_bytes(payload.as_bytes()))
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

fn sha256_json(bytes: &[u8]) -> Result<String> {
    if bytes.is_empty() {
        return Err(anyhow!("json bytes must not be empty"));
    }
    Ok(sha256_bytes(bytes))
}

fn sha256_bytes(bytes: &[u8]) -> String {
    format!("sha256:{:x}", Sha256::digest(bytes))
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
