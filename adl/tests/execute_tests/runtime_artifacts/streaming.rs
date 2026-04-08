use super::*;

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
    assert!(out.status.success());
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
    assert!(run_stream.status.success());

    let out_quiet = base.join("out-quiet");
    let run_quiet = run_swarm(&[
        yaml_path.to_str().unwrap(),
        "--run",
        "--quiet",
        "--out",
        out_quiet.to_str().unwrap(),
    ]);
    assert!(run_quiet.status.success());

    let stream_html = fs::read_to_string(out_stream.join("index.html")).unwrap();
    let quiet_html = fs::read_to_string(out_quiet.join("index.html")).unwrap();
    assert_eq!(stream_html, quiet_html);
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
    assert!(out1.status.success());
    assert!(out2.status.success());

    let stdout1 = String::from_utf8_lossy(&out1.stdout);
    let stdout2 = String::from_utf8_lossy(&out2.stdout);
    assert!(stdout1.contains("StepOutputChunk step=s1"));
    assert!(stdout1.contains("StepOutputChunk step=s2"));
    assert!(stdout1.contains("StepOutputChunk step=s3"));
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
fn run_streaming_mid_step_failure_is_observational_and_resume_rejected() {
    let base = tmp_dir("exec-streaming-mid-failure");
    let _bin = write_mock_ollama(&base, MockOllamaBehavior::StreamThenFailOnToken);
    let new_path = prepend_path(&base);
    let _path_guard = EnvVarGuard::set("PATH", new_path);

    let yaml_stream = r#"
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
      user: "stream {{text}}"
run:
  name: "streaming-mid-failure"
  workflow:
    kind: "sequential"
    steps:
      - id: "s1"
        agent: "a"
        task: "t"
        inputs:
          text: "ok"
        save_as: "s1_out"
        write_to: "s1.txt"
      - id: "s2"
        agent: "a"
        task: "t"
        inputs:
          text: "FAIL_THIS_STEP"
        save_as: "s2_out"
        write_to: "s2.txt"
      - id: "s3"
        agent: "a"
        task: "t"
        inputs:
          text: "never"
        save_as: "s3_out"
        write_to: "s3.txt"
"#;
    let yaml_quiet = yaml_stream.replace("streaming-mid-failure", "streaming-mid-failure-quiet");
    let yaml_stream_path = base.join("streaming-mid-failure.yaml");
    let yaml_quiet_path = base.join("streaming-mid-failure-quiet.yaml");
    fs::write(&yaml_stream_path, yaml_stream).unwrap();
    fs::write(&yaml_quiet_path, &yaml_quiet).unwrap();

    let out_stream_dir = base.join("out-stream");
    let out_stream = run_swarm(&[
        yaml_stream_path.to_str().unwrap(),
        "--run",
        "--trace",
        "--out",
        out_stream_dir.to_str().unwrap(),
    ]);
    let out_quiet_dir = base.join("out-quiet");
    let out_quiet = run_swarm(&[
        yaml_quiet_path.to_str().unwrap(),
        "--run",
        "--quiet",
        "--out",
        out_quiet_dir.to_str().unwrap(),
    ]);
    assert!(!out_stream.status.success());
    assert!(!out_quiet.status.success());

    let trace_stdout = String::from_utf8_lossy(&out_stream.stdout);
    assert!(trace_stdout.contains("StepOutputChunk step=s2"));
}

#[test]
fn run_emits_progress_banners_on_stderr() {
    let base = tmp_dir("exec-progress-banners");
    let _bin = write_mock_ollama(&base, MockOllamaBehavior::Success);
    let new_path = prepend_path(&base);
    let _path_guard = EnvVarGuard::set("PATH", new_path);

    let out = run_swarm(&["examples/v0-3-concurrency-fork-join.adl.yaml", "--run"]);
    assert!(out.status.success());

    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("RUN start"));
    assert!(stderr.contains("STEP start"));
    assert!(stderr.contains("STEP done"));
    assert!(stderr.contains("RUN done"));
    assert!(stderr.contains("duration_ms="));
}
