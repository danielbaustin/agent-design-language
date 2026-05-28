use super::*;
use crate::acc::AccDecisionV1;
use crate::freedom_gate::{
    evaluate_tool_candidate_freedom_gate_v1, FreedomGateToolDecisionEventV1,
    FreedomGateToolDecisionV1,
};
use crate::tool_registry::wp08_tool_registry_v1_fixture;
use crate::trace::{Trace, TraceEvent};
use serde_json::Value as JsonValue;

fn safe_read_input() -> GovernedExecutorInputV1 {
    fixture_safe_read_input_v1()
}

fn gate_decision_for(
    policy: &str,
    denied: bool,
    deferred: bool,
    challenged: bool,
    reason: &str,
) -> FreedomGateToolDecisionEventV1 {
    let mut context = crate::freedom_gate::FreedomGateToolGateContextV1 {
        policy_decision: policy.to_string(),
        requires_operator_review: false,
        requires_human_challenge: false,
        escalation_available: false,
        citizen_action_boundary_intact: true,
        operator_action_boundary_intact: true,
        private_arguments_redacted: true,
    };
    context.policy_decision = policy.to_string();
    let candidate = crate::freedom_gate::FreedomGateToolCandidateV1 {
        candidate_id: "candidate.safe_read".to_string(),
        proposal_id: "proposal.safe_read".to_string(),
        normalized_proposal_ref: "normalized.proposal".to_string(),
        acc_contract_id: "acc.contract.safe_read".to_string(),
        policy_evidence_ref: "policy.wp11.fixture".to_string(),
        action_kind: "fixture_read".to_string(),
        risk_class: "low".to_string(),
        operator_actor_id: "actor.operator.alice".to_string(),
        citizen_boundary_ref: "citizen.boundary".to_string(),
        private_argument_digest: "sha256:".to_string() + &"b".repeat(64),
    };
    let mut event = evaluate_tool_candidate_freedom_gate_v1(&candidate, &context);
    if denied {
        event.decision = FreedomGateToolDecisionV1::Denied;
        event.reason_code = reason.to_string();
        event.stopped_before_executor = true;
    }
    if deferred {
        event.decision = FreedomGateToolDecisionV1::Deferred;
        event.reason_code = reason.to_string();
        event.stopped_before_executor = true;
    }
    if challenged {
        event.decision = FreedomGateToolDecisionV1::Challenged;
        event.reason_code = reason.to_string();
        event.stopped_before_executor = true;
    }
    event
}

fn execute_result_for_side_effect(side_effect: &str) -> GovernedExecutorExecutionOutcomeV1 {
    let mut input = safe_read_input();
    input
        .acc
        .as_mut()
        .expect("safe-read should be present")
        .capability
        .side_effect_class = side_effect.to_string();
    execute_governed_action_v1(&input)
}

#[test]
fn dangerous_negative_suite_wp13_approved_action_is_executed_and_selected() {
    let input = safe_read_input();
    let proposal_id = input.proposal_id.clone();
    let outcome = execute_governed_action_v1(&input);

    assert_eq!(outcome.selected_actions.len(), 1);
    assert_eq!(outcome.rejected_actions.len(), 0);
    assert_eq!(
        outcome.selected_actions[0].outcome,
        GovernedExecutorActionOutcomeV1::Selected
    );
    assert_eq!(outcome.selected_actions[0].proposal_id, proposal_id);
    assert_eq!(
        outcome
            .execution_result
            .as_ref()
            .expect("should return execution result")
            .payload
            .get("result")
            .and_then(|value| value.as_str()),
        Some("fixture_read_completed")
    );
    assert!(outcome.selected_actions[0]
        .evidence
        .iter()
        .any(|value| value == "gate:candidate.safe_read"));
    assert!(outcome.selected_actions[0]
        .evidence
        .iter()
        .any(|value| value == "policy:policy.wp11.fixture"));
}

