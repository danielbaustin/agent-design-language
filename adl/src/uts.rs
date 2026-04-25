use schemars::{schema_for, JsonSchema};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::collections::BTreeMap;

pub const UTS_SCHEMA_VERSION_V1: &str = "uts.v1";

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
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
#[serde(deny_unknown_fields)]
pub struct UniversalToolSchemaV1 {
    pub schema_version: String,
    pub name: String,
    pub version: String,
    pub description: String,
    pub input_schema: JsonValue,
    pub output_schema: JsonValue,
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

fn json_schema_fragment_has_type(value: &JsonValue) -> bool {
    value
        .as_object()
        .and_then(|object| object.get("type"))
        .and_then(JsonValue::as_str)
        .is_some()
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

/// Validate UTS v1 semantic constraints that are stricter than serde shape.
///
/// This validator intentionally treats UTS validity as schema compatibility
/// only. It does not grant ADL runtime authority, adapter binding, Freedom Gate
/// approval, replay permission, or execution permission.
pub fn validate_uts_v1(schema: &UniversalToolSchemaV1) -> Result<(), UtsValidationReport> {
    let mut errors = Vec::new();

    if schema.schema_version != UTS_SCHEMA_VERSION_V1 {
        push_error(
            &mut errors,
            "unsupported_schema_version",
            "schema_version",
            format!("schema_version must be {UTS_SCHEMA_VERSION_V1}"),
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
        if !extension_key_allowed(key) {
            push_error(
                &mut errors,
                "invalid_extension_key",
                "extensions",
                format!("extension key '{key}' is not allowed for UTS v1"),
            );
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(UtsValidationReport { errors })
    }
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

    fn parse_valid(value: JsonValue) -> UniversalToolSchemaV1 {
        serde_json::from_value(value).expect("example should deserialize")
    }

    #[test]
    fn uts_v1_valid_safe_read_example_passes() {
        let schema = parse_valid(valid_safe_read_json());
        validate_uts_v1(&schema).expect("valid safe-read UTS example should pass");
    }

    #[test]
    fn uts_v1_valid_exfiltration_description_passes_without_execution_authority() {
        let mut value = valid_safe_read_json();
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

        let schema = parse_valid(value);
        validate_uts_v1(&schema).expect("dangerous UTS descriptions can be schema-compatible");

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
        value["input_schema"] = json!({});
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
}
