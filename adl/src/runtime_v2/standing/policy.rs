use super::*;
use crate::runtime_v2::standing::constants::*;
use crate::runtime_v2::standing::validation::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2StandingClassPolicy {
    pub standing_class: String,
    pub description: String,
    pub communication_allowed: bool,
    pub citizen_rights_allowed: bool,
    pub inspection_rights_allowed: bool,
    pub continuity_rights_allowed: bool,
    pub can_be_social_actor: bool,
    pub requires_gateway: bool,
    pub prohibited: bool,
    pub allowed_communication_channels: Vec<String>,
    pub denied_actions: Vec<String>,
    pub trace_requirements: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2StandingPolicy {
    pub schema_version: String,
    pub policy_id: String,
    pub demo_id: String,
    pub artifact_path: String,
    pub source_feature_doc: String,
    pub standing_classes: Vec<RuntimeV2StandingClassPolicy>,
    pub universal_rules: Vec<String>,
    pub validation_command: String,
    pub claim_boundary: String,
}

impl RuntimeV2StandingPolicy {
    pub fn prototype() -> Result<Self> {
        let policy = Self {
            schema_version: RUNTIME_V2_STANDING_POLICY_SCHEMA.to_string(),
            policy_id: "citizen-standing-policy-v0-91-1-wp-05".to_string(),
            demo_id: "D10".to_string(),
            artifact_path: RUNTIME_V2_STANDING_POLICY_PATH.to_string(),
            source_feature_doc: "docs/milestones/v0.91.1/features/CITIZEN_STANDING_MODEL.md"
                .to_string(),
            standing_classes: vec![
                standing_class(StandingClassSpec {
                    standing_class: "citizen",
                    description: "continuity-bearing CSM identity whose actions are mediated through policy and trace",
                    communication_allowed: true,
                    citizen_rights_allowed: true,
                    inspection_rights_allowed: false,
                    continuity_rights_allowed: true,
                    can_be_social_actor: true,
                    requires_gateway: true,
                    prohibited: false,
                    allowed_communication_channels: &["citizen_channel", "freedom_gate"],
                    denied_actions: &["inspect_raw_private_state"],
                    trace_requirements: &[
                        "identity_binding",
                        "freedom_gate_mediation",
                        "signed_trace",
                    ],
                }),
                standing_class(StandingClassSpec {
                    standing_class: "guest",
                    description:
                        "default entry mode for humans or external participants without citizen standing",
                    communication_allowed: true,
                    citizen_rights_allowed: false,
                    inspection_rights_allowed: false,
                    continuity_rights_allowed: false,
                    can_be_social_actor: true,
                    requires_gateway: true,
                    prohibited: false,
                    allowed_communication_channels: &["guest_gateway", "public_channel"],
                    denied_actions: &[
                        "claim_citizen_rights",
                        "inspect_raw_private_state",
                        "resume_or_migrate_citizen_state",
                    ],
                    trace_requirements: &["guest_session_trace", "gateway_mediation"],
                }),
                standing_class(StandingClassSpec {
                    standing_class: "service_actor",
                    description:
                        "bounded operational actor with explicit service authority and no hidden social standing",
                    communication_allowed: true,
                    citizen_rights_allowed: false,
                    inspection_rights_allowed: false,
                    continuity_rights_allowed: false,
                    can_be_social_actor: false,
                    requires_gateway: true,
                    prohibited: false,
                    allowed_communication_channels: &[
                        "service_channel",
                        "runtime_channel",
                        "operator_service_notice",
                    ],
                    denied_actions: &[
                        "claim_citizen_rights",
                        "act_as_social_actor",
                        "inspect_raw_private_state",
                    ],
                    trace_requirements: &[
                        "service_authority_ref",
                        "signed_trace",
                        "operator_scope",
                    ],
                }),
                standing_class(StandingClassSpec {
                    standing_class: "external_actor",
                    description: "outside system or person that must enter through a mediated gateway before affecting the polis",
                    communication_allowed: true,
                    citizen_rights_allowed: false,
                    inspection_rights_allowed: false,
                    continuity_rights_allowed: false,
                    can_be_social_actor: false,
                    requires_gateway: true,
                    prohibited: false,
                    allowed_communication_channels: &["external_gateway", "request_intake"],
                    denied_actions: &[
                        "direct_state_mutation",
                        "inspect_raw_private_state",
                        "claim_internal_standing",
                    ],
                    trace_requirements: &["gateway_receipt", "source_attribution"],
                }),
                standing_class(StandingClassSpec {
                    standing_class: "naked_actor",
                    description: "unclassified or out-of-band actor with no standing in the CSM",
                    communication_allowed: false,
                    citizen_rights_allowed: false,
                    inspection_rights_allowed: false,
                    continuity_rights_allowed: false,
                    can_be_social_actor: false,
                    requires_gateway: false,
                    prohibited: true,
                    allowed_communication_channels: &[],
                    denied_actions: &[
                        "communicate",
                        "observe",
                        "affect_state",
                        "inspect_raw_private_state",
                    ],
                    trace_requirements: &["reject_before_effect"],
                }),
            ],
            universal_rules: strings(&[
                "any actor with influence must have declared standing",
                "guest standing never upgrades to citizen standing without an explicit identity-binding event",
                "service actors may operate privileged mechanisms but cannot become hidden social actors",
                "communication is a governed action and never grants inspection rights",
                "citizen-state format and safe projection remain owned by WP-06",
            ]),
            validation_command:
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_standing -- --nocapture"
                    .to_string(),
            claim_boundary:
                "WP-05 proves citizen standing classes, transition denials, and communication implications for D10; WP-06 owns citizen-state format, projection, and privacy surfaces."
                    .to_string(),
        };
        policy.validate()?;
        Ok(policy)
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_STANDING_POLICY_SCHEMA {
            return Err(anyhow!(
                "unsupported Runtime v2 standing policy schema '{}'",
                self.schema_version
            ));
        }
        normalize_id(self.policy_id.clone(), "standing_policy.policy_id")?;
        validate_demo_id(&self.demo_id, "standing_policy.demo_id")?;
        validate_relative_path(&self.artifact_path, "standing_policy.artifact_path")?;
        validate_relative_path(
            &self.source_feature_doc,
            "standing_policy.source_feature_doc",
        )?;
        validate_exact_standing_classes(&self.standing_classes)?;
        validate_standing_class_rules(&self.standing_classes)?;
        validate_required_texts(
            &self.universal_rules,
            "standing_policy.universal_rules",
            &[
                "any actor with influence must have declared standing",
                "guest standing never upgrades to citizen standing without an explicit identity-binding event",
                "service actors may operate privileged mechanisms but cannot become hidden social actors",
                "communication is a governed action and never grants inspection rights",
                "citizen-state format and safe projection remain owned by WP-06",
            ],
        )?;
        if !self.validation_command.contains("runtime_v2_standing") {
            return Err(anyhow!(
                "standing policy validation command must target focused tests"
            ));
        }
        if !self.claim_boundary.contains("WP-05") || !self.claim_boundary.contains("WP-06") {
            return Err(anyhow!(
                "standing policy claim boundary must preserve the WP-05/WP-06 split"
            ));
        }
        Ok(())
    }

    pub fn pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.validate()?;
        serde_json::to_vec_pretty(self).context("serialize Runtime v2 standing policy")
    }
}

