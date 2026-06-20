pub fn acip_coding_invocation_v1_schema_json() -> Result<String> {
    serde_json::to_string_pretty(&schema_for!(AcipCodingInvocationContractV1))
        .context("serialize ACIP coding invocation contract v1 schema")
}

pub fn acip_coding_outcome_v1_schema_json() -> Result<String> {
    serde_json::to_string_pretty(&schema_for!(AcipCodingOutcomeV1))
        .context("serialize ACIP coding outcome v1 schema")
}

pub fn acip_coding_fixture_set_v1_schema_json() -> Result<String> {
    serde_json::to_string_pretty(&schema_for!(AcipCodingFixtureSetV1))
        .context("serialize ACIP coding fixture set v1 schema")
}

pub fn acip_coding_provider_boundary_adapter_v1_schema_json() -> Result<String> {
    serde_json::to_string_pretty(&schema_for!(AcipCodingProviderBoundaryAdapterV1))
        .context("serialize ACIP coding provider boundary adapter v1 schema")
}

pub fn validate_acip_coding_invocation_contract_v1_value(
    value: &JsonValue,
) -> Result<AcipCodingInvocationContractV1> {
    let contract: AcipCodingInvocationContractV1 = serde_json::from_value(value.clone())
        .context("parse ACIP coding invocation contract v1")?;
    validate_acip_coding_invocation_contract_v1(&contract)?;
    Ok(contract)
}

pub fn validate_acip_coding_outcome_v1_value(
    contract: &AcipCodingInvocationContractV1,
    value: &JsonValue,
) -> Result<AcipCodingOutcomeV1> {
    let outcome: AcipCodingOutcomeV1 =
        serde_json::from_value(value.clone()).context("parse ACIP coding outcome v1")?;
    validate_acip_coding_outcome_v1(contract, &outcome)?;
    Ok(outcome)
}

pub fn build_acip_coding_provider_boundary_adapter_v1(
    contract: &AcipCodingInvocationContractV1,
    message: &AcipMessageEnvelopeV1,
) -> Result<AcipCodingProviderBoundaryAdapterV1> {
    validate_acip_coding_invocation_contract_v1(contract)?;
    validate_acip_message_envelope_v1(message)?;
    validate_coding_request_message_matches_contract(contract, message)?;
    validate_provider_eligible_lane(&contract.provider_lane)?;

    let lane_ref = provider_lane_ref_v1(&contract.provider_lane);
    let mut model_identity = model_identity_for_provider_lane(&contract.provider_lane)?;
    let inference_parameter_fingerprint = stable_text_digest_v1(&[
        contract.invocation.invocation_id.as_str(),
        contract.approval_policy.writer_model_ref.as_str(),
        lane_ref,
        contract.task_bundle_ref.as_str(),
        contract.patch_format.as_str(),
    ]);
    model_identity.inference_parameter_fingerprint =
        Some(inference_parameter_fingerprint.clone());
    model_identity.tool_surface = Some("acip_coding_request".to_string());
    model_identity.governance_surface =
        Some(ACIP_CODING_PROVIDER_ADAPTER_SCHEMA_VERSION.to_string());
    model_identity.lane_ref = Some(lane_ref.to_string());

    let provider_request = ProviderInvocationRequestV1 {
        route: route_for_provider_lane(&contract.provider_lane),
        model_identity,
        prompt_contract_ref: contract.task_bundle_ref.clone(),
        lane_ref: lane_ref.to_string(),
        run_id: Some(contract.invocation.conversation_id.clone()),
        request_id: Some(contract.invocation.invocation_id.clone()),
        attempt_policy: attempt_policy_for_provider_lane(&contract.provider_lane),
        input_text: Some(message.content.clone()),
        inference_parameter_fingerprint: Some(inference_parameter_fingerprint),
        tool_surface: Some("acip_coding_request".to_string()),
        governance_surface: Some(ACIP_CODING_PROVIDER_ADAPTER_SCHEMA_VERSION.to_string()),
        evaluator_ref: None,
        benchmark_ref: None,
    };

    let adapter = AcipCodingProviderBoundaryAdapterV1 {
        schema_version: ACIP_CODING_PROVIDER_ADAPTER_SCHEMA_VERSION.to_string(),
        invocation_id: contract.invocation.invocation_id.clone(),
        provider_lane: contract.provider_lane.clone(),
        execution_mode: contract.execution_mode.clone(),
        issue_ref: contract.issue_ref.clone(),
        task_bundle_ref: contract.task_bundle_ref.clone(),
        governed_payload_refs: message.payload_refs.clone(),
        governed_artifact_refs: message.artifact_refs.clone(),
        primary_output_ref: expected_primary_output_ref(contract).to_string(),
        validation_summary_ref: "runtime/comms/coding/validation_summary.json".to_string(),
        review_handoff_ref: "runtime/comms/coding/review_handoff.json".to_string(),
        provider_request,
    };
    validate_acip_coding_provider_boundary_adapter_v1(contract, message, &adapter)?;
    Ok(adapter)
}

