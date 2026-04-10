use anyhow::{anyhow, Context, Result};
use serde_json::Value;
use std::collections::{BTreeMap, HashMap};
use std::env;
use std::error::Error as StdError;
use std::fmt;
use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use crate::adl;
use crate::provider_substrate::{self, ProviderInvocationTargetV1};

/// A minimal blocking provider interface for v0.1.
pub trait Provider: Send + Sync {
    fn complete(&self, prompt: &str) -> Result<String>;

    fn complete_stream(&self, prompt: &str, on_chunk: &mut dyn FnMut(&str)) -> Result<String> {
        let out = self.complete(prompt)?;
        on_chunk(&out);
        Ok(out)
    }
}

#[derive(Debug, Clone, Copy)]
enum ProviderErrorKind {
    UnknownKind,
    InvalidConfig,
    Timeout,
    Panic,
    RuntimeRetryable,
    RuntimeNonRetryable,
}

#[derive(Debug)]
struct ProviderError {
    kind: ProviderErrorKind,
    provider: Option<String>,
    message: String,
}

impl ProviderError {
    fn unknown_kind(kind: &str) -> Self {
        Self {
            kind: ProviderErrorKind::UnknownKind,
            provider: None,
            message: format!(
                "provider kind '{kind}' is not supported (supported: ollama, local_ollama, mock, http, http_remote, openai, anthropic). \
Set providers.<id>.type to one of: ollama, local_ollama, mock, http, http_remote, openai, anthropic. The remote provider surfaces are HTTPS-only."
            ),
        }
    }

    fn invalid_config(provider: &str, message: impl Into<String>) -> Self {
        Self {
            kind: ProviderErrorKind::InvalidConfig,
            provider: Some(provider.to_string()),
            message: message.into(),
        }
    }

    fn runtime(provider: &str, message: impl Into<String>) -> Self {
        Self {
            kind: ProviderErrorKind::RuntimeRetryable,
            provider: Some(provider.to_string()),
            message: message.into(),
        }
    }

    fn runtime_non_retryable(provider: &str, message: impl Into<String>) -> Self {
        Self {
            kind: ProviderErrorKind::RuntimeNonRetryable,
            provider: Some(provider.to_string()),
            message: message.into(),
        }
    }

    fn timeout(provider: &str, message: impl Into<String>) -> Self {
        Self {
            kind: ProviderErrorKind::Timeout,
            provider: Some(provider.to_string()),
            message: message.into(),
        }
    }

    fn panic(provider: &str, message: impl Into<String>) -> Self {
        Self {
            kind: ProviderErrorKind::Panic,
            provider: Some(provider.to_string()),
            message: message.into(),
        }
    }
}

impl fmt::Display for ProviderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            ProviderErrorKind::UnknownKind => write!(f, "{}", self.message),
            ProviderErrorKind::InvalidConfig => write!(
                f,
                "provider {} invalid config: {}",
                self.provider.as_deref().unwrap_or("<unknown>"),
                self.message
            ),
            ProviderErrorKind::Timeout => write!(
                f,
                "provider {} timeout: {}",
                self.provider.as_deref().unwrap_or("<unknown>"),
                self.message
            ),
            ProviderErrorKind::Panic => write!(
                f,
                "provider {} panic: {}",
                self.provider.as_deref().unwrap_or("<unknown>"),
                self.message
            ),
            ProviderErrorKind::RuntimeRetryable => write!(
                f,
                "provider {} runtime error (retryable): {}",
                self.provider.as_deref().unwrap_or("<unknown>"),
                self.message
            ),
            ProviderErrorKind::RuntimeNonRetryable => write!(
                f,
                "provider {} runtime error (non-retryable): {}",
                self.provider.as_deref().unwrap_or("<unknown>"),
                self.message
            ),
        }
    }
}

impl StdError for ProviderError {}

fn unknown_kind(kind: &str) -> anyhow::Error {
    ProviderError::unknown_kind(kind).into()
}

fn invalid_config(provider: &str, message: impl Into<String>) -> anyhow::Error {
    ProviderError::invalid_config(provider, message).into()
}

fn runtime_error(provider: &str, message: impl Into<String>) -> anyhow::Error {
    ProviderError::runtime(provider, message).into()
}

fn runtime_error_non_retryable(provider: &str, message: impl Into<String>) -> anyhow::Error {
    ProviderError::runtime_non_retryable(provider, message).into()
}

fn timeout_error(provider: &str, message: impl Into<String>) -> anyhow::Error {
    ProviderError::timeout(provider, message).into()
}

fn panic_error(provider: &str, message: impl Into<String>) -> anyhow::Error {
    ProviderError::panic(provider, message).into()
}

pub fn is_retryable_error(err: &anyhow::Error) -> bool {
    for cause in err.chain() {
        if let Some(p) = cause.downcast_ref::<ProviderError>() {
            return matches!(
                p.kind,
                ProviderErrorKind::RuntimeRetryable | ProviderErrorKind::Timeout
            );
        }
    }
    if let Some(retryable) = crate::remote_exec::retryability(err) {
        return retryable;
    }
    true
}

