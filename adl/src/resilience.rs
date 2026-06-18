use schemars::{schema_for, JsonSchema};
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub const RESILIENCE_FAULT_CLASSIFICATION_SCHEMA_V1: &str =
    "adl.resilience.fault_classification.v1";
pub const RESILIENCE_CITIZEN_HEALTH_SCHEMA_V1: &str = "adl.resilience.citizen_health.v1";
pub const RESILIENCE_RECOVERY_ARTIFACT_SCHEMA_V1: &str = "adl.resilience.recovery_artifact.v1";
pub const RESILIENCE_CHECKPOINT_SCHEMA_V1: &str = "adl.resilience.checkpoint.v1";
pub const RESILIENCE_TELEMETRY_EVENT_SCHEMA_V1: &str = "adl.resilience.telemetry_event.v1";
pub const RESILIENCE_POLICY_SCHEMA_V1: &str = "adl.resilience.policy.v1";
pub const RESILIENCE_SUBSTRATE_SCHEMA_V1: &str = "adl.resilience.substrate_manifest.v1";

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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub retryable_fault_classes: Vec<ResilienceFaultClassV1>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct TimeoutPolicyV1 {
    pub timeout_ms: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hard_deadline_ms: Option<u64>,
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

#[cfg(test)]
mod tests {
    use super::*;

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
}
