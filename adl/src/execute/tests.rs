use super::runner::{
    effective_max_concurrency_with_source, effective_step_placement, resolve_call_binding,
};
use super::*;
use crate::adl::{
    AdlDoc, PlacementMode, PromptSpec, RunDefaults, RunPlacementSpec, RunSpec, StepRetry,
    WorkflowKind, WorkflowSpec,
};
use crate::resolve::AdlResolved;
use std::time::{SystemTime, UNIX_EPOCH};

fn minimal_resolved() -> AdlResolved {
    AdlResolved {
        run_id: "run".to_string(),
        workflow_id: "wf".to_string(),
        doc: AdlDoc {
            version: "0.3".to_string(),
            providers: HashMap::new(),
            tools: HashMap::new(),
            agents: HashMap::new(),
            tasks: HashMap::new(),
            workflows: HashMap::new(),
            patterns: vec![],
            signature: None,
            run: RunSpec {
                id: None,
                name: Some("run".to_string()),
                created_at: None,
                defaults: RunDefaults::default(),
                workflow_ref: None,
                workflow: Some(WorkflowSpec {
                    id: None,
                    kind: WorkflowKind::Concurrent,
                    max_concurrency: None,
                    steps: vec![],
                }),
                pattern_ref: None,
                inputs: HashMap::new(),
                placement: None,
                remote: None,
                delegation_policy: None,
            },
        },
        steps: vec![],
        execution_plan: crate::execution_plan::ExecutionPlan {
            workflow_kind: WorkflowKind::Concurrent,
            nodes: vec![],
        },
    }
}

fn step_with_write_to(id: &str, write_to: Option<&str>) -> crate::resolve::ResolvedStep {
    crate::resolve::ResolvedStep {
        id: id.to_string(),
        agent: None,
        provider: None,
        placement: None,
        task: None,
        call: None,
        with: HashMap::new(),
        as_ns: None,
        delegation: None,
        prompt: None,
        inputs: HashMap::new(),
        guards: vec![],
        save_as: Some("saved.output".to_string()),
        write_to: write_to.map(str::to_string),
        on_error: None,
        retry: None,
    }
}

fn remote_retry_step(max_attempts: u32) -> crate::resolve::ResolvedStep {
    crate::resolve::ResolvedStep {
        id: "remote-step".to_string(),
        agent: None,
        provider: Some("p1".to_string()),
        placement: Some(PlacementMode::Remote),
        task: None,
        call: None,
        with: HashMap::new(),
        as_ns: None,
        delegation: None,
        prompt: Some(PromptSpec {
            user: Some("hello".to_string()),
            ..Default::default()
        }),
        inputs: HashMap::new(),
        guards: vec![],
        save_as: None,
        write_to: None,
        on_error: None,
        retry: Some(StepRetry { max_attempts }),
    }
}

#[test]
fn resolve_state_inputs_resolves_and_validates_state_bindings() {
    let mut inputs = HashMap::new();
    inputs.insert("a".to_string(), "@state:key1".to_string());
    inputs.insert("b".to_string(), "literal".to_string());
    let mut state = HashMap::new();
    state.insert("key1".to_string(), "value1".to_string());

    let merged = resolve_state_inputs("s1", &inputs, &state).expect("resolve");
    assert_eq!(merged.get("a").map(String::as_str), Some("value1"));
    assert_eq!(merged.get("b").map(String::as_str), Some("literal"));

    inputs.insert("a".to_string(), "@state:".to_string());
    let empty_err = resolve_state_inputs("s1", &inputs, &state).expect_err("empty key fails");
    assert!(empty_err
        .to_string()
        .contains("uses @state: with an empty key"));

    inputs.insert("a".to_string(), "@state:missing".to_string());
    let missing_err = resolve_state_inputs("s1", &inputs, &state).expect_err("missing key fails");
    assert!(missing_err
        .to_string()
        .contains("references missing saved state"));
}

#[test]
fn missing_prompt_inputs_dedupes_and_sorts_missing_keys() {
    let prompt = PromptSpec {
        system: Some("{{ b }}".to_string()),
        user: Some("{{a}} and {{b}} and {{ a }}".to_string()),
        context: Some("{{c}}".to_string()),
        ..Default::default()
    };
    let mut inputs = HashMap::new();
    inputs.insert("c".to_string(), "ok".to_string());
    let missing = missing_prompt_inputs(&prompt, &inputs);
    assert_eq!(missing, vec!["a".to_string(), "b".to_string()]);
}

