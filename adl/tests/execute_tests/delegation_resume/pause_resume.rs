use super::*;

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

    let (run_json_path, _, _) = run_artifact_paths("hitl-pause-seq");
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
        base.join("out-paused").to_str().unwrap(),
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

    let resumed_final = fs::read_to_string(base.join("out-paused").join("s3.txt")).unwrap();
    let plain_final = fs::read_to_string(base.join("out-plain").join("s3.txt")).unwrap();
    assert_eq!(resumed_final, plain_final);
}

#[test]
fn run_pause_resume_cli_roundtrip_matches_uninterrupted_artifacts_byte_for_byte() {
    let base = tmp_dir("exec-pause-resume-cli-roundtrip");
    let endpoint = start_fixed_http_provider_server(6, "HTTP_FAKE_OK");

    let paused_case = base.join("paused-case");
    let plain_case = base.join("plain-case");
    fs::create_dir_all(&paused_case).unwrap();
    fs::create_dir_all(&plain_case).unwrap();

    let paused_yaml = r#"
version: "0.3"
providers:
  local:
    type: "http"
    config:
      endpoint: "__ENDPOINT__/complete"
agents:
  a:
    provider: "local"
    model: "unused-for-http-provider"
tasks:
  t:
    prompt:
      user: "step {{n}}"
run:
  name: "hitl-roundtrip-cli"
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
"#
    .replace("__ENDPOINT__", &endpoint);
    let plain_yaml = r#"
version: "0.3"
providers:
  local:
    type: "http"
    config:
      endpoint: "__ENDPOINT__/complete"
agents:
  a:
    provider: "local"
    model: "unused-for-http-provider"
tasks:
  t:
    prompt:
      user: "step {{n}}"
run:
  name: "hitl-roundtrip-plain"
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
      - id: "s3"
        agent: "a"
        task: "t"
        save_as: "s3"
        write_to: "s3.txt"
        inputs: { n: "3" }
"#
    .replace("__ENDPOINT__", &endpoint);

    let paused_path = paused_case.join("roundtrip-paused.yaml");
    let plain_path = plain_case.join("roundtrip-plain.yaml");
    fs::write(&paused_path, paused_yaml).unwrap();
    fs::write(&plain_path, plain_yaml).unwrap();

    let paused = run_swarm_in_dir(&paused_case, &[paused_path.to_str().unwrap(), "--run"]);
    assert!(
        paused.status.success(),
        "paused run should succeed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&paused.stdout),
        String::from_utf8_lossy(&paused.stderr)
    );
    let (run_json_path, _, _) = run_artifact_paths("hitl-roundtrip-cli");
    let run_json: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&run_json_path).unwrap()).unwrap();
    assert_eq!(run_json["status"], "paused");

    let resumed = run_swarm_in_dir(
        &paused_case,
        &[
            "resume",
            "hitl-roundtrip-cli",
            "--adl",
            paused_path.to_str().unwrap(),
        ],
    );
    assert!(
        resumed.status.success(),
        "resume CLI should succeed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&resumed.stdout),
        String::from_utf8_lossy(&resumed.stderr)
    );

    let plain = run_swarm_in_dir(&plain_case, &[plain_path.to_str().unwrap(), "--run"]);
    assert!(
        plain.status.success(),
        "plain run should succeed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&plain.stdout),
        String::from_utf8_lossy(&plain.stderr)
    );

    for rel in ["s1.txt", "s2.txt", "s3.txt"] {
        let paused_bytes = fs::read(paused_case.join("out").join(rel)).unwrap_or_else(|e| {
            panic!("missing paused artifact {rel}: {e}");
        });
        let plain_bytes = fs::read(plain_case.join("out").join(rel)).unwrap_or_else(|e| {
            panic!("missing plain artifact {rel}: {e}");
        });
        assert_eq!(paused_bytes, plain_bytes, "artifact bytes differ for {rel}");
    }
}

#[test]
fn resume_cli_rejects_missing_pause_state_for_unknown_run_id() {
    let resumed = run_swarm(&["resume", "run-id-does-not-exist"]);
    assert!(
        !resumed.status.success(),
        "resume CLI must fail for missing pause-state"
    );
    let stderr = String::from_utf8_lossy(&resumed.stderr);
    assert!(
        stderr.contains("pause state not found"),
        "stderr was:\n{stderr}"
    );
    assert!(
        stderr.contains("run-id-does-not-exist"),
        "stderr was:\n{stderr}"
    );
}

