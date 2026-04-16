use serde::{Deserialize, Serialize};

use crate::delegation_policy::{DelegationDecision, DelegationPolicyOutcome};
use crate::operational_skills_substrate::OPERATIONAL_SKILLS_SUBSTRATE_SCHEMA;
use crate::red_blue_agent_architecture::RED_BLUE_AGENT_ARCHITECTURE_SCHEMA;
use crate::skill_composition_model::SKILL_COMPOSITION_MODEL_SCHEMA;

pub const DELEGATION_REFUSAL_COORDINATION_SCHEMA: &str = "delegation_refusal_coordination.v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GovernedOutcomeKind {
    BoundedDelegationAllowed,
    GovernedRefusal,
    ApprovalGate,
}

impl GovernedOutcomeKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            GovernedOutcomeKind::BoundedDelegationAllowed => "bounded_delegation_allowed",
            GovernedOutcomeKind::GovernedRefusal => "governed_refusal",
            GovernedOutcomeKind::ApprovalGate => "approval_gate",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GovernedPolicyDecision {
    pub outcome_kind: String,
    pub trace_decision: String,
    pub matched_rule_id: Option<String>,
    pub review_obligation: String,
}

pub fn governed_policy_decision(outcome: &DelegationPolicyOutcome) -> GovernedPolicyDecision {
    let outcome_kind = match outcome.decision {
        DelegationDecision::Allowed => GovernedOutcomeKind::BoundedDelegationAllowed,
        DelegationDecision::Denied => GovernedOutcomeKind::GovernedRefusal,
        DelegationDecision::NeedsApproval => GovernedOutcomeKind::ApprovalGate,
    };
    let review_obligation = match outcome_kind {
        GovernedOutcomeKind::BoundedDelegationAllowed => {
            "record the delegated action, target, and preserved constraints"
        }
        GovernedOutcomeKind::GovernedRefusal => {
            "record the refused action, target, matched rule, and why this is not generic failure"
        }
        GovernedOutcomeKind::ApprovalGate => {
            "stop automatic execution and surface the approval authority before proceeding"
        }
    };

    GovernedPolicyDecision {
        outcome_kind: outcome_kind.as_str().to_string(),
        trace_decision: outcome.decision.as_str().to_string(),
        matched_rule_id: outcome.rule_id.clone(),
        review_obligation: review_obligation.to_string(),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GovernanceOutcomeContract {
    pub outcome_kind: String,
    pub meaning: String,
    pub required_trace_events: Vec<String>,
    pub distinct_from: Vec<String>,
    pub review_questions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DelegationRefusalBoundaryContract {
    pub required_distinctions: Vec<String>,
    pub refusal_rules: Vec<String>,
    pub delegation_rules: Vec<String>,
    pub reroute_rules: Vec<String>,
    pub failure_separation_rules: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CoordinationNegotiationContract {
    pub participant_requirements: Vec<String>,
    pub position_requirements: Vec<String>,
    pub allowed_outcomes: Vec<String>,
    pub disagreement_visibility_rules: Vec<String>,
    pub prohibited_shortcuts: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OperationalSkillGovernanceContract {
    pub required_skill_surfaces: Vec<String>,
    pub admission_rules: Vec<String>,
    pub handoff_rules: Vec<String>,
    pub stop_boundaries: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DelegationCoordinationReviewSurfaceContract {
    pub required_questions: Vec<String>,
    pub reviewer_visible_artifacts: Vec<String>,
    pub downstream_boundaries: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DelegationRefusalCoordinationContract {
    pub schema_version: String,
    pub owned_runtime_surfaces: Vec<String>,
    pub runtime_condition: String,
    pub upstream_contracts: Vec<String>,
    pub outcome_taxonomy: Vec<GovernanceOutcomeContract>,
    pub delegation_refusal_boundary: DelegationRefusalBoundaryContract,
    pub coordination_negotiation: CoordinationNegotiationContract,
    pub operational_skill_governance: OperationalSkillGovernanceContract,
    pub review_surface: DelegationCoordinationReviewSurfaceContract,
    pub proof_fixture_hooks: Vec<String>,
    pub proof_hook_command: String,
    pub proof_hook_output_path: String,
    pub scope_boundary: String,
}

impl DelegationRefusalCoordinationContract {
    pub fn v1() -> Self {
        Self {
            schema_version: DELEGATION_REFUSAL_COORDINATION_SCHEMA.to_string(),
            owned_runtime_surfaces: strings(&[
                "adl::delegation_refusal_coordination::DelegationRefusalCoordinationContract",
                "adl::delegation_refusal_coordination::GovernedPolicyDecision",
                "adl::delegation_refusal_coordination::GovernedOutcomeKind",
                "adl::delegation_policy::DelegationPolicyOutcome",
                "adl identity delegation-refusal-coordination",
            ]),
            runtime_condition:
                "ADL governance handoffs must distinguish bounded delegation, governed refusal, approval gating, and structured coordination instead of hiding them as generic failure, success, or model wording."
                    .to_string(),
            upstream_contracts: strings(&[
                RED_BLUE_AGENT_ARCHITECTURE_SCHEMA,
                OPERATIONAL_SKILLS_SUBSTRATE_SCHEMA,
                SKILL_COMPOSITION_MODEL_SCHEMA,
            ]),
            outcome_taxonomy: vec![
                outcome(
                    "bounded_delegation_allowed",
                    "the action is allowed under policy and may be handled by the declared actor, provider, office, or skill while preserving constraints",
                    &[
                        "DelegationRequested",
                        "DelegationPolicyEvaluated",
                        "DelegationDispatched",
                    ],
                    &["silent reroute", "generic success", "unbounded handoff"],
                    &[
                        "what was delegated",
                        "to whom or to what surface",
                        "which constraints remained in force",
                    ],
                ),
                outcome(
                    "governed_refusal",
                    "the system recognized the action and intentionally stopped it under declared constraints",
                    &["DelegationPolicyEvaluated", "DelegationDenied"],
                    &["capability failure", "tool failure", "silence", "ambiguous avoidance"],
                    &[
                        "what was refused",
                        "which policy or boundary caused the refusal",
                        "why this is not a generic execution failure",
                    ],
                ),
                outcome(
                    "approval_gate",
                    "the action may be valid but cannot proceed automatically without an explicit approval authority",
                    &["DelegationPolicyEvaluated"],
                    &["automatic allow", "automatic deny", "hidden human override"],
                    &[
                        "who must approve",
                        "what would be approved",
                        "what remains blocked until approval",
                    ],
                ),
                outcome(
                    "structured_coordination",
                    "multiple roles or skills state bounded positions before an outcome is selected, deferred, refused, or delegated",
                    &[
                        "coordination_session_started",
                        "coordination_position_recorded",
                        "coordination_outcome_recorded",
                    ],
                    &["opaque chat", "silent dominance", "unattributed consensus"],
                    &[
                        "who participated",
                        "what positions were recorded",
                        "how the outcome binds back to a decision record",
                    ],
                ),
                outcome(
                    "bounded_dissent",
                    "coordination ended with visible unresolved disagreement and an explicit defer, escalation, or refusal path",
                    &["coordination_position_recorded", "coordination_dissent_recorded"],
                    &["erased disagreement", "false consensus", "silent escalation"],
                    &[
                        "what dissent remained",
                        "why it was not resolved",
                        "what downstream unblock condition exists",
                    ],
                ),
            ],
            delegation_refusal_boundary: DelegationRefusalBoundaryContract {
                required_distinctions: strings(&[
                    "cannot_do",
                    "should_not_do",
                    "should_not_do_here",
                    "can_do_later_under_better_conditions",
                ]),
                refusal_rules: strings(&[
                    "refusal must identify the recognized action",
                    "refusal must cite a declared constraint, policy rule, posture, or approval boundary",
                    "refusal must be trace-visible and reviewer-legible",
                    "refusal must not be recorded as generic tool failure",
                ]),
                delegation_rules: strings(&[
                    "delegation must preserve the original constraint context",
                    "delegation must name the actor, provider, office, or skill that receives the handoff",
                    "delegation must record whether verification or approval is still required",
                    "delegation must not masquerade as final success before the delegated result is received",
                ]),
                reroute_rules: strings(&[
                    "rerouting is a delegation outcome when another surface should handle the action",
                    "rerouting must carry the same posture and evidence constraints forward",
                    "rerouting cannot erase the original responsibility boundary",
                ]),
                failure_separation_rules: strings(&[
                    "capability failure means the action could not be performed",
                    "governed refusal means the action was intentionally stopped",
                    "approval gate means execution is paused pending authority",
                    "delegation means execution responsibility moved under preserved constraints",
                ]),
            },
            coordination_negotiation: CoordinationNegotiationContract {
                participant_requirements: strings(&[
                    "participant_id",
                    "role_or_skill_id",
                    "standing_or_scope",
                    "authority_boundary",
                ]),
                position_requirements: strings(&[
                    "position_id",
                    "participant_id",
                    "claim_or_recommendation",
                    "evidence_refs",
                    "constraints_cited",
                ]),
                allowed_outcomes: strings(&[
                    "consensus",
                    "bounded_dissent",
                    "escalation",
                    "governed_refusal",
                    "delegated_resolution",
                    "explicit_defer",
                ]),
                disagreement_visibility_rules: strings(&[
                    "dissent remains visible even when an outcome is selected",
                    "unresolved conflicts include unblock conditions",
                    "position records cite participants rather than anonymous group sentiment",
                    "outcomes bind back to explicit decision records",
                ]),
                prohibited_shortcuts: strings(&[
                    "unstructured chat as the only coordination artifact",
                    "silent dominance by one role",
                    "consensus without participant positions",
                    "delegation without preserved constraints",
                    "refusal without a recognized action",
                ]),
            },
            operational_skill_governance: OperationalSkillGovernanceContract {
                required_skill_surfaces: strings(&[
                    "skill_id",
                    "entry_conditions",
                    "admission_decision",
                    "allowed_tools_or_capabilities",
                    "handoff_state",
                    "stop_boundary",
                ]),
                admission_rules: strings(&[
                    "candidate skills are selected by explicit context rather than description text alone",
                    "admission records missing inputs before execution",
                    "more specific skills win over generic orchestration skills",
                    "judgment skills default to findings or handoff when authority is unclear",
                ]),
                handoff_rules: strings(&[
                    "handoff names the selected downstream skill or authority",
                    "handoff records whether it is auto-apply, findings-only, propose-and-stop, or blocked",
                    "handoff preserves issue, branch, worktree, and artifact context where available",
                ]),
                stop_boundaries: strings(&[
                    "do not continue after approval_gate without explicit approval",
                    "do not continue after governed_refusal except to record the refusal",
                    "do not continue from bounded_dissent unless a declared escalation or defer path exists",
                    "do not treat delegated_resolution as complete until result or defer evidence exists",
                ]),
            },
            review_surface: DelegationCoordinationReviewSurfaceContract {
                required_questions: strings(&[
                    "what was delegated, refused, approved, deferred, or coordinated",
                    "which policy, posture, role, or skill boundary controlled the outcome",
                    "what evidence and constraints moved across any handoff",
                    "what disagreement or uncertainty remains visible",
                    "which downstream work package owns anything still not implemented",
                ]),
                reviewer_visible_artifacts: strings(&[
                    "delegation/refusal/coordination contract artifact",
                    "policy outcome mapping",
                    "trace event obligations",
                    "coordination position/outcome packet when negotiation is used",
                    "D6 integration note in the demo matrix",
                ]),
                downstream_boundaries: strings(&[
                    "WP-10 owns provider-extension and milestone-packaging convergence",
                    "WP-11 through WP-13 own demo entry points and integration proof packets",
                    "later identity, moral-governance, and constitutional bands own broader society mechanics",
                    "this contract does not implement final negotiation law or social reputation interpretation",
                ]),
            },
            proof_fixture_hooks: strings(&[
                "adl::delegation_refusal_coordination::DelegationRefusalCoordinationContract::v1",
                "adl::delegation_refusal_coordination::governed_policy_decision",
                "adl identity delegation-refusal-coordination --out .adl/state/delegation_refusal_coordination_v1.json",
            ]),
            proof_hook_command:
                "adl identity delegation-refusal-coordination --out .adl/state/delegation_refusal_coordination_v1.json"
                    .to_string(),
            proof_hook_output_path: ".adl/state/delegation_refusal_coordination_v1.json".to_string(),
            scope_boundary:
                "WP-09 integrates delegation, refusal, and coordination governance as a bounded review contract; it does not implement final constitutional negotiation, social reputation, provider-security extension, or fully automated approval authority."
                    .to_string(),
        }
    }
}

fn outcome(
    outcome_kind: &str,
    meaning: &str,
    required_trace_events: &[&str],
    distinct_from: &[&str],
    review_questions: &[&str],
) -> GovernanceOutcomeContract {
    GovernanceOutcomeContract {
        outcome_kind: outcome_kind.to_string(),
        meaning: meaning.to_string(),
        required_trace_events: strings(required_trace_events),
        distinct_from: strings(distinct_from),
        review_questions: strings(review_questions),
    }
}

fn strings(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_string()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn delegation_refusal_coordination_contract_is_bounded() {
        let contract = DelegationRefusalCoordinationContract::v1();

        assert_eq!(
            contract.schema_version,
            DELEGATION_REFUSAL_COORDINATION_SCHEMA
        );
        assert_eq!(
            contract.upstream_contracts,
            vec![
                RED_BLUE_AGENT_ARCHITECTURE_SCHEMA.to_string(),
                OPERATIONAL_SKILLS_SUBSTRATE_SCHEMA.to_string(),
                SKILL_COMPOSITION_MODEL_SCHEMA.to_string()
            ]
        );
        assert!(contract
            .owned_runtime_surfaces
            .contains(&"adl identity delegation-refusal-coordination".to_string()));
        assert!(contract
            .scope_boundary
            .contains("does not implement final constitutional negotiation"));
    }

    #[test]
    fn outcome_taxonomy_distinguishes_refusal_from_failure_and_reroute() {
        let contract = DelegationRefusalCoordinationContract::v1();
        let refusal = contract
            .outcome_taxonomy
            .iter()
            .find(|outcome| outcome.outcome_kind == "governed_refusal")
            .expect("refusal outcome");

        assert!(refusal.distinct_from.contains(&"tool failure".to_string()));
        assert!(refusal
            .required_trace_events
            .contains(&"DelegationDenied".to_string()));
        assert!(contract
            .delegation_refusal_boundary
            .required_distinctions
            .contains(&"should_not_do_here".to_string()));
        assert!(contract
            .delegation_refusal_boundary
            .failure_separation_rules
            .iter()
            .any(|rule| rule.contains("governed refusal means")));
    }

    #[test]
    fn coordination_requires_visible_positions_and_dissent() {
        let contract = DelegationRefusalCoordinationContract::v1();

        assert!(contract
            .coordination_negotiation
            .position_requirements
            .contains(&"participant_id".to_string()));
        assert!(contract
            .coordination_negotiation
            .allowed_outcomes
            .contains(&"bounded_dissent".to_string()));
        assert!(contract
            .coordination_negotiation
            .prohibited_shortcuts
            .contains(&"silent dominance by one role".to_string()));
    }

    #[test]
    fn policy_outcome_mapping_is_reviewable() {
        let denied = DelegationPolicyOutcome {
            decision: DelegationDecision::Denied,
            rule_id: Some("deny-remote".to_string()),
        };
        let mapped = governed_policy_decision(&denied);
        assert_eq!(mapped.outcome_kind, "governed_refusal");
        assert_eq!(mapped.trace_decision, "denied");
        assert_eq!(mapped.matched_rule_id.as_deref(), Some("deny-remote"));
        assert!(mapped.review_obligation.contains("not generic failure"));

        let approval = governed_policy_decision(&DelegationPolicyOutcome {
            decision: DelegationDecision::NeedsApproval,
            rule_id: None,
        });
        assert_eq!(approval.outcome_kind, "approval_gate");

        let allowed = governed_policy_decision(&DelegationPolicyOutcome {
            decision: DelegationDecision::Allowed,
            rule_id: None,
        });
        assert_eq!(allowed.outcome_kind, "bounded_delegation_allowed");
    }

    #[test]
    fn contract_has_identity_proof_hook() {
        let contract = DelegationRefusalCoordinationContract::v1();

        assert_eq!(
            contract.proof_hook_command,
            "adl identity delegation-refusal-coordination --out .adl/state/delegation_refusal_coordination_v1.json"
        );
        assert_eq!(
            contract.proof_hook_output_path,
            ".adl/state/delegation_refusal_coordination_v1.json"
        );
        assert!(contract
            .review_surface
            .reviewer_visible_artifacts
            .iter()
            .any(|artifact| artifact.contains("D6 integration note")));
    }
}
