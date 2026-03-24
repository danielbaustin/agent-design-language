use super::validation::{validate_provider, validate_tool};
use super::*;
use super::loading::load_yaml_with_includes;
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn runspec_resolve_workflow_shapes() {
    let doc = AdlDoc {
        version: "0.5".to_string(),
        providers: HashMap::new(),
        tools: HashMap::new(),
        agents: HashMap::new(),
        tasks: HashMap::new(),
        workflows: HashMap::new(),
        patterns: vec![],
        signature: None,
        run: RunSpec {
            id: None,
            name: None,
            created_at: None,
            defaults: RunDefaults::default(),
            workflow_ref: None,
            workflow: None,
            pattern_ref: None,
            inputs: HashMap::new(),
            placement: None,
            remote: None,
            delegation_policy: None,
        },
    };

    let err = doc
        .run
        .resolve_workflow(&doc)
        .expect_err("missing workflow shape should fail");
    assert!(err
        .to_string()
        .contains("must define either workflow_ref or inline workflow"));
}

#[test]
fn resolve_workflow_ref_unknown_and_inline_success() {
    let mut doc = AdlDoc {
        version: "0.5".to_string(),
        providers: HashMap::new(),
        tools: HashMap::new(),
        agents: HashMap::new(),
        tasks: HashMap::new(),
        workflows: HashMap::new(),
        patterns: vec![],
        signature: None,
        run: RunSpec {
            id: None,
            name: None,
            created_at: None,
            defaults: RunDefaults::default(),
            workflow_ref: Some("missing".to_string()),
            workflow: None,
            pattern_ref: None,
            inputs: HashMap::new(),
            placement: None,
            remote: None,
            delegation_policy: None,
        },
    };
    let err = doc
        .run
        .resolve_workflow(&doc)
        .expect_err("unknown workflow ref");
    assert!(err.to_string().contains("references unknown workflow"));

    doc.run.workflow_ref = None;
    doc.run.workflow = Some(WorkflowSpec {
        id: Some("wf".to_string()),
        kind: WorkflowKind::Sequential,
        max_concurrency: None,
        steps: vec![],
    });
    let resolved = doc.run.resolve_workflow(&doc).expect("inline workflow");
    assert_eq!(resolved.id.as_deref(), Some("wf"));
}

#[test]
fn delegation_canonicalized_and_empty_detection() {
    let d = DelegationSpec {
        role: Some("review".to_string()),
        requires_verification: Some(false),
        escalation_target: None,
        tags: vec!["z".to_string(), "a".to_string(), "z".to_string()],
    };
    let canonical = d.canonicalized();
    assert_eq!(canonical.tags, vec!["a".to_string(), "z".to_string()]);
    assert_eq!(canonical.requires_verification, None);

    let empty = DelegationSpec {
        role: None,
        requires_verification: Some(false),
        escalation_target: None,
        tags: vec![],
    };
    assert!(empty.is_effectively_empty());
    assert!(!DelegationSpec {
        role: None,
        requires_verification: Some(true),
        escalation_target: None,
        tags: vec![],
    }
    .is_effectively_empty());
}

#[test]
fn run_placement_mode_resolves_legacy_values() {
    assert_eq!(
        RunPlacementSpec::Mode(PlacementMode::Remote).mode(),
        Some(PlacementMode::Remote)
    );
    let legacy_remote = RunPlacementSpec::Legacy(RunPlacementLegacySpec {
        provider: None,
        target: Some("REMOTE".to_string()),
    });
    assert_eq!(legacy_remote.mode(), Some(PlacementMode::Remote));
    let legacy_unknown = RunPlacementSpec::Legacy(RunPlacementLegacySpec {
        provider: None,
        target: Some("somewhere".to_string()),
    });
    assert_eq!(legacy_unknown.mode(), None);
}

