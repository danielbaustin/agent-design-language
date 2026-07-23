use std::collections::BTreeMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{ErrorCode, RecordError, Result};

pub const CONTRACT_VERSION: &str = "adl.records.v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Limits {
    pub max_envelope_bytes: usize,
    pub max_payload_bytes: usize,
    pub max_string_bytes: usize,
    pub max_metadata_entries: usize,
    pub max_trace_attributes: usize,
    pub max_replay_entries: usize,
    pub max_json_depth: usize,
    pub max_json_members: usize,
}

impl Default for Limits {
    fn default() -> Self {
        Self {
            max_envelope_bytes: 1_048_576,
            max_payload_bytes: 524_288,
            max_string_bytes: 4096,
            max_metadata_entries: 128,
            max_trace_attributes: 128,
            max_replay_entries: 4096,
            max_json_depth: 32,
            max_json_members: 4096,
        }
    }
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, JsonSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum RecordKind {
    Error,
    Event,
    Trace,
    ExecutionResult,
    Artifact,
}

impl RecordKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Error => "error",
            Self::Event => "event",
            Self::Trace => "trace",
            Self::ExecutionResult => "execution_result",
            Self::Artifact => "artifact",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct RecordHeader {
    pub contract_version: String,
    pub record_id: String,
    pub subject_id: String,
    pub sequence: u64,
    pub logical_timestamp: u64,
    #[serde(default)]
    pub metadata: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ErrorRecord {
    pub header: RecordHeader,
    pub code: String,
    pub message: String,
    pub retryable: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct EventRecord {
    pub header: RecordHeader,
    pub name: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct TraceRecord {
    pub header: RecordHeader,
    pub trace_id: String,
    pub span_id: String,
    pub parent_span_id: Option<String>,
    pub operation: String,
    #[serde(default)]
    pub attributes: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ExecutionResult {
    pub header: RecordHeader,
    pub status: String,
    pub output_digest: Option<String>,
    pub diagnostic: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ArtifactDescriptor {
    pub header: RecordHeader,
    pub media_type: String,
    pub content_digest: String,
    pub byte_length: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(
    deny_unknown_fields,
    tag = "kind",
    content = "record",
    rename_all = "snake_case"
)]
pub enum Record {
    Error(ErrorRecord),
    Event(EventRecord),
    Trace(TraceRecord),
    ExecutionResult(ExecutionResult),
    Artifact(ArtifactDescriptor),
}

impl Record {
    pub fn kind(&self) -> RecordKind {
        match self {
            Self::Error(_) => RecordKind::Error,
            Self::Event(_) => RecordKind::Event,
            Self::Trace(_) => RecordKind::Trace,
            Self::ExecutionResult(_) => RecordKind::ExecutionResult,
            Self::Artifact(_) => RecordKind::Artifact,
        }
    }

    pub fn header(&self) -> &RecordHeader {
        match self {
            Self::Error(value) => &value.header,
            Self::Event(value) => &value.header,
            Self::Trace(value) => &value.header,
            Self::ExecutionResult(value) => &value.header,
            Self::Artifact(value) => &value.header,
        }
    }

    pub fn validate(&self, limits: &Limits) -> Result<()> {
        self.header().validate(limits)?;
        match self {
            Self::Error(value) => {
                bounded(&value.code, limits)?;
                bounded(&value.message, limits)
            }
            Self::Event(value) => {
                bounded(&value.name, limits)?;
                bounded(&value.detail, limits)
            }
            Self::Trace(value) => {
                bounded(&value.trace_id, limits)?;
                bounded(&value.span_id, limits)?;
                if let Some(parent) = &value.parent_span_id {
                    bounded(parent, limits)?;
                }
                bounded(&value.operation, limits)?;
                validate_map(&value.attributes, limits.max_trace_attributes, limits)
            }
            Self::ExecutionResult(value) => {
                bounded(&value.status, limits)?;
                if let Some(digest) = &value.output_digest {
                    validate_digest(digest)?;
                }
                if let Some(diagnostic) = &value.diagnostic {
                    bounded(diagnostic, limits)?;
                }
                Ok(())
            }
            Self::Artifact(value) => {
                bounded(&value.media_type, limits)?;
                validate_digest(&value.content_digest)
            }
        }
    }
}

impl RecordHeader {
    fn validate(&self, limits: &Limits) -> Result<()> {
        if self.contract_version != CONTRACT_VERSION || self.sequence == 0 {
            return Err(RecordError::new(
                ErrorCode::InvalidRecord,
                "invalid record header",
            ));
        }
        bounded(&self.record_id, limits)?;
        bounded(&self.subject_id, limits)?;
        validate_map(&self.metadata, limits.max_metadata_entries, limits)
    }
}

fn bounded(value: &str, limits: &Limits) -> Result<()> {
    if value.is_empty() || value.len() > limits.max_string_bytes {
        return Err(RecordError::new(ErrorCode::Bounds, "string bound exceeded"));
    }
    Ok(())
}

fn validate_map(map: &BTreeMap<String, String>, maximum: usize, limits: &Limits) -> Result<()> {
    if map.len() > maximum {
        return Err(RecordError::new(ErrorCode::Bounds, "map bound exceeded"));
    }
    for (key, value) in map {
        bounded(key, limits)?;
        bounded(value, limits)?;
    }
    Ok(())
}

fn validate_digest(value: &str) -> Result<()> {
    if value.len() != 64 || hex::decode(value).is_err() {
        return Err(RecordError::new(
            ErrorCode::InvalidRecord,
            "digest must be SHA-256 hex",
        ));
    }
    Ok(())
}
