use anyhow::{anyhow, Result};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub const SCHEDULER_ECONOMICS_INPUT_SCHEMA_V1: &str = "adl.scheduler.economics_input.v1";
pub const SCHEDULER_ECONOMICS_INPUT_BUNDLE_SCHEMA_V1: &str =
    "adl.scheduler.economics_input_bundle.v1";
pub const COGNITIVE_SCHEDULER_DECISION_SCHEMA_V1: &str = "adl.scheduler.decision.v1";
pub const COGNITIVE_SCHEDULER_PLAN_SCHEMA_V1: &str = "adl.scheduler.plan.v1";

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
pub struct CognitiveSchedulerDecisionV1 {
    pub schema_version: String,
    pub task_id: String,
    pub selected_lane: CognitiveSchedulerLaneV1,
    pub alternatives_considered: Vec<SchedulerAlternativeV1>,
    pub reason: String,
    pub score_breakdown: SchedulerScoreBreakdownV1,
    pub dependency_status: SchedulerDependencyPostureV1,
    pub manual_override: SchedulerManualOverrideV1,
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
    if bundle.schema_version != SCHEDULER_ECONOMICS_INPUT_BUNDLE_SCHEMA_V1 {
        return Err(anyhow!(
            "unsupported scheduler economics bundle schema: {}",
            bundle.schema_version
        ));
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
    let mut decisions = bundle
        .inputs
        .iter()
        .map(schedule_economics_input)
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

pub fn schedule_economics_input(
    input: &SchedulerEconomicsInputV1,
) -> Result<CognitiveSchedulerDecisionV1> {
    let summary = summarize_economics_input(input)?;
    let selected_lane = select_lane(input, &summary);
    let reason = decision_reason(input, &summary, &selected_lane);
    let alternatives_considered = alternatives_for(input, &selected_lane);
    let scheduling_rank_key = scheduling_rank_key(input, &summary, &selected_lane);

    Ok(CognitiveSchedulerDecisionV1 {
        schema_version: COGNITIVE_SCHEDULER_DECISION_SCHEMA_V1.to_string(),
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
    _selected_lane: &CognitiveSchedulerLaneV1,
) -> String {
    format!(
        "blocked={};dependency={:02};gate={:02};risk={:02};urgency={:02};value={:02};validation={:02};premium_pressure={:02};coordination={:02};parallelism={:02};confidence={:02};task={}",
        u8::from(summary.blocked),
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
    fn cognitive_scheduler_plan_order_is_deterministic_and_explainable() {
        let bundle = parse_economics_bundle_json(FIXTURE).expect("fixture parses");
        let first = schedule_economics_bundle(&bundle).expect("first plan");
        let second = schedule_economics_bundle(&bundle).expect("second plan");
        assert_eq!(first, second);
        assert_eq!(
            first.recommended_order.first().unwrap(),
            "release-authority"
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
        assert!(!premium.scheduling_rank_key.contains("lane="));
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
}
