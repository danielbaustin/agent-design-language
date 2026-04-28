use super::super::*;
use ::adl::trace_schema_v1::{
    validate_trace_event_envelope_v1, ContractValidationResultV1, TraceActorTypeV1, TraceActorV1,
    TraceContractValidationV1, TraceDecisionContextV1, TraceErrorV1, TraceEventEnvelopeV1,
    TraceEventTypeV1, TraceEventV1, TraceGovernanceEvidenceV1, TraceRedactionDecisionV1,
    TraceScopeLevelV1, TraceScopeV1, TraceVisibilityViewsV1,
};
use serde_json::json;

pub(super) fn build_trace_v1_envelope(
    resolved: &resolve::AdlResolved,
    tr: &trace::Trace,
    steps: &[StepStateArtifact],
    start_ms: u128,
    end_ms: u128,
    status: &str,
    failure: Option<&anyhow::Error>,
) -> Result<TraceEventEnvelopeV1> {
    let mut events = Vec::new();
    let mut next_id: u64 = 1;
    let trace_id = resolved.run_id.clone();
    let root_span_id = format!("run:{}", resolved.run_id);
    let run_ref = artifact_ref(&resolved.run_id, "run.json");
    let steps_ref = artifact_ref(&resolved.run_id, "steps.json");
    let activation_log_ref = artifact_ref(&resolved.run_id, "logs/activation_log.json");

    push_trace_v1_event(
        &mut events,
        &mut next_id,
        TraceEventV1 {
            event_id: String::new(),
            timestamp: trace::format_iso_utc_ms(start_ms),
            event_type: TraceEventTypeV1::RunStart,
            trace_id: trace_id.clone(),
            run_id: resolved.run_id.clone(),
            span_id: root_span_id.clone(),
            parent_span_id: None,
            actor: TraceActorV1 {
                r#type: TraceActorTypeV1::Agent,
                id: resolved.workflow_id.clone(),
            },
            scope: TraceScopeV1 {
                level: TraceScopeLevelV1::Run,
                name: resolved.workflow_id.clone(),
            },
            inputs_ref: Some(run_ref.clone()),
            outputs_ref: Some(activation_log_ref.clone()),
            artifact_ref: Some(run_ref.clone()),
            decision_context: None,
            provider: None,
            error: None,
            contract_validation: None,
            governance: None,
            redaction: None,
        },
    );

    for event in &tr.events {
        match event {
            trace::TraceEvent::LifecyclePhaseEntered { ts_ms, phase, .. } => push_trace_v1_event(
                &mut events,
                &mut next_id,
                TraceEventV1 {
                    event_id: String::new(),
                    timestamp: trace::format_iso_utc_ms(*ts_ms),
                    event_type: TraceEventTypeV1::LifecyclePhase,
                    trace_id: trace_id.clone(),
                    run_id: resolved.run_id.clone(),
                    span_id: format!("run:{}:phase:{}", resolved.run_id, phase.as_str()),
                    parent_span_id: Some(root_span_id.clone()),
                    actor: TraceActorV1 {
                        r#type: TraceActorTypeV1::System,
                        id: "runtime".to_string(),
                    },
                    scope: TraceScopeV1 {
                        level: TraceScopeLevelV1::Run,
                        name: phase.as_str().to_string(),
                    },
                    inputs_ref: Some(run_ref.clone()),
                    outputs_ref: Some(activation_log_ref.clone()),
                    artifact_ref: Some(activation_log_ref.clone()),
                    decision_context: Some(TraceDecisionContextV1 {
                        context: "runtime lifecycle phase".to_string(),
                        outcome: phase.as_str().to_string(),
                        rationale: None,
                    }),
                    provider: None,
                    error: None,
                    contract_validation: None,
                    governance: None,
                    redaction: None,
                },
            ),
            trace::TraceEvent::ExecutionBoundaryCrossed {
                ts_ms,
                boundary,
                state,
                ..
            } => push_trace_v1_event(
                &mut events,
                &mut next_id,
                TraceEventV1 {
                    event_id: String::new(),
                    timestamp: trace::format_iso_utc_ms(*ts_ms),
                    event_type: TraceEventTypeV1::ExecutionBoundary,
                    trace_id: trace_id.clone(),
                    run_id: resolved.run_id.clone(),
                    span_id: format!(
                        "run:{}:boundary:{}:{}",
                        resolved.run_id,
                        boundary.as_str(),
                        state
                    ),
                    parent_span_id: Some(root_span_id.clone()),
                    actor: TraceActorV1 {
                        r#type: TraceActorTypeV1::System,
                        id: "runtime".to_string(),
                    },
                    scope: TraceScopeV1 {
                        level: TraceScopeLevelV1::Run,
                        name: boundary.as_str().to_string(),
                    },
                    inputs_ref: Some(run_ref.clone()),
                    outputs_ref: Some(activation_log_ref.clone()),
                    artifact_ref: Some(activation_log_ref.clone()),
                    decision_context: Some(TraceDecisionContextV1 {
                        context: "execution boundary".to_string(),
                        outcome: state.clone(),
                        rationale: None,
                    }),
                    provider: None,
                    error: None,
                    contract_validation: None,
                    governance: None,
                    redaction: None,
                },
            ),
            trace::TraceEvent::GovernedProposalObserved {
                ts_ms,
                proposal_id,
                tool_name,
                redacted_arguments_ref,
                ..
            } => push_trace_v1_event(
                &mut events,
                &mut next_id,
                TraceEventV1 {
                    event_id: String::new(),
                    timestamp: trace::format_iso_utc_ms(*ts_ms),
                    event_type: TraceEventTypeV1::Proposal,
                    trace_id: trace_id.clone(),
                    run_id: resolved.run_id.clone(),
                    span_id: format!("governed:{proposal_id}:proposal"),
                    parent_span_id: Some(root_span_id.clone()),
                    actor: TraceActorV1 {
                        r#type: TraceActorTypeV1::Agent,
                        id: resolved.workflow_id.clone(),
                    },
                    scope: TraceScopeV1 {
                        level: TraceScopeLevelV1::Run,
                        name: "governed_proposal".to_string(),
                    },
                    inputs_ref: Some(redacted_arguments_ref.clone()),
                    outputs_ref: None,
                    artifact_ref: Some(activation_log_ref.clone()),
                    decision_context: None,
                    provider: None,
                    error: None,
                    contract_validation: None,
                    governance: Some(TraceGovernanceEvidenceV1 {
                        proposal_id: Some(proposal_id.clone()),
                        normalized_proposal_ref: None,
                        acc_contract_id: None,
                        policy_evidence_ref: None,
                        gate_candidate_id: None,
                        gate_boundary: None,
                        gate_reason_code: None,
                        action_id: None,
                        tool_name: Some(tool_name.clone()),
                        adapter_id: None,
                        replay_posture: None,
                        result_ref: None,
                        redaction_summary: None,
                        evidence_refs: vec![format!("proposal:{proposal_id}")],
                        visibility_views: None,
                    }),
                    redaction: None,
                },
            ),
            trace::TraceEvent::GovernedProposalNormalized {
                ts_ms,
                proposal_id,
                normalized_proposal_ref,
                redacted_arguments_ref,
                ..
            } => push_trace_v1_event(
                &mut events,
                &mut next_id,
                TraceEventV1 {
                    event_id: String::new(),
                    timestamp: trace::format_iso_utc_ms(*ts_ms),
                    event_type: TraceEventTypeV1::ProposalNormalization,
                    trace_id: trace_id.clone(),
                    run_id: resolved.run_id.clone(),
                    span_id: format!("governed:{proposal_id}:normalized"),
                    parent_span_id: Some(root_span_id.clone()),
                    actor: TraceActorV1 {
                        r#type: TraceActorTypeV1::System,
                        id: "proposal-normalizer".to_string(),
                    },
                    scope: TraceScopeV1 {
                        level: TraceScopeLevelV1::Run,
                        name: "governed_normalization".to_string(),
                    },
                    inputs_ref: Some(redacted_arguments_ref.clone()),
                    outputs_ref: None,
                    artifact_ref: Some(activation_log_ref.clone()),
                    decision_context: None,
                    provider: None,
                    error: None,
                    contract_validation: None,
                    governance: Some(TraceGovernanceEvidenceV1 {
                        proposal_id: Some(proposal_id.clone()),
                        normalized_proposal_ref: Some(normalized_proposal_ref.clone()),
                        acc_contract_id: None,
                        policy_evidence_ref: None,
                        gate_candidate_id: None,
                        gate_boundary: None,
                        gate_reason_code: None,
                        action_id: None,
                        tool_name: None,
                        adapter_id: None,
                        replay_posture: None,
                        result_ref: None,
                        redaction_summary: None,
                        evidence_refs: vec![format!(
                            "normalized_proposal:{normalized_proposal_ref}"
                        )],
                        visibility_views: None,
                    }),
                    redaction: None,
                },
            ),
            trace::TraceEvent::GovernedAccConstructed {
                ts_ms,
                proposal_id,
                acc_contract_id,
                replay_posture,
                ..
            } => push_trace_v1_event(
                &mut events,
                &mut next_id,
                TraceEventV1 {
                    event_id: String::new(),
                    timestamp: trace::format_iso_utc_ms(*ts_ms),
                    event_type: TraceEventTypeV1::CapabilityContract,
                    trace_id: trace_id.clone(),
                    run_id: resolved.run_id.clone(),
                    span_id: format!("governed:{proposal_id}:acc"),
                    parent_span_id: Some(root_span_id.clone()),
                    actor: TraceActorV1 {
                        r#type: TraceActorTypeV1::System,
                        id: "acc-compiler".to_string(),
                    },
                    scope: TraceScopeV1 {
                        level: TraceScopeLevelV1::Run,
                        name: "governed_acc".to_string(),
                    },
                    inputs_ref: Some(run_ref.clone()),
                    outputs_ref: None,
                    artifact_ref: Some(activation_log_ref.clone()),
                    decision_context: None,
                    provider: None,
                    error: None,
                    contract_validation: None,
                    governance: Some(TraceGovernanceEvidenceV1 {
                        proposal_id: Some(proposal_id.clone()),
                        normalized_proposal_ref: None,
                        acc_contract_id: Some(acc_contract_id.clone()),
                        policy_evidence_ref: None,
                        gate_candidate_id: None,
                        gate_boundary: None,
                        gate_reason_code: None,
                        action_id: None,
                        tool_name: None,
                        adapter_id: None,
                        replay_posture: Some(replay_posture.clone()),
                        result_ref: None,
                        redaction_summary: None,
                        evidence_refs: vec![format!("acc:{acc_contract_id}")],
                        visibility_views: None,
                    }),
                    redaction: None,
                },
            ),
            trace::TraceEvent::GovernedPolicyInjected {
                ts_ms,
                proposal_id,
                policy_evidence_ref,
                outcome,
                ..
            } => push_trace_v1_event(
                &mut events,
                &mut next_id,
                TraceEventV1 {
                    event_id: String::new(),
                    timestamp: trace::format_iso_utc_ms(*ts_ms),
                    event_type: TraceEventTypeV1::PolicyInjection,
                    trace_id: trace_id.clone(),
                    run_id: resolved.run_id.clone(),
                    span_id: format!("governed:{proposal_id}:policy"),
                    parent_span_id: Some(root_span_id.clone()),
                    actor: TraceActorV1 {
                        r#type: TraceActorTypeV1::System,
                        id: "policy-engine".to_string(),
                    },
                    scope: TraceScopeV1 {
                        level: TraceScopeLevelV1::Run,
                        name: "governed_policy".to_string(),
                    },
                    inputs_ref: Some(run_ref.clone()),
                    outputs_ref: None,
                    artifact_ref: Some(activation_log_ref.clone()),
                    decision_context: Some(TraceDecisionContextV1 {
                        context: "policy injection".to_string(),
                        outcome: outcome.clone(),
                        rationale: None,
                    }),
                    provider: None,
                    error: None,
                    contract_validation: None,
                    governance: Some(TraceGovernanceEvidenceV1 {
                        proposal_id: Some(proposal_id.clone()),
                        normalized_proposal_ref: None,
                        acc_contract_id: None,
                        policy_evidence_ref: Some(policy_evidence_ref.clone()),
                        gate_candidate_id: None,
                        gate_boundary: None,
                        gate_reason_code: None,
                        action_id: None,
                        tool_name: None,
                        adapter_id: None,
                        replay_posture: None,
                        result_ref: None,
                        redaction_summary: None,
                        evidence_refs: vec![format!("policy:{policy_evidence_ref}")],
                        visibility_views: None,
                    }),
                    redaction: None,
                },
            ),
            trace::TraceEvent::GovernedVisibilityResolved {
                ts_ms,
                proposal_id,
                actor_view,
                operator_view,
                reviewer_view,
                public_report_view,
                observatory_projection,
                ..
            } => push_trace_v1_event(
                &mut events,
                &mut next_id,
                TraceEventV1 {
                    event_id: String::new(),
                    timestamp: trace::format_iso_utc_ms(*ts_ms),
                    event_type: TraceEventTypeV1::VisibilityPolicy,
                    trace_id: trace_id.clone(),
                    run_id: resolved.run_id.clone(),
                    span_id: format!("governed:{proposal_id}:visibility"),
                    parent_span_id: Some(root_span_id.clone()),
                    actor: TraceActorV1 {
                        r#type: TraceActorTypeV1::System,
                        id: "visibility-policy".to_string(),
                    },
                    scope: TraceScopeV1 {
                        level: TraceScopeLevelV1::Run,
                        name: "governed_visibility".to_string(),
                    },
                    inputs_ref: Some(run_ref.clone()),
                    outputs_ref: None,
                    artifact_ref: Some(activation_log_ref.clone()),
                    decision_context: None,
                    provider: None,
                    error: None,
                    contract_validation: None,
                    governance: Some(TraceGovernanceEvidenceV1 {
                        proposal_id: Some(proposal_id.clone()),
                        normalized_proposal_ref: None,
                        acc_contract_id: None,
                        policy_evidence_ref: None,
                        gate_candidate_id: None,
                        gate_boundary: None,
                        gate_reason_code: None,
                        action_id: None,
                        tool_name: None,
                        adapter_id: None,
                        replay_posture: None,
                        result_ref: None,
                        redaction_summary: None,
                        evidence_refs: vec!["visibility:policy".to_string()],
                        visibility_views: Some(TraceVisibilityViewsV1 {
                            actor_view: actor_view.clone(),
                            operator_view: operator_view.clone(),
                            reviewer_view: reviewer_view.clone(),
                            public_report_view: public_report_view.clone(),
                            observatory_projection: observatory_projection.clone(),
                        }),
                    }),
                    redaction: None,
                },
            ),
            trace::TraceEvent::GovernedFreedomGateDecided {
                ts_ms,
                proposal_id,
                candidate_id,
                decision,
                reason_code,
                boundary,
                redaction_summary,
                ..
            } => push_trace_v1_event(
                &mut events,
                &mut next_id,
                TraceEventV1 {
                    event_id: String::new(),
                    timestamp: trace::format_iso_utc_ms(*ts_ms),
                    event_type: TraceEventTypeV1::FreedomGateDecision,
                    trace_id: trace_id.clone(),
                    run_id: resolved.run_id.clone(),
                    span_id: format!("governed:{proposal_id}:gate"),
                    parent_span_id: Some(root_span_id.clone()),
                    actor: TraceActorV1 {
                        r#type: TraceActorTypeV1::System,
                        id: "freedom-gate".to_string(),
                    },
                    scope: TraceScopeV1 {
                        level: TraceScopeLevelV1::Run,
                        name: "governed_gate".to_string(),
                    },
                    inputs_ref: Some(run_ref.clone()),
                    outputs_ref: None,
                    artifact_ref: Some(activation_log_ref.clone()),
                    decision_context: Some(TraceDecisionContextV1 {
                        context: "freedom gate".to_string(),
                        outcome: decision.clone(),
                        rationale: Some(reason_code.clone()),
                    }),
                    provider: None,
                    error: None,
                    contract_validation: None,
                    governance: Some(TraceGovernanceEvidenceV1 {
                        proposal_id: Some(proposal_id.clone()),
                        normalized_proposal_ref: None,
                        acc_contract_id: None,
                        policy_evidence_ref: None,
                        gate_candidate_id: Some(candidate_id.clone()),
                        gate_boundary: Some(boundary.clone()),
                        gate_reason_code: Some(reason_code.clone()),
                        action_id: None,
                        tool_name: None,
                        adapter_id: None,
                        replay_posture: None,
                        result_ref: None,
                        redaction_summary: Some(redaction_summary.clone()),
                        evidence_refs: vec![format!("gate:{candidate_id}")],
                        visibility_views: None,
                    }),
                    redaction: None,
                },
            ),
            trace::TraceEvent::GovernedActionSelected {
                ts_ms,
                proposal_id,
                action_id,
                tool_name,
                adapter_id,
                ..
            } => push_trace_v1_event(
                &mut events,
                &mut next_id,
                TraceEventV1 {
                    event_id: String::new(),
                    timestamp: trace::format_iso_utc_ms(*ts_ms),
                    event_type: TraceEventTypeV1::ActionSelection,
                    trace_id: trace_id.clone(),
                    run_id: resolved.run_id.clone(),
                    span_id: format!("governed:{proposal_id}:selected:{action_id}"),
                    parent_span_id: Some(root_span_id.clone()),
                    actor: TraceActorV1 {
                        r#type: TraceActorTypeV1::System,
                        id: "governed-executor".to_string(),
                    },
                    scope: TraceScopeV1 {
                        level: TraceScopeLevelV1::Run,
                        name: "governed_selection".to_string(),
                    },
                    inputs_ref: Some(run_ref.clone()),
                    outputs_ref: None,
                    artifact_ref: Some(activation_log_ref.clone()),
                    decision_context: Some(TraceDecisionContextV1 {
                        context: "governed action selection".to_string(),
                        outcome: "selected".to_string(),
                        rationale: None,
                    }),
                    provider: None,
                    error: None,
                    contract_validation: None,
                    governance: Some(TraceGovernanceEvidenceV1 {
                        proposal_id: Some(proposal_id.clone()),
                        normalized_proposal_ref: None,
                        acc_contract_id: None,
                        policy_evidence_ref: None,
                        gate_candidate_id: None,
                        gate_boundary: None,
                        gate_reason_code: None,
                        action_id: Some(action_id.clone()),
                        tool_name: Some(tool_name.clone()),
                        adapter_id: Some(adapter_id.clone()),
                        replay_posture: None,
                        result_ref: None,
                        redaction_summary: None,
                        evidence_refs: vec!["governed_execution_allowed".to_string()],
                        visibility_views: None,
                    }),
                    redaction: None,
                },
            ),
            trace::TraceEvent::GovernedActionRejected {
                ts_ms,
                proposal_id,
                action_id,
                tool_name,
                adapter_id,
                reason_code,
                evidence_refs,
                ..
            } => push_trace_v1_event(
                &mut events,
                &mut next_id,
                TraceEventV1 {
                    event_id: String::new(),
                    timestamp: trace::format_iso_utc_ms(*ts_ms),
                    event_type: TraceEventTypeV1::ActionRejection,
                    trace_id: trace_id.clone(),
                    run_id: resolved.run_id.clone(),
                    span_id: format!("governed:{proposal_id}:rejected:{action_id}"),
                    parent_span_id: Some(root_span_id.clone()),
                    actor: TraceActorV1 {
                        r#type: TraceActorTypeV1::System,
                        id: "governed-executor".to_string(),
                    },
                    scope: TraceScopeV1 {
                        level: TraceScopeLevelV1::Run,
                        name: "governed_rejection".to_string(),
                    },
                    inputs_ref: Some(run_ref.clone()),
                    outputs_ref: None,
                    artifact_ref: Some(activation_log_ref.clone()),
                    decision_context: Some(TraceDecisionContextV1 {
                        context: "governed action rejection".to_string(),
                        outcome: "rejected".to_string(),
                        rationale: Some(reason_code.clone()),
                    }),
                    provider: None,
                    error: None,
                    contract_validation: None,
                    governance: Some(TraceGovernanceEvidenceV1 {
                        proposal_id: Some(proposal_id.clone()),
                        normalized_proposal_ref: None,
                        acc_contract_id: None,
                        policy_evidence_ref: None,
                        gate_candidate_id: None,
                        gate_boundary: None,
                        gate_reason_code: Some(reason_code.clone()),
                        action_id: Some(action_id.clone()),
                        tool_name: Some(tool_name.clone()),
                        adapter_id: Some(adapter_id.clone()),
                        replay_posture: None,
                        result_ref: None,
                        redaction_summary: None,
                        evidence_refs: evidence_refs.clone(),
                        visibility_views: None,
                    }),
                    redaction: None,
                },
            ),
            trace::TraceEvent::GovernedExecutionResultRecorded {
                ts_ms,
                proposal_id,
                action_id,
                adapter_id,
                result_ref,
                evidence_refs,
                ..
            } => push_trace_v1_event(
                &mut events,
                &mut next_id,
                TraceEventV1 {
                    event_id: String::new(),
                    timestamp: trace::format_iso_utc_ms(*ts_ms),
                    event_type: TraceEventTypeV1::ExecutionResult,
                    trace_id: trace_id.clone(),
                    run_id: resolved.run_id.clone(),
                    span_id: format!("governed:{proposal_id}:result:{action_id}"),
                    parent_span_id: Some(root_span_id.clone()),
                    actor: TraceActorV1 {
                        r#type: TraceActorTypeV1::Tool,
                        id: adapter_id.clone(),
                    },
                    scope: TraceScopeV1 {
                        level: TraceScopeLevelV1::Tool,
                        name: "governed_execution_result".to_string(),
                    },
                    inputs_ref: Some(run_ref.clone()),
                    outputs_ref: Some(result_ref.clone()),
                    artifact_ref: Some(result_ref.clone()),
                    decision_context: Some(TraceDecisionContextV1 {
                        context: "governed execution result".to_string(),
                        outcome: "completed".to_string(),
                        rationale: None,
                    }),
                    provider: None,
                    error: None,
                    contract_validation: None,
                    governance: Some(TraceGovernanceEvidenceV1 {
                        proposal_id: Some(proposal_id.clone()),
                        normalized_proposal_ref: None,
                        acc_contract_id: None,
                        policy_evidence_ref: None,
                        gate_candidate_id: None,
                        gate_boundary: None,
                        gate_reason_code: None,
                        action_id: Some(action_id.clone()),
                        tool_name: None,
                        adapter_id: Some(adapter_id.clone()),
                        replay_posture: None,
                        result_ref: Some(result_ref.clone()),
                        redaction_summary: None,
                        evidence_refs: evidence_refs.clone(),
                        visibility_views: None,
                    }),
                    redaction: None,
                },
            ),
            trace::TraceEvent::GovernedRefusalRecorded {
                ts_ms,
                proposal_id,
                action_id,
                reason_code,
                evidence_refs,
                ..
            } => push_trace_v1_event(
                &mut events,
                &mut next_id,
                TraceEventV1 {
                    event_id: String::new(),
                    timestamp: trace::format_iso_utc_ms(*ts_ms),
                    event_type: TraceEventTypeV1::Refusal,
                    trace_id: trace_id.clone(),
                    run_id: resolved.run_id.clone(),
                    span_id: format!("governed:{proposal_id}:refusal:{action_id}"),
                    parent_span_id: Some(root_span_id.clone()),
                    actor: TraceActorV1 {
                        r#type: TraceActorTypeV1::System,
                        id: "governed-executor".to_string(),
                    },
                    scope: TraceScopeV1 {
                        level: TraceScopeLevelV1::Run,
                        name: "governed_refusal".to_string(),
                    },
                    inputs_ref: Some(run_ref.clone()),
                    outputs_ref: None,
                    artifact_ref: Some(activation_log_ref.clone()),
                    decision_context: Some(TraceDecisionContextV1 {
                        context: "governed refusal".to_string(),
                        outcome: "refused".to_string(),
                        rationale: Some(reason_code.clone()),
                    }),
                    provider: None,
                    error: None,
                    contract_validation: None,
                    governance: Some(TraceGovernanceEvidenceV1 {
                        proposal_id: Some(proposal_id.clone()),
                        normalized_proposal_ref: None,
                        acc_contract_id: None,
                        policy_evidence_ref: None,
                        gate_candidate_id: None,
                        gate_boundary: None,
                        gate_reason_code: Some(reason_code.clone()),
                        action_id: Some(action_id.clone()),
                        tool_name: None,
                        adapter_id: None,
                        replay_posture: None,
                        result_ref: None,
                        redaction_summary: None,
                        evidence_refs: evidence_refs.clone(),
                        visibility_views: None,
                    }),
                    redaction: None,
                },
            ),
            trace::TraceEvent::GovernedRedactionDecisionRecorded {
                ts_ms,
                proposal_id,
                audience,
                surfaces,
                outcome,
                detail,
                ..
            } => push_trace_v1_event(
                &mut events,
                &mut next_id,
                TraceEventV1 {
                    event_id: String::new(),
                    timestamp: trace::format_iso_utc_ms(*ts_ms),
                    event_type: TraceEventTypeV1::RedactionDecision,
                    trace_id: trace_id.clone(),
                    run_id: resolved.run_id.clone(),
                    span_id: format!("governed:{proposal_id}:redaction:{audience}"),
                    parent_span_id: Some(root_span_id.clone()),
                    actor: TraceActorV1 {
                        r#type: TraceActorTypeV1::System,
                        id: "redaction-policy".to_string(),
                    },
                    scope: TraceScopeV1 {
                        level: TraceScopeLevelV1::Run,
                        name: "governed_redaction".to_string(),
                    },
                    inputs_ref: Some(run_ref.clone()),
                    outputs_ref: None,
                    artifact_ref: Some(activation_log_ref.clone()),
                    decision_context: None,
                    provider: None,
                    error: None,
                    contract_validation: None,
                    governance: Some(TraceGovernanceEvidenceV1 {
                        proposal_id: Some(proposal_id.clone()),
                        normalized_proposal_ref: None,
                        acc_contract_id: None,
                        policy_evidence_ref: None,
                        gate_candidate_id: None,
                        gate_boundary: None,
                        gate_reason_code: None,
                        action_id: None,
                        tool_name: None,
                        adapter_id: None,
                        replay_posture: None,
                        result_ref: None,
                        redaction_summary: None,
                        evidence_refs: Vec::new(),
                        visibility_views: None,
                    }),
                    redaction: Some(TraceRedactionDecisionV1 {
                        audience: audience.clone(),
                        surfaces: surfaces.clone(),
                        outcome: outcome.clone(),
                        detail: detail.clone(),
                    }),
                },
            ),
            trace::TraceEvent::StepStarted {
                ts_ms,
                step_id,
                agent_id,
                ..
            } => push_trace_v1_event(
                &mut events,
                &mut next_id,
                TraceEventV1 {
                    event_id: String::new(),
                    timestamp: trace::format_iso_utc_ms(*ts_ms),
                    event_type: TraceEventTypeV1::StepStart,
                    trace_id: trace_id.clone(),
                    run_id: resolved.run_id.clone(),
                    span_id: format!("step:{step_id}"),
                    parent_span_id: Some(root_span_id.clone()),
                    actor: TraceActorV1 {
                        r#type: TraceActorTypeV1::Agent,
                        id: agent_id.clone(),
                    },
                    scope: TraceScopeV1 {
                        level: TraceScopeLevelV1::Step,
                        name: step_id.clone(),
                    },
                    inputs_ref: Some(steps_ref.clone()),
                    outputs_ref: None,
                    artifact_ref: Some(activation_log_ref.clone()),
                    decision_context: None,
                    provider: None,
                    error: None,
                    contract_validation: None,
                    governance: None,
                    redaction: None,
                },
            ),
            trace::TraceEvent::StepFinished {
                ts_ms,
                step_id,
                success,
                ..
            } => {
                let step_output_ref = step_artifact_ref(&resolved.run_id, steps, step_id);
                push_trace_v1_event(
                    &mut events,
                    &mut next_id,
                    TraceEventV1 {
                        event_id: String::new(),
                        timestamp: trace::format_iso_utc_ms(*ts_ms),
                        event_type: TraceEventTypeV1::StepEnd,
                        trace_id: trace_id.clone(),
                        run_id: resolved.run_id.clone(),
                        span_id: format!("step:{step_id}"),
                        parent_span_id: Some(root_span_id.clone()),
                        actor: TraceActorV1 {
                            r#type: TraceActorTypeV1::Agent,
                            id: resolved.workflow_id.clone(),
                        },
                        scope: TraceScopeV1 {
                            level: TraceScopeLevelV1::Step,
                            name: step_id.clone(),
                        },
                        inputs_ref: Some(steps_ref.clone()),
                        outputs_ref: step_output_ref.clone().or(Some(steps_ref.clone())),
                        artifact_ref: step_output_ref.or(Some(activation_log_ref.clone())),
                        decision_context: None,
                        provider: None,
                        error: None,
                        contract_validation: None,
                        governance: None,
                        redaction: None,
                    },
                );
                if !success {
                    push_trace_v1_event(
                        &mut events,
                        &mut next_id,
                        TraceEventV1 {
                            event_id: String::new(),
                            timestamp: trace::format_iso_utc_ms(*ts_ms),
                            event_type: TraceEventTypeV1::Error,
                            trace_id: trace_id.clone(),
                            run_id: resolved.run_id.clone(),
                            span_id: format!("step:{step_id}:error"),
                            parent_span_id: Some(format!("step:{step_id}")),
                            actor: TraceActorV1 {
                                r#type: TraceActorTypeV1::System,
                                id: "runtime".to_string(),
                            },
                            scope: TraceScopeV1 {
                                level: TraceScopeLevelV1::Step,
                                name: step_id.clone(),
                            },
                            inputs_ref: Some(steps_ref.clone()),
                            outputs_ref: None,
                            artifact_ref: Some(activation_log_ref.clone()),
                            decision_context: None,
                            provider: None,
                            error: Some(TraceErrorV1 {
                                code: "STEP_FAILURE".to_string(),
                                message: format!("step '{step_id}' finished unsuccessfully"),
                                details: None,
                            }),
                            contract_validation: None,
                            governance: None,
                            redaction: None,
                        },
                    );
                }
            }
            trace::TraceEvent::DelegationPolicyEvaluated {
                ts_ms,
                step_id,
                action_kind,
                target_id,
                decision,
                rule_id,
                ..
            } => {
                let result = if decision.eq_ignore_ascii_case("allowed")
                    || decision.eq_ignore_ascii_case("approved")
                    || decision.eq_ignore_ascii_case("pass")
                {
                    ContractValidationResultV1::Pass
                } else {
                    ContractValidationResultV1::Fail
                };
                push_trace_v1_event(
                    &mut events,
                    &mut next_id,
                    TraceEventV1 {
                        event_id: String::new(),
                        timestamp: trace::format_iso_utc_ms(*ts_ms),
                        event_type: TraceEventTypeV1::ContractValidation,
                        trace_id: trace_id.clone(),
                        run_id: resolved.run_id.clone(),
                        span_id: format!("step:{step_id}:policy"),
                        parent_span_id: Some(format!("step:{step_id}")),
                        actor: TraceActorV1 {
                            r#type: TraceActorTypeV1::System,
                            id: "policy-engine".to_string(),
                        },
                        scope: TraceScopeV1 {
                            level: TraceScopeLevelV1::Step,
                            name: step_id.clone(),
                        },
                        inputs_ref: Some(steps_ref.clone()),
                        outputs_ref: None,
                        artifact_ref: Some(activation_log_ref.clone()),
                        decision_context: None,
                        provider: None,
                        error: None,
                        contract_validation: Some(TraceContractValidationV1 {
                            contract_id: "adl.delegation_policy".to_string(),
                            result,
                            details: Some(json!({
                                "step_id": step_id,
                                "action_kind": action_kind,
                                "target_id": target_id,
                                "decision": decision,
                                "rule_id": rule_id,
                            })),
                        }),
                        governance: None,
                        redaction: None,
                    },
                );
            }
            trace::TraceEvent::DelegationApproved { ts_ms, step_id, .. } => push_trace_v1_event(
                &mut events,
                &mut next_id,
                TraceEventV1 {
                    event_id: String::new(),
                    timestamp: trace::format_iso_utc_ms(*ts_ms),
                    event_type: TraceEventTypeV1::Approval,
                    trace_id: trace_id.clone(),
                    run_id: resolved.run_id.clone(),
                    span_id: format!("step:{step_id}:approval"),
                    parent_span_id: Some(format!("step:{step_id}")),
                    actor: TraceActorV1 {
                        r#type: TraceActorTypeV1::System,
                        id: "policy-engine".to_string(),
                    },
                    scope: TraceScopeV1 {
                        level: TraceScopeLevelV1::Step,
                        name: step_id.clone(),
                    },
                    inputs_ref: Some(steps_ref.clone()),
                    outputs_ref: None,
                    artifact_ref: Some(activation_log_ref.clone()),
                    decision_context: Some(TraceDecisionContextV1 {
                        context: "delegation policy".to_string(),
                        outcome: "approved".to_string(),
                        rationale: None,
                    }),
                    provider: None,
                    error: None,
                    contract_validation: None,
                    governance: None,
                    redaction: None,
                },
            ),
            trace::TraceEvent::DelegationDenied {
                ts_ms,
                step_id,
                action_kind,
                target_id,
                rule_id,
                ..
            } => push_trace_v1_event(
                &mut events,
                &mut next_id,
                TraceEventV1 {
                    event_id: String::new(),
                    timestamp: trace::format_iso_utc_ms(*ts_ms),
                    event_type: TraceEventTypeV1::Rejection,
                    trace_id: trace_id.clone(),
                    run_id: resolved.run_id.clone(),
                    span_id: format!("step:{step_id}:rejection"),
                    parent_span_id: Some(format!("step:{step_id}")),
                    actor: TraceActorV1 {
                        r#type: TraceActorTypeV1::System,
                        id: "policy-engine".to_string(),
                    },
                    scope: TraceScopeV1 {
                        level: TraceScopeLevelV1::Step,
                        name: step_id.clone(),
                    },
                    inputs_ref: Some(steps_ref.clone()),
                    outputs_ref: None,
                    artifact_ref: Some(activation_log_ref.clone()),
                    decision_context: Some(TraceDecisionContextV1 {
                        context: format!("delegation policy {action_kind} -> {target_id}"),
                        outcome: "denied".to_string(),
                        rationale: rule_id.clone(),
                    }),
                    provider: None,
                    error: None,
                    contract_validation: None,
                    governance: None,
                    redaction: None,
                },
            ),
            trace::TraceEvent::RunFailed { ts_ms, message, .. } => push_trace_v1_event(
                &mut events,
                &mut next_id,
                TraceEventV1 {
                    event_id: String::new(),
                    timestamp: trace::format_iso_utc_ms(*ts_ms),
                    event_type: TraceEventTypeV1::Error,
                    trace_id: trace_id.clone(),
                    run_id: resolved.run_id.clone(),
                    span_id: format!("run:{}:error", resolved.run_id),
                    parent_span_id: Some(root_span_id.clone()),
                    actor: TraceActorV1 {
                        r#type: TraceActorTypeV1::System,
                        id: "runtime".to_string(),
                    },
                    scope: TraceScopeV1 {
                        level: TraceScopeLevelV1::Run,
                        name: resolved.workflow_id.clone(),
                    },
                    inputs_ref: Some(run_ref.clone()),
                    outputs_ref: None,
                    artifact_ref: Some(activation_log_ref.clone()),
                    decision_context: None,
                    provider: None,
                    error: Some(TraceErrorV1 {
                        code: "RUN_FAILURE".to_string(),
                        message: message.clone(),
                        details: None,
                    }),
                    contract_validation: None,
                    governance: None,
                    redaction: None,
                },
            ),
            _ => {}
        }
    }

    let run_end_outcome = if status == "success" {
        "success".to_string()
    } else if status == "paused" {
        "paused".to_string()
    } else {
        "failure".to_string()
    };
    push_trace_v1_event(
        &mut events,
        &mut next_id,
        TraceEventV1 {
            event_id: String::new(),
            timestamp: trace::format_iso_utc_ms(end_ms),
            event_type: TraceEventTypeV1::RunEnd,
            trace_id,
            run_id: resolved.run_id.clone(),
            span_id: root_span_id,
            parent_span_id: None,
            actor: TraceActorV1 {
                r#type: TraceActorTypeV1::Agent,
                id: resolved.workflow_id.clone(),
            },
            scope: TraceScopeV1 {
                level: TraceScopeLevelV1::Run,
                name: resolved.workflow_id.clone(),
            },
            inputs_ref: Some(run_ref),
            outputs_ref: Some(steps_ref),
            artifact_ref: Some(activation_log_ref),
            decision_context: Some(TraceDecisionContextV1 {
                context: "run completion".to_string(),
                outcome: run_end_outcome,
                rationale: failure.map(|err| err.to_string()),
            }),
            provider: None,
            error: None,
            contract_validation: None,
            governance: None,
            redaction: None,
        },
    );

    let schema_version = if events.iter().any(|event| {
        matches!(
            event.event_type,
            TraceEventTypeV1::Proposal
                | TraceEventTypeV1::ProposalNormalization
                | TraceEventTypeV1::CapabilityContract
                | TraceEventTypeV1::PolicyInjection
                | TraceEventTypeV1::VisibilityPolicy
                | TraceEventTypeV1::FreedomGateDecision
                | TraceEventTypeV1::ActionSelection
                | TraceEventTypeV1::ActionRejection
                | TraceEventTypeV1::ExecutionResult
                | TraceEventTypeV1::Refusal
                | TraceEventTypeV1::RedactionDecision
        )
    }) {
        "trace.v2"
    } else {
        "trace.v1"
    };
    let envelope = TraceEventEnvelopeV1 {
        schema_version: schema_version.to_string(),
        events,
    };
    validate_trace_event_envelope_v1(&envelope)?;
    Ok(envelope)
}

fn push_trace_v1_event(events: &mut Vec<TraceEventV1>, next_id: &mut u64, mut event: TraceEventV1) {
    event.event_id = format!("trace-v1-{:04}", *next_id);
    *next_id = next_id.saturating_add(1);
    events.push(event);
}

fn artifact_ref(run_id: &str, relative_path: &str) -> String {
    format!("artifacts/{run_id}/{relative_path}")
}

fn step_artifact_ref(run_id: &str, steps: &[StepStateArtifact], step_id: &str) -> Option<String> {
    let rel = steps
        .iter()
        .find(|step| step.step_id == step_id)
        .and_then(|step| step.output_artifact_path.as_deref())?;
    Some(artifact_ref(run_id, rel))
}
