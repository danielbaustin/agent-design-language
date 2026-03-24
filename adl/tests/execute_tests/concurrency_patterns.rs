use super::*;

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
        (2.5..=7.5).contains(&elapsed),
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
    let fork_out = Path::new("out").join("fork");

    let _ = fs::remove_dir_all(&fork_out);
    let out1 = run_swarm(&["examples/v0-3-concurrency-fork-join.adl.yaml", "--run"]);
    assert!(
        out1.status.success(),
        "first run failed.\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out1.stdout),
        String::from_utf8_lossy(&out1.stderr)
    );

    let _ = fs::remove_dir_all(&fork_out);
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