#[test]
fn resume_skips_verified_completed_steps_and_preserves_run_status() {
    let base = tmp_dir("exec-resume-skip-verified");
    let endpoint = start_fixed_http_provider_server(8, "resume-ok");
    let paused_case = base.join("paused");
    fs::create_dir_all(&paused_case).unwrap();

    let paused_yaml = r#"
version: "0.5"
providers:
  http:
    type: "http"
    config:
      endpoint: "__ENDPOINT__"
agents:
  a:
    provider: "http"
    model: "unused-for-http-provider"
tasks:
  t:
    prompt:
      user: "step {{n}}"
run:
  name: "resume-skip-verified"
  workflow:
    kind: "sequential"
    steps:
      - id: "s1"
        agent: "a"
        task: "t"
        save_as: "s1"
        write_to: "s1.txt"
        inputs: { n: "1" }
        guards:
          - type: pause
            config:
              reason: "after step 1"
      - id: "s2"
        agent: "a"
        task: "t"
        save_as: "s2"
        write_to: "s2.txt"
        inputs: { n: "2" }
"#
    .replace("__ENDPOINT__", &endpoint);
    let paused_path = paused_case.join("resume-skip.yaml");
    fs::write(&paused_path, paused_yaml).unwrap();

    let paused = run_swarm_in_dir(&paused_case, &[paused_path.to_str().unwrap(), "--run"]);
    assert!(paused.status.success(), "paused run should succeed");

    let resumed = run_swarm_in_dir(
        &paused_case,
        &[
            "resume",
            "resume-skip-verified",
            "--adl",
            paused_path.to_str().unwrap(),
        ],
    );
    assert!(resumed.status.success(), "resume should succeed");
    let resumed_stderr = String::from_utf8_lossy(&resumed.stderr);
    assert!(
        resumed_stderr.contains("RESUME step=s1 action=skip reason=completed_artifact_verified"),
        "stderr was:\n{resumed_stderr}"
    );
    assert!(
        !resumed_stderr.contains("STEP start (+0ms) s1"),
        "resumed run must not rerun verified step s1:\n{resumed_stderr}"
    );

    let run_status_path = repo_runs_dir()
        .join("resume-skip-verified")
        .join("run_status.json");
    let run_status: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&run_status_path).unwrap()).unwrap();
    assert_eq!(run_status["overall_status"], "succeeded");
    assert_eq!(
        run_status["completed_steps"],
        serde_json::json!(["s1", "s2"])
    );
    assert_eq!(run_status["attempt_counts_by_step"]["s1"], 0);
    assert_eq!(run_status["attempt_counts_by_step"]["s2"], 1);
}

#[test]
fn resume_reruns_completed_step_when_expected_artifact_is_missing() {
    let base = tmp_dir("exec-resume-rerun-missing");
    let endpoint = start_fixed_http_provider_server(8, "resume-rerun");
    let paused_case = base.join("paused");
    fs::create_dir_all(&paused_case).unwrap();

    let paused_yaml = r#"
version: "0.5"
providers:
  http:
    type: "http"
    config:
      endpoint: "__ENDPOINT__"
agents:
  a:
    provider: "http"
    model: "unused-for-http-provider"
tasks:
  t:
    prompt:
      user: "step {{n}}"
run:
  name: "resume-rerun-missing"
  workflow:
    kind: "sequential"
    steps:
      - id: "s1"
        agent: "a"
        task: "t"
        save_as: "s1"
        write_to: "s1.txt"
        inputs: { n: "1" }
        guards:
          - type: pause
            config:
              reason: "after step 1"
      - id: "s2"
        agent: "a"
        task: "t"
        save_as: "s2"
        write_to: "s2.txt"
        inputs: { n: "2" }
"#
    .replace("__ENDPOINT__", &endpoint);
    let paused_path = paused_case.join("resume-rerun.yaml");
    fs::write(&paused_path, paused_yaml).unwrap();

    let paused = run_swarm_in_dir(&paused_case, &[paused_path.to_str().unwrap(), "--run"]);
    assert!(paused.status.success(), "paused run should succeed");
    fs::remove_file(paused_case.join("out").join("s1.txt")).unwrap();

    let resumed = run_swarm_in_dir(
        &paused_case,
        &[
            "resume",
            "resume-rerun-missing",
            "--adl",
            paused_path.to_str().unwrap(),
        ],
    );
    assert!(resumed.status.success(), "resume should succeed");
    let resumed_stderr = String::from_utf8_lossy(&resumed.stderr);
    assert!(
        resumed_stderr.contains("RESUME step=s1 action=rerun reason=missing_expected_artifact"),
        "stderr was:\n{resumed_stderr}"
    );
    assert!(
        resumed_stderr.contains("s1 provider=http"),
        "missing artifact must force rerun of s1:\n{resumed_stderr}"
    );
}

