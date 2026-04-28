use super::*;
use ::adl::trace_schema_v1::{
    validate_trace_event_envelope_v1, ContractValidationResultV1, TraceEventEnvelopeV1,
    TraceEventTypeV1,
};
use serde_json::Value as JsonValue;

#[test]
fn write_run_state_and_load_resume_round_trip() {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time")
        .as_nanos();
    let run_id = format!("run-{now}-{}", std::process::id());
    let resolved = minimal_resolved_for_artifacts(run_id.clone());
    let out_dir = std::env::temp_dir().join(format!("adl-main-out-{now}"));
    let runs_root = unique_temp_dir("adl-main-runs-roundtrip");
    let _runs_guard = EnvGuard::set("ADL_RUNS_ROOT", &runs_root.to_string_lossy());

    let mut tr = trace::Trace::new(run_id.clone(), "wf".to_string(), "0.5".to_string());
    tr.lifecycle_phase_entered(execute::RuntimeLifecyclePhase::Init);
    tr.execution_boundary_crossed(execute::ExecutionBoundary::RuntimeInit, "fresh_start");
    tr.lifecycle_phase_entered(execute::RuntimeLifecyclePhase::Execute);
    tr.step_started("s1", "a1", "p1", "t1", None);
    tr.step_finished("s1", true);
    tr.execution_boundary_crossed(execute::ExecutionBoundary::Pause, "entered");
    tr.lifecycle_phase_entered(execute::RuntimeLifecyclePhase::Teardown);

    let pause = execute::PauseState {
        paused_step_id: "s1".to_string(),
        reason: Some("review".to_string()),
        completed_step_ids: vec!["s1".to_string()],
        remaining_step_ids: vec![],
        saved_state: HashMap::from([(String::from("k"), String::from("v"))]),
        completed_outputs: HashMap::from([(String::from("s1_out"), String::from("done"))]),
    };

    let run_dir = write_run_state_artifacts(
        &resolved,
        &tr,
        Path::new("examples/adl-0.1.yaml"),
        &out_dir,
        100,
        150,
        "paused",
        Some(&pause),
        &[],
        &runtime_control_for("paused", &tr),
        None,
        None,
    )
    .expect("write run artifacts");

    assert!(
        run_dir.join("outputs").is_dir(),
        "artifact model v1 requires outputs/ directory"
    );
    assert!(
        run_dir.join("logs").is_dir(),
        "artifact model v1 requires logs/ directory"
    );
    assert!(
        run_dir.join("learning/overlays").is_dir(),
        "artifact model v1 requires learning/overlays directory"
    );
    assert!(
        run_dir.join("meta/ARTIFACT_MODEL.json").is_file(),
        "artifact model v1 requires version marker"
    );
    assert!(
        run_dir.join("logs/trace_v1.json").is_file(),
        "wp-03 requires canonical trace_v1.json artifact"
    );
    assert!(
        run_dir.join("run_manifest.json").is_file(),
        "trace provenance requires canonical run_manifest.json artifact"
    );
    let run_manifest: JsonValue = serde_json::from_str(
        &std::fs::read_to_string(run_dir.join("run_manifest.json"))
            .expect("read run_manifest.json"),
    )
    .expect("parse run_manifest.json");
    assert_eq!(
        run_manifest.get("schema_version").and_then(|v| v.as_str()),
        Some("trace_run_manifest.v1")
    );
    assert_eq!(
        run_manifest.get("milestone").and_then(|v| v.as_str()),
        Some("v0.5")
    );
    assert_eq!(
        run_manifest.get("runs_root").and_then(|v| v.as_str()),
        Some("external_runs_root")
    );
    assert!(
        !run_manifest
            .to_string()
            .contains(&runs_root.to_string_lossy().to_string()),
        "run manifest must not leak absolute override runs root"
    );

    let trace_v1: TraceEventEnvelopeV1 = serde_json::from_str(
        &std::fs::read_to_string(run_dir.join("logs/trace_v1.json")).expect("read trace_v1.json"),
    )
    .expect("parse trace_v1.json");
    validate_trace_event_envelope_v1(&trace_v1).expect("trace v1 must validate");
    assert_eq!(trace_v1.schema_version, "trace.v1");
    let event_types: Vec<TraceEventTypeV1> = trace_v1
        .events
        .iter()
        .map(|event| event.event_type.clone())
        .collect();
    assert!(event_types.contains(&TraceEventTypeV1::RunStart));
    assert!(event_types.contains(&TraceEventTypeV1::LifecyclePhase));
    assert!(event_types.contains(&TraceEventTypeV1::ExecutionBoundary));
    assert!(event_types.contains(&TraceEventTypeV1::StepStart));
    assert!(event_types.contains(&TraceEventTypeV1::StepEnd));
    assert!(event_types.contains(&TraceEventTypeV1::RunEnd));

    let resume =
        load_resume_state(&run_dir.join("run.json"), &resolved).expect("load resume state");
    assert!(resume.completed_step_ids.contains("s1"));
    assert_eq!(resume.saved_state.get("k").map(String::as_str), Some("v"));
    let run_status: JsonValue = serde_json::from_str(
        &std::fs::read_to_string(run_dir.join("run_status.json")).expect("read run_status.json"),
    )
    .expect("parse run_status.json");
    assert_eq!(
        run_status.get("resilience_classification"),
        Some(&JsonValue::String("interruption".to_string()))
    );
    assert_eq!(
        run_status.get("continuity_status"),
        Some(&JsonValue::String("resume_ready".to_string()))
    );
    assert_eq!(
        run_status.get("preservation_status"),
        Some(&JsonValue::String("pause_state_preserved".to_string()))
    );
    assert_eq!(
        run_status.get("shepherd_decision"),
        Some(&JsonValue::String("preserve_and_resume".to_string()))
    );
    assert_eq!(
        run_status.get("persistence_mode"),
        Some(&JsonValue::String("checkpoint_resume_state".to_string()))
    );
    assert_eq!(
        run_status.get("cleanup_disposition"),
        Some(&JsonValue::String("retain_pause_state".to_string()))
    );
    assert_eq!(
        run_status.get("resume_guard"),
        Some(&JsonValue::String(
            "execution_plan_hash_match_required".to_string()
        ))
    );
    assert_eq!(
        run_status.get("state_artifacts"),
        Some(&JsonValue::Array(vec![
            JsonValue::String("run.json".to_string()),
            JsonValue::String("steps.json".to_string()),
            JsonValue::String("run_status.json".to_string()),
            JsonValue::String("logs/trace_v1.json".to_string()),
            JsonValue::String("pause_state.json".to_string()),
        ]))
    );
    assert!(
        run_dir.join("pause_state.json").exists(),
        "paused runs must persist pause_state.json"
    );
    let pause_artifact =
        run_artifacts::load_pause_state_artifact(&run_dir.join("pause_state.json"))
            .expect("load pause state");
    assert_eq!(pause_artifact.adl_path, "examples/adl-0.1.yaml");
    let _ = std::fs::remove_dir_all(run_dir);
    let _ = std::fs::remove_dir_all(out_dir);
}

