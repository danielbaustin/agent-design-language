//! Runtime-v2 cultivating-intelligence contract.
//!
//! WP-14 turns formation evidence into a bounded, reviewable runtime surface.
//! The packet must stay trace-linked, operational, and explicit about the
//! adjacent v0.91.1 capability/intelligence/memory/ToM boundary.

use super::*;

pub const CULTIVATING_INTELLIGENCE_PACKET_SCHEMA_VERSION: &str =
    "cultivating_intelligence_review_packet.v1";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CultivationDimensionDefinition {
    pub dimension_id: String,
    pub display_name: String,
    pub purpose: String,
    pub evidence_field_refs: Vec<String>,
    pub interpretation_boundary: String,
    pub limitations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CultivationReviewCriterion {
    pub criterion_id: String,
    pub dimension_id: String,
    pub review_question: String,
    pub evidence_requirements: Vec<String>,
    pub pass_condition: String,
    pub limitations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CultivationBoundaryReference {
    pub boundary_ref_id: String,
    pub boundary_kind: String,
    pub doc_path: String,
    pub summary: String,
    pub deferred_work: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CultivationDimensionAssessment {
    pub dimension_id: String,
    pub cultivation_level: String,
    pub summary: String,
    pub evidence_refs: Vec<String>,
    pub criterion_ids: Vec<String>,
    pub limitations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CultivationFixture {
    pub fixture_id: String,
    pub fixture_kind: String,
    pub scenario_summary: String,
    pub supporting_trace_refs: Vec<String>,
    pub supporting_outcome_linkage_refs: Vec<String>,
    pub supporting_trajectory_finding_refs: Vec<String>,
    pub supporting_wellbeing_fixture_refs: Vec<String>,
    pub supporting_moral_resource_claim_refs: Vec<String>,
    pub supporting_kindness_fixture_refs: Vec<String>,
    pub supporting_affect_fixture_refs: Vec<String>,
    pub supporting_humor_fixture_refs: Vec<String>,
    pub dimension_assessments: Vec<CultivationDimensionAssessment>,
    pub overall_outcome: String,
    pub interpretation_boundary: String,
    pub limitations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CultivationReviewFinding {
    pub finding_id: String,
    pub fixture_id: String,
    pub review_status: String,
    pub covered_dimension_ids: Vec<String>,
    pub summary: String,
    pub evidence_refs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CultivatingIntelligenceReviewPacket {
    pub schema_version: String,
    pub packet_id: String,
    pub summary: String,
    pub interpretation_boundary: String,
    pub deterministic_ordering_rule: String,
    pub dimensions: Vec<CultivationDimensionDefinition>,
    pub review_criteria: Vec<CultivationReviewCriterion>,
    pub boundary_refs: Vec<CultivationBoundaryReference>,
    pub fixtures: Vec<CultivationFixture>,
    pub review_findings: Vec<CultivationReviewFinding>,
}
