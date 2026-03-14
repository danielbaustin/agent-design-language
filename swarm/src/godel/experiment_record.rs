use std::fs;
use std::path::{Path, PathBuf};

use jsonschema::JSONSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};

use super::evaluation::EvaluationOutcome;
use super::mutation::MutationProposal;

pub const EXPERIMENT_RECORD_RUNTIME_SCHEMA: &str = "experiment_record.runtime.v1";
pub const EXPERIMENT_RECORD_CANONICAL_SCHEMA_NAME: &str = "experiment_record";
pub const EXPERIMENT_RECORD_CANONICAL_SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StageExperimentRecord {
    pub run_id: String,
    pub workflow_id: String,
    pub failure_code: String,
    pub hypothesis_id: String,
    pub mutation_id: String,
    pub mutation_target_surface: String,
    pub evaluation_decision: String,
    pub evaluation_rationale: String,
    pub improvement_delta: i32,
    pub evidence_refs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PersistedExperimentRecord {
    pub schema: String,
    pub record: StageExperimentRecord,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CanonicalExperimentRecord {
    pub schema_name: String,
    pub schema_version: u32,
    pub experiment_id: String,
    pub experiment_seed: String,
    pub comparison_key: CanonicalComparisonKey,
    pub runs: CanonicalRuns,
    pub hypothesis: CanonicalHypothesis,
    pub mutation: CanonicalMutation,
    pub evaluation_plan: CanonicalEvaluationPlan,
    pub evidence: CanonicalEvidence,
    pub outcome: CanonicalOutcome,
    pub improvement_delta: CanonicalImprovementDelta,
    pub decision: CanonicalDecision,
    pub replay: CanonicalReplay,
    pub obsmem_index: CanonicalObsMemIndex,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanonicalComparisonKey {
    pub subject: String,
    pub baseline_label: String,
    pub variant_label: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanonicalRuns {
    pub baseline_run_id: String,
    pub variant_run_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanonicalHypothesis {
    pub statement: String,
    pub expected_effect: String,
    pub risk_class: String,
    pub success_criteria: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanonicalMutation {
    pub mutation_id: String,
    pub mutation_version: u32,
    pub mutation_ref: String,
    pub scope: Vec<String>,
    pub change_summary: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanonicalEvaluationPlan {
    pub evaluation_plan_id: String,
    pub evaluation_plan_version: u32,
    pub evaluation_plan_ref: String,
    pub check_ids: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CanonicalEvidence {
    pub canonical_evidence_view_id: String,
    pub canonical_evidence_view_version: u32,
    pub canonical_evidence_ref: String,
    pub evidence_items: Vec<CanonicalEvidenceItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CanonicalEvidenceItem {
    pub evidence_id: String,
    pub kind: String,
    pub value: Value,
    pub source_ref: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CanonicalOutcome {
    pub primary_metric: String,
    pub direction: String,
    pub baseline_value: f64,
    pub variant_value: f64,
    pub delta: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CanonicalImprovementDelta {
    pub metric: String,
    pub baseline: f64,
    pub experiment: f64,
    pub delta: f64,
    pub direction: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanonicalDecision {
    pub result: String,
    pub rationale: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanonicalReplay {
    pub replay_profile: String,
    pub artifact_manifest: Vec<CanonicalReplayArtifact>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanonicalReplayArtifact {
    pub path: String,
    pub sha256: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanonicalObsMemIndex {
    pub memory_kind: String,
    pub index_key: String,
    pub tags: Vec<String>,
    pub facets: CanonicalObsMemFacets,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanonicalObsMemFacets {
    pub decision_result: String,
    pub primary_metric: String,
    pub subject: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExperimentRecordError {
    Invalid(String),
    Io(String),
    Serialize(String),
}

impl std::fmt::Display for ExperimentRecordError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Invalid(msg) => write!(f, "GODEL_EXPERIMENT_RECORD_INVALID: {msg}"),
            Self::Io(msg) => write!(f, "GODEL_EXPERIMENT_RECORD_IO: {msg}"),
            Self::Serialize(msg) => write!(f, "GODEL_EXPERIMENT_RECORD_SERIALIZE: {msg}"),
        }
    }
}

impl std::error::Error for ExperimentRecordError {}

pub fn build_record(
    run_id: &str,
    workflow_id: &str,
    failure_code: &str,
    evidence_refs: &[String],
    mutation: &MutationProposal,
    evaluation: &EvaluationOutcome,
) -> StageExperimentRecord {
    let mut refs = evidence_refs.to_vec();
    refs.sort();
    refs.dedup();

    StageExperimentRecord {
        run_id: run_id.to_string(),
        workflow_id: workflow_id.to_string(),
        failure_code: failure_code.to_string(),
        hypothesis_id: mutation.hypothesis_id.clone(),
        mutation_id: mutation.id.clone(),
        mutation_target_surface: mutation.target_surface.clone(),
        evaluation_decision: format!("{:?}", evaluation.decision).to_lowercase(),
        evaluation_rationale: evaluation.rationale.clone(),
        improvement_delta: evaluation.score_delta,
        evidence_refs: refs,
    }
}

pub fn persist_record(
    runs_root: &Path,
    record: &StageExperimentRecord,
) -> Result<PathBuf, ExperimentRecordError> {
    validate_record(record)?;

    let rel_path = PathBuf::from("runs")
        .join(&record.run_id)
        .join("godel")
        .join("experiment_record.runtime.v1.json");
    let out_path = runs_root.join(&record.run_id).join("godel");
    fs::create_dir_all(&out_path)
        .map_err(|err| ExperimentRecordError::Io(format!("create dir failed: {err}")))?;

    let persisted = PersistedExperimentRecord {
        schema: EXPERIMENT_RECORD_RUNTIME_SCHEMA.to_string(),
        record: record.clone(),
    };
    let json = serde_json::to_string_pretty(&persisted)
        .map_err(|err| ExperimentRecordError::Serialize(err.to_string()))?;

    fs::write(out_path.join("experiment_record.runtime.v1.json"), json)
        .map_err(|err| ExperimentRecordError::Io(format!("write failed: {err}")))?;

    Ok(rel_path)
}

pub fn build_canonical_record(
    runs_root: &Path,
    record: &StageExperimentRecord,
    runtime_record_rel_path: &Path,
    index_rel_path: &Path,
) -> Result<CanonicalExperimentRecord, ExperimentRecordError> {
    validate_record(record)?;

    let repo_root = repo_root_from_manifest()?;
    let mutation_ref = "adl-spec/examples/v0.8/mutation.v1.example.json".to_string();
    let evaluation_plan_ref = "adl-spec/examples/v0.8/evaluation_plan.v1.example.json".to_string();
    let evidence_ref = "adl-spec/examples/v0.8/canonical_evidence_view.v1.example.json".to_string();

    let manifest_paths = [
        mutation_ref.as_str(),
        evaluation_plan_ref.as_str(),
        evidence_ref.as_str(),
        runtime_record_rel_path.to_str().ok_or_else(|| {
            ExperimentRecordError::Invalid("runtime record path must be valid utf-8".to_string())
        })?,
        index_rel_path.to_str().ok_or_else(|| {
            ExperimentRecordError::Invalid("index path must be valid utf-8".to_string())
        })?,
    ];

    let artifact_manifest = manifest_paths
        .iter()
        .map(|path| {
            let absolute = if path.starts_with("runs/") {
                let suffix = path.strip_prefix("runs/").ok_or_else(|| {
                    ExperimentRecordError::Invalid(
                        "runtime artifact path must start with runs/".to_string(),
                    )
                })?;
                runs_root.join(suffix)
            } else {
                repo_root.join(path)
            };
            Ok(CanonicalReplayArtifact {
                path: (*path).to_string(),
                sha256: sha256_file(&absolute)?,
            })
        })
        .collect::<Result<Vec<_>, ExperimentRecordError>>()?;

    let mut evidence_items = vec![CanonicalEvidenceItem {
        evidence_id: format!("failure-code-{}", sanitize_identifier(&record.failure_code)),
        kind: "failure_code".to_string(),
        value: Value::String(record.failure_code.clone()),
        source_ref: runtime_record_rel_path.display().to_string(),
    }];
    for (idx, evidence_ref_path) in record.evidence_refs.iter().enumerate() {
        evidence_items.push(CanonicalEvidenceItem {
            evidence_id: format!("evidence-ref-{:02}", idx),
            kind: "trace".to_string(),
            value: Value::String(evidence_ref_path.clone()),
            source_ref: evidence_ref_path.clone(),
        });
    }
    evidence_items.sort_by(|a, b| a.evidence_id.cmp(&b.evidence_id));

    let improvement = f64::from(record.improvement_delta);
    let experiment_id = format!(
        "exp-{}-{}",
        sanitize_identifier(&record.run_id),
        sanitize_identifier(&record.mutation_id)
    );
    let experiment_seed = format!(
        "seed:{}:{}:{}",
        record.run_id, record.failure_code, record.mutation_id
    );
    let decision_result = canonical_decision_result(&record.evaluation_decision);

    let canonical = CanonicalExperimentRecord {
        schema_name: EXPERIMENT_RECORD_CANONICAL_SCHEMA_NAME.to_string(),
        schema_version: EXPERIMENT_RECORD_CANONICAL_SCHEMA_VERSION,
        experiment_id,
        experiment_seed,
        comparison_key: CanonicalComparisonKey {
            subject: record.mutation_target_surface.clone(),
            baseline_label: "bounded-baseline".to_string(),
            variant_label: "bounded-variant".to_string(),
        },
        runs: CanonicalRuns {
            baseline_run_id: record.run_id.clone(),
            variant_run_id: record.run_id.clone(),
        },
        hypothesis: CanonicalHypothesis {
            statement: format!(
                "Bounded Gödel hypothesis for failure_code={} on {}.",
                record.failure_code, record.mutation_target_surface
            ),
            expected_effect: "improvement_delta_positive".to_string(),
            risk_class: "low".to_string(),
            success_criteria: vec![
                "deterministic_record_persisted".to_string(),
                "deterministic_index_entry_persisted".to_string(),
                "improvement_delta_non_negative".to_string(),
            ],
        },
        mutation: CanonicalMutation {
            mutation_id: record.mutation_id.clone(),
            mutation_version: 1,
            mutation_ref,
            scope: vec![record.mutation_target_surface.clone()],
            change_summary: vec![format!(
                "bounded mutation selected for {}",
                record.mutation_target_surface
            )],
        },
        evaluation_plan: CanonicalEvaluationPlan {
            evaluation_plan_id: format!("evalplan-{}", sanitize_identifier(&record.run_id)),
            evaluation_plan_version: 1,
            evaluation_plan_ref,
            check_ids: vec![
                "godel_stage_loop".to_string(),
                "experiment_record_runtime_validation".to_string(),
                "obsmem_index_persistence".to_string(),
            ],
        },
        evidence: CanonicalEvidence {
            canonical_evidence_view_id: format!("cev-{}", sanitize_identifier(&record.run_id)),
            canonical_evidence_view_version: 1,
            canonical_evidence_ref: evidence_ref,
            evidence_items,
        },
        outcome: CanonicalOutcome {
            primary_metric: "improvement_delta".to_string(),
            direction: "increase_is_better".to_string(),
            baseline_value: 0.0,
            variant_value: improvement,
            delta: improvement,
        },
        improvement_delta: CanonicalImprovementDelta {
            metric: "improvement_delta".to_string(),
            baseline: 0.0,
            experiment: improvement,
            delta: improvement,
            direction: "increase_is_better".to_string(),
        },
        decision: CanonicalDecision {
            result: decision_result.clone(),
            rationale: record.evaluation_rationale.clone(),
        },
        replay: CanonicalReplay {
            replay_profile: "strict".to_string(),
            artifact_manifest,
        },
        obsmem_index: CanonicalObsMemIndex {
            memory_kind: "experiment_record".to_string(),
            index_key: format!(
                "{}::{}::bounded-baseline::bounded-variant",
                sanitize_identifier(&record.run_id),
                sanitize_identifier(&record.mutation_target_surface)
            ),
            tags: canonical_tags(&record.failure_code, &decision_result),
            facets: CanonicalObsMemFacets {
                decision_result,
                primary_metric: "improvement_delta".to_string(),
                subject: record.mutation_target_surface.clone(),
            },
        },
        notes: Some(vec![
            "Bounded runtime integration bridges the current single-run Gödel loop to the canonical experiment_record.v1 contract.".to_string(),
            "Mutation, evaluation_plan, and canonical_evidence references point to canonical v0.8 example artifacts until broader runtime emission exists.".to_string(),
        ]),
    };
    validate_canonical_record(&canonical)?;
    Ok(canonical)
}

pub fn persist_canonical_record(
    runs_root: &Path,
    canonical: &CanonicalExperimentRecord,
) -> Result<PathBuf, ExperimentRecordError> {
    validate_canonical_record(canonical)?;

    let rel_path = PathBuf::from("runs")
        .join(canonical.runs.variant_run_id.as_str())
        .join("godel")
        .join("experiment_record.v1.json");
    let out_path = runs_root
        .join(canonical.runs.variant_run_id.as_str())
        .join("godel");
    fs::create_dir_all(&out_path)
        .map_err(|err| ExperimentRecordError::Io(format!("create dir failed: {err}")))?;

    let json = serde_json::to_string_pretty(canonical)
        .map_err(|err| ExperimentRecordError::Serialize(err.to_string()))?;
    fs::write(out_path.join("experiment_record.v1.json"), json)
        .map_err(|err| ExperimentRecordError::Io(format!("write failed: {err}")))?;

    Ok(rel_path)
}

pub fn load_canonical_record(
    path: &Path,
) -> Result<CanonicalExperimentRecord, ExperimentRecordError> {
    let raw = fs::read_to_string(path)
        .map_err(|err| ExperimentRecordError::Io(format!("read failed: {err}")))?;
    let parsed: CanonicalExperimentRecord = serde_json::from_str(&raw)
        .map_err(|err| ExperimentRecordError::Invalid(format!("parse failed: {err}")))?;
    validate_canonical_record(&parsed)?;
    Ok(parsed)
}

pub fn validate_canonical_record(
    canonical: &CanonicalExperimentRecord,
) -> Result<(), ExperimentRecordError> {
    let repo_root = repo_root_from_manifest()?;
    let schema_path = repo_root
        .join("adl-spec")
        .join("schemas")
        .join("v0.8")
        .join("experiment_record.v1.schema.json");
    let schema_json: Value = serde_json::from_str(
        &fs::read_to_string(&schema_path)
            .map_err(|err| ExperimentRecordError::Io(format!("read schema failed: {err}")))?,
    )
    .map_err(|err| ExperimentRecordError::Invalid(format!("parse schema failed: {err}")))?;
    let compiled = JSONSchema::options()
        .compile(&schema_json)
        .map_err(|err| ExperimentRecordError::Invalid(format!("compile schema failed: {err}")))?;
    let value = serde_json::to_value(canonical)
        .map_err(|err| ExperimentRecordError::Serialize(err.to_string()))?;
    if let Err(errors) = compiled.validate(&value) {
        let first = errors
            .into_iter()
            .next()
            .map(|err| err.to_string())
            .unwrap_or_else(|| "unknown schema validation failure".to_string());
        return Err(ExperimentRecordError::Invalid(format!(
            "canonical schema validation failed: {first}"
        )));
    }
    Ok(())
}

fn validate_record(record: &StageExperimentRecord) -> Result<(), ExperimentRecordError> {
    if record.run_id.trim().is_empty()
        || record.workflow_id.trim().is_empty()
        || record.failure_code.trim().is_empty()
        || record.hypothesis_id.trim().is_empty()
        || record.mutation_id.trim().is_empty()
    {
        return Err(ExperimentRecordError::Invalid(
            "required id fields must be non-empty".to_string(),
        ));
    }

    for path in &record.evidence_refs {
        if path.trim().is_empty()
            || path.starts_with('/')
            || path.contains("..")
            || path.contains(':')
            || path.contains('\\')
        {
            return Err(ExperimentRecordError::Invalid(format!(
                "invalid evidence ref path: {path}"
            )));
        }
    }

    let mut content_scan = format!(
        "{}\n{}\n{}\n{}",
        record.failure_code,
        record.evaluation_rationale,
        record.mutation_target_surface,
        record.evidence_refs.join("\n")
    );
    content_scan.push('\n');
    content_scan.push_str(&record.hypothesis_id);

    if content_scan.contains("/Users/")
        || content_scan.contains("/home/")
        || content_scan.contains("gho_")
        || content_scan.contains("sk-")
    {
        return Err(ExperimentRecordError::Invalid(
            "record contains disallowed host-path or token-like content".to_string(),
        ));
    }

    Ok(())
}

fn repo_root_from_manifest() -> Result<PathBuf, ExperimentRecordError> {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
    let Some(repo_root) = manifest.parent() else {
        return Err(ExperimentRecordError::Invalid(
            "unable to derive repository root from CARGO_MANIFEST_DIR".to_string(),
        ));
    };
    Ok(repo_root.to_path_buf())
}

fn sanitize_identifier(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for ch in value.chars() {
        if ch.is_ascii_alphanumeric() || matches!(ch, '.' | '_' | '-') {
            out.push(ch.to_ascii_lowercase());
        } else {
            out.push('-');
        }
    }
    let trimmed = out.trim_matches('-');
    if trimmed.is_empty() {
        "record".to_string()
    } else {
        trimmed.chars().take(96).collect()
    }
}

fn canonical_decision_result(decision: &str) -> String {
    match decision {
        "adopt" => "adopt",
        "reject" => "reject",
        _ => "requires_human_review",
    }
    .to_string()
}

fn canonical_tags(failure_code: &str, decision_result: &str) -> Vec<String> {
    let mut tags = vec![
        "godel".to_string(),
        "v0.8".to_string(),
        sanitize_identifier(failure_code),
        sanitize_identifier(decision_result),
    ];
    tags.sort();
    tags.dedup();
    tags
}

fn sha256_file(path: &Path) -> Result<String, ExperimentRecordError> {
    let bytes =
        fs::read(path).map_err(|err| ExperimentRecordError::Io(format!("read failed: {err}")))?;
    let digest = Sha256::digest(bytes);
    Ok(format!("{digest:x}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::godel::evaluation::{EvaluationDecision, EvaluationOutcome};
    use crate::godel::mutation::MutationProposal;
    use std::fs;
    use std::path::PathBuf;

    fn test_tmp_dir(label: &str) -> PathBuf {
        let root = std::env::temp_dir().join(format!("adl-godel-{label}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&root).expect("mkdir test tmp");
        root
    }

    #[test]
    fn build_record_uses_deterministic_field_mapping() {
        let m = MutationProposal {
            id: "mut:r1:tool_failure".to_string(),
            hypothesis_id: "hyp:r1:tool_failure".to_string(),
            target_surface: "workflow-step-config".to_string(),
            bounded_change: "y".to_string(),
        };
        let e = EvaluationOutcome {
            decision: EvaluationDecision::Adopt,
            rationale: "ok".to_string(),
            score_delta: 2,
        };
        let r = build_record(
            "r1",
            "wf1",
            "tool_failure",
            &["runs/r1/run_status.json".to_string()],
            &m,
            &e,
        );
        assert_eq!(r.evaluation_decision, "adopt");
        assert_eq!(r.improvement_delta, 2);
        assert_eq!(r.failure_code, "tool_failure");
    }

    #[test]
    fn persist_record_writes_expected_runtime_path() {
        let tmp = test_tmp_dir("record-persist");
        let record = StageExperimentRecord {
            run_id: "run-746-a".to_string(),
            workflow_id: "wf-godel".to_string(),
            failure_code: "tool_failure".to_string(),
            hypothesis_id: "hyp:run-746-a:tool_failure".to_string(),
            mutation_id: "mut:run-746-a:tool_failure".to_string(),
            mutation_target_surface: "workflow-step-config".to_string(),
            evaluation_decision: "adopt".to_string(),
            evaluation_rationale: "deterministic rationale".to_string(),
            improvement_delta: 1,
            evidence_refs: vec!["runs/run-746-a/run_status.json".to_string()],
        };

        let rel = persist_record(&tmp, &record).expect("persist");
        assert_eq!(
            rel,
            PathBuf::from("runs/run-746-a/godel/experiment_record.runtime.v1.json")
        );

        let raw = fs::read_to_string(
            tmp.join("run-746-a")
                .join("godel")
                .join("experiment_record.runtime.v1.json"),
        )
        .expect("read file");
        let persisted: PersistedExperimentRecord = serde_json::from_str(&raw).expect("json");
        assert_eq!(persisted.schema, EXPERIMENT_RECORD_RUNTIME_SCHEMA);
        assert_eq!(persisted.record, record);
        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn persist_record_rejects_unsafe_evidence_ref() {
        let tmp = test_tmp_dir("record-invalid");
        let record = StageExperimentRecord {
            run_id: "run-746-a".to_string(),
            workflow_id: "wf-godel".to_string(),
            failure_code: "tool_failure".to_string(),
            hypothesis_id: "hyp:run-746-a:tool_failure".to_string(),
            mutation_id: "mut:run-746-a:tool_failure".to_string(),
            mutation_target_surface: "workflow-step-config".to_string(),
            evaluation_decision: "adopt".to_string(),
            evaluation_rationale: "deterministic rationale".to_string(),
            improvement_delta: 1,
            evidence_refs: vec!["/Users/daniel/secret.json".to_string()],
        };

        let err = persist_record(&tmp, &record).expect_err("must fail");
        assert!(err.to_string().contains("GODEL_EXPERIMENT_RECORD_INVALID"));
        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn canonical_record_round_trip_validates_against_schema() {
        let tmp = test_tmp_dir("record-canonical");
        let record = StageExperimentRecord {
            run_id: "run-746-a".to_string(),
            workflow_id: "wf-godel".to_string(),
            failure_code: "tool_failure".to_string(),
            hypothesis_id: "hyp:run-746-a:tool_failure".to_string(),
            mutation_id: "mut:run-746-a:tool_failure".to_string(),
            mutation_target_surface: "workflow-step-config".to_string(),
            evaluation_decision: "adopt".to_string(),
            evaluation_rationale: "deterministic rationale".to_string(),
            improvement_delta: 1,
            evidence_refs: vec!["runs/run-746-a/run_status.json".to_string()],
        };

        let runtime_rel = persist_record(&tmp, &record).expect("persist runtime record");
        let index_rel = PathBuf::from("runs")
            .join("run-746-a")
            .join("godel")
            .join("obsmem_index_entry.runtime.v1.json");
        let index_abs = tmp
            .join("run-746-a")
            .join("godel")
            .join("obsmem_index_entry.runtime.v1.json");
        fs::write(
            &index_abs,
            r#"{"schema":"godel_obsmem_index.runtime.v1","entry":{"index_key":"tool_failure:hyp:run-746-a:tool_failure:adopt","run_id":"run-746-a","workflow_id":"wf-godel","failure_code":"tool_failure","hypothesis_id":"hyp:run-746-a:tool_failure","mutation_id":"mut:run-746-a:tool_failure","experiment_outcome":"adopt"}}"#,
        )
        .expect("write index");

        let canonical = build_canonical_record(&tmp, &record, &runtime_rel, &index_rel)
            .expect("build canonical record");
        let canonical_rel =
            persist_canonical_record(&tmp, &canonical).expect("persist canonical record");
        assert_eq!(
            canonical_rel,
            PathBuf::from("runs/run-746-a/godel/experiment_record.v1.json")
        );

        let loaded = load_canonical_record(
            &tmp.join("run-746-a")
                .join("godel")
                .join("experiment_record.v1.json"),
        )
        .expect("load canonical record");
        assert_eq!(loaded.schema_name, "experiment_record");
        assert_eq!(loaded.schema_version, 1);
        assert_eq!(loaded.runs.variant_run_id, "run-746-a");
        assert_eq!(loaded.improvement_delta.delta, 1.0);
        assert!(loaded
            .replay
            .artifact_manifest
            .iter()
            .all(|entry| entry.sha256.len() == 64));

        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn load_canonical_record_rejects_schema_mismatch() {
        let tmp = test_tmp_dir("record-canonical-invalid");
        let path = tmp.join("experiment_record.v1.json");
        fs::write(
            &path,
            r#"{"schema_name":"wrong","schema_version":1,"experiment_id":"exp-a","experiment_seed":"seed:a","comparison_key":{"subject":"x","baseline_label":"b","variant_label":"v"},"runs":{"baseline_run_id":"r1","variant_run_id":"r1"},"hypothesis":{"statement":"s","expected_effect":"e"},"mutation":{"mutation_id":"m1","mutation_version":1,"mutation_ref":"adl-spec/examples/v0.8/mutation.v1.example.json","scope":["x"]},"evaluation_plan":{"evaluation_plan_id":"p1","evaluation_plan_version":1,"evaluation_plan_ref":"adl-spec/examples/v0.8/evaluation_plan.v1.example.json","check_ids":["c1"]},"evidence":{"canonical_evidence_view_id":"e1","canonical_evidence_view_version":1,"canonical_evidence_ref":"adl-spec/examples/v0.8/canonical_evidence_view.v1.example.json","evidence_items":[{"evidence_id":"i1","kind":"trace","value":"runs/r1/run_status.json","source_ref":"runs/r1/run_status.json"}]},"outcome":{"primary_metric":"improvement_delta","direction":"increase_is_better","baseline_value":0,"variant_value":1,"delta":1},"improvement_delta":{"metric":"improvement_delta","baseline":0,"experiment":1,"delta":1,"direction":"increase_is_better"},"decision":{"result":"adopt","rationale":"ok"},"replay":{"replay_profile":"strict","artifact_manifest":[{"path":"adl-spec/examples/v0.8/mutation.v1.example.json","sha256":"1111111111111111111111111111111111111111111111111111111111111111"}]},"obsmem_index":{"memory_kind":"experiment_record","index_key":"exp-a::x","tags":["adopt","godel"],"facets":{"decision_result":"adopt","primary_metric":"improvement_delta","subject":"x"}}}"#,
        )
        .expect("write invalid canonical record");

        let err = load_canonical_record(&path).expect_err("schema mismatch must fail");
        assert!(err.to_string().contains("GODEL_EXPERIMENT_RECORD_INVALID"));
        let _ = std::fs::remove_dir_all(&tmp);
    }
}