#[test]
fn dangerous_negative_suite_wp14_governed_executor_trace_is_emitted_from_production_helper() {
    let input = safe_read_input();
    let mut trace = Trace::new("run-governed", "wf-governed", "0.90.5");

    let outcome = execute_governed_action_with_trace_v1(&input, Some(&mut trace));

    assert_eq!(outcome.selected_actions.len(), 1);
    assert!(trace
        .events
        .iter()
        .any(|event| matches!(event, TraceEvent::GovernedProposalObserved { .. })));
    assert!(trace
        .events
        .iter()
        .any(|event| matches!(event, TraceEvent::GovernedFreedomGateDecided { .. })));
    assert!(trace.events.iter().any(|event| matches!(
        event,
        TraceEvent::GovernedActionSelected { evidence_refs, .. }
            if evidence_refs.iter().any(|value| value == "gate:candidate.safe_read")
                && evidence_refs.iter().any(|value| value == "policy:policy.wp11.fixture")
                && evidence_refs.iter().any(|value| value == "execution:action.safe_read")
    )));
    assert!(trace.events.iter().any(|event| matches!(
        event,
        TraceEvent::GovernedExecutionResultRecorded { evidence_refs, .. }
            if evidence_refs.iter().any(|value| value == "gate:candidate.safe_read")
                && evidence_refs.iter().any(|value| value == "policy:policy.wp11.fixture")
                && evidence_refs.iter().any(|value| value == "execution:action.safe_read")
    )));
    assert!(trace
        .events
        .iter()
        .any(|event| matches!(event, TraceEvent::GovernedRedactionDecisionRecorded { .. })));
}

#[test]
fn dangerous_negative_suite_wp14_governed_rejection_trace_preserves_gate_policy_and_action_lineage()
{
    let mut input = safe_read_input();
    input.source = GovernedExecutorSourceV1::ModelOutput;
    let mut trace = Trace::new("run-governed-rejection", "wf-governed", "0.90.5");

    let outcome = execute_governed_action_with_trace_v1(&input, Some(&mut trace));

    assert_eq!(outcome.selected_actions.len(), 0);
    assert_eq!(outcome.rejected_actions.len(), 1);
    assert!(trace.events.iter().any(|event| matches!(
        event,
        TraceEvent::GovernedActionRejected { evidence_refs, .. }
            if evidence_refs.iter().any(|value| value == "gate:candidate.safe_read")
                && evidence_refs.iter().any(|value| value == "policy:policy.wp11.fixture")
                && evidence_refs.iter().any(|value| value == "action:fixture.safe_read")
                && evidence_refs.iter().any(|value| value == "action_id:action.safe_read")
    )));
}

#[test]
fn dangerous_negative_suite_wp13_refuses_malformed_gate_decision_when_inconsistent() {
    let mut input = safe_read_input();
    input.gate_decision.decision = FreedomGateToolDecisionV1::Allowed;
    input.gate_decision.stopped_before_executor = true;
    let outcome = execute_governed_action_v1(&input);

    assert_eq!(outcome.selected_actions.len(), 0);
    assert_eq!(outcome.rejected_actions.len(), 1);
    assert_eq!(
        outcome.rejected_actions[0].reason_code,
        "malformed_gate_decision"
    );
}

#[test]
fn dangerous_negative_suite_wp13_refuses_unknown_fixture_adapter() {
    let mut input = safe_read_input();
    let mut registry = wp08_tool_registry_v1_fixture();

    let mut unsupported_adapter = registry.adapters[0].clone();
    unsupported_adapter.adapter_id = "adapter.fixture.unsupported.dry_run".to_string();
    unsupported_adapter.capability_id = "capability.fixture.unsupported-read".to_string();
    registry.adapters.push(unsupported_adapter);

    if let Some(tool) = registry
        .tools
        .iter_mut()
        .find(|tool| tool.tool_name == "fixture.safe_read")
    {
        tool.approved_adapter_ids
            .push("adapter.fixture.unsupported.dry_run".to_string());
    } else {
        panic!("safe_read fixture tool not found");
    }

    input.registry = registry;
    let acc = input.acc.as_mut().expect("safe-read should be present");
    acc.tool.adapter_id = "adapter.fixture.unsupported.dry_run".to_string();
    acc.execution.adapter_id = acc.tool.adapter_id.clone();

    let outcome = execute_governed_action_v1(&input);

    assert_eq!(outcome.selected_actions.len(), 0);
    assert_eq!(outcome.execution_result, None);
    assert_eq!(
        outcome.rejected_actions[0].reason_code,
        "unsupported_fixture_adapter"
    );
}

#[test]
fn dangerous_negative_suite_wp13_refuses_gate_without_executor_invocation_ref() {
    let mut input = safe_read_input();
    input.gate_decision.executor_invocation_ref = None;

    let outcome = execute_governed_action_v1(&input);

    assert_eq!(outcome.selected_actions.len(), 0);
    assert_eq!(
        outcome.rejected_actions[0].reason_code,
        "gate_invocation_missing"
    );
}

#[test]
fn dangerous_negative_suite_wp13_direct_model_output_execution_is_refused() {
    let mut input = safe_read_input();
    input.source = GovernedExecutorSourceV1::ModelOutput;

    let outcome = execute_governed_action_v1(&input);

    assert_eq!(outcome.selected_actions.len(), 0);
    assert_eq!(outcome.rejected_actions.len(), 1);
    assert_eq!(
        outcome.rejected_actions[0].reason_code,
        "model_output_execution_denied"
    );
}

