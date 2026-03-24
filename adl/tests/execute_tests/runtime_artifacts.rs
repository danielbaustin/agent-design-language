use super::*;

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
        save_as: "s1"
        write_to: "s1.txt"
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
    let run_status_path = run_dir.join("run_status.json");
    let steps_json_path = run_dir.join("steps.json");
    let run_summary_path = run_dir.join("run_summary.json");
    let scores_path = run_dir.join("learning").join("scores.json");
    let suggestions_path = run_dir.join("learning").join("suggestions.json");
    let aee_decision_path = run_dir.join("learning").join("aee_decision.json");
    let cluster_groundwork_path = run_dir.join("meta").join("cluster_groundwork.json");
    assert!(
        run_json_path.is_file(),
        "missing {}",
        run_json_path.display()
    );
    assert!(
        run_status_path.is_file(),
        "missing {}",
        run_status_path.display()
    );
    assert!(
        steps_json_path.is_file(),
        "missing {}",
        steps_json_path.display()
    );
    assert!(
        run_summary_path.is_file(),
        "missing {}",
        run_summary_path.display()
    );
    assert!(scores_path.is_file(), "missing {}", scores_path.display());
    assert!(
        suggestions_path.is_file(),
        "missing {}",
        suggestions_path.display()
    );
    assert!(
        aee_decision_path.is_file(),
        "missing {}",
        aee_decision_path.display()
    );
    assert!(
        cluster_groundwork_path.is_file(),
        "missing {}",
        cluster_groundwork_path.display()
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
    assert_eq!(steps[0]["output_artifact_path"], "s1.txt");

    let run_status_json: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&run_status_path).unwrap()).unwrap();
    assert_eq!(run_status_json["run_status_version"], 1);
    assert_eq!(run_status_json["run_id"], run_id);
    assert_eq!(run_status_json["workflow_id"], "workflow");
    assert_eq!(run_status_json["overall_status"], "succeeded");
    assert_eq!(
        run_status_json["completed_steps"],
        serde_json::json!(["s1"])
    );
    assert_eq!(run_status_json["pending_steps"], serde_json::json!([]));
    assert_eq!(run_status_json["started_steps"], serde_json::json!(["s1"]));
    assert_eq!(run_status_json["attempt_counts_by_step"]["s1"], 1);
    let run_status_raw = fs::read_to_string(&run_status_path).unwrap();
    assert!(
        !run_status_raw.contains(base.to_str().unwrap()),
        "run_status.json must not leak absolute host paths:\n{run_status_raw}"
    );

    let summary_json: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&run_summary_path).unwrap()).unwrap();
    assert_eq!(summary_json["run_summary_version"], 1);
    assert_eq!(summary_json["artifact_model_version"], 1);
    assert_eq!(summary_json["run_id"], run_id);
    assert_eq!(summary_json["workflow_id"], "workflow");
    assert_eq!(summary_json["status"], "success");
    assert_eq!(summary_json["links"]["run_json"], "run.json");
    assert_eq!(summary_json["links"]["steps_json"], "steps.json");
    assert_eq!(summary_json["links"]["outputs_dir"], "outputs");
    assert_eq!(summary_json["links"]["learning_dir"], "learning");
    assert_eq!(summary_json["links"]["scores_json"], "learning/scores.json");
    assert_eq!(
        summary_json["links"]["suggestions_json"],
        "learning/suggestions.json"
    );
    assert_eq!(
        summary_json["links"]["aee_decision_json"],
        "learning/aee_decision.json"
    );
    assert_eq!(
        summary_json["links"]["cluster_groundwork_json"],
        "meta/cluster_groundwork.json"
    );
    assert!(
        summary_json
            .get("links")
            .and_then(|v| v.get("trace_json"))
            .is_none(),
        "trace_json should be omitted when tracing is disabled"
    );
    assert!(
        summary_json
            .get("links")
            .and_then(|v| v.get("pause_state_json"))
            .is_none(),
        "pause_state_json should be omitted for non-paused runs"
    );
    assert!(
        summary_json.get("error_kind").is_none(),
        "error_kind should be omitted for successful runs"
    );
    assert!(
        summary_json.get("started_at").is_none(),
        "run summary v1 should avoid wall-clock timestamps by default"
    );
    let scores_json: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&scores_path).unwrap()).unwrap();
    assert_eq!(scores_json["scores_version"], 1);
    assert_eq!(scores_json["run_id"], run_id);
    assert_eq!(scores_json["generated_from"]["artifact_model_version"], 1);
    assert_eq!(scores_json["generated_from"]["run_summary_version"], 1);
    assert!(scores_json["summary"]["success_ratio"].is_number());
    assert!(scores_json["summary"]["failure_count"].is_number());
    assert!(scores_json["summary"]["retry_count"].is_number());
    assert!(
        scores_json["metrics"]["scheduler_max_parallel_observed"].is_number(),
        "scores metrics should include deterministic scheduler observation"
    );
    let suggestions_json: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&suggestions_path).unwrap()).unwrap();
    assert_eq!(suggestions_json["suggestions_version"], 1);
    assert_eq!(suggestions_json["run_id"], run_id);
    assert_eq!(
        suggestions_json["generated_from"]["artifact_model_version"],
        1
    );
    assert_eq!(suggestions_json["generated_from"]["run_summary_version"], 1);
    assert_eq!(suggestions_json["generated_from"]["scores_version"], 1);
    let suggestions = suggestions_json["suggestions"]
        .as_array()
        .expect("suggestions should be an array");
    for (idx, item) in suggestions.iter().enumerate() {
        assert_eq!(item["id"], format!("sug-{:03}", idx + 1));
        let proposed_change = &item["proposed_change"];
        assert!(proposed_change["intent"].is_string());
        assert!(proposed_change["target"].is_string());
    }
    let aee_decision_json: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&aee_decision_path).unwrap()).unwrap();
    assert_eq!(aee_decision_json["aee_decision_version"], 1);
    assert_eq!(aee_decision_json["run_id"], run_id);
    assert_eq!(
        aee_decision_json["generated_from"]["artifact_model_version"],
        1
    );
    assert_eq!(
        aee_decision_json["generated_from"]["run_summary_version"],
        1
    );
    assert_eq!(
        aee_decision_json["generated_from"]["suggestions_version"],
        1
    );
    assert_eq!(aee_decision_json["generated_from"]["scores_version"], 1);
    assert!(aee_decision_json["decision"]["decision_id"].is_string());
    assert!(aee_decision_json["decision"]["intent"].is_string());
    assert!(aee_decision_json["decision"]["deterministic_selection_rule"].is_string());
    let cluster_groundwork_json: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&cluster_groundwork_path).unwrap()).unwrap();
    assert_eq!(cluster_groundwork_json["cluster_groundwork_version"], 1);
    assert_eq!(cluster_groundwork_json["run_id"], run_id);
    assert_eq!(cluster_groundwork_json["workflow_id"], "workflow");
    assert_eq!(
        cluster_groundwork_json["canonical_ordering_key"],
        "(run_id, step_id, attempt)"
    );
    assert_eq!(
        cluster_groundwork_json["frontier_ordering"],
        "topological_frontier_then_step_id"
    );
    assert_eq!(
        cluster_groundwork_json["lease_records"][0]["claim_owner"],
        "adl-coordinator-local"
    );
    assert_eq!(
        cluster_groundwork_json["lease_records"][0]["worker_id"],
        "adl-worker-local"
    );
    assert_eq!(
        cluster_groundwork_json["lease_records"][0]["lease_state"],
        "completed"
    );
    let cluster_groundwork_raw = fs::read_to_string(&cluster_groundwork_path).unwrap();
    assert!(
        !cluster_groundwork_raw.contains(base.to_str().unwrap()),
        "cluster_groundwork.json must not leak absolute host paths:\n{cluster_groundwork_raw}"
    );

    let _ = fs::remove_dir_all(&run_dir);
}

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
    let success_run_dir = repo_runs_dir().join(success_run_id);
    let _ = fs::remove_dir_all(&success_run_dir);

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
    assert!(
        first.status.success(),
        "first success run should pass.\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&first.stdout),
        String::from_utf8_lossy(&first.stderr)
    );

    let second = run_swarm(&[
        success_yaml_path.to_str().unwrap(),
        "--run",
        "--trace",
        "--out",
        out_b.to_str().unwrap(),
    ]);
    assert!(
        second.status.success(),
        "second success run should pass.\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&second.stdout),
        String::from_utf8_lossy(&second.stderr)
    );

    // Stable event ordering: start order is deterministic across repeated runs.
    let started_a = trace_started_step_ids(&String::from_utf8_lossy(&first.stdout));
    let started_b = trace_started_step_ids(&String::from_utf8_lossy(&second.stdout));
    assert_eq!(started_a, vec!["s1", "s2", "s3", "s4"]);
    assert_eq!(started_a, started_b);

    // Replay runner determinism against WP-02 activation-log schema wrapper.
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

    // Replay runner determinism: equivalent activation logs must produce identical replay JSON.
    let replay_a = run_swarm(&["instrument", "replay", trace_a.to_str().unwrap()]);
    let replay_b = run_swarm(&["instrument", "replay", trace_b.to_str().unwrap()]);
    assert!(
        replay_a.status.success() && replay_b.status.success(),
        "replay should succeed.\nreplay_a stderr:\n{}\nreplay_b stderr:\n{}",
        String::from_utf8_lossy(&replay_a.stderr),
        String::from_utf8_lossy(&replay_b.stderr)
    );
    assert_eq!(
        replay_a.stdout, replay_b.stdout,
        "replay output JSON should be byte-stable across repeated equivalent runs"
    );

    // Stable artifact layout and stable output bytes.
    let collect_files = |root: &Path| -> Vec<(String, Vec<u8>)> {
        fn walk(root: &Path, cur: &Path, out: &mut Vec<(String, Vec<u8>)>) {
            let mut entries: Vec<_> = fs::read_dir(cur)
                .unwrap()
                .map(|e| e.unwrap().path())
                .collect();
            entries.sort();
            for p in entries {
                if p.is_dir() {
                    walk(root, &p, out);
                } else {
                    let rel = p
                        .strip_prefix(root)
                        .unwrap()
                        .to_string_lossy()
                        .replace('\\', "/");
                    out.push((rel, fs::read(&p).unwrap()));
                }
            }
        }
        let mut out = Vec::new();
        walk(root, root, &mut out);
        out
    };
    assert_eq!(
        collect_files(&out_a),
        collect_files(&out_b),
        "output artifact tree and bytes should be stable across repeated runs"
    );

    let failure_run_id = "replay-regression-failure";
    let failure_run_dir = repo_runs_dir().join(failure_run_id);
    let _ = fs::remove_dir_all(&failure_run_dir);
    let failure_yaml = format!(
        r#"
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
  name: "{failure_run_id}"
  delegation_policy:
    default_allow: true
    rules:
      - id: "deny-local-provider"
        action: provider_call
        target_id: "local"
        effect: deny
  workflow:
    kind: sequential
    steps:
      - id: "s1"
        agent: "a"
        task: "t"
        inputs: {{ n: "1" }}
        delegation:
          role: "reviewer"
"#
    );
    let failure_yaml_path = base.join("replay-regression-failure.yaml");
    fs::write(&failure_yaml_path, failure_yaml).unwrap();

    let fail1 = run_swarm(&[failure_yaml_path.to_str().unwrap(), "--run", "--trace"]);
    assert!(
        !fail1.status.success(),
        "expected first policy-denied run to fail"
    );
    let status1: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(failure_run_dir.join("run_status.json")).unwrap())
            .unwrap();
    assert_eq!(status1["failure_kind"], "policy_denied");

    let fail2 = run_swarm(&[failure_yaml_path.to_str().unwrap(), "--run", "--trace"]);
    assert!(
        !fail2.status.success(),
        "expected second policy-denied run to fail"
    );
    let status2: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(failure_run_dir.join("run_status.json")).unwrap())
            .unwrap();
    assert_eq!(status2["failure_kind"], "policy_denied");
    let fail_trace_a = base.join("trace-fail-a.json");
    let fail_trace_b = base.join("trace-fail-b.json");
    let fail_events_a = vec![
        ::adl::trace::TraceEvent::StepStarted {
            ts_ms: 20,
            elapsed_ms: 20,
            step_id: "s1".to_string(),
            agent_id: "a".to_string(),
            provider_id: "local".to_string(),
            task_id: "t".to_string(),
            delegation: None,
        },
        ::adl::trace::TraceEvent::RunFailed {
            ts_ms: 21,
            elapsed_ms: 21,
            message: "DELEGATION_POLICY_DENIED: denied".to_string(),
        },
    ];
    let fail_events_b = vec![
        ::adl::trace::TraceEvent::StepStarted {
            ts_ms: 120,
            elapsed_ms: 120,
            step_id: "s1".to_string(),
            agent_id: "a".to_string(),
            provider_id: "local".to_string(),
            task_id: "t".to_string(),
            delegation: None,
        },
        ::adl::trace::TraceEvent::RunFailed {
            ts_ms: 121,
            elapsed_ms: 121,
            message: "DELEGATION_POLICY_DENIED: denied".to_string(),
        },
    ];
    ::adl::instrumentation::write_trace_artifact(&fail_trace_a, &fail_events_a).unwrap();
    ::adl::instrumentation::write_trace_artifact(&fail_trace_b, &fail_events_b).unwrap();

    let fail_replay_a = run_swarm(&["instrument", "replay", fail_trace_a.to_str().unwrap()]);
    let fail_replay_b = run_swarm(&["instrument", "replay", fail_trace_b.to_str().unwrap()]);
    assert!(
        fail_replay_a.status.success() && fail_replay_b.status.success(),
        "failed-run replay should still parse deterministically"
    );
    assert_eq!(
        fail_replay_a.stdout, fail_replay_b.stdout,
        "failed-run replay output should be stable"
    );
}

