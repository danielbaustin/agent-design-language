use super::*;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;

pub const RUNTIME_V2_PRIVATE_STATE_ANTI_EQUIVOCATION_CONFLICT_SCHEMA: &str =
    "runtime_v2.private_state_anti_equivocation_conflict.v1";
pub const RUNTIME_V2_PRIVATE_STATE_ANTI_EQUIVOCATION_DISPOSITION_SCHEMA: &str =
    "runtime_v2.private_state_anti_equivocation_disposition.v1";
pub const RUNTIME_V2_PRIVATE_STATE_ANTI_EQUIVOCATION_PROOF_SCHEMA: &str =
    "runtime_v2.private_state_anti_equivocation_proof.v1";
pub const RUNTIME_V2_PRIVATE_STATE_ANTI_EQUIVOCATION_CONFLICT_PATH: &str =
    "runtime_v2/private_state/anti_equivocation_conflict.json";
pub const RUNTIME_V2_PRIVATE_STATE_ANTI_EQUIVOCATION_DISPOSITION_PATH: &str =
    "runtime_v2/private_state/anti_equivocation_disposition.json";
pub const RUNTIME_V2_PRIVATE_STATE_ANTI_EQUIVOCATION_PROOF_PATH: &str =
    "runtime_v2/private_state/anti_equivocation_negative_cases.json";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2PrivateStateAntiEquivocationConflict {
    pub schema_version: String,
    pub conflict_id: String,
    pub artifact_path: String,
    pub conflict_kind: String,
    pub citizen_id: String,
    pub manifold_id: String,
    pub lineage_id: String,
    pub ledger_ref: String,
    pub accepted_head_entry_hash: String,
    pub current_head_sequence: u64,
    pub contested_sequence: u64,
    pub predecessor_entry_hash: String,
    pub predecessor_state_hash: String,
    pub witness_set_ref: String,
    pub receipt_set_ref: String,
    pub attempted_active_candidate_ids: Vec<String>,
    pub candidates: Vec<RuntimeV2PrivateStateEquivocationCandidate>,
    pub activation_rule: String,
    pub evidence_preservation_rule: String,
    pub conflict_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2PrivateStateEquivocationCandidate {
    pub candidate_id: String,
    pub entry_id: String,
    pub entry_hash: String,
    pub transition_type: String,
    pub previous_entry_hash: String,
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
    pub witness_ref: String,
    pub witness_hash: String,
    pub receipt_ref: String,
    pub receipt_hash: String,
    pub signature_key_id: String,
    pub signature_algorithm: String,
    pub writer_identity: String,
    pub claim_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2PrivateStateAntiEquivocationDisposition {
    pub schema_version: String,
    pub disposition_id: String,
    pub artifact_path: String,
    pub conflict_ref: String,
    pub conflict_hash: String,
    pub disposition: String,
    pub activation_allowed: bool,
    pub active_candidate_id: Option<String>,
    pub reason: String,
    pub evidence_refs: Vec<String>,
    pub preserved_candidate_ids: Vec<String>,
    pub preserved_candidate_entry_hashes: Vec<String>,
    pub preserved_candidate_claim_hashes: Vec<String>,
    pub destructive_transition_policy: String,
    pub review_route: String,
    pub disposition_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2PrivateStateAntiEquivocationNegativeCase {
    pub case_id: String,
    pub mutation: String,
    pub expected_error_fragment: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2PrivateStateAntiEquivocationProof {
    pub schema_version: String,
    pub proof_id: String,
    pub demo_id: String,
    pub conflict_ref: String,
    pub disposition_ref: String,
    pub ledger_ref: String,
    pub witness_set_ref: String,
    pub required_negative_cases: Vec<RuntimeV2PrivateStateAntiEquivocationNegativeCase>,
    pub validation_command: String,
    pub claim_boundary: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeV2PrivateStateAntiEquivocationArtifacts {
    pub witness_artifacts: RuntimeV2PrivateStateWitnessArtifacts,
    pub conflict: RuntimeV2PrivateStateAntiEquivocationConflict,
    pub disposition: RuntimeV2PrivateStateAntiEquivocationDisposition,
    pub negative_cases: RuntimeV2PrivateStateAntiEquivocationProof,
}

impl RuntimeV2PrivateStateAntiEquivocationArtifacts {
    pub fn prototype() -> Result<Self> {
        let witness_artifacts = RuntimeV2PrivateStateWitnessArtifacts::prototype()?;
        let conflict =
            RuntimeV2PrivateStateAntiEquivocationConflict::prototype(&witness_artifacts)?;
        let disposition =
            RuntimeV2PrivateStateAntiEquivocationDisposition::from_conflict(&conflict)?;
        let negative_cases = RuntimeV2PrivateStateAntiEquivocationProof::prototype();
        let artifacts = Self {
            witness_artifacts,
            conflict,
            disposition,
            negative_cases,
        };
        artifacts.validate()?;
        Ok(artifacts)
    }

    pub fn validate(&self) -> Result<()> {
        self.witness_artifacts.validate()?;
        self.conflict.validate_against(&self.witness_artifacts)?;
        self.disposition.validate_against(&self.conflict)?;
        self.negative_cases
            .validate_against(&self.conflict, &self.disposition)
    }

    pub fn write_to_root(&self, root: impl AsRef<Path>) -> Result<()> {
        self.validate()?;
        let root = root.as_ref();
        write_relative(
            root,
            RUNTIME_V2_PRIVATE_STATE_ANTI_EQUIVOCATION_CONFLICT_PATH,
            self.conflict.pretty_json_bytes()?,
        )?;
        write_relative(
            root,
            RUNTIME_V2_PRIVATE_STATE_ANTI_EQUIVOCATION_DISPOSITION_PATH,
            self.disposition.pretty_json_bytes()?,
        )?;
        write_relative(
            root,
            RUNTIME_V2_PRIVATE_STATE_ANTI_EQUIVOCATION_PROOF_PATH,
            self.negative_cases.pretty_json_bytes()?,
        )?;
        Ok(())
    }
}

impl RuntimeV2PrivateStateAntiEquivocationConflict {
    pub fn prototype(witness: &RuntimeV2PrivateStateWitnessArtifacts) -> Result<Self> {
        witness.validate()?;
        let ledger = &witness.witness_artifacts_lineage().ledger;
        let head = ledger.accepted_head()?;
        let candidates = vec![
            RuntimeV2PrivateStateEquivocationCandidate::prototype(
                "candidate-alpha-snapshot",
                "snapshot",
                "a",
                head,
                ledger,
                &witness.witness_set,
                &witness.receipt_set,
            )?,
            RuntimeV2PrivateStateEquivocationCandidate::prototype(
                "candidate-alpha-wake",
                "wake",
                "b",
                head,
                ledger,
                &witness.witness_set,
                &witness.receipt_set,
            )?,
        ];
        let attempted_active_candidate_ids = candidates
            .iter()
            .map(|candidate| candidate.candidate_id.clone())
            .collect::<Vec<_>>();
        let mut conflict = Self {
            schema_version: RUNTIME_V2_PRIVATE_STATE_ANTI_EQUIVOCATION_CONFLICT_SCHEMA
                .to_string(),
            conflict_id: "anti-equivocation-conflict-proto-citizen-alpha-0002".to_string(),
            artifact_path: RUNTIME_V2_PRIVATE_STATE_ANTI_EQUIVOCATION_CONFLICT_PATH.to_string(),
            conflict_kind: "conflicting_signed_successors".to_string(),
            citizen_id: ledger.citizen_id.clone(),
            manifold_id: ledger.manifold_id.clone(),
            lineage_id: ledger.lineage_id.clone(),
            ledger_ref: ledger.artifact_path.clone(),
            accepted_head_entry_hash: ledger.accepted_head_entry_hash.clone(),
            current_head_sequence: head.state_sequence,
            contested_sequence: head.state_sequence + 1,
            predecessor_entry_hash: head.entry_hash.clone(),
            predecessor_state_hash: head.canonical_state_hash.clone(),
            witness_set_ref: witness.witness_set.artifact_path.clone(),
            receipt_set_ref: witness.receipt_set.artifact_path.clone(),
            attempted_active_candidate_ids,
            candidates,
            activation_rule:
                "Conflicting signed successors for the same citizen, lineage, predecessor, and sequence cannot both become active."
                    .to_string(),
            evidence_preservation_rule:
                "All conflicting successor claims, witness refs, receipt refs, and hashes must be preserved for sanctuary or quarantine review."
                    .to_string(),
            conflict_hash: String::new(),
        };
        conflict.conflict_hash = conflict.computed_hash()?;
        conflict.validate_against(witness)?;
        Ok(conflict)
    }

    pub fn validate_against(&self, witness: &RuntimeV2PrivateStateWitnessArtifacts) -> Result<()> {
        self.validate_shape()?;
        witness.validate()?;
        let ledger = &witness.witness_artifacts_lineage().ledger;
        let head = ledger.accepted_head()?;
        if self.citizen_id != ledger.citizen_id
            || self.manifold_id != ledger.manifold_id
            || self.lineage_id != ledger.lineage_id
            || self.ledger_ref != ledger.artifact_path
            || self.accepted_head_entry_hash != ledger.accepted_head_entry_hash
            || self.current_head_sequence != head.state_sequence
            || self.contested_sequence != head.state_sequence + 1
            || self.predecessor_entry_hash != head.entry_hash
            || self.predecessor_state_hash != head.canonical_state_hash
            || self.witness_set_ref != witness.witness_set.artifact_path
            || self.receipt_set_ref != witness.receipt_set.artifact_path
        {
            return Err(anyhow!(
                "private-state anti-equivocation conflict must bind to current ledger head"
            ));
        }
        self.detect_conflicting_successors()?;
        Ok(())
    }

    pub fn validate_shape(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_PRIVATE_STATE_ANTI_EQUIVOCATION_CONFLICT_SCHEMA {
            return Err(anyhow!(
                "unsupported private-state anti-equivocation conflict schema '{}'",
                self.schema_version
            ));
        }
        normalize_id(
            self.conflict_id.clone(),
            "private_anti_equivocation.conflict_id",
        )?;
        validate_relative_path(
            &self.artifact_path,
            "private_anti_equivocation.conflict_artifact_path",
        )?;
        if self.conflict_kind != "conflicting_signed_successors" {
            return Err(anyhow!(
                "private-state anti-equivocation conflict kind must be conflicting_signed_successors"
            ));
        }
        normalize_id(
            self.citizen_id.clone(),
            "private_anti_equivocation.citizen_id",
        )?;
        normalize_id(
            self.manifold_id.clone(),
            "private_anti_equivocation.manifold_id",
        )?;
        normalize_id(
            self.lineage_id.clone(),
            "private_anti_equivocation.lineage_id",
        )?;
        validate_relative_path(&self.ledger_ref, "private_anti_equivocation.ledger_ref")?;
        validate_sha256_hash(
            &self.accepted_head_entry_hash,
            "private_anti_equivocation.accepted_head_entry_hash",
        )?;
        if self.current_head_sequence == 0
            || self.contested_sequence != self.current_head_sequence + 1
        {
            return Err(anyhow!(
                "private-state anti-equivocation contested sequence must be the next successor"
            ));
        }
        validate_sha256_hash(
            &self.predecessor_entry_hash,
            "private_anti_equivocation.predecessor_entry_hash",
        )?;
        validate_sha256_hash(
            &self.predecessor_state_hash,
            "private_anti_equivocation.predecessor_state_hash",
        )?;
        validate_relative_path(
            &self.witness_set_ref,
            "private_anti_equivocation.witness_set_ref",
        )?;
        validate_relative_path(
            &self.receipt_set_ref,
            "private_anti_equivocation.receipt_set_ref",
        )?;
        if self.candidates.len() < 2 {
            return Err(anyhow!(
                "private-state anti-equivocation conflict requires at least two candidates"
            ));
        }
        require_text_list(
            &self.attempted_active_candidate_ids,
            "private_anti_equivocation.attempted_active_candidate_ids",
            2,
        )?;
        if !self.activation_rule.contains("cannot both become active") {
            return Err(anyhow!(
                "private-state anti-equivocation activation rule must forbid dual activation"
            ));
        }
        if !self.evidence_preservation_rule.contains("preserved") {
            return Err(anyhow!(
                "private-state anti-equivocation conflict must preserve evidence"
            ));
        }
        for candidate in &self.candidates {
            candidate.validate_shape()?;
        }
        validate_sha256_hash(
            &self.conflict_hash,
            "private_anti_equivocation.conflict_hash",
        )?;
        if self.conflict_hash != self.computed_hash()? {
            return Err(anyhow!(
                "private-state anti-equivocation conflict hash mismatch"
            ));
        }
        Ok(())
    }

    pub fn detect_conflicting_successors(&self) -> Result<()> {
        let mut sequence_positions = BTreeSet::new();
        let mut claim_hashes = BTreeSet::new();
        for candidate in &self.candidates {
            if candidate.citizen_id != self.citizen_id
                || candidate.manifold_id != self.manifold_id
                || candidate.lineage_id != self.lineage_id
                || candidate.state_sequence != self.contested_sequence
                || candidate.previous_entry_hash != self.predecessor_entry_hash
                || candidate.predecessor_state_hash != self.predecessor_state_hash
            {
                return Err(anyhow!(
                    "private-state anti-equivocation candidate does not target the contested successor position"
                ));
            }
            sequence_positions.insert(format!(
                "{}:{}:{}:{}:{}",
                candidate.citizen_id,
                candidate.lineage_id,
                candidate.previous_entry_hash,
                candidate.state_sequence,
                candidate.predecessor_state_hash
            ));
            claim_hashes.insert(candidate.claim_hash.clone());
        }
        if sequence_positions.len() != 1 || claim_hashes.len() < 2 {
            return Err(anyhow!(
                "private-state anti-equivocation fixture must contain conflicting signed successors"
            ));
        }
        Ok(())
    }

    pub fn validate_activation_attempt(&self, active_candidate_ids: &[String]) -> Result<()> {
        self.validate_shape()?;
        if active_candidate_ids.len() <= 1 {
            return Ok(());
        }
        let known_ids = self
            .candidates
            .iter()
            .map(|candidate| candidate.candidate_id.clone())
            .collect::<BTreeSet<_>>();
        for active_id in active_candidate_ids {
            if !known_ids.contains(active_id) {
                return Err(anyhow!(
                    "private-state anti-equivocation activation references unknown candidate"
                ));
            }
        }
        Err(anyhow!(
            "private-state anti-equivocation: conflicting signed successors for the same sequence cannot both become active"
        ))
    }

    pub fn computed_hash(&self) -> Result<String> {
        normalize_id(
            self.conflict_id.clone(),
            "private_anti_equivocation.conflict_id",
        )?;
        Ok(sha256_bytes(self.hash_payload()?.as_bytes()))
    }

    pub fn pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.validate_shape()?;
        serde_json::to_vec_pretty(self)
            .context("serialize private-state anti-equivocation conflict")
    }

    fn hash_payload(&self) -> Result<String> {
        let candidate_hashes = self
            .candidates
            .iter()
            .map(|candidate| candidate.claim_hash.clone())
            .collect::<Vec<_>>()
            .join("|");
        Ok(format!(
            "schema={}\nconflict_id={}\nconflict_kind={}\ncitizen_id={}\nmanifold_id={}\nlineage_id={}\nledger_ref={}\naccepted_head_entry_hash={}\ncurrent_head_sequence={}\ncontested_sequence={}\npredecessor_entry_hash={}\npredecessor_state_hash={}\nwitness_set_ref={}\nreceipt_set_ref={}\nattempted_active_candidate_ids={}\ncandidate_hashes={}\nactivation_rule={}\nevidence_preservation_rule={}\n",
            RUNTIME_V2_PRIVATE_STATE_ANTI_EQUIVOCATION_CONFLICT_SCHEMA,
            self.conflict_id,
            self.conflict_kind,
            self.citizen_id,
            self.manifold_id,
            self.lineage_id,
            self.ledger_ref,
            self.accepted_head_entry_hash,
            self.current_head_sequence,
            self.contested_sequence,
            self.predecessor_entry_hash,
            self.predecessor_state_hash,
            self.witness_set_ref,
            self.receipt_set_ref,
            self.attempted_active_candidate_ids.join("|"),
            candidate_hashes,
            self.activation_rule,
            self.evidence_preservation_rule,
        ))
    }
}

impl RuntimeV2PrivateStateEquivocationCandidate {
    #[allow(clippy::too_many_arguments)]
    pub fn prototype(
        candidate_id: &str,
        transition_type: &str,
        suffix: &str,
        head: &RuntimeV2PrivateStateLineageEntry,
        ledger: &RuntimeV2PrivateStateLineageLedger,
        witness_set: &RuntimeV2PrivateStateContinuityWitnessSet,
        receipt_set: &RuntimeV2PrivateStateCitizenReceiptSet,
    ) -> Result<Self> {
        let witness = witness_set.find_transition(transition_type)?;
        let receipt = receipt_set
            .receipts
            .iter()
            .find(|receipt| receipt.transition_type == transition_type)
            .ok_or_else(|| anyhow!("private-state anti-equivocation receipt transition missing"))?;
        let canonical_state_hash = sha256_label(&format!("{candidate_id}:canonical-state"));
        let envelope_hash = sha256_label(&format!("{candidate_id}:signed-envelope"));
        let sealed_checkpoint_hash = sha256_label(&format!("{candidate_id}:sealed-checkpoint"));
        let mut candidate = Self {
            candidate_id: candidate_id.to_string(),
            entry_id: format!("lineage-entry-proto-citizen-alpha-0002-{suffix}"),
            entry_hash: String::new(),
            transition_type: transition_type.to_string(),
            previous_entry_hash: head.entry_hash.clone(),
            citizen_id: ledger.citizen_id.clone(),
            manifold_id: ledger.manifold_id.clone(),
            lineage_id: ledger.lineage_id.clone(),
            state_sequence: head.state_sequence + 1,
            predecessor_state_hash: head.canonical_state_hash.clone(),
            envelope_ref: format!(
                "runtime_v2/private_state/conflicts/{candidate_id}.envelope.json"
            ),
            envelope_hash,
            sealed_checkpoint_ref: format!(
                "runtime_v2/private_state/conflicts/{candidate_id}.sealed-checkpoint.json"
            ),
            sealed_checkpoint_hash,
            canonical_state_hash,
            witness_ref: witness_set.artifact_path.clone(),
            witness_hash: witness.witness_hash.clone(),
            receipt_ref: receipt_set.artifact_path.clone(),
            receipt_hash: receipt.receipt_hash.clone(),
            signature_key_id: "local-trust-root-proto-ed25519".to_string(),
            signature_algorithm: "ed25519".to_string(),
            writer_identity: head.writer_identity.clone(),
            claim_hash: String::new(),
        };
        candidate.entry_hash = candidate.computed_entry_hash()?;
        candidate.claim_hash = candidate.computed_claim_hash()?;
        candidate.validate_shape()?;
        Ok(candidate)
    }

    pub fn validate_shape(&self) -> Result<()> {
        normalize_id(
            self.candidate_id.clone(),
            "private_anti_equivocation.candidate_id",
        )?;
        normalize_id(self.entry_id.clone(), "private_anti_equivocation.entry_id")?;
        validate_sha256_hash(&self.entry_hash, "private_anti_equivocation.entry_hash")?;
        match self.transition_type.as_str() {
            "snapshot" | "wake" | "quarantine" | "release-from-quarantine" => {}
            other => {
                return Err(anyhow!(
                    "unsupported private-state anti-equivocation transition '{other}'"
                ))
            }
        }
        validate_sha256_hash(
            &self.previous_entry_hash,
            "private_anti_equivocation.previous_entry_hash",
        )?;
        normalize_id(
            self.citizen_id.clone(),
            "private_anti_equivocation.candidate_citizen_id",
        )?;
        normalize_id(
            self.manifold_id.clone(),
            "private_anti_equivocation.candidate_manifold_id",
        )?;
        normalize_id(
            self.lineage_id.clone(),
            "private_anti_equivocation.candidate_lineage_id",
        )?;
        if self.state_sequence == 0 {
            return Err(anyhow!(
                "private-state anti-equivocation candidate sequence must be positive"
            ));
        }
        validate_sha256_hash(
            &self.predecessor_state_hash,
            "private_anti_equivocation.predecessor_state_hash",
        )?;
        validate_relative_path(&self.envelope_ref, "private_anti_equivocation.envelope_ref")?;
        validate_sha256_hash(
            &self.envelope_hash,
            "private_anti_equivocation.envelope_hash",
        )?;
        validate_relative_path(
            &self.sealed_checkpoint_ref,
            "private_anti_equivocation.sealed_checkpoint_ref",
        )?;
        validate_sha256_hash(
            &self.sealed_checkpoint_hash,
            "private_anti_equivocation.sealed_checkpoint_hash",
        )?;
        validate_sha256_hash(
            &self.canonical_state_hash,
            "private_anti_equivocation.canonical_state_hash",
        )?;
        validate_relative_path(&self.witness_ref, "private_anti_equivocation.witness_ref")?;
        validate_sha256_hash(&self.witness_hash, "private_anti_equivocation.witness_hash")?;
        validate_relative_path(&self.receipt_ref, "private_anti_equivocation.receipt_ref")?;
        validate_sha256_hash(&self.receipt_hash, "private_anti_equivocation.receipt_hash")?;
        normalize_id(
            self.signature_key_id.clone(),
            "private_anti_equivocation.signature_key_id",
        )?;
        if self.signature_algorithm != "ed25519" {
            return Err(anyhow!(
                "private-state anti-equivocation candidate must be signed with ed25519 fixture algorithm"
            ));
        }
        normalize_id(
            self.writer_identity.clone(),
            "private_anti_equivocation.writer_identity",
        )?;
        validate_sha256_hash(&self.claim_hash, "private_anti_equivocation.claim_hash")?;
        if self.entry_hash != self.computed_entry_hash()? {
            return Err(anyhow!(
                "private-state anti-equivocation candidate entry hash mismatch"
            ));
        }
        if self.claim_hash != self.computed_claim_hash()? {
            return Err(anyhow!(
                "private-state anti-equivocation candidate claim hash mismatch"
            ));
        }
        Ok(())
    }

    pub fn computed_entry_hash(&self) -> Result<String> {
        Ok(sha256_bytes(self.entry_hash_payload()?.as_bytes()))
    }

    pub fn computed_claim_hash(&self) -> Result<String> {
        Ok(sha256_bytes(self.claim_hash_payload()?.as_bytes()))
    }

    fn entry_hash_payload(&self) -> Result<String> {
        Ok(format!(
            "schema={}\nentry_id={}\nprevious_entry_hash={}\ntransition_type={}\ncitizen_id={}\nmanifold_id={}\nlineage_id={}\nstate_sequence={}\npredecessor_state_hash={}\nenvelope_ref={}\nenvelope_hash={}\nsealed_checkpoint_ref={}\nsealed_checkpoint_hash={}\ncanonical_state_hash={}\nwriter_identity={}\nwitness_ref={}\nreceipt_ref={}\ndisposition=candidate_only_not_active\n",
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
            self.witness_ref,
            self.receipt_ref,
        ))
    }

    fn claim_hash_payload(&self) -> Result<String> {
        Ok(format!(
            "candidate_id={}\nentry_hash={}\nsignature_key_id={}\nsignature_algorithm={}\nwitness_hash={}\nreceipt_hash={}\n",
            self.candidate_id,
            self.entry_hash,
            self.signature_key_id,
            self.signature_algorithm,
            self.witness_hash,
            self.receipt_hash,
        ))
    }
}

impl RuntimeV2PrivateStateAntiEquivocationDisposition {
    pub fn from_conflict(conflict: &RuntimeV2PrivateStateAntiEquivocationConflict) -> Result<Self> {
        conflict.validate_shape()?;
        conflict.detect_conflicting_successors()?;
        let preserved_candidate_ids = conflict
            .candidates
            .iter()
            .map(|candidate| candidate.candidate_id.clone())
            .collect::<Vec<_>>();
        let preserved_candidate_entry_hashes = conflict
            .candidates
            .iter()
            .map(|candidate| candidate.entry_hash.clone())
            .collect::<Vec<_>>();
        let preserved_candidate_claim_hashes = conflict
            .candidates
            .iter()
            .map(|candidate| candidate.claim_hash.clone())
            .collect::<Vec<_>>();
        let mut disposition = Self {
            schema_version: RUNTIME_V2_PRIVATE_STATE_ANTI_EQUIVOCATION_DISPOSITION_SCHEMA
                .to_string(),
            disposition_id: "anti-equivocation-disposition-proto-citizen-alpha-0002".to_string(),
            artifact_path: RUNTIME_V2_PRIVATE_STATE_ANTI_EQUIVOCATION_DISPOSITION_PATH
                .to_string(),
            conflict_ref: conflict.artifact_path.clone(),
            conflict_hash: conflict.conflict_hash.clone(),
            disposition: "sanctuary_or_quarantine".to_string(),
            activation_allowed: false,
            active_candidate_id: None,
            reason:
                "Two signed successor claims target the same citizen lineage, predecessor, and sequence; activation stops and evidence is preserved for review."
                    .to_string(),
            evidence_refs: vec![
                conflict.ledger_ref.clone(),
                conflict.witness_set_ref.clone(),
                conflict.receipt_set_ref.clone(),
                conflict.artifact_path.clone(),
            ],
            preserved_candidate_ids,
            preserved_candidate_entry_hashes,
            preserved_candidate_claim_hashes,
            destructive_transition_policy:
                "block_activation_and_preserve_evidence_until_review".to_string(),
            review_route: "sanctuary_or_quarantine_operator_review".to_string(),
            disposition_hash: String::new(),
        };
        disposition.disposition_hash = disposition.computed_hash()?;
        disposition.validate_against(conflict)?;
        Ok(disposition)
    }

    pub fn validate_against(
        &self,
        conflict: &RuntimeV2PrivateStateAntiEquivocationConflict,
    ) -> Result<()> {
        self.validate_shape()?;
        if self.conflict_ref != conflict.artifact_path
            || self.conflict_hash != conflict.conflict_hash
        {
            return Err(anyhow!(
                "private-state anti-equivocation disposition conflict ref mismatch"
            ));
        }
        if self.activation_allowed || self.active_candidate_id.is_some() {
            return Err(anyhow!(
                "private-state anti-equivocation disposition must block activation"
            ));
        }
        let candidate_ids = conflict
            .candidates
            .iter()
            .map(|candidate| candidate.candidate_id.clone())
            .collect::<BTreeSet<_>>();
        let entry_hashes = conflict
            .candidates
            .iter()
            .map(|candidate| candidate.entry_hash.clone())
            .collect::<BTreeSet<_>>();
        let claim_hashes = conflict
            .candidates
            .iter()
            .map(|candidate| candidate.claim_hash.clone())
            .collect::<BTreeSet<_>>();
        if self
            .preserved_candidate_ids
            .iter()
            .cloned()
            .collect::<BTreeSet<_>>()
            != candidate_ids
            || self
                .preserved_candidate_entry_hashes
                .iter()
                .cloned()
                .collect::<BTreeSet<_>>()
                != entry_hashes
            || self
                .preserved_candidate_claim_hashes
                .iter()
                .cloned()
                .collect::<BTreeSet<_>>()
                != claim_hashes
        {
            return Err(anyhow!(
                "private-state anti-equivocation disposition must preserve all candidate evidence"
            ));
        }
        for required_ref in [
            conflict.ledger_ref.as_str(),
            conflict.witness_set_ref.as_str(),
            conflict.receipt_set_ref.as_str(),
            conflict.artifact_path.as_str(),
        ] {
            if !self.evidence_refs.iter().any(|entry| entry == required_ref) {
                return Err(anyhow!(
                    "private-state anti-equivocation disposition missing preserved evidence ref"
                ));
            }
        }
        Ok(())
    }

    pub fn validate_shape(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_PRIVATE_STATE_ANTI_EQUIVOCATION_DISPOSITION_SCHEMA {
            return Err(anyhow!(
                "unsupported private-state anti-equivocation disposition schema '{}'",
                self.schema_version
            ));
        }
        normalize_id(
            self.disposition_id.clone(),
            "private_anti_equivocation.disposition_id",
        )?;
        validate_relative_path(
            &self.artifact_path,
            "private_anti_equivocation.disposition_artifact_path",
        )?;
        validate_relative_path(
            &self.conflict_ref,
            "private_anti_equivocation.disposition_conflict_ref",
        )?;
        validate_sha256_hash(
            &self.conflict_hash,
            "private_anti_equivocation.disposition_conflict_hash",
        )?;
        if self.disposition != "sanctuary_or_quarantine" {
            return Err(anyhow!(
                "private-state anti-equivocation disposition must enter sanctuary_or_quarantine"
            ));
        }
        if self.activation_allowed {
            return Err(anyhow!(
                "private-state anti-equivocation disposition cannot allow activation"
            ));
        }
        validate_nonempty_text(&self.reason, "private_anti_equivocation.reason")?;
        require_text_list(
            &self.evidence_refs,
            "private_anti_equivocation.evidence_refs",
            4,
        )?;
        for evidence_ref in &self.evidence_refs {
            validate_relative_path(evidence_ref, "private_anti_equivocation.evidence_ref")?;
        }
        require_text_list(
            &self.preserved_candidate_ids,
            "private_anti_equivocation.preserved_candidate_ids",
            2,
        )?;
        require_text_list(
            &self.preserved_candidate_entry_hashes,
            "private_anti_equivocation.preserved_candidate_entry_hashes",
            2,
        )?;
        for entry_hash in &self.preserved_candidate_entry_hashes {
            validate_sha256_hash(
                entry_hash,
                "private_anti_equivocation.preserved_candidate_entry_hash",
            )?;
        }
        require_text_list(
            &self.preserved_candidate_claim_hashes,
            "private_anti_equivocation.preserved_candidate_claim_hashes",
            2,
        )?;
        for claim_hash in &self.preserved_candidate_claim_hashes {
            validate_sha256_hash(
                claim_hash,
                "private_anti_equivocation.preserved_candidate_claim_hash",
            )?;
        }
        if self.destructive_transition_policy
            != "block_activation_and_preserve_evidence_until_review"
        {
            return Err(anyhow!(
                "private-state anti-equivocation disposition must block destructive transitions"
            ));
        }
        if self.review_route != "sanctuary_or_quarantine_operator_review" {
            return Err(anyhow!(
                "private-state anti-equivocation disposition must route to sanctuary/quarantine review"
            ));
        }
        validate_sha256_hash(
            &self.disposition_hash,
            "private_anti_equivocation.disposition_hash",
        )?;
        if self.disposition_hash != self.computed_hash()? {
            return Err(anyhow!(
                "private-state anti-equivocation disposition hash mismatch"
            ));
        }
        Ok(())
    }

    pub fn computed_hash(&self) -> Result<String> {
        normalize_id(
            self.disposition_id.clone(),
            "private_anti_equivocation.disposition_id",
        )?;
        Ok(sha256_bytes(self.hash_payload()?.as_bytes()))
    }

    pub fn pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.validate_shape()?;
        serde_json::to_vec_pretty(self)
            .context("serialize private-state anti-equivocation disposition")
    }

    fn hash_payload(&self) -> Result<String> {
        Ok(format!(
            "schema={}\ndisposition_id={}\nconflict_ref={}\nconflict_hash={}\ndisposition={}\nactivation_allowed={}\nactive_candidate_id={}\nreason={}\nevidence_refs={}\npreserved_candidate_ids={}\npreserved_candidate_entry_hashes={}\npreserved_candidate_claim_hashes={}\ndestructive_transition_policy={}\nreview_route={}\n",
            RUNTIME_V2_PRIVATE_STATE_ANTI_EQUIVOCATION_DISPOSITION_SCHEMA,
            self.disposition_id,
            self.conflict_ref,
            self.conflict_hash,
            self.disposition,
            self.activation_allowed,
            self.active_candidate_id.clone().unwrap_or_else(|| "none".to_string()),
            self.reason,
            self.evidence_refs.join("|"),
            self.preserved_candidate_ids.join("|"),
            self.preserved_candidate_entry_hashes.join("|"),
            self.preserved_candidate_claim_hashes.join("|"),
            self.destructive_transition_policy,
            self.review_route,
        ))
    }
}

impl RuntimeV2PrivateStateAntiEquivocationProof {
    pub fn prototype() -> Self {
        Self {
            schema_version: RUNTIME_V2_PRIVATE_STATE_ANTI_EQUIVOCATION_PROOF_SCHEMA.to_string(),
            proof_id: "private-state-anti-equivocation-negative-cases-proto-citizen-alpha"
                .to_string(),
            demo_id: "D7".to_string(),
            conflict_ref: RUNTIME_V2_PRIVATE_STATE_ANTI_EQUIVOCATION_CONFLICT_PATH.to_string(),
            disposition_ref: RUNTIME_V2_PRIVATE_STATE_ANTI_EQUIVOCATION_DISPOSITION_PATH
                .to_string(),
            ledger_ref: RUNTIME_V2_PRIVATE_STATE_LINEAGE_LEDGER_PATH.to_string(),
            witness_set_ref: RUNTIME_V2_PRIVATE_STATE_CONTINUITY_WITNESSES_PATH.to_string(),
            required_negative_cases: vec![
                anti_equivocation_negative_case(
                    "dual_active_successors",
                    "attempt to activate two signed successor candidates for the same sequence",
                    "cannot both become active",
                ),
                anti_equivocation_negative_case(
                    "missing_candidate_evidence",
                    "drop one candidate entry hash from the disposition",
                    "preserve all candidate evidence",
                ),
                anti_equivocation_negative_case(
                    "candidate_not_bound_to_head",
                    "change a candidate predecessor hash away from the accepted head",
                    "candidate does not target the contested successor position",
                ),
                anti_equivocation_negative_case(
                    "non_conflicting_candidates",
                    "submit duplicate claim hashes instead of conflicting successors",
                    "must contain conflicting signed successors",
                ),
            ],
            validation_command:
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_private_state_anti_equivocation -- --nocapture"
                    .to_string(),
            claim_boundary:
                "Proves fixture-backed anti-equivocation detection for conflicting signed successors; does not implement full sanctuary UX or challenge/appeal."
                    .to_string(),
        }
    }

    pub fn validate_against(
        &self,
        conflict: &RuntimeV2PrivateStateAntiEquivocationConflict,
        disposition: &RuntimeV2PrivateStateAntiEquivocationDisposition,
    ) -> Result<()> {
        self.validate_shape()?;
        if self.conflict_ref != conflict.artifact_path
            || self.disposition_ref != disposition.artifact_path
            || self.ledger_ref != conflict.ledger_ref
            || self.witness_set_ref != conflict.witness_set_ref
        {
            return Err(anyhow!(
                "private-state anti-equivocation proof refs must match conflict and disposition"
            ));
        }
        Ok(())
    }

    pub fn validate_shape(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_PRIVATE_STATE_ANTI_EQUIVOCATION_PROOF_SCHEMA {
            return Err(anyhow!(
                "unsupported private-state anti-equivocation proof schema '{}'",
                self.schema_version
            ));
        }
        normalize_id(self.proof_id.clone(), "private_anti_equivocation.proof_id")?;
        normalize_id(self.demo_id.clone(), "private_anti_equivocation.demo_id")?;
        validate_relative_path(
            &self.conflict_ref,
            "private_anti_equivocation.proof_conflict_ref",
        )?;
        validate_relative_path(
            &self.disposition_ref,
            "private_anti_equivocation.proof_disposition_ref",
        )?;
        validate_relative_path(
            &self.ledger_ref,
            "private_anti_equivocation.proof_ledger_ref",
        )?;
        validate_relative_path(
            &self.witness_set_ref,
            "private_anti_equivocation.proof_witness_set_ref",
        )?;
        if self.required_negative_cases.len() < 4 {
            return Err(anyhow!(
                "private-state anti-equivocation proof must include focused negative cases"
            ));
        }
        for case in &self.required_negative_cases {
            case.validate()?;
        }
        if !self
            .validation_command
            .contains("runtime_v2_private_state_anti_equivocation")
        {
            return Err(anyhow!(
                "private-state anti-equivocation proof validation command must target anti-equivocation tests"
            ));
        }
        if !self
            .claim_boundary
            .contains("does not implement full sanctuary UX")
        {
            return Err(anyhow!(
                "private-state anti-equivocation proof must preserve sanctuary UX non-claim"
            ));
        }
        Ok(())
    }

    pub fn pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.validate_shape()?;
        serde_json::to_vec_pretty(self).context("serialize private-state anti-equivocation proof")
    }
}

impl RuntimeV2PrivateStateAntiEquivocationNegativeCase {
    pub fn validate(&self) -> Result<()> {
        normalize_id(
            self.case_id.clone(),
            "private_anti_equivocation.negative_case_id",
        )?;
        validate_nonempty_text(
            &self.mutation,
            "private_anti_equivocation.negative_case_mutation",
        )?;
        validate_nonempty_text(
            &self.expected_error_fragment,
            "private_anti_equivocation.negative_case_expected_error_fragment",
        )
    }
}

trait RuntimeV2PrivateStateWitnessArtifactsExt {
    fn witness_artifacts_lineage(&self) -> &RuntimeV2PrivateStateLineageArtifacts;
}

impl RuntimeV2PrivateStateWitnessArtifactsExt for RuntimeV2PrivateStateWitnessArtifacts {
    fn witness_artifacts_lineage(&self) -> &RuntimeV2PrivateStateLineageArtifacts {
        &self.lineage_artifacts
    }
}

fn anti_equivocation_negative_case(
    case_id: &str,
    mutation: &str,
    expected_error_fragment: &str,
) -> RuntimeV2PrivateStateAntiEquivocationNegativeCase {
    RuntimeV2PrivateStateAntiEquivocationNegativeCase {
        case_id: case_id.to_string(),
        mutation: mutation.to_string(),
        expected_error_fragment: expected_error_fragment.to_string(),
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

fn sha256_label(label: &str) -> String {
    sha256_bytes(label.as_bytes())
}

fn sha256_bytes(bytes: &[u8]) -> String {
    format!("sha256:{:x}", Sha256::digest(bytes))
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
