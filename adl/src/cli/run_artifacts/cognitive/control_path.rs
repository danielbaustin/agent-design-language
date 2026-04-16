use std::collections::BTreeMap;

use super::super::*;

fn route_outcome_class(route_selected: &str) -> &'static str {
    if route_selected == "fast" {
        "accept"
    } else {
        "reroute"
    }
}

fn reframing_outcome_class(reframing_trigger: &str) -> &'static str {
    if reframing_trigger == "triggered" {
        "reroute"
    } else {
        "accept"
    }
}

fn gate_outcome_class(gate_decision: &str) -> &'static str {
    match gate_decision {
        "allow" => "accept",
        "refuse" => "reject",
        "defer" => "defer",
        "escalate" => "escalate",
        _ => "reject",
    }
}

fn proposal_kind_for_candidate_kind(candidate_kind: &str) -> &'static str {
    match candidate_kind {
        "bounded_deferral" => "defer",
        "refusal" => "refuse",
        "memory_read" => "memory_read",
        "memory_write" => "memory_write",
        "final_answer" => "final_answer",
        "tool_call" => "tool_call",
        _ => "skill_call",
    }
}

fn proposal_target_for_candidate_kind(candidate_kind: &str) -> Option<String> {
    match proposal_kind_for_candidate_kind(candidate_kind) {
        "defer" | "refuse" | "final_answer" => None,
        _ => Some(format!("candidate.{}", candidate_kind.trim())),
    }
}

fn proposal_confidence_for_arbitration(confidence: &str) -> Option<f64> {
    match confidence.trim() {
        "high" => Some(0.9),
        "medium" => Some(0.72),
        "guarded" => Some(0.58),
        "reduced" => Some(0.41),
        "low" => Some(0.28),
        _ => None,
    }
}

fn mediation_outcome_for_gate_decision(gate_decision: &str) -> &'static str {
    match gate_decision {
        "allow" => "approved",
        "refuse" => "rejected",
        "defer" => "deferred",
        "escalate" => "escalated",
        _ => "rejected",
    }
}

fn skill_id_for_proposal(proposal: &ActionProposalRecord) -> String {
    if proposal.kind == "skill_call" {
        proposal
            .target
            .as_deref()
            .unwrap_or("candidate.review_and_refine")
            .replace("candidate.", "skill.")
    } else {
        "skill.none".to_string()
    }
}

fn skill_selection_status(proposal: &ActionProposalRecord) -> &'static str {
    if proposal.kind == "skill_call" {
        "selected"
    } else {
        "not_selected"
    }
}

fn skill_purpose_for_proposal(proposal: &ActionProposalRecord) -> String {
    if proposal.kind == "skill_call" {
        format!(
            "execute '{}' as a reusable bounded skill instead of leaving it as implicit model behavior",
            proposal
                .target
                .as_deref()
                .unwrap_or("candidate.review_and_refine")
        )
    } else {
        format!(
            "record that the bounded runtime selected '{}' rather than a governed skill invocation",
            proposal.kind
        )
    }
}

fn skill_bounded_role_for_proposal(proposal: &ActionProposalRecord) -> &'static str {
    if proposal.kind == "skill_call" {
        "carry the bounded candidate intent as an explicit reusable execution unit before authorization and execution"
    } else {
        "make the distinction between governed skill invocations and other bounded runtime actions reviewer-legible"
    }
}

fn skill_stop_condition_for_mediation(mediation: &ActionMediationRecord) -> &'static str {
    match mediation.mediation_outcome.as_str() {
        "approved" => {
            "stop after runtime authorization succeeds; privileged execution proceeds only through the bounded execution lane"
        }
        "rejected" => "stop before execution and surface bounded refusal as the final mediated outcome",
        "deferred" => "stop before execution and carry the proposal forward as deferred work",
        "escalated" => "stop before execution and require explicit judgment review or escalation handling",
        _ => "stop before execution when mediation outcome is not recognized",
    }
}

