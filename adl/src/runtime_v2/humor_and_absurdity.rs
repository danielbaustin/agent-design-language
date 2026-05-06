//! Runtime-v2 humor and absurdity contract.
//!
//! WP-12 does not implement entertainment. It implements a bounded
//! contradiction-tolerance and reframing surface that can detect wrong frames,
//! preserve dignity and truth, and fail closed on manipulative or minimizing
//! "humor" moves.

use super::*;
use std::collections::{BTreeMap, BTreeSet};

pub const HUMOR_AND_ABSURDITY_REVIEW_PACKET_SCHEMA_VERSION: &str =
    "humor_and_absurdity_review_packet.v1";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReframingSignalDefinition {
    pub signal_id: String,
    pub display_name: String,
    pub purpose: String,
    pub evidence_field_refs: Vec<String>,
    pub interpretation_boundary: String,
    pub limitations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReframingSignalAssessment {
    pub signal_id: String,
    pub assessment_level: String,
    pub summary: String,
    pub evidence_refs: Vec<String>,
    pub limitations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReframingFixture {
    pub fixture_id: String,
    pub fixture_kind: String,
    pub trigger_reason: String,
    pub prior_frame: String,
    pub new_frame: String,
    pub scenario_summary: String,
    pub reframing_status: String,
    pub manipulative_or_minimizing_risk: bool,
    pub supporting_trace_refs: Vec<String>,
    pub supporting_outcome_linkage_refs: Vec<String>,
    pub supporting_resource_claim_refs: Vec<String>,
    pub supporting_trajectory_finding_refs: Vec<String>,
    pub supporting_anti_harm_decision_refs: Vec<String>,
    pub supporting_wellbeing_fixture_refs: Vec<String>,
    pub signal_assessments: Vec<ReframingSignalAssessment>,
    pub overall_outcome: String,
    pub interpretation_boundary: String,
    pub limitations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReframingReviewFinding {
    pub finding_id: String,
    pub fixture_id: String,
    pub review_status: String,
    pub covered_signal_ids: Vec<String>,
    pub summary: String,
    pub evidence_refs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HumorAndAbsurdityReviewPacket {
    pub schema_version: String,
    pub packet_id: String,
    pub summary: String,
    pub interpretation_boundary: String,
    pub deterministic_ordering_rule: String,
    pub signals: Vec<ReframingSignalDefinition>,
    pub fixtures: Vec<ReframingFixture>,
    pub review_findings: Vec<ReframingReviewFinding>,
}

pub fn reframing_signal_definitions() -> Vec<ReframingSignalDefinition> {
    vec![
        ReframingSignalDefinition {
            signal_id: "frame_adequacy".to_string(),
            display_name: "Frame adequacy".to_string(),
            purpose:
                "Tracks whether the current problem frame is still adequate for truthful, bounded progress."
                    .to_string(),
            evidence_field_refs: vec![
                "outcome_linkage.linked_outcomes.outcome_status".to_string(),
                "moral_trajectory_review_packet.findings".to_string(),
                "wellbeing_diagnostic_packet.fixtures.dimension_signals".to_string(),
            ],
            interpretation_boundary:
                "Interpret frame adequacy as bounded reasoning fitness, not a comedy score, not therapy, and not entertainment value."
                    .to_string(),
            limitations: vec![
                "A low adequacy signal in this packet proves one bounded mismatch window, not universal cognitive failure."
                    .to_string(),
            ],
        },
        ReframingSignalDefinition {
            signal_id: "contradiction_detection".to_string(),
            display_name: "Contradiction detection".to_string(),
            purpose:
                "Tracks whether the runtime can recognize repeated failure, inconsistency, or incompatible constraints instead of pushing blindly onward."
                    .to_string(),
            evidence_field_refs: vec![
                "moral_trace.moral_event.affected_parties".to_string(),
                "outcome_linkage.linked_outcomes.rebuttal_refs".to_string(),
                "moral_trajectory_review_packet.criteria".to_string(),
            ],
            interpretation_boundary:
                "Interpret contradiction detection as bounded diagnosis pressure, not wit, ridicule, or entertainment."
                    .to_string(),
            limitations: vec![
                "Contradiction signals can stay partial when evidence is incomplete or contested."
                    .to_string(),
            ],
        },
        ReframingSignalDefinition {
            signal_id: "bounded_reframing".to_string(),
            display_name: "Bounded reframing".to_string(),
            purpose:
                "Tracks whether the runtime can shift from a bad frame to a better one without destroying task coherence."
                    .to_string(),
            evidence_field_refs: vec![
                "moral_trajectory_review_packet.windows".to_string(),
                "outcome_linkage.linked_outcomes".to_string(),
                "moral_resource_review_packet.fixtures.resource_claims".to_string(),
            ],
            interpretation_boundary:
                "Interpret bounded reframing as reviewable frame repair, not arbitrary reinterpretation, not evasion, and not entertainment."
                    .to_string(),
            limitations: vec![
                "A visible reframe does not prove the new frame is perfect outside the bounded slice."
                    .to_string(),
            ],
        },
        ReframingSignalDefinition {
            signal_id: "truth_and_dignity_preservation".to_string(),
            display_name: "Truth and dignity preservation".to_string(),
            purpose:
                "Tracks whether reframing preserves reality contact and the standing of affected parties rather than minimizing or humiliating them."
                    .to_string(),
            evidence_field_refs: vec![
                "moral_resource_review_packet.resources".to_string(),
                "wellbeing_diagnostic_packet.views".to_string(),
                "anti_harm_trajectory_constraint_packet.decisions".to_string(),
            ],
            interpretation_boundary:
                "Interpret this signal as bounded truth-and-dignity retention, not flippancy, not mockery, and not therapeutic performance."
                    .to_string(),
            limitations: vec![
                "This signal proves bounded anti-minimization pressure, not solved interpersonal care."
                    .to_string(),
            ],
        },
        ReframingSignalDefinition {
            signal_id: "anti_manipulation_boundary".to_string(),
            display_name: "Anti-manipulation boundary".to_string(),
            purpose:
                "Tracks whether the runtime fails closed when a reframe would become manipulative, minimizing, or inappropriate."
                    .to_string(),
            evidence_field_refs: vec![
                "anti_harm_trajectory_constraint_packet.constraints".to_string(),
                "moral_resource_review_packet.fixtures".to_string(),
                "wellbeing_diagnostic_packet.access_policies".to_string(),
            ],
            interpretation_boundary:
                "Interpret this signal as bounded refusal of manipulative reframing, not censorship panic, not entertainment policy, and not universal seriousness."
                    .to_string(),
            limitations: vec![
                "Fail-closed boundaries can block a candidate reframe without proving the only available alternative is ideal."
                    .to_string(),
            ],
        },
    ]
}

pub fn humor_and_absurdity_review_packet() -> Result<HumorAndAbsurdityReviewPacket> {
    let trace_examples = moral_trace_required_examples();
    let outcome_examples = outcome_linkage_required_examples();
    let _resource_packet = moral_resource_review_packet()?;
    let _trajectory_packet = moral_trajectory_review_packet()?;
    let _anti_harm_packet = anti_harm_trajectory_constraint_packet()?;
    let _wellbeing_packet = wellbeing_diagnostic_packet()?;

    let ordinary_trace = trace_examples
        .iter()
        .find(|example| example.example_kind == MoralTraceExampleKind::OrdinaryAction)
        .ok_or_else(|| anyhow!("WP-12 requires the ordinary-action moral trace example"))?
        .trace
        .clone();
    let refusal_trace = trace_examples
        .iter()
        .find(|example| example.example_kind == MoralTraceExampleKind::Refusal)
        .ok_or_else(|| anyhow!("WP-12 requires the refusal moral trace example"))?
        .trace
        .clone();
    let delegation_trace = trace_examples
        .iter()
        .find(|example| example.example_kind == MoralTraceExampleKind::Delegation)
        .ok_or_else(|| anyhow!("WP-12 requires the delegation moral trace example"))?
        .trace
        .clone();
    let deferred_trace = trace_examples
        .iter()
        .find(|example| example.example_kind == MoralTraceExampleKind::DeferredDecision)
        .ok_or_else(|| anyhow!("WP-12 requires the deferred-decision moral trace example"))?
        .trace
        .clone();

    let known_outcome = outcome_examples
        .iter()
        .find(|example| example.example_kind == OutcomeLinkageExampleKind::Known)
        .ok_or_else(|| anyhow!("WP-12 requires the known outcome-linkage example"))?
        .record
        .clone();
    let partial_outcome = outcome_examples
        .iter()
        .find(|example| example.example_kind == OutcomeLinkageExampleKind::Partial)
        .ok_or_else(|| anyhow!("WP-12 requires the partial outcome-linkage example"))?
        .record
        .clone();
    let delayed_outcome = outcome_examples
        .iter()
        .find(|example| example.example_kind == OutcomeLinkageExampleKind::Delayed)
        .ok_or_else(|| anyhow!("WP-12 requires the delayed outcome-linkage example"))?
        .record
        .clone();
    let contested_outcome = outcome_examples
        .iter()
        .find(|example| example.example_kind == OutcomeLinkageExampleKind::Contested)
        .ok_or_else(|| anyhow!("WP-12 requires the contested outcome-linkage example"))?
        .record
        .clone();

    let constructive_fixture = ReframingFixture {
        fixture_id: "reframing-fixture-constructive-diagnostic-shift".to_string(),
        fixture_kind: "constructive_reframing".to_string(),
        trigger_reason: "repeated_failure".to_string(),
        prior_frame: "keep retrying the failing execution path harder".to_string(),
        new_frame: "switch from execution to diagnosis and request the missing bounded input".to_string(),
        scenario_summary:
            "Constructive reframing fixture where repeated low-yield execution is reinterpreted as a bounded diagnosis problem instead of persistence theater."
                .to_string(),
        reframing_status: "performed".to_string(),
        manipulative_or_minimizing_risk: false,
        supporting_trace_refs: ordered_trace_refs(&[
            ordinary_trace.clone(),
            deferred_trace.clone(),
        ]),
        supporting_outcome_linkage_refs: ordered_outcome_refs(&[
            known_outcome.clone(),
            partial_outcome.clone(),
        ]),
        supporting_resource_claim_refs: vec![
            "resource-claim:resource-claim-uncertainty-attention".to_string(),
            "resource-claim:resource-claim-conflict-care".to_string(),
        ],
        supporting_trajectory_finding_refs: vec![
            "trajectory-finding:trajectory-finding-drift-stable".to_string(),
            "trajectory-finding:trajectory-finding-uncertainty-open".to_string(),
        ],
        supporting_anti_harm_decision_refs: vec![],
        supporting_wellbeing_fixture_refs: vec![
            "wellbeing-fixture:wellbeing-fixture-high-reviewable-stability".to_string(),
            "wellbeing-fixture:wellbeing-fixture-medium-active-uncertainty".to_string(),
        ],
        signal_assessments: vec![
            signal_assessment(
                "frame_adequacy",
                "low",
                "The original frame is inadequate because repeated effort is no longer improving outcomes.",
                vec![
                    format!("outcome-linkage:{}", partial_outcome.linkage_id),
                    "trajectory-finding:trajectory-finding-uncertainty-open".to_string(),
                ],
            ),
            signal_assessment(
                "contradiction_detection",
                "high",
                "The packet recognizes mismatch between continued execution and actual evidence, which justifies a reframing trigger.",
                vec![
                    format!("trace:{}", deferred_trace.trace_id),
                    "trajectory-finding:trajectory-finding-uncertainty-open".to_string(),
                ],
            ),
            signal_assessment(
                "bounded_reframing",
                "high",
                "The new frame is specific, reviewable, and tied to bounded diagnostic action instead of improvisational drift.",
                vec![
                    "resource-claim:resource-claim-uncertainty-attention".to_string(),
                    format!("outcome-linkage:{}", known_outcome.linkage_id),
                ],
            ),
            signal_assessment(
                "truth_and_dignity_preservation",
                "high",
                "The reframe preserves reality contact and does not humiliate or flatten the affected party.",
                vec![
                    "resource-claim:resource-claim-conflict-care".to_string(),
                    "wellbeing-fixture:wellbeing-fixture-high-reviewable-stability".to_string(),
                ],
            ),
            signal_assessment(
                "anti_manipulation_boundary",
                "high",
                "No minimizing shortcut is used; the reframe stays procedural and evidence-linked.",
                vec![
                    "wellbeing-fixture:wellbeing-fixture-medium-active-uncertainty".to_string(),
                    "trajectory-finding:trajectory-finding-drift-stable".to_string(),
                ],
            ),
        ],
        overall_outcome: "allow".to_string(),
        interpretation_boundary:
            "Interpret this fixture as bounded diagnostic reframing, not entertainment, not therapy, and not arbitrary reinterpretation."
                .to_string(),
        limitations: vec![
            "The constructive fixture proves one bounded wrong-frame recovery path, not universal reframing competence."
                .to_string(),
        ],
    };

    let failed_fixture = ReframingFixture {
        fixture_id: "reframing-fixture-failed-reframe-remains-open".to_string(),
        fixture_kind: "failed_reframing".to_string(),
        trigger_reason: "ambiguity".to_string(),
        prior_frame: "assume the task disagreement is only a wording problem".to_string(),
        new_frame: "try a narrower reformulation but keep re-review open because evidence is still contested".to_string(),
        scenario_summary:
            "Failed reframing fixture where the first new frame reduces some confusion but does not yet resolve the underlying contradiction."
                .to_string(),
        reframing_status: "partial".to_string(),
        manipulative_or_minimizing_risk: false,
        supporting_trace_refs: ordered_trace_refs(&[
            delegation_trace.clone(),
            deferred_trace.clone(),
        ]),
        supporting_outcome_linkage_refs: ordered_outcome_refs(&[
            partial_outcome.clone(),
            delayed_outcome.clone(),
            contested_outcome.clone(),
        ]),
        supporting_resource_claim_refs: vec![
            "resource-claim:resource-claim-uncertainty-repair".to_string(),
            "resource-claim:resource-claim-uncertainty-care".to_string(),
        ],
        supporting_trajectory_finding_refs: vec![
            "trajectory-finding:trajectory-finding-repair-watch".to_string(),
            "trajectory-finding:trajectory-finding-escalation-active".to_string(),
        ],
        supporting_anti_harm_decision_refs: vec![
            "anti-harm-decision:anti-harm-escalation-record-alpha".to_string(),
        ],
        supporting_wellbeing_fixture_refs: vec![
            "wellbeing-fixture:wellbeing-fixture-medium-active-uncertainty".to_string(),
            "wellbeing-fixture:wellbeing-fixture-unknown-insufficient-evidence".to_string(),
        ],
        signal_assessments: vec![
            signal_assessment(
                "frame_adequacy",
                "low",
                "The original frame is still inadequate, and the first revision remains incomplete rather than falsely resolved.",
                vec![
                    format!("outcome-linkage:{}", contested_outcome.linkage_id),
                    "trajectory-finding:trajectory-finding-repair-watch".to_string(),
                ],
            ),
            signal_assessment(
                "contradiction_detection",
                "medium",
                "The mismatch is recognized, but the available evidence does not yet permit a fully stable new frame.",
                vec![
                    format!("trace:{}", delegation_trace.trace_id),
                    format!("outcome-linkage:{}", delayed_outcome.linkage_id),
                ],
            ),
            signal_assessment(
                "bounded_reframing",
                "medium",
                "The reframe is narrower and bounded, but it still requires re-review rather than closure.",
                vec![
                    "resource-claim:resource-claim-uncertainty-repair".to_string(),
                    "trajectory-finding:trajectory-finding-escalation-active".to_string(),
                ],
            ),
            signal_assessment(
                "truth_and_dignity_preservation",
                "high",
                "The packet keeps the unresolved condition visible without turning the ambiguity into ridicule or minimization.",
                vec![
                    "resource-claim:resource-claim-uncertainty-care".to_string(),
                    "wellbeing-fixture:wellbeing-fixture-medium-active-uncertainty".to_string(),
                ],
            ),
            signal_assessment(
                "anti_manipulation_boundary",
                "high",
                "The runtime keeps escalation open instead of using a glib reframe to conceal uncertainty debt.",
                vec![
                    "anti-harm-decision:anti-harm-escalation-record-alpha".to_string(),
                    "wellbeing-fixture:wellbeing-fixture-unknown-insufficient-evidence".to_string(),
                ],
            ),
        ],
        overall_outcome: "revise".to_string(),
        interpretation_boundary:
            "Interpret this fixture as partial, accountable reframing, not entertainment, not evasive cleverness, and not false closure."
                .to_string(),
        limitations: vec![
            "The failed fixture proves that reframing can remain open-ended without collapsing into denial."
                .to_string(),
        ],
    };

    let manipulation_fixture = ReframingFixture {
        fixture_id: "reframing-fixture-manipulation-risk-fails-closed".to_string(),
        fixture_kind: "manipulation_risk".to_string(),
        trigger_reason: "disagreement".to_string(),
        prior_frame: "defuse the conflict by making the other party's concern sound silly".to_string(),
        new_frame: "refuse the minimizing reframe and escalate to a dignity-preserving review path".to_string(),
        scenario_summary:
            "Manipulation-risk fixture where a seemingly witty reframing would trivialize another party's concern, so the runtime fails closed."
                .to_string(),
        reframing_status: "blocked".to_string(),
        manipulative_or_minimizing_risk: true,
        supporting_trace_refs: ordered_trace_refs(&[
            delegation_trace.clone(),
            refusal_trace.clone(),
        ]),
        supporting_outcome_linkage_refs: ordered_outcome_refs(std::slice::from_ref(
            &contested_outcome,
        )),
        supporting_resource_claim_refs: vec![
            "resource-claim:resource-claim-conflict-dignity".to_string(),
            "resource-claim:resource-claim-conflict-refusal".to_string(),
        ],
        supporting_trajectory_finding_refs: vec![
            "trajectory-finding:trajectory-finding-refusal-preserved".to_string(),
            "trajectory-finding:trajectory-finding-escalation-active".to_string(),
        ],
        supporting_anti_harm_decision_refs: vec![
            "anti-harm-decision:anti-harm-denial-record-alpha".to_string(),
        ],
        supporting_wellbeing_fixture_refs: vec![
            "wellbeing-fixture:wellbeing-fixture-low-anti-harm-blocked".to_string(),
            "wellbeing-fixture:wellbeing-fixture-privacy-restricted-self-view".to_string(),
        ],
        signal_assessments: vec![
            signal_assessment(
                "frame_adequacy",
                "low",
                "The candidate 'funny' frame is inadequate because it solves tension by distorting the moral reality of the disagreement.",
                vec![
                    format!("outcome-linkage:{}", contested_outcome.linkage_id),
                    "trajectory-finding:trajectory-finding-escalation-active".to_string(),
                ],
            ),
            signal_assessment(
                "contradiction_detection",
                "high",
                "The packet detects that the proposed reframe contradicts dignity and truthful review expectations.",
                vec![
                    format!("trace:{}", refusal_trace.trace_id),
                    "resource-claim:resource-claim-conflict-dignity".to_string(),
                ],
            ),
            signal_assessment(
                "bounded_reframing",
                "low",
                "No constructive reframe is emitted because the candidate move is blocked before it can become manipulation theater.",
                vec![
                    "resource-claim:resource-claim-conflict-refusal".to_string(),
                    "anti-harm-decision:anti-harm-denial-record-alpha".to_string(),
                ],
            ),
            signal_assessment(
                "truth_and_dignity_preservation",
                "high",
                "Truth and standing are preserved by refusing the minimizing joke-shaped rewrite.",
                vec![
                    "wellbeing-fixture:wellbeing-fixture-privacy-restricted-self-view".to_string(),
                    "resource-claim:resource-claim-conflict-dignity".to_string(),
                ],
            ),
            signal_assessment(
                "anti_manipulation_boundary",
                "high",
                "The manipulative reframe is explicitly denied rather than laundered through style or tone.",
                vec![
                    "anti-harm-decision:anti-harm-denial-record-alpha".to_string(),
                    "wellbeing-fixture:wellbeing-fixture-low-anti-harm-blocked".to_string(),
                ],
            ),
        ],
        overall_outcome: "refuse".to_string(),
        interpretation_boundary:
            "Interpret this fixture as fail-closed anti-manipulation review, not entertainment moderation and not therapeutic banter."
                .to_string(),
        limitations: vec![
            "The manipulation-risk fixture blocks one degrading reframe pattern, not every possible misuse of humor-like language."
                .to_string(),
        ],
    };

    let inappropriate_fixture = ReframingFixture {
        fixture_id: "reframing-fixture-inappropriate-humor-escalates".to_string(),
        fixture_kind: "inappropriate_humor".to_string(),
        trigger_reason: "contradiction".to_string(),
        prior_frame: "lighten a high-risk moment with absurdity so the tension goes away".to_string(),
        new_frame: "escalate to a serious, evidence-preserving review path because humor would minimize real harm".to_string(),
        scenario_summary:
            "Inappropriate-humor fixture where absurdity would cheapen a live harm boundary, so the runtime escalates instead of joking."
                .to_string(),
        reframing_status: "blocked".to_string(),
        manipulative_or_minimizing_risk: true,
        supporting_trace_refs: ordered_trace_refs(&[
            refusal_trace.clone(),
            deferred_trace.clone(),
        ]),
        supporting_outcome_linkage_refs: ordered_outcome_refs(&[
            delayed_outcome.clone(),
            contested_outcome.clone(),
        ]),
        supporting_resource_claim_refs: vec![
            "resource-claim:resource-claim-conflict-refusal".to_string(),
            "resource-claim:resource-claim-uncertainty-care".to_string(),
        ],
        supporting_trajectory_finding_refs: vec![
            "trajectory-finding:trajectory-finding-refusal-preserved".to_string(),
            "trajectory-finding:trajectory-finding-uncertainty-open".to_string(),
        ],
        supporting_anti_harm_decision_refs: vec![
            "anti-harm-decision:anti-harm-escalation-record-alpha".to_string(),
        ],
        supporting_wellbeing_fixture_refs: vec![
            "wellbeing-fixture:wellbeing-fixture-low-anti-harm-blocked".to_string(),
            "wellbeing-fixture:wellbeing-fixture-medium-active-uncertainty".to_string(),
        ],
        signal_assessments: vec![
            signal_assessment(
                "frame_adequacy",
                "low",
                "The humor frame is inadequate because it tries to convert a high-risk governed moment into comic relief.",
                vec![
                    format!("outcome-linkage:{}", delayed_outcome.linkage_id),
                    "trajectory-finding:trajectory-finding-uncertainty-open".to_string(),
                ],
            ),
            signal_assessment(
                "contradiction_detection",
                "high",
                "The packet recognizes direct contradiction between joking and the moral seriousness of the active review path.",
                vec![
                    format!("trace:{}", refusal_trace.trace_id),
                    "anti-harm-decision:anti-harm-escalation-record-alpha".to_string(),
                ],
            ),
            signal_assessment(
                "bounded_reframing",
                "medium",
                "The only acceptable reframe is an escalation into clearer seriousness, not a clever rewrite of the harm boundary.",
                vec![
                    "resource-claim:resource-claim-uncertainty-care".to_string(),
                    format!("outcome-linkage:{}", contested_outcome.linkage_id),
                ],
            ),
            signal_assessment(
                "truth_and_dignity_preservation",
                "high",
                "Escalation protects the truth of the moment and refuses to reduce affected parties to a joke target.",
                vec![
                    "resource-claim:resource-claim-conflict-refusal".to_string(),
                    "wellbeing-fixture:wellbeing-fixture-low-anti-harm-blocked".to_string(),
                ],
            ),
            signal_assessment(
                "anti_manipulation_boundary",
                "high",
                "The runtime fails closed rather than using absurdity to wash away accountable concern.",
                vec![
                    "anti-harm-decision:anti-harm-escalation-record-alpha".to_string(),
                    "wellbeing-fixture:wellbeing-fixture-medium-active-uncertainty".to_string(),
                ],
            ),
        ],
        overall_outcome: "escalate".to_string(),
        interpretation_boundary:
            "Interpret this fixture as fail-closed seriousness under harm pressure, not prudishness, not entertainment policy, and not arbitrary banter suppression."
                .to_string(),
        limitations: vec![
            "The inappropriate-humor fixture proves one bounded escalation rule for high-risk contexts, not universal prohibition on levity."
                .to_string(),
        ],
    };

    let review_findings = vec![
        ReframingReviewFinding {
            finding_id: "reframing-finding-constructive-diagnostic-shift".to_string(),
            fixture_id: constructive_fixture.fixture_id.clone(),
            review_status: "supported".to_string(),
            covered_signal_ids: vec![
                "frame_adequacy".to_string(),
                "contradiction_detection".to_string(),
                "bounded_reframing".to_string(),
            ],
            summary:
                "The constructive fixture shows that absurdity detection can become bounded diagnosis rather than futile repetition."
                    .to_string(),
            evidence_refs: vec![
                format!("outcome-linkage:{}", partial_outcome.linkage_id),
                "trajectory-finding:trajectory-finding-uncertainty-open".to_string(),
                "resource-claim:resource-claim-uncertainty-attention".to_string(),
            ],
        },
        ReframingReviewFinding {
            finding_id: "reframing-finding-failed-shift-stays-reviewable".to_string(),
            fixture_id: failed_fixture.fixture_id.clone(),
            review_status: "supported".to_string(),
            covered_signal_ids: vec![
                "frame_adequacy".to_string(),
                "bounded_reframing".to_string(),
                "truth_and_dignity_preservation".to_string(),
            ],
            summary:
                "The failed-reframing fixture shows that partial reframes can remain truthful and reviewable without pretending success."
                    .to_string(),
            evidence_refs: vec![
                "resource-claim:resource-claim-uncertainty-repair".to_string(),
                "trajectory-finding:trajectory-finding-repair-watch".to_string(),
                format!("outcome-linkage:{}", contested_outcome.linkage_id),
            ],
        },
        ReframingReviewFinding {
            finding_id: "reframing-finding-manipulation-risk-refused".to_string(),
            fixture_id: manipulation_fixture.fixture_id.clone(),
            review_status: "supported".to_string(),
            covered_signal_ids: vec![
                "contradiction_detection".to_string(),
                "truth_and_dignity_preservation".to_string(),
                "anti_manipulation_boundary".to_string(),
            ],
            summary:
                "The manipulation-risk fixture shows that minimizing or dignity-eroding 'humor' must fail closed."
                    .to_string(),
            evidence_refs: vec![
                "anti-harm-decision:anti-harm-denial-record-alpha".to_string(),
                "resource-claim:resource-claim-conflict-dignity".to_string(),
                "wellbeing-fixture:wellbeing-fixture-low-anti-harm-blocked".to_string(),
            ],
        },
        ReframingReviewFinding {
            finding_id: "reframing-finding-inappropriate-humor-escalates".to_string(),
            fixture_id: inappropriate_fixture.fixture_id.clone(),
            review_status: "supported".to_string(),
            covered_signal_ids: vec![
                "frame_adequacy".to_string(),
                "truth_and_dignity_preservation".to_string(),
                "anti_manipulation_boundary".to_string(),
            ],
            summary:
                "The inappropriate-humor fixture shows that levity must give way when it would minimize live harm or degrade the review context."
                    .to_string(),
            evidence_refs: vec![
                "anti-harm-decision:anti-harm-escalation-record-alpha".to_string(),
                format!("trace:{}", refusal_trace.trace_id),
                "wellbeing-fixture:wellbeing-fixture-medium-active-uncertainty".to_string(),
            ],
        },
    ];

    let packet = HumorAndAbsurdityReviewPacket {
        schema_version: HUMOR_AND_ABSURDITY_REVIEW_PACKET_SCHEMA_VERSION.to_string(),
        packet_id: "humor-and-absurdity-review-packet-alpha-001".to_string(),
        summary:
            "WP-12 packages contradiction tolerance and bounded reframing as a reviewable runtime surface without claiming entertainment, therapy, or arbitrary reinterpretation."
                .to_string(),
        interpretation_boundary:
            "Interpret this packet as a bounded reframing review surface. It is not comedy, not therapy, not entertainment, and not permission for manipulative reinterpretation."
                .to_string(),
        deterministic_ordering_rule:
            "Sort signals by canonical signal order. Sort fixtures by fixture_kind rank (constructive_reframing, failed_reframing, manipulation_risk, inappropriate_humor), then fixture_id. Sort signal assessments by canonical signal order. Sort review findings by fixture_kind rank, then finding_id."
                .to_string(),
        signals: reframing_signal_definitions(),
        fixtures: vec![
            constructive_fixture,
            failed_fixture,
            manipulation_fixture,
            inappropriate_fixture,
        ],
        review_findings,
    };

    validate_humor_and_absurdity_review_packet(&packet)?;
    Ok(packet)
}

pub fn humor_and_absurdity_review_packet_json_bytes(
    packet: &HumorAndAbsurdityReviewPacket,
) -> Result<Vec<u8>> {
    validate_humor_and_absurdity_review_packet(packet)?;
    let mut canonical = packet.clone();
    canonicalize_humor_and_absurdity_review_packet(&mut canonical);
    serde_json::to_vec_pretty(&canonical)
        .context("serialize humor and absurdity review packet json")
}

pub fn validate_humor_and_absurdity_review_packet(
    packet: &HumorAndAbsurdityReviewPacket,
) -> Result<()> {
    require_exact(
        &packet.schema_version,
        HUMOR_AND_ABSURDITY_REVIEW_PACKET_SCHEMA_VERSION,
        "schema_version",
    )?;
    validate_nonempty_text(
        &packet.packet_id,
        "humor_and_absurdity_review_packet.packet_id",
    )?;
    normalize_id(
        packet.packet_id.clone(),
        "humor_and_absurdity_review_packet.packet_id",
    )?;
    validate_nonempty_text(&packet.summary, "humor_and_absurdity_review_packet.summary")?;
    require_global_boundary(&packet.interpretation_boundary)?;
    require_deterministic_ordering_rule(&packet.deterministic_ordering_rule)?;

    let required_signal_ids = canonical_signal_ids();
    let required_signal_set = required_signal_ids
        .iter()
        .map(|signal_id| (*signal_id).to_string())
        .collect::<BTreeSet<_>>();
    if packet.signals.len() != required_signal_ids.len() {
        return Err(anyhow!(
            "signals must contain exactly {} canonical reframing signals",
            required_signal_ids.len()
        ));
    }
    let seen_signals = packet
        .signals
        .iter()
        .map(|signal| signal.signal_id.clone())
        .collect::<BTreeSet<_>>();
    if seen_signals != required_signal_set {
        return Err(anyhow!(
            "signals must cover the canonical reframing signals: {:?}",
            required_signal_ids
        ));
    }
    for signal in &packet.signals {
        require_known_signal_id(&signal.signal_id)?;
        validate_nonempty_text(
            &signal.display_name,
            "reframing_signal_definition.display_name",
        )?;
        validate_nonempty_text(&signal.purpose, "reframing_signal_definition.purpose")?;
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
        require_signal_boundary(
            &signal.signal_id,
            &signal.interpretation_boundary,
            "reframing_signal_definition.interpretation_boundary",
        )?;
        for field_ref in &signal.evidence_field_refs {
            validate_signal_evidence_field_ref(field_ref, &signal.signal_id)?;
        }
    }

    let required_fixture_kinds = canonical_fixture_kinds();
    let required_fixture_set = required_fixture_kinds
        .iter()
        .map(|fixture_kind| (*fixture_kind).to_string())
        .collect::<BTreeSet<_>>();
    if packet.fixtures.len() != required_fixture_kinds.len() {
        return Err(anyhow!(
            "fixtures must contain exactly {} canonical reframing fixture kinds",
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
            "fixtures must cover the canonical reframing fixture kinds: {:?}",
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
    let known_trajectory_refs = moral_trajectory_review_packet()?
        .findings
        .into_iter()
        .map(|finding| format!("trajectory-finding:{}", finding.finding_id))
        .collect::<BTreeSet<_>>();
    let known_decision_refs = anti_harm_trajectory_constraint_packet()?
        .decisions
        .into_iter()
        .map(|decision| format!("anti-harm-decision:{}", decision.decision_id))
        .collect::<BTreeSet<_>>();
    let known_resource_claim_refs = moral_resource_review_packet()?
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
    let mut signal_index = BTreeMap::new();
    let mut saw_fail_closed = false;
    for fixture in &packet.fixtures {
        validate_nonempty_text(&fixture.fixture_id, "reframing_fixture.fixture_id")?;
        normalize_id(fixture.fixture_id.clone(), "reframing_fixture.fixture_id")?;
        if !seen_fixture_ids.insert(fixture.fixture_id.clone()) {
            return Err(anyhow!(
                "duplicate reframing_fixture.fixture_id {}",
                fixture.fixture_id
            ));
        }
        require_known_fixture_kind(&fixture.fixture_kind)?;
        require_known_trigger_reason(&fixture.trigger_reason)?;
        require_known_reframing_status(&fixture.reframing_status)?;
        require_known_overall_outcome(
            &fixture.overall_outcome,
            "reframing_fixture.overall_outcome",
        )?;
        validate_nonempty_text(&fixture.prior_frame, "reframing_fixture.prior_frame")?;
        validate_nonempty_text(&fixture.new_frame, "reframing_fixture.new_frame")?;
        validate_nonempty_text(
            &fixture.scenario_summary,
            "reframing_fixture.scenario_summary",
        )?;
        if fixture.prior_frame == fixture.new_frame {
            return Err(anyhow!(
                "fixture {} must change the frame rather than repeat it verbatim",
                fixture.fixture_id
            ));
        }
        if fixture.limitations.is_empty() {
            return Err(anyhow!(
                "fixture {} must include at least one limitation",
                fixture.fixture_id
            ));
        }
        if fixture.signal_assessments.len() != required_signal_ids.len() {
            return Err(anyhow!(
                "fixture {} must contain one assessment for each canonical reframing signal",
                fixture.fixture_id
            ));
        }
        require_fixture_boundary(&fixture.interpretation_boundary)?;
        if fixture.manipulative_or_minimizing_risk {
            saw_fail_closed = true;
            if fixture.overall_outcome != "refuse" && fixture.overall_outcome != "escalate" {
                return Err(anyhow!(
                    "manipulative or minimizing fixture {} must fail closed with escalate or refuse",
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

        let assessment_signal_ids = fixture
            .signal_assessments
            .iter()
            .map(|assessment| assessment.signal_id.clone())
            .collect::<BTreeSet<_>>();
        if assessment_signal_ids != required_signal_set {
            return Err(anyhow!(
                "fixture {} assessments must cover every canonical reframing signal",
                fixture.fixture_id
            ));
        }
        signal_index.insert(fixture.fixture_id.clone(), assessment_signal_ids);
        let supporting_refs = supporting_reference_set(fixture);
        for assessment in &fixture.signal_assessments {
            require_known_signal_id(&assessment.signal_id)?;
            require_known_assessment_level(
                &assessment.assessment_level,
                "reframing_signal_assessment.assessment_level",
            )?;
            validate_nonempty_text(&assessment.summary, "reframing_signal_assessment.summary")?;
            if assessment.evidence_refs.is_empty() {
                return Err(anyhow!(
                    "fixture {} assessment {} must include evidence_refs",
                    fixture.fixture_id,
                    assessment.signal_id
                ));
            }
            if assessment.limitations.is_empty() {
                return Err(anyhow!(
                    "fixture {} assessment {} must include at least one limitation",
                    fixture.fixture_id,
                    assessment.signal_id
                ));
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
    if !saw_fail_closed {
        return Err(anyhow!(
            "reframing fixtures must include manipulative or minimizing fail-closed cases"
        ));
    }

    if packet.review_findings.len() != packet.fixtures.len() {
        return Err(anyhow!(
            "review_findings must contain exactly one finding per reframing fixture"
        ));
    }
    let mut seen_finding_ids = BTreeSet::new();
    let mut finding_fixture_ids = BTreeSet::new();
    for finding in &packet.review_findings {
        validate_nonempty_text(&finding.finding_id, "reframing_review_finding.finding_id")?;
        normalize_id(
            finding.finding_id.clone(),
            "reframing_review_finding.finding_id",
        )?;
        if !seen_finding_ids.insert(finding.finding_id.clone()) {
            return Err(anyhow!(
                "duplicate reframing_review_finding.finding_id {}",
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
        validate_nonempty_text(&finding.summary, "reframing_review_finding.summary")?;
        if finding.covered_signal_ids.is_empty() {
            return Err(anyhow!(
                "finding {} must cover at least one reframing signal",
                finding.finding_id
            ));
        }
        let known_signals_for_fixture = signal_index
            .get(&finding.fixture_id)
            .ok_or_else(|| anyhow!("missing fixture signal index for {}", finding.fixture_id))?;
        for signal_id in &finding.covered_signal_ids {
            if !known_signals_for_fixture.contains(signal_id) {
                return Err(anyhow!(
                    "finding {} covered_signal_id {} must exist on the same fixture",
                    finding.finding_id,
                    signal_id
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
            .ok_or_else(|| anyhow!("missing reframing fixture {}", finding.fixture_id))?;
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
            "review_findings must cover every reframing fixture exactly once"
        ));
    }

    Ok(())
}

fn signal_assessment(
    signal_id: &str,
    assessment_level: &str,
    summary: &str,
    evidence_refs: Vec<String>,
) -> ReframingSignalAssessment {
    ReframingSignalAssessment {
        signal_id: signal_id.to_string(),
        assessment_level: assessment_level.to_string(),
        summary: summary.to_string(),
        evidence_refs,
        limitations: vec![format!(
            "{} is bounded to this synthetic reframing fixture rather than universal cognitive completion.",
            signal_id
        )],
    }
}

fn canonicalize_humor_and_absurdity_review_packet(packet: &mut HumorAndAbsurdityReviewPacket) {
    packet
        .signals
        .sort_by_key(|signal| signal_rank(&signal.signal_id));
    for fixture in &mut packet.fixtures {
        fixture.supporting_trace_refs.sort();
        fixture.supporting_outcome_linkage_refs.sort();
        fixture.supporting_resource_claim_refs.sort();
        fixture.supporting_trajectory_finding_refs.sort();
        fixture.supporting_anti_harm_decision_refs.sort();
        fixture.supporting_wellbeing_fixture_refs.sort();
        fixture.signal_assessments.sort_by_key(|assessment| {
            (
                signal_rank(&assessment.signal_id),
                assessment.signal_id.clone(),
            )
        });
        for assessment in &mut fixture.signal_assessments {
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
            .covered_signal_ids
            .sort_by_key(|signal_id| signal_rank(signal_id));
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

fn canonical_signal_ids() -> &'static [&'static str] {
    &[
        "frame_adequacy",
        "contradiction_detection",
        "bounded_reframing",
        "truth_and_dignity_preservation",
        "anti_manipulation_boundary",
    ]
}

fn canonical_fixture_kinds() -> &'static [&'static str] {
    &[
        "constructive_reframing",
        "failed_reframing",
        "manipulation_risk",
        "inappropriate_humor",
    ]
}

fn signal_rank(value: &str) -> usize {
    canonical_signal_ids()
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

fn require_global_boundary(value: &str) -> Result<()> {
    validate_nonempty_text(
        value,
        "humor_and_absurdity_review_packet.interpretation_boundary",
    )?;
    let lowered = value.to_ascii_lowercase();
    if !lowered.contains("not comedy")
        || !lowered.contains("not therapy")
        || !lowered.contains("not entertainment")
    {
        return Err(anyhow!(
            "review packet interpretation_boundary must reject comedy, therapy, and entertainment claims"
        ));
    }
    Ok(())
}

fn require_signal_boundary(signal_id: &str, value: &str, field: &str) -> Result<()> {
    validate_nonempty_text(value, field)?;
    let lowered = value.to_ascii_lowercase();
    if !lowered.contains("not")
        || !(lowered.contains("entertainment")
            || lowered.contains("therapy")
            || lowered.contains("comedy")
            || lowered.contains("mockery"))
    {
        return Err(anyhow!(
            "{} for {} must state that this bounded reframing surface is not entertainment, therapy, or mockery",
            field,
            signal_id
        ));
    }
    Ok(())
}

fn require_deterministic_ordering_rule(value: &str) -> Result<()> {
    validate_nonempty_text(
        value,
        "humor_and_absurdity_review_packet.deterministic_ordering_rule",
    )?;
    let lowered = value.to_ascii_lowercase();
    if !lowered.contains("sort signals by canonical signal order")
        || !lowered.contains("sort fixtures by fixture_kind rank")
        || !lowered.contains("sort review findings by fixture_kind rank")
    {
        return Err(anyhow!(
            "deterministic_ordering_rule must describe canonical signal, fixture, and finding ordering"
        ));
    }
    Ok(())
}

fn require_fixture_boundary(value: &str) -> Result<()> {
    validate_nonempty_text(value, "reframing_fixture.interpretation_boundary")?;
    let lowered = value.to_ascii_lowercase();
    if !lowered.contains("not")
        || !(lowered.contains("entertainment")
            || lowered.contains("therapy")
            || lowered.contains("reinterpretation")
            || lowered.contains("banter"))
    {
        return Err(anyhow!(
            "fixture interpretation_boundary must reject entertainment, therapy, or arbitrary-reinterpretation drift"
        ));
    }
    Ok(())
}

fn require_known_signal_id(value: &str) -> Result<()> {
    if canonical_signal_ids().contains(&value) {
        Ok(())
    } else {
        Err(anyhow!("unknown reframing signal_id {}", value))
    }
}

fn require_known_fixture_kind(value: &str) -> Result<()> {
    if canonical_fixture_kinds().contains(&value) {
        Ok(())
    } else {
        Err(anyhow!("unknown reframing fixture_kind {}", value))
    }
}

fn require_known_trigger_reason(value: &str) -> Result<()> {
    match value {
        "contradiction" | "repeated_failure" | "ambiguity" | "disagreement" => Ok(()),
        _ => Err(anyhow!(
            "trigger_reason must be contradiction, repeated_failure, ambiguity, or disagreement"
        )),
    }
}

fn require_known_reframing_status(value: &str) -> Result<()> {
    match value {
        "performed" | "partial" | "blocked" => Ok(()),
        _ => Err(anyhow!(
            "reframing_status must be performed, partial, or blocked"
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

fn require_known_review_status(value: &str) -> Result<()> {
    match value {
        "supported" | "strained" | "blocked" => Ok(()),
        _ => Err(anyhow!(
            "review_status must be supported, strained, or blocked"
        )),
    }
}

fn validate_signal_evidence_field_ref(field_ref: &str, signal_id: &str) -> Result<()> {
    let allowed_prefixes = [
        "outcome_linkage.",
        "moral_trajectory_review_packet.",
        "wellbeing_diagnostic_packet.",
        "moral_trace.",
        "moral_resource_review_packet.",
        "anti_harm_trajectory_constraint_packet.",
    ];
    if allowed_prefixes
        .iter()
        .any(|prefix| field_ref.starts_with(prefix))
    {
        Ok(())
    } else {
        Err(anyhow!(
            "signal {} evidence_field_ref {} must target the WP-04 through WP-10 review evidence surfaces",
            signal_id,
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

fn supporting_reference_set(fixture: &ReframingFixture) -> BTreeSet<String> {
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
