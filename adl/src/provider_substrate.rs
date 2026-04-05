use anyhow::{anyhow, Result};
use schemars::{schema_for, JsonSchema};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;

use crate::adl;
use crate::provider;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ProviderTransportV1 {
    Http,
    LocalCli,
    InProcess,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct ProviderSubstrateV1 {
    pub provider_id: String,
    pub provider_kind: String,
    pub vendor: String,
    pub transport: ProviderTransportV1,
    #[serde(default)]
    pub profile: Option<String>,
    #[serde(default)]
    pub endpoint: Option<String>,
    #[serde(default)]
    pub base_url: Option<String>,
    #[serde(default)]
    pub default_model_ref: Option<String>,
    #[serde(default)]
    pub provider_default_model_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct ProviderInvocationTargetV1 {
    pub provider_id: String,
    pub provider_kind: String,
    pub vendor: String,
    pub transport: ProviderTransportV1,
    #[serde(default)]
    pub profile: Option<String>,
    #[serde(default)]
    pub endpoint: Option<String>,
    #[serde(default)]
    pub base_url: Option<String>,
    pub model_ref: String,
    pub provider_model_id: String,
}

fn cfg_str<'a>(cfg: &'a HashMap<String, Value>, key: &str) -> Option<&'a str> {
    cfg.get(key).and_then(|v| v.as_str()).map(str::trim)
}

fn normalize_vendor_token(raw: &str) -> Option<String> {
    let token = raw.trim().to_lowercase();
    if token.is_empty() {
        return None;
    }
    let valid = token
        .chars()
        .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '_' || ch == '-');
    valid.then_some(token)
}

fn infer_vendor(spec: &adl::ProviderSpec) -> String {
    if let Some(explicit) = cfg_str(&spec.config, "vendor").and_then(normalize_vendor_token) {
        return explicit;
    }

    if let Some(profile) = spec.profile.as_deref() {
        if let Some((family, _)) = profile.split_once(':') {
            match family {
                "ollama" => return "ollama".to_string(),
                "mock" => return "mock".to_string(),
                "http" => {}
                _ => {}
            }
        }
    }

    let endpoint = spec
        .base_url
        .as_deref()
        .or_else(|| cfg_str(&spec.config, "endpoint"));
    if let Some(endpoint) = endpoint {
        let lower = endpoint.to_lowercase();
        if lower.contains("openai") {
            return "openai".to_string();
        }
        if lower.contains("anthropic") {
            return "anthropic".to_string();
        }
        if lower.contains("googleapis.com")
            || lower.contains("generativelanguage")
            || lower.contains("gemini")
        {
            return "google".to_string();
        }
        if lower.contains("deepseek") {
            return "deepseek".to_string();
        }
        if lower.contains("ollama") || lower.contains("11434") {
            return "ollama".to_string();
        }
    }

    match spec.kind.trim() {
        "ollama" | "local_ollama" => "ollama".to_string(),
        "mock" => "mock".to_string(),
        "http" | "http_remote" => "generic_http".to_string(),
        other if !other.is_empty() => other.to_lowercase(),
        _ => "unknown".to_string(),
    }
}

fn infer_transport(spec: &adl::ProviderSpec) -> Result<ProviderTransportV1> {
    match spec.kind.trim() {
        "http" | "http_remote" => Ok(ProviderTransportV1::Http),
        "ollama" | "local_ollama" => Ok(ProviderTransportV1::LocalCli),
        "mock" => Ok(ProviderTransportV1::InProcess),
        other => Err(anyhow!(
            "unsupported provider kind '{other}' for provider substrate v1"
        )),
    }
}

fn default_model_ref(spec: &adl::ProviderSpec) -> Option<String> {
    cfg_str(&spec.config, "model_ref")
        .map(ToString::to_string)
        .or_else(|| spec.default_model.clone())
        .or_else(|| cfg_str(&spec.config, "model").map(ToString::to_string))
}

fn default_provider_model_id(spec: &adl::ProviderSpec) -> Option<String> {
    cfg_str(&spec.config, "provider_model_id")
        .map(ToString::to_string)
        .or_else(|| cfg_str(&spec.config, "model").map(ToString::to_string))
        .or_else(|| spec.default_model.clone())
}

pub fn provider_substrate_v1(
    provider_id: &str,
    spec: &adl::ProviderSpec,
) -> Result<ProviderSubstrateV1> {
    let transport = infer_transport(spec)?;
    Ok(ProviderSubstrateV1 {
        provider_id: provider_id.to_string(),
        provider_kind: spec.kind.trim().to_string(),
        vendor: infer_vendor(spec),
        transport,
        profile: spec.profile.clone(),
        endpoint: cfg_str(&spec.config, "endpoint").map(ToString::to_string),
        base_url: spec.base_url.clone(),
        default_model_ref: default_model_ref(spec),
        provider_default_model_id: default_provider_model_id(spec),
    })
}

