use super::*;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;

pub const RUNTIME_V2_PRIVATE_STATE_CONTINUITY_WITNESS_SCHEMA: &str =
    "runtime_v2.private_state_continuity_witness.v1";
pub const RUNTIME_V2_PRIVATE_STATE_CONTINUITY_WITNESS_SET_SCHEMA: &str =
    "runtime_v2.private_state_continuity_witness_set.v1";
pub const RUNTIME_V2_PRIVATE_STATE_CITIZEN_RECEIPT_SCHEMA: &str =
    "runtime_v2.private_state_citizen_receipt.v1";
pub const RUNTIME_V2_PRIVATE_STATE_CITIZEN_RECEIPT_SET_SCHEMA: &str =
    "runtime_v2.private_state_citizen_receipt_set.v1";
pub const RUNTIME_V2_PRIVATE_STATE_WITNESS_RECEIPT_PROOF_SCHEMA: &str =
    "runtime_v2.private_state_witness_receipt_proof.v1";
pub const RUNTIME_V2_PRIVATE_STATE_CONTINUITY_WITNESSES_PATH: &str =
    "runtime_v2/private_state/continuity_witnesses.json";
pub const RUNTIME_V2_PRIVATE_STATE_CITIZEN_RECEIPTS_PATH: &str =
    "runtime_v2/private_state/citizen_receipts.json";
pub const RUNTIME_V2_PRIVATE_STATE_WITNESS_RECEIPT_PROOF_PATH: &str =
    "runtime_v2/private_state/witness_receipt_negative_cases.json";

const REQUIRED_TRANSITIONS: [&str; 5] = [
    "admission",
    "snapshot",
    "wake",
    "quarantine",
    "release-from-quarantine",
];

