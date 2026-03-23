use anyhow::{anyhow, Context, Result};
use jsonschema::{Draft, JSONSchema};
use once_cell::sync::Lazy;
use schemars::schema_for;
use serde_json::Value as JsonValue;

/// **Written** JSON Schema for ADL v0.1 (committed to the repo).
///
/// Location: `schemas/adl.schema.json` (relative to the crate root).
///
/// Notes:
/// - This file is treated as a *published artifact* for humans/tools.
/// - Runtime validation intentionally uses the generated schema below so it stays
///   in lockstep with our Rust structs (and avoids schema drift).
pub static ADL_SCHEMA_JSON: Lazy<JsonValue> = Lazy::new(|| {
    let raw = include_str!("../schemas/adl.schema.json");
    serde_json::from_str::<JsonValue>(raw).expect("schemas/adl.schema.json must be valid JSON")
});

/// Schema generated directly from the Rust ADL structs.
///
/// This is the authoritative validator for v0.1 because it matches `crate::adl::*`.
static ADL_SCHEMA_GENERATED: Lazy<JsonValue> = Lazy::new(|| {
    let schema = schema_for!(crate::adl::AdlDoc);
    serde_json::to_value(&schema).expect("schemars schema must serialize to JSON")
});

/// Compiled schema from the generated version (generally permissive at nested levels).
static ADL_SCHEMA_LOOSE: Lazy<JSONSchema> = Lazy::new(|| {
    let schema_json: &JsonValue = &ADL_SCHEMA_GENERATED;

    JSONSchema::options()
        .with_draft(Draft::Draft7)
        .compile(schema_json)
        .expect("failed to compile generated ADL JSON schema")
});

/// A strict-ish validator for v0.1 that enforces:
/// - top-level `version` and `run` are required
/// - unknown top-level keys are rejected
///
/// Nested objects remain permissive, but this catches common typos early.
static ADL_SCHEMA_STRICT_TOPLEVEL: Lazy<JSONSchema> = Lazy::new(|| {
    let mut schema_json = ADL_SCHEMA_GENERATED.clone();

    if let Some(obj) = schema_json.as_object_mut() {
        // Reject unknown keys at the top level.
        obj.insert("additionalProperties".to_string(), JsonValue::Bool(false));

        // Require the core fields for v0.1.
        obj.insert(
            "required".to_string(),
            JsonValue::Array(vec![
                JsonValue::String("version".to_string()),
                JsonValue::String("run".to_string()),
            ]),
        );
    }

    JSONSchema::options()
        .with_draft(Draft::Draft7)
        .compile(&schema_json)
        .expect("failed to compile strict top-level ADL JSON schema")
});

/// Validate an ADL YAML document against the v0.1 schema.
///
/// This is intentionally strict-ish for v0.1: if the schema rejects, we fail early.
pub fn validate_adl_yaml(yaml_text: &str) -> Result<()> {
    // Parse YAML -> serde_yaml::Value
    let yaml_value: serde_yaml::Value =
        serde_yaml::from_str(yaml_text).context("parse yaml into Value")?;

    // Convert YAML -> JSON Value (jsonschema validates JSON)
    let json_value: JsonValue =
        serde_json::to_value(&yaml_value).context("convert yaml value to json value")?;

    // Prefer strict top-level schema for user-facing validation.
    let compiled = &*ADL_SCHEMA_STRICT_TOPLEVEL;

    // IMPORTANT: `validate()` returns an iterator that *borrows* `json_value`.
    // We must consume it before `json_value` is dropped.
    let result: Result<()> = match compiled.validate(&json_value) {
        Ok(()) => Ok(()),
        Err(err_iter) => {
            let mut msgs: Vec<String> = Vec::new();
            for (i, e) in err_iter.enumerate() {
                if i >= 10 {
                    msgs.push("... (more schema errors omitted)".to_string());
                    break;
                }
                let instance_path = e.instance_path.to_string();
                let path = if instance_path.is_empty() {
                    "/".to_string()
                } else {
                    instance_path
                };
                msgs.push(format!("at {path}: {e}"));
            }

            Err(anyhow!(
                "ADL schema validation failed:\n{}",
                msgs.join("\n")
            ))
        }
    };

    // Returning `result` ensures the borrow/iterator is dropped before `json_value`.
    result
}

/// Expose a compiled schema for any internal uses.
///
/// Keeping this available lets us add a CLI like `--schema` without recompiling.
#[allow(dead_code)]
pub fn compiled_schema_loose() -> &'static JSONSchema {
    &ADL_SCHEMA_LOOSE
}

/// Expose the *committed* (written) schema JSON for CLI/display.
#[allow(dead_code)]
pub fn committed_schema_json() -> &'static JsonValue {
    &ADL_SCHEMA_JSON
}
