use super::*;
use anyhow::{anyhow, Result};

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
