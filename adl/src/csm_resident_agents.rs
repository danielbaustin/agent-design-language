//! Provider-backed CSM resident-agent bridge.
//!
//! The runtime crate owns the resident-agent contract. This module adapts ADL's
//! existing provider substrate into that contract so privileged and ordinary
//! CSM agents share one admission path.

use adl_runtime::resident_agent::{
    CsmResidentAgentAffectModel, CsmResidentAgentAuthority, CsmResidentAgentChannels,
    CsmResidentAgentLifecycleState, CsmResidentAgentPolicyGates, CsmResidentAgentProviderBinding,
    CsmResidentAgentSet, CsmResidentAgentSpec, CSM_RESIDENT_AGENT_SCHEMA,
    CSM_RESIDENT_AGENT_SET_SCHEMA,
};
use anyhow::{anyhow, Result};
use serde_json::{json, Value};
use std::collections::HashMap;

use crate::adl;
use crate::provider_substrate::{
    provider_invocation_target_v1, CapabilityModeV1, ProviderInvocationTargetV1,
    ProviderTransportV1,
};

pub const CSM_RESIDENT_AGENTS_STATUS_REF: &str = "csm_resident_agents_status.json";

pub fn resident_agent_set(agent_instance_id: &str) -> Result<CsmResidentAgentSet> {
    let set = CsmResidentAgentSet {
        schema: CSM_RESIDENT_AGENT_SET_SCHEMA.to_string(),
        runtime_owner: "csm".to_string(),
        admission_model: "provider_bound_resident_agents".to_string(),
        provider_entrypoint: "provider_substrate".to_string(),
        agents: vec![
            shepherd_resident_agent(agent_instance_id)?,
            codex_resident_agent(agent_instance_id)?,
            ollama_worker_resident_agent(agent_instance_id)?,
        ],
    };
    set.validate().map_err(|err| anyhow!(err))?;
    Ok(set)
}

pub fn resident_agent_set_status(agent_instance_id: &str) -> Value {
    match resident_agent_set(agent_instance_id) {
        Ok(set) => json!({
            "status": "available",
            "ref": CSM_RESIDENT_AGENTS_STATUS_REF,
            "value": set,
            "proof": {
                "resident_agent_count": 3,
                "privileged_agent": "polis_shepherd_agent",
                "ordinary_agents": ["codex_chatgpt_resident", "local_ollama_resident"],
                "provider_entrypoint": "provider_substrate",
                "affect_model_integrated": true,
                "affect_model_schema": crate::runtime_v2::AFFECT_HAPPINESS_SAFE_TEST_MODEL_SCHEMA_VERSION,
                "affect_invocation_policy": "operational_reasoning_control_only",
                "no_shepherd_bespoke_provider_path": true
            }
        }),
        Err(err) => json!({
            "status": "invalid",
            "ref": CSM_RESIDENT_AGENTS_STATUS_REF,
            "error": err.to_string()
        }),
    }
}

pub fn shepherd_resident_agent_value(agent_instance_id: &str) -> Value {
    match shepherd_resident_agent(agent_instance_id) {
        Ok(agent) => json!(agent),
        Err(err) => json!({
            "schema": CSM_RESIDENT_AGENT_SCHEMA,
            "agent_instance_id": format!("{agent_instance_id}:polis_shepherd_agent"),
            "status": "invalid",
            "error": err.to_string()
        }),
    }
}

pub fn shepherd_resident_agent(agent_instance_id: &str) -> Result<CsmResidentAgentSpec> {
    let target = provider_invocation_target_v1(
        "local_ollama",
        &provider_spec(
            "ollama",
            Some("ollama:gemma4"),
            Some("gemma4:12b-mlx"),
            Some("gemma4:12b-mlx"),
            Some("http://127.0.0.1:11434"),
        ),
        Some("gemma4:12b-mlx"),
    )?;
    resident_agent(
        agent_instance_id,
        "polis_shepherd_agent",
        "Polis Shepherd Agent",
        "privileged_polis_operator",
        CsmResidentAgentAuthority::ShepherdOperator,
        "elevated_operator_agent_for_runtime_preservation_and_recovery_requests",
        target,
    )
}