#[test]
fn write_run_state_artifacts_sanitizes_external_absolute_adl_path_in_pause_state() {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time")
        .as_nanos();
    let run_id = format!("pause-sanitize-{now}-{}", std::process::id());
    let resolved = minimal_resolved_for_artifacts(run_id.clone());
    let out_dir = std::env::temp_dir().join(format!("adl-main-out-sanitize-{now}"));
    let runs_root = unique_temp_dir("adl-main-runs-pause-sanitize");
    let _runs_guard = EnvGuard::set("ADL_RUNS_ROOT", &runs_root.to_string_lossy());

    let mut tr = trace::Trace::new(run_id.clone(), "wf".to_string(), "0.5".to_string());
    tr.step_started("s1", "a1", "p1", "t1", None);
    tr.step_finished("s1", true);

    let pause = execute::PauseState {
        paused_step_id: "s1".to_string(),
        reason: Some("review".to_string()),
        completed_step_ids: vec!["s1".to_string()],
        remaining_step_ids: vec![],
        saved_state: HashMap::new(),
        completed_outputs: HashMap::new(),
    };

    let absolute_adl = std::env::temp_dir().join("adl-sensitive-host-path-demo.adl.yaml");
    let run_dir = write_run_state_artifacts(
        &resolved,
        &tr,
        &absolute_adl,
        &out_dir,
        100,
        150,
        "paused",
        Some(&pause),
        &[],
        &runtime_control_for("paused", &tr),
        None,
        None,
    )
    .expect("write run artifacts");

    let pause_artifact =
        run_artifacts::load_pause_state_artifact(&run_dir.join("pause_state.json"))
            .expect("load pause state");
    assert_eq!(
        pause_artifact.adl_path,
        "external:/adl-sensitive-host-path-demo.adl.yaml"
    );
    assert!(
        !pause_artifact.adl_path.starts_with('/'),
        "pause artifact should not retain raw host absolute paths"
    );

    let _ = std::fs::remove_dir_all(run_dir);
    let _ = std::fs::remove_dir_all(out_dir);
}

