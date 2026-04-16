use serde::{Deserialize, Serialize};

pub const ADVERSARIAL_RUNTIME_MODEL_SCHEMA: &str = "adversarial_runtime_model.v1";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AdversarialPressureContract {
    pub core_claim: String,
    pub operating_assumptions: Vec<String>,
    pub displaced_legacy_assumptions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DynamicAttackSurfaceContract {
    pub model: String,
    pub graph_dimensions: Vec<String>,
    pub traversal_expectations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AdversarialRuntimeGuaranteeContract {
    pub guaranteed_properties: Vec<String>,
    pub evidence_requirements: Vec<String>,
    pub prohibited_shortcuts: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AdversarialRuntimeReviewSurfaceContract {
    pub required_questions: Vec<String>,
    pub required_visibility: Vec<String>,
    pub downstream_boundaries: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AdversarialRuntimeModelContract {
    pub schema_version: String,
    pub owned_runtime_surfaces: Vec<String>,
    pub runtime_condition: String,
    pub adversarial_pressure: AdversarialPressureContract,
    pub dynamic_attack_surface: DynamicAttackSurfaceContract,
    pub runtime_guarantees: AdversarialRuntimeGuaranteeContract,
    pub review_surface: AdversarialRuntimeReviewSurfaceContract,
    pub proof_fixture_hooks: Vec<String>,
    pub proof_hook_command: String,
    pub proof_hook_output_path: String,
    pub scope_boundary: String,
}

impl AdversarialRuntimeModelContract {
    pub fn v1() -> Self {
        Self {
            schema_version: ADVERSARIAL_RUNTIME_MODEL_SCHEMA.to_string(),
            owned_runtime_surfaces: vec![
                "adl::adversarial_runtime::AdversarialRuntimeModelContract".to_string(),
                "adl::adversarial_runtime::AdversarialPressureContract".to_string(),
                "adl::adversarial_runtime::DynamicAttackSurfaceContract".to_string(),
                "adl::adversarial_runtime::AdversarialRuntimeGuaranteeContract".to_string(),
                "adl identity adversarial-runtime".to_string(),
            ],
            runtime_condition:
                "ADL runtimes must assume continuous intelligent opposition as a first-class operating condition."
                    .to_string(),
            adversarial_pressure: AdversarialPressureContract {
                core_claim:
                    "meaningful weaknesses should be treated as eventually discoverable under sustained automated reasoning pressure"
                        .to_string(),
                operating_assumptions: vec![
                    "systems are probed continuously rather than only during scheduled review windows"
                        .to_string(),
                    "attack discovery is scalable, partially automated, and persistence-friendly"
                        .to_string(),
                    "valuable targets should be expected to operate under contest rather than in a safe hidden state"
                        .to_string(),
                ],
                displaced_legacy_assumptions: vec![
                    "obscurity is sufficient protection".to_string(),
                    "manual pentesting cadence is the main detection path".to_string(),
                    "security failure is exceptional rather than an always-possible runtime condition"
                        .to_string(),
                ],
            },
            dynamic_attack_surface: DynamicAttackSurfaceContract {
                model: "the attack surface is a dynamic graph over interfaces, state transitions, temporal conditions, and policy constraints rather than a static checklist".to_string(),
                graph_dimensions: vec![
                    "current system state".to_string(),
                    "available actions and interfaces".to_string(),
                    "temporal conditions and recurrence".to_string(),
                    "policy and posture constraints".to_string(),
                ],
                traversal_expectations: vec![
                    "adversarial exploration must remain attributable and review-visible".to_string(),
                    "graph traversal pressure should be bounded by declared posture and policy".to_string(),
                    "changes in target state or posture should be treated as first-class review context".to_string(),
                ],
            },
            runtime_guarantees: AdversarialRuntimeGuaranteeContract {
                guaranteed_properties: vec![
                    "adversarial activity must be traceable".to_string(),
                    "adversarial outcomes must be attributable to declared configuration and role context".to_string(),
                    "security-relevant actions must remain policy-bounded".to_string(),
                    "replay and review surfaces must be explicit rather than implied".to_string(),
                ],
                evidence_requirements: vec![
                    "posture and target scope must be reviewer-visible".to_string(),
                    "contested execution must produce legible trace or artifact references".to_string(),
                    "security claims must state whether replay is strict, bounded, or deferred downstream"
                        .to_string(),
                ],
                prohibited_shortcuts: vec![
                    "unobserved adversarial execution".to_string(),
                    "non-attributable mitigation claims".to_string(),
                    "hidden escalation from review surface into exploit automation".to_string(),
                ],
            },
            review_surface: AdversarialRuntimeReviewSurfaceContract {
                required_questions: vec![
                    "what contested-runtime assumption is ADL making".to_string(),
                    "how is the attack surface modeled at a conceptual runtime level".to_string(),
                    "which guarantees must any later adversarial runner or replay surface preserve"
                        .to_string(),
                ],
                required_visibility: vec![
                    "continuous adversarial pressure assumption".to_string(),
                    "dynamic attack-surface graph model".to_string(),
                    "boundedness and evidence requirements".to_string(),
                ],
                downstream_boundaries: vec![
                    "persistent red/blue/purple role architecture remains WP-03".to_string(),
                    "the adversarial execution runner remains WP-04".to_string(),
                    "exploit artifacts and replay manifests remain WP-05".to_string(),
                    "continuous verification and self-attack loops remain WP-06".to_string(),
                ],
            },
            proof_fixture_hooks: vec![
                "adl::adversarial_runtime::AdversarialRuntimeModelContract::v1".to_string(),
                "adl identity adversarial-runtime --out .adl/state/adversarial_runtime_model_v1.json".to_string(),
            ],
            proof_hook_command:
                "adl identity adversarial-runtime --out .adl/state/adversarial_runtime_model_v1.json"
                    .to_string(),
            proof_hook_output_path: ".adl/state/adversarial_runtime_model_v1.json".to_string(),
            scope_boundary:
                "bounded contested-runtime contract only; persistent role architecture, executable runner behavior, exploit artifact schemas, and replay loops remain downstream milestone work."
                    .to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::AdversarialRuntimeModelContract;

    #[test]
    fn adversarial_runtime_model_contract_is_stable_and_bounded() {
        let contract = AdversarialRuntimeModelContract::v1();

        assert_eq!(contract.schema_version, "adversarial_runtime_model.v1");
        assert!(contract
            .owned_runtime_surfaces
            .iter()
            .any(|surface| surface == "adl identity adversarial-runtime"));
        assert!(contract
            .adversarial_pressure
            .operating_assumptions
            .iter()
            .any(|item| item.contains("probed continuously")));
        assert!(contract
            .dynamic_attack_surface
            .graph_dimensions
            .iter()
            .any(|item| item == "policy and posture constraints"));
        assert!(contract
            .runtime_guarantees
            .guaranteed_properties
            .iter()
            .any(|item| item == "adversarial activity must be traceable"));
        assert!(contract
            .review_surface
            .downstream_boundaries
            .iter()
            .any(|item| item.contains("WP-04")));
        assert!(contract
            .scope_boundary
            .contains("downstream milestone work"));
    }
}
