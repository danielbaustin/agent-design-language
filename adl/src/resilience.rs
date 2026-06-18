use crate::model_identity::observed_at_now_v1;
use schemars::{schema_for, JsonSchema};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

pub const RESILIENCE_FAULT_CLASSIFICATION_SCHEMA_V1: &str =
    "adl.resilience.fault_classification.v1";
pub const RESILIENCE_CITIZEN_HEALTH_SCHEMA_V1: &str = "adl.resilience.citizen_health.v1";
pub const RESILIENCE_RECOVERY_ARTIFACT_SCHEMA_V1: &str = "adl.resilience.recovery_artifact.v1";
pub const RESILIENCE_CHECKPOINT_SCHEMA_V1: &str = "adl.resilience.checkpoint.v1";
pub const RESILIENCE_TELEMETRY_EVENT_SCHEMA_V1: &str = "adl.resilience.telemetry_event.v1";
pub const RESILIENCE_RETRY_ATTEMPT_SCHEMA_V1: &str = "adl.resilience.retry_attempt.v1";
pub const RESILIENCE_RETRY_EXECUTION_TRACE_SCHEMA_V1: &str =
    "adl.resilience.retry_execution_trace.v1";
pub const RESILIENCE_TIMEOUT_EXECUTION_TRACE_SCHEMA_V1: &str =
    "adl.resilience.timeout_execution_trace.v1";
pub const RESILIENCE_CIRCUIT_BREAKER_EXECUTION_TRACE_SCHEMA_V1: &str =
    "adl.resilience.circuit_breaker_execution_trace.v1";
pub const RESILIENCE_CIRCUIT_BREAKER_STATE_SCHEMA_V1: &str =
    "adl.resilience.circuit_breaker_state.v1";
pub const RESILIENCE_POLICY_SCHEMA_V1: &str = "adl.resilience.policy.v1";
pub const RESILIENCE_SUBSTRATE_SCHEMA_V1: &str = "adl.resilience.substrate_manifest.v1";

