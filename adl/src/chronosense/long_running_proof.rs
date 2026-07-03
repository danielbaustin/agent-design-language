//! End-to-end Chronosense long-running continuity proof.

use std::path::{Path, PathBuf};

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};

use super::{
    build_temporal_causality_trace_review, ChronosenseRuntimeService,
    ChronosenseRuntimeServiceConfig, TemporalCausalityTraceReviewArtifact,
    CHRONOSENSE_EVENT_ANCHOR_SCHEMA, COMMITMENT_DEADLINE_SCHEMA,
};
use crate::{
    obsmem_adapter::ObsMemAdapter,
    obsmem_contract::{
        MemoryCitation, MemoryQueryResult, MemoryTemporalAnchor, MemoryTemporalQuery,
        MemoryTraceRef, MemoryWriteRequest, ObsMemClient, OBSMEM_CONTRACT_VERSION,
    },
    obsmem_store::FileObsMemClient,
    scheduler::{
        schedule_economics_bundle, ChronosenseCommitmentSchedulingSignalV1,
        ChronosenseCommitmentStatusV1, ChronosenseDeadlineFrameV1, ChronosenseDeadlinePostureV1,
        ChronosenseSchedulerContextV1, CognitiveSchedulerLaneV1, SchedulerCostLevelV1,
        SchedulerDependencyPostureV1, SchedulerEconomicsInputBundleV1, SchedulerEconomicsInputV1,
        SchedulerEffortV1, SchedulerExpectedValueV1, SchedulerParallelismPotentialV1,
        SchedulerPressureLevelV1, SchedulerRiskLevelV1, SchedulerTaskTypeV1, SchedulerUrgencyV1,
        SCHEDULER_ECONOMICS_INPUT_BUNDLE_SCHEMA_V1, SCHEDULER_ECONOMICS_INPUT_SCHEMA_V1,
    },
    trace_schema_v1::{
        validate_trace_event_envelope_v1, TraceActorTypeV1, TraceActorV1,
        TraceChronosenseClockStackV1, TraceDecisionContextV1, TraceEventEnvelopeV1,
        TraceEventTypeV1, TraceEventV1, TraceScopeLevelV1, TraceScopeV1, TraceTemporalAnchorV1,
    },
};

pub const LONG_RUNNING_CONTEXT_CONTINUITY_PROOF_SCHEMA: &str =
    "chronosense.long_running_context_continuity_proof.v1";
pub const LONG_RUNNING_CONTEXT_CONTINUITY_TRACE_ARTIFACT_SCHEMA: &str =
    "chronosense.long_running_context_continuity_trace_artifact.v1";
pub const LONG_RUNNING_CONTEXT_CONTINUITY_PROOF_PATH: &str =
    ".adl/state/chronosense/long_running_context_continuity_proof_v1.json";
pub const LONG_RUNNING_CONTEXT_CONTINUITY_RUNTIME_STATE_PATH: &str =
    ".adl/state/chronosense/long_running_runtime_state_v1.json";
