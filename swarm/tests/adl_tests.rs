use std::collections::HashMap;

use swarm::adl::{
    AdlDoc, AgentSpec, PromptSpec, RunDefaults, RunSpec, StepSpec, TaskSpec, WorkflowKind,
    WorkflowSpec,
};

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
    assert_eq!(doc.run.workflow.steps.len(), 1);
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
        patterns: vec![],
        run: RunSpec {
            name: Some("demo".to_string()),
            created_at: None,
            defaults: RunDefaults::default(),
            pattern_ref: None,
            workflow: WorkflowSpec {
                kind: WorkflowKind::Sequential,
                steps: vec![],
            },
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
fn validate_rejects_pattern_ref_with_inline_workflow_steps() {
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
    steps:
      - task: "A"
"#;

    let doc: AdlDoc = serde_yaml::from_str(yaml).expect("yaml should parse");
    let err = doc
        .validate()
        .expect_err("should reject ambiguous run shape");
    assert!(
        err.to_string()
            .contains("run.pattern_ref cannot be combined with run.workflow.steps"),
        "unexpected error: {err:#}"
    );
}

#[test]
fn validate_rejects_pattern_ref_with_non_sequential_workflow_kind() {
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
    kind: concurrent
    steps: []
"#;

    let doc: AdlDoc = serde_yaml::from_str(yaml).expect("yaml should parse");
    let err = doc
        .validate()
        .expect_err("should reject ambiguous run shape");
    assert!(
        err.to_string()
            .contains("run.pattern_ref cannot be combined with non-sequential run.workflow.kind"),
        "unexpected error: {err:#}"
    );
}
