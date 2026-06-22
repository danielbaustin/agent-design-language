use crate::provider_communication::{
    hosted_model_identity, ollama_model_identity, provider_attempt_policy_as_resilience_policy,
    provider_failure_classification_from_failure, provider_failure_from_classification,
    provider_failure_from_note, validate_provider_request, ProviderAttemptPolicyV1,
    ProviderAttemptStatusV1, ProviderAttemptV1, ProviderFailureKindV1, ProviderFailureV1,
    ProviderInvocationFinalStatusV1, ProviderInvocationRequestV1, ProviderInvocationResultV1,
    ProviderRunLogEventV1, ProviderRunLoggerV1, RuntimeSurfaceV1,
    PROVIDER_COMMUNICATION_SCHEMA_VERSION,
};
use crate::resilience::{
    execute_retry_policy, execute_timeout_policy, ResilienceFaultClassV1, ResilienceSurfaceV1,
    RetryAttemptRecordV1, TimeoutBreachKindV1, TimeoutObservation,
};
use anyhow::{anyhow, Result};
use reqwest::blocking::Client;
use serde_json::{json, Value};
use std::cell::RefCell;
use std::collections::HashMap;
use std::env;
use std::sync::{Mutex, MutexGuard, OnceLock, TryLockError};
use std::time::{Duration, Instant};

const DEFAULT_OPENAI_RESPONSES_URL: &str = "https://api.openai.com/v1/responses";
const DEFAULT_ANTHROPIC_MESSAGES_URL: &str = "https://api.anthropic.com/v1/messages";
const DEFAULT_DEEPSEEK_CHAT_COMPLETIONS_URL: &str = "https://api.deepseek.com/chat/completions";
const DEFAULT_OPENROUTER_CHAT_COMPLETIONS_URL: &str =
    "https://openrouter.ai/api/v1/chat/completions";
const DEFAULT_OPENROUTER_MAX_TOKENS: u64 = 2048;
const DEFAULT_GEMINI_BASE_URL: &str = "https://generativelanguage.googleapis.com/v1beta";
const DEFAULT_OLLAMA_BASE_URL: &str = "http://127.0.0.1:11434";
static OLLAMA_RUNTIME_BULKHEADS: OnceLock<Mutex<HashMap<String, &'static Mutex<()>>>> =
    OnceLock::new();

pub fn execute_provider_invocation(
    mut request: ProviderInvocationRequestV1,
    logger: &mut ProviderRunLoggerV1,
) -> ProviderInvocationResultV1 {
    ensure_request_identity(&mut request);
    let started = Instant::now();

    if let Err(error) = validate_adapter_request(&request) {
        let failure = provider_failure_from_note(&error.to_string(), None);
        let _ = logger.event(
            event("run_finish", &request)
                .with_failure(&failure)
                .with_status("failed"),
        );
        let attempts = vec![preflight_failure_attempt(&request, &failure)];
        return failed_result(
            request,
            attempts,
            failure,
            started.elapsed().as_millis() as u64,
        );
    }

    let _ = logger.event(event("run_start", &request).with_status("started"));
    let policy = request.attempt_policy.clone();
    let resilience_policy =
        provider_attempt_policy_as_resilience_policy("provider_adapter.execute", &policy);
    let route = request.route.clone();
    let lane_ref = request.lane_ref.clone();
    let event_model_identity = request.model_identity.clone();
    let logger_cell = RefCell::new(&mut *logger);
    let execution = execute_retry_policy(
        &resilience_policy,
        ResilienceSurfaceV1::Provider,
        "provider_adapter.execute",
        |attempt_index| {
            let _ = logger_cell.borrow_mut().event(
                ProviderRunLogEventV1::new("attempt_start")
                    .with_route(&route, &event_model_identity)
                    .with_lane(&lane_ref)
                    .with_attempt(attempt_index),
            );
            let started = Instant::now();
            execute_timeout_policy(
                &resilience_policy,
                ResilienceSurfaceV1::Provider,
                "provider_adapter.attempt",
                || TimeoutObservation {
                    result: execute_runtime_surface_attempt(&mut request, &policy),
                    elapsed_ms: started.elapsed().as_millis() as u64,
                    cancelled: false,
                },
                provider_failure_classification_from_failure,
                timeout_failure,
                cancelled_failure,
            )
            .result
        },
        provider_failure_classification_from_failure,
        |delay_ms| std::thread::sleep(Duration::from_millis(delay_ms)),
        |attempt| {
            emit_retry_attempt_event(
                &mut logger_cell.borrow_mut(),
                &route,
                &lane_ref,
                &event_model_identity,
                attempt,
            )
        },
    );
    let duration_ms = started.elapsed().as_millis() as u64;

    match execution.result {
        Ok(provider_response) => {
            let output_text = provider_response.output_text.clone();
            let mut model_identity = request.model_identity.clone();
            if let Some(ref observed_provider_model_id) =
                provider_response.observed_provider_model_id
            {
                if !observed_provider_model_id.trim().is_empty() {
                    model_identity.provider_model_id = observed_provider_model_id.clone();
                }
            }
            let attempts = provider_attempts_from_trace(
                &execution.trace.attempts,
                Some(&provider_response),
                None,
            );
            let _ = logger_cell
                .borrow_mut()
                .event(event("run_finish", &request).with_status("ok"));
            ProviderInvocationResultV1 {
                schema_version: PROVIDER_COMMUNICATION_SCHEMA_VERSION.to_string(),
                route: request.route,
                model_identity,
                attempts,
                final_status: ProviderInvocationFinalStatusV1::Ok,
                duration_ms,
                request_id: request.request_id.clone(),
                output_text: Some(output_text.clone()),
                output_text_excerpt: Some(redacted_response_marker(&output_text)),
                failure: None,
                artifact_ref: None,
                trace_ref: None,
            }
        }
        Err(failure) => {
            let failure = normalize_terminal_failure(&execution.trace.attempts, &failure);
            let attempts =
                provider_attempts_from_trace(&execution.trace.attempts, None, Some(&failure));
            let _ = logger_cell.borrow_mut().event(
                event("run_finish", &request)
                    .with_failure(&failure)
                    .with_status("failed"),
            );
            failed_result(request, attempts, failure, duration_ms)
        }
    }
}

fn execute_runtime_surface_attempt(
    request: &mut ProviderInvocationRequestV1,
    policy: &ProviderAttemptPolicyV1,
) -> std::result::Result<ProviderTextResponse, ProviderFailureV1> {
    match request.route.runtime_surface {
        RuntimeSurfaceV1::HostedApi => execute_hosted(request, policy),
        RuntimeSurfaceV1::OllamaHttp => {
            let _guard = acquire_ollama_runtime_slot(request)?;
            execute_ollama_http(request, policy)
        }
        RuntimeSurfaceV1::OllamaCli | RuntimeSurfaceV1::Mock | RuntimeSurfaceV1::Unknown => Err(
            provider_failure_from_note("unsupported provider runtime surface", None),
        ),
    }
}

fn timeout_failure(
    breach_kind: TimeoutBreachKindV1,
    elapsed_ms: u64,
    breached_budget_ms: u64,
) -> ProviderFailureV1 {
    let breach_label = match breach_kind {
        TimeoutBreachKindV1::Timeout => "timeout",
        TimeoutBreachKindV1::HardDeadline => "hard_deadline",
    };
    provider_failure_from_note(
        &format!(
            "provider timeout after {elapsed_ms}ms ({breach_label} budget {breached_budget_ms}ms)"
        ),
        None,
    )
}

fn cancelled_failure(elapsed_ms: u64) -> ProviderFailureV1 {
    provider_failure_from_note(
        &format!("provider invocation cancelled after {elapsed_ms}ms"),
        None,
    )
}

fn ollama_runtime_bulkheads() -> &'static Mutex<HashMap<String, &'static Mutex<()>>> {
    OLLAMA_RUNTIME_BULKHEADS.get_or_init(|| Mutex::new(HashMap::new()))
}

fn acquire_ollama_runtime_slot(
    request: &ProviderInvocationRequestV1,
) -> std::result::Result<MutexGuard<'static, ()>, ProviderFailureV1> {
    let key = ollama_runtime_bulkhead_key(request);
    let slot = {
        let mut registry = ollama_runtime_bulkheads()
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        *registry
            .entry(key)
            .or_insert_with(|| Box::leak(Box::new(Mutex::new(()))))
    };
    match slot.try_lock() {
        Ok(guard) => Ok(guard),
        Err(TryLockError::WouldBlock) => Err(provider_failure_from_note(
            "local_runtime_busy: ollama runtime bulkhead saturated; retry later",
            None,
        )),
        Err(TryLockError::Poisoned(_)) => Err(provider_failure_from_note(
            "local_runtime_hung: ollama runtime bulkhead is poisoned; repair the local runtime before retrying",
            None,
        )),
    }
}

fn ollama_runtime_bulkhead_key(request: &ProviderInvocationRequestV1) -> String {
    format!(
        "{}::{}",
        ollama_runtime_base(request.route.endpoint_ref.as_deref()),
        request.route.provider_model_id
    )
}

fn ollama_runtime_base(endpoint_ref: Option<&str>) -> String {
    let base = endpoint_ref
        .filter(|endpoint| endpoint.starts_with("http://") || endpoint.starts_with("https://"))
        .unwrap_or(DEFAULT_OLLAMA_BASE_URL)
        .trim_end_matches('/');
    base.trim_end_matches("/api/generate")
        .trim_end_matches("/api/show")
        .trim_end_matches("/api/tags")
        .trim_end_matches('/')
        .to_string()
}