#[test]
fn pattern_validate_exercises_linear_and_fork_join_errors() {
    let linear = PatternSpec {
        id: "p".to_string(),
        kind: PatternKind::Linear,
        steps: vec![],
        fork: None,
        join: None,
    };
    assert!(linear.validate().is_err());

    let fork_missing = PatternSpec {
        id: "p".to_string(),
        kind: PatternKind::ForkJoin,
        steps: vec![],
        fork: None,
        join: Some(PatternJoinSpec {
            step: "J".to_string(),
        }),
    };
    assert!(fork_missing.validate().is_err());

    let duplicate_branch = PatternSpec {
        id: "p".to_string(),
        kind: PatternKind::ForkJoin,
        steps: vec![],
        fork: Some(PatternForkSpec {
            branches: vec![
                PatternBranchSpec {
                    id: "b".to_string(),
                    steps: vec!["A".to_string()],
                },
                PatternBranchSpec {
                    id: "b".to_string(),
                    steps: vec!["B".to_string()],
                },
            ],
        }),
        join: Some(PatternJoinSpec {
            step: "J".to_string(),
        }),
    };
    let err = duplicate_branch
        .validate()
        .expect_err("duplicate branch id");
    assert!(err.to_string().contains("duplicate branch id"));
}

#[test]
fn load_from_file_include_merge_and_cycle_errors() {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time")
        .as_nanos();
    let base = std::env::temp_dir().join(format!("adl-load-test-{now}-{}", std::process::id()));
    std::fs::create_dir_all(&base).expect("create base");

    let fragment = base.join("fragment.yaml");
    let root = base.join("root.yaml");
    std::fs::write(
        &fragment,
        r#"
providers:
  p:
    type: "ollama"
agents:
  a:
    provider: "p"
    model: "m"
tasks:
  t:
    prompt:
      user: "u"
"#,
    )
    .expect("write fragment");
    std::fs::write(
        &root,
        r#"
version: "0.1"
include: ["fragment.yaml"]
run:
  workflow:
    steps:
      - agent: "a"
        task: "t"
"#,
    )
    .expect("write root");

    let doc = AdlDoc::load_from_file(root.to_str().expect("utf8")).expect("load merged doc");
    assert!(doc.providers.contains_key("p"));
    assert!(doc.tasks.contains_key("t"));

    let a = base.join("a.yaml");
    let b = base.join("b.yaml");
    std::fs::write(
        &a,
        r#"
version: "0.1"
include: ["b.yaml"]
run:
  workflow:
    steps: []
"#,
    )
    .expect("write a");
    std::fs::write(
        &b,
        r#"
include: ["a.yaml"]
"#,
    )
    .expect("write b");

    let err = AdlDoc::load_from_file(a.to_str().expect("utf8")).expect_err("cycle error");
    let msg = format!("{err:#}");
    assert!(
        msg.contains("include cycle detected"),
        "unexpected cycle error: {msg}"
    );
    let _ = std::fs::remove_dir_all(base);
}

#[test]
fn load_from_file_rejects_invalid_include_shapes_and_paths() {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time")
        .as_nanos();
    let base =
        std::env::temp_dir().join(format!("adl-load-invalid-{now}-{}", std::process::id()));
    std::fs::create_dir_all(&base).expect("create base");

    let root = base.join("root.yaml");

    std::fs::write(
        &root,
        r#"
version: "0.1"
include: "fragment.yaml"
run:
  workflow:
    steps: []
"#,
    )
    .expect("write invalid include shape");
    let err = load_yaml_with_includes(&root, &mut Vec::new()).expect_err("include must be a sequence");
    assert!(err.to_string().contains("include must be a YAML sequence"));

    std::fs::write(
        &root,
        r#"
version: "0.1"
include: [7]
run:
  workflow:
    steps: []
"#,
    )
    .expect("write non-string include");
    let err = load_yaml_with_includes(&root, &mut Vec::new())
        .expect_err("include entries must be strings");
    assert!(err.to_string().contains("include entries must be strings"));

    std::fs::write(
        &root,
        r#"
version: "0.1"
include: ["../escape.yaml"]
run:
  workflow:
    steps: []
"#,
    )
    .expect("write parent include");
    let err = load_yaml_with_includes(&root, &mut Vec::new())
        .expect_err("include path with parent dir should fail");
    assert!(err
        .to_string()
        .contains("include path must be relative and must not contain '..'"));

    let absolute_include = base.join("fragment.yaml");
    std::fs::write(
        &root,
        format!(
            r#"
version: "0.1"
include: ["{}"]
run:
  workflow:
    steps: []
"#,
            absolute_include.display()
        ),
    )
    .expect("write absolute include");
    let err = load_yaml_with_includes(&root, &mut Vec::new())
        .expect_err("absolute include path should fail");
    assert!(err
        .to_string()
        .contains("include path must be relative and must not contain '..'"));

    let _ = std::fs::remove_dir_all(base);
}

