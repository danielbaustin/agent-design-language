use anyhow::{anyhow, Context, Result};
use clap::{Parser, Subcommand};
use regex::Regex;
use serde::Deserialize;
use serde_json::json;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use tracing::{info, warn};

/// Swarm: reference runtime for Agent Design Language (ADL).
///
/// Day-1 MVP focus:
/// - Load ADL YAML
/// - Resolve run → workflow → task → agent → provider
/// - Print a deterministic execution plan
/// - Deterministically assemble prompts (no provider calls yet)
#[derive(Parser, Debug)]
#[command(name = "swarm")]
#[command(about = "Reference runtime for Agent Design Language (ADL)", long_about = None)]
struct Cli {
    /// Enable debug logging
    #[arg(long)]
    debug: bool,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Execute a run from an ADL document
    Run {
        /// Path to the ADL YAML file
        path: PathBuf,

        /// Run ID to execute (defaults to first run)
        #[arg(long)]
        run: Option<String>,

        /// Print the resolved execution plan and exit
        #[arg(long)]
        print_plan: bool,

        /// Print the deterministically assembled prompt(s) and exit
        #[arg(long)]
        print_prompt: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Tracing
    let filter = if cli.debug { "debug" } else { "info" };
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .without_time()
        .init();

    match cli.command {
        Command::Run {
            path,
            run,
            print_plan,
            print_prompt,
        } => {
            let doc = AdlDoc::load_from_file(&path)
                .with_context(|| format!("failed to load ADL document: {}", path.display()))?;

            let resolved = resolve_run(&doc, run.as_deref())?;

            if print_plan {
                print_resolved_plan(&resolved);
                return Ok(());
            }

            if print_prompt {
                print_prompts(&doc, &resolved)?;
                return Ok(());
            }

            // For Day 1 we print the plan.
            // Next: provider call + contracts + trace.
            print_resolved_plan(&resolved);
            Ok(())
        }
    }
}

// -----------------------------
// ADL data model (MVP subset)
// -----------------------------

#[derive(Debug, Deserialize)]
struct AdlDoc {
    adl: AdlHeader,

    #[serde(default)]
    providers: Vec<Provider>,
    #[serde(default)]
    tools: Vec<Tool>,
    #[serde(default)]
    agents: Vec<Agent>,
    #[serde(default)]
    tasks: Vec<Task>,
    #[serde(default)]
    workflows: Vec<Workflow>,
    #[serde(default)]
    runs: Vec<Run>,
}

#[derive(Debug, Deserialize)]
struct AdlHeader {
    version: String,
}

#[derive(Debug, Deserialize, Clone)]
struct Provider {
    id: String,
    kind: String,
    model: String,

    #[serde(default)]
    params: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Deserialize, Clone)]
struct Tool {
    id: String,
    kind: String,

    #[serde(default)]
    description: Option<String>,

    #[serde(default)]
    input: Option<Contract>,
    #[serde(default)]
    output: Option<Contract>,
}

#[derive(Debug, Deserialize, Clone)]
struct Contract {
    #[serde(default)]
    profile: Option<String>,

    /// NOTE: For MVP we treat schema as an opaque JSON value.
    /// Later we can define a portable subset (and/or protobuf).
    #[serde(default)]
    schema: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, Clone)]
struct Agent {
    id: String,
    provider: String,

    #[serde(default)]
    tools_allowed: Vec<String>,

    #[serde(default)]
    prompt: Option<Prompt>,
}

#[derive(Debug, Deserialize, Clone)]
struct Prompt {
    #[serde(default)]
    system: Option<String>,
    #[serde(default)]
    developer: Option<String>,
    #[serde(default)]
    instructions: Option<String>,

    #[serde(default)]
    examples: Vec<PromptExample>,
}

#[derive(Debug, Deserialize, Clone)]
struct PromptExample {
    input: HashMap<String, serde_json::Value>,
    output: serde_json::Value,
}

#[derive(Debug, Deserialize, Clone)]
struct Task {
    id: String,
    agent: String,

    #[serde(default)]
    inputs: HashMap<String, TaskInput>,

    #[serde(default)]
    output: Option<Contract>,

    #[serde(default)]
    repair: Option<RepairPolicy>,
}

#[derive(Debug, Deserialize, Clone)]
struct TaskInput {
    #[serde(default)]
    required: bool,

