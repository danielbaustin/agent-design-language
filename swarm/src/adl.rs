use anyhow::{anyhow, Context, Result};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::fs;

use crate::schema;

/// Top-level ADL document.
///
/// MVP v0.1 supports:
/// - providers, tools, agents, tasks
/// - a single `run` with a workflow
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

    pub run: RunSpec,
}

impl AdlDoc {
    /// Load an ADL YAML document from a file path.
    pub fn load_from_file(path: &str) -> Result<Self> {
        let s = fs::read_to_string(path).with_context(|| format!("read adl file: {path}"))?;

        // Schema validation first, so users get crisp errors.
        schema::validate_adl_yaml(&s)
            .with_context(|| format!("schema validate adl yaml: {path}"))?;

        let doc: Self =
            serde_yaml::from_str(&s).with_context(|| format!("parse adl yaml: {path}"))?;

        doc.validate().with_context(|| "validate adl")?;
        Ok(doc)
    }

    /// Lightweight validation so we can fail fast with good errors.
    pub fn validate(&self) -> Result<()> {
        // Validate run.workflow references
        for (idx, step) in self.run.workflow.steps.iter().enumerate() {
            let step_id = step
                .id
                .as_deref()
                .map(|s| s.to_string())
                .unwrap_or_else(|| format!("step-{idx}"));

            if let Some(agent) = step.agent.as_ref() {
                if !self.agents.is_empty() && !self.agents.contains_key(agent) {
                    return Err(anyhow!(
                        "run.workflow.steps[{idx}] references unknown agent '{agent}'"
                    ));
                }
            }

            if let Some(task) = step.task.as_ref() {
                if !self.tasks.is_empty() && !self.tasks.contains_key(task) {
                    return Err(anyhow!(
                        "run.workflow.steps[{idx}] references unknown task '{task}'"
                    ));
                }
            }

            if let Some(prompt) = step.prompt.as_ref() {
                // In MVP, `prompt` is an inline PromptSpec, so nothing to resolve.
                // Keep a placeholder for future prompt registries.
                let _ = prompt;
            }

            if step.write_to.is_some() && step.save_as.is_none() {
                return Err(anyhow!(
                    "step '{}' uses write_to but is missing save_as",
                    step_id
                ));
            }

            if let Some(write_to) = step.write_to.as_deref() {
                if write_to.trim().is_empty() {
                    return Err(anyhow!("step '{}' has empty write_to path", step_id));
                }
                let path = std::path::Path::new(write_to);
                if path.is_absolute()
                    || path
                        .components()
                        .any(|c| matches!(c, std::path::Component::ParentDir))
                {
                    return Err(anyhow!(
                        "step '{}' write_to must be a relative path without '..'",
                        step_id
                    ));
                }
            }
        }

        Ok(())
    }
}

/// Provider spec: local Ollama, OpenAI, etc.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ProviderSpec {
    /// Provider type (e.g. "ollama", "openai").
    #[serde(rename = "type")]
    pub kind: String,

    /// Optional base URL.
    #[serde(default)]
    pub base_url: Option<String>,

    /// Optional default model name (provider-specific).
    #[serde(default)]
    pub default_model: Option<String>,

    /// Arbitrary provider config.
    #[serde(default)]
    pub config: HashMap<String, JsonValue>,
}

/// Tool spec (eventually maps to MCP tools, local tools, etc.).
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct ToolSpec {
    /// Tool type (e.g. "mcp", "http", "local").
    #[serde(rename = "type")]
    pub kind: String,

    /// Arbitrary config.
    #[serde(default)]
    pub config: HashMap<String, JsonValue>,
}

/// Agent spec.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct AgentSpec {
    /// Provider this agent uses.
    pub provider: String,

    /// Model name (provider-specific).
    pub model: String,

    /// Optional temperature for this agent.
    #[serde(default, deserialize_with = "de_opt_string_from_number_or_string")]
    #[schemars(with = "StringOrNumber")]
    pub temperature: Option<String>,

    /// Optional top-k for this agent.
    #[serde(default)]
    pub top_k: Option<u32>,

    /// Optional agent description.
    #[serde(default)]
    pub description: Option<String>,

    /// Optional default prompt for the agent.
    #[serde(default)]
    pub prompt: Option<PromptSpec>,

    /// Optional tool ids this agent may use.
    #[serde(default)]
    pub tools: Vec<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(untagged)]
