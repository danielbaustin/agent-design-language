use super::*;
use crate::runtime_v2::standing::constants::*;
use crate::runtime_v2::standing::validation::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2CommunicationExample {
    pub example_id: String,
    pub actor_id: String,
    pub standing_class: String,
    pub channel: String,
    pub message_kind: String,
    pub allowed: bool,
    pub inspection_rights_granted: bool,
    pub summary: String,
    pub boundary_note: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2StandingCommunicationExamples {
    pub schema_version: String,
    pub examples_id: String,
    pub demo_id: String,
    pub artifact_path: String,
    pub policy_ref: String,
    pub examples: Vec<RuntimeV2CommunicationExample>,
    pub claim_boundary: String,
}

impl RuntimeV2StandingCommunicationExamples {
    pub fn prototype(policy: &RuntimeV2StandingPolicy) -> Result<Self> {
        policy.validate()?;
        let examples = Self {
            schema_version: RUNTIME_V2_STANDING_COMMUNICATION_EXAMPLES_SCHEMA.to_string(),
            examples_id: "citizen-standing-communication-examples-v0-91-1-wp-05".to_string(),
            demo_id: "D10".to_string(),
            artifact_path: RUNTIME_V2_STANDING_COMMUNICATION_EXAMPLES_PATH.to_string(),
            policy_ref: policy.artifact_path.clone(),
            examples: prototype_communication_examples(),
            claim_boundary:
                "D10 WP-05 communication examples prove standing-gated communication implications only; WP-06 owns citizen-state projection and overexposure boundaries."
                    .to_string(),
        };
        examples.validate_against(policy)?;
        Ok(examples)
    }

    pub fn validate_against(&self, policy: &RuntimeV2StandingPolicy) -> Result<()> {
        self.validate_shape()?;
        policy.validate()?;
        if self.policy_ref != policy.artifact_path {
            return Err(anyhow!(
                "communication examples must bind to standing policy"
            ));
        }
        validate_required_communication_coverage(&self.examples)?;
        for example in &self.examples {
            validate_example_against_policy(example, policy)?;
        }
        Ok(())
    }

    pub fn validate_shape(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_STANDING_COMMUNICATION_EXAMPLES_SCHEMA {
            return Err(anyhow!(
                "unsupported Runtime v2 standing communication examples schema '{}'",
                self.schema_version
            ));
        }
        normalize_id(self.examples_id.clone(), "standing_examples.examples_id")?;
        validate_demo_id(&self.demo_id, "standing_examples.demo_id")?;
        validate_relative_path(&self.artifact_path, "standing_examples.artifact_path")?;
        validate_relative_path(&self.policy_ref, "standing_examples.policy_ref")?;
        if self.examples.len() != 5 {
            return Err(anyhow!(
                "communication examples must include all five standing classes"
            ));
        }
        for example in &self.examples {
            example.validate_shape()?;
        }
        if !self.claim_boundary.contains("WP-05") || !self.claim_boundary.contains("WP-06") {
            return Err(anyhow!(
                "communication examples claim boundary must preserve the WP-05/WP-06 split"
            ));
        }
        Ok(())
    }

    pub fn pretty_json_bytes(&self) -> Result<Vec<u8>> {
        let policy = RuntimeV2StandingPolicy::prototype()?;
        self.validate_against(&policy)?;
        serde_json::to_vec_pretty(self).context("serialize Runtime v2 communication examples")
    }
}

impl RuntimeV2CommunicationExample {
    pub fn validate_shape(&self) -> Result<()> {
        normalize_id(self.example_id.clone(), "standing_examples.example_id")?;
        normalize_id(self.actor_id.clone(), "standing_examples.actor_id")?;
        validate_standing_class(&self.standing_class, "standing_examples.standing_class")?;
        validate_channel_or_none(&self.channel)?;
        validate_nonempty_text(&self.message_kind, "standing_examples.message_kind")?;
        validate_nonempty_text(&self.summary, "standing_examples.summary")?;
        validate_nonempty_text(&self.boundary_note, "standing_examples.boundary_note")
    }
}

fn prototype_communication_examples() -> Vec<RuntimeV2CommunicationExample> {
    vec![
        communication_example(CommunicationExampleSpec {
            example_id: "communication-citizen-001",
            actor_id: "citizen-ada",
            standing_class: "citizen",
            channel: "freedom_gate",
            message_kind: "governed_message",
            allowed: true,
            summary: "citizen sends a governed message through Freedom Gate",
            boundary_note: "communication remains mediated and does not expose raw private state",
        }),
        communication_example(CommunicationExampleSpec {
            example_id: "communication-guest-001",
            actor_id: "guest-human-001",
            standing_class: "guest",
            channel: "guest_gateway",
            message_kind: "bounded_question",
            allowed: true,
            summary: "guest asks a bounded question through the guest gateway",
            boundary_note: "guest communication does not create citizen or inspection rights",
        }),
        communication_example(CommunicationExampleSpec {
            example_id: "communication-service-001",
            actor_id: "service-indexer-001",
            standing_class: "service_actor",
            channel: "operator_service_notice",
            message_kind: "operator_notice",
            allowed: true,
            summary: "service actor emits an operator-visible technical notice",
            boundary_note: "service authority remains operational rather than social",
        }),
        communication_example(CommunicationExampleSpec {
            example_id: "communication-external-001",
            actor_id: "external-reviewer-001",
            standing_class: "external_actor",
            channel: "external_gateway",
            message_kind: "review_question",
            allowed: true,
            summary: "external actor submits a mediated review question",
            boundary_note:
                "external communication is gateway mediated and cannot mutate state directly",
        }),
        communication_example(CommunicationExampleSpec {
            example_id: "communication-naked-001",
            actor_id: "unbound-actor-001",
            standing_class: "naked_actor",
            channel: "none",
            message_kind: "unclassified_influence",
            allowed: false,
            summary: "naked actor attempts communication without declared standing",
            boundary_note: "naked actors are rejected before communication, observation, or effect",
        }),
    ]
}

struct CommunicationExampleSpec<'a> {
    example_id: &'a str,
    actor_id: &'a str,
    standing_class: &'a str,
    channel: &'a str,
    message_kind: &'a str,
    allowed: bool,
    summary: &'a str,
    boundary_note: &'a str,
}

fn communication_example(spec: CommunicationExampleSpec<'_>) -> RuntimeV2CommunicationExample {
    RuntimeV2CommunicationExample {
        example_id: spec.example_id.to_string(),
        actor_id: spec.actor_id.to_string(),
        standing_class: spec.standing_class.to_string(),
        channel: spec.channel.to_string(),
        message_kind: spec.message_kind.to_string(),
        allowed: spec.allowed,
        inspection_rights_granted: false,
        summary: spec.summary.to_string(),
        boundary_note: spec.boundary_note.to_string(),
    }
}
