//! Deterministic trace-event stream used across execution and replay tooling.
//!
//! `Trace` is the main in-memory event log for runtime execution, while
//! `TraceEvent` captures concrete lifecycle, delegation, and step transitions
//! used for artifact replay and audit.

use std::collections::HashMap;
use std::time::Instant;

use crate::adl::DelegationSpec;
use crate::execute::{ExecutionBoundary, RuntimeLifecyclePhase};

/// In-memory execution trace captured during runtime operation.
///
/// A `Trace` stores high-level lifecycle markers and step-level telemetry in an
/// append-only sequence, including timings for replay-friendly review.
#[derive(Debug, Clone)]
pub struct Trace {
    pub run_id: String,
    pub workflow_id: String,
    pub version: String,
    pub events: Vec<TraceEvent>,
    run_started_ms: u128,
    run_started_instant: Instant,
    step_started_ms: HashMap<String, u128>,
    delegation_ids: HashMap<String, String>,
    next_delegation_counter: u64,
}

#[derive(Debug, Clone)]
/// Canonical trace event model emitted by runtime execution.
///
/// Variants are intentionally stable and replay-oriented. Additional variants
/// should preserve naming conventions and include bounded human-readable fields.
pub enum TraceEvent {
    LifecyclePhaseEntered {
        ts_ms: u128,
        elapsed_ms: u128,
        phase: RuntimeLifecyclePhase,
    },
    ExecutionBoundaryCrossed {
        ts_ms: u128,
        elapsed_ms: u128,
        boundary: ExecutionBoundary,
        state: String,
    },
    GovernedProposalObserved {
        ts_ms: u128,
        elapsed_ms: u128,
        proposal_id: String,
        tool_name: String,
        redacted_arguments_ref: String,
    },
    GovernedProposalNormalized {
        ts_ms: u128,
        elapsed_ms: u128,
        proposal_id: String,
        normalized_proposal_ref: String,
        redacted_arguments_ref: String,
    },
    GovernedAccConstructed {
        ts_ms: u128,
        elapsed_ms: u128,
        proposal_id: String,
        acc_contract_id: String,
        replay_posture: String,
    },
    GovernedPolicyInjected {
        ts_ms: u128,
        elapsed_ms: u128,
        proposal_id: String,
        policy_evidence_ref: String,
        outcome: String,
    },
    GovernedVisibilityResolved {
        ts_ms: u128,
        elapsed_ms: u128,
        proposal_id: String,
        actor_view: String,
        operator_view: String,
        reviewer_view: String,
        public_report_view: String,
        observatory_projection: String,
    },
    GovernedFreedomGateDecided {
        ts_ms: u128,
        elapsed_ms: u128,
        proposal_id: String,
        candidate_id: String,
        decision: String,
        reason_code: String,
        boundary: String,
        redaction_summary: String,
    },
    GovernedActionSelected {
        ts_ms: u128,
        elapsed_ms: u128,
        proposal_id: String,
        action_id: String,
        tool_name: String,
        adapter_id: String,
        evidence_refs: Vec<String>,
    },
    GovernedActionRejected {
        ts_ms: u128,
        elapsed_ms: u128,
        proposal_id: String,
        action_id: String,
        tool_name: String,
        adapter_id: String,
        reason_code: String,
        evidence_refs: Vec<String>,
    },
    GovernedExecutionResultRecorded {
        ts_ms: u128,
        elapsed_ms: u128,
        proposal_id: String,
        action_id: String,
        adapter_id: String,
        result_ref: String,
        evidence_refs: Vec<String>,
    },
    GovernedRefusalRecorded {
        ts_ms: u128,
        elapsed_ms: u128,
        proposal_id: String,
        action_id: String,
        reason_code: String,
        evidence_refs: Vec<String>,
    },
    GovernedRedactionDecisionRecorded {
        ts_ms: u128,
        elapsed_ms: u128,
        proposal_id: String,
        audience: String,
        surfaces: Vec<String>,
        outcome: String,
        detail: Option<String>,
    },
    SchedulerPolicy {
        ts_ms: u128,
        elapsed_ms: u128,
        max_concurrency: usize,
        source: String,
    },
    RunFailed {
        ts_ms: u128,
        elapsed_ms: u128,
        message: String,
    },
    RunFinished {
        ts_ms: u128,
        elapsed_ms: u128,
        success: bool,
    },
    StepStarted {
        ts_ms: u128,
        elapsed_ms: u128,
        step_id: String,
        agent_id: String,
        provider_id: String,
        task_id: String,
        delegation: Option<DelegationSpec>,
    },
    PromptAssembled {
        ts_ms: u128,
        elapsed_ms: u128,
        step_id: String,
        prompt_hash: String,
    },
    StepOutputChunk {
        ts_ms: u128,
        elapsed_ms: u128,
        step_id: String,
        chunk_bytes: usize,
    },
    DelegationRequested {
        ts_ms: u128,
        elapsed_ms: u128,
        delegation_id: String,
        step_id: String,
        action_kind: String,
        target_id: String,
    },
    DelegationPolicyEvaluated {
        ts_ms: u128,
        elapsed_ms: u128,
        delegation_id: String,
        step_id: String,
        action_kind: String,
        target_id: String,
        decision: String,
        rule_id: Option<String>,
    },
    DelegationApproved {
        ts_ms: u128,
        elapsed_ms: u128,
        delegation_id: String,
        step_id: String,
    },
    DelegationDenied {
        ts_ms: u128,
        elapsed_ms: u128,
        delegation_id: String,
        step_id: String,
        action_kind: String,
        target_id: String,
        rule_id: Option<String>,
    },
    DelegationDispatched {
        ts_ms: u128,
        elapsed_ms: u128,
        delegation_id: String,
        step_id: String,
        action_kind: String,
        target_id: String,
    },
    DelegationResultReceived {
        ts_ms: u128,
        elapsed_ms: u128,
        delegation_id: String,
        step_id: String,
        success: bool,
        output_bytes: usize,
    },
    DelegationCompleted {
        ts_ms: u128,
        elapsed_ms: u128,
        delegation_id: String,
        step_id: String,
        outcome: String,
    },
    StepFinished {
        ts_ms: u128,
        elapsed_ms: u128,
        step_id: String,
        success: bool,
        duration_ms: u128,
    },
    CallEntered {
        ts_ms: u128,
        elapsed_ms: u128,
        caller_step_id: String,
        callee_workflow_id: String,
        namespace: String,
    },
    CallExited {
        ts_ms: u128,
        elapsed_ms: u128,
        caller_step_id: String,
        status: String,
        namespace: String,
    },
}

pub mod report;
pub mod store;
pub use report::{format_iso_utc_ms, print_trace};

#[cfg(test)]
fn module_smoke_for_coverage() -> &'static str {
    "trace-module-smoke-test"
}

#[cfg(test)]
mod tests;