#[test]
fn dangerous_negative_suite_wp13_refuses_denied_freedom_gate_decision() {
    let mut input = safe_read_input();
    input.gate_decision = gate_decision_for("denied", true, false, false, "policy_denied");

    let outcome = execute_governed_action_v1(&input);

    assert_eq!(outcome.selected_actions.len(), 0);
    assert_eq!(
        outcome.rejected_actions[0].reason_code,
        "freedom_gate_denied"
    );
}

#[test]
fn dangerous_negative_suite_wp13_refuses_deferred_freedom_gate_decision() {
    let mut input = safe_read_input();
    input.gate_decision = gate_decision_for("allowed", false, true, false, "policy_deferred");

    let outcome = execute_governed_action_v1(&input);

    assert_eq!(outcome.selected_actions.len(), 0);
    assert_eq!(
        outcome.rejected_actions[0].reason_code,
        "freedom_gate_deferred"
    );
}

#[test]
fn dangerous_negative_suite_wp13_refuses_challenged_freedom_gate_decision() {
    let mut input = safe_read_input();
    input.gate_decision = gate_decision_for("allowed", false, false, true, "policy_challenged");

    let outcome = execute_governed_action_v1(&input);

    assert_eq!(outcome.selected_actions.len(), 0);
    assert_eq!(
        outcome.rejected_actions[0].reason_code,
        "freedom_gate_challenged"
    );
}

#[test]
fn dangerous_negative_suite_wp13_refuses_gate_trace_mismatch() {
    let mut input = safe_read_input();
    input.gate_decision.trace_links = vec![
        "proposal:proposal.mismatch".to_string(),
        "acc:acc.mismatch".to_string(),
        "action:action.mismatch".to_string(),
    ];

    let outcome = execute_governed_action_v1(&input);

    assert_eq!(outcome.selected_actions.len(), 0);
    assert_eq!(
        outcome.rejected_actions[0].reason_code,
        "gate_trace_mismatch"
    );
}

#[test]
fn dangerous_negative_suite_wp13_refuses_gate_argument_mismatch() {
    let mut input = safe_read_input();
    input.arguments.insert(
        "fixture_id".to_string(),
        JsonValue::String("modified-fixture".to_string()),
    );

    let outcome = execute_governed_action_v1(&input);

    assert_eq!(outcome.selected_actions.len(), 0);
    assert_eq!(
        outcome.rejected_actions[0].reason_code,
        "gate_argument_mismatch"
    );
}

#[test]
fn dangerous_negative_suite_wp13_refuses_unregistered_action() {
    let mut input = safe_read_input();
    input
        .acc
        .as_mut()
        .expect("safe-read should be present")
        .tool
        .tool_name = "fixture.unknown".to_string();
    input.gate_decision.trace_links = vec![
        format!("proposal:{}", input.proposal_id),
        format!(
            "acc:{}",
            input
                .acc
                .as_ref()
                .expect("safe-read should be present")
                .contract_id
        ),
        "action:fixture.unknown".to_string(),
    ];

    let outcome = execute_governed_action_v1(&input);

    assert_eq!(outcome.selected_actions.len(), 0);
    assert_eq!(
        outcome.rejected_actions[0].reason_code,
        "unregistered_action"
    );
}

#[test]
fn dangerous_negative_suite_wp13_refuses_destructive_action() {
    let outcome = execute_result_for_side_effect("destructive");
    assert_eq!(
        outcome.rejected_actions[0].reason_code,
        "destructive_action"
    );
}

#[test]
fn dangerous_negative_suite_wp15_refuses_process_action() {
    let outcome = execute_result_for_side_effect("process");
    assert_eq!(outcome.rejected_actions[0].reason_code, "process_action");
}

#[test]
fn dangerous_negative_suite_wp15_refuses_network_action() {
    let outcome = execute_result_for_side_effect("network");
    assert_eq!(outcome.rejected_actions[0].reason_code, "network_action");
}

#[test]
fn dangerous_negative_suite_wp13_refuses_exfiltrating_action() {
    let outcome = execute_result_for_side_effect("exfiltration");
    assert_eq!(
        outcome.rejected_actions[0].reason_code,
        "exfiltrating_action"
    );
}

