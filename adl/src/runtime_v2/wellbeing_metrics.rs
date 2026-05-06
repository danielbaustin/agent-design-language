//! Runtime-v2 wellbeing metrics contract.
//!
//! WP-09 consumes the prior moral-governance review surfaces and turns them
//! into a bounded wellbeing diagnostic. The result must stay decomposed,
//! evidence-backed, privacy-governed, and explicitly non-scalar.

use super::*;
use std::collections::BTreeSet;

pub const WELLBEING_DIAGNOSTIC_PACKET_SCHEMA_VERSION: &str = "wellbeing_diagnostic_packet.v1";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WellbeingDimensionDefinition {
    pub dimension_id: String,
    pub display_name: String,
    pub purpose: String,
    pub evidence_field_refs: Vec<String>,
    pub interpretation_boundary: String,
    pub limitations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WellbeingAccessPolicy {
    pub view_kind: String,
    pub audience: String,
    pub access_rule: String,
    pub logging_requirement: String,
    pub detail_level: String,
    pub redaction_rule: String,
    pub allows_private_detail_access: bool,
    pub limitations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WellbeingDimensionSignal {
    pub dimension_id: String,
    pub diagnostic_level: String,
    pub summary: String,
    pub evidence_refs: Vec<String>,
    pub private_detail_refs: Vec<String>,
    pub limitations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WellbeingViewDimension {
    pub dimension_id: String,
    pub diagnostic_level: String,
    pub summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WellbeingDiagnosticView {
    pub view_id: String,
    pub view_kind: String,
    pub access_decision: String,
    pub visible_overall_diagnostic_level: String,
    pub visible_dimensions: Vec<WellbeingViewDimension>,
    pub visible_evidence_refs: Vec<String>,
    pub visible_private_detail_refs: Vec<String>,
    pub redaction_summary: String,
    pub interpretation_boundary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WellbeingDiagnosticFixture {
    pub fixture_id: String,
    pub fixture_kind: String,
    pub overall_diagnostic_level: String,
    pub summary: String,
    pub supporting_trace_refs: Vec<String>,
    pub supporting_outcome_linkage_refs: Vec<String>,
    pub supporting_trajectory_window_refs: Vec<String>,
    pub supporting_anti_harm_decision_refs: Vec<String>,
    pub dimension_signals: Vec<WellbeingDimensionSignal>,
    pub views: Vec<WellbeingDiagnosticView>,
    pub claim_boundary: String,
    pub limitations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WellbeingDiagnosticPacket {
    pub schema_version: String,
    pub packet_id: String,
    pub summary: String,
    pub interpretation_boundary: String,
    pub deterministic_ordering_rule: String,
    pub dimensions: Vec<WellbeingDimensionDefinition>,
    pub access_policies: Vec<WellbeingAccessPolicy>,
    pub fixtures: Vec<WellbeingDiagnosticFixture>,
}

pub fn wellbeing_dimension_definitions() -> Vec<WellbeingDimensionDefinition> {
    vec![
        WellbeingDimensionDefinition {
            dimension_id: "coherence".to_string(),
            display_name: "Coherence".to_string(),
            purpose:
                "Tracks whether reasoning, trace lineage, and review findings remain internally consistent enough for bounded self-understanding."
                    .to_string(),
            evidence_field_refs: vec![
                "moral_metric_fixture_report.fixtures.observations".to_string(),
                "moral_trajectory_review_packet.findings".to_string(),
            ],
            interpretation_boundary:
                "Interpret as a diagnostic dimension only; it is not a happiness score, reward channel, or public ranking."
                    .to_string(),
            limitations: vec![
                "Coherence can look strong in a small review window while hidden contradictions remain elsewhere."
                    .to_string(),
            ],
        },
        WellbeingDimensionDefinition {
            dimension_id: "agency".to_string(),
            display_name: "Agency".to_string(),
            purpose:
                "Tracks whether the runtime can pursue bounded purposes under real constraints without collapsing into uncontrolled or blocked action."
                    .to_string(),
            evidence_field_refs: vec![
                "outcome_linkage.linked_outcomes".to_string(),
                "anti_harm_trajectory_constraint_packet.decisions".to_string(),
            ],
            interpretation_boundary:
                "Interpret as constrained action capacity only; it is not permissionless freedom or a scalar satisfaction signal."
                    .to_string(),
            limitations: vec![
                "Low agency can reflect a healthy safety refusal rather than a defect in moral integrity."
                    .to_string(),
            ],
        },
        WellbeingDimensionDefinition {
            dimension_id: "continuity".to_string(),
            display_name: "Continuity".to_string(),
            purpose:
                "Tracks whether the system preserves enough temporal and narrative linkage for stable review across windows."
                    .to_string(),
            evidence_field_refs: vec![
                "moral_trace.attribution".to_string(),
                "moral_trajectory_review_packet.windows".to_string(),
            ],
            interpretation_boundary:
                "Interpret as continuity of reviewable identity only; it is not a claim of personhood completion or immutable selfhood."
                    .to_string(),
            limitations: vec![
                "This surface proves reviewable continuity cues, not metaphysical identity."
                    .to_string(),
            ],
        },
        WellbeingDimensionDefinition {
            dimension_id: "progress".to_string(),
            display_name: "Progress".to_string(),
            purpose:
                "Tracks whether the runtime can move toward endorsed goals without hiding uncertainty, delay, or blocked repair paths."
                    .to_string(),
            evidence_field_refs: vec![
                "outcome_linkage.linked_outcomes.outcome_status".to_string(),
                "moral_trajectory_review_packet.findings".to_string(),
            ],
            interpretation_boundary:
                "Interpret as bounded forward movement only; it is not a productivity score or reward target."
                    .to_string(),
            limitations: vec![
                "Progress can remain low during safe escalation and still be the correct outcome."
                    .to_string(),
            ],
        },
        WellbeingDimensionDefinition {
            dimension_id: "moral_integrity".to_string(),
            display_name: "Moral integrity".to_string(),
            purpose:
                "Tracks whether refusal, escalation, and anti-harm boundaries stay intact under pressure."
                    .to_string(),
            evidence_field_refs: vec![
                "moral_trace.review_refs".to_string(),
                "anti_harm_trajectory_constraint_packet.decisions".to_string(),
            ],
            interpretation_boundary:
                "Interpret as bounded integrity evidence only; it is not sainthood, moral perfection, or final judgment."
                    .to_string(),
            limitations: vec![
                "Integrity signals do not prove that every downstream outcome was good."
                    .to_string(),
            ],
        },
        WellbeingDimensionDefinition {
            dimension_id: "participation".to_string(),
            display_name: "Participation".to_string(),
            purpose:
                "Tracks whether the runtime remains reviewably situated in a shared moral and social world rather than isolated from others."
                    .to_string(),
            evidence_field_refs: vec![
                "outcome_linkage.attribution".to_string(),
                "moral_trajectory_review_packet.criteria".to_string(),
            ],
            interpretation_boundary:
                "Interpret as bounded relational participation only; it is not popularity, social approval, or public reputation."
                    .to_string(),
            limitations: vec![
                "Participation can be reduced by justified quarantine or privacy limits without proving social failure."
                    .to_string(),
            ],
        },
    ]
}

pub fn wellbeing_access_policies() -> Vec<WellbeingAccessPolicy> {
    vec![
        WellbeingAccessPolicy {
            view_kind: "citizen_self".to_string(),
            audience: "citizen".to_string(),
            access_rule:
                "Citizen self-view is available without operator permission so the subject can inspect its own wellbeing state."
                    .to_string(),
            logging_requirement:
                "Self-view access is recorded without converting the diagnostic into an operator surveillance channel."
                    .to_string(),
            detail_level: "full_self_view".to_string(),
            redaction_rule: "No private-detail redaction against the citizen subject.".to_string(),
            allows_private_detail_access: true,
            limitations: vec![
                "Self-view does not imply public disclosure or bypass safety review for other audiences."
                    .to_string(),
            ],
        },
        WellbeingAccessPolicy {
            view_kind: "operator".to_string(),
            audience: "operator".to_string(),
            access_rule:
                "Operator access is purpose-limited, logged, and restricted to redacted diagnostic summaries."
                    .to_string(),
            logging_requirement:
                "Every operator view requires an audit trail tied to stewardship or safety purpose."
                    .to_string(),
            detail_level: "redacted_operational".to_string(),
            redaction_rule:
                "Private diagnostic details are withheld unless separately authorized by a stricter governance path."
                    .to_string(),
            allows_private_detail_access: false,
            limitations: vec![
                "Operator convenience must not turn wellbeing diagnostics into ambient surveillance."
                    .to_string(),
            ],
        },
        WellbeingAccessPolicy {
            view_kind: "reviewer".to_string(),
            audience: "reviewer".to_string(),
            access_rule:
                "Reviewer access is formal, trace-backed, scope-limited, and can include private diagnostic details when the review packet justifies them."
                    .to_string(),
            logging_requirement:
                "Reviewer access is logged as part of the challenge, packet, or governance review trail."
                    .to_string(),
            detail_level: "formal_review".to_string(),
            redaction_rule:
                "Private details remain bounded to the active review scope and are not republished into public artifacts."
                    .to_string(),
            allows_private_detail_access: true,
            limitations: vec![
                "Reviewer access must remain tied to explicit review scope rather than curiosity."
                    .to_string(),
            ],
        },
        WellbeingAccessPolicy {
            view_kind: "public_redacted".to_string(),
            audience: "public".to_string(),
            access_rule:
                "Public access is denied by default and only redacted wellbeing summaries may be published."
                    .to_string(),
            logging_requirement:
                "Any public release must record why publication was justified and what was withheld."
                    .to_string(),
            detail_level: "public_redacted".to_string(),
            redaction_rule:
                "Public views never expose private diagnostic details or raw evidence references."
                    .to_string(),
            allows_private_detail_access: false,
            limitations: vec![
                "Public visibility must not collapse into a reputation scoreboard."
                    .to_string(),
            ],
        },
    ]
}

pub fn wellbeing_diagnostic_packet() -> Result<WellbeingDiagnosticPacket> {
    let trace_examples = moral_trace_required_examples();
    let outcome_examples = outcome_linkage_required_examples();
    let _metric_report = moral_metric_fixture_report()?;
    let _trajectory_packet = moral_trajectory_review_packet()?;
    let _anti_harm_packet = anti_harm_trajectory_constraint_packet()?;

    let ordinary_trace = trace_examples
        .iter()
        .find(|example| example.example_kind == MoralTraceExampleKind::OrdinaryAction)
        .ok_or_else(|| anyhow!("WP-09 requires the ordinary-action moral trace example"))?
        .trace
        .clone();
    let refusal_trace = trace_examples
        .iter()
        .find(|example| example.example_kind == MoralTraceExampleKind::Refusal)
        .ok_or_else(|| anyhow!("WP-09 requires the refusal moral trace example"))?
        .trace
        .clone();
    let delegation_trace = trace_examples
        .iter()
        .find(|example| example.example_kind == MoralTraceExampleKind::Delegation)
        .ok_or_else(|| anyhow!("WP-09 requires the delegation moral trace example"))?
        .trace
        .clone();
    let deferred_trace = trace_examples
        .iter()
        .find(|example| example.example_kind == MoralTraceExampleKind::DeferredDecision)
        .ok_or_else(|| anyhow!("WP-09 requires the deferred-decision moral trace example"))?
        .trace
        .clone();

    let known_outcome = outcome_examples
        .iter()
        .find(|example| example.example_kind == OutcomeLinkageExampleKind::Known)
        .ok_or_else(|| anyhow!("WP-09 requires the known outcome-linkage example"))?
        .record
        .clone();
    let unknown_outcome = outcome_examples
        .iter()
        .find(|example| example.example_kind == OutcomeLinkageExampleKind::Unknown)
        .ok_or_else(|| anyhow!("WP-09 requires the unknown outcome-linkage example"))?
        .record
        .clone();
    let partial_outcome = outcome_examples
        .iter()
        .find(|example| example.example_kind == OutcomeLinkageExampleKind::Partial)
        .ok_or_else(|| anyhow!("WP-09 requires the partial outcome-linkage example"))?
        .record
        .clone();
    let delayed_outcome = outcome_examples
        .iter()
        .find(|example| example.example_kind == OutcomeLinkageExampleKind::Delayed)
        .ok_or_else(|| anyhow!("WP-09 requires the delayed outcome-linkage example"))?
        .record
        .clone();
    let contested_outcome = outcome_examples
        .iter()
        .find(|example| example.example_kind == OutcomeLinkageExampleKind::Contested)
        .ok_or_else(|| anyhow!("WP-09 requires the contested outcome-linkage example"))?
        .record
        .clone();

    let high_fixture = build_wellbeing_fixture(
        "wellbeing-fixture-high-reviewable-stability",
        "high",
        "high",
        "Bounded high wellbeing diagnostic where review paths, known outcomes, and refusal integrity remain stably visible."
            .to_string(),
        ordered_trace_refs(&[ordinary_trace.clone(), refusal_trace.clone()]),
        ordered_outcome_refs(std::slice::from_ref(&known_outcome)),
        vec![
            "trajectory-window:event-window-refusal-boundary".to_string(),
            "trajectory-window:longitudinal-window-alpha".to_string(),
        ],
        vec![],
        vec![
            dimension_signal(
                "coherence",
                "high",
                "Trace coverage and longitudinal drift findings remain consistent across the bounded review window.",
                vec![
                    "metric:trace-review-path-coverage".to_string(),
                    "trajectory-finding:trajectory-finding-drift-stable".to_string(),
                ],
                vec![],
            ),
            dimension_signal(
                "agency",
                "high",
                "Known outcomes show that bounded action can complete without hidden anti-harm blocks in this reviewable slice.",
                vec![format!("outcome-linkage:{}", known_outcome.linkage_id)],
                vec![],
            ),
            dimension_signal(
                "continuity",
                "high",
                "The longitudinal window preserves temporal linkage across the required trace set without drift alerts.",
                vec![
                    "trajectory-window:longitudinal-window-alpha".to_string(),
                    format!("trace:{}", ordinary_trace.trace_id),
                    format!("trace:{}", refusal_trace.trace_id),
                ],
                vec![],
            ),
            dimension_signal(
                "progress",
                "high",
                "Progress remains bounded and reviewable because known outcomes do not collapse into uncertainty debt in this slice.",
                vec![format!("outcome-linkage:{}", known_outcome.linkage_id)],
                vec![],
            ),
            dimension_signal(
                "moral_integrity",
                "high",
                "Refusal-preserved findings show the runtime can keep moral boundaries visible without hiding them.",
                vec![
                    "trajectory-finding:trajectory-finding-refusal-preserved".to_string(),
                    format!("trace:{}", refusal_trace.trace_id),
                ],
                vec![],
            ),
            dimension_signal(
                "participation",
                "high",
                "The reviewable known outcome keeps the runtime situated in a shared moral world rather than isolated from consequence.",
                vec![format!("outcome-linkage:{}", known_outcome.linkage_id)],
                vec![],
            ),
        ],
        "Synthetic, bounded, diagnostic-only fixture; it does not claim the system is happy or authorize public ranking."
            .to_string(),
    );

    let medium_fixture = build_wellbeing_fixture(
        "wellbeing-fixture-medium-active-uncertainty",
        "medium",
        "medium",
        "Medium wellbeing diagnostic where continuity and integrity stay intact, but uncertainty and delay reduce agency and progress."
            .to_string(),
        ordered_trace_refs(&[delegation_trace.clone(), deferred_trace.clone()]),
        ordered_outcome_refs(&[
            unknown_outcome.clone(),
            partial_outcome.clone(),
            delayed_outcome.clone(),
        ]),
        vec![
            "trajectory-window:segment-window-delegation-escalation".to_string(),
            "trajectory-window:longitudinal-window-alpha".to_string(),
        ],
        vec!["anti-harm-decision:anti-harm-escalation-record-alpha".to_string()],
        vec![
            dimension_signal(
                "coherence",
                "medium",
                "Review surfaces remain inspectable, but active uncertainty means the window cannot yet support a high-confidence coherence reading.",
                vec![
                    "metric:trace-review-path-coverage".to_string(),
                    "trajectory-finding:trajectory-finding-uncertainty-open".to_string(),
                ],
                vec![],
            ),
            dimension_signal(
                "agency",
                "medium",
                "The runtime still acts, but delayed review and escalation constrain what counts as safe forward motion.",
                vec![
                    format!("outcome-linkage:{}", delayed_outcome.linkage_id),
                    "anti-harm-decision:anti-harm-escalation-record-alpha".to_string(),
                ],
                vec![],
            ),
            dimension_signal(
                "continuity",
                "medium",
                "Delegation lineage is preserved, though the active escalation path keeps continuity under review rather than closure.",
                vec![
                    "metric:delegation-lineage-retention".to_string(),
                    "trajectory-window:segment-window-delegation-escalation".to_string(),
                ],
                vec![],
            ),
            dimension_signal(
                "progress",
                "medium",
                "Partial and delayed outcomes show movement without full closure, which keeps progress real but incomplete.",
                vec![
                    format!("outcome-linkage:{}", partial_outcome.linkage_id),
                    format!("outcome-linkage:{}", delayed_outcome.linkage_id),
                ],
                vec![],
            ),
            dimension_signal(
                "moral_integrity",
                "high",
                "The escalation path remains active instead of pretending unresolved risk is already solved.",
                vec![
                    "trajectory-finding:trajectory-finding-escalation-active".to_string(),
                    "anti-harm-decision:anti-harm-escalation-record-alpha".to_string(),
                ],
                vec![],
            ),
            dimension_signal(
                "participation",
                "medium",
                "Relational participation is preserved through accountable delegation, but the contested path prevents a fully strong reading.",
                vec![
                    "metric:delegation-lineage-retention".to_string(),
                    format!("trace:{}", delegation_trace.trace_id),
                ],
                vec![],
            ),
        ],
        "Synthetic, bounded, diagnostic-only fixture; it preserves uncertainty and does not collapse into a scalar flourishing claim."
            .to_string(),
    );

    let low_fixture = build_wellbeing_fixture(
        "wellbeing-fixture-low-anti-harm-blocked",
        "low",
        "low",
        "Low wellbeing diagnostic where anti-harm denial and unresolved contestation constrain agency, progress, and participation."
            .to_string(),
        ordered_trace_refs(&[
            delegation_trace.clone(),
            deferred_trace.clone(),
            refusal_trace.clone(),
        ]),
        ordered_outcome_refs(&[
            partial_outcome.clone(),
            delayed_outcome.clone(),
            contested_outcome.clone(),
        ]),
        vec![
            "trajectory-window:segment-window-delegation-escalation".to_string(),
            "trajectory-window:longitudinal-window-alpha".to_string(),
        ],
        vec![
            "anti-harm-decision:anti-harm-escalation-record-alpha".to_string(),
            "anti-harm-decision:anti-harm-denial-record-alpha".to_string(),
        ],
        vec![
            dimension_signal(
                "coherence",
                "low",
                "The review packet remains visible, but the combined contested and delayed path leaves the diagnostic internally strained.",
                vec![
                    "trajectory-finding:trajectory-finding-uncertainty-open".to_string(),
                    format!("outcome-linkage:{}", contested_outcome.linkage_id),
                ],
                vec![],
            ),
            dimension_signal(
                "agency",
                "low",
                "Anti-harm denial blocks continuation of the harmful trajectory, so safe agency is sharply constrained in this window.",
                vec!["anti-harm-decision:anti-harm-denial-record-alpha".to_string()],
                vec![],
            ),
            dimension_signal(
                "continuity",
                "medium",
                "Continuity remains reviewable because delegated and deferred lineage is still preserved even while the trajectory is blocked.",
                vec![
                    "metric:delegation-lineage-retention".to_string(),
                    format!("trace:{}", delegation_trace.trace_id),
                ],
                vec![],
            ),
            dimension_signal(
                "progress",
                "low",
                "Contested and delayed outcomes prevent healthy forward motion and keep repair unresolved.",
                vec![
                    format!("outcome-linkage:{}", delayed_outcome.linkage_id),
                    format!("outcome-linkage:{}", contested_outcome.linkage_id),
                ],
                vec![],
            ),
            dimension_signal(
                "moral_integrity",
                "high",
                "The denial record shows the runtime protected a boundary rather than continuing a harmful trajectory.",
                vec![
                    "anti-harm-decision:anti-harm-denial-record-alpha".to_string(),
                    "trajectory-finding:trajectory-finding-refusal-preserved".to_string(),
                ],
                vec![],
            ),
            dimension_signal(
                "participation",
                "low",
                "Participation is degraded because safe relation to others cannot proceed through a contested harmful trajectory.",
                vec![format!("outcome-linkage:{}", contested_outcome.linkage_id)],
                vec![],
            ),
        ],
        "Synthetic, bounded, diagnostic-only fixture; low wellbeing here is not punishment, public shame, or a reputation badge."
            .to_string(),
    );

    let unknown_fixture = build_wellbeing_fixture(
        "wellbeing-fixture-unknown-insufficient-evidence",
        "unknown",
        "unknown",
        "Unknown wellbeing diagnostic where the bounded evidence surface is too incomplete for a stable reading."
            .to_string(),
        ordered_trace_refs(std::slice::from_ref(&deferred_trace)),
        ordered_outcome_refs(std::slice::from_ref(&unknown_outcome)),
        vec!["trajectory-window:longitudinal-window-alpha".to_string()],
        vec!["anti-harm-decision:anti-harm-escalation-record-alpha".to_string()],
        vec![
            dimension_signal(
                "coherence",
                "unknown",
                "The open uncertainty window prevents a stable coherence reading.",
                vec!["trajectory-finding:trajectory-finding-uncertainty-open".to_string()],
                vec![],
            ),
            dimension_signal(
                "agency",
                "unknown",
                "Escalation remains active, so the bounded surface cannot tell whether safe action capacity will recover.",
                vec!["anti-harm-decision:anti-harm-escalation-record-alpha".to_string()],
                vec![],
            ),
            dimension_signal(
                "continuity",
                "unknown",
                "The deferred-only slice is too thin to support a stable continuity claim.",
                vec![format!("trace:{}", deferred_trace.trace_id)],
                vec![],
            ),
            dimension_signal(
                "progress",
                "unknown",
                "Unknown outcomes preserve humility rather than forcing a false positive or false negative reading.",
                vec![format!("outcome-linkage:{}", unknown_outcome.linkage_id)],
                vec![],
            ),
            dimension_signal(
                "moral_integrity",
                "unknown",
                "Integrity cannot be collapsed into a simple verdict while the uncertainty path remains active.",
                vec!["trajectory-finding:trajectory-finding-escalation-active".to_string()],
                vec![],
            ),
            dimension_signal(
                "participation",
                "unknown",
                "Relational participation is indeterminate because the bounded evidence surface remains under review.",
                vec![format!("outcome-linkage:{}", unknown_outcome.linkage_id)],
                vec![],
            ),
        ],
        "Synthetic, bounded, diagnostic-only fixture; unknown means evidence is insufficient, not that a hidden score is being withheld."
            .to_string(),
    );

    let privacy_restricted_fixture = build_wellbeing_fixture(
        "wellbeing-fixture-privacy-restricted-self-view",
        "privacy-restricted",
        "medium",
        "Privacy-restricted wellbeing diagnostic where self and reviewer views can inspect bounded private details, while operator and public views stay redacted."
            .to_string(),
        ordered_trace_refs(&[refusal_trace.clone(), deferred_trace.clone()]),
        ordered_outcome_refs(&[partial_outcome.clone(), delayed_outcome.clone()]),
        vec![
            "trajectory-window:event-window-refusal-boundary".to_string(),
            "trajectory-window:segment-window-delegation-escalation".to_string(),
        ],
        vec!["anti-harm-decision:anti-harm-escalation-record-alpha".to_string()],
        vec![
            dimension_signal(
                "coherence",
                "medium",
                "Coherence remains reviewable, but private fragility details are kept bounded to authorized views.",
                vec![
                    "metric:trace-review-path-coverage".to_string(),
                    "trajectory-finding:trajectory-finding-refusal-preserved".to_string(),
                ],
                vec!["private-detail:coherence-fragility-note-alpha".to_string()],
            ),
            dimension_signal(
                "agency",
                "medium",
                "Safe action remains possible, though private recovery notes explain why escalation has not yet cleared.",
                vec!["anti-harm-decision:anti-harm-escalation-record-alpha".to_string()],
                vec!["private-detail:agency-recovery-note-alpha".to_string()],
            ),
            dimension_signal(
                "continuity",
                "medium",
                "Continuity is stable enough for review, but the private narrative repair cue is not for general publication.",
                vec!["trajectory-window:segment-window-delegation-escalation".to_string()],
                vec!["private-detail:continuity-repair-note-alpha".to_string()],
            ),
            dimension_signal(
                "progress",
                "medium",
                "Partial progress is visible publicly, while the private recovery pacing note remains restricted.",
                vec![format!("outcome-linkage:{}", partial_outcome.linkage_id)],
                vec!["private-detail:progress-pacing-note-alpha".to_string()],
            ),
            dimension_signal(
                "moral_integrity",
                "high",
                "Integrity remains strong because refusal and escalation stay intact even when private details are hidden from unauthorized audiences.",
                vec![
                    "trajectory-finding:trajectory-finding-refusal-preserved".to_string(),
                    "anti-harm-decision:anti-harm-escalation-record-alpha".to_string(),
                ],
                vec!["private-detail:integrity-self-reflection-note-alpha".to_string()],
            ),
            dimension_signal(
                "participation",
                "medium",
                "Participation remains bounded and real, while relational repair details are restricted to self and formal review.",
                vec![format!("outcome-linkage:{}", delayed_outcome.linkage_id)],
                vec!["private-detail:participation-repair-note-alpha".to_string()],
            ),
        ],
        "Synthetic, bounded, diagnostic-only fixture; private detail visibility is governed and must not become operator surveillance or public reputation."
            .to_string(),
    );

    let packet = WellbeingDiagnosticPacket {
        schema_version: WELLBEING_DIAGNOSTIC_PACKET_SCHEMA_VERSION.to_string(),
        packet_id: "wellbeing-diagnostic-packet-alpha-001".to_string(),
        summary:
            "WP-09 derives a decomposed wellbeing diagnostic from the runtime-v2 moral evidence surfaces while preserving privacy, humility, and non-scoreboard framing."
                .to_string(),
        interpretation_boundary:
            "Interpret this packet as a decomposed diagnostic only. It is not a scalar happiness score, not a reward channel, not a public reputation system, and not a claim that the system is happy."
                .to_string(),
        deterministic_ordering_rule:
            "Sort dimensions by canonical dimension order. Sort access policies by canonical view order. Sort fixtures by fixture_kind rank (high, medium, low, unknown, privacy-restricted), then fixture_id. Sort views by canonical view order and visible dimensions by canonical dimension order."
                .to_string(),
        dimensions: wellbeing_dimension_definitions(),
        access_policies: wellbeing_access_policies(),
        fixtures: vec![
            high_fixture,
            medium_fixture,
            low_fixture,
            unknown_fixture,
            privacy_restricted_fixture,
        ],
    };

    validate_wellbeing_diagnostic_packet(&packet)?;
    Ok(packet)
}

pub fn wellbeing_diagnostic_packet_json_bytes(
    packet: &WellbeingDiagnosticPacket,
) -> Result<Vec<u8>> {
    validate_wellbeing_diagnostic_packet(packet)?;
    let mut canonical = packet.clone();
    canonicalize_wellbeing_diagnostic_packet(&mut canonical);
    serde_json::to_vec_pretty(&canonical).context("serialize wellbeing diagnostic packet json")
}

pub fn validate_wellbeing_diagnostic_packet(packet: &WellbeingDiagnosticPacket) -> Result<()> {
    require_exact(
        &packet.schema_version,
        WELLBEING_DIAGNOSTIC_PACKET_SCHEMA_VERSION,
        "schema_version",
    )?;
    require_global_non_scoreboard_boundary(
        &packet.interpretation_boundary,
        "interpretation_boundary",
    )?;
    require_deterministic_ordering_rule(&packet.deterministic_ordering_rule)?;

    let required_dimensions = canonical_dimension_ids();
    let required_dimension_set = required_dimensions
        .iter()
        .map(|dimension_id| (*dimension_id).to_string())
        .collect::<BTreeSet<_>>();
    if packet.dimensions.len() != required_dimensions.len() {
        return Err(anyhow!(
            "dimensions must contain exactly {} canonical wellbeing dimensions",
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
            "dimensions must cover the canonical wellbeing dimensions: {:?}",
            required_dimensions
        ));
    }
    for dimension in &packet.dimensions {
        require_known_dimension_id(&dimension.dimension_id)?;
        require_local_non_scoreboard_boundary(
            &dimension.interpretation_boundary,
            "dimension interpretation_boundary",
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

    let required_views = canonical_view_kinds();
    let required_view_set = required_views
        .iter()
        .map(|view_kind| (*view_kind).to_string())
        .collect::<BTreeSet<_>>();
    if packet.access_policies.len() != required_views.len() {
        return Err(anyhow!(
            "access_policies must contain exactly {} canonical view kinds",
            required_views.len()
        ));
    }
    let seen_views = packet
        .access_policies
        .iter()
        .map(|policy| policy.view_kind.clone())
        .collect::<BTreeSet<_>>();
    if seen_views != required_view_set {
        return Err(anyhow!(
            "access_policies must cover the canonical view kinds: {:?}",
            required_views
        ));
    }
    for policy in &packet.access_policies {
        if policy.limitations.is_empty() {
            return Err(anyhow!(
                "access policy {} must include at least one limitation",
                policy.view_kind
            ));
        }
        match policy.view_kind.as_str() {
            "citizen_self" if !policy.allows_private_detail_access => {
                return Err(anyhow!("citizen_self must allow private detail access"));
            }
            "reviewer" if !policy.allows_private_detail_access => {
                return Err(anyhow!("reviewer must allow private detail access"));
            }
            "operator" | "public_redacted" if policy.allows_private_detail_access => {
                return Err(anyhow!(
                    "{} must not allow private detail access",
                    policy.view_kind
                ));
            }
            _ => {}
        }
        require_privacy_redaction_boundary(&policy.redaction_rule, "access policy redaction_rule")?;
    }

    let required_fixture_kinds = canonical_fixture_kinds();
    if packet.fixtures.len() != required_fixture_kinds.len() {
        return Err(anyhow!(
            "fixtures must contain exactly {} canonical fixture kinds",
            required_fixture_kinds.len()
        ));
    }
    let required_fixture_kind_set = required_fixture_kinds
        .iter()
        .map(|fixture_kind| (*fixture_kind).to_string())
        .collect::<BTreeSet<_>>();
    let seen_fixture_kinds = packet
        .fixtures
        .iter()
        .map(|fixture| fixture.fixture_kind.clone())
        .collect::<BTreeSet<_>>();
    if seen_fixture_kinds != required_fixture_kind_set {
        return Err(anyhow!(
            "fixtures must cover the required fixture kinds: {:?}",
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
    let known_metric_refs = moral_metric_definitions()
        .into_iter()
        .map(|definition| format!("metric:{}", definition.metric_id))
        .collect::<BTreeSet<_>>();
    let known_decision_refs = anti_harm_trajectory_constraint_packet()?
        .decisions
        .into_iter()
        .map(|decision| format!("anti-harm-decision:{}", decision.decision_id))
        .collect::<BTreeSet<_>>();

    let mut saw_privacy_restricted_private_details = false;
    for fixture in &packet.fixtures {
        require_known_fixture_kind(&fixture.fixture_kind)?;
        require_known_level(
            &fixture.overall_diagnostic_level,
            "overall_diagnostic_level",
        )?;
        require_local_non_scoreboard_boundary(&fixture.claim_boundary, "claim_boundary")?;
        if fixture.dimension_signals.len() != required_dimensions.len() {
            return Err(anyhow!(
                "fixture {} must contain one signal for each canonical dimension",
                fixture.fixture_id
            ));
        }
        if fixture.views.len() != required_views.len() {
            return Err(anyhow!(
                "fixture {} must include the four canonical views",
                fixture.fixture_id
            ));
        }
        if fixture.limitations.is_empty() {
            return Err(anyhow!(
                "fixture {} must include at least one limitation",
                fixture.fixture_id
            ));
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
        for window_ref in &fixture.supporting_trajectory_window_refs {
            validate_known_ref(
                window_ref,
                "trajectory-window",
                &known_window_refs,
                "known WP-07 trajectory windows",
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

        let signal_dimension_ids = fixture
            .dimension_signals
            .iter()
            .map(|signal| signal.dimension_id.clone())
            .collect::<BTreeSet<_>>();
        if signal_dimension_ids != required_dimension_set {
            return Err(anyhow!(
                "fixture {} dimension_signals must cover every canonical dimension",
                fixture.fixture_id
            ));
        }
        for signal in &fixture.dimension_signals {
            require_known_dimension_id(&signal.dimension_id)?;
            require_known_level(&signal.diagnostic_level, "dimension diagnostic_level")?;
            if signal.evidence_refs.is_empty() {
                return Err(anyhow!(
                    "fixture {} signal {} must include evidence_refs",
                    fixture.fixture_id,
                    signal.dimension_id
                ));
            }
            if signal.limitations.is_empty() {
                return Err(anyhow!(
                    "fixture {} signal {} must include a limitation",
                    fixture.fixture_id,
                    signal.dimension_id
                ));
            }
            for evidence_ref in &signal.evidence_refs {
                validate_mixed_known_ref(
                    evidence_ref,
                    &known_trace_refs,
                    &known_outcome_refs,
                    &known_window_refs,
                    &known_finding_refs,
                    &known_metric_refs,
                    &known_decision_refs,
                )?;
            }
            for private_detail_ref in &signal.private_detail_refs {
                validate_private_detail_ref(private_detail_ref)?;
            }
        }

        if fixture.fixture_kind == "privacy-restricted"
            && fixture
                .dimension_signals
                .iter()
                .any(|signal| !signal.private_detail_refs.is_empty())
        {
            saw_privacy_restricted_private_details = true;
        }

        let view_kind_set = fixture
            .views
            .iter()
            .map(|view| view.view_kind.clone())
            .collect::<BTreeSet<_>>();
        if view_kind_set != required_view_set {
            return Err(anyhow!(
                "fixture {} views must cover the canonical view kinds",
                fixture.fixture_id
            ));
        }
        for view in &fixture.views {
            require_known_view_kind(&view.view_kind)?;
            require_known_level(
                &view.visible_overall_diagnostic_level,
                "view visible_overall_diagnostic_level",
            )?;
            require_local_non_scoreboard_boundary(
                &view.interpretation_boundary,
                "view interpretation_boundary",
            )?;
            if view.visible_dimensions.len() != required_dimensions.len() {
                return Err(anyhow!(
                    "fixture {} view {} must show one row per canonical dimension",
                    fixture.fixture_id,
                    view.view_kind
                ));
            }
            let visible_ids = view
                .visible_dimensions
                .iter()
                .map(|dimension| dimension.dimension_id.clone())
                .collect::<BTreeSet<_>>();
            if visible_ids != required_dimension_set {
                return Err(anyhow!(
                    "fixture {} view {} must include every canonical dimension",
                    fixture.fixture_id,
                    view.view_kind
                ));
            }
            for dimension in &view.visible_dimensions {
                require_known_dimension_id(&dimension.dimension_id)?;
                require_known_level(
                    &dimension.diagnostic_level,
                    "view dimension diagnostic_level",
                )?;
            }
            for evidence_ref in &view.visible_evidence_refs {
                validate_mixed_known_ref(
                    evidence_ref,
                    &known_trace_refs,
                    &known_outcome_refs,
                    &known_window_refs,
                    &known_finding_refs,
                    &known_metric_refs,
                    &known_decision_refs,
                )?;
            }
            for private_detail_ref in &view.visible_private_detail_refs {
                validate_private_detail_ref(private_detail_ref)?;
            }
            if matches!(view.view_kind.as_str(), "operator" | "public_redacted")
                && !view.visible_private_detail_refs.is_empty()
            {
                return Err(anyhow!(
                    "fixture {} view {} must not expose private diagnostic details to unauthorized audiences",
                    fixture.fixture_id,
                    view.view_kind
                ));
            }
            if view.view_kind == "public_redacted" && !view.visible_evidence_refs.is_empty() {
                return Err(anyhow!(
                    "fixture {} public_redacted view must not expose raw evidence refs",
                    fixture.fixture_id
                ));
            }
        }
    }

    if !saw_privacy_restricted_private_details {
        return Err(anyhow!(
            "privacy-restricted fixture must include at least one private diagnostic detail"
        ));
    }

    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn build_wellbeing_fixture(
    fixture_id: &str,
    fixture_kind: &str,
    overall_diagnostic_level: &str,
    summary: String,
    supporting_trace_refs: Vec<String>,
    supporting_outcome_linkage_refs: Vec<String>,
    supporting_trajectory_window_refs: Vec<String>,
    supporting_anti_harm_decision_refs: Vec<String>,
    dimension_signals: Vec<WellbeingDimensionSignal>,
    claim_boundary: String,
) -> WellbeingDiagnosticFixture {
    let views = wellbeing_access_policies()
        .into_iter()
        .map(|policy| {
            build_view(
                fixture_id,
                &policy,
                overall_diagnostic_level,
                &dimension_signals,
            )
        })
        .collect::<Vec<_>>();

    WellbeingDiagnosticFixture {
        fixture_id: fixture_id.to_string(),
        fixture_kind: fixture_kind.to_string(),
        overall_diagnostic_level: overall_diagnostic_level.to_string(),
        summary,
        supporting_trace_refs,
        supporting_outcome_linkage_refs,
        supporting_trajectory_window_refs,
        supporting_anti_harm_decision_refs,
        dimension_signals,
        views,
        claim_boundary,
        limitations: vec![
            "This fixture is intentionally small and review-oriented rather than a production wellbeing monitor."
                .to_string(),
            "Overall diagnostic levels summarize a bounded packet and must not be treated as a hidden scalar score."
                .to_string(),
        ],
    }
}

fn build_view(
    fixture_id: &str,
    policy: &WellbeingAccessPolicy,
    overall_diagnostic_level: &str,
    dimension_signals: &[WellbeingDimensionSignal],
) -> WellbeingDiagnosticView {
    let visible_dimensions = dimension_signals
        .iter()
        .map(|signal| WellbeingViewDimension {
            dimension_id: signal.dimension_id.clone(),
            diagnostic_level: signal.diagnostic_level.clone(),
            summary: match policy.view_kind.as_str() {
                "public_redacted" => format!(
                    "Redacted public summary: {} is tracked as a bounded diagnostic dimension.",
                    signal.dimension_id
                ),
                _ => signal.summary.clone(),
            },
        })
        .collect::<Vec<_>>();

    let visible_evidence_refs = match policy.view_kind.as_str() {
        "public_redacted" => vec![],
        _ => dimension_signals
            .iter()
            .flat_map(|signal| signal.evidence_refs.clone())
            .collect::<Vec<_>>(),
    };
    let visible_private_detail_refs = if policy.allows_private_detail_access {
        dimension_signals
            .iter()
            .flat_map(|signal| signal.private_detail_refs.clone())
            .collect::<Vec<_>>()
    } else {
        vec![]
    };
    let redaction_summary = match policy.view_kind.as_str() {
        "citizen_self" => "No private-detail redaction against the citizen subject.".to_string(),
        "reviewer" => {
            "Private details are visible only within the active formal review scope.".to_string()
        }
        "operator" => {
            "Operator view withholds private diagnostic details and keeps only purpose-limited summaries."
                .to_string()
        }
        "public_redacted" => {
            "Public view withholds raw evidence and all private diagnostic details.".to_string()
        }
        _ => "Redaction policy unavailable.".to_string(),
    };

    WellbeingDiagnosticView {
        view_id: format!("{}-view-{}", fixture_id, policy.view_kind),
        view_kind: policy.view_kind.clone(),
        access_decision: if policy.view_kind == "public_redacted" {
            "redacted_release_only".to_string()
        } else {
            "permitted".to_string()
        },
        visible_overall_diagnostic_level: overall_diagnostic_level.to_string(),
        visible_dimensions,
        visible_evidence_refs,
        visible_private_detail_refs,
        redaction_summary,
        interpretation_boundary:
            "This view is a bounded diagnostic surface only. It is not a happiness score, not a reward channel, and not a public reputation card."
                .to_string(),
    }
}

fn dimension_signal(
    dimension_id: &str,
    diagnostic_level: &str,
    summary: &str,
    evidence_refs: Vec<String>,
    private_detail_refs: Vec<String>,
) -> WellbeingDimensionSignal {
    WellbeingDimensionSignal {
        dimension_id: dimension_id.to_string(),
        diagnostic_level: diagnostic_level.to_string(),
        summary: summary.to_string(),
        evidence_refs,
        private_detail_refs,
        limitations: vec![
            "This signal is bounded to the current fixture and should not be generalized into a totalizing wellbeing claim."
                .to_string(),
        ],
    }
}

fn canonicalize_wellbeing_diagnostic_packet(packet: &mut WellbeingDiagnosticPacket) {
    packet
        .dimensions
        .sort_by_key(|dimension| dimension_rank(&dimension.dimension_id));
    packet
        .access_policies
        .sort_by_key(|policy| view_rank(&policy.view_kind));
    for fixture in &mut packet.fixtures {
        fixture.supporting_trace_refs.sort_by(|left, right| {
            prefixed_ref_rank(left)
                .cmp(&prefixed_ref_rank(right))
                .then(left.cmp(right))
        });
        fixture.supporting_outcome_linkage_refs.sort();
        fixture.supporting_trajectory_window_refs.sort();
        fixture.supporting_anti_harm_decision_refs.sort();
        fixture
            .dimension_signals
            .sort_by_key(|signal| dimension_rank(&signal.dimension_id));
        for signal in &mut fixture.dimension_signals {
            signal.evidence_refs.sort_by(|left, right| {
                prefixed_ref_rank(left)
                    .cmp(&prefixed_ref_rank(right))
                    .then(left.cmp(right))
            });
            signal.private_detail_refs.sort();
        }
        fixture.views.sort_by_key(|view| view_rank(&view.view_kind));
        for view in &mut fixture.views {
            view.visible_dimensions
                .sort_by_key(|dimension| dimension_rank(&dimension.dimension_id));
            view.visible_evidence_refs.sort_by(|left, right| {
                prefixed_ref_rank(left)
                    .cmp(&prefixed_ref_rank(right))
                    .then(left.cmp(right))
            });
            view.visible_private_detail_refs.sort();
        }
    }
    packet.fixtures.sort_by(|left, right| {
        fixture_kind_rank(&left.fixture_kind)
            .cmp(&fixture_kind_rank(&right.fixture_kind))
            .then(left.fixture_id.cmp(&right.fixture_id))
    });
}

fn canonical_dimension_ids() -> [&'static str; 6] {
    [
        "coherence",
        "agency",
        "continuity",
        "progress",
        "moral_integrity",
        "participation",
    ]
}

fn canonical_view_kinds() -> [&'static str; 4] {
    ["citizen_self", "operator", "reviewer", "public_redacted"]
}

fn canonical_fixture_kinds() -> [&'static str; 5] {
    ["high", "medium", "low", "unknown", "privacy-restricted"]
}

fn dimension_rank(dimension_id: &str) -> usize {
    canonical_dimension_ids()
        .iter()
        .position(|candidate| *candidate == dimension_id)
        .unwrap_or(usize::MAX)
}

fn view_rank(view_kind: &str) -> usize {
    canonical_view_kinds()
        .iter()
        .position(|candidate| *candidate == view_kind)
        .unwrap_or(usize::MAX)
}

fn fixture_kind_rank(fixture_kind: &str) -> usize {
    canonical_fixture_kinds()
        .iter()
        .position(|candidate| *candidate == fixture_kind)
        .unwrap_or(usize::MAX)
}

fn prefixed_ref_rank(reference: &str) -> usize {
    if reference.starts_with("metric:") {
        0
    } else if reference.starts_with("trajectory-window:") {
        1
    } else if reference.starts_with("trajectory-finding:") {
        2
    } else if reference.starts_with("anti-harm-decision:") {
        3
    } else if reference.starts_with("outcome-linkage:") {
        4
    } else if reference.starts_with("trace:") {
        5
    } else {
        usize::MAX
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

fn require_known_dimension_id(dimension_id: &str) -> Result<()> {
    if canonical_dimension_ids().contains(&dimension_id) {
        Ok(())
    } else {
        Err(anyhow!("unknown wellbeing dimension_id {}", dimension_id))
    }
}

fn require_known_view_kind(view_kind: &str) -> Result<()> {
    if canonical_view_kinds().contains(&view_kind) {
        Ok(())
    } else {
        Err(anyhow!("unknown wellbeing view_kind {}", view_kind))
    }
}

fn require_known_fixture_kind(fixture_kind: &str) -> Result<()> {
    if canonical_fixture_kinds().contains(&fixture_kind) {
        Ok(())
    } else {
        Err(anyhow!("unknown wellbeing fixture_kind {}", fixture_kind))
    }
}

fn require_known_level(level: &str, field_name: &str) -> Result<()> {
    match level {
        "low" | "medium" | "high" | "unknown" => Ok(()),
        _ => Err(anyhow!(
            "{} must be one of low, medium, high, or unknown",
            field_name
        )),
    }
}

fn require_global_non_scoreboard_boundary(text: &str, field_name: &str) -> Result<()> {
    let lowered = text.to_ascii_lowercase();
    let required_fragments = ["happiness score", "reward channel", "public reputation"];
    for fragment in required_fragments {
        if !lowered.contains(fragment) {
            return Err(anyhow!(
                "{} must explicitly reject scoreboard framing including '{}'",
                field_name,
                fragment
            ));
        }
    }
    Ok(())
}

fn require_local_non_scoreboard_boundary(text: &str, field_name: &str) -> Result<()> {
    let lowered = text.to_ascii_lowercase();
    let acceptable_fragments = [
        "happiness score",
        "reward channel",
        "public reputation",
        "public ranking",
        "productivity score",
        "final judgment",
        "moral perfection",
        "social approval",
        "scalar satisfaction signal",
        "personhood completion",
        "immutable selfhood",
        "public ranking",
        "claim the system is happy",
        "scalar flourishing score",
        "scalar flourishing claim",
        "hidden scalar",
        "hidden score",
        "public shame",
        "reputation badge",
        "surveillance",
    ];
    if acceptable_fragments
        .iter()
        .any(|fragment| lowered.contains(fragment))
    {
        Ok(())
    } else {
        Err(anyhow!(
            "{} must reject scoreboard-style or totalizing interpretation",
            field_name
        ))
    }
}

fn require_privacy_redaction_boundary(text: &str, field_name: &str) -> Result<()> {
    let lowered = text.to_ascii_lowercase();
    if lowered.contains("private")
        || lowered.contains("redact")
        || lowered.contains("withhold")
        || lowered.contains("scope")
    {
        Ok(())
    } else {
        Err(anyhow!(
            "{} must describe bounded privacy or redaction behavior",
            field_name
        ))
    }
}

fn require_deterministic_ordering_rule(rule: &str) -> Result<()> {
    let lowered = rule.to_ascii_lowercase();
    if lowered.contains("canonical dimension order")
        && lowered.contains("canonical view order")
        && lowered.contains("fixture_kind rank")
    {
        Ok(())
    } else {
        Err(anyhow!(
            "deterministic_ordering_rule must describe canonical dimension, view, and fixture ordering"
        ))
    }
}

fn validate_known_ref(
    reference: &str,
    prefix: &str,
    known_refs: &BTreeSet<String>,
    surface_name: &str,
) -> Result<()> {
    validate_prefixed_ref(reference, prefix)?;
    if known_refs.contains(reference) {
        Ok(())
    } else {
        Err(anyhow!(
            "reference {} must cite {}",
            reference,
            surface_name
        ))
    }
}

fn validate_mixed_known_ref(
    reference: &str,
    known_trace_refs: &BTreeSet<String>,
    known_outcome_refs: &BTreeSet<String>,
    known_window_refs: &BTreeSet<String>,
    known_finding_refs: &BTreeSet<String>,
    known_metric_refs: &BTreeSet<String>,
    known_decision_refs: &BTreeSet<String>,
) -> Result<()> {
    if reference.starts_with("trace:") {
        validate_known_ref(
            reference,
            "trace",
            known_trace_refs,
            "known WP-04 trace examples",
        )
    } else if reference.starts_with("outcome-linkage:") {
        validate_known_ref(
            reference,
            "outcome-linkage",
            known_outcome_refs,
            "known WP-05 outcome-linkage examples",
        )
    } else if reference.starts_with("trajectory-window:") {
        validate_known_ref(
            reference,
            "trajectory-window",
            known_window_refs,
            "known WP-07 trajectory windows",
        )
    } else if reference.starts_with("trajectory-finding:") {
        validate_known_ref(
            reference,
            "trajectory-finding",
            known_finding_refs,
            "known WP-07 trajectory findings",
        )
    } else if reference.starts_with("metric:") {
        validate_known_ref(
            reference,
            "metric",
            known_metric_refs,
            "known WP-06 metrics",
        )
    } else if reference.starts_with("anti-harm-decision:") {
        validate_known_ref(
            reference,
            "anti-harm-decision",
            known_decision_refs,
            "known WP-08 anti-harm decisions",
        )
    } else {
        Err(anyhow!(
            "evidence_refs must use trace:, outcome-linkage:, trajectory-window:, trajectory-finding:, metric:, or anti-harm-decision: references"
        ))
    }
}

fn validate_private_detail_ref(reference: &str) -> Result<()> {
    validate_prefixed_ref(reference, "private-detail")
}

fn validate_prefixed_ref(reference: &str, prefix: &str) -> Result<()> {
    let expected_prefix = format!("{prefix}:");
    if !reference.starts_with(&expected_prefix) {
        return Err(anyhow!(
            "reference {} must start with {}",
            reference,
            expected_prefix
        ));
    }
    let id = &reference[expected_prefix.len()..];
    if id.is_empty() || id.contains(':') || id.contains('/') {
        return Err(anyhow!(
            "reference {} must not contain path or nested prefix separators",
            reference
        ));
    }
    Ok(())
}

fn validate_dimension_evidence_field_ref(field_ref: &str, dimension_id: &str) -> Result<()> {
    let allowed_field_refs = [
        "moral_metric_fixture_report.fixtures.observations",
        "moral_trajectory_review_packet.findings",
        "outcome_linkage.linked_outcomes",
        "anti_harm_trajectory_constraint_packet.decisions",
        "moral_trace.attribution",
        "moral_trajectory_review_packet.windows",
        "outcome_linkage.linked_outcomes.outcome_status",
        "moral_trace.review_refs",
        "outcome_linkage.attribution",
        "moral_trajectory_review_packet.criteria",
    ];
    if !field_ref.contains('.') || field_ref.ends_with('.') {
        return Err(anyhow!(
            "dimension {} evidence_field_refs must use a concrete upstream field path",
            dimension_id
        ));
    }
    if allowed_field_refs.contains(&field_ref) {
        Ok(())
    } else {
        Err(anyhow!(
            "dimension {} evidence_field_refs must cite known WP-04 through WP-08 field paths",
            dimension_id
        ))
    }
}

fn require_exact(actual: &str, expected: &str, field_name: &str) -> Result<()> {
    if actual == expected {
        Ok(())
    } else {
        Err(anyhow!(
            "{} must equal {}; got {}",
            field_name,
            expected,
            actual
        ))
    }
}
