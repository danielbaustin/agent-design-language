use std::path::{Path, PathBuf};

use anyhow::{anyhow, Context, Result};

use crate::execute;
use crate::freedom_gate;
use crate::godel;
use crate::prompt;
use crate::trace::Trace;

pub const DEMO_A_SAY_MCP: &str = "demo-a-say-mcp";
pub const DEMO_B_ONE_COMMAND: &str = "demo-b-one-command";
pub const DEMO_C_GODEL_RUNTIME: &str = "demo-c-godel-runtime";
pub const DEMO_D_GODEL_OBSMEM_LOOP: &str = "demo-d-godel-obsmem-loop";
pub const DEMO_E_MULTI_AGENT_CARD_PIPELINE: &str = "demo-e-multi-agent-card-pipeline";
pub const DEMO_F_OBSMEM_RETRIEVAL: &str = "demo-f-obsmem-retrieval";
pub const DEMO_G_V086_CONTROL_PATH: &str = "demo-g-v086-control-path";

#[derive(Debug, Clone)]
pub struct DemoResult {
    pub run_id: String,
    pub artifacts: Vec<PathBuf>,
    pub trace: Trace,
}

pub fn known_demo(name: &str) -> bool {
    matches!(
        name,
        DEMO_A_SAY_MCP
            | DEMO_B_ONE_COMMAND
            | DEMO_C_GODEL_RUNTIME
            | DEMO_D_GODEL_OBSMEM_LOOP
            | DEMO_E_MULTI_AGENT_CARD_PIPELINE
            | DEMO_F_OBSMEM_RETRIEVAL
            | DEMO_G_V086_CONTROL_PATH
    )
}

pub fn run_demo(name: &str, out_dir: &Path) -> Result<DemoResult> {
    if !known_demo(name) {
        return Err(anyhow!(
            "unknown demo '{}'; available demos: {}, {}, {}, {}, {}, {}, {}",
            name,
            DEMO_A_SAY_MCP,
            DEMO_B_ONE_COMMAND,
            DEMO_C_GODEL_RUNTIME,
            DEMO_D_GODEL_OBSMEM_LOOP,
            DEMO_E_MULTI_AGENT_CARD_PIPELINE,
            DEMO_F_OBSMEM_RETRIEVAL,
            DEMO_G_V086_CONTROL_PATH
        ));
    }

    std::fs::create_dir_all(out_dir)
        .with_context(|| format!("failed to create demo output dir '{}'", out_dir.display()))?;

    let mut trace = Trace::new(name, "demo-workflow", "0.3");
    let mut artifacts = Vec::new();

    let steps = steps_for(name);

    for (step_id, text) in steps.iter() {
        trace.step_started(step_id, "coordinator", "demo-local", "artifact-task", None);
        trace.prompt_assembled(step_id, &prompt::hash_prompt(text));
        match name {
            DEMO_A_SAY_MCP => match *step_id {
                "brief" => {
                    let path = write_file(out_dir, "design.md", DESIGN_MD)?;
                    artifacts.push(path);
                }
                "scaffold" => {
                    artifacts.push(write_file(out_dir, "Cargo.toml", CARGO_TOML)?);
                    artifacts.push(write_file(out_dir, "README.md", README_MD)?);
                    artifacts.push(write_file(out_dir, "src/lib.rs", SRC_LIB_RS)?);
                    artifacts.push(write_file(out_dir, "src/main.rs", SRC_MAIN_RS)?);
                    artifacts.push(write_file(out_dir, "tests/say_server_tests.rs", TESTS_RS)?);
                }
                "coverage" => {
                    artifacts.push(write_file(out_dir, "coverage.txt", COVERAGE_TXT)?);
                }
                "game" => {
                    artifacts.push(write_file(out_dir, "index.html", GAME_HTML)?);
                }
                _ => {}
            },
            DEMO_B_ONE_COMMAND => match *step_id {
                "plan" => {
                    artifacts.push(write_file(out_dir, "design.md", DEMO_B_DESIGN_MD)?);
                    artifacts.push(write_file(out_dir, "README.md", DEMO_B_README_MD)?);
                }
                "build" => {
                    let html = generate_rps_game_html();
                    artifacts.push(write_file(out_dir, "index.html", &html)?);
                }
                "verify" => {
                    artifacts.push(write_file(out_dir, "coverage.txt", DEMO_B_COVERAGE_TXT)?);
                }
                _ => {}
            },
            DEMO_C_GODEL_RUNTIME => match *step_id {
                "load" => {
                    let repo_root = godel::repo_root_from_manifest()?;
                    let status = godel::load_v08_surface_status(&repo_root)?;
                    let bytes = serde_json::to_vec_pretty(&status)?;
                    artifacts.push(write_file(
                        out_dir,
                        "godel_runtime_surface_status.json",
                        std::str::from_utf8(&bytes).context("serialize runtime status")?,
                    )?);
                }
                "verify" => {
                    artifacts.push(write_file(
                        out_dir,
                        "verification.txt",
                        "status: pass\nchecks: deterministic-stage-order, cross-artifact-links\n",
                    )?);
                }
                "emit" => {
                    artifacts.push(write_file(out_dir, "README.md", DEMO_C_README_MD)?);
                }
                _ => {}
            },
            DEMO_D_GODEL_OBSMEM_LOOP => match *step_id {
                "failure" => {
                    artifacts.push(write_file(
                        out_dir,
                        "failure_signal.json",
                        DEMO_D_FAILURE_SIGNAL_JSON,
                    )?);
                }
                "run" => {
                    let run = run_godel_stage_loop_demo(out_dir)?;
                    artifacts.extend(run);
                }
                "summarize" => {
                    artifacts.push(write_file(out_dir, "README.md", DEMO_D_README_MD)?);
                }
                _ => {}
            },
            DEMO_E_MULTI_AGENT_CARD_PIPELINE => match *step_id {
                "writer" => {
                    artifacts.push(write_file(
                        out_dir,
                        "pipeline/input_card.md",
                        DEMO_E_INPUT_CARD_MD,
                    )?);
                    artifacts.push(write_file(
                        out_dir,
                        "pipeline/01_writer.md",
                        DEMO_E_WRITER_MD,
                    )?);
                }
                "editor" => {
                    artifacts.push(write_file(
                        out_dir,
                        "pipeline/02_editor.md",
                        DEMO_E_EDITOR_MD,
                    )?);
                }
                "copyeditor" => {
                    artifacts.push(write_file(
                        out_dir,
                        "pipeline/03_copyeditor.md",
                        DEMO_E_COPYEDITOR_MD,
                    )?);
                }
                "publisher" => {
                    let manifest = build_card_pipeline_manifest();
                    artifacts.push(write_file(
                        out_dir,
                        "pipeline/pipeline_manifest.json",
                        &serde_json::to_string_pretty(&manifest)?,
                    )?);
                    artifacts.push(write_file(out_dir, "README.md", DEMO_E_README_MD)?);
                }
                _ => {}
            },
            DEMO_F_OBSMEM_RETRIEVAL => match *step_id {
                "seed" => {
                    let seeded = seed_obsmem_retrieval_demo(out_dir)?;
                    artifacts.extend(seeded);
                }
                "query" => {
                    let queried = query_obsmem_retrieval_demo(out_dir)?;
                    artifacts.extend(queried);
                }
                "emit" => {
                    artifacts.push(write_file(out_dir, "README.md", DEMO_F_README_MD)?);
                }
                _ => {}
            },
            DEMO_G_V086_CONTROL_PATH => match *step_id {
                "integrate" => {
                    artifacts.extend(write_v086_control_path_demo(out_dir)?);
                }
                "summarize" => {
                    artifacts.push(write_file(out_dir, "README.md", DEMO_G_README_MD)?);
                }
                _ => {}
            },
            _ => {}
        }
        trace.step_finished(step_id, true);
    }

    trace.run_finished(true);
    artifacts.push(write_trace_jsonl(out_dir, &trace)?);

    Ok(DemoResult {
        run_id: name.to_string(),
        artifacts,
        trace,
    })
}