    #[serde(default)]
    constraints: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Deserialize, Clone)]
struct RepairPolicy {
    #[serde(default)]
    max_attempts: Option<u32>,

    #[serde(default)]
    strategy: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
struct Workflow {
    id: String,
    steps: Vec<Step>,
}

#[derive(Debug, Deserialize, Clone)]
struct Step {
    id: String,
    task: String,

    #[serde(default)]
    with: HashMap<String, String>,

    #[serde(default)]
    save_as: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
struct Run {
    id: String,
    workflow: String,

    #[serde(default)]
    inputs: HashMap<String, String>,

    #[serde(default)]
    hints: HashMap<String, serde_json::Value>,
}

impl AdlDoc {
    fn load_from_file(path: &PathBuf) -> Result<Self> {
        let text = fs::read_to_string(path)?;
        let doc: AdlDoc = serde_yaml::from_str(&text)
            .map_err(|e| anyhow!("YAML parse error: {e}"))
            .with_context(|| format!("while parsing {}", path.display()))?;

        // Basic sanity checks
        if doc.providers.is_empty() {
            warn!("No providers defined in ADL document");
        }
        if doc.runs.is_empty() {
            warn!("No runs defined in ADL document");
        }
        info!(version = %doc.adl.version, "Loaded ADL document");
        Ok(doc)
    }
}

// -----------------------------
// Resolution (Day 1 deliverable)
// -----------------------------

#[derive(Debug)]
struct Resolved<'a> {
    run: &'a Run,
    workflow: &'a Workflow,
    steps: Vec<ResolvedStep<'a>>,
}

#[derive(Debug)]
struct ResolvedStep<'a> {
    step: &'a Step,
    task: &'a Task,
    agent: &'a Agent,
    provider: &'a Provider,
}

fn resolve_run<'a>(doc: &'a AdlDoc, run_id: Option<&str>) -> Result<Resolved<'a>> {
    let run = match run_id {
        Some(id) => doc
            .runs
            .iter()
            .find(|r| r.id == id)
            .ok_or_else(|| anyhow!("Run not found: {id}"))?,
        None => doc
            .runs
            .first()
            .ok_or_else(|| anyhow!("No runs found in ADL document"))?,
    };

    let workflow = doc
        .workflows
        .iter()
        .find(|w| w.id == run.workflow)
        .ok_or_else(|| anyhow!("Workflow not found for run '{}': {}", run.id, run.workflow))?;

    let mut steps = Vec::with_capacity(workflow.steps.len());
    for step in &workflow.steps {
        let task = doc
            .tasks
            .iter()
            .find(|t| t.id == step.task)
            .ok_or_else(|| anyhow!("Task not found for step '{}': {}", step.id, step.task))?;

        let agent = doc
            .agents
            .iter()
            .find(|a| a.id == task.agent)
            .ok_or_else(|| anyhow!("Agent not found for task '{}': {}", task.id, task.agent))?;

        let provider = doc
            .providers
            .iter()
            .find(|p| p.id == agent.provider)
            .ok_or_else(|| {
                anyhow!(
                    "Provider not found for agent '{}': {}",
                    agent.id,
                    agent.provider
                )
            })?;

        // Minimal tool validation for Day 1: ensure declared tools exist.
        for tool_id in &agent.tools_allowed {
            if !doc.tools.iter().any(|t| t.id == *tool_id) {
                return Err(anyhow!(
                    "Tool '{}' listed in tools_allowed for agent '{}' but not defined",
                    tool_id,
                    agent.id
                ));
            }
        }

        steps.push(ResolvedStep {
            step,
            task,
            agent,
            provider,
        });
    }

    Ok(Resolved {
        run,
        workflow,
        steps,
    })
}