pub const LONG_RUNNING_CONTEXT_CONTINUITY_TRACE_ARTIFACT_REF: &str =
    "artifacts/run-chronosense-long-running-context/chronosense_long_running_context_continuity_proof_v1.json";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LongRunningContextContinuityProof {
    pub schema_version: String,
    pub proof_id: String,
    pub continuity_id: String,
    pub runtime_started_epoch_ms: u128,
    pub interruption_epoch_ms: u128,
    pub resumed_epoch_ms: u128,
    pub final_epoch_ms: u128,
    pub runtime_state_rel_path: String,
    pub memory_store_rel_path: String,
    pub memory_query_hit_run_ids: Vec<String>,
    pub memory_query_hit_count: usize,
    pub scheduler_recommended_order: Vec<String>,
    pub scheduler_selected_lane: String,
    pub scheduler_dependency_status: String,
    pub scheduler_reason: String,
    pub trace_event_count: usize,
    pub trace_run_id: String,
    pub trace_validated: bool,
    pub trace_artifact_rel_path: String,
    pub temporal_causality_review_validated: bool,
    pub temporal_causality_sequence_only_count: usize,
    pub temporal_causality_or_dependency_count: usize,
    pub temporal_causality_uncertainty_count: usize,
    pub proof_checks: Vec<ContinuityProofCheck>,
    pub review_surface: String,
    pub claim_boundary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContinuityProofCheck {
    pub check_id: String,
    pub status: String,
    pub evidence: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LongRunningContextContinuityTraceArtifact {
    pub schema_version: String,
    pub proof_id: String,
    pub continuity_id: String,
    pub trace: TraceEventEnvelopeV1,
    pub temporal_causality_review: TemporalCausalityTraceReviewArtifact,
    pub review_surface: String,
    pub claim_boundary: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct LongRunningContextContinuityBuild {
    proof: LongRunningContextContinuityProof,
    trace_artifact: LongRunningContextContinuityTraceArtifact,
}

pub fn build_long_running_context_continuity_proof(
    proof_root: impl AsRef<Path>,
) -> Result<LongRunningContextContinuityProof> {
    Ok(build_long_running_context_continuity_output(proof_root)?.proof)
}

fn build_long_running_context_continuity_output(
    proof_root: impl AsRef<Path>,
) -> Result<LongRunningContextContinuityBuild> {
    let proof_root = proof_root.as_ref();
    let continuity_id = "chronosense-proof-chain-a";
    let started_epoch_ms = 1_800_000_000_000_u128;
    let interruption_epoch_ms = started_epoch_ms + 3_600_000;
    let resumed_epoch_ms = started_epoch_ms + 28_800_000;
    let final_epoch_ms = resumed_epoch_ms + 600_000;
    let runtime_config = ChronosenseRuntimeServiceConfig::utc(started_epoch_ms);
    let service = ChronosenseRuntimeService::new(runtime_config.clone())?;
    let start_clock = service.capture_epoch_millis(started_epoch_ms)?;
    let interruption_clock = service.capture_epoch_millis(interruption_epoch_ms)?;
    persist_runtime_state(proof_root, &runtime_config)?;
    let resumed_service = rehydrate_runtime_service_from_state(proof_root)?;
    let resumed_clock = resumed_service.capture_epoch_millis(resumed_epoch_ms)?;
    let final_clock = resumed_service.capture_epoch_millis(final_epoch_ms)?;

    let store_rel_path = ".adl/state/chronosense/long_running_obsmem_store_v1.json";
    let store_path = proof_root.join(store_rel_path);
    let store = FileObsMemClient::new(&store_path);
    store.write_entry(&memory_write_request(
        "chronosense-run-before-interruption",
        "pre-interruption continuity snapshot retained the active deadline and context handoff",
        started_epoch_ms,
        2,
        continuity_id,
    ))?;
    store.write_entry(&memory_write_request(
        "chronosense-run-after-resume",
        "resumed context recovered the continuity chain and active commitment before scheduling",
        resumed_epoch_ms,
        4,
        continuity_id,
    ))?;
    let adapter = ObsMemAdapter::new(FileObsMemClient::new(&store_path));
    let memory_query = adapter.query_temporal(
        Some("wf-chronosense-proof"),
        None,
        &["chronosense".to_string(), "continuity".to_string()],
        MemoryTemporalQuery {
            interval_start_epoch_ms: Some(started_epoch_ms),
            interval_end_epoch_ms: Some(final_epoch_ms),
            continuity_id: Some(continuity_id.to_string()),
            ..Default::default()
        },
        10,
    )?;
    let memory_query_hit_run_ids = memory_query_run_ids(&memory_query);

    let scheduler_bundle = scheduler_bundle_with_chronosense_context();
    let scheduler_plan = schedule_economics_bundle(&scheduler_bundle)?;
    let scheduler_decision = scheduler_plan
        .decisions
        .iter()
        .find(|decision| decision.task_id == "resume-context-proof")
        .ok_or_else(|| anyhow!("scheduler did not produce resume-context-proof decision"))?;

    let trace = trace_envelope(
        &start_clock,
        &interruption_clock,
        &resumed_clock,
        &final_clock,
        continuity_id,
    )?;
    validate_trace_event_envelope_v1(&trace)?;
    let causality_review = build_temporal_causality_trace_review(&trace)?;

    let proof = LongRunningContextContinuityProof {
        schema_version: LONG_RUNNING_CONTEXT_CONTINUITY_PROOF_SCHEMA.to_string(),
        proof_id: "chronosense-long-running-context-proof-v1".to_string(),
        continuity_id: continuity_id.to_string(),
        runtime_started_epoch_ms: started_epoch_ms,
        interruption_epoch_ms,
        resumed_epoch_ms,
        final_epoch_ms,
        runtime_state_rel_path: LONG_RUNNING_CONTEXT_CONTINUITY_RUNTIME_STATE_PATH.to_string(),
        memory_store_rel_path: store_rel_path.to_string(),
        memory_query_hit_count: memory_query.hits.len(),
        memory_query_hit_run_ids,
        scheduler_recommended_order: scheduler_plan.recommended_order,
        scheduler_selected_lane: format!("{:?}", scheduler_decision.selected_lane),
        scheduler_dependency_status: format!("{:?}", scheduler_decision.dependency_status),
        scheduler_reason: scheduler_decision.reason.clone(),
        trace_event_count: trace.events.len(),
        trace_run_id: trace.events[0].run_id.clone(),
        trace_validated: true,
        trace_artifact_rel_path: LONG_RUNNING_CONTEXT_CONTINUITY_TRACE_ARTIFACT_REF.to_string(),
        temporal_causality_review_validated: true,
        temporal_causality_sequence_only_count: causality_review.sequence_only_count,
        temporal_causality_or_dependency_count: causality_review.causal_or_dependency_count,
        temporal_causality_uncertainty_count: causality_review.uncertainty_count,
        proof_checks: vec![
            proof_check(
                "runtime_survived_resume",
                resumed_service.config().started_at_epoch_ms == service.config().started_at_epoch_ms
                    && resumed_clock.monotonic_elapsed_ms > interruption_clock.monotonic_elapsed_ms
                    && final_clock.monotonic_elapsed_ms > resumed_clock.monotonic_elapsed_ms,
                "Chronosense runtime state was persisted, rehydrated into a new service, and advanced across resume captures",
            ),
            proof_check(
                "temporal_memory_query",
                memory_query.hits.len() == 2,
                "file-backed ObsMem temporal query returned both continuity-chain records",
            ),
            proof_check(
                "commitment_scheduler_preserved_deadline",
                scheduler_decision.selected_lane == CognitiveSchedulerLaneV1::Governor
                    && scheduler_decision.dependency_status == SchedulerDependencyPostureV1::Partial,
                "Chronosense commitment signal made due commitment review-visible in scheduler decision",
            ),
            proof_check(
                "trace_review_surface",
                trace.events.len() == 5
                    && causality_review.sequence_only_count > 0
                    && causality_review.causal_or_dependency_count > 0,
                "trace envelope and temporal-causality review artifact validated with Chronosense event anchors across resume path",
            ),
        ],
        review_surface:
            "runtime clock stack + file-backed ObsMem temporal query + scheduler Chronosense context + trace envelope validation"
                .to_string(),
        claim_boundary:
            "bounded local deterministic proof; does not claim distributed memory, live AEE agent execution, or wall-clock soak duration"
                .to_string(),
    };
    validate_long_running_context_continuity_proof(&proof)?;
    let trace_artifact = LongRunningContextContinuityTraceArtifact {
        schema_version: LONG_RUNNING_CONTEXT_CONTINUITY_TRACE_ARTIFACT_SCHEMA.to_string(),
        proof_id: proof.proof_id.clone(),
        continuity_id: proof.continuity_id.clone(),
        trace,
        temporal_causality_review: causality_review,
        review_surface: proof.review_surface.clone(),
        claim_boundary: proof.claim_boundary.clone(),
    };
    validate_long_running_context_continuity_trace_artifact(&trace_artifact, &proof)?;
    Ok(LongRunningContextContinuityBuild {
        proof,
        trace_artifact,
    })
}

pub fn write_long_running_context_continuity_proof(
    proof_root: impl AsRef<Path>,
) -> Result<PathBuf> {
    let proof_root = proof_root.as_ref();
    let output = build_long_running_context_continuity_output(proof_root)?;
    let output_path = proof_root.join(LONG_RUNNING_CONTEXT_CONTINUITY_PROOF_PATH);
    if let Some(parent) = output_path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("create proof directory '{}'", parent.display()))?;
    }
    let bytes = serde_json::to_vec_pretty(&output.proof)?;
    std::fs::write(&output_path, bytes)
        .with_context(|| format!("write proof '{}'", output_path.display()))?;
    let artifact_path = proof_root.join(LONG_RUNNING_CONTEXT_CONTINUITY_TRACE_ARTIFACT_REF);
    if let Some(parent) = artifact_path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("create trace artifact directory '{}'", parent.display()))?;
    }
    let artifact_bytes = serde_json::to_vec_pretty(&output.trace_artifact)?;
    std::fs::write(&artifact_path, artifact_bytes)
        .with_context(|| format!("write trace artifact '{}'", artifact_path.display()))?;
    Ok(output_path)
}

