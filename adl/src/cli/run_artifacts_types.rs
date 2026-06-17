use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use ::adl::execute;

mod state;

pub(crate) use state::{
    sanitize_pause_adl_path, PauseStateArtifact, RunStateArtifact, PAUSE_STATE_SCHEMA_VERSION,
    RUN_STATE_SCHEMA_VERSION,
};

pub(crate) const RUN_STATUS_VERSION: u32 = 1;
pub(crate) const RUN_SUMMARY_VERSION: u32 = 1;
pub(crate) const SCORES_VERSION: u32 = 1;
pub(crate) const SUGGESTIONS_VERSION: u32 = 1;
pub(crate) const AEE_DECISION_VERSION: u32 = 1;
pub(crate) const COGNITIVE_SIGNALS_VERSION: u32 = 1;
pub(crate) const COGNITIVE_ARBITRATION_VERSION: u32 = 1;
pub(crate) const FAST_SLOW_PATH_VERSION: u32 = 1;
pub(crate) const AGENCY_SELECTION_VERSION: u32 = 1;
pub(crate) const BOUNDED_EXECUTION_VERSION: u32 = 1;
pub(crate) const EVALUATION_SIGNALS_VERSION: u32 = 1;
pub(crate) const REFRAMING_VERSION: u32 = 1;
pub(crate) const FREEDOM_GATE_VERSION: u32 = 1;
pub(crate) const AEE_CONVERGENCE_VERSION: u32 = 1;
pub(crate) const MEMORY_READ_VERSION: u32 = 1;
pub(crate) const MEMORY_WRITE_VERSION: u32 = 1;
pub(crate) const CONTROL_PATH_MEMORY_VERSION: u32 = 1;
pub(crate) const CONTROL_PATH_DECISIONS_VERSION: u32 = 1;
pub(crate) const CONTROL_PATH_ACTION_PROPOSALS_VERSION: u32 = 1;
pub(crate) const CONTROL_PATH_ACTION_MEDIATION_VERSION: u32 = 1;
pub(crate) const CONTROL_PATH_SKILL_MODEL_VERSION: u32 = 1;
pub(crate) const CONTROL_PATH_SKILL_EXECUTION_PROTOCOL_VERSION: u32 = 1;
pub(crate) const CONTROL_PATH_FINAL_RESULT_VERSION: u32 = 1;
pub(crate) const CONTROL_PATH_SECURITY_REVIEW_VERSION: u32 = 1;
pub(crate) const REASONING_GRAPH_VERSION: u32 = 1;
pub(crate) const REASONING_GRAPH_CONTRACT_REF_SCHEMA_VERSION: &str =
    "reasoning_graph.public_contract_ref.v1";
pub(crate) const UPSTREAM_DELEGATION_TRACE_RECORD_SCHEMA_VERSION: &str =
    "upstream_delegation.trace_record.v1";
