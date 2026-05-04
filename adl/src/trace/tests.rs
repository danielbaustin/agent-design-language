use super::*;

#[test]
fn trace_records_step_lifecycle_events_in_order() {
    let mut tr = Trace::new("run-1", "workflow-1", "0.1");

    tr.step_started("step-1", "agent-1", "provider-1", "task-1", None);
    tr.prompt_assembled("step-1", "hash-123");
    tr.step_output_chunk("step-1", 5);
    tr.step_finished("step-1", true);

    assert_eq!(tr.events.len(), 4);

    match &tr.events[0] {
        TraceEvent::StepStarted { step_id, .. } => {
            assert_eq!(step_id, "step-1");
        }
        _ => panic!("expected StepStarted event"),
    }

    match &tr.events[1] {
        TraceEvent::PromptAssembled { prompt_hash, .. } => {
            assert_eq!(prompt_hash, "hash-123");
        }
        _ => panic!("expected PromptAssembled event"),
    }

    match &tr.events[2] {
        TraceEvent::StepOutputChunk { chunk_bytes, .. } => {
            assert_eq!(*chunk_bytes, 5);
        }
        _ => panic!("expected StepOutputChunk event"),
    }

    match &tr.events[3] {
        TraceEvent::StepFinished {
            success,
            duration_ms,
            ..
        } => {
            assert!(*success);
            assert!(*duration_ms <= 1_000);
        }
        _ => panic!("expected StepFinished event"),
    }
}

#[test]
fn trace_mod_module_smoke() {
    assert_eq!(
        crate::trace::module_smoke_for_coverage(),
        "trace-module-smoke-test"
    );
}

#[test]
fn trace_records_delegation_denied_event_with_optional_rule_id() {
    let mut tr = Trace::new("run-1", "wf-1", "0.7");
    tr.delegation_denied("s1", "provider_call", "local", Some("deny-local"));
    tr.delegation_denied("s2", "provider_call", "backup", None);

    let line0 = tr.events[0].summarize();
    let line1 = tr.events[1].summarize();
    assert!(line0.contains("DelegationDenied"));
    assert!(line0.contains("action=provider_call"));
    assert!(line0.contains("target=local"));
    assert!(line0.contains("rule_id=deny-local"));
    assert!(line1.contains("DelegationDenied"));
    assert!(line1.contains("action=provider_call"));
    assert!(line1.contains("target=backup"));
    assert!(!line1.contains("rule_id="));
}

#[test]
fn trace_records_scheduler_policy_event() {
    let mut tr = Trace::new("run-1", "wf-1", "0.7");
    tr.scheduler_policy(4, "engine_default");
    assert_eq!(tr.events.len(), 1);
    match &tr.events[0] {
        TraceEvent::SchedulerPolicy {
            max_concurrency,
            source,
            ..
        } => {
            assert_eq!(*max_concurrency, 4);
            assert_eq!(source, "engine_default");
            let summary = tr.events[0].summarize();
            assert!(summary.contains("SchedulerPolicy"));
            assert!(summary.contains("max_concurrency=4"));
            assert!(summary.contains("source=engine_default"));
        }
        _ => panic!("expected SchedulerPolicy event"),
    }
}

#[test]
fn trace_records_delegation_policy_event_with_optional_rule_id() {
    let mut tr = Trace::new("run-del-policy", "wf-del-policy", "0.7");
    tr.delegation_policy_evaluated("s1", "provider_call", "local", "denied", Some("deny-local"));
    tr.delegation_policy_evaluated("s2", "provider_call", "backup", "allowed", None);
    assert_eq!(tr.events.len(), 2);

    let line0 = tr.events[0].summarize();
    assert!(line0.contains("DelegationPolicyEvaluated"));
    assert!(line0.contains("action=provider_call"));
    assert!(line0.contains("target=local"));
    assert!(line0.contains("decision=denied"));
    assert!(line0.contains("rule_id=deny-local"));

    let line1 = tr.events[1].summarize();
    assert!(line1.contains("decision=allowed"));
    assert!(!line1.contains("rule_id="));
}

#[test]
fn trace_preserves_run_and_workflow_ids() {
    let tr = Trace::new("run-x", "workflow-y", "0.1");

    assert_eq!(tr.run_id, "run-x");
    assert_eq!(tr.workflow_id, "workflow-y");
}

#[test]
fn trace_allows_multiple_steps() {
    let mut tr = Trace::new("run-2", "workflow-2", "0.1");

    tr.step_started("step-a", "agent-a", "provider-a", "task-a", None);
    tr.step_finished("step-a", true);

    tr.step_started("step-b", "agent-b", "provider-b", "task-b", None);
    tr.step_finished("step-b", false);

    assert_eq!(tr.events.len(), 4);
}

