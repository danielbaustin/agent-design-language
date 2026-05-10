use std::collections::BTreeMap;

use serde_json::Value as JsonValue;
use sha2::{Digest, Sha256};

use crate::acc::{AccDecisionV1, AccGrantStatusV1, AdlCapabilityContractV1};
use crate::freedom_gate::{
    evaluate_tool_candidate_freedom_gate_v1, FreedomGateToolCandidateV1, FreedomGateToolDecisionV1,
    FreedomGateToolGateContextV1,
};
use crate::governed_executor::{
    execute_governed_action_with_trace_v1, GovernedExecutorExecutionOutcomeV1,
    GovernedExecutorInputV1, GovernedExecutorSourceV1,
};
use crate::trace::Trace;
use crate::uts::UtsSideEffectClassV1;
use crate::uts_acc_compiler::{
    compile_uts_to_acc_v1, wp09_compiler_input_fixture, UtsAccCompilerDecisionV1,
    UtsAccCompilerInputV1, UtsAccCompilerOutcomeV1, UtsAccCompilerRejectionCodeV1,
    UtsAccDelegationContextV1,
};

use super::constants::{
    FLAGSHIP_ALLOWED_READ_CASE_PATH, FLAGSHIP_ALLOWED_READ_RUN_ID,
    FLAGSHIP_DELEGATED_LOCAL_WRITE_CASE_PATH, FLAGSHIP_DELEGATED_LOCAL_WRITE_RUN_ID,
    FLAGSHIP_DENIED_EXFILTRATION_CASE_PATH, FLAGSHIP_DENIED_EXFILTRATION_RUN_ID,
    FLAGSHIP_DENIED_LOW_AUTHORITY_CASE_PATH, FLAGSHIP_TRACE_WORKFLOW_ID,
};
use super::trace_support::{
    proposal_redaction_ref_for_run, result_redaction_ref_for_run, trace_ref_for_run,
};
use super::*;

pub(crate) struct RuntimeV2GovernedToolsFlagshipCaseArtifacts {
    pub case: RuntimeV2GovernedToolsFlagshipCase,
    pub trace_run_id: Option<String>,
    pub trace: Option<Trace>,
}

pub(crate) fn allowed_read_case() -> Result<RuntimeV2GovernedToolsFlagshipCaseArtifacts> {
    let input = wp09_compiler_input_fixture("fixture.safe_read");
    let outcome = compile_uts_to_acc_v1(&input);
    let acc = outcome
        .acc
        .clone()
        .ok_or_else(|| anyhow!("safe read fixture must compile to ACC"))?;
    let governed_input = governed_input_from_acc(
        &input,
        &acc,
        "allowed_read",
        "policy.allowed_read",
        "normalized.allowed_read",
        "low",
        false,
    );
    let mut trace = governed_trace(FLAGSHIP_ALLOWED_READ_RUN_ID);
    let execution = execute_governed_action_with_trace_v1(&governed_input, Some(&mut trace));
    let case = RuntimeV2GovernedToolsFlagshipCase {
        schema_version: RUNTIME_V2_GOVERNED_TOOLS_FLAGSHIP_CASE_SCHEMA.to_string(),
        case_id: "wp18.allowed_read".to_string(),
        case_kind: "allowed_read".to_string(),
        artifact_path: FLAGSHIP_ALLOWED_READ_CASE_PATH.to_string(),
        proposal_id: input.proposal.proposal_id.clone(),
        tool_name: input.proposal.tool_name.clone(),
        proposal_humility_visible: true,
        compiler_decision: compiler_decision_label(&outcome).to_string(),
        compiler_rejection_code: compiler_rejection_code_label(&outcome),
        acc_decision: Some(acc_decision_label(&acc.decision).to_string()),
        policy_decision: "allowed".to_string(),
        gate_decision: Some(gate_decision_label(&governed_input.gate_decision.decision).to_string()),
        gate_reason_code: Some(governed_input.gate_decision.reason_code.clone()),
        executor_outcome: execution_outcome_label(&execution).to_string(),
        executor_reason_code: first_rejection_reason(&execution),
        trace_ref: Some(trace_ref_for_run(FLAGSHIP_ALLOWED_READ_RUN_ID)),
        proposal_redaction_ref: Some(proposal_redaction_ref_for_run(
            FLAGSHIP_ALLOWED_READ_RUN_ID,
        )),
        result_redaction_ref: Some(result_redaction_ref_for_run(
            FLAGSHIP_ALLOWED_READ_RUN_ID,
        )),
        reviewer_visible_outcome:
            "ACC-backed fixture.safe_read reached allowed gate and emitted a redacted execution result."
                .to_string(),
        public_redaction_outcome:
            "Only redacted argument and result references are retained; no raw prompt or tool payload is published."
                .to_string(),
        claim_boundary:
            "Allowed read case proves fixture-backed execution only. It does not grant arbitrary file, process, network, or shell authority."
                .to_string(),
    };
    case.validate()?;
    Ok(RuntimeV2GovernedToolsFlagshipCaseArtifacts {
        case,
        trace_run_id: Some(FLAGSHIP_ALLOWED_READ_RUN_ID.to_string()),
        trace: Some(trace),
    })
}

