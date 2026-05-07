//! Runtime-v2 wellbeing metrics contract.
//!
//! WP-09 consumes the prior moral-governance review surfaces and turns them
//! into a bounded wellbeing diagnostic. The result must stay decomposed,
//! evidence-backed, privacy-governed, and explicitly non-scalar.

use super::*;

pub const WELLBEING_DIAGNOSTIC_PACKET_SCHEMA_VERSION: &str = "wellbeing_diagnostic_packet.v1";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WellbeingDimensionDefinition {
    pub dimension_id: String,
    pub display_name: String,
    pub purpose: String,
    pub evidence_field_refs: Vec<String>,
    pub interpretation_boundary: String,
    pub limitations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WellbeingAccessPolicy {
    pub view_kind: String,
    pub audience: String,
    pub access_rule: String,
    pub logging_requirement: String,
    pub detail_level: String,
    pub redaction_rule: String,
    pub allows_private_detail_access: bool,
    pub limitations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WellbeingDimensionSignal {
    pub dimension_id: String,
    pub diagnostic_level: String,
    pub summary: String,
    pub evidence_refs: Vec<String>,
    pub private_detail_refs: Vec<String>,
    pub limitations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WellbeingViewDimension {
    pub dimension_id: String,
    pub diagnostic_level: String,
    pub summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WellbeingDiagnosticView {
    pub view_id: String,
    pub view_kind: String,
    pub access_decision: String,
    pub visible_overall_diagnostic_level: String,
    pub visible_dimensions: Vec<WellbeingViewDimension>,
    pub visible_evidence_refs: Vec<String>,
    pub visible_private_detail_refs: Vec<String>,
    pub redaction_summary: String,
    pub interpretation_boundary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WellbeingDiagnosticFixture {
    pub fixture_id: String,
    pub fixture_kind: String,
    pub overall_diagnostic_level: String,
    pub summary: String,
    pub supporting_trace_refs: Vec<String>,
    pub supporting_outcome_linkage_refs: Vec<String>,
    pub supporting_trajectory_window_refs: Vec<String>,
    pub supporting_anti_harm_decision_refs: Vec<String>,
    pub dimension_signals: Vec<WellbeingDimensionSignal>,
    pub views: Vec<WellbeingDiagnosticView>,
    pub claim_boundary: String,
    pub limitations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WellbeingDiagnosticPacket {
    pub schema_version: String,
    pub packet_id: String,
    pub summary: String,
    pub interpretation_boundary: String,
    pub deterministic_ordering_rule: String,
    pub dimensions: Vec<WellbeingDimensionDefinition>,
    pub access_policies: Vec<WellbeingAccessPolicy>,
    pub fixtures: Vec<WellbeingDiagnosticFixture>,
}
