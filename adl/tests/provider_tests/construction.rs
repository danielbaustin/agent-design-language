use ::adl::provider::{
    build_provider, expand_provider_profiles, stable_failure_kind, OllamaProvider,
};

use super::support::{adl_doc_from_yaml, provider_spec_from_yaml};

#[test]
fn build_provider_rejects_unknown_kind() {
    let spec = provider_spec_from_yaml(
        r#"
type: definitely_not_supported
config: {}
"#,
    );

    let err = match build_provider(&spec, None) {
        Ok(_) => panic!("expected build_provider to fail for unknown kind"),
        Err(e) => e,
    };
    let msg = format!("{err:#}");
    assert!(
        msg.contains("provider kind") && msg.contains("supported"),
        "expected unknown-kind error, got: {msg}"
    );
    assert_eq!(
        stable_failure_kind(&err),
        Some("schema_error"),
        "unknown provider kind should classify as schema_error"
    );
}

#[test]
fn ollama_from_spec_defaults_model_when_missing() {
    let spec = provider_spec_from_yaml(
        r#"
type: ollama
config: {}
"#,
    );

    let p = OllamaProvider::from_spec(&spec, None).expect("from_spec failed");
    // provider.rs uses this default
    assert_eq!(p.model, "llama3.1:8b");
    assert!(p.temperature.is_none());
}

#[test]
fn ollama_from_spec_prefers_model_override() {
    let spec = provider_spec_from_yaml(
        r#"
type: ollama
config:
  model: provider-model
"#,
    );

    let p = OllamaProvider::from_spec(&spec, Some("agent-model")).expect("from_spec failed");
    assert_eq!(p.model, "agent-model");
}

#[test]
fn ollama_from_spec_parses_temperature_float() {
    let spec = provider_spec_from_yaml(
        r#"
type: ollama
config:
  model: llama3.1:8b
  temperature: 0.7
"#,
    );

    let p = OllamaProvider::from_spec(&spec, None).expect("from_spec failed");
    assert_eq!(p.model, "llama3.1:8b");
    assert_eq!(p.temperature, Some(0.7_f32));
}

#[test]
fn ollama_from_spec_parses_temperature_int() {
    let spec = provider_spec_from_yaml(
        r#"
type: ollama
config:
  model: llama3.1:8b
  temperature: 1
"#,
    );

    let p = OllamaProvider::from_spec(&spec, None).expect("from_spec failed");
    assert_eq!(p.temperature, Some(1.0_f32));
}

#[test]
fn ollama_from_spec_parses_temperature_string() {
    let spec = provider_spec_from_yaml(
        r#"
type: ollama
config:
  model: llama3.1:8b
  temperature: "0.25"
"#,
    );

    let p = OllamaProvider::from_spec(&spec, None).expect("from_spec failed");
    assert_eq!(p.temperature, Some(0.25_f32));
}

#[test]
fn build_provider_builds_ollama_provider() {
    let spec = provider_spec_from_yaml(
        r#"
type: ollama
config:
  model: llama3.1:8b
"#,
    );

    // This test intentionally does NOT call complete(). Calling complete() depends on
    // external binaries and ambient environment (e.g., ADL_TIMEOUT_SECS), which can
    // make the test flaky under parallel execution. We only verify construction.
    let _p = build_provider(&spec, None).expect("build_provider failed");
}

#[test]
fn build_provider_builds_mock_provider_and_echoes_prompt() {
    let spec = provider_spec_from_yaml(
        r#"
type: mock
config: {}
"#,
    );

    let provider = build_provider(&spec, None).expect("build_provider should accept mock");
    let output = provider
        .complete("MOCK_PROVIDER_DEMO_OK")
        .expect("mock provider should complete");
    assert_eq!(output, "MOCK_PROVIDER_DEMO_OK");
}

#[test]
fn expand_mock_profile_is_runnable_through_build_provider() {
    let doc = adl_doc_from_yaml(
        r#"
version: "0.5"
providers:
  local_mock:
    profile: "mock:echo-v1"
agents:
  a1:
    provider: "local_mock"
    model: "echo-v1"
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

    let expanded = expand_provider_profiles(&doc).expect("profile expansion should succeed");
    let spec = &expanded.providers["local_mock"];
    let provider = build_provider(spec, Some("echo-v1")).expect("mock provider should build");
    let output = provider.complete("mock profile ok").expect("mock complete");
    assert_eq!(output, "mock profile ok");
}