fn failed_result(
    request: ProviderInvocationRequestV1,
    attempts: Vec<ProviderAttemptV1>,
    failure: ProviderFailureV1,
    duration_ms: u64,
) -> ProviderInvocationResultV1 {
    ProviderInvocationResultV1 {
        schema_version: PROVIDER_COMMUNICATION_SCHEMA_VERSION.to_string(),
        route: request.route,
        model_identity: request.model_identity,
        attempts,
        final_status: ProviderInvocationFinalStatusV1::Failed,
        duration_ms,
        request_id: request.request_id,
        output_text: None,
        output_text_excerpt: None,
        failure: Some(failure),
        artifact_ref: None,
        trace_ref: None,
    }
}

fn preflight_failure_attempt(
    request: &ProviderInvocationRequestV1,
    failure: &ProviderFailureV1,
) -> ProviderAttemptV1 {
    ProviderAttemptV1 {
        attempt_index: 1,
        started_at: request.model_identity.observed_at.clone(),
        duration_ms: 0,
        status: ProviderAttemptStatusV1::Error,
        retryable: false,
        http_status: failure.http_status,
        failure: Some(failure.clone()),
        raw_response_excerpt: None,
    }
}

fn provider_attempts_from_trace(
    records: &[RetryAttemptRecordV1],
    success: Option<&ProviderTextResponse>,
    final_failure: Option<&ProviderFailureV1>,
) -> Vec<ProviderAttemptV1> {
    let success_attempt = success.map(|_| records.len() as u32);
    records
        .iter()
        .map(|record| {
            if let Some(classification) = &record.fault {
                ProviderAttemptV1 {
                    attempt_index: record.attempt_index,
                    started_at: record.started_at.clone(),
                    duration_ms: record.duration_ms,
                    status: if matches!(
                        classification.fault_class,
                        ResilienceFaultClassV1::ProviderTimeout
                    ) {
                        ProviderAttemptStatusV1::Timeout
                    } else {
                        ProviderAttemptStatusV1::Error
                    },
                    retryable: record.retry_allowed,
                    http_status: classification.http_status,
                    failure: provider_failure_from_attempt_record(record),
                    raw_response_excerpt: None,
                }
            } else {
                let output_text = if success_attempt == Some(record.attempt_index) {
                    success.map(|response| redacted_response_marker(&response.output_text))
                } else {
                    final_failure.map(|_| redacted_response_marker(""))
                };
                ProviderAttemptV1 {
                    attempt_index: record.attempt_index,
                    started_at: record.started_at.clone(),
                    duration_ms: record.duration_ms,
                    status: ProviderAttemptStatusV1::Ok,
                    retryable: false,
                    http_status: success.map(|response| response.http_status),
                    failure: None,
                    raw_response_excerpt: output_text,
                }
            }
        })
        .collect()
}

fn provider_failure_from_attempt_record(
    record: &RetryAttemptRecordV1,
) -> Option<ProviderFailureV1> {
    let classification = record.fault.as_ref()?;
    let mut failure = provider_failure_from_classification(classification);
    failure.retryable = record.retry_allowed;
    Some(failure)
}

fn normalize_terminal_failure(
    records: &[RetryAttemptRecordV1],
    failure: &ProviderFailureV1,
) -> ProviderFailureV1 {
    let mut normalized = failure.clone();
    if let Some(record) = records.last() {
        normalized.retryable = record.retry_allowed;
    }
    normalized
}

fn emit_retry_attempt_event(
    logger: &mut ProviderRunLoggerV1,
    route: &crate::provider_communication::ProviderRouteV1,
    lane_ref: &str,
    model_identity: &crate::model_identity::ModelIdentityV1,
    attempt: &RetryAttemptRecordV1,
) {
    match &attempt.fault {
        Some(_) => {
            let failure = provider_failure_from_attempt_record(attempt)
                .expect("attempt with fault must map to provider failure");
            let _ = logger.event(
                ProviderRunLogEventV1::new("attempt_failure")
                    .with_route(route, model_identity)
                    .with_lane(lane_ref)
                    .with_attempt(attempt.attempt_index)
                    .with_failure(&failure)
                    .with_status("failed"),
            );
        }
        None => {
            let _ = logger.event(
                ProviderRunLogEventV1::new("attempt_success")
                    .with_route(route, model_identity)
                    .with_lane(lane_ref)
                    .with_attempt(attempt.attempt_index)
                    .with_status("ok"),
            );
        }
    }
}

fn validate_adapter_request(request: &ProviderInvocationRequestV1) -> Result<()> {
    validate_provider_request(request)?;
    let input = request
        .input_text
        .as_deref()
        .ok_or_else(|| anyhow!("provider adapter request requires input_text"))?;
    if input.trim().is_empty() {
        return Err(anyhow!(
            "provider adapter request requires non-empty input_text"
        ));
    }
    Ok(())
}

fn ensure_request_identity(request: &mut ProviderInvocationRequestV1) {
    let expected_runtime = match request.route.runtime_surface {
        RuntimeSurfaceV1::HostedApi => "hosted_api",
        RuntimeSurfaceV1::OllamaHttp => "ollama_http",
        RuntimeSurfaceV1::OllamaCli => "ollama_cli",
        RuntimeSurfaceV1::Mock => "mock",
        RuntimeSurfaceV1::Unknown => "unknown",
    };
    if !request.model_identity.model_ref.trim().is_empty()
        && request.model_identity.runtime_surface == expected_runtime
    {
        return;
    }
    request.model_identity = match request.route.runtime_surface {
        RuntimeSurfaceV1::HostedApi => hosted_model_identity(
            request.route.provider.clone(),
            request.route.provider_model_id.clone(),
            request.route.provider_model_id.clone(),
            request.route.source_registry.clone(),
        ),
        RuntimeSurfaceV1::OllamaHttp => ollama_model_identity(
            request.route.provider_model_id.clone(),
            request.route.provider_model_id.clone(),
            None,
            request.route.source_registry.clone(),
        ),
        _ => request.model_identity.clone(),
    };
}

struct ProviderTextResponse {
    output_text: String,
    http_status: u16,
    observed_provider_model_id: Option<String>,
}

fn execute_hosted(
    request: &ProviderInvocationRequestV1,
    policy: &ProviderAttemptPolicyV1,
) -> std::result::Result<ProviderTextResponse, ProviderFailureV1> {
    match request.route.provider.to_ascii_lowercase().as_str() {
        "openai" | "chatgpt" => execute_hosted_openai(request, policy),
        "anthropic" | "claude" => execute_hosted_anthropic(request, policy),
        "deepseek" => execute_hosted_deepseek(request, policy),
        "openrouter" => execute_hosted_openrouter(request, policy),
        "google" | "gemini" => execute_hosted_gemini(request, policy),
        _ => Err(ProviderFailureV1 {
            kind: ProviderFailureKindV1::ProviderError,
            retryable: false,
            message: "unsupported hosted provider".to_string(),
            provider_error_excerpt: None,
            http_status: None,
        }),
    }
}

fn execute_hosted_openai(
    request: &ProviderInvocationRequestV1,
    policy: &ProviderAttemptPolicyV1,
) -> std::result::Result<ProviderTextResponse, ProviderFailureV1> {
    let key = resolve_credential(request.route.credential_ref.as_deref(), "OPENAI_API_KEY")?;
    let url = request
        .route
        .endpoint_ref
        .as_deref()
        .filter(|endpoint| endpoint.starts_with("http://") || endpoint.starts_with("https://"))
        .unwrap_or(DEFAULT_OPENAI_RESPONSES_URL);
    let response = client(policy)?
        .post(url)
        .bearer_auth(key)
        .json(&json!({
            "model": request.route.provider_model_id,
            "input": request.input_text.as_deref().unwrap_or_default(),
        }))
        .send()
        .map_err(map_reqwest_error)?;
    decode_text_response(
        response,
        extract_openai_output_text,
        |_| None,
        RuntimeSurfaceV1::HostedApi,
    )
}

fn execute_hosted_anthropic(
    request: &ProviderInvocationRequestV1,
    policy: &ProviderAttemptPolicyV1,
) -> std::result::Result<ProviderTextResponse, ProviderFailureV1> {
    let key = resolve_credential(request.route.credential_ref.as_deref(), "ANTHROPIC_API_KEY")?;
    let url = request
        .route
        .endpoint_ref
        .as_deref()
        .filter(|endpoint| endpoint.starts_with("http://") || endpoint.starts_with("https://"))
        .unwrap_or(DEFAULT_ANTHROPIC_MESSAGES_URL);
    let response = client(policy)?
        .post(url)
        .header("x-api-key", key)
        .header("anthropic-version", "2023-06-01")
        .json(&json!({
            "model": request.route.provider_model_id,
            "max_tokens": 256,
            "messages": [{
                "role": "user",
                "content": request.input_text.as_deref().unwrap_or_default(),
            }],
        }))
        .send()
        .map_err(map_reqwest_error)?;
    decode_text_response(
        response,
        extract_anthropic_output_text,
        |_| None,
        RuntimeSurfaceV1::HostedApi,
    )
}

