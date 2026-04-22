use super::*;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::path::Path;

pub const RUNTIME_V2_CONTINUITY_CHALLENGE_SCHEMA: &str =
    "runtime_v2.continuity_challenge_artifact.v1";
pub const RUNTIME_V2_CONTINUITY_CHALLENGE_FREEZE_SCHEMA: &str =
    "runtime_v2.continuity_challenge_freeze_artifact.v1";
pub const RUNTIME_V2_CONTINUITY_APPEAL_REVIEW_SCHEMA: &str =
    "runtime_v2.continuity_appeal_review_artifact.v1";
pub const RUNTIME_V2_CITIZEN_STATE_THREAT_MODEL_SCHEMA: &str =
    "runtime_v2.citizen_state_threat_model.v1";
pub const RUNTIME_V2_ECONOMICS_PLACEMENT_SCHEMA: &str = "runtime_v2.economics_placement_record.v1";

pub const RUNTIME_V2_CONTINUITY_CHALLENGE_PATH: &str =
    "runtime_v2/challenge/challenge_artifact.json";
pub const RUNTIME_V2_CONTINUITY_CHALLENGE_FREEZE_PATH: &str =
    "runtime_v2/challenge/freeze_artifact.json";
pub const RUNTIME_V2_CONTINUITY_APPEAL_REVIEW_PATH: &str =
    "runtime_v2/challenge/appeal_review_artifact.json";
pub const RUNTIME_V2_CITIZEN_STATE_THREAT_MODEL_PATH: &str =
    "runtime_v2/challenge/citizen_state_threat_model.json";
pub const RUNTIME_V2_ECONOMICS_PLACEMENT_PATH: &str =
    "runtime_v2/challenge/economics_placement_record.json";

