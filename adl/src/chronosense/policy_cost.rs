use serde::{Deserialize, Serialize};

use super::{
    temporal_schema::{CostVectorSchema, ExecutionPolicySchema, ExecutionRealizationSchema},
    EXECUTION_POLICY_COST_MODEL_SCHEMA,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CostPolicyContract {
    pub requested_mode: String,
    pub max_usd_per_run: String,
    pub max_tokens: String,
    pub max_duration_ms: String,
    pub max_cognitive_units: String,
    pub max_branches: String,
    pub max_tool_calls: String,
    pub preferred_models: String,
    pub disallowed_models: String,
    pub allow_parallel: String,
    pub priority: String,
    pub replay_strictness: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CostAnchorContract {
    pub trace_event_id: String,
    pub run_id: String,
    pub agent_id: String,
    pub observed_at_utc: String,
    pub execution_policy: String,
    pub duration_ms: String,
    pub cost_vector: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CostReviewSurfaceContract {
    pub required_questions: Vec<String>,
    pub required_trace_hooks: Vec<String>,
    pub comparison_rule: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExecutionPolicyCostModelContract {
    pub schema_version: String,
    pub owned_runtime_surfaces: Vec<String>,
    pub execution_policy: ExecutionPolicySchema,
    pub execution_realization: ExecutionRealizationSchema,
    pub cost_vector: CostVectorSchema,
    pub cost_policy: CostPolicyContract,
    pub cost_anchor: CostAnchorContract,
    pub review_surface: CostReviewSurfaceContract,
    pub proof_fixture_hooks: Vec<String>,
    pub proof_hook_command: String,
    pub proof_hook_output_path: String,
    pub scope_boundary: String,
}

impl ExecutionPolicyCostModelContract {
    pub fn v1() -> Self {
        Self {
            schema_version: EXECUTION_POLICY_COST_MODEL_SCHEMA.to_string(),
            owned_runtime_surfaces: vec![
                "adl::chronosense::ExecutionPolicyCostModelContract".to_string(),
                "adl::chronosense::ExecutionPolicySchema".to_string(),
                "adl::chronosense::ExecutionRealizationSchema".to_string(),
                "adl::chronosense::CostVectorSchema".to_string(),
                "adl::chronosense::CostPolicyContract".to_string(),
                "adl::chronosense::CostAnchorContract".to_string(),
                "adl identity cost".to_string(),
            ],
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
                branch_count: "required realized branch count when branching is enabled"
                    .to_string(),
                tool_calls: "required realized tool-call count when tools are used".to_string(),
                refinement_cycles: "optional realized refinement-cycle count".to_string(),
                replay_variance: "required one of strict|bounded|high when recorded".to_string(),
            },
            cost_vector: CostVectorSchema {
                time_ms: "required realized runtime in milliseconds".to_string(),
                tokens_in: "optional input token count".to_string(),
                tokens_out: "optional output token count".to_string(),
                usd: "optional realized USD cost".to_string(),
                cognitive_units: "optional bounded cognitive-cost unit".to_string(),
            },
            cost_policy: CostPolicyContract {
                requested_mode: "required one of efficient|fast|deterministic|exploratory"
                    .to_string(),
                max_usd_per_run: "optional USD budget ceiling".to_string(),
                max_tokens: "optional token ceiling".to_string(),
                max_duration_ms: "optional runtime ceiling".to_string(),
                max_cognitive_units: "optional cognitive-cost ceiling".to_string(),
                max_branches: "optional branch ceiling".to_string(),
                max_tool_calls: "optional tool-call ceiling".to_string(),
                preferred_models: "optional ordered preferred-model list".to_string(),
                disallowed_models: "optional disallowed-model list".to_string(),
                allow_parallel: "required true|false policy flag".to_string(),
                priority: "required one of cost|latency|quality".to_string(),
                replay_strictness: "required one of strict|bounded|relaxed".to_string(),
            },
            cost_anchor: CostAnchorContract {
                trace_event_id: "required canonical trace event id".to_string(),
                run_id: "required canonical run id".to_string(),
                agent_id: "required canonical agent id".to_string(),
                observed_at_utc: "required RFC3339 UTC timestamp".to_string(),
                execution_policy: "required execution policy reference".to_string(),
                duration_ms: "required duration anchor for cost comparison".to_string(),
                cost_vector: "required realized cost vector reference".to_string(),
            },
            review_surface: CostReviewSurfaceContract {
                required_questions: vec![
                    "what did this run cost".to_string(),
                    "where was cost incurred".to_string(),
                    "why was this execution posture chosen".to_string(),
                ],
                required_trace_hooks: vec![
                    "run_state.v1.duration_ms".to_string(),
                    "run_state.v1.scheduler_max_concurrency".to_string(),
                    "run_summary.v1.policy".to_string(),
                    "run_summary.v1.counts.provider_call_count".to_string(),
                ],
                comparison_rule:
                    "reviewers must be able to compare requested execution posture against realized cost and execution behavior"
                        .to_string(),
            },
            proof_fixture_hooks: vec![
                "adl::chronosense::ExecutionPolicyCostModelContract::v1".to_string(),
                "adl identity cost --out .adl/state/execution_policy_cost_model_v1.json"
                    .to_string(),
            ],
            proof_hook_command:
                "adl identity cost --out .adl/state/execution_policy_cost_model_v1.json"
                    .to_string(),
            proof_hook_output_path: ".adl/state/execution_policy_cost_model_v1.json".to_string(),
            scope_boundary:
                "execution policy and cost reviewability only; enterprise pricing, instinct policy, and broader economics strategy remain downstream work"
                    .to_string(),
        }
    }
}
