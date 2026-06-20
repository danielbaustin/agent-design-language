pub fn acip_local_carrier_v1_schema_json() -> Result<String> {
    serde_json::to_string_pretty(&schema_for!(AcipLocalCarrierV1))
        .context("serialize ACIP local carrier v1 schema")
}

pub fn acip_local_invocation_exchange_v1_schema_json() -> Result<String> {
    serde_json::to_string_pretty(&schema_for!(AcipLocalInvocationExchangeV1))
        .context("serialize ACIP local invocation exchange v1 schema")
}

pub fn validate_acip_local_carrier_v1_value(value: &JsonValue) -> Result<AcipLocalCarrierV1> {
    let carrier: AcipLocalCarrierV1 =
        serde_json::from_value(value.clone()).context("parse ACIP local carrier v1")?;
    validate_acip_local_carrier_v1(&carrier)?;
    Ok(carrier)
}

pub fn validate_acip_local_carrier_v1(carrier: &AcipLocalCarrierV1) -> Result<()> {
    if carrier.schema_version != ACIP_LOCAL_CARRIER_SCHEMA_VERSION {
        return Err(anyhow!(
            "ACIP local carrier requires schema_version '{}'",
            ACIP_LOCAL_CARRIER_SCHEMA_VERSION
        ));
    }
    if carrier.routes.is_empty() {
        return Err(anyhow!("ACIP local carrier requires at least one route"));
    }

    let mut seen_agents = BTreeSet::new();
    for route in &carrier.routes {
        validate_address(&route.agent, "routes[].agent")?;
        if route.agent.kind != AcipAddressKindV1::Agent {
            return Err(anyhow!(
                "ACIP local carrier routes must target concrete agent addresses"
            ));
        }
        if !seen_agents.insert(route.agent.id.clone()) {
            return Err(anyhow!(
                "ACIP local carrier contains duplicate route for '{}'",
                route.agent.id
            ));
        }
        if route.route_class != AcipRouteClassV1::LocalOnly {
            return Err(anyhow!(
                "ACIP local carrier route '{}' must remain local_only in v0.91.6",
                route.agent.id
            ));
        }
        if route.allowed_caller_classes.is_empty() {
            return Err(anyhow!(
                "ACIP local carrier route '{}' requires allowed_caller_classes",
                route.agent.id
            ));
        }
        let mut seen_caller_classes = BTreeSet::new();
        for caller_class in &route.allowed_caller_classes {
            if !seen_caller_classes.insert(caller_class.clone()) {
                return Err(anyhow!(
                    "ACIP local carrier route '{}' contains duplicate allowed_caller_classes entries",
                    route.agent.id
                ));
            }
        }
        if route.supported_intents.is_empty() {
            return Err(anyhow!(
                "ACIP local carrier route '{}' requires supported_intents",
                route.agent.id
            ));
        }

        let mut seen_intents = BTreeSet::new();
        for intent in &route.supported_intents {
            if !seen_intents.insert(intent.as_str().to_string()) {
                return Err(anyhow!(
                    "ACIP local carrier route '{}' contains duplicate supported_intents entries",
                    route.agent.id
                ));
            }
            match intent {
                AcipIntentV1::ReviewRequest | AcipIntentV1::CodingRequest => {}
                _ => {
                    return Err(anyhow!(
                        "ACIP local carrier route '{}' must only advertise supported invocation intents",
                        route.agent.id
                    ))
                }
            }
        }

        match route.handler {
            AcipLocalHandlerKindV1::Review => {
                if route.supported_intents != vec![AcipIntentV1::ReviewRequest] {
                    return Err(anyhow!(
                        "ACIP local carrier review handler '{}' must only support review_request",
                        route.agent.id
                    ));
                }
            }
            AcipLocalHandlerKindV1::Coding => {
                if route.supported_intents != vec![AcipIntentV1::CodingRequest] {
                    return Err(anyhow!(
                        "ACIP local carrier coding handler '{}' must only support coding_request",
                        route.agent.id
                    ));
                }
            }
        }
    }

    Ok(())
}

pub fn validate_acip_local_invocation_exchange_v1_value(
    value: &JsonValue,
) -> Result<AcipLocalInvocationExchangeV1> {
    let exchange: AcipLocalInvocationExchangeV1 = serde_json::from_value(value.clone())
        .context("parse ACIP local invocation exchange v1")?;
    validate_acip_local_invocation_exchange_v1(&exchange)?;
    Ok(exchange)
}