enum StringOrNumber {
    Str(String),
    I64(i64),
    U64(u64),
    F64(f64),
}

fn de_opt_string_from_number_or_string<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let opt = Option::<StringOrNumber>::deserialize(deserializer)?;
    Ok(opt.map(|v| match v {
        StringOrNumber::Str(s) => s,
        StringOrNumber::I64(i) => i.to_string(),
        StringOrNumber::U64(u) => u.to_string(),
        StringOrNumber::F64(f) => f.to_string(),
    }))
}

/// Task spec.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct TaskSpec {
    #[serde(default)]
    pub description: Option<String>,

    /// Default prompt for this task.
    pub prompt: PromptSpec,
}

/// Prompt specification: structured fields that will be assembled into a final prompt.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default, PartialEq, Eq)]
pub struct PromptSpec {
    #[serde(default)]
    pub system: Option<String>,

    #[serde(default)]
    pub developer: Option<String>,

    #[serde(default)]
    pub user: Option<String>,

    /// Extra context (e.g. retrieved docs) to be injected.
    #[serde(default)]
    pub context: Option<String>,

    /// Output requirements / format.
    #[serde(default)]
    pub output: Option<String>,
}

/// Run spec: what to execute.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct RunSpec {
    #[serde(default)]
    pub name: Option<String>,

    /// Optional creation time in RFC3339 (e.g. "2026-01-24T10:30:00Z").
    #[serde(default)]
    pub created_at: Option<String>,

    #[serde(default)]
    pub defaults: RunDefaults,

    pub workflow: WorkflowSpec,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default, PartialEq, Eq)]
pub struct RunDefaults {
    /// Default system string applied if prompt has no system.
    #[serde(default)]
    pub system: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct WorkflowSpec {
    #[serde(default)]
    pub kind: WorkflowKind,

    #[serde(default)]
    pub steps: Vec<StepSpec>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum WorkflowKind {
    #[default]
    Sequential,
    Concurrent,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct StepSpec {
    /// Optional explicit id for the step.
    #[serde(default)]
    pub id: Option<String>,

    /// Optional state key to save the step output under.
    #[serde(default)]
    pub save_as: Option<String>,

    /// Optional relative path to write the step output under `--out`.
    #[serde(default)]
    pub write_to: Option<String>,

    /// Agent id to run (key in `agents`).
    #[serde(default)]
    pub agent: Option<String>,

    /// Task id to run (key in `tasks`).
    #[serde(default)]
    pub task: Option<String>,

    /// Inline prompt override.
    #[serde(default)]
    pub prompt: Option<PromptSpec>,

    /// Named inputs that can be used by the runtime/prompt assembly.
    #[serde(default)]
    pub inputs: HashMap<String, String>,

    /// Guard directives (content normalization / output constraints, etc.).
    #[serde(default)]
    pub guards: Vec<GuardSpec>,
}

impl StepSpec {
    // Helper for prompt selection precedence (step > task > agent).
    // Not currently used by the v0.1 binary, but relied upon by integration tests and kept
    // as a stable utility for upcoming resolver/runtime work.
    #[allow(dead_code)]
    /// Returns the prompt to use for this step in priority order:
    /// 1) step.prompt
    /// 2) task.prompt (if task is set)
    /// 3) agent.prompt (if agent is set)
    pub fn effective_prompt<'a>(&'a self, doc: &'a AdlDoc) -> Option<&'a PromptSpec> {
        if let Some(p) = self.prompt.as_ref() {
            return Some(p);
        }

        if let Some(task_key) = self.task.as_ref() {
            if let Some(t) = doc.tasks.get(task_key) {
                return Some(&t.prompt);
            }
        }

        if let Some(agent_key) = self.agent.as_ref() {
            if let Some(a) = doc.agents.get(agent_key) {
                if let Some(p) = a.prompt.as_ref() {
                    return Some(p);
                }
            }
        }

        None
    }
}

/// Guard directive (content normalization / output constraints, etc.).
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct GuardSpec {
    #[serde(rename = "type")]
    pub kind: String,

    #[serde(default)]
    pub config: HashMap<String, JsonValue>,
}
