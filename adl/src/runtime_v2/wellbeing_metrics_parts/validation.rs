use super::*;
use anyhow::{anyhow, Context, Result};
use std::collections::BTreeSet;

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
pub(crate) fn build_wellbeing_fixture(
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

pub(crate) fn build_view(
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

pub(crate) fn dimension_signal(
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

pub(crate) fn ordered_trace_refs(traces: &[MoralTraceRecord]) -> Vec<String> {
    let mut refs = traces
        .iter()
        .map(|trace| format!("trace:{}", trace.trace_id))
        .collect::<Vec<_>>();
    refs.sort();
    refs
}

pub(crate) fn ordered_outcome_refs(outcomes: &[OutcomeLinkageRecord]) -> Vec<String> {
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