pub fn validate_acip_local_invocation_exchange_v1(
    exchange: &AcipLocalInvocationExchangeV1,
) -> Result<()> {
    if exchange.schema_version != ACIP_LOCAL_INVOCATION_EXCHANGE_SCHEMA_VERSION {
        return Err(anyhow!(
            "ACIP local invocation exchange requires schema_version '{}'",
            ACIP_LOCAL_INVOCATION_EXCHANGE_SCHEMA_VERSION
        ));
    }
    validate_acip_local_carrier_v1(&exchange.carrier)?;
    validate_acip_message_envelope_v1(&exchange.request_message)?;
    validate_acip_invocation_contract_v1(&exchange.invocation_contract)?;
    validate_acip_invocation_event_against_contract(
        &exchange.invocation_contract,
        &exchange.invocation_event,
    )?;
    validate_acip_message_envelope_v1(&exchange.response_message)?;

    if exchange.request_message.message_id != exchange.invocation_contract.causal_message_id {
        return Err(anyhow!(
            "local exchange request_message must bind invocation_contract.causal_message_id"
        ));
    }
    if exchange.request_message.conversation_id != exchange.invocation_contract.conversation_id {
        return Err(anyhow!(
            "local exchange request_message must bind invocation_contract.conversation_id"
        ));
    }
    if exchange.request_message.sender != exchange.invocation_contract.caller
        || exchange.request_message.recipient != exchange.invocation_contract.target
    {
        return Err(anyhow!(
            "local exchange request_message must preserve invocation caller/target identity"
        ));
    }
    if exchange.request_message.trace_requirement != exchange.invocation_contract.trace_requirement {
        return Err(anyhow!(
            "local exchange request_message must preserve invocation trace_requirement"
        ));
    }
    validate_request_contract_alignment(
        &exchange.request_message,
        &exchange.invocation_contract,
        "local exchange",
    )?;
    if exchange.response_message.sender != exchange.invocation_contract.target
        || exchange.response_message.recipient != exchange.invocation_contract.caller
    {
        return Err(anyhow!(
            "local exchange response_message must reverse the invocation caller/target identity"
        ));
    }
    if exchange.response_message.trace_requirement
        != exchange.invocation_contract.trace_requirement
    {
        return Err(anyhow!(
            "local exchange response_message must preserve invocation trace_requirement"
        ));
    }
    if exchange.response_message.intent != AcipIntentV1::Conversation {
        return Err(anyhow!(
            "local exchange response_message must use conversation intent for bounded local replies"
        ));
    }
    if exchange.response_message.conversation_id != exchange.invocation_contract.conversation_id {
        return Err(anyhow!(
            "local exchange response_message must preserve invocation conversation_id"
        ));
    }
    if exchange.response_message.prior_message_id.as_deref()
        != Some(exchange.request_message.message_id.as_str())
    {
        return Err(anyhow!(
            "local exchange response_message must preserve the request prior_message_id link"
        ));
    }
    if exchange.response_message.monotonic_order <= exchange.request_message.monotonic_order {
        return Err(anyhow!(
            "local exchange response_message must advance monotonic_order"
        ));
    }
    if exchange.request_message.correlation_id.as_deref()
        != Some(exchange.invocation_contract.invocation_id.as_str())
    {
        return Err(anyhow!(
            "local exchange request_message must bind correlation_id to invocation_contract.invocation_id"
        ));
    }
    if exchange.response_message.correlation_id.as_deref()
        != Some(exchange.invocation_contract.invocation_id.as_str())
    {
        return Err(anyhow!(
            "local exchange response_message must preserve invocation_contract.invocation_id as correlation_id"
        ));
    }
    if exchange.response_message.artifact_refs != exchange.invocation_event.output_refs {
        return Err(anyhow!(
            "local exchange response_message.artifact_refs must match invocation_event.output_refs"
        ));
    }
    let route =
        ensure_local_target_route_exists(&exchange.carrier, &exchange.invocation_contract.target)?;
    match exchange.invocation_event.status {
        AcipInvocationStatusV1::Completed | AcipInvocationStatusV1::Partial => {
            ensure_local_route_supports_contract(route, &exchange.invocation_contract)?;
        }
        AcipInvocationStatusV1::Refused => {
            validate_local_refusal_anchor(
                route,
                &exchange.invocation_contract,
                &exchange.invocation_event,
            )?;
        }
        AcipInvocationStatusV1::Requested | AcipInvocationStatusV1::Failed => {
            ensure_local_route_supports_contract(route, &exchange.invocation_contract)?;
        }
    }

    Ok(())
}

