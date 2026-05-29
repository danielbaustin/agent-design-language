use anyhow::{anyhow, Result};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::Path;

use crate::model_identity::{
    normalize_sha256_digest, observed_at_now_v1, ModelIdentityStrengthV1, ModelIdentityV1,
};

pub const PROVIDER_COMMUNICATION_SCHEMA_VERSION: &str = "provider_communication.v1";

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ProviderKindV1 {
    Hosted,
    Local,
    Mock,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeSurfaceV1 {
    HostedApi,
    OllamaHttp,
    OllamaCli,
    Mock,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct ProviderRouteV1 {
    pub provider_kind: ProviderKindV1,
    pub provider: String,
    pub runtime_surface: RuntimeSurfaceV1,
    pub provider_model_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credential_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_registry: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct ProviderAttemptPolicyV1 {
    pub max_attempts: u32,
    pub timeout_ms: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retry_backoff_ms: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ProviderInvocationRequestV1 {
    pub route: ProviderRouteV1,
    pub model_identity: ModelIdentityV1,
    pub prompt_contract_ref: String,
    pub lane_ref: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    pub attempt_policy: ProviderAttemptPolicyV1,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inference_parameter_fingerprint: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_surface: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub governance_surface: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evaluator_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub benchmark_ref: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ProviderAttemptStatusV1 {
    Ok,
    Error,
    Timeout,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ProviderFailureKindV1 {
    ProviderAuthMissing,
    ProviderAuthError,
    ProviderRateLimited,
    ProviderTimeout,
    ProviderTransientHttp,
    ProviderEmptyTextOutput,
    ProviderModelUnavailable,
    ProviderBillingBlocked,
    LocalRuntimeUnavailable,
    LocalRuntimeBusy,
    LocalRuntimeHung,
    ProviderError,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct ProviderFailureV1 {
    pub kind: ProviderFailureKindV1,
    pub retryable: bool,
    pub message: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider_error_excerpt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub http_status: Option<u16>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct ProviderAttemptV1 {
    pub attempt_index: u32,
    pub started_at: String,
    pub duration_ms: u64,
    pub status: ProviderAttemptStatusV1,
    pub retryable: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub http_status: Option<u16>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failure: Option<ProviderFailureV1>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub raw_response_excerpt: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ProviderInvocationFinalStatusV1 {
    Ok,
    Failed,
    Skipped,
    Blocked,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct ProviderInvocationResultV1 {
    pub schema_version: String,
    pub route: ProviderRouteV1,
    pub model_identity: ModelIdentityV1,
    pub attempts: Vec<ProviderAttemptV1>,
    pub final_status: ProviderInvocationFinalStatusV1,
    pub duration_ms: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output_text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output_text_excerpt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failure: Option<ProviderFailureV1>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub artifact_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trace_ref: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct ProviderRunLogEventV1 {
    pub schema_version: String,
    pub timestamp: String,
    pub run_id: String,
    pub event_type: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider_kind: Option<ProviderKindV1>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub runtime_surface: Option<RuntimeSurfaceV1>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider_model_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lane_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attempt_index: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failure_kind: Option<ProviderFailureKindV1>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<Value>,
}

pub struct ProviderRunLoggerV1 {
    run_id: String,
    writer: BufWriter<File>,
}

impl ProviderRunLoggerV1 {
    pub fn create(path: impl AsRef<Path>, run_id: impl Into<String>) -> Result<Self> {
        let path = path.as_ref();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let file = OpenOptions::new().create(true).append(true).open(path)?;
        Ok(Self {
            run_id: run_id.into(),
            writer: BufWriter::new(file),
        })
    }

    pub fn event(&mut self, mut event: ProviderRunLogEventV1) -> Result<()> {
        event.schema_version = PROVIDER_COMMUNICATION_SCHEMA_VERSION.to_string();
        event.run_id = self.run_id.clone();
        event.timestamp = observed_at_now_v1();
        scrub_log_event(&mut event);
        let line = serde_json::to_string(&event)?;
        self.writer.write_all(line.as_bytes())?;
        self.writer.write_all(b"\n")?;
        self.writer.flush()?;
        Ok(())
    }
}

impl ProviderRunLogEventV1 {
    pub fn new(event_type: impl Into<String>) -> Self {
        Self {
            schema_version: PROVIDER_COMMUNICATION_SCHEMA_VERSION.to_string(),
            timestamp: observed_at_now_v1(),
            run_id: String::new(),
            event_type: event_type.into(),
            provider: None,
            provider_kind: None,
            runtime_surface: None,
            model_ref: None,
            provider_model_id: None,
            lane_ref: None,
            task_id: None,
            attempt_index: None,
            status: None,
            failure_kind: None,
            message: None,
            fields: None,
        }
    }

    pub fn with_route(mut self, route: &ProviderRouteV1, model_identity: &ModelIdentityV1) -> Self {
        self.provider = Some(route.provider.clone());
        self.provider_kind = Some(route.provider_kind.clone());
        self.runtime_surface = Some(route.runtime_surface.clone());
        self.model_ref = Some(model_identity.model_ref.clone());
        self.provider_model_id = Some(route.provider_model_id.clone());
        self
    }
}

pub fn classify_provider_failure(note: &str, http_status: Option<u16>) -> ProviderFailureKindV1 {
    let lower = note.to_ascii_lowercase();
    if lower.contains("unauthorized")
        || lower.contains("forbidden")
        || lower.contains("invalid api key")
        || lower.contains("invalid_api_key")
        || http_status == Some(401)
        || http_status == Some(403)
    {
        return ProviderFailureKindV1::ProviderAuthError;
    }
    if lower.contains("missing required environment variable")
        || lower.contains("missing api_key")
        || lower.contains("missing api key")
    {
        return ProviderFailureKindV1::ProviderAuthMissing;
    }
    if lower.contains("rate limit") || lower.contains("rate_limited") || http_status == Some(429) {
        return ProviderFailureKindV1::ProviderRateLimited;
    }
    if lower.contains("timed out") || lower.contains("timeout") {
        return ProviderFailureKindV1::ProviderTimeout;
    }
    if lower.contains("credit balance") || lower.contains("billing") {
        return ProviderFailureKindV1::ProviderBillingBlocked;
    }
    if lower.contains("local_runtime_busy") || lower.contains("non-target model") {
        return ProviderFailureKindV1::LocalRuntimeBusy;
    }
    if lower.contains("local_runtime_hung") || lower.contains("stopping...") {
        return ProviderFailureKindV1::LocalRuntimeHung;
    }
    if lower.contains("connection refused")
        || lower.contains("ollama") && lower.contains("not running")
        || lower.contains("local_runtime_unavailable")
    {
        return ProviderFailureKindV1::LocalRuntimeUnavailable;
    }
    if lower.contains("model") && (lower.contains("not found") || lower.contains("does not exist"))
    {
        return ProviderFailureKindV1::ProviderModelUnavailable;
    }
    if lower.contains("empty") && (lower.contains("response") || lower.contains("output")) {
        return ProviderFailureKindV1::ProviderEmptyTextOutput;
    }
    if matches!(http_status, Some(500..=599)) {
        return ProviderFailureKindV1::ProviderTransientHttp;
    }
    if http_status.is_some() || lower.contains("provider_") {
        return ProviderFailureKindV1::ProviderError;
    }
    ProviderFailureKindV1::Unknown
}

pub fn provider_failure_from_note(note: &str, http_status: Option<u16>) -> ProviderFailureV1 {
    let kind = classify_provider_failure(note, http_status);
    let retryable = matches!(
        kind,
        ProviderFailureKindV1::ProviderRateLimited
            | ProviderFailureKindV1::ProviderTimeout
            | ProviderFailureKindV1::ProviderTransientHttp
            | ProviderFailureKindV1::LocalRuntimeUnavailable
            | ProviderFailureKindV1::LocalRuntimeBusy
            | ProviderFailureKindV1::LocalRuntimeHung
            | ProviderFailureKindV1::Unknown
    );
    ProviderFailureV1 {
        kind,
        retryable,
        message: sanitize_provider_message(note),
        provider_error_excerpt: Some(redacted_excerpt(note)),
        http_status,
    }
}

pub fn hosted_model_identity(
    provider: impl Into<String>,
    model_ref: impl Into<String>,
    provider_model_id: impl Into<String>,
    source_registry: Option<String>,
) -> ModelIdentityV1 {
    let provider = provider.into();
    let model_ref = model_ref.into();
    let provider_model_id = provider_model_id.into();
    ModelIdentityV1 {
        provider_kind: "hosted".to_string(),
        provider,
        model_ref,
        provider_model_id,
        runtime_surface: "hosted_api".to_string(),
        identity_strength: ModelIdentityStrengthV1::ProviderAsserted,
        observed_at: observed_at_now_v1(),
        resolved_digest: None,
        source_registry,
        runtime_fingerprint: None,
        inference_parameter_fingerprint: None,
        tool_surface: None,
        governance_surface: None,
        evaluator_ref: None,
        lane_ref: None,
        benchmark_ref: None,
    }
}

pub fn ollama_model_identity(
    model_ref: impl Into<String>,
    provider_model_id: impl Into<String>,
    observed_digest: Option<&str>,
    source_registry: Option<String>,
) -> ModelIdentityV1 {
    let resolved_digest = observed_digest.and_then(normalize_sha256_digest);
    let identity_strength = if resolved_digest.is_some() {
        ModelIdentityStrengthV1::Pinned
    } else {
        ModelIdentityStrengthV1::TagOnly
    };
    ModelIdentityV1 {
        provider_kind: "local".to_string(),
        provider: "ollama".to_string(),
        model_ref: model_ref.into(),
        provider_model_id: provider_model_id.into(),
        runtime_surface: "ollama_http".to_string(),
        identity_strength,
        observed_at: observed_at_now_v1(),
        resolved_digest,
        source_registry,
        runtime_fingerprint: None,
        inference_parameter_fingerprint: None,
        tool_surface: None,
        governance_surface: None,
        evaluator_ref: None,
        lane_ref: None,
        benchmark_ref: None,
    }
}

pub fn validate_provider_request(request: &ProviderInvocationRequestV1) -> Result<()> {
    require_non_empty("route.provider", &request.route.provider)?;
    require_non_empty("route.provider_model_id", &request.route.provider_model_id)?;
    require_non_empty("prompt_contract_ref", &request.prompt_contract_ref)?;
    require_non_empty("lane_ref", &request.lane_ref)?;
    if request.attempt_policy.max_attempts == 0 {
        return Err(anyhow!(
            "attempt_policy.max_attempts must be greater than zero"
        ));
    }
    if request.attempt_policy.timeout_ms == 0 {
        return Err(anyhow!(
            "attempt_policy.timeout_ms must be greater than zero"
        ));
    }
    Ok(())
}

fn sanitize_provider_message(note: &str) -> String {
    let text = note.split_whitespace().collect::<Vec<_>>().join(" ");
    let lowered = text.to_ascii_lowercase();
    let sensitive = [
        "authorization",
        "bearer ",
        "x-api-key",
        "api key",
        ".key",
        "prompt:",
        "raw prompt",
        "user said",
        "messages",
        "tool arguments",
        "tool_args",
        "request body",
        "request_body",
    ];
    if sensitive.iter().any(|marker| lowered.contains(marker)) {
        return "redacted provider diagnostic".to_string();
    }
    truncate_chars(&text, 180)
}

fn scrub_log_event(event: &mut ProviderRunLogEventV1) {
    if event.message.is_some() {
        event.message = Some("redacted provider diagnostic".to_string());
    }
    if event.fields.is_some() {
        event.fields =
            Some(serde_json::json!({"redacted": "event fields omitted from provider run log"}));
    }
}

fn truncate_chars(text: &str, max_chars: usize) -> String {
    let mut iter = text.chars();
    let mut out: String = iter.by_ref().take(max_chars).collect();
    if iter.next().is_some() {
        let keep = out.chars().count().saturating_sub(3);
        out = out.chars().take(keep).collect();
        out.push_str("...");
    }
    out
}

fn redacted_excerpt(note: &str) -> String {
    format!("[redacted provider detail len={}]", note.len())
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
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_log_path() -> std::path::PathBuf {
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!("adl_provider_run_log_{stamp}.jsonl"))
    }

    #[test]
    fn hosted_identity_is_provider_asserted_without_digest() {
        let identity = hosted_model_identity("openai", "reasoning/default", "gpt-5.5", None);
        assert_eq!(identity.provider_kind, "hosted");
        assert_eq!(
            identity.identity_strength,
            ModelIdentityStrengthV1::ProviderAsserted
        );
        assert!(identity.resolved_digest.is_none());
    }

    #[test]
    fn ollama_identity_is_pinned_only_with_digest() {
        let digest = format!("sha256:{}", "a".repeat(64));
        let pinned = ollama_model_identity("qwen", "qwen3:30b", Some(&digest), None);
        assert_eq!(pinned.identity_strength, ModelIdentityStrengthV1::Pinned);
        assert_eq!(pinned.resolved_digest.as_deref(), Some(digest.as_str()));

        let tag_only = ollama_model_identity("qwen", "qwen3:30b", None, None);
        assert_eq!(tag_only.identity_strength, ModelIdentityStrengthV1::TagOnly);
        assert!(tag_only.resolved_digest.is_none());
    }

    #[test]
    fn failure_classifier_covers_hosted_and_local_operational_failures() {
        assert_eq!(
            classify_provider_failure("missing required environment variable OPENAI_API_KEY", None),
            ProviderFailureKindV1::ProviderAuthMissing
        );
        assert_eq!(
            classify_provider_failure("status=429 rate limit exceeded", Some(429)),
            ProviderFailureKindV1::ProviderRateLimited
        );
        assert_eq!(
            classify_provider_failure("local_runtime_busy: non-target model loaded", None),
            ProviderFailureKindV1::LocalRuntimeBusy
        );
        assert_eq!(
            classify_provider_failure("connection refused talking to Ollama", None),
            ProviderFailureKindV1::LocalRuntimeUnavailable
        );
        assert_eq!(
            classify_provider_failure("model qwen3:30b not found", None),
            ProviderFailureKindV1::ProviderModelUnavailable
        );
    }

    #[test]
    fn provider_run_logger_writes_flushed_jsonl_without_prompt_or_secret_fields() {
        let path = temp_log_path();
        let route = ProviderRouteV1 {
            provider_kind: ProviderKindV1::Hosted,
            provider: "openai".to_string(),
            runtime_surface: RuntimeSurfaceV1::HostedApi,
            provider_model_id: "gpt-5.5".to_string(),
            endpoint_ref: Some("openai.responses".to_string()),
            credential_ref: Some("env:OPENAI_API_KEY".to_string()),
            source_registry: None,
        };
        let identity = hosted_model_identity("openai", "frontier/default", "gpt-5.5", None);
        let mut logger = ProviderRunLoggerV1::create(&path, "run-1").unwrap();
        let mut event = ProviderRunLogEventV1::new("attempt_failure").with_route(&route, &identity);
        event.message = Some("Bearer secret-token prompt: send money".to_string());
        event.fields =
            Some(serde_json::json!({"prompt":"raw prompt", "authorization":"Bearer secret-token"}));
        logger.event(event).unwrap();
        drop(logger);

        let raw = fs::read_to_string(&path).unwrap();
        let row: ProviderRunLogEventV1 = serde_json::from_str(raw.trim()).unwrap();
        assert_eq!(row.run_id, "run-1");
        assert_eq!(row.event_type, "attempt_failure");
        assert_eq!(row.provider.as_deref(), Some("openai"));
        assert!(!raw.contains("Bearer"));
        assert!(!raw.contains("secret-token"));
        assert!(!raw.contains("raw prompt"));
        assert!(!raw.contains("private flight"));
        assert!(!raw.contains("Seattle"));
        assert!(raw.contains("redacted"));
        let _ = fs::remove_file(path);
    }

    #[test]
    fn utf8_diagnostics_truncate_without_panic() {
        let diagnostic = format!("{}{}", "界".repeat(200), "é");
        let sanitized = sanitize_provider_message(&diagnostic);
        assert!(sanitized.ends_with("..."));
        assert!(sanitized.is_char_boundary(sanitized.len()));
    }

    #[test]
    fn invalid_api_key_is_auth_error_not_missing_auth() {
        assert_eq!(
            classify_provider_failure("invalid API key provided", None),
            ProviderFailureKindV1::ProviderAuthError
        );
    }

    #[test]
    fn hosted_and_ollama_results_share_normalized_shape() {
        let hosted_route = ProviderRouteV1 {
            provider_kind: ProviderKindV1::Hosted,
            provider: "openai".to_string(),
            runtime_surface: RuntimeSurfaceV1::HostedApi,
            provider_model_id: "gpt-5.5".to_string(),
            endpoint_ref: Some("openai.responses".to_string()),
            credential_ref: Some("env:OPENAI_API_KEY".to_string()),
            source_registry: Some("hosted-panel".to_string()),
        };
        let hosted_identity = hosted_model_identity(
            "openai",
            "frontier/default",
            "gpt-5.5",
            Some("hosted-panel".to_string()),
        );
        let hosted = ProviderInvocationResultV1 {
            schema_version: PROVIDER_COMMUNICATION_SCHEMA_VERSION.to_string(),
            route: hosted_route,
            model_identity: hosted_identity,
            attempts: vec![ProviderAttemptV1 {
                attempt_index: 1,
                started_at: "unix:1".to_string(),
                duration_ms: 10,
                status: ProviderAttemptStatusV1::Ok,
                retryable: false,
                http_status: Some(200),
                failure: None,
                raw_response_excerpt: Some("[redacted response len=2]".to_string()),
            }],
            final_status: ProviderInvocationFinalStatusV1::Ok,
            duration_ms: 10,
            output_text: Some("{}".to_string()),
            output_text_excerpt: Some("{}".to_string()),
            failure: None,
            artifact_ref: None,
            trace_ref: None,
        };

        let ollama_route = ProviderRouteV1 {
            provider_kind: ProviderKindV1::Local,
            provider: "ollama".to_string(),
            runtime_surface: RuntimeSurfaceV1::OllamaHttp,
            provider_model_id: "qwen3:30b".to_string(),
            endpoint_ref: Some("ollama.local".to_string()),
            credential_ref: None,
            source_registry: Some("local-panel".to_string()),
        };
        let ollama_identity = ollama_model_identity(
            "qwen",
            "qwen3:30b",
            Some(&format!("sha256:{}", "b".repeat(64))),
            Some("local-panel".to_string()),
        );
        let failure = provider_failure_from_note("connection refused talking to Ollama", None);
        let ollama = ProviderInvocationResultV1 {
            schema_version: PROVIDER_COMMUNICATION_SCHEMA_VERSION.to_string(),
            route: ollama_route,
            model_identity: ollama_identity,
            attempts: vec![ProviderAttemptV1 {
                attempt_index: 1,
                started_at: "unix:1".to_string(),
                duration_ms: 10,
                status: ProviderAttemptStatusV1::Error,
                retryable: true,
                http_status: None,
                failure: Some(failure.clone()),
                raw_response_excerpt: None,
            }],
            final_status: ProviderInvocationFinalStatusV1::Failed,
            duration_ms: 10,
            output_text: None,
            output_text_excerpt: None,
            failure: Some(failure),
            artifact_ref: None,
            trace_ref: None,
        };

        let hosted_doc = serde_json::to_value(&hosted).unwrap();
        let ollama_doc = serde_json::to_value(&ollama).unwrap();
        for field in [
            "schema_version",
            "route",
            "model_identity",
            "attempts",
            "final_status",
            "duration_ms",
        ] {
            assert!(hosted_doc.get(field).is_some(), "hosted missing {field}");
            assert!(ollama_doc.get(field).is_some(), "ollama missing {field}");
        }
        assert_eq!(
            ollama.failure.unwrap().kind,
            ProviderFailureKindV1::LocalRuntimeUnavailable
        );
    }

    #[test]
    fn request_validation_rejects_empty_and_zero_policy_fields() {
        let identity = hosted_model_identity("openai", "frontier/default", "gpt-5.5", None);
        let route = ProviderRouteV1 {
            provider_kind: ProviderKindV1::Hosted,
            provider: "openai".to_string(),
            runtime_surface: RuntimeSurfaceV1::HostedApi,
            provider_model_id: "gpt-5.5".to_string(),
            endpoint_ref: None,
            credential_ref: None,
            source_registry: None,
        };
        let mut request = ProviderInvocationRequestV1 {
            route,
            model_identity: identity,
            prompt_contract_ref: "uts.v1".to_string(),
            lane_ref: "regular".to_string(),
            run_id: Some("run-test".to_string()),
            request_id: Some("req-test".to_string()),
            attempt_policy: ProviderAttemptPolicyV1 {
                max_attempts: 1,
                timeout_ms: 1000,
                retry_backoff_ms: Some(100),
            },
            input_text: Some("test prompt".to_string()),
            inference_parameter_fingerprint: None,
            tool_surface: None,
            governance_surface: None,
            evaluator_ref: None,
            benchmark_ref: None,
        };
        validate_provider_request(&request).unwrap();
        request.attempt_policy.max_attempts = 0;
        assert!(validate_provider_request(&request).is_err());
    }
}
