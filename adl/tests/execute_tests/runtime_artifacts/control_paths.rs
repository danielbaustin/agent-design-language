use super::*;

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
            conversation: None,
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
