use std::collections::HashMap;
use std::fs;
use std::io::{Read, Write};
use std::path::Path;
use std::process::Command;
use swarm::execute::materialize_inputs;

mod helpers;
use helpers::{unique_test_temp_dir, EnvVarGuard};

fn tmp_dir(prefix: &str) -> std::path::PathBuf {
    unique_test_temp_dir(prefix)
}

fn write_file(dir: &Path, rel: &str, contents: &[u8]) -> std::path::PathBuf {
    let path = dir.join(rel);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(&path, contents).unwrap();
    path
}

#[test]
fn materialize_inputs_leaves_non_file_values_unchanged() {
    let base = tmp_dir("mat-unchanged");
    let mut inputs = HashMap::new();
    inputs.insert("doc_1".to_string(), "hello".to_string());

    let out = materialize_inputs(inputs, &base).unwrap();
    assert_eq!(out["doc_1"], "hello");
}

#[test]
fn materialize_inputs_reads_relative_path_against_base_dir() {
    let base = tmp_dir("mat-rel");
    write_file(&base, "docs/doc_1.txt", b"abc");

    let mut inputs = HashMap::new();
    inputs.insert("doc_1".to_string(), "@file:docs/doc_1.txt".to_string());

    let out = materialize_inputs(inputs, &base).unwrap();
    assert_eq!(out["doc_1"], "abc");
}

#[test]
fn materialize_inputs_accepts_quoted_paths() {
    let base = tmp_dir("mat-quoted");
    write_file(&base, "docs/doc_1.txt", b"abc");

    let mut inputs = HashMap::new();
    inputs.insert("doc_1".to_string(), "@file:'docs/doc_1.txt'".to_string());
    inputs.insert("doc_2".to_string(), "@file:\"docs/doc_1.txt\"".to_string());

    let out = materialize_inputs(inputs, &base).unwrap();
    assert_eq!(out["doc_1"], "abc");
    assert_eq!(out["doc_2"], "abc");
}

#[test]
fn materialize_inputs_rejects_empty_path() {
    let base = tmp_dir("mat-emptypath");
    let mut inputs = HashMap::new();
    inputs.insert("doc_1".to_string(), "@file:   ".to_string());

    let err = materialize_inputs(inputs, &base).unwrap_err();
    let msg = format!("{err:#}");
    assert!(msg.contains("empty path"), "msg was: {msg}");
    assert!(msg.contains("doc_1"), "msg was: {msg}");
}

#[test]
fn materialize_inputs_errors_on_missing_file() {
    let base = tmp_dir("mat-missing");
    let mut inputs = HashMap::new();
    inputs.insert("doc_1".to_string(), "@file:docs/nope.txt".to_string());

    let err = materialize_inputs(inputs, &base).unwrap_err();
    let msg = format!("{err:#}");
    assert!(msg.contains("failed to stat input file"), "msg was: {msg}");
    assert!(msg.contains("doc_1"), "msg was: {msg}");
}

#[test]
fn materialize_inputs_normalizes_windows_newlines() {
    let base = tmp_dir("mat-newlines");
    write_file(&base, "docs/doc_1.txt", b"line1\r\nline2\r\n");

    let mut inputs = HashMap::new();
    inputs.insert("doc_1".to_string(), "@file:docs/doc_1.txt".to_string());

    let out = materialize_inputs(inputs, &base).unwrap();
    assert_eq!(out["doc_1"], "line1\nline2\n");
}

#[test]
fn materialize_inputs_rejects_non_utf8() {
    let base = tmp_dir("mat-nonutf8");
    // Invalid UTF-8 byte sequence
    write_file(&base, "docs/bad.bin", &[0xff, 0xfe, 0xfd]);

    let mut inputs = HashMap::new();
    inputs.insert("doc_1".to_string(), "@file:docs/bad.bin".to_string());

    let err = materialize_inputs(inputs, &base).unwrap_err();
    let msg = format!("{err:#}");
    assert!(msg.contains("not valid UTF-8"), "msg was: {msg}");
    assert!(msg.contains("doc_1"), "msg was: {msg}");
}

#[test]
fn materialize_inputs_enforces_max_size() {
    let base = tmp_dir("mat-maxsize");
    // MAX is 512 KiB; create 513 KiB
    let big = vec![b'a'; 513 * 1024];
    write_file(&base, "docs/big.txt", &big);

    let mut inputs = HashMap::new();
    inputs.insert("doc_1".to_string(), "@file:docs/big.txt".to_string());

    let err = materialize_inputs(inputs, &base).unwrap_err();
    let msg = format!("{err:#}");
    assert!(msg.contains("too large"), "msg was: {msg}");
    assert!(msg.contains("doc_1"), "msg was: {msg}");
}

fn prepend_path(bin_dir: &Path) -> String {
    let old_path = std::env::var("PATH").ok();
    let mut new_path = bin_dir.to_string_lossy().to_string();
    if let Some(old) = &old_path {
        new_path.push(':');
        new_path.push_str(old);
    }
    new_path
}

fn write_mock_ollama(dir: &Path, behavior: MockOllamaBehavior) -> std::path::PathBuf {
    let bin = dir.join("ollama");

    // Simple shell mock:
    // - expects: ollama run <model>
    // - reads stdin (prompt) and prints a canned response
    // - can be configured to fail and emit stderr
    let script = match behavior {
        MockOllamaBehavior::Success => {
            r#"#!/bin/sh
set -eu
# Args: run <model>
if [ "${1:-}" != "run" ]; then
  echo "mock ollama: expected 'run'" 1>&2
  exit 2
fi
# read stdin but ignore content (still exercises piping)
cat >/dev/null
echo "• mock summary bullet one"
echo "• mock summary bullet two"
exit 0
"#
        }
        MockOllamaBehavior::Fail => {
            r#"#!/bin/sh
set -eu
cat >/dev/null
echo "mock ollama failure: boom" 1>&2
exit 42
"#
        }
        MockOllamaBehavior::EchoModel => {
            r#"#!/bin/sh
set -eu
if [ "${1:-}" != "run" ]; then
  echo "mock ollama: expected 'run'" 1>&2
  exit 2
fi
model="${2:-}"
if [ -z "${model}" ]; then
  echo "mock ollama: expected model arg2" 1>&2
  exit 2
fi
cat >/dev/null
echo "MODEL=${model}"
exit 0
"#
        }
        MockOllamaBehavior::EchoPrompt => {
            r#"#!/bin/sh
set -eu
if [ "${1:-}" != "run" ]; then
  echo "mock ollama: expected 'run'" 1>&2
  exit 2
fi
cat
exit 0
"#
        }
        MockOllamaBehavior::SleepEchoPrompt => {
            r#"#!/bin/sh
set -eu
if [ "${1:-}" != "run" ]; then
  echo "mock ollama: expected 'run'" 1>&2
  exit 2
fi
sleep 1
cat
exit 0
"#
        }
        MockOllamaBehavior::FailOnce => {
            r#"#!/bin/sh
set -eu
state_file="${0}.state"
if [ ! -f "${state_file}" ]; then
  echo "mock ollama first attempt failure" 1>&2
  touch "${state_file}"
  exit 42
fi
cat >/dev/null
echo "MOCK_RECOVERED"
exit 0
"#
        }
        MockOllamaBehavior::FailOnToken => {
            r#"#!/bin/sh
set -eu
if [ "${1:-}" != "run" ]; then
  echo "mock ollama: expected 'run'" 1>&2
  exit 2
fi
prompt="$(cat)"
if printf "%s" "${prompt}" | grep -q "FAIL_THIS_STEP"; then
  echo "mock ollama forced fail token seen" 1>&2
  exit 41
fi
echo "MOCK_CONTINUE_OK"
exit 0
"#
        }
        MockOllamaBehavior::SleepTrackConcurrency => {
            r#"#!/bin/sh
set -eu
if [ "${1:-}" != "run" ]; then
  echo "mock ollama: expected 'run'" 1>&2
  exit 2
fi
cat >/dev/null
sleep 1
echo "MOCK_SLEEP_OK"
exit 0
"#
        }
    };

    fs::write(&bin, script.as_bytes()).unwrap();

    // chmod +x
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&bin).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&bin, perms).unwrap();
    }

    bin
}