#[test]
fn trace_v1_records_delegation_and_failure_events() {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time")
        .as_nanos();
    let run_id = format!("trace-delegation-failure-{now}-{}", std::process::id());
    let resolved = minimal_resolved_for_artifacts(run_id.clone());
    let out_dir = std::env::temp_dir().join(format!("adl-main-trace-delegation-{now}"));
    let runs_root = unique_temp_dir("adl-main-runs-trace-delegation");
    let _runs_guard = EnvGuard::set("ADL_RUNS_ROOT", &runs_root.to_string_lossy());

    let mut tr = trace::Trace::new(run_id.clone(), "wf".to_string(), "0.5".to_string());
    tr.governed_proposal_observed(
        "proposal.fixture.safe-read",
        "fixture.safe_read",
        &format!("artifacts/{run_id}/governed/proposal_arguments.redacted.json"),
    );
    tr.governed_proposal_normalized(
        "proposal.fixture.safe-read",
        "normalized.proposal.fixture.safe-read",
        &format!("artifacts/{run_id}/governed/proposal_arguments.redacted.json"),
    );
    tr.governed_acc_constructed(
        "proposal.fixture.safe-read",
        "acc.compiler.proposal.fixture.safe-read",
        "deterministic_fixture_compiler",
    );
    tr.governed_policy_injected(
        "proposal.fixture.safe-read",
        "policy.fixture.safe-read",
        "allowed",
    );
    tr.governed_visibility_resolved(
        "proposal.fixture.safe-read",
        "compiled ACC request status",
        "full compiler fixture evidence",
        "redacted compiler evidence and policy result",
        "aggregate compiler pass/fail only",
        "redacted compiler governance event",
    );
    tr.governed_freedom_gate_decided(
        "proposal.fixture.safe-read",
        "candidate.safe-read",
        "allowed",
        "gate_allowed",
        "execution",
        "private_arguments_redacted digest=sha256:bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
    );
    tr.governed_action_selected(
        "proposal.fixture.safe-read",
        "action.safe_read",
        "fixture.safe_read",
        "adapter.fixture.safe_read.dry_run",
        vec![
            "proposal:proposal.fixture.safe-read".to_string(),
            "acc:acc.compiler.proposal.fixture.safe-read".to_string(),
            "action:fixture_read".to_string(),
            "normalized_proposal:normalized.proposal.fixture.safe-read".to_string(),
            "policy:policy.fixture.safe-read".to_string(),
            "gate:candidate.safe-read".to_string(),
            "execution:action.safe_read".to_string(),
        ],
    );
    tr.governed_execution_result(
        "proposal.fixture.safe-read",
        "action.safe_read",
        "adapter.fixture.safe_read.dry_run",
        &format!("artifacts/{run_id}/governed/result.redacted.json"),
        vec![
            "gate:candidate.safe-read".to_string(),
            "execution:action.safe_read".to_string(),
        ],
    );
    tr.governed_action_rejected(
        "proposal.fixture.safe-read",
        "action.exfiltration",
        "fixture.safe_read",
        "adapter.fixture.safe_read.dry_run",
        "exfiltrating_action",
        vec![
            "classification:exfiltration".to_string(),
            "review:redacted".to_string(),
        ],
    );
    tr.governed_refusal(
        "proposal.fixture.safe-read",
        "action.exfiltration",
        "exfiltrating_action",
        vec![
            "classification:exfiltration".to_string(),
            "review:redacted".to_string(),
        ],
    );
    tr.governed_redaction_decision(
        "proposal.fixture.safe-read",
        "reviewer",
        vec![
            "arguments".to_string(),
            "results".to_string(),
            "errors".to_string(),
            "rejected_alternatives".to_string(),
        ],
        "redacted",
        Some("digest_only"),
    );
    tr.step_started("s1", "a1", "p1", "t1", None);
    tr.delegation_policy_evaluated("s1", "provider_call", "remote", "allowed", Some("rule-a"));
    tr.delegation_approved("s1");
    tr.step_finished("s1", true);
    tr.step_started("s2", "a1", "p1", "t2", None);
    tr.delegation_policy_evaluated(
        "s2",
        "filesystem_write",
        "/tmp/out",
        "denied",
        Some("rule-b"),
    );
    tr.delegation_denied("s2", "filesystem_write", "/tmp/out", Some("rule-b"));
    tr.step_finished("s2", false);
    tr.run_failed("delegation denied");

    let run_dir = write_run_state_artifacts(
        &resolved,
        &tr,
        Path::new("examples/adl-0.1.yaml"),
        &out_dir,
        100,
        175,
        "failure",
        None,
        &[],
        &runtime_control_for("failure", &tr),
        None,
        Some(&anyhow::anyhow!("delegation denied")),
    )
    .expect("write failed run artifacts");

    let trace_v1: TraceEventEnvelopeV1 = serde_json::from_str(
        &std::fs::read_to_string(run_dir.join("logs/trace_v1.json")).expect("read trace_v1.json"),
    )
    .expect("parse trace_v1.json");
    validate_trace_event_envelope_v1(&trace_v1).expect("trace v1 must validate");
    assert_eq!(trace_v1.schema_version, "trace.v2");

    assert!(trace_v1.events.iter().any(|event| {
        event.event_type == TraceEventTypeV1::ContractValidation
            && event
                .contract_validation
                .as_ref()
                .is_some_and(|validation| validation.result == ContractValidationResultV1::Pass)
    }));
    assert!(trace_v1
        .events
        .iter()
        .any(|event| event.event_type == TraceEventTypeV1::Proposal));
    assert!(trace_v1
        .events
        .iter()
        .any(|event| event.event_type == TraceEventTypeV1::ProposalNormalization));
    assert!(trace_v1
        .events
        .iter()
        .any(|event| event.event_type == TraceEventTypeV1::CapabilityContract));
    assert!(trace_v1
        .events
        .iter()
        .any(|event| event.event_type == TraceEventTypeV1::PolicyInjection));
    assert!(trace_v1
        .events
        .iter()
        .any(|event| event.event_type == TraceEventTypeV1::VisibilityPolicy));
    assert!(trace_v1
        .events
        .iter()
        .any(|event| event.event_type == TraceEventTypeV1::FreedomGateDecision));
    assert!(trace_v1
        .events
        .iter()
        .any(|event| event.event_type == TraceEventTypeV1::ActionSelection));
    assert!(trace_v1.events.iter().any(|event| {
        event.event_type == TraceEventTypeV1::ContractValidation
            && event
                .contract_validation
                .as_ref()
                .is_some_and(|validation| validation.result == ContractValidationResultV1::Fail)
    }));
    assert!(trace_v1
        .events
        .iter()
        .any(|event| event.event_type == TraceEventTypeV1::Approval));
    assert!(trace_v1
        .events
        .iter()
        .any(|event| event.event_type == TraceEventTypeV1::Rejection));
    assert!(trace_v1
        .events
        .iter()
        .any(|event| event.event_type == TraceEventTypeV1::ActionRejection));
    assert!(trace_v1
        .events
        .iter()
        .any(|event| event.event_type == TraceEventTypeV1::ExecutionResult));
    assert!(trace_v1.events.iter().any(|event| {
        event.event_type == TraceEventTypeV1::ActionSelection
            && event.governance.as_ref().is_some_and(|governance| {
                governance
                    .evidence_refs
                    .iter()
                    .any(|value| value == "gate:candidate.safe-read")
                    && governance
                        .evidence_refs
                        .iter()
                        .any(|value| value == "policy:policy.fixture.safe-read")
                    && governance
                        .evidence_refs
                        .iter()
                        .any(|value| value == "execution:action.safe_read")
            })
    }));
    assert!(trace_v1
        .events
        .iter()
        .any(|event| event.event_type == TraceEventTypeV1::Refusal));
    assert!(trace_v1.events.iter().any(|event| {
        event.event_type == TraceEventTypeV1::RedactionDecision
            && event.redaction.as_ref().is_some_and(|redaction| {
                redaction.audience == "reviewer"
                    && redaction
                        .surfaces
                        .iter()
                        .any(|surface| surface == "rejected_alternatives")
            })
    }));
    assert!(trace_v1.events.iter().any(|event| {
        event.event_type == TraceEventTypeV1::Error
            && event
                .error
                .as_ref()
                .is_some_and(|error| error.code == "STEP_FAILURE")
    }));
    assert!(trace_v1.events.iter().any(|event| {
        event.event_type == TraceEventTypeV1::Error
            && event
                .error
                .as_ref()
                .is_some_and(|error| error.code == "RUN_FAILURE")
    }));
    assert_eq!(
        trace_v1
            .events
            .last()
            .and_then(|event| event.decision_context.as_ref())
            .map(|context| context.outcome.as_str()),
        Some("failure")
    );
    assert_eq!(
        trace_v1.events.last().and_then(|event| {
            event
                .decision_context
                .as_ref()
                .and_then(|context| context.rationale.as_deref())
        }),
        Some("delegation denied")
    );
    let trace_v1_json = serde_json::to_string(&trace_v1).expect("serialize trace_v1");
    assert!(
        !trace_v1_json.contains("fixture-secret-token"),
        "trace must not leak secret-like payloads"
    );
    assert!(
        !trace_v1_json.contains("/Users/"),
        "trace must not leak absolute host paths"
    );
    assert!(
        trace_v1_json.contains("private_arguments_redacted"),
        "gate redaction summary should preserve accountable digest-only evidence"
    );
    let governed_arguments = run_dir.join("governed/proposal_arguments.redacted.json");
    let governed_results = run_dir.join("governed/result.redacted.json");
    assert!(
        governed_arguments.is_file(),
        "governed proposal redaction artifact should be persisted"
    );
    assert!(
        governed_results.is_file(),
        "governed result redaction artifact should be persisted"
    );

    let _ = std::fs::remove_dir_all(run_dir);
    let _ = std::fs::remove_dir_all(out_dir);
}

