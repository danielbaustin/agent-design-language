//! Runtime-v2 Godel agent runtime contract.
//!
//! This surface binds the WP-11 GHB proof loop into Runtime v2 as executable
//! runtime planning truth: independent agents, provider targets, scheduling
//! bounds, and replay/proof artifact references.

use super::*;
use crate::{
    adl,
    provider_substrate::{
        provider_invocation_target_v1, CapabilityModeV1, ProviderInvocationTargetV1,
        ProviderTransportV1,
    },
};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fs;

pub const RUNTIME_V2_GODEL_AGENT_RUNTIME_SCHEMA: &str = "runtime_v2.godel_agent_runtime.v1";
pub const RUNTIME_V2_GODEL_AGENT_RUNTIME_PATH: &str =
    "runtime_v2/godel_agent_runtime/godel_agent_runtime.json";
pub const RUNTIME_V2_GODEL_AGENT_RUNTIME_TEST_MARKER: &str = "runtime_v2_godel_agent_runtime";
const MIN_GODEL_AGENT_COUNT: usize = 10;
const MAX_GODEL_AGENT_COUNT: usize = 64;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2GodelAgentRuntimePacket {
    pub schema_version: String,
    pub runtime_id: String,
    pub milestone: String,
    pub wp: String,
    pub artifact_path: String,
    pub reasoning_graph_ref: String,
    pub reasoning_graph_id: String,
    pub loop_runtime_ref: String,
    pub loop_runtime_id: String,
    pub ghb_proof_command: String,
    pub scheduling: RuntimeV2GodelSchedulingPolicy,
    pub agents: Vec<RuntimeV2GodelAgentSpec>,
    pub provider_registry: Vec<RuntimeV2GodelProviderBinding>,
    pub runtime_channels: RuntimeV2GodelRuntimeChannels,
    pub launch_plan: RuntimeV2GodelAgentLaunchPlan,
    pub replay: RuntimeV2GodelRuntimeReplay,
    pub validation_commands: Vec<String>,
    pub claim_boundary: String,
    pub non_claims: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2GodelSchedulingPolicy {
    pub min_independent_agents: u32,
    pub max_independent_agents: u32,
    pub max_concurrent_agents: u32,
    pub backpressure_policy: String,
    pub lifecycle_policy: String,
    pub fairness_policy: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2GodelAgentSpec {
    pub agent_instance_id: String,
    pub agent_role: String,
    pub provider_id: String,
    pub model_ref: String,
    pub loop_runtime_id: String,
    pub reasoning_graph_id: String,
    pub initial_state_id: String,
    pub channel_id: String,
    pub lifecycle_state: String,
    pub evidence_root_ref: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2GodelProviderBinding {
    pub provider_id: String,
    pub provider_kind: String,
    pub vendor: String,
    pub transport: String,
    pub model_ref: String,
    pub provider_model_id: String,
    pub runtime_surface: String,
    pub tool_calling_mode: String,
    pub structured_json_mode: String,
    pub invocation_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2GodelRuntimeChannels {
    pub channel_schema: String,
    pub supervision_channel: String,
    pub lifecycle_channel: String,
    pub provider_request_channel: String,
    pub provider_response_channel: String,
    pub evidence_channel: String,
    pub backpressure_signal: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2GodelAgentLaunchPlan {
    pub schema_version: String,
    pub plan_id: String,
    pub admission_model: String,
    pub runtime_owner: String,
    pub provider_entrypoint: String,
    pub supervision_channel: String,
    pub lifecycle_channel: String,
    pub max_concurrent_agents: u32,
    pub ready_agent_count: u32,
    pub provider_request_count: u32,
    pub policy_gates: RuntimeV2GodelAgentPolicyGates,
    pub provider_requests: Vec<RuntimeV2GodelAgentProviderRequest>,
    pub execution_guarantees: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2GodelAgentPolicyGates {
    pub freedom_gate_required: bool,
    pub cav_required: bool,
    pub constructability_anchor_required: bool,
    pub constitutional_policy_required: bool,
    pub model_output_advisory_only: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2GodelAgentProviderRequest {
    pub agent_instance_id: String,
    pub agent_role: String,
    pub provider_id: String,
    pub model_ref: String,
    pub lifecycle_state: String,
    pub supervision_channel: String,
    pub lifecycle_channel: String,
    pub provider_request_channel: String,
    pub provider_response_channel: String,
    pub evidence_channel: String,
    pub checkpoint_ref: String,
    pub invocation_mode: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2GodelRuntimeReplay {
    pub replay_status: String,
    pub scheduled_agent_count: u32,
    pub provider_binding_count: u32,
    pub independent_agent_ids: Vec<String>,
    pub replay_guarantees: Vec<String>,
}

impl RuntimeV2GodelAgentRuntimePacket {
    pub fn prototype(agent_count: usize) -> Result<Self> {
        let graph = runtime_v2_reasoning_graph_contract()?;
        let loop_runtime = runtime_v2_loop_runtime_contract()?;
        runtime_v2_godel_agent_runtime_contract_for(
            agent_count,
            &graph.graph_id,
            &loop_runtime.runtime_id,
        )
    }

    pub fn validate(&self) -> Result<()> {
        require_exact(
            &self.schema_version,
            RUNTIME_V2_GODEL_AGENT_RUNTIME_SCHEMA,
            "godel_agent_runtime.schema_version",
        )?;
        normalize_id(self.runtime_id.clone(), "godel_agent_runtime.runtime_id")?;
        require_exact(&self.milestone, "v0.91.7", "godel_agent_runtime.milestone")?;
        require_exact(&self.wp, "WP-11", "godel_agent_runtime.wp")?;
        require_exact(
            &self.artifact_path,
            RUNTIME_V2_GODEL_AGENT_RUNTIME_PATH,
            "godel_agent_runtime.artifact_path",
        )?;
        validate_relative_path(&self.artifact_path, "godel_agent_runtime.artifact_path")?;
        validate_relative_path(
            &self.reasoning_graph_ref,
            "godel_agent_runtime.reasoning_graph_ref",
        )?;
        validate_relative_path(
            &self.loop_runtime_ref,
            "godel_agent_runtime.loop_runtime_ref",
        )?;
        normalize_id(
            self.reasoning_graph_id.clone(),
            "godel_agent_runtime.reasoning_graph_id",
        )?;
        normalize_id(
            self.loop_runtime_id.clone(),
            "godel_agent_runtime.loop_runtime_id",
        )?;

        let graph = runtime_v2_reasoning_graph_contract()?;
        let loop_runtime = runtime_v2_loop_runtime_contract()?;
        if self.reasoning_graph_id != graph.graph_id {
            return Err(anyhow!(
                "Godel runtime reasoning graph id must match Runtime v2 reasoning graph"
            ));
        }
        if self.loop_runtime_id != loop_runtime.runtime_id {
            return Err(anyhow!(
                "Godel runtime loop runtime id must match Runtime v2 loop runtime"
            ));
        }
        ensure_contains(
            &self.ghb_proof_command,
            "adl godel ghb-proof",
            "Godel runtime must retain the GHB proof command bridge",
        )?;
        validate_scheduling(&self.scheduling)?;
        validate_scheduling_against_agents(&self.scheduling, &self.agents)?;
        validate_agents(
            &self.agents,
            &self.provider_registry,
            &self.reasoning_graph_id,
            &self.loop_runtime_id,
        )?;
        validate_provider_registry(&self.provider_registry)?;
        validate_channels(&self.runtime_channels)?;
        validate_launch_plan(
            &self.launch_plan,
            &self.scheduling,
            &self.agents,
            &self.provider_registry,
            &self.runtime_channels,
        )?;
        validate_replay(&self.replay, &self.agents, &self.provider_registry)?;
        validate_command_list(&self.validation_commands)?;
        ensure_contains_in_list(
            &self.non_claims,
            "live_hosted_provider_invocation",
            "Godel runtime non-claims must preserve hosted invocation boundary",
        )?;
        ensure_contains(
            &self.claim_boundary,
            "Runtime v2 Godel agent runtime",
            "Godel runtime claim boundary must name Runtime v2 integration",
        )?;
        ensure_contains(
            &self.claim_boundary,
            "10+ independent Godel agents",
            "Godel runtime claim boundary must name multi-agent readiness",
        )
    }

    pub fn canonicalized(&self) -> Result<Self> {
        self.validate()?;
        let mut canonical = self.clone();
        canonical
            .agents
            .sort_by(|a, b| a.agent_instance_id.cmp(&b.agent_instance_id));
        canonical
            .provider_registry
            .sort_by(|a, b| a.provider_id.cmp(&b.provider_id));
        canonical
            .launch_plan
            .provider_requests
            .sort_by(|a, b| a.agent_instance_id.cmp(&b.agent_instance_id));
        canonical.launch_plan.execution_guarantees.sort();
        canonical.replay.independent_agent_ids.sort();
        canonical.replay.replay_guarantees.sort();
        canonical.validation_commands.sort();
        canonical.non_claims.sort();
        canonical.validate()?;
        Ok(canonical)
    }

    pub fn pretty_json_bytes(&self) -> Result<Vec<u8>> {
        serde_json::to_vec_pretty(&self.canonicalized()?)
            .context("serialize Runtime v2 Godel agent runtime packet")
    }

    pub fn write_to_path(&self, path: impl AsRef<Path>) -> Result<()> {
        let path = path.as_ref();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).with_context(|| {
                format!(
                    "create Runtime v2 Godel agent runtime parent '{}'",
                    parent.display()
                )
            })?;
        }
        fs::write(path, self.pretty_json_bytes()?).with_context(|| {
            format!(
                "write Runtime v2 Godel agent runtime packet to '{}'",
                path.display()
            )
        })
    }
}

pub fn runtime_v2_godel_agent_runtime_contract() -> Result<RuntimeV2GodelAgentRuntimePacket> {
    RuntimeV2GodelAgentRuntimePacket::prototype(MIN_GODEL_AGENT_COUNT)
}

pub fn runtime_v2_godel_agent_runtime_contract_for(
    agent_count: usize,
    reasoning_graph_id: &str,
    loop_runtime_id: &str,
) -> Result<RuntimeV2GodelAgentRuntimePacket> {
    if !(MIN_GODEL_AGENT_COUNT..=MAX_GODEL_AGENT_COUNT).contains(&agent_count) {
        return Err(anyhow!(
            "Godel runtime agent count must be between {MIN_GODEL_AGENT_COUNT} and {MAX_GODEL_AGENT_COUNT}"
        ));
    }
    let providers = prototype_provider_targets()?;
    let mut agents = Vec::with_capacity(agent_count);
    for idx in 0..agent_count {
        let provider = &providers[idx % providers.len()];
        let n = idx + 1;
        agents.push(RuntimeV2GodelAgentSpec {
            agent_instance_id: format!("godel-agent-{n:02}"),
            agent_role: if idx == 0 {
                "shepherd_candidate".to_string()
            } else {
                "independent_godel_worker".to_string()
            },
            provider_id: provider.provider_id.clone(),
            model_ref: provider.model_ref.clone(),
            loop_runtime_id: loop_runtime_id.to_string(),
            reasoning_graph_id: reasoning_graph_id.to_string(),
            initial_state_id: format!("godel-agent-{n:02}-state-0001"),
            channel_id: format!("godel-agent-{n:02}-runtime-channel"),
            lifecycle_state: "ready".to_string(),
            evidence_root_ref: format!("runtime_v2/godel_agent_runtime/agents/godel-agent-{n:02}"),
        });
    }
    let runtime_channels = RuntimeV2GodelRuntimeChannels {
        channel_schema: "runtime_v2.godel_agent_channels.v1".to_string(),
        supervision_channel: "csm.supervision.godel_agents".to_string(),
        lifecycle_channel: "csm.lifecycle.godel_agents".to_string(),
        provider_request_channel: "csm.provider_requests.godel_agents".to_string(),
        provider_response_channel: "csm.provider_responses.godel_agents".to_string(),
        evidence_channel: "csm.evidence.godel_agents".to_string(),
        backpressure_signal: "provider_request_queue_depth_and_agent_join_set_capacity".to_string(),
    };
    let scheduling = RuntimeV2GodelSchedulingPolicy {
        min_independent_agents: MIN_GODEL_AGENT_COUNT as u32,
        max_independent_agents: MAX_GODEL_AGENT_COUNT as u32,
        max_concurrent_agents: 10,
        backpressure_policy: "bounded_join_set_with_provider_request_backpressure".to_string(),
        lifecycle_policy: "supervised_agent_runtime_start_run_checkpoint_stop".to_string(),
        fairness_policy: "deterministic_round_robin_provider_target_assignment".to_string(),
    };
    let launch_plan = godel_agent_launch_plan(&agents, &providers, &runtime_channels, &scheduling)?;
    let replay = RuntimeV2GodelRuntimeReplay {
        replay_status: "deterministic_schedule_ready".to_string(),
        scheduled_agent_count: agents.len() as u32,
        provider_binding_count: providers.len() as u32,
        independent_agent_ids: agents
            .iter()
            .map(|agent| agent.agent_instance_id.clone())
            .collect(),
        replay_guarantees: vec![
            "agent ids are deterministic and unique".to_string(),
            "provider bindings are resolved before agent admission".to_string(),
            "scheduler bounds concurrent Godel agents before provider requests are admitted"
                .to_string(),
            "each Godel agent is bound to the Runtime v2 reasoning graph and loop runtime"
                .to_string(),
        ],
    };
    let packet = RuntimeV2GodelAgentRuntimePacket {
        schema_version: RUNTIME_V2_GODEL_AGENT_RUNTIME_SCHEMA.to_string(),
        runtime_id: "runtime-v2-godel-agent-runtime-v0-91-7-wp-11".to_string(),
        milestone: "v0.91.7".to_string(),
        wp: "WP-11".to_string(),
        artifact_path: RUNTIME_V2_GODEL_AGENT_RUNTIME_PATH.to_string(),
        reasoning_graph_ref: RUNTIME_V2_REASONING_GRAPH_PATH.to_string(),
        reasoning_graph_id: reasoning_graph_id.to_string(),
        loop_runtime_ref: RUNTIME_V2_LOOP_RUNTIME_PATH.to_string(),
        loop_runtime_id: loop_runtime_id.to_string(),
        ghb_proof_command: "adl godel ghb-proof --out <proof-dir> --json".to_string(),
        scheduling,
        agents,
        provider_registry: providers,
        runtime_channels,
        launch_plan,
        replay,
        validation_commands: vec![
            "cargo fmt --manifest-path adl/Cargo.toml --all -- --check".to_string(),
            format!(
                "cargo test --manifest-path adl/Cargo.toml {} -- --nocapture",
                RUNTIME_V2_GODEL_AGENT_RUNTIME_TEST_MARKER
            ),
            "cargo test --manifest-path adl/Cargo.toml ghb_loop -- --nocapture".to_string(),
            "cargo test --manifest-path adl/Cargo.toml trace_runtime_v2_godel_agent_runtime -- --nocapture".to_string(),
            "cargo run --manifest-path adl/Cargo.toml --bin adl -- runtime-v2 godel-agent-runtime --agents 10 --out <path>".to_string(),
            "git diff --check".to_string(),
        ],
        claim_boundary: "WP-11 #5136 integrates GHB into Runtime v2 as a Runtime v2 Godel agent runtime plan for 10+ independent Godel agents with provider target binding, supervised lifecycle channels, bounded concurrency, executable provider-request admission, and deterministic replay. It prepares the runtime/provider mechanism needed to run agents without claiming live hosted model calls until an explicit provider executor invokes them.".to_string(),
        non_claims: vec![
            "not_unbounded_recursive_self_improvement".to_string(),
            "not_live_hosted_provider_invocation".to_string(),
            "not_source_code_mutation_without_review".to_string(),
            "not_credential_capture".to_string(),
            "not_private_prompt_persistence".to_string(),
            "not_v092_adaptive_learning_dag_completion".to_string(),
        ],
    };
    packet.validate()?;
    Ok(packet)
}

fn godel_agent_launch_plan(
    agents: &[RuntimeV2GodelAgentSpec],
    providers: &[RuntimeV2GodelProviderBinding],
    channels: &RuntimeV2GodelRuntimeChannels,
    scheduling: &RuntimeV2GodelSchedulingPolicy,
) -> Result<RuntimeV2GodelAgentLaunchPlan> {
    let provider_by_id: BTreeMap<&str, &RuntimeV2GodelProviderBinding> = providers
        .iter()
        .map(|provider| (provider.provider_id.as_str(), provider))
        .collect();
    let mut provider_requests = Vec::with_capacity(agents.len());
    for agent in agents {
        let provider = provider_by_id
            .get(agent.provider_id.as_str())
            .ok_or_else(|| anyhow!("Godel launch plan agent references unknown provider"))?;
        provider_requests.push(RuntimeV2GodelAgentProviderRequest {
            agent_instance_id: agent.agent_instance_id.clone(),
            agent_role: agent.agent_role.clone(),
            provider_id: agent.provider_id.clone(),
            model_ref: provider.model_ref.clone(),
            lifecycle_state: agent.lifecycle_state.clone(),
            supervision_channel: format!(
                "{}.{}",
                channels.supervision_channel, agent.agent_instance_id
            ),
            lifecycle_channel: format!(
                "{}.{}",
                channels.lifecycle_channel, agent.agent_instance_id
            ),
            provider_request_channel: format!(
                "{}.{}",
                channels.provider_request_channel, agent.agent_instance_id
            ),
            provider_response_channel: format!(
                "{}.{}",
                channels.provider_response_channel, agent.agent_instance_id
            ),
            evidence_channel: format!("{}.{}", channels.evidence_channel, agent.agent_instance_id),
            checkpoint_ref: format!("{}/checkpoint-0001.json", agent.evidence_root_ref),
            invocation_mode: "admitted_provider_request_not_invoked".to_string(),
        });
    }
    let launch_plan = RuntimeV2GodelAgentLaunchPlan {
        schema_version: "runtime_v2.godel_agent_launch_plan.v1".to_string(),
        plan_id: "runtime-v2-godel-agent-launch-plan-v0-91-7".to_string(),
        admission_model: "csm_supervised_provider_request_admission".to_string(),
        runtime_owner: "csm".to_string(),
        provider_entrypoint: "provider_substrate".to_string(),
        supervision_channel: channels.supervision_channel.clone(),
        lifecycle_channel: channels.lifecycle_channel.clone(),
        max_concurrent_agents: scheduling.max_concurrent_agents,
        ready_agent_count: agents.len() as u32,
        provider_request_count: provider_requests.len() as u32,
        policy_gates: RuntimeV2GodelAgentPolicyGates {
            freedom_gate_required: true,
            cav_required: true,
            constructability_anchor_required: true,
            constitutional_policy_required: true,
            model_output_advisory_only: true,
        },
        provider_requests,
        execution_guarantees: vec![
            "every Godel agent has a concrete provider request channel".to_string(),
            "every Godel agent has a concrete provider response channel".to_string(),
            "every Godel agent remains supervised by the CSM lifecycle channel".to_string(),
            "provider requests are admitted only after Freedom Gate, CAV, constructability, and constitutional policy gates".to_string(),
            "provider targets are resolved but not invoked by this launch plan".to_string(),
        ],
    };
    validate_launch_plan(&launch_plan, scheduling, agents, providers, channels)?;
    Ok(launch_plan)
}

fn prototype_provider_targets() -> Result<Vec<RuntimeV2GodelProviderBinding>> {
    let specs = vec![
        (
            "local_qwen",
            adl_provider(
                "ollama",
                Some("ollama:qwen"),
                Some("qwen2.5-coder:latest"),
                None,
            ),
        ),
        (
            "local_gemma",
            adl_provider("ollama", Some("ollama:gemma"), Some("gemma4:latest"), None),
        ),
        (
            "bedrock_nova_pro",
            adl_provider(
                "bedrock",
                Some("bedrock:nova-pro"),
                Some("hosted:bedrock/nova-pro"),
                Some("amazon.nova-pro-v1:0"),
            ),
        ),
        (
            "z_ai_glm",
            adl_provider(
                "z_ai",
                Some("z_ai:glm"),
                Some("hosted:z-ai/glm"),
                Some("glm-5"),
            ),
        ),
        (
            "fable_5",
            adl_provider(
                "anthropic",
                Some("claude:fable-5"),
                Some("hosted:anthropic/fable-5"),
                Some("fable-5"),
            ),
        ),
    ];
    specs
        .into_iter()
        .map(|(provider_id, spec)| {
            provider_invocation_target_v1(provider_id, &spec, None)
                .map(|target| provider_binding_from_target(provider_id, target))
        })
        .collect()
}

fn adl_provider(
    kind: &str,
    profile: Option<&str>,
    default_model: Option<&str>,
    provider_model_id: Option<&str>,
) -> adl::ProviderSpec {
    let mut config = HashMap::new();
    if let Some(provider_model_id) = provider_model_id {
        config.insert(
            "provider_model_id".to_string(),
            serde_json::Value::String(provider_model_id.to_string()),
        );
    }
    adl::ProviderSpec {
        id: None,
        profile: profile.map(ToString::to_string),
        kind: kind.to_string(),
        base_url: None,
        default_model: default_model.map(ToString::to_string),
        config,
    }
}

fn provider_binding_from_target(
    provider_id: &str,
    target: ProviderInvocationTargetV1,
) -> RuntimeV2GodelProviderBinding {
    RuntimeV2GodelProviderBinding {
        provider_id: provider_id.to_string(),
        provider_kind: target.provider_kind,
        vendor: target.vendor,
        transport: transport_label(&target.transport).to_string(),
        model_ref: target.model_ref,
        provider_model_id: target.provider_model_id,
        runtime_surface: target.model_identity.runtime_surface,
        tool_calling_mode: capability_mode_label(&target.capabilities.tool_calling.mode)
            .to_string(),
        structured_json_mode: capability_mode_label(&target.capabilities.structured_json.mode)
            .to_string(),
        invocation_status: "provider_target_resolved_not_invoked".to_string(),
    }
}

fn transport_label(transport: &ProviderTransportV1) -> &'static str {
    match transport {
        ProviderTransportV1::Http => "http",
        ProviderTransportV1::LocalCli => "local_cli",
        ProviderTransportV1::InProcess => "in_process",
    }
}

fn capability_mode_label(mode: &CapabilityModeV1) -> &'static str {
    match mode {
        CapabilityModeV1::Native => "native",
        CapabilityModeV1::PromptBased => "prompt_based",
        CapabilityModeV1::SemanticFallback => "semantic_fallback",
        CapabilityModeV1::None => "none",
    }
}

fn validate_scheduling(policy: &RuntimeV2GodelSchedulingPolicy) -> Result<()> {
    if policy.min_independent_agents < MIN_GODEL_AGENT_COUNT as u32 {
        return Err(anyhow!(
            "Godel runtime must support at least {MIN_GODEL_AGENT_COUNT} independent agents"
        ));
    }
    if policy.max_independent_agents < policy.min_independent_agents
        || policy.max_independent_agents > MAX_GODEL_AGENT_COUNT as u32
    {
        return Err(anyhow!(
            "Godel runtime max independent agents must be within supported bounds"
        ));
    }
    if policy.max_concurrent_agents == 0
        || policy.max_concurrent_agents > policy.max_independent_agents
    {
        return Err(anyhow!(
            "Godel runtime max concurrent agents must be positive and bounded"
        ));
    }
    validate_nonempty_text(
        &policy.backpressure_policy,
        "godel_agent_runtime.backpressure_policy",
    )?;
    validate_nonempty_text(
        &policy.lifecycle_policy,
        "godel_agent_runtime.lifecycle_policy",
    )?;
    validate_nonempty_text(
        &policy.fairness_policy,
        "godel_agent_runtime.fairness_policy",
    )
}

fn validate_scheduling_against_agents(
    policy: &RuntimeV2GodelSchedulingPolicy,
    agents: &[RuntimeV2GodelAgentSpec],
) -> Result<()> {
    let agent_count = agents.len() as u32;
    if agent_count < policy.min_independent_agents {
        return Err(anyhow!(
            "Godel runtime actual agent count is below scheduling minimum"
        ));
    }
    if agent_count > policy.max_independent_agents {
        return Err(anyhow!(
            "Godel runtime actual agent count exceeds scheduling maximum"
        ));
    }
    if policy.max_concurrent_agents > agent_count {
        return Err(anyhow!(
            "Godel runtime max concurrent agents cannot exceed actual agent count"
        ));
    }
    Ok(())
}

fn validate_agents(
    agents: &[RuntimeV2GodelAgentSpec],
    providers: &[RuntimeV2GodelProviderBinding],
    reasoning_graph_id: &str,
    loop_runtime_id: &str,
) -> Result<()> {
    if agents.len() < MIN_GODEL_AGENT_COUNT {
        return Err(anyhow!(
            "Godel runtime must include at least {MIN_GODEL_AGENT_COUNT} independent agents"
        ));
    }
    let provider_ids: BTreeSet<&str> = providers
        .iter()
        .map(|provider| provider.provider_id.as_str())
        .collect();
    let provider_by_id: BTreeMap<&str, &RuntimeV2GodelProviderBinding> = providers
        .iter()
        .map(|provider| (provider.provider_id.as_str(), provider))
        .collect();
    let mut ids = BTreeSet::new();
    let mut channel_ids = BTreeSet::new();
    for agent in agents {
        normalize_id(
            agent.agent_instance_id.clone(),
            "godel_agent_runtime.agent_instance_id",
        )?;
        if !ids.insert(agent.agent_instance_id.as_str()) {
            return Err(anyhow!("Godel runtime agent ids must be unique"));
        }
        if !provider_ids.contains(agent.provider_id.as_str()) {
            return Err(anyhow!(
                "Godel runtime agent '{}' references unknown provider '{}'",
                agent.agent_instance_id,
                agent.provider_id
            ));
        }
        let provider = provider_by_id
            .get(agent.provider_id.as_str())
            .expect("provider existence checked above");
        if agent.model_ref != provider.model_ref {
            return Err(anyhow!(
                "Godel runtime agent '{}' model_ref must match provider '{}' target",
                agent.agent_instance_id,
                agent.provider_id
            ));
        }
        if agent.reasoning_graph_id != reasoning_graph_id {
            return Err(anyhow!(
                "Godel runtime agent reasoning graph binding mismatch"
            ));
        }
        if agent.loop_runtime_id != loop_runtime_id {
            return Err(anyhow!("Godel runtime agent loop runtime binding mismatch"));
        }
        normalize_id(
            agent.initial_state_id.clone(),
            "godel_agent_runtime.initial_state_id",
        )?;
        if !channel_ids.insert(agent.channel_id.as_str()) {
            return Err(anyhow!("Godel runtime agent channel ids must be unique"));
        }
        validate_nonempty_text(&agent.agent_role, "godel_agent_runtime.agent_role")?;
        validate_nonempty_text(&agent.model_ref, "godel_agent_runtime.model_ref")?;
        validate_nonempty_text(
            &agent.lifecycle_state,
            "godel_agent_runtime.lifecycle_state",
        )?;
        validate_relative_path(
            &agent.evidence_root_ref,
            "godel_agent_runtime.evidence_root_ref",
        )?;
    }
    Ok(())
}

fn validate_provider_registry(providers: &[RuntimeV2GodelProviderBinding]) -> Result<()> {
    if providers.len() < 3 {
        return Err(anyhow!(
            "Godel runtime provider registry must include local and hosted provider choices"
        ));
    }
    let mut ids = BTreeSet::new();
    let mut vendors = BTreeSet::new();
    for provider in providers {
        normalize_id(
            provider.provider_id.clone(),
            "godel_agent_runtime.provider_id",
        )?;
        if !ids.insert(provider.provider_id.as_str()) {
            return Err(anyhow!("Godel runtime provider ids must be unique"));
        }
        vendors.insert(provider.vendor.as_str());
        validate_nonempty_text(&provider.provider_kind, "godel_agent_runtime.provider_kind")?;
        validate_nonempty_text(&provider.vendor, "godel_agent_runtime.vendor")?;
        validate_nonempty_text(&provider.transport, "godel_agent_runtime.transport")?;
        validate_nonempty_text(&provider.model_ref, "godel_agent_runtime.model_ref")?;
        validate_nonempty_text(
            &provider.provider_model_id,
            "godel_agent_runtime.provider_model_id",
        )?;
        validate_nonempty_text(
            &provider.runtime_surface,
            "godel_agent_runtime.runtime_surface",
        )?;
        require_exact(
            &provider.invocation_status,
            "provider_target_resolved_not_invoked",
            "godel_agent_runtime.invocation_status",
        )?;
    }
    for required in ["ollama", "aws_bedrock", "z_ai", "anthropic"] {
        if !vendors.contains(required) {
            return Err(anyhow!(
                "Godel runtime provider registry must include vendor '{required}'"
            ));
        }
    }
    Ok(())
}

fn validate_channels(channels: &RuntimeV2GodelRuntimeChannels) -> Result<()> {
    validate_nonempty_text(
        &channels.channel_schema,
        "godel_agent_runtime.channel_schema",
    )?;
    validate_nonempty_text(
        &channels.supervision_channel,
        "godel_agent_runtime.supervision_channel",
    )?;
    validate_nonempty_text(
        &channels.lifecycle_channel,
        "godel_agent_runtime.lifecycle_channel",
    )?;
    validate_nonempty_text(
        &channels.provider_request_channel,
        "godel_agent_runtime.provider_request_channel",
    )?;
    validate_nonempty_text(
        &channels.provider_response_channel,
        "godel_agent_runtime.provider_response_channel",
    )?;
    validate_nonempty_text(
        &channels.evidence_channel,
        "godel_agent_runtime.evidence_channel",
    )?;
    validate_nonempty_text(
        &channels.backpressure_signal,
        "godel_agent_runtime.backpressure_signal",
    )
}

fn validate_launch_plan(
    launch_plan: &RuntimeV2GodelAgentLaunchPlan,
    scheduling: &RuntimeV2GodelSchedulingPolicy,
    agents: &[RuntimeV2GodelAgentSpec],
    providers: &[RuntimeV2GodelProviderBinding],
    channels: &RuntimeV2GodelRuntimeChannels,
) -> Result<()> {
    require_exact(
        &launch_plan.schema_version,
        "runtime_v2.godel_agent_launch_plan.v1",
        "godel_agent_runtime.launch_plan.schema_version",
    )?;
    normalize_id(
        launch_plan.plan_id.clone(),
        "godel_agent_runtime.launch_plan.plan_id",
    )?;
    require_exact(
        &launch_plan.admission_model,
        "csm_supervised_provider_request_admission",
        "godel_agent_runtime.launch_plan.admission_model",
    )?;
    require_exact(
        &launch_plan.runtime_owner,
        "csm",
        "godel_agent_runtime.launch_plan.runtime_owner",
    )?;
    require_exact(
        &launch_plan.provider_entrypoint,
        "provider_substrate",
        "godel_agent_runtime.launch_plan.provider_entrypoint",
    )?;
    require_exact(
        &launch_plan.supervision_channel,
        &channels.supervision_channel,
        "godel_agent_runtime.launch_plan.supervision_channel",
    )?;
    require_exact(
        &launch_plan.lifecycle_channel,
        &channels.lifecycle_channel,
        "godel_agent_runtime.launch_plan.lifecycle_channel",
    )?;
    if launch_plan.max_concurrent_agents != scheduling.max_concurrent_agents {
        return Err(anyhow!(
            "Godel launch plan max concurrency must match scheduling policy"
        ));
    }
    if launch_plan.ready_agent_count != agents.len() as u32
        || launch_plan.ready_agent_count < MIN_GODEL_AGENT_COUNT as u32
    {
        return Err(anyhow!(
            "Godel launch plan must admit the full 10+ ready-agent set"
        ));
    }
    if launch_plan.provider_request_count != launch_plan.provider_requests.len() as u32
        || launch_plan.provider_request_count != agents.len() as u32
    {
        return Err(anyhow!(
            "Godel launch plan provider request count must match admitted agents"
        ));
    }
    if !launch_plan.policy_gates.freedom_gate_required
        || !launch_plan.policy_gates.cav_required
        || !launch_plan.policy_gates.constructability_anchor_required
        || !launch_plan.policy_gates.constitutional_policy_required
        || !launch_plan.policy_gates.model_output_advisory_only
    {
        return Err(anyhow!(
            "Godel launch plan must require Freedom Gate, CAV, constructability, constitutional policy, and advisory-only model output"
        ));
    }

    let agents_by_id: BTreeMap<&str, &RuntimeV2GodelAgentSpec> = agents
        .iter()
        .map(|agent| (agent.agent_instance_id.as_str(), agent))
        .collect();
    let providers_by_id: BTreeMap<&str, &RuntimeV2GodelProviderBinding> = providers
        .iter()
        .map(|provider| (provider.provider_id.as_str(), provider))
        .collect();
    let mut request_agent_ids = BTreeSet::new();
    for request in &launch_plan.provider_requests {
        normalize_id(
            request.agent_instance_id.clone(),
            "godel_agent_runtime.launch_plan.provider_request.agent_instance_id",
        )?;
        if !request_agent_ids.insert(request.agent_instance_id.as_str()) {
            return Err(anyhow!(
                "Godel launch plan provider requests must have unique agent ids"
            ));
        }
        let agent = agents_by_id
            .get(request.agent_instance_id.as_str())
            .ok_or_else(|| anyhow!("Godel launch plan references unknown agent"))?;
        let provider = providers_by_id
            .get(request.provider_id.as_str())
            .ok_or_else(|| anyhow!("Godel launch plan references unknown provider"))?;
        require_exact(
            &request.provider_id,
            &agent.provider_id,
            "godel_agent_runtime.launch_plan.provider_request.provider_id",
        )?;
        require_exact(
            &request.agent_role,
            &agent.agent_role,
            "godel_agent_runtime.launch_plan.provider_request.agent_role",
        )?;
        require_exact(
            &request.model_ref,
            &agent.model_ref,
            "godel_agent_runtime.launch_plan.provider_request.agent_model_ref",
        )?;
        require_exact(
            &request.model_ref,
            &provider.model_ref,
            "godel_agent_runtime.launch_plan.provider_request.model_ref",
        )?;
        require_exact(
            &request.lifecycle_state,
            "ready",
            "godel_agent_runtime.launch_plan.provider_request.lifecycle_state",
        )?;
        require_exact(
            &request.invocation_mode,
            "admitted_provider_request_not_invoked",
            "godel_agent_runtime.launch_plan.provider_request.invocation_mode",
        )?;
        for (value, prefix, field) in [
            (
                &request.supervision_channel,
                &channels.supervision_channel,
                "supervision_channel",
            ),
            (
                &request.lifecycle_channel,
                &channels.lifecycle_channel,
                "lifecycle_channel",
            ),
            (
                &request.provider_request_channel,
                &channels.provider_request_channel,
                "provider_request_channel",
            ),
            (
                &request.provider_response_channel,
                &channels.provider_response_channel,
                "provider_response_channel",
            ),
            (
                &request.evidence_channel,
                &channels.evidence_channel,
                "evidence_channel",
            ),
        ] {
            if !value.starts_with(prefix) || !value.ends_with(&request.agent_instance_id) {
                return Err(anyhow!(
                    "Godel launch plan provider request {field} must be agent-scoped"
                ));
            }
        }
        validate_relative_path(
            &request.checkpoint_ref,
            "godel_agent_runtime.launch_plan.provider_request.checkpoint_ref",
        )?;
    }
    let expected_agent_ids: BTreeSet<&str> = agents
        .iter()
        .map(|agent| agent.agent_instance_id.as_str())
        .collect();
    if request_agent_ids != expected_agent_ids {
        return Err(anyhow!(
            "Godel launch plan provider requests must cover every admitted agent"
        ));
    }
    require_fields(
        &launch_plan.execution_guarantees,
        &[
            "concrete provider request channel",
            "provider response channel",
            "CSM lifecycle channel",
            "Freedom Gate, CAV, constructability",
            "resolved but not invoked",
        ],
        "godel_agent_runtime.launch_plan.execution_guarantees",
    )
}

fn validate_replay(
    replay: &RuntimeV2GodelRuntimeReplay,
    agents: &[RuntimeV2GodelAgentSpec],
    providers: &[RuntimeV2GodelProviderBinding],
) -> Result<()> {
    require_exact(
        &replay.replay_status,
        "deterministic_schedule_ready",
        "godel_agent_runtime.replay_status",
    )?;
    if replay.scheduled_agent_count != agents.len() as u32 {
        return Err(anyhow!(
            "Godel runtime replay scheduled agent count must match agents"
        ));
    }
    if replay.provider_binding_count != providers.len() as u32 {
        return Err(anyhow!(
            "Godel runtime replay provider binding count must match registry"
        ));
    }
    let expected: BTreeSet<&str> = agents
        .iter()
        .map(|agent| agent.agent_instance_id.as_str())
        .collect();
    let observed: BTreeSet<&str> = replay
        .independent_agent_ids
        .iter()
        .map(String::as_str)
        .collect();
    if expected != observed {
        return Err(anyhow!(
            "Godel runtime replay independent agent ids must match agents"
        ));
    }
    validate_requirement_list(
        &replay.replay_guarantees,
        "godel_agent_runtime.replay_guarantees",
    )
}

fn validate_command_list(commands: &[String]) -> Result<()> {
    if commands.is_empty() {
        return Err(anyhow!(
            "Godel runtime validation commands must not be empty"
        ));
    }
    for command in commands {
        validate_nonempty_text(command, "godel_agent_runtime.validation_commands")?;
    }
    Ok(())
}

fn validate_requirement_list(values: &[String], field: &str) -> Result<()> {
    if values.is_empty() {
        return Err(anyhow!("{field} must not be empty"));
    }
    for value in values {
        validate_nonempty_text(value, field)?;
    }
    Ok(())
}

fn require_fields(values: &[String], required_fragments: &[&str], field_name: &str) -> Result<()> {
    for fragment in required_fragments {
        if !values.iter().any(|value| value.contains(fragment)) {
            return Err(anyhow!(
                "{field_name} must contain required fragment '{fragment}'"
            ));
        }
    }
    Ok(())
}

fn ensure_contains_in_list(values: &[String], needle: &str, message: &str) -> Result<()> {
    if values.iter().any(|value| value.contains(needle)) {
        Ok(())
    } else {
        Err(anyhow!("{message}"))
    }
}

fn ensure_contains(value: &str, needle: &str, message: &str) -> Result<()> {
    if value.contains(needle) {
        Ok(())
    } else {
        Err(anyhow!("{message}"))
    }
}

fn require_exact(actual: &str, expected: &str, field: &str) -> Result<()> {
    if actual == expected {
        Ok(())
    } else {
        Err(anyhow!("{field} must be '{expected}', got '{actual}'"))
    }
}

pub fn runtime_v2_godel_provider_summary(
    packet: &RuntimeV2GodelAgentRuntimePacket,
) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();
    for agent in &packet.agents {
        *counts.entry(agent.provider_id.clone()).or_insert(0) += 1;
    }
    counts
}
