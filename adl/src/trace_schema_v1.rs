use anyhow::{anyhow, Context, Result};
use schemars::{schema_for, JsonSchema};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TraceEventTypeV1 {
    RunStart,
    RunEnd,
    LifecyclePhase,
    ExecutionBoundary,
    Proposal,
    ProposalNormalization,
    CapabilityContract,
    PolicyInjection,
    VisibilityPolicy,
    FreedomGateDecision,
    ActionSelection,
    ActionRejection,
    ExecutionResult,
    Refusal,
    RedactionDecision,
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
pub struct TraceVisibilityViewsV1 {
    pub actor_view: String,
    pub operator_view: String,
    pub reviewer_view: String,
    pub public_report_view: String,
    pub observatory_projection: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
pub struct TraceGovernanceEvidenceV1 {
    pub proposal_id: Option<String>,
    pub normalized_proposal_ref: Option<String>,
    pub acc_contract_id: Option<String>,
    pub policy_evidence_ref: Option<String>,
    pub gate_candidate_id: Option<String>,
    pub gate_boundary: Option<String>,
    pub gate_reason_code: Option<String>,
    pub action_id: Option<String>,
    pub tool_name: Option<String>,
    pub adapter_id: Option<String>,
    pub replay_posture: Option<String>,
    pub result_ref: Option<String>,
    pub redaction_summary: Option<String>,
    pub evidence_refs: Vec<String>,
    pub visibility_views: Option<TraceVisibilityViewsV1>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
pub struct TraceRedactionDecisionV1 {
    pub audience: String,
    pub surfaces: Vec<String>,
    pub outcome: String,
    pub detail: Option<String>,
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
    pub governance: Option<TraceGovernanceEvidenceV1>,
    pub redaction: Option<TraceRedactionDecisionV1>,
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
    if envelope.schema_version != "trace.v1" && envelope.schema_version != "trace.v2" {
        return Err(anyhow!(
            "trace schema v1 envelope requires schema_version trace.v1 or trace.v2, found '{}'",
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
    let has_governed_events = envelope
        .events
        .iter()
        .any(|event| is_governed_event_type(&event.event_type));
    if has_governed_events && envelope.schema_version != "trace.v2" {
        return Err(anyhow!(
            "governed trace events require schema_version=trace.v2"
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
    if matches!(
        event.event_type,
        TraceEventTypeV1::Proposal
            | TraceEventTypeV1::ProposalNormalization
            | TraceEventTypeV1::CapabilityContract
            | TraceEventTypeV1::PolicyInjection
            | TraceEventTypeV1::VisibilityPolicy
            | TraceEventTypeV1::FreedomGateDecision
            | TraceEventTypeV1::ActionSelection
            | TraceEventTypeV1::ActionRejection
            | TraceEventTypeV1::ExecutionResult
            | TraceEventTypeV1::Refusal
            | TraceEventTypeV1::RedactionDecision
    ) && event.governance.is_none()
    {
        return Err(anyhow!(
            "governed execution events require a governance block"
        ));
    }
    if matches!(event.event_type, TraceEventTypeV1::RedactionDecision) && event.redaction.is_none()
    {
        return Err(anyhow!(
            "REDACTION_DECISION events require a redaction block"
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
    if let Some(governance) = &event.governance {
        validate_trace_governance(event, governance)?;
    }
    if let Some(redaction) = &event.redaction {
        require_non_empty("redaction.audience", &redaction.audience)?;
        require_non_empty("redaction.outcome", &redaction.outcome)?;
        if redaction.surfaces.is_empty() {
            return Err(anyhow!("redaction.surfaces must not be empty"));
        }
        for surface in &redaction.surfaces {
            require_non_empty("redaction.surfaces[]", surface)?;
        }
        if let Some(detail) = redaction.detail.as_deref() {
            require_non_empty("redaction.detail", detail)?;
            reject_sensitive_text("redaction.detail", detail)?;
        }
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

fn validate_trace_governance(
    event: &TraceEventV1,
    governance: &TraceGovernanceEvidenceV1,
) -> Result<()> {
    if let Some(value) = governance.proposal_id.as_deref() {
        require_non_empty("governance.proposal_id", value)?;
    }
    if let Some(value) = governance.normalized_proposal_ref.as_deref() {
        require_non_empty("governance.normalized_proposal_ref", value)?;
    }
    if let Some(value) = governance.acc_contract_id.as_deref() {
        require_non_empty("governance.acc_contract_id", value)?;
    }
    if let Some(value) = governance.policy_evidence_ref.as_deref() {
        require_non_empty("governance.policy_evidence_ref", value)?;
    }
    if let Some(value) = governance.gate_candidate_id.as_deref() {
        require_non_empty("governance.gate_candidate_id", value)?;
    }
    if let Some(value) = governance.gate_boundary.as_deref() {
        require_non_empty("governance.gate_boundary", value)?;
    }
    if let Some(value) = governance.gate_reason_code.as_deref() {
        require_non_empty("governance.gate_reason_code", value)?;
    }
    if let Some(value) = governance.action_id.as_deref() {
        require_non_empty("governance.action_id", value)?;
    }
    if let Some(value) = governance.tool_name.as_deref() {
        require_non_empty("governance.tool_name", value)?;
    }
    if let Some(value) = governance.adapter_id.as_deref() {
        require_non_empty("governance.adapter_id", value)?;
    }
    if let Some(value) = governance.replay_posture.as_deref() {
        require_non_empty("governance.replay_posture", value)?;
    }
    if let Some(value) = governance.result_ref.as_deref() {
        validate_artifact_ref("governance.result_ref", value)?;
    }
    if let Some(value) = governance.redaction_summary.as_deref() {
        require_non_empty("governance.redaction_summary", value)?;
        reject_sensitive_text("governance.redaction_summary", value)?;
    }
    for evidence_ref in &governance.evidence_refs {
        require_non_empty("governance.evidence_refs[]", evidence_ref)?;
    }
    if let Some(views) = &governance.visibility_views {
        require_non_empty("governance.visibility_views.actor_view", &views.actor_view)?;
        reject_sensitive_text("governance.visibility_views.actor_view", &views.actor_view)?;
        require_non_empty(
            "governance.visibility_views.operator_view",
            &views.operator_view,
        )?;
        reject_sensitive_text(
            "governance.visibility_views.operator_view",
            &views.operator_view,
        )?;
        require_non_empty(
            "governance.visibility_views.reviewer_view",
            &views.reviewer_view,
        )?;
        reject_sensitive_text(
            "governance.visibility_views.reviewer_view",
            &views.reviewer_view,
        )?;
        require_non_empty(
            "governance.visibility_views.public_report_view",
            &views.public_report_view,
        )?;
        reject_sensitive_text(
            "governance.visibility_views.public_report_view",
            &views.public_report_view,
        )?;
        require_non_empty(
            "governance.visibility_views.observatory_projection",
            &views.observatory_projection,
        )?;
        reject_sensitive_text(
            "governance.visibility_views.observatory_projection",
            &views.observatory_projection,
        )?;
    }

    match event.event_type {
        TraceEventTypeV1::Proposal => {
            require_some("governance.proposal_id", governance.proposal_id.as_deref())?;
            require_some("governance.tool_name", governance.tool_name.as_deref())?;
        }
        TraceEventTypeV1::ProposalNormalization => {
            require_some("governance.proposal_id", governance.proposal_id.as_deref())?;
            require_some(
                "governance.normalized_proposal_ref",
                governance.normalized_proposal_ref.as_deref(),
            )?;
        }
        TraceEventTypeV1::CapabilityContract => {
            require_some("governance.proposal_id", governance.proposal_id.as_deref())?;
            require_some(
                "governance.acc_contract_id",
                governance.acc_contract_id.as_deref(),
            )?;
            require_some(
                "governance.replay_posture",
                governance.replay_posture.as_deref(),
            )?;
        }
        TraceEventTypeV1::PolicyInjection => {
            require_some("governance.proposal_id", governance.proposal_id.as_deref())?;
            require_some(
                "governance.policy_evidence_ref",
                governance.policy_evidence_ref.as_deref(),
            )?;
        }
        TraceEventTypeV1::VisibilityPolicy => {
            require_some("governance.proposal_id", governance.proposal_id.as_deref())?;
            if governance.visibility_views.is_none() {
                return Err(anyhow!(
                    "VISIBILITY_POLICY events require governance.visibility_views"
                ));
            }
        }
        TraceEventTypeV1::FreedomGateDecision => {
            require_some("governance.proposal_id", governance.proposal_id.as_deref())?;
            require_some(
                "governance.gate_candidate_id",
                governance.gate_candidate_id.as_deref(),
            )?;
            require_some(
                "governance.gate_reason_code",
                governance.gate_reason_code.as_deref(),
            )?;
            require_some(
                "governance.gate_boundary",
                governance.gate_boundary.as_deref(),
            )?;
        }
        TraceEventTypeV1::ActionSelection | TraceEventTypeV1::ActionRejection => {
            require_some("governance.proposal_id", governance.proposal_id.as_deref())?;
            require_some("governance.action_id", governance.action_id.as_deref())?;
            require_some("governance.tool_name", governance.tool_name.as_deref())?;
            require_some("governance.adapter_id", governance.adapter_id.as_deref())?;
            if governance.evidence_refs.is_empty() {
                return Err(anyhow!(
                    "ACTION_SELECTION/ACTION_REJECTION events require evidence_refs"
                ));
            }
        }
        TraceEventTypeV1::ExecutionResult => {
            require_some("governance.proposal_id", governance.proposal_id.as_deref())?;
            require_some("governance.action_id", governance.action_id.as_deref())?;
            require_some("governance.adapter_id", governance.adapter_id.as_deref())?;
            require_some("governance.result_ref", governance.result_ref.as_deref())?;
            if governance.evidence_refs.is_empty() {
                return Err(anyhow!("EXECUTION_RESULT events require evidence_refs"));
            }
        }
        TraceEventTypeV1::Refusal => {
            require_some("governance.proposal_id", governance.proposal_id.as_deref())?;
            require_some("governance.action_id", governance.action_id.as_deref())?;
            require_some(
                "governance.gate_reason_code",
                governance.gate_reason_code.as_deref(),
            )?;
            if governance.evidence_refs.is_empty() {
                return Err(anyhow!("REFUSAL events require evidence_refs"));
            }
        }
        TraceEventTypeV1::RedactionDecision => {
            require_some("governance.proposal_id", governance.proposal_id.as_deref())?;
        }
        _ => {}
    }

    Ok(())
}

fn require_non_empty(field: &str, value: &str) -> Result<()> {
    if value.trim().is_empty() {
        return Err(anyhow!("{field} must not be empty"));
    }
    Ok(())
}

fn is_governed_event_type(event_type: &TraceEventTypeV1) -> bool {
    matches!(
        event_type,
        TraceEventTypeV1::Proposal
            | TraceEventTypeV1::ProposalNormalization
            | TraceEventTypeV1::CapabilityContract
            | TraceEventTypeV1::PolicyInjection
            | TraceEventTypeV1::VisibilityPolicy
            | TraceEventTypeV1::FreedomGateDecision
            | TraceEventTypeV1::ActionSelection
            | TraceEventTypeV1::ActionRejection
            | TraceEventTypeV1::ExecutionResult
            | TraceEventTypeV1::Refusal
            | TraceEventTypeV1::RedactionDecision
    )
}

fn require_some(field: &str, value: Option<&str>) -> Result<()> {
    if value.is_none() {
        return Err(anyhow!("{field} is required for this event type"));
    }
    Ok(())
}

fn reject_sensitive_text(field: &str, value: &str) -> Result<()> {
    let contains_sensitive_marker = value.contains("/Users/")
        || value.contains("/home/")
        || value.contains("sk-")
        || value.contains("gho_")
        || value.contains("BEGIN PRIVATE KEY")
        || value.contains('{')
        || value.contains('}');
    if contains_sensitive_marker {
        return Err(anyhow!(
            "{field} must not contain raw payload markers, secret-like tokens, or host paths"
        ));
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
            governance: None,
            redaction: None,
        }
    }

    #[test]
    fn trace_schema_v1_json_mentions_required_event_types() {
        let schema_json = trace_schema_v1_json().expect("schema json");
        assert!(schema_json.contains("RUN_START"));
        assert!(schema_json.contains("PROPOSAL"));
        assert!(schema_json.contains("FREEDOM_GATE_DECISION"));
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
                    "contract_validation": null,
                    "governance": null,
                    "redaction": null
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
                    "contract_validation": null,
                    "governance": null,
                    "redaction": null
                }
            ]
        });
        validate_trace_event_envelope_v1_value(&value).expect("value must validate");
    }

    #[test]
    fn validate_trace_event_envelope_v1_accepts_governed_trace_events() {
        let run_start = sample_event(TraceEventTypeV1::RunStart);
        let mut proposal = sample_event(TraceEventTypeV1::Proposal);
        proposal.governance = Some(TraceGovernanceEvidenceV1 {
            proposal_id: Some("proposal.fixture.safe-read".to_string()),
            normalized_proposal_ref: None,
            acc_contract_id: None,
            policy_evidence_ref: None,
            gate_candidate_id: None,
            gate_boundary: None,
            gate_reason_code: None,
            action_id: None,
            tool_name: Some("fixture.safe_read".to_string()),
            adapter_id: None,
            replay_posture: None,
            result_ref: None,
            redaction_summary: None,
            evidence_refs: vec!["proposal:proposal.fixture.safe-read".to_string()],
            visibility_views: None,
        });
        let mut redaction = sample_event(TraceEventTypeV1::RedactionDecision);
        redaction.governance = Some(TraceGovernanceEvidenceV1 {
            proposal_id: Some("proposal.fixture.safe-read".to_string()),
            normalized_proposal_ref: None,
            acc_contract_id: None,
            policy_evidence_ref: None,
            gate_candidate_id: None,
            gate_boundary: None,
            gate_reason_code: None,
            action_id: None,
            tool_name: None,
            adapter_id: None,
            replay_posture: None,
            result_ref: None,
            redaction_summary: None,
            evidence_refs: Vec::new(),
            visibility_views: None,
        });
        redaction.redaction = Some(TraceRedactionDecisionV1 {
            audience: "reviewer".to_string(),
            surfaces: vec!["arguments".to_string(), "results".to_string()],
            outcome: "redacted".to_string(),
            detail: Some("digest_only".to_string()),
        });
        let run_end = sample_event(TraceEventTypeV1::RunEnd);
        let envelope = TraceEventEnvelopeV1 {
            schema_version: "trace.v2".to_string(),
            events: vec![run_start, proposal, redaction, run_end],
        };
        validate_trace_event_envelope_v1(&envelope).expect("governed envelope");
    }

    #[test]
    fn validate_trace_event_envelope_v1_rejects_governed_events_without_required_blocks() {
        let envelope = TraceEventEnvelopeV1 {
            schema_version: "trace.v1".to_string(),
            events: vec![
                sample_event(TraceEventTypeV1::RunStart),
                sample_event(TraceEventTypeV1::Proposal),
                sample_event(TraceEventTypeV1::RunEnd),
            ],
        };
        let err = validate_trace_event_envelope_v1(&envelope)
            .expect_err("missing governance block must fail");
        assert!(err.to_string().contains("governance block"));
    }

    #[test]
    fn validate_trace_event_envelope_v1_rejects_execution_result_without_evidence_refs() {
        let mut result = sample_event(TraceEventTypeV1::ExecutionResult);
        result.governance = Some(TraceGovernanceEvidenceV1 {
            proposal_id: Some("proposal.fixture.safe-read".to_string()),
            normalized_proposal_ref: None,
            acc_contract_id: None,
            policy_evidence_ref: None,
            gate_candidate_id: None,
            gate_boundary: None,
            gate_reason_code: None,
            action_id: Some("action.safe_read".to_string()),
            tool_name: None,
            adapter_id: Some("adapter.fixture.safe_read.dry_run".to_string()),
            replay_posture: None,
            result_ref: Some("artifacts/run-1/governed/result.redacted.json".to_string()),
            redaction_summary: None,
            evidence_refs: Vec::new(),
            visibility_views: None,
        });
        let envelope = TraceEventEnvelopeV1 {
            schema_version: "trace.v2".to_string(),
            events: vec![
                sample_event(TraceEventTypeV1::RunStart),
                result,
                sample_event(TraceEventTypeV1::RunEnd),
            ],
        };
        let err =
            validate_trace_event_envelope_v1(&envelope).expect_err("empty evidence must fail");
        assert!(err.to_string().contains("evidence_refs"));
    }

    #[test]
    fn validate_trace_event_envelope_v1_rejects_visibility_payload_markers() {
        let mut visibility = sample_event(TraceEventTypeV1::VisibilityPolicy);
        visibility.governance = Some(TraceGovernanceEvidenceV1 {
            proposal_id: Some("proposal.fixture.safe-read".to_string()),
            normalized_proposal_ref: None,
            acc_contract_id: None,
            policy_evidence_ref: None,
            gate_candidate_id: None,
            gate_boundary: None,
            gate_reason_code: None,
            action_id: None,
            tool_name: None,
            adapter_id: None,
            replay_posture: None,
            result_ref: None,
            redaction_summary: None,
            evidence_refs: vec!["visibility:policy".to_string()],
            visibility_views: Some(TraceVisibilityViewsV1 {
                actor_view: "compiled ACC request status".to_string(),
                operator_view: "{\"secret\":\"leak\"}".to_string(),
                reviewer_view: "redacted compiler evidence".to_string(),
                public_report_view: "aggregate compiler pass/fail only".to_string(),
                observatory_projection: "redacted compiler governance event".to_string(),
            }),
        });
        let envelope = TraceEventEnvelopeV1 {
            schema_version: "trace.v2".to_string(),
            events: vec![
                sample_event(TraceEventTypeV1::RunStart),
                visibility,
                sample_event(TraceEventTypeV1::RunEnd),
            ],
        };
        let err = validate_trace_event_envelope_v1(&envelope)
            .expect_err("visibility payload markers must fail");
        assert!(err.to_string().contains("payload markers"));
    }

    #[test]
    fn validate_trace_event_envelope_v1_rejects_governed_events_on_trace_v1() {
        let mut proposal = sample_event(TraceEventTypeV1::Proposal);
        proposal.governance = Some(TraceGovernanceEvidenceV1 {
            proposal_id: Some("proposal.fixture.safe-read".to_string()),
            normalized_proposal_ref: None,
            acc_contract_id: None,
            policy_evidence_ref: None,
            gate_candidate_id: None,
            gate_boundary: None,
            gate_reason_code: None,
            action_id: None,
            tool_name: Some("fixture.safe_read".to_string()),
            adapter_id: None,
            replay_posture: None,
            result_ref: None,
            redaction_summary: None,
            evidence_refs: vec!["proposal:proposal.fixture.safe-read".to_string()],
            visibility_views: None,
        });
        let envelope = TraceEventEnvelopeV1 {
            schema_version: "trace.v1".to_string(),
            events: vec![
                sample_event(TraceEventTypeV1::RunStart),
                proposal,
                sample_event(TraceEventTypeV1::RunEnd),
            ],
        };
        let err = validate_trace_event_envelope_v1(&envelope)
            .expect_err("governed events on trace.v1 must fail");
        assert!(err.to_string().contains("trace.v2"));
    }
}