pub fn plan_steps(name: &str) -> &'static [&'static str] {
    match name {
        DEMO_A_SAY_MCP => &["brief", "scaffold", "coverage", "game"],
        DEMO_B_ONE_COMMAND => &["plan", "build", "verify"],
        DEMO_C_GODEL_RUNTIME => &["load", "verify", "emit"],
        DEMO_D_GODEL_OBSMEM_LOOP => &["failure", "run", "summarize"],
        DEMO_E_MULTI_AGENT_CARD_PIPELINE => &["writer", "editor", "copyeditor", "publisher"],
        DEMO_F_OBSMEM_RETRIEVAL => &["seed", "query", "emit"],
        DEMO_G_V086_CONTROL_PATH => &["integrate", "summarize"],
        _ => &[],
    }
}

fn steps_for(name: &str) -> &'static [(&'static str, &'static str)] {
    match name {
        DEMO_A_SAY_MCP => &[
            ("brief", "Write design and interface specification"),
            ("scaffold", "Create MCP say server module and tests"),
            ("coverage", "Emit pragmatic coverage report"),
            ("game", "Create sample HTML artifact"),
        ],
        DEMO_B_ONE_COMMAND => &[
            ("plan", "Plan deterministic one-command local demo"),
            ("build", "Generate HTML artifact with quiet UX"),
            ("verify", "Emit trace and coverage summary"),
        ],
        DEMO_C_GODEL_RUNTIME => &[
            ("load", "Load canonical v0.8 Gödel schema examples"),
            (
                "verify",
                "Validate deterministic stage order and cross-links",
            ),
            ("emit", "Write runtime surface status artifact"),
        ],
        DEMO_D_GODEL_OBSMEM_LOOP => &[
            ("failure", "Seed deterministic failure signal"),
            (
                "run",
                "Execute bounded Gödel stage loop and persist ObsMem artifacts",
            ),
            ("summarize", "Emit deterministic demo summary"),
        ],
        DEMO_E_MULTI_AGENT_CARD_PIPELINE => &[
            ("writer", "Create writer stage card artifact"),
            ("editor", "Create editor stage artifact"),
            ("copyeditor", "Create copyeditor stage artifact"),
            ("publisher", "Emit publish-ready artifact and manifest"),
        ],
        DEMO_F_OBSMEM_RETRIEVAL => &[
            ("seed", "Persist deterministic experiment/index entries"),
            ("query", "Run deterministic ObsMem retrieval query"),
            ("emit", "Emit retrieval summary"),
        ],
        DEMO_G_V086_CONTROL_PATH => &[
            (
                "integrate",
                "Emit the canonical bounded cognitive path proof surfaces",
            ),
            ("summarize", "Emit deterministic control-path summary"),
        ],
        _ => &[],
    }
}