#[test]
fn load_from_file_rejects_invalid_include_merge_shapes() {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time")
        .as_nanos();
    let base = std::env::temp_dir().join(format!("adl-load-merge-{now}-{}", std::process::id()));
    std::fs::create_dir_all(&base).expect("create base");

    let root = base.join("root.yaml");
    let include = base.join("fragment.yaml");

    std::fs::write(
        &root,
        r#"
version: "0.1"
include: ["fragment.yaml"]
run:
  workflow:
    steps: []
"#,
    )
    .expect("write root");

    std::fs::write(&include, "[]\n").expect("write non-mapping include");
    let err = load_yaml_with_includes(&root, &mut Vec::new())
        .expect_err("included doc must be a mapping");
    assert!(err
        .to_string()
        .contains("top-level ADL document must be a mapping"));

    std::fs::write(
        &include,
        r#"
providers: []
"#,
    )
    .expect("write invalid providers shape");
    let err = load_yaml_with_includes(&root, &mut Vec::new())
        .expect_err("providers must be a mapping");
    assert!(err
        .to_string()
        .contains("top-level 'providers' must be a mapping"));

    std::fs::write(
        &include,
        r#"
patterns: {}
"#,
    )
    .expect("write invalid patterns shape");
    let err = load_yaml_with_includes(&root, &mut Vec::new())
        .expect_err("patterns must be a sequence");
    assert!(err
        .to_string()
        .contains("top-level 'patterns' must be a sequence"));

    std::fs::write(
        &include,
        r#"
providers:
  p:
    type: "ollama"
"#,
    )
    .expect("write provider include");
    std::fs::write(
        &root,
        r#"
version: "0.1"
include: ["fragment.yaml"]
providers:
  p:
    type: "ollama"
run:
  workflow:
    steps: []
"#,
    )
    .expect("write duplicate provider root");
    let err = load_yaml_with_includes(&root, &mut Vec::new())
        .expect_err("duplicate provider id should fail");
    assert!(err
        .to_string()
        .contains("duplicate providers id 'p' while processing includes"));

    std::fs::write(
        &include,
        r#"
version: "0.1"
"#,
    )
    .expect("write duplicate top-level include");
    std::fs::write(
        &root,
        r#"
version: "0.2"
include: ["fragment.yaml"]
run:
  workflow:
    steps: []
"#,
    )
    .expect("write duplicate top-level root");
    let err = load_yaml_with_includes(&root, &mut Vec::new())
        .expect_err("duplicate top-level key should fail");
    assert!(err
        .to_string()
        .contains("duplicate top-level key 'version' while processing includes"));

    let _ = std::fs::remove_dir_all(base);
}

#[test]
fn deserialize_temperature_number_formats() {
    let yaml = r#"
id: "a1"
provider: "p"
model: "m"
temperature: 0.7
"#;
    let agent: AgentSpec = serde_yaml::from_str(yaml).expect("parse agent");
    assert_eq!(agent.temperature.as_deref(), Some("0.7"));
}

