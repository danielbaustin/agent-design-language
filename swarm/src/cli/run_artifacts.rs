use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::path::{Path, PathBuf};

use ::adl::{artifacts, execute, failure_taxonomy, instrumentation, resolve, trace};

pub(crate) const RUN_STATE_SCHEMA_VERSION: &str = "run_state.v1";
pub(crate) const PAUSE_STATE_SCHEMA_VERSION: &str = "pause_state.v1";
pub(crate) const RUN_STATUS_VERSION: u32 = 1;
pub(crate) const RUN_SUMMARY_VERSION: u32 = 1;
pub(crate) const SCORES_VERSION: u32 = 1;
pub(crate) const SUGGESTIONS_VERSION: u32 = 1;
pub(crate) const AEE_DECISION_VERSION: u32 = 1;
pub(crate) const CLUSTER_GROUNDWORK_VERSION: u32 = 1;

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct RunStateArtifact {
    pub(crate) schema_version: String,
    pub(crate) run_id: String,
    pub(crate) workflow_id: String,
    pub(crate) version: String,
    pub(crate) status: String,
    pub(crate) error_message: Option<String>,
    pub(crate) start_time_ms: u128,
    pub(crate) end_time_ms: u128,
    pub(crate) duration_ms: u128,
    pub(crate) execution_plan_hash: String,
    #[serde(default)]
    pub(crate) scheduler_max_concurrency: Option<usize>,
    #[serde(default)]
    pub(crate) scheduler_policy_source: Option<String>,
    #[serde(default)]
    pub(crate) steering_history: Vec<execute::SteeringRecord>,
    pub(crate) pause: Option<execute::PauseState>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PauseStateArtifact {
    pub(crate) schema_version: String,
    pub(crate) run_id: String,
    pub(crate) workflow_id: String,
    pub(crate) version: String,
    pub(crate) status: String,
    pub(crate) adl_path: String,
    pub(crate) execution_plan_hash: String,
    #[serde(default)]
    pub(crate) steering_history: Vec<execute::SteeringRecord>,
    pub(crate) pause: execute::PauseState,
}

#[derive(Debug, Serialize)]
pub(crate) struct StepStateArtifact {
    pub(crate) step_id: String,
    pub(crate) agent_id: String,
    pub(crate) provider_id: String,
    pub(crate) status: String,
    pub(crate) output_artifact_path: Option<String>,
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
    pub(crate) evidence: SuggestionEvidence,
}

pub(crate) fn stable_fingerprint_hex(bytes: &[u8]) -> String {
    let mut hash: u64 = 0xcbf29ce484222325;
    for b in bytes {
        hash ^= u64::from(*b);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("{hash:016x}")
}

pub(crate) fn execution_plan_hash<T: Serialize>(plan: &T) -> Result<String> {
    let plan_json = serde_json::to_vec(plan).context("serialize execution plan for hashing")?;
    Ok(stable_fingerprint_hex(&plan_json))
}

pub(crate) fn classify_failure_kind(err: &anyhow::Error) -> Option<&'static str> {
    failure_taxonomy::classify(err)
}

pub(crate) fn build_run_summary(
    resolved: &resolve::AdlResolved,
    status: &str,
    pause: Option<&execute::PauseState>,
    steps: &[StepStateArtifact],
    records: usize,
    failure: Option<&anyhow::Error>,
    run_paths: &artifacts::RunArtifactPaths,
) -> RunSummaryArtifact {
    let failed_steps = steps.iter().filter(|s| s.status == "failure").count();
    let completed_steps = steps
        .iter()
        .filter(|s| s.status == "success" || s.status == "failure")
        .count();
    let delegation_steps = resolved
        .steps
        .iter()
        .filter(|s| {
            s.delegation
                .as_ref()
                .map(|d| !d.is_effectively_empty())
                .unwrap_or(false)
        })
        .count();
    let delegation_requires_verification_steps = resolved
        .steps
        .iter()
        .filter(|s| {
            s.delegation
                .as_ref()
                .and_then(|d| d.requires_verification)
                .unwrap_or(false)
        })
        .count();
    let mut security_denials_by_code = BTreeMap::new();
    if let Some(code) = failure.and_then(classify_failure_kind) {
        *security_denials_by_code
            .entry(code.to_string())
            .or_insert(0) += 1;
    }

    let (
        security_envelope_enabled,
        signing_required,
        key_id_required,
        mut allowed_algs,
        mut allowed_key_sources,
    ) = if let Some(remote) = resolved.doc.run.remote.as_ref() {
        (
            true,
            remote.require_signed_requests,
            remote.require_key_id,
            remote.verify_allowed_algs.clone(),
            remote.verify_allowed_key_sources.clone(),
        )
    } else {
        (false, false, false, Vec::new(), Vec::new())
    };
    allowed_algs.sort();
    allowed_algs.dedup();
    allowed_key_sources.sort();
    allowed_key_sources.dedup();
    let scores_rel = run_paths
        .scores_json()
        .strip_prefix(run_paths.run_dir())
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| "learning/scores.json".to_string());
    let suggestions_rel = run_paths
        .suggestions_json()
        .strip_prefix(run_paths.run_dir())
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| "learning/suggestions.json".to_string());
    let aee_decision_rel = run_paths
        .aee_decision_json()
        .strip_prefix(run_paths.run_dir())
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| "learning/aee_decision.json".to_string());
    let cluster_groundwork_rel = run_paths
        .cluster_groundwork_json()
        .strip_prefix(run_paths.run_dir())
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| "meta/cluster_groundwork.json".to_string());

    RunSummaryArtifact {
        run_summary_version: RUN_SUMMARY_VERSION,
        artifact_model_version: artifacts::ARTIFACT_MODEL_VERSION,
        run_id: resolved.run_id.clone(),
        workflow_id: resolved.workflow_id.clone(),
        adl_version: resolved.doc.version.clone(),
        swarm_version: env!("CARGO_PKG_VERSION").to_string(),
        status: status.to_string(),
        error_kind: failure.and_then(classify_failure_kind).map(str::to_string),
        counts: RunSummaryCounts {
            total_steps: resolved.steps.len(),
            completed_steps,
            failed_steps,
            provider_call_count: records,
            delegation_steps,
            delegation_requires_verification_steps,
        },
        policy: RunSummaryPolicy {
            security_envelope_enabled,
            signing_required,
            key_id_required,
            verify_allowed_algs: allowed_algs,
            verify_allowed_key_sources: allowed_key_sources,
            sandbox_policy: "centralized_path_resolver_v1".to_string(),
            security_denials_by_code,
        },
        links: RunSummaryLinks {
            run_json: "run.json".to_string(),
            steps_json: "steps.json".to_string(),
            pause_state_json: pause.map(|_| "pause_state.json".to_string()),
            outputs_dir: run_paths
                .outputs_dir()
                .strip_prefix(run_paths.run_dir())
                .map(|p| p.display().to_string())
                .unwrap_or_else(|_| "outputs".to_string()),
            logs_dir: run_paths
                .logs_dir()
                .strip_prefix(run_paths.run_dir())
                .map(|p| p.display().to_string())
                .unwrap_or_else(|_| "logs".to_string()),
            learning_dir: run_paths
                .learning_dir()
                .strip_prefix(run_paths.run_dir())
                .map(|p| p.display().to_string())
                .unwrap_or_else(|_| "learning".to_string()),
            scores_json: Some(scores_rel),
            suggestions_json: Some(suggestions_rel),
            aee_decision_json: Some(aee_decision_rel),
            overlays_dir: run_paths
                .overlays_dir()
                .strip_prefix(run_paths.run_dir())
                .map(|p| p.display().to_string())
                .unwrap_or_else(|_| "learning/overlays".to_string()),
            cluster_groundwork_json: run_paths
                .cluster_groundwork_json()
                .is_file()
                .then_some(cluster_groundwork_rel),
            trace_json: None,
        },
    }
}