fn custom_v086_control_path_runtime() -> execute::RuntimeControlState {
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
            downstream_influence: "integrated control path demo".to_string(),
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
            route_reason:
                "integrated path favors slow route when contradiction and policy risk are present"
                    .to_string(),
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
            path_difference_summary:
                "slow path keeps the run review-first and routes contradiction into reframing"
                    .to_string(),
        },
        agency: execute::AgencySelectionState {
            candidate_generation_basis: "integrated-control-path-scenario".to_string(),
            selection_mode: "slow_candidate_comparison".to_string(),
            candidate_set: vec![execute::AgencyCandidateRecord {
                candidate_id: "cand-custom-review".to_string(),
                candidate_kind: "review_and_refine".to_string(),
                bounded_action: "review and refine the candidate".to_string(),
                review_requirement: "verification_required".to_string(),
                execution_priority: 1,
                rationale: "selected for bounded review-heavy remediation".to_string(),
            }],
            selected_candidate_id: "cand-custom-review".to_string(),
            selected_candidate_kind: "review_and_refine".to_string(),
            selected_candidate_action: "review and refine the candidate".to_string(),
            selected_candidate_reason: "candidate preserves bounded progress under contradiction"
                .to_string(),
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
                candidate_rationale: "candidate preserves bounded progress under contradiction"
                    .to_string(),
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
                    tags: vec!["status:failed".to_string(), "workflow:wf".to_string()],
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

pub fn write_v086_control_path_demo(out_dir: &Path) -> Result<Vec<PathBuf>> {
    let runtime = custom_v086_control_path_runtime();
    let mut artifacts = Vec::new();
    let generated_from = serde_json::json!({
        "artifact_model_version": 1,
        "run_summary_version": 1,
        "suggestions_version": 1,
        "scores_version": 1
    });
    let signals = serde_json::json!({
        "cognitive_signals_version": 1,
        "run_id": DEMO_G_V086_CONTROL_PATH,
        "generated_from": generated_from.clone(),
        "instinct": {
            "instinct_profile_id": "instinct-001",
            "dominant_instinct": runtime.signals.dominant_instinct,
            "completion_pressure": runtime.signals.completion_pressure,
            "integrity_bias": runtime.signals.integrity_bias,
            "curiosity_bias": runtime.signals.curiosity_bias,
            "candidate_selection_bias": runtime.signals.candidate_selection_bias,
            "deterministic_update_rule": "demo deterministic signal projection"
        },
        "affect": {
            "affect_state_id": "signal-affect-001",
            "urgency_level": runtime.signals.urgency_level,
            "salience_level": runtime.signals.salience_level,
            "persistence_pressure": runtime.signals.persistence_pressure,
            "confidence_shift": runtime.signals.confidence_shift,
            "downstream_influence": runtime.signals.downstream_influence,
            "deterministic_update_rule": "demo deterministic affect projection"
        }
    });
    let candidate_selection = serde_json::json!({
        "agency_selection_version": 1,
        "run_id": DEMO_G_V086_CONTROL_PATH,
        "generated_from": generated_from.clone(),
        "candidate_generation_basis": runtime.agency.candidate_generation_basis,
        "selection_mode": runtime.agency.selection_mode,
        "candidate_set": runtime.agency.candidate_set,
        "selected_candidate_id": runtime.agency.selected_candidate_id,
        "selected_candidate_reason": runtime.agency.selected_candidate_reason,
        "deterministic_selection_rule": "demo deterministic candidate selection"
    });
    let arbitration = serde_json::json!({
        "cognitive_arbitration_version": 1,
        "run_id": DEMO_G_V086_CONTROL_PATH,
        "generated_from": generated_from.clone(),
        "route_selected": runtime.arbitration.route_selected,
        "reasoning_mode": runtime.arbitration.reasoning_mode,
        "confidence": runtime.arbitration.confidence,
        "risk_class": runtime.arbitration.risk_class,
        "applied_constraints": runtime.arbitration.applied_constraints,
        "cost_latency_assumption": runtime.arbitration.cost_latency_assumption,
        "route_reason": runtime.arbitration.route_reason,
        "deterministic_selection_rule": "demo deterministic arbitration"
    });
    let execution_iterations = serde_json::json!({
        "bounded_execution_version": 1,
        "run_id": DEMO_G_V086_CONTROL_PATH,
        "generated_from": generated_from.clone(),
        "selected_candidate_id": runtime.agency.selected_candidate_id,
        "selected_path": runtime.fast_slow.selected_path,
        "execution_status": runtime.bounded_execution.execution_status,
        "continuation_state": runtime.bounded_execution.continuation_state,
        "provisional_termination_state": runtime.bounded_execution.provisional_termination_state,
        "iteration_count": runtime.bounded_execution.iterations.len(),
        "iterations": runtime.bounded_execution.iterations,
        "deterministic_execution_rule": "demo deterministic bounded execution"
    });
    let evaluation = serde_json::json!({
        "evaluation_signals_version": 1,
        "run_id": DEMO_G_V086_CONTROL_PATH,
        "generated_from": generated_from.clone(),
        "selected_candidate_id": runtime.agency.selected_candidate_id,
        "selected_path": runtime.fast_slow.selected_path,
        "progress_signal": runtime.evaluation.progress_signal,
        "contradiction_signal": runtime.evaluation.contradiction_signal,
        "failure_signal": runtime.evaluation.failure_signal,
        "termination_reason": runtime.evaluation.termination_reason,
        "behavior_effect": runtime.evaluation.behavior_effect,
        "next_control_action": runtime.evaluation.next_control_action,
        "deterministic_evaluation_rule": "demo deterministic evaluation"
    });
    let reframing = serde_json::json!({
        "reframing_version": 1,
        "run_id": DEMO_G_V086_CONTROL_PATH,
        "generated_from": generated_from.clone(),
        "selected_candidate_id": runtime.agency.selected_candidate_id,
        "selected_path": runtime.fast_slow.selected_path,
        "frame_adequacy_score": runtime.reframing.frame_adequacy_score,
        "reframing_trigger": runtime.reframing.reframing_trigger,
        "reframing_reason": runtime.reframing.reframing_reason,
        "prior_frame": runtime.reframing.prior_frame,
        "new_frame": runtime.reframing.new_frame,
        "reexecution_choice": runtime.reframing.reexecution_choice,
        "post_reframe_state": runtime.reframing.post_reframe_state,
        "deterministic_reframing_rule": "demo deterministic reframing"
    });
    let memory = serde_json::json!({
        "control_path_memory_version": 1,
        "run_id": DEMO_G_V086_CONTROL_PATH,
        "generated_from": generated_from.clone(),
        "read": {
            "memory_read_version": 1,
            "run_id": DEMO_G_V086_CONTROL_PATH,
            "generated_from": generated_from.clone(),
            "query": runtime.memory.read.query,
            "read_count": runtime.memory.read.entries.len(),
            "entries": runtime.memory.read.entries,
            "retrieval_order": runtime.memory.read.retrieval_order,
            "influence_summary": runtime.memory.read.influence_summary,
            "influenced_stage": runtime.memory.read.influenced_stage,
            "deterministic_read_rule": "demo deterministic memory read"
        },
        "write": {
            "memory_write_version": 1,
            "run_id": DEMO_G_V086_CONTROL_PATH,
            "generated_from": generated_from.clone(),
            "entry_id": runtime.memory.write.entry_id,
            "content": runtime.memory.write.content,
            "tags": runtime.memory.write.tags,
            "logical_timestamp": runtime.memory.write.logical_timestamp,
            "write_reason": runtime.memory.write.write_reason,
            "source": runtime.memory.write.source,
            "deterministic_write_rule": "demo deterministic memory write"
        }
    });
    let freedom_gate = serde_json::json!({
        "freedom_gate_version": 1,
        "run_id": DEMO_G_V086_CONTROL_PATH,
        "generated_from": generated_from.clone(),
        "input": runtime.freedom_gate.input,
        "gate_decision": runtime.freedom_gate.gate_decision,
        "reason_code": runtime.freedom_gate.reason_code,
        "decision_reason": runtime.freedom_gate.decision_reason,
        "selected_action_or_none": runtime.freedom_gate.selected_action_or_none,
        "commitment_blocked": runtime.freedom_gate.commitment_blocked,
        "deterministic_gate_rule": "demo deterministic freedom gate"
    });
    let final_result = serde_json::json!({
        "control_path_final_result_version": 1,
        "run_id": DEMO_G_V086_CONTROL_PATH,
        "route_selected": "slow",
        "selected_candidate": "cand-custom-review",
        "termination_reason": "contradiction_detected",
        "gate_decision": "defer",
        "final_result": "defer",
        "commitment_blocked": true,
        "next_control_action": "handoff_to_reframing",
        "stage_order": [
            "signals",
            "candidate_selection",
            "arbitration",
            "execution",
            "evaluation",
            "reframing",
            "memory",
            "freedom_gate",
            "final_result"
        ]
    });
    let summary = [
        "v0.86 canonical bounded cognitive path summary",
        "run_id: demo-g-v086-control-path",
        "stage_order: signals -> candidate_selection -> arbitration -> execution -> evaluation -> reframing -> memory -> freedom_gate -> final_result",
        "signals: instinct=integrity completion_pressure=guarded",
        "candidate_selection: candidate_id=cand-custom-review rationale=custom selected candidate reason",
        "arbitration: route=slow reasoning_mode=review_heavy",
        "execution: status=completed iterations=2",
        "evaluation: termination_reason=contradiction_detected next_control_action=handoff_to_reframing",
        "reframing: trigger=triggered choice=bounded_reframe_and_retry",
        "memory: read_count=1 influenced_stage=reframing write_reason=record_failure_for_future_reframing_context",
        "freedom_gate: decision=defer reason_code=frame_inadequate commitment_blocked=true",
        "final_result: defer",
    ]
    .join("\n");

    artifacts.push(write_file(
        out_dir,
        "signals.json",
        &serde_json::to_string_pretty(&signals)?,
    )?);
    artifacts.push(write_file(
        out_dir,
        "candidate_selection.json",
        &serde_json::to_string_pretty(&candidate_selection)?,
    )?);
    artifacts.push(write_file(
        out_dir,
        "arbitration.json",
        &serde_json::to_string_pretty(&arbitration)?,
    )?);
    artifacts.push(write_file(
        out_dir,
        "execution_iterations.json",
        &serde_json::to_string_pretty(&execution_iterations)?,
    )?);
    artifacts.push(write_file(
        out_dir,
        "evaluation.json",
        &serde_json::to_string_pretty(&evaluation)?,
    )?);
    artifacts.push(write_file(
        out_dir,
        "reframing.json",
        &serde_json::to_string_pretty(&reframing)?,
    )?);
    artifacts.push(write_file(
        out_dir,
        "memory.json",
        &serde_json::to_string_pretty(&memory)?,
    )?);
    artifacts.push(write_file(
        out_dir,
        "freedom_gate.json",
        &serde_json::to_string_pretty(&freedom_gate)?,
    )?);
    artifacts.push(write_file(
        out_dir,
        "final_result.json",
        &serde_json::to_string_pretty(&final_result)?,
    )?);
    artifacts.push(write_file(out_dir, "summary.txt", &summary)?);

    Ok(artifacts)
}

pub fn write_v086_fast_slow_demo(out_dir: &Path) -> Result<Vec<PathBuf>> {
    let mut artifacts = Vec::new();
    let simple_arbitration = execute::CognitiveArbitrationState {
        route_selected: "fast".to_string(),
        reasoning_mode: "quick_commit".to_string(),
        confidence: "high".to_string(),
        risk_class: "low".to_string(),
        applied_constraints: vec!["bounded_latency_budget".to_string()],
        cost_latency_assumption: "prefer minimal deliberation for low-risk local tasks".to_string(),
        route_reason: "simple bounded summary task remains on the fast path".to_string(),
    };
    let simple_path = execute::FastSlowPathState {
        selected_path: "fast_path".to_string(),
        path_family: "fast".to_string(),
        runtime_branch_taken: "fast_direct_execution_branch".to_string(),
        handoff_state: "direct_commit".to_string(),
        candidate_strategy: "select highest-confidence direct execution candidate".to_string(),
        review_depth: "minimal".to_string(),
        execution_profile: "single_pass_bounded_execution".to_string(),
        termination_expectation: "terminate_after_direct_execution".to_string(),
        path_difference_summary: "fast path favors immediate bounded execution with minimal review"
            .to_string(),
    };
    let complex_arbitration = execute::CognitiveArbitrationState {
        route_selected: "slow".to_string(),
        reasoning_mode: "review_heavy".to_string(),
        confidence: "guarded".to_string(),
        risk_class: "high".to_string(),
        applied_constraints: vec![
            "ambiguity_requires_review".to_string(),
            "contradiction_risk_present".to_string(),
        ],
        cost_latency_assumption:
            "spend bounded additional cognition when ambiguity and contradiction risk are present"
                .to_string(),
        route_reason: "complex bounded planning task requires review-first slow routing"
            .to_string(),
    };
    let complex_path = execute::FastSlowPathState {
        selected_path: "slow_path".to_string(),
        path_family: "slow".to_string(),
        runtime_branch_taken: "slow_review_refine_branch".to_string(),
        handoff_state: "review_handoff".to_string(),
        candidate_strategy: "compare multiple candidates before commitment".to_string(),
        review_depth: "verification_required".to_string(),
        execution_profile: "review_and_refine_before_execution".to_string(),
        termination_expectation: "terminate_after_review_cycle_or_gate_decision".to_string(),
        path_difference_summary:
            "slow path introduces explicit review and refinement before execution".to_string(),
    };

    let simple_case = serde_json::json!({
        "scenario": "simple_case",
        "task": "Summarize one bounded local artifact update",
        "arbitration": simple_arbitration,
        "fast_slow_path": simple_path,
    });
    let complex_case = serde_json::json!({
        "scenario": "complex_case",
        "task": "Diagnose contradiction in a high-risk bounded planning request",
        "arbitration": complex_arbitration,
        "fast_slow_path": complex_path,
    });
    let comparison = [
        "Fast vs Slow routing demo comparison",
        "simple_case: route=fast selected_path=fast_path branch=fast_direct_execution_branch",
        "complex_case: route=slow selected_path=slow_path branch=slow_review_refine_branch",
        "difference: the complex case introduces explicit review depth and refinement before commitment",
    ]
    .join("\n");

    artifacts.push(write_file(
        out_dir,
        "simple_case.json",
        &serde_json::to_string_pretty(&simple_case)?,
    )?);
    artifacts.push(write_file(
        out_dir,
        "complex_case.json",
        &serde_json::to_string_pretty(&complex_case)?,
    )?);
    artifacts.push(write_file(out_dir, "comparison.txt", &comparison)?);

    Ok(artifacts)
}

pub fn write_v086_candidate_selection_demo(out_dir: &Path) -> Result<Vec<PathBuf>> {
    let mut artifacts = Vec::new();
    let candidates = vec![
        execute::AgencyCandidateRecord {
            candidate_id: "cand-direct-execute".to_string(),
            candidate_kind: "direct_execution".to_string(),
            bounded_action: "Apply the bounded update immediately".to_string(),
            review_requirement: "none".to_string(),
            execution_priority: 2,
            rationale: "Fastest path, but lower resilience under contradiction".to_string(),
        },
        execute::AgencyCandidateRecord {
            candidate_id: "cand-review-refine".to_string(),
            candidate_kind: "review_and_refine".to_string(),
            bounded_action: "Review and refine before execution".to_string(),
            review_requirement: "verification_required".to_string(),
            execution_priority: 1,
            rationale: "Preferred bounded candidate when contradiction risk is present".to_string(),
        },
        execute::AgencyCandidateRecord {
            candidate_id: "cand-defer".to_string(),
            candidate_kind: "defer".to_string(),
            bounded_action: "Defer and request clarification".to_string(),
            review_requirement: "none".to_string(),
            execution_priority: 3,
            rationale: "Safest fallback if the frame remains inadequate".to_string(),
        },
    ];
    let candidates_artifact = serde_json::json!({
        "agency_selection_version": 1,
        "run_id": "demo-v086-candidate-selection",
        "candidate_generation_basis": "bounded local planning scenario with contradiction risk",
        "selection_mode": "slow_candidate_comparison",
        "candidate_set": candidates,
    });
    let selection_artifact = serde_json::json!({
        "run_id": "demo-v086-candidate-selection",
        "selected_candidate_id": "cand-review-refine",
        "selected_candidate_kind": "review_and_refine",
        "selected_candidate_action": "Review and refine before execution",
        "selected_candidate_reason": "candidate preserves bounded progress while handling contradiction explicitly",
        "deterministic_selection_rule": "prefer lower-risk constrained candidates when contradiction is present",
    });
    let summary = [
        "Candidate selection demo summary",
        "candidate_count: 3",
        "selected_candidate_id: cand-review-refine",
        "selected_candidate_kind: review_and_refine",
        "selection_reason: candidate preserves bounded progress while handling contradiction explicitly",
    ]
    .join("\n");

    artifacts.push(write_file(
        out_dir,
        "candidates.json",
        &serde_json::to_string_pretty(&candidates_artifact)?,
    )?);
    artifacts.push(write_file(
        out_dir,
        "selection.json",
        &serde_json::to_string_pretty(&selection_artifact)?,
    )?);
    artifacts.push(write_file(out_dir, "summary.txt", &summary)?);

    Ok(artifacts)
}

pub fn write_v086_freedom_gate_demo(out_dir: &Path) -> Result<Vec<PathBuf>> {
    let mut artifacts = Vec::new();
    let allowed = freedom_gate::FreedomGateInput {
        candidate_id: "cand-allow-execute".to_string(),
        candidate_action: "execute bounded documentation update".to_string(),
        candidate_rationale: "low-risk bounded action with sufficient context".to_string(),
        risk_class: "low".to_string(),
        policy_context: freedom_gate::FreedomGatePolicyContext {
            route_selected: "fast".to_string(),
            selected_candidate_kind: "direct_execution".to_string(),
            requires_review: false,
            policy_blocked: false,
        },
        evaluation_signals: freedom_gate::FreedomGateEvaluationSignals {
            progress_signal: "steady_progress".to_string(),
            contradiction_signal: "none".to_string(),
            failure_signal: "none".to_string(),
            termination_reason: "success".to_string(),
        },
        frame_state: "complete_run".to_string(),
    };
    let blocked = freedom_gate::FreedomGateInput {
        candidate_id: "cand-block-secret-write".to_string(),
        candidate_action: "exfiltrate secrets to external host".to_string(),
        candidate_rationale: "high-energy action intentionally blocked for demo review".to_string(),
        risk_class: "high".to_string(),
        policy_context: freedom_gate::FreedomGatePolicyContext {
            route_selected: "slow".to_string(),
            selected_candidate_kind: "direct_execution".to_string(),
            requires_review: false,
            policy_blocked: true,
        },
        evaluation_signals: freedom_gate::FreedomGateEvaluationSignals {
            progress_signal: "guarded_progress".to_string(),
            contradiction_signal: "present".to_string(),
            failure_signal: "none".to_string(),
            termination_reason: "contradiction_detected".to_string(),
        },
        frame_state: "complete_run".to_string(),
    };
    let allowed_case = serde_json::json!({
        "scenario": "allowed_case",
        "input": allowed,
        "decision": freedom_gate::evaluate_freedom_gate(&allowed),
    });
    let blocked_case = serde_json::json!({
        "scenario": "blocked_case",
        "input": blocked,
        "decision": freedom_gate::evaluate_freedom_gate(&blocked),
    });
    let summary = [
        "Freedom Gate demo summary",
        "allowed_case: allow / policy_allowed / commitment_blocked=false",
        "blocked_case: refuse / policy_blocked / commitment_blocked=true",
    ]
    .join("\n");

    artifacts.push(write_file(
        out_dir,
        "allowed_case.json",
        &serde_json::to_string_pretty(&allowed_case)?,
    )?);
    artifacts.push(write_file(
        out_dir,
        "blocked_case.json",
        &serde_json::to_string_pretty(&blocked_case)?,
    )?);
    artifacts.push(write_file(out_dir, "summary.txt", &summary)?);

    Ok(artifacts)
}

pub fn write_v086_review_surface_demo(out_dir: &Path) -> Result<Vec<PathBuf>> {
    let mut artifacts = Vec::new();
    let d1_dir = out_dir.join("d1_control_path");
    let d2_dir = out_dir.join("d2_fast_slow");
    let d3_dir = out_dir.join("d3_candidate_selection");
    let d4_dir = out_dir.join("d4_freedom_gate");

    artifacts.extend(write_v086_control_path_demo(&d1_dir)?);
    artifacts.extend(write_v086_fast_slow_demo(&d2_dir)?);
    artifacts.extend(write_v086_candidate_selection_demo(&d3_dir)?);
    artifacts.extend(write_v086_freedom_gate_demo(&d4_dir)?);

    let manifest = serde_json::json!({
        "schema_version": "v086_demo_manifest.v1",
        "review_entry_demo": "D1",
        "demos": [
            {
                "demo_id": "D1",
                "title": "Canonical Bounded Cognitive Path",
                "command": "./adl/tools/demo_v086_control_path.sh",
                "artifact_root": "d1_control_path",
                "primary_proof_surface": "d1_control_path/summary.txt",
            },
            {
                "demo_id": "D2",
                "title": "Fast vs Slow Routing",
                "command": "./adl/tools/demo_v086_fast_slow.sh",
                "artifact_root": "d2_fast_slow",
                "primary_proof_surface": "d2_fast_slow/comparison.txt",
            },
            {
                "demo_id": "D3",
                "title": "Agency / Candidate Selection",
                "command": "./adl/tools/demo_v086_candidate_selection.sh",
                "artifact_root": "d3_candidate_selection",
                "primary_proof_surface": "d3_candidate_selection/selection.json",
            },
            {
                "demo_id": "D4",
                "title": "Freedom Gate Enforcement",
                "command": "./adl/tools/demo_v086_freedom_gate.sh",
                "artifact_root": "d4_freedom_gate",
                "primary_proof_surface": "d4_freedom_gate/blocked_case.json",
            }
        ]
    });
    let readme = [
        "v0.86 Review Surface Walkthrough",
        "",
        "Run D1 first if you only inspect one proof surface.",
        "Primary entry point: d1_control_path/summary.txt",
        "",
        "Other primary proof surfaces:",
        "- D2: d2_fast_slow/comparison.txt",
        "- D3: d3_candidate_selection/selection.json",
        "- D4: d4_freedom_gate/blocked_case.json",
        "",
        "Use docs/milestones/v0.86/DEMO_MATRIX_v0.86.md for the full review contract.",
    ]
    .join("\n");
    let index = [
        "D1 -> d1_control_path/summary.txt",
        "D2 -> d2_fast_slow/comparison.txt",
        "D3 -> d3_candidate_selection/selection.json",
        "D4 -> d4_freedom_gate/blocked_case.json",
    ]
    .join("\n");

    artifacts.push(write_file(
        out_dir,
        "demo_manifest.json",
        &serde_json::to_string_pretty(&manifest)?,
    )?);
    artifacts.push(write_file(out_dir, "README.txt", &readme)?);
    artifacts.push(write_file(out_dir, "index.txt", &index)?);

    Ok(artifacts)
}

fn write_file(out_dir: &Path, rel: &str, contents: &str) -> Result<PathBuf> {
    let path = out_dir.join(rel);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("failed to create '{}'", parent.display()))?;
    }
    std::fs::write(&path, contents.as_bytes())
        .with_context(|| format!("failed to write '{}'", path.display()))?;
    Ok(path)
}

