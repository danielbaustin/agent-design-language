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