pub fn stable_failure_kind(err: &anyhow::Error) -> Option<&'static str> {
    for cause in err.chain() {
        if let Some(p) = cause.downcast_ref::<ProviderError>() {
            return Some(match p.kind {
                ProviderErrorKind::Timeout => "timeout",
                ProviderErrorKind::Panic => "panic",
                ProviderErrorKind::UnknownKind | ProviderErrorKind::InvalidConfig => "schema_error",
                ProviderErrorKind::RuntimeRetryable | ProviderErrorKind::RuntimeNonRetryable => {
                    "provider_error"
                }
            });
        }
    }
    None
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ProviderProfilePreset {
    kind: &'static str,
    default_model: Option<&'static str>,
    endpoint: Option<&'static str>,
}

const HTTP_PROFILE_PLACEHOLDER_ENDPOINT: &str = "https://api.example.invalid/v1/complete";
const INVALID_ENDPOINT_HOST_MARKER: &str = "example.invalid";

fn validate_profile_endpoint(provider_id: &str, profile_name: &str, endpoint: &str) -> Result<()> {
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

const OPENAI_RESPONSES_ENDPOINT: &str = "https://api.openai.com/v1/responses";
const ANTHROPIC_MESSAGES_ENDPOINT: &str = "https://api.anthropic.com/v1/messages";
const ANTHROPIC_VERSION: &str = "2023-06-01";

fn provider_profile_registry() -> BTreeMap<&'static str, ProviderProfilePreset> {
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

/// Factory: build a provider implementation from ADL ProviderSpec.
///
/// Expected schema (based on your compiler errors):
/// ProviderSpec { kind, base_url, config }
pub fn build_provider(
    spec: &adl::ProviderSpec,
    model_override: Option<&str>,
) -> Result<Box<dyn Provider>> {
    build_provider_for_id(
        spec.id.as_deref().unwrap_or("<anonymous-provider>"),
        spec,
        model_override,
    )
}

pub fn build_provider_for_id(
    provider_id: &str,
    spec: &adl::ProviderSpec,
    model_override: Option<&str>,
) -> Result<Box<dyn Provider>> {
    match spec.kind.trim() {
        "http" | "http_remote" | "ollama" | "local_ollama" | "mock" | "openai" | "anthropic" => {}
        other => return Err(unknown_kind(other)),
    }

    let target =
        provider_substrate::provider_invocation_target_v1(provider_id, spec, model_override)
            .with_context(|| format!("normalize provider substrate for '{provider_id}'"))?;
    match target.transport {
        provider_substrate::ProviderTransportV1::Http => match target.provider_kind.as_str() {
            "http" | "http_remote" => Ok(Box::new(HttpProvider::from_target(spec, &target)?)),
            "openai" => Ok(Box::new(OpenAiProvider::from_target(spec, &target)?)),
            "anthropic" => Ok(Box::new(AnthropicProvider::from_target(spec, &target)?)),
            other => Err(unknown_kind(other)),
        },
        provider_substrate::ProviderTransportV1::LocalCli
        | provider_substrate::ProviderTransportV1::InProcess => match target.provider_kind.as_str()
        {
            "ollama" | "local_ollama" => Ok(Box::new(OllamaProvider::from_target(spec, &target)?)),
            "mock" => Ok(Box::new(MockProvider::from_target(&target))),
            other => Err(unknown_kind(other)),
        },
    }
}

fn cfg_str<'a>(cfg: &'a HashMap<String, Value>, key: &str) -> Option<&'a str> {
    cfg.get(key).and_then(|v| v.as_str()).map(str::trim)
}

fn auth_env_for(spec: &adl::ProviderSpec, default_env: &str) -> Result<String> {
    let Some(auth_val) = spec.config.get("auth") else {
        return Ok(default_env.to_string());
    };
    let obj = auth_val
        .as_object()
        .ok_or_else(|| invalid_config("native", "config.auth must be an object"))?;
    let auth_type = obj
        .get("type")
        .and_then(|v| v.as_str())
        .ok_or_else(|| invalid_config("native", "config.auth.type is required"))?;
    if auth_type != "bearer" {
        return Err(invalid_config(
            "native",
            format!("config.auth.type must be 'bearer' (got '{auth_type}')"),
        ));
    }
    let env_key = obj
        .get("env")
        .and_then(|v| v.as_str())
        .ok_or_else(|| invalid_config("native", "config.auth.env is required"))?;
    let trimmed = env_key.trim();
    if trimmed.is_empty() {
        return Err(invalid_config(
            "native",
            "config.auth.env must not be empty",
        ));
    }
    Ok(trimmed.to_string())
}