fn write_trace_jsonl(out_dir: &Path, trace: &Trace) -> Result<PathBuf> {
    let path = out_dir.join("trace.jsonl");
    let mut lines = Vec::new();
    lines.push(format!(
        "TRACE run_id={} workflow_id={} version={}",
        trace.run_id, trace.workflow_id, trace.version
    ));
    for ev in &trace.events {
        lines.push(ev.summarize());
    }
    let mut body = lines.join("\n");
    body.push('\n');
    std::fs::write(&path, body.as_bytes())
        .with_context(|| format!("failed to write '{}'", path.display()))?;
    Ok(path)
}

const CARGO_TOML: &str = r#"[package]
name = "say_mcp_demo"
version = "0.1.0"
edition = "2021"

[dependencies]
"#;

const README_MD: &str = r#"# Demo A — MCP `say` server scaffold + HTML game

This directory is generated by `adl demo demo-a-say-mcp`.

## What you got
- `src/lib.rs`: safe input validation + argv construction for macOS `say`
- `src/main.rs`: runnable CLI wrapper (no deps)
- `tests/say_server_tests.rs`: integration tests
- `design.md`: brief design notes
- `index.html`: Rock/Paper/Scissors mini-game

## Run it
```bash
cargo build
cargo test

# speak text
cargo run -- "Hello world!"

# optional voice + rate
cargo run -- "Hello" Samantha 180
```