pub fn codex_resident_agent(agent_instance_id: &str) -> Result<CsmResidentAgentSpec> {
    let target = provider_invocation_target_v1(
        "chatgpt_codex",
        &provider_spec(
            "openai",
            Some("chatgpt:codex"),
            Some("hosted:chatgpt/codex-agent"),
            Some("gpt-5-codex"),
            None,
        ),
        Some("hosted:chatgpt/codex-agent"),
    )?;
    resident_agent(
        agent_instance_id,
        "codex_chatgpt_resident",
        "Codex/ChatGPT Resident Agent",
        "ordinary_runtime_agent",
        CsmResidentAgentAuthority::Ordinary,
        "ordinary_provider_backed_agent_occupying_csm_for_operator_proof",
        target,
    )
}

pub fn ollama_worker_resident_agent(agent_instance_id: &str) -> Result<CsmResidentAgentSpec> {
    let target = provider_invocation_target_v1(
        "local_ollama_qwen",
        &provider_spec(
            "ollama",
            Some("ollama:qwen"),
            Some("qwen3.5:9b"),
            Some("qwen3.5:9b"),
            Some("http://127.0.0.1:11434"),
        ),
        Some("qwen3.5:9b"),
    )?;
    resident_agent(
        agent_instance_id,
        "local_ollama_resident",
        "Local Ollama Resident Agent",
        "ordinary_runtime_agent",
        CsmResidentAgentAuthority::Ordinary,
        "ordinary_local_provider_agent_for_resident_runtime_proof",
        target,
    )
}

fn resident_agent(
    polis_id: &str,
    local_agent_id: &str,
    display_name: &str,
    agent_role: &str,
    authority: CsmResidentAgentAuthority,
    privilege_reason: &str,
    target: ProviderInvocationTargetV1,
) -> Result<CsmResidentAgentSpec> {
    let agent_instance_id = format!("{polis_id}:{local_agent_id}");
    let agent = CsmResidentAgentSpec {
        schema: CSM_RESIDENT_AGENT_SCHEMA.to_string(),
        agent_instance_id: agent_instance_id.clone(),
        display_name: display_name.to_string(),
        agent_role: agent_role.to_string(),
        authority,
        lifecycle_state: CsmResidentAgentLifecycleState::Admitted,
        provider_binding: provider_binding_from_target(target),
        channels: channels(&agent_instance_id),
        policy_gates: policy_gates(),
        affect_model: affect_model()?,
        checkpoint_policy: "periodic_and_agent_requested_with_runtime_min_interval".to_string(),
        lifelog_policy:
            "append_admission_lifecycle_provider_invocation_refusal_recovery_and_affect_model_events".to_string(),
        observability_policy:
            "emit_resident_agent_provider_lifecycle_affect_model_metrics_traces_logs_and_runtime_events"
                .to_string(),
        privilege_reason: privilege_reason.to_string(),
    };
    agent.validate().map_err(|err| anyhow!(err))?;
    Ok(agent)
}

fn provider_binding_from_target(
    target: ProviderInvocationTargetV1,
) -> CsmResidentAgentProviderBinding {
    CsmResidentAgentProviderBinding {
        provider_id: target.provider_id,
        provider_kind: target.provider_kind,
        vendor: target.vendor,
        transport: transport_label(&target.transport).to_string(),
        runtime_surface: target.model_identity.runtime_surface,
        model_ref: target.model_ref,
        provider_model_id: target.provider_model_id,
        tool_calling_mode: capability_mode_label(&target.capabilities.tool_calling.mode)
            .to_string(),
        structured_json_mode: capability_mode_label(&target.capabilities.structured_json.mode)
            .to_string(),
        binding_status: "provider_target_resolved".to_string(),
        source: "provider_substrate".to_string(),
    }
}

fn provider_spec(
    kind: &str,
    profile: Option<&str>,
    default_model: Option<&str>,
    provider_model_id: Option<&str>,
    base_url: Option<&str>,
) -> adl::ProviderSpec {
    let mut config = HashMap::new();
    if let Some(provider_model_id) = provider_model_id {
        config.insert(
            "provider_model_id".to_string(),
            Value::String(provider_model_id.to_string()),
        );
    }
    adl::ProviderSpec {
        id: None,
        profile: profile.map(ToString::to_string),
        kind: kind.to_string(),
        base_url: base_url.map(ToString::to_string),
        default_model: default_model.map(ToString::to_string),
        config,
    }
}