#[test]
fn validate_provider_http_requires_endpoint() {
    let provider = ProviderSpec {
        id: None,
        profile: None,
        kind: "http".to_string(),
        base_url: None,
        default_model: None,
        config: HashMap::new(),
    };
    let err = validate_provider("p1", &provider).expect_err("http provider must require endpoint");
    assert!(err
        .to_string()
        .contains("requires base_url or config.endpoint"));
}

#[test]
fn validate_provider_profile_rejects_empty_value() {
    let provider = ProviderSpec {
        id: None,
        profile: Some("   ".to_string()),
        kind: "".to_string(),
        base_url: None,
        default_model: None,
        config: HashMap::new(),
    };
    let err = validate_provider("p1", &provider).expect_err("empty profile should fail");
    assert!(err.to_string().contains("profile must not be empty"));
}

#[test]
fn validate_tool_rejects_unsupported_kind() {
    let tool = ToolSpec {
        id: None,
        kind: "nope".to_string(),
        config: HashMap::new(),
    };
    let err = validate_tool("t1", &tool).expect_err("unsupported tool kind");
    assert!(err.to_string().contains("unsupported kind"));
}

#[test]
fn validate_rejects_explicit_id_mismatch_with_key() {
    let doc = AdlDoc {
        version: "0.5".to_string(),
        providers: HashMap::from([(
            "p1".to_string(),
            ProviderSpec {
                id: Some("wrong".to_string()),
                profile: None,
                kind: "ollama".to_string(),
                base_url: None,
                default_model: None,
                config: HashMap::new(),
            },
        )]),
        tools: HashMap::new(),
        agents: HashMap::new(),
        tasks: HashMap::new(),
        workflows: HashMap::new(),
        patterns: vec![],
        signature: None,
        run: RunSpec {
            id: None,
            name: None,
            created_at: None,
            defaults: RunDefaults::default(),
            workflow_ref: None,
            workflow: Some(WorkflowSpec {
                id: None,
                kind: WorkflowKind::Sequential,
                max_concurrency: None,
                steps: vec![],
            }),
            pattern_ref: None,
            inputs: HashMap::new(),
            placement: None,
            remote: None,
            delegation_policy: None,
        },
    };
    let err = doc.validate().expect_err("id mismatch should fail");
    assert!(err
        .to_string()
        .contains("providers.p1.id must match key 'p1'"));
}

#[test]
fn run_remote_trust_policy_defaults_and_source_validation() {
    let mut doc: AdlDoc = serde_yaml::from_str(
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
  t1:
    prompt:
      user: "hello"
run:
  workflow:
    steps:
      - id: "s1"
        agent: "a1"
        task: "t1"
  remote:
    endpoint: "http://127.0.0.1:7000"
"#,
    )
    .expect("parse");
    doc.validate()
        .expect("default remote policy should validate");
    let remote = doc.run.remote.as_ref().expect("remote");
    assert!(!remote.require_signed_requests);
    assert!(!remote.require_key_id);
    assert!(remote.verify_allowed_algs.is_empty());
    assert!(remote.verify_allowed_key_sources.is_empty());

    let remote = doc.run.remote.as_mut().expect("remote");
    remote.require_key_id = true;
    remote.require_signed_requests = false;
    let err = doc
        .validate()
        .expect_err("require_key_id needs require_signed_requests");
    assert!(err.to_string().contains(
        "run.remote.require_key_id=true requires run.remote.require_signed_requests=true"
    ));

    let remote = doc.run.remote.as_mut().expect("remote");
    remote.require_signed_requests = true;
    remote.verify_allowed_key_sources = vec!["bad-source".to_string()];
    let err = doc.validate().expect_err("bad key source should fail");
    assert!(err
        .to_string()
        .contains("verify_allowed_key_sources contains unsupported source"));
}

