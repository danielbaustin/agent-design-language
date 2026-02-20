use swarm::adl::AdlDoc;
use swarm::schema::validate_adl_yaml;

#[test]
fn schema_accepts_examples_adl_0_1() {
    let doc = include_str!("../examples/adl-0.1.yaml");
    validate_adl_yaml(doc).expect("example ADL should validate against schema");
}

#[test]
fn schema_examples_adl_0_1_parses_into_structs() {
    let doc = include_str!("../examples/adl-0.1.yaml");
    // If the schema accepts it, the Rust structs should parse it too.
    // This guards against schema/struct drift.
    let _parsed: AdlDoc = serde_yaml::from_str(doc).expect("example ADL should parse into AdlDoc");
}

#[test]
fn schema_accepts_cli_smoke_fixture() {
    // Keep the CLI smoke YAML in a standalone fixture so it can evolve independently.
    // This test ensures the fixture stays aligned with the schema.
    let doc = include_str!("fixtures/cli_smoke.adl.yaml");
    validate_adl_yaml(doc).expect("cli smoke fixture should validate against schema");
}

#[test]
fn schema_cli_smoke_fixture_parses_into_structs() {
    let doc = include_str!("fixtures/cli_smoke.adl.yaml");
    let _parsed: AdlDoc =
        serde_yaml::from_str(doc).expect("cli smoke fixture should parse into AdlDoc");
}

#[test]
fn schema_rejects_missing_run() {
    let bad = r#"
version: "0.1"
providers: {}
agents: {}
tasks: {}
"#;
    let err = validate_adl_yaml(bad).unwrap_err();
    let msg = format!("{err:#}");
    assert!(msg.contains("schema validation failed"));
}

#[test]
fn schema_rejects_unknown_top_level_fields() {
    let bad = r#"
version: "0.1"
providers: {}
agents: {}
tasks: {}
run:
  name: test
  workflow:
    kind: sequential
    steps:
      - id: s0
        agent: a
        task: t
extra_nope: true
"#;
    let err = validate_adl_yaml(bad).unwrap_err();
    let msg = format!("{err:#}");
    assert!(msg.contains("schema validation failed"));
}

use serde_json::Value as JsonValue;
use swarm::schema;

/// Helper: validate a YAML string with the *loose* compiled schema
/// (generated from Rust structs, no strict top-level tweaks).
fn loose_validate_yaml(yaml_text: &str) -> anyhow::Result<()> {
    let yaml_value: serde_yaml::Value = serde_yaml::from_str(yaml_text)?;
    let json_value: JsonValue = serde_json::to_value(&yaml_value)?;

    // Avoid borrowing `json_value` across the end of the function via the iterator
    // returned by `jsonschema::JSONSchema::validate`.
    let validation_result = schema::compiled_schema_loose().validate(&json_value);

    match validation_result {
        Ok(()) => Ok(()),
        Err(mut it) => {
            // Collect just one error for debugging.
            let first = it.next().map(|e| e.to_string()).unwrap_or_default();
            anyhow::bail!("loose schema rejected doc: {first}");
        }
    }
}

#[test]
fn committed_schema_json_is_present_and_has_basic_fields() {
    let j = schema::committed_schema_json();

    // Touches ADL_SCHEMA_JSON Lazy and committed_schema_json().
    let obj = j
        .as_object()
        .expect("committed schema should be a JSON object");

    // Keep these assertions lightweight (avoid overfitting).
    assert!(obj.contains_key("$schema") || obj.contains_key("$id"));
    assert!(obj.contains_key("title"));
    assert!(obj.contains_key("type"));

    // Nice sanity check that it claims to be an object schema.
    assert_eq!(obj.get("type").and_then(|v| v.as_str()), Some("object"));
}