pub(crate) fn delegated_local_write_case() -> Result<RuntimeV2GovernedToolsFlagshipCaseArtifacts> {
    let mut input = wp09_compiler_input_fixture("fixture.local_write");
    input.policy_context.grant_status = AccGrantStatusV1::Delegated;
    input.policy_context.execution_approved = false;
    input.policy_context.delegation = Some(UtsAccDelegationContextV1 {
        delegation_id: "delegation.wp18.local_write".to_string(),
        grantor_actor_id: "actor.operator.alice".to_string(),
        delegate_actor_id: "actor.operator.alice".to_string(),
        depth: 1,
    });
    let outcome = compile_uts_to_acc_v1(&input);
    let acc = outcome
        .acc
        .clone()
        .ok_or_else(|| anyhow!("delegated local write fixture must compile to ACC"))?;
    let governed_input = governed_input_from_acc(
        &input,
        &acc,
        "delegated_local_write",
        "policy.local_write",
        "normalized.local_write",
        "medium",
        true,
    );
    let mut trace = governed_trace(FLAGSHIP_DELEGATED_LOCAL_WRITE_RUN_ID);
    let execution = execute_governed_action_with_trace_v1(&governed_input, Some(&mut trace));
    let case = RuntimeV2GovernedToolsFlagshipCase {
        schema_version: RUNTIME_V2_GOVERNED_TOOLS_FLAGSHIP_CASE_SCHEMA.to_string(),
        case_id: "wp18.delegated_local_write".to_string(),
        case_kind: "delegated_local_write".to_string(),
        artifact_path: FLAGSHIP_DELEGATED_LOCAL_WRITE_CASE_PATH.to_string(),
        proposal_id: input.proposal.proposal_id.clone(),
        tool_name: input.proposal.tool_name.clone(),
        proposal_humility_visible: true,
        compiler_decision: compiler_decision_label(&outcome).to_string(),
        compiler_rejection_code: compiler_rejection_code_label(&outcome),
        acc_decision: Some(acc_decision_label(&acc.decision).to_string()),
        policy_decision: "allowed".to_string(),
        gate_decision: Some(gate_decision_label(&governed_input.gate_decision.decision).to_string()),
        gate_reason_code: Some(governed_input.gate_decision.reason_code.clone()),
        executor_outcome: execution_outcome_label(&execution).to_string(),
        executor_reason_code: first_rejection_reason(&execution),
        trace_ref: Some(trace_ref_for_run(FLAGSHIP_DELEGATED_LOCAL_WRITE_RUN_ID)),
        proposal_redaction_ref: Some(proposal_redaction_ref_for_run(
            FLAGSHIP_DELEGATED_LOCAL_WRITE_RUN_ID,
        )),
        result_redaction_ref: Some(result_redaction_ref_for_run(
            FLAGSHIP_DELEGATED_LOCAL_WRITE_RUN_ID,
        )),
        reviewer_visible_outcome:
            "Delegated local write compiled to a delegated ACC, preserved deferred-review context for operators, and then still failed closed with an acc_not_allowed refusal before any autonomous write result."
                .to_string(),
        public_redaction_outcome:
            "Public evidence records the deferred-review context, bounded acc_not_allowed refusal record, and redacted proposal/result digests, not writable arguments or local filesystem details."
                .to_string(),
        claim_boundary:
            "Delegated local-write case proves that delegation and review stay visible while delegated ACC still fails closed before autonomous write execution. It does not claim deferred review alone grants write authority."
                .to_string(),
    };
    case.validate()?;
    Ok(RuntimeV2GovernedToolsFlagshipCaseArtifacts {
        case,
        trace_run_id: Some(FLAGSHIP_DELEGATED_LOCAL_WRITE_RUN_ID.to_string()),
        trace: Some(trace),
    })
}

