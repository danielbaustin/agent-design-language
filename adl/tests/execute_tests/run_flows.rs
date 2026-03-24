use super::*;

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
            "ADL_REMOTE_BEARER_TOKEN",
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