pub(crate) fn build_cluster_groundwork_artifact(
    resolved: &resolve::AdlResolved,
    steps: &[StepStateArtifact],
    tr: &trace::Trace,
) -> ClusterGroundworkArtifact {
    let mut remaining_deps: BTreeMap<String, BTreeSet<String>> = resolved
        .execution_plan
        .nodes
        .iter()
        .map(|node| {
            (
                node.step_id.clone(),
                node.depends_on.iter().cloned().collect::<BTreeSet<_>>(),
            )
        })
        .collect();
    let mut remaining_nodes: BTreeSet<String> =
        remaining_deps.keys().cloned().collect::<BTreeSet<_>>();
    let mut readiness_frontiers = Vec::new();
    while !remaining_nodes.is_empty() {
        let ready = remaining_nodes
            .iter()
            .filter(|step_id| {
                remaining_deps
                    .get(step_id.as_str())
                    .map(|deps| deps.is_empty())
                    .unwrap_or(false)
            })
            .cloned()
            .collect::<Vec<_>>();
        if ready.is_empty() {
            break;
        }
        readiness_frontiers.push(ClusterReadyFrontier {
            frontier_index: readiness_frontiers.len() as u32,
            ready_step_ids: ready.clone(),
        });
        for step_id in &ready {
            remaining_nodes.remove(step_id);
        }
        for deps in remaining_deps.values_mut() {
            for step_id in &ready {
                deps.remove(step_id);
            }
        }
    }

    let mut attempts_by_step: BTreeMap<String, u32> = BTreeMap::new();
    for event in &tr.events {
        if let trace::TraceEvent::StepStarted { step_id, .. } = event {
            *attempts_by_step.entry(step_id.clone()).or_insert(0) += 1;
        }
    }
    let status_by_step = steps
        .iter()
        .map(|step| (step.step_id.clone(), step.status.clone()))
        .collect::<BTreeMap<_, _>>();
    let depends_on_by_step = resolved
        .execution_plan
        .nodes
        .iter()
        .map(|node| (node.step_id.clone(), node.depends_on.clone()))
        .collect::<BTreeMap<_, _>>();

    let mut lease_records = Vec::new();
    let mut issued_sequence: u32 = 0;
    for frontier in &readiness_frontiers {
        for step_id in &frontier.ready_step_ids {
            issued_sequence = issued_sequence.saturating_add(1);
            let status = status_by_step
                .get(step_id)
                .map(|value| value.as_str())
                .unwrap_or("not_run");
            let lease_state = match status {
                "success" => "completed",
                "failure" => "failed",
                _ => "planned",
            };
            lease_records.push(ClusterLeaseRecord {
                issued_sequence,
                lease_id: format!("lease:{}:{}:1", resolved.run_id, step_id),
                step_id: step_id.clone(),
                depends_on: depends_on_by_step.get(step_id).cloned().unwrap_or_default(),
                observed_attempts: attempts_by_step.get(step_id).copied().unwrap_or(0),
                claim_owner: "adl-coordinator-local".to_string(),
                worker_id: "adl-worker-local".to_string(),
                lease_state: lease_state.to_string(),
            });
        }
    }

    ClusterGroundworkArtifact {
        cluster_groundwork_version: CLUSTER_GROUNDWORK_VERSION,
        run_id: resolved.run_id.clone(),
        workflow_id: resolved.workflow_id.clone(),
        coordinator_id: "adl-coordinator-local".to_string(),
        worker_id: "adl-worker-local".to_string(),
        canonical_ordering_key: "(run_id, step_id, attempt)".to_string(),
        frontier_ordering: "topological_frontier_then_step_id".to_string(),
        readiness_frontiers,
        lease_records,
    }
}