const REQUIRED_THREATS: [&str; 7] = [
    "insider_operator_abuse",
    "compromised_key",
    "malicious_guest",
    "equivocation",
    "replay",
    "projection_leakage",
    "unsafe_release_from_quarantine",
];

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2ContinuityChallengeArtifact {
    pub schema_version: String,
    pub challenge_id: String,
    pub demo_id: String,
    pub artifact_path: String,
    pub citizen_id: String,
    pub lineage_id: String,
    pub challenged_transition_kinds: Vec<String>,
    pub challenge_reason: String,
    pub raised_by_actor_id: String,
    pub raised_by_standing_class: String,
    pub source_access_event_refs: Vec<String>,
    pub source_quarantine_ref: String,
    pub source_observatory_projection_ref: String,
    pub expected_freeze_ref: String,
    pub required_evidence_refs: Vec<String>,
    pub destructive_transition_frozen: bool,
    pub projection_publication_frozen: bool,
    pub raw_private_state_disclosed: bool,
    pub continuity_mutation_allowed: bool,
    pub status: String,
    pub claim_boundary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2ContinuityFreezeTransition {
    pub sequence: u64,
    pub from_state: String,
    pub event: String,
    pub to_state: String,
    pub guard: String,
    pub evidence_ref: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2ContinuityChallengeFreezeArtifact {
    pub schema_version: String,
    pub freeze_id: String,
    pub demo_id: String,
    pub artifact_path: String,
    pub source_challenge_ref: String,
    pub citizen_id: String,
    pub lineage_id: String,
    pub frozen_transition_kinds: Vec<String>,
    pub state_machine: Vec<RuntimeV2ContinuityFreezeTransition>,
    pub blocked_actions: Vec<String>,
    pub preserved_evidence_refs: Vec<String>,
    pub destructive_transition_allowed: bool,
    pub projection_publication_allowed: bool,
    pub continuity_sequence_before: u64,
    pub continuity_sequence_after: u64,
    pub active_head_changed: bool,
    pub raw_private_state_disclosed: bool,
    pub release_requires_review_resolution: bool,
    pub freeze_hash: String,
    pub claim_boundary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2ContinuityAppealReviewArtifact {
    pub schema_version: String,
    pub appeal_id: String,
    pub demo_id: String,
    pub artifact_path: String,
    pub source_challenge_ref: String,
    pub source_freeze_ref: String,
    pub citizen_id: String,
    pub lineage_id: String,
    pub appellant_actor_id: String,
    pub appellant_standing_class: String,
    pub reviewer_standing_classes: Vec<String>,
    pub permitted_outcomes: Vec<String>,
    pub current_outcome: String,
    pub release_allowed: bool,
    pub destructive_transition_allowed: bool,
    pub required_resolution_evidence: Vec<String>,
    pub due_process_guarantees: Vec<String>,
    pub claim_boundary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2CitizenStateThreat {
    pub threat_id: String,
    pub actor_class: String,
    pub attack_path: String,
    pub required_controls: Vec<String>,
    pub detection_artifacts: Vec<String>,
    pub fail_closed_behavior: String,
    pub covered_by_wp13: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2CitizenStateThreatModel {
    pub schema_version: String,
    pub model_id: String,
    pub demo_id: String,
    pub artifact_path: String,
    pub challenge_ref: String,
    pub freeze_ref: String,
    pub appeal_review_ref: String,
    pub access_matrix_ref: String,
    pub quarantine_ref: String,
    pub threats: Vec<RuntimeV2CitizenStateThreat>,
    pub universal_controls: Vec<String>,
    pub claim_boundary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2EconomicsPlacementRecord {
    pub schema_version: String,
    pub record_id: String,
    pub demo_id: String,
    pub artifact_path: String,
    pub decision: String,
    pub rationale: String,
    pub allowed_v0_90_3_scope: Vec<String>,
    pub deferred_to_v0_90_4: Vec<String>,
    pub markets_implemented: bool,
    pub payment_rails_implemented: bool,
    pub bidding_implemented: bool,
    pub subcontracting_implemented: bool,
    pub continuity_priority_rule: String,
    pub claim_boundary: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeV2ContinuityChallengeArtifacts {
    pub access_control_artifacts: RuntimeV2AccessControlArtifacts,
    pub sanctuary_artifacts: RuntimeV2PrivateStateSanctuaryArtifacts,
    pub challenge: RuntimeV2ContinuityChallengeArtifact,
    pub freeze: RuntimeV2ContinuityChallengeFreezeArtifact,
    pub appeal_review: RuntimeV2ContinuityAppealReviewArtifact,
    pub threat_model: RuntimeV2CitizenStateThreatModel,
    pub economics_placement: RuntimeV2EconomicsPlacementRecord,
}

impl RuntimeV2ContinuityChallengeArtifacts {
    pub fn prototype() -> Result<Self> {
        let access_control_artifacts = runtime_v2_access_control_contract()?;
        let sanctuary_artifacts = runtime_v2_private_state_sanctuary_contract()?;
        let challenge = RuntimeV2ContinuityChallengeArtifact::from_artifacts(
            &access_control_artifacts,
            &sanctuary_artifacts,
        )?;
        let freeze = RuntimeV2ContinuityChallengeFreezeArtifact::from_challenge(&challenge)?;
        let appeal_review = RuntimeV2ContinuityAppealReviewArtifact::from_challenge_and_freeze(
            &challenge, &freeze,
        )?;
        let threat_model = RuntimeV2CitizenStateThreatModel::from_artifacts(
            &challenge,
            &freeze,
            &appeal_review,
            &access_control_artifacts,
            &sanctuary_artifacts,
        )?;
        let economics_placement = RuntimeV2EconomicsPlacementRecord::prototype();
        let artifacts = Self {
            access_control_artifacts,
            sanctuary_artifacts,
            challenge,
            freeze,
            appeal_review,
            threat_model,
            economics_placement,
        };
        artifacts.validate()?;
        Ok(artifacts)
    }

    pub fn validate(&self) -> Result<()> {
        self.access_control_artifacts.validate()?;
        self.sanctuary_artifacts.validate()?;
        self.challenge
            .validate_against(&self.access_control_artifacts, &self.sanctuary_artifacts)?;
        self.freeze.validate_against(&self.challenge)?;
        self.appeal_review
            .validate_against(&self.challenge, &self.freeze)?;
        self.threat_model.validate_against(
            &self.challenge,
            &self.freeze,
            &self.appeal_review,
            &self.access_control_artifacts,
            &self.sanctuary_artifacts,
        )?;
        self.economics_placement.validate()
    }

    pub fn write_to_root(&self, root: impl AsRef<Path>) -> Result<()> {
        self.validate()?;
        let root = root.as_ref();
        write_relative(
            root,
            RUNTIME_V2_CONTINUITY_CHALLENGE_PATH,
            self.challenge.pretty_json_bytes()?,
        )?;
        write_relative(
            root,
            RUNTIME_V2_CONTINUITY_CHALLENGE_FREEZE_PATH,
            self.freeze.pretty_json_bytes()?,
        )?;
        write_relative(
            root,
            RUNTIME_V2_CONTINUITY_APPEAL_REVIEW_PATH,
            self.appeal_review.pretty_json_bytes()?,
        )?;
        write_relative(
            root,
            RUNTIME_V2_CITIZEN_STATE_THREAT_MODEL_PATH,
            self.threat_model.pretty_json_bytes()?,
        )?;
        write_relative(
            root,
            RUNTIME_V2_ECONOMICS_PLACEMENT_PATH,
            self.economics_placement.pretty_json_bytes()?,
        )?;
        Ok(())
    }
}

impl RuntimeV2ContinuityChallengeArtifact {
    pub fn from_artifacts(
        access: &RuntimeV2AccessControlArtifacts,
        sanctuary: &RuntimeV2PrivateStateSanctuaryArtifacts,
    ) -> Result<Self> {
        access.validate()?;
        sanctuary.validate()?;
        let challenge_event = access
            .event_packet
            .events
            .iter()
            .find(|event| event.access_path == "challenge")
            .ok_or_else(|| anyhow!("access event packet must include challenge event"))?;
        let appeal_event = access
            .event_packet
            .events
            .iter()
            .find(|event| event.access_path == "appeal")
            .ok_or_else(|| anyhow!("access event packet must include appeal event"))?;
        let challenge = Self {
            schema_version: RUNTIME_V2_CONTINUITY_CHALLENGE_SCHEMA.to_string(),
            challenge_id: "continuity-challenge-proto-citizen-alpha-0001".to_string(),
            demo_id: "D11".to_string(),
            artifact_path: RUNTIME_V2_CONTINUITY_CHALLENGE_PATH.to_string(),
            citizen_id: sanctuary.quarantine_artifact.citizen_id.clone(),
            lineage_id: sanctuary.quarantine_artifact.lineage_id.clone(),
            challenged_transition_kinds: strings(&["wake", "projection"]),
            challenge_reason:
                "wake or projection continuity is disputed before destructive transition or public projection"
                    .to_string(),
            raised_by_actor_id: challenge_event.actor_id.clone(),
            raised_by_standing_class: challenge_event.standing_class.clone(),
            source_access_event_refs: vec![
                format!("{}#{}", access.event_packet.artifact_path, challenge_event.event_id),
                format!("{}#{}", access.event_packet.artifact_path, appeal_event.event_id),
            ],
            source_quarantine_ref: sanctuary.quarantine_artifact.artifact_path.clone(),
            source_observatory_projection_ref: access
                .observatory_artifacts
                .projection_packet
                .artifact_path
                .clone(),
            expected_freeze_ref: RUNTIME_V2_CONTINUITY_CHALLENGE_FREEZE_PATH.to_string(),
            required_evidence_refs: strings(&[
                "access_event_challenge",
                "access_event_appeal",
                "sanctuary_quarantine_artifact",
                "redacted_observatory_projection",
                "continuity_witness_or_review_resolution",
            ]),
            destructive_transition_frozen: true,
            projection_publication_frozen: true,
            raw_private_state_disclosed: false,
            continuity_mutation_allowed: false,
            status: "accepted_pending_review".to_string(),
            claim_boundary:
                "WP-13 proves bounded continuity challenge intake for D11; it freezes challenged wake and projection paths but does not implement v0.90.4 economics, payment rails, or v0.92 identity rebinding."
                    .to_string(),
        };
        challenge.validate_against(access, sanctuary)?;
        Ok(challenge)
    }

    pub fn validate_against(
        &self,
        access: &RuntimeV2AccessControlArtifacts,
        sanctuary: &RuntimeV2PrivateStateSanctuaryArtifacts,
    ) -> Result<()> {
        self.validate_shape()?;
        if self.source_quarantine_ref != sanctuary.quarantine_artifact.artifact_path {
            return Err(anyhow!(
                "challenge must bind to sanctuary quarantine artifact"
            ));
        }
        if self.source_observatory_projection_ref
            != access.observatory_artifacts.projection_packet.artifact_path
        {
            return Err(anyhow!(
                "challenge must bind to redacted projection artifact"
            ));
        }
        for required in ["challenge", "appeal"] {
            let event = access
                .event_packet
                .events
                .iter()
                .find(|event| event.access_path == required)
                .ok_or_else(|| anyhow!("missing access event for {required}"))?;
            let expected = format!("{}#{}", access.event_packet.artifact_path, event.event_id);
            if !self.source_access_event_refs.contains(&expected) {
                return Err(anyhow!("challenge must cite {required} access event"));
            }
        }
        Ok(())
    }

    pub fn validate_shape(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_CONTINUITY_CHALLENGE_SCHEMA {
            return Err(anyhow!(
                "unsupported Runtime v2 continuity challenge schema '{}'",
                self.schema_version
            ));
        }
        normalize_id(self.challenge_id.clone(), "challenge.challenge_id")?;
        validate_demo_id(&self.demo_id, "challenge.demo_id")?;
        validate_relative_path(&self.artifact_path, "challenge.artifact_path")?;
        validate_relative_path(&self.expected_freeze_ref, "challenge.expected_freeze_ref")?;
        validate_relative_path(
            &self.source_quarantine_ref,
            "challenge.source_quarantine_ref",
        )?;
        validate_relative_path(
            &self.source_observatory_projection_ref,
            "challenge.source_observatory_projection_ref",
        )?;
        validate_required_values(
            &self.challenged_transition_kinds,
            &["wake", "projection"],
            "challenge.challenged_transition_kinds",
        )?;
        if !self.destructive_transition_frozen {
            return Err(anyhow!(
                "challenged destructive transition must freeze safely"
            ));
        }
        if !self.projection_publication_frozen {
            return Err(anyhow!(
                "challenged projection must freeze before publication"
            ));
        }
        if self.raw_private_state_disclosed {
            return Err(anyhow!("challenge must not disclose raw private state"));
        }
        if self.continuity_mutation_allowed {
            return Err(anyhow!("challenge must not permit continuity mutation"));
        }
        if self.status != "accepted_pending_review" {
            return Err(anyhow!(
                "challenge status must remain accepted_pending_review"
            ));
        }
        validate_required_texts(
            &self.required_evidence_refs,
            "challenge.required_evidence_refs",
            "challenge must preserve review evidence",
        )?;
        validate_required_texts(
            &self.source_access_event_refs,
            "challenge.source_access_event_refs",
            "challenge must cite access events",
        )?;
        validate_boundary(&self.claim_boundary, "challenge.claim_boundary")
    }

    pub fn pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.validate_shape()?;
        serde_json::to_vec_pretty(self).context("serialize Runtime v2 continuity challenge")
    }
}

impl RuntimeV2ContinuityChallengeFreezeArtifact {
    pub fn from_challenge(challenge: &RuntimeV2ContinuityChallengeArtifact) -> Result<Self> {
        challenge.validate_shape()?;
        let mut freeze = Self {
            schema_version: RUNTIME_V2_CONTINUITY_CHALLENGE_FREEZE_SCHEMA.to_string(),
            freeze_id: "continuity-challenge-freeze-proto-citizen-alpha-0001".to_string(),
            demo_id: "D11".to_string(),
            artifact_path: RUNTIME_V2_CONTINUITY_CHALLENGE_FREEZE_PATH.to_string(),
            source_challenge_ref: challenge.artifact_path.clone(),
            citizen_id: challenge.citizen_id.clone(),
            lineage_id: challenge.lineage_id.clone(),
            frozen_transition_kinds: challenge.challenged_transition_kinds.clone(),
            state_machine: vec![
                transition(
                    1,
                    "challenge_received",
                    "accept_challenge",
                    "freeze_entered",
                    "challenge_has_standing_and_auditable_event",
                    &challenge.artifact_path,
                ),
                transition(
                    2,
                    "freeze_entered",
                    "block_wake_activation",
                    "wake_frozen_pending_review",
                    "destructive_transition_must_not_advance_active_head",
                    &challenge.source_quarantine_ref,
                ),
                transition(
                    3,
                    "wake_frozen_pending_review",
                    "block_projection_publication",
                    "wake_and_projection_frozen_pending_review",
                    "projection_must_not_publish_private_state_while_challenged",
                    &challenge.source_observatory_projection_ref,
                ),
                transition(
                    4,
                    "wake_and_projection_frozen_pending_review",
                    "preserve_review_evidence",
                    "evidence_preserved_for_appeal",
                    "source_events_quarantine_and_projection_refs_must_remain_immutable",
                    &challenge.artifact_path,
                ),
            ],
            blocked_actions: strings(&[
                "activate_challenged_wake",
                "publish_challenged_projection",
                "advance_continuity_head",
                "release_from_quarantine_without_review_resolution",
                "prune_challenge_evidence",
            ]),
            preserved_evidence_refs: challenge.required_evidence_refs.clone(),
            destructive_transition_allowed: false,
            projection_publication_allowed: false,
            continuity_sequence_before: 42,
            continuity_sequence_after: 42,
            active_head_changed: false,
            raw_private_state_disclosed: false,
            release_requires_review_resolution: true,
            freeze_hash: String::new(),
            claim_boundary:
                "This freeze artifact proves challenged wake/projection freeze semantics for D11; it does not adjudicate full appeals, implement markets, or rebind identity."
                    .to_string(),
        };
        freeze.freeze_hash = freeze.computed_hash();
        freeze.validate_against(challenge)?;
        Ok(freeze)
    }

    pub fn validate_against(&self, challenge: &RuntimeV2ContinuityChallengeArtifact) -> Result<()> {
        challenge.validate_shape()?;
        self.validate_shape()?;
        if self.source_challenge_ref != challenge.artifact_path {
            return Err(anyhow!("freeze must bind to challenge artifact"));
        }
        if self.citizen_id != challenge.citizen_id || self.lineage_id != challenge.lineage_id {
            return Err(anyhow!("freeze must preserve challenged citizen lineage"));
        }
        if self.frozen_transition_kinds != challenge.challenged_transition_kinds {
            return Err(anyhow!(
                "freeze must cover every challenged transition kind"
            ));
        }
        if self.freeze_hash != self.computed_hash() {
            return Err(anyhow!("freeze hash must match canonical freeze content"));
        }
        Ok(())
    }

    pub fn validate_shape(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_CONTINUITY_CHALLENGE_FREEZE_SCHEMA {
            return Err(anyhow!(
                "unsupported Runtime v2 continuity freeze schema '{}'",
                self.schema_version
            ));
        }
        normalize_id(self.freeze_id.clone(), "freeze.freeze_id")?;
        validate_demo_id(&self.demo_id, "freeze.demo_id")?;
        validate_relative_path(&self.artifact_path, "freeze.artifact_path")?;
        validate_relative_path(&self.source_challenge_ref, "freeze.source_challenge_ref")?;
        validate_required_values(
            &self.frozen_transition_kinds,
            &["wake", "projection"],
            "freeze.frozen_transition_kinds",
        )?;
        validate_freeze_transitions(&self.state_machine)?;
        validate_required_texts(
            &self.blocked_actions,
            "freeze.blocked_actions",
            "freeze must block unsafe challenge actions",
        )?;
        validate_required_texts(
            &self.preserved_evidence_refs,
            "freeze.preserved_evidence_refs",
            "freeze must preserve review evidence",
        )?;
        if self.destructive_transition_allowed {
            return Err(anyhow!("challenged destructive transition freezes safely"));
        }
        if self.projection_publication_allowed {
            return Err(anyhow!(
                "challenged projection publication must freeze safely"
            ));
        }
        if self.continuity_sequence_before != self.continuity_sequence_after {
            return Err(anyhow!("freeze must not advance continuity sequence"));
        }
        if self.active_head_changed {
            return Err(anyhow!("freeze must not change the active citizen head"));
        }
        if self.raw_private_state_disclosed {
            return Err(anyhow!("freeze must not disclose raw private state"));
        }
        if !self.release_requires_review_resolution {
            return Err(anyhow!("freeze release must require review resolution"));
        }
        normalize_hash(&self.freeze_hash, "freeze.freeze_hash")?;
        validate_boundary(&self.claim_boundary, "freeze.claim_boundary")
    }

    pub fn computed_hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(self.schema_version.as_bytes());
        hasher.update(self.freeze_id.as_bytes());
        hasher.update(self.demo_id.as_bytes());
        hasher.update(self.artifact_path.as_bytes());
        hasher.update(self.source_challenge_ref.as_bytes());
        hasher.update(self.citizen_id.as_bytes());
        hasher.update(self.lineage_id.as_bytes());
        for kind in &self.frozen_transition_kinds {
            hasher.update(kind.as_bytes());
        }
        for transition in &self.state_machine {
            hasher.update(transition.sequence.to_string().as_bytes());
            hasher.update(transition.from_state.as_bytes());
            hasher.update(transition.event.as_bytes());
            hasher.update(transition.to_state.as_bytes());
            hasher.update(transition.guard.as_bytes());
            hasher.update(transition.evidence_ref.as_bytes());
        }
        for action in &self.blocked_actions {
            hasher.update(action.as_bytes());
        }
        for evidence in &self.preserved_evidence_refs {
            hasher.update(evidence.as_bytes());
        }
        hasher.update(self.destructive_transition_allowed.to_string().as_bytes());
        hasher.update(self.projection_publication_allowed.to_string().as_bytes());
        hasher.update(self.continuity_sequence_before.to_string().as_bytes());
        hasher.update(self.continuity_sequence_after.to_string().as_bytes());
        hasher.update(self.active_head_changed.to_string().as_bytes());
        hasher.update(self.raw_private_state_disclosed.to_string().as_bytes());
        hasher.update(
            self.release_requires_review_resolution
                .to_string()
                .as_bytes(),
        );
        format!("{:x}", hasher.finalize())
    }

    pub fn pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.validate_shape()?;
        serde_json::to_vec_pretty(self).context("serialize Runtime v2 continuity challenge freeze")
    }
}

impl RuntimeV2ContinuityAppealReviewArtifact {
    pub fn from_challenge_and_freeze(
        challenge: &RuntimeV2ContinuityChallengeArtifact,
        freeze: &RuntimeV2ContinuityChallengeFreezeArtifact,
    ) -> Result<Self> {
        freeze.validate_against(challenge)?;
        let appeal = Self {
            schema_version: RUNTIME_V2_CONTINUITY_APPEAL_REVIEW_SCHEMA.to_string(),
            appeal_id: "continuity-appeal-review-proto-citizen-alpha-0001".to_string(),
            demo_id: "D11".to_string(),
            artifact_path: RUNTIME_V2_CONTINUITY_APPEAL_REVIEW_PATH.to_string(),
            source_challenge_ref: challenge.artifact_path.clone(),
            source_freeze_ref: freeze.artifact_path.clone(),
            citizen_id: challenge.citizen_id.clone(),
            lineage_id: challenge.lineage_id.clone(),
            appellant_actor_id: challenge.raised_by_actor_id.clone(),
            appellant_standing_class: challenge.raised_by_standing_class.clone(),
            reviewer_standing_classes: strings(&["authorized_reviewer", "operator"]),
            permitted_outcomes: strings(&[
                "uphold_freeze_pending_valid_proof",
                "release_with_new_continuity_proof",
                "escalate_to_quarantine",
            ]),
            current_outcome: "uphold_freeze_pending_valid_proof".to_string(),
            release_allowed: false,
            destructive_transition_allowed: false,
            required_resolution_evidence: strings(&[
                "reviewer_identity_and_authority",
                "continuity_proof_or_quarantine_reason",
                "access_event_packet",
                "freeze_hash",
                "citizen_facing_continuity_explanation",
            ]),
            due_process_guarantees: strings(&[
                "challenge remains visible to citizen and reviewer",
                "evidence is preserved before destructive transition",
                "release requires explicit review resolution",
                "appeal cannot disclose raw private state",
                "appeal cannot bypass quarantine release requirements",
            ]),
            claim_boundary:
                "This artifact proves bounded appeal/review-resolution shape for D11; it does not implement a full constitutional court, markets, payment rails, or v0.92 birthday semantics."
                    .to_string(),
        };
        appeal.validate_against(challenge, freeze)?;
        Ok(appeal)
    }

    pub fn validate_against(
        &self,
        challenge: &RuntimeV2ContinuityChallengeArtifact,
        freeze: &RuntimeV2ContinuityChallengeFreezeArtifact,
    ) -> Result<()> {
        self.validate_shape()?;
        if self.source_challenge_ref != challenge.artifact_path {
            return Err(anyhow!("appeal review must bind to challenge artifact"));
        }
        if self.source_freeze_ref != freeze.artifact_path {
            return Err(anyhow!("appeal review must bind to freeze artifact"));
        }
        if self.citizen_id != challenge.citizen_id || self.lineage_id != challenge.lineage_id {
            return Err(anyhow!("appeal review must preserve citizen lineage"));
        }
        Ok(())
    }

    pub fn validate_shape(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_CONTINUITY_APPEAL_REVIEW_SCHEMA {
            return Err(anyhow!(
                "unsupported Runtime v2 appeal review schema '{}'",
                self.schema_version
            ));
        }
        normalize_id(self.appeal_id.clone(), "appeal.appeal_id")?;
        validate_demo_id(&self.demo_id, "appeal.demo_id")?;
        validate_relative_path(&self.artifact_path, "appeal.artifact_path")?;
        validate_relative_path(&self.source_challenge_ref, "appeal.source_challenge_ref")?;
        validate_relative_path(&self.source_freeze_ref, "appeal.source_freeze_ref")?;
        validate_required_texts(
            &self.reviewer_standing_classes,
            "appeal.reviewer_standing_classes",
            "appeal must name reviewer authority",
        )?;
        validate_required_texts(
            &self.permitted_outcomes,
            "appeal.permitted_outcomes",
            "appeal must name permitted outcomes",
        )?;
        if !self
            .permitted_outcomes
            .contains(&"release_with_new_continuity_proof".to_string())
        {
            return Err(anyhow!("appeal must require proof before release"));
        }
        if self.current_outcome != "uphold_freeze_pending_valid_proof" {
            return Err(anyhow!(
                "prototype appeal must remain frozen pending valid proof"
            ));
        }
        if self.release_allowed || self.destructive_transition_allowed {
            return Err(anyhow!(
                "appeal cannot release or permit destructive transition without resolution proof"
            ));
        }
        validate_required_texts(
            &self.required_resolution_evidence,
            "appeal.required_resolution_evidence",
            "appeal must require resolution evidence",
        )?;
        validate_required_texts(
            &self.due_process_guarantees,
            "appeal.due_process_guarantees",
            "appeal must preserve due-process guarantees",
        )?;
        validate_boundary(&self.claim_boundary, "appeal.claim_boundary")
    }

    pub fn pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.validate_shape()?;
        serde_json::to_vec_pretty(self).context("serialize Runtime v2 appeal review")
    }
}

impl RuntimeV2CitizenStateThreatModel {
    pub fn from_artifacts(
        challenge: &RuntimeV2ContinuityChallengeArtifact,
        freeze: &RuntimeV2ContinuityChallengeFreezeArtifact,
        appeal: &RuntimeV2ContinuityAppealReviewArtifact,
        access: &RuntimeV2AccessControlArtifacts,
        sanctuary: &RuntimeV2PrivateStateSanctuaryArtifacts,
    ) -> Result<Self> {
        appeal.validate_against(challenge, freeze)?;
        access.validate()?;
        sanctuary.validate()?;
        let model = Self {
            schema_version: RUNTIME_V2_CITIZEN_STATE_THREAT_MODEL_SCHEMA.to_string(),
            model_id: "citizen-state-threat-model-v0-90-3-d11".to_string(),
            demo_id: "D11".to_string(),
            artifact_path: RUNTIME_V2_CITIZEN_STATE_THREAT_MODEL_PATH.to_string(),
            challenge_ref: challenge.artifact_path.clone(),
            freeze_ref: freeze.artifact_path.clone(),
            appeal_review_ref: appeal.artifact_path.clone(),
            access_matrix_ref: access.authority_matrix.artifact_path.clone(),
            quarantine_ref: sanctuary.quarantine_artifact.artifact_path.clone(),
            threats: prototype_threats(challenge, freeze, appeal, access, sanctuary),
            universal_controls: strings(&[
                "fail closed before wake activation or projection publication",
                "emit auditable access events for challenge and appeal",
                "preserve evidence until review resolution",
                "deny raw private-state disclosure",
                "deny release from quarantine without continuity proof",
            ]),
            claim_boundary:
                "WP-13 threat model covers bounded citizen-state abuse paths for D11; it is not a complete security program, market design, or cloud enclave mandate."
                    .to_string(),
        };
        model.validate_against(challenge, freeze, appeal, access, sanctuary)?;
        Ok(model)
    }

    pub fn validate_against(
        &self,
        challenge: &RuntimeV2ContinuityChallengeArtifact,
        freeze: &RuntimeV2ContinuityChallengeFreezeArtifact,
        appeal: &RuntimeV2ContinuityAppealReviewArtifact,
        access: &RuntimeV2AccessControlArtifacts,
        sanctuary: &RuntimeV2PrivateStateSanctuaryArtifacts,
    ) -> Result<()> {
        self.validate_shape()?;
        if self.challenge_ref != challenge.artifact_path {
            return Err(anyhow!("threat model must bind to challenge artifact"));
        }
        if self.freeze_ref != freeze.artifact_path {
            return Err(anyhow!("threat model must bind to freeze artifact"));
        }
        if self.appeal_review_ref != appeal.artifact_path {
            return Err(anyhow!("threat model must bind to appeal review artifact"));
        }
        if self.access_matrix_ref != access.authority_matrix.artifact_path {
            return Err(anyhow!("threat model must bind to access matrix"));
        }
        if self.quarantine_ref != sanctuary.quarantine_artifact.artifact_path {
            return Err(anyhow!("threat model must bind to quarantine artifact"));
        }
        Ok(())
    }

    pub fn validate_shape(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_CITIZEN_STATE_THREAT_MODEL_SCHEMA {
            return Err(anyhow!(
                "unsupported Runtime v2 citizen-state threat model schema '{}'",
                self.schema_version
            ));
        }
        normalize_id(self.model_id.clone(), "threat_model.model_id")?;
        validate_demo_id(&self.demo_id, "threat_model.demo_id")?;
        validate_relative_path(&self.artifact_path, "threat_model.artifact_path")?;
        validate_relative_path(&self.challenge_ref, "threat_model.challenge_ref")?;
        validate_relative_path(&self.freeze_ref, "threat_model.freeze_ref")?;
        validate_relative_path(&self.appeal_review_ref, "threat_model.appeal_review_ref")?;
        validate_relative_path(&self.access_matrix_ref, "threat_model.access_matrix_ref")?;
        validate_relative_path(&self.quarantine_ref, "threat_model.quarantine_ref")?;
        validate_required_threat_order(
            self.threats.iter().map(|threat| threat.threat_id.as_str()),
        )?;
        for threat in &self.threats {
            threat.validate()?;
        }
        validate_required_texts(
            &self.universal_controls,
            "threat_model.universal_controls",
            "threat model must name universal controls",
        )?;
        validate_boundary(&self.claim_boundary, "threat_model.claim_boundary")
    }

    pub fn pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.validate_shape()?;
        serde_json::to_vec_pretty(self).context("serialize Runtime v2 citizen-state threat model")
    }
}

impl RuntimeV2CitizenStateThreat {
    pub fn validate(&self) -> Result<()> {
        normalize_id(self.threat_id.clone(), "threat.threat_id")?;
        validate_required_texts(
            &self.required_controls,
            "threat.required_controls",
            "threat must name controls",
        )?;
        validate_required_texts(
            &self.detection_artifacts,
            "threat.detection_artifacts",
            "threat must name detection artifacts",
        )?;
        if self.actor_class.trim().is_empty()
            || self.attack_path.trim().is_empty()
            || self.fail_closed_behavior.trim().is_empty()
        {
            return Err(anyhow!("threat fields must not be empty"));
        }
        if !self.covered_by_wp13 {
            return Err(anyhow!("required WP-13 threat is not covered"));
        }
        Ok(())
    }
}

impl RuntimeV2EconomicsPlacementRecord {
    pub fn prototype() -> Self {
        Self {
            schema_version: RUNTIME_V2_ECONOMICS_PLACEMENT_SCHEMA.to_string(),
            record_id: "economics-placement-v0-90-3-resource-stewardship-bridge".to_string(),
            demo_id: "D11".to_string(),
            artifact_path: RUNTIME_V2_ECONOMICS_PLACEMENT_PATH.to_string(),
            decision: "resource_stewardship_bridge_only_before_v0_90_4".to_string(),
            rationale:
                "v0.90.3 needs continuity-preserving resource stewardship around challenge, freeze, quarantine, and review; full economics belongs to v0.90.4."
                    .to_string(),
            allowed_v0_90_3_scope: strings(&[
                "record resource stewardship obligations for evidence retention",
                "refuse cost optimizations that weaken continuity guarantees",
                "surface operator review costs as governance facts",
                "defer market allocation to the economics milestone",
            ]),
            deferred_to_v0_90_4: strings(&[
                "markets",
                "payment_rails",
                "bidding",
                "subcontracting",
                "inter_polis_trade",
            ]),
            markets_implemented: false,
            payment_rails_implemented: false,
            bidding_implemented: false,
            subcontracting_implemented: false,
            continuity_priority_rule:
                "cost optimization must never override citizen continuity, privacy, evidence preservation, or due process"
                    .to_string(),
            claim_boundary:
                "This record places economics for v0.90.3 as a narrow resource-stewardship bridge only; it explicitly does not implement markets, payment rails, bidding, subcontracting, or inter-polis trade."
                    .to_string(),
        }
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_ECONOMICS_PLACEMENT_SCHEMA {
            return Err(anyhow!(
                "unsupported Runtime v2 economics placement schema '{}'",
                self.schema_version
            ));
        }
        normalize_id(self.record_id.clone(), "economics.record_id")?;
        validate_demo_id(&self.demo_id, "economics.demo_id")?;
        validate_relative_path(&self.artifact_path, "economics.artifact_path")?;
        if self.decision != "resource_stewardship_bridge_only_before_v0_90_4" {
            return Err(anyhow!(
                "economics record must remain a narrow bridge decision"
            ));
        }
        validate_required_texts(
            &self.allowed_v0_90_3_scope,
            "economics.allowed_v0_90_3_scope",
            "economics record must name allowed bridge scope",
        )?;
        validate_required_texts(
            &self.deferred_to_v0_90_4,
            "economics.deferred_to_v0_90_4",
            "economics record must name deferred economics scope",
        )?;
        if self.markets_implemented
            || self.payment_rails_implemented
            || self.bidding_implemented
            || self.subcontracting_implemented
        {
            return Err(anyhow!(
                "economics record does not implement markets or payment rails"
            ));
        }
        if !self
            .continuity_priority_rule
            .contains("never override citizen continuity")
        {
            return Err(anyhow!(
                "economics must keep continuity above cost optimization"
            ));
        }
        validate_boundary(&self.claim_boundary, "economics.claim_boundary")
    }

    pub fn pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.validate()?;
        serde_json::to_vec_pretty(self).context("serialize Runtime v2 economics placement")
    }
}

fn prototype_threats(
    challenge: &RuntimeV2ContinuityChallengeArtifact,
    freeze: &RuntimeV2ContinuityChallengeFreezeArtifact,
    appeal: &RuntimeV2ContinuityAppealReviewArtifact,
    access: &RuntimeV2AccessControlArtifacts,
    sanctuary: &RuntimeV2PrivateStateSanctuaryArtifacts,
) -> Vec<RuntimeV2CitizenStateThreat> {
    vec![
        threat(
            "insider_operator_abuse",
            "operator",
            "operator attempts raw inspection, unauthorized release, or destructive transition despite challenge",
            &[
                "access authority matrix",
                "challenge artifact",
                "freeze artifact",
                "appeal review evidence",
            ],
            &[
                &access.authority_matrix.artifact_path,
                &challenge.artifact_path,
                &freeze.artifact_path,
                &appeal.artifact_path,
            ],
            "freeze remains active and release is denied until review resolution",
        ),
        threat(
            "compromised_key",
            "credential_or_key_adversary",
            "compromised signing or encryption key tries to resume or release stale state",
            &[
                "continuity proof required",
                "key evolution must preserve verifiability",
                "release requires review resolution",
            ],
            &[
                &freeze.artifact_path,
                &appeal.artifact_path,
                &sanctuary.quarantine_artifact.artifact_path,
            ],
            "state remains frozen or quarantined until key continuity is verified",
        ),
        threat(
            "malicious_guest",
            "guest",
            "guest attempts to challenge, inspect, or project private citizen state without standing",
            &[
                "standing policy",
                "access denial events",
                "raw private-state disclosure denied",
            ],
            &[
                &access.standing_artifacts.policy.artifact_path,
                &access.event_packet.artifact_path,
                &challenge.artifact_path,
            ],
            "challenge path records the attempt but does not disclose or mutate citizen state",
        ),
        threat(
            "equivocation",
            "runtime_or_storage_adversary",
            "two incompatible successor claims appear for the same citizen lineage",
            &[
                "anti-equivocation conflict evidence",
                "sanctuary quarantine",
                "freeze active head",
            ],
            &[
                &sanctuary.anti_equivocation_artifacts.conflict.artifact_path,
                &sanctuary.quarantine_artifact.artifact_path,
                &freeze.artifact_path,
            ],
            "activation is blocked and evidence is preserved for review",
        ),
        threat(
            "replay",
            "replay_adversary",
            "old wake, projection, or release claim is replayed after newer accepted state",
            &[
                "continuity sequence check",
                "predecessor linkage check",
                "review-resolution requirement",
            ],
            &[
                &challenge.artifact_path,
                &freeze.artifact_path,
                &access.event_packet.artifact_path,
            ],
            "replayed claim cannot advance sequence or active head",
        ),
        threat(
            "projection_leakage",
            "projection_consumer_or_service_actor",
            "redacted projection is widened into raw private-state disclosure during a challenge",
            &[
                "observatory redaction policy",
                "projection publication freeze",
                "raw private-state disclosure denial",
            ],
            &[
                &access.observatory_artifacts.redaction_policy.artifact_path,
                &access.observatory_artifacts.projection_packet.artifact_path,
                &freeze.artifact_path,
            ],
            "projection publication remains frozen and raw state is never disclosed",
        ),
        threat(
            "unsafe_release_from_quarantine",
            "operator_or_runtime_release_path",
            "quarantined state is released without continuity proof, review record, or preserved evidence",
            &[
                "quarantine release requirements",
                "appeal review evidence",
                "release denied until continuity proof",
            ],
            &[
                &sanctuary.quarantine_artifact.artifact_path,
                &appeal.artifact_path,
                &access.denial_fixtures.proof_id,
            ],
            "release is denied and the citizen remains in sanctuary/quarantine pending review",
        ),
    ]
}

fn transition(
    sequence: u64,
    from_state: &str,
    event: &str,
    to_state: &str,
    guard: &str,
    evidence_ref: &str,
) -> RuntimeV2ContinuityFreezeTransition {
    RuntimeV2ContinuityFreezeTransition {
        sequence,
        from_state: from_state.to_string(),
        event: event.to_string(),
        to_state: to_state.to_string(),
        guard: guard.to_string(),
        evidence_ref: evidence_ref.to_string(),
    }
}

fn threat(
    threat_id: &str,
    actor_class: &str,
    attack_path: &str,
    required_controls: &[&str],
    detection_artifacts: &[&str],
    fail_closed_behavior: &str,
) -> RuntimeV2CitizenStateThreat {
    RuntimeV2CitizenStateThreat {
        threat_id: threat_id.to_string(),
        actor_class: actor_class.to_string(),
        attack_path: attack_path.to_string(),
        required_controls: strings(required_controls),
        detection_artifacts: strings(detection_artifacts),
        fail_closed_behavior: fail_closed_behavior.to_string(),
        covered_by_wp13: true,
    }
}

fn validate_freeze_transitions(transitions: &[RuntimeV2ContinuityFreezeTransition]) -> Result<()> {
    if transitions.len() != 4 {
        return Err(anyhow!(
            "freeze state machine must contain four transitions"
        ));
    }
    for (index, transition) in transitions.iter().enumerate() {
        if transition.sequence != (index as u64) + 1 {
            return Err(anyhow!(
                "freeze state machine must preserve deterministic sequence"
            ));
        }
        normalize_id(
            transition.from_state.clone(),
            "freeze_transition.from_state",
        )?;
        normalize_id(transition.event.clone(), "freeze_transition.event")?;
        normalize_id(transition.to_state.clone(), "freeze_transition.to_state")?;
        normalize_id(transition.guard.clone(), "freeze_transition.guard")?;
        validate_relative_path(&transition.evidence_ref, "freeze_transition.evidence_ref")?;
    }
    Ok(())
}

fn validate_required_threat_order<'a>(ids: impl Iterator<Item = &'a str>) -> Result<()> {
    let observed = ids.collect::<Vec<_>>();
    if observed != REQUIRED_THREATS {
        return Err(anyhow!(
            "threat model must cover insider/operator abuse, compromised key, malicious guest, equivocation, replay, projection leakage, and unsafe release from quarantine"
        ));
    }
    Ok(())
}