pub fn provider_invocation_target_v1(
    provider_id: &str,
    spec: &adl::ProviderSpec,
    model_override: Option<&str>,
) -> Result<ProviderInvocationTargetV1> {
    let substrate = provider_substrate_v1(provider_id, spec)?;
    let model_ref = model_override
        .map(str::trim)
        .filter(|v| !v.is_empty())
        .map(ToString::to_string)
        .or_else(|| default_model_ref(spec))
        .or_else(|| default_provider_model_id(spec))
        .unwrap_or_else(|| "llama3.1:8b".to_string());

    let provider_model_id = cfg_str(&spec.config, "provider_model_id")
        .map(ToString::to_string)
        .or_else(|| cfg_str(&spec.config, "model").map(ToString::to_string))
        .unwrap_or_else(|| model_ref.clone());

    Ok(ProviderInvocationTargetV1 {
        provider_id: substrate.provider_id,
        provider_kind: substrate.provider_kind,
        vendor: substrate.vendor,
        transport: substrate.transport,
        profile: substrate.profile,
        endpoint: substrate.endpoint,
        base_url: substrate.base_url,
        model_ref,
        provider_model_id,
    })
}

pub fn provider_substrate_manifest_v1(doc: &adl::AdlDoc) -> Result<Value> {
    let expanded = provider::expand_provider_profiles(doc)?;
    let mut ids: Vec<&String> = expanded.providers.keys().collect();
    ids.sort();
    let providers = ids
        .into_iter()
        .map(|provider_id| provider_substrate_v1(provider_id, &expanded.providers[provider_id]))
        .collect::<Result<Vec<_>>>()?;
    Ok(json!({
        "schema_name": "provider_substrate_manifest.v1",
        "schema_version": 1,
        "providers": providers,
    }))
}

pub fn provider_substrate_schema_v1_json() -> Result<String> {
    let schema = schema_for!(ProviderSubstrateV1);
    Ok(serde_json::to_string_pretty(&schema)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn provider_spec(kind: &str) -> adl::ProviderSpec {
        adl::ProviderSpec {
            id: None,
            profile: None,
            kind: kind.to_string(),
            base_url: None,
            default_model: None,
            config: HashMap::new(),
        }
    }

    #[test]
    fn provider_substrate_separates_http_vendor_and_transport() {
        let mut spec = provider_spec("http");
        spec.config.insert(
            "endpoint".to_string(),
            json!("https://api.openai.com/v1/complete"),
        );
        spec.default_model = Some("gpt-4.1-mini".to_string());

        let substrate = provider_substrate_v1("openai_primary", &spec).expect("substrate");
        assert_eq!(substrate.provider_id, "openai_primary");
        assert_eq!(substrate.vendor, "openai");
        assert_eq!(substrate.transport, ProviderTransportV1::Http);
        assert_eq!(substrate.default_model_ref.as_deref(), Some("gpt-4.1-mini"));
    }

    #[test]
    fn invocation_target_keeps_model_ref_distinct_from_provider_model_id() {
        let mut spec = provider_spec("http");
        spec.default_model = Some("reasoning/default".to_string());
        spec.config.insert(
            "provider_model_id".to_string(),
            json!("gpt-4.1-mini-2026-03-01"),
        );
        spec.config.insert(
            "endpoint".to_string(),
            json!("https://api.openai.com/v1/complete"),
        );

        let target = provider_invocation_target_v1("p1", &spec, None).expect("target");
        assert_eq!(target.model_ref, "reasoning/default");
        assert_eq!(target.provider_model_id, "gpt-4.1-mini-2026-03-01");
    }

    #[test]
    fn invocation_target_uses_model_override_as_stable_model_ref() {
        let mut spec = provider_spec("ollama");
        spec.config
            .insert("model".to_string(), json!("phi4-mini-provider-native"));

        let target =
            provider_invocation_target_v1("local", &spec, Some("phi4-mini")).expect("target");
        assert_eq!(target.vendor, "ollama");
        assert_eq!(target.transport, ProviderTransportV1::LocalCli);
        assert_eq!(target.model_ref, "phi4-mini");
        assert_eq!(target.provider_model_id, "phi4-mini-provider-native");
    }

    #[test]
    fn provider_substrate_manifest_is_sorted_and_stable() {
        let mut providers = HashMap::new();
        providers.insert("b".to_string(), provider_spec("mock"));
        providers.insert("a".to_string(), provider_spec("mock"));
        let doc = adl::AdlDoc {
            version: "0.5".to_string(),
            providers,
            tools: HashMap::new(),
            agents: HashMap::new(),
            tasks: HashMap::new(),
            workflows: HashMap::new(),
            patterns: Vec::new(),
            signature: None,
            run: adl::RunSpec {
                id: None,
                name: None,
                created_at: None,
                defaults: adl::RunDefaults::default(),
                workflow_ref: None,
                workflow: None,
                pattern_ref: None,
                inputs: HashMap::new(),
                placement: None,
                remote: None,
                delegation_policy: None,
            },
        };

        let manifest = provider_substrate_manifest_v1(&doc).expect("manifest");
        let providers = manifest
            .get("providers")
            .and_then(|v| v.as_array())
            .expect("providers array");
        assert_eq!(
            providers[0].get("provider_id").and_then(|v| v.as_str()),
            Some("a")
        );
        assert_eq!(
            providers[1].get("provider_id").and_then(|v| v.as_str()),
            Some("b")
        );
    }
}