pub fn validate_long_running_context_continuity_trace_artifact(
    artifact: &LongRunningContextContinuityTraceArtifact,
    proof: &LongRunningContextContinuityProof,
) -> Result<()> {
    if artifact.schema_version != LONG_RUNNING_CONTEXT_CONTINUITY_TRACE_ARTIFACT_SCHEMA {
        return Err(anyhow!(
            "unsupported long-running context continuity trace artifact schema '{}'",
            artifact.schema_version
        ));
    }
    if artifact.proof_id != proof.proof_id || artifact.continuity_id != proof.continuity_id {
        return Err(anyhow!(
            "trace artifact identity does not match continuity proof"
        ));
    }
    validate_trace_event_envelope_v1(&artifact.trace)?;
    if artifact.trace.events.len() != proof.trace_event_count {
        return Err(anyhow!("trace artifact event count does not match proof"));
    }
    if artifact.temporal_causality_review.run_id != proof.trace_run_id {
        return Err(anyhow!(
            "trace artifact causality review run id does not match proof"
        ));
    }
    if artifact.temporal_causality_review.sequence_only_count
        != proof.temporal_causality_sequence_only_count
        || artifact
            .temporal_causality_review
            .causal_or_dependency_count
            != proof.temporal_causality_or_dependency_count
        || artifact.temporal_causality_review.uncertainty_count
            != proof.temporal_causality_uncertainty_count
    {
        return Err(anyhow!(
            "trace artifact causality review counts do not match proof"
        ));
    }
    Ok(())
}

