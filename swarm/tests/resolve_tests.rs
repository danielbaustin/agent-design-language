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
fn resolve_run_expands_provider_profile_deterministically() {
    let doc = parse_doc(
        r#"
version: "0.5"
providers:
  p1:
    profile: "ollama:phi4-mini"
agents:
  a1:
    provider: "p1"
    model: "phi4-mini"
tasks:
  t1:
    prompt:
      user: "u"
run:
  workflow:
    kind: sequential
    steps:
      - agent: "a1"
        task: "t1"
"#,
    );

    let resolved1 = resolve::resolve_run(&doc).expect("resolve should succeed");
    let resolved2 = resolve::resolve_run(&doc).expect("resolve should succeed");
    assert_eq!(
        resolved1.doc.providers["p1"].kind, "ollama",
        "profile should expand to concrete provider kind"
    );
    assert_eq!(
        resolved1.doc.providers["p1"].default_model.as_deref(),
        Some("phi4-mini")
    );
    let json1 = serde_json::to_string(&resolved1.doc.providers).expect("serialize providers");
    let json2 = serde_json::to_string(&resolved2.doc.providers).expect("serialize providers");
    assert_eq!(json1, json2, "expanded providers should be byte-stable");
}

#[test]
fn resolve_run_rejects_unknown_provider_profile() {
    let doc = parse_doc(
        r#"
version: "0.5"
providers:
  p1:
    profile: "unknown:profile"
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
    kind: sequential
    steps:
      - agent: "a1"
        task: "t1"
"#,
    );
    let err = resolve::resolve_run(&doc).expect_err("unknown profile should fail");
    let msg = format!("{err:#}");
    assert!(msg.contains("unknown:profile"), "unexpected error: {msg}");
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

#[test]
fn resolve_v0_2_multi_step_examples() {
    let examples = [
        include_str!("../examples/v0-2-multi-step-basic.adl.yaml"),
        include_str!("../examples/v0-2-multi-step-file-input.adl.yaml"),
        include_str!("../examples/v0-2-coordinator-agents-sdk.adl.yaml"),
    ];

    for doc_str in examples {
        let doc = parse_doc(doc_str);
        resolve::resolve_run(&doc).expect("resolve should succeed");
    }
}

#[test]
fn resolve_preserves_explicit_step_ids_for_v0_2() {
    let doc = parse_doc(include_str!("../examples/v0-2-multi-step-basic.adl.yaml"));
    let resolved = resolve::resolve_run(&doc).expect("resolve should succeed");
    assert_eq!(resolved.steps.len(), 2);
    assert_eq!(resolved.steps[0].id, "step-1");
    assert_eq!(resolved.steps[1].id, "step-2");
}