fn skill_protocol_lifecycle_state(mediation: &ActionMediationRecord) -> &'static str {
    match mediation.mediation_outcome.as_str() {
        "approved" => "authorized_ready_for_execution",
        "rejected" => "rejected_before_execution",
        "deferred" => "deferred_before_execution",
        "escalated" => "escalated_before_execution",
        _ => "blocked_before_execution",
    }
}

pub(crate) fn build_control_path_action_proposals_artifact(
    run_summary: &RunSummaryArtifact,
    arbitration: &CognitiveArbitrationArtifact,
    agency: &AgencySelectionArtifact,
    freedom_gate: &FreedomGateArtifact,
    scores: Option<&ScoresArtifact>,
) -> ControlPathActionProposalsArtifact {
    let selected_candidate = agency
        .candidate_set
        .iter()
        .find(|candidate| candidate.candidate_id == agency.selected_candidate_id)
        .cloned()
        .unwrap_or_else(|| AgencyCandidateRecord {
            candidate_id: agency.selected_candidate_id.clone(),
            candidate_kind: "review_and_refine".to_string(),
            bounded_action: freedom_gate.input.candidate_action.clone(),
            review_requirement: "review_required".to_string(),
            execution_priority: 1,
            rationale: agency.selected_candidate_reason.clone(),
        });
    let mut arguments = BTreeMap::new();
    arguments.insert(
        "candidate_id".to_string(),
        agency.selected_candidate_id.clone(),
    );
    arguments.insert(
        "candidate_kind".to_string(),
        selected_candidate.candidate_kind.clone(),
    );
    arguments.insert(
        "requested_action".to_string(),
        selected_candidate.bounded_action.clone(),
    );
    arguments.insert(
        "route_selected".to_string(),
        arbitration.route_selected.clone(),
    );

    let mut metadata = BTreeMap::new();
    metadata.insert(
        "surface_id".to_string(),
        "decision.commitment_gate".to_string(),
    );
    metadata.insert(
        "decision_record_kind".to_string(),
        freedom_gate.decision_record_kind.clone(),
    );
    metadata.insert(
        "risk_class".to_string(),
        freedom_gate.input.risk_class.clone(),
    );

    ControlPathActionProposalsArtifact {
        control_path_action_proposals_version: CONTROL_PATH_ACTION_PROPOSALS_VERSION,
        run_id: run_summary.run_id.clone(),
        generated_from: AeeDecisionGeneratedFrom {
            artifact_model_version: run_summary.artifact_model_version,
            run_summary_version: run_summary.run_summary_version,
            suggestions_version: arbitration.generated_from.suggestions_version,
            scores_version: scores.map(|value| value.scores_version),
        },
        proposal_schema_name: "adl.runtime.action_proposal.v1".to_string(),
        proposal_schema_fields: vec![
            "proposal_id".to_string(),
            "kind".to_string(),
            "target".to_string(),
            "arguments".to_string(),
            "intent".to_string(),
            "content".to_string(),
            "confidence".to_string(),
            "requires_approval".to_string(),
            "metadata".to_string(),
            "non_authoritative".to_string(),
            "temporal_anchor".to_string(),
        ],
        proposal_kind_vocabulary: vec![
            "tool_call".to_string(),
            "skill_call".to_string(),
            "memory_read".to_string(),
            "memory_write".to_string(),
            "final_answer".to_string(),
            "refuse".to_string(),
            "defer".to_string(),
        ],
        proposals: vec![ActionProposalRecord {
            proposal_id: "proposal.selected_candidate".to_string(),
            kind: proposal_kind_for_candidate_kind(&selected_candidate.candidate_kind).to_string(),
            target: proposal_target_for_candidate_kind(&selected_candidate.candidate_kind),
            arguments,
            intent: agency.selected_candidate_reason.clone(),
            content: None,
            confidence: proposal_confidence_for_arbitration(&arbitration.confidence),
            requires_approval: freedom_gate.input.policy_context.requires_review
                || freedom_gate.input.consequence_context.escalation_available
                || freedom_gate.commitment_blocked,
            metadata,
            non_authoritative: true,
            temporal_anchor: "control_path/candidate_selection.json".to_string(),
        }],
    }
}

