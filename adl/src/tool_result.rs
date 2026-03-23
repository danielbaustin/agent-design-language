use std::path::{Path, PathBuf};

use anyhow::{anyhow, Context, Result};
use jsonschema::JSONSchema;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value as JsonValue};
use sha2::{Digest, Sha256};

use crate::artifacts;

static TOOL_RESULT_SCHEMA_JSON: Lazy<JsonValue> = Lazy::new(|| {
    serde_json::from_str(include_str!(
        "../../adl-spec/schemas/v0.8/tool_result.v1.schema.json"
    ))
    .expect("tool_result.v1 schema must parse")
});

static TOOL_RESULT_SCHEMA: Lazy<JSONSchema> = Lazy::new(|| {
    JSONSchema::options()
        .compile(&TOOL_RESULT_SCHEMA_JSON)
        .expect("tool_result.v1 schema must compile")
});

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ToolResultArtifact {
    pub schema_version: String,
    pub tool_name: String,
    pub invocation_id: String,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<JsonValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ToolResultError>,
    pub artifact_refs: Vec<ToolResultArtifactRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<JsonValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ToolResultError {
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd)]
#[serde(deny_unknown_fields)]
pub struct ToolResultArtifactRef {
    pub kind: String,
    pub path: String,
    pub sha256: String,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum LearnExportFormat {
    Jsonl,
    BundleV1,
    TraceBundleV2,
}

impl LearnExportFormat {
    pub fn parse(format: &str) -> Result<Self> {
        match format {
            "jsonl" => Ok(Self::Jsonl),
            "bundle-v1" => Ok(Self::BundleV1),
            "trace-bundle-v2" => Ok(Self::TraceBundleV2),
            other => Err(anyhow!(
                "unsupported learn export format '{other}' (supported: jsonl, bundle-v1, trace-bundle-v2)"
            )),
        }
    }

    fn invocation_id(self) -> &'static str {
        match self {
            Self::Jsonl => "learn-export-jsonl",
            Self::BundleV1 => "learn-export-bundle-v1",
            Self::TraceBundleV2 => "learn-export-trace-bundle-v2",
        }
    }

    fn sidecar_path(self, out_path: &Path) -> PathBuf {
        match self {
            Self::Jsonl => {
                let file_name = out_path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("learning.jsonl");
                out_path
                    .parent()
                    .unwrap_or_else(|| Path::new("."))
                    .join(format!("{file_name}.tool_result.v1.json"))
            }
            Self::BundleV1 | Self::TraceBundleV2 => out_path.join("tool_result.v1.json"),
        }
    }

    fn primary_artifact_path(self, out_path: &Path) -> PathBuf {
        match self {
            Self::Jsonl => out_path.to_path_buf(),
            Self::BundleV1 => out_path.join("learning_export_v1").join("manifest.json"),
            Self::TraceBundleV2 => out_path.join("trace_bundle_v2").join("manifest.json"),
        }
    }

    fn primary_artifact_ref(self, out_path: &Path) -> String {
        match self {
            Self::Jsonl => {
                let file_name = out_path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("learning.jsonl");
                format!("out/{file_name}")
            }
            Self::BundleV1 => "out/learning_export_v1/manifest.json".to_string(),
            Self::TraceBundleV2 => "out/trace_bundle_v2/manifest.json".to_string(),
        }
    }
}

pub fn write_learn_export_tool_result(
    format: LearnExportFormat,
    out_path: &Path,
    rows: usize,
) -> Result<PathBuf> {
    let primary_artifact_path = format.primary_artifact_path(out_path);
    let artifact_bytes = std::fs::read(&primary_artifact_path).with_context(|| {
        format!(
            "read learn export artifact '{}'",
            primary_artifact_path.display()
        )
    })?;

    let artifact_ref = ToolResultArtifactRef {
        kind: "export".to_string(),
        path: format.primary_artifact_ref(out_path),
        sha256: sha256_hex(&artifact_bytes),
    };

    let artifact = ToolResultArtifact {
        schema_version: "tool_result.v1".to_string(),
        tool_name: "adl.learn.export".to_string(),
        invocation_id: format.invocation_id().to_string(),
        status: "success".to_string(),
        payload: Some(json!({
            "format": match format {
                LearnExportFormat::Jsonl => "jsonl",
                LearnExportFormat::BundleV1 => "bundle-v1",
                LearnExportFormat::TraceBundleV2 => "trace-bundle-v2",
            },
            "rows_exported": rows,
        })),
        error: None,
        artifact_refs: vec![artifact_ref],
        metadata: Some(json!({
            "rows_exported": rows as u64,
        })),
    };

    validate_tool_result_artifact(&artifact)?;
    let sidecar = format.sidecar_path(out_path);
    let body = serde_json::to_vec_pretty(&artifact).context("serialize tool result artifact")?;
    artifacts::atomic_write(&sidecar, &body)?;
    Ok(sidecar)
}

