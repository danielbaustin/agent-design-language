use crate::chronosense::{CommitmentDeadlineContract, COMMITMENT_DEADLINE_SCHEMA};
use crate::provider::provider_profile_names;
use anyhow::{anyhow, Result};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

pub const SCHEDULER_ECONOMICS_INPUT_SCHEMA_V1: &str = "adl.scheduler.economics_input.v1";
pub const SCHEDULER_ECONOMICS_INPUT_BUNDLE_SCHEMA_V1: &str =
    "adl.scheduler.economics_input_bundle.v1";
pub const SCHEDULER_ECONOMICS_INPUT_BUNDLE_WITH_PROVIDER_ROUTE_SCHEMA_V1: &str =
    "adl.scheduler.economics_input_bundle.provider_route.v1";
pub const SCHEDULER_MODEL_SUITABILITY_INPUT_BUNDLE_SCHEMA_V1: &str =
    "adl.scheduler.economics_input_bundle.model_suitability.v1";
pub const SCHEDULER_CHEAPEST_VALIDATED_OUTCOME_INPUT_BUNDLE_SCHEMA_V1: &str =
    "adl.scheduler.economics_input_bundle.cheapest_validated_outcome.v1";
pub const SCHEDULER_PROVIDER_CHEAPEST_VALIDATED_OUTCOME_INPUT_BUNDLE_SCHEMA_V1: &str =
    "adl.scheduler.economics_input_bundle.provider_cheapest_validated_outcome.v1";
pub const COGNITIVE_SCHEDULER_DECISION_SCHEMA_V1: &str = "adl.scheduler.decision.v1";
pub const COGNITIVE_SCHEDULER_DECISION_WITH_PROVIDER_ROUTE_SCHEMA_V1: &str =
    "adl.scheduler.decision.provider_route.v1";
pub const COGNITIVE_SCHEDULER_DECISION_MODEL_SUITABILITY_SCHEMA_V1: &str =
    "adl.scheduler.decision.model_suitability.v1";
pub const COGNITIVE_SCHEDULER_DECISION_CHEAPEST_VALIDATED_OUTCOME_SCHEMA_V1: &str =
    "adl.scheduler.decision.cheapest_validated_outcome.v1";
pub const COGNITIVE_SCHEDULER_DECISION_PROVIDER_CHEAPEST_VALIDATED_OUTCOME_SCHEMA_V1: &str =
    "adl.scheduler.decision.provider_cheapest_validated_outcome.v1";
pub const COGNITIVE_SCHEDULER_PLAN_SCHEMA_V1: &str = "adl.scheduler.plan.v1";
pub const CHRONOSENSE_SCHEDULER_CONTEXT_SCHEMA_V1: &str = "adl.scheduler.chronosense_context.v1";
pub const ROLE_PROVIDER_SELECTION_CONTEXT_SCHEMA_V1: &str =
    "adl.scheduler.role_provider_selection_context.v1";
pub const MODEL_SUITABILITY_SELECTION_CONTEXT_SCHEMA_V1: &str =
    "adl.scheduler.model_suitability_selection_context.v1";