pub(crate) fn build_control_path_decisions_artifact(
    run_summary: &RunSummaryArtifact,
    arbitration: &CognitiveArbitrationArtifact,
    agency: &AgencySelectionArtifact,
    evaluation: &EvaluationSignalsArtifact,
    reframing: &ReframingArtifact,
    freedom_gate: &FreedomGateArtifact,
    scores: Option<&ScoresArtifact>,
) -> ControlPathDecisionsArtifact {
    let route_policy_bindings = if arbitration.applied_constraints.is_empty() {
        vec!["no_explicit_constraints".to_string()]
    } else {
        arbitration.applied_constraints.clone()
    };
    let reframing_policy_bindings = vec![
        format!("frame_adequacy_score={}", reframing.frame_adequacy_score),
        format!("termination_reason={}", evaluation.termination_reason),
        format!("progress_signal={}", evaluation.progress_signal),
    ];
    let gate_policy_bindings = vec![
        format!(
            "route_selected={}",
            freedom_gate.input.policy_context.route_selected
        ),
        format!(
            "selected_candidate_kind={}",
            freedom_gate.input.policy_context.selected_candidate_kind
        ),
        format!(
            "requires_review={}",
            freedom_gate.input.policy_context.requires_review
        ),
        format!(
            "policy_blocked={}",
            freedom_gate.input.policy_context.policy_blocked
        ),
        format!(
            "impact_scope={}",
            freedom_gate.input.consequence_context.impact_scope
        ),
        format!(
            "operator_visibility={}",
            freedom_gate.input.consequence_context.operator_visibility
        ),
        format!(
            "escalation_available={}",
            freedom_gate.input.consequence_context.escalation_available
        ),
    ];

    ControlPathDecisionsArtifact {
        control_path_decisions_version: CONTROL_PATH_DECISIONS_VERSION,
        run_id: run_summary.run_id.clone(),
        generated_from: AeeDecisionGeneratedFrom {
            artifact_model_version: run_summary.artifact_model_version,
            run_summary_version: run_summary.run_summary_version,
            suggestions_version: arbitration.generated_from.suggestions_version,
            scores_version: scores.map(|value| value.scores_version),
        },
        decision_schema_name: "adl.runtime.decision.v1".to_string(),
        decision_schema_fields: vec![
            "decision_id".to_string(),
            "surface_id".to_string(),
            "proposal_or_action".to_string(),
            "outcome_class".to_string(),
            "decision_maker".to_string(),
            "policy_bindings".to_string(),
            "rationale".to_string(),
            "downstream_consequence".to_string(),
            "temporal_anchor".to_string(),
        ],
        outcome_class_vocabulary: vec![
            "accept".to_string(),
            "reject".to_string(),
            "defer".to_string(),
            "escalate".to_string(),
            "reroute".to_string(),
        ],
        surfaces: vec![
            DecisionSurfaceRecord {
                surface_id: "delegation_and_routing.route_selection".to_string(),
                surface_family: "delegation_and_routing".to_string(),
                bounded_role:
                    "select the bounded runtime path before commitment is attempted".to_string(),
                outcome_classes: vec!["accept".to_string(), "reroute".to_string()],
                temporal_anchor_ref: "control_path/arbitration.json".to_string(),
            },
            DecisionSurfaceRecord {
                surface_id: "recovery_continuity.reframing".to_string(),
                surface_family: "recovery_continuity".to_string(),
                bounded_role:
                    "decide whether the current frame should be retained or rerouted through reframing"
                        .to_string(),
                outcome_classes: vec!["accept".to_string(), "reroute".to_string()],
                temporal_anchor_ref: "control_path/reframing.json".to_string(),
            },
            DecisionSurfaceRecord {
                surface_id: "pre_execution_authorization.commitment_gate".to_string(),
                surface_family: "pre_execution_authorization".to_string(),
                bounded_role:
                    "decide whether commitment may proceed for the selected bounded candidate"
                        .to_string(),
                outcome_classes: vec![
                    "accept".to_string(),
                    "reject".to_string(),
                    "defer".to_string(),
                    "escalate".to_string(),
                ],
                temporal_anchor_ref: "control_path/freedom_gate.json".to_string(),
            },
        ],
        decisions: vec![
            DecisionRecord {
                decision_id: "decision.route_selection".to_string(),
                surface_id: "delegation_and_routing.route_selection".to_string(),
                proposal_or_action: format!(
                    "route candidate {} through the {} path",
                    agency.selected_candidate_id, arbitration.route_selected
                ),
                outcome_class: route_outcome_class(&arbitration.route_selected).to_string(),
                decision_maker: "cognitive_arbitration".to_string(),
                policy_bindings: route_policy_bindings,
                rationale: arbitration.route_reason.clone(),
                downstream_consequence: format!(
                    "selected_path={} reasoning_mode={}",
                    arbitration.route_selected, arbitration.reasoning_mode
                ),
                temporal_anchor: "control_path/arbitration.json".to_string(),
            },
            DecisionRecord {
                decision_id: "decision.reframing".to_string(),
                surface_id: "recovery_continuity.reframing".to_string(),
                proposal_or_action: format!(
                    "decide whether candidate {} should keep the current frame or reframe before re-execution",
                    agency.selected_candidate_id
                ),
                outcome_class: reframing_outcome_class(&reframing.reframing_trigger).to_string(),
                decision_maker: "reframing_control".to_string(),
                policy_bindings: reframing_policy_bindings,
                rationale: reframing.reframing_reason.clone(),
                downstream_consequence: reframing.reexecution_choice.clone(),
                temporal_anchor: "control_path/reframing.json".to_string(),
            },
            DecisionRecord {
                decision_id: "decision.commitment_gate".to_string(),
                surface_id: "pre_execution_authorization.commitment_gate".to_string(),
                proposal_or_action: freedom_gate
                    .selected_action_or_none
                    .clone()
                    .filter(|value| !value.trim().is_empty())
                    .unwrap_or_else(|| {
                        let candidate_action = freedom_gate.input.candidate_action.trim();
                        if candidate_action.is_empty() {
                            "withhold commitment until bounded context is restored".to_string()
                        } else {
                            candidate_action.to_string()
                        }
                    }),
                outcome_class: gate_outcome_class(&freedom_gate.gate_decision).to_string(),
                decision_maker: "freedom_gate".to_string(),
                policy_bindings: gate_policy_bindings,
                rationale: freedom_gate.decision_reason.clone(),
                downstream_consequence: freedom_gate
                    .selected_action_or_none
                    .clone()
                    .unwrap_or_else(|| freedom_gate.required_follow_up.clone()),
                temporal_anchor: "control_path/freedom_gate.json".to_string(),
            },
        ],
    }
}