fn vendor_endpoint(
    spec: &adl::ProviderSpec,
    target: &ProviderInvocationTargetV1,
    default_endpoint: &str,
    provider_label: &str,
) -> Result<String> {
    let endpoint = target
        .endpoint
        .clone()
        .or_else(|| target.base_url.clone())
        .unwrap_or_else(|| default_endpoint.to_string());
    if !is_allowed_remote_endpoint(&endpoint) {
        return Err(invalid_config(
            provider_label,
            "endpoint must use https://; plaintext http:// is only allowed for localhost/loopback test endpoints",
        ));
    }
    if let Some(override_endpoint) = cfg_str(&spec.config, "endpoint") {
        if override_endpoint.is_empty() {
            return Err(invalid_config(
                provider_label,
                "config.endpoint must not be empty when provided",
            ));
        }
    }
    Ok(endpoint)
}

fn truncate_provider_body(text: &str) -> &str {
    let trimmed = text.trim();
    if trimmed.len() > 200 {
        &trimmed[..200]
    } else {
        trimmed
    }
}

fn provider_http_json(
    provider_label: &str,
    req: reqwest::blocking::RequestBuilder,
) -> Result<(Value, u16)> {
    let resp = match req.send() {
        Ok(resp) => resp,
        Err(err) => {
            if err.is_timeout() {
                return Err(timeout_error(
                    provider_label,
                    "kind=timeout native provider request timed out",
                ));
            }
            return Err(runtime_error(
                provider_label,
                format!("kind=request_failed native provider request failed: {err}"),
            ));
        }
    };

    if !resp.status().is_success() {
        let status = resp.status();
        let text = resp.text().unwrap_or_default();
        let class = if status.is_client_error() {
            "client_error"
        } else if status.is_server_error() {
            "server_error"
        } else {
            "http_error"
        };
        let msg = format!(
            "kind={class} status={status} body={}",
            truncate_provider_body(&text)
        );
        if status.is_client_error() {
            return Err(runtime_error_non_retryable(provider_label, msg));
        }
        return Err(runtime_error(provider_label, msg));
    }

    let http_status = resp.status().as_u16();
    let json = resp
        .json()
        .context("native provider response was not valid JSON")
        .map_err(|err| runtime_error_non_retryable(provider_label, err.to_string()))?;
    Ok((json, http_status))
}

fn write_native_invocation_record(
    family: &str,
    model: &str,
    prompt: &str,
    output: &str,
    http_status: u16,
) -> Result<()> {
    let Some(path) = env::var_os("ADL_PROVIDER_INVOCATIONS_PATH") else {
        return Ok(());
    };
    let path = PathBuf::from(path);
    let mut payload = if path.is_file() {
        serde_json::from_slice::<Value>(&fs::read(&path).map_err(|err| {
            runtime_error(
                family,
                format!("failed to read provider invocation artifact: {err}"),
            )
        })?)
        .map_err(|err| {
            runtime_error_non_retryable(
                family,
                format!("provider invocation artifact is invalid JSON: {err}"),
            )
        })?
    } else {
        serde_json::json!({
            "schema_version": "adl.native_provider_invocations.v1",
            "credential_policy": "operator_env_only_no_secret_material_recorded",
            "invocations": []
        })
    };

    let Some(invocations) = payload
        .get_mut("invocations")
        .and_then(|v| v.as_array_mut())
    else {
        return Err(runtime_error_non_retryable(
            family,
            "provider invocation artifact missing invocations array",
        ));
    };
    let timestamp_unix_ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64;
    invocations.push(serde_json::json!({
        "family": family,
        "model": model,
        "http_status": http_status,
        "timestamp_unix_ms": timestamp_unix_ms,
        "prompt_chars": prompt.chars().count(),
        "output_chars": output.chars().count()
    }));

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|err| {
            runtime_error(
                family,
                format!("failed to create provider invocation artifact directory: {err}"),
            )
        })?;
    }
    let bytes = serde_json::to_vec_pretty(&payload).map_err(|err| {
        runtime_error_non_retryable(
            family,
            format!("failed to serialize provider invocation artifact: {err}"),
        )
    })?;
    write_file_atomic(&path, &bytes).map_err(|err| {
        runtime_error(
            family,
            format!("failed to write invocation artifact: {err}"),
        )
    })
}

fn write_file_atomic(path: &Path, bytes: &[u8]) -> std::io::Result<()> {
    let tmp = path.with_extension("tmp");
    fs::write(&tmp, bytes)?;
    fs::rename(tmp, path)
}

fn extract_openai_output_text(json: &Value) -> Option<String> {
    if let Some(text) = json.get("output_text").and_then(|v| v.as_str()) {
        let trimmed = text.trim();
        if !trimmed.is_empty() {
            return Some(trimmed.to_string());
        }
    }

    let mut chunks = Vec::new();
    for item in json.get("output")?.as_array()? {
        for content in item.get("content").and_then(|v| v.as_array())? {
            if let Some(text) = content.get("text").and_then(|v| v.as_str()) {
                chunks.push(text);
            }
        }
    }
    let joined = chunks.join("\n").trim().to_string();
    (!joined.is_empty()).then_some(joined)
}

