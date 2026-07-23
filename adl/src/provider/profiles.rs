//! Provider profile presets and expansion helpers.
//!
//! This module maps profile names to deterministic provider defaults and expands
//! ADL documents into explicit provider specs before execution.
use super::*;
use reqwest::Url;

/// Profile payload used by `provider_profile_registry`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ProviderProfilePreset {
    pub(crate) kind: &'static str,
    pub(crate) default_model: Option<&'static str>,
    pub(crate) provider_model_id: Option<&'static str>,
    pub(crate) endpoint: Option<&'static str>,
}

const HTTP_PROFILE_PLACEHOLDER_ENDPOINT: &str = "https://api.example.invalid/v1/complete";
const INVALID_ENDPOINT_HOST_MARKER: &str = "example.invalid";

fn profile_vendor(profile: &str) -> Option<&'static str> {
    match profile.split_once(':').map(|(family, _)| family) {
        Some("kimi") => Some("kimi"),
        Some("minimax") => Some("minimax"),
        Some("qwen") => Some("qwen"),
        Some("xai") => Some("xai"),
        Some("mistral") => Some("mistral"),
        Some("cohere") => Some("cohere"),
        Some("deepseek") => Some("deepseek"),
        Some("z_ai" | "zai" | "zhipu") => Some("z_ai"),
        Some("gemini") => Some("google"),
        Some("chatgpt") => Some("openai"),
        Some("claude") => Some("anthropic"),
        _ => None,
    }
}

/// Validate that a profile-provided endpoint is usable and non-placeholder.
pub(crate) fn validate_profile_endpoint(
    provider_id: &str,
    profile_name: &str,
    endpoint: &str,
) -> Result<()> {
    let trimmed = endpoint.trim();
    if trimmed.is_empty()
        || trimmed == HTTP_PROFILE_PLACEHOLDER_ENDPOINT
        || trimmed.contains(INVALID_ENDPOINT_HOST_MARKER)
    {
        return Err(anyhow!(
            "providers.{provider_id}.profile '{}' has placeholder or invalid endpoint; configure providers.{provider_id}.config.endpoint with a real endpoint",
            profile_name
        ));
    }
    if !is_allowed_remote_endpoint(trimmed) {
        return Err(anyhow!(
            "providers.{provider_id}.profile '{}' must use an https:// endpoint; plaintext http:// is only allowed for localhost/loopback test endpoints",
            profile_name
        ));
    }
    Ok(())
}

pub(crate) fn is_allowed_remote_endpoint(endpoint: &str) -> bool {
    let Ok(url) = Url::parse(endpoint.trim()) else {
        return false;
    };
    match url.scheme() {
        "https" => url.host_str().is_some_and(|host| !host.is_empty()),
        "http" => matches!(
            url.host_str(),
            Some("localhost") | Some("127.0.0.1") | Some("[::1]") | Some("::1")
        ),
        _ => false,
    }
}

pub(crate) fn is_allowed_ollama_endpoint(endpoint: &str) -> bool {
    let normalized = endpoint.trim().to_ascii_lowercase();
    normalized.starts_with("https://") || normalized.starts_with("http://")
}

pub(crate) const OPENAI_RESPONSES_ENDPOINT: &str = "https://api.openai.com/v1/responses";
pub(crate) const ANTHROPIC_MESSAGES_ENDPOINT: &str = "https://api.anthropic.com/v1/messages";
pub(crate) const DEEPSEEK_CHAT_COMPLETIONS_ENDPOINT: &str =
    "https://api.deepseek.com/chat/completions";
pub(crate) const OPENROUTER_CHAT_COMPLETIONS_ENDPOINT: &str =
    "https://openrouter.ai/api/v1/chat/completions";
pub(crate) const Z_AI_CHAT_COMPLETIONS_ENDPOINT: &str =
    "https://open.bigmodel.cn/api/paas/v4/chat/completions";
pub(crate) const KIMI_CHAT_COMPLETIONS_ENDPOINT: &str =
    "https://api.moonshot.ai/v1/chat/completions";
pub(crate) const MINIMAX_CHAT_COMPLETIONS_ENDPOINT: &str =
    "https://api.minimax.io/v1/text/chatcompletion_v2";
pub(crate) const QWEN_CHAT_COMPLETIONS_ENDPOINT: &str =
    "https://dashscope.aliyuncs.com/compatible-mode/v1/chat/completions";
pub(crate) const XAI_CHAT_COMPLETIONS_ENDPOINT: &str = "https://api.x.ai/v1/chat/completions";
pub(crate) const MISTRAL_CHAT_COMPLETIONS_ENDPOINT: &str =
    "https://api.mistral.ai/v1/chat/completions";
pub(crate) const COHERE_CHAT_ENDPOINT: &str = "https://api.cohere.com/v2/chat";
/// Canonical Anthropic API version used by the HTTP adapter.
pub(crate) const ANTHROPIC_VERSION: &str = "2023-06-01";

