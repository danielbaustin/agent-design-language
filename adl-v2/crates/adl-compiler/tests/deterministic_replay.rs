mod common;

use adl_compiler::{canonical_diagnostic_bytes, canonical_plan_bytes, compile, PlanEdgeKind};
use adl_language::{
    parse_and_validate_json, parse_and_validate_yaml, parse_json, parse_yaml, WorkflowKind,
};
use std::process::Command;

#[test]
fn repeated_compilation_is_byte_identical() {
    let document = common::document(WorkflowKind::Concurrent);
    let expected = canonical_plan_bytes(&compile(&document).unwrap()).unwrap();
    for _ in 0..100 {
        assert_eq!(
            canonical_plan_bytes(&compile(&document).unwrap()).unwrap(),
            expected
        );
    }
}

#[test]
fn json_and_yaml_key_permutations_compile_identically() {
    let json = serde_json::to_string(&common::document(WorkflowKind::Sequential)).unwrap();
    let yaml = r#"
run: {workflow_ref: flow, name: example, id: run-1, inputs: {request: hello}}
workflows:
  flow:
    steps:
      - {save_as: result, inputs: {value: {a: 1, b: 2}}, task: produce, id: first}
      - {inputs: {source: "@state:result"}, id: second, task: consume}
    kind: sequential
tasks:
  consume: {tool_allowlist: [alpha], inputs: [source], agent_ref: worker, prompt: {user: consume}}
  produce: {agent_ref: worker, prompt: {user: produce}}
agents: {worker: {tools: [zeta, alpha], provider: local}}
tools: {zeta: {type: test}, alpha: {type: test}}
providers: {local: {default_model: model-a, type: test}}
version: "0.5"
"#;
    let from_json = compile(&parse_and_validate_json(&json).unwrap()).unwrap();
    let from_yaml = compile(&parse_and_validate_yaml(yaml).unwrap()).unwrap();
    assert_eq!(
        canonical_plan_bytes(&from_json).unwrap(),
        canonical_plan_bytes(&from_yaml).unwrap()
    );
}

#[test]
fn concurrent_workflow_only_orders_explicit_state_dependencies() {
    let plan = compile(&common::document(WorkflowKind::Concurrent)).unwrap();
    assert_eq!(plan.edges.len(), 1);
    assert_eq!(plan.edges[0].kind, PlanEdgeKind::StateDependency);
    assert_eq!(plan.edges[0].state.as_deref(), Some("result"));
    assert_eq!(plan.nodes[1].tools, vec!["alpha"]);
    assert_eq!(plan.nodes[0].model.as_deref(), Some("model-a"));
    assert_eq!(plan.nodes[1].ports.inputs, vec!["source"]);
    assert!(plan.nodes[1].ports.outputs.is_empty());
    assert_eq!(plan.nodes[1].prompt.user, "consume");
    assert_eq!(
        plan.nodes[1].provenance.semantic_path,
        "$.run.workflow.steps.second"
    );
}

#[test]
fn sequential_and_state_edges_are_both_explicit() {
    let plan = compile(&common::document(WorkflowKind::Sequential)).unwrap();
    assert_eq!(plan.edges.len(), 2);
    assert!(plan
        .edges
        .iter()
        .any(|edge| edge.kind == PlanEdgeKind::Sequential));
    assert!(plan
        .edges
        .iter()
        .any(|edge| edge.kind == PlanEdgeKind::StateDependency));
}

#[test]
fn declared_input_port_order_is_preserved() {
    let mut document = common::document(WorkflowKind::Sequential);
    document.tasks.get_mut("consume").unwrap().inputs = vec!["zeta".into(), "alpha".into()];
    let plan = compile(&document).unwrap();
    assert_eq!(plan.nodes[1].ports.inputs, vec!["zeta", "alpha"]);
}

#[test]
fn equivalent_invalid_representations_have_identical_diagnostic_bytes() {
    let json = r#"{"version":"0.5","providers":{},"agents":{"worker":{"provider":"missing"}},"tasks":{"one":{"agent_ref":"worker","prompt":{"user":"one"}}},"run":{"name":"bad","workflow":{"kind":"sequential","steps":[{"id":"one","task":"one"}]}}}"#;
    let yaml = r#"
version: "0.5"
providers: {}
agents: {worker: {provider: missing}}
tasks: {one: {agent_ref: worker, prompt: {user: one}}}
run:
  name: bad
  workflow: {kind: sequential, steps: [{id: one, task: one}]}
"#;
    let json_errors = compile(&parse_json(json).unwrap()).unwrap_err();
    let yaml_errors = compile(&parse_yaml(yaml).unwrap()).unwrap_err();
    assert_eq!(
        canonical_diagnostic_bytes(&json_errors).unwrap(),
        canonical_diagnostic_bytes(&yaml_errors).unwrap()
    );
}

#[test]
fn clean_process_plan_and_diagnostic_replay_is_byte_identical() {
    let executable = std::env::current_exe().unwrap();
    let run = || {
        Command::new(&executable)
            .args(["--exact", "clean_process_worker", "--nocapture"])
            .env("ADL_COMPILER_REPLAY_CHILD", "1")
            .output()
            .unwrap()
    };
    let first = run();
    let second = run();
    assert!(first.status.success());
    assert!(second.status.success());
    let payloads = |output: Vec<u8>| {
        String::from_utf8(output)
            .unwrap()
            .lines()
            .filter(|line| line.starts_with("PLAN=") || line.starts_with("DIAGNOSTICS="))
            .map(str::to_owned)
            .collect::<Vec<_>>()
    };
    let first_payloads = payloads(first.stdout);
    let second_payloads = payloads(second.stdout);
    assert_eq!(first_payloads.len(), 2);
    assert_eq!(second_payloads.len(), 2);
    assert_eq!(first_payloads, second_payloads);
}

#[test]
fn clean_process_worker() {
    if std::env::var_os("ADL_COMPILER_REPLAY_CHILD").is_none() {
        return;
    }
    let plan = compile(&common::document(WorkflowKind::Sequential)).unwrap();
    let mut invalid = common::document(WorkflowKind::Sequential);
    invalid.agents.get_mut("worker").unwrap().provider = "missing".into();
    let diagnostics = compile(&invalid).unwrap_err();
    println!("PLAN={}", hex::encode(canonical_plan_bytes(&plan).unwrap()));
    println!(
        "DIAGNOSTICS={}",
        hex::encode(canonical_diagnostic_bytes(&diagnostics).unwrap())
    );
}
