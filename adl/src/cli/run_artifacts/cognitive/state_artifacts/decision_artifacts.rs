use super::*;

fn aee_decision_kind_for_intent(intent: &str) -> (&'static str, &'static str) {
    match intent {
        "increase_step_retry_budget" => (
            "bounded_retry_recovery",
            "raise retry budget for the failed step set on the next bounded run",
        ),
        "evaluate_parallelizable_dependencies" => (
            "bounded_scheduler_review",
            "review whether the workflow can safely increase bounded parallelism",
        ),
        "review_delegation_policy_scope" => (
            "bounded_delegation_review",
            "review delegation boundaries before the next bounded run",
        ),
        "review_security_policy_expectations" => (
            "bounded_security_review",
            "review trust-policy expectations before the next bounded run",
        ),
        "review_failure_hotspots" => (
            "bounded_failure_review",
            "review failing dependency hotspots before the next bounded run",
        ),
        _ => (
            "bounded_runtime_review",
            "review bounded runtime signals before the next run",
        ),
    }
}

pub(crate) fn build_aee_decision_artifact(
    run_summary: &RunSummaryArtifact,
    suggestions: &SuggestionsArtifact,
    affect_state: &AffectStateArtifact,
    scores: Option<&ScoresArtifact>,
) -> AeeDecisionArtifact {
    let selected = suggestions
        .suggestions
        .first()
        .cloned()
        .unwrap_or_else(|| SuggestionItem {
            id: "sug-000".to_string(),
            category: "stability".to_string(),
            severity: "info".to_string(),
            rationale: "No bounded adaptation signals fired; keep current policy state."
                .to_string(),
            evidence: SuggestionEvidence {
                failure_count: 0,
                retry_count: 0,
                delegation_denied_count: 0,
                security_denied_count: 0,
                success_ratio: 1.0,
                scheduler_max_parallel_observed: 1,
            },
            proposed_change: SuggestedChangeIntent {
                intent: "maintain_current_policy".to_string(),
                target: "workflow-runtime".to_string(),
            },
        });
    let (decision_kind, expected_downstream_effect) =
        aee_decision_kind_for_intent(&selected.proposed_change.intent);
    let recommended_retry_budget = (selected.proposed_change.intent
        == "increase_step_retry_budget"
        && affect_state.affect.recovery_bias >= 2)
        .then_some(2);
    let expected_downstream_effect = if let Some(budget) = recommended_retry_budget {
        format!(
            "{expected_downstream_effect}; affect-guided recovery bias recommends retry budget max_attempts={budget}"
        )
    } else {
        expected_downstream_effect.to_string()
    };

    AeeDecisionArtifact {
        aee_decision_version: AEE_DECISION_VERSION,
        run_id: run_summary.run_id.clone(),
        generated_from: AeeDecisionGeneratedFrom {
            artifact_model_version: run_summary.artifact_model_version,
            run_summary_version: run_summary.run_summary_version,
            suggestions_version: suggestions.suggestions_version,
            scores_version: scores.map(|value| value.scores_version),
        },
        affect_state: AffectStateRef {
            affect_state_id: affect_state.affect.affect_state_id.clone(),
            affect_mode: affect_state.affect.affect_mode.clone(),
            downstream_priority: affect_state.affect.downstream_priority.clone(),
            recovery_bias: affect_state.affect.recovery_bias,
        },
        decision: AeeDecisionRecord {
            decision_id: "aee-001".to_string(),
            decision_kind: decision_kind.to_string(),
            selected_suggestion_id: selected.id,
            category: selected.category,
            intent: selected.proposed_change.intent,
            target: selected.proposed_change.target,
            rationale: selected.rationale,
            expected_downstream_effect,
            deterministic_selection_rule:
                "select the first suggestion emitted by build_suggestions_artifact after stable category ordering, then apply the deterministic affect-state recovery bias"
                    .to_string(),
            recommended_retry_budget,
            evidence: selected.evidence,
        },
    }
}