pub(crate) fn provider_profile_registry() -> BTreeMap<&'static str, ProviderProfilePreset> {
    let mut m = BTreeMap::new();
    // Ollama / local presets
    m.insert(
        "ollama:phi4-mini",
        ProviderProfilePreset {
            kind: "ollama",
            default_model: Some("phi4-mini"),
            provider_model_id: None,
            endpoint: None,
        },
    );
    m.insert(
        "ollama:qwen2.5-7b",
        ProviderProfilePreset {
            kind: "ollama",
            default_model: Some("qwen2.5:7b"),
            provider_model_id: None,
            endpoint: None,
        },
    );
    m.insert(
        "ollama:llama3.1-8b",
        ProviderProfilePreset {
            kind: "ollama",
            default_model: Some("llama3.1:8b"),
            provider_model_id: None,
            endpoint: None,
        },
    );
    m.insert(
        "ollama:mistral-7b",
        ProviderProfilePreset {
            kind: "ollama",
            default_model: Some("mistral:7b"),
            provider_model_id: None,
            endpoint: None,
        },
    );
    // Mock/testing preset
    m.insert(
        "mock:echo-v1",
        ProviderProfilePreset {
            kind: "mock",
            default_model: Some("echo-v1"),
            provider_model_id: None,
            endpoint: None,
        },
    );
    // AWS Bedrock hosted presets.
    for (name, stable_ref, provider_model_id) in [
        (
            "bedrock:nova-lite-v1",
            "hosted:adl-bedrock:amazon.nova-lite-v1:0",
            "amazon.nova-lite-v1:0",
        ),
        (
            "bedrock:nova-pro-v1",
            "hosted:adl-bedrock:us.amazon.nova-pro-v1:0",
            "us.amazon.nova-pro-v1:0",
        ),
    ] {
        m.insert(
            name,
            ProviderProfilePreset {
                kind: "bedrock",
                default_model: Some(stable_ref),
                provider_model_id: Some(provider_model_id),
                endpoint: None,
            },
        );
    }
    m.insert(
        "z_ai:glm-5",
        ProviderProfilePreset {
            kind: "z_ai",
            default_model: Some("hosted:adl-z-ai:glm-5"),
            provider_model_id: Some("glm-5"),
            endpoint: Some(Z_AI_CHAT_COMPLETIONS_ENDPOINT),
        },
    );
    // First-class hosted provider identities. These profiles intentionally
    // share the bounded HTTP transport while retaining vendor/model identity.
    for (name, model, endpoint) in [
        ("kimi:k2.5", "kimi-k2.5", KIMI_CHAT_COMPLETIONS_ENDPOINT),
        (
            "minimax:m2.5",
            "MiniMax-M2.5",
            MINIMAX_CHAT_COMPLETIONS_ENDPOINT,
        ),
        (
            "qwen:qwen3-max",
            "qwen3-max",
            QWEN_CHAT_COMPLETIONS_ENDPOINT,
        ),
        ("xai:grok-4.5", "grok-4.5", XAI_CHAT_COMPLETIONS_ENDPOINT),
        (
            "mistral:medium-3.5",
            "mistral-medium-3.5",
            MISTRAL_CHAT_COMPLETIONS_ENDPOINT,
        ),
        (
            "mistral:small-4",
            "mistral-small-4",
            MISTRAL_CHAT_COMPLETIONS_ENDPOINT,
        ),
        (
            "mistral:devstral-2",
            "devstral-2",
            MISTRAL_CHAT_COMPLETIONS_ENDPOINT,
        ),
        (
            "cohere:command-a-plus",
            "command-a-plus",
            COHERE_CHAT_ENDPOINT,
        ),
        (
            "cohere:north-mini-code",
            "north-mini-code",
            COHERE_CHAT_ENDPOINT,
        ),
        (
            "deepseek:v4",
            "deepseek-v4",
            DEEPSEEK_CHAT_COMPLETIONS_ENDPOINT,
        ),
        (
            "z_ai:glm-5-current",
            "glm-5",
            Z_AI_CHAT_COMPLETIONS_ENDPOINT,
        ),
        (
            "gemini:3.1-pro-preview",
            "gemini-3.1-pro-preview",
            "https://generativelanguage.googleapis.com/v1beta/models",
        ),
        (
            "gemini:3.1-flash-lite",
            "gemini-3.1-flash-lite",
            "https://generativelanguage.googleapis.com/v1beta/models",
        ),
    ] {
        m.insert(
            name,
            ProviderProfilePreset {
                kind: "http",
                default_model: Some(model),
                provider_model_id: Some(model),
                endpoint: Some(endpoint),
            },
        );
    }
    // HTTP presets (explicit fixed endpoint placeholders; no secrets)
    for (name, model) in [
        ("http:gpt-4o-mini", "gpt-4o-mini"),
        ("http:gpt-4.1-mini", "gpt-4.1-mini"),
        ("http:claude-3-5-haiku", "claude-3-5-haiku-latest"),
        ("http:claude-3-7-sonnet", "claude-3-7-sonnet-latest"),
        ("http:gemini-2.0-flash", "gemini-2.0-flash"),
        ("http:gemini-2.5-flash", "gemini-2.5-flash"),
        ("http:deepseek-chat", "deepseek-chat"),
        ("http:llama-3.3-70b", "llama-3.3-70b-instruct"),
    ] {
        m.insert(
            name,
            ProviderProfilePreset {
                kind: "http",
                default_model: Some(model),
                provider_model_id: None,
                endpoint: Some(HTTP_PROFILE_PLACEHOLDER_ENDPOINT),
            },
        );
    }
    // ChatGPT-facing presets (same bounded HTTP substrate, distinct profile family)
    for (name, model) in [
        ("chatgpt:gpt-5.4", "gpt-5.4"),
        ("chatgpt:gpt-5.4-mini", "gpt-5.4-mini"),
        ("chatgpt:gpt-5.3-codex", "gpt-5.3-codex"),
        ("chatgpt:gpt-5.2", "gpt-5.2"),
    ] {
        m.insert(
            name,
            ProviderProfilePreset {
                kind: "http",
                default_model: Some(model),
                provider_model_id: None,
                endpoint: Some(HTTP_PROFILE_PLACEHOLDER_ENDPOINT),
            },
        );
    }
    // Claude-facing presets (same bounded HTTP substrate, distinct profile family)
    for (name, model) in [
        ("claude:claude-3-7-sonnet", "claude-3-7-sonnet-latest"),
        ("claude:claude-3-5-haiku", "claude-3-5-haiku-latest"),
    ] {
        m.insert(
            name,
            ProviderProfilePreset {
                kind: "http",
                default_model: Some(model),
                provider_model_id: None,
                endpoint: Some(HTTP_PROFILE_PLACEHOLDER_ENDPOINT),
            },
        );
    }
    m
}