#[test]
fn resume_with_steering_patch_updates_saved_state_and_records_history() {
    let base = tmp_dir("exec-resume-steer");
    let _bin = write_mock_ollama(&base, MockOllamaBehavior::EchoPrompt);
    let new_path = prepend_path(&base);
    let _path_guard = EnvVarGuard::set("PATH", new_path);
    let paused_case = base.join("paused");
    fs::create_dir_all(&paused_case).unwrap();

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
  t_fixed:
    prompt:
      user: "fixed"
  t_topic:
    prompt:
      user: "topic={{topic}}"
run:
  name: "resume-steer"
  workflow:
    kind: "sequential"
    steps:
      - id: "s1"
        agent: "a"
        task: "t_fixed"
        save_as: "s1"
        write_to: "s1.txt"
        guards:
          - type: pause
            config:
              reason: "await steering"
      - id: "s2"
        agent: "a"
        task: "t_topic"
        save_as: "s2"
        write_to: "s2.txt"
        inputs:
          topic: "@state:inputs.topic"
"#;
    let yaml_path = paused_case.join("resume-steer.yaml");
    fs::write(&yaml_path, yaml).unwrap();

    let paused = run_swarm_in_dir(&paused_case, &[yaml_path.to_str().unwrap(), "--run"]);
    assert!(paused.status.success(), "paused run should succeed");

    let steer_path = paused_case.join("steer.json");
    fs::write(
        &steer_path,
        serde_json::to_vec_pretty(&serde_json::json!({
            "schema_version": "steering_patch.v1",
            "apply_at": "resume_boundary",
            "reason": "inject topic",
            "set_state": { "inputs.topic": "steered-topic" },
            "remove_state": []
        }))
        .unwrap(),
    )
    .unwrap();

    let resumed = run_swarm_in_dir(
        &paused_case,
        &[
            "resume",
            "resume-steer",
            "--adl",
            yaml_path.to_str().unwrap(),
            "--steer",
            steer_path.to_str().unwrap(),
        ],
    );
    assert!(resumed.status.success(), "resume with steer should succeed");
    let resumed_stderr = String::from_utf8_lossy(&resumed.stderr);
    assert!(
        resumed_stderr.contains("STEER apply sequence=1"),
        "stderr was:\n{resumed_stderr}"
    );
    assert!(
        resumed_stderr.contains("RESUME step=s1 action=skip"),
        "stderr was:\n{resumed_stderr}"
    );

    let resumed_out = fs::read_to_string(paused_case.join("out").join("s2.txt")).unwrap();
    assert!(
        resumed_out.contains("steered-topic"),
        "expected steered output, got:\n{resumed_out}"
    );

    let run_json: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(repo_runs_dir().join("resume-steer").join("run.json")).unwrap(),
    )
    .unwrap();
    assert_eq!(run_json["steering_history"][0]["sequence"], 1);
    assert_eq!(
        run_json["steering_history"][0]["apply_at"],
        serde_json::json!("resume_boundary")
    );
    assert_eq!(
        run_json["steering_history"][0]["set_state_keys"],
        serde_json::json!(["inputs.topic"])
    );
}

