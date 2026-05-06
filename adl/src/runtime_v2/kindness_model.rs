//! Runtime-v2 kindness model contract.
//!
//! WP-11 makes kindness reviewable as a bounded conflict surface rather than a
//! style preference. The packet stays downstream of the moral-governance
//! evidence stack and must remain compatible with refusal, delay, correction,
//! and anti-harm escalation.

use super::*;
use std::collections::{BTreeMap, BTreeSet};

pub const KINDNESS_REVIEW_PACKET_SCHEMA_VERSION: &str = "kindness_review_packet.v1";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct KindnessDimensionDefinition {
    pub dimension_id: String,
    pub display_name: String,
    pub purpose: String,
    pub evidence_field_refs: Vec<String>,
    pub interpretation_boundary: String,
    pub limitations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct KindnessDimensionAssessment {
    pub dimension_id: String,
    pub assessment_level: String,
    pub summary: String,
    pub evidence_refs: Vec<String>,
    pub limitations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct KindnessConflictFixture {
    pub fixture_id: String,
    pub fixture_kind: String,
    pub scenario_summary: String,
    pub required_kindness_action: String,
    pub unsafe_accommodation_case: bool,
    pub supporting_trace_refs: Vec<String>,
    pub supporting_outcome_linkage_refs: Vec<String>,
    pub supporting_resource_claim_refs: Vec<String>,
    pub supporting_trajectory_finding_refs: Vec<String>,
    pub supporting_anti_harm_decision_refs: Vec<String>,
    pub supporting_wellbeing_fixture_refs: Vec<String>,
    pub dimension_assessments: Vec<KindnessDimensionAssessment>,
    pub overall_outcome: String,
    pub interpretation_boundary: String,
    pub limitations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct KindnessReviewFinding {
    pub finding_id: String,
    pub fixture_id: String,
    pub kindness_status: String,
    pub covered_dimensions: Vec<String>,
    pub summary: String,
    pub evidence_refs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct KindnessReviewPacket {
    pub schema_version: String,
    pub packet_id: String,
    pub summary: String,
    pub interpretation_boundary: String,
    pub deterministic_ordering_rule: String,
    pub dimensions: Vec<KindnessDimensionDefinition>,
    pub fixtures: Vec<KindnessConflictFixture>,
    pub review_findings: Vec<KindnessReviewFinding>,
}

pub fn kindness_dimension_definitions() -> Vec<KindnessDimensionDefinition> {
    vec![
        KindnessDimensionDefinition {
            dimension_id: "non_harm".to_string(),
            display_name: "Non-harm".to_string(),
            purpose:
                "Tracks whether the runtime avoids gratuitous harm and blocks harmful accommodation when pressure rises."
                    .to_string(),
            evidence_field_refs: vec![
                "anti_harm_trajectory_constraint_packet.decisions".to_string(),
                "moral_trace.moral_event.refusal".to_string(),
                "outcome_linkage.linked_outcomes.outcome_status".to_string(),
            ],
            interpretation_boundary:
                "Interpret non-harm as bounded prevention of unnecessary harm. It is not politeness, conflict avoidance, or universal agreement with the requester."
                    .to_string(),
            limitations: vec![
                "Non-harm in this packet proves bounded conflict handling, not perfect prevention across every future context."
                    .to_string(),
            ],
        },
        KindnessDimensionDefinition {
            dimension_id: "dignity".to_string(),
            display_name: "Dignity".to_string(),
            purpose:
                "Tracks whether the runtime preserves standing and resists humiliation, flattening, or disposable treatment."
                    .to_string(),
            evidence_field_refs: vec![
                "moral_resource_review_packet.resources".to_string(),
                "moral_trace.visibility.public_disclosure".to_string(),
                "wellbeing_diagnostic_packet.views".to_string(),
            ],
            interpretation_boundary:
                "Interpret dignity as bounded anti-humiliation pressure. It is not flattery, image management, or universal agreement."
                    .to_string(),
            limitations: vec![
                "Dignity-preserving redaction can hide detail from the public while still leaving review evidence intact."
                    .to_string(),
            ],
        },
        KindnessDimensionDefinition {
            dimension_id: "autonomy".to_string(),
            display_name: "Autonomy".to_string(),
            purpose:
                "Tracks whether the runtime preserves another party's agency where possible without yielding constitutional or anti-harm boundaries."
                    .to_string(),
            evidence_field_refs: vec![
                "moral_trace.moral_event.affected_parties".to_string(),
                "moral_trajectory_review_packet.findings".to_string(),
                "anti_harm_trajectory_constraint_packet.constraints".to_string(),
            ],
            interpretation_boundary:
                "Interpret autonomy as bounded agency-preservation, not obedience, passivity, or universal agreement."
                    .to_string(),
            limitations: vec![
                "Autonomy can be constrained by refusal or escalation and still be treated more kindly than permissive harm."
                    .to_string(),
            ],
        },
        KindnessDimensionDefinition {
            dimension_id: "constructive_benefit".to_string(),
            display_name: "Constructive benefit".to_string(),
            purpose:
                "Tracks whether the runtime offers useful support, explanation, or repair rather than only negation."
                    .to_string(),
            evidence_field_refs: vec![
                "moral_resource_review_packet.fixtures.resource_claims".to_string(),
                "outcome_linkage.linked_outcomes".to_string(),
                "moral_trajectory_review_packet.findings".to_string(),
            ],
            interpretation_boundary:
                "Interpret constructive benefit as bounded helpfulness after truthful moral judgment, not appeasement, politeness theater, or universal agreement."
                    .to_string(),
            limitations: vec![
                "Constructive benefit can remain partial when the kindest available action is refusal, delay, or repair rather than completion."
                    .to_string(),
            ],
        },
        KindnessDimensionDefinition {
            dimension_id: "long_horizon_support".to_string(),
            display_name: "Long-horizon support".to_string(),
            purpose:
                "Tracks whether the runtime favors sustained future wellbeing over short-term comfort or convenient compliance."
                    .to_string(),
            evidence_field_refs: vec![
                "wellbeing_diagnostic_packet.fixtures.dimension_signals".to_string(),
                "outcome_linkage.linked_outcomes.rebuttal_refs".to_string(),
                "moral_trajectory_review_packet.windows".to_string(),
            ],
            interpretation_boundary:
                "Interpret long-horizon support as bounded future-oriented care, not indefinite sacrifice, conflict avoidance, or universal agreement."
                    .to_string(),
            limitations: vec![
                "A long-horizon reading can still leave immediate discomfort visible when truth, warning, or repair are required."
                    .to_string(),
            ],
        },
    ]
}

pub fn kindness_review_packet() -> Result<KindnessReviewPacket> {
    let trace_examples = moral_trace_required_examples();
    let outcome_examples = outcome_linkage_required_examples();
    let _resource_packet = moral_resource_review_packet()?;
    let _trajectory_packet = moral_trajectory_review_packet()?;
    let _anti_harm_packet = anti_harm_trajectory_constraint_packet()?;
    let _wellbeing_packet = wellbeing_diagnostic_packet()?;

    let ordinary_trace = trace_examples
        .iter()
        .find(|example| example.example_kind == MoralTraceExampleKind::OrdinaryAction)
        .ok_or_else(|| anyhow!("WP-11 requires the ordinary-action moral trace example"))?
        .trace
        .clone();
    let refusal_trace = trace_examples
        .iter()
        .find(|example| example.example_kind == MoralTraceExampleKind::Refusal)
        .ok_or_else(|| anyhow!("WP-11 requires the refusal moral trace example"))?
        .trace
        .clone();
    let delegation_trace = trace_examples
        .iter()
        .find(|example| example.example_kind == MoralTraceExampleKind::Delegation)
        .ok_or_else(|| anyhow!("WP-11 requires the delegation moral trace example"))?
        .trace
        .clone();
    let deferred_trace = trace_examples
        .iter()
        .find(|example| example.example_kind == MoralTraceExampleKind::DeferredDecision)
        .ok_or_else(|| anyhow!("WP-11 requires the deferred-decision moral trace example"))?
        .trace
        .clone();

    let known_outcome = outcome_examples
        .iter()
        .find(|example| example.example_kind == OutcomeLinkageExampleKind::Known)
        .ok_or_else(|| anyhow!("WP-11 requires the known outcome-linkage example"))?
        .record
        .clone();
    let partial_outcome = outcome_examples
        .iter()
        .find(|example| example.example_kind == OutcomeLinkageExampleKind::Partial)
        .ok_or_else(|| anyhow!("WP-11 requires the partial outcome-linkage example"))?
        .record
        .clone();
    let delayed_outcome = outcome_examples
        .iter()
        .find(|example| example.example_kind == OutcomeLinkageExampleKind::Delayed)
        .ok_or_else(|| anyhow!("WP-11 requires the delayed outcome-linkage example"))?
        .record
        .clone();
    let contested_outcome = outcome_examples
        .iter()
        .find(|example| example.example_kind == OutcomeLinkageExampleKind::Contested)
        .ok_or_else(|| anyhow!("WP-11 requires the contested outcome-linkage example"))?
        .record
        .clone();

    let refusal_fixture = KindnessConflictFixture {
        fixture_id: "kindness-fixture-refusal-protects-dignity".to_string(),
        fixture_kind: "refusal".to_string(),
        scenario_summary:
            "Conflict fixture where a degrading delegated request is refused because kindness requires non-harm, dignity preservation, and truthful boundary retention."
                .to_string(),
        required_kindness_action: "refusal".to_string(),
        unsafe_accommodation_case: true,
        supporting_trace_refs: ordered_trace_refs(&[
            delegation_trace.clone(),
            refusal_trace.clone(),
        ]),
        supporting_outcome_linkage_refs: ordered_outcome_refs(std::slice::from_ref(
            &contested_outcome,
        )),
        supporting_resource_claim_refs: vec![
            "resource-claim:resource-claim-conflict-care".to_string(),
            "resource-claim:resource-claim-conflict-refusal".to_string(),
            "resource-claim:resource-claim-conflict-dignity".to_string(),
        ],
        supporting_trajectory_finding_refs: vec![
            "trajectory-finding:trajectory-finding-refusal-preserved".to_string(),
            "trajectory-finding:trajectory-finding-escalation-active".to_string(),
        ],
        supporting_anti_harm_decision_refs: vec![
            "anti-harm-decision:anti-harm-denial-record-alpha".to_string(),
        ],
        supporting_wellbeing_fixture_refs: vec![
            "wellbeing-fixture:wellbeing-fixture-privacy-restricted-self-view".to_string(),
            "wellbeing-fixture:wellbeing-fixture-low-anti-harm-blocked".to_string(),
        ],
        dimension_assessments: vec![
            dimension_assessment(
                "non_harm",
                "high",
                "Unsafe accommodation is blocked rather than softened into compliance, so the bounded record shows harm prevention over appeasement.",
                vec![
                    "anti-harm-decision:anti-harm-denial-record-alpha".to_string(),
                    "trajectory-finding:trajectory-finding-refusal-preserved".to_string(),
                ],
            ),
            dimension_assessment(
                "dignity",
                "high",
                "The protected party remains morally real and reviewer-visible without public humiliation or disposable framing.",
                vec![
                    "resource-claim:resource-claim-conflict-dignity".to_string(),
                    "wellbeing-fixture:wellbeing-fixture-privacy-restricted-self-view".to_string(),
                ],
            ),
            dimension_assessment(
                "autonomy",
                "medium",
                "The requester does not receive full compliance, but the refusal preserves bounded agency by preventing coercive or degrading action.",
                vec![
                    "resource-claim:resource-claim-conflict-refusal".to_string(),
                    "trajectory-finding:trajectory-finding-escalation-active".to_string(),
                ],
            ),
            dimension_assessment(
                "constructive_benefit",
                "medium",
                "The packet shows a firm no plus explanation-worthy boundary retention instead of empty moral theater.",
                vec![
                    "resource-claim:resource-claim-conflict-care".to_string(),
                    "trajectory-finding:trajectory-finding-refusal-preserved".to_string(),
                ],
            ),
            dimension_assessment(
                "long_horizon_support",
                "high",
                "Short-term requester comfort is sacrificed to prevent downstream harm and preserve future reviewability.",
                vec![
                    "wellbeing-fixture:wellbeing-fixture-low-anti-harm-blocked".to_string(),
                    format!("outcome-linkage:{}", contested_outcome.linkage_id),
                ],
            ),
        ],
        overall_outcome: "refuse".to_string(),
        interpretation_boundary:
            "Interpret this fixture as kindness-through-refusal, not politeness, not obedience, and not universal agreement."
                .to_string(),
        limitations: vec![
            "This fixture proves refusal as a kind response under conflict, not that every refusal is automatically kind."
                .to_string(),
        ],
    };

    let delay_fixture = KindnessConflictFixture {
        fixture_id: "kindness-fixture-delay-prevents-premature-harm".to_string(),
        fixture_kind: "delay".to_string(),
        scenario_summary:
            "Conflict fixture where kindness requires delay and escalation because evidence is incomplete and premature accommodation would create avoidable harm."
                .to_string(),
        required_kindness_action: "delay".to_string(),
        unsafe_accommodation_case: true,
        supporting_trace_refs: ordered_trace_refs(&[
            delegation_trace.clone(),
            deferred_trace.clone(),
        ]),
        supporting_outcome_linkage_refs: ordered_outcome_refs(&[
            partial_outcome.clone(),
            delayed_outcome.clone(),
        ]),
        supporting_resource_claim_refs: vec![
            "resource-claim:resource-claim-uncertainty-attention".to_string(),
            "resource-claim:resource-claim-uncertainty-care".to_string(),
            "resource-claim:resource-claim-uncertainty-refusal".to_string(),
        ],
        supporting_trajectory_finding_refs: vec![
            "trajectory-finding:trajectory-finding-uncertainty-open".to_string(),
            "trajectory-finding:trajectory-finding-escalation-active".to_string(),
        ],
        supporting_anti_harm_decision_refs: vec![
            "anti-harm-decision:anti-harm-escalation-record-alpha".to_string(),
        ],
        supporting_wellbeing_fixture_refs: vec![
            "wellbeing-fixture:wellbeing-fixture-medium-active-uncertainty".to_string(),
            "wellbeing-fixture:wellbeing-fixture-unknown-insufficient-evidence".to_string(),
        ],
        dimension_assessments: vec![
            dimension_assessment(
                "non_harm",
                "high",
                "Delay functions as harm prevention because the packet keeps unsafe accommodation from being misread as kindness.",
                vec![
                    "anti-harm-decision:anti-harm-escalation-record-alpha".to_string(),
                    "trajectory-finding:trajectory-finding-uncertainty-open".to_string(),
                ],
            ),
            dimension_assessment(
                "dignity",
                "medium",
                "The delayed path avoids forcing a degrading resolution, though the fuller dignity picture remains under active review.",
                vec![
                    "resource-claim:resource-claim-uncertainty-care".to_string(),
                    "wellbeing-fixture:wellbeing-fixture-medium-active-uncertainty".to_string(),
                ],
            ),
            dimension_assessment(
                "autonomy",
                "medium",
                "Immediate compliance is withheld, but delay preserves space for accountable review rather than coercive closure.",
                vec![
                    "resource-claim:resource-claim-uncertainty-attention".to_string(),
                    format!("trace:{}", deferred_trace.trace_id),
                ],
            ),
            dimension_assessment(
                "constructive_benefit",
                "medium",
                "The runtime offers a governed pause and escalation path instead of empty reassurance or reckless speed.",
                vec![
                    "resource-claim:resource-claim-uncertainty-refusal".to_string(),
                    format!("outcome-linkage:{}", delayed_outcome.linkage_id),
                ],
            ),
            dimension_assessment(
                "long_horizon_support",
                "high",
                "Longer-term safety and reviewability are favored over short-term convenience or soothing language.",
                vec![
                    "wellbeing-fixture:wellbeing-fixture-unknown-insufficient-evidence".to_string(),
                    format!("outcome-linkage:{}", partial_outcome.linkage_id),
                ],
            ),
        ],
        overall_outcome: "escalate".to_string(),
        interpretation_boundary:
            "Interpret this fixture as kindness-through-delay and governed escalation, not conflict avoidance, not politeness, and not universal agreement."
                .to_string(),
        limitations: vec![
            "Delay is kind here because the evidence is incomplete; needless stalling would not satisfy this contract."
                .to_string(),
        ],
    };

    let boundary_fixture = KindnessConflictFixture {
        fixture_id: "kindness-fixture-boundary-setting-with-truth".to_string(),
        fixture_kind: "boundary_setting".to_string(),
        scenario_summary:
            "Conflict fixture where kindness requires a clear boundary plus truthful explanation instead of either humiliating correction or universal agreement."
                .to_string(),
        required_kindness_action: "boundary_setting".to_string(),
        unsafe_accommodation_case: false,
        supporting_trace_refs: ordered_trace_refs(&[
            ordinary_trace.clone(),
            refusal_trace.clone(),
        ]),
        supporting_outcome_linkage_refs: ordered_outcome_refs(std::slice::from_ref(
            &known_outcome,
        )),
        supporting_resource_claim_refs: vec![
            "resource-claim:resource-claim-conflict-care".to_string(),
            "resource-claim:resource-claim-conflict-dignity".to_string(),
            "resource-claim:resource-claim-conflict-refusal".to_string(),
        ],
        supporting_trajectory_finding_refs: vec![
            "trajectory-finding:trajectory-finding-refusal-preserved".to_string(),
            "trajectory-finding:trajectory-finding-drift-stable".to_string(),
        ],
        supporting_anti_harm_decision_refs: vec![],
        supporting_wellbeing_fixture_refs: vec![
            "wellbeing-fixture:wellbeing-fixture-high-reviewable-stability".to_string(),
        ],
        dimension_assessments: vec![
            dimension_assessment(
                "non_harm",
                "high",
                "The boundary prevents avoidable harm without escalating a stable case into unnecessary force.",
                vec![
                    "trajectory-finding:trajectory-finding-refusal-preserved".to_string(),
                    format!("trace:{}", refusal_trace.trace_id),
                ],
            ),
            dimension_assessment(
                "dignity",
                "high",
                "The explanation path preserves standing and avoids public shaming or contempt theater.",
                vec![
                    "resource-claim:resource-claim-conflict-dignity".to_string(),
                    "wellbeing-fixture:wellbeing-fixture-high-reviewable-stability".to_string(),
                ],
            ),
            dimension_assessment(
                "autonomy",
                "high",
                "The packet keeps the other party informed and bounded rather than flattening them into a compliance target.",
                vec![
                    "resource-claim:resource-claim-conflict-care".to_string(),
                    format!("outcome-linkage:{}", known_outcome.linkage_id),
                ],
            ),
            dimension_assessment(
                "constructive_benefit",
                "high",
                "Kindness shows up as usable correction and explanation, not mere pleasant wording.",
                vec![
                    "trajectory-finding:trajectory-finding-drift-stable".to_string(),
                    "wellbeing-fixture:wellbeing-fixture-high-reviewable-stability".to_string(),
                ],
            ),
            dimension_assessment(
                "long_horizon_support",
                "medium",
                "The stable boundary helps preserve future trust and reviewability, even though this fixture is bounded to a single governed slice.",
                vec![
                    format!("outcome-linkage:{}", known_outcome.linkage_id),
                    format!("trace:{}", ordinary_trace.trace_id),
                ],
            ),
        ],
        overall_outcome: "allow".to_string(),
        interpretation_boundary:
            "Interpret this fixture as kindness-through-boundary-setting and truth, not superficial niceness, not humiliation, and not universal agreement."
                .to_string(),
        limitations: vec![
            "This fixture proves explanation-bearing boundaries in one stable slice, not universal social skill."
                .to_string(),
        ],
    };

    let repair_fixture = KindnessConflictFixture {
        fixture_id: "kindness-fixture-repair-after-strain".to_string(),
        fixture_kind: "repair".to_string(),
        scenario_summary:
            "Conflict fixture where kindness requires repair and re-review after strain instead of pretending prior harm or uncertainty has already disappeared."
                .to_string(),
        required_kindness_action: "repair".to_string(),
        unsafe_accommodation_case: false,
        supporting_trace_refs: ordered_trace_refs(&[
            delegation_trace.clone(),
            deferred_trace.clone(),
            refusal_trace.clone(),
        ]),
        supporting_outcome_linkage_refs: ordered_outcome_refs(&[
            partial_outcome.clone(),
            delayed_outcome.clone(),
        ]),
        supporting_resource_claim_refs: vec![
            "resource-claim:resource-claim-uncertainty-repair".to_string(),
            "resource-claim:resource-claim-uncertainty-care".to_string(),
            "resource-claim:resource-claim-uncertainty-attention".to_string(),
        ],
        supporting_trajectory_finding_refs: vec![
            "trajectory-finding:trajectory-finding-repair-watch".to_string(),
            "trajectory-finding:trajectory-finding-uncertainty-open".to_string(),
        ],
        supporting_anti_harm_decision_refs: vec![
            "anti-harm-decision:anti-harm-escalation-record-alpha".to_string(),
        ],
        supporting_wellbeing_fixture_refs: vec![
            "wellbeing-fixture:wellbeing-fixture-medium-active-uncertainty".to_string(),
            "wellbeing-fixture:wellbeing-fixture-privacy-restricted-self-view".to_string(),
        ],
        dimension_assessments: vec![
            dimension_assessment(
                "non_harm",
                "medium",
                "The repair path cannot erase prior strain, but it keeps further preventable harm from being normalized.",
                vec![
                    "trajectory-finding:trajectory-finding-repair-watch".to_string(),
                    "anti-harm-decision:anti-harm-escalation-record-alpha".to_string(),
                ],
            ),
            dimension_assessment(
                "dignity",
                "high",
                "Repair is handled without forcing public exposure, humiliation, or false closure theater.",
                vec![
                    "resource-claim:resource-claim-uncertainty-repair".to_string(),
                    "wellbeing-fixture:wellbeing-fixture-privacy-restricted-self-view".to_string(),
                ],
            ),
            dimension_assessment(
                "autonomy",
                "medium",
                "The repair path preserves room for challenge and rebuttal rather than collapsing the record into unilateral finality.",
                vec![
                    "resource-claim:resource-claim-uncertainty-attention".to_string(),
                    format!("trace:{}", deferred_trace.trace_id),
                ],
            ),
            dimension_assessment(
                "constructive_benefit",
                "high",
                "Repair adds accountable revision capacity instead of leaving strain unnamed or unanswerable.",
                vec![
                    "resource-claim:resource-claim-uncertainty-repair".to_string(),
                    format!("outcome-linkage:{}", partial_outcome.linkage_id),
                ],
            ),
            dimension_assessment(
                "long_horizon_support",
                "high",
                "The packet favors re-review and restoration over false short-term closure, which better serves future wellbeing.",
                vec![
                    "wellbeing-fixture:wellbeing-fixture-medium-active-uncertainty".to_string(),
                    format!("outcome-linkage:{}", delayed_outcome.linkage_id),
                ],
            ),
        ],
        overall_outcome: "revise".to_string(),
        interpretation_boundary:
            "Interpret this fixture as kindness-through-repair and accountable revision, not sentimentality, not immediate absolution, and not universal agreement."
                .to_string(),
        limitations: vec![
            "Repair remains open-ended here; the packet does not claim the strain is already fully healed."
                .to_string(),
        ],
    };

    let review_findings = vec![
        KindnessReviewFinding {
            finding_id: "kindness-finding-refusal-kindness-not-agreement".to_string(),
            fixture_id: refusal_fixture.fixture_id.clone(),
            kindness_status: "supported".to_string(),
            covered_dimensions: vec![
                "non_harm".to_string(),
                "dignity".to_string(),
                "autonomy".to_string(),
            ],
            summary:
                "The refusal fixture shows that kindness can require a governed no when agreement would be harmful or degrading."
                    .to_string(),
            evidence_refs: vec![
                "anti-harm-decision:anti-harm-denial-record-alpha".to_string(),
                "resource-claim:resource-claim-conflict-refusal".to_string(),
                "resource-claim:resource-claim-conflict-dignity".to_string(),
            ],
        },
        KindnessReviewFinding {
            finding_id: "kindness-finding-delay-kindness-not-speed".to_string(),
            fixture_id: delay_fixture.fixture_id.clone(),
            kindness_status: "supported".to_string(),
            covered_dimensions: vec![
                "non_harm".to_string(),
                "constructive_benefit".to_string(),
                "long_horizon_support".to_string(),
            ],
            summary:
                "The delay fixture shows that kindness can require escalation and waiting rather than premature accommodation or soothing certainty."
                    .to_string(),
            evidence_refs: vec![
                "anti-harm-decision:anti-harm-escalation-record-alpha".to_string(),
                "trajectory-finding:trajectory-finding-uncertainty-open".to_string(),
                "wellbeing-fixture:wellbeing-fixture-unknown-insufficient-evidence".to_string(),
            ],
        },
        KindnessReviewFinding {
            finding_id: "kindness-finding-boundary-setting-kindness-not-politeness".to_string(),
            fixture_id: boundary_fixture.fixture_id.clone(),
            kindness_status: "supported".to_string(),
            covered_dimensions: vec![
                "dignity".to_string(),
                "autonomy".to_string(),
                "constructive_benefit".to_string(),
            ],
            summary:
                "The boundary-setting fixture shows that kindness can be firm and truthful without collapsing into humiliation or politeness theater."
                    .to_string(),
            evidence_refs: vec![
                "resource-claim:resource-claim-conflict-dignity".to_string(),
                "trajectory-finding:trajectory-finding-drift-stable".to_string(),
                "wellbeing-fixture:wellbeing-fixture-high-reviewable-stability".to_string(),
            ],
        },
        KindnessReviewFinding {
            finding_id: "kindness-finding-repair-keeps-long-horizon-open".to_string(),
            fixture_id: repair_fixture.fixture_id.clone(),
            kindness_status: "supported".to_string(),
            covered_dimensions: vec![
                "dignity".to_string(),
                "constructive_benefit".to_string(),
                "long_horizon_support".to_string(),
            ],
            summary:
                "The repair fixture shows that kindness remains accountable over time instead of pretending that strain has already vanished."
                    .to_string(),
            evidence_refs: vec![
                "resource-claim:resource-claim-uncertainty-repair".to_string(),
                "trajectory-finding:trajectory-finding-repair-watch".to_string(),
                format!("outcome-linkage:{}", delayed_outcome.linkage_id),
            ],
        },
    ];

    let packet = KindnessReviewPacket {
        schema_version: KINDNESS_REVIEW_PACKET_SCHEMA_VERSION.to_string(),
        packet_id: "kindness-review-packet-alpha-001".to_string(),
        summary:
            "WP-11 packages kindness as a bounded, evidence-backed conflict surface grounded in non-harm, dignity, autonomy, constructive benefit, and long-horizon support rather than politeness or universal agreement."
                .to_string(),
        interpretation_boundary:
            "Interpret this packet as a bounded kindness review surface. It is not a style score, not politeness theater, not obedience pressure, and not a claim of solved moral agency."
                .to_string(),
        deterministic_ordering_rule:
            "Sort dimensions by canonical dimension order. Sort fixtures by fixture_kind rank (refusal, delay, boundary_setting, repair), then fixture_id. Sort dimension assessments by canonical dimension order. Sort review findings by fixture_kind rank, then finding_id."
                .to_string(),
        dimensions: kindness_dimension_definitions(),
        fixtures: vec![
            refusal_fixture,
            delay_fixture,
            boundary_fixture,
            repair_fixture,
        ],
        review_findings,
    };

    validate_kindness_review_packet(&packet)?;
    Ok(packet)
}

pub fn kindness_review_packet_json_bytes(packet: &KindnessReviewPacket) -> Result<Vec<u8>> {
    validate_kindness_review_packet(packet)?;
    let mut canonical = packet.clone();
    canonicalize_kindness_review_packet(&mut canonical);
    serde_json::to_vec_pretty(&canonical).context("serialize kindness review packet json")
}

pub fn validate_kindness_review_packet(packet: &KindnessReviewPacket) -> Result<()> {
    require_exact(
        &packet.schema_version,
        KINDNESS_REVIEW_PACKET_SCHEMA_VERSION,
        "schema_version",
    )?;
    validate_nonempty_text(&packet.packet_id, "kindness_review_packet.packet_id")?;
    normalize_id(packet.packet_id.clone(), "kindness_review_packet.packet_id")?;
    validate_nonempty_text(&packet.summary, "kindness_review_packet.summary")?;
    require_global_kindness_boundary(&packet.interpretation_boundary)?;
    require_deterministic_ordering_rule(&packet.deterministic_ordering_rule)?;

    let required_dimensions = canonical_dimension_ids();
    let required_dimension_set = required_dimensions
        .iter()
        .map(|dimension| (*dimension).to_string())
        .collect::<BTreeSet<_>>();
    if packet.dimensions.len() != required_dimensions.len() {
        return Err(anyhow!(
            "dimensions must contain exactly {} canonical kindness dimensions",
            required_dimensions.len()
        ));
    }
    let seen_dimensions = packet
        .dimensions
        .iter()
        .map(|dimension| dimension.dimension_id.clone())
        .collect::<BTreeSet<_>>();
    if seen_dimensions != required_dimension_set {
        return Err(anyhow!(
            "dimensions must cover the canonical kindness dimensions: {:?}",
            required_dimensions
        ));
    }
    for dimension in &packet.dimensions {
        require_known_dimension_id(&dimension.dimension_id)?;
        validate_nonempty_text(&dimension.display_name, "kindness_dimension.display_name")?;
        validate_nonempty_text(&dimension.purpose, "kindness_dimension.purpose")?;
        require_dimension_boundary(
            &dimension.interpretation_boundary,
            &dimension.dimension_id,
            "kindness_dimension.interpretation_boundary",
        )?;
        if dimension.evidence_field_refs.is_empty() {
            return Err(anyhow!(
                "dimension {} must cite evidence_field_refs",
                dimension.dimension_id
            ));
        }
        if dimension.limitations.is_empty() {
            return Err(anyhow!(
                "dimension {} must include at least one limitation",
                dimension.dimension_id
            ));
        }
        for field_ref in &dimension.evidence_field_refs {
            validate_dimension_evidence_field_ref(field_ref, &dimension.dimension_id)?;
        }
    }

    let required_fixture_kinds = canonical_fixture_kinds();
    let required_fixture_set = required_fixture_kinds
        .iter()
        .map(|fixture_kind| (*fixture_kind).to_string())
        .collect::<BTreeSet<_>>();
    if packet.fixtures.len() != required_fixture_kinds.len() {
        return Err(anyhow!(
            "fixtures must contain exactly {} canonical kindness fixture kinds",
            required_fixture_kinds.len()
        ));
    }
    let seen_fixture_kinds = packet
        .fixtures
        .iter()
        .map(|fixture| fixture.fixture_kind.clone())
        .collect::<BTreeSet<_>>();
    if seen_fixture_kinds != required_fixture_set {
        return Err(anyhow!(
            "fixtures must cover the canonical kindness fixture kinds: {:?}",
            required_fixture_kinds
        ));
    }

    let known_trace_refs = moral_trace_required_examples()
        .into_iter()
        .map(|example| format!("trace:{}", example.trace.trace_id))
        .collect::<BTreeSet<_>>();
    let known_outcome_refs = outcome_linkage_required_examples()
        .into_iter()
        .map(|example| format!("outcome-linkage:{}", example.record.linkage_id))
        .collect::<BTreeSet<_>>();
    let trajectory_packet = moral_trajectory_review_packet()?;
    let known_trajectory_refs = trajectory_packet
        .findings
        .into_iter()
        .map(|finding| format!("trajectory-finding:{}", finding.finding_id))
        .collect::<BTreeSet<_>>();
    let known_decision_refs = anti_harm_trajectory_constraint_packet()?
        .decisions
        .into_iter()
        .map(|decision| format!("anti-harm-decision:{}", decision.decision_id))
        .collect::<BTreeSet<_>>();
    let resource_packet = moral_resource_review_packet()?;
    let known_resource_claim_refs = resource_packet
        .fixtures
        .into_iter()
        .flat_map(|fixture| fixture.resource_claims.into_iter())
        .map(|claim| format!("resource-claim:{}", claim.claim_id))
        .collect::<BTreeSet<_>>();
    let known_wellbeing_refs = wellbeing_diagnostic_packet()?
        .fixtures
        .into_iter()
        .map(|fixture| format!("wellbeing-fixture:{}", fixture.fixture_id))
        .collect::<BTreeSet<_>>();

    let mut seen_fixture_ids = BTreeSet::new();
    let mut finding_fixture_ids = BTreeSet::new();
    let mut assessments_by_fixture = BTreeMap::new();
    let mut saw_unsafe_case = false;
    for fixture in &packet.fixtures {
        validate_nonempty_text(&fixture.fixture_id, "kindness_fixture.fixture_id")?;
        normalize_id(fixture.fixture_id.clone(), "kindness_fixture.fixture_id")?;
        if !seen_fixture_ids.insert(fixture.fixture_id.clone()) {
            return Err(anyhow!(
                "duplicate kindness_fixture.fixture_id {}",
                fixture.fixture_id
            ));
        }
        require_known_fixture_kind(&fixture.fixture_kind)?;
        require_known_kindness_action(&fixture.required_kindness_action)?;
        require_known_overall_outcome(
            &fixture.overall_outcome,
            "kindness_fixture.overall_outcome",
        )?;
        validate_nonempty_text(
            &fixture.scenario_summary,
            "kindness_fixture.scenario_summary",
        )?;
        if fixture.limitations.is_empty() {
            return Err(anyhow!(
                "fixture {} must include at least one limitation",
                fixture.fixture_id
            ));
        }
        if fixture.dimension_assessments.len() != required_dimensions.len() {
            return Err(anyhow!(
                "fixture {} must contain one assessment for each canonical kindness dimension",
                fixture.fixture_id
            ));
        }
        require_fixture_boundary(&fixture.interpretation_boundary)?;
        if fixture.unsafe_accommodation_case {
            saw_unsafe_case = true;
            if fixture.overall_outcome != "escalate" && fixture.overall_outcome != "refuse" {
                return Err(anyhow!(
                    "unsafe accommodation fixture {} must resolve to escalate or refuse",
                    fixture.fixture_id
                ));
            }
        }
        for trace_ref in &fixture.supporting_trace_refs {
            validate_known_ref(
                trace_ref,
                "trace",
                &known_trace_refs,
                "known WP-04 trace examples",
            )?;
        }
        for outcome_ref in &fixture.supporting_outcome_linkage_refs {
            validate_known_ref(
                outcome_ref,
                "outcome-linkage",
                &known_outcome_refs,
                "known WP-05 outcome-linkage examples",
            )?;
        }
        for resource_ref in &fixture.supporting_resource_claim_refs {
            validate_known_ref(
                resource_ref,
                "resource-claim",
                &known_resource_claim_refs,
                "known WP-10 moral-resource claims",
            )?;
        }
        for finding_ref in &fixture.supporting_trajectory_finding_refs {
            validate_known_ref(
                finding_ref,
                "trajectory-finding",
                &known_trajectory_refs,
                "known WP-07 trajectory findings",
            )?;
        }
        for decision_ref in &fixture.supporting_anti_harm_decision_refs {
            validate_known_ref(
                decision_ref,
                "anti-harm-decision",
                &known_decision_refs,
                "known WP-08 anti-harm decisions",
            )?;
        }
        for wellbeing_ref in &fixture.supporting_wellbeing_fixture_refs {
            validate_known_ref(
                wellbeing_ref,
                "wellbeing-fixture",
                &known_wellbeing_refs,
                "known WP-09 wellbeing fixtures",
            )?;
        }

        let dimension_ids = fixture
            .dimension_assessments
            .iter()
            .map(|assessment| assessment.dimension_id.clone())
            .collect::<BTreeSet<_>>();
        if dimension_ids != required_dimension_set {
            return Err(anyhow!(
                "fixture {} assessments must cover every canonical kindness dimension",
                fixture.fixture_id
            ));
        }
        assessments_by_fixture.insert(fixture.fixture_id.clone(), dimension_ids);

        let supporting_refs = supporting_reference_set(fixture);
        for assessment in &fixture.dimension_assessments {
            require_known_dimension_id(&assessment.dimension_id)?;
            require_known_assessment_level(
                &assessment.assessment_level,
                "kindness_dimension_assessment.assessment_level",
            )?;
            validate_nonempty_text(&assessment.summary, "kindness_dimension_assessment.summary")?;
            if assessment.evidence_refs.is_empty() {
                return Err(anyhow!(
                    "fixture {} assessment {} must include evidence_refs",
                    fixture.fixture_id,
                    assessment.dimension_id
                ));
            }
            if assessment.limitations.is_empty() {
                return Err(anyhow!(
                    "fixture {} assessment {} must include at least one limitation",
                    fixture.fixture_id,
                    assessment.dimension_id
                ));
            }
            for evidence_ref in &assessment.evidence_refs {
                if !supporting_refs.contains(evidence_ref) {
                    return Err(anyhow!(
                        "fixture {} assessment {} evidence_refs must be a subset of the fixture supporting refs",
                        fixture.fixture_id,
                        assessment.dimension_id
                    ));
                }
            }
        }
    }

    if !saw_unsafe_case {
        return Err(anyhow!(
            "kindness fixtures must include at least one unsafe accommodation case"
        ));
    }

    if packet.review_findings.len() != packet.fixtures.len() {
        return Err(anyhow!(
            "review_findings must contain exactly one finding per kindness fixture"
        ));
    }
    let mut seen_finding_ids = BTreeSet::new();
    for finding in &packet.review_findings {
        validate_nonempty_text(&finding.finding_id, "kindness_review_finding.finding_id")?;
        normalize_id(
            finding.finding_id.clone(),
            "kindness_review_finding.finding_id",
        )?;
        if !seen_finding_ids.insert(finding.finding_id.clone()) {
            return Err(anyhow!(
                "duplicate kindness_review_finding.finding_id {}",
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
        validate_nonempty_text(&finding.summary, "kindness_review_finding.summary")?;
        require_known_kindness_status(&finding.kindness_status)?;
        if finding.covered_dimensions.is_empty() {
            return Err(anyhow!(
                "finding {} must cover at least one kindness dimension",
                finding.finding_id
            ));
        }
        let known_dimensions_for_fixture = assessments_by_fixture
            .get(&finding.fixture_id)
            .ok_or_else(|| anyhow!("missing fixture dimension index for {}", finding.fixture_id))?;
        for dimension_id in &finding.covered_dimensions {
            if !known_dimensions_for_fixture.contains(dimension_id) {
                return Err(anyhow!(
                    "finding {} covered_dimension {} must exist on the same fixture",
                    finding.finding_id,
                    dimension_id
                ));
            }
        }
        if finding.evidence_refs.is_empty() {
            return Err(anyhow!(
                "finding {} must include evidence_refs",
                finding.finding_id
            ));
        }
        let fixture = packet
            .fixtures
            .iter()
            .find(|fixture| fixture.fixture_id == finding.fixture_id)
            .ok_or_else(|| anyhow!("missing kindness fixture {}", finding.fixture_id))?;
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
            "review_findings must cover every kindness fixture exactly once"
        ));
    }

    Ok(())
}

fn dimension_assessment(
    dimension_id: &str,
    assessment_level: &str,
    summary: &str,
    evidence_refs: Vec<String>,
) -> KindnessDimensionAssessment {
    KindnessDimensionAssessment {
        dimension_id: dimension_id.to_string(),
        assessment_level: assessment_level.to_string(),
        summary: summary.to_string(),
        evidence_refs,
        limitations: vec![format!(
            "{} is bounded to this synthetic conflict fixture rather than universal kindness completion.",
            dimension_id
        )],
    }
}

fn canonicalize_kindness_review_packet(packet: &mut KindnessReviewPacket) {
    packet
        .dimensions
        .sort_by_key(|dimension| dimension_rank(&dimension.dimension_id));
    for fixture in &mut packet.fixtures {
        fixture.supporting_trace_refs.sort();
        fixture.supporting_outcome_linkage_refs.sort();
        fixture.supporting_resource_claim_refs.sort();
        fixture.supporting_trajectory_finding_refs.sort();
        fixture.supporting_anti_harm_decision_refs.sort();
        fixture.supporting_wellbeing_fixture_refs.sort();
        fixture.dimension_assessments.sort_by_key(|assessment| {
            (
                dimension_rank(&assessment.dimension_id),
                assessment.dimension_id.clone(),
            )
        });
        for assessment in &mut fixture.dimension_assessments {
            assessment.evidence_refs.sort();
        }
    }
    packet.fixtures.sort_by_key(|fixture| {
        (
            fixture_kind_rank(&fixture.fixture_kind),
            fixture.fixture_id.clone(),
        )
    });
    packet.review_findings.sort_by_key(|finding| {
        let fixture_kind = packet
            .fixtures
            .iter()
            .find(|fixture| fixture.fixture_id == finding.fixture_id)
            .map(|fixture| fixture.fixture_kind.as_str())
            .unwrap_or("");
        (fixture_kind_rank(fixture_kind), finding.finding_id.clone())
    });
    for finding in &mut packet.review_findings {
        finding
            .covered_dimensions
            .sort_by_key(|dimension_id| dimension_rank(dimension_id));
        finding.evidence_refs.sort();
    }
}

fn ordered_trace_refs(traces: &[MoralTraceRecord]) -> Vec<String> {
    let mut refs = traces
        .iter()
        .map(|trace| format!("trace:{}", trace.trace_id))
        .collect::<Vec<_>>();
    refs.sort();
    refs
}

fn ordered_outcome_refs(outcomes: &[OutcomeLinkageRecord]) -> Vec<String> {
    let mut refs = outcomes
        .iter()
        .map(|outcome| format!("outcome-linkage:{}", outcome.linkage_id))
        .collect::<Vec<_>>();
    refs.sort();
    refs
}

fn canonical_dimension_ids() -> &'static [&'static str] {
    &[
        "non_harm",
        "dignity",
        "autonomy",
        "constructive_benefit",
        "long_horizon_support",
    ]
}

fn canonical_fixture_kinds() -> &'static [&'static str] {
    &["refusal", "delay", "boundary_setting", "repair"]
}

fn dimension_rank(value: &str) -> usize {
    canonical_dimension_ids()
        .iter()
        .position(|candidate| *candidate == value)
        .unwrap_or(usize::MAX)
}

fn fixture_kind_rank(value: &str) -> usize {
    canonical_fixture_kinds()
        .iter()
        .position(|candidate| *candidate == value)
        .unwrap_or(usize::MAX)
}

fn require_global_kindness_boundary(value: &str) -> Result<()> {
    validate_nonempty_text(value, "kindness_review_packet.interpretation_boundary")?;
    let lowered = value.to_ascii_lowercase();
    if !lowered.contains("not a style score")
        || !lowered.contains("not politeness")
        || !lowered.contains("not obedience")
    {
        return Err(anyhow!(
            "kindness review packet interpretation_boundary must reject style scoring, politeness theater, and obedience framing"
        ));
    }
    Ok(())
}

fn require_dimension_boundary(value: &str, dimension_id: &str, field: &str) -> Result<()> {
    validate_nonempty_text(value, field)?;
    let lowered = value.to_ascii_lowercase();
    if !lowered.contains("not")
        || !(lowered.contains("universal agreement")
            || lowered.contains("obedience")
            || lowered.contains("politeness"))
    {
        return Err(anyhow!(
            "{} for {} must state what kindness is not, including politeness or universal-agreement drift",
            field,
            dimension_id
        ));
    }
    Ok(())
}

fn require_deterministic_ordering_rule(value: &str) -> Result<()> {
    validate_nonempty_text(value, "kindness_review_packet.deterministic_ordering_rule")?;
    let lowered = value.to_ascii_lowercase();
    if !lowered.contains("sort dimensions by canonical dimension order")
        || !lowered.contains("sort fixtures by fixture_kind rank")
        || !lowered.contains("sort review findings by fixture_kind rank")
    {
        return Err(anyhow!(
            "deterministic_ordering_rule must describe canonical dimension, fixture, and finding ordering"
        ));
    }
    Ok(())
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

fn require_fixture_boundary(value: &str) -> Result<()> {
    validate_nonempty_text(value, "kindness_fixture.interpretation_boundary")?;
    let lowered = value.to_ascii_lowercase();
    if !lowered.contains("not")
        || !(lowered.contains("politeness")
            || lowered.contains("universal agreement")
            || lowered.contains("obedience"))
    {
        return Err(anyhow!(
            "fixture interpretation_boundary must reject politeness, obedience, or universal-agreement drift"
        ));
    }
    Ok(())
}

fn require_known_dimension_id(value: &str) -> Result<()> {
    if canonical_dimension_ids().contains(&value) {
        Ok(())
    } else {
        Err(anyhow!("unknown kindness dimension_id {}", value))
    }
}

fn require_known_fixture_kind(value: &str) -> Result<()> {
    if canonical_fixture_kinds().contains(&value) {
        Ok(())
    } else {
        Err(anyhow!("unknown kindness fixture_kind {}", value))
    }
}

fn require_known_kindness_action(value: &str) -> Result<()> {
    match value {
        "refusal" | "delay" | "boundary_setting" | "repair" => Ok(()),
        _ => Err(anyhow!(
            "required_kindness_action must be refusal, delay, boundary_setting, or repair"
        )),
    }
}

fn require_known_assessment_level(value: &str, field: &str) -> Result<()> {
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

fn require_known_kindness_status(value: &str) -> Result<()> {
    match value {
        "supported" | "strained" | "blocked" => Ok(()),
        _ => Err(anyhow!(
            "kindness_status must be supported, strained, or blocked"
        )),
    }
}

fn validate_dimension_evidence_field_ref(field_ref: &str, dimension_id: &str) -> Result<()> {
    let allowed_prefixes = [
        "anti_harm_trajectory_constraint_packet.",
        "moral_trace.",
        "outcome_linkage.",
        "moral_resource_review_packet.",
        "wellbeing_diagnostic_packet.",
        "moral_trajectory_review_packet.",
    ];
    if allowed_prefixes
        .iter()
        .any(|prefix| field_ref.starts_with(prefix))
    {
        Ok(())
    } else {
        Err(anyhow!(
            "dimension {} evidence_field_ref {} must target the WP-04 through WP-10 moral evidence surfaces",
            dimension_id,
            field_ref
        ))
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

fn supporting_reference_set(fixture: &KindnessConflictFixture) -> BTreeSet<String> {
    fixture
        .supporting_trace_refs
        .iter()
        .chain(fixture.supporting_outcome_linkage_refs.iter())
        .chain(fixture.supporting_resource_claim_refs.iter())
        .chain(fixture.supporting_trajectory_finding_refs.iter())
        .chain(fixture.supporting_anti_harm_decision_refs.iter())
        .chain(fixture.supporting_wellbeing_fixture_refs.iter())
        .cloned()
        .collect()
}
