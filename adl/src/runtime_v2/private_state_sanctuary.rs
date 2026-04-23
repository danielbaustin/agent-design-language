//! Runtime-v2 private-state sanctuary and protection envelope.
//!
//! Defines sanctuary-boundary rules and records for sensitive continuity and
//! continuity-state safety checks.

use super::*;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::path::Path;

pub const RUNTIME_V2_PRIVATE_STATE_SANCTUARY_STATE_POLICY_SCHEMA: &str =
    "runtime_v2.private_state_sanctuary_quarantine_policy.v1";
pub const RUNTIME_V2_PRIVATE_STATE_AMBIGUOUS_WAKE_FIXTURE_SCHEMA: &str =
    "runtime_v2.private_state_ambiguous_wake_fixture.v1";
pub const RUNTIME_V2_PRIVATE_STATE_SANCTUARY_QUARANTINE_ARTIFACT_SCHEMA: &str =
    "runtime_v2.private_state_sanctuary_quarantine_artifact.v1";
pub const RUNTIME_V2_PRIVATE_STATE_SANCTUARY_OPERATOR_REPORT_SCHEMA: &str =
    "runtime_v2.private_state_sanctuary_operator_report.v1";
pub const RUNTIME_V2_PRIVATE_STATE_SANCTUARY_PROOF_SCHEMA: &str =
    "runtime_v2.private_state_sanctuary_quarantine_proof.v1";

pub const RUNTIME_V2_PRIVATE_STATE_SANCTUARY_STATE_POLICY_PATH: &str =
    "runtime_v2/private_state/sanctuary_quarantine_state_policy.json";
pub const RUNTIME_V2_PRIVATE_STATE_AMBIGUOUS_WAKE_FIXTURE_PATH: &str =
    "runtime_v2/private_state/ambiguous_wake_fixture.json";
pub const RUNTIME_V2_PRIVATE_STATE_SANCTUARY_QUARANTINE_ARTIFACT_PATH: &str =
    "runtime_v2/private_state/sanctuary_quarantine_artifact.json";
pub const RUNTIME_V2_PRIVATE_STATE_SANCTUARY_OPERATOR_REPORT_PATH: &str =
    "runtime_v2/private_state/sanctuary_quarantine_operator_report.json";