#[test]
fn validate_rejects_unknown_references_across_agent_and_task_fields() {
    let mut doc: AdlDoc = serde_yaml::from_str(
        r#"
version: "0.5"
providers:
  p1:
    type: "ollama"
tools:
  t1:
    kind: "builtin"
agents:
  a1:
    provider: "p1"
    model: "m1"
    tools: ["t1"]
tasks:
  task1:
    agent_ref: "a1"
    tool_allowlist: ["t1"]
    prompt:
      user: "u"
run:
  workflow:
    steps:
      - id: "s1"
        agent: "a1"
        task: "task1"
"#,
    )
    .expect("parse");
    doc.validate().expect("baseline doc should validate");

    doc.agents.get_mut("a1").expect("agent").provider = "missing-provider".to_string();
    let err = doc.validate().expect_err("unknown provider should fail");
    assert!(err.to_string().contains("references unknown provider"));

    doc.agents.get_mut("a1").expect("agent").provider = "p1".to_string();
    doc.agents.get_mut("a1").expect("agent").tools = vec!["missing-tool".to_string()];
    let err = doc.validate().expect_err("unknown agent tool should fail");
    assert!(err.to_string().contains("tools references unknown tool"));

    doc.agents.get_mut("a1").expect("agent").tools = vec!["t1".to_string()];
    doc.tasks.get_mut("task1").expect("task").agent_ref = Some("missing-agent".to_string());
    let err = doc
        .validate()
        .expect_err("unknown task agent_ref should fail");
    assert!(err
        .to_string()
        .contains("agent_ref references unknown agent"));

    doc.tasks.get_mut("task1").expect("task").agent_ref = Some("a1".to_string());
    doc.tasks.get_mut("task1").expect("task").tool_allowlist = vec!["missing-tool".to_string()];
    let err = doc
        .validate()
        .expect_err("unknown task tool_allowlist entry should fail");
    assert!(err
        .to_string()
        .contains("tool_allowlist references unknown tool"));
}

#[test]
fn pattern_validate_rejects_additional_fork_join_and_linear_edge_cases() {
    let mut linear = PatternSpec {
        id: "p".to_string(),
        kind: PatternKind::Linear,
        steps: vec!["   ".to_string()],
        fork: None,
        join: None,
    };
    let err = linear
        .validate()
        .expect_err("linear pattern should reject empty step symbols");
    assert!(err.to_string().contains("empty step symbol"));
    linear.steps = vec!["A".to_string()];
    linear.validate().expect("valid linear pattern");

    let mut fork_join = PatternSpec {
        id: "p".to_string(),
        kind: PatternKind::ForkJoin,
        steps: vec![],
        fork: Some(PatternForkSpec { branches: vec![] }),
        join: Some(PatternJoinSpec {
            step: "join".to_string(),
        }),
    };
    let err = fork_join
        .validate()
        .expect_err("fork_join must reject empty branch list");
    assert!(err.to_string().contains("fork.branches must not be empty"));

    fork_join.join = Some(PatternJoinSpec {
        step: "   ".to_string(),
    });
    fork_join.fork = Some(PatternForkSpec {
        branches: vec![PatternBranchSpec {
            id: "b1".to_string(),
            steps: vec!["A".to_string()],
        }],
    });
    let err = fork_join
        .validate()
        .expect_err("fork_join must reject empty join.step");
    assert!(err.to_string().contains("join.step must not be empty"));

    fork_join.join = Some(PatternJoinSpec {
        step: "join".to_string(),
    });
    fork_join.fork = Some(PatternForkSpec {
        branches: vec![PatternBranchSpec {
            id: "   ".to_string(),
            steps: vec!["A".to_string()],
        }],
    });
    let err = fork_join
        .validate()
        .expect_err("fork_join must reject empty branch id");
    assert!(err.to_string().contains("branch with empty id"));

    fork_join.fork = Some(PatternForkSpec {
        branches: vec![PatternBranchSpec {
            id: "p::reserved".to_string(),
            steps: vec!["A".to_string()],
        }],
    });
    let err = fork_join
        .validate()
        .expect_err("fork_join must reject reserved branch id prefix");
    assert!(err.to_string().contains("cannot use reserved prefix 'p::'"));

    fork_join.fork = Some(PatternForkSpec {
        branches: vec![PatternBranchSpec {
            id: "b1".to_string(),
            steps: vec![],
        }],
    });
    let err = fork_join
        .validate()
        .expect_err("fork_join must require non-empty branch steps");
    assert!(err
        .to_string()
        .contains("branch 'b1' requires non-empty steps"));

    fork_join.fork = Some(PatternForkSpec {
        branches: vec![PatternBranchSpec {
            id: "b1".to_string(),
            steps: vec!["   ".to_string()],
        }],
    });
    let err = fork_join
        .validate()
        .expect_err("fork_join must reject empty branch step symbol");
    assert!(err
        .to_string()
        .contains("branch 'b1' has empty step symbol"));
}

