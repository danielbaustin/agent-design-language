use serde::{Deserialize, Serialize};

use super::COMMITMENT_DEADLINE_SCHEMA;

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