fn channels(agent_instance_id: &str) -> CsmResidentAgentChannels {
    let channel_id = agent_instance_id.replace(':', ".");
    CsmResidentAgentChannels {
        lifecycle: format!("csm.lifecycle.{channel_id}"),
        provider_request: format!("csm.provider_requests.{channel_id}"),
        provider_response: format!("csm.provider_responses.{channel_id}"),
        checkpoint: format!("csm.checkpoint.{channel_id}"),
        observability: format!("csm.observability.{channel_id}"),
        lifelog: format!("csm.lifelog.{channel_id}"),
    }
}

fn policy_gates() -> CsmResidentAgentPolicyGates {
    CsmResidentAgentPolicyGates {
        freedom_gate_required: true,
        cav_required: true,
        constitutional_policy_required: true,
        model_output_advisory_only: true,
    }
}

fn affect_model() -> Result<CsmResidentAgentAffectModel> {
    let model = crate::runtime_v2::affect_happiness_safe_test_model()?;
    Ok(CsmResidentAgentAffectModel {
        schema_version: model.schema_version,
        model_id: model.model_id,
        affect_signal_count: model.consumed_affect_signal_ids.len() as u32,
        wellbeing_dimension_count: model.consumed_wellbeing_dimension_ids.len() as u32,
        safe_test_scenario_count: model.safe_test_scenarios.len() as u32,
        public_claim_boundary_id: model.public_claim_boundary.boundary_id,
        invocation_policy: "operational_reasoning_control_only".to_string(),
        interpretation_boundary:
            "Operational reasoning-control only; not hidden emotion and not subjective happiness."
                .to_string(),
        unsupported_public_claims: model.public_claim_boundary.unsupported_claims,
    })
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn csm_resident_agent_set_admits_shepherd_codex_and_ollama_through_provider_substrate() {
        let set = resident_agent_set("polis-test").expect("resident set");
        assert_eq!(set.agents.len(), 3);
        assert_eq!(set.provider_entrypoint, "provider_substrate");
        let shepherd = set
            .agents
            .iter()
            .find(|agent| agent.agent_role == "privileged_polis_operator")
            .expect("shepherd");
        assert_eq!(shepherd.provider_binding.provider_id, "local_ollama");
        assert_eq!(shepherd.provider_binding.model_ref, "gemma4:12b-mlx");
        assert_eq!(
            shepherd.provider_binding.binding_status,
            "provider_target_resolved"
        );
        assert_eq!(
            shepherd.affect_model.schema_version,
            crate::runtime_v2::AFFECT_HAPPINESS_SAFE_TEST_MODEL_SCHEMA_VERSION
        );
        assert_eq!(
            shepherd.affect_model.invocation_policy,
            "operational_reasoning_control_only"
        );
        assert!(set.agents.iter().all(|agent| agent
            .affect_model
            .unsupported_public_claims
            .contains(&"subjective_happiness".to_string())));
        assert!(set
            .agents
            .iter()
            .any(|agent| agent.provider_binding.provider_id == "chatgpt_codex"));
        assert!(set
            .agents
            .iter()
            .any(|agent| agent.provider_binding.provider_id == "local_ollama_qwen"));
    }

    #[test]
    fn csm_shepherd_is_privileged_by_role_not_by_provider_bypass() {
        let shepherd = shepherd_resident_agent("polis-test").expect("shepherd");
        assert_eq!(
            shepherd.authority,
            CsmResidentAgentAuthority::ShepherdOperator
        );
        assert_eq!(shepherd.provider_binding.source, "provider_substrate");
        assert!(shepherd.policy_gates.freedom_gate_required);
        assert!(shepherd.policy_gates.cav_required);
        assert_eq!(
            shepherd.lifelog_policy,
            "append_admission_lifecycle_provider_invocation_refusal_recovery_and_affect_model_events"
        );
    }
}
