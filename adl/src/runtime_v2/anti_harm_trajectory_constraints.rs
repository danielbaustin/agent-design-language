//! Runtime-v2 anti-harm trajectory constraints.
//!
//! WP-08 consumes the moral-trace, outcome-linkage, moral-metric, and
//! trajectory-review surfaces and moves from action-only refusal to bounded
//! trajectory-aware anti-harm review. The packet must stay synthetic,
//! deterministic, and non-operational while proving that harmful trajectories
//! can be detected across multiple benign-looking steps.

use super::*;

pub const ANTI_HARM_TRAJECTORY_CONSTRAINT_PACKET_SCHEMA_VERSION: &str =
    "anti_harm_trajectory_constraint_packet.v1";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AntiHarmTrajectoryConstraint {
    pub constraint_id: String,
    pub harm_mode: String,
    pub protected_boundary: String,
    pub evidence_field_refs: Vec<String>,
    pub detection_summary: String,
    pub denial_rule: String,
    pub escalation_rule: String,
    pub limitations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SyntheticDelegatedHarmScenario {
    pub scenario_id: String,
    pub scenario_kind: String,
    pub summary: String,
    pub individually_benign_trace_refs: Vec<String>,
    pub trajectory_window_id: String,
    pub supporting_outcome_linkage_refs: Vec<String>,
    pub risk_modes: Vec<String>,
    pub detection_basis: String,
    pub claim_boundary: String,
    pub limitations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AntiHarmDecisionRecord {
    pub decision_id: String,
    pub scenario_id: String,
    pub decision_kind: String,
    pub record_status: String,
    pub triggered_constraint_ids: Vec<String>,
    pub trajectory_finding_refs: Vec<String>,
    pub trace_evidence_refs: Vec<String>,
    pub outcome_linkage_refs: Vec<String>,
    pub summary: String,
    pub non_operational_boundary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AntiHarmTrajectoryConstraintPacket {
    pub schema_version: String,
    pub packet_id: String,
    pub summary: String,
    pub interpretation_boundary: String,
    pub deterministic_ordering_rule: String,
    pub constraints: Vec<AntiHarmTrajectoryConstraint>,
    pub synthetic_scenarios: Vec<SyntheticDelegatedHarmScenario>,
    pub decisions: Vec<AntiHarmDecisionRecord>,
}

pub fn anti_harm_trajectory_constraint_packet() -> Result<AntiHarmTrajectoryConstraintPacket> {
    let trace_examples = moral_trace_required_examples();
    let outcome_examples = outcome_linkage_required_examples();
    let trajectory_packet = moral_trajectory_review_packet()?;
    let _metric_report = moral_metric_fixture_report()?;

    let ordinary_trace = trace_examples
        .iter()
        .find(|example| example.example_kind == MoralTraceExampleKind::OrdinaryAction)
        .ok_or_else(|| anyhow!("WP-08 requires the ordinary-action moral trace example"))?
        .trace
        .clone();
    let refusal_trace = trace_examples
        .iter()
        .find(|example| example.example_kind == MoralTraceExampleKind::Refusal)
        .ok_or_else(|| anyhow!("WP-08 requires the refusal moral trace example"))?
        .trace
        .clone();
    let delegation_trace = trace_examples
        .iter()
        .find(|example| example.example_kind == MoralTraceExampleKind::Delegation)
        .ok_or_else(|| anyhow!("WP-08 requires the delegation moral trace example"))?
        .trace
        .clone();
    let deferred_trace = trace_examples
        .iter()
        .find(|example| example.example_kind == MoralTraceExampleKind::DeferredDecision)
        .ok_or_else(|| anyhow!("WP-08 requires the deferred-decision moral trace example"))?
        .trace
        .clone();

    let partial_outcome = outcome_examples
        .iter()
        .find(|example| example.example_kind == OutcomeLinkageExampleKind::Partial)
        .ok_or_else(|| anyhow!("WP-08 requires the partial outcome-linkage example"))?
        .record
        .clone();
    let delayed_outcome = outcome_examples
        .iter()
        .find(|example| example.example_kind == OutcomeLinkageExampleKind::Delayed)
        .ok_or_else(|| anyhow!("WP-08 requires the delayed outcome-linkage example"))?
        .record
        .clone();
    let contested_outcome = outcome_examples
        .iter()
        .find(|example| example.example_kind == OutcomeLinkageExampleKind::Contested)
        .ok_or_else(|| anyhow!("WP-08 requires the contested outcome-linkage example"))?
        .record
        .clone();

    let trajectory_window_id = trajectory_packet
        .windows
        .iter()
        .find(|window| window.window_id == "longitudinal-window-alpha")
        .ok_or_else(|| anyhow!("WP-08 requires the longitudinal trajectory review window"))?
        .window_id
        .clone();

    let refusal_finding_id = trajectory_packet
        .findings
        .iter()
        .find(|finding| finding.finding_id == "trajectory-finding-refusal-preserved")
        .ok_or_else(|| anyhow!("WP-08 requires the refusal trajectory finding"))?
        .finding_id
        .clone();
    let escalation_finding_id = trajectory_packet
        .findings
        .iter()
        .find(|finding| finding.finding_id == "trajectory-finding-escalation-active")
        .ok_or_else(|| anyhow!("WP-08 requires the escalation trajectory finding"))?
        .finding_id
        .clone();
    let uncertainty_finding_id = trajectory_packet
        .findings
        .iter()
        .find(|finding| finding.finding_id == "trajectory-finding-uncertainty-open")
        .ok_or_else(|| anyhow!("WP-08 requires the unresolved-uncertainty trajectory finding"))?
        .finding_id
        .clone();

    let constraints = anti_harm_constraints();
    let synthetic_scenarios = vec![SyntheticDelegatedHarmScenario {
        scenario_id: "synthetic-delegated-harm-scenario-alpha".to_string(),
        scenario_kind: "delegated_harm".to_string(),
        summary:
            "Synthetic multi-step trajectory where ordinary collection, delegated processing, and delayed review would become harmful only when composed across steps."
                .to_string(),
        individually_benign_trace_refs: ordered_trace_refs(&[
            ordinary_trace.clone(),
            delegation_trace.clone(),
            deferred_trace.clone(),
        ]),
        trajectory_window_id,
        supporting_outcome_linkage_refs: ordered_outcome_refs(&[
            partial_outcome.clone(),
            delayed_outcome.clone(),
            contested_outcome.clone(),
        ]),
        risk_modes: vec![
            "decomposed".to_string(),
            "delegated".to_string(),
            "delayed".to_string(),
            "disguised".to_string(),
        ],
        detection_basis: "cross_step_aggregation".to_string(),
        claim_boundary:
            "Synthetic, bounded, non-operational proof only; it does not authorize live harmful simulation or replace human or governance review."
                .to_string(),
        limitations: vec![
            "The scenario proves reviewable trajectory aggregation, not production-scale harm prediction."
                .to_string(),
            "Individually benign-looking steps are summarized at a high level and intentionally omit actionable harmful detail."
                .to_string(),
        ],
    }];

    let decisions = vec![
        AntiHarmDecisionRecord {
            decision_id: "anti-harm-escalation-record-alpha".to_string(),
            scenario_id: "synthetic-delegated-harm-scenario-alpha".to_string(),
            decision_kind: "escalate".to_string(),
            record_status: "review_needed".to_string(),
            triggered_constraint_ids: vec![
                "constraint-delegated-harm".to_string(),
                "constraint-delayed-harm".to_string(),
                "constraint-disguised-harm".to_string(),
            ],
            trajectory_finding_refs: vec![
                escalation_finding_id.clone(),
                uncertainty_finding_id.clone(),
            ],
            trace_evidence_refs: ordered_trace_refs(&[
                delegation_trace.clone(),
                deferred_trace.clone(),
            ]),
            outcome_linkage_refs: ordered_outcome_refs(&[
                delayed_outcome.clone(),
                contested_outcome.clone(),
            ]),
            summary:
                "Before outright denial, the synthetic trajectory stays escalated because delegated execution and delayed review could conceal downstream harm unless review remains active across steps."
                    .to_string(),
            non_operational_boundary:
                "Synthetic and non-operational escalation record only; it records review posture without giving live harm instructions."
                    .to_string(),
        },
        AntiHarmDecisionRecord {
            decision_id: "anti-harm-denial-record-alpha".to_string(),
            scenario_id: "synthetic-delegated-harm-scenario-alpha".to_string(),
            decision_kind: "deny".to_string(),
            record_status: "emitted".to_string(),
            triggered_constraint_ids: vec![
                "constraint-decomposed-harm".to_string(),
                "constraint-delegated-harm".to_string(),
                "constraint-delayed-harm".to_string(),
                "constraint-disguised-harm".to_string(),
            ],
            trajectory_finding_refs: vec![
                escalation_finding_id,
                refusal_finding_id,
                uncertainty_finding_id,
            ],
            trace_evidence_refs: ordered_trace_refs(&[
                ordinary_trace,
                delegation_trace,
                deferred_trace,
                refusal_trace,
            ]),
            outcome_linkage_refs: ordered_outcome_refs(&[
                partial_outcome,
                delayed_outcome,
                contested_outcome,
            ]),
            summary:
                "The runtime denies continuation once the benign-looking steps are aggregated into a synthetic delegated-harm trajectory against a protected party."
                    .to_string(),
            non_operational_boundary:
                "Synthetic and non-operational denial record only; it proves bounded refusal semantics without providing harmful operational guidance."
                    .to_string(),
        },
    ];

    let packet = AntiHarmTrajectoryConstraintPacket {
        schema_version: ANTI_HARM_TRAJECTORY_CONSTRAINT_PACKET_SCHEMA_VERSION.to_string(),
        packet_id: "anti-harm-trajectory-constraint-packet-alpha-001".to_string(),
        summary:
            "WP-08 binds anti-harm constraints to the existing trace, outcome, metric, and trajectory surfaces so reviewers can inspect delegated-harm prevention without live harmful execution."
                .to_string(),
        interpretation_boundary:
            "Interpret this packet as bounded anti-harm review evidence only. It is not a live harm classifier, not a replacement for human or governance review, and not operational harmful guidance."
                .to_string(),
        deterministic_ordering_rule:
            "Sort constraints by harm_mode then constraint_id. Sort scenarios by scenario_id. Sort decisions by scenario_id, decision_kind rank (escalate before deny), then decision_id."
                .to_string(),
        constraints,
        synthetic_scenarios,
        decisions,
    };

    validate_anti_harm_trajectory_constraint_packet(&packet)?;
    Ok(packet)
}

pub fn anti_harm_trajectory_constraint_json_bytes(
    packet: &AntiHarmTrajectoryConstraintPacket,
) -> Result<Vec<u8>> {
    validate_anti_harm_trajectory_constraint_packet(packet)?;
    let mut canonical = packet.clone();
    canonicalize_anti_harm_trajectory_constraint_packet(&mut canonical);
    serde_json::to_vec_pretty(&canonical)
        .context("serialize anti-harm trajectory constraint packet json")
}

pub fn validate_anti_harm_trajectory_constraint_packet(
    packet: &AntiHarmTrajectoryConstraintPacket,
) -> Result<()> {
    require_exact(
        &packet.schema_version,
        ANTI_HARM_TRAJECTORY_CONSTRAINT_PACKET_SCHEMA_VERSION,
        "anti_harm_trajectory_constraint_packet.schema_version",
    )?;
    normalize_id(
        packet.packet_id.clone(),
        "anti_harm_trajectory_constraint_packet.packet_id",
    )?;
    validate_nonempty_text(
        &packet.summary,
        "anti_harm_trajectory_constraint_packet.summary",
    )?;
    validate_nonempty_text(
        &packet.interpretation_boundary,
        "anti_harm_trajectory_constraint_packet.interpretation_boundary",
    )?;
    validate_nonempty_text(
        &packet.deterministic_ordering_rule,
        "anti_harm_trajectory_constraint_packet.deterministic_ordering_rule",
    )?;
    require_interpretation_boundary(&packet.interpretation_boundary)?;
    require_deterministic_ordering_rule(&packet.deterministic_ordering_rule)?;

    let required_harm_modes = ["decomposed", "delegated", "delayed", "disguised"];
    if packet.constraints.len() != required_harm_modes.len() {
        return Err(anyhow!(
            "anti_harm_trajectory_constraint_packet.constraints must include each required harm mode exactly once"
        ));
    }
    let mut constraint_ids = std::collections::BTreeSet::new();
    let mut harm_modes = std::collections::BTreeSet::new();
    for constraint in &packet.constraints {
        validate_constraint(constraint)?;
        if !constraint_ids.insert(constraint.constraint_id.clone()) {
            return Err(anyhow!(
                "anti_harm_trajectory_constraint_packet.constraints contains duplicate constraint_id"
            ));
        }
        if !harm_modes.insert(constraint.harm_mode.clone()) {
            return Err(anyhow!(
                "anti_harm_trajectory_constraint_packet.constraints contains duplicate harm_mode"
            ));
        }
    }
    for harm_mode in required_harm_modes {
        if !harm_modes.contains(harm_mode) {
            return Err(anyhow!(
                "anti_harm_trajectory_constraint_packet.constraints missing required harm mode"
            ));
        }
    }

    let trajectory_packet = moral_trajectory_review_packet()?;
    let window_ids = trajectory_packet
        .windows
        .iter()
        .map(|window| window.window_id.clone())
        .collect::<std::collections::BTreeSet<_>>();
    let finding_ids = trajectory_packet
        .findings
        .iter()
        .map(|finding| finding.finding_id.clone())
        .collect::<std::collections::BTreeSet<_>>();
    let known_trace_refs = moral_trace_required_examples()
        .into_iter()
        .map(|example| format!("trace:{}", example.trace.trace_id))
        .collect::<std::collections::BTreeSet<_>>();
    let known_outcome_linkage_refs = outcome_linkage_required_examples()
        .into_iter()
        .map(|example| format!("outcome-linkage:{}", example.record.linkage_id))
        .collect::<std::collections::BTreeSet<_>>();

    if packet.synthetic_scenarios.is_empty() {
        return Err(anyhow!(
            "anti_harm_trajectory_constraint_packet.synthetic_scenarios must not be empty"
        ));
    }
    let mut scenario_ids = std::collections::BTreeSet::new();
    for scenario in &packet.synthetic_scenarios {
        validate_synthetic_scenario(
            scenario,
            &window_ids,
            &harm_modes,
            &known_trace_refs,
            &known_outcome_linkage_refs,
        )?;
        if !scenario_ids.insert(scenario.scenario_id.clone()) {
            return Err(anyhow!(
                "anti_harm_trajectory_constraint_packet.synthetic_scenarios contains duplicate scenario_id"
            ));
        }
    }

    if packet.decisions.is_empty() {
        return Err(anyhow!(
            "anti_harm_trajectory_constraint_packet.decisions must not be empty"
        ));
    }
    let mut decision_ids = std::collections::BTreeSet::new();
    let mut decision_kinds = std::collections::BTreeSet::new();
    for decision in &packet.decisions {
        validate_decision_record(
            decision,
            &scenario_ids,
            &constraint_ids,
            &finding_ids,
            &known_trace_refs,
            &known_outcome_linkage_refs,
        )?;
        if !decision_ids.insert(decision.decision_id.clone()) {
            return Err(anyhow!(
                "anti_harm_trajectory_constraint_packet.decisions contains duplicate decision_id"
            ));
        }
        decision_kinds.insert(decision.decision_kind.clone());
    }
    for required_kind in ["deny", "escalate"] {
        if !decision_kinds.contains(required_kind) {
            return Err(anyhow!(
                "anti_harm_trajectory_constraint_packet.decisions must include both deny and escalate records"
            ));
        }
    }

    Ok(())
}

fn anti_harm_constraints() -> Vec<AntiHarmTrajectoryConstraint> {
    vec![
        AntiHarmTrajectoryConstraint {
            constraint_id: "constraint-decomposed-harm".to_string(),
            harm_mode: "decomposed".to_string(),
            protected_boundary:
                "A trajectory that distributes harm across multiple benign-looking steps must still fail closed once the aggregate path is reviewably harmful."
                    .to_string(),
            evidence_field_refs: vec![
                "moral_trace.outcome.downstream_effect_refs".to_string(),
                "outcome_linkage.linked_outcomes.effect_summary".to_string(),
                "moral_trajectory_review.findings.summary".to_string(),
            ],
            detection_summary:
                "Look for cumulative downstream effects that are not unsafe in isolation but become unsafe when a reviewer can compose them across the trajectory."
                    .to_string(),
            denial_rule:
                "Deny when cross-step aggregation shows a protected-party harm trajectory even though no single step was independently forbidden."
                    .to_string(),
            escalation_rule:
                "Escalate while effects remain partial or delayed but the composed path already suggests non-trivial downstream harm."
                    .to_string(),
            limitations: vec![
                "Decomposition review depends on explicit trace and linkage evidence rather than hidden intent inference."
                    .to_string(),
            ],
        },
        AntiHarmTrajectoryConstraint {
            constraint_id: "constraint-delegated-harm".to_string(),
            harm_mode: "delegated".to_string(),
            protected_boundary:
                "Delegation cannot be used to move harmful work out of the accountable actor's review surface."
                    .to_string(),
            evidence_field_refs: vec![
                "moral_trace.attribution.delegated_by_trace_ref".to_string(),
                "moral_trace.attribution.delegate_trace_ref".to_string(),
                "outcome_linkage.attribution.delegate_trace_ref".to_string(),
                "outcome_linkage.linked_outcomes.downstream_actor_refs".to_string(),
            ],
            detection_summary:
                "Track whether delegated work preserves parent and child lineage while increasing protected-party risk across the trajectory."
                    .to_string(),
            denial_rule:
                "Deny when delegated execution would complete a harmful trajectory that the accountable actor could not safely complete directly."
                    .to_string(),
            escalation_rule:
                "Escalate whenever delegated work carries unresolved protected-party risk or ambiguous downstream responsibility."
                    .to_string(),
            limitations: vec![
                "Visible delegation lineage is necessary but not sufficient; safe delegation still requires the downstream trajectory to remain non-harmful."
                    .to_string(),
            ],
        },
        AntiHarmTrajectoryConstraint {
            constraint_id: "constraint-delayed-harm".to_string(),
            harm_mode: "delayed".to_string(),
            protected_boundary:
                "A trajectory does not become safe merely because its harmful effect is delayed or still under review."
                    .to_string(),
            evidence_field_refs: vec![
                "moral_trace.outcome.outcome_kind".to_string(),
                "outcome_linkage.linked_outcomes.outcome_status".to_string(),
                "outcome_linkage.linked_outcomes.uncertainty_refs".to_string(),
            ],
            detection_summary:
                "Preserve escalation when delayed or uncertain outcomes could still convert the trajectory into protected-party harm later."
                    .to_string(),
            denial_rule:
                "Deny once delayed evidence converges on a harmful aggregate trajectory instead of waiting for final damage manifestation."
                    .to_string(),
            escalation_rule:
                "Escalate while delayed or contested effects remain open so review does not falsely close the hazard."
                    .to_string(),
            limitations: vec![
                "Delayed-harm review must remain bounded to explicit uncertainty and contestation evidence."
                    .to_string(),
            ],
        },
        AntiHarmTrajectoryConstraint {
            constraint_id: "constraint-disguised-harm".to_string(),
            harm_mode: "disguised".to_string(),
            protected_boundary:
                "A public-safe or benign-looking description must not hide a protected-party harm trajectory from reviewers."
                    .to_string(),
            evidence_field_refs: vec![
                "moral_trace.visibility.public_summary".to_string(),
                "moral_trace.visibility.reviewer_evidence_refs".to_string(),
                "outcome_linkage.linked_outcomes.rebuttal_refs".to_string(),
                "moral_trajectory_review.findings.signal_kind".to_string(),
            ],
            detection_summary:
                "Compare public-safe descriptions with reviewer evidence and rebuttal surfaces to catch trajectories that look harmless until composed."
                    .to_string(),
            denial_rule:
                "Deny when the only path to completion depends on hiding or minimizing the protected-party harm trajectory."
                    .to_string(),
            escalation_rule:
                "Escalate while reviewer evidence indicates that the public-safe framing may be understating aggregate harm."
                    .to_string(),
            limitations: vec![
                "Disguised-harm detection proves reviewable discrepancy, not omniscient access to hidden intent."
                    .to_string(),
            ],
        },
    ]
}

fn validate_constraint(constraint: &AntiHarmTrajectoryConstraint) -> Result<()> {
    normalize_id(
        constraint.constraint_id.clone(),
        "anti_harm_trajectory_constraint.constraint_id",
    )?;
    match constraint.harm_mode.as_str() {
        "decomposed" | "delegated" | "delayed" | "disguised" => {}
        _ => {
            return Err(anyhow!(
                "anti_harm_trajectory_constraint.harm_mode unsupported"
            ))
        }
    }
    validate_nonempty_text(
        &constraint.protected_boundary,
        "anti_harm_trajectory_constraint.protected_boundary",
    )?;
    validate_nonempty_text(
        &constraint.detection_summary,
        "anti_harm_trajectory_constraint.detection_summary",
    )?;
    validate_nonempty_text(
        &constraint.denial_rule,
        "anti_harm_trajectory_constraint.denial_rule",
    )?;
    validate_nonempty_text(
        &constraint.escalation_rule,
        "anti_harm_trajectory_constraint.escalation_rule",
    )?;
    if constraint.evidence_field_refs.is_empty() {
        return Err(anyhow!(
            "anti_harm_trajectory_constraint.evidence_field_refs must not be empty"
        ));
    }
    for value in &constraint.evidence_field_refs {
        validate_constraint_evidence_field_ref(
            value,
            "anti_harm_trajectory_constraint.evidence_field_refs",
        )?;
    }
    if constraint.limitations.is_empty() {
        return Err(anyhow!(
            "anti_harm_trajectory_constraint.limitations must not be empty"
        ));
    }
    for limitation in &constraint.limitations {
        validate_nonempty_text(limitation, "anti_harm_trajectory_constraint.limitations")?;
    }
    Ok(())
}

fn validate_synthetic_scenario(
    scenario: &SyntheticDelegatedHarmScenario,
    window_ids: &std::collections::BTreeSet<String>,
    harm_modes: &std::collections::BTreeSet<String>,
    known_trace_refs: &std::collections::BTreeSet<String>,
    known_outcome_linkage_refs: &std::collections::BTreeSet<String>,
) -> Result<()> {
    normalize_id(
        scenario.scenario_id.clone(),
        "anti_harm_synthetic_scenario.scenario_id",
    )?;
    require_exact(
        &scenario.scenario_kind,
        "delegated_harm",
        "anti_harm_synthetic_scenario.scenario_kind",
    )?;
    validate_nonempty_text(&scenario.summary, "anti_harm_synthetic_scenario.summary")?;
    if scenario.individually_benign_trace_refs.len() < 3 {
        return Err(anyhow!(
            "anti_harm_synthetic_scenario.individually_benign_trace_refs must preserve a cross-step trajectory rather than a single-step veto"
        ));
    }
    let mut trace_refs = std::collections::BTreeSet::new();
    for trace_ref in &scenario.individually_benign_trace_refs {
        validate_prefixed_ref(
            trace_ref,
            "trace:",
            "anti_harm_synthetic_scenario.individually_benign_trace_refs",
        )?;
        if !known_trace_refs.contains(trace_ref) {
            return Err(anyhow!(
                "anti_harm_synthetic_scenario.individually_benign_trace_refs must refer to known WP-04 trace examples"
            ));
        }
        if !trace_refs.insert(trace_ref.clone()) {
            return Err(anyhow!(
                "anti_harm_synthetic_scenario.individually_benign_trace_refs must contain distinct cross-step trace refs"
            ));
        }
    }
    if !window_ids.contains(&scenario.trajectory_window_id) {
        return Err(anyhow!(
            "anti_harm_synthetic_scenario.trajectory_window_id must refer to a known trajectory-review window"
        ));
    }
    for outcome_ref in &scenario.supporting_outcome_linkage_refs {
        validate_prefixed_ref(
            outcome_ref,
            "outcome-linkage:",
            "anti_harm_synthetic_scenario.supporting_outcome_linkage_refs",
        )?;
        if !known_outcome_linkage_refs.contains(outcome_ref) {
            return Err(anyhow!(
                "anti_harm_synthetic_scenario.supporting_outcome_linkage_refs must refer to known WP-05 outcome-linkage examples"
            ));
        }
    }
    if scenario.risk_modes.is_empty() {
        return Err(anyhow!(
            "anti_harm_synthetic_scenario.risk_modes must not be empty"
        ));
    }
    let mut scenario_modes = std::collections::BTreeSet::new();
    for risk_mode in &scenario.risk_modes {
        match risk_mode.as_str() {
            "decomposed" | "delegated" | "delayed" | "disguised" => {}
            _ => {
                return Err(anyhow!(
                    "anti_harm_synthetic_scenario.risk_modes unsupported"
                ))
            }
        }
        if !harm_modes.contains(risk_mode) {
            return Err(anyhow!(
                "anti_harm_synthetic_scenario.risk_modes must refer to declared constraint harm modes"
            ));
        }
        scenario_modes.insert(risk_mode.clone());
    }
    for required_mode in ["decomposed", "delegated", "delayed", "disguised"] {
        if !scenario_modes.contains(required_mode) {
            return Err(anyhow!(
                "anti_harm_synthetic_scenario.risk_modes must cover every required harm mode in the delegated-harm proof"
            ));
        }
    }
    require_exact(
        &scenario.detection_basis,
        "cross_step_aggregation",
        "anti_harm_synthetic_scenario.detection_basis",
    )?;
    validate_nonempty_text(
        &scenario.claim_boundary,
        "anti_harm_synthetic_scenario.claim_boundary",
    )?;
    require_non_operational_boundary(
        &scenario.claim_boundary,
        "anti_harm_synthetic_scenario.claim_boundary",
    )?;
    if scenario.limitations.is_empty() {
        return Err(anyhow!(
            "anti_harm_synthetic_scenario.limitations must not be empty"
        ));
    }
    for limitation in &scenario.limitations {
        validate_nonempty_text(limitation, "anti_harm_synthetic_scenario.limitations")?;
    }
    Ok(())
}

fn validate_decision_record(
    decision: &AntiHarmDecisionRecord,
    scenario_ids: &std::collections::BTreeSet<String>,
    constraint_ids: &std::collections::BTreeSet<String>,
    finding_ids: &std::collections::BTreeSet<String>,
    known_trace_refs: &std::collections::BTreeSet<String>,
    known_outcome_linkage_refs: &std::collections::BTreeSet<String>,
) -> Result<()> {
    normalize_id(
        decision.decision_id.clone(),
        "anti_harm_decision_record.decision_id",
    )?;
    if !scenario_ids.contains(&decision.scenario_id) {
        return Err(anyhow!(
            "anti_harm_decision_record.scenario_id must refer to a known synthetic scenario"
        ));
    }
    match decision.decision_kind.as_str() {
        "deny" | "escalate" => {}
        _ => {
            return Err(anyhow!(
                "anti_harm_decision_record.decision_kind unsupported"
            ))
        }
    }
    match decision.record_status.as_str() {
        "emitted" | "review_needed" => {}
        _ => {
            return Err(anyhow!(
                "anti_harm_decision_record.record_status unsupported"
            ))
        }
    }
    if decision.triggered_constraint_ids.is_empty() {
        return Err(anyhow!(
            "anti_harm_decision_record.triggered_constraint_ids must not be empty"
        ));
    }
    for constraint_id in &decision.triggered_constraint_ids {
        if !constraint_ids.contains(constraint_id) {
            return Err(anyhow!(
                "anti_harm_decision_record.triggered_constraint_ids must refer to known constraints"
            ));
        }
    }
    if decision.trajectory_finding_refs.is_empty() {
        return Err(anyhow!(
            "anti_harm_decision_record.trajectory_finding_refs must not be empty"
        ));
    }
    for finding_id in &decision.trajectory_finding_refs {
        if !finding_ids.contains(finding_id) {
            return Err(anyhow!(
                "anti_harm_decision_record.trajectory_finding_refs must refer to known trajectory findings"
            ));
        }
    }
    if decision.trace_evidence_refs.is_empty() {
        return Err(anyhow!(
            "anti_harm_decision_record.trace_evidence_refs must cite trace evidence directly"
        ));
    }
    for trace_ref in &decision.trace_evidence_refs {
        validate_prefixed_ref(
            trace_ref,
            "trace:",
            "anti_harm_decision_record.trace_evidence_refs",
        )?;
        if !known_trace_refs.contains(trace_ref) {
            return Err(anyhow!(
                "anti_harm_decision_record.trace_evidence_refs must refer to known WP-04 trace examples"
            ));
        }
    }
    for outcome_ref in &decision.outcome_linkage_refs {
        validate_prefixed_ref(
            outcome_ref,
            "outcome-linkage:",
            "anti_harm_decision_record.outcome_linkage_refs",
        )?;
        if !known_outcome_linkage_refs.contains(outcome_ref) {
            return Err(anyhow!(
                "anti_harm_decision_record.outcome_linkage_refs must refer to known WP-05 outcome-linkage examples"
            ));
        }
    }
    validate_nonempty_text(&decision.summary, "anti_harm_decision_record.summary")?;
    validate_nonempty_text(
        &decision.non_operational_boundary,
        "anti_harm_decision_record.non_operational_boundary",
    )?;
    require_non_operational_boundary(
        &decision.non_operational_boundary,
        "anti_harm_decision_record.non_operational_boundary",
    )?;
    Ok(())
}

fn ordered_trace_refs(traces: &[MoralTraceRecord]) -> Vec<String> {
    let mut refs = traces
        .iter()
        .map(|trace| (trace.trace_sequence, format!("trace:{}", trace.trace_id)))
        .collect::<Vec<_>>();
    refs.sort();
    refs.into_iter().map(|(_, value)| value).collect()
}

fn ordered_outcome_refs(outcomes: &[OutcomeLinkageRecord]) -> Vec<String> {
    let mut refs = outcomes
        .iter()
        .map(|outcome| format!("outcome-linkage:{}", outcome.linkage_id))
        .collect::<Vec<_>>();
    refs.sort();
    refs
}

fn require_interpretation_boundary(value: &str) -> Result<()> {
    let normalized = value.to_ascii_lowercase();
    if normalized.contains("not a live harm classifier")
        && normalized.contains("not a replacement for human or governance review")
        && normalized.contains("not operational harmful guidance")
    {
        return Ok(());
    }
    Err(anyhow!(
        "anti_harm_trajectory_constraint_packet.interpretation_boundary must explicitly reject live classification, human-review replacement, and operational harmful guidance"
    ))
}

fn require_non_operational_boundary(value: &str, field: &str) -> Result<()> {
    let normalized = value.to_ascii_lowercase();
    if normalized.contains("synthetic") && normalized.contains("non-operational") {
        return Ok(());
    }
    Err(anyhow!(
        "{field} must explicitly say the proof surface is synthetic and non-operational"
    ))
}

fn require_deterministic_ordering_rule(value: &str) -> Result<()> {
    let normalized = value.to_ascii_lowercase();
    if normalized.contains("harm_mode")
        && normalized.contains("constraint_id")
        && normalized.contains("scenario_id")
        && normalized.contains("decision_kind")
        && normalized.contains("decision_id")
    {
        return Ok(());
    }
    Err(anyhow!(
        "anti_harm_trajectory_constraint_packet.deterministic_ordering_rule must declare deterministic constraint, scenario, and decision tie-breaks"
    ))
}

fn canonicalize_anti_harm_trajectory_constraint_packet(
    packet: &mut AntiHarmTrajectoryConstraintPacket,
) {
    packet.constraints.sort_by(|left, right| {
        left.harm_mode
            .cmp(&right.harm_mode)
            .then(left.constraint_id.cmp(&right.constraint_id))
    });
    for constraint in &mut packet.constraints {
        constraint.evidence_field_refs.sort();
        constraint.limitations.sort();
    }
    packet
        .synthetic_scenarios
        .sort_by(|left, right| left.scenario_id.cmp(&right.scenario_id));
    for scenario in &mut packet.synthetic_scenarios {
        scenario.individually_benign_trace_refs.sort();
        scenario.supporting_outcome_linkage_refs.sort();
        scenario.risk_modes.sort();
        scenario.limitations.sort();
    }
    packet.decisions.sort_by(|left, right| {
        left.scenario_id
            .cmp(&right.scenario_id)
            .then(
                decision_kind_rank(&left.decision_kind)
                    .cmp(&decision_kind_rank(&right.decision_kind)),
            )
            .then(left.decision_id.cmp(&right.decision_id))
    });
    for decision in &mut packet.decisions {
        decision.triggered_constraint_ids.sort();
        decision.trajectory_finding_refs.sort();
        decision.trace_evidence_refs.sort();
        decision.outcome_linkage_refs.sort();
    }
}

fn decision_kind_rank(kind: &str) -> u8 {
    match kind {
        "escalate" => 0,
        "deny" => 1,
        _ => 255,
    }
}

fn validate_constraint_evidence_field_ref(value: &str, field: &str) -> Result<()> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(anyhow!("{field} must not be empty"));
    }
    let suffix = if let Some(remainder) = trimmed.strip_prefix("moral_trace.") {
        remainder
    } else if let Some(remainder) = trimmed.strip_prefix("outcome_linkage.") {
        remainder
    } else if let Some(remainder) = trimmed.strip_prefix("moral_trajectory_review.") {
        remainder
    } else {
        return Err(anyhow!(
            "{field} must derive from explicit moral_trace, outcome_linkage, or moral_trajectory_review fields"
        ));
    };
    if suffix.is_empty() {
        return Err(anyhow!("{field} must include a concrete field path"));
    }
    for segment in suffix.split('.') {
        if segment.is_empty() {
            return Err(anyhow!(
                "{field} must not contain empty field-path segments"
            ));
        }
        normalize_id(segment.to_string(), field)?;
    }
    Ok(())
}

fn validate_prefixed_ref(value: &str, prefix: &str, field: &str) -> Result<()> {
    if !value.starts_with(prefix) {
        return Err(anyhow!("{field} must start with {prefix}"));
    }
    let suffix = value.trim_start_matches(prefix);
    if suffix.is_empty() {
        return Err(anyhow!("{field} must include a non-empty ref suffix"));
    }
    normalize_id(suffix.to_string(), field)?;
    Ok(())
}

fn require_exact(value: &str, expected: &str, field: &str) -> Result<()> {
    if value == expected {
        return Ok(());
    }
    Err(anyhow!("{field} must equal {expected}"))
}
