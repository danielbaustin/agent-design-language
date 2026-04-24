//! Chronosense phi-integration and metric contracts.
use serde::{Deserialize, Serialize};

use super::PHI_INTEGRATION_METRICS_SCHEMA;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PhiMetricDimension {
    pub name: String,
    pub interpretation: String,
    pub low_signal: String,
    pub medium_signal: String,
    pub high_signal: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PhiComparisonProfile {
    pub profile_name: String,
    pub integration_band: String,
    pub expected_runtime_surfaces: Vec<String>,
    pub why_it_matters: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PhiComparisonFixture {
    pub profile_name: String,
    pub structural_coupling: String,
    pub memory_coupling: String,
    pub feedback_depth: String,
    pub policy_continuity: String,
    pub instinct_coupling: String,
    pub graph_irreducibility: String,
    pub adaptive_depth: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PhiReviewSurfaceContract {
    pub required_questions: Vec<String>,
    pub comparison_rule: String,
    pub non_goals: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PhiIntegrationMetricsContract {
    pub schema_version: String,
    pub owned_runtime_surfaces: Vec<String>,
    pub dimensions: Vec<PhiMetricDimension>,
    pub comparison_profiles: Vec<PhiComparisonProfile>,
    pub comparison_fixtures: Vec<PhiComparisonFixture>,
    pub review_surface: PhiReviewSurfaceContract,
    pub proof_fixture_hooks: Vec<String>,
    pub proof_hook_command: String,
    pub proof_hook_output_path: String,
    pub scope_boundary: String,
}

impl PhiIntegrationMetricsContract {
    pub fn v1() -> Self {
        Self {
            schema_version: PHI_INTEGRATION_METRICS_SCHEMA.to_string(),
            owned_runtime_surfaces: vec![
                "adl::chronosense::PhiIntegrationMetricsContract".to_string(),
                "adl::chronosense::PhiMetricDimension".to_string(),
                "adl::chronosense::PhiComparisonProfile".to_string(),
                "adl::chronosense::PhiComparisonFixture".to_string(),
                "adl::chronosense::TemporalQueryRetrievalContract".to_string(),
                "adl::chronosense::CommitmentDeadlineContract".to_string(),
                "adl::chronosense::ExecutionPolicyCostModelContract".to_string(),
                "adl identity phi".to_string(),
            ],
            dimensions: vec![
                PhiMetricDimension {
                    name: "structural_coupling".to_string(),
                    interpretation:
                        "how many runtime surfaces must remain coordinated for the path to work"
                            .to_string(),
                    low_signal: "mostly isolated steps with minimal cross-surface dependency"
                        .to_string(),
                    medium_signal:
                        "some shared state or review surfaces must remain aligned".to_string(),
                    high_signal:
                        "behavior depends on multiple tightly coupled runtime surfaces".to_string(),
                },
                PhiMetricDimension {
                    name: "memory_coupling".to_string(),
                    interpretation:
                        "how much the path relies on retrieval state or continuity-preserved records"
                            .to_string(),
                    low_signal: "little or no retrieval dependence".to_string(),
                    medium_signal: "retrieval helps but is not the sole determinant".to_string(),
                    high_signal:
                        "retrieval or preserved state materially shapes behavior".to_string(),
                },
                PhiMetricDimension {
                    name: "feedback_depth".to_string(),
                    interpretation:
                        "how much iterative review, adaptation, or reflexive correction is required"
                            .to_string(),
                    low_signal: "single-pass execution with little feedback".to_string(),
                    medium_signal: "bounded refinement or review loops".to_string(),
                    high_signal: "multi-step feedback materially changes the resulting path"
                        .to_string(),
                },
                PhiMetricDimension {
                    name: "policy_continuity".to_string(),
                    interpretation:
                        "how much stable policy and execution posture must persist across the path"
                            .to_string(),
                    low_signal: "policy can vary without changing the main outcome".to_string(),
                    medium_signal: "policy consistency improves comparability and trust"
                        .to_string(),
                    high_signal:
                        "policy continuity is necessary for meaningful comparison or replay"
                            .to_string(),
                },
                PhiMetricDimension {
                    name: "instinct_coupling".to_string(),
                    interpretation:
                        "how much instinct or bounded-priority posture must couple to execution"
                            .to_string(),
                    low_signal: "no instinct-sensitive routing involved".to_string(),
                    medium_signal:
                        "priority posture influences comparison but is still bounded".to_string(),
                    high_signal:
                        "instinct-sensitive posture materially changes execution shape"
                            .to_string(),
                },
                PhiMetricDimension {
                    name: "graph_irreducibility".to_string(),
                    interpretation:
                        "how much explanatory power is lost if the path is split into independent parts"
                            .to_string(),
                    low_signal: "the path stays understandable when decomposed".to_string(),
                    medium_signal:
                        "some explanatory loss appears when the path is decomposed".to_string(),
                    high_signal:
                        "decomposition hides important cross-surface behavior".to_string(),
                },
                PhiMetricDimension {
                    name: "adaptive_depth".to_string(),
                    interpretation:
                        "how much bounded adaptation or runtime re-weighting is present"
                            .to_string(),
                    low_signal: "fixed execution with no meaningful adaptation".to_string(),
                    medium_signal: "bounded adaptation within an explicit review surface"
                        .to_string(),
                    high_signal:
                        "adaptation materially changes path selection or integration profile"
                            .to_string(),
                },
            ],
            comparison_profiles: vec![
                PhiComparisonProfile {
                    profile_name: "low_integration_path".to_string(),
                    integration_band: "low".to_string(),
                    expected_runtime_surfaces: vec![
                        "adl identity foundation".to_string(),
                        "adl identity schema".to_string(),
                    ],
                    why_it_matters:
                        "establishes the baseline for bounded, mostly decomposable execution"
                            .to_string(),
                },
                PhiComparisonProfile {
                    profile_name: "medium_integration_path".to_string(),
                    integration_band: "medium".to_string(),
                    expected_runtime_surfaces: vec![
                        "adl identity retrieval".to_string(),
                        "adl identity commitments".to_string(),
                        "adl identity cost".to_string(),
                    ],
                    why_it_matters:
                        "shows when memory, commitment, and cost surfaces begin to couple"
                            .to_string(),
                },
                PhiComparisonProfile {
                    profile_name: "high_integration_path".to_string(),
                    integration_band: "high".to_string(),
                    expected_runtime_surfaces: vec![
                        "adl identity continuity".to_string(),
                        "adl identity causality".to_string(),
                        "instinct runtime surface".to_string(),
                    ],
                    why_it_matters:
                        "gives reviewers a bounded model for tightly coupled adaptive runtime behavior"
                            .to_string(),
                },
            ],
            comparison_fixtures: vec![
                PhiComparisonFixture {
                    profile_name: "low_integration_path".to_string(),
                    structural_coupling: "low".to_string(),
                    memory_coupling: "low".to_string(),
                    feedback_depth: "low".to_string(),
                    policy_continuity: "low".to_string(),
                    instinct_coupling: "low".to_string(),
                    graph_irreducibility: "low".to_string(),
                    adaptive_depth: "low".to_string(),
                },
                PhiComparisonFixture {
                    profile_name: "medium_integration_path".to_string(),
                    structural_coupling: "medium".to_string(),
                    memory_coupling: "medium".to_string(),
                    feedback_depth: "medium".to_string(),
                    policy_continuity: "medium".to_string(),
                    instinct_coupling: "low".to_string(),
                    graph_irreducibility: "medium".to_string(),
                    adaptive_depth: "medium".to_string(),
                },
                PhiComparisonFixture {
                    profile_name: "high_integration_path".to_string(),
                    structural_coupling: "high".to_string(),
                    memory_coupling: "high".to_string(),
                    feedback_depth: "high".to_string(),
                    policy_continuity: "high".to_string(),
                    instinct_coupling: "medium".to_string(),
                    graph_irreducibility: "high".to_string(),
                    adaptive_depth: "high".to_string(),
                },
            ],
            review_surface: PhiReviewSurfaceContract {
                required_questions: vec![
                    "which integration dimensions changed across low, medium, and high paths"
                        .to_string(),
                    "which runtime surfaces explain those differences".to_string(),
                    "why the comparison matters for ADL execution behavior".to_string(),
                ],
                comparison_rule:
                    "reviewers must be able to compare low, medium, and high integration profiles without collapsing them into a single metaphysical scalar"
                        .to_string(),
                non_goals: vec![
                    "formal IIT phi calculation".to_string(),
                    "consciousness or sentience claims".to_string(),
                    "one-number replacement for cost, time, or routing".to_string(),
                ],
            },
            proof_fixture_hooks: vec![
                "adl::chronosense::PhiIntegrationMetricsContract::v1".to_string(),
                "adl identity phi --out .adl/state/phi_integration_metrics_v1.json"
                    .to_string(),
            ],
            proof_hook_command:
                "adl identity phi --out .adl/state/phi_integration_metrics_v1.json"
                    .to_string(),
            proof_hook_output_path: ".adl/state/phi_integration_metrics_v1.json".to_string(),
            scope_boundary:
                "bounded engineering comparison surface only; no formal IIT, no consciousness claims, and no replacement of cost or temporal review surfaces"
                    .to_string(),
        }
    }
}