fn extract_anthropic_output_text(json: &Value) -> Option<String> {
    let mut chunks = Vec::new();
    for content in json.get("content")?.as_array()? {
        let content_type = content.get("type").and_then(|v| v.as_str());
        if content_type == Some("text") {
            if let Some(text) = content.get("text").and_then(|v| v.as_str()) {
                chunks.push(text);
            }
        }
    }
    let joined = chunks.join("\n").trim().to_string();
    (!joined.is_empty()).then_some(joined)
}

#[derive(Debug, Clone)]
pub struct OpenAiProvider {
    endpoint: String,
    auth_env: String,
    model: String,
    max_output_tokens: u64,
    timeout_secs: Option<u64>,
}

impl OpenAiProvider {
    pub fn from_target(
        spec: &adl::ProviderSpec,
        target: &ProviderInvocationTargetV1,
    ) -> Result<Self> {
        Ok(Self {
            endpoint: vendor_endpoint(spec, target, OPENAI_RESPONSES_ENDPOINT, "openai")?,
            auth_env: auth_env_for(spec, "OPENAI_API_KEY")?,
            model: target.provider_model_id.clone(),
            max_output_tokens: cfg_u64(&spec.config, "max_output_tokens").unwrap_or(220),
            timeout_secs: cfg_u64(&spec.config, "timeout_secs"),
        })
    }
}

impl Provider for OpenAiProvider {
    fn complete(&self, prompt: &str) -> Result<String> {
        let token = env::var(&self.auth_env).map_err(|_| {
            invalid_config(
                "openai",
                format!("missing required auth env var '{}'", self.auth_env),
            )
        })?;
        let mut client_builder = reqwest::blocking::Client::builder();
        if let Some(secs) = self.timeout_secs {
            client_builder = client_builder.timeout(Duration::from_secs(secs));
        }
        let client = client_builder
            .build()
            .context("failed to build OpenAI client")
            .map_err(|err| runtime_error("openai", err.to_string()))?;
        let req = client
            .post(&self.endpoint)
            .header("Content-Type", "application/json")
            .bearer_auth(token)
            .json(&serde_json::json!({
                "model": self.model,
                "input": prompt,
                "max_output_tokens": self.max_output_tokens,
            }));
        let (json, http_status) = provider_http_json("openai", req)?;
        let output = extract_openai_output_text(&json)
            .ok_or_else(|| runtime_error_non_retryable("openai", "response missing text output"))?;
        write_native_invocation_record("openai", &self.model, prompt, &output, http_status)?;
        Ok(output)
    }
}

#[derive(Debug, Clone)]
pub struct AnthropicProvider {
    endpoint: String,
    auth_env: String,
    model: String,
    max_tokens: u64,
    timeout_secs: Option<u64>,
}

impl AnthropicProvider {
    pub fn from_target(
        spec: &adl::ProviderSpec,
        target: &ProviderInvocationTargetV1,
    ) -> Result<Self> {
        Ok(Self {
            endpoint: vendor_endpoint(spec, target, ANTHROPIC_MESSAGES_ENDPOINT, "anthropic")?,
            auth_env: auth_env_for(spec, "ANTHROPIC_API_KEY")?,
            model: target.provider_model_id.clone(),
            max_tokens: cfg_u64(&spec.config, "max_tokens")
                .or_else(|| cfg_u64(&spec.config, "max_output_tokens"))
                .unwrap_or(220),
            timeout_secs: cfg_u64(&spec.config, "timeout_secs"),
        })
    }
}

impl Provider for AnthropicProvider {
    fn complete(&self, prompt: &str) -> Result<String> {
        let token = env::var(&self.auth_env).map_err(|_| {
            invalid_config(
                "anthropic",
                format!("missing required auth env var '{}'", self.auth_env),
            )
        })?;
        let mut client_builder = reqwest::blocking::Client::builder();
        if let Some(secs) = self.timeout_secs {
            client_builder = client_builder.timeout(Duration::from_secs(secs));
        }
        let client = client_builder
            .build()
            .context("failed to build Anthropic client")
            .map_err(|err| runtime_error("anthropic", err.to_string()))?;
        let req = client
            .post(&self.endpoint)
            .header("Content-Type", "application/json")
            .header("x-api-key", token)
            .header("anthropic-version", ANTHROPIC_VERSION)
            .json(&serde_json::json!({
                "model": self.model,
                "max_tokens": self.max_tokens,
                "messages": [{"role": "user", "content": prompt}],
            }));
        let (json, http_status) = provider_http_json("anthropic", req)?;
        let output = extract_anthropic_output_text(&json).ok_or_else(|| {
            runtime_error_non_retryable("anthropic", "response missing text output")
        })?;
        write_native_invocation_record("anthropic", &self.model, prompt, &output, http_status)?;
        Ok(output)
    }
}

#[derive(Debug, Clone)]
pub struct MockProvider {
    model: String,
}