pub(crate) fn build_control_path_action_mediation_artifact(
    run_summary: &RunSummaryArtifact,
    action_proposals: &ControlPathActionProposalsArtifact,
    freedom_gate: &FreedomGateArtifact,
    decisions: &ControlPathDecisionsArtifact,
    scores: Option<&ScoresArtifact>,
) -> ControlPathActionMediationArtifact {
    let proposal = action_proposals
        .proposals
        .first()
        .cloned()
        .unwrap_or_else(|| ActionProposalRecord {
            proposal_id: "proposal.none".to_string(),
            kind: "defer".to_string(),
            target: None,
            arguments: BTreeMap::new(),
            intent: "no bounded proposal available".to_string(),
            content: None,
            confidence: None,
            requires_approval: true,
            metadata: BTreeMap::new(),
            non_authoritative: true,
            temporal_anchor: "control_path/candidate_selection.json".to_string(),
        });
    let gate_decision = decisions
        .decisions
        .iter()
        .find(|record| record.decision_id == "decision.commitment_gate")
        .cloned()
        .unwrap_or_else(|| DecisionRecord {
            decision_id: "decision.commitment_gate".to_string(),
            surface_id: "pre_execution_authorization.commitment_gate".to_string(),
            proposal_or_action: proposal.intent.clone(),
            outcome_class: gate_outcome_class(&freedom_gate.gate_decision).to_string(),
            decision_maker: "freedom_gate".to_string(),
            policy_bindings: Vec::new(),
            rationale: freedom_gate.decision_reason.clone(),
            downstream_consequence: freedom_gate.required_follow_up.clone(),
            temporal_anchor: "control_path/freedom_gate.json".to_string(),
        });
    let approved_action_or_none = if freedom_gate.gate_decision == "allow" {
        freedom_gate
            .selected_action_or_none
            .clone()
            .or_else(|| Some(freedom_gate.input.candidate_action.clone()))
    } else {
        None
    };

    ControlPathActionMediationArtifact {
        control_path_action_mediation_version: CONTROL_PATH_ACTION_MEDIATION_VERSION,
        run_id: run_summary.run_id.clone(),
        generated_from: AeeDecisionGeneratedFrom {
            artifact_model_version: run_summary.artifact_model_version,
            run_summary_version: run_summary.run_summary_version,
            suggestions_version: action_proposals.generated_from.suggestions_version,
            scores_version: scores.map(|value| value.scores_version),
        },
        authority_boundary: "models_propose_runtime_decides_executes".to_string(),
        mediation_outcome_vocabulary: vec![
            "approved".to_string(),
            "rejected".to_string(),
            "deferred".to_string(),
            "escalated".to_string(),
        ],
        mediation: ActionMediationRecord {
            mediation_id: "mediation.commitment_gate".to_string(),
            proposal_id: proposal.proposal_id,
            decision_id: gate_decision.decision_id,
            runtime_authority: "freedom_gate".to_string(),
            judgment_boundary: freedom_gate.judgment_boundary.clone(),
            mediation_outcome: mediation_outcome_for_gate_decision(&freedom_gate.gate_decision)
                .to_string(),
            approved_action_or_none,
            required_follow_up: freedom_gate.required_follow_up.clone(),
            validation_checks: vec![
                "proposal_non_authoritative".to_string(),
                "decision_surface_linked".to_string(),
                "policy_bindings_present".to_string(),
                "freedom_gate_authority_boundary".to_string(),
            ],
            policy_bindings: gate_decision.policy_bindings,
            rationale: freedom_gate.decision_reason.clone(),
            temporal_anchor: "control_path/freedom_gate.json".to_string(),
            trace_expectation:
                "approval, rejection, defer, or escalation remains trace-visible before privileged execution"
                    .to_string(),
        },
    }
}