#[test]
fn strict_rejects_unknown_top_level_key_but_loose_allows_it() {
    // This doc is structurally valid for AdlDoc, but includes an extra top-level key.
    // validate_adl_yaml() uses the strict-toplevel schema and should reject it.
    // compiled_schema_loose() should allow it (generated schema is permissive at top level).
    let yaml = r#"
version: "0.1"

providers:
  local:
    type: "ollama"
    config:
      model: "phi4-mini"

agents:
  a1:
    provider: "local"
    model: "phi4-mini"

tasks:
  t1:
    prompt:
      user: "Summarize: {{text}}"

run:
  name: "x"
  workflow:
    kind: "sequential"
    steps: []

extra_top_level_key: "oops"
"#;

    // Strict path should fail.
    let err = schema::validate_adl_yaml(yaml).unwrap_err();
    let msg = format!("{err:#}");
    assert!(
        msg.contains("Additional properties are not allowed")
            || msg.contains("extra_top_level_key")
            || msg.contains("unexpected"),
        "expected strict schema to reject unknown top-level keys; got:\n{msg}"
    );

    // Loose path should succeed.
    loose_validate_yaml(yaml).expect("loose schema should allow extra top-level keys");
}

#[test]
fn strict_rejects_unknown_provider_field() {
    let yaml = r#"
version: "0.1"

providers:
  local:
    type: "ollama"
    modell: "typo"

agents:
  a1:
    provider: "local"
    model: "phi4-mini"

tasks:
  t1:
    prompt:
      user: "Summarize: {{text}}"

run:
  name: "x"
  workflow:
    kind: "sequential"
    steps: []
"#;

    let err = schema::validate_adl_yaml(yaml).unwrap_err();
    let msg = format!("{err:#}");
    assert!(
        msg.contains("unknown") || msg.contains("Additional properties"),
        "expected unknown-field error; got:\n{msg}"
    );
    assert!(msg.contains("modell"), "expected bad key name; got:\n{msg}");
    assert!(
        msg.contains("at /providers/local"),
        "expected path context for bad field; got:\n{msg}"
    );
}

#[test]
fn strict_rejects_unknown_agent_field() {
    let yaml = r#"
version: "0.1"

providers:
  local:
    type: "ollama"

agents:
  a1:
    provider: "local"
    model: "phi4-mini"
    modell: "typo"

tasks:
  t1:
    prompt:
      user: "Summarize: {{text}}"

run:
  name: "x"
  workflow:
    kind: "sequential"
    steps: []
"#;

    let err = schema::validate_adl_yaml(yaml).unwrap_err();
    let msg = format!("{err:#}");
    assert!(
        msg.contains("unknown") || msg.contains("Additional properties"),
        "expected unknown-field error; got:\n{msg}"
    );
    assert!(msg.contains("modell"), "expected bad key name; got:\n{msg}");
    assert!(
        msg.contains("at /agents/a1"),
        "expected path context for bad field; got:\n{msg}"
    );
}

#[test]
fn strict_rejects_unknown_step_field() {
    let yaml = r#"
version: "0.1"

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
      user: "Summarize: {{text}}"

run:
  name: "x"
  workflow:
    kind: "sequential"
    steps:
      - agent: "a1"
        task: "t1"
        agentt: "typo"
"#;

    let err = schema::validate_adl_yaml(yaml).unwrap_err();
    let msg = format!("{err:#}");
    assert!(
        msg.contains("unknown") || msg.contains("Additional properties") || msg.contains("anyOf"),
        "expected unknown-field error; got:\n{msg}"
    );
    assert!(msg.contains("agentt"), "expected bad key name; got:\n{msg}");
    assert!(
        msg.contains("at /run/workflow/steps/0") || msg.contains("at /run/workflow"),
        "expected path context for bad field; got:\n{msg}"
    );
}