impl MockProvider {
    pub fn from_target(target: &ProviderInvocationTargetV1) -> Self {
        Self {
            model: target.model_ref.clone(),
        }
    }
}

impl Provider for MockProvider {
    fn complete(&self, prompt: &str) -> Result<String> {
        let _model = &self.model;
        Ok(prompt.to_string())
    }
}

/// Ollama provider (blocking) using the local `ollama` CLI.
/// This keeps v0.1 dependency-light and works well for local prototyping.
#[derive(Debug, Clone)]
pub struct OllamaProvider {
    pub model: String,
    pub temperature: Option<f32>,
}

impl OllamaProvider {
    pub fn from_spec(spec: &adl::ProviderSpec, model_override: Option<&str>) -> Result<Self> {
        let target = provider_substrate::provider_invocation_target_v1(
            spec.id.as_deref().unwrap_or("<anonymous-provider>"),
            spec,
            model_override,
        )?;
        Self::from_target(spec, &target)
    }

    pub fn from_target(
        spec: &adl::ProviderSpec,
        target: &ProviderInvocationTargetV1,
    ) -> Result<Self> {
        let temperature = cfg_f32(&spec.config, "temperature");

        Ok(Self {
            // Local CLI execution has no separate provider-native model identifier surface,
            // so the stable model_ref is the runtime model we should actually invoke.
            model: target.model_ref.clone(),
            temperature,
        })
    }

    fn complete_streaming(
        &self,
        prompt: &str,
        mut on_chunk: Option<&mut dyn FnMut(&str)>,
    ) -> Result<String> {
        let timeout_secs =
            timeout_secs().map_err(|err| invalid_config("ollama", err.to_string()))?;

        // v0.1: We parse `temperature` from provider config for forward-compatibility,
        // but the `ollama` CLI does not consistently expose a stable flag across versions.
        // Read the field so it does not trip `-D dead-code`, and keep behavior deterministic.
        let _temperature = self.temperature;
        let mut child = Command::new(ollama_bin())
            .arg("run")
            .arg(&self.model)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .with_context(|| "failed to spawn `ollama run` (is Ollama installed and on PATH?)")
            .map_err(|err| runtime_error("ollama", err.to_string()))?;

        let stdout = child
            .stdout
            .take()
            .context("failed to open stdout for ollama")
            .map_err(|err| runtime_error("ollama", err.to_string()))?;
        let stderr = child
            .stderr
            .take()
            .context("failed to open stderr for ollama")
            .map_err(|err| runtime_error("ollama", err.to_string()))?;

        let (tx, rx) = mpsc::channel::<Vec<u8>>();
        let out_handle = thread::spawn(move || -> std::io::Result<()> {
            let mut r = stdout;
            let mut buf = [0u8; 4096];
            loop {
                let n = r.read(&mut buf)?;
                if n == 0 {
                    break;
                }
                if tx.send(buf[..n].to_vec()).is_err() {
                    break;
                }
            }
            Ok(())
        });

        let err_handle = thread::spawn(move || -> std::io::Result<Vec<u8>> {
            let mut r = stderr;
            let mut buf = Vec::new();
            r.read_to_end(&mut buf)?;
            Ok(buf)
        });

        {
            let mut stdin = child
                .stdin
                .take()
                .context("failed to open stdin for ollama")
                .map_err(|err| runtime_error("ollama", err.to_string()))?;
            stdin
                .write_all(prompt.as_bytes())
                .context("failed writing prompt to ollama stdin")
                .map_err(|err| runtime_error("ollama", err.to_string()))?;
            drop(stdin);
        }

        let start = Instant::now();
        let timeout = Duration::from_secs(timeout_secs);
        let mut out_buf = Vec::new();

        let status = loop {
            while let Ok(chunk) = rx.try_recv() {
                out_buf.extend_from_slice(&chunk);
                if let Some(cb) = on_chunk.as_deref_mut() {
                    cb(&String::from_utf8_lossy(&chunk));
                }
            }

            if let Some(status) = child
                .try_wait()
                .context("failed waiting for ollama process")
                .map_err(|err| runtime_error("ollama", err.to_string()))?
            {
                break status;
            }

            if start.elapsed() >= timeout {
                let _ = child.kill();
                let kill_start = Instant::now();
                loop {
                    if let Some(_status) = child
                        .try_wait()
                        .context("failed waiting for ollama process")
                        .map_err(|err| runtime_error("ollama", err.to_string()))?
                    {
                        break;
                    }
                    if kill_start.elapsed() >= Duration::from_secs(1) {
                        break;
                    }
                    std::thread::sleep(Duration::from_millis(10));
                }
                return Err(timeout_error(
                    "ollama",
                    format!("timed out after {timeout_secs}s (set ADL_TIMEOUT_SECS to override)"),
                ));
            }

            std::thread::sleep(Duration::from_millis(10));
        };

        while let Ok(chunk) = rx.try_recv() {
            out_buf.extend_from_slice(&chunk);
            if let Some(cb) = on_chunk.as_deref_mut() {
                cb(&String::from_utf8_lossy(&chunk));
            }
        }

        out_handle
            .join()
            .map_err(|_| panic_error("ollama", "stdout reader thread panicked"))?
            .context("failed reading ollama stdout")
            .map_err(|err| runtime_error("ollama", err.to_string()))?;
        let err_buf = err_handle
            .join()
            .map_err(|_| panic_error("ollama", "stderr reader thread panicked"))?
            .context("failed reading ollama stderr")
            .map_err(|err| runtime_error("ollama", err.to_string()))?;

        if !status.success() {
            let stderr = String::from_utf8_lossy(&err_buf);
            return Err(runtime_error(
                "ollama",
                format!(
                    "ollama run failed (exit={:?}): {}",
                    status.code(),
                    stderr.trim()
                ),
            ));
        }

        let stdout = String::from_utf8(out_buf)
            .context("ollama output was not valid UTF-8")
            .map_err(|err| runtime_error("ollama", err.to_string()))?;
        Ok(stdout)
    }
}