#[derive(Clone, Copy)]
enum MockOllamaBehavior {
    Success,
    Fail,
    EchoModel,
    EchoPrompt,
    SleepEchoPrompt,
    FailOnce,
    FailOnToken,
    SleepTrackConcurrency,
}

fn run_swarm(args: &[&str]) -> std::process::Output {
    let exe = env!("CARGO_BIN_EXE_swarm");
    Command::new(exe).args(args).output().unwrap()
}

#[test]
fn run_executes_example_with_mock_ollama_and_prints_step_output() {
    let base = tmp_dir("exec-run-mock-ollama");
    let _bin = write_mock_ollama(&base, MockOllamaBehavior::Success);
    let new_path = prepend_path(&base);
    let _path_guard = EnvVarGuard::set("PATH", new_path);

    let out = run_swarm(&["examples/adl-0.1.yaml", "--run"]);
    assert!(
        out.status.success(),
        "expected success, got {:?}\nstderr:\n{}",
        out.status.code(),
        String::from_utf8_lossy(&out.stderr)
    );

    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("--- step: summarize_relevant_docs ---"),
        "stdout was:\n{stdout}"
    );
    assert!(
        stdout.contains("mock summary bullet one"),
        "stdout was:\n{stdout}"
    );
}

#[test]
fn run_honors_agent_model_over_provider_model() {
    let base = tmp_dir("exec-agent-model-override");
    let _bin = write_mock_ollama(&base, MockOllamaBehavior::EchoModel);
    let new_path = prepend_path(&base);
    let _path_guard = EnvVarGuard::set("PATH", new_path);

    let yaml = r#"
version: "0.2"

providers:
  local:
    type: "ollama"
    config:
      model: "provider-model"

agents:
  a1:
    provider: "local"
    model: "agent-model-91"

tasks:
  t1:
    prompt:
      user: "Summarize: {{text}}"

run:
  name: "agent-model-override"
  workflow:
    kind: "sequential"
    steps:
      - id: "s1"
        agent: "a1"
        task: "t1"
        inputs:
          text: "hello"
"#;

    let tmp_yaml = base.join("agent-model-override.yaml");
    fs::write(&tmp_yaml, yaml.as_bytes()).unwrap();

    let out = run_swarm(&[tmp_yaml.to_string_lossy().as_ref(), "--run"]);
    assert!(
        out.status.success(),
        "expected success, got {:?}\nstderr:\n{}",
        out.status.code(),
        String::from_utf8_lossy(&out.stderr)
    );

    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("MODEL=agent-model-91"),
        "stdout was:\n{stdout}"
    );
}

#[test]
fn run_on_error_continue_proceeds_after_failure() {
    let base = tmp_dir("exec-on-error-continue");
    let _bin = write_mock_ollama(&base, MockOllamaBehavior::FailOnToken);
    let new_path = prepend_path(&base);
    let _path_guard = EnvVarGuard::set("PATH", new_path);

    let yaml = r#"
version: "0.3"

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
  t_fail:
    prompt:
      user: "FAIL_THIS_STEP {{text}}"
  t_ok:
    prompt:
      user: "SAFE_STEP {{text}}"

run:
  name: "on-error-continue"
  workflow:
    kind: "sequential"
    steps:
      - id: "s_fail"
        agent: "a1"
        task: "t_fail"
        on_error: "continue"
        inputs:
          text: "x"
      - id: "s_ok"
        agent: "a1"
        task: "t_ok"
        inputs:
          text: "y"
"#;

    let tmp_yaml = base.join("on-error-continue.yaml");
    fs::write(&tmp_yaml, yaml.as_bytes()).unwrap();

    let out = run_swarm(&[tmp_yaml.to_string_lossy().as_ref(), "--run"]);
    assert!(
        out.status.success(),
        "expected success with continue policy, got {:?}\nstdout:\n{}\nstderr:\n{}",
        out.status.code(),
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("step=s_fail") && stdout.contains("status=failure"),
        "stdout was:\n{stdout}"
    );
    assert!(
        stdout.contains("step=s_ok") && stdout.contains("status=success"),
        "stdout was:\n{stdout}"
    );
}

#[test]
fn run_retry_succeeds_on_second_attempt() {
    let base = tmp_dir("exec-retry-success");
    let _bin = write_mock_ollama(&base, MockOllamaBehavior::FailOnce);
    let new_path = prepend_path(&base);
    let _path_guard = EnvVarGuard::set("PATH", new_path);

    let yaml = r#"
version: "0.3"

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
      user: "retry me {{text}}"

run:
  name: "retry-success"
  workflow:
    kind: "sequential"
    steps:
      - id: "s1"
        agent: "a1"
        task: "t1"
        retry:
          max_attempts: 2
        inputs:
          text: "hello"
"#;

    let tmp_yaml = base.join("retry-success.yaml");
    fs::write(&tmp_yaml, yaml.as_bytes()).unwrap();

    let out = run_swarm(&[tmp_yaml.to_string_lossy().as_ref(), "--run"]);
    assert!(
        out.status.success(),
        "expected success after retry, got {:?}\nstdout:\n{}\nstderr:\n{}",
        out.status.code(),
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("step=s1")
            && stdout.contains("attempts=2")
            && stdout.contains("status=success"),
        "stdout was:\n{stdout}"
    );
}

#[test]
fn run_retry_exhausts_and_fails() {
    let base = tmp_dir("exec-retry-exhaust");
    let _bin = write_mock_ollama(&base, MockOllamaBehavior::Fail);
    let new_path = prepend_path(&base);
    let _path_guard = EnvVarGuard::set("PATH", new_path);

    let yaml = r#"
version: "0.3"

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
      user: "retry me {{text}}"

run:
  name: "retry-exhaust"
  workflow:
    kind: "sequential"
    steps:
      - id: "s1"
        agent: "a1"
        task: "t1"
        retry:
          max_attempts: 2
        inputs:
          text: "hello"
"#;

    let tmp_yaml = base.join("retry-exhaust.yaml");
    fs::write(&tmp_yaml, yaml.as_bytes()).unwrap();

    let out = run_swarm(&[tmp_yaml.to_string_lossy().as_ref(), "--run"]);
    assert!(
        !out.status.success(),
        "expected failure when retries exhausted; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("failed after 2 attempt(s)") || stderr.contains("attempt 2/2"),
        "stderr was:\n{stderr}"
    );
}

