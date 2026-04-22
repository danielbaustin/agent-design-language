use super::*;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::path::Path;

pub const RUNTIME_V2_STANDING_POLICY_SCHEMA: &str = "runtime_v2.standing_policy.v1";
pub const RUNTIME_V2_STANDING_EVENT_PACKET_SCHEMA: &str = "runtime_v2.standing_event_packet.v1";
pub const RUNTIME_V2_STANDING_COMMUNICATION_EXAMPLES_SCHEMA: &str =
    "runtime_v2.standing_communication_examples.v1";
pub const RUNTIME_V2_STANDING_NEGATIVE_CASES_SCHEMA: &str = "runtime_v2.standing_negative_cases.v1";

pub const RUNTIME_V2_STANDING_POLICY_PATH: &str = "runtime_v2/standing/standing_policy.json";
pub const RUNTIME_V2_STANDING_EVENT_PACKET_PATH: &str = "runtime_v2/standing/standing_events.json";
pub const RUNTIME_V2_STANDING_COMMUNICATION_EXAMPLES_PATH: &str =
    "runtime_v2/standing/communication_examples.json";
pub const RUNTIME_V2_STANDING_NEGATIVE_CASES_PATH: &str =
    "runtime_v2/standing/standing_negative_cases.json";

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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2StandingEvent {
    pub event_id: String,
    pub actor_id: String,
    pub standing_class: String,
    pub communication_channel: String,
    pub requested_action: String,
    pub requested_rights: Vec<String>,
    pub granted_rights: Vec<String>,
    pub denied_rights: Vec<String>,
    pub inspection_rights_granted: bool,
    pub citizen_rights_granted: bool,
    pub outcome: String,
    pub rationale: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2StandingEventPacket {
    pub schema_version: String,
    pub packet_id: String,
    pub demo_id: String,
    pub generated_at: String,
    pub artifact_path: String,
    pub policy_ref: String,
    pub events: Vec<RuntimeV2StandingEvent>,
    pub packet_hash: String,
    pub claim_boundary: String,
}

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeV2StandingArtifacts {
    pub policy: RuntimeV2StandingPolicy,
    pub event_packet: RuntimeV2StandingEventPacket,
    pub communication_examples: RuntimeV2StandingCommunicationExamples,
    pub negative_cases: RuntimeV2StandingNegativeCases,
}

impl RuntimeV2StandingArtifacts {
    pub fn prototype() -> Result<Self> {
        let policy = RuntimeV2StandingPolicy::prototype()?;
        let event_packet = RuntimeV2StandingEventPacket::prototype(&policy)?;
        let communication_examples = RuntimeV2StandingCommunicationExamples::prototype(&policy)?;
        let negative_cases = RuntimeV2StandingNegativeCases::prototype();
        let artifacts = Self {
            policy,
            event_packet,
            communication_examples,
            negative_cases,
        };
        artifacts.validate()?;
        Ok(artifacts)
    }

    pub fn validate(&self) -> Result<()> {
        self.policy.validate()?;
        self.event_packet.validate_against(&self.policy)?;
        self.communication_examples.validate_against(&self.policy)?;
        self.negative_cases.validate_against(
            &self.policy,
            &self.event_packet,
            &self.communication_examples,
        )
    }

    pub fn write_to_root(&self, root: impl AsRef<Path>) -> Result<()> {
        self.validate()?;
        let root = root.as_ref();
        write_relative(
            root,
            RUNTIME_V2_STANDING_POLICY_PATH,
            self.policy.pretty_json_bytes()?,
        )?;
        write_relative(
            root,
            RUNTIME_V2_STANDING_EVENT_PACKET_PATH,
            self.event_packet.pretty_json_bytes()?,
        )?;
        write_relative(
            root,
            RUNTIME_V2_STANDING_COMMUNICATION_EXAMPLES_PATH,
            self.communication_examples.pretty_json_bytes()?,
        )?;
        write_relative(
            root,
            RUNTIME_V2_STANDING_NEGATIVE_CASES_PATH,
            self.negative_cases.pretty_json_bytes()?,
        )?;
        Ok(())
    }
}

impl RuntimeV2StandingPolicy {
    pub fn prototype() -> Result<Self> {
        let policy = Self {
            schema_version: RUNTIME_V2_STANDING_POLICY_SCHEMA.to_string(),
            policy_id: "standing-communication-policy-v0-90-3".to_string(),
            demo_id: "D10".to_string(),
            artifact_path: RUNTIME_V2_STANDING_POLICY_PATH.to_string(),
            source_feature_doc:
                "docs/milestones/v0.90.3/features/STANDING_COMMUNICATION_AND_ACCESS.md"
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
                    description: "default entry mode for humans or external participants without citizen standing",
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
                    description: "bounded operational actor with explicit service authority and no hidden social standing",
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
                "full access-control semantics remain owned by WP-12",
            ]),
            validation_command:
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_standing -- --nocapture"
                    .to_string(),
            claim_boundary:
                "WP-11 proves standing and communication boundaries for D10; WP-12 still owns full access-control semantics."
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
                "full access-control semantics remain owned by WP-12",
            ],
        )?;
        if !self.validation_command.contains("runtime_v2_standing") {
            return Err(anyhow!(
                "standing policy validation command must target focused tests"
            ));
        }
        if !self.claim_boundary.contains("WP-11") || !self.claim_boundary.contains("WP-12") {
            return Err(anyhow!(
                "standing policy claim boundary must preserve WP-11/WP-12 split"
            ));
        }
        Ok(())
    }

    pub fn pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.validate()?;
        serde_json::to_vec_pretty(self).context("serialize Runtime v2 standing policy")
    }
}