impl Provider for OllamaProvider {
    fn complete(&self, prompt: &str) -> Result<String> {
        self.complete_streaming(prompt, None)
    }

    fn complete_stream(&self, prompt: &str, on_chunk: &mut dyn FnMut(&str)) -> Result<String> {
        self.complete_streaming(prompt, Some(on_chunk))
    }
}

fn ollama_bin() -> PathBuf {
    // Allows tests (and power users) to override the binary path.
    // Defaults to `ollama` on PATH.
    std::env::var_os("ADL_OLLAMA_BIN")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("ollama"))
}

fn cfg_f32(cfg: &HashMap<String, Value>, key: &str) -> Option<f32> {
    cfg.get(key).and_then(|v| {
        if let Some(f) = v.as_f64() {
            Some(f as f32)
        } else if let Some(i) = v.as_i64() {
            Some(i as f32)
        } else if let Some(s) = v.as_str() {
            s.parse::<f32>().ok()
        } else {
            None
        }
    })
}

#[derive(Debug, Clone)]
pub struct HttpProvider {
    endpoint: String,
    auth: Option<HttpAuth>,
    headers: HashMap<String, String>,
    timeout_secs: Option<u64>,
}

#[derive(Debug, Clone)]
struct HttpAuth {
    env: String,
}

impl HttpProvider {
    pub fn from_spec(spec: &adl::ProviderSpec) -> Result<Self> {
        let target = provider_substrate::provider_invocation_target_v1(
            spec.id.as_deref().unwrap_or("<anonymous-provider>"),
            spec,
            None,
        )?;
        Self::from_target(spec, &target)
    }

    pub fn from_target(
        spec: &adl::ProviderSpec,
        target: &ProviderInvocationTargetV1,
    ) -> Result<Self> {
        let cfg = &spec.config;
        let endpoint = target
            .endpoint
            .clone()
            .or_else(|| target.base_url.clone())
            .ok_or_else(|| {
                invalid_config(
                    "http",
                    "config.endpoint is required (set providers.<id>.config.endpoint)",
                )
            })?;
        if !is_allowed_remote_endpoint(&endpoint) {
            return Err(invalid_config(
                "http",
                "config.endpoint must use https://; plaintext http:// is only allowed for localhost/loopback test endpoints",
            ));
        }

        let timeout_secs = cfg_u64(cfg, "timeout_secs");

        let mut headers = HashMap::new();
        if let Some(h) = cfg.get("headers") {
            let obj = h.as_object().ok_or_else(|| {
                invalid_config("http", "config.headers must be an object of string values")
            })?;
            for (k, v) in obj {
                let v = v.as_str().ok_or_else(|| {
                    invalid_config("http", "config.headers values must be strings")
                })?;
                headers.insert(k.clone(), v.to_string());
            }
        }

        let auth = if let Some(auth_val) = cfg.get("auth") {
            let obj = auth_val
                .as_object()
                .ok_or_else(|| invalid_config("http", "config.auth must be an object"))?;
            let auth_type = obj
                .get("type")
                .and_then(|v| v.as_str())
                .ok_or_else(|| invalid_config("http", "config.auth.type is required"))?;
            if auth_type != "bearer" {
                return Err(invalid_config(
                    "http",
                    format!("config.auth.type must be 'bearer' (got '{auth_type}')"),
                ));
            }
            let env_key = obj
                .get("env")
                .and_then(|v| v.as_str())
                .ok_or_else(|| invalid_config("http", "config.auth.env is required"))?;
            Some(HttpAuth {
                env: env_key.to_string(),
            })
        } else {
            None
        };

        Ok(Self {
            endpoint,
            auth,
            headers,
            timeout_secs,
        })
    }
}

