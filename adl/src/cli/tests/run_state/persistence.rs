use super::*;
use ::adl::trace_schema_v1::{
    validate_trace_event_envelope_v1, TraceEventEnvelopeV1, TraceEventTypeV1,
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
    assert_eq!(freedom_gate.gate_decision, "defer");
    assert_eq!(freedom_gate.reason_code, "frame_inadequate");
    assert!(freedom_gate.commitment_blocked);
    assert_eq!(freedom_gate.input.candidate_id, "cand-custom-review");

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
    assert_eq!(control_path_final_result.gate_decision, "defer");
    assert_eq!(control_path_final_result.final_result, "defer");

    let control_path_summary =
        std::fs::read_to_string(run_dir.join("control_path/summary.txt")).expect("read summary");
    assert!(
        control_path_summary.contains(
            "stage_order: signals -> candidate_selection -> arbitration -> execution -> evaluation -> reframing -> memory -> freedom_gate -> final_result"
        ),
        "summary was:\n{control_path_summary}"
    );
    assert!(
        control_path_summary.contains("freedom_gate: decision=defer"),
        "summary was:\n{control_path_summary}"
    );

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
