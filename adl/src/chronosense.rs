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
}
