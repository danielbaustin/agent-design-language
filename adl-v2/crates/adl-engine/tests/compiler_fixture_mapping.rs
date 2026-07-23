mod common;

use adl_compiler::{compile, ExecutionPlan, PlanEdgeKind};
use adl_engine::{
    CompletionOutcome, Engine, EngineErrorCode, EnginePolicy, NodeState, PortCompletion,
    PortOutput, ProviderCompletion, TurnInput,
};
use adl_language::parse_and_validate_yaml;
use serde_json::{json, Value};
use std::collections::BTreeSet;
use std::fs;
use std::path::PathBuf;

const APPLICABLE: &[(&str, &str)] = &[
    ("map-a.adl.yaml", "actual_compiler_single_node"),
    ("map-b.adl.yaml", "actual_compiler_single_node"),
    ("mock-run.adl.yaml", "actual_compiler_ports"),
    ("sequential-a.adl.yaml", "actual_compiler_sequential"),
    ("sequential-b.adl.yaml", "actual_compiler_sequential"),
    ("six-primitives.adl.yaml", "actual_compiler_ports"),
];
const LEGACY_NON_ENGINE: &[&str] = &[
    "branch-a.adl.yaml",
    "branch-b.adl.yaml",
    "fork-join.adl.yaml",
];
const LANGUAGE_NEGATIVE: &[&str] = &[
    "cycle.adl.yaml",
    "malformed.adl.yaml",
    "schema-unknown.adl.yaml",
    "state-missing.adl.yaml",
    "unknown-agent.adl.yaml",
    "unknown-provider.adl.yaml",
    "unknown-task.adl.yaml",
    "unknown-tool.adl.yaml",
    "unknown-workflow.adl.yaml",
    "unsupported-run-field.adl.yaml",
];

fn fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../../adl-characterization/corpus/v1/fixtures")
}

fn compiler_plan(kind: &str) -> ExecutionPlan {
    let document = serde_json::from_value(json!({
        "version": "0.5",
        "providers": {"local": {"type": "test", "default_model": "model-a"}},
        "tools": {"alpha": {"type": "test"}},
        "agents": {"worker": {"provider": "local", "tools": ["alpha"]}},
        "tasks": {
            "produce": {"agent_ref": "worker", "prompt": {"user": "produce"}},
            "consume": {
                "agent_ref": "worker",
                "inputs": ["source"],
                "tool_allowlist": ["alpha"],
                "prompt": {"user": "consume"}
            }
        },
        "workflows": {"flow": {
            "kind": kind,
            "steps": [
                {
                    "id": "first",
                    "task": "produce",
                    "inputs": {"value": {"b": 2, "a": 1}},
                    "save_as": "result"
                },
                {
                    "id": "second",
                    "task": "consume",
                    "inputs": {"source": {"nested": ["@state:result"]}}
                }
            ]
        }},
        "run": {
            "id": "run-1",
            "name": "fixture-map",
            "workflow_ref": "flow",
            "inputs": {"request": "hello"}
        }
    }))
    .unwrap();
    compile(&document).unwrap()
}

fn json_success(request: &adl_engine::ProviderRequest, value: Value) -> PortCompletion {
    PortCompletion::Provider(Box::new(ProviderCompletion {
        request_id: request.request_id.clone(),
        node_id: request.node_id.clone(),
        attempt: request.attempt,
        outcome: CompletionOutcome::Success(PortOutput::new(
            "application/json",
            serde_json::to_vec(&value).unwrap(),
        )),
    }))
}

fn downstream_request(value: Value) -> adl_engine::ProviderRequest {
    let plan = compiler_plan("concurrent");
    let policy = EnginePolicy::provider_for(&plan, 20);
    let mut engine = Engine::new(plan, policy, common::limits()).unwrap();
    let first = common::provider_request(&engine.turn(TurnInput::tick(1)).unwrap());
    let second_turn = engine
        .turn(TurnInput {
            logical_tick: 2,
            completions: vec![json_success(&first, value)],
            cancellations: vec![],
        })
        .unwrap();
    common::provider_request(&second_turn)
}

fn execute_compiled_plan(plan: ExecutionPlan) {
    let policy = EnginePolicy::provider_for(&plan, 20);
    let mut engine = Engine::new(plan, policy, common::limits()).unwrap();
    let mut tick = 1_u64;
    let mut output = engine.turn(TurnInput::tick(tick)).unwrap();
    while !engine.is_terminal() {
        let requests = common::provider_requests(&output);
        assert!(
            !requests.is_empty(),
            "compiled fixture stalled before terminal"
        );
        tick += 1;
        output = engine
            .turn(TurnInput {
                logical_tick: tick,
                completions: requests
                    .iter()
                    .map(|request| common::provider_success(request, b"fixture-ok"))
                    .collect(),
                cancellations: vec![],
            })
            .unwrap();
    }
    assert!(engine
        .snapshot()
        .nodes
        .values()
        .all(|node| matches!(node.state, NodeState::Succeeded { .. })));
}

