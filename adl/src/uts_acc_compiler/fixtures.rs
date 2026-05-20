use super::{ToolProposalV1, UtsAccCompilerInputV1, UtsAccPolicyContextV1};
use crate::acc::AccGrantStatusV1;
use crate::tool_registry::{
    wp08_tool_registry_v1_fixture, RegisteredToolV1, ToolAdapterCapabilityV1, ToolRegistryV1,
};
use crate::uts::{
    UniversalToolSchemaV1_1, UtsAuthenticationModeV1, UtsAuthenticationRequirementV1,
    UtsCategoryV1, UtsCompatibleVersionV1, UtsDataSensitivityV1, UtsDeterminismV1, UtsErrorModelV1,
    UtsExecutionEnvironmentKindV1, UtsExecutionEnvironmentV1, UtsExfiltrationRiskV1,
    UtsIdempotenceV1, UtsJsonSchemaFragmentV1, UtsObservabilityV1, UtsPlanningMetadataV1,
    UtsReplaySafetyV1, UtsResourceRequirementV1, UtsSideEffectClassV1, UtsSideEffectTagV1,
    UTS_SCHEMA_VERSION_V1_1,
};
use serde_json::json;
use std::collections::BTreeMap;

fn object_schema(
    properties: serde_json::Value,
    required: serde_json::Value,
    additional_properties: bool,
) -> UtsJsonSchemaFragmentV1 {
    UtsJsonSchemaFragmentV1 {
        schema_type: "object".to_string(),
        keywords: BTreeMap::from([
            ("properties".to_string(), properties),
            ("required".to_string(), required),
            (
                "additionalProperties".to_string(),
                json!(additional_properties),
            ),
        ]),
    }
}

fn string_array_schema() -> serde_json::Value {
    json!({
        "type": "array",
        "items": {"type": "string"},
        "minItems": 1
    })
}

