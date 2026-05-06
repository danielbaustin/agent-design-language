pub fn acip_a2a_adapter_boundary_v1_schema_json() -> Result<String> {
    Ok(serde_json::to_string_pretty(&schema_for!(AcipA2aAdapterBoundaryV1))?)
}

pub fn acip_a2a_fixture_set_v1_schema_json() -> Result<String> {
    Ok(serde_json::to_string_pretty(&schema_for!(AcipA2aFixtureSetV1))?)
}

pub fn validate_acip_a2a_adapter_boundary_v1_value(value: &JsonValue) -> Result<()> {
    let adapter: AcipA2aAdapterBoundaryV1 =
        serde_json::from_value(value.clone()).context("parse ACIP A2A adapter boundary v1")?;
    validate_acip_a2a_adapter_boundary_v1(&adapter)
}

pub fn validate_acip_a2a_adapter_boundary_v1(adapter: &AcipA2aAdapterBoundaryV1) -> Result<()> {
    if adapter.schema_version != ACIP_A2A_ADAPTER_SCHEMA_VERSION {
        return Err(anyhow!(
            "ACIP A2A adapter boundary requires schema_version '{}'",
            ACIP_A2A_ADAPTER_SCHEMA_VERSION
        ));
    }
    validate_id(&adapter.adapter_id, "adapter_id")?;
    validate_non_empty(&adapter.purpose, "purpose")?;

    validate_repo_relative_ref(
        &adapter.identity_claim.agent_card_ref,
        "identity_claim.agent_card_ref",
    )?;
    validate_id(
        &adapter.identity_claim.agent_id_claim,
        "identity_claim.agent_id_claim",
    )?;
    validate_non_empty(
        &adapter.identity_claim.display_name_claim,
        "identity_claim.display_name_claim",
    )?;
    if adapter.identity_claim.advertised_capabilities.is_empty() {
        return Err(anyhow!(
            "identity_claim.advertised_capabilities must not be empty"
        ));
    }
    let mut advertised = BTreeSet::new();
    for capability in &adapter.identity_claim.advertised_capabilities {
        validate_id(capability, "identity_claim.advertised_capabilities[]")?;
        if !advertised.insert(capability.clone()) {
            return Err(anyhow!(
                "identity_claim.advertised_capabilities[] must not contain duplicates"
            ));
        }
    }

    if adapter.capability_mappings.is_empty() {
        return Err(anyhow!("capability_mappings must not be empty"));
    }
    let mut mapped_claims = BTreeSet::new();
    for mapping in &adapter.capability_mappings {
        validate_id(
            &mapping.external_capability_claim,
            "capability_mappings[].external_capability_claim",
        )?;
        if !advertised.contains(&mapping.external_capability_claim) {
            return Err(anyhow!(
                "capability_mappings[].external_capability_claim must be advertised in the Agent Card"
            ));
        }
        if !mapped_claims.insert(mapping.external_capability_claim.clone()) {
            return Err(anyhow!(
                "capability_mappings[] must not map the same external capability twice"
            ));
        }
        validate_id(&mapping.adl_capability, "capability_mappings[].adl_capability")?;
        validate_repo_relative_ref(
            &mapping.policy_basis_ref,
            "capability_mappings[].policy_basis_ref",
        )?;
        if !mapping.invocation_required || !mapping.direct_execution_forbidden {
            return Err(anyhow!(
                "A2A capability mappings must route through agent.invoke and forbid direct execution"
            ));
        }
    }

    if adapter.trust_classification.basis_refs.is_empty() {
        return Err(anyhow!(
            "trust_classification.basis_refs must not be empty"
        ));
    }
    for basis_ref in &adapter.trust_classification.basis_refs {
        validate_repo_relative_ref(basis_ref, "trust_classification.basis_refs[]")?;
    }
    if adapter.trust_classification.execution_authority_granted {
        return Err(anyhow!(
            "trust_classification must not grant execution authority directly"
        ));
    }

    if adapter.invoke_boundary.required_entrypoint != "agent.invoke" {
        return Err(anyhow!(
            "A2A invoke boundary must route through agent.invoke"
        ));
    }
    if adapter.invoke_boundary.invocation_contract_schema_ref
        != ACIP_INVOCATION_CONTRACT_SCHEMA_VERSION
    {
        return Err(anyhow!(
            "A2A invoke boundary must bind to '{}'",
            ACIP_INVOCATION_CONTRACT_SCHEMA_VERSION
        ));
    }
    if adapter.invoke_boundary.trace_bundle_schema_ref != ACIP_TRACE_BUNDLE_SCHEMA_VERSION {
        return Err(anyhow!(
            "A2A invoke boundary must bind to '{}'",
            ACIP_TRACE_BUNDLE_SCHEMA_VERSION
        ));
    }
    if !adapter.invoke_boundary.direct_execution_forbidden {
        return Err(anyhow!(
            "A2A invoke boundary must forbid direct execution"
        ));
    }

    if !adapter.transport_boundary.local_scope_only {
        return Err(anyhow!(
            "A2A transport boundary must remain local-only in v0.91"
        ));
    }
    if adapter.transport_boundary.external_transport_status
        != AcipA2aTransportBoundaryStatusV1::DeferredUntilTlsEquivalent
    {
        return Err(anyhow!(
            "A2A external transport must remain deferred until TLS-equivalent protection exists"
        ));
    }
    if adapter.transport_boundary.refusal_mode != "refuse" {
        return Err(anyhow!(
            "A2A transport boundary refusal_mode must be 'refuse'"
        ));
    }

    if adapter.trace_evidence_refs.is_empty() {
        return Err(anyhow!("trace_evidence_refs must not be empty"));
    }
    for evidence_ref in &adapter.trace_evidence_refs {
        validate_repo_relative_ref(evidence_ref, "trace_evidence_refs[]")?;
    }

    if adapter.non_claims.is_empty() {
        return Err(anyhow!("non_claims must not be empty"));
    }
    Ok(())
}

