use super::*;

#[test]
fn godel_run_validates_required_and_unknown_args() {
    let missing_run_id = run_adl(&["godel", "run"]);
    assert!(!missing_run_id.status.success());
    let stderr_missing = String::from_utf8_lossy(&missing_run_id.stderr);
    assert!(
        stderr_missing.contains("godel run requires --run-id <id>"),
        "stderr:\n{stderr_missing}"
    );

    let unknown = run_adl(&["godel", "run", "--bogus", "x"]);
    assert!(!unknown.status.success());
    let stderr_unknown = String::from_utf8_lossy(&unknown.stderr);
    assert!(
        stderr_unknown.contains("unknown godel run arg"),
        "stderr:\n{stderr_unknown}"
    );
}

#[test]
fn godel_run_executes_bounded_stage_loop_and_persists_artifacts() {
    let runs_dir = unique_test_temp_dir("adl-godel-run");
    let out = run_adl(&[
        "godel",
        "run",
        "--run-id",
        "run-745-a",
        "--workflow-id",
        "wf-godel-loop",
        "--failure-code",
        "tool_failure",
        "--failure-summary",
        "step failed with deterministic parse error",
        "--evidence-ref",
        "runs/run-745-a/run_status.json",
        "--evidence-ref",
        "runs/run-745-a/logs/activation_log.json",
        "--runs-dir",
        runs_dir.to_str().unwrap(),
    ]);
    assert!(
        out.status.success(),
        "stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );

    let stdout = String::from_utf8_lossy(&out.stdout);
    let summary: serde_json::Value = serde_json::from_str(&stdout).expect("parse godel summary");
    assert_eq!(summary["run_id"], "run-745-a");
    assert_eq!(summary["workflow_id"], "wf-godel-loop");
    assert_eq!(
        summary["stage_order"],
        serde_json::json!([
            "failure",
            "hypothesis",
            "mutation",
            "experiment",
            "evaluation",
            "record",
            "indexing"
        ])
    );
    assert_eq!(
        summary["hypothesis_path"],
        "runs/run-745-a/godel/godel_hypothesis.v1.json"
    );
    assert_eq!(
        summary["policy_path"],
        "runs/run-745-a/godel/godel_policy.v1.json"
    );
    assert_eq!(
        summary["policy_comparison_path"],
        "runs/run-745-a/godel/godel_policy_comparison.v1.json"
    );
    assert_eq!(
        summary["prioritization_path"],
        "runs/run-745-a/godel/godel_experiment_priority.v1.json"
    );
    assert_eq!(
        summary["cross_workflow_path"],
        "runs/run-745-a/godel/godel_cross_workflow_learning.v1.json"
    );
    assert_eq!(
        summary["eval_report_path"],
        "runs/run-745-a/godel/godel_eval_report.v1.json"
    );
    assert_eq!(
        summary["promotion_decision_path"],
        "runs/run-745-a/godel/godel_promotion_decision.v1.json"
    );
    assert_eq!(
        summary["canonical_evaluation_plan_path"],
        "runs/run-745-a/godel/evaluation_plan.v1.json"
    );
    assert_eq!(
        summary["canonical_mutation_path"],
        "runs/run-745-a/godel/mutation.v1.json"
    );
    assert_eq!(
        summary["canonical_evidence_path"],
        "runs/run-745-a/godel/canonical_evidence_view.v1.json"
    );
    assert_eq!(
        summary["experiment_record_path"],
        "runs/run-745-a/godel/experiment_record.runtime.v1.json"
    );
    assert_eq!(
        summary["canonical_experiment_record_path"],
        "runs/run-745-a/godel/experiment_record.v1.json"
    );
    assert_eq!(
        summary["obsmem_index_path"],
        "runs/run-745-a/godel/obsmem_index_entry.runtime.v1.json"
    );
    assert!(runs_dir
        .join("run-745-a/godel/godel_hypothesis.v1.json")
        .is_file());
    assert!(runs_dir
        .join("run-745-a/godel/godel_policy.v1.json")
        .is_file());
    assert!(runs_dir
        .join("run-745-a/godel/godel_policy_comparison.v1.json")
        .is_file());
    assert!(runs_dir
        .join("run-745-a/godel/godel_experiment_priority.v1.json")
        .is_file());
    assert!(runs_dir
        .join("run-745-a/godel/godel_cross_workflow_learning.v1.json")
        .is_file());
    assert!(runs_dir
        .join("run-745-a/godel/godel_eval_report.v1.json")
        .is_file());
    assert!(runs_dir
        .join("run-745-a/godel/godel_promotion_decision.v1.json")
        .is_file());
    assert!(runs_dir
        .join("run-745-a/godel/evaluation_plan.v1.json")
        .is_file());
    assert!(runs_dir.join("run-745-a/godel/mutation.v1.json").is_file());
    assert!(runs_dir
        .join("run-745-a/godel/canonical_evidence_view.v1.json")
        .is_file());
    assert!(runs_dir
        .join("run-745-a/godel/experiment_record.runtime.v1.json")
        .is_file());
    assert!(runs_dir
        .join("run-745-a/godel/experiment_record.v1.json")
        .is_file());
    assert!(runs_dir
        .join("run-745-a/godel/obsmem_index_entry.runtime.v1.json")
        .is_file());
}

#[test]
fn godel_inspect_validates_required_and_unknown_args() {
    let missing_run_id = run_adl(&["godel", "inspect"]);
    assert!(!missing_run_id.status.success());
    let stderr_missing = String::from_utf8_lossy(&missing_run_id.stderr);
    assert!(
        stderr_missing.contains("godel inspect requires --run-id <id>"),
        "stderr:\n{stderr_missing}"
    );

    let unknown = run_adl(&["godel", "inspect", "--bogus", "x"]);
    assert!(!unknown.status.success());
    let stderr_unknown = String::from_utf8_lossy(&unknown.stderr);
    assert!(
        stderr_unknown.contains("unknown godel inspect arg"),
        "stderr:\n{stderr_unknown}"
    );
}

#[test]
fn godel_evaluate_validates_required_and_unknown_args() {
    let missing_failure_code = run_adl(&["godel", "evaluate"]);
    assert!(!missing_failure_code.status.success());
    let stderr_missing = String::from_utf8_lossy(&missing_failure_code.stderr);
    assert!(
        stderr_missing.contains("godel evaluate requires --failure-code <code>"),
        "stderr:\n{stderr_missing}"
    );

    let unknown = run_adl(&["godel", "evaluate", "--bogus", "x"]);
    assert!(!unknown.status.success());
    let stderr_unknown = String::from_utf8_lossy(&unknown.stderr);
    assert!(
        stderr_unknown.contains("unknown godel evaluate arg"),
        "stderr:\n{stderr_unknown}"
    );
}

#[test]
fn godel_affect_slice_validates_required_and_unknown_args() {
    let missing_initial = run_adl(&["godel", "affect-slice"]);
    assert!(!missing_initial.status.success());
    let stderr_missing = String::from_utf8_lossy(&missing_initial.stderr);
    assert!(
        stderr_missing.contains("godel affect-slice requires --initial-run-id <id>"),
        "stderr:\n{stderr_missing}"
    );

    let unknown = run_adl(&["godel", "affect-slice", "--bogus", "x"]);
    assert!(!unknown.status.success());
    let stderr_unknown = String::from_utf8_lossy(&unknown.stderr);
    assert!(
        stderr_unknown.contains("unknown godel affect-slice arg"),
        "stderr:\n{stderr_unknown}"
    );
}

#[test]
fn godel_inspect_reads_runtime_artifacts_deterministically() {
    let runs_dir = unique_test_temp_dir("adl-godel-inspect");
    let run = run_adl(&[
        "godel",
        "run",
        "--run-id",
        "run-745-a",
        "--workflow-id",
        "wf-godel-loop",
        "--failure-code",
        "tool_failure",
        "--failure-summary",
        "step failed with deterministic parse error",
        "--evidence-ref",
        "runs/run-745-a/run_status.json",
        "--evidence-ref",
        "runs/run-745-a/logs/activation_log.json",
        "--runs-dir",
        runs_dir.to_str().unwrap(),
    ]);
    assert!(
        run.status.success(),
        "stderr:\n{}",
        String::from_utf8_lossy(&run.stderr)
    );

    let out1 = run_adl(&[
        "godel",
        "inspect",
        "--run-id",
        "run-745-a",
        "--runs-dir",
        runs_dir.to_str().unwrap(),
    ]);
    let out2 = run_adl(&[
        "godel",
        "inspect",
        "--run-id",
        "run-745-a",
        "--runs-dir",
        runs_dir.to_str().unwrap(),
    ]);
    assert!(
        out1.status.success() && out2.status.success(),
        "stderr1:\n{}\nstderr2:\n{}",
        String::from_utf8_lossy(&out1.stderr),
        String::from_utf8_lossy(&out2.stderr)
    );
    assert_eq!(
        out1.stdout, out2.stdout,
        "expected deterministic inspect output"
    );

    let stdout = String::from_utf8_lossy(&out1.stdout);
    let summary: serde_json::Value =
        serde_json::from_str(&stdout).expect("parse godel inspect summary");
    assert_eq!(summary["run_id"], "run-745-a");
    assert_eq!(
        summary["hypothesis_path"],
        "runs/run-745-a/godel/godel_hypothesis.v1.json"
    );
    assert_eq!(
        summary["policy_path"],
        "runs/run-745-a/godel/godel_policy.v1.json"
    );
    assert_eq!(
        summary["policy_comparison_path"],
        "runs/run-745-a/godel/godel_policy_comparison.v1.json"
    );
    assert_eq!(
        summary["prioritization_path"],
        "runs/run-745-a/godel/godel_experiment_priority.v1.json"
    );
    assert_eq!(
        summary["cross_workflow_path"],
        "runs/run-745-a/godel/godel_cross_workflow_learning.v1.json"
    );
    assert_eq!(
        summary["eval_report_path"],
        "runs/run-745-a/godel/godel_eval_report.v1.json"
    );
    assert_eq!(
        summary["promotion_decision_path"],
        "runs/run-745-a/godel/godel_promotion_decision.v1.json"
    );
    assert_eq!(
        summary["canonical_evaluation_plan_path"],
        "runs/run-745-a/godel/evaluation_plan.v1.json"
    );
    assert_eq!(
        summary["canonical_mutation_path"],
        "runs/run-745-a/godel/mutation.v1.json"
    );
    assert_eq!(
        summary["canonical_evidence_path"],
        "runs/run-745-a/godel/canonical_evidence_view.v1.json"
    );
    assert_eq!(
        summary["experiment_record_path"],
        "runs/run-745-a/godel/experiment_record.runtime.v1.json"
    );
    assert_eq!(
        summary["canonical_experiment_record_path"],
        "runs/run-745-a/godel/experiment_record.v1.json"
    );
    assert_eq!(
        summary["obsmem_index_path"],
        "runs/run-745-a/godel/obsmem_index_entry.runtime.v1.json"
    );
    assert_eq!(summary["failure_code"], "tool_failure");
    assert_eq!(summary["workflow_id"], "wf-godel-loop");
    assert_eq!(summary["hypothesis_id"], "hyp:run-745-a:tool_failure:00");
    assert!(summary["hypothesis_claim"]
        .as_str()
        .expect("claim")
        .contains("tool_failure"));
    assert_eq!(summary["policy_id"], "policy:run-745-a:tool_failure");
    assert_eq!(summary["policy_mode_before"], "baseline");
    assert_eq!(summary["policy_mode_after"], "adaptive_reviewed");
    assert_eq!(
        summary["changed_policy_fields"],
        serde_json::json!(["experiment_budget", "policy_mode", "retry_budget"])
    );
    assert_eq!(
        summary["cross_workflow_learning_id"],
        "cross-workflow:run-745-a:exp:retry-budget"
    );
    assert_eq!(
        summary["downstream_workflow_id"],
        "wf-aee-retry-budget-adaptation"
    );
    assert_eq!(
        summary["downstream_decision_id"],
        "decision:run-745-a:exp:retry-budget"
    );
    assert_eq!(
        summary["downstream_decision_class"],
        "cross_workflow_learning_update"
    );
    assert!(summary["downstream_expected_behavior_change"]
        .as_str()
        .expect("behavior change")
        .contains("Apply retry budget 2"));
    assert_eq!(summary["top_experiment_candidate_id"], "exp:retry-budget");
    assert_eq!(summary["top_experiment_confidence"], 0.86);
    assert_eq!(
        summary["prioritization_tie_break_rule"],
        "sort by priority_score desc, then confidence desc, then candidate_id asc"
    );
    assert_eq!(
        summary["evaluation_id"],
        "evaluation:run-745-a:exp:retry-budget"
    );
    assert_eq!(summary["evaluation_score"], 95);
    assert_eq!(
        summary["promotion_id"],
        "promotion:run-745-a:exp:retry-budget"
    );
    assert_eq!(summary["promotion_decision"], "promote");
    assert!(summary["promotion_reason"]
        .as_str()
        .expect("promotion reason")
        .contains("score=95 -> promote"));
    assert_eq!(summary["evaluation_decision"], "adopt");
    assert_eq!(summary["improvement_delta"], 1);
    assert_eq!(
        summary["canonical_evaluation_plan_id"],
        "plan_run_745_a_tool_failure"
    );
    assert_eq!(
        summary["canonical_mutation_id"],
        "mut_mut_run_745_a_tool_failure_00"
    );
    assert_eq!(
        summary["canonical_evidence_view_id"],
        "cev-run-745-a-tool_failure"
    );
    assert_eq!(
        summary["canonical_experiment_id"],
        "exp-run-745-a-mut-run-745-a-tool_failure-00"
    );
    assert_eq!(summary["canonical_decision_result"], "adopt");
    assert!(summary["canonical_decision_rationale"]
        .as_str()
        .expect("canonical rationale")
        .contains("decision=Adopt"));
    assert_eq!(summary["baseline_run_id"], "run-745-a");
    assert_eq!(summary["variant_run_id"], "run-745-a");
    assert_eq!(
        summary["obsmem_index_key"],
        "tool_failure:hyp:run-745-a:tool_failure:00:adopt"
    );
    assert_eq!(summary["experiment_outcome"], "adopt");
}

#[test]
fn godel_evaluate_produces_deterministic_summary() {
    let out1 = run_adl(&[
        "godel",
        "evaluate",
        "--failure-code",
        "tool_failure",
        "--experiment-result",
        "ok",
        "--score-delta",
        "1",
    ]);
    let out2 = run_adl(&[
        "godel",
        "evaluate",
        "--failure-code",
        "tool_failure",
        "--experiment-result",
        "ok",
        "--score-delta",
        "1",
    ]);
    assert!(
        out1.status.success() && out2.status.success(),
        "stderr1:\n{}\nstderr2:\n{}",
        String::from_utf8_lossy(&out1.stderr),
        String::from_utf8_lossy(&out2.stderr)
    );
    assert_eq!(out1.stdout, out2.stdout, "expected deterministic summary");

    let stdout = String::from_utf8_lossy(&out1.stdout);
    let summary: serde_json::Value =
        serde_json::from_str(&stdout).expect("parse godel evaluate summary");
    assert_eq!(summary["failure_code"], "tool_failure");
    assert_eq!(summary["experiment_result"], "ok");
    assert_eq!(summary["score_delta"], 1);
    assert_eq!(summary["decision"], "adopt");
    assert_eq!(
        summary["rationale"],
        "Evaluation for failure_code=tool_failure produced decision=Adopt with score_delta=1."
    );
    assert_eq!(
        summary["evaluation_plan_example"],
        "adl-spec/examples/v0.8/evaluation_plan.v1.example.json"
    );
}

#[test]
fn affect_godel_vertical_slice_demo_emits_changed_strategy_artifact() {
    let demo_root = unique_test_temp_dir("affect-godel-vertical-slice-demo");
    let script = repo_root().join("adl/tools/demo_affect_godel_vertical_slice.sh");
    let runs_root = demo_root.join("aee-runs");
    let out = Command::new("bash")
        .env("ADL_RUNS_ROOT", &runs_root)
        .arg(&script)
        .arg(&demo_root)
        .output()
        .expect("run affect-godel demo");
    assert!(
        out.status.success(),
        "stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );

    let artifact_path =
        demo_root.join("runs/review-godel-affect-001/godel/godel_affect_vertical_slice.v1.json");
    assert!(
        artifact_path.is_file(),
        "missing {}",
        artifact_path.display()
    );

    let artifact: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&artifact_path).expect("read artifact"))
            .expect("parse artifact");
    assert_eq!(
        artifact["downstream_change"]["initial_selected_candidate_id"],
        "exp:retry-budget"
    );
    assert_eq!(
        artifact["downstream_change"]["adapted_selected_candidate_id"],
        "exp:maintain-policy"
    );
    assert_eq!(artifact["downstream_change"]["changed"], true);
}

