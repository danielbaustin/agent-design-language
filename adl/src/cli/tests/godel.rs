use super::*;

#[test]
fn real_godel_validates_subcommand_and_run_args() {
    let err = real_godel(&[]).expect_err("missing subcommand");
    assert!(err
        .to_string()
        .contains("supported: run, evaluate, inspect, affect-slice"));

    let err = real_godel(&["unknown".to_string()]).expect_err("unknown subcommand");
    assert!(err.to_string().contains("unknown godel subcommand"));

    let err = real_godel_run(&[]).expect_err("missing run-id");
    assert!(err.to_string().contains("requires --run-id"));

    let err = real_godel_run(&[
        "--run-id".to_string(),
        "run-745-a".to_string(),
        "--workflow-id".to_string(),
        "wf-godel-loop".to_string(),
        "--failure-code".to_string(),
        "tool_failure".to_string(),
        "--failure-summary".to_string(),
        "deterministic parse error".to_string(),
        "--evidence-ref".to_string(),
        "../bad.json".to_string(),
    ])
    .expect_err("unsafe evidence ref should fail");
    assert!(err.to_string().contains("GODEL_STAGE_LOOP_INVALID_INPUT"));
}

#[test]
fn real_godel_run_rejects_missing_value_flags() {
    let cases = [
        (vec!["--run-id"], "--run-id requires a value"),
        (vec!["--workflow-id"], "--workflow-id requires a value"),
        (vec!["--failure-code"], "--failure-code requires a value"),
        (
            vec!["--failure-summary"],
            "--failure-summary requires a value",
        ),
        (
            vec!["--evidence-ref"],
            "--evidence-ref requires a relative path",
        ),
        (vec!["--runs-dir"], "--runs-dir requires a directory path"),
    ];

    for (args, needle) in cases {
        let args = args.into_iter().map(|s| s.to_string()).collect::<Vec<_>>();
        let err = real_godel_run(&args).expect_err("missing value flag should fail");
        assert!(
            err.to_string().contains(needle),
            "args={args:?}\nerr={}",
            err
        );
    }
}

#[test]
fn real_godel_inspect_validates_args_and_missing_paths() {
    let err = real_godel_inspect(&[]).expect_err("missing run-id");
    assert!(err.to_string().contains("requires --run-id"));

    let err =
        real_godel_inspect(&["--bogus".to_string(), "x".to_string()]).expect_err("unknown arg");
    assert!(err.to_string().contains("unknown godel inspect arg"));

    let missing_root =
        std::env::temp_dir().join(format!("adl-godel-inspect-missing-{}", std::process::id()));
    let err = real_godel_inspect(&[
        "--run-id".to_string(),
        "run-745-a".to_string(),
        "--runs-dir".to_string(),
        missing_root.to_string_lossy().to_string(),
    ])
    .expect_err("missing artifacts");
    assert!(err.to_string().contains("GODEL_INSPECT_IO"));
}

#[test]
fn real_godel_inspect_rejects_missing_value_flags() {
    let err = real_godel_inspect(&["--run-id".to_string()]).expect_err("missing run-id value");
    assert!(err.to_string().contains("--run-id requires a value"));

    let err = real_godel_inspect(&[
        "--run-id".to_string(),
        "run-745-a".to_string(),
        "--runs-dir".to_string(),
    ])
    .expect_err("missing runs-dir value");
    assert!(err
        .to_string()
        .contains("--runs-dir requires a directory path"));
}

#[test]
fn real_godel_affect_slice_rejects_missing_value_flags() {
    let cases = [
        (
            vec!["--initial-run-id"],
            "--initial-run-id requires a value",
        ),
        (
            vec!["--adapted-run-id"],
            "--adapted-run-id requires a value",
        ),
        (vec!["--godel-run-id"], "--godel-run-id requires a value"),
        (
            vec!["--aee-runs-dir"],
            "--aee-runs-dir requires a directory path",
        ),
        (
            vec!["--godel-runs-dir"],
            "--godel-runs-dir requires a directory path",
        ),
    ];

    for (args, needle) in cases {
        let args = args.into_iter().map(|s| s.to_string()).collect::<Vec<_>>();
        let err = real_godel_affect_slice(&args).expect_err("missing value flag should fail");
        assert!(err.to_string().contains(needle), "err={err}");
    }
}

