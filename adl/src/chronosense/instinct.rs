use serde::{Deserialize, Serialize};

use super::{INSTINCT_MODEL_SCHEMA, INSTINCT_RUNTIME_SURFACE_SCHEMA};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InstinctEntryContract {
    pub instinct_id: String,
    pub meaning: String,
    pub default_strength: String,
    pub allowed_influences: Vec<String>,
    pub subordinate_to: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InstinctSemanticsContract {
    pub instinct_definition: String,
    pub distinctions_from_goals: Vec<String>,
    pub distinctions_from_affect: Vec<String>,
    pub boundedness_rules: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InstinctRepresentationContract {
    pub required_fields: Vec<String>,
    pub optional_fields: Vec<String>,
    pub persistence_expectations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InstinctReviewSurfaceContract {
    pub required_questions: Vec<String>,
    pub required_visibility: Vec<String>,
    pub non_goals: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InstinctModelContract {
    pub schema_version: String,
    pub owned_runtime_surfaces: Vec<String>,
    pub instinct_set: Vec<InstinctEntryContract>,
    pub semantics: InstinctSemanticsContract,
    pub representation: InstinctRepresentationContract,
    pub review_surface: InstinctReviewSurfaceContract,
    pub proof_fixture_hooks: Vec<String>,
    pub proof_hook_command: String,
    pub proof_hook_output_path: String,
    pub scope_boundary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InstinctRuntimeProofCase {
    pub case_id: String,
    pub selected_path: String,
    pub dominant_instinct: String,
    pub risk_class: String,
    pub expected_candidate_id: String,
    pub expected_candidate_kind: String,
    pub expected_effect: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InstinctRuntimeReviewSurfaceContract {
    pub visible_fields: Vec<String>,
    pub required_questions: Vec<String>,
    pub policy_override_rule: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InstinctRuntimeSurfaceContract {
    pub schema_version: String,
    pub owned_runtime_surfaces: Vec<String>,
    pub shared_selection_rule: String,
    pub bounded_influence_rules: Vec<String>,
    pub proof_cases: Vec<InstinctRuntimeProofCase>,
    pub review_surface: InstinctRuntimeReviewSurfaceContract,
    pub proof_hook_command: String,
    pub proof_hook_output_path: String,
    pub scope_boundary: String,
}

impl InstinctModelContract {
    pub fn v1() -> Self {
        Self {
            schema_version: INSTINCT_MODEL_SCHEMA.to_string(),
            owned_runtime_surfaces: vec![
                "adl::chronosense::InstinctModelContract".to_string(),
                "adl::chronosense::InstinctEntryContract".to_string(),
                "adl::chronosense::InstinctSemanticsContract".to_string(),
                "adl::chronosense::InstinctRepresentationContract".to_string(),
                "adl identity instinct".to_string(),
            ],
            instinct_set: vec![
                InstinctEntryContract {
                    instinct_id: "integrity".to_string(),
                    meaning: "bias toward policy-respecting, constraint-honoring action under uncertainty".to_string(),
                    default_strength: "medium".to_string(),
                    allowed_influences: vec![
                        "favor constrained route selection".to_string(),
                        "increase anomaly attention when invariants look threatened".to_string(),
                    ],
                    subordinate_to: vec![
                        "policy".to_string(),
                        "safety".to_string(),
                        "explicit task goals".to_string(),
                    ],
                },
                InstinctEntryContract {
                    instinct_id: "curiosity".to_string(),
                    meaning: "bias toward bounded anomaly follow-up and unresolved-question attention".to_string(),
                    default_strength: "medium".to_string(),
                    allowed_influences: vec![
                        "increase follow-up priority for unexplained signals".to_string(),
                        "favor deeper inspection when competing options are otherwise close".to_string(),
                    ],
                    subordinate_to: vec![
                        "policy".to_string(),
                        "safety".to_string(),
                        "budgeted execution limits".to_string(),
                    ],
                },
                InstinctEntryContract {
                    instinct_id: "coherence".to_string(),
                    meaning: "bias toward internally consistent plans, traces, and explanations".to_string(),
                    default_strength: "high".to_string(),
                    allowed_influences: vec![
                        "favor plans with clearer explanatory alignment".to_string(),
                        "increase pressure to resolve contradictions before closing work".to_string(),
                    ],
                    subordinate_to: vec![
                        "policy".to_string(),
                        "safety".to_string(),
                        "explicit task goals".to_string(),
                    ],
                },
                InstinctEntryContract {
                    instinct_id: "completion".to_string(),
                    meaning: "bias toward finishing started bounded work rather than abandoning it casually".to_string(),
                    default_strength: "medium".to_string(),
                    allowed_influences: vec![
                        "favor finishing current bounded work when policy permits".to_string(),
                        "increase follow-through pressure on accepted obligations".to_string(),
                    ],
                    subordinate_to: vec![
                        "policy".to_string(),
                        "safety".to_string(),
                        "higher-priority explicit goals".to_string(),
                    ],
                },
            ],
            semantics: InstinctSemanticsContract {
                instinct_definition: "persistent background pressure that shapes prioritization, routing, and follow-through without replacing goals, affect, policy, or review".to_string(),
                distinctions_from_goals: vec![
                    "goals are explicit and task-specific".to_string(),
                    "instinct remains active across tasks as directional bias".to_string(),
                ],
                distinctions_from_affect: vec![
                    "affect is dynamic evaluation of current state".to_string(),
                    "instinct is the lower-latency persistent leaning beneath that evaluation".to_string(),
                ],
                boundedness_rules: vec![
                    "instinct must stay explicit and inspectable".to_string(),
                    "instinct may influence prioritization or routing but may not bypass policy".to_string(),
                    "instinct must not introduce hidden non-determinism".to_string(),
                ],
            },
            representation: InstinctRepresentationContract {
                required_fields: vec![
                    "instinct_id".to_string(),
                    "default_strength".to_string(),
                    "meaning".to_string(),
                ],
                optional_fields: vec![
                    "enabled".to_string(),
                    "allowed_influences".to_string(),
                    "subordinate_to".to_string(),
                ],
                persistence_expectations: vec![
                    "remain available across tasks".to_string(),
                    "stay explicit in execution context attachment".to_string(),
                    "support later runtime wiring without changing the semantic core".to_string(),
                ],
            },
            review_surface: InstinctReviewSurfaceContract {
                required_questions: vec![
                    "which instincts were declared".to_string(),
                    "how does instinct differ from goals and affect".to_string(),
                    "what higher-level constraints keep instinct bounded".to_string(),
                ],
                required_visibility: vec![
                    "declared instinct set".to_string(),
                    "meaning of each instinct".to_string(),
                    "subordination to policy and safety".to_string(),
                ],
                non_goals: vec![
                    "full psychology model".to_string(),
                    "identity formation through instinct alone".to_string(),
                    "hidden autonomy justification".to_string(),
                ],
            },
            proof_fixture_hooks: vec![
                "adl::chronosense::InstinctModelContract::v1".to_string(),
                "adl identity instinct --out .adl/state/instinct_model_v1.json".to_string(),
            ],
            proof_hook_command:
                "adl identity instinct --out .adl/state/instinct_model_v1.json".to_string(),
            proof_hook_output_path: ".adl/state/instinct_model_v1.json".to_string(),
            scope_boundary:
                "bounded instinct substrate only; runtime influence, prioritization hooks, and bounded agency proof remain downstream work in WP-11".to_string(),
        }
    }
}

impl InstinctRuntimeSurfaceContract {
    pub fn v1() -> Self {
        Self {
            schema_version: INSTINCT_RUNTIME_SURFACE_SCHEMA.to_string(),
            owned_runtime_surfaces: vec![
                "adl::execute::select_instinct_runtime_candidate".to_string(),
                "adl::execute::AgencySelectionState".to_string(),
                "adl::execute::RuntimeControlState".to_string(),
                "adl identity instinct-runtime".to_string(),
            ],
            shared_selection_rule:
                "derive the selected bounded agency candidate from the already-selected fast/slow path, dominant instinct, and risk class using one deterministic shared rule"
                    .to_string(),
            bounded_influence_rules: vec![
                "instinct may change candidate selection but may not bypass fast/slow routing".to_string(),
                "high-risk slow-path decisions stay review-first regardless of completion pressure".to_string(),
                "all instinct influence must remain visible through selected candidate id, kind, and reason".to_string(),
            ],
            proof_cases: vec![
                InstinctRuntimeProofCase {
                    case_id: "fast-curiosity-verification".to_string(),
                    selected_path: "fast_path".to_string(),
                    dominant_instinct: "curiosity".to_string(),
                    risk_class: "medium".to_string(),
                    expected_candidate_id: "cand-fast-verify".to_string(),
                    expected_candidate_kind: "bounded_verification".to_string(),
                    expected_effect: "curiosity upgrades fast-path direct execution to a single bounded verification pass".to_string(),
                },
                InstinctRuntimeProofCase {
                    case_id: "slow-curiosity-defer".to_string(),
                    selected_path: "slow_path".to_string(),
                    dominant_instinct: "curiosity".to_string(),
                    risk_class: "medium".to_string(),
                    expected_candidate_id: "cand-slow-defer".to_string(),
                    expected_candidate_kind: "bounded_deferral".to_string(),
                    expected_effect: "curiosity can choose a bounded uncertainty-hold candidate instead of immediate execution or refinement".to_string(),
                },
                InstinctRuntimeProofCase {
                    case_id: "slow-high-risk-review".to_string(),
                    selected_path: "slow_path".to_string(),
                    dominant_instinct: "curiosity".to_string(),
                    risk_class: "high".to_string(),
                    expected_candidate_id: "cand-slow-review".to_string(),
                    expected_candidate_kind: "review_and_refine".to_string(),
                    expected_effect: "policy-constrained high-risk review overrides curiosity's bounded deferral pressure".to_string(),
                },
            ],
            review_surface: InstinctRuntimeReviewSurfaceContract {
                visible_fields: vec![
                    "dominant_instinct".to_string(),
                    "selected_path".to_string(),
                    "risk_class".to_string(),
                    "selected_candidate_id".to_string(),
                    "selected_candidate_reason".to_string(),
                ],
                required_questions: vec![
                    "did instinct change the candidate or leave it unchanged".to_string(),
                    "was the change still bounded by path and risk policy".to_string(),
                    "can the selected candidate be replay-explained from visible fields".to_string(),
                ],
                policy_override_rule:
                    "high-risk slow-path review remains mandatory even when curiosity would otherwise choose bounded deferral"
                        .to_string(),
            },
            proof_hook_command:
                "adl identity instinct-runtime --out .adl/state/instinct_runtime_surface_v1.json"
                    .to_string(),
            proof_hook_output_path: ".adl/state/instinct_runtime_surface_v1.json".to_string(),
            scope_boundary:
                "bounded instinct runtime hook only; this does not introduce open-ended autonomy, hidden initiative, or governance-layer override logic"
                    .to_string(),
        }
    }
}