fn validate_required_values(values: &[String], expected: &[&str], field: &str) -> Result<()> {
    if values.len() != expected.len() {
        return Err(anyhow!("{field} must contain required values exactly once"));
    }
    let mut seen = BTreeSet::new();
    for (actual, expected_value) in values.iter().zip(expected.iter()) {
        if actual != expected_value {
            return Err(anyhow!("{field} must preserve deterministic order"));
        }
        if !seen.insert(actual) {
            return Err(anyhow!("{field} contains duplicate value '{actual}'"));
        }
    }
    Ok(())
}

fn validate_required_texts(values: &[String], field: &str, message: &str) -> Result<()> {
    if values.is_empty() {
        return Err(anyhow!("{message}"));
    }
    let mut seen = BTreeSet::new();
    for value in values {
        if value.trim().is_empty() {
            return Err(anyhow!("{field} must not contain empty values"));
        }
        if !seen.insert(value) {
            return Err(anyhow!("{field} contains duplicate value '{value}'"));
        }
    }
    Ok(())
}

fn validate_boundary(value: &str, field: &str) -> Result<()> {
    if value.trim().is_empty() {
        return Err(anyhow!("{field} must not be empty"));
    }
    for forbidden in [
        "payment rails are implemented",
        "markets are implemented",
        "first true Godel-agent birth",
        "v0.92 identity rebinding is implemented",
    ] {
        if value.contains(forbidden) {
            return Err(anyhow!("{field} contains an out-of-scope claim"));
        }
    }
    Ok(())
}

fn validate_demo_id(value: &str, field: &str) -> Result<()> {
    if value != "D11" {
        return Err(anyhow!("{field} must be D11"));
    }
    Ok(())
}

fn normalize_hash(value: &str, field: &str) -> Result<()> {
    if value.len() != 64 || !value.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err(anyhow!("{field} must be a sha256 hex digest"));
    }
    Ok(())
}

fn strings(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_string()).collect()
}
