use anyhow::{anyhow, Result};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ModelIdentityStrengthV1 {
    Pinned,
    ProviderAsserted,
    TagOnly,
    AdHoc,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct ModelIdentityV1 {
    pub provider_kind: String,
    pub provider: String,
    pub model_ref: String,
    pub provider_model_id: String,
    pub runtime_surface: String,
    pub identity_strength: ModelIdentityStrengthV1,
    pub observed_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolved_digest: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_registry: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub runtime_fingerprint: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inference_parameter_fingerprint: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_surface: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub governance_surface: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evaluator_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lane_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub benchmark_ref: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct EvaluatorIdentityV1 {
    pub evaluator_ref: String,
    pub evaluator_version: String,
    pub prompt_contract_version: String,
    pub classifier_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum LaneKindV1 {
    Regular,
    UtsOnly,
    UtsAccGoverned,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct LaneIdentityV1 {
    pub lane_ref: String,
    pub lane_kind: LaneKindV1,
    pub contract_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct BenchmarkIdentityV1 {
    pub benchmark_ref: String,
    pub benchmark_version: String,
    pub task_panel_digest: String,
    pub model_panel_digest: String,
    pub runner_version: String,
    pub contract_lock_digest: String,
}

pub fn observed_at_now_v1() -> String {
    let seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or_default();
    format!("unix:{seconds}")
}

pub fn normalize_sha256_digest(raw: &str) -> Option<String> {
    let trimmed = raw.trim();
    let digest = trimmed
        .strip_prefix("sha256:")
        .or_else(|| trimmed.strip_prefix("SHA256:"))
        .unwrap_or(trimmed);
    if digest.len() != 64 || !digest.chars().all(|ch| ch.is_ascii_hexdigit()) {
        return None;
    }
    Some(format!("sha256:{}", digest.to_ascii_lowercase()))
}

pub fn stable_text_digest_v1(parts: &[&str]) -> String {
    const OFFSET: u64 = 0xcbf29ce484222325;
    const PRIME: u64 = 0x100000001b3;
    let mut hash = OFFSET;
    for part in parts {
        for byte in part.as_bytes() {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(PRIME);
        }
        hash ^= 0xff;
        hash = hash.wrapping_mul(PRIME);
    }
    format!("fnv1a64:{hash:016x}")
}

pub fn validate_model_identity_v1(identity: &ModelIdentityV1) -> Result<()> {
    require_non_empty("model_identity.provider_kind", &identity.provider_kind)?;
    require_non_empty("model_identity.provider", &identity.provider)?;
    require_non_empty("model_identity.model_ref", &identity.model_ref)?;
    require_non_empty(
        "model_identity.provider_model_id",
        &identity.provider_model_id,
    )?;
    require_non_empty("model_identity.runtime_surface", &identity.runtime_surface)?;
    require_non_empty("model_identity.observed_at", &identity.observed_at)?;

    if let Some(digest) = identity.resolved_digest.as_deref() {
        if normalize_sha256_digest(digest).as_deref() != Some(digest) {
            return Err(anyhow!(
                "model_identity.resolved_digest must be normalized sha256:<64-hex>"
            ));
        }
    }
    if matches!(identity.identity_strength, ModelIdentityStrengthV1::Pinned)
        && identity.resolved_digest.is_none()
    {
        return Err(anyhow!(
            "pinned model identity requires model_identity.resolved_digest"
        ));
    }
    Ok(())
}

fn require_non_empty(field: &str, value: &str) -> Result<()> {
    if value.trim().is_empty() {
        return Err(anyhow!("{field} must not be empty"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base_identity() -> ModelIdentityV1 {
        ModelIdentityV1 {
            provider_kind: "openai".to_string(),
            provider: "openai_primary".to_string(),
            model_ref: "reasoning/default".to_string(),
            provider_model_id: "gpt-5.5".to_string(),
            runtime_surface: "hosted_http".to_string(),
            identity_strength: ModelIdentityStrengthV1::ProviderAsserted,
            observed_at: "unix:1".to_string(),
            resolved_digest: None,
            source_registry: None,
            runtime_fingerprint: None,
            inference_parameter_fingerprint: None,
            tool_surface: None,
            governance_surface: None,
            evaluator_ref: None,
            lane_ref: None,
            benchmark_ref: None,
        }
    }

    #[test]
    fn hosted_provider_asserted_identity_does_not_require_digest() {
        validate_model_identity_v1(&base_identity()).expect("provider asserted hosted identity");
    }

    #[test]
    fn local_pinned_identity_requires_normalized_digest() {
        let mut identity = base_identity();
        identity.provider_kind = "ollama".to_string();
        identity.provider = "local_ollama_http".to_string();
        identity.runtime_surface = "local_http".to_string();
        identity.identity_strength = ModelIdentityStrengthV1::Pinned;
        identity.resolved_digest = Some(format!("sha256:{}", "a".repeat(64)));
        validate_model_identity_v1(&identity).expect("pinned identity with digest");

        identity.resolved_digest = Some("abc123".to_string());
        assert!(validate_model_identity_v1(&identity).is_err());
    }

    #[test]
    fn empty_required_model_identity_fields_fail() {
        let mut identity = base_identity();
        identity.provider_model_id = " ".to_string();
        assert!(validate_model_identity_v1(&identity).is_err());
    }

    #[test]
    fn model_ref_and_provider_model_id_can_differ() {
        let identity = base_identity();
        assert_ne!(identity.model_ref, identity.provider_model_id);
        validate_model_identity_v1(&identity).expect("distinct model refs are valid");
    }
}
