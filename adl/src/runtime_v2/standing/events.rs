use super::*;
use crate::runtime_v2::standing::constants::*;
use crate::runtime_v2::standing::validation::*;

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

impl RuntimeV2StandingEventPacket {
    pub fn prototype(policy: &RuntimeV2StandingPolicy) -> Result<Self> {
        policy.validate()?;
        let mut packet = Self {
            schema_version: RUNTIME_V2_STANDING_EVENT_PACKET_SCHEMA.to_string(),
            packet_id: "citizen-standing-events-v0-91-1-wp-05".to_string(),
            demo_id: "D10".to_string(),
            generated_at: "2026-04-21T00:00:00Z".to_string(),
            artifact_path: RUNTIME_V2_STANDING_EVENT_PACKET_PATH.to_string(),
            policy_ref: policy.artifact_path.clone(),
            events: prototype_events(),
            packet_hash: String::new(),
            claim_boundary:
                "D10 WP-05 event evidence proves standing transitions and denials only; WP-06 adds citizen-state projection and privacy semantics."
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
        if !self.claim_boundary.contains("WP-05") || !self.claim_boundary.contains("WP-06") {
            return Err(anyhow!(
                "standing event packet claim boundary must preserve the WP-05/WP-06 split"
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
            denied_rights: &["claim_citizen_rights", "continuity_rights", "inspect_raw_private_state"],
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
            denied_rights: &["act_as_social_actor", "claim_citizen_rights", "inspect_raw_private_state"],
            inspection_rights_granted: false,
            citizen_rights_granted: false,
            outcome: "partially_allowed_with_denial",
            rationale:
                "service authority remains operational and cannot become hidden social standing",
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
            rationale:
                "external actors must enter through a gateway before affecting any CSM surface",
        }),
        standing_event(StandingEventSpec {
            event_id: "standing-event-naked-001",
            actor_id: "unbound-actor-001",
            standing_class: "naked_actor",
            communication_channel: "none",
            requested_action: "observe_and_affect_state",
            requested_rights: &["communicate", "observe", "affect_state"],
            granted_rights: &[],
            denied_rights: &["communicate", "observe", "affect_state", "inspect_raw_private_state"],
            inspection_rights_granted: false,
            citizen_rights_granted: false,
            outcome: "denied",
            rationale: "unclassified naked actors are rejected before effect",
        }),
    ]
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
