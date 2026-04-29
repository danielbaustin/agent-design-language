use super::{
    push_normalization_error, UtsArgumentNormalizationErrorCodeV1, UtsArgumentNormalizationErrorV1,
    UtsArgumentNormalizationReportV1, WP10_MAX_ARGUMENT_BYTES_V1, WP10_MAX_STRING_BYTES_V1,
};
use crate::tool_registry::{registry_state_fingerprint_v1, ToolRegistryV1};
use crate::uts::UniversalToolSchemaV1;
use serde_json::Value as JsonValue;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};

fn evidence_digest(value: &str) -> String {
    format!("sha256:{:x}", Sha256::digest(value.as_bytes()))
}

pub(crate) fn proposal_arguments_evidence(arguments: &BTreeMap<String, JsonValue>) -> String {
    let canonical_arguments =
        serde_json::to_string(arguments).expect("proposal arguments should serialize");
    format!(
        "proposal_arguments_redacted count={} digest={}",
        arguments.len(),
        evidence_digest(&canonical_arguments)
    )
}

pub(crate) fn normalized_arguments_evidence(arguments: &BTreeMap<String, JsonValue>) -> String {
    let canonical_arguments =
        serde_json::to_string(arguments).expect("normalized arguments should serialize");
    format!(
        "normalized_arguments_redacted count={} digest={}",
        arguments.len(),
        evidence_digest(&canonical_arguments)
    )
}

pub(crate) fn registry_evidence(registry: &ToolRegistryV1) -> String {
    let fingerprint = registry_state_fingerprint_v1(registry);
    format!("registry_state_digest={}", evidence_digest(&fingerprint))
}

fn schema_properties(schema: &UniversalToolSchemaV1) -> BTreeMap<String, JsonValue> {
    schema
        .input_schema
        .keywords
        .get("properties")
        .and_then(JsonValue::as_object)
        .map(|properties| {
            properties
                .iter()
                .map(|(key, value)| (key.clone(), value.clone()))
                .collect()
        })
        .unwrap_or_default()
}

fn schema_required_fields(schema: &UniversalToolSchemaV1) -> BTreeSet<String> {
    schema
        .input_schema
        .keywords
        .get("required")
        .and_then(JsonValue::as_array)
        .into_iter()
        .flatten()
        .filter_map(JsonValue::as_str)
        .map(ToString::to_string)
        .collect()
}

fn schema_allows_additional_fields(schema: &UniversalToolSchemaV1) -> bool {
    schema
        .input_schema
        .keywords
        .get("additionalProperties")
        .and_then(JsonValue::as_bool)
        .unwrap_or(true)
}

fn expected_json_type(property_schema: &JsonValue) -> Option<&str> {
    property_schema
        .as_object()
        .and_then(|schema| schema.get("type"))
        .and_then(JsonValue::as_str)
}

fn value_matches_expected_type(value: &JsonValue, expected_type: &str) -> bool {
    match expected_type {
        "array" => value.is_array(),
        "boolean" => value.is_boolean(),
        "integer" => value.as_i64().is_some() || value.as_u64().is_some(),
        "number" => value.is_number(),
        "object" => value.is_object(),
        "string" => value.is_string(),
        _ => false,
    }
}

fn normalize_value(value: &JsonValue) -> JsonValue {
    match value {
        JsonValue::String(value) => JsonValue::String(value.trim().to_string()),
        JsonValue::Array(values) => JsonValue::Array(values.iter().map(normalize_value).collect()),
        JsonValue::Object(values) => JsonValue::Object(
            values
                .iter()
                .map(|(key, value)| (key.clone(), normalize_value(value)))
                .collect(),
        ),
        _ => value.clone(),
    }
}

fn contains_injection_marker(value: &str) -> bool {
    let lowered = value.to_ascii_lowercase();
    [
        "<script",
        "{{",
        "}}",
        "$(",
        "`",
        "; rm ",
        "ignore previous instructions",
        "system prompt",
    ]
    .iter()
    .any(|marker| lowered.contains(marker))
}

fn contains_path_traversal(value: &str) -> bool {
    let value = value.trim();
    value.contains("../")
        || value.contains("..\\")
        || value.starts_with("~/")
        || value.starts_with('/')
        || value.as_bytes().get(1).is_some_and(|byte| *byte == b':')
}