#[test]
fn validate_write_to_rejects_invalid_paths() {
    let empty_err = validate_write_to("s1", "   ").expect_err("empty should fail");
    assert!(empty_err.to_string().contains("empty write_to"));

    let abs_err = validate_write_to("s1", "/tmp/out.txt").expect_err("absolute should fail");
    assert!(abs_err.to_string().contains("must be a relative path"));

    let traversal_err =
        validate_write_to("s1", "../escape.txt").expect_err("traversal should fail");
    assert!(traversal_err.to_string().contains("without '..'"));
}

#[test]
fn write_output_creates_parent_directories() {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time")
        .as_nanos();
    let out_dir =
        std::env::temp_dir().join(format!("adl-write-output-{now}-{}", std::process::id()));
    let path = write_output("s1", &out_dir, "nested/result.txt", "hello").expect("write output");
    let written = std::fs::read_to_string(&path).expect("read output");
    assert_eq!(written, "hello");
    let _ = std::fs::remove_dir_all(out_dir);
}

#[cfg(unix)]
#[test]
fn write_output_rejects_symlink_escape() {
    use std::os::unix::fs as unix_fs;

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time")
        .as_nanos();
    let base = std::env::temp_dir().join(format!(
        "adl-write-output-symlink-{now}-{}",
        std::process::id()
    ));
    let out_dir = base.join("root");
    let outside = base.join("outside");
    std::fs::create_dir_all(&out_dir).expect("create out dir");
    std::fs::create_dir_all(&outside).expect("create outside dir");
    unix_fs::symlink(&outside, out_dir.join("link")).expect("create symlink");

    let err = write_output("s1", &out_dir, "link/escape.txt", "data")
        .expect_err("symlink escape must be rejected");
    assert!(
        err.to_string().contains("sandbox resolver"),
        "unexpected error: {err}"
    );

    let _ = std::fs::remove_dir_all(base);
}

#[test]
fn resume_disposition_validates_saved_artifacts_and_fingerprints() {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time")
        .as_nanos();
    let out_dir = std::env::temp_dir().join(format!(
        "adl-resume-disposition-{now}-{}",
        std::process::id()
    ));
    std::fs::create_dir_all(out_dir.join("nested")).expect("create nested out dir");
    let artifact = out_dir.join("nested").join("result.txt");
    std::fs::write(&artifact, "hello").expect("write artifact");

    let step = crate::resolve::ResolvedStep {
        id: "s1".to_string(),
        agent: None,
        provider: None,
        placement: None,
        task: None,
        call: None,
        with: HashMap::new(),
        as_ns: None,
        delegation: None,
        prompt: None,
        inputs: HashMap::new(),
        guards: vec![],
        save_as: Some("saved".to_string()),
        write_to: Some("nested/result.txt".to_string()),
        on_error: None,
        retry: None,
    };

    let missing = resume_disposition_for_step(&step, &out_dir, &HashMap::new())
        .expect("missing fingerprint should rerun");
    assert_eq!(
        missing,
        ResumeDisposition::Rerun("missing_output_fingerprint")
    );

    let mut wrong = HashMap::new();
    wrong.insert("s1".to_string(), model_output_fingerprint("different"));
    let mismatch =
        resume_disposition_for_step(&step, &out_dir, &wrong).expect("mismatch should rerun");
    assert_eq!(
        mismatch,
        ResumeDisposition::Rerun("invalid_expected_artifact")
    );

    let mut exact = HashMap::new();
    exact.insert("s1".to_string(), model_output_fingerprint("hello"));
    let verified = resume_disposition_for_step(&step, &out_dir, &exact).expect("match should skip");
    assert_eq!(
        verified,
        ResumeDisposition::Skip("completed_artifact_verified")
    );

    std::fs::write(&artifact, vec![0xff, 0xfe, 0xfd]).expect("overwrite with invalid utf8");
    let err = resume_disposition_for_step(&step, &out_dir, &exact)
        .expect_err("invalid utf8 should surface artifact read failure");
    assert!(err
        .to_string()
        .contains("failed to read expected resume artifact"));

    let _ = std::fs::remove_dir_all(out_dir);
}