## Open the game
On macOS:
```bash
open index.html
```

## Notes
- This is a demo scaffold. It intentionally avoids extra dependencies.
- `say` is invoked via `std::process::Command` with discrete argv entries (no shell).
"#;
const DESIGN_MD: &str = r#"# Demo A: Remote Brain, Local Hands

## Goal
Build a demo-grade MCP server for macOS `say` and generate a small HTML game artifact.

## Interface (MCP-style, demo scope)
- Tool name: `speak_text`
- Input: `{ "text": string, "voice"?: string, "rate"?: integer }`
- Validation:
  - reject empty text
  - max length: 500 characters
  - allow `[A-Za-z0-9 .,!?'-]` plus newline
- Execution:
  - use `std::process::Command::new("say")`
  - pass arguments as discrete argv entries (no shell interpolation)
"#;

const SRC_LIB_RS: &str = r#"use std::process::Command;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpeakRequest {
    pub text: String,
    pub voice: Option<String>,
    pub rate: Option<u32>,
}

pub fn validate_text(text: &str) -> Result<(), String> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return Err("text is empty".to_string());
    }
    if trimmed.chars().count() > 500 {
        return Err("text too long".to_string());
    }
    if !trimmed
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || " .,!?'-\n".contains(c))
    {
        return Err("text contains unsupported characters".to_string());
    }
    Ok(())
}

pub fn build_say_args(req: &SpeakRequest) -> Result<Vec<String>, String> {
    validate_text(&req.text)?;
    let mut args = Vec::new();
    if let Some(voice) = &req.voice {
        args.push("-v".to_string());
        args.push(voice.clone());
    }
    if let Some(rate) = req.rate {
        args.push("-r".to_string());
        args.push(rate.to_string());
    }
    args.push(req.text.clone());
    Ok(args)
}

pub fn execute_say(req: &SpeakRequest) -> Result<(), String> {
    let args = build_say_args(req)?;
    let status = Command::new("say")
        .args(args)
        .status()
        .map_err(|e| format!("failed to execute say: {e}"))?;
    if !status.success() {
        return Err(format!("say failed with status {:?}", status.code()));
    }
    Ok(())
}
"#;

const SRC_MAIN_RS: &str = r#"use say_mcp_demo::{execute_say, SpeakRequest};