pub fn validate_long_running_context_continuity_proof(
    proof: &LongRunningContextContinuityProof,
) -> Result<()> {
    if proof.schema_version != LONG_RUNNING_CONTEXT_CONTINUITY_PROOF_SCHEMA {
        return Err(anyhow!(
            "unsupported long-running context continuity proof schema '{}'",
            proof.schema_version
        ));
    }
    if proof.continuity_id.trim().is_empty() {
        return Err(anyhow!("continuity_id is required"));
    }
    if !(proof.runtime_started_epoch_ms < proof.interruption_epoch_ms
        && proof.interruption_epoch_ms < proof.resumed_epoch_ms
        && proof.resumed_epoch_ms < proof.final_epoch_ms)
    {
        return Err(anyhow!(
            "long-running proof requires start < interruption < resume < final"
        ));
    }
    if proof.memory_query_hit_run_ids
        != [
            "chronosense-run-before-interruption".to_string(),
            "chronosense-run-after-resume".to_string(),
        ]
    {
        return Err(anyhow!(
            "temporal memory query did not preserve continuity order"
        ));
    }
    if proof.memory_query_hit_count != proof.memory_query_hit_run_ids.len() {
        return Err(anyhow!(
            "temporal memory query hit count does not match recorded hit ids"
        ));
    }
    if Path::new(&proof.runtime_state_rel_path).is_absolute() {
        return Err(anyhow!(
            "runtime_state_rel_path must be repository-relative"
        ));
    }
    if Path::new(&proof.memory_store_rel_path).is_absolute() {
        return Err(anyhow!("memory_store_rel_path must be repository-relative"));
    }
    if !proof
        .scheduler_recommended_order
        .iter()
        .any(|task_id| task_id == "resume-context-proof")
    {
        return Err(anyhow!(
            "scheduler proof missing resume-context-proof decision"
        ));
    }
    if proof.scheduler_selected_lane != "Governor" {
        return Err(anyhow!(
            "scheduler proof must preserve governor review lane for due commitment"
        ));
    }
    if proof.scheduler_dependency_status != "Partial" {
        return Err(anyhow!(
            "scheduler proof must preserve partial dependency status for due commitment"
        ));
    }
    if proof.trace_event_count < 5 || !proof.trace_validated {
        return Err(anyhow!("trace review surface was not validated"));
    }
    if proof.trace_run_id != "run-chronosense-long-running-context" {
        return Err(anyhow!("trace run id does not match continuity proof run"));
    }
    if proof.trace_artifact_rel_path != LONG_RUNNING_CONTEXT_CONTINUITY_TRACE_ARTIFACT_REF {
        return Err(anyhow!(
            "trace artifact path does not match validated artifact ref"
        ));
    }
    if !proof.temporal_causality_review_validated
        || proof.temporal_causality_sequence_only_count == 0
        || proof.temporal_causality_or_dependency_count < 4
    {
        return Err(anyhow!(
            "temporal causality trace review did not validate succession and dependency evidence"
        ));
    }
    if proof
        .proof_checks
        .iter()
        .any(|check| check.status != "pass")
    {
        return Err(anyhow!("all long-running proof checks must pass"));
    }
    Ok(())
}

