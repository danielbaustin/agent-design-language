//! HTTP-based provider implementations and request transport helpers.
//!
//! Supports OpenAI, Anthropic, DeepSeek, OpenRouter, Z.ai, generic HTTP, and Ollama-HTTP style backends.
use super::*;
use aws_config::{meta::region::RegionProviderChain, BehaviorVersion};
use aws_sdk_bedrockruntime as bedrockruntime;
use aws_sdk_sts as sts;
use fs2::FileExt;
use sha2::{Digest, Sha256};
use std::fs::{File, OpenOptions};
use std::thread;
use std::time::Duration;

mod config;

use config::{
    auth_env_for, cfg_u64_strict, ollama_generate_endpoint, validate_http_credential_endpoint,
    validate_vendor_credential_endpoint, vendor_endpoint, HttpAuth,
};
pub(crate) use config::{cfg_u64, timeout_secs};

struct InvocationArtifactLock {
    _file: File,
}

const INVOCATION_ARTIFACT_LOCK_TIMEOUT: Duration = Duration::from_secs(2);
const INVOCATION_ARTIFACT_LOCK_TIMEOUT_ENV: &str = "ADL_INVOCATION_LOCK_TIMEOUT_MS";

fn invocation_lock_path(path: &Path) -> PathBuf {
    let mut os = path.as_os_str().to_os_string();
    os.push(".lock");
    PathBuf::from(os)
}

fn acquire_invocation_artifact_lock(path: &Path) -> std::io::Result<InvocationArtifactLock> {
    let lock_path = invocation_lock_path(path);
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(lock_path)?;
    let started = Instant::now();
    let timeout = invocation_artifact_lock_timeout();
    loop {
        match file.try_lock_exclusive() {
            Ok(()) => return Ok(InvocationArtifactLock { _file: file }),
            Err(err) if err.kind() == std::io::ErrorKind::WouldBlock => {
                if started.elapsed() > timeout {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::TimedOut,
                        "timed out waiting for invocation artifact lock",
                    ));
                }
                thread::sleep(Duration::from_millis(10));
            }
            Err(err) => return Err(err),
        }
    }
}

fn invocation_artifact_lock_timeout() -> Duration {
    env::var(INVOCATION_ARTIFACT_LOCK_TIMEOUT_ENV)
        .ok()
        .and_then(|raw| raw.parse::<u64>().ok())
        .map(Duration::from_millis)
        .unwrap_or(INVOCATION_ARTIFACT_LOCK_TIMEOUT)
}

/// Maximum number of provider error-body characters kept for inline request-failure messages.
const MAX_PROVIDER_ERROR_BODY_BYTES: usize = 200;