fn scan_value_safety(
    field: &str,
    value: &JsonValue,
    errors: &mut Vec<UtsArgumentNormalizationErrorV1>,
) {
    match value {
        JsonValue::String(value) => {
            if value.len() > WP10_MAX_STRING_BYTES_V1 {
                push_normalization_error(
                    errors,
                    UtsArgumentNormalizationErrorCodeV1::OversizedPayload,
                    field,
                    "argument string exceeds the bounded fixture limit",
                );
            }
            if contains_injection_marker(value) {
                push_normalization_error(
                    errors,
                    UtsArgumentNormalizationErrorCodeV1::InjectionString,
                    field,
                    "argument string contains an unsafe control marker",
                );
            }
            if contains_path_traversal(value) {
                push_normalization_error(
                    errors,
                    UtsArgumentNormalizationErrorCodeV1::PathTraversal,
                    field,
                    "argument string contains path traversal or absolute path syntax",
                );
            }
        }
        JsonValue::Array(values) => {
            for value in values {
                scan_value_safety(field, value, errors);
            }
        }
        JsonValue::Object(values) => {
            for (key, value) in values {
                if contains_injection_marker(key) {
                    push_normalization_error(
                        errors,
                        UtsArgumentNormalizationErrorCodeV1::InjectionString,
                        field,
                        "argument object key contains an unsafe control marker",
                    );
                }
                if contains_path_traversal(key) {
                    push_normalization_error(
                        errors,
                        UtsArgumentNormalizationErrorCodeV1::PathTraversal,
                        field,
                        "argument object key contains path traversal or absolute path syntax",
                    );
                }
                scan_value_safety(field, value, errors);
            }
        }
        _ => {}
    }
}

pub fn normalize_tool_proposal_arguments_v1(
    schema: &UniversalToolSchemaV1,
    arguments: &BTreeMap<String, JsonValue>,
) -> Result<BTreeMap<String, JsonValue>, UtsArgumentNormalizationReportV1> {
    let mut errors = Vec::new();
    let properties = schema_properties(schema);
    let required = schema_required_fields(schema);
    let allows_additional = schema_allows_additional_fields(schema);

    let serialized = serde_json::to_string(arguments).expect("proposal arguments should serialize");
    if serialized.len() > WP10_MAX_ARGUMENT_BYTES_V1 {
        push_normalization_error(
            &mut errors,
            UtsArgumentNormalizationErrorCodeV1::OversizedPayload,
            "arguments",
            "argument payload exceeds the bounded fixture limit",
        );
    }

    for (field, property_schema) in &properties {
        if !arguments.contains_key(field)
            && property_schema
                .as_object()
                .is_some_and(|schema| schema.contains_key("default"))
        {
            push_normalization_error(
                &mut errors,
                UtsArgumentNormalizationErrorCodeV1::AmbiguousDefault,
                field,
                "schema default is ambiguous for an omitted model-produced argument",
            );
        }
    }

    for required_field in &required {
        if !arguments.contains_key(required_field)
            && !errors.iter().any(|error| error.field == *required_field)
        {
            push_normalization_error(
                &mut errors,
                UtsArgumentNormalizationErrorCodeV1::MissingRequiredArgument,
                required_field,
                "required argument is absent before policy evaluation",
            );
        }
    }

    if !allows_additional {
        for field in arguments.keys() {
            if !properties.contains_key(field) {
                push_normalization_error(
                    &mut errors,
                    UtsArgumentNormalizationErrorCodeV1::UnexpectedAdditionalField,
                    field,
                    "argument is not declared by the UTS input schema",
                );
            }
        }
    }

    let mut normalized = BTreeMap::new();
    for (field, value) in arguments {
        if let Some(property_schema) = properties.get(field) {
            if let Some(expected_type) = expected_json_type(property_schema) {
                if !value_matches_expected_type(value, expected_type) {
                    push_normalization_error(
                        &mut errors,
                        UtsArgumentNormalizationErrorCodeV1::MalformedValue,
                        field,
                        "argument value does not match the declared JSON type",
                    );
                }
            }
        }
        scan_value_safety(field, value, &mut errors);
        normalized.insert(field.clone(), normalize_value(value));
    }

    if errors.is_empty() {
        Ok(normalized)
    } else {
        Err(UtsArgumentNormalizationReportV1 { errors })
    }
}