#[test]
fn resume_helpers_validate_pause_artifacts_and_steering_patches() {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time")
        .as_nanos();
    let run_id = format!("resume-helper-{now}-{}", std::process::id());
    let resolved = minimal_resolved_for_artifacts(run_id.clone());
    let out_dir = std::env::temp_dir().join(format!("adl-main-resume-helper-{now}"));
    let runs_root = unique_temp_dir("adl-main-runs-resume-helper");
    let _runs_guard = EnvGuard::set("ADL_RUNS_ROOT", &runs_root.to_string_lossy());

    let pause = execute::PauseState {
        paused_step_id: "s1".to_string(),
        reason: Some("review".to_string()),
        completed_step_ids: vec!["s1".to_string()],
        remaining_step_ids: vec!["s2".to_string()],
        saved_state: HashMap::from([(String::from("token"), String::from("kept"))]),
        completed_outputs: HashMap::new(),
    };
    let tr = trace::Trace::new(run_id.clone(), "wf".to_string(), "0.5".to_string());
    let run_dir = write_run_state_artifacts(
        &resolved,
        &tr,
        Path::new("examples/adl-0.1.yaml"),
        &out_dir,
        0,
        1,
        "paused",
        Some(&pause),
        &[],
        &runtime_control_for("paused", &tr),
        None,
        None,
    )
    .expect("write paused artifacts");

    let derived_path = resume_state_path_for_run_id(&run_id).expect("derive pause state path");
    assert_eq!(derived_path, run_dir.join("pause_state.json"));

    let pause_artifact =
        run_artifacts::load_pause_state_artifact(&derived_path).expect("load pause state");
    validate_pause_artifact_for_resume(&pause_artifact, &run_id, &resolved)
        .expect("pause artifact should validate for matching resolved run");

    let mut wrong_workflow = pause_artifact;
    wrong_workflow.workflow_id = "wf-other".to_string();
    let err = validate_pause_artifact_for_resume(&wrong_workflow, &run_id, &resolved)
        .expect_err("workflow mismatch should fail");
    assert!(err.to_string().contains("workflow_id mismatch"));

    let patch_path = run_dir.join("steering_patch.json");
    std::fs::write(
        &patch_path,
        serde_json::json!({
            "schema_version": execute::STEERING_PATCH_SCHEMA_VERSION,
            "apply_at": execute::STEERING_APPLY_AT_RESUME_BOUNDARY,
            "reason": "operator update",
            "set_state": { "token": "updated" },
            "remove_state": ["stale"]
        })
        .to_string(),
    )
    .expect("write steering patch");
    let (patch, fingerprint) = load_steering_patch(&patch_path).expect("load steering patch");
    assert_eq!(patch.reason.as_deref(), Some("operator update"));
    assert_eq!(
        patch.set_state.get("token").map(String::as_str),
        Some("updated")
    );
    assert!(!fingerprint.is_empty());

    let invalid_patch_path = run_dir.join("invalid_steering_patch.json");
    std::fs::write(
        &invalid_patch_path,
        serde_json::json!({
            "schema_version": execute::STEERING_PATCH_SCHEMA_VERSION,
            "apply_at": "wrong_boundary",
            "set_state": { "token": "updated" }
        })
        .to_string(),
    )
    .expect("write invalid steering patch");
    let err = load_steering_patch(&invalid_patch_path).expect_err("invalid patch should fail");
    assert!(err.to_string().contains("apply_at"));

    let _ = std::fs::remove_dir_all(run_dir);
    let _ = std::fs::remove_dir_all(out_dir);
}

