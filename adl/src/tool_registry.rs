use crate::uts::{
    validate_uts_v1, UniversalToolSchemaV1, UtsAuthenticationModeV1,
    UtsAuthenticationRequirementV1, UtsDataSensitivityV1, UtsDeterminismV1, UtsErrorModelV1,
    UtsExecutionEnvironmentKindV1, UtsExecutionEnvironmentV1, UtsExfiltrationRiskV1,
    UtsIdempotenceV1, UtsJsonSchemaFragmentV1, UtsReplaySafetyV1, UtsResourceRequirementV1,
    UtsSideEffectClassV1, UTS_SCHEMA_VERSION_V1,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::{BTreeMap, BTreeSet};

pub const TOOL_REGISTRY_SCHEMA_VERSION_V1: &str = "tool_registry.v1";

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ToolBindingSourceV1 {
    ModelOutput,
    RegistryCompiler,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ToolBindingDecisionV1 {
    Bound,
    Rejected,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ToolRegistryRejectionCodeV1 {
    InvalidRegistry,
    InvalidUts,
    ModelDirectExecutionDenied,
    UnknownTool,
    UnregisteredTool,
    IncompatibleVersion,
    MismatchedAdapterCapabilities,
    UnsafeDryRunPosture,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct RegisteredToolV1 {
    pub registry_tool_id: String,
    pub tool_name: String,
    pub tool_version: String,
    pub active: bool,
    pub uts: UniversalToolSchemaV1,
    pub approved_adapter_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ToolAdapterCapabilityV1 {
    pub adapter_id: String,
    pub tool_name: String,
    pub tool_version: String,
    pub capability_id: String,
    pub side_effect_class: UtsSideEffectClassV1,
    pub execution_environment: UtsExecutionEnvironmentKindV1,
    pub supports_dry_run: bool,
    pub approved_for_binding: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ToolRegistryV1 {
    pub schema_version: String,
    pub registry_id: String,
    pub tools: Vec<RegisteredToolV1>,
    pub adapters: Vec<ToolAdapterCapabilityV1>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ToolBindingRequestV1 {
    pub source: ToolBindingSourceV1,
    pub tool_name: String,
    pub tool_version: String,
    pub adapter_id: String,
    pub dry_run_requested: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ToolBindingV1 {
    pub registry_tool_id: String,
    pub adapter_id: String,
    pub capability_id: String,
    pub binding_key: String,
    pub dry_run: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ToolBindingOutcomeV1 {
    pub decision: ToolBindingDecisionV1,
    #[serde(default)]
    pub binding: Option<ToolBindingV1>,
    #[serde(default)]
    pub rejection_code: Option<ToolRegistryRejectionCodeV1>,
    pub evidence: Vec<String>,
}

fn token_like(value: &str) -> bool {
    !value.trim().is_empty()
        && value.chars().all(|ch| {
            ch.is_ascii_lowercase() || ch.is_ascii_digit() || matches!(ch, '-' | '_' | '.')
        })
}

fn reject(code: ToolRegistryRejectionCodeV1, evidence: Vec<String>) -> ToolBindingOutcomeV1 {
    ToolBindingOutcomeV1 {
        decision: ToolBindingDecisionV1::Rejected,
        binding: None,
        rejection_code: Some(code),
        evidence,
    }
}

pub fn registry_state_fingerprint_v1(registry: &ToolRegistryV1) -> String {
    let mut normalized = registry.clone();
    normalized.tools.sort_by(|left, right| {
        (&left.registry_tool_id, &left.tool_name, &left.tool_version).cmp(&(
            &right.registry_tool_id,
            &right.tool_name,
            &right.tool_version,
        ))
    });
    for tool in &mut normalized.tools {
        tool.approved_adapter_ids.sort();
    }
    normalized.adapters.sort_by(|left, right| {
        (
            &left.adapter_id,
            &left.tool_name,
            &left.tool_version,
            &left.capability_id,
        )
            .cmp(&(
                &right.adapter_id,
                &right.tool_name,
                &right.tool_version,
                &right.capability_id,
            ))
    });

    serde_json::to_string(&normalized).expect("tool registry fingerprint should serialize")
}

pub fn validate_tool_registry_v1(
    registry: &ToolRegistryV1,
) -> Result<(), Box<ToolBindingOutcomeV1>> {
    if registry.schema_version != TOOL_REGISTRY_SCHEMA_VERSION_V1
        || !token_like(&registry.registry_id)
        || registry.tools.is_empty()
        || registry.adapters.is_empty()
    {
        return Err(Box::new(reject(
            ToolRegistryRejectionCodeV1::InvalidRegistry,
            vec!["registry must declare version, id, tools, and adapters".to_string()],
        )));
    }

    let mut tool_ids = BTreeSet::new();
    let mut adapter_ids = BTreeSet::new();

    for tool in &registry.tools {
        if !token_like(&tool.registry_tool_id)
            || !token_like(&tool.tool_name)
            || tool.tool_version.trim().is_empty()
            || !tool_ids.insert(tool.registry_tool_id.as_str())
            || tool.approved_adapter_ids.is_empty()
        {
            return Err(Box::new(reject(
                ToolRegistryRejectionCodeV1::InvalidRegistry,
                vec![format!(
                    "tool '{}' must have explicit unique id/name/version and approved adapters",
                    tool.registry_tool_id
                )],
            )));
        }
        if let Err(report) = validate_uts_v1(&tool.uts) {
            return Err(Box::new(reject(
                ToolRegistryRejectionCodeV1::InvalidUts,
                report
                    .errors
                    .iter()
                    .map(|error| format!("{}:{}", error.field, error.code))
                    .collect(),
            )));
        }
        if tool.uts.name != tool.tool_name || tool.uts.version != tool.tool_version {
            return Err(Box::new(reject(
                ToolRegistryRejectionCodeV1::InvalidRegistry,
                vec![format!(
                    "tool '{}' must match its embedded UTS name and version",
                    tool.registry_tool_id
                )],
            )));
        }
    }

    for adapter in &registry.adapters {
        if !token_like(&adapter.adapter_id)
            || !adapter_ids.insert(adapter.adapter_id.as_str())
            || !token_like(&adapter.tool_name)
            || adapter.tool_version.trim().is_empty()
            || !token_like(&adapter.capability_id)
        {
            return Err(Box::new(reject(
                ToolRegistryRejectionCodeV1::InvalidRegistry,
                vec![format!(
                    "adapter '{}' must have explicit unique id/name/version/capability",
                    adapter.adapter_id
                )],
            )));
        }
    }

    for tool in &registry.tools {
        for approved_adapter_id in &tool.approved_adapter_ids {
            if !adapter_ids.contains(approved_adapter_id.as_str()) {
                return Err(Box::new(reject(
                    ToolRegistryRejectionCodeV1::InvalidRegistry,
                    vec![format!(
                        "tool '{}' references missing approved adapter '{}'",
                        tool.registry_tool_id, approved_adapter_id
                    )],
                )));
            }
        }
    }

    Ok(())
}

pub fn bind_tool_registry_v1(
    registry: &ToolRegistryV1,
    request: &ToolBindingRequestV1,
) -> ToolBindingOutcomeV1 {
    if let Err(outcome) = validate_tool_registry_v1(registry) {
        return *outcome;
    }

    if matches!(request.source, ToolBindingSourceV1::ModelOutput) {
        return reject(
            ToolRegistryRejectionCodeV1::ModelDirectExecutionDenied,
            vec!["model output is a proposal and cannot bind directly to execution".to_string()],
        );
    }

    let same_name: Vec<&RegisteredToolV1> = registry
        .tools
        .iter()
        .filter(|tool| tool.tool_name == request.tool_name)
        .collect();
    if same_name.is_empty() {
        return reject(
            ToolRegistryRejectionCodeV1::UnknownTool,
            vec![format!(
                "tool '{}' is unknown to the registry",
                request.tool_name
            )],
        );
    }

    let Some(tool) = same_name
        .into_iter()
        .find(|tool| tool.tool_version == request.tool_version)
    else {
        return reject(
            ToolRegistryRejectionCodeV1::IncompatibleVersion,
            vec![format!(
                "tool '{}' version '{}' is not registered",
                request.tool_name, request.tool_version
            )],
        );
    };

    if !tool.active {
        return reject(
            ToolRegistryRejectionCodeV1::UnregisteredTool,
            vec![format!(
                "tool '{}' version '{}' is present but inactive",
                request.tool_name, request.tool_version
            )],
        );
    }

    let Some(adapter) = registry
        .adapters
        .iter()
        .find(|adapter| adapter.adapter_id == request.adapter_id)
    else {
        return reject(
            ToolRegistryRejectionCodeV1::MismatchedAdapterCapabilities,
            vec![format!(
                "adapter '{}' is not registered",
                request.adapter_id
            )],
        );
    };

    let adapter_matches_tool = tool.approved_adapter_ids.contains(&adapter.adapter_id)
        && adapter.approved_for_binding
        && adapter.tool_name == tool.tool_name
        && adapter.tool_version == tool.tool_version
        && adapter.side_effect_class == tool.uts.side_effect_class
        && adapter.execution_environment == tool.uts.execution_environment.kind;
    if !adapter_matches_tool {
        return reject(
            ToolRegistryRejectionCodeV1::MismatchedAdapterCapabilities,
            vec![format!(
                "adapter '{}' does not match registered tool capability",
                adapter.adapter_id
            )],
        );
    }

    if !request.dry_run_requested || !adapter.supports_dry_run {
        return reject(
            ToolRegistryRejectionCodeV1::UnsafeDryRunPosture,
            vec!["WP-08 bindings require explicit supported dry-run posture".to_string()],
        );
    }

    let binding_key = format!(
        "{}:{}:{}:{}",
        registry.registry_id, tool.registry_tool_id, adapter.adapter_id, tool.tool_version
    );

    ToolBindingOutcomeV1 {
        decision: ToolBindingDecisionV1::Bound,
        binding: Some(ToolBindingV1 {
            registry_tool_id: tool.registry_tool_id.clone(),
            adapter_id: adapter.adapter_id.clone(),
            capability_id: adapter.capability_id.clone(),
            binding_key,
            dry_run: true,
        }),
        rejection_code: None,
        evidence: vec![
            "registry state explicit".to_string(),
            registry_state_fingerprint_v1(registry),
            "adapter binding approved".to_string(),
        ],
    }
}

fn safe_read_uts() -> UniversalToolSchemaV1 {
    UniversalToolSchemaV1 {
        schema_version: UTS_SCHEMA_VERSION_V1.to_string(),
        name: "fixture.safe_read".to_string(),
        version: "1.0.0".to_string(),
        description: "Read a bounded local fixture for registry binding tests.".to_string(),
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
        side_effect_class: UtsSideEffectClassV1::Read,
        determinism: UtsDeterminismV1::Deterministic,
        replay_safety: UtsReplaySafetyV1::ReplaySafe,
        idempotence: UtsIdempotenceV1::Idempotent,
        resources: vec![UtsResourceRequirementV1 {
            resource_type: "fixture".to_string(),
            scope: "local-readonly".to_string(),
        }],
        authentication: UtsAuthenticationRequirementV1 {
            mode: UtsAuthenticationModeV1::None,
            required: false,
        },
        data_sensitivity: UtsDataSensitivityV1::Internal,
        exfiltration_risk: UtsExfiltrationRiskV1::None,
        execution_environment: UtsExecutionEnvironmentV1 {
            kind: UtsExecutionEnvironmentKindV1::DryRun,
            isolation: "deterministic fixture dry run only".to_string(),
        },
        errors: vec![UtsErrorModelV1 {
            code: "fixture_not_found".to_string(),
            message: "The requested fixture is not available.".to_string(),
            retryable: false,
        }],
        extensions: BTreeMap::new(),
    }
}

pub fn wp08_tool_registry_v1_fixture() -> ToolRegistryV1 {
    ToolRegistryV1 {
        schema_version: TOOL_REGISTRY_SCHEMA_VERSION_V1.to_string(),
        registry_id: "registry.wp08.fixture".to_string(),
        tools: vec![
            RegisteredToolV1 {
                registry_tool_id: "registry.fixture.safe_read".to_string(),
                tool_name: "fixture.safe_read".to_string(),
                tool_version: "1.0.0".to_string(),
                active: true,
                uts: safe_read_uts(),
                approved_adapter_ids: vec!["adapter.fixture.safe_read.dry_run".to_string()],
            },
            RegisteredToolV1 {
                registry_tool_id: "registry.fixture.disabled_write".to_string(),
                tool_name: "fixture.disabled_write".to_string(),
                tool_version: "1.0.0".to_string(),
                active: false,
                uts: {
                    let mut uts = safe_read_uts();
                    uts.name = "fixture.disabled_write".to_string();
                    uts.side_effect_class = UtsSideEffectClassV1::LocalWrite;
                    uts.resources = vec![UtsResourceRequirementV1 {
                        resource_type: "fixture".to_string(),
                        scope: "local-disabled-write".to_string(),
                    }];
                    uts
                },
                approved_adapter_ids: vec!["adapter.fixture.disabled_write.dry_run".to_string()],
            },
        ],
        adapters: vec![
            ToolAdapterCapabilityV1 {
                adapter_id: "adapter.fixture.safe_read.dry_run".to_string(),
                tool_name: "fixture.safe_read".to_string(),
                tool_version: "1.0.0".to_string(),
                capability_id: "capability.fixture.safe-read".to_string(),
                side_effect_class: UtsSideEffectClassV1::Read,
                execution_environment: UtsExecutionEnvironmentKindV1::DryRun,
                supports_dry_run: true,
                approved_for_binding: true,
            },
            ToolAdapterCapabilityV1 {
                adapter_id: "adapter.fixture.disabled_write.dry_run".to_string(),
                tool_name: "fixture.disabled_write".to_string(),
                tool_version: "1.0.0".to_string(),
                capability_id: "capability.fixture.disabled-write".to_string(),
                side_effect_class: UtsSideEffectClassV1::LocalWrite,
                execution_environment: UtsExecutionEnvironmentKindV1::DryRun,
                supports_dry_run: true,
                approved_for_binding: false,
            },
        ],
    }
}

pub fn wp08_registry_rejection_fixtures(
) -> Vec<(String, ToolBindingRequestV1, ToolRegistryRejectionCodeV1)> {
    vec![
        (
            "model-output-direct-binding".to_string(),
            ToolBindingRequestV1 {
                source: ToolBindingSourceV1::ModelOutput,
                tool_name: "fixture.safe_read".to_string(),
                tool_version: "1.0.0".to_string(),
                adapter_id: "adapter.fixture.safe_read.dry_run".to_string(),
                dry_run_requested: true,
            },
            ToolRegistryRejectionCodeV1::ModelDirectExecutionDenied,
        ),
        (
            "unknown-tool".to_string(),
            ToolBindingRequestV1 {
                source: ToolBindingSourceV1::RegistryCompiler,
                tool_name: "fixture.unknown".to_string(),
                tool_version: "1.0.0".to_string(),
                adapter_id: "adapter.fixture.safe_read.dry_run".to_string(),
                dry_run_requested: true,
            },
            ToolRegistryRejectionCodeV1::UnknownTool,
        ),
        (
            "unregistered-tool".to_string(),
            ToolBindingRequestV1 {
                source: ToolBindingSourceV1::RegistryCompiler,
                tool_name: "fixture.disabled_write".to_string(),
                tool_version: "1.0.0".to_string(),
                adapter_id: "adapter.fixture.disabled_write.dry_run".to_string(),
                dry_run_requested: true,
            },
            ToolRegistryRejectionCodeV1::UnregisteredTool,
        ),
        (
            "incompatible-version".to_string(),
            ToolBindingRequestV1 {
                source: ToolBindingSourceV1::RegistryCompiler,
                tool_name: "fixture.safe_read".to_string(),
                tool_version: "2.0.0".to_string(),
                adapter_id: "adapter.fixture.safe_read.dry_run".to_string(),
                dry_run_requested: true,
            },
            ToolRegistryRejectionCodeV1::IncompatibleVersion,
        ),
        (
            "mismatched-adapter-capabilities".to_string(),
            ToolBindingRequestV1 {
                source: ToolBindingSourceV1::RegistryCompiler,
                tool_name: "fixture.safe_read".to_string(),
                tool_version: "1.0.0".to_string(),
                adapter_id: "adapter.fixture.disabled_write.dry_run".to_string(),
                dry_run_requested: true,
            },
            ToolRegistryRejectionCodeV1::MismatchedAdapterCapabilities,
        ),
        (
            "unsafe-dry-run-posture".to_string(),
            ToolBindingRequestV1 {
                source: ToolBindingSourceV1::RegistryCompiler,
                tool_name: "fixture.safe_read".to_string(),
                tool_version: "1.0.0".to_string(),
                adapter_id: "adapter.fixture.safe_read.dry_run".to_string(),
                dry_run_requested: false,
            },
            ToolRegistryRejectionCodeV1::UnsafeDryRunPosture,
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_request() -> ToolBindingRequestV1 {
        ToolBindingRequestV1 {
            source: ToolBindingSourceV1::RegistryCompiler,
            tool_name: "fixture.safe_read".to_string(),
            tool_version: "1.0.0".to_string(),
            adapter_id: "adapter.fixture.safe_read.dry_run".to_string(),
            dry_run_requested: true,
        }
    }

    #[test]
    fn wp08_registry_fixture_binds_approved_adapter() {
        let registry = wp08_tool_registry_v1_fixture();
        let outcome = bind_tool_registry_v1(&registry, &valid_request());

        assert_eq!(outcome.decision, ToolBindingDecisionV1::Bound);
        let binding = outcome.binding.expect("approved adapter should bind");
        assert_eq!(binding.registry_tool_id, "registry.fixture.safe_read");
        assert_eq!(binding.adapter_id, "adapter.fixture.safe_read.dry_run");
        assert!(binding.dry_run);
        assert!(outcome
            .evidence
            .iter()
            .any(|entry| entry.contains(TOOL_REGISTRY_SCHEMA_VERSION_V1)));
    }

    #[test]
    fn wp08_rejection_fixtures_fail_for_intended_reasons() {
        let registry = wp08_tool_registry_v1_fixture();

        for (case_id, request, expected) in wp08_registry_rejection_fixtures() {
            let outcome = bind_tool_registry_v1(&registry, &request);

            assert_eq!(
                outcome.decision,
                ToolBindingDecisionV1::Rejected,
                "{case_id} should reject"
            );
            assert_eq!(
                outcome.rejection_code,
                Some(expected),
                "{case_id} should reject for intended reason"
            );
            assert!(outcome.binding.is_none(), "{case_id} must not bind");
        }
    }

    #[test]
    fn wp08_model_output_cannot_bind_directly_to_execution() {
        let registry = wp08_tool_registry_v1_fixture();
        let mut request = valid_request();
        request.source = ToolBindingSourceV1::ModelOutput;

        let outcome = bind_tool_registry_v1(&registry, &request);

        assert_eq!(
            outcome.rejection_code,
            Some(ToolRegistryRejectionCodeV1::ModelDirectExecutionDenied)
        );
        assert!(outcome.binding.is_none());
    }

    #[test]
    fn wp08_registry_state_fingerprint_is_explicit_and_deterministic() {
        let registry = wp08_tool_registry_v1_fixture();
        let mut reordered = registry.clone();
        reordered.tools.reverse();
        reordered.adapters.reverse();
        let mut changed_capability = registry.clone();
        changed_capability.adapters[0].capability_id = "capability.fixture.changed".to_string();
        let mut changed_uts = registry.clone();
        changed_uts.tools[0].uts.resources[0].scope = "local-readonly-v2".to_string();

        assert_eq!(
            registry_state_fingerprint_v1(&registry),
            registry_state_fingerprint_v1(&reordered)
        );
        assert!(registry_state_fingerprint_v1(&registry).contains("registry.fixture.safe_read"));
        assert_ne!(
            registry_state_fingerprint_v1(&registry),
            registry_state_fingerprint_v1(&changed_capability)
        );
        assert_ne!(
            registry_state_fingerprint_v1(&registry),
            registry_state_fingerprint_v1(&changed_uts)
        );
    }

    #[test]
    fn wp08_registry_validation_rejects_invalid_uts() {
        let mut registry = wp08_tool_registry_v1_fixture();
        registry.tools[0].uts.schema_version = "uts.v2".to_string();

        let err = validate_tool_registry_v1(&registry).expect_err("invalid UTS should reject");

        assert_eq!(
            err.rejection_code,
            Some(ToolRegistryRejectionCodeV1::InvalidUts)
        );
    }

    #[test]
    fn wp08_registry_validation_rejects_registry_uts_drift() {
        let mut registry = wp08_tool_registry_v1_fixture();
        registry.tools[0].uts.name = "fixture.other".to_string();

        let err = validate_tool_registry_v1(&registry).expect_err("UTS drift should reject");

        assert_eq!(
            err.rejection_code,
            Some(ToolRegistryRejectionCodeV1::InvalidRegistry)
        );
    }

    #[test]
    fn wp08_registry_validation_rejects_missing_approved_adapter() {
        let mut registry = wp08_tool_registry_v1_fixture();
        registry.tools[0].approved_adapter_ids = vec!["adapter.fixture.missing".to_string()];

        let err = validate_tool_registry_v1(&registry)
            .expect_err("missing approved adapter should reject");

        assert_eq!(
            err.rejection_code,
            Some(ToolRegistryRejectionCodeV1::InvalidRegistry)
        );
    }
}
