//! Chronosense temporal causality contracts.
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use super::TEMPORAL_CAUSALITY_EXPLANATION_SCHEMA;
use crate::trace_schema_v1::{
    validate_trace_event_envelope_v1, TraceEventEnvelopeV1, TraceEventV1,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CausalRelationContract {
    pub relation_types: Vec<String>,
    pub sequence_boundary_rule: String,
    pub dependency_evidence_requirements: Vec<String>,
    pub uncertainty_classes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExplanationSurfaceContract {
    pub required_fields: Vec<String>,
    pub citation_requirements: Vec<String>,
    pub non_goals: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExplanationFixture {
    pub scenario: String,
    pub relation_type: String,
    pub confidence: String,
    pub explanation_note: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TemporalCausalityExplanationContract {
    pub schema_version: String,
    pub owned_runtime_surfaces: Vec<String>,
    pub causal_relations: CausalRelationContract,
    pub explanation_surface: ExplanationSurfaceContract,
    pub explanation_fixtures: Vec<ExplanationFixture>,
    pub proof_fixture_hooks: Vec<String>,
    pub proof_hook_command: String,
    pub proof_hook_output_path: String,
    pub scope_boundary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TemporalCausalityTraceReviewArtifact {
    pub schema_version: String,
    pub source_trace_schema_version: String,
    pub run_id: String,
    pub explanations: Vec<TemporalCausalityTraceExplanation>,
    pub sequence_only_count: usize,
    pub causal_or_dependency_count: usize,
    pub uncertainty_count: usize,
    pub review_notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TemporalCausalityTraceExplanation {
    pub source_event_id: String,
    pub target_event_id: String,
    pub source_event_sequence: Option<u64>,
    pub target_event_sequence: Option<u64>,
    pub relation_type: String,
    pub confidence: String,
    pub temporal_delta_ms: Option<u64>,
    pub evidence_refs: Vec<String>,
    pub explanation_note: String,
}

impl TemporalCausalityExplanationContract {
    pub fn v1() -> Self {
        Self {
            schema_version: TEMPORAL_CAUSALITY_EXPLANATION_SCHEMA.to_string(),
            owned_runtime_surfaces: vec![
                "adl::chronosense::TemporalCausalityExplanationContract".to_string(),
                "adl::chronosense::CausalRelationContract".to_string(),
                "adl::chronosense::ExplanationSurfaceContract".to_string(),
                "adl::chronosense::ExplanationFixture".to_string(),
                "adl::chronosense::TemporalQueryRetrievalContract".to_string(),
                "adl::chronosense::CommitmentDeadlineContract".to_string(),
                "adl identity causality".to_string(),
            ],
            causal_relations: CausalRelationContract {
                relation_types: vec![
                    "temporal_succession".to_string(),
                    "declared_dependency".to_string(),
                    "causal_contribution".to_string(),
                    "unknown_relation".to_string(),
                ],
                sequence_boundary_rule:
                    "sequence alone is insufficient evidence for causality".to_string(),
                dependency_evidence_requirements: vec![
                    "cite source event or condition".to_string(),
                    "cite target event or state".to_string(),
                    "name explicit relation type".to_string(),
                    "record bounded confidence or uncertainty".to_string(),
                ],
                uncertainty_classes: vec![
                    "high".to_string(),
                    "medium".to_string(),
                    "low".to_string(),
                    "unknown".to_string(),
                ],
            },
            explanation_surface: ExplanationSurfaceContract {
                required_fields: vec![
                    "source_event_or_condition".to_string(),
                    "target_event_or_state".to_string(),
                    "relation_type".to_string(),
                    "confidence".to_string(),
                    "explanation_note".to_string(),
                ],
                citation_requirements: vec![
                    "cite dependency or state-change evidence".to_string(),
                    "cite prior temporal anchor when available".to_string(),
                    "cite uncertainty explicitly when causal evidence is incomplete".to_string(),
                ],
                non_goals: vec![
                    "probabilistic global causal graphs".to_string(),
                    "scientific causal inference engines".to_string(),
                    "overclaiming causality from ordering alone".to_string(),
                ],
            },
            explanation_fixtures: vec![
                ExplanationFixture {
                    scenario: "deadline_miss_after_interruption".to_string(),
                    relation_type: "causal_contribution".to_string(),
                    confidence: "medium".to_string(),
                    explanation_note:
                        "interruption preserved continuity boundary and contributed to missed commitment visibility"
                            .to_string(),
                },
                ExplanationFixture {
                    scenario: "adjacent_events_without_dependency".to_string(),
                    relation_type: "unknown_relation".to_string(),
                    confidence: "unknown".to_string(),
                    explanation_note:
                        "adjacent temporal order is recorded, but no dependency evidence is present"
                            .to_string(),
                },
            ],
            proof_fixture_hooks: vec![
                "adl::chronosense::TemporalCausalityExplanationContract::v1".to_string(),
                "adl identity causality --out .adl/state/temporal_causality_explanation_v1.json"
                    .to_string(),
            ],
            proof_hook_command:
                "adl identity causality --out .adl/state/temporal_causality_explanation_v1.json"
                    .to_string(),
            proof_hook_output_path: ".adl/state/temporal_causality_explanation_v1.json"
                .to_string(),
            scope_boundary:
                "bounded causal-link and explanation semantics only; full causal inference, planning policy, and global explanation graphs remain downstream work"
                    .to_string(),
        }
    }
}

pub fn build_temporal_causality_trace_review(
    envelope: &TraceEventEnvelopeV1,
) -> Result<TemporalCausalityTraceReviewArtifact> {
    validate_trace_event_envelope_v1(envelope)?;
    let contract = TemporalCausalityExplanationContract::v1();
    let ordered_events = events_in_temporal_review_order(envelope);
    if ordered_events.len() < 2 {
        return Err(anyhow!(
            "temporal causality review requires at least two trace events"
        ));
    }
    if let Some(event) = ordered_events
        .iter()
        .find(|event| event.temporal_anchor.is_none())
    {
        return Err(anyhow!(
            "temporal causality review requires temporal_anchor on event '{}'",
            event.event_id
        ));
    }

    let mut explanations = Vec::new();
    for window in ordered_events.windows(2) {
        let source = window[0];
        let target = window[1];
        explanations.push(explain_temporal_successor_relation(
            source, target, &contract,
        ));
    }
    for (target_index, target) in ordered_events.iter().enumerate() {
        explanations.extend(explain_explicit_evidence_relations(
            &ordered_events[..target_index],
            target,
            &contract,
        ));
    }

    let sequence_only_count = explanations
        .iter()
        .filter(|explanation| explanation.relation_type == "temporal_succession")
        .count();
    let causal_or_dependency_count = explanations
        .iter()
        .filter(|explanation| {
            matches!(
                explanation.relation_type.as_str(),
                "declared_dependency" | "causal_contribution"
            )
        })
        .count();
    let uncertainty_count = explanations
        .iter()
        .filter(|explanation| matches!(explanation.confidence.as_str(), "unknown" | "low"))
        .count();

    Ok(TemporalCausalityTraceReviewArtifact {
        schema_version: TEMPORAL_CAUSALITY_EXPLANATION_SCHEMA.to_string(),
        source_trace_schema_version: envelope.schema_version.clone(),
        run_id: ordered_events[0].run_id.clone(),
        explanations,
        sequence_only_count,
        causal_or_dependency_count,
        uncertainty_count,
        review_notes: vec![
            contract.causal_relations.sequence_boundary_rule,
            "reviewers must inspect relation_type, confidence, and evidence_refs before treating sequence as causality".to_string(),
        ],
    })
}

fn events_in_temporal_review_order(envelope: &TraceEventEnvelopeV1) -> Vec<&TraceEventV1> {
    let mut events = envelope.events.iter().collect::<Vec<_>>();
    events.sort_by(|left, right| {
        let left_key = left
            .temporal_anchor
            .as_ref()
            .map(|anchor| (anchor.event_sequence, anchor.runtime_monotonic_elapsed_ms))
            .unwrap_or((0, 0));
        let right_key = right
            .temporal_anchor
            .as_ref()
            .map(|anchor| (anchor.event_sequence, anchor.runtime_monotonic_elapsed_ms))
            .unwrap_or((0, 0));
        left_key
            .cmp(&right_key)
            .then_with(|| left.event_id.cmp(&right.event_id))
    });
    events
}

fn explain_temporal_successor_relation(
    source: &TraceEventV1,
    target: &TraceEventV1,
    contract: &TemporalCausalityExplanationContract,
) -> TemporalCausalityTraceExplanation {
    build_trace_explanation(
        source,
        target,
        "temporal_succession",
        "unknown",
        Vec::new(),
        contract.causal_relations.sequence_boundary_rule.clone(),
    )
}

fn explain_explicit_evidence_relations(
    prior_events: &[&TraceEventV1],
    target: &TraceEventV1,
    contract: &TemporalCausalityExplanationContract,
) -> Vec<TemporalCausalityTraceExplanation> {
    let mut explanations = Vec::new();
    let target_evidence_refs = target
        .governance
        .as_ref()
        .map(|governance| governance.evidence_refs.clone())
        .unwrap_or_default();

    for source in prior_events {
        if target_evidence_refs
            .iter()
            .any(|evidence_ref| evidence_ref == &source.event_id)
        {
            explanations.push(build_trace_explanation(
                source,
                target,
                "causal_contribution",
                "medium",
                target_evidence_refs.clone(),
                format!(
                    "target event cites source event as evidence; {}",
                    contract.causal_relations.dependency_evidence_requirements[3]
                ),
            ));
        }
        if target
            .parent_span_id
            .as_ref()
            .is_some_and(|parent_span_id| parent_span_id == &source.span_id)
        {
            explanations.push(build_trace_explanation(
                source,
                target,
                "declared_dependency",
                "medium",
                vec![format!(
                    "parent_span_id:{}->{}",
                    source.span_id, target.span_id
                )],
                "target event declares the source span as its parent; this is dependency evidence, not global causal proof"
                    .to_string(),
            ));
        }
    }
    explanations
}

fn build_trace_explanation(
    source: &TraceEventV1,
    target: &TraceEventV1,
    relation_type: &str,
    confidence: &str,
    evidence_refs: Vec<String>,
    explanation_note: String,
) -> TemporalCausalityTraceExplanation {
    let source_sequence = source
        .temporal_anchor
        .as_ref()
        .map(|anchor| anchor.event_sequence);
    let target_sequence = target
        .temporal_anchor
        .as_ref()
        .map(|anchor| anchor.event_sequence);
    let temporal_delta_ms = source
        .temporal_anchor
        .as_ref()
        .zip(target.temporal_anchor.as_ref())
        .map(|(source_anchor, target_anchor)| {
            target_anchor
                .runtime_monotonic_elapsed_ms
                .saturating_sub(source_anchor.runtime_monotonic_elapsed_ms)
        });

    TemporalCausalityTraceExplanation {
        source_event_id: source.event_id.clone(),
        target_event_id: target.event_id.clone(),
        source_event_sequence: source_sequence,
        target_event_sequence: target_sequence,
        relation_type: relation_type.to_string(),
        confidence: confidence.to_string(),
        temporal_delta_ms,
        evidence_refs,
        explanation_note,
    }
}
