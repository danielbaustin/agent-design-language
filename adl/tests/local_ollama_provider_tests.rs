use ::adl::adl;
use ::adl::provider::build_provider;

mod helpers;
use helpers::EnvVarGuard;

fn provider_spec_from_yaml(yaml: &str) -> adl::ProviderSpec {
    serde_yaml::from_str::<adl::ProviderSpec>(yaml).expect("failed to parse ProviderSpec YAML")
}

#[test]
fn build_provider_accepts_local_ollama_kind() {
    let spec = provider_spec_from_yaml(
        r#"
type: local_ollama
config:
  model: phi4-mini
"#,
    );

    let mock = format!("{}/tools/mock_ollama_v0_4.sh", env!("CARGO_MANIFEST_DIR"));
    let _env_guard = EnvVarGuard::set("ADL_OLLAMA_BIN", mock);

    let provider = build_provider(&spec, Some("phi4-mini"))
        .expect("build_provider should accept local_ollama");
    let output = provider
        .complete("LOCAL_OLLAMA_PROVIDER_TEST_OK")
        .expect("local_ollama provider should complete with mock binary");
    assert!(output.contains("LOCAL_OLLAMA_PROVIDER_TEST_OK"));
}