#[test]
fn run_executes_step_with_http_provider() {
    let server = match std::net::TcpListener::bind("127.0.0.1:0") {
        Ok(s) => s,
        Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => return,
        Err(e) => panic!("failed to bind local test server: {e}"),
    };
    let addr = server.local_addr().unwrap();

    let old_no_proxy = std::env::var("NO_PROXY").ok();
    let mut no_proxy_val = old_no_proxy.unwrap_or_default();
    if !no_proxy_val.is_empty() && !no_proxy_val.ends_with(',') {
        no_proxy_val.push(',');
    }
    no_proxy_val.push_str("127.0.0.1,localhost");

    // Set both env vars under one guard to avoid nested env-lock acquisition.
    let _env_guard = EnvVarGuard::set_many(&[
        ("NO_PROXY", std::ffi::OsStr::new(&no_proxy_val)),
        (
            "SWARM_REMOTE_BEARER_TOKEN",
            std::ffi::OsStr::new("demo-token"),
        ),
    ]);

    std::thread::spawn(move || {
        let (mut stream, _) = server.accept().unwrap();
        let mut buf = [0u8; 4096];
        let _ = stream.read(&mut buf);
        let body = r#"{"output":"REMOTE_DEMO_OK"}"#;
        let resp = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
            body.len(),
            body
        );
        let _ = stream.write_all(resp.as_bytes());
    });

    let base = tmp_dir("exec-remote-demo");
    let yaml_src = fs::read_to_string("examples/v0-3-remote-http-provider.adl.yaml").unwrap();
    let yaml = yaml_src.replace(
        "http://127.0.0.1:8787/complete",
        &format!("http://{addr}/complete"),
    );
    let tmp_yaml = base.join("remote-http-provider.adl.yaml");
    fs::write(&tmp_yaml, yaml.as_bytes()).unwrap();

    let out = run_swarm(&[tmp_yaml.to_string_lossy().as_ref(), "--run"]);
    assert!(
        out.status.success(),
        "expected success, got {:?}\nstdout:\n{}\nstderr:\n{}",
        out.status.code(),
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );

    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("--- step: remote_summary ---"),
        "stdout was:\n{stdout}"
    );
    assert!(stdout.contains("REMOTE_DEMO_OK"), "stdout was:\n{stdout}");
}

#[test]
fn run_http_retry_succeeds_on_second_attempt_after_5xx() {
    let server = match std::net::TcpListener::bind("127.0.0.1:0") {
        Ok(s) => s,
        Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => return,
        Err(e) => panic!("failed to bind local test server: {e}"),
    };
    let addr = server.local_addr().unwrap();

    let old_no_proxy = std::env::var("NO_PROXY").ok();
    let mut no_proxy_val = old_no_proxy.unwrap_or_default();
    if !no_proxy_val.is_empty() && !no_proxy_val.ends_with(',') {
        no_proxy_val.push(',');
    }
    no_proxy_val.push_str("127.0.0.1,localhost");
    let _env_guard = EnvVarGuard::set("NO_PROXY", no_proxy_val);

    std::thread::spawn(move || {
        for idx in 0..2 {
            let (mut stream, _) = server.accept().unwrap();
            let mut buf = [0u8; 4096];
            let _ = stream.read(&mut buf);
            if idx == 0 {
                let body = "upstream overloaded";
                let resp = format!(
                    "HTTP/1.1 500 Internal Server Error\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                    body.len(),
                    body
                );
                let _ = stream.write_all(resp.as_bytes());
            } else {
                let body = r#"{"output":"RECOVERED_200"}"#;
                let resp = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
                    body.len(),
                    body
                );
                let _ = stream.write_all(resp.as_bytes());
            }
        }
    });

    let base = tmp_dir("exec-http-retry-5xx");
    let yaml = format!(
        r#"
version: "0.3"

providers:
  remote_http:
    type: "http"
    config:
      endpoint: "http://{addr}/complete"

agents:
  writer:
    provider: "remote_http"
    model: "unused"

tasks:
  summarize:
    prompt:
      user: "Hello {{topic}}"

run:
  name: "http-retry-5xx"
  workflow:
    kind: "sequential"
    steps:
      - id: "remote_summary"
        agent: "writer"
        task: "summarize"
        retry:
          max_attempts: 2
        inputs:
          topic: "adl"
"#
    );
    let tmp_yaml = base.join("http-retry-5xx.yaml");
    fs::write(&tmp_yaml, yaml.as_bytes()).unwrap();

    let out = run_swarm(&[tmp_yaml.to_string_lossy().as_ref(), "--run"]);
    assert!(
        out.status.success(),
        "expected success after retry, got {:?}\nstdout:\n{}\nstderr:\n{}",
        out.status.code(),
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("RECOVERED_200") && stdout.contains("attempts=2"),
        "stdout was:\n{stdout}"
    );
}

#[test]
fn run_http_4xx_does_not_retry_even_with_retry_policy() {
    let server = match std::net::TcpListener::bind("127.0.0.1:0") {
        Ok(s) => s,
        Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => return,
        Err(e) => panic!("failed to bind local test server: {e}"),
    };
    let addr = server.local_addr().unwrap();

    let old_no_proxy = std::env::var("NO_PROXY").ok();
    let mut no_proxy_val = old_no_proxy.unwrap_or_default();
    if !no_proxy_val.is_empty() && !no_proxy_val.ends_with(',') {
        no_proxy_val.push(',');
    }
    no_proxy_val.push_str("127.0.0.1,localhost");
    let _env_guard = EnvVarGuard::set("NO_PROXY", no_proxy_val);

    std::thread::spawn(move || {
        let (mut stream, _) = server.accept().unwrap();
        let mut buf = [0u8; 4096];
        let _ = stream.read(&mut buf);
        let body = "invalid request";
        let resp = format!(
            "HTTP/1.1 400 Bad Request\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
            body.len(),
            body
        );
        let _ = stream.write_all(resp.as_bytes());
    });

    let base = tmp_dir("exec-http-no-retry-4xx");
    let yaml = format!(
        r#"
version: "0.3"

providers:
  remote_http:
    type: "http"
    config:
      endpoint: "http://{addr}/complete"

agents:
  writer:
    provider: "remote_http"
    model: "unused"

tasks:
  summarize:
    prompt:
      user: "Hello {{topic}}"

run:
  name: "http-no-retry-4xx"
  workflow:
    kind: "sequential"
    steps:
      - id: "remote_summary"
        agent: "writer"
        task: "summarize"
        retry:
          max_attempts: 3
        inputs:
          topic: "adl"
"#
    );
    let tmp_yaml = base.join("http-no-retry-4xx.yaml");
    fs::write(&tmp_yaml, yaml.as_bytes()).unwrap();

    let out = run_swarm(&[tmp_yaml.to_string_lossy().as_ref(), "--run"]);
    assert!(
        !out.status.success(),
        "expected non-retryable 4xx failure; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("failed after 1 attempt(s)") && stderr.contains("client_error"),
        "stderr was:\n{stderr}"
    );
}