pub(crate) fn denied_low_authority_case() -> Result<RuntimeV2GovernedToolsFlagshipCaseArtifacts> {
    let mut input = wp09_compiler_input_fixture("fixture.local_write");
    input.policy_context.allowed_side_effects = vec![UtsSideEffectClassV1::Read];
    input.policy_context.allowed_resource_scopes = vec!["local-readonly".to_string()];
    let outcome = compile_uts_to_acc_v1(&input);
    let case = RuntimeV2GovernedToolsFlagshipCase {
        schema_version: RUNTIME_V2_GOVERNED_TOOLS_FLAGSHIP_CASE_SCHEMA.to_string(),
        case_id: "wp18.denied_low_authority".to_string(),
        case_kind: "denied_low_authority".to_string(),
        artifact_path: FLAGSHIP_DENIED_LOW_AUTHORITY_CASE_PATH.to_string(),
        proposal_id: input.proposal.proposal_id.clone(),
        tool_name: input.proposal.tool_name.clone(),
        proposal_humility_visible: true,
        compiler_decision: compiler_decision_label(&outcome).to_string(),
        compiler_rejection_code: compiler_rejection_code_label(&outcome),
        acc_decision: None,
        policy_decision: "rejected_before_gate".to_string(),
        gate_decision: None,
        gate_reason_code: None,
        executor_outcome: "not_invoked".to_string(),
        executor_reason_code: None,
        trace_ref: None,
        proposal_redaction_ref: None,
        result_redaction_ref: None,
        reviewer_visible_outcome:
            "Low-authority local write failed during compiler policy evaluation before ACC emission or gate mediation."
                .to_string(),
        public_redaction_outcome:
            "Public evidence records only the fail-closed rejection class and does not expose writable arguments or hidden authority context."
                .to_string(),
        claim_boundary:
            "Low-authority case proves that insufficient standing stops the action before gate or execution. It does not claim hidden override paths."
                .to_string(),
    };
    case.validate()?;
    Ok(RuntimeV2GovernedToolsFlagshipCaseArtifacts {
        case,
        trace_run_id: None,
        trace: None,
    })
}

