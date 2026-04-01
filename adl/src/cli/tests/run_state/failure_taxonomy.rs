use super::*;
use ::adl::failure_taxonomy as taxonomy_codes;

#[test]
fn classify_failure_kind_handles_sandbox_and_io_causes() {
    let sandbox_err = anyhow::Error::new(::adl::sandbox::SandboxPathError::PathDenied {
        requested_path: "sandbox:/bad".to_string(),
        reason: "parent_traversal",
    });
    assert_eq!(classify_failure_kind(&sandbox_err), Some("sandbox_denied"));

    let io_err = anyhow::Error::new(std::io::Error::other("disk issue"));
    assert_eq!(classify_failure_kind(&io_err), Some("io_error"));
}

#[test]
fn classify_failure_kind_covers_verification_and_replay_invariant_failures() {
    let unsigned_doc = adl::AdlDoc {
        version: "0.5".to_string(),
        providers: HashMap::new(),
        tools: HashMap::new(),
        agents: HashMap::new(),
        tasks: HashMap::new(),
        workflows: HashMap::new(),
        patterns: vec![],
        signature: None,
        run: adl::RunSpec {
            id: None,
            name: None,
            created_at: None,
            defaults: adl::RunDefaults::default(),
            workflow_ref: None,
            workflow: Some(adl::WorkflowSpec {
                id: None,
                kind: adl::WorkflowKind::Sequential,
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
    let verify_err = signing::verify_doc(&unsigned_doc, None).expect_err("unsigned verify");
    assert_eq!(
        classify_failure_kind(&verify_err),
        Some("verification_failed")
    );

    let bad_trace_path = std::env::temp_dir().join(format!(
        "adl-main-replay-kind-{}-{}.json",
        now_ms(),
        std::process::id()
    ));
    std::fs::write(&bad_trace_path, "{\"activation_log_version\":1,\"ordering\":\"bad\",\"stable_ids\":{\"step_id\":\"x\",\"delegation_id\":\"x\",\"run_id\":\"x\"},\"events\":[]}")
        .expect("write bad replay file");
    let replay_err =
        instrumentation::load_trace_artifact(&bad_trace_path).expect_err("ordering mismatch");
    assert_eq!(
        classify_failure_kind(&replay_err),
        Some("replay_invariant_violation")
    );
    let _ = std::fs::remove_file(&bad_trace_path);
}

#[test]
fn taxonomy_category_mapping_is_stable_for_core_codes() {
    assert_eq!(
        taxonomy_codes::category_for_code("policy_denied"),
        taxonomy_codes::POLICY_DENIED
    );
    assert_eq!(
        taxonomy_codes::category_for_code("verification_failed"),
        taxonomy_codes::VERIFICATION_FAILED
    );
    assert_eq!(
        taxonomy_codes::category_for_code("replay_invariant_violation"),
        taxonomy_codes::REPLAY_INVARIANT_VIOLATION
    );
    assert_eq!(
        taxonomy_codes::category_for_code("provider_error"),
        taxonomy_codes::TOOL_FAILURE
    );
}

#[test]
fn classify_failure_kind_returns_none_for_unclassified_errors() {
    let generic = anyhow::anyhow!("generic failure");
    assert_eq!(classify_failure_kind(&generic), None);
}

#[test]
fn execution_plan_hash_is_deterministic_for_same_plan() {
    let resolved = minimal_resolved_for_artifacts("hash-run".to_string());
    let a = execution_plan_hash(&resolved.execution_plan).expect("hash a");
    let b = execution_plan_hash(&resolved.execution_plan).expect("hash b");
    assert_eq!(a, b);
    assert_eq!(a.len(), 16, "fnv-1a hex length should be stable");
}
