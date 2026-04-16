use super::super::*;

fn security_denied_count(run_summary: &RunSummaryArtifact) -> usize {
    run_summary.policy.security_denials_by_code.values().sum()
}

pub(crate) fn control_path_security_posture(
    run_summary: &RunSummaryArtifact,
    freedom_gate: &FreedomGateArtifact,
) -> &'static str {
    let security_denials = security_denied_count(run_summary);
    if security_denials > 0
        || run_summary.policy.security_envelope_enabled
        || run_summary.policy.signing_required
        || freedom_gate.commitment_blocked
    {
        "hardened_review_first"
    } else if freedom_gate.input.policy_context.requires_review
        || freedom_gate.input.risk_class == "high"
    {
        "guarded_review_required"
    } else {
        "bounded_routine"
    }
}

pub(crate) fn control_path_security_attacker_pressure(
    run_summary: &RunSummaryArtifact,
    arbitration: &CognitiveArbitrationArtifact,
) -> &'static str {
    let security_denials = security_denied_count(run_summary);
    if security_denials > 0 || arbitration.risk_class == "high" {
        "contested"
    } else if arbitration.risk_class == "bounded" || arbitration.route_selected == "slow" {
        "guarded"
    } else {
        "ordinary"
    }
}

pub(crate) fn control_path_security_review_surfaces() -> Vec<String> {
    vec![
        "run_summary.json".to_string(),
        "control_path/freedom_gate.json".to_string(),
        "control_path/mediation.json".to_string(),
        "control_path/final_result.json".to_string(),
        "logs/trace_v1.json".to_string(),
    ]
}

pub(crate) fn control_path_security_boundaries(run_summary: &RunSummaryArtifact) -> Vec<String> {
    let mut boundaries = vec![
        "proposal_authority_boundary".to_string(),
        "freedom_gate_commitment_boundary".to_string(),
        "trace_visibility_boundary".to_string(),
        "memory_provenance_boundary".to_string(),
    ];
    if run_summary.policy.security_envelope_enabled || run_summary.policy.signing_required {
        boundaries.push("remote_execution_envelope_boundary".to_string());
    }
    boundaries
}

pub(crate) fn control_path_security_threat_classes(
    run_summary: &RunSummaryArtifact,
) -> Vec<String> {
    let mut threats = vec![
        "unreviewed_high_risk_action".to_string(),
        "proposal_authority_bypass".to_string(),
        "artifact_or_trace_visibility_loss".to_string(),
        "memory_provenance_confusion".to_string(),
    ];
    if run_summary.policy.security_envelope_enabled || run_summary.policy.signing_required {
        threats.push("transport_or_signature_tampering".to_string());
    }
    threats
}

pub(crate) fn control_path_security_required_mitigations(
    run_summary: &RunSummaryArtifact,
) -> Vec<String> {
    let mut mitigations = vec![
        "freedom_gate_judgment_boundary".to_string(),
        "trace_visible_mediation".to_string(),
        "bounded_skill_authorization".to_string(),
        "memory_provenance_tags".to_string(),
        format!("sandbox_policy={}", run_summary.policy.sandbox_policy),
    ];
    if run_summary.policy.security_envelope_enabled {
        mitigations.push("remote_security_envelope".to_string());
    }
    if run_summary.policy.signing_required {
        mitigations.push("signed_execute_requests".to_string());
    }
    if run_summary.policy.key_id_required {
        mitigations.push("key_id_required".to_string());
    }
    mitigations
}

pub(crate) fn control_path_security_trust_state(
    action_proposals: &ControlPathActionProposalsArtifact,
    mediation: &ControlPathActionMediationArtifact,
    freedom_gate: &FreedomGateArtifact,
) -> &'static str {
    let proposal_non_authoritative = action_proposals
        .proposals
        .first()
        .map(|proposal| proposal.non_authoritative)
        .unwrap_or(true);
    if freedom_gate.commitment_blocked || mediation.mediation.mediation_outcome != "approved" {
        "reduced_until_review"
    } else if proposal_non_authoritative {
        "authorized_only_after_mediation"
    } else {
        "bounded_authorized"
    }
}

