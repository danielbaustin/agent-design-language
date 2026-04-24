//! ADL document model and public loader APIs.
//!
//! This module defines the top-level `AdlDoc` structure and re-exports the
//! schema-facing types that back ADL YAML loading, validation, and execution.
use anyhow::{Context, Result};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

use crate::schema;

mod loading;
#[cfg(test)]
mod tests;
mod types;
mod validation;

pub use types::*;

/// Top-level ADL document.
///
/// MVP v0.1 supports:
/// - providers, tools, agents, tasks
/// - a single `run` with a workflow
///
/// Use this as the authoritative in-memory representation for ADL YAML input.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct AdlDoc {
    pub version: String,

    #[serde(default)]
    pub providers: HashMap<String, ProviderSpec>,

    #[serde(default)]
    pub tools: HashMap<String, ToolSpec>,

    #[serde(default)]
    pub agents: HashMap<String, AgentSpec>,

    #[serde(default)]
    pub tasks: HashMap<String, TaskSpec>,

    #[serde(default)]
    pub workflows: HashMap<String, WorkflowSpec>,

    #[serde(default)]
    pub patterns: Vec<PatternSpec>,

    #[serde(default)]
    pub signature: Option<SignatureSpec>,

    pub run: RunSpec,
}

impl AdlDoc {
    /// Load and validate an ADL document from a file path.
    ///
    /// This is the canonical bootstrap path for local execution and tests:
    /// it merges nested includes, validates schema, parses typed objects, and
    /// runs semantic validation.
    ///
    /// Loading order:
    /// 1. expands top-level `include` files with deterministic merge semantics
    /// 2. validates the merged document against schema
    /// 3. parses typed structures and runs semantic validation
    ///
    /// Security boundary:
    /// - include paths must be relative and cannot traverse via `..`
    /// - include cycles are rejected
    pub fn load_from_file(path: &str) -> Result<Self> {
        let merged = loading::load_yaml_with_includes(Path::new(path), &mut Vec::new())
            .with_context(|| format!("read/merge adl file (with includes): {path}"))?;
        let s = serde_yaml::to_string(&merged)
            .with_context(|| format!("serialize merged adl yaml: {path}"))?;

        // Schema validation first, so users get crisp errors.
        schema::validate_adl_yaml(&s)
            .with_context(|| format!("schema validate adl yaml: {path}"))?;

        let doc: Self = serde_yaml::from_value(merged)
            .with_context(|| format!("parse merged adl yaml: {path}"))?;

        doc.validate().with_context(|| "validate adl")?;
        Ok(doc)
    }
}
