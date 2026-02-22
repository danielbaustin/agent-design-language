use std::collections::HashMap;
use std::fs;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::path::Path;
use std::process::Command;
use std::thread;
use std::time::Duration;
use swarm::execute::{materialize_inputs, MATERIALIZE_INPUT_MAX_FILE_BYTES};

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

fn reserve_local_port() -> u16 {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind ephemeral port");
    let port = listener.local_addr().unwrap().port();
    drop(listener);
    port
}

fn start_swarm_remote_server() -> String {
    let port = reserve_local_port();
    let bind_addr = format!("127.0.0.1:{port}");
    thread::spawn({
        let bind_addr = bind_addr.clone();
        move || {
            let _ = swarm::remote_exec::run_server(&bind_addr);
        }
    });
    thread::sleep(Duration::from_millis(120));
    format!("http://{bind_addr}")
}

fn start_raw_http_server(raw_response: &'static str) -> String {
    let port = reserve_local_port();
    let bind_addr = format!("127.0.0.1:{port}");
    thread::spawn({
        let bind_addr = bind_addr.clone();
        move || {
            let listener = TcpListener::bind(&bind_addr).expect("bind raw http test server");
            if let Ok((mut stream, _)) = listener.accept() {
                let mut buf = [0_u8; 2048];
                let _ = stream.read(&mut buf);
                let _ = stream.write_all(raw_response.as_bytes());
                let _ = stream.flush();
            }
        }
    });
    thread::sleep(Duration::from_millis(80));
    format!("http://{bind_addr}")
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
fn materialize_inputs_accepts_exact_max_file_input_size() {
    let base = tmp_dir("mat-maxsize-exact");
    // Boundary check for the @file: materialization size limit.
    let max = MATERIALIZE_INPUT_MAX_FILE_BYTES as usize;
    let exact = vec![b'a'; max];
    write_file(&base, "docs/exact.txt", &exact);

    let mut inputs = HashMap::new();
    inputs.insert("doc_1".to_string(), "@file:docs/exact.txt".to_string());

    let out = materialize_inputs(inputs, &base).unwrap();
    assert_eq!(
        out.get("doc_1").map(|s| s.len()),
        Some(max),
        "exact materialization MAX payload should be accepted"
    );
}

#[test]
fn materialize_inputs_rejects_max_plus_one_file_input_size() {
    let base = tmp_dir("mat-maxsize");
    // Boundary check for the @file: materialization size limit (MAX + 1).
    let max_plus_one = MATERIALIZE_INPUT_MAX_FILE_BYTES as usize + 1;
    let big = vec![b'a'; max_plus_one];
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
    Command::new(exe)
        .env("ADL_ALLOW_UNSIGNED", "1")
        .args(args)
        .output()
        .unwrap()
}

fn run_artifact_paths(run_id: &str) -> (std::path::PathBuf, std::path::PathBuf) {
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("repo root");
    let run_dir = repo_root.join(".adl").join("runs").join(run_id);
    (run_dir.join("run.json"), run_dir.join("steps.json"))
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
        stderr.contains("attempt 2/2") && stderr.contains("max_attempts=2"),
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
fn run_executes_mixed_local_remote_local_steps() {
    let base = tmp_dir("exec-v0-5-remote-mixed");
    let _bin = write_mock_ollama(&base, MockOllamaBehavior::EchoPrompt);
    let new_path = prepend_path(&base);
    let _path_guard = EnvVarGuard::set("PATH", new_path);
    let endpoint = start_swarm_remote_server();

    let yaml = format!(
        r#"
version: "0.5"

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
  t:
    prompt:
      user: "STEP={{step}} INPUT={{input}}"

run:
  name: "v0-5-remote-mixed"
  placement: local
  remote:
    endpoint: "{endpoint}"
    timeout_ms: 2000
  workflow:
    kind: "sequential"
    steps:
      - id: "local.first"
        agent: "a1"
        task: "t"
        placement: local
        save_as: "first"
        inputs:
          step: "local-1"
          input: "seed"
      - id: "remote.mid"
        agent: "a1"
        task: "t"
        placement: remote
        save_as: "mid"
        inputs:
          step: "remote-2"
          input: "@state:first"
      - id: "local.last"
        agent: "a1"
        task: "t"
        placement: local
        inputs:
          step: "local-3"
          input: "@state:mid"
"#
    );
    let tmp_yaml = base.join("v0-5-remote-mixed.yaml");
    fs::write(&tmp_yaml, yaml).unwrap();

    let out = run_swarm(&[tmp_yaml.to_str().unwrap(), "--run"]);
    assert!(
        out.status.success(),
        "expected success.\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("--- step: local.first ---"),
        "stdout:\n{stdout}"
    );
    assert!(
        stdout.contains("--- step: remote.mid ---"),
        "stdout:\n{stdout}"
    );
    assert!(
        stdout.contains("--- step: local.last ---"),
        "stdout:\n{stdout}"
    );
}

#[test]
fn run_remote_unreachable_is_reported() {
    let base = tmp_dir("exec-v0-5-remote-unreachable");
    let _bin = write_mock_ollama(&base, MockOllamaBehavior::EchoPrompt);
    let new_path = prepend_path(&base);
    let _path_guard = EnvVarGuard::set("PATH", new_path);
    let port = reserve_local_port();
    let endpoint = format!("http://127.0.0.1:{port}");

    let yaml = format!(
        r#"
version: "0.5"
providers:
  local:
    type: "ollama"
agents:
  a1:
    provider: "local"
    model: "phi4-mini"
tasks:
  t:
    prompt:
      user: "hello"
run:
  name: "v0-5-remote-unreachable"
  placement: remote
  remote:
    endpoint: "{endpoint}"
    timeout_ms: 300
  workflow:
    kind: "sequential"
    steps:
      - id: "remote.only"
        agent: "a1"
        task: "t"
"#
    );
    let tmp_yaml = base.join("v0-5-remote-unreachable.yaml");
    fs::write(&tmp_yaml, yaml).unwrap();

    let out = run_swarm(&[tmp_yaml.to_str().unwrap(), "--run"]);
    assert!(
        !out.status.success(),
        "expected failure for unreachable remote"
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("REMOTE_UNREACHABLE"), "stderr:\n{stderr}");
}

#[test]
fn run_remote_timeout_and_invalid_json_are_mapped() {
    let base = tmp_dir("exec-v0-5-remote-timeout-json");
    let _bin = write_mock_ollama(&base, MockOllamaBehavior::SleepEchoPrompt);
    let new_path = prepend_path(&base);
    let _path_guard = EnvVarGuard::set("PATH", new_path);
    let endpoint_timeout = start_swarm_remote_server();

    let yaml_timeout = format!(
        r#"
version: "0.5"
providers:
  local:
    type: "ollama"
agents:
  a1:
    provider: "local"
    model: "phi4-mini"
tasks:
  t:
    prompt:
      user: "hello"
run:
  name: "v0-5-remote-timeout"
  placement: remote
  remote:
    endpoint: "{endpoint_timeout}"
    timeout_ms: 10
  workflow:
    kind: "sequential"
    steps:
      - id: "remote.timeout"
        agent: "a1"
        task: "t"
"#
    );
    let tmp_yaml_timeout = base.join("v0-5-remote-timeout.yaml");
    fs::write(&tmp_yaml_timeout, yaml_timeout).unwrap();

    let out_timeout = run_swarm(&[tmp_yaml_timeout.to_str().unwrap(), "--run"]);
    assert!(!out_timeout.status.success(), "expected timeout failure");
    let stderr_timeout = String::from_utf8_lossy(&out_timeout.stderr);
    assert!(
        stderr_timeout.contains("REMOTE_TIMEOUT"),
        "stderr:\n{stderr_timeout}"
    );

    let endpoint_bad_json = start_raw_http_server(
        "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 8\r\n\r\nnot-json",
    );
    let yaml_bad_json = format!(
        r#"
version: "0.5"
providers:
  local:
    type: "ollama"
agents:
  a1:
    provider: "local"
    model: "phi4-mini"
tasks:
  t:
    prompt:
      user: "hello"
run:
  name: "v0-5-remote-bad-json"
  placement: remote
  remote:
    endpoint: "{endpoint_bad_json}"
    timeout_ms: 1000
  workflow:
    kind: "sequential"
    steps:
      - id: "remote.bad_json"
        agent: "a1"
        task: "t"
"#
    );
    let tmp_yaml_bad_json = base.join("v0-5-remote-bad-json.yaml");
    fs::write(&tmp_yaml_bad_json, yaml_bad_json).unwrap();

    let out_bad_json = run_swarm(&[tmp_yaml_bad_json.to_str().unwrap(), "--run"]);
    assert!(!out_bad_json.status.success(), "expected bad-json failure");
    let stderr_bad_json = String::from_utf8_lossy(&out_bad_json.stderr);
    assert!(
        stderr_bad_json.contains("REMOTE_INVALID_JSON"),
        "stderr:\n{stderr_bad_json}"
    );

    let endpoint_bad_status =
        start_raw_http_server("HTTP/1.1 503 Service Unavailable\r\nContent-Length: 0\r\n\r\n");
    let yaml_bad_status = format!(
        r#"
version: "0.5"
providers:
  local:
    type: "ollama"
agents:
  a1:
    provider: "local"
    model: "phi4-mini"
tasks:
  t:
    prompt:
      user: "hello"
run:
  name: "v0-5-remote-bad-status"
  placement: remote
  remote:
    endpoint: "{endpoint_bad_status}"
    timeout_ms: 1000
  workflow:
    kind: "sequential"
    steps:
      - id: "remote.bad_status"
        agent: "a1"
        task: "t"
"#
    );
    let tmp_yaml_bad_status = base.join("v0-5-remote-bad-status.yaml");
    fs::write(&tmp_yaml_bad_status, yaml_bad_status).unwrap();
    let out_bad_status = run_swarm(&[tmp_yaml_bad_status.to_str().unwrap(), "--run"]);
    assert!(
        !out_bad_status.status.success(),
        "expected bad-status failure"
    );
    let stderr_bad_status = String::from_utf8_lossy(&out_bad_status.stderr);
    assert!(
        stderr_bad_status.contains("REMOTE_BAD_STATUS"),
        "stderr:\n{stderr_bad_status}"
    );
}

#[test]
fn run_remote_failure_with_continue_keeps_scheduler_state_intact() {
    let base = tmp_dir("exec-v0-5-remote-continue");
    let _bin = write_mock_ollama(&base, MockOllamaBehavior::EchoPrompt);
    let new_path = prepend_path(&base);
    let _path_guard = EnvVarGuard::set("PATH", new_path);

    let endpoint_remote_fail = start_raw_http_server(
        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: 167\r\n\r\n{\"ok\":false,\"run_id\":\"r\",\"workflow_id\":\"w\",\"step_id\":\"remote.fail\",\"result\":null,\"artifacts\":[],\"error\":{\"code\":\"REMOTE_EXECUTION_ERROR\",\"message\":\"boom\",\"details\":{}}}",
    );
    let yaml = format!(
        r#"
version: "0.5"
providers:
  local:
    type: "ollama"
agents:
  a1:
    provider: "local"
    model: "phi4-mini"
tasks:
  t:
    prompt:
      user: "STEP={{step}} INPUT={{input}}"
run:
  name: "v0-5-remote-continue"
  placement: local
  remote:
    endpoint: "{endpoint_remote_fail}"
    timeout_ms: 1000
  workflow:
    kind: "sequential"
    steps:
      - id: "local.first"
        placement: local
        save_as: "first"
        agent: "a1"
        task: "t"
        inputs:
          step: "local-1"
          input: "seed"
      - id: "remote.fail"
        placement: remote
        on_error: continue
        agent: "a1"
        task: "t"
        inputs:
          step: "remote-2"
          input: "@state:first"
      - id: "local.after"
        placement: local
        agent: "a1"
        task: "t"
        inputs:
          step: "local-3"
          input: "@state:first"
"#
    );
    let tmp_yaml = base.join("v0-5-remote-continue.yaml");
    fs::write(&tmp_yaml, yaml).unwrap();

    let out = run_swarm(&[tmp_yaml.to_str().unwrap(), "--run"]);
    assert!(
        out.status.success(),
        "expected success with continue policy.\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );

    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("--- step: local.first ---"),
        "stdout:\n{stdout}"
    );
    assert!(
        stdout.contains("--- step: local.after ---"),
        "stdout:\n{stdout}"
    );
    assert!(
        stdout.contains("status=failure") && stdout.contains("step=remote.fail"),
        "run summary should record remote failure under continue policy; stdout:\n{stdout}"
    );
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
        stderr.contains("attempt 1/3")
            && stderr.contains("max_attempts=3")
            && stderr.contains("client_error")
            && !stderr.contains("attempt 2/3"),
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
        stderr.contains("attempt 2/2")
            && stderr.contains("max_attempts=2")
            && stderr.contains("timed out"),
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

    let out = run_swarm(&[tmp_yaml.to_str().unwrap(), "--run"]);
    assert!(
        out.status.success(),
        "expected success for bounded parallelism run.\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );

    let stderr = String::from_utf8_lossy(&out.stderr);
    // Deterministic bounded scheduling evidence:
    // - first batch (s1..s4) starts before any step completion
    // - second batch (s5) cannot start until at least one completion occurs
    let s1_start = stderr
        .find("STEP start")
        .expect("missing step start progress in stderr");
    let s4_start = stderr
        .find("STEP start (+0ms) s4 provider=local")
        .or_else(|| stderr.find(" s4 provider=local"))
        .expect("missing start marker for s4 in stderr");
    let first_done = stderr
        .find("STEP done")
        .expect("missing step completion progress in stderr");
    let s5_start = stderr
        .find(" s5 provider=local")
        .expect("missing start marker for s5 in stderr");

    assert!(
        s1_start < s4_start && s4_start < first_done,
        "expected first bounded batch (s1..s4) to start before first completion.\nstderr:\n{stderr}"
    );
    assert!(
        first_done < s5_start,
        "expected s5 to wait for a completion from the first bounded batch.\nstderr:\n{stderr}"
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
fn run_streaming_is_observational_only_for_artifacts() {
    let base = tmp_dir("exec-streaming-observational");
    let _bin = write_mock_ollama(&base, MockOllamaBehavior::Success);
    let new_path = prepend_path(&base);
    let _path_guard = EnvVarGuard::set("PATH", new_path);

    let yaml = r#"
version: "0.3"
providers:
  local:
    type: "ollama"
agents:
  a:
    provider: "local"
    model: "phi4-mini"
tasks:
  t:
    prompt:
      user: "Summarize: {{text}}"
run:
  name: "streaming-observational"
  workflow:
    kind: "sequential"
    steps:
      - id: "s1"
        agent: "a"
        task: "t"
        inputs:
          text: "alpha"
        save_as: "summary"
        write_to: "index.html"
"#;
    let yaml_path = base.join("streaming-observational.yaml");
    fs::write(&yaml_path, yaml).unwrap();

    let out_stream = base.join("out-stream");
    let run_stream = run_swarm(&[
        yaml_path.to_str().unwrap(),
        "--run",
        "--out",
        out_stream.to_str().unwrap(),
    ]);
    assert!(run_stream.status.success(), "stream run should succeed");

    let out_quiet = base.join("out-quiet");
    let run_quiet = run_swarm(&[
        yaml_path.to_str().unwrap(),
        "--run",
        "--quiet",
        "--out",
        out_quiet.to_str().unwrap(),
    ]);
    assert!(run_quiet.status.success(), "quiet run should succeed");

    let stream_html = fs::read_to_string(out_stream.join("index.html")).unwrap();
    let quiet_html = fs::read_to_string(out_quiet.join("index.html")).unwrap();
    assert_eq!(
        stream_html, quiet_html,
        "streaming must not change output artifacts"
    );
}

#[test]
fn run_streaming_trace_emits_chunk_events_deterministically() {
    let base = tmp_dir("exec-streaming-trace-events");
    let _bin = write_mock_ollama(&base, MockOllamaBehavior::Success);
    let new_path = prepend_path(&base);
    let _path_guard = EnvVarGuard::set("PATH", new_path);

    let yaml = r#"
version: "0.3"
providers:
  local:
    type: "ollama"
agents:
  a:
    provider: "local"
    model: "phi4-mini"
tasks:
  t:
    prompt:
      user: "stream {{n}}"
run:
  name: "streaming-trace-events"
  workflow:
    kind: "concurrent"
    steps:
      - id: "s3"
        agent: "a"
        task: "t"
        inputs: { n: "3" }
      - id: "s1"
        agent: "a"
        task: "t"
        inputs: { n: "1" }
      - id: "s2"
        agent: "a"
        task: "t"
        inputs: { n: "2" }
"#;
    let yaml_path = base.join("streaming-trace-events.yaml");
    fs::write(&yaml_path, yaml).unwrap();

    let out1 = run_swarm(&[yaml_path.to_str().unwrap(), "--run", "--trace"]);
    let out2 = run_swarm(&[yaml_path.to_str().unwrap(), "--run", "--trace"]);
    assert!(out1.status.success(), "run #1 should succeed");
    assert!(out2.status.success(), "run #2 should succeed");

    let stdout1 = String::from_utf8_lossy(&out1.stdout);
    let stdout2 = String::from_utf8_lossy(&out2.stdout);
    assert!(
        stdout1.contains("StepOutputChunk step=s1")
            && stdout1.contains("StepOutputChunk step=s2")
            && stdout1.contains("StepOutputChunk step=s3"),
        "trace missing StepOutputChunk events:\n{stdout1}"
    );
    assert!(
        !stdout1
            .lines()
            .filter(|l| l.contains("StepOutputChunk"))
            .any(|l| l.contains("delegation=")),
        "chunk events must not include delegation metadata:\n{stdout1}"
    );
    assert_eq!(
        trace_started_step_ids(&stdout1),
        trace_started_step_ids(&stdout2)
    );
    assert_eq!(
        trace_chunk_step_ids(&stdout1),
        trace_chunk_step_ids(&stdout2)
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
            profile: None,
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
            placement: None,
            task: Some(step.task_symbol.clone()),
            call: None,
            with: HashMap::new(),
            as_ns: None,
            delegation: None,
            prompt: None,
            inputs: HashMap::new(),
            guards: vec![],
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
            remote: None,
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
fn trace_started_step_ids(stdout: &str) -> Vec<String> {
    stdout
        .lines()
        .filter_map(|line| {
            let marker = "StepStarted step=";
            let (_, tail) = line.split_once(marker)?;
            Some(tail.split_whitespace().next()?.to_string())
        })
        .collect()
}

fn trace_chunk_step_ids(stdout: &str) -> Vec<String> {
    stdout
        .lines()
        .filter_map(|line| {
            let marker = "StepOutputChunk step=";
            let (_, tail) = line.split_once(marker)?;
            Some(tail.split_whitespace().next()?.to_string())
        })
        .collect()
}

#[test]
fn run_executes_call_workflow_with_namespaced_state_and_trace_events() {
    let base = tmp_dir("exec-call-workflow");
    let _bin = write_mock_ollama(&base, MockOllamaBehavior::EchoPrompt);
    let new_path = prepend_path(&base);
    let _path_guard = EnvVarGuard::set("PATH", new_path);

    let yaml = r#"
version: "0.5"

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
  t_child:
    prompt:
      user: "child {{inputs.topic}}"
  t_join:
    prompt:
      user: "join {{a}} + {{b}}"

workflows:
  wf_child:
    kind: sequential
    steps:
      - id: "child_s1"
        agent: "a1"
        task: "t_child"
        save_as: "child_out"

run:
  workflow:
    kind: sequential
    steps:
      - id: "call_one"
        call: "wf_child"
        with:
          topic: "A"
        as: "one"
      - id: "call_two"
        call: "wf_child"
        with:
          topic: "B"
        as: "two"
      - id: "join"
        agent: "a1"
        task: "t_join"
        inputs:
          a: "@state:one.child_out"
          b: "@state:two.child_out"
"#;

    let tmp_yaml = base.join("call-workflow.yaml");
    fs::write(&tmp_yaml, yaml.as_bytes()).unwrap();

    let out = run_swarm(&[tmp_yaml.to_string_lossy().as_ref(), "--run", "--trace"]);
    assert!(
        out.status.success(),
        "expected success, got {:?}\nstdout:\n{}\nstderr:\n{}",
        out.status.code(),
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );

    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("step=call_one::child_s1"),
        "stdout was:\n{stdout}"
    );
    assert!(
        stdout.contains("step=call_two::child_s1"),
        "stdout was:\n{stdout}"
    );
    assert!(
        stdout.contains("CallEntered caller_step=call_one"),
        "stdout was:\n{stdout}"
    );
    assert!(
        stdout.contains("CallExited caller_step=call_two status=success"),
        "stdout was:\n{stdout}"
    );
}

#[test]
fn run_v0_3_concurrent_scheduler_uses_lexicographic_batches_with_max_concurrency_2() {
    let base = tmp_dir("exec-concurrent-v0-3-max-concurrency-2");
    let _bin = write_mock_ollama(&base, MockOllamaBehavior::SleepEchoPrompt);
    let new_path = prepend_path(&base);
    let _path_guard = EnvVarGuard::set("PATH", new_path);

    let yaml = r#"
version: "0.3"
providers:
  local:
    type: "ollama"
agents:
  a:
    provider: "local"
    model: "phi4-mini"
tasks:
  t:
    prompt:
      user: "work {{n}}"
run:
  name: "v0-3-max-concurrency-2"
  defaults:
    max_concurrency: 2
  workflow:
    kind: "concurrent"
    steps:
      - id: "s3"
        agent: "a"
        task: "t"
        inputs: { n: "3" }
      - id: "s1"
        agent: "a"
        task: "t"
        inputs: { n: "1" }
      - id: "s4"
        agent: "a"
        task: "t"
        inputs: { n: "4" }
      - id: "s2"
        agent: "a"
        task: "t"
        inputs: { n: "2" }
"#;
    let tmp_yaml = base.join("max-concurrency-2.yaml");
    fs::write(&tmp_yaml, yaml).unwrap();

    let out1 = run_swarm(&[tmp_yaml.to_str().unwrap(), "--run", "--trace"]);
    assert!(
        out1.status.success(),
        "expected success.\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out1.stdout),
        String::from_utf8_lossy(&out1.stderr)
    );
    let started1 = trace_started_step_ids(&String::from_utf8_lossy(&out1.stdout));
    assert_eq!(started1, vec!["s1", "s2", "s3", "s4"]);

    // Determinism regression guard: identical started order on a second run.
    let out2 = run_swarm(&[tmp_yaml.to_str().unwrap(), "--run", "--trace"]);
    assert!(
        out2.status.success(),
        "expected success.\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out2.stdout),
        String::from_utf8_lossy(&out2.stderr)
    );
    let started2 = trace_started_step_ids(&String::from_utf8_lossy(&out2.stdout));
    assert_eq!(started1, started2);
}

#[test]
fn trace_step_started_includes_step_delegation_metadata() {
    let base = tmp_dir("exec-step-delegation-trace");
    let _bin = write_mock_ollama(&base, MockOllamaBehavior::EchoPrompt);
    let new_path = prepend_path(&base);
    let _path_guard = EnvVarGuard::set("PATH", new_path);

    let yaml = r#"
version: "0.5"
providers:
  local:
    type: "ollama"
agents:
  a:
    provider: "local"
    model: "phi4-mini"
tasks:
  t:
    prompt:
      user: "work {{n}}"
run:
  workflow:
    kind: "sequential"
    steps:
      - id: "s1"
        agent: "a"
        task: "t"
        inputs: { n: "1" }
        delegation:
          role: "reviewer"
          requires_verification: true
          escalation_target: "human"
          tags: ["safety", "compliance"]
"#;
    let tmp_yaml = base.join("delegation-trace.yaml");
    fs::write(&tmp_yaml, yaml).unwrap();

    let out = run_swarm(&[tmp_yaml.to_str().unwrap(), "--run", "--trace"]);
    assert!(
        out.status.success(),
        "expected success.\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );

    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("StepStarted step=s1"),
        "stdout was:\n{stdout}"
    );
    assert!(
        stdout.contains("delegation={\"role\":\"reviewer\",\"requires_verification\":true,\"escalation_target\":\"human\",\"tags\":[\"compliance\",\"safety\"]}"),
        "stdout was:\n{stdout}"
    );
}

#[test]
fn step_delegation_does_not_change_concurrent_step_order_determinism() {
    let base = tmp_dir("exec-step-delegation-determinism");
    let _bin = write_mock_ollama(&base, MockOllamaBehavior::SleepEchoPrompt);
    let new_path = prepend_path(&base);
    let _path_guard = EnvVarGuard::set("PATH", new_path);

    let yaml = r#"
version: "0.3"
providers:
  local:
    type: "ollama"
agents:
  a:
    provider: "local"
    model: "phi4-mini"
tasks:
  t:
    prompt:
      user: "work {{n}}"
run:
  name: "v0-3-delegation-determinism"
  defaults:
    max_concurrency: 2
  workflow:
    kind: "concurrent"
    steps:
      - id: "s3"
        agent: "a"
        task: "t"
        inputs: { n: "3" }
      - id: "s1"
        agent: "a"
        task: "t"
        inputs: { n: "1" }
        delegation:
          role: "reviewer"
          tags: ["safety"]
      - id: "s4"
        agent: "a"
        task: "t"
        inputs: { n: "4" }
      - id: "s2"
        agent: "a"
        task: "t"
        inputs: { n: "2" }
"#;

    let tmp_yaml = base.join("delegation-determinism.yaml");
    fs::write(&tmp_yaml, yaml).unwrap();

    let out1 = run_swarm(&[tmp_yaml.to_str().unwrap(), "--run", "--trace"]);
    assert!(
        out1.status.success(),
        "expected success.\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out1.stdout),
        String::from_utf8_lossy(&out1.stderr)
    );
    let out2 = run_swarm(&[tmp_yaml.to_str().unwrap(), "--run", "--trace"]);
    assert!(
        out2.status.success(),
        "expected success.\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out2.stdout),
        String::from_utf8_lossy(&out2.stderr)
    );

    let started1 = trace_started_step_ids(&String::from_utf8_lossy(&out1.stdout));
    let started2 = trace_started_step_ids(&String::from_utf8_lossy(&out2.stdout));
    assert_eq!(started1, vec!["s1", "s2", "s3", "s4"]);
    assert_eq!(started1, started2);
}

#[test]
fn run_v0_3_concurrent_scheduler_max_concurrency_1_matches_sequential_step_start_order() {
    let base = tmp_dir("exec-concurrent-v0-3-max-concurrency-1");
    let _bin = write_mock_ollama(&base, MockOllamaBehavior::SleepEchoPrompt);
    let new_path = prepend_path(&base);
    let _path_guard = EnvVarGuard::set("PATH", new_path);

    let yaml = r#"
version: "0.3"
providers:
  local:
    type: "ollama"
agents:
  a:
    provider: "local"
    model: "phi4-mini"
tasks:
  t:
    prompt:
      user: "work {{n}}"
run:
  name: "v0-3-max-concurrency-1"
  defaults:
    max_concurrency: 1
  workflow:
    kind: "concurrent"
    steps:
      - id: "s3"
        agent: "a"
        task: "t"
        inputs: { n: "3" }
      - id: "s1"
        agent: "a"
        task: "t"
        inputs: { n: "1" }
      - id: "s2"
        agent: "a"
        task: "t"
        inputs: { n: "2" }
"#;
    let tmp_yaml = base.join("max-concurrency-1.yaml");
    fs::write(&tmp_yaml, yaml).unwrap();

    let out = run_swarm(&[tmp_yaml.to_str().unwrap(), "--run", "--trace"]);
    assert!(
        out.status.success(),
        "expected success.\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    let started = trace_started_step_ids(&String::from_utf8_lossy(&out.stdout));
    assert_eq!(started, vec!["s1", "s2", "s3"]);
}

#[test]
fn run_v0_3_max_concurrency_1_matches_sequential_outputs_for_same_plan() {
    let base = tmp_dir("exec-concurrent-v0-3-max1-vs-seq");
    let _bin = write_mock_ollama(&base, MockOllamaBehavior::EchoPrompt);
    let new_path = prepend_path(&base);
    let _path_guard = EnvVarGuard::set("PATH", new_path);

    let seq_yaml = r#"
version: "0.3"
providers:
  local:
    type: "ollama"
agents:
  a:
    provider: "local"
    model: "phi4-mini"
tasks:
  t:
    prompt:
      user: "work {{n}}"
run:
  name: "v0-3-seq"
  workflow:
    kind: "sequential"
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
"#;
    let conc_yaml = r#"
version: "0.3"
providers:
  local:
    type: "ollama"
agents:
  a:
    provider: "local"
    model: "phi4-mini"
tasks:
  t:
    prompt:
      user: "work {{n}}"
run:
  name: "v0-3-conc-max1"
  defaults:
    max_concurrency: 1
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
"#;
    let seq_path = base.join("seq.yaml");
    let conc_path = base.join("conc.yaml");
    fs::write(&seq_path, seq_yaml).unwrap();
    fs::write(&conc_path, conc_yaml).unwrap();

    let out_seq = run_swarm(&[seq_path.to_str().unwrap(), "--run"]);
    let out_conc = run_swarm(&[conc_path.to_str().unwrap(), "--run"]);
    assert!(
        out_seq.status.success(),
        "seq failed: {:?}",
        out_seq.status.code()
    );
    assert!(
        out_conc.status.success(),
        "conc failed: {:?}",
        out_conc.status.code()
    );
    assert_eq!(
        String::from_utf8_lossy(&out_seq.stdout),
        String::from_utf8_lossy(&out_conc.stdout),
        "max_concurrency=1 concurrent output should match sequential output for the same ordered plan"
    );
}

#[test]
fn run_v0_3_workflow_max_concurrency_override_takes_precedence_over_run_default() {
    let base = tmp_dir("exec-concurrent-v0-3-workflow-override");
    let _bin = write_mock_ollama(&base, MockOllamaBehavior::SleepTrackConcurrency);
    let new_path = prepend_path(&base);
    let _path_guard = EnvVarGuard::set("PATH", new_path);

    let yaml = r#"
version: "0.3"
providers:
  local:
    type: "ollama"
agents:
  a:
    provider: "local"
    model: "phi4-mini"
tasks:
  t:
    prompt:
      user: "work {{n}}"
run:
  name: "v0-3-workflow-override"
  defaults:
    max_concurrency: 1
  workflow:
    kind: "concurrent"
    max_concurrency: 2
    steps:
      - id: "s3"
        agent: "a"
        task: "t"
        inputs: { n: "3" }
      - id: "s1"
        agent: "a"
        task: "t"
        inputs: { n: "1" }
      - id: "s5"
        agent: "a"
        task: "t"
        inputs: { n: "5" }
      - id: "s2"
        agent: "a"
        task: "t"
        inputs: { n: "2" }
      - id: "s4"
        agent: "a"
        task: "t"
        inputs: { n: "4" }
"#;
    let tmp_yaml = base.join("workflow-override.yaml");
    fs::write(&tmp_yaml, yaml).unwrap();

    let out1 = run_swarm(&[tmp_yaml.to_str().unwrap(), "--run", "--trace"]);
    assert!(
        out1.status.success(),
        "expected success.
stdout:
{}
stderr:
{}",
        String::from_utf8_lossy(&out1.stdout),
        String::from_utf8_lossy(&out1.stderr)
    );

    let started1 = trace_started_step_ids(&String::from_utf8_lossy(&out1.stdout));
    assert_eq!(started1, vec!["s1", "s2", "s3", "s4", "s5"]);

    let stderr1 = String::from_utf8_lossy(&out1.stderr);
    let s2_start = stderr1
        .find(" s2 provider=local")
        .expect("missing start marker for s2");
    let first_done = stderr1.find("STEP done").expect("missing first completion");
    let s3_start = stderr1
        .find(" s3 provider=local")
        .expect("missing start marker for s3");
    assert!(
        s2_start < first_done,
        "expected workflow override max_concurrency=2 to allow s1/s2 in first batch.
stderr:
{stderr1}"
    );
    assert!(
        first_done < s3_start,
        "expected s3 to wait for a completion after first bounded batch.
stderr:
{stderr1}"
    );

    let out2 = run_swarm(&[tmp_yaml.to_str().unwrap(), "--run", "--trace"]);
    assert!(
        out2.status.success(),
        "expected success.
stdout:
{}
stderr:
{}",
        String::from_utf8_lossy(&out2.stdout),
        String::from_utf8_lossy(&out2.stderr)
    );
    let started2 = trace_started_step_ids(&String::from_utf8_lossy(&out2.stdout));
    assert_eq!(started1, started2);
}

#[test]
fn run_pause_then_resume_matches_non_paused_final_artifact() {
    let base = tmp_dir("exec-pause-resume-seq");
    let _bin = write_mock_ollama(&base, MockOllamaBehavior::EchoPrompt);
    let new_path = prepend_path(&base);
    let _path_guard = EnvVarGuard::set("PATH", new_path);

    let paused_yaml = r#"
version: "0.3"
providers:
  local:
    type: "ollama"
agents:
  a:
    provider: "local"
    model: "phi4-mini"
tasks:
  t:
    prompt:
      user: "step {{n}}"
run:
  name: "hitl-pause-seq"
  workflow:
    kind: "sequential"
    steps:
      - id: "s1"
        agent: "a"
        task: "t"
        save_as: "s1"
        write_to: "s1.txt"
        inputs: { n: "1" }
      - id: "s2"
        agent: "a"
        task: "t"
        save_as: "s2"
        write_to: "s2.txt"
        inputs: { n: "2" }
        guards:
          - type: pause
            config: { reason: "await_review" }
      - id: "s3"
        agent: "a"
        task: "t"
        save_as: "s3"
        write_to: "s3.txt"
        inputs: { n: "3" }
"#;
    let plain_yaml = paused_yaml.replace(
        "        guards:\n          - type: pause\n            config: { reason: \"await_review\" }\n",
        "",
    );
    let paused_path = base.join("paused.yaml");
    let plain_path = base.join("plain.yaml");
    fs::write(&paused_path, paused_yaml).unwrap();
    fs::write(&plain_path, plain_yaml).unwrap();

    let out_paused = run_swarm(&[
        paused_path.to_str().unwrap(),
        "--run",
        "--out",
        base.join("out-paused").to_str().unwrap(),
    ]);
    assert!(
        out_paused.status.success(),
        "paused run should succeed with paused state.\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out_paused.stdout),
        String::from_utf8_lossy(&out_paused.stderr)
    );

    let (run_json_path, _) = run_artifact_paths("hitl-pause-seq");
    let run_json: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&run_json_path).unwrap()).unwrap();
    assert_eq!(run_json["status"], "paused");
    assert_eq!(run_json["pause"]["paused_step_id"], "s2");

    let out_resumed = run_swarm(&[
        paused_path.to_str().unwrap(),
        "--run",
        "--resume",
        run_json_path.to_str().unwrap(),
        "--out",
        base.join("out-resume").to_str().unwrap(),
    ]);
    assert!(
        out_resumed.status.success(),
        "resume run should succeed.\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out_resumed.stdout),
        String::from_utf8_lossy(&out_resumed.stderr)
    );

    let out_plain = run_swarm(&[
        plain_path.to_str().unwrap(),
        "--run",
        "--out",
        base.join("out-plain").to_str().unwrap(),
    ]);
    assert!(
        out_plain.status.success(),
        "plain run should succeed.\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out_plain.stdout),
        String::from_utf8_lossy(&out_plain.stderr)
    );

    let resumed_final = fs::read_to_string(base.join("out-resume").join("s3.txt")).unwrap();
    let plain_final = fs::read_to_string(base.join("out-plain").join("s3.txt")).unwrap();
    assert_eq!(resumed_final, plain_final);
}

#[test]
fn run_concurrent_pause_then_resume_is_deterministic() {
    let base = tmp_dir("exec-pause-resume-concurrent");
    let _bin = write_mock_ollama(&base, MockOllamaBehavior::SleepTrackConcurrency);
    let new_path = prepend_path(&base);
    let _path_guard = EnvVarGuard::set("PATH", new_path);

    let yaml = r#"
version: "0.3"
providers:
  local:
    type: "ollama"
agents:
  a:
    provider: "local"
    model: "phi4-mini"
tasks:
  t:
    prompt:
      user: "step {{n}}"
run:
  name: "hitl-pause-concurrent"
  defaults:
    max_concurrency: 2
  workflow:
    kind: "concurrent"
    steps:
      - id: "s3"
        agent: "a"
        task: "t"
        inputs: { n: "3" }
      - id: "s1"
        agent: "a"
        task: "t"
        inputs: { n: "1" }
        guards:
          - type: pause
      - id: "s4"
        agent: "a"
        task: "t"
        inputs: { n: "4" }
      - id: "s2"
        agent: "a"
        task: "t"
        inputs: { n: "2" }
"#;
    let yaml_path = base.join("concurrent-pause.yaml");
    fs::write(&yaml_path, yaml).unwrap();

    let paused = run_swarm(&[yaml_path.to_str().unwrap(), "--run", "--trace"]);
    assert!(paused.status.success(), "paused run should succeed");
    let (run_json_path, _) = run_artifact_paths("hitl-pause-concurrent");
    let run_json: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&run_json_path).unwrap()).unwrap();
    assert_eq!(run_json["status"], "paused");
    let paused_resume_1 = base.join("resume-1.run.json");
    let paused_resume_2 = base.join("resume-2.run.json");
    fs::copy(&run_json_path, &paused_resume_1).unwrap();
    fs::copy(&run_json_path, &paused_resume_2).unwrap();

    let resumed1 = run_swarm(&[
        yaml_path.to_str().unwrap(),
        "--run",
        "--trace",
        "--resume",
        paused_resume_1.to_str().unwrap(),
    ]);
    let resumed2 = run_swarm(&[
        yaml_path.to_str().unwrap(),
        "--run",
        "--trace",
        "--resume",
        paused_resume_2.to_str().unwrap(),
    ]);
    assert!(resumed1.status.success(), "resume run #1 should succeed");
    assert!(resumed2.status.success(), "resume run #2 should succeed");
    let started1 = trace_started_step_ids(&String::from_utf8_lossy(&resumed1.stdout));
    let started2 = trace_started_step_ids(&String::from_utf8_lossy(&resumed2.stdout));
    assert_eq!(started1, started2);
}

#[test]
fn run_resume_trace_does_not_reemit_completed_step_chunks() {
    let base = tmp_dir("exec-pause-resume-no-duplicate-chunks");
    let _bin = write_mock_ollama(&base, MockOllamaBehavior::Success);
    let new_path = prepend_path(&base);
    let _path_guard = EnvVarGuard::set("PATH", new_path);

    let yaml = r#"
version: "0.3"
providers:
  local:
    type: "ollama"
agents:
  a:
    provider: "local"
    model: "phi4-mini"
tasks:
  t:
    prompt:
      user: "chunk {{n}}"
run:
  name: "hitl-pause-no-dup-chunks"
  workflow:
    kind: "sequential"
    steps:
      - id: "s1"
        agent: "a"
        task: "t"
        inputs: { n: "1" }
      - id: "s2"
        agent: "a"
        task: "t"
        inputs: { n: "2" }
        guards:
          - type: pause
      - id: "s3"
        agent: "a"
        task: "t"
        inputs: { n: "3" }
"#;
    let yaml_path = base.join("pause-no-dup-chunks.yaml");
    fs::write(&yaml_path, yaml).unwrap();

    let paused = run_swarm(&[yaml_path.to_str().unwrap(), "--run", "--trace"]);
    assert!(paused.status.success(), "paused run should succeed");
    let (run_json_path, _) = run_artifact_paths("hitl-pause-no-dup-chunks");

    let resumed = run_swarm(&[
        yaml_path.to_str().unwrap(),
        "--run",
        "--trace",
        "--resume",
        run_json_path.to_str().unwrap(),
    ]);
    assert!(resumed.status.success(), "resume run should succeed");

    let resumed_stdout = String::from_utf8_lossy(&resumed.stdout);
    let chunk_ids = trace_chunk_step_ids(&resumed_stdout);
    assert_eq!(
        chunk_ids,
        vec!["s3".to_string()],
        "resume trace should emit chunks only for remaining steps:\n{resumed_stdout}"
    );
}

#[test]
fn run_pause_resume_with_provider_profile_keeps_resume_plan_compatible() {
    let base = tmp_dir("exec-pause-resume-profile");
    let _bin = write_mock_ollama(&base, MockOllamaBehavior::Success);
    let new_path = prepend_path(&base);
    let _path_guard = EnvVarGuard::set("PATH", new_path);

    let yaml = r#"
version: "0.3"
providers:
  local:
    profile: "ollama:phi4-mini"
agents:
  a:
    provider: "local"
    model: "phi4-mini"
tasks:
  t:
    prompt:
      user: "profile {{n}}"
run:
  name: "hitl-pause-profile"
  workflow:
    kind: "sequential"
    steps:
      - id: "s1"
        agent: "a"
        task: "t"
        inputs: { n: "1" }
      - id: "s2"
        agent: "a"
        task: "t"
        inputs: { n: "2" }
        guards:
          - type: pause
      - id: "s3"
        agent: "a"
        task: "t"
        inputs: { n: "3" }
"#;
    let yaml_path = base.join("pause-profile.yaml");
    fs::write(&yaml_path, yaml).unwrap();

    let paused = run_swarm(&[yaml_path.to_str().unwrap(), "--run"]);
    assert!(paused.status.success(), "paused run should succeed");
    let (run_json_path, _) = run_artifact_paths("hitl-pause-profile");

    let resumed = run_swarm(&[
        yaml_path.to_str().unwrap(),
        "--run",
        "--resume",
        run_json_path.to_str().unwrap(),
    ]);
    assert!(
        resumed.status.success(),
        "resume with profile-expanded provider should succeed.\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&resumed.stdout),
        String::from_utf8_lossy(&resumed.stderr)
    );
}

#[test]
fn run_resume_rejects_modified_plan() {
    let base = tmp_dir("exec-resume-plan-mismatch");
    let _bin = write_mock_ollama(&base, MockOllamaBehavior::EchoPrompt);
    let new_path = prepend_path(&base);
    let _path_guard = EnvVarGuard::set("PATH", new_path);

    let yaml_a = r#"
version: "0.3"
providers: { local: { type: "ollama" } }
agents: { a: { provider: "local", model: "phi4-mini" } }
tasks: { t: { prompt: { user: "step {{n}}" } } }
run:
  name: "hitl-resume-mismatch"
  workflow:
    kind: "sequential"
    steps:
      - id: "s1"
        agent: "a"
        task: "t"
        inputs: { n: "1" }
        guards: [ { type: pause } ]
      - id: "s2"
        agent: "a"
        task: "t"
        inputs: { n: "2" }
"#;
    let yaml_b = yaml_a.replace("id: \"s2\"", "id: \"s2_changed\"");
    let a_path = base.join("a.yaml");
    let b_path = base.join("b.yaml");
    fs::write(&a_path, yaml_a).unwrap();
    fs::write(&b_path, yaml_b).unwrap();

    let paused = run_swarm(&[a_path.to_str().unwrap(), "--run"]);
    assert!(paused.status.success(), "paused run should succeed");
    let (run_json_path, _) = run_artifact_paths("hitl-resume-mismatch");

    let resumed = run_swarm(&[
        b_path.to_str().unwrap(),
        "--run",
        "--resume",
        run_json_path.to_str().unwrap(),
    ]);
    assert!(
        !resumed.status.success(),
        "resume with modified plan must fail"
    );
    let stderr = String::from_utf8_lossy(&resumed.stderr);
    assert!(
        stderr.contains("execution plan mismatch"),
        "stderr was:\n{stderr}"
    );
}

#[test]
fn run_resume_rejects_non_paused_state_file() {
    let base = tmp_dir("exec-resume-invalid-state");
    let _bin = write_mock_ollama(&base, MockOllamaBehavior::EchoPrompt);
    let new_path = prepend_path(&base);
    let _path_guard = EnvVarGuard::set("PATH", new_path);

    let yaml = r#"
version: "0.3"
providers: { local: { type: "ollama" } }
agents: { a: { provider: "local", model: "phi4-mini" } }
tasks: { t: { prompt: { user: "step {{n}}" } } }
run:
  name: "hitl-invalid-state"
  workflow:
    kind: "sequential"
    steps:
      - id: "s1"
        agent: "a"
        task: "t"
        inputs: { n: "1" }
"#;
    let yaml_path = base.join("state.yaml");
    fs::write(&yaml_path, yaml).unwrap();

    let first = run_swarm(&[yaml_path.to_str().unwrap(), "--run"]);
    assert!(first.status.success(), "initial run should succeed");
    let (run_json_path, _) = run_artifact_paths("hitl-invalid-state");

    let resumed = run_swarm(&[
        yaml_path.to_str().unwrap(),
        "--run",
        "--resume",
        run_json_path.to_str().unwrap(),
    ]);
    assert!(
        !resumed.status.success(),
        "resume should fail for success run.json"
    );
    let stderr = String::from_utf8_lossy(&resumed.stderr);
    assert!(stderr.contains("status='paused'"), "stderr was:\n{stderr}");
}