fn persist_runtime_state(
    proof_root: &Path,
    config: &ChronosenseRuntimeServiceConfig,
) -> Result<PathBuf> {
    let path = proof_root.join(LONG_RUNNING_CONTEXT_CONTINUITY_RUNTIME_STATE_PATH);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("create runtime state directory '{}'", parent.display()))?;
    }
    let bytes = serde_json::to_vec_pretty(config)?;
    std::fs::write(&path, bytes)
        .with_context(|| format!("write runtime state '{}'", path.display()))?;
    Ok(path)
}

fn rehydrate_runtime_service_from_state(proof_root: &Path) -> Result<ChronosenseRuntimeService> {
    let path = proof_root.join(LONG_RUNNING_CONTEXT_CONTINUITY_RUNTIME_STATE_PATH);
    let bytes =
        std::fs::read(&path).with_context(|| format!("read runtime state '{}'", path.display()))?;
    let config: ChronosenseRuntimeServiceConfig = serde_json::from_slice(&bytes)
        .with_context(|| format!("parse runtime state '{}'", path.display()))?;
    ChronosenseRuntimeService::new(config)
        .with_context(|| format!("rehydrate runtime service from '{}'", path.display()))
}

fn memory_write_request(
    run_id: &str,
    summary: &str,
    effective_epoch_ms: u128,
    event_sequence: usize,
    continuity_id: &str,
) -> MemoryWriteRequest {
    let mut request = MemoryWriteRequest {
        contract_version: OBSMEM_CONTRACT_VERSION,
        run_id: run_id.to_string(),
        workflow_id: "wf-chronosense-proof".to_string(),
        trace_bundle_rel_path: format!(".adl/state/chronosense/{run_id}/trace.json"),
        activation_log_rel_path: format!(".adl/state/chronosense/{run_id}/activation.log"),
        failure_code: None,
        summary: summary.to_string(),
        tags: vec!["continuity".to_string(), "chronosense".to_string()],
        citations: vec![MemoryCitation {
            path: "adl/src/chronosense/long_running_proof.rs".to_string(),
            hash: "source-reviewed".to_string(),
        }],
        trace_event_refs: vec![MemoryTraceRef {
            event_sequence,
            event_kind: "chronosense_continuity".to_string(),
            step_id: Some(format!("step-{event_sequence}")),
            delegation_id: None,
        }],
        temporal_anchor: Some(MemoryTemporalAnchor {
            t_created_epoch_ms: effective_epoch_ms,
            t_observed_epoch_ms: Some(effective_epoch_ms),
            t_effective_epoch_ms: Some(effective_epoch_ms),
            continuity_id: Some(continuity_id.to_string()),
            event_sequence: Some(event_sequence),
        }),
        review_findings: Vec::new(),
        residual_risks: Vec::new(),
        follow_on_refs: Vec::new(),
    };
    request.normalize();
    request
}