fn baseline_doc() -> AdlDoc {
    serde_yaml::from_str(
        r#"
version: "0.5"
providers:
  p1:
    type: "ollama"
tools:
  t1:
    kind: "builtin"
agents:
  a1:
    provider: "p1"
    model: "m1"
tasks:
  task1:
    prompt:
      user: "u"
run:
  workflow:
    steps:
      - id: "s1"
        agent: "a1"
        task: "task1"
"#,
    )
    .expect("baseline doc")
}

#[test]
fn validate_rejects_pattern_and_step_guardrail_edges() {
    let mut doc = baseline_doc();
    doc.patterns.push(PatternSpec {
        id: "".to_string(),
        kind: PatternKind::Linear,
        steps: vec!["s1".to_string()],
        fork: None,
        join: None,
    });
    let err = doc.validate().expect_err("empty pattern id should fail");
    assert!(err.to_string().contains("pattern id must not be empty"));

    let mut doc = baseline_doc();
    doc.patterns.push(PatternSpec {
        id: "dup".to_string(),
        kind: PatternKind::Linear,
        steps: vec!["s1".to_string()],
        fork: None,
        join: None,
    });
    doc.patterns.push(PatternSpec {
        id: "dup".to_string(),
        kind: PatternKind::Linear,
        steps: vec!["s1".to_string()],
        fork: None,
        join: None,
    });
    let err = doc
        .validate()
        .expect_err("duplicate pattern id should fail");
    assert!(err.to_string().contains("duplicate pattern id"));

    let mut doc = baseline_doc();
    doc.run.workflow.as_mut().expect("workflow").steps[0].id = Some("p::bad".to_string());
    let err = doc
        .validate()
        .expect_err("reserved step id prefix should fail");
    assert!(err.to_string().contains("reserved compiler prefix"));

    let mut doc = baseline_doc();
    let step = &mut doc.run.workflow.as_mut().expect("workflow").steps[0];
    step.save_as = None;
    step.write_to = Some("out.txt".to_string());
    let err = doc
        .validate()
        .expect_err("write_to without save_as should fail");
    assert!(err
        .to_string()
        .contains("uses write_to but is missing save_as"));

    let mut doc = baseline_doc();
    let step = &mut doc.run.workflow.as_mut().expect("workflow").steps[0];
    step.save_as = Some("x".to_string());
    step.write_to = Some("../escape.txt".to_string());
    let err = doc.validate().expect_err("parent traversal should fail");
    assert!(err.to_string().contains("relative path without '..'"));

    let mut doc = baseline_doc();
    let step = &mut doc.run.workflow.as_mut().expect("workflow").steps[0];
    step.retry = Some(StepRetry { max_attempts: 0 });
    let err = doc
        .validate()
        .expect_err("retry.max_attempts=0 should fail");
    assert!(err.to_string().contains("invalid retry.max_attempts=0"));
}