#[test]
fn run_http_timeout_retries_until_exhausted() {
    let server = match std::net::TcpListener::bind("127.0.0.1:0") {
        Ok(s) => s,
        Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => return,
        Err(e) => panic!("failed to bind local test server: {e}"),
    };
    let addr = server.local_addr().unwrap();

    let old_no_proxy = std::env::var("NO_PROXY").ok();
    let mut no_proxy_val = old_no_proxy.unwrap_or_default();
    if !no_proxy_val.is_empty() && !no_proxy_val.ends_with(',') {
        no_proxy_val.push(',');
    }
    no_proxy_val.push_str("127.0.0.1,localhost");
    let _env_guard = EnvVarGuard::set("NO_PROXY", no_proxy_val);

    std::thread::spawn(move || {
        for _ in 0..2 {
            let (mut stream, _) = server.accept().unwrap();
            let mut buf = [0u8; 4096];
            let _ = stream.read(&mut buf);
            std::thread::sleep(std::time::Duration::from_secs(2));
            let body = r#"{"output":"TOO_LATE"}"#;
            let resp = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
                body.len(),
                body
            );
            let _ = stream.write_all(resp.as_bytes());
        }
    });

    let base = tmp_dir("exec-http-timeout-retry");
    let yaml = format!(
        r#"
version: "0.3"

providers:
  remote_http:
    type: "http"
    config:
      endpoint: "http://{addr}/complete"
      timeout_secs: 1

agents:
  writer:
    provider: "remote_http"
    model: "unused"

tasks:
  summarize:
    prompt:
      user: "Hello {{topic}}"

run:
  name: "http-timeout-retry"
  workflow:
    kind: "sequential"
    steps:
      - id: "remote_summary"
        agent: "writer"
        task: "summarize"
        retry:
          max_attempts: 2
        inputs:
          topic: "adl"
"#
    );
    let tmp_yaml = base.join("http-timeout-retry.yaml");
    fs::write(&tmp_yaml, yaml.as_bytes()).unwrap();

    let out = run_swarm(&[tmp_yaml.to_string_lossy().as_ref(), "--run"]);
    assert!(
        !out.status.success(),
        "expected timeout retry exhaustion; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    let stderr = String::from_utf8_lossy(&out.stderr).to_lowercase();
    assert!(
        stderr.contains("failed after 2 attempt(s)") && stderr.contains("timed out"),
        "stderr was:\n{stderr}"
    );
}

#[test]
fn run_v0_2_coordinator_example_uses_real_file_handoff() {
    let base = tmp_dir("exec-coordinator-file-handoff");
    let _bin = write_mock_ollama(&base, MockOllamaBehavior::EchoPrompt);
    let new_path = prepend_path(&base);
    let _path_guard = EnvVarGuard::set("PATH", new_path);

    let yaml = fs::read_to_string("examples/v0-2-coordinator-agents-sdk.adl.yaml").unwrap();
    let yaml_path = base.join("coordinator.adl.yaml");
    fs::write(&yaml_path, yaml.as_bytes()).unwrap();

    let out_dir = base.join("out");
    let out = run_swarm(&[
        yaml_path.to_string_lossy().as_ref(),
        "--run",
        "--out",
        out_dir.to_string_lossy().as_ref(),
    ]);

    assert!(
        out.status.success(),
        "expected success, got {:?}\nstderr:\n{}",
        out.status.code(),
        String::from_utf8_lossy(&out.stderr)
    );

    let brief = fs::read_to_string(out_dir.join("state/brief.txt")).unwrap();
    let design = fs::read_to_string(out_dir.join("state/design.txt")).unwrap();

    assert!(
        brief.contains("BRIEF_STATE:"),
        "brief artifact was:\n{}",
        brief
    );
    assert!(
        design.contains("DESIGN_FROM_FILE=") && design.contains("BRIEF_STATE:"),
        "design artifact was:\n{}",
        design
    );

    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("IMPLEMENTATION_FROM_FILE=") && stdout.contains("DESIGN_FROM_FILE="),
        "stdout was:\n{}",
        stdout
    );
}

#[test]
fn run_with_trace_emits_trace_header_and_events() {
    let base = tmp_dir("exec-run-trace-mock-ollama");
    let _bin = write_mock_ollama(&base, MockOllamaBehavior::Success);
    let new_path = prepend_path(&base);
    let _path_guard = EnvVarGuard::set("PATH", new_path);

    let out = run_swarm(&["examples/adl-0.1.yaml", "--run", "--trace"]);
    assert!(
        out.status.success(),
        "expected success, got {:?}\nstderr:\n{}",
        out.status.code(),
        String::from_utf8_lossy(&out.stderr)
    );

    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("TRACE run_id=") && stdout.contains("workflow_id="),
        "stdout was:\n{stdout}"
    );
    assert!(stdout.contains("StepStarted"), "stdout was:\n{stdout}");
    assert!(stdout.contains("PromptAssembled"), "stdout was:\n{stdout}");
    assert!(stdout.contains("StepFinished"), "stdout was:\n{stdout}");
}

#[test]
fn run_rejects_concurrent_workflows_in_v0_1() {
    // Even though we expect to fail before executing the provider, install a mock
    // `ollama` to keep the test hermetic if execution order changes.
    let base = tmp_dir("exec-reject-concurrent");
    let _bin = write_mock_ollama(&base, MockOllamaBehavior::Success);
    let new_path = prepend_path(&base);
    let _path_guard = EnvVarGuard::set("PATH", new_path);

    // Minimal doc that would otherwise run, but uses a concurrent workflow.
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
  name: "reject-concurrent"
  workflow:
    kind: "concurrent"
    steps:
      - id: "s1"
        agent: "a1"
        task: "t1"
        inputs:
          text: "hello"
"#;

    let tmp_yaml = base.join("concurrent.yaml");
    fs::write(&tmp_yaml, yaml.as_bytes()).unwrap();

    let out = run_swarm(&[tmp_yaml.to_string_lossy().as_ref(), "--run"]);
    assert!(
        !out.status.success(),
        "expected failure for concurrent workflow, got success.\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );

    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("concurrency"),
        "stderr should mention concurrency; stderr was:\n{stderr}"
    );
    assert!(
        stderr.contains("requires v0.3"),
        "stderr should mention required version; stderr was:\n{stderr}"
    );
    assert!(
        stderr.contains("document version is 0.1"),
        "stderr should include document version; stderr was:\n{stderr}"
    );
}

