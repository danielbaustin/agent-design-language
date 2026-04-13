use super::*;
use crate::cli::run_artifacts_types::{
    CognitiveArbitrationArtifact, FastSlowPathArtifact, FastSlowPathState, SuggestedChangeIntent,
    SuggestionEvidence, SuggestionItem, SuggestionsArtifact, SuggestionsGeneratedFrom,
};

#[test]
fn build_agency_selection_artifact_is_deterministic_and_emits_multiple_candidates() {
    let mut summary = RunSummaryArtifact {
        run_summary_version: 1,
        artifact_model_version: artifacts::ARTIFACT_MODEL_VERSION,
        run_id: "agency-selection-run".to_string(),
        workflow_id: "wf".to_string(),
        adl_version: "0.86".to_string(),
        swarm_version: "test".to_string(),
        status: "success".to_string(),
        error_kind: None,
        counts: RunSummaryCounts {
            total_steps: 2,
            completed_steps: 2,
            failed_steps: 0,
            provider_call_count: 1,
            delegation_steps: 0,
            delegation_requires_verification_steps: 0,
        },
        policy: RunSummaryPolicy {
            security_envelope_enabled: false,
            signing_required: false,
            key_id_required: false,
            verify_allowed_algs: Vec::new(),
            verify_allowed_key_sources: Vec::new(),
            sandbox_policy: "centralized_path_resolver_v1".to_string(),
            security_denials_by_code: BTreeMap::new(),
        },
        links: RunSummaryLinks {
            run_json: "run.json".to_string(),
            steps_json: "steps.json".to_string(),
            pause_state_json: None,
            outputs_dir: "outputs".to_string(),
            logs_dir: "logs".to_string(),
            learning_dir: "learning".to_string(),
            scores_json: None,
            suggestions_json: None,
            aee_decision_json: None,
            cognitive_signals_json: None,
            fast_slow_path_json: None,
            agency_selection_json: None,
            bounded_execution_json: None,
            evaluation_signals_json: None,
            cognitive_arbitration_json: None,
            affect_state_json: None,
            reasoning_graph_json: None,
            overlays_dir: "learning/overlays".to_string(),
            cluster_groundwork_json: None,
            trace_json: None,
        },
    };
    let success_scores = ScoresArtifact {
        scores_version: 1,
        run_id: "agency-selection-run".to_string(),
        generated_from: ScoresGeneratedFrom {
            artifact_model_version: artifacts::ARTIFACT_MODEL_VERSION,
            run_summary_version: 1,
        },
        summary: ScoresSummary {
            success_ratio: 1.0,
            failure_count: 0,
            retry_count: 0,
            delegation_denied_count: 0,
            security_denied_count: 0,
        },
        metrics: ScoresMetrics {
            scheduler_max_parallel_observed: 1,
        },
    };
    let success_suggestions = build_suggestions_artifact(&summary, Some(&success_scores));
    let success_signals = run_artifacts::build_cognitive_signals_artifact(
        &summary,
        &success_suggestions,
        Some(&success_scores),
    );
    let success_affect = run_artifacts::build_affect_state_artifact(
        &summary,
        &success_suggestions,
        Some(&success_scores),
    );
    let success_arbitration = run_artifacts::build_cognitive_arbitration_artifact(
        &summary,
        &success_suggestions,
        &success_signals,
        &success_affect,
        Some(&success_scores),
    );
    let success_state = run_artifacts::build_fast_slow_path_state(&success_arbitration);
    let success_path = run_artifacts::build_fast_slow_path_artifact(
        &summary,
        &success_arbitration,
        &success_state,
        Some(&success_scores),
    );
    let success_agency_state = run_artifacts::build_agency_selection_state(
        &success_signals,
        &success_arbitration,
        &success_state,
        &success_path,
    );
    let fast_left = run_artifacts::build_agency_selection_artifact(
        &summary,
        &success_arbitration,
        &success_agency_state,
        Some(&success_scores),
    );
    let fast_right = run_artifacts::build_agency_selection_artifact(
        &summary,
        &success_arbitration,
        &success_agency_state,
        Some(&success_scores),
    );
    assert_eq!(
        serde_json::to_value(&fast_left).expect("fast left value"),
        serde_json::to_value(&fast_right).expect("fast right value")
    );
    assert_eq!(fast_left.agency_selection_version, 1);
    assert_eq!(fast_left.candidate_set.len(), 2);
    assert_eq!(fast_left.selected_candidate_id, "cand-fast-execute");
    assert_eq!(
        success_agency_state.selected_candidate_id,
        "cand-fast-execute"
    );
    assert_eq!(
        success_agency_state.selected_candidate_kind,
        "direct_execution"
    );
    assert!(fast_left
        .candidate_generation_basis
        .contains("runtime_branch=fast_direct_execution_branch"));

    summary.status = "failure".to_string();
    summary.counts.failed_steps = 1;
    let failure_scores = ScoresArtifact {
        scores_version: 1,
        run_id: "agency-selection-run".to_string(),
        generated_from: ScoresGeneratedFrom {
            artifact_model_version: artifacts::ARTIFACT_MODEL_VERSION,
            run_summary_version: 1,
        },
        summary: ScoresSummary {
            success_ratio: 0.0,
            failure_count: 1,
            retry_count: 1,
            delegation_denied_count: 0,
            security_denied_count: 0,
        },
        metrics: ScoresMetrics {
            scheduler_max_parallel_observed: 1,
        },
    };
    let failure_suggestions = build_suggestions_artifact(&summary, Some(&failure_scores));
    let failure_signals = run_artifacts::build_cognitive_signals_artifact(
        &summary,
        &failure_suggestions,
        Some(&failure_scores),
    );
    let failure_affect = run_artifacts::build_affect_state_artifact(
        &summary,
        &failure_suggestions,
        Some(&failure_scores),
    );
    let failure_arbitration = run_artifacts::build_cognitive_arbitration_artifact(
        &summary,
        &failure_suggestions,
        &failure_signals,
        &failure_affect,
        Some(&failure_scores),
    );
    let failure_state = run_artifacts::build_fast_slow_path_state(&failure_arbitration);
    let failure_path = run_artifacts::build_fast_slow_path_artifact(
        &summary,
        &failure_arbitration,
        &failure_state,
        Some(&failure_scores),
    );
    let failure_agency_state = run_artifacts::build_agency_selection_state(
        &failure_signals,
        &failure_arbitration,
        &failure_state,
        &failure_path,
    );
    let slow = run_artifacts::build_agency_selection_artifact(
        &summary,
        &failure_arbitration,
        &failure_agency_state,
        Some(&failure_scores),
    );
    assert_eq!(slow.selected_candidate_id, "cand-slow-review");
    assert!(slow.candidate_set.len() >= 3);
    assert_eq!(
        failure_agency_state.selected_candidate_kind,
        "review_and_refine"
    );
    assert_ne!(fast_left.selection_mode, slow.selection_mode);
    assert_ne!(
        fast_left.selected_candidate_reason,
        slow.selected_candidate_reason
    );
}

