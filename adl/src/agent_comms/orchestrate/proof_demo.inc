pub fn acip_proof_demo_packet_v1_schema_json() -> Result<String> {
    serde_json::to_string_pretty(&schema_for!(AcipProofDemoPacketV1))
        .context("serialize ACIP proof demo packet v1 schema")
}

pub fn validate_acip_proof_demo_packet_v1_value(
    value: &JsonValue,
) -> Result<AcipProofDemoPacketV1> {
    let packet: AcipProofDemoPacketV1 =
        serde_json::from_value(value.clone()).context("parse ACIP proof demo packet v1")?;
    validate_acip_proof_demo_packet_v1(&packet)?;
    Ok(packet)
}

pub fn acip_proof_demo_packet_v1() -> AcipProofDemoPacketV1 {
    let conversation = sample_acip_proof_conversation();
    let coding_invocation = sample_acip_proof_coding_contract();
    let coding_outcome = sample_acip_proof_coding_outcome(&coding_invocation);
    let trace_bundle = sample_acip_proof_trace_bundle(&coding_invocation, &coding_outcome);

    AcipProofDemoPacketV1 {
        schema_version: ACIP_PROOF_DEMO_SCHEMA_VERSION.to_string(),
        demo_id: "acip-proof-demo".to_string(),
        title: "ACIP proof demo: consultation to governed coding proposal".to_string(),
        proof_classification: AcipProofClassificationV1::Proving,
        represented_modes: vec![
            AcipIntentV1::Consultation,
            AcipIntentV1::Negotiation,
            AcipIntentV1::CodingRequest,
        ],
        proves: vec![
            "ACIP preserves natural consultation before governed work binding.".to_string(),
            "ACIP can carry explicit capability negotiation before a governed coding request is bound."
                .to_string(),
            "ACIP coding specialization turns the final coding_request into explicit governed invocation contract and outcome surfaces.".to_string(),
            "ACIP proposal-ready outcomes remain review handoff evidence rather than self-authorizing execution or merge authority.".to_string(),
            "ACIP trace and audience views preserve reviewer/public redaction boundaries while remaining replayable.".to_string(),
        ],
        conversation,
        coding_invocation,
        coding_outcome,
        trace_bundle,
        reviewer_visible_artifact_refs: vec![
            "runtime/comms/proof/acip_demo_reviewer_trace.json".to_string(),
            "runtime/comms/coding/proposal.json".to_string(),
            "runtime/comms/coding/review_handoff.json".to_string(),
        ],
        public_visible_artifact_refs: vec![
            "runtime/comms/proof/acip_demo_public_summary.json".to_string(),
            "runtime/comms/coding/proposal.json".to_string(),
        ],
        feature_doc_refs: vec![
            "docs/milestones/v0.90.5/features/AGENT_COMMS_v1.md".to_string(),
            "docs/milestones/v0.90.5/features/GOVERNED_EXECUTION_AND_TRACE.md".to_string(),
            "docs/milestones/v0.90.5/features/MODEL_TESTING_AND_FLAGSHIP_DEMO.md".to_string(),
            "docs/milestones/v0.90.5/DEMO_MATRIX_v0.90.5.md".to_string(),
        ],
        validation_commands: vec![
            "cargo fmt --check".to_string(),
            "cargo test agent_comms --lib".to_string(),
        ],
        non_proving_statements: vec![
            "This proof packet does not prove live transport.".to_string(),
            "This proof packet does not prove encrypted external transport.".to_string(),
            "This proof packet does not prove reputation systems.".to_string(),
            "This proof packet does not prove cross-polis federation.".to_string(),
        ],
    }
}

