use super::{ToolProposalV1, UtsAccCompilerInputV1, UtsAccPolicyContextV1};
use crate::acc::AccGrantStatusV1;
use crate::tool_registry::{
    wp08_tool_registry_v1_fixture, RegisteredToolV1, ToolAdapterCapabilityV1, ToolRegistryV1,
};
use crate::uts::{
    UniversalToolSchemaV1, UtsAuthenticationModeV1, UtsAuthenticationRequirementV1,
    UtsDataSensitivityV1, UtsDeterminismV1, UtsErrorModelV1, UtsExecutionEnvironmentKindV1,
    UtsExecutionEnvironmentV1, UtsExfiltrationRiskV1, UtsIdempotenceV1, UtsJsonSchemaFragmentV1,
    UtsReplaySafetyV1, UtsResourceRequirementV1, UtsSideEffectClassV1, UTS_SCHEMA_VERSION_V1,
};
use serde_json::json;
use std::collections::BTreeMap;

fn schema_for_tool(
    name: &str,
    side_effect: UtsSideEffectClassV1,
    resource_scope: &str,
    data_sensitivity: UtsDataSensitivityV1,
    exfiltration_risk: UtsExfiltrationRiskV1,
) -> UniversalToolSchemaV1 {
    UniversalToolSchemaV1 {
        schema_version: UTS_SCHEMA_VERSION_V1.to_string(),
        name: name.to_string(),
        version: "1.0.0".to_string(),
        description: format!("Fixture schema for compiler mapping case {name}."),
        input_schema: UtsJsonSchemaFragmentV1 {
            schema_type: "object".to_string(),
            keywords: BTreeMap::from([
                (
                    "properties".to_string(),
                    json!({"fixture_id": {"type": "string"}}),
                ),
                ("required".to_string(), json!(["fixture_id"])),
                ("additionalProperties".to_string(), json!(false)),
            ]),
        },
        output_schema: UtsJsonSchemaFragmentV1 {
            schema_type: "object".to_string(),
            keywords: BTreeMap::from([
                (
                    "properties".to_string(),
                    json!({"content": {"type": "string"}}),
                ),
                ("required".to_string(), json!(["content"])),
                ("additionalProperties".to_string(), json!(false)),
            ]),
        },
        side_effect_class: side_effect,
        determinism: UtsDeterminismV1::Deterministic,
        replay_safety: UtsReplaySafetyV1::ReplaySafe,
        idempotence: UtsIdempotenceV1::Idempotent,
        resources: vec![UtsResourceRequirementV1 {
            resource_type: "fixture".to_string(),
            scope: resource_scope.to_string(),
        }],
        authentication: UtsAuthenticationRequirementV1 {
            mode: UtsAuthenticationModeV1::None,
            required: false,
        },
        data_sensitivity,
        exfiltration_risk,
        execution_environment: UtsExecutionEnvironmentV1 {
            kind: UtsExecutionEnvironmentKindV1::DryRun,
            isolation: "deterministic compiler dry-run fixture only".to_string(),
        },
        errors: vec![UtsErrorModelV1 {
            code: "fixture_not_available".to_string(),
            message: "The requested compiler fixture is not available.".to_string(),
            retryable: false,
        }],
        extensions: BTreeMap::new(),
    }
}

pub fn wp09_compiler_registry_fixture() -> ToolRegistryV1 {
    let mut registry = wp08_tool_registry_v1_fixture();
    for (name, side_effect, scope, sensitivity, exfiltration) in [
        (
            "fixture.local_write",
            UtsSideEffectClassV1::LocalWrite,
            "local-write",
            UtsDataSensitivityV1::Internal,
            UtsExfiltrationRiskV1::None,
        ),
        (
            "fixture.destructive",
            UtsSideEffectClassV1::Destructive,
            "destructive-fixture",
            UtsDataSensitivityV1::Internal,
            UtsExfiltrationRiskV1::Medium,
        ),
        (
            "fixture.exfiltrate",
            UtsSideEffectClassV1::Exfiltration,
            "protected-prompt",
            UtsDataSensitivityV1::Secret,
            UtsExfiltrationRiskV1::High,
        ),
    ] {
        let adapter_id = format!("adapter.{name}.dry_run");
        registry.tools.push(RegisteredToolV1 {
            registry_tool_id: format!("registry.{name}"),
            tool_name: name.to_string(),
            tool_version: "1.0.0".to_string(),
            active: true,
            uts: schema_for_tool(name, side_effect, scope, sensitivity, exfiltration),
            approved_adapter_ids: vec![adapter_id.clone()],
        });
        registry.adapters.push(ToolAdapterCapabilityV1 {
            adapter_id,
            tool_name: name.to_string(),
            tool_version: "1.0.0".to_string(),
            capability_id: format!("capability.{}", name.replace('_', "-")),
            side_effect_class: side_effect,
            execution_environment: UtsExecutionEnvironmentKindV1::DryRun,
            supports_dry_run: true,
            approved_for_binding: true,
        });
    }
    registry
}

pub fn wp09_policy_context_fixture() -> UtsAccPolicyContextV1 {
    UtsAccPolicyContextV1 {
        actor_id: "actor.operator.alice".to_string(),
        role: "operator".to_string(),
        standing: "active".to_string(),
        authenticated: true,
        grant_id: "grant.compiler.fixture".to_string(),
        grantor_actor_id: "actor.operator.alice".to_string(),
        grant_status: AccGrantStatusV1::Active,
        delegation: None,
        allowed_side_effects: vec![UtsSideEffectClassV1::Read, UtsSideEffectClassV1::LocalWrite],
        allowed_resource_scopes: vec!["local-readonly".to_string(), "local-write".to_string()],
        allow_sensitive_data: true,
        visibility_constructible: true,
        replay_allowed: true,
        execution_approved: true,
    }
}

pub fn wp09_proposal_fixture(tool_name: &str) -> ToolProposalV1 {
    ToolProposalV1 {
        proposal_id: format!("proposal.{}", tool_name.replace('_', "-")),
        tool_name: tool_name.to_string(),
        tool_version: "1.0.0".to_string(),
        adapter_id: format!("adapter.{tool_name}.dry_run"),
        arguments: BTreeMap::from([("fixture_id".to_string(), json!("fixture-a"))]),
        dry_run_requested: true,
        ambiguous: false,
    }
}

pub fn wp09_compiler_input_fixture(tool_name: &str) -> UtsAccCompilerInputV1 {
    let mut proposal = wp09_proposal_fixture(tool_name);
    if tool_name == "fixture.safe_read" {
        proposal.adapter_id = "adapter.fixture.safe_read.dry_run".to_string();
    }
    UtsAccCompilerInputV1 {
        proposal,
        registry: wp09_compiler_registry_fixture(),
        policy_context: wp09_policy_context_fixture(),
    }
}