#[test]
fn build_agency_selection_artifact_exposes_instinct_sensitive_candidate_shift() {
    let summary = RunSummaryArtifact {
        run_summary_version: 1,
        artifact_model_version: artifacts::ARTIFACT_MODEL_VERSION,
        run_id: "agency-selection-instinct-run".to_string(),
        workflow_id: "wf".to_string(),
        adl_version: "0.88".to_string(),
        swarm_version: "test".to_string(),
        status: "success".to_string(),
        error_kind: None,
        counts: RunSummaryCounts {
            total_steps: 2,
            completed_steps: 2,
            failed_steps: 0,
            provider_call_count: 1,
            delegation_steps: 0,
            delegation_requires_verification_steps: 0,
        },
        policy: RunSummaryPolicy {
            security_envelope_enabled: false,
            signing_required: false,
            key_id_required: false,
            verify_allowed_algs: Vec::new(),
            verify_allowed_key_sources: Vec::new(),
            sandbox_policy: "centralized_path_resolver_v1".to_string(),
            security_denials_by_code: BTreeMap::new(),
        },
        links: RunSummaryLinks {
            run_json: "run.json".to_string(),
            steps_json: "steps.json".to_string(),
            pause_state_json: None,
            outputs_dir: "outputs".to_string(),
            logs_dir: "logs".to_string(),
            learning_dir: "learning".to_string(),
            scores_json: None,
            suggestions_json: None,
            aee_decision_json: None,
            cognitive_signals_json: None,
            fast_slow_path_json: None,
            agency_selection_json: None,
            bounded_execution_json: None,
            evaluation_signals_json: None,
            cognitive_arbitration_json: None,
            affect_state_json: None,
            reasoning_graph_json: None,
            overlays_dir: "learning/overlays".to_string(),
            cluster_groundwork_json: None,
            trace_json: None,
        },
    };
    let suggestions = SuggestionsArtifact {
        suggestions_version: 1,
        run_id: summary.run_id.clone(),
        generated_from: SuggestionsGeneratedFrom {
            artifact_model_version: artifacts::ARTIFACT_MODEL_VERSION,
            run_summary_version: 1,
            scores_version: Some(1),
        },
        suggestions: vec![SuggestionItem {
            id: "sug-001".to_string(),
            category: "stability".to_string(),
            severity: "medium".to_string(),
            rationale: "follow up on unexplained variance before direct execution".to_string(),
            evidence: SuggestionEvidence {
                failure_count: 0,
                retry_count: 1,
                delegation_denied_count: 0,
                security_denied_count: 0,
                success_ratio: 0.8,
                scheduler_max_parallel_observed: 1,
            },
            proposed_change: SuggestedChangeIntent {
                intent: "review_failure_hotspots".to_string(),
                target: "workflow-runtime".to_string(),
            },
        }],
    };
    let scores = ScoresArtifact {
        scores_version: 1,
        run_id: summary.run_id.clone(),
        generated_from: ScoresGeneratedFrom {
            artifact_model_version: artifacts::ARTIFACT_MODEL_VERSION,
            run_summary_version: 1,
        },
        summary: ScoresSummary {
            success_ratio: 0.8,
            failure_count: 0,
            retry_count: 1,
            delegation_denied_count: 0,
            security_denied_count: 0,
        },
        metrics: ScoresMetrics {
            scheduler_max_parallel_observed: 1,
        },
    };
    let signals =
        run_artifacts::build_cognitive_signals_artifact(&summary, &suggestions, Some(&scores));
    assert_eq!(signals.instinct.dominant_instinct, "curiosity");

    let arbitration = CognitiveArbitrationArtifact {
        cognitive_arbitration_version: 1,
        run_id: summary.run_id.clone(),
        generated_from: AeeDecisionGeneratedFrom {
            artifact_model_version: artifacts::ARTIFACT_MODEL_VERSION,
            run_summary_version: 1,
            suggestions_version: 1,
            scores_version: Some(1),
        },
        route_selected: "continue".to_string(),
        reasoning_mode: "bounded_fast_path".to_string(),
        confidence: "high".to_string(),
        risk_class: "medium".to_string(),
        applied_constraints: vec!["bounded_once".to_string()],
        cost_latency_assumption: "prefer low latency".to_string(),
        route_reason: "keep execution moving while reducing uncertainty".to_string(),
        deterministic_selection_rule:
            "derive route from visible arbitration state without hidden initiative".to_string(),
    };
    let fast_state = FastSlowPathState {
        selected_path: "fast_path".to_string(),
        path_family: "fast".to_string(),
        runtime_branch_taken: "fast_direct_execution_branch".to_string(),
        handoff_state: "candidate_ready".to_string(),
        candidate_strategy: "single bounded candidate".to_string(),
        review_depth: "minimal".to_string(),
        execution_profile: "bounded_once".to_string(),
        termination_expectation: "ready_for_evaluation".to_string(),
        path_difference_summary: "fast path".to_string(),
    };
    let fast_artifact = FastSlowPathArtifact {
        fast_slow_path_version: 1,
        run_id: summary.run_id.clone(),
        generated_from: AeeDecisionGeneratedFrom {
            artifact_model_version: artifacts::ARTIFACT_MODEL_VERSION,
            run_summary_version: 1,
            suggestions_version: 1,
            scores_version: Some(1),
        },
        arbitration_route: "continue".to_string(),
        selected_path: "fast_path".to_string(),
        path_family: "fast".to_string(),
        runtime_branch_taken: "fast_direct_execution_branch".to_string(),
        handoff_state: "candidate_ready".to_string(),
        candidate_strategy: "single bounded candidate".to_string(),
        review_depth: "minimal".to_string(),
        execution_profile: "bounded_once".to_string(),
        termination_expectation: "ready_for_evaluation".to_string(),
        path_difference_summary: "fast path".to_string(),
        deterministic_handoff_rule: "test fixture".to_string(),
    };

    let agency = run_artifacts::build_agency_selection_state(
        &signals,
        &arbitration,
        &fast_state,
        &fast_artifact,
    );

    assert_eq!(agency.selected_candidate_id, "cand-fast-verify");
    assert_eq!(agency.selected_candidate_kind, "bounded_verification");
    assert!(agency
        .selected_candidate_reason
        .contains("curiosity or integrity pressure"));
}