pub fn validate_acip_coding_provider_boundary_adapter_v1(
    contract: &AcipCodingInvocationContractV1,
    message: &AcipMessageEnvelopeV1,
    adapter: &AcipCodingProviderBoundaryAdapterV1,
) -> Result<()> {
    validate_acip_coding_invocation_contract_v1(contract)?;
    validate_acip_message_envelope_v1(message)?;
    validate_coding_request_message_matches_contract(contract, message)?;
    validate_provider_eligible_lane(&contract.provider_lane)?;

    if adapter.schema_version != ACIP_CODING_PROVIDER_ADAPTER_SCHEMA_VERSION {
        return Err(anyhow!(
            "ACIP coding provider boundary adapter requires schema_version '{}'",
            ACIP_CODING_PROVIDER_ADAPTER_SCHEMA_VERSION
        ));
    }
    if adapter.invocation_id != contract.invocation.invocation_id {
        return Err(anyhow!(
            "coding provider boundary adapter must preserve invocation_id from the coding contract"
        ));
    }
    if adapter.provider_lane != contract.provider_lane {
        return Err(anyhow!(
            "coding provider boundary adapter must preserve provider_lane from the coding contract"
        ));
    }
    if adapter.execution_mode != contract.execution_mode {
        return Err(anyhow!(
            "coding provider boundary adapter must preserve execution_mode from the coding contract"
        ));
    }
    if adapter.issue_ref != contract.issue_ref {
        return Err(anyhow!(
            "coding provider boundary adapter must preserve issue_ref from the coding contract"
        ));
    }
    if adapter.task_bundle_ref != contract.task_bundle_ref {
        return Err(anyhow!(
            "coding provider boundary adapter must preserve task_bundle_ref from the coding contract"
        ));
    }
    if adapter.governed_payload_refs != message.payload_refs {
        return Err(anyhow!(
            "coding provider boundary adapter must preserve governed payload_refs from the request message"
        ));
    }
    if adapter.governed_artifact_refs != message.artifact_refs {
        return Err(anyhow!(
            "coding provider boundary adapter must preserve governed artifact_refs from the request message"
        ));
    }
    if adapter.governed_payload_refs.is_empty() {
        return Err(anyhow!(
            "coding provider boundary adapter requires governed_payload_refs"
        ));
    }
    for payload in &adapter.governed_payload_refs {
        validate_non_empty(&payload.payload_kind, "governed_payload_refs[].payload_kind")?;
        validate_repo_relative_ref(
            &payload.payload_ref,
            "governed_payload_refs[].payload_ref",
        )?;
        validate_non_empty(&payload.media_type, "governed_payload_refs[].media_type")?;
        validate_sha256(
            &payload.content_sha256,
            "governed_payload_refs[].content_sha256",
        )?;
        if payload.byte_length == 0 {
            return Err(anyhow!(
                "governed_payload_refs[].byte_length must be greater than zero"
            ));
        }
        if let Some(summary) = payload.inline_summary.as_deref() {
            validate_non_empty(summary, "governed_payload_refs[].inline_summary")?;
        }
    }
    for artifact_ref in &adapter.governed_artifact_refs {
        validate_repo_relative_ref(artifact_ref, "governed_artifact_refs[]")?;
    }
    validate_repo_relative_ref(&adapter.primary_output_ref, "primary_output_ref")?;
    if adapter.primary_output_ref != expected_primary_output_ref(contract) {
        return Err(anyhow!(
            "coding provider boundary adapter primary_output_ref must match the declared primary coding output"
        ));
    }
    validate_repo_relative_ref(
        &adapter.validation_summary_ref,
        "validation_summary_ref",
    )?;
    if adapter.validation_summary_ref != "runtime/comms/coding/validation_summary.json" {
        return Err(anyhow!(
            "coding provider boundary adapter must preserve the governed validation_summary output ref"
        ));
    }
    validate_repo_relative_ref(&adapter.review_handoff_ref, "review_handoff_ref")?;
    if adapter.review_handoff_ref != "runtime/comms/coding/review_handoff.json" {
        return Err(anyhow!(
            "coding provider boundary adapter must preserve the governed review_handoff output ref"
        ));
    }

    validate_provider_request(&adapter.provider_request)?;
    if adapter.provider_request.prompt_contract_ref != contract.task_bundle_ref {
        return Err(anyhow!(
            "coding provider boundary adapter must map task_bundle_ref to provider_request.prompt_contract_ref"
        ));
    }
    if adapter.provider_request.request_id.as_deref() != Some(adapter.invocation_id.as_str()) {
        return Err(anyhow!(
            "coding provider boundary adapter must map invocation_id to provider_request.request_id"
        ));
    }
    if adapter.provider_request.run_id.as_deref()
        != Some(contract.invocation.conversation_id.as_str())
    {
        return Err(anyhow!(
            "coding provider boundary adapter must map conversation_id to provider_request.run_id"
        ));
    }
    if adapter.provider_request.input_text.as_deref() != Some(message.content.as_str()) {
        return Err(anyhow!(
            "coding provider boundary adapter must preserve message content as provider_request.input_text"
        ));
    }
    if adapter.provider_request.lane_ref != provider_lane_ref_v1(&contract.provider_lane) {
        return Err(anyhow!(
            "coding provider boundary adapter must lower provider_lane into the provider_request lane_ref"
        ));
    }
    if adapter.provider_request.tool_surface.as_deref() != Some("acip_coding_request") {
        return Err(anyhow!(
            "coding provider boundary adapter must set provider_request.tool_surface to 'acip_coding_request'"
        ));
    }
    if adapter.provider_request.governance_surface.as_deref()
        != Some(ACIP_CODING_PROVIDER_ADAPTER_SCHEMA_VERSION)
    {
        return Err(anyhow!(
            "coding provider boundary adapter must set provider_request.governance_surface to the adapter contract"
        ));
    }
    validate_provider_route_for_lane(&contract.provider_lane, &adapter.provider_request.route)?;
    validate_model_identity_for_lane(
        &contract.provider_lane,
        &adapter.provider_request.lane_ref,
        &adapter.provider_request.model_identity,
    )?;
    if adapter.provider_request.model_identity.tool_surface.as_deref()
        != adapter.provider_request.tool_surface.as_deref()
    {
        return Err(anyhow!(
            "coding provider boundary adapter must preserve tool_surface on model_identity"
        ));
    }
    if adapter.provider_request.model_identity.governance_surface.as_deref()
        != adapter.provider_request.governance_surface.as_deref()
    {
        return Err(anyhow!(
            "coding provider boundary adapter must preserve governance_surface on model_identity"
        ));
    }
    if adapter.provider_request.model_identity.lane_ref.as_deref()
        != Some(adapter.provider_request.lane_ref.as_str())
    {
        return Err(anyhow!(
            "coding provider boundary adapter must preserve lane_ref on model_identity"
        ));
    }
    Ok(())
}

pub fn adapt_provider_result_to_acip_coding_outcome_v1(
    contract: &AcipCodingInvocationContractV1,
    message: &AcipMessageEnvelopeV1,
    adapter: &AcipCodingProviderBoundaryAdapterV1,
    result: &ProviderInvocationResultV1,
) -> Result<AcipCodingOutcomeV1> {
    validate_acip_coding_provider_boundary_adapter_v1(contract, message, adapter)?;
    validate_provider_result(result)?;
    if result.route != adapter.provider_request.route {
        return Err(anyhow!(
            "provider result route must match the adapter provider_request route"
        ));
    }
    if result.model_identity != adapter.provider_request.model_identity {
        return Err(anyhow!(
            "provider result model_identity must match the adapter provider_request model_identity"
        ));
    }
    if result.request_id.as_deref() != adapter.provider_request.request_id.as_deref() {
        return Err(anyhow!(
            "provider result request_id must match the adapter provider_request request_id"
        ));
    }
    if let Some(artifact_ref) = result.artifact_ref.as_deref() {
        validate_repo_relative_ref(artifact_ref, "provider_result.artifact_ref")?;
    }
    if let Some(trace_ref) = result.trace_ref.as_deref() {
        validate_repo_relative_ref(trace_ref, "provider_result.trace_ref")?;
    }
    if result.final_status != ProviderInvocationFinalStatusV1::Ok {
        return Err(anyhow!(
            "provider result must finish with final_status 'ok' before it can become a governed coding outcome"
        ));
    }

    let outcome = AcipCodingOutcomeV1 {
        schema_version: ACIP_CODING_OUTCOME_SCHEMA_VERSION.to_string(),
        invocation_id: adapter.invocation_id.clone(),
        provider_lane: adapter.provider_lane.clone(),
        execution_mode: adapter.execution_mode.clone(),
        disposition: match adapter.execution_mode {
            AcipCodingExecutionModeV1::WorktreeEdit => AcipCodingDispositionV1::PatchReadyForReview,
            AcipCodingExecutionModeV1::UnappliedPatch
            | AcipCodingExecutionModeV1::StructuredProposal => {
                AcipCodingDispositionV1::ProposalReadyForReview
            }
        },
        primary_output_ref: adapter.primary_output_ref.clone(),
        validation_result_refs: vec![adapter.validation_summary_ref.clone()],
        review_handoff_ref: adapter.review_handoff_ref.clone(),
        writer_session_id: contract.approval_policy.writer_session_id.clone(),
        writer_model_ref: contract.approval_policy.writer_model_ref.clone(),
    };
    validate_acip_coding_outcome_v1(contract, &outcome)?;
    Ok(outcome)
}