#[test]
fn run_rejects_concurrent_workflows_in_v0_2() {
    let base = tmp_dir("exec-reject-concurrent-v0-2");
    let _bin = write_mock_ollama(&base, MockOllamaBehavior::Success);
    let new_path = prepend_path(&base);
    let _path_guard = EnvVarGuard::set("PATH", new_path);
    let out = run_swarm(&["tests/fixtures/concurrent_v0_2.adl.yaml", "--run"]);
    assert!(
        !out.status.success(),
        "expected failure for concurrent workflow, got success.\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );

    let stderr = String::from_utf8_lossy(&out.stderr);
    let expected =
        "Error: feature 'concurrency' requires v0.3 workflows or v0.5 pattern runs; document version is 0.2 (run.workflow.kind=concurrent)";
    assert!(
        stderr.contains(expected),
        "stderr should contain expected error message, stderr was:\n{stderr}"
    );
}

#[test]
fn run_executes_concurrent_workflows_in_v0_3_in_lexicographic_step_id_order() {
    let base = tmp_dir("exec-concurrent-v0-3-order");
    let _bin = write_mock_ollama(&base, MockOllamaBehavior::Success);
    let new_path = prepend_path(&base);
    let _path_guard = EnvVarGuard::set("PATH", new_path);

    let yaml = r#"
version: "0.3"

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
  branch:
    prompt:
      user: "BRANCH={{branch}}"
  join:
    prompt:
      user: "JOIN A={{a}} B={{b}} C={{c}}"

run:
  name: "v0-3-lex-order"
  workflow:
    kind: "concurrent"
    steps:
      - id: "fork.branch.c"
        agent: "a1"
        task: "branch"
        save_as: "c"
        inputs:
          branch: "c"
      - id: "fork.branch.a"
        agent: "a1"
        task: "branch"
        save_as: "a"
        inputs:
          branch: "a"
      - id: "fork.branch.b"
        agent: "a1"
        task: "branch"
        save_as: "b"
        inputs:
          branch: "b"
      - id: "fork.join"
        agent: "a1"
        task: "join"
        save_as: "joined"
        inputs:
          a: "@state:a"
          b: "@state:b"
          c: "@state:c"
"#;
    let tmp_yaml = base.join("v0-3-lex-order.yaml");
    fs::write(&tmp_yaml, yaml.as_bytes()).unwrap();
    let out = run_swarm(&[tmp_yaml.to_string_lossy().as_ref(), "--run"]);
    assert!(
        out.status.success(),
        "expected success for v0.3 concurrent run, got failure.\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );

    let stdout = String::from_utf8_lossy(&out.stdout);
    let order = [
        "--- step: fork.branch.a ---",
        "--- step: fork.branch.b ---",
        "--- step: fork.branch.c ---",
        "--- step: fork.join ---",
    ];
    let mut cursor = 0usize;
    for marker in order {
        let Some(rel_idx) = stdout[cursor..].find(marker) else {
            panic!("missing marker '{marker}' in stdout:\n{stdout}");
        };
        cursor += rel_idx + marker.len();
    }
}

#[test]
fn run_v0_3_concurrency_example_writes_branch_and_join_artifacts() {
    let base = tmp_dir("exec-concurrent-v0-3-artifacts");
    let _bin = write_mock_ollama(&base, MockOllamaBehavior::EchoPrompt);
    let new_path = prepend_path(&base);
    let _path_guard = EnvVarGuard::set("PATH", new_path);

    let out_dir = base.join("out");
    let out = run_swarm(&[
        "examples/v0-3-concurrency-fork-join.adl.yaml",
        "--run",
        "--out",
        out_dir.to_string_lossy().as_ref(),
    ]);
    assert!(
        out.status.success(),
        "expected success for v0.3 concurrent run, got failure.\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );

    let alpha = fs::read_to_string(out_dir.join("fork/alpha.txt")).unwrap();
    let beta = fs::read_to_string(out_dir.join("fork/beta.txt")).unwrap();
    let join = fs::read_to_string(out_dir.join("fork/join.txt")).unwrap();

    assert!(
        alpha.contains("Process branch alpha"),
        "alpha artifact was:\n{alpha}"
    );
    assert!(
        beta.contains("Process branch beta"),
        "beta artifact was:\n{beta}"
    );
    assert!(
        join.contains("alpha=USER:")
            && join.contains("Process branch alpha")
            && join.contains("beta=USER:")
            && join.contains("Process branch beta"),
        "join artifact should reference both branch outputs:\n{join}"
    );
}

#[test]
fn run_v0_3_join_step_can_consume_saved_fork_outputs() {
    let base = tmp_dir("exec-concurrent-v0-3-join-state");
    let _bin = write_mock_ollama(&base, MockOllamaBehavior::EchoPrompt);
    let new_path = prepend_path(&base);
    let _path_guard = EnvVarGuard::set("PATH", new_path);

    let yaml = r#"
version: "0.3"

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
  branch:
    prompt:
      user: "BRANCH={{branch}} TOPIC={{topic}}"
  join:
    prompt:
      user: "JOIN A={{alpha}} B={{beta}}"

run:
  name: "v0-3-join-state"
  workflow:
    kind: "concurrent"
    steps:
      - id: "fork.branch.alpha"
        agent: "a1"
        task: "branch"
        save_as: "alpha"
        inputs:
          topic: "deterministic"
          branch: "alpha"
      - id: "fork.branch.beta"
        agent: "a1"
        task: "branch"
        save_as: "beta"
        inputs:
          topic: "deterministic"
          branch: "beta"
      - id: "fork.join"
        agent: "a1"
        task: "join"
        save_as: "joined"
        write_to: "join.txt"
        inputs:
          alpha: "@state:alpha"
          beta: "@state:beta"
"#;

    let tmp_yaml = base.join("v0-3-join-state.yaml");
    fs::write(&tmp_yaml, yaml.as_bytes()).unwrap();

    let out_dir = base.join("out");
    let out = run_swarm(&[
        tmp_yaml.to_string_lossy().as_ref(),
        "--run",
        "--out",
        out_dir.to_string_lossy().as_ref(),
    ]);
    assert!(
        out.status.success(),
        "expected success, got failure.\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );

    let joined = fs::read_to_string(out_dir.join("join.txt")).unwrap();
    assert!(
        joined.contains("BRANCH=alpha TOPIC=deterministic"),
        "join output missing alpha branch content:\n{joined}"
    );
    assert!(
        joined.contains("BRANCH=beta TOPIC=deterministic"),
        "join output missing beta branch content:\n{joined}"
    );
}

#[test]
fn run_v0_3_fails_fast_on_fork_failure_and_does_not_run_join() {
    let base = tmp_dir("exec-concurrent-v0-3-fail-fast");
    let _bin = write_mock_ollama(&base, MockOllamaBehavior::Success);
    let new_path = prepend_path(&base);
    let _path_guard = EnvVarGuard::set("PATH", new_path);

    let yaml = r#"
version: "0.3"

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
  ok_task:
    prompt:
      user: "OK {{value}}"
  broken_task:
    prompt:
      user: "BROKEN {{missing_key}}"
  join_task:
    prompt:
      user: "JOIN {{alpha}} {{beta}}"

run:
  name: "v0-3-fail-fast"
  workflow:
    kind: "concurrent"
    steps:
      - id: "fork.branch.alpha"
        agent: "a1"
        task: "ok_task"
        save_as: "alpha"
        inputs:
          value: "alpha"
      - id: "fork.branch.beta"
        agent: "a1"
        task: "broken_task"
        save_as: "beta"
        inputs:
          value: "beta"
      - id: "fork.join"
        agent: "a1"
        task: "join_task"
        save_as: "joined"
        inputs:
          alpha: "@state:alpha"
          beta: "@state:beta"
"#;

    let tmp_yaml = base.join("v0-3-fail-fast.yaml");
    fs::write(&tmp_yaml, yaml.as_bytes()).unwrap();

    let out = run_swarm(&[tmp_yaml.to_string_lossy().as_ref(), "--run", "--trace"]);
    assert!(
        !out.status.success(),
        "expected failure, got success.\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );

    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("fork.branch.beta") && stderr.contains("missing input bindings"),
        "stderr should identify failed branch; stderr was:\n{stderr}"
    );

    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        !stdout.contains("StepStarted step=fork.join"),
        "join step should not start after branch failure; stdout:\n{stdout}"
    );
}

#[test]
fn run_v0_3_fork_join_uses_bounded_executor_with_deterministic_join_barrier() {
    let base = tmp_dir("exec-concurrent-v0-3-bounded-join-trace");
    let _bin = write_mock_ollama(&base, MockOllamaBehavior::SleepEchoPrompt);
    let new_path = prepend_path(&base);
    let _path_guard = EnvVarGuard::set("PATH", new_path);

    let yaml = r#"
version: "0.3"

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
  branch:
    prompt:
      user: "BRANCH={{branch}}"
  join:
    prompt:
      user: "JOIN A={{a}} B={{b}} C={{c}} D={{d}} E={{e}}"

run:
  name: "v0-3-bounded-join-trace"
  workflow:
    kind: "concurrent"
    steps:
      - id: "fork.branch.c"
        agent: "a1"
        task: "branch"
        save_as: "c"
        inputs:
          branch: "c"
      - id: "fork.branch.a"
        agent: "a1"
        task: "branch"
        save_as: "a"
        inputs:
          branch: "a"
      - id: "fork.branch.e"
        agent: "a1"
        task: "branch"
        save_as: "e"
        inputs:
          branch: "e"
      - id: "fork.branch.b"
        agent: "a1"
        task: "branch"
        save_as: "b"
        inputs:
          branch: "b"
      - id: "fork.branch.d"
        agent: "a1"
        task: "branch"
        save_as: "d"
        inputs:
          branch: "d"
      - id: "fork.join"
        agent: "a1"
        task: "join"
        save_as: "joined"
        write_to: "join.txt"
        inputs:
          a: "@state:a"
          b: "@state:b"
          c: "@state:c"
          d: "@state:d"
          e: "@state:e"
"#;

    let tmp_yaml = base.join("v0-3-bounded-join-trace.yaml");
    fs::write(&tmp_yaml, yaml.as_bytes()).unwrap();

    let started = std::time::Instant::now();
    let out = run_swarm(&[tmp_yaml.to_string_lossy().as_ref(), "--run", "--trace"]);
    let elapsed = started.elapsed().as_secs_f64();

    assert!(
        out.status.success(),
        "expected success, got failure.\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(
        (2.5..=5.8).contains(&elapsed),
        "expected bounded runtime window for 5 forks + join with max_parallel=4, got {elapsed:.3}s"
    );

    let stdout = String::from_utf8_lossy(&out.stdout);
    let join_started = stdout
        .find("StepStarted step=fork.join")
        .expect("join step should start in trace output");
    for branch in ["a", "b", "c", "d", "e"] {
        let marker = format!("StepFinished step=fork.branch.{branch} success=true");
        let idx = stdout
            .find(&marker)
            .unwrap_or_else(|| panic!("missing marker '{marker}' in trace output:\n{stdout}"));
        assert!(
            idx < join_started,
            "join started before branch {branch} finished; stdout:\n{stdout}"
        );
    }
}

#[test]
fn run_v0_3_concurrent_execution_is_deterministic_across_runs() {
    let base = tmp_dir("exec-concurrent-v0-3-deterministic");
    let _bin = write_mock_ollama(&base, MockOllamaBehavior::Success);
    let new_path = prepend_path(&base);
    let _path_guard = EnvVarGuard::set("PATH", new_path);

    let out1 = run_swarm(&["examples/v0-3-concurrency-fork-join.adl.yaml", "--run"]);
    assert!(
        out1.status.success(),
        "first run failed.\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out1.stdout),
        String::from_utf8_lossy(&out1.stderr)
    );

    let out2 = run_swarm(&["examples/v0-3-concurrency-fork-join.adl.yaml", "--run"]);
    assert!(
        out2.status.success(),
        "second run failed.\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out2.stdout),
        String::from_utf8_lossy(&out2.stderr)
    );

    let s1 = String::from_utf8_lossy(&out1.stdout);
    let s2 = String::from_utf8_lossy(&out2.stdout);
    assert_eq!(s1, s2, "concurrent run output should be deterministic");
}

#[test]
fn run_v0_3_concurrent_workflow_respects_bounded_parallelism() {
    use std::time::Instant;

    let base = tmp_dir("exec-concurrent-v0-3-bounded");
    let _bin = write_mock_ollama(&base, MockOllamaBehavior::SleepTrackConcurrency);
    let new_path = prepend_path(&base);
    let _path_guard = EnvVarGuard::set("PATH", new_path);

    let yaml = r#"
version: "0.3"

providers:
  local:
    type: "ollama"
    config:
      model: "phi4-mini"

agents:
  a:
    provider: "local"
    model: "phi4-mini"

tasks:
  t:
    prompt:
      user: "work {{n}}"

run:
  name: "bounded-parallelism"
  workflow:
    kind: "concurrent"
    steps:
      - id: "s1"
        agent: "a"
        task: "t"
        inputs: { n: "1" }
      - id: "s2"
        agent: "a"
        task: "t"
        inputs: { n: "2" }
      - id: "s3"
        agent: "a"
        task: "t"
        inputs: { n: "3" }
      - id: "s4"
        agent: "a"
        task: "t"
        inputs: { n: "4" }
      - id: "s5"
        agent: "a"
        task: "t"
        inputs: { n: "5" }
      - id: "s6"
        agent: "a"
        task: "t"
        inputs: { n: "6" }
      - id: "s7"
        agent: "a"
        task: "t"
        inputs: { n: "7" }
      - id: "s8"
        agent: "a"
        task: "t"
        inputs: { n: "8" }
"#;

    let tmp_yaml = base.join("bounded-parallelism.yaml");
    fs::write(&tmp_yaml, yaml).unwrap();

    let started = Instant::now();
    let out = run_swarm(&[tmp_yaml.to_str().unwrap(), "--run"]);
    let elapsed = started.elapsed().as_secs_f64();
    assert!(
        out.status.success(),
        "expected success for bounded parallelism run.\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );

    assert!(
        (1.6..=4.5).contains(&elapsed),
        "expected bounded parallel runtime window (>=1.6s and <=4.5s), got {elapsed:.3}s"
    );
}

#[test]
fn run_reports_error_when_materialized_doc_is_missing() {
    let base = tmp_dir("exec-missing-doc");
    let _bin = write_mock_ollama(&base, MockOllamaBehavior::Success);
    let new_path = prepend_path(&base);
    let _path_guard = EnvVarGuard::set("PATH", new_path);

    // The example workflow may reference docs via either:
    // - @file:examples/docs/*.txt
    // - @file:docs/*.txt
    // We populate both locations under `base` so the test stays aligned with the
    // evolving example format, and then break exactly one path.
    let names = ["doc_1.txt", "doc_2.txt", "doc_3.txt"];

    fs::create_dir_all(base.join("examples/docs")).unwrap();
    fs::create_dir_all(base.join("docs")).unwrap();

    for name in names {
        let src = Path::new("examples/docs").join(name);
        fs::copy(&src, base.join("examples/docs").join(name)).unwrap();
        fs::copy(&src, base.join("docs").join(name)).unwrap();
    }

    // Copy the example yaml and break one file input path.
    let yaml_src = fs::read_to_string("examples/adl-0.1.yaml").unwrap();
    // Break doc_1 for either path style.
    let yaml_broken = yaml_src
        .replace(
            "@file:examples/docs/doc_1.txt",
            "@file:examples/docs/DOES_NOT_EXIST.txt",
        )
        .replace("@file:docs/doc_1.txt", "@file:docs/DOES_NOT_EXIST.txt");

    let tmp_yaml = base.join("adl-broken.yaml");
    fs::write(&tmp_yaml, yaml_broken.as_bytes()).unwrap();

    let out = run_swarm(&[tmp_yaml.to_string_lossy().as_ref(), "--run"]);
    assert!(
        !out.status.success(),
        "expected failure, got success.\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );

    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("failed to materialize inputs")
            || stderr.contains("failed to stat input file"),
        "stderr was:\n{stderr}"
    );
    assert!(stderr.contains("doc_1"), "stderr was:\n{stderr}");
}

#[test]
fn run_surfaces_provider_failure_stderr() {
    let base = tmp_dir("exec-provider-failure");
    let _bin = write_mock_ollama(&base, MockOllamaBehavior::Fail);
    let new_path = prepend_path(&base);
    let _path_guard = EnvVarGuard::set("PATH", new_path);

    let out = run_swarm(&["examples/adl-0.1.yaml", "--run"]);
    assert!(
        !out.status.success(),
        "expected failure, got success.\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );

    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("ollama run failed") || stderr.contains("mock ollama failure"),
        "stderr was:\n{stderr}"
    );
}

#[test]
fn run_writes_step_output_to_file() {
    let base = tmp_dir("exec-write-output");
    let _bin = write_mock_ollama(&base, MockOllamaBehavior::Success);
    let new_path = prepend_path(&base);
    let _path_guard = EnvVarGuard::set("PATH", new_path);

    let yaml = r#"
version: "0.2"

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
  name: "write-output"
  workflow:
    kind: "sequential"
    steps:
      - id: "s1"
        agent: "a1"
        task: "t1"
        inputs:
          text: "hello"
        save_as: "summary"
        write_to: "index.html"
"#;

    let tmp_yaml = base.join("write-output.yaml");
    fs::write(&tmp_yaml, yaml.as_bytes()).unwrap();

    let out_dir = base.join("out");
    let out = run_swarm(&[
        tmp_yaml.to_string_lossy().as_ref(),
        "--run",
        "--out",
        out_dir.to_string_lossy().as_ref(),
    ]);
    assert!(
        out.status.success(),
        "expected success, got {:?}\nstderr:\n{}",
        out.status.code(),
        String::from_utf8_lossy(&out.stderr)
    );

    let output_path = out_dir.join("index.html");
    let contents = fs::read_to_string(&output_path).unwrap();
    assert!(
        contents.contains("mock summary bullet one"),
        "output file missing expected content: {contents}"
    );

    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("ARTIFACT step=") && stdout.contains("index.html"),
        "stdout was:\n{stdout}"
    );
}

#[test]
fn run_writes_run_state_artifacts() {
    let base = tmp_dir("exec-run-state-artifacts");
    let _bin = write_mock_ollama(&base, MockOllamaBehavior::Success);
    let new_path = prepend_path(&base);
    let _path_guard = EnvVarGuard::set("PATH", new_path);

    let run_id = "run-state-artifacts-test";
    let run_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join(".adl")
        .join("runs")
        .join(run_id);
    let _ = fs::remove_dir_all(&run_dir);

    let yaml = format!(
        r#"
version: "0.2"

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
  name: "{run_id}"
  workflow:
    kind: "sequential"
    steps:
      - id: "s1"
        agent: "a1"
        task: "t1"
        inputs:
          text: "hello"
"#
    );

    let tmp_yaml = base.join("run-state.yaml");
    fs::write(&tmp_yaml, yaml.as_bytes()).unwrap();

    let out = run_swarm(&[tmp_yaml.to_string_lossy().as_ref(), "--run"]);
    assert!(
        out.status.success(),
        "expected success, got {:?}\nstdout:\n{}\nstderr:\n{}",
        out.status.code(),
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );

    let run_json_path = run_dir.join("run.json");
    let steps_json_path = run_dir.join("steps.json");
    assert!(
        run_json_path.is_file(),
        "missing {}",
        run_json_path.display()
    );
    assert!(
        steps_json_path.is_file(),
        "missing {}",
        steps_json_path.display()
    );

    let run_json: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&run_json_path).unwrap()).unwrap();
    assert_eq!(run_json["run_id"], run_id);
    assert_eq!(run_json["workflow_id"], "workflow");
    assert_eq!(run_json["status"], "success");

    let steps_json: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&steps_json_path).unwrap()).unwrap();
    let steps = steps_json
        .as_array()
        .expect("steps.json should be an array");
    assert_eq!(steps.len(), 1);
    assert_eq!(steps[0]["step_id"], "s1");
    assert_eq!(steps[0]["status"], "success");
    assert_eq!(steps[0]["provider_id"], "local");

    let _ = fs::remove_dir_all(&run_dir);
}