fn sample_acip_proof_conversation() -> AcipConversationEnvelopeV1 {
    let mut consultation = crate::agent_comms::transport::sample_message(
        "msg-acip-proof-0001",
        "conv-acip-proof-001",
        1,
        AcipIntentV1::Consultation,
        "Can you help me compare two implementation paths before I bind a governed coding request?",
    );
    consultation.sender.id = "writer.agent".to_string();
    consultation.recipient.id = "planner.agent".to_string();

    let mut negotiation = crate::agent_comms::transport::sample_message(
        "msg-acip-proof-0002",
        "conv-acip-proof-001",
        2,
        AcipIntentV1::Negotiation,
        "I can offer a provider-optional proposal-only coding lane with explicit validation, trace binding, and review handoff.",
    );
    negotiation.sender.id = "planner.agent".to_string();
    negotiation.recipient.id = "writer.agent".to_string();

    let mut coding_request = crate::agent_comms::transport::sample_message_with_payloads(
        "msg-acip-proof-0003",
        "conv-acip-proof-001",
        3,
        AcipIntentV1::CodingRequest,
        "Please prepare a bounded proposal-only coding response with explicit validation and review handoff.",
        vec![payload_ref(
            "task_bundle",
            "runtime/comms/proof/acip_demo_task_bundle.json",
            "application/json",
            2_048,
            Some("Bounded coding task bundle plus validation and review-handoff expectations."),
        )],
    );
    coding_request.sender.id = "writer.agent".to_string();
    coding_request.recipient.id = "coding.agent".to_string();
    coding_request.trace_requirement = AcipTraceRequirementV1::Full;

    AcipConversationEnvelopeV1 {
        schema_version: ACIP_CONVERSATION_SCHEMA_VERSION.to_string(),
        messages: vec![consultation, negotiation, coding_request],
    }
}

fn sample_acip_proof_coding_contract() -> AcipCodingInvocationContractV1 {
    let mut contract = crate::agent_comms::dispatch::coding::sample_coding_invocation_contract(
        AcipCodingProviderLaneV1::OtherProposalOnly,
        AcipCodingExecutionModeV1::StructuredProposal,
    );
    contract.invocation.conversation_id = "conv-acip-proof-001".to_string();
    contract.invocation.causal_message_id = "msg-acip-proof-0003".to_string();
    contract.invocation.decision_event_ref = "gate.acip-proof-0001".to_string();
    contract.invocation.input_refs = vec![
        "runtime/comms/proof/acip_demo_task_bundle.json".to_string(),
        "runtime/comms/proof/acip_demo_consultation_summary.md".to_string(),
        "runtime/comms/proof/acip_demo_scope.json".to_string(),
    ];
    contract.invocation.constraints.policy_refs = vec![
        "runtime/comms/proof/acip_demo_policy.md".to_string(),
        "runtime/comms/policy/coding_policy.json".to_string(),
    ];
    contract.invocation.authority_scope.authority_basis_refs =
        vec!["runtime/comms/proof/acip_demo_authority_basis.json".to_string()];
    contract.invocation.response_channel = AcipResponseChannelV1 {
        kind: AcipResponseChannelKindV1::ArtifactReply,
        channel_ref: "runtime/comms/proof/acip_demo_coding_outcome.json".to_string(),
    };
    contract.issue_ref = "issues/2634".to_string();
    contract.task_bundle_ref = "runtime/comms/proof/acip_demo_task_bundle.json".to_string();
    contract.allowed_edit_paths = vec![
        "adl/src/agent_comms.rs".to_string(),
        "docs/milestones/v0.90.5/features/AGENT_COMMS_v1.md".to_string(),
        "docs/milestones/v0.90.5/features/MODEL_TESTING_AND_FLAGSHIP_DEMO.md".to_string(),
        "docs/milestones/v0.90.5/DEMO_MATRIX_v0.90.5.md".to_string(),
    ];
    contract.validation_commands = vec![
        "cargo fmt --check".to_string(),
        "cargo test agent_comms --lib".to_string(),
    ];
    contract
}

fn sample_acip_proof_coding_outcome(
    contract: &AcipCodingInvocationContractV1,
) -> AcipCodingOutcomeV1 {
    let mut outcome = crate::agent_comms::dispatch::coding::sample_coding_outcome(contract);
    outcome.primary_output_ref = "runtime/comms/coding/proposal.json".to_string();
    outcome.validation_result_refs =
        vec!["runtime/comms/coding/validation_summary.json".to_string()];
    outcome.review_handoff_ref = "runtime/comms/coding/review_handoff.json".to_string();
    outcome
}

