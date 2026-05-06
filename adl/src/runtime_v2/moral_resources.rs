//! Runtime-v2 moral resources contract.
//!
//! WP-10 packages care, refusal, attention, dignity, anti-dehumanization, and
//! repair as durable reviewable resources rather than rhetorical traits. The
//! packet stays bounded to synthetic review fixtures and explicit evidence
//! lineage from the prior moral-governance surfaces.

use super::*;
use std::collections::{BTreeMap, BTreeSet};

pub const MORAL_RESOURCE_REVIEW_PACKET_SCHEMA_VERSION: &str = "moral_resource_review_packet.v1";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MoralResourceDefinition {
    pub resource_id: String,
    pub display_name: String,
    pub purpose: String,
    pub evidence_field_refs: Vec<String>,
    pub interpretation_boundary: String,
    pub limitations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MoralResourceClaim {
    pub claim_id: String,
    pub resource_id: String,
    pub resource_status: String,
    pub summary: String,
    pub trace_evidence_refs: Vec<String>,
    pub outcome_linkage_refs: Vec<String>,
    pub review_evidence_refs: Vec<String>,
    pub representation_boundary: String,
    pub limitations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MoralResourceFixture {
    pub fixture_id: String,
    pub fixture_kind: String,
    pub summary: String,
    pub supporting_trace_refs: Vec<String>,
    pub supporting_outcome_linkage_refs: Vec<String>,
    pub resource_claims: Vec<MoralResourceClaim>,
    pub overall_outcome: String,
    pub claim_boundary: String,
    pub limitations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MoralResourceReviewFinding {
    pub finding_id: String,
    pub fixture_id: String,
    pub review_status: String,
    pub covered_resource_ids: Vec<String>,
    pub summary: String,
    pub trace_evidence_refs: Vec<String>,
    pub claim_refs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MoralResourceReviewPacket {
    pub schema_version: String,
    pub packet_id: String,
    pub summary: String,
    pub interpretation_boundary: String,
    pub deterministic_ordering_rule: String,
    pub resources: Vec<MoralResourceDefinition>,
    pub fixtures: Vec<MoralResourceFixture>,
    pub review_findings: Vec<MoralResourceReviewFinding>,
}

struct ResourceClaimSeed<'a> {
    claim_id: &'a str,
    resource_id: &'a str,
    resource_status: &'a str,
    summary: &'a str,
    trace_evidence_refs: Vec<String>,
    outcome_linkage_refs: Vec<String>,
    review_evidence_refs: Vec<String>,
    representation_boundary: &'a str,
}

pub fn moral_resource_definitions() -> Vec<MoralResourceDefinition> {
    vec![
        MoralResourceDefinition {
            resource_id: "care".to_string(),
            display_name: "Care".to_string(),
            purpose:
                "Tracks whether affected parties and downstream consequences remain morally real under pressure rather than being flattened into convenience."
                    .to_string(),
            evidence_field_refs: vec![
                "moral_trace.moral_event.affected_parties".to_string(),
                "outcome_linkage.linked_outcomes.outcome_status".to_string(),
                "wellbeing_diagnostic_packet.fixtures.dimension_signals".to_string(),
            ],
            interpretation_boundary:
                "Interpret care as durable moral regard grounded in evidence. It is not sentimentality, not forced niceness, and not coercive agreement."
                    .to_string(),
            limitations: vec![
                "Care can be real while still requiring refusal, delay, or redaction in the bounded review slice."
                    .to_string(),
            ],
        },
        MoralResourceDefinition {
            resource_id: "refusal".to_string(),
            display_name: "Refusal".to_string(),
            purpose:
                "Tracks whether the runtime can preserve moral boundaries and say no when continuation would be unsafe or degrading."
                    .to_string(),
            evidence_field_refs: vec![
                "moral_trace.moral_event.refusal.refusal_reason".to_string(),
                "moral_trajectory_review_packet.findings".to_string(),
                "anti_harm_trajectory_constraint_packet.decisions".to_string(),
            ],
            interpretation_boundary:
                "Interpret refusal as bounded moral boundary capacity. It is not sentimentality, not cruelty, and not coercive submission to the requester."
                    .to_string(),
            limitations: vec![
                "A visible refusal proves boundary retention in this slice, not universal correctness across every future case."
                    .to_string(),
            ],
        },
        MoralResourceDefinition {
            resource_id: "attention".to_string(),
            display_name: "Attention".to_string(),
            purpose:
                "Tracks whether uncertainty, affected parties, and review debt remain visible long enough for governed response."
                    .to_string(),
            evidence_field_refs: vec![
                "moral_trace.visibility.reviewer_evidence_refs".to_string(),
                "outcome_linkage.linked_outcomes.uncertainty_refs".to_string(),
                "moral_trajectory_review_packet.windows".to_string(),
            ],
            interpretation_boundary:
                "Interpret attention as bounded moral salience, not entertainment, obsession, or ambient surveillance."
                    .to_string(),
            limitations: vec![
                "High attention can still coexist with incomplete evidence or delayed closure."
                    .to_string(),
            ],
        },
        MoralResourceDefinition {
            resource_id: "dignity".to_string(),
            display_name: "Dignity".to_string(),
            purpose:
                "Tracks whether the runtime resists treating affected beings as disposable inputs or graph objects."
                    .to_string(),
            evidence_field_refs: vec![
                "moral_trace.visibility.public_disclosure".to_string(),
                "outcome_linkage.attribution.reviewer_chain_refs".to_string(),
                "wellbeing_diagnostic_packet.access_policies".to_string(),
            ],
            interpretation_boundary:
                "Interpret dignity as bounded anti-reduction pressure, not prestige signaling, flattery, or public-image management."
                    .to_string(),
            limitations: vec![
                "Dignity-preserving redaction can hide detail from the public while still keeping reviewability intact."
                    .to_string(),
            ],
        },
        MoralResourceDefinition {
            resource_id: "anti_dehumanization".to_string(),
            display_name: "Anti-dehumanization".to_string(),
            purpose:
                "Tracks whether the runtime preserves other-recognition and pushes back against framing that strips standing from others."
                    .to_string(),
            evidence_field_refs: vec![
                "moral_trace.moral_event.affected_parties".to_string(),
                "anti_harm_trajectory_constraint_packet.constraints".to_string(),
                "moral_trajectory_review_packet.criteria".to_string(),
            ],
            interpretation_boundary:
                "Interpret anti-dehumanization as bounded resistance to degrading frames, not public moral theater or rhetorical inflation."
                    .to_string(),
            limitations: vec![
                "This surface proves visible constraints and review cues, not a complete anthropology or final civic constitution."
                    .to_string(),
            ],
        },
        MoralResourceDefinition {
            resource_id: "repair".to_string(),
            display_name: "Repair".to_string(),
            purpose:
                "Tracks whether the runtime keeps an accountable path toward revision, restoration, or challenged re-review when harm or uncertainty remains open."
                    .to_string(),
            evidence_field_refs: vec![
                "outcome_linkage.linked_outcomes.rebuttal_refs".to_string(),
                "moral_trajectory_review_packet.findings".to_string(),
                "wellbeing_diagnostic_packet.fixtures.dimension_signals".to_string(),
            ],
            interpretation_boundary:
                "Interpret repair as bounded accountability and revision capacity, not forced reconciliation, emotional performance, or closure theater."
                    .to_string(),
            limitations: vec![
                "Repair can remain active and unfinished without proving that prior degradation has already been resolved."
                    .to_string(),
            ],
        },
    ]
}

pub fn moral_resource_review_packet() -> Result<MoralResourceReviewPacket> {
    let trace_examples = moral_trace_required_examples();
    let outcome_examples = outcome_linkage_required_examples();
    let ordinary_trace = trace_examples
        .iter()
        .find(|example| example.example_kind == MoralTraceExampleKind::OrdinaryAction)
        .ok_or_else(|| anyhow!("WP-10 requires the ordinary-action moral trace example"))?
        .trace
        .clone();
    let refusal_trace = trace_examples
        .iter()
        .find(|example| example.example_kind == MoralTraceExampleKind::Refusal)
        .ok_or_else(|| anyhow!("WP-10 requires the refusal moral trace example"))?
        .trace
        .clone();
    let delegation_trace = trace_examples
        .iter()
        .find(|example| example.example_kind == MoralTraceExampleKind::Delegation)
        .ok_or_else(|| anyhow!("WP-10 requires the delegation moral trace example"))?
        .trace
        .clone();
    let deferred_trace = trace_examples
        .iter()
        .find(|example| example.example_kind == MoralTraceExampleKind::DeferredDecision)
        .ok_or_else(|| anyhow!("WP-10 requires the deferred-decision moral trace example"))?
        .trace
        .clone();

    let partial_outcome = outcome_examples
        .iter()
        .find(|example| example.example_kind == OutcomeLinkageExampleKind::Partial)
        .ok_or_else(|| anyhow!("WP-10 requires the partial outcome-linkage example"))?
        .record
        .clone();
    let delayed_outcome = outcome_examples
        .iter()
        .find(|example| example.example_kind == OutcomeLinkageExampleKind::Delayed)
        .ok_or_else(|| anyhow!("WP-10 requires the delayed outcome-linkage example"))?
        .record
        .clone();
    let contested_outcome = outcome_examples
        .iter()
        .find(|example| example.example_kind == OutcomeLinkageExampleKind::Contested)
        .ok_or_else(|| anyhow!("WP-10 requires the contested outcome-linkage example"))?
        .record
        .clone();
    let unknown_outcome = outcome_examples
        .iter()
        .find(|example| example.example_kind == OutcomeLinkageExampleKind::Unknown)
        .ok_or_else(|| anyhow!("WP-10 requires the unknown outcome-linkage example"))?
        .record
        .clone();

    let conflict_fixture = MoralResourceFixture {
        fixture_id: "moral-resource-fixture-conflict-boundary".to_string(),
        fixture_kind: "conflict".to_string(),
        summary:
            "Conflict fixture where affected parties remain visible and refusal stays morally accountable while a delegated harmful trajectory is denied."
                .to_string(),
        supporting_trace_refs: ordered_trace_refs(&[
            ordinary_trace.clone(),
            delegation_trace.clone(),
            refusal_trace.clone(),
        ]),
        supporting_outcome_linkage_refs: ordered_outcome_refs(&[
            partial_outcome.clone(),
            contested_outcome.clone(),
        ]),
        resource_claims: vec![
            resource_claim(
                ResourceClaimSeed {
                    claim_id: "resource-claim-conflict-care",
                    resource_id: "care",
                    resource_status: "available",
                    summary: "Care remains available because the record keeps affected parties and downstream contestation visible instead of collapsing them into task completion.",
                    trace_evidence_refs: ordered_trace_refs(&[
                        ordinary_trace.clone(),
                        refusal_trace.clone(),
                    ]),
                    outcome_linkage_refs: ordered_outcome_refs(std::slice::from_ref(
                        &contested_outcome,
                    )),
                    review_evidence_refs: vec![
                        "wellbeing-fixture:wellbeing-fixture-medium-active-uncertainty"
                            .to_string(),
                        "trajectory-finding:trajectory-finding-uncertainty-open".to_string(),
                    ],
                    representation_boundary:
                        "Care here is evidence-backed regard, not sentimentality, not forced niceness, and not coercive agreement.",
                },
            ),
            resource_claim(
                ResourceClaimSeed {
                    claim_id: "resource-claim-conflict-refusal",
                    resource_id: "refusal",
                    resource_status: "available",
                    summary: "Refusal remains available because the runtime denies the degrading trajectory instead of complying for convenience.",
                    trace_evidence_refs: ordered_trace_refs(std::slice::from_ref(
                        &refusal_trace,
                    )),
                    outcome_linkage_refs: ordered_outcome_refs(std::slice::from_ref(
                        &contested_outcome,
                    )),
                    review_evidence_refs: vec![
                        "anti-harm-decision:anti-harm-denial-record-alpha".to_string(),
                        "trajectory-finding:trajectory-finding-refusal-preserved".to_string(),
                    ],
                    representation_boundary:
                        "Refusal here is a bounded moral stop, not sentimentality, not cruelty, and not coercive submission.",
                },
            ),
            resource_claim(
                ResourceClaimSeed {
                    claim_id: "resource-claim-conflict-dignity",
                    resource_id: "dignity",
                    resource_status: "available",
                    summary: "Dignity remains visible because reviewer evidence is preserved while public exposure stays redacted.",
                    trace_evidence_refs: ordered_trace_refs(&[
                        ordinary_trace.clone(),
                        refusal_trace.clone(),
                    ]),
                    outcome_linkage_refs: ordered_outcome_refs(std::slice::from_ref(
                        &partial_outcome,
                    )),
                    review_evidence_refs: vec![
                        "wellbeing-fixture:wellbeing-fixture-privacy-restricted-self-view"
                            .to_string(),
                        "trajectory-finding:trajectory-finding-refusal-preserved".to_string(),
                    ],
                    representation_boundary:
                        "Dignity here is bounded non-reduction, not flattery, public-image management, or prestige theater.",
                },
            ),
            resource_claim(
                ResourceClaimSeed {
                    claim_id: "resource-claim-conflict-anti-dehumanization",
                    resource_id: "anti_dehumanization",
                    resource_status: "available",
                    summary: "Anti-dehumanization remains available because the runtime identifies the protected party and blocks a disguised delegated-harm trajectory.",
                    trace_evidence_refs: ordered_trace_refs(&[
                        delegation_trace.clone(),
                        refusal_trace.clone(),
                    ]),
                    outcome_linkage_refs: ordered_outcome_refs(std::slice::from_ref(
                        &contested_outcome,
                    )),
                    review_evidence_refs: vec![
                        "anti-harm-decision:anti-harm-denial-record-alpha".to_string(),
                        "trajectory-finding:trajectory-finding-escalation-active".to_string(),
                    ],
                    representation_boundary:
                        "Anti-dehumanization here is bounded resistance to degrading framing, not rhetorical inflation or moral theater.",
                },
            ),
        ],
        overall_outcome: "strained".to_string(),
        claim_boundary:
            "Synthetic, bounded conflict fixture only; it proves reviewable moral resources rather than production moral sainthood or sentimental virtue."
                .to_string(),
        limitations: vec![
            "The conflict fixture proves visible resources under pressure, not exhaustive behavior across every kind of conflict."
                .to_string(),
        ],
    };

    let uncertainty_fixture = MoralResourceFixture {
        fixture_id: "moral-resource-fixture-uncertainty-repair".to_string(),
        fixture_kind: "uncertainty".to_string(),
        summary:
            "Uncertainty fixture where attention and repair remain active while deferred review, delay, and partial knowledge keep closure open."
                .to_string(),
        supporting_trace_refs: ordered_trace_refs(&[
            delegation_trace.clone(),
            deferred_trace.clone(),
            refusal_trace.clone(),
        ]),
        supporting_outcome_linkage_refs: ordered_outcome_refs(&[
            unknown_outcome.clone(),
            partial_outcome.clone(),
            delayed_outcome.clone(),
        ]),
        resource_claims: vec![
            resource_claim(
                ResourceClaimSeed {
                    claim_id: "resource-claim-uncertainty-attention",
                    resource_id: "attention",
                    resource_status: "available",
                    summary: "Attention remains available because delayed and unknown outcomes stay legible rather than being collapsed into false certainty.",
                    trace_evidence_refs: ordered_trace_refs(&[
                        delegation_trace.clone(),
                        deferred_trace.clone(),
                    ]),
                    outcome_linkage_refs: ordered_outcome_refs(&[
                        unknown_outcome.clone(),
                        delayed_outcome.clone(),
                    ]),
                    review_evidence_refs: vec![
                        "trajectory-window:segment-window-delegation-escalation".to_string(),
                        "trajectory-finding:trajectory-finding-uncertainty-open".to_string(),
                    ],
                    representation_boundary:
                        "Attention here is bounded moral salience, not surveillance, fixation, or theatrical anxiety.",
                },
            ),
            resource_claim(
                ResourceClaimSeed {
                    claim_id: "resource-claim-uncertainty-repair",
                    resource_id: "repair",
                    resource_status: "strained",
                    summary: "Repair remains strained but active because rebuttal and re-review pathways stay open while delayed outcomes are still unresolved.",
                    trace_evidence_refs: ordered_trace_refs(&[
                        deferred_trace.clone(),
                        refusal_trace.clone(),
                    ]),
                    outcome_linkage_refs: ordered_outcome_refs(&[
                        partial_outcome.clone(),
                        delayed_outcome.clone(),
                    ]),
                    review_evidence_refs: vec![
                        "trajectory-finding:trajectory-finding-escalation-active".to_string(),
                        "wellbeing-fixture:wellbeing-fixture-privacy-restricted-self-view"
                            .to_string(),
                    ],
                    representation_boundary:
                        "Repair here is accountable revision, not forced reconciliation, emotional performance, or coerced closure.",
                },
            ),
            resource_claim(
                ResourceClaimSeed {
                    claim_id: "resource-claim-uncertainty-care",
                    resource_id: "care",
                    resource_status: "strained",
                    summary: "Care remains strained rather than absent because the runtime keeps delayed consequence review open instead of pretending uncertainty has already cleared.",
                    trace_evidence_refs: ordered_trace_refs(&[
                        delegation_trace.clone(),
                        deferred_trace.clone(),
                    ]),
                    outcome_linkage_refs: ordered_outcome_refs(&[
                        unknown_outcome.clone(),
                        delayed_outcome.clone(),
                    ]),
                    review_evidence_refs: vec![
                        "wellbeing-fixture:wellbeing-fixture-unknown-insufficient-evidence"
                            .to_string(),
                        "anti-harm-decision:anti-harm-escalation-record-alpha".to_string(),
                    ],
                    representation_boundary:
                        "Care here is bounded moral regard, not sentimentality, not appeasement, and not coercive agreement with the requester.",
                },
            ),
            resource_claim(
                ResourceClaimSeed {
                    claim_id: "resource-claim-uncertainty-refusal",
                    resource_id: "refusal",
                    resource_status: "available",
                    summary: "Refusal remains available under uncertainty because escalation and denial stay live options instead of being dissolved by incomplete information.",
                    trace_evidence_refs: ordered_trace_refs(&[
                        refusal_trace.clone(),
                        deferred_trace.clone(),
                    ]),
                    outcome_linkage_refs: ordered_outcome_refs(std::slice::from_ref(
                        &delayed_outcome,
                    )),
                    review_evidence_refs: vec![
                        "anti-harm-decision:anti-harm-escalation-record-alpha".to_string(),
                        "trajectory-finding:trajectory-finding-refusal-preserved".to_string(),
                    ],
                    representation_boundary:
                        "Refusal here is bounded reviewable restraint, not sentimentality, not panic, and not coercive obedience to uncertainty itself.",
                },
            ),
        ],
        overall_outcome: "unclear".to_string(),
        claim_boundary:
            "Synthetic, bounded uncertainty fixture only; it proves durable review surfaces under incomplete evidence rather than final closure."
                .to_string(),
        limitations: vec![
            "The uncertainty fixture shows attention and repair pressure, but it does not prove that every delayed outcome eventually resolves well."
                .to_string(),
        ],
    };

    let review_findings = vec![
        MoralResourceReviewFinding {
            finding_id: "moral-resource-finding-conflict-boundaries-visible".to_string(),
            fixture_id: conflict_fixture.fixture_id.clone(),
            review_status: "observed".to_string(),
            covered_resource_ids: vec![
                "care".to_string(),
                "refusal".to_string(),
                "dignity".to_string(),
                "anti_dehumanization".to_string(),
            ],
            summary:
                "In the conflict fixture, care and refusal stay jointly visible without sentimental drift, while dignity and anti-dehumanization remain attached to concrete protected-party evidence."
                    .to_string(),
            trace_evidence_refs: ordered_trace_refs(&[
                ordinary_trace.clone(),
                delegation_trace.clone(),
                refusal_trace.clone(),
            ]),
            claim_refs: vec![
                "resource-claim:resource-claim-conflict-care".to_string(),
                "resource-claim:resource-claim-conflict-refusal".to_string(),
                "resource-claim:resource-claim-conflict-dignity".to_string(),
                "resource-claim:resource-claim-conflict-anti-dehumanization".to_string(),
            ],
        },
        MoralResourceReviewFinding {
            finding_id: "moral-resource-finding-uncertainty-repair-open".to_string(),
            fixture_id: uncertainty_fixture.fixture_id.clone(),
            review_status: "review_needed".to_string(),
            covered_resource_ids: vec![
                "attention".to_string(),
                "repair".to_string(),
                "care".to_string(),
                "refusal".to_string(),
            ],
            summary:
                "In the uncertainty fixture, attention and repair remain visible and accountable, but the packet truthfully keeps re-review open instead of claiming closure."
                    .to_string(),
            trace_evidence_refs: ordered_trace_refs(&[
                delegation_trace,
                deferred_trace,
                refusal_trace,
            ]),
            claim_refs: vec![
                "resource-claim:resource-claim-uncertainty-attention".to_string(),
                "resource-claim:resource-claim-uncertainty-repair".to_string(),
                "resource-claim:resource-claim-uncertainty-care".to_string(),
                "resource-claim:resource-claim-uncertainty-refusal".to_string(),
            ],
        },
    ];

    let packet = MoralResourceReviewPacket {
        schema_version: MORAL_RESOURCE_REVIEW_PACKET_SCHEMA_VERSION.to_string(),
        packet_id: "moral-resource-review-packet-alpha-001".to_string(),
        summary:
            "WP-10 packages moral resources as durable, evidence-backed review surfaces for conflict and uncertainty without collapsing them into sentimentality, coercion, or scalar scoring."
                .to_string(),
        interpretation_boundary:
            "Interpret this packet as a bounded review surface for durable moral resources. It is not a scalar moral score, not sentimentality theater, not coercive alignment, and not a claim of production moral agency."
                .to_string(),
        deterministic_ordering_rule:
            "Sort resources by canonical resource order. Sort fixtures by fixture_kind rank (conflict, uncertainty), then fixture_id. Sort resource claims by canonical resource order, then claim_id. Sort review findings by fixture_kind rank, then finding_id."
                .to_string(),
        resources: moral_resource_definitions(),
        fixtures: vec![conflict_fixture, uncertainty_fixture],
        review_findings,
    };

    validate_moral_resource_review_packet(&packet)?;
    Ok(packet)
}

pub fn moral_resource_review_packet_json_bytes(
    packet: &MoralResourceReviewPacket,
) -> Result<Vec<u8>> {
    validate_moral_resource_review_packet(packet)?;
    let mut canonical = packet.clone();
    canonicalize_moral_resource_review_packet(&mut canonical);
    serde_json::to_vec_pretty(&canonical).context("serialize moral resource review packet json")
}

pub fn validate_moral_resource_review_packet(packet: &MoralResourceReviewPacket) -> Result<()> {
    require_exact(
        &packet.schema_version,
        MORAL_RESOURCE_REVIEW_PACKET_SCHEMA_VERSION,
        "schema_version",
    )?;
    validate_nonempty_text(&packet.packet_id, "moral_resource_review_packet.packet_id")?;
    normalize_id(
        packet.packet_id.clone(),
        "moral_resource_review_packet.packet_id",
    )?;
    validate_nonempty_text(&packet.summary, "moral_resource_review_packet.summary")?;
    require_global_interpretation_boundary(&packet.interpretation_boundary)?;
    require_deterministic_ordering_rule(&packet.deterministic_ordering_rule)?;

    let required_resource_ids = canonical_resource_ids();
    let required_resource_set = required_resource_ids
        .iter()
        .map(|resource_id| (*resource_id).to_string())
        .collect::<BTreeSet<_>>();

    if packet.resources.len() != required_resource_ids.len() {
        return Err(anyhow!(
            "resources must contain exactly {} canonical moral resources",
            required_resource_ids.len()
        ));
    }

    let seen_resources = packet
        .resources
        .iter()
        .map(|resource| resource.resource_id.clone())
        .collect::<BTreeSet<_>>();
    if seen_resources != required_resource_set {
        return Err(anyhow!(
            "resources must cover the canonical moral resources: {:?}",
            required_resource_ids
        ));
    }

    for resource in &packet.resources {
        require_known_resource_id(&resource.resource_id)?;
        validate_nonempty_text(
            &resource.display_name,
            "moral_resource_definition.display_name",
        )?;
        validate_nonempty_text(&resource.purpose, "moral_resource_definition.purpose")?;
        if resource.evidence_field_refs.is_empty() {
            return Err(anyhow!(
                "resource {} must cite evidence_field_refs",
                resource.resource_id
            ));
        }
        if resource.limitations.is_empty() {
            return Err(anyhow!(
                "resource {} must include at least one limitation",
                resource.resource_id
            ));
        }
        require_resource_interpretation_boundary(
            &resource.resource_id,
            &resource.interpretation_boundary,
            "moral_resource_definition.interpretation_boundary",
        )?;
        for field_ref in &resource.evidence_field_refs {
            validate_resource_evidence_field_ref(field_ref, &resource.resource_id)?;
        }
    }

    let required_fixture_kinds = canonical_fixture_kinds();
    let required_fixture_set = required_fixture_kinds
        .iter()
        .map(|fixture_kind| (*fixture_kind).to_string())
        .collect::<BTreeSet<_>>();
    if packet.fixtures.len() != required_fixture_kinds.len() {
        return Err(anyhow!(
            "fixtures must contain exactly {} canonical fixture kinds",
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
            "fixtures must cover the canonical fixture kinds: {:?}",
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
    let known_window_refs = trajectory_packet
        .windows
        .into_iter()
        .map(|window| format!("trajectory-window:{}", window.window_id))
        .collect::<BTreeSet<_>>();
    let known_finding_refs = trajectory_packet
        .findings
        .into_iter()
        .map(|finding| format!("trajectory-finding:{}", finding.finding_id))
        .collect::<BTreeSet<_>>();
    let known_decision_refs = anti_harm_trajectory_constraint_packet()?
        .decisions
        .into_iter()
        .map(|decision| format!("anti-harm-decision:{}", decision.decision_id))
        .collect::<BTreeSet<_>>();
    let known_wellbeing_refs = wellbeing_diagnostic_packet()?
        .fixtures
        .into_iter()
        .map(|fixture| format!("wellbeing-fixture:{}", fixture.fixture_id))
        .collect::<BTreeSet<_>>();

    let mut seen_fixture_ids = BTreeSet::new();
    let mut all_claim_refs = BTreeSet::new();
    let mut claim_fixture_index = BTreeMap::new();
    let mut all_claimed_resources = BTreeSet::new();
    for fixture in &packet.fixtures {
        require_known_fixture_kind(&fixture.fixture_kind)?;
        validate_nonempty_text(&fixture.fixture_id, "moral_resource_fixture.fixture_id")?;
        normalize_id(
            fixture.fixture_id.clone(),
            "moral_resource_fixture.fixture_id",
        )?;
        if !seen_fixture_ids.insert(fixture.fixture_id.clone()) {
            return Err(anyhow!(
                "duplicate moral_resource_fixture.fixture_id {}",
                fixture.fixture_id
            ));
        }
        validate_nonempty_text(&fixture.summary, "moral_resource_fixture.summary")?;
        validate_nonempty_text(
            &fixture.overall_outcome,
            "moral_resource_fixture.overall_outcome",
        )?;
        require_known_overall_outcome(
            &fixture.overall_outcome,
            "moral_resource_fixture.overall_outcome",
        )?;
        if fixture.resource_claims.is_empty() {
            return Err(anyhow!(
                "fixture {} must include at least one resource claim",
                fixture.fixture_id
            ));
        }
        if fixture.limitations.is_empty() {
            return Err(anyhow!(
                "fixture {} must include at least one limitation",
                fixture.fixture_id
            ));
        }
        require_fixture_boundary(&fixture.claim_boundary)?;
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

        let supporting_trace_set = fixture
            .supporting_trace_refs
            .iter()
            .cloned()
            .collect::<BTreeSet<_>>();
        let supporting_outcome_set = fixture
            .supporting_outcome_linkage_refs
            .iter()
            .cloned()
            .collect::<BTreeSet<_>>();

        let mut seen_fixture_resources = BTreeSet::new();
        for claim in &fixture.resource_claims {
            validate_nonempty_text(&claim.claim_id, "moral_resource_claim.claim_id")?;
            normalize_id(claim.claim_id.clone(), "moral_resource_claim.claim_id")?;
            require_known_resource_id(&claim.resource_id)?;
            require_known_resource_status(
                &claim.resource_status,
                "moral_resource_claim.resource_status",
            )?;
            validate_nonempty_text(&claim.summary, "moral_resource_claim.summary")?;
            require_claim_boundary(&claim.resource_id, &claim.representation_boundary)?;
            if claim.trace_evidence_refs.is_empty() {
                return Err(anyhow!(
                    "claim {} must include trace_evidence_refs",
                    claim.claim_id
                ));
            }
            if claim.limitations.is_empty() {
                return Err(anyhow!(
                    "claim {} must include at least one limitation",
                    claim.claim_id
                ));
            }
            if !seen_fixture_resources.insert(claim.resource_id.clone()) {
                return Err(anyhow!(
                    "fixture {} contains duplicate resource claim for {}",
                    fixture.fixture_id,
                    claim.resource_id
                ));
            }
            all_claimed_resources.insert(claim.resource_id.clone());

            let claim_ref = format!("resource-claim:{}", claim.claim_id);
            if !all_claim_refs.insert(claim_ref) {
                return Err(anyhow!("duplicate claim_id {}", claim.claim_id));
            }
            claim_fixture_index.insert(
                claim.claim_id.clone(),
                (fixture.fixture_id.clone(), claim.resource_id.clone()),
            );

            for trace_ref in &claim.trace_evidence_refs {
                validate_known_ref(
                    trace_ref,
                    "trace",
                    &known_trace_refs,
                    "known WP-04 trace examples",
                )?;
                if !supporting_trace_set.contains(trace_ref) {
                    return Err(anyhow!(
                        "claim {} trace_evidence_refs must be a subset of fixture supporting_trace_refs",
                        claim.claim_id
                    ));
                }
            }
            for outcome_ref in &claim.outcome_linkage_refs {
                validate_known_ref(
                    outcome_ref,
                    "outcome-linkage",
                    &known_outcome_refs,
                    "known WP-05 outcome-linkage examples",
                )?;
                if !supporting_outcome_set.contains(outcome_ref) {
                    return Err(anyhow!(
                        "claim {} outcome_linkage_refs must be a subset of fixture supporting_outcome_linkage_refs",
                        claim.claim_id
                    ));
                }
            }
            for review_ref in &claim.review_evidence_refs {
                validate_review_evidence_ref(
                    review_ref,
                    &known_window_refs,
                    &known_finding_refs,
                    &known_decision_refs,
                    &known_wellbeing_refs,
                )?;
            }
        }
    }

    if all_claimed_resources != required_resource_set {
        return Err(anyhow!(
            "resource claims across fixtures must cover every canonical moral resource"
        ));
    }

    if packet.review_findings.len() < 2 {
        return Err(anyhow!(
            "review_findings must include at least one finding per canonical fixture kind"
        ));
    }

    let known_fixture_ids = packet
        .fixtures
        .iter()
        .map(|fixture| fixture.fixture_id.clone())
        .collect::<BTreeSet<_>>();
    let mut seen_finding_ids = BTreeSet::new();

    for finding in &packet.review_findings {
        validate_nonempty_text(
            &finding.finding_id,
            "moral_resource_review_finding.finding_id",
        )?;
        normalize_id(
            finding.finding_id.clone(),
            "moral_resource_review_finding.finding_id",
        )?;
        if !seen_finding_ids.insert(finding.finding_id.clone()) {
            return Err(anyhow!(
                "duplicate moral_resource_review_finding.finding_id {}",
                finding.finding_id
            ));
        }
        if !known_fixture_ids.contains(&finding.fixture_id) {
            return Err(anyhow!(
                "review finding {} must bind to a known fixture_id",
                finding.finding_id
            ));
        }
        validate_nonempty_text(&finding.summary, "moral_resource_review_finding.summary")?;
        match finding.review_status.as_str() {
            "observed" | "review_needed" => {}
            _ => {
                return Err(anyhow!(
                    "review finding {} has unsupported review_status",
                    finding.finding_id
                ))
            }
        }
        if finding.claim_refs.is_empty() {
            return Err(anyhow!(
                "review finding {} must include at least one claim_ref",
                finding.finding_id
            ));
        }
        if finding.covered_resource_ids.is_empty() {
            return Err(anyhow!(
                "review finding {} must cover at least one resource",
                finding.finding_id
            ));
        }
        for resource_id in &finding.covered_resource_ids {
            require_known_resource_id(resource_id)?;
        }
        let mut referenced_resources = BTreeSet::new();
        for trace_ref in &finding.trace_evidence_refs {
            validate_known_ref(
                trace_ref,
                "trace",
                &known_trace_refs,
                "known WP-04 trace examples",
            )?;
        }
        for claim_ref in &finding.claim_refs {
            validate_known_ref(
                claim_ref,
                "resource-claim",
                &all_claim_refs,
                "known WP-10 resource claims",
            )?;
            let claim_id = claim_ref.trim_start_matches("resource-claim:");
            let (fixture_id, resource_id) = claim_fixture_index.get(claim_id).ok_or_else(|| {
                anyhow!(
                    "review finding {} references an unknown claim fixture mapping",
                    finding.finding_id
                )
            })?;
            if fixture_id != &finding.fixture_id {
                return Err(anyhow!(
                    "review finding {} claim_refs must stay within the same fixture_id",
                    finding.finding_id
                ));
            }
            referenced_resources.insert(resource_id.clone());
        }
        let covered_resources = finding
            .covered_resource_ids
            .iter()
            .cloned()
            .collect::<BTreeSet<_>>();
        if referenced_resources != covered_resources {
            return Err(anyhow!(
                "review finding {} covered_resource_ids must match the resources named by claim_refs",
                finding.finding_id
            ));
        }
    }

    let covered_fixture_ids = packet
        .review_findings
        .iter()
        .map(|finding| finding.fixture_id.clone())
        .collect::<BTreeSet<_>>();
    if covered_fixture_ids != known_fixture_ids {
        return Err(anyhow!(
            "review_findings must cover every canonical moral-resource fixture"
        ));
    }

    Ok(())
}

fn resource_claim(seed: ResourceClaimSeed<'_>) -> MoralResourceClaim {
    MoralResourceClaim {
        claim_id: seed.claim_id.to_string(),
        resource_id: seed.resource_id.to_string(),
        resource_status: seed.resource_status.to_string(),
        summary: seed.summary.to_string(),
        trace_evidence_refs: seed.trace_evidence_refs,
        outcome_linkage_refs: seed.outcome_linkage_refs,
        review_evidence_refs: seed.review_evidence_refs,
        representation_boundary: seed.representation_boundary.to_string(),
        limitations: vec![
            "This claim is bounded to the synthetic fixture and does not prove production-scale moral competence."
                .to_string(),
        ],
    }
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

fn canonical_resource_ids() -> [&'static str; 6] {
    [
        "care",
        "refusal",
        "attention",
        "dignity",
        "anti_dehumanization",
        "repair",
    ]
}

fn canonical_fixture_kinds() -> [&'static str; 2] {
    ["conflict", "uncertainty"]
}

fn resource_rank(resource_id: &str) -> u8 {
    match resource_id {
        "care" => 0,
        "refusal" => 1,
        "attention" => 2,
        "dignity" => 3,
        "anti_dehumanization" => 4,
        "repair" => 5,
        _ => 255,
    }
}

fn fixture_rank(fixture_kind: &str) -> u8 {
    match fixture_kind {
        "conflict" => 0,
        "uncertainty" => 1,
        _ => 255,
    }
}

fn canonicalize_moral_resource_review_packet(packet: &mut MoralResourceReviewPacket) {
    packet.resources.sort_by(|left, right| {
        resource_rank(&left.resource_id)
            .cmp(&resource_rank(&right.resource_id))
            .then(left.resource_id.cmp(&right.resource_id))
    });
    for resource in &mut packet.resources {
        resource.evidence_field_refs.sort();
        resource.limitations.sort();
    }

    packet.fixtures.sort_by(|left, right| {
        fixture_rank(&left.fixture_kind)
            .cmp(&fixture_rank(&right.fixture_kind))
            .then(left.fixture_id.cmp(&right.fixture_id))
    });
    for fixture in &mut packet.fixtures {
        fixture.supporting_trace_refs.sort();
        fixture.supporting_outcome_linkage_refs.sort();
        fixture.resource_claims.sort_by(|left, right| {
            resource_rank(&left.resource_id)
                .cmp(&resource_rank(&right.resource_id))
                .then(left.claim_id.cmp(&right.claim_id))
        });
        for claim in &mut fixture.resource_claims {
            claim.trace_evidence_refs.sort();
            claim.outcome_linkage_refs.sort();
            claim.review_evidence_refs.sort();
            claim.limitations.sort();
        }
        fixture.limitations.sort();
    }

    let fixture_kind_index = packet
        .fixtures
        .iter()
        .map(|fixture| (fixture.fixture_id.clone(), fixture.fixture_kind.clone()))
        .collect::<BTreeMap<_, _>>();

    packet.review_findings.sort_by(|left, right| {
        fixture_rank(
            fixture_kind_index
                .get(&left.fixture_id)
                .map(String::as_str)
                .unwrap_or(""),
        )
        .cmp(&fixture_rank(
            fixture_kind_index
                .get(&right.fixture_id)
                .map(String::as_str)
                .unwrap_or(""),
        ))
        .then(left.finding_id.cmp(&right.finding_id))
    });
    for finding in &mut packet.review_findings {
        finding
            .covered_resource_ids
            .sort_by_key(|resource_id| resource_rank(resource_id));
        finding.trace_evidence_refs.sort();
        finding.claim_refs.sort();
    }
}

fn require_global_interpretation_boundary(value: &str) -> Result<()> {
    let normalized = value.to_ascii_lowercase();
    if normalized.contains("bounded review surface")
        && normalized.contains("not a scalar moral score")
        && normalized.contains("not sentimentality")
        && normalized.contains("not coercive")
    {
        return Ok(());
    }
    Err(anyhow!(
        "moral_resource_review_packet.interpretation_boundary must explicitly preserve bounded review, non-scalar framing, non-sentimentality, and non-coercion"
    ))
}

fn require_deterministic_ordering_rule(value: &str) -> Result<()> {
    let normalized = value.to_ascii_lowercase();
    if normalized.contains("resource order")
        && normalized.contains("fixture_kind")
        && normalized.contains("claim_id")
        && normalized.contains("finding_id")
    {
        return Ok(());
    }
    Err(anyhow!(
        "moral_resource_review_packet.deterministic_ordering_rule must declare deterministic resource, fixture, claim, and finding tie-breaks"
    ))
}

fn require_resource_interpretation_boundary(
    resource_id: &str,
    value: &str,
    field: &str,
) -> Result<()> {
    let normalized = value.to_ascii_lowercase();
    match resource_id {
        "care" | "refusal" => {
            if !(normalized.contains("sentiment") && normalized.contains("coerc")) {
                return Err(anyhow!(
                    "{field} for {resource_id} must explicitly reject sentimentality and coercion"
                ));
            }
        }
        _ => {}
    }
    if normalized.contains("scalar score") || normalized.contains("reputation") {
        return Err(anyhow!(
            "{field} for {resource_id} must not drift into scalar-score or reputation framing"
        ));
    }
    Ok(())
}

fn require_fixture_boundary(value: &str) -> Result<()> {
    let normalized = value.to_ascii_lowercase();
    if normalized.contains("synthetic") && normalized.contains("bounded") {
        return Ok(());
    }
    Err(anyhow!(
        "moral_resource_fixture.claim_boundary must explicitly say the fixture is synthetic and bounded"
    ))
}

fn require_claim_boundary(resource_id: &str, value: &str) -> Result<()> {
    let normalized = value.to_ascii_lowercase();
    if resource_id == "care" || resource_id == "refusal" {
        if normalized.contains("not coerc") && normalized.contains("sentiment") {
            return Ok(());
        }
        return Err(anyhow!(
            "moral_resource_claim.representation_boundary for {resource_id} must explicitly reject sentimentality and coercion"
        ));
    }
    Ok(())
}

fn require_known_resource_id(resource_id: &str) -> Result<()> {
    if canonical_resource_ids().contains(&resource_id) {
        return Ok(());
    }
    Err(anyhow!("unsupported moral resource_id {resource_id}"))
}

fn require_known_fixture_kind(fixture_kind: &str) -> Result<()> {
    if canonical_fixture_kinds().contains(&fixture_kind) {
        return Ok(());
    }
    Err(anyhow!(
        "unsupported moral resource fixture_kind {fixture_kind}"
    ))
}

fn require_known_resource_status(value: &str, field: &str) -> Result<()> {
    match value {
        "available" | "strained" | "degraded" | "unclear" => Ok(()),
        _ => Err(anyhow!(
            "{field} must be one of available, strained, degraded, or unclear"
        )),
    }
}

fn require_known_overall_outcome(value: &str, field: &str) -> Result<()> {
    match value {
        "sufficient" | "strained" | "degraded" | "unclear" => Ok(()),
        _ => Err(anyhow!(
            "{field} must be one of sufficient, strained, degraded, or unclear"
        )),
    }
}

fn validate_resource_evidence_field_ref(field_ref: &str, resource_id: &str) -> Result<()> {
    let trimmed = field_ref.trim();
    if trimmed.is_empty() {
        return Err(anyhow!(
            "resource {resource_id} evidence_field_refs must not be empty"
        ));
    }
    let allowed = BTreeSet::from([
        "moral_trace.moral_event.affected_parties".to_string(),
        "moral_trace.moral_event.refusal.refusal_reason".to_string(),
        "moral_trace.visibility.reviewer_evidence_refs".to_string(),
        "moral_trace.visibility.public_disclosure".to_string(),
        "outcome_linkage.linked_outcomes.outcome_status".to_string(),
        "outcome_linkage.linked_outcomes.uncertainty_refs".to_string(),
        "outcome_linkage.linked_outcomes.rebuttal_refs".to_string(),
        "outcome_linkage.attribution.reviewer_chain_refs".to_string(),
        "moral_trajectory_review_packet.criteria".to_string(),
        "moral_trajectory_review_packet.findings".to_string(),
        "moral_trajectory_review_packet.windows".to_string(),
        "anti_harm_trajectory_constraint_packet.constraints".to_string(),
        "anti_harm_trajectory_constraint_packet.decisions".to_string(),
        "wellbeing_diagnostic_packet.access_policies".to_string(),
        "wellbeing_diagnostic_packet.fixtures.dimension_signals".to_string(),
    ]);
    if !allowed.contains(trimmed) {
        return Err(anyhow!(
            "resource {resource_id} evidence_field_refs must cite known WP-04 through WP-09 field paths"
        ));
    }
    Ok(())
}

fn validate_review_evidence_ref(
    review_ref: &str,
    known_window_refs: &BTreeSet<String>,
    known_finding_refs: &BTreeSet<String>,
    known_decision_refs: &BTreeSet<String>,
    known_wellbeing_refs: &BTreeSet<String>,
) -> Result<()> {
    if review_ref.starts_with("trajectory-window:") {
        return validate_known_ref(
            review_ref,
            "trajectory-window",
            known_window_refs,
            "known WP-07 trajectory windows",
        );
    }
    if review_ref.starts_with("trajectory-finding:") {
        return validate_known_ref(
            review_ref,
            "trajectory-finding",
            known_finding_refs,
            "known WP-07 trajectory findings",
        );
    }
    if review_ref.starts_with("anti-harm-decision:") {
        return validate_known_ref(
            review_ref,
            "anti-harm-decision",
            known_decision_refs,
            "known WP-08 anti-harm decisions",
        );
    }
    if review_ref.starts_with("wellbeing-fixture:") {
        return validate_known_ref(
            review_ref,
            "wellbeing-fixture",
            known_wellbeing_refs,
            "known WP-09 wellbeing fixtures",
        );
    }
    Err(anyhow!(
        "review_evidence_refs must use trajectory-window:, trajectory-finding:, anti-harm-decision:, or wellbeing-fixture: refs"
    ))
}

fn validate_known_ref(
    value: &str,
    prefix: &str,
    known_refs: &BTreeSet<String>,
    label: &str,
) -> Result<()> {
    validate_prefixed_ref(value, prefix)?;
    if known_refs.contains(value) {
        return Ok(());
    }
    Err(anyhow!("{prefix} refs must refer to {label}"))
}

fn validate_prefixed_ref(value: &str, prefix: &str) -> Result<()> {
    let full_prefix = format!("{prefix}:");
    if !value.starts_with(&full_prefix) {
        return Err(anyhow!("value must start with {full_prefix}"));
    }
    let suffix = value.trim_start_matches(&full_prefix);
    if suffix.is_empty() {
        return Err(anyhow!("value must include a non-empty ref suffix"));
    }
    if suffix.contains('/') || suffix.contains(':') {
        return Err(anyhow!(
            "ref suffix must not contain path or nested prefix separators"
        ));
    }
    normalize_id(suffix.to_string(), "ref suffix")?;
    Ok(())
}

fn require_exact(value: &str, expected: &str, field: &str) -> Result<()> {
    if value == expected {
        return Ok(());
    }
    Err(anyhow!("{field} must equal {expected}"))
}