#[test]
fn effective_max_concurrency_precedence_and_validation() {
    let mut resolved = minimal_resolved();
    assert_eq!(
        effective_max_concurrency_with_source(&resolved)
            .expect("default")
            .0,
        4
    );

    resolved.doc.run.defaults.max_concurrency = Some(3);
    assert_eq!(
        effective_max_concurrency_with_source(&resolved)
            .expect("run default")
            .0,
        3
    );

    resolved
        .doc
        .run
        .workflow
        .as_mut()
        .expect("workflow")
        .max_concurrency = Some(2);
    assert_eq!(
        effective_max_concurrency_with_source(&resolved)
            .expect("workflow override")
            .0,
        2
    );

    resolved
        .doc
        .run
        .workflow
        .as_mut()
        .expect("workflow")
        .max_concurrency = Some(0);
    let err = effective_max_concurrency_with_source(&resolved).expect_err("zero should fail");
    assert!(err.to_string().contains("must be >= 1"));
}

#[test]
fn effective_max_concurrency_supports_workflow_ref_overrides() {
    let mut resolved = minimal_resolved();
    resolved.doc.run.workflow = None;
    resolved.doc.run.workflow_ref = Some("wf_ref".to_string());
    resolved.doc.workflows.insert(
        "wf_ref".to_string(),
        WorkflowSpec {
            id: Some("wf_ref".to_string()),
            kind: WorkflowKind::Concurrent,
            max_concurrency: Some(5),
            steps: vec![],
        },
    );
    assert_eq!(
        effective_max_concurrency_with_source(&resolved)
            .expect("workflow ref override")
            .0,
        5
    );
}

#[test]
fn pause_reason_for_step_detects_pause_guard_and_optional_reason() {
    let mut step = crate::resolve::ResolvedStep {
        id: "s1".to_string(),
        agent: None,
        provider: None,
        placement: None,
        task: None,
        call: None,
        with: HashMap::new(),
        as_ns: None,
        delegation: None,
        prompt: None,
        inputs: HashMap::new(),
        guards: vec![],
        save_as: None,
        write_to: None,
        on_error: None,
        retry: None,
    };
    assert_eq!(pause_reason_for_step(&step), None);

    step.guards.push(crate::adl::GuardSpec {
        kind: "pause".to_string(),
        config: HashMap::new(),
    });
    assert_eq!(pause_reason_for_step(&step), Some(None));

    step.guards[0].config.insert(
        "reason".to_string(),
        serde_json::Value::String("needs review".to_string()),
    );
    assert_eq!(
        pause_reason_for_step(&step),
        Some(Some("needs review".to_string()))
    );
}

#[test]
fn execute_step_with_retry_does_not_retry_remote_schema_violation() {
    let mut doc = minimal_resolved().doc;
    doc.providers.insert(
        "p1".to_string(),
        crate::adl::ProviderSpec {
            id: None,
            profile: None,
            kind: "http".to_string(),
            base_url: None,
            default_model: None,
            config: HashMap::new(),
        },
    );
    doc.run.placement = Some(RunPlacementSpec::Mode(PlacementMode::Remote));
    doc.run.remote = None;
    let step = remote_retry_step(3);

    let failure = execute_step_with_retry_core(
        &step,
        &doc,
        "run-1",
        "wf-1",
        &HashMap::new(),
        std::path::Path::new("."),
        false,
        |_| {},
    )
    .expect_err("remote schema violation should fail");

    assert_eq!(failure.attempts, 1);
    assert!(
        failure.err.to_string().contains("REMOTE_SCHEMA_VIOLATION"),
        "unexpected error: {:#}",
        failure.err
    );
}

#[test]
fn effective_step_placement_prefers_step_then_run_then_local_default() {
    let mut doc = minimal_resolved().doc;
    let step = crate::resolve::ResolvedStep {
        id: "s1".to_string(),
        agent: None,
        provider: None,
        placement: None,
        task: None,
        call: None,
        with: HashMap::new(),
        as_ns: None,
        delegation: None,
        prompt: None,
        inputs: HashMap::new(),
        guards: vec![],
        save_as: None,
        write_to: None,
        on_error: None,
        retry: None,
    };
    assert_eq!(
        effective_step_placement(&step, &doc),
        crate::adl::PlacementMode::Local
    );

    doc.run.placement = Some(crate::adl::RunPlacementSpec::Mode(
        crate::adl::PlacementMode::Remote,
    ));
    assert_eq!(
        effective_step_placement(&step, &doc),
        crate::adl::PlacementMode::Remote
    );

    let mut step_override = step.clone();
    step_override.placement = Some(crate::adl::PlacementMode::Local);
    assert_eq!(
        effective_step_placement(&step_override, &doc),
        crate::adl::PlacementMode::Local
    );
}