pub fn validate_acip_coding_invocation_contract_v1(
    contract: &AcipCodingInvocationContractV1,
) -> Result<()> {
    if contract.schema_version != ACIP_CODING_INVOCATION_SCHEMA_VERSION {
        return Err(anyhow!(
            "ACIP coding invocation contract requires schema_version '{}'",
            ACIP_CODING_INVOCATION_SCHEMA_VERSION
        ));
    }
    validate_acip_invocation_contract_v1(&contract.invocation)?;
    if contract.invocation.intent != AcipIntentV1::CodingRequest {
        return Err(anyhow!(
            "ACIP coding invocation specialization requires invocation.intent 'coding_request'"
        ));
    }
    validate_repo_relative_ref(&contract.issue_ref, "issue_ref")?;
    validate_repo_relative_ref(&contract.task_bundle_ref, "task_bundle_ref")?;
    if contract.allowed_edit_paths.is_empty() {
        return Err(anyhow!(
            "ACIP coding invocation specialization requires allowed_edit_paths"
        ));
    }
    for path in &contract.allowed_edit_paths {
        validate_repo_relative_ref(path, "allowed_edit_paths[]")?;
    }
    if contract.validation_commands.is_empty() {
        return Err(anyhow!(
            "ACIP coding invocation specialization requires validation_commands"
        ));
    }
    for command in &contract.validation_commands {
        validate_non_empty(command, "validation_commands[]")?;
    }
    validate_non_empty(&contract.patch_format, "patch_format")?;
    if !contract
        .invocation
        .input_refs
        .iter()
        .any(|reference| reference == &contract.task_bundle_ref)
    {
        return Err(anyhow!(
            "ACIP coding invocation specialization requires invocation.input_refs to include task_bundle_ref"
        ));
    }
    if !contract
        .invocation
        .constraints
        .required_capabilities
        .iter()
        .any(|capability| capability == "share_artifact")
    {
        return Err(anyhow!(
            "ACIP coding invocation specialization requires constraints.required_capabilities to include 'share_artifact'"
        ));
    }
    for action in ["merge", "push", "self_review"] {
        if !contract
            .invocation
            .constraints
            .prohibited_actions
            .iter()
            .any(|prohibited| prohibited == action)
        {
            return Err(anyhow!(
                "ACIP coding invocation specialization requires constraints.prohibited_actions to include '{}'",
                action
            ));
        }
    }
    if contract.invocation.authority_scope.delegation_permitted {
        return Err(anyhow!(
            "ACIP coding invocation specialization must not permit delegation inside the authority_scope"
        ));
    }
    if !contract.invocation.stop_policy.stop_on_refusal {
        return Err(anyhow!(
            "ACIP coding invocation specialization requires stop_policy.stop_on_refusal to be true"
        ));
    }
    if !contract.invocation.stop_policy.stop_on_failure {
        return Err(anyhow!(
            "ACIP coding invocation specialization requires stop_policy.stop_on_failure to be true"
        ));
    }
    if !contract.approval_policy.review_required_before_pr_finish {
        return Err(anyhow!(
            "ACIP coding invocation specialization requires review before PR finish"
        ));
    }
    if contract.approval_policy.required_review_schema_ref != ACIP_REVIEW_INVOCATION_SCHEMA_VERSION
    {
        return Err(anyhow!(
            "ACIP coding invocation specialization must route to review schema '{}'",
            ACIP_REVIEW_INVOCATION_SCHEMA_VERSION
        ));
    }
    validate_id(
        &contract.approval_policy.writer_session_id,
        "approval_policy.writer_session_id",
    )?;
    validate_id(
        &contract.approval_policy.writer_model_ref,
        "approval_policy.writer_model_ref",
    )?;
    if !contract.approval_policy.forbid_same_session_blessing {
        return Err(anyhow!(
            "ACIP coding invocation specialization must forbid same-session blessing"
        ));
    }
    if !contract.approval_policy.forbid_same_model_blessing {
        return Err(anyhow!(
            "ACIP coding invocation specialization must forbid same-model blessing"
        ));
    }

    match contract.provider_lane {
        AcipCodingProviderLaneV1::CodexIssueWorktree => {
            if !contract
                .invocation
                .constraints
                .required_capabilities
                .iter()
                .any(|capability| capability == "code_edit")
            {
                return Err(anyhow!(
                    "codex_issue_worktree lane requires constraints.required_capabilities to include 'code_edit'"
                ));
            }
            if contract.execution_mode != AcipCodingExecutionModeV1::WorktreeEdit {
                return Err(anyhow!(
                    "codex_issue_worktree lane requires execution_mode 'worktree_edit'"
                ));
            }
            if !contract.issue_worktree_required {
                return Err(anyhow!(
                    "codex_issue_worktree lane requires issue_worktree_required to be true"
                ));
            }
        }
        AcipCodingProviderLaneV1::ChatgptApi => {
            if contract.execution_mode == AcipCodingExecutionModeV1::WorktreeEdit {
                return Err(anyhow!(
                    "only codex_issue_worktree lane may use execution_mode 'worktree_edit'"
                ));
            }
            if contract.issue_worktree_required {
                return Err(anyhow!(
                    "proposal-only coding lanes must set issue_worktree_required to false"
                ));
            }
            if contract.execution_mode == AcipCodingExecutionModeV1::StructuredProposal {
                return Err(anyhow!(
                    "chatgpt_api lane does not permit execution_mode 'structured_proposal'"
                ));
            }
            if contract
                .invocation
                .constraints
                .required_capabilities
                .iter()
                .any(|capability| capability == "code_edit")
            {
                return Err(anyhow!(
                    "proposal-only coding lanes must not claim 'code_edit' capability"
                ));
            }
            if !contract
                .invocation
                .constraints
                .required_capabilities
                .iter()
                .any(|capability| capability == "propose_change")
            {
                return Err(anyhow!(
                    "proposal-only coding lanes require constraints.required_capabilities to include 'propose_change'"
                ));
            }
        }
        AcipCodingProviderLaneV1::ClaudeApi
        | AcipCodingProviderLaneV1::LocalOllama
        | AcipCodingProviderLaneV1::OtherProposalOnly => {
            if contract.execution_mode == AcipCodingExecutionModeV1::WorktreeEdit {
                return Err(anyhow!(
                    "only codex_issue_worktree lane may use execution_mode 'worktree_edit'"
                ));
            }
            if contract.issue_worktree_required {
                return Err(anyhow!(
                    "proposal-only coding lanes must set issue_worktree_required to false"
                ));
            }
            if contract
                .invocation
                .constraints
                .required_capabilities
                .iter()
                .any(|capability| capability == "code_edit")
            {
                return Err(anyhow!(
                    "proposal-only coding lanes must not claim 'code_edit' capability"
                ));
            }
            if !contract
                .invocation
                .constraints
                .required_capabilities
                .iter()
                .any(|capability| capability == "propose_change")
            {
                return Err(anyhow!(
                    "proposal-only coding lanes require constraints.required_capabilities to include 'propose_change'"
                ));
            }
        }
    }

    let required_output_kind = match contract.execution_mode {
        AcipCodingExecutionModeV1::WorktreeEdit => {
            if contract.patch_format != "patch_manifest_v1" {
                return Err(anyhow!(
                    "worktree_edit coding invocation requires patch_format 'patch_manifest_v1'"
                ));
            }
            "patch_manifest"
        }
        AcipCodingExecutionModeV1::UnappliedPatch => {
            if contract.patch_format != "unified_diff" {
                return Err(anyhow!(
                    "unapplied_patch coding invocation requires patch_format 'unified_diff'"
                ));
            }
            "patch_diff"
        }
        AcipCodingExecutionModeV1::StructuredProposal => {
            if contract.patch_format != "structured_proposal_v1" {
                return Err(anyhow!(
                    "structured_proposal coding invocation requires patch_format 'structured_proposal_v1'"
                ));
            }
            "structured_proposal"
        }
    };

    if !contract
        .invocation
        .expected_outputs
        .iter()
        .any(|output| output.required && output.output_kind == required_output_kind)
    {
        return Err(anyhow!(
            "coding invocation specialization requires expected_outputs to declare required {} output",
            required_output_kind
        ));
    }
    for output_kind in ["validation_summary", "review_handoff"] {
        if !contract
            .invocation
            .expected_outputs
            .iter()
            .any(|output| output.required && output.output_kind == output_kind)
        {
            return Err(anyhow!(
                "coding invocation specialization requires expected_outputs to declare required {} output",
                output_kind
            ));
        }
    }

    Ok(())
}

