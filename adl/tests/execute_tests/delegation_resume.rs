use super::*;

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
    assert!(
        stdout.contains("LifecyclePhaseEntered phase=init"),
        "stdout was:\n{stdout}"
    );
    assert!(
        stdout.contains("LifecyclePhaseEntered phase=execute"),
        "stdout was:\n{stdout}"
    );
    assert!(
        stdout.contains("ExecutionBoundaryCrossed boundary=runtime_init state=fresh_start"),
        "stdout was:\n{stdout}"
    );
    assert!(
        stdout.contains("ExecutionBoundaryCrossed boundary=workflow_call state=entered"),
        "stdout was:\n{stdout}"
    );
    assert!(
        stdout.contains("ExecutionBoundaryCrossed boundary=workflow_call state=success"),
        "stdout was:\n{stdout}"
    );
    assert!(
        stdout.contains("ExecutionBoundaryCrossed boundary=run_completion state=success"),
        "stdout was:\n{stdout}"
    );
    assert!(
        stdout.contains("LifecyclePhaseEntered phase=complete"),
        "stdout was:\n{stdout}"
    );
    assert!(
        stdout.contains("LifecyclePhaseEntered phase=teardown"),
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
    let stdout1 = String::from_utf8_lossy(&out1.stdout);
    assert!(
        stdout1.contains("SchedulerPolicy max_concurrency=2 source=run_default"),
        "stdout was:\n{stdout1}"
    );
    assert!(
        stdout1.contains("SCHEDULER POLICY: max_concurrency=2 source=run_default"),
        "stdout was:\n{stdout1}"
    );

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
    let stdout2 = String::from_utf8_lossy(&out2.stdout);
    assert!(
        stdout2.contains("SchedulerPolicy max_concurrency=2 source=run_default"),
        "stdout was:\n{stdout2}"
    );
    assert!(
        stdout2.contains("SCHEDULER POLICY: max_concurrency=2 source=run_default"),
        "stdout was:\n{stdout2}"
    );
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
fn trace_emits_deterministic_delegation_lifecycle_sequence() {
    let base = tmp_dir("exec-delegation-lifecycle-trace");
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
          tags: ["safety"]
"#;
    let tmp_yaml = base.join("delegation-lifecycle-trace.yaml");
    fs::write(&tmp_yaml, yaml).unwrap();

    let out1 = run_swarm(&[tmp_yaml.to_str().unwrap(), "--run", "--trace"]);
    let out2 = run_swarm(&[tmp_yaml.to_str().unwrap(), "--run", "--trace"]);
    assert!(
        out1.status.success(),
        "stdout:
{}
stderr:
{}",
        String::from_utf8_lossy(&out1.stdout),
        String::from_utf8_lossy(&out1.stderr)
    );
    assert!(
        out2.status.success(),
        "stdout:
{}
stderr:
{}",
        String::from_utf8_lossy(&out2.stdout),
        String::from_utf8_lossy(&out2.stderr)
    );

    let normalize = |stdout: &str| -> Vec<String> {
        stdout
            .lines()
            .filter_map(|line| line.split_once(") ").map(|(_, rest)| rest.to_string()))
            .filter(|line| line.starts_with("Delegation"))
            .collect()
    };

    let lifecycle1 = normalize(&String::from_utf8_lossy(&out1.stdout));
    let lifecycle2 = normalize(&String::from_utf8_lossy(&out2.stdout));

    assert_eq!(
        lifecycle1,
        vec![
            "DelegationRequested delegation_id=del-1 step=s1 action=provider_call target=local"
                .to_string(),
            "DelegationPolicyEvaluated delegation_id=del-1 step=s1 action=provider_call target=local decision=allowed"
                .to_string(),
            "DelegationDispatched delegation_id=del-1 step=s1 action=provider_call target=local"
                .to_string(),
            "DelegationResultReceived delegation_id=del-1 step=s1 success=true bytes=12"
                .to_string(),
            "DelegationCompleted delegation_id=del-1 step=s1 outcome=success".to_string(),
        ]
    );
    assert_eq!(
        lifecycle1, lifecycle2,
        "delegation lifecycle should be byte-stable across identical runs"
    );

    let stdout = String::from_utf8_lossy(&out1.stdout);
    assert!(
        !stdout.contains(base.to_str().unwrap()),
        "trace should not leak absolute temp paths:
{stdout}"
    );
}

#[test]
fn concurrent_delegation_ids_are_deterministic_across_repeated_runs() {
    let base = tmp_dir("exec-concurrent-delegation-ids");
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
  defaults:
    max_concurrency: 2
  workflow:
    kind: "concurrent"
    steps:
      - id: "s2"
        agent: "a"
        task: "t"
        inputs: { n: "2" }
        delegation:
          role: "reviewer"
      - id: "s1"
        agent: "a"
        task: "t"
        inputs: { n: "1" }
        delegation:
          role: "reviewer"
"#;
    let tmp_yaml = base.join("concurrent-delegation-ids.yaml");
    fs::write(&tmp_yaml, yaml).unwrap();

    let parse_ids = |stdout: &str| -> Vec<(String, String)> {
        stdout
            .lines()
            .filter_map(|line| line.split_once(") ").map(|(_, rest)| rest))
            .filter(|line| line.starts_with("DelegationRequested "))
            .map(|line| {
                let mut step_id = String::new();
                let mut delegation_id = String::new();
                for part in line.split_whitespace() {
                    if let Some(v) = part.strip_prefix("step=") {
                        step_id = v.to_string();
                    }
                    if let Some(v) = part.strip_prefix("delegation_id=") {
                        delegation_id = v.to_string();
                    }
                }
                (step_id, delegation_id)
            })
            .collect()
    };

    let out1 = run_swarm(&[tmp_yaml.to_str().unwrap(), "--run", "--trace"]);
    let out2 = run_swarm(&[tmp_yaml.to_str().unwrap(), "--run", "--trace"]);
    assert!(
        out1.status.success(),
        "stdout:
{}
stderr:
{}",
        String::from_utf8_lossy(&out1.stdout),
        String::from_utf8_lossy(&out1.stderr)
    );
    assert!(
        out2.status.success(),
        "stdout:
{}
stderr:
{}",
        String::from_utf8_lossy(&out2.stdout),
        String::from_utf8_lossy(&out2.stderr)
    );

    let ids1 = parse_ids(&String::from_utf8_lossy(&out1.stdout));
    let ids2 = parse_ids(&String::from_utf8_lossy(&out2.stdout));
    assert_eq!(
        ids1,
        vec![
            ("s1".to_string(), "del-1".to_string()),
            ("s2".to_string(), "del-2".to_string())
        ]
    );
    assert_eq!(
        ids1, ids2,
        "delegation ids should remain deterministic across repeated concurrent runs"
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
fn run_delegation_policy_deny_has_stable_error_code_and_trace() {
    let base = tmp_dir("exec-delegation-policy-deny");

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
        inputs: { n: "1" }
        delegation:
          role: "reviewer"
          tags: ["safety"]
"#;

    let tmp_yaml = base.join("delegation-policy-deny.yaml");
    fs::write(&tmp_yaml, yaml).unwrap();

    let out = run_swarm(&[tmp_yaml.to_str().unwrap(), "--run", "--trace"]);
    assert!(
        !out.status.success(),
        "expected denial failure.
stdout:
{}
stderr:
{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );

    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_eq!(
        delegation_error_code(&stderr),
        Some(::adl::execute::DELEGATION_POLICY_DENY_CODE),
        "stderr was:
{stderr}"
    );
    assert!(
        stderr.contains("action 'provider_call' target 'local' denied"),
        "stderr was:
{stderr}"
    );
    assert!(
        stderr.contains("rule_id=deny-local-provider"),
        "stderr was:
{stderr}"
    );

    let stdout = String::from_utf8_lossy(&out.stdout);
    let lifecycle: Vec<&str> = stdout
        .lines()
        .filter_map(|line| line.split_once(") ").map(|(_, rest)| rest))
        .filter(|line| line.starts_with("Delegation"))
        .collect();
    assert_eq!(
        lifecycle,
        vec![
            "DelegationPolicyEvaluated delegation_id=del-1 step=s1 action=provider_call target=local decision=denied rule_id=deny-local-provider",
            "DelegationDenied delegation_id=del-1 step=s1 action=provider_call target=local rule_id=deny-local-provider",
        ],
        "stdout was:
{stdout}"
    );
    assert!(
        !stdout.contains("DelegationDispatched"),
        "denied policy path must not dispatch. stdout was:
{stdout}"
    );
    assert!(
        !stdout.contains("StepStarted step=s1"),
        "policy denial should happen before StepStarted. stdout was:
{stdout}"
    );
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

    let normalize = |s: &str| -> String {
        s.lines()
            .filter(|line| !line.starts_with("SCHEDULER POLICY:"))
            .collect::<Vec<_>>()
            .join("\n")
    };

    assert_eq!(
        normalize(&String::from_utf8_lossy(&out_seq.stdout)),
        normalize(&String::from_utf8_lossy(&out_conc.stdout)),
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
    let stdout1 = String::from_utf8_lossy(&out1.stdout);
    assert!(
        stdout1.contains("SchedulerPolicy max_concurrency=2 source=workflow_override"),
        "stdout was:\n{stdout1}"
    );
    assert!(
        stdout1.contains("SCHEDULER POLICY: max_concurrency=2 source=workflow_override"),
        "stdout was:\n{stdout1}"
    );

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

#[test]
fn run_overlay_apply_changes_behavior_deterministically() {
    let base = tmp_dir("exec-overlay-retry");
    let _bin = write_mock_ollama(&base, MockOllamaBehavior::FailOnce);
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
      user: "fail_once_token"
run:
  name: "overlay-retry-run"
  workflow:
    kind: "sequential"
    steps:
      - id: "s1"
        agent: "a"
        task: "t"
        save_as: "s1"
        write_to: "s1.txt"
"#;
    let yaml_path = base.join("overlay-retry.yaml");
    fs::write(&yaml_path, yaml).unwrap();

    let overlay = r#"
{
  "overlay_version": 1,
  "created_by": "test",
  "created_from": { "artifact_model_version": 1 },
  "changes": [
    {
      "id": "retry-all",
      "path": "run.workflow.steps.*.retry.max_attempts",
      "op": "set",
      "value": 2,
      "rationale": "allow one retry deterministically"
    }
  ]
}
"#;
    let overlay_path = base.join("overlay.json");
    fs::write(&overlay_path, overlay).unwrap();

    let without_overlay = run_swarm(&[yaml_path.to_str().unwrap(), "--run"]);
    assert!(
        !without_overlay.status.success(),
        "without overlay should fail due to fail-once behavior"
    );

    let with_overlay_1 = run_swarm(&[
        yaml_path.to_str().unwrap(),
        "--overlay",
        overlay_path.to_str().unwrap(),
        "--run",
    ]);
    assert!(
        with_overlay_1.status.success(),
        "overlay run 1 should succeed\nstderr:\n{}",
        String::from_utf8_lossy(&with_overlay_1.stderr)
    );

    let with_overlay_2 = run_swarm(&[
        yaml_path.to_str().unwrap(),
        "--overlay",
        overlay_path.to_str().unwrap(),
        "--run",
    ]);
    assert!(
        with_overlay_2.status.success(),
        "overlay run 2 should succeed"
    );

    let run_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join(".adl")
        .join("runs")
        .join("overlay-retry-run");
    let run_json: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(run_dir.join("run.json")).unwrap()).unwrap();
    assert_eq!(
        run_json["status"], "success",
        "overlay should flip run to success"
    );

    let out_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("out")
        .join("s1.txt");
    assert!(out_path.is_file(), "overlay run should emit out/s1.txt");
    let _ = fs::remove_file(out_path);
}

#[test]
fn run_overlay_apply_writes_stable_audit_artifacts() {
    let base = tmp_dir("exec-overlay-audit");
    let _bin = write_mock_ollama(&base, MockOllamaBehavior::Success);
    let new_path = prepend_path(&base);
    let _path_guard = EnvVarGuard::set("PATH", new_path);

    let yaml = r#"
version: "0.3"
providers: { local: { type: "ollama" } }
agents: { a: { provider: "local", model: "phi4-mini" } }
tasks: { t: { prompt: { user: "hello" } } }
run:
  name: "overlay-audit-run"
  workflow:
    kind: "sequential"
    steps:
      - id: "s1"
        agent: "a"
        task: "t"
"#;
    let yaml_path = base.join("overlay-audit.yaml");
    fs::write(&yaml_path, yaml).unwrap();

    let overlay = r#"
{
  "overlay_version": 1,
  "created_by": "test",
  "created_from": { "artifact_model_version": 1 },
  "changes": [
    {
      "id": "retry-all",
      "path": "run.workflow.steps.*.retry.max_attempts",
      "op": "set",
      "value": 2,
      "rationale": "record hash + applied fields"
    }
  ]
}
"#;
    let overlay_path = base.join("overlay.json");
    fs::write(&overlay_path, overlay).unwrap();

    let first = run_swarm(&[
        yaml_path.to_str().unwrap(),
        "--overlay",
        overlay_path.to_str().unwrap(),
        "--run",
    ]);
    assert!(first.status.success(), "first overlay run should succeed");

    let second = run_swarm(&[
        yaml_path.to_str().unwrap(),
        "--overlay",
        overlay_path.to_str().unwrap(),
        "--run",
    ]);
    assert!(second.status.success(), "second overlay run should succeed");

    let run_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join(".adl")
        .join("runs")
        .join("overlay-audit-run");
    let audit_path = run_dir
        .join("learning")
        .join("overlays")
        .join("applied_overlay.json");
    let source_path = run_dir
        .join("learning")
        .join("overlays")
        .join("source_overlay.json");

    assert!(audit_path.is_file(), "overlay audit file must exist");
    assert!(source_path.is_file(), "overlay source copy must exist");

    let audit_text = fs::read_to_string(audit_path).unwrap();
    let audit_json: serde_json::Value = serde_json::from_str(&audit_text).unwrap();
    assert!(audit_json["overlay_hash"].is_string());
    assert_eq!(audit_json["applied_change_ids"][0], "retry-all");
    assert_eq!(
        audit_json["applied_paths"][0],
        "run.workflow.steps.*.retry.max_attempts"
    );
}
