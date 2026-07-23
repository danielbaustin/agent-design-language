mod common;

use adl_compiler::{compile, compile_with_limits, CompilerDiagnosticCode, CompilerLimits};
use adl_language::WorkflowKind;

#[test]
fn node_limit_fails_closed_with_stable_diagnostic() {
    let error = compile_with_limits(
        &common::document(WorkflowKind::Sequential),
        CompilerLimits {
            max_nodes: 1,
            ..CompilerLimits::default()
        },
    )
    .unwrap_err();
    assert_eq!(error[0].code, CompilerDiagnosticCode::LimitExceeded);
    assert_eq!(error[0].path, "$.run.workflow");
}

#[test]
fn edge_limit_is_enforced_during_construction() {
    let error = compile_with_limits(
        &common::document(WorkflowKind::Sequential),
        CompilerLimits {
            max_edges: 1,
            ..CompilerLimits::default()
        },
    )
    .unwrap_err();
    assert_eq!(error[0].code, CompilerDiagnosticCode::LimitExceeded);
    assert!(error[0].message.contains("edge limit 1"));
}

#[test]
fn input_value_limit_is_aggregate_across_run_and_steps() {
    let error = compile_with_limits(
        &common::document(WorkflowKind::Sequential),
        CompilerLimits {
            max_input_values: 4,
            ..CompilerLimits::default()
        },
    )
    .unwrap_err();
    assert_eq!(error[0].code, CompilerDiagnosticCode::LimitExceeded);
}

#[test]
fn invalid_language_diagnostics_are_sorted_and_preserved() {
    let mut document = common::document(WorkflowKind::Sequential);
    document.agents.get_mut("worker").unwrap().provider = "missing".into();
    document.workflows.get_mut("flow").unwrap().steps[0].task = "missing".into();
    let errors = compile(&document).unwrap_err();
    assert!(errors.windows(2).all(|pair| pair[0] <= pair[1]));
    assert!(errors
        .iter()
        .all(|error| error.code == CompilerDiagnosticCode::InvalidDocument));
}

#[test]
fn missing_effective_agent_fails_closed() {
    let mut document = common::document(WorkflowKind::Sequential);
    document.tasks.get_mut("produce").unwrap().agent_ref = None;
    let error = compile(&document).unwrap_err();
    assert_eq!(error[0].code, CompilerDiagnosticCode::InvalidDocument);
    assert!(error[0].message.contains("do not resolve an agent"));
}
