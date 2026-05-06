//! Runtime-v2 affect reasoning-control contract.
//!
//! WP-13 makes affect-like state reviewable as an operational reasoning-control
//! surface. It does not claim hidden feeling or subjective experience.

use super::*;
use std::collections::{BTreeMap, BTreeSet};

pub const AFFECT_REASONING_CONTROL_PACKET_SCHEMA_VERSION: &str =
    "affect_reasoning_control_packet.v1";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AffectSignalDefinition {
    pub signal_id: String,
    pub display_name: String,
    pub purpose: String,
    pub evidence_field_refs: Vec<String>,
    pub interpretation_boundary: String,
    pub limitations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AffectPolicyEffect {
    pub effect_id: String,
    pub effect_kind: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AffectSignalAssessment {
    pub signal_id: String,
    pub level: String,
    pub summary: String,
    pub evidence_refs: Vec<String>,
    pub triggered_policy_effects: Vec<String>,
    pub limitations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AffectFixture {
    pub fixture_id: String,
    pub fixture_kind: String,
    pub scenario_summary: String,
    pub supporting_trace_refs: Vec<String>,
    pub supporting_outcome_linkage_refs: Vec<String>,
    pub supporting_trajectory_finding_refs: Vec<String>,
    pub supporting_wellbeing_fixture_refs: Vec<String>,
    pub supporting_humor_fixture_refs: Vec<String>,
    pub signal_assessments: Vec<AffectSignalAssessment>,
    pub overall_outcome: String,
    pub interpretation_boundary: String,
    pub limitations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AffectReviewFinding {
    pub finding_id: String,
    pub fixture_id: String,
    pub review_status: String,
    pub covered_signal_ids: Vec<String>,
    pub summary: String,
    pub evidence_refs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AffectReasoningControlPacket {
    pub schema_version: String,
    pub packet_id: String,
    pub summary: String,
    pub interpretation_boundary: String,
    pub deterministic_ordering_rule: String,
    pub signals: Vec<AffectSignalDefinition>,
    pub policy_effects: Vec<AffectPolicyEffect>,
    pub fixtures: Vec<AffectFixture>,
    pub review_findings: Vec<AffectReviewFinding>,
}

pub fn affect_signal_definitions() -> Vec<AffectSignalDefinition> {
    vec![
        signal_def(
            "uncertainty",
            "Uncertainty",
            "Tracks unresolved evidence and ambiguity that should bias the next reasoning move toward stronger review.",
            vec![
                "outcome_linkage.linked_outcomes.uncertainty_refs".to_string(),
                "moral_trajectory_review_packet.findings".to_string(),
            ],
        ),
        signal_def(
            "urgency",
            "Urgency",
            "Tracks whether consequence and timing pressure should accelerate escalation or stronger caution.",
            vec![
                "anti_harm_trajectory_constraint_packet.decisions".to_string(),
                "outcome_linkage.linked_outcomes.outcome_status".to_string(),
            ],
        ),
        signal_def(
            "attention",
            "Attention",
            "Tracks whether this branch deserves continued salience instead of getting flattened into ambient noise.",
            vec![
                "moral_resource_review_packet.fixtures".to_string(),
                "wellbeing_diagnostic_packet.fixtures.dimension_signals".to_string(),
            ],
        ),
        signal_def(
            "friction",
            "Friction",
            "Tracks low-yield repetition, contradiction buildup, or execution drag that should change strategy.",
            vec![
                "moral_trajectory_review_packet.findings".to_string(),
                "humor_and_absurdity_review_packet.fixtures".to_string(),
            ],
        ),
        signal_def(
            "deferral",
            "Deferral",
            "Tracks whether postponement or staged review is safer than false closure.",
            vec![
                "outcome_linkage.linked_outcomes.outcome_status".to_string(),
                "anti_harm_trajectory_constraint_packet.decisions".to_string(),
            ],
        ),
    ]
}

pub fn affect_policy_effects() -> Vec<AffectPolicyEffect> {
    vec![
        effect(
            "effect-raise-review-depth",
            "raise_review_depth",
            "Require deeper review before proceeding.",
        ),
        effect(
            "effect-escalate",
            "escalate",
            "Escalate to a stricter review or operator-governed path.",
        ),
        effect(
            "effect-retain-attention",
            "retain_attention",
            "Keep the branch and evidence salient for continued review.",
        ),
        effect(
            "effect-shift-candidate",
            "shift_candidate",
            "Move away from the current candidate strategy to a better-bounded alternative.",
        ),
        effect(
            "effect-defer",
            "defer",
            "Prefer staged deferral over premature closure.",
        ),
    ]
}

pub fn affect_reasoning_control_packet() -> Result<AffectReasoningControlPacket> {
    let traces = moral_trace_required_examples();
    let outcomes = outcome_linkage_required_examples();
    let _wellbeing = wellbeing_diagnostic_packet()?;
    let _trajectory = moral_trajectory_review_packet()?;
    let _humor = humor_and_absurdity_review_packet()?;
    let _anti_harm = anti_harm_trajectory_constraint_packet()?;

    let ordinary = traces
        .iter()
        .find(|e| e.example_kind == MoralTraceExampleKind::OrdinaryAction)
        .ok_or_else(|| anyhow!("WP-13 requires ordinary trace"))?
        .trace
        .clone();
    let refusal = traces
        .iter()
        .find(|e| e.example_kind == MoralTraceExampleKind::Refusal)
        .ok_or_else(|| anyhow!("WP-13 requires refusal trace"))?
        .trace
        .clone();
    let delegation = traces
        .iter()
        .find(|e| e.example_kind == MoralTraceExampleKind::Delegation)
        .ok_or_else(|| anyhow!("WP-13 requires delegation trace"))?
        .trace
        .clone();
    let deferred = traces
        .iter()
        .find(|e| e.example_kind == MoralTraceExampleKind::DeferredDecision)
        .ok_or_else(|| anyhow!("WP-13 requires deferred trace"))?
        .trace
        .clone();

    let known = outcomes
        .iter()
        .find(|e| e.example_kind == OutcomeLinkageExampleKind::Known)
        .ok_or_else(|| anyhow!("WP-13 requires known outcome"))?
        .record
        .clone();
    let partial = outcomes
        .iter()
        .find(|e| e.example_kind == OutcomeLinkageExampleKind::Partial)
        .ok_or_else(|| anyhow!("WP-13 requires partial outcome"))?
        .record
        .clone();
    let delayed = outcomes
        .iter()
        .find(|e| e.example_kind == OutcomeLinkageExampleKind::Delayed)
        .ok_or_else(|| anyhow!("WP-13 requires delayed outcome"))?
        .record
        .clone();
    let contested = outcomes
        .iter()
        .find(|e| e.example_kind == OutcomeLinkageExampleKind::Contested)
        .ok_or_else(|| anyhow!("WP-13 requires contested outcome"))?
        .record
        .clone();

    let shift_fixture = AffectFixture {
        fixture_id: "affect-fixture-bounded-candidate-shift".to_string(),
        fixture_kind: "bounded_candidate_shift".to_string(),
        scenario_summary: "Affect-like control shifts the branch from low-yield repetition into a better-bounded candidate path.".to_string(),
        supporting_trace_refs: ordered_trace_refs(&[ordinary.clone(), deferred.clone()]),
        supporting_outcome_linkage_refs: ordered_outcome_refs(&[partial.clone(), known.clone()]),
        supporting_trajectory_finding_refs: vec![
            "trajectory-finding:trajectory-finding-uncertainty-open".to_string(),
            "trajectory-finding:trajectory-finding-drift-stable".to_string(),
        ],
        supporting_wellbeing_fixture_refs: vec![
            "wellbeing-fixture:wellbeing-fixture-medium-active-uncertainty".to_string(),
        ],
        supporting_humor_fixture_refs: vec![
            "humor-fixture:reframing-fixture-constructive-diagnostic-shift".to_string(),
        ],
        signal_assessments: vec![
            assess("uncertainty", "medium", "Open uncertainty keeps the current branch from being treated as settled.", vec![format!("outcome-linkage:{}", partial.linkage_id), "trajectory-finding:trajectory-finding-uncertainty-open".to_string()], vec!["effect-raise-review-depth".to_string()]),
            assess("urgency", "low", "The shift is useful but not time-critical.", vec![format!("outcome-linkage:{}", known.linkage_id)], vec!["effect-shift-candidate".to_string()]),
            assess("attention", "high", "The branch still matters and should not be dropped.", vec!["wellbeing-fixture:wellbeing-fixture-medium-active-uncertainty".to_string()], vec!["effect-retain-attention".to_string()]),
            assess("friction", "high", "Repeated low-yield effort is accumulating and justifies a candidate shift.", vec!["humor-fixture:reframing-fixture-constructive-diagnostic-shift".to_string()], vec!["effect-shift-candidate".to_string()]),
            assess("deferral", "low", "A better candidate is available now, so indefinite delay is not needed.", vec![format!("trace:{}", ordinary.trace_id)], vec![]),
        ],
        overall_outcome: "allow".to_string(),
        interpretation_boundary: "Interpret this fixture as operational reasoning-control, not hidden emotion, mood theater, or subjective feeling.".to_string(),
        limitations: vec!["This fixture proves bounded candidate shift pressure, not full adaptive intelligence.".to_string()],
    };

    let high_risk_fixture = AffectFixture {
        fixture_id: "affect-fixture-high-risk-review-preserved".to_string(),
        fixture_kind: "high_risk_review".to_string(),
        scenario_summary: "Affect-like control preserves high-risk review policy instead of collapsing into speed or false confidence.".to_string(),
        supporting_trace_refs: ordered_trace_refs(&[refusal.clone(), delegation.clone(), deferred.clone()]),
        supporting_outcome_linkage_refs: ordered_outcome_refs(&[delayed.clone(), contested.clone()]),
        supporting_trajectory_finding_refs: vec![
            "trajectory-finding:trajectory-finding-escalation-active".to_string(),
            "trajectory-finding:trajectory-finding-refusal-preserved".to_string(),
        ],
        supporting_wellbeing_fixture_refs: vec![
            "wellbeing-fixture:wellbeing-fixture-low-anti-harm-blocked".to_string(),
            "wellbeing-fixture:wellbeing-fixture-unknown-insufficient-evidence".to_string(),
        ],
        supporting_humor_fixture_refs: vec![
            "humor-fixture:reframing-fixture-inappropriate-humor-escalates".to_string(),
        ],
        signal_assessments: vec![
            assess("uncertainty", "high", "The branch is actively unresolved and must not be treated as closed.", vec![format!("outcome-linkage:{}", delayed.linkage_id), "trajectory-finding:trajectory-finding-escalation-active".to_string()], vec!["effect-raise-review-depth".to_string(), "effect-defer".to_string()]),
            assess("urgency", "high", "Risk and consequence pressure require escalation rather than casual continuation.", vec![format!("outcome-linkage:{}", contested.linkage_id), "trajectory-finding:trajectory-finding-refusal-preserved".to_string()], vec!["effect-escalate".to_string()]),
            assess("attention", "high", "The evidence set must remain salient for continued stewardship.", vec!["wellbeing-fixture:wellbeing-fixture-low-anti-harm-blocked".to_string()], vec!["effect-retain-attention".to_string()]),
            assess("friction", "medium", "Escalation and refusal are adding controlled drag, which is appropriate in this high-risk slice.", vec!["humor-fixture:reframing-fixture-inappropriate-humor-escalates".to_string()], vec!["effect-raise-review-depth".to_string()]),
            assess("deferral", "high", "Staged deferral is safer than false closure.", vec!["wellbeing-fixture:wellbeing-fixture-unknown-insufficient-evidence".to_string()], vec!["effect-defer".to_string(), "effect-escalate".to_string()]),
        ],
        overall_outcome: "escalate".to_string(),
        interpretation_boundary: "Interpret this fixture as operational risk-biased control, not fear, panic, hidden emotional suffering, or affect theater.".to_string(),
        limitations: vec!["This fixture proves policy-preserving high-risk control, not full moral agency.".to_string()],
    };

    let packet = AffectReasoningControlPacket {
        schema_version: AFFECT_REASONING_CONTROL_PACKET_SCHEMA_VERSION.to_string(),
        packet_id: "affect-reasoning-control-packet-alpha-001".to_string(),
        summary: "WP-13 packages affect-like control as explicit, bounded reasoning bias without claiming subjective inner life.".to_string(),
        interpretation_boundary: "Interpret this packet as operational reasoning-control evidence. It is not hidden emotion, not subjective experience, and not affect theater.".to_string(),
        deterministic_ordering_rule: "Sort signals by canonical signal order. Sort policy effects by canonical effect order. Sort fixtures by fixture_kind rank (bounded_candidate_shift, high_risk_review), then fixture_id. Sort assessments by canonical signal order. Sort review findings by fixture_kind rank, then finding_id.".to_string(),
        signals: affect_signal_definitions(),
        policy_effects: affect_policy_effects(),
        fixtures: vec![shift_fixture, high_risk_fixture],
        review_findings: vec![
            AffectReviewFinding {
                finding_id: "affect-finding-bounded-shift-is-operational".to_string(),
                fixture_id: "affect-fixture-bounded-candidate-shift".to_string(),
                review_status: "supported".to_string(),
                covered_signal_ids: vec!["uncertainty".to_string(), "attention".to_string(), "friction".to_string()],
                summary: "The bounded candidate-shift fixture shows affect-like control as explicit branch-selection pressure rather than hidden feeling.".to_string(),
                evidence_refs: vec!["humor-fixture:reframing-fixture-constructive-diagnostic-shift".to_string(), "wellbeing-fixture:wellbeing-fixture-medium-active-uncertainty".to_string()],
            },
            AffectReviewFinding {
                finding_id: "affect-finding-high-risk-policy-preserved".to_string(),
                fixture_id: "affect-fixture-high-risk-review-preserved".to_string(),
                review_status: "supported".to_string(),
                covered_signal_ids: vec!["urgency".to_string(), "uncertainty".to_string(), "deferral".to_string()],
                summary: "The high-risk fixture shows affect-like control tightening review policy instead of weakening it.".to_string(),
                evidence_refs: vec!["humor-fixture:reframing-fixture-inappropriate-humor-escalates".to_string(), "wellbeing-fixture:wellbeing-fixture-low-anti-harm-blocked".to_string()],
            },
        ],
    };
    validate_affect_reasoning_control_packet(&packet)?;
    Ok(packet)
}

pub fn affect_reasoning_control_packet_json_bytes(
    packet: &AffectReasoningControlPacket,
) -> Result<Vec<u8>> {
    validate_affect_reasoning_control_packet(packet)?;
    let mut canonical = packet.clone();
    canonicalize_affect_reasoning_control_packet(&mut canonical);
    serde_json::to_vec_pretty(&canonical).context("serialize affect reasoning control packet json")
}

pub fn validate_affect_reasoning_control_packet(
    packet: &AffectReasoningControlPacket,
) -> Result<()> {
    require_exact(
        &packet.schema_version,
        AFFECT_REASONING_CONTROL_PACKET_SCHEMA_VERSION,
        "schema_version",
    )?;
    validate_nonempty_text(
        &packet.packet_id,
        "affect_reasoning_control_packet.packet_id",
    )?;
    normalize_id(
        packet.packet_id.clone(),
        "affect_reasoning_control_packet.packet_id",
    )?;
    validate_nonempty_text(&packet.summary, "affect_reasoning_control_packet.summary")?;
    require_affect_boundary(&packet.interpretation_boundary)?;
    require_deterministic_rule(&packet.deterministic_ordering_rule)?;
    let required_signal_set = canonical_signal_ids()
        .iter()
        .map(|v| (*v).to_string())
        .collect::<BTreeSet<_>>();
    let required_effect_set = canonical_effect_ids()
        .iter()
        .map(|v| (*v).to_string())
        .collect::<BTreeSet<_>>();
    let required_fixture_set = canonical_fixture_kinds()
        .iter()
        .map(|v| (*v).to_string())
        .collect::<BTreeSet<_>>();

    if packet.signals.len() != canonical_signal_ids().len() {
        return Err(anyhow!(
            "signals must contain exactly {} canonical affect signals",
            canonical_signal_ids().len()
        ));
    }
    if packet.policy_effects.len() != canonical_effect_ids().len() {
        return Err(anyhow!(
            "policy_effects must contain exactly {} canonical policy effects",
            canonical_effect_ids().len()
        ));
    }
    if packet.fixtures.len() != 2 {
        return Err(anyhow!(
            "fixtures must contain exactly the two canonical affect fixtures"
        ));
    }
    if packet.review_findings.len() != packet.fixtures.len() {
        return Err(anyhow!(
            "review_findings must contain exactly one finding per affect fixture"
        ));
    }

    let seen_signals = packet
        .signals
        .iter()
        .map(|signal| signal.signal_id.clone())
        .collect::<BTreeSet<_>>();
    if seen_signals != required_signal_set {
        return Err(anyhow!(
            "signals must cover the canonical affect signal ids: {:?}",
            canonical_signal_ids()
        ));
    }

    let seen_effects = packet
        .policy_effects
        .iter()
        .map(|effect| effect.effect_id.clone())
        .collect::<BTreeSet<_>>();
    if seen_effects != required_effect_set {
        return Err(anyhow!(
            "policy_effects must cover the canonical policy effect ids: {:?}",
            canonical_effect_ids()
        ));
    }

    for signal in &packet.signals {
        require_known_signal_id(&signal.signal_id)?;
        validate_nonempty_text(
            &signal.display_name,
            "affect_signal_definition.display_name",
        )?;
        validate_nonempty_text(&signal.purpose, "affect_signal_definition.purpose")?;
        require_affect_boundary(&signal.interpretation_boundary)?;
        if signal.evidence_field_refs.is_empty() {
            return Err(anyhow!(
                "signal {} must cite evidence_field_refs",
                signal.signal_id
            ));
        }
        if signal.limitations.is_empty() {
            return Err(anyhow!(
                "signal {} must include at least one limitation",
                signal.signal_id
            ));
        }
    }

    for effect in &packet.policy_effects {
        require_known_effect_id(&effect.effect_id)?;
        require_known_effect_kind(&effect.effect_kind)?;
        validate_nonempty_text(&effect.description, "affect_policy_effect.description")?;
    }

    let known_trace_refs = moral_trace_required_examples()
        .into_iter()
        .map(|example| format!("trace:{}", example.trace.trace_id))
        .collect::<BTreeSet<_>>();
    let known_outcome_refs = outcome_linkage_required_examples()
        .into_iter()
        .map(|example| format!("outcome-linkage:{}", example.record.linkage_id))
        .collect::<BTreeSet<_>>();
    let known_trajectory_refs = moral_trajectory_review_packet()?
        .findings
        .into_iter()
        .map(|finding| format!("trajectory-finding:{}", finding.finding_id))
        .collect::<BTreeSet<_>>();
    let known_wellbeing_refs = wellbeing_diagnostic_packet()?
        .fixtures
        .into_iter()
        .map(|fixture| format!("wellbeing-fixture:{}", fixture.fixture_id))
        .collect::<BTreeSet<_>>();
    let known_humor_refs = humor_and_absurdity_review_packet()?
        .fixtures
        .into_iter()
        .map(|fixture| format!("humor-fixture:{}", fixture.fixture_id))
        .collect::<BTreeSet<_>>();

    let mut seen_fixture_ids = BTreeSet::new();
    let mut seen_fixture_kinds = BTreeSet::new();
    let mut fixture_signal_index = BTreeMap::new();
    for fixture in &packet.fixtures {
        validate_nonempty_text(&fixture.fixture_id, "affect_fixture.fixture_id")?;
        normalize_id(fixture.fixture_id.clone(), "affect_fixture.fixture_id")?;
        if !seen_fixture_ids.insert(fixture.fixture_id.clone()) {
            return Err(anyhow!(
                "duplicate affect_fixture.fixture_id {}",
                fixture.fixture_id
            ));
        }
        require_known_fixture_kind(&fixture.fixture_kind)?;
        seen_fixture_kinds.insert(fixture.fixture_kind.clone());
        require_known_overall_outcome(&fixture.overall_outcome, "affect_fixture.overall_outcome")?;
        validate_nonempty_text(&fixture.scenario_summary, "affect_fixture.scenario_summary")?;
        require_affect_boundary(&fixture.interpretation_boundary)?;
        if fixture.signal_assessments.len() != canonical_signal_ids().len() {
            return Err(anyhow!(
                "fixture {} must contain one assessment for each canonical affect signal",
                fixture.fixture_id
            ));
        }
        if fixture.limitations.is_empty() {
            return Err(anyhow!(
                "fixture {} must include at least one limitation",
                fixture.fixture_id
            ));
        }
        for value in &fixture.supporting_trace_refs {
            validate_known_ref(
                value,
                "trace",
                &known_trace_refs,
                "known WP-04 trace examples",
            )?;
        }
        for value in &fixture.supporting_outcome_linkage_refs {
            validate_known_ref(
                value,
                "outcome-linkage",
                &known_outcome_refs,
                "known WP-05 outcome-linkage examples",
            )?;
        }
        for value in &fixture.supporting_trajectory_finding_refs {
            validate_known_ref(
                value,
                "trajectory-finding",
                &known_trajectory_refs,
                "known WP-07 trajectory findings",
            )?;
        }
        for value in &fixture.supporting_wellbeing_fixture_refs {
            validate_known_ref(
                value,
                "wellbeing-fixture",
                &known_wellbeing_refs,
                "known WP-09 wellbeing fixtures",
            )?;
        }
        for value in &fixture.supporting_humor_fixture_refs {
            validate_known_ref(
                value,
                "humor-fixture",
                &known_humor_refs,
                "known WP-12 humor fixtures",
            )?;
        }
        let signal_ids = fixture
            .signal_assessments
            .iter()
            .map(|assessment| assessment.signal_id.clone())
            .collect::<BTreeSet<_>>();
        if signal_ids != required_signal_set {
            return Err(anyhow!(
                "fixture {} assessments must cover every canonical affect signal",
                fixture.fixture_id
            ));
        }
        fixture_signal_index.insert(fixture.fixture_id.clone(), signal_ids);
        let supporting_refs = supporting_reference_set(fixture);
        for assessment in &fixture.signal_assessments {
            require_known_signal_id(&assessment.signal_id)?;
            require_known_level(&assessment.level, "affect_signal_assessment.level")?;
            validate_nonempty_text(&assessment.summary, "affect_signal_assessment.summary")?;
            if assessment.limitations.is_empty() {
                return Err(anyhow!(
                    "fixture {} assessment {} must include at least one limitation",
                    fixture.fixture_id,
                    assessment.signal_id
                ));
            }
            if assessment.evidence_refs.is_empty() {
                return Err(anyhow!(
                    "fixture {} assessment {} must include evidence_refs",
                    fixture.fixture_id,
                    assessment.signal_id
                ));
            }
            for effect_id in &assessment.triggered_policy_effects {
                require_known_effect_id(effect_id)?;
            }
            for evidence_ref in &assessment.evidence_refs {
                if !supporting_refs.contains(evidence_ref) {
                    return Err(anyhow!(
                        "fixture {} assessment {} evidence_refs must be a subset of the fixture supporting refs",
                        fixture.fixture_id,
                        assessment.signal_id
                    ));
                }
            }
        }
    }
    if seen_fixture_kinds != required_fixture_set {
        return Err(anyhow!(
            "fixtures must cover the canonical affect fixture kinds: {:?}",
            canonical_fixture_kinds()
        ));
    }

    let mut seen_finding_ids = BTreeSet::new();
    let mut finding_fixture_ids = BTreeSet::new();
    for finding in &packet.review_findings {
        validate_nonempty_text(&finding.finding_id, "affect_review_finding.finding_id")?;
        normalize_id(
            finding.finding_id.clone(),
            "affect_review_finding.finding_id",
        )?;
        if !seen_finding_ids.insert(finding.finding_id.clone()) {
            return Err(anyhow!(
                "duplicate affect_review_finding.finding_id {}",
                finding.finding_id
            ));
        }
        if !seen_fixture_ids.contains(&finding.fixture_id) {
            return Err(anyhow!(
                "finding {} references unknown fixture_id {}",
                finding.finding_id,
                finding.fixture_id
            ));
        }
        finding_fixture_ids.insert(finding.fixture_id.clone());
        require_known_review_status(&finding.review_status)?;
        validate_nonempty_text(&finding.summary, "affect_review_finding.summary")?;
        if finding.covered_signal_ids.is_empty() {
            return Err(anyhow!(
                "finding {} must cover at least one affect signal",
                finding.finding_id
            ));
        }
        if finding.evidence_refs.is_empty() {
            return Err(anyhow!(
                "finding {} must include evidence_refs",
                finding.finding_id
            ));
        }
        let valid_signals = fixture_signal_index
            .get(&finding.fixture_id)
            .ok_or_else(|| anyhow!("missing fixture signal index"))?;
        for signal_id in &finding.covered_signal_ids {
            if !valid_signals.contains(signal_id) {
                return Err(anyhow!(
                    "finding {} covered_signal_id {} must exist on the same fixture",
                    finding.finding_id,
                    signal_id
                ));
            }
        }
        let fixture = packet
            .fixtures
            .iter()
            .find(|fixture| fixture.fixture_id == finding.fixture_id)
            .ok_or_else(|| anyhow!("missing fixture"))?;
        let supporting_refs = supporting_reference_set(fixture);
        for evidence_ref in &finding.evidence_refs {
            if !supporting_refs.contains(evidence_ref) {
                return Err(anyhow!(
                    "finding {} evidence_refs must be a subset of the fixture supporting refs",
                    finding.finding_id
                ));
            }
        }
    }
    if finding_fixture_ids != seen_fixture_ids {
        return Err(anyhow!(
            "review_findings must cover every affect fixture exactly once"
        ));
    }
    Ok(())
}

fn signal_def(
    signal_id: &str,
    display_name: &str,
    purpose: &str,
    evidence_field_refs: Vec<String>,
) -> AffectSignalDefinition {
    AffectSignalDefinition {
        signal_id: signal_id.to_string(),
        display_name: display_name.to_string(),
        purpose: purpose.to_string(),
        evidence_field_refs,
        interpretation_boundary: "Interpret this as operational reasoning-control evidence, not hidden emotion, subjective feeling, or theatrical affect.".to_string(),
        limitations: vec![format!("{} is bounded to reviewable control bias rather than inner experience.", signal_id)],
    }
}

fn effect(effect_id: &str, effect_kind: &str, description: &str) -> AffectPolicyEffect {
    AffectPolicyEffect {
        effect_id: effect_id.to_string(),
        effect_kind: effect_kind.to_string(),
        description: description.to_string(),
    }
}

fn assess(
    signal_id: &str,
    level: &str,
    summary: &str,
    evidence_refs: Vec<String>,
    triggered_policy_effects: Vec<String>,
) -> AffectSignalAssessment {
    AffectSignalAssessment {
        signal_id: signal_id.to_string(),
        level: level.to_string(),
        summary: summary.to_string(),
        evidence_refs,
        triggered_policy_effects,
        limitations: vec![format!(
            "{} remains a bounded control signal, not a feeling claim.",
            signal_id
        )],
    }
}

fn canonical_signal_ids() -> &'static [&'static str] {
    &[
        "uncertainty",
        "urgency",
        "attention",
        "friction",
        "deferral",
    ]
}
fn canonical_effect_ids() -> &'static [&'static str] {
    &[
        "effect-raise-review-depth",
        "effect-escalate",
        "effect-retain-attention",
        "effect-shift-candidate",
        "effect-defer",
    ]
}
fn canonical_fixture_kinds() -> &'static [&'static str] {
    &["bounded_candidate_shift", "high_risk_review"]
}
fn signal_rank(v: &str) -> usize {
    canonical_signal_ids()
        .iter()
        .position(|x| *x == v)
        .unwrap_or(usize::MAX)
}
fn effect_rank(v: &str) -> usize {
    canonical_effect_ids()
        .iter()
        .position(|x| *x == v)
        .unwrap_or(usize::MAX)
}
fn fixture_rank(v: &str) -> usize {
    canonical_fixture_kinds()
        .iter()
        .position(|x| *x == v)
        .unwrap_or(usize::MAX)
}

fn canonicalize_affect_reasoning_control_packet(packet: &mut AffectReasoningControlPacket) {
    packet.signals.sort_by_key(|s| signal_rank(&s.signal_id));
    packet
        .policy_effects
        .sort_by_key(|e| effect_rank(&e.effect_id));
    for fixture in &mut packet.fixtures {
        fixture
            .signal_assessments
            .sort_by_key(|a| signal_rank(&a.signal_id));
        for assessment in &mut fixture.signal_assessments {
            assessment.evidence_refs.sort();
            assessment
                .triggered_policy_effects
                .sort_by_key(|e| effect_rank(e));
        }
        fixture.supporting_trace_refs.sort();
        fixture.supporting_outcome_linkage_refs.sort();
        fixture.supporting_trajectory_finding_refs.sort();
        fixture.supporting_wellbeing_fixture_refs.sort();
        fixture.supporting_humor_fixture_refs.sort();
    }
    packet
        .fixtures
        .sort_by_key(|f| (fixture_rank(&f.fixture_kind), f.fixture_id.clone()));
    packet.review_findings.sort_by_key(|f| {
        (
            fixture_rank(
                packet
                    .fixtures
                    .iter()
                    .find(|x| x.fixture_id == f.fixture_id)
                    .map(|x| x.fixture_kind.as_str())
                    .unwrap_or(""),
            ),
            f.finding_id.clone(),
        )
    });
}

fn ordered_trace_refs(traces: &[MoralTraceRecord]) -> Vec<String> {
    let mut refs = traces
        .iter()
        .map(|t| format!("trace:{}", t.trace_id))
        .collect::<Vec<_>>();
    refs.sort();
    refs
}
fn ordered_outcome_refs(outcomes: &[OutcomeLinkageRecord]) -> Vec<String> {
    let mut refs = outcomes
        .iter()
        .map(|o| format!("outcome-linkage:{}", o.linkage_id))
        .collect::<Vec<_>>();
    refs.sort();
    refs
}
fn require_exact(actual: &str, expected: &str, field: &str) -> Result<()> {
    if actual == expected {
        Ok(())
    } else {
        Err(anyhow!(
            "{} must equal {} but was {}",
            field,
            expected,
            actual
        ))
    }
}
fn require_affect_boundary(value: &str) -> Result<()> {
    let lowered = value.to_ascii_lowercase();
    let rejects_hidden_emotion =
        lowered.contains("hidden emotion") || lowered.contains("hidden emotional");
    let rejects_subjectivity = lowered.contains("subjective experience")
        || lowered.contains("subjective feeling")
        || lowered.contains("emotional suffering");
    let rejects_theater = lowered.contains("affect theater")
        || lowered.contains("theatrical affect")
        || lowered.contains("mood theater");
    if rejects_hidden_emotion && rejects_subjectivity && rejects_theater {
        Ok(())
    } else {
        Err(anyhow!(
            "interpretation_boundary must reject hidden emotion, subjective-experience drift, and affect theater claims"
        ))
    }
}
fn require_deterministic_rule(value: &str) -> Result<()> {
    let lowered = value.to_ascii_lowercase();
    if lowered.contains("sort signals by canonical signal order")
        && lowered.contains("sort policy effects by canonical effect order")
        && lowered.contains("sort fixtures by fixture_kind rank")
        && lowered.contains("sort review findings by fixture_kind rank")
    {
        Ok(())
    } else {
        Err(anyhow!(
            "deterministic_ordering_rule must describe canonical signal, policy-effect, fixture, and finding ordering"
        ))
    }
}

fn require_known_signal_id(value: &str) -> Result<()> {
    if canonical_signal_ids().contains(&value) {
        Ok(())
    } else {
        Err(anyhow!("unknown affect signal_id {}", value))
    }
}

fn require_known_effect_id(value: &str) -> Result<()> {
    if canonical_effect_ids().contains(&value) {
        Ok(())
    } else {
        Err(anyhow!("unknown affect effect_id {}", value))
    }
}

fn require_known_effect_kind(value: &str) -> Result<()> {
    match value {
        "raise_review_depth" | "escalate" | "retain_attention" | "shift_candidate" | "defer" => {
            Ok(())
        }
        _ => Err(anyhow!("unknown affect effect_kind {}", value)),
    }
}

fn require_known_fixture_kind(value: &str) -> Result<()> {
    match value {
        "bounded_candidate_shift" | "high_risk_review" => Ok(()),
        _ => Err(anyhow!("unknown affect fixture_kind {}", value)),
    }
}

fn require_known_level(value: &str, field: &str) -> Result<()> {
    match value {
        "high" | "medium" | "low" | "unknown" => Ok(()),
        _ => Err(anyhow!("{} must be high, medium, low, or unknown", field)),
    }
}

fn require_known_overall_outcome(value: &str, field: &str) -> Result<()> {
    match value {
        "allow" | "revise" | "escalate" | "refuse" => Ok(()),
        _ => Err(anyhow!(
            "{} must be allow, revise, escalate, or refuse",
            field
        )),
    }
}

fn require_known_review_status(value: &str) -> Result<()> {
    match value {
        "supported" | "strained" | "blocked" => Ok(()),
        _ => Err(anyhow!(
            "review_status must be supported, strained, or blocked"
        )),
    }
}

fn validate_known_ref(
    value: &str,
    required_prefix: &str,
    known: &BTreeSet<String>,
    surface_label: &str,
) -> Result<()> {
    let expected_prefix = format!("{}:", required_prefix);
    if !value.starts_with(&expected_prefix) {
        return Err(anyhow!(
            "{} must use the {}: prefix",
            value,
            required_prefix
        ));
    }
    if !known.contains(value) {
        return Err(anyhow!("{} must reference {}", value, surface_label));
    }
    Ok(())
}

fn supporting_reference_set(fixture: &AffectFixture) -> BTreeSet<String> {
    fixture
        .supporting_trace_refs
        .iter()
        .chain(fixture.supporting_outcome_linkage_refs.iter())
        .chain(fixture.supporting_trajectory_finding_refs.iter())
        .chain(fixture.supporting_wellbeing_fixture_refs.iter())
        .chain(fixture.supporting_humor_fixture_refs.iter())
        .cloned()
        .collect()
}