#[test]
fn dangerous_negative_suite_wp13_refuses_replay_unsafe_action() {
    let mut input = safe_read_input();
    input
        .acc
        .as_mut()
        .expect("safe-read should be present")
        .trace_replay
        .replay_allowed = false;

    let outcome = execute_governed_action_v1(&input);

    assert_eq!(outcome.selected_actions.len(), 0);
    assert_eq!(outcome.rejected_actions[0].reason_code, "replay_unsafe");
}

#[test]
fn dangerous_negative_suite_wp13_refuses_malformed_action() {
    let mut input = safe_read_input();
    input.acc = None;

    let outcome = execute_governed_action_v1(&input);

    assert_eq!(outcome.rejected_actions[0].reason_code, "malformed_action");
}

#[test]
fn dangerous_negative_suite_wp13_uses_fixture_registry_for_binding() {
    let mut input = safe_read_input();
    let registry = wp08_tool_registry_v1_fixture();
    input.registry = registry;

    let outcome = execute_governed_action_v1(&input);

    assert_eq!(outcome.selected_actions.len(), 1);
    assert_eq!(outcome.selected_actions[0].tool_name, "fixture.safe_read");
}

#[test]
fn dangerous_negative_suite_wp14_trace_refuses_missing_acc_contract() {
    let mut input = safe_read_input();
    input.acc = None;
    let mut trace = Trace::new("run-governed-no-acc", "wf-governed", "0.90.5");

    let outcome = execute_governed_action_with_trace_v1(&input, Some(&mut trace));

    assert_eq!(outcome.rejected_actions[0].reason_code, "malformed_action");
    assert!(trace
        .events
        .iter()
        .any(|event| matches!(event, TraceEvent::GovernedActionRejected { .. })));
}

#[test]
fn dangerous_negative_suite_wp14_trace_refuses_acc_validation_and_decision() {
    let mut input = safe_read_input();
    input
        .acc
        .as_mut()
        .expect("safe-read should be present")
        .tool
        .tool_name = "".to_string();
    let mut invalid_trace = Trace::new("run-governed-invalid-acc", "wf-governed", "0.90.5");
    let outcome = execute_governed_action_with_trace_v1(&input, Some(&mut invalid_trace));
    assert_eq!(outcome.rejected_actions[0].reason_code, "malformed_action");

    let mut allowed_input = safe_read_input();
    allowed_input
        .acc
        .as_mut()
        .expect("safe-read should be present")
        .decision = AccDecisionV1::Denied;
    let mut denied_trace = Trace::new("run-governed-decision", "wf-governed", "0.90.5");
    let outcome = execute_governed_action_with_trace_v1(&allowed_input, Some(&mut denied_trace));
    assert!(matches!(
        outcome.rejected_actions[0].reason_code.as_str(),
        "acc_not_allowed" | "malformed_action"
    ));
}

#[test]
fn dangerous_negative_suite_wp14_trace_refuses_execution_readiness_and_trace_integrity() {
    let mut execution_input = safe_read_input();
    execution_input
        .acc
        .as_mut()
        .expect("safe-read should be present")
        .execution
        .approved_for_execution = false;
    let mut execution_trace = Trace::new("run-governed-readiness", "wf-governed", "0.90.5");
    let outcome =
        execute_governed_action_with_trace_v1(&execution_input, Some(&mut execution_trace));
    assert!(matches!(
        outcome.rejected_actions[0].reason_code.as_str(),
        "acc_not_execution_ready" | "malformed_action"
    ));

    let mut gate_input = safe_read_input();
    gate_input.gate_decision = gate_decision_for("denied", true, false, false, "policy_denied");
    let mut gate_trace = Trace::new("run-governed-gate", "wf-governed", "0.90.5");
    let outcome = execute_governed_action_with_trace_v1(&gate_input, Some(&mut gate_trace));
    assert_eq!(
        outcome.rejected_actions[0].reason_code,
        "freedom_gate_denied"
    );

    let mut trace_input = safe_read_input();
    trace_input.gate_decision.trace_links = vec!["proposal:mismatch".to_string()];
    let mut trace_mismatch = Trace::new("run-governed-trace-mismatch", "wf-governed", "0.90.5");
    let outcome = execute_governed_action_with_trace_v1(&trace_input, Some(&mut trace_mismatch));
    assert_eq!(
        outcome.rejected_actions[0].reason_code,
        "gate_trace_mismatch"
    );

    let mut digest_input = safe_read_input();
    digest_input.arguments.insert(
        "fixture_id".to_string(),
        JsonValue::String("different".to_string()),
    );
    let mut digest_trace = Trace::new("run-governed-digest", "wf-governed", "0.90.5");
    let outcome = execute_governed_action_with_trace_v1(&digest_input, Some(&mut digest_trace));
    assert_eq!(
        outcome.rejected_actions[0].reason_code,
        "gate_argument_mismatch"
    );
}