fn truncate_provider_body(text: &str) -> String {
    let trimmed = text.trim();
    if trimmed.len() <= MAX_PROVIDER_ERROR_BODY_BYTES {
        return trimmed.to_string();
    }

    let end = trimmed
        .char_indices()
        .map(|(idx, _)| idx)
        .chain(std::iter::once(trimmed.len()))
        .take_while(|idx| *idx <= MAX_PROVIDER_ERROR_BODY_BYTES)
        .last()
        .unwrap_or(0);
    trimmed[..end].to_string()
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
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|err| {
            post_success_invocation_artifact_io_error(
                family,
                format!("failed to create provider invocation artifact directory: {err}"),
            )
        })?;
    }
    let _artifact_lock = acquire_invocation_artifact_lock(&path).map_err(|err| {
        runtime_error_non_retryable(
            family,
            format!("partial_success_unknown_invocation_record_lock_unavailable: provider call completed but invocation artifact lock could not be acquired without risking duplicate retry: {err}"),
        )
    })?;
    let mut payload = if path.is_file() {
        serde_json::from_slice::<Value>(&fs::read(&path).map_err(|err| {
            post_success_invocation_artifact_io_error(
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
    let bytes = serde_json::to_vec_pretty(&payload).map_err(|err| {
        runtime_error_non_retryable(
            family,
            format!("failed to serialize provider invocation artifact: {err}"),
        )
    })?;
    write_file_atomic(&path, &bytes).map_err(|err| {
        post_success_invocation_artifact_io_error(
            family,
            format!("failed to write invocation artifact: {err}"),
        )
    })
}

struct BedrockInvocationRecord<'a> {
    model: &'a str,
    prompt: &'a str,
    output: &'a str,
    http_status: u16,
    profile: &'a str,
    region: &'a str,
    account_id_sha256: Option<&'a str>,
    account_profile_validation_status: &'a str,
}

fn write_bedrock_invocation_record(record: BedrockInvocationRecord<'_>) -> Result<()> {
    let Some(path) = env::var_os("ADL_PROVIDER_INVOCATIONS_PATH") else {
        return Ok(());
    };
    let path = PathBuf::from(path);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|err| {
            post_success_invocation_artifact_io_error(
                "bedrock",
                format!("failed to create provider invocation artifact directory: {err}"),
            )
        })?;
    }
    let _artifact_lock = acquire_invocation_artifact_lock(&path).map_err(|err| {
        runtime_error_non_retryable(
            "bedrock",
            format!("partial_success_unknown_invocation_record_lock_unavailable: Bedrock call completed but invocation artifact lock could not be acquired without risking duplicate retry: {err}"),
        )
    })?;
    let mut payload = if path.is_file() {
        serde_json::from_slice::<Value>(&fs::read(&path).map_err(|err| {
            post_success_invocation_artifact_io_error(
                "bedrock",
                format!("failed to read provider invocation artifact: {err}"),
            )
        })?)
        .map_err(|err| {
            runtime_error_non_retryable(
                "bedrock",
                format!("provider invocation artifact is invalid JSON: {err}"),
            )
        })?
    } else {
        serde_json::json!({
            "schema_version": "adl.native_provider_invocations.v1",
            "credential_policy": "operator_env_or_aws_profile_only_no_secret_material_recorded",
            "invocations": []
        })
    };

    let Some(invocations) = payload
        .get_mut("invocations")
        .and_then(|v| v.as_array_mut())
    else {
        return Err(runtime_error_non_retryable(
            "bedrock",
            "provider invocation artifact missing invocations array",
        ));
    };
    let timestamp_unix_ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64;
    invocations.push(serde_json::json!({
        "family": "bedrock",
        "model": record.model,
        "http_status": record.http_status,
        "timestamp_unix_ms": timestamp_unix_ms,
        "prompt_chars": record.prompt.chars().count(),
        "output_chars": record.output.chars().count(),
        "aws_profile": record.profile,
        "aws_region": record.region,
        "account_id_sha256": record.account_id_sha256,
        "account_profile_validation_status": record.account_profile_validation_status
    }));
    let bytes = serde_json::to_vec_pretty(&payload).map_err(|err| {
        runtime_error_non_retryable(
            "bedrock",
            format!("failed to serialize provider invocation artifact: {err}"),
        )
    })?;
    write_file_atomic(&path, &bytes).map_err(|err| {
        post_success_invocation_artifact_io_error(
            "bedrock",
            format!("failed to write invocation artifact: {err}"),
        )
    })
}

fn post_success_invocation_artifact_io_error(
    provider: &str,
    message: impl Into<String>,
) -> anyhow::Error {
    runtime_error_non_retryable(
        provider,
        format!(
            "partial_success_unknown_invocation_record_io_failure: provider call completed but invocation artifact I/O failed without a safe retry boundary: {}",
            message.into()
        ),
    )
}

fn write_file_atomic(path: &Path, bytes: &[u8]) -> std::io::Result<()> {
    let mut os = path.as_os_str().to_os_string();
    os.push(format!(
        ".tmp-{}-{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos()
    ));
    let tmp = PathBuf::from(os);
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
    if let Some(contents) = json.get("content").and_then(|v| v.as_array()) {
        for content in contents {
            let content_type = content.get("type").and_then(|v| v.as_str());
            if content_type == Some("text") {
                if let Some(text) = content.get("text").and_then(|v| v.as_str()) {
                    chunks.push(text);
                }
            }
        }
    }
    let joined = chunks.join("\n").trim().to_string();
    if joined.is_empty() && json.get("stop_reason").and_then(|v| v.as_str()) == Some("refusal") {
        return Some(r#"{"refusal":"provider refused the request"}"#.to_string());
    }
    (!joined.is_empty()).then_some(joined)
}

fn extract_deepseek_output_text(json: &Value) -> Option<String> {
    let mut chunks = Vec::new();
    for choice in json.get("choices")?.as_array()? {
        if let Some(text) = choice
            .get("message")
            .and_then(|v| v.get("content"))
            .and_then(|v| v.as_str())
        {
            chunks.push(text);
        }
    }
    let joined = chunks.join("\n").trim().to_string();
    (!joined.is_empty()).then_some(joined)
}

fn extract_openrouter_output_text(json: &Value) -> Option<String> {
    extract_deepseek_output_text(json)
}

fn extract_bedrock_nova_output_text(json: &Value) -> Option<String> {
    let mut chunks = Vec::new();
    if let Some(content) = json
        .pointer("/output/message/content")
        .and_then(|v| v.as_array())
    {
        for part in content {
            if let Some(text) = part.get("text").and_then(|v| v.as_str()) {
                chunks.push(text);
            }
        }
    }
    if chunks.is_empty() {
        if let Some(text) = json.get("outputText").and_then(|v| v.as_str()) {
            chunks.push(text);
        }
    }
    let joined = chunks.join("\n").trim().to_string();
    (!joined.is_empty()).then_some(joined)
}

#[derive(Debug, Clone)]
/// OpenAI-compatible provider backed by HTTP/requests API.
pub struct OpenAiProvider {
    endpoint: String,
    auth_env: String,
    model: String,
    max_output_tokens: u64,
    timeout_secs: Option<u64>,
}

impl OpenAiProvider {
    /// Build an OpenAI provider from normalized invocation target.
    pub fn from_target(
        spec: &adl::ProviderSpec,
        target: &ProviderInvocationTargetV1,
    ) -> Result<Self> {
        let endpoint = vendor_endpoint(spec, target, OPENAI_RESPONSES_ENDPOINT, "openai")?;
        let auth_env = auth_env_for(spec, "OPENAI_API_KEY")?;
        validate_vendor_credential_endpoint(
            spec,
            "openai",
            &endpoint,
            &auth_env,
            "OPENAI_API_KEY",
            &["api.openai.com"],
        )?;
        Ok(Self {
            endpoint,
            auth_env,
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
/// Anthropic-compatible provider using the messages API format.
pub struct AnthropicProvider {
    endpoint: String,
    auth_env: String,
    model: String,
    max_tokens: u64,
    timeout_secs: Option<u64>,
}

impl AnthropicProvider {
    /// Build an Anthropic provider from normalized invocation target.
    pub fn from_target(
        spec: &adl::ProviderSpec,
        target: &ProviderInvocationTargetV1,
    ) -> Result<Self> {
        let endpoint = vendor_endpoint(spec, target, ANTHROPIC_MESSAGES_ENDPOINT, "anthropic")?;
        let auth_env = auth_env_for(spec, "ANTHROPIC_API_KEY")?;
        validate_vendor_credential_endpoint(
            spec,
            "anthropic",
            &endpoint,
            &auth_env,
            "ANTHROPIC_API_KEY",
            &["api.anthropic.com"],
        )?;
        Ok(Self {
            endpoint,
            auth_env,
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
/// DeepSeek native provider using the chat completions API format.
pub struct DeepSeekProvider {
    endpoint: String,
    auth_env: String,
    model: String,
    max_tokens: u64,
    timeout_secs: Option<u64>,
}

impl DeepSeekProvider {
    /// Build a DeepSeek provider from normalized invocation target.
    pub fn from_target(
        spec: &adl::ProviderSpec,
        target: &ProviderInvocationTargetV1,
    ) -> Result<Self> {
        let endpoint =
            vendor_endpoint(spec, target, DEEPSEEK_CHAT_COMPLETIONS_ENDPOINT, "deepseek")?;
        let auth_env = auth_env_for(spec, "DEEPSEEK_API_KEY")?;
        validate_vendor_credential_endpoint(
            spec,
            "deepseek",
            &endpoint,
            &auth_env,
            "DEEPSEEK_API_KEY",
            &["api.deepseek.com"],
        )?;
        Ok(Self {
            endpoint,
            auth_env,
            model: target.provider_model_id.clone(),
            max_tokens: cfg_u64(&spec.config, "max_tokens")
                .or_else(|| cfg_u64(&spec.config, "max_output_tokens"))
                .unwrap_or(220),
            timeout_secs: cfg_u64(&spec.config, "timeout_secs"),
        })
    }
}

impl Provider for DeepSeekProvider {
    fn complete(&self, prompt: &str) -> Result<String> {
        let token = env::var(&self.auth_env).map_err(|_| {
            invalid_config(
                "deepseek",
                format!("missing required auth env var '{}'", self.auth_env),
            )
        })?;
        let mut client_builder = reqwest::blocking::Client::builder();
        if let Some(secs) = self.timeout_secs {
            client_builder = client_builder.timeout(Duration::from_secs(secs));
        }
        let client = client_builder
            .build()
            .context("failed to build DeepSeek client")
            .map_err(|err| runtime_error("deepseek", err.to_string()))?;
        let req = client
            .post(&self.endpoint)
            .header("Content-Type", "application/json")
            .bearer_auth(token)
            .json(&serde_json::json!({
                "model": self.model,
                "messages": [{"role": "user", "content": prompt}],
                "max_tokens": self.max_tokens,
                "stream": false,
            }));
        let (json, http_status) = provider_http_json("deepseek", req)?;
        let output = extract_deepseek_output_text(&json).ok_or_else(|| {
            runtime_error_non_retryable("deepseek", "response missing message content")
        })?;
        write_native_invocation_record("deepseek", &self.model, prompt, &output, http_status)?;
        Ok(output)
    }
}

#[derive(Debug, Clone)]
/// OpenRouter native provider using the OpenAI-compatible chat completions format.
pub struct OpenRouterProvider {
    endpoint: String,
    auth_env: String,
    model: String,
    max_tokens: u64,
    timeout_secs: Option<u64>,
}

impl OpenRouterProvider {
    /// Build an OpenRouter provider from normalized invocation target.
    pub fn from_target(
        spec: &adl::ProviderSpec,
        target: &ProviderInvocationTargetV1,
    ) -> Result<Self> {
        let endpoint = vendor_endpoint(
            spec,
            target,
            OPENROUTER_CHAT_COMPLETIONS_ENDPOINT,
            "openrouter",
        )?;
        let auth_env = auth_env_for(spec, "OPENROUTER_API_KEY")?;
        validate_vendor_credential_endpoint(
            spec,
            "openrouter",
            &endpoint,
            &auth_env,
            "OPENROUTER_API_KEY",
            &["openrouter.ai"],
        )?;
        Ok(Self {
            endpoint,
            auth_env,
            model: target.provider_model_id.clone(),
            max_tokens: cfg_u64(&spec.config, "max_tokens")
                .or_else(|| cfg_u64(&spec.config, "max_output_tokens"))
                .unwrap_or(220),
            timeout_secs: cfg_u64(&spec.config, "timeout_secs"),
        })
    }
}

impl Provider for OpenRouterProvider {
    fn complete(&self, prompt: &str) -> Result<String> {
        let token = env::var(&self.auth_env).map_err(|_| {
            invalid_config(
                "openrouter",
                format!("missing required auth env var '{}'", self.auth_env),
            )
        })?;
        let mut client_builder = reqwest::blocking::Client::builder();
        if let Some(secs) = self.timeout_secs {
            client_builder = client_builder.timeout(Duration::from_secs(secs));
        }
        let client = client_builder
            .build()
            .context("failed to build OpenRouter client")
            .map_err(|err| runtime_error("openrouter", err.to_string()))?;
        let req = client
            .post(&self.endpoint)
            .header("Content-Type", "application/json")
            .bearer_auth(token)
            .json(&serde_json::json!({
                "model": self.model,
                "messages": [{"role": "user", "content": prompt}],
                "max_tokens": self.max_tokens,
                "stream": false,
            }));
        let (json, http_status) = provider_http_json("openrouter", req)?;
        let output = extract_openrouter_output_text(&json).ok_or_else(|| {
            runtime_error_non_retryable("openrouter", "response missing message content")
        })?;
        write_native_invocation_record("openrouter", &self.model, prompt, &output, http_status)?;
        Ok(output)
    }
}

const DEFAULT_BEDROCK_PROFILE: &str = "agent-logic-admin";
const DEFAULT_BEDROCK_REGION: &str = "us-west-2";
const BEDROCK_EXPECTED_ACCOUNT_SHA256_ENV: &str = "ADL_AWS_BEDROCK_ACCOUNT_SHA256";

#[derive(Debug, Clone)]
/// AWS Bedrock native provider using Bedrock Runtime InvokeModel.
pub struct AwsBedrockProvider {
    model: String,
    region: String,
    profile: String,
    expected_account_sha256: Option<String>,
    max_tokens: u64,
    timeout_secs: Option<u64>,
}

impl AwsBedrockProvider {
    /// Build an AWS Bedrock provider from normalized invocation target.
    pub fn from_target(
        spec: &adl::ProviderSpec,
        target: &ProviderInvocationTargetV1,
    ) -> Result<Self> {
        let region = cfg_string(&spec.config, "region")
            .or_else(|| env::var("AWS_REGION").ok())
            .or_else(|| env::var("AWS_DEFAULT_REGION").ok())
            .unwrap_or_else(|| DEFAULT_BEDROCK_REGION.to_string());
        let profile = cfg_string(&spec.config, "profile")
            .or_else(|| env::var("ADL_AWS_PROFILE").ok())
            .or_else(|| env::var("AWS_PROFILE").ok())
            .unwrap_or_else(|| DEFAULT_BEDROCK_PROFILE.to_string());
        if profile != DEFAULT_BEDROCK_PROFILE {
            return Err(invalid_config(
                "bedrock",
                format!(
                    "AWS Bedrock provider requires Agent Logic AWS profile '{DEFAULT_BEDROCK_PROFILE}' (got '{profile}')"
                ),
            ));
        }
        let config_expected_account_sha256 = cfg_string(&spec.config, "expected_account_sha256")
            .or_else(|| cfg_string(&spec.config, "expected-account-sha256"));
        let env_expected_account_sha256 = env::var(BEDROCK_EXPECTED_ACCOUNT_SHA256_ENV).ok();
        let expected_account_sha256 = match (
            env_expected_account_sha256.as_deref(),
            config_expected_account_sha256.as_deref(),
        ) {
            (Some(env_expected), Some(config_expected)) => {
                let env_expected = normalize_sha256_hex(env_expected)
                    .map_err(|err| invalid_config("bedrock", err))?;
                validate_sha256_hex(config_expected)
                    .map_err(|err| invalid_config("bedrock", err))?;
                let config_expected = config_expected.to_ascii_lowercase();
                if env_expected != config_expected {
                    return Err(invalid_config(
                        "bedrock",
                        format!(
                            "{BEDROCK_EXPECTED_ACCOUNT_SHA256_ENV} is authoritative and conflicts with config.expected_account_sha256"
                        ),
                    ));
                }
                Some(env_expected)
            }
            (Some(env_expected), None) => Some(
                normalize_sha256_hex(env_expected).map_err(|err| invalid_config("bedrock", err))?,
            ),
            (None, Some(config_expected)) => Some(
                normalize_sha256_hex(config_expected)
                    .map_err(|err| invalid_config("bedrock", err))?,
            ),
            (None, None) => None,
        };
        if let Some(expected) = expected_account_sha256.as_deref() {
            validate_sha256_hex(expected).map_err(|err| invalid_config("bedrock", err))?;
        }
        Ok(Self {
            model: target.provider_model_id.clone(),
            region,
            profile,
            expected_account_sha256,
            max_tokens: cfg_u64(&spec.config, "max_tokens")
                .or_else(|| cfg_u64(&spec.config, "max_output_tokens"))
                .unwrap_or(220),
            timeout_secs: cfg_u64(&spec.config, "timeout_secs"),
        })
    }

    async fn complete_async(&self, prompt: &str) -> Result<String> {
        let region_provider =
            RegionProviderChain::first_try(Some(aws_config::Region::new(self.region.clone())));
        let mut timeout_config = aws_config::timeout::TimeoutConfig::builder()
            .connect_timeout(Duration::from_secs(5))
            .operation_timeout(Duration::from_secs(self.timeout_secs.unwrap_or(45)));
        if let Some(secs) = self.timeout_secs {
            timeout_config = timeout_config.operation_attempt_timeout(Duration::from_secs(secs));
        }
        let shared_config = aws_config::defaults(BehaviorVersion::latest())
            .region(region_provider)
            .profile_name(&self.profile)
            .timeout_config(timeout_config.build())
            .load()
            .await;
        let identity = sts::Client::new(&shared_config)
            .get_caller_identity()
            .send()
            .await
            .map_err(|err| bedrock_sdk_error(format!("{err:?}")))?;
        let account_id_sha256 = identity.account().map(sha256_hex);
        verify_bedrock_account_identity(
            account_id_sha256.as_deref(),
            self.expected_account_sha256.as_deref(),
        )?;
        let body = bedrock_nova_request_body(prompt, self.max_tokens);
        let response = bedrockruntime::Client::new(&shared_config)
            .invoke_model()
            .model_id(&self.model)
            .content_type("application/json")
            .accept("application/json")
            .body(bedrockruntime::primitives::Blob::new(
                serde_json::to_vec(&body).map_err(|err| {
                    runtime_error_non_retryable(
                        "bedrock",
                        format!("failed to serialize Bedrock request: {err}"),
                    )
                })?,
            ))
            .send()
            .await
            .map_err(|err| bedrock_sdk_error(format!("{err:?}")))?;
        let json: Value = serde_json::from_slice(response.body().as_ref()).map_err(|err| {
            runtime_error_non_retryable("bedrock", format!("invalid Bedrock JSON: {err}"))
        })?;
        let output = extract_bedrock_nova_output_text(&json).ok_or_else(|| {
            runtime_error_non_retryable("bedrock", "response missing Bedrock output text")
        })?;
        write_bedrock_invocation_record(BedrockInvocationRecord {
            model: &self.model,
            prompt,
            output: &output,
            http_status: 200,
            profile: &self.profile,
            region: &self.region,
            account_id_sha256: account_id_sha256.as_deref(),
            account_profile_validation_status: "account_hash_verified",
        })?;
        Ok(output)
    }
}

fn validate_sha256_hex(value: &str) -> std::result::Result<(), String> {
    if value.len() == 64 && value.chars().all(|ch| ch.is_ascii_hexdigit()) {
        Ok(())
    } else {
        Err("expected account hash must be a 64-character SHA-256 hex digest".to_string())
    }
}

fn normalize_sha256_hex(value: &str) -> std::result::Result<String, String> {
    validate_sha256_hex(value)?;
    Ok(value.to_ascii_lowercase())
}

fn verify_bedrock_account_identity(
    account_id_sha256: Option<&str>,
    expected_account_sha256: Option<&str>,
) -> Result<()> {
    let Some(expected) = expected_account_sha256 else {
        return Err(runtime_error_non_retryable(
            "bedrock",
            format!(
                "AWS Bedrock provider requires operator-approved expected account hash; set {BEDROCK_EXPECTED_ACCOUNT_SHA256_ENV} or config.expected_account_sha256"
            ),
        ));
    };
    let expected = normalize_sha256_hex(expected).map_err(|err| invalid_config("bedrock", err))?;
    let Some(observed) = account_id_sha256 else {
        return Err(runtime_error_non_retryable(
            "bedrock",
            "AWS Bedrock STS identity did not include an account id",
        ));
    };
    if observed != expected {
        return Err(runtime_error_non_retryable(
            "bedrock",
            "AWS Bedrock profile account hash does not match expected Agent Logic account hash",
        ));
    }
    Ok(())
}

impl Provider for AwsBedrockProvider {
    fn complete(&self, prompt: &str) -> Result<String> {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|err| runtime_error("bedrock", format!("failed to build runtime: {err}")))?
            .block_on(self.complete_async(prompt))
    }
}

fn cfg_string(cfg: &HashMap<String, Value>, key: &str) -> Option<String> {
    cfg.get(key)
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|v| !v.is_empty())
        .map(ToString::to_string)
}

fn bedrock_nova_request_body(prompt: &str, max_tokens: u64) -> Value {
    serde_json::json!({
        "schemaVersion": "messages-v1",
        "messages": [{
            "role": "user",
            "content": [{"text": prompt}],
        }],
        "inferenceConfig": {
            "maxTokens": max_tokens,
        },
    })
}

fn bedrock_sdk_error(message: String) -> anyhow::Error {
    let sanitized = sanitize_bedrock_error(&message);
    let retryable = sanitized.contains("Throttling")
        || sanitized.contains("TooManyRequests")
        || sanitized.contains("timeout")
        || sanitized.contains("Timeout")
        || sanitized.contains("ServiceUnavailable")
        || sanitized.contains("InternalServer");
    if retryable {
        runtime_error("bedrock", sanitized)
    } else {
        runtime_error_non_retryable("bedrock", sanitized)
    }
}

fn sanitize_bedrock_error(message: &str) -> String {
    let mut out = message.replace('\n', " ");
    for marker in [
        "Authorization: ",
        "Authorization=",
        "Credential=",
        "X-Amz-Signature=",
        "SecretAccessKey=",
    ] {
        out = redact_aws_error_value(&out, marker);
    }
    out = redact_aws_arns(&out);
    out = redact_aws_account_ids(&out);
    truncate_provider_body(&out)
}

fn redact_aws_arns(input: &str) -> String {
    let mut redacted = String::with_capacity(input.len());
    let mut cursor = 0;
    while let Some(relative_start) = input[cursor..].find("arn:aws") {
        let arn_start = cursor + relative_start;
        let Some(next) = input[arn_start + "arn:aws".len()..].chars().next() else {
            redacted.push_str(&input[cursor..]);
            return redacted;
        };
        if next != ':' && next != '-' {
            let prefix_end = arn_start + "arn:aws".len();
            redacted.push_str(&input[cursor..prefix_end]);
            cursor = prefix_end;
            continue;
        }
        redacted.push_str(&input[cursor..arn_start]);
        redacted.push_str("<redacted-aws-arn>");

        let arn_end = input[arn_start..]
            .char_indices()
            .find_map(|(idx, ch)| {
                matches!(ch, ' ' | ',' | ';' | '"' | '\'' | ')' | '}' | ']')
                    .then_some(arn_start + idx)
            })
            .unwrap_or(input.len());
        cursor = arn_end;
    }
    redacted.push_str(&input[cursor..]);
    redacted
}

fn redact_aws_account_ids(input: &str) -> String {
    let mut redacted = String::with_capacity(input.len());
    let mut digit_start = None;
    let mut digit_count = 0usize;
    let mut last_end = 0usize;

    for (idx, ch) in input.char_indices() {
        if ch.is_ascii_digit() {
            if digit_start.is_none() {
                digit_start = Some(idx);
            }
            digit_count += 1;
            continue;
        }

        if let Some(start) = digit_start {
            if digit_count == 12 {
                redacted.push_str(&input[last_end..start]);
                redacted.push_str("<redacted-aws-account-id>");
                last_end = idx;
            }
        }
        digit_start = None;
        digit_count = 0;
    }

    if let Some(start) = digit_start {
        if digit_count == 12 {
            redacted.push_str(&input[last_end..start]);
            redacted.push_str("<redacted-aws-account-id>");
            last_end = input.len();
        }
    }

    redacted.push_str(&input[last_end..]);
    redacted
}

fn redact_aws_error_value(input: &str, marker: &str) -> String {
    let mut redacted = String::with_capacity(input.len());
    let mut cursor = 0;
    while let Some(relative_start) = input[cursor..].find(marker) {
        let marker_start = cursor + relative_start;
        let value_start = marker_start + marker.len();
        redacted.push_str(&input[cursor..value_start]);
        redacted.push_str("<redacted>");

        let value_end = input[value_start..]
            .char_indices()
            .find_map(|(idx, ch)| {
                matches!(ch, ' ' | ',' | '&' | ';' | '"' | '\'' | ')' | '}' | ']')
                    .then_some(value_start + idx)
            })
            .unwrap_or(input.len());
        cursor = value_end;
    }
    redacted.push_str(&input[cursor..]);
    redacted
}

fn sha256_hex(value: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(value.as_bytes());
    format!("{:x}", hasher.finalize())
}

#[derive(Debug, Clone)]
/// Z.ai native provider using the OpenAI-compatible chat completions API format.
pub struct ZAiProvider {
    endpoint: String,
    auth_env: String,
    model: String,
    max_tokens: u64,
    timeout_secs: Option<u64>,
}

impl ZAiProvider {
    /// Build a Z.ai provider from normalized invocation target.
    pub fn from_target(
        spec: &adl::ProviderSpec,
        target: &ProviderInvocationTargetV1,
    ) -> Result<Self> {
        let endpoint = vendor_endpoint(spec, target, Z_AI_CHAT_COMPLETIONS_ENDPOINT, "z_ai")?;
        let auth_env = auth_env_for(spec, "ZAI_API_KEY")?;
        validate_vendor_credential_endpoint(
            spec,
            "z_ai",
            &endpoint,
            &auth_env,
            "ZAI_API_KEY",
            &["open.bigmodel.cn", "api.z.ai"],
        )?;
        Ok(Self {
            endpoint,
            auth_env,
            model: target.provider_model_id.clone(),
            max_tokens: cfg_u64(&spec.config, "max_tokens")
                .or_else(|| cfg_u64(&spec.config, "max_output_tokens"))
                .unwrap_or(220),
            timeout_secs: cfg_u64(&spec.config, "timeout_secs"),
        })
    }
}

impl Provider for ZAiProvider {
    fn complete(&self, prompt: &str) -> Result<String> {
        let token = env::var(&self.auth_env).map_err(|_| {
            invalid_config(
                "z_ai",
                format!("missing required auth env var '{}'", self.auth_env),
            )
        })?;
        let mut client_builder = reqwest::blocking::Client::builder();
        if let Some(secs) = self.timeout_secs {
            client_builder = client_builder.timeout(Duration::from_secs(secs));
        }
        let client = client_builder
            .build()
            .context("failed to build Z.ai client")
            .map_err(|err| runtime_error("z_ai", err.to_string()))?;
        let req = client
            .post(&self.endpoint)
            .header("Content-Type", "application/json")
            .bearer_auth(token)
            .json(&serde_json::json!({
                "model": self.model,
                "messages": [{"role": "user", "content": prompt}],
                "max_tokens": self.max_tokens,
                "stream": false,
            }));
        let (json, http_status) = provider_http_json("z_ai", req)?;
        let output = extract_deepseek_output_text(&json).ok_or_else(|| {
            runtime_error_non_retryable("z_ai", "response missing message content")
        })?;
        write_native_invocation_record("z_ai", &self.model, prompt, &output, http_status)?;
        Ok(output)
    }
}

#[derive(Debug, Clone)]
/// Generic HTTP provider for configurable endpoint + optional bearer auth.
pub struct HttpProvider {
    endpoint: String,
    auth: Option<HttpAuth>,
    headers: HashMap<String, String>,
    timeout_secs: Option<u64>,
}

#[derive(Debug, Clone)]
/// Ollama-specific HTTP provider with prompt/model serialization.
pub struct OllamaHttpProvider {
    endpoint: String,
    model: String,
    temperature: Option<f32>,
    timeout_secs: Option<u64>,
}

impl OllamaHttpProvider {
    /// Build an Ollama HTTP provider from the normalized invocation target.
    pub fn from_target(
        spec: &adl::ProviderSpec,
        target: &ProviderInvocationTargetV1,
    ) -> Result<Self> {
        let timeout_secs = match cfg_u64_strict(&spec.config, "timeout_secs", "ollama")? {
            Some(value) => value,
            None => timeout_secs().map_err(|err| invalid_config("ollama", err.to_string()))?,
        };
        Ok(Self {
            endpoint: ollama_generate_endpoint(spec)?,
            model: target.provider_model_id.clone(),
            temperature: super::local::cfg_f32(&spec.config, "temperature"),
            timeout_secs: Some(timeout_secs),
        })
    }
}

impl Provider for OllamaHttpProvider {
    fn complete(&self, prompt: &str) -> Result<String> {
        let mut client_builder = reqwest::blocking::Client::builder();
        if let Some(secs) = self.timeout_secs {
            client_builder = client_builder.timeout(Duration::from_secs(secs));
        }
        let client = client_builder
            .build()
            .context("failed to build ollama http client")
            .map_err(|err| runtime_error("ollama", err.to_string()))?;

        let mut body = serde_json::json!({
            "model": self.model,
            "prompt": prompt,
            "stream": false,
        });
        if let Some(temperature) = self.temperature {
            body["options"] = serde_json::json!({ "temperature": temperature });
        }

        let req = client
            .post(&self.endpoint)
            .header("Content-Type", "application/json")
            .json(&body);
        let (json, http_status) = provider_http_json("ollama", req)?;
        let output = json
            .get("response")
            .and_then(|v| v.as_str())
            .map(str::trim)
            .filter(|v| !v.is_empty())
            .ok_or_else(|| {
                runtime_error_non_retryable("ollama", "response missing 'response' text field")
            })?
            .to_string();
        write_native_invocation_record("ollama", &self.model, prompt, &output, http_status)?;
        Ok(output)
    }
}

impl HttpProvider {
    /// Build an HTTP provider from an already-normalized invocation spec.
    pub fn from_spec(spec: &adl::ProviderSpec) -> Result<Self> {
        let target = provider_substrate::provider_invocation_target_v1(
            spec.id.as_deref().unwrap_or("<anonymous-provider>"),
            spec,
            None,
        )?;
        Self::from_target(spec, &target)
    }

    /// Build a generic HTTP provider from the normalized invocation target.
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
        if auth.is_some() {
            validate_http_credential_endpoint(cfg, &endpoint)?;
        }

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

#[cfg(test)]
mod tests;