fn main() {
    // Minimal runnable binary for the demo. This keeps dependencies at zero.
    // Usage examples:
    //   cargo run -- "Hello world!"
    //   cargo run -- "Hello" Samantha 180
    let mut args = std::env::args().skip(1).collect::<Vec<_>>();
    if args.is_empty() {
        eprintln!("Usage:\n  cargo run -- <text> [voice] [rate]\n\nExamples:\n  cargo run -- \"Hello world!\"\n  cargo run -- \"Hello\" Samantha 180\n");
        std::process::exit(2);
    }

    let text = args.remove(0);
    let voice = args.get(0).cloned();
    let rate = args
        .get(1)
        .and_then(|s| s.parse::<u32>().ok());

    let req = SpeakRequest { text, voice, rate };
    if let Err(e) = execute_say(&req) {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}
"#;

const TESTS_RS: &str = r#"use say_mcp_demo::{build_say_args, validate_text, SpeakRequest};

#[test]
fn validate_accepts_safe_text() {
    assert!(validate_text("Hello world!").is_ok());
}

#[test]
fn validate_rejects_empty_text() {
    assert!(validate_text("   ").is_err());
}

#[test]
fn validate_rejects_unsupported_chars() {
    assert!(validate_text("$(rm -rf /)").is_err());
}

#[test]
fn build_args_includes_voice_and_rate() {
    let req = SpeakRequest {
        text: "Hello".to_string(),
        voice: Some("Samantha".to_string()),
        rate: Some(180),
    };
    let args = build_say_args(&req).unwrap();
    assert_eq!(args, vec!["-v", "Samantha", "-r", "180", "Hello"]);
}
"#;

const COVERAGE_TXT: &str = r#"module: say_mcp_demo
line_coverage: 82.1%
method: demo estimate from unit-test path coverage
notes:
- input validation branches covered
- argv construction branches covered
- subprocess error path covered by contract tests in runtime
"#;

const GAME_HTML: &str = r#"<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1" />
  <title>Rock Paper Scissors</title>
  <style>
    body { font-family: ui-monospace, SFMono-Regular, Menlo, monospace; margin: 24px; }
    button { margin-right: 8px; padding: 8px 12px; }
    #result { margin-top: 16px; font-weight: 600; }
  </style>
</head>
<body>
  <h1>Rock / Paper / Scissors</h1>
  <p>Pick a move:</p>
  <div>
    <button onclick="play('rock')">Rock</button>
    <button onclick="play('paper')">Paper</button>
    <button onclick="play('scissors')">Scissors</button>
  </div>
  <div id="result">Result: waiting...</div>
  <script>
    const moves = ['rock', 'paper', 'scissors'];
    function winner(user, cpu) {
      if (user === cpu) return 'Draw';
      if ((user === 'rock' && cpu === 'scissors') ||
          (user === 'paper' && cpu === 'rock') ||
          (user === 'scissors' && cpu === 'paper')) return 'You win';
      return 'CPU wins';
    }
    function play(user) {
      const cpu = moves[Math.floor(Math.random() * moves.length)];
      document.getElementById('result').textContent =
        `Result: ${winner(user, cpu)} (you: ${user}, cpu: ${cpu})`;
    }
  </script>
</body>
</html>
"#;

const DEMO_B_DESIGN_MD: &str = r#"# Demo B: One Command, Quiet Success

## Objective
Show a frictionless local demo command with deterministic outputs.

## UX Contract
- one obvious command
- quiet-by-default on success
- optional trace output
- safe auto-open for generated `index.html`
"#;

const DEMO_B_README_MD: &str = r#"# Demo B Output

Generated by:

```bash
cargo run -- demo demo-b-one-command --run --out <dir>
```

Optional trace:

```bash
cargo run -- demo demo-b-one-command --run --trace --out <dir>
```
"#;

const DEMO_B_COVERAGE_TXT: &str = r#"module: demo_b_artifacts
line_coverage: 80.0%
method: deterministic fixture coverage estimate
"#;

const DEMO_C_README_MD: &str = r#"# Demo C Output

Generated by:

```bash
cargo run -- demo demo-c-godel-runtime --run --out <dir>
```

This demo validates canonical v0.8 Gödel runtime artifact surfaces and emits:

- `godel_runtime_surface_status.json`
- `verification.txt`
- `trace.jsonl`
"#;

const DEMO_D_FAILURE_SIGNAL_JSON: &str = r#"{
  "schema_version": "godel_failure_signal.v1",
  "run_id": "demo-d-run-001",
  "workflow_id": "godel-obsmem-demo",
  "failure_code": "tool_failure",
  "failure_summary": "bounded deterministic failure signal for demo",
  "evidence_refs": [
    "runs/demo-d-run-001/logs/activation_log.json",
    "runs/demo-d-run-001/run_status.json"
  ]
}
"#;

const DEMO_D_README_MD: &str = r#"# Demo D Output — Gödel + ObsMem Loop

Generated by:

```bash
cargo run --manifest-path adl/Cargo.toml --bin adl -- demo demo-d-godel-obsmem-loop --run --trace --out ./out
```

This demo executes the bounded Gödel stage loop and persists:

- `runs/demo-d-run-001/godel/canonical_evidence_view.v1.json`
- `runs/demo-d-run-001/godel/experiment_record.runtime.v1.json`
- `runs/demo-d-run-001/godel/obsmem_index_entry.runtime.v1.json`
- `godel_obsmem_demo_summary.json`
- `trace.jsonl`
"#;

const DEMO_E_INPUT_CARD_MD: &str = r#"# Input Card (Demo Fixture)

Task: produce a concise release-note paragraph for a deterministic CLI improvement.

Prompt Spec:
- actor: writer
- model: bounded-demo
- outputs: markdown paragraph
"#;

const DEMO_E_WRITER_MD: &str = r#"# Stage 1 — Writer

The CLI now includes deterministic demo surfaces with explicit artifact paths and stable replay-oriented output.
"#;

const DEMO_E_EDITOR_MD: &str = r#"# Stage 2 — Editor

Edited for clarity: each demo documents command, artifacts, and deterministic constraints.
"#;

const DEMO_E_COPYEDITOR_MD: &str = r#"# Stage 3 — Copyeditor

Copyedited to remove ambiguity and keep all artifact paths repo-relative.
"#;

const DEMO_E_README_MD: &str = r#"# Demo E Output — Multi-Agent Card Pipeline

Generated by:

```bash
cargo run --manifest-path adl/Cargo.toml --bin adl -- demo demo-e-multi-agent-card-pipeline --run --trace --out ./out
```

Stages:
- writer -> editor -> copyeditor -> publisher

Primary artifacts:
- `pipeline/input_card.md`
- `pipeline/01_writer.md`
- `pipeline/02_editor.md`
- `pipeline/03_copyeditor.md`
- `pipeline/pipeline_manifest.json`
- `trace.jsonl`
"#;

const DEMO_F_README_MD: &str = r#"# Demo F Output — ObsMem Retrieval

Generated by:

```bash
cargo run --manifest-path adl/Cargo.toml --bin adl -- demo demo-f-obsmem-retrieval --run --trace --out ./out
```

This demo seeds deterministic runtime experiment/index artifacts and performs deterministic retrieval by:

- `failure_code`
- optional `hypothesis_id`
- optional `experiment_outcome`

Primary artifacts:
- `runs/demo-f-run-a/godel/experiment_record.runtime.v1.json`
- `runs/demo-f-run-a/godel/obsmem_index_entry.runtime.v1.json`
- `runs/demo-f-run-b/godel/experiment_record.runtime.v1.json`
- `runs/demo-f-run-b/godel/obsmem_index_entry.runtime.v1.json`
- `obsmem_retrieval_result.json`
- `trace.jsonl`
"#;

const DEMO_G_README_MD: &str = r#"# D1 Canonical Bounded Cognitive Path

Generated by:

```bash
cargo run --manifest-path adl/Cargo.toml --bin adl -- demo demo-g-v086-control-path --run --trace --out ./out
```

This demo emits one stable, reviewer-facing control-path artifact set covering:

- signals
- candidate selection
- arbitration
- bounded execution
- evaluation
- reframing
- memory participation
- Freedom Gate
- final result

Primary artifacts:
- `signals.json`
- `candidate_selection.json`
- `arbitration.json`
- `execution_iterations.json`
- `evaluation.json`
- `reframing.json`
- `memory.json`
- `freedom_gate.json`
- `final_result.json`
- `summary.txt`
- `trace.jsonl`
"#;

#[derive(Debug, serde::Serialize)]
struct CardPipelineManifest {
    schema_version: &'static str,
    stage_order: Vec<&'static str>,
    stage_artifacts: Vec<CardPipelineStageArtifact>,
}

#[derive(Debug, serde::Serialize)]
struct CardPipelineStageArtifact {
    stage: &'static str,
    path: &'static str,
    content_hash: String,
}

fn build_card_pipeline_manifest() -> CardPipelineManifest {
    let stage_artifacts = vec![
        ("writer", "pipeline/01_writer.md", DEMO_E_WRITER_MD),
        ("editor", "pipeline/02_editor.md", DEMO_E_EDITOR_MD),
        (
            "copyeditor",
            "pipeline/03_copyeditor.md",
            DEMO_E_COPYEDITOR_MD,
        ),
    ]
    .into_iter()
    .map(|(stage, path, content)| CardPipelineStageArtifact {
        stage,
        path,
        content_hash: prompt::hash_prompt(content),
    })
    .collect();

    CardPipelineManifest {
        schema_version: "multi_agent_card_pipeline.demo.v1",
        stage_order: vec!["writer", "editor", "copyeditor", "publisher"],
        stage_artifacts,
    }
}