#[test]
fn write_run_state_artifacts_projects_execute_owned_runtime_control_state() {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time")
        .as_nanos();
    let run_id = format!("runtime-control-{now}-{}", std::process::id());
    let resolved = minimal_resolved_for_artifacts(run_id.clone());
    let out_dir = std::env::temp_dir().join(format!("adl-main-runtime-control-{now}"));
    let runs_root = unique_temp_dir("adl-main-runs-runtime-control");
    let _runs_guard = EnvGuard::set("ADL_RUNS_ROOT", &runs_root.to_string_lossy());

    let mut tr = trace::Trace::new(run_id.clone(), "wf".to_string(), "0.5".to_string());
    tr.step_started("s1", "a1", "p1", "t1", None);
    tr.step_finished("s1", true);

    let runtime_control = custom_runtime_control();
    let run_dir = write_run_state_artifacts(
        &resolved,
        &tr,
        Path::new("examples/adl-0.1.yaml"),
        &out_dir,
        50,
        75,
        "success",
        None,
        &[],
        &runtime_control,
        None,
        None,
    )
    .expect("write projected runtime-control artifacts");

    let signals: CognitiveSignalsArtifact = serde_json::from_str(
        &std::fs::read_to_string(run_dir.join("learning/cognitive_signals.v1.json"))
            .expect("read cognitive signals"),
    )
    .expect("parse cognitive signals");
    assert_eq!(signals.instinct.dominant_instinct, "integrity");
    assert_eq!(
        signals.affect.downstream_influence,
        "custom downstream influence"
    );

    let arbitration: CognitiveArbitrationArtifact = serde_json::from_str(
        &std::fs::read_to_string(run_dir.join("learning/cognitive_arbitration.v1.json"))
            .expect("read cognitive arbitration"),
    )
    .expect("parse cognitive arbitration");
    assert_eq!(arbitration.route_selected, "slow");
    assert_eq!(arbitration.route_reason, "custom arbitration reason");

    let fast_slow: FastSlowPathArtifact = serde_json::from_str(
        &std::fs::read_to_string(run_dir.join("learning/fast_slow_path.v1.json"))
            .expect("read fast slow path"),
    )
    .expect("parse fast slow path");
    assert_eq!(fast_slow.runtime_branch_taken, "slow_review_refine_branch");
    assert_eq!(
        fast_slow.path_difference_summary,
        "custom path difference summary"
    );

    let agency: AgencySelectionArtifact = serde_json::from_str(
        &std::fs::read_to_string(run_dir.join("learning/agency_selection.v1.json"))
            .expect("read agency selection"),
    )
    .expect("parse agency selection");
    assert_eq!(agency.selected_candidate_id, "cand-custom-review");
    assert_eq!(agency.candidate_generation_basis, "custom generation basis");

    let bounded_execution: BoundedExecutionArtifact = serde_json::from_str(
        &std::fs::read_to_string(run_dir.join("learning/bounded_execution.v1.json"))
            .expect("read bounded execution"),
    )
    .expect("parse bounded execution");
    assert_eq!(bounded_execution.iteration_count, 2);
    assert_eq!(bounded_execution.iterations[1].stage, "execute");

    let evaluation: EvaluationSignalsArtifact = serde_json::from_str(
        &std::fs::read_to_string(run_dir.join("learning/evaluation_signals.v1.json"))
            .expect("read evaluation signals"),
    )
    .expect("parse evaluation signals");
    assert_eq!(evaluation.termination_reason, "contradiction_detected");
    assert_eq!(
        evaluation.behavior_effect,
        "surface contradiction for bounded follow-up"
    );

    let reframing: run_artifacts::ReframingArtifact = serde_json::from_str(
        &std::fs::read_to_string(run_dir.join("learning/reframing.v1.json"))
            .expect("read reframing artifact"),
    )
    .expect("parse reframing artifact");
    assert_eq!(reframing.frame_adequacy_score, 24);
    assert_eq!(reframing.reframing_trigger, "triggered");
    assert_eq!(reframing.reexecution_choice, "bounded_reframe_and_retry");

    let freedom_gate: FreedomGateArtifact = serde_json::from_str(
        &std::fs::read_to_string(run_dir.join("learning/freedom_gate.v1.json"))
            .expect("read freedom gate artifact"),
    )
    .expect("parse freedom gate artifact");
    assert_eq!(freedom_gate.gate_decision, "escalate");
    assert_eq!(freedom_gate.reason_code, "frame_escalation_required");
    assert!(freedom_gate.commitment_blocked);
    assert_eq!(
        freedom_gate.required_follow_up,
        "escalate_for_judgment_review"
    );
    assert_eq!(freedom_gate.input.candidate_id, "cand-custom-review");

    let convergence: run_artifacts::AeeConvergenceArtifact = serde_json::from_str(
        &std::fs::read_to_string(run_dir.join("control_path/convergence.json"))
            .expect("read convergence artifact"),
    )
    .expect("parse convergence artifact");
    assert_eq!(convergence.convergence_state, "policy_stop");
    assert_eq!(convergence.stop_condition_family, "policy_boundary");
    assert_eq!(convergence.progress_signal, "steady_progress");

    let decisions: run_artifacts::ControlPathDecisionsArtifact = serde_json::from_str(
        &std::fs::read_to_string(run_dir.join("control_path/decisions.json"))
            .expect("read decisions artifact"),
    )
    .expect("parse decisions artifact");
    assert_eq!(decisions.decision_schema_name, "adl.runtime.decision.v1");
    assert_eq!(
        decisions.outcome_class_vocabulary,
        vec![
            "accept".to_string(),
            "reject".to_string(),
            "defer".to_string(),
            "escalate".to_string(),
            "reroute".to_string(),
        ]
    );
    assert_eq!(decisions.surfaces.len(), 3);
    assert_eq!(decisions.decisions.len(), 3);
    assert_eq!(
        decisions.decisions[0].surface_id,
        "delegation_and_routing.route_selection"
    );
    assert_eq!(decisions.decisions[0].outcome_class, "reroute");
    assert_eq!(
        decisions.decisions[2].surface_id,
        "pre_execution_authorization.commitment_gate"
    );
    assert_eq!(decisions.decisions[2].outcome_class, "escalate");
    assert_eq!(
        decisions.decisions[2].downstream_consequence,
        "escalate_for_judgment_review"
    );

    let action_proposals: run_artifacts::ControlPathActionProposalsArtifact = serde_json::from_str(
        &std::fs::read_to_string(run_dir.join("control_path/action_proposals.json"))
            .expect("read action proposals artifact"),
    )
    .expect("parse action proposals artifact");
    assert_eq!(
        action_proposals.proposal_schema_name,
        "adl.runtime.action_proposal.v1"
    );
    assert_eq!(action_proposals.proposals.len(), 1);
    assert!(action_proposals.proposals[0].non_authoritative);
    assert_eq!(action_proposals.proposals[0].kind, "skill_call");
    assert_eq!(
        action_proposals.proposals[0].target.as_deref(),
        Some("candidate.review_and_refine")
    );
    assert!(action_proposals.proposals[0].requires_approval);

    let mediation: run_artifacts::ControlPathActionMediationArtifact = serde_json::from_str(
        &std::fs::read_to_string(run_dir.join("control_path/mediation.json"))
            .expect("read mediation artifact"),
    )
    .expect("parse mediation artifact");
    assert_eq!(
        mediation.authority_boundary,
        "models_propose_runtime_decides_executes"
    );
    assert_eq!(
        mediation.mediation.proposal_id,
        "proposal.selected_candidate"
    );
    assert_eq!(mediation.mediation.decision_id, "decision.commitment_gate");
    assert_eq!(mediation.mediation.runtime_authority, "freedom_gate");
    assert_eq!(mediation.mediation.mediation_outcome, "escalated");
    assert_eq!(
        mediation.mediation.required_follow_up,
        "escalate_for_judgment_review"
    );
    assert!(mediation.mediation.approved_action_or_none.is_none());

    let skill_model: run_artifacts::ControlPathSkillModelArtifact = serde_json::from_str(
        &std::fs::read_to_string(run_dir.join("control_path/skill_model.json"))
            .expect("read skill model artifact"),
    )
    .expect("parse skill model artifact");
    assert_eq!(skill_model.skill_schema_name, "adl.runtime.skill_model.v1");
    assert_eq!(skill_model.selected_execution_unit_kind, "skill_call");
    assert_eq!(skill_model.skill.selection_status, "selected");
    assert_eq!(skill_model.skill.skill_id, "skill.review_and_refine");
    assert_eq!(
        skill_model.skill.output_contract_surfaces,
        vec![
            "control_path/mediation.json".to_string(),
            "control_path/final_result.json".to_string(),
            "logs/trace_v1.json".to_string(),
        ]
    );

    let skill_execution_protocol: run_artifacts::ControlPathSkillExecutionProtocolArtifact =
        serde_json::from_str(
            &std::fs::read_to_string(run_dir.join("control_path/skill_execution_protocol.json"))
                .expect("read skill execution protocol artifact"),
        )
        .expect("parse skill execution protocol artifact");
    assert_eq!(
        skill_execution_protocol.protocol_name,
        "adl.runtime.skill_execution_protocol.v1"
    );
    assert_eq!(
        skill_execution_protocol.invocation.skill_id,
        "skill.review_and_refine"
    );
    assert_eq!(
        skill_execution_protocol.invocation.lifecycle_state,
        "escalated_before_execution"
    );
    assert_eq!(
        skill_execution_protocol.invocation.authorization_decision,
        "escalated"
    );
    assert_eq!(
        skill_execution_protocol.invocation.proposal_id,
        "proposal.selected_candidate"
    );

    let memory_read: MemoryReadArtifact = serde_json::from_str(
        &std::fs::read_to_string(run_dir.join("learning/memory_read.v1.json"))
            .expect("read memory read artifact"),
    )
    .expect("parse memory read artifact");
    assert_eq!(memory_read.read_count, 1);
    assert_eq!(memory_read.query.status_filter, "failed");
    assert_eq!(memory_read.influenced_stage, "reframing_decision");
    assert_eq!(memory_read.entries[0].run_id, "prev-run");

    let memory_write: MemoryWriteArtifact = serde_json::from_str(
        &std::fs::read_to_string(run_dir.join("learning/memory_write.v1.json"))
            .expect("read memory write artifact"),
    )
    .expect("parse memory write artifact");
    assert_eq!(memory_write.entry_id, "mem-entry::wf::runtime-control");
    assert_eq!(
        memory_write.write_reason,
        "record_failure_for_future_reframing_context"
    );
    assert_eq!(memory_write.logical_timestamp, "run:runtime-control");

    let control_path_memory: ControlPathMemoryArtifact = serde_json::from_str(
        &std::fs::read_to_string(run_dir.join("control_path/memory.json"))
            .expect("read control path memory artifact"),
    )
    .expect("parse control path memory artifact");
    assert_eq!(control_path_memory.read.read_count, 1);
    assert_eq!(
        control_path_memory.write.write_reason,
        "record_failure_for_future_reframing_context"
    );

    let control_path_final_result: ControlPathFinalResultArtifact = serde_json::from_str(
        &std::fs::read_to_string(run_dir.join("control_path/final_result.json"))
            .expect("read control path final result artifact"),
    )
    .expect("parse control path final result artifact");
    assert_eq!(control_path_final_result.route_selected, "slow");
    assert_eq!(
        control_path_final_result.selected_candidate,
        "cand-custom-review"
    );
    assert_eq!(control_path_final_result.gate_decision, "escalate");
    assert_eq!(control_path_final_result.final_result, "escalate");

    let control_path_security_review: run_artifacts::ControlPathSecurityReviewArtifact =
        serde_json::from_str(
            &std::fs::read_to_string(run_dir.join("control_path/security_review.json"))
                .expect("read control path security review artifact"),
        )
        .expect("parse control path security review artifact");
    assert_eq!(
        control_path_security_review.posture.declared_posture,
        "hardened_review_first"
    );
    assert_eq!(
        control_path_security_review.threat_model.attacker_pressure,
        "contested"
    );
    assert_eq!(
        control_path_security_review
            .trust_under_adversary
            .trust_state,
        "reduced_until_review"
    );
    assert_eq!(
        control_path_security_review.evidence.security_denied_count,
        0
    );
    assert!(control_path_security_review
        .threat_model
        .reviewer_visible_surfaces
        .contains(&"control_path/freedom_gate.json".to_string()));
    assert_eq!(
        control_path_security_review.evidence.trace_visibility_expectation,
        "approval, rejection, defer, or escalation remains trace-visible before privileged execution"
    );

    let control_path_summary =
        std::fs::read_to_string(run_dir.join("control_path/summary.txt")).expect("read summary");
    assert!(
        control_path_summary.contains(
            "stage_order: signals -> candidate_selection -> arbitration -> execution -> evaluation -> reframing -> memory -> freedom_gate -> final_result"
        ),
        "summary was:\n{control_path_summary}"
    );
    assert!(
        control_path_summary.contains(
            "freedom_gate: decision=escalate reason_code=frame_escalation_required follow_up=escalate_for_judgment_review"
        ),
        "summary was:\n{control_path_summary}"
    );
    assert!(
        control_path_summary.contains(
            "convergence: state=policy_stop stop_condition_family=policy_boundary progress_signal=steady_progress"
        ),
        "summary was:\n{control_path_summary}"
    );
    assert!(
        control_path_summary.contains(
            "decisions: route_selection=reroute reframing=reroute commitment_gate=escalate"
        ),
        "summary was:\n{control_path_summary}"
    );
    assert!(
        control_path_summary.contains(
            "action_proposal: kind=skill_call target=candidate.review_and_refine requires_approval=true"
        ),
        "summary was:\n{control_path_summary}"
    );
    assert!(
        control_path_summary.contains(
            "action_mediation: outcome=escalated authority=freedom_gate follow_up=escalate_for_judgment_review"
        ),
        "summary was:\n{control_path_summary}"
    );
    assert!(
        control_path_summary.contains(
            "skill_model: selection_status=selected skill_id=skill.review_and_refine invocation_kind=skill_call"
        ),
        "summary was:\n{control_path_summary}"
    );
    assert!(
        control_path_summary.contains(
            "skill_execution_protocol: lifecycle_state=escalated_before_execution authorization=escalated trace_expectation=approval, rejection, defer, or escalation remains trace-visible before privileged execution"
        ),
        "summary was:\n{control_path_summary}"
    );
    assert!(
        control_path_summary.contains(
            "security_review: posture=hardened_review_first trust_state=reduced_until_review attacker_pressure=contested"
        ),
        "summary was:\n{control_path_summary}"
    );
    run_artifacts::validate_control_path_artifact_set(&run_dir.join("control_path"))
        .expect("real runtime control-path layout should validate");

    let _ = std::fs::remove_dir_all(run_dir);
    let _ = std::fs::remove_dir_all(out_dir);
}