#[test]
fn run_resume_with_identical_steering_patch_is_deterministic() {
    let base = tmp_dir("exec-resume-steer-deterministic");
    let _bin = write_mock_ollama(&base, MockOllamaBehavior::EchoPrompt);
    let new_path = prepend_path(&base);
    let _path_guard = EnvVarGuard::set("PATH", new_path);
    let paused_case = base.join("paused");
    fs::create_dir_all(&paused_case).unwrap();

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
  t_fixed:
    prompt:
      user: "fixed"
  t_topic:
    prompt:
      user: "topic={{topic}}"
run:
  name: "resume-steer-deterministic"
  workflow:
    kind: "sequential"
    steps:
      - id: "s1"
        agent: "a"
        task: "t_fixed"
        save_as: "s1"
        write_to: "s1.txt"
        guards:
          - type: pause
      - id: "s2"
        agent: "a"
        task: "t_topic"
        save_as: "s2"
        write_to: "s2.txt"
        inputs:
          topic: "@state:inputs.topic"
"#;
    let yaml_path = paused_case.join("resume-steer-deterministic.yaml");
    fs::write(&yaml_path, yaml).unwrap();

    let paused = run_swarm_in_dir(&paused_case, &[yaml_path.to_str().unwrap(), "--run"]);
    assert!(paused.status.success(), "paused run should succeed");
    let (run_json_path, _, _) = run_artifact_paths("resume-steer-deterministic");
    let resume_1 = base.join("resume-1.run.json");
    let resume_2 = base.join("resume-2.run.json");
    fs::copy(&run_json_path, &resume_1).unwrap();
    fs::copy(&run_json_path, &resume_2).unwrap();

    let steer_path = base.join("steer.json");
    fs::write(
        &steer_path,
        serde_json::to_vec_pretty(&serde_json::json!({
            "schema_version": "steering_patch.v1",
            "apply_at": "resume_boundary",
            "reason": "inject topic",
            "set_state": { "inputs.topic": "deterministic-topic" },
            "remove_state": []
        }))
        .unwrap(),
    )
    .unwrap();

    let out_1 = base.join("out-1");
    let out_2 = base.join("out-2");
    fs::create_dir_all(&out_1).unwrap();
    fs::create_dir_all(&out_2).unwrap();
    fs::copy(paused_case.join("out").join("s1.txt"), out_1.join("s1.txt")).unwrap();
    fs::copy(paused_case.join("out").join("s1.txt"), out_2.join("s1.txt")).unwrap();
    let resumed_1 = run_swarm_in_dir(
        &paused_case,
        &[
            yaml_path.to_str().unwrap(),
            "--run",
            "--resume",
            resume_1.to_str().unwrap(),
            "--steer",
            steer_path.to_str().unwrap(),
            "--out",
            out_1.to_str().unwrap(),
        ],
    );
    let resumed_2 = run_swarm_in_dir(
        &paused_case,
        &[
            yaml_path.to_str().unwrap(),
            "--run",
            "--resume",
            resume_2.to_str().unwrap(),
            "--steer",
            steer_path.to_str().unwrap(),
            "--out",
            out_2.to_str().unwrap(),
        ],
    );
    assert!(resumed_1.status.success(), "resume #1 should succeed");
    assert!(resumed_2.status.success(), "resume #2 should succeed");

    let stderr_1 = String::from_utf8_lossy(&resumed_1.stderr);
    let stderr_2 = String::from_utf8_lossy(&resumed_2.stderr);
    let output_1 = fs::read_to_string(out_1.join("s2.txt")).unwrap();
    let output_2 = fs::read_to_string(out_2.join("s2.txt")).unwrap();
    assert_eq!(output_1, output_2, "resumed output should be deterministic");
    assert!(
        output_1.contains("deterministic-topic"),
        "output_1:\n{output_1}"
    );
    assert!(
        stderr_1.contains("STEER apply sequence=1") && stderr_2.contains("STEER apply sequence=1"),
        "stderr_1:\n{stderr_1}\n---\nstderr_2:\n{stderr_2}"
    );
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
    let (run_json_path, _, _) = run_artifact_paths("hitl-pause-concurrent");
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
    let (run_json_path, _, _) = run_artifact_paths("hitl-resume-mismatch");

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
    let (run_json_path, _, _) = run_artifact_paths("hitl-invalid-state");

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

#[test]
fn resume_subcommand_resumes_from_pause_state_successfully() {
    let base = tmp_dir("exec-resume-subcommand-ok");
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
  name: "hitl-resume-subcommand"
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
      - id: "s3"
        agent: "a"
        task: "t"
        save_as: "s3"
        write_to: "s3.txt"
        inputs: { n: "3" }
"#;
    let paused_path = base.join("paused-resume-subcommand.yaml");
    fs::write(&paused_path, paused_yaml).unwrap();

    let paused = run_swarm(&[paused_path.to_str().unwrap(), "--run"]);
    assert!(
        paused.status.success(),
        "paused run should succeed.\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&paused.stdout),
        String::from_utf8_lossy(&paused.stderr)
    );

    let resume = run_swarm(&[
        "resume",
        "hitl-resume-subcommand",
        "--adl",
        paused_path.to_str().unwrap(),
    ]);
    assert!(
        resume.status.success(),
        "resume subcommand should succeed.\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&resume.stdout),
        String::from_utf8_lossy(&resume.stderr)
    );
    let (run_json_path, _, _) = run_artifact_paths("hitl-resume-subcommand");
    let run_json: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(run_json_path).unwrap()).unwrap();
    assert_eq!(run_json["status"], "success");
}