pub fn execute_acip_local_invocation_round_trip(
    carrier: &AcipLocalCarrierV1,
    request_message: &AcipMessageEnvelopeV1,
    invocation_contract: &AcipInvocationContractV1,
) -> Result<AcipLocalInvocationExchangeV1> {
    validate_acip_local_carrier_v1(carrier)?;
    validate_acip_message_envelope_v1(request_message)?;
    validate_acip_invocation_contract_v1(invocation_contract)?;
    validate_request_contract_alignment(
        request_message,
        invocation_contract,
        "local ACIP carrier",
    )?;
    let route = ensure_local_target_route_exists(carrier, &invocation_contract.target)?;
    let invocation_event = match authorize_local_invocation(route, invocation_contract) {
        Ok(route) => materialize_local_invocation_event(route, invocation_contract)?,
        Err(failure) => materialize_local_refusal_event(route, invocation_contract, &failure),
    };
    let response_message =
        build_local_response_message(request_message, invocation_contract, &invocation_event)?;

    let exchange = AcipLocalInvocationExchangeV1 {
        schema_version: ACIP_LOCAL_INVOCATION_EXCHANGE_SCHEMA_VERSION.to_string(),
        carrier: carrier.clone(),
        request_message: request_message.clone(),
        invocation_contract: invocation_contract.clone(),
        invocation_event,
        response_message,
    };
    validate_acip_local_invocation_exchange_v1(&exchange)?;
    Ok(exchange)
}

pub fn sample_acip_local_carrier_v1() -> AcipLocalCarrierV1 {
    AcipLocalCarrierV1 {
        schema_version: ACIP_LOCAL_CARRIER_SCHEMA_VERSION.to_string(),
        routes: vec![
            AcipLocalRouteV1 {
                agent: AcipAddressV1 {
                    kind: AcipAddressKindV1::Agent,
                    id: "reviewer.agent".to_string(),
                },
                route_class: AcipRouteClassV1::LocalOnly,
                allowed_caller_classes: vec![AcipCallerClassV1::WorkflowAgent],
                supported_intents: vec![AcipIntentV1::ReviewRequest],
                handler: AcipLocalHandlerKindV1::Review,
            },
            AcipLocalRouteV1 {
                agent: AcipAddressV1 {
                    kind: AcipAddressKindV1::Agent,
                    id: "coding.agent".to_string(),
                },
                route_class: AcipRouteClassV1::LocalOnly,
                allowed_caller_classes: vec![AcipCallerClassV1::WorkflowAgent],
                supported_intents: vec![AcipIntentV1::CodingRequest],
                handler: AcipLocalHandlerKindV1::Coding,
            },
        ],
    }
}

#[cfg(test)]
pub(crate) fn sample_local_review_request_message(
    contract: &AcipInvocationContractV1,
) -> AcipMessageEnvelopeV1 {
    let mut message = transport::sample_message_with_payloads(
        &contract.causal_message_id,
        &contract.conversation_id,
        1,
        contract.intent.clone(),
        &contract.purpose,
        contract
            .input_refs
            .iter()
            .enumerate()
            .map(|(index, input_ref)| {
                payload_ref(
                    &format!("input_{index}"),
                    input_ref,
                    "application/json",
                    1_024 + index as u64,
                    Some("Bounded invocation input for local ACIP carrier proof."),
                )
            })
            .collect(),
    );
    message.sender = contract.caller.clone();
    message.recipient = contract.target.clone();
    message.trace_requirement = contract.trace_requirement.clone();
    message.correlation_id = Some(contract.invocation_id.clone());
    message
}

fn materialize_local_invocation_event(
    route: &AcipLocalRouteV1,
    contract: &AcipInvocationContractV1,
) -> Result<AcipInvocationEventV1> {
    let required_outputs: Vec<_> = contract
        .expected_outputs
        .iter()
        .filter(|output| output.required)
        .collect();
    let output_refs = match contract.response_channel.kind {
        AcipResponseChannelKindV1::DirectReply => required_outputs
            .iter()
            .map(|output| {
                format!(
                    "runtime/comms/local_carrier/{}/{}.json",
                    contract.invocation_id, output.output_id
                )
            })
            .collect(),
        AcipResponseChannelKindV1::ArtifactReply => {
            if required_outputs.len() == 1 {
                vec![contract.response_channel.channel_ref.clone()]
            } else {
                let channel_path = Path::new(&contract.response_channel.channel_ref);
                let Some(parent) = channel_path.parent() else {
                    return Err(anyhow!(
                        "artifact_reply response_channel.channel_ref must include a parent path"
                    ));
                };
                if parent.as_os_str().is_empty() {
                    return Err(anyhow!(
                        "artifact_reply response_channel.channel_ref must include a parent path"
                    ));
                }
                let parent = parent.to_string_lossy();
                required_outputs
                    .iter()
                    .map(|output| format!("{parent}/{}.json", output.output_id))
                    .collect()
            }
        }
    };

    let evidence_refs = match route.handler {
        AcipLocalHandlerKindV1::Review => {
            vec!["runtime/comms/local_carrier/evidence/review_trace.json".to_string()]
        }
        AcipLocalHandlerKindV1::Coding => {
            vec!["runtime/comms/local_carrier/evidence/coding_trace.json".to_string()]
        }
    };

    Ok(sample_invocation_event(
        contract,
        AcipInvocationStatusV1::Completed,
        output_refs,
        "completed_declared_outputs".to_string(),
        None,
        None,
        evidence_refs,
    ))
}

