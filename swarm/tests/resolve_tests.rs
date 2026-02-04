use std::collections::HashMap;

use swarm::adl;
use swarm::resolve;

fn parse_doc(yaml: &str) -> adl::AdlDoc {
    serde_yaml::from_str::<adl::AdlDoc>(yaml).expect("yaml should parse")
}

#[test]
fn resolve_produces_run_and_workflow_ids() {
    let doc = parse_doc(
        r#"
version: "0.1"
run:
  name: demo
  created_at: "2026-01-01T00:00:00Z"
  defaults:
    system: "sys"
  workflow:
    kind: sequential
    steps:
      - agent: a
        task: t
agents:
  a:
    provider: ollama_local
    model: "llama3.1:8b"
    prompt:
      system: "agent sys"
      developer: "agent dev"
      user: "agent user"
tasks:
  t:
    prompt:
      system: "task sys"
      developer: "task dev"
      user: "task user"
providers:
  ollama_local:
    type: ollama
    base_url: "http://localhost:11434"
    config:
      model: "llama3.1:8b"
"#,
    );

    let resolved = resolve::resolve_run(&doc).expect("resolve should succeed");

    assert_eq!(resolved.run_id, "demo");
    assert_eq!(resolved.workflow_id, "workflow");
    assert_eq!(resolved.steps.len(), 1);
}

#[test]
fn resolve_sets_step_id_deterministically() {
    // The resolver should assign a stable `id` even if StepSpec has no explicit id.
    let doc = parse_doc(
        r#"
version: "0.1"
run:
  name: demo
  created_at: "2026-01-01T00:00:00Z"
  defaults:
    system: "sys"
  workflow:
    kind: sequential
    steps:
      - agent: a
        task: t
agents:
  a:
    provider: ollama_local
    model: "llama3.1:8b"
    prompt:
      system: "agent sys"
      developer: "agent dev"
      user: "agent user"
tasks:
  t:
    prompt:
      system: "task sys"
      developer: "task dev"
      user: "task user"
providers:
  ollama_local:
    type: ollama
    base_url: "http://localhost:11434"
    config:
      model: "llama3.1:8b"
"#,
    );

    let r1 = resolve::resolve_run(&doc).expect("resolve should succeed");
    let r2 = resolve::resolve_run(&doc).expect("resolve should succeed");

    let s1 = &r1.steps[0];
    let s2 = &r2.steps[0];

    assert!(!s1.id.trim().is_empty());
    assert_eq!(s1.id, s2.id);
}

#[test]
fn resolve_provider_selection_uses_agent_provider() {
    // In v0.1, provider selection for a step comes from the agent.
    let doc = parse_doc(
        r#"
version: "0.1"
run:
  name: demo
  created_at: "2026-01-01T00:00:00Z"
  defaults:
    system: "sys"
  workflow:
    kind: sequential
    steps:
      - agent: a
        task: t
agents:
  a:
    provider: p1
    model: "llama3.1:8b"
    prompt:
      system: "agent sys"
      developer: "agent dev"
      user: "agent user"
tasks:
  t:
    prompt:
      system: "task sys"
      developer: "task dev"
      user: "task user"
providers:
  p1:
    type: ollama
    base_url: "http://localhost:11434"
    config:
      model: "llama3.1:8b"
"#,
    );

    let resolved = resolve::resolve_run(&doc).expect("resolve should succeed");
    assert_eq!(resolved.steps.len(), 1);
    assert_eq!(resolved.steps[0].provider.as_deref(), Some("p1"));
}

#[test]
fn resolve_step_inputs_are_preserved() {
    // Resolution should carry step.inputs through unchanged (materialization is tested elsewhere).
    let doc = parse_doc(
        r#"
version: "0.1"
run:
  name: demo
  created_at: "2026-01-01T00:00:00Z"
  defaults:
    system: "sys"
  workflow:
    kind: sequential
    steps:
      - agent: a
        task: t
        inputs:
          doc_1: "@file:examples/docs/doc_1.txt"
          note: "hello"
agents:
  a:
    provider: p1
    model: "llama3.1:8b"
    prompt:
      system: "agent sys"
      developer: "agent dev"
      user: "agent user"
tasks:
  t:
    prompt:
      system: "task sys"
      developer: "task dev"
      user: "task user"
providers:
  p1:
    type: ollama
    base_url: "http://localhost:11434"
    config:
      model: "llama3.1:8b"
"#,
    );

    let resolved = resolve::resolve_run(&doc).expect("resolve should succeed");
    let step = &resolved.steps[0];

    let mut expected = HashMap::new();
    expected.insert(
        "doc_1".to_string(),
        "@file:examples/docs/doc_1.txt".to_string(),
    );
    expected.insert("note".to_string(), "hello".to_string());

    assert_eq!(step.inputs, expected);
}

#[test]
fn resolve_sets_provider_none_when_agent_provider_is_empty_string() {
    // Current v0.1 behavior: an empty agent.provider resolves to None (it does not error).
    // This test locks in the behavior so we can change it intentionally later if desired.
    let doc = parse_doc(
        r#"
version: "0.1"
run:
  name: demo
  created_at: "2026-01-01T00:00:00Z"
  defaults:
    system: "sys"
  workflow:
    kind: sequential
    steps:
      - agent: a
        task: t
agents:
  a:
    provider: ""
    model: "llama3.1:8b"
    prompt:
      system: "agent sys"
      developer: "agent dev"
      user: "agent user"
tasks:
  t:
    prompt:
      system: "task sys"
      developer: "task dev"
      user: "task user"
providers: {}
"#,
    );

    let resolved = resolve::resolve_run(&doc).expect("resolve should succeed");
    assert_eq!(resolved.steps.len(), 1);
    assert_eq!(resolved.steps[0].provider, None);
}

#[test]
fn resolve_sets_agent_and_task_refs_on_steps() {
    // Sanity check: resolved step should preserve agent/task references.
    let doc = parse_doc(
        r#"
version: "0.1"
run:
  name: demo
  created_at: "2026-01-01T00:00:00Z"
  defaults:
    system: "sys"
  workflow:
    kind: sequential
    steps:
      - agent: a
        task: t
agents:
  a:
    provider: p1
    model: "llama3.1:8b"
    prompt:
      system: "agent sys"
      developer: "agent dev"
      user: "agent user"
tasks:
  t:
    prompt:
      system: "task sys"
      developer: "task dev"
      user: "task user"
providers:
  p1:
    type: ollama
    base_url: "http://localhost:11434"
    config:
      model: "llama3.1:8b"
"#,
    );

    let resolved = resolve::resolve_run(&doc).expect("resolve should succeed");
    let step = &resolved.steps[0];
    assert_eq!(step.agent.as_deref(), Some("a"));
    assert_eq!(step.task.as_deref(), Some("t"));
}