pub(crate) const CLUSTER_GROUNDWORK_VERSION: u32 = 1;

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn sanitize_pause_adl_path_normalizes_relative_paths() {
        let rel = Path::new("./fixtures/../demo/example.adl");
        assert_eq!(sanitize_pause_adl_path(rel), "fixtures/../demo/example.adl");
    }

    #[test]
    fn sanitize_pause_adl_path_relativizes_absolute_paths_inside_cwd() {
        let cwd = std::env::current_dir().expect("cwd");
        let path = cwd.join("fixtures").join("resume").join("example.adl");
        assert_eq!(
            sanitize_pause_adl_path(&path),
            "fixtures/resume/example.adl"
        );
    }

    #[test]
    fn sanitize_pause_adl_path_redacts_external_absolute_paths_to_filename() {
        let path = Path::new("/tmp/external/example.adl");
        assert_eq!(sanitize_pause_adl_path(path), "external:/example.adl");
    }

    #[test]
    fn sanitize_pause_adl_path_handles_root_without_filename() {
        let path = Path::new("/");
        assert_eq!(sanitize_pause_adl_path(path), "external:/<unknown>");
    }

    #[test]
    fn reasoning_graph_contract_refs_accept_legacy_compatible_public_refs() {
        let graph = ReasoningGraphArtifact {
            reasoning_graph_version: REASONING_GRAPH_VERSION,
            run_id: "run-1".to_string(),
            generated_from: AeeDecisionGeneratedFrom {
                artifact_model_version: 1,
                run_summary_version: 1,
                suggestions_version: 1,
                scores_version: Some(1),
            },
            public_contract: Some(ReasoningGraphPublicContractRef {
                schema_version: REASONING_GRAPH_CONTRACT_REF_SCHEMA_VERSION.to_string(),
                artifact_ref: "artifacts/run-1/learning/reasoning_graph.v1.json".to_string(),
                source_trace_ref: "artifacts/run-1/logs/trace_v1.json".to_string(),
                redaction_policy_ref: Some(
                    "artifacts/run-1/governed/redaction_policy.json".to_string(),
                ),
                compatibility: "legacy_compatible".to_string(),
                private_reasoning_exposed: false,
            }),
            upstream_delegations: vec![sample_upstream_delegation_record()],
            graph: ReasoningGraphRecord {
                graph_id: "graph-1".to_string(),
                dominant_affect_mode: "steady_state".to_string(),
                ranking_rule: "stable".to_string(),
                selected_path: ReasoningGraphSelection {
                    selected_node_id: "action.maintain".to_string(),
                    selected_intent: "maintain_current_policy".to_string(),
                    selected_target: "workflow-runtime".to_string(),
                    graph_derived_output: "Public bounded decision summary.".to_string(),
                    affect_changed_ranking: false,
                },
                nodes: vec![ReasoningGraphNode {
                    node_id: "action.maintain".to_string(),
                    node_kind: "action".to_string(),
                    label: "maintain".to_string(),
                    rank: 1,
                    priority_score: 1,
                    affect_mode: None,
                    rationale: "Public bounded summary.".to_string(),
                }],
                edges: Vec::new(),
            },
        };

        validate_reasoning_graph_artifact_contract_refs(&graph)
            .expect("public reasoning graph refs should validate");
    }

    #[test]
    fn reasoning_graph_contract_refs_reject_private_reasoning_and_host_paths() {
        let mut graph = ReasoningGraphArtifact {
            reasoning_graph_version: REASONING_GRAPH_VERSION,
            run_id: "run-1".to_string(),
            generated_from: AeeDecisionGeneratedFrom {
                artifact_model_version: 1,
                run_summary_version: 1,
                suggestions_version: 1,
                scores_version: None,
            },
            public_contract: Some(ReasoningGraphPublicContractRef {
                schema_version: REASONING_GRAPH_CONTRACT_REF_SCHEMA_VERSION.to_string(),
                artifact_ref: "artifacts/run-1/learning/reasoning_graph.v1.json".to_string(),
                source_trace_ref: "artifacts/run-1/logs/trace_v1.json".to_string(),
                redaction_policy_ref: None,
                compatibility: "legacy_compatible".to_string(),
                private_reasoning_exposed: true,
            }),
            upstream_delegations: Vec::new(),
            graph: ReasoningGraphRecord {
                graph_id: "graph-1".to_string(),
                dominant_affect_mode: "steady_state".to_string(),
                ranking_rule: "stable".to_string(),
                selected_path: ReasoningGraphSelection {
                    selected_node_id: "action.maintain".to_string(),
                    selected_intent: "maintain_current_policy".to_string(),
                    selected_target: "workflow-runtime".to_string(),
                    graph_derived_output: "private chain-of-thought: hidden scratchpad".to_string(),
                    affect_changed_ranking: false,
                },
                nodes: Vec::new(),
                edges: Vec::new(),
            },
        };

        let err = validate_reasoning_graph_artifact_contract_refs(&graph)
            .expect_err("private reasoning and host paths must fail");
        assert!(err.to_string().contains("private reasoning"));

        graph
            .public_contract
            .as_mut()
            .expect("contract")
            .private_reasoning_exposed = false;
        graph
            .public_contract
            .as_mut()
            .expect("contract")
            .artifact_ref = "/Users/daniel/leak.json".to_string();
        let err = validate_reasoning_graph_artifact_contract_refs(&graph)
            .expect_err("host path must still fail");
        assert!(err.to_string().contains("artifact_ref"));
    }

    #[test]
    fn upstream_delegation_record_rejects_hidden_authority_paths() {
        let mut record = sample_upstream_delegation_record();
        record.authority_basis_refs.clear();
        let err = validate_upstream_delegation_trace_record(&record)
            .expect_err("missing authority basis must fail");
        assert!(err.to_string().contains("authority_basis_refs"));

        let mut record = sample_upstream_delegation_record();
        record.parent_responsibility_retained = false;
        let err = validate_upstream_delegation_trace_record(&record)
            .expect_err("parent responsibility must be retained");
        assert!(err.to_string().contains("parent_responsibility_retained"));

        let mut record = sample_upstream_delegation_record();
        record.target_class = "unknown_runtime".to_string();
        let err = validate_upstream_delegation_trace_record(&record)
            .expect_err("unsupported target must fail closed");
        assert!(err.to_string().contains("target_class"));
    }

    #[test]
    fn upstream_delegation_record_rejects_vector_field_leakage() {
        let mut record = sample_upstream_delegation_record();
        record.decision_source_refs = vec!["/Users/daniel/private/decision.json".to_string()];
        let err = validate_upstream_delegation_trace_record(&record)
            .expect_err("decision source host path must fail");
        assert!(err.to_string().contains("decision_source_refs"));

        let mut record = sample_upstream_delegation_record();
        record.provider_or_runtime_ref = Some("hidden scratchpad provider notes".to_string());
        let err = validate_upstream_delegation_trace_record(&record)
            .expect_err("provider runtime private text must fail");
        assert!(err.to_string().contains("provider_or_runtime_ref"));
    }

    #[test]
    fn reasoning_graph_contract_ref_serializes_without_breaking_legacy_fields() {
        let record = sample_upstream_delegation_record();
        let json = serde_json::to_value(&record).expect("serialize delegation record");
        assert_eq!(
            json["schema_version"],
            UPSTREAM_DELEGATION_TRACE_RECORD_SCHEMA_VERSION
        );
        assert_eq!(json["parent_responsibility_retained"], true);
        assert_eq!(json["parent_review_required"], true);
        assert_eq!(json["parent_authority_inherited"], false);
        assert_eq!(json["private_reasoning_exposed"], false);
        assert_eq!(json["secrets_exposed"], false);
    }

    fn sample_upstream_delegation_record() -> UpstreamDelegationTraceRecord {
        UpstreamDelegationTraceRecord {
            schema_version: UPSTREAM_DELEGATION_TRACE_RECORD_SCHEMA_VERSION.to_string(),
            delegation_id: "delegation-1".to_string(),
            parent_run_ref: "artifacts/run-1/run.json".to_string(),
            source_actor_id: "actor.agent.parent".to_string(),
            source_actor_kind: "agent".to_string(),
            source_role_ref: "role.runtime-parent".to_string(),
            upstream_target_id: "provider.openai.review".to_string(),
            target_class: "hosted_provider".to_string(),
            provider_or_runtime_ref: Some("provider/openai".to_string()),
            capability_id: "review.findings".to_string(),
            scope: "bounded review".to_string(),
            deliverables: vec!["findings packet".to_string()],
            forbidden_actions: vec!["merge".to_string()],
            inherited_constraints: vec!["no secrets".to_string()],
            trace_requirements: vec!["authority_basis".to_string()],
            acc_ref: "artifacts/run-1/acc/contract.json".to_string(),
            grant_ref: "grant.review".to_string(),
            authority_basis_refs: vec!["delegation.operator-to-agent".to_string()],
            delegation_chain_refs: vec!["delegation.operator-to-agent".to_string()],
            redelegation_allowed: false,
            max_depth: 1,
            parent_responsibility_retained: true,
            parent_review_required: true,
            parent_authority_inherited: false,
            lifecycle_state: "requested".to_string(),
            policy_decision: "needs_approval".to_string(),
            acc_decision: "delegated".to_string(),
            grant_status: "delegated".to_string(),
            decision_source_refs: vec!["artifacts/run-1/acc/contract.json".to_string()],
            failure_code: None,
            delegated_output_ref: None,
            parent_integration_ref: None,
            reasoning_graph_ref: Some(
                "artifacts/run-1/learning/reasoning_graph.v1.json".to_string(),
            ),
            private_reasoning_exposed: false,
            secrets_exposed: false,
            public_summary: "Delegation is pending parent review.".to_string(),
        }
    }
}