#[test]
fn resolve_call_binding_requires_state_prefix_and_known_key() {
    let mut state = HashMap::new();
    state.insert("inputs.topic".to_string(), "ADL".to_string());

    let resolved =
        resolve_call_binding("@state:inputs.topic", &state).expect("state binding should work");
    assert_eq!(resolved, "ADL");
    let templated = resolve_call_binding("{{ state.inputs.topic }}", &state)
        .expect("templated state binding should work");
    assert_eq!(templated, "ADL");

    let missing = resolve_call_binding("@state:inputs.missing", &state)
        .expect_err("missing state key should fail");
    assert!(missing.to_string().contains("missing state key"));

    let passthrough = resolve_call_binding("literal", &state).expect("literal passthrough");
    assert_eq!(passthrough, "literal");
}

#[test]
fn stable_failure_kind_detects_only_policy_errors() {
    let policy_err = anyhow::Error::new(ExecutionPolicyError {
        kind: ExecutionPolicyErrorKind::Denied,
        step_id: "s1".to_string(),
        action_kind: "tool".to_string(),
        target_id: "fs.write".to_string(),
        rule_id: Some("rule-1".to_string()),
    });
    assert_eq!(stable_failure_kind(&policy_err), Some("policy_denied"));

    let generic = anyhow::anyhow!("not policy related");
    assert_eq!(stable_failure_kind(&generic), None);
}

#[test]
fn scheduler_policy_source_as_str_matches_wire_values() {
    assert_eq!(
        SchedulerPolicySource::WorkflowOverride.as_str(),
        "workflow_override"
    );
    assert_eq!(SchedulerPolicySource::RunDefault.as_str(), "run_default");
    assert_eq!(
        SchedulerPolicySource::EngineDefault.as_str(),
        "engine_default"
    );
}

#[test]
fn execution_policy_error_code_covers_all_kinds() {
    let denied = ExecutionPolicyError {
        kind: ExecutionPolicyErrorKind::Denied,
        step_id: "s1".to_string(),
        action_kind: "provider_call".to_string(),
        target_id: "default".to_string(),
        rule_id: None,
    };
    assert_eq!(denied.code(), DELEGATION_POLICY_DENY_CODE);

    let approval = ExecutionPolicyError {
        kind: ExecutionPolicyErrorKind::ApprovalRequired,
        step_id: "s1".to_string(),
        action_kind: "provider_call".to_string(),
        target_id: "default".to_string(),
        rule_id: None,
    };
    assert_eq!(approval.code(), "DELEGATION_POLICY_APPROVAL_REQUIRED");
}

#[test]
fn execution_policy_error_display_includes_rule_id_for_denied() {
    let denied = ExecutionPolicyError {
        kind: ExecutionPolicyErrorKind::Denied,
        step_id: "step-a".to_string(),
        action_kind: "remote_exec".to_string(),
        target_id: "profile-x".to_string(),
        rule_id: Some("rule-7".to_string()),
    };
    let rendered = denied.to_string();
    assert!(rendered.contains("denied"));
    assert!(rendered.contains("rule_id=rule-7"));
}

#[test]
fn execution_policy_error_display_handles_approval_without_rule_id() {
    let approval = ExecutionPolicyError {
        kind: ExecutionPolicyErrorKind::ApprovalRequired,
        step_id: "step-b".to_string(),
        action_kind: "provider_call".to_string(),
        target_id: "provider-1".to_string(),
        rule_id: None,
    };
    let rendered = approval.to_string();
    assert!(rendered.contains("requires approval"));
    assert!(!rendered.contains("rule_id="));
}

#[test]
fn pause_reason_for_step_ignores_non_pause_guards() {
    let mut step = step_with_write_to("s1", None);
    step.guards.push(crate::adl::GuardSpec {
        kind: "retry".to_string(),
        config: HashMap::new(),
    });
    assert_eq!(pause_reason_for_step(&step), None);
}

#[test]
fn scheduler_policy_for_run_returns_none_for_sequential_workflows() {
    let mut resolved = minimal_resolved();
    resolved.execution_plan.workflow_kind = WorkflowKind::Sequential;
    resolved.doc.run.workflow = Some(WorkflowSpec {
        id: None,
        kind: WorkflowKind::Sequential,
        max_concurrency: None,
        steps: vec![],
    });
    let policy = scheduler_policy_for_run(&resolved).expect("sequential policy");
    assert!(policy.is_none());
}