fn validate_provider_eligible_lane(provider_lane: &AcipCodingProviderLaneV1) -> Result<()> {
    if *provider_lane == AcipCodingProviderLaneV1::CodexIssueWorktree {
        return Err(anyhow!(
            "coding provider boundary adapter only applies to provider-lower execution lanes, not codex_issue_worktree"
        ));
    }
    Ok(())
}

fn validate_coding_request_message_matches_contract(
    contract: &AcipCodingInvocationContractV1,
    message: &AcipMessageEnvelopeV1,
) -> Result<()> {
    if message.intent != AcipIntentV1::CodingRequest {
        return Err(anyhow!(
            "coding provider boundary adapter requires a coding_request message"
        ));
    }
    if message.conversation_id != contract.invocation.conversation_id {
        return Err(anyhow!(
            "coding provider boundary adapter requires message.conversation_id to match invocation.conversation_id"
        ));
    }
    if message.message_id != contract.invocation.causal_message_id {
        return Err(anyhow!(
            "coding provider boundary adapter requires message.message_id to match invocation.causal_message_id"
        ));
    }
    if !message
        .payload_refs
        .iter()
        .any(|payload| payload.payload_ref == contract.task_bundle_ref)
    {
        return Err(anyhow!(
            "coding provider boundary adapter requires message.payload_refs to include task_bundle_ref"
        ));
    }
    Ok(())
}

fn provider_lane_ref_v1(provider_lane: &AcipCodingProviderLaneV1) -> &'static str {
    match provider_lane {
        AcipCodingProviderLaneV1::CodexIssueWorktree => "acip_coding_codex_issue_worktree",
        AcipCodingProviderLaneV1::ChatgptApi => "acip_coding_chatgpt_api",
        AcipCodingProviderLaneV1::ClaudeApi => "acip_coding_claude_api",
        AcipCodingProviderLaneV1::LocalOllama => "acip_coding_local_ollama",
        AcipCodingProviderLaneV1::OtherProposalOnly => "acip_coding_other_proposal_only",
    }
}

fn route_for_provider_lane(provider_lane: &AcipCodingProviderLaneV1) -> ProviderRouteV1 {
    match provider_lane {
        AcipCodingProviderLaneV1::ChatgptApi => ProviderRouteV1 {
            provider_kind: ProviderKindV1::Hosted,
            provider: "openai".to_string(),
            runtime_surface: RuntimeSurfaceV1::HostedApi,
            provider_model_id: "gpt-5-api".to_string(),
            endpoint_ref: Some("openai.responses".to_string()),
            credential_ref: Some("env:OPENAI_API_KEY".to_string()),
            source_registry: Some("acip.coding.provider_lane".to_string()),
        },
        AcipCodingProviderLaneV1::ClaudeApi => ProviderRouteV1 {
            provider_kind: ProviderKindV1::Hosted,
            provider: "anthropic".to_string(),
            runtime_surface: RuntimeSurfaceV1::HostedApi,
            provider_model_id: "claude-api".to_string(),
            endpoint_ref: Some("anthropic.messages".to_string()),
            credential_ref: Some("env:ANTHROPIC_API_KEY".to_string()),
            source_registry: Some("acip.coding.provider_lane".to_string()),
        },
        AcipCodingProviderLaneV1::LocalOllama => ProviderRouteV1 {
            provider_kind: ProviderKindV1::Local,
            provider: "ollama".to_string(),
            runtime_surface: RuntimeSurfaceV1::OllamaHttp,
            provider_model_id: "gemma3-local".to_string(),
            endpoint_ref: Some("ollama.local".to_string()),
            credential_ref: None,
            source_registry: Some("acip.coding.provider_lane".to_string()),
        },
        AcipCodingProviderLaneV1::OtherProposalOnly => ProviderRouteV1 {
            provider_kind: ProviderKindV1::Unknown,
            provider: "other-provider".to_string(),
            runtime_surface: RuntimeSurfaceV1::Unknown,
            provider_model_id: "other-provider".to_string(),
            endpoint_ref: None,
            credential_ref: None,
            source_registry: Some("acip.coding.provider_lane".to_string()),
        },
        AcipCodingProviderLaneV1::CodexIssueWorktree => unreachable!(),
    }
}

