use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use swarm::adl::{
    AdlDoc, AgentSpec, PromptSpec, RunDefaults, RunSpec, StepSpec, TaskSpec, WorkflowKind,
    WorkflowSpec,
};

fn parse_doc(yaml: &str) -> AdlDoc {
    let doc: AdlDoc = serde_yaml::from_str(yaml).expect("yaml should parse");
    doc.validate().expect("doc should validate");
    doc
}

fn temp_dir(prefix: &str) -> PathBuf {
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let dir = std::env::temp_dir().join(format!("swarm-{prefix}-{ts}"));
    fs::create_dir_all(&dir).expect("create temp dir");
    dir
}

#[test]
fn validate_allows_minimal_valid_doc() {
    let yaml = r#"
version: "0.1"
providers:
  local:
    type: "ollama"
agents:
  a1:
    provider: "local"
    model: "phi4"
tasks:
  t1:
    description: "summarize"
    prompt:
      user: "Summarize the input."
run:
  name: "demo"
  workflow:
    kind: sequential
    steps:
      - agent: "a1"
        task: "t1"
        inputs:
          text: "hello"
"#;

    let doc = parse_doc(yaml);
    assert_eq!(doc.version, "0.1");
    assert!(doc.providers.contains_key("local"));
    assert!(doc.agents.contains_key("a1"));
    assert!(doc.tasks.contains_key("t1"));
    assert_eq!(doc.run.workflow.steps.len(), 1);
}

#[test]
fn validate_rejects_unknown_agent_reference() {
    let yaml = r#"
version: "0.1"
providers:
  p:
    type: "ollama"
agents:
  a1:
    provider: "p"
    model: "m"
tasks:
  t1:
    prompt:
      user: "Do it"
run:
  workflow:
    steps:
      - agent: "missing"
        task: "t1"
"#;

    let doc: AdlDoc = serde_yaml::from_str(yaml).expect("yaml should parse");
    let err = doc.validate().expect_err("should fail validation");
    let msg = format!("{err:#}");
    assert!(msg.contains("unknown agent"), "unexpected error: {msg}");
}

#[test]
fn validate_rejects_unknown_task_reference() {
    let yaml = r#"
version: "0.1"
providers:
  p:
    type: "ollama"
agents:
  a1:
    provider: "p"
    model: "m"
tasks:
  t1:
    prompt:
      user: "Do it"
run:
  workflow:
    steps:
      - agent: "a1"
        task: "missing"
"#;

    let doc: AdlDoc = serde_yaml::from_str(yaml).expect("yaml should parse");
    let err = doc.validate().expect_err("should fail validation");
    let msg = format!("{err:#}");
    assert!(msg.contains("unknown task"), "unexpected error: {msg}");
}

#[test]
fn effective_prompt_priority_is_step_then_task_then_agent() {
    let mut doc = AdlDoc {
        version: "0.1".to_string(),
        include: vec![],
        providers: HashMap::new(),
        tools: HashMap::new(),
        agents: HashMap::new(),
        tasks: HashMap::new(),
        workflows: HashMap::new(),
        run: RunSpec {
            name: Some("demo".to_string()),
            created_at: None,
            defaults: RunDefaults::default(),
            workflow_ref: None,
            workflow: WorkflowSpec {
                kind: WorkflowKind::Sequential,
                steps: vec![],
            },
            inputs: HashMap::new(),
        },
    };

    let agent_prompt = PromptSpec {
        system: Some("agent-system".to_string()),
        ..Default::default()
    };
    doc.agents.insert(
        "a1".to_string(),
        AgentSpec {
            provider: "p".to_string(),
            model: "m".to_string(),
            temperature: None,
            top_k: None,
            description: None,
            prompt: Some(agent_prompt.clone()),
            tools: vec![],
        },
    );

    let task_prompt = PromptSpec {
        user: Some("task-user".to_string()),
        ..Default::default()
    };
    doc.tasks.insert(
        "t1".to_string(),
        TaskSpec {
            description: None,
            agent_ref: None,
            inputs: vec![],
            tool_allowlist: vec![],
            prompt: task_prompt.clone(),
        },
    );

    let step_prompt = PromptSpec {
        developer: Some("step-dev".to_string()),
        ..Default::default()
    };

    let step_with_step_prompt = StepSpec {
        id: None,
        save_as: None,
        agent: Some("a1".to_string()),
        task: Some("t1".to_string()),
        prompt: Some(step_prompt.clone()),
        ..Default::default()
    };
    let p = step_with_step_prompt
        .effective_prompt(&doc)
        .expect("should resolve a prompt");
    assert_eq!(p.developer.as_deref(), Some("step-dev"));

    let step_with_task_prompt = StepSpec {
        id: None,
        save_as: None,
        agent: Some("a1".to_string()),
        task: Some("t1".to_string()),
        prompt: None,
        ..Default::default()
    };
    let p = step_with_task_prompt
        .effective_prompt(&doc)
        .expect("should resolve a prompt");
    assert_eq!(p.user.as_deref(), Some("task-user"));

    let step_with_agent_prompt = StepSpec {
        id: None,
        save_as: None,
        agent: Some("a1".to_string()),
        task: None,
        prompt: None,
        ..Default::default()
    };
    let p = step_with_agent_prompt
        .effective_prompt(&doc)
        .expect("should resolve a prompt");
    assert_eq!(p.system.as_deref(), Some("agent-system"));

    let step_with_no_prompt = StepSpec {
        id: None,
        save_as: None,
        agent: None,
        task: None,
        prompt: None,
        ..Default::default()
    };
    assert!(step_with_no_prompt.effective_prompt(&doc).is_none());
}