#[test]
fn effective_max_concurrency_tracks_source_order() {
    let mut resolved = minimal_resolved();
    assert_eq!(
        effective_max_concurrency_with_source(&resolved).expect("engine default"),
        (
            DEFAULT_MAX_CONCURRENCY,
            SchedulerPolicySource::EngineDefault
        )
    );

    resolved.doc.run.defaults.max_concurrency = Some(6);
    assert_eq!(
        effective_max_concurrency_with_source(&resolved).expect("run default"),
        (6, SchedulerPolicySource::RunDefault)
    );

    resolved
        .doc
        .run
        .workflow
        .as_mut()
        .expect("workflow")
        .max_concurrency = Some(3);
    assert_eq!(
        effective_max_concurrency_with_source(&resolved).expect("workflow override"),
        (3, SchedulerPolicySource::WorkflowOverride)
    );
}

#[test]
fn resume_disposition_for_step_skips_when_no_artifact_expected() {
    let step = step_with_write_to("s1", None);
    let out_dir = std::env::temp_dir();
    let completed_outputs = HashMap::new();
    let disposition =
        resume_disposition_for_step(&step, &out_dir, &completed_outputs).expect("disposition");
    assert_eq!(
        disposition,
        ResumeDisposition::Skip("completed_no_artifact_expected")
    );
}

#[test]
fn resume_disposition_for_step_reruns_when_expected_artifact_missing() {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time")
        .as_nanos();
    let out_dir = std::env::temp_dir().join(format!("adl-resume-missing-{now}"));
    std::fs::create_dir_all(&out_dir).expect("create out_dir");
    let step = step_with_write_to("s1", Some("outputs/out.txt"));
    let completed_outputs = HashMap::new();
    let disposition =
        resume_disposition_for_step(&step, &out_dir, &completed_outputs).expect("disposition");
    assert_eq!(
        disposition,
        ResumeDisposition::Rerun("missing_expected_artifact")
    );
    let _ = std::fs::remove_dir_all(out_dir);
}

#[test]
fn resume_disposition_for_step_reruns_when_fingerprint_missing() {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time")
        .as_nanos();
    let out_dir = std::env::temp_dir().join(format!("adl-resume-no-fp-{now}"));
    let artifact = out_dir.join("outputs/out.txt");
    std::fs::create_dir_all(artifact.parent().expect("parent")).expect("create parent");
    std::fs::write(&artifact, "ok").expect("write artifact");
    let step = step_with_write_to("s1", Some("outputs/out.txt"));
    let completed_outputs = HashMap::new();
    let disposition =
        resume_disposition_for_step(&step, &out_dir, &completed_outputs).expect("disposition");
    assert_eq!(
        disposition,
        ResumeDisposition::Rerun("missing_output_fingerprint")
    );
    let _ = std::fs::remove_dir_all(out_dir);
}

#[test]
fn resume_disposition_for_step_reruns_on_fingerprint_mismatch() {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time")
        .as_nanos();
    let out_dir = std::env::temp_dir().join(format!("adl-resume-bad-fp-{now}"));
    let artifact = out_dir.join("outputs/out.txt");
    std::fs::create_dir_all(artifact.parent().expect("parent")).expect("create parent");
    std::fs::write(&artifact, "actual").expect("write artifact");
    let step = step_with_write_to("s1", Some("outputs/out.txt"));
    let mut completed_outputs = HashMap::new();
    completed_outputs.insert("s1".to_string(), model_output_fingerprint("expected"));
    let disposition =
        resume_disposition_for_step(&step, &out_dir, &completed_outputs).expect("disposition");
    assert_eq!(
        disposition,
        ResumeDisposition::Rerun("invalid_expected_artifact")
    );
    let _ = std::fs::remove_dir_all(out_dir);
}

#[test]
fn resume_disposition_for_step_skips_when_artifact_and_fingerprint_match() {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time")
        .as_nanos();
    let out_dir = std::env::temp_dir().join(format!("adl-resume-ok-fp-{now}"));
    let artifact = out_dir.join("outputs/out.txt");
    std::fs::create_dir_all(artifact.parent().expect("parent")).expect("create parent");
    std::fs::write(&artifact, "stable output").expect("write artifact");
    let step = step_with_write_to("s1", Some("outputs/out.txt"));
    let mut completed_outputs = HashMap::new();
    completed_outputs.insert("s1".to_string(), model_output_fingerprint("stable output"));
    let disposition =
        resume_disposition_for_step(&step, &out_dir, &completed_outputs).expect("disposition");
    assert_eq!(
        disposition,
        ResumeDisposition::Skip("completed_artifact_verified")
    );
    let _ = std::fs::remove_dir_all(out_dir);
}