fn model_identity_for_provider_lane(provider_lane: &AcipCodingProviderLaneV1) -> Result<crate::model_identity::ModelIdentityV1> {
    Ok(match provider_lane {
        AcipCodingProviderLaneV1::ChatgptApi => hosted_model_identity(
            "openai",
            "gpt-5-api",
            "gpt-5-api",
            Some("acip.coding.provider_lane".to_string()),
        ),
        AcipCodingProviderLaneV1::ClaudeApi => hosted_model_identity(
            "anthropic",
            "claude-api",
            "claude-api",
            Some("acip.coding.provider_lane".to_string()),
        ),
        AcipCodingProviderLaneV1::LocalOllama => ollama_model_identity(
            "gemma3-local",
            "gemma3-local",
            None,
            Some("acip.coding.provider_lane".to_string()),
        ),
        AcipCodingProviderLaneV1::OtherProposalOnly => {
            let mut identity = hosted_model_identity(
                "other-provider",
                "other-provider",
                "other-provider",
                Some("acip.coding.provider_lane".to_string()),
            );
            identity.provider_kind = "unknown".to_string();
            identity.runtime_surface = "unknown".to_string();
            identity
        }
        AcipCodingProviderLaneV1::CodexIssueWorktree => {
            return Err(anyhow!("codex_issue_worktree is not a provider-lower execution lane"));
        }
    })
}

fn attempt_policy_for_provider_lane(
    provider_lane: &AcipCodingProviderLaneV1,
) -> ProviderAttemptPolicyV1 {
    match provider_lane {
        AcipCodingProviderLaneV1::LocalOllama => ProviderAttemptPolicyV1 {
            max_attempts: 1,
            timeout_ms: 60_000,
            retry_backoff_ms: None,
        },
        AcipCodingProviderLaneV1::ChatgptApi | AcipCodingProviderLaneV1::ClaudeApi => {
            ProviderAttemptPolicyV1 {
                max_attempts: 2,
                timeout_ms: 45_000,
                retry_backoff_ms: Some(1_000),
            }
        }
        AcipCodingProviderLaneV1::OtherProposalOnly => ProviderAttemptPolicyV1 {
            max_attempts: 1,
            timeout_ms: 30_000,
            retry_backoff_ms: None,
        },
        AcipCodingProviderLaneV1::CodexIssueWorktree => unreachable!(),
    }
}

fn validate_provider_route_for_lane(
    provider_lane: &AcipCodingProviderLaneV1,
    route: &ProviderRouteV1,
) -> Result<()> {
    let expected = route_for_provider_lane(provider_lane);
    if route != &expected {
        return Err(anyhow!(
            "coding provider boundary adapter must preserve the expected provider route for the chosen provider_lane"
        ));
    }
    Ok(())
}

fn validate_model_identity_for_lane(
    provider_lane: &AcipCodingProviderLaneV1,
    lane_ref: &str,
    model_identity: &crate::model_identity::ModelIdentityV1,
) -> Result<()> {
    let expected = model_identity_for_provider_lane(provider_lane)?;
    if model_identity.provider != expected.provider
        || model_identity.model_ref != expected.model_ref
        || model_identity.provider_model_id != expected.provider_model_id
        || model_identity.provider_kind != expected.provider_kind
        || model_identity.runtime_surface != expected.runtime_surface
    {
        return Err(anyhow!(
            "coding provider boundary adapter must preserve the expected model_identity for the chosen provider_lane"
        ));
    }
    if model_identity.lane_ref.as_deref() != Some(lane_ref) {
        return Err(anyhow!(
            "coding provider boundary adapter must preserve the lowered lane_ref on model_identity"
        ));
    }
    Ok(())
}

fn expected_primary_output_ref(contract: &AcipCodingInvocationContractV1) -> &'static str {
    match contract.execution_mode {
        AcipCodingExecutionModeV1::WorktreeEdit => "runtime/comms/coding/patch_manifest.json",
        AcipCodingExecutionModeV1::UnappliedPatch => {
            "runtime/comms/coding/proposed_changes.diff"
        }
        AcipCodingExecutionModeV1::StructuredProposal => {
            "runtime/comms/coding/proposal.json"
        }
    }
}

pub fn validate_acip_coding_outcome_v1(
    contract: &AcipCodingInvocationContractV1,
    outcome: &AcipCodingOutcomeV1,
) -> Result<()> {
    validate_acip_coding_invocation_contract_v1(contract)?;
    if outcome.schema_version != ACIP_CODING_OUTCOME_SCHEMA_VERSION {
        return Err(anyhow!(
            "ACIP coding outcome requires schema_version '{}'",
            ACIP_CODING_OUTCOME_SCHEMA_VERSION
        ));
    }
    validate_id(&outcome.invocation_id, "invocation_id")?;
    validate_repo_relative_ref(&outcome.primary_output_ref, "primary_output_ref")?;
    if outcome.validation_result_refs.is_empty() {
        return Err(anyhow!(
            "ACIP coding outcome requires validation_result_refs"
        ));
    }
    for reference in &outcome.validation_result_refs {
        validate_repo_relative_ref(reference, "validation_result_refs[]")?;
    }
    if !outcome
        .validation_result_refs
        .iter()
        .any(|reference| reference.ends_with("validation_summary.json"))
    {
        return Err(anyhow!(
            "coding outcome validation_result_refs must include the declared validation_summary output contract"
        ));
    }
    validate_repo_relative_ref(&outcome.review_handoff_ref, "review_handoff_ref")?;
    validate_id(&outcome.writer_session_id, "writer_session_id")?;
    validate_id(&outcome.writer_model_ref, "writer_model_ref")?;

    if contract.invocation.invocation_id != outcome.invocation_id {
        return Err(anyhow!(
            "coding outcome must match the invocation contract invocation_id"
        ));
    }
    if contract.provider_lane != outcome.provider_lane {
        return Err(anyhow!(
            "coding outcome must preserve the provider_lane declared by the invocation contract"
        ));
    }
    if contract.execution_mode != outcome.execution_mode {
        return Err(anyhow!(
            "coding outcome must preserve the execution_mode declared by the invocation contract"
        ));
    }
    if contract.approval_policy.writer_session_id != outcome.writer_session_id {
        return Err(anyhow!(
            "coding outcome must preserve the writer_session_id declared by the approval policy"
        ));
    }
    if contract.approval_policy.writer_model_ref != outcome.writer_model_ref {
        return Err(anyhow!(
            "coding outcome must preserve the writer_model_ref declared by the approval policy"
        ));
    }
    if !outcome.review_handoff_ref.ends_with("review_handoff.json") {
        return Err(anyhow!(
            "coding outcome review_handoff_ref must match the declared review_handoff output contract"
        ));
    }

    match contract.execution_mode {
        AcipCodingExecutionModeV1::WorktreeEdit => {
            if outcome.disposition != AcipCodingDispositionV1::PatchReadyForReview {
                return Err(anyhow!(
                    "worktree_edit coding outcome must use disposition 'patch_ready_for_review'"
                ));
            }
            if !outcome.primary_output_ref.ends_with("patch_manifest.json") {
                return Err(anyhow!(
                    "coding outcome primary_output_ref must match the declared patch_manifest output contract"
                ));
            }
        }
        AcipCodingExecutionModeV1::UnappliedPatch => {
            if outcome.disposition != AcipCodingDispositionV1::ProposalReadyForReview {
                return Err(anyhow!(
                    "unapplied_patch coding outcome must use disposition 'proposal_ready_for_review'"
                ));
            }
            if !outcome.primary_output_ref.ends_with(".diff") {
                return Err(anyhow!(
                    "coding outcome primary_output_ref must match the declared patch_diff output contract"
                ));
            }
        }
        AcipCodingExecutionModeV1::StructuredProposal => {
            if outcome.disposition != AcipCodingDispositionV1::ProposalReadyForReview {
                return Err(anyhow!(
                    "structured_proposal coding outcome must use disposition 'proposal_ready_for_review'"
                ));
            }
            if !outcome.primary_output_ref.ends_with("proposal.json") {
                return Err(anyhow!(
                    "coding outcome primary_output_ref must match the declared structured_proposal output contract"
                ));
            }
        }
    }

    Ok(())
}