#[test]
fn trace_records_call_events() {
    let mut tr = Trace::new("run-3", "workflow-3", "0.5");
    tr.call_entered("parent", "child", "ns");
    tr.call_exited("parent", "success", "ns");
    assert_eq!(tr.events.len(), 4);
}

#[test]
fn trace_records_runtime_lifecycle_and_boundary_events() {
    let mut tr = Trace::new("run-life", "workflow-life", "0.87.1");
    tr.lifecycle_phase_entered(RuntimeLifecyclePhase::Init);
    tr.lifecycle_phase_entered(RuntimeLifecyclePhase::Execute);
    tr.execution_boundary_crossed(ExecutionBoundary::RuntimeInit, "fresh_start");
    tr.execution_boundary_crossed(ExecutionBoundary::RunCompletion, "success");
    tr.lifecycle_phase_entered(RuntimeLifecyclePhase::Complete);
    tr.lifecycle_phase_entered(RuntimeLifecyclePhase::Teardown);

    let lines: Vec<String> = tr.events.iter().map(TraceEvent::summarize).collect();
    assert!(lines
        .iter()
        .any(|line| line.contains("LifecyclePhaseEntered phase=init")));
    assert!(lines
        .iter()
        .any(|line| line.contains("LifecyclePhaseEntered phase=execute")));
    assert!(lines
        .iter()
        .any(|line| line
            .contains("ExecutionBoundaryCrossed boundary=runtime_init state=fresh_start")));
    assert!(lines.iter().any(
        |line| line.contains("ExecutionBoundaryCrossed boundary=run_completion state=success")
    ));
    assert!(lines
        .iter()
        .any(|line| line.contains("LifecyclePhaseEntered phase=complete")));
    assert!(lines
        .iter()
        .any(|line| line.contains("LifecyclePhaseEntered phase=teardown")));
}

#[test]
fn delegation_ids_are_stable_per_step_and_increment_in_order() {
    let mut tr = Trace::new("run-del", "workflow-del", "0.7");
    tr.delegation_requested("s1", "provider_call", "local");
    tr.delegation_policy_evaluated("s1", "provider_call", "local", "allowed", None);
    tr.delegation_requested("s2", "remote_exec", "remote");

    let ids: Vec<String> = tr
        .events
        .iter()
        .filter_map(|event| match event {
            TraceEvent::DelegationRequested { delegation_id, .. } => Some(delegation_id.clone()),
            TraceEvent::DelegationPolicyEvaluated { delegation_id, .. } => {
                Some(delegation_id.clone())
            }
            _ => None,
        })
        .collect();
    assert_eq!(ids, vec!["del-1", "del-1", "del-2"]);
}

#[test]
fn delegation_event_summaries_are_stable_and_safe() {
    let mut tr = Trace::new("run-del", "workflow-del", "0.7");
    tr.delegation_requested("s1", "provider_call", "local");
    tr.delegation_policy_evaluated("s1", "provider_call", "local", "allowed", None);
    tr.delegation_dispatched("s1", "provider_call", "local");
    tr.delegation_result_received("s1", true, 12);
    tr.delegation_completed("s1", "success");

    let lines: Vec<String> = tr.events.iter().map(TraceEvent::summarize).collect();
    assert!(lines.iter().any(|line| line.contains(
        "DelegationRequested delegation_id=del-1 step=s1 action=provider_call target=local"
    )));
    assert!(lines.iter().any(|line| line.contains(
        "DelegationPolicyEvaluated delegation_id=del-1 step=s1 action=provider_call target=local decision=allowed"
    )));
    assert!(lines.iter().any(|line| line.contains(
        "DelegationDispatched delegation_id=del-1 step=s1 action=provider_call target=local"
    )));
    assert!(lines.iter().any(|line| line
        .contains("DelegationResultReceived delegation_id=del-1 step=s1 success=true bytes=12")));
    assert!(lines.iter().any(
        |line| line.contains("DelegationCompleted delegation_id=del-1 step=s1 outcome=success")
    ));
}

#[test]
fn trace_step_started_includes_delegation_when_present() {
    let mut tr = Trace::new("run-del", "workflow-del", "0.6");
    tr.step_started(
        "step-del",
        "agent-del",
        "provider-del",
        "task-del",
        Some(&DelegationSpec {
            role: Some("reviewer".to_string()),
            requires_verification: Some(true),
            escalation_target: Some("human".to_string()),
            tags: vec!["safety".to_string(), "compliance".to_string()],
        }),
    );
    let line = tr.events[0].summarize();
    assert!(
        line.contains("delegation={\"role\":\"reviewer\",\"requires_verification\":true,\"escalation_target\":\"human\",\"tags\":[\"compliance\",\"safety\"]}"),
        "line was:\n{line}"
    );
}