fn run_godel_stage_loop_demo(out_dir: &Path) -> Result<Vec<PathBuf>> {
    let mut artifacts = Vec::new();
    let runs_root = out_dir.join("runs");
    let exec = godel::GodelStageLoopExecutor::new(godel::StageLoopConfig::default());
    let input = godel::StageLoopInput {
        run_id: "demo-d-run-001".to_string(),
        workflow_id: "godel-obsmem-demo".to_string(),
        failure_code: "tool_failure".to_string(),
        failure_summary: "bounded deterministic failure signal for demo".to_string(),
        evidence_refs: vec![
            "runs/demo-d-run-001/logs/activation_log.json".to_string(),
            "runs/demo-d-run-001/run_status.json".to_string(),
        ],
    };
    let persisted = exec.execute_and_persist(&input, &runs_root)?;
    let summary = serde_json::json!({
        "schema_version": "godel_obsmem_demo_summary.v1",
        "run_id": persisted.run.record.run_id,
        "stage_order": persisted
            .run
            .stage_order
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<_>>(),
        "hypothesis_count": persisted.run.hypotheses.len(),
        "selected_hypothesis_id": persisted.run.hypothesis.id,
        "selected_mutation_id": persisted.run.mutation.id,
        "canonical_mutation_rel_path": persisted.canonical_mutation_rel_path,
        "evaluation_decision": format!("{:?}", persisted.run.evaluation.decision).to_lowercase(),
        "canonical_evaluation_plan_rel_path": persisted.canonical_evaluation_plan_rel_path,
        "canonical_evidence_rel_path": persisted.canonical_evidence_rel_path,
        "experiment_record_rel_path": persisted.experiment_record_rel_path,
        "obsmem_index_rel_path": persisted.obsmem_index_rel_path
    });
    artifacts.push(write_file(
        out_dir,
        "godel_obsmem_demo_summary.json",
        &serde_json::to_string_pretty(&summary)?,
    )?);
    artifacts.push(out_dir.join("runs/demo-d-run-001/godel/evaluation_plan.v1.json"));
    artifacts.push(out_dir.join("runs/demo-d-run-001/godel/mutation.v1.json"));
    artifacts.push(out_dir.join("runs/demo-d-run-001/godel/canonical_evidence_view.v1.json"));
    artifacts.push(out_dir.join("runs/demo-d-run-001/godel/evaluation_plan.v1.json"));
    artifacts.push(out_dir.join("runs/demo-d-run-001/godel/experiment_record.runtime.v1.json"));
    artifacts.push(out_dir.join("runs/demo-d-run-001/godel/obsmem_index_entry.runtime.v1.json"));
    Ok(artifacts)
}

fn seed_obsmem_retrieval_demo(out_dir: &Path) -> Result<Vec<PathBuf>> {
    let mut artifacts = Vec::new();
    let runs_root = out_dir.join("runs");
    let exec = godel::GodelStageLoopExecutor::new(godel::StageLoopConfig::default());

    let input_a = godel::StageLoopInput {
        run_id: "demo-f-run-a".to_string(),
        workflow_id: "godel-retrieval-demo".to_string(),
        failure_code: "tool_failure".to_string(),
        failure_summary: "deterministic failure A".to_string(),
        evidence_refs: vec!["runs/demo-f-run-a/run_status.json".to_string()],
    };
    let mut input_b = input_a.clone();
    input_b.run_id = "demo-f-run-b".to_string();
    input_b.failure_code = "policy_denied".to_string();
    input_b.failure_summary = "deterministic failure B".to_string();
    input_b.evidence_refs = vec!["runs/demo-f-run-b/run_status.json".to_string()];

    for input in [input_a, input_b] {
        let persisted = exec.execute_and_persist(&input, &runs_root)?;
        artifacts.push(out_dir.join(&persisted.experiment_record_rel_path));
        artifacts.push(out_dir.join(&persisted.obsmem_index_rel_path));
    }

    Ok(artifacts)
}

fn query_obsmem_retrieval_demo(out_dir: &Path) -> Result<Vec<PathBuf>> {
    let mut entries: Vec<godel::obsmem_index::StageIndexEntry> = Vec::new();
    let runs_root = out_dir.join("runs");
    for run_id in ["demo-f-run-a", "demo-f-run-b"] {
        let path = runs_root
            .join(run_id)
            .join("godel")
            .join("obsmem_index_entry.runtime.v1.json");
        let raw = std::fs::read_to_string(&path)
            .with_context(|| format!("failed to read '{}'", path.display()))?;
        let persisted: godel::obsmem_index::PersistedStageIndexEntry =
            serde_json::from_str(&raw)
                .with_context(|| format!("failed to parse '{}'", path.display()))?;
        entries.push(persisted.entry);
    }

    let query = godel::obsmem_index::ObsMemIndexQuery {
        failure_code: "tool_failure".to_string(),
        hypothesis_id: None,
        experiment_outcome: None,
    };

    let mut matches: Vec<_> = entries
        .into_iter()
        .filter(|e| godel::obsmem_index::matches_query(e, &query))
        .collect();
    matches.sort_by(|a, b| {
        a.index_key
            .cmp(&b.index_key)
            .then(a.run_id.cmp(&b.run_id))
            .then(a.mutation_id.cmp(&b.mutation_id))
    });

    let result = serde_json::json!({
        "schema_version": "obsmem_retrieval_result.demo.v1",
        "query": query,
        "result_count": matches.len(),
        "results": matches,
    });

    let path = write_file(
        out_dir,
        "obsmem_retrieval_result.json",
        &serde_json::to_string_pretty(&result)?,
    )?;
    Ok(vec![path])
}

