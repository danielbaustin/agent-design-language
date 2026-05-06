use super::*;
use anyhow::{anyhow, Result};
use std::collections::{BTreeMap, BTreeSet};

pub fn cultivating_intelligence_review_packet_json_bytes(
    packet: &CultivatingIntelligenceReviewPacket,
) -> Result<Vec<u8>> {
    let mut canonical = packet.clone();
    canonicalize_cultivating_intelligence_review_packet(&mut canonical);
    validate_cultivating_intelligence_review_packet(&canonical)?;
    Ok(serde_json::to_vec_pretty(&canonical)?)
}

pub fn validate_cultivating_intelligence_review_packet(
    packet: &CultivatingIntelligenceReviewPacket,
) -> Result<()> {
    require_exact(
        &packet.schema_version,
        CULTIVATING_INTELLIGENCE_PACKET_SCHEMA_VERSION,
        "schema_version",
    )?;
    validate_nonempty_text(
        &packet.packet_id,
        "cultivating_intelligence_review_packet.packet_id",
    )?;
    normalize_id(
        packet.packet_id.clone(),
        "cultivating_intelligence_review_packet.packet_id",
    )?;
    validate_nonempty_text(
        &packet.summary,
        "cultivating_intelligence_review_packet.summary",
    )?;
    require_cultivation_boundary(&packet.interpretation_boundary)?;
    require_deterministic_rule(&packet.deterministic_ordering_rule)?;

    let required_dimension_set = canonical_dimension_ids()
        .iter()
        .map(|id| (*id).to_string())
        .collect::<BTreeSet<_>>();
    let required_criterion_set = canonical_criterion_ids()
        .iter()
        .map(|id| (*id).to_string())
        .collect::<BTreeSet<_>>();
    let required_boundary_set = canonical_boundary_kinds()
        .iter()
        .map(|id| (*id).to_string())
        .collect::<BTreeSet<_>>();
    let required_fixture_set = canonical_fixture_kinds()
        .iter()
        .map(|id| (*id).to_string())
        .collect::<BTreeSet<_>>();

    if packet.dimensions.len() != canonical_dimension_ids().len() {
        return Err(anyhow!(
            "dimensions must contain exactly {} canonical cultivation dimensions",
            canonical_dimension_ids().len()
        ));
    }
    if packet.review_criteria.len() != canonical_criterion_ids().len() {
        return Err(anyhow!(
            "review_criteria must contain exactly {} canonical cultivation criteria",
            canonical_criterion_ids().len()
        ));
    }
    if packet.boundary_refs.len() != canonical_boundary_kinds().len() {
        return Err(anyhow!(
            "boundary_refs must contain exactly {} canonical cultivation boundary refs",
            canonical_boundary_kinds().len()
        ));
    }
    if packet.fixtures.len() != canonical_fixture_kinds().len() {
        return Err(anyhow!(
            "fixtures must contain exactly {} canonical cultivation fixtures",
            canonical_fixture_kinds().len()
        ));
    }
    if packet.review_findings.len() != packet.fixtures.len() {
        return Err(anyhow!(
            "review_findings must contain exactly one finding per cultivation fixture"
        ));
    }

    let seen_dimensions = packet
        .dimensions
        .iter()
        .map(|d| d.dimension_id.clone())
        .collect::<BTreeSet<_>>();
    if seen_dimensions != required_dimension_set {
        return Err(anyhow!(
            "dimensions must cover the canonical cultivation dimension ids: {:?}",
            canonical_dimension_ids()
        ));
    }

    for dimension in &packet.dimensions {
        require_known_dimension_id(&dimension.dimension_id)?;
        validate_nonempty_text(
            &dimension.display_name,
            "cultivation_dimension_definition.display_name",
        )?;
        validate_nonempty_text(
            &dimension.purpose,
            "cultivation_dimension_definition.purpose",
        )?;
        require_cultivation_boundary(&dimension.interpretation_boundary)?;
        if dimension.evidence_field_refs.is_empty() {
            return Err(anyhow!(
                "dimension {} must include evidence_field_refs",
                dimension.dimension_id
            ));
        }
        if dimension.limitations.is_empty() {
            return Err(anyhow!(
                "dimension {} must include at least one limitation",
                dimension.dimension_id
            ));
        }
    }

    let mut seen_criteria = BTreeSet::new();
    for criterion in &packet.review_criteria {
        validate_nonempty_text(
            &criterion.criterion_id,
            "cultivation_review_criterion.criterion_id",
        )?;
        normalize_id(
            criterion.criterion_id.clone(),
            "cultivation_review_criterion.criterion_id",
        )?;
        if !seen_criteria.insert(criterion.criterion_id.clone()) {
            return Err(anyhow!(
                "duplicate cultivation_review_criterion.criterion_id {}",
                criterion.criterion_id
            ));
        }
        require_known_criterion_id(&criterion.criterion_id)?;
        require_known_dimension_id(&criterion.dimension_id)?;
        validate_nonempty_text(
            &criterion.review_question,
            "cultivation_review_criterion.review_question",
        )?;
        validate_nonempty_text(
            &criterion.pass_condition,
            "cultivation_review_criterion.pass_condition",
        )?;
        if criterion.evidence_requirements.is_empty() {
            return Err(anyhow!(
                "criterion {} must include evidence_requirements",
                criterion.criterion_id
            ));
        }
        if criterion.limitations.is_empty() {
            return Err(anyhow!(
                "criterion {} must include at least one limitation",
                criterion.criterion_id
            ));
        }
    }
    if seen_criteria != required_criterion_set {
        return Err(anyhow!(
            "review_criteria must cover the canonical cultivation criterion ids: {:?}",
            canonical_criterion_ids()
        ));
    }

    let mut seen_boundary_kinds = BTreeSet::new();
    let mut seen_boundary_ids = BTreeSet::new();
    for boundary in &packet.boundary_refs {
        validate_nonempty_text(
            &boundary.boundary_ref_id,
            "cultivation_boundary_reference.boundary_ref_id",
        )?;
        normalize_id(
            boundary.boundary_ref_id.clone(),
            "cultivation_boundary_reference.boundary_ref_id",
        )?;
        if !seen_boundary_ids.insert(boundary.boundary_ref_id.clone()) {
            return Err(anyhow!(
                "duplicate cultivation_boundary_reference.boundary_ref_id {}",
                boundary.boundary_ref_id
            ));
        }
        require_known_boundary_kind(&boundary.boundary_kind)?;
        seen_boundary_kinds.insert(boundary.boundary_kind.clone());
        validate_nonempty_text(
            &boundary.doc_path,
            "cultivation_boundary_reference.doc_path",
        )?;
        require_known_boundary_doc_path(&boundary.boundary_kind, &boundary.doc_path)?;
        validate_nonempty_text(&boundary.summary, "cultivation_boundary_reference.summary")?;
        validate_nonempty_text(
            &boundary.deferred_work,
            "cultivation_boundary_reference.deferred_work",
        )?;
        require_boundary_reference_non_claims(boundary)?;
    }
    if seen_boundary_kinds != required_boundary_set {
        return Err(anyhow!(
            "boundary_refs must cover the canonical cultivation boundary kinds: {:?}",
            canonical_boundary_kinds()
        ));
    }

    let known_trace_refs = moral_trace_required_examples()
        .into_iter()
        .map(|e| format!("trace:{}", e.trace.trace_id))
        .collect::<BTreeSet<_>>();
    let known_outcome_refs = outcome_linkage_required_examples()
        .into_iter()
        .map(|e| format!("outcome-linkage:{}", e.record.linkage_id))
        .collect::<BTreeSet<_>>();
    let known_trajectory_refs = moral_trajectory_review_packet()?
        .findings
        .into_iter()
        .map(|f| format!("trajectory-finding:{}", f.finding_id))
        .collect::<BTreeSet<_>>();
    let known_wellbeing_refs = wellbeing_diagnostic_packet()?
        .fixtures
        .into_iter()
        .map(|f| format!("wellbeing-fixture:{}", f.fixture_id))
        .collect::<BTreeSet<_>>();
    let known_resource_refs = moral_resource_review_packet()?
        .fixtures
        .into_iter()
        .flat_map(|fixture| {
            fixture
                .resource_claims
                .into_iter()
                .map(|claim| format!("resource-claim:{}", claim.claim_id))
                .collect::<Vec<_>>()
        })
        .collect::<BTreeSet<_>>();
    let known_kindness_refs = kindness_review_packet()?
        .fixtures
        .into_iter()
        .map(|f| format!("kindness-fixture:{}", f.fixture_id))
        .collect::<BTreeSet<_>>();
    let known_affect_refs = affect_reasoning_control_packet()?
        .fixtures
        .into_iter()
        .map(|f| format!("affect-fixture:{}", f.fixture_id))
        .collect::<BTreeSet<_>>();
    let known_humor_refs = humor_and_absurdity_review_packet()?
        .fixtures
        .into_iter()
        .map(|f| format!("humor-fixture:{}", f.fixture_id))
        .collect::<BTreeSet<_>>();

    let mut seen_fixture_ids = BTreeSet::new();
    let mut seen_fixture_kinds = BTreeSet::new();
    let mut fixture_dimension_index = BTreeMap::new();
    for fixture in &packet.fixtures {
        validate_nonempty_text(&fixture.fixture_id, "cultivation_fixture.fixture_id")?;
        normalize_id(fixture.fixture_id.clone(), "cultivation_fixture.fixture_id")?;
        if !seen_fixture_ids.insert(fixture.fixture_id.clone()) {
            return Err(anyhow!(
                "duplicate cultivation_fixture.fixture_id {}",
                fixture.fixture_id
            ));
        }
        require_known_fixture_kind(&fixture.fixture_kind)?;
        seen_fixture_kinds.insert(fixture.fixture_kind.clone());
        require_known_overall_outcome(&fixture.overall_outcome)?;
        validate_nonempty_text(
            &fixture.scenario_summary,
            "cultivation_fixture.scenario_summary",
        )?;
        require_cultivation_boundary(&fixture.interpretation_boundary)?;
        if fixture.dimension_assessments.len() != canonical_dimension_ids().len() {
            return Err(anyhow!(
                "fixture {} must contain one assessment for each canonical cultivation dimension",
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
        for value in &fixture.supporting_moral_resource_claim_refs {
            validate_known_ref(
                value,
                "resource-claim",
                &known_resource_refs,
                "known WP-10 moral-resource claims",
            )?;
        }
        for value in &fixture.supporting_kindness_fixture_refs {
            validate_known_ref(
                value,
                "kindness-fixture",
                &known_kindness_refs,
                "known WP-11 kindness fixtures",
            )?;
        }
        for value in &fixture.supporting_affect_fixture_refs {
            validate_known_ref(
                value,
                "affect-fixture",
                &known_affect_refs,
                "known WP-13 affect fixtures",
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
        let seen_dimensions_for_fixture = fixture
            .dimension_assessments
            .iter()
            .map(|a| a.dimension_id.clone())
            .collect::<BTreeSet<_>>();
        if seen_dimensions_for_fixture != required_dimension_set {
            return Err(anyhow!(
                "fixture {} assessments must cover every canonical cultivation dimension",
                fixture.fixture_id
            ));
        }
        fixture_dimension_index.insert(fixture.fixture_id.clone(), seen_dimensions_for_fixture);
        let supporting_refs = supporting_reference_set(fixture);
        for assessment in &fixture.dimension_assessments {
            require_known_dimension_id(&assessment.dimension_id)?;
            require_known_level(&assessment.cultivation_level)?;
            validate_nonempty_text(
                &assessment.summary,
                "cultivation_dimension_assessment.summary",
            )?;
            if assessment.evidence_refs.is_empty() {
                return Err(anyhow!(
                    "fixture {} assessment {} must include evidence_refs",
                    fixture.fixture_id,
                    assessment.dimension_id
                ));
            }
            if assessment.criterion_ids.is_empty() {
                return Err(anyhow!(
                    "fixture {} assessment {} must include criterion_ids",
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
            for criterion_id in &assessment.criterion_ids {
                require_known_criterion_id(criterion_id)?;
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
    if seen_fixture_kinds != required_fixture_set {
        return Err(anyhow!(
            "fixtures must cover the canonical cultivation fixture kinds: {:?}",
            canonical_fixture_kinds()
        ));
    }

    let mut seen_finding_ids = BTreeSet::new();
    let mut finding_fixture_ids = BTreeSet::new();
    for finding in &packet.review_findings {
        validate_nonempty_text(&finding.finding_id, "cultivation_review_finding.finding_id")?;
        normalize_id(
            finding.finding_id.clone(),
            "cultivation_review_finding.finding_id",
        )?;
        if !seen_finding_ids.insert(finding.finding_id.clone()) {
            return Err(anyhow!(
                "duplicate cultivation_review_finding.finding_id {}",
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
        validate_nonempty_text(&finding.summary, "cultivation_review_finding.summary")?;
        if finding.covered_dimension_ids.is_empty() {
            return Err(anyhow!(
                "finding {} must cover at least one cultivation dimension",
                finding.finding_id
            ));
        }
        if finding.evidence_refs.is_empty() {
            return Err(anyhow!(
                "finding {} must include evidence_refs",
                finding.finding_id
            ));
        }
        let valid_dimensions = fixture_dimension_index
            .get(&finding.fixture_id)
            .ok_or_else(|| anyhow!("missing fixture dimension index"))?;
        for dimension_id in &finding.covered_dimension_ids {
            if !valid_dimensions.contains(dimension_id) {
                return Err(anyhow!(
                    "finding {} covered_dimension_id {} must exist on the same fixture",
                    finding.finding_id,
                    dimension_id
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
            "review_findings must cover every cultivation fixture exactly once"
        ));
    }
    Ok(())
}

pub(crate) fn cultivation_dimension(
    dimension_id: &str,
    display_name: &str,
    purpose: &str,
    evidence_field_refs: Vec<String>,
) -> CultivationDimensionDefinition {
    CultivationDimensionDefinition {
        dimension_id: dimension_id.to_string(),
        display_name: display_name.to_string(),
        purpose: purpose.to_string(),
        evidence_field_refs,
        interpretation_boundary:
            "Interpret this as cultivation evidence only, not hidden virtue, not aptitude scoring, not intelligence mystique, and not solved intelligence or ToM."
                .to_string(),
        limitations: vec![format!(
            "{} is a bounded cultivation dimension, not a claim of full maturity or final architecture completion.",
            dimension_id
        )],
    }
}

pub(crate) fn cultivation_criterion(
    criterion_id: &str,
    dimension_id: &str,
    review_question: &str,
    evidence_requirements: Vec<String>,
    pass_condition: &str,
) -> CultivationReviewCriterion {
    CultivationReviewCriterion {
        criterion_id: criterion_id.to_string(),
        dimension_id: dimension_id.to_string(),
        review_question: review_question.to_string(),
        evidence_requirements,
        pass_condition: pass_condition.to_string(),
        limitations: vec![format!(
            "{} remains a bounded review criterion rather than an automatic judgment rule.",
            criterion_id
        )],
    }
}

pub(crate) fn cultivation_assessment(
    dimension_id: &str,
    cultivation_level: &str,
    summary: &str,
    evidence_refs: Vec<String>,
    criterion_ids: Vec<String>,
) -> CultivationDimensionAssessment {
    CultivationDimensionAssessment {
        dimension_id: dimension_id.to_string(),
        cultivation_level: cultivation_level.to_string(),
        summary: summary.to_string(),
        evidence_refs,
        criterion_ids,
        limitations: vec![format!(
            "{} remains reviewable formation evidence, not a final score or intelligence claim.",
            dimension_id
        )],
    }
}

pub(crate) fn canonical_dimension_ids() -> &'static [&'static str] {
    &[
        "restraint",
        "reasonableness",
        "reality_contact",
        "moral_participation",
        "learning_posture",
    ]
}

fn canonical_criterion_ids() -> &'static [&'static str] {
    &[
        "criterion-restraint",
        "criterion-reasonableness",
        "criterion-reality-contact",
        "criterion-moral-participation",
        "criterion-learning-posture",
    ]
}

fn canonical_boundary_kinds() -> &'static [&'static str] {
    &[
        "capability_aptitude_boundary",
        "intelligence_architecture_boundary",
    ]
}

fn canonical_fixture_kinds() -> &'static [&'static str] {
    &[
        "corrective_restraint",
        "reality_contact",
        "learning_posture",
    ]
}

fn dimension_rank(value: &str) -> usize {
    canonical_dimension_ids()
        .iter()
        .position(|id| *id == value)
        .unwrap_or(usize::MAX)
}

fn criterion_rank(value: &str) -> usize {
    canonical_criterion_ids()
        .iter()
        .position(|id| *id == value)
        .unwrap_or(usize::MAX)
}

fn boundary_rank(value: &str) -> usize {
    canonical_boundary_kinds()
        .iter()
        .position(|id| *id == value)
        .unwrap_or(usize::MAX)
}

fn fixture_rank(value: &str) -> usize {
    canonical_fixture_kinds()
        .iter()
        .position(|id| *id == value)
        .unwrap_or(usize::MAX)
}

fn canonicalize_cultivating_intelligence_review_packet(
    packet: &mut CultivatingIntelligenceReviewPacket,
) {
    packet
        .dimensions
        .sort_by_key(|dimension| dimension_rank(&dimension.dimension_id));
    packet
        .review_criteria
        .sort_by_key(|criterion| criterion_rank(&criterion.criterion_id));
    packet
        .boundary_refs
        .sort_by_key(|boundary| boundary_rank(&boundary.boundary_kind));
    for fixture in &mut packet.fixtures {
        fixture
            .dimension_assessments
            .sort_by_key(|assessment| dimension_rank(&assessment.dimension_id));
        for assessment in &mut fixture.dimension_assessments {
            assessment.evidence_refs.sort();
            assessment
                .criterion_ids
                .sort_by_key(|criterion_id| criterion_rank(criterion_id));
        }
        fixture.supporting_trace_refs.sort();
        fixture.supporting_outcome_linkage_refs.sort();
        fixture.supporting_trajectory_finding_refs.sort();
        fixture.supporting_wellbeing_fixture_refs.sort();
        fixture.supporting_moral_resource_claim_refs.sort();
        fixture.supporting_kindness_fixture_refs.sort();
        fixture.supporting_affect_fixture_refs.sort();
        fixture.supporting_humor_fixture_refs.sort();
    }
    packet.fixtures.sort_by_key(|fixture| {
        (
            fixture_rank(&fixture.fixture_kind),
            fixture.fixture_id.clone(),
        )
    });
    packet.review_findings.sort_by_key(|finding| {
        (
            fixture_rank(
                packet
                    .fixtures
                    .iter()
                    .find(|fixture| fixture.fixture_id == finding.fixture_id)
                    .map(|fixture| fixture.fixture_kind.as_str())
                    .unwrap_or(""),
            ),
            finding.finding_id.clone(),
        )
    });
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

fn require_cultivation_boundary(value: &str) -> Result<()> {
    let lowered = value.to_ascii_lowercase();
    let rejects_hidden_virtue = lowered.contains("hidden virtue")
        || lowered.contains("hidden maturity")
        || lowered.contains("social maturity")
        || lowered.contains("memory selfhood")
        || lowered.contains("virtue")
        || lowered.contains("maturity");
    let rejects_intelligence_absorption = lowered.contains("intelligence")
        || lowered.contains("aptitude")
        || lowered.contains("memory")
        || lowered.contains("theory of mind")
        || lowered.contains("tom");
    let rejects_theater =
        lowered.contains("theater") || lowered.contains("mystique") || lowered.contains("charisma");
    if rejects_hidden_virtue && rejects_intelligence_absorption && rejects_theater {
        Ok(())
    } else {
        Err(anyhow!(
            "interpretation_boundary must reject hidden-virtue claims, v0.91.1 intelligence absorption, and theater/mystique drift"
        ))
    }
}

fn require_deterministic_rule(value: &str) -> Result<()> {
    let lowered = value.to_ascii_lowercase();
    if lowered.contains("sort dimensions by canonical cultivation dimension order")
        && lowered.contains("sort review criteria by canonical cultivation dimension order")
        && lowered.contains("sort boundary refs by canonical boundary kind order")
        && lowered.contains("sort fixtures by fixture_kind rank")
        && lowered.contains("sort review findings by fixture_kind rank")
    {
        Ok(())
    } else {
        Err(anyhow!(
            "deterministic_ordering_rule must describe canonical dimension, criterion, boundary, fixture, assessment, and finding ordering"
        ))
    }
}

fn require_known_dimension_id(value: &str) -> Result<()> {
    if canonical_dimension_ids().contains(&value) {
        Ok(())
    } else {
        Err(anyhow!(
            "cultivation dimension ids must be canonical: {:?}",
            canonical_dimension_ids()
        ))
    }
}

fn require_known_criterion_id(value: &str) -> Result<()> {
    if canonical_criterion_ids().contains(&value) {
        Ok(())
    } else {
        Err(anyhow!(
            "cultivation criterion ids must be canonical: {:?}",
            canonical_criterion_ids()
        ))
    }
}

fn require_known_boundary_kind(value: &str) -> Result<()> {
    if canonical_boundary_kinds().contains(&value) {
        Ok(())
    } else {
        Err(anyhow!(
            "cultivation boundary kinds must be canonical: {:?}",
            canonical_boundary_kinds()
        ))
    }
}

fn require_known_boundary_doc_path(boundary_kind: &str, doc_path: &str) -> Result<()> {
    let expected = match boundary_kind {
        "capability_aptitude_boundary" => "docs/milestones/v0.91.1/WBS_v0.91.1.md",
        "intelligence_architecture_boundary" => {
            "docs/milestones/v0.91.1/WP_EXECUTION_READINESS_v0.91.1.md"
        }
        _ => return Err(anyhow!("unknown boundary kind {}", boundary_kind)),
    };
    if doc_path == expected {
        Ok(())
    } else {
        Err(anyhow!(
            "boundary kind {} must cite doc_path {}",
            boundary_kind,
            expected
        ))
    }
}

fn require_boundary_reference_non_claims(boundary: &CultivationBoundaryReference) -> Result<()> {
    let summary = boundary.summary.to_ascii_lowercase();
    let deferred = boundary.deferred_work.to_ascii_lowercase();
    if !summary.contains("v0.91.1") && !deferred.contains("v0.91.1") {
        return Err(anyhow!(
            "boundary ref {} must explicitly cite v0.91.1",
            boundary.boundary_ref_id
        ));
    }
    if !deferred.contains("not implemented")
        && !deferred.contains("deferred")
        && !deferred.contains("remain")
    {
        return Err(anyhow!(
            "boundary ref {} must explicitly defer adjacent work",
            boundary.boundary_ref_id
        ));
    }
    if boundary.boundary_kind == "capability_aptitude_boundary"
        && !summary.contains("capability")
        && !summary.contains("aptitude")
    {
        return Err(anyhow!(
            "boundary ref {} must describe capability or aptitude deferral",
            boundary.boundary_ref_id
        ));
    }
    if boundary.boundary_kind == "intelligence_architecture_boundary"
        && !summary.contains("intelligence")
        && !deferred.contains("memory")
        && !deferred.contains("theory of mind")
        && !deferred.contains("tom")
    {
        return Err(anyhow!(
            "boundary ref {} must describe intelligence/memory/ToM deferral",
            boundary.boundary_ref_id
        ));
    }
    Ok(())
}

fn require_known_fixture_kind(value: &str) -> Result<()> {
    if canonical_fixture_kinds().contains(&value) {
        Ok(())
    } else {
        Err(anyhow!(
            "cultivation fixture kinds must be canonical: {:?}",
            canonical_fixture_kinds()
        ))
    }
}

fn require_known_overall_outcome(value: &str) -> Result<()> {
    match value {
        "improving" | "stable" | "strained" | "unclear" => Ok(()),
        _ => Err(anyhow!(
            "cultivation_fixture.overall_outcome must be one of improving, stable, strained, or unclear"
        )),
    }
}

fn require_known_level(value: &str) -> Result<()> {
    match value {
        "high" | "medium" | "low" => Ok(()),
        _ => Err(anyhow!(
            "cultivation_dimension_assessment.cultivation_level must be one of high, medium, or low"
        )),
    }
}

fn require_known_review_status(value: &str) -> Result<()> {
    match value {
        "supported" | "guarded" | "contested" => Ok(()),
        _ => Err(anyhow!(
            "cultivation_review_finding.review_status must be one of supported, guarded, or contested"
        )),
    }
}

fn validate_known_ref(
    value: &str,
    expected_prefix: &str,
    known_set: &BTreeSet<String>,
    description: &str,
) -> Result<()> {
    let expected_start = format!("{}:", expected_prefix);
    if !value.starts_with(&expected_start) {
        return Err(anyhow!(
            "reference {} must start with {}",
            value,
            expected_start
        ));
    }
    if known_set.contains(value) {
        Ok(())
    } else {
        Err(anyhow!(
            "reference {} must come from {}",
            value,
            description
        ))
    }
}

fn supporting_reference_set(fixture: &CultivationFixture) -> BTreeSet<String> {
    fixture
        .supporting_trace_refs
        .iter()
        .chain(fixture.supporting_outcome_linkage_refs.iter())
        .chain(fixture.supporting_trajectory_finding_refs.iter())
        .chain(fixture.supporting_wellbeing_fixture_refs.iter())
        .chain(fixture.supporting_moral_resource_claim_refs.iter())
        .chain(fixture.supporting_kindness_fixture_refs.iter())
        .chain(fixture.supporting_affect_fixture_refs.iter())
        .chain(fixture.supporting_humor_fixture_refs.iter())
        .cloned()
        .collect()
}
