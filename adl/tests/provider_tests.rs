use std::fs;
use std::io;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use ::adl::adl;
use ::adl::provider::{
    build_provider, expand_provider_profiles, is_retryable_error, provider_profile_names,
    stable_failure_kind, OllamaProvider,
};

mod helpers;
use helpers::{unique_test_temp_dir, EnvVarGuard};

fn write_executable(path: &Path, contents: &str) -> io::Result<()> {
    fs::write(path, contents)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(path)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(path, perms)?;
    }
    Ok(())
}

fn block_incoming_localhost() -> EnvVarGuard {
    let key = "NO_PROXY";
    let old = std::env::var(key).ok();
    let mut new_val = old.clone().unwrap_or_default();
    if !new_val.is_empty() && !new_val.ends_with(',') {
        new_val.push(',');
    }
    new_val.push_str("127.0.0.1,localhost");
    EnvVarGuard::set(key, new_val)
}

fn make_mock_ollama_success(dir: &Path) -> io::Result<PathBuf> {
    let bin = dir.join("mock_ollama_ok.sh");
    // Mimic: `ollama run <model>` and read prompt from stdin.
    // We ignore args but verify shape is reasonable.
    let script = r#"#!/bin/sh
set -eu
# Expect: run <model>
if [ "${1:-}" != "run" ]; then
  echo "expected arg1=run, got '${1:-}'" 1>&2
  exit 2
fi
if [ -z "${2:-}" ]; then
  echo "expected model arg2" 1>&2
  exit 2
fi
# Consume stdin (the prompt)
cat >/dev/null
# Emit a deterministic response
echo "MOCK_COMPLETION_OK"
"#;
    write_executable(&bin, script)?;
    Ok(bin)
}

fn make_mock_ollama_failure(dir: &Path) -> io::Result<PathBuf> {
    let bin = dir.join("mock_ollama_fail.sh");
    let script = r#"#!/bin/sh
set -eu
echo "something went wrong" 1>&2
exit 42
"#;
    write_executable(&bin, script)?;
    Ok(bin)
}

fn make_mock_ollama_sleep(dir: &Path) -> io::Result<PathBuf> {
    let bin = dir.join("mock_ollama_sleep.sh");
    let script = r#"#!/bin/sh
set -eu
sleep 2
echo "MOCK_COMPLETION_SLOW"
"#;
    write_executable(&bin, script)?;
    Ok(bin)
}

fn provider_spec_from_yaml(yaml: &str) -> adl::ProviderSpec {
    serde_yaml::from_str::<adl::ProviderSpec>(yaml).expect("failed to parse ProviderSpec YAML")
}

fn adl_doc_from_yaml(yaml: &str) -> adl::AdlDoc {
    serde_yaml::from_str::<adl::AdlDoc>(yaml).expect("failed to parse AdlDoc YAML")
}

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

#[test]
fn http_provider_happy_path() {
    let server = match std::net::TcpListener::bind("127.0.0.1:0") {
        Ok(s) => s,
        Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => return,
        Err(e) => panic!("failed to bind local test server: {e}"),
    };
    let addr = server.local_addr().unwrap();
    let _server_guard = block_incoming_localhost();

    std::thread::spawn(move || {
        let (mut stream, _) = server.accept().unwrap();
        let mut buf = [0u8; 1024];
        let _ = stream.read(&mut buf);
        let body = r#"{"output":"OK"}"#;
        let resp = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
            body.len(),
            body
        );
        let _ = stream.write_all(resp.as_bytes());
    });

    let spec = provider_spec_from_yaml(&format!(
        r#"
type: http
config:
  endpoint: "http://{addr}/"
"#
    ));

    let p = build_provider(&spec, None).expect("build_provider failed");
    let out = p.complete("hello").expect("http provider should succeed");
    assert_eq!(out, "OK");
}

#[test]
fn http_provider_accepts_https_endpoint() {
    // This test is intentionally config-only: we verify that `https://` endpoints
    // are accepted by parsing/building the provider, without performing a network call.
    let spec = provider_spec_from_yaml(
        r#"
type: http
config:
  endpoint: "https://example.com/v1/complete"
"#,
    );

    let _p = build_provider(&spec, None).expect("build_provider should accept https endpoints");
}

#[test]
fn http_provider_rejects_plaintext_remote_endpoint() {
    let spec = provider_spec_from_yaml(
        r#"
type: http
config:
  endpoint: "http://api.example.com/v1/complete"
"#,
    );

    let err = match build_provider(&spec, None) {
        Ok(_) => panic!("plain remote http should fail"),
        Err(err) => err,
    };
    assert!(err
        .to_string()
        .contains("plaintext http:// is only allowed for localhost/loopback test endpoints"));
}

