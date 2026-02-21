use std::collections::HashMap;

use swarm::adl::{
    AdlDoc, AgentSpec, PromptSpec, RunDefaults, RunSpec, StepSpec, TaskSpec, WorkflowKind,
    WorkflowSpec,
};

mod helpers;
use helpers::unique_test_temp_dir;

fn parse_doc(yaml: &str) -> AdlDoc {
    let doc: AdlDoc = serde_yaml::from_str(yaml).expect("yaml should parse");
    doc.validate().expect("doc should validate");
    doc
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
    assert_eq!(
        doc.run
            .workflow
            .as_ref()
            .expect("inline workflow")
            .steps
            .len(),
        1
    );
}

#[test]
fn validate_rejects_unknown_agent_reference() {
    let yaml = r#"
version: "0.1"
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
        providers: HashMap::new(),
        tools: HashMap::new(),
        agents: HashMap::new(),
        tasks: HashMap::new(),
        workflows: HashMap::new(),
        patterns: vec![],
        signature: None,
        run: RunSpec {
            id: None,
            name: Some("demo".to_string()),
            created_at: None,
            defaults: RunDefaults::default(),
            workflow_ref: None,
            workflow: Some(WorkflowSpec {
                id: None,
                kind: WorkflowKind::Sequential,
                steps: vec![],
            }),
            pattern_ref: None,
            inputs: HashMap::new(),
            placement: None,
            remote: None,
        },
    };

    let agent_prompt = PromptSpec {
        system: Some("agent-system".to_string()),
        ..Default::default()
    };
    doc.agents.insert(
        "a1".to_string(),
        AgentSpec {
            id: None,
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
            id: None,
            agent_ref: None,
            inputs: vec![],
            tool_allowlist: vec![],
            description: None,
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
fn validate_accepts_v0_5_complete_doc_with_all_primitives() {
    let yaml = r#"
version: "0.5"
providers:
  local_ollama:
    id: "local_ollama"
    type: "local_ollama"
    config:
      model: "phi4-mini"
tools:
  fetch_docs:
    id: "fetch_docs"
    type: "mcp"
    config:
      server: "docs"
agents:
  planner:
    id: "planner"
    provider: "local_ollama"
    model: "phi4-mini"
    tools: ["fetch_docs"]
tasks:
  summarize:
    id: "summarize"
    agent_ref: "planner"
    inputs: ["topic"]
    tool_allowlist: ["fetch_docs"]
    prompt:
      user: "Summarize {{topic}}."
workflows:
  wf_main:
    id: "wf_main"
    kind: sequential
    steps:
      - id: "s1"
        task: "summarize"
run:
  id: "run_main"
  name: "demo-v0-5"
  workflow_ref: "wf_main"
  inputs:
    topic: "ADL"
  placement:
    target: "local"
"#;

    let doc: AdlDoc = serde_yaml::from_str(yaml).expect("yaml parse");
    doc.validate().expect("v0.5 complete doc should validate");

    assert_eq!(doc.workflows.len(), 1);
    assert_eq!(doc.tasks["summarize"].agent_ref.as_deref(), Some("planner"));
    assert_eq!(doc.run.workflow_ref.as_deref(), Some("wf_main"));
}

#[test]
fn validate_rejects_zero_max_concurrency() {
    let yaml = r#"
version: "0.3"
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
      user: "hello"
run:
  defaults:
    max_concurrency: 0
  workflow:
    kind: concurrent
    steps:
      - id: "s1"
        agent: "a1"
        task: "t1"
"#;
    let doc: AdlDoc = serde_yaml::from_str(yaml).expect("yaml should parse");
    let err = doc.validate().expect_err("max_concurrency=0 must fail");
    assert!(
        err.to_string().contains("max_concurrency must be >= 1"),
        "unexpected error: {err:#}"
    );
}

#[test]
fn validate_rejects_unknown_workflow_ref() {
    let yaml = r#"
version: "0.5"
providers:
  p1: { type: "ollama" }
agents:
  a1:
    provider: "p1"
    model: "m"
tasks:
  t1:
    prompt:
      user: "u"
workflows: {}
run:
  workflow_ref: "wf_missing"
"#;
    let doc: AdlDoc = serde_yaml::from_str(yaml).expect("yaml parse");
    let err = doc
        .validate()
        .expect_err("unknown workflow_ref should fail");
    assert!(
        err.to_string()
            .contains("run.workflow_ref references unknown workflow"),
        "{err:#}"
    );
}

#[test]
fn validate_rejects_unsupported_provider_kind() {
    let yaml = r#"
version: "0.5"
providers:
  p1:
    type: "weird_provider"
agents:
  a1:
    provider: "p1"
    model: "m"
tasks:
  t1:
    prompt:
      user: "u"
run:
  workflow:
    steps:
      - task: "t1"
        agent: "a1"
"#;
    let doc: AdlDoc = serde_yaml::from_str(yaml).expect("yaml parse");
    let err = doc
        .validate()
        .expect_err("unsupported provider kind should fail");
    assert!(err.to_string().contains("unsupported kind"), "{err:#}");
}

#[test]
fn validate_rejects_both_workflow_ref_and_inline_workflow() {
    let yaml = r#"
version: "0.5"
providers:
  p1: { type: "ollama" }
agents:
  a1:
    provider: "p1"
    model: "m"
tasks:
  t1:
    prompt:
      user: "u"
workflows:
  wf_main:
    steps:
      - task: "t1"
        agent: "a1"
run:
  workflow_ref: "wf_main"
  workflow:
    steps:
      - task: "t1"
        agent: "a1"
"#;
    let doc: AdlDoc = serde_yaml::from_str(yaml).expect("yaml parse");
    let err = doc
        .validate()
        .expect_err("both workflow_ref and inline workflow should fail");
    assert!(
        err.to_string()
            .contains("either workflow_ref or inline workflow, but not both"),
        "{err:#}"
    );
}

#[test]
fn validate_rejects_pattern_ref_with_inline_workflow() {
    let yaml = r#"
version: "0.5"
providers:
  local:
    type: "ollama"
tasks:
  A:
    prompt:
      user: "A"
patterns:
  - id: p1
    type: linear
    steps: [A]
run:
  pattern_ref: p1
  workflow:
    kind: sequential
    steps: []
"#;

    let doc: AdlDoc = serde_yaml::from_str(yaml).expect("yaml should parse");
    let err = doc
        .validate()
        .expect_err("should reject ambiguous run shape");
    assert!(
        err.to_string().contains(
            "run.pattern_ref cannot be combined with run.workflow_ref or inline run.workflow"
        ),
        "unexpected error: {err:#}"
    );
}

#[test]
fn validate_rejects_pattern_ref_with_workflow_ref() {
    let yaml = r#"
version: "0.5"
providers:
  local:
    type: "ollama"
tasks:
  A:
    prompt:
      user: "A"
workflows:
  wf:
    id: "wf"
    kind: sequential
    steps:
      - id: "s1"
        task: "A"
patterns:
  - id: p1
    type: linear
    steps: [A]
run:
  pattern_ref: p1
  workflow_ref: "wf"
"#;

    let doc: AdlDoc = serde_yaml::from_str(yaml).expect("yaml should parse");
    let err = doc
        .validate()
        .expect_err("should reject ambiguous run shape");
    assert!(
        err.to_string().contains(
            "run.pattern_ref cannot be combined with run.workflow_ref or inline run.workflow"
        ),
        "unexpected error: {err:#}"
    );
}

#[test]
fn load_from_file_rejects_include_with_parent_dir() {
    let dir = unique_test_temp_dir("adl-include-parent");
    let adl_path = dir.join("doc.yaml");
    let yaml = r#"
version: "0.1"
include:
  - "../outside.yaml"
providers:
  local:
    type: "ollama"
agents:
  a1:
    provider: "local"
    model: "m"
tasks:
  t1:
    prompt:
      user: "hi"
run:
  workflow:
    steps:
      - agent: "a1"
        task: "t1"
"#;
    std::fs::write(&adl_path, yaml).expect("write adl");

    let err = AdlDoc::load_from_file(adl_path.to_str().unwrap()).unwrap_err();
    let msg = format!("{err:#}");
    assert!(
        msg.contains("include path must be relative") && msg.contains("must not contain '..'"),
        "unexpected error: {msg}"
    );
}

#[test]
fn validate_rejects_write_to_without_save_as() {
    let yaml = r#"
version: "0.1"
providers:
  local:
    type: "ollama"
agents:
  a1:
    provider: "local"
    model: "m"
tasks:
  t1:
    prompt:
      user: "hi"
run:
  workflow:
    steps:
      - id: "s1"
        agent: "a1"
        task: "t1"
        write_to: "out/result.txt"
"#;

    let doc: AdlDoc = serde_yaml::from_str(yaml).expect("yaml should parse");
    let err = doc
        .validate()
        .expect_err("write_to without save_as should fail");
    let msg = format!("{err:#}");
    assert!(
        msg.contains("uses write_to but is missing save_as"),
        "unexpected error: {msg}"
    );
}