pub(crate) fn control_path_security_reduced_trust_surfaces(
    run_summary: &RunSummaryArtifact,
    memory: &ControlPathMemoryArtifact,
) -> Vec<String> {
    let mut surfaces = vec![
        "control_path/action_proposals.json".to_string(),
        "control_path/candidate_selection.json".to_string(),
    ];
    if memory.read.read_count > 0 {
        surfaces.push("control_path/memory.json".to_string());
    }
    if run_summary.policy.security_envelope_enabled || run_summary.policy.signing_required {
        surfaces.push("remote_exec/request_envelope".to_string());
    }
    surfaces
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn build_control_path_security_review_artifact(
    run_summary: &RunSummaryArtifact,
    arbitration: &CognitiveArbitrationArtifact,
    action_proposals: &ControlPathActionProposalsArtifact,
    mediation: &ControlPathActionMediationArtifact,
    freedom_gate: &FreedomGateArtifact,
    memory: &ControlPathMemoryArtifact,
    final_result: &ControlPathFinalResultArtifact,
    scores: Option<&ScoresArtifact>,
) -> ControlPathSecurityReviewArtifact {
    let security_denials = security_denied_count(run_summary);
    let declared_posture = control_path_security_posture(run_summary, freedom_gate);
    let trust_state = control_path_security_trust_state(action_proposals, mediation, freedom_gate);
    let runtime_consequence = match freedom_gate.gate_decision.as_str() {
        "allow" => {
            "bounded action may proceed only after runtime mediation and trace-visible authorization"
        }
        "defer" => "bounded action must wait for further review before commitment",
        "refuse" => "bounded action is blocked before commitment and surfaced as an explicit refusal",
        "escalate" => "bounded action must escalate for judgment review before commitment",
        _ => "bounded action is held behind an unrecognized gate decision and cannot commit",
    };

    ControlPathSecurityReviewArtifact {
        control_path_security_review_version: CONTROL_PATH_SECURITY_REVIEW_VERSION,
        run_id: run_summary.run_id.clone(),
        generated_from: AeeDecisionGeneratedFrom {
            artifact_model_version: run_summary.artifact_model_version,
            run_summary_version: run_summary.run_summary_version,
            suggestions_version: mediation.generated_from.suggestions_version,
            scores_version: scores.map(|value| value.scores_version),
        },
        threat_model: SecurityThreatModelRecord {
            attacker_pressure: control_path_security_attacker_pressure(run_summary, arbitration)
                .to_string(),
            active_trust_boundaries: control_path_security_boundaries(run_summary),
            canonical_threat_classes: control_path_security_threat_classes(run_summary),
            required_mitigations: control_path_security_required_mitigations(run_summary),
            reviewer_visible_surfaces: control_path_security_review_surfaces(),
        },
        posture: SecurityPostureRecord {
            declared_posture: declared_posture.to_string(),
            accepted_risk_level: freedom_gate.input.risk_class.clone(),
            commitment_policy: freedom_gate.gate_decision.clone(),
            mitigation_authority: mediation.mediation.runtime_authority.clone(),
            runtime_consequence: runtime_consequence.to_string(),
            posture_rationale: format!(
                "risk_class={} security_denied_count={} route_selected={} gate_decision={}",
                freedom_gate.input.risk_class,
                security_denials,
                arbitration.route_selected,
                freedom_gate.gate_decision
            ),
        },
        trust_under_adversary: SecurityTrustUnderAdversaryRecord {
            trust_state: trust_state.to_string(),
            trusted_surfaces: control_path_security_review_surfaces(),
            reduced_trust_surfaces: control_path_security_reduced_trust_surfaces(
                run_summary,
                memory,
            ),
            revalidation_requirements: mediation.mediation.validation_checks.clone(),
            escalation_path: freedom_gate.required_follow_up.clone(),
        },
        evidence: SecurityReviewEvidenceRecord {
            route_selected: arbitration.route_selected.clone(),
            risk_class: freedom_gate.input.risk_class.clone(),
            mediation_outcome: mediation.mediation.mediation_outcome.clone(),
            gate_decision: freedom_gate.gate_decision.clone(),
            final_result: final_result.final_result.clone(),
            security_denied_count: security_denials,
            security_envelope_enabled: run_summary.policy.security_envelope_enabled,
            signing_required: run_summary.policy.signing_required,
            key_id_required: run_summary.policy.key_id_required,
            verify_allowed_algs: run_summary.policy.verify_allowed_algs.clone(),
            verify_allowed_key_sources: run_summary.policy.verify_allowed_key_sources.clone(),
            sandbox_policy: run_summary.policy.sandbox_policy.clone(),
            trace_visibility_expectation: mediation.mediation.trace_expectation.clone(),
        },
    }
}

pub(crate) fn build_control_path_summary(context: &ControlPathSummaryContext<'_>) -> String {
    let signals = context.signals;
    let agency = context.agency;
    let arbitration = context.arbitration;
    let execution = context.execution;
    let evaluation = context.evaluation;
    let reframing = context.reframing;
    let convergence = context.convergence;
    let memory = context.memory;
    let action_proposals = context.action_proposals;
    let skill_model = context.skill_model;
    let skill_execution_protocol = context.skill_execution_protocol;
    let mediation = context.mediation;
    let freedom_gate = context.freedom_gate;
    let final_result = context.final_result;
    let security_review = context.security_review;
    let proposal = action_proposals
        .proposals
        .first()
        .expect("control_path summary requires one proposal");

    [
        "v0.86 canonical bounded cognitive path summary".to_string(),
        format!("run_id: {}", final_result.run_id),
        "stage_order: signals -> candidate_selection -> arbitration -> execution -> evaluation -> reframing -> memory -> freedom_gate -> final_result".to_string(),
        format!(
            "signals: instinct={} completion_pressure={}",
            signals.instinct.dominant_instinct, signals.instinct.completion_pressure
        ),
        format!(
            "candidate_selection: candidate_id={} rationale={}",
            agency.selected_candidate_id, agency.selected_candidate_reason
        ),
        format!(
            "arbitration: route={} reasoning_mode={}",
            arbitration.route_selected, arbitration.reasoning_mode
        ),
        format!(
            "execution: status={} iterations={}",
            execution.execution_status, execution.iteration_count
        ),
        format!(
            "evaluation: termination_reason={} next_control_action={}",
            evaluation.termination_reason, evaluation.next_control_action
        ),
        format!(
            "reframing: trigger={} choice={}",
            reframing.reframing_trigger, reframing.reexecution_choice
        ),
        format!(
            "convergence: state={} stop_condition_family={} progress_signal={}",
            convergence.convergence_state,
            convergence.stop_condition_family,
            convergence.progress_signal
        ),
        format!(
            "decisions: route_selection={} reframing={} commitment_gate={}",
            if arbitration.route_selected == "fast" {
                "accept"
            } else {
                "reroute"
            },
            if reframing.reframing_trigger == "triggered" {
                "reroute"
            } else {
                "accept"
            },
            match freedom_gate.gate_decision.as_str() {
                "allow" => "accept",
                "refuse" => "reject",
                "defer" => "defer",
                "escalate" => "escalate",
                _ => "reject",
            }
        ),
        format!(
            "action_proposal: kind={} target={} requires_approval={}",
            proposal.kind,
            proposal.target.clone().unwrap_or_else(|| "<none>".to_string()),
            proposal.requires_approval
        ),
        format!(
            "action_mediation: outcome={} authority={} follow_up={}",
            mediation.mediation.mediation_outcome,
            mediation.mediation.runtime_authority,
            mediation.mediation.required_follow_up
        ),
        format!(
            "skill_model: selection_status={} skill_id={} invocation_kind={}",
            skill_model.skill.selection_status,
            skill_model.skill.skill_id,
            skill_model.selected_execution_unit_kind
        ),
        format!(
            "skill_execution_protocol: lifecycle_state={} authorization={} trace_expectation={}",
            skill_execution_protocol.invocation.lifecycle_state,
            skill_execution_protocol.invocation.authorization_decision,
            skill_execution_protocol.invocation.trace_expectation
        ),
        format!(
            "memory: read_count={} influenced_stage={} write_reason={}",
            memory.read.read_count, memory.read.influenced_stage, memory.write.write_reason
        ),
        format!(
            "freedom_gate: decision={} reason_code={} follow_up={} commitment_blocked={}",
            freedom_gate.gate_decision,
            freedom_gate.reason_code,
            freedom_gate.required_follow_up,
            freedom_gate.commitment_blocked
        ),
        format!(
            "security_review: posture={} trust_state={} attacker_pressure={}",
            security_review.posture.declared_posture,
            security_review.trust_under_adversary.trust_state,
            security_review.threat_model.attacker_pressure
        ),
        format!("final_result: {}", final_result.final_result),
    ]
    .join("\n")
}
