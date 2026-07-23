mod common;

use adl_compiler::compile;
use adl_language::WorkflowKind;

#[test]
fn stable_ids_ignore_unrelated_document_values() {
    let original = common::document(WorkflowKind::Sequential);
    let mut changed = original.clone();
    changed
        .run
        .inputs
        .insert("extra".into(), serde_json::json!(true));
    let left: Vec<_> = compile(&original)
        .unwrap()
        .nodes
        .into_iter()
        .map(|node| node.id)
        .collect();
    let right: Vec<_> = compile(&changed)
        .unwrap()
        .nodes
        .into_iter()
        .map(|node| node.id)
        .collect();
    assert_eq!(left, right);
}

#[test]
fn stable_ids_change_with_resolved_execution_semantics() {
    let original = common::document(WorkflowKind::Sequential);
    let original_ids: Vec<_> = compile(&original)
        .unwrap()
        .nodes
        .into_iter()
        .map(|node| node.id)
        .collect();

    let mut prompt_changed = original.clone();
    prompt_changed.tasks.get_mut("produce").unwrap().prompt.user = "changed".into();
    let prompt_ids: Vec<_> = compile(&prompt_changed)
        .unwrap()
        .nodes
        .into_iter()
        .map(|node| node.id)
        .collect();
    assert_ne!(original_ids[0], prompt_ids[0]);
    assert_eq!(original_ids[1], prompt_ids[1]);

    let mut model_changed = original.clone();
    model_changed.agents.get_mut("worker").unwrap().model = Some("model-b".into());
    let model_ids: Vec<_> = compile(&model_changed)
        .unwrap()
        .nodes
        .into_iter()
        .map(|node| node.id)
        .collect();
    assert_ne!(original_ids, model_ids);

    let mut tools_changed = original.clone();
    tools_changed.agents.get_mut("worker").unwrap().tools = vec!["alpha".into()];
    let tool_ids: Vec<_> = compile(&tools_changed)
        .unwrap()
        .nodes
        .into_iter()
        .map(|node| node.id)
        .collect();
    assert_ne!(original_ids[0], tool_ids[0]);
    assert_eq!(original_ids[1], tool_ids[1]);

    let mut normalized_equivalent = original.clone();
    normalized_equivalent
        .agents
        .get_mut("worker")
        .unwrap()
        .tools
        .reverse();
    normalized_equivalent
        .providers
        .get_mut("local")
        .unwrap()
        .config
        .insert("unused".into(), serde_json::json!(true));
    let equivalent_ids: Vec<_> = compile(&normalized_equivalent)
        .unwrap()
        .nodes
        .into_iter()
        .map(|node| node.id)
        .collect();
    assert_eq!(original_ids, equivalent_ids);
}

#[test]
fn stable_ids_change_with_semantic_step_identity() {
    let original = common::document(WorkflowKind::Sequential);
    let mut changed = original.clone();
    changed.workflows.get_mut("flow").unwrap().steps[1].id = "renamed".into();
    let left = compile(&original).unwrap().nodes;
    let right = compile(&changed).unwrap().nodes;
    assert_eq!(left[0].id, right[0].id);
    assert_ne!(left[1].id, right[1].id);
}

#[test]
fn stable_identity_golden_vector_is_versioned() {
    let plan = compile(&common::document(WorkflowKind::Sequential)).unwrap();
    assert_eq!(
        plan.nodes[0].id,
        "node_v1_4c74406a7d00ee7c00554dc8aba2a8b76166f30bc812e072a85da0a5b47663ea"
    );
}