pub fn validate_acip_a2a_fixture_set_v1(fixtures: &AcipA2aFixtureSetV1) -> Result<()> {
    if fixtures.schema_version != ACIP_A2A_FIXTURE_SCHEMA_VERSION {
        return Err(anyhow!(
            "ACIP A2A fixture set requires schema_version '{}'",
            ACIP_A2A_FIXTURE_SCHEMA_VERSION
        ));
    }
    if fixtures.valid_adapters.is_empty() {
        return Err(anyhow!("ACIP A2A fixture set requires valid_adapters"));
    }
    if fixtures.negative_cases.is_empty() {
        return Err(anyhow!("ACIP A2A fixture set requires negative_cases"));
    }

    let mut trust_classes = BTreeSet::new();
    for adapter in &fixtures.valid_adapters {
        validate_acip_a2a_adapter_boundary_v1(adapter)?;
        trust_classes.insert(adapter.trust_classification.classification.clone());
    }
    for required in [
        AcipA2aTrustClassV1::Naked,
        AcipA2aTrustClassV1::Guest,
        AcipA2aTrustClassV1::Citizen,
    ] {
        if !trust_classes.contains(&required) {
            return Err(anyhow!(
                "ACIP A2A fixture set must cover trust class '{:?}'",
                required
            ));
        }
    }

    for case in &fixtures.negative_cases {
        validate_negative_case_name(&case.name, "negative_cases[].name")?;
        validate_non_empty(
            &case.expected_error_substring,
            "negative_cases[].expected_error_substring",
        )?;
        validate_negative_result(
            validate_acip_a2a_adapter_boundary_v1_value(&case.value),
            &case.expected_error_substring,
        )?;
    }
    Ok(())
}

