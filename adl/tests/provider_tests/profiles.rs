use ::adl::provider::{expand_provider_profiles, provider_profile_names};

use super::support::adl_doc_from_yaml;

#[test]
fn provider_profiles_registry_is_deterministic_and_has_at_least_twelve_profiles() {
    let names = provider_profile_names();
    let mut sorted = names.clone();
    sorted.sort();
    assert_eq!(
        names, sorted,
        "profile names must be sorted deterministically"
    );
    assert!(
        names.len() >= 12,
        "expected at least 12 profiles, got {}",
        names.len()
    );
}

#[test]
fn expand_provider_profiles_rejects_unknown_profile() {
    let doc = adl_doc_from_yaml(
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
    let err = expand_provider_profiles(&doc).expect_err("unknown profile should fail");
    let msg = err.to_string();
    assert!(
        msg.contains("unknown:profile") && msg.contains("available:"),
        "unexpected error: {msg}"
    );
}

#[test]
fn expand_provider_profiles_rejects_profile_with_explicit_fields() {
    let doc = adl_doc_from_yaml(
        r#"
version: "0.5"
providers:
  p1:
    profile: "ollama:phi4-mini"
    type: "ollama"
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
    let err = expand_provider_profiles(&doc).expect_err("profile + explicit fields must fail");
    assert!(
        err.to_string()
            .contains("profile and explicit provider identity fields together"),
        "{err:#}"
    );
}

#[test]
fn expand_provider_profiles_is_byte_stable_across_runs() {
    let doc = adl_doc_from_yaml(
        r#"
version: "0.5"
providers:
  a_mock:
    profile: "mock:echo-v1"
  z_ollama:
    profile: "ollama:phi4-mini"
agents:
  a1:
    provider: "z_ollama"
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
    let expanded1 = expand_provider_profiles(&doc).expect("expand run 1");
    let expanded2 = expand_provider_profiles(&doc).expect("expand run 2");

    let json1 = serde_json::to_string(&expanded1.providers).expect("serialize providers");
    let json2 = serde_json::to_string(&expanded2.providers).expect("serialize providers");
    assert_eq!(json1, json2, "profile expansion must be byte-stable");

    assert_eq!(
        expanded1.providers["z_ollama"].kind, "ollama",
        "ollama profile should expand to kind=ollama"
    );
    assert_eq!(
        expanded1.providers["a_mock"].kind, "mock",
        "mock profile should expand to kind=mock"
    );
}

#[test]
fn expand_provider_profiles_rejects_http_profile_without_endpoint_override() {
    let doc = adl_doc_from_yaml(
        r#"
version: "0.5"
providers:
  p1:
    profile: "http:gpt-4o-mini"
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
    let err = expand_provider_profiles(&doc).expect_err("placeholder endpoint profile must fail");
    let msg = err.to_string();
    assert!(
        msg.contains("providers.p1.profile 'http:gpt-4o-mini'")
            && msg.contains("placeholder or invalid endpoint")
            && msg.contains("configure providers.p1.config.endpoint"),
        "unexpected error: {msg}"
    );
}

#[test]
fn expand_provider_profiles_accepts_http_profile_with_endpoint_override() {
    let doc = adl_doc_from_yaml(
        r#"
version: "0.5"
providers:
  p1:
    profile: "http:gpt-4o-mini"
    config:
      endpoint: "https://api.openai.com/v1/complete"
      headers:
        X-Client: "adl-test"
      timeout_secs: 12
agents:
  a1:
    provider: "p1"
    model: "gpt-4o-mini"
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
    let provider = &expanded.providers["p1"];
    assert_eq!(provider.kind, "http");
    assert_eq!(provider.default_model.as_deref(), Some("gpt-4o-mini"));
    assert_eq!(
        provider.config.get("endpoint").and_then(|v| v.as_str()),
        Some("https://api.openai.com/v1/complete")
    );
    assert_eq!(
        provider.config.get("timeout_secs").and_then(|v| v.as_u64()),
        Some(12)
    );
}

#[test]
fn expand_provider_profiles_accepts_chatgpt_profile_with_endpoint_override() {
    let doc = adl_doc_from_yaml(
        r#"
version: "0.5"
providers:
  p1:
    profile: "chatgpt:gpt-5.4"
    config:
      endpoint: "https://api.openai.com/v1/complete"
      auth:
        type: "bearer"
        env: "OPENAI_API_KEY"
      timeout_secs: 20
agents:
  a1:
    provider: "p1"
    model: "gpt-5.4"
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
    let provider = &expanded.providers["p1"];
    assert_eq!(provider.kind, "http");
    assert_eq!(provider.profile.as_deref(), Some("chatgpt:gpt-5.4"));
    assert_eq!(provider.default_model.as_deref(), Some("gpt-5.4"));
    assert_eq!(
        provider.config.get("endpoint").and_then(|v| v.as_str()),
        Some("https://api.openai.com/v1/complete")
    );
    assert_eq!(
        provider
            .config
            .get("auth")
            .and_then(|v| v.get("env"))
            .and_then(|v| v.as_str()),
        Some("OPENAI_API_KEY")
    );
}

#[test]
fn expand_provider_profiles_accepts_claude_profile_with_endpoint_override() {
    let doc = adl_doc_from_yaml(
        r#"
version: "0.5"
providers:
  p1:
    profile: "claude:claude-3-7-sonnet"
    config:
      endpoint: "https://api.anthropic.com/v1/complete"
      auth:
        type: "bearer"
        env: "ANTHROPIC_API_KEY"
      timeout_secs: 20
agents:
  a1:
    provider: "p1"
    model: "claude-3-7-sonnet"
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
    let provider = &expanded.providers["p1"];
    assert_eq!(provider.kind, "http");
    assert_eq!(
        provider.profile.as_deref(),
        Some("claude:claude-3-7-sonnet")
    );
    assert_eq!(
        provider.default_model.as_deref(),
        Some("claude-3-7-sonnet-latest")
    );
    assert_eq!(
        provider.config.get("endpoint").and_then(|v| v.as_str()),
        Some("https://api.anthropic.com/v1/complete")
    );
    assert_eq!(
        provider
            .config
            .get("auth")
            .and_then(|v| v.get("env"))
            .and_then(|v| v.as_str()),
        Some("ANTHROPIC_API_KEY")
    );
}

#[test]
fn provider_profile_names_include_chatgpt_family() {
    let names = provider_profile_names();
    for required in [
        "chatgpt:gpt-5.4",
        "chatgpt:gpt-5.4-mini",
        "chatgpt:gpt-5.3-codex",
        "chatgpt:gpt-5.2",
    ] {
        assert!(
            names.iter().any(|name| name == required),
            "missing provider profile {required}"
        );
    }
}

#[test]
fn provider_profile_names_include_claude_family() {
    let names = provider_profile_names();
    for required in ["claude:claude-3-7-sonnet", "claude:claude-3-5-haiku"] {
        assert!(
            names.iter().any(|name| name == required),
            "missing provider profile {required}"
        );
    }
}

#[test]
fn resolve_run_accepts_http_profile_with_valid_endpoint_override() {
    let doc = adl_doc_from_yaml(
        r#"
version: "0.5"
providers:
  p1:
    profile: "http:gpt-4o-mini"
    config:
      endpoint: "https://api.openai.com/v1/complete"
agents:
  a1:
    provider: "p1"
    model: "reasoning/default"
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
    let resolved = ::adl::resolve::resolve_run(&doc).expect("valid endpoint should pass resolve");
    assert_eq!(
        resolved.steps.len(),
        1,
        "expected exactly one resolved step"
    );
    assert_eq!(resolved.doc.providers["p1"].kind, "http");
}
