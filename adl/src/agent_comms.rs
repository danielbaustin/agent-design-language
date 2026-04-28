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

pub fn acip_message_envelope_v1_schema_json() -> Result<String> {
    serde_json::to_string_pretty(&schema_for!(AcipMessageEnvelopeV1))
        .context("serialize ACIP message envelope v1 schema")
}

pub fn acip_fixture_set_v1_schema_json() -> Result<String> {
    serde_json::to_string_pretty(&schema_for!(AcipFixtureSetV1))
        .context("serialize ACIP fixture set v1 schema")
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