static TIMEOUT_EXECUTION_COUNTER: AtomicU64 = AtomicU64::new(0);
static CIRCUIT_BREAKER_EXECUTION_COUNTER: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ResilienceSurfaceV1 {
    Provider,
    Tool,
    Workflow,
    CitizenRuntime,
    Runtime,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ResilienceFaultClassV1 {
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
    ToolFailure,
    WorkflowFailure,
    RuntimeFailure,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ResilienceFaultDispositionV1 {
    Retryable,
    Terminal,
    OperatorGated,
    DegradedAllowed,
    QuarantineRequired,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ResilienceFaultClassificationV1 {
    pub schema_version: String,
    pub surface: ResilienceSurfaceV1,
    pub fault_class: ResilienceFaultClassV1,
    pub disposition: ResilienceFaultDispositionV1,
    pub retryable: bool,
    pub summary: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub component_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub http_status: Option<u16>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CitizenHealthStateV1 {
    Healthy,
    Degraded,
    Recovering,
    Blocked,
    Quarantined,
    Sleeping,
    Hibernating,
    Migrating,
    Restoring,
    Replaying,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct CitizenHealthRecordV1 {
    pub schema_version: String,
    pub citizen_id: String,
    pub state: CitizenHealthStateV1,
    pub observed_at: String,
    pub continuity_claim: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blocking_fault: Option<ResilienceFaultClassificationV1>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checkpoint_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recovery_artifact_ref: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryDispositionV1 {
    ResumeAllowed,
    RetryAllowed,
    QuarantineRequired,
    OperatorInterventionRequired,
    FallbackAllowed,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct RecoveryArtifactV1 {
    pub schema_version: String,
    pub artifact_id: String,
    pub surface: ResilienceSurfaceV1,
    pub triggering_fault: ResilienceFaultClassificationV1,
    pub disposition: RecoveryDispositionV1,
    pub next_action: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_run_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checkpoint_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub evidence_refs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CheckpointKindV1 {
    Provisional,
    Durable,
    SleepWake,
    Migration,
    ReplayAnchor,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct CheckpointRecordV1 {
    pub schema_version: String,
    pub checkpoint_id: String,
    pub kind: CheckpointKindV1,
    pub state_ref: String,
    pub created_at: String,
    pub replayable: bool,
    pub claim_boundary: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub citizen_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lineage_ref: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TelemetryEventKindV1 {
    RetryDecision,
    TimeoutDecision,
    CircuitBreakerDecision,
    RateLimitDecision,
    BulkheadDecision,
    FallbackDecision,
    RecoveryDecision,
    CheckpointCreated,
    CitizenHealthTransition,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ResilienceTelemetryEventV1 {
    pub schema_version: String,
    pub event_id: String,
    pub event_kind: TelemetryEventKindV1,
    pub surface: ResilienceSurfaceV1,
    pub decision_summary: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fault: Option<ResilienceFaultClassificationV1>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub artifact_ref: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct RetryPolicyV1 {
    pub max_attempts: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backoff_ms: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jitter_ms: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_elapsed_ms: Option<u64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub retryable_fault_classes: Vec<ResilienceFaultClassV1>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RetryTerminalReasonV1 {
    Succeeded,
    NonRetryableFault,
    RetryBudgetExhausted,
    RetryTimeBudgetExhausted,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct RetryAttemptRecordV1 {
    pub schema_version: String,
    pub attempt_index: u32,
    pub started_at: String,
    pub duration_ms: u64,
    pub retry_allowed: bool,
    pub scheduled_backoff_ms: u64,
    pub decision_summary: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub terminal_reason: Option<RetryTerminalReasonV1>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fault: Option<ResilienceFaultClassificationV1>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RetryExecutionFinalStatusV1 {
    Succeeded,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct RetryExecutionTraceV1 {
    pub schema_version: String,
    pub policy_id: String,
    pub surface: ResilienceSurfaceV1,
    pub final_status: RetryExecutionFinalStatusV1,
    pub total_duration_ms: u64,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attempts: Vec<RetryAttemptRecordV1>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub telemetry_events: Vec<ResilienceTelemetryEventV1>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recovery_artifact: Option<RecoveryArtifactV1>,
}

#[derive(Debug)]
pub struct RetryExecution<T, E> {
    pub result: Result<T, E>,
    pub trace: RetryExecutionTraceV1,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct TimeoutPolicyV1 {
    pub timeout_ms: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hard_deadline_ms: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TimeoutBreachKindV1 {
    Timeout,
    HardDeadline,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TimeoutExecutionFinalStatusV1 {
    Succeeded,
    Failed,
    TimedOut,
    Cancelled,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimeoutObservation<T, E> {
    pub result: Result<T, E>,
    pub elapsed_ms: u64,
    pub cancelled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct TimeoutExecutionTraceV1 {
    pub schema_version: String,
    pub policy_id: String,
    pub surface: ResilienceSurfaceV1,
    pub final_status: TimeoutExecutionFinalStatusV1,
    pub elapsed_ms: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout_ms: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hard_deadline_ms: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub breach_kind: Option<TimeoutBreachKindV1>,
    pub decision_summary: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fault: Option<ResilienceFaultClassificationV1>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub telemetry_event: Option<ResilienceTelemetryEventV1>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recovery_artifact: Option<RecoveryArtifactV1>,
}

#[derive(Debug)]
pub struct TimeoutExecution<T, E> {
    pub result: Result<T, E>,
    pub trace: TimeoutExecutionTraceV1,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CircuitBreakerStateKindV1 {
    Closed,
    Open,
    HalfOpen,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CircuitBreakerFinalStatusV1 {
    ClosedSuccess,
    ClosedFailure,
    OpenRejected,
    OpenFallback,
    HalfOpenProbeSuccess,
    HalfOpenProbeFailure,
    HalfOpenProbeRejected,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct CircuitBreakerStateV1 {
    pub schema_version: String,
    pub policy_id: String,
    pub state: CircuitBreakerStateKindV1,
    pub consecutive_failures: u32,
    pub half_open_attempts: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub opened_at_ms: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_failure: Option<ResilienceFaultClassificationV1>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct CircuitBreakerExecutionTraceV1 {
    pub schema_version: String,
    pub policy_id: String,
    pub surface: ResilienceSurfaceV1,
    pub state_before: CircuitBreakerStateKindV1,
    pub state_after: CircuitBreakerStateKindV1,
    pub final_status: CircuitBreakerFinalStatusV1,
    pub operation_executed: bool,
    pub used_fallback: bool,
    pub now_ms: u64,
    pub decision_summary: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fault: Option<ResilienceFaultClassificationV1>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub telemetry_event: Option<ResilienceTelemetryEventV1>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recovery_artifact: Option<RecoveryArtifactV1>,
}

#[derive(Debug)]
pub struct CircuitBreakerExecution<T, E> {
    pub result: Result<T, E>,
    pub state: CircuitBreakerStateV1,
    pub trace: CircuitBreakerExecutionTraceV1,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct CircuitBreakerPolicyV1 {
    pub failure_threshold: u32,
    pub recovery_window_ms: u64,
    pub half_open_max_attempts: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct RateLimitPolicyV1 {
    pub max_requests: u32,
    pub window_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct BulkheadPolicyV1 {
    pub fault_domain: String,
    pub max_concurrency: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_queue_depth: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct FallbackPolicyV1 {
    pub fallback_ref: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub activation_fault_classes: Vec<ResilienceFaultClassV1>,
    pub marks_output_degraded: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ResiliencePolicyV1 {
    pub schema_version: String,
    pub policy_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retry: Option<RetryPolicyV1>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<TimeoutPolicyV1>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub circuit_breaker: Option<CircuitBreakerPolicyV1>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rate_limit: Option<RateLimitPolicyV1>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bulkhead: Option<BulkheadPolicyV1>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fallback: Option<FallbackPolicyV1>,
    pub checkpoint_required: bool,
    pub telemetry_required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ResilienceSubstrateManifestV1 {
    pub schema_version: String,
    pub manifest_id: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supported_surfaces: Vec<ResilienceSurfaceV1>,
    pub fault_schema_ref: String,
    pub citizen_health_schema_ref: String,
    pub recovery_artifact_schema_ref: String,
    pub checkpoint_schema_ref: String,
    pub telemetry_schema_ref: String,
    pub policy: ResiliencePolicyV1,
}

impl ResilienceFaultClassificationV1 {
    pub fn provider(note: &str, http_status: Option<u16>) -> Self {
        let lower = note.to_ascii_lowercase();
        let (fault_class, disposition) = if lower.contains("unauthorized")
            || lower.contains("forbidden")
            || lower.contains("invalid api key")
            || lower.contains("invalid_api_key")
            || http_status == Some(401)
            || http_status == Some(403)
        {
            (
                ResilienceFaultClassV1::ProviderAuthError,
                ResilienceFaultDispositionV1::OperatorGated,
            )
        } else if lower.contains("missing required environment variable")
            || lower.contains("missing api_key")
            || lower.contains("missing api key")
        {
            (
                ResilienceFaultClassV1::ProviderAuthMissing,
                ResilienceFaultDispositionV1::OperatorGated,
            )
        } else if lower.contains("rate limit")
            || lower.contains("rate_limited")
            || http_status == Some(429)
        {
            (
                ResilienceFaultClassV1::ProviderRateLimited,
                ResilienceFaultDispositionV1::Retryable,
            )
        } else if lower.contains("timed out") || lower.contains("timeout") {
            (
                ResilienceFaultClassV1::ProviderTimeout,
                ResilienceFaultDispositionV1::Retryable,
            )
        } else if lower.contains("credit balance") || lower.contains("billing") {
            (
                ResilienceFaultClassV1::ProviderBillingBlocked,
                ResilienceFaultDispositionV1::OperatorGated,
            )
        } else if lower.contains("local_runtime_busy") || lower.contains("non-target model") {
            (
                ResilienceFaultClassV1::LocalRuntimeBusy,
                ResilienceFaultDispositionV1::Retryable,
            )
        } else if lower.contains("local_runtime_hung") || lower.contains("stopping...") {
            (
                ResilienceFaultClassV1::LocalRuntimeHung,
                ResilienceFaultDispositionV1::Retryable,
            )
        } else if lower.contains("connection refused")
            || lower.contains("ollama") && lower.contains("not running")
            || lower.contains("local_runtime_unavailable")
        {
            (
                ResilienceFaultClassV1::LocalRuntimeUnavailable,
                ResilienceFaultDispositionV1::Retryable,
            )
        } else if lower.contains("model")
            && (lower.contains("not found") || lower.contains("does not exist"))
        {
            (
                ResilienceFaultClassV1::ProviderModelUnavailable,
                ResilienceFaultDispositionV1::Terminal,
            )
        } else if lower.contains("empty")
            && (lower.contains("response") || lower.contains("output"))
        {
            (
                ResilienceFaultClassV1::ProviderEmptyTextOutput,
                ResilienceFaultDispositionV1::Terminal,
            )
        } else if matches!(http_status, Some(500..=599)) {
            (
                ResilienceFaultClassV1::ProviderTransientHttp,
                ResilienceFaultDispositionV1::Retryable,
            )
        } else if http_status.is_some() || lower.contains("provider_") {
            (
                ResilienceFaultClassV1::ProviderError,
                ResilienceFaultDispositionV1::Terminal,
            )
        } else {
            (
                ResilienceFaultClassV1::Unknown,
                ResilienceFaultDispositionV1::Retryable,
            )
        };

        let retryable = matches!(disposition, ResilienceFaultDispositionV1::Retryable);
        Self {
            schema_version: RESILIENCE_FAULT_CLASSIFICATION_SCHEMA_V1.to_string(),
            surface: ResilienceSurfaceV1::Provider,
            fault_class,
            disposition,
            retryable,
            summary: sanitize_resilience_summary(note),
            component_ref: None,
            http_status,
        }
    }
}

impl ResiliencePolicyV1 {
    pub fn provider_attempt_policy(
        policy_id: impl Into<String>,
        max_attempts: u32,
        timeout_ms: u64,
    ) -> Self {
        Self {
            schema_version: RESILIENCE_POLICY_SCHEMA_V1.to_string(),
            policy_id: policy_id.into(),
            retry: Some(RetryPolicyV1 {
                max_attempts,
                backoff_ms: None,
                jitter_ms: None,
                max_elapsed_ms: None,
                retryable_fault_classes: vec![
                    ResilienceFaultClassV1::ProviderRateLimited,
                    ResilienceFaultClassV1::ProviderTimeout,
                    ResilienceFaultClassV1::ProviderTransientHttp,
                    ResilienceFaultClassV1::LocalRuntimeUnavailable,
                    ResilienceFaultClassV1::LocalRuntimeBusy,
                    ResilienceFaultClassV1::LocalRuntimeHung,
                    ResilienceFaultClassV1::Unknown,
                ],
            }),
            timeout: Some(TimeoutPolicyV1 {
                timeout_ms,
                hard_deadline_ms: None,
            }),
            circuit_breaker: None,
            rate_limit: None,
            bulkhead: None,
            fallback: None,
            checkpoint_required: false,
            telemetry_required: true,
        }
    }
}

impl RetryPolicyV1 {
    fn permits_fault(&self, classification: &ResilienceFaultClassificationV1) -> bool {
        if !classification.retryable {
            return false;
        }
        self.retryable_fault_classes.is_empty()
            || self
                .retryable_fault_classes
                .contains(&classification.fault_class)
    }

    fn next_delay_ms(&self, policy_id: &str, attempt_index: u32) -> u64 {
        let base = self.backoff_ms.unwrap_or(0);
        let multiplier_shift = attempt_index.saturating_sub(1).min(20);
        let multiplier = 1_u64 << multiplier_shift;
        let backoff = base.saturating_mul(multiplier);
        backoff.saturating_add(deterministic_jitter_ms(
            policy_id,
            attempt_index,
            self.jitter_ms.unwrap_or(0),
        ))
    }
}

pub fn execute_retry_policy<T, E, F, C, S, O>(
    policy: &ResiliencePolicyV1,
    surface: ResilienceSurfaceV1,
    operation_ref: &str,
    mut operation: F,
    mut classify_error: C,
    mut sleep_fn: S,
    mut observe_attempt: O,
) -> RetryExecution<T, E>
where
    E: Clone,
    F: FnMut(u32) -> Result<T, E>,
    C: FnMut(&E) -> ResilienceFaultClassificationV1,
    S: FnMut(u64),
    O: FnMut(&RetryAttemptRecordV1),
{
    let retry = policy.retry.clone().unwrap_or(RetryPolicyV1 {
        max_attempts: 1,
        backoff_ms: None,
        jitter_ms: None,
        max_elapsed_ms: None,
        retryable_fault_classes: Vec::new(),
    });
    let started = Instant::now();
    let mut attempts = Vec::new();
    let mut telemetry_events = Vec::new();

    for attempt_index in 1..=retry.max_attempts.max(1) {
        let attempt_started_at = observed_at_now_v1();
        let attempt_started = Instant::now();
        match operation(attempt_index) {
            Ok(value) => {
                let record = RetryAttemptRecordV1 {
                    schema_version: RESILIENCE_RETRY_ATTEMPT_SCHEMA_V1.to_string(),
                    attempt_index,
                    started_at: attempt_started_at.clone(),
                    duration_ms: attempt_started.elapsed().as_millis() as u64,
                    retry_allowed: false,
                    scheduled_backoff_ms: 0,
                    decision_summary: format!("attempt {attempt_index} succeeded"),
                    terminal_reason: Some(RetryTerminalReasonV1::Succeeded),
                    fault: None,
                };
                observe_attempt(&record);
                telemetry_events.push(retry_decision_event(
                    policy,
                    surface.clone(),
                    operation_ref,
                    attempt_index,
                    &record.decision_summary,
                    None,
                ));
                attempts.push(record);
                return RetryExecution {
                    result: Ok(value),
                    trace: RetryExecutionTraceV1 {
                        schema_version: RESILIENCE_RETRY_EXECUTION_TRACE_SCHEMA_V1.to_string(),
                        policy_id: policy.policy_id.clone(),
                        surface,
                        final_status: RetryExecutionFinalStatusV1::Succeeded,
                        total_duration_ms: started.elapsed().as_millis() as u64,
                        attempts,
                        telemetry_events,
                        recovery_artifact: None,
                    },
                };
            }
            Err(error) => {
                let classification = classify_error(&error);
                let delay_ms = retry.next_delay_ms(&policy.policy_id, attempt_index);
                let elapsed_ms = started.elapsed().as_millis() as u64;
                let retryable_fault = retry.permits_fault(&classification);
                let within_attempt_budget = attempt_index < retry.max_attempts.max(1);
                let within_time_budget = retry
                    .max_elapsed_ms
                    .map(|max_elapsed_ms| elapsed_ms.saturating_add(delay_ms) <= max_elapsed_ms)
                    .unwrap_or(true);
                let retry_allowed = retryable_fault && within_attempt_budget && within_time_budget;
                let terminal_reason = if retry_allowed {
                    None
                } else if !retryable_fault {
                    Some(RetryTerminalReasonV1::NonRetryableFault)
                } else if !within_attempt_budget {
                    Some(RetryTerminalReasonV1::RetryBudgetExhausted)
                } else {
                    Some(RetryTerminalReasonV1::RetryTimeBudgetExhausted)
                };
                let decision_summary = retry_decision_summary(
                    attempt_index,
                    &classification,
                    retry_allowed,
                    delay_ms,
                    terminal_reason.as_ref(),
                );
                let record = RetryAttemptRecordV1 {
                    schema_version: RESILIENCE_RETRY_ATTEMPT_SCHEMA_V1.to_string(),
                    attempt_index,
                    started_at: attempt_started_at,
                    duration_ms: attempt_started.elapsed().as_millis() as u64,
                    retry_allowed,
                    scheduled_backoff_ms: if retry_allowed { delay_ms } else { 0 },
                    decision_summary: decision_summary.clone(),
                    terminal_reason: terminal_reason.clone(),
                    fault: Some(classification.clone()),
                };
                observe_attempt(&record);
                telemetry_events.push(retry_decision_event(
                    policy,
                    surface.clone(),
                    operation_ref,
                    attempt_index,
                    &decision_summary,
                    Some(classification.clone()),
                ));
                attempts.push(record);
                if retry_allowed {
                    sleep_fn(delay_ms);
                    continue;
                }

                let recovery_artifact = Some(recovery_artifact_for_failure(
                    policy,
                    surface.clone(),
                    attempt_index,
                    &classification,
                    terminal_reason.unwrap_or(RetryTerminalReasonV1::NonRetryableFault),
                ));
                return RetryExecution {
                    result: Err(error),
                    trace: RetryExecutionTraceV1 {
                        schema_version: RESILIENCE_RETRY_EXECUTION_TRACE_SCHEMA_V1.to_string(),
                        policy_id: policy.policy_id.clone(),
                        surface,
                        final_status: RetryExecutionFinalStatusV1::Failed,
                        total_duration_ms: started.elapsed().as_millis() as u64,
                        attempts,
                        telemetry_events,
                        recovery_artifact,
                    },
                };
            }
        }
    }

    unreachable!("retry execution should return inside the attempt loop")
}

impl ResilienceSubstrateManifestV1 {
    pub fn phase1_foundation() -> Self {
        Self {
            schema_version: RESILIENCE_SUBSTRATE_SCHEMA_V1.to_string(),
            manifest_id: "phase1_resilience_substrate_foundation".to_string(),
            supported_surfaces: vec![
                ResilienceSurfaceV1::Provider,
                ResilienceSurfaceV1::Tool,
                ResilienceSurfaceV1::Workflow,
                ResilienceSurfaceV1::CitizenRuntime,
            ],
            fault_schema_ref: RESILIENCE_FAULT_CLASSIFICATION_SCHEMA_V1.to_string(),
            citizen_health_schema_ref: RESILIENCE_CITIZEN_HEALTH_SCHEMA_V1.to_string(),
            recovery_artifact_schema_ref: RESILIENCE_RECOVERY_ARTIFACT_SCHEMA_V1.to_string(),
            checkpoint_schema_ref: RESILIENCE_CHECKPOINT_SCHEMA_V1.to_string(),
            telemetry_schema_ref: RESILIENCE_TELEMETRY_EVENT_SCHEMA_V1.to_string(),
            policy: ResiliencePolicyV1::provider_attempt_policy(
                "provider_attempt_default",
                3,
                30_000,
            ),
        }
    }
}

pub fn execute_timeout_policy<T, E, F, C, TO, CO>(
    policy: &ResiliencePolicyV1,
    surface: ResilienceSurfaceV1,
    operation_ref: &str,
    operation: F,
    mut classify_error: C,
    mut timeout_error: TO,
    mut cancellation_error: CO,
) -> TimeoutExecution<T, E>
where
    F: FnOnce() -> TimeoutObservation<T, E>,
    C: FnMut(&E) -> ResilienceFaultClassificationV1,
    TO: FnMut(TimeoutBreachKindV1, u64, u64) -> E,
    CO: FnMut(u64) -> E,
{
    let observation = operation();
    let timeout_ms = policy.timeout.as_ref().map(|timeout| timeout.timeout_ms);
    let hard_deadline_ms = policy
        .timeout
        .as_ref()
        .and_then(|timeout| timeout.hard_deadline_ms);
    let breach = timeout_breach(timeout_ms, hard_deadline_ms, observation.elapsed_ms);

    if observation.cancelled {
        let error = cancellation_error(observation.elapsed_ms);
        let fault = timeout_cancellation_fault(surface.clone(), operation_ref);
        let decision_summary = format!(
            "{operation_ref}: operation cancelled after {}ms",
            observation.elapsed_ms
        );
        let telemetry_event = Some(timeout_decision_event(
            policy,
            surface.clone(),
            operation_ref,
            &decision_summary,
            Some(fault.clone()),
        ));
        let recovery_artifact = Some(timeout_recovery_artifact(
            policy,
            surface.clone(),
            operation_ref,
            &fault,
            RecoveryDispositionV1::ResumeAllowed,
            "handle explicit cancellation before retrying or rescheduling",
        ));
        return TimeoutExecution {
            result: Err(error),
            trace: TimeoutExecutionTraceV1 {
                schema_version: RESILIENCE_TIMEOUT_EXECUTION_TRACE_SCHEMA_V1.to_string(),
                policy_id: policy.policy_id.clone(),
                surface,
                final_status: TimeoutExecutionFinalStatusV1::Cancelled,
                elapsed_ms: observation.elapsed_ms,
                timeout_ms,
                hard_deadline_ms,
                breach_kind: None,
                decision_summary,
                fault: Some(fault),
                telemetry_event,
                recovery_artifact,
            },
        };
    }

    match observation.result {
        Ok(value) => {
            if let Some((breach_kind, breached_budget_ms)) = breach.clone() {
                let error = timeout_error(
                    breach_kind.clone(),
                    observation.elapsed_ms,
                    breached_budget_ms,
                );
                let fault = timeout_deadline_fault(
                    surface.clone(),
                    operation_ref,
                    observation.elapsed_ms,
                    breach_kind.clone(),
                    breached_budget_ms,
                );
                let decision_summary = format!(
                    "{operation_ref}: {} exceeded after {}ms (budget {}ms)",
                    timeout_breach_label(&breach_kind),
                    observation.elapsed_ms,
                    breached_budget_ms
                );
                let telemetry_event = Some(timeout_decision_event(
                    policy,
                    surface.clone(),
                    operation_ref,
                    &decision_summary,
                    Some(fault.clone()),
                ));
                let recovery_artifact = Some(timeout_recovery_artifact(
                    policy,
                    surface.clone(),
                    operation_ref,
                    &fault,
                    RecoveryDispositionV1::RetryAllowed,
                    "operation exceeded deadline; retry only through the caller's bounded policy",
                ));
                return TimeoutExecution {
                    result: Err(error),
                    trace: TimeoutExecutionTraceV1 {
                        schema_version: RESILIENCE_TIMEOUT_EXECUTION_TRACE_SCHEMA_V1.to_string(),
                        policy_id: policy.policy_id.clone(),
                        surface,
                        final_status: TimeoutExecutionFinalStatusV1::TimedOut,
                        elapsed_ms: observation.elapsed_ms,
                        timeout_ms,
                        hard_deadline_ms,
                        breach_kind: Some(breach_kind),
                        decision_summary,
                        fault: Some(fault),
                        telemetry_event,
                        recovery_artifact,
                    },
                };
            }

            let decision_summary =
                format!("{operation_ref}: completed before timeout/deadline budget");
            let telemetry_event = timeout_ms.map(|_| {
                timeout_decision_event(
                    policy,
                    surface.clone(),
                    operation_ref,
                    &decision_summary,
                    None,
                )
            });
            TimeoutExecution {
                result: Ok(value),
                trace: TimeoutExecutionTraceV1 {
                    schema_version: RESILIENCE_TIMEOUT_EXECUTION_TRACE_SCHEMA_V1.to_string(),
                    policy_id: policy.policy_id.clone(),
                    surface,
                    final_status: TimeoutExecutionFinalStatusV1::Succeeded,
                    elapsed_ms: observation.elapsed_ms,
                    timeout_ms,
                    hard_deadline_ms,
                    breach_kind: None,
                    decision_summary,
                    fault: None,
                    telemetry_event,
                    recovery_artifact: None,
                },
            }
        }
        Err(error) => {
            let classification = classify_error(&error);
            let timed_out = classification_represents_timeout(&classification);
            let final_status = if timed_out {
                TimeoutExecutionFinalStatusV1::TimedOut
            } else {
                TimeoutExecutionFinalStatusV1::Failed
            };
            let decision_summary = if timed_out {
                let budget_summary = breach
                    .as_ref()
                    .map(|(kind, ms)| format!(" ({}, {}ms)", timeout_breach_label(kind), ms))
                    .unwrap_or_default();
                format!(
                    "{operation_ref}: timeout failure after {}ms{}",
                    observation.elapsed_ms, budget_summary
                )
            } else if let Some((kind, budget_ms)) = breach.as_ref() {
                format!(
                    "{operation_ref}: failed after {} exceeded ({}ms budget) with {:?}",
                    timeout_breach_label(kind),
                    budget_ms,
                    classification.fault_class
                )
            } else {
                format!(
                    "{operation_ref}: failed before deadline with {:?}",
                    classification.fault_class
                )
            };
            let telemetry_event = Some(timeout_decision_event(
                policy,
                surface.clone(),
                operation_ref,
                &decision_summary,
                Some(classification.clone()),
            ));
            let recovery_artifact = if timed_out {
                Some(timeout_recovery_artifact(
                    policy,
                    surface.clone(),
                    operation_ref,
                    &classification,
                    RecoveryDispositionV1::RetryAllowed,
                    "timeout classified distinctly from business failure; retry only through the caller's bounded policy",
                ))
            } else {
                None
            };
            TimeoutExecution {
                result: Err(error),
                trace: TimeoutExecutionTraceV1 {
                    schema_version: RESILIENCE_TIMEOUT_EXECUTION_TRACE_SCHEMA_V1.to_string(),
                    policy_id: policy.policy_id.clone(),
                    surface,
                    final_status,
                    elapsed_ms: observation.elapsed_ms,
                    timeout_ms,
                    hard_deadline_ms,
                    breach_kind: if timed_out {
                        breach.as_ref().map(|(kind, _)| kind.clone())
                    } else {
                        None
                    },
                    decision_summary,
                    fault: Some(classification),
                    telemetry_event,
                    recovery_artifact,
                },
            }
        }
    }
}

pub fn circuit_breaker_initial_state(policy: &ResiliencePolicyV1) -> CircuitBreakerStateV1 {
    CircuitBreakerStateV1 {
        schema_version: RESILIENCE_CIRCUIT_BREAKER_STATE_SCHEMA_V1.to_string(),
        policy_id: policy.policy_id.clone(),
        state: CircuitBreakerStateKindV1::Closed,
        consecutive_failures: 0,
        half_open_attempts: 0,
        opened_at_ms: None,
        last_failure: None,
    }
}

#[allow(clippy::too_many_arguments)]
pub fn execute_circuit_breaker_policy<T, E, F, C, R, FB>(
    policy: &ResiliencePolicyV1,
    surface: ResilienceSurfaceV1,
    operation_ref: &str,
    current_state: &CircuitBreakerStateV1,
    now_ms: u64,
    operation: F,
    mut classify_error: C,
    mut rejection_error: R,
    mut fallback: Option<FB>,
) -> CircuitBreakerExecution<T, E>
where
    F: FnOnce() -> Result<T, E>,
    C: FnMut(&E) -> ResilienceFaultClassificationV1,
    R: FnMut(&CircuitBreakerStateV1, u64) -> E,
    FB: FnMut() -> T,
{
    let policy_state = circuit_breaker_state_for_policy(current_state, policy);
    let Some(breaker_policy) = policy.circuit_breaker.as_ref() else {
        let result = operation();
        let state = circuit_breaker_initial_state(policy);
        let fault = result.as_ref().err().map(&mut classify_error);
        let final_status = if result.is_ok() {
            CircuitBreakerFinalStatusV1::ClosedSuccess
        } else {
            CircuitBreakerFinalStatusV1::ClosedFailure
        };
        let decision_summary = if result.is_ok() {
            format!("{operation_ref}: breaker disabled; operation completed")
        } else {
            format!("{operation_ref}: breaker disabled; operation failed")
        };
        let telemetry_event = Some(circuit_breaker_decision_event(
            policy,
            surface.clone(),
            operation_ref,
            &decision_summary,
            fault.clone(),
        ));
        return CircuitBreakerExecution {
            result,
            state: state.clone(),
            trace: CircuitBreakerExecutionTraceV1 {
                schema_version: RESILIENCE_CIRCUIT_BREAKER_EXECUTION_TRACE_SCHEMA_V1.to_string(),
                policy_id: policy.policy_id.clone(),
                surface,
                state_before: CircuitBreakerStateKindV1::Closed,
                state_after: CircuitBreakerStateKindV1::Closed,
                final_status,
                operation_executed: true,
                used_fallback: false,
                now_ms,
                decision_summary,
                fault,
                telemetry_event,
                recovery_artifact: None,
            },
        };
    };

    let state_before = policy_state.state.clone();
    let normalized_state = circuit_breaker_state_for_now(&policy_state, breaker_policy, now_ms);
    let fallback_allowed = normalized_state
        .last_failure
        .as_ref()
        .map(|fault| circuit_breaker_fallback_allowed(policy, fault))
        .unwrap_or(false);

    match normalized_state.state {
        CircuitBreakerStateKindV1::Open => {
            if fallback_allowed {
                if let Some(ref mut fallback_fn) = fallback {
                    let value = fallback_fn();
                    let decision_summary = format!(
                        "{operation_ref}: breaker open at {} failures; fallback executed",
                        normalized_state.consecutive_failures
                    );
                    let telemetry_event = Some(circuit_breaker_decision_event(
                        policy,
                        surface.clone(),
                        operation_ref,
                        &decision_summary,
                        normalized_state.last_failure.clone(),
                    ));
                    let recovery_artifact = normalized_state.last_failure.as_ref().map(|fault| {
                        circuit_breaker_recovery_artifact(
                            policy,
                            surface.clone(),
                            operation_ref,
                            fault,
                            RecoveryDispositionV1::FallbackAllowed,
                            "breaker remained open; fallback path executed instead of calling the dependency",
                        )
                    });
                    return CircuitBreakerExecution {
                        result: Ok(value),
                        state: normalized_state.clone(),
                        trace: CircuitBreakerExecutionTraceV1 {
                            schema_version: RESILIENCE_CIRCUIT_BREAKER_EXECUTION_TRACE_SCHEMA_V1
                                .to_string(),
                            policy_id: policy.policy_id.clone(),
                            surface,
                            state_before,
                            state_after: normalized_state.state.clone(),
                            final_status: CircuitBreakerFinalStatusV1::OpenFallback,
                            operation_executed: false,
                            used_fallback: true,
                            now_ms,
                            decision_summary,
                            fault: normalized_state.last_failure.clone(),
                            telemetry_event,
                            recovery_artifact,
                        },
                    };
                }
            }

            let error = rejection_error(&normalized_state, now_ms);
            let decision_summary = if fallback.is_some() && normalized_state.last_failure.is_some()
            {
                format!(
                    "{operation_ref}: breaker open at {} failures; fallback policy did not activate",
                    normalized_state.consecutive_failures
                )
            } else {
                format!(
                    "{operation_ref}: breaker open at {} failures; dependency call rejected",
                    normalized_state.consecutive_failures
                )
            };
            let telemetry_event = Some(circuit_breaker_decision_event(
                policy,
                surface.clone(),
                operation_ref,
                &decision_summary,
                normalized_state.last_failure.clone(),
            ));
            let recovery_artifact = normalized_state.last_failure.as_ref().map(|fault| {
                circuit_breaker_recovery_artifact(
                    policy,
                    surface.clone(),
                    operation_ref,
                    fault,
                    RecoveryDispositionV1::RetryAllowed,
                    "breaker remained open; wait for the recovery window before probing again",
                )
            });
            return CircuitBreakerExecution {
                result: Err(error),
                state: normalized_state.clone(),
                trace: CircuitBreakerExecutionTraceV1 {
                    schema_version: RESILIENCE_CIRCUIT_BREAKER_EXECUTION_TRACE_SCHEMA_V1
                        .to_string(),
                    policy_id: policy.policy_id.clone(),
                    surface,
                    state_before,
                    state_after: normalized_state.state.clone(),
                    final_status: CircuitBreakerFinalStatusV1::OpenRejected,
                    operation_executed: false,
                    used_fallback: false,
                    now_ms,
                    decision_summary,
                    fault: normalized_state.last_failure.clone(),
                    telemetry_event,
                    recovery_artifact,
                },
            };
        }
        CircuitBreakerStateKindV1::HalfOpen
            if normalized_state.half_open_attempts >= breaker_policy.half_open_max_attempts =>
        {
            let error = rejection_error(&normalized_state, now_ms);
            let decision_summary = format!(
                "{operation_ref}: half-open probe budget exhausted ({}/{})",
                normalized_state.half_open_attempts, breaker_policy.half_open_max_attempts
            );
            let telemetry_event = Some(circuit_breaker_decision_event(
                policy,
                surface.clone(),
                operation_ref,
                &decision_summary,
                normalized_state.last_failure.clone(),
            ));
            let recovery_artifact = normalized_state.last_failure.as_ref().map(|fault| {
                circuit_breaker_recovery_artifact(
                    policy,
                    surface.clone(),
                    operation_ref,
                    fault,
                    RecoveryDispositionV1::RetryAllowed,
                    "half-open probe limit reached; wait for the next recovery window",
                )
            });
            return CircuitBreakerExecution {
                result: Err(error),
                state: normalized_state.clone(),
                trace: CircuitBreakerExecutionTraceV1 {
                    schema_version: RESILIENCE_CIRCUIT_BREAKER_EXECUTION_TRACE_SCHEMA_V1
                        .to_string(),
                    policy_id: policy.policy_id.clone(),
                    surface,
                    state_before,
                    state_after: normalized_state.state.clone(),
                    final_status: CircuitBreakerFinalStatusV1::HalfOpenProbeRejected,
                    operation_executed: false,
                    used_fallback: false,
                    now_ms,
                    decision_summary,
                    fault: normalized_state.last_failure.clone(),
                    telemetry_event,
                    recovery_artifact,
                },
            };
        }
        _ => {}
    }

    let mut state_after = normalized_state.clone();
    let half_open_probe_attempt = if normalized_state.state == CircuitBreakerStateKindV1::HalfOpen {
        let next_attempt = normalized_state.half_open_attempts.saturating_add(1);
        state_after.half_open_attempts = next_attempt;
        Some(next_attempt)
    } else {
        None
    };
    let result = operation();
    match result {
        Ok(value) => {
            let final_status = if normalized_state.state == CircuitBreakerStateKindV1::HalfOpen {
                CircuitBreakerFinalStatusV1::HalfOpenProbeSuccess
            } else {
                CircuitBreakerFinalStatusV1::ClosedSuccess
            };
            state_after.state = CircuitBreakerStateKindV1::Closed;
            state_after.consecutive_failures = 0;
            state_after.half_open_attempts = 0;
            state_after.opened_at_ms = None;
            state_after.last_failure = None;
            let decision_summary =
                if final_status == CircuitBreakerFinalStatusV1::HalfOpenProbeSuccess {
                    format!("{operation_ref}: half-open probe succeeded; breaker closed")
                } else {
                    format!("{operation_ref}: breaker remained closed after successful call")
                };
            let telemetry_event = Some(circuit_breaker_decision_event(
                policy,
                surface.clone(),
                operation_ref,
                &decision_summary,
                None,
            ));
            CircuitBreakerExecution {
                result: Ok(value),
                state: state_after.clone(),
                trace: CircuitBreakerExecutionTraceV1 {
                    schema_version: RESILIENCE_CIRCUIT_BREAKER_EXECUTION_TRACE_SCHEMA_V1
                        .to_string(),
                    policy_id: policy.policy_id.clone(),
                    surface,
                    state_before,
                    state_after: state_after.state.clone(),
                    final_status,
                    operation_executed: true,
                    used_fallback: false,
                    now_ms,
                    decision_summary,
                    fault: None,
                    telemetry_event,
                    recovery_artifact: None,
                },
            }
        }
        Err(error) => {
            let fault = classify_error(&error);
            let final_status = if normalized_state.state == CircuitBreakerStateKindV1::HalfOpen {
                let probe_attempt = half_open_probe_attempt.unwrap_or(1);
                state_after.consecutive_failures = breaker_policy.failure_threshold;
                state_after.last_failure = Some(fault.clone());
                if probe_attempt >= breaker_policy.half_open_max_attempts {
                    state_after.state = CircuitBreakerStateKindV1::Open;
                    state_after.opened_at_ms = Some(now_ms);
                } else {
                    state_after.state = CircuitBreakerStateKindV1::HalfOpen;
                    state_after.opened_at_ms = None;
                }
                CircuitBreakerFinalStatusV1::HalfOpenProbeFailure
            } else {
                state_after.consecutive_failures =
                    normalized_state.consecutive_failures.saturating_add(1);
                state_after.last_failure = Some(fault.clone());
                if state_after.consecutive_failures >= breaker_policy.failure_threshold {
                    state_after.state = CircuitBreakerStateKindV1::Open;
                    state_after.opened_at_ms = Some(now_ms);
                }
                CircuitBreakerFinalStatusV1::ClosedFailure
            };
            let decision_summary = match final_status {
                CircuitBreakerFinalStatusV1::HalfOpenProbeFailure
                    if state_after.state == CircuitBreakerStateKindV1::Open =>
                {
                    format!("{operation_ref}: half-open probe failed; breaker reopened")
                }
                CircuitBreakerFinalStatusV1::HalfOpenProbeFailure => format!(
                    "{operation_ref}: half-open probe failed; {} probe attempt(s) remain before reopening",
                    breaker_policy
                        .half_open_max_attempts
                        .saturating_sub(state_after.half_open_attempts)
                ),
                _ if state_after.state == CircuitBreakerStateKindV1::Open => format!(
                    "{operation_ref}: breaker opened after {} consecutive failures",
                    state_after.consecutive_failures
                ),
                _ => format!(
                    "{operation_ref}: breaker counted failure {}/{} while remaining closed",
                    state_after.consecutive_failures, breaker_policy.failure_threshold
                ),
            };
            let telemetry_event = Some(circuit_breaker_decision_event(
                policy,
                surface.clone(),
                operation_ref,
                &decision_summary,
                Some(fault.clone()),
            ));
            let recovery_artifact = if state_after.state == CircuitBreakerStateKindV1::Open {
                Some(circuit_breaker_recovery_artifact(
                    policy,
                    surface.clone(),
                    operation_ref,
                    &fault,
                    RecoveryDispositionV1::RetryAllowed,
                    "breaker opened; defer new attempts until the recovery window allows a bounded half-open probe",
                ))
            } else {
                None
            };
            CircuitBreakerExecution {
                result: Err(error),
                state: state_after.clone(),
                trace: CircuitBreakerExecutionTraceV1 {
                    schema_version: RESILIENCE_CIRCUIT_BREAKER_EXECUTION_TRACE_SCHEMA_V1
                        .to_string(),
                    policy_id: policy.policy_id.clone(),
                    surface,
                    state_before,
                    state_after: state_after.state.clone(),
                    final_status,
                    operation_executed: true,
                    used_fallback: false,
                    now_ms,
                    decision_summary,
                    fault: Some(fault),
                    telemetry_event,
                    recovery_artifact,
                },
            }
        }
    }
}

pub fn resilience_schema_smoke() -> Value {
    serde_json::to_value(schema_for!(ResilienceSubstrateManifestV1))
        .expect("resilience substrate schema should serialize")
}

pub(crate) fn sanitize_resilience_summary(note: &str) -> String {
    let text = note.split_whitespace().collect::<Vec<_>>().join(" ");
    let lowered = text.to_ascii_lowercase();
    let sensitive = [
        "authorization",
        "bearer ",
        "x-api-key",
        "key=",
        "api_key=",
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

fn deterministic_jitter_ms(policy_id: &str, attempt_index: u32, max_jitter_ms: u64) -> u64 {
    if max_jitter_ms == 0 {
        return 0;
    }
    let mut hash = 1469598103934665603_u64;
    for byte in policy_id.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(1099511628211);
    }
    hash ^= u64::from(attempt_index);
    hash = hash.wrapping_mul(1099511628211);
    hash % max_jitter_ms.saturating_add(1)
}

fn retry_decision_summary(
    attempt_index: u32,
    classification: &ResilienceFaultClassificationV1,
    retry_allowed: bool,
    delay_ms: u64,
    terminal_reason: Option<&RetryTerminalReasonV1>,
) -> String {
    if retry_allowed {
        format!(
            "attempt {attempt_index} classified {:?}; retry scheduled after {delay_ms}ms",
            classification.fault_class
        )
    } else {
        match terminal_reason {
            Some(reason) => format!(
                "attempt {attempt_index} classified {:?}; terminal reason {:?}",
                classification.fault_class, reason
            ),
            None => format!(
                "attempt {attempt_index} classified {:?}; retry not allowed",
                classification.fault_class
            ),
        }
    }
}

fn retry_decision_event(
    policy: &ResiliencePolicyV1,
    surface: ResilienceSurfaceV1,
    operation_ref: &str,
    attempt_index: u32,
    decision_summary: &str,
    fault: Option<ResilienceFaultClassificationV1>,
) -> ResilienceTelemetryEventV1 {
    ResilienceTelemetryEventV1 {
        schema_version: RESILIENCE_TELEMETRY_EVENT_SCHEMA_V1.to_string(),
        event_id: format!("{}:retry:{attempt_index}", policy.policy_id),
        event_kind: TelemetryEventKindV1::RetryDecision,
        surface,
        decision_summary: format!("{operation_ref}: {decision_summary}"),
        run_id: None,
        request_id: None,
        policy_ref: Some(policy.policy_id.clone()),
        fault,
        artifact_ref: None,
    }
}

fn recovery_artifact_for_failure(
    policy: &ResiliencePolicyV1,
    surface: ResilienceSurfaceV1,
    attempt_index: u32,
    triggering_fault: &ResilienceFaultClassificationV1,
    terminal_reason: RetryTerminalReasonV1,
) -> RecoveryArtifactV1 {
    let (disposition, next_action) = match terminal_reason {
        RetryTerminalReasonV1::RetryBudgetExhausted => (
            RecoveryDispositionV1::OperatorInterventionRequired,
            format!(
                "retry budget exhausted after attempt {attempt_index}; operator must decide whether to widen the retry budget"
            ),
        ),
        RetryTerminalReasonV1::RetryTimeBudgetExhausted => (
            RecoveryDispositionV1::OperatorInterventionRequired,
            format!(
                "retry time budget exhausted after attempt {attempt_index}; operator must decide whether to widen the elapsed budget"
            ),
        ),
        RetryTerminalReasonV1::NonRetryableFault => match triggering_fault.disposition {
            ResilienceFaultDispositionV1::DegradedAllowed => (
                RecoveryDispositionV1::FallbackAllowed,
                "fault is non-retryable here; route to a degraded or fallback path".to_string(),
            ),
            ResilienceFaultDispositionV1::QuarantineRequired => (
                RecoveryDispositionV1::QuarantineRequired,
                "fault requires quarantine before retrying the surface".to_string(),
            ),
            _ => (
                RecoveryDispositionV1::OperatorInterventionRequired,
                "fault is non-retryable; inspect the failure before attempting recovery".to_string(),
            ),
        },
        RetryTerminalReasonV1::Succeeded => (
            RecoveryDispositionV1::ResumeAllowed,
            "operation completed successfully".to_string(),
        ),
    };
    RecoveryArtifactV1 {
        schema_version: RESILIENCE_RECOVERY_ARTIFACT_SCHEMA_V1.to_string(),
        artifact_id: format!("{}:recovery:{attempt_index}", policy.policy_id),
        surface,
        triggering_fault: triggering_fault.clone(),
        disposition,
        next_action,
        source_run_id: None,
        checkpoint_ref: None,
        evidence_refs: vec![policy.policy_id.clone()],
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

fn timeout_deadline_fault(
    surface: ResilienceSurfaceV1,
    operation_ref: &str,
    elapsed_ms: u64,
    breach_kind: TimeoutBreachKindV1,
    breached_budget_ms: u64,
) -> ResilienceFaultClassificationV1 {
    ResilienceFaultClassificationV1 {
        schema_version: RESILIENCE_FAULT_CLASSIFICATION_SCHEMA_V1.to_string(),
        surface,
        fault_class: ResilienceFaultClassV1::RuntimeFailure,
        disposition: ResilienceFaultDispositionV1::Retryable,
        retryable: true,
        summary: format!(
            "{operation_ref} exceeded {} after {elapsed_ms}ms (budget {breached_budget_ms}ms)",
            timeout_breach_label(&breach_kind)
        ),
        component_ref: Some(operation_ref.to_string()),
        http_status: None,
    }
}

fn timeout_cancellation_fault(
    surface: ResilienceSurfaceV1,
    operation_ref: &str,
) -> ResilienceFaultClassificationV1 {
    ResilienceFaultClassificationV1 {
        schema_version: RESILIENCE_FAULT_CLASSIFICATION_SCHEMA_V1.to_string(),
        surface,
        fault_class: ResilienceFaultClassV1::RuntimeFailure,
        disposition: ResilienceFaultDispositionV1::Terminal,
        retryable: false,
        summary: format!("{operation_ref} cancelled before completion"),
        component_ref: Some(operation_ref.to_string()),
        http_status: None,
    }
}

fn timeout_decision_event(
    policy: &ResiliencePolicyV1,
    surface: ResilienceSurfaceV1,
    operation_ref: &str,
    decision_summary: &str,
    fault: Option<ResilienceFaultClassificationV1>,
) -> ResilienceTelemetryEventV1 {
    let correlation_suffix = timeout_execution_correlation_suffix();
    ResilienceTelemetryEventV1 {
        schema_version: RESILIENCE_TELEMETRY_EVENT_SCHEMA_V1.to_string(),
        event_id: format!(
            "{}:timeout:{operation_ref}:{correlation_suffix}",
            policy.policy_id
        ),
        event_kind: TelemetryEventKindV1::TimeoutDecision,
        surface,
        decision_summary: decision_summary.to_string(),
        run_id: None,
        request_id: None,
        policy_ref: Some(policy.policy_id.clone()),
        fault,
        artifact_ref: None,
    }
}

fn timeout_recovery_artifact(
    policy: &ResiliencePolicyV1,
    surface: ResilienceSurfaceV1,
    operation_ref: &str,
    fault: &ResilienceFaultClassificationV1,
    disposition: RecoveryDispositionV1,
    next_action: &str,
) -> RecoveryArtifactV1 {
    let correlation_suffix = timeout_execution_correlation_suffix();
    RecoveryArtifactV1 {
        schema_version: RESILIENCE_RECOVERY_ARTIFACT_SCHEMA_V1.to_string(),
        artifact_id: format!(
            "{}:timeout:{operation_ref}:{correlation_suffix}",
            policy.policy_id
        ),
        surface,
        triggering_fault: fault.clone(),
        disposition,
        next_action: next_action.to_string(),
        source_run_id: None,
        checkpoint_ref: None,
        evidence_refs: vec![policy.policy_id.clone()],
    }
}

fn timeout_execution_correlation_suffix() -> String {
    TIMEOUT_EXECUTION_COUNTER
        .fetch_add(1, Ordering::Relaxed)
        .saturating_add(1)
        .to_string()
}

fn timeout_breach(
    timeout_ms: Option<u64>,
    hard_deadline_ms: Option<u64>,
    elapsed_ms: u64,
) -> Option<(TimeoutBreachKindV1, u64)> {
    let mut budgets = Vec::new();
    if let Some(timeout_ms) = timeout_ms {
        budgets.push((timeout_ms, TimeoutBreachKindV1::Timeout));
    }
    if let Some(hard_deadline_ms) = hard_deadline_ms {
        budgets.push((hard_deadline_ms, TimeoutBreachKindV1::HardDeadline));
    }
    budgets.sort_by_key(|(budget_ms, _)| *budget_ms);
    budgets
        .into_iter()
        .find(|(budget_ms, _)| elapsed_ms > *budget_ms)
        .map(|(budget_ms, kind)| (kind, budget_ms))
}

fn timeout_breach_label(kind: &TimeoutBreachKindV1) -> &'static str {
    match kind {
        TimeoutBreachKindV1::Timeout => "timeout budget",
        TimeoutBreachKindV1::HardDeadline => "hard deadline",
    }
}

fn classification_represents_timeout(classification: &ResilienceFaultClassificationV1) -> bool {
    if classification.fault_class == ResilienceFaultClassV1::ProviderTimeout
        || classification.fault_class == ResilienceFaultClassV1::LocalRuntimeHung
    {
        return true;
    }
    if matches!(
        classification.fault_class,
        ResilienceFaultClassV1::RuntimeFailure
            | ResilienceFaultClassV1::WorkflowFailure
            | ResilienceFaultClassV1::ToolFailure
            | ResilienceFaultClassV1::Unknown
    ) {
        let summary = classification.summary.to_ascii_lowercase();
        return summary.contains("timeout")
            || summary.contains("timed out")
            || summary.contains("deadline");
    }
    false
}

fn circuit_breaker_state_for_now(
    state: &CircuitBreakerStateV1,
    policy: &CircuitBreakerPolicyV1,
    now_ms: u64,
) -> CircuitBreakerStateV1 {
    if state.state != CircuitBreakerStateKindV1::Open {
        return state.clone();
    }
    let ready_for_probe = state
        .opened_at_ms
        .map(|opened_at_ms| now_ms.saturating_sub(opened_at_ms) >= policy.recovery_window_ms)
        .unwrap_or(false);
    if !ready_for_probe {
        return state.clone();
    }

    let mut next = state.clone();
    next.state = CircuitBreakerStateKindV1::HalfOpen;
    next.half_open_attempts = 0;
    next
}

fn circuit_breaker_state_for_policy(
    state: &CircuitBreakerStateV1,
    policy: &ResiliencePolicyV1,
) -> CircuitBreakerStateV1 {
    if state.policy_id == policy.policy_id {
        state.clone()
    } else {
        circuit_breaker_initial_state(policy)
    }
}

fn circuit_breaker_fallback_allowed(
    policy: &ResiliencePolicyV1,
    fault: &ResilienceFaultClassificationV1,
) -> bool {
    let Some(fallback_policy) = policy.fallback.as_ref() else {
        return false;
    };
    fallback_policy.activation_fault_classes.is_empty()
        || fallback_policy
            .activation_fault_classes
            .contains(&fault.fault_class)
}

fn circuit_breaker_decision_event(
    policy: &ResiliencePolicyV1,
    surface: ResilienceSurfaceV1,
    operation_ref: &str,
    decision_summary: &str,
    fault: Option<ResilienceFaultClassificationV1>,
) -> ResilienceTelemetryEventV1 {
    let correlation_suffix = circuit_breaker_execution_correlation_suffix();
    ResilienceTelemetryEventV1 {
        schema_version: RESILIENCE_TELEMETRY_EVENT_SCHEMA_V1.to_string(),
        event_id: format!(
            "{}:circuit-breaker:{operation_ref}:{correlation_suffix}",
            policy.policy_id
        ),
        event_kind: TelemetryEventKindV1::CircuitBreakerDecision,
        surface,
        decision_summary: decision_summary.to_string(),
        run_id: None,
        request_id: None,
        policy_ref: Some(policy.policy_id.clone()),
        fault,
        artifact_ref: None,
    }
}

fn circuit_breaker_recovery_artifact(
    policy: &ResiliencePolicyV1,
    surface: ResilienceSurfaceV1,
    operation_ref: &str,
    fault: &ResilienceFaultClassificationV1,
    disposition: RecoveryDispositionV1,
    next_action: &str,
) -> RecoveryArtifactV1 {
    let correlation_suffix = circuit_breaker_execution_correlation_suffix();
    RecoveryArtifactV1 {
        schema_version: RESILIENCE_RECOVERY_ARTIFACT_SCHEMA_V1.to_string(),
        artifact_id: format!(
            "{}:circuit-breaker:{operation_ref}:{correlation_suffix}",
            policy.policy_id
        ),
        surface,
        triggering_fault: fault.clone(),
        disposition,
        next_action: next_action.to_string(),
        source_run_id: None,
        checkpoint_ref: None,
        evidence_refs: vec![policy.policy_id.clone()],
    }
}

fn circuit_breaker_execution_correlation_suffix() -> String {
    CIRCUIT_BREAKER_EXECUTION_COUNTER
        .fetch_add(1, Ordering::Relaxed)
        .saturating_add(1)
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    fn clone_fault_classification(
        error: &ResilienceFaultClassificationV1,
    ) -> ResilienceFaultClassificationV1 {
        error.clone()
    }

    fn workflow_timeout_fault(
        breach_kind: TimeoutBreachKindV1,
        elapsed_ms: u64,
        budget_ms: u64,
    ) -> ResilienceFaultClassificationV1 {
        ResilienceFaultClassificationV1 {
            schema_version: RESILIENCE_FAULT_CLASSIFICATION_SCHEMA_V1.to_string(),
            surface: ResilienceSurfaceV1::Workflow,
            fault_class: ResilienceFaultClassV1::RuntimeFailure,
            disposition: ResilienceFaultDispositionV1::Retryable,
            retryable: true,
            summary: format!(
                "{} exceeded at {elapsed_ms}/{budget_ms}",
                timeout_breach_label(&breach_kind)
            ),
            component_ref: None,
            http_status: None,
        }
    }

    fn provider_timeout_fault(
        breach_kind: TimeoutBreachKindV1,
        elapsed_ms: u64,
        budget_ms: u64,
    ) -> ResilienceFaultClassificationV1 {
        ResilienceFaultClassificationV1 {
            schema_version: RESILIENCE_FAULT_CLASSIFICATION_SCHEMA_V1.to_string(),
            surface: ResilienceSurfaceV1::Provider,
            fault_class: ResilienceFaultClassV1::ProviderTimeout,
            disposition: ResilienceFaultDispositionV1::Retryable,
            retryable: true,
            summary: format!(
                "{} exceeded at {elapsed_ms}/{budget_ms}",
                timeout_breach_label(&breach_kind)
            ),
            component_ref: None,
            http_status: None,
        }
    }

    fn tool_timeout_fault(
        breach_kind: TimeoutBreachKindV1,
        elapsed_ms: u64,
        budget_ms: u64,
    ) -> ResilienceFaultClassificationV1 {
        ResilienceFaultClassificationV1 {
            schema_version: RESILIENCE_FAULT_CLASSIFICATION_SCHEMA_V1.to_string(),
            surface: ResilienceSurfaceV1::Tool,
            fault_class: ResilienceFaultClassV1::RuntimeFailure,
            disposition: ResilienceFaultDispositionV1::Retryable,
            retryable: true,
            summary: format!(
                "{} exceeded at {elapsed_ms}/{budget_ms}",
                timeout_breach_label(&breach_kind)
            ),
            component_ref: None,
            http_status: None,
        }
    }

    fn runtime_timeout_fault(
        breach_kind: TimeoutBreachKindV1,
        elapsed_ms: u64,
        budget_ms: u64,
    ) -> ResilienceFaultClassificationV1 {
        ResilienceFaultClassificationV1 {
            schema_version: RESILIENCE_FAULT_CLASSIFICATION_SCHEMA_V1.to_string(),
            surface: ResilienceSurfaceV1::Runtime,
            fault_class: ResilienceFaultClassV1::RuntimeFailure,
            disposition: ResilienceFaultDispositionV1::Retryable,
            retryable: true,
            summary: format!(
                "{} exceeded at {elapsed_ms}/{budget_ms}",
                timeout_breach_label(&breach_kind)
            ),
            component_ref: None,
            http_status: None,
        }
    }

    fn tool_cancelled_fault(elapsed_ms: u64) -> ResilienceFaultClassificationV1 {
        ResilienceFaultClassificationV1 {
            schema_version: RESILIENCE_FAULT_CLASSIFICATION_SCHEMA_V1.to_string(),
            surface: ResilienceSurfaceV1::Tool,
            fault_class: ResilienceFaultClassV1::RuntimeFailure,
            disposition: ResilienceFaultDispositionV1::Terminal,
            retryable: false,
            summary: format!("cancelled at {elapsed_ms}"),
            component_ref: None,
            http_status: None,
        }
    }

    fn workflow_cancelled_fault(elapsed_ms: u64) -> ResilienceFaultClassificationV1 {
        ResilienceFaultClassificationV1 {
            schema_version: RESILIENCE_FAULT_CLASSIFICATION_SCHEMA_V1.to_string(),
            surface: ResilienceSurfaceV1::Workflow,
            fault_class: ResilienceFaultClassV1::RuntimeFailure,
            disposition: ResilienceFaultDispositionV1::Terminal,
            retryable: false,
            summary: format!("cancelled at {elapsed_ms}"),
            component_ref: None,
            http_status: None,
        }
    }

    fn provider_cancelled_fault(elapsed_ms: u64) -> ResilienceFaultClassificationV1 {
        ResilienceFaultClassificationV1 {
            schema_version: RESILIENCE_FAULT_CLASSIFICATION_SCHEMA_V1.to_string(),
            surface: ResilienceSurfaceV1::Provider,
            fault_class: ResilienceFaultClassV1::RuntimeFailure,
            disposition: ResilienceFaultDispositionV1::Terminal,
            retryable: false,
            summary: format!("cancelled at {elapsed_ms}"),
            component_ref: None,
            http_status: None,
        }
    }

    fn runtime_cancelled_fault(elapsed_ms: u64) -> ResilienceFaultClassificationV1 {
        ResilienceFaultClassificationV1 {
            schema_version: RESILIENCE_FAULT_CLASSIFICATION_SCHEMA_V1.to_string(),
            surface: ResilienceSurfaceV1::Runtime,
            fault_class: ResilienceFaultClassV1::RuntimeFailure,
            disposition: ResilienceFaultDispositionV1::Terminal,
            retryable: false,
            summary: format!("cancelled at {elapsed_ms}"),
            component_ref: None,
            http_status: None,
        }
    }

    fn provider_breaker_rejection(
        state: &CircuitBreakerStateV1,
        now_ms: u64,
    ) -> ResilienceFaultClassificationV1 {
        ResilienceFaultClassificationV1 {
            schema_version: RESILIENCE_FAULT_CLASSIFICATION_SCHEMA_V1.to_string(),
            surface: ResilienceSurfaceV1::Provider,
            fault_class: ResilienceFaultClassV1::ProviderTimeout,
            disposition: ResilienceFaultDispositionV1::Retryable,
            retryable: true,
            summary: format!(
                "breaker open at {} after {}ms",
                state.consecutive_failures, now_ms
            ),
            component_ref: None,
            http_status: None,
        }
    }

    fn provider_breaker_probe_rejection(
        state: &CircuitBreakerStateV1,
        now_ms: u64,
    ) -> ResilienceFaultClassificationV1 {
        ResilienceFaultClassificationV1 {
            schema_version: RESILIENCE_FAULT_CLASSIFICATION_SCHEMA_V1.to_string(),
            surface: ResilienceSurfaceV1::Provider,
            fault_class: ResilienceFaultClassV1::ProviderTimeout,
            disposition: ResilienceFaultDispositionV1::Retryable,
            retryable: true,
            summary: format!(
                "breaker probe rejected at {} after {}ms",
                state.half_open_attempts, now_ms
            ),
            component_ref: None,
            http_status: None,
        }
    }

    fn test_circuit_breaker_policy() -> ResiliencePolicyV1 {
        ResiliencePolicyV1 {
            schema_version: RESILIENCE_POLICY_SCHEMA_V1.to_string(),
            policy_id: "breaker.policy".to_string(),
            retry: Some(RetryPolicyV1 {
                max_attempts: 3,
                backoff_ms: Some(25),
                jitter_ms: Some(5),
                max_elapsed_ms: None,
                retryable_fault_classes: vec![
                    ResilienceFaultClassV1::ProviderTimeout,
                    ResilienceFaultClassV1::ProviderTransientHttp,
                ],
            }),
            timeout: Some(TimeoutPolicyV1 {
                timeout_ms: 100,
                hard_deadline_ms: Some(150),
            }),
            circuit_breaker: Some(CircuitBreakerPolicyV1 {
                failure_threshold: 2,
                recovery_window_ms: 30,
                half_open_max_attempts: 1,
            }),
            rate_limit: None,
            bulkhead: None,
            fallback: Some(FallbackPolicyV1 {
                fallback_ref: "test.fallback".to_string(),
                activation_fault_classes: vec![ResilienceFaultClassV1::ProviderTimeout],
                marks_output_degraded: true,
            }),
            checkpoint_required: false,
            telemetry_required: true,
        }
    }

    #[test]
    fn provider_fault_classifier_emits_retryable_timeout() {
        let fault = ResilienceFaultClassificationV1::provider("provider timeout", None);
        assert_eq!(fault.fault_class, ResilienceFaultClassV1::ProviderTimeout);
        assert_eq!(fault.disposition, ResilienceFaultDispositionV1::Retryable);
        assert!(fault.retryable);
    }

    #[test]
    fn provider_fault_classifier_emits_operator_gated_auth_missing() {
        let fault = ResilienceFaultClassificationV1::provider(
            "missing required environment variable OPENAI_API_KEY",
            None,
        );
        assert_eq!(
            fault.fault_class,
            ResilienceFaultClassV1::ProviderAuthMissing
        );
        assert_eq!(
            fault.disposition,
            ResilienceFaultDispositionV1::OperatorGated
        );
        assert!(!fault.retryable);
    }

    #[test]
    fn provider_fault_classifier_covers_remaining_provider_fault_branches() {
        let cases = [
            (
                "provider rate limit exceeded",
                None,
                ResilienceFaultClassV1::ProviderRateLimited,
                ResilienceFaultDispositionV1::Retryable,
                true,
            ),
            (
                "provider timeout while waiting for upstream",
                None,
                ResilienceFaultClassV1::ProviderTimeout,
                ResilienceFaultDispositionV1::Retryable,
                true,
            ),
            (
                "billing blocked due to credit balance",
                None,
                ResilienceFaultClassV1::ProviderBillingBlocked,
                ResilienceFaultDispositionV1::OperatorGated,
                false,
            ),
            (
                "local_runtime_busy because this is a non-target model",
                None,
                ResilienceFaultClassV1::LocalRuntimeBusy,
                ResilienceFaultDispositionV1::Retryable,
                true,
            ),
            (
                "local_runtime_hung while stopping...",
                None,
                ResilienceFaultClassV1::LocalRuntimeHung,
                ResilienceFaultDispositionV1::Retryable,
                true,
            ),
            (
                "ollama not running: connection refused",
                None,
                ResilienceFaultClassV1::LocalRuntimeUnavailable,
                ResilienceFaultDispositionV1::Retryable,
                true,
            ),
            (
                "provider model not found",
                None,
                ResilienceFaultClassV1::ProviderModelUnavailable,
                ResilienceFaultDispositionV1::Terminal,
                false,
            ),
            (
                "empty provider response output",
                None,
                ResilienceFaultClassV1::ProviderEmptyTextOutput,
                ResilienceFaultDispositionV1::Terminal,
                false,
            ),
            (
                "upstream exploded",
                Some(503),
                ResilienceFaultClassV1::ProviderTransientHttp,
                ResilienceFaultDispositionV1::Retryable,
                true,
            ),
            (
                "provider_internal_error",
                Some(418),
                ResilienceFaultClassV1::ProviderError,
                ResilienceFaultDispositionV1::Terminal,
                false,
            ),
            (
                "something ambiguous happened",
                None,
                ResilienceFaultClassV1::Unknown,
                ResilienceFaultDispositionV1::Retryable,
                true,
            ),
        ];

        for (note, http_status, expected_class, expected_disposition, expected_retryable) in cases {
            let fault = ResilienceFaultClassificationV1::provider(note, http_status);
            assert_eq!(fault.fault_class, expected_class, "{note}");
            assert_eq!(fault.disposition, expected_disposition, "{note}");
            assert_eq!(fault.retryable, expected_retryable, "{note}");
        }
    }

    #[test]
    fn resilience_foundation_defaults_stay_wired_to_phase1_contract() {
        let policy =
            ResiliencePolicyV1::provider_attempt_policy("provider_attempt_default", 3, 30_000);
        let retry = policy.retry.as_ref().expect("retry policy");
        let timeout = policy.timeout.as_ref().expect("timeout policy");
        assert_eq!(policy.schema_version, RESILIENCE_POLICY_SCHEMA_V1);
        assert_eq!(retry.max_attempts, 3);
        assert_eq!(retry.backoff_ms, None);
        assert_eq!(retry.jitter_ms, None);
        assert!(retry
            .retryable_fault_classes
            .contains(&ResilienceFaultClassV1::ProviderRateLimited));
        assert!(retry
            .retryable_fault_classes
            .contains(&ResilienceFaultClassV1::LocalRuntimeHung));
        assert_eq!(timeout.timeout_ms, 30_000);
        assert_eq!(timeout.hard_deadline_ms, None);
        assert!(policy.circuit_breaker.is_none());
        assert!(policy.rate_limit.is_none());
        assert!(policy.bulkhead.is_none());
        assert!(policy.fallback.is_none());
        assert!(!policy.checkpoint_required);
        assert!(policy.telemetry_required);

        let manifest = ResilienceSubstrateManifestV1::phase1_foundation();
        assert_eq!(manifest.schema_version, RESILIENCE_SUBSTRATE_SCHEMA_V1);
        assert_eq!(
            manifest.supported_surfaces,
            vec![
                ResilienceSurfaceV1::Provider,
                ResilienceSurfaceV1::Tool,
                ResilienceSurfaceV1::Workflow,
                ResilienceSurfaceV1::CitizenRuntime,
            ]
        );
        assert_eq!(
            manifest.policy,
            ResiliencePolicyV1::provider_attempt_policy("provider_attempt_default", 3, 30_000)
        );
    }

    #[test]
    fn phase1_manifest_references_all_required_schema_surfaces() {
        let manifest = ResilienceSubstrateManifestV1::phase1_foundation();
        assert_eq!(manifest.schema_version, RESILIENCE_SUBSTRATE_SCHEMA_V1);
        assert_eq!(
            manifest.fault_schema_ref,
            RESILIENCE_FAULT_CLASSIFICATION_SCHEMA_V1
        );
        assert_eq!(
            manifest.citizen_health_schema_ref,
            RESILIENCE_CITIZEN_HEALTH_SCHEMA_V1
        );
        assert_eq!(
            manifest.recovery_artifact_schema_ref,
            RESILIENCE_RECOVERY_ARTIFACT_SCHEMA_V1
        );
        assert_eq!(
            manifest.checkpoint_schema_ref,
            RESILIENCE_CHECKPOINT_SCHEMA_V1
        );
        assert_eq!(
            manifest.telemetry_schema_ref,
            RESILIENCE_TELEMETRY_EVENT_SCHEMA_V1
        );
        assert!(manifest
            .supported_surfaces
            .contains(&ResilienceSurfaceV1::CitizenRuntime));
    }

    #[test]
    fn schema_smoke_contains_manifest_title() {
        let schema = resilience_schema_smoke();
        let title = schema
            .get("title")
            .and_then(Value::as_str)
            .unwrap_or_default();
        assert_eq!(title, "ResilienceSubstrateManifestV1");
    }

    #[test]
    fn provider_fault_summary_redacts_secret_like_content() {
        let classification = ResilienceFaultClassificationV1::provider(
            "request failed with key=super-secret-token prompt: send money",
            None,
        );
        assert_eq!(classification.summary, "redacted provider diagnostic");
        assert!(!classification.summary.contains("super-secret-token"));
    }

    #[test]
    fn provider_fault_summary_normalizes_whitespace_and_truncates_long_messages() {
        let note = format!("{}\n  {}", "word ".repeat(40), "tail");
        let summary = sanitize_resilience_summary(&note);
        assert!(!summary.contains('\n'));
        assert!(!summary.contains("  "));
        assert!(summary.ends_with("..."));
        assert_eq!(summary.chars().count(), 180);
    }

    #[test]
    fn fault_classification_round_trips_with_snake_case_schema_values() {
        let classification =
            ResilienceFaultClassificationV1::provider("provider timeout", Some(504));
        let json = serde_json::to_value(&classification).expect("serialize classification");
        assert_eq!(json["surface"], "provider");
        assert_eq!(json["fault_class"], "provider_timeout");
        assert_eq!(json["disposition"], "retryable");
        let reparsed: ResilienceFaultClassificationV1 =
            serde_json::from_value(json).expect("round trip classification");
        assert_eq!(reparsed, classification);
    }

    #[test]
    fn execute_timeout_policy_succeeds_before_deadline() {
        let policy = ResiliencePolicyV1::provider_attempt_policy("timeout.success", 1, 100);
        let execution = execute_timeout_policy(
            &policy,
            ResilienceSurfaceV1::Tool,
            "test.timeout.success",
            || TimeoutObservation {
                result: Ok("ok"),
                elapsed_ms: 40,
                cancelled: false,
            },
            clone_fault_classification,
            tool_timeout_fault,
            tool_cancelled_fault,
        );

        assert_eq!(execution.result.expect("success"), "ok");
        assert_eq!(
            execution.trace.final_status,
            TimeoutExecutionFinalStatusV1::Succeeded
        );
        assert_eq!(execution.trace.timeout_ms, Some(100));
        assert_eq!(execution.trace.hard_deadline_ms, None);
        assert!(execution.trace.recovery_artifact.is_none());
    }

    #[test]
    fn execute_timeout_policy_emits_timeout_artifact_when_timeout_budget_is_exceeded() {
        let policy = ResiliencePolicyV1::provider_attempt_policy("timeout.deadline", 1, 50);
        let execution = execute_timeout_policy(
            &policy,
            ResilienceSurfaceV1::Workflow,
            "test.timeout.deadline",
            || TimeoutObservation {
                result: Ok::<(), ResilienceFaultClassificationV1>(()),
                elapsed_ms: 75,
                cancelled: false,
            },
            clone_fault_classification,
            workflow_timeout_fault,
            workflow_cancelled_fault,
        );

        let failure = execution.result.expect_err("timeout failure");
        assert!(failure.retryable);
        assert_eq!(
            execution.trace.final_status,
            TimeoutExecutionFinalStatusV1::TimedOut
        );
        assert_eq!(
            execution.trace.breach_kind,
            Some(TimeoutBreachKindV1::Timeout)
        );
        assert_eq!(
            execution.trace.schema_version,
            RESILIENCE_TIMEOUT_EXECUTION_TRACE_SCHEMA_V1
        );
        assert_eq!(
            execution
                .trace
                .telemetry_event
                .as_ref()
                .map(|event| event.event_kind.clone()),
            Some(TelemetryEventKindV1::TimeoutDecision)
        );
        assert_eq!(
            execution
                .trace
                .recovery_artifact
                .as_ref()
                .map(|artifact| artifact.disposition.clone()),
            Some(RecoveryDispositionV1::RetryAllowed)
        );
    }

    #[test]
    fn execute_timeout_policy_distinguishes_timeout_budget_from_hard_deadline() {
        let mut policy = ResiliencePolicyV1::provider_attempt_policy("timeout.budgets", 1, 50);
        policy
            .timeout
            .as_mut()
            .expect("timeout policy")
            .hard_deadline_ms = Some(90);
        let execution = execute_timeout_policy(
            &policy,
            ResilienceSurfaceV1::Workflow,
            "test.timeout.budgets",
            || TimeoutObservation {
                result: Ok::<(), ResilienceFaultClassificationV1>(()),
                elapsed_ms: 60,
                cancelled: false,
            },
            clone_fault_classification,
            workflow_timeout_fault,
            workflow_cancelled_fault,
        );

        assert!(execution.result.is_err());
        assert_eq!(
            execution.trace.breach_kind,
            Some(TimeoutBreachKindV1::Timeout)
        );
        assert_eq!(execution.trace.timeout_ms, Some(50));
        assert_eq!(execution.trace.hard_deadline_ms, Some(90));
    }

    #[test]
    fn execute_timeout_policy_emits_hard_deadline_breach_when_timeout_budget_is_absent() {
        let policy = ResiliencePolicyV1 {
            schema_version: RESILIENCE_POLICY_SCHEMA_V1.to_string(),
            policy_id: "timeout.deadline-only".to_string(),
            retry: None,
            timeout: Some(TimeoutPolicyV1 {
                timeout_ms: 120,
                hard_deadline_ms: Some(90),
            }),
            circuit_breaker: None,
            rate_limit: None,
            bulkhead: None,
            fallback: None,
            checkpoint_required: false,
            telemetry_required: true,
        };
        let execution = execute_timeout_policy(
            &policy,
            ResilienceSurfaceV1::Runtime,
            "test.timeout.deadline-only",
            || TimeoutObservation {
                result: Ok::<(), ResilienceFaultClassificationV1>(()),
                elapsed_ms: 100,
                cancelled: false,
            },
            clone_fault_classification,
            runtime_timeout_fault,
            runtime_cancelled_fault,
        );

        assert!(execution.result.is_err());
        assert_eq!(
            execution.trace.breach_kind,
            Some(TimeoutBreachKindV1::HardDeadline)
        );
        assert!(execution
            .trace
            .decision_summary
            .contains("hard deadline exceeded"));
    }

    #[test]
    fn execute_timeout_policy_distinguishes_timeout_from_terminal_business_failure() {
        let policy = ResiliencePolicyV1::provider_attempt_policy("timeout.failure", 1, 100);
        let execution = execute_timeout_policy(
            &policy,
            ResilienceSurfaceV1::Provider,
            "test.timeout.failure",
            || TimeoutObservation::<(), ResilienceFaultClassificationV1> {
                result: Err(ResilienceFaultClassificationV1::provider(
                    "provider invalid api key",
                    Some(401),
                )),
                elapsed_ms: 20,
                cancelled: false,
            },
            clone_fault_classification,
            provider_timeout_fault,
            provider_cancelled_fault,
        );

        let failure = execution.result.expect_err("terminal failure");
        assert_eq!(
            failure.fault_class,
            ResilienceFaultClassV1::ProviderAuthError
        );
        assert_eq!(
            execution.trace.final_status,
            TimeoutExecutionFinalStatusV1::Failed
        );
        assert!(execution.trace.recovery_artifact.is_none());
    }

    #[test]
    fn execute_timeout_policy_keeps_late_terminal_errors_terminal() {
        let policy = ResiliencePolicyV1::provider_attempt_policy("timeout.late-failure", 1, 50);
        let execution = execute_timeout_policy(
            &policy,
            ResilienceSurfaceV1::Provider,
            "test.timeout.late-failure",
            || TimeoutObservation::<(), ResilienceFaultClassificationV1> {
                result: Err(ResilienceFaultClassificationV1::provider(
                    "provider invalid api key",
                    Some(401),
                )),
                elapsed_ms: 80,
                cancelled: false,
            },
            clone_fault_classification,
            provider_timeout_fault,
            provider_cancelled_fault,
        );

        let failure = execution.result.expect_err("terminal failure");
        assert_eq!(
            failure.fault_class,
            ResilienceFaultClassV1::ProviderAuthError
        );
        assert_eq!(
            execution.trace.final_status,
            TimeoutExecutionFinalStatusV1::Failed
        );
        assert!(execution.trace.decision_summary.contains("failed after"));
        assert!(execution.trace.recovery_artifact.is_none());
    }

    #[test]
    fn execute_timeout_policy_recognizes_generic_timeout_failures() {
        let policy = ResiliencePolicyV1::provider_attempt_policy("timeout.generic", 1, 100);
        let execution = execute_timeout_policy(
            &policy,
            ResilienceSurfaceV1::Tool,
            "test.timeout.generic",
            || TimeoutObservation::<(), ResilienceFaultClassificationV1> {
                result: Err(ResilienceFaultClassificationV1 {
                    schema_version: RESILIENCE_FAULT_CLASSIFICATION_SCHEMA_V1.to_string(),
                    surface: ResilienceSurfaceV1::Tool,
                    fault_class: ResilienceFaultClassV1::RuntimeFailure,
                    disposition: ResilienceFaultDispositionV1::Retryable,
                    retryable: true,
                    summary: "tool timeout while waiting for child process".to_string(),
                    component_ref: None,
                    http_status: None,
                }),
                elapsed_ms: 105,
                cancelled: false,
            },
            clone_fault_classification,
            tool_timeout_fault,
            tool_cancelled_fault,
        );

        let failure = execution.result.expect_err("generic timeout failure");
        assert!(failure.retryable);
        assert_eq!(
            execution.trace.final_status,
            TimeoutExecutionFinalStatusV1::TimedOut
        );
        assert_eq!(
            execution.trace.breach_kind,
            Some(TimeoutBreachKindV1::Timeout)
        );
        assert!(execution.trace.recovery_artifact.is_some());
    }

    #[test]
    fn execute_timeout_policy_handles_timeout_classification_without_budget_breach() {
        let policy = ResiliencePolicyV1 {
            schema_version: RESILIENCE_POLICY_SCHEMA_V1.to_string(),
            policy_id: "timeout.classified-only".to_string(),
            retry: None,
            timeout: None,
            circuit_breaker: None,
            rate_limit: None,
            bulkhead: None,
            fallback: None,
            checkpoint_required: false,
            telemetry_required: true,
        };
        let execution = execute_timeout_policy(
            &policy,
            ResilienceSurfaceV1::Tool,
            "test.timeout.classified-only",
            || TimeoutObservation::<(), ResilienceFaultClassificationV1> {
                result: Err(ResilienceFaultClassificationV1 {
                    schema_version: RESILIENCE_FAULT_CLASSIFICATION_SCHEMA_V1.to_string(),
                    surface: ResilienceSurfaceV1::Tool,
                    fault_class: ResilienceFaultClassV1::Unknown,
                    disposition: ResilienceFaultDispositionV1::Retryable,
                    retryable: true,
                    summary: "operation timed out without explicit timeout budget".to_string(),
                    component_ref: None,
                    http_status: None,
                }),
                elapsed_ms: 45,
                cancelled: false,
            },
            clone_fault_classification,
            tool_timeout_fault,
            tool_cancelled_fault,
        );

        assert!(execution.result.is_err());
        assert_eq!(
            execution.trace.final_status,
            TimeoutExecutionFinalStatusV1::TimedOut
        );
        assert_eq!(execution.trace.breach_kind, None);
        assert!(execution
            .trace
            .decision_summary
            .contains("timeout failure after 45ms"));
        assert!(execution.trace.recovery_artifact.is_some());
    }

    #[test]
    fn execute_timeout_policy_marks_cancellation_as_cancelled_not_success() {
        let policy = ResiliencePolicyV1::provider_attempt_policy("timeout.cancel", 1, 100);
        let execution = execute_timeout_policy(
            &policy,
            ResilienceSurfaceV1::Workflow,
            "test.timeout.cancel",
            || TimeoutObservation::<(), ResilienceFaultClassificationV1> {
                result: Err(ResilienceFaultClassificationV1 {
                    schema_version: RESILIENCE_FAULT_CLASSIFICATION_SCHEMA_V1.to_string(),
                    surface: ResilienceSurfaceV1::Workflow,
                    fault_class: ResilienceFaultClassV1::WorkflowFailure,
                    disposition: ResilienceFaultDispositionV1::Terminal,
                    retryable: false,
                    summary: "cancelled".to_string(),
                    component_ref: None,
                    http_status: None,
                }),
                elapsed_ms: 15,
                cancelled: true,
            },
            clone_fault_classification,
            workflow_timeout_fault,
            workflow_cancelled_fault,
        );

        let failure = execution.result.expect_err("cancelled result");
        assert_eq!(failure.summary, "cancelled at 15");
        assert_eq!(
            execution.trace.final_status,
            TimeoutExecutionFinalStatusV1::Cancelled
        );
        assert_eq!(
            execution
                .trace
                .recovery_artifact
                .as_ref()
                .map(|artifact| artifact.disposition.clone()),
            Some(RecoveryDispositionV1::ResumeAllowed)
        );
    }

    #[test]
    fn timeout_event_and_artifact_ids_remain_unique_across_repeated_emissions() {
        let policy = ResiliencePolicyV1::provider_attempt_policy("timeout.ids", 1, 10);
        let fault = timeout_deadline_fault(
            ResilienceSurfaceV1::Workflow,
            "test.timeout.ids",
            12,
            TimeoutBreachKindV1::Timeout,
            10,
        );
        let first_event = timeout_decision_event(
            &policy,
            ResilienceSurfaceV1::Workflow,
            "test.timeout.ids",
            "first timeout",
            Some(fault.clone()),
        );
        let second_event = timeout_decision_event(
            &policy,
            ResilienceSurfaceV1::Workflow,
            "test.timeout.ids",
            "second timeout",
            Some(fault.clone()),
        );
        let first_artifact = timeout_recovery_artifact(
            &policy,
            ResilienceSurfaceV1::Workflow,
            "test.timeout.ids",
            &fault,
            RecoveryDispositionV1::RetryAllowed,
            "retry",
        );
        let second_artifact = timeout_recovery_artifact(
            &policy,
            ResilienceSurfaceV1::Workflow,
            "test.timeout.ids",
            &fault,
            RecoveryDispositionV1::RetryAllowed,
            "retry",
        );

        assert_ne!(first_event.event_id, second_event.event_id);
        assert_ne!(first_artifact.artifact_id, second_artifact.artifact_id);
    }

    #[test]
    fn timeout_helper_functions_cover_remaining_branch_cases() {
        assert_eq!(timeout_breach(None, None, 10), None);
        assert_eq!(
            timeout_breach(Some(50), Some(90), 95),
            Some((TimeoutBreachKindV1::Timeout, 50))
        );
        assert_eq!(
            timeout_breach(Some(120), Some(90), 100),
            Some((TimeoutBreachKindV1::HardDeadline, 90))
        );
        assert_eq!(
            timeout_breach_label(&TimeoutBreachKindV1::Timeout),
            "timeout budget"
        );
        assert_eq!(
            timeout_breach_label(&TimeoutBreachKindV1::HardDeadline),
            "hard deadline"
        );

        let provider_timeout = ResilienceFaultClassificationV1::provider("provider timeout", None);
        assert!(classification_represents_timeout(&provider_timeout));

        let runtime_deadline = ResilienceFaultClassificationV1 {
            schema_version: RESILIENCE_FAULT_CLASSIFICATION_SCHEMA_V1.to_string(),
            surface: ResilienceSurfaceV1::Runtime,
            fault_class: ResilienceFaultClassV1::RuntimeFailure,
            disposition: ResilienceFaultDispositionV1::Retryable,
            retryable: true,
            summary: "deadline elapsed while waiting".to_string(),
            component_ref: None,
            http_status: None,
        };
        assert!(classification_represents_timeout(&runtime_deadline));

        let runtime_non_timeout = ResilienceFaultClassificationV1 {
            schema_version: RESILIENCE_FAULT_CLASSIFICATION_SCHEMA_V1.to_string(),
            surface: ResilienceSurfaceV1::Runtime,
            fault_class: ResilienceFaultClassV1::RuntimeFailure,
            disposition: ResilienceFaultDispositionV1::Retryable,
            retryable: true,
            summary: "worker exited with code 2".to_string(),
            component_ref: None,
            http_status: None,
        };
        assert!(!classification_represents_timeout(&runtime_non_timeout));

        let provider_error =
            ResilienceFaultClassificationV1::provider("provider_internal_error", Some(500));
        assert!(!classification_represents_timeout(&provider_error));
    }

    #[test]
    fn retry_policy_delay_is_deterministic_and_bounded_by_jitter() {
        let retry = RetryPolicyV1 {
            max_attempts: 3,
            backoff_ms: Some(100),
            jitter_ms: Some(25),
            max_elapsed_ms: None,
            retryable_fault_classes: vec![ResilienceFaultClassV1::ProviderTimeout],
        };
        let first = retry.next_delay_ms("policy.retry", 1);
        let second = retry.next_delay_ms("policy.retry", 1);
        let third_attempt = retry.next_delay_ms("policy.retry", 3);
        assert_eq!(first, second);
        assert!((100..=125).contains(&first));
        assert!((400..=425).contains(&third_attempt));
    }

    #[test]
    fn execute_retry_policy_retries_and_emits_trace() {
        let policy = ResiliencePolicyV1 {
            schema_version: RESILIENCE_POLICY_SCHEMA_V1.to_string(),
            policy_id: "retry.trace".to_string(),
            retry: Some(RetryPolicyV1 {
                max_attempts: 3,
                backoff_ms: Some(5),
                jitter_ms: Some(0),
                max_elapsed_ms: None,
                retryable_fault_classes: vec![ResilienceFaultClassV1::ProviderTimeout],
            }),
            timeout: None,
            circuit_breaker: None,
            rate_limit: None,
            bulkhead: None,
            fallback: None,
            checkpoint_required: false,
            telemetry_required: true,
        };
        let mut attempts = Vec::new();
        let mut sleeps = Vec::new();
        let mut observed = Vec::new();
        let execution = execute_retry_policy(
            &policy,
            ResilienceSurfaceV1::Provider,
            "test.retry",
            |attempt_index| {
                attempts.push(attempt_index);
                if attempt_index < 3 {
                    Err(ResilienceFaultClassificationV1::provider(
                        "provider timeout",
                        Some(504),
                    ))
                } else {
                    Ok("ok")
                }
            },
            |error| error.clone(),
            |delay_ms| sleeps.push(delay_ms),
            |record| observed.push(record.clone()),
        );
        assert_eq!(execution.result.expect("final success"), "ok");
        assert_eq!(attempts, vec![1, 2, 3]);
        assert_eq!(sleeps, vec![5, 10]);
        assert_eq!(observed.len(), 3);
        assert_eq!(
            execution.trace.schema_version,
            RESILIENCE_RETRY_EXECUTION_TRACE_SCHEMA_V1
        );
        assert!(execution
            .trace
            .attempts
            .iter()
            .all(|attempt| attempt.schema_version == RESILIENCE_RETRY_ATTEMPT_SCHEMA_V1));
        assert_eq!(execution.trace.telemetry_events.len(), 3);
        assert_eq!(
            execution.trace.final_status,
            RetryExecutionFinalStatusV1::Succeeded
        );
        assert!(execution.trace.recovery_artifact.is_none());
    }

    #[test]
    fn execute_retry_policy_emits_recovery_artifact_when_budget_exhausts() {
        let policy = ResiliencePolicyV1 {
            schema_version: RESILIENCE_POLICY_SCHEMA_V1.to_string(),
            policy_id: "retry.exhausted".to_string(),
            retry: Some(RetryPolicyV1 {
                max_attempts: 2,
                backoff_ms: Some(1),
                jitter_ms: Some(0),
                max_elapsed_ms: None,
                retryable_fault_classes: vec![ResilienceFaultClassV1::ProviderTransientHttp],
            }),
            timeout: None,
            circuit_breaker: None,
            rate_limit: None,
            bulkhead: None,
            fallback: None,
            checkpoint_required: false,
            telemetry_required: true,
        };
        let execution: RetryExecution<(), ResilienceFaultClassificationV1> = execute_retry_policy(
            &policy,
            ResilienceSurfaceV1::Provider,
            "test.retry.exhausted",
            |_| {
                Err(ResilienceFaultClassificationV1::provider(
                    "server 503",
                    Some(503),
                ))
            },
            |error| error.clone(),
            |_| {},
            |_| {},
        );
        let failure = execution.result.expect_err("final failure");
        assert_eq!(
            failure.fault_class,
            ResilienceFaultClassV1::ProviderTransientHttp
        );
        assert_eq!(execution.trace.attempts.len(), 2);
        let recovery = execution
            .trace
            .recovery_artifact
            .expect("recovery artifact");
        assert_eq!(
            recovery.disposition,
            RecoveryDispositionV1::OperatorInterventionRequired
        );
        assert!(recovery.next_action.contains("retry budget exhausted"));
    }

    #[test]
    fn timeout_fault_builder_helpers_cover_all_remaining_surfaces() {
        let tool_timeout = tool_timeout_fault(TimeoutBreachKindV1::Timeout, 12, 10);
        assert_eq!(tool_timeout.surface, ResilienceSurfaceV1::Tool);
        assert!(tool_timeout.retryable);

        let provider_timeout = provider_timeout_fault(TimeoutBreachKindV1::HardDeadline, 22, 20);
        assert_eq!(provider_timeout.surface, ResilienceSurfaceV1::Provider);
        assert!(provider_timeout.summary.contains("hard deadline"));

        let tool_cancel = tool_cancelled_fault(13);
        assert_eq!(tool_cancel.surface, ResilienceSurfaceV1::Tool);
        assert!(!tool_cancel.retryable);

        let provider_cancel = provider_cancelled_fault(14);
        assert_eq!(provider_cancel.surface, ResilienceSurfaceV1::Provider);
        assert!(provider_cancel.summary.contains("cancelled"));

        let runtime_cancel = runtime_cancelled_fault(15);
        assert_eq!(runtime_cancel.surface, ResilienceSurfaceV1::Runtime);
        assert!(runtime_cancel.summary.contains("15"));
    }

    #[test]
    fn circuit_breaker_trips_open_and_rejects_follow_up_calls() {
        let policy = test_circuit_breaker_policy();
        let first = execute_circuit_breaker_policy(
            &policy,
            ResilienceSurfaceV1::Provider,
            "test.breaker.trip",
            &circuit_breaker_initial_state(&policy),
            10,
            || {
                Err(ResilienceFaultClassificationV1::provider(
                    "provider timeout",
                    None,
                ))
            },
            clone_fault_classification,
            provider_breaker_rejection,
            None::<fn() -> &'static str>,
        );
        assert!(first.result.is_err());
        assert_eq!(first.state.state, CircuitBreakerStateKindV1::Closed);
        assert_eq!(first.state.consecutive_failures, 1);

        let second = execute_circuit_breaker_policy(
            &policy,
            ResilienceSurfaceV1::Provider,
            "test.breaker.trip",
            &first.state,
            20,
            || {
                Err(ResilienceFaultClassificationV1::provider(
                    "provider timeout",
                    None,
                ))
            },
            clone_fault_classification,
            provider_breaker_rejection,
            None::<fn() -> &'static str>,
        );
        assert!(second.result.is_err());
        assert_eq!(second.state.state, CircuitBreakerStateKindV1::Open);
        assert_eq!(second.state.consecutive_failures, 2);
        assert!(second.trace.recovery_artifact.is_some());

        let called = Cell::new(0);
        let third = execute_circuit_breaker_policy(
            &policy,
            ResilienceSurfaceV1::Provider,
            "test.breaker.trip",
            &second.state,
            25,
            || {
                called.set(called.get() + 1);
                Ok::<_, ResilienceFaultClassificationV1>("should-not-run")
            },
            clone_fault_classification,
            provider_breaker_rejection,
            None::<fn() -> &'static str>,
        );
        assert_eq!(called.get(), 0);
        assert!(third.result.is_err());
        assert_eq!(
            third.trace.final_status,
            CircuitBreakerFinalStatusV1::OpenRejected
        );
        assert!(!third.trace.operation_executed);
    }

    #[test]
    fn circuit_breaker_uses_fallback_when_open() {
        let policy = test_circuit_breaker_policy();
        let open_state = CircuitBreakerStateV1 {
            schema_version: RESILIENCE_CIRCUIT_BREAKER_STATE_SCHEMA_V1.to_string(),
            policy_id: policy.policy_id.clone(),
            state: CircuitBreakerStateKindV1::Open,
            consecutive_failures: 2,
            half_open_attempts: 0,
            opened_at_ms: Some(10),
            last_failure: Some(ResilienceFaultClassificationV1::provider(
                "provider timeout",
                None,
            )),
        };
        let called = Cell::new(0);
        let execution = execute_circuit_breaker_policy(
            &policy,
            ResilienceSurfaceV1::Workflow,
            "test.breaker.fallback",
            &open_state,
            20,
            || {
                called.set(called.get() + 1);
                Ok::<_, ResilienceFaultClassificationV1>("primary")
            },
            clone_fault_classification,
            provider_breaker_rejection,
            Some(|| "fallback"),
        );

        assert_eq!(called.get(), 0);
        assert_eq!(execution.result.expect("fallback result"), "fallback");
        assert_eq!(
            execution.trace.final_status,
            CircuitBreakerFinalStatusV1::OpenFallback
        );
        assert!(execution.trace.used_fallback);
        assert!(execution.trace.recovery_artifact.is_some());
    }

    #[test]
    fn circuit_breaker_allows_half_open_probe_and_closes_on_success() {
        let policy = test_circuit_breaker_policy();
        let open_state = CircuitBreakerStateV1 {
            schema_version: RESILIENCE_CIRCUIT_BREAKER_STATE_SCHEMA_V1.to_string(),
            policy_id: policy.policy_id.clone(),
            state: CircuitBreakerStateKindV1::Open,
            consecutive_failures: 2,
            half_open_attempts: 0,
            opened_at_ms: Some(10),
            last_failure: Some(ResilienceFaultClassificationV1::provider(
                "provider timeout",
                None,
            )),
        };
        let execution = execute_circuit_breaker_policy(
            &policy,
            ResilienceSurfaceV1::Tool,
            "test.breaker.half-open-success",
            &open_state,
            50,
            || Ok::<_, ResilienceFaultClassificationV1>("ok"),
            clone_fault_classification,
            provider_breaker_rejection,
            None::<fn() -> &'static str>,
        );

        assert_eq!(execution.result.expect("success"), "ok");
        assert_eq!(
            execution.trace.state_before,
            CircuitBreakerStateKindV1::Open
        );
        assert_eq!(
            execution.trace.state_after,
            CircuitBreakerStateKindV1::Closed
        );
        assert_eq!(
            execution.trace.final_status,
            CircuitBreakerFinalStatusV1::HalfOpenProbeSuccess
        );
        assert_eq!(execution.state.consecutive_failures, 0);
        assert!(execution.state.last_failure.is_none());
    }

    #[test]
    fn circuit_breaker_reopens_after_failed_half_open_probe() {
        let policy = test_circuit_breaker_policy();
        let half_open_state = CircuitBreakerStateV1 {
            schema_version: RESILIENCE_CIRCUIT_BREAKER_STATE_SCHEMA_V1.to_string(),
            policy_id: policy.policy_id.clone(),
            state: CircuitBreakerStateKindV1::HalfOpen,
            consecutive_failures: 2,
            half_open_attempts: 0,
            opened_at_ms: Some(10),
            last_failure: Some(ResilienceFaultClassificationV1::provider(
                "provider timeout",
                None,
            )),
        };
        let execution = execute_circuit_breaker_policy(
            &policy,
            ResilienceSurfaceV1::Provider,
            "test.breaker.half-open-failure",
            &half_open_state,
            60,
            || {
                Err(ResilienceFaultClassificationV1::provider(
                    "provider timeout",
                    None,
                ))
            },
            clone_fault_classification,
            provider_breaker_rejection,
            None::<fn() -> &'static str>,
        );

        assert!(execution.result.is_err());
        assert_eq!(execution.state.state, CircuitBreakerStateKindV1::Open);
        assert_eq!(
            execution.trace.final_status,
            CircuitBreakerFinalStatusV1::HalfOpenProbeFailure
        );
        assert!(execution.trace.recovery_artifact.is_some());
    }

    #[test]
    fn circuit_breaker_bounds_half_open_probe_attempts() {
        let policy = test_circuit_breaker_policy();
        let half_open_state = CircuitBreakerStateV1 {
            schema_version: RESILIENCE_CIRCUIT_BREAKER_STATE_SCHEMA_V1.to_string(),
            policy_id: policy.policy_id.clone(),
            state: CircuitBreakerStateKindV1::HalfOpen,
            consecutive_failures: 2,
            half_open_attempts: 1,
            opened_at_ms: Some(10),
            last_failure: Some(ResilienceFaultClassificationV1::provider(
                "provider timeout",
                None,
            )),
        };
        let called = Cell::new(0);
        let execution = execute_circuit_breaker_policy(
            &policy,
            ResilienceSurfaceV1::Provider,
            "test.breaker.half-open-limit",
            &half_open_state,
            60,
            || {
                called.set(called.get() + 1);
                Ok::<_, ResilienceFaultClassificationV1>("should-not-run")
            },
            clone_fault_classification,
            provider_breaker_probe_rejection,
            None::<fn() -> &'static str>,
        );

        assert_eq!(called.get(), 0);
        assert!(execution.result.is_err());
        assert_eq!(
            execution.trace.final_status,
            CircuitBreakerFinalStatusV1::HalfOpenProbeRejected
        );
        assert_eq!(execution.state.state, CircuitBreakerStateKindV1::HalfOpen);
    }

    #[test]
    fn circuit_breaker_honors_multi_probe_budget_before_reopening() {
        let mut policy = test_circuit_breaker_policy();
        policy
            .circuit_breaker
            .as_mut()
            .expect("breaker policy")
            .half_open_max_attempts = 2;
        let open_state = CircuitBreakerStateV1 {
            schema_version: RESILIENCE_CIRCUIT_BREAKER_STATE_SCHEMA_V1.to_string(),
            policy_id: policy.policy_id.clone(),
            state: CircuitBreakerStateKindV1::Open,
            consecutive_failures: 2,
            half_open_attempts: 0,
            opened_at_ms: Some(10),
            last_failure: Some(ResilienceFaultClassificationV1::provider(
                "provider timeout",
                None,
            )),
        };

        let first_failure = execute_circuit_breaker_policy(
            &policy,
            ResilienceSurfaceV1::Provider,
            "test.breaker.multi-probe.first",
            &open_state,
            50,
            || {
                Err(ResilienceFaultClassificationV1::provider(
                    "provider timeout",
                    None,
                ))
            },
            clone_fault_classification,
            provider_breaker_rejection,
            None::<fn() -> &'static str>,
        );
        assert!(first_failure.result.is_err());
        assert_eq!(
            first_failure.trace.final_status,
            CircuitBreakerFinalStatusV1::HalfOpenProbeFailure
        );
        assert_eq!(
            first_failure.state.state,
            CircuitBreakerStateKindV1::HalfOpen
        );
        assert_eq!(first_failure.state.half_open_attempts, 1);
        assert!(first_failure.state.opened_at_ms.is_none());

        let second_failure = execute_circuit_breaker_policy(
            &policy,
            ResilienceSurfaceV1::Provider,
            "test.breaker.multi-probe.second",
            &first_failure.state,
            55,
            || {
                Err(ResilienceFaultClassificationV1::provider(
                    "provider timeout",
                    None,
                ))
            },
            clone_fault_classification,
            provider_breaker_rejection,
            None::<fn() -> &'static str>,
        );
        assert!(second_failure.result.is_err());
        assert_eq!(
            second_failure.trace.final_status,
            CircuitBreakerFinalStatusV1::HalfOpenProbeFailure
        );
        assert_eq!(second_failure.state.state, CircuitBreakerStateKindV1::Open);
        assert_eq!(second_failure.state.half_open_attempts, 2);
        assert_eq!(second_failure.state.opened_at_ms, Some(55));
    }

    #[test]
    fn circuit_breaker_resets_mismatched_policy_state() {
        let policy = test_circuit_breaker_policy();
        let stale_state = CircuitBreakerStateV1 {
            schema_version: RESILIENCE_CIRCUIT_BREAKER_STATE_SCHEMA_V1.to_string(),
            policy_id: "stale.policy".to_string(),
            state: CircuitBreakerStateKindV1::Open,
            consecutive_failures: 7,
            half_open_attempts: 1,
            opened_at_ms: Some(10),
            last_failure: Some(ResilienceFaultClassificationV1::provider(
                "provider timeout",
                None,
            )),
        };

        let execution = execute_circuit_breaker_policy(
            &policy,
            ResilienceSurfaceV1::Provider,
            "test.breaker.policy-reset",
            &stale_state,
            15,
            || Ok::<_, ResilienceFaultClassificationV1>("ok"),
            clone_fault_classification,
            provider_breaker_rejection,
            None::<fn() -> &'static str>,
        );
        assert_eq!(execution.result.expect("success"), "ok");
        assert_eq!(
            execution.trace.state_before,
            CircuitBreakerStateKindV1::Closed
        );
        assert_eq!(execution.state.policy_id, policy.policy_id);
        assert_eq!(execution.state.state, CircuitBreakerStateKindV1::Closed);
        assert_eq!(execution.state.consecutive_failures, 0);
    }

    #[test]
    fn circuit_breaker_composes_timeout_faults_without_retry_storms() {
        let policy = test_circuit_breaker_policy();
        let first_timeout = execute_timeout_policy(
            &policy,
            ResilienceSurfaceV1::Provider,
            "test.breaker.compose.timeout",
            || TimeoutObservation {
                result: Ok::<_, ResilienceFaultClassificationV1>("late"),
                elapsed_ms: 125,
                cancelled: false,
            },
            clone_fault_classification,
            provider_timeout_fault,
            provider_cancelled_fault,
        );
        let first_fault = first_timeout.trace.fault.clone().expect("timeout fault");
        assert_eq!(
            first_fault.fault_class,
            ResilienceFaultClassV1::RuntimeFailure
        );

        let first_breaker = execute_circuit_breaker_policy(
            &policy,
            ResilienceSurfaceV1::Provider,
            "test.breaker.compose.first",
            &circuit_breaker_initial_state(&policy),
            10,
            || Err(first_fault.clone()),
            clone_fault_classification,
            provider_breaker_rejection,
            None::<fn() -> &'static str>,
        );
        assert!(first_breaker.result.is_err());
        assert_eq!(first_breaker.state.state, CircuitBreakerStateKindV1::Closed);

        let second_timeout = execute_timeout_policy(
            &policy,
            ResilienceSurfaceV1::Provider,
            "test.breaker.compose.timeout",
            || TimeoutObservation {
                result: Ok::<_, ResilienceFaultClassificationV1>("late"),
                elapsed_ms: 130,
                cancelled: false,
            },
            clone_fault_classification,
            provider_timeout_fault,
            provider_cancelled_fault,
        );
        let second_fault = second_timeout.trace.fault.clone().expect("timeout fault");

        let second_breaker = execute_circuit_breaker_policy(
            &policy,
            ResilienceSurfaceV1::Provider,
            "test.breaker.compose.second",
            &first_breaker.state,
            20,
            || Err(second_fault),
            clone_fault_classification,
            provider_breaker_rejection,
            None::<fn() -> &'static str>,
        );
        assert!(second_breaker.result.is_err());
        assert_eq!(second_breaker.state.state, CircuitBreakerStateKindV1::Open);

        let called = Cell::new(0);
        let rejected = execute_circuit_breaker_policy(
            &policy,
            ResilienceSurfaceV1::Provider,
            "test.breaker.compose.third",
            &second_breaker.state,
            25,
            || {
                called.set(called.get() + 1);
                Ok::<_, ResilienceFaultClassificationV1>("should-not-run")
            },
            clone_fault_classification,
            provider_breaker_rejection,
            None::<fn() -> &'static str>,
        );
        assert_eq!(called.get(), 0);
        assert!(rejected.result.is_err());
        assert_eq!(
            rejected.trace.final_status,
            CircuitBreakerFinalStatusV1::OpenRejected
        );
    }

    #[test]
    fn circuit_breaker_disabled_path_reports_success_and_failure() {
        let policy = ResiliencePolicyV1 {
            circuit_breaker: None,
            ..test_circuit_breaker_policy()
        };
        let state = circuit_breaker_initial_state(&policy);

        let success = execute_circuit_breaker_policy(
            &policy,
            ResilienceSurfaceV1::Tool,
            "test.breaker.disabled.success",
            &state,
            5,
            || Ok::<_, ResilienceFaultClassificationV1>("ok"),
            clone_fault_classification,
            provider_breaker_rejection,
            None::<fn() -> &'static str>,
        );
        assert_eq!(success.result.expect("success"), "ok");
        assert_eq!(
            success.trace.final_status,
            CircuitBreakerFinalStatusV1::ClosedSuccess
        );
        assert!(success.trace.decision_summary.contains("breaker disabled"));

        let failure = execute_circuit_breaker_policy(
            &policy,
            ResilienceSurfaceV1::Tool,
            "test.breaker.disabled.failure",
            &state,
            6,
            || {
                Err(ResilienceFaultClassificationV1::provider(
                    "provider timeout",
                    None,
                ))
            },
            clone_fault_classification,
            provider_breaker_rejection,
            None::<fn() -> &'static str>,
        );
        assert!(failure.result.is_err());
        assert_eq!(
            failure.trace.final_status,
            CircuitBreakerFinalStatusV1::ClosedFailure
        );
        assert!(failure.trace.fault.is_some());
    }

    #[test]
    fn circuit_breaker_closed_success_resets_prior_failure_state() {
        let policy = test_circuit_breaker_policy();
        let prior_state = CircuitBreakerStateV1 {
            schema_version: RESILIENCE_CIRCUIT_BREAKER_STATE_SCHEMA_V1.to_string(),
            policy_id: policy.policy_id.clone(),
            state: CircuitBreakerStateKindV1::Closed,
            consecutive_failures: 1,
            half_open_attempts: 0,
            opened_at_ms: None,
            last_failure: Some(ResilienceFaultClassificationV1::provider(
                "provider timeout",
                None,
            )),
        };
        let execution = execute_circuit_breaker_policy(
            &policy,
            ResilienceSurfaceV1::Workflow,
            "test.breaker.closed-success",
            &prior_state,
            40,
            || Ok::<_, ResilienceFaultClassificationV1>("ok"),
            clone_fault_classification,
            provider_breaker_rejection,
            None::<fn() -> &'static str>,
        );

        assert_eq!(execution.result.expect("success"), "ok");
        assert_eq!(
            execution.trace.final_status,
            CircuitBreakerFinalStatusV1::ClosedSuccess
        );
        assert_eq!(execution.state.consecutive_failures, 0);
        assert_eq!(execution.state.state, CircuitBreakerStateKindV1::Closed);
        assert!(execution.state.last_failure.is_none());
    }

    #[test]
    fn circuit_breaker_helper_functions_cover_state_window_and_id_generation() {
        let policy = test_circuit_breaker_policy();
        let open_state = CircuitBreakerStateV1 {
            schema_version: RESILIENCE_CIRCUIT_BREAKER_STATE_SCHEMA_V1.to_string(),
            policy_id: policy.policy_id.clone(),
            state: CircuitBreakerStateKindV1::Open,
            consecutive_failures: 2,
            half_open_attempts: 0,
            opened_at_ms: Some(10),
            last_failure: Some(ResilienceFaultClassificationV1::provider(
                "provider timeout",
                None,
            )),
        };
        let still_open = circuit_breaker_state_for_now(
            &open_state,
            policy.circuit_breaker.as_ref().expect("breaker policy"),
            20,
        );
        assert_eq!(still_open.state, CircuitBreakerStateKindV1::Open);
        let half_open = circuit_breaker_state_for_now(
            &open_state,
            policy.circuit_breaker.as_ref().expect("breaker policy"),
            45,
        );
        assert_eq!(half_open.state, CircuitBreakerStateKindV1::HalfOpen);
        assert_eq!(half_open.half_open_attempts, 0);
        let unchanged = circuit_breaker_state_for_now(
            &circuit_breaker_initial_state(&policy),
            policy.circuit_breaker.as_ref().expect("breaker policy"),
            45,
        );
        assert_eq!(unchanged.state, CircuitBreakerStateKindV1::Closed);

        let fault = ResilienceFaultClassificationV1::provider("provider timeout", None);
        let first_event = circuit_breaker_decision_event(
            &policy,
            ResilienceSurfaceV1::Provider,
            "test.breaker.ids",
            "first",
            Some(fault.clone()),
        );
        let second_event = circuit_breaker_decision_event(
            &policy,
            ResilienceSurfaceV1::Provider,
            "test.breaker.ids",
            "second",
            Some(fault.clone()),
        );
        let first_artifact = circuit_breaker_recovery_artifact(
            &policy,
            ResilienceSurfaceV1::Provider,
            "test.breaker.ids",
            &fault,
            RecoveryDispositionV1::RetryAllowed,
            "retry later",
        );
        let second_artifact = circuit_breaker_recovery_artifact(
            &policy,
            ResilienceSurfaceV1::Provider,
            "test.breaker.ids",
            &fault,
            RecoveryDispositionV1::RetryAllowed,
            "retry later",
        );
        assert_ne!(first_event.event_id, second_event.event_id);
        assert_ne!(first_artifact.artifact_id, second_artifact.artifact_id);
    }
}
