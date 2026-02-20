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

    #[serde(default)]
    pub workflows: HashMap<String, WorkflowSpec>,

    #[serde(default)]
    pub patterns: Vec<PatternSpec>,

    #[serde(default)]
    pub signature: Option<SignatureSpec>,

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
        if matches!(self.run.defaults.max_concurrency, Some(0)) {
            return Err(anyhow!(
                "run.defaults.max_concurrency must be >= 1 when provided"
            ));
        }

        validate_id_fields("providers", &self.providers, |spec| spec.id.as_deref())?;
        validate_id_fields("tools", &self.tools, |spec| spec.id.as_deref())?;
        validate_id_fields("agents", &self.agents, |spec| spec.id.as_deref())?;
        validate_id_fields("tasks", &self.tasks, |spec| spec.id.as_deref())?;
        validate_id_fields("workflows", &self.workflows, |spec| spec.id.as_deref())?;

        for provider_id in sorted_keys(&self.providers) {
            let provider = &self.providers[provider_id];
            validate_provider(provider_id, provider)?;
        }

        for tool_id in sorted_keys(&self.tools) {
            let tool = &self.tools[tool_id];
            validate_tool(tool_id, tool)?;
        }

        for agent_id in sorted_keys(&self.agents) {
            let agent = &self.agents[agent_id];
            if !self.providers.is_empty() && !self.providers.contains_key(&agent.provider) {
                return Err(anyhow!(
                    "agents.{agent_id}.provider references unknown provider '{}'",
                    agent.provider
                ));
            }
            for tool_ref in &agent.tools {
                if !self.tools.contains_key(tool_ref) {
                    return Err(anyhow!(
                        "agents.{agent_id}.tools references unknown tool '{tool_ref}'"
                    ));
                }
            }
        }

        for task_id in sorted_keys(&self.tasks) {
            let task = &self.tasks[task_id];
            if let Some(agent_ref) = task.agent_ref.as_deref() {
                if !self.agents.contains_key(agent_ref) {
                    return Err(anyhow!(
                        "tasks.{task_id}.agent_ref references unknown agent '{agent_ref}'"
                    ));
                }
            }
            for tool_ref in &task.tool_allowlist {
                if !self.tools.contains_key(tool_ref) {
                    return Err(anyhow!(
                        "tasks.{task_id}.tool_allowlist references unknown tool '{tool_ref}'"
                    ));
                }
            }
        }

        let mut seen_patterns = std::collections::HashSet::new();
        for pattern in &self.patterns {
            if pattern.id.trim().is_empty() {
                return Err(anyhow!("pattern id must not be empty"));
            }
            if !seen_patterns.insert(pattern.id.clone()) {
                return Err(anyhow!("duplicate pattern id '{}'", pattern.id));
            }
            pattern.validate()?;
        }

        if let Some(pattern_ref) = self.run.pattern_ref.as_ref() {
            if !self.patterns.iter().any(|p| p.id == *pattern_ref) {
                return Err(anyhow!(
                    "run.pattern_ref references unknown pattern '{}'",
                    pattern_ref
                ));
            }
            if self.run.workflow_ref.is_some() || self.run.workflow.is_some() {
                return Err(anyhow!(
                    "run.pattern_ref cannot be combined with run.workflow_ref or inline run.workflow"
                ));
            }
            return Ok(());
        }

        let workflow = self.run.resolve_workflow(self)?;
        for (idx, step) in workflow.steps.iter().enumerate() {
            let step_id = step
                .id
                .as_deref()
                .map(|s| s.to_string())
                .unwrap_or_else(|| format!("step-{idx}"));

            if step_id.starts_with("p::") {
                return Err(anyhow!(
                    "step id '{}' uses reserved compiler prefix 'p::'",
                    step_id
                ));
            }

            if let Some(agent) = step.agent.as_ref() {
                if !self.agents.is_empty() && !self.agents.contains_key(agent) {
                    return Err(anyhow!(
                        "run.workflow.steps[{idx}] references unknown agent '{agent}'"
                    ));
                }
            }

            if let Some(task_ref) = step.task.as_ref() {
                if !self.tasks.is_empty() && !self.tasks.contains_key(task_ref) {
                    return Err(anyhow!(
                        "run.workflow.steps[{idx}] references unknown task '{task_ref}'"
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

            if let Some(retry) = step.retry.as_ref() {
                if retry.max_attempts == 0 {
                    return Err(anyhow!(
                        "step '{}' has invalid retry.max_attempts=0 (must be >= 1)",
                        step_id
                    ));
                }
            }
        }

        Ok(())
    }
}

fn sorted_keys<T>(m: &HashMap<String, T>) -> Vec<&String> {
    let mut keys: Vec<&String> = m.keys().collect();
    keys.sort();
    keys
}

fn validate_id_fields<T>(
    section: &str,
    items: &HashMap<String, T>,
    get_id: impl Fn(&T) -> Option<&str>,
) -> Result<()> {
    for key in sorted_keys(items) {
        if let Some(explicit_id) = get_id(&items[key]) {
            if explicit_id != key {
                return Err(anyhow!(
                    "{section}.{key}.id must match key '{key}' when provided (found '{explicit_id}')"
                ));
            }
        }
    }
    Ok(())
}

fn validate_provider(provider_id: &str, provider: &ProviderSpec) -> Result<()> {
    match provider.kind.as_str() {
        "ollama" | "local_ollama" | "mock" => Ok(()),
        "http" | "http_remote" => {
            let endpoint_ok = provider.base_url.is_some()
                || provider
                    .config
                    .get("endpoint")
                    .and_then(|v| v.as_str())
                    .is_some();
            if !endpoint_ok {
                return Err(anyhow!(
                    "providers.{provider_id} kind '{}' requires base_url or config.endpoint",
                    provider.kind
                ));
            }
            Ok(())
        }
        other => Err(anyhow!(
            "providers.{provider_id} has unsupported kind '{other}' (supported: ollama, local_ollama, mock, http, http_remote)"
        )),
    }
}

fn validate_tool(tool_id: &str, tool: &ToolSpec) -> Result<()> {
    match tool.kind.as_str() {
        "mcp" | "local" | "http" | "builtin" => Ok(()),
        other => Err(anyhow!(
            "tools.{tool_id} has unsupported kind '{other}' (supported: mcp, local, http, builtin)"
        )),
    }
}

/// Provider spec: local Ollama, OpenAI, etc.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ProviderSpec {
    #[serde(default)]
    pub id: Option<String>,

    /// Provider type (e.g. "ollama", "openai").
    #[serde(rename = "type", alias = "kind")]
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
#[serde(deny_unknown_fields)]
pub struct ToolSpec {
    #[serde(default)]
    pub id: Option<String>,

    /// Tool type (e.g. "mcp", "http", "local").
    #[serde(rename = "type", alias = "kind")]
    pub kind: String,

    /// Arbitrary config.
    #[serde(default)]
    pub config: HashMap<String, JsonValue>,
}

/// Agent spec.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct AgentSpec {
    #[serde(default)]
    pub id: Option<String>,

    /// Provider this agent uses.
    #[serde(alias = "provider_ref")]
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
#[serde(deny_unknown_fields)]
pub struct TaskSpec {
    #[serde(default)]
    pub id: Option<String>,

    #[serde(default)]
    pub agent_ref: Option<String>,

    #[serde(default)]
    pub inputs: Vec<String>,

    #[serde(default)]
    pub tool_allowlist: Vec<String>,

    #[serde(default)]
    pub description: Option<String>,

    /// Default prompt for this task.
    pub prompt: PromptSpec,
}

/// Prompt specification: structured fields that will be assembled into a final prompt.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
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
#[serde(deny_unknown_fields)]
pub struct RunSpec {
    #[serde(default)]
    pub id: Option<String>,

    #[serde(default)]
    pub name: Option<String>,

    /// Optional creation time in RFC3339 (e.g. "2026-01-24T10:30:00Z").
    #[serde(default)]
    pub created_at: Option<String>,

    #[serde(default)]
    pub defaults: RunDefaults,

    #[serde(default)]
    pub workflow_ref: Option<String>,

    #[serde(default)]
    pub workflow: Option<WorkflowSpec>,

    #[serde(default)]
    pub pattern_ref: Option<String>,

    #[serde(default)]
    pub inputs: HashMap<String, String>,

    #[serde(default)]
    pub placement: Option<RunPlacementSpec>,

    #[serde(default)]
    pub remote: Option<RunRemoteSpec>,
}

impl RunSpec {
    pub fn resolve_workflow<'a>(&'a self, doc: &'a AdlDoc) -> Result<&'a WorkflowSpec> {
        if self.pattern_ref.is_some() {
            return Err(anyhow!(
                "run.pattern_ref cannot be combined with run.workflow_ref or inline run.workflow"
            ));
        }
        match (self.workflow_ref.as_deref(), self.workflow.as_ref()) {
            (Some(_), Some(_)) => Err(anyhow!(
                "run may define either workflow_ref or inline workflow, but not both"
            )),
            (Some(workflow_ref), None) => doc.workflows.get(workflow_ref).ok_or_else(|| {
                anyhow!("run.workflow_ref references unknown workflow '{workflow_ref}'")
            }),
            (None, Some(workflow)) => Ok(workflow),
            (None, None) => Err(anyhow!(
                "run must define either workflow_ref or inline workflow"
            )),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct RunDefaults {
    /// Default system string applied if prompt has no system.
    #[serde(default)]
    pub system: Option<String>,

    /// Global runtime concurrency cap for concurrent workflows/pattern runs.
    /// When omitted, runtime uses a conservative default.
    #[serde(default)]
    pub max_concurrency: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct WorkflowSpec {
    #[serde(default)]
    pub id: Option<String>,

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

    /// Step-level error behavior. Defaults to fail-fast.
    #[serde(default)]
    pub on_error: Option<StepOnError>,

    /// Optional deterministic retry policy.
    #[serde(default)]
    pub retry: Option<StepRetry>,

    /// Agent id to run (key in `agents`).
    #[serde(default, alias = "agent_ref")]
    pub agent: Option<String>,

    /// Task id to run (key in `tasks`).
    #[serde(default, alias = "task_ref")]
    pub task: Option<String>,

    /// Workflow id to call (key in `workflows`).
    #[serde(default)]
    pub call: Option<String>,

    /// Optional call input bindings.
    #[serde(default)]
    pub with: HashMap<String, String>,

    /// Optional namespace for call results.
    #[serde(default, rename = "as")]
    pub as_ns: Option<String>,

    /// Inline prompt override.
    #[serde(default)]
    pub prompt: Option<PromptSpec>,

    /// Named inputs that can be used by the runtime/prompt assembly.
    #[serde(default)]
    pub inputs: HashMap<String, String>,

    /// Optional placement override for this step.
    #[serde(default)]
    pub placement: Option<PlacementMode>,

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
#[serde(deny_unknown_fields)]
pub struct GuardSpec {
    #[serde(rename = "type")]
    pub kind: String,

    #[serde(default)]
    pub config: HashMap<String, JsonValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum PlacementMode {
    Local,
    Remote,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(untagged)]
pub enum RunPlacementSpec {
    Mode(PlacementMode),
    Legacy(RunPlacementLegacySpec),
}

impl RunPlacementSpec {
    pub fn mode(&self) -> Option<PlacementMode> {
        match self {
            RunPlacementSpec::Mode(mode) => Some(mode.clone()),
            RunPlacementSpec::Legacy(legacy) => legacy.target.as_deref().and_then(|v| {
                match v.trim().to_ascii_lowercase().as_str() {
                    "local" => Some(PlacementMode::Local),
                    "remote" => Some(PlacementMode::Remote),
                    _ => None,
                }
            }),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct RunPlacementLegacySpec {
    #[serde(default)]
    pub provider: Option<String>,

    #[serde(default)]
    pub target: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SignatureSpec {
    pub alg: String,
    pub key_id: String,
    #[serde(default)]
    pub public_key_b64: Option<String>,
    pub sig_b64: String,
    pub signed_header: SignedHeaderSpec,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SignedHeaderSpec {
    pub adl_version: String,
    #[serde(default)]
    pub workflow_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct RunRemoteSpec {
    pub endpoint: String,
    #[serde(default)]
    pub timeout_ms: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum StepOnError {
    Fail,
    Continue,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct StepRetry {
    pub max_attempts: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct PatternSpec {
    pub id: String,
    #[serde(rename = "type")]
    pub kind: PatternKind,
    #[serde(default)]
    pub steps: Vec<String>,
    #[serde(default)]
    pub fork: Option<PatternForkSpec>,
    #[serde(default)]
    pub join: Option<PatternJoinSpec>,
}

impl PatternSpec {
    fn validate(&self) -> Result<()> {
        match self.kind {
            PatternKind::Linear => {
                if self.steps.is_empty() {
                    return Err(anyhow!(
                        "pattern '{}' type=linear requires non-empty steps",
                        self.id
                    ));
                }
                for sym in &self.steps {
                    if sym.trim().is_empty() {
                        return Err(anyhow!("pattern '{}' has empty step symbol", self.id));
                    }
                }
            }
            PatternKind::ForkJoin => {
                let fork = self
                    .fork
                    .as_ref()
                    .ok_or_else(|| anyhow!("pattern '{}' type=fork_join requires fork", self.id))?;
                let join = self
                    .join
                    .as_ref()
                    .ok_or_else(|| anyhow!("pattern '{}' type=fork_join requires join", self.id))?;
                if join.step.trim().is_empty() {
                    return Err(anyhow!("pattern '{}' join.step must not be empty", self.id));
                }
                if fork.branches.is_empty() {
                    return Err(anyhow!(
                        "pattern '{}' fork.branches must not be empty",
                        self.id
                    ));
                }
                let mut seen = std::collections::HashSet::new();
                for br in &fork.branches {
                    if br.id.trim().is_empty() {
                        return Err(anyhow!("pattern '{}' has branch with empty id", self.id));
                    }
                    if br.id.starts_with("p::") {
                        return Err(anyhow!(
                            "pattern '{}' branch id '{}' cannot use reserved prefix 'p::'",
                            self.id,
                            br.id
                        ));
                    }
                    if !seen.insert(br.id.clone()) {
                        return Err(anyhow!(
                            "pattern '{}' has duplicate branch id '{}'",
                            self.id,
                            br.id
                        ));
                    }
                    if br.steps.is_empty() {
                        return Err(anyhow!(
                            "pattern '{}' branch '{}' requires non-empty steps",
                            self.id,
                            br.id
                        ));
                    }
                    for sym in &br.steps {
                        if sym.trim().is_empty() {
                            return Err(anyhow!(
                                "pattern '{}' branch '{}' has empty step symbol",
                                self.id,
                                br.id
                            ));
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PatternKind {
    Linear,
    ForkJoin,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct PatternForkSpec {
    #[serde(default)]
    pub branches: Vec<PatternBranchSpec>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct PatternBranchSpec {
    pub id: String,
    #[serde(default)]
    pub steps: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct PatternJoinSpec {
    pub step: String,
}