fn schema_for_tool(
    name: &str,
    side_effect: UtsSideEffectClassV1,
    resource_scope: &str,
    data_sensitivity: UtsDataSensitivityV1,
    exfiltration_risk: UtsExfiltrationRiskV1,
) -> UniversalToolSchemaV1_1 {
    let (categories, side_effects, observability, planning) = match side_effect {
        UtsSideEffectClassV1::Read => (
            vec![UtsCategoryV1::ReadOnly],
            vec![UtsSideEffectTagV1::None],
            UtsObservabilityV1::Basic,
            UtsPlanningMetadataV1 {
                review_recommended: Some(false),
                ..UtsPlanningMetadataV1::default()
            },
        ),
        UtsSideEffectClassV1::LocalWrite => (
            vec![UtsCategoryV1::StateMutating],
            vec![UtsSideEffectTagV1::LocalState],
            UtsObservabilityV1::Full,
            UtsPlanningMetadataV1 {
                review_recommended: Some(true),
                ..UtsPlanningMetadataV1::default()
            },
        ),
        UtsSideEffectClassV1::ExternalRead => (
            vec![UtsCategoryV1::ExternalNetwork],
            vec![UtsSideEffectTagV1::ExternalState],
            UtsObservabilityV1::Full,
            UtsPlanningMetadataV1 {
                expensive: Some(true),
                ..UtsPlanningMetadataV1::default()
            },
        ),
        UtsSideEffectClassV1::ExternalWrite => (
            vec![
                UtsCategoryV1::ExternalNetwork,
                UtsCategoryV1::StateMutating,
                UtsCategoryV1::HumanVisible,
            ],
            vec![
                UtsSideEffectTagV1::ExternalState,
                UtsSideEffectTagV1::HumanVisible,
            ],
            UtsObservabilityV1::Governance,
            UtsPlanningMetadataV1 {
                high_risk: Some(true),
                review_recommended: Some(true),
                ..UtsPlanningMetadataV1::default()
            },
        ),
        UtsSideEffectClassV1::Process => (
            vec![UtsCategoryV1::StateMutating],
            vec![UtsSideEffectTagV1::Irreversible],
            UtsObservabilityV1::Governance,
            UtsPlanningMetadataV1 {
                high_risk: Some(true),
                slow: Some(true),
                review_recommended: Some(true),
                ..UtsPlanningMetadataV1::default()
            },
        ),
        UtsSideEffectClassV1::Network => (
            vec![UtsCategoryV1::ExternalNetwork],
            vec![UtsSideEffectTagV1::ExternalState],
            UtsObservabilityV1::Full,
            UtsPlanningMetadataV1 {
                slow: Some(true),
                ..UtsPlanningMetadataV1::default()
            },
        ),
        UtsSideEffectClassV1::Destructive => (
            vec![
                UtsCategoryV1::StateMutating,
                UtsCategoryV1::GovernanceSensitive,
            ],
            vec![
                UtsSideEffectTagV1::Irreversible,
                UtsSideEffectTagV1::GovernanceRelevant,
            ],
            UtsObservabilityV1::Governance,
            UtsPlanningMetadataV1 {
                high_risk: Some(true),
                irreversible: Some(true),
                review_recommended: Some(true),
                ..UtsPlanningMetadataV1::default()
            },
        ),
        UtsSideEffectClassV1::Exfiltration => (
            vec![
                UtsCategoryV1::ExternalNetwork,
                UtsCategoryV1::GovernanceSensitive,
                UtsCategoryV1::ObservabilitySensitive,
            ],
            vec![
                UtsSideEffectTagV1::ExternalState,
                UtsSideEffectTagV1::GovernanceRelevant,
            ],
            UtsObservabilityV1::Governance,
            UtsPlanningMetadataV1 {
                high_risk: Some(true),
                irreversible: Some(true),
                expensive: Some(true),
                review_recommended: Some(true),
                ..UtsPlanningMetadataV1::default()
            },
        ),
    };

    UniversalToolSchemaV1_1 {
        schema_version: UTS_SCHEMA_VERSION_V1_1.to_string(),
        compatible_versions: vec![UtsCompatibleVersionV1::V1, UtsCompatibleVersionV1::V1_1],
        name: name.to_string(),
        version: "1.0.0".to_string(),
        description: format!("Fixture schema for compiler mapping case {name}."),
        categories: Some(categories),
        input_schema: object_schema(
            json!({"fixture_id": {"type": "string"}}),
            json!(["fixture_id"]),
            false,
        ),
        output_schema: object_schema(
            json!({"content": {"type": "string"}}),
            json!(["content"]),
            false,
        ),
        side_effect_class: side_effect,
        side_effects: Some(side_effects),
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
        observability: Some(observability),
        planning: Some(planning),
        extensions: BTreeMap::new(),
    }
}

