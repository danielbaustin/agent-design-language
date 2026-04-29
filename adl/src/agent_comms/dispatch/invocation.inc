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


pub(crate) fn sample_invocation_contract() -> AcipInvocationContractV1 {
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

pub(crate) fn sample_invocation_event(
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