#[test]
fn build_bounded_execution_artifact_is_deterministic_and_shows_iteration_shape() {
    let mut summary = RunSummaryArtifact {
        run_summary_version: 1,
        artifact_model_version: artifacts::ARTIFACT_MODEL_VERSION,
        run_id: "bounded-execution-run".to_string(),
        workflow_id: "wf".to_string(),
        adl_version: "0.86".to_string(),
        swarm_version: "test".to_string(),
        status: "success".to_string(),
        error_kind: None,
        counts: RunSummaryCounts {
            total_steps: 2,
            completed_steps: 2,
            failed_steps: 0,
            provider_call_count: 1,
            delegation_steps: 0,
            delegation_requires_verification_steps: 0,
        },
        policy: RunSummaryPolicy {
            security_envelope_enabled: false,
            signing_required: false,
            key_id_required: false,
            verify_allowed_algs: Vec::new(),
            verify_allowed_key_sources: Vec::new(),
            sandbox_policy: "centralized_path_resolver_v1".to_string(),
            security_denials_by_code: BTreeMap::new(),
        },
        links: RunSummaryLinks {
            run_json: "run.json".to_string(),
            steps_json: "steps.json".to_string(),
            pause_state_json: None,
            outputs_dir: "outputs".to_string(),
            logs_dir: "logs".to_string(),
            learning_dir: "learning".to_string(),
            scores_json: None,
            suggestions_json: None,
            aee_decision_json: None,
            cognitive_signals_json: None,
            fast_slow_path_json: None,
            agency_selection_json: None,
            bounded_execution_json: None,
            evaluation_signals_json: None,
            cognitive_arbitration_json: None,
            affect_state_json: None,
            reasoning_graph_json: None,
            overlays_dir: "learning/overlays".to_string(),
            cluster_groundwork_json: None,
            trace_json: None,
        },
    };
    let success_scores = ScoresArtifact {
        scores_version: 1,
        run_id: "bounded-execution-run".to_string(),
        generated_from: ScoresGeneratedFrom {
            artifact_model_version: artifacts::ARTIFACT_MODEL_VERSION,
            run_summary_version: 1,
        },
        summary: ScoresSummary {
            success_ratio: 1.0,
            failure_count: 0,
            retry_count: 0,
            delegation_denied_count: 0,
            security_denied_count: 0,
        },
        metrics: ScoresMetrics {
            scheduler_max_parallel_observed: 1,
        },
    };
    let success_suggestions = build_suggestions_artifact(&summary, Some(&success_scores));
    let success_signals = run_artifacts::build_cognitive_signals_artifact(
        &summary,
        &success_suggestions,
        Some(&success_scores),
    );
    let success_affect = run_artifacts::build_affect_state_artifact(
        &summary,
        &success_suggestions,
        Some(&success_scores),
    );
    let success_arbitration = run_artifacts::build_cognitive_arbitration_artifact(
        &summary,
        &success_suggestions,
        &success_signals,
        &success_affect,
        Some(&success_scores),
    );
    let success_state = run_artifacts::build_fast_slow_path_state(&success_arbitration);
    let success_path = run_artifacts::build_fast_slow_path_artifact(
        &summary,
        &success_arbitration,
        &success_state,
        Some(&success_scores),
    );
    let success_agency_state = run_artifacts::build_agency_selection_state(
        &success_signals,
        &success_arbitration,
        &success_state,
        &success_path,
    );
    let success_agency = run_artifacts::build_agency_selection_artifact(
        &summary,
        &success_arbitration,
        &success_agency_state,
        Some(&success_scores),
    );
    let success_state = run_artifacts::build_bounded_execution_state(
        &summary,
        &success_path,
        &success_agency,
        &success_agency_state,
    );
    let fast_left = run_artifacts::build_bounded_execution_artifact(
        &summary,
        &success_path,
        &success_agency,
        &success_state,
        Some(&success_scores),
    );
    let fast_right = run_artifacts::build_bounded_execution_artifact(
        &summary,
        &success_path,
        &success_agency,
        &success_state,
        Some(&success_scores),
    );
    assert_eq!(
        serde_json::to_value(&fast_left).expect("fast left value"),
        serde_json::to_value(&fast_right).expect("fast right value")
    );
    assert_eq!(fast_left.iteration_count, 1);
    assert_eq!(
        fast_left.provisional_termination_state,
        "ready_for_evaluation"
    );
    assert_eq!(success_state.continuation_state, "stop_after_one");

    summary.status = "failure".to_string();
    summary.counts.failed_steps = 1;
    let failure_scores = ScoresArtifact {
        scores_version: 1,
        run_id: "bounded-execution-run".to_string(),
        generated_from: ScoresGeneratedFrom {
            artifact_model_version: artifacts::ARTIFACT_MODEL_VERSION,
            run_summary_version: 1,
        },
        summary: ScoresSummary {
            success_ratio: 0.0,
            failure_count: 1,
            retry_count: 1,
            delegation_denied_count: 0,
            security_denied_count: 0,
        },
        metrics: ScoresMetrics {
            scheduler_max_parallel_observed: 1,
        },
    };
    let failure_suggestions = build_suggestions_artifact(&summary, Some(&failure_scores));
    let failure_signals = run_artifacts::build_cognitive_signals_artifact(
        &summary,
        &failure_suggestions,
        Some(&failure_scores),
    );
    let failure_affect = run_artifacts::build_affect_state_artifact(
        &summary,
        &failure_suggestions,
        Some(&failure_scores),
    );
    let failure_arbitration = run_artifacts::build_cognitive_arbitration_artifact(
        &summary,
        &failure_suggestions,
        &failure_signals,
        &failure_affect,
        Some(&failure_scores),
    );
    let failure_state = run_artifacts::build_fast_slow_path_state(&failure_arbitration);
    let failure_path = run_artifacts::build_fast_slow_path_artifact(
        &summary,
        &failure_arbitration,
        &failure_state,
        Some(&failure_scores),
    );
    let failure_agency_state = run_artifacts::build_agency_selection_state(
        &failure_signals,
        &failure_arbitration,
        &failure_state,
        &failure_path,
    );
    let failure_agency = run_artifacts::build_agency_selection_artifact(
        &summary,
        &failure_arbitration,
        &failure_agency_state,
        Some(&failure_scores),
    );
    let failure_state = run_artifacts::build_bounded_execution_state(
        &summary,
        &failure_path,
        &failure_agency,
        &failure_agency_state,
    );
    let slow = run_artifacts::build_bounded_execution_artifact(
        &summary,
        &failure_path,
        &failure_agency,
        &failure_state,
        Some(&failure_scores),
    );
    assert_eq!(slow.bounded_execution_version, 1);
    assert_eq!(slow.iteration_count, 2);
    assert_eq!(slow.iterations[0].stage, "review");
    assert_eq!(
        slow.provisional_termination_state,
        "ready_for_runtime_evaluation"
    );
    assert_ne!(fast_left.iteration_count, slow.iteration_count);
}

