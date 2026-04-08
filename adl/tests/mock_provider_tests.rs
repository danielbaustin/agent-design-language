use ::adl::adl;
use ::adl::provider::{build_provider, expand_provider_profiles};

fn provider_spec_from_yaml(yaml: &str) -> adl::ProviderSpec {
    serde_yaml::from_str::<adl::ProviderSpec>(yaml).expect("failed to parse ProviderSpec YAML")
}

fn adl_doc_from_yaml(yaml: &str) -> adl::AdlDoc {
    serde_yaml::from_str::<adl::AdlDoc>(yaml).expect("failed to parse AdlDoc YAML")
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