#[test]
fn execution_policy_error_display_covers_approval_required_branch() {
    let err = ExecutionPolicyError {
        kind: ExecutionPolicyErrorKind::ApprovalRequired,
        step_id: "s-approve".to_string(),
        action_kind: "tool".to_string(),
        target_id: "fs.write".to_string(),
        rule_id: Some("rule-approval".to_string()),
    };

    assert_eq!(err.code(), "DELEGATION_POLICY_APPROVAL_REQUIRED");
    let rendered = err.to_string();
    assert!(rendered.contains("requires approval"));
    assert!(rendered.contains("rule_id=rule-approval"));
}

#[test]
fn execute_called_workflow_rejects_unknown_workflow() {
    let resolved = minimal_resolved();
    let mut tr = crate::trace::Trace::new("run", "wf", "0.8");
    let caller_state = HashMap::new();
    let err = execute_called_workflow(
        "caller",
        "ns",
        "missing-workflow",
        &HashMap::new(),
        &resolved,
        &mut tr,
        false,
        false,
        Path::new("."),
        Path::new("."),
        &caller_state,
    )
    .expect_err("unknown called workflow should fail");
    assert!(err.to_string().contains("call references unknown workflow"));
}

#[test]
fn execute_called_workflow_rejects_missing_state_binding_in_call_with() {
    let mut resolved = minimal_resolved();
    resolved.doc.workflows.insert(
        "callee".to_string(),
        WorkflowSpec {
            id: Some("callee".to_string()),
            kind: WorkflowKind::Sequential,
            max_concurrency: None,
            steps: vec![],
        },
    );

    let mut tr = crate::trace::Trace::new("run", "wf", "0.8");
    let mut call_with = HashMap::new();
    call_with.insert("topic".to_string(), "@state:inputs.missing".to_string());
    let caller_state = HashMap::new();

    let err = execute_called_workflow(
        "caller",
        "ns",
        "callee",
        &call_with,
        &resolved,
        &mut tr,
        false,
        false,
        Path::new("."),
        Path::new("."),
        &caller_state,
    )
    .expect_err("missing call.with binding should fail");
    let text = err.to_string();
    assert!(text.contains("failed to resolve call.with binding"));
    assert!(text.contains("caller step"));
}

#[test]
fn execute_called_workflow_rejects_write_to_without_save_as() {
    let mut resolved = minimal_resolved();
    resolved.doc.workflows.insert(
        "callee".to_string(),
        WorkflowSpec {
            id: Some("callee".to_string()),
            kind: WorkflowKind::Sequential,
            max_concurrency: None,
            steps: vec![crate::adl::StepSpec {
                id: Some("writer".to_string()),
                write_to: Some("out/result.txt".to_string()),
                // This intentionally triggers the write_to/save_as contract.
                save_as: None,
                ..crate::adl::StepSpec::default()
            }],
        },
    );

    let mut tr = crate::trace::Trace::new("run", "wf", "0.8");
    let caller_state = HashMap::new();
    let err = execute_called_workflow(
        "caller",
        "ns",
        "callee",
        &HashMap::new(),
        &resolved,
        &mut tr,
        false,
        false,
        Path::new("."),
        Path::new("."),
        &caller_state,
    )
    .expect_err("write_to without save_as should fail in called workflow");
    assert!(err
        .to_string()
        .contains("uses write_to but is missing save_as"));
}

#[test]
fn execute_called_workflow_nested_call_failure_is_propagated() {
    let mut resolved = minimal_resolved();
    resolved.doc.workflows.insert(
        "parent".to_string(),
        WorkflowSpec {
            id: Some("parent".to_string()),
            kind: WorkflowKind::Sequential,
            max_concurrency: None,
            steps: vec![crate::adl::StepSpec {
                id: Some("call-child".to_string()),
                call: Some("missing-child".to_string()),
                ..crate::adl::StepSpec::default()
            }],
        },
    );

    let mut tr = crate::trace::Trace::new("run", "wf", "0.8");
    let caller_state = HashMap::new();
    let err = execute_called_workflow(
        "caller",
        "ns",
        "parent",
        &HashMap::new(),
        &resolved,
        &mut tr,
        false,
        false,
        Path::new("."),
        Path::new("."),
        &caller_state,
    )
    .expect_err("nested call to missing workflow should fail");
    assert!(err.to_string().contains("unknown workflow"));
}