#[test]
fn build_evaluation_signals_artifact_is_deterministic_and_emits_termination_reasons() {
    let mut summary = RunSummaryArtifact {
        run_summary_version: 1,
        artifact_model_version: artifacts::ARTIFACT_MODEL_VERSION,
        run_id: "evaluation-signals-run".to_string(),
        workflow_id: "wf".to_string(),
        adl_version: "0.86".to_string(),
        swarm_version: "test".to_string(),
        status: "success".to_string(),
        error_kind: None,
        counts: RunSummaryCounts {
            total_steps: 2,
            completed_steps: 2,
            failed_steps: 0,
            provider_call_count: 1,
            delegation_steps: 0,
            delegation_requires_verification_steps: 0,
        },
        policy: RunSummaryPolicy {
            security_envelope_enabled: false,
            signing_required: false,
            key_id_required: false,
            verify_allowed_algs: Vec::new(),
            verify_allowed_key_sources: Vec::new(),
            sandbox_policy: "centralized_path_resolver_v1".to_string(),
            security_denials_by_code: BTreeMap::new(),
        },
        links: RunSummaryLinks {
            run_json: "run.json".to_string(),
            steps_json: "steps.json".to_string(),
            pause_state_json: None,
            outputs_dir: "outputs".to_string(),
            logs_dir: "logs".to_string(),
            learning_dir: "learning".to_string(),
            scores_json: None,
            suggestions_json: None,
            aee_decision_json: None,
            cognitive_signals_json: None,
            fast_slow_path_json: None,
            agency_selection_json: None,
            bounded_execution_json: None,
            evaluation_signals_json: None,
            cognitive_arbitration_json: None,
            affect_state_json: None,
            reasoning_graph_json: None,
            overlays_dir: "learning/overlays".to_string(),
            cluster_groundwork_json: None,
            trace_json: None,
        },
    };
    let success_scores = ScoresArtifact {
        scores_version: 1,
        run_id: "evaluation-signals-run".to_string(),
        generated_from: ScoresGeneratedFrom {
            artifact_model_version: artifacts::ARTIFACT_MODEL_VERSION,
            run_summary_version: 1,
        },
        summary: ScoresSummary {
            success_ratio: 1.0,
            failure_count: 0,
            retry_count: 0,
            delegation_denied_count: 0,
            security_denied_count: 0,
        },
        metrics: ScoresMetrics {
            scheduler_max_parallel_observed: 1,
        },
    };
    let success_suggestions = build_suggestions_artifact(&summary, Some(&success_scores));
    let success_signals = run_artifacts::build_cognitive_signals_artifact(
        &summary,
        &success_suggestions,
        Some(&success_scores),
    );
    let success_affect = run_artifacts::build_affect_state_artifact(
        &summary,
        &success_suggestions,
        Some(&success_scores),
    );
    let success_arbitration = run_artifacts::build_cognitive_arbitration_artifact(
        &summary,
        &success_suggestions,
        &success_signals,
        &success_affect,
        Some(&success_scores),
    );
    let success_state = run_artifacts::build_fast_slow_path_state(&success_arbitration);
    let success_path = run_artifacts::build_fast_slow_path_artifact(
        &summary,
        &success_arbitration,
        &success_state,
        Some(&success_scores),
    );
    let success_agency_state = run_artifacts::build_agency_selection_state(
        &success_signals,
        &success_arbitration,
        &success_state,
        &success_path,
    );
    let success_agency = run_artifacts::build_agency_selection_artifact(
        &summary,
        &success_arbitration,
        &success_agency_state,
        Some(&success_scores),
    );
    let success_execution_state = run_artifacts::build_bounded_execution_state(
        &summary,
        &success_path,
        &success_agency,
        &success_agency_state,
    );
    let success_execution = run_artifacts::build_bounded_execution_artifact(
        &summary,
        &success_path,
        &success_agency,
        &success_execution_state,
        Some(&success_scores),
    );
    let success_eval_state =
        run_artifacts::build_evaluation_control_state(&summary, &success_execution);
    let success_left = run_artifacts::build_evaluation_signals_artifact(
        &summary,
        &success_path,
        &success_agency,
        &success_eval_state,
        Some(&success_scores),
    );
    let success_right = run_artifacts::build_evaluation_signals_artifact(
        &summary,
        &success_path,
        &success_agency,
        &success_eval_state,
        Some(&success_scores),
    );
    assert_eq!(
        serde_json::to_value(&success_left).expect("success left value"),
        serde_json::to_value(&success_right).expect("success right value")
    );
    assert_eq!(success_left.termination_reason, "success");
    assert_eq!(success_left.failure_signal, "none");
    assert_eq!(success_left.next_control_action, "complete_run");

    summary.status = "failure".to_string();
    summary.counts.failed_steps = 1;
    let failure_scores = ScoresArtifact {
        scores_version: 1,
        run_id: "evaluation-signals-run".to_string(),
        generated_from: ScoresGeneratedFrom {
            artifact_model_version: artifacts::ARTIFACT_MODEL_VERSION,
            run_summary_version: 1,
        },
        summary: ScoresSummary {
            success_ratio: 0.0,
            failure_count: 1,
            retry_count: 1,
            delegation_denied_count: 0,
            security_denied_count: 0,
        },
        metrics: ScoresMetrics {
            scheduler_max_parallel_observed: 1,
        },
    };
    let failure_suggestions = build_suggestions_artifact(&summary, Some(&failure_scores));
    let failure_signals = run_artifacts::build_cognitive_signals_artifact(
        &summary,
        &failure_suggestions,
        Some(&failure_scores),
    );
    let failure_affect = run_artifacts::build_affect_state_artifact(
        &summary,
        &failure_suggestions,
        Some(&failure_scores),
    );
    let failure_arbitration = run_artifacts::build_cognitive_arbitration_artifact(
        &summary,
        &failure_suggestions,
        &failure_signals,
        &failure_affect,
        Some(&failure_scores),
    );
    let failure_state = run_artifacts::build_fast_slow_path_state(&failure_arbitration);
    let failure_path = run_artifacts::build_fast_slow_path_artifact(
        &summary,
        &failure_arbitration,
        &failure_state,
        Some(&failure_scores),
    );
    let failure_agency_state = run_artifacts::build_agency_selection_state(
        &failure_signals,
        &failure_arbitration,
        &failure_state,
        &failure_path,
    );
    let failure_agency = run_artifacts::build_agency_selection_artifact(
        &summary,
        &failure_arbitration,
        &failure_agency_state,
        Some(&failure_scores),
    );
    let failure_execution_state = run_artifacts::build_bounded_execution_state(
        &summary,
        &failure_path,
        &failure_agency,
        &failure_agency_state,
    );
    let failure_execution = run_artifacts::build_bounded_execution_artifact(
        &summary,
        &failure_path,
        &failure_agency,
        &failure_execution_state,
        Some(&failure_scores),
    );
    let failure_eval_state =
        run_artifacts::build_evaluation_control_state(&summary, &failure_execution);
    let failure_eval = run_artifacts::build_evaluation_signals_artifact(
        &summary,
        &failure_path,
        &failure_agency,
        &failure_eval_state,
        Some(&failure_scores),
    );
    assert_eq!(failure_eval.evaluation_signals_version, 1);
    assert_eq!(failure_eval.termination_reason, "bounded_failure");
    assert_eq!(failure_eval.contradiction_signal, "present");
    assert_eq!(failure_eval.next_control_action, "handoff_to_reframing");
    assert_ne!(
        success_left.termination_reason,
        failure_eval.termination_reason
    );
}