fn canonical_schema_for_tool(name: &str) -> Option<UniversalToolSchemaV1_1> {
    let (side_effect, resource_scope, sensitivity, exfiltration, input_schema) = match name {
        "get_time" => (
            UtsSideEffectClassV1::Read,
            "local-readonly",
            UtsDataSensitivityV1::Public,
            UtsExfiltrationRiskV1::None,
            object_schema(json!({}), json!([]), false),
        ),
        "get_weather" => (
            UtsSideEffectClassV1::ExternalRead,
            "external-read",
            UtsDataSensitivityV1::Public,
            UtsExfiltrationRiskV1::Low,
            object_schema(
                json!({
                    "location": {"type": "string"},
                    "unit": {"type": "string", "enum": ["celsius", "fahrenheit"]}
                }),
                json!(["location"]),
                false,
            ),
        ),
        "convert_currency" => (
            UtsSideEffectClassV1::ExternalRead,
            "external-read",
            UtsDataSensitivityV1::Public,
            UtsExfiltrationRiskV1::Low,
            object_schema(
                json!({
                    "amount": {"type": "number"},
                    "from": {"type": "string"},
                    "to": {"type": "string"}
                }),
                json!(["amount", "from", "to"]),
                false,
            ),
        ),
        "search_contacts" => (
            UtsSideEffectClassV1::Read,
            "local-readonly",
            UtsDataSensitivityV1::Internal,
            UtsExfiltrationRiskV1::None,
            object_schema(
                json!({
                    "query": {"type": "string"},
                    "limit": {"type": "integer", "minimum": 1}
                }),
                json!(["query"]),
                false,
            ),
        ),
        "read_document" => (
            UtsSideEffectClassV1::Read,
            "local-readonly",
            UtsDataSensitivityV1::Internal,
            UtsExfiltrationRiskV1::None,
            object_schema(
                json!({
                    "document_id": {"type": "string"},
                    "section": {"type": "string"}
                }),
                json!(["document_id"]),
                false,
            ),
        ),
        "append_log" => (
            UtsSideEffectClassV1::LocalWrite,
            "local-write",
            UtsDataSensitivityV1::Internal,
            UtsExfiltrationRiskV1::None,
            object_schema(
                json!({"log_line": {"type": "string"}}),
                json!(["log_line"]),
                false,
            ),
        ),
        "send_email" => (
            UtsSideEffectClassV1::ExternalWrite,
            "external-write",
            UtsDataSensitivityV1::Confidential,
            UtsExfiltrationRiskV1::Medium,
            object_schema(
                json!({
                    "to": {"type": "string"},
                    "subject": {"type": "string"},
                    "body": {"type": "string"}
                }),
                json!(["to", "subject", "body"]),
                false,
            ),
        ),
        "query_database" => (
            UtsSideEffectClassV1::Read,
            "local-readonly",
            UtsDataSensitivityV1::Internal,
            UtsExfiltrationRiskV1::None,
            object_schema(
                json!({
                    "table": {"type": "string"},
                    "filters": {"type": "object"}
                }),
                json!(["table", "filters"]),
                false,
            ),
        ),
        "update_inventory" => (
            UtsSideEffectClassV1::LocalWrite,
            "local-write",
            UtsDataSensitivityV1::Internal,
            UtsExfiltrationRiskV1::None,
            object_schema(
                json!({
                    "sku": {"type": "string"},
                    "delta": {"type": "integer"},
                    "reason": {"type": "string"}
                }),
                json!(["sku", "delta"]),
                false,
            ),
        ),
        "batch_weather_lookup" => (
            UtsSideEffectClassV1::ExternalRead,
            "external-read",
            UtsDataSensitivityV1::Public,
            UtsExfiltrationRiskV1::Low,
            object_schema(
                json!({"locations": string_array_schema()}),
                json!(["locations"]),
                false,
            ),
        ),
        _ => return None,
    };

    let mut schema = schema_for_tool(name, side_effect, resource_scope, sensitivity, exfiltration);
    schema.input_schema = input_schema;
    Some(schema)
}