fn materialize_local_refusal_event(
    route: &AcipLocalRouteV1,
    contract: &AcipInvocationContractV1,
    failure: &LocalAuthorizationFailure,
) -> AcipInvocationEventV1 {
    let evidence_refs = match route.handler {
        AcipLocalHandlerKindV1::Review => {
            vec!["runtime/comms/local_carrier/evidence/review_refusal_trace.json".to_string()]
        }
        AcipLocalHandlerKindV1::Coding => {
            vec!["runtime/comms/local_carrier/evidence/coding_refusal_trace.json".to_string()]
        }
    };

    sample_invocation_event(
        contract,
        AcipInvocationStatusV1::Refused,
        Vec::new(),
        failure.stop_reason.to_string(),
        Some(failure.refusal_code.to_string()),
        None,
        evidence_refs,
    )
}

fn build_local_response_message(
    request_message: &AcipMessageEnvelopeV1,
    contract: &AcipInvocationContractV1,
    invocation_event: &AcipInvocationEventV1,
) -> Result<AcipMessageEnvelopeV1> {
    let body = match invocation_event.status {
        AcipInvocationStatusV1::Completed => format!(
            "Invocation '{}' completed locally with status '{}' and {} output artifact(s).",
            contract.invocation_id,
            invocation_event.status.as_str(),
            invocation_event.output_refs.len()
        ),
        AcipInvocationStatusV1::Refused => format!(
            "Invocation '{}' was refused locally with code '{}' before execution.",
            contract.invocation_id,
            invocation_event.refusal_code.as_deref().unwrap_or("unknown_refusal")
        ),
        AcipInvocationStatusV1::Failed => format!(
            "Invocation '{}' failed locally with code '{}'.",
            contract.invocation_id,
            invocation_event.failure_code.as_deref().unwrap_or("unknown_failure")
        ),
        _ => format!(
            "Invocation '{}' ended locally with status '{}'.",
            contract.invocation_id,
            invocation_event.status.as_str()
        ),
    };
    let mut response = transport::sample_message(
        &format!("msg-{}-reply", contract.invocation_id),
        &contract.conversation_id,
        request_message.monotonic_order + 1,
        AcipIntentV1::Conversation,
        &body,
    );
    response.sender = contract.target.clone();
    response.recipient = contract.caller.clone();
    response.trace_requirement = contract.trace_requirement.clone();
    response.artifact_refs = invocation_event.output_refs.clone();
    response.correlation_id = Some(contract.invocation_id.clone());
    response.prior_message_id = Some(request_message.message_id.clone());
    response.authority_scope = None;

    validate_acip_message_envelope_v1(&response)?;
    Ok(response)
}

fn validate_request_contract_alignment(
    request_message: &AcipMessageEnvelopeV1,
    invocation_contract: &AcipInvocationContractV1,
    scope: &str,
) -> Result<()> {
    if request_message.intent != invocation_contract.intent {
        return Err(anyhow!(
            "{scope} requires request_message.intent to match invocation_contract.intent"
        ));
    }
    if request_message.message_id != invocation_contract.causal_message_id {
        return Err(anyhow!(
            "{scope} requires request_message.message_id to match invocation_contract.causal_message_id"
        ));
    }
    if request_message.conversation_id != invocation_contract.conversation_id {
        return Err(anyhow!(
            "{scope} requires request_message.conversation_id to match invocation_contract.conversation_id"
        ));
    }
    if request_message.sender != invocation_contract.caller
        || request_message.recipient != invocation_contract.target
    {
        return Err(anyhow!(
            "{scope} requires request_message sender/recipient to match invocation caller/target"
        ));
    }
    if request_message.correlation_id.as_deref()
        != Some(invocation_contract.invocation_id.as_str())
    {
        return Err(anyhow!(
            "{scope} requires request_message.correlation_id to match invocation_contract.invocation_id"
        ));
    }
    if request_message.authority_scope.as_ref() != Some(&invocation_contract.authority_scope) {
        return Err(anyhow!(
            "{scope} requires request_message.authority_scope to match invocation_contract.authority_scope"
        ));
    }

    Ok(())
}