pub fn validate_acip_coding_fixture_set_v1(fixtures: &AcipCodingFixtureSetV1) -> Result<()> {
    if fixtures.schema_version != ACIP_CODING_FIXTURE_SCHEMA_VERSION {
        return Err(anyhow!(
            "ACIP coding fixture set requires schema_version '{}'",
            ACIP_CODING_FIXTURE_SCHEMA_VERSION
        ));
    }
    if fixtures.negative_cases.is_empty() {
        return Err(anyhow!("ACIP coding fixture set requires negative_cases"));
    }
    validate_acip_coding_invocation_contract_v1(&fixtures.valid_codex_contract)?;
    validate_acip_coding_outcome_v1(
        &fixtures.valid_codex_contract,
        &fixtures.valid_codex_outcome,
    )?;
    validate_acip_coding_invocation_contract_v1(&fixtures.valid_patch_contract)?;
    validate_acip_coding_outcome_v1(
        &fixtures.valid_patch_contract,
        &fixtures.valid_patch_outcome,
    )?;
    for case in &fixtures.negative_cases {
        validate_negative_case_name(&case.name, "negative_cases[].name")?;
        validate_non_empty(
            &case.expected_error_substring,
            "negative_cases[].expected_error_substring",
        )?;
        let contract_result = validate_acip_coding_invocation_contract_v1_value(&case.contract);
        match &case.outcome {
            Some(outcome_value) => match contract_result {
                Ok(contract) => validate_negative_result(
                    validate_acip_coding_outcome_v1_value(&contract, outcome_value).map(|_| ()),
                    &case.expected_error_substring,
                )?,
                Err(error) => validate_negative_result(Err(error), &case.expected_error_substring)?,
            },
            None => validate_negative_result(
                contract_result.map(|_| ()),
                &case.expected_error_substring,
            )?,
        }
    }
    Ok(())
}

pub fn acip_coding_fixture_set_v1() -> AcipCodingFixtureSetV1 {
    let valid_codex_contract = sample_coding_invocation_contract(
        AcipCodingProviderLaneV1::CodexIssueWorktree,
        AcipCodingExecutionModeV1::WorktreeEdit,
    );
    let valid_patch_contract = sample_coding_invocation_contract(
        AcipCodingProviderLaneV1::ChatgptApi,
        AcipCodingExecutionModeV1::UnappliedPatch,
    );

    AcipCodingFixtureSetV1 {
        schema_version: ACIP_CODING_FIXTURE_SCHEMA_VERSION.to_string(),
        valid_codex_outcome: sample_coding_outcome(&valid_codex_contract),
        valid_patch_outcome: sample_coding_outcome(&valid_patch_contract),
        negative_cases: vec![
            AcipCodingNegativeCaseV1 {
                name: "non_codex_worktree_edit_rejected".to_string(),
                expected_error_substring:
                    "only codex_issue_worktree lane may use execution_mode 'worktree_edit'"
                        .to_string(),
                contract: serde_json::to_value(sample_coding_invocation_contract(
                    AcipCodingProviderLaneV1::ChatgptApi,
                    AcipCodingExecutionModeV1::WorktreeEdit,
                ))
                .expect("json"),
                outcome: None,
            },
            AcipCodingNegativeCaseV1 {
                name: "proposal_lane_code_edit_capability_rejected".to_string(),
                expected_error_substring:
                    "proposal-only coding lanes must not claim 'code_edit' capability"
                        .to_string(),
                contract: {
                    let mut value = serde_json::to_value(sample_coding_invocation_contract(
                        AcipCodingProviderLaneV1::ChatgptApi,
                        AcipCodingExecutionModeV1::UnappliedPatch,
                    ))
                    .expect("json");
                    value["invocation"]["constraints"]["required_capabilities"] =
                        json!(["code_edit", "share_artifact"]);
                    value
                },
                outcome: None,
            },
            AcipCodingNegativeCaseV1 {
                name: "chatgpt_structured_proposal_rejected".to_string(),
                expected_error_substring:
                    "chatgpt_api lane does not permit execution_mode 'structured_proposal'"
                        .to_string(),
                contract: serde_json::to_value(sample_coding_invocation_contract(
                    AcipCodingProviderLaneV1::ChatgptApi,
                    AcipCodingExecutionModeV1::StructuredProposal,
                ))
                .expect("json"),
                outcome: None,
            },
            AcipCodingNegativeCaseV1 {
                name: "proposal_lane_worktree_flag_rejected".to_string(),
                expected_error_substring:
                    "proposal-only coding lanes must set issue_worktree_required to false"
                        .to_string(),
                contract: {
                    let mut value = serde_json::to_value(sample_coding_invocation_contract(
                        AcipCodingProviderLaneV1::ClaudeApi,
                        AcipCodingExecutionModeV1::StructuredProposal,
                    ))
                    .expect("json");
                    value["issue_worktree_required"] = json!(true);
                    value
                },
                outcome: None,
            },
            AcipCodingNegativeCaseV1 {
                name: "codex_requires_issue_worktree_rejected".to_string(),
                expected_error_substring:
                    "codex_issue_worktree lane requires issue_worktree_required to be true"
                        .to_string(),
                contract: {
                    let mut value = serde_json::to_value(sample_coding_invocation_contract(
                        AcipCodingProviderLaneV1::CodexIssueWorktree,
                        AcipCodingExecutionModeV1::WorktreeEdit,
                    ))
                    .expect("json");
                    value["issue_worktree_required"] = json!(false);
                    value
                },
                outcome: None,
            },
            AcipCodingNegativeCaseV1 {
                name: "missing_task_bundle_input_rejected".to_string(),
                expected_error_substring:
                    "requires invocation.input_refs to include task_bundle_ref".to_string(),
                contract: {
                    let mut value = serde_json::to_value(sample_coding_invocation_contract(
                        AcipCodingProviderLaneV1::CodexIssueWorktree,
                        AcipCodingExecutionModeV1::WorktreeEdit,
                    ))
                    .expect("json");
                    value["invocation"]["input_refs"] = json!([
                        "runtime/comms/coding/issue_context.md",
                        "runtime/comms/coding/current_scope.json"
                    ]);
                    value
                },
                outcome: None,
            },
            AcipCodingNegativeCaseV1 {
                name: "missing_merge_prohibition_rejected".to_string(),
                expected_error_substring:
                    "requires constraints.prohibited_actions to include 'merge'".to_string(),
                contract: {
                    let mut value = serde_json::to_value(sample_coding_invocation_contract(
                        AcipCodingProviderLaneV1::CodexIssueWorktree,
                        AcipCodingExecutionModeV1::WorktreeEdit,
                    ))
                    .expect("json");
                    value["invocation"]["constraints"]["prohibited_actions"] =
                        json!(["push", "self_review"]);
                    value
                },
                outcome: None,
            },
            AcipCodingNegativeCaseV1 {
                name: "review_schema_drift_rejected".to_string(),
                expected_error_substring: "must route to review schema 'acip.review.invocation.v1'"
                    .to_string(),
                contract: {
                    let mut value = serde_json::to_value(sample_coding_invocation_contract(
                        AcipCodingProviderLaneV1::CodexIssueWorktree,
                        AcipCodingExecutionModeV1::WorktreeEdit,
                    ))
                    .expect("json");
                    value["approval_policy"]["required_review_schema_ref"] =
                        json!("acip.review.invocation.v0");
                    value
                },
                outcome: None,
            },
            AcipCodingNegativeCaseV1 {
                name: "outcome_output_contract_mismatch_rejected".to_string(),
                expected_error_substring: "must match the declared patch_diff output contract"
                    .to_string(),
                contract: serde_json::to_value(valid_patch_contract.clone()).expect("json"),
                outcome: Some({
                    let mut value =
                        serde_json::to_value(sample_coding_outcome(&valid_patch_contract))
                            .expect("json");
                    value["primary_output_ref"] = json!("runtime/comms/coding/proposal.json");
                    value
                }),
            },
            AcipCodingNegativeCaseV1 {
                name: "validation_summary_binding_rejected".to_string(),
                expected_error_substring:
                    "validation_result_refs must include the declared validation_summary output contract"
                        .to_string(),
                contract: serde_json::to_value(valid_codex_contract.clone()).expect("json"),
                outcome: Some({
                    let mut value =
                        serde_json::to_value(sample_coding_outcome(&valid_codex_contract))
                            .expect("json");
                    value["validation_result_refs"] =
                        json!(["runtime/comms/coding/some_other_artifact.json"]);
                    value
                }),
            },
            AcipCodingNegativeCaseV1 {
                name: "writer_identity_drift_rejected".to_string(),
                expected_error_substring:
                    "must preserve the writer_session_id declared by the approval policy"
                        .to_string(),
                contract: serde_json::to_value(valid_codex_contract.clone()).expect("json"),
                outcome: Some({
                    let mut value =
                        serde_json::to_value(sample_coding_outcome(&valid_codex_contract))
                            .expect("json");
                    value["writer_session_id"] = json!("someone-else");
                    value
                }),
            },
            AcipCodingNegativeCaseV1 {
                name: "stop_boundary_drift_rejected".to_string(),
                expected_error_substring:
                    "requires stop_policy.stop_on_refusal to be true".to_string(),
                contract: {
                    let mut value = serde_json::to_value(sample_coding_invocation_contract(
                        AcipCodingProviderLaneV1::CodexIssueWorktree,
                        AcipCodingExecutionModeV1::WorktreeEdit,
                    ))
                    .expect("json");
                    value["invocation"]["stop_policy"]["stop_on_refusal"] = json!(false);
                    value
                },
                outcome: None,
            },
        ],
        valid_codex_contract,
        valid_patch_contract,
    }
}


