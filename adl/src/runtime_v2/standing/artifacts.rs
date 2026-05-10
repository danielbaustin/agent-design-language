use super::*;
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeV2StandingArtifacts {
    pub policy: RuntimeV2StandingPolicy,
    pub event_packet: RuntimeV2StandingEventPacket,
    pub communication_examples: RuntimeV2StandingCommunicationExamples,
    pub transition_packet: RuntimeV2StandingTransitionPacket,
    pub negative_cases: RuntimeV2StandingNegativeCases,
}

impl RuntimeV2StandingArtifacts {
    pub fn prototype() -> Result<Self> {
        let policy = RuntimeV2StandingPolicy::prototype()?;
        let event_packet = RuntimeV2StandingEventPacket::prototype(&policy)?;
        let communication_examples = RuntimeV2StandingCommunicationExamples::prototype(&policy)?;
        let transition_packet = RuntimeV2StandingTransitionPacket::prototype(&policy)?;
        let negative_cases = RuntimeV2StandingNegativeCases::prototype();
        let artifacts = Self {
            policy,
            event_packet,
            communication_examples,
            transition_packet,
            negative_cases,
        };
        artifacts.validate()?;
        Ok(artifacts)
    }

    pub fn validate(&self) -> Result<()> {
        self.policy.validate()?;
        self.event_packet.validate_against(&self.policy)?;
        self.communication_examples.validate_against(&self.policy)?;
        self.transition_packet.validate_against(&self.policy)?;
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
            RUNTIME_V2_STANDING_TRANSITION_PACKET_PATH,
            self.transition_packet.pretty_json_bytes()?,
        )?;
        write_relative(
            root,
            RUNTIME_V2_STANDING_NEGATIVE_CASES_PATH,
            self.negative_cases.pretty_json_bytes()?,
        )?;
        Ok(())
    }
}
