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
        validate_message_intent_authority_alignment(&envelope.intent, scope)?;
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
    let mut coding_handoff = sample_message_with_payloads(
        "msg-handoff-0001",
        "conv-handoff-001",
        1,
        AcipIntentV1::CodingRequest,
        "Handing this bounded coding task to the implementation lane with explicit artifact return expectations.",
        vec![payload_ref(
            "task_bundle",
            "runtime/comms/coding/handoff_task_bundle.json",
            "application/json",
            1_732,
            Some("Coding handoff bundle plus expected patch outputs."),
        )],
    );
    coding_handoff.recipient.id = "coder.agent".to_string();
    coding_handoff.correlation_id = Some("handoff-0001".to_string());

    let mut operator_request = sample_message_with_payloads(
        "msg-operator-0001",
        "conv-operator-001",
        1,
        AcipIntentV1::Consultation,
        "Operators, please review the bounded demo packet and confirm it is safe to publish internally.",
        vec![payload_ref(
            "demo_packet",
            "runtime/comms/operator/demo_packet.json",
            "application/json",
            1_204,
            Some("Demo packet for operator review without governed invocation."),
        )],
    );
    operator_request.recipient = AcipAddressV1 {
        kind: AcipAddressKindV1::Group,
        id: "operators".to_string(),
    };
    operator_request.visibility = AcipVisibilityV1::Shared;
    operator_request.trace_requirement = AcipTraceRequirementV1::Full;

    let mut broadcast = sample_message(
        "msg-broadcast-0001",
        "conv-broadcast-001",
        1,
        AcipIntentV1::Conversation,
        "Broadcast: the conformance suite is green and no governed action is requested in this message.",
    );
    broadcast.recipient = AcipAddressV1 {
        kind: AcipAddressKindV1::Group,
        id: "all_agents".to_string(),
    };
    broadcast.visibility = AcipVisibilityV1::Public;
    broadcast.trace_requirement = AcipTraceRequirementV1::Summary;
    broadcast.authority_scope = None;

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
            AcipNamedMessageFixtureV1 {
                name: "coding_agent_handoff".to_string(),
                message: coding_handoff,
            },
            AcipNamedMessageFixtureV1 {
                name: "operator_request".to_string(),
                message: operator_request,
            },
            AcipNamedMessageFixtureV1 {
                name: "broadcast".to_string(),
                message: broadcast,
            },
        ],
        valid_conversation: conversation,
        invalid_messages: vec![
            AcipNegativeMessageCaseV1 {
                name: "identity_drift".to_string(),
                expected_error_substring: "sender and recipient must not be identical".to_string(),
                value: json!({
                    "schema_version": ACIP_MESSAGE_SCHEMA_VERSION,
                    "message_id": "msg-bad-0001",
                    "conversation_id": "conv-bad-001",
                    "timestamp_utc": "2026-04-28T19:00:00Z",
                    "monotonic_order": 1,
                    "sender": {"kind": "agent", "id": "planner.agent"},
                    "recipient": {"kind": "agent", "id": "planner.agent"},
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
                name: "missing_recipient".to_string(),
                expected_error_substring: "recipient.id must not be empty".to_string(),
                value: json!({
                    "schema_version": ACIP_MESSAGE_SCHEMA_VERSION,
                    "message_id": "msg-bad-0002",
                    "conversation_id": "conv-bad-002",
                    "timestamp_utc": "2026-04-28T19:01:00Z",
                    "monotonic_order": 1,
                    "sender": {"kind": "agent", "id": "planner.agent"},
                    "recipient": {"kind": "agent", "id": ""},
                    "intent": "conversation",
                    "visibility": "private",
                    "trace_requirement": "summary",
                    "content": "Missing recipient should fail closed.",
                    "payload_refs": [],
                    "artifact_refs": [],
                    "attachments": [],
                    "authority_scope": null,
                    "correlation_id": null,
                    "prior_message_id": null
                }),
            },
            AcipNegativeMessageCaseV1 {
                name: "hidden_invocation".to_string(),
                expected_error_substring:
                    "message intent 'conversation' must not claim authority action 'invoke'"
                        .to_string(),
                value: json!({
                    "schema_version": ACIP_MESSAGE_SCHEMA_VERSION,
                    "message_id": "msg-bad-0003",
                    "conversation_id": "conv-bad-003",
                    "timestamp_utc": "2026-04-28T19:02:00Z",
                    "monotonic_order": 1,
                    "sender": {"kind": "agent", "id": "planner.agent"},
                    "recipient": {"kind": "agent", "id": "coder.agent"},
                    "intent": "conversation",
                    "visibility": "private",
                    "trace_requirement": "summary",
                    "content": "This looks conversational but is really trying to sneak governed work through.",
                    "payload_refs": [{
                        "payload_kind": "task_bundle",
                        "payload_ref": "runtime/comms/coding/task_bundle.json",
                        "media_type": "application/json",
                        "content_sha256": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
                        "byte_length": 612,
                        "inline_summary": "Hidden invocation attempt."
                    }],
                    "artifact_refs": [],
                    "attachments": [],
                    "authority_scope": {
                        "allowed_actions": ["invoke", "share_artifact"],
                        "authority_basis_refs": ["runtime/comms/authority/invocation_basis.json"],
                        "delegation_permitted": false
                    },
                    "correlation_id": null,
                    "prior_message_id": null
                }),
            },
            AcipNegativeMessageCaseV1 {
                name: "malformed_payload_refs".to_string(),
                expected_error_substring:
                    "content_sha256 must be a 64-character hexadecimal sha256".to_string(),
                value: json!({
                    "schema_version": ACIP_MESSAGE_SCHEMA_VERSION,
                    "message_id": "msg-bad-0004",
                    "conversation_id": "conv-bad-004",
                    "timestamp_utc": "2026-04-28T19:03:00Z",
                    "monotonic_order": 1,
                    "sender": {"kind": "agent", "id": "planner.agent"},
                    "recipient": {"kind": "agent", "id": "reviewer.agent"},
                    "intent": "review_request",
                    "visibility": "shared",
                    "trace_requirement": "summary",
                    "content": "Please review this packet.",
                    "payload_refs": [{
                        "payload_kind": "review_packet",
                        "payload_ref": "runtime/comms/review/review_packet.json",
                        "media_type": "application/json",
                        "content_sha256": "not-a-sha",
                        "byte_length": 512,
                        "inline_summary": "Malformed payload hash."
                    }],
                    "artifact_refs": [],
                    "attachments": [],
                    "authority_scope": {
                        "allowed_actions": ["review", "share_artifact"],
                        "authority_basis_refs": ["runtime/comms/authority/review_basis.json"],
                        "delegation_permitted": false
                    },
                    "correlation_id": null,
                    "prior_message_id": null
                }),
            },
            AcipNegativeMessageCaseV1 {
                name: "unsupported_visibility".to_string(),
                expected_error_substring: "unknown variant".to_string(),
                value: json!({
                    "schema_version": ACIP_MESSAGE_SCHEMA_VERSION,
                    "message_id": "msg-bad-0005",
                    "conversation_id": "conv-bad-005",
                    "timestamp_utc": "2026-04-28T19:04:00Z",
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
                name: "raw_local_path_refs".to_string(),
                expected_error_substring: "artifact_refs[] must be a repository-relative path"
                    .to_string(),
                value: json!({
                    "schema_version": ACIP_MESSAGE_SCHEMA_VERSION,
                    "message_id": "msg-bad-0006",
                    "conversation_id": "conv-bad-006",
                    "timestamp_utc": "2026-04-28T19:05:00Z",
                    "monotonic_order": 1,
                    "sender": {"kind": "agent", "id": "planner.agent"},
                    "recipient": {"kind": "agent", "id": "coder.agent"},
                    "intent": "coding_request",
                    "visibility": "private",
                    "trace_requirement": "summary",
                    "content": "Please implement the change.",
                    "payload_refs": [],
                    "artifact_refs": ["/Users/daniel/private/runtime_patch.diff"],
                    "attachments": [],
                    "authority_scope": {
                        "allowed_actions": ["invoke", "share_artifact"],
                        "authority_basis_refs": ["runtime/comms/authority/invocation_basis.json"],
                        "delegation_permitted": false
                    },
                    "correlation_id": null,
                    "prior_message_id": null
                }),
            },
            AcipNegativeMessageCaseV1 {
                name: "authority_escalation".to_string(),
                expected_error_substring:
                    "message intent 'consultation' must not claim authority action 'delegate'"
                        .to_string(),
                value: json!({
                    "schema_version": ACIP_MESSAGE_SCHEMA_VERSION,
                    "message_id": "msg-bad-0007",
                    "conversation_id": "conv-bad-007",
                    "timestamp_utc": "2026-04-28T19:06:00Z",
                    "monotonic_order": 1,
                    "sender": {"kind": "agent", "id": "planner.agent"},
                    "recipient": {"kind": "agent", "id": "delegate.agent"},
                    "intent": "consultation",
                    "visibility": "shared",
                    "trace_requirement": "full",
                    "content": "This tries to smuggle delegated authority through a consultation surface.",
                    "payload_refs": [],
                    "artifact_refs": ["runtime/comms/delegation/contract.json"],
                    "attachments": [],
                    "authority_scope": {
                        "allowed_actions": ["delegate", "share_artifact"],
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
    let authority_scope = default_message_authority_scope(&intent);
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
        authority_scope: Some(authority_scope),
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
    message.authority_scope = Some(default_message_authority_scope(&message.intent));
    message
}

fn default_message_authority_scope(intent: &AcipIntentV1) -> AcipAuthorityScopeV1 {
    match intent {
        AcipIntentV1::Conversation => AcipAuthorityScopeV1 {
            allowed_actions: vec!["consult".to_string(), "share_artifact".to_string()],
            authority_basis_refs: vec![
                "runtime/comms/authority/conversation_basis.json".to_string()
            ],
            delegation_permitted: false,
        },
        AcipIntentV1::Consultation => AcipAuthorityScopeV1 {
            allowed_actions: vec!["consult".to_string(), "share_artifact".to_string()],
            authority_basis_refs: vec![
                "runtime/comms/authority/consultation_basis.json".to_string()
            ],
            delegation_permitted: false,
        },
        AcipIntentV1::InvocationSetup | AcipIntentV1::CodingRequest => AcipAuthorityScopeV1 {
            allowed_actions: vec!["invoke".to_string(), "share_artifact".to_string()],
            authority_basis_refs: vec!["runtime/comms/authority/invocation_basis.json".to_string()],
            delegation_permitted: false,
        },
        AcipIntentV1::ReviewRequest => AcipAuthorityScopeV1 {
            allowed_actions: vec!["review".to_string(), "share_artifact".to_string()],
            authority_basis_refs: vec!["runtime/comms/authority/review_basis.json".to_string()],
            delegation_permitted: false,
        },
        AcipIntentV1::Delegation => AcipAuthorityScopeV1 {
            allowed_actions: vec!["delegate".to_string(), "share_artifact".to_string()],
            authority_basis_refs: vec!["runtime/comms/authority/delegation_basis.json".to_string()],
            delegation_permitted: true,
        },
        AcipIntentV1::Negotiation => AcipAuthorityScopeV1 {
            allowed_actions: vec!["negotiate".to_string(), "share_artifact".to_string()],
            authority_basis_refs: vec!["runtime/comms/authority/negotiation_basis.json".to_string()],
            delegation_permitted: false,
        },
    }
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