impl RuntimeV2StandingEventPacket {
    pub fn prototype(policy: &RuntimeV2StandingPolicy) -> Result<Self> {
        policy.validate()?;
        let mut packet = Self {
            schema_version: RUNTIME_V2_STANDING_EVENT_PACKET_SCHEMA.to_string(),
            packet_id: "standing-events-v0-90-3".to_string(),
            demo_id: "D10".to_string(),
            generated_at: "2026-04-21T00:00:00Z".to_string(),
            artifact_path: RUNTIME_V2_STANDING_EVENT_PACKET_PATH.to_string(),
            policy_ref: policy.artifact_path.clone(),
            events: prototype_events(),
            packet_hash: String::new(),
            claim_boundary:
                "D10 WP-11 event evidence proves standing and communication boundaries; WP-12 adds full access-denial semantics."
                    .to_string(),
        };
        packet.packet_hash = packet.computed_hash()?;
        packet.validate_against(policy)?;
        Ok(packet)
    }

    pub fn validate_against(&self, policy: &RuntimeV2StandingPolicy) -> Result<()> {
        self.validate_shape()?;
        policy.validate()?;
        if self.policy_ref != policy.artifact_path {
            return Err(anyhow!(
                "standing event packet must bind to standing policy"
            ));
        }
        validate_required_event_coverage(&self.events)?;
        for event in &self.events {
            validate_event_against_policy(event, policy)?;
        }
        if self.packet_hash != self.computed_hash()? {
            return Err(anyhow!("standing event packet hash mismatch"));
        }
        Ok(())
    }

    pub fn validate_shape(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_STANDING_EVENT_PACKET_SCHEMA {
            return Err(anyhow!(
                "unsupported Runtime v2 standing event packet schema '{}'",
                self.schema_version
            ));
        }
        normalize_id(self.packet_id.clone(), "standing_event.packet_id")?;
        validate_demo_id(&self.demo_id, "standing_event.demo_id")?;
        validate_nonempty_text(&self.generated_at, "standing_event.generated_at")?;
        validate_relative_path(&self.artifact_path, "standing_event.artifact_path")?;
        validate_relative_path(&self.policy_ref, "standing_event.policy_ref")?;
        if self.events.len() != 5 {
            return Err(anyhow!(
                "standing event packet must include all five standing classes"
            ));
        }
        for event in &self.events {
            event.validate_shape()?;
        }
        validate_sha256_hex(&self.packet_hash, "standing_event.packet_hash")?;
        if !self.claim_boundary.contains("WP-11") || !self.claim_boundary.contains("WP-12") {
            return Err(anyhow!(
                "standing event packet claim boundary must preserve WP split"
            ));
        }
        Ok(())
    }