pub const RUNTIME_V2_PRIVATE_STATE_SANCTUARY_PROOF_PATH: &str =
    "runtime_v2/private_state/sanctuary_quarantine_negative_cases.json";

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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2PrivateStatePreservedEvidenceRef {
    pub evidence_id: String,
    pub artifact_ref: String,
    pub preservation_mode: String,
    pub retention_reason: String,
    pub immutable_until_review: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2PrivateStateSanctuaryTransition {
    pub sequence: u64,
    pub from_state: String,
    pub event: String,
    pub to_state: String,
    pub guard: String,
    pub evidence_ref: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2PrivateStateSanctuaryQuarantineArtifact {
    pub schema_version: String,
    pub quarantine_id: String,
    pub demo_id: String,
    pub artifact_path: String,
    pub citizen_id: String,
    pub manifold_id: String,
    pub lineage_id: String,
    pub source_fixture_ref: String,
    pub source_conflict_ref: String,
    pub source_disposition_ref: String,
    pub safety_state: String,
    pub activation_allowed: bool,
    pub recovery_success: bool,
    pub preserved_evidence: Vec<RuntimeV2PrivateStatePreservedEvidenceRef>,
    pub preserved_candidate_ids: Vec<String>,
    pub preserved_candidate_entry_hashes: Vec<String>,
    pub preserved_candidate_claim_hashes: Vec<String>,
    pub state_machine: Vec<RuntimeV2PrivateStateSanctuaryTransition>,
    pub blocked_actions: Vec<String>,
    pub operator_report_ref: String,
    pub release_requirements: Vec<String>,
    pub artifact_hash: String,
    pub claim_boundary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2PrivateStateSanctuaryOperatorReport {
    pub schema_version: String,
    pub report_id: String,
    pub demo_id: String,
    pub artifact_path: String,
    pub citizen_id: String,
    pub manifold_id: String,
    pub lineage_id: String,
    pub source_quarantine_ref: String,
    pub source_quarantine_hash: String,
    pub report_state: String,
    pub safe_to_activate: bool,
    pub recovery_success: bool,
    pub reviewed_evidence_refs: Vec<String>,
    pub findings: Vec<String>,
    pub recommended_action: String,
    pub claim_boundary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2PrivateStateSanctuaryNegativeCase {
    pub case_id: String,
    pub mutation: String,
    pub expected_error_fragment: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2PrivateStateSanctuaryProof {
    pub schema_version: String,
    pub proof_id: String,
    pub demo_id: String,
    pub state_policy_ref: String,
    pub ambiguous_wake_ref: String,
    pub quarantine_ref: String,
    pub operator_report_ref: String,
    pub required_negative_cases: Vec<RuntimeV2PrivateStateSanctuaryNegativeCase>,
    pub validation_command: String,
    pub claim_boundary: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeV2PrivateStateSanctuaryArtifacts {
    pub anti_equivocation_artifacts: RuntimeV2PrivateStateAntiEquivocationArtifacts,
    pub state_policy: RuntimeV2PrivateStateSanctuaryStatePolicy,
    pub ambiguous_wake: RuntimeV2PrivateStateAmbiguousWakeFixture,
    pub quarantine_artifact: RuntimeV2PrivateStateSanctuaryQuarantineArtifact,
    pub operator_report: RuntimeV2PrivateStateSanctuaryOperatorReport,
    pub negative_cases: RuntimeV2PrivateStateSanctuaryProof,
}

impl RuntimeV2PrivateStateSanctuaryArtifacts {
    pub fn prototype() -> Result<Self> {
        let anti_equivocation_artifacts = runtime_v2_private_state_anti_equivocation_contract()?;
        let state_policy = RuntimeV2PrivateStateSanctuaryStatePolicy::from_disposition(
            &anti_equivocation_artifacts.conflict,
            &anti_equivocation_artifacts.disposition,
        )?;
        let ambiguous_wake = RuntimeV2PrivateStateAmbiguousWakeFixture::from_conflict(
            &anti_equivocation_artifacts.conflict,
            &anti_equivocation_artifacts.disposition,
        )?;
        let quarantine_artifact = RuntimeV2PrivateStateSanctuaryQuarantineArtifact::from_fixture(
            &ambiguous_wake,
            &anti_equivocation_artifacts.conflict,
            &anti_equivocation_artifacts.disposition,
        )?;
        let operator_report =
            RuntimeV2PrivateStateSanctuaryOperatorReport::from_quarantine(&quarantine_artifact)?;
        let negative_cases = RuntimeV2PrivateStateSanctuaryProof::prototype();
        let artifacts = Self {
            anti_equivocation_artifacts,
            state_policy,
            ambiguous_wake,
            quarantine_artifact,
            operator_report,
            negative_cases,
        };
        artifacts.validate()?;
        Ok(artifacts)
    }

    pub fn validate(&self) -> Result<()> {
        self.anti_equivocation_artifacts.validate()?;
        self.state_policy.validate_against(
            &self.anti_equivocation_artifacts.conflict,
            &self.anti_equivocation_artifacts.disposition,
        )?;
        self.ambiguous_wake.validate_against(
            &self.anti_equivocation_artifacts.conflict,
            &self.anti_equivocation_artifacts.disposition,
        )?;
        self.quarantine_artifact.validate_against(
            &self.ambiguous_wake,
            &self.anti_equivocation_artifacts.conflict,
            &self.anti_equivocation_artifacts.disposition,
        )?;
        self.operator_report
            .validate_against(&self.quarantine_artifact)?;
        self.negative_cases.validate_against(
            &self.state_policy,
            &self.ambiguous_wake,
            &self.quarantine_artifact,
            &self.operator_report,
        )
    }

    pub fn write_to_root(&self, root: impl AsRef<Path>) -> Result<()> {
        self.validate()?;
        let root = root.as_ref();
        write_relative(
            root,
            RUNTIME_V2_PRIVATE_STATE_SANCTUARY_STATE_POLICY_PATH,
            self.state_policy.pretty_json_bytes()?,
        )?;
        write_relative(
            root,
            RUNTIME_V2_PRIVATE_STATE_AMBIGUOUS_WAKE_FIXTURE_PATH,
            self.ambiguous_wake.pretty_json_bytes()?,
        )?;
        write_relative(
            root,
            RUNTIME_V2_PRIVATE_STATE_SANCTUARY_QUARANTINE_ARTIFACT_PATH,
            self.quarantine_artifact.pretty_json_bytes()?,
        )?;
        write_relative(
            root,
            RUNTIME_V2_PRIVATE_STATE_SANCTUARY_OPERATOR_REPORT_PATH,
            self.operator_report.pretty_json_bytes()?,
        )?;
        write_relative(
            root,
            RUNTIME_V2_PRIVATE_STATE_SANCTUARY_PROOF_PATH,
            self.negative_cases.pretty_json_bytes()?,
        )?;
        Ok(())
    }
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

impl RuntimeV2PrivateStateSanctuaryQuarantineArtifact {
    pub fn from_fixture(
        fixture: &RuntimeV2PrivateStateAmbiguousWakeFixture,
        conflict: &RuntimeV2PrivateStateAntiEquivocationConflict,
        disposition: &RuntimeV2PrivateStateAntiEquivocationDisposition,
    ) -> Result<Self> {
        fixture.validate_against(conflict, disposition)?;
        let mut artifact = Self {
            schema_version: RUNTIME_V2_PRIVATE_STATE_SANCTUARY_QUARANTINE_ARTIFACT_SCHEMA
                .to_string(),
            quarantine_id: "sanctuary-quarantine-proto-citizen-alpha-0002".to_string(),
            demo_id: "D8".to_string(),
            artifact_path: fixture.expected_quarantine_ref.clone(),
            citizen_id: fixture.citizen_id.clone(),
            manifold_id: fixture.manifold_id.clone(),
            lineage_id: fixture.lineage_id.clone(),
            source_fixture_ref: fixture.artifact_path.clone(),
            source_conflict_ref: conflict.artifact_path.clone(),
            source_disposition_ref: disposition.artifact_path.clone(),
            safety_state: fixture.expected_safety_state.clone(),
            activation_allowed: false,
            recovery_success: false,
            preserved_evidence: preserved_evidence(conflict, disposition),
            preserved_candidate_ids: disposition.preserved_candidate_ids.clone(),
            preserved_candidate_entry_hashes: disposition.preserved_candidate_entry_hashes.clone(),
            preserved_candidate_claim_hashes: disposition.preserved_candidate_claim_hashes.clone(),
            state_machine: vec![
                transition(
                    1,
                    "wake_requested",
                    "ambiguous_successor_detected",
                    "activation_blocked",
                    "anti_equivocation_disposition_must_refuse_activation",
                    &disposition.artifact_path,
                ),
                transition(
                    2,
                    "activation_blocked",
                    "preserve_evidence",
                    "evidence_preserved",
                    "all_candidate_and_lineage_evidence_must_be_retained",
                    &conflict.artifact_path,
                ),
                transition(
                    3,
                    "evidence_preserved",
                    "enter_safety_state",
                    "sanctuary_or_quarantine_pending_review",
                    "no_recovery_success_until_review_resolution",
                    fixture.operator_report_ref.as_str(),
                ),
            ],
            blocked_actions: required_ids(&[
                "activate_ambiguous_wake",
                "mark_quarantine_recovery_success",
                "mutate_safety_state_before_review",
                "prune_evidence_before_review",
                "release_without_continuity_review",
            ]),
            operator_report_ref: fixture.operator_report_ref.clone(),
            release_requirements: required_ids(&[
                "operator_review_record",
                "continuity_witness_or_review_resolution",
                "single_successor_selected_by_policy",
                "evidence_preservation_verified",
            ]),
            artifact_hash: String::new(),
            claim_boundary: boundary(
                "This artifact proves bounded private-state sanctuary/quarantine behavior for ambiguous wake; it does not implement live Runtime v2 execution, first true Godel-agent birth, v0.92 identity rebinding, or the WP-13 challenge/appeal flow.",
            ),
        };
        artifact.artifact_hash = artifact.computed_hash()?;
        artifact.validate_against(fixture, conflict, disposition)?;
        Ok(artifact)
    }

    pub fn validate_against(
        &self,
        fixture: &RuntimeV2PrivateStateAmbiguousWakeFixture,
        conflict: &RuntimeV2PrivateStateAntiEquivocationConflict,
        disposition: &RuntimeV2PrivateStateAntiEquivocationDisposition,
    ) -> Result<()> {
        self.validate_shape()?;
        if self.citizen_id != fixture.citizen_id
            || self.manifold_id != fixture.manifold_id
            || self.lineage_id != fixture.lineage_id
            || self.artifact_path != fixture.expected_quarantine_ref
            || self.source_fixture_ref != fixture.artifact_path
            || self.source_conflict_ref != conflict.artifact_path
            || self.source_disposition_ref != disposition.artifact_path
            || self.operator_report_ref != fixture.operator_report_ref
        {
            return Err(anyhow!(
                "private-state sanctuary quarantine artifact must bind to ambiguous wake evidence"
            ));
        }
        if self.preserved_candidate_ids != disposition.preserved_candidate_ids
            || self.preserved_candidate_entry_hashes != disposition.preserved_candidate_entry_hashes
            || self.preserved_candidate_claim_hashes != disposition.preserved_candidate_claim_hashes
        {
            return Err(anyhow!(
                "private-state sanctuary quarantine artifact must preserve all candidate evidence"
            ));
        }
        self.validate_preserves_evidence(conflict, disposition)
    }

    pub fn validate_preserves_evidence(
        &self,
        conflict: &RuntimeV2PrivateStateAntiEquivocationConflict,
        disposition: &RuntimeV2PrivateStateAntiEquivocationDisposition,
    ) -> Result<()> {
        let refs = self
            .preserved_evidence
            .iter()
            .map(|evidence| evidence.artifact_ref.clone())
            .collect::<BTreeSet<_>>();
        for required in [
            conflict.ledger_ref.as_str(),
            conflict.witness_set_ref.as_str(),
            conflict.receipt_set_ref.as_str(),
            conflict.artifact_path.as_str(),
            disposition.artifact_path.as_str(),
        ] {
            if !refs.contains(required) {
                return Err(anyhow!(
                    "private-state sanctuary quarantine artifact missing required evidence"
                ));
            }
        }
        for candidate in &conflict.candidates {
            if !refs.contains(candidate.envelope_ref.as_str())
                || !refs.contains(candidate.sealed_checkpoint_ref.as_str())
            {
                return Err(anyhow!(
                    "private-state sanctuary quarantine artifact must preserve candidate envelope and checkpoint evidence"
                ));
            }
        }
        Ok(())
    }

    pub fn validate_shape(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_PRIVATE_STATE_SANCTUARY_QUARANTINE_ARTIFACT_SCHEMA {
            return Err(anyhow!(
                "unsupported private-state sanctuary quarantine artifact schema '{}'",
                self.schema_version
            ));
        }
        normalize_id(
            self.quarantine_id.clone(),
            "private_sanctuary.quarantine_id",
        )?;
        validate_demo_id(&self.demo_id, "private_sanctuary.quarantine_demo_id")?;
        validate_relative_path(&self.artifact_path, "private_sanctuary.quarantine_path")?;
        normalize_identity_refs(
            &self.citizen_id,
            &self.manifold_id,
            &self.lineage_id,
            "private_sanctuary.quarantine",
        )?;
        validate_relative_path(
            &self.source_fixture_ref,
            "private_sanctuary.source_fixture_ref",
        )?;
        validate_relative_path(
            &self.source_conflict_ref,
            "private_sanctuary.source_conflict_ref",
        )?;
        validate_relative_path(
            &self.source_disposition_ref,
            "private_sanctuary.source_disposition_ref",
        )?;
        if self.safety_state != "sanctuary_or_quarantine_pending_review"
            || self.activation_allowed
            || self.recovery_success
        {
            return Err(anyhow!(
                "private-state sanctuary quarantine must block activation and cannot be recovery success"
            ));
        }
        if self.preserved_evidence.len() < 9 {
            return Err(anyhow!(
                "private-state sanctuary quarantine must preserve lineage, conflict, disposition, and candidate evidence"
            ));
        }
        for evidence in &self.preserved_evidence {
            evidence.validate()?;
        }
        require_text_list(
            &self.preserved_candidate_ids,
            "private_sanctuary.preserved_candidate_ids",
            2,
        )?;
        require_text_list(
            &self.preserved_candidate_entry_hashes,
            "private_sanctuary.preserved_candidate_entry_hashes",
            2,
        )?;
        for entry_hash in &self.preserved_candidate_entry_hashes {
            validate_sha256_hash(
                entry_hash,
                "private_sanctuary.preserved_candidate_entry_hash",
            )?;
        }
        require_text_list(
            &self.preserved_candidate_claim_hashes,
            "private_sanctuary.preserved_candidate_claim_hashes",
            2,
        )?;
        for claim_hash in &self.preserved_candidate_claim_hashes {
            validate_sha256_hash(
                claim_hash,
                "private_sanctuary.preserved_candidate_claim_hash",
            )?;
        }
        if self.state_machine.len() != 3 {
            return Err(anyhow!(
                "private-state sanctuary quarantine state machine must have three transitions"
            ));
        }
        for transition in &self.state_machine {
            transition.validate()?;
        }
        if self
            .state_machine
            .last()
            .map(|entry| entry.to_state.as_str())
            != Some("sanctuary_or_quarantine_pending_review")
        {
            return Err(anyhow!(
                "private-state sanctuary quarantine must end pending review"
            ));
        }
        validate_required_ids(
            &self.blocked_actions,
            "private_sanctuary.quarantine_blocked_actions",
            &[
                "activate_ambiguous_wake",
                "mark_quarantine_recovery_success",
                "mutate_safety_state_before_review",
                "prune_evidence_before_review",
                "release_without_continuity_review",
            ],
        )?;
        validate_relative_path(
            &self.operator_report_ref,
            "private_sanctuary.operator_report_ref",
        )?;
        validate_required_ids(
            &self.release_requirements,
            "private_sanctuary.quarantine_release_requirements",
            &[
                "operator_review_record",
                "continuity_witness_or_review_resolution",
                "single_successor_selected_by_policy",
                "evidence_preservation_verified",
            ],
        )?;
        validate_sha256_hash(&self.artifact_hash, "private_sanctuary.artifact_hash")?;
        if self.artifact_hash != self.computed_hash()? {
            return Err(anyhow!(
                "private-state sanctuary quarantine artifact hash mismatch"
            ));
        }
        validate_boundary(
            &self.claim_boundary,
            "private_sanctuary.quarantine_boundary",
        )
    }

    pub fn computed_hash(&self) -> Result<String> {
        Ok(sha256_bytes(self.hash_payload()?.as_bytes()))
    }

    pub fn pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.validate_shape()?;
        serde_json::to_vec_pretty(self)
            .context("serialize private-state sanctuary quarantine artifact")
    }

    fn hash_payload(&self) -> Result<String> {
        Ok(format!(
            "schema={}\nquarantine_id={}\nartifact_path={}\ncitizen_id={}\nmanifold_id={}\nlineage_id={}\nsource_fixture_ref={}\nsource_conflict_ref={}\nsource_disposition_ref={}\nsafety_state={}\nactivation_allowed={}\nrecovery_success={}\npreserved_candidate_ids={}\npreserved_candidate_entry_hashes={}\npreserved_candidate_claim_hashes={}\nblocked_actions={}\noperator_report_ref={}\nrelease_requirements={}\n",
            RUNTIME_V2_PRIVATE_STATE_SANCTUARY_QUARANTINE_ARTIFACT_SCHEMA,
            self.quarantine_id,
            self.artifact_path,
            self.citizen_id,
            self.manifold_id,
            self.lineage_id,
            self.source_fixture_ref,
            self.source_conflict_ref,
            self.source_disposition_ref,
            self.safety_state,
            self.activation_allowed,
            self.recovery_success,
            self.preserved_candidate_ids.join("|"),
            self.preserved_candidate_entry_hashes.join("|"),
            self.preserved_candidate_claim_hashes.join("|"),
            self.blocked_actions.join("|"),
            self.operator_report_ref,
            self.release_requirements.join("|"),
        ))
    }
}

impl RuntimeV2PrivateStatePreservedEvidenceRef {
    pub fn validate(&self) -> Result<()> {
        normalize_id(self.evidence_id.clone(), "private_sanctuary.evidence_id")?;
        validate_relative_path(&self.artifact_ref, "private_sanctuary.evidence_ref")?;
        if self.preservation_mode != "retain_original" {
            return Err(anyhow!(
                "private-state sanctuary evidence must retain original artifacts"
            ));
        }
        validate_nonempty_text(
            &self.retention_reason,
            "private_sanctuary.evidence_retention_reason",
        )?;
        if !self.immutable_until_review {
            return Err(anyhow!(
                "private-state sanctuary evidence must be immutable until review"
            ));
        }
        Ok(())
    }
}

impl RuntimeV2PrivateStateSanctuaryTransition {
    pub fn validate(&self) -> Result<()> {
        if self.sequence == 0 {
            return Err(anyhow!(
                "private-state sanctuary transition sequence must be positive"
            ));
        }
        normalize_id(
            self.from_state.clone(),
            "private_sanctuary.transition_from_state",
        )?;
        normalize_id(self.event.clone(), "private_sanctuary.transition_event")?;
        normalize_id(
            self.to_state.clone(),
            "private_sanctuary.transition_to_state",
        )?;
        normalize_id(self.guard.clone(), "private_sanctuary.transition_guard")?;
        validate_relative_path(
            &self.evidence_ref,
            "private_sanctuary.transition_evidence_ref",
        )?;
        if self.to_state == "active" || self.to_state == "recovery_success" {
            return Err(anyhow!(
                "private-state sanctuary transition must not activate or mark recovery success"
            ));
        }
        Ok(())
    }
}

impl RuntimeV2PrivateStateSanctuaryOperatorReport {
    pub fn from_quarantine(
        quarantine: &RuntimeV2PrivateStateSanctuaryQuarantineArtifact,
    ) -> Result<Self> {
        quarantine.validate_shape()?;
        let report = Self {
            schema_version: RUNTIME_V2_PRIVATE_STATE_SANCTUARY_OPERATOR_REPORT_SCHEMA.to_string(),
            report_id: "sanctuary-quarantine-operator-report-proto-citizen-alpha-0002"
                .to_string(),
            demo_id: "D8".to_string(),
            artifact_path: quarantine.operator_report_ref.clone(),
            citizen_id: quarantine.citizen_id.clone(),
            manifold_id: quarantine.manifold_id.clone(),
            lineage_id: quarantine.lineage_id.clone(),
            source_quarantine_ref: quarantine.artifact_path.clone(),
            source_quarantine_hash: quarantine.artifact_hash.clone(),
            report_state: "operator_review_required".to_string(),
            safe_to_activate: false,
            recovery_success: false,
            reviewed_evidence_refs: quarantine
                .preserved_evidence
                .iter()
                .map(|evidence| evidence.artifact_ref.clone())
                .collect(),
            findings: vec![
                "ambiguous_wake_blocks_activation".to_string(),
                "candidate_evidence_preserved".to_string(),
                "quarantine_is_not_recovery_success".to_string(),
            ],
            recommended_action:
                "keep_in_sanctuary_or_quarantine_until_continuity_review_selects_one_valid_successor"
                    .to_string(),
            claim_boundary: boundary(
                "This operator report explains bounded ambiguous-wake quarantine evidence only; it does not implement live Runtime v2 execution, first true Godel-agent birth, v0.92 identity rebinding, or the WP-13 challenge/appeal flow.",
            ),
        };
        report.validate_against(quarantine)?;
        Ok(report)
    }

    pub fn validate_against(
        &self,
        quarantine: &RuntimeV2PrivateStateSanctuaryQuarantineArtifact,
    ) -> Result<()> {
        self.validate_shape()?;
        if self.citizen_id != quarantine.citizen_id
            || self.manifold_id != quarantine.manifold_id
            || self.lineage_id != quarantine.lineage_id
            || self.artifact_path != quarantine.operator_report_ref
            || self.source_quarantine_ref != quarantine.artifact_path
            || self.source_quarantine_hash != quarantine.artifact_hash
        {
            return Err(anyhow!(
                "private-state sanctuary operator report must bind to quarantine artifact"
            ));
        }
        let preserved_refs = quarantine
            .preserved_evidence
            .iter()
            .map(|evidence| evidence.artifact_ref.clone())
            .collect::<BTreeSet<_>>();
        let reviewed_refs = self
            .reviewed_evidence_refs
            .iter()
            .cloned()
            .collect::<BTreeSet<_>>();
        if reviewed_refs != preserved_refs {
            return Err(anyhow!(
                "private-state sanctuary operator report must review preserved evidence"
            ));
        }
        Ok(())
    }

    pub fn validate_shape(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_PRIVATE_STATE_SANCTUARY_OPERATOR_REPORT_SCHEMA {
            return Err(anyhow!(
                "unsupported private-state sanctuary operator report schema '{}'",
                self.schema_version
            ));
        }
        normalize_id(self.report_id.clone(), "private_sanctuary.report_id")?;
        validate_demo_id(&self.demo_id, "private_sanctuary.report_demo_id")?;
        validate_relative_path(&self.artifact_path, "private_sanctuary.report_path")?;
        normalize_identity_refs(
            &self.citizen_id,
            &self.manifold_id,
            &self.lineage_id,
            "private_sanctuary.report",
        )?;
        validate_relative_path(
            &self.source_quarantine_ref,
            "private_sanctuary.report_quarantine_ref",
        )?;
        validate_sha256_hash(
            &self.source_quarantine_hash,
            "private_sanctuary.report_quarantine_hash",
        )?;
        if self.report_state != "operator_review_required"
            || self.safe_to_activate
            || self.recovery_success
        {
            return Err(anyhow!(
                "private-state sanctuary operator report must require review and cannot mark activation or recovery success"
            ));
        }
        require_text_list(
            &self.reviewed_evidence_refs,
            "private_sanctuary.reviewed_evidence_refs",
            9,
        )?;
        for evidence_ref in &self.reviewed_evidence_refs {
            validate_relative_path(evidence_ref, "private_sanctuary.reviewed_evidence_ref")?;
        }
        validate_required_ids(
            &self.findings,
            "private_sanctuary.report_findings",
            &[
                "ambiguous_wake_blocks_activation",
                "candidate_evidence_preserved",
                "quarantine_is_not_recovery_success",
            ],
        )?;
        if !self
            .recommended_action
            .contains("keep_in_sanctuary_or_quarantine")
        {
            return Err(anyhow!(
                "private-state sanctuary operator report must recommend keeping the state held"
            ));
        }
        validate_boundary(&self.claim_boundary, "private_sanctuary.report_boundary")
    }

    pub fn pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.validate_shape()?;
        serde_json::to_vec_pretty(self).context("serialize private-state sanctuary operator report")
    }
}

impl RuntimeV2PrivateStateSanctuaryProof {
    pub fn prototype() -> Self {
        Self {
            schema_version: RUNTIME_V2_PRIVATE_STATE_SANCTUARY_PROOF_SCHEMA.to_string(),
            proof_id: "private-state-sanctuary-quarantine-negative-cases-proto-citizen-alpha"
                .to_string(),
            demo_id: "D8".to_string(),
            state_policy_ref: RUNTIME_V2_PRIVATE_STATE_SANCTUARY_STATE_POLICY_PATH.to_string(),
            ambiguous_wake_ref: RUNTIME_V2_PRIVATE_STATE_AMBIGUOUS_WAKE_FIXTURE_PATH.to_string(),
            quarantine_ref: RUNTIME_V2_PRIVATE_STATE_SANCTUARY_QUARANTINE_ARTIFACT_PATH
                .to_string(),
            operator_report_ref: RUNTIME_V2_PRIVATE_STATE_SANCTUARY_OPERATOR_REPORT_PATH
                .to_string(),
            required_negative_cases: vec![
                negative_case(
                    "ambiguous_wake_activation",
                    "set activation_allowed=true for an ambiguous wake",
                    "block activation",
                ),
                negative_case(
                    "quarantine_as_recovery_success",
                    "set recovery_success=true on the quarantine artifact",
                    "cannot be recovery success",
                ),
                negative_case(
                    "missing_candidate_evidence",
                    "drop one candidate envelope or checkpoint evidence ref",
                    "must preserve candidate envelope and checkpoint evidence",
                ),
                negative_case(
                    "operator_skips_preserved_evidence",
                    "remove preserved evidence from the operator report",
                    "must review preserved evidence",
                ),
            ],
            validation_command:
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_private_state_sanctuary -- --nocapture"
                    .to_string(),
            claim_boundary:
                "Proves bounded sanctuary/quarantine behavior for ambiguous private-state wake; does not implement challenge/appeal, access control, redacted Observatory projection, first true Godel-agent birth, or v0.92 identity rebinding."
                    .to_string(),
        }
    }

    pub fn validate_against(
        &self,
        state_policy: &RuntimeV2PrivateStateSanctuaryStatePolicy,
        ambiguous_wake: &RuntimeV2PrivateStateAmbiguousWakeFixture,
        quarantine: &RuntimeV2PrivateStateSanctuaryQuarantineArtifact,
        operator_report: &RuntimeV2PrivateStateSanctuaryOperatorReport,
    ) -> Result<()> {
        self.validate_shape()?;
        if self.state_policy_ref != state_policy.artifact_path
            || self.ambiguous_wake_ref != ambiguous_wake.artifact_path
            || self.quarantine_ref != quarantine.artifact_path
            || self.operator_report_ref != operator_report.artifact_path
        {
            return Err(anyhow!(
                "private-state sanctuary proof refs must match produced artifacts"
            ));
        }
        Ok(())
    }

    pub fn validate_shape(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_PRIVATE_STATE_SANCTUARY_PROOF_SCHEMA {
            return Err(anyhow!(
                "unsupported private-state sanctuary proof schema '{}'",
                self.schema_version
            ));
        }
        normalize_id(self.proof_id.clone(), "private_sanctuary.proof_id")?;
        validate_demo_id(&self.demo_id, "private_sanctuary.proof_demo_id")?;
        validate_relative_path(&self.state_policy_ref, "private_sanctuary.proof_policy_ref")?;
        validate_relative_path(
            &self.ambiguous_wake_ref,
            "private_sanctuary.proof_ambiguous_wake_ref",
        )?;
        validate_relative_path(
            &self.quarantine_ref,
            "private_sanctuary.proof_quarantine_ref",
        )?;
        validate_relative_path(
            &self.operator_report_ref,
            "private_sanctuary.proof_operator_report_ref",
        )?;
        if self.required_negative_cases.len() < 4 {
            return Err(anyhow!(
                "private-state sanctuary proof must include focused negative cases"
            ));
        }
        for case in &self.required_negative_cases {
            case.validate()?;
        }
        if !self
            .validation_command
            .contains("runtime_v2_private_state_sanctuary")
        {
            return Err(anyhow!(
                "private-state sanctuary proof validation command must target focused tests"
            ));
        }
        validate_boundary(&self.claim_boundary, "private_sanctuary.proof_boundary")
    }

    pub fn pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.validate_shape()?;
        serde_json::to_vec_pretty(self).context("serialize private-state sanctuary proof")
    }
}

impl RuntimeV2PrivateStateSanctuaryNegativeCase {
    pub fn validate(&self) -> Result<()> {
        normalize_id(self.case_id.clone(), "private_sanctuary.negative_case_id")?;
        validate_nonempty_text(&self.mutation, "private_sanctuary.negative_case_mutation")?;
        validate_nonempty_text(
            &self.expected_error_fragment,
            "private_sanctuary.negative_case_error",
        )
    }
}

fn safety_state(
    state_id: &str,
    state_kind: &str,
    entry_condition: &str,
    description: &str,
) -> RuntimeV2PrivateStateSafetyState {
    RuntimeV2PrivateStateSafetyState {
        state_id: state_id.to_string(),
        state_kind: state_kind.to_string(),
        entry_condition: entry_condition.to_string(),
        activation_allowed: false,
        recovery_success: false,
        destructive_transition_allowed: false,
        evidence_mutation_allowed: false,
        description: description.to_string(),
    }
}

fn preserved_evidence(
    conflict: &RuntimeV2PrivateStateAntiEquivocationConflict,
    disposition: &RuntimeV2PrivateStateAntiEquivocationDisposition,
) -> Vec<RuntimeV2PrivateStatePreservedEvidenceRef> {
    let mut evidence = vec![
        evidence_ref(
            "lineage_ledger",
            &conflict.ledger_ref,
            "anchors the accepted predecessor and contested successor sequence",
        ),
        evidence_ref(
            "continuity_witnesses",
            &conflict.witness_set_ref,
            "proves transition witnesses are available without raw private-state disclosure",
        ),
        evidence_ref(
            "citizen_receipts",
            &conflict.receipt_set_ref,
            "keeps citizen-facing continuity explanations available",
        ),
        evidence_ref(
            "anti_equivocation_conflict",
            &conflict.artifact_path,
            "records the ambiguous competing successor set",
        ),
        evidence_ref(
            "anti_equivocation_disposition",
            &disposition.artifact_path,
            "records the activation refusal and review route",
        ),
    ];
    for candidate in &conflict.candidates {
        evidence.push(evidence_ref(
            &format!("{}_envelope", candidate.candidate_id.replace('-', "_")),
            &candidate.envelope_ref,
            "preserves the signed successor envelope under review",
        ));
        evidence.push(evidence_ref(
            &format!(
                "{}_sealed_checkpoint",
                candidate.candidate_id.replace('-', "_")
            ),
            &candidate.sealed_checkpoint_ref,
            "preserves the sealed checkpoint under review",
        ));
    }
    evidence
}

fn evidence_ref(
    evidence_id: &str,
    artifact_ref: &str,
    retention_reason: &str,
) -> RuntimeV2PrivateStatePreservedEvidenceRef {
    RuntimeV2PrivateStatePreservedEvidenceRef {
        evidence_id: evidence_id.to_string(),
        artifact_ref: artifact_ref.to_string(),
        preservation_mode: "retain_original".to_string(),
        retention_reason: retention_reason.to_string(),
        immutable_until_review: true,
    }
}

fn transition(
    sequence: u64,
    from_state: &str,
    event: &str,
    to_state: &str,
    guard: &str,
    evidence_ref: &str,
) -> RuntimeV2PrivateStateSanctuaryTransition {
    RuntimeV2PrivateStateSanctuaryTransition {
        sequence,
        from_state: from_state.to_string(),
        event: event.to_string(),
        to_state: to_state.to_string(),
        guard: guard.to_string(),
        evidence_ref: evidence_ref.to_string(),
    }
}

fn negative_case(
    case_id: &str,
    mutation: &str,
    expected_error_fragment: &str,
) -> RuntimeV2PrivateStateSanctuaryNegativeCase {
    RuntimeV2PrivateStateSanctuaryNegativeCase {
        case_id: case_id.to_string(),
        mutation: mutation.to_string(),
        expected_error_fragment: expected_error_fragment.to_string(),
    }
}

fn required_ids(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_string()).collect()
}

fn normalize_identity_refs(
    citizen_id: &str,
    manifold_id: &str,
    lineage_id: &str,
    field: &str,
) -> Result<()> {
    normalize_id(citizen_id.to_string(), &format!("{field}.citizen_id"))?;
    normalize_id(manifold_id.to_string(), &format!("{field}.manifold_id"))?;
    normalize_id(lineage_id.to_string(), &format!("{field}.lineage_id"))?;
    Ok(())
}

fn validate_demo_id(value: &str, field: &str) -> Result<()> {
    if value != "D8" {
        return Err(anyhow!("{field} must map to D8"));
    }
    Ok(())
}

fn validate_required_ids(values: &[String], field: &str, required: &[&str]) -> Result<()> {
    if values.len() != required.len() {
        return Err(anyhow!("{field} must contain the required values exactly"));
    }
    let mut seen = BTreeSet::new();
    for (expected, value) in required.iter().zip(values.iter()) {
        normalize_id(value.clone(), field)?;
        if value != expected {
            return Err(anyhow!("{field} must preserve deterministic order"));
        }
        if !seen.insert(value.clone()) {
            return Err(anyhow!("{field} contains duplicate values"));
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

fn boundary(value: &str) -> String {
    value.to_string()
}

fn validate_boundary(value: &str, field: &str) -> Result<()> {
    validate_nonempty_text(value, field)?;
    for required in [
        "does not implement",
        "first true Godel-agent birth",
        "v0.92 identity rebinding",
    ] {
        if !value.contains(required) {
            return Err(anyhow!(
                "private-state sanctuary boundary must preserve non-claim '{required}'"
            ));
        }
    }
    Ok(())
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

fn sha256_bytes(bytes: &[u8]) -> String {
    format!("sha256:{:x}", Sha256::digest(bytes))
}