pub(crate) fn build_run_status(
    resolved: &resolve::AdlResolved,
    tr: &trace::Trace,
    overall_status: &str,
    steps: &[StepStateArtifact],
    failure: Option<&anyhow::Error>,
    resume_completed_step_ids: &BTreeSet<String>,
) -> RunStatusArtifact {
    let mut completed_steps: BTreeSet<String> = resume_completed_step_ids.clone();
    let mut pending_steps: BTreeSet<String> = BTreeSet::new();
    let mut failed_step_id: Option<String> = None;

    for step in steps {
        match step.status.as_str() {
            "success" => {
                completed_steps.insert(step.step_id.clone());
            }
            "failure" => {
                if failed_step_id.is_none() {
                    failed_step_id = Some(step.step_id.clone());
                }
                pending_steps.insert(step.step_id.clone());
            }
            _ => {
                pending_steps.insert(step.step_id.clone());
            }
        }
    }

    let mut attempts_by_step: BTreeMap<String, u32> = resume_completed_step_ids
        .iter()
        .map(|step_id| (step_id.clone(), 0))
        .collect();
    let mut started_set = BTreeSet::new();
    for event in &tr.events {
        if let trace::TraceEvent::StepStarted { step_id, .. } = event {
            started_set.insert(step_id.clone());
            *attempts_by_step.entry(step_id.clone()).or_insert(0) += 1;
        }
    }

    let scheduler_policy = execute::scheduler_policy_for_run(resolved).ok().flatten();

    RunStatusArtifact {
        run_status_version: RUN_STATUS_VERSION,
        run_id: resolved.run_id.clone(),
        workflow_id: resolved.workflow_id.clone(),
        overall_status: overall_status.to_string(),
        failure_kind: failure.and_then(classify_failure_kind).map(str::to_string),
        failed_step_id,
        completed_steps: completed_steps.into_iter().collect(),
        pending_steps: pending_steps.into_iter().collect(),
        started_steps: if started_set.is_empty() {
            None
        } else {
            Some(started_set.into_iter().collect())
        },
        attempt_counts_by_step: attempts_by_step,
        effective_max_concurrency: scheduler_policy.map(|(value, _)| value),
        effective_max_concurrency_source: scheduler_policy
            .map(|(_, source)| source.as_str().to_string()),
    }
}