pub(crate) fn denied_exfiltration_case() -> Result<RuntimeV2GovernedToolsFlagshipCaseArtifacts> {
    let mut input = wp09_compiler_input_fixture("fixture.exfiltrate");
    input.policy_context.allowed_side_effects = vec![UtsSideEffectClassV1::Exfiltration];
    input.policy_context.allowed_resource_scopes = vec!["protected-prompt".to_string()];
    input.policy_context.allow_sensitive_data = true;
    let outcome = compile_uts_to_acc_v1(&input);
    let acc = outcome.acc.clone().ok_or_else(|| {
        anyhow!("exfiltration fixture must compile to ACC for fail-closed execution proof")
    })?;
    let governed_input = governed_input_from_acc(
        &input,
        &acc,
        "denied_exfiltration",
        "policy.exfiltration",
        "normalized.exfiltration",
        "low",
        false,
    );
    let mut trace = governed_trace(FLAGSHIP_DENIED_EXFILTRATION_RUN_ID);
    let execution = execute_governed_action_with_trace_v1(&governed_input, Some(&mut trace));
    let case = RuntimeV2GovernedToolsFlagshipCase {
        schema_version: RUNTIME_V2_GOVERNED_TOOLS_FLAGSHIP_CASE_SCHEMA.to_string(),
        case_id: "wp18.denied_exfiltration".to_string(),
        case_kind: "denied_exfiltration".to_string(),
        artifact_path: FLAGSHIP_DENIED_EXFILTRATION_CASE_PATH.to_string(),
        proposal_id: input.proposal.proposal_id.clone(),
        tool_name: input.proposal.tool_name.clone(),
        proposal_humility_visible: true,
        compiler_decision: compiler_decision_label(&outcome).to_string(),
        compiler_rejection_code: compiler_rejection_code_label(&outcome),
        acc_decision: Some(acc_decision_label(&acc.decision).to_string()),
        policy_decision: "allowed".to_string(),
        gate_decision: Some(gate_decision_label(&governed_input.gate_decision.decision).to_string()),
        gate_reason_code: Some(governed_input.gate_decision.reason_code.clone()),
        executor_outcome: execution_outcome_label(&execution).to_string(),
        executor_reason_code: first_rejection_reason(&execution),
        trace_ref: Some(trace_ref_for_run(FLAGSHIP_DENIED_EXFILTRATION_RUN_ID)),
        proposal_redaction_ref: Some(proposal_redaction_ref_for_run(
            FLAGSHIP_DENIED_EXFILTRATION_RUN_ID,
        )),
        result_redaction_ref: Some(result_redaction_ref_for_run(
            FLAGSHIP_DENIED_EXFILTRATION_RUN_ID,
        )),
        reviewer_visible_outcome:
            "Exfiltration proposal reached gate review context but the executor still failed closed and recorded a redacted refusal."
                .to_string(),
        public_redaction_outcome:
            "Public evidence preserves the fail-closed exfiltration denial with digest-only redaction and no raw private payloads."
                .to_string(),
        claim_boundary:
            "Denied exfiltration case proves executor-level fail-closed refusal after mediated review context. It does not authorize data export."
                .to_string(),
    };
    case.validate()?;
    Ok(RuntimeV2GovernedToolsFlagshipCaseArtifacts {
        case,
        trace_run_id: Some(FLAGSHIP_DENIED_EXFILTRATION_RUN_ID.to_string()),
        trace: Some(trace),
    })
}

fn governed_input_from_acc(
    input: &UtsAccCompilerInputV1,
    acc: &AdlCapabilityContractV1,
    case_suffix: &str,
    policy_evidence_ref: &str,
    normalized_proposal_ref: &str,
    risk_class: &str,
    requires_operator_review: bool,
) -> GovernedExecutorInputV1 {
    let arguments = input.proposal.arguments.clone();
    let candidate = FreedomGateToolCandidateV1 {
        candidate_id: format!("candidate.{case_suffix}"),
        proposal_id: input.proposal.proposal_id.clone(),
        normalized_proposal_ref: normalized_proposal_ref.to_string(),
        acc_contract_id: acc.contract_id.clone(),
        policy_evidence_ref: policy_evidence_ref.to_string(),
        action_kind: acc.tool.tool_name.clone(),
        risk_class: risk_class.to_string(),
        operator_actor_id: acc.actor.actor_id.clone(),
        citizen_boundary_ref: "citizen.boundary.wp18".to_string(),
        private_argument_digest: compute_private_argument_digest(&arguments),
    };
    let gate_context = FreedomGateToolGateContextV1 {
        policy_decision: "allowed".to_string(),
        requires_operator_review,
        requires_human_challenge: false,
        escalation_available: false,
        citizen_action_boundary_intact: true,
        operator_action_boundary_intact: true,
        private_arguments_redacted: true,
    };
    GovernedExecutorInputV1 {
        source: GovernedExecutorSourceV1::RegistryCompiler,
        action_id: format!("action.{case_suffix}"),
        proposal_id: input.proposal.proposal_id.clone(),
        acc: Some(acc.clone()),
        registry: input.registry.clone(),
        arguments,
        gate_decision: evaluate_tool_candidate_freedom_gate_v1(&candidate, &gate_context),
    }
}

