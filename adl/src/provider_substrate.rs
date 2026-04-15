use anyhow::{anyhow, Result};
use schemars::{schema_for, JsonSchema};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;

use crate::adl;
use crate::provider;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityModeV1 {
    Native,
    PromptBased,
    SemanticFallback,
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct CapabilitySupportV1 {
    pub supported: bool,
    pub mode: CapabilityModeV1,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct ProviderCapabilitiesV1 {
    pub tool_calling: CapabilitySupportV1,
    pub structured_json: CapabilitySupportV1,
    pub semantic_tool_fallback: CapabilitySupportV1,
}

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
    pub capabilities: ProviderCapabilitiesV1,
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
    pub capabilities: ProviderCapabilitiesV1,
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

fn parse_capability_mode(raw: &str) -> Option<CapabilityModeV1> {
    match raw.trim().to_lowercase().as_str() {
        "native" => Some(CapabilityModeV1::Native),
        "prompt_based" | "prompt-based" => Some(CapabilityModeV1::PromptBased),
        "semantic_fallback" | "semantic-fallback" => Some(CapabilityModeV1::SemanticFallback),
        "none" => Some(CapabilityModeV1::None),
        _ => None,
    }
}

fn capability_override(cfg: &HashMap<String, Value>, key: &str) -> Option<CapabilitySupportV1> {
    let caps = cfg.get("capabilities")?.as_object()?;
    let entry = caps.get(key)?.as_object()?;
    let supported = entry.get("supported")?.as_bool()?;
    let mode = entry
        .get("mode")
        .and_then(|v| v.as_str())
        .and_then(parse_capability_mode)?;
    Some(CapabilitySupportV1 { supported, mode })
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
                "chatgpt" => return "openai".to_string(),
                "claude" => return "anthropic".to_string(),
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
        "openai" => "openai".to_string(),
        "anthropic" => "anthropic".to_string(),
        "http" | "http_remote" => "generic_http".to_string(),
        other if !other.is_empty() => other.to_lowercase(),
        _ => "unknown".to_string(),
    }
}

fn infer_transport(spec: &adl::ProviderSpec) -> Result<ProviderTransportV1> {
    match spec.kind.trim() {
        "ollama" => {
            if spec.base_url.is_some() || cfg_str(&spec.config, "endpoint").is_some() {
                Ok(ProviderTransportV1::Http)
            } else {
                Ok(ProviderTransportV1::LocalCli)
            }
        }
        "http" | "http_remote" | "openai" | "anthropic" => Ok(ProviderTransportV1::Http),
        "local_ollama" => Ok(ProviderTransportV1::LocalCli),
        "mock" => Ok(ProviderTransportV1::InProcess),
        other => Err(anyhow!(
            "unsupported provider kind '{other}' for provider substrate v1"
        )),
    }
}

fn infer_capability_defaults(
    transport: &ProviderTransportV1,
    vendor: &str,
    model_ref: Option<&str>,
) -> ProviderCapabilitiesV1 {
    let model = model_ref.unwrap_or("").trim().to_lowercase();
    if vendor == "ollama" {
        let native_supported = model.contains("gpt-oss")
            || model.contains("qwen3-coder")
            || model.contains("qwen2.5-coder");
        let tool_calling = if native_supported {
            CapabilitySupportV1 {
                supported: true,
                mode: CapabilityModeV1::Native,
            }
        } else {
            CapabilitySupportV1 {
                supported: false,
                mode: CapabilityModeV1::None,
            }
        };
        let structured_json = if tool_calling.supported {
            CapabilitySupportV1 {
                supported: true,
                mode: CapabilityModeV1::Native,
            }
        } else {
            CapabilitySupportV1 {
                supported: true,
                mode: CapabilityModeV1::PromptBased,
            }
        };
        return ProviderCapabilitiesV1 {
            tool_calling,
            structured_json,
            semantic_tool_fallback: CapabilitySupportV1 {
                supported: true,
                mode: CapabilityModeV1::SemanticFallback,
            },
        };
    }

    let native_tool_calling = match transport {
        ProviderTransportV1::Http | ProviderTransportV1::InProcess => CapabilitySupportV1 {
            supported: true,
            mode: CapabilityModeV1::Native,
        },
        ProviderTransportV1::LocalCli => {
            let native_supported = model.contains("gpt-oss")
                || model.contains("qwen3-coder")
                || model.contains("qwen2.5-coder");
            if native_supported {
                CapabilitySupportV1 {
                    supported: true,
                    mode: CapabilityModeV1::Native,
                }
            } else {
                CapabilitySupportV1 {
                    supported: false,
                    mode: CapabilityModeV1::None,
                }
            }
        }
    };

    let structured_json = match transport {
        ProviderTransportV1::Http | ProviderTransportV1::InProcess => CapabilitySupportV1 {
            supported: true,
            mode: CapabilityModeV1::Native,
        },
        ProviderTransportV1::LocalCli => {
            if native_tool_calling.supported {
                CapabilitySupportV1 {
                    supported: true,
                    mode: CapabilityModeV1::Native,
                }
            } else {
                CapabilitySupportV1 {
                    supported: true,
                    mode: CapabilityModeV1::PromptBased,
                }
            }
        }
    };

    let semantic_tool_fallback = match transport {
        ProviderTransportV1::LocalCli if vendor == "ollama" => CapabilitySupportV1 {
            supported: true,
            mode: CapabilityModeV1::SemanticFallback,
        },
        _ => CapabilitySupportV1 {
            supported: false,
            mode: CapabilityModeV1::None,
        },
    };

    ProviderCapabilitiesV1 {
        tool_calling: native_tool_calling,
        structured_json,
        semantic_tool_fallback,
    }
}

fn provider_capabilities_v1(
    spec: &adl::ProviderSpec,
    transport: &ProviderTransportV1,
    vendor: &str,
    model_ref: Option<&str>,
) -> ProviderCapabilitiesV1 {
    let mut caps = infer_capability_defaults(transport, vendor, model_ref);
    if let Some(v) = capability_override(&spec.config, "tool_calling") {
        caps.tool_calling = v;
    }
    if let Some(v) = capability_override(&spec.config, "structured_json") {
        caps.structured_json = v;
    }
    if let Some(v) = capability_override(&spec.config, "semantic_tool_fallback") {
        caps.semantic_tool_fallback = v;
    }
    caps
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
    let vendor = infer_vendor(spec);
    let default_model_ref = default_model_ref(spec);
    Ok(ProviderSubstrateV1 {
        provider_id: provider_id.to_string(),
        provider_kind: spec.kind.trim().to_string(),
        vendor: vendor.clone(),
        transport: transport.clone(),
        profile: spec.profile.clone(),
        endpoint: cfg_str(&spec.config, "endpoint").map(ToString::to_string),
        base_url: spec.base_url.clone(),
        default_model_ref: default_model_ref.clone(),
        provider_default_model_id: default_provider_model_id(spec),
        capabilities: provider_capabilities_v1(
            spec,
            &transport,
            &vendor,
            default_model_ref.as_deref(),
        ),
    })
}

pub fn provider_invocation_target_v1(
    provider_id: &str,
    spec: &adl::ProviderSpec,
    model_override: Option<&str>,
) -> Result<ProviderInvocationTargetV1> {
    let substrate = provider_substrate_v1(provider_id, spec)?;
    let transport = substrate.transport.clone();
    let vendor = substrate.vendor.clone();
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
    let capabilities = provider_capabilities_v1(spec, &transport, &vendor, Some(&model_ref));

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
        capabilities,
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
    fn provider_substrate_infers_first_class_claude_profile_vendor() {
        let mut spec = provider_spec("http");
        spec.profile = Some("claude:claude-3-7-sonnet".to_string());
        spec.config.insert(
            "endpoint".to_string(),
            json!("http://127.0.0.1:8787/complete"),
        );
        spec.default_model = Some("claude-3-7-sonnet-latest".to_string());

        let substrate = provider_substrate_v1("claude_primary", &spec).expect("substrate");
        assert_eq!(substrate.provider_id, "claude_primary");
        assert_eq!(substrate.vendor, "anthropic");
        assert_eq!(substrate.transport, ProviderTransportV1::Http);
        assert_eq!(
            substrate.default_model_ref.as_deref(),
            Some("claude-3-7-sonnet-latest")
        );
    }

    #[test]
    fn provider_substrate_accepts_native_openai_and_anthropic_kinds() {
        let mut openai = provider_spec("openai");
        openai.default_model = Some("gpt-test".to_string());
        let openai_substrate =
            provider_substrate_v1("openai_primary", &openai).expect("openai substrate");
        assert_eq!(openai_substrate.vendor, "openai");
        assert_eq!(openai_substrate.transport, ProviderTransportV1::Http);
        assert_eq!(openai_substrate.provider_kind, "openai");

        let mut anthropic = provider_spec("anthropic");
        anthropic.default_model = Some("claude-test".to_string());
        let anthropic_substrate =
            provider_substrate_v1("anthropic_primary", &anthropic).expect("anthropic substrate");
        assert_eq!(anthropic_substrate.vendor, "anthropic");
        assert_eq!(anthropic_substrate.transport, ProviderTransportV1::Http);
        assert_eq!(anthropic_substrate.provider_kind, "anthropic");
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
    fn provider_substrate_uses_http_transport_for_ollama_with_endpoint() {
        let mut spec = provider_spec("ollama");
        spec.base_url = Some("http://192.168.68.73:11434".to_string());
        spec.default_model = Some("phi4-mini".to_string());

        let substrate = provider_substrate_v1("remote_ollama", &spec).expect("substrate");
        assert_eq!(substrate.vendor, "ollama");
        assert_eq!(substrate.transport, ProviderTransportV1::Http);
        assert!(substrate.capabilities.semantic_tool_fallback.supported);
        assert_eq!(
            substrate.capabilities.structured_json.mode,
            CapabilityModeV1::PromptBased
        );
    }

    #[test]
    fn provider_substrate_keeps_local_ollama_cli_transport() {
        let mut spec = provider_spec("local_ollama");
        spec.default_model = Some("phi4-mini".to_string());

        let substrate = provider_substrate_v1("local_ollama", &spec).expect("substrate");
        assert_eq!(substrate.vendor, "ollama");
        assert_eq!(substrate.transport, ProviderTransportV1::LocalCli);
    }

    #[test]
    fn provider_substrate_marks_gpt_oss_ollama_as_tool_capable() {
        let mut spec = provider_spec("ollama");
        spec.default_model = Some("gpt-oss:latest".to_string());

        let substrate = provider_substrate_v1("local", &spec).expect("substrate");
        assert_eq!(substrate.vendor, "ollama");
        assert!(substrate.capabilities.tool_calling.supported);
        assert_eq!(
            substrate.capabilities.tool_calling.mode,
            CapabilityModeV1::Native
        );
        assert!(substrate.capabilities.semantic_tool_fallback.supported);
    }

    #[test]
    fn provider_substrate_marks_deepseek_ollama_for_semantic_fallback() {
        let mut spec = provider_spec("ollama");
        spec.default_model = Some("deepseek-r1:latest".to_string());

        let substrate = provider_substrate_v1("local", &spec).expect("substrate");
        assert_eq!(substrate.vendor, "ollama");
        assert!(!substrate.capabilities.tool_calling.supported);
        assert_eq!(
            substrate.capabilities.tool_calling.mode,
            CapabilityModeV1::None
        );
        assert!(substrate.capabilities.structured_json.supported);
        assert_eq!(
            substrate.capabilities.structured_json.mode,
            CapabilityModeV1::PromptBased
        );
        assert!(substrate.capabilities.semantic_tool_fallback.supported);
        assert_eq!(
            substrate.capabilities.semantic_tool_fallback.mode,
            CapabilityModeV1::SemanticFallback
        );
    }

    #[test]
    fn provider_substrate_honors_explicit_capability_overrides() {
        let mut spec = provider_spec("ollama");
        spec.default_model = Some("deepseek-r1:latest".to_string());
        spec.config.insert(
            "capabilities".to_string(),
            json!({
                "tool_calling": { "supported": true, "mode": "native" },
                "structured_json": { "supported": true, "mode": "native" },
                "semantic_tool_fallback": { "supported": false, "mode": "none" }
            }),
        );

        let substrate = provider_substrate_v1("local", &spec).expect("substrate");
        assert!(substrate.capabilities.tool_calling.supported);
        assert_eq!(
            substrate.capabilities.tool_calling.mode,
            CapabilityModeV1::Native
        );
        assert!(!substrate.capabilities.semantic_tool_fallback.supported);
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