pub(crate) fn build_reasoning_graph_artifact(
    run_summary: &RunSummaryArtifact,
    affect_state: &AffectStateArtifact,
    aee_decision: &AeeDecisionArtifact,
    scores: Option<&ScoresArtifact>,
) -> ReasoningGraphArtifact {
    let retry_score = if affect_state.affect.recovery_bias >= 2 {
        92
    } else if affect_state.affect.recovery_bias == 1 {
        68
    } else {
        22
    };
    let maintain_score = if affect_state.affect.recovery_bias == 0 {
        88
    } else {
        36
    };
    let failure_signal_score = if aee_decision.decision.evidence.failure_count > 0 {
        80
    } else {
        24
    };

    let mut action_nodes = [
        ReasoningGraphNode {
            node_id: "action.retry_budget".to_string(),
            node_kind: "action".to_string(),
            label: "increase retry budget".to_string(),
            rank: 0,
            priority_score: retry_score,
            affect_mode: Some(affect_state.affect.affect_mode.clone()),
            rationale: format!(
                "Affect-guided recovery bias {} favors bounded retry recovery.",
                affect_state.affect.recovery_bias
            ),
        },
        ReasoningGraphNode {
            node_id: "action.maintain_policy".to_string(),
            node_kind: "action".to_string(),
            label: "maintain current policy".to_string(),
            rank: 0,
            priority_score: maintain_score,
            affect_mode: Some(affect_state.affect.affect_mode.clone()),
            rationale: "Steady-state or low-bias runs preserve the current bounded runtime policy."
                .to_string(),
        },
    ];
    action_nodes.sort_by(|a, b| {
        b.priority_score
            .cmp(&a.priority_score)
            .then_with(|| a.node_id.cmp(&b.node_id))
    });
    for (idx, node) in action_nodes.iter_mut().enumerate() {
        node.rank = (idx + 1) as u32;
    }

    let selected_node = action_nodes
        .iter()
        .find(|node| {
            (aee_decision.decision.intent == "increase_step_retry_budget"
                && node.node_id == "action.retry_budget")
                || (aee_decision.decision.intent != "increase_step_retry_budget"
                    && node.node_id == "action.maintain_policy")
        })
        .cloned()
        .unwrap_or_else(|| action_nodes[0].clone());

    let nodes = vec![
        ReasoningGraphNode {
            node_id: "evidence.runtime".to_string(),
            node_kind: "evidence".to_string(),
            label: "bounded runtime evidence".to_string(),
            rank: 1,
            priority_score: failure_signal_score,
            affect_mode: None,
            rationale: format!(
                "failure_count={} retry_count={} success_ratio={}",
                aee_decision.decision.evidence.failure_count,
                aee_decision.decision.evidence.retry_count,
                aee_decision.decision.evidence.success_ratio
            ),
        },
        ReasoningGraphNode {
            node_id: "affect.current".to_string(),
            node_kind: "affect".to_string(),
            label: affect_state.affect.affect_mode.replace('_', " "),
            rank: 1,
            priority_score: 100,
            affect_mode: Some(affect_state.affect.affect_mode.clone()),
            rationale: affect_state.affect.downstream_priority.clone(),
        },
        action_nodes[0].clone(),
        action_nodes[1].clone(),
    ];

    let edges = vec![
        ReasoningGraphEdge {
            edge_id: "edge-001".to_string(),
            from: "evidence.runtime".to_string(),
            to: "affect.current".to_string(),
            relation: "updates".to_string(),
            rationale: affect_state.affect.update_reason.clone(),
        },
        ReasoningGraphEdge {
            edge_id: "edge-002".to_string(),
            from: "affect.current".to_string(),
            to: "action.retry_budget".to_string(),
            relation: "prioritizes".to_string(),
            rationale: "High recovery bias increases retry-budget priority.".to_string(),
        },
        ReasoningGraphEdge {
            edge_id: "edge-003".to_string(),
            from: "affect.current".to_string(),
            to: "action.maintain_policy".to_string(),
            relation: "prioritizes".to_string(),
            rationale: "Low recovery bias preserves the maintain-current-policy path.".to_string(),
        },
    ];

    ReasoningGraphArtifact {
        reasoning_graph_version: REASONING_GRAPH_VERSION,
        run_id: run_summary.run_id.clone(),
        generated_from: AeeDecisionGeneratedFrom {
            artifact_model_version: run_summary.artifact_model_version,
            run_summary_version: run_summary.run_summary_version,
            suggestions_version: affect_state.generated_from.suggestions_version,
            scores_version: scores.map(|value| value.scores_version),
        },
        graph: ReasoningGraphRecord {
            graph_id: "reasoning-graph-001".to_string(),
            dominant_affect_mode: affect_state.affect.affect_mode.clone(),
            ranking_rule:
                "sort action nodes by descending priority_score, then lexicographic node_id"
                    .to_string(),
            selected_path: ReasoningGraphSelection {
                selected_node_id: selected_node.node_id,
                selected_intent: aee_decision.decision.intent.clone(),
                selected_target: aee_decision.decision.target.clone(),
                graph_derived_output: aee_decision.decision.expected_downstream_effect.clone(),
                affect_changed_ranking: affect_state.affect.recovery_bias > 0,
            },
            nodes,
            edges,
        },
    }
}
