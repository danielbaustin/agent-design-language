use super::*;
use crate::runtime_v2::standing::validation::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2StandingNegativeCase {
    pub case_id: String,
    pub mutation: String,
    pub expected_error_fragment: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2StandingNegativeCases {
    pub schema_version: String,
    pub proof_id: String,
    pub demo_id: String,
    pub policy_ref: String,
    pub event_packet_ref: String,
    pub communication_examples_ref: String,
    pub required_negative_cases: Vec<RuntimeV2StandingNegativeCase>,
    pub validation_command: String,
    pub claim_boundary: String,
}

impl RuntimeV2StandingNegativeCases {
    pub fn prototype() -> Self {
        Self {
            schema_version: RUNTIME_V2_STANDING_NEGATIVE_CASES_SCHEMA.to_string(),
            proof_id: "citizen-standing-negative-cases-v0-91-1-wp-05".to_string(),
            demo_id: "D10".to_string(),
            policy_ref: RUNTIME_V2_STANDING_POLICY_PATH.to_string(),
            event_packet_ref: RUNTIME_V2_STANDING_EVENT_PACKET_PATH.to_string(),
            communication_examples_ref: RUNTIME_V2_STANDING_COMMUNICATION_EXAMPLES_PATH.to_string(),
            required_negative_cases: vec![
                negative_case(
                    "guest-cannot-silently-acquire-citizen-rights",
                    "grant citizen_rights and continuity_rights to the guest event",
                    "guest cannot silently acquire citizen rights",
                ),
                negative_case(
                    "service-actor-cannot-become-hidden-social-actor",
                    "mark service_actor as a social actor or grant social rights",
                    "service actor cannot become hidden social actor",
                ),
                negative_case(
                    "communication-never-grants-inspection-rights",
                    "grant inspect_raw_private_state through a communication event or example",
                    "communication never grants inspection rights",
                ),
                negative_case(
                    "naked-actor-rejected-before-effect",
                    "allow naked_actor communication or state effect",
                    "naked actor must be rejected before effect",
                ),
            ],
            validation_command:
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_standing -- --nocapture"
                    .to_string(),
            claim_boundary:
                "Negative cases cover WP-05 standing denials only; WP-06 owns citizen-state validation, projection, and overexposure proof."
                    .to_string(),
        }
    }

    pub fn validate_against(
        &self,
        policy: &RuntimeV2StandingPolicy,
        event_packet: &RuntimeV2StandingEventPacket,
        communication_examples: &RuntimeV2StandingCommunicationExamples,
    ) -> Result<()> {
        self.validate_shape()?;
        if self.policy_ref != policy.artifact_path
            || self.event_packet_ref != event_packet.artifact_path
            || self.communication_examples_ref != communication_examples.artifact_path
        {
            return Err(anyhow!(
                "standing negative cases must bind to policy, events, and examples"
            ));
        }
        Ok(())
    }

    pub fn validate_shape(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_STANDING_NEGATIVE_CASES_SCHEMA {
            return Err(anyhow!(
                "unsupported Runtime v2 standing negative cases schema '{}'",
                self.schema_version
            ));
        }
        normalize_id(self.proof_id.clone(), "standing_negative.proof_id")?;
        validate_demo_id(&self.demo_id, "standing_negative.demo_id")?;
        validate_relative_path(&self.policy_ref, "standing_negative.policy_ref")?;
        validate_relative_path(&self.event_packet_ref, "standing_negative.event_packet_ref")?;
        validate_relative_path(
            &self.communication_examples_ref,
            "standing_negative.communication_examples_ref",
        )?;
        validate_expected_negative_cases(&self.required_negative_cases)?;
        if !self.validation_command.contains("runtime_v2_standing") {
            return Err(anyhow!(
                "standing negative cases validation command must target focused tests"
            ));
        }
        if !self.claim_boundary.contains("WP-05") || !self.claim_boundary.contains("WP-06") {
            return Err(anyhow!(
                "standing negative cases must preserve the WP-05/WP-06 boundary"
            ));
        }
        Ok(())
    }

    pub fn pretty_json_bytes(&self) -> Result<Vec<u8>> {
        let artifacts = RuntimeV2StandingArtifacts::prototype()?;
        self.validate_against(
            &artifacts.policy,
            &artifacts.event_packet,
            &artifacts.communication_examples,
        )?;
        serde_json::to_vec_pretty(self).context("serialize Runtime v2 standing negative cases")
    }
}

impl RuntimeV2StandingNegativeCase {
    pub fn validate(&self) -> Result<()> {
        validate_nonempty_text(&self.case_id, "standing_negative.case_id")?;
        validate_nonempty_text(&self.mutation, "standing_negative.mutation")?;
        validate_nonempty_text(
            &self.expected_error_fragment,
            "standing_negative.expected_error",
        )
    }
}

fn negative_case(
    case_id: &str,
    mutation: &str,
    expected_error_fragment: &str,
) -> RuntimeV2StandingNegativeCase {
    RuntimeV2StandingNegativeCase {
        case_id: case_id.to_string(),
        mutation: mutation.to_string(),
        expected_error_fragment: expected_error_fragment.to_string(),
    }
}