fn memory_query_run_ids(result: &MemoryQueryResult) -> Vec<String> {
    result.hits.iter().map(|hit| hit.run_id.clone()).collect()
}

fn scheduler_bundle_with_chronosense_context() -> SchedulerEconomicsInputBundleV1 {
    SchedulerEconomicsInputBundleV1 {
        schema_version: SCHEDULER_ECONOMICS_INPUT_BUNDLE_SCHEMA_V1.to_string(),
        source_doc_ref: ".adl/state/chronosense/long_running_context_continuity_proof_v1.json"
            .to_string(),
        included_concepts: vec![
            "chronosense_commitment_signal".to_string(),
            "resume_context_deadline".to_string(),
        ],
        deferred_concepts: vec!["calendar_integration".to_string()],
        chronosense_context: Some(ChronosenseSchedulerContextV1 {
            schema_version: crate::scheduler::CHRONOSENSE_SCHEDULER_CONTEXT_SCHEMA_V1.to_string(),
            contract_schema_version: COMMITMENT_DEADLINE_SCHEMA.to_string(),
            generated_from: "long_running_context_continuity_proof".to_string(),
            signals: vec![ChronosenseCommitmentSchedulingSignalV1 {
                task_id: "resume-context-proof".to_string(),
                commitment_id: "commitment-resume-context".to_string(),
                status: ChronosenseCommitmentStatusV1::Active,
                deadline_posture: ChronosenseDeadlinePostureV1::Due,
                deadline_frame: Some(ChronosenseDeadlineFrameV1::ContinuityRelative),
                temporal_urgency: SchedulerUrgencyV1::Immediate,
                fulfillment_ready: true,
                review_required: true,
                reason: Some(
                    "resumed context must review active commitment before next work".to_string(),
                ),
            }],
        }),
        role_provider_context: None,
        model_suitability_context: None,
        cheapest_validated_outcome_policy: None,
        inputs: vec![SchedulerEconomicsInputV1 {
            schema_version: SCHEDULER_ECONOMICS_INPUT_SCHEMA_V1.to_string(),
            task_id: "resume-context-proof".to_string(),
            task_type: SchedulerTaskTypeV1::Review,
            estimated_effort: SchedulerEffortV1::Small,
            estimated_validation_cost: SchedulerCostLevelV1::Low,
            estimated_coordination_cost: SchedulerCostLevelV1::Low,
            risk_level: SchedulerRiskLevelV1::Medium,
            expected_value: SchedulerExpectedValueV1::High,
            urgency: SchedulerUrgencyV1::Normal,
            dependency_posture: SchedulerDependencyPostureV1::Clear,
            parallelism_potential: SchedulerParallelismPotentialV1::Serial,
            premium_capacity_pressure: SchedulerPressureLevelV1::Low,
            governor_attention_pressure: SchedulerPressureLevelV1::Low,
            confidence: crate::scheduler::SchedulerConfidenceV1::High,
            human_required: false,
            dependencies: Vec::new(),
            required_capabilities: vec!["chronosense".to_string()],
            manual_override: None,
            claim_boundary: "bounded local continuity proof, not_exact wall-clock soak".to_string(),
        }],
    }
}

