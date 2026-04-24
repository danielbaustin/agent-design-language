//! Chronosense continuity and resumption semantics.
use serde::{Deserialize, Serialize};

use super::CONTINUITY_SEMANTICS_SCHEMA;

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