fn execute_hosted_deepseek(
    request: &ProviderInvocationRequestV1,
    policy: &ProviderAttemptPolicyV1,
) -> std::result::Result<ProviderTextResponse, ProviderFailureV1> {
    let key = resolve_credential(request.route.credential_ref.as_deref(), "DEEPSEEK_API_KEY")?;
    let url = request
        .route
        .endpoint_ref
        .as_deref()
        .filter(|endpoint| endpoint.starts_with("http://") || endpoint.starts_with("https://"))
        .unwrap_or(DEFAULT_DEEPSEEK_CHAT_COMPLETIONS_URL);
    let response = client(policy)?
        .post(url)
        .bearer_auth(key)
        .json(&json!({
            "model": request.route.provider_model_id,
            "messages": [{
                "role": "user",
                "content": request.input_text.as_deref().unwrap_or_default(),
            }],
            "max_tokens": 256,
            "stream": false,
        }))
        .send()
        .map_err(map_reqwest_error)?;
    decode_text_response(
        response,
        extract_deepseek_output_text,
        |_| None,
        RuntimeSurfaceV1::HostedApi,
    )
}

fn execute_hosted_openrouter(
    request: &ProviderInvocationRequestV1,
    policy: &ProviderAttemptPolicyV1,
) -> std::result::Result<ProviderTextResponse, ProviderFailureV1> {
    let key = resolve_credential(
        request.route.credential_ref.as_deref(),
        "OPENROUTER_API_KEY",
    )?;
    let url = request
        .route
        .endpoint_ref
        .as_deref()
        .filter(|endpoint| endpoint.starts_with("http://") || endpoint.starts_with("https://"))
        .unwrap_or(DEFAULT_OPENROUTER_CHAT_COMPLETIONS_URL);
    let response = client(policy)?
        .post(url)
        .bearer_auth(key)
        .json(&json!({
            "model": request.route.provider_model_id,
            "messages": [{
                "role": "user",
                "content": request.input_text.as_deref().unwrap_or_default(),
            }],
            "max_tokens": DEFAULT_OPENROUTER_MAX_TOKENS,
            "stream": false,
        }))
        .send()
        .map_err(map_reqwest_error)?;
    decode_text_response(
        response,
        extract_chat_completion_output_text,
        extract_chat_completion_model_id,
        RuntimeSurfaceV1::HostedApi,
    )
}

fn execute_hosted_gemini(
    request: &ProviderInvocationRequestV1,
    policy: &ProviderAttemptPolicyV1,
) -> std::result::Result<ProviderTextResponse, ProviderFailureV1> {
    let key = resolve_credential(
        request.route.credential_ref.as_deref(),
        if env::var("GEMINI_API_KEY").is_ok() {
            "GEMINI_API_KEY"
        } else {
            "GOOGLE_API_KEY"
        },
    )?;
    let url = gemini_generate_url(
        request.route.endpoint_ref.as_deref(),
        &request.route.provider_model_id,
    );
    let response = client(policy)?
        .post(url)
        .header("x-goog-api-key", key)
        .json(&json!({
            "contents": [{
                "role": "user",
                "parts": [{"text": request.input_text.as_deref().unwrap_or_default()}],
            }],
        }))
        .send()
        .map_err(map_reqwest_error)?;
    decode_text_response(
        response,
        extract_gemini_output_text,
        |_| None,
        RuntimeSurfaceV1::HostedApi,
    )
}

fn execute_ollama_http(
    request: &mut ProviderInvocationRequestV1,
    policy: &ProviderAttemptPolicyV1,
) -> std::result::Result<ProviderTextResponse, ProviderFailureV1> {
    refresh_ollama_identity(request, policy);
    let response = client(policy)?
        .post(ollama_generate_url(request.route.endpoint_ref.as_deref()))
        .json(&json!({
            "model": request.route.provider_model_id,
            "prompt": request.input_text.as_deref().unwrap_or_default(),
            "stream": false,
            "think": false,
        }))
        .send()
        .map_err(map_reqwest_error)?;
    decode_text_response(
        response,
        |json| {
            json.get("response")
                .and_then(Value::as_str)
                .map(str::to_string)
        },
        |_| None,
        RuntimeSurfaceV1::OllamaHttp,
    )
}

fn decode_text_response(
    response: reqwest::blocking::Response,
    extractor: impl Fn(&Value) -> Option<String>,
    model_extractor: impl Fn(&Value) -> Option<String>,
    runtime_surface: RuntimeSurfaceV1,
) -> std::result::Result<ProviderTextResponse, ProviderFailureV1> {
    let status = response.status();
    let body = response.text().map_err(map_reqwest_error)?;
    if !status.is_success() {
        return Err(map_http_failure(status.as_u16(), &body, runtime_surface));
    }
    let json: Value = serde_json::from_str(&body).map_err(|error| {
        provider_failure_from_note(
            &format!("provider invalid_json: {error}"),
            Some(status.as_u16()),
        )
    })?;
    if let Some(failure) = provider_error_from_json_envelope(&json, status.as_u16()) {
        return Err(failure);
    }
    let output_text = extractor(&json)
        .filter(|text| !text.trim().is_empty())
        .ok_or_else(|| provider_output_failure_from_json(&json, status.as_u16()))?;
    Ok(ProviderTextResponse {
        output_text,
        http_status: status.as_u16(),
        observed_provider_model_id: model_extractor(&json),
    })
}

fn provider_error_from_json_envelope(json: &Value, http_status: u16) -> Option<ProviderFailureV1> {
    let error = json.get("error")?;
    let message = if let Some(message) = error.get("message").and_then(Value::as_str) {
        message.trim()
    } else if let Some(message) = error.as_str() {
        message.trim()
    } else {
        ""
    };
    if message.is_empty() {
        return None;
    }
    Some(provider_failure_from_note(message, Some(http_status)))
}

fn provider_output_failure_from_json(json: &Value, http_status: u16) -> ProviderFailureV1 {
    if chat_completion_reasoning_only(json) {
        return ProviderFailureV1 {
            kind: ProviderFailureKindV1::ProviderError,
            retryable: false,
            message: "provider returned reasoning-only output without final content".to_string(),
            provider_error_excerpt: Some(
                "provider returned reasoning-only output without final content".to_string(),
            ),
            http_status: Some(http_status),
        };
    }
    provider_failure_from_note("empty provider output", Some(http_status))
}

fn client(policy: &ProviderAttemptPolicyV1) -> std::result::Result<Client, ProviderFailureV1> {
    Client::builder()
        .timeout(Duration::from_millis(policy.timeout_ms))
        .build()
        .map_err(|error| provider_failure_from_note(&error.to_string(), None))
}

fn resolve_credential(
    credential_ref: Option<&str>,
    default_env: &str,
) -> std::result::Result<String, ProviderFailureV1> {
    let env_name = credential_ref
        .and_then(|credential| credential.strip_prefix("env:"))
        .unwrap_or(default_env);
    env::var(env_name).map_err(|_| ProviderFailureV1 {
        kind: ProviderFailureKindV1::ProviderAuthMissing,
        retryable: false,
        message: "missing provider credential".to_string(),
        provider_error_excerpt: None,
        http_status: None,
    })
}

fn gemini_generate_url(endpoint_ref: Option<&str>, model: &str) -> String {
    let encoded_model = model.trim_start_matches("models/");
    let base = endpoint_ref
        .filter(|endpoint| endpoint.starts_with("http://") || endpoint.starts_with("https://"))
        .unwrap_or(DEFAULT_GEMINI_BASE_URL)
        .trim_end_matches('/');
    if base.contains(":generateContent") {
        base.to_string()
    } else if base.ends_with(&format!("models/{encoded_model}")) {
        format!("{base}:generateContent")
    } else {
        format!("{base}/models/{encoded_model}:generateContent")
    }
}

fn ollama_generate_url(endpoint_ref: Option<&str>) -> String {
    if let Some(endpoint) = endpoint_ref {
        if endpoint.starts_with("http://") || endpoint.starts_with("https://") {
            let trimmed = endpoint.trim_end_matches('/');
            if trimmed.ends_with("/api/generate") {
                return trimmed.to_string();
            }
            return format!("{trimmed}/api/generate");
        }
    }
    let base = env::var("OLLAMA_HOST").unwrap_or_else(|_| DEFAULT_OLLAMA_BASE_URL.to_string());
    format!("{}/api/generate", base.trim_end_matches('/'))
}

fn map_reqwest_error(error: reqwest::Error) -> ProviderFailureV1 {
    if error.is_timeout() {
        provider_failure_from_note(
            "provider timeout",
            error.status().map(|status| status.as_u16()),
        )
    } else if error.is_connect() {
        provider_failure_from_note(
            "connection refused talking to local runtime",
            error.status().map(|status| status.as_u16()),
        )
    } else {
        provider_failure_from_note(
            &error.to_string(),
            error.status().map(|status| status.as_u16()),
        )
    }
}

fn map_http_failure(
    status: u16,
    body: &str,
    runtime_surface: RuntimeSurfaceV1,
) -> ProviderFailureV1 {
    let note = if runtime_surface == RuntimeSurfaceV1::OllamaHttp {
        let lower = body.to_ascii_lowercase();
        if lower.contains("not found") || lower.contains("does not exist") {
            format!("model not found: {body}")
        } else if lower.contains("stopping") || lower.contains("hung") {
            format!("local_runtime_hung: {body}")
        } else if matches!(status, 500..=599) {
            format!("local_runtime_busy http {status}: {body}")
        } else {
            format!("ollama http {status}: {body}")
        }
    } else {
        format!("provider http {status}: {body}")
    };
    provider_failure_from_note(&note, Some(status))
}

