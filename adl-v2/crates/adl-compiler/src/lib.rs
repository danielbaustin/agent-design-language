//! Pure, deterministic lowering from validated ADL documents to inert plans.

use adl_language::{AdlDocument, Diagnostic as LanguageDiagnostic, Workflow, WorkflowKind};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

pub const EXECUTION_PLAN_VERSION: &str = "adl.execution-plan.v1";
const NODE_ID_DOMAIN: &[u8] = b"adl.compiler.node-id.v1\0";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompilerLimits {
    pub max_nodes: usize,
    pub max_edges: usize,
    pub max_input_depth: usize,
    pub max_input_values: usize,
}

impl Default for CompilerLimits {
    fn default() -> Self {
        Self {
            max_nodes: 10_000,
            max_edges: 100_000,
            max_input_depth: 64,
            max_input_values: 1_000_000,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompilerDiagnosticCode {
    InvalidDocument,
    LimitExceeded,
    IdentityCollision,
    DependencyCycle,
    InternalInvariant,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct CompilerDiagnostic {
    pub code: CompilerDiagnosticCode,
    pub path: String,
    pub message: String,
}

impl fmt::Display for CompilerDiagnostic {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.path, self.message)
    }
}

impl std::error::Error for CompilerDiagnostic {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExecutionPlan {
    pub contract: String,
    pub source_digest: String,
    pub run: PlanRun,
    pub workflow: PlanWorkflow,
    pub nodes: Vec<PlanNode>,
    pub edges: Vec<PlanEdge>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlanRun {
    pub identity: String,
    pub name: String,
    pub inputs: BTreeMap<String, Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_target: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlanWorkflow {
    pub identity: String,
    pub kind: WorkflowKind,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlanNode {
    pub id: String,
    pub step_id: String,
    pub task_ref: String,
    pub agent_ref: String,
    pub provider_ref: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    pub tools: Vec<String>,
    pub ports: PlanPorts,
    pub prompt: PlanPrompt,
    pub inputs: BTreeMap<String, Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub save_as: Option<String>,
    pub provenance: PlanProvenance,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlanPorts {
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlanPrompt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
    pub user: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlanProvenance {
    pub document_version: String,
    pub workflow_identity: String,
    pub semantic_path: String,
    pub task_ref: String,
    pub agent_ref: String,
    pub provider_ref: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct PlanEdge {
    pub from: String,
    pub to: String,
    pub kind: PlanEdgeKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlanEdgeKind {
    Sequential,
    StateDependency,
}

pub fn compile(document: &AdlDocument) -> Result<ExecutionPlan, Vec<CompilerDiagnostic>> {
    compile_with_limits(document, CompilerLimits::default())
}

pub fn compile_with_limits(
    document: &AdlDocument,
    limits: CompilerLimits,
) -> Result<ExecutionPlan, Vec<CompilerDiagnostic>> {
    if let Err(diagnostics) = adl_language::validate(document) {
        return Err(language_diagnostics(diagnostics));
    }
    let (workflow_identity, workflow) = resolve_workflow(document);
    if workflow.steps.len() > limits.max_nodes {
        return Err(vec![diagnostic(
            CompilerDiagnosticCode::LimitExceeded,
            "$.run.workflow",
            format!(
                "workflow has {} nodes; limit is {}",
                workflow.steps.len(),
                limits.max_nodes
            ),
        )]);
    }
    let mut input_value_count = 0;
    for value in document.run.inputs.values() {
        check_value_limits(value, 1, &mut input_value_count, limits, "$.run.inputs")?;
    }
    for (index, step) in workflow.steps.iter().enumerate() {
        for value in step.inputs.values() {
            check_value_limits(
                value,
                1,
                &mut input_value_count,
                limits,
                &format!("$.run.workflow.steps[{index}].inputs"),
            )?;
        }
    }

    let run_identity = document
        .run
        .id
        .clone()
        .unwrap_or_else(|| document.run.name.clone());
    let mut node_ids = BTreeMap::new();
    let mut identities = BTreeSet::new();
    for step in &workflow.steps {
        let semantic_digest = resolved_declaration_digest(document, step)?;
        let id = stable_node_id(
            &run_identity,
            &workflow_identity,
            &step.id,
            &step.task,
            &semantic_digest,
        );
        if !identities.insert(id.clone()) {
            return Err(vec![diagnostic(
                CompilerDiagnosticCode::IdentityCollision,
                format!("$.run.workflow.steps.{}", step.id),
                format!("stable node identity collision `{id}`"),
            )]);
        }
        node_ids.insert(step.id.clone(), id);
    }

    let mut edge_steps = BTreeSet::new();
    if workflow.kind == WorkflowKind::Sequential {
        for pair in workflow.steps.windows(2) {
            insert_edge(
                &mut edge_steps,
                (
                    pair[0].id.clone(),
                    pair[1].id.clone(),
                    PlanEdgeKind::Sequential,
                    None,
                ),
                limits.max_edges,
            )?;
        }
    }
    let state_owners: BTreeMap<_, _> = workflow
        .steps
        .iter()
        .filter_map(|step| {
            step.save_as
                .as_ref()
                .map(|state| (state.clone(), step.id.clone()))
        })
        .collect();
    for step in &workflow.steps {
        let mut references = BTreeSet::new();
        for value in step.inputs.values() {
            collect_state_references(value, &mut references);
        }
        for state in references {
            let owner = state_owners.get(&state).ok_or_else(|| {
                vec![diagnostic(
                    CompilerDiagnosticCode::InternalInvariant,
                    format!("$.run.workflow.steps.{}.inputs", step.id),
                    format!("validated state `{state}` has no owner"),
                )]
            })?;
            insert_edge(
                &mut edge_steps,
                (
                    owner.clone(),
                    step.id.clone(),
                    PlanEdgeKind::StateDependency,
                    Some(state),
                ),
                limits.max_edges,
            )?;
        }
    }
    let order = topological_order(workflow, &edge_steps)?;
    let step_by_id: BTreeMap<_, _> = workflow
        .steps
        .iter()
        .map(|step| (step.id.as_str(), step))
        .collect();
    let mut nodes = Vec::with_capacity(order.len());
    for step_id in order {
        let step = step_by_id[step_id.as_str()];
        let task = &document.tasks[&step.task];
        let agent_ref = step
            .agent
            .as_ref()
            .or(task.agent_ref.as_ref())
            .ok_or_else(|| {
                vec![diagnostic(
                    CompilerDiagnosticCode::InvalidDocument,
                    format!("$.run.workflow.steps.{}.agent", step.id),
                    "step and task do not resolve an agent",
                )]
            })?;
        let agent = &document.agents[agent_ref];
        let provider = &document.providers[&agent.provider];
        let mut tools: BTreeSet<String> = agent.tools.iter().cloned().collect();
        if !task.tool_allowlist.is_empty() {
            tools.retain(|tool| task.tool_allowlist.contains(tool));
        }
        nodes.push(PlanNode {
            id: node_ids[&step.id].clone(),
            step_id: step.id.clone(),
            task_ref: step.task.clone(),
            agent_ref: agent_ref.clone(),
            provider_ref: agent.provider.clone(),
            model: agent
                .model
                .clone()
                .or_else(|| provider.default_model.clone()),
            tools: tools.into_iter().collect(),
            ports: PlanPorts {
                inputs: task.inputs.clone(),
                outputs: step.save_as.iter().cloned().collect(),
            },
            prompt: PlanPrompt {
                system: task.prompt.system.clone(),
                user: task.prompt.user.clone(),
            },
            inputs: step.inputs.clone(),
            save_as: step.save_as.clone(),
            provenance: PlanProvenance {
                document_version: document.version.clone(),
                workflow_identity: workflow_identity.clone(),
                semantic_path: format!("$.run.workflow.steps.{}", step.id),
                task_ref: step.task.clone(),
                agent_ref: agent_ref.clone(),
                provider_ref: agent.provider.clone(),
            },
        });
    }
    let edges = edge_steps
        .into_iter()
        .map(|(from, to, kind, state)| PlanEdge {
            from: node_ids[&from].clone(),
            to: node_ids[&to].clone(),
            kind,
            state,
        })
        .collect();
    let source = adl_language::canonical_bytes(document).map_err(language_diagnostics)?;
    Ok(ExecutionPlan {
        contract: EXECUTION_PLAN_VERSION.to_owned(),
        source_digest: sha256_hex(&source),
        run: PlanRun {
            identity: run_identity,
            name: document.run.name.clone(),
            inputs: document.run.inputs.clone(),
            placement_target: document
                .run
                .placement
                .as_ref()
                .map(|value| value.target.clone()),
        },
        workflow: PlanWorkflow {
            identity: workflow_identity,
            kind: workflow.kind,
        },
        nodes,
        edges,
    })
}

pub fn canonical_plan_bytes(plan: &ExecutionPlan) -> Result<Vec<u8>, serde_json::Error> {
    serde_json::to_vec(plan)
}

pub fn canonical_diagnostic_bytes(
    diagnostics: &[CompilerDiagnostic],
) -> Result<Vec<u8>, serde_json::Error> {
    serde_json::to_vec(diagnostics)
}

fn resolve_workflow(document: &AdlDocument) -> (String, &Workflow) {
    match (&document.run.workflow_ref, &document.run.workflow) {
        (Some(reference), None) => (reference.clone(), &document.workflows[reference]),
        (None, Some(workflow)) => (
            workflow
                .id
                .clone()
                .unwrap_or_else(|| format!("inline:{}", document.run.name)),
            workflow,
        ),
        _ => unreachable!("validated run target"),
    }
}

fn topological_order(
    workflow: &Workflow,
    edges: &BTreeSet<(String, String, PlanEdgeKind, Option<String>)>,
) -> Result<Vec<String>, Vec<CompilerDiagnostic>> {
    let mut incoming: BTreeMap<String, usize> = workflow
        .steps
        .iter()
        .map(|step| (step.id.clone(), 0))
        .collect();
    let mut outgoing: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for (from, to, _, _) in edges {
        if outgoing.entry(from.clone()).or_default().insert(to.clone()) {
            *incoming.get_mut(to).expect("validated edge target") += 1;
        }
    }
    let mut ready: BTreeSet<String> = incoming
        .iter()
        .filter(|(_, count)| **count == 0)
        .map(|(id, _)| id.clone())
        .collect();
    let mut order = Vec::with_capacity(incoming.len());
    while let Some(id) = ready.pop_first() {
        order.push(id.clone());
        for target in outgoing.get(&id).into_iter().flatten() {
            let count = incoming.get_mut(target).expect("validated edge target");
            *count -= 1;
            if *count == 0 {
                ready.insert(target.clone());
            }
        }
    }
    if order.len() != incoming.len() {
        return Err(vec![diagnostic(
            CompilerDiagnosticCode::DependencyCycle,
            "$.run.workflow",
            "workflow dependencies do not form a directed acyclic graph",
        )]);
    }
    Ok(order)
}

fn insert_edge(
    edges: &mut BTreeSet<(String, String, PlanEdgeKind, Option<String>)>,
    edge: (String, String, PlanEdgeKind, Option<String>),
    max_edges: usize,
) -> Result<(), Vec<CompilerDiagnostic>> {
    if !edges.contains(&edge) && edges.len() >= max_edges {
        return Err(vec![diagnostic(
            CompilerDiagnosticCode::LimitExceeded,
            "$.run.workflow",
            format!("workflow exceeds edge limit {max_edges}"),
        )]);
    }
    edges.insert(edge);
    Ok(())
}

fn check_value_limits(
    value: &Value,
    depth: usize,
    count: &mut usize,
    limits: CompilerLimits,
    path: &str,
) -> Result<(), Vec<CompilerDiagnostic>> {
    *count += 1;
    if depth > limits.max_input_depth || *count > limits.max_input_values {
        return Err(vec![diagnostic(
            CompilerDiagnosticCode::LimitExceeded,
            path,
            format!(
                "input exceeds depth {} or value-count {} limit",
                limits.max_input_depth, limits.max_input_values
            ),
        )]);
    }
    match value {
        Value::Array(values) => {
            for value in values {
                check_value_limits(value, depth + 1, count, limits, path)?;
            }
        }
        Value::Object(values) => {
            for value in values.values() {
                check_value_limits(value, depth + 1, count, limits, path)?;
            }
        }
        _ => {}
    }
    Ok(())
}

fn collect_state_references(value: &Value, references: &mut BTreeSet<String>) {
    match value {
        Value::String(value) => {
            if let Some(reference) = value.strip_prefix("@state:") {
                references.insert(reference.to_owned());
            }
        }
        Value::Array(values) => {
            for value in values {
                collect_state_references(value, references);
            }
        }
        Value::Object(values) => {
            for value in values.values() {
                collect_state_references(value, references);
            }
        }
        _ => {}
    }
}

fn resolved_declaration_digest(
    document: &AdlDocument,
    step: &adl_language::WorkflowStep,
) -> Result<String, Vec<CompilerDiagnostic>> {
    let task = &document.tasks[&step.task];
    let agent_ref = step
        .agent
        .as_ref()
        .or(task.agent_ref.as_ref())
        .ok_or_else(|| {
            vec![diagnostic(
                CompilerDiagnosticCode::InvalidDocument,
                format!("$.run.workflow.steps.{}.agent", step.id),
                "step and task do not resolve an agent",
            )]
        })?;
    let agent = &document.agents[agent_ref];
    let provider = &document.providers[&agent.provider];
    let resolved_model = agent.model.as_ref().or(provider.default_model.as_ref());
    let mut effective_tools: BTreeSet<&str> = agent.tools.iter().map(String::as_str).collect();
    if !task.tool_allowlist.is_empty() {
        effective_tools.retain(|tool| task.tool_allowlist.iter().any(|allowed| allowed == tool));
    }
    let bytes = serde_json::to_vec(&(
        &step.id,
        &step.task,
        agent_ref,
        &agent.provider,
        resolved_model,
        effective_tools,
        &task.inputs,
        &task.prompt,
        &step.inputs,
        &step.save_as,
    ))
    .map_err(|error| {
        vec![diagnostic(
            CompilerDiagnosticCode::InternalInvariant,
            format!("$.run.workflow.steps.{}", step.id),
            format!("resolved declaration serialization failed: {error}"),
        )]
    })?;
    Ok(sha256_hex(&bytes))
}

fn stable_node_id(
    run: &str,
    workflow: &str,
    step: &str,
    task: &str,
    semantic_digest: &str,
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(NODE_ID_DOMAIN);
    for component in [
        EXECUTION_PLAN_VERSION,
        run,
        workflow,
        step,
        task,
        "task",
        semantic_digest,
    ] {
        hasher.update((component.len() as u64).to_be_bytes());
        hasher.update(component.as_bytes());
    }
    format!("node_v1_{}", hex::encode(hasher.finalize()))
}

fn sha256_hex(bytes: &[u8]) -> String {
    hex::encode(Sha256::digest(bytes))
}

fn language_diagnostics(values: Vec<LanguageDiagnostic>) -> Vec<CompilerDiagnostic> {
    let mut diagnostics: Vec<_> = values
        .into_iter()
        .map(|value| {
            diagnostic(
                CompilerDiagnosticCode::InvalidDocument,
                value.path,
                format!("{:?}: {}", value.code, value.message),
            )
        })
        .collect();
    diagnostics.sort();
    diagnostics
}

fn diagnostic(
    code: CompilerDiagnosticCode,
    path: impl Into<String>,
    message: impl Into<String>,
) -> CompilerDiagnostic {
    CompilerDiagnostic {
        code,
        path: path.into(),
        message: message.into(),
    }
}