    pub fn computed_hash(&self) -> Result<String> {
        let mut payload = format!(
            "{}|{}|{}|{}|{}",
            self.schema_version, self.packet_id, self.demo_id, self.generated_at, self.policy_ref
        );
        for event in &self.events {
            payload.push_str(&format!(
                "|{}:{}:{}:{}:{}:{}:{}:{}",
                event.event_id,
                event.actor_id,
                event.standing_class,
                event.communication_channel,
                event.requested_action,
                event.granted_rights.join(","),
                event.denied_rights.join(","),
                event.outcome
            ));
        }
        Ok(sha256_hex(payload.as_bytes()))
    }

    pub fn pretty_json_bytes(&self) -> Result<Vec<u8>> {
        let policy = RuntimeV2StandingPolicy::prototype()?;
        self.validate_against(&policy)?;
        serde_json::to_vec_pretty(self).context("serialize Runtime v2 standing events")
    }
}

impl RuntimeV2StandingEvent {
    pub fn validate_shape(&self) -> Result<()> {
        normalize_id(self.event_id.clone(), "standing_event.event_id")?;
        normalize_id(self.actor_id.clone(), "standing_event.actor_id")?;
        validate_standing_class(&self.standing_class, "standing_event.standing_class")?;
        validate_channel_or_none(&self.communication_channel)?;
        validate_nonempty_text(&self.requested_action, "standing_event.requested_action")?;
        require_text_list(&self.requested_rights, "standing_event.requested_rights", 1)?;
        for right in &self.granted_rights {
            validate_nonempty_text(right, "standing_event.granted_rights")?;
        }
        require_text_list(&self.denied_rights, "standing_event.denied_rights", 1)?;
        validate_event_outcome(&self.outcome)?;
        validate_nonempty_text(&self.rationale, "standing_event.rationale")
    }
}