#[test]
fn strict_rejects_unknown_prompt_field_with_path() {
    let yaml = r#"
version: "0.2"

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
      user: "Summarize: {{text}}"
      usre: "typo"

run:
  name: "x"
  workflow:
    kind: "sequential"
    steps:
      - agent: "a1"
        task: "t1"
"#;

    let err = schema::validate_adl_yaml(yaml).unwrap_err();
    let msg = format!("{err:#}");
    assert!(
        msg.contains("unknown") || msg.contains("Additional properties"),
        "expected unknown-field error; got:\n{msg}"
    );
    assert!(msg.contains("usre"), "expected bad key name; got:\n{msg}");
    assert!(
        msg.contains("at /tasks/t1/prompt"),
        "expected path context for bad field; got:\n{msg}"
    );
}

#[test]
fn validate_adl_yaml_returns_yaml_parse_error_on_invalid_yaml() {
    // Hits the early parse-yaml failure path in schema.rs
    // (the `.context("parse yaml into Value")?` branch).
    let invalid_yaml = "version: \"0.1\"\nrun: [this is not: valid YAML\n";
    let err = schema::validate_adl_yaml(invalid_yaml).unwrap_err();
    let msg = format!("{err:#}");
    assert!(
        msg.contains("parse yaml into Value") || msg.to_lowercase().contains("yaml"),
        "expected YAML parse context; got:\n{msg}"
    );
}

#[test]
fn schema_error_message_truncates_after_ten_errors() {
    // This is designed to generate LOTS of schema errors so we hit:
    // - error collection loop
    // - "... (more schema errors omitted)" truncation line
    //
    // We include required top-level keys so we get deep structural errors (providers/agents/tasks).
    let mut yaml = String::new();
    yaml.push_str("version: \"0.1\"\n");
    yaml.push_str("providers:\n");
    for i in 0..15 {
        // Missing required "type" field (ProviderSpec.kind renamed to "type")
        yaml.push_str(&format!("  p{i}:\n"));
        yaml.push_str("    config:\n");
        yaml.push_str("      model: \"x\"\n");
    }

    yaml.push_str("agents:\n");
    for i in 0..15 {
        // Missing required "model" field (AgentSpec.model is required)
        yaml.push_str(&format!("  a{i}:\n"));
        yaml.push_str("    provider: \"local\"\n");
    }

    yaml.push_str("tasks:\n");
    for i in 0..15 {
        // Wrong type for prompt (PromptSpec is an object in Rust schema; use a number)
        yaml.push_str(&format!("  t{i}:\n"));
        yaml.push_str("    prompt: 123\n");
    }

    // Minimal run section
    yaml.push_str("run:\n");
    yaml.push_str("  name: \"many-errors\"\n");
    yaml.push_str("  workflow:\n");
    yaml.push_str("    kind: \"sequential\"\n");
    yaml.push_str("    steps: []\n");

    let err = schema::validate_adl_yaml(&yaml).unwrap_err();
    let msg = format!("{err:#}");

    assert!(
        msg.contains("ADL schema validation failed"),
        "expected schema failure header; got:\n{msg}"
    );
    assert!(
        msg.contains("... (more schema errors omitted)"),
        "expected truncation marker; got:\n{msg}"
    );

    // Also sanity check: we don't spam more than ~11–12 lines of messages.
    // (header + up to 10 errors + truncation line)
    let lines = msg.lines().count();
    assert!(
        lines <= 30,
        "error output is unexpectedly long ({lines} lines):\n{msg}"
    );
}

#[test]
fn schema_accepts_cli_smoke_fixture_via_schema_module() {
    // This directly exercises validate_adl_yaml on the fixture file, ensuring
    // it stays in lockstep with your real CLI smoke doc format.
    let doc = include_str!("fixtures/cli_smoke.adl.yaml");
    schema::validate_adl_yaml(doc).expect("cli smoke fixture should validate");
}

#[test]
fn schema_accepts_example_adl_0_1_via_schema_module() {
    // Same for your example doc (ensures schema stays aligned with examples).
    let doc = include_str!("../examples/adl-0.1.yaml");
    schema::validate_adl_yaml(doc).expect("example adl-0.1.yaml should validate");
}