fn refresh_ollama_identity(
    request: &mut ProviderInvocationRequestV1,
    policy: &ProviderAttemptPolicyV1,
) {
    let digest =
        ollama_show_digest(request, policy).or_else(|| ollama_tags_digest(request, policy));
    if digest.is_some() {
        request.model_identity = ollama_model_identity(
            request.route.provider_model_id.clone(),
            request.route.provider_model_id.clone(),
            digest.as_deref(),
            request.route.source_registry.clone(),
        );
    }
}

fn ollama_show_digest(
    request: &ProviderInvocationRequestV1,
    policy: &ProviderAttemptPolicyV1,
) -> Option<String> {
    let response = client(policy)
        .ok()?
        .post(ollama_show_url(request.route.endpoint_ref.as_deref()))
        .json(&json!({"model": request.route.provider_model_id}))
        .send()
        .ok()?;
    if !response.status().is_success() {
        return None;
    }
    let json = serde_json::from_str::<Value>(&response.text().ok()?).ok()?;
    json.get("digest")
        .and_then(Value::as_str)
        .or_else(|| json.pointer("/details/digest").and_then(Value::as_str))
        .map(str::to_string)
}

fn ollama_tags_digest(
    request: &ProviderInvocationRequestV1,
    policy: &ProviderAttemptPolicyV1,
) -> Option<String> {
    let response = client(policy)
        .ok()?
        .get(ollama_tags_url(request.route.endpoint_ref.as_deref()))
        .send()
        .ok()?;
    if !response.status().is_success() {
        return None;
    }
    let json = serde_json::from_str::<Value>(&response.text().ok()?).ok()?;
    json.get("models")?
        .as_array()?
        .iter()
        .find(|model| {
            model.get("name").and_then(Value::as_str) == Some(&request.route.provider_model_id)
                || model.get("model").and_then(Value::as_str)
                    == Some(&request.route.provider_model_id)
        })
        .and_then(|model| model.get("digest").and_then(Value::as_str))
        .map(str::to_string)
}

fn ollama_tags_url(endpoint_ref: Option<&str>) -> String {
    if let Some(endpoint) = endpoint_ref {
        if endpoint.starts_with("http://") || endpoint.starts_with("https://") {
            let trimmed = endpoint.trim_end_matches('/');
            if trimmed.ends_with("/api/tags") {
                return trimmed.to_string();
            }
            if trimmed.ends_with("/api/generate") {
                return trimmed.trim_end_matches("/api/generate").to_string() + "/api/tags";
            }
            if trimmed.ends_with("/api/show") {
                return trimmed.trim_end_matches("/api/show").to_string() + "/api/tags";
            }
            return format!("{trimmed}/api/tags");
        }
    }
    let base = env::var("OLLAMA_HOST").unwrap_or_else(|_| DEFAULT_OLLAMA_BASE_URL.to_string());
    format!("{}/api/tags", base.trim_end_matches('/'))
}

fn ollama_show_url(endpoint_ref: Option<&str>) -> String {
    if let Some(endpoint) = endpoint_ref {
        if endpoint.starts_with("http://") || endpoint.starts_with("https://") {
            let trimmed = endpoint.trim_end_matches('/');
            if trimmed.ends_with("/api/show") {
                return trimmed.to_string();
            }
            if trimmed.ends_with("/api/generate") {
                return trimmed.trim_end_matches("/api/generate").to_string() + "/api/show";
            }
            return format!("{trimmed}/api/show");
        }
    }
    let base = env::var("OLLAMA_HOST").unwrap_or_else(|_| DEFAULT_OLLAMA_BASE_URL.to_string());
    format!("{}/api/show", base.trim_end_matches('/'))
}

fn extract_openai_output_text(json: &Value) -> Option<String> {
    if let Some(text) = json.get("output_text").and_then(Value::as_str) {
        if !text.trim().is_empty() {
            return Some(text.to_string());
        }
    }
    let mut chunks = Vec::new();
    if let Some(output) = json.get("output").and_then(Value::as_array) {
        for item in output {
            if let Some(content) = item.get("content").and_then(Value::as_array) {
                for part in content {
                    if let Some(text) = part.get("text").and_then(Value::as_str) {
                        if !text.trim().is_empty() {
                            chunks.push(text.to_string());
                        }
                    }
                }
            }
        }
    }
    (!chunks.is_empty()).then(|| chunks.join("\n"))
}

fn extract_anthropic_output_text(json: &Value) -> Option<String> {
    let mut chunks = Vec::new();
    if let Some(content) = json.get("content").and_then(Value::as_array) {
        for part in content {
            if part.get("type").and_then(Value::as_str) == Some("text") {
                if let Some(text) = part.get("text").and_then(Value::as_str) {
                    if !text.trim().is_empty() {
                        chunks.push(text.to_string());
                    }
                }
            }
        }
    }
    (!chunks.is_empty()).then(|| {
        chunks.join(
            "
",
        )
    })
}

fn extract_deepseek_output_text(json: &Value) -> Option<String> {
    extract_chat_completion_output_text(json)
}

fn extract_chat_completion_output_text(json: &Value) -> Option<String> {
    let content = json
        .get("choices")
        .and_then(Value::as_array)
        .and_then(|choices| choices.first())
        .and_then(|choice| choice.get("message"))
        .and_then(|message| message.get("content"))?;
    if let Some(text) = content.as_str().filter(|text| !text.trim().is_empty()) {
        return Some(text.to_string());
    }
    let mut chunks = Vec::new();
    if let Some(parts) = content.as_array() {
        for part in parts {
            if let Some(text) = part.get("text").and_then(Value::as_str) {
                if !text.trim().is_empty() {
                    chunks.push(text.to_string());
                }
            }
        }
    }
    (!chunks.is_empty()).then(|| chunks.join("\n"))
}

fn extract_chat_completion_model_id(json: &Value) -> Option<String> {
    json.get("model")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|text| !text.is_empty())
        .map(str::to_string)
}

fn chat_completion_reasoning_only(json: &Value) -> bool {
    json.get("choices")
        .and_then(Value::as_array)
        .and_then(|choices| choices.first())
        .and_then(|choice| choice.get("message"))
        .map(|message| {
            message.get("content").is_some_and(Value::is_null)
                && message
                    .get("reasoning")
                    .and_then(Value::as_str)
                    .is_some_and(|text| !text.trim().is_empty())
        })
        .unwrap_or(false)
}

fn extract_gemini_output_text(json: &Value) -> Option<String> {
    let mut chunks = Vec::new();
    if let Some(candidates) = json.get("candidates").and_then(Value::as_array) {
        for candidate in candidates {
            if let Some(parts) = candidate
                .get("content")
                .and_then(|content| content.get("parts"))
                .and_then(Value::as_array)
            {
                for part in parts {
                    if let Some(text) = part.get("text").and_then(Value::as_str) {
                        if !text.trim().is_empty() {
                            chunks.push(text.to_string());
                        }
                    }
                }
            }
        }
    }
    (!chunks.is_empty()).then(|| {
        chunks.join(
            "
",
        )
    })
}

fn redacted_response_marker(text: &str) -> String {
    format!("[redacted provider text len={}]", text.len())
}

fn event(event_type: &str, request: &ProviderInvocationRequestV1) -> ProviderRunLogEventV1 {
    ProviderRunLogEventV1::new(event_type)
        .with_route(&request.route, &request.model_identity)
        .with_lane(&request.lane_ref)
}

trait ProviderRunLogEventExt {
    fn with_attempt(self, attempt_index: u32) -> Self;
    fn with_status(self, status: &str) -> Self;
    fn with_failure(self, failure: &ProviderFailureV1) -> Self;
    fn with_lane(self, lane_ref: &str) -> Self;
}

impl ProviderRunLogEventExt for ProviderRunLogEventV1 {
    fn with_attempt(mut self, attempt_index: u32) -> Self {
        self.attempt_index = Some(attempt_index);
        self
    }

    fn with_status(mut self, status: &str) -> Self {
        self.status = Some(status.to_string());
        self
    }

    fn with_failure(mut self, failure: &ProviderFailureV1) -> Self {
        self.failure_kind = Some(failure.kind.clone());
        self.http_status_field(failure.http_status);
        self
    }

    fn with_lane(mut self, lane_ref: &str) -> Self {
        self.lane_ref = Some(lane_ref.to_string());
        self
    }
}

trait HttpStatusField {
    fn http_status_field(&mut self, status: Option<u16>);
}