fn sample_acip_proof_trace_bundle(
    contract: &AcipCodingInvocationContractV1,
    outcome: &AcipCodingOutcomeV1,
) -> AcipTraceBundleV1 {
    AcipTraceBundleV1 {
        schema_version: ACIP_TRACE_BUNDLE_SCHEMA_VERSION.to_string(),
        conversation_id: contract.invocation.conversation_id.clone(),
        trace_events: vec![
            AcipTraceEventV1 {
                event_id: "trace-acip-proof-0001".to_string(),
                conversation_id: contract.invocation.conversation_id.clone(),
                invocation_id: None,
                event_kind: AcipTraceEventKindV1::MessageCreated,
                source_message_id: Some(contract.invocation.causal_message_id.clone()),
                contract_ref: None,
                decision_event_ref: None,
                invocation_status: None,
                output_refs: Vec::new(),
                evidence_refs: vec![
                    "runtime/comms/proof/acip_demo_message_anchor.json".to_string()
                ],
                summary: "Coding request message anchored the governed ACIP proof path."
                    .to_string(),
                requires_redaction: false,
            },
            AcipTraceEventV1 {
                event_id: "trace-acip-proof-0002".to_string(),
                conversation_id: contract.invocation.conversation_id.clone(),
                invocation_id: Some(contract.invocation.invocation_id.clone()),
                event_kind: AcipTraceEventKindV1::InvocationContractDeclared,
                source_message_id: Some(contract.invocation.causal_message_id.clone()),
                contract_ref: Some(
                    "runtime/comms/proof/acip_demo_coding_contract.json".to_string(),
                ),
                decision_event_ref: Some(contract.invocation.decision_event_ref.clone()),
                invocation_status: None,
                output_refs: Vec::new(),
                evidence_refs: vec![
                    "runtime/comms/proof/acip_demo_contract_anchor.json".to_string()
                ],
                summary: "Proposal-only coding contract declared explicit validation and review-handoff bounds."
                    .to_string(),
                requires_redaction: false,
            },
            AcipTraceEventV1 {
                event_id: "trace-acip-proof-0003".to_string(),
                conversation_id: contract.invocation.conversation_id.clone(),
                invocation_id: Some(contract.invocation.invocation_id.clone()),
                event_kind: AcipTraceEventKindV1::DecisionRecorded,
                source_message_id: Some(contract.invocation.causal_message_id.clone()),
                contract_ref: Some(
                    "runtime/comms/proof/acip_demo_coding_contract.json".to_string(),
                ),
                decision_event_ref: Some(contract.invocation.decision_event_ref.clone()),
                invocation_status: None,
                output_refs: Vec::new(),
                evidence_refs: vec![
                    "runtime/comms/proof/acip_demo_gate_result.json".to_string()
                ],
                summary: "Freedom Gate recorded bounded proposal-only authorization before coding output."
                    .to_string(),
                requires_redaction: false,
            },
            AcipTraceEventV1 {
                event_id: "trace-acip-proof-0004".to_string(),
                conversation_id: contract.invocation.conversation_id.clone(),
                invocation_id: Some(contract.invocation.invocation_id.clone()),
                event_kind: AcipTraceEventKindV1::InvocationCompleted,
                source_message_id: Some(contract.invocation.causal_message_id.clone()),
                contract_ref: Some(
                    "runtime/comms/proof/acip_demo_coding_contract.json".to_string(),
                ),
                decision_event_ref: Some(contract.invocation.decision_event_ref.clone()),
                invocation_status: Some(AcipInvocationStatusV1::Completed),
                output_refs: vec![
                    outcome.primary_output_ref.clone(),
                    outcome.review_handoff_ref.clone(),
                ],
                evidence_refs: outcome.validation_result_refs.clone(),
                summary: "Coding outcome completed with proposal-ready review evidence and deterministic redaction views."
                    .to_string(),
                requires_redaction: false,
            },
        ],
        audience_views: vec![
            AcipTraceAudienceViewV1 {
                audience: AcipTraceAudienceV1::Actor,
                narrative_ref: "runtime/comms/proof/acip_demo_actor_view.json".to_string(),
                visible_event_ids: vec![
                    "trace-acip-proof-0001".to_string(),
                    "trace-acip-proof-0002".to_string(),
                    "trace-acip-proof-0003".to_string(),
                    "trace-acip-proof-0004".to_string(),
                ],
                visible_artifact_refs: vec![
                    "runtime/comms/proof/acip_demo_actor_view.json".to_string(),
                    outcome.primary_output_ref.clone(),
                    outcome.review_handoff_ref.clone(),
                ],
                redacted_elements: vec!["none".to_string()],
                allows_private_payload_refs: false,
                allows_raw_tool_args: false,
                allows_local_host_paths: false,
                allows_rejected_alternative_details: false,
            },
            AcipTraceAudienceViewV1 {
                audience: AcipTraceAudienceV1::Operator,
                narrative_ref: "runtime/comms/proof/acip_demo_operator_view.json".to_string(),
                visible_event_ids: vec![
                    "trace-acip-proof-0001".to_string(),
                    "trace-acip-proof-0002".to_string(),
                    "trace-acip-proof-0003".to_string(),
                    "trace-acip-proof-0004".to_string(),
                ],
                visible_artifact_refs: vec![
                    "runtime/comms/proof/acip_demo_operator_view.json".to_string(),
                    outcome.primary_output_ref.clone(),
                    "runtime/comms/coding/validation_summary.json".to_string(),
                ],
                redacted_elements: vec!["raw_tool_args".to_string()],
                allows_private_payload_refs: false,
                allows_raw_tool_args: false,
                allows_local_host_paths: false,
                allows_rejected_alternative_details: false,
            },
            AcipTraceAudienceViewV1 {
                audience: AcipTraceAudienceV1::Reviewer,
                narrative_ref: "runtime/comms/proof/acip_demo_reviewer_trace.json".to_string(),
                visible_event_ids: vec![
                    "trace-acip-proof-0002".to_string(),
                    "trace-acip-proof-0003".to_string(),
                    "trace-acip-proof-0004".to_string(),
                ],
                visible_artifact_refs: vec![
                    "runtime/comms/proof/acip_demo_reviewer_trace.json".to_string(),
                    outcome.primary_output_ref.clone(),
                    outcome.review_handoff_ref.clone(),
                ],
                redacted_elements: vec![
                    "private_payload_refs".to_string(),
                    "raw_tool_args".to_string(),
                    "rejected_alternative_details".to_string(),
                ],
                allows_private_payload_refs: false,
                allows_raw_tool_args: false,
                allows_local_host_paths: false,
                allows_rejected_alternative_details: false,
            },
            AcipTraceAudienceViewV1 {
                audience: AcipTraceAudienceV1::Public,
                narrative_ref: "runtime/comms/proof/acip_demo_public_summary.json".to_string(),
                visible_event_ids: vec![
                    "trace-acip-proof-0003".to_string(),
                    "trace-acip-proof-0004".to_string(),
                ],
                visible_artifact_refs: vec![
                    "runtime/comms/proof/acip_demo_public_summary.json".to_string(),
                    outcome.primary_output_ref.clone(),
                ],
                redacted_elements: vec![
                    "private_payload_refs".to_string(),
                    "raw_tool_args".to_string(),
                    "local_host_paths".to_string(),
                    "rejected_alternative_details".to_string(),
                ],
                allows_private_payload_refs: false,
                allows_raw_tool_args: false,
                allows_local_host_paths: false,
                allows_rejected_alternative_details: false,
            },
            AcipTraceAudienceViewV1 {
                audience: AcipTraceAudienceV1::Observatory,
                narrative_ref: "runtime/comms/proof/acip_demo_observatory_view.json".to_string(),
                visible_event_ids: vec![
                    "trace-acip-proof-0001".to_string(),
                    "trace-acip-proof-0003".to_string(),
                    "trace-acip-proof-0004".to_string(),
                ],
                visible_artifact_refs: vec![
                    "runtime/comms/proof/acip_demo_observatory_view.json".to_string(),
                    "runtime/comms/proof/acip_demo_public_summary.json".to_string(),
                ],
                redacted_elements: vec![
                    "private_payload_refs".to_string(),
                    "raw_tool_args".to_string(),
                    "rejected_alternative_details".to_string(),
                ],
                allows_private_payload_refs: false,
                allows_raw_tool_args: false,
                allows_local_host_paths: false,
                allows_rejected_alternative_details: false,
            },
        ],
        replay_contract: AcipReplayContractV1 {
            replay_posture: AcipReplayPostureV1::FixtureBackedDeterministic,
            fixture_ref: "runtime/comms/fixtures/acip_proof_demo_packet_v1.json".to_string(),
            fixture_case: "consultation_to_coding_proposal".to_string(),
            deterministic_event_order: true,
            deterministic_redaction_views: true,
            remote_provider_required: false,
        },
        evidence_packet_refs: vec![
            "runtime/comms/proof/acip_demo_message_anchor.json".to_string(),
            "runtime/comms/proof/acip_demo_gate_result.json".to_string(),
            "runtime/comms/coding/review_handoff.json".to_string(),
        ],
    }
}

