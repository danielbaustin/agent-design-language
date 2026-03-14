use std::fs;
use std::path::{Path, PathBuf};

use jsonschema::JSONSchema;
use serde::{Deserialize, Serialize};

use super::stage_loop::StageLoopInput;

pub const CANONICAL_EVIDENCE_SCHEMA_NAME: &str = "canonical_evidence_view";
pub const CANONICAL_EVIDENCE_SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CanonicalEvidenceView {
    pub schema_name: String,
    pub schema_version: u32,
    pub evidence_view_id: String,
    pub run_context: CanonicalEvidenceRunContext,
    pub canonicalization_profile: CanonicalizationProfile,
    pub failure_codes: Vec<String>,
    pub verification_results: Vec<VerificationResult>,
    pub artifact_hashes: Vec<ArtifactHash>,
    pub trace_bundle_ref: String,
    pub activation_log_ref: String,
    pub comparison_axes: ComparisonAxes,
    pub privacy: PrivacySummary,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub derived_metrics: Option<Vec<DerivedMetric>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanonicalEvidenceRunContext {
    pub run_id: String,
    pub workflow_id: String,
    pub subject: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant_label: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanonicalizationProfile {
    pub profile_name: String,
    pub profile_version: u32,
    pub volatile_fields_excluded: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VerificationResult {
    pub check_id: String,
    pub status: String,
    pub evidence_ref: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactHash {
    pub path: String,
    pub sha256: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComparisonAxes {
    pub primary_metric: String,
    pub direction: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_metrics: Option<Vec<SecondaryMetric>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SecondaryMetric {
    pub metric: String,
    pub direction: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrivacySummary {
    pub secrets_present: bool,
    pub raw_prompt_or_tool_args_present: bool,
    pub absolute_host_paths_present: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redaction_notes: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DerivedMetric {
    pub metric: String,
    pub value: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CanonicalEvidenceError {
    Invalid(String),
    Io(String),
    Serialize(String),
}

impl std::fmt::Display for CanonicalEvidenceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Invalid(msg) => write!(f, "GODEL_CANONICAL_EVIDENCE_INVALID: {msg}"),
            Self::Io(msg) => write!(f, "GODEL_CANONICAL_EVIDENCE_IO: {msg}"),
            Self::Serialize(msg) => write!(f, "GODEL_CANONICAL_EVIDENCE_SERIALIZE: {msg}"),
        }
    }
}

impl std::error::Error for CanonicalEvidenceError {}

pub fn build_canonical_evidence(
    input: &StageLoopInput,
) -> Result<CanonicalEvidenceView, CanonicalEvidenceError> {
    validate_input(input)?;

    let mut failure_codes = vec![input.failure_code.clone()];
    failure_codes.sort();
    failure_codes.dedup();

    let activation_log_ref = input
        .evidence_refs
        .iter()
        .find(|path| path.ends_with("activation_log.json"))
        .cloned()
        .unwrap_or_else(|| format!("runs/{}/logs/activation_log.json", input.run_id));
    let trace_bundle_ref = input
        .evidence_refs
        .iter()
        .find(|path| path.ends_with("trace_bundle_v2.json"))
        .cloned()
        .unwrap_or_else(|| format!("runs/{}/trace_bundle_v2.json", input.run_id));

    let mut volatile_fields_excluded = vec![
        "elapsed_ms".to_string(),
        "host_paths".to_string(),
        "timestamps".to_string(),
    ];
    volatile_fields_excluded.sort();
    volatile_fields_excluded.dedup();

    let mut redaction_notes = vec![
        "prompt bodies omitted".to_string(),
        "tool argument payloads omitted".to_string(),
    ];
    redaction_notes.sort();
    redaction_notes.dedup();

    let evidence = CanonicalEvidenceView {
        schema_name: CANONICAL_EVIDENCE_SCHEMA_NAME.to_string(),
        schema_version: CANONICAL_EVIDENCE_SCHEMA_VERSION,
        evidence_view_id: format!(
            "cev-{}-{}",
            sanitize_identifier(&input.run_id),
            sanitize_identifier(&input.failure_code)
        ),
        run_context: CanonicalEvidenceRunContext {
            run_id: input.run_id.clone(),
            workflow_id: input.workflow_id.clone(),
            subject: format!("workflow:{}", input.workflow_id),
            variant_label: Some("bounded-runtime".to_string()),
        },
        canonicalization_profile: CanonicalizationProfile {
            profile_name: "godel-evidence-default".to_string(),
            profile_version: 1,
            volatile_fields_excluded,
        },
        failure_codes,
        verification_results: Vec::new(),
        artifact_hashes: Vec::new(),
        trace_bundle_ref,
        activation_log_ref,
        comparison_axes: ComparisonAxes {
            primary_metric: "failure_occurrence".to_string(),
            direction: "decrease_is_better".to_string(),
            secondary_metrics: Some(vec![SecondaryMetric {
                metric: "evidence_ref_count".to_string(),
                direction: "target_match".to_string(),
            }]),
        },
        privacy: PrivacySummary {
            secrets_present: false,
            raw_prompt_or_tool_args_present: false,
            absolute_host_paths_present: false,
            redaction_notes: Some(redaction_notes),
        },
        derived_metrics: Some(vec![DerivedMetric {
            metric: "evidence_ref_count".to_string(),
            value: {
                let mut refs = input.evidence_refs.clone();
                refs.sort();
                refs.dedup();
                refs.len() as f64
            },
            unit: Some("count".to_string()),
        }]),
        notes: Some(vec![
            "failure_codes sorted lexicographically".to_string(),
            "volatile fields explicitly excluded by bounded runtime profile".to_string(),
        ]),
    };

    validate_canonical_evidence(&evidence)?;
    Ok(evidence)
}

pub fn persist_canonical_evidence(
    runs_root: &Path,
    evidence: &CanonicalEvidenceView,
) -> Result<PathBuf, CanonicalEvidenceError> {
    validate_canonical_evidence(evidence)?;
    let rel_path = PathBuf::from("runs")
        .join(evidence.run_context.run_id.as_str())
        .join("godel")
        .join("canonical_evidence_view.v1.json");
    let out_dir = runs_root
        .join(evidence.run_context.run_id.as_str())
        .join("godel");
    fs::create_dir_all(&out_dir)
        .map_err(|err| CanonicalEvidenceError::Io(format!("create dir failed: {err}")))?;
    let json = serde_json::to_string_pretty(evidence)
        .map_err(|err| CanonicalEvidenceError::Serialize(err.to_string()))?;
    fs::write(out_dir.join("canonical_evidence_view.v1.json"), json)
        .map_err(|err| CanonicalEvidenceError::Io(format!("write failed: {err}")))?;
    Ok(rel_path)
}

pub fn load_canonical_evidence(
    path: &Path,
) -> Result<CanonicalEvidenceView, CanonicalEvidenceError> {
    let raw = fs::read_to_string(path)
        .map_err(|err| CanonicalEvidenceError::Io(format!("read failed: {err}")))?;
    let parsed: CanonicalEvidenceView = serde_json::from_str(&raw)
        .map_err(|err| CanonicalEvidenceError::Invalid(format!("parse failed: {err}")))?;
    validate_canonical_evidence(&parsed)?;
    Ok(parsed)
}

pub fn validate_canonical_evidence(
    evidence: &CanonicalEvidenceView,
) -> Result<(), CanonicalEvidenceError> {
    let repo_root = repo_root_from_manifest()?;
    let schema_path = repo_root
        .join("adl-spec")
        .join("schemas")
        .join("v0.8")
        .join("canonical_evidence_view.v1.schema.json");
    let schema_json: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(&schema_path)
            .map_err(|err| CanonicalEvidenceError::Io(format!("read schema failed: {err}")))?,
    )
    .map_err(|err| CanonicalEvidenceError::Invalid(format!("parse schema failed: {err}")))?;
    let compiled = JSONSchema::options()
        .compile(&schema_json)
        .map_err(|err| CanonicalEvidenceError::Invalid(format!("compile schema failed: {err}")))?;
    let value = serde_json::to_value(evidence)
        .map_err(|err| CanonicalEvidenceError::Serialize(err.to_string()))?;
    if let Err(errors) = compiled.validate(&value) {
        let first = errors
            .into_iter()
            .next()
            .map(|err| err.to_string())
            .unwrap_or_else(|| "unknown schema validation failure".to_string());
        return Err(CanonicalEvidenceError::Invalid(format!(
            "canonical schema validation failed: {first}"
        )));
    }
    Ok(())
}

fn validate_input(input: &StageLoopInput) -> Result<(), CanonicalEvidenceError> {
    if input.run_id.trim().is_empty()
        || input.workflow_id.trim().is_empty()
        || input.failure_code.trim().is_empty()
        || input.failure_summary.trim().is_empty()
    {
        return Err(CanonicalEvidenceError::Invalid(
            "run_id, workflow_id, failure_code, and failure_summary must be non-empty".to_string(),
        ));
    }
    for path in &input.evidence_refs {
        validate_repo_relative_ref(path)?;
    }
    Ok(())
}

fn validate_repo_relative_ref(path: &str) -> Result<(), CanonicalEvidenceError> {
    if path.trim().is_empty()
        || path.starts_with('/')
        || path.contains("..")
        || path.contains(':')
        || path.contains('\\')
    {
        return Err(CanonicalEvidenceError::Invalid(format!(
            "invalid repo-relative ref: {path}"
        )));
    }
    Ok(())
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
        "evidence".to_string()
    } else {
        trimmed.chars().take(96).collect()
    }
}

fn repo_root_from_manifest() -> Result<PathBuf, CanonicalEvidenceError> {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
    let Some(repo_root) = manifest.parent() else {
        return Err(CanonicalEvidenceError::Invalid(
            "unable to derive repository root from CARGO_MANIFEST_DIR".to_string(),
        ));
    };
    Ok(repo_root.to_path_buf())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture_input() -> StageLoopInput {
        StageLoopInput {
            run_id: "run-745-a".to_string(),
            workflow_id: "wf-godel-loop".to_string(),
            failure_code: "tool_failure".to_string(),
            failure_summary: "step failed with deterministic parse error".to_string(),
            evidence_refs: vec![
                "runs/run-745-a/logs/activation_log.json".to_string(),
                "runs/run-745-a/run_status.json".to_string(),
                "runs/run-745-a/run_status.json".to_string(),
            ],
        }
    }

    fn test_tmp_dir(label: &str) -> PathBuf {
        let root =
            std::env::temp_dir().join(format!("adl-godel-evidence-{label}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&root).expect("mkdir test tmp");
        root
    }

    #[test]
    fn canonical_evidence_round_trip_validates_against_schema() {
        let tmp = test_tmp_dir("round-trip");
        let evidence = build_canonical_evidence(&fixture_input()).expect("build evidence");
        let rel = persist_canonical_evidence(&tmp, &evidence).expect("persist evidence");
        assert_eq!(
            rel,
            PathBuf::from("runs/run-745-a/godel/canonical_evidence_view.v1.json")
        );
        let loaded = load_canonical_evidence(
            &tmp.join("run-745-a")
                .join("godel")
                .join("canonical_evidence_view.v1.json"),
        )
        .expect("load evidence");
        assert_eq!(loaded.schema_name, "canonical_evidence_view");
        assert_eq!(loaded.failure_codes, vec!["tool_failure"]);
        assert_eq!(loaded.verification_results.len(), 0);
        assert_eq!(loaded.artifact_hashes.len(), 0);
        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn canonical_evidence_rejects_unsafe_refs() {
        let mut input = fixture_input();
        input.evidence_refs = vec!["/Users/daniel/secret.json".to_string()];
        let err = build_canonical_evidence(&input).expect_err("unsafe ref must fail");
        assert!(err.to_string().contains("GODEL_CANONICAL_EVIDENCE_INVALID"));
    }
}
