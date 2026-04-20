use std::fs;

use ::adl::provider::build_provider;

use super::helpers::{unique_test_temp_dir, EnvVarGuard};
use super::support::{
    make_mock_ollama_failure, make_mock_ollama_sleep, make_mock_ollama_success,
    provider_spec_from_yaml,
};

#[test]
fn provider_complete_uses_mock_binary_success() {
    let dir = unique_test_temp_dir("adl-provider-tests");
    let bin = make_mock_ollama_success(&dir).unwrap();

    let _env_guard = EnvVarGuard::set("ADL_OLLAMA_BIN", &bin);

    let spec = provider_spec_from_yaml(
        r#"
type: ollama
config:
  model: llama3.1:8b
"#,
    );

    let p = build_provider(&spec, None).expect("build_provider failed");
    let out = p
        .complete("test prompt")
        .expect("complete() should succeed with mock");
    assert!(
        out.contains("MOCK_COMPLETION_OK"),
        "expected mock output, got: {out:?}"
    );

    let _ = fs::remove_dir_all(dir);
}

#[test]
fn provider_complete_surfaces_stderr_on_failure() {
    let dir = unique_test_temp_dir("adl-provider-tests");
    let bin = make_mock_ollama_failure(&dir).unwrap();

    let _env_guard = EnvVarGuard::set("ADL_OLLAMA_BIN", &bin);

    let spec = provider_spec_from_yaml(
        r#"
type: ollama
config:
  model: llama3.1:8b
"#,
    );

    let p = build_provider(&spec, None).expect("build_provider failed");
    let err = p.complete("test prompt").unwrap_err();
    let msg = format!("{err:#}");

    let mentions_launch_failure = msg.contains("ollama run failed");
    let mentions_stdin_failure = msg.contains("failed writing prompt to ollama stdin");
    assert!(
        mentions_launch_failure || mentions_stdin_failure,
        "expected failure to mention launch or stdin write failure, got: {msg}"
    );
    if mentions_launch_failure {
        assert!(
            msg.contains("something went wrong"),
            "expected stderr to be included on launch failure, got: {msg}"
        );
    }

    let _ = fs::remove_dir_all(dir);
}

#[test]
fn env_var_guard_restores_previous_value() {
    let key = "ADL_TEST_ENV_GUARD_RESTORE";
    let original = std::env::var_os(key);

    {
        let _guard = EnvVarGuard::set(key, "temporary");
        let val = std::env::var(key).expect("env var should be set");
        assert_eq!(val, "temporary");
    }

    {
        let _guard = EnvVarGuard::unset(key);
        assert!(std::env::var_os(key).is_none());
    }

    assert_eq!(std::env::var_os(key), original);
}

#[test]
fn provider_complete_times_out_with_env_override() {
    let dir = unique_test_temp_dir("adl-provider-timeout");
    let bin = make_mock_ollama_sleep(&dir).unwrap();

    let _env_guard = EnvVarGuard::set_many(&[
        ("ADL_OLLAMA_BIN", bin.as_os_str()),
        ("ADL_TIMEOUT_SECS", std::ffi::OsStr::new("1")),
    ]);

    let spec = provider_spec_from_yaml(
        r#"
type: ollama
config:
  model: llama3.1:8b
"#,
    );

    let p = build_provider(&spec, None).expect("build_provider failed");
    let err = p.complete("test prompt").unwrap_err();
    let msg = format!("{err:#}");
    assert!(
        msg.contains("timed out") && msg.contains("1"),
        "expected timeout error, got: {msg}"
    );

    let _ = fs::remove_dir_all(dir);
}

#[test]
fn provider_complete_rejects_invalid_timeout_env() {
    let dir = unique_test_temp_dir("adl-provider-bad-timeout");
    let bin = make_mock_ollama_success(&dir).unwrap();

    let _env_guard = EnvVarGuard::set_many(&[
        ("ADL_OLLAMA_BIN", bin.as_os_str()),
        ("ADL_TIMEOUT_SECS", std::ffi::OsStr::new("nope")),
    ]);

    let spec = provider_spec_from_yaml(
        r#"
type: ollama
config:
  model: llama3.1:8b
"#,
    );

    let p = build_provider(&spec, None).expect("build_provider failed");
    let err = p.complete("test prompt").unwrap_err();
    let msg = format!("{err:#}");
    assert!(
        msg.contains("ADL_TIMEOUT_SECS") && msg.contains("invalid"),
        "expected invalid config error, got: {msg}"
    );

    let _ = fs::remove_dir_all(dir);
}