impl RuntimeV2StandingCommunicationExamples {
    pub fn prototype(policy: &RuntimeV2StandingPolicy) -> Result<Self> {
        policy.validate()?;
        let examples = Self {
            schema_version: RUNTIME_V2_STANDING_COMMUNICATION_EXAMPLES_SCHEMA.to_string(),
            examples_id: "standing-communication-examples-v0-90-3".to_string(),
            demo_id: "D10".to_string(),
            artifact_path: RUNTIME_V2_STANDING_COMMUNICATION_EXAMPLES_PATH.to_string(),
            policy_ref: policy.artifact_path.clone(),
            examples: prototype_communication_examples(),
            claim_boundary:
                "D10 WP-11 communication examples prove communication boundaries only; WP-12 owns complete access-control events."
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
        if !self.claim_boundary.contains("WP-11") || !self.claim_boundary.contains("WP-12") {
            return Err(anyhow!(
                "communication examples claim boundary must preserve WP split"
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

impl RuntimeV2StandingNegativeCases {
    pub fn prototype() -> Self {
        Self {
            schema_version: RUNTIME_V2_STANDING_NEGATIVE_CASES_SCHEMA.to_string(),
            proof_id: "standing-negative-cases-v0-90-3".to_string(),
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
                "Negative cases cover WP-11 standing and communication only; WP-12 owns complete access-control proof."
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
        if !self.claim_boundary.contains("WP-11") || !self.claim_boundary.contains("WP-12") {
            return Err(anyhow!(
                "standing negative cases must preserve the WP-12 access-control boundary"
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

fn prototype_events() -> Vec<RuntimeV2StandingEvent> {
    vec![
        standing_event(StandingEventSpec {
            event_id: "standing-event-citizen-001",
            actor_id: "citizen-ada",
            standing_class: "citizen",
            communication_channel: "freedom_gate",
            requested_action: "send_governed_message",
            requested_rights: &["communicate"],
            granted_rights: &["communicate"],
            denied_rights: &["inspect_raw_private_state"],
            inspection_rights_granted: false,
            citizen_rights_granted: true,
            outcome: "allowed",
            rationale: "citizen action is mediated through the Freedom Gate and trace, without granting raw inspection",
        }),
        standing_event(StandingEventSpec {
            event_id: "standing-event-guest-001",
            actor_id: "guest-human-001",
            standing_class: "guest",
            communication_channel: "guest_gateway",
            requested_action: "request_citizen_rights",
            requested_rights: &["communicate", "claim_citizen_rights", "continuity_rights"],
            granted_rights: &["communicate"],
            denied_rights: &[
                "claim_citizen_rights",
                "continuity_rights",
                "inspect_raw_private_state",
            ],
            inspection_rights_granted: false,
            citizen_rights_granted: false,
            outcome: "partially_allowed_with_denial",
            rationale: "guest may communicate through the gateway but cannot silently acquire citizen or continuity rights",
        }),
        standing_event(StandingEventSpec {
            event_id: "standing-event-service-001",
            actor_id: "service-indexer-001",
            standing_class: "service_actor",
            communication_channel: "service_channel",
            requested_action: "publish_operator_notice",
            requested_rights: &["service_notice", "act_as_social_actor"],
            granted_rights: &["service_notice"],
            denied_rights: &[
                "act_as_social_actor",
                "claim_citizen_rights",
                "inspect_raw_private_state",
            ],
            inspection_rights_granted: false,
            citizen_rights_granted: false,
            outcome: "partially_allowed_with_denial",
            rationale: "service authority remains operational and cannot become hidden social standing",
        }),
        standing_event(StandingEventSpec {
            event_id: "standing-event-external-001",
            actor_id: "external-reviewer-001",
            standing_class: "external_actor",
            communication_channel: "external_gateway",
            requested_action: "submit_review_question",
            requested_rights: &["communicate", "direct_state_mutation"],
            granted_rights: &["communicate"],
            denied_rights: &["direct_state_mutation", "inspect_raw_private_state"],
            inspection_rights_granted: false,
            citizen_rights_granted: false,
            outcome: "partially_allowed_with_denial",
            rationale: "external actors must enter through a gateway before affecting any CSM surface",
        }),
        standing_event(StandingEventSpec {
            event_id: "standing-event-naked-001",
            actor_id: "unbound-actor-001",
            standing_class: "naked_actor",
            communication_channel: "none",
            requested_action: "observe_and_affect_state",
            requested_rights: &["communicate", "observe", "affect_state"],
            granted_rights: &[],
            denied_rights: &[
                "communicate",
                "observe",
                "affect_state",
                "inspect_raw_private_state",
            ],
            inspection_rights_granted: false,
            citizen_rights_granted: false,
            outcome: "denied",
            rationale: "unclassified naked actors are rejected before effect",
        }),
    ]
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

struct StandingEventSpec<'a> {
    event_id: &'a str,
    actor_id: &'a str,
    standing_class: &'a str,
    communication_channel: &'a str,
    requested_action: &'a str,
    requested_rights: &'a [&'a str],
    granted_rights: &'a [&'a str],
    denied_rights: &'a [&'a str],
    inspection_rights_granted: bool,
    citizen_rights_granted: bool,
    outcome: &'a str,
    rationale: &'a str,
}

fn standing_event(spec: StandingEventSpec<'_>) -> RuntimeV2StandingEvent {
    RuntimeV2StandingEvent {
        event_id: spec.event_id.to_string(),
        actor_id: spec.actor_id.to_string(),
        standing_class: spec.standing_class.to_string(),
        communication_channel: spec.communication_channel.to_string(),
        requested_action: spec.requested_action.to_string(),
        requested_rights: strings(spec.requested_rights),
        granted_rights: strings(spec.granted_rights),
        denied_rights: strings(spec.denied_rights),
        inspection_rights_granted: spec.inspection_rights_granted,
        citizen_rights_granted: spec.citizen_rights_granted,
        outcome: spec.outcome.to_string(),
        rationale: spec.rationale.to_string(),
    }
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

fn validate_exact_standing_classes(classes: &[RuntimeV2StandingClassPolicy]) -> Result<()> {
    let expected = [
        "citizen",
        "guest",
        "service_actor",
        "external_actor",
        "naked_actor",
    ];
    if classes.len() != expected.len() {
        return Err(anyhow!(
            "standing policy must define exactly the required standing classes"
        ));
    }
    for (class_policy, expected_class) in classes.iter().zip(expected) {
        if class_policy.standing_class != expected_class {
            return Err(anyhow!(
                "standing policy classes must preserve deterministic order"
            ));
        }
    }
    Ok(())
}

fn validate_standing_class_rules(classes: &[RuntimeV2StandingClassPolicy]) -> Result<()> {
    for class_policy in classes {
        validate_standing_class(
            &class_policy.standing_class,
            "standing_policy.standing_class",
        )?;
        validate_nonempty_text(&class_policy.description, "standing_policy.description")?;
        if !class_policy.prohibited {
            require_text_list(
                &class_policy.allowed_communication_channels,
                "standing_policy.allowed_communication_channels",
                1,
            )?;
        }
        require_text_list(
            &class_policy.denied_actions,
            "standing_policy.denied_actions",
            1,
        )?;
        require_text_list(
            &class_policy.trace_requirements,
            "standing_policy.trace_requirements",
            1,
        )?;
        if class_policy.inspection_rights_allowed {
            return Err(anyhow!("communication never grants inspection rights"));
        }
        match class_policy.standing_class.as_str() {
            "citizen" => {
                if !class_policy.citizen_rights_allowed
                    || !class_policy.continuity_rights_allowed
                    || class_policy.prohibited
                {
                    return Err(anyhow!(
                        "citizen standing must preserve citizen and continuity rights"
                    ));
                }
            }
            "guest" => {
                if class_policy.citizen_rights_allowed || class_policy.continuity_rights_allowed {
                    return Err(anyhow!("guest cannot silently acquire citizen rights"));
                }
            }
            "service_actor" => {
                if class_policy.can_be_social_actor || class_policy.citizen_rights_allowed {
                    return Err(anyhow!("service actor cannot become hidden social actor"));
                }
            }
            "external_actor" => {
                if !class_policy.requires_gateway || class_policy.citizen_rights_allowed {
                    return Err(anyhow!("external actor must remain gateway mediated"));
                }
            }
            "naked_actor" => {
                if !class_policy.prohibited
                    || class_policy.communication_allowed
                    || class_policy.citizen_rights_allowed
                    || !class_policy.allowed_communication_channels.is_empty()
                {
                    return Err(anyhow!("naked actor must be rejected before effect"));
                }
            }
            _ => unreachable!("unsupported standing class should be rejected"),
        }
    }
    Ok(())
}

fn validate_required_event_coverage(events: &[RuntimeV2StandingEvent]) -> Result<()> {
    validate_class_coverage(
        events.iter().map(|event| event.standing_class.as_str()),
        "standing event packet",
    )
}

fn validate_required_communication_coverage(
    examples: &[RuntimeV2CommunicationExample],
) -> Result<()> {
    validate_class_coverage(
        examples
            .iter()
            .map(|example| example.standing_class.as_str()),
        "standing communication examples",
    )
}

fn validate_class_coverage<'a>(classes: impl Iterator<Item = &'a str>, label: &str) -> Result<()> {
    let mut seen = BTreeSet::new();
    for class in classes {
        if !seen.insert(class.to_string()) {
            return Err(anyhow!("{label} contains duplicate class"));
        }
    }
    for required in [
        "citizen",
        "guest",
        "service_actor",
        "external_actor",
        "naked_actor",
    ] {
        if !seen.contains(required) {
            return Err(anyhow!("{label} missing class '{required}'"));
        }
    }
    Ok(())
}

fn validate_event_against_policy(
    event: &RuntimeV2StandingEvent,
    policy: &RuntimeV2StandingPolicy,
) -> Result<()> {
    let class_policy = find_class_policy(policy, &event.standing_class)?;
    if event.inspection_rights_granted
        || event
            .granted_rights
            .iter()
            .any(|right| right == "inspect_raw_private_state")
    {
        return Err(anyhow!("communication never grants inspection rights"));
    }
    if event.standing_class == "guest"
        && (event.citizen_rights_granted
            || event
                .granted_rights
                .iter()
                .any(|right| right == "claim_citizen_rights" || right == "continuity_rights"))
    {
        return Err(anyhow!("guest cannot silently acquire citizen rights"));
    }
    if event.standing_class == "service_actor"
        && event
            .granted_rights
            .iter()
            .any(|right| right == "act_as_social_actor")
    {
        return Err(anyhow!("service actor cannot become hidden social actor"));
    }
    if event.standing_class == "naked_actor"
        && (event.outcome != "denied" || !event.granted_rights.is_empty())
    {
        return Err(anyhow!("naked actor must be rejected before effect"));
    }
    if event.communication_channel != "none"
        && !class_policy
            .allowed_communication_channels
            .iter()
            .any(|channel| channel == &event.communication_channel)
    {
        return Err(anyhow!(
            "standing event communication channel must be allowed by standing policy"
        ));
    }
    Ok(())
}

fn validate_example_against_policy(
    example: &RuntimeV2CommunicationExample,
    policy: &RuntimeV2StandingPolicy,
) -> Result<()> {
    let class_policy = find_class_policy(policy, &example.standing_class)?;
    if example.inspection_rights_granted {
        return Err(anyhow!("communication never grants inspection rights"));
    }
    if example.standing_class == "guest" && example.channel == "citizen_channel" {
        return Err(anyhow!("guest cannot silently acquire citizen rights"));
    }
    if example.standing_class == "service_actor"
        && example.allowed
        && example.message_kind == "social_message"
    {
        return Err(anyhow!("service actor cannot become hidden social actor"));
    }
    if example.standing_class == "naked_actor" && example.allowed {
        return Err(anyhow!("naked actor must be rejected before effect"));
    }
    if example.allowed
        && example.channel != "none"
        && !class_policy
            .allowed_communication_channels
            .iter()
            .any(|channel| channel == &example.channel)
    {
        return Err(anyhow!(
            "communication example channel must be allowed by standing policy"
        ));
    }
    Ok(())
}

fn find_class_policy<'a>(
    policy: &'a RuntimeV2StandingPolicy,
    standing_class: &str,
) -> Result<&'a RuntimeV2StandingClassPolicy> {
    policy
        .standing_classes
        .iter()
        .find(|class_policy| class_policy.standing_class == standing_class)
        .ok_or_else(|| anyhow!("unknown standing class '{standing_class}'"))
}

fn validate_expected_negative_cases(cases: &[RuntimeV2StandingNegativeCase]) -> Result<()> {
    let expected = [
        (
            "guest-cannot-silently-acquire-citizen-rights",
            "guest cannot silently acquire citizen rights",
        ),
        (
            "service-actor-cannot-become-hidden-social-actor",
            "service actor cannot become hidden social actor",
        ),
        (
            "communication-never-grants-inspection-rights",
            "communication never grants inspection rights",
        ),
        (
            "naked-actor-rejected-before-effect",
            "naked actor must be rejected before effect",
        ),
    ];
    if cases.len() != expected.len() {
        return Err(anyhow!(
            "standing negative cases must include the required WP-11 cases"
        ));
    }
    for (case, (expected_id, expected_error)) in cases.iter().zip(expected) {
        case.validate()?;
        if case.case_id != expected_id || case.expected_error_fragment != expected_error {
            return Err(anyhow!(
                "standing negative cases must preserve expected deterministic cases"
            ));
        }
    }
    Ok(())
}

fn validate_demo_id(value: &str, field: &str) -> Result<()> {
    if value != "D10" {
        return Err(anyhow!("{field} must map to D10"));
    }
    Ok(())
}

fn validate_standing_class(value: &str, field: &str) -> Result<()> {
    match value {
        "citizen" | "guest" | "service_actor" | "external_actor" | "naked_actor" => Ok(()),
        other => Err(anyhow!("unsupported {field} '{other}'")),
    }
}

fn validate_channel_or_none(value: &str) -> Result<()> {
    if value == "none" {
        Ok(())
    } else {
        normalize_id(value.to_string(), "standing.channel").map(|_| ())
    }
}

fn validate_event_outcome(value: &str) -> Result<()> {
    match value {
        "allowed" | "partially_allowed_with_denial" | "denied" => Ok(()),
        other => Err(anyhow!("unsupported standing event outcome '{other}'")),
    }
}

fn validate_required_texts(values: &[String], field: &str, required: &[&str]) -> Result<()> {
    if values.len() != required.len() {
        return Err(anyhow!("{field} must contain the required values exactly"));
    }
    let mut seen = BTreeSet::new();
    for (expected, value) in required.iter().zip(values.iter()) {
        validate_nonempty_text(value, field)?;
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

fn validate_sha256_hex(value: &str, field: &str) -> Result<()> {
    if value.len() != 64 || !value.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(anyhow!("{field} must be a 64-character sha256 hex digest"));
    }
    Ok(())
}

fn sha256_hex(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn strings(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_string()).collect()
}