#[test]
fn real_godel_affect_slice_rejects_missing_required_args() {
    let missing_initial = real_godel_affect_slice(&[
        "--adapted-run-id".to_string(),
        "run-b".to_string(),
        "--godel-run-id".to_string(),
        "run-c".to_string(),
    ])
    .expect_err("missing initial run id should fail");
    assert!(missing_initial
        .to_string()
        .contains("godel affect-slice requires --initial-run-id <id>"));

    let missing_adapted = real_godel_affect_slice(&[
        "--initial-run-id".to_string(),
        "run-a".to_string(),
        "--godel-run-id".to_string(),
        "run-c".to_string(),
    ])
    .expect_err("missing adapted run id should fail");
    assert!(missing_adapted
        .to_string()
        .contains("godel affect-slice requires --adapted-run-id <id>"));

    let missing_godel = real_godel_affect_slice(&[
        "--initial-run-id".to_string(),
        "run-a".to_string(),
        "--adapted-run-id".to_string(),
        "run-b".to_string(),
    ])
    .expect_err("missing godel run id should fail");
    assert!(missing_godel
        .to_string()
        .contains("godel affect-slice requires --godel-run-id <id>"));
}

#[test]
fn real_godel_affect_slice_persists_vertical_slice_artifact() {
    let base = std::env::temp_dir().join(format!("adl-godel-affect-slice-{}", std::process::id()));
    let aee_root = base.join("aee-runs");
    let godel_root = base.join("godel-runs");
    std::fs::create_dir_all(aee_root.join("v0-3-aee-recovery-initial/learning"))
        .expect("create initial learning dir");
    std::fs::create_dir_all(aee_root.join("v0-3-aee-recovery-adapted/learning"))
        .expect("create adapted learning dir");
    std::fs::create_dir_all(godel_root.join("run-745-a/godel")).expect("create godel dir");

    std::fs::write(
        aee_root.join("v0-3-aee-recovery-initial/learning/affect_state.v1.json"),
        serde_json::to_string_pretty(&serde_json::json!({
            "run_id": "v0-3-aee-recovery-initial",
            "affect": {
                "affect_state_id": "affect-initial",
                "affect_mode": "recovery_focus",
                "recovery_bias": 2,
                "downstream_priority": "retry_recovery",
                "update_reason": "deterministic failure evidence"
            }
        }))
        .unwrap(),
    )
    .unwrap();
    std::fs::write(
        aee_root.join("v0-3-aee-recovery-initial/learning/reasoning_graph.v1.json"),
        serde_json::to_string_pretty(&serde_json::json!({
            "run_id": "v0-3-aee-recovery-initial",
            "graph": {
                "dominant_affect_mode": "recovery_focus",
                "ranking_rule": "sort by priority_score desc, then node_id asc",
                "selected_path": {
                    "selected_node_id": "action.retry_budget",
                    "selected_intent": "increase_step_retry_budget",
                    "selected_target": "workflow-runtime",
                    "graph_derived_output": "retry budget experiment",
                    "affect_changed_ranking": true
                },
                "nodes": [
                    {"node_id": "action.retry_budget", "node_kind": "action", "rank": 1, "priority_score": 92},
                    {"node_id": "action.maintain_policy", "node_kind": "action", "rank": 2, "priority_score": 36}
                ]
            }
        }))
        .unwrap(),
    )
    .unwrap();
    std::fs::write(
        aee_root.join("v0-3-aee-recovery-adapted/learning/affect_state.v1.json"),
        serde_json::to_string_pretty(&serde_json::json!({
            "run_id": "v0-3-aee-recovery-adapted",
            "affect": {
                "affect_state_id": "affect-adapted",
                "affect_mode": "steady_state",
                "recovery_bias": 0,
                "downstream_priority": "maintain_current_policy",
                "update_reason": "deterministic adapted rerun"
            }
        }))
        .unwrap(),
    )
    .unwrap();
    std::fs::write(
        aee_root.join("v0-3-aee-recovery-adapted/learning/reasoning_graph.v1.json"),
        serde_json::to_string_pretty(&serde_json::json!({
            "run_id": "v0-3-aee-recovery-adapted",
            "graph": {
                "dominant_affect_mode": "steady_state",
                "ranking_rule": "sort by priority_score desc, then node_id asc",
                "selected_path": {
                    "selected_node_id": "action.maintain_policy",
                    "selected_intent": "maintain_current_policy",
                    "selected_target": "workflow-runtime",
                    "graph_derived_output": "maintain policy review",
                    "affect_changed_ranking": false
                },
                "nodes": [
                    {"node_id": "action.maintain_policy", "node_kind": "action", "rank": 1, "priority_score": 88},
                    {"node_id": "action.retry_budget", "node_kind": "action", "rank": 2, "priority_score": 22}
                ]
            }
        }))
        .unwrap(),
    )
    .unwrap();

    std::fs::write(
        godel_root.join("run-745-a/godel/godel_hypothesis.v1.json"),
        serde_json::to_string_pretty(&serde_json::json!({
            "artifact_version": "godel_hypothesis.v1",
            "hypothesis_id": "hyp:run-745-a:tool_failure:00",
            "run_id": "run-745-a",
            "workflow_id": "wf-godel-loop",
            "failure_id": "failure:run-745-a:tool_failure",
            "failure_class": "tool_failure",
            "claim": "Primary hypothesis",
            "confidence": 0.67,
            "evidence_refs": ["runs/run-745-a/run_status.json"],
            "related_run_refs": ["run-745-a"]
        }))
        .unwrap(),
    )
    .unwrap();
    std::fs::write(
        godel_root.join("run-745-a/godel/godel_policy.v1.json"),
        serde_json::to_string_pretty(&serde_json::json!({
            "artifact_version": "godel_policy.v1",
            "policy_id": "policy:run-745-a:tool_failure",
            "run_id": "run-745-a",
            "workflow_id": "wf-godel-loop",
            "hypothesis_id": "hyp:run-745-a:tool_failure:00",
            "hypothesis_artifact_path": "runs/run-745-a/godel/godel_hypothesis.v1.json",
            "source_signal": "hypothesis:tool_failure:godel_hypothesis.v1",
            "selection_reason": "Deterministic policy update",
            "before_policy": {
                "retry_budget": 1,
                "experiment_budget": 1,
                "target_surface": "tool-invocation-config",
                "policy_mode": "baseline"
            },
            "after_policy": {
                "retry_budget": 2,
                "experiment_budget": 2,
                "target_surface": "tool-invocation-config",
                "policy_mode": "adaptive_reviewed"
            }
        }))
        .unwrap(),
    )
    .unwrap();
    std::fs::write(
        godel_root.join("run-745-a/godel/godel_experiment_priority.v1.json"),
        serde_json::to_string_pretty(&serde_json::json!({
            "artifact_version": "godel_experiment_priority.v1",
            "prioritization_id": "prioritize:run-745-a:tool_failure",
            "run_id": "run-745-a",
            "workflow_id": "wf-godel-loop",
            "hypothesis_id": "hyp:run-745-a:tool_failure:00",
            "policy_id": "policy:run-745-a:tool_failure",
            "hypothesis_artifact_path": "runs/run-745-a/godel/godel_hypothesis.v1.json",
            "policy_artifact_path": "runs/run-745-a/godel/godel_policy.v1.json",
            "tie_break_rule": "sort by priority_score desc, then confidence desc, then candidate_id asc",
            "input_candidates": [
                {"candidate_id": "exp:retry-budget", "strategy": "retry_budget_probe", "target_surface": "tool-invocation-config"}
            ],
            "ranked_candidates": [
                {"candidate_id": "exp:retry-budget", "strategy": "retry_budget_probe", "target_surface": "tool-invocation-config", "priority_score": 95, "confidence": 0.86, "ranking_reason": "deterministic"}
            ]
        }))
        .unwrap(),
    )
    .unwrap();

    real_godel_affect_slice(&[
        "--initial-run-id".to_string(),
        "v0-3-aee-recovery-initial".to_string(),
        "--adapted-run-id".to_string(),
        "v0-3-aee-recovery-adapted".to_string(),
        "--godel-run-id".to_string(),
        "run-745-a".to_string(),
        "--aee-runs-dir".to_string(),
        aee_root.to_string_lossy().to_string(),
        "--godel-runs-dir".to_string(),
        godel_root.to_string_lossy().to_string(),
    ])
    .expect("run affect slice");

    let persisted = godel_root.join("run-745-a/godel/godel_affect_vertical_slice.v1.json");
    assert!(persisted.is_file());
    let artifact: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&persisted).unwrap()).unwrap();
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
fn real_godel_inspect_reads_persisted_runtime_artifacts() {
    let base = std::env::temp_dir().join(format!("adl-godel-inspect-ok-{}", std::process::id()));
    let run_dir = base.join("run-745-a").join("godel");
    std::fs::create_dir_all(&run_dir).expect("create godel dir");

    let record = PersistedExperimentRecord {
        schema: EXPERIMENT_RECORD_RUNTIME_SCHEMA.to_string(),
        record: StageExperimentRecord {
            run_id: "run-745-a".to_string(),
            workflow_id: "wf-godel-loop".to_string(),
            failure_code: "tool_failure".to_string(),
            hypothesis_id: "hyp:run-745-a:tool_failure:00".to_string(),
            mutation_id: "mut:run-745-a:tool_failure:00".to_string(),
            mutation_target_surface: "workflow-step-config".to_string(),
            evaluation_decision: "adopt".to_string(),
            evaluation_rationale: "deterministic rationale".to_string(),
            improvement_delta: 1,
            evidence_refs: vec!["runs/run-745-a/run_status.json".to_string()],
        },
    };
    let index = PersistedStageIndexEntry {
        schema: OBSMEM_INDEX_RUNTIME_SCHEMA.to_string(),
        entry: StageIndexEntry {
            index_key: "tool_failure:hyp:run-745-a:tool_failure:00:adopt".to_string(),
            run_id: "run-745-a".to_string(),
            workflow_id: "wf-godel-loop".to_string(),
            failure_code: "tool_failure".to_string(),
            hypothesis_id: "hyp:run-745-a:tool_failure:00".to_string(),
            mutation_id: "mut:run-745-a:tool_failure:00".to_string(),
            experiment_outcome: "adopt".to_string(),
        },
    };
    let hypothesis = PersistedHypothesisArtifact {
        artifact_version: HYPOTHESIS_ARTIFACT_VERSION.to_string(),
        hypothesis_id: "hyp:run-745-a:tool_failure:00".to_string(),
        run_id: "run-745-a".to_string(),
        workflow_id: "wf-godel-loop".to_string(),
        failure_id: "failure:run-745-a:tool_failure".to_string(),
        failure_class: "tool_failure".to_string(),
        claim: "Primary hypothesis: failure_code=tool_failure indicates a bounded execution weakness derived from 'deterministic parse failure'.".to_string(),
        confidence: 0.67,
        evidence_refs: vec!["runs/run-745-a/run_status.json".to_string()],
        related_run_refs: vec!["run-745-a".to_string()],
    };
    let policy = PersistedPolicyArtifact {
        artifact_version: POLICY_ARTIFACT_VERSION.to_string(),
        policy_id: "policy:run-745-a:tool_failure".to_string(),
        run_id: "run-745-a".to_string(),
        workflow_id: "wf-godel-loop".to_string(),
        hypothesis_id: "hyp:run-745-a:tool_failure:00".to_string(),
        hypothesis_artifact_path: "runs/run-745-a/godel/godel_hypothesis.v1.json".to_string(),
        source_signal: "hypothesis:tool_failure:godel_hypothesis.v1".to_string(),
        selection_reason: "Deterministic policy update derived from hypothesis_id=hyp:run-745-a:tool_failure:00 and failure_class=tool_failure.".to_string(),
        before_policy: ::adl::godel::policy::PolicyState {
            retry_budget: 1,
            experiment_budget: 1,
            target_surface: "tool-invocation-config".to_string(),
            policy_mode: "baseline".to_string(),
        },
        after_policy: ::adl::godel::policy::PolicyState {
            retry_budget: 2,
            experiment_budget: 2,
            target_surface: "tool-invocation-config".to_string(),
            policy_mode: "adaptive_reviewed".to_string(),
        },
    };
    let comparison = PersistedPolicyComparisonArtifact {
        artifact_version: POLICY_COMPARISON_ARTIFACT_VERSION.to_string(),
        comparison_id: "cmp:run-745-a:tool_failure".to_string(),
        run_id: "run-745-a".to_string(),
        workflow_id: "wf-godel-loop".to_string(),
        policy_id: "policy:run-745-a:tool_failure".to_string(),
        hypothesis_id: "hyp:run-745-a:tool_failure:00".to_string(),
        changed_fields: vec![
            "experiment_budget".to_string(),
            "policy_mode".to_string(),
            "retry_budget".to_string(),
        ],
        deterministic_mapping:
            "stable failure_class -> baseline policy -> bounded policy adjustment".to_string(),
        before_policy: policy.before_policy.clone(),
        after_policy: policy.after_policy.clone(),
    };
    let prioritization = PersistedPrioritizationArtifact {
        artifact_version: PRIORITIZATION_ARTIFACT_VERSION.to_string(),
        prioritization_id: "prioritize:run-745-a:tool_failure".to_string(),
        run_id: "run-745-a".to_string(),
        workflow_id: "wf-godel-loop".to_string(),
        hypothesis_id: "hyp:run-745-a:tool_failure:00".to_string(),
        policy_id: "policy:run-745-a:tool_failure".to_string(),
        hypothesis_artifact_path: "runs/run-745-a/godel/godel_hypothesis.v1.json".to_string(),
        policy_artifact_path: "runs/run-745-a/godel/godel_policy.v1.json".to_string(),
        tie_break_rule: "sort by priority_score desc, then confidence desc, then candidate_id asc"
            .to_string(),
        input_candidates: vec![
            ::adl::godel::prioritization::PrioritizationInputCandidate {
                candidate_id: "exp:fallback-check".to_string(),
                strategy: "fallback_surface_probe".to_string(),
                target_surface: "tool-invocation-config".to_string(),
            },
            ::adl::godel::prioritization::PrioritizationInputCandidate {
                candidate_id: "exp:parser-guardrail".to_string(),
                strategy: "parser_guardrail_probe".to_string(),
                target_surface: "tool-invocation-config".to_string(),
            },
            ::adl::godel::prioritization::PrioritizationInputCandidate {
                candidate_id: "exp:retry-budget".to_string(),
                strategy: "retry_budget_probe".to_string(),
                target_surface: "tool-invocation-config".to_string(),
            },
        ],
        ranked_candidates: vec![
            ::adl::godel::prioritization::RankedExperimentCandidate {
                candidate_id: "exp:retry-budget".to_string(),
                strategy: "retry_budget_probe".to_string(),
                target_surface: "tool-invocation-config".to_string(),
                priority_score: 95,
                confidence: 0.86,
                ranking_reason: "deterministic".to_string(),
            },
            ::adl::godel::prioritization::RankedExperimentCandidate {
                candidate_id: "exp:parser-guardrail".to_string(),
                strategy: "parser_guardrail_probe".to_string(),
                target_surface: "tool-invocation-config".to_string(),
                priority_score: 79,
                confidence: 0.74,
                ranking_reason: "deterministic".to_string(),
            },
            ::adl::godel::prioritization::RankedExperimentCandidate {
                candidate_id: "exp:fallback-check".to_string(),
                strategy: "fallback_surface_probe".to_string(),
                target_surface: "tool-invocation-config".to_string(),
                priority_score: 68,
                confidence: 0.68,
                ranking_reason: "deterministic".to_string(),
            },
        ],
    };
    let cross_workflow = PersistedCrossWorkflowArtifact {
        artifact_version: CROSS_WORKFLOW_ARTIFACT_VERSION.to_string(),
        learning_id: "cross-workflow:run-745-a:exp:retry-budget".to_string(),
        source_run_id: "run-745-a".to_string(),
        source_workflow_id: "wf-godel-loop".to_string(),
        source_hypothesis_id: "hyp:run-745-a:tool_failure:00".to_string(),
        source_policy_id: "policy:run-745-a:tool_failure".to_string(),
        source_prioritization_id: "prioritize:run-745-a:tool_failure".to_string(),
        source_hypothesis_artifact_path: "runs/run-745-a/godel/godel_hypothesis.v1.json"
            .to_string(),
        source_policy_artifact_path: "runs/run-745-a/godel/godel_policy.v1.json".to_string(),
        source_prioritization_artifact_path:
            "runs/run-745-a/godel/godel_experiment_priority.v1.json".to_string(),
        linkage_rule:
            "consume highest-ranked experiment candidate and map strategy to downstream workflow decision"
                .to_string(),
        downstream_decision: DownstreamWorkflowDecision {
            workflow_id: "wf-aee-retry-budget-adaptation".to_string(),
            decision_id: "decision:run-745-a:exp:retry-budget".to_string(),
            selected_candidate_id: "exp:retry-budget".to_string(),
            selected_strategy: "retry_budget_probe".to_string(),
            decision_class: "cross_workflow_learning_update".to_string(),
            expected_behavior_change:
                "Apply retry budget 2 to downstream recovery workflow for failure_class=tool_failure."
                    .to_string(),
        },
    };
    let eval_report = PersistedEvalReportArtifact {
        artifact_version: EVAL_REPORT_ARTIFACT_VERSION.to_string(),
        evaluation_id: "evaluation:run-745-a:exp:retry-budget".to_string(),
        run_id: "run-745-a".to_string(),
        workflow_id: "wf-aee-retry-budget-adaptation".to_string(),
        hypothesis_id: "hyp:run-745-a:tool_failure:00".to_string(),
        policy_id: "policy:run-745-a:tool_failure".to_string(),
        prioritization_id: "prioritize:run-745-a:tool_failure".to_string(),
        cross_workflow_learning_id: "cross-workflow:run-745-a:exp:retry-budget".to_string(),
        score: 95,
        confidence_basis:
            "failure_class=tool_failure, policy_mode=adaptive_reviewed, ranked_candidate=exp:retry-budget, confidence=0.86".to_string(),
        report:
            "Evaluate downstream workflow wf-aee-retry-budget-adaptation using ranked candidate exp:retry-budget and retry budget 2."
                .to_string(),
        hypothesis_artifact_path: "runs/run-745-a/godel/godel_hypothesis.v1.json".to_string(),
        policy_artifact_path: "runs/run-745-a/godel/godel_policy.v1.json".to_string(),
        prioritization_artifact_path:
            "runs/run-745-a/godel/godel_experiment_priority.v1.json".to_string(),
        cross_workflow_artifact_path:
            "runs/run-745-a/godel/godel_cross_workflow_learning.v1.json".to_string(),
    };
    let promotion_decision = PersistedPromotionDecisionArtifact {
        artifact_version: PROMOTION_DECISION_ARTIFACT_VERSION.to_string(),
        promotion_id: "promotion:run-745-a:exp:retry-budget".to_string(),
        run_id: "run-745-a".to_string(),
        workflow_id: "wf-aee-retry-budget-adaptation".to_string(),
        evaluation_id: "evaluation:run-745-a:exp:retry-budget".to_string(),
        decision: "promote".to_string(),
        decision_reason: "Deterministic threshold decision: score=95 -> promote".to_string(),
        evaluation_artifact_path: "runs/run-745-a/godel/godel_eval_report.v1.json".to_string(),
    };
    let canonical_input = ::adl::godel::StageLoopInput {
        run_id: "run-745-a".to_string(),
        workflow_id: "wf-godel-loop".to_string(),
        failure_code: "tool_failure".to_string(),
        failure_summary: "deterministic parse error".to_string(),
        evidence_refs: vec!["runs/run-745-a/run_status.json".to_string()],
    };
    let mutation = ::adl::godel::mutation::MutationProposal {
        id: "mut:run-745-a:tool_failure:00".to_string(),
        hypothesis_id: "hyp:run-745-a:tool_failure:00".to_string(),
        target_surface: "workflow-step-config".to_string(),
        bounded_change: "Apply deterministic bounded workflow-step adjustment.".to_string(),
    };
    let canonical_mutation = ::adl::godel::mutation::build_canonical_mutation(
        "run-745-a",
        "wf-godel-loop",
        "tool_failure",
        &::adl::godel::hypothesis::HypothesisCandidate {
            id: "hyp:run-745-a:tool_failure:00".to_string(),
            statement: hypothesis.claim.clone(),
            failure_code: "tool_failure".to_string(),
            evidence_refs: vec!["runs/run-745-a/run_status.json".to_string()],
        },
        &mutation,
    )
    .expect("build canonical mutation");
    let canonical_evidence =
        ::adl::godel::canonical_evidence::build_canonical_evidence(&canonical_input)
            .expect("build canonical evidence");
    let canonical_evaluation_plan = ::adl::godel::evaluation::build_canonical_evaluation_plan(
        "run-745-a",
        "wf-godel-loop",
        "tool_failure",
        &canonical_input.evidence_refs,
        &::adl::godel::hypothesis::HypothesisCandidate {
            id: "hyp:run-745-a:tool_failure:00".to_string(),
            statement: hypothesis.claim.clone(),
            failure_code: "tool_failure".to_string(),
            evidence_refs: vec!["runs/run-745-a/run_status.json".to_string()],
        },
        &mutation,
    )
    .expect("build canonical evaluation plan");
    let runtime_record_rel =
        std::path::PathBuf::from("runs/run-745-a/godel/experiment_record.runtime.v1.json");
    let index_rel =
        std::path::PathBuf::from("runs/run-745-a/godel/obsmem_index_entry.runtime.v1.json");

    std::fs::write(
        run_dir.join("experiment_record.runtime.v1.json"),
        serde_json::to_string_pretty(&record).expect("record json"),
    )
    .expect("write record");
    std::fs::write(
        run_dir.join("godel_hypothesis.v1.json"),
        serde_json::to_string_pretty(&hypothesis).expect("hypothesis json"),
    )
    .expect("write hypothesis");
    std::fs::write(
        run_dir.join("godel_policy.v1.json"),
        serde_json::to_string_pretty(&policy).expect("policy json"),
    )
    .expect("write policy");
    std::fs::write(
        run_dir.join("godel_policy_comparison.v1.json"),
        serde_json::to_string_pretty(&comparison).expect("comparison json"),
    )
    .expect("write comparison");
    std::fs::write(
        run_dir.join("godel_experiment_priority.v1.json"),
        serde_json::to_string_pretty(&prioritization).expect("prioritization json"),
    )
    .expect("write prioritization");
    std::fs::write(
        run_dir.join("godel_cross_workflow_learning.v1.json"),
        serde_json::to_string_pretty(&cross_workflow).expect("cross-workflow json"),
    )
    .expect("write cross-workflow");
    std::fs::write(
        run_dir.join("godel_eval_report.v1.json"),
        serde_json::to_string_pretty(&eval_report).expect("eval report json"),
    )
    .expect("write eval report");
    std::fs::write(
        run_dir.join("godel_promotion_decision.v1.json"),
        serde_json::to_string_pretty(&promotion_decision).expect("promotion json"),
    )
    .expect("write promotion decision");
    std::fs::write(
        run_dir.join("obsmem_index_entry.runtime.v1.json"),
        serde_json::to_string_pretty(&index).expect("index json"),
    )
    .expect("write index");
    ::adl::godel::mutation::persist_canonical_mutation(&base, "run-745-a", &canonical_mutation)
        .expect("write canonical mutation");
    ::adl::godel::canonical_evidence::persist_canonical_evidence(&base, &canonical_evidence)
        .expect("write canonical evidence");
    ::adl::godel::evaluation::persist_canonical_evaluation_plan(
        &base,
        "run-745-a",
        &canonical_evaluation_plan,
    )
    .expect("write canonical evaluation plan");
    let canonical_record = ::adl::godel::experiment_record::build_canonical_record(
        &base,
        &record.record,
        &runtime_record_rel,
        &index_rel,
    )
    .expect("build canonical record");
    ::adl::godel::experiment_record::persist_canonical_record(&base, &canonical_record)
        .expect("write canonical record");

    real_godel_inspect(&[
        "--run-id".to_string(),
        "run-745-a".to_string(),
        "--runs-dir".to_string(),
        base.to_string_lossy().to_string(),
    ])
    .expect("inspect should succeed");

    let write_all = || {
        std::fs::write(
            run_dir.join("experiment_record.runtime.v1.json"),
            serde_json::to_string_pretty(&record).expect("record json"),
        )
        .expect("write record");
        std::fs::write(
            run_dir.join("godel_hypothesis.v1.json"),
            serde_json::to_string_pretty(&hypothesis).expect("hypothesis json"),
        )
        .expect("write hypothesis");
        std::fs::write(
            run_dir.join("godel_policy.v1.json"),
            serde_json::to_string_pretty(&policy).expect("policy json"),
        )
        .expect("write policy");
        std::fs::write(
            run_dir.join("godel_policy_comparison.v1.json"),
            serde_json::to_string_pretty(&comparison).expect("comparison json"),
        )
        .expect("write comparison");
        std::fs::write(
            run_dir.join("godel_experiment_priority.v1.json"),
            serde_json::to_string_pretty(&prioritization).expect("prioritization json"),
        )
        .expect("write prioritization");
        std::fs::write(
            run_dir.join("godel_cross_workflow_learning.v1.json"),
            serde_json::to_string_pretty(&cross_workflow).expect("cross-workflow json"),
        )
        .expect("write cross-workflow");
        std::fs::write(
            run_dir.join("godel_eval_report.v1.json"),
            serde_json::to_string_pretty(&eval_report).expect("eval report json"),
        )
        .expect("write eval report");
        std::fs::write(
            run_dir.join("godel_promotion_decision.v1.json"),
            serde_json::to_string_pretty(&promotion_decision).expect("promotion decision json"),
        )
        .expect("write promotion decision");
        std::fs::write(
            run_dir.join("obsmem_index_entry.runtime.v1.json"),
            serde_json::to_string_pretty(&index).expect("index json"),
        )
        .expect("write index");
        ::adl::godel::mutation::persist_canonical_mutation(&base, "run-745-a", &canonical_mutation)
            .expect("write canonical mutation");
        ::adl::godel::canonical_evidence::persist_canonical_evidence(&base, &canonical_evidence)
            .expect("write canonical evidence");
        ::adl::godel::evaluation::persist_canonical_evaluation_plan(
            &base,
            "run-745-a",
            &canonical_evaluation_plan,
        )
        .expect("write canonical evaluation plan");
        let canonical_record = ::adl::godel::experiment_record::build_canonical_record(
            &base,
            &record.record,
            &runtime_record_rel,
            &index_rel,
        )
        .expect("build canonical record");
        ::adl::godel::experiment_record::persist_canonical_record(&base, &canonical_record)
            .expect("write canonical record");
    };

    let parse_cases = [
        ("godel_hypothesis.v1.json", "{", "GODEL_INSPECT_INVALID"),
        (
            "experiment_record.runtime.v1.json",
            "{",
            "GODEL_INSPECT_INVALID",
        ),
        ("godel_policy.v1.json", "{", "GODEL_INSPECT_INVALID"),
        (
            "godel_policy_comparison.v1.json",
            "{",
            "GODEL_INSPECT_INVALID",
        ),
        (
            "godel_experiment_priority.v1.json",
            "{",
            "GODEL_INSPECT_INVALID",
        ),
        (
            "godel_cross_workflow_learning.v1.json",
            "{",
            "GODEL_INSPECT_INVALID",
        ),
        ("godel_eval_report.v1.json", "{", "GODEL_INSPECT_INVALID"),
        (
            "godel_promotion_decision.v1.json",
            "{",
            "GODEL_INSPECT_INVALID",
        ),
        ("evaluation_plan.v1.json", "{", "GODEL_INSPECT_INVALID"),
        ("mutation.v1.json", "{", "GODEL_INSPECT_INVALID"),
        (
            "canonical_evidence_view.v1.json",
            "{",
            "GODEL_INSPECT_INVALID",
        ),
        ("experiment_record.v1.json", "{", "GODEL_INSPECT_INVALID"),
        (
            "obsmem_index_entry.runtime.v1.json",
            "{",
            "GODEL_INSPECT_INVALID",
        ),
    ];
    for (file_name, invalid_json, needle) in parse_cases {
        write_all();
        std::fs::write(run_dir.join(file_name), invalid_json).expect("write invalid json");
        let err = real_godel_inspect(&[
            "--run-id".to_string(),
            "run-745-a".to_string(),
            "--runs-dir".to_string(),
            base.to_string_lossy().to_string(),
        ])
        .expect_err("invalid runtime artifact should fail");
        assert!(
            err.to_string().contains(needle),
            "file={file_name}\nerr={err}"
        );
    }

    let missing_cases = [
        "godel_eval_report.v1.json",
        "godel_promotion_decision.v1.json",
        "evaluation_plan.v1.json",
        "experiment_record.v1.json",
    ];
    for file_name in missing_cases {
        write_all();
        std::fs::remove_file(run_dir.join(file_name)).expect("remove runtime artifact");
        let err = real_godel_inspect(&[
            "--run-id".to_string(),
            "run-745-a".to_string(),
            "--runs-dir".to_string(),
            base.to_string_lossy().to_string(),
        ])
        .expect_err("missing runtime artifact should fail");
        assert!(
            err.to_string().contains("GODEL_INSPECT_IO"),
            "file={file_name}\nerr={err}"
        );
    }
    write_all();
    let mut mismatched_index = index.clone();
    mismatched_index.entry.run_id = "run-745-b".to_string();
    std::fs::write(
        run_dir.join("obsmem_index_entry.runtime.v1.json"),
        serde_json::to_string_pretty(&mismatched_index).expect("mismatched index json"),
    )
    .expect("write mismatched index");
    let err = real_godel_inspect(&[
        "--run-id".to_string(),
        "run-745-a".to_string(),
        "--runs-dir".to_string(),
        base.to_string_lossy().to_string(),
    ])
    .expect_err("mismatched run id should fail");
    assert!(
        err.to_string()
            .contains("persisted run_id did not match requested run_id"),
        "err={err}"
    );

    write_all();
    std::fs::write(run_dir.join("godel_experiment_priority.v1.json"), "{")
        .expect("write invalid prioritization");
    let err = real_godel_inspect(&[
        "--run-id".to_string(),
        "run-745-a".to_string(),
        "--runs-dir".to_string(),
        base.to_string_lossy().to_string(),
    ])
    .expect_err("invalid prioritization artifact should fail");
    assert!(err.to_string().contains("GODEL_INSPECT_INVALID"));

    write_all();
    let mut empty_prioritization = prioritization.clone();
    empty_prioritization.ranked_candidates.clear();
    std::fs::write(
        run_dir.join("godel_experiment_priority.v1.json"),
        serde_json::to_string_pretty(&empty_prioritization).expect("empty prioritization json"),
    )
    .expect("write empty prioritization");
    real_godel_inspect(&[
        "--run-id".to_string(),
        "run-745-a".to_string(),
        "--runs-dir".to_string(),
        base.to_string_lossy().to_string(),
    ])
    .expect("inspect should succeed with empty ranked candidates");
    let _ = std::fs::remove_dir_all(base);
}