#[test]
fn derive_runtime_control_state_projects_memory_participation_for_prior_failed_runs() {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time")
        .as_nanos();
    let runs_root = unique_temp_dir("adl-main-runs-memory-participation");
    let _runs_guard = EnvGuard::set("ADL_RUNS_ROOT", &runs_root.to_string_lossy());

    let prior_run_id = format!("prior-memory-{now}-{}", std::process::id());
    let prior_resolved = minimal_resolved_for_artifacts(prior_run_id.clone());
    let prior_out_dir = std::env::temp_dir().join(format!("adl-main-prior-memory-{now}"));
    let mut prior_trace =
        trace::Trace::new(prior_run_id.clone(), "wf".to_string(), "0.86".to_string());
    prior_trace.step_started("s1", "agent", "local", "task", None);
    prior_trace.step_finished("s1", false);
    prior_trace.run_failed("prior failure");
    write_run_state_artifacts(
        &prior_resolved,
        &prior_trace,
        Path::new("examples/adl-0.1.yaml"),
        &prior_out_dir,
        0,
        1,
        "failure",
        None,
        &[],
        &runtime_control_for("failure", &prior_trace),
        None,
        None,
    )
    .expect("write prior failure artifacts");

    let current_trace = trace::Trace::new(
        format!("current-memory-{now}-{}", std::process::id()),
        "wf".to_string(),
        "0.86".to_string(),
    );
    let records = vec![execute::StepExecutionRecord {
        step_id: "s1".to_string(),
        provider_id: "p1".to_string(),
        status: "failure".to_string(),
        attempts: 1,
        output_bytes: 0,
    }];
    let runtime_control =
        execute::derive_runtime_control_state("failure", &records, &current_trace);

    assert_eq!(runtime_control.memory.read.query.workflow_id, "wf");
    assert_eq!(runtime_control.memory.read.query.status_filter, "failed");
    assert_eq!(
        runtime_control.memory.read.influenced_stage,
        "reframing_decision"
    );
    assert_eq!(runtime_control.memory.read.entries.len(), 1);
    assert_eq!(runtime_control.memory.read.entries[0].run_id, prior_run_id);
    assert!(runtime_control
        .memory
        .read
        .influence_summary
        .contains("prior_failure_memory reinforces bounded reframing"));
    assert_eq!(
        runtime_control.memory.write.write_reason,
        "record_failure_for_future_reframing_context"
    );
}

