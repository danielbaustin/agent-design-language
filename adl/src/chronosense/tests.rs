use super::*;
use chrono::{TimeZone, Utc};
use std::{
    env, fs,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

fn unique_temp_path(label: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time went backwards")
        .as_nanos();
    env::temp_dir().join(format!(
        "chronosense-{label}-{}-{nanos}",
        std::process::id()
    ))
}

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
fn identity_profile_rejects_invalid_birthday_timezone_and_empty_fields() {
    let err = IdentityProfile::from_birthday(
        "codex",
        "Codex",
        "not-a-date",
        "America/Los_Angeles",
        "daniel",
    )
    .expect_err("invalid RFC3339 should fail");
    assert!(err
        .to_string()
        .contains("invalid RFC3339 datetime 'not-a-date'"));

    let err = IdentityProfile::from_birthday(
        "codex",
        "Codex",
        "2026-03-30T13:34:00-07:00",
        "Mars/Olympus",
        "daniel",
    )
    .expect_err("invalid timezone should fail");
    assert!(err
        .to_string()
        .contains("unsupported timezone 'Mars/Olympus'"));

    let err = IdentityProfile::from_birthday(
        "  ",
        "Codex",
        "2026-03-30T13:34:00-07:00",
        "America/Los_Angeles",
        "daniel",
    )
    .expect_err("empty identity field should fail");
    assert!(err.to_string().contains("agent_id must not be empty"));
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

    let context =
        TemporalContext::from_now(now_utc, "America/Los_Angeles", Some(&profile)).expect("context");

    assert_eq!(context.schema_version, TEMPORAL_CONTEXT_SCHEMA);
    assert_eq!(context.local_date, "2026-03-31");
    assert_eq!(context.local_weekday, "Tuesday");
    assert_eq!(context.utc_offset, "-07:00");
    assert_eq!(context.identity_agent_id.as_deref(), Some("codex"));
    assert_eq!(context.age_days_since_birthday, Some(1));
}

#[test]
fn temporal_context_rejects_identity_with_invalid_persisted_birthday() {
    let profile = IdentityProfile {
        schema_version: IDENTITY_PROFILE_SCHEMA.to_string(),
        agent_id: "codex".to_string(),
        display_name: "Codex".to_string(),
        birthday_rfc3339: "bad-value".to_string(),
        birth_date_local: "2026-03-30".to_string(),
        birth_weekday_local: "Monday".to_string(),
        birth_timezone: "America/Los_Angeles".to_string(),
        created_by: "daniel".to_string(),
        continuity_mode: "repo_local_persistent".to_string(),
    };
    let now_utc = Utc.with_ymd_and_hms(2026, 3, 31, 20, 0, 0).unwrap();

    let err = TemporalContext::from_now(now_utc, "America/Los_Angeles", Some(&profile))
        .expect_err("invalid persisted birthday should fail");
    assert!(err
        .to_string()
        .contains("invalid RFC3339 datetime 'bad-value'"));
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
fn write_identity_profile_creates_parent_dirs_and_load_round_trips() {
    let root = unique_temp_path("identity-profile-roundtrip");
    let path = root.join("adl/identity/identity_profile.v1.json");
    let profile = IdentityProfile::from_birthday(
        "codex",
        "Codex",
        "2026-03-30T13:34:00-07:00",
        "America/Los_Angeles",
        "daniel",
    )
    .expect("profile");

    write_identity_profile(&path, &profile).expect("write identity profile");
    let loaded = load_identity_profile(&path).expect("load identity profile");

    assert_eq!(loaded.agent_id, "codex");
    assert_eq!(loaded.birth_timezone, "America/Los_Angeles");
    assert_eq!(loaded.birthday_rfc3339, "2026-03-30T13:34:00-07:00");

    let _ = fs::remove_dir_all(root);
}

#[test]
fn load_identity_profile_rejects_malformed_json_and_unsupported_schema() {
    let malformed_root = unique_temp_path("identity-profile-malformed");
    let malformed_path = malformed_root.join("identity_profile.v1.json");
    fs::create_dir_all(&malformed_root).expect("create malformed root");
    fs::write(&malformed_path, b"{not json").expect("write malformed profile");

    let err = load_identity_profile(&malformed_path).expect_err("malformed JSON should fail");
    assert!(err.to_string().contains("failed to parse identity profile"));

    let unsupported_root = unique_temp_path("identity-profile-unsupported");
    let unsupported_path = unsupported_root.join("identity_profile.v1.json");
    fs::create_dir_all(&unsupported_root).expect("create unsupported root");
    fs::write(
        &unsupported_path,
        serde_json::to_vec_pretty(&serde_json::json!({
            "schema_version": "identity_profile.v0",
            "agent_id": "codex",
            "display_name": "Codex",
            "birthday_rfc3339": "2026-03-30T20:34:00+00:00",
            "birth_date_local": "2026-03-30",
            "birth_weekday_local": "Monday",
            "birth_timezone": "America/Los_Angeles",
            "created_by": "daniel",
            "continuity_mode": "repo_local_persistent"
        }))
        .expect("json bytes"),
    )
    .expect("write unsupported profile");

    let err = load_identity_profile(&unsupported_path).expect_err("unsupported schema should fail");
    assert!(err
        .to_string()
        .contains("unsupported identity profile schema version 'identity_profile.v0'"));

    let _ = fs::remove_dir_all(malformed_root);
    let _ = fs::remove_dir_all(unsupported_root);
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

#[test]
fn instinct_model_contract_is_small_explicit_and_policy_subordinate() {
    let contract = InstinctModelContract::v1();

    assert_eq!(contract.schema_version, INSTINCT_MODEL_SCHEMA);
    assert!(contract
        .owned_runtime_surfaces
        .contains(&"adl identity instinct".to_string()));
    assert_eq!(contract.instinct_set.len(), 4);
    assert!(contract
        .instinct_set
        .iter()
        .any(|entry| entry.instinct_id == "integrity"));
    assert!(contract
        .instinct_set
        .iter()
        .all(|entry| entry.subordinate_to.contains(&"policy".to_string())));
}

#[test]
fn instinct_model_contract_distinguishes_instinct_from_goals_and_affect() {
    let contract = InstinctModelContract::v1();

    assert!(contract
        .semantics
        .distinctions_from_goals
        .iter()
        .any(|value| value.contains("task-specific")));
    assert!(contract
        .semantics
        .distinctions_from_affect
        .iter()
        .any(|value| value.contains("dynamic evaluation")));
    assert!(contract
        .review_surface
        .non_goals
        .contains(&"full psychology model".to_string()));
    assert!(contract
        .proof_hook_output_path
        .contains("instinct_model_v1.json"));
}

#[test]
fn instinct_runtime_surface_contract_proves_bounded_candidate_shift() {
    let contract = InstinctRuntimeSurfaceContract::v1();

    assert_eq!(contract.schema_version, INSTINCT_RUNTIME_SURFACE_SCHEMA);
    assert!(contract
        .owned_runtime_surfaces
        .contains(&"adl identity instinct-runtime".to_string()));
    assert!(contract
        .proof_cases
        .iter()
        .any(|case| case.expected_candidate_id == "cand-fast-verify"));
    assert!(contract
        .proof_cases
        .iter()
        .any(|case| case.expected_candidate_id == "cand-slow-defer"));
}

#[test]
fn instinct_runtime_surface_contract_keeps_high_risk_review_policy_visible() {
    let contract = InstinctRuntimeSurfaceContract::v1();

    assert!(contract
        .bounded_influence_rules
        .iter()
        .any(|value| value.contains("high-risk slow-path decisions stay review-first")));
    assert!(contract
        .review_surface
        .policy_override_rule
        .contains("high-risk slow-path review remains mandatory"));
    assert!(contract
        .proof_hook_output_path
        .contains("instinct_runtime_surface_v1.json"));
}
