use anyhow::{anyhow, Context, Result};
use chrono::DateTime;
use schemars::{schema_for, JsonSchema};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value as JsonValue};
use std::collections::BTreeSet;
use std::path::{Component, Path};

const ACIP_MESSAGE_SCHEMA_VERSION: &str = "acip.message.v1";
const ACIP_CONVERSATION_SCHEMA_VERSION: &str = "acip.conversation.v1";
const ACIP_FIXTURE_SCHEMA_VERSION: &str = "acip.fixture.v1";
const ACIP_INVOCATION_CONTRACT_SCHEMA_VERSION: &str = "acip.invocation.contract.v1";
const ACIP_INVOCATION_EVENT_SCHEMA_VERSION: &str = "acip.invocation.event.v1";
const ACIP_INVOCATION_FIXTURE_SCHEMA_VERSION: &str = "acip.invocation.fixture.v1";
const MAX_CONTENT_CHARS: usize = 4_000;
const MAX_INLINE_SUMMARY_CHARS: usize = 512;
const MAX_LIST_LEN: usize = 16;
const MAX_INLINE_BYTE_LENGTH: u64 = 4_096;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AcipAddressKindV1 {
    Agent,
    Group,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AcipIntentV1 {
    Conversation,
    Consultation,
    InvocationSetup,
    ReviewRequest,
    CodingRequest,
    Delegation,
    Negotiation,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AcipVisibilityV1 {
    Private,
    Shared,
    Public,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AcipTraceRequirementV1 {
    None,
    Summary,
    Full,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AcipInvocationStatusV1 {
    Requested,
    Completed,
    Refused,
    Failed,
    Partial,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AcipResponseChannelKindV1 {
    DirectReply,
    ArtifactReply,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AcipAddressV1 {
    pub kind: AcipAddressKindV1,
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AcipPayloadRefV1 {
    pub payload_kind: String,
    pub payload_ref: String,
    pub media_type: String,
    pub content_sha256: String,
    pub byte_length: u64,
    pub inline_summary: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AcipAttachmentRefV1 {
    pub name: String,
    pub media_type: String,
    pub content_sha256: String,
    pub byte_length: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AcipAuthorityScopeV1 {
    pub allowed_actions: Vec<String>,
    pub authority_basis_refs: Vec<String>,
    pub delegation_permitted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AcipMessageEnvelopeV1 {
    pub schema_version: String,
    pub message_id: String,
    pub conversation_id: String,
    pub timestamp_utc: String,
    pub monotonic_order: u64,
    pub sender: AcipAddressV1,
    pub recipient: AcipAddressV1,
    pub intent: AcipIntentV1,
    pub visibility: AcipVisibilityV1,
    pub trace_requirement: AcipTraceRequirementV1,
    pub content: String,
    pub payload_refs: Vec<AcipPayloadRefV1>,
    pub artifact_refs: Vec<String>,
    pub attachments: Vec<AcipAttachmentRefV1>,
    pub authority_scope: Option<AcipAuthorityScopeV1>,
    pub correlation_id: Option<String>,
    pub prior_message_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AcipConversationEnvelopeV1 {
    pub schema_version: String,
    pub messages: Vec<AcipMessageEnvelopeV1>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AcipNamedMessageFixtureV1 {
    pub name: String,
    pub message: AcipMessageEnvelopeV1,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AcipNegativeMessageCaseV1 {
    pub name: String,
    pub expected_error_substring: String,
    pub value: JsonValue,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AcipNegativeConversationCaseV1 {
    pub name: String,
    pub expected_error_substring: String,
    pub value: JsonValue,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AcipFixtureSetV1 {
    pub schema_version: String,
    pub valid_messages: Vec<AcipNamedMessageFixtureV1>,
    pub valid_conversation: AcipConversationEnvelopeV1,
    pub invalid_messages: Vec<AcipNegativeMessageCaseV1>,
    pub invalid_conversations: Vec<AcipNegativeConversationCaseV1>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AcipInvocationConstraintsV1 {
    pub policy_refs: Vec<String>,
    pub required_capabilities: Vec<String>,
    pub prohibited_actions: Vec<String>,
    pub requires_redaction: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AcipExpectedOutputV1 {
    pub output_id: String,
    pub output_kind: String,
    pub artifact_role: String,
    pub schema_ref: Option<String>,
    pub required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AcipInvocationStopPolicyV1 {
    pub max_turns: u32,
    pub max_output_artifacts: u32,
    pub completion_condition: String,
    pub stop_on_refusal: bool,
    pub stop_on_failure: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AcipResponseChannelV1 {
    pub kind: AcipResponseChannelKindV1,
    pub channel_ref: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AcipInvocationContractV1 {
    pub schema_version: String,
    pub invocation_id: String,
    pub conversation_id: String,
    pub causal_message_id: String,
    pub caller: AcipAddressV1,
    pub target: AcipAddressV1,
    pub intent: AcipIntentV1,
    pub purpose: String,
    pub input_refs: Vec<String>,
    pub constraints: AcipInvocationConstraintsV1,
    pub expected_outputs: Vec<AcipExpectedOutputV1>,
    pub stop_policy: AcipInvocationStopPolicyV1,
    pub authority_scope: AcipAuthorityScopeV1,
    pub decision_event_ref: String,
    pub response_channel: AcipResponseChannelV1,
    pub trace_requirement: AcipTraceRequirementV1,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AcipInvocationEventV1 {
    pub schema_version: String,
    pub invocation_id: String,
    pub conversation_id: String,
    pub causal_message_id: String,
    pub caller: AcipAddressV1,
    pub target: AcipAddressV1,
    pub contract_ref: Option<String>,
    pub contract_sha256: Option<String>,
    pub decision_event_ref: String,
    pub input_refs: Vec<String>,
    pub output_refs: Vec<String>,
    pub status: AcipInvocationStatusV1,
    pub stop_reason: String,
    pub refusal_code: Option<String>,
    pub failure_code: Option<String>,
    pub evidence_refs: Vec<String>,
    pub trace_requirement: AcipTraceRequirementV1,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AcipInvocationNegativeCaseV1 {
    pub name: String,
    pub expected_error_substring: String,
    pub contract: JsonValue,
    pub event: Option<JsonValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AcipInvocationFixtureSetV1 {
    pub schema_version: String,
    pub valid_contract: AcipInvocationContractV1,
    pub valid_completed_event: AcipInvocationEventV1,
    pub valid_refused_event: AcipInvocationEventV1,
    pub valid_failed_event: AcipInvocationEventV1,
    pub negative_cases: Vec<AcipInvocationNegativeCaseV1>,
}

pub fn acip_message_envelope_v1_schema_json() -> Result<String> {
    serde_json::to_string_pretty(&schema_for!(AcipMessageEnvelopeV1))
        .context("serialize ACIP message envelope v1 schema")
}

pub fn acip_fixture_set_v1_schema_json() -> Result<String> {
    serde_json::to_string_pretty(&schema_for!(AcipFixtureSetV1))
        .context("serialize ACIP fixture set v1 schema")
}

pub fn acip_invocation_contract_v1_schema_json() -> Result<String> {
    serde_json::to_string_pretty(&schema_for!(AcipInvocationContractV1))
        .context("serialize ACIP invocation contract v1 schema")
}

pub fn acip_invocation_event_v1_schema_json() -> Result<String> {
    serde_json::to_string_pretty(&schema_for!(AcipInvocationEventV1))
        .context("serialize ACIP invocation event v1 schema")
}

pub fn acip_invocation_fixture_set_v1_schema_json() -> Result<String> {
    serde_json::to_string_pretty(&schema_for!(AcipInvocationFixtureSetV1))
        .context("serialize ACIP invocation fixture set v1 schema")
}

pub fn validate_acip_message_envelope_v1_value(value: &JsonValue) -> Result<AcipMessageEnvelopeV1> {
    let envelope: AcipMessageEnvelopeV1 =
        serde_json::from_value(value.clone()).context("parse ACIP message envelope v1")?;
    validate_acip_message_envelope_v1(&envelope)?;
    Ok(envelope)
}

pub fn validate_acip_message_envelope_v1(envelope: &AcipMessageEnvelopeV1) -> Result<()> {
    if envelope.schema_version != ACIP_MESSAGE_SCHEMA_VERSION {
        return Err(anyhow!(
            "ACIP message envelope requires schema_version '{}'",
            ACIP_MESSAGE_SCHEMA_VERSION
        ));
    }
    validate_id(&envelope.message_id, "message_id")?;
    validate_id(&envelope.conversation_id, "conversation_id")?;
    validate_timestamp(&envelope.timestamp_utc, "timestamp_utc")?;
    if envelope.monotonic_order == 0 {
        return Err(anyhow!("monotonic_order must be positive"));
    }
    validate_address(&envelope.sender, "sender")?;
    validate_address(&envelope.recipient, "recipient")?;
    if envelope.sender == envelope.recipient {
        return Err(anyhow!("sender and recipient must not be identical"));
    }
    if let Some(correlation_id) = &envelope.correlation_id {
        validate_id(correlation_id, "correlation_id")?;
    }
    if let Some(prior_message_id) = &envelope.prior_message_id {
        validate_id(prior_message_id, "prior_message_id")?;
    }
    if envelope.content.trim().is_empty()
        && envelope.payload_refs.is_empty()
        && envelope.attachments.is_empty()
    {
        return Err(anyhow!(
            "message must contain content, payload_refs, or attachments"
        ));
    }
    if envelope.content.chars().count() > MAX_CONTENT_CHARS {
        return Err(anyhow!(
            "content exceeds bounded inline posture of {MAX_CONTENT_CHARS} characters"
        ));
    }
    if envelope.payload_refs.len() > MAX_LIST_LEN {
        return Err(anyhow!("payload_refs exceeds bounded list length"));
    }
    if envelope.artifact_refs.len() > MAX_LIST_LEN {
        return Err(anyhow!("artifact_refs exceeds bounded list length"));
    }
    if envelope.attachments.len() > MAX_LIST_LEN {
        return Err(anyhow!("attachments exceeds bounded list length"));
    }
    for payload_ref in &envelope.payload_refs {
        validate_payload_ref(payload_ref)?;
    }
    for artifact_ref in &envelope.artifact_refs {
        validate_repo_relative_ref(artifact_ref, "artifact_refs[]")?;
    }
    for attachment in &envelope.attachments {
        validate_attachment_ref(attachment)?;
    }
    if let Some(scope) = &envelope.authority_scope {
        validate_authority_scope(scope)?;
    }
    Ok(())
}

pub fn validate_acip_conversation_envelope_v1_value(
    value: &JsonValue,
) -> Result<AcipConversationEnvelopeV1> {
    let envelope: AcipConversationEnvelopeV1 =
        serde_json::from_value(value.clone()).context("parse ACIP conversation envelope v1")?;
    validate_acip_conversation_envelope_v1(&envelope)?;
    Ok(envelope)
}

pub fn validate_acip_conversation_envelope_v1(envelope: &AcipConversationEnvelopeV1) -> Result<()> {
    if envelope.schema_version != ACIP_CONVERSATION_SCHEMA_VERSION {
        return Err(anyhow!(
            "ACIP conversation envelope requires schema_version '{}'",
            ACIP_CONVERSATION_SCHEMA_VERSION
        ));
    }
    if envelope.messages.is_empty() {
        return Err(anyhow!(
            "ACIP conversation envelope requires at least one message"
        ));
    }
    let mut seen_message_ids = BTreeSet::new();
    let mut prior_order = 0_u64;
    let mut conversation_id: Option<&str> = None;
    for message in &envelope.messages {
        validate_acip_message_envelope_v1(message)?;
        if !seen_message_ids.insert(message.message_id.clone()) {
            return Err(anyhow!(
                "conversation contains duplicate message_id '{}'",
                message.message_id
            ));
        }
        if prior_order >= message.monotonic_order {
            return Err(anyhow!(
                "conversation messages must preserve strictly increasing monotonic_order"
            ));
        }
        prior_order = message.monotonic_order;
        if let Some(expected) = conversation_id {
            if expected != message.conversation_id {
                return Err(anyhow!(
                    "conversation contains message from a different conversation_id"
                ));
            }
        } else {
            conversation_id = Some(message.conversation_id.as_str());
        }
    }
    Ok(())
}

pub fn validate_acip_invocation_contract_v1_value(
    value: &JsonValue,
) -> Result<AcipInvocationContractV1> {
    let contract: AcipInvocationContractV1 =
        serde_json::from_value(value.clone()).context("parse ACIP invocation contract v1")?;
    validate_acip_invocation_contract_v1(&contract)?;
    Ok(contract)
}

pub fn validate_acip_invocation_contract_v1(contract: &AcipInvocationContractV1) -> Result<()> {
    if contract.schema_version != ACIP_INVOCATION_CONTRACT_SCHEMA_VERSION {
        return Err(anyhow!(
            "ACIP invocation contract requires schema_version '{}'",
            ACIP_INVOCATION_CONTRACT_SCHEMA_VERSION
        ));
    }
    validate_id(&contract.invocation_id, "invocation_id")?;
    validate_id(&contract.conversation_id, "conversation_id")?;
    validate_id(&contract.causal_message_id, "causal_message_id")?;
    validate_address(&contract.caller, "caller")?;
    validate_address(&contract.target, "target")?;
    if contract.caller == contract.target {
        return Err(anyhow!("caller and target must not be identical"));
    }
    match contract.intent {
        AcipIntentV1::InvocationSetup
        | AcipIntentV1::ReviewRequest
        | AcipIntentV1::CodingRequest
        | AcipIntentV1::Delegation => {}
        _ => {
            return Err(anyhow!(
                "invocation contract intent must be invocation_setup, review_request, coding_request, or delegation"
            ))
        }
    }
    validate_non_empty(&contract.purpose, "purpose")?;
    if contract.input_refs.is_empty() {
        return Err(anyhow!("input_refs must not be empty"));
    }
    if contract.input_refs.len() > MAX_LIST_LEN {
        return Err(anyhow!("input_refs exceeds bounded list length"));
    }
    for input_ref in &contract.input_refs {
        validate_repo_relative_ref(input_ref, "input_refs[]")?;
    }
    validate_invocation_constraints(&contract.constraints)?;
    validate_invocation_authority_alignment(contract)?;
    if contract.expected_outputs.is_empty() {
        return Err(anyhow!("expected_outputs must not be empty"));
    }
    if contract.expected_outputs.len() > MAX_LIST_LEN {
        return Err(anyhow!("expected_outputs exceeds bounded list length"));
    }
    validate_expected_outputs(&contract.expected_outputs)?;
    validate_stop_policy(&contract.stop_policy)?;
    validate_authority_scope(&contract.authority_scope)?;
    validate_gate_decision_ref(&contract.decision_event_ref, "decision_event_ref")?;
    validate_response_channel(&contract.response_channel)?;
    Ok(())
}

pub fn validate_acip_invocation_event_v1_value(value: &JsonValue) -> Result<AcipInvocationEventV1> {
    let event: AcipInvocationEventV1 =
        serde_json::from_value(value.clone()).context("parse ACIP invocation event v1")?;
    validate_acip_invocation_event_v1(&event)?;
    Ok(event)
}

pub fn validate_acip_invocation_event_v1(event: &AcipInvocationEventV1) -> Result<()> {
    if event.schema_version != ACIP_INVOCATION_EVENT_SCHEMA_VERSION {
        return Err(anyhow!(
            "ACIP invocation event requires schema_version '{}'",
            ACIP_INVOCATION_EVENT_SCHEMA_VERSION
        ));
    }
    validate_id(&event.invocation_id, "invocation_id")?;
    validate_id(&event.conversation_id, "conversation_id")?;
    validate_id(&event.causal_message_id, "causal_message_id")?;
    validate_address(&event.caller, "caller")?;
    validate_address(&event.target, "target")?;
    validate_gate_decision_ref(&event.decision_event_ref, "decision_event_ref")?;
    if event.input_refs.len() > MAX_LIST_LEN {
        return Err(anyhow!("input_refs exceeds bounded list length"));
    }
    for input_ref in &event.input_refs {
        validate_repo_relative_ref(input_ref, "input_refs[]")?;
    }
    if event.output_refs.len() > MAX_LIST_LEN {
        return Err(anyhow!("output_refs exceeds bounded list length"));
    }
    for output_ref in &event.output_refs {
        validate_repo_relative_ref(output_ref, "output_refs[]")?;
    }
    if let Some(contract_ref) = &event.contract_ref {
        validate_repo_relative_ref(contract_ref, "contract_ref")?;
    }
    if let Some(contract_sha256) = &event.contract_sha256 {
        validate_sha256(contract_sha256, "contract_sha256")?;
    }
    if event.contract_ref.is_none() && event.contract_sha256.is_none() {
        return Err(anyhow!(
            "invocation event must carry contract_ref or contract_sha256"
        ));
    }
    validate_non_empty(&event.stop_reason, "stop_reason")?;
    if event.evidence_refs.len() > MAX_LIST_LEN {
        return Err(anyhow!("evidence_refs exceeds bounded list length"));
    }
    for evidence_ref in &event.evidence_refs {
        validate_repo_relative_ref(evidence_ref, "evidence_refs[]")?;
    }
    validate_invocation_status_consistency(event)?;
    Ok(())
}

pub fn validate_acip_invocation_event_against_contract(
    contract: &AcipInvocationContractV1,
    event: &AcipInvocationEventV1,
) -> Result<()> {
    validate_acip_invocation_contract_v1(contract)?;
    validate_acip_invocation_event_v1(event)?;

    if contract.invocation_id != event.invocation_id {
        return Err(anyhow!(
            "invocation event must match contract invocation_id"
        ));
    }
    if contract.conversation_id != event.conversation_id {
        return Err(anyhow!(
            "invocation event must match contract conversation_id"
        ));
    }
    if contract.causal_message_id != event.causal_message_id {
        return Err(anyhow!(
            "invocation event must match contract causal_message_id"
        ));
    }
    if contract.caller != event.caller || contract.target != event.target {
        return Err(anyhow!(
            "invocation event caller and target must match contract identity binding"
        ));
    }
    if contract.decision_event_ref != event.decision_event_ref {
        return Err(anyhow!(
            "invocation event must preserve the contract decision_event_ref"
        ));
    }
    if contract.input_refs != event.input_refs {
        return Err(anyhow!(
            "invocation event must preserve the contract input_refs"
        ));
    }
    if contract.trace_requirement != event.trace_requirement {
        return Err(anyhow!(
            "invocation event must preserve the contract trace_requirement"
        ));
    }

    let required_output_count = contract
        .expected_outputs
        .iter()
        .filter(|expected| expected.required)
        .count();

    match event.status {
        AcipInvocationStatusV1::Requested => {}
        AcipInvocationStatusV1::Completed => {
            if event.output_refs.len() > contract.stop_policy.max_output_artifacts as usize {
                return Err(anyhow!(
                    "invocation event exceeds stop_policy.max_output_artifacts"
                ));
            }
            if event.output_refs.len() < required_output_count {
                return Err(anyhow!(
                    "completed invocation must satisfy declared required output contracts"
                ));
            }
            for expected in contract
                .expected_outputs
                .iter()
                .filter(|expected| expected.required)
            {
                let matched = event.output_refs.iter().any(|output_ref| {
                    output_ref.contains(&expected.output_id)
                        || output_ref.contains(&expected.output_kind)
                });
                if !matched {
                    return Err(anyhow!(
                        "completed invocation must satisfy declared required output contracts"
                    ));
                }
            }
        }
        AcipInvocationStatusV1::Partial => {
            if event.output_refs.len() > contract.stop_policy.max_output_artifacts as usize {
                return Err(anyhow!(
                    "invocation event exceeds stop_policy.max_output_artifacts"
                ));
            }
            if event.output_refs.is_empty() {
                return Err(anyhow!(
                    "partial invocation must emit at least one output ref"
                ));
            }
        }
        AcipInvocationStatusV1::Refused | AcipInvocationStatusV1::Failed => {}
    }
    Ok(())
}

pub fn validate_acip_fixture_set_v1(fixtures: &AcipFixtureSetV1) -> Result<()> {
    if fixtures.schema_version != ACIP_FIXTURE_SCHEMA_VERSION {
        return Err(anyhow!(
            "ACIP fixture set requires schema_version '{}'",
            ACIP_FIXTURE_SCHEMA_VERSION
        ));
    }
    if fixtures.valid_messages.is_empty() {
        return Err(anyhow!("ACIP fixture set requires valid_messages"));
    }
    if fixtures.invalid_messages.is_empty() {
        return Err(anyhow!("ACIP fixture set requires invalid_messages"));
    }
    if fixtures.invalid_conversations.is_empty() {
        return Err(anyhow!("ACIP fixture set requires invalid_conversations"));
    }

    let mut seen_names = BTreeSet::new();
    for fixture in &fixtures.valid_messages {
        validate_id(&fixture.name, "valid_messages[].name")?;
        if !seen_names.insert(fixture.name.clone()) {
            return Err(anyhow!(
                "ACIP fixture set contains duplicate valid fixture '{}'",
                fixture.name
            ));
        }
        validate_acip_message_envelope_v1(&fixture.message)?;
    }
    validate_acip_conversation_envelope_v1(&fixtures.valid_conversation)?;

    for case in &fixtures.invalid_messages {
        validate_negative_case_name(&case.name, "invalid_messages[].name")?;
        validate_negative_case(case.value.clone(), &case.expected_error_substring, true)?;
    }
    for case in &fixtures.invalid_conversations {
        validate_negative_case_name(&case.name, "invalid_conversations[].name")?;
        validate_negative_case(case.value.clone(), &case.expected_error_substring, false)?;
    }
    Ok(())
}

pub fn validate_acip_invocation_fixture_set_v1(
    fixtures: &AcipInvocationFixtureSetV1,
) -> Result<()> {
    if fixtures.schema_version != ACIP_INVOCATION_FIXTURE_SCHEMA_VERSION {
        return Err(anyhow!(
            "ACIP invocation fixture set requires schema_version '{}'",
            ACIP_INVOCATION_FIXTURE_SCHEMA_VERSION
        ));
    }
    if fixtures.negative_cases.is_empty() {
        return Err(anyhow!(
            "ACIP invocation fixture set requires negative_cases"
        ));
    }

    validate_acip_invocation_contract_v1(&fixtures.valid_contract)?;
    validate_acip_invocation_event_against_contract(
        &fixtures.valid_contract,
        &fixtures.valid_completed_event,
    )?;
    validate_acip_invocation_event_against_contract(
        &fixtures.valid_contract,
        &fixtures.valid_refused_event,
    )?;
    validate_acip_invocation_event_against_contract(
        &fixtures.valid_contract,
        &fixtures.valid_failed_event,
    )?;

    for case in &fixtures.negative_cases {
        validate_negative_case_name(&case.name, "negative_cases[].name")?;
        validate_non_empty(
            &case.expected_error_substring,
            "negative_cases[].expected_error_substring",
        )?;
        let contract = validate_acip_invocation_contract_v1_value(&case.contract);
        match (&case.event, contract) {
            (Some(event_value), Ok(contract)) => {
                let event = validate_acip_invocation_event_v1_value(event_value);
                let result = match event {
                    Ok(event) => validate_acip_invocation_event_against_contract(&contract, &event),
                    Err(error) => Err(error),
                };
                validate_negative_result(result, &case.expected_error_substring)?;
            }
            (Some(_), Err(error)) | (None, Err(error)) => {
                validate_negative_result(Err(error), &case.expected_error_substring)?;
            }
            (None, Ok(_)) => {
                return Err(anyhow!(
                    "negative case '{}' unexpectedly validated without event failure",
                    case.name
                ));
            }
        }
    }

    Ok(())
}

pub fn acip_fixture_set_v1() -> AcipFixtureSetV1 {
    let consultation = sample_message(
        "msg-consultation-0001",
        "conv-consult-001",
        1,
        AcipIntentV1::Consultation,
        "Can you compare these two planning options and call out the stronger evidence chain?",
    );
    let invocation_setup = sample_message_with_payloads(
        "msg-invoke-0001",
        "conv-invoke-001",
        1,
        AcipIntentV1::InvocationSetup,
        "Please prepare the bounded invocation contract and preserve policy hooks.",
        vec![payload_ref(
            "structured_prompt",
            "runtime/comms/contracts/invocation_setup.stp.json",
            "application/json",
            1_812,
            Some("Bounded invocation contract with explicit stop conditions."),
        )],
    );
    let review_request = sample_message_with_payloads(
        "msg-review-0001",
        "conv-review-001",
        1,
        AcipIntentV1::ReviewRequest,
        "Please review the attached packet for correctness, completeness, and policy drift.",
        vec![payload_ref(
            "review_packet",
            "runtime/comms/review/review_packet.json",
            "application/json",
            2_104,
            Some("Review packet plus SRP references."),
        )],
    );
    let coding_request = sample_message_with_payloads(
        "msg-coding-0001",
        "conv-coding-001",
        1,
        AcipIntentV1::CodingRequest,
        "Please implement the bounded change without widening the issue scope.",
        vec![payload_ref(
            "task_bundle",
            "runtime/comms/coding/task_bundle.json",
            "application/json",
            1_944,
            Some("Bounded coding task bundle with acceptance criteria."),
        )],
    );
    let delegation = sample_message_with_payloads(
        "msg-delegate-0001",
        "conv-delegate-001",
        1,
        AcipIntentV1::Delegation,
        "I am delegating a bounded artifact-generation task and preserving parent accountability.",
        vec![payload_ref(
            "delegation_contract",
            "runtime/comms/delegation/delegation_contract.json",
            "application/json",
            1_688,
            Some("Delegation contract with explicit review return path."),
        )],
    );
    let negotiation = sample_message(
        "msg-negotiate-0001",
        "conv-negotiate-001",
        1,
        AcipIntentV1::Negotiation,
        "I can take the trace task if you cover conformance fixtures and we keep one shared authority basis.",
    );
    let conversation = AcipConversationEnvelopeV1 {
        schema_version: ACIP_CONVERSATION_SCHEMA_VERSION.to_string(),
        messages: vec![
            sample_message(
                "msg-conversation-0001",
                "conv-shared-001",
                1,
                AcipIntentV1::Conversation,
                "Hi, I have code that needs review. Do you have time?",
            ),
            sample_message(
                "msg-conversation-0002",
                "conv-shared-001",
                2,
                AcipIntentV1::Consultation,
                "Yes. Share the bounded packet and I will keep the review surface narrow.",
            ),
            sample_message_with_payloads(
                "msg-conversation-0003",
                "conv-shared-001",
                3,
                AcipIntentV1::ReviewRequest,
                "Here is the packet and the trace anchor.",
                vec![payload_ref(
                    "review_packet",
                    "runtime/comms/review/packet.json",
                    "application/json",
                    1_096,
                    Some("Packet plus trace anchor."),
                )],
            ),
        ],
    };

    AcipFixtureSetV1 {
        schema_version: ACIP_FIXTURE_SCHEMA_VERSION.to_string(),
        valid_messages: vec![
            AcipNamedMessageFixtureV1 {
                name: "conversation".to_string(),
                message: sample_message(
                    "msg-conversation-standalone-0001",
                    "conv-standalone-001",
                    1,
                    AcipIntentV1::Conversation,
                    "Can we talk through the transition plan before we bind an invocation?",
                ),
            },
            AcipNamedMessageFixtureV1 {
                name: "consultation".to_string(),
                message: consultation,
            },
            AcipNamedMessageFixtureV1 {
                name: "invocation_setup".to_string(),
                message: invocation_setup,
            },
            AcipNamedMessageFixtureV1 {
                name: "review_request".to_string(),
                message: review_request,
            },
            AcipNamedMessageFixtureV1 {
                name: "coding_request".to_string(),
                message: coding_request,
            },
            AcipNamedMessageFixtureV1 {
                name: "delegation".to_string(),
                message: delegation,
            },
            AcipNamedMessageFixtureV1 {
                name: "negotiation".to_string(),
                message: negotiation,
            },
        ],
        valid_conversation: conversation,
        invalid_messages: vec![
            AcipNegativeMessageCaseV1 {
                name: "missing_identity".to_string(),
                expected_error_substring: "sender.id must not be empty".to_string(),
                value: json!({
                    "schema_version": ACIP_MESSAGE_SCHEMA_VERSION,
                    "message_id": "msg-bad-0001",
                    "conversation_id": "conv-bad-001",
                    "timestamp_utc": "2026-04-28T19:00:00Z",
                    "monotonic_order": 1,
                    "sender": {"kind": "agent", "id": ""},
                    "recipient": {"kind": "agent", "id": "reviewer.agent"},
                    "intent": "consultation",
                    "visibility": "private",
                    "trace_requirement": "summary",
                    "content": "Please help.",
                    "payload_refs": [],
                    "artifact_refs": [],
                    "attachments": [],
                    "authority_scope": null,
                    "correlation_id": null,
                    "prior_message_id": null
                }),
            },
            AcipNegativeMessageCaseV1 {
                name: "malformed_refs".to_string(),
                expected_error_substring: "payload_ref must be a repository-relative path"
                    .to_string(),
                value: json!({
                    "schema_version": ACIP_MESSAGE_SCHEMA_VERSION,
                    "message_id": "msg-bad-0002",
                    "conversation_id": "conv-bad-002",
                    "timestamp_utc": "2026-04-28T19:01:00Z",
                    "monotonic_order": 1,
                    "sender": {"kind": "agent", "id": "planner.agent"},
                    "recipient": {"kind": "agent", "id": "reviewer.agent"},
                    "intent": "review_request",
                    "visibility": "shared",
                    "trace_requirement": "summary",
                    "content": "Please review this packet.",
                    "payload_refs": [{
                        "payload_kind": "review_packet",
                        "payload_ref": "/Users/daniel/private/review_packet.json",
                        "media_type": "application/json",
                        "content_sha256": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
                        "byte_length": 512,
                        "inline_summary": "Unsafe path."
                    }],
                    "artifact_refs": [],
                    "attachments": [],
                    "authority_scope": null,
                    "correlation_id": null,
                    "prior_message_id": null
                }),
            },
            AcipNegativeMessageCaseV1 {
                name: "unsafe_visibility".to_string(),
                expected_error_substring: "unknown variant".to_string(),
                value: json!({
                    "schema_version": ACIP_MESSAGE_SCHEMA_VERSION,
                    "message_id": "msg-bad-0003",
                    "conversation_id": "conv-bad-003",
                    "timestamp_utc": "2026-04-28T19:02:00Z",
                    "monotonic_order": 1,
                    "sender": {"kind": "agent", "id": "planner.agent"},
                    "recipient": {"kind": "group", "id": "operators"},
                    "intent": "conversation",
                    "visibility": "operator_only",
                    "trace_requirement": "none",
                    "content": "This should fail.",
                    "payload_refs": [],
                    "artifact_refs": [],
                    "attachments": [],
                    "authority_scope": null,
                    "correlation_id": null,
                    "prior_message_id": null
                }),
            },
            AcipNegativeMessageCaseV1 {
                name: "overlarge_inline_payload_posture".to_string(),
                expected_error_substring: "inline_summary exceeds bounded inline posture"
                    .to_string(),
                value: json!({
                    "schema_version": ACIP_MESSAGE_SCHEMA_VERSION,
                    "message_id": "msg-bad-0004",
                    "conversation_id": "conv-bad-004",
                    "timestamp_utc": "2026-04-28T19:03:00Z",
                    "monotonic_order": 1,
                    "sender": {"kind": "agent", "id": "planner.agent"},
                    "recipient": {"kind": "agent", "id": "coder.agent"},
                    "intent": "coding_request",
                    "visibility": "private",
                    "trace_requirement": "summary",
                    "content": "Please implement the change.",
                    "payload_refs": [{
                        "payload_kind": "task_bundle",
                        "payload_ref": "runtime/comms/coding/task_bundle.json",
                        "media_type": "application/json",
                        "content_sha256": "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
                        "byte_length": 2048,
                        "inline_summary": "X".repeat(MAX_INLINE_SUMMARY_CHARS + 1)
                    }],
                    "artifact_refs": [],
                    "attachments": [],
                    "authority_scope": null,
                    "correlation_id": null,
                    "prior_message_id": null
                }),
            },
            AcipNegativeMessageCaseV1 {
                name: "unsupported_authority_assertion".to_string(),
                expected_error_substring: "unsupported authority_scope.allowed_actions".to_string(),
                value: json!({
                    "schema_version": ACIP_MESSAGE_SCHEMA_VERSION,
                    "message_id": "msg-bad-0005",
                    "conversation_id": "conv-bad-005",
                    "timestamp_utc": "2026-04-28T19:04:00Z",
                    "monotonic_order": 1,
                    "sender": {"kind": "agent", "id": "planner.agent"},
                    "recipient": {"kind": "agent", "id": "delegate.agent"},
                    "intent": "delegation",
                    "visibility": "shared",
                    "trace_requirement": "full",
                    "content": "Please take this bounded task.",
                    "payload_refs": [],
                    "artifact_refs": ["runtime/comms/delegation/contract.json"],
                    "attachments": [],
                    "authority_scope": {
                        "allowed_actions": ["root_inspection"],
                        "authority_basis_refs": ["runtime/comms/delegation/basis.json"],
                        "delegation_permitted": true
                    },
                    "correlation_id": null,
                    "prior_message_id": null
                }),
            },
        ],
        invalid_conversations: vec![AcipNegativeConversationCaseV1 {
            name: "stale_ordering".to_string(),
            expected_error_substring:
                "conversation messages must preserve strictly increasing monotonic_order"
                    .to_string(),
            value: json!({
                "schema_version": ACIP_CONVERSATION_SCHEMA_VERSION,
                "messages": [
                    sample_message("msg-order-0001", "conv-order-001", 2, AcipIntentV1::Conversation, "Late."),
                    sample_message("msg-order-0002", "conv-order-001", 1, AcipIntentV1::Conversation, "Early.")
                ]
            }),
        }],
    }
}

pub fn acip_invocation_fixture_set_v1() -> AcipInvocationFixtureSetV1 {
    let valid_contract = sample_invocation_contract();

    AcipInvocationFixtureSetV1 {
        schema_version: ACIP_INVOCATION_FIXTURE_SCHEMA_VERSION.to_string(),
        valid_contract: valid_contract.clone(),
        valid_completed_event: sample_invocation_event(
            &valid_contract,
            AcipInvocationStatusV1::Completed,
            vec![
                "runtime/comms/invocation/review_report.json".to_string(),
                "runtime/comms/invocation/review_summary.json".to_string(),
            ],
            "completed_output_contract".to_string(),
            None,
            None,
            vec!["runtime/comms/invocation/evidence/review_trace.json".to_string()],
        ),
        valid_refused_event: sample_invocation_event(
            &valid_contract,
            AcipInvocationStatusV1::Refused,
            Vec::new(),
            "policy_refusal".to_string(),
            Some("operator_review_required".to_string()),
            None,
            vec!["runtime/comms/invocation/evidence/refusal_note.json".to_string()],
        ),
        valid_failed_event: sample_invocation_event(
            &valid_contract,
            AcipInvocationStatusV1::Failed,
            Vec::new(),
            "output_validation_failed".to_string(),
            None,
            Some("output_contract_mismatch".to_string()),
            vec!["runtime/comms/invocation/evidence/failure_packet.json".to_string()],
        ),
        negative_cases: vec![
            AcipInvocationNegativeCaseV1 {
                name: "missing_gate_rejects_governed_invocation".to_string(),
                expected_error_substring: "missing field `decision_event_ref`".to_string(),
                contract: json!({
                    "schema_version": ACIP_INVOCATION_CONTRACT_SCHEMA_VERSION,
                    "invocation_id": "invoke-0002",
                    "conversation_id": "conv-invoke-001",
                    "causal_message_id": "msg-review-0001",
                    "caller": {"kind": "agent", "id": "planner.agent"},
                    "target": {"kind": "agent", "id": "reviewer.agent"},
                    "intent": "review_request",
                    "purpose": "Perform bounded review work.",
                    "input_refs": ["runtime/comms/invocation/review_packet.json"],
                    "constraints": {
                        "policy_refs": ["runtime/comms/policy/review_policy.json"],
                        "required_capabilities": ["review"],
                        "prohibited_actions": ["merge"],
                        "requires_redaction": true
                    },
                    "expected_outputs": [{
                        "output_id": "review_report",
                        "output_kind": "review_report",
                        "artifact_role": "primary_report",
                        "schema_ref": "schemas/review_report.schema.json",
                        "required": true
                    }],
                    "stop_policy": {
                        "max_turns": 1,
                        "max_output_artifacts": 2,
                        "completion_condition": "emit refusal or report",
                        "stop_on_refusal": true,
                        "stop_on_failure": true
                    },
                    "authority_scope": {
                        "allowed_actions": ["review", "share_artifact"],
                        "authority_basis_refs": ["runtime/comms/authority/review_basis.json"],
                        "delegation_permitted": false
                    },
                    "response_channel": {
                        "kind": "direct_reply",
                        "channel_ref": "reply.review.thread"
                    },
                    "trace_requirement": "full"
                }),
                event: None,
            },
            AcipInvocationNegativeCaseV1 {
                name: "ambiguous_stop_policy_rejected".to_string(),
                expected_error_substring: "stop_policy.max_turns must be positive".to_string(),
                contract: json!({
                    "schema_version": ACIP_INVOCATION_CONTRACT_SCHEMA_VERSION,
                    "invocation_id": "invoke-0003",
                    "conversation_id": "conv-invoke-001",
                    "causal_message_id": "msg-review-0001",
                    "caller": {"kind": "agent", "id": "planner.agent"},
                    "target": {"kind": "agent", "id": "reviewer.agent"},
                    "intent": "review_request",
                    "purpose": "Perform bounded review work.",
                    "input_refs": ["runtime/comms/invocation/review_packet.json"],
                    "constraints": {
                        "policy_refs": ["runtime/comms/policy/review_policy.json"],
                        "required_capabilities": ["review"],
                        "prohibited_actions": ["merge"],
                        "requires_redaction": true
                    },
                    "expected_outputs": [{
                        "output_id": "review_report",
                        "output_kind": "review_report",
                        "artifact_role": "primary_report",
                        "schema_ref": "schemas/review_report.schema.json",
                        "required": true
                    }],
                    "stop_policy": {
                        "max_turns": 0,
                        "max_output_artifacts": 2,
                        "completion_condition": "emit refusal or report",
                        "stop_on_refusal": true,
                        "stop_on_failure": true
                    },
                    "authority_scope": {
                        "allowed_actions": ["review", "share_artifact"],
                        "authority_basis_refs": ["runtime/comms/authority/review_basis.json"],
                        "delegation_permitted": false
                    },
                    "decision_event_ref": "gate.review-0001",
                    "response_channel": {
                        "kind": "direct_reply",
                        "channel_ref": "reply.review.thread"
                    },
                    "trace_requirement": "full"
                }),
                event: None,
            },
            AcipInvocationNegativeCaseV1 {
                name: "unsafe_input_refs_rejected".to_string(),
                expected_error_substring: "input_refs[] must be a repository-relative path"
                    .to_string(),
                contract: json!({
                    "schema_version": ACIP_INVOCATION_CONTRACT_SCHEMA_VERSION,
                    "invocation_id": "invoke-0004",
                    "conversation_id": "conv-invoke-001",
                    "causal_message_id": "msg-review-0001",
                    "caller": {"kind": "agent", "id": "planner.agent"},
                    "target": {"kind": "agent", "id": "reviewer.agent"},
                    "intent": "review_request",
                    "purpose": "Perform bounded review work.",
                    "input_refs": ["/Users/daniel/private/review_packet.json"],
                    "constraints": {
                        "policy_refs": ["runtime/comms/policy/review_policy.json"],
                        "required_capabilities": ["review"],
                        "prohibited_actions": ["merge"],
                        "requires_redaction": true
                    },
                    "expected_outputs": [{
                        "output_id": "review_report",
                        "output_kind": "review_report",
                        "artifact_role": "primary_report",
                        "schema_ref": "schemas/review_report.schema.json",
                        "required": true
                    }],
                    "stop_policy": {
                        "max_turns": 1,
                        "max_output_artifacts": 2,
                        "completion_condition": "emit refusal or report",
                        "stop_on_refusal": true,
                        "stop_on_failure": true
                    },
                    "authority_scope": {
                        "allowed_actions": ["review", "share_artifact"],
                        "authority_basis_refs": ["runtime/comms/authority/review_basis.json"],
                        "delegation_permitted": false
                    },
                    "decision_event_ref": "gate.review-0001",
                    "response_channel": {
                        "kind": "direct_reply",
                        "channel_ref": "reply.review.thread"
                    },
                    "trace_requirement": "full"
                }),
                event: None,
            },
            AcipInvocationNegativeCaseV1 {
                name: "status_refusal_inconsistency_rejected".to_string(),
                expected_error_substring:
                    "completed invocation event must not carry refusal_code or failure_code"
                        .to_string(),
                contract: serde_json::to_value(valid_contract.clone()).expect("contract json"),
                event: Some(json!({
                    "schema_version": ACIP_INVOCATION_EVENT_SCHEMA_VERSION,
                    "invocation_id": valid_contract.invocation_id,
                    "conversation_id": valid_contract.conversation_id,
                    "causal_message_id": valid_contract.causal_message_id,
                    "caller": {"kind": "agent", "id": "planner.agent"},
                    "target": {"kind": "agent", "id": "reviewer.agent"},
                    "contract_ref": "runtime/comms/invocation/contracts/review_request.json",
                    "contract_sha256": "89abcdef0123456789abcdef0123456789abcdef0123456789abcdef01234567",
                    "decision_event_ref": valid_contract.decision_event_ref,
                    "input_refs": valid_contract.input_refs,
                    "output_refs": ["runtime/comms/invocation/review_report.json"],
                    "status": "completed",
                    "stop_reason": "completed_output_contract",
                    "refusal_code": "should-not-be-here",
                    "failure_code": null,
                    "evidence_refs": ["runtime/comms/invocation/evidence/review_trace.json"],
                    "trace_requirement": "full"
                })),
            },
            AcipInvocationNegativeCaseV1 {
                name: "output_contract_mismatch_rejected".to_string(),
                expected_error_substring:
                    "completed invocation must satisfy declared required output contracts"
                        .to_string(),
                contract: serde_json::to_value(valid_contract.clone()).expect("contract json"),
                event: Some(json!({
                    "schema_version": ACIP_INVOCATION_EVENT_SCHEMA_VERSION,
                    "invocation_id": valid_contract.invocation_id,
                    "conversation_id": valid_contract.conversation_id,
                    "causal_message_id": valid_contract.causal_message_id,
                    "caller": {"kind": "agent", "id": "planner.agent"},
                    "target": {"kind": "agent", "id": "reviewer.agent"},
                    "contract_ref": "runtime/comms/invocation/contracts/review_request.json",
                    "contract_sha256": "89abcdef0123456789abcdef0123456789abcdef0123456789abcdef01234567",
                    "decision_event_ref": valid_contract.decision_event_ref,
                    "input_refs": valid_contract.input_refs,
                    "output_refs": ["runtime/comms/invocation/operator_summary.json"],
                    "status": "completed",
                    "stop_reason": "completed_output_contract",
                    "refusal_code": null,
                    "failure_code": null,
                    "evidence_refs": ["runtime/comms/invocation/evidence/review_trace.json"],
                    "trace_requirement": "full"
                })),
            },
        ],
    }
}

fn sample_message(
    message_id: &str,
    conversation_id: &str,
    monotonic_order: u64,
    intent: AcipIntentV1,
    content: &str,
) -> AcipMessageEnvelopeV1 {
    AcipMessageEnvelopeV1 {
        schema_version: ACIP_MESSAGE_SCHEMA_VERSION.to_string(),
        message_id: message_id.to_string(),
        conversation_id: conversation_id.to_string(),
        timestamp_utc: format!("2026-04-28T19:{:02}:00Z", monotonic_order.saturating_sub(1)),
        monotonic_order,
        sender: AcipAddressV1 {
            kind: AcipAddressKindV1::Agent,
            id: "planner.agent".to_string(),
        },
        recipient: AcipAddressV1 {
            kind: AcipAddressKindV1::Agent,
            id: "reviewer.agent".to_string(),
        },
        intent,
        visibility: AcipVisibilityV1::Private,
        trace_requirement: AcipTraceRequirementV1::Summary,
        content: content.to_string(),
        payload_refs: Vec::new(),
        artifact_refs: vec!["runtime/comms/trace/thread_anchor.json".to_string()],
        attachments: Vec::new(),
        authority_scope: Some(AcipAuthorityScopeV1 {
            allowed_actions: vec!["consult".to_string(), "share_artifact".to_string()],
            authority_basis_refs: vec![
                "runtime/comms/authority/consultation_basis.json".to_string()
            ],
            delegation_permitted: false,
        }),
        correlation_id: None,
        prior_message_id: None,
    }
}

fn sample_message_with_payloads(
    message_id: &str,
    conversation_id: &str,
    monotonic_order: u64,
    intent: AcipIntentV1,
    content: &str,
    payload_refs: Vec<AcipPayloadRefV1>,
) -> AcipMessageEnvelopeV1 {
    let mut message = sample_message(
        message_id,
        conversation_id,
        monotonic_order,
        intent,
        content,
    );
    message.payload_refs = payload_refs;
    message.authority_scope = Some(AcipAuthorityScopeV1 {
        allowed_actions: vec![
            "consult".to_string(),
            "invoke".to_string(),
            "review".to_string(),
            "delegate".to_string(),
            "share_artifact".to_string(),
        ],
        authority_basis_refs: vec!["runtime/comms/authority/invocation_basis.json".to_string()],
        delegation_permitted: true,
    });
    message
}

fn sample_invocation_contract() -> AcipInvocationContractV1 {
    AcipInvocationContractV1 {
        schema_version: ACIP_INVOCATION_CONTRACT_SCHEMA_VERSION.to_string(),
        invocation_id: "invoke-0001".to_string(),
        conversation_id: "conv-review-001".to_string(),
        causal_message_id: "msg-review-0001".to_string(),
        caller: AcipAddressV1 {
            kind: AcipAddressKindV1::Agent,
            id: "planner.agent".to_string(),
        },
        target: AcipAddressV1 {
            kind: AcipAddressKindV1::Agent,
            id: "reviewer.agent".to_string(),
        },
        intent: AcipIntentV1::ReviewRequest,
        purpose: "Perform a bounded review and emit the declared report artifacts.".to_string(),
        input_refs: vec![
            "runtime/comms/invocation/review_packet.json".to_string(),
            "runtime/comms/invocation/trace_anchor.json".to_string(),
        ],
        constraints: AcipInvocationConstraintsV1 {
            policy_refs: vec!["runtime/comms/policy/review_policy.json".to_string()],
            required_capabilities: vec!["review".to_string(), "share_artifact".to_string()],
            prohibited_actions: vec!["merge".to_string(), "destructive_edit".to_string()],
            requires_redaction: true,
        },
        expected_outputs: vec![
            AcipExpectedOutputV1 {
                output_id: "review_report".to_string(),
                output_kind: "review_report".to_string(),
                artifact_role: "primary_report".to_string(),
                schema_ref: Some("schemas/review_report.schema.json".to_string()),
                required: true,
            },
            AcipExpectedOutputV1 {
                output_id: "review_summary".to_string(),
                output_kind: "review_summary".to_string(),
                artifact_role: "operator_summary".to_string(),
                schema_ref: None,
                required: false,
            },
        ],
        stop_policy: AcipInvocationStopPolicyV1 {
            max_turns: 1,
            max_output_artifacts: 2,
            completion_condition: "emit refusal or satisfy required outputs".to_string(),
            stop_on_refusal: true,
            stop_on_failure: true,
        },
        authority_scope: AcipAuthorityScopeV1 {
            allowed_actions: vec!["review".to_string(), "share_artifact".to_string()],
            authority_basis_refs: vec!["runtime/comms/authority/review_basis.json".to_string()],
            delegation_permitted: false,
        },
        decision_event_ref: "gate.review-0001".to_string(),
        response_channel: AcipResponseChannelV1 {
            kind: AcipResponseChannelKindV1::DirectReply,
            channel_ref: "reply.review.thread".to_string(),
        },
        trace_requirement: AcipTraceRequirementV1::Full,
    }
}

fn sample_invocation_event(
    contract: &AcipInvocationContractV1,
    status: AcipInvocationStatusV1,
    output_refs: Vec<String>,
    stop_reason: String,
    refusal_code: Option<String>,
    failure_code: Option<String>,
    evidence_refs: Vec<String>,
) -> AcipInvocationEventV1 {
    AcipInvocationEventV1 {
        schema_version: ACIP_INVOCATION_EVENT_SCHEMA_VERSION.to_string(),
        invocation_id: contract.invocation_id.clone(),
        conversation_id: contract.conversation_id.clone(),
        causal_message_id: contract.causal_message_id.clone(),
        caller: contract.caller.clone(),
        target: contract.target.clone(),
        contract_ref: Some("runtime/comms/invocation/contracts/review_request.json".to_string()),
        contract_sha256: Some(
            "89abcdef0123456789abcdef0123456789abcdef0123456789abcdef01234567".to_string(),
        ),
        decision_event_ref: contract.decision_event_ref.clone(),
        input_refs: contract.input_refs.clone(),
        output_refs,
        status,
        stop_reason,
        refusal_code,
        failure_code,
        evidence_refs,
        trace_requirement: contract.trace_requirement.clone(),
    }
}

fn payload_ref(
    payload_kind: &str,
    payload_ref: &str,
    media_type: &str,
    byte_length: u64,
    inline_summary: Option<&str>,
) -> AcipPayloadRefV1 {
    AcipPayloadRefV1 {
        payload_kind: payload_kind.to_string(),
        payload_ref: payload_ref.to_string(),
        media_type: media_type.to_string(),
        content_sha256: "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
            .to_string(),
        byte_length,
        inline_summary: inline_summary.map(str::to_string),
    }
}

fn validate_address(address: &AcipAddressV1, field: &str) -> Result<()> {
    validate_id(&address.id, &format!("{field}.id"))?;
    Ok(())
}

fn validate_payload_ref(payload_ref: &AcipPayloadRefV1) -> Result<()> {
    validate_id(&payload_ref.payload_kind, "payload_kind")?;
    validate_repo_relative_ref(&payload_ref.payload_ref, "payload_ref")?;
    validate_non_empty(&payload_ref.media_type, "media_type")?;
    validate_sha256(&payload_ref.content_sha256, "content_sha256")?;
    if payload_ref.byte_length == 0 {
        return Err(anyhow!("byte_length must be positive"));
    }
    if let Some(summary) = &payload_ref.inline_summary {
        if summary.chars().count() > MAX_INLINE_SUMMARY_CHARS {
            return Err(anyhow!(
                "inline_summary exceeds bounded inline posture of {MAX_INLINE_SUMMARY_CHARS} characters"
            ));
        }
        if payload_ref.byte_length > MAX_INLINE_BYTE_LENGTH {
            return Err(anyhow!(
                "inline_summary may only summarize bounded payloads up to {MAX_INLINE_BYTE_LENGTH} bytes"
            ));
        }
    }
    Ok(())
}

fn validate_attachment_ref(attachment: &AcipAttachmentRefV1) -> Result<()> {
    validate_id(&attachment.name, "attachment.name")?;
    validate_non_empty(&attachment.media_type, "attachment.media_type")?;
    validate_sha256(&attachment.content_sha256, "attachment.content_sha256")?;
    if attachment.byte_length == 0 {
        return Err(anyhow!("attachment.byte_length must be positive"));
    }
    Ok(())
}

fn validate_authority_scope(scope: &AcipAuthorityScopeV1) -> Result<()> {
    if scope.allowed_actions.is_empty() {
        return Err(anyhow!("authority_scope.allowed_actions must not be empty"));
    }
    if scope.authority_basis_refs.is_empty() {
        return Err(anyhow!(
            "authority_scope.authority_basis_refs must not be empty"
        ));
    }
    let supported = [
        "consult",
        "invoke",
        "review",
        "delegate",
        "negotiate",
        "share_artifact",
    ];
    let mut seen = BTreeSet::new();
    for action in &scope.allowed_actions {
        validate_id(action, "authority_scope.allowed_actions")?;
        if !supported.contains(&action.as_str()) {
            return Err(anyhow!(
                "unsupported authority_scope.allowed_actions '{}'",
                action
            ));
        }
        if !seen.insert(action.clone()) {
            return Err(anyhow!(
                "authority_scope.allowed_actions contains duplicate '{}'",
                action
            ));
        }
    }
    for basis_ref in &scope.authority_basis_refs {
        validate_repo_relative_ref(basis_ref, "authority_scope.authority_basis_refs[]")?;
    }
    Ok(())
}

fn validate_invocation_constraints(constraints: &AcipInvocationConstraintsV1) -> Result<()> {
    if constraints.policy_refs.is_empty() {
        return Err(anyhow!("constraints.policy_refs must not be empty"));
    }
    if constraints.policy_refs.len() > MAX_LIST_LEN {
        return Err(anyhow!(
            "constraints.policy_refs exceeds bounded list length"
        ));
    }
    for policy_ref in &constraints.policy_refs {
        validate_repo_relative_ref(policy_ref, "constraints.policy_refs[]")?;
    }
    if constraints.required_capabilities.is_empty() {
        return Err(anyhow!(
            "constraints.required_capabilities must not be empty"
        ));
    }
    if constraints.required_capabilities.len() > MAX_LIST_LEN {
        return Err(anyhow!(
            "constraints.required_capabilities exceeds bounded list length"
        ));
    }
    let mut seen_capabilities = BTreeSet::new();
    for capability in &constraints.required_capabilities {
        validate_id(capability, "constraints.required_capabilities[]")?;
        if !seen_capabilities.insert(capability.clone()) {
            return Err(anyhow!(
                "constraints.required_capabilities contains duplicate '{}'",
                capability
            ));
        }
    }
    let mut seen_prohibited = BTreeSet::new();
    for action in &constraints.prohibited_actions {
        validate_id(action, "constraints.prohibited_actions[]")?;
        if !seen_prohibited.insert(action.clone()) {
            return Err(anyhow!(
                "constraints.prohibited_actions contains duplicate '{}'",
                action
            ));
        }
    }
    Ok(())
}

fn validate_invocation_authority_alignment(contract: &AcipInvocationContractV1) -> Result<()> {
    let allows = |action: &str| {
        contract
            .authority_scope
            .allowed_actions
            .iter()
            .any(|allowed| allowed == action)
    };

    match contract.intent {
        AcipIntentV1::InvocationSetup | AcipIntentV1::CodingRequest => {
            if !allows("invoke") {
                return Err(anyhow!(
                    "invocation contract intent '{}' requires authority_scope.allowed_actions to include 'invoke'",
                    contract.intent.as_str()
                ));
            }
        }
        AcipIntentV1::ReviewRequest => {
            if !allows("review") {
                return Err(anyhow!(
                    "invocation contract intent 'review_request' requires authority_scope.allowed_actions to include 'review'"
                ));
            }
        }
        AcipIntentV1::Delegation => {
            if !allows("delegate") {
                return Err(anyhow!(
                    "invocation contract intent 'delegation' requires authority_scope.allowed_actions to include 'delegate'"
                ));
            }
            if !contract.authority_scope.delegation_permitted {
                return Err(anyhow!(
                    "invocation contract intent 'delegation' requires authority_scope.delegation_permitted"
                ));
            }
        }
        AcipIntentV1::Conversation | AcipIntentV1::Consultation | AcipIntentV1::Negotiation => {
            unreachable!("validated above")
        }
    }

    Ok(())
}

fn validate_expected_outputs(outputs: &[AcipExpectedOutputV1]) -> Result<()> {
    let mut seen = BTreeSet::new();
    for output in outputs {
        validate_id(&output.output_id, "expected_outputs[].output_id")?;
        validate_id(&output.output_kind, "expected_outputs[].output_kind")?;
        validate_id(&output.artifact_role, "expected_outputs[].artifact_role")?;
        if let Some(schema_ref) = &output.schema_ref {
            validate_repo_relative_ref(schema_ref, "expected_outputs[].schema_ref")?;
        }
        if !seen.insert(output.output_id.clone()) {
            return Err(anyhow!(
                "expected_outputs contains duplicate output_id '{}'",
                output.output_id
            ));
        }
    }
    Ok(())
}

fn validate_stop_policy(stop_policy: &AcipInvocationStopPolicyV1) -> Result<()> {
    if stop_policy.max_turns == 0 {
        return Err(anyhow!("stop_policy.max_turns must be positive"));
    }
    if stop_policy.max_output_artifacts == 0 {
        return Err(anyhow!("stop_policy.max_output_artifacts must be positive"));
    }
    validate_non_empty(
        &stop_policy.completion_condition,
        "stop_policy.completion_condition",
    )?;
    Ok(())
}

fn validate_response_channel(channel: &AcipResponseChannelV1) -> Result<()> {
    match channel.kind {
        AcipResponseChannelKindV1::DirectReply => {
            validate_id(&channel.channel_ref, "response_channel.channel_ref")
        }
        AcipResponseChannelKindV1::ArtifactReply => {
            validate_repo_relative_ref(&channel.channel_ref, "response_channel.channel_ref")
        }
    }
}

fn validate_gate_decision_ref(value: &str, field: &str) -> Result<()> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(anyhow!("{field} must not be empty"));
    }
    let Some(suffix) = trimmed
        .strip_prefix("gate.")
        .or_else(|| trimmed.strip_prefix("gate:"))
    else {
        return Err(anyhow!(
            "{field} must link to a Freedom Gate decision token"
        ));
    };
    if suffix.is_empty() {
        return Err(anyhow!(
            "{field} must include a Freedom Gate decision identifier"
        ));
    }
    if !suffix
        .chars()
        .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || matches!(ch, '-' | '_' | '.'))
    {
        return Err(anyhow!(
            "{field} must use a stable Freedom Gate decision token"
        ));
    }
    Ok(())
}

impl AcipIntentV1 {
    fn as_str(&self) -> &'static str {
        match self {
            AcipIntentV1::Conversation => "conversation",
            AcipIntentV1::Consultation => "consultation",
            AcipIntentV1::InvocationSetup => "invocation_setup",
            AcipIntentV1::ReviewRequest => "review_request",
            AcipIntentV1::CodingRequest => "coding_request",
            AcipIntentV1::Delegation => "delegation",
            AcipIntentV1::Negotiation => "negotiation",
        }
    }
}

fn validate_invocation_status_consistency(event: &AcipInvocationEventV1) -> Result<()> {
    match event.status {
        AcipInvocationStatusV1::Requested => {
            if !event.output_refs.is_empty() {
                return Err(anyhow!(
                    "requested invocation event must not carry output_refs"
                ));
            }
            if event.refusal_code.is_some() || event.failure_code.is_some() {
                return Err(anyhow!(
                    "requested invocation event must not carry refusal_code or failure_code"
                ));
            }
        }
        AcipInvocationStatusV1::Completed => {
            if event.output_refs.is_empty() {
                return Err(anyhow!("completed invocation event must carry output_refs"));
            }
            if event.refusal_code.is_some() || event.failure_code.is_some() {
                return Err(anyhow!(
                    "completed invocation event must not carry refusal_code or failure_code"
                ));
            }
        }
        AcipInvocationStatusV1::Refused => {
            if !event.output_refs.is_empty() {
                return Err(anyhow!(
                    "refused invocation event must not carry output_refs"
                ));
            }
            if event.refusal_code.is_none() {
                return Err(anyhow!("refused invocation event must carry refusal_code"));
            }
            if event.failure_code.is_some() {
                return Err(anyhow!(
                    "refused invocation event must not carry failure_code"
                ));
            }
            if event.evidence_refs.is_empty() {
                return Err(anyhow!("refused invocation event must carry evidence_refs"));
            }
        }
        AcipInvocationStatusV1::Failed => {
            if !event.output_refs.is_empty() {
                return Err(anyhow!(
                    "failed invocation event must not carry output_refs"
                ));
            }
            if event.failure_code.is_none() {
                return Err(anyhow!("failed invocation event must carry failure_code"));
            }
            if event.refusal_code.is_some() {
                return Err(anyhow!(
                    "failed invocation event must not carry refusal_code"
                ));
            }
            if event.evidence_refs.is_empty() {
                return Err(anyhow!("failed invocation event must carry evidence_refs"));
            }
        }
        AcipInvocationStatusV1::Partial => {
            if event.output_refs.is_empty() {
                return Err(anyhow!("partial invocation event must carry output_refs"));
            }
            if event.refusal_code.is_some() || event.failure_code.is_some() {
                return Err(anyhow!(
                    "partial invocation event must not carry refusal_code or failure_code"
                ));
            }
        }
    }
    Ok(())
}

fn validate_negative_case(value: JsonValue, expected: &str, is_message: bool) -> Result<()> {
    validate_non_empty(expected, "expected_error_substring")?;
    let result = if is_message {
        validate_acip_message_envelope_v1_value(&value).map(|_| ())
    } else {
        validate_acip_conversation_envelope_v1_value(&value).map(|_| ())
    };
    match result {
        Ok(()) => Err(anyhow!("negative case unexpectedly validated")),
        Err(error) => {
            let text = format!("{error:#}");
            if !text.contains(expected) {
                return Err(anyhow!(
                    "negative case expected error containing '{}', found '{}'",
                    expected,
                    text
                ));
            }
            Ok(())
        }
    }
}

fn validate_negative_result(result: Result<()>, expected: &str) -> Result<()> {
    match result {
        Ok(()) => Err(anyhow!("negative case unexpectedly validated")),
        Err(error) => {
            let text = format!("{error:#}");
            if !text.contains(expected) {
                return Err(anyhow!(
                    "negative case expected error containing '{}', found '{}'",
                    expected,
                    text
                ));
            }
            Ok(())
        }
    }
}

fn validate_negative_case_name(value: &str, field: &str) -> Result<()> {
    validate_id(value, field)
}

fn validate_id(value: &str, field: &str) -> Result<()> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(anyhow!("{field} must not be empty"));
    }
    if trimmed.starts_with('/')
        || trimmed.starts_with('\\')
        || trimmed.contains('\\')
        || trimmed.contains(':')
        || trimmed.contains("..")
    {
        return Err(anyhow!("{field} must be a stable identifier"));
    }
    Ok(())
}

fn validate_non_empty(value: &str, field: &str) -> Result<()> {
    if value.trim().is_empty() {
        return Err(anyhow!("{field} must not be empty"));
    }
    Ok(())
}

fn validate_timestamp(value: &str, field: &str) -> Result<()> {
    validate_non_empty(value, field)?;
    let parsed = DateTime::parse_from_rfc3339(value)
        .with_context(|| format!("{field} must be an RFC3339-style UTC timestamp"))?;
    if parsed.offset().local_minus_utc() != 0 {
        return Err(anyhow!("{field} must be a UTC timestamp ending in Z"));
    }
    Ok(())
}

fn validate_sha256(value: &str, field: &str) -> Result<()> {
    if value.len() != 64 || !value.chars().all(|ch| ch.is_ascii_hexdigit()) {
        return Err(anyhow!("{field} must be a 64-character hexadecimal sha256"));
    }
    Ok(())
}

fn validate_repo_relative_ref(value: &str, field: &str) -> Result<()> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(anyhow!("{field} must not be empty"));
    }
    if trimmed.starts_with('/')
        || trimmed.starts_with('\\')
        || trimmed.contains('\\')
        || trimmed.contains(':')
    {
        return Err(anyhow!("{field} must be a repository-relative path"));
    }
    for component in Path::new(trimmed).components() {
        match component {
            Component::Normal(_) | Component::CurDir => {}
            _ => {
                return Err(anyhow!(
                    "{field} must be a repository-relative path without traversal"
                ))
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn acip_fixture_set_v1_contract_is_stable() {
        let schema = acip_message_envelope_v1_schema_json().expect("schema");
        assert!(schema.contains("\"message_id\""));
        assert!(schema.contains("\"conversation_id\""));
        assert!(schema.contains("\"authority_scope\""));
    }

    #[test]
    fn acip_fixture_set_v1_matches_required_modes_and_negative_cases() {
        let fixtures = acip_fixture_set_v1();
        validate_acip_fixture_set_v1(&fixtures).expect("fixtures should validate");

        let valid_names: Vec<_> = fixtures
            .valid_messages
            .iter()
            .map(|fixture| fixture.name.as_str())
            .collect();
        assert_eq!(
            valid_names,
            vec![
                "conversation",
                "consultation",
                "invocation_setup",
                "review_request",
                "coding_request",
                "delegation",
                "negotiation"
            ]
        );

        let invalid_names: Vec<_> = fixtures
            .invalid_messages
            .iter()
            .map(|case| case.name.as_str())
            .collect();
        assert_eq!(
            invalid_names,
            vec![
                "missing_identity",
                "malformed_refs",
                "unsafe_visibility",
                "overlarge_inline_payload_posture",
                "unsupported_authority_assertion"
            ]
        );
    }

    #[test]
    fn acip_conversation_envelope_rejects_stale_ordering() {
        let mut conversation = acip_fixture_set_v1().valid_conversation;
        conversation.messages.swap(0, 1);
        let error = validate_acip_conversation_envelope_v1(&conversation)
            .expect_err("ordering should fail");
        assert!(error
            .to_string()
            .contains("strictly increasing monotonic_order"));
    }

    #[test]
    fn acip_message_envelope_rejects_host_path_leakage() {
        let mut message = acip_fixture_set_v1().valid_messages[0].message.clone();
        message.artifact_refs = vec!["/Users/daniel/private/trace.json".to_string()];
        let error =
            validate_acip_message_envelope_v1(&message).expect_err("absolute path should fail");
        assert!(error
            .to_string()
            .contains("artifact_refs[] must be a repository-relative path"));
    }

    #[test]
    fn acip_fixture_schema_json_is_available() {
        let schema = acip_fixture_set_v1_schema_json().expect("fixture schema");
        assert!(schema.contains("\"valid_messages\""));
        assert!(schema.contains("\"invalid_conversations\""));
    }

    #[test]
    fn acip_invocation_contract_and_event_schemas_are_available() {
        let contract_schema = acip_invocation_contract_v1_schema_json().expect("contract schema");
        let event_schema = acip_invocation_event_v1_schema_json().expect("event schema");
        assert!(contract_schema.contains("\"decision_event_ref\""));
        assert!(event_schema.contains("\"stop_reason\""));
    }

    #[test]
    fn acip_invocation_fixture_set_matches_required_contract_and_negative_cases() {
        let fixtures = acip_invocation_fixture_set_v1();
        validate_acip_invocation_fixture_set_v1(&fixtures)
            .expect("invocation fixtures should validate");
        assert_eq!(fixtures.negative_cases.len(), 5);
    }

    #[test]
    fn acip_invocation_requires_gate_linkage_for_governed_work() {
        let mut value = serde_json::to_value(sample_invocation_contract()).expect("contract json");
        value
            .as_object_mut()
            .expect("object")
            .remove("decision_event_ref");
        let error = validate_acip_invocation_contract_v1_value(&value)
            .expect_err("missing gate decision should fail");
        assert!(format!("{error:#}").contains("missing field `decision_event_ref`"));
    }

    #[test]
    fn acip_invocation_event_rejects_output_contract_mismatch() {
        let contract = sample_invocation_contract();
        let event = sample_invocation_event(
            &contract,
            AcipInvocationStatusV1::Completed,
            vec!["runtime/comms/invocation/operator_summary.json".to_string()],
            "completed_output_contract".to_string(),
            None,
            None,
            vec!["runtime/comms/invocation/evidence/review_trace.json".to_string()],
        );
        let error = validate_acip_invocation_event_against_contract(&contract, &event)
            .expect_err("missing outputs should fail");
        assert!(error
            .to_string()
            .contains("completed invocation must satisfy declared required output contracts"));
    }

    #[test]
    fn acip_invocation_event_preserves_contract_input_and_trace_binding() {
        let contract = sample_invocation_contract();
        let mut event = sample_invocation_event(
            &contract,
            AcipInvocationStatusV1::Completed,
            vec!["runtime/comms/invocation/review_report.json".to_string()],
            "completed_output_contract".to_string(),
            None,
            None,
            vec!["runtime/comms/invocation/evidence/review_trace.json".to_string()],
        );
        event.input_refs = vec!["runtime/comms/invocation/drifted_input.json".to_string()];
        let input_error = validate_acip_invocation_event_against_contract(&contract, &event)
            .expect_err("drifted input refs should fail");
        assert!(input_error
            .to_string()
            .contains("invocation event must preserve the contract input_refs"));

        let mut trace_event = sample_invocation_event(
            &contract,
            AcipInvocationStatusV1::Completed,
            vec!["runtime/comms/invocation/review_report.json".to_string()],
            "completed_output_contract".to_string(),
            None,
            None,
            vec!["runtime/comms/invocation/evidence/review_trace.json".to_string()],
        );
        trace_event.trace_requirement = AcipTraceRequirementV1::Summary;
        let trace_error = validate_acip_invocation_event_against_contract(&contract, &trace_event)
            .expect_err("trace drift should fail");
        assert!(trace_error
            .to_string()
            .contains("invocation event must preserve the contract trace_requirement"));
    }

    #[test]
    fn acip_invocation_event_respects_max_output_artifact_bound() {
        let contract = sample_invocation_contract();
        let event = sample_invocation_event(
            &contract,
            AcipInvocationStatusV1::Completed,
            vec![
                "runtime/comms/invocation/review_report.json".to_string(),
                "runtime/comms/invocation/review_summary.json".to_string(),
                "runtime/comms/invocation/overflow.json".to_string(),
            ],
            "completed_output_contract".to_string(),
            None,
            None,
            vec!["runtime/comms/invocation/evidence/review_trace.json".to_string()],
        );
        let error = validate_acip_invocation_event_against_contract(&contract, &event)
            .expect_err("too many outputs should fail");
        assert!(error
            .to_string()
            .contains("invocation event exceeds stop_policy.max_output_artifacts"));
    }

    #[test]
    fn acip_invocation_refusal_requires_evidence_and_no_outputs() {
        let contract = sample_invocation_contract();
        let event = sample_invocation_event(
            &contract,
            AcipInvocationStatusV1::Refused,
            vec!["runtime/comms/invocation/review_report.json".to_string()],
            "policy_refusal".to_string(),
            Some("operator_review_required".to_string()),
            None,
            Vec::new(),
        );
        let error = validate_acip_invocation_event_v1(&event)
            .expect_err("refused invocation with outputs and no evidence should fail");
        assert!(error
            .to_string()
            .contains("refused invocation event must not carry output_refs"));
    }

    #[test]
    fn acip_invocation_contract_and_event_reject_unknown_fields() {
        let mut contract_value =
            serde_json::to_value(sample_invocation_contract()).expect("contract json");
        contract_value["unexpected_field"] = json!("surprise");
        let contract_error = validate_acip_invocation_contract_v1_value(&contract_value)
            .expect_err("unknown contract field should fail");
        assert!(format!("{contract_error:#}").contains("unknown field"));

        let contract = sample_invocation_contract();
        let mut event_value = serde_json::to_value(sample_invocation_event(
            &contract,
            AcipInvocationStatusV1::Completed,
            vec!["runtime/comms/invocation/review_report.json".to_string()],
            "completed_output_contract".to_string(),
            None,
            None,
            vec!["runtime/comms/invocation/evidence/review_trace.json".to_string()],
        ))
        .expect("event json");
        event_value["unexpected_field"] = json!("surprise");
        let event_error = validate_acip_invocation_event_v1_value(&event_value)
            .expect_err("unknown event field should fail");
        assert!(format!("{event_error:#}").contains("unknown field"));
    }

    #[test]
    fn acip_invocation_contract_requires_authority_scope_alignment() {
        let mut delegation_contract = sample_invocation_contract();
        delegation_contract.intent = AcipIntentV1::Delegation;
        delegation_contract.authority_scope.allowed_actions = vec!["review".to_string()];
        delegation_contract.authority_scope.delegation_permitted = false;
        let delegation_error = validate_acip_invocation_contract_v1(&delegation_contract)
            .expect_err("delegation intent without delegation authority should fail");
        assert!(delegation_error
            .to_string()
            .contains("requires authority_scope.allowed_actions to include 'delegate'"));

        let mut coding_contract = sample_invocation_contract();
        coding_contract.intent = AcipIntentV1::CodingRequest;
        coding_contract.authority_scope.allowed_actions = vec!["review".to_string()];
        let coding_error = validate_acip_invocation_contract_v1(&coding_contract)
            .expect_err("coding intent without invoke authority should fail");
        assert!(coding_error
            .to_string()
            .contains("requires authority_scope.allowed_actions to include 'invoke'"));
    }

    #[test]
    fn acip_message_envelope_rejects_unknown_fields() {
        let mut value =
            serde_json::to_value(acip_fixture_set_v1().valid_messages[0].message.clone())
                .expect("json");
        value["unexpected_field"] = json!("surprise");
        let error =
            validate_acip_message_envelope_v1_value(&value).expect_err("unknown field should fail");
        assert!(format!("{error:#}").contains("unknown field"));
    }

    #[test]
    fn acip_message_envelope_rejects_invalid_timestamp_shape() {
        let mut message = acip_fixture_set_v1().valid_messages[0].message.clone();
        message.timestamp_utc = "not-a-timeTstill-badZ".to_string();
        let error = validate_acip_message_envelope_v1(&message).expect_err("timestamp should fail");
        assert!(error.to_string().contains("RFC3339-style UTC timestamp"));
    }
}