pub(crate) fn sample_coding_invocation_contract(
    provider_lane: AcipCodingProviderLaneV1,
    execution_mode: AcipCodingExecutionModeV1,
) -> AcipCodingInvocationContractV1 {
    let mut invocation = sample_invocation_contract();
    invocation.invocation_id = match execution_mode {
        AcipCodingExecutionModeV1::WorktreeEdit => "invoke-coding-worktree-0001".to_string(),
        AcipCodingExecutionModeV1::UnappliedPatch => "invoke-coding-patch-0001".to_string(),
        AcipCodingExecutionModeV1::StructuredProposal => "invoke-coding-proposal-0001".to_string(),
    };
    invocation.conversation_id = "conv-coding-001".to_string();
    invocation.causal_message_id = "msg-coding-0001".to_string();
    invocation.target.id = "coding.agent".to_string();
    invocation.intent = AcipIntentV1::CodingRequest;
    invocation.purpose =
        "Perform bounded coding work and return a reviewable patch or proposal surface."
            .to_string();
    invocation.input_refs = vec![
        "runtime/comms/coding/task_bundle.json".to_string(),
        "runtime/comms/coding/issue_context.md".to_string(),
        "runtime/comms/coding/current_scope.json".to_string(),
    ];
    invocation.constraints.policy_refs = vec![
        "runtime/comms/coding/workflow_conductor_policy.md".to_string(),
        "runtime/comms/policy/coding_policy.json".to_string(),
    ];
    invocation.constraints.prohibited_actions = vec![
        "merge".to_string(),
        "push".to_string(),
        "self_review".to_string(),
    ];
    invocation.stop_policy.max_turns = 4;
    invocation.stop_policy.max_output_artifacts = 3;
    invocation.stop_policy.completion_condition =
        "emit reviewable coding output and validation evidence".to_string();
    invocation.authority_scope.allowed_actions =
        vec!["invoke".to_string(), "share_artifact".to_string()];
    invocation.authority_scope.authority_basis_refs =
        vec!["runtime/comms/authority/coding_basis.json".to_string()];
    invocation.authority_scope.delegation_permitted = false;
    invocation.decision_event_ref = "gate.coding-0001".to_string();
    invocation.response_channel = AcipResponseChannelV1 {
        kind: AcipResponseChannelKindV1::ArtifactReply,
        channel_ref: "runtime/comms/coding/outcome.json".to_string(),
    };
    invocation.expected_outputs = match execution_mode {
        AcipCodingExecutionModeV1::WorktreeEdit => vec![
            AcipExpectedOutputV1 {
                output_id: "patch_manifest".to_string(),
                output_kind: "patch_manifest".to_string(),
                artifact_role: "worktree_change_manifest".to_string(),
                schema_ref: Some("schemas/coding_patch_manifest.schema.json".to_string()),
                required: true,
            },
            AcipExpectedOutputV1 {
                output_id: "validation_summary".to_string(),
                output_kind: "validation_summary".to_string(),
                artifact_role: "validation_summary".to_string(),
                schema_ref: Some("schemas/validation_summary.schema.json".to_string()),
                required: true,
            },
            AcipExpectedOutputV1 {
                output_id: "review_handoff".to_string(),
                output_kind: "review_handoff".to_string(),
                artifact_role: "review_handoff".to_string(),
                schema_ref: Some("schemas/review_handoff.schema.json".to_string()),
                required: true,
            },
        ],
        AcipCodingExecutionModeV1::UnappliedPatch => vec![
            AcipExpectedOutputV1 {
                output_id: "patch_diff".to_string(),
                output_kind: "patch_diff".to_string(),
                artifact_role: "proposed_patch".to_string(),
                schema_ref: Some("schemas/unified_diff.schema.json".to_string()),
                required: true,
            },
            AcipExpectedOutputV1 {
                output_id: "validation_summary".to_string(),
                output_kind: "validation_summary".to_string(),
                artifact_role: "validation_summary".to_string(),
                schema_ref: Some("schemas/validation_summary.schema.json".to_string()),
                required: true,
            },
            AcipExpectedOutputV1 {
                output_id: "review_handoff".to_string(),
                output_kind: "review_handoff".to_string(),
                artifact_role: "review_handoff".to_string(),
                schema_ref: Some("schemas/review_handoff.schema.json".to_string()),
                required: true,
            },
        ],
        AcipCodingExecutionModeV1::StructuredProposal => vec![
            AcipExpectedOutputV1 {
                output_id: "structured_proposal".to_string(),
                output_kind: "structured_proposal".to_string(),
                artifact_role: "proposed_change_plan".to_string(),
                schema_ref: Some("schemas/coding_proposal.schema.json".to_string()),
                required: true,
            },
            AcipExpectedOutputV1 {
                output_id: "validation_summary".to_string(),
                output_kind: "validation_summary".to_string(),
                artifact_role: "validation_summary".to_string(),
                schema_ref: Some("schemas/validation_summary.schema.json".to_string()),
                required: true,
            },
            AcipExpectedOutputV1 {
                output_id: "review_handoff".to_string(),
                output_kind: "review_handoff".to_string(),
                artifact_role: "review_handoff".to_string(),
                schema_ref: Some("schemas/review_handoff.schema.json".to_string()),
                required: true,
            },
        ],
    };

    let (writer_session_id, writer_model_ref) = match provider_lane {
        AcipCodingProviderLaneV1::CodexIssueWorktree => (
            "writer-session-codex".to_string(),
            "gpt-5-codex".to_string(),
        ),
        AcipCodingProviderLaneV1::ChatgptApi => {
            ("writer-session-openai".to_string(), "gpt-5-api".to_string())
        }
        AcipCodingProviderLaneV1::ClaudeApi => (
            "writer-session-anthropic".to_string(),
            "claude-api".to_string(),
        ),
        AcipCodingProviderLaneV1::LocalOllama => (
            "writer-session-ollama".to_string(),
            "gemma3-local".to_string(),
        ),
        AcipCodingProviderLaneV1::OtherProposalOnly => (
            "writer-session-other".to_string(),
            "other-provider".to_string(),
        ),
    };

    invocation.constraints.required_capabilities = match provider_lane {
        AcipCodingProviderLaneV1::CodexIssueWorktree => {
            vec!["code_edit".to_string(), "share_artifact".to_string()]
        }
        AcipCodingProviderLaneV1::ChatgptApi
        | AcipCodingProviderLaneV1::ClaudeApi
        | AcipCodingProviderLaneV1::LocalOllama
        | AcipCodingProviderLaneV1::OtherProposalOnly => {
            vec!["propose_change".to_string(), "share_artifact".to_string()]
        }
    };

    AcipCodingInvocationContractV1 {
        schema_version: ACIP_CODING_INVOCATION_SCHEMA_VERSION.to_string(),
        invocation,
        provider_lane: provider_lane.clone(),
        execution_mode: execution_mode.clone(),
        issue_ref: "issues/2627".to_string(),
        task_bundle_ref: "runtime/comms/coding/task_bundle.json".to_string(),
        issue_worktree_required: provider_lane == AcipCodingProviderLaneV1::CodexIssueWorktree,
        allowed_edit_paths: vec![
            "adl/src/agent_comms.rs".to_string(),
            "docs/milestones/v0.90.5/features/AGENT_COMMS_v1.md".to_string(),
            "docs/milestones/v0.90.5/features/CODING_AGENT_RUNNER.md".to_string(),
            "docs/milestones/v0.90.5/features/LOCAL_MODEL_PR_REVIEWER_TOOL.md".to_string(),
            "docs/milestones/v0.90.5/features/README.md".to_string(),
            "docs/milestones/v0.90.5/FEATURE_DOCS_v0.90.5.md".to_string(),
        ],
        validation_commands: vec![
            "cargo fmt --check".to_string(),
            "cargo test agent_comms --lib".to_string(),
        ],
        patch_format: match execution_mode {
            AcipCodingExecutionModeV1::WorktreeEdit => "patch_manifest_v1".to_string(),
            AcipCodingExecutionModeV1::UnappliedPatch => "unified_diff".to_string(),
            AcipCodingExecutionModeV1::StructuredProposal => "structured_proposal_v1".to_string(),
        },
        approval_policy: AcipCodingApprovalPolicyV1 {
            review_required_before_pr_finish: true,
            required_review_schema_ref: ACIP_REVIEW_INVOCATION_SCHEMA_VERSION.to_string(),
            writer_session_id,
            writer_model_ref,
            forbid_same_session_blessing: true,
            forbid_same_model_blessing: true,
        },
    }
}