impl HttpStatusField for ProviderRunLogEventV1 {
    fn http_status_field(&mut self, status: Option<u16>) {
        if let Some(status) = status {
            self.fields = Some(json!({"http_status": status}));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model_identity::ModelIdentityStrengthV1;
    use crate::provider_communication::{ProviderKindV1, ProviderRouteV1, RuntimeSurfaceV1};
    use std::fs;
    use std::io::{Read, Write};
    use std::net::TcpListener;
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::sync::mpsc;
    use std::thread;

    static TEMP_LOG_COUNTER: AtomicU64 = AtomicU64::new(0);

    fn temp_log(name: &str) -> PathBuf {
        let mut path = env::temp_dir();
        let unique = TEMP_LOG_COUNTER.fetch_add(1, Ordering::Relaxed);
        path.push(format!(
            "adl-provider-adapter-{name}-{}-{unique}.jsonl",
            std::process::id(),
        ));
        let _ = fs::remove_file(&path);
        path
    }

    fn request(
        runtime_surface: RuntimeSurfaceV1,
        endpoint_ref: String,
    ) -> ProviderInvocationRequestV1 {
        let route = ProviderRouteV1 {
            provider_kind: if runtime_surface == RuntimeSurfaceV1::HostedApi {
                ProviderKindV1::Hosted
            } else {
                ProviderKindV1::Local
            },
            provider: if runtime_surface == RuntimeSurfaceV1::HostedApi {
                "openai".to_string()
            } else {
                "ollama".to_string()
            },
            runtime_surface: runtime_surface.clone(),
            provider_model_id: "test-model".to_string(),
            endpoint_ref: Some(endpoint_ref),
            credential_ref: Some("env:ADL_PROVIDER_ADAPTER_TEST_KEY".to_string()),
            source_registry: Some("test-registry".to_string()),
        };
        let model_identity = if runtime_surface == RuntimeSurfaceV1::HostedApi {
            hosted_model_identity(
                "openai",
                "test-model",
                "test-model",
                Some("test".to_string()),
            )
        } else {
            let mut identity =
                ollama_model_identity("test-model", "test-model", None, Some("test".to_string()));
            identity.identity_strength = ModelIdentityStrengthV1::TagOnly;
            identity
        };
        ProviderInvocationRequestV1 {
            route,
            model_identity,
            prompt_contract_ref: "test.prompt.v1".to_string(),
            lane_ref: "regular".to_string(),
            run_id: Some("run-test".to_string()),
            request_id: Some("req-test".to_string()),
            attempt_policy: ProviderAttemptPolicyV1 {
                max_attempts: 1,
                timeout_ms: 5_000,
                retry_backoff_ms: Some(1),
            },
            input_text: Some("secret prompt should not enter logs".to_string()),
            inference_parameter_fingerprint: None,
            tool_surface: None,
            governance_surface: None,
            evaluator_ref: None,
            benchmark_ref: None,
        }
    }

    fn one_shot_server(body: &'static str, status: &'static str) -> String {
        scripted_server(vec![(body, status)])
    }

    fn capture_one_request_server(
        body: &'static str,
        status: &'static str,
    ) -> (String, mpsc::Receiver<String>) {
        let listener = TcpListener::bind("127.0.0.1:0").expect("bind server");
        let addr = listener.local_addr().expect("server addr");
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let (mut stream, _) = listener.accept().expect("accept");
            let mut buffer = [0_u8; 8192];
            let bytes_read = stream.read(&mut buffer).unwrap_or(0);
            let request = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();
            let _ = tx.send(request);
            let response = format!(
                "HTTP/1.1 {status}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                body.len()
            );
            stream
                .write_all(response.as_bytes())
                .expect("write response");
        });
        (format!("http://{addr}"), rx)
    }

    #[test]
    fn gemini_generate_url_does_not_embed_api_key() {
        let url = gemini_generate_url(
            Some("https://generativelanguage.googleapis.com/v1beta"),
            "models/gemini-test",
        );
        assert_eq!(
            url,
            "https://generativelanguage.googleapis.com/v1beta/models/gemini-test:generateContent"
        );
        assert!(!url.contains("key="));
    }

    #[test]
    fn hosted_gemini_adapter_sends_api_key_header_without_url_key() {
        env::set_var(
            "ADL_PROVIDER_ADAPTER_GEMINI_TEST_KEY",
            "synthetic-gemini-key",
        );
        let (endpoint, rx) = capture_one_request_server(
            r#"{"candidates":[{"content":{"parts":[{"text":"gemini ok"}]}}]}"#,
            "200 OK",
        );
        let path = temp_log("gemini");
        let mut logger = ProviderRunLoggerV1::create(&path, "run-gemini").expect("open logger");
        let mut req = request(RuntimeSurfaceV1::HostedApi, endpoint);
        req.route.provider = "gemini".to_string();
        req.route.provider_model_id = "gemini-test".to_string();
        req.route.credential_ref = Some("env:ADL_PROVIDER_ADAPTER_GEMINI_TEST_KEY".to_string());

        let result = execute_provider_invocation(req, &mut logger);
        drop(logger);

        assert_eq!(result.final_status, ProviderInvocationFinalStatusV1::Ok);
        assert_eq!(result.output_text.as_deref(), Some("gemini ok"));
        let raw_request = rx.recv().expect("captured request");
        assert!(raw_request
            .to_ascii_lowercase()
            .contains("x-goog-api-key: synthetic-gemini-key"));
        assert!(!raw_request
            .lines()
            .next()
            .unwrap_or_default()
            .contains("key="));
        let raw_log = fs::read_to_string(&path).unwrap();
        assert!(!raw_log.contains("synthetic-gemini-key"));
        env::remove_var("ADL_PROVIDER_ADAPTER_GEMINI_TEST_KEY");
        let _ = fs::remove_file(path);
    }

    fn scripted_server(responses: Vec<(&'static str, &'static str)>) -> String {
        let listener = TcpListener::bind("127.0.0.1:0").expect("bind server");
        let addr = listener.local_addr().expect("server addr");
        thread::spawn(move || {
            for (body, status) in responses {
                let (mut stream, _) = listener.accept().expect("accept");
                let mut buffer = [0_u8; 8192];
                let _ = stream.read(&mut buffer);
                let response = format!(
                    "HTTP/1.1 {status}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                    body.len()
                );
                stream
                    .write_all(response.as_bytes())
                    .expect("write response");
            }
        });
        format!("http://{addr}")
    }

    fn delayed_server(body: &'static str, status: &'static str, delay_ms: u64) -> String {
        let listener = TcpListener::bind("127.0.0.1:0").expect("bind delayed server");
        let addr = listener.local_addr().expect("server addr");
        thread::spawn(move || {
            let (mut stream, _) = listener.accept().expect("accept");
            let mut buffer = [0_u8; 8192];
            let _ = stream.read(&mut buffer);
            thread::sleep(Duration::from_millis(delay_ms));
            let response = format!(
                "HTTP/1.1 {status}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                body.len()
            );
            stream
                .write_all(response.as_bytes())
                .expect("write delayed response");
        });
        format!("http://{addr}")
    }

