//! Runtime-v2 Godel/constructability boundary for v0.92 claim consumption.
//!
//! WP-13 does not create a new Godel runtime. It composes the retained WP-11
//! Godel agent runtime packet with the WP-10 constructability anchor validator
//! and makes the v0.92 birthday claim boundary executable and reviewable.

use super::*;
use std::collections::BTreeSet;

pub const RUNTIME_V2_GODEL_CONSTRUCTABILITY_BOUNDARY_SCHEMA: &str =
    "runtime_v2.godel_constructability_boundary.v1";
pub const RUNTIME_V2_GODEL_CONSTRUCTABILITY_BOUNDARY_TEST_MARKER: &str =
    "runtime_v2_godel_constructability_boundary";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2GodelConstructabilityBoundaryPacket {
    pub schema_version: String,
    pub boundary_id: String,
    pub milestone: String,
    pub wp: String,
    pub godel_runtime: RuntimeV2GodelBoundaryInput,
    pub constructability_validator: RuntimeV2ConstructabilityBoundaryInput,
    pub v092_allowed_claims: Vec<String>,
    pub v092_prohibited_claims: Vec<String>,
    pub promotion_requirements: Vec<String>,
    pub validation_commands: Vec<String>,
    pub claim_boundary: String,
    pub non_claims: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2GodelBoundaryInput {
    pub schema_version: String,
    pub runtime_id: String,
    pub reasoning_graph_id: String,
    pub loop_runtime_id: String,
    pub agent_count: u32,
    pub provider_binding_count: u32,
    pub launch_plan_status: String,
    pub provider_request_count: u32,
    pub hosted_invocation_status: String,
    pub retained_non_claims: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2ConstructabilityBoundaryInput {
    pub schema_version: String,
    pub validator_id: String,
    pub promotion_requires_anchor: bool,
    pub promotion_requires_validator_pass: bool,
    pub promotion_requires_operator_review: bool,
    pub failure_mode_count: u32,
    pub retained_non_claims: Vec<String>,
}

pub fn runtime_v2_godel_constructability_boundary(
) -> Result<RuntimeV2GodelConstructabilityBoundaryPacket> {
    let godel = runtime_v2_godel_agent_runtime_contract()?;
    let constructability = runtime_v2_constructability_anchor_validator_contract()?;
    let packet = RuntimeV2GodelConstructabilityBoundaryPacket {
        schema_version: RUNTIME_V2_GODEL_CONSTRUCTABILITY_BOUNDARY_SCHEMA.to_string(),
        boundary_id: "runtime-v2-godel-constructability-boundary-v0-91-7-wp-13".to_string(),
        milestone: "v0.91.7".to_string(),
        wp: "WP-13".to_string(),
        godel_runtime: RuntimeV2GodelBoundaryInput {
            schema_version: godel.schema_version.clone(),
            runtime_id: godel.runtime_id.clone(),
            reasoning_graph_id: godel.reasoning_graph_id.clone(),
            loop_runtime_id: godel.loop_runtime_id.clone(),
            agent_count: godel.agents.len() as u32,
            provider_binding_count: godel.provider_registry.len() as u32,
            launch_plan_status: "csm_supervised_provider_request_admission_ready".to_string(),
            provider_request_count: godel.launch_plan.provider_request_count,
            hosted_invocation_status: "provider_target_resolved_not_invoked".to_string(),
            retained_non_claims: godel.non_claims.clone(),
        },
        constructability_validator: RuntimeV2ConstructabilityBoundaryInput {
            schema_version: constructability.schema_version.clone(),
            validator_id: constructability.validator_id.clone(),
            promotion_requires_anchor: constructability
                .shared_reality_boundary
                .promotion_requires_anchor,
            promotion_requires_validator_pass: constructability
                .shared_reality_boundary
                .promotion_requires_validator_pass,
            promotion_requires_operator_review: constructability
                .shared_reality_boundary
                .promotion_requires_operator_review,
            failure_mode_count: constructability.failure_modes.len() as u32,
            retained_non_claims: constructability.non_claims.clone(),
        },
        v092_allowed_claims: vec![
            "v0.92 may describe a bounded Godel-agent birthday as a reviewed Runtime v2 event when the packet cites retained Godel runtime evidence and constructability validation.".to_string(),
            "v0.92 may consume 10+ independent Godel-agent runtime readiness as deterministic scheduling and provider-binding evidence, not live hosted execution evidence.".to_string(),
            "v0.92 may consume the CSM-supervised Godel launch plan as provider-request admission readiness, not hosted-provider invocation proof.".to_string(),
            "v0.92 may promote Godel mechanics into public birthday copy only through constructability anchors, validator pass, and operator review.".to_string(),
        ],
        v092_prohibited_claims: vec![
            "autonomous_self_improvement".to_string(),
            "unbounded_recursive_self_improvement".to_string(),
            "live_hosted_provider_invocation".to_string(),
            "shared_reality_without_constructability_anchor".to_string(),
            "source_code_mutation_without_review".to_string(),
            "v092_adaptive_learning_dag_completion".to_string(),
        ],
        promotion_requirements: vec![
            "retain Runtime v2 Godel agent runtime packet evidence".to_string(),
            "retain Runtime v2 Godel agent launch-plan evidence".to_string(),
            "retain constructability anchor validator packet evidence".to_string(),
            "require constructability anchor before shared-reality promotion".to_string(),
            "require validator pass before public birthday claims".to_string(),
            "require operator review before external/shared publication".to_string(),
            "preserve hosted-provider invocation non-claim until live provider proof exists".to_string(),
        ],
        validation_commands: vec![
            "cargo test --manifest-path adl/Cargo.toml --lib runtime_v2_godel_agent_runtime -- --nocapture".to_string(),
            "cargo test --manifest-path adl/Cargo.toml --lib runtime_v2_godel_constructability_boundary -- --nocapture".to_string(),
            "git diff --check".to_string(),
        ],
        claim_boundary: "WP-13 #4753 proves a Runtime v2 Godel/constructability boundary for v0.92 claim consumption. It consumes the WP-11 Godel agent runtime and WP-10 constructability anchor validator, and permits only evidence-backed birthday claims that preserve provider invocation, constructability, and self-improvement non-claims.".to_string(),
        non_claims: vec![
            "does not implement a new Godel runtime".to_string(),
            "does not claim autonomous self-improvement".to_string(),
            "does not claim live hosted provider invocation".to_string(),
            "does not permit shared-reality promotion without constructability anchors".to_string(),
            "does not complete the v0.92 adaptive learning DAG".to_string(),
        ],
    };
    validate_runtime_v2_godel_constructability_boundary(&packet, &godel, &constructability)?;
    Ok(packet)
}

pub fn runtime_v2_godel_constructability_boundary_json_bytes(
    packet: &RuntimeV2GodelConstructabilityBoundaryPacket,
) -> Result<Vec<u8>> {
    let godel = runtime_v2_godel_agent_runtime_contract()?;
    let constructability = runtime_v2_constructability_anchor_validator_contract()?;
    validate_runtime_v2_godel_constructability_boundary(packet, &godel, &constructability)?;
    let mut canonical = packet.clone();
    canonicalize_runtime_v2_godel_constructability_boundary(&mut canonical);
    serde_json::to_vec_pretty(&canonical)
        .context("serialize Runtime v2 Godel/constructability boundary packet")
}

pub fn validate_runtime_v2_godel_constructability_boundary(
    packet: &RuntimeV2GodelConstructabilityBoundaryPacket,
    godel: &RuntimeV2GodelAgentRuntimePacket,
    constructability: &RuntimeV2ConstructabilityAnchorValidatorPacket,
) -> Result<()> {
    godel.validate()?;
    constructability.validate()?;
    require_exact(
        &packet.schema_version,
        RUNTIME_V2_GODEL_CONSTRUCTABILITY_BOUNDARY_SCHEMA,
        "godel_constructability_boundary.schema_version",
    )?;
    normalize_id(
        packet.boundary_id.clone(),
        "godel_constructability_boundary.boundary_id",
    )?;
    require_exact(
        &packet.milestone,
        "v0.91.7",
        "godel_constructability_boundary.milestone",
    )?;
    require_exact(&packet.wp, "WP-13", "godel_constructability_boundary.wp")?;
    validate_godel_input(&packet.godel_runtime, godel)?;
    validate_constructability_input(&packet.constructability_validator, constructability)?;
    require_fields(
        &packet.v092_allowed_claims,
        &[
            "bounded Godel-agent birthday",
            "10+ independent Godel-agent runtime readiness",
            "CSM-supervised Godel launch plan",
            "constructability anchors",
        ],
        "godel_constructability_boundary.v092_allowed_claims",
    )?;
    require_fields(
        &packet.v092_prohibited_claims,
        &[
            "autonomous_self_improvement",
            "unbounded_recursive_self_improvement",
            "live_hosted_provider_invocation",
            "shared_reality_without_constructability_anchor",
            "source_code_mutation_without_review",
            "v092_adaptive_learning_dag_completion",
        ],
        "godel_constructability_boundary.v092_prohibited_claims",
    )?;
    reject_prohibited_claim_fragments(
        &packet.v092_allowed_claims,
        &packet.v092_prohibited_claims,
        "godel_constructability_boundary.v092_allowed_claims",
    )?;
    require_fields(
        &packet.promotion_requirements,
        &[
            "Runtime v2 Godel agent runtime packet evidence",
            "Runtime v2 Godel agent launch-plan evidence",
            "constructability anchor validator packet evidence",
            "constructability anchor",
            "validator pass",
            "operator review",
            "hosted-provider invocation non-claim",
        ],
        "godel_constructability_boundary.promotion_requirements",
    )?;
    require_fields(
        &packet.validation_commands,
        &[
            "runtime_v2_godel_constructability_boundary",
            "git diff --check",
        ],
        "godel_constructability_boundary.validation_commands",
    )?;
    require_fields(
        &packet.non_claims,
        &[
            "does not implement a new Godel runtime",
            "does not claim autonomous self-improvement",
            "does not claim live hosted provider invocation",
            "does not permit shared-reality promotion without constructability anchors",
            "does not complete the v0.92 adaptive learning DAG",
        ],
        "godel_constructability_boundary.non_claims",
    )?;
    ensure_contains(
        &packet.claim_boundary,
        "Runtime v2 Godel/constructability boundary",
        "Godel/constructability boundary must name the bridge",
    )?;
    ensure_contains(
        &packet.claim_boundary,
        "provider invocation",
        "Godel/constructability boundary must preserve provider invocation non-claim",
    )?;
    ensure_contains(
        &packet.claim_boundary,
        "constructability",
        "Godel/constructability boundary must preserve constructability gate",
    )
}

fn validate_godel_input(
    input: &RuntimeV2GodelBoundaryInput,
    godel: &RuntimeV2GodelAgentRuntimePacket,
) -> Result<()> {
    require_exact(
        &input.schema_version,
        RUNTIME_V2_GODEL_AGENT_RUNTIME_SCHEMA,
        "godel_constructability_boundary.godel.schema_version",
    )?;
    require_exact(
        &input.runtime_id,
        &godel.runtime_id,
        "godel_constructability_boundary.godel.runtime_id",
    )?;
    require_exact(
        &input.reasoning_graph_id,
        &godel.reasoning_graph_id,
        "godel_constructability_boundary.godel.reasoning_graph_id",
    )?;
    require_exact(
        &input.loop_runtime_id,
        &godel.loop_runtime_id,
        "godel_constructability_boundary.godel.loop_runtime_id",
    )?;
    if input.agent_count != godel.agents.len() as u32 || input.agent_count < 10 {
        return Err(anyhow!(
            "Godel/constructability boundary must consume 10+ Godel agents"
        ));
    }
    if input.provider_binding_count != godel.provider_registry.len() as u32 {
        return Err(anyhow!(
            "Godel/constructability boundary provider count must match Godel runtime"
        ));
    }
    require_exact(
        &input.launch_plan_status,
        "csm_supervised_provider_request_admission_ready",
        "godel_constructability_boundary.godel.launch_plan_status",
    )?;
    if input.provider_request_count != godel.launch_plan.provider_request_count
        || input.provider_request_count != godel.agents.len() as u32
        || input.provider_request_count < 10
    {
        return Err(anyhow!(
            "Godel/constructability boundary must retain launch-plan provider requests for every 10+ Godel agent"
        ));
    }
    require_exact(
        &input.hosted_invocation_status,
        "provider_target_resolved_not_invoked",
        "godel_constructability_boundary.godel.hosted_invocation_status",
    )?;
    require_fields(
        &input.retained_non_claims,
        &[
            "not_unbounded_recursive_self_improvement",
            "not_live_hosted_provider_invocation",
            "not_source_code_mutation_without_review",
            "not_v092_adaptive_learning_dag_completion",
        ],
        "godel_constructability_boundary.godel.retained_non_claims",
    )?;
    require_exact_set(
        &input.retained_non_claims,
        &godel.non_claims,
        "godel_constructability_boundary.godel.retained_non_claims",
    )
}

fn validate_constructability_input(
    input: &RuntimeV2ConstructabilityBoundaryInput,
    constructability: &RuntimeV2ConstructabilityAnchorValidatorPacket,
) -> Result<()> {
    require_exact(
        &input.schema_version,
        RUNTIME_V2_CONSTRUCTABILITY_ANCHOR_VALIDATOR_SCHEMA,
        "godel_constructability_boundary.constructability.schema_version",
    )?;
    require_exact(
        &input.validator_id,
        RUNTIME_V2_CONSTRUCTABILITY_ANCHOR_VALIDATOR_REF,
        "godel_constructability_boundary.constructability.validator_id",
    )?;
    if input.validator_id != constructability.validator_id {
        return Err(anyhow!(
            "Godel/constructability boundary validator id must match constructability packet"
        ));
    }
    if !input.promotion_requires_anchor
        || !input.promotion_requires_validator_pass
        || !input.promotion_requires_operator_review
    {
        return Err(anyhow!(
            "Godel/constructability boundary requires anchor, validator pass, and operator review"
        ));
    }
    if input.failure_mode_count != constructability.failure_modes.len() as u32
        || input.failure_mode_count == 0
    {
        return Err(anyhow!(
            "Godel/constructability boundary must consume constructability failure modes"
        ));
    }
    require_fields(
        &input.retained_non_claims,
        &[
            "does not adjudicate universal truth",
            "does not publish shared reality without admissible anchors",
            "does not bypass Freedom Gate, CAV, or operator review",
        ],
        "godel_constructability_boundary.constructability.retained_non_claims",
    )?;
    require_exact_set(
        &input.retained_non_claims,
        &constructability.non_claims,
        "godel_constructability_boundary.constructability.retained_non_claims",
    )
}

fn canonicalize_runtime_v2_godel_constructability_boundary(
    packet: &mut RuntimeV2GodelConstructabilityBoundaryPacket,
) {
    packet.godel_runtime.retained_non_claims.sort();
    packet.constructability_validator.retained_non_claims.sort();
    packet.v092_allowed_claims.sort();
    packet.v092_prohibited_claims.sort();
    packet.promotion_requirements.sort();
    packet.validation_commands.sort();
    packet.non_claims.sort();
}

fn require_fields(values: &[String], required_fragments: &[&str], field_name: &str) -> Result<()> {
    if values.is_empty() {
        return Err(anyhow!("{field_name} must not be empty"));
    }
    let joined = values.join("\n");
    for required in required_fragments {
        if !joined.contains(required) {
            return Err(anyhow!("{field_name} must mention {required}"));
        }
    }
    let observed = values.iter().map(String::as_str).collect::<BTreeSet<_>>();
    if observed.len() != values.len() {
        return Err(anyhow!("{field_name} must not contain duplicate values"));
    }
    Ok(())
}

fn require_exact_set(actual: &[String], expected: &[String], field_name: &str) -> Result<()> {
    let actual_set = actual.iter().map(String::as_str).collect::<BTreeSet<_>>();
    let expected_set = expected.iter().map(String::as_str).collect::<BTreeSet<_>>();
    if actual_set != expected_set {
        return Err(anyhow!("{field_name} must exactly match consumed packet"));
    }
    Ok(())
}

fn reject_prohibited_claim_fragments(
    allowed_claims: &[String],
    prohibited_claims: &[String],
    field_name: &str,
) -> Result<()> {
    for allowed in allowed_claims {
        let normalized_allowed = normalize_claim_fragment(allowed);
        for prohibited in prohibited_claims {
            let normalized_prohibited = normalize_claim_fragment(prohibited);
            if normalized_allowed.contains(&normalized_prohibited) {
                return Err(anyhow!(
                    "{field_name} must not assert prohibited claim {prohibited}"
                ));
            }
        }
    }
    Ok(())
}

fn normalize_claim_fragment(value: &str) -> String {
    value
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() {
                ch.to_ascii_lowercase()
            } else {
                ' '
            }
        })
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
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
