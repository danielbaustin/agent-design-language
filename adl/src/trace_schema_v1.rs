use anyhow::{anyhow, Context, Result};
use schemars::{schema_for, JsonSchema};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TraceEventTypeV1 {
    RunStart,
    RunEnd,
    StepStart,
    StepEnd,
    ModelInvocation,
    ToolInvocation,
    SkillExecution,
    MemoryRead,
    MemoryWrite,
    ContractValidation,
    Decision,
    Approval,
    Rejection,
    Revision,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TraceActorTypeV1 {
    Agent,
    Tool,
    Provider,
    Skill,
    System,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TraceScopeLevelV1 {
    Run,
    Step,
    Substep,
    Tool,
    Model,
    Skill,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ContractValidationResultV1 {
    Pass,
    Fail,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
pub struct TraceActorV1 {
    #[schemars(description = "Actor type that owns or emits the event.")]
    pub r#type: TraceActorTypeV1,
    #[schemars(description = "Stable actor identifier inside the bounded execution surface.")]
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
pub struct TraceScopeV1 {
    #[schemars(description = "Execution scope level for the event.")]
    pub level: TraceScopeLevelV1,
    #[schemars(description = "Human-reviewable name for the execution scope.")]
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
pub struct TraceProviderV1 {
    pub vendor: String,
    pub transport: String,
    pub model_ref: String,
    #[schemars(description = "Provider-native raw model identifier.")]
    pub provider_model_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
pub struct TraceErrorV1 {
    pub code: String,
    pub message: String,
    #[schemars(description = "Optional structured error details.")]
    pub details: Option<JsonValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
pub struct TraceContractValidationV1 {
    pub contract_id: String,
    pub result: ContractValidationResultV1,
    pub details: Option<JsonValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
pub struct TraceDecisionContextV1 {
    pub context: String,
    pub outcome: String,
    pub rationale: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
pub struct TraceEventV1 {
    pub event_id: String,
    pub timestamp: String,
    pub event_type: TraceEventTypeV1,
    pub trace_id: String,
    pub run_id: String,
    pub span_id: String,
    pub parent_span_id: Option<String>,
    pub actor: TraceActorV1,
    pub scope: TraceScopeV1,
    pub inputs_ref: Option<String>,
    pub outputs_ref: Option<String>,
    pub artifact_ref: Option<String>,
    pub decision_context: Option<TraceDecisionContextV1>,
    pub provider: Option<TraceProviderV1>,
    pub error: Option<TraceErrorV1>,
    pub contract_validation: Option<TraceContractValidationV1>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
pub struct TraceEventEnvelopeV1 {
    pub schema_version: String,
    pub events: Vec<TraceEventV1>,
}

pub fn trace_schema_v1_json() -> Result<String> {
    serde_json::to_string_pretty(&schema_for!(TraceEventEnvelopeV1))
        .context("serialize trace schema v1 json schema")
}

pub fn validate_trace_event_envelope_v1_value(value: &JsonValue) -> Result<TraceEventEnvelopeV1> {
    let envelope: TraceEventEnvelopeV1 =
        serde_json::from_value(value.clone()).context("parse trace schema v1 envelope")?;
    validate_trace_event_envelope_v1(&envelope)?;
    Ok(envelope)
}

pub fn validate_trace_event_envelope_v1(envelope: &TraceEventEnvelopeV1) -> Result<()> {
    if envelope.schema_version != "trace.v1" {
        return Err(anyhow!(
            "trace schema v1 envelope requires schema_version=trace.v1, found '{}'",
            envelope.schema_version
        ));
    }
    if envelope.events.is_empty() {
        return Err(anyhow!(
            "trace schema v1 envelope requires at least one event"
        ));
    }

    let mut has_run_start = false;
    let mut has_run_end = false;
    for event in &envelope.events {
        validate_trace_event_v1(event)?;
        match event.event_type {
            TraceEventTypeV1::RunStart => has_run_start = true,
            TraceEventTypeV1::RunEnd => has_run_end = true,
            _ => {}
        }
    }
    if !has_run_start {
        return Err(anyhow!(
            "trace schema v1 requires at least one RUN_START event"
        ));
    }
    if !has_run_end {
        return Err(anyhow!(
            "trace schema v1 requires at least one RUN_END event"
        ));
    }
    Ok(())
}

pub fn validate_trace_event_v1(event: &TraceEventV1) -> Result<()> {
    require_non_empty("event_id", &event.event_id)?;
    require_non_empty("timestamp", &event.timestamp)?;
    require_non_empty("trace_id", &event.trace_id)?;
    require_non_empty("run_id", &event.run_id)?;
    require_non_empty("span_id", &event.span_id)?;
    require_non_empty("actor.id", &event.actor.id)?;
    require_non_empty("scope.name", &event.scope.name)?;

    if matches!(event.event_type, TraceEventTypeV1::ModelInvocation) && event.provider.is_none() {
        return Err(anyhow!("MODEL_INVOCATION events require a provider block"));
    }
    if matches!(event.event_type, TraceEventTypeV1::Error) && event.error.is_none() {
        return Err(anyhow!("ERROR events require an error block"));
    }
    if matches!(event.event_type, TraceEventTypeV1::ContractValidation)
        && event.contract_validation.is_none()
    {
        return Err(anyhow!(
            "CONTRACT_VALIDATION events require a contract_validation block"
        ));
    }
    if matches!(
        event.event_type,
        TraceEventTypeV1::Decision
            | TraceEventTypeV1::Approval
            | TraceEventTypeV1::Rejection
            | TraceEventTypeV1::Revision
    ) && event.decision_context.is_none()
    {
        return Err(anyhow!(
            "DECISION/APPROVAL/REJECTION/REVISION events require a decision_context block"
        ));
    }
    if let Some(provider) = &event.provider {
        require_non_empty("provider.vendor", &provider.vendor)?;
        require_non_empty("provider.transport", &provider.transport)?;
        require_non_empty("provider.model_ref", &provider.model_ref)?;
    }
    if let Some(error) = &event.error {
        require_non_empty("error.code", &error.code)?;
        require_non_empty("error.message", &error.message)?;
    }
    if let Some(cv) = &event.contract_validation {
        require_non_empty("contract_validation.contract_id", &cv.contract_id)?;
    }
    if let Some(dc) = &event.decision_context {
        require_non_empty("decision_context.context", &dc.context)?;
        require_non_empty("decision_context.outcome", &dc.outcome)?;
    }

    if let Some(path) = event.inputs_ref.as_deref() {
        validate_artifact_ref("inputs_ref", path)?;
    }
    if let Some(path) = event.outputs_ref.as_deref() {
        validate_artifact_ref("outputs_ref", path)?;
    }
    if let Some(path) = event.artifact_ref.as_deref() {
        validate_artifact_ref("artifact_ref", path)?;
    }

    Ok(())
}

fn require_non_empty(field: &str, value: &str) -> Result<()> {
    if value.trim().is_empty() {
        return Err(anyhow!("{field} must not be empty"));
    }
    Ok(())
}

fn validate_artifact_ref(field: &str, value: &str) -> Result<()> {
    if !value.starts_with("artifacts/") {
        return Err(anyhow!(
            "{field} must point under artifacts/<run_id>/..., found '{value}'"
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_event(event_type: TraceEventTypeV1) -> TraceEventV1 {
        TraceEventV1 {
            event_id: format!("event-{event_type:?}"),
            timestamp: "2026-04-03T12:00:00Z".to_string(),
            event_type,
            trace_id: "trace-1".to_string(),
            run_id: "run-1".to_string(),
            span_id: "span-1".to_string(),
            parent_span_id: None,
            actor: TraceActorV1 {
                r#type: TraceActorTypeV1::Agent,
                id: "agent.main".to_string(),
            },
            scope: TraceScopeV1 {
                level: TraceScopeLevelV1::Run,
                name: "run".to_string(),
            },
            inputs_ref: Some("artifacts/run-1/inputs.json".to_string()),
            outputs_ref: Some("artifacts/run-1/outputs.json".to_string()),
            artifact_ref: None,
            decision_context: None,
            provider: None,
            error: None,
            contract_validation: None,
        }
    }

    #[test]
    fn trace_schema_v1_json_mentions_required_event_types() {
        let schema_json = trace_schema_v1_json().expect("schema json");
        assert!(schema_json.contains("RUN_START"));
        assert!(schema_json.contains("MODEL_INVOCATION"));
        assert!(schema_json.contains("CONTRACT_VALIDATION"));
    }

    #[test]
    fn validate_trace_event_envelope_v1_accepts_minimal_valid_trace() {
        let mut run_start = sample_event(TraceEventTypeV1::RunStart);
        run_start.scope.level = TraceScopeLevelV1::Run;
        let mut model = sample_event(TraceEventTypeV1::ModelInvocation);
        model.parent_span_id = Some("span-root".to_string());
        model.provider = Some(TraceProviderV1 {
            vendor: "openai".to_string(),
            transport: "openai_http".to_string(),
            model_ref: "gpt-5".to_string(),
            provider_model_id: Some("gpt-5".to_string()),
        });
        let mut run_end = sample_event(TraceEventTypeV1::RunEnd);
        run_end.parent_span_id = Some("span-root".to_string());
        let envelope = TraceEventEnvelopeV1 {
            schema_version: "trace.v1".to_string(),
            events: vec![run_start, model, run_end],
        };
        validate_trace_event_envelope_v1(&envelope).expect("valid envelope");
    }

    #[test]
    fn validate_trace_event_envelope_v1_rejects_missing_provider_on_model_invocation() {
        let envelope = TraceEventEnvelopeV1 {
            schema_version: "trace.v1".to_string(),
            events: vec![
                sample_event(TraceEventTypeV1::RunStart),
                sample_event(TraceEventTypeV1::ModelInvocation),
                sample_event(TraceEventTypeV1::RunEnd),
            ],
        };
        let err =
            validate_trace_event_envelope_v1(&envelope).expect_err("missing provider must fail");
        assert!(err.to_string().contains("MODEL_INVOCATION"));
    }

    #[test]
    fn validate_trace_event_envelope_v1_rejects_bad_artifact_ref() {
        let mut run_start = sample_event(TraceEventTypeV1::RunStart);
        run_start.inputs_ref = Some("/tmp/not-allowed.json".to_string());
        let envelope = TraceEventEnvelopeV1 {
            schema_version: "trace.v1".to_string(),
            events: vec![run_start, sample_event(TraceEventTypeV1::RunEnd)],
        };
        let err = validate_trace_event_envelope_v1(&envelope)
            .expect_err("non-artifacts path must fail validation");
        assert!(err.to_string().contains("artifacts/<run_id>"));
    }

    #[test]
    fn validate_trace_event_envelope_v1_value_accepts_structural_json() {
        let value = serde_json::json!({
            "schema_version": "trace.v1",
            "events": [
                {
                    "event_id": "run-start-1",
                    "timestamp": "2026-04-03T12:00:00Z",
                    "event_type": "RUN_START",
                    "trace_id": "trace-1",
                    "run_id": "run-1",
                    "span_id": "span-root",
                    "parent_span_id": null,
                    "actor": {"type": "agent", "id": "agent.main"},
                    "scope": {"level": "run", "name": "run"},
                    "inputs_ref": "artifacts/run-1/inputs.json",
                    "outputs_ref": null,
                    "artifact_ref": null,
                    "decision_context": null,
                    "provider": null,
                    "error": null,
                    "contract_validation": null
                },
                {
                    "event_id": "run-end-1",
                    "timestamp": "2026-04-03T12:00:01Z",
                    "event_type": "RUN_END",
                    "trace_id": "trace-1",
                    "run_id": "run-1",
                    "span_id": "span-root",
                    "parent_span_id": null,
                    "actor": {"type": "agent", "id": "agent.main"},
                    "scope": {"level": "run", "name": "run"},
                    "inputs_ref": null,
                    "outputs_ref": "artifacts/run-1/outputs.json",
                    "artifact_ref": null,
                    "decision_context": null,
                    "provider": null,
                    "error": null,
                    "contract_validation": null
                }
            ]
        });
        validate_trace_event_envelope_v1_value(&value).expect("value must validate");
    }
}