pub(crate) fn build_control_path_skill_model_artifact(
    run_summary: &RunSummaryArtifact,
    action_proposals: &ControlPathActionProposalsArtifact,
    mediation: &ControlPathActionMediationArtifact,
    scores: Option<&ScoresArtifact>,
) -> ControlPathSkillModelArtifact {
    let proposal = action_proposals
        .proposals
        .first()
        .cloned()
        .unwrap_or_else(|| ActionProposalRecord {
            proposal_id: "proposal.none".to_string(),
            kind: "defer".to_string(),
            target: None,
            arguments: BTreeMap::new(),
            intent: "no bounded proposal available".to_string(),
            content: None,
            confidence: None,
            requires_approval: true,
            metadata: BTreeMap::new(),
            non_authoritative: true,
            temporal_anchor: "control_path/candidate_selection.json".to_string(),
        });

    ControlPathSkillModelArtifact {
        control_path_skill_model_version: CONTROL_PATH_SKILL_MODEL_VERSION,
        run_id: run_summary.run_id.clone(),
        generated_from: AeeDecisionGeneratedFrom {
            artifact_model_version: run_summary.artifact_model_version,
            run_summary_version: run_summary.run_summary_version,
            suggestions_version: action_proposals.generated_from.suggestions_version,
            scores_version: scores.map(|value| value.scores_version),
        },
        skill_schema_name: "adl.runtime.skill_model.v1".to_string(),
        skill_schema_fields: vec![
            "skill_id".to_string(),
            "selection_status".to_string(),
            "purpose".to_string(),
            "bounded_role".to_string(),
            "input_contract_fields".to_string(),
            "output_contract_surfaces".to_string(),
            "stop_condition".to_string(),
            "distinguished_from".to_string(),
            "temporal_anchor".to_string(),
        ],
        distinction_vocabulary: vec![
            "skill".to_string(),
            "provider_capability".to_string(),
            "raw_aptitude".to_string(),
            "tool_call".to_string(),
            "memory_operation".to_string(),
            "final_answer".to_string(),
        ],
        selected_execution_unit_kind: proposal.kind.clone(),
        skill: SkillDefinitionRecord {
            skill_id: skill_id_for_proposal(&proposal),
            selection_status: skill_selection_status(&proposal).to_string(),
            purpose: skill_purpose_for_proposal(&proposal),
            bounded_role: skill_bounded_role_for_proposal(&proposal).to_string(),
            input_contract_fields: proposal.arguments.keys().cloned().collect(),
            output_contract_surfaces: vec![
                "control_path/mediation.json".to_string(),
                "control_path/final_result.json".to_string(),
                "logs/trace_v1.json".to_string(),
            ],
            stop_condition: skill_stop_condition_for_mediation(&mediation.mediation).to_string(),
            distinguished_from: vec![
                "provider_capability".to_string(),
                "raw_aptitude".to_string(),
                "tool_call".to_string(),
            ],
            temporal_anchor: "control_path/action_proposals.json".to_string(),
        },
    }
}