#[test]
fn validate_rejects_delegation_policy_and_pattern_ref_conflicts() {
    let mut doc = baseline_doc();
    doc.run.delegation_policy = Some(DelegationPolicySpec {
        default_allow: true,
        rules: vec![DelegationPolicyRuleSpec {
            id: "   ".to_string(),
            action: DelegationActionKind::ToolInvoke,
            target_id: None,
            effect: DelegationRuleEffect::Allow,
            require_approval: false,
        }],
    });
    let err = doc
        .validate()
        .expect_err("empty delegation rule id should fail");
    assert!(err.to_string().contains("id must not be empty"));

    let mut doc = baseline_doc();
    doc.run.delegation_policy = Some(DelegationPolicySpec {
        default_allow: true,
        rules: vec![
            DelegationPolicyRuleSpec {
                id: "r1".to_string(),
                action: DelegationActionKind::ToolInvoke,
                target_id: None,
                effect: DelegationRuleEffect::Allow,
                require_approval: false,
            },
            DelegationPolicyRuleSpec {
                id: "r1".to_string(),
                action: DelegationActionKind::ToolInvoke,
                target_id: Some(" ".to_string()),
                effect: DelegationRuleEffect::Deny,
                require_approval: false,
            },
        ],
    });
    let err = doc
        .validate()
        .expect_err("duplicate delegation rule id should fail");
    assert!(err.to_string().contains("duplicate id 'r1'"));

    let mut doc = baseline_doc();
    doc.run.pattern_ref = Some("missing-pattern".to_string());
    let err = doc.validate().expect_err("unknown pattern_ref should fail");
    assert!(err.to_string().contains("references unknown pattern"));

    let mut doc = baseline_doc();
    doc.patterns.push(PatternSpec {
        id: "p1".to_string(),
        kind: PatternKind::Linear,
        steps: vec!["s1".to_string()],
        fork: None,
        join: None,
    });
    doc.run.pattern_ref = Some("p1".to_string());
    let err = doc
        .validate()
        .expect_err("pattern_ref conflicts with workflow should fail");
    assert!(err
        .to_string()
        .contains("cannot be combined with run.workflow_ref or inline run.workflow"));
}

#[test]
fn load_from_file_include_validation_and_merge_errors() {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time")
        .as_nanos();
    let base =
        std::env::temp_dir().join(format!("adl-include-errors-{now}-{}", std::process::id()));
    std::fs::create_dir_all(&base).expect("create base");

    let err_doc = base.join("err.yaml");

    std::fs::write(&err_doc, "[]").expect("write seq top-level");
    let err = AdlDoc::load_from_file(err_doc.to_str().expect("utf8"))
        .expect_err("non-mapping top-level should fail");
    let msg = err.to_string();
    assert!(
        msg.contains("top-level ADL document must be a mapping")
            || msg.contains("parse adl yaml")
            || msg.contains("read/merge adl file"),
        "unexpected error: {msg}"
    );

    std::fs::write(
        &err_doc,
        r#"
version: "0.5"
include: "fragment.yaml"
run:
  workflow:
    steps: []
"#,
    )
    .expect("write include scalar");
    let err = AdlDoc::load_from_file(err_doc.to_str().expect("utf8"))
        .expect_err("include scalar should fail");
    assert!(err.to_string().contains("read/merge adl file"));

    std::fs::write(
        &err_doc,
        r#"
version: "0.5"
include: [7]
run:
  workflow:
    steps: []
"#,
    )
    .expect("write include non-string entry");
    let err = AdlDoc::load_from_file(err_doc.to_str().expect("utf8"))
        .expect_err("include entry non-string should fail");
    assert!(err.to_string().contains("read/merge adl file"));

    std::fs::write(
        &err_doc,
        r#"
version: "0.5"
include: ["../escape.yaml"]
run:
  workflow:
    steps: []
"#,
    )
    .expect("write include parent traversal");
    let err = AdlDoc::load_from_file(err_doc.to_str().expect("utf8"))
        .expect_err("include parent traversal should fail");
    assert!(err.to_string().contains("read/merge adl file"));

    let frag = base.join("fragment.yaml");
    std::fs::write(&frag, "[]").expect("write non-map fragment");
    std::fs::write(
        &err_doc,
        r#"
version: "0.5"
include: ["fragment.yaml"]
run:
  workflow:
    steps: []
"#,
    )
    .expect("write include fragment");
    let err = AdlDoc::load_from_file(err_doc.to_str().expect("utf8"))
        .expect_err("included non-mapping should fail");
    assert!(err.to_string().contains("read/merge adl file"));

    std::fs::write(
        &frag,
        r#"
providers:
  p:
    type: "ollama"
"#,
    )
    .expect("write provider fragment");
    std::fs::write(
        &err_doc,
        r#"
version: "0.5"
include: ["fragment.yaml"]
providers:
  p:
    type: "ollama"
run:
  workflow:
    steps: []
"#,
    )
    .expect("write duplicate provider map id");
    let err = AdlDoc::load_from_file(err_doc.to_str().expect("utf8"))
        .expect_err("duplicate map id should fail");
    assert!(err.to_string().contains("read/merge adl file"));

    std::fs::write(
        &frag,
        r#"
run:
  workflow:
    steps: []
"#,
    )
    .expect("write duplicate top-level key fragment");
    std::fs::write(
        &err_doc,
        r#"
version: "0.5"
include: ["fragment.yaml"]
run:
  workflow:
    steps: []
"#,
    )
    .expect("write duplicate top-level key main");
    let err = AdlDoc::load_from_file(err_doc.to_str().expect("utf8"))
        .expect_err("duplicate top-level key should fail");
    assert!(err.to_string().contains("read/merge adl file"));

    let _ = std::fs::remove_dir_all(base);
}