#[test]
fn http_provider_non_200_response() {
    let server = match std::net::TcpListener::bind("127.0.0.1:0") {
        Ok(s) => s,
        Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => return,
        Err(e) => panic!("failed to bind local test server: {e}"),
    };
    let addr = server.local_addr().unwrap();
    let _server_guard = block_incoming_localhost();

    std::thread::spawn(move || {
        let (mut stream, _) = server.accept().unwrap();
        let mut buf = [0u8; 1024];
        let _ = stream.read(&mut buf);
        let body = "bad";
        let resp = format!(
            "HTTP/1.1 500 Internal Server Error\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
            body.len(),
            body
        );
        let _ = stream.write_all(resp.as_bytes());
    });

    let spec = provider_spec_from_yaml(&format!(
        r#"
type: http
config:
  endpoint: "http://{addr}/"
"#
    ));

    let p = build_provider(&spec, None).expect("build_provider failed");
    let err = p.complete("hello").unwrap_err();
    let msg = format!("{err:#}");
    assert!(
        msg.contains("server_error") && msg.contains("500"),
        "unexpected error: {msg}"
    );
    assert!(
        is_retryable_error(&err),
        "5xx responses should be retryable: {msg}"
    );
}

#[test]
fn http_provider_long_error_body_is_truncated_deterministically() {
    let server = match std::net::TcpListener::bind("127.0.0.1:0") {
        Ok(s) => s,
        Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => return,
        Err(e) => panic!("failed to bind local test server: {e}"),
    };
    let addr = server.local_addr().unwrap();
    let _server_guard = block_incoming_localhost();

    std::thread::spawn(move || {
        let (mut stream, _) = server.accept().unwrap();
        let mut buf = [0u8; 1024];
        let _ = stream.read(&mut buf);
        let body = "x".repeat(300);
        let resp = format!(
            "HTTP/1.1 500 Internal Server Error\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
            body.len(),
            body
        );
        let _ = stream.write_all(resp.as_bytes());
    });

    let spec = provider_spec_from_yaml(&format!(
        r#"
type: http
config:
  endpoint: "http://{addr}/"
"#
    ));

    let p = build_provider(&spec, None).expect("build_provider failed");
    let err = p.complete("hello").expect_err("500 should fail");
    let msg = format!("{err:#}");
    assert!(
        msg.contains("kind=server_error"),
        "expected server_error classification, got: {msg}"
    );
    assert!(
        msg.contains("status=500"),
        "expected status code in message, got: {msg}"
    );
    assert!(
        msg.len() < 600,
        "response body should be truncated in error message, got len={}",
        msg.len()
    );
}

#[test]
fn http_provider_rejects_json_without_output_field() {
    let server = match std::net::TcpListener::bind("127.0.0.1:0") {
        Ok(s) => s,
        Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => return,
        Err(e) => panic!("failed to bind local test server: {e}"),
    };
    let addr = server.local_addr().unwrap();
    let _server_guard = block_incoming_localhost();

    std::thread::spawn(move || {
        let (mut stream, _) = server.accept().unwrap();
        let mut buf = [0u8; 1024];
        let _ = stream.read(&mut buf);
        let body = r#"{"not_output":"value"}"#;
        let resp = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
            body.len(),
            body
        );
        let _ = stream.write_all(resp.as_bytes());
    });

    let spec = provider_spec_from_yaml(&format!(
        r#"
type: http
config:
  endpoint: "http://{addr}/"
"#
    ));

    let p = build_provider(&spec, None).expect("build_provider failed");
    let err = p
        .complete("hello")
        .expect_err("response without output should fail");
    let msg = format!("{err:#}");
    assert!(
        msg.contains("response missing 'output' field"),
        "unexpected error: {msg}"
    );
    assert_eq!(stable_failure_kind(&err), Some("provider_error"));
}

#[test]
fn http_provider_4xx_is_non_retryable() {
    let server = match std::net::TcpListener::bind("127.0.0.1:0") {
        Ok(s) => s,
        Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => return,
        Err(e) => panic!("failed to bind local test server: {e}"),
    };
    let addr = server.local_addr().unwrap();
    let _server_guard = block_incoming_localhost();

    std::thread::spawn(move || {
        let (mut stream, _) = server.accept().unwrap();
        let mut buf = [0u8; 1024];
        let _ = stream.read(&mut buf);
        let body = "bad request";
        let resp = format!(
            "HTTP/1.1 400 Bad Request\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
            body.len(),
            body
        );
        let _ = stream.write_all(resp.as_bytes());
    });

    let spec = provider_spec_from_yaml(&format!(
        r#"
type: http
config:
  endpoint: "http://{addr}/"
"#
    ));

    let p = build_provider(&spec, None).expect("build_provider failed");
    let err = p.complete("hello").unwrap_err();
    let msg = format!("{err:#}");
    assert!(
        msg.contains("client_error") && msg.contains("400"),
        "unexpected error: {msg}"
    );
    assert!(
        !is_retryable_error(&err),
        "4xx responses should be non-retryable: {msg}"
    );
}