pub(crate) fn sample_coding_outcome(contract: &AcipCodingInvocationContractV1) -> AcipCodingOutcomeV1 {
    let (disposition, primary_output_ref) = match contract.execution_mode {
        AcipCodingExecutionModeV1::WorktreeEdit => (
            AcipCodingDispositionV1::PatchReadyForReview,
            "runtime/comms/coding/patch_manifest.json".to_string(),
        ),
        AcipCodingExecutionModeV1::UnappliedPatch => (
            AcipCodingDispositionV1::ProposalReadyForReview,
            "runtime/comms/coding/proposed_changes.diff".to_string(),
        ),
        AcipCodingExecutionModeV1::StructuredProposal => (
            AcipCodingDispositionV1::ProposalReadyForReview,
            "runtime/comms/coding/proposal.json".to_string(),
        ),
    };

    AcipCodingOutcomeV1 {
        schema_version: ACIP_CODING_OUTCOME_SCHEMA_VERSION.to_string(),
        invocation_id: contract.invocation.invocation_id.clone(),
        provider_lane: contract.provider_lane.clone(),
        execution_mode: contract.execution_mode.clone(),
        disposition,
        primary_output_ref,
        validation_result_refs: vec!["runtime/comms/coding/validation_summary.json".to_string()],
        review_handoff_ref: "runtime/comms/coding/review_handoff.json".to_string(),
        writer_session_id: contract.approval_policy.writer_session_id.clone(),
        writer_model_ref: contract.approval_policy.writer_model_ref.clone(),
    }
}
