use super::*;

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
    assert_eq!(
        summary_json["links"]["trace_json"], "logs/trace_v1.json",
        "trace_json should point at the canonical runtime trace artifact"
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

    let _ = fs::remove_dir_all(&run_dir);
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