const REQUIRED_INVARIANTS: [&str; 7] = [
    "signature_checked",
    "content_hash_checked",
    "state_sequence_checked",
    "predecessor_linkage_checked",
    "lifecycle_legality_checked",
    "ledger_head_agreement_checked",
    "envelope_evidence_bound",
];

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2PrivateStateContinuityWitnessSet {
    pub schema_version: String,
    pub witness_set_id: String,
    pub artifact_path: String,
    pub ledger_ref: String,
    pub ledger_root_hash: String,
    pub materialized_head_ref: String,
    pub witness_authority: String,
    pub witnesses: Vec<RuntimeV2PrivateStateContinuityWitness>,
    pub claim_boundary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2PrivateStateContinuityWitness {
    pub schema_version: String,
    pub witness_id: String,
    pub transition_type: String,
    pub citizen_id: String,
    pub manifold_id: String,
    pub lineage_id: String,
    pub state_sequence: u64,
    pub predecessor_entry_hash: String,
    pub ledger_ref: String,
    pub ledger_root_hash: String,
    pub lineage_entry_id: String,
    pub lineage_entry_hash: String,
    pub accepted_head_entry_hash: String,
    pub materialized_head_ref: String,
    pub envelope_ref: String,
    pub envelope_hash: String,
    pub sealed_checkpoint_ref: String,
    pub sealed_checkpoint_hash: String,
    pub canonical_state_hash: String,
    pub evidence_binding: String,
    pub invariants_checked: Vec<String>,
    pub witness_authority: String,
    pub issued_at_logical_tick: u64,
    pub witness_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2PrivateStateCitizenReceiptSet {
    pub schema_version: String,
    pub receipt_set_id: String,
    pub artifact_path: String,
    pub witness_set_ref: String,
    pub ledger_ref: String,
    pub receipts: Vec<RuntimeV2PrivateStateCitizenReceipt>,
    pub claim_boundary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2PrivateStateCitizenReceipt {
    pub schema_version: String,
    pub receipt_id: String,
    pub transition_type: String,
    pub citizen_id: String,
    pub lineage_id: String,
    pub state_sequence: u64,
    pub witness_id: String,
    pub witness_hash: String,
    pub ledger_ref: String,
    pub accepted_head_entry_hash: String,
    pub continuity_explanation: Vec<String>,
    pub citizen_visible_evidence: Vec<String>,
    pub privacy_boundary: Vec<String>,
    pub withheld_private_material: Vec<String>,
    pub receipt_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2PrivateStateWitnessReceiptNegativeCase {
    pub case_id: String,
    pub mutation: String,
    pub expected_error_fragment: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2PrivateStateWitnessReceiptProof {
    pub schema_version: String,
    pub proof_id: String,
    pub demo_id: String,
    pub witness_set_ref: String,
    pub receipt_set_ref: String,
    pub ledger_ref: String,
    pub required_negative_cases: Vec<RuntimeV2PrivateStateWitnessReceiptNegativeCase>,
    pub validation_command: String,
    pub claim_boundary: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeV2PrivateStateWitnessArtifacts {
    pub lineage_artifacts: RuntimeV2PrivateStateLineageArtifacts,
    pub witness_set: RuntimeV2PrivateStateContinuityWitnessSet,
    pub receipt_set: RuntimeV2PrivateStateCitizenReceiptSet,
    pub negative_cases: RuntimeV2PrivateStateWitnessReceiptProof,
}

impl RuntimeV2PrivateStateWitnessArtifacts {
    pub fn prototype() -> Result<Self> {
        let lineage_artifacts = RuntimeV2PrivateStateLineageArtifacts::prototype()?;
        let witness_set = RuntimeV2PrivateStateContinuityWitnessSet::prototype(&lineage_artifacts)?;
        let receipt_set =
            RuntimeV2PrivateStateCitizenReceiptSet::prototype(&lineage_artifacts, &witness_set)?;
        let negative_cases = RuntimeV2PrivateStateWitnessReceiptProof::prototype();
        let artifacts = Self {
            lineage_artifacts,
            witness_set,
            receipt_set,
            negative_cases,
        };
        artifacts.validate()?;
        Ok(artifacts)
    }

    pub fn validate(&self) -> Result<()> {
        self.lineage_artifacts.validate()?;
        self.witness_set.validate_against(&self.lineage_artifacts)?;
        self.receipt_set
            .validate_against(&self.lineage_artifacts, &self.witness_set)?;
        self.negative_cases
            .validate_against(&self.witness_set, &self.receipt_set)
    }

    pub fn write_to_root(&self, root: impl AsRef<Path>) -> Result<()> {
        self.validate()?;
        let root = root.as_ref();
        write_relative(
            root,
            RUNTIME_V2_PRIVATE_STATE_CONTINUITY_WITNESSES_PATH,
            self.witness_set.pretty_json_bytes()?,
        )?;
        write_relative(
            root,
            RUNTIME_V2_PRIVATE_STATE_CITIZEN_RECEIPTS_PATH,
            self.receipt_set.pretty_json_bytes()?,
        )?;
        write_relative(
            root,
            RUNTIME_V2_PRIVATE_STATE_WITNESS_RECEIPT_PROOF_PATH,
            self.negative_cases.pretty_json_bytes()?,
        )?;
        Ok(())
    }
}

impl RuntimeV2PrivateStateContinuityWitnessSet {
    pub fn prototype(lineage: &RuntimeV2PrivateStateLineageArtifacts) -> Result<Self> {
        lineage.validate()?;
        let witnesses = REQUIRED_TRANSITIONS
            .iter()
            .enumerate()
            .map(|(idx, transition)| {
                RuntimeV2PrivateStateContinuityWitness::from_lineage_head(
                    transition,
                    idx as u64 + 1,
                    lineage,
                )
            })
            .collect::<Result<Vec<_>>>()?;
        let witness_set = Self {
            schema_version: RUNTIME_V2_PRIVATE_STATE_CONTINUITY_WITNESS_SET_SCHEMA.to_string(),
            witness_set_id: "private-state-continuity-witnesses-proto-citizen-alpha".to_string(),
            artifact_path: RUNTIME_V2_PRIVATE_STATE_CONTINUITY_WITNESSES_PATH.to_string(),
            ledger_ref: lineage.ledger.artifact_path.clone(),
            ledger_root_hash: lineage.ledger.ledger_root_hash.clone(),
            materialized_head_ref: lineage.materialized_head.artifact_path.clone(),
            witness_authority: "runtime_v2_continuity_witness".to_string(),
            witnesses,
            claim_boundary:
                "Witnesses bind transition examples to ledger, materialized-head, envelope, and sealed-checkpoint evidence; they do not expose private state."
                    .to_string(),
        };
        witness_set.validate_against(lineage)?;
        Ok(witness_set)
    }

    pub fn validate_against(&self, lineage: &RuntimeV2PrivateStateLineageArtifacts) -> Result<()> {
        self.validate_shape()?;
        lineage.validate()?;
        if self.ledger_ref != lineage.ledger.artifact_path
            || self.ledger_root_hash != lineage.ledger.ledger_root_hash
            || self.materialized_head_ref != lineage.materialized_head.artifact_path
        {
            return Err(anyhow!(
                "private-state continuity witness set ledger/materialized-head evidence mismatch"
            ));
        }
        let mut seen_ids = BTreeSet::new();
        let mut seen_transitions = BTreeSet::new();
        for witness in &self.witnesses {
            if !seen_ids.insert(witness.witness_id.clone()) {
                return Err(anyhow!(
                    "private-state continuity witness set contains duplicate witness id"
                ));
            }
            if !seen_transitions.insert(witness.transition_type.clone()) {
                return Err(anyhow!(
                    "private-state continuity witness set contains duplicate transition"
                ));
            }
            witness.validate_against(lineage)?;
        }
        require_required_transitions(&seen_transitions, "private_witness.witnesses")
    }

    pub fn validate_shape(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_PRIVATE_STATE_CONTINUITY_WITNESS_SET_SCHEMA {
            return Err(anyhow!(
                "unsupported private-state continuity witness set schema '{}'",
                self.schema_version
            ));
        }
        normalize_id(
            self.witness_set_id.clone(),
            "private_witness.witness_set_id",
        )?;
        validate_relative_path(&self.artifact_path, "private_witness.artifact_path")?;
        validate_relative_path(&self.ledger_ref, "private_witness.ledger_ref")?;
        validate_sha256_hash(&self.ledger_root_hash, "private_witness.ledger_root_hash")?;
        validate_relative_path(
            &self.materialized_head_ref,
            "private_witness.materialized_head_ref",
        )?;
        normalize_id(
            self.witness_authority.clone(),
            "private_witness.witness_authority",
        )?;
        if !self.claim_boundary.contains("do not expose private state")
            || !self.claim_boundary.contains("ledger")
        {
            return Err(anyhow!(
                "private-state continuity witness set must state ledger binding and privacy boundary"
            ));
        }
        Ok(())
    }

    pub fn find_transition(
        &self,
        transition_type: &str,
    ) -> Result<&RuntimeV2PrivateStateContinuityWitness> {
        self.witnesses
            .iter()
            .find(|witness| witness.transition_type == transition_type)
            .ok_or_else(|| {
                anyhow!("private-state continuity witness missing transition '{transition_type}'")
            })
    }

    pub fn pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.validate_shape()?;
        serde_json::to_vec_pretty(self).context("serialize private-state continuity witness set")
    }
}

impl RuntimeV2PrivateStateContinuityWitness {
    pub fn from_lineage_head(
        transition_type: &str,
        issued_at_logical_tick: u64,
        lineage: &RuntimeV2PrivateStateLineageArtifacts,
    ) -> Result<Self> {
        lineage.validate()?;
        validate_required_transition(transition_type)?;
        let ledger = &lineage.ledger;
        let head = ledger.accepted_head()?;
        let mut witness = Self {
            schema_version: RUNTIME_V2_PRIVATE_STATE_CONTINUITY_WITNESS_SCHEMA.to_string(),
            witness_id: format!("continuity-witness-proto-citizen-alpha-{transition_type}"),
            transition_type: transition_type.to_string(),
            citizen_id: ledger.citizen_id.clone(),
            manifold_id: ledger.manifold_id.clone(),
            lineage_id: ledger.lineage_id.clone(),
            state_sequence: head.state_sequence,
            predecessor_entry_hash: head.previous_entry_hash.clone(),
            ledger_ref: ledger.artifact_path.clone(),
            ledger_root_hash: ledger.ledger_root_hash.clone(),
            lineage_entry_id: head.entry_id.clone(),
            lineage_entry_hash: head.entry_hash.clone(),
            accepted_head_entry_hash: ledger.accepted_head_entry_hash.clone(),
            materialized_head_ref: lineage.materialized_head.artifact_path.clone(),
            envelope_ref: head.envelope_ref.clone(),
            envelope_hash: head.envelope_hash.clone(),
            sealed_checkpoint_ref: head.sealed_checkpoint_ref.clone(),
            sealed_checkpoint_hash: head.sealed_checkpoint_hash.clone(),
            canonical_state_hash: head.canonical_state_hash.clone(),
            evidence_binding: "ledger_head_plus_signed_envelope_plus_sealed_checkpoint".to_string(),
            invariants_checked: REQUIRED_INVARIANTS
                .iter()
                .map(|invariant| (*invariant).to_string())
                .collect(),
            witness_authority: "runtime_v2_continuity_witness".to_string(),
            issued_at_logical_tick,
            witness_hash: String::new(),
        };
        witness.witness_hash = witness.computed_hash()?;
        witness.validate_against(lineage)?;
        Ok(witness)
    }

    pub fn validate_against(&self, lineage: &RuntimeV2PrivateStateLineageArtifacts) -> Result<()> {
        self.validate_shape()?;
        lineage.validate()?;
        let ledger = &lineage.ledger;
        let head = ledger.accepted_head()?;
        if self.citizen_id != ledger.citizen_id
            || self.manifold_id != ledger.manifold_id
            || self.lineage_id != ledger.lineage_id
        {
            return Err(anyhow!(
                "private-state continuity witness identity must match ledger"
            ));
        }
        if self.state_sequence != head.state_sequence
            || self.predecessor_entry_hash != head.previous_entry_hash
            || self.ledger_ref != ledger.artifact_path
            || self.ledger_root_hash != ledger.ledger_root_hash
            || self.lineage_entry_id != head.entry_id
            || self.lineage_entry_hash != head.entry_hash
            || self.accepted_head_entry_hash != ledger.accepted_head_entry_hash
            || self.materialized_head_ref != lineage.materialized_head.artifact_path
            || self.envelope_ref != head.envelope_ref
            || self.envelope_hash != head.envelope_hash
            || self.sealed_checkpoint_ref != head.sealed_checkpoint_ref
            || self.sealed_checkpoint_hash != head.sealed_checkpoint_hash
            || self.canonical_state_hash != head.canonical_state_hash
        {
            return Err(anyhow!(
                "private-state continuity witness must bind to ledger and envelope evidence"
            ));
        }
        Ok(())
    }

    pub fn validate_shape(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_PRIVATE_STATE_CONTINUITY_WITNESS_SCHEMA {
            return Err(anyhow!(
                "unsupported private-state continuity witness schema '{}'",
                self.schema_version
            ));
        }
        normalize_id(self.witness_id.clone(), "private_witness.witness_id")?;
        validate_required_transition(&self.transition_type)?;
        normalize_id(self.citizen_id.clone(), "private_witness.citizen_id")?;
        normalize_id(self.manifold_id.clone(), "private_witness.manifold_id")?;
        normalize_id(self.lineage_id.clone(), "private_witness.lineage_id")?;
        if self.state_sequence == 0 {
            return Err(anyhow!(
                "private-state continuity witness sequence must be greater than zero"
            ));
        }
        validate_hash_or_genesis(
            &self.predecessor_entry_hash,
            "private_witness.predecessor_entry_hash",
        )?;
        validate_relative_path(&self.ledger_ref, "private_witness.ledger_ref")?;
        validate_sha256_hash(&self.ledger_root_hash, "private_witness.ledger_root_hash")?;
        normalize_id(
            self.lineage_entry_id.clone(),
            "private_witness.lineage_entry_id",
        )?;
        validate_sha256_hash(
            &self.lineage_entry_hash,
            "private_witness.lineage_entry_hash",
        )?;
        validate_sha256_hash(
            &self.accepted_head_entry_hash,
            "private_witness.accepted_head_entry_hash",
        )?;
        validate_relative_path(
            &self.materialized_head_ref,
            "private_witness.materialized_head_ref",
        )?;
        validate_relative_path(&self.envelope_ref, "private_witness.envelope_ref")?;
        validate_sha256_hash(&self.envelope_hash, "private_witness.envelope_hash")?;
        validate_relative_path(
            &self.sealed_checkpoint_ref,
            "private_witness.sealed_checkpoint_ref",
        )?;
        validate_sha256_hash(
            &self.sealed_checkpoint_hash,
            "private_witness.sealed_checkpoint_hash",
        )?;
        validate_sha256_hash(
            &self.canonical_state_hash,
            "private_witness.canonical_state_hash",
        )?;
        if self.evidence_binding != "ledger_head_plus_signed_envelope_plus_sealed_checkpoint" {
            return Err(anyhow!(
                "private-state continuity witness evidence binding must include ledger, envelope, and checkpoint"
            ));
        }
        require_invariants(&self.invariants_checked)?;
        normalize_id(
            self.witness_authority.clone(),
            "private_witness.witness_authority",
        )?;
        if self.issued_at_logical_tick == 0 {
            return Err(anyhow!(
                "private-state continuity witness logical tick must be positive"
            ));
        }
        validate_sha256_hash(&self.witness_hash, "private_witness.witness_hash")?;
        if self.witness_hash != self.computed_hash()? {
            return Err(anyhow!("private-state continuity witness hash mismatch"));
        }
        Ok(())
    }

    pub fn computed_hash(&self) -> Result<String> {
        normalize_id(self.witness_id.clone(), "private_witness.witness_id")?;
        Ok(sha256_bytes(self.hash_payload()?.as_bytes()))
    }

    fn hash_payload(&self) -> Result<String> {
        Ok(format!(
            "schema={}\nwitness_id={}\ntransition_type={}\ncitizen_id={}\nmanifold_id={}\nlineage_id={}\nstate_sequence={}\npredecessor_entry_hash={}\nledger_ref={}\nledger_root_hash={}\nlineage_entry_id={}\nlineage_entry_hash={}\naccepted_head_entry_hash={}\nmaterialized_head_ref={}\nenvelope_ref={}\nenvelope_hash={}\nsealed_checkpoint_ref={}\nsealed_checkpoint_hash={}\ncanonical_state_hash={}\nevidence_binding={}\ninvariants_checked={}\nwitness_authority={}\nissued_at_logical_tick={}\n",
            RUNTIME_V2_PRIVATE_STATE_CONTINUITY_WITNESS_SCHEMA,
            self.witness_id,
            self.transition_type,
            self.citizen_id,
            self.manifold_id,
            self.lineage_id,
            self.state_sequence,
            self.predecessor_entry_hash,
            self.ledger_ref,
            self.ledger_root_hash,
            self.lineage_entry_id,
            self.lineage_entry_hash,
            self.accepted_head_entry_hash,
            self.materialized_head_ref,
            self.envelope_ref,
            self.envelope_hash,
            self.sealed_checkpoint_ref,
            self.sealed_checkpoint_hash,
            self.canonical_state_hash,
            self.evidence_binding,
            self.invariants_checked.join("|"),
            self.witness_authority,
            self.issued_at_logical_tick,
        ))
    }
}

impl RuntimeV2PrivateStateCitizenReceiptSet {
    pub fn prototype(
        lineage: &RuntimeV2PrivateStateLineageArtifacts,
        witness_set: &RuntimeV2PrivateStateContinuityWitnessSet,
    ) -> Result<Self> {
        witness_set.validate_against(lineage)?;
        let receipts = witness_set
            .witnesses
            .iter()
            .map(RuntimeV2PrivateStateCitizenReceipt::from_witness)
            .collect::<Result<Vec<_>>>()?;
        let receipt_set = Self {
            schema_version: RUNTIME_V2_PRIVATE_STATE_CITIZEN_RECEIPT_SET_SCHEMA.to_string(),
            receipt_set_id: "private-state-citizen-receipts-proto-citizen-alpha".to_string(),
            artifact_path: RUNTIME_V2_PRIVATE_STATE_CITIZEN_RECEIPTS_PATH.to_string(),
            witness_set_ref: witness_set.artifact_path.clone(),
            ledger_ref: witness_set.ledger_ref.clone(),
            receipts,
            claim_boundary:
                "Receipts explain continuity to the citizen using ledger and witness evidence without exposing raw private state, sealed payloads, or unrelated citizen state."
                    .to_string(),
        };
        receipt_set.validate_against(lineage, witness_set)?;
        Ok(receipt_set)
    }

    pub fn validate_against(
        &self,
        lineage: &RuntimeV2PrivateStateLineageArtifacts,
        witness_set: &RuntimeV2PrivateStateContinuityWitnessSet,
    ) -> Result<()> {
        self.validate_shape()?;
        witness_set.validate_against(lineage)?;
        if self.witness_set_ref != witness_set.artifact_path
            || self.ledger_ref != witness_set.ledger_ref
        {
            return Err(anyhow!(
                "private-state citizen receipt set witness/ledger ref mismatch"
            ));
        }
        let mut seen_ids = BTreeSet::new();
        let mut seen_transitions = BTreeSet::new();
        for receipt in &self.receipts {
            if !seen_ids.insert(receipt.receipt_id.clone()) {
                return Err(anyhow!(
                    "private-state citizen receipt set contains duplicate receipt id"
                ));
            }
            if !seen_transitions.insert(receipt.transition_type.clone()) {
                return Err(anyhow!(
                    "private-state citizen receipt set contains duplicate transition"
                ));
            }
            let witness = witness_set.find_transition(&receipt.transition_type)?;
            receipt.validate_against(lineage, witness)?;
        }
        require_required_transitions(&seen_transitions, "private_receipt.receipts")
    }

    pub fn validate_shape(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_PRIVATE_STATE_CITIZEN_RECEIPT_SET_SCHEMA {
            return Err(anyhow!(
                "unsupported private-state citizen receipt set schema '{}'",
                self.schema_version
            ));
        }
        normalize_id(
            self.receipt_set_id.clone(),
            "private_receipt.receipt_set_id",
        )?;
        validate_relative_path(&self.artifact_path, "private_receipt.artifact_path")?;
        validate_relative_path(&self.witness_set_ref, "private_receipt.witness_set_ref")?;
        validate_relative_path(&self.ledger_ref, "private_receipt.ledger_ref")?;
        if !self
            .claim_boundary
            .contains("without exposing raw private state")
            || !self.claim_boundary.contains("sealed payloads")
        {
            return Err(anyhow!(
                "private-state citizen receipt set must state private-state disclosure boundary"
            ));
        }
        Ok(())
    }

    pub fn pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.validate_shape()?;
        serde_json::to_vec_pretty(self).context("serialize private-state citizen receipt set")
    }
}

impl RuntimeV2PrivateStateCitizenReceipt {
    pub fn from_witness(witness: &RuntimeV2PrivateStateContinuityWitness) -> Result<Self> {
        witness.validate_shape()?;
        let mut receipt = Self {
            schema_version: RUNTIME_V2_PRIVATE_STATE_CITIZEN_RECEIPT_SCHEMA.to_string(),
            receipt_id: format!(
                "citizen-receipt-proto-citizen-alpha-{}",
                witness.transition_type
            ),
            transition_type: witness.transition_type.clone(),
            citizen_id: witness.citizen_id.clone(),
            lineage_id: witness.lineage_id.clone(),
            state_sequence: witness.state_sequence,
            witness_id: witness.witness_id.clone(),
            witness_hash: witness.witness_hash.clone(),
            ledger_ref: witness.ledger_ref.clone(),
            accepted_head_entry_hash: witness.accepted_head_entry_hash.clone(),
            continuity_explanation: vec![
                format!(
                    "The polis recognizes this {transition} as a valid continuation of the same governed participant because the accepted ledger head, signed envelope, and sealed checkpoint evidence agree.",
                    transition = receipt_transition_label(&witness.transition_type),
                ),
                "The continuity witness checked signature, content hash, sequence, predecessor linkage, lifecycle legality, ledger/head agreement, and envelope evidence.".to_string(),
                "This receipt is an explanation of continuity, not authority to inspect unrelated private state.".to_string(),
            ],
            citizen_visible_evidence: vec![
                format!("witness_id={}", witness.witness_id),
                format!("witness_hash={}", witness.witness_hash),
                format!("ledger_ref={}", witness.ledger_ref),
                format!("accepted_head_entry_hash={}", witness.accepted_head_entry_hash),
                format!("envelope_hash={}", witness.envelope_hash),
                format!("sealed_checkpoint_hash={}", witness.sealed_checkpoint_hash),
                format!("canonical_state_hash={}", witness.canonical_state_hash),
            ],
            privacy_boundary: vec![
                "does not expose unrelated private state".to_string(),
                "does not expose raw private state bytes".to_string(),
                "does not expose sealed payload material".to_string(),
                "does not expose other citizens' state".to_string(),
            ],
            withheld_private_material: vec![
                "raw private state".to_string(),
                "sealed payload".to_string(),
                "private memory compartments".to_string(),
                "unrelated citizen state".to_string(),
            ],
            receipt_hash: String::new(),
        };
        receipt.receipt_hash = receipt.computed_hash()?;
        receipt.validate_shape()?;
        Ok(receipt)
    }

    pub fn validate_against(
        &self,
        lineage: &RuntimeV2PrivateStateLineageArtifacts,
        witness: &RuntimeV2PrivateStateContinuityWitness,
    ) -> Result<()> {
        self.validate_shape()?;
        witness.validate_against(lineage)?;
        if self.transition_type != witness.transition_type
            || self.citizen_id != witness.citizen_id
            || self.lineage_id != witness.lineage_id
            || self.state_sequence != witness.state_sequence
            || self.witness_id != witness.witness_id
            || self.witness_hash != witness.witness_hash
            || self.ledger_ref != witness.ledger_ref
            || self.accepted_head_entry_hash != witness.accepted_head_entry_hash
        {
            return Err(anyhow!(
                "private-state citizen receipt must reference matching witness and ledger evidence"
            ));
        }
        self.validate_explains_continuity()?;
        self.validate_no_private_leakage(&lineage.lineage_artifacts_sealed_payload())?;
        Ok(())
    }

    pub fn validate_shape(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_PRIVATE_STATE_CITIZEN_RECEIPT_SCHEMA {
            return Err(anyhow!(
                "unsupported private-state citizen receipt schema '{}'",
                self.schema_version
            ));
        }
        normalize_id(self.receipt_id.clone(), "private_receipt.receipt_id")?;
        validate_required_transition(&self.transition_type)?;
        normalize_id(self.citizen_id.clone(), "private_receipt.citizen_id")?;
        normalize_id(self.lineage_id.clone(), "private_receipt.lineage_id")?;
        if self.state_sequence == 0 {
            return Err(anyhow!(
                "private-state citizen receipt sequence must be greater than zero"
            ));
        }
        normalize_id(self.witness_id.clone(), "private_receipt.witness_id")?;
        validate_sha256_hash(&self.witness_hash, "private_receipt.witness_hash")?;
        validate_relative_path(&self.ledger_ref, "private_receipt.ledger_ref")?;
        validate_sha256_hash(
            &self.accepted_head_entry_hash,
            "private_receipt.accepted_head_entry_hash",
        )?;
        require_text_list(
            &self.continuity_explanation,
            "private_receipt.continuity_explanation",
            2,
        )?;
        require_text_list(
            &self.citizen_visible_evidence,
            "private_receipt.citizen_visible_evidence",
            4,
        )?;
        require_text_list(
            &self.privacy_boundary,
            "private_receipt.privacy_boundary",
            3,
        )?;
        require_text_list(
            &self.withheld_private_material,
            "private_receipt.withheld_private_material",
            3,
        )?;
        validate_sha256_hash(&self.receipt_hash, "private_receipt.receipt_hash")?;
        if self.receipt_hash != self.computed_hash()? {
            return Err(anyhow!("private-state citizen receipt hash mismatch"));
        }
        Ok(())
    }

    pub fn validate_explains_continuity(&self) -> Result<()> {
        let explanation = self.continuity_explanation.join("\n").to_ascii_lowercase();
        if !explanation.contains("valid continuation")
            || !explanation.contains("same governed participant")
        {
            return Err(anyhow!(
                "private-state citizen receipt must explain valid continuation of the same governed participant"
            ));
        }
        if !self
            .privacy_boundary
            .iter()
            .any(|entry| entry.contains("does not expose unrelated private state"))
        {
            return Err(anyhow!(
                "private-state citizen receipt must state it does not expose unrelated private state"
            ));
        }
        Ok(())
    }

    pub fn validate_no_private_leakage(&self, sealed_payload_b64: &str) -> Result<()> {
        let json = serde_json::to_string(self).context("serialize receipt for leakage check")?;
        if json.contains(sealed_payload_b64)
            || json.contains("sealed_payload_b64")
            || json.contains("raw_private_state")
            || json.contains("private_key")
        {
            return Err(anyhow!(
                "private-state citizen receipt leaked private or sealed material"
            ));
        }
        Ok(())
    }

    pub fn computed_hash(&self) -> Result<String> {
        normalize_id(self.receipt_id.clone(), "private_receipt.receipt_id")?;
        Ok(sha256_bytes(self.hash_payload()?.as_bytes()))
    }

    fn hash_payload(&self) -> Result<String> {
        Ok(format!(
            "schema={}\nreceipt_id={}\ntransition_type={}\ncitizen_id={}\nlineage_id={}\nstate_sequence={}\nwitness_id={}\nwitness_hash={}\nledger_ref={}\naccepted_head_entry_hash={}\ncontinuity_explanation={}\ncitizen_visible_evidence={}\nprivacy_boundary={}\nwithheld_private_material={}\n",
            RUNTIME_V2_PRIVATE_STATE_CITIZEN_RECEIPT_SCHEMA,
            self.receipt_id,
            self.transition_type,
            self.citizen_id,
            self.lineage_id,
            self.state_sequence,
            self.witness_id,
            self.witness_hash,
            self.ledger_ref,
            self.accepted_head_entry_hash,
            self.continuity_explanation.join("|"),
            self.citizen_visible_evidence.join("|"),
            self.privacy_boundary.join("|"),
            self.withheld_private_material.join("|"),
        ))
    }
}

impl RuntimeV2PrivateStateWitnessReceiptProof {
    pub fn prototype() -> Self {
        Self {
            schema_version: RUNTIME_V2_PRIVATE_STATE_WITNESS_RECEIPT_PROOF_SCHEMA.to_string(),
            proof_id: "private-state-witness-receipt-negative-cases-proto-citizen-alpha"
                .to_string(),
            demo_id: "D6".to_string(),
            witness_set_ref: RUNTIME_V2_PRIVATE_STATE_CONTINUITY_WITNESSES_PATH.to_string(),
            receipt_set_ref: RUNTIME_V2_PRIVATE_STATE_CITIZEN_RECEIPTS_PATH.to_string(),
            ledger_ref: RUNTIME_V2_PRIVATE_STATE_LINEAGE_LEDGER_PATH.to_string(),
            required_negative_cases: vec![
                witness_negative_case(
                    "tampered_witness_hash",
                    "replace witness_hash after evidence is bound",
                    "private-state continuity witness hash mismatch",
                ),
                witness_negative_case(
                    "mismatched_ledger_root",
                    "point a witness at a ledger root that does not match the accepted ledger",
                    "private-state continuity witness must bind to ledger and envelope evidence",
                ),
                witness_negative_case(
                    "receipt_leaks_sealed_payload",
                    "copy sealed_payload_b64 into a citizen-facing receipt",
                    "private-state citizen receipt leaked private or sealed material",
                ),
                witness_negative_case(
                    "missing_continuity_explanation",
                    "remove valid-continuation language from a receipt",
                    "private-state citizen receipt must explain valid continuation",
                ),
                witness_negative_case(
                    "missing_required_transition",
                    "omit one of admission, snapshot, wake, quarantine, release-from-quarantine",
                    "missing required transition",
                ),
            ],
            validation_command:
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_private_state_witness -- --nocapture"
                    .to_string(),
            claim_boundary:
                "Proves witness and receipt schema fixtures for five transition examples; does not implement anti-equivocation or full challenge/appeal."
                    .to_string(),
        }
    }

    pub fn validate_against(
        &self,
        witness_set: &RuntimeV2PrivateStateContinuityWitnessSet,
        receipt_set: &RuntimeV2PrivateStateCitizenReceiptSet,
    ) -> Result<()> {
        self.validate_shape()?;
        if self.witness_set_ref != witness_set.artifact_path
            || self.receipt_set_ref != receipt_set.artifact_path
            || self.ledger_ref != witness_set.ledger_ref
        {
            return Err(anyhow!(
                "private-state witness receipt proof refs must match witness and receipt artifacts"
            ));
        }
        Ok(())
    }

    pub fn validate_shape(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_PRIVATE_STATE_WITNESS_RECEIPT_PROOF_SCHEMA {
            return Err(anyhow!(
                "unsupported private-state witness receipt proof schema '{}'",
                self.schema_version
            ));
        }
        normalize_id(self.proof_id.clone(), "private_witness.proof_id")?;
        normalize_id(self.demo_id.clone(), "private_witness.demo_id")?;
        validate_relative_path(&self.witness_set_ref, "private_witness.witness_set_ref")?;
        validate_relative_path(&self.receipt_set_ref, "private_witness.receipt_set_ref")?;
        validate_relative_path(&self.ledger_ref, "private_witness.proof_ledger_ref")?;
        if self.required_negative_cases.len() < 5 {
            return Err(anyhow!(
                "private-state witness receipt proof must include focused negative cases"
            ));
        }
        for case in &self.required_negative_cases {
            case.validate()?;
        }
        if !self
            .validation_command
            .contains("runtime_v2_private_state_witness")
        {
            return Err(anyhow!(
                "private-state witness receipt proof validation command must target witness tests"
            ));
        }
        if !self
            .claim_boundary
            .contains("does not implement anti-equivocation")
        {
            return Err(anyhow!(
                "private-state witness receipt proof must preserve anti-equivocation non-claim"
            ));
        }
        Ok(())
    }

    pub fn pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.validate_shape()?;
        serde_json::to_vec_pretty(self).context("serialize private-state witness receipt proof")
    }
}

impl RuntimeV2PrivateStateWitnessReceiptNegativeCase {
    pub fn validate(&self) -> Result<()> {
        normalize_id(self.case_id.clone(), "private_witness.negative_case_id")?;
        validate_nonempty_text(&self.mutation, "private_witness.negative_case_mutation")?;
        validate_nonempty_text(
            &self.expected_error_fragment,
            "private_witness.negative_case_expected_error_fragment",
        )
    }
}

trait RuntimeV2PrivateStateLineageArtifactsExt {
    fn lineage_artifacts_sealed_payload(&self) -> String;
}

impl RuntimeV2PrivateStateLineageArtifactsExt for RuntimeV2PrivateStateLineageArtifacts {
    fn lineage_artifacts_sealed_payload(&self) -> String {
        self.sealing_artifacts
            .sealed_checkpoint
            .sealed_payload_b64
            .clone()
    }
}

fn witness_negative_case(
    case_id: &str,
    mutation: &str,
    expected_error_fragment: &str,
) -> RuntimeV2PrivateStateWitnessReceiptNegativeCase {
    RuntimeV2PrivateStateWitnessReceiptNegativeCase {
        case_id: case_id.to_string(),
        mutation: mutation.to_string(),
        expected_error_fragment: expected_error_fragment.to_string(),
    }
}

fn require_required_transitions(seen: &BTreeSet<String>, field: &str) -> Result<()> {
    for transition in REQUIRED_TRANSITIONS {
        if !seen.contains(transition) {
            return Err(anyhow!(
                "{field} missing required transition '{transition}'"
            ));
        }
    }
    Ok(())
}

fn validate_required_transition(transition_type: &str) -> Result<()> {
    if REQUIRED_TRANSITIONS.contains(&transition_type) {
        Ok(())
    } else {
        Err(anyhow!(
            "unsupported private-state continuity transition '{}'",
            transition_type
        ))
    }
}

fn require_invariants(invariants: &[String]) -> Result<()> {
    let seen = invariants.iter().cloned().collect::<BTreeSet<_>>();
    for invariant in REQUIRED_INVARIANTS {
        if !seen.contains(invariant) {
            return Err(anyhow!(
                "private-state continuity witness missing invariant '{invariant}'"
            ));
        }
    }
    Ok(())
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

fn receipt_transition_label(transition_type: &str) -> String {
    transition_type.replace('-', " ")
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
