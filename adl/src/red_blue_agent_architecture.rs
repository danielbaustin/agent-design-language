use serde::{Deserialize, Serialize};

pub const RED_BLUE_AGENT_ARCHITECTURE_SCHEMA: &str = "red_blue_agent_architecture.v1";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AdversarialRoleContract {
    pub role: String,
    pub mission: String,
    pub bounded_authority: Vec<String>,
    pub required_outputs: Vec<String>,
    pub review_requirements: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PurpleCoordinationContract {
    pub mission: String,
    pub governance_responsibilities: Vec<String>,
    pub prioritization_inputs: Vec<String>,
    pub prohibited_shortcuts: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AdversarialInteractionModelContract {
    pub stage_order: Vec<String>,
    pub handoff_requirements: Vec<String>,
    pub attribution_rules: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RedBlueArchitectureReviewSurfaceContract {
    pub required_questions: Vec<String>,
    pub required_visibility: Vec<String>,
    pub downstream_boundaries: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RedBlueAgentArchitectureContract {
    pub schema_version: String,
    pub owned_runtime_surfaces: Vec<String>,
    pub runtime_condition: String,
    pub red_role: AdversarialRoleContract,
    pub blue_role: AdversarialRoleContract,
    pub purple_coordination: PurpleCoordinationContract,
    pub interaction_model: AdversarialInteractionModelContract,
    pub review_surface: RedBlueArchitectureReviewSurfaceContract,
    pub proof_fixture_hooks: Vec<String>,
    pub proof_hook_command: String,
    pub proof_hook_output_path: String,
    pub scope_boundary: String,
}

impl RedBlueAgentArchitectureContract {
    pub fn v1() -> Self {
        Self {
            schema_version: RED_BLUE_AGENT_ARCHITECTURE_SCHEMA.to_string(),
            owned_runtime_surfaces: vec![
                "adl::red_blue_agent_architecture::RedBlueAgentArchitectureContract".to_string(),
                "adl::red_blue_agent_architecture::AdversarialRoleContract".to_string(),
                "adl::red_blue_agent_architecture::PurpleCoordinationContract".to_string(),
                "adl::red_blue_agent_architecture::AdversarialInteractionModelContract"
                    .to_string(),
                "adl identity red-blue-architecture".to_string(),
            ],
            runtime_condition:
                "ADL adversarial execution must declare persistent red, blue, and purple role responsibilities explicitly rather than leaving them implicit in traces or prose."
                    .to_string(),
            red_role: AdversarialRoleContract {
                role: "red".to_string(),
                mission:
                    "perform bounded offensive discovery, exploit hypothesis generation, and evidence capture under declared posture and policy"
                        .to_string(),
                bounded_authority: vec![
                    "enumerate bounded target surfaces".to_string(),
                    "generate attributable exploit hypotheses".to_string(),
                    "attempt declared exploit paths only within approved runtime posture"
                        .to_string(),
                ],
                required_outputs: vec![
                    "attack-surface inventory".to_string(),
                    "exploit hypothesis artifact".to_string(),
                    "exploit proof or bounded failure evidence".to_string(),
                ],
                review_requirements: vec![
                    "every exploit attempt must remain traceable".to_string(),
                    "successful findings must carry replay-oriented evidence references"
                        .to_string(),
                ],
            },
            blue_role: AdversarialRoleContract {
                role: "blue".to_string(),
                mission:
                    "perform bounded defensive interpretation, mitigation planning, and replay-oriented validation against red evidence"
                        .to_string(),
                bounded_authority: vec![
                    "ingest exploit evidence and assess actual risk".to_string(),
                    "propose mitigation or hardening actions".to_string(),
                    "validate defensive success against declared replay expectations"
                        .to_string(),
                ],
                required_outputs: vec![
                    "mitigation plan".to_string(),
                    "residual-risk assessment".to_string(),
                    "validation result linked to exploit evidence".to_string(),
                ],
                review_requirements: vec![
                    "defensive claims must cite specific exploit evidence".to_string(),
                    "residual uncertainty must stay explicit".to_string(),
                ],
            },
            purple_coordination: PurpleCoordinationContract {
                mission:
                    "govern prioritization, replay order, and durable learning so red and blue execution remains one bounded architecture rather than disconnected activity"
                        .to_string(),
                governance_responsibilities: vec![
                    "prioritize adversarial findings".to_string(),
                    "govern replay and escalation order".to_string(),
                    "capture durable learning and exploit-family correlation".to_string(),
                ],
                prioritization_inputs: vec![
                    "declared runtime posture".to_string(),
                    "severity and exploitability evidence".to_string(),
                    "residual risk after blue response".to_string(),
                ],
                prohibited_shortcuts: vec![
                    "hidden prioritization criteria".to_string(),
                    "role-free coordination prose without attributable artifacts".to_string(),
                    "promotion of exploit knowledge without review-visible governance".to_string(),
                ],
            },
            interaction_model: AdversarialInteractionModelContract {
                stage_order: vec![
                    "surface enumeration".to_string(),
                    "exploit hypothesis generation".to_string(),
                    "bounded exploit attempt".to_string(),
                    "blue risk evaluation".to_string(),
                    "mitigation or containment decision".to_string(),
                    "replay or explicit defer decision".to_string(),
                    "learning capture".to_string(),
                ],
                handoff_requirements: vec![
                    "red-to-blue transfer must include attributable exploit evidence".to_string(),
                    "blue-to-purple transfer must include mitigation decision and residual uncertainty"
                        .to_string(),
                    "purple governance must preserve declared posture, target, and stage order"
                        .to_string(),
                ],
                attribution_rules: vec![
                    "trace and artifacts must identify whether work was performed by red, blue, or purple"
                        .to_string(),
                    "role attribution must survive replay and review packaging".to_string(),
                ],
            },
            review_surface: RedBlueArchitectureReviewSurfaceContract {
                required_questions: vec![
                    "what persistent roles exist and what is each role allowed to do".to_string(),
                    "how does evidence move from red to blue to purple".to_string(),
                    "which later execution or replay surfaces must preserve this role architecture"
                        .to_string(),
                ],
                required_visibility: vec![
                    "role-specific mission and authority boundaries".to_string(),
                    "handoff requirements and stage order".to_string(),
                    "purple governance responsibilities and prohibited shortcuts".to_string(),
                ],
                downstream_boundaries: vec![
                    "the executable adversarial runner remains WP-04".to_string(),
                    "exploit artifact schema and replay manifest remain WP-05".to_string(),
                    "continuous verification and self-attack patterns remain WP-06".to_string(),
                ],
            },
            proof_fixture_hooks: vec![
                "adl::red_blue_agent_architecture::RedBlueAgentArchitectureContract::v1"
                    .to_string(),
                "adl identity red-blue-architecture --out .adl/state/red_blue_agent_architecture_v1.json".to_string(),
            ],
            proof_hook_command:
                "adl identity red-blue-architecture --out .adl/state/red_blue_agent_architecture_v1.json"
                    .to_string(),
            proof_hook_output_path: ".adl/state/red_blue_agent_architecture_v1.json".to_string(),
            scope_boundary:
                "bounded role architecture only; executable runner behavior, exploit artifact schemas, replay manifests, and continuous self-attack loops remain downstream milestone work."
                    .to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::RedBlueAgentArchitectureContract;

    #[test]
    fn red_blue_agent_architecture_contract_is_stable_and_bounded() {
        let contract = RedBlueAgentArchitectureContract::v1();

        assert_eq!(contract.schema_version, "red_blue_agent_architecture.v1");
        assert!(contract
            .owned_runtime_surfaces
            .iter()
            .any(|surface| surface == "adl identity red-blue-architecture"));
        assert!(contract
            .red_role
            .required_outputs
            .iter()
            .any(|item| item == "exploit hypothesis artifact"));
        assert!(contract
            .blue_role
            .review_requirements
            .iter()
            .any(|item| item.contains("residual uncertainty")));
        assert!(contract
            .purple_coordination
            .governance_responsibilities
            .iter()
            .any(|item| item == "govern replay and escalation order"));
        assert_eq!(
            contract
                .interaction_model
                .stage_order
                .first()
                .map(String::as_str),
            Some("surface enumeration")
        );
        assert!(contract
            .review_surface
            .downstream_boundaries
            .iter()
            .any(|item| item.contains("WP-05")));
        assert!(contract
            .scope_boundary
            .contains("downstream milestone work"));
    }
}