#[test]
fn load_from_file_merges_includes_in_order() {
    let dir = temp_dir("include-merge");
    let inc = dir.join("defs.yaml");
    let root = dir.join("root.yaml");

    fs::write(
        &inc,
        r#"
providers:
  local:
    type: "ollama"
agents:
  a1:
    provider: "local"
    model: "phi4-mini"
tasks:
  t1:
    prompt:
      user: "Summarize {{inputs.topic}}"
workflows:
  wf_child:
    kind: sequential
    steps:
      - id: "child_s1"
        agent: "a1"
        task: "t1"
"#,
    )
    .expect("write include");

    fs::write(
        &root,
        r#"
version: "0.5"
include:
  - "defs.yaml"
run:
  workflow:
    kind: sequential
    steps:
      - id: "parent"
        call: "wf_child"
        with:
          topic: "ADL"
        as: "child"
"#,
    )
    .expect("write root");

    let doc = AdlDoc::load_from_file(root.to_str().unwrap()).expect("load composed doc");
    assert!(doc.providers.contains_key("local"));
    assert!(doc.workflows.contains_key("wf_child"));
    assert_eq!(doc.run.workflow.steps[0].call.as_deref(), Some("wf_child"));
}

#[test]
fn load_from_file_rejects_duplicate_ids_across_includes() {
    let dir = temp_dir("include-dup");
    let a = dir.join("a.yaml");
    let b = dir.join("b.yaml");
    let root = dir.join("root.yaml");

    fs::write(
        &a,
        r#"
providers:
  local:
    type: "ollama"
"#,
    )
    .expect("write a");
    fs::write(
        &b,
        r#"
providers:
  local:
    type: "ollama"
"#,
    )
    .expect("write b");
    fs::write(
        &root,
        r#"
version: "0.5"
include:
  - "a.yaml"
  - "b.yaml"
run:
  workflow:
    kind: sequential
    steps: []
"#,
    )
    .expect("write root");

    let err = AdlDoc::load_from_file(root.to_str().unwrap())
        .expect_err("duplicate provider id should fail");
    let msg = format!("{err:#}");
    assert!(msg.contains("duplicate provider id 'local'"), "{msg}");
    assert!(msg.contains("a.yaml"), "{msg}");
    assert!(msg.contains("b.yaml"), "{msg}");
}

#[test]
fn load_from_file_rejects_missing_include_file() {
    let dir = temp_dir("include-missing");
    let root = dir.join("root.yaml");
    fs::write(
        &root,
        r#"
version: "0.5"
include:
  - "missing.yaml"
run:
  workflow:
    kind: sequential
    steps: []
"#,
    )
    .expect("write root");

    let err =
        AdlDoc::load_from_file(root.to_str().unwrap()).expect_err("missing include should fail");
    let msg = format!("{err:#}");
    assert!(msg.contains("missing.yaml"), "{msg}");
}
