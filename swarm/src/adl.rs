use anyhow::{anyhow, Context, Result};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::fs;
use std::path::{Component, Path, PathBuf};

/// Top-level ADL document.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct AdlDoc {
    pub version: String,

    #[serde(default)]
    pub include: Vec<String>,

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

    pub run: RunSpec,
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(deny_unknown_fields)]
struct AdlFragment {
    #[serde(default)]
    version: Option<String>,

    #[serde(default)]
    include: Vec<String>,

    #[serde(default)]
    providers: HashMap<String, ProviderSpec>,

    #[serde(default)]
    tools: HashMap<String, ToolSpec>,

    #[serde(default)]
    agents: HashMap<String, AgentSpec>,

    #[serde(default)]
    tasks: HashMap<String, TaskSpec>,

    #[serde(default)]
    workflows: HashMap<String, WorkflowSpec>,

    #[serde(default)]
    run: Option<RunSpec>,
}

#[derive(Default)]
struct MergeState {
    version: Option<String>,
    run: Option<RunSpec>,
    providers: HashMap<String, ProviderSpec>,
    tools: HashMap<String, ToolSpec>,
    agents: HashMap<String, AgentSpec>,
    tasks: HashMap<String, TaskSpec>,
    workflows: HashMap<String, WorkflowSpec>,
    providers_src: HashMap<String, String>,
    tools_src: HashMap<String, String>,
    agents_src: HashMap<String, String>,
    tasks_src: HashMap<String, String>,
    workflows_src: HashMap<String, String>,
}

impl AdlDoc {
    /// Load an ADL YAML document from a file path.
    ///
    /// Supports minimal composition includes:
    /// - `include: ["rel/path.yaml", ...]`
    /// - includes are processed in listed order
    /// - root document is merged last
    pub fn load_from_file(path: &str) -> Result<Self> {
        let root_path = PathBuf::from(path);
        let root_path = if root_path.is_absolute() {
            root_path
        } else {
            std::env::current_dir()
                .context("resolve current_dir")?
                .join(root_path)
        };

        let mut state = MergeState::default();
        let mut stack = Vec::new();
        load_fragment_recursive(&root_path, &mut state, &mut stack, true)?;

        let version = state
            .version
            .clone()
            .ok_or_else(|| anyhow!("missing required top-level field: version"))?;
        let run = state
            .run
            .clone()
            .ok_or_else(|| anyhow!("missing required top-level field: run"))?;

        let doc = AdlDoc {
            version,
            include: Vec::new(),
            providers: state.providers,
            tools: state.tools,
            agents: state.agents,
            tasks: state.tasks,
            workflows: state.workflows,
            run,
        };

        // Validate merged semantics. Includes may assemble fragments that are not
        // standalone schema-valid until merged, so semantic validation is the
        // authoritative check here.
        doc.validate().with_context(|| "validate merged adl")?;
        Ok(doc)
    }

    /// Lightweight validation so we can fail fast with good errors.
    pub fn validate(&self) -> Result<()> {
        let workflow = self.run.resolve_workflow(self)?;

        // Validate task/agent/tool references.
        for (task_id, task) in &self.tasks {
            if let Some(agent_ref) = task.agent_ref.as_ref() {
                if !self.agents.contains_key(agent_ref) {
                    return Err(anyhow!(
                        "task '{}' references unknown agent_ref '{}'",
                        task_id,
                        agent_ref
                    ));
                }
            }
            for tool_ref in &task.tool_allowlist {
                if !self.tools.contains_key(tool_ref) {
                    return Err(anyhow!(
                        "task '{}' references unknown tool_allowlist entry '{}'",
                        task_id,
                        tool_ref
                    ));
                }
            }
        }

        for (agent_id, agent) in &self.agents {
            if !agent.provider.trim().is_empty() && !self.providers.contains_key(&agent.provider) {
                return Err(anyhow!(
                    "agent '{}' references unknown provider '{}'",
                    agent_id,
                    agent.provider
                ));
            }
            for tool_ref in &agent.tools {
                if !self.tools.contains_key(tool_ref) {
                    return Err(anyhow!(
                        "agent '{}' references unknown tool '{}'",
                        agent_id,
                        tool_ref
                    ));
                }
            }
        }

        validate_workflow_steps(self, workflow, "run.workflow")?;
        for (wf_id, wf) in &self.workflows {
            validate_workflow_steps(self, wf, &format!("workflows.{}", wf_id))?;
        }

        Ok(())
    }
}