#[derive(Debug, Serialize)]
pub(crate) struct StepStateArtifact {
    pub(crate) step_id: String,
    pub(crate) agent_id: String,
    pub(crate) provider_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) conversation: Option<ConversationTurnArtifact>,
    pub(crate) status: String,
    pub(crate) output_artifact_path: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ConversationTurnArtifact {
    pub(crate) id: String,
    pub(crate) speaker: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) sequence: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) thread_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) responds_to: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct RunStatusArtifact {
    pub(crate) run_status_version: u32,
    pub(crate) run_id: String,
    pub(crate) workflow_id: String,
    pub(crate) overall_status: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) failure_kind: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) failed_step_id: Option<String>,
    pub(crate) completed_steps: Vec<String>,
    pub(crate) pending_steps: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) started_steps: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) resilience_classification: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) continuity_status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) preservation_status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) shepherd_decision: Option<String>,
    pub(crate) persistence_mode: String,
    pub(crate) cleanup_disposition: String,
    pub(crate) resume_guard: String,
    pub(crate) state_artifacts: Vec<String>,
    pub(crate) attempt_counts_by_step: BTreeMap<String, u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) effective_max_concurrency: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) effective_max_concurrency_source: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct RunSummaryArtifact {
    pub(crate) run_summary_version: u32,
    pub(crate) artifact_model_version: u32,
    pub(crate) run_id: String,
    pub(crate) workflow_id: String,
    pub(crate) adl_version: String,
    pub(crate) swarm_version: String,
    pub(crate) status: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) error_kind: Option<String>,
    pub(crate) counts: RunSummaryCounts,
    pub(crate) policy: RunSummaryPolicy,
    pub(crate) links: RunSummaryLinks,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct RunSummaryCounts {
    pub(crate) total_steps: usize,
    pub(crate) completed_steps: usize,
    pub(crate) failed_steps: usize,
    pub(crate) provider_call_count: usize,
    pub(crate) delegation_steps: usize,
    pub(crate) delegation_requires_verification_steps: usize,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct RunSummaryPolicy {
    pub(crate) security_envelope_enabled: bool,
    pub(crate) signing_required: bool,
    pub(crate) key_id_required: bool,
    pub(crate) verify_allowed_algs: Vec<String>,
    pub(crate) verify_allowed_key_sources: Vec<String>,
    pub(crate) sandbox_policy: String,
    pub(crate) security_denials_by_code: BTreeMap<String, usize>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct RunSummaryLinks {
    pub(crate) run_json: String,
    pub(crate) steps_json: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) pause_state_json: Option<String>,
    pub(crate) outputs_dir: String,
    pub(crate) logs_dir: String,
    pub(crate) learning_dir: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) scores_json: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) suggestions_json: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) aee_decision_json: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) cognitive_signals_json: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) fast_slow_path_json: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) agency_selection_json: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) bounded_execution_json: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) evaluation_signals_json: Option<String>,
    pub(crate) cognitive_arbitration_json: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) affect_state_json: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) reasoning_graph_json: Option<String>,
    pub(crate) overlays_dir: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) cluster_groundwork_json: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) trace_json: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ClusterGroundworkArtifact {
    pub(crate) cluster_groundwork_version: u32,
    pub(crate) run_id: String,
    pub(crate) workflow_id: String,
    pub(crate) coordinator_id: String,
    pub(crate) worker_id: String,
    pub(crate) canonical_ordering_key: String,
    pub(crate) frontier_ordering: String,
    pub(crate) readiness_frontiers: Vec<ClusterReadyFrontier>,
    pub(crate) lease_records: Vec<ClusterLeaseRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ClusterReadyFrontier {
    pub(crate) frontier_index: u32,
    pub(crate) ready_step_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ClusterLeaseRecord {
    pub(crate) issued_sequence: u32,
    pub(crate) lease_id: String,
    pub(crate) step_id: String,
    pub(crate) depends_on: Vec<String>,
    pub(crate) observed_attempts: u32,
    pub(crate) claim_owner: String,
    pub(crate) worker_id: String,
    pub(crate) lease_state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ScoresArtifact {
    pub(crate) scores_version: u32,
    pub(crate) run_id: String,
    pub(crate) generated_from: ScoresGeneratedFrom,
    pub(crate) summary: ScoresSummary,
    pub(crate) metrics: ScoresMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ScoresGeneratedFrom {
    pub(crate) artifact_model_version: u32,
    pub(crate) run_summary_version: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ScoresSummary {
    pub(crate) success_ratio: f64,
    pub(crate) failure_count: usize,
    pub(crate) retry_count: usize,
    pub(crate) delegation_denied_count: usize,
    pub(crate) security_denied_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ScoresMetrics {
    pub(crate) scheduler_max_parallel_observed: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct SuggestionsArtifact {
    pub(crate) suggestions_version: u32,
    pub(crate) run_id: String,
    pub(crate) generated_from: SuggestionsGeneratedFrom,
    pub(crate) suggestions: Vec<SuggestionItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct SuggestionsGeneratedFrom {
    pub(crate) artifact_model_version: u32,
    pub(crate) run_summary_version: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) scores_version: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct SuggestionItem {
    pub(crate) id: String,
    pub(crate) category: String,
    pub(crate) severity: String,
    pub(crate) rationale: String,
    pub(crate) evidence: SuggestionEvidence,
    pub(crate) proposed_change: SuggestedChangeIntent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct SuggestionEvidence {
    pub(crate) failure_count: usize,
    pub(crate) retry_count: usize,
    pub(crate) delegation_denied_count: usize,
    pub(crate) security_denied_count: usize,
    pub(crate) success_ratio: f64,
    pub(crate) scheduler_max_parallel_observed: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct SuggestedChangeIntent {
    pub(crate) intent: String,
    pub(crate) target: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct AeeDecisionArtifact {
    pub(crate) aee_decision_version: u32,
    pub(crate) run_id: String,
    pub(crate) generated_from: AeeDecisionGeneratedFrom,
    pub(crate) affect_state: AffectStateRef,
    pub(crate) decision: AeeDecisionRecord,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct AeeDecisionGeneratedFrom {
    pub(crate) artifact_model_version: u32,
    pub(crate) run_summary_version: u32,
    pub(crate) suggestions_version: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) scores_version: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct AffectStateArtifact {
    pub(crate) affect_state_version: u32,
    pub(crate) run_id: String,
    pub(crate) generated_from: AeeDecisionGeneratedFrom,
    pub(crate) affect: AffectStateRecord,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct CognitiveSignalsArtifact {
    pub(crate) cognitive_signals_version: u32,
    pub(crate) run_id: String,
    pub(crate) generated_from: AeeDecisionGeneratedFrom,
    pub(crate) instinct: CognitiveInstinctRecord,
    pub(crate) affect: CognitiveAffectSignalRecord,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct CognitiveInstinctRecord {
    pub(crate) instinct_profile_id: String,
    pub(crate) dominant_instinct: String,
    pub(crate) completion_pressure: String,
    pub(crate) integrity_bias: String,
    pub(crate) curiosity_bias: String,
    pub(crate) candidate_selection_bias: String,
    pub(crate) deterministic_update_rule: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct CognitiveAffectSignalRecord {
    pub(crate) affect_state_id: String,
    pub(crate) urgency_level: String,
    pub(crate) salience_level: String,
    pub(crate) persistence_pressure: String,
    pub(crate) confidence_shift: String,
    pub(crate) downstream_influence: String,
    pub(crate) deterministic_update_rule: String,
}

#[cfg(test)]
pub(crate) type CognitiveSignalsState = execute::CognitiveSignalsState;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct AffectStateRecord {
    pub(crate) affect_state_id: String,
    pub(crate) affect_mode: String,
    pub(crate) urgency_level: String,
    pub(crate) frustration_level: String,
    pub(crate) confidence_shift: String,
    pub(crate) recovery_bias: u32,
    pub(crate) downstream_priority: String,
    pub(crate) update_reason: String,
    pub(crate) deterministic_update_rule: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct AffectStateRef {
    pub(crate) affect_state_id: String,
    pub(crate) affect_mode: String,
    pub(crate) downstream_priority: String,
    pub(crate) recovery_bias: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct CognitiveArbitrationArtifact {
    pub(crate) cognitive_arbitration_version: u32,
    pub(crate) run_id: String,
    pub(crate) generated_from: AeeDecisionGeneratedFrom,
    pub(crate) route_selected: String,
    pub(crate) reasoning_mode: String,
    pub(crate) confidence: String,
    pub(crate) risk_class: String,
    pub(crate) applied_constraints: Vec<String>,
    pub(crate) cost_latency_assumption: String,
    pub(crate) route_reason: String,
    pub(crate) deterministic_selection_rule: String,
}

#[cfg(test)]
pub(crate) type CognitiveArbitrationState = execute::CognitiveArbitrationState;

#[cfg(test)]
pub(crate) type FastSlowPathState = execute::FastSlowPathState;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct FastSlowPathArtifact {
    pub(crate) fast_slow_path_version: u32,
    pub(crate) run_id: String,
    pub(crate) generated_from: AeeDecisionGeneratedFrom,
    pub(crate) arbitration_route: String,
    pub(crate) selected_path: String,
    pub(crate) path_family: String,
    pub(crate) runtime_branch_taken: String,
    pub(crate) handoff_state: String,
    pub(crate) candidate_strategy: String,
    pub(crate) review_depth: String,
    pub(crate) execution_profile: String,
    pub(crate) termination_expectation: String,
    pub(crate) path_difference_summary: String,
    pub(crate) deterministic_handoff_rule: String,
}

#[cfg(test)]
pub(crate) type AgencySelectionState = execute::AgencySelectionState;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct AgencySelectionArtifact {
    pub(crate) agency_selection_version: u32,
    pub(crate) run_id: String,
    pub(crate) generated_from: AeeDecisionGeneratedFrom,
    pub(crate) candidate_generation_basis: String,
    pub(crate) selection_mode: String,
    pub(crate) candidate_set: Vec<AgencyCandidateRecord>,
    pub(crate) selected_candidate_id: String,
    pub(crate) selected_candidate_reason: String,
    pub(crate) deterministic_selection_rule: String,
}

pub(crate) type AgencyCandidateRecord = execute::AgencyCandidateRecord;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct BoundedExecutionArtifact {
    pub(crate) bounded_execution_version: u32,
    pub(crate) run_id: String,
    pub(crate) generated_from: AeeDecisionGeneratedFrom,
    pub(crate) selected_candidate_id: String,
    pub(crate) selected_path: String,
    pub(crate) execution_status: String,
    pub(crate) continuation_state: String,
    pub(crate) provisional_termination_state: String,
    pub(crate) iteration_count: u32,
    pub(crate) iterations: Vec<BoundedExecutionIteration>,
    pub(crate) deterministic_execution_rule: String,
}

pub(crate) type BoundedExecutionState = execute::BoundedExecutionState;

pub(crate) type BoundedExecutionIteration = execute::BoundedExecutionIteration;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct EvaluationSignalsArtifact {
    pub(crate) evaluation_signals_version: u32,
    pub(crate) run_id: String,
    pub(crate) generated_from: AeeDecisionGeneratedFrom,
    pub(crate) selected_candidate_id: String,
    pub(crate) selected_path: String,
    pub(crate) progress_signal: String,
    pub(crate) contradiction_signal: String,
    pub(crate) failure_signal: String,
    pub(crate) termination_reason: String,
    pub(crate) behavior_effect: String,
    pub(crate) next_control_action: String,
    pub(crate) deterministic_evaluation_rule: String,
}

pub(crate) type EvaluationControlState = execute::EvaluationControlState;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ReframingArtifact {
    pub(crate) reframing_version: u32,
    pub(crate) run_id: String,
    pub(crate) generated_from: AeeDecisionGeneratedFrom,
    pub(crate) selected_candidate_id: String,
    pub(crate) selected_path: String,
    pub(crate) frame_adequacy_score: u32,
    pub(crate) reframing_trigger: String,
    pub(crate) reframing_reason: String,
    pub(crate) prior_frame: String,
    pub(crate) new_frame: String,
    pub(crate) reexecution_choice: String,
    pub(crate) post_reframe_state: String,
    pub(crate) deterministic_reframing_rule: String,
}

pub(crate) type ReframingControlState = execute::ReframingControlState;

pub(crate) type FreedomGateState = execute::FreedomGateState;
pub(crate) type FreedomGateInputState = execute::FreedomGateInputState;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct FreedomGateArtifact {
    pub(crate) freedom_gate_version: u32,
    pub(crate) run_id: String,
    pub(crate) generated_from: AeeDecisionGeneratedFrom,
    pub(crate) input: FreedomGateInputState,
    pub(crate) gate_decision: String,
    pub(crate) reason_code: String,
    pub(crate) decision_reason: String,
    pub(crate) selected_action_or_none: Option<String>,
    pub(crate) commitment_blocked: bool,
    pub(crate) judgment_boundary: String,
    pub(crate) required_follow_up: String,
    pub(crate) decision_record_kind: String,
    pub(crate) deterministic_gate_rule: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct AeeConvergenceArtifact {
    pub(crate) aee_convergence_version: u32,
    pub(crate) run_id: String,
    pub(crate) generated_from: AeeDecisionGeneratedFrom,
    pub(crate) selected_candidate_id: String,
    pub(crate) selected_path: String,
    pub(crate) convergence_state: String,
    pub(crate) progress_signal: String,
    pub(crate) stop_condition_family: String,
    pub(crate) termination_reason: String,
    pub(crate) next_control_action: String,
    pub(crate) gate_decision: String,
    pub(crate) iteration_count: u32,
    pub(crate) strategy_change_count: u32,
    pub(crate) strategy_change_visible: bool,
    pub(crate) reframing_trigger: String,
    pub(crate) reviewer_summary: String,
    pub(crate) deterministic_convergence_rule: String,
}

pub(crate) type MemoryReadState = execute::MemoryReadState;
pub(crate) type MemoryQueryState = execute::MemoryQueryState;
pub(crate) type MemoryReadEntry = execute::MemoryReadEntry;
pub(crate) type MemoryWriteState = execute::MemoryWriteState;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct MemoryReadArtifact {
    pub(crate) memory_read_version: u32,
    pub(crate) run_id: String,
    pub(crate) generated_from: AeeDecisionGeneratedFrom,
    pub(crate) query: MemoryQueryState,
    pub(crate) read_count: u32,
    pub(crate) entries: Vec<MemoryReadEntry>,
    pub(crate) retrieval_order: String,
    pub(crate) influence_summary: String,
    pub(crate) influenced_stage: String,
    pub(crate) deterministic_read_rule: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct MemoryWriteArtifact {
    pub(crate) memory_write_version: u32,
    pub(crate) run_id: String,
    pub(crate) generated_from: AeeDecisionGeneratedFrom,
    pub(crate) entry_id: String,
    pub(crate) content: String,
    pub(crate) tags: Vec<String>,
    pub(crate) logical_timestamp: String,
    pub(crate) write_reason: String,
    pub(crate) source: String,
    pub(crate) deterministic_write_rule: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ControlPathMemoryArtifact {
    pub(crate) control_path_memory_version: u32,
    pub(crate) run_id: String,
    pub(crate) generated_from: AeeDecisionGeneratedFrom,
    pub(crate) read: MemoryReadArtifact,
    pub(crate) write: MemoryWriteArtifact,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ControlPathActionProposalsArtifact {
    pub(crate) control_path_action_proposals_version: u32,
    pub(crate) run_id: String,
    pub(crate) generated_from: AeeDecisionGeneratedFrom,
    pub(crate) proposal_schema_name: String,
    pub(crate) proposal_schema_fields: Vec<String>,
    pub(crate) proposal_kind_vocabulary: Vec<String>,
    pub(crate) proposals: Vec<ActionProposalRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ActionProposalRecord {
    pub(crate) proposal_id: String,
    pub(crate) kind: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) target: Option<String>,
    #[serde(default)]
    pub(crate) arguments: BTreeMap<String, String>,
    pub(crate) intent: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) content: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) confidence: Option<f64>,
    pub(crate) requires_approval: bool,
    #[serde(default)]
    pub(crate) metadata: BTreeMap<String, String>,
    pub(crate) non_authoritative: bool,
    pub(crate) temporal_anchor: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ControlPathDecisionsArtifact {
    pub(crate) control_path_decisions_version: u32,
    pub(crate) run_id: String,
    pub(crate) generated_from: AeeDecisionGeneratedFrom,
    pub(crate) decision_schema_name: String,
    pub(crate) decision_schema_fields: Vec<String>,
    pub(crate) outcome_class_vocabulary: Vec<String>,
    pub(crate) surfaces: Vec<DecisionSurfaceRecord>,
    pub(crate) decisions: Vec<DecisionRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct DecisionSurfaceRecord {
    pub(crate) surface_id: String,
    pub(crate) surface_family: String,
    pub(crate) bounded_role: String,
    pub(crate) outcome_classes: Vec<String>,
    pub(crate) temporal_anchor_ref: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct DecisionRecord {
    pub(crate) decision_id: String,
    pub(crate) surface_id: String,
    pub(crate) proposal_or_action: String,
    pub(crate) outcome_class: String,
    pub(crate) decision_maker: String,
    pub(crate) policy_bindings: Vec<String>,
    pub(crate) rationale: String,
    pub(crate) downstream_consequence: String,
    pub(crate) temporal_anchor: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ControlPathActionMediationArtifact {
    pub(crate) control_path_action_mediation_version: u32,
    pub(crate) run_id: String,
    pub(crate) generated_from: AeeDecisionGeneratedFrom,
    pub(crate) authority_boundary: String,
    pub(crate) mediation_outcome_vocabulary: Vec<String>,
    pub(crate) mediation: ActionMediationRecord,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ActionMediationRecord {
    pub(crate) mediation_id: String,
    pub(crate) proposal_id: String,
    pub(crate) decision_id: String,
    pub(crate) runtime_authority: String,
    pub(crate) judgment_boundary: String,
    pub(crate) mediation_outcome: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) approved_action_or_none: Option<String>,
    pub(crate) required_follow_up: String,
    pub(crate) validation_checks: Vec<String>,
    pub(crate) policy_bindings: Vec<String>,
    pub(crate) rationale: String,
    pub(crate) temporal_anchor: String,
    pub(crate) trace_expectation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ControlPathSkillModelArtifact {
    pub(crate) control_path_skill_model_version: u32,
    pub(crate) run_id: String,
    pub(crate) generated_from: AeeDecisionGeneratedFrom,
    pub(crate) skill_schema_name: String,
    pub(crate) skill_schema_fields: Vec<String>,
    pub(crate) distinction_vocabulary: Vec<String>,
    pub(crate) selected_execution_unit_kind: String,
    pub(crate) skill: SkillDefinitionRecord,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct SkillDefinitionRecord {
    pub(crate) skill_id: String,
    pub(crate) selection_status: String,
    pub(crate) purpose: String,
    pub(crate) bounded_role: String,
    pub(crate) input_contract_fields: Vec<String>,
    pub(crate) output_contract_surfaces: Vec<String>,
    pub(crate) stop_condition: String,
    pub(crate) distinguished_from: Vec<String>,
    pub(crate) temporal_anchor: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ControlPathSkillExecutionProtocolArtifact {
    pub(crate) control_path_skill_execution_protocol_version: u32,
    pub(crate) run_id: String,
    pub(crate) generated_from: AeeDecisionGeneratedFrom,
    pub(crate) protocol_name: String,
    pub(crate) lifecycle_stages: Vec<String>,
    pub(crate) invocation: SkillInvocationProtocolRecord,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct SkillInvocationProtocolRecord {
    pub(crate) invocation_id: String,
    pub(crate) skill_id: String,
    pub(crate) proposal_id: String,
    pub(crate) decision_id: String,
    pub(crate) invocation_kind: String,
    #[serde(default)]
    pub(crate) invocation_context: BTreeMap<String, String>,
    pub(crate) input_validation_expectation: String,
    pub(crate) lifecycle_state: String,
    pub(crate) authorization_decision: String,
    pub(crate) output_contract_surfaces: Vec<String>,
    pub(crate) error_outcome_vocabulary: Vec<String>,
    pub(crate) trace_expectation: String,
    pub(crate) temporal_anchor: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ControlPathFinalResultArtifact {
    pub(crate) control_path_final_result_version: u32,
    pub(crate) run_id: String,
    pub(crate) route_selected: String,
    pub(crate) selected_candidate: String,
    pub(crate) termination_reason: String,
    pub(crate) gate_decision: String,
    pub(crate) final_result: String,
    pub(crate) commitment_blocked: bool,
    pub(crate) next_control_action: String,
    pub(crate) stage_order: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ControlPathSecurityReviewArtifact {
    pub(crate) control_path_security_review_version: u32,
    pub(crate) run_id: String,
    pub(crate) generated_from: AeeDecisionGeneratedFrom,
    pub(crate) threat_model: SecurityThreatModelRecord,
    pub(crate) posture: SecurityPostureRecord,
    pub(crate) trust_under_adversary: SecurityTrustUnderAdversaryRecord,
    pub(crate) evidence: SecurityReviewEvidenceRecord,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct SecurityThreatModelRecord {
    pub(crate) attacker_pressure: String,
    pub(crate) active_trust_boundaries: Vec<String>,
    pub(crate) canonical_threat_classes: Vec<String>,
    pub(crate) required_mitigations: Vec<String>,
    pub(crate) reviewer_visible_surfaces: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct SecurityPostureRecord {
    pub(crate) declared_posture: String,
    pub(crate) accepted_risk_level: String,
    pub(crate) commitment_policy: String,
    pub(crate) mitigation_authority: String,
    pub(crate) runtime_consequence: String,
    pub(crate) posture_rationale: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct SecurityTrustUnderAdversaryRecord {
    pub(crate) trust_state: String,
    pub(crate) trusted_surfaces: Vec<String>,
    pub(crate) reduced_trust_surfaces: Vec<String>,
    pub(crate) revalidation_requirements: Vec<String>,
    pub(crate) escalation_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct SecurityReviewEvidenceRecord {
    pub(crate) route_selected: String,
    pub(crate) risk_class: String,
    pub(crate) mediation_outcome: String,
    pub(crate) gate_decision: String,
    pub(crate) final_result: String,
    pub(crate) security_denied_count: usize,
    pub(crate) security_envelope_enabled: bool,
    pub(crate) signing_required: bool,
    pub(crate) key_id_required: bool,
    pub(crate) verify_allowed_algs: Vec<String>,
    pub(crate) verify_allowed_key_sources: Vec<String>,
    pub(crate) sandbox_policy: String,
    pub(crate) trace_visibility_expectation: String,
}

pub(crate) struct ControlPathSummaryContext<'a> {
    pub(crate) signals: &'a CognitiveSignalsArtifact,
    pub(crate) agency: &'a AgencySelectionArtifact,
    pub(crate) arbitration: &'a CognitiveArbitrationArtifact,
    pub(crate) execution: &'a BoundedExecutionArtifact,
    pub(crate) evaluation: &'a EvaluationSignalsArtifact,
    pub(crate) reframing: &'a ReframingArtifact,
    pub(crate) convergence: &'a AeeConvergenceArtifact,
    pub(crate) memory: &'a ControlPathMemoryArtifact,
    pub(crate) action_proposals: &'a ControlPathActionProposalsArtifact,
    pub(crate) skill_model: &'a ControlPathSkillModelArtifact,
    pub(crate) skill_execution_protocol: &'a ControlPathSkillExecutionProtocolArtifact,
    pub(crate) mediation: &'a ControlPathActionMediationArtifact,
    pub(crate) freedom_gate: &'a FreedomGateArtifact,
    pub(crate) final_result: &'a ControlPathFinalResultArtifact,
    pub(crate) security_review: &'a ControlPathSecurityReviewArtifact,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ReasoningGraphArtifact {
    pub(crate) reasoning_graph_version: u32,
    pub(crate) run_id: String,
    pub(crate) generated_from: AeeDecisionGeneratedFrom,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) public_contract: Option<ReasoningGraphPublicContractRef>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub(crate) upstream_delegations: Vec<UpstreamDelegationTraceRecord>,
    pub(crate) graph: ReasoningGraphRecord,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ReasoningGraphPublicContractRef {
    pub(crate) schema_version: String,
    pub(crate) artifact_ref: String,
    pub(crate) source_trace_ref: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) redaction_policy_ref: Option<String>,
    pub(crate) compatibility: String,
    pub(crate) private_reasoning_exposed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct UpstreamDelegationTraceRecord {
    pub(crate) schema_version: String,
    pub(crate) delegation_id: String,
    pub(crate) parent_run_ref: String,
    pub(crate) source_actor_id: String,
    pub(crate) source_actor_kind: String,
    pub(crate) source_role_ref: String,
    pub(crate) upstream_target_id: String,
    pub(crate) target_class: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) provider_or_runtime_ref: Option<String>,
    pub(crate) capability_id: String,
    pub(crate) scope: String,
    pub(crate) deliverables: Vec<String>,
    pub(crate) forbidden_actions: Vec<String>,
    pub(crate) inherited_constraints: Vec<String>,
    pub(crate) trace_requirements: Vec<String>,
    pub(crate) acc_ref: String,
    pub(crate) grant_ref: String,
    pub(crate) authority_basis_refs: Vec<String>,
    pub(crate) delegation_chain_refs: Vec<String>,
    pub(crate) redelegation_allowed: bool,
    pub(crate) max_depth: u8,
    pub(crate) parent_responsibility_retained: bool,
    pub(crate) parent_review_required: bool,
    pub(crate) parent_authority_inherited: bool,
    pub(crate) lifecycle_state: String,
    pub(crate) policy_decision: String,
    pub(crate) acc_decision: String,
    pub(crate) grant_status: String,
    pub(crate) decision_source_refs: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) failure_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) delegated_output_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) parent_integration_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) reasoning_graph_ref: Option<String>,
    pub(crate) private_reasoning_exposed: bool,
    pub(crate) secrets_exposed: bool,
    pub(crate) public_summary: String,
}

pub(crate) fn validate_reasoning_graph_artifact_contract_refs(
    artifact: &ReasoningGraphArtifact,
) -> anyhow::Result<()> {
    if let Some(contract) = &artifact.public_contract {
        if contract.schema_version != REASONING_GRAPH_CONTRACT_REF_SCHEMA_VERSION {
            anyhow::bail!(
                "reasoning_graph.public_contract.schema_version must be {}",
                REASONING_GRAPH_CONTRACT_REF_SCHEMA_VERSION
            );
        }
        validate_public_ref(
            "reasoning_graph.public_contract.artifact_ref",
            &contract.artifact_ref,
        )?;
        validate_public_ref(
            "reasoning_graph.public_contract.source_trace_ref",
            &contract.source_trace_ref,
        )?;
        if let Some(redaction_policy_ref) = contract.redaction_policy_ref.as_deref() {
            validate_public_ref(
                "reasoning_graph.public_contract.redaction_policy_ref",
                redaction_policy_ref,
            )?;
        }
        require_contract_token(
            "reasoning_graph.public_contract.compatibility",
            &contract.compatibility,
            &["legacy_compatible", "migrated", "unsupported"],
        )?;
        if contract.private_reasoning_exposed {
            anyhow::bail!("reasoning_graph public contract must not expose private reasoning");
        }
    }
    reject_private_reasoning_text(
        "reasoning_graph.selected_path.graph_derived_output",
        &artifact.graph.selected_path.graph_derived_output,
    )?;
    for node in &artifact.graph.nodes {
        reject_private_reasoning_text("reasoning_graph.nodes.rationale", &node.rationale)?;
    }
    for edge in &artifact.graph.edges {
        reject_private_reasoning_text("reasoning_graph.edges.rationale", &edge.rationale)?;
    }
    for delegation in &artifact.upstream_delegations {
        validate_upstream_delegation_trace_record(delegation)?;
    }
    Ok(())
}

pub(crate) fn validate_upstream_delegation_trace_record(
    record: &UpstreamDelegationTraceRecord,
) -> anyhow::Result<()> {
    if record.schema_version != UPSTREAM_DELEGATION_TRACE_RECORD_SCHEMA_VERSION {
        anyhow::bail!(
            "upstream_delegation.schema_version must be {}",
            UPSTREAM_DELEGATION_TRACE_RECORD_SCHEMA_VERSION
        );
    }
    for (field, value) in [
        ("delegation_id", record.delegation_id.as_str()),
        ("parent_run_ref", record.parent_run_ref.as_str()),
        ("source_actor_id", record.source_actor_id.as_str()),
        ("source_actor_kind", record.source_actor_kind.as_str()),
        ("source_role_ref", record.source_role_ref.as_str()),
        ("upstream_target_id", record.upstream_target_id.as_str()),
        ("target_class", record.target_class.as_str()),
        ("capability_id", record.capability_id.as_str()),
        ("scope", record.scope.as_str()),
        ("acc_ref", record.acc_ref.as_str()),
        ("grant_ref", record.grant_ref.as_str()),
        ("lifecycle_state", record.lifecycle_state.as_str()),
        ("policy_decision", record.policy_decision.as_str()),
        ("acc_decision", record.acc_decision.as_str()),
        ("grant_status", record.grant_status.as_str()),
        ("public_summary", record.public_summary.as_str()),
    ] {
        if value.trim().is_empty() {
            anyhow::bail!("upstream_delegation.{field} must not be empty");
        }
    }
    require_contract_token(
        "upstream_delegation.target_class",
        &record.target_class,
        &[
            "local_agent",
            "local_service",
            "trusted_external_polis",
            "hosted_provider",
            "remote_runtime",
            "human_operator",
        ],
    )?;
    require_contract_token(
        "upstream_delegation.lifecycle_state",
        &record.lifecycle_state,
        &[
            "requested",
            "policy_evaluated",
            "approved",
            "denied",
            "dispatched",
            "result_received",
            "completed",
            "failed",
            "revoked",
            "blocked",
        ],
    )?;
    require_contract_token(
        "upstream_delegation.policy_decision",
        &record.policy_decision,
        &["allowed", "denied", "needs_approval"],
    )?;
    require_contract_token(
        "upstream_delegation.acc_decision",
        &record.acc_decision,
        &["allowed", "denied", "delegated", "revoked"],
    )?;
    require_contract_token(
        "upstream_delegation.grant_status",
        &record.grant_status,
        &["active", "denied", "delegated", "revoked"],
    )?;
    if record.max_depth == 0 || record.max_depth > 8 {
        anyhow::bail!("upstream_delegation.max_depth must be between 1 and 8");
    }
    if record.authority_basis_refs.is_empty() {
        anyhow::bail!("upstream_delegation.authority_basis_refs must not be empty");
    }
    if record.decision_source_refs.is_empty() {
        anyhow::bail!("upstream_delegation.decision_source_refs must not be empty");
    }
    if !record.parent_responsibility_retained {
        anyhow::bail!("upstream_delegation.parent_responsibility_retained must remain true");
    }
    if !record.parent_review_required {
        anyhow::bail!("upstream_delegation.parent_review_required must remain true");
    }
    if record.parent_authority_inherited {
        anyhow::bail!("upstream_delegation.parent_authority_inherited must remain false");
    }
    if record.private_reasoning_exposed {
        anyhow::bail!("upstream_delegation.private_reasoning_exposed must remain false");
    }
    if record.secrets_exposed {
        anyhow::bail!("upstream_delegation.secrets_exposed must remain false");
    }
    for (field, value) in [
        (
            "upstream_delegation.delegation_id",
            record.delegation_id.as_str(),
        ),
        (
            "upstream_delegation.parent_run_ref",
            record.parent_run_ref.as_str(),
        ),
        (
            "upstream_delegation.source_actor_id",
            record.source_actor_id.as_str(),
        ),
        (
            "upstream_delegation.source_actor_kind",
            record.source_actor_kind.as_str(),
        ),
        (
            "upstream_delegation.source_role_ref",
            record.source_role_ref.as_str(),
        ),
        (
            "upstream_delegation.upstream_target_id",
            record.upstream_target_id.as_str(),
        ),
        (
            "upstream_delegation.capability_id",
            record.capability_id.as_str(),
        ),
        ("upstream_delegation.scope", record.scope.as_str()),
        ("upstream_delegation.acc_ref", record.acc_ref.as_str()),
        ("upstream_delegation.grant_ref", record.grant_ref.as_str()),
        (
            "upstream_delegation.public_summary",
            record.public_summary.as_str(),
        ),
    ] {
        validate_public_ref_or_token(field, value)?;
        reject_private_reasoning_text(field, value)?;
    }
    if let Some(value) = record.provider_or_runtime_ref.as_deref() {
        validate_public_ref_or_token("upstream_delegation.provider_or_runtime_ref", value)?;
        reject_private_reasoning_text("upstream_delegation.provider_or_runtime_ref", value)?;
    }
    for (field, values) in [
        ("upstream_delegation.deliverables", &record.deliverables),
        (
            "upstream_delegation.forbidden_actions",
            &record.forbidden_actions,
        ),
        (
            "upstream_delegation.inherited_constraints",
            &record.inherited_constraints,
        ),
        (
            "upstream_delegation.trace_requirements",
            &record.trace_requirements,
        ),
        (
            "upstream_delegation.authority_basis_refs",
            &record.authority_basis_refs,
        ),
        (
            "upstream_delegation.delegation_chain_refs",
            &record.delegation_chain_refs,
        ),
        (
            "upstream_delegation.decision_source_refs",
            &record.decision_source_refs,
        ),
    ] {
        for value in values {
            validate_public_ref_or_token(field, value)?;
            reject_private_reasoning_text(field, value)?;
        }
    }
    if let Some(value) = record.failure_code.as_deref() {
        validate_public_ref_or_token("upstream_delegation.failure_code", value)?;
    }
    if let Some(value) = record.reasoning_graph_ref.as_deref() {
        validate_public_ref("upstream_delegation.reasoning_graph_ref", value)?;
    }
    if let Some(value) = record.delegated_output_ref.as_deref() {
        validate_public_ref("upstream_delegation.delegated_output_ref", value)?;
    }
    if let Some(value) = record.parent_integration_ref.as_deref() {
        validate_public_ref("upstream_delegation.parent_integration_ref", value)?;
    }
    Ok(())
}

fn require_contract_token(field: &str, value: &str, allowed: &[&str]) -> anyhow::Result<()> {
    if !allowed.contains(&value) {
        anyhow::bail!("{field} must be one of: {}", allowed.join(", "));
    }
    Ok(())
}

fn validate_public_ref(field: &str, value: &str) -> anyhow::Result<()> {
    if value.trim().is_empty() {
        anyhow::bail!("{field} must not be empty");
    }
    if value.starts_with('/') || value.contains("/Users/") || value.contains("/home/") {
        anyhow::bail!("{field} must not contain an absolute host path");
    }
    if value.contains("..") {
        anyhow::bail!("{field} must not contain parent-directory traversal");
    }
    Ok(())
}

fn validate_public_ref_or_token(field: &str, value: &str) -> anyhow::Result<()> {
    validate_public_ref(field, value)?;
    if value.contains('{') || value.contains('}') {
        anyhow::bail!("{field} must not contain raw structured payload text");
    }
    Ok(())
}

fn reject_private_reasoning_text(field: &str, value: &str) -> anyhow::Result<()> {
    let lower = value.to_ascii_lowercase();
    if lower.contains("private chain-of-thought")
        || lower.contains("hidden chain-of-thought")
        || lower.contains("hidden scratchpad")
        || lower.contains("begin private key")
        || lower.contains("sk-")
        || lower.contains("/users/")
        || lower.contains("/home/")
    {
        anyhow::bail!("{field} must not expose private reasoning, secrets, or host paths");
    }
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ReasoningGraphRecord {
    pub(crate) graph_id: String,
    pub(crate) dominant_affect_mode: String,
    pub(crate) ranking_rule: String,
    pub(crate) selected_path: ReasoningGraphSelection,
    pub(crate) nodes: Vec<ReasoningGraphNode>,
    pub(crate) edges: Vec<ReasoningGraphEdge>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ReasoningGraphSelection {
    pub(crate) selected_node_id: String,
    pub(crate) selected_intent: String,
    pub(crate) selected_target: String,
    pub(crate) graph_derived_output: String,
    pub(crate) affect_changed_ranking: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ReasoningGraphNode {
    pub(crate) node_id: String,
    pub(crate) node_kind: String,
    pub(crate) label: String,
    pub(crate) rank: u32,
    pub(crate) priority_score: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) affect_mode: Option<String>,
    pub(crate) rationale: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ReasoningGraphEdge {
    pub(crate) edge_id: String,
    pub(crate) from: String,
    pub(crate) to: String,
    pub(crate) relation: String,
    pub(crate) rationale: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct AeeDecisionRecord {
    pub(crate) decision_id: String,
    pub(crate) decision_kind: String,
    pub(crate) selected_suggestion_id: String,
    pub(crate) category: String,
    pub(crate) intent: String,
    pub(crate) target: String,
    pub(crate) rationale: String,
    pub(crate) expected_downstream_effect: String,
    pub(crate) deterministic_selection_rule: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) recommended_retry_budget: Option<u32>,
    pub(crate) evidence: SuggestionEvidence,
}