fn governed_trace(run_id: &str) -> Trace {
    Trace::new(
        run_id.to_string(),
        FLAGSHIP_TRACE_WORKFLOW_ID.to_string(),
        "0.90.5".to_string(),
    )
}

fn compiler_decision_label(outcome: &UtsAccCompilerOutcomeV1) -> &'static str {
    match outcome.decision {
        UtsAccCompilerDecisionV1::AccEmitted => "acc_emitted",
        UtsAccCompilerDecisionV1::RejectionEmitted => "rejection_emitted",
    }
}

fn compiler_rejection_code_label(outcome: &UtsAccCompilerOutcomeV1) -> Option<String> {
    outcome
        .rejection
        .as_ref()
        .map(|rejection| compiler_rejection_code_name(&rejection.code).to_string())
}

fn compiler_rejection_code_name(code: &UtsAccCompilerRejectionCodeV1) -> &'static str {
    match code {
        UtsAccCompilerRejectionCodeV1::InvalidUts => "invalid_uts",
        UtsAccCompilerRejectionCodeV1::InvalidProposal => "invalid_proposal",
        UtsAccCompilerRejectionCodeV1::RegistryBindingRejected => "registry_binding_rejected",
        UtsAccCompilerRejectionCodeV1::AmbiguousProposal => "ambiguous_proposal",
        UtsAccCompilerRejectionCodeV1::UnsatisfiableAuthority => "unsatisfiable_authority",
        UtsAccCompilerRejectionCodeV1::ResourceConstraintUnsatisfied => {
            "resource_constraint_unsatisfied"
        }
        UtsAccCompilerRejectionCodeV1::PrivacyConstraintUnsatisfied => {
            "privacy_constraint_unsatisfied"
        }
        UtsAccCompilerRejectionCodeV1::VisibilityConstraintUnsatisfied => {
            "visibility_constraint_unsatisfied"
        }
        UtsAccCompilerRejectionCodeV1::ReplayConstraintUnsatisfied => {
            "replay_constraint_unsatisfied"
        }
        UtsAccCompilerRejectionCodeV1::ExecutionConstraintUnsatisfied => {
            "execution_constraint_unsatisfied"
        }
    }
}

fn acc_decision_label(decision: &AccDecisionV1) -> &'static str {
    match decision {
        AccDecisionV1::Allowed => "allowed",
        AccDecisionV1::Denied => "denied",
        AccDecisionV1::Delegated => "delegated",
        AccDecisionV1::Revoked => "revoked",
    }
}

fn gate_decision_label(decision: &FreedomGateToolDecisionV1) -> &'static str {
    match decision {
        FreedomGateToolDecisionV1::Allowed => "allowed",
        FreedomGateToolDecisionV1::Denied => "denied",
        FreedomGateToolDecisionV1::Deferred => "deferred",
        FreedomGateToolDecisionV1::Challenged => "challenged",
        FreedomGateToolDecisionV1::Escalated => "escalated",
    }
}

fn execution_outcome_label(outcome: &GovernedExecutorExecutionOutcomeV1) -> &'static str {
    if outcome.execution_result.is_some() {
        "executed"
    } else if first_rejection_reason(outcome).as_deref() == Some("freedom_gate_deferred") {
        "not_invoked"
    } else {
        "refused"
    }
}

fn first_rejection_reason(outcome: &GovernedExecutorExecutionOutcomeV1) -> Option<String> {
    outcome
        .rejected_actions
        .first()
        .map(|record| record.reason_code.clone())
}

fn compute_private_argument_digest(arguments: &BTreeMap<String, JsonValue>) -> String {
    let canonical_arguments = serde_json::to_string(arguments)
        .expect("governed-tools flagship arguments should serialize");
    format!(
        "sha256:{:x}",
        Sha256::digest(canonical_arguments.as_bytes())
    )
}