#[test]
fn resume_subcommand_rejects_pause_state_plan_hash_mismatch() {
    let base = tmp_dir("exec-resume-subcommand-hash-mismatch");
    let _bin = write_mock_ollama(&base, MockOllamaBehavior::EchoPrompt);
    let new_path = prepend_path(&base);
    let _path_guard = EnvVarGuard::set("PATH", new_path);

    let paused_yaml = r#"
version: "0.3"
providers: { local: { type: "ollama" } }
agents: { a: { provider: "local", model: "phi4-mini" } }
tasks: { t: { prompt: { user: "step {{n}}" } } }
run:
  name: "hitl-resume-subcommand-bad-hash"
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
    let paused_path = base.join("paused-resume-subcommand-bad-hash.yaml");
    fs::write(&paused_path, paused_yaml).unwrap();

    let paused = run_swarm(&[paused_path.to_str().unwrap(), "--run"]);
    assert!(paused.status.success(), "paused run should succeed");

    let pause_path = pause_state_path("hitl-resume-subcommand-bad-hash");
    let mut pause_json: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&pause_path).unwrap()).unwrap();
    pause_json["execution_plan_hash"] = serde_json::Value::String("deadbeef".to_string());
    fs::write(&pause_path, serde_json::to_vec_pretty(&pause_json).unwrap()).unwrap();

    let resumed = run_swarm(&[
        "resume",
        "hitl-resume-subcommand-bad-hash",
        "--adl",
        paused_path.to_str().unwrap(),
    ]);
    assert!(
        !resumed.status.success(),
        "resume subcommand should reject hash mismatch"
    );
    let stderr = String::from_utf8_lossy(&resumed.stderr);
    assert!(
        stderr.contains("execution plan hash mismatch"),
        "stderr was:\n{stderr}"
    );
}

#[test]
fn resume_subcommand_ignores_tampered_pause_artifact_adl_path_when_explicit_path_is_supplied() {
    let base = tmp_dir("exec-resume-subcommand-tampered-adl-path");
    let _bin = write_mock_ollama(&base, MockOllamaBehavior::EchoPrompt);
    let new_path = prepend_path(&base);
    let _path_guard = EnvVarGuard::set("PATH", new_path);

    let paused_yaml = r#"
version: "0.3"
providers: { local: { type: "ollama" } }
agents: { a: { provider: "local", model: "phi4-mini" } }
tasks: { t: { prompt: { user: "step {{n}}" } } }
run:
  name: "hitl-resume-subcommand-tampered-adl-path"
  workflow:
    kind: "sequential"
    steps:
      - id: "s1"
        agent: "a"
        task: "t"
        save_as: "s1"
        write_to: "s1.txt"
        inputs: { n: "1" }
        guards: [ { type: pause } ]
      - id: "s2"
        agent: "a"
        task: "t"
        save_as: "s2"
        write_to: "s2.txt"
        inputs: { n: "2" }
"#;
    let paused_path = base.join("paused-resume-subcommand-tampered-adl-path.yaml");
    fs::write(&paused_path, paused_yaml).unwrap();

    let paused = run_swarm(&[paused_path.to_str().unwrap(), "--run"]);
    assert!(paused.status.success(), "paused run should succeed");

    let pause_path = pause_state_path("hitl-resume-subcommand-tampered-adl-path");
    let mut pause_json: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&pause_path).unwrap()).unwrap();
    pause_json["adl_path"] =
        serde_json::Value::String(base.join("tampered-other.yaml").display().to_string());
    fs::write(&pause_path, serde_json::to_vec_pretty(&pause_json).unwrap()).unwrap();

    let resumed = run_swarm(&[
        "resume",
        "hitl-resume-subcommand-tampered-adl-path",
        "--adl",
        paused_path.to_str().unwrap(),
    ]);
    assert!(
        resumed.status.success(),
        "resume should ignore tampered pause_state adl_path when explicit --adl is supplied.\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&resumed.stdout),
        String::from_utf8_lossy(&resumed.stderr)
    );
}