#[test]
fn load_resume_state_rejects_non_paused_status() {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time")
        .as_nanos();
    let run_id = format!("run-nonpaused-{now}-{}", std::process::id());
    let resolved = minimal_resolved_for_artifacts(run_id.clone());
    let out_dir = std::env::temp_dir().join(format!("adl-main-nonpaused-{now}"));
    let runs_root = unique_temp_dir("adl-main-runs-nonpaused");
    let _runs_guard = EnvGuard::set("ADL_RUNS_ROOT", &runs_root.to_string_lossy());

    let tr = trace::Trace::new(run_id, "wf".to_string(), "0.5".to_string());
    let run_dir = write_run_state_artifacts(
        &resolved,
        &tr,
        Path::new("examples/adl-0.1.yaml"),
        &out_dir,
        0,
        1,
        "success",
        None,
        &[],
        &runtime_control_for("success", &tr),
        None,
        None,
    )
    .expect("write non-paused artifacts");
    let err = load_resume_state(&run_dir.join("run.json"), &resolved)
        .expect_err("non-paused run.json should fail for resume");
    assert!(err.to_string().contains("status='paused'"));
    assert!(err.to_string().contains("run_id='"));
    assert!(
        !run_dir.join("pause_state.json").exists(),
        "non-paused runs must not emit pause_state.json"
    );
    let run_status: JsonValue = serde_json::from_str(
        &std::fs::read_to_string(run_dir.join("run_status.json")).expect("read run_status.json"),
    )
    .expect("parse run_status.json");
    assert_eq!(
        run_status.get("persistence_mode"),
        Some(&JsonValue::String("completed_run_record".to_string()))
    );
    assert_eq!(
        run_status.get("cleanup_disposition"),
        Some(&JsonValue::String("no_resume_state_retained".to_string()))
    );
    assert_eq!(
        run_status.get("resume_guard"),
        Some(&JsonValue::String("not_applicable".to_string()))
    );
    let _ = std::fs::remove_dir_all(run_dir);
    let _ = std::fs::remove_dir_all(out_dir);
}

#[test]
fn load_resume_state_rejects_unknown_schema_version() {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time")
        .as_nanos();
    let run_id = format!("run-schema-{now}-{}", std::process::id());
    let resolved = minimal_resolved_for_artifacts(run_id.clone());
    let out_dir = std::env::temp_dir().join(format!("adl-main-schema-{now}"));
    let runs_root = unique_temp_dir("adl-main-runs-schema");
    let _runs_guard = EnvGuard::set("ADL_RUNS_ROOT", &runs_root.to_string_lossy());

    let mut tr = trace::Trace::new(run_id, "wf".to_string(), "0.5".to_string());
    tr.step_started("s1", "a1", "p1", "t1", None);
    tr.step_finished("s1", true);
    let pause = execute::PauseState {
        paused_step_id: "s1".to_string(),
        reason: Some("review".to_string()),
        completed_step_ids: vec!["s1".to_string()],
        remaining_step_ids: vec![],
        saved_state: HashMap::new(),
        completed_outputs: HashMap::new(),
    };
    let run_dir = write_run_state_artifacts(
        &resolved,
        &tr,
        Path::new("examples/adl-0.1.yaml"),
        &out_dir,
        10,
        20,
        "paused",
        Some(&pause),
        &[],
        &runtime_control_for("paused", &tr),
        None,
        None,
    )
    .expect("write run artifacts");

    let run_json_path = run_dir.join("run.json");
    let mut run_json: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&run_json_path).expect("read run.json"))
            .expect("parse run.json");
    run_json["schema_version"] = serde_json::Value::String("run_state.v0".to_string());
    artifacts::atomic_write(
        &run_json_path,
        serde_json::to_vec_pretty(&run_json)
            .expect("serialize modified run.json")
            .as_slice(),
    )
    .expect("rewrite run.json");

    let err = load_resume_state(&run_json_path, &resolved)
        .expect_err("schema mismatch should be rejected");
    assert!(err.to_string().contains("schema_version mismatch"));
    let _ = std::fs::remove_dir_all(run_dir);
    let _ = std::fs::remove_dir_all(out_dir);
}