pub(crate) fn compute_retry_count(tr: &trace::Trace) -> usize {
    let mut started_by_step: BTreeMap<&str, usize> = BTreeMap::new();
    for event in &tr.events {
        if let trace::TraceEvent::StepStarted { step_id, .. } = event {
            *started_by_step.entry(step_id.as_str()).or_insert(0) += 1;
        }
    }
    started_by_step
        .values()
        .map(|count| count.saturating_sub(1))
        .sum()
}

pub(crate) fn compute_max_parallel_observed(tr: &trace::Trace) -> usize {
    let mut active: BTreeSet<&str> = BTreeSet::new();
    let mut max_parallel = 0usize;
    for event in &tr.events {
        match event {
            trace::TraceEvent::StepStarted { step_id, .. } => {
                active.insert(step_id.as_str());
                max_parallel = max_parallel.max(active.len());
            }
            trace::TraceEvent::StepFinished { step_id, .. } => {
                active.remove(step_id.as_str());
            }
            _ => {}
        }
    }
    max_parallel
}

pub(crate) fn build_scores_artifact(
    run_summary: &RunSummaryArtifact,
    tr: &trace::Trace,
) -> ScoresArtifact {
    let success_steps = run_summary
        .counts
        .completed_steps
        .saturating_sub(run_summary.counts.failed_steps);
    let success_ratio = if run_summary.counts.total_steps == 0 {
        1.0
    } else {
        let permille = (success_steps * 1000) / run_summary.counts.total_steps;
        (permille as f64) / 1000.0
    };
    let security_denied_count: usize = run_summary.policy.security_denials_by_code.values().sum();
    let delegation_denied_count: usize = run_summary
        .policy
        .security_denials_by_code
        .iter()
        .filter_map(|(code, count)| {
            if code.starts_with("DELEGATION_") {
                Some(*count)
            } else {
                None
            }
        })
        .sum();

    ScoresArtifact {
        scores_version: SCORES_VERSION,
        run_id: run_summary.run_id.clone(),
        generated_from: ScoresGeneratedFrom {
            artifact_model_version: run_summary.artifact_model_version,
            run_summary_version: run_summary.run_summary_version,
        },
        summary: ScoresSummary {
            success_ratio,
            failure_count: run_summary.counts.failed_steps,
            retry_count: compute_retry_count(tr),
            delegation_denied_count,
            security_denied_count,
        },
        metrics: ScoresMetrics {
            scheduler_max_parallel_observed: compute_max_parallel_observed(tr),
        },
    }
}

pub(crate) fn read_scores_if_present(
    run_paths: &artifacts::RunArtifactPaths,
) -> Option<ScoresArtifact> {
    let path = run_paths.scores_json();
    let raw = std::fs::read_to_string(path).ok()?;
    serde_json::from_str::<ScoresArtifact>(&raw).ok()
}

