use super::*;
use crate::cli::run_artifacts;
use crate::cli::run_artifacts::{
    AgencySelectionArtifact, BoundedExecutionArtifact, CognitiveArbitrationArtifact,
    CognitiveSignalsArtifact, ControlPathFinalResultArtifact, ControlPathMemoryArtifact,
    EvaluationSignalsArtifact, FastSlowPathArtifact, FreedomGateArtifact, MemoryReadArtifact,
    MemoryWriteArtifact,
};

#[test]
fn select_open_artifact_returns_none_without_html() {
    let artifacts = vec![PathBuf::from("out/one.txt"), PathBuf::from("out/two.md")];
    assert!(select_open_artifact(&artifacts).is_none());
}

#[test]
fn run_artifacts_root_points_to_repo_adl_runs() {
    let _runs_guard = EnvGuard::set("ADL_RUNS_ROOT", "");
    let root = artifacts::runs_root().expect("run artifacts root");
    let s = root.to_string_lossy();
    assert!(s.ends_with(".adl/runs"), "unexpected path: {s}");
}

#[test]
fn enforce_signature_policy_skips_when_not_running_or_not_v0_5() {
    let mk_doc = |version: &str| adl::AdlDoc {
        version: version.to_string(),
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

    enforce_signature_policy(&mk_doc("0.5"), false, false).expect("do_run=false should skip");
    enforce_signature_policy(&mk_doc("0.4"), true, false).expect("v0.4 should skip");
    enforce_signature_policy(&mk_doc("0.5"), true, true).expect("allow_unsigned should skip");
}

pub(super) fn minimal_resolved_for_artifacts(run_id: String) -> resolve::AdlResolved {
    resolve::AdlResolved {
        run_id,
        workflow_id: "wf".to_string(),
        steps: vec![resolve::ResolvedStep {
            id: "s1".to_string(),
            agent: Some("a1".to_string()),
            provider: Some("p1".to_string()),
            placement: None,
            task: Some("t1".to_string()),
            call: None,
            with: HashMap::new(),
            as_ns: None,
            delegation: None,
            prompt: Some(adl::PromptSpec {
                user: Some("u".to_string()),
                ..Default::default()
            }),
            inputs: HashMap::new(),
            guards: vec![],
            save_as: Some("s1_out".to_string()),
            write_to: Some("out/s1.txt".to_string()),
            on_error: None,
            retry: None,
        }],
        execution_plan: ::adl::execution_plan::ExecutionPlan {
            workflow_kind: adl::WorkflowKind::Sequential,
            nodes: vec![::adl::execution_plan::ExecutionNode {
                step_id: "s1".to_string(),
                depends_on: vec![],
                save_as: Some("s1_out".to_string()),
                delegation: None,
            }],
        },
        doc: adl::AdlDoc {
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
                name: Some("run".to_string()),
                created_at: None,
                defaults: adl::RunDefaults::default(),
                workflow_ref: None,
                workflow: Some(adl::WorkflowSpec {
                    id: Some("wf".to_string()),
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
        },
    }
}

fn runtime_control_for(status: &str, tr: &trace::Trace) -> execute::RuntimeControlState {
    execute::derive_runtime_control_state(status, &[], tr)
}

#[test]
fn derive_runtime_control_state_triggers_reframing_on_failure() {
    let tr = trace::Trace::new(
        "runtime-reframing-failure".to_string(),
        "wf".to_string(),
        "0.86".to_string(),
    );
    let records = vec![execute::StepExecutionRecord {
        step_id: "s1".to_string(),
        provider_id: "p1".to_string(),
        status: "failure".to_string(),
        attempts: 2,
        output_bytes: 0,
    }];
    let runtime_control = execute::derive_runtime_control_state("failure", &records, &tr);
    assert_eq!(
        runtime_control.evaluation.next_control_action,
        "handoff_to_reframing"
    );
    assert_eq!(runtime_control.reframing.reframing_trigger, "triggered");
    assert_eq!(
        runtime_control.reframing.reexecution_choice,
        "bounded_reframe_and_retry"
    );
    assert_eq!(runtime_control.freedom_gate.gate_decision, "defer");
    assert_eq!(runtime_control.freedom_gate.reason_code, "frame_inadequate");
    assert!(
        runtime_control.reframing.frame_adequacy_score < 50,
        "failure should lower the frame adequacy score"
    );
}

#[test]
fn derive_runtime_control_state_retains_current_frame_on_success() {
    let tr = trace::Trace::new(
        "runtime-reframing-success".to_string(),
        "wf".to_string(),
        "0.86".to_string(),
    );
    let runtime_control = runtime_control_for("success", &tr);
    assert_eq!(
        runtime_control.evaluation.next_control_action,
        "complete_run"
    );
    assert_eq!(runtime_control.reframing.reframing_trigger, "not_triggered");
    assert_eq!(runtime_control.reframing.new_frame, "retain_current_frame");
    assert_eq!(runtime_control.reframing.post_reframe_state, "complete_run");
    assert_eq!(runtime_control.freedom_gate.gate_decision, "allow");
    assert_eq!(runtime_control.freedom_gate.reason_code, "policy_allowed");
}

fn custom_runtime_control() -> execute::RuntimeControlState {
    execute::RuntimeControlState {
        signals: execute::CognitiveSignalsState {
            dominant_instinct: "integrity".to_string(),
            completion_pressure: "guarded".to_string(),
            integrity_bias: "high".to_string(),
            curiosity_bias: "bounded".to_string(),
            candidate_selection_bias: "prefer lower-risk constrained candidates".to_string(),
            urgency_level: "moderate".to_string(),
            salience_level: "high".to_string(),
            persistence_pressure: "stabilize_then_retry".to_string(),
            confidence_shift: "reduced".to_string(),
            downstream_influence: "custom downstream influence".to_string(),
        },
        arbitration: execute::CognitiveArbitrationState {
            route_selected: "slow".to_string(),
            reasoning_mode: "review_heavy".to_string(),
            confidence: "guarded".to_string(),
            risk_class: "high".to_string(),
            applied_constraints: vec![
                "security_denial_present".to_string(),
                "failure_recovery_bias".to_string(),
            ],
            cost_latency_assumption:
                "spend bounded additional cognition when failure or policy risk is present"
                    .to_string(),
            route_reason: "custom arbitration reason".to_string(),
        },
        fast_slow: execute::FastSlowPathState {
            selected_path: "slow_path".to_string(),
            path_family: "slow".to_string(),
            runtime_branch_taken: "slow_review_refine_branch".to_string(),
            handoff_state: "review_handoff".to_string(),
            candidate_strategy: "validate, refine, or veto the current bounded candidate"
                .to_string(),
            review_depth: "verification_required".to_string(),
            execution_profile: "review_and_refine_before_execution".to_string(),
            termination_expectation: "terminate_after_bounded_review_cycle_or_policy_block"
                .to_string(),
            path_difference_summary: "custom path difference summary".to_string(),
        },
        agency: execute::AgencySelectionState {
            candidate_generation_basis: "custom generation basis".to_string(),
            selection_mode: "slow_candidate_comparison".to_string(),
            candidate_set: vec![execute::AgencyCandidateRecord {
                candidate_id: "cand-custom-review".to_string(),
                candidate_kind: "review_and_refine".to_string(),
                bounded_action: "review and refine the candidate".to_string(),
                review_requirement: "verification_required".to_string(),
                execution_priority: 1,
                rationale: "custom rationale".to_string(),
            }],
            selected_candidate_id: "cand-custom-review".to_string(),
            selected_candidate_kind: "review_and_refine".to_string(),
            selected_candidate_action: "review and refine the candidate".to_string(),
            selected_candidate_reason: "custom selected candidate reason".to_string(),
        },
        bounded_execution: execute::BoundedExecutionState {
            execution_status: "completed".to_string(),
            continuation_state: "bounded_review_complete".to_string(),
            provisional_termination_state: "ready_for_evaluation".to_string(),
            iterations: vec![
                execute::BoundedExecutionIteration {
                    iteration_index: 1,
                    stage: "review".to_string(),
                    action: "review the candidate".to_string(),
                    outcome: "bounded_review_pass_complete".to_string(),
                },
                execute::BoundedExecutionIteration {
                    iteration_index: 2,
                    stage: "execute".to_string(),
                    action: "execute the reviewed candidate".to_string(),
                    outcome: "bounded_reviewed_execution_complete".to_string(),
                },
            ],
        },
        evaluation: execute::EvaluationControlState {
            progress_signal: "steady_progress".to_string(),
            contradiction_signal: "present".to_string(),
            failure_signal: "none".to_string(),
            termination_reason: "contradiction_detected".to_string(),
            behavior_effect: "surface contradiction for bounded follow-up".to_string(),
            next_control_action: "handoff_to_reframing".to_string(),
        },
        reframing: execute::ReframingControlState {
            frame_adequacy_score: 24,
            reframing_trigger: "triggered".to_string(),
            reframing_reason: "contradiction_detected_after_bounded_execution".to_string(),
            prior_frame: "review_and_refine_under_current_frame".to_string(),
            new_frame: "diagnose_and_restructure_before_retry".to_string(),
            reexecution_choice: "bounded_reframe_and_retry".to_string(),
            post_reframe_state: "ready_for_reframed_execution".to_string(),
        },
        freedom_gate: execute::FreedomGateState {
            input: execute::FreedomGateInputState {
                candidate_id: "cand-custom-review".to_string(),
                candidate_action: "review and refine the candidate".to_string(),
                candidate_rationale: "custom selected candidate reason".to_string(),
                risk_class: "high".to_string(),
                policy_context: execute::FreedomGatePolicyContextState {
                    route_selected: "slow".to_string(),
                    selected_candidate_kind: "review_and_refine".to_string(),
                    requires_review: false,
                    policy_blocked: false,
                },
                evaluation_signals: execute::FreedomGateEvaluationSignalsState {
                    progress_signal: "steady_progress".to_string(),
                    contradiction_signal: "present".to_string(),
                    failure_signal: "none".to_string(),
                    termination_reason: "contradiction_detected".to_string(),
                },
                frame_state: "ready_for_reframed_execution".to_string(),
            },
            gate_decision: "defer".to_string(),
            reason_code: "frame_inadequate".to_string(),
            decision_reason:
                "frame state requires bounded reframing before commitment can be allowed"
                    .to_string(),
            selected_action_or_none: None,
            commitment_blocked: true,
        },
        memory: execute::MemoryParticipationState {
            read: execute::MemoryReadState {
                query: execute::MemoryQueryState {
                    workflow_id: "wf".to_string(),
                    status_filter: "failed".to_string(),
                    limit: 3,
                    source: "repo_local_runs_root".to_string(),
                },
                entries: vec![execute::MemoryReadEntry {
                    memory_entry_id: "prev-run::wf".to_string(),
                    run_id: "prev-run".to_string(),
                    workflow_id: "wf".to_string(),
                    summary: "prior failure memory".to_string(),
                    tags: vec![
                        "status:failed".to_string(),
                        "workflow:wf".to_string(),
                    ],
                    source: "indexed_run_artifacts".to_string(),
                }],
                retrieval_order: "workflow_id_then_run_id_ascending".to_string(),
                influence_summary:
                    "prior_failure_memory reinforces bounded reframing for route=slow selected_candidate=cand-custom-review"
                        .to_string(),
                influenced_stage: "reframing_decision".to_string(),
            },
            write: execute::MemoryWriteState {
                entry_id: "mem-entry::wf::runtime-control".to_string(),
                content: "workflow=wf status=failure next_control_action=handoff_to_reframing influence=prior_failure_memory reinforces bounded reframing for route=slow selected_candidate=cand-custom-review".to_string(),
                tags: vec![
                    "action:handoff_to_reframing".to_string(),
                    "candidate:review_and_refine".to_string(),
                    "status:failure".to_string(),
                    "workflow:wf".to_string(),
                ],
                logical_timestamp: "run:runtime-control".to_string(),
                write_reason: "record_failure_for_future_reframing_context".to_string(),
                source: "runtime_control_projection".to_string(),
            },
        },
    }
}

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
    tr.step_started("s1", "a1", "p1", "t1", None);
    tr.step_finished("s1", true);

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

    let resume =
        load_resume_state(&run_dir.join("run.json"), &resolved).expect("load resume state");
    assert!(resume.completed_step_ids.contains("s1"));
    assert_eq!(resume.saved_state.get("k").map(String::as_str), Some("v"));
    assert!(
        run_dir.join("pause_state.json").exists(),
        "paused runs must persist pause_state.json"
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
    let prior_trace = trace::Trace::new(prior_run_id.clone(), "wf".to_string(), "0.86".to_string());
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
        failure_taxonomy::category_for_code("policy_denied"),
        failure_taxonomy::POLICY_DENIED
    );
    assert_eq!(
        failure_taxonomy::category_for_code("verification_failed"),
        failure_taxonomy::VERIFICATION_FAILED
    );
    assert_eq!(
        failure_taxonomy::category_for_code("replay_invariant_violation"),
        failure_taxonomy::REPLAY_INVARIANT_VIOLATION
    );
    assert_eq!(
        failure_taxonomy::category_for_code("provider_error"),
        failure_taxonomy::TOOL_FAILURE
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
