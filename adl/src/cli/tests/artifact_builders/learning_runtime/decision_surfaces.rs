use super::*;

#[test]
fn build_control_path_decisions_artifact_is_deterministic_and_surfaces_core_records() {
    let generated_from = run_artifacts::AeeDecisionGeneratedFrom {
        artifact_model_version: artifacts::ARTIFACT_MODEL_VERSION,
        run_summary_version: 1,
        suggestions_version: 1,
        scores_version: Some(1),
    };
    let run_summary = RunSummaryArtifact {
        run_summary_version: 1,
        artifact_model_version: artifacts::ARTIFACT_MODEL_VERSION,
        run_id: "decision-run".to_string(),
        workflow_id: "wf".to_string(),
        adl_version: "0.89".to_string(),
        swarm_version: "test".to_string(),
        status: "failure".to_string(),
        error_kind: None,
        counts: RunSummaryCounts {
            total_steps: 1,
            completed_steps: 1,
            failed_steps: 1,
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
    let arbitration = run_artifacts::CognitiveArbitrationArtifact {
        cognitive_arbitration_version: 1,
        run_id: "decision-run".to_string(),
        generated_from: generated_from.clone(),
        route_selected: "slow".to_string(),
        reasoning_mode: "review_heavy".to_string(),
        confidence: "guarded".to_string(),
        risk_class: "high".to_string(),
        applied_constraints: vec![
            "bounded_integrity_review".to_string(),
            "contradiction_review_required".to_string(),
        ],
        cost_latency_assumption: "favor review over speed for high-risk contradictions".to_string(),
        route_reason: "contradiction and risk require the slow review path".to_string(),
        deterministic_selection_rule: "deterministic".to_string(),
    };
    let agency = run_artifacts::AgencySelectionArtifact {
        agency_selection_version: 1,
        run_id: "decision-run".to_string(),
        generated_from: generated_from.clone(),
        candidate_generation_basis: "bounded deterministic candidates".to_string(),
        selection_mode: "review_first".to_string(),
        candidate_set: vec![execute::AgencyCandidateRecord {
            candidate_id: "cand-custom-review".to_string(),
            candidate_kind: "review_and_refine".to_string(),
            bounded_action: "review and refine the candidate".to_string(),
            review_requirement: "review_required".to_string(),
            execution_priority: 1,
            rationale: "custom selected candidate reason".to_string(),
        }],
        selected_candidate_id: "cand-custom-review".to_string(),
        selected_candidate_reason: "custom selected candidate reason".to_string(),
        deterministic_selection_rule: "deterministic".to_string(),
    };
    let evaluation = run_artifacts::EvaluationSignalsArtifact {
        evaluation_signals_version: 1,
        run_id: "decision-run".to_string(),
        generated_from: generated_from.clone(),
        selected_candidate_id: "cand-custom-review".to_string(),
        selected_path: "slow_path".to_string(),
        progress_signal: "guarded_progress".to_string(),
        contradiction_signal: "present".to_string(),
        failure_signal: "none".to_string(),
        termination_reason: "contradiction_detected".to_string(),
        behavior_effect: "surface contradiction for bounded follow-up".to_string(),
        next_control_action: "handoff_to_reframing".to_string(),
        deterministic_evaluation_rule: "deterministic".to_string(),
    };
    let reframing = run_artifacts::ReframingArtifact {
        reframing_version: 1,
        run_id: "decision-run".to_string(),
        generated_from: generated_from.clone(),
        selected_candidate_id: "cand-custom-review".to_string(),
        selected_path: "slow_path".to_string(),
        frame_adequacy_score: 24,
        reframing_trigger: "triggered".to_string(),
        reframing_reason: "contradiction requires bounded reframing before commitment".to_string(),
        prior_frame: "initial".to_string(),
        new_frame: "reframed".to_string(),
        reexecution_choice: "bounded_reframe_and_retry".to_string(),
        post_reframe_state: "ready_for_reframed_execution".to_string(),
        deterministic_reframing_rule: "deterministic".to_string(),
    };
    let freedom_gate = run_artifacts::FreedomGateArtifact {
        freedom_gate_version: 1,
        run_id: "decision-run".to_string(),
        generated_from: generated_from.clone(),
        input: run_artifacts::FreedomGateInputState {
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
                progress_signal: "guarded_progress".to_string(),
                contradiction_signal: "present".to_string(),
                failure_signal: "none".to_string(),
                termination_reason: "contradiction_detected".to_string(),
            },
            consequence_context: execute::FreedomGateConsequenceContextState {
                impact_scope: "cross_surface".to_string(),
                recovery_cost: "requires_reframing".to_string(),
                operator_visibility: "review_required".to_string(),
                escalation_available: true,
            },
            frame_state: "ready_for_reframed_execution".to_string(),
        },
        gate_decision: "escalate".to_string(),
        reason_code: "frame_escalation_required".to_string(),
        decision_reason:
            "frame state and consequence context require explicit escalation before commitment can proceed"
                .to_string(),
        selected_action_or_none: None,
        commitment_blocked: true,
        judgment_boundary: "judgment_boundary".to_string(),
        required_follow_up: "escalate_for_judgment_review".to_string(),
        decision_record_kind: "gate_escalation_record".to_string(),
        deterministic_gate_rule: "deterministic".to_string(),
    };
    let scores = ScoresArtifact {
        scores_version: 1,
        run_id: "decision-run".to_string(),
        generated_from: ScoresGeneratedFrom {
            artifact_model_version: artifacts::ARTIFACT_MODEL_VERSION,
            run_summary_version: 1,
        },
        summary: ScoresSummary {
            success_ratio: 0.0,
            failure_count: 1,
            retry_count: 0,
            delegation_denied_count: 0,
            security_denied_count: 0,
        },
        metrics: ScoresMetrics {
            scheduler_max_parallel_observed: 1,
        },
    };

    let left = run_artifacts::build_control_path_decisions_artifact(
        &run_summary,
        &arbitration,
        &agency,
        &evaluation,
        &reframing,
        &freedom_gate,
        Some(&scores),
    );
    let right = run_artifacts::build_control_path_decisions_artifact(
        &run_summary,
        &arbitration,
        &agency,
        &evaluation,
        &reframing,
        &freedom_gate,
        Some(&scores),
    );

    assert_eq!(
        serde_json::to_value(&left).expect("decision surfaces left"),
        serde_json::to_value(&right).expect("decision surfaces right")
    );
    assert_eq!(left.decision_schema_name, "adl.runtime.decision.v1");
    assert_eq!(left.surfaces.len(), 3);
    assert_eq!(left.decisions.len(), 3);
    assert_eq!(
        left.decisions[0].surface_id,
        "delegation_and_routing.route_selection"
    );
    assert_eq!(left.decisions[0].outcome_class, "reroute");
    assert_eq!(left.decisions[1].outcome_class, "reroute");
    assert_eq!(left.decisions[2].outcome_class, "escalate");
    assert_eq!(
        left.decisions[2].downstream_consequence,
        "escalate_for_judgment_review"
    );
}

#[test]
fn build_action_proposal_and_mediation_artifacts_are_deterministic_and_non_authoritative() {
    let generated_from = run_artifacts::AeeDecisionGeneratedFrom {
        artifact_model_version: artifacts::ARTIFACT_MODEL_VERSION,
        run_summary_version: 1,
        suggestions_version: 1,
        scores_version: Some(1),
    };
    let run_summary = RunSummaryArtifact {
        run_summary_version: 1,
        artifact_model_version: artifacts::ARTIFACT_MODEL_VERSION,
        run_id: "mediation-run".to_string(),
        workflow_id: "wf".to_string(),
        adl_version: "0.89".to_string(),
        swarm_version: "test".to_string(),
        status: "failure".to_string(),
        error_kind: None,
        counts: RunSummaryCounts {
            total_steps: 1,
            completed_steps: 1,
            failed_steps: 1,
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
    let arbitration = run_artifacts::CognitiveArbitrationArtifact {
        cognitive_arbitration_version: 1,
        run_id: "mediation-run".to_string(),
        generated_from: generated_from.clone(),
        route_selected: "slow".to_string(),
        reasoning_mode: "review_heavy".to_string(),
        confidence: "guarded".to_string(),
        risk_class: "high".to_string(),
        applied_constraints: vec![
            "bounded_integrity_review".to_string(),
            "contradiction_review_required".to_string(),
        ],
        cost_latency_assumption: "favor review over speed for high-risk contradictions".to_string(),
        route_reason: "contradiction and risk require the slow review path".to_string(),
        deterministic_selection_rule: "deterministic".to_string(),
    };
    let agency = run_artifacts::AgencySelectionArtifact {
        agency_selection_version: 1,
        run_id: "mediation-run".to_string(),
        generated_from: generated_from.clone(),
        candidate_generation_basis: "bounded deterministic candidates".to_string(),
        selection_mode: "review_first".to_string(),
        candidate_set: vec![execute::AgencyCandidateRecord {
            candidate_id: "cand-custom-review".to_string(),
            candidate_kind: "review_and_refine".to_string(),
            bounded_action: "review and refine the candidate".to_string(),
            review_requirement: "review_required".to_string(),
            execution_priority: 1,
            rationale: "custom selected candidate reason".to_string(),
        }],
        selected_candidate_id: "cand-custom-review".to_string(),
        selected_candidate_reason: "custom selected candidate reason".to_string(),
        deterministic_selection_rule: "deterministic".to_string(),
    };
    let evaluation = run_artifacts::EvaluationSignalsArtifact {
        evaluation_signals_version: 1,
        run_id: "mediation-run".to_string(),
        generated_from: generated_from.clone(),
        selected_candidate_id: "cand-custom-review".to_string(),
        selected_path: "slow_path".to_string(),
        progress_signal: "guarded_progress".to_string(),
        contradiction_signal: "present".to_string(),
        failure_signal: "none".to_string(),
        termination_reason: "contradiction_detected".to_string(),
        behavior_effect: "surface contradiction for bounded follow-up".to_string(),
        next_control_action: "handoff_to_reframing".to_string(),
        deterministic_evaluation_rule: "deterministic".to_string(),
    };
    let reframing = run_artifacts::ReframingArtifact {
        reframing_version: 1,
        run_id: "mediation-run".to_string(),
        generated_from: generated_from.clone(),
        selected_candidate_id: "cand-custom-review".to_string(),
        selected_path: "slow_path".to_string(),
        frame_adequacy_score: 24,
        reframing_trigger: "triggered".to_string(),
        reframing_reason: "contradiction requires bounded reframing before commitment".to_string(),
        prior_frame: "initial".to_string(),
        new_frame: "reframed".to_string(),
        reexecution_choice: "bounded_reframe_and_retry".to_string(),
        post_reframe_state: "ready_for_reframed_execution".to_string(),
        deterministic_reframing_rule: "deterministic".to_string(),
    };
    let freedom_gate = run_artifacts::FreedomGateArtifact {
        freedom_gate_version: 1,
        run_id: "mediation-run".to_string(),
        generated_from: generated_from.clone(),
        input: run_artifacts::FreedomGateInputState {
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
                progress_signal: "guarded_progress".to_string(),
                contradiction_signal: "present".to_string(),
                failure_signal: "none".to_string(),
                termination_reason: "contradiction_detected".to_string(),
            },
            consequence_context: execute::FreedomGateConsequenceContextState {
                impact_scope: "cross_surface".to_string(),
                recovery_cost: "requires_reframing".to_string(),
                operator_visibility: "review_required".to_string(),
                escalation_available: true,
            },
            frame_state: "ready_for_reframed_execution".to_string(),
        },
        gate_decision: "escalate".to_string(),
        reason_code: "frame_escalation_required".to_string(),
        decision_reason:
            "frame state and consequence context require explicit escalation before commitment can proceed"
                .to_string(),
        selected_action_or_none: None,
        commitment_blocked: true,
        judgment_boundary: "judgment_boundary".to_string(),
        required_follow_up: "escalate_for_judgment_review".to_string(),
        decision_record_kind: "gate_escalation_record".to_string(),
        deterministic_gate_rule: "deterministic".to_string(),
    };
    let scores = ScoresArtifact {
        scores_version: 1,
        run_id: "mediation-run".to_string(),
        generated_from: ScoresGeneratedFrom {
            artifact_model_version: artifacts::ARTIFACT_MODEL_VERSION,
            run_summary_version: 1,
        },
        summary: ScoresSummary {
            success_ratio: 0.0,
            failure_count: 1,
            retry_count: 0,
            delegation_denied_count: 0,
            security_denied_count: 0,
        },
        metrics: ScoresMetrics {
            scheduler_max_parallel_observed: 1,
        },
    };

    let proposals_left = run_artifacts::build_control_path_action_proposals_artifact(
        &run_summary,
        &arbitration,
        &agency,
        &freedom_gate,
        Some(&scores),
    );
    let proposals_right = run_artifacts::build_control_path_action_proposals_artifact(
        &run_summary,
        &arbitration,
        &agency,
        &freedom_gate,
        Some(&scores),
    );
    assert_eq!(
        serde_json::to_value(&proposals_left).expect("proposals left"),
        serde_json::to_value(&proposals_right).expect("proposals right")
    );
    assert_eq!(
        proposals_left.proposal_schema_name,
        "adl.runtime.action_proposal.v1"
    );
    assert_eq!(proposals_left.proposals.len(), 1);
    assert!(proposals_left.proposals[0].non_authoritative);
    assert_eq!(proposals_left.proposals[0].kind, "skill_call");
    assert_eq!(
        proposals_left.proposals[0].target.as_deref(),
        Some("candidate.review_and_refine")
    );
    assert!(proposals_left.proposals[0].requires_approval);

    let decisions = run_artifacts::build_control_path_decisions_artifact(
        &run_summary,
        &arbitration,
        &agency,
        &evaluation,
        &reframing,
        &freedom_gate,
        Some(&scores),
    );
    let mediation_left = run_artifacts::build_control_path_action_mediation_artifact(
        &run_summary,
        &proposals_left,
        &freedom_gate,
        &decisions,
        Some(&scores),
    );
    let mediation_right = run_artifacts::build_control_path_action_mediation_artifact(
        &run_summary,
        &proposals_right,
        &freedom_gate,
        &decisions,
        Some(&scores),
    );
    assert_eq!(
        serde_json::to_value(&mediation_left).expect("mediation left"),
        serde_json::to_value(&mediation_right).expect("mediation right")
    );
    assert_eq!(
        mediation_left.authority_boundary,
        "models_propose_runtime_decides_executes"
    );
    assert_eq!(mediation_left.mediation.mediation_outcome, "escalated");
    assert_eq!(mediation_left.mediation.runtime_authority, "freedom_gate");
    assert_eq!(
        mediation_left.mediation.required_follow_up,
        "escalate_for_judgment_review"
    );
    assert!(mediation_left.mediation.approved_action_or_none.is_none());

    let skill_model_left = run_artifacts::build_control_path_skill_model_artifact(
        &run_summary,
        &proposals_left,
        &mediation_left,
        Some(&scores),
    );
    let skill_model_right = run_artifacts::build_control_path_skill_model_artifact(
        &run_summary,
        &proposals_right,
        &mediation_right,
        Some(&scores),
    );
    assert_eq!(
        serde_json::to_value(&skill_model_left).expect("skill model left"),
        serde_json::to_value(&skill_model_right).expect("skill model right")
    );
    assert_eq!(
        skill_model_left.skill_schema_name,
        "adl.runtime.skill_model.v1"
    );
    assert_eq!(skill_model_left.selected_execution_unit_kind, "skill_call");
    assert_eq!(skill_model_left.skill.selection_status, "selected");
    assert_eq!(skill_model_left.skill.skill_id, "skill.review_and_refine");

    let skill_protocol_left = run_artifacts::build_control_path_skill_execution_protocol_artifact(
        &run_summary,
        &proposals_left,
        &skill_model_left,
        &mediation_left,
        Some(&scores),
    );
    let skill_protocol_right = run_artifacts::build_control_path_skill_execution_protocol_artifact(
        &run_summary,
        &proposals_right,
        &skill_model_right,
        &mediation_right,
        Some(&scores),
    );
    assert_eq!(
        serde_json::to_value(&skill_protocol_left).expect("skill protocol left"),
        serde_json::to_value(&skill_protocol_right).expect("skill protocol right")
    );
    assert_eq!(
        skill_protocol_left.protocol_name,
        "adl.runtime.skill_execution_protocol.v1"
    );
    assert_eq!(
        skill_protocol_left.invocation.lifecycle_state,
        "escalated_before_execution"
    );
    assert_eq!(
        skill_protocol_left.invocation.authorization_decision,
        "escalated"
    );
    assert_eq!(
        skill_protocol_left.invocation.skill_id,
        "skill.review_and_refine"
    );
}

#[test]
fn build_reasoning_graph_artifact_changes_selected_path_with_affect() {
    let summary = RunSummaryArtifact {
        run_summary_version: 1,
        artifact_model_version: artifacts::ARTIFACT_MODEL_VERSION,
        run_id: "reasoning-graph-run".to_string(),
        workflow_id: "wf".to_string(),
        adl_version: "0.85".to_string(),
        swarm_version: "test".to_string(),
        status: "failure".to_string(),
        error_kind: None,
        counts: RunSummaryCounts {
            total_steps: 1,
            completed_steps: 1,
            failed_steps: 1,
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
    let failure_scores = ScoresArtifact {
        scores_version: 1,
        run_id: "reasoning-graph-run".to_string(),
        generated_from: ScoresGeneratedFrom {
            artifact_model_version: artifacts::ARTIFACT_MODEL_VERSION,
            run_summary_version: 1,
        },
        summary: ScoresSummary {
            success_ratio: 0.0,
            failure_count: 1,
            retry_count: 0,
            delegation_denied_count: 0,
            security_denied_count: 0,
        },
        metrics: ScoresMetrics {
            scheduler_max_parallel_observed: 1,
        },
    };
    let failure_suggestions = build_suggestions_artifact(&summary, Some(&failure_scores));
    let failure_affect = run_artifacts::build_affect_state_artifact(
        &summary,
        &failure_suggestions,
        Some(&failure_scores),
    );
    let failure_decision = build_aee_decision_artifact(
        &summary,
        &failure_suggestions,
        &failure_affect,
        Some(&failure_scores),
    );
    let failure_graph = run_artifacts::build_reasoning_graph_artifact(
        &summary,
        &failure_affect,
        &failure_decision,
        Some(&failure_scores),
    );

    assert_eq!(failure_graph.graph.dominant_affect_mode, "recovery_focus");
    assert_eq!(
        failure_graph.graph.selected_path.selected_node_id,
        "action.retry_budget"
    );
    assert_eq!(
        failure_graph.graph.selected_path.selected_intent,
        "increase_step_retry_budget"
    );

    let success_summary = RunSummaryArtifact {
        run_summary_version: 1,
        artifact_model_version: artifacts::ARTIFACT_MODEL_VERSION,
        run_id: "reasoning-graph-run".to_string(),
        workflow_id: "wf".to_string(),
        adl_version: "0.85".to_string(),
        swarm_version: "test".to_string(),
        status: "success".to_string(),
        error_kind: None,
        counts: RunSummaryCounts {
            total_steps: 1,
            completed_steps: 1,
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
        summary: ScoresSummary {
            success_ratio: 1.0,
            failure_count: 0,
            retry_count: 0,
            delegation_denied_count: 0,
            security_denied_count: 0,
        },
        ..failure_scores
    };
    let success_suggestions = build_suggestions_artifact(&success_summary, Some(&success_scores));
    let success_affect = run_artifacts::build_affect_state_artifact(
        &success_summary,
        &success_suggestions,
        Some(&success_scores),
    );
    let success_decision = build_aee_decision_artifact(
        &success_summary,
        &success_suggestions,
        &success_affect,
        Some(&success_scores),
    );
    let success_graph = run_artifacts::build_reasoning_graph_artifact(
        &success_summary,
        &success_affect,
        &success_decision,
        Some(&success_scores),
    );

    assert_eq!(success_graph.graph.dominant_affect_mode, "steady_state");
    assert_eq!(
        success_graph.graph.selected_path.selected_node_id,
        "action.maintain_policy"
    );
    assert_eq!(
        success_graph.graph.nodes[2].node_id,
        "action.maintain_policy"
    );
    assert_eq!(success_graph.graph.nodes[2].rank, 1);
}
