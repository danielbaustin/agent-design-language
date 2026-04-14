use serde::{Deserialize, Serialize};

use super::TEMPORAL_SCHEMA_V01;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SubjectiveTimeSchema {
    pub narrative_position: String,
    pub integration_window: String,
    pub temporal_gap: String,
    pub experienced_duration: String,
    pub temporal_density: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TemporalAnchorSchema {
    pub observed_at_utc: String,
    pub observed_at_local: String,
    pub agent_age: String,
    pub turn_index: String,
    pub monotonic_order: String,
    pub prior_event_delta: String,
    pub temporal_confidence: String,
    pub subjective_time: SubjectiveTimeSchema,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExecutionPolicySchema {
    pub requested_mode: String,
    pub replay_strictness: String,
    pub max_tokens: String,
    pub max_duration_ms: String,
    pub max_branches: String,
    pub max_tool_calls: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExecutionRealizationSchema {
    pub branch_count: String,
    pub tool_calls: String,
    pub refinement_cycles: String,
    pub replay_variance: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CostVectorSchema {
    pub time_ms: String,
    pub tokens_in: String,
    pub tokens_out: String,
    pub usd: String,
    pub cognitive_units: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TemporalReferenceFramesSchema {
    pub internal_reasoning: Vec<String>,
    pub external_translation: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TemporalSchemaContract {
    pub schema_version: String,
    pub owned_runtime_surfaces: Vec<String>,
    pub primary_temporal_anchor: TemporalAnchorSchema,
    pub execution_policy: ExecutionPolicySchema,
    pub execution_realization: ExecutionRealizationSchema,
    pub cost_vector: CostVectorSchema,
    pub reference_frames: TemporalReferenceFramesSchema,
    pub proof_hook_command: String,
    pub proof_hook_output_path: String,
    pub execution_policy_trace_hooks: Vec<String>,
    pub scope_boundary: String,
}

impl TemporalSchemaContract {
    pub fn v01() -> Self {
        Self {
            schema_version: TEMPORAL_SCHEMA_V01.to_string(),
            owned_runtime_surfaces: vec![
                "adl::chronosense::TemporalSchemaContract".to_string(),
                "adl::chronosense::TemporalAnchorSchema".to_string(),
                "adl::chronosense::ExecutionPolicySchema".to_string(),
                "adl::chronosense::ExecutionRealizationSchema".to_string(),
                "adl::chronosense::CostVectorSchema".to_string(),
                "adl identity schema".to_string(),
            ],
            primary_temporal_anchor: TemporalAnchorSchema {
                observed_at_utc: "required RFC3339 UTC timestamp".to_string(),
                observed_at_local: "required RFC3339 local timestamp".to_string(),
                agent_age: "required lifetime-relative duration".to_string(),
                turn_index: "required narrative/event sequence index".to_string(),
                monotonic_order: "required strictly increasing order token".to_string(),
                prior_event_delta: "required elapsed duration since prior relevant event"
                    .to_string(),
                temporal_confidence: "required one of high|medium|low".to_string(),
                subjective_time: SubjectiveTimeSchema {
                    narrative_position:
                        "required logical position within the active reasoning frame"
                            .to_string(),
                    integration_window:
                        "required specious-present span or explicit bounded placeholder"
                            .to_string(),
                    temporal_gap: "required one of none|explicit_gap|unknown".to_string(),
                    experienced_duration:
                        "optional but recommended agent-relative duration estimate".to_string(),
                    temporal_density:
                        "optional but recommended low|medium|high density signal".to_string(),
                },
            },
            execution_policy: ExecutionPolicySchema {
                requested_mode: "required one of efficient|fast|deterministic|exploratory"
                    .to_string(),
                replay_strictness: "required one of strict|bounded|relaxed".to_string(),
                max_tokens: "optional integer cap".to_string(),
                max_duration_ms: "optional integer cap".to_string(),
                max_branches: "optional integer cap".to_string(),
                max_tool_calls: "optional integer cap".to_string(),
            },
            execution_realization: ExecutionRealizationSchema {
                branch_count: "optional realized branch count".to_string(),
                tool_calls: "optional realized tool-call count".to_string(),
                refinement_cycles: "optional realized refinement-cycle count".to_string(),
                replay_variance: "required one of strict|bounded|high when recorded".to_string(),
            },
            cost_vector: CostVectorSchema {
                time_ms: "optional realized runtime in milliseconds".to_string(),
                tokens_in: "optional input token count".to_string(),
                tokens_out: "optional output token count".to_string(),
                usd: "optional realized USD cost".to_string(),
                cognitive_units: "optional ADL-specific coarse cognitive-cost unit".to_string(),
            },
            reference_frames: TemporalReferenceFramesSchema {
                internal_reasoning: vec![
                    "UTC".to_string(),
                    "monotonic".to_string(),
                    "lifetime".to_string(),
                ],
                external_translation: vec![
                    "human_local".to_string(),
                    "organization_local".to_string(),
                ],
            },
            proof_hook_command:
                "adl identity schema --out .adl/state/temporal_schema_v01.json".to_string(),
            proof_hook_output_path: ".adl/state/temporal_schema_v01.json".to_string(),
            execution_policy_trace_hooks: vec![
                "run_state.v1.duration_ms".to_string(),
                "run_state.v1.scheduler_max_concurrency".to_string(),
                "run_summary.v1.policy".to_string(),
                "run_summary.v1.counts.provider_call_count".to_string(),
            ],
            scope_boundary:
                "schema contract only; continuity validation, retrieval semantics, commitments, causality, and cost interpretation remain downstream work"
                    .to_string(),
        }
    }
}