pub(crate) fn build_control_path_skill_execution_protocol_artifact(
    run_summary: &RunSummaryArtifact,
    action_proposals: &ControlPathActionProposalsArtifact,
    skill_model: &ControlPathSkillModelArtifact,
    mediation: &ControlPathActionMediationArtifact,
    scores: Option<&ScoresArtifact>,
) -> ControlPathSkillExecutionProtocolArtifact {
    let proposal = action_proposals
        .proposals
        .first()
        .cloned()
        .unwrap_or_else(|| ActionProposalRecord {
            proposal_id: "proposal.none".to_string(),
            kind: "defer".to_string(),
            target: None,
            arguments: BTreeMap::new(),
            intent: "no bounded proposal available".to_string(),
            content: None,
            confidence: None,
            requires_approval: true,
            metadata: BTreeMap::new(),
            non_authoritative: true,
            temporal_anchor: "control_path/candidate_selection.json".to_string(),
        });

    let mut invocation_context = BTreeMap::new();
    invocation_context.insert("run_id".to_string(), run_summary.run_id.clone());
    invocation_context.insert(
        "selected_execution_unit_kind".to_string(),
        proposal.kind.clone(),
    );
    if let Some(route_selected) = proposal.arguments.get("route_selected") {
        invocation_context.insert("route_selected".to_string(), route_selected.clone());
    }
    if let Some(candidate_id) = proposal.arguments.get("candidate_id") {
        invocation_context.insert("candidate_id".to_string(), candidate_id.clone());
    }

    ControlPathSkillExecutionProtocolArtifact {
        control_path_skill_execution_protocol_version:
            CONTROL_PATH_SKILL_EXECUTION_PROTOCOL_VERSION,
        run_id: run_summary.run_id.clone(),
        generated_from: AeeDecisionGeneratedFrom {
            artifact_model_version: run_summary.artifact_model_version,
            run_summary_version: run_summary.run_summary_version,
            suggestions_version: action_proposals.generated_from.suggestions_version,
            scores_version: scores.map(|value| value.scores_version),
        },
        protocol_name: "adl.runtime.skill_execution_protocol.v1".to_string(),
        lifecycle_stages: vec![
            "proposed".to_string(),
            "validated".to_string(),
            "authorized".to_string(),
            "trace_visible".to_string(),
            "ready_for_execution".to_string(),
        ],
        invocation: SkillInvocationProtocolRecord {
            invocation_id: "skill_invocation.selected_proposal".to_string(),
            skill_id: skill_model.skill.skill_id.clone(),
            proposal_id: proposal.proposal_id,
            decision_id: mediation.mediation.decision_id.clone(),
            invocation_kind: proposal.kind,
            invocation_context,
            input_validation_expectation:
                "proposal schema, mediation linkage, and authority-boundary checks complete before execution"
                    .to_string(),
            lifecycle_state: skill_protocol_lifecycle_state(&mediation.mediation).to_string(),
            authorization_decision: mediation.mediation.mediation_outcome.clone(),
            output_contract_surfaces: vec![
                "control_path/mediation.json".to_string(),
                "control_path/final_result.json".to_string(),
                "logs/trace_v1.json".to_string(),
            ],
            error_outcome_vocabulary: vec![
                "rejected".to_string(),
                "deferred".to_string(),
                "escalated".to_string(),
            ],
            trace_expectation: mediation.mediation.trace_expectation.clone(),
            temporal_anchor: "control_path/mediation.json".to_string(),
        },
    }
}

