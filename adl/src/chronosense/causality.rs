use serde::{Deserialize, Serialize};

use super::TEMPORAL_CAUSALITY_EXPLANATION_SCHEMA;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CausalRelationContract {
    pub relation_types: Vec<String>,
    pub sequence_boundary_rule: String,
    pub dependency_evidence_requirements: Vec<String>,
    pub uncertainty_classes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExplanationSurfaceContract {
    pub required_fields: Vec<String>,
    pub citation_requirements: Vec<String>,
    pub non_goals: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExplanationFixture {
    pub scenario: String,
    pub relation_type: String,
    pub confidence: String,
    pub explanation_note: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TemporalCausalityExplanationContract {
    pub schema_version: String,
    pub owned_runtime_surfaces: Vec<String>,
    pub causal_relations: CausalRelationContract,
    pub explanation_surface: ExplanationSurfaceContract,
    pub explanation_fixtures: Vec<ExplanationFixture>,
    pub proof_fixture_hooks: Vec<String>,
    pub proof_hook_command: String,
    pub proof_hook_output_path: String,
    pub scope_boundary: String,
}

impl TemporalCausalityExplanationContract {
    pub fn v1() -> Self {
        Self {
            schema_version: TEMPORAL_CAUSALITY_EXPLANATION_SCHEMA.to_string(),
            owned_runtime_surfaces: vec![
                "adl::chronosense::TemporalCausalityExplanationContract".to_string(),
                "adl::chronosense::CausalRelationContract".to_string(),
                "adl::chronosense::ExplanationSurfaceContract".to_string(),
                "adl::chronosense::ExplanationFixture".to_string(),
                "adl::chronosense::TemporalQueryRetrievalContract".to_string(),
                "adl::chronosense::CommitmentDeadlineContract".to_string(),
                "adl identity causality".to_string(),
            ],
            causal_relations: CausalRelationContract {
                relation_types: vec![
                    "temporal_succession".to_string(),
                    "declared_dependency".to_string(),
                    "causal_contribution".to_string(),
                    "unknown_relation".to_string(),
                ],
                sequence_boundary_rule:
                    "sequence alone is insufficient evidence for causality".to_string(),
                dependency_evidence_requirements: vec![
                    "cite source event or condition".to_string(),
                    "cite target event or state".to_string(),
                    "name explicit relation type".to_string(),
                    "record bounded confidence or uncertainty".to_string(),
                ],
                uncertainty_classes: vec![
                    "high".to_string(),
                    "medium".to_string(),
                    "low".to_string(),
                    "unknown".to_string(),
                ],
            },
            explanation_surface: ExplanationSurfaceContract {
                required_fields: vec![
                    "source_event_or_condition".to_string(),
                    "target_event_or_state".to_string(),
                    "relation_type".to_string(),
                    "confidence".to_string(),
                    "explanation_note".to_string(),
                ],
                citation_requirements: vec![
                    "cite dependency or state-change evidence".to_string(),
                    "cite prior temporal anchor when available".to_string(),
                    "cite uncertainty explicitly when causal evidence is incomplete".to_string(),
                ],
                non_goals: vec![
                    "probabilistic global causal graphs".to_string(),
                    "scientific causal inference engines".to_string(),
                    "overclaiming causality from ordering alone".to_string(),
                ],
            },
            explanation_fixtures: vec![
                ExplanationFixture {
                    scenario: "deadline_miss_after_interruption".to_string(),
                    relation_type: "causal_contribution".to_string(),
                    confidence: "medium".to_string(),
                    explanation_note:
                        "interruption preserved continuity boundary and contributed to missed commitment visibility"
                            .to_string(),
                },
                ExplanationFixture {
                    scenario: "adjacent_events_without_dependency".to_string(),
                    relation_type: "unknown_relation".to_string(),
                    confidence: "unknown".to_string(),
                    explanation_note:
                        "adjacent temporal order is recorded, but no dependency evidence is present"
                            .to_string(),
                },
            ],
            proof_fixture_hooks: vec![
                "adl::chronosense::TemporalCausalityExplanationContract::v1".to_string(),
                "adl identity causality --out .adl/state/temporal_causality_explanation_v1.json"
                    .to_string(),
            ],
            proof_hook_command:
                "adl identity causality --out .adl/state/temporal_causality_explanation_v1.json"
                    .to_string(),
            proof_hook_output_path: ".adl/state/temporal_causality_explanation_v1.json"
                .to_string(),
            scope_boundary:
                "bounded causal-link and explanation semantics only; full causal inference, planning policy, and global explanation graphs remain downstream work"
                    .to_string(),
        }
    }
}