#[test]
fn real_godel_evaluate_validates_args_and_returns_summary() {
    let err = real_godel_evaluate(&[]).expect_err("missing failure-code");
    assert!(err.to_string().contains("requires --failure-code"));

    let err = real_godel_evaluate(&[
        "--failure-code".to_string(),
        "tool_failure".to_string(),
        "--experiment-result".to_string(),
        "mystery".to_string(),
        "--score-delta".to_string(),
        "1".to_string(),
    ])
    .expect_err("invalid experiment result");
    assert!(err.to_string().contains("<ok|blocked>"));

    let err = real_godel_evaluate(&[
        "--failure-code".to_string(),
        "tool_failure".to_string(),
        "--experiment-result".to_string(),
        "ok".to_string(),
        "--score-delta".to_string(),
        "nope".to_string(),
    ])
    .expect_err("invalid score delta");
    assert!(err.to_string().contains("valid i32"));

    let err = real_godel_evaluate(&[
        "--failure-code".to_string(),
        "tool_failure".to_string(),
        "--score-delta".to_string(),
        "1".to_string(),
    ])
    .expect_err("missing experiment result");
    assert!(err
        .to_string()
        .contains("godel evaluate requires --experiment-result <ok|blocked>"));

    let err = real_godel_evaluate(&[
        "--failure-code".to_string(),
        "tool_failure".to_string(),
        "--experiment-result".to_string(),
        "ok".to_string(),
    ])
    .expect_err("missing score delta");
    assert!(err
        .to_string()
        .contains("godel evaluate requires --score-delta <int>"));

    real_godel_evaluate(&[
        "--failure-code".to_string(),
        "tool_failure".to_string(),
        "--experiment-result".to_string(),
        "blocked".to_string(),
        "--score-delta".to_string(),
        "0".to_string(),
    ])
    .expect("evaluate summary");
}