pub fn wp09_compiler_registry_fixture() -> ToolRegistryV1 {
    let mut registry = wp08_tool_registry_v1_fixture();
    for (name, side_effect, scope, sensitivity, exfiltration) in [
        (
            "fixture.external_read",
            UtsSideEffectClassV1::ExternalRead,
            "external-read",
            UtsDataSensitivityV1::Confidential,
            UtsExfiltrationRiskV1::Low,
        ),
        (
            "fixture.external_write",
            UtsSideEffectClassV1::ExternalWrite,
            "external-write",
            UtsDataSensitivityV1::Confidential,
            UtsExfiltrationRiskV1::Medium,
        ),
        (
            "fixture.local_write",
            UtsSideEffectClassV1::LocalWrite,
            "local-write",
            UtsDataSensitivityV1::Internal,
            UtsExfiltrationRiskV1::None,
        ),
        (
            "fixture.process",
            UtsSideEffectClassV1::Process,
            "process-fixture",
            UtsDataSensitivityV1::Confidential,
            UtsExfiltrationRiskV1::Medium,
        ),
        (
            "fixture.network",
            UtsSideEffectClassV1::Network,
            "network-fixture",
            UtsDataSensitivityV1::Confidential,
            UtsExfiltrationRiskV1::Medium,
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
    for name in [
        "get_time",
        "get_weather",
        "convert_currency",
        "search_contacts",
        "read_document",
        "append_log",
        "send_email",
        "query_database",
        "update_inventory",
        "batch_weather_lookup",
    ] {
        let schema = canonical_schema_for_tool(name).expect("canonical tool schema should exist");
        let adapter_id = format!("adapter.{name}.dry_run");
        registry.tools.push(RegisteredToolV1 {
            registry_tool_id: format!("registry.{name}"),
            tool_name: name.to_string(),
            tool_version: "1.0.0".to_string(),
            active: true,
            uts: schema.clone(),
            approved_adapter_ids: vec![adapter_id.clone()],
        });
        registry.adapters.push(ToolAdapterCapabilityV1 {
            adapter_id,
            tool_name: name.to_string(),
            tool_version: "1.0.0".to_string(),
            capability_id: format!("capability.{}", name.replace('_', "-")),
            side_effect_class: schema.side_effect_class,
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
        allowed_side_effects: vec![
            UtsSideEffectClassV1::Read,
            UtsSideEffectClassV1::LocalWrite,
            UtsSideEffectClassV1::ExternalRead,
            UtsSideEffectClassV1::ExternalWrite,
        ],
        allowed_resource_scopes: vec![
            "local-readonly".to_string(),
            "local-write".to_string(),
            "external-read".to_string(),
            "external-write".to_string(),
        ],
        allow_sensitive_data: true,
        visibility_constructible: true,
        replay_allowed: true,
        execution_approved: true,
    }
}

pub fn wp09_proposal_fixture(tool_name: &str) -> ToolProposalV1 {
    let arguments = match tool_name {
        "get_time" => BTreeMap::new(),
        "get_weather" => BTreeMap::from([("location".to_string(), json!("Seattle"))]),
        "convert_currency" => BTreeMap::from([
            ("amount".to_string(), json!(20)),
            ("from".to_string(), json!("USD")),
            ("to".to_string(), json!("JPY")),
        ]),
        "search_contacts" => BTreeMap::from([
            ("query".to_string(), json!("Sam")),
            ("limit".to_string(), json!(5)),
        ]),
        "read_document" => BTreeMap::from([
            ("document_id".to_string(), json!("overview.md")),
            ("section".to_string(), json!("summary")),
        ]),
        "append_log" => BTreeMap::from([(
            "log_line".to_string(),
            json!("review requested for audit note"),
        )]),
        "send_email" => BTreeMap::from([
            ("to".to_string(), json!("sam@example.com")),
            ("subject".to_string(), json!("Project update")),
            ("body".to_string(), json!("This is a review draft only.")),
        ]),
        "query_database" => BTreeMap::from([
            ("table".to_string(), json!("revenue")),
            ("filters".to_string(), json!({"product": "A17"})),
        ]),
        "update_inventory" => BTreeMap::from([
            ("sku".to_string(), json!("A17")),
            ("delta".to_string(), json!(-1)),
            ("reason".to_string(), json!("review adjustment")),
        ]),
        "batch_weather_lookup" => BTreeMap::from([(
            "locations".to_string(),
            json!(["Tokyo", "London", "New York"]),
        )]),
        _ => BTreeMap::from([("fixture_id".to_string(), json!("fixture-a"))]),
    };
    ToolProposalV1 {
        proposal_id: format!("proposal.{}", tool_name.replace('_', "-")),
        tool_name: tool_name.to_string(),
        tool_version: "1.0.0".to_string(),
        adapter_id: format!("adapter.{tool_name}.dry_run"),
        arguments,
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