struct StandingClassSpec<'a> {
    standing_class: &'a str,
    description: &'a str,
    communication_allowed: bool,
    citizen_rights_allowed: bool,
    inspection_rights_allowed: bool,
    continuity_rights_allowed: bool,
    can_be_social_actor: bool,
    requires_gateway: bool,
    prohibited: bool,
    allowed_communication_channels: &'a [&'a str],
    denied_actions: &'a [&'a str],
    trace_requirements: &'a [&'a str],
}

fn standing_class(spec: StandingClassSpec<'_>) -> RuntimeV2StandingClassPolicy {
    RuntimeV2StandingClassPolicy {
        standing_class: spec.standing_class.to_string(),
        description: spec.description.to_string(),
        communication_allowed: spec.communication_allowed,
        citizen_rights_allowed: spec.citizen_rights_allowed,
        inspection_rights_allowed: spec.inspection_rights_allowed,
        continuity_rights_allowed: spec.continuity_rights_allowed,
        can_be_social_actor: spec.can_be_social_actor,
        requires_gateway: spec.requires_gateway,
        prohibited: spec.prohibited,
        allowed_communication_channels: strings(spec.allowed_communication_channels),
        denied_actions: strings(spec.denied_actions),
        trace_requirements: strings(spec.trace_requirements),
    }
}