#[test]
fn landed_fixture_inventory_has_one_reviewed_engine_classification() {
    let observed = fs::read_dir(fixture_root())
        .unwrap()
        .map(|entry| entry.unwrap().file_name().into_string().unwrap())
        .filter(|name| name.ends_with(".adl.yaml"))
        .collect::<BTreeSet<_>>();
    let classified = APPLICABLE
        .iter()
        .map(|(name, _)| (*name).to_owned())
        .chain(LEGACY_NON_ENGINE.iter().map(|name| (*name).to_owned()))
        .chain(LANGUAGE_NEGATIVE.iter().map(|name| (*name).to_owned()))
        .collect::<BTreeSet<_>>();
    assert_eq!(observed, classified, "#5338 fixture inventory drifted");
    for (name, route) in APPLICABLE {
        assert!(route.starts_with("actual_compiler_"));
        let source = fs::read_to_string(fixture_root().join(name)).unwrap();
        let document = parse_and_validate_yaml(&source)
            .unwrap_or_else(|error| panic!("applicable fixture {name} failed parse: {error:?}"));
        let plan = compile(&document)
            .unwrap_or_else(|error| panic!("applicable fixture {name} failed compile: {error:?}"));
        match *route {
            "actual_compiler_single_node" => {
                assert_eq!(plan.nodes.len(), 1, "{name}");
                assert!(plan.edges.is_empty(), "{name}");
            }
            "actual_compiler_sequential" => {
                assert_eq!(plan.nodes.len(), 2, "{name}");
                assert!(plan
                    .edges
                    .iter()
                    .any(|edge| edge.kind == PlanEdgeKind::Sequential));
            }
            "actual_compiler_ports" => {
                assert!(!plan.nodes.is_empty(), "{name}");
                assert!(plan.nodes.iter().all(|node| !node.provider_ref.is_empty()));
            }
            other => panic!("unrecognized applicable route {other}"),
        }
        execute_compiled_plan(plan);
    }
    for name in LEGACY_NON_ENGINE.iter().chain(LANGUAGE_NEGATIVE) {
        assert!(!fs::read_to_string(fixture_root().join(name))
            .unwrap()
            .is_empty());
    }
}

fn repeated_state_plan(references: usize) -> ExecutionPlan {
    let mut plan = compiler_plan("concurrent");
    let consumer = plan
        .nodes
        .iter_mut()
        .find(|node| node.step_id == "second")
        .unwrap();
    consumer.inputs.insert(
        "source".into(),
        Value::Array(vec![Value::String("@state:result".into()); references]),
    );
    plan
}

fn repeated_state_turn(
    plan: ExecutionPlan,
    maximum: u64,
) -> Result<adl_engine::TurnOutput, adl_engine::EngineError> {
    let policy = EnginePolicy::provider_for(&plan, 20);
    let mut limits = common::limits();
    limits.max_request_bytes = maximum;
    let mut engine = Engine::new(plan, policy, limits)?;
    let first = common::provider_request(&engine.turn(TurnInput::tick(1))?);
    engine.turn(TurnInput {
        logical_tick: 2,
        completions: vec![json_success(&first, json!({"payload": "x".repeat(256)}))],
        cancellations: vec![],
    })
}

#[test]
fn repeated_state_materialization_is_bounded_before_clone_expansion() {
    let plan = repeated_state_plan(12);
    let output = repeated_state_turn(plan.clone(), common::limits().max_request_bytes).unwrap();
    let required = serde_json::to_vec(&output.effects[0]).unwrap().len() as u64;
    assert!(repeated_state_turn(plan.clone(), required + 1).is_ok());
    assert!(repeated_state_turn(plan.clone(), required).is_ok());
    assert_eq!(
        repeated_state_turn(plan, required - 1).unwrap_err().code,
        EngineErrorCode::ResourceLimit
    );
}

#[test]
fn actual_compiler_plans_preserve_edges_ports_provenance_and_state_dataflow() {
    let concurrent = compiler_plan("concurrent");
    assert_eq!(concurrent.edges.len(), 1);
    assert_eq!(concurrent.edges[0].kind, PlanEdgeKind::StateDependency);
    assert_eq!(concurrent.edges[0].state.as_deref(), Some("result"));
    let consumer = concurrent
        .nodes
        .iter()
        .find(|node| node.step_id == "second")
        .unwrap();
    assert_eq!(consumer.ports.inputs, vec!["source"]);
    assert_eq!(consumer.tools, vec!["alpha"]);
    assert_eq!(
        consumer.provenance.semantic_path,
        "$.run.workflow.steps.second"
    );

    let sequential = compiler_plan("sequential");
    assert!(sequential
        .edges
        .iter()
        .any(|edge| edge.kind == PlanEdgeKind::Sequential));
    assert!(sequential
        .edges
        .iter()
        .any(|edge| edge.kind == PlanEdgeKind::StateDependency));

    let first_value = json!({"answer": 42});
    let request = downstream_request(first_value.clone());
    assert_eq!(request.inputs["source"]["nested"][0], first_value);
    let changed = downstream_request(json!({"answer": 43}));
    assert_ne!(request.request_id, changed.request_id);
    assert_ne!(request.idempotency_key, changed.idempotency_key);
}

#[test]
fn failed_state_source_never_dispatches_a_placeholder_consumer() {
    let plan = compiler_plan("concurrent");
    let consumer_id = plan
        .nodes
        .iter()
        .find(|node| node.step_id == "second")
        .unwrap()
        .id
        .clone();
    let policy = EnginePolicy::provider_for(&plan, 20);
    let mut engine = Engine::new(plan, policy, common::limits()).unwrap();
    let first = common::provider_request(&engine.turn(TurnInput::tick(1)).unwrap());
    let output = engine
        .turn(TurnInput {
            logical_tick: 2,
            completions: vec![common::provider_failure(
                &first,
                adl_engine::FailureClass::Permanent,
            )],
            cancellations: vec![],
        })
        .unwrap();
    assert!(output.effects.is_empty());
    assert!(matches!(
        engine.snapshot().nodes[&consumer_id].state,
        NodeState::Failed { .. }
    ));
}