pub const CHEAPEST_VALIDATED_OUTCOME_POLICY_SCHEMA_V1: &str =
    "adl.scheduler.cheapest_validated_outcome_policy.v1";

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SchedulerTaskTypeV1 {
    IssueCard,
    Planning,
    Documentation,
    Review,
    TestGeneration,
    Implementation,
    Refactor,
    SecurityReview,
    ReleaseGate,
    Architecture,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SchedulerRiskLevelV1 {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SchedulerUrgencyV1 {
    Low,
    Normal,
    High,
    Immediate,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SchedulerEffortV1 {
    Small,
    Medium,
    Large,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SchedulerCostLevelV1 {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SchedulerExpectedValueV1 {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SchedulerParallelismPotentialV1 {
    Blocked,
    Serial,
    Parallelizable,
    HighlyParallelizable,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SchedulerDependencyPostureV1 {
    Clear,
    Partial,
    Blocked,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SchedulerPressureLevelV1 {
    Low,
    Medium,
    High,
    Constrained,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SchedulerConfidenceV1 {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ChronosenseCommitmentStatusV1 {
    Proposed,
    Accepted,
    Active,
    Fulfilled,
    Deferred,
    Canceled,
    Expired,
    Missed,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ChronosenseDeadlineFrameV1 {
    WallClock,
    EventCount,
    ReviewGate,
    ContinuityRelative,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ChronosenseDeadlinePostureV1 {
    None,
    Future,
    Approaching,
    Due,
    Missed,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum RoleProviderProfileV1 {
    ConductorProvider,
    ArchitectProvider,
    ImplementerProvider,
    ReviewerProvider,
    TesterProvider,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ModelSuitabilityRoleV1 {
    Watcher,
    CardValidator,
    Reviewer,
    Planner,
    CloseoutChecker,
    Worker,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ModelSuitabilityClassificationV1 {
    UsefulWithLimits,
    SupportedWithLimits,
    CandidateOnly,
    RuntimeUnsuitableForThisPanel,
    HistoricalOnly,
    Blocked,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ModelSuitabilityTraceDispositionV1 {
    Selected,
    Rejected,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CognitiveSchedulerLaneV1 {
    Local,
    CheapRemote,
    Premium,
    Governor,
    Delayed,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SchedulerAlternativeDispositionV1 {
    Rejected,
    Fallback,
    Equivalent,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SchedulerDependencyRefV1 {
    pub task_id: String,
    pub status: SchedulerDependencyPostureV1,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ChronosenseCommitmentSchedulingSignalV1 {
    pub task_id: String,
    pub commitment_id: String,
    pub status: ChronosenseCommitmentStatusV1,
    pub deadline_posture: ChronosenseDeadlinePostureV1,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deadline_frame: Option<ChronosenseDeadlineFrameV1>,
    pub temporal_urgency: SchedulerUrgencyV1,
    #[serde(default)]
    pub fulfillment_ready: bool,
    #[serde(default)]
    pub review_required: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ChronosenseSchedulerContextV1 {
    pub schema_version: String,
    pub contract_schema_version: String,
    pub generated_from: String,
    pub signals: Vec<ChronosenseCommitmentSchedulingSignalV1>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ProviderRouteV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider_profile_ref: Option<String>,
    pub provider_spec_kind: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider_family: Option<String>,
    pub model_ref: String,
    pub model_identity: String,
    pub runtime_surface: String,
    pub provider_selection_reason: String,
    pub route_resolution_trace: Vec<String>,
    pub output_contract_ref: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct RoleProviderCandidateRouteV1 {
    pub route: ProviderRouteV1,
    pub eligible: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ineligibility_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct RoleProviderProfilePolicyV1 {
    pub role_profile: RoleProviderProfileV1,
    pub advisory_authority_limit: String,
    pub required_capabilities: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub forbidden_capabilities: Vec<String>,
    pub candidate_routes: Vec<RoleProviderCandidateRouteV1>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct RoleProviderTaskAssignmentV1 {
    pub task_id: String,
    pub role_profile: RoleProviderProfileV1,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct RoleProviderSelectionContextV1 {
    pub schema_version: String,
    pub generated_from: String,
    pub policies: Vec<RoleProviderProfilePolicyV1>,
    pub assignments: Vec<RoleProviderTaskAssignmentV1>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ModelSuitabilityCandidateV1 {
    pub candidate_id: String,
    pub provider_profile_ref: String,
    pub provider_family: String,
    pub model_ref: String,
    pub runtime_surface: String,
    pub classification: ModelSuitabilityClassificationV1,
    pub selection_priority: u32,
    pub roles: Vec<ModelSuitabilityRoleV1>,
    pub source_ref: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evidence_digest: Option<String>,
    #[serde(default)]
    pub advisory_authority_only: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ModelSuitabilityTaskRequirementV1 {
    pub task_id: String,
    pub role: ModelSuitabilityRoleV1,
    pub minimum_classification: ModelSuitabilityClassificationV1,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub allowed_provider_profile_refs: Vec<String>,
    pub claim_boundary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ModelSuitabilitySelectionContextV1 {
    pub schema_version: String,
    pub generated_from: String,
    pub evidence_refs: Vec<String>,
    pub candidates: Vec<ModelSuitabilityCandidateV1>,
    pub task_requirements: Vec<ModelSuitabilityTaskRequirementV1>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct CheapestValidatedCandidateEvidenceV1 {
    pub candidate_id: String,
    pub candidate_source_ref: String,
    pub outcome_cost_tier: SchedulerCostLevelV1,
    pub validation_ref: String,
    #[serde(default)]
    pub validated_outcome: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct CheapestValidatedOutcomeTaskPolicyV1 {
    pub task_id: String,
    pub max_outcome_cost_tier: SchedulerCostLevelV1,
    #[serde(default)]
    pub allow_unvalidated_fallback: bool,
    pub claim_boundary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct CheapestValidatedOutcomePolicyV1 {
    pub schema_version: String,
    pub generated_from: String,
    pub evidence_refs: Vec<String>,
    pub candidate_evidence: Vec<CheapestValidatedCandidateEvidenceV1>,
    pub task_policies: Vec<CheapestValidatedOutcomeTaskPolicyV1>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SchedulerEconomicsInputV1 {
    pub schema_version: String,
    pub task_id: String,
    pub task_type: SchedulerTaskTypeV1,
    pub estimated_effort: SchedulerEffortV1,
    pub estimated_validation_cost: SchedulerCostLevelV1,
    pub estimated_coordination_cost: SchedulerCostLevelV1,
    pub risk_level: SchedulerRiskLevelV1,
    pub expected_value: SchedulerExpectedValueV1,
    pub urgency: SchedulerUrgencyV1,
    pub dependency_posture: SchedulerDependencyPostureV1,
    pub parallelism_potential: SchedulerParallelismPotentialV1,
    pub premium_capacity_pressure: SchedulerPressureLevelV1,
    pub governor_attention_pressure: SchedulerPressureLevelV1,
    pub confidence: SchedulerConfidenceV1,
    #[serde(default)]
    pub human_required: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dependencies: Vec<SchedulerDependencyRefV1>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub required_capabilities: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manual_override: Option<String>,
    pub claim_boundary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SchedulerEconomicsInputBundleV1 {
    pub schema_version: String,
    pub source_doc_ref: String,
    pub included_concepts: Vec<String>,
    pub deferred_concepts: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chronosense_context: Option<ChronosenseSchedulerContextV1>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role_provider_context: Option<RoleProviderSelectionContextV1>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model_suitability_context: Option<ModelSuitabilitySelectionContextV1>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cheapest_validated_outcome_policy: Option<CheapestValidatedOutcomePolicyV1>,
    pub inputs: Vec<SchedulerEconomicsInputV1>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SchedulerEconomicsSummaryV1 {
    pub task_id: String,
    pub blocked: bool,
    pub lifecycle_cost_score: u32,
    pub value_score: u32,
    pub attention_pressure_score: u32,
    pub parallelism_score: u32,
    pub dependency_posture_score: u32,
    pub confidence_score: u32,
    pub deterministic_rank_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SchedulerAlternativeV1 {
    pub lane: CognitiveSchedulerLaneV1,
    pub disposition: SchedulerAlternativeDispositionV1,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SchedulerScoreBreakdownV1 {
    pub lifecycle_cost_score: u32,
    pub value_score: u32,
    pub attention_pressure_score: u32,
    pub parallelism_score: u32,
    pub dependency_posture_score: u32,
    pub confidence_score: u32,
    pub validation_cost: SchedulerCostLevelV1,
    pub coordination_cost: SchedulerCostLevelV1,
    pub risk: SchedulerRiskLevelV1,
    pub urgency: SchedulerUrgencyV1,
    pub expected_value: SchedulerExpectedValueV1,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SchedulerManualOverrideV1 {
    pub present: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ModelSuitabilitySelectionTraceV1 {
    pub candidate_id: String,
    pub provider_profile_ref: String,
    pub model_ref: String,
    pub disposition: ModelSuitabilityTraceDispositionV1,
    pub reason: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outcome_cost_tier: Option<SchedulerCostLevelV1>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub validation_ref: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ModelSuitabilitySelectionV1 {
    pub role: ModelSuitabilityRoleV1,
    pub selected_candidate_id: String,
    pub provider_profile_ref: String,
    pub provider_family: String,
    pub model_ref: String,
    pub runtime_surface: String,
    pub classification: ModelSuitabilityClassificationV1,
    pub source_ref: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evidence_digest: Option<String>,
    pub advisory_authority_only: bool,
    pub claim_boundary: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outcome_cost_tier: Option<SchedulerCostLevelV1>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub validation_ref: Option<String>,
    #[serde(default)]
    pub cheapest_validated_outcome: bool,
    pub selection_trace: Vec<ModelSuitabilitySelectionTraceV1>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct CognitiveSchedulerDecisionV1 {
    pub schema_version: String,
    pub task_id: String,
    pub selected_lane: CognitiveSchedulerLaneV1,
    pub alternatives_considered: Vec<SchedulerAlternativeV1>,
    pub reason: String,
    pub score_breakdown: SchedulerScoreBreakdownV1,
    pub dependency_status: SchedulerDependencyPostureV1,
    pub manual_override: SchedulerManualOverrideV1,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider_route: Option<ProviderRouteV1>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model_suitability_selection: Option<ModelSuitabilitySelectionV1>,
    pub confidence: SchedulerConfidenceV1,
    pub scheduling_rank_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct CognitiveSchedulerPlanV1 {
    pub schema_version: String,
    pub source_schema_version: String,
    pub decisions: Vec<CognitiveSchedulerDecisionV1>,
    pub recommended_order: Vec<String>,
}

pub fn parse_economics_input_json(input: &str) -> Result<SchedulerEconomicsInputV1> {
    let parsed: SchedulerEconomicsInputV1 = serde_json::from_str(input)?;
    validate_economics_input(&parsed)?;
    Ok(parsed)
}

pub fn parse_economics_input_yaml(input: &str) -> Result<SchedulerEconomicsInputV1> {
    let parsed: SchedulerEconomicsInputV1 = serde_yaml::from_str(input)?;
    validate_economics_input(&parsed)?;
    Ok(parsed)
}

pub fn parse_economics_bundle_json(input: &str) -> Result<SchedulerEconomicsInputBundleV1> {
    let parsed: SchedulerEconomicsInputBundleV1 = serde_json::from_str(input)?;
    validate_economics_bundle(&parsed)?;
    Ok(parsed)
}

pub fn validate_economics_bundle(bundle: &SchedulerEconomicsInputBundleV1) -> Result<()> {
    match (
        bundle.schema_version.as_str(),
        bundle.role_provider_context.as_ref(),
        bundle.model_suitability_context.as_ref(),
        bundle.cheapest_validated_outcome_policy.as_ref(),
    ) {
        (SCHEDULER_ECONOMICS_INPUT_BUNDLE_SCHEMA_V1, None, None, None) => {}
        (SCHEDULER_ECONOMICS_INPUT_BUNDLE_SCHEMA_V1, Some(_), _, _) => {
            return Err(anyhow!(
                "scheduler economics bundle with role_provider_context must use schema {SCHEDULER_ECONOMICS_INPUT_BUNDLE_WITH_PROVIDER_ROUTE_SCHEMA_V1}"
            ));
        }
        (SCHEDULER_ECONOMICS_INPUT_BUNDLE_SCHEMA_V1, _, Some(_), _) => {
            return Err(anyhow!(
                "model_suitability_context requires scheduler bundle schema {SCHEDULER_MODEL_SUITABILITY_INPUT_BUNDLE_SCHEMA_V1}"
            ));
        }
        (SCHEDULER_ECONOMICS_INPUT_BUNDLE_SCHEMA_V1, _, _, Some(_)) => {
            return Err(anyhow!(
                "cheapest_validated_outcome_policy requires scheduler bundle schema {SCHEDULER_CHEAPEST_VALIDATED_OUTCOME_INPUT_BUNDLE_SCHEMA_V1}"
            ));
        }
        (SCHEDULER_ECONOMICS_INPUT_BUNDLE_WITH_PROVIDER_ROUTE_SCHEMA_V1, Some(_), None, None) => {}
        (SCHEDULER_ECONOMICS_INPUT_BUNDLE_WITH_PROVIDER_ROUTE_SCHEMA_V1, None, _, _) => {
            return Err(anyhow!(
                "scheduler bundle schema {SCHEDULER_ECONOMICS_INPUT_BUNDLE_WITH_PROVIDER_ROUTE_SCHEMA_V1} requires role_provider_context"
            ));
        }
        (SCHEDULER_ECONOMICS_INPUT_BUNDLE_WITH_PROVIDER_ROUTE_SCHEMA_V1, _, Some(_), _) => {
            return Err(anyhow!(
                "scheduler bundle schema {SCHEDULER_ECONOMICS_INPUT_BUNDLE_WITH_PROVIDER_ROUTE_SCHEMA_V1} cannot include model_suitability_context without a combined schema"
            ));
        }
        (SCHEDULER_ECONOMICS_INPUT_BUNDLE_WITH_PROVIDER_ROUTE_SCHEMA_V1, _, _, Some(_)) => {
            return Err(anyhow!(
                "scheduler bundle schema {SCHEDULER_ECONOMICS_INPUT_BUNDLE_WITH_PROVIDER_ROUTE_SCHEMA_V1} cannot include cheapest_validated_outcome_policy without a combined schema"
            ));
        }
        (SCHEDULER_MODEL_SUITABILITY_INPUT_BUNDLE_SCHEMA_V1, None, Some(_), None) => {}
        (SCHEDULER_MODEL_SUITABILITY_INPUT_BUNDLE_SCHEMA_V1, Some(_), _, _) => {
            return Err(anyhow!(
                "scheduler bundle schema {SCHEDULER_MODEL_SUITABILITY_INPUT_BUNDLE_SCHEMA_V1} cannot include role_provider_context without a combined schema"
            ));
        }
        (SCHEDULER_MODEL_SUITABILITY_INPUT_BUNDLE_SCHEMA_V1, _, None, _) => {
            return Err(anyhow!(
                "scheduler bundle schema {SCHEDULER_MODEL_SUITABILITY_INPUT_BUNDLE_SCHEMA_V1} requires model_suitability_context"
            ));
        }
        (SCHEDULER_MODEL_SUITABILITY_INPUT_BUNDLE_SCHEMA_V1, _, _, Some(_)) => {
            return Err(anyhow!(
                "cheapest_validated_outcome_policy requires scheduler bundle schema {SCHEDULER_CHEAPEST_VALIDATED_OUTCOME_INPUT_BUNDLE_SCHEMA_V1}"
            ));
        }
        (SCHEDULER_CHEAPEST_VALIDATED_OUTCOME_INPUT_BUNDLE_SCHEMA_V1, None, Some(_), Some(_)) => {}
        (SCHEDULER_CHEAPEST_VALIDATED_OUTCOME_INPUT_BUNDLE_SCHEMA_V1, Some(_), _, _) => {
            return Err(anyhow!(
                "scheduler bundle schema {SCHEDULER_CHEAPEST_VALIDATED_OUTCOME_INPUT_BUNDLE_SCHEMA_V1} cannot include role_provider_context"
            ));
        }
        (SCHEDULER_CHEAPEST_VALIDATED_OUTCOME_INPUT_BUNDLE_SCHEMA_V1, _, None, _) => {
            return Err(anyhow!(
                "scheduler bundle schema {SCHEDULER_CHEAPEST_VALIDATED_OUTCOME_INPUT_BUNDLE_SCHEMA_V1} requires model_suitability_context"
            ));
        }
        (SCHEDULER_CHEAPEST_VALIDATED_OUTCOME_INPUT_BUNDLE_SCHEMA_V1, _, _, None) => {
            return Err(anyhow!(
                "scheduler bundle schema {SCHEDULER_CHEAPEST_VALIDATED_OUTCOME_INPUT_BUNDLE_SCHEMA_V1} requires cheapest_validated_outcome_policy"
            ));
        }
        (
            SCHEDULER_PROVIDER_CHEAPEST_VALIDATED_OUTCOME_INPUT_BUNDLE_SCHEMA_V1,
            Some(_),
            Some(_),
            Some(_),
        ) => {}
        (SCHEDULER_PROVIDER_CHEAPEST_VALIDATED_OUTCOME_INPUT_BUNDLE_SCHEMA_V1, None, _, _) => {
            return Err(anyhow!(
                "scheduler bundle schema {SCHEDULER_PROVIDER_CHEAPEST_VALIDATED_OUTCOME_INPUT_BUNDLE_SCHEMA_V1} requires role_provider_context"
            ));
        }
        (SCHEDULER_PROVIDER_CHEAPEST_VALIDATED_OUTCOME_INPUT_BUNDLE_SCHEMA_V1, _, None, _) => {
            return Err(anyhow!(
                "scheduler bundle schema {SCHEDULER_PROVIDER_CHEAPEST_VALIDATED_OUTCOME_INPUT_BUNDLE_SCHEMA_V1} requires model_suitability_context"
            ));
        }
        (SCHEDULER_PROVIDER_CHEAPEST_VALIDATED_OUTCOME_INPUT_BUNDLE_SCHEMA_V1, _, _, None) => {
            return Err(anyhow!(
                "scheduler bundle schema {SCHEDULER_PROVIDER_CHEAPEST_VALIDATED_OUTCOME_INPUT_BUNDLE_SCHEMA_V1} requires cheapest_validated_outcome_policy"
            ));
        }
        (other, _, _, _) => {
            return Err(anyhow!(
                "unsupported scheduler economics bundle schema: {other}"
            ));
        }
    }
    if bundle.source_doc_ref.trim().is_empty() {
        return Err(anyhow!(
            "scheduler economics bundle source_doc_ref is required"
        ));
    }
    if bundle.inputs.is_empty() {
        return Err(anyhow!(
            "scheduler economics bundle must include at least one input"
        ));
    }
    if bundle.included_concepts.is_empty() {
        return Err(anyhow!(
            "scheduler economics bundle must record included v1 concepts"
        ));
    }
    if bundle.deferred_concepts.is_empty() {
        return Err(anyhow!(
            "scheduler economics bundle must record deferred economics concepts"
        ));
    }
    for input in &bundle.inputs {
        validate_economics_input(input)?;
    }
    if let Some(context) = &bundle.chronosense_context {
        validate_chronosense_scheduler_context(context, &bundle.inputs)?;
    }
    if let Some(context) = &bundle.role_provider_context {
        validate_role_provider_selection_context(context, &bundle.inputs)?;
    }
    if let Some(context) = &bundle.model_suitability_context {
        validate_model_suitability_context(context, &bundle.inputs)?;
    }
    if let Some(policy) = &bundle.cheapest_validated_outcome_policy {
        let context = bundle.model_suitability_context.as_ref().ok_or_else(|| {
            anyhow!("cheapest validated outcome policy requires model suitability context")
        })?;
        validate_cheapest_validated_outcome_policy(policy, context, &bundle.inputs)?;
    }
    Ok(())
}

pub fn validate_chronosense_scheduler_context(
    context: &ChronosenseSchedulerContextV1,
    inputs: &[SchedulerEconomicsInputV1],
) -> Result<()> {
    if context.schema_version != CHRONOSENSE_SCHEDULER_CONTEXT_SCHEMA_V1 {
        return Err(anyhow!(
            "unsupported chronosense scheduler context schema: {}",
            context.schema_version
        ));
    }
    if context.contract_schema_version != COMMITMENT_DEADLINE_SCHEMA {
        return Err(anyhow!(
            "chronosense scheduler context must reference commitment contract {COMMITMENT_DEADLINE_SCHEMA}"
        ));
    }
    if context.generated_from.trim().is_empty() {
        return Err(anyhow!(
            "chronosense scheduler context generated_from is required"
        ));
    }
    let contract = CommitmentDeadlineContract::v1();
    for required_surface in [
        "open commitments",
        "approaching deadlines",
        "missed commitments in interval",
    ] {
        if !contract
            .missed_commitment_detection
            .retrieval_surfaces
            .iter()
            .any(|surface| surface == required_surface)
        {
            return Err(anyhow!(
                "commitment deadline contract missing scheduler retrieval surface {required_surface}"
            ));
        }
    }
    for signal in &context.signals {
        if signal.task_id.trim().is_empty() {
            return Err(anyhow!("chronosense signal task_id is required"));
        }
        if signal.commitment_id.trim().is_empty() {
            return Err(anyhow!("chronosense signal commitment_id is required"));
        }
        if signal.deadline_posture != ChronosenseDeadlinePostureV1::None
            && signal.deadline_frame.is_none()
        {
            return Err(anyhow!(
                "chronosense signal {} deadline_frame is required when deadline_posture is not none",
                signal.task_id
            ));
        }
        if !inputs.iter().any(|input| input.task_id == signal.task_id) {
            return Err(anyhow!(
                "chronosense signal {} does not match a scheduler input task_id",
                signal.task_id
            ));
        }
    }
    Ok(())
}

pub fn validate_role_provider_selection_context(
    context: &RoleProviderSelectionContextV1,
    inputs: &[SchedulerEconomicsInputV1],
) -> Result<()> {
    if context.schema_version != ROLE_PROVIDER_SELECTION_CONTEXT_SCHEMA_V1 {
        return Err(anyhow!(
            "unsupported role provider selection context schema: {}",
            context.schema_version
        ));
    }
    if context.generated_from.trim().is_empty() {
        return Err(anyhow!(
            "role provider selection context generated_from is required"
        ));
    }
    if context.policies.is_empty() {
        return Err(anyhow!(
            "role provider selection context must include at least one policy"
        ));
    }
    if context.assignments.is_empty() {
        return Err(anyhow!(
            "role provider selection context must include at least one assignment"
        ));
    }

    let known_profiles = provider_profile_names()
        .into_iter()
        .collect::<BTreeSet<_>>();
    let input_ids = inputs
        .iter()
        .map(|input| input.task_id.as_str())
        .collect::<BTreeSet<_>>();
    let mut policy_roles = BTreeSet::new();
    for policy in &context.policies {
        if !policy_roles.insert(policy.role_profile.clone()) {
            return Err(anyhow!(
                "duplicate role provider policy for {:?}",
                policy.role_profile
            ));
        }
        if policy.advisory_authority_limit.trim().is_empty() {
            return Err(anyhow!(
                "role provider policy {:?} advisory_authority_limit is required",
                policy.role_profile
            ));
        }
        if policy.required_capabilities.is_empty() {
            return Err(anyhow!(
                "role provider policy {:?} must name required capabilities",
                policy.role_profile
            ));
        }
        if policy.candidate_routes.is_empty() {
            return Err(anyhow!(
                "role provider policy {:?} must include candidate routes",
                policy.role_profile
            ));
        }
        let mut has_eligible_route = false;
        for candidate in &policy.candidate_routes {
            validate_provider_route(&candidate.route, &known_profiles)?;
            match (candidate.eligible, candidate.ineligibility_reason.as_ref()) {
                (true, Some(reason)) if !reason.trim().is_empty() => {
                    return Err(anyhow!(
                        "eligible role provider candidate {:?} must not include ineligibility_reason",
                        policy.role_profile
                    ));
                }
                (false, None) => {
                    return Err(anyhow!(
                        "ineligible role provider candidate {:?} must include ineligibility_reason",
                        policy.role_profile
                    ));
                }
                (false, Some(reason)) if reason.trim().is_empty() => {
                    return Err(anyhow!(
                        "ineligible role provider candidate {:?} ineligibility_reason is empty",
                        policy.role_profile
                    ));
                }
                _ => {}
            }
            if candidate.eligible {
                has_eligible_route = true;
            }
        }
        if !has_eligible_route {
            return Err(anyhow!(
                "role provider policy {:?} has no eligible candidate route",
                policy.role_profile
            ));
        }
    }

    let mut assigned_task_ids = BTreeSet::new();
    for assignment in &context.assignments {
        if assignment.task_id.trim().is_empty() {
            return Err(anyhow!("role provider assignment task_id is required"));
        }
        if !assigned_task_ids.insert(assignment.task_id.clone()) {
            return Err(anyhow!(
                "duplicate role provider assignment for task {}",
                assignment.task_id
            ));
        }
        if !input_ids.contains(assignment.task_id.as_str()) {
            return Err(anyhow!(
                "role provider assignment {} does not match a scheduler input task_id",
                assignment.task_id
            ));
        }
        if !policy_roles.contains(&assignment.role_profile) {
            return Err(anyhow!(
                "role provider assignment {} references role {:?} without a policy",
                assignment.task_id,
                assignment.role_profile
            ));
        }
    }

    Ok(())
}

pub fn validate_model_suitability_context(
    context: &ModelSuitabilitySelectionContextV1,
    inputs: &[SchedulerEconomicsInputV1],
) -> Result<()> {
    if context.schema_version != MODEL_SUITABILITY_SELECTION_CONTEXT_SCHEMA_V1 {
        return Err(anyhow!(
            "unsupported model suitability context schema: {}",
            context.schema_version
        ));
    }
    if context.generated_from.trim().is_empty() {
        return Err(anyhow!(
            "model suitability context generated_from is required"
        ));
    }
    if context.evidence_refs.is_empty() {
        return Err(anyhow!(
            "model suitability context must retain at least one evidence_ref"
        ));
    }
    if context.candidates.is_empty() {
        return Err(anyhow!(
            "model suitability context must include at least one candidate"
        ));
    }
    if context.task_requirements.is_empty() {
        return Err(anyhow!(
            "model suitability context must include at least one task requirement"
        ));
    }

    let evidence_refs = context
        .evidence_refs
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    let mut seen_candidates = BTreeSet::new();
    let mut seen_provider_models = BTreeSet::new();
    for candidate in &context.candidates {
        if candidate.candidate_id.trim().is_empty()
            || candidate.provider_profile_ref.trim().is_empty()
            || candidate.provider_family.trim().is_empty()
            || candidate.model_ref.trim().is_empty()
            || candidate.runtime_surface.trim().is_empty()
            || candidate.source_ref.trim().is_empty()
        {
            return Err(anyhow!(
                "model suitability candidate fields are required for {}",
                candidate.candidate_id
            ));
        }
        if !seen_candidates.insert(candidate.candidate_id.clone()) {
            return Err(anyhow!(
                "duplicate model suitability candidate {}",
                candidate.candidate_id
            ));
        }
        if !seen_provider_models.insert((
            candidate.provider_profile_ref.clone(),
            candidate.model_ref.clone(),
        )) {
            return Err(anyhow!(
                "duplicate model suitability provider/model pair {} {}",
                candidate.provider_profile_ref,
                candidate.model_ref
            ));
        }
        if !evidence_refs.contains(&candidate.source_ref) {
            return Err(anyhow!(
                "model suitability candidate {} source_ref is not retained in evidence_refs",
                candidate.candidate_id
            ));
        }
        if candidate.roles.is_empty() {
            return Err(anyhow!(
                "model suitability candidate {} must include at least one role",
                candidate.candidate_id
            ));
        }
        if candidate.selection_priority == 0 {
            return Err(anyhow!(
                "model suitability candidate {} selection_priority must be greater than zero",
                candidate.candidate_id
            ));
        }
        if !candidate.advisory_authority_only {
            return Err(anyhow!(
                "model suitability candidate {} must be advisory_authority_only",
                candidate.candidate_id
            ));
        }
    }

    let input_ids = inputs
        .iter()
        .map(|input| input.task_id.as_str())
        .collect::<BTreeSet<_>>();
    let mut seen_requirements = BTreeSet::new();
    for requirement in &context.task_requirements {
        if requirement.task_id.trim().is_empty() {
            return Err(anyhow!("model suitability task_id is required"));
        }
        if !seen_requirements.insert(requirement.task_id.clone()) {
            return Err(anyhow!(
                "duplicate model suitability task requirement for {}",
                requirement.task_id
            ));
        }
        if !input_ids.contains(requirement.task_id.as_str()) {
            return Err(anyhow!(
                "model suitability task {} does not match a scheduler input task_id",
                requirement.task_id
            ));
        }
        if !valid_model_suitability_claim_boundary(&requirement.claim_boundary) {
            return Err(anyhow!(
                "model suitability task {} claim_boundary must be one of the approved bounded non-authority values",
                requirement.task_id
            ));
        }
        for allowed in &requirement.allowed_provider_profile_refs {
            if !context
                .candidates
                .iter()
                .any(|candidate| candidate.provider_profile_ref == *allowed)
            {
                return Err(anyhow!(
                    "model suitability task {} allows unknown provider_profile_ref {}",
                    requirement.task_id,
                    allowed
                ));
            }
        }
        select_model_suitability_candidate(context, requirement, None).map_err(|err| {
            anyhow!(
                "model suitability task {} has no eligible candidate: {err}",
                requirement.task_id
            )
        })?;
    }

    Ok(())
}

pub fn validate_cheapest_validated_outcome_policy(
    policy: &CheapestValidatedOutcomePolicyV1,
    context: &ModelSuitabilitySelectionContextV1,
    inputs: &[SchedulerEconomicsInputV1],
) -> Result<()> {
    if policy.schema_version != CHEAPEST_VALIDATED_OUTCOME_POLICY_SCHEMA_V1 {
        return Err(anyhow!(
            "unsupported cheapest validated outcome policy schema: {}",
            policy.schema_version
        ));
    }
    if policy.generated_from.trim().is_empty() {
        return Err(anyhow!(
            "cheapest validated outcome policy generated_from is required"
        ));
    }
    if policy.evidence_refs.is_empty() {
        return Err(anyhow!(
            "cheapest validated outcome policy must retain at least one evidence_ref"
        ));
    }
    if policy.candidate_evidence.is_empty() {
        return Err(anyhow!(
            "cheapest validated outcome policy must include candidate evidence"
        ));
    }
    if policy.task_policies.is_empty() {
        return Err(anyhow!(
            "cheapest validated outcome policy must include task policies"
        ));
    }

    let retained_evidence_refs = policy
        .evidence_refs
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    let candidate_ids = context
        .candidates
        .iter()
        .map(|candidate| candidate.candidate_id.as_str())
        .collect::<BTreeSet<_>>();
    let mut seen_candidate_evidence = BTreeSet::new();
    for evidence in &policy.candidate_evidence {
        if evidence.candidate_id.trim().is_empty() {
            return Err(anyhow!(
                "cheapest validated outcome candidate_id is required"
            ));
        }
        if evidence.candidate_source_ref.trim().is_empty() {
            return Err(anyhow!(
                "cheapest validated outcome candidate {} candidate_source_ref is required",
                evidence.candidate_id
            ));
        }
        if !seen_candidate_evidence.insert(evidence.candidate_id.clone()) {
            return Err(anyhow!(
                "duplicate cheapest validated outcome candidate evidence for {}",
                evidence.candidate_id
            ));
        }
        let candidate = context
            .candidates
            .iter()
            .find(|candidate| candidate.candidate_id == evidence.candidate_id)
            .ok_or_else(|| {
                anyhow!(
                    "cheapest validated outcome candidate {} is not in model suitability candidates",
                    evidence.candidate_id
                )
            })?;
        if !candidate_ids.contains(evidence.candidate_id.as_str()) {
            return Err(anyhow!(
                "cheapest validated outcome candidate {} is not in model suitability candidates",
                evidence.candidate_id
            ));
        }
        if evidence.candidate_source_ref != candidate.source_ref {
            return Err(anyhow!(
                "cheapest validated outcome candidate {} candidate_source_ref does not match model suitability source_ref",
                evidence.candidate_id
            ));
        }
        if !retained_evidence_refs.contains(&evidence.candidate_source_ref) {
            return Err(anyhow!(
                "cheapest validated outcome candidate {} candidate_source_ref is not retained in evidence_refs",
                evidence.candidate_id
            ));
        }
        if evidence.validation_ref.trim().is_empty() {
            return Err(anyhow!(
                "cheapest validated outcome candidate {} validation_ref is required",
                evidence.candidate_id
            ));
        }
        if !retained_evidence_refs.contains(&evidence.validation_ref) {
            return Err(anyhow!(
                "cheapest validated outcome candidate {} validation_ref is not retained in evidence_refs",
                evidence.candidate_id
            ));
        }
        if !evidence.validated_outcome {
            return Err(anyhow!(
                "cheapest validated outcome candidate {} must have validated_outcome=true",
                evidence.candidate_id
            ));
        }
    }

    let input_ids = inputs
        .iter()
        .map(|input| input.task_id.as_str())
        .collect::<BTreeSet<_>>();
    let mut seen_task_policies = BTreeSet::new();
    for task_policy in &policy.task_policies {
        if task_policy.task_id.trim().is_empty() {
            return Err(anyhow!("cheapest validated outcome task_id is required"));
        }
        if !seen_task_policies.insert(task_policy.task_id.clone()) {
            return Err(anyhow!(
                "duplicate cheapest validated outcome task policy for {}",
                task_policy.task_id
            ));
        }
        if !input_ids.contains(task_policy.task_id.as_str()) {
            return Err(anyhow!(
                "cheapest validated outcome task {} does not match a scheduler input task_id",
                task_policy.task_id
            ));
        }
        if !valid_cheapest_validated_outcome_claim_boundary(&task_policy.claim_boundary) {
            return Err(anyhow!(
                "cheapest validated outcome task {} claim_boundary must be an approved bounded policy value",
                task_policy.task_id
            ));
        }
        if task_policy.allow_unvalidated_fallback {
            return Err(anyhow!(
                "cheapest validated outcome task {} cannot allow unvalidated fallback",
                task_policy.task_id
            ));
        }
        let requirement = context
            .task_requirements
            .iter()
            .find(|requirement| requirement.task_id == task_policy.task_id)
            .ok_or_else(|| {
                anyhow!(
                    "cheapest validated outcome task {} has no model suitability requirement",
                    task_policy.task_id
                )
            })?;
        select_model_suitability_candidate(context, requirement, Some(policy)).map_err(|err| {
            anyhow!(
                "cheapest validated outcome task {} has no validated affordable candidate: {err}",
                task_policy.task_id
            )
        })?;
    }

    Ok(())
}

pub fn validate_provider_route(
    route: &ProviderRouteV1,
    known_profiles: &BTreeSet<String>,
) -> Result<()> {
    if let Some(profile_ref) = &route.provider_profile_ref {
        if profile_ref.trim().is_empty() {
            return Err(anyhow!("provider route provider_profile_ref is empty"));
        }
        if !known_profiles.contains(profile_ref) {
            return Err(anyhow!(
                "provider route profile ref {profile_ref} is not tracked in provider profile registry"
            ));
        }
    }
    if route.provider_spec_kind.trim().is_empty() {
        return Err(anyhow!("provider route provider_spec_kind is required"));
    }
    if route.model_ref.trim().is_empty() {
        return Err(anyhow!("provider route model_ref is required"));
    }
    if route.model_identity.trim().is_empty() {
        return Err(anyhow!("provider route model_identity is required"));
    }
    if route.runtime_surface.trim().is_empty() {
        return Err(anyhow!("provider route runtime_surface is required"));
    }
    if route.provider_selection_reason.trim().is_empty() {
        return Err(anyhow!(
            "provider route provider_selection_reason is required"
        ));
    }
    if route.route_resolution_trace.is_empty() {
        return Err(anyhow!(
            "provider route route_resolution_trace must include at least one entry"
        ));
    }
    if route.output_contract_ref.trim().is_empty() {
        return Err(anyhow!("provider route output_contract_ref is required"));
    }
    Ok(())
}

pub fn validate_economics_input(input: &SchedulerEconomicsInputV1) -> Result<()> {
    if input.schema_version != SCHEDULER_ECONOMICS_INPUT_SCHEMA_V1 {
        return Err(anyhow!(
            "unsupported scheduler economics input schema: {}",
            input.schema_version
        ));
    }
    if input.task_id.trim().is_empty() {
        return Err(anyhow!("scheduler economics input task_id is required"));
    }
    if input.claim_boundary.trim().is_empty() {
        return Err(anyhow!(
            "scheduler economics input claim_boundary is required"
        ));
    }
    if !input.claim_boundary.contains("not_exact") && !input.claim_boundary.contains("bounded") {
        return Err(anyhow!(
            "scheduler economics input claim_boundary must state bounded or not_exact measurement"
        ));
    }
    if input.dependency_posture == SchedulerDependencyPostureV1::Blocked
        && input.dependencies.is_empty()
    {
        return Err(anyhow!(
            "blocked scheduler economics input must name at least one dependency"
        ));
    }
    for dependency in &input.dependencies {
        if dependency.task_id.trim().is_empty() {
            return Err(anyhow!("scheduler dependency task_id is required"));
        }
    }
    Ok(())
}

pub fn summarize_economics_input(
    input: &SchedulerEconomicsInputV1,
) -> Result<SchedulerEconomicsSummaryV1> {
    validate_economics_input(input)?;

    let blocked = input.dependency_posture == SchedulerDependencyPostureV1::Blocked
        || input.parallelism_potential == SchedulerParallelismPotentialV1::Blocked;
    let lifecycle_cost_score = effort_weight(&input.estimated_effort)
        + cost_weight(&input.estimated_validation_cost)
        + cost_weight(&input.estimated_coordination_cost)
        + risk_weight(&input.risk_level);
    let value_score = expected_value_weight(&input.expected_value) + urgency_weight(&input.urgency);
    let attention_pressure_score = pressure_weight(&input.premium_capacity_pressure)
        + pressure_weight(&input.governor_attention_pressure)
        + u32::from(input.human_required) * 3;
    let parallelism_score = parallelism_weight(&input.parallelism_potential);
    let dependency_posture_score = dependency_posture_weight(&input.dependency_posture);
    let confidence_score = confidence_weight(&input.confidence);

    Ok(SchedulerEconomicsSummaryV1 {
        task_id: input.task_id.clone(),
        blocked,
        lifecycle_cost_score,
        value_score,
        attention_pressure_score,
        parallelism_score,
        dependency_posture_score,
        confidence_score,
        deterministic_rank_key: format!(
            "blocked={};dependency={:02};risk={:02};urgency={:02};value={:02};cost={:02};attention={:02};parallelism={:02};confidence={:02};task={}",
            u8::from(blocked),
            dependency_posture_score,
            risk_weight(&input.risk_level),
            urgency_weight(&input.urgency),
            expected_value_weight(&input.expected_value),
            lifecycle_cost_score,
            attention_pressure_score,
            parallelism_score,
            confidence_score,
            input.task_id
        ),
    })
}

pub fn schedule_economics_bundle(
    bundle: &SchedulerEconomicsInputBundleV1,
) -> Result<CognitiveSchedulerPlanV1> {
    validate_economics_bundle(bundle)?;
    let adjusted_inputs =
        apply_chronosense_scheduler_context(&bundle.inputs, bundle.chronosense_context.as_ref());
    let provider_routes =
        resolve_role_provider_assignments(bundle.role_provider_context.as_ref(), &adjusted_inputs)?;
    let model_suitability = resolve_model_suitability_assignments(
        bundle.model_suitability_context.as_ref(),
        bundle.cheapest_validated_outcome_policy.as_ref(),
    )?;
    let mut decisions = adjusted_inputs
        .iter()
        .map(|input| {
            schedule_economics_input_with_provider_route(
                input,
                provider_routes.get(&input.task_id).cloned(),
                model_suitability.get(&input.task_id).cloned(),
            )
        })
        .collect::<Result<Vec<_>>>()?;
    decisions.sort_by(|left, right| left.scheduling_rank_key.cmp(&right.scheduling_rank_key));
    let recommended_order = decisions
        .iter()
        .map(|decision| decision.task_id.clone())
        .collect::<Vec<_>>();
    Ok(CognitiveSchedulerPlanV1 {
        schema_version: COGNITIVE_SCHEDULER_PLAN_SCHEMA_V1.to_string(),
        source_schema_version: bundle.schema_version.clone(),
        decisions,
        recommended_order,
    })
}

pub fn resolve_model_suitability_assignments(
    context: Option<&ModelSuitabilitySelectionContextV1>,
    cheapest_policy: Option<&CheapestValidatedOutcomePolicyV1>,
) -> Result<BTreeMap<String, ModelSuitabilitySelectionV1>> {
    let Some(context) = context else {
        return Ok(BTreeMap::new());
    };
    let mut selected = BTreeMap::new();
    for requirement in &context.task_requirements {
        selected.insert(
            requirement.task_id.clone(),
            select_model_suitability_candidate(context, requirement, cheapest_policy)?,
        );
    }
    Ok(selected)
}

fn select_model_suitability_candidate(
    context: &ModelSuitabilitySelectionContextV1,
    requirement: &ModelSuitabilityTaskRequirementV1,
    cheapest_policy: Option<&CheapestValidatedOutcomePolicyV1>,
) -> Result<ModelSuitabilitySelectionV1> {
    let task_policy = cheapest_policy.and_then(|policy| {
        policy
            .task_policies
            .iter()
            .find(|task_policy| task_policy.task_id == requirement.task_id)
            .map(|task_policy| (policy, task_policy))
    });
    let mut eligible = context
        .candidates
        .iter()
        .filter(|candidate| {
            model_candidate_matches_requirement(candidate, requirement)
                && cheapest_policy_allows_candidate(candidate, task_policy)
        })
        .collect::<Vec<_>>();
    if eligible.is_empty() {
        return Err(anyhow!(
            "no candidate matched required role and classification"
        ));
    }
    if let Some((policy, _task_policy)) = task_policy {
        eligible.sort_by(|left, right| {
            cheapest_candidate_cost_rank(policy, left)
                .cmp(&cheapest_candidate_cost_rank(policy, right))
                .then_with(|| {
                    model_classification_rank(&right.classification)
                        .cmp(&model_classification_rank(&left.classification))
                })
                .then_with(|| right.selection_priority.cmp(&left.selection_priority))
                .then_with(|| left.provider_profile_ref.cmp(&right.provider_profile_ref))
                .then_with(|| left.model_ref.cmp(&right.model_ref))
                .then_with(|| left.candidate_id.cmp(&right.candidate_id))
        });
    } else {
        eligible.sort_by(|left, right| {
            model_classification_rank(&right.classification)
                .cmp(&model_classification_rank(&left.classification))
                .then_with(|| right.selection_priority.cmp(&left.selection_priority))
                .then_with(|| left.provider_profile_ref.cmp(&right.provider_profile_ref))
                .then_with(|| left.model_ref.cmp(&right.model_ref))
                .then_with(|| left.candidate_id.cmp(&right.candidate_id))
        });
    }
    let selected = eligible[0];
    let selected_policy_evidence = task_policy
        .and_then(|(policy, _task_policy)| cheapest_candidate_evidence(policy, selected));
    let selection_trace = context
        .candidates
        .iter()
        .map(|candidate| {
            let selected_candidate = candidate.candidate_id == selected.candidate_id;
            ModelSuitabilitySelectionTraceV1 {
                candidate_id: candidate.candidate_id.clone(),
                provider_profile_ref: candidate.provider_profile_ref.clone(),
                model_ref: candidate.model_ref.clone(),
                disposition: if selected_candidate {
                    ModelSuitabilityTraceDispositionV1::Selected
                } else {
                    ModelSuitabilityTraceDispositionV1::Rejected
                },
                reason: model_suitability_trace_reason(
                    candidate,
                    requirement,
                    selected_candidate,
                    task_policy,
                ),
                outcome_cost_tier: task_policy
                    .and_then(|(policy, _)| cheapest_candidate_evidence(policy, candidate))
                    .map(|evidence| evidence.outcome_cost_tier.clone()),
                validation_ref: task_policy
                    .and_then(|(policy, _)| cheapest_candidate_evidence(policy, candidate))
                    .map(|evidence| evidence.validation_ref.clone()),
            }
        })
        .collect::<Vec<_>>();
    Ok(ModelSuitabilitySelectionV1 {
        role: requirement.role.clone(),
        selected_candidate_id: selected.candidate_id.clone(),
        provider_profile_ref: selected.provider_profile_ref.clone(),
        provider_family: selected.provider_family.clone(),
        model_ref: selected.model_ref.clone(),
        runtime_surface: selected.runtime_surface.clone(),
        classification: selected.classification.clone(),
        source_ref: selected.source_ref.clone(),
        evidence_digest: selected.evidence_digest.clone(),
        advisory_authority_only: selected.advisory_authority_only,
        claim_boundary: requirement.claim_boundary.clone(),
        outcome_cost_tier: selected_policy_evidence
            .as_ref()
            .map(|evidence| evidence.outcome_cost_tier.clone()),
        validation_ref: selected_policy_evidence
            .as_ref()
            .map(|evidence| evidence.validation_ref.clone()),
        cheapest_validated_outcome: selected_policy_evidence.is_some(),
        selection_trace,
    })
}

fn cheapest_policy_allows_candidate(
    candidate: &ModelSuitabilityCandidateV1,
    task_policy: Option<(
        &CheapestValidatedOutcomePolicyV1,
        &CheapestValidatedOutcomeTaskPolicyV1,
    )>,
) -> bool {
    let Some((policy, task_policy)) = task_policy else {
        return true;
    };
    let Some(evidence) = cheapest_candidate_evidence(policy, candidate) else {
        return false;
    };
    evidence.validated_outcome
        && cost_weight(&evidence.outcome_cost_tier)
            <= cost_weight(&task_policy.max_outcome_cost_tier)
}

fn cheapest_candidate_evidence<'a>(
    policy: &'a CheapestValidatedOutcomePolicyV1,
    candidate: &ModelSuitabilityCandidateV1,
) -> Option<&'a CheapestValidatedCandidateEvidenceV1> {
    policy
        .candidate_evidence
        .iter()
        .find(|evidence| evidence.candidate_id == candidate.candidate_id)
}

fn cheapest_candidate_cost_rank(
    policy: &CheapestValidatedOutcomePolicyV1,
    candidate: &ModelSuitabilityCandidateV1,
) -> u32 {
    cheapest_candidate_evidence(policy, candidate)
        .map(|evidence| cost_weight(&evidence.outcome_cost_tier))
        .unwrap_or(99)
}

fn model_candidate_matches_requirement(
    candidate: &ModelSuitabilityCandidateV1,
    requirement: &ModelSuitabilityTaskRequirementV1,
) -> bool {
    candidate.roles.contains(&requirement.role)
        && model_classification_rank(&candidate.classification)
            >= model_classification_rank(&requirement.minimum_classification)
        && (requirement.allowed_provider_profile_refs.is_empty()
            || requirement
                .allowed_provider_profile_refs
                .contains(&candidate.provider_profile_ref))
}

fn model_suitability_trace_reason(
    candidate: &ModelSuitabilityCandidateV1,
    requirement: &ModelSuitabilityTaskRequirementV1,
    selected: bool,
    task_policy: Option<(
        &CheapestValidatedOutcomePolicyV1,
        &CheapestValidatedOutcomeTaskPolicyV1,
    )>,
) -> String {
    if selected {
        if let Some((policy, _)) = task_policy {
            if let Some(evidence) = cheapest_candidate_evidence(policy, candidate) {
                return format!(
                    "selected by cheapest validated outcome policy with {:?} cost and priority {}",
                    evidence.outcome_cost_tier, candidate.selection_priority
                );
            }
        }
        return format!(
            "selected by bounded role suitability ranking with priority {}",
            candidate.selection_priority
        );
    }
    if !candidate.roles.contains(&requirement.role) {
        return "does not support the requested role".to_string();
    }
    if model_classification_rank(&candidate.classification)
        < model_classification_rank(&requirement.minimum_classification)
    {
        return "classification below task minimum".to_string();
    }
    if !requirement.allowed_provider_profile_refs.is_empty()
        && !requirement
            .allowed_provider_profile_refs
            .contains(&candidate.provider_profile_ref)
    {
        return "provider profile not allowed for this task".to_string();
    }
    if let Some((policy, task_policy)) = task_policy {
        let Some(evidence) = cheapest_candidate_evidence(policy, candidate) else {
            return "no retained cost/validation evidence for cheapest validated outcome policy"
                .to_string();
        };
        if !evidence.validated_outcome {
            return "outcome is not validated for cheapest validated outcome policy".to_string();
        }
        if cost_weight(&evidence.outcome_cost_tier)
            > cost_weight(&task_policy.max_outcome_cost_tier)
        {
            return "outcome cost exceeds task policy maximum".to_string();
        }
    }
    "eligible but not selected by deterministic ranking".to_string()
}

fn valid_model_suitability_claim_boundary(value: &str) -> bool {
    matches!(
        value,
        "bounded_role_suitability_not_authority"
            | "bounded_model_suitability_not_authority"
            | "bounded_current_panel_not_authority"
    )
}

fn valid_cheapest_validated_outcome_claim_boundary(value: &str) -> bool {
    matches!(
        value,
        "bounded_cheapest_validated_outcome_not_exact_cost"
            | "bounded_cost_heuristic_not_authority"
    )
}

fn model_classification_rank(classification: &ModelSuitabilityClassificationV1) -> u32 {
    match classification {
        ModelSuitabilityClassificationV1::UsefulWithLimits => 5,
        ModelSuitabilityClassificationV1::SupportedWithLimits => 4,
        ModelSuitabilityClassificationV1::CandidateOnly => 3,
        ModelSuitabilityClassificationV1::RuntimeUnsuitableForThisPanel => 2,
        ModelSuitabilityClassificationV1::HistoricalOnly => 1,
        ModelSuitabilityClassificationV1::Blocked => 0,
    }
}

pub fn resolve_role_provider_assignments(
    context: Option<&RoleProviderSelectionContextV1>,
    inputs: &[SchedulerEconomicsInputV1],
) -> Result<BTreeMap<String, ProviderRouteV1>> {
    let Some(context) = context else {
        return Ok(BTreeMap::new());
    };
    validate_role_provider_selection_context(context, inputs)?;
    let policy_by_role = context
        .policies
        .iter()
        .map(|policy| (policy.role_profile.clone(), policy))
        .collect::<BTreeMap<_, _>>();
    let mut selected_routes = BTreeMap::new();
    for assignment in &context.assignments {
        let policy = policy_by_role
            .get(&assignment.role_profile)
            .ok_or_else(|| {
                anyhow!(
                    "role provider assignment {} references role {:?} without a policy",
                    assignment.task_id,
                    assignment.role_profile
                )
            })?;
        let selected = policy
            .candidate_routes
            .iter()
            .find(|candidate| candidate.eligible)
            .ok_or_else(|| {
                anyhow!(
                    "role provider policy {:?} has no eligible candidate route",
                    policy.role_profile
                )
            })?;
        let mut route = selected.route.clone();
        route.route_resolution_trace = role_provider_resolution_trace(policy, &selected.route);
        selected_routes.insert(assignment.task_id.clone(), route);
    }
    Ok(selected_routes)
}

fn role_provider_resolution_trace(
    policy: &RoleProviderProfilePolicyV1,
    selected_route: &ProviderRouteV1,
) -> Vec<String> {
    let mut trace = vec![
        format!("role_profile={:?}", policy.role_profile),
        "resolution_policy=ordered_first_eligible_fail_closed".to_string(),
    ];
    for candidate in &policy.candidate_routes {
        let label = candidate
            .route
            .provider_profile_ref
            .as_deref()
            .unwrap_or(candidate.route.model_ref.as_str());
        if candidate.route == *selected_route {
            trace.push(format!("selected={label}"));
        } else if candidate.eligible {
            trace.push(format!(
                "not_selected={label};reason=later_ordered_candidate"
            ));
        } else {
            trace.push(format!(
                "rejected={label};reason={}",
                candidate
                    .ineligibility_reason
                    .as_deref()
                    .unwrap_or("not_eligible")
            ));
        }
    }
    trace
}

pub fn apply_chronosense_scheduler_context(
    inputs: &[SchedulerEconomicsInputV1],
    context: Option<&ChronosenseSchedulerContextV1>,
) -> Vec<SchedulerEconomicsInputV1> {
    let Some(context) = context else {
        return inputs.to_vec();
    };
    inputs
        .iter()
        .map(|input| {
            let mut adjusted = input.clone();
            for signal in context
                .signals
                .iter()
                .filter(|signal| signal.task_id == input.task_id)
            {
                apply_chronosense_signal(&mut adjusted, signal);
            }
            adjusted
        })
        .collect()
}

fn apply_chronosense_signal(
    input: &mut SchedulerEconomicsInputV1,
    signal: &ChronosenseCommitmentSchedulingSignalV1,
) {
    input.urgency = max_urgency(&input.urgency, &signal.temporal_urgency);
    if signal.review_required {
        input.human_required = true;
        input.governor_attention_pressure = max_pressure(
            &input.governor_attention_pressure,
            &SchedulerPressureLevelV1::High,
        );
    }
    match signal.deadline_posture {
        ChronosenseDeadlinePostureV1::Missed => {
            input.urgency = SchedulerUrgencyV1::Immediate;
            input.dependency_posture = SchedulerDependencyPostureV1::Blocked;
            input.governor_attention_pressure = max_pressure(
                &input.governor_attention_pressure,
                &SchedulerPressureLevelV1::High,
            );
            ensure_chronosense_dependency(
                input,
                signal,
                SchedulerDependencyPostureV1::Blocked,
                "chronosense_missed_commitment_blocks_follow_on_until_review",
            );
        }
        ChronosenseDeadlinePostureV1::Due => {
            input.urgency = SchedulerUrgencyV1::Immediate;
            input.governor_attention_pressure = max_pressure(
                &input.governor_attention_pressure,
                &SchedulerPressureLevelV1::High,
            );
            ensure_chronosense_dependency(
                input,
                signal,
                SchedulerDependencyPostureV1::Partial,
                "chronosense_due_commitment_requires_explicit_review",
            );
            if input.dependency_posture == SchedulerDependencyPostureV1::Clear {
                input.dependency_posture = SchedulerDependencyPostureV1::Partial;
            }
        }
        ChronosenseDeadlinePostureV1::Approaching => {
            input.urgency = max_urgency(&input.urgency, &SchedulerUrgencyV1::High);
            input.governor_attention_pressure = max_pressure(
                &input.governor_attention_pressure,
                &SchedulerPressureLevelV1::Medium,
            );
        }
        ChronosenseDeadlinePostureV1::Future | ChronosenseDeadlinePostureV1::None => {}
    }
    match signal.status {
        ChronosenseCommitmentStatusV1::Missed | ChronosenseCommitmentStatusV1::Expired => {
            input.dependency_posture = SchedulerDependencyPostureV1::Blocked;
            ensure_chronosense_dependency(
                input,
                signal,
                SchedulerDependencyPostureV1::Blocked,
                "chronosense_terminal_commitment_state_requires_recovery_before_scheduling",
            );
        }
        ChronosenseCommitmentStatusV1::Deferred if !signal.fulfillment_ready => {
            if input.dependency_posture == SchedulerDependencyPostureV1::Clear {
                input.dependency_posture = SchedulerDependencyPostureV1::Partial;
            }
            ensure_chronosense_dependency(
                input,
                signal,
                SchedulerDependencyPostureV1::Partial,
                "chronosense_deferred_commitment_not_fulfillment_ready",
            );
        }
        ChronosenseCommitmentStatusV1::Proposed
        | ChronosenseCommitmentStatusV1::Accepted
        | ChronosenseCommitmentStatusV1::Active
        | ChronosenseCommitmentStatusV1::Fulfilled
        | ChronosenseCommitmentStatusV1::Deferred
        | ChronosenseCommitmentStatusV1::Canceled => {}
    }
    let capability = format!("chronosense_commitment:{}", signal.commitment_id);
    if !input
        .required_capabilities
        .iter()
        .any(|item| item == &capability)
    {
        input.required_capabilities.push(capability);
    }
}

fn ensure_chronosense_dependency(
    input: &mut SchedulerEconomicsInputV1,
    signal: &ChronosenseCommitmentSchedulingSignalV1,
    status: SchedulerDependencyPostureV1,
    default_reason: &str,
) {
    let dependency_id = format!("chronosense:{}", signal.commitment_id);
    let reason = signal
        .reason
        .clone()
        .unwrap_or_else(|| default_reason.to_string());
    if let Some(existing) = input
        .dependencies
        .iter()
        .find(|dependency| dependency.task_id == dependency_id)
    {
        if dependency_posture_weight(&status) > dependency_posture_weight(&existing.status) {
            let existing = input
                .dependencies
                .iter_mut()
                .find(|dependency| dependency.task_id == dependency_id)
                .expect("existing dependency located for mutation");
            existing.status = status;
            existing.reason = Some(reason);
        }
        return;
    }
    input.dependencies.push(SchedulerDependencyRefV1 {
        task_id: dependency_id,
        status,
        reason: Some(reason),
    });
}

fn max_urgency(left: &SchedulerUrgencyV1, right: &SchedulerUrgencyV1) -> SchedulerUrgencyV1 {
    if urgency_weight(left) >= urgency_weight(right) {
        left.clone()
    } else {
        right.clone()
    }
}

fn max_pressure(
    left: &SchedulerPressureLevelV1,
    right: &SchedulerPressureLevelV1,
) -> SchedulerPressureLevelV1 {
    if pressure_weight(left) >= pressure_weight(right) {
        left.clone()
    } else {
        right.clone()
    }
}

pub fn schedule_economics_input(
    input: &SchedulerEconomicsInputV1,
) -> Result<CognitiveSchedulerDecisionV1> {
    schedule_economics_input_with_provider_route(input, None, None)
}

fn schedule_economics_input_with_provider_route(
    input: &SchedulerEconomicsInputV1,
    provider_route: Option<ProviderRouteV1>,
    model_suitability_selection: Option<ModelSuitabilitySelectionV1>,
) -> Result<CognitiveSchedulerDecisionV1> {
    let summary = summarize_economics_input(input)?;
    let selected_lane = select_lane(input, &summary);
    let reason = decision_reason(input, &summary, &selected_lane);
    let alternatives_considered = alternatives_for(input, &selected_lane);
    let scheduling_rank_key = scheduling_rank_key(input, &summary, &selected_lane);

    Ok(CognitiveSchedulerDecisionV1 {
        schema_version: if provider_route.is_some()
            && model_suitability_selection
                .as_ref()
                .is_some_and(|selection| selection.cheapest_validated_outcome)
        {
            COGNITIVE_SCHEDULER_DECISION_PROVIDER_CHEAPEST_VALIDATED_OUTCOME_SCHEMA_V1.to_string()
        } else if model_suitability_selection
            .as_ref()
            .is_some_and(|selection| selection.cheapest_validated_outcome)
        {
            COGNITIVE_SCHEDULER_DECISION_CHEAPEST_VALIDATED_OUTCOME_SCHEMA_V1.to_string()
        } else if provider_route.is_some() {
            COGNITIVE_SCHEDULER_DECISION_WITH_PROVIDER_ROUTE_SCHEMA_V1.to_string()
        } else if model_suitability_selection.is_some() {
            COGNITIVE_SCHEDULER_DECISION_MODEL_SUITABILITY_SCHEMA_V1.to_string()
        } else {
            COGNITIVE_SCHEDULER_DECISION_SCHEMA_V1.to_string()
        },
        task_id: input.task_id.clone(),
        selected_lane,
        alternatives_considered,
        reason,
        score_breakdown: SchedulerScoreBreakdownV1 {
            lifecycle_cost_score: summary.lifecycle_cost_score,
            value_score: summary.value_score,
            attention_pressure_score: summary.attention_pressure_score,
            parallelism_score: summary.parallelism_score,
            dependency_posture_score: summary.dependency_posture_score,
            confidence_score: summary.confidence_score,
            validation_cost: input.estimated_validation_cost.clone(),
            coordination_cost: input.estimated_coordination_cost.clone(),
            risk: input.risk_level.clone(),
            urgency: input.urgency.clone(),
            expected_value: input.expected_value.clone(),
        },
        dependency_status: input.dependency_posture.clone(),
        manual_override: SchedulerManualOverrideV1 {
            present: input.manual_override.is_some(),
            reason: input.manual_override.clone(),
        },
        provider_route,
        model_suitability_selection,
        confidence: input.confidence.clone(),
        scheduling_rank_key,
    })
}

fn select_lane(
    input: &SchedulerEconomicsInputV1,
    summary: &SchedulerEconomicsSummaryV1,
) -> CognitiveSchedulerLaneV1 {
    if summary.blocked {
        return CognitiveSchedulerLaneV1::Delayed;
    }
    if input.urgency == SchedulerUrgencyV1::Low
        && input.premium_capacity_pressure == SchedulerPressureLevelV1::Constrained
    {
        return CognitiveSchedulerLaneV1::Delayed;
    }
    if should_wait_for_governor_capacity(input) {
        return CognitiveSchedulerLaneV1::Delayed;
    }
    if governor_candidate(input) {
        return CognitiveSchedulerLaneV1::Governor;
    }
    if matches!(
        input.task_type,
        SchedulerTaskTypeV1::Implementation
            | SchedulerTaskTypeV1::Refactor
            | SchedulerTaskTypeV1::SecurityReview
    ) || input.risk_level == SchedulerRiskLevelV1::High
        || input.expected_value == SchedulerExpectedValueV1::Critical
    {
        return CognitiveSchedulerLaneV1::Premium;
    }
    if matches!(
        input.task_type,
        SchedulerTaskTypeV1::Review | SchedulerTaskTypeV1::TestGeneration
    ) || input.estimated_validation_cost != SchedulerCostLevelV1::Low
        || input.estimated_coordination_cost != SchedulerCostLevelV1::Low
    {
        return CognitiveSchedulerLaneV1::CheapRemote;
    }
    CognitiveSchedulerLaneV1::Local
}

fn decision_reason(
    input: &SchedulerEconomicsInputV1,
    summary: &SchedulerEconomicsSummaryV1,
    selected_lane: &CognitiveSchedulerLaneV1,
) -> String {
    match selected_lane {
        CognitiveSchedulerLaneV1::Delayed if summary.blocked => {
            "delayed because dependency or parallelism posture is blocked".to_string()
        }
        CognitiveSchedulerLaneV1::Delayed => {
            if governor_candidate(input)
                && input.governor_attention_pressure == SchedulerPressureLevelV1::Constrained
            {
                "delayed because governor attention is constrained and the task is not an immediate critical decision".to_string()
            } else {
                "delayed because urgency is low while premium capacity is constrained".to_string()
            }
        }
        CognitiveSchedulerLaneV1::Governor => {
            "routed to governor because human authority, release/architecture scope, critical risk, or manual override is present".to_string()
        }
        CognitiveSchedulerLaneV1::Premium => {
            "routed to premium cognition because the work is high risk, implementation/security/refactor shaped, or critical value".to_string()
        }
        CognitiveSchedulerLaneV1::CheapRemote => {
            "routed to cheap remote cognition because review/test generation or non-low validation and coordination burden can be parallelized".to_string()
        }
        CognitiveSchedulerLaneV1::Local => {
            format!(
                "routed local because {} is low-risk, low-cost, dependency-clear work",
                input.task_id
            )
        }
    }
}

fn alternatives_for(
    input: &SchedulerEconomicsInputV1,
    selected_lane: &CognitiveSchedulerLaneV1,
) -> Vec<SchedulerAlternativeV1> {
    all_lanes()
        .into_iter()
        .filter(|lane| lane != selected_lane)
        .map(|lane| SchedulerAlternativeV1 {
            disposition: alternative_disposition(input, &lane),
            reason: alternative_reason(input, &lane, selected_lane),
            lane,
        })
        .collect()
}

fn all_lanes() -> Vec<CognitiveSchedulerLaneV1> {
    vec![
        CognitiveSchedulerLaneV1::Local,
        CognitiveSchedulerLaneV1::CheapRemote,
        CognitiveSchedulerLaneV1::Premium,
        CognitiveSchedulerLaneV1::Governor,
        CognitiveSchedulerLaneV1::Delayed,
    ]
}

fn alternative_disposition(
    input: &SchedulerEconomicsInputV1,
    lane: &CognitiveSchedulerLaneV1,
) -> SchedulerAlternativeDispositionV1 {
    if matches!(lane, CognitiveSchedulerLaneV1::Delayed)
        && input.dependency_posture == SchedulerDependencyPostureV1::Partial
    {
        return SchedulerAlternativeDispositionV1::Fallback;
    }
    if matches!(lane, CognitiveSchedulerLaneV1::CheapRemote)
        && input.parallelism_potential == SchedulerParallelismPotentialV1::HighlyParallelizable
    {
        return SchedulerAlternativeDispositionV1::Fallback;
    }
    SchedulerAlternativeDispositionV1::Rejected
}

fn alternative_reason(
    input: &SchedulerEconomicsInputV1,
    lane: &CognitiveSchedulerLaneV1,
    selected_lane: &CognitiveSchedulerLaneV1,
) -> String {
    if lane == selected_lane {
        return "selected".to_string();
    }
    match lane {
        CognitiveSchedulerLaneV1::Local => {
            "local lane rejected when risk, validation, coordination, or urgency exceeds routine local work".to_string()
        }
        CognitiveSchedulerLaneV1::CheapRemote => {
            if input.parallelism_potential == SchedulerParallelismPotentialV1::HighlyParallelizable
            {
                "cheap remote remains a fallback for highly parallelizable support work".to_string()
            } else {
                "cheap remote rejected because the selected lane better matches authority, risk, or cost posture".to_string()
            }
        }
        CognitiveSchedulerLaneV1::Premium => {
            "premium lane rejected unless high-risk implementation, security/refactor work, or critical value justifies scarce capacity".to_string()
        }
        CognitiveSchedulerLaneV1::Governor => {
            "governor lane rejected unless human authority, critical risk, release/architecture scope, or manual override is required".to_string()
        }
        CognitiveSchedulerLaneV1::Delayed => {
            if input.dependency_posture == SchedulerDependencyPostureV1::Partial {
                "delayed lane remains a fallback if partial dependency evidence does not land".to_string()
            } else {
                "delayed lane rejected because the task is schedulable now".to_string()
            }
        }
    }
}

fn scheduling_rank_key(
    input: &SchedulerEconomicsInputV1,
    summary: &SchedulerEconomicsSummaryV1,
    selected_lane: &CognitiveSchedulerLaneV1,
) -> String {
    format!(
        "blocked={};deferred={};dependency={:02};gate={:02};risk={:02};urgency={:02};value={:02};validation={:02};premium_pressure={:02};coordination={:02};parallelism={:02};confidence={:02};task={}",
        u8::from(summary.blocked),
        deferred_lane_rank(selected_lane),
        summary.dependency_posture_score,
        gate_priority(input),
        reverse_weight(risk_weight(&input.risk_level)),
        reverse_weight(urgency_weight(&input.urgency)),
        reverse_weight(expected_value_weight(&input.expected_value)),
        cost_weight(&input.estimated_validation_cost),
        pressure_weight(&input.premium_capacity_pressure),
        cost_weight(&input.estimated_coordination_cost),
        reverse_weight(summary.parallelism_score),
        reverse_weight(summary.confidence_score),
        input.task_id
    )
}

fn deferred_lane_rank(selected_lane: &CognitiveSchedulerLaneV1) -> u8 {
    u8::from(matches!(selected_lane, CognitiveSchedulerLaneV1::Delayed))
}

fn governor_candidate(input: &SchedulerEconomicsInputV1) -> bool {
    input.manual_override.is_some()
        || input.human_required
        || input.risk_level == SchedulerRiskLevelV1::Critical
        || matches!(
            input.task_type,
            SchedulerTaskTypeV1::ReleaseGate | SchedulerTaskTypeV1::Architecture
        )
}

fn should_wait_for_governor_capacity(input: &SchedulerEconomicsInputV1) -> bool {
    governor_candidate(input)
        && input.governor_attention_pressure == SchedulerPressureLevelV1::Constrained
        && input.urgency != SchedulerUrgencyV1::Immediate
        && input.risk_level != SchedulerRiskLevelV1::Critical
}

fn gate_priority(input: &SchedulerEconomicsInputV1) -> u32 {
    if governor_candidate(input) {
        0
    } else if matches!(
        input.task_type,
        SchedulerTaskTypeV1::Implementation
            | SchedulerTaskTypeV1::Refactor
            | SchedulerTaskTypeV1::SecurityReview
    ) || input.risk_level == SchedulerRiskLevelV1::High
    {
        1
    } else {
        2
    }
}

fn reverse_weight(value: u32) -> u32 {
    99 - value
}

fn effort_weight(value: &SchedulerEffortV1) -> u32 {
    match value {
        SchedulerEffortV1::Small => 1,
        SchedulerEffortV1::Medium => 2,
        SchedulerEffortV1::Large => 3,
    }
}

fn cost_weight(value: &SchedulerCostLevelV1) -> u32 {
    match value {
        SchedulerCostLevelV1::Low => 1,
        SchedulerCostLevelV1::Medium => 2,
        SchedulerCostLevelV1::High => 3,
    }
}

fn risk_weight(value: &SchedulerRiskLevelV1) -> u32 {
    match value {
        SchedulerRiskLevelV1::Low => 1,
        SchedulerRiskLevelV1::Medium => 2,
        SchedulerRiskLevelV1::High => 3,
        SchedulerRiskLevelV1::Critical => 4,
    }
}

fn urgency_weight(value: &SchedulerUrgencyV1) -> u32 {
    match value {
        SchedulerUrgencyV1::Low => 1,
        SchedulerUrgencyV1::Normal => 2,
        SchedulerUrgencyV1::High => 3,
        SchedulerUrgencyV1::Immediate => 4,
    }
}

fn expected_value_weight(value: &SchedulerExpectedValueV1) -> u32 {
    match value {
        SchedulerExpectedValueV1::Low => 1,
        SchedulerExpectedValueV1::Medium => 2,
        SchedulerExpectedValueV1::High => 3,
        SchedulerExpectedValueV1::Critical => 4,
    }
}

fn pressure_weight(value: &SchedulerPressureLevelV1) -> u32 {
    match value {
        SchedulerPressureLevelV1::Low => 1,
        SchedulerPressureLevelV1::Medium => 2,
        SchedulerPressureLevelV1::High => 3,
        SchedulerPressureLevelV1::Constrained => 4,
    }
}

fn parallelism_weight(value: &SchedulerParallelismPotentialV1) -> u32 {
    match value {
        SchedulerParallelismPotentialV1::Blocked => 0,
        SchedulerParallelismPotentialV1::Serial => 1,
        SchedulerParallelismPotentialV1::Parallelizable => 2,
        SchedulerParallelismPotentialV1::HighlyParallelizable => 3,
    }
}

fn dependency_posture_weight(value: &SchedulerDependencyPostureV1) -> u32 {
    match value {
        SchedulerDependencyPostureV1::Clear => 0,
        SchedulerDependencyPostureV1::Partial => 1,
        SchedulerDependencyPostureV1::Blocked => 2,
    }
}

fn confidence_weight(value: &SchedulerConfidenceV1) -> u32 {
    match value {
        SchedulerConfidenceV1::Low => 1,
        SchedulerConfidenceV1::Medium => 2,
        SchedulerConfidenceV1::High => 3,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIXTURE: &str = include_str!("../tests/fixtures/scheduler/economics_inputs_v1.json");
    const MODEL_SUITABILITY_FIXTURE: &str =
        include_str!("../tests/fixtures/scheduler/model_suitability_inputs_v1.json");
    const CHEAPEST_VALIDATED_OUTCOME_FIXTURE: &str =
        include_str!("../tests/fixtures/scheduler/cheapest_validated_outcome_inputs_v1.json");

    #[test]
    fn scheduler_economics_bundle_fixture_parses_and_summarizes() {
        let bundle = parse_economics_bundle_json(FIXTURE).expect("fixture parses");
        assert_eq!(bundle.inputs.len(), 7);
        assert!(bundle
            .included_concepts
            .contains(&"validation_cost".to_string()));
        assert!(bundle
            .deferred_concepts
            .contains(&"live_provider_price_lookup".to_string()));

        let summaries = bundle
            .inputs
            .iter()
            .map(summarize_economics_input)
            .collect::<Result<Vec<_>>>()
            .expect("summaries");
        assert_eq!(summaries[0].task_id, "docs-status-check");
        assert!(!summaries[0].blocked);
        assert!(summaries
            .iter()
            .any(|summary| summary.task_id == "blocked-proof" && summary.blocked));
    }

    #[test]
    fn model_suitability_context_selects_bounded_role_candidate() {
        let bundle =
            parse_economics_bundle_json(MODEL_SUITABILITY_FIXTURE).expect("fixture parses");
        assert_eq!(
            bundle.schema_version,
            SCHEDULER_MODEL_SUITABILITY_INPUT_BUNDLE_SCHEMA_V1
        );

        let plan = schedule_economics_bundle(&bundle).expect("model suitability plan");
        assert_eq!(
            plan.source_schema_version,
            SCHEDULER_MODEL_SUITABILITY_INPUT_BUNDLE_SCHEMA_V1
        );
        let review = decision(&plan, "first-pass-review");
        assert_eq!(
            review.schema_version,
            COGNITIVE_SCHEDULER_DECISION_MODEL_SUITABILITY_SCHEMA_V1
        );
        let selection = review
            .model_suitability_selection
            .as_ref()
            .expect("review task has model suitability selection");
        assert_eq!(selection.role, ModelSuitabilityRoleV1::Reviewer);
        assert_eq!(selection.selected_candidate_id, "openrouter:gpt-5.4");
        assert_eq!(
            selection.provider_profile_ref,
            "unprofiled:openrouter:openai/gpt-5.4"
        );
        assert_eq!(
            selection.classification,
            ModelSuitabilityClassificationV1::UsefulWithLimits
        );
        assert!(selection.advisory_authority_only);
        assert_eq!(
            selection.claim_boundary,
            "bounded_role_suitability_not_authority"
        );
        assert!(selection.selection_trace.iter().any(|trace| {
            trace.candidate_id == "local:gemma4-e2b"
                && trace.disposition == ModelSuitabilityTraceDispositionV1::Rejected
                && trace.reason.contains("does not support the requested role")
        }));

        let docs = decision(&plan, "docs-status-check");
        assert_eq!(docs.schema_version, COGNITIVE_SCHEDULER_DECISION_SCHEMA_V1);
        assert!(docs.model_suitability_selection.is_none());
        assert!(docs.provider_route.is_none());
    }

    #[test]
    fn cheapest_validated_outcome_policy_selects_lowest_cost_valid_candidate() {
        let bundle = parse_economics_bundle_json(CHEAPEST_VALIDATED_OUTCOME_FIXTURE)
            .expect("fixture parses");
        assert_eq!(
            bundle.schema_version,
            SCHEDULER_PROVIDER_CHEAPEST_VALIDATED_OUTCOME_INPUT_BUNDLE_SCHEMA_V1
        );

        let plan = schedule_economics_bundle(&bundle).expect("cheapest validated outcome plan");
        assert_eq!(
            plan.source_schema_version,
            SCHEDULER_PROVIDER_CHEAPEST_VALIDATED_OUTCOME_INPUT_BUNDLE_SCHEMA_V1
        );
        let review = decision(&plan, "first-pass-review");
        assert_eq!(
            review.schema_version,
            COGNITIVE_SCHEDULER_DECISION_PROVIDER_CHEAPEST_VALIDATED_OUTCOME_SCHEMA_V1
        );
        let route = review
            .provider_route
            .as_ref()
            .expect("combined policy keeps provider route");
        assert_eq!(
            route.provider_profile_ref.as_deref(),
            Some("chatgpt:gpt-5.3-codex")
        );
        let selection = review
            .model_suitability_selection
            .as_ref()
            .expect("review task has cheapest validated outcome selection");
        assert_eq!(selection.selected_candidate_id, "gemini:gemini-2.5-flash");
        assert_eq!(selection.outcome_cost_tier, Some(SchedulerCostLevelV1::Low));
        assert!(selection.cheapest_validated_outcome);
        assert_eq!(
            selection.validation_ref.as_deref(),
            Some(
                "docs/milestones/v0.91.7/review/provider/artifacts/cheapest_validated_cost_table_4674.json"
            )
        );
        assert!(selection.selection_trace.iter().any(|trace| {
            trace.candidate_id == "openrouter:gpt-5.4"
                && trace.reason == "outcome cost exceeds task policy maximum"
                && trace.outcome_cost_tier == Some(SchedulerCostLevelV1::High)
        }));
        assert!(selection.selection_trace.iter().any(|trace| {
            trace.candidate_id == "local:gemma4-e2b"
                && trace.reason == "classification below task minimum"
        }));
    }

    #[test]
    fn cheapest_validated_outcome_policy_requires_schema_bump() {
        let mut bundle = parse_economics_bundle_json(CHEAPEST_VALIDATED_OUTCOME_FIXTURE)
            .expect("fixture parses");
        bundle.schema_version = SCHEDULER_MODEL_SUITABILITY_INPUT_BUNDLE_SCHEMA_V1.to_string();
        bundle.role_provider_context = None;
        let err = validate_economics_bundle(&bundle).expect_err("schema bump required");
        assert!(err
            .to_string()
            .contains("cheapest_validated_outcome_policy requires scheduler bundle schema"));
    }

    #[test]
    fn cheapest_validated_outcome_policy_rejects_unvalidated_candidate() {
        let mut bundle = parse_economics_bundle_json(CHEAPEST_VALIDATED_OUTCOME_FIXTURE)
            .expect("fixture parses");
        let policy = bundle
            .cheapest_validated_outcome_policy
            .as_mut()
            .expect("cheapest policy");
        policy.candidate_evidence[1].validated_outcome = false;
        let err = validate_economics_bundle(&bundle).expect_err("unvalidated outcome rejected");
        assert!(err.to_string().contains("must have validated_outcome=true"));
    }

    #[test]
    fn cheapest_validated_outcome_policy_rejects_uncited_validation_ref() {
        let mut bundle = parse_economics_bundle_json(CHEAPEST_VALIDATED_OUTCOME_FIXTURE)
            .expect("fixture parses");
        let policy = bundle
            .cheapest_validated_outcome_policy
            .as_mut()
            .expect("cheapest policy");
        policy.candidate_evidence[1].validation_ref =
            "docs/milestones/v0.91.7/review/provider/artifacts/missing-cost-table.json".to_string();
        let err = validate_economics_bundle(&bundle).expect_err("uncited validation rejected");
        assert!(err
            .to_string()
            .contains("validation_ref is not retained in evidence_refs"));
    }

    #[test]
    fn cheapest_validated_outcome_policy_rejects_mismatched_candidate_source_ref() {
        let mut bundle = parse_economics_bundle_json(CHEAPEST_VALIDATED_OUTCOME_FIXTURE)
            .expect("fixture parses");
        let policy = bundle
            .cheapest_validated_outcome_policy
            .as_mut()
            .expect("cheapest policy");
        policy.candidate_evidence[1].candidate_source_ref =
            "docs/milestones/v0.91.6/review/provider/openrouter_current_models/openrouter_current_model_suitability_state_2026-06-22.json".to_string();
        let err = validate_economics_bundle(&bundle).expect_err("mismatched source rejected");
        assert!(err
            .to_string()
            .contains("candidate_source_ref does not match model suitability source_ref"));
    }

    #[test]
    fn model_suitability_context_requires_schema_bump() {
        let mut bundle =
            parse_economics_bundle_json(MODEL_SUITABILITY_FIXTURE).expect("fixture parses");
        bundle.schema_version = SCHEDULER_ECONOMICS_INPUT_BUNDLE_SCHEMA_V1.to_string();
        let err = validate_economics_bundle(&bundle).expect_err("schema bump required");
        assert!(err
            .to_string()
            .contains("model_suitability_context requires scheduler bundle schema"));
    }

    #[test]
    fn model_suitability_schema_requires_context() {
        let mut bundle =
            parse_economics_bundle_json(MODEL_SUITABILITY_FIXTURE).expect("fixture parses");
        bundle.model_suitability_context = None;
        let err = validate_economics_bundle(&bundle).expect_err("context required");
        assert!(err
            .to_string()
            .contains("requires model_suitability_context"));
    }

    #[test]
    fn model_suitability_context_rejects_non_advisory_candidate() {
        let mut bundle =
            parse_economics_bundle_json(MODEL_SUITABILITY_FIXTURE).expect("fixture parses");
        let context = bundle
            .model_suitability_context
            .as_mut()
            .expect("model context");
        context.candidates[0].advisory_authority_only = false;
        let err = validate_economics_bundle(&bundle).expect_err("non advisory rejected");
        assert!(err.to_string().contains("must be advisory_authority_only"));
    }

    #[test]
    fn model_suitability_context_rejects_unmatched_role_requirement() {
        let mut bundle =
            parse_economics_bundle_json(MODEL_SUITABILITY_FIXTURE).expect("fixture parses");
        let context = bundle
            .model_suitability_context
            .as_mut()
            .expect("model context");
        context.task_requirements[0].role = ModelSuitabilityRoleV1::Worker;
        context.task_requirements[0].allowed_provider_profile_refs =
            vec!["unprofiled:gemini:gemini-2.5-flash".to_string()];
        let err = validate_economics_bundle(&bundle).expect_err("no eligible worker");
        assert!(err.to_string().contains("no eligible candidate"));
    }

    #[test]
    fn model_suitability_context_rejects_overclaiming_claim_boundary() {
        let mut bundle =
            parse_economics_bundle_json(MODEL_SUITABILITY_FIXTURE).expect("fixture parses");
        let context = bundle
            .model_suitability_context
            .as_mut()
            .expect("model context");
        context.task_requirements[0].claim_boundary = "unbounded_authority".to_string();
        let err = validate_economics_bundle(&bundle).expect_err("overclaim rejected");
        assert!(err.to_string().contains("claim_boundary must be one of"));
    }

    #[test]
    fn model_suitability_context_rejects_uncited_candidate_source() {
        let mut bundle =
            parse_economics_bundle_json(MODEL_SUITABILITY_FIXTURE).expect("fixture parses");
        let context = bundle
            .model_suitability_context
            .as_mut()
            .expect("model context");
        context.candidates[0].source_ref =
            "docs/milestones/v0.91.7/review/provider/missing-source.md".to_string();
        let err = validate_economics_bundle(&bundle).expect_err("uncited source rejected");
        assert!(err
            .to_string()
            .contains("source_ref is not retained in evidence_refs"));
    }

    #[test]
    fn scheduler_economics_rank_key_is_deterministic() {
        let bundle = parse_economics_bundle_json(FIXTURE).expect("fixture parses");
        let input = bundle
            .inputs
            .iter()
            .find(|input| input.task_id == "premium-code-repair")
            .expect("premium fixture");

        let first = summarize_economics_input(input).expect("first summary");
        let second = summarize_economics_input(input).expect("second summary");
        assert_eq!(first, second);
        assert!(first
            .deterministic_rank_key
            .ends_with("task=premium-code-repair"));
    }

    #[test]
    fn scheduler_economics_partial_dependency_posture_is_explicit() {
        let bundle = parse_economics_bundle_json(FIXTURE).expect("fixture parses");
        let input = bundle
            .inputs
            .iter()
            .find(|input| input.task_id == "partial-dependency-review")
            .expect("partial dependency fixture");

        let summary = summarize_economics_input(input).expect("summary");
        assert!(!summary.blocked);
        assert_eq!(summary.dependency_posture_score, 1);
        assert_eq!(summary.confidence_score, 2);
        assert!(summary.deterministic_rank_key.contains("dependency=01"));
        assert!(summary.deterministic_rank_key.contains("confidence=02"));
    }

    #[test]
    fn cognitive_scheduler_plan_routes_fixture_lanes() {
        let bundle = parse_economics_bundle_json(FIXTURE).expect("fixture parses");
        let plan = schedule_economics_bundle(&bundle).expect("scheduler plan");
        assert_eq!(plan.schema_version, COGNITIVE_SCHEDULER_PLAN_SCHEMA_V1);
        assert_eq!(
            plan.source_schema_version,
            SCHEDULER_ECONOMICS_INPUT_BUNDLE_SCHEMA_V1
        );
        assert_eq!(plan.decisions.len(), 7);

        assert_lane(
            &plan,
            "release-authority",
            CognitiveSchedulerLaneV1::Governor,
        );
        assert_lane(
            &plan,
            "premium-code-repair",
            CognitiveSchedulerLaneV1::Premium,
        );
        assert_lane(
            &plan,
            "first-pass-review",
            CognitiveSchedulerLaneV1::CheapRemote,
        );
        assert_lane(
            &plan,
            "partial-dependency-review",
            CognitiveSchedulerLaneV1::CheapRemote,
        );
        assert_lane(&plan, "docs-status-check", CognitiveSchedulerLaneV1::Local);
        assert_lane(
            &plan,
            "low-urgency-cleanup",
            CognitiveSchedulerLaneV1::Delayed,
        );
        assert_lane(&plan, "blocked-proof", CognitiveSchedulerLaneV1::Delayed);

        let blocked = decision(&plan, "blocked-proof");
        assert_eq!(
            blocked.reason,
            "delayed because dependency or parallelism posture is blocked"
        );
        assert_eq!(
            blocked.dependency_status,
            SchedulerDependencyPostureV1::Blocked
        );
    }

    #[test]
    fn role_provider_context_selects_first_eligible_tracked_profile_for_scheduler_decision() {
        let mut bundle = parse_economics_bundle_json(FIXTURE).expect("fixture parses");
        bundle.schema_version =
            SCHEDULER_ECONOMICS_INPUT_BUNDLE_WITH_PROVIDER_ROUTE_SCHEMA_V1.to_string();
        bundle.role_provider_context = Some(role_provider_context_fixture());

        let plan = schedule_economics_bundle(&bundle).expect("provider-aware scheduler plan");
        assert_eq!(
            plan.source_schema_version,
            SCHEDULER_ECONOMICS_INPUT_BUNDLE_WITH_PROVIDER_ROUTE_SCHEMA_V1
        );
        let review = decision(&plan, "first-pass-review");
        assert_eq!(
            review.schema_version,
            COGNITIVE_SCHEDULER_DECISION_WITH_PROVIDER_ROUTE_SCHEMA_V1
        );
        let route = review
            .provider_route
            .as_ref()
            .expect("review task has provider route");

        assert_eq!(
            route.provider_profile_ref.as_deref(),
            Some("chatgpt:gpt-5.3-codex")
        );
        assert_eq!(route.provider_spec_kind, "http");
        assert_eq!(route.model_ref, "gpt-5.3-codex");
        assert!(route
            .route_resolution_trace
            .contains(&"resolution_policy=ordered_first_eligible_fail_closed".to_string()));
        assert!(route.route_resolution_trace.iter().any(|entry| {
            entry == "rejected=ollama:qwen2.5-7b;reason=local model not yet proven for reviewer role"
        }));
        assert!(route
            .route_resolution_trace
            .contains(&"selected=chatgpt:gpt-5.3-codex".to_string()));

        let docs = decision(&plan, "docs-status-check");
        assert_eq!(docs.schema_version, COGNITIVE_SCHEDULER_DECISION_SCHEMA_V1);
        assert!(docs.provider_route.is_none());
    }

    #[test]
    fn role_provider_context_requires_provider_route_bundle_schema() {
        let mut bundle = parse_economics_bundle_json(FIXTURE).expect("fixture parses");
        bundle.role_provider_context = Some(role_provider_context_fixture());

        let err = schedule_economics_bundle(&bundle).expect_err("old bundle schema rejected");
        assert!(err
            .to_string()
            .contains(SCHEDULER_ECONOMICS_INPUT_BUNDLE_WITH_PROVIDER_ROUTE_SCHEMA_V1));
    }

    #[test]
    fn role_provider_context_rejects_untracked_provider_profile_ref() {
        let mut bundle = parse_economics_bundle_json(FIXTURE).expect("fixture parses");
        let mut context = role_provider_context_fixture();
        context.policies[0].candidate_routes[1]
            .route
            .provider_profile_ref = Some("unprofiled:openrouter:gpt-5.4".to_string());
        bundle.schema_version =
            SCHEDULER_ECONOMICS_INPUT_BUNDLE_WITH_PROVIDER_ROUTE_SCHEMA_V1.to_string();
        bundle.role_provider_context = Some(context);

        let err = schedule_economics_bundle(&bundle).expect_err("untracked profile rejected");
        assert!(err
            .to_string()
            .contains("is not tracked in provider profile registry"));
    }

    #[test]
    fn role_provider_context_rejects_assignment_without_policy() {
        let mut bundle = parse_economics_bundle_json(FIXTURE).expect("fixture parses");
        let mut context = role_provider_context_fixture();
        context.assignments[0].role_profile = RoleProviderProfileV1::TesterProvider;
        bundle.schema_version =
            SCHEDULER_ECONOMICS_INPUT_BUNDLE_WITH_PROVIDER_ROUTE_SCHEMA_V1.to_string();
        bundle.role_provider_context = Some(context);

        let err = schedule_economics_bundle(&bundle).expect_err("missing policy rejected");
        assert!(err.to_string().contains("TesterProvider"));
        assert!(err.to_string().contains("without a policy"));
    }

    #[test]
    fn role_provider_context_rejects_policy_without_eligible_route() {
        let mut bundle = parse_economics_bundle_json(FIXTURE).expect("fixture parses");
        let mut context = role_provider_context_fixture();
        for candidate in &mut context.policies[0].candidate_routes {
            candidate.eligible = false;
            candidate.ineligibility_reason = Some("not available in this proof".to_string());
        }
        bundle.schema_version =
            SCHEDULER_ECONOMICS_INPUT_BUNDLE_WITH_PROVIDER_ROUTE_SCHEMA_V1.to_string();
        bundle.role_provider_context = Some(context);

        let err = schedule_economics_bundle(&bundle).expect_err("no eligible route rejected");
        assert!(err.to_string().contains("has no eligible candidate route"));
    }

    #[test]
    fn role_provider_context_rejects_duplicate_task_assignment() {
        let mut bundle = parse_economics_bundle_json(FIXTURE).expect("fixture parses");
        let mut context = role_provider_context_fixture();
        context.assignments.push(RoleProviderTaskAssignmentV1 {
            task_id: "first-pass-review".to_string(),
            role_profile: RoleProviderProfileV1::ReviewerProvider,
        });
        bundle.schema_version =
            SCHEDULER_ECONOMICS_INPUT_BUNDLE_WITH_PROVIDER_ROUTE_SCHEMA_V1.to_string();
        bundle.role_provider_context = Some(context);

        let err = schedule_economics_bundle(&bundle).expect_err("duplicate assignment rejected");
        assert!(err
            .to_string()
            .contains("duplicate role provider assignment for task first-pass-review"));
    }

    #[test]
    fn chronosense_context_raises_approaching_deadline_priority() {
        let mut bundle = parse_economics_bundle_json(FIXTURE).expect("fixture parses");
        bundle.chronosense_context = Some(ChronosenseSchedulerContextV1 {
            schema_version: CHRONOSENSE_SCHEDULER_CONTEXT_SCHEMA_V1.to_string(),
            contract_schema_version: COMMITMENT_DEADLINE_SCHEMA.to_string(),
            generated_from: "chronosense commitment retrieval fixture".to_string(),
            signals: vec![ChronosenseCommitmentSchedulingSignalV1 {
                task_id: "docs-status-check".to_string(),
                commitment_id: "commitment-docs-before-review".to_string(),
                status: ChronosenseCommitmentStatusV1::Active,
                deadline_posture: ChronosenseDeadlinePostureV1::Approaching,
                deadline_frame: Some(ChronosenseDeadlineFrameV1::ReviewGate),
                temporal_urgency: SchedulerUrgencyV1::High,
                fulfillment_ready: true,
                review_required: false,
                reason: Some("review gate is approaching".to_string()),
            }],
        });

        let plan = schedule_economics_bundle(&bundle).expect("chronosense-aware plan");
        let docs = decision(&plan, "docs-status-check");
        assert_eq!(docs.score_breakdown.urgency, SchedulerUrgencyV1::High);
        assert!(docs.scheduling_rank_key.contains("urgency=96"));
        assert!(docs
            .reason
            .contains("low-risk, low-cost, dependency-clear work"));
    }

    #[test]
    fn chronosense_context_blocks_missed_commitment_until_review() {
        let mut bundle = parse_economics_bundle_json(FIXTURE).expect("fixture parses");
        bundle.chronosense_context = Some(ChronosenseSchedulerContextV1 {
            schema_version: CHRONOSENSE_SCHEDULER_CONTEXT_SCHEMA_V1.to_string(),
            contract_schema_version: COMMITMENT_DEADLINE_SCHEMA.to_string(),
            generated_from: "chronosense missed commitment retrieval fixture".to_string(),
            signals: vec![ChronosenseCommitmentSchedulingSignalV1 {
                task_id: "first-pass-review".to_string(),
                commitment_id: "commitment-review-before-closeout".to_string(),
                status: ChronosenseCommitmentStatusV1::Missed,
                deadline_posture: ChronosenseDeadlinePostureV1::Missed,
                deadline_frame: Some(ChronosenseDeadlineFrameV1::WallClock),
                temporal_urgency: SchedulerUrgencyV1::Immediate,
                fulfillment_ready: false,
                review_required: true,
                reason: Some("missed closeout commitment must be reviewed".to_string()),
            }],
        });

        let adjusted = apply_chronosense_scheduler_context(
            &bundle.inputs,
            bundle.chronosense_context.as_ref(),
        );
        let review_input = adjusted
            .iter()
            .find(|input| input.task_id == "first-pass-review")
            .expect("adjusted review input");
        assert_eq!(
            review_input.dependency_posture,
            SchedulerDependencyPostureV1::Blocked
        );
        assert_eq!(review_input.urgency, SchedulerUrgencyV1::Immediate);
        assert!(review_input.human_required);
        assert!(review_input
            .required_capabilities
            .contains(&"chronosense_commitment:commitment-review-before-closeout".to_string()));

        let plan = schedule_economics_bundle(&bundle).expect("chronosense-aware plan");
        let review = decision(&plan, "first-pass-review");
        assert_eq!(review.selected_lane, CognitiveSchedulerLaneV1::Delayed);
        assert_eq!(
            review.dependency_status,
            SchedulerDependencyPostureV1::Blocked
        );
        assert_eq!(
            review.reason,
            "delayed because dependency or parallelism posture is blocked"
        );
        assert!(review.scheduling_rank_key.starts_with("blocked=1"));
    }

    #[test]
    fn chronosense_context_due_commitment_requires_explicit_review() {
        let mut bundle = parse_economics_bundle_json(FIXTURE).expect("fixture parses");
        bundle.chronosense_context = Some(ChronosenseSchedulerContextV1 {
            schema_version: CHRONOSENSE_SCHEDULER_CONTEXT_SCHEMA_V1.to_string(),
            contract_schema_version: COMMITMENT_DEADLINE_SCHEMA.to_string(),
            generated_from: "chronosense due commitment retrieval fixture".to_string(),
            signals: vec![ChronosenseCommitmentSchedulingSignalV1 {
                task_id: "premium-code-repair".to_string(),
                commitment_id: "commitment-repair-before-release".to_string(),
                status: ChronosenseCommitmentStatusV1::Active,
                deadline_posture: ChronosenseDeadlinePostureV1::Due,
                deadline_frame: Some(ChronosenseDeadlineFrameV1::ContinuityRelative),
                temporal_urgency: SchedulerUrgencyV1::High,
                fulfillment_ready: false,
                review_required: true,
                reason: Some("release commitment is due now".to_string()),
            }],
        });

        let adjusted = apply_chronosense_scheduler_context(
            &bundle.inputs,
            bundle.chronosense_context.as_ref(),
        );
        let repair_input = adjusted
            .iter()
            .find(|input| input.task_id == "premium-code-repair")
            .expect("adjusted repair input");
        assert_eq!(
            repair_input.dependency_posture,
            SchedulerDependencyPostureV1::Partial
        );
        assert_eq!(repair_input.urgency, SchedulerUrgencyV1::Immediate);
        assert!(repair_input.human_required);
        assert!(repair_input.dependencies.iter().any(|dependency| {
            dependency.task_id == "chronosense:commitment-repair-before-release"
                && dependency.status == SchedulerDependencyPostureV1::Partial
        }));

        let plan = schedule_economics_bundle(&bundle).expect("chronosense-aware plan");
        let repair = decision(&plan, "premium-code-repair");
        assert_eq!(repair.selected_lane, CognitiveSchedulerLaneV1::Governor);
        assert_eq!(
            repair.dependency_status,
            SchedulerDependencyPostureV1::Partial
        );
        assert_eq!(
            repair.score_breakdown.urgency,
            SchedulerUrgencyV1::Immediate
        );
        assert!(repair.scheduling_rank_key.contains("dependency=01"));
        assert!(repair.scheduling_rank_key.contains("urgency=95"));
    }

    #[test]
    fn chronosense_context_upgrades_repeated_commitment_dependency_truth() {
        let mut bundle = parse_economics_bundle_json(FIXTURE).expect("fixture parses");
        bundle.chronosense_context = Some(ChronosenseSchedulerContextV1 {
            schema_version: CHRONOSENSE_SCHEDULER_CONTEXT_SCHEMA_V1.to_string(),
            contract_schema_version: COMMITMENT_DEADLINE_SCHEMA.to_string(),
            generated_from: "chronosense repeated commitment fixture".to_string(),
            signals: vec![
                ChronosenseCommitmentSchedulingSignalV1 {
                    task_id: "first-pass-review".to_string(),
                    commitment_id: "commitment-review-before-closeout".to_string(),
                    status: ChronosenseCommitmentStatusV1::Active,
                    deadline_posture: ChronosenseDeadlinePostureV1::Due,
                    deadline_frame: Some(ChronosenseDeadlineFrameV1::ReviewGate),
                    temporal_urgency: SchedulerUrgencyV1::High,
                    fulfillment_ready: false,
                    review_required: true,
                    reason: Some("review commitment due".to_string()),
                },
                ChronosenseCommitmentSchedulingSignalV1 {
                    task_id: "first-pass-review".to_string(),
                    commitment_id: "commitment-review-before-closeout".to_string(),
                    status: ChronosenseCommitmentStatusV1::Missed,
                    deadline_posture: ChronosenseDeadlinePostureV1::Missed,
                    deadline_frame: Some(ChronosenseDeadlineFrameV1::ReviewGate),
                    temporal_urgency: SchedulerUrgencyV1::Immediate,
                    fulfillment_ready: false,
                    review_required: true,
                    reason: Some("review commitment missed".to_string()),
                },
            ],
        });

        let adjusted = apply_chronosense_scheduler_context(
            &bundle.inputs,
            bundle.chronosense_context.as_ref(),
        );
        let review_input = adjusted
            .iter()
            .find(|input| input.task_id == "first-pass-review")
            .expect("adjusted review input");
        let chronosense_dependency = review_input
            .dependencies
            .iter()
            .find(|dependency| {
                dependency.task_id == "chronosense:commitment-review-before-closeout"
            })
            .expect("chronosense dependency");
        assert_eq!(
            chronosense_dependency.status,
            SchedulerDependencyPostureV1::Blocked
        );
        assert_eq!(
            chronosense_dependency.reason.as_deref(),
            Some("review commitment missed")
        );
        assert_eq!(
            review_input.dependency_posture,
            SchedulerDependencyPostureV1::Blocked
        );
    }

    #[test]
    fn chronosense_context_validates_commitment_contract_surface() {
        let bundle = parse_economics_bundle_json(FIXTURE).expect("fixture parses");
        let context = ChronosenseSchedulerContextV1 {
            schema_version: CHRONOSENSE_SCHEDULER_CONTEXT_SCHEMA_V1.to_string(),
            contract_schema_version: COMMITMENT_DEADLINE_SCHEMA.to_string(),
            generated_from: "chronosense retrieval".to_string(),
            signals: vec![ChronosenseCommitmentSchedulingSignalV1 {
                task_id: "premium-code-repair".to_string(),
                commitment_id: "commitment-repair-before-release".to_string(),
                status: ChronosenseCommitmentStatusV1::Accepted,
                deadline_posture: ChronosenseDeadlinePostureV1::Due,
                deadline_frame: Some(ChronosenseDeadlineFrameV1::ContinuityRelative),
                temporal_urgency: SchedulerUrgencyV1::High,
                fulfillment_ready: false,
                review_required: true,
                reason: None,
            }],
        };

        validate_chronosense_scheduler_context(&context, &bundle.inputs)
            .expect("context references the commitment deadline contract");
        let mut invalid = context.clone();
        invalid.contract_schema_version = "commitment_deadline_semantics.v0".to_string();
        let err = validate_chronosense_scheduler_context(&invalid, &bundle.inputs)
            .expect_err("wrong contract schema must fail");
        assert!(err
            .to_string()
            .contains("chronosense scheduler context must reference commitment contract"));
    }

    #[test]
    fn chronosense_context_rejects_signal_without_deadline_frame() {
        let bundle = parse_economics_bundle_json(FIXTURE).expect("fixture parses");
        let context = ChronosenseSchedulerContextV1 {
            schema_version: CHRONOSENSE_SCHEDULER_CONTEXT_SCHEMA_V1.to_string(),
            contract_schema_version: COMMITMENT_DEADLINE_SCHEMA.to_string(),
            generated_from: "chronosense retrieval".to_string(),
            signals: vec![ChronosenseCommitmentSchedulingSignalV1 {
                task_id: "premium-code-repair".to_string(),
                commitment_id: "commitment-repair-before-release".to_string(),
                status: ChronosenseCommitmentStatusV1::Active,
                deadline_posture: ChronosenseDeadlinePostureV1::Due,
                deadline_frame: None,
                temporal_urgency: SchedulerUrgencyV1::High,
                fulfillment_ready: false,
                review_required: true,
                reason: None,
            }],
        };

        let err = validate_chronosense_scheduler_context(&context, &bundle.inputs)
            .expect_err("deadline frame required");
        assert!(err.to_string().contains("deadline_frame is required"));
    }

    #[test]
    fn cognitive_scheduler_plan_order_is_deterministic_and_explainable() {
        let bundle = parse_economics_bundle_json(FIXTURE).expect("fixture parses");
        let first = schedule_economics_bundle(&bundle).expect("first plan");
        let second = schedule_economics_bundle(&bundle).expect("second plan");
        assert_eq!(first, second);
        assert_eq!(
            first.recommended_order.first().unwrap(),
            "release-authority"
        );
        assert!(
            first
                .recommended_order
                .iter()
                .position(|task| task == "low-urgency-cleanup")
                .expect("delayed low-urgency task in order")
                > first
                    .recommended_order
                    .iter()
                    .position(|task| task == "partial-dependency-review")
                    .expect("schedulable partial dependency task in order")
        );
        assert_eq!(first.recommended_order.last().unwrap(), "blocked-proof");

        let premium = decision(&first, "premium-code-repair");
        assert!(premium.reason.contains("premium cognition"));
        assert!(premium
            .alternatives_considered
            .iter()
            .any(
                |alternative| alternative.lane == CognitiveSchedulerLaneV1::Governor
                    && alternative.disposition == SchedulerAlternativeDispositionV1::Rejected
            ));
        assert!(premium.scheduling_rank_key.contains("gate=01"));
        assert!(premium.scheduling_rank_key.contains("deferred=0"));
    }

    #[test]
    fn cognitive_scheduler_delays_non_immediate_governor_work_when_attention_is_constrained() {
        let bundle = parse_economics_bundle_json(FIXTURE).expect("fixture parses");
        let mut input = bundle
            .inputs
            .iter()
            .find(|input| input.task_id == "release-authority")
            .expect("release fixture")
            .clone();
        input.task_id = "architecture-decision-later".to_string();
        input.task_type = SchedulerTaskTypeV1::Architecture;
        input.risk_level = SchedulerRiskLevelV1::High;
        input.urgency = SchedulerUrgencyV1::High;
        input.human_required = true;

        let decision = schedule_economics_input(&input).expect("decision");
        assert_eq!(decision.selected_lane, CognitiveSchedulerLaneV1::Delayed);
        assert_eq!(
            decision.reason,
            "delayed because governor attention is constrained and the task is not an immediate critical decision"
        );
    }

    #[test]
    fn cognitive_scheduler_rejects_malformed_bundle_before_decision() {
        let mut bundle = parse_economics_bundle_json(FIXTURE).expect("fixture parses");
        bundle.inputs[0].claim_boundary = "exact_cost_claim".to_string();
        let err = schedule_economics_bundle(&bundle).expect_err("invalid claim boundary");
        assert!(err.to_string().contains("bounded or not_exact"));
    }

    #[test]
    fn scheduler_economics_input_parses_yaml() {
        let yaml = r#"
schema_version: adl.scheduler.economics_input.v1
task_id: review-fast-path
task_type: review
estimated_effort: small
estimated_validation_cost: low
estimated_coordination_cost: low
risk_level: medium
expected_value: high
urgency: normal
dependency_posture: clear
parallelism_potential: parallelizable
premium_capacity_pressure: high
governor_attention_pressure: low
confidence: medium
human_required: false
claim_boundary: bounded_v1_inputs_not_exact_measurement
"#;
        let input = parse_economics_input_yaml(yaml).expect("yaml input");
        assert_eq!(input.task_id, "review-fast-path");
        assert_eq!(
            summarize_economics_input(&input)
                .expect("summary")
                .parallelism_score,
            2
        );
    }

    #[test]
    fn scheduler_economics_rejects_unknown_schema() {
        let mut input = parse_economics_bundle_json(FIXTURE)
            .expect("fixture parses")
            .inputs
            .remove(0);
        input.schema_version = "adl.scheduler.economics_input.v0".to_string();

        let err = validate_economics_input(&input).expect_err("schema rejected");
        assert!(err
            .to_string()
            .contains("unsupported scheduler economics input schema"));
    }

    #[test]
    fn scheduler_economics_rejects_blocked_input_without_dependency() {
        let mut input = parse_economics_bundle_json(FIXTURE)
            .expect("fixture parses")
            .inputs
            .remove(0);
        input.dependency_posture = SchedulerDependencyPostureV1::Blocked;
        input.dependencies.clear();

        let err = validate_economics_input(&input).expect_err("dependency rejected");
        assert!(err
            .to_string()
            .contains("must name at least one dependency"));
    }

    #[test]
    fn scheduler_economics_rejects_unbounded_claim_boundary() {
        let mut input = parse_economics_bundle_json(FIXTURE)
            .expect("fixture parses")
            .inputs
            .remove(0);
        input.claim_boundary = "exact_roi_prediction".to_string();

        let err = validate_economics_input(&input).expect_err("claim rejected");
        assert!(err.to_string().contains("bounded or not_exact"));
    }

    fn decision<'a>(
        plan: &'a CognitiveSchedulerPlanV1,
        task_id: &str,
    ) -> &'a CognitiveSchedulerDecisionV1 {
        plan.decisions
            .iter()
            .find(|decision| decision.task_id == task_id)
            .expect("decision exists")
    }

    fn assert_lane(plan: &CognitiveSchedulerPlanV1, task_id: &str, lane: CognitiveSchedulerLaneV1) {
        assert_eq!(decision(plan, task_id).selected_lane, lane);
    }

    fn role_provider_context_fixture() -> RoleProviderSelectionContextV1 {
        RoleProviderSelectionContextV1 {
            schema_version: ROLE_PROVIDER_SELECTION_CONTEXT_SCHEMA_V1.to_string(),
            generated_from: "provider profile selection fixture for #4672".to_string(),
            policies: vec![RoleProviderProfilePolicyV1 {
                role_profile: RoleProviderProfileV1::ReviewerProvider,
                advisory_authority_limit:
                    "advisory review only; cannot merge, close, or override operator gates"
                        .to_string(),
                required_capabilities: vec![
                    "code_review".to_string(),
                    "csdlc_truth_review".to_string(),
                ],
                forbidden_capabilities: vec!["merge_authority".to_string()],
                candidate_routes: vec![
                    RoleProviderCandidateRouteV1 {
                        route: ProviderRouteV1 {
                            provider_profile_ref: Some("ollama:qwen2.5-7b".to_string()),
                            provider_spec_kind: "ollama".to_string(),
                            provider_family: Some("local_ollama".to_string()),
                            model_ref: "qwen2.5:7b".to_string(),
                            model_identity: "qwen2.5:7b local ollama profile".to_string(),
                            runtime_surface: "provider::OllamaProvider".to_string(),
                            provider_selection_reason:
                                "local reviewer candidate retained as fallback evidence"
                                    .to_string(),
                            route_resolution_trace: vec!["candidate=local_ollama".to_string()],
                            output_contract_ref: "adl.provider.output.review_advisory.v1"
                                .to_string(),
                        },
                        eligible: false,
                        ineligibility_reason: Some(
                            "local model not yet proven for reviewer role".to_string(),
                        ),
                    },
                    RoleProviderCandidateRouteV1 {
                        route: ProviderRouteV1 {
                            provider_profile_ref: Some("chatgpt:gpt-5.3-codex".to_string()),
                            provider_spec_kind: "http".to_string(),
                            provider_family: Some("chatgpt".to_string()),
                            model_ref: "gpt-5.3-codex".to_string(),
                            model_identity: "ChatGPT GPT-5.3 Codex profile".to_string(),
                            runtime_surface: "provider::HttpProvider".to_string(),
                            provider_selection_reason:
                                "tracked reviewer-capable provider profile selected for advisory review"
                                    .to_string(),
                            route_resolution_trace: vec!["candidate=chatgpt".to_string()],
                            output_contract_ref: "adl.provider.output.review_advisory.v1"
                                .to_string(),
                        },
                        eligible: true,
                        ineligibility_reason: None,
                    },
                ],
            }],
            assignments: vec![RoleProviderTaskAssignmentV1 {
                task_id: "first-pass-review".to_string(),
                role_profile: RoleProviderProfileV1::ReviewerProvider,
            }],
        }
    }
}