#[test]
fn bounded_aee_recovery_demo_shows_failure_suggestion_overlay_and_recovery() {
    let demo_root = unique_test_temp_dir("aee-recovery-demo");
    let initial_out = demo_root.join("initial");
    let adapted_out = demo_root.join("adapted");
    let initial_state = demo_root.join("state-initial");
    let adapted_state = demo_root.join("state-adapted");
    let initial_yaml = fixture_path("examples/v0-3-aee-recovery-initial.adl.yaml");
    let adapted_yaml = fixture_path("examples/v0-3-aee-recovery-adapted.adl.yaml");
    let overlay = repo_root().join("demos/aee-recovery/retry-budget.overlay.json");
    let mock = fixture_path("tools/mock_ollama_fail_once.sh");
    let runs_root = demo_root.join("runs");
    let initial_run = runs_root.join("v0-3-aee-recovery-initial");
    let adapted_run = runs_root.join("v0-3-aee-recovery-adapted");

    let _ = fs::remove_dir_all(&initial_run);
    let _ = fs::remove_dir_all(&adapted_run);
    let _ = fs::remove_dir_all(&initial_out);
    let _ = fs::remove_dir_all(&adapted_out);

    let initial = run_adl_with_env(
        &[
            initial_yaml.to_str().unwrap(),
            "--run",
            "--trace",
            "--out",
            initial_out.to_str().unwrap(),
        ],
        &[
            ("ADL_OLLAMA_BIN", mock.to_str().unwrap()),
            ("ADL_AEE_DEMO_STATE_DIR", initial_state.to_str().unwrap()),
            ("ADL_RUNS_ROOT", runs_root.to_str().unwrap()),
        ],
    );
    assert!(
        !initial.status.success(),
        "expected initial failure, stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&initial.stdout),
        String::from_utf8_lossy(&initial.stderr)
    );
    let initial_stderr = String::from_utf8_lossy(&initial.stderr);
    assert!(
        initial_stderr.contains("attempt 1/1")
            && initial_stderr.contains("mock aee demo transient failure"),
        "stderr:\n{initial_stderr}"
    );

    let suggestions_path = initial_run.join("learning/suggestions.json");
    let affect_path = initial_run.join("learning/affect_state.v1.json");
    let decision_path = initial_run.join("learning/aee_decision.json");
    let graph_path = initial_run.join("learning/reasoning_graph.v1.json");
    let suggestions: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(&suggestions_path).expect("read initial suggestions"),
    )
    .expect("parse suggestions");
    let affect_json: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&affect_path).expect("read affect artifact"))
            .expect("parse affect artifact");
    let decision_json: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&decision_path).expect("read aee decision"))
            .expect("parse aee decision");
    let graph_json: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&graph_path).expect("read reasoning graph"))
            .expect("parse reasoning graph");
    let intents: Vec<&str> = suggestions["suggestions"]
        .as_array()
        .expect("suggestions array")
        .iter()
        .filter_map(|s| {
            s.get("proposed_change")
                .and_then(|p| p.get("intent"))
                .and_then(|v| v.as_str())
        })
        .collect();
    assert!(
        intents.contains(&"increase_step_retry_budget"),
        "suggestions:\n{}",
        serde_json::to_string_pretty(&suggestions).unwrap()
    );
    assert_eq!(affect_json["affect"]["affect_mode"], "recovery_focus");
    assert_eq!(affect_json["affect"]["recovery_bias"], 2);
    assert_eq!(
        decision_json["affect_state"]["affect_mode"],
        "recovery_focus"
    );
    assert_eq!(decision_json["decision"]["recommended_retry_budget"], 2);
    assert_eq!(
        graph_json["graph"]["dominant_affect_mode"],
        "recovery_focus"
    );
    assert_eq!(
        graph_json["graph"]["selected_path"]["selected_node_id"],
        "action.retry_budget"
    );

    let initial_replay = run_adl(&[
        "instrument",
        "replay",
        initial_run
            .join("logs/activation_log.json")
            .to_str()
            .unwrap(),
    ]);
    assert!(
        initial_replay.status.success(),
        "replay stderr:\n{}",
        String::from_utf8_lossy(&initial_replay.stderr)
    );

    let adapted = run_adl_with_env(
        &[
            adapted_yaml.to_str().unwrap(),
            "--run",
            "--trace",
            "--overlay",
            overlay.to_str().unwrap(),
            "--out",
            adapted_out.to_str().unwrap(),
        ],
        &[
            ("ADL_OLLAMA_BIN", mock.to_str().unwrap()),
            ("ADL_AEE_DEMO_STATE_DIR", adapted_state.to_str().unwrap()),
            ("ADL_RUNS_ROOT", runs_root.to_str().unwrap()),
        ],
    );
    assert!(
        adapted.status.success(),
        "expected adapted success, stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&adapted.stdout),
        String::from_utf8_lossy(&adapted.stderr)
    );

    let overlay_audit = adapted_run.join("learning/overlays/applied_overlay.json");
    let overlay_source = adapted_run.join("learning/overlays/source_overlay.json");
    let adapted_affect_path = adapted_run.join("learning/affect_state.v1.json");
    let adapted_graph_path = adapted_run.join("learning/reasoning_graph.v1.json");
    assert!(
        overlay_audit.is_file(),
        "missing {}",
        overlay_audit.display()
    );
    assert!(
        overlay_source.is_file(),
        "missing {}",
        overlay_source.display()
    );
    assert!(
        adapted_affect_path.is_file(),
        "missing {}",
        adapted_affect_path.display()
    );
    assert!(
        adapted_graph_path.is_file(),
        "missing {}",
        adapted_graph_path.display()
    );

    let summary_json: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(adapted_run.join("run_summary.json")).unwrap())
            .expect("parse run_summary");
    let adapted_graph_json: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&adapted_graph_path).expect("read adapted graph"))
            .expect("parse adapted graph");
    assert_eq!(summary_json["status"], "success");
    assert_eq!(
        adapted_graph_json["graph"]["dominant_affect_mode"],
        "steady_state"
    );
    assert_eq!(
        adapted_graph_json["graph"]["selected_path"]["selected_node_id"],
        "action.maintain_policy"
    );

    let adapted_replay = run_adl(&[
        "instrument",
        "replay",
        adapted_run
            .join("logs/activation_log.json")
            .to_str()
            .unwrap(),
    ]);
    assert!(
        adapted_replay.status.success(),
        "replay stderr:\n{}",
        String::from_utf8_lossy(&adapted_replay.stderr)
    );

    let _ = fs::remove_dir_all(&initial_run);
    let _ = fs::remove_dir_all(&adapted_run);
}