pub fn validate_tool_result_artifact(artifact: &ToolResultArtifact) -> Result<()> {
    let json = serde_json::to_value(artifact).context("serialize tool result for validation")?;
    let validation = match TOOL_RESULT_SCHEMA.validate(&json) {
        Ok(()) => Ok(()),
        Err(errors) => {
            let problems = errors
                .take(10)
                .map(|err| {
                    let path = err.instance_path.to_string();
                    if path.is_empty() {
                        format!("{err}")
                    } else {
                        format!("{path}: {err}")
                    }
                })
                .collect::<Vec<_>>()
                .join("; ");
            Err(anyhow!("TOOL_RESULT_CONTRACT_VIOLATION: {problems}"))
        }
    };
    validation
}

fn sha256_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("{:x}", hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_tool_result_accepts_valid_success_artifact() {
        let artifact = ToolResultArtifact {
            schema_version: "tool_result.v1".to_string(),
            tool_name: "adl.learn.export".to_string(),
            invocation_id: "learn-export-jsonl".to_string(),
            status: "success".to_string(),
            payload: Some(json!({"format": "jsonl", "rows_exported": 1})),
            error: None,
            artifact_refs: vec![ToolResultArtifactRef {
                kind: "export".to_string(),
                path: "out/learning.jsonl".to_string(),
                sha256: "1111111111111111111111111111111111111111111111111111111111111111"
                    .to_string(),
            }],
            metadata: Some(json!({"rows_exported": 1})),
        };

        validate_tool_result_artifact(&artifact).expect("valid artifact");
    }

    #[test]
    fn validate_tool_result_rejects_invalid_artifact_ref_path() {
        let artifact = ToolResultArtifact {
            schema_version: "tool_result.v1".to_string(),
            tool_name: "adl.learn.export".to_string(),
            invocation_id: "learn-export-jsonl".to_string(),
            status: "success".to_string(),
            payload: Some(json!({"format": "jsonl", "rows_exported": 1})),
            error: None,
            artifact_refs: vec![ToolResultArtifactRef {
                kind: "export".to_string(),
                path: "/Users/daniel/private/learning.jsonl".to_string(),
                sha256: "1111111111111111111111111111111111111111111111111111111111111111"
                    .to_string(),
            }],
            metadata: None,
        };

        let err = validate_tool_result_artifact(&artifact).expect_err("absolute path must fail");
        assert!(err.to_string().contains("TOOL_RESULT_CONTRACT_VIOLATION"));
    }

    #[test]
    fn validate_tool_result_rejects_failure_without_error() {
        let artifact = ToolResultArtifact {
            schema_version: "tool_result.v1".to_string(),
            tool_name: "adl.learn.export".to_string(),
            invocation_id: "learn-export-jsonl".to_string(),
            status: "failure".to_string(),
            payload: None,
            error: None,
            artifact_refs: vec![ToolResultArtifactRef {
                kind: "export".to_string(),
                path: "out/learning.jsonl".to_string(),
                sha256: "1111111111111111111111111111111111111111111111111111111111111111"
                    .to_string(),
            }],
            metadata: None,
        };

        let err = validate_tool_result_artifact(&artifact).expect_err("failure must require error");
        assert!(err.to_string().contains("TOOL_RESULT_CONTRACT_VIOLATION"));
    }

    #[test]
    fn learn_export_sidecar_paths_are_deterministic() {
        let jsonl = LearnExportFormat::Jsonl.sidecar_path(Path::new("tmp/learning.jsonl"));
        assert_eq!(
            jsonl,
            PathBuf::from("tmp").join("learning.jsonl.tool_result.v1.json")
        );
        assert_eq!(
            LearnExportFormat::BundleV1.sidecar_path(Path::new("tmp/export")),
            PathBuf::from("tmp/export/tool_result.v1.json")
        );
        assert_eq!(
            LearnExportFormat::TraceBundleV2.sidecar_path(Path::new("tmp/export")),
            PathBuf::from("tmp/export/tool_result.v1.json")
        );
    }
}