#[test]
fn http_provider_missing_auth_env_var() {
    let addr = "127.0.0.1:9";

    let spec = provider_spec_from_yaml(&format!(
        r#"
type: http
config:
  endpoint: "http://{addr}/"
  auth:
    type: bearer
    env: MISSING_ENV
"#
    ));

    let p = build_provider(&spec, None).expect("build_provider failed");
    let err = p.complete("hello").unwrap_err();
    let msg = format!("{err:#}");
    assert!(
        msg.contains("missing required auth env var"),
        "unexpected error: {msg}"
    );
    assert_eq!(
        stable_failure_kind(&err),
        Some("schema_error"),
        "missing auth env var should classify as schema_error"
    );
}

#[test]
fn http_provider_rejects_non_object_headers_and_non_string_values() {
    let non_object = provider_spec_from_yaml(
        r#"
type: http
config:
  endpoint: "http://127.0.0.1:9/"
  headers: "not-an-object"
"#,
    );
    let err = match build_provider(&non_object, None) {
        Ok(_) => panic!("non-object headers should fail"),
        Err(err) => err,
    };
    assert!(
        err.to_string().contains("config.headers must be an object"),
        "unexpected error: {err:#}"
    );

    let non_string_value = provider_spec_from_yaml(
        r#"
type: http
config:
  endpoint: "http://127.0.0.1:9/"
  headers:
    X-Number: 123
"#,
    );
    let err2 = match build_provider(&non_string_value, None) {
        Ok(_) => panic!("non-string header should fail"),
        Err(err) => err,
    };
    assert!(
        err2.to_string()
            .contains("config.headers values must be strings"),
        "unexpected error: {err2:#}"
    );
}

#[test]
fn http_provider_rejects_non_bearer_auth_type() {
    let spec = provider_spec_from_yaml(
        r#"
type: http
config:
  endpoint: "http://127.0.0.1:9/"
  auth:
    type: basic
    env: API_KEY
"#,
    );
    let err = match build_provider(&spec, None) {
        Ok(_) => panic!("non-bearer auth should fail"),
        Err(err) => err,
    };
    assert!(
        err.to_string()
            .contains("config.auth.type must be 'bearer'"),
        "unexpected error: {err:#}"
    );
}

#[test]
fn http_provider_supports_timeout_secs_string_and_rejects_negative_number() {
    let string_timeout = provider_spec_from_yaml(
        r#"
type: http
config:
  endpoint: "http://127.0.0.1:9/"
  timeout_secs: "7"
"#,
    );
    let _provider =
        build_provider(&string_timeout, None).expect("string timeout should parse as u64");

    let negative_timeout = provider_spec_from_yaml(
        r#"
type: http
config:
  endpoint: "http://127.0.0.1:9/"
  timeout_secs: -3
"#,
    );
    let _provider = build_provider(&negative_timeout, None)
        .expect("negative timeout should be treated as absent, not a parse failure");
}

#[test]
fn http_provider_rejects_missing_endpoint() {
    let spec = provider_spec_from_yaml(
        r#"
type: http
config: {}
"#,
    );

    let err = match build_provider(&spec, None) {
        Ok(_) => panic!("expected build_provider to fail for missing endpoint"),
        Err(err) => err,
    };
    let msg = format!("{err:#}");
    assert!(
        msg.contains("invalid config") && msg.contains("endpoint"),
        "unexpected error: {msg}"
    );
}

#[test]
fn http_provider_timeout() {
    let server = match std::net::TcpListener::bind("127.0.0.1:0") {
        Ok(s) => s,
        Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => return,
        Err(e) => panic!("failed to bind local test server: {e}"),
    };
    let addr = server.local_addr().unwrap();
    let _server_guard = block_incoming_localhost();

    std::thread::spawn(move || {
        let (mut stream, _) = server.accept().unwrap();
        let mut buf = [0u8; 1024];
        let _ = stream.read(&mut buf);
        std::thread::sleep(std::time::Duration::from_secs(2));
        let body = r#"{"output":"OK"}"#;
        let resp = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
            body.len(),
            body
        );
        let _ = stream.write_all(resp.as_bytes());
    });

    let spec = provider_spec_from_yaml(&format!(
        r#"
type: http
config:
  endpoint: "http://{addr}/"
  timeout_secs: 1
"#
    ));

    let p = build_provider(&spec, None).expect("build_provider failed");
    let err = p.complete("hello").unwrap_err();
    let msg = format!("{err:#}").to_lowercase();
    assert!(
        msg.contains("timeout") || msg.contains("timed out"),
        "unexpected error: {msg}"
    );
}

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
