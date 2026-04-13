use anyhow::{anyhow, Context, Result};
use chrono::{DateTime, Datelike, FixedOffset, Offset, Timelike, Utc};
use chrono_tz::Tz;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

pub const IDENTITY_PROFILE_SCHEMA: &str = "identity_profile.v1";
pub const TEMPORAL_CONTEXT_SCHEMA: &str = "temporal_context.v1";
pub const CHRONOSENSE_FOUNDATION_SCHEMA: &str = "chronosense_foundation.v1";
pub const TEMPORAL_SCHEMA_V01: &str = "temporal_schema.v0_1";
pub const CONTINUITY_SEMANTICS_SCHEMA: &str = "continuity_semantics.v1";
pub const TEMPORAL_QUERY_RETRIEVAL_SCHEMA: &str = "temporal_query_retrieval.v1";
pub const COMMITMENT_DEADLINE_SCHEMA: &str = "commitment_deadline_semantics.v1";
pub const TEMPORAL_CAUSALITY_EXPLANATION_SCHEMA: &str = "temporal_causality_explanation.v1";
pub const EXECUTION_POLICY_COST_MODEL_SCHEMA: &str = "execution_policy_cost_model.v1";
pub const PHI_INTEGRATION_METRICS_SCHEMA: &str = "phi_integration_metrics.v1";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IdentityProfile {
    pub schema_version: String,
    pub agent_id: String,
    pub display_name: String,
    pub birthday_rfc3339: String,
    pub birth_date_local: String,
    pub birth_weekday_local: String,
    pub birth_timezone: String,
    pub created_by: String,
    pub continuity_mode: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TemporalContext {
    pub schema_version: String,
    pub captured_at_rfc3339: String,
    pub local_timestamp_rfc3339: String,
    pub local_date: String,
    pub local_time: String,
    pub local_weekday: String,
    pub timezone: String,
    pub utc_offset: String,
    pub identity_agent_id: Option<String>,
    pub identity_display_name: Option<String>,
    pub age_days_since_birthday: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ChronosenseFoundation {
    pub schema_version: String,
    pub owned_runtime_surfaces: Vec<String>,
    pub required_capabilities: Vec<String>,
    pub proof_hook_command: String,
    pub proof_hook_output_path: String,
    pub scope_boundary: String,
}

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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContinuityStateContract {
    pub resilience_classification: Vec<String>,
    pub continuity_status: Vec<String>,
    pub preservation_status: Vec<String>,
    pub shepherd_decision: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ResumptionRule {
    pub continuity_status: String,
    pub resume_permitted: bool,
    pub identity_preserved: bool,
    pub required_guard: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContinuitySemanticsContract {
    pub schema_version: String,
    pub owned_runtime_surfaces: Vec<String>,
    pub continuity_state_contract: ContinuityStateContract,
    pub resumption_rules: Vec<ResumptionRule>,
    pub proof_fixture_hooks: Vec<String>,
    pub proof_hook_command: String,
    pub proof_hook_output_path: String,
    pub scope_boundary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TemporalQueryPrimitiveSet {
    pub relative_order_queries: Vec<String>,
    pub interval_queries: Vec<String>,
    pub staleness_queries: Vec<String>,
    pub continuity_queries: Vec<String>,
    pub commitment_state_queries: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TemporalRetrievalSemantics {
    pub temporal_anchors: Vec<String>,
    pub multiple_time_views: Vec<String>,
    pub staleness_factors: Vec<String>,
    pub continuity_inputs: Vec<String>,
    pub index_expectations: Vec<String>,
    pub deterministic_ordering: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TemporalQueryRetrievalContract {
    pub schema_version: String,
    pub owned_runtime_surfaces: Vec<String>,
    pub query_primitives: TemporalQueryPrimitiveSet,
    pub retrieval_semantics: TemporalRetrievalSemantics,
    pub proof_fixture_hooks: Vec<String>,
    pub proof_hook_command: String,
    pub proof_hook_output_path: String,
    pub scope_boundary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CommitmentLifecycleContract {
    pub states: Vec<String>,
    pub open_states: Vec<String>,
    pub terminal_states: Vec<String>,
    pub state_distinctions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CommitmentRecordRequirements {
    pub required_fields: Vec<String>,
    pub history_requirements: Vec<String>,
    pub persistence_expectations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeadlineSemanticsContract {
    pub supported_frames: Vec<String>,
    pub frame_rule: String,
    pub explicitness_requirements: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MissedCommitmentDetectionContract {
    pub detection_conditions: Vec<String>,
    pub visibility_requirements: Vec<String>,
    pub retrieval_surfaces: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CommitmentDeadlineContract {
    pub schema_version: String,
    pub owned_runtime_surfaces: Vec<String>,
    pub lifecycle: CommitmentLifecycleContract,
    pub record_requirements: CommitmentRecordRequirements,
    pub deadline_semantics: DeadlineSemanticsContract,
    pub missed_commitment_detection: MissedCommitmentDetectionContract,
    pub proof_fixture_hooks: Vec<String>,
    pub proof_hook_command: String,
    pub proof_hook_output_path: String,
    pub scope_boundary: String,
}

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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PhiMetricDimension {
    pub name: String,
    pub interpretation: String,
    pub low_signal: String,
    pub medium_signal: String,
    pub high_signal: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PhiComparisonProfile {
    pub profile_name: String,
    pub integration_band: String,
    pub expected_runtime_surfaces: Vec<String>,
    pub why_it_matters: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PhiComparisonFixture {
    pub profile_name: String,
    pub structural_coupling: String,
    pub memory_coupling: String,
    pub feedback_depth: String,
    pub policy_continuity: String,
    pub instinct_coupling: String,
    pub graph_irreducibility: String,
    pub adaptive_depth: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PhiReviewSurfaceContract {
    pub required_questions: Vec<String>,
    pub comparison_rule: String,
    pub non_goals: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PhiIntegrationMetricsContract {
    pub schema_version: String,
    pub owned_runtime_surfaces: Vec<String>,
    pub dimensions: Vec<PhiMetricDimension>,
    pub comparison_profiles: Vec<PhiComparisonProfile>,
    pub comparison_fixtures: Vec<PhiComparisonFixture>,
    pub review_surface: PhiReviewSurfaceContract,
    pub proof_fixture_hooks: Vec<String>,
    pub proof_hook_command: String,
    pub proof_hook_output_path: String,
    pub scope_boundary: String,
}

impl IdentityProfile {
    pub fn from_birthday(
        agent_id: impl Into<String>,
        display_name: impl Into<String>,
        birthday_rfc3339: &str,
        timezone_name: &str,
        created_by: impl Into<String>,
    ) -> Result<Self> {
        let birthday = parse_rfc3339(birthday_rfc3339)?;
        let timezone = parse_timezone(timezone_name)?;
        let local_birthday = birthday.with_timezone(&timezone);

        Ok(Self {
            schema_version: IDENTITY_PROFILE_SCHEMA.to_string(),
            agent_id: normalize_nonempty(agent_id.into(), "agent_id")?,
            display_name: normalize_nonempty(display_name.into(), "display_name")?,
            birthday_rfc3339: birthday.to_rfc3339(),
            birth_date_local: local_birthday.format("%Y-%m-%d").to_string(),
            birth_weekday_local: local_birthday.format("%A").to_string(),
            birth_timezone: timezone.name().to_string(),
            created_by: normalize_nonempty(created_by.into(), "created_by")?,
            continuity_mode: "repo_local_persistent".to_string(),
        })
    }
}

impl TemporalContext {
    pub fn from_now(
        now_utc: DateTime<Utc>,
        timezone_name: &str,
        identity: Option<&IdentityProfile>,
    ) -> Result<Self> {
        let timezone = parse_timezone(timezone_name)?;
        let local_now = now_utc.with_timezone(&timezone);
        let offset = local_now.offset().fix();
        let age_days_since_birthday = match identity {
            Some(profile) => {
                let birthday = parse_rfc3339(&profile.birthday_rfc3339)?;
                let birthday_local = birthday.with_timezone(&timezone);
                Some(
                    local_now
                        .date_naive()
                        .signed_duration_since(birthday_local.date_naive())
                        .num_days(),
                )
            }
            None => None,
        };

        Ok(Self {
            schema_version: TEMPORAL_CONTEXT_SCHEMA.to_string(),
            captured_at_rfc3339: now_utc.to_rfc3339(),
            local_timestamp_rfc3339: local_now.to_rfc3339(),
            local_date: format!(
                "{:04}-{:02}-{:02}",
                local_now.year(),
                local_now.month(),
                local_now.day()
            ),
            local_time: format!(
                "{:02}:{:02}:{:02}",
                local_now.hour(),
                local_now.minute(),
                local_now.second()
            ),
            local_weekday: local_now.format("%A").to_string(),
            timezone: timezone.name().to_string(),
            utc_offset: format_offset(offset),
            identity_agent_id: identity.map(|profile| profile.agent_id.clone()),
            identity_display_name: identity.map(|profile| profile.display_name.clone()),
            age_days_since_birthday,
        })
    }
}

impl ChronosenseFoundation {
    pub fn bounded_v088() -> Self {
        Self {
            schema_version: CHRONOSENSE_FOUNDATION_SCHEMA.to_string(),
            owned_runtime_surfaces: vec![
                "adl::chronosense::IdentityProfile".to_string(),
                "adl::chronosense::TemporalContext".to_string(),
                "adl identity init".to_string(),
                "adl identity now".to_string(),
                "adl identity foundation".to_string(),
            ],
            required_capabilities: vec![
                "now_sense".to_string(),
                "sequence_sense".to_string(),
                "duration_sense".to_string(),
                "lifetime_sense".to_string(),
            ],
            proof_hook_command:
                "adl identity foundation --out .adl/state/chronosense_foundation.v1.json"
                    .to_string(),
            proof_hook_output_path: ".adl/state/chronosense_foundation.v1.json".to_string(),
            scope_boundary:
                "bounded chronosense substrate only; continuity semantics, temporal schema, commitments, retrieval, and causality remain downstream work"
                    .to_string(),
        }
    }
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

impl ContinuitySemanticsContract {
    pub fn v1() -> Self {
        Self {
            schema_version: CONTINUITY_SEMANTICS_SCHEMA.to_string(),
            owned_runtime_surfaces: vec![
                "adl::chronosense::ContinuitySemanticsContract".to_string(),
                "adl::cli::run_artifacts::summary::derive_resilience_status".to_string(),
                "run_status.v1.continuity_status".to_string(),
                "run_status.v1.preservation_status".to_string(),
                "run_status.v1.shepherd_decision".to_string(),
                "adl identity continuity".to_string(),
            ],
            continuity_state_contract: ContinuityStateContract {
                resilience_classification: vec![
                    "interruption".to_string(),
                    "crash".to_string(),
                    "corruption".to_string(),
                    "not_applicable".to_string(),
                ],
                continuity_status: vec![
                    "resume_ready".to_string(),
                    "continuity_unverified".to_string(),
                    "continuity_refused".to_string(),
                    "continuous".to_string(),
                ],
                preservation_status: vec![
                    "pause_state_preserved".to_string(),
                    "preserved_for_review".to_string(),
                    "inspection_only".to_string(),
                    "no_preservation_needed".to_string(),
                ],
                shepherd_decision: vec![
                    "preserve_and_resume".to_string(),
                    "operator_review_required".to_string(),
                    "refuse_resume".to_string(),
                    "none".to_string(),
                ],
            },
            resumption_rules: vec![
                ResumptionRule {
                    continuity_status: "resume_ready".to_string(),
                    resume_permitted: true,
                    identity_preserved: true,
                    required_guard: "execution_plan_hash_match_required".to_string(),
                },
                ResumptionRule {
                    continuity_status: "continuity_unverified".to_string(),
                    resume_permitted: false,
                    identity_preserved: false,
                    required_guard: "operator_review_required".to_string(),
                },
                ResumptionRule {
                    continuity_status: "continuity_refused".to_string(),
                    resume_permitted: false,
                    identity_preserved: false,
                    required_guard: "resume_not_permitted".to_string(),
                },
                ResumptionRule {
                    continuity_status: "continuous".to_string(),
                    resume_permitted: false,
                    identity_preserved: true,
                    required_guard: "not_applicable".to_string(),
                },
            ],
            proof_fixture_hooks: vec![
                "build_run_status_marks_paused_runs_as_resumable_interruption".to_string(),
                "build_run_status_refuses_resume_for_replay_invariant_corruption".to_string(),
            ],
            proof_hook_command:
                "adl identity continuity --out .adl/state/continuity_semantics_v1.json"
                    .to_string(),
            proof_hook_output_path: ".adl/state/continuity_semantics_v1.json".to_string(),
            scope_boundary:
                "continuity and identity semantics only; retrieval, commitments, causality, and governance semantics remain downstream work"
                    .to_string(),
        }
    }
}

impl TemporalQueryRetrievalContract {
    pub fn v1() -> Self {
        Self {
            schema_version: TEMPORAL_QUERY_RETRIEVAL_SCHEMA.to_string(),
            owned_runtime_surfaces: vec![
                "adl::chronosense::TemporalQueryRetrievalContract".to_string(),
                "adl::chronosense::TemporalQueryPrimitiveSet".to_string(),
                "adl::execute::state::runtime_control::MemoryQueryState".to_string(),
                "adl::obsmem_contract::MemoryQuery".to_string(),
                "adl::obsmem_retrieval_policy::RetrievalPolicyV1".to_string(),
                "adl identity retrieval".to_string(),
            ],
            query_primitives: TemporalQueryPrimitiveSet {
                relative_order_queries: vec![
                    "before focal event".to_string(),
                    "after focal event".to_string(),
                    "nearest prior relevant record".to_string(),
                ],
                interval_queries: vec![
                    "between T1 and T2".to_string(),
                    "during run window".to_string(),
                    "neighboring records around focal event".to_string(),
                ],
                staleness_queries: vec![
                    "stale beyond decision horizon".to_string(),
                    "older than last confirmation".to_string(),
                    "downweight due to age or inactivity".to_string(),
                ],
                continuity_queries: vec![
                    "last valid continuity boundary".to_string(),
                    "interruption boundaries".to_string(),
                    "state transitions that threaten continuity".to_string(),
                ],
                commitment_state_queries: vec![
                    "open commitments".to_string(),
                    "approaching deadlines".to_string(),
                    "missed commitments in interval".to_string(),
                ],
            },
            retrieval_semantics: TemporalRetrievalSemantics {
                temporal_anchors: vec![
                    "t_created".to_string(),
                    "t_observed".to_string(),
                    "t_effective".to_string(),
                    "monotonic event order".to_string(),
                    "run-local sequence order".to_string(),
                    "continuity-chain identifiers".to_string(),
                ],
                multiple_time_views: vec![
                    "wall_clock".to_string(),
                    "event_order".to_string(),
                    "continuity_order".to_string(),
                ],
                staleness_factors: vec![
                    "age".to_string(),
                    "task_context".to_string(),
                    "change_rate".to_string(),
                    "commitment_or_invariant_durability".to_string(),
                ],
                continuity_inputs: vec![
                    "run_status.v1.continuity_status".to_string(),
                    "run_status.v1.preservation_status".to_string(),
                    "run_status.v1.shepherd_decision".to_string(),
                ],
                index_expectations: vec![
                    "lookup by time anchor".to_string(),
                    "lookup by interval".to_string(),
                    "ordering by monotonic sequence".to_string(),
                    "filtering by continuity-relevant boundaries".to_string(),
                    "neighbor retrieval around focal event".to_string(),
                ],
                deterministic_ordering: vec![
                    "workflow_id_then_run_id_ascending".to_string(),
                    "score_desc_id_asc".to_string(),
                    "evidence_adjusted_desc_id_asc".to_string(),
                    "id_asc".to_string(),
                ],
            },
            proof_fixture_hooks: vec![
                "obsmem_retrieval_policy::apply_policy_filters_and_orders_deterministically"
                    .to_string(),
                "obsmem_validation_tests::retrieval_determinism_returns_identical_result_set_and_order"
                    .to_string(),
                "build_memory_artifacts_are_deterministic_and_preserve_read_write_semantics"
                    .to_string(),
            ],
            proof_hook_command:
                "adl identity retrieval --out .adl/state/temporal_query_retrieval_v1.json"
                    .to_string(),
            proof_hook_output_path: ".adl/state/temporal_query_retrieval_v1.json".to_string(),
            scope_boundary:
                "temporal query/retrieval semantics only; full temporal indexing, causality, and distributed temporal truth remain downstream work"
                    .to_string(),
        }
    }
}

impl CommitmentDeadlineContract {
    pub fn v1() -> Self {
        Self {
            schema_version: COMMITMENT_DEADLINE_SCHEMA.to_string(),
            owned_runtime_surfaces: vec![
                "adl::chronosense::CommitmentDeadlineContract".to_string(),
                "adl::chronosense::CommitmentLifecycleContract".to_string(),
                "adl::chronosense::DeadlineSemanticsContract".to_string(),
                "adl::chronosense::MissedCommitmentDetectionContract".to_string(),
                "adl::chronosense::TemporalQueryRetrievalContract".to_string(),
                "adl identity commitments".to_string(),
            ],
            lifecycle: CommitmentLifecycleContract {
                states: vec![
                    "proposed".to_string(),
                    "accepted".to_string(),
                    "active".to_string(),
                    "fulfilled".to_string(),
                    "deferred".to_string(),
                    "canceled".to_string(),
                    "expired".to_string(),
                    "missed".to_string(),
                ],
                open_states: vec![
                    "accepted".to_string(),
                    "active".to_string(),
                    "deferred".to_string(),
                ],
                terminal_states: vec![
                    "fulfilled".to_string(),
                    "canceled".to_string(),
                    "expired".to_string(),
                    "missed".to_string(),
                ],
                state_distinctions: vec![
                    "accepted_and_open_vs_not_yet_accepted".to_string(),
                    "deferred_vs_silent_neglect".to_string(),
                    "canceled_vs_fulfilled".to_string(),
                    "expired_vs_missed".to_string(),
                ],
            },
            record_requirements: CommitmentRecordRequirements {
                required_fields: vec![
                    "obligation_or_intended_action".to_string(),
                    "accepted_by".to_string(),
                    "applicable_office_or_authority".to_string(),
                    "created_at".to_string(),
                    "deadline_or_review_window".to_string(),
                    "current_status".to_string(),
                    "fulfillment_conditions".to_string(),
                ],
                history_requirements: vec![
                    "what_changed".to_string(),
                    "when_it_changed".to_string(),
                    "why_it_changed".to_string(),
                ],
                persistence_expectations: vec![
                    "remain_queryable_across_runs".to_string(),
                    "survive_bounded_interruption".to_string(),
                    "support_honest_resume_or_cancellation".to_string(),
                ],
            },
            deadline_semantics: DeadlineSemanticsContract {
                supported_frames: vec![
                    "wall_clock".to_string(),
                    "event_count".to_string(),
                    "review_gate".to_string(),
                    "continuity_relative".to_string(),
                ],
                frame_rule: "a deadline is meaningful only relative to an explicit temporal frame"
                    .to_string(),
                explicitness_requirements: vec![
                    "deadline_frame_must_be_named".to_string(),
                    "review_window_must_be_recorded_when_used".to_string(),
                    "no_implicit_single_clock_assumption".to_string(),
                ],
            },
            missed_commitment_detection: MissedCommitmentDetectionContract {
                detection_conditions: vec![
                    "overdue_active_commitment".to_string(),
                    "fulfillment_conditions_not_met_before_deadline".to_string(),
                    "commitment_invalidated_by_interruption_or_context_change".to_string(),
                ],
                visibility_requirements: vec![
                    "missed_commitments_remain_visible_for_review".to_string(),
                    "missed_commitments_remain_visible_for_accountability".to_string(),
                    "missed_commitments_remain_visible_for_planning_correction".to_string(),
                ],
                retrieval_surfaces: vec![
                    "open commitments".to_string(),
                    "approaching deadlines".to_string(),
                    "missed commitments in interval".to_string(),
                ],
            },
            proof_fixture_hooks: vec![
                "adl::chronosense::CommitmentDeadlineContract::v1".to_string(),
                "adl identity commitments --out .adl/state/commitment_deadline_semantics_v1.json"
                    .to_string(),
            ],
            proof_hook_command:
                "adl identity commitments --out .adl/state/commitment_deadline_semantics_v1.json"
                    .to_string(),
            proof_hook_output_path: ".adl/state/commitment_deadline_semantics_v1.json"
                .to_string(),
            scope_boundary:
                "commitment and deadline semantics only; scheduling automation, negotiation, and calendar integration remain downstream work"
                    .to_string(),
        }
    }
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

impl PhiIntegrationMetricsContract {
    pub fn v1() -> Self {
        Self {
            schema_version: PHI_INTEGRATION_METRICS_SCHEMA.to_string(),
            owned_runtime_surfaces: vec![
                "adl::chronosense::PhiIntegrationMetricsContract".to_string(),
                "adl::chronosense::PhiMetricDimension".to_string(),
                "adl::chronosense::PhiComparisonProfile".to_string(),
                "adl::chronosense::PhiComparisonFixture".to_string(),
                "adl::chronosense::TemporalQueryRetrievalContract".to_string(),
                "adl::chronosense::CommitmentDeadlineContract".to_string(),
                "adl::chronosense::ExecutionPolicyCostModelContract".to_string(),
                "adl identity phi".to_string(),
            ],
            dimensions: vec![
                PhiMetricDimension {
                    name: "structural_coupling".to_string(),
                    interpretation:
                        "how many runtime surfaces must remain coordinated for the path to work"
                            .to_string(),
                    low_signal: "mostly isolated steps with minimal cross-surface dependency"
                        .to_string(),
                    medium_signal:
                        "some shared state or review surfaces must remain aligned".to_string(),
                    high_signal:
                        "behavior depends on multiple tightly coupled runtime surfaces".to_string(),
                },
                PhiMetricDimension {
                    name: "memory_coupling".to_string(),
                    interpretation:
                        "how much the path relies on retrieval state or continuity-preserved records"
                            .to_string(),
                    low_signal: "little or no retrieval dependence".to_string(),
                    medium_signal: "retrieval helps but is not the sole determinant".to_string(),
                    high_signal:
                        "retrieval or preserved state materially shapes behavior".to_string(),
                },
                PhiMetricDimension {
                    name: "feedback_depth".to_string(),
                    interpretation:
                        "how much iterative review, adaptation, or reflexive correction is required"
                            .to_string(),
                    low_signal: "single-pass execution with little feedback".to_string(),
                    medium_signal: "bounded refinement or review loops".to_string(),
                    high_signal: "multi-step feedback materially changes the resulting path"
                        .to_string(),
                },
                PhiMetricDimension {
                    name: "policy_continuity".to_string(),
                    interpretation:
                        "how much stable policy and execution posture must persist across the path"
                            .to_string(),
                    low_signal: "policy can vary without changing the main outcome".to_string(),
                    medium_signal: "policy consistency improves comparability and trust"
                        .to_string(),
                    high_signal:
                        "policy continuity is necessary for meaningful comparison or replay"
                            .to_string(),
                },
                PhiMetricDimension {
                    name: "instinct_coupling".to_string(),
                    interpretation:
                        "how much instinct or bounded-priority posture must couple to execution"
                            .to_string(),
                    low_signal: "no instinct-sensitive routing involved".to_string(),
                    medium_signal:
                        "priority posture influences comparison but is still bounded".to_string(),
                    high_signal:
                        "instinct-sensitive posture materially changes execution shape"
                            .to_string(),
                },
                PhiMetricDimension {
                    name: "graph_irreducibility".to_string(),
                    interpretation:
                        "how much explanatory power is lost if the path is split into independent parts"
                            .to_string(),
                    low_signal: "the path stays understandable when decomposed".to_string(),
                    medium_signal:
                        "some explanatory loss appears when the path is decomposed".to_string(),
                    high_signal:
                        "decomposition hides important cross-surface behavior".to_string(),
                },
                PhiMetricDimension {
                    name: "adaptive_depth".to_string(),
                    interpretation:
                        "how much bounded adaptation or runtime re-weighting is present"
                            .to_string(),
                    low_signal: "fixed execution with no meaningful adaptation".to_string(),
                    medium_signal: "bounded adaptation within an explicit review surface"
                        .to_string(),
                    high_signal:
                        "adaptation materially changes path selection or integration profile"
                            .to_string(),
                },
            ],
            comparison_profiles: vec![
                PhiComparisonProfile {
                    profile_name: "low_integration_path".to_string(),
                    integration_band: "low".to_string(),
                    expected_runtime_surfaces: vec![
                        "adl identity foundation".to_string(),
                        "adl identity schema".to_string(),
                    ],
                    why_it_matters:
                        "establishes the baseline for bounded, mostly decomposable execution"
                            .to_string(),
                },
                PhiComparisonProfile {
                    profile_name: "medium_integration_path".to_string(),
                    integration_band: "medium".to_string(),
                    expected_runtime_surfaces: vec![
                        "adl identity retrieval".to_string(),
                        "adl identity commitments".to_string(),
                        "adl identity cost".to_string(),
                    ],
                    why_it_matters:
                        "shows when memory, commitment, and cost surfaces begin to couple"
                            .to_string(),
                },
                PhiComparisonProfile {
                    profile_name: "high_integration_path".to_string(),
                    integration_band: "high".to_string(),
                    expected_runtime_surfaces: vec![
                        "adl identity continuity".to_string(),
                        "adl identity causality".to_string(),
                        "instinct runtime surface".to_string(),
                    ],
                    why_it_matters:
                        "gives reviewers a bounded model for tightly coupled adaptive runtime behavior"
                            .to_string(),
                },
            ],
            comparison_fixtures: vec![
                PhiComparisonFixture {
                    profile_name: "low_integration_path".to_string(),
                    structural_coupling: "low".to_string(),
                    memory_coupling: "low".to_string(),
                    feedback_depth: "low".to_string(),
                    policy_continuity: "low".to_string(),
                    instinct_coupling: "low".to_string(),
                    graph_irreducibility: "low".to_string(),
                    adaptive_depth: "low".to_string(),
                },
                PhiComparisonFixture {
                    profile_name: "medium_integration_path".to_string(),
                    structural_coupling: "medium".to_string(),
                    memory_coupling: "medium".to_string(),
                    feedback_depth: "medium".to_string(),
                    policy_continuity: "medium".to_string(),
                    instinct_coupling: "low".to_string(),
                    graph_irreducibility: "medium".to_string(),
                    adaptive_depth: "medium".to_string(),
                },
                PhiComparisonFixture {
                    profile_name: "high_integration_path".to_string(),
                    structural_coupling: "high".to_string(),
                    memory_coupling: "high".to_string(),
                    feedback_depth: "high".to_string(),
                    policy_continuity: "high".to_string(),
                    instinct_coupling: "medium".to_string(),
                    graph_irreducibility: "high".to_string(),
                    adaptive_depth: "high".to_string(),
                },
            ],
            review_surface: PhiReviewSurfaceContract {
                required_questions: vec![
                    "which integration dimensions changed across low, medium, and high paths"
                        .to_string(),
                    "which runtime surfaces explain those differences".to_string(),
                    "why the comparison matters for ADL execution behavior".to_string(),
                ],
                comparison_rule:
                    "reviewers must be able to compare low, medium, and high integration profiles without collapsing them into a single metaphysical scalar"
                        .to_string(),
                non_goals: vec![
                    "formal IIT phi calculation".to_string(),
                    "consciousness or sentience claims".to_string(),
                    "one-number replacement for cost, time, or routing".to_string(),
                ],
            },
            proof_fixture_hooks: vec![
                "adl::chronosense::PhiIntegrationMetricsContract::v1".to_string(),
                "adl identity phi --out .adl/state/phi_integration_metrics_v1.json"
                    .to_string(),
            ],
            proof_hook_command:
                "adl identity phi --out .adl/state/phi_integration_metrics_v1.json"
                    .to_string(),
            proof_hook_output_path: ".adl/state/phi_integration_metrics_v1.json".to_string(),
            scope_boundary:
                "bounded engineering comparison surface only; no formal IIT, no consciousness claims, and no replacement of cost or temporal review surfaces"
                    .to_string(),
        }
    }
}

pub fn default_identity_profile_path(repo_root: &Path) -> PathBuf {
    repo_root
        .join("adl")
        .join("identity")
        .join("identity_profile.v1.json")
}

pub fn write_identity_profile(path: &Path, profile: &IdentityProfile) -> Result<()> {
    let Some(parent) = path.parent() else {
        return Err(anyhow!(
            "identity profile path must have a parent directory"
        ));
    };
    fs::create_dir_all(parent).with_context(|| {
        format!(
            "failed to create identity profile parent directory {}",
            parent.display()
        )
    })?;
    let bytes = serde_json::to_vec_pretty(profile)?;
    fs::write(path, bytes)
        .with_context(|| format!("failed to write identity profile to {}", path.display()))
}

pub fn load_identity_profile(path: &Path) -> Result<IdentityProfile> {
    let bytes = fs::read(path)
        .with_context(|| format!("failed to read identity profile from {}", path.display()))?;
    let profile: IdentityProfile = serde_json::from_slice(&bytes)
        .with_context(|| format!("failed to parse identity profile {}", path.display()))?;
    if profile.schema_version != IDENTITY_PROFILE_SCHEMA {
        return Err(anyhow!(
            "unsupported identity profile schema version '{}'",
            profile.schema_version
        ));
    }
    Ok(profile)
}

fn parse_rfc3339(value: &str) -> Result<DateTime<FixedOffset>> {
    DateTime::parse_from_rfc3339(value)
        .with_context(|| format!("invalid RFC3339 datetime '{}'", value))
}

fn parse_timezone(value: &str) -> Result<Tz> {
    value
        .parse::<Tz>()
        .with_context(|| format!("unsupported timezone '{}'", value))
}

fn normalize_nonempty(value: String, field: &str) -> Result<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(anyhow!("{field} must not be empty"));
    }
    Ok(trimmed.to_string())
}

fn format_offset(offset: FixedOffset) -> String {
    let total = offset.local_minus_utc();
    let sign = if total >= 0 { '+' } else { '-' };
    let absolute = total.abs();
    let hours = absolute / 3600;
    let minutes = (absolute % 3600) / 60;
    format!("{sign}{hours:02}:{minutes:02}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn identity_profile_derives_local_birth_fields() {
        let profile = IdentityProfile::from_birthday(
            "codex",
            "Codex",
            "2026-03-30T13:34:00-07:00",
            "America/Los_Angeles",
            "daniel",
        )
        .expect("profile");

        assert_eq!(profile.schema_version, IDENTITY_PROFILE_SCHEMA);
        assert_eq!(profile.birth_date_local, "2026-03-30");
        assert_eq!(profile.birth_weekday_local, "Monday");
        assert_eq!(profile.birth_timezone, "America/Los_Angeles");
        assert_eq!(profile.continuity_mode, "repo_local_persistent");
    }

    #[test]
    fn temporal_context_includes_identity_and_age() {
        let profile = IdentityProfile::from_birthday(
            "codex",
            "Codex",
            "2026-03-30T13:34:00-07:00",
            "America/Los_Angeles",
            "daniel",
        )
        .expect("profile");
        let now_utc = Utc.with_ymd_and_hms(2026, 3, 31, 20, 0, 0).unwrap();

        let context = TemporalContext::from_now(now_utc, "America/Los_Angeles", Some(&profile))
            .expect("context");

        assert_eq!(context.schema_version, TEMPORAL_CONTEXT_SCHEMA);
        assert_eq!(context.local_date, "2026-03-31");
        assert_eq!(context.local_weekday, "Tuesday");
        assert_eq!(context.utc_offset, "-07:00");
        assert_eq!(context.identity_agent_id.as_deref(), Some("codex"));
        assert_eq!(context.age_days_since_birthday, Some(1));
    }

    #[test]
    fn default_identity_profile_path_is_repo_relative() {
        let path = default_identity_profile_path(Path::new("/repo"));
        assert_eq!(
            path,
            PathBuf::from("/repo/adl/identity/identity_profile.v1.json")
        );
    }

    #[test]
    fn chronosense_foundation_is_bounded_and_reviewable() {
        let foundation = ChronosenseFoundation::bounded_v088();

        assert_eq!(foundation.schema_version, CHRONOSENSE_FOUNDATION_SCHEMA);
        assert!(foundation
            .owned_runtime_surfaces
            .contains(&"adl identity foundation".to_string()));
        assert_eq!(
            foundation.required_capabilities,
            vec![
                "now_sense".to_string(),
                "sequence_sense".to_string(),
                "duration_sense".to_string(),
                "lifetime_sense".to_string(),
            ]
        );
        assert!(foundation
            .proof_hook_command
            .contains("chronosense_foundation.v1.json"));
        assert!(foundation
            .scope_boundary
            .contains("bounded chronosense substrate"));
    }

    #[test]
    fn temporal_schema_contract_is_reviewable_and_trace_linked() {
        let schema = TemporalSchemaContract::v01();

        assert_eq!(schema.schema_version, TEMPORAL_SCHEMA_V01);
        assert!(schema
            .owned_runtime_surfaces
            .contains(&"adl identity schema".to_string()));
        assert_eq!(
            schema.primary_temporal_anchor.temporal_confidence,
            "required one of high|medium|low"
        );
        assert!(schema
            .execution_policy_trace_hooks
            .contains(&"run_state.v1.duration_ms".to_string()));
        assert!(schema
            .proof_hook_output_path
            .contains("temporal_schema_v01.json"));
    }

    #[test]
    fn continuity_semantics_contract_matches_runtime_status_surface() {
        let contract = ContinuitySemanticsContract::v1();

        assert_eq!(contract.schema_version, CONTINUITY_SEMANTICS_SCHEMA);
        assert!(contract
            .owned_runtime_surfaces
            .contains(&"adl identity continuity".to_string()));
        assert!(contract
            .continuity_state_contract
            .continuity_status
            .contains(&"resume_ready".to_string()));
        assert!(contract
            .resumption_rules
            .iter()
            .any(|rule| rule.continuity_status == "continuity_refused" && !rule.resume_permitted));
        assert!(contract
            .proof_hook_output_path
            .contains("continuity_semantics_v1.json"));
    }

    #[test]
    fn temporal_query_retrieval_contract_matches_runtime_and_retrieval_surfaces() {
        let contract = TemporalQueryRetrievalContract::v1();

        assert_eq!(contract.schema_version, TEMPORAL_QUERY_RETRIEVAL_SCHEMA);
        assert!(contract
            .owned_runtime_surfaces
            .contains(&"adl identity retrieval".to_string()));
        assert!(contract
            .owned_runtime_surfaces
            .contains(&"adl::obsmem_contract::MemoryQuery".to_string()));
        assert!(contract
            .query_primitives
            .staleness_queries
            .contains(&"stale beyond decision horizon".to_string()));
        assert!(contract
            .retrieval_semantics
            .continuity_inputs
            .contains(&"run_status.v1.continuity_status".to_string()));
        assert!(contract
            .retrieval_semantics
            .deterministic_ordering
            .contains(&"score_desc_id_asc".to_string()));
        assert!(contract
            .proof_hook_output_path
            .contains("temporal_query_retrieval_v1.json"));
    }

    #[test]
    fn commitment_deadline_contract_matches_runtime_and_review_surfaces() {
        let contract = CommitmentDeadlineContract::v1();

        assert_eq!(contract.schema_version, COMMITMENT_DEADLINE_SCHEMA);
        assert!(contract
            .owned_runtime_surfaces
            .contains(&"adl identity commitments".to_string()));
        assert!(contract.lifecycle.states.contains(&"missed".to_string()));
        assert!(contract
            .deadline_semantics
            .supported_frames
            .contains(&"continuity_relative".to_string()));
        assert!(contract
            .missed_commitment_detection
            .retrieval_surfaces
            .contains(&"approaching deadlines".to_string()));
        assert!(contract
            .proof_hook_output_path
            .contains("commitment_deadline_semantics_v1.json"));
    }

    #[test]
    fn temporal_causality_explanation_contract_distinguishes_sequence_from_causality() {
        let contract = TemporalCausalityExplanationContract::v1();

        assert_eq!(
            contract.schema_version,
            TEMPORAL_CAUSALITY_EXPLANATION_SCHEMA
        );
        assert_eq!(
            contract.causal_relations.sequence_boundary_rule,
            "sequence alone is insufficient evidence for causality"
        );
        assert!(contract
            .causal_relations
            .relation_types
            .contains(&"unknown_relation".to_string()));
        assert!(contract
            .owned_runtime_surfaces
            .contains(&"adl identity causality".to_string()));
    }

    #[test]
    fn temporal_causality_explanation_contract_has_bounded_proof_hook_and_uncertainty() {
        let contract = TemporalCausalityExplanationContract::v1();

        assert!(contract
            .explanation_surface
            .required_fields
            .contains(&"relation_type".to_string()));
        assert!(contract
            .explanation_surface
            .citation_requirements
            .iter()
            .any(|value| value.contains("uncertainty")));
        assert!(contract
            .proof_hook_command
            .contains("adl identity causality"));
        assert!(contract
            .proof_hook_output_path
            .contains("temporal_causality_explanation_v1.json"));
    }

    #[test]
    fn execution_policy_cost_model_contract_is_trace_anchored_and_reviewable() {
        let contract = ExecutionPolicyCostModelContract::v1();

        assert_eq!(contract.schema_version, EXECUTION_POLICY_COST_MODEL_SCHEMA);
        assert!(contract
            .owned_runtime_surfaces
            .contains(&"adl identity cost".to_string()));
        assert_eq!(
            contract.review_surface.comparison_rule,
            "reviewers must be able to compare requested execution posture against realized cost and execution behavior"
        );
        assert!(contract
            .review_surface
            .required_trace_hooks
            .contains(&"run_state.v1.duration_ms".to_string()));
    }

    #[test]
    fn execution_policy_cost_model_contract_exposes_policy_and_cost_bounds() {
        let contract = ExecutionPolicyCostModelContract::v1();

        assert_eq!(
            contract.cost_policy.priority,
            "required one of cost|latency|quality"
        );
        assert_eq!(
            contract.cost_anchor.trace_event_id,
            "required canonical trace event id"
        );
        assert!(contract.proof_hook_command.contains("adl identity cost"));
        assert!(contract
            .proof_hook_output_path
            .contains("execution_policy_cost_model_v1.json"));
    }

    #[test]
    fn phi_integration_metrics_contract_has_low_medium_high_comparison_profiles() {
        let contract = PhiIntegrationMetricsContract::v1();

        assert_eq!(contract.schema_version, PHI_INTEGRATION_METRICS_SCHEMA);
        assert!(contract
            .owned_runtime_surfaces
            .contains(&"adl identity phi".to_string()));
        assert_eq!(contract.comparison_profiles.len(), 3);
        assert!(contract
            .comparison_profiles
            .iter()
            .any(|profile| profile.integration_band == "low"));
        assert!(contract
            .comparison_profiles
            .iter()
            .any(|profile| profile.integration_band == "medium"));
        assert!(contract
            .comparison_profiles
            .iter()
            .any(|profile| profile.integration_band == "high"));
    }

    #[test]
    fn phi_integration_metrics_contract_is_engineering_facing_not_metaphysical() {
        let contract = PhiIntegrationMetricsContract::v1();

        assert!(contract
            .dimensions
            .iter()
            .any(|dimension| dimension.name == "graph_irreducibility"));
        assert!(contract
            .review_surface
            .non_goals
            .contains(&"formal IIT phi calculation".to_string()));
        assert!(contract
            .review_surface
            .comparison_rule
            .contains("without collapsing them into a single metaphysical scalar"));
        assert!(contract
            .proof_hook_output_path
            .contains("phi_integration_metrics_v1.json"));
    }
}
