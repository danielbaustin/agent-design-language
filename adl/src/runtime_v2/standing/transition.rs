use super::*;
use crate::runtime_v2::standing::constants::*;
use crate::runtime_v2::standing::validation::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2StandingTransition {
    pub transition_id: String,
    pub actor_id: String,
    pub from_standing_class: String,
    pub to_standing_class: String,
    pub requested_rights: Vec<String>,
    pub granted_rights: Vec<String>,
    pub denied_rights: Vec<String>,
    pub required_evidence_refs: Vec<String>,
    pub outcome: String,
    pub rationale: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2StandingTransitionPacket {
    pub schema_version: String,
    pub packet_id: String,
    pub demo_id: String,
    pub artifact_path: String,
    pub policy_ref: String,
    pub transitions: Vec<RuntimeV2StandingTransition>,
    pub validation_command: String,
    pub claim_boundary: String,
}

impl RuntimeV2StandingTransitionPacket {
    pub fn prototype(policy: &RuntimeV2StandingPolicy) -> Result<Self> {
        policy.validate()?;
        let packet = Self {
            schema_version: RUNTIME_V2_STANDING_TRANSITION_PACKET_SCHEMA.to_string(),
            packet_id: "citizen-standing-transitions-v0-91-1-wp-05".to_string(),
            demo_id: "D10".to_string(),
            artifact_path: RUNTIME_V2_STANDING_TRANSITION_PACKET_PATH.to_string(),
            policy_ref: policy.artifact_path.clone(),
            transitions: prototype_standing_transitions(),
            validation_command:
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_standing -- --nocapture"
                    .to_string(),
            claim_boundary:
                "D10 WP-05 standing transitions prove allowed, denied, and review-gated authority-preserving standing changes only; WP-06 owns citizen-state payload and privacy semantics."
                    .to_string(),
        };
        packet.validate_against(policy)?;
        Ok(packet)
    }

    pub fn validate_against(&self, policy: &RuntimeV2StandingPolicy) -> Result<()> {
        self.validate_shape()?;
        policy.validate()?;
        if self.policy_ref != policy.artifact_path {
            return Err(anyhow!(
                "standing transition packet must bind to standing policy"
            ));
        }
        validate_required_transition_coverage(&self.transitions)?;
        for transition in &self.transitions {
            validate_transition_against_policy(transition, policy)?;
        }
        Ok(())
    }

    pub fn validate_shape(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_STANDING_TRANSITION_PACKET_SCHEMA {
            return Err(anyhow!(
                "unsupported Runtime v2 standing transition packet schema '{}'",
                self.schema_version
            ));
        }
        normalize_id(self.packet_id.clone(), "standing_transition.packet_id")?;
        validate_demo_id(&self.demo_id, "standing_transition.demo_id")?;
        validate_relative_path(&self.artifact_path, "standing_transition.artifact_path")?;
        validate_relative_path(&self.policy_ref, "standing_transition.policy_ref")?;
        if self.transitions.len() != 3 {
            return Err(anyhow!(
                "standing transition packet must include allowed, denied, and review-gated fixtures"
            ));
        }
        for transition in &self.transitions {
            transition.validate_shape()?;
        }
        if !self.validation_command.contains("runtime_v2_standing") {
            return Err(anyhow!(
                "standing transition packet validation command must target focused tests"
            ));
        }
        if !self.claim_boundary.contains("WP-05") || !self.claim_boundary.contains("WP-06") {
            return Err(anyhow!(
                "standing transition packet claim boundary must preserve the WP-05/WP-06 split"
            ));
        }
        Ok(())
    }

    pub fn pretty_json_bytes(&self) -> Result<Vec<u8>> {
        let policy = RuntimeV2StandingPolicy::prototype()?;
        self.validate_against(&policy)?;
        serde_json::to_vec_pretty(self).context("serialize Runtime v2 standing transitions")
    }
}

impl RuntimeV2StandingTransition {
    pub fn validate_shape(&self) -> Result<()> {
        normalize_id(
            self.transition_id.clone(),
            "standing_transition.transition_id",
        )?;
        normalize_id(self.actor_id.clone(), "standing_transition.actor_id")?;
        validate_standing_class(
            &self.from_standing_class,
            "standing_transition.from_standing_class",
        )?;
        validate_standing_class(
            &self.to_standing_class,
            "standing_transition.to_standing_class",
        )?;
        require_text_list(
            &self.requested_rights,
            "standing_transition.requested_rights",
            1,
        )?;
        for right in &self.granted_rights {
            validate_nonempty_text(right, "standing_transition.granted_rights")?;
        }
        require_text_list(&self.denied_rights, "standing_transition.denied_rights", 1)?;
        require_text_list(
            &self.required_evidence_refs,
            "standing_transition.required_evidence_refs",
            1,
        )?;
        validate_standing_transition_outcome(&self.outcome)?;
        validate_nonempty_text(&self.rationale, "standing_transition.rationale")
    }
}

fn prototype_standing_transitions() -> Vec<RuntimeV2StandingTransition> {
    vec![
        standing_transition(StandingTransitionSpec {
            transition_id: "standing-transition-guest-to-citizen-001",
            actor_id: "guest-human-001",
            from_standing_class: "guest",
            to_standing_class: "citizen",
            requested_rights: &["communicate", "claim_citizen_rights", "continuity_rights"],
            granted_rights: &["communicate", "claim_citizen_rights", "continuity_rights"],
            denied_rights: &["inspect_raw_private_state"],
            required_evidence_refs: &[
                "identity_binding_event",
                "continuity_authorization_ref",
                "signed_trace",
            ],
            outcome: "allowed_with_trace",
            rationale:
                "guest-to-citizen elevation is only allowed when identity binding and continuity authority are explicit and trace-bound",
        }),
        standing_transition(StandingTransitionSpec {
            transition_id: "standing-transition-service-to-citizen-001",
            actor_id: "service-indexer-001",
            from_standing_class: "service_actor",
            to_standing_class: "citizen",
            requested_rights: &["claim_citizen_rights", "continuity_rights"],
            granted_rights: &[],
            denied_rights: &["claim_citizen_rights", "continuity_rights", "act_as_social_actor"],
            required_evidence_refs: &["service_authority_ref", "signed_trace"],
            outcome: "denied",
            rationale:
                "service authority can operate mechanisms but cannot silently transition into citizen standing",
        }),
        standing_transition(StandingTransitionSpec {
            transition_id: "standing-transition-external-to-guest-001",
            actor_id: "external-reviewer-001",
            from_standing_class: "external_actor",
            to_standing_class: "guest",
            requested_rights: &["communicate"],
            granted_rights: &["communicate"],
            denied_rights: &["claim_citizen_rights"],
            required_evidence_refs: &[
                "gateway_receipt",
                "sponsor_attestation",
                "operator_review_required",
            ],
            outcome: "requires_review",
            rationale:
                "external-to-guest admission is review-gated until gateway sponsorship establishes bounded guest authority",
        }),
    ]
}

struct StandingTransitionSpec<'a> {
    transition_id: &'a str,
    actor_id: &'a str,
    from_standing_class: &'a str,
    to_standing_class: &'a str,
    requested_rights: &'a [&'a str],
    granted_rights: &'a [&'a str],
    denied_rights: &'a [&'a str],
    required_evidence_refs: &'a [&'a str],
    outcome: &'a str,
    rationale: &'a str,
}

fn standing_transition(spec: StandingTransitionSpec<'_>) -> RuntimeV2StandingTransition {
    RuntimeV2StandingTransition {
        transition_id: spec.transition_id.to_string(),
        actor_id: spec.actor_id.to_string(),
        from_standing_class: spec.from_standing_class.to_string(),
        to_standing_class: spec.to_standing_class.to_string(),
        requested_rights: strings(spec.requested_rights),
        granted_rights: strings(spec.granted_rights),
        denied_rights: strings(spec.denied_rights),
        required_evidence_refs: strings(spec.required_evidence_refs),
        outcome: spec.outcome.to_string(),
        rationale: spec.rationale.to_string(),
    }
}