#[test]
fn run_status_failure_kind_maps_timeout_without_raw_provider_error_text() {
    let listener = match TcpListener::bind("127.0.0.1:0") {
        Ok(listener) => listener,
        Err(e) => panic!("failed to bind local test server: {e}"),
    };
    let bind_addr = listener.local_addr().unwrap();
    thread::spawn(move || {
        if let Ok((mut stream, _)) = listener.accept() {
            let mut buf = [0_u8; 1024];
            let _ = stream.read(&mut buf);
            std::thread::sleep(Duration::from_millis(1200));
            let _ = stream.write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\n\r\n{}");
        }
    });

    let base = tmp_dir("exec-run-status-timeout");
    let run_id = "run-status-timeout";
    let run_dir = repo_runs_dir().join(run_id);
    let _ = fs::remove_dir_all(&run_dir);
    let yaml = format!(
        r#"
version: "0.5"
providers:
  http:
    type: "http"
    config:
      endpoint: "http://{bind_addr}"
      timeout_secs: 1
agents:
  a1:
    provider: "http"
    model: "unused-for-http-provider"
tasks:
  t1:
    prompt:
      user: "timeout"
run:
  name: "{run_id}"
  workflow:
    kind: "sequential"
    steps:
      - id: "s1"
        agent: "a1"
        task: "t1"
"#
    );
    let tmp_yaml = base.join("run-status-timeout.yaml");
    fs::write(&tmp_yaml, yaml).unwrap();

    let out = run_swarm(&[tmp_yaml.to_string_lossy().as_ref(), "--run"]);
    assert!(!out.status.success(), "timeout run must fail");

    let run_status: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(run_dir.join("run_status.json")).unwrap())
            .unwrap();
    assert_eq!(run_status["overall_status"], "failed");
    assert_eq!(run_status["failure_kind"], "timeout");
    let raw = serde_json::to_string_pretty(&run_status).unwrap();
    assert!(!raw.contains("127.0.0.1"));
    assert!(!raw.contains("providers.<id>.config.timeout_secs"));

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
    assert!(
        !out_stream.status.success(),
        "streaming run must fail.\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out_stream.stdout),
        String::from_utf8_lossy(&out_stream.stderr)
    );

    let out_quiet_dir = base.join("out-quiet");
    let out_quiet = run_swarm(&[
        yaml_quiet_path.to_str().unwrap(),
        "--run",
        "--quiet",
        "--out",
        out_quiet_dir.to_str().unwrap(),
    ]);
    assert!(
        !out_quiet.status.success(),
        "non-stream baseline must fail.\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out_quiet.stdout),
        String::from_utf8_lossy(&out_quiet.stderr)
    );

    let collect_artifacts = |root: &Path| -> Vec<(String, Vec<u8>)> {
        if !root.exists() {
            return Vec::new();
        }
        let mut files: Vec<_> = fs::read_dir(root)
            .unwrap()
            .map(|e| e.unwrap().path())
            .filter(|p| p.is_file())
            .collect();
        files.sort();
        files
            .into_iter()
            .map(|path| {
                (
                    path.file_name().unwrap().to_string_lossy().to_string(),
                    fs::read(&path).unwrap(),
                )
            })
            .collect()
    };
    assert_eq!(
        collect_artifacts(&out_stream_dir),
        collect_artifacts(&out_quiet_dir),
        "streaming failure mode must not change final artifact bytes"
    );

    let trace_stdout = String::from_utf8_lossy(&out_stream.stdout);
    let run_id = trace_stdout
        .lines()
        .find_map(|line| {
            let (_, tail) = line.split_once("TRACE run_id=")?;
            Some(tail.split_whitespace().next()?.to_string())
        })
        .expect("expected TRACE header with run_id");

    let run_dir_from_stderr = String::from_utf8_lossy(&out_stream.stderr)
        .lines()
        .find_map(|line| {
            let marker = "artifacts=";
            let (_, tail) = line.split_once(marker)?;
            Some(Path::new(tail.trim()).to_path_buf())
        });
    let run_json_path = run_dir_from_stderr
        .map(|dir| dir.join("run.json"))
        .unwrap_or_else(|| run_artifact_paths(&run_id).0);
    assert!(
        run_json_path.exists(),
        "expected run.json at '{}'\nstdout:\n{}\nstderr:\n{}",
        run_json_path.display(),
        trace_stdout,
        String::from_utf8_lossy(&out_stream.stderr)
    );
    let run_json: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&run_json_path).unwrap()).unwrap();
    assert_eq!(run_json["status"], "failure");
    let run_dir = run_json_path.parent().unwrap();
    assert!(
        !run_dir.join("pause_state.json").exists(),
        "pause_state.json must not be written for non-paused failure runs"
    );

    let resume_after_failure = run_swarm(&["resume", &run_id]);
    assert!(
        !resume_after_failure.status.success(),
        "resume must fail from failure state"
    );
    let resume_stderr = String::from_utf8_lossy(&resume_after_failure.stderr);
    assert!(
        resume_stderr.contains("pause state not found")
            || resume_stderr.contains("status='paused'")
            || resume_stderr.contains("status=\"paused\""),
        "resume error should reject failed/non-paused runs; stderr:\n{resume_stderr}"
    );

    assert!(
        trace_stdout.contains("StepOutputChunk step=s2"),
        "expected streamed chunk for failing step before failure:\n{trace_stdout}"
    );
    let lines: Vec<&str> = trace_stdout.lines().collect();
    let run_failed_idx = lines
        .iter()
        .position(|line| line.contains("RunFailed"))
        .expect("missing RunFailed trace event");
    let chunk_indices: Vec<usize> = lines
        .iter()
        .enumerate()
        .filter_map(|(i, line)| line.contains("StepOutputChunk step=").then_some(i))
        .collect();
    assert!(
        !chunk_indices.is_empty(),
        "expected at least one StepOutputChunk event before failure"
    );
    assert!(
        chunk_indices.iter().all(|idx| *idx < run_failed_idx),
        "chunk events must not appear after terminal RunFailed event"
    );
    assert!(
        !trace_stdout.contains("StepStarted step=s3"),
        "s3 should not start after fail-fast s2 failure"
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

    let pattern = ::adl::adl::PatternSpec {
        id: "p_fork".to_string(),
        kind: ::adl::adl::PatternKind::ForkJoin,
        steps: vec![],
        fork: Some(::adl::adl::PatternForkSpec {
            branches: vec![
                ::adl::adl::PatternBranchSpec {
                    id: "left".to_string(),
                    steps: vec!["L1".to_string(), "L2".to_string()],
                },
                ::adl::adl::PatternBranchSpec {
                    id: "right".to_string(),
                    steps: vec!["R1".to_string()],
                },
            ],
        }),
        join: Some(::adl::adl::PatternJoinSpec {
            step: "J".to_string(),
        }),
    };

    let compiled = ::adl::execution_plan::compile_pattern(&pattern).expect("compile pattern");

    let mut providers = HashMap::new();
    providers.insert(
        "local".to_string(),
        ::adl::adl::ProviderSpec {
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
        ::adl::adl::AgentSpec {
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
            ::adl::adl::TaskSpec {
                id: None,
                agent_ref: None,
                inputs: vec![],
                tool_allowlist: vec![],
                description: None,
                prompt: ::adl::adl::PromptSpec {
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

    let steps: Vec<::adl::resolve::ResolvedStep> = compiled
        .compiled_steps
        .iter()
        .map(|step| ::adl::resolve::ResolvedStep {
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

    let doc = ::adl::adl::AdlDoc {
        version: "0.5".to_string(),
        providers,
        tools: HashMap::new(),
        agents,
        tasks,
        workflows: HashMap::new(),
        patterns: vec![pattern],
        signature: None,
        run: ::adl::adl::RunSpec {
            id: None,
            name: Some("compiled-pattern-run".to_string()),
            created_at: None,
            defaults: ::adl::adl::RunDefaults::default(),
            workflow_ref: None,
            workflow: None,
            pattern_ref: Some("p_fork".to_string()),
            inputs: HashMap::new(),
            placement: None,
            remote: None,
            delegation_policy: None,
        },
    };

    let resolved = ::adl::resolve::AdlResolved {
        run_id: "compiled-pattern-run".to_string(),
        workflow_id: "pattern:p_fork".to_string(),
        steps,
        execution_plan: compiled.execution_plan,
        doc,
    };

    let mut tr = ::adl::trace::Trace::new("compiled-pattern-run", "pattern:p_fork", "0.5");
    let out_dir = base.join("out");
    fs::create_dir_all(&out_dir).unwrap();

    let result =
        ::adl::execute::execute_sequential(&resolved, &mut tr, false, false, &base, &out_dir)
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