    #[test]
    fn hosted_openai_adapter_returns_output_and_redacts_log() {
        env::set_var("ADL_PROVIDER_ADAPTER_HOSTED_SUCCESS_KEY", "test-key");
        let endpoint = one_shot_server(r#"{"output_text":"adapter success"}"#, "200 OK");
        let path = temp_log("hosted");
        let mut logger = ProviderRunLoggerV1::create(&path, "run-test").expect("open logger");
        let mut req = request(RuntimeSurfaceV1::HostedApi, endpoint);
        req.route.credential_ref = Some("env:ADL_PROVIDER_ADAPTER_HOSTED_SUCCESS_KEY".to_string());
        let result = execute_provider_invocation(req, &mut logger);
        drop(logger);

        assert_eq!(result.final_status, ProviderInvocationFinalStatusV1::Ok);
        assert_eq!(result.request_id.as_deref(), Some("req-test"));
        assert_eq!(result.output_text.as_deref(), Some("adapter success"));
        let log = fs::read_to_string(&path).expect("read log");
        assert!(log.contains("attempt_success"));
        assert!(!log.contains("secret prompt"));
        assert!(!log.contains("adapter success"));
        let _ = fs::remove_file(path);
        env::remove_var("ADL_PROVIDER_ADAPTER_HOSTED_SUCCESS_KEY");
    }

    #[test]
    fn ollama_adapter_returns_output_and_redacts_log() {
        let endpoint = scripted_server(vec![
            (r#"{}"#, "200 OK"),
            (
                r#"{"models":[{"name":"test-model","digest":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"}]}"#,
                "200 OK",
            ),
            (r#"{"response":"ollama success"}"#, "200 OK"),
        ]);
        let path = temp_log("ollama");
        let mut logger = ProviderRunLoggerV1::create(&path, "run-test").expect("open logger");
        let result = execute_provider_invocation(
            request(RuntimeSurfaceV1::OllamaHttp, endpoint),
            &mut logger,
        );
        drop(logger);

        assert_eq!(result.final_status, ProviderInvocationFinalStatusV1::Ok);
        assert_eq!(result.output_text.as_deref(), Some("ollama success"));
        assert_eq!(
            result.model_identity.identity_strength,
            ModelIdentityStrengthV1::Pinned
        );
        let log = fs::read_to_string(&path).expect("read log");
        assert!(log.contains("attempt_success"));
        assert!(!log.contains("secret prompt"));
        assert!(!log.contains("ollama success"));
        let _ = fs::remove_file(path);
    }

    #[test]
    fn openai_output_array_shape_is_supported() {
        env::set_var("ADL_PROVIDER_ADAPTER_OPENAI_ARRAY_KEY", "test-key");
        let endpoint = one_shot_server(
            r#"{"output":[{"content":[{"text":"array success"}]}]}"#,
            "200 OK",
        );
        let path = temp_log("openai-array");
        let mut logger = ProviderRunLoggerV1::create(&path, "run-test").expect("open logger");
        let mut req = request(RuntimeSurfaceV1::HostedApi, endpoint);
        req.route.credential_ref = Some("env:ADL_PROVIDER_ADAPTER_OPENAI_ARRAY_KEY".to_string());
        let result = execute_provider_invocation(req, &mut logger);
        drop(logger);

        assert_eq!(result.final_status, ProviderInvocationFinalStatusV1::Ok);
        assert_eq!(result.output_text.as_deref(), Some("array success"));
        assert_eq!(
            result
                .attempts
                .first()
                .and_then(|attempt| attempt.http_status),
            Some(200)
        );
        let _ = fs::remove_file(path);
        env::remove_var("ADL_PROVIDER_ADAPTER_OPENAI_ARRAY_KEY");
    }

    #[test]
    fn hosted_openai_retries_retryable_failure_then_succeeds() {
        env::set_var("ADL_PROVIDER_ADAPTER_RETRY_KEY", "test-key");
        let endpoint = scripted_server(vec![
            (
                r#"{"error":"rate limit exceeded"}"#,
                "429 Too Many Requests",
            ),
            (r#"{"output_text":"retry success"}"#, "200 OK"),
        ]);
        let path = temp_log("retry-success");
        let mut logger = ProviderRunLoggerV1::create(&path, "run-test").expect("open logger");
        let mut req = request(RuntimeSurfaceV1::HostedApi, endpoint);
        req.route.credential_ref = Some("env:ADL_PROVIDER_ADAPTER_RETRY_KEY".to_string());
        req.attempt_policy.max_attempts = 2;
        req.attempt_policy.retry_backoff_ms = Some(1);

        let result = execute_provider_invocation(req, &mut logger);
        drop(logger);

        assert_eq!(result.final_status, ProviderInvocationFinalStatusV1::Ok);
        assert_eq!(result.output_text.as_deref(), Some("retry success"));
        assert_eq!(result.attempts.len(), 2);
        assert_eq!(
            result.attempts[0]
                .failure
                .as_ref()
                .map(|failure| failure.kind.clone()),
            Some(ProviderFailureKindV1::ProviderRateLimited)
        );
        assert!(result.attempts[0].retryable);
        assert_eq!(
            result.attempts[0]
                .failure
                .as_ref()
                .map(|failure| failure.retryable),
            Some(true)
        );
        assert_eq!(result.attempts[1].status, ProviderAttemptStatusV1::Ok);

        let _ = fs::remove_file(path);
        env::remove_var("ADL_PROVIDER_ADAPTER_RETRY_KEY");
    }

    #[test]
    fn hosted_openai_retry_flow_emits_reviewable_log_sequence() {
        env::set_var("ADL_PROVIDER_ADAPTER_RETRY_LOG_KEY", "test-key");
        let endpoint = scripted_server(vec![
            (
                r#"{"error":"rate limit exceeded"}"#,
                "429 Too Many Requests",
            ),
            (r#"{"output_text":"retry success"}"#, "200 OK"),
        ]);
        let path = temp_log("retry-log-sequence");
        let mut logger = ProviderRunLoggerV1::create(&path, "run-test").expect("open logger");
        let mut req = request(RuntimeSurfaceV1::HostedApi, endpoint);
        req.route.credential_ref = Some("env:ADL_PROVIDER_ADAPTER_RETRY_LOG_KEY".to_string());
        req.attempt_policy.max_attempts = 2;
        req.attempt_policy.retry_backoff_ms = Some(1);

        let result = execute_provider_invocation(req, &mut logger);
        drop(logger);

        assert_eq!(result.final_status, ProviderInvocationFinalStatusV1::Ok);
        let log = fs::read_to_string(&path).expect("read log");
        let run_start = log.find("run_start").expect("run_start log");
        let first_attempt_start = log.find("attempt_start").expect("attempt_start log");
        let attempt_failure = log.find("attempt_failure").expect("attempt_failure log");
        let second_attempt_start = log[first_attempt_start + 1..]
            .find("attempt_start")
            .map(|offset| first_attempt_start + 1 + offset)
            .expect("second attempt_start log");
        let attempt_success = log.find("attempt_success").expect("attempt_success log");
        let run_finish = log.find("run_finish").expect("run_finish log");

        assert!(run_start < first_attempt_start);
        assert!(first_attempt_start < attempt_failure);
        assert!(attempt_failure < second_attempt_start);
        assert!(second_attempt_start < attempt_success);
        assert!(attempt_success < run_finish);
        assert!(log.contains("provider_rate_limited"));
        assert!(!log.contains("secret prompt"));
        assert!(!log.contains("retry success"));

        let _ = fs::remove_file(path);
        env::remove_var("ADL_PROVIDER_ADAPTER_RETRY_LOG_KEY");
    }

    #[test]
    fn hosted_openai_stops_after_non_retryable_failure() {
        env::set_var("ADL_PROVIDER_ADAPTER_NONRETRY_KEY", "test-key");
        let endpoint = scripted_server(vec![
            (r#"{"error":"unauthorized"}"#, "401 Unauthorized"),
            (r#"{"output_text":"should not be reached"}"#, "200 OK"),
        ]);
        let path = temp_log("nonretryable");
        let mut logger = ProviderRunLoggerV1::create(&path, "run-test").expect("open logger");
        let mut req = request(RuntimeSurfaceV1::HostedApi, endpoint);
        req.route.credential_ref = Some("env:ADL_PROVIDER_ADAPTER_NONRETRY_KEY".to_string());
        req.attempt_policy.max_attempts = 3;
        req.attempt_policy.retry_backoff_ms = Some(1);

        let result = execute_provider_invocation(req, &mut logger);
        drop(logger);

        assert_eq!(result.final_status, ProviderInvocationFinalStatusV1::Failed);
        assert_eq!(result.attempts.len(), 1);
        assert_eq!(
            result.failure.as_ref().map(|failure| failure.kind.clone()),
            Some(ProviderFailureKindV1::ProviderAuthError)
        );
        assert!(!result.attempts[0].retryable);
        assert_eq!(
            result.attempts[0]
                .failure
                .as_ref()
                .map(|failure| failure.retryable),
            Some(false)
        );
        assert_eq!(
            result.failure.as_ref().map(|failure| failure.retryable),
            Some(false)
        );

        let _ = fs::remove_file(path);
        env::remove_var("ADL_PROVIDER_ADAPTER_NONRETRY_KEY");
    }

    #[test]
    fn hosted_openai_reports_retry_budget_exhaustion_after_last_retryable_failure() {
        env::set_var("ADL_PROVIDER_ADAPTER_EXHAUST_KEY", "test-key");
        let endpoint = scripted_server(vec![
            (
                r#"{"error":"server overloaded"}"#,
                "503 Service Unavailable",
            ),
            (
                r#"{"error":"server overloaded"}"#,
                "503 Service Unavailable",
            ),
        ]);
        let path = temp_log("retry-exhausted");
        let mut logger = ProviderRunLoggerV1::create(&path, "run-test").expect("open logger");
        let mut req = request(RuntimeSurfaceV1::HostedApi, endpoint);
        req.route.credential_ref = Some("env:ADL_PROVIDER_ADAPTER_EXHAUST_KEY".to_string());
        req.attempt_policy.max_attempts = 2;
        req.attempt_policy.retry_backoff_ms = Some(1);

        let result = execute_provider_invocation(req, &mut logger);
        drop(logger);

        assert_eq!(result.final_status, ProviderInvocationFinalStatusV1::Failed);
        assert_eq!(result.attempts.len(), 2);
        assert_eq!(
            result.failure.as_ref().map(|failure| failure.kind.clone()),
            Some(ProviderFailureKindV1::ProviderTransientHttp)
        );
        assert!(result.attempts[0].retryable);
        assert!(!result.attempts[1].retryable);
        assert_eq!(
            result.attempts[0]
                .failure
                .as_ref()
                .map(|failure| failure.retryable),
            Some(true)
        );
        assert_eq!(
            result.attempts[1]
                .failure
                .as_ref()
                .map(|failure| failure.retryable),
            Some(false)
        );
        assert_eq!(
            result.failure.as_ref().map(|failure| failure.retryable),
            Some(false)
        );

        let _ = fs::remove_file(path);
        env::remove_var("ADL_PROVIDER_ADAPTER_EXHAUST_KEY");
    }

    #[test]
    fn hosted_openai_timeout_is_reported_through_shared_timeout_layer() {
        env::set_var("ADL_PROVIDER_ADAPTER_TIMEOUT_DIRECT_KEY", "test-key");
        let endpoint = delayed_server(r#"{"output_text":"too slow"}"#, "200 OK", 80);
        let path = temp_log("timeout");
        let mut logger = ProviderRunLoggerV1::create(&path, "run-timeout").expect("open logger");
        let mut req = request(RuntimeSurfaceV1::HostedApi, endpoint);
        req.route.credential_ref = Some("env:ADL_PROVIDER_ADAPTER_TIMEOUT_DIRECT_KEY".to_string());
        req.attempt_policy.timeout_ms = 10;

        let result = execute_provider_invocation(req, &mut logger);
        drop(logger);

        assert_eq!(result.final_status, ProviderInvocationFinalStatusV1::Failed);
        assert_eq!(result.attempts.len(), 1);
        assert_eq!(result.attempts[0].status, ProviderAttemptStatusV1::Timeout);
        assert_eq!(
            result.failure.as_ref().map(|failure| failure.kind.clone()),
            Some(ProviderFailureKindV1::ProviderTimeout)
        );
        assert_eq!(
            result.attempts[0]
                .failure
                .as_ref()
                .map(|failure| failure.kind.clone()),
            Some(ProviderFailureKindV1::ProviderTimeout)
        );

        let _ = fs::remove_file(path);
        env::remove_var("ADL_PROVIDER_ADAPTER_TIMEOUT_DIRECT_KEY");
    }

    #[test]
    fn ollama_bulkhead_saturation_is_reported_as_local_runtime_busy() {
        let held = request(
            RuntimeSurfaceV1::OllamaHttp,
            "http://127.0.0.1:9".to_string(),
        );
        let _guard = acquire_ollama_runtime_slot(&held).expect("hold local runtime slot");
        let path = temp_log("ollama-bulkhead");
        let mut logger = ProviderRunLoggerV1::create(&path, "run-bulkhead").expect("open logger");
        let mut req = request(
            RuntimeSurfaceV1::OllamaHttp,
            "http://127.0.0.1:9".to_string(),
        );
        req.attempt_policy.max_attempts = 1;

        let result = execute_provider_invocation(req, &mut logger);
        drop(logger);

        assert_eq!(result.final_status, ProviderInvocationFinalStatusV1::Failed);
        assert_eq!(result.attempts.len(), 1);
        assert_eq!(
            result.failure.as_ref().map(|failure| failure.kind.clone()),
            Some(ProviderFailureKindV1::LocalRuntimeBusy)
        );
        assert_eq!(
            result.attempts[0]
                .failure
                .as_ref()
                .map(|failure| failure.kind.clone()),
            Some(ProviderFailureKindV1::LocalRuntimeBusy)
        );
        assert!(!result.attempts[0].retryable);
        assert_eq!(
            result.attempts[0]
                .failure
                .as_ref()
                .map(|failure| failure.retryable),
            Some(false)
        );

        let _ = fs::remove_file(path);
    }

    #[test]
    fn ollama_bulkhead_is_scoped_per_endpoint_and_model() {
        let first = request(
            RuntimeSurfaceV1::OllamaHttp,
            "http://127.0.0.1:11434".to_string(),
        );
        let second = request(
            RuntimeSurfaceV1::OllamaHttp,
            "http://127.0.0.1:22434/api/generate".to_string(),
        );
        let third = {
            let mut req = request(
                RuntimeSurfaceV1::OllamaHttp,
                "http://127.0.0.1:11434".to_string(),
            );
            req.route.provider_model_id = "different-model".to_string();
            req
        };

        let _guard = acquire_ollama_runtime_slot(&first).expect("hold first slot");
        let _other_endpoint =
            acquire_ollama_runtime_slot(&second).expect("different endpoint should not block");
        let _other_model =
            acquire_ollama_runtime_slot(&third).expect("different model should not block");
    }

    #[test]
    fn ollama_missing_model_is_not_reported_as_busy() {
        let endpoint = scripted_server(vec![
            (r#"{}"#, "200 OK"),
            (r#"{"models":[]}"#, "200 OK"),
            (r#"{"error":"model test-model not found"}"#, "404 Not Found"),
        ]);
        let path = temp_log("ollama-missing-model");
        let mut logger = ProviderRunLoggerV1::create(&path, "run-test").expect("open logger");
        let result = execute_provider_invocation(
            request(RuntimeSurfaceV1::OllamaHttp, endpoint),
            &mut logger,
        );
        drop(logger);

        assert_eq!(result.final_status, ProviderInvocationFinalStatusV1::Failed);
        assert_eq!(
            result.failure.as_ref().map(|failure| failure.kind.clone()),
            Some(ProviderFailureKindV1::ProviderModelUnavailable)
        );
        let _ = fs::remove_file(path);
    }

    #[test]
    fn claude_hosted_adapter_returns_output_and_redacts_log() {
        env::set_var("ADL_PROVIDER_ADAPTER_CLAUDE_KEY", "test-key");
        let endpoint = one_shot_server(
            r#"{"content":[{"type":"text","text":"claude success"}]}"#,
            "200 OK",
        );
        let path = temp_log("claude");
        let mut logger = ProviderRunLoggerV1::create(&path, "run-test").expect("open logger");
        let mut req = request(RuntimeSurfaceV1::HostedApi, endpoint);
        req.route.provider = "anthropic".to_string();
        req.route.credential_ref = Some("env:ADL_PROVIDER_ADAPTER_CLAUDE_KEY".to_string());
        req.model_identity = hosted_model_identity(
            "anthropic",
            "claude-test",
            "test-model",
            Some("test".to_string()),
        );
        let result = execute_provider_invocation(req, &mut logger);
        drop(logger);

        assert_eq!(result.final_status, ProviderInvocationFinalStatusV1::Ok);
        assert_eq!(result.output_text.as_deref(), Some("claude success"));
        let log = fs::read_to_string(&path).expect("read log");
        assert!(log.contains("attempt_success"));
        assert!(!log.contains("secret prompt"));
        assert!(!log.contains("claude success"));
        let _ = fs::remove_file(path);
        env::remove_var("ADL_PROVIDER_ADAPTER_CLAUDE_KEY");
    }

    #[test]
    fn deepseek_hosted_adapter_returns_output_and_redacts_log() {
        env::set_var("ADL_PROVIDER_ADAPTER_DEEPSEEK_KEY", "test-key");
        let endpoint = one_shot_server(
            r#"{"choices":[{"message":{"content":"deepseek success"}}]}"#,
            "200 OK",
        );
        let path = temp_log("deepseek");
        let mut logger = ProviderRunLoggerV1::create(&path, "run-test").expect("open logger");
        let mut req = request(RuntimeSurfaceV1::HostedApi, endpoint);
        req.route.provider = "deepseek".to_string();
        req.route.credential_ref = Some("env:ADL_PROVIDER_ADAPTER_DEEPSEEK_KEY".to_string());
        req.model_identity = hosted_model_identity(
            "deepseek",
            "deepseek-test",
            "test-model",
            Some("test".to_string()),
        );
        let result = execute_provider_invocation(req, &mut logger);
        drop(logger);

        assert_eq!(result.final_status, ProviderInvocationFinalStatusV1::Ok);
        assert_eq!(result.output_text.as_deref(), Some("deepseek success"));
        let log = fs::read_to_string(&path).expect("read log");
        assert!(log.contains("attempt_success"));
        assert!(!log.contains("secret prompt"));
        assert!(!log.contains("deepseek success"));
        let _ = fs::remove_file(path);
        env::remove_var("ADL_PROVIDER_ADAPTER_DEEPSEEK_KEY");
    }

    #[test]
    fn openrouter_hosted_adapter_returns_output_and_redacts_log() {
        env::set_var("ADL_PROVIDER_ADAPTER_OPENROUTER_SUCCESS_KEY", "test-key");
        let (endpoint, rx) = capture_one_request_server(
            r#"{"model":"anthropic/claude-3.5-haiku","choices":[{"message":{"content":"openrouter success"}}]}"#,
            "200 OK",
        );
        let path = temp_log("openrouter");
        let mut logger = ProviderRunLoggerV1::create(&path, "run-test").expect("open logger");
        let mut req = request(RuntimeSurfaceV1::HostedApi, endpoint);
        req.route.provider = "openrouter".to_string();
        req.route.provider_model_id = "anthropic/claude-3.5-haiku".to_string();
        req.route.credential_ref =
            Some("env:ADL_PROVIDER_ADAPTER_OPENROUTER_SUCCESS_KEY".to_string());
        req.model_identity = hosted_model_identity(
            "openrouter",
            "anthropic/claude-3.5-haiku",
            "reviewer/fast",
            Some("test".to_string()),
        );
        let result = execute_provider_invocation(req, &mut logger);
        drop(logger);

        assert_eq!(result.final_status, ProviderInvocationFinalStatusV1::Ok);
        assert_eq!(result.output_text.as_deref(), Some("openrouter success"));
        assert_eq!(
            result.model_identity.provider_model_id,
            "anthropic/claude-3.5-haiku"
        );
        let raw_request = rx.recv().expect("captured request");
        assert!(raw_request
            .to_ascii_lowercase()
            .contains("authorization: bearer test-key"));
        assert!(raw_request.contains(r#""model":"anthropic/claude-3.5-haiku""#));
        let log = fs::read_to_string(&path).expect("read log");
        assert!(log.contains("attempt_success"));
        assert!(!log.contains("secret prompt"));
        assert!(!log.contains("openrouter success"));
        let _ = fs::remove_file(path);
        env::remove_var("ADL_PROVIDER_ADAPTER_OPENROUTER_SUCCESS_KEY");
    }

    #[test]
    fn openrouter_hosted_adapter_supports_array_content_shape() {
        env::set_var("ADL_PROVIDER_ADAPTER_OPENROUTER_ARRAY_KEY", "test-key");
        let endpoint = one_shot_server(
            r#"{"model":"moonshotai/kimi-k2.7-code","choices":[{"message":{"content":[{"type":"text","text":"array success"}]}}]}"#,
            "200 OK",
        );
        let path = temp_log("openrouter-array");
        let mut logger = ProviderRunLoggerV1::create(&path, "run-test").expect("open logger");
        let mut req = request(RuntimeSurfaceV1::HostedApi, endpoint);
        req.route.provider = "openrouter".to_string();
        req.route.provider_model_id = "moonshotai/kimi-k2.7-code".to_string();
        req.route.credential_ref =
            Some("env:ADL_PROVIDER_ADAPTER_OPENROUTER_ARRAY_KEY".to_string());
        req.model_identity = hosted_model_identity(
            "openrouter",
            "moonshotai/kimi-k2.7-code",
            "reviewer/fast",
            Some("test".to_string()),
        );
        let result = execute_provider_invocation(req, &mut logger);
        drop(logger);

        assert_eq!(result.final_status, ProviderInvocationFinalStatusV1::Ok);
        assert_eq!(result.output_text.as_deref(), Some("array success"));
        assert_eq!(
            result.model_identity.provider_model_id,
            "moonshotai/kimi-k2.7-code"
        );

        let _ = fs::remove_file(path);
        env::remove_var("ADL_PROVIDER_ADAPTER_OPENROUTER_ARRAY_KEY");
    }

    #[test]
    fn openrouter_hosted_adapter_surfaces_error_envelope_on_success_status() {
        env::set_var("ADL_PROVIDER_ADAPTER_OPENROUTER_ERROR_KEY", "test-key");
        let endpoint = one_shot_server(
            r#"{"error":{"message":"Claude Fable 5 is not available","code":404}}"#,
            "200 OK",
        );
        let path = temp_log("openrouter-error-envelope");
        let mut logger = ProviderRunLoggerV1::create(&path, "run-test").expect("open logger");
        let mut req = request(RuntimeSurfaceV1::HostedApi, endpoint);
        req.route.provider = "openrouter".to_string();
        req.route.provider_model_id = "anthropic/claude-fable-5".to_string();
        req.route.credential_ref =
            Some("env:ADL_PROVIDER_ADAPTER_OPENROUTER_ERROR_KEY".to_string());
        req.model_identity = hosted_model_identity(
            "openrouter",
            "anthropic/claude-fable-5",
            "reviewer/fast",
            Some("test".to_string()),
        );

        let result = execute_provider_invocation(req, &mut logger);
        drop(logger);

        assert_eq!(result.final_status, ProviderInvocationFinalStatusV1::Failed);
        assert_eq!(
            result.failure.as_ref().map(|failure| failure.kind.clone()),
            Some(ProviderFailureKindV1::ProviderError)
        );
        assert!(result
            .failure
            .as_ref()
            .is_some_and(|failure| failure.message.contains("not available")));

        let _ = fs::remove_file(path);
        env::remove_var("ADL_PROVIDER_ADAPTER_OPENROUTER_ERROR_KEY");
    }

    #[test]
    fn openrouter_hosted_adapter_classifies_reasoning_only_output() {
        env::set_var("ADL_PROVIDER_ADAPTER_OPENROUTER_REASONING_KEY", "test-key");
        let endpoint = one_shot_server(
            r#"{"model":"moonshotai/kimi-k2.7-code","choices":[{"message":{"content":null,"reasoning":"spent all budget thinking"}}]}"#,
            "200 OK",
        );
        let path = temp_log("openrouter-reasoning-only");
        let mut logger = ProviderRunLoggerV1::create(&path, "run-test").expect("open logger");
        let mut req = request(RuntimeSurfaceV1::HostedApi, endpoint);
        req.route.provider = "openrouter".to_string();
        req.route.provider_model_id = "moonshotai/kimi-k2.7-code".to_string();
        req.route.credential_ref =
            Some("env:ADL_PROVIDER_ADAPTER_OPENROUTER_REASONING_KEY".to_string());
        req.model_identity = hosted_model_identity(
            "openrouter",
            "moonshotai/kimi-k2.7-code",
            "reviewer/fast",
            Some("test".to_string()),
        );

        let result = execute_provider_invocation(req, &mut logger);
        drop(logger);

        assert_eq!(result.final_status, ProviderInvocationFinalStatusV1::Failed);
        assert_eq!(
            result.failure.as_ref().map(|failure| failure.kind.clone()),
            Some(ProviderFailureKindV1::ProviderError)
        );
        assert!(result.failure.as_ref().is_some_and(|failure| {
            failure
                .message
                .contains("reasoning-only output without final content")
        }));

        let _ = fs::remove_file(path);
        env::remove_var("ADL_PROVIDER_ADAPTER_OPENROUTER_REASONING_KEY");
    }

    #[test]
    fn openrouter_hosted_adapter_prefers_observed_provider_model_identity() {
        env::set_var("ADL_PROVIDER_ADAPTER_OPENROUTER_OBSERVED_KEY", "test-key");
        let endpoint = one_shot_server(
            r#"{"model":"qwen/qwen3.6-flash","choices":[{"message":{"content":"identity success"}}]}"#,
            "200 OK",
        );
        let path = temp_log("openrouter-observed-model");
        let mut logger = ProviderRunLoggerV1::create(&path, "run-test").expect("open logger");
        let mut req = request(RuntimeSurfaceV1::HostedApi, endpoint);
        req.route.provider = "openrouter".to_string();
        req.route.provider_model_id = "reviewer/fast".to_string();
        req.route.credential_ref =
            Some("env:ADL_PROVIDER_ADAPTER_OPENROUTER_OBSERVED_KEY".to_string());
        req.model_identity = hosted_model_identity(
            "openrouter",
            "reviewer/fast",
            "reviewer/fast",
            Some("test".to_string()),
        );

        let result = execute_provider_invocation(req, &mut logger);
        drop(logger);

        assert_eq!(result.final_status, ProviderInvocationFinalStatusV1::Ok);
        assert_eq!(result.output_text.as_deref(), Some("identity success"));
        assert_eq!(
            result.model_identity.provider_model_id,
            "qwen/qwen3.6-flash"
        );

        let _ = fs::remove_file(path);
        env::remove_var("ADL_PROVIDER_ADAPTER_OPENROUTER_OBSERVED_KEY");
    }

    #[test]
    fn gemini_hosted_adapter_returns_output_and_redacts_log() {
        env::set_var("ADL_PROVIDER_ADAPTER_GEMINI_KEY", "test-key");
        let endpoint = one_shot_server(
            r#"{"candidates":[{"content":{"parts":[{"text":"gemini success"}]}}]}"#,
            "200 OK",
        );
        let path = temp_log("gemini");
        let mut logger = ProviderRunLoggerV1::create(&path, "run-test").expect("open logger");
        let mut req = request(RuntimeSurfaceV1::HostedApi, endpoint);
        req.route.provider = "gemini".to_string();
        req.route.credential_ref = Some("env:ADL_PROVIDER_ADAPTER_GEMINI_KEY".to_string());
        req.model_identity = hosted_model_identity(
            "gemini",
            "gemini-test",
            "test-model",
            Some("test".to_string()),
        );
        let result = execute_provider_invocation(req, &mut logger);
        drop(logger);

        assert_eq!(result.final_status, ProviderInvocationFinalStatusV1::Ok);
        assert_eq!(result.output_text.as_deref(), Some("gemini success"));
        let log = fs::read_to_string(&path).expect("read log");
        assert!(log.contains("attempt_success"));
        assert!(!log.contains("secret prompt"));
        assert!(!log.contains("gemini success"));
        let _ = fs::remove_file(path);
        env::remove_var("ADL_PROVIDER_ADAPTER_GEMINI_KEY");
    }

    #[test]
    fn unsupported_hosted_provider_is_rejected_not_misrouted() {
        let endpoint = one_shot_server(r#"{"output_text":"should not happen"}"#, "200 OK");
        let path = temp_log("unsupported-hosted");
        let mut logger = ProviderRunLoggerV1::create(&path, "run-test").expect("open logger");
        let mut req = request(RuntimeSurfaceV1::HostedApi, endpoint);
        req.route.provider = "unknown-hosted".to_string();
        let result = execute_provider_invocation(req, &mut logger);
        drop(logger);

        assert_eq!(result.final_status, ProviderInvocationFinalStatusV1::Failed);
        assert_eq!(
            result.failure.as_ref().map(|failure| failure.kind.clone()),
            Some(ProviderFailureKindV1::ProviderError)
        );
        let _ = fs::remove_file(path);
    }

    #[test]
    fn missing_hosted_key_is_normalized_without_network_call() {
        env::remove_var("ADL_PROVIDER_ADAPTER_TEST_KEY");
        let (tx, rx) = mpsc::channel();
        let listener = TcpListener::bind("127.0.0.1:0").expect("bind server");
        let addr = listener.local_addr().expect("server addr");
        thread::spawn(move || {
            if listener.accept().is_ok() {
                let _ = tx.send(());
            }
        });
        let path = temp_log("missing-key");
        let mut logger = ProviderRunLoggerV1::create(&path, "run-test").expect("open logger");
        let mut missing_request = request(RuntimeSurfaceV1::HostedApi, format!("http://{addr}"));
        missing_request.route.credential_ref =
            Some("env:ADL_PROVIDER_ADAPTER_MISSING_KEY".to_string());
        let result = execute_provider_invocation(missing_request, &mut logger);
        drop(logger);

        assert_eq!(result.final_status, ProviderInvocationFinalStatusV1::Failed);
        assert_eq!(result.request_id.as_deref(), Some("req-test"));
        assert_eq!(
            result.failure.as_ref().map(|failure| failure.kind.clone()),
            Some(ProviderFailureKindV1::ProviderAuthMissing)
        );
        assert!(rx.try_recv().is_err());
        let _ = fs::remove_file(path);
    }

    #[test]
    fn preflight_request_validation_failure_emits_valid_failed_result_shape() {
        let path = temp_log("preflight-validation");
        let mut logger = ProviderRunLoggerV1::create(&path, "run-test").expect("open logger");
        let mut invalid_request = request(
            RuntimeSurfaceV1::HostedApi,
            "http://127.0.0.1:1".to_string(),
        );
        invalid_request.input_text = None;

        let result = execute_provider_invocation(invalid_request, &mut logger);
        drop(logger);

        assert_eq!(result.final_status, ProviderInvocationFinalStatusV1::Failed);
        assert_eq!(result.attempts.len(), 1);
        assert_eq!(result.attempts[0].status, ProviderAttemptStatusV1::Error);
        assert!(result.attempts[0].failure.is_some());
        crate::provider_communication::validate_provider_result(&result)
            .expect("preflight failure result stays contract-valid");

        let _ = fs::remove_file(path);
    }
}