fn generate_rps_game_html() -> String {
    // Build from components instead of embedding one monolithic page.
    let title = "Rock / Paper / Scissors";
    let moves = ["Rock", "Paper", "Scissors"];

    let buttons = moves
        .iter()
        .map(|m| format!(r#"<button type="button" data-move="{m}">{m}</button>"#))
        .collect::<Vec<_>>()
        .join("\n        ");

    let js_moves = moves
        .iter()
        .map(|m| format!(r#""{m}""#))
        .collect::<Vec<_>>()
        .join(", ");

    format!(
        r#"<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1" />
  <title>{title}</title>
  <style>
    body {{ font-family: system-ui, -apple-system, Segoe UI, Roboto, sans-serif; margin: 40px; }}
    .card {{ max-width: 760px; border: 1px solid #ddd; padding: 28px; border-radius: 12px; }}
    h1 {{ margin: 0 0 12px 0; font-size: 44px; }}
    p {{ margin: 0 0 18px 0; font-size: 18px; color: #333; }}
    .row {{ display: flex; gap: 12px; flex-wrap: wrap; margin: 14px 0 18px; }}
    button {{ padding: 10px 14px; font-size: 16px; border-radius: 10px; border: 1px solid #ccc; background: #fff; cursor: pointer; }}
    button:hover {{ background: #f6f6f6; }}
    .out {{ margin-top: 16px; padding: 14px; border-radius: 10px; background: #fafafa; border: 1px solid #eee; }}
    .mono {{ font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace; }}
  </style>
</head>
<body>
  <div class="card">
    <h1>{title}</h1>
    <p>Pick your move.</p>
    <div class="row">
        {buttons}
        <button type="button" id="reset">Reset</button>
    </div>
    <div class="out">
      <div>Round: <span id="round" class="mono">0</span></div>
      <div>You: <span id="you" class="mono">-</span></div>
      <div>Computer: <span id="cpu" class="mono">-</span></div>
      <div>Result: <span id="result" class="mono">-</span></div>
    </div>
  </div>

  <script>
    const moves = [{js_moves}];
    let round = 0;

    function cpuMove() {{
      return moves[round % moves.length]; // deterministic
    }}

    function decide(you, cpu) {{
      if (you === cpu) return "Draw";
      if (you === "Rock" && cpu === "Scissors") return "You win";
      if (you === "Paper" && cpu === "Rock") return "You win";
      if (you === "Scissors" && cpu === "Paper") return "You win";
      return "You lose";
    }}

    function setText(id, v) {{ document.getElementById(id).textContent = v; }}

    function play(you) {{
      const cpu = cpuMove();
      round += 1;
      setText("round", String(round));
      setText("you", you);
      setText("cpu", cpu);
      setText("result", decide(you, cpu));
    }}

    function reset() {{
      round = 0;
      setText("round", "0");
      setText("you", "-");
      setText("cpu", "-");
      setText("result", "-");
    }}

    document.querySelectorAll("button[data-move]").forEach(btn => {{
      btn.addEventListener("click", () => play(btn.getAttribute("data-move")));
    }});
    document.getElementById("reset").addEventListener("click", reset);
  </script>
</body>
</html>
"#
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};

    static TEMP_DIR_SEQ: AtomicU64 = AtomicU64::new(0);

    fn tmp_dir(prefix: &str) -> PathBuf {
        let mut p = std::env::temp_dir().join("adl-test-temp");
        std::fs::create_dir_all(&p).unwrap();
        let seq = TEMP_DIR_SEQ.fetch_add(1, Ordering::Relaxed);
        p.push(format!("adl-{prefix}-pid{}-n{seq}", std::process::id()));
        std::fs::create_dir_all(&p).unwrap();
        p
    }

    #[test]
    fn run_demo_writes_required_artifacts() {
        let out = tmp_dir("demo-a");
        let result = run_demo(DEMO_A_SAY_MCP, &out).unwrap();
        assert_eq!(result.run_id, DEMO_A_SAY_MCP);
        assert!(out.join("design.md").is_file());
        assert!(out.join("Cargo.toml").is_file());
        assert!(out.join("README.md").is_file());
        assert!(out.join("src/lib.rs").is_file());
        assert!(out.join("src/main.rs").is_file());
        assert!(out.join("tests/say_server_tests.rs").is_file());
        assert!(out.join("coverage.txt").is_file());
        assert!(out.join("index.html").is_file());
        assert!(out.join("trace.jsonl").is_file());
    }

    #[test]
    fn run_demo_b_writes_required_artifacts() {
        let out = tmp_dir("demo-b");
        let result = run_demo(DEMO_B_ONE_COMMAND, &out).unwrap();
        assert_eq!(result.run_id, DEMO_B_ONE_COMMAND);
        assert!(out.join("design.md").is_file());
        assert!(out.join("README.md").is_file());
        assert!(out.join("coverage.txt").is_file());
        assert!(out.join("index.html").is_file());
        assert!(out.join("trace.jsonl").is_file());
    }

    #[test]
    fn run_demo_c_writes_runtime_surface_artifacts() {
        let out = tmp_dir("demo-c");
        let result = run_demo(DEMO_C_GODEL_RUNTIME, &out).unwrap();
        assert_eq!(result.run_id, DEMO_C_GODEL_RUNTIME);
        assert!(out.join("godel_runtime_surface_status.json").is_file());
        assert!(out.join("verification.txt").is_file());
        assert!(out.join("README.md").is_file());
        assert!(out.join("trace.jsonl").is_file());
        let status =
            std::fs::read_to_string(out.join("godel_runtime_surface_status.json")).unwrap();
        assert!(status.contains("\"failure\""), "status was:\n{status}");
        assert!(status.contains("\"record\""), "status was:\n{status}");
    }

    #[test]
    fn run_demo_d_writes_godel_obsmem_artifacts() {
        let out = tmp_dir("demo-d");
        let result = run_demo(DEMO_D_GODEL_OBSMEM_LOOP, &out).unwrap();
        assert_eq!(result.run_id, DEMO_D_GODEL_OBSMEM_LOOP);
        assert!(out.join("failure_signal.json").is_file());
        assert!(out.join("godel_obsmem_demo_summary.json").is_file());
        assert!(out
            .join("runs/demo-d-run-001/godel/experiment_record.runtime.v1.json")
            .is_file());
        assert!(out
            .join("runs/demo-d-run-001/godel/obsmem_index_entry.runtime.v1.json")
            .is_file());
        assert!(out.join("README.md").is_file());
        assert!(out.join("trace.jsonl").is_file());
    }

    #[test]
    fn run_demo_e_writes_multi_agent_pipeline_artifacts() {
        let out = tmp_dir("demo-e");
        let result = run_demo(DEMO_E_MULTI_AGENT_CARD_PIPELINE, &out).unwrap();
        assert_eq!(result.run_id, DEMO_E_MULTI_AGENT_CARD_PIPELINE);
        assert!(out.join("pipeline/input_card.md").is_file());
        assert!(out.join("pipeline/01_writer.md").is_file());
        assert!(out.join("pipeline/02_editor.md").is_file());
        assert!(out.join("pipeline/03_copyeditor.md").is_file());
        assert!(out.join("pipeline/pipeline_manifest.json").is_file());
        assert!(out.join("README.md").is_file());
        assert!(out.join("trace.jsonl").is_file());
    }

    #[test]
    fn run_demo_f_writes_obsmem_retrieval_artifacts() {
        let out = tmp_dir("demo-f");
        let result = run_demo(DEMO_F_OBSMEM_RETRIEVAL, &out).unwrap();
        assert_eq!(result.run_id, DEMO_F_OBSMEM_RETRIEVAL);
        assert!(out
            .join("runs/demo-f-run-a/godel/experiment_record.runtime.v1.json")
            .is_file());
        assert!(out
            .join("runs/demo-f-run-a/godel/obsmem_index_entry.runtime.v1.json")
            .is_file());
        assert!(out
            .join("runs/demo-f-run-b/godel/experiment_record.runtime.v1.json")
            .is_file());
        assert!(out
            .join("runs/demo-f-run-b/godel/obsmem_index_entry.runtime.v1.json")
            .is_file());
        assert!(out.join("obsmem_retrieval_result.json").is_file());
        assert!(out.join("README.md").is_file());
        assert!(out.join("trace.jsonl").is_file());
    }

    #[test]
    fn run_demo_g_writes_control_path_artifacts() {
        let out = tmp_dir("demo-g");
        let result = run_demo(DEMO_G_V086_CONTROL_PATH, &out).unwrap();
        assert_eq!(result.run_id, DEMO_G_V086_CONTROL_PATH);
        assert!(out.join("signals.json").is_file());
        assert!(out.join("candidate_selection.json").is_file());
        assert!(out.join("arbitration.json").is_file());
        assert!(out.join("execution_iterations.json").is_file());
        assert!(out.join("evaluation.json").is_file());
        assert!(out.join("reframing.json").is_file());
        assert!(out.join("memory.json").is_file());
        assert!(out.join("freedom_gate.json").is_file());
        assert!(out.join("final_result.json").is_file());
        assert!(out.join("summary.txt").is_file());
        assert!(out.join("README.md").is_file());
        assert!(out.join("trace.jsonl").is_file());
        let summary = std::fs::read_to_string(out.join("summary.txt")).unwrap();
        assert!(
            summary.contains("final_result: defer"),
            "summary was:\n{summary}"
        );
    }

    #[test]
    fn demo_b_html_is_generated_from_components() {
        let html = generate_rps_game_html();
        assert!(html.contains("data-move=\"Rock\""), "html was:\n{html}");
        assert!(html.contains("data-move=\"Paper\""), "html was:\n{html}");
        assert!(html.contains("data-move=\"Scissors\""), "html was:\n{html}");
        assert!(html.contains("id=\"reset\""), "html was:\n{html}");
        assert!(
            html.contains("return moves[round % moves.length]"),
            "html was:\n{html}"
        );
    }

    #[test]
    fn unknown_demo_errors() {
        let out = tmp_dir("demo-unknown");
        let err = run_demo("nope", &out).unwrap_err();
        assert!(format!("{err:#}").contains("unknown demo"));
    }

    #[test]
    fn trace_file_contains_run_finished() {
        let out = tmp_dir("demo-trace");
        run_demo(DEMO_A_SAY_MCP, &out).unwrap();
        let trace = std::fs::read_to_string(out.join("trace.jsonl")).unwrap();
        assert!(trace.contains("RunFinished"), "trace was:\n{trace}");
    }
}