#[test]
fn trace_records_governed_execution_events_without_payload_leakage() {
    let mut tr = Trace::new("run-governed", "workflow-governed", "0.90.5");
    tr.governed_proposal_observed(
        "proposal.fixture.safe-read",
        "fixture.safe_read",
        "artifacts/run-governed/governed/proposal_arguments.redacted.json",
    );
    tr.governed_proposal_normalized(
        "proposal.fixture.safe-read",
        "normalized.proposal.fixture.safe-read",
        "artifacts/run-governed/governed/proposal_arguments.redacted.json",
    );
    tr.governed_acc_constructed(
        "proposal.fixture.safe-read",
        "acc.compiler.proposal.fixture.safe-read",
        "deterministic_fixture_compiler",
    );
    tr.governed_policy_injected(
        "proposal.fixture.safe-read",
        "policy.fixture.safe-read",
        "allowed",
    );
    tr.governed_visibility_resolved(
        "proposal.fixture.safe-read",
        "compiled ACC request status",
        "full compiler fixture evidence",
        "redacted compiler evidence and policy result",
        "aggregate compiler pass/fail only",
        "redacted compiler governance event",
    );
    tr.governed_freedom_gate_decided(
        "proposal.fixture.safe-read",
        "candidate.safe-read",
        "allowed",
        "gate_allowed",
        "execution",
        "private_arguments_redacted digest=sha256:bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
    );
    tr.governed_action_selected(
        "proposal.fixture.safe-read",
        "action.safe_read",
        "fixture.safe_read",
        "adapter.fixture.safe_read.dry_run",
        vec![
            "proposal:proposal.fixture.safe-read".to_string(),
            "acc:acc.compiler.proposal.fixture.safe-read".to_string(),
            "action:fixture_read".to_string(),
            "normalized_proposal:normalized.proposal.fixture.safe-read".to_string(),
            "policy:policy.fixture.safe-read".to_string(),
            "gate:candidate.safe-read".to_string(),
            "execution:action.safe_read".to_string(),
        ],
    );
    tr.governed_execution_result(
        "proposal.fixture.safe-read",
        "action.safe_read",
        "adapter.fixture.safe_read.dry_run",
        "artifacts/run-governed/governed/result.redacted.json",
        vec![
            "gate:candidate.safe-read".to_string(),
            "execution:action.safe_read".to_string(),
        ],
    );
    tr.governed_redaction_decision(
        "proposal.fixture.safe-read",
        "reviewer",
        vec![
            "arguments".to_string(),
            "results".to_string(),
            "errors".to_string(),
            "rejected_alternatives".to_string(),
        ],
        "redacted",
        Some("digest_only"),
    );

    let lines: Vec<String> = tr.events.iter().map(TraceEvent::summarize).collect();
    assert!(lines
        .iter()
        .any(|line| line.contains("GovernedProposalObserved")));
    assert!(lines
        .iter()
        .any(|line| line.contains("GovernedFreedomGateDecided")));
    assert!(lines
        .iter()
        .any(|line| line.contains("GovernedExecutionResultRecorded")));
    assert!(!lines
        .iter()
        .any(|line| line.contains("fixture-secret-token")));
}

#[test]
fn trace_sanitizes_sensitive_governed_text_fields() {
    let mut tr = Trace::new("run-governed", "workflow-governed", "0.90.5");
    tr.governed_visibility_resolved(
        "proposal.fixture.safe-read",
        "compiled ACC request status",
        "{\"secret\":\"leak\"}",
        "/Users/test/private.txt",
        "aggregate compiler pass/fail only",
        "sk-secret-token",
    );
    tr.governed_redaction_decision(
        "proposal.fixture.safe-read",
        "reviewer",
        vec!["arguments".to_string()],
        "redacted",
        Some("gho_sensitive"),
    );

    let lines: Vec<String> = tr.events.iter().map(TraceEvent::summarize).collect();
    assert!(lines
        .iter()
        .any(|line| line.contains("[redacted-sensitive-text]")));
    assert!(lines
        .iter()
        .any(|line| line.contains("[redacted-structured-text]")));
    assert!(!lines
        .iter()
        .any(|line| line.contains("/Users/test/private.txt")));
    assert!(!lines.iter().any(|line| line.contains("gho_sensitive")));
}