/// Return available profile names for validation and command completions.
pub fn provider_profile_names() -> Vec<String> {
    provider_profile_registry()
        .keys()
        .map(|name| (*name).to_string())
        .collect()
}

/// Expand provider profiles in an ADL document into explicit concrete specs.
///
/// This is a bounded transform: it expands profile-only providers while keeping
/// explicit `kind`/`base_url`/`default_model` usage unchanged.
pub fn expand_provider_profiles(doc: &adl::AdlDoc) -> Result<adl::AdlDoc> {
    let registry = provider_profile_registry();
    let available = provider_profile_names().join(", ");
    let mut expanded = doc.clone();
    let mut provider_ids: Vec<String> = expanded.providers.keys().cloned().collect();
    provider_ids.sort();

    for provider_id in provider_ids {
        let Some(spec) = expanded.providers.get(&provider_id).cloned() else {
            continue;
        };
        let Some(profile_name_raw) = spec.profile.as_deref() else {
            continue;
        };

        if !spec.kind.trim().is_empty() || spec.base_url.is_some() || spec.default_model.is_some() {
            return Err(anyhow!(
                "providers.{provider_id} uses profile and explicit provider identity fields together (remove type/base_url/default_model when profile is set; config remains available for bounded compatibility overrides)"
            ));
        }

        let profile_name = profile_name_raw.trim();
        let Some(preset) = registry.get(profile_name) else {
            return Err(anyhow!(
                "providers.{provider_id}.profile '{}' is unknown (available: {})",
                profile_name,
                available
            ));
        };

        let mut config = spec.config.clone();
        if let (Some(explicit), Some(expected)) = (
            config.get("vendor").and_then(|value| value.as_str()),
            profile_vendor(profile_name),
        ) {
            let normalized = explicit.trim().to_ascii_lowercase();
            if normalized != expected {
                return Err(anyhow!(
                    "providers.{provider_id}.config.vendor '{}' conflicts with profile vendor '{}'",
                    explicit.trim(),
                    expected
                ));
            }
        }

        if let Some(provider_model_id) = preset.provider_model_id {
            config
                .entry("provider_model_id".to_string())
                .or_insert_with(|| Value::String(provider_model_id.to_string()));
        }
        if let Some(endpoint) = preset.endpoint {
            match config.get("endpoint").and_then(|v| v.as_str()) {
                Some(explicit) => validate_profile_endpoint(&provider_id, profile_name, explicit)?,
                None => {
                    validate_profile_endpoint(&provider_id, profile_name, endpoint)?;
                    config.insert("endpoint".to_string(), Value::String(endpoint.to_string()));
                }
            }
        }
        expanded.providers.insert(
            provider_id,
            adl::ProviderSpec {
                id: spec.id.clone(),
                profile: Some(profile_name.to_string()),
                kind: preset.kind.to_string(),
                base_url: None,
                default_model: preset.default_model.map(|m| m.to_string()),
                config,
            },
        );
    }
    Ok(expanded)
}