fn trace_envelope(
    start_clock: &super::ChronosenseClockStack,
    interruption_clock: &super::ChronosenseClockStack,
    resumed_clock: &super::ChronosenseClockStack,
    final_clock: &super::ChronosenseClockStack,
    continuity_id: &str,
) -> Result<TraceEventEnvelopeV1> {
    Ok(TraceEventEnvelopeV1 {
        schema_version: "trace.v2".to_string(),
        chronosense_clock_stack: Some(TraceChronosenseClockStackV1 {
            schema_version: crate::chronosense::CHRONOSENSE_CLOCK_STACK_SCHEMA.to_string(),
            utc_timestamp_rfc3339: final_clock.utc_timestamp_rfc3339.clone(),
            local_timestamp_rfc3339: final_clock.local_timestamp_rfc3339.clone(),
            timezone: final_clock.timezone.clone(),
            utc_offset: final_clock.utc_offset.clone(),
            lifetime_elapsed_ms: u64::try_from(final_clock.lifetime_elapsed_ms)?,
            monotonic_elapsed_ms: u64::try_from(final_clock.monotonic_elapsed_ms)?,
            reference_frames: final_clock.reference_frames.clone(),
        }),
        events: vec![
            trace_event(
                "event-run-start",
                TraceEventTypeV1::RunStart,
                1,
                0,
                None,
                start_clock,
                continuity_id,
            )?,
            trace_event(
                "event-memory-write-before-interruption",
                TraceEventTypeV1::MemoryWrite,
                2,
                3_600_000,
                Some("span-event-run-start"),
                interruption_clock,
                continuity_id,
            )?,
            trace_event(
                "event-memory-read-after-resume",
                TraceEventTypeV1::MemoryRead,
                3,
                25_200_000,
                Some("span-event-memory-write-before-interruption"),
                resumed_clock,
                continuity_id,
            )?,
            trace_event(
                "event-scheduler-review",
                TraceEventTypeV1::Decision,
                4,
                600_000,
                Some("span-event-memory-read-after-resume"),
                final_clock,
                continuity_id,
            )?,
            trace_event(
                "event-run-end",
                TraceEventTypeV1::RunEnd,
                5,
                0,
                Some("span-event-scheduler-review"),
                final_clock,
                continuity_id,
            )?,
        ],
    })
}

fn trace_event(
    event_id: &str,
    event_type: TraceEventTypeV1,
    event_sequence: u64,
    prior_event_delta_ms: u64,
    parent_span_id: Option<&str>,
    clock: &super::ChronosenseClockStack,
    continuity_id: &str,
) -> Result<TraceEventV1> {
    let decision_context = if event_type == TraceEventTypeV1::Decision {
        Some(TraceDecisionContextV1 {
            context: "chronosense long-running context continuity proof".to_string(),
            outcome: "resume commitment review required before next work".to_string(),
            rationale: Some(
                "scheduler consumed active commitment/deadline signal after context resume"
                    .to_string(),
            ),
        })
    } else {
        None
    };

    Ok(TraceEventV1 {
        event_id: event_id.to_string(),
        timestamp: clock.utc_timestamp_rfc3339.clone(),
        temporal_anchor: Some(TraceTemporalAnchorV1 {
            schema_version: CHRONOSENSE_EVENT_ANCHOR_SCHEMA.to_string(),
            utc_timestamp_rfc3339: clock.utc_timestamp_rfc3339.clone(),
            local_timestamp_rfc3339: clock.local_timestamp_rfc3339.clone(),
            timezone: clock.timezone.clone(),
            utc_offset: clock.utc_offset.clone(),
            runtime_lifetime_elapsed_ms: u64::try_from(clock.lifetime_elapsed_ms)?,
            runtime_monotonic_elapsed_ms: u64::try_from(clock.monotonic_elapsed_ms)?,
            event_sequence,
            prior_event_delta_ms,
            reference_frames: vec![
                "utc_epoch_millis".to_string(),
                "local_civil_time".to_string(),
                "runtime_lifetime".to_string(),
                "runtime_monotonic_elapsed".to_string(),
                "event_sequence".to_string(),
            ],
        }),
        event_type,
        trace_id: "trace-chronosense-long-running-context".to_string(),
        run_id: "run-chronosense-long-running-context".to_string(),
        span_id: format!("span-{event_id}"),
        parent_span_id: parent_span_id.map(str::to_string),
        actor: TraceActorV1 {
            r#type: TraceActorTypeV1::Agent,
            id: "agent.chronosense-proof".to_string(),
        },
        scope: TraceScopeV1 {
            level: TraceScopeLevelV1::Run,
            name: format!("long-running-context:{continuity_id}"),
        },
        inputs_ref: None,
        outputs_ref: None,
        artifact_ref: Some(LONG_RUNNING_CONTEXT_CONTINUITY_TRACE_ARTIFACT_REF.to_string()),
        decision_context,
        provider: None,
        error: None,
        contract_validation: None,
        governance: None,
        redaction: None,
    })
}

fn proof_check(check_id: &str, passed: bool, evidence: &str) -> ContinuityProofCheck {
    ContinuityProofCheck {
        check_id: check_id.to_string(),
        status: if passed { "pass" } else { "fail" }.to_string(),
        evidence: evidence.to_string(),
    }
}
