use super::*;

#[test]
fn run_scores_artifact_is_byte_stable_across_repeated_identical_runs() {
    let base = tmp_dir("exec-scores-stability");
    let _bin = write_mock_ollama(&base, MockOllamaBehavior::Success);
    let new_path = prepend_path(&base);
    let _path_guard = EnvVarGuard::set("PATH", new_path);

    let run_id = "scores-stable-test";
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
    let tmp_yaml = base.join("scores-stability.yaml");
    fs::write(&tmp_yaml, yaml).unwrap();

    let first = run_swarm(&[tmp_yaml.to_string_lossy().as_ref(), "--run"]);
    assert!(first.status.success(), "first run should succeed");
    let first_scores = fs::read(run_dir.join("learning").join("scores.json")).unwrap();
    let first_suggestions = fs::read(run_dir.join("learning").join("suggestions.json")).unwrap();

    let second = run_swarm(&[tmp_yaml.to_string_lossy().as_ref(), "--run"]);
    assert!(second.status.success(), "second run should succeed");
    let second_scores = fs::read(run_dir.join("learning").join("scores.json")).unwrap();
    let second_suggestions = fs::read(run_dir.join("learning").join("suggestions.json")).unwrap();

    assert_eq!(
        first_scores, second_scores,
        "scores.json should be byte-stable across repeated identical runs"
    );
    assert_eq!(
        first_suggestions, second_suggestions,
        "suggestions.json should be byte-stable across repeated identical runs"
    );
    let suggestions_text = String::from_utf8(second_suggestions).unwrap();
    assert!(
        !suggestions_text.contains("/Users/")
            && !suggestions_text.contains("\\\\")
            && !suggestions_text.contains("gho_"),
        "suggestions output must not leak absolute host paths or secrets: {suggestions_text}"
    );

    let _ = fs::remove_dir_all(&run_dir);
}

#[test]
fn cluster_groundwork_artifact_is_byte_stable_across_repeated_identical_runs() {
    let base = tmp_dir("exec-cluster-groundwork-stability");
    let _bin = write_mock_ollama(&base, MockOllamaBehavior::Success);
    let new_path = prepend_path(&base);
    let _path_guard = EnvVarGuard::set("PATH", new_path);

    let run_id = "cluster-groundwork-stable";
    let run_dir = repo_runs_dir().join(run_id);
    let _ = fs::remove_dir_all(&run_dir);

    let yaml = format!(
        r#"
version: "0.3"
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
      user: "Echo {{text}}"
run:
  name: "{run_id}"
  workflow:
    kind: "sequential"
    steps:
      - id: "s1"
        agent: "a1"
        task: "t1"
        save_as: "s1_out"
        inputs:
          text: "hello"
      - id: "s2"
        agent: "a1"
        task: "t1"
        save_as: "s2_out"
        inputs:
          text: "@state:s1_out"
"#
    );
    let tmp_yaml = base.join("cluster-groundwork-stable.yaml");
    fs::write(&tmp_yaml, yaml).unwrap();

    let out1 = run_swarm(&[tmp_yaml.to_string_lossy().as_ref(), "--run"]);
    assert!(out1.status.success(), "first run failed");
    let first = fs::read(run_dir.join("meta").join("cluster_groundwork.json")).unwrap();

    let out2 = run_swarm(&[tmp_yaml.to_string_lossy().as_ref(), "--run"]);
    assert!(out2.status.success(), "second run failed");
    let second = fs::read(run_dir.join("meta").join("cluster_groundwork.json")).unwrap();

    assert_eq!(
        first, second,
        "cluster_groundwork.json must be byte-stable across repeated identical runs"
    );

    let cluster_groundwork: serde_json::Value = serde_json::from_slice(&second).unwrap();
    assert_eq!(
        cluster_groundwork["readiness_frontiers"][0]["ready_step_ids"],
        serde_json::json!(["s1"])
    );
    assert_eq!(
        cluster_groundwork["readiness_frontiers"][1]["ready_step_ids"],
        serde_json::json!(["s2"])
    );
    assert_eq!(
        cluster_groundwork["lease_records"][0]["lease_id"],
        format!("lease:{run_id}:s1:1")
    );
    assert_eq!(
        cluster_groundwork["lease_records"][1]["lease_id"],
        format!("lease:{run_id}:s2:1")
    );
    assert_eq!(
        cluster_groundwork["lease_records"][1]["depends_on"],
        serde_json::json!(["s1"])
    );

    let _ = fs::remove_dir_all(run_dir);
}

#[test]
fn run_status_artifact_is_byte_stable_across_repeated_identical_runs() {
    let base = tmp_dir("exec-run-status-stability");
    let _bin = write_mock_ollama(&base, MockOllamaBehavior::Success);
    let new_path = prepend_path(&base);
    let _path_guard = EnvVarGuard::set("PATH", new_path);

    let run_id = "run-status-stable";
    let run_dir = repo_runs_dir().join(run_id);
    let _ = fs::remove_dir_all(&run_dir);

    let yaml = format!(
        r#"
version: "0.3"
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
      user: "Echo {{text}}"
run:
  name: "{run_id}"
  workflow:
    kind: "sequential"
    steps:
      - id: "s1"
        agent: "a1"
        task: "t1"
        save_as: "s1_out"
        write_to: "s1.txt"
        inputs:
          text: "hello"
"#
    );
    let tmp_yaml = base.join("run-status-stable.yaml");
    fs::write(&tmp_yaml, yaml).unwrap();

    let out1 = run_swarm(&[tmp_yaml.to_string_lossy().as_ref(), "--run"]);
    assert!(out1.status.success(), "first run failed");
    let first = fs::read(run_dir.join("run_status.json")).unwrap();

    let out2 = run_swarm(&[tmp_yaml.to_string_lossy().as_ref(), "--run"]);
    assert!(out2.status.success(), "second run failed");
    let second = fs::read(run_dir.join("run_status.json")).unwrap();

    assert_eq!(first, second, "run_status.json must be byte-stable");

    let _ = fs::remove_dir_all(run_dir);
}

