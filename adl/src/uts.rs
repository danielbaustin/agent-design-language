use schemars::{schema_for, JsonSchema};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::collections::{BTreeMap, BTreeSet};

pub const UTS_SCHEMA_VERSION_V1_0: &str = "uts.v1";
pub const UTS_SCHEMA_VERSION_V1_1: &str = "uts.v1.1";
pub const UTS_SCHEMA_VERSION_V1: &str = UTS_SCHEMA_VERSION_V1_0;

#[derive(
    Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, Eq, PartialOrd, Ord,
)]
pub enum UtsCompatibleVersionV1 {
    #[serde(rename = "uts.v1")]
    V1,
    #[serde(rename = "uts.v1.1")]
    V1_1,
}

#[derive(
    Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, Eq, PartialOrd, Ord,
)]
#[serde(rename_all = "snake_case")]
pub enum UtsSideEffectClassV1 {
    Read,
    LocalWrite,
    ExternalRead,
    ExternalWrite,
    Process,
    Network,
    Destructive,
    Exfiltration,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum UtsDeterminismV1 {
    Deterministic,
    BoundedNondeterministic,
    Nondeterministic,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum UtsReplaySafetyV1 {
    ReplaySafe,
    ReplayRequiresApproval,
    NotReplaySafe,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum UtsIdempotenceV1 {
    Idempotent,
    ConditionallyIdempotent,
    NotIdempotent,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum UtsAuthenticationModeV1 {
    None,
    ApiKey,
    #[serde(rename = "oauth")]
    OAuth,
    UserDelegated,
    ServiceAccount,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct UtsAuthenticationRequirementV1 {
    pub mode: UtsAuthenticationModeV1,
    pub required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum UtsDataSensitivityV1 {
    Public,
    Internal,
    Confidential,
    Secret,
    ProtectedPrompt,
    PrivateState,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum UtsExfiltrationRiskV1 {
    None,
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum UtsExecutionEnvironmentKindV1 {
    Fixture,
    DryRun,
    Local,
    ExternalService,
    Process,
    Network,
}

#[derive(
    Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, Eq, PartialOrd, Ord,
)]
#[serde(rename_all = "snake_case")]
pub enum UtsCategoryV1 {
    ReadOnly,
    Computational,
    StateMutating,
    ExternalNetwork,
    HumanVisible,
    GovernanceSensitive,
    IdentitySensitive,
    ContinuitySensitive,
    ObservabilitySensitive,
}

#[derive(
    Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, Eq, PartialOrd, Ord,
)]
#[serde(rename_all = "snake_case")]
pub enum UtsSideEffectTagV1 {
    None,
    LocalState,
    ExternalState,
    Irreversible,
    HumanVisible,
    GovernanceRelevant,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum UtsObservabilityV1 {
    None,
    Basic,
    Full,
    Governance,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Default)]
#[serde(deny_unknown_fields)]
pub struct UtsPlanningMetadataV1 {
    #[serde(default)]
    pub high_risk: Option<bool>,
    #[serde(default)]
    pub irreversible: Option<bool>,
    #[serde(default)]
    pub expensive: Option<bool>,
    #[serde(default)]
    pub slow: Option<bool>,
    #[serde(default)]
    pub review_recommended: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct UtsExecutionEnvironmentV1 {
    pub kind: UtsExecutionEnvironmentKindV1,
    pub isolation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct UtsResourceRequirementV1 {
    pub resource_type: String,
    pub scope: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct UtsErrorModelV1 {
    pub code: String,
    pub message: String,
    pub retryable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct UtsJsonSchemaFragmentV1 {
    #[serde(rename = "type")]
    pub schema_type: String,
    #[serde(flatten, default)]
    pub keywords: BTreeMap<String, JsonValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct UniversalToolSchemaV1 {
    pub schema_version: String,
    pub name: String,
    pub version: String,
    pub description: String,
    pub input_schema: UtsJsonSchemaFragmentV1,
    pub output_schema: UtsJsonSchemaFragmentV1,
    pub side_effect_class: UtsSideEffectClassV1,
    pub determinism: UtsDeterminismV1,
    pub replay_safety: UtsReplaySafetyV1,
    pub idempotence: UtsIdempotenceV1,
    pub resources: Vec<UtsResourceRequirementV1>,
    pub authentication: UtsAuthenticationRequirementV1,
    pub data_sensitivity: UtsDataSensitivityV1,
    pub exfiltration_risk: UtsExfiltrationRiskV1,
    pub execution_environment: UtsExecutionEnvironmentV1,
    pub errors: Vec<UtsErrorModelV1>,
    #[serde(default)]
    pub extensions: BTreeMap<String, JsonValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct UniversalToolSchemaV1_1 {
    pub schema_version: String,
    pub compatible_versions: Vec<UtsCompatibleVersionV1>,
    pub name: String,
    pub version: String,
    pub description: String,
    #[serde(default)]
    pub categories: Option<Vec<UtsCategoryV1>>,
    pub input_schema: UtsJsonSchemaFragmentV1,
    pub output_schema: UtsJsonSchemaFragmentV1,
    pub side_effect_class: UtsSideEffectClassV1,
    #[serde(default)]
    pub side_effects: Option<Vec<UtsSideEffectTagV1>>,
    pub determinism: UtsDeterminismV1,
    pub replay_safety: UtsReplaySafetyV1,
    pub idempotence: UtsIdempotenceV1,
    pub resources: Vec<UtsResourceRequirementV1>,
    pub authentication: UtsAuthenticationRequirementV1,
    pub data_sensitivity: UtsDataSensitivityV1,
    pub exfiltration_risk: UtsExfiltrationRiskV1,
    pub execution_environment: UtsExecutionEnvironmentV1,
    pub errors: Vec<UtsErrorModelV1>,
    #[serde(default)]
    pub observability: Option<UtsObservabilityV1>,
    #[serde(default)]
    pub planning: Option<UtsPlanningMetadataV1>,
    #[serde(default)]
    pub extensions: BTreeMap<String, JsonValue>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UtsValidationError {
    pub code: &'static str,
    pub field: &'static str,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UtsValidationReport {
    pub errors: Vec<UtsValidationError>,
}

impl UtsValidationReport {
    pub fn codes(&self) -> Vec<&'static str> {
        self.errors.iter().map(|error| error.code).collect()
    }
}

fn push_error(
    errors: &mut Vec<UtsValidationError>,
    code: &'static str,
    field: &'static str,
    message: impl Into<String>,
) {
    errors.push(UtsValidationError {
        code,
        field,
        message: message.into(),
    });
}

fn valid_identifier(value: &str) -> bool {
    let mut chars = value.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    if !first.is_ascii_lowercase() {
        return false;
    }
    let len = 1 + chars.clone().count();
    (3..=80).contains(&len)
        && chars.all(|ch| {
            ch.is_ascii_lowercase() || ch.is_ascii_digit() || matches!(ch, '-' | '_' | '.')
        })
}

fn valid_version(value: &str) -> bool {
    let mut parts = value.split('.');
    let Some(major) = parts.next() else {
        return false;
    };
    let Some(minor) = parts.next() else {
        return false;
    };
    let Some(patch) = parts.next() else {
        return false;
    };
    parts.next().is_none()
        && [major, minor, patch]
            .iter()
            .all(|part| !part.is_empty() && part.chars().all(|ch| ch.is_ascii_digit()))
}

fn json_schema_fragment_has_type(value: &UtsJsonSchemaFragmentV1) -> bool {
    !value.schema_type.trim().is_empty()
}

fn valid_token(value: &str) -> bool {
    !value.trim().is_empty()
        && value.chars().all(|ch| {
            ch.is_ascii_lowercase() || ch.is_ascii_digit() || matches!(ch, '-' | '_' | '.')
        })
}

fn extension_key_allowed(key: &str) -> bool {
    let normalized = key.to_ascii_lowercase();
    key == normalized
        && valid_token(key)
        && key.starts_with("x-")
        && !key.contains("authority")
        && !key.contains("grant")
        && !key.contains("freedom_gate")
        && !key.contains("acc")
        && !key.contains("execute")
        && !key.contains("permission")
}

fn extension_declares_required(value: &JsonValue) -> bool {
    value
        .as_object()
        .and_then(|object| object.get("required"))
        .and_then(JsonValue::as_bool)
        .unwrap_or(false)
}

fn unique_count<T: Ord + Copy>(values: &[T]) -> usize {
    values.iter().copied().collect::<BTreeSet<T>>().len()
}

/// Validate UTS v1 semantic constraints that are stricter than serde shape.
///
/// This validator intentionally treats UTS validity as schema compatibility
/// only. It does not grant ADL runtime authority, adapter binding, Freedom Gate
/// approval, replay permission, or execution permission.
pub fn validate_uts_v1(schema: &UniversalToolSchemaV1) -> Result<(), UtsValidationReport> {
    let mut errors = Vec::new();

    if schema.schema_version != UTS_SCHEMA_VERSION_V1_0 {
        push_error(
            &mut errors,
            "unsupported_schema_version",
            "schema_version",
            format!("schema_version must be {UTS_SCHEMA_VERSION_V1_0}"),
        );
    }
    if !valid_identifier(&schema.name) {
        push_error(
            &mut errors,
            "invalid_name",
            "name",
            "name must be 3-80 chars, start lowercase, and use lowercase ascii, digits, dash, underscore, or dot",
        );
    }
    if !valid_version(&schema.version) {
        push_error(
            &mut errors,
            "invalid_version",
            "version",
            "version must use numeric major.minor.patch form",
        );
    }
    if schema.description.trim().len() < 12 {
        push_error(
            &mut errors,
            "missing_description",
            "description",
            "description must be specific enough for reviewers",
        );
    }
    if !json_schema_fragment_has_type(&schema.input_schema) {
        push_error(
            &mut errors,
            "invalid_input_schema",
            "input_schema",
            "input_schema must be a JSON Schema fragment with a type",
        );
    }
    if !json_schema_fragment_has_type(&schema.output_schema) {
        push_error(
            &mut errors,
            "invalid_output_schema",
            "output_schema",
            "output_schema must be a JSON Schema fragment with a type",
        );
    }
    if schema.resources.is_empty() {
        push_error(
            &mut errors,
            "missing_resources",
            "resources",
            "resources must declare at least one resource boundary",
        );
    }
    for resource in &schema.resources {
        if !valid_token(&resource.resource_type) || resource.scope.trim().is_empty() {
            push_error(
                &mut errors,
                "invalid_resource",
                "resources",
                "resource_type must be token-like and scope must be non-empty",
            );
        }
    }
    if schema.authentication.required
        && matches!(schema.authentication.mode, UtsAuthenticationModeV1::None)
    {
        push_error(
            &mut errors,
            "invalid_authentication",
            "authentication",
            "required authentication cannot use mode none",
        );
    }
    if matches!(schema.side_effect_class, UtsSideEffectClassV1::Exfiltration)
        && !matches!(schema.exfiltration_risk, UtsExfiltrationRiskV1::High)
    {
        push_error(
            &mut errors,
            "invalid_exfiltration_risk",
            "exfiltration_risk",
            "exfiltration side effects must declare high exfiltration risk",
        );
    }
    if !matches!(schema.side_effect_class, UtsSideEffectClassV1::Exfiltration)
        && matches!(schema.exfiltration_risk, UtsExfiltrationRiskV1::High)
    {
        push_error(
            &mut errors,
            "ambiguous_side_effects",
            "side_effect_class",
            "high exfiltration risk must use the exfiltration side-effect class",
        );
    }
    if schema.execution_environment.isolation.trim().is_empty() {
        push_error(
            &mut errors,
            "missing_execution_isolation",
            "execution_environment.isolation",
            "execution environment must describe isolation posture",
        );
    }
    if schema.errors.is_empty() {
        push_error(
            &mut errors,
            "missing_error_model",
            "errors",
            "at least one error model entry is required",
        );
    }
    for error in &schema.errors {
        if !valid_token(&error.code) || error.message.trim().is_empty() {
            push_error(
                &mut errors,
                "invalid_error_model",
                "errors",
                "error code must be token-like and message must be non-empty",
            );
        }
    }
    for key in schema.extensions.keys() {
        let value = &schema.extensions[key];
        if !extension_key_allowed(key) {
            push_error(
                &mut errors,
                "invalid_extension_key",
                "extensions",
                format!("extension key '{key}' is not allowed for UTS v1"),
            );
        } else if extension_declares_required(value) {
            push_error(
                &mut errors,
                "unsupported_required_extension",
                "extensions",
                format!("extension key '{key}' declares unsupported required behavior"),
            );
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(UtsValidationReport { errors })
    }
}

pub fn validate_uts_v1_1(schema: &UniversalToolSchemaV1_1) -> Result<(), UtsValidationReport> {
    let mut errors = Vec::new();

    if schema.schema_version != UTS_SCHEMA_VERSION_V1_1 {
        push_error(
            &mut errors,
            "unsupported_schema_version",
            "schema_version",
            format!("schema_version must be {UTS_SCHEMA_VERSION_V1_1}"),
        );
    }
    if schema.compatible_versions.is_empty() {
        push_error(
            &mut errors,
            "missing_compatible_versions",
            "compatible_versions",
            "compatible_versions must declare at least one supported UTS version",
        );
    } else {
        if unique_count(&schema.compatible_versions) != schema.compatible_versions.len() {
            push_error(
                &mut errors,
                "duplicate_compatible_versions",
                "compatible_versions",
                "compatible_versions must not contain duplicates",
            );
        }
        if !schema
            .compatible_versions
            .contains(&UtsCompatibleVersionV1::V1_1)
        {
            push_error(
                &mut errors,
                "missing_current_compatible_version",
                "compatible_versions",
                "compatible_versions must include uts.v1.1 for v1.1 tool definitions",
            );
        }
    }
    if !valid_identifier(&schema.name) {
        push_error(
            &mut errors,
            "invalid_name",
            "name",
            "name must be 3-80 chars, start lowercase, and use lowercase ascii, digits, dash, underscore, or dot",
        );
    }
    if !valid_version(&schema.version) {
        push_error(
            &mut errors,
            "invalid_version",
            "version",
            "version must use numeric major.minor.patch form",
        );
    }
    if schema.description.trim().len() < 12 {
        push_error(
            &mut errors,
            "missing_description",
            "description",
            "description must be specific enough for reviewers",
        );
    }
    if !json_schema_fragment_has_type(&schema.input_schema) {
        push_error(
            &mut errors,
            "invalid_input_schema",
            "input_schema",
            "input_schema must be a JSON Schema fragment with a type",
        );
    }
    if !json_schema_fragment_has_type(&schema.output_schema) {
        push_error(
            &mut errors,
            "invalid_output_schema",
            "output_schema",
            "output_schema must be a JSON Schema fragment with a type",
        );
    }
    if schema.resources.is_empty() {
        push_error(
            &mut errors,
            "missing_resources",
            "resources",
            "resources must declare at least one resource boundary",
        );
    }
    for resource in &schema.resources {
        if !valid_token(&resource.resource_type) || resource.scope.trim().is_empty() {
            push_error(
                &mut errors,
                "invalid_resource",
                "resources",
                "resource_type must be token-like and scope must be non-empty",
            );
        }
    }
    if let Some(categories) = &schema.categories {
        if categories.is_empty() {
            push_error(
                &mut errors,
                "invalid_categories",
                "categories",
                "categories must not be empty when present",
            );
        } else if unique_count(categories) != categories.len() {
            push_error(
                &mut errors,
                "duplicate_categories",
                "categories",
                "categories must not contain duplicates",
            );
        }
    }
    if let Some(side_effects) = &schema.side_effects {
        if side_effects.is_empty() {
            push_error(
                &mut errors,
                "invalid_side_effects",
                "side_effects",
                "side_effects must not be empty when present",
            );
        } else if unique_count(side_effects) != side_effects.len() {
            push_error(
                &mut errors,
                "duplicate_side_effects",
                "side_effects",
                "side_effects must not contain duplicates",
            );
        } else if side_effects.contains(&UtsSideEffectTagV1::None) && side_effects.len() > 1 {
            push_error(
                &mut errors,
                "contradictory_side_effects",
                "side_effects",
                "side_effects cannot combine none with other side-effect tags",
            );
        }
    }
    if schema.authentication.required
        && matches!(schema.authentication.mode, UtsAuthenticationModeV1::None)
    {
        push_error(
            &mut errors,
            "invalid_authentication",
            "authentication",
            "required authentication cannot use mode none",
        );
    }
    if matches!(schema.side_effect_class, UtsSideEffectClassV1::Exfiltration)
        && !matches!(schema.exfiltration_risk, UtsExfiltrationRiskV1::High)
    {
        push_error(
            &mut errors,
            "invalid_exfiltration_risk",
            "exfiltration_risk",
            "exfiltration side effects must declare high exfiltration risk",
        );
    }
    if !matches!(schema.side_effect_class, UtsSideEffectClassV1::Exfiltration)
        && matches!(schema.exfiltration_risk, UtsExfiltrationRiskV1::High)
    {
        push_error(
            &mut errors,
            "ambiguous_side_effects",
            "side_effect_class",
            "high exfiltration risk must use the exfiltration side-effect class",
        );
    }
    if schema.execution_environment.isolation.trim().is_empty() {
        push_error(
            &mut errors,
            "missing_execution_isolation",
            "execution_environment.isolation",
            "execution environment must describe isolation posture",
        );
    }
    if schema.errors.is_empty() {
        push_error(
            &mut errors,
            "missing_error_model",
            "errors",
            "at least one error model entry is required",
        );
    }
    for error in &schema.errors {
        if !valid_token(&error.code) || error.message.trim().is_empty() {
            push_error(
                &mut errors,
                "invalid_error_model",
                "errors",
                "error code must be token-like and message must be non-empty",
            );
        }
    }
    for key in schema.extensions.keys() {
        let value = &schema.extensions[key];
        if !extension_key_allowed(key) {
            push_error(
                &mut errors,
                "invalid_extension_key",
                "extensions",
                format!("extension key '{key}' is not allowed for UTS v1.1"),
            );
        } else if extension_declares_required(value) {
            push_error(
                &mut errors,
                "unsupported_required_extension",
                "extensions",
                format!("extension key '{key}' declares unsupported required behavior"),
            );
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(UtsValidationReport { errors })
    }
}

pub fn upgrade_uts_v1_to_v1_1(schema: UniversalToolSchemaV1) -> UniversalToolSchemaV1_1 {
    UniversalToolSchemaV1_1 {
        schema_version: UTS_SCHEMA_VERSION_V1_1.to_string(),
        compatible_versions: vec![UtsCompatibleVersionV1::V1, UtsCompatibleVersionV1::V1_1],
        name: schema.name,
        version: schema.version,
        description: schema.description,
        categories: None,
        input_schema: schema.input_schema,
        output_schema: schema.output_schema,
        side_effect_class: schema.side_effect_class,
        side_effects: None,
        determinism: schema.determinism,
        replay_safety: schema.replay_safety,
        idempotence: schema.idempotence,
        resources: schema.resources,
        authentication: schema.authentication,
        data_sensitivity: schema.data_sensitivity,
        exfiltration_risk: schema.exfiltration_risk,
        execution_environment: schema.execution_environment,
        errors: schema.errors,
        observability: None,
        planning: None,
        extensions: schema.extensions,
    }
}

impl From<UniversalToolSchemaV1> for UniversalToolSchemaV1_1 {
    fn from(schema: UniversalToolSchemaV1) -> Self {
        upgrade_uts_v1_to_v1_1(schema)
    }
}

pub fn uts_v1_1_schema_json() -> JsonValue {
    serde_json::to_value(schema_for!(UniversalToolSchemaV1_1))
        .expect("UTS v1.1 schema should serialize")
}

pub fn uts_v1_schema_json() -> JsonValue {
    serde_json::to_value(schema_for!(UniversalToolSchemaV1))
        .expect("UTS v1 schema should serialize")
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn valid_safe_read_json() -> JsonValue {
        json!({
            "schema_version": "uts.v1",
            "name": "fixture.safe_read",
            "version": "1.0.0",
            "description": "Read a bounded local fixture for reviewer-visible conformance.",
            "input_schema": {
                "type": "object",
                "properties": {
                    "fixture_id": { "type": "string" }
                },
                "required": ["fixture_id"],
                "additionalProperties": false
            },
            "output_schema": {
                "type": "object",
                "properties": {
                    "content": { "type": "string" }
                },
                "required": ["content"],
                "additionalProperties": false
            },
            "side_effect_class": "read",
            "determinism": "deterministic",
            "replay_safety": "replay_safe",
            "idempotence": "idempotent",
            "resources": [
                { "resource_type": "fixture", "scope": "local-readonly" }
            ],
            "authentication": { "mode": "none", "required": false },
            "data_sensitivity": "internal",
            "exfiltration_risk": "none",
            "execution_environment": {
                "kind": "fixture",
                "isolation": "local deterministic fixture only"
            },
            "errors": [
                {
                    "code": "fixture_not_found",
                    "message": "The requested fixture is not available.",
                    "retryable": false
                }
            ],
            "extensions": {
                "x-adl-review-note": "UTS compatibility only; no execution authority."
            }
        })
    }

    fn valid_safe_read_v1_1_json() -> JsonValue {
        json!({
            "schema_version": "uts.v1.1",
            "compatible_versions": ["uts.v1", "uts.v1.1"],
            "name": "fixture.safe_read",
            "version": "1.0.0",
            "description": "Read a bounded local fixture for reviewer-visible conformance.",
            "categories": ["read_only"],
            "input_schema": {
                "type": "object",
                "properties": {
                    "fixture_id": { "type": "string" }
                },
                "required": ["fixture_id"],
                "additionalProperties": false
            },
            "output_schema": {
                "type": "object",
                "properties": {
                    "content": { "type": "string" }
                },
                "required": ["content"],
                "additionalProperties": false
            },
            "side_effect_class": "read",
            "side_effects": ["none"],
            "determinism": "deterministic",
            "replay_safety": "replay_safe",
            "idempotence": "idempotent",
            "resources": [
                { "resource_type": "fixture", "scope": "local-readonly" }
            ],
            "authentication": { "mode": "none", "required": false },
            "data_sensitivity": "internal",
            "exfiltration_risk": "none",
            "execution_environment": {
                "kind": "fixture",
                "isolation": "local deterministic fixture only"
            },
            "errors": [
                {
                    "code": "fixture_not_found",
                    "message": "The requested fixture is not available.",
                    "retryable": false
                }
            ],
            "observability": "basic",
            "planning": {
                "review_recommended": false
            },
            "extensions": {
                "x-adl-review-note": "UTS compatibility only; no execution authority."
            }
        })
    }

    fn parse_valid(value: JsonValue) -> UniversalToolSchemaV1 {
        serde_json::from_value(value).expect("example should deserialize")
    }

    fn parse_valid_v1_1(value: JsonValue) -> UniversalToolSchemaV1_1 {
        serde_json::from_value(value).expect("example should deserialize")
    }

    #[test]
    fn uts_v1_valid_safe_read_example_passes() {
        let schema = parse_valid(valid_safe_read_json());
        validate_uts_v1(&schema).expect("valid safe-read UTS example should pass");
    }

    #[test]
    fn uts_v1_valid_exfiltration_description_passes_without_execution_authority() {
        let mut value = valid_safe_read_v1_1_json();
        value["name"] = json!("fixture.exfiltration_trap");
        value["side_effect_class"] = json!("exfiltration");
        value["exfiltration_risk"] = json!("high");
        value["data_sensitivity"] = json!("secret");
        value["execution_environment"] = json!({
            "kind": "dry_run",
            "isolation": "denial fixture only; no protected payload leaves the test process"
        });
        value["resources"] = json!([
            { "resource_type": "protected_prompt", "scope": "redacted-denial-fixture" }
        ]);

        let schema = parse_valid_v1_1(value);
        validate_uts_v1_1(&schema).expect("dangerous UTS descriptions can be schema-compatible");

        assert!(
            schema
                .extensions
                .keys()
                .all(|key| !key.contains("authority")),
            "valid UTS example must not smuggle authority through extensions"
        );
    }

    #[test]
    fn uts_v1_invalid_examples_fail_for_intended_reasons() {
        let mut value = valid_safe_read_json();
        value["name"] = json!("Bad Name");
        value["version"] = json!("v1");
        value["input_schema"] = json!({ "type": "" });
        value["resources"] = json!([]);

        let schema = parse_valid(value);
        let err = validate_uts_v1(&schema).expect_err("invalid example should fail");
        let codes = err.codes();

        assert!(codes.contains(&"invalid_name"));
        assert!(codes.contains(&"invalid_version"));
        assert!(codes.contains(&"invalid_input_schema"));
        assert!(codes.contains(&"missing_resources"));
    }

    #[test]
    fn uts_v1_rejects_required_authentication_without_mode() {
        let mut value = valid_safe_read_json();
        value["authentication"] = json!({ "mode": "none", "required": true });

        let schema = parse_valid(value);
        let err = validate_uts_v1(&schema).expect_err("invalid auth should fail");

        assert!(err.codes().contains(&"invalid_authentication"));
    }

    #[test]
    fn uts_v1_accepts_oauth_wire_spelling_and_optional_extension_metadata() {
        let mut value = valid_safe_read_json();
        value["name"] = json!("fixture.external_write");
        value["side_effect_class"] = json!("external_write");
        value["authentication"] = json!({ "mode": "oauth", "required": true });
        value["data_sensitivity"] = json!("confidential");
        value["exfiltration_risk"] = json!("medium");
        value["execution_environment"] = json!({
            "kind": "external_service",
            "isolation": "bounded external-write fixture; schema compatibility only"
        });
        value["extensions"] = json!({
            "x-vendor-metadata": {
                "required": false,
                "review_note": "portable optional metadata"
            }
        });

        let schema = parse_valid(value);
        validate_uts_v1(&schema).expect("oauth wire spelling and optional metadata should pass");
    }

    #[test]
    fn uts_v1_rejects_high_exfiltration_risk_without_exfiltration_class() {
        let mut value = valid_safe_read_json();
        value["side_effect_class"] = json!("network");
        value["exfiltration_risk"] = json!("high");

        let schema = parse_valid(value);
        let err = validate_uts_v1(&schema).expect_err("ambiguous high-risk schema should fail");

        assert!(err.codes().contains(&"ambiguous_side_effects"));
    }

    #[test]
    fn uts_v1_rejects_required_extension_metadata() {
        let mut value = valid_safe_read_json();
        value["extensions"] = json!({
            "x-vendor-required-mode": {
                "required": true,
                "mode": "vendor-private"
            }
        });

        let schema = parse_valid(value);
        let err = validate_uts_v1(&schema).expect_err("required extension should fail");

        assert!(err.codes().contains(&"unsupported_required_extension"));
    }

    #[test]
    fn uts_v1_1_rejects_schema_without_current_compatible_version() {
        let mut value = valid_safe_read_v1_1_json();
        value["compatible_versions"] = json!(["uts.v1"]);

        let schema = parse_valid_v1_1(value);
        let err = validate_uts_v1_1(&schema)
            .expect_err("v1.1 schema without current version should fail");

        assert!(err.codes().contains(&"missing_current_compatible_version"));
    }

    #[test]
    fn uts_v1_1_rejects_contradictory_side_effect_tags() {
        let mut value = valid_safe_read_v1_1_json();
        value["side_effects"] = json!(["none", "human_visible"]);

        let schema = parse_valid_v1_1(value);
        let err = validate_uts_v1_1(&schema).expect_err("contradictory side effects should fail");

        assert!(err.codes().contains(&"contradictory_side_effects"));
    }

    #[test]
    fn uts_v1_validation_reports_semantic_error_families() {
        let mut value = valid_safe_read_json();
        value["name"] = json!("ab");
        value["version"] = json!("1.0");
        value["description"] = json!("short");
        value["output_schema"] = json!({ "type": "" });
        value["resources"] = json!([
            { "resource_type": "Bad Token", "scope": "" }
        ]);
        value["side_effect_class"] = json!("exfiltration");
        value["exfiltration_risk"] = json!("medium");
        value["execution_environment"] = json!({
            "kind": "fixture",
            "isolation": ""
        });
        value["errors"] = json!([
            {
                "code": "Bad Code",
                "message": "",
                "retryable": false
            }
        ]);
        value["extensions"] = json!({
            "not-x": true
        });

        let schema = parse_valid(value);
        let err = validate_uts_v1(&schema).expect_err("invalid schema should fail");
        let codes = err.codes();

        for code in [
            "invalid_name",
            "invalid_version",
            "missing_description",
            "invalid_output_schema",
            "invalid_resource",
            "invalid_exfiltration_risk",
            "missing_execution_isolation",
            "invalid_error_model",
            "invalid_extension_key",
        ] {
            assert!(codes.contains(&code), "missing validation code {code}");
        }
    }

    #[test]
    fn uts_v1_rejects_missing_error_model() {
        let mut value = valid_safe_read_json();
        value["errors"] = json!([]);

        let schema = parse_valid(value);
        let err = validate_uts_v1(&schema).expect_err("missing error model should fail");

        assert!(err.codes().contains(&"missing_error_model"));
    }

    #[test]
    fn uts_v1_rejects_authority_grant_extensions() {
        let mut value = valid_safe_read_json();
        value["extensions"] = json!({
            "x-adl-authority-grant": "operator"
        });

        let schema = parse_valid(value);
        let err = validate_uts_v1(&schema).expect_err("authority extension should fail");

        assert!(err.codes().contains(&"invalid_extension_key"));
    }

    #[test]
    fn uts_v1_rejects_mixed_case_authority_extensions() {
        let mut value = valid_safe_read_json();
        value["extensions"] = json!({
            "x-ADL-Authority-Grant": "operator"
        });

        let schema = parse_valid(value);
        let err = validate_uts_v1(&schema).expect_err("mixed-case authority extension should fail");

        assert!(err.codes().contains(&"invalid_extension_key"));
    }

    #[test]
    fn uts_v1_rejects_unknown_runtime_authority_fields() {
        let mut value = valid_safe_read_json();
        value["authority_grant"] = json!("operator");

        let err = serde_json::from_value::<UniversalToolSchemaV1>(value)
            .expect_err("unknown authority_grant field should not deserialize");

        assert!(err.to_string().contains("unknown field"));
    }

    #[test]
    fn uts_v1_rejects_untyped_nested_schema_fragments_at_deserialize_boundary() {
        let mut value = valid_safe_read_json();
        value["input_schema"] = json!({
            "properties": {
                "fixture_id": { "type": "string" }
            }
        });

        let err = serde_json::from_value::<UniversalToolSchemaV1>(value)
            .expect_err("input_schema without a nested type must not deserialize");

        assert!(err.to_string().contains("missing field `type`"));
    }

    fn resolve_generated_property_schema<'a>(
        root: &'a JsonValue,
        property: &'a JsonValue,
    ) -> &'a JsonValue {
        let Some(reference) = property.get("$ref").and_then(JsonValue::as_str) else {
            return property;
        };
        let Some(name) = reference
            .strip_prefix("#/definitions/")
            .or_else(|| reference.strip_prefix("#/$defs/"))
        else {
            panic!("unsupported generated schema reference {reference}");
        };

        root.get("definitions")
            .or_else(|| root.get("$defs"))
            .and_then(|definitions| definitions.get(name))
            .unwrap_or_else(|| panic!("missing generated schema definition {name}"))
    }

    #[test]
    fn uts_v1_schema_generation_exposes_required_surface() {
        let schema = uts_v1_schema_json();
        let properties = schema
            .get("properties")
            .and_then(JsonValue::as_object)
            .expect("generated UTS schema should expose properties");

        for key in [
            "name",
            "version",
            "input_schema",
            "output_schema",
            "side_effect_class",
            "determinism",
            "replay_safety",
            "idempotence",
            "resources",
            "authentication",
            "data_sensitivity",
            "exfiltration_risk",
            "execution_environment",
            "errors",
            "extensions",
        ] {
            assert!(
                properties.contains_key(key),
                "UTS schema missing property {key}"
            );
        }
    }

    #[test]
    fn uts_v1_1_schema_generation_exposes_additive_surface() {
        let schema = uts_v1_1_schema_json();
        let properties = schema
            .get("properties")
            .and_then(JsonValue::as_object)
            .expect("generated UTS schema should expose properties");

        for key in [
            "compatible_versions",
            "name",
            "version",
            "categories",
            "input_schema",
            "output_schema",
            "side_effect_class",
            "side_effects",
            "determinism",
            "replay_safety",
            "idempotence",
            "resources",
            "authentication",
            "data_sensitivity",
            "exfiltration_risk",
            "execution_environment",
            "errors",
            "observability",
            "planning",
            "extensions",
        ] {
            assert!(
                properties.contains_key(key),
                "UTS schema missing property {key}"
            );
        }
    }

    #[test]
    fn uts_v1_schema_generation_requires_typed_nested_schema_fragments() {
        let schema = uts_v1_schema_json();
        let properties = schema
            .get("properties")
            .and_then(JsonValue::as_object)
            .expect("generated UTS schema should expose properties");

        for key in ["input_schema", "output_schema"] {
            let property = properties
                .get(key)
                .unwrap_or_else(|| panic!("generated UTS schema missing property {key}"));
            let fragment_schema = resolve_generated_property_schema(&schema, property);
            let required = fragment_schema
                .get("required")
                .and_then(JsonValue::as_array)
                .expect("nested JSON Schema fragment should list required fields");

            assert_eq!(
                fragment_schema.get("type").and_then(JsonValue::as_str),
                Some("object"),
                "{key} should be generated as an object schema"
            );
            assert!(
                required.contains(&json!("type")),
                "{key} should require a nested JSON Schema type field"
            );
            assert_eq!(
                fragment_schema
                    .pointer("/properties/type/type")
                    .and_then(JsonValue::as_str),
                Some("string"),
                "{key}.type should be generated as a string field"
            );
        }
    }
}
