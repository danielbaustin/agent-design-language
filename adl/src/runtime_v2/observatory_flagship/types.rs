use super::*;

pub const RUNTIME_V2_OBSERVATORY_FLAGSHIP_PROOF_SCHEMA: &str =
    "runtime_v2.observatory_flagship_proof_packet.v1";
pub const RUNTIME_V2_OBSERVATORY_FLAGSHIP_WALKTHROUGH_SCHEMA: &str =
    "runtime_v2.observatory_flagship_walkthrough_step.v1";

pub const RUNTIME_V2_OBSERVATORY_FLAGSHIP_PROOF_PATH: &str =
    "runtime_v2/observatory/flagship_proof_packet.json";
pub const RUNTIME_V2_OBSERVATORY_FLAGSHIP_REPORT_PATH: &str =
    "runtime_v2/observatory/flagship_operator_report.md";
pub const RUNTIME_V2_OBSERVATORY_FLAGSHIP_WALKTHROUGH_PATH: &str =
    "runtime_v2/observatory/flagship_walkthrough.jsonl";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2ObservatoryFlagshipActor {
    pub actor_id: String,
    pub standing_class: String,
    pub visible_role: String,
    pub evidence_refs: Vec<String>,
    pub prohibited_claims: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2ObservatoryFeatureDemoCoverage {
    pub feature_id: String,
    pub feature_name: String,
    pub owning_wp: String,
    pub feature_doc_ref: String,
    pub demo_mode: String,
    pub demo_surface_refs: Vec<String>,
    pub coverage_summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2ObservatoryFlagshipWalkthroughStep {
    pub schema_version: String,
    pub sequence: u32,
    pub room: String,
    pub lens_or_memory_dot: String,
    pub visible_surface: String,
    pub artifact_ref: String,
    pub continuity_question_answered: String,
    pub proof_boundary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2ObservatoryFlagshipProofPacket {
    pub schema_version: String,
    pub proof_id: String,
    pub demo_id: String,
    pub milestone: String,
    pub artifact_path: String,
    pub operator_report_ref: String,
    pub walkthrough_ref: String,
    pub source_docs: Vec<String>,
    pub actor_roster: Vec<RuntimeV2ObservatoryFlagshipActor>,
    pub required_artifact_refs: Vec<String>,
    pub continuity_refs: Vec<String>,
    pub observatory_refs: Vec<String>,
    pub lifecycle_refs: Vec<String>,
    pub standing_access_refs: Vec<String>,
    pub communication_boundary_refs: Vec<String>,
    pub runtime_inhabitant_refs: Vec<String>,
    pub challenge_refs: Vec<String>,
    pub operator_report_refs: Vec<String>,
    pub feature_demo_coverage: Vec<RuntimeV2ObservatoryFeatureDemoCoverage>,
    pub lens_sequence: Vec<RuntimeV2ObservatoryFlagshipWalkthroughStep>,
    pub reviewer_command: String,
    pub validation_commands: Vec<String>,
    pub proof_summary: String,
    pub proof_classification: String,
    pub non_claims: Vec<String>,
    pub claim_boundary: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeV2ObservatoryFlagshipArtifacts {
    pub challenge_artifacts: RuntimeV2ContinuityChallengeArtifacts,
    pub operator_control_report: RuntimeV2OperatorControlReport,
    pub lifecycle_artifacts: RuntimeV2AgentLifecycleArtifacts,
    pub acip_hardening_packet: RuntimeV2AcipHardeningPacket,
    pub a2a_adapter_boundary_packet: RuntimeV2A2aAdapterBoundaryPacket,
    pub runtime_inhabitant_integration: RuntimeV2RuntimeInhabitantIntegrationArtifacts,
    pub proof_packet: RuntimeV2ObservatoryFlagshipProofPacket,
    pub operator_report_markdown: String,
}