#[test]
fn runspec_pattern_ref_rejection_and_effective_prompt_priority_edges() {
    let mut doc = baseline_doc();
    doc.run.pattern_ref = Some("p1".to_string());
    let err = doc
        .run
        .resolve_workflow(&doc)
        .expect_err("pattern_ref should reject resolve_workflow");
    assert!(err
        .to_string()
        .contains("cannot be combined with run.workflow_ref"));

    let mut doc = baseline_doc();
    {
        let step = &mut doc.run.workflow.as_mut().expect("workflow").steps[0];
        step.prompt = None;
        step.task = Some("task1".to_string());
    }
    let p_from_task = doc.run.workflow.as_ref().expect("workflow").steps[0]
        .effective_prompt(&doc)
        .expect("task prompt")
        .user
        .clone();
    assert_eq!(p_from_task, Some("u".to_string()));

    doc.tasks.clear();
    doc.agents.get_mut("a1").expect("agent").prompt = Some(PromptSpec {
        system: Some("sys".to_string()),
        developer: None,
        user: Some("agent-user".to_string()),
        context: None,
        output: None,
    });
    let p_from_agent = doc.run.workflow.as_ref().expect("workflow").steps[0]
        .effective_prompt(&doc)
        .expect("agent prompt")
        .user
        .clone();
    assert_eq!(p_from_agent, Some("agent-user".to_string()));

    let empty = StepSpec {
        id: Some("s2".to_string()),
        ..StepSpec::default()
    };
    assert!(empty.effective_prompt(&doc).is_none());
}

#[test]
fn delegation_action_kind_as_str_covers_all_variants() {
    assert_eq!(DelegationActionKind::ToolInvoke.as_str(), "tool_invoke");
    assert_eq!(DelegationActionKind::ProviderCall.as_str(), "provider_call");
    assert_eq!(DelegationActionKind::RemoteExec.as_str(), "remote_exec");
    assert_eq!(
        DelegationActionKind::FilesystemRead.as_str(),
        "filesystem_read"
    );
    assert_eq!(
        DelegationActionKind::FilesystemWrite.as_str(),
        "filesystem_write"
    );
}