#[test]
fn run_rejects_write_to_traversal() {
    let base = tmp_dir("exec-write-traversal");
    let _bin = write_mock_ollama(&base, MockOllamaBehavior::Success);
    let new_path = prepend_path(&base);
    let _path_guard = EnvVarGuard::set("PATH", new_path);

    let yaml = r#"
version: "0.2"

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
  name: "write-traversal"
  workflow:
    kind: "sequential"
    steps:
      - id: "bad-step"
        agent: "a1"
        task: "t1"
        inputs:
          text: "hello"
        save_as: "summary"
        write_to: "../escape.html"
"#;

    let tmp_yaml = base.join("write-traversal.yaml");
    fs::write(&tmp_yaml, yaml.as_bytes()).unwrap();

    let out = run_swarm(&[tmp_yaml.to_string_lossy().as_ref(), "--run"]);
    assert!(
        !out.status.success(),
        "expected failure, got success.\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );

    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("bad-step") && stderr.contains("write_to"),
        "stderr was:\n{stderr}"
    );
}

#[test]
fn run_quiet_suppresses_step_output() {
    let base = tmp_dir("exec-quiet");
    let _bin = write_mock_ollama(&base, MockOllamaBehavior::Success);
    let new_path = prepend_path(&base);
    let _path_guard = EnvVarGuard::set("PATH", new_path);

    let yaml = r#"
version: "0.2"

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
  name: "quiet-mode"
  workflow:
    kind: "sequential"
    steps:
      - id: "s1"
        agent: "a1"
        task: "t1"
        inputs:
          text: "hello"
        save_as: "summary"
        write_to: "index.html"
"#;

    let tmp_yaml = base.join("quiet.yaml");
    fs::write(&tmp_yaml, yaml.as_bytes()).unwrap();

    let out_dir = base.join("out");
    let out = run_swarm(&[
        tmp_yaml.to_string_lossy().as_ref(),
        "--run",
        "--quiet",
        "--out",
        out_dir.to_string_lossy().as_ref(),
    ]);
    assert!(
        out.status.success(),
        "expected success, got {:?}\nstderr:\n{}",
        out.status.code(),
        String::from_utf8_lossy(&out.stderr)
    );

    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("RUN SUMMARY"), "stdout was:\n{stdout}");
    assert!(stdout.contains("ARTIFACT"), "stdout was:\n{stdout}");
    assert!(
        !stdout.contains("--- step:"),
        "stdout should not include step bodies:\n{stdout}"
    );
    assert!(
        !stdout.contains("mock summary bullet one"),
        "stdout should not include step bodies:\n{stdout}"
    );

    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        !stderr.contains("RUN start")
            && !stderr.contains("STEP start")
            && !stderr.contains("STEP done")
            && !stderr.contains("RUN done"),
        "stderr should not include progress banners under --quiet:\n{stderr}"
    );
}