pub fn validate_acip_proof_demo_packet_v1(packet: &AcipProofDemoPacketV1) -> Result<()> {
    if packet.schema_version != ACIP_PROOF_DEMO_SCHEMA_VERSION {
        return Err(anyhow!(
            "ACIP proof demo packet requires schema_version '{}'",
            ACIP_PROOF_DEMO_SCHEMA_VERSION
        ));
    }
    validate_id(&packet.demo_id, "demo_id")?;
    validate_non_empty(&packet.title, "title")?;
    if packet.proof_classification != AcipProofClassificationV1::Proving {
        return Err(anyhow!(
            "ACIP proof demo packet must classify this v0.90.5 surface as 'proving'"
        ));
    }
    let required_modes = [
        AcipIntentV1::Consultation,
        AcipIntentV1::Negotiation,
        AcipIntentV1::CodingRequest,
    ];
    if packet.represented_modes.len() > MAX_LIST_LEN {
        return Err(anyhow!("represented_modes exceeds bounded list length"));
    }
    if packet.represented_modes.len() != required_modes.len() {
        return Err(anyhow!(
            "ACIP proof demo packet must carry exactly consultation, negotiation, and coding_request represented_modes"
        ));
    }
    let mut seen_modes = BTreeSet::new();
    for mode in &packet.represented_modes {
        if !seen_modes.insert(mode.as_str()) {
            return Err(anyhow!("represented_modes must not contain duplicates"));
        }
    }
    for required_mode in required_modes {
        if !seen_modes.contains(required_mode.as_str()) {
            return Err(anyhow!(
                "ACIP proof demo packet must carry exactly consultation, negotiation, and coding_request represented_modes"
            ));
        }
    }
    if packet.proves.is_empty() {
        return Err(anyhow!("ACIP proof demo packet requires proves"));
    }
    for proof in &packet.proves {
        validate_non_empty(proof, "proves[]")?;
    }

    validate_acip_conversation_envelope_v1(&packet.conversation)?;
    validate_acip_coding_invocation_contract_v1(&packet.coding_invocation)?;
    validate_acip_coding_outcome_v1(&packet.coding_invocation, &packet.coding_outcome)?;
    validate_acip_trace_bundle_v1(&packet.trace_bundle)?;

    if packet.conversation.messages.len() != 3 {
        return Err(anyhow!(
            "ACIP proof demo packet requires exactly consultation, negotiation, and coding_request conversation turns"
        ));
    }
    let first_message = &packet.conversation.messages[0];
    let negotiation_message = &packet.conversation.messages[1];
    let last_message = packet
        .conversation
        .messages
        .last()
        .expect("length checked above");
    if first_message.intent != AcipIntentV1::Consultation {
        return Err(anyhow!(
            "ACIP proof demo conversation must begin with consultation"
        ));
    }
    if last_message.intent != AcipIntentV1::CodingRequest {
        return Err(anyhow!(
            "ACIP proof demo conversation must terminate in coding_request before invocation binding"
        ));
    }
    if negotiation_message.intent != AcipIntentV1::Negotiation {
        return Err(anyhow!(
            "ACIP proof demo conversation must keep capability negotiation between consultation and coding_request"
        ));
    }
    if packet.coding_invocation.invocation.conversation_id
        != packet.conversation.messages[0].conversation_id
    {
        return Err(anyhow!(
            "ACIP proof demo packet must keep one canonical conversation_id across conversation and coding invocation"
        ));
    }
    if packet.trace_bundle.conversation_id != packet.coding_invocation.invocation.conversation_id {
        return Err(anyhow!(
            "ACIP proof demo packet must keep one canonical conversation_id across coding invocation and trace bundle"
        ));
    }
    if packet.coding_invocation.invocation.causal_message_id != last_message.message_id {
        return Err(anyhow!(
            "ACIP proof demo packet must bind coding invocation causal_message_id to the final coding_request message"
        ));
    }
    if packet.coding_invocation.provider_lane != AcipCodingProviderLaneV1::OtherProposalOnly {
        return Err(anyhow!(
            "ACIP proof demo packet must use provider-optional proposal-only coding lane"
        ));
    }
    if packet.coding_invocation.execution_mode != AcipCodingExecutionModeV1::StructuredProposal {
        return Err(anyhow!(
            "ACIP proof demo packet must use execution_mode 'structured_proposal'"
        ));
    }
    if packet.coding_outcome.disposition != AcipCodingDispositionV1::ProposalReadyForReview {
        return Err(anyhow!(
            "ACIP proof demo packet must return proposal_ready_for_review outcome"
        ));
    }

    let reviewer_view = packet
        .trace_bundle
        .audience_views
        .iter()
        .find(|view| view.audience == AcipTraceAudienceV1::Reviewer)
        .ok_or_else(|| {
            anyhow!("ACIP proof demo packet trace bundle requires reviewer audience view")
        })?;
    let public_view = packet
        .trace_bundle
        .audience_views
        .iter()
        .find(|view| view.audience == AcipTraceAudienceV1::Public)
        .ok_or_else(|| {
            anyhow!("ACIP proof demo packet trace bundle requires public audience view")
        })?;

    if packet.reviewer_visible_artifact_refs.is_empty() {
        return Err(anyhow!(
            "ACIP proof demo packet requires reviewer_visible_artifact_refs"
        ));
    }
    for reference in &packet.reviewer_visible_artifact_refs {
        validate_repo_relative_ref(reference, "reviewer_visible_artifact_refs[]")?;
        if !reviewer_view
            .visible_artifact_refs
            .iter()
            .any(|item| item == reference)
        {
            return Err(anyhow!(
                "reviewer_visible_artifact_refs must be present in the reviewer trace audience view"
            ));
        }
    }
    for required_ref in [
        reviewer_view.narrative_ref.as_str(),
        packet.coding_outcome.primary_output_ref.as_str(),
        packet.coding_outcome.review_handoff_ref.as_str(),
    ] {
        if !packet
            .reviewer_visible_artifact_refs
            .iter()
            .any(|reference| reference == required_ref)
        {
            return Err(anyhow!(
                "reviewer_visible_artifact_refs must include reviewer narrative, coding primary output, and review handoff"
            ));
        }
    }
    if packet.public_visible_artifact_refs.is_empty() {
        return Err(anyhow!(
            "ACIP proof demo packet requires public_visible_artifact_refs"
        ));
    }
    for reference in &packet.public_visible_artifact_refs {
        validate_repo_relative_ref(reference, "public_visible_artifact_refs[]")?;
        if !public_view
            .visible_artifact_refs
            .iter()
            .any(|item| item == reference)
        {
            return Err(anyhow!(
                "public_visible_artifact_refs must be present in the public trace audience view"
            ));
        }
    }
    for required_ref in [
        public_view.narrative_ref.as_str(),
        packet.coding_outcome.primary_output_ref.as_str(),
    ] {
        if !packet
            .public_visible_artifact_refs
            .iter()
            .any(|reference| reference == required_ref)
        {
            return Err(anyhow!(
                "public_visible_artifact_refs must include public narrative and coding primary output"
            ));
        }
    }

    let required_doc_refs = [
        "docs/milestones/v0.90.5/features/AGENT_COMMS_v1.md",
        "docs/milestones/v0.90.5/features/GOVERNED_EXECUTION_AND_TRACE.md",
        "docs/milestones/v0.90.5/features/MODEL_TESTING_AND_FLAGSHIP_DEMO.md",
        "docs/milestones/v0.90.5/DEMO_MATRIX_v0.90.5.md",
    ];
    if packet.feature_doc_refs.len() < required_doc_refs.len() {
        return Err(anyhow!(
            "ACIP proof demo packet must cite the feature and demo matrix docs it proves"
        ));
    }
    for reference in &packet.feature_doc_refs {
        validate_repo_relative_ref(reference, "feature_doc_refs[]")?;
    }
    for required_ref in required_doc_refs {
        if !packet
            .feature_doc_refs
            .iter()
            .any(|reference| reference == required_ref)
        {
            return Err(anyhow!(
                "ACIP proof demo packet feature_doc_refs must include '{}'",
                required_ref
            ));
        }
    }

    if packet.validation_commands.is_empty() {
        return Err(anyhow!(
            "ACIP proof demo packet requires validation_commands"
        ));
    }
    for command in &packet.validation_commands {
        validate_non_empty(command, "validation_commands[]")?;
    }
    if packet.validation_commands != packet.coding_invocation.validation_commands {
        return Err(anyhow!(
            "ACIP proof demo packet validation_commands must match coding invocation validation_commands"
        ));
    }
    let required_non_proving = [
        "This proof packet does not prove live transport.",
        "This proof packet does not prove encrypted external transport.",
        "This proof packet does not prove reputation systems.",
        "This proof packet does not prove cross-polis federation.",
    ];
    if packet.non_proving_statements.len() < required_non_proving.len() {
        return Err(anyhow!(
            "ACIP proof demo packet must carry explicit non_proving_statements"
        ));
    }
    for statement in &packet.non_proving_statements {
        validate_non_empty(statement, "non_proving_statements[]")?;
    }
    for required in required_non_proving {
        if !packet
            .non_proving_statements
            .iter()
            .any(|statement| statement == required)
        {
            return Err(anyhow!(
                "ACIP proof demo packet must include non-proving statement '{}'",
                required
            ));
        }
    }

    Ok(())
}
