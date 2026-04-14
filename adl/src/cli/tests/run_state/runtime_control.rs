use super::*;

#[test]
fn derive_runtime_control_state_triggers_reframing_on_failure() {
    let tr = trace::Trace::new(
        "runtime-reframing-failure".to_string(),
        "wf".to_string(),
        "0.86".to_string(),
    );
    let records = vec![execute::StepExecutionRecord {
        step_id: "s1".to_string(),
        provider_id: "p1".to_string(),
        status: "failure".to_string(),
        attempts: 2,
        output_bytes: 0,
    }];
    let runtime_control = execute::derive_runtime_control_state("failure", &records, &tr);
    assert_eq!(
        runtime_control.evaluation.next_control_action,
        "handoff_to_reframing"
    );
    assert_eq!(runtime_control.reframing.reframing_trigger, "triggered");
    assert_eq!(
        runtime_control.reframing.reexecution_choice,
        "bounded_reframe_and_retry"
    );
    assert_eq!(runtime_control.freedom_gate.gate_decision, "escalate");
    assert_eq!(
        runtime_control.freedom_gate.reason_code,
        "frame_escalation_required"
    );
    assert_eq!(
        runtime_control.freedom_gate.required_follow_up,
        "escalate_for_judgment_review"
    );
    assert!(
        runtime_control.reframing.frame_adequacy_score < 50,
        "failure should lower the frame adequacy score"
    );
}

#[test]
fn derive_runtime_control_state_retains_current_frame_on_success() {
    let tr = trace::Trace::new(
        "runtime-reframing-success".to_string(),
        "wf".to_string(),
        "0.86".to_string(),
    );
    let runtime_control = runtime_control_for("success", &tr);
    assert_eq!(
        runtime_control.evaluation.next_control_action,
        "complete_run"
    );
    assert_eq!(runtime_control.reframing.reframing_trigger, "not_triggered");
    assert_eq!(runtime_control.reframing.new_frame, "retain_current_frame");
    assert_eq!(runtime_control.reframing.post_reframe_state, "complete_run");
    assert_eq!(runtime_control.freedom_gate.gate_decision, "allow");
    assert_eq!(runtime_control.freedom_gate.reason_code, "policy_allowed");
}