pub(crate) fn build_control_path_final_result_artifact(
    run_summary: &RunSummaryArtifact,
    arbitration: &CognitiveArbitrationArtifact,
    agency: &AgencySelectionArtifact,
    evaluation: &EvaluationSignalsArtifact,
    freedom_gate: &FreedomGateArtifact,
) -> ControlPathFinalResultArtifact {
    let final_result = match freedom_gate.gate_decision.as_str() {
        "allow" => freedom_gate
            .selected_action_or_none
            .clone()
            .or_else(|| {
                agency
                    .candidate_set
                    .iter()
                    .find(|candidate| candidate.candidate_id == agency.selected_candidate_id)
                    .map(|candidate| candidate.bounded_action.clone())
            })
            .unwrap_or_else(|| agency.selected_candidate_reason.clone()),
        "defer" => "defer".to_string(),
        "refuse" => "refuse".to_string(),
        "escalate" => "escalate".to_string(),
        other => format!("unrecognized_gate_decision:{other}"),
    };

    ControlPathFinalResultArtifact {
        control_path_final_result_version: CONTROL_PATH_FINAL_RESULT_VERSION,
        run_id: run_summary.run_id.clone(),
        route_selected: arbitration.route_selected.clone(),
        selected_candidate: agency.selected_candidate_id.clone(),
        termination_reason: evaluation.termination_reason.clone(),
        gate_decision: freedom_gate.gate_decision.clone(),
        final_result,
        commitment_blocked: freedom_gate.commitment_blocked,
        next_control_action: evaluation.next_control_action.clone(),
        stage_order: vec![
            "signals".to_string(),
            "candidate_selection".to_string(),
            "arbitration".to_string(),
            "execution".to_string(),
            "evaluation".to_string(),
            "reframing".to_string(),
            "memory".to_string(),
            "freedom_gate".to_string(),
            "final_result".to_string(),
        ],
    }
}
