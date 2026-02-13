use std::fs;
use std::io;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use swarm::adl;
use swarm::provider::{build_provider, OllamaProvider};

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
    // external binaries and ambient environment (e.g., SWARM_TIMEOUT_SECS), which can
    // make the test flaky under parallel execution. We only verify construction.
    let _p = build_provider(&spec, None).expect("build_provider failed");
}

#[test]
fn provider_complete_uses_mock_binary_success() {
    let dir = unique_test_temp_dir("swarm-provider-tests");
    let bin = make_mock_ollama_success(&dir).unwrap();

    let _env_guard = EnvVarGuard::set("SWARM_OLLAMA_BIN", &bin);

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
    let dir = unique_test_temp_dir("swarm-provider-tests");
    let bin = make_mock_ollama_failure(&dir).unwrap();

    let _env_guard = EnvVarGuard::set("SWARM_OLLAMA_BIN", &bin);

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
        msg.contains("ollama run failed"),
        "expected failure to mention ollama run failed, got: {msg}"
    );
    assert!(
        msg.contains("something went wrong"),
        "expected stderr to be included, got: {msg}"
    );

    let _ = fs::remove_dir_all(dir);
}

#[test]
fn env_var_guard_restores_previous_value() {
    let key = "SWARM_TEST_ENV_GUARD_RESTORE";
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
    let dir = unique_test_temp_dir("swarm-provider-timeout");
    let bin = make_mock_ollama_sleep(&dir).unwrap();

    let _env_guard = EnvVarGuard::set_many(&[
        ("SWARM_OLLAMA_BIN", bin.as_os_str()),
        ("SWARM_TIMEOUT_SECS", std::ffi::OsStr::new("1")),
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
    let dir = unique_test_temp_dir("swarm-provider-bad-timeout");
    let bin = make_mock_ollama_success(&dir).unwrap();

    let _env_guard = EnvVarGuard::set_many(&[
        ("SWARM_OLLAMA_BIN", bin.as_os_str()),
        ("SWARM_TIMEOUT_SECS", std::ffi::OsStr::new("nope")),
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
        msg.contains("SWARM_TIMEOUT_SECS") && msg.contains("invalid"),
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
        msg.contains("non-200") && msg.contains("500"),
        "unexpected error: {msg}"
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