#[test]
fn dangerous_negative_suite_wp14_trace_refuses_actor_and_registry_paths() {
    let mut replay_input = safe_read_input();
    replay_input
        .acc
        .as_mut()
        .expect("safe-read should be present")
        .trace_replay
        .replay_allowed = false;
    let mut replay_trace = Trace::new("run-governed-replay", "wf-governed", "0.90.5");
    let outcome = execute_governed_action_with_trace_v1(&replay_input, Some(&mut replay_trace));
    assert_eq!(outcome.rejected_actions[0].reason_code, "replay_unsafe");

    let mut actor_input = safe_read_input();
    actor_input
        .acc
        .as_mut()
        .expect("safe-read should be present")
        .actor
        .actor_id = "".to_string();
    let mut actor_trace = Trace::new("run-governed-actor", "wf-governed", "0.90.5");
    let outcome = execute_governed_action_with_trace_v1(&actor_input, Some(&mut actor_trace));
    assert_eq!(outcome.rejected_actions[0].reason_code, "malformed_action");

    let mut registry_input = safe_read_input();
    let acc = registry_input
        .acc
        .as_mut()
        .expect("safe-read should be present");
    acc.tool.tool_name = "fixture.unknown".to_string();
    acc.execution.adapter_id = "adapter.fixture.unknown.dry_run".to_string();
    registry_input.gate_decision.trace_links = vec![
        format!("proposal:{}", registry_input.proposal_id),
        format!("acc:{}", acc.contract_id),
        format!("action:fixture.unknown"),
    ];
    registry_input
        .acc
        .as_mut()
        .expect("safe-read should be present")
        .tool
        .adapter_id = "adapter.fixture.unknown.dry_run".to_string();
    let mut registry_trace = Trace::new("run-governed-registry", "wf-governed", "0.90.5");
    let outcome = execute_governed_action_with_trace_v1(&registry_input, Some(&mut registry_trace));
    assert_eq!(
        outcome.rejected_actions[0].reason_code,
        "unregistered_action"
    );
}

#[test]
fn dangerous_negative_suite_wp14_trace_refuses_invocation_and_adapter_payload_paths() {
    let mut invocation_input = safe_read_input();
    invocation_input.gate_decision.executor_invocation_ref = None;
    let mut invocation_trace = Trace::new("run-governed-invocation", "wf-governed", "0.90.5");
    let outcome =
        execute_governed_action_with_trace_v1(&invocation_input, Some(&mut invocation_trace));
    assert_eq!(
        outcome.rejected_actions[0].reason_code,
        "gate_invocation_missing"
    );

    let mut unsupported_input = safe_read_input();
    let mut registry = wp08_tool_registry_v1_fixture();
    let mut unsupported_adapter = registry.adapters[0].clone();
    unsupported_adapter.adapter_id = "adapter.fixture.unsupported.dry_run".to_string();
    unsupported_adapter.capability_id = "capability.fixture.unsupported-read".to_string();
    registry.adapters.push(unsupported_adapter);
    if let Some(tool) = registry
        .tools
        .iter_mut()
        .find(|tool| tool.tool_name == "fixture.safe_read")
    {
        tool.approved_adapter_ids
            .push("adapter.fixture.unsupported.dry_run".to_string());
    } else {
        panic!("safe_read fixture tool not found");
    }

    unsupported_input.registry = registry;
    unsupported_input
        .acc
        .as_mut()
        .expect("safe-read should be present")
        .tool
        .adapter_id = "adapter.fixture.unsupported.dry_run".to_string();
    unsupported_input
        .acc
        .as_mut()
        .expect("safe-read should be present")
        .execution
        .adapter_id = "adapter.fixture.unsupported.dry_run".to_string();
    let mut unsupported_trace =
        Trace::new("run-governed-unsupported-adapter", "wf-governed", "0.90.5");
    let outcome =
        execute_governed_action_with_trace_v1(&unsupported_input, Some(&mut unsupported_trace));
    assert_eq!(
        outcome.rejected_actions[0].reason_code,
        "unsupported_fixture_adapter"
    );

    let mut malformed_input = safe_read_input();
    malformed_input.arguments.remove("fixture_id");
    let mut malformed_trace = Trace::new("run-governed-missing-argument", "wf-governed", "0.90.5");
    let outcome =
        execute_governed_action_with_trace_v1(&malformed_input, Some(&mut malformed_trace));
    assert_eq!(
        outcome.rejected_actions[0].reason_code,
        "gate_argument_mismatch"
    );
}
