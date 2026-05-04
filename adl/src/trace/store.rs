use std::collections::HashMap;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

use crate::adl::DelegationSpec;
use crate::execute::{ExecutionBoundary, RuntimeLifecyclePhase};

use super::{report::sanitize_governed_text, Trace, TraceEvent};

impl Trace {
    pub fn new(
        run_id: impl Into<String>,
        workflow_id: impl Into<String>,
        version: impl Into<String>,
    ) -> Self {
        let started = Self::now_ms();
        Self {
            run_id: run_id.into(),
            workflow_id: workflow_id.into(),
            version: version.into(),
            events: Vec::new(),
            run_started_ms: started,
            run_started_instant: Instant::now(),
            step_started_ms: HashMap::new(),
            delegation_ids: HashMap::new(),
            next_delegation_counter: 0,
        }
    }

    fn now_ms() -> u128 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis())
            .unwrap_or(0)
    }

    /// Record a phase transition in the runtime lifecycle.
    pub fn lifecycle_phase_entered(&mut self, phase: RuntimeLifecyclePhase) {
        let elapsed_ms = self.run_started_instant.elapsed().as_millis();
        let ts_ms = self.run_started_ms.saturating_add(elapsed_ms);
        self.events.push(TraceEvent::LifecyclePhaseEntered {
            ts_ms,
            elapsed_ms,
            phase,
        });
    }

    /// Record a boundary transition such as runtime init, workflow call, or pause.
    pub fn execution_boundary_crossed(&mut self, boundary: ExecutionBoundary, state: &str) {
        let elapsed_ms = self.run_started_instant.elapsed().as_millis();
        let ts_ms = self.run_started_ms.saturating_add(elapsed_ms);
        self.events.push(TraceEvent::ExecutionBoundaryCrossed {
            ts_ms,
            elapsed_ms,
            boundary,
            state: state.to_string(),
        });
    }

    pub fn governed_proposal_observed(
        &mut self,
        proposal_id: &str,
        tool_name: &str,
        redacted_arguments_ref: &str,
    ) {
        let elapsed_ms = self.run_started_instant.elapsed().as_millis();
        let ts_ms = self.run_started_ms.saturating_add(elapsed_ms);
        self.events.push(TraceEvent::GovernedProposalObserved {
            ts_ms,
            elapsed_ms,
            proposal_id: proposal_id.to_string(),
            tool_name: tool_name.to_string(),
            redacted_arguments_ref: redacted_arguments_ref.to_string(),
        });
    }

    pub fn governed_proposal_normalized(
        &mut self,
        proposal_id: &str,
        normalized_proposal_ref: &str,
        redacted_arguments_ref: &str,
    ) {
        let elapsed_ms = self.run_started_instant.elapsed().as_millis();
        let ts_ms = self.run_started_ms.saturating_add(elapsed_ms);
        self.events.push(TraceEvent::GovernedProposalNormalized {
            ts_ms,
            elapsed_ms,
            proposal_id: proposal_id.to_string(),
            normalized_proposal_ref: normalized_proposal_ref.to_string(),
            redacted_arguments_ref: redacted_arguments_ref.to_string(),
        });
    }

    pub fn governed_acc_constructed(
        &mut self,
        proposal_id: &str,
        acc_contract_id: &str,
        replay_posture: &str,
    ) {
        let elapsed_ms = self.run_started_instant.elapsed().as_millis();
        let ts_ms = self.run_started_ms.saturating_add(elapsed_ms);
        self.events.push(TraceEvent::GovernedAccConstructed {
            ts_ms,
            elapsed_ms,
            proposal_id: proposal_id.to_string(),
            acc_contract_id: acc_contract_id.to_string(),
            replay_posture: replay_posture.to_string(),
        });
    }

    pub fn governed_policy_injected(
        &mut self,
        proposal_id: &str,
        policy_evidence_ref: &str,
        outcome: &str,
    ) {
        let elapsed_ms = self.run_started_instant.elapsed().as_millis();
        let ts_ms = self.run_started_ms.saturating_add(elapsed_ms);
        self.events.push(TraceEvent::GovernedPolicyInjected {
            ts_ms,
            elapsed_ms,
            proposal_id: proposal_id.to_string(),
            policy_evidence_ref: policy_evidence_ref.to_string(),
            outcome: outcome.to_string(),
        });
    }

    pub fn governed_visibility_resolved(
        &mut self,
        proposal_id: &str,
        actor_view: &str,
        operator_view: &str,
        reviewer_view: &str,
        public_report_view: &str,
        observatory_projection: &str,
    ) {
        let elapsed_ms = self.run_started_instant.elapsed().as_millis();
        let ts_ms = self.run_started_ms.saturating_add(elapsed_ms);
        self.events.push(TraceEvent::GovernedVisibilityResolved {
            ts_ms,
            elapsed_ms,
            proposal_id: proposal_id.to_string(),
            actor_view: sanitize_governed_text(actor_view),
            operator_view: sanitize_governed_text(operator_view),
            reviewer_view: sanitize_governed_text(reviewer_view),
            public_report_view: sanitize_governed_text(public_report_view),
            observatory_projection: sanitize_governed_text(observatory_projection),
        });
    }

    pub fn governed_freedom_gate_decided(
        &mut self,
        proposal_id: &str,
        candidate_id: &str,
        decision: &str,
        reason_code: &str,
        boundary: &str,
        redaction_summary: &str,
    ) {
        let elapsed_ms = self.run_started_instant.elapsed().as_millis();
        let ts_ms = self.run_started_ms.saturating_add(elapsed_ms);
        self.events.push(TraceEvent::GovernedFreedomGateDecided {
            ts_ms,
            elapsed_ms,
            proposal_id: proposal_id.to_string(),
            candidate_id: candidate_id.to_string(),
            decision: decision.to_string(),
            reason_code: reason_code.to_string(),
            boundary: boundary.to_string(),
            redaction_summary: sanitize_governed_text(redaction_summary),
        });
    }

    pub fn governed_action_selected(
        &mut self,
        proposal_id: &str,
        action_id: &str,
        tool_name: &str,
        adapter_id: &str,
        evidence_refs: Vec<String>,
    ) {
        let elapsed_ms = self.run_started_instant.elapsed().as_millis();
        let ts_ms = self.run_started_ms.saturating_add(elapsed_ms);
        self.events.push(TraceEvent::GovernedActionSelected {
            ts_ms,
            elapsed_ms,
            proposal_id: proposal_id.to_string(),
            action_id: action_id.to_string(),
            tool_name: tool_name.to_string(),
            adapter_id: adapter_id.to_string(),
            evidence_refs,
        });
    }

    pub fn governed_action_rejected(
        &mut self,
        proposal_id: &str,
        action_id: &str,
        tool_name: &str,
        adapter_id: &str,
        reason_code: &str,
        evidence_refs: Vec<String>,
    ) {
        let elapsed_ms = self.run_started_instant.elapsed().as_millis();
        let ts_ms = self.run_started_ms.saturating_add(elapsed_ms);
        self.events.push(TraceEvent::GovernedActionRejected {
            ts_ms,
            elapsed_ms,
            proposal_id: proposal_id.to_string(),
            action_id: action_id.to_string(),
            tool_name: tool_name.to_string(),
            adapter_id: adapter_id.to_string(),
            reason_code: reason_code.to_string(),
            evidence_refs,
        });
    }

    pub fn governed_execution_result(
        &mut self,
        proposal_id: &str,
        action_id: &str,
        adapter_id: &str,
        result_ref: &str,
        evidence_refs: Vec<String>,
    ) {
        let elapsed_ms = self.run_started_instant.elapsed().as_millis();
        let ts_ms = self.run_started_ms.saturating_add(elapsed_ms);
        self.events
            .push(TraceEvent::GovernedExecutionResultRecorded {
                ts_ms,
                elapsed_ms,
                proposal_id: proposal_id.to_string(),
                action_id: action_id.to_string(),
                adapter_id: adapter_id.to_string(),
                result_ref: result_ref.to_string(),
                evidence_refs,
            });
    }

    pub fn governed_refusal(
        &mut self,
        proposal_id: &str,
        action_id: &str,
        reason_code: &str,
        evidence_refs: Vec<String>,
    ) {
        let elapsed_ms = self.run_started_instant.elapsed().as_millis();
        let ts_ms = self.run_started_ms.saturating_add(elapsed_ms);
        self.events.push(TraceEvent::GovernedRefusalRecorded {
            ts_ms,
            elapsed_ms,
            proposal_id: proposal_id.to_string(),
            action_id: action_id.to_string(),
            reason_code: reason_code.to_string(),
            evidence_refs,
        });
    }

    pub fn governed_redaction_decision(
        &mut self,
        proposal_id: &str,
        audience: &str,
        surfaces: Vec<String>,
        outcome: &str,
        detail: Option<&str>,
    ) {
        let elapsed_ms = self.run_started_instant.elapsed().as_millis();
        let ts_ms = self.run_started_ms.saturating_add(elapsed_ms);
        self.events
            .push(TraceEvent::GovernedRedactionDecisionRecorded {
                ts_ms,
                elapsed_ms,
                proposal_id: proposal_id.to_string(),
                audience: audience.to_string(),
                surfaces,
                outcome: outcome.to_string(),
                detail: detail.map(sanitize_governed_text),
            });
    }

    /// Mark a step start with normalized actor/provider/task metadata.
    pub fn step_started(
        &mut self,
        step_id: &str,
        agent_id: &str,
        provider_id: &str,
        task_id: &str,
        delegation: Option<&DelegationSpec>,
    ) {
        let elapsed_ms = self.run_started_instant.elapsed().as_millis();
        let ts_ms = self.run_started_ms.saturating_add(elapsed_ms);
        self.events.push(TraceEvent::StepStarted {
            ts_ms,
            elapsed_ms,
            step_id: step_id.to_string(),
            agent_id: agent_id.to_string(),
            provider_id: provider_id.to_string(),
            task_id: task_id.to_string(),
            delegation: delegation.cloned(),
        });
        self.step_started_ms.insert(step_id.to_string(), elapsed_ms);
    }

    pub fn run_failed(&mut self, message: &str) {
        let elapsed_ms = self.run_started_instant.elapsed().as_millis();
        let ts_ms = self.run_started_ms.saturating_add(elapsed_ms);
        self.events.push(TraceEvent::RunFailed {
            ts_ms,
            elapsed_ms,
            message: message.to_string(),
        });
    }

    pub fn scheduler_policy(&mut self, max_concurrency: usize, source: &str) {
        let elapsed_ms = self.run_started_instant.elapsed().as_millis();
        let ts_ms = self.run_started_ms.saturating_add(elapsed_ms);
        self.events.push(TraceEvent::SchedulerPolicy {
            ts_ms,
            elapsed_ms,
            max_concurrency,
            source: source.to_string(),
        });
    }

    pub fn prompt_assembled(&mut self, step_id: &str, prompt_hash: &str) {
        let elapsed_ms = self.run_started_instant.elapsed().as_millis();
        let ts_ms = self.run_started_ms.saturating_add(elapsed_ms);
        self.events.push(TraceEvent::PromptAssembled {
            ts_ms,
            elapsed_ms,
            step_id: step_id.to_string(),
            prompt_hash: prompt_hash.to_string(),
        });
    }

    pub fn step_output_chunk(&mut self, step_id: &str, chunk_bytes: usize) {
        let elapsed_ms = self.run_started_instant.elapsed().as_millis();
        let ts_ms = self.run_started_ms.saturating_add(elapsed_ms);
        self.events.push(TraceEvent::StepOutputChunk {
            ts_ms,
            elapsed_ms,
            step_id: step_id.to_string(),
            chunk_bytes,
        });
    }

    fn delegation_id_for_step(&mut self, step_id: &str) -> String {
        if let Some(existing) = self.delegation_ids.get(step_id) {
            return existing.clone();
        }
        self.next_delegation_counter = self.next_delegation_counter.saturating_add(1);
        let delegation_id = format!("del-{}", self.next_delegation_counter);
        self.delegation_ids
            .insert(step_id.to_string(), delegation_id.clone());
        delegation_id
    }

    pub fn delegation_requested(&mut self, step_id: &str, action_kind: &str, target_id: &str) {
        let elapsed_ms = self.run_started_instant.elapsed().as_millis();
        let ts_ms = self.run_started_ms.saturating_add(elapsed_ms);
        let delegation_id = self.delegation_id_for_step(step_id);
        self.events.push(TraceEvent::DelegationRequested {
            ts_ms,
            elapsed_ms,
            delegation_id,
            step_id: step_id.to_string(),
            action_kind: action_kind.to_string(),
            target_id: target_id.to_string(),
        });
    }

    pub fn delegation_policy_evaluated(
        &mut self,
        step_id: &str,
        action_kind: &str,
        target_id: &str,
        decision: &str,
        rule_id: Option<&str>,
    ) {
        let elapsed_ms = self.run_started_instant.elapsed().as_millis();
        let ts_ms = self.run_started_ms.saturating_add(elapsed_ms);
        let delegation_id = self.delegation_id_for_step(step_id);
        self.events.push(TraceEvent::DelegationPolicyEvaluated {
            ts_ms,
            elapsed_ms,
            delegation_id,
            step_id: step_id.to_string(),
            action_kind: action_kind.to_string(),
            target_id: target_id.to_string(),
            decision: decision.to_string(),
            rule_id: rule_id.map(|v| v.to_string()),
        });
    }

    pub fn delegation_approved(&mut self, step_id: &str) {
        let elapsed_ms = self.run_started_instant.elapsed().as_millis();
        let ts_ms = self.run_started_ms.saturating_add(elapsed_ms);
        let delegation_id = self.delegation_id_for_step(step_id);
        self.events.push(TraceEvent::DelegationApproved {
            ts_ms,
            elapsed_ms,
            delegation_id,
            step_id: step_id.to_string(),
        });
    }

    pub fn delegation_denied(
        &mut self,
        step_id: &str,
        action_kind: &str,
        target_id: &str,
        rule_id: Option<&str>,
    ) {
        let elapsed_ms = self.run_started_instant.elapsed().as_millis();
        let ts_ms = self.run_started_ms.saturating_add(elapsed_ms);
        let delegation_id = self.delegation_id_for_step(step_id);
        self.events.push(TraceEvent::DelegationDenied {
            ts_ms,
            elapsed_ms,
            delegation_id,
            step_id: step_id.to_string(),
            action_kind: action_kind.to_string(),
            target_id: target_id.to_string(),
            rule_id: rule_id.map(|v| v.to_string()),
        });
    }

    pub fn delegation_dispatched(&mut self, step_id: &str, action_kind: &str, target_id: &str) {
        let elapsed_ms = self.run_started_instant.elapsed().as_millis();
        let ts_ms = self.run_started_ms.saturating_add(elapsed_ms);
        let delegation_id = self.delegation_id_for_step(step_id);
        self.events.push(TraceEvent::DelegationDispatched {
            ts_ms,
            elapsed_ms,
            delegation_id,
            step_id: step_id.to_string(),
            action_kind: action_kind.to_string(),
            target_id: target_id.to_string(),
        });
    }

    pub fn delegation_result_received(
        &mut self,
        step_id: &str,
        success: bool,
        output_bytes: usize,
    ) {
        let elapsed_ms = self.run_started_instant.elapsed().as_millis();
        let ts_ms = self.run_started_ms.saturating_add(elapsed_ms);
        let delegation_id = self.delegation_id_for_step(step_id);
        self.events.push(TraceEvent::DelegationResultReceived {
            ts_ms,
            elapsed_ms,
            delegation_id,
            step_id: step_id.to_string(),
            success,
            output_bytes,
        });
    }

    pub fn delegation_completed(&mut self, step_id: &str, outcome: &str) {
        let elapsed_ms = self.run_started_instant.elapsed().as_millis();
        let ts_ms = self.run_started_ms.saturating_add(elapsed_ms);
        let delegation_id = self.delegation_id_for_step(step_id);
        self.events.push(TraceEvent::DelegationCompleted {
            ts_ms,
            elapsed_ms,
            delegation_id,
            step_id: step_id.to_string(),
            outcome: outcome.to_string(),
        });
    }

    /// Record a terminal step result including success and duration.
    pub fn step_finished(&mut self, step_id: &str, success: bool) {
        let elapsed_ms = self.run_started_instant.elapsed().as_millis();
        let ts_ms = self.run_started_ms.saturating_add(elapsed_ms);
        let duration_ms = self
            .step_started_ms
            .remove(step_id)
            .map(|started| elapsed_ms.saturating_sub(started))
            .unwrap_or(0);
        self.events.push(TraceEvent::StepFinished {
            ts_ms,
            elapsed_ms,
            step_id: step_id.to_string(),
            success,
            duration_ms,
        });
    }

    pub fn call_entered(
        &mut self,
        caller_step_id: &str,
        callee_workflow_id: &str,
        namespace: &str,
    ) {
        self.execution_boundary_crossed(ExecutionBoundary::WorkflowCall, "entered");
        let elapsed_ms = self.run_started_instant.elapsed().as_millis();
        let ts_ms = self.run_started_ms.saturating_add(elapsed_ms);
        self.events.push(TraceEvent::CallEntered {
            ts_ms,
            elapsed_ms,
            caller_step_id: caller_step_id.to_string(),
            callee_workflow_id: callee_workflow_id.to_string(),
            namespace: namespace.to_string(),
        });
    }

    pub fn call_exited(&mut self, caller_step_id: &str, status: &str, namespace: &str) {
        self.execution_boundary_crossed(ExecutionBoundary::WorkflowCall, status);
        let elapsed_ms = self.run_started_instant.elapsed().as_millis();
        let ts_ms = self.run_started_ms.saturating_add(elapsed_ms);
        self.events.push(TraceEvent::CallExited {
            ts_ms,
            elapsed_ms,
            caller_step_id: caller_step_id.to_string(),
            status: status.to_string(),
            namespace: namespace.to_string(),
        });
    }

    /// Record final run completion.
    pub fn run_finished(&mut self, success: bool) {
        let elapsed_ms = self.run_started_instant.elapsed().as_millis();
        let ts_ms = self.run_started_ms.saturating_add(elapsed_ms);
        self.events.push(TraceEvent::RunFinished {
            ts_ms,
            elapsed_ms,
            success,
        });
    }

    /// Return elapsed milliseconds from trace start to now.
    pub fn current_elapsed_ms(&self) -> u128 {
        self.run_started_instant.elapsed().as_millis()
    }

    /// Return the current trace timestamp in wall-clock milliseconds.
    pub fn current_ts_ms(&self) -> u128 {
        self.run_started_ms
            .saturating_add(self.run_started_instant.elapsed().as_millis())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adl::DelegationSpec;

    #[test]
    fn trace_store_records_every_event_constructor() {
        let mut trace = Trace::new("run-store", "wf-store", "0.90.5");

        trace.lifecycle_phase_entered(RuntimeLifecyclePhase::Init);
        trace.execution_boundary_crossed(ExecutionBoundary::RuntimeInit, "start");
        trace.governed_proposal_observed("proposal-1", "tool", "arg-ref");
        trace.governed_proposal_normalized("proposal-1", "norm-ref", "arg-ref");
        trace.governed_acc_constructed("proposal-1", "acc-1", "strict");
        trace.governed_policy_injected("proposal-1", "policy-1", "allowed");
        trace.governed_visibility_resolved(
            "proposal-1",
            "actor",
            "operator",
            "reviewer",
            "public",
            "projection",
        );
        trace.governed_freedom_gate_decided(
            "proposal-1",
            "candidate",
            "deny",
            "reason",
            "boundary",
            "summary",
        );
        trace.governed_action_selected(
            "proposal-1",
            "action-1",
            "tool",
            "adapter",
            vec!["a".into()],
        );
        trace.governed_action_rejected(
            "proposal-1",
            "action-1",
            "tool",
            "adapter",
            "reason",
            vec!["a".into()],
        );
        trace.governed_execution_result(
            "proposal-1",
            "action-1",
            "adapter",
            "result-ref",
            vec!["r".into()],
        );
        trace.governed_refusal("proposal-1", "action-1", "refused", vec!["e1".into()]);
        trace.governed_redaction_decision(
            "proposal-1",
            "reviewer",
            vec!["arg".into(), "result".into()],
            "redacted",
            Some("private"),
        );
        trace.governed_redaction_decision(
            "proposal-1",
            "reviewer",
            vec!["arg".into()],
            "redacted",
            None,
        );

        let delegation_spec = DelegationSpec {
            role: Some("reviewer".to_string()),
            requires_verification: Some(true),
            escalation_target: None,
            tags: vec!["safety".to_string()],
        };
        trace.step_started("s1", "agent", "provider", "task", Some(&delegation_spec));
        trace.run_failed("failed");
        trace.scheduler_policy(4, "planner");
        trace.prompt_assembled("s1", "hash");
        trace.step_output_chunk("s1", 3);
        trace.delegation_requested("s1", "provider_call", "local");
        trace.delegation_policy_evaluated("s1", "provider_call", "local", "allow", None);
        trace.delegation_policy_evaluated("s1", "provider_call", "local", "deny", Some("rule-id"));
        trace.delegation_approved("s1");
        trace.delegation_denied("s1", "provider_call", "local", Some("deny-1"));
        trace.delegation_denied("s1", "provider_call", "local", None);
        trace.delegation_dispatched("s1", "provider_call", "local");
        trace.delegation_result_received("s1", true, 12);
        trace.delegation_completed("s1", "ok");
        trace.step_finished("s1", true);
        trace.call_entered("s1", "wf-nested", "ns");
        trace.call_exited("s1", "ok", "ns");
        trace.run_finished(true);

        assert!(!trace.events.is_empty());
        assert_eq!(
            trace.events.len(),
            34,
            "every exercised constructor should have recorded an event, including call boundary helper events"
        );
        assert_eq!(trace.current_elapsed_ms(), trace.current_elapsed_ms());
        assert!(trace.current_ts_ms() >= trace.current_elapsed_ms());

        let seen_step_started = trace
            .events
            .iter()
            .any(|event| matches!(event, TraceEvent::StepStarted { .. }));
        let seen_governed = trace
            .events
            .iter()
            .any(|event| matches!(event, TraceEvent::GovernedProposalObserved { .. }));
        assert!(seen_step_started);
        assert!(seen_governed);
    }

    #[test]
    fn trace_store_preserves_delegation_counter_sequences() {
        let mut trace = Trace::new("run-store-2", "wf-store-2", "0.90.5");

        trace.delegation_requested("s1", "provider_call", "local");
        trace.delegation_dispatched("s1", "provider_call", "local");
        trace.delegation_completed("s1", "ok");
        trace.delegation_requested("s2", "provider_call", "remote");

        let ids: Vec<_> = trace
            .events
            .iter()
            .filter_map(|event| match event {
                TraceEvent::DelegationRequested { delegation_id, .. } => {
                    Some(delegation_id.as_str())
                }
                _ => None,
            })
            .collect();
        assert_eq!(ids, vec!["del-1", "del-2"]);
    }
}