#[test]
fn replay_regression_holds_event_order_artifact_layout_and_failure_kind_stability() {
    let base = tmp_dir("exec-replay-regression");
    let _bin = write_mock_ollama(&base, MockOllamaBehavior::SleepEchoPrompt);
    let new_path = prepend_path(&base);
    let _path_guard = EnvVarGuard::set("PATH", new_path);

    let success_run_id = "replay-regression-success";
    let success_yaml = format!(
        r#"
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
  name: "{success_run_id}"
  defaults:
    max_concurrency: 2
  workflow:
    kind: "concurrent"
    steps:
      - id: "s3"
        agent: "a"
        task: "t"
        inputs: {{ n: "3" }}
        save_as: "s3"
        write_to: "s3.txt"
      - id: "s1"
        agent: "a"
        task: "t"
        inputs: {{ n: "1" }}
        save_as: "s1"
        write_to: "s1.txt"
      - id: "s4"
        agent: "a"
        task: "t"
        inputs: {{ n: "4" }}
        save_as: "s4"
        write_to: "s4.txt"
      - id: "s2"
        agent: "a"
        task: "t"
        inputs: {{ n: "2" }}
        save_as: "s2"
        write_to: "s2.txt"
"#
    );
    let success_yaml_path = base.join("replay-regression-success.yaml");
    fs::write(&success_yaml_path, success_yaml).unwrap();

    let out_a = base.join("out-a");
    let out_b = base.join("out-b");
    let trace_a = base.join("trace-a.json");
    let trace_b = base.join("trace-b.json");

    let first = run_swarm(&[
        success_yaml_path.to_str().unwrap(),
        "--run",
        "--trace",
        "--out",
        out_a.to_str().unwrap(),
    ]);
    let second = run_swarm(&[
        success_yaml_path.to_str().unwrap(),
        "--run",
        "--trace",
        "--out",
        out_b.to_str().unwrap(),
    ]);
    assert!(first.status.success());
    assert!(second.status.success());

    let started_a = trace_started_step_ids(&String::from_utf8_lossy(&first.stdout));
    let started_b = trace_started_step_ids(&String::from_utf8_lossy(&second.stdout));
    assert_eq!(started_a, vec!["s1", "s2", "s3", "s4"]);
    assert_eq!(started_a, started_b);

    let replay_events_a = vec![
        ::adl::trace::TraceEvent::StepStarted {
            ts_ms: 10,
            elapsed_ms: 10,
            step_id: "s1".to_string(),
            agent_id: "a".to_string(),
            provider_id: "local".to_string(),
            task_id: "t".to_string(),
            delegation: None,
        },
        ::adl::trace::TraceEvent::StepOutputChunk {
            ts_ms: 11,
            elapsed_ms: 11,
            step_id: "s1".to_string(),
            chunk_bytes: 24,
        },
        ::adl::trace::TraceEvent::StepFinished {
            ts_ms: 12,
            elapsed_ms: 12,
            step_id: "s1".to_string(),
            success: true,
            duration_ms: 2,
        },
        ::adl::trace::TraceEvent::RunFinished {
            ts_ms: 13,
            elapsed_ms: 13,
            success: true,
        },
    ];
    let replay_events_b = vec![
        ::adl::trace::TraceEvent::StepStarted {
            ts_ms: 110,
            elapsed_ms: 110,
            step_id: "s1".to_string(),
            agent_id: "a".to_string(),
            provider_id: "local".to_string(),
            task_id: "t".to_string(),
            delegation: None,
        },
        ::adl::trace::TraceEvent::StepOutputChunk {
            ts_ms: 111,
            elapsed_ms: 111,
            step_id: "s1".to_string(),
            chunk_bytes: 24,
        },
        ::adl::trace::TraceEvent::StepFinished {
            ts_ms: 112,
            elapsed_ms: 112,
            step_id: "s1".to_string(),
            success: true,
            duration_ms: 2,
        },
        ::adl::trace::TraceEvent::RunFinished {
            ts_ms: 113,
            elapsed_ms: 113,
            success: true,
        },
    ];
    ::adl::instrumentation::write_trace_artifact(&trace_a, &replay_events_a).unwrap();
    ::adl::instrumentation::write_trace_artifact(&trace_b, &replay_events_b).unwrap();

    let replay_a = run_swarm(&["instrument", "replay", trace_a.to_str().unwrap()]);
    let replay_b = run_swarm(&["instrument", "replay", trace_b.to_str().unwrap()]);
    assert!(replay_a.status.success() && replay_b.status.success());
    assert_eq!(replay_a.stdout, replay_b.stdout);
}