fn ensure_local_target_route_exists<'a>(
    carrier: &'a AcipLocalCarrierV1,
    target: &AcipAddressV1,
) -> Result<&'a AcipLocalRouteV1> {
    carrier
        .routes
        .iter()
        .find(|route| route.agent == *target)
        .ok_or_else(|| anyhow!("local ACIP carrier has no route for '{}'", target.id))
}

fn ensure_local_route_supports_contract<'a>(
    route: &'a AcipLocalRouteV1,
    invocation_contract: &AcipInvocationContractV1,
) -> Result<&'a AcipLocalRouteV1> {

    if !route.supported_intents.contains(&invocation_contract.intent) {
        return Err(anyhow!(
            "local ACIP carrier route '{}' does not support intent '{}'",
            route.agent.id,
            invocation_contract.intent.as_str()
        ));
    }

    Ok(route)
}

#[derive(Debug, Clone)]
struct LocalAuthorizationFailure {
    refusal_code: &'static str,
    stop_reason: &'static str,
}

fn authorize_local_invocation<'a>(
    route: &'a AcipLocalRouteV1,
    invocation_contract: &AcipInvocationContractV1,
) -> Result<&'a AcipLocalRouteV1, LocalAuthorizationFailure> {
    if !route.supported_intents.contains(&invocation_contract.intent) {
        return Err(LocalAuthorizationFailure {
            refusal_code: "intent_denied",
            stop_reason: "route_intent_not_authorized",
        });
    }

    if invocation_contract.route_class != AcipRouteClassV1::LocalOnly
        || route.route_class != invocation_contract.route_class
    {
        return Err(LocalAuthorizationFailure {
            refusal_code: "route_class_denied",
            stop_reason: "route_class_not_authorized",
        });
    }

    if !route
        .allowed_caller_classes
        .contains(&invocation_contract.caller_class)
    {
        return Err(LocalAuthorizationFailure {
            refusal_code: "caller_class_denied",
            stop_reason: "caller_class_not_authorized",
        });
    }

    Ok(route)
}

fn validate_local_refusal_anchor(
    route: &AcipLocalRouteV1,
    invocation_contract: &AcipInvocationContractV1,
    invocation_event: &AcipInvocationEventV1,
) -> Result<()> {
    match invocation_event.refusal_code.as_deref() {
        Some("intent_denied") => {
            if route.supported_intents.contains(&invocation_contract.intent) {
                return Err(anyhow!(
                    "refused local exchange with refusal_code 'intent_denied' must bind to an unsupported route intent"
                ));
            }
        }
        Some("route_class_denied") => {
            if !route.supported_intents.contains(&invocation_contract.intent) {
                return Err(anyhow!(
                    "refused local exchange with refusal_code 'route_class_denied' must preserve a supported route intent"
                ));
            }
            if invocation_contract.route_class == AcipRouteClassV1::LocalOnly
                && route.route_class == invocation_contract.route_class
            {
                return Err(anyhow!(
                    "refused local exchange with refusal_code 'route_class_denied' must bind to a real route-class mismatch"
                ));
            }
        }
        Some("caller_class_denied") => {
            if !route.supported_intents.contains(&invocation_contract.intent) {
                return Err(anyhow!(
                    "refused local exchange with refusal_code 'caller_class_denied' must preserve a supported route intent"
                ));
            }
            if invocation_contract.route_class != AcipRouteClassV1::LocalOnly
                || route.route_class != invocation_contract.route_class
            {
                return Err(anyhow!(
                    "refused local exchange with refusal_code 'caller_class_denied' must bind to a route-class-authorized path"
                ));
            }
            if route
                .allowed_caller_classes
                .contains(&invocation_contract.caller_class)
            {
                return Err(anyhow!(
                    "refused local exchange with refusal_code 'caller_class_denied' must bind to a real caller-class denial"
                ));
            }
        }
        Some(other) => {
            return Err(anyhow!(
                "refused local exchange uses unsupported refusal_code '{}'",
                other
            ));
        }
        None => return Err(anyhow!("refused local exchange must preserve refusal_code")),
    }

    Ok(())
}