fn validate_workflow_steps(doc: &AdlDoc, wf: &WorkflowSpec, wf_label: &str) -> Result<()> {
    for (idx, step) in wf.steps.iter().enumerate() {
        let step_id = step
            .id
            .as_deref()
            .map(|s| s.to_string())
            .unwrap_or_else(|| format!("step-{idx}"));

        if let Some(call_target) = step.call.as_ref() {
            if !doc.workflows.contains_key(call_target) {
                return Err(anyhow!(
                    "{}.steps[{}] call references unknown workflow '{}'",
                    wf_label,
                    idx,
                    call_target
                ));
            }
            if step.agent.is_some()
                || step.task.is_some()
                || step.prompt.is_some()
                || !step.inputs.is_empty()
                || step.save_as.is_some()
                || step.write_to.is_some()
                || step.on_error.is_some()
                || step.retry.is_some()
                || !step.guards.is_empty()
            {
                return Err(anyhow!(
                    "{}.steps[{}] call step '{}' may only use id/call/with/as",
                    wf_label,
                    idx,
                    step_id
                ));
            }
            if let Some(ns) = step.as_ns.as_deref() {
                if ns.trim().is_empty() {
                    return Err(anyhow!(
                        "{}.steps[{}] call step '{}' has empty 'as' namespace",
                        wf_label,
                        idx,
                        step_id
                    ));
                }
            }
            continue;
        }

        if let Some(agent) = step.agent.as_ref() {
            if !doc.agents.is_empty() && !doc.agents.contains_key(agent) {
                return Err(anyhow!(
                    "{}.steps[{}] references unknown agent '{}'",
                    wf_label,
                    idx,
                    agent
                ));
            }
        }

        if let Some(task) = step.task.as_ref() {
            if !doc.tasks.is_empty() && !doc.tasks.contains_key(task) {
                return Err(anyhow!(
                    "{}.steps[{}] references unknown task '{}'",
                    wf_label,
                    idx,
                    task
                ));
            }
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

fn load_fragment_recursive(
    path: &Path,
    state: &mut MergeState,
    stack: &mut Vec<PathBuf>,
    is_root: bool,
) -> Result<()> {
    let canon = path
        .canonicalize()
        .with_context(|| format!("resolve include path '{}'", path.display()))?;

    if stack.contains(&canon) {
        return Err(anyhow!("include cycle detected at '{}'", canon.display()));
    }
    stack.push(canon.clone());

    let text = fs::read_to_string(&canon)
        .with_context(|| format!("read adl/include file: {}", canon.display()))?;

    let fragment: AdlFragment = serde_yaml::from_str(&text)
        .with_context(|| format!("parse adl yaml: {}", canon.display()))?;

    // Includes are processed first, in listed order.
    let parent = canon
        .parent()
        .ok_or_else(|| anyhow!("include file has no parent: {}", canon.display()))?;
    for inc in &fragment.include {
        let inc_path = resolve_include_path(parent, inc)
            .with_context(|| format!("invalid include '{}' in {}", inc, canon.display()))?;
        load_fragment_recursive(&inc_path, state, stack, false)?;
    }

    merge_fragment(state, &fragment, &canon, is_root)?;

    stack.pop();
    Ok(())
}

fn resolve_include_path(base_dir: &Path, include: &str) -> Result<PathBuf> {
    let raw = include.trim();
    if raw.is_empty() {
        return Err(anyhow!("include path is empty"));
    }
    let p = Path::new(raw);
    if p.is_absolute() {
        return Err(anyhow!("include path must be relative: '{}'", raw));
    }
    if p.components().any(|c| matches!(c, Component::ParentDir)) {
        return Err(anyhow!("include path must not contain '..': '{}'", raw));
    }
    Ok(base_dir.join(p))
}

fn merge_fragment(
    state: &mut MergeState,
    fragment: &AdlFragment,
    src: &Path,
    is_root: bool,
) -> Result<()> {
    let src_s = src.display().to_string();

    if is_root {
        if let Some(v) = fragment.version.as_ref() {
            state.version = Some(v.clone());
        }
        if let Some(run) = fragment.run.as_ref() {
            state.run = Some(run.clone());
        }
    } else if fragment.run.is_some() {
        return Err(anyhow!(
            "included file '{}' must not define top-level run",
            src.display()
        ));
    }

    merge_map(
        &mut state.providers,
        &mut state.providers_src,
        fragment.providers.clone(),
        "provider",
        &src_s,
    )?;
    merge_map(
        &mut state.tools,
        &mut state.tools_src,
        fragment.tools.clone(),
        "tool",
        &src_s,
    )?;
    merge_map(
        &mut state.agents,
        &mut state.agents_src,
        fragment.agents.clone(),
        "agent",
        &src_s,
    )?;
    merge_map(
        &mut state.tasks,
        &mut state.tasks_src,
        fragment.tasks.clone(),
        "task",
        &src_s,
    )?;
    merge_map(
        &mut state.workflows,
        &mut state.workflows_src,
        fragment.workflows.clone(),
        "workflow",
        &src_s,
    )?;

    Ok(())
}

fn merge_map<T: Clone>(
    dst: &mut HashMap<String, T>,
    src_map: &mut HashMap<String, String>,
    incoming: HashMap<String, T>,
    kind: &str,
    src: &str,
) -> Result<()> {
    for (id, value) in incoming {
        if let Some(prev_src) = src_map.get(&id) {
            return Err(anyhow!(
                "duplicate {} id '{}' found; first defined in '{}', conflicting definition in '{}'",
                kind,
                id,
                prev_src,
                src
            ));
        }
        dst.insert(id.clone(), value);
        src_map.insert(id, src.to_string());
    }
    Ok(())
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
#[serde(deny_unknown_fields)]
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
#[serde(deny_unknown_fields)]
pub struct TaskSpec {
    #[serde(default)]
    pub description: Option<String>,

    #[serde(default)]
    pub agent_ref: Option<String>,

    #[serde(default)]
    pub inputs: Vec<String>,

    #[serde(default)]
    pub tool_allowlist: Vec<String>,

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
    pub name: Option<String>,

    /// Optional creation time in RFC3339 (e.g. "2026-01-24T10:30:00Z").
    #[serde(default)]
    pub created_at: Option<String>,

    #[serde(default)]
    pub defaults: RunDefaults,

    #[serde(default)]
    pub workflow_ref: Option<String>,

    pub workflow: WorkflowSpec,

    #[serde(default)]
    pub inputs: HashMap<String, String>,
}

impl RunSpec {
    pub fn resolve_workflow<'a>(&'a self, doc: &'a AdlDoc) -> Result<&'a WorkflowSpec> {
        match self.workflow_ref.as_deref() {
            Some(workflow_ref) => doc.workflows.get(workflow_ref).ok_or_else(|| {
                anyhow!(
                    "run.workflow_ref references unknown workflow '{}'",
                    workflow_ref
                )
            }),
            None => Ok(&self.workflow),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct RunDefaults {
    /// Default system string applied if prompt has no system.
    #[serde(default)]
    pub system: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
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

    /// Step-level error behavior. Defaults to fail-fast.
    #[serde(default)]
    pub on_error: Option<StepOnError>,

    /// Optional deterministic retry policy.
    #[serde(default)]
    pub retry: Option<StepRetry>,

    /// Agent id to run (key in `agents`).
    #[serde(default)]
    pub agent: Option<String>,

    /// Task id to run (key in `tasks`).
    #[serde(default)]
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

    /// Guard directives (content normalization / output constraints, etc.).
    #[serde(default)]
    pub guards: Vec<GuardSpec>,
}

impl StepSpec {
    // Helper for prompt selection precedence (step > task > agent).
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
pub enum StepOnError {
    Fail,
    Continue,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct StepRetry {
    pub max_attempts: u32,
}
