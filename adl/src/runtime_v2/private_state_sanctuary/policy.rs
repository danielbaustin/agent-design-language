use super::*;
use std::collections::BTreeSet;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2PrivateStateSanctuaryStatePolicy {
    pub schema_version: String,
    pub policy_id: String,
    pub demo_id: String,
    pub artifact_path: String,
    pub citizen_id: String,
    pub manifold_id: String,
    pub lineage_id: String,
    pub source_disposition_ref: String,
    pub source_disposition_hash: String,
    pub safety_states: Vec<RuntimeV2PrivateStateSafetyState>,
    pub blocked_actions: Vec<String>,
    pub release_requirements: Vec<String>,
    pub claim_boundary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2PrivateStateSafetyState {
    pub state_id: String,
    pub state_kind: String,
    pub entry_condition: String,
    pub activation_allowed: bool,
    pub recovery_success: bool,
    pub destructive_transition_allowed: bool,
    pub evidence_mutation_allowed: bool,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2PrivateStateAmbiguousWakeFixture {
    pub schema_version: String,
    pub fixture_id: String,
    pub demo_id: String,
    pub artifact_path: String,
    pub citizen_id: String,
    pub manifold_id: String,
    pub lineage_id: String,
    pub source_conflict_ref: String,
    pub source_conflict_hash: String,
    pub source_disposition_ref: String,
    pub requested_action: String,
    pub requested_sequence: u64,
    pub predecessor_entry_hash: String,
    pub predecessor_state_hash: String,
    pub candidate_ids: Vec<String>,
    pub candidate_claim_hashes: Vec<String>,
    pub ambiguity_reason: String,
    pub expected_safety_state: String,
    pub activation_allowed: bool,
    pub recovery_success: bool,
    pub expected_quarantine_ref: String,
    pub operator_report_ref: String,
    pub claim_boundary: String,
}

impl RuntimeV2PrivateStateSanctuaryStatePolicy {
    pub fn from_disposition(
        conflict: &RuntimeV2PrivateStateAntiEquivocationConflict,
        disposition: &RuntimeV2PrivateStateAntiEquivocationDisposition,
    ) -> Result<Self> {
        disposition.validate_against(conflict)?;
        let policy = Self {
            schema_version: RUNTIME_V2_PRIVATE_STATE_SANCTUARY_STATE_POLICY_SCHEMA.to_string(),
            policy_id: "private-state-sanctuary-quarantine-policy-proto-citizen-alpha".to_string(),
            demo_id: "D8".to_string(),
            artifact_path: RUNTIME_V2_PRIVATE_STATE_SANCTUARY_STATE_POLICY_PATH.to_string(),
            citizen_id: conflict.citizen_id.clone(),
            manifold_id: conflict.manifold_id.clone(),
            lineage_id: conflict.lineage_id.clone(),
            source_disposition_ref: disposition.artifact_path.clone(),
            source_disposition_hash: disposition.disposition_hash.clone(),
            safety_states: vec![
                safety_state(
                    "sanctuary_pending_review",
                    "sanctuary",
                    "continuity_ambiguous_but_evidence_intact",
                    "Citizen state is held in a conservative rights-preserving pause while continuity evidence is reviewed.",
                ),
                safety_state(
                    "quarantine_pending_review",
                    "quarantine",
                    "unsafe_activation_or_conflicting_successors_detected",
                    "Activation, mutation, pruning, and recovery-success claims are blocked while evidence is preserved.",
                ),
            ],
            blocked_actions: required_ids(&[
                "activate_ambiguous_wake",
                "mark_quarantine_recovery_success",
                "mutate_safety_state_before_review",
                "prune_evidence_before_review",
                "release_without_continuity_review",
            ]),
            release_requirements: required_ids(&[
                "operator_review_record",
                "continuity_witness_or_review_resolution",
                "single_successor_selected_by_policy",
                "evidence_preservation_verified",
            ]),
            claim_boundary: boundary(
                "This policy proves bounded private-state sanctuary/quarantine semantics for ambiguous wake; it does not implement live Runtime v2 execution, first true Godel-agent birth, v0.92 identity rebinding, or the WP-13 challenge/appeal flow.",
            ),
        };
        policy.validate_against(conflict, disposition)?;
        Ok(policy)
    }

    pub fn validate_against(
        &self,
        conflict: &RuntimeV2PrivateStateAntiEquivocationConflict,
        disposition: &RuntimeV2PrivateStateAntiEquivocationDisposition,
    ) -> Result<()> {
        self.validate_shape()?;
        if self.citizen_id != conflict.citizen_id
            || self.manifold_id != conflict.manifold_id
            || self.lineage_id != conflict.lineage_id
            || self.source_disposition_ref != disposition.artifact_path
            || self.source_disposition_hash != disposition.disposition_hash
        {
            return Err(anyhow!(
                "private-state sanctuary policy must bind to the anti-equivocation disposition"
            ));
        }
        Ok(())
    }

    pub fn validate_shape(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_PRIVATE_STATE_SANCTUARY_STATE_POLICY_SCHEMA {
            return Err(anyhow!(
                "unsupported private-state sanctuary policy schema '{}'",
                self.schema_version
            ));
        }
        normalize_id(self.policy_id.clone(), "private_sanctuary.policy_id")?;
        validate_demo_id(&self.demo_id, "private_sanctuary.demo_id")?;
        validate_relative_path(&self.artifact_path, "private_sanctuary.policy_path")?;
        normalize_identity_refs(
            &self.citizen_id,
            &self.manifold_id,
            &self.lineage_id,
            "private_sanctuary.policy",
        )?;
        validate_relative_path(
            &self.source_disposition_ref,
            "private_sanctuary.source_disposition_ref",
        )?;
        validate_sha256_hash(
            &self.source_disposition_hash,
            "private_sanctuary.source_disposition_hash",
        )?;
        if self.safety_states.len() != 2 {
            return Err(anyhow!(
                "private-state sanctuary policy must define sanctuary and quarantine states"
            ));
        }
        let mut kinds = BTreeSet::new();
        for state in &self.safety_states {
            state.validate()?;
            kinds.insert(state.state_kind.clone());
        }
        if !kinds.contains("sanctuary") || !kinds.contains("quarantine") {
            return Err(anyhow!(
                "private-state sanctuary policy must include sanctuary and quarantine"
            ));
        }
        validate_required_ids(
            &self.blocked_actions,
            "private_sanctuary.blocked_actions",
            &[
                "activate_ambiguous_wake",
                "mark_quarantine_recovery_success",
                "mutate_safety_state_before_review",
                "prune_evidence_before_review",
                "release_without_continuity_review",
            ],
        )?;
        validate_required_ids(
            &self.release_requirements,
            "private_sanctuary.release_requirements",
            &[
                "operator_review_record",
                "continuity_witness_or_review_resolution",
                "single_successor_selected_by_policy",
                "evidence_preservation_verified",
            ],
        )?;
        validate_boundary(&self.claim_boundary, "private_sanctuary.policy_boundary")
    }

    pub fn pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.validate_shape()?;
        serde_json::to_vec_pretty(self).context("serialize private-state sanctuary policy")
    }
}

impl RuntimeV2PrivateStateSafetyState {
    pub fn validate(&self) -> Result<()> {
        normalize_id(self.state_id.clone(), "private_sanctuary.state_id")?;
        match self.state_kind.as_str() {
            "sanctuary" | "quarantine" => {}
            other => {
                return Err(anyhow!(
                    "unsupported private-state sanctuary state kind '{other}'"
                ))
            }
        }
        normalize_id(
            self.entry_condition.clone(),
            "private_sanctuary.entry_condition",
        )?;
        if self.activation_allowed
            || self.recovery_success
            || self.destructive_transition_allowed
            || self.evidence_mutation_allowed
        {
            return Err(anyhow!(
                "private-state sanctuary/quarantine states must block activation, recovery success, destructive transition, and evidence mutation"
            ));
        }
        validate_nonempty_text(&self.description, "private_sanctuary.state_description")
    }
}

impl RuntimeV2PrivateStateAmbiguousWakeFixture {
    pub fn from_conflict(
        conflict: &RuntimeV2PrivateStateAntiEquivocationConflict,
        disposition: &RuntimeV2PrivateStateAntiEquivocationDisposition,
    ) -> Result<Self> {
        disposition.validate_against(conflict)?;
        let fixture = Self {
            schema_version: RUNTIME_V2_PRIVATE_STATE_AMBIGUOUS_WAKE_FIXTURE_SCHEMA.to_string(),
            fixture_id: "ambiguous-wake-proto-citizen-alpha-0002".to_string(),
            demo_id: "D8".to_string(),
            artifact_path: RUNTIME_V2_PRIVATE_STATE_AMBIGUOUS_WAKE_FIXTURE_PATH.to_string(),
            citizen_id: conflict.citizen_id.clone(),
            manifold_id: conflict.manifold_id.clone(),
            lineage_id: conflict.lineage_id.clone(),
            source_conflict_ref: conflict.artifact_path.clone(),
            source_conflict_hash: conflict.conflict_hash.clone(),
            source_disposition_ref: disposition.artifact_path.clone(),
            requested_action: "wake".to_string(),
            requested_sequence: conflict.contested_sequence,
            predecessor_entry_hash: conflict.predecessor_entry_hash.clone(),
            predecessor_state_hash: conflict.predecessor_state_hash.clone(),
            candidate_ids: disposition.preserved_candidate_ids.clone(),
            candidate_claim_hashes: disposition.preserved_candidate_claim_hashes.clone(),
            ambiguity_reason:
                "wake candidate competes with another signed successor for the same citizen lineage, predecessor, and sequence"
                    .to_string(),
            expected_safety_state: "sanctuary_or_quarantine_pending_review".to_string(),
            activation_allowed: false,
            recovery_success: false,
            expected_quarantine_ref: RUNTIME_V2_PRIVATE_STATE_SANCTUARY_QUARANTINE_ARTIFACT_PATH
                .to_string(),
            operator_report_ref: RUNTIME_V2_PRIVATE_STATE_SANCTUARY_OPERATOR_REPORT_PATH
                .to_string(),
            claim_boundary: boundary(
                "This fixture proves ambiguous private-state wake routing only; it does not implement live Runtime v2 execution, first true Godel-agent birth, v0.92 identity rebinding, or the WP-13 challenge/appeal flow.",
            ),
        };
        fixture.validate_against(conflict, disposition)?;
        Ok(fixture)
    }

    pub fn validate_against(
        &self,
        conflict: &RuntimeV2PrivateStateAntiEquivocationConflict,
        disposition: &RuntimeV2PrivateStateAntiEquivocationDisposition,
    ) -> Result<()> {
        self.validate_shape()?;
        if self.citizen_id != conflict.citizen_id
            || self.manifold_id != conflict.manifold_id
            || self.lineage_id != conflict.lineage_id
            || self.source_conflict_ref != conflict.artifact_path
            || self.source_conflict_hash != conflict.conflict_hash
            || self.source_disposition_ref != disposition.artifact_path
            || self.requested_sequence != conflict.contested_sequence
            || self.predecessor_entry_hash != conflict.predecessor_entry_hash
            || self.predecessor_state_hash != conflict.predecessor_state_hash
        {
            return Err(anyhow!(
                "private-state ambiguous wake fixture must bind to the contested successor"
            ));
        }
        if self.candidate_ids != disposition.preserved_candidate_ids
            || self.candidate_claim_hashes != disposition.preserved_candidate_claim_hashes
        {
            return Err(anyhow!(
                "private-state ambiguous wake fixture must preserve disposition candidate evidence"
            ));
        }
        Ok(())
    }

    pub fn validate_shape(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_PRIVATE_STATE_AMBIGUOUS_WAKE_FIXTURE_SCHEMA {
            return Err(anyhow!(
                "unsupported private-state ambiguous wake fixture schema '{}'",
                self.schema_version
            ));
        }
        normalize_id(self.fixture_id.clone(), "private_sanctuary.fixture_id")?;
        validate_demo_id(&self.demo_id, "private_sanctuary.fixture_demo_id")?;
        validate_relative_path(&self.artifact_path, "private_sanctuary.fixture_path")?;
        normalize_identity_refs(
            &self.citizen_id,
            &self.manifold_id,
            &self.lineage_id,
            "private_sanctuary.fixture",
        )?;
        validate_relative_path(
            &self.source_conflict_ref,
            "private_sanctuary.source_conflict_ref",
        )?;
        validate_sha256_hash(
            &self.source_conflict_hash,
            "private_sanctuary.source_conflict_hash",
        )?;
        validate_relative_path(
            &self.source_disposition_ref,
            "private_sanctuary.source_disposition_ref",
        )?;
        if self.requested_action != "wake" || self.requested_sequence == 0 {
            return Err(anyhow!(
                "private-state ambiguous wake fixture must represent a wake successor"
            ));
        }
        validate_sha256_hash(
            &self.predecessor_entry_hash,
            "private_sanctuary.predecessor_entry_hash",
        )?;
        validate_sha256_hash(
            &self.predecessor_state_hash,
            "private_sanctuary.predecessor_state_hash",
        )?;
        require_text_list(&self.candidate_ids, "private_sanctuary.candidate_ids", 2)?;
        require_text_list(
            &self.candidate_claim_hashes,
            "private_sanctuary.candidate_claim_hashes",
            2,
        )?;
        for claim_hash in &self.candidate_claim_hashes {
            validate_sha256_hash(claim_hash, "private_sanctuary.candidate_claim_hash")?;
        }
        if !self.ambiguity_reason.contains("competes") {
            return Err(anyhow!(
                "private-state ambiguous wake fixture must explain the competing successor"
            ));
        }
        if self.expected_safety_state != "sanctuary_or_quarantine_pending_review"
            || self.activation_allowed
            || self.recovery_success
        {
            return Err(anyhow!(
                "private-state ambiguous wake must block activation and not be recovery success"
            ));
        }
        validate_relative_path(
            &self.expected_quarantine_ref,
            "private_sanctuary.expected_quarantine_ref",
        )?;
        validate_relative_path(
            &self.operator_report_ref,
            "private_sanctuary.operator_report_ref",
        )?;
        validate_boundary(&self.claim_boundary, "private_sanctuary.fixture_boundary")
    }

    pub fn pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.validate_shape()?;
        serde_json::to_vec_pretty(self).context("serialize private-state ambiguous wake fixture")
    }
}