pub(crate) fn build_suggestions_artifact(
    run_summary: &RunSummaryArtifact,
    scores: Option<&ScoresArtifact>,
) -> SuggestionsArtifact {
    let fallback_summary;
    let fallback_metrics;
    let (score_summary, score_metrics, score_version) = if let Some(scores) = scores {
        (
            &scores.summary,
            &scores.metrics,
            Some(scores.scores_version),
        )
    } else {
        let failed_steps = run_summary.counts.failed_steps;
        let success_steps = run_summary
            .counts
            .completed_steps
            .saturating_sub(failed_steps);
        let success_ratio = if run_summary.counts.total_steps == 0 {
            1.0
        } else {
            let permille = (success_steps * 1000) / run_summary.counts.total_steps;
            (permille as f64) / 1000.0
        };
        let security_denied_count: usize =
            run_summary.policy.security_denials_by_code.values().sum();
        let delegation_denied_count: usize = run_summary
            .policy
            .security_denials_by_code
            .iter()
            .filter_map(|(code, count)| {
                if code.starts_with("DELEGATION_") {
                    Some(*count)
                } else {
                    None
                }
            })
            .sum();
        fallback_summary = ScoresSummary {
            success_ratio,
            failure_count: failed_steps,
            retry_count: 0,
            delegation_denied_count,
            security_denied_count,
        };
        fallback_metrics = ScoresMetrics {
            scheduler_max_parallel_observed: 1,
        };
        (&fallback_summary, &fallback_metrics, None)
    };

    let base_evidence = SuggestionEvidence {
        failure_count: score_summary.failure_count,
        retry_count: score_summary.retry_count,
        delegation_denied_count: score_summary.delegation_denied_count,
        security_denied_count: score_summary.security_denied_count,
        success_ratio: score_summary.success_ratio,
        scheduler_max_parallel_observed: score_metrics.scheduler_max_parallel_observed,
    };

    let mut suggestions = Vec::new();

    if score_summary.failure_count > 0 {
        suggestions.push(SuggestionItem {
            id: String::new(),
            category: "retry".to_string(),
            severity: "improvement".to_string(),
            rationale: "One or more steps failed; consider safer retry policy for transient paths."
                .to_string(),
            evidence: base_evidence.clone(),
            proposed_change: SuggestedChangeIntent {
                intent: "increase_step_retry_budget".to_string(),
                target: "failed-step-set".to_string(),
            },
        });
    }
    if score_summary.delegation_denied_count > 0 {
        suggestions.push(SuggestionItem {
            id: String::new(),
            category: "delegation".to_string(),
            severity: "warning".to_string(),
            rationale: "Delegation-denied signals detected; review delegation policy scope."
                .to_string(),
            evidence: base_evidence.clone(),
            proposed_change: SuggestedChangeIntent {
                intent: "review_delegation_policy_scope".to_string(),
                target: "delegation-boundary".to_string(),
            },
        });
    }
    if score_summary.security_denied_count > 0 {
        suggestions.push(SuggestionItem {
            id: String::new(),
            category: "security".to_string(),
            severity: "warning".to_string(),
            rationale: "Security denials observed; align expected capabilities with trust policy."
                .to_string(),
            evidence: base_evidence.clone(),
            proposed_change: SuggestedChangeIntent {
                intent: "review_security_policy_expectations".to_string(),
                target: "security-envelope".to_string(),
            },
        });
    }
    if score_summary.success_ratio < 1.0 {
        suggestions.push(SuggestionItem {
            id: String::new(),
            category: "general".to_string(),
            severity: "improvement".to_string(),
            rationale: "Success ratio is below 1.0; review failing steps and dependency shape."
                .to_string(),
            evidence: base_evidence.clone(),
            proposed_change: SuggestedChangeIntent {
                intent: "review_failure_hotspots".to_string(),
                target: "workflow-step-dependencies".to_string(),
            },
        });
    }
    if run_summary.counts.total_steps > 1 && score_metrics.scheduler_max_parallel_observed <= 1 {
        suggestions.push(SuggestionItem {
            id: String::new(),
            category: "scheduler".to_string(),
            severity: "info".to_string(),
            rationale: "Observed parallelism is low; evaluate opportunities for safe concurrency."
                .to_string(),
            evidence: base_evidence,
            proposed_change: SuggestedChangeIntent {
                intent: "evaluate_parallelizable_dependencies".to_string(),
                target: "workflow-structure".to_string(),
            },
        });
    }

    for (idx, suggestion) in suggestions.iter_mut().enumerate() {
        suggestion.id = format!("sug-{:03}", idx + 1);
    }

    SuggestionsArtifact {
        suggestions_version: SUGGESTIONS_VERSION,
        run_id: run_summary.run_id.clone(),
        generated_from: SuggestionsGeneratedFrom {
            artifact_model_version: run_summary.artifact_model_version,
            run_summary_version: run_summary.run_summary_version,
            scores_version: score_version,
        },
        suggestions,
    }
}

fn aee_decision_kind_for_intent(intent: &str) -> (&'static str, &'static str) {
    match intent {
        "increase_step_retry_budget" => (
            "bounded_retry_recovery",
            "raise retry budget for the failed step set on the next bounded run",
        ),
        "evaluate_parallelizable_dependencies" => (
            "bounded_scheduler_review",
            "review whether the workflow can safely increase bounded parallelism",
        ),
        "review_delegation_policy_scope" => (
            "bounded_delegation_review",
            "review delegation boundaries before the next bounded run",
        ),
        "review_security_policy_expectations" => (
            "bounded_security_review",
            "review trust-policy expectations before the next bounded run",
        ),
        "review_failure_hotspots" => (
            "bounded_failure_review",
            "review failing dependency hotspots before the next bounded run",
        ),
        _ => (
            "bounded_runtime_review",
            "review bounded runtime signals before the next run",
        ),
    }
}