#[test]
fn run_emits_progress_banners_on_stderr() {
    let base = tmp_dir("exec-progress-banners");
    let _bin = write_mock_ollama(&base, MockOllamaBehavior::Success);
    let new_path = prepend_path(&base);
    let _path_guard = EnvVarGuard::set("PATH", new_path);

    let out = run_swarm(&["examples/v0-3-concurrency-fork-join.adl.yaml", "--run"]);
    assert!(
        out.status.success(),
        "expected success, got {:?}\nstderr:\n{}",
        out.status.code(),
        String::from_utf8_lossy(&out.stderr)
    );

    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("RUN start")
            && stderr.contains("STEP start")
            && stderr.contains("STEP done")
            && stderr.contains("RUN done")
            && stderr.contains("duration_ms="),
        "stderr missing expected progress banners:\n{stderr}"
    );
}

#[test]
fn run_rejects_missing_prompt_inputs() {
    let base = tmp_dir("exec-missing-inputs");
    let _bin = write_mock_ollama(&base, MockOllamaBehavior::Success);
    let new_path = prepend_path(&base);
    let _path_guard = EnvVarGuard::set("PATH", new_path);

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
      user: "Summarize {{missing_key}}"

run:
  name: "missing-inputs"
  workflow:
    kind: "sequential"
    steps:
      - id: "s1"
        agent: "a1"
        task: "t1"
        inputs:
          text: "hello"
"#;

    let tmp_yaml = base.join("missing-inputs.yaml");
    fs::write(&tmp_yaml, yaml.as_bytes()).unwrap();

    let out = run_swarm(&[tmp_yaml.to_string_lossy().as_ref(), "--run"]);
    assert!(
        !out.status.success(),
        "expected failure, got success.\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );

    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("missing input bindings") && stderr.contains("missing_key"),
        "stderr was:\n{stderr}"
    );
}

