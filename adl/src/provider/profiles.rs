use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ProviderProfilePreset {
    pub(crate) kind: &'static str,
    pub(crate) default_model: Option<&'static str>,
    pub(crate) endpoint: Option<&'static str>,
}

const HTTP_PROFILE_PLACEHOLDER_ENDPOINT: &str = "https://api.example.invalid/v1/complete";
const INVALID_ENDPOINT_HOST_MARKER: &str = "example.invalid";

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
    let normalized = endpoint.trim().to_ascii_lowercase();
    normalized.starts_with("https://")
        || normalized.starts_with("http://localhost")
        || normalized.starts_with("http://127.0.0.1")
        || normalized.starts_with("http://[::1]")
}

pub(crate) fn is_allowed_ollama_endpoint(endpoint: &str) -> bool {
    let normalized = endpoint.trim().to_ascii_lowercase();
    normalized.starts_with("https://") || normalized.starts_with("http://")
}

pub(crate) const OPENAI_RESPONSES_ENDPOINT: &str = "https://api.openai.com/v1/responses";
pub(crate) const ANTHROPIC_MESSAGES_ENDPOINT: &str = "https://api.anthropic.com/v1/messages";
pub(crate) const ANTHROPIC_VERSION: &str = "2023-06-01";

pub(crate) fn provider_profile_registry() -> BTreeMap<&'static str, ProviderProfilePreset> {
    let mut m = BTreeMap::new();
    // Ollama / local presets
    m.insert(
        "ollama:phi4-mini",
        ProviderProfilePreset {
            kind: "ollama",
            default_model: Some("phi4-mini"),
            endpoint: None,
        },
    );
    m.insert(
        "ollama:qwen2.5-7b",
        ProviderProfilePreset {
            kind: "ollama",
            default_model: Some("qwen2.5:7b"),
            endpoint: None,
        },
    );
    m.insert(
        "ollama:llama3.1-8b",
        ProviderProfilePreset {
            kind: "ollama",
            default_model: Some("llama3.1:8b"),
            endpoint: None,
        },
    );
    m.insert(
        "ollama:mistral-7b",
        ProviderProfilePreset {
            kind: "ollama",
            default_model: Some("mistral:7b"),
            endpoint: None,
        },
    );
    // Mock/testing preset
    m.insert(
        "mock:echo-v1",
        ProviderProfilePreset {
            kind: "mock",
            default_model: Some("echo-v1"),
            endpoint: None,
        },
    );
    // HTTP presets (explicit fixed endpoint placeholders; no secrets)
    for (name, model) in [
        ("http:gpt-4o-mini", "gpt-4o-mini"),
        ("http:gpt-4.1-mini", "gpt-4.1-mini"),
        ("http:claude-3-5-haiku", "claude-3-5-haiku-latest"),
        ("http:claude-3-7-sonnet", "claude-3-7-sonnet-latest"),
        ("http:gemini-2.0-flash", "gemini-2.0-flash"),
        ("http:deepseek-chat", "deepseek-chat"),
        ("http:llama-3.3-70b", "llama-3.3-70b-instruct"),
    ] {
        m.insert(
            name,
            ProviderProfilePreset {
                kind: "http",
                default_model: Some(model),
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
                endpoint: Some(HTTP_PROFILE_PLACEHOLDER_ENDPOINT),
            },
        );
    }
    m
}

pub fn provider_profile_names() -> Vec<String> {
    provider_profile_registry()
        .keys()
        .map(|name| (*name).to_string())
        .collect()
}

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