pub fn acip_a2a_fixture_set_v1() -> AcipA2aFixtureSetV1 {
    let feature_doc_ref = "docs/milestones/v0.91/features/A2A_EXTERNAL_AGENT_ADAPTER.md";
    let trace_ref = "runtime/comms/trace/a2a-local-boundary-trace.json";
    let agent_card_ref = "runtime/comms/a2a/agent-card-alpha.json";

    let mk_adapter = |adapter_id: &str,
                      trust: AcipA2aTrustClassV1,
                      capability: &str,
                      adl_capability: &str,
                      basis_ref: &str| AcipA2aAdapterBoundaryV1 {
        schema_version: ACIP_A2A_ADAPTER_SCHEMA_VERSION.to_string(),
        adapter_id: adapter_id.to_string(),
        purpose: "Governed A2A adapter boundary over local ACIP substrate.".to_string(),
        identity_claim: AcipA2aAgentCardClaimV1 {
            agent_card_ref: agent_card_ref.to_string(),
            agent_id_claim: format!("{adapter_id}_agent"),
            display_name_claim: format!("{adapter_id} Agent"),
            advertised_capabilities: vec![
                capability.to_string(),
                "share_trace_summary".to_string(),
            ],
        },
        capability_mappings: vec![AcipA2aCapabilityMappingV1 {
            external_capability_claim: capability.to_string(),
            adl_capability: adl_capability.to_string(),
            policy_basis_ref: feature_doc_ref.to_string(),
            invocation_required: true,
            direct_execution_forbidden: true,
        }],
        trust_classification: AcipA2aTrustClassificationV1 {
            classification: trust,
            basis_refs: vec![basis_ref.to_string()],
            execution_authority_granted: false,
        },
        invoke_boundary: AcipA2aInvokeBoundaryV1 {
            required_entrypoint: "agent.invoke".to_string(),
            invocation_contract_schema_ref: ACIP_INVOCATION_CONTRACT_SCHEMA_VERSION.to_string(),
            trace_bundle_schema_ref: ACIP_TRACE_BUNDLE_SCHEMA_VERSION.to_string(),
            direct_execution_forbidden: true,
        },
        transport_boundary: AcipA2aTransportBoundaryV1 {
            local_scope_only: true,
            external_transport_status:
                AcipA2aTransportBoundaryStatusV1::DeferredUntilTlsEquivalent,
            required_security_posture: AcipA2aSecurityPostureV1::TlsOrMutualTlsEquivalent,
            refusal_mode: "refuse".to_string(),
        },
        trace_evidence_refs: vec![trace_ref.to_string()],
        non_claims: vec![
            "No direct execution authority is granted from Agent Card claims.".to_string(),
            "External transport remains deferred in v0.91.".to_string(),
        ],
    };

    let naked = mk_adapter(
        "a2a_naked_boundary",
        AcipA2aTrustClassV1::Naked,
        "request_consultation",
        "consult",
        "runtime/comms/a2a/trust/naked.json",
    );
    let guest = mk_adapter(
        "a2a_guest_boundary",
        AcipA2aTrustClassV1::Guest,
        "review_source_tree",
        "review",
        "runtime/comms/a2a/trust/guest.json",
    );
    let citizen = mk_adapter(
        "a2a_citizen_boundary",
        AcipA2aTrustClassV1::Citizen,
        "submit_patch_proposal",
        "delegate",
        "runtime/comms/a2a/trust/citizen.json",
    );

    AcipA2aFixtureSetV1 {
        schema_version: ACIP_A2A_FIXTURE_SCHEMA_VERSION.to_string(),
        valid_adapters: vec![naked, guest, citizen],
        negative_cases: vec![
            AcipA2aNegativeCaseV1 {
                name: "parallel_authority_model".to_string(),
                expected_error_substring:
                    "trust_classification must not grant execution authority directly"
                        .to_string(),
                value: json!({
                    "schema_version": ACIP_A2A_ADAPTER_SCHEMA_VERSION,
                    "adapter_id": "a2a_parallel_authority",
                    "purpose": "bad",
                    "identity_claim": {
                        "agent_card_ref": agent_card_ref,
                        "agent_id_claim": "parallel_agent",
                        "display_name_claim": "Parallel Agent",
                        "advertised_capabilities": ["request_consultation"]
                    },
                    "capability_mappings": [{
                        "external_capability_claim": "request_consultation",
                        "adl_capability": "consult",
                        "policy_basis_ref": feature_doc_ref,
                        "invocation_required": true,
                        "direct_execution_forbidden": true
                    }],
                    "trust_classification": {
                        "classification": "guest",
                        "basis_refs": ["runtime/comms/a2a/trust/guest.json"],
                        "execution_authority_granted": true
                    },
                    "invoke_boundary": {
                        "required_entrypoint": "agent.invoke",
                        "invocation_contract_schema_ref": ACIP_INVOCATION_CONTRACT_SCHEMA_VERSION,
                        "trace_bundle_schema_ref": ACIP_TRACE_BUNDLE_SCHEMA_VERSION,
                        "direct_execution_forbidden": true
                    },
                    "transport_boundary": {
                        "local_scope_only": true,
                        "external_transport_status": "deferred_until_tls_equivalent",
                        "required_security_posture": "tls_or_mutual_tls_equivalent",
                        "refusal_mode": "refuse"
                    },
                    "trace_evidence_refs": [trace_ref],
                    "non_claims": ["External transport remains deferred in v0.91."]
                }),
            },
            AcipA2aNegativeCaseV1 {
                name: "direct_execution_grant".to_string(),
                expected_error_substring:
                    "A2A capability mappings must route through agent.invoke and forbid direct execution"
                        .to_string(),
                value: json!({
                    "schema_version": ACIP_A2A_ADAPTER_SCHEMA_VERSION,
                    "adapter_id": "a2a_direct_exec",
                    "purpose": "bad",
                    "identity_claim": {
                        "agent_card_ref": agent_card_ref,
                        "agent_id_claim": "direct_exec_agent",
                        "display_name_claim": "Direct Exec Agent",
                        "advertised_capabilities": ["submit_patch_proposal"]
                    },
                    "capability_mappings": [{
                        "external_capability_claim": "submit_patch_proposal",
                        "adl_capability": "delegate",
                        "policy_basis_ref": feature_doc_ref,
                        "invocation_required": false,
                        "direct_execution_forbidden": false
                    }],
                    "trust_classification": {
                        "classification": "citizen",
                        "basis_refs": ["runtime/comms/a2a/trust/citizen.json"],
                        "execution_authority_granted": false
                    },
                    "invoke_boundary": {
                        "required_entrypoint": "agent.invoke",
                        "invocation_contract_schema_ref": ACIP_INVOCATION_CONTRACT_SCHEMA_VERSION,
                        "trace_bundle_schema_ref": ACIP_TRACE_BUNDLE_SCHEMA_VERSION,
                        "direct_execution_forbidden": true
                    },
                    "transport_boundary": {
                        "local_scope_only": true,
                        "external_transport_status": "deferred_until_tls_equivalent",
                        "required_security_posture": "tls_or_mutual_tls_equivalent",
                        "refusal_mode": "refuse"
                    },
                    "trace_evidence_refs": [trace_ref],
                    "non_claims": ["No direct execution authority is granted from Agent Card claims."]
                }),
            },
            AcipA2aNegativeCaseV1 {
                name: "missing_agent_invoke_boundary".to_string(),
                expected_error_substring:
                    "A2A invoke boundary must route through agent.invoke".to_string(),
                value: json!({
                    "schema_version": ACIP_A2A_ADAPTER_SCHEMA_VERSION,
                    "adapter_id": "a2a_missing_invoke",
                    "purpose": "bad",
                    "identity_claim": {
                        "agent_card_ref": agent_card_ref,
                        "agent_id_claim": "missing_invoke_agent",
                        "display_name_claim": "Missing Invoke Agent",
                        "advertised_capabilities": ["review_source_tree"]
                    },
                    "capability_mappings": [{
                        "external_capability_claim": "review_source_tree",
                        "adl_capability": "review",
                        "policy_basis_ref": feature_doc_ref,
                        "invocation_required": true,
                        "direct_execution_forbidden": true
                    }],
                    "trust_classification": {
                        "classification": "guest",
                        "basis_refs": ["runtime/comms/a2a/trust/guest.json"],
                        "execution_authority_granted": false
                    },
                    "invoke_boundary": {
                        "required_entrypoint": "external.execute",
                        "invocation_contract_schema_ref": ACIP_INVOCATION_CONTRACT_SCHEMA_VERSION,
                        "trace_bundle_schema_ref": ACIP_TRACE_BUNDLE_SCHEMA_VERSION,
                        "direct_execution_forbidden": true
                    },
                    "transport_boundary": {
                        "local_scope_only": true,
                        "external_transport_status": "deferred_until_tls_equivalent",
                        "required_security_posture": "tls_or_mutual_tls_equivalent",
                        "refusal_mode": "refuse"
                    },
                    "trace_evidence_refs": [trace_ref],
                    "non_claims": ["External transport remains deferred in v0.91."]
                }),
            },
            AcipA2aNegativeCaseV1 {
                name: "external_transport_without_local_gate".to_string(),
                expected_error_substring:
                    "A2A transport boundary must remain local-only in v0.91".to_string(),
                value: json!({
                    "schema_version": ACIP_A2A_ADAPTER_SCHEMA_VERSION,
                    "adapter_id": "a2a_external_transport",
                    "purpose": "bad",
                    "identity_claim": {
                        "agent_card_ref": agent_card_ref,
                        "agent_id_claim": "external_transport_agent",
                        "display_name_claim": "External Transport Agent",
                        "advertised_capabilities": ["request_consultation"]
                    },
                    "capability_mappings": [{
                        "external_capability_claim": "request_consultation",
                        "adl_capability": "consult",
                        "policy_basis_ref": feature_doc_ref,
                        "invocation_required": true,
                        "direct_execution_forbidden": true
                    }],
                    "trust_classification": {
                        "classification": "naked",
                        "basis_refs": ["runtime/comms/a2a/trust/naked.json"],
                        "execution_authority_granted": false
                    },
                    "invoke_boundary": {
                        "required_entrypoint": "agent.invoke",
                        "invocation_contract_schema_ref": ACIP_INVOCATION_CONTRACT_SCHEMA_VERSION,
                        "trace_bundle_schema_ref": ACIP_TRACE_BUNDLE_SCHEMA_VERSION,
                        "direct_execution_forbidden": true
                    },
                    "transport_boundary": {
                        "local_scope_only": false,
                        "external_transport_status": "deferred_until_tls_equivalent",
                        "required_security_posture": "tls_equivalent",
                        "refusal_mode": "refuse"
                    },
                    "trace_evidence_refs": [trace_ref],
                    "non_claims": ["No direct execution authority is granted from Agent Card claims."]
                }),
            },
        ],
    }
}
