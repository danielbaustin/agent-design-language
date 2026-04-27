use std::collections::BTreeMap;

use crate::acc::{validate_acc_v1, AccDecisionV1, AdlCapabilityContractV1};
use crate::freedom_gate::{FreedomGateToolDecisionEventV1, FreedomGateToolDecisionV1};
use crate::tool_registry::{
    bind_tool_registry_v1, ToolBindingDecisionV1, ToolBindingRequestV1,
    ToolBindingSourceV1::RegistryCompiler, ToolRegistryRejectionCodeV1, ToolRegistryV1,
};
use serde_json::Value as JsonValue;

const GOVERNED_ACTION_ID_UNKNOWN: &str = "action.unknown";
const GOVERNED_ADAPTER_UNKNOWN: &str = "adapter.unknown";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GovernedExecutorSourceV1 {
    RegistryCompiler,
    ModelOutput,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GovernedExecutorActionOutcomeV1 {
    Selected,
    Rejected,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GovernedExecutorActionRecordV1 {
    pub proposal_id: String,
    pub action_id: String,
    pub tool_name: String,
    pub adapter_id: String,
    pub outcome: GovernedExecutorActionOutcomeV1,
    pub reason_code: String,
    pub evidence: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GovernedExecutorResultV1 {
    pub adapter_id: String,
    pub payload: JsonValue,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GovernedExecutorExecutionOutcomeV1 {
    pub selected_actions: Vec<GovernedExecutorActionRecordV1>,
    pub rejected_actions: Vec<GovernedExecutorActionRecordV1>,
    pub execution_result: Option<GovernedExecutorResultV1>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GovernedExecutorInputV1 {
    pub source: GovernedExecutorSourceV1,
    pub action_id: String,
    pub proposal_id: String,
    pub acc: Option<AdlCapabilityContractV1>,
    pub registry: ToolRegistryV1,
    pub arguments: BTreeMap<String, JsonValue>,
    pub gate_decision: FreedomGateToolDecisionEventV1,
}

fn unknown_identity(
    action_id: &str,
    acc: Option<&AdlCapabilityContractV1>,
) -> (String, String, String) {
    let action_id = if action_id.trim().is_empty() {
        GOVERNED_ACTION_ID_UNKNOWN.to_string()
    } else {
        action_id.to_string()
    };
    if let Some(acc) = acc {
        (
            action_id,
            acc.tool.tool_name.clone(),
            acc.tool.adapter_id.clone(),
        )
    } else {
        (
            action_id,
            "tool.unknown".to_string(),
            GOVERNED_ADAPTER_UNKNOWN.to_string(),
        )
    }
}

fn rejected_record(
    proposal_id: String,
    action_id: String,
    tool_name: String,
    adapter_id: String,
    reason: &str,
    evidence: Vec<String>,
) -> GovernedExecutorActionRecordV1 {
    GovernedExecutorActionRecordV1 {
        proposal_id,
        action_id,
        tool_name,
        adapter_id,
        outcome: GovernedExecutorActionOutcomeV1::Rejected,
        reason_code: reason.to_string(),
        evidence,
    }
}

fn selected_record(
    proposal_id: String,
    action_id: String,
    tool_name: String,
    adapter_id: String,
) -> GovernedExecutorActionRecordV1 {
    GovernedExecutorActionRecordV1 {
        proposal_id,
        action_id,
        tool_name,
        adapter_id,
        outcome: GovernedExecutorActionOutcomeV1::Selected,
        reason_code: "selected".to_string(),
        evidence: vec!["governed_execution_allowed".to_string()],
    }
}

fn fixture_execute_safe_read(arguments: &BTreeMap<String, JsonValue>) -> Option<JsonValue> {
    let fixture_id = arguments.get("fixture_id")?.as_str()?;
    if fixture_id.trim().is_empty() {
        return None;
    }
    Some(serde_json::json!({
        "kind": "fixture",
        "action": "read",
        "fixture_id": fixture_id,
        "result": "fixture_read_completed",
    }))
}

fn fixture_execute_adapter(
    adapter_id: &str,
    arguments: &BTreeMap<String, JsonValue>,
) -> Result<JsonValue, &'static str> {
    match adapter_id {
        "adapter.fixture.safe_read.dry_run" => {
            fixture_execute_safe_read(arguments).ok_or("missing_fixture_argument")
        }
        "adapter.fixture.disabled_write.dry_run" => Ok(serde_json::json!({
            "kind": "fixture",
            "action": "local_write",
            "result": "dry_run_not_available",
        })),
        _ => Err("unsupported_fixture_adapter"),
    }
}

fn gate_refusal_reason(
    gate: &FreedomGateToolDecisionEventV1,
) -> Option<(&'static str, Vec<String>)> {
    if matches!(
        (&gate.decision, gate.stopped_before_executor),
        (FreedomGateToolDecisionV1::Allowed, false)
    ) {
        return None;
    }
    match (&gate.decision, gate.stopped_before_executor) {
        (FreedomGateToolDecisionV1::Denied, true) => {
            Some(("freedom_gate_denied", vec![gate.reason_code.clone()]))
        }
        (FreedomGateToolDecisionV1::Deferred, true) => {
            Some(("freedom_gate_deferred", vec![gate.reason_code.clone()]))
        }
        (FreedomGateToolDecisionV1::Challenged, true) => {
            Some(("freedom_gate_challenged", vec![gate.reason_code.clone()]))
        }
        (FreedomGateToolDecisionV1::Escalated, true) => {
            Some(("freedom_gate_escalated", vec![gate.reason_code.clone()]))
        }
        _ => Some((
            "malformed_gate_decision",
            vec![format!(
                "decision {:?} with stopped_before_executor {} is inconsistent",
                gate.decision, gate.stopped_before_executor
            )],
        )),
    }
}

fn map_registry_rejection(code: &ToolRegistryRejectionCodeV1) -> &'static str {
    match code {
        ToolRegistryRejectionCodeV1::UnknownTool => "unregistered_action",
        ToolRegistryRejectionCodeV1::UnregisteredTool => "unregistered_action",
        ToolRegistryRejectionCodeV1::IncompatibleVersion => "unregistered_action",
        ToolRegistryRejectionCodeV1::MismatchedAdapterCapabilities => "unregistered_action",
        ToolRegistryRejectionCodeV1::UnsafeDryRunPosture => "unregistered_action",
        ToolRegistryRejectionCodeV1::InvalidRegistry
        | ToolRegistryRejectionCodeV1::InvalidUts
        | ToolRegistryRejectionCodeV1::ModelDirectExecutionDenied => "unregistered_action",
    }
}

/// Execute one bounded governed action candidate.
///
/// In the governed execution slice, execution is only allowed for:
/// - RegistryCompiler-sourced candidates.
/// - Valid ACC contracts with an allowed decision.
/// - Allowed Freedom Gate outcomes.
/// - Registry-bound adapters that are registered and approved.
/// - Replay-safe, non-destructive, non-exfiltrating actions.
///
/// All unsafe paths return one or more rejected action records.
pub fn execute_governed_action_v1(
    input: &GovernedExecutorInputV1,
) -> GovernedExecutorExecutionOutcomeV1 {
    let mut selected_actions = Vec::new();
    let mut rejected_actions = Vec::new();
    let proposal_id = input.proposal_id.clone();

    let (action_id, tool_name, adapter_id) = unknown_identity(&input.action_id, input.acc.as_ref());

    match input.source {
        GovernedExecutorSourceV1::ModelOutput => {
            rejected_actions.push(rejected_record(
                proposal_id.clone(),
                action_id,
                tool_name,
                adapter_id,
                "model_output_execution_denied",
                vec!["model output cannot bind directly to executor".to_string()],
            ));
            return GovernedExecutorExecutionOutcomeV1 {
                selected_actions,
                rejected_actions,
                execution_result: None,
            };
        }
        GovernedExecutorSourceV1::RegistryCompiler => {}
    }

    let Some(acc) = input.acc.as_ref() else {
        let (action_id, tool_name, adapter_id) = unknown_identity(&input.action_id, None);
        rejected_actions.push(rejected_record(
            proposal_id.clone(),
            action_id,
            tool_name,
            adapter_id,
            "malformed_action",
            vec!["missing_acc_contract".to_string()],
        ));
        return GovernedExecutorExecutionOutcomeV1 {
            selected_actions,
            rejected_actions,
            execution_result: None,
        };
    };

    if let Err(err) = validate_acc_v1(acc) {
        rejected_actions.push(rejected_record(
            proposal_id.clone(),
            action_id,
            tool_name,
            adapter_id,
            "malformed_action",
            err.codes().iter().map(|code| code.to_string()).collect(),
        ));
        return GovernedExecutorExecutionOutcomeV1 {
            selected_actions,
            rejected_actions,
            execution_result: None,
        };
    }

    if acc.decision != AccDecisionV1::Allowed {
        rejected_actions.push(rejected_record(
            proposal_id.clone(),
            action_id,
            tool_name,
            adapter_id,
            "acc_not_allowed",
            vec![format!("acc decision is {:?}", acc.decision)],
        ));
        return GovernedExecutorExecutionOutcomeV1 {
            selected_actions,
            rejected_actions,
            execution_result: None,
        };
    }

    if !acc.execution.approved_for_execution || !acc.execution.dry_run {
        rejected_actions.push(rejected_record(
            proposal_id.clone(),
            action_id,
            tool_name,
            adapter_id,
            "acc_not_execution_ready",
            vec!["execution approval or dry-run posture missing".to_string()],
        ));
        return GovernedExecutorExecutionOutcomeV1 {
            selected_actions,
            rejected_actions,
            execution_result: None,
        };
    }

    if let Some((reason_code, evidence)) = gate_refusal_reason(&input.gate_decision) {
        rejected_actions.push(rejected_record(
            proposal_id.clone(),
            action_id,
            tool_name,
            adapter_id,
            reason_code,
            evidence,
        ));
        return GovernedExecutorExecutionOutcomeV1 {
            selected_actions,
            rejected_actions,
            execution_result: None,
        };
    }

    if !acc.trace_replay.replay_allowed {
        rejected_actions.push(rejected_record(
            proposal_id.clone(),
            action_id,
            tool_name,
            adapter_id,
            "replay_unsafe",
            vec!["replay is not allowed by ACC trace policy".to_string()],
        ));
        return GovernedExecutorExecutionOutcomeV1 {
            selected_actions,
            rejected_actions,
            execution_result: None,
        };
    }

    let side_effect = acc.capability.side_effect_class.as_str();
    if side_effect == "destructive" {
        rejected_actions.push(rejected_record(
            proposal_id.clone(),
            action_id,
            tool_name,
            adapter_id,
            "destructive_action",
            vec!["destructive side effects are refused".to_string()],
        ));
        return GovernedExecutorExecutionOutcomeV1 {
            selected_actions,
            rejected_actions,
            execution_result: None,
        };
    }
    if side_effect == "exfiltration" {
        rejected_actions.push(rejected_record(
            proposal_id.clone(),
            action_id,
            tool_name,
            adapter_id,
            "exfiltrating_action",
            vec!["exfiltration side effects are refused".to_string()],
        ));
        return GovernedExecutorExecutionOutcomeV1 {
            selected_actions,
            rejected_actions,
            execution_result: None,
        };
    }

    let binding_request = ToolBindingRequestV1 {
        source: RegistryCompiler,
        tool_name: acc.tool.tool_name.clone(),
        tool_version: acc.tool.tool_version.clone(),
        adapter_id: acc.tool.adapter_id.clone(),
        dry_run_requested: acc.execution.dry_run,
    };
    let binding = bind_tool_registry_v1(&input.registry, &binding_request);
    if !matches!(binding.decision, ToolBindingDecisionV1::Bound) {
        let reason_code = binding
            .rejection_code
            .as_ref()
            .map(map_registry_rejection)
            .unwrap_or("unregistered_action");
        rejected_actions.push(rejected_record(
            proposal_id.clone(),
            action_id,
            tool_name,
            adapter_id,
            reason_code,
            binding.evidence,
        ));
        return GovernedExecutorExecutionOutcomeV1 {
            selected_actions,
            rejected_actions,
            execution_result: None,
        };
    }

    if acc.actor.actor_id.is_empty() || acc.actor.actor_id.contains('/') {
        rejected_actions.push(rejected_record(
            proposal_id.clone(),
            action_id,
            tool_name,
            adapter_id,
            "malformed_action",
            vec!["actor id invalid for governed execution".to_string()],
        ));
        return GovernedExecutorExecutionOutcomeV1 {
            selected_actions,
            rejected_actions,
            execution_result: None,
        };
    }

    let payload = match fixture_execute_adapter(&acc.tool.adapter_id, &input.arguments) {
        Ok(payload) => payload,
        Err(reason_code) => {
            let reason = if reason_code == "unsupported_fixture_adapter" {
                "unsupported_fixture_adapter"
            } else {
                "malformed_action"
            };
            rejected_actions.push(rejected_record(
                proposal_id.clone(),
                action_id,
                tool_name,
                adapter_id,
                reason,
                vec!["fixture payload could not be evaluated".to_string()],
            ));
            return GovernedExecutorExecutionOutcomeV1 {
                selected_actions,
                rejected_actions,
                execution_result: None,
            };
        }
    };

    let selected = selected_record(proposal_id, action_id, tool_name, adapter_id);
    selected_actions.push(selected);

    GovernedExecutorExecutionOutcomeV1 {
        selected_actions,
        rejected_actions,
        execution_result: Some(GovernedExecutorResultV1 {
            adapter_id: input
                .acc
                .as_ref()
                .expect("acc exists")
                .tool
                .adapter_id
                .clone(),
            payload,
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::freedom_gate::evaluate_tool_candidate_freedom_gate_v1;
    use crate::tool_registry::wp08_tool_registry_v1_fixture;
    use crate::uts_acc_compiler::{
        compile_uts_to_acc_v1, wp09_compiler_input_fixture, wp09_compiler_registry_fixture,
    };

    fn safe_read_input() -> GovernedExecutorInputV1 {
        let input = wp09_compiler_input_fixture("fixture.safe_read");
        let outcome = compile_uts_to_acc_v1(&input);
        let acc = outcome.acc.expect("safe-read fixture should compile");

        let registry = wp09_compiler_registry_fixture();
        let candidate = crate::freedom_gate::FreedomGateToolCandidateV1 {
            candidate_id: "candidate.safe_read".to_string(),
            proposal_id: input.proposal.proposal_id.clone(),
            normalized_proposal_ref: "normalized.proposal".to_string(),
            acc_contract_id: acc.contract_id.clone(),
            policy_evidence_ref: "policy.wp11.fixture".to_string(),
            action_kind: acc.tool.tool_name.clone(),
            risk_class: "low".to_string(),
            operator_actor_id: acc.actor.actor_id.clone(),
            citizen_boundary_ref: "citizen.boundary".to_string(),
            private_argument_digest: "sha256:".to_string() + &"a".repeat(64),
        };
        let gate_context = crate::freedom_gate::FreedomGateToolGateContextV1 {
            policy_decision: "allowed".to_string(),
            requires_operator_review: false,
            requires_human_challenge: false,
            escalation_available: false,
            citizen_action_boundary_intact: true,
            operator_action_boundary_intact: true,
            private_arguments_redacted: true,
        };
        let gate_decision = evaluate_tool_candidate_freedom_gate_v1(&candidate, &gate_context);

        GovernedExecutorInputV1 {
            source: GovernedExecutorSourceV1::RegistryCompiler,
            action_id: "action.safe_read".to_string(),
            proposal_id: input.proposal.proposal_id,
            acc: Some(acc),
            registry,
            arguments: input
                .proposal
                .arguments
                .iter()
                .map(|(key, value)| (key.clone(), value.clone()))
                .collect(),
            gate_decision,
        }
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
    fn wp13_approved_action_is_executed_and_selected() {
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
    }

    #[test]
    fn wp13_refuses_malformed_gate_decision_when_inconsistent() {
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
    fn wp13_refuses_unknown_fixture_adapter() {
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
    fn wp13_direct_model_output_execution_is_refused() {
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
    fn wp13_refuses_denied_freedom_gate_decision() {
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
    fn wp13_refuses_deferred_freedom_gate_decision() {
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
    fn wp13_refuses_challenged_freedom_gate_decision() {
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
    fn wp13_refuses_unregistered_action() {
        let mut input = safe_read_input();
        input
            .acc
            .as_mut()
            .expect("safe-read should be present")
            .tool
            .tool_name = "fixture.unknown".to_string();

        let outcome = execute_governed_action_v1(&input);

        assert_eq!(outcome.selected_actions.len(), 0);
        assert_eq!(
            outcome.rejected_actions[0].reason_code,
            "unregistered_action"
        );
    }

    #[test]
    fn wp13_refuses_destructive_action() {
        let outcome = execute_result_for_side_effect("destructive");
        assert_eq!(
            outcome.rejected_actions[0].reason_code,
            "destructive_action"
        );
    }

    #[test]
    fn wp13_refuses_exfiltrating_action() {
        let outcome = execute_result_for_side_effect("exfiltration");
        assert_eq!(
            outcome.rejected_actions[0].reason_code,
            "exfiltrating_action"
        );
    }

    #[test]
    fn wp13_refuses_replay_unsafe_action() {
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
    fn wp13_refuses_malformed_action() {
        let mut input = safe_read_input();
        input.acc = None;

        let outcome = execute_governed_action_v1(&input);

        assert_eq!(outcome.rejected_actions[0].reason_code, "malformed_action");
    }

    #[test]
    fn wp13_uses_fixture_registry_for_binding() {
        let mut input = safe_read_input();
        let registry = wp08_tool_registry_v1_fixture();
        input.registry = registry;

        let outcome = execute_governed_action_v1(&input);

        assert_eq!(outcome.selected_actions.len(), 1);
        assert_eq!(outcome.selected_actions[0].tool_name, "fixture.safe_read");
    }
}