#[test]
fn load_resume_state_rejects_missing_pause_payload() {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time")
        .as_nanos();
    let run_id = format!("run-missing-pause-{now}-{}", std::process::id());
    let resolved = minimal_resolved_for_artifacts(run_id.clone());
    let out_dir = std::env::temp_dir().join(format!("adl-main-missing-pause-{now}"));
    let runs_root = unique_temp_dir("adl-main-runs-missing-pause");
    let _runs_guard = EnvGuard::set("ADL_RUNS_ROOT", &runs_root.to_string_lossy());

    let tr = trace::Trace::new(run_id, "wf".to_string(), "0.5".to_string());
    let run_dir = write_run_state_artifacts(
        &resolved,
        &tr,
        Path::new("examples/adl-0.1.yaml"),
        &out_dir,
        0,
        1,
        "paused",
        None,
        &[],
        &runtime_control_for("paused", &tr),
        None,
        None,
    )
    .expect("write paused artifacts");

    let err = load_resume_state(&run_dir.join("run.json"), &resolved)
        .expect_err("paused run.json without pause payload should fail");
    assert!(err.to_string().contains("missing pause payload"));
    let _ = std::fs::remove_dir_all(run_dir);
    let _ = std::fs::remove_dir_all(out_dir);
}

#[test]
fn load_resume_state_rejects_workflow_mismatch() {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time")
        .as_nanos();
    let run_id = format!("run-wf-mismatch-{now}-{}", std::process::id());
    let resolved = minimal_resolved_for_artifacts(run_id.clone());
    let out_dir = std::env::temp_dir().join(format!("adl-main-wf-mismatch-{now}"));
    let runs_root = unique_temp_dir("adl-main-runs-wf-mismatch");
    let _runs_guard = EnvGuard::set("ADL_RUNS_ROOT", &runs_root.to_string_lossy());

    let pause = execute::PauseState {
        paused_step_id: "s1".to_string(),
        reason: None,
        completed_step_ids: vec!["s1".to_string()],
        remaining_step_ids: vec![],
        saved_state: HashMap::new(),
        completed_outputs: HashMap::new(),
    };
    let tr = trace::Trace::new(run_id, "wf".to_string(), "0.5".to_string());
    let run_dir = write_run_state_artifacts(
        &resolved,
        &tr,
        Path::new("examples/adl-0.1.yaml"),
        &out_dir,
        0,
        1,
        "paused",
        Some(&pause),
        &[],
        &runtime_control_for("paused", &tr),
        None,
        None,
    )
    .expect("write paused artifacts");

    let mut mismatch = resolved.clone();
    mismatch.workflow_id = "wf-other".to_string();
    let err = load_resume_state(&run_dir.join("run.json"), &mismatch)
        .expect_err("workflow mismatch must fail");
    assert!(err.to_string().contains("workflow_id mismatch"));
    assert!(err.to_string().contains("state='wf'"));
    assert!(err.to_string().contains("current='wf-other'"));
    let _ = std::fs::remove_dir_all(run_dir);
    let _ = std::fs::remove_dir_all(out_dir);
}

#[test]
fn load_resume_state_rejects_version_mismatch() {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time")
        .as_nanos();
    let run_id = format!("run-version-mismatch-{now}-{}", std::process::id());
    let resolved = minimal_resolved_for_artifacts(run_id.clone());
    let out_dir = std::env::temp_dir().join(format!("adl-main-version-mismatch-{now}"));
    let runs_root = unique_temp_dir("adl-main-runs-version-mismatch");
    let _runs_guard = EnvGuard::set("ADL_RUNS_ROOT", &runs_root.to_string_lossy());

    let pause = execute::PauseState {
        paused_step_id: "s1".to_string(),
        reason: None,
        completed_step_ids: vec!["s1".to_string()],
        remaining_step_ids: vec![],
        saved_state: HashMap::new(),
        completed_outputs: HashMap::new(),
    };
    let tr = trace::Trace::new(run_id, "wf".to_string(), "0.5".to_string());
    let run_dir = write_run_state_artifacts(
        &resolved,
        &tr,
        Path::new("examples/adl-0.1.yaml"),
        &out_dir,
        0,
        1,
        "paused",
        Some(&pause),
        &[],
        &runtime_control_for("paused", &tr),
        None,
        None,
    )
    .expect("write paused artifacts");

    let mut mismatch = resolved.clone();
    mismatch.doc.version = "0.6".to_string();
    let err = load_resume_state(&run_dir.join("run.json"), &mismatch)
        .expect_err("version mismatch must fail");
    assert!(err.to_string().contains("version mismatch"));
    assert!(err.to_string().contains("state='0.5'"));
    assert!(err.to_string().contains("current='0.6'"));
    let _ = std::fs::remove_dir_all(run_dir);
    let _ = std::fs::remove_dir_all(out_dir);
}

#[test]
fn load_resume_state_rejects_execution_plan_mismatch() {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time")
        .as_nanos();
    let run_id = format!("run-plan-mismatch-{now}-{}", std::process::id());
    let resolved = minimal_resolved_for_artifacts(run_id.clone());
    let out_dir = std::env::temp_dir().join(format!("adl-main-plan-mismatch-{now}"));
    let runs_root = unique_temp_dir("adl-main-runs-plan-mismatch");
    let _runs_guard = EnvGuard::set("ADL_RUNS_ROOT", &runs_root.to_string_lossy());

    let pause = execute::PauseState {
        paused_step_id: "s1".to_string(),
        reason: None,
        completed_step_ids: vec!["s1".to_string()],
        remaining_step_ids: vec![],
        saved_state: HashMap::new(),
        completed_outputs: HashMap::new(),
    };
    let tr = trace::Trace::new(run_id, "wf".to_string(), "0.5".to_string());
    let run_dir = write_run_state_artifacts(
        &resolved,
        &tr,
        Path::new("examples/adl-0.1.yaml"),
        &out_dir,
        0,
        1,
        "paused",
        Some(&pause),
        &[],
        &runtime_control_for("paused", &tr),
        None,
        None,
    )
    .expect("write paused artifacts");

    let run_json = run_dir.join("run.json");
    let raw = std::fs::read_to_string(&run_json).expect("read run.json");
    let mut value: serde_json::Value = serde_json::from_str(&raw).expect("parse run.json");
    value["execution_plan_hash"] = serde_json::Value::String("tampered-hash".to_string());
    std::fs::write(
        &run_json,
        serde_json::to_vec_pretty(&value).expect("serialize tampered run.json"),
    )
    .expect("write tampered run.json");

    let err = load_resume_state(&run_json, &resolved).expect_err("plan mismatch must fail");
    assert!(err.to_string().contains("execution plan mismatch"));
    assert!(err.to_string().contains("state plan != current plan"));
    let _ = std::fs::remove_dir_all(run_dir);
    let _ = std::fs::remove_dir_all(out_dir);
}