impl Provider for HttpProvider {
    fn complete(&self, prompt: &str) -> Result<String> {
        let mut client_builder = reqwest::blocking::Client::builder();
        if let Some(secs) = self.timeout_secs {
            client_builder = client_builder.timeout(Duration::from_secs(secs));
        }
        let client = client_builder
            .build()
            .context("failed to build http client")
            .map_err(|err| runtime_error("http", err.to_string()))?;

        let mut req = client
            .post(&self.endpoint)
            .header("Content-Type", "application/json");

        for (k, v) in self.headers.iter() {
            req = req.header(k, v);
        }

        if let Some(auth) = &self.auth {
            let token = env::var(&auth.env).map_err(|_| {
                invalid_config(
                    "http",
                    format!(
                        "missing required auth env var '{}' (set it or remove config.auth)",
                        auth.env
                    ),
                )
            })?;
            req = req.bearer_auth(token);
        }

        let body = serde_json::json!({ "prompt": prompt });

        let resp = match req.json(&body).send() {
            Ok(resp) => resp,
            Err(err) => {
                if err.is_timeout() {
                    let msg = match self.timeout_secs {
                        Some(secs) => format!(
                            "kind=timeout timed out after {secs}s (set providers.<id>.config.timeout_secs or ADL_TIMEOUT_SECS to override)"
                        ),
                        None => {
                            "kind=timeout timed out (set providers.<id>.config.timeout_secs or ADL_TIMEOUT_SECS to override)"
                                .to_string()
                        }
                    };
                    return Err(timeout_error("http", msg));
                }

                return Err(runtime_error(
                    "http",
                    format!("kind=request_failed http provider request failed: {err}"),
                ));
            }
        };

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().unwrap_or_default();
            let trimmed = text.trim();
            let trimmed = if trimmed.len() > 200 {
                &trimmed[..200]
            } else {
                trimmed
            };
            let class = if status.is_client_error() {
                "client_error"
            } else if status.is_server_error() {
                "server_error"
            } else {
                "http_error"
            };
            let msg = format!("kind={class} status={status} body={trimmed}");
            if status.is_client_error() {
                return Err(runtime_error_non_retryable("http", msg));
            }
            return Err(runtime_error("http", msg));
        }

        let json: serde_json::Value = resp
            .json()
            .context("http provider response was not valid JSON")
            .map_err(|err| runtime_error_non_retryable("http", err.to_string()))?;
        let out = json.get("output").and_then(|v| v.as_str()).ok_or_else(|| {
            runtime_error_non_retryable("http", "response missing 'output' field")
        })?;

        Ok(out.to_string())
    }
}

fn timeout_secs() -> Result<u64> {
    let raw = std::env::var("ADL_TIMEOUT_SECS").ok();
    let secs = match raw {
        None => 120_u64,
        Some(v) => {
            let parsed: u64 = v.parse().map_err(|_| {
                anyhow!("invalid ADL_TIMEOUT_SECS: '{v}' (must be a positive integer)")
            })?;
            if parsed == 0 {
                return Err(anyhow!(
                    "invalid ADL_TIMEOUT_SECS: '{v}' (must be a positive integer)"
                ));
            }
            parsed
        }
    };
    Ok(secs)
}

