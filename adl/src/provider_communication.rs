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
use crate::resilience::{
    sanitize_resilience_summary, ResilienceFaultClassV1, ResilienceFaultClassificationV1,
    ResilienceFaultDispositionV1, ResiliencePolicyV1,
};

pub const PROVIDER_COMMUNICATION_SCHEMA_VERSION: &str = "provider_communication.v1";
pub const REVIEW_PROVIDER_AUTHORITY_BOUNDARY_V1: &str =
    "advisory_findings_only_requires_codefriend_synthesis";

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
    pub request_id: Option<String>,
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
#[serde(rename_all = "snake_case")]
pub enum ReviewProviderRoleV1 {
    Reviewer,
    SecurityReviewer,
    TestReviewer,
    DocsReviewer,
    ArchitectureReviewer,
    SynthesisReviewer,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ReviewResultStatusV1 {
    Passed,
    Findings,
    FailedProvider,
    FailedMalformed,
    Blocked,
    Skipped,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ReviewRedactionStatusV1 {
    Redacted,
    NoSensitiveContentObserved,
    RedactionRequired,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ReviewFindingSeverityV1 {
    P0,
    P1,
    P2,
    P3,
    Info,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ReviewProviderV1 {
    pub schema_version: String,
    pub provider_ref: String,
    pub role: ReviewProviderRoleV1,
    pub provider_request: ProviderInvocationRequestV1,
    pub authority_boundary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ReviewProviderRequestV1 {
    pub schema_version: String,
    pub review_request_id: String,
    pub review_provider: ReviewProviderV1,
    pub review_packet_ref: String,
    pub rubric_ref: String,
    pub requested_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issue_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pr_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub diff_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub file_refs: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_output_contract_ref: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ReviewFindingV1 {
    pub severity: ReviewFindingSeverityV1,
    pub title: String,
    pub body: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evidence_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ReviewProviderResultV1 {
    pub schema_version: String,
    pub review_request_id: String,
    pub provider_result: ProviderInvocationResultV1,
    pub review_status: ReviewResultStatusV1,
    pub redaction_status: ReviewRedactionStatusV1,
    pub findings: Vec<ReviewFindingV1>,
    pub started_at: String,
    pub completed_at: String,
    pub elapsed_ms: u64,
    pub log_ref: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub artifact_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub malformed_output_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ReviewRunRecordV1 {
    pub schema_version: String,
    pub run_id: String,
    pub review_request: ReviewProviderRequestV1,
    pub review_result: ReviewProviderResultV1,
    pub model_identity: ModelIdentityV1,
    pub route: ProviderRouteV1,
    pub log_ref: String,
    pub redaction_status: ReviewRedactionStatusV1,
    pub authority_boundary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct ProviderRunLogEventV1 {
    pub schema_version: String,
    pub timestamp: String,
    pub run_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
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
    pub artifact_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<Value>,
}

pub struct ProviderRunLoggerV1 {
    run_id: String,
    request_id: Option<String>,
    artifact_ref: Option<String>,
    writer: BufWriter<File>,
}

impl ProviderRunLoggerV1 {
    pub fn create(path: impl AsRef<Path>, run_id: impl Into<String>) -> Result<Self> {
        Self::create_with_context(path, run_id, None, None)
    }

    pub fn create_with_context(
        path: impl AsRef<Path>,
        run_id: impl Into<String>,
        request_id: Option<String>,
        artifact_ref: Option<String>,
    ) -> Result<Self> {
        let path = path.as_ref();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let file = OpenOptions::new().create(true).append(true).open(path)?;
        Ok(Self {
            run_id: run_id.into(),
            request_id,
            artifact_ref,
            writer: BufWriter::new(file),
        })
    }

    pub fn event(&mut self, mut event: ProviderRunLogEventV1) -> Result<()> {
        event.schema_version = PROVIDER_COMMUNICATION_SCHEMA_VERSION.to_string();
        event.run_id = self.run_id.clone();
        if event.request_id.is_none() {
            event.request_id = self.request_id.clone();
        }
        if event.artifact_ref.is_none() {
            event.artifact_ref = self.artifact_ref.clone();
        }
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
            request_id: None,
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
            artifact_ref: None,
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
    provider_failure_kind_from_resilience(&provider_fault_classification(note, http_status))
}

pub fn provider_failure_from_note(note: &str, http_status: Option<u16>) -> ProviderFailureV1 {
    let classification = provider_fault_classification(note, http_status);
    provider_failure_from_classification(&classification)
}

pub fn provider_fault_classification(
    note: &str,
    http_status: Option<u16>,
) -> ResilienceFaultClassificationV1 {
    ResilienceFaultClassificationV1::provider(note, http_status)
}

pub fn provider_failure_classification_from_failure(
    failure: &ProviderFailureV1,
) -> ResilienceFaultClassificationV1 {
    let (fault_class, disposition) = match failure.kind {
        ProviderFailureKindV1::ProviderAuthMissing => (
            ResilienceFaultClassV1::ProviderAuthMissing,
            ResilienceFaultDispositionV1::OperatorGated,
        ),
        ProviderFailureKindV1::ProviderAuthError => (
            ResilienceFaultClassV1::ProviderAuthError,
            ResilienceFaultDispositionV1::OperatorGated,
        ),
        ProviderFailureKindV1::ProviderRateLimited => (
            ResilienceFaultClassV1::ProviderRateLimited,
            ResilienceFaultDispositionV1::Retryable,
        ),
        ProviderFailureKindV1::ProviderTimeout => (
            ResilienceFaultClassV1::ProviderTimeout,
            ResilienceFaultDispositionV1::Retryable,
        ),
        ProviderFailureKindV1::ProviderTransientHttp => (
            ResilienceFaultClassV1::ProviderTransientHttp,
            ResilienceFaultDispositionV1::Retryable,
        ),
        ProviderFailureKindV1::ProviderEmptyTextOutput => (
            ResilienceFaultClassV1::ProviderEmptyTextOutput,
            ResilienceFaultDispositionV1::Terminal,
        ),
        ProviderFailureKindV1::ProviderModelUnavailable => (
            ResilienceFaultClassV1::ProviderModelUnavailable,
            ResilienceFaultDispositionV1::Terminal,
        ),
        ProviderFailureKindV1::ProviderBillingBlocked => (
            ResilienceFaultClassV1::ProviderBillingBlocked,
            ResilienceFaultDispositionV1::OperatorGated,
        ),
        ProviderFailureKindV1::LocalRuntimeUnavailable => (
            ResilienceFaultClassV1::LocalRuntimeUnavailable,
            ResilienceFaultDispositionV1::Retryable,
        ),
        ProviderFailureKindV1::LocalRuntimeBusy => (
            ResilienceFaultClassV1::LocalRuntimeBusy,
            ResilienceFaultDispositionV1::Retryable,
        ),
        ProviderFailureKindV1::LocalRuntimeHung => (
            ResilienceFaultClassV1::LocalRuntimeHung,
            ResilienceFaultDispositionV1::Retryable,
        ),
        ProviderFailureKindV1::ProviderError => (
            ResilienceFaultClassV1::ProviderError,
            ResilienceFaultDispositionV1::Terminal,
        ),
        ProviderFailureKindV1::Unknown => (
            ResilienceFaultClassV1::Unknown,
            ResilienceFaultDispositionV1::Retryable,
        ),
    };
    ResilienceFaultClassificationV1 {
        schema_version: crate::resilience::RESILIENCE_FAULT_CLASSIFICATION_SCHEMA_V1.to_string(),
        surface: crate::resilience::ResilienceSurfaceV1::Provider,
        fault_class,
        disposition: disposition.clone(),
        retryable: matches!(disposition, ResilienceFaultDispositionV1::Retryable),
        summary: sanitize_resilience_summary(&failure.message),
        component_ref: None,
        http_status: failure.http_status,
    }
}

pub fn provider_failure_from_classification(
    classification: &ResilienceFaultClassificationV1,
) -> ProviderFailureV1 {
    ProviderFailureV1 {
        kind: provider_failure_kind_from_resilience(classification),
        retryable: matches!(
            classification.disposition,
            ResilienceFaultDispositionV1::Retryable
        ),
        message: classification.summary.clone(),
        provider_error_excerpt: Some(classification.summary.clone()),
        http_status: classification.http_status,
    }
}

pub fn provider_attempt_policy_as_resilience_policy(
    policy_id: impl Into<String>,
    attempt_policy: &ProviderAttemptPolicyV1,
) -> ResiliencePolicyV1 {
    let mut policy = ResiliencePolicyV1::provider_attempt_policy(
        policy_id,
        attempt_policy.max_attempts,
        attempt_policy.timeout_ms,
    );
    if let Some(retry) = policy.retry.as_mut() {
        retry.backoff_ms = attempt_policy.retry_backoff_ms;
    }
    policy
}

fn provider_failure_kind_from_resilience(
    classification: &ResilienceFaultClassificationV1,
) -> ProviderFailureKindV1 {
    match classification.fault_class {
        ResilienceFaultClassV1::ProviderAuthMissing => ProviderFailureKindV1::ProviderAuthMissing,
        ResilienceFaultClassV1::ProviderAuthError => ProviderFailureKindV1::ProviderAuthError,
        ResilienceFaultClassV1::ProviderRateLimited => ProviderFailureKindV1::ProviderRateLimited,
        ResilienceFaultClassV1::ProviderTimeout => ProviderFailureKindV1::ProviderTimeout,
        ResilienceFaultClassV1::ProviderTransientHttp => {
            ProviderFailureKindV1::ProviderTransientHttp
        }
        ResilienceFaultClassV1::ProviderEmptyTextOutput => {
            ProviderFailureKindV1::ProviderEmptyTextOutput
        }
        ResilienceFaultClassV1::ProviderModelUnavailable => {
            ProviderFailureKindV1::ProviderModelUnavailable
        }
        ResilienceFaultClassV1::ProviderBillingBlocked => {
            ProviderFailureKindV1::ProviderBillingBlocked
        }
        ResilienceFaultClassV1::LocalRuntimeUnavailable => {
            ProviderFailureKindV1::LocalRuntimeUnavailable
        }
        ResilienceFaultClassV1::LocalRuntimeBusy => ProviderFailureKindV1::LocalRuntimeBusy,
        ResilienceFaultClassV1::LocalRuntimeHung => ProviderFailureKindV1::LocalRuntimeHung,
        ResilienceFaultClassV1::ProviderError => ProviderFailureKindV1::ProviderError,
        ResilienceFaultClassV1::ToolFailure
        | ResilienceFaultClassV1::WorkflowFailure
        | ResilienceFaultClassV1::RuntimeFailure
        | ResilienceFaultClassV1::Unknown => ProviderFailureKindV1::Unknown,
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

pub fn validate_provider_result(result: &ProviderInvocationResultV1) -> Result<()> {
    require_schema_version("provider_result.schema_version", &result.schema_version)?;
    require_non_empty("provider_result.route.provider", &result.route.provider)?;
    require_non_empty(
        "provider_result.route.provider_model_id",
        &result.route.provider_model_id,
    )?;
    require_non_empty(
        "provider_result.model_identity.provider_kind",
        &result.model_identity.provider_kind,
    )?;
    require_non_empty(
        "provider_result.model_identity.provider",
        &result.model_identity.provider,
    )?;
    require_non_empty(
        "provider_result.model_identity.model_ref",
        &result.model_identity.model_ref,
    )?;
    require_non_empty(
        "provider_result.model_identity.provider_model_id",
        &result.model_identity.provider_model_id,
    )?;
    require_non_empty(
        "provider_result.model_identity.runtime_surface",
        &result.model_identity.runtime_surface,
    )?;
    require_non_empty(
        "provider_result.model_identity.observed_at",
        &result.model_identity.observed_at,
    )?;
    if result.attempts.is_empty() {
        return Err(anyhow!("provider_result.attempts must not be empty"));
    }
    for attempt in &result.attempts {
        require_non_empty("provider_result.attempts.started_at", &attempt.started_at)?;
        match &attempt.status {
            ProviderAttemptStatusV1::Ok => {
                if attempt.failure.is_some() {
                    return Err(anyhow!(
                        "provider_result ok attempts must not carry failure details"
                    ));
                }
            }
            ProviderAttemptStatusV1::Error | ProviderAttemptStatusV1::Timeout => {
                if attempt.failure.is_none() {
                    return Err(anyhow!(
                        "provider_result failed attempts must carry failure details"
                    ));
                }
            }
        }
    }
    match &result.final_status {
        ProviderInvocationFinalStatusV1::Ok => {
            if result.failure.is_some() {
                return Err(anyhow!(
                    "provider_result final_status ok must not carry failure"
                ));
            }
            if result.output_text.is_none() && result.output_text_excerpt.is_none() {
                return Err(anyhow!(
                    "provider_result final_status ok requires output text or excerpt"
                ));
            }
        }
        ProviderInvocationFinalStatusV1::Failed => {
            if result.failure.is_none() {
                return Err(anyhow!(
                    "provider_result final_status failed requires failure"
                ));
            }
        }
        ProviderInvocationFinalStatusV1::Skipped | ProviderInvocationFinalStatusV1::Blocked => {
            if result.failure.is_none() {
                return Err(anyhow!(
                    "provider_result final_status skipped or blocked requires failure"
                ));
            }
        }
    }
    Ok(())
}

pub fn validate_review_provider_request(request: &ReviewProviderRequestV1) -> Result<()> {
    require_schema_version("schema_version", &request.schema_version)?;
    require_non_empty("review_request_id", &request.review_request_id)?;
    require_non_empty("review_packet_ref", &request.review_packet_ref)?;
    require_non_empty("rubric_ref", &request.rubric_ref)?;
    require_non_empty("requested_at", &request.requested_at)?;
    require_schema_version(
        "review_provider.schema_version",
        &request.review_provider.schema_version,
    )?;
    require_non_empty(
        "review_provider.provider_ref",
        &request.review_provider.provider_ref,
    )?;
    require_non_empty(
        "review_provider.authority_boundary",
        &request.review_provider.authority_boundary,
    )?;
    if request.review_provider.authority_boundary != REVIEW_PROVIDER_AUTHORITY_BOUNDARY_V1 {
        return Err(anyhow!(
            "review_provider.authority_boundary must be {REVIEW_PROVIDER_AUTHORITY_BOUNDARY_V1}"
        ));
    }
    validate_provider_request(&request.review_provider.provider_request)?;
    let has_scope = request
        .issue_ref
        .as_deref()
        .is_some_and(|value| !value.trim().is_empty())
        || request
            .pr_ref
            .as_deref()
            .is_some_and(|value| !value.trim().is_empty())
        || request
            .diff_ref
            .as_deref()
            .is_some_and(|value| !value.trim().is_empty())
        || request
            .file_refs
            .iter()
            .any(|value| !value.trim().is_empty());
    if !has_scope {
        return Err(anyhow!(
            "review request must include at least one issue, PR, diff, or file scope reference"
        ));
    }
    Ok(())
}

pub fn validate_review_provider_result(result: &ReviewProviderResultV1) -> Result<()> {
    require_schema_version("schema_version", &result.schema_version)?;
    require_non_empty("review_request_id", &result.review_request_id)?;
    require_non_empty("started_at", &result.started_at)?;
    require_non_empty("completed_at", &result.completed_at)?;
    require_non_empty("log_ref", &result.log_ref)?;
    validate_provider_result(&result.provider_result)?;
    let provider_failed = matches!(
        result.provider_result.final_status,
        ProviderInvocationFinalStatusV1::Failed
            | ProviderInvocationFinalStatusV1::Skipped
            | ProviderInvocationFinalStatusV1::Blocked
    );
    if provider_failed
        && !matches!(
            result.review_status,
            ReviewResultStatusV1::FailedProvider
                | ReviewResultStatusV1::FailedMalformed
                | ReviewResultStatusV1::Blocked
                | ReviewResultStatusV1::Skipped
        )
    {
        return Err(anyhow!(
            "provider_result failure requires failed_provider, failed_malformed, blocked, or skipped review_status"
        ));
    }
    if matches!(
        result.provider_result.final_status,
        ProviderInvocationFinalStatusV1::Ok
    ) && matches!(
        result.review_status,
        ReviewResultStatusV1::FailedProvider
            | ReviewResultStatusV1::Blocked
            | ReviewResultStatusV1::Skipped
    ) {
        return Err(anyhow!(
            "provider_result ok must not be reported as provider failed, blocked, or skipped"
        ));
    }
    let failure_status = matches!(
        result.review_status,
        ReviewResultStatusV1::FailedProvider
            | ReviewResultStatusV1::FailedMalformed
            | ReviewResultStatusV1::Blocked
            | ReviewResultStatusV1::Skipped
    );
    if failure_status && !result.findings.is_empty() {
        return Err(anyhow!(
            "failed, blocked, or skipped review-provider results must not carry scored findings"
        ));
    }
    if matches!(result.review_status, ReviewResultStatusV1::Findings) && result.findings.is_empty()
    {
        return Err(anyhow!(
            "review_status findings requires at least one finding"
        ));
    }
    if matches!(result.review_status, ReviewResultStatusV1::Passed) && !result.findings.is_empty() {
        return Err(anyhow!("review_status passed must not carry findings"));
    }
    Ok(())
}

#[cfg(test)]
fn sanitize_provider_message(note: &str) -> String {
    sanitize_resilience_summary(note)
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

fn require_non_empty(field: &str, value: &str) -> Result<()> {
    if value.trim().is_empty() {
        return Err(anyhow!("{field} must not be empty"));
    }
    Ok(())
}

fn require_schema_version(field: &str, value: &str) -> Result<()> {
    if value != PROVIDER_COMMUNICATION_SCHEMA_VERSION {
        return Err(anyhow!(
            "{field} must be {PROVIDER_COMMUNICATION_SCHEMA_VERSION}"
        ));
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
    fn provider_run_logger_injects_request_and_artifact_refs_from_context() {
        let path = temp_log_path();
        let mut logger = ProviderRunLoggerV1::create_with_context(
            &path,
            "run-2",
            Some("req-2".to_string()),
            Some("logs/provider-run.jsonl".to_string()),
        )
        .unwrap();
        logger
            .event(ProviderRunLogEventV1::new("run_start"))
            .unwrap();
        drop(logger);

        let raw = fs::read_to_string(&path).unwrap();
        let row: ProviderRunLogEventV1 = serde_json::from_str(raw.trim()).unwrap();
        assert_eq!(row.run_id, "run-2");
        assert_eq!(row.request_id.as_deref(), Some("req-2"));
        assert_eq!(row.artifact_ref.as_deref(), Some("logs/provider-run.jsonl"));
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
    fn provider_fault_classification_uses_shared_resilience_vocab() {
        let classification =
            provider_fault_classification("status=429 rate limit exceeded", Some(429));
        assert_eq!(
            classification.fault_class,
            ResilienceFaultClassV1::ProviderRateLimited
        );
        assert_eq!(
            classification.disposition,
            ResilienceFaultDispositionV1::Retryable
        );
    }

    #[test]
    fn provider_attempt_policy_maps_to_resilience_policy() {
        let policy = provider_attempt_policy_as_resilience_policy(
            "provider_default",
            &ProviderAttemptPolicyV1 {
                max_attempts: 4,
                timeout_ms: 5_000,
                retry_backoff_ms: Some(250),
            },
        );
        let retry = policy.retry.expect("retry policy");
        let timeout = policy.timeout.expect("timeout policy");
        assert_eq!(retry.max_attempts, 4);
        assert_eq!(retry.backoff_ms, Some(250));
        assert_eq!(timeout.timeout_ms, 5_000);
    }

    #[test]
    fn provider_fault_classification_is_total_for_generic_provider_failures() {
        let classification =
            provider_fault_classification("unexpected provider socket wobble", None);
        assert_eq!(classification.fault_class, ResilienceFaultClassV1::Unknown);
        assert_eq!(
            classification.disposition,
            ResilienceFaultDispositionV1::Retryable
        );
    }

    #[test]
    fn provider_fault_classification_redacts_sensitive_summary() {
        let classification = provider_fault_classification(
            "provider request failed for key=super-secret-token",
            None,
        );
        assert_eq!(classification.summary, "redacted provider diagnostic");
    }

    #[test]
    fn provider_failure_round_trips_through_shared_resilience_vocab() {
        let failure = ProviderFailureV1 {
            kind: ProviderFailureKindV1::ProviderRateLimited,
            retryable: true,
            message: "rate limit exceeded".to_string(),
            provider_error_excerpt: Some("rate limit exceeded".to_string()),
            http_status: Some(429),
        };
        let classification = provider_failure_classification_from_failure(&failure);
        assert_eq!(
            classification.fault_class,
            ResilienceFaultClassV1::ProviderRateLimited
        );
        let remapped = provider_failure_from_classification(&classification);
        assert_eq!(remapped.kind, ProviderFailureKindV1::ProviderRateLimited);
        assert!(remapped.retryable);
        assert_eq!(remapped.http_status, Some(429));
    }

    #[test]
    fn url_query_api_keys_are_redacted_from_provider_failures() {
        let failure = provider_failure_from_note(
            "error sending request for url (https://generativelanguage.googleapis.com/v1beta/models/gemini:generateContent?key=synthetic-secret)",
            None,
        );
        assert_eq!(failure.message, "redacted provider diagnostic");
        assert!(!failure
            .provider_error_excerpt
            .as_deref()
            .unwrap_or_default()
            .contains("synthetic-secret"));
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
            request_id: None,
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
            request_id: None,
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

    fn review_provider_request_fixture() -> ReviewProviderRequestV1 {
        let identity = hosted_model_identity("anthropic", "review/default", "claude-test", None);
        let route = ProviderRouteV1 {
            provider_kind: ProviderKindV1::Hosted,
            provider: "anthropic".to_string(),
            runtime_surface: RuntimeSurfaceV1::HostedApi,
            provider_model_id: "claude-test".to_string(),
            endpoint_ref: Some("anthropic.messages".to_string()),
            credential_ref: Some("env:ANTHROPIC_API_KEY".to_string()),
            source_registry: Some("review-provider-profile".to_string()),
        };
        let provider_request = ProviderInvocationRequestV1 {
            route,
            model_identity: identity,
            prompt_contract_ref: "codefriend.review_packet.v1".to_string(),
            lane_ref: "external_review_provider".to_string(),
            run_id: Some("review-run-1".to_string()),
            request_id: Some("provider-request-1".to_string()),
            attempt_policy: ProviderAttemptPolicyV1 {
                max_attempts: 1,
                timeout_ms: 30_000,
                retry_backoff_ms: None,
            },
            input_text: None,
            inference_parameter_fingerprint: Some("sha256:test".to_string()),
            tool_surface: None,
            governance_surface: Some("codefriend.synthesis_required.v1".to_string()),
            evaluator_ref: Some("review-provider-contract.v1".to_string()),
            benchmark_ref: None,
        };
        ReviewProviderRequestV1 {
            schema_version: PROVIDER_COMMUNICATION_SCHEMA_VERSION.to_string(),
            review_request_id: "review-request-1".to_string(),
            review_provider: ReviewProviderV1 {
                schema_version: PROVIDER_COMMUNICATION_SCHEMA_VERSION.to_string(),
                provider_ref: "review-provider/anthropic/default".to_string(),
                role: ReviewProviderRoleV1::Reviewer,
                provider_request,
                authority_boundary: REVIEW_PROVIDER_AUTHORITY_BOUNDARY_V1.to_string(),
            },
            review_packet_ref: "review/packets/issue-1.json".to_string(),
            rubric_ref: "codefriend.rubric.findings_first.v1".to_string(),
            requested_at: "unix:1".to_string(),
            issue_ref: Some("https://github.com/example/repo/issues/1".to_string()),
            pr_ref: None,
            diff_ref: None,
            file_refs: Vec::new(),
            expected_output_contract_ref: Some("ReviewProviderResultV1".to_string()),
        }
    }

    #[test]
    fn review_provider_request_validates_scope_authority_and_provider_request() {
        let request = review_provider_request_fixture();
        validate_review_provider_request(&request).unwrap();

        let doc = serde_json::to_value(&request).unwrap();
        assert_eq!(
            doc["review_provider"]["authority_boundary"],
            REVIEW_PROVIDER_AUTHORITY_BOUNDARY_V1
        );
        assert_eq!(
            doc["review_provider"]["provider_request"]["lane_ref"],
            "external_review_provider"
        );
    }

    #[test]
    fn review_provider_request_fails_closed_without_scope_or_authority() {
        let mut no_scope = review_provider_request_fixture();
        no_scope.issue_ref = None;
        assert!(validate_review_provider_request(&no_scope)
            .unwrap_err()
            .to_string()
            .contains("at least one issue, PR, diff, or file scope"));

        let mut blank_scope = review_provider_request_fixture();
        blank_scope.issue_ref = Some("  ".to_string());
        blank_scope.file_refs = vec!["".to_string(), "   ".to_string()];
        assert!(validate_review_provider_request(&blank_scope)
            .unwrap_err()
            .to_string()
            .contains("at least one issue, PR, diff, or file scope"));

        let mut no_authority = review_provider_request_fixture();
        no_authority.review_provider.authority_boundary.clear();
        assert!(validate_review_provider_request(&no_authority)
            .unwrap_err()
            .to_string()
            .contains("review_provider.authority_boundary"));

        let mut wrong_authority = review_provider_request_fixture();
        wrong_authority.review_provider.authority_boundary = "advisory_but_ambiguous".to_string();
        assert!(validate_review_provider_request(&wrong_authority)
            .unwrap_err()
            .to_string()
            .contains(REVIEW_PROVIDER_AUTHORITY_BOUNDARY_V1));

        let mut wrong_schema = review_provider_request_fixture();
        wrong_schema.schema_version = "provider_communication.v0".to_string();
        assert!(validate_review_provider_request(&wrong_schema)
            .unwrap_err()
            .to_string()
            .contains(PROVIDER_COMMUNICATION_SCHEMA_VERSION));
    }

    #[test]
    fn review_provider_result_preserves_findings_and_provider_failure_boundaries() {
        let request = review_provider_request_fixture();
        let route = request.review_provider.provider_request.route.clone();
        let identity = request
            .review_provider
            .provider_request
            .model_identity
            .clone();
        let provider_failure = provider_failure_from_note("provider returned empty output", None);
        let provider_result = ProviderInvocationResultV1 {
            schema_version: PROVIDER_COMMUNICATION_SCHEMA_VERSION.to_string(),
            route: route.clone(),
            model_identity: identity.clone(),
            attempts: vec![ProviderAttemptV1 {
                attempt_index: 1,
                started_at: "unix:1".to_string(),
                duration_ms: 25,
                status: ProviderAttemptStatusV1::Error,
                retryable: false,
                http_status: None,
                failure: Some(provider_failure.clone()),
                raw_response_excerpt: None,
            }],
            final_status: ProviderInvocationFinalStatusV1::Failed,
            duration_ms: 25,
            request_id: Some("provider-request-1".to_string()),
            output_text: None,
            output_text_excerpt: None,
            failure: Some(provider_failure),
            artifact_ref: None,
            trace_ref: None,
        };
        let result = ReviewProviderResultV1 {
            schema_version: PROVIDER_COMMUNICATION_SCHEMA_VERSION.to_string(),
            review_request_id: request.review_request_id.clone(),
            provider_result,
            review_status: ReviewResultStatusV1::FailedProvider,
            redaction_status: ReviewRedactionStatusV1::Redacted,
            findings: Vec::new(),
            started_at: "unix:1".to_string(),
            completed_at: "unix:2".to_string(),
            elapsed_ms: 25,
            log_ref: "review/logs/review-run-1.jsonl".to_string(),
            artifact_ref: Some("review/results/review-run-1.json".to_string()),
            malformed_output_reason: None,
        };
        let run = ReviewRunRecordV1 {
            schema_version: PROVIDER_COMMUNICATION_SCHEMA_VERSION.to_string(),
            run_id: "review-run-1".to_string(),
            review_request: request,
            review_result: result,
            model_identity: identity,
            route,
            log_ref: "review/logs/review-run-1.jsonl".to_string(),
            redaction_status: ReviewRedactionStatusV1::Redacted,
            authority_boundary: REVIEW_PROVIDER_AUTHORITY_BOUNDARY_V1.to_string(),
        };
        validate_review_provider_result(&run.review_result).unwrap();
        assert_eq!(
            run.review_result.review_status,
            ReviewResultStatusV1::FailedProvider
        );
        assert!(run.review_result.findings.is_empty());
        assert_eq!(
            run.review_result
                .provider_result
                .failure
                .as_ref()
                .unwrap()
                .kind,
            ProviderFailureKindV1::ProviderEmptyTextOutput
        );
    }

    #[test]
    fn review_provider_result_rejects_findings_on_failed_or_passed_status() {
        let request = review_provider_request_fixture();
        let route = request.review_provider.provider_request.route.clone();
        let identity = request
            .review_provider
            .provider_request
            .model_identity
            .clone();
        let provider_result = ProviderInvocationResultV1 {
            schema_version: PROVIDER_COMMUNICATION_SCHEMA_VERSION.to_string(),
            route,
            model_identity: identity,
            attempts: vec![ProviderAttemptV1 {
                attempt_index: 1,
                started_at: "unix:1".to_string(),
                duration_ms: 25,
                status: ProviderAttemptStatusV1::Ok,
                retryable: false,
                http_status: Some(200),
                failure: None,
                raw_response_excerpt: Some("[redacted response len=120]".to_string()),
            }],
            final_status: ProviderInvocationFinalStatusV1::Ok,
            duration_ms: 25,
            request_id: Some("provider-request-1".to_string()),
            output_text: None,
            output_text_excerpt: Some("[redacted review output]".to_string()),
            failure: None,
            artifact_ref: None,
            trace_ref: None,
        };
        let ok_provider_result = provider_result.clone();
        let finding = ReviewFindingV1 {
            severity: ReviewFindingSeverityV1::P2,
            title: "Example finding".to_string(),
            body: "Finding body stays advisory until CodeFriend synthesis.".to_string(),
            evidence_ref: Some("review/packets/issue-1.json".to_string()),
            file_ref: Some("src/lib.rs".to_string()),
            line: Some(12),
        };
        let mut result = ReviewProviderResultV1 {
            schema_version: PROVIDER_COMMUNICATION_SCHEMA_VERSION.to_string(),
            review_request_id: request.review_request_id,
            provider_result,
            review_status: ReviewResultStatusV1::Findings,
            redaction_status: ReviewRedactionStatusV1::Redacted,
            findings: vec![finding],
            started_at: "unix:1".to_string(),
            completed_at: "unix:2".to_string(),
            elapsed_ms: 25,
            log_ref: "review/logs/review-run-1.jsonl".to_string(),
            artifact_ref: Some("review/results/review-run-1.json".to_string()),
            malformed_output_reason: None,
        };
        validate_review_provider_result(&result).unwrap();

        let provider_failure = provider_failure_from_note("provider returned empty output", None);
        let mut failed_provider_result = ok_provider_result.clone();
        failed_provider_result.attempts[0].status = ProviderAttemptStatusV1::Error;
        failed_provider_result.attempts[0].failure = Some(provider_failure.clone());
        failed_provider_result.attempts[0].raw_response_excerpt = None;
        failed_provider_result.final_status = ProviderInvocationFinalStatusV1::Failed;
        failed_provider_result.output_text = None;
        failed_provider_result.output_text_excerpt = None;
        failed_provider_result.failure = Some(provider_failure);
        result.provider_result = failed_provider_result;
        result.review_status = ReviewResultStatusV1::FailedProvider;
        assert!(validate_review_provider_result(&result)
            .unwrap_err()
            .to_string()
            .contains("must not carry scored findings"));

        result.provider_result = ok_provider_result.clone();
        result.review_status = ReviewResultStatusV1::Passed;
        assert!(validate_review_provider_result(&result)
            .unwrap_err()
            .to_string()
            .contains("passed must not carry findings"));

        result.review_status = ReviewResultStatusV1::Findings;
        result.findings.clear();
        assert!(validate_review_provider_result(&result)
            .unwrap_err()
            .to_string()
            .contains("requires at least one finding"));
    }

    #[test]
    fn review_provider_result_rejects_passed_status_when_provider_failed() {
        let request = review_provider_request_fixture();
        let route = request.review_provider.provider_request.route.clone();
        let identity = request
            .review_provider
            .provider_request
            .model_identity
            .clone();
        let provider_failure = provider_failure_from_note("provider timed out", None);
        let provider_result = ProviderInvocationResultV1 {
            schema_version: PROVIDER_COMMUNICATION_SCHEMA_VERSION.to_string(),
            route,
            model_identity: identity,
            attempts: vec![ProviderAttemptV1 {
                attempt_index: 1,
                started_at: "unix:1".to_string(),
                duration_ms: 25,
                status: ProviderAttemptStatusV1::Timeout,
                retryable: true,
                http_status: None,
                failure: Some(provider_failure.clone()),
                raw_response_excerpt: None,
            }],
            final_status: ProviderInvocationFinalStatusV1::Failed,
            duration_ms: 25,
            request_id: Some("provider-request-1".to_string()),
            output_text: None,
            output_text_excerpt: None,
            failure: Some(provider_failure),
            artifact_ref: None,
            trace_ref: None,
        };
        let result = ReviewProviderResultV1 {
            schema_version: PROVIDER_COMMUNICATION_SCHEMA_VERSION.to_string(),
            review_request_id: request.review_request_id,
            provider_result,
            review_status: ReviewResultStatusV1::Passed,
            redaction_status: ReviewRedactionStatusV1::Redacted,
            findings: Vec::new(),
            started_at: "unix:1".to_string(),
            completed_at: "unix:2".to_string(),
            elapsed_ms: 25,
            log_ref: "review/logs/review-run-1.jsonl".to_string(),
            artifact_ref: Some("review/results/review-run-1.json".to_string()),
            malformed_output_reason: None,
        };

        assert!(validate_review_provider_result(&result)
            .unwrap_err()
            .to_string()
            .contains("provider_result failure requires"));
    }

    #[test]
    fn review_provider_result_transitively_rejects_malformed_provider_result() {
        let request = review_provider_request_fixture();
        let route = request.review_provider.provider_request.route.clone();
        let identity = request
            .review_provider
            .provider_request
            .model_identity
            .clone();
        let provider_result = ProviderInvocationResultV1 {
            schema_version: "provider_communication.v0".to_string(),
            route,
            model_identity: identity,
            attempts: vec![ProviderAttemptV1 {
                attempt_index: 1,
                started_at: "unix:1".to_string(),
                duration_ms: 25,
                status: ProviderAttemptStatusV1::Ok,
                retryable: false,
                http_status: Some(200),
                failure: None,
                raw_response_excerpt: Some("[redacted response len=120]".to_string()),
            }],
            final_status: ProviderInvocationFinalStatusV1::Ok,
            duration_ms: 25,
            request_id: Some("provider-request-1".to_string()),
            output_text: None,
            output_text_excerpt: Some("[redacted review output]".to_string()),
            failure: None,
            artifact_ref: None,
            trace_ref: None,
        };
        let result = ReviewProviderResultV1 {
            schema_version: PROVIDER_COMMUNICATION_SCHEMA_VERSION.to_string(),
            review_request_id: request.review_request_id,
            provider_result,
            review_status: ReviewResultStatusV1::Passed,
            redaction_status: ReviewRedactionStatusV1::Redacted,
            findings: Vec::new(),
            started_at: "unix:1".to_string(),
            completed_at: "unix:2".to_string(),
            elapsed_ms: 25,
            log_ref: "review/logs/review-run-1.jsonl".to_string(),
            artifact_ref: Some("review/results/review-run-1.json".to_string()),
            malformed_output_reason: None,
        };

        assert!(validate_review_provider_result(&result)
            .unwrap_err()
            .to_string()
            .contains("provider_result.schema_version"));
    }
}
