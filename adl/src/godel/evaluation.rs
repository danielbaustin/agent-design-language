//! Evaluation contracts used by the Gödel experiment loop.
use std::fs;
use std::path::{Path, PathBuf};

use jsonschema::JSONSchema;
use serde::{Deserialize, Serialize};

use super::hypothesis::HypothesisCandidate;
use super::mutation::MutationProposal;

pub const CANONICAL_EVALUATION_PLAN_SCHEMA_NAME: &str = "evaluation_plan";
pub const CANONICAL_EVALUATION_PLAN_SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EvaluationDecision {
    Adopt,
    Reject,
    Review,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvaluationOutcome {
    pub decision: EvaluationDecision,
    pub rationale: String,
    pub score_delta: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CanonicalEvaluationPlan {
    pub schema_name: String,
    pub schema_version: u32,
    pub plan_id: String,
    pub experiment_id: String,
    pub baseline_run_id: String,
    pub candidate_run_id: String,
    pub mutation_ref: CanonicalEvaluationPlanMutationRef,
    pub evidence_inputs: Vec<CanonicalEvaluationPlanEvidenceInput>,
    pub metrics: Vec<CanonicalEvaluationMetric>,
    pub decision_rules: Vec<CanonicalEvaluationDecisionRule>,
    pub outcome_model: CanonicalEvaluationOutcomeModel,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experiment_policy: Option<CanonicalExperimentPolicy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<CanonicalEvaluationMetadata>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanonicalEvaluationPlanMutationRef {
    pub mutation_id: String,
    pub schema_name: String,
    pub schema_version: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanonicalEvaluationPlanEvidenceInput {
    pub evidence_id: String,
    pub source_artifact: String,
    pub evidence_view_ref: CanonicalEvidenceViewRef,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selectors: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanonicalEvidenceViewRef {
    pub schema_name: String,
    pub schema_version: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CanonicalEvaluationMetric {
    pub metric_id: String,
    pub metric_type: String,
    pub direction: String,
    pub aggregation: String,
    pub weight: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CanonicalEvaluationDecisionRule {
    pub rule_id: String,
    pub kind: String,
    pub precedence: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_id: Option<String>,
    pub operator: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_value: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_fail: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanonicalEvaluationOutcomeModel {
    pub decision_order: Vec<String>,
    pub default_decision: String,
    pub tie_break: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CanonicalExperimentPolicy {
    pub max_hypotheses_per_failure: u32,
    pub max_parallel_experiments: u32,
    pub max_experiments_per_hypothesis: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admission_thresholds: Option<CanonicalAdmissionThresholds>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CanonicalAdmissionThresholds {
    pub min_evidence_count: u32,
    pub min_confidence_score: f64,
    pub require_policy_approval: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanonicalEvaluationMetadata {
    pub tags: Vec<String>,
    pub created_by: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EvaluationPlanError {
    Invalid(String),
    Io(String),
    Serialize(String),
}

impl std::fmt::Display for EvaluationPlanError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Invalid(msg) => write!(f, "GODEL_EVALUATION_PLAN_INVALID: {msg}"),
            Self::Io(msg) => write!(f, "GODEL_EVALUATION_PLAN_IO: {msg}"),
            Self::Serialize(msg) => write!(f, "GODEL_EVALUATION_PLAN_SERIALIZE: {msg}"),
        }
    }
}

impl std::error::Error for EvaluationPlanError {}

pub fn evaluate_experiment(
    failure_code: &str,
    experiment_result: &str,
    score_delta: i32,
) -> EvaluationOutcome {
    let decision = if score_delta > 0 {
        EvaluationDecision::Adopt
    } else if experiment_result == "blocked" {
        EvaluationDecision::Review
    } else {
        EvaluationDecision::Reject
    };

    EvaluationOutcome {
        decision,
        rationale: format!(
            "Evaluation for failure_code={failure_code} produced decision={:?} with score_delta={score_delta}.",
            decision
        ),
        score_delta,
    }
}

pub fn build_canonical_evaluation_plan(
    run_id: &str,
    workflow_id: &str,
    failure_code: &str,
    evidence_refs: &[String],
    hypothesis: &HypothesisCandidate,
    proposal: &MutationProposal,
) -> Result<CanonicalEvaluationPlan, EvaluationPlanError> {
    if run_id.trim().is_empty() || workflow_id.trim().is_empty() || failure_code.trim().is_empty() {
        return Err(EvaluationPlanError::Invalid(
            "run_id, workflow_id, and failure_code must be non-empty".to_string(),
        ));
    }

    let plan_id = format!(
        "plan_{}_{}",
        sanitize_identifier(run_id),
        sanitize_identifier(failure_code)
    );
    let experiment_id = format!(
        "exp_{}_{}",
        sanitize_identifier(run_id),
        sanitize_identifier(workflow_id)
    );
    let baseline_run_id = format!("{run_id}:baseline");
    let candidate_run_id = format!("{run_id}:candidate");

    let mut normalized_refs = evidence_refs.to_vec();
    normalized_refs.sort();
    normalized_refs.dedup();

    let evidence_inputs = build_evidence_inputs(run_id, &normalized_refs);
    let metrics = build_metrics(failure_code);
    let decision_rules = build_decision_rules();

    let mut tags = vec![
        "deterministic".to_string(),
        "godel".to_string(),
        "v0.8".to_string(),
        sanitize_identifier(&proposal.target_surface),
    ];
    tags.sort();
    tags.dedup();

    let plan = CanonicalEvaluationPlan {
        schema_name: CANONICAL_EVALUATION_PLAN_SCHEMA_NAME.to_string(),
        schema_version: CANONICAL_EVALUATION_PLAN_SCHEMA_VERSION,
        plan_id,
        experiment_id,
        baseline_run_id,
        candidate_run_id,
        mutation_ref: CanonicalEvaluationPlanMutationRef {
            mutation_id: format!("mut_{}", sanitize_identifier(&proposal.id)),
            schema_name: "mutation".to_string(),
            schema_version: 1,
        },
        evidence_inputs,
        metrics,
        decision_rules,
        outcome_model: CanonicalEvaluationOutcomeModel {
            decision_order: vec![
                "adopt".to_string(),
                "requires_human_review".to_string(),
                "reject".to_string(),
            ],
            default_decision: "requires_human_review".to_string(),
            tie_break: "higher_weighted_score".to_string(),
        },
        experiment_policy: Some(CanonicalExperimentPolicy {
            max_hypotheses_per_failure: 3,
            max_parallel_experiments: 1,
            max_experiments_per_hypothesis: 1,
            admission_thresholds: Some(CanonicalAdmissionThresholds {
                min_evidence_count: normalized_refs.len().max(1) as u32,
                min_confidence_score: confidence_score_for_failure(failure_code),
                require_policy_approval: proposal.target_surface == "delegation-policy-input",
            }),
        }),
        notes: Some(format!(
            "Bounded evaluation plan for workflow_id={workflow_id} hypothesis_id={}.",
            hypothesis.id
        )),
        metadata: Some(CanonicalEvaluationMetadata {
            tags,
            created_by: "godel.evaluation.planner".to_string(),
        }),
    };

    validate_canonical_evaluation_plan(&plan)?;
    Ok(plan)
}

pub fn persist_canonical_evaluation_plan(
    runs_root: &Path,
    run_id: &str,
    plan: &CanonicalEvaluationPlan,
) -> Result<PathBuf, EvaluationPlanError> {
    validate_canonical_evaluation_plan(plan)?;
    let rel_path = PathBuf::from("runs")
        .join(run_id)
        .join("godel")
        .join("evaluation_plan.v1.json");
    let out_dir = runs_root.join(run_id).join("godel");
    fs::create_dir_all(&out_dir)
        .map_err(|err| EvaluationPlanError::Io(format!("create dir failed: {err}")))?;
    let json = serde_json::to_string_pretty(plan)
        .map_err(|err| EvaluationPlanError::Serialize(err.to_string()))?;
    fs::write(out_dir.join("evaluation_plan.v1.json"), json)
        .map_err(|err| EvaluationPlanError::Io(format!("write failed: {err}")))?;
    Ok(rel_path)
}

pub fn load_canonical_evaluation_plan(
    path: &Path,
) -> Result<CanonicalEvaluationPlan, EvaluationPlanError> {
    let raw = fs::read_to_string(path)
        .map_err(|err| EvaluationPlanError::Io(format!("read failed: {err}")))?;
    let parsed: CanonicalEvaluationPlan = serde_json::from_str(&raw)
        .map_err(|err| EvaluationPlanError::Invalid(format!("parse failed: {err}")))?;
    validate_canonical_evaluation_plan(&parsed)?;
    Ok(parsed)
}

pub fn validate_canonical_evaluation_plan(
    plan: &CanonicalEvaluationPlan,
) -> Result<(), EvaluationPlanError> {
    let repo_root = repo_root_from_manifest()?;
    let schema_path = repo_root
        .join("adl-spec")
        .join("schemas")
        .join("v0.8")
        .join("evaluation_plan.v1.json");
    let schema_json: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(&schema_path)
            .map_err(|err| EvaluationPlanError::Io(format!("read schema failed: {err}")))?,
    )
    .map_err(|err| EvaluationPlanError::Invalid(format!("parse schema failed: {err}")))?;
    let compiled = JSONSchema::options()
        .compile(&schema_json)
        .map_err(|err| EvaluationPlanError::Invalid(format!("compile schema failed: {err}")))?;
    let value = serde_json::to_value(plan)
        .map_err(|err| EvaluationPlanError::Serialize(err.to_string()))?;
    if let Err(errors) = compiled.validate(&value) {
        let first = errors
            .into_iter()
            .next()
            .map(|err| err.to_string())
            .unwrap_or_else(|| "unknown schema validation failure".to_string());
        return Err(EvaluationPlanError::Invalid(format!(
            "canonical schema validation failed: {first}"
        )));
    }

    require_sorted_unique_by_key(
        &plan
            .evidence_inputs
            .iter()
            .map(|item| item.evidence_id.clone())
            .collect::<Vec<_>>(),
        "evidence_inputs must be sorted by evidence_id and unique",
    )?;
    require_sorted_unique_by_key(
        &plan
            .metrics
            .iter()
            .map(|metric| metric.metric_id.clone())
            .collect::<Vec<_>>(),
        "metrics must be sorted by metric_id and unique",
    )?;
    validate_decision_rule_order(&plan.decision_rules)?;
    require_unique(
        &plan.outcome_model.decision_order,
        "outcome_model.decision_order must be unique and stable",
    )?;
    if let Some(metadata) = &plan.metadata {
        require_sorted_unique_by_key(&metadata.tags, "metadata.tags must be sorted and unique")?;
    }
    if let Some(notes) = &plan.notes {
        reject_unsafe_text(notes, "notes")?;
    }
    for input in &plan.evidence_inputs {
        reject_unsafe_text(&input.source_artifact, "source_artifact")?;
        if let Some(selectors) = &input.selectors {
            require_sorted_unique_by_key(selectors, "selectors must be sorted and unique")?;
        }
    }

    Ok(())
}

fn build_evidence_inputs(
    run_id: &str,
    evidence_refs: &[String],
) -> Vec<CanonicalEvaluationPlanEvidenceInput> {
    let mut inputs = evidence_refs
        .iter()
        .enumerate()
        .map(|(idx, evidence_ref)| CanonicalEvaluationPlanEvidenceInput {
            evidence_id: format!("ev_{}_{}", sanitize_identifier(run_id), idx + 1),
            source_artifact: format!("tmp/{evidence_ref}"),
            evidence_view_ref: CanonicalEvidenceViewRef {
                schema_name: "canonical_evidence_view".to_string(),
                schema_version: 1,
            },
            selectors: Some(vec![
                "artifact_hashes".to_string(),
                "failure_codes".to_string(),
                "verification_results".to_string(),
            ]),
        })
        .collect::<Vec<_>>();
    inputs.sort_by(|a, b| a.evidence_id.cmp(&b.evidence_id));
    inputs
}

fn build_metrics(failure_code: &str) -> Vec<CanonicalEvaluationMetric> {
    let mut metrics = vec![
        CanonicalEvaluationMetric {
            metric_id: "met_failure_rate_delta".to_string(),
            metric_type: "failure_rate_delta".to_string(),
            direction: "decrease_is_better".to_string(),
            aggregation: "latest".to_string(),
            weight: 0.5,
            threshold: Some(if failure_code == "transient_failure" {
                -0.10
            } else {
                -0.05
            }),
        },
        CanonicalEvaluationMetric {
            metric_id: "met_replay_match".to_string(),
            metric_type: "deterministic_replay_match".to_string(),
            direction: "target_match".to_string(),
            aggregation: "boolean_all".to_string(),
            weight: 0.3,
            threshold: Some(1.0),
        },
        CanonicalEvaluationMetric {
            metric_id: "met_policy_violations".to_string(),
            metric_type: "policy_violation_count".to_string(),
            direction: "decrease_is_better".to_string(),
            aggregation: "sum".to_string(),
            weight: 0.2,
            threshold: Some(0.0),
        },
    ];
    metrics.sort_by(|a, b| a.metric_id.cmp(&b.metric_id));
    metrics
}

fn build_decision_rules() -> Vec<CanonicalEvaluationDecisionRule> {
    vec![
        CanonicalEvaluationDecisionRule {
            rule_id: "rule_hard_fail_policy_violations".to_string(),
            kind: "hard_fail_if".to_string(),
            precedence: 10,
            metric_id: Some("met_policy_violations".to_string()),
            operator: ">".to_string(),
            target_value: Some(serde_json::json!(0)),
            on_fail: Some("reject".to_string()),
        },
        CanonicalEvaluationDecisionRule {
            rule_id: "rule_threshold_replay_match".to_string(),
            kind: "threshold_gate".to_string(),
            precedence: 20,
            metric_id: Some("met_replay_match".to_string()),
            operator: "==".to_string(),
            target_value: Some(serde_json::json!(1)),
            on_fail: Some("requires_human_review".to_string()),
        },
        CanonicalEvaluationDecisionRule {
            rule_id: "rule_weighted_score_gate".to_string(),
            kind: "weighted_score_gate".to_string(),
            precedence: 30,
            metric_id: None,
            operator: ">=".to_string(),
            target_value: Some(serde_json::json!(0.7)),
            on_fail: Some("reject".to_string()),
        },
    ]
}

fn validate_decision_rule_order(
    rules: &[CanonicalEvaluationDecisionRule],
) -> Result<(), EvaluationPlanError> {
    let mut ordered = rules.to_vec();
    ordered.sort_by(|a, b| {
        a.precedence
            .cmp(&b.precedence)
            .then(a.rule_id.cmp(&b.rule_id))
    });
    if rules != ordered {
        return Err(EvaluationPlanError::Invalid(
            "decision_rules must be sorted by precedence then rule_id".to_string(),
        ));
    }
    Ok(())
}

fn require_sorted_unique_by_key(values: &[String], msg: &str) -> Result<(), EvaluationPlanError> {
    let mut sorted = values.to_vec();
    sorted.sort();
    sorted.dedup();
    if values != sorted {
        return Err(EvaluationPlanError::Invalid(msg.to_string()));
    }
    Ok(())
}

fn require_unique(values: &[String], msg: &str) -> Result<(), EvaluationPlanError> {
    let mut sorted = values.to_vec();
    sorted.sort();
    sorted.dedup();
    if values.len() != sorted.len() {
        return Err(EvaluationPlanError::Invalid(msg.to_string()));
    }
    Ok(())
}

fn reject_unsafe_text(value: &str, field: &str) -> Result<(), EvaluationPlanError> {
    if value.starts_with('/') || value.contains("..") || value.contains('\\') {
        return Err(EvaluationPlanError::Invalid(format!(
            "{field} must not contain absolute or traversal paths"
        )));
    }
    if value.len() > 1 && value.as_bytes()[1] == b':' {
        return Err(EvaluationPlanError::Invalid(format!(
            "{field} must not contain drive-prefixed paths"
        )));
    }
    Ok(())
}

fn confidence_score_for_failure(failure_code: &str) -> f64 {
    if failure_code.contains("policy") {
        0.7
    } else {
        0.6
    }
}

fn sanitize_identifier(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for ch in value.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch.to_ascii_lowercase());
        } else {
            out.push('_');
        }
    }
    let trimmed = out.trim_matches('_');
    if trimmed.is_empty() {
        "evaluation".to_string()
    } else {
        trimmed.chars().take(96).collect()
    }
}

fn repo_root_from_manifest() -> Result<PathBuf, EvaluationPlanError> {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
    let Some(repo_root) = manifest.parent() else {
        return Err(EvaluationPlanError::Invalid(
            "unable to derive repository root from CARGO_MANIFEST_DIR".to_string(),
        ));
    };
    Ok(repo_root.to_path_buf())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture_hypothesis() -> HypothesisCandidate {
        HypothesisCandidate {
            id: "hyp:run-1:tool_failure:00".to_string(),
            statement: "retrying reduces flaky failures".to_string(),
            failure_code: "tool_failure".to_string(),
            evidence_refs: vec!["runs/run-1/run_status.json".to_string()],
        }
    }

    fn fixture_proposal() -> MutationProposal {
        MutationProposal {
            id: "mut:run-1:tool_failure:00".to_string(),
            hypothesis_id: "hyp:run-1:tool_failure:00".to_string(),
            target_surface: "tool-invocation-config".to_string(),
            bounded_change: "Apply deterministic bounded retry adjustment.".to_string(),
        }
    }

    fn test_tmp_dir(label: &str) -> PathBuf {
        let root = std::env::temp_dir().join(format!(
            "adl-godel-eval-plan-{label}-{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&root).expect("mkdir test tmp");
        root
    }

    #[test]
    fn evaluate_experiment_selects_expected_decisions() {
        assert_eq!(
            evaluate_experiment("tool_failure", "ok", 1).decision,
            EvaluationDecision::Adopt
        );
        assert_eq!(
            evaluate_experiment("tool_failure", "ok", 0).decision,
            EvaluationDecision::Reject
        );
        assert_eq!(
            evaluate_experiment("tool_failure", "blocked", 0).decision,
            EvaluationDecision::Review
        );
    }

    #[test]
    fn canonical_evaluation_plan_round_trip_validates_against_schema() {
        let tmp = test_tmp_dir("round-trip");
        let plan = build_canonical_evaluation_plan(
            "run-1",
            "wf-godel-loop",
            "tool_failure",
            &["runs/run-1/run_status.json".to_string()],
            &fixture_hypothesis(),
            &fixture_proposal(),
        )
        .expect("build plan");
        let rel = persist_canonical_evaluation_plan(&tmp, "run-1", &plan).expect("persist plan");
        assert_eq!(
            rel,
            PathBuf::from("runs/run-1/godel/evaluation_plan.v1.json")
        );
        let loaded =
            load_canonical_evaluation_plan(&tmp.join("run-1/godel/evaluation_plan.v1.json"))
                .expect("load plan");
        assert_eq!(loaded.schema_name, "evaluation_plan");
        assert_eq!(loaded.metrics.len(), 3);
        assert_eq!(loaded.decision_rules.len(), 3);
        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn canonical_evaluation_plan_rejects_unsorted_metric_and_rule_order() {
        let mut plan = build_canonical_evaluation_plan(
            "run-1",
            "wf-godel-loop",
            "tool_failure",
            &["runs/run-1/run_status.json".to_string()],
            &fixture_hypothesis(),
            &fixture_proposal(),
        )
        .expect("build plan");
        plan.metrics.swap(0, 2);
        let err =
            validate_canonical_evaluation_plan(&plan).expect_err("unsorted metrics must fail");
        assert!(err.to_string().contains("metrics must be sorted"));

        plan.metrics.sort_by(|a, b| a.metric_id.cmp(&b.metric_id));
        plan.decision_rules.swap(0, 1);
        let err = validate_canonical_evaluation_plan(&plan).expect_err("unsorted rules must fail");
        assert!(err.to_string().contains("decision_rules must be sorted"));
    }
}