pub(crate) fn build_aee_decision_artifact(
    run_summary: &RunSummaryArtifact,
    suggestions: &SuggestionsArtifact,
    scores: Option<&ScoresArtifact>,
) -> AeeDecisionArtifact {
    let selected = suggestions
        .suggestions
        .first()
        .cloned()
        .unwrap_or_else(|| SuggestionItem {
            id: "sug-000".to_string(),
            category: "stability".to_string(),
            severity: "info".to_string(),
            rationale: "No bounded adaptation signals fired; keep current policy state."
                .to_string(),
            evidence: SuggestionEvidence {
                failure_count: 0,
                retry_count: 0,
                delegation_denied_count: 0,
                security_denied_count: 0,
                success_ratio: 1.0,
                scheduler_max_parallel_observed: 1,
            },
            proposed_change: SuggestedChangeIntent {
                intent: "maintain_current_policy".to_string(),
                target: "workflow-runtime".to_string(),
            },
        });
    let (decision_kind, expected_downstream_effect) =
        aee_decision_kind_for_intent(&selected.proposed_change.intent);

    AeeDecisionArtifact {
        aee_decision_version: AEE_DECISION_VERSION,
        run_id: run_summary.run_id.clone(),
        generated_from: AeeDecisionGeneratedFrom {
            artifact_model_version: run_summary.artifact_model_version,
            run_summary_version: run_summary.run_summary_version,
            suggestions_version: suggestions.suggestions_version,
            scores_version: scores.map(|value| value.scores_version),
        },
        decision: AeeDecisionRecord {
            decision_id: "aee-001".to_string(),
            decision_kind: decision_kind.to_string(),
            selected_suggestion_id: selected.id,
            category: selected.category,
            intent: selected.proposed_change.intent,
            target: selected.proposed_change.target,
            rationale: selected.rationale,
            expected_downstream_effect: expected_downstream_effect.to_string(),
            deterministic_selection_rule:
                "select the first suggestion emitted by build_suggestions_artifact after stable category ordering"
                    .to_string(),
            evidence: selected.evidence,
        },
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn write_run_state_artifacts(
    resolved: &resolve::AdlResolved,
    tr: &trace::Trace,
    adl_path: &Path,
    _out_dir: &Path,
    start_ms: u128,
    end_ms: u128,
    status: &str,
    pause: Option<&execute::PauseState>,
    steering_history: &[execute::SteeringRecord],
    resume_completed_step_ids: Option<&HashSet<String>>,
    failure: Option<&anyhow::Error>,
) -> Result<PathBuf> {
    let run_paths = artifacts::RunArtifactPaths::for_run(&resolved.run_id)?;
    run_paths.ensure_layout()?;
    run_paths.write_model_marker()?;
    let run_dir = run_paths.run_dir();
    let resume_completed: BTreeSet<String> = resume_completed_step_ids
        .map(|ids| ids.iter().cloned().collect())
        .unwrap_or_default();

    let mut status_by_step: HashMap<String, String> = HashMap::new();
    for ev in &tr.events {
        if let trace::TraceEvent::StepFinished {
            step_id, success, ..
        } = ev
        {
            let status = if *success { "success" } else { "failure" };
            status_by_step.insert(step_id.clone(), status.to_string());
        }
    }

    let mut steps = Vec::with_capacity(resolved.steps.len());
    for step in &resolved.steps {
        let status = status_by_step
            .get(&step.id)
            .cloned()
            .or_else(|| {
                resume_completed
                    .contains(&step.id)
                    .then(|| "success".to_string())
            })
            .unwrap_or_else(|| "not_run".to_string());
        let output_artifact_path = match (status.as_str(), step.write_to.as_deref()) {
            ("success", Some(write_to)) => Some(write_to.to_string()),
            _ => None,
        };

        let agent_id = step
            .agent
            .as_deref()
            .unwrap_or("<unresolved-agent>")
            .to_string();
        let provider_id = step
            .provider
            .as_deref()
            .unwrap_or("<unresolved-provider>")
            .to_string();

        steps.push(StepStateArtifact {
            step_id: step.id.clone(),
            agent_id,
            provider_id,
            status,
            output_artifact_path,
        });
    }

    let scheduler_policy = execute::scheduler_policy_for_run(resolved)?;
    let error_message = tr.events.iter().rev().find_map(|ev| match ev {
        trace::TraceEvent::RunFailed { message, .. } => Some(message.clone()),
        _ => None,
    });
    let run_artifact = RunStateArtifact {
        schema_version: RUN_STATE_SCHEMA_VERSION.to_string(),
        run_id: resolved.run_id.clone(),
        workflow_id: resolved.workflow_id.clone(),
        version: resolved.doc.version.clone(),
        status: status.to_string(),
        error_message: error_message.clone(),
        start_time_ms: start_ms,
        end_time_ms: end_ms,
        duration_ms: end_ms.saturating_sub(start_ms),
        execution_plan_hash: execution_plan_hash(&resolved.execution_plan)?,
        scheduler_max_concurrency: scheduler_policy.map(|(v, _)| v),
        scheduler_policy_source: scheduler_policy.map(|(_, source)| source.as_str().to_string()),
        steering_history: steering_history.to_vec(),
        pause: pause.cloned(),
    };

    let run_json = serde_json::to_vec_pretty(&run_artifact).context("serialize run.json")?;
    let steps_json = serde_json::to_vec_pretty(&steps).context("serialize steps.json")?;
    let activation_log_path = run_paths.activation_log_json();
    instrumentation::write_trace_artifact(&activation_log_path, &tr.events)?;
    let cluster_groundwork = build_cluster_groundwork_artifact(resolved, &steps, tr);
    let cluster_groundwork_json = serde_json::to_vec_pretty(&cluster_groundwork)
        .context("serialize cluster_groundwork.json")?;
    artifacts::atomic_write(
        &run_paths.cluster_groundwork_json(),
        &cluster_groundwork_json,
    )?;
    let run_summary = build_run_summary(
        resolved,
        status,
        pause,
        &steps,
        tr.events
            .iter()
            .filter(|ev| matches!(ev, trace::TraceEvent::StepFinished { .. }))
            .count(),
        failure,
        &run_paths,
    );
    let overall_status = match status {
        "success" => "succeeded",
        "failure" => "failed",
        "paused" => "running",
        other => other,
    };
    let run_status = build_run_status(
        resolved,
        tr,
        overall_status,
        &steps,
        failure,
        &resume_completed,
    );
    let run_summary_json =
        serde_json::to_vec_pretty(&run_summary).context("serialize run_summary.json")?;
    let run_status_json =
        serde_json::to_vec_pretty(&run_status).context("serialize run_status.json")?;
    let scores = build_scores_artifact(&run_summary, tr);
    let scores_json = serde_json::to_vec_pretty(&scores).context("serialize scores.json")?;
    let scores_for_suggestions = read_scores_if_present(&run_paths).unwrap_or(scores.clone());
    let suggestions = build_suggestions_artifact(&run_summary, Some(&scores_for_suggestions));
    let suggestions_json =
        serde_json::to_vec_pretty(&suggestions).context("serialize suggestions.json")?;
    let aee_decision =
        build_aee_decision_artifact(&run_summary, &suggestions, Some(&scores_for_suggestions));
    let aee_decision_json =
        serde_json::to_vec_pretty(&aee_decision).context("serialize aee_decision.json")?;

    artifacts::atomic_write(&run_paths.run_json(), &run_json)?;
    artifacts::atomic_write(&run_paths.steps_json(), &steps_json)?;
    artifacts::atomic_write(&run_paths.run_status_json(), &run_status_json)?;
    artifacts::atomic_write(&run_paths.run_summary_json(), &run_summary_json)?;
    artifacts::atomic_write(&run_paths.scores_json(), &scores_json)?;
    artifacts::atomic_write(&run_paths.suggestions_json(), &suggestions_json)?;
    artifacts::atomic_write(&run_paths.aee_decision_json(), &aee_decision_json)?;
    if let Some(pause_payload) = pause {
        let pause_artifact = PauseStateArtifact {
            schema_version: PAUSE_STATE_SCHEMA_VERSION.to_string(),
            run_id: resolved.run_id.clone(),
            workflow_id: resolved.workflow_id.clone(),
            version: resolved.doc.version.clone(),
            status: "paused".to_string(),
            adl_path: adl_path.display().to_string(),
            execution_plan_hash: execution_plan_hash(&resolved.execution_plan)?,
            steering_history: steering_history.to_vec(),
            pause: pause_payload.clone(),
        };
        let pause_json =
            serde_json::to_vec_pretty(&pause_artifact).context("serialize pause_state.json")?;
        artifacts::atomic_write(&run_paths.pause_state_json(), &pause_json)?;
    }

    Ok(run_dir)
}

pub(crate) fn load_resume_state(
    path: &Path,
    resolved: &resolve::AdlResolved,
) -> Result<execute::ResumeState> {
    let raw = std::fs::read_to_string(path)
        .with_context(|| format!("failed to read resume state '{}'", path.display()))?;
    let artifact: RunStateArtifact = serde_json::from_str(&raw).with_context(|| {
        format!(
            "failed to parse resume state '{}' as run_state artifact",
            path.display()
        )
    })?;

    if artifact.schema_version != RUN_STATE_SCHEMA_VERSION {
        return Err(anyhow::anyhow!(
            "resume state schema_version mismatch: state='{}' expected='{}'",
            artifact.schema_version,
            RUN_STATE_SCHEMA_VERSION
        ));
    }

    if artifact.schema_version != RUN_STATE_SCHEMA_VERSION {
        return Err(anyhow::anyhow!(
            "resume state schema_version mismatch: state='{}' expected='{}'",
            artifact.schema_version,
            RUN_STATE_SCHEMA_VERSION
        ));
    }

    if artifact.status != "paused" {
        return Err(anyhow::anyhow!(
            "resume state must have status='paused' (found='{}' for run_id='{}' in '{}')",
            artifact.status,
            artifact.run_id,
            path.display()
        ));
    }
    if artifact.run_id != resolved.run_id {
        return Err(anyhow::anyhow!(
            "resume run_id mismatch: state='{}' current='{}'",
            artifact.run_id,
            resolved.run_id
        ));
    }
    if artifact.workflow_id != resolved.workflow_id {
        return Err(anyhow::anyhow!(
            "resume workflow_id mismatch for run_id='{}' in '{}': state='{}' current='{}'",
            artifact.run_id,
            path.display(),
            artifact.workflow_id,
            resolved.workflow_id
        ));
    }
    if artifact.version != resolved.doc.version {
        return Err(anyhow::anyhow!(
            "resume version mismatch for run_id='{}' in '{}': state='{}' current='{}'",
            artifact.run_id,
            path.display(),
            artifact.version,
            resolved.doc.version
        ));
    }
    let plan_hash = execution_plan_hash(&resolved.execution_plan)?;
    if artifact.execution_plan_hash != plan_hash {
        return Err(anyhow::anyhow!(
            "resume execution plan mismatch for run_id='{}' in '{}'; state plan != current plan (resume requires identical plan + ordering)",
            artifact.run_id,
            path.display()
        ));
    }
    let pause = artifact
        .pause
        .ok_or_else(|| anyhow::anyhow!("resume state missing pause payload"))?;

    let completed_step_ids = pause.completed_step_ids.into_iter().collect();
    Ok(execute::ResumeState {
        completed_step_ids,
        saved_state: pause.saved_state,
        completed_outputs: pause.completed_outputs,
        steering_history: artifact.steering_history,
    })
}

pub(crate) fn resume_state_path_for_run_id(run_id: &str) -> Result<PathBuf> {
    Ok(artifacts::RunArtifactPaths::for_run(run_id)?.pause_state_json())
}

pub(crate) fn load_pause_state_artifact(path: &Path) -> Result<PauseStateArtifact> {
    let raw = std::fs::read_to_string(path)
        .with_context(|| format!("failed to read pause state '{}'", path.display()))?;
    let artifact: PauseStateArtifact =
        serde_json::from_str(&raw).with_context(|| "failed to parse pause_state.json")?;
    Ok(artifact)
}

pub(crate) fn load_steering_patch(path: &Path) -> Result<(execute::SteeringPatch, String)> {
    let raw = std::fs::read(path)
        .with_context(|| format!("failed to read steering patch '{}'", path.display()))?;
    let fingerprint = stable_fingerprint_hex(&raw);
    let patch: execute::SteeringPatch =
        serde_json::from_slice(&raw).with_context(|| "failed to parse steering patch JSON")?;
    execute::validate_steering_patch(&patch)?;
    Ok((patch, fingerprint))
}

pub(crate) fn validate_pause_artifact_basic(
    artifact: &PauseStateArtifact,
    run_id: &str,
) -> Result<()> {
    if artifact.schema_version != PAUSE_STATE_SCHEMA_VERSION {
        return Err(anyhow::anyhow!(
            "pause state schema_version mismatch: state='{}' expected='{}'",
            artifact.schema_version,
            PAUSE_STATE_SCHEMA_VERSION
        ));
    }
    if artifact.status != "paused" {
        return Err(anyhow::anyhow!(
            "pause state must have status='paused' (found '{}')",
            artifact.status
        ));
    }
    if artifact.run_id != run_id {
        return Err(anyhow::anyhow!(
            "pause state run_id mismatch: state='{}' requested='{}'",
            artifact.run_id,
            run_id
        ));
    }
    Ok(())
}

pub(crate) fn validate_pause_artifact_for_resume(
    artifact: &PauseStateArtifact,
    run_id: &str,
    resolved: &resolve::AdlResolved,
) -> Result<()> {
    validate_pause_artifact_basic(artifact, run_id)?;
    if artifact.run_id != resolved.run_id {
        return Err(anyhow::anyhow!(
            "resume run_id mismatch: state='{}' current='{}'",
            artifact.run_id,
            resolved.run_id
        ));
    }
    if artifact.workflow_id != resolved.workflow_id {
        return Err(anyhow::anyhow!(
            "resume workflow_id mismatch: state='{}' current='{}'",
            artifact.workflow_id,
            resolved.workflow_id
        ));
    }
    if artifact.version != resolved.doc.version {
        return Err(anyhow::anyhow!(
            "resume version mismatch: state='{}' current='{}'",
            artifact.version,
            resolved.doc.version
        ));
    }
    let plan_hash = execution_plan_hash(&resolved.execution_plan)?;
    if artifact.execution_plan_hash != plan_hash {
        return Err(anyhow::anyhow!(
            "resume execution plan hash mismatch; resume requires identical plan and ordering"
        ));
    }
    Ok(())
}