fn cfg_u64(cfg: &HashMap<String, Value>, key: &str) -> Option<u64> {
    cfg.get(key).and_then(|v| {
        if let Some(u) = v.as_u64() {
            Some(u)
        } else if let Some(i) = v.as_i64() {
            if i >= 0 {
                Some(i as u64)
            } else {
                None
            }
        } else if let Some(s) = v.as_str() {
            s.parse::<u64>().ok()
        } else {
            None
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn provider_error_helpers_and_classification_are_stable() {
        let retryable = runtime_error("mock", "retryable");
        assert!(is_retryable_error(&retryable));
        assert_eq!(stable_failure_kind(&retryable), Some("provider_error"));

        let non_retryable = runtime_error_non_retryable("mock", "non-retryable");
        assert!(!is_retryable_error(&non_retryable));
        assert_eq!(stable_failure_kind(&non_retryable), Some("provider_error"));

        let timeout = timeout_error("mock", "timeout");
        assert!(is_retryable_error(&timeout));
        assert_eq!(stable_failure_kind(&timeout), Some("timeout"));

        let panic = panic_error("mock", "panic");
        assert!(!is_retryable_error(&panic));
        assert_eq!(stable_failure_kind(&panic), Some("panic"));
        assert!(format!("{panic:#}").contains("provider mock panic: panic"));
    }

    #[test]
    fn remote_retry_classification_distinguishes_deterministic_failures() {
        let schema = anyhow::Error::new(crate::remote_exec::RemoteExecuteClientError::new(
            crate::remote_exec::RemoteExecuteClientErrorKind::SchemaViolation,
            "REMOTE_SCHEMA_VIOLATION",
            "missing result on ok response",
        ));
        assert!(!is_retryable_error(&schema));

        let envelope = anyhow::Error::new(crate::remote_exec::SecurityEnvelopeError::MissingKeyId);
        assert!(!is_retryable_error(&envelope));

        let remote_schema = anyhow::Error::new(crate::remote_exec::RemoteExecuteClientError::new(
            crate::remote_exec::RemoteExecuteClientErrorKind::RemoteExecution,
            "REMOTE_SCHEMA_VIOLATION",
            "invalid provider config",
        ));
        assert!(!is_retryable_error(&remote_schema));

        let timeout = anyhow::Error::new(crate::remote_exec::RemoteExecuteClientError::new(
            crate::remote_exec::RemoteExecuteClientErrorKind::Timeout,
            "REMOTE_TIMEOUT",
            "timed out",
        ));
        assert!(is_retryable_error(&timeout));
    }

    #[test]
    fn profile_endpoint_validation_rejects_placeholder_and_invalid_hosts() {
        let empty =
            validate_profile_endpoint("p1", "http:gpt-4o-mini", " ").expect_err("empty endpoint");
        assert!(empty
            .to_string()
            .contains("placeholder or invalid endpoint"));

        let invalid_host = validate_profile_endpoint(
            "p1",
            "http:gpt-4o-mini",
            "https://api.example.invalid/v1/complete",
        )
        .expect_err("placeholder host should fail");
        assert!(invalid_host
            .to_string()
            .contains("configure providers.p1.config.endpoint"));

        validate_profile_endpoint("p1", "custom", "https://api.openai.com/v1/complete")
            .expect("real endpoint should pass");
    }

    #[test]
    fn profile_endpoint_validation_rejects_plain_http() {
        let err = validate_profile_endpoint(
            "p1",
            "http:gpt-4o-mini",
            "http://api.example.com/v1/complete",
        )
        .expect_err("plain http should fail");
        assert!(err.to_string().contains("must use an https:// endpoint"));
    }

    #[test]
    fn profile_endpoint_validation_allows_loopback_http_for_local_harnesses() {
        validate_profile_endpoint("p1", "http:gpt-4o-mini", "http://127.0.0.1:8787/complete")
            .expect("loopback http should remain allowed");
    }

    #[test]
    fn provider_profile_registry_includes_first_class_claude_profiles() {
        let names = provider_profile_names();
        assert!(names.contains(&"claude:claude-3-7-sonnet".to_string()));
        assert!(names.contains(&"claude:claude-3-5-haiku".to_string()));

        let preset = provider_profile_registry()
            .get("claude:claude-3-7-sonnet")
            .copied()
            .expect("claude sonnet preset");
        assert_eq!(preset.kind, "http");
        assert_eq!(preset.default_model, Some("claude-3-7-sonnet-latest"));
    }

    #[test]
    fn cfg_numeric_helpers_cover_all_supported_and_rejected_types() {
        let mut cfg = HashMap::new();
        cfg.insert("f64".to_string(), serde_json::json!(0.5));
        cfg.insert("i64".to_string(), serde_json::json!(2));
        cfg.insert("str".to_string(), serde_json::json!("3.25"));
        cfg.insert("bad_str".to_string(), serde_json::json!("not-a-number"));
        cfg.insert("bool".to_string(), serde_json::json!(true));
        cfg.insert("u64".to_string(), serde_json::json!(7));
        cfg.insert("neg_i64".to_string(), serde_json::json!(-1));

        assert_eq!(cfg_f32(&cfg, "f64"), Some(0.5_f32));
        assert_eq!(cfg_f32(&cfg, "i64"), Some(2.0_f32));
        assert_eq!(cfg_f32(&cfg, "str"), Some(3.25_f32));
        assert_eq!(cfg_f32(&cfg, "bad_str"), None);
        assert_eq!(cfg_f32(&cfg, "bool"), None);
        assert_eq!(cfg_f32(&cfg, "missing"), None);

        assert_eq!(cfg_u64(&cfg, "u64"), Some(7_u64));
        assert_eq!(cfg_u64(&cfg, "i64"), Some(2_u64));
        assert_eq!(cfg_u64(&cfg, "str"), None);
        assert_eq!(cfg_u64(&cfg, "neg_i64"), None);
        assert_eq!(cfg_u64(&cfg, "bad_str"), None);
        assert_eq!(cfg_u64(&cfg, "bool"), None);
    }

    #[test]
    fn timeout_secs_rejects_zero_and_uses_default_without_env() {
        let prev_adl = env::var_os("ADL_TIMEOUT_SECS");

        env::set_var("ADL_TIMEOUT_SECS", "0");
        let err = timeout_secs().expect_err("zero timeout env should fail");
        assert!(err.to_string().contains("invalid ADL_TIMEOUT_SECS"));

        env::remove_var("ADL_TIMEOUT_SECS");
        assert_eq!(timeout_secs().expect("default timeout"), 120);

        match prev_adl {
            Some(v) => env::set_var("ADL_TIMEOUT_SECS", v),
            None => env::remove_var("ADL_TIMEOUT_SECS"),
        }
    }
}