fn print_resolved_plan(resolved: &Resolved<'_>) {
    println!("ADL plan");
    println!("  run:      {}", resolved.run.id);
    println!("  workflow: {}", resolved.workflow.id);

    println!("  inputs:");
    if resolved.run.inputs.is_empty() {
        println!("    (none)");
    } else {
        for (k, v) in &resolved.run.inputs {
            println!("    {} = {}", k, v);
        }
    }

    println!("  steps:");
    for (i, rs) in resolved.steps.iter().enumerate() {
        println!("    {}. step: {}", i + 1, rs.step.id);
        println!("       task: {}", rs.task.id);
        println!("       agent: {}", rs.agent.id);
        println!(
            "       provider: {} ({}, model={})",
            rs.provider.id, rs.provider.kind, rs.provider.model
        );
        if let Some(save_as) = &rs.step.save_as {
            println!("       save_as: {}", save_as);
        }
        if !rs.step.with.is_empty() {
            println!("       with:");
            for (k, v) in &rs.step.with {
                println!("         {}: {}", k, v);
            }
        }
    }
}

fn print_prompts(doc: &AdlDoc, resolved: &Resolved<'_>) -> Result<()> {
    for (i, rs) in resolved.steps.iter().enumerate() {
        let bound = resolve_bindings(&resolved.run.inputs, &rs.step.with)
            .with_context(|| format!("while resolving bindings for step '{}'", rs.step.id))?;

        let prompt = rs.agent.prompt.as_ref();
        let system = prompt.and_then(|p| p.system.as_ref()).map(|s| s.as_str());
        let developer = prompt
            .and_then(|p| p.developer.as_ref())
            .map(|s| s.as_str());
        let instructions = prompt
            .and_then(|p| p.instructions.as_ref())
            .map(|s| s.as_str());

        println!("====================");
        println!("STEP {}: {}", i + 1, rs.step.id);
        println!("task: {}", rs.task.id);
        println!("agent: {}", rs.agent.id);
        println!("provider: {} ({}, model={})", rs.provider.id, rs.provider.kind, rs.provider.model);

        if let Some(s) = system {
            println!("\n[SYSTEM]\n{s}");
        }
        if let Some(s) = developer {
            println!("\n[DEVELOPER]\n{s}");
        }
        if let Some(s) = instructions {
            println!("\n[INSTRUCTIONS]\n{s}");
        }

        // For MVP prompt determinism, we serialize the bound inputs as JSON
        // and present them as the final user message.
        let user_payload = json!({
            "task": rs.task.id,
            "inputs": bound
        });

        let pretty = serde_json::to_string_pretty(&user_payload)?;
        println!("\n[USER]\n{pretty}\n");

        // Minimal check: ensure the agent's allowed tools exist (already validated in resolve_run)
        // but print them so the prompt output is self-explanatory.
        if !rs.agent.tools_allowed.is_empty() {
            let tools: Vec<&str> = rs.agent.tools_allowed.iter().map(|s| s.as_str()).collect();
            println!("[TOOLS_ALLOWED] {:?}", tools);
        }

        // Suppress unused warning for now; doc is kept in scope for near-future additions
        // such as tool schemas and example rendering.
        let _ = doc;
    }

    Ok(())
}

fn resolve_bindings(
    run_inputs: &HashMap<String, String>,
    with_map: &HashMap<String, String>,
) -> Result<HashMap<String, serde_json::Value>> {
    let mut out: HashMap<String, serde_json::Value> = HashMap::new();

    // Deterministic iteration: sort keys.
    let mut keys: Vec<&String> = with_map.keys().collect();
    keys.sort();

    for k in keys {
        let template = with_map
            .get(k)
            .ok_or_else(|| anyhow!("missing binding for key '{k}'"))?;
        let expanded = expand_inputs_template(template, run_inputs)?;
        out.insert(k.clone(), serde_json::Value::String(expanded));
    }

    Ok(out)
}

fn expand_inputs_template(template: &str, run_inputs: &HashMap<String, String>) -> Result<String> {
    // Match ${inputs.KEY}
    // KEY may include letters, numbers, underscore, dash.
    let re = Regex::new(r"\$\{inputs\.([A-Za-z0-9_-]+)\}")
        .expect("regex compile must succeed");

    let mut result = String::new();
    let mut last = 0usize;

    for cap in re.captures_iter(template) {
        let m = cap.get(0).expect("match");
        let key = cap.get(1).expect("key").as_str();

        // text before match
        result.push_str(&template[last..m.start()]);

        let val = run_inputs
            .get(key)
            .ok_or_else(|| anyhow!("missing run input '{key}' needed by template '{template}'"))?;
        result.push_str(val);

        last = m.end();
    }

    // trailing text
    result.push_str(&template[last..]);

    Ok(result)
}