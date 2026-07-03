//! Chronosense temporal retrieval contracts.
use serde::{Deserialize, Serialize};

use super::TEMPORAL_QUERY_RETRIEVAL_SCHEMA;

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

impl TemporalQueryRetrievalContract {
    pub fn v1() -> Self {
        Self {
            schema_version: TEMPORAL_QUERY_RETRIEVAL_SCHEMA.to_string(),
            owned_runtime_surfaces: vec![
                "adl::chronosense::TemporalQueryRetrievalContract".to_string(),
                "adl::chronosense::TemporalQueryPrimitiveSet".to_string(),
                "adl::execute::state::runtime_control::MemoryQueryState".to_string(),
                "adl::obsmem_contract::MemoryTemporalAnchor".to_string(),
                "adl::obsmem_contract::MemoryTemporalQuery".to_string(),
                "adl::obsmem_contract::MemoryQuery".to_string(),
                "adl::obsmem_adapter::ObsMemAdapter::query_temporal".to_string(),
                "adl::obsmem_store::FileObsMemClient temporal filtering".to_string(),
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
                    "ordering by effective temporal anchor then event sequence".to_string(),
                    "filtering by continuity-relevant boundaries".to_string(),
                    "neighbor retrieval around focal event".to_string(),
                ],
                deterministic_ordering: vec![
                    "effective_epoch_ms_then_event_sequence_then_id_ascending".to_string(),
                    "workflow_id_then_run_id_ascending".to_string(),
                    "score_desc_id_asc".to_string(),
                    "evidence_adjusted_desc_id_asc".to_string(),
                    "id_asc".to_string(),
                ],
            },
            proof_fixture_hooks: vec![
                "obsmem_retrieval_policy::apply_policy_filters_and_orders_deterministically"
                    .to_string(),
                "obsmem_store::file_store_temporal_query_filters_and_orders_deterministically"
                    .to_string(),
                "obsmem_store::file_store_temporal_query_supports_staleness".to_string(),
                "obsmem_adapter::adapter_query_temporal_uses_file_store_temporal_index"
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
                "ObsMem / Memory Palace temporal anchors and local temporal index filtering are implemented here; causality, neighbor expansion, and distributed temporal truth remain downstream work"
                    .to_string(),
        }
    }
}