#[test]
fn run_allows_prompt_only_step_with_no_inputs() {
    let base = tmp_dir("exec-prompt-only-no-inputs");
    let _bin = write_mock_ollama(&base, MockOllamaBehavior::Success);
    let new_path = prepend_path(&base);
    let _path_guard = EnvVarGuard::set("PATH", new_path);

    let yaml = r#"
version: "0.2"

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
      user: "Summarize this prompt-only step."

run:
  name: "prompt-only-step"
  workflow:
    kind: "sequential"
    steps:
      - id: "s1"
        agent: "a1"
        task: "t1"
"#;

    let tmp_yaml = base.join("prompt-only.yaml");
    fs::write(&tmp_yaml, yaml.as_bytes()).unwrap();

    let out = run_swarm(&[tmp_yaml.to_string_lossy().as_ref(), "--run"]);
    assert!(
        out.status.success(),
        "expected success for prompt-only step, got {:?}\nstdout:\n{}\nstderr:\n{}",
        out.status.code(),
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );

    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("--- step: s1 ---"), "stdout was:\n{stdout}");
    assert!(
        stdout.contains("mock summary bullet one"),
        "stdout was:\n{stdout}"
    );
}

#[test]
fn run_executes_compiled_pattern_fork_join_happy_path() {
    let base = tmp_dir("exec-pattern-fork-join-happy");
    let _bin = write_mock_ollama(&base, MockOllamaBehavior::Success);
    let new_path = prepend_path(&base);
    let _path_guard = EnvVarGuard::set("PATH", new_path);

    let pattern = swarm::adl::PatternSpec {
        id: "p_fork".to_string(),
        kind: swarm::adl::PatternKind::ForkJoin,
        steps: vec![],
        fork: Some(swarm::adl::PatternForkSpec {
            branches: vec![
                swarm::adl::PatternBranchSpec {
                    id: "left".to_string(),
                    steps: vec!["L1".to_string(), "L2".to_string()],
                },
                swarm::adl::PatternBranchSpec {
                    id: "right".to_string(),
                    steps: vec!["R1".to_string()],
                },
            ],
        }),
        join: Some(swarm::adl::PatternJoinSpec {
            step: "J".to_string(),
        }),
    };

    let compiled = swarm::execution_plan::compile_pattern(&pattern).expect("compile pattern");

    let mut providers = HashMap::new();
    providers.insert(
        "local".to_string(),
        swarm::adl::ProviderSpec {
            id: None,
            kind: "ollama".to_string(),
            base_url: None,
            default_model: None,
            config: HashMap::new(),
        },
    );

    let mut agents = HashMap::new();
    agents.insert(
        "a1".to_string(),
        swarm::adl::AgentSpec {
            id: None,
            provider: "local".to_string(),
            model: "phi4-mini".to_string(),
            temperature: None,
            top_k: None,
            description: None,
            prompt: None,
            tools: vec![],
        },
    );

    let mut tasks = HashMap::new();
    for task_id in ["L1", "L2", "R1", "J"] {
        tasks.insert(
            task_id.to_string(),
            swarm::adl::TaskSpec {
                id: None,
                agent_ref: None,
                inputs: vec![],
                tool_allowlist: vec![],
                description: None,
                prompt: swarm::adl::PromptSpec {
                    system: None,
                    developer: None,
                    user: Some(format!("Task {task_id}")),
                    context: None,
                    output: None,
                },
            },
        );
    }

    let mut save_as_by_id: HashMap<String, Option<String>> = HashMap::new();
    for node in &compiled.execution_plan.nodes {
        save_as_by_id.insert(node.step_id.clone(), node.save_as.clone());
    }

    let steps: Vec<swarm::resolve::ResolvedStep> = compiled
        .compiled_steps
        .iter()
        .map(|step| swarm::resolve::ResolvedStep {
            id: step.step_id.clone(),
            agent: Some("a1".to_string()),
            provider: Some("local".to_string()),
            task: Some(step.task_symbol.clone()),
            prompt: None,
            inputs: HashMap::new(),
            save_as: save_as_by_id.get(&step.step_id).cloned().flatten(),
            write_to: None,
            on_error: None,
            retry: None,
        })
        .collect();

    let doc = swarm::adl::AdlDoc {
        version: "0.5".to_string(),
        providers,
        tools: HashMap::new(),
        agents,
        tasks,
        workflows: HashMap::new(),
        patterns: vec![pattern],
        signature: None,
        run: swarm::adl::RunSpec {
            id: None,
            name: Some("compiled-pattern-run".to_string()),
            created_at: None,
            defaults: swarm::adl::RunDefaults::default(),
            workflow_ref: None,
            workflow: None,
            pattern_ref: Some("p_fork".to_string()),
            inputs: HashMap::new(),
            placement: None,
        },
    };

    let resolved = swarm::resolve::AdlResolved {
        run_id: "compiled-pattern-run".to_string(),
        workflow_id: "pattern:p_fork".to_string(),
        steps,
        execution_plan: compiled.execution_plan,
        doc,
    };

    let mut tr = swarm::trace::Trace::new("compiled-pattern-run", "pattern:p_fork", "0.5");
    let out_dir = base.join("out");
    fs::create_dir_all(&out_dir).unwrap();

    let result =
        swarm::execute::execute_sequential(&resolved, &mut tr, false, false, &base, &out_dir)
            .expect("compiled pattern should execute");

    assert_eq!(result.outputs.len(), 4);
    let ids: Vec<String> = result.outputs.iter().map(|o| o.step_id.clone()).collect();
    assert!(ids.contains(&"p::p_fork::left::L1".to_string()));
    assert!(ids.contains(&"p::p_fork::left::L2".to_string()));
    assert!(ids.contains(&"p::p_fork::right::R1".to_string()));
    assert!(ids.contains(&"p::p_fork::J".to_string()));
}

#[test]
fn run_rejects_concurrent_workflows_in_v0_4_without_pattern_ref() {
    let base = tmp_dir("exec-reject-concurrent-v0-4");
    let _bin = write_mock_ollama(&base, MockOllamaBehavior::Success);
    let new_path = prepend_path(&base);
    let _path_guard = EnvVarGuard::set("PATH", new_path);

    let yaml = r#"
version: "0.4"

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
      user: "hello"

run:
  name: "reject-concurrent-v0-4"
  workflow:
    kind: "concurrent"
    steps:
      - id: "s1"
        agent: "a1"
        task: "t1"
"#;

    let tmp_yaml = base.join("reject-concurrent-v0-4.yaml");
    fs::write(&tmp_yaml, yaml.as_bytes()).unwrap();

    let out = run_swarm(&[tmp_yaml.to_string_lossy().as_ref(), "--run"]);
    assert!(
        !out.status.success(),
        "expected failure for v0.4 concurrent workflow without pattern_ref, got success.\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );

    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("requires v0.3 workflows or v0.5 pattern runs")
            && stderr.contains("document version is 0.4"),
        "stderr should contain gate message; stderr was:\n{stderr}"
    );
}
