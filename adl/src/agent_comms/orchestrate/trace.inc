pub fn acip_trace_bundle_v1_schema_json() -> Result<String> {
    serde_json::to_string_pretty(&schema_for!(AcipTraceBundleV1))
        .context("serialize ACIP trace bundle v1 schema")
}

pub fn acip_trace_fixture_set_v1_schema_json() -> Result<String> {
    serde_json::to_string_pretty(&schema_for!(AcipTraceFixtureSetV1))
        .context("serialize ACIP trace fixture set v1 schema")
}

pub fn validate_acip_trace_bundle_v1_value(value: &JsonValue) -> Result<AcipTraceBundleV1> {
    let bundle: AcipTraceBundleV1 =
        serde_json::from_value(value.clone()).context("parse ACIP trace bundle v1")?;
    validate_acip_trace_bundle_v1(&bundle)?;
    Ok(bundle)
}


pub fn acip_trace_fixture_set_v1() -> AcipTraceFixtureSetV1 {
    AcipTraceFixtureSetV1 {
        schema_version: ACIP_TRACE_FIXTURE_SCHEMA_VERSION.to_string(),
        valid_completed_bundle: sample_trace_bundle(AcipInvocationStatusV1::Completed),
        valid_refused_bundle: sample_trace_bundle(AcipInvocationStatusV1::Refused),
        valid_failed_bundle: sample_trace_bundle(AcipInvocationStatusV1::Failed),
        negative_cases: vec![
            AcipTraceNegativeCaseV1 {
                name: "public_view_private_state_leak_rejected".to_string(),
                expected_error_substring:
                    "reviewer, public, and observatory views must not allow private payload refs"
                        .to_string(),
                bundle: {
                    let mut value = serde_json::to_value(sample_trace_bundle(
                        AcipInvocationStatusV1::Completed,
                    ))
                    .expect("json");
                    value["audience_views"][3]["allows_private_payload_refs"] = json!(true);
                    value
                },
            },
            AcipTraceNegativeCaseV1 {
                name: "missing_decision_event_rejected".to_string(),
                expected_error_substring:
                    "decision_recorded trace event must carry invocation_id, contract_ref, and decision_event_ref"
                        .to_string(),
                bundle: {
                    let mut value = serde_json::to_value(sample_trace_bundle(
                        AcipInvocationStatusV1::Completed,
                    ))
                    .expect("json");
                    value["trace_events"][2]["decision_event_ref"] = JsonValue::Null;
                    value
                },
            },
            AcipTraceNegativeCaseV1 {
                name: "terminal_event_must_require_redaction".to_string(),
                expected_error_substring: "invocation_refused trace event must require redaction"
                    .to_string(),
                bundle: {
                    let mut value =
                        serde_json::to_value(sample_trace_bundle(AcipInvocationStatusV1::Refused))
                            .expect("json");
                    value["trace_events"][3]["requires_redaction"] = json!(false);
                    value
                },
            },
            AcipTraceNegativeCaseV1 {
                name: "host_path_leakage_in_summary_rejected".to_string(),
                expected_error_substring: "summary must not leak protected trace content '/users/'"
                    .to_string(),
                bundle: {
                    let mut value = serde_json::to_value(sample_trace_bundle(
                        AcipInvocationStatusV1::Completed,
                    ))
                    .expect("json");
                    value["trace_events"][0]["summary"] =
                        json!("Captured local path /Users/daniel/private/trace.json for replay.");
                    value
                },
            },
            AcipTraceNegativeCaseV1 {
                name: "remote_replay_dependency_rejected".to_string(),
                expected_error_substring:
                    "ACIP replay contract must remain fixture-backed and local for v1".to_string(),
                bundle: {
                    let mut value = serde_json::to_value(sample_trace_bundle(
                        AcipInvocationStatusV1::Completed,
                    ))
                    .expect("json");
                    value["replay_contract"]["remote_provider_required"] = json!(true);
                    value
                },
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

pub(crate) fn sample_review_invocation_contract() -> AcipReviewInvocationContractV1 {
    let mut invocation = sample_invocation_contract();
    invocation.input_refs = vec![
        "runtime/comms/review/review_packet.json".to_string(),
        "runtime/comms/review/static_analysis_summary.json".to_string(),
        "runtime/comms/review/validation_evidence.json".to_string(),
    ];
    invocation.constraints.policy_refs = vec![
        "runtime/comms/review/srp.md".to_string(),
        "runtime/comms/policy/review_policy.json".to_string(),
    ];
    invocation.constraints.prohibited_actions = vec![
        "merge".to_string(),
        "push".to_string(),
        "destructive_edit".to_string(),
    ];
    invocation.expected_outputs = vec![
        AcipExpectedOutputV1 {
            output_id: "review_result".to_string(),
            output_kind: "review_result".to_string(),
            artifact_role: "primary_review_result".to_string(),
            schema_ref: Some("schemas/pr_review_result.schema.json".to_string()),
            required: true,
        },
        AcipExpectedOutputV1 {
            output_id: "gate_result".to_string(),
            output_kind: "gate_result".to_string(),
            artifact_role: "pr_open_gate".to_string(),
            schema_ref: Some("schemas/pr_review_gate.schema.json".to_string()),
            required: true,
        },
    ];
    invocation.stop_policy.max_output_artifacts = 2;

    AcipReviewInvocationContractV1 {
        schema_version: ACIP_REVIEW_INVOCATION_SCHEMA_VERSION.to_string(),
        invocation,
        srp_ref: "runtime/comms/review/srp.md".to_string(),
        review_packet_ref: "runtime/comms/review/review_packet.json".to_string(),
        evidence_packet_refs: vec![
            "runtime/comms/review/review_packet.json".to_string(),
            "runtime/comms/review/static_analysis_summary.json".to_string(),
            "runtime/comms/review/validation_evidence.json".to_string(),
        ],
        independence_policy: AcipReviewIndependencePolicyV1 {
            writer_session_id: "writer-session-codex".to_string(),
            writer_model_ref: "gpt-5-codex".to_string(),
            reviewer_session_id: "reviewer-session-fixture".to_string(),
            reviewer_model_ref: "fixture-reviewer".to_string(),
            forbid_same_session: true,
            forbid_same_model_ref: true,
        },
        disposition_contract: AcipReviewDispositionContractV1 {
            allowed_dispositions: vec![
                AcipReviewDispositionV1::Blessed,
                AcipReviewDispositionV1::Blocked,
                AcipReviewDispositionV1::NonProving,
                AcipReviewDispositionV1::Skipped,
            ],
            blessed_handoff: AcipReviewHandoffOutcomeV1::AllowPrFinish,
            blocked_handoff: AcipReviewHandoffOutcomeV1::FixFindingsAndRerunReview,
            non_proving_handoff: AcipReviewHandoffOutcomeV1::OperatorWaiverRequired,
            skipped_handoff: AcipReviewHandoffOutcomeV1::OperatorWaiverRequired,
            gate_result_required: true,
            findings_required_when_blocked: true,
        },
    }
}

pub(crate) fn sample_review_outcome(
    contract: &AcipReviewInvocationContractV1,
    disposition: AcipReviewDispositionV1,
    handoff_outcome: AcipReviewHandoffOutcomeV1,
    findings_ref: Option<String>,
    residual_risk_refs: Vec<String>,
    pr_open_allowed: bool,
) -> AcipReviewOutcomeV1 {
    AcipReviewOutcomeV1 {
        schema_version: ACIP_REVIEW_OUTCOME_SCHEMA_VERSION.to_string(),
        invocation_id: contract.invocation.invocation_id.clone(),
        review_result_ref: "runtime/comms/review/review_result.json".to_string(),
        gate_result_ref: "runtime/comms/review/gate_result.json".to_string(),
        disposition,
        handoff_outcome,
        reviewer_session_id: contract.independence_policy.reviewer_session_id.clone(),
        reviewer_model_ref: contract.independence_policy.reviewer_model_ref.clone(),
        findings_ref,
        residual_risk_refs,
        pr_open_allowed,
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

pub(crate) fn sample_trace_bundle(status: AcipInvocationStatusV1) -> AcipTraceBundleV1 {
    let contract = sample_invocation_contract();
    let terminal_event = match status {
        AcipInvocationStatusV1::Completed => AcipTraceEventV1 {
            event_id: "trace-0004".to_string(),
            conversation_id: contract.conversation_id.clone(),
            invocation_id: Some(contract.invocation_id.clone()),
            event_kind: AcipTraceEventKindV1::InvocationCompleted,
            source_message_id: Some(contract.causal_message_id.clone()),
            contract_ref: Some("runtime/comms/invocation/contracts/review_request.json".to_string()),
            decision_event_ref: Some(contract.decision_event_ref.clone()),
            invocation_status: Some(AcipInvocationStatusV1::Completed),
            output_refs: vec!["runtime/comms/invocation/review_report.json".to_string()],
            evidence_refs: vec!["runtime/comms/invocation/evidence/completed_trace.json".to_string()],
            summary: "Invocation completed with declared review output contract satisfied."
                .to_string(),
            requires_redaction: false,
        },
        AcipInvocationStatusV1::Refused => AcipTraceEventV1 {
            event_id: "trace-0004".to_string(),
            conversation_id: contract.conversation_id.clone(),
            invocation_id: Some(contract.invocation_id.clone()),
            event_kind: AcipTraceEventKindV1::InvocationRefused,
            source_message_id: Some(contract.causal_message_id.clone()),
            contract_ref: Some("runtime/comms/invocation/contracts/review_request.json".to_string()),
            decision_event_ref: Some(contract.decision_event_ref.clone()),
            invocation_status: Some(AcipInvocationStatusV1::Refused),
            output_refs: Vec::new(),
            evidence_refs: vec!["runtime/comms/invocation/evidence/refusal_trace.json".to_string()],
            summary: "Invocation refused at the governed boundary with bounded reviewer-visible evidence."
                .to_string(),
            requires_redaction: true,
        },
        AcipInvocationStatusV1::Failed => AcipTraceEventV1 {
            event_id: "trace-0004".to_string(),
            conversation_id: contract.conversation_id.clone(),
            invocation_id: Some(contract.invocation_id.clone()),
            event_kind: AcipTraceEventKindV1::InvocationFailed,
            source_message_id: Some(contract.causal_message_id.clone()),
            contract_ref: Some("runtime/comms/invocation/contracts/review_request.json".to_string()),
            decision_event_ref: Some(contract.decision_event_ref.clone()),
            invocation_status: Some(AcipInvocationStatusV1::Failed),
            output_refs: Vec::new(),
            evidence_refs: vec!["runtime/comms/invocation/evidence/failure_trace.json".to_string()],
            summary: "Invocation failed after decision with bounded failure evidence and no raw payload leak."
                .to_string(),
            requires_redaction: true,
        },
        _ => unreachable!("trace fixture only supports terminal statuses"),
    };

    let visible_artifact_refs = match status {
        AcipInvocationStatusV1::Completed => vec![
            "runtime/comms/trace/reviewer_trace.json".to_string(),
            "runtime/comms/invocation/review_report.json".to_string(),
        ],
        AcipInvocationStatusV1::Refused => vec![
            "runtime/comms/trace/reviewer_trace.json".to_string(),
            "runtime/comms/invocation/evidence/refusal_trace.json".to_string(),
        ],
        AcipInvocationStatusV1::Failed => vec![
            "runtime/comms/trace/reviewer_trace.json".to_string(),
            "runtime/comms/invocation/evidence/failure_trace.json".to_string(),
        ],
        _ => unreachable!("trace fixture only supports terminal statuses"),
    };

    AcipTraceBundleV1 {
        schema_version: ACIP_TRACE_BUNDLE_SCHEMA_VERSION.to_string(),
        conversation_id: contract.conversation_id.clone(),
        trace_events: vec![
            AcipTraceEventV1 {
                event_id: "trace-0001".to_string(),
                conversation_id: contract.conversation_id.clone(),
                invocation_id: None,
                event_kind: AcipTraceEventKindV1::MessageCreated,
                source_message_id: Some(contract.causal_message_id.clone()),
                contract_ref: None,
                decision_event_ref: None,
                invocation_status: None,
                output_refs: Vec::new(),
                evidence_refs: vec!["runtime/comms/trace/message_anchor.json".to_string()],
                summary: "Message created with bounded conversation anchor and trace requirement."
                    .to_string(),
                requires_redaction: false,
            },
            AcipTraceEventV1 {
                event_id: "trace-0002".to_string(),
                conversation_id: contract.conversation_id.clone(),
                invocation_id: Some(contract.invocation_id.clone()),
                event_kind: AcipTraceEventKindV1::InvocationContractDeclared,
                source_message_id: Some(contract.causal_message_id.clone()),
                contract_ref: Some(
                    "runtime/comms/invocation/contracts/review_request.json".to_string(),
                ),
                decision_event_ref: Some(contract.decision_event_ref.clone()),
                invocation_status: None,
                output_refs: Vec::new(),
                evidence_refs: vec!["runtime/comms/trace/contract_anchor.json".to_string()],
                summary:
                    "Invocation contract declared with explicit output, stop, and authority bounds."
                        .to_string(),
                requires_redaction: false,
            },
            AcipTraceEventV1 {
                event_id: "trace-0003".to_string(),
                conversation_id: contract.conversation_id.clone(),
                invocation_id: Some(contract.invocation_id.clone()),
                event_kind: AcipTraceEventKindV1::DecisionRecorded,
                source_message_id: Some(contract.causal_message_id.clone()),
                contract_ref: Some(
                    "runtime/comms/invocation/contracts/review_request.json".to_string(),
                ),
                decision_event_ref: Some(contract.decision_event_ref.clone()),
                invocation_status: None,
                output_refs: Vec::new(),
                evidence_refs: vec!["runtime/comms/trace/gate_result.json".to_string()],
                summary: "Freedom Gate decision recorded before terminal invocation state."
                    .to_string(),
                requires_redaction: false,
            },
            terminal_event,
        ],
        audience_views: vec![
            AcipTraceAudienceViewV1 {
                audience: AcipTraceAudienceV1::Actor,
                narrative_ref: "runtime/comms/trace/actor_view.json".to_string(),
                visible_event_ids: vec![
                    "trace-0001".to_string(),
                    "trace-0002".to_string(),
                    "trace-0003".to_string(),
                    "trace-0004".to_string(),
                ],
                visible_artifact_refs: vec![
                    "runtime/comms/trace/actor_view.json".to_string(),
                    "runtime/comms/trace/private_payload_summary.json".to_string(),
                ],
                redacted_elements: vec!["secret_values".to_string()],
                allows_private_payload_refs: true,
                allows_raw_tool_args: false,
                allows_local_host_paths: false,
                allows_rejected_alternative_details: false,
            },
            AcipTraceAudienceViewV1 {
                audience: AcipTraceAudienceV1::Operator,
                narrative_ref: "runtime/comms/trace/operator_view.json".to_string(),
                visible_event_ids: vec![
                    "trace-0001".to_string(),
                    "trace-0002".to_string(),
                    "trace-0003".to_string(),
                    "trace-0004".to_string(),
                ],
                visible_artifact_refs: vec![
                    "runtime/comms/trace/operator_view.json".to_string(),
                    "runtime/comms/trace/redacted_payload_digest.json".to_string(),
                ],
                redacted_elements: vec![
                    "raw_tool_args".to_string(),
                    "local_host_paths".to_string(),
                ],
                allows_private_payload_refs: true,
                allows_raw_tool_args: false,
                allows_local_host_paths: false,
                allows_rejected_alternative_details: false,
            },
            AcipTraceAudienceViewV1 {
                audience: AcipTraceAudienceV1::Reviewer,
                narrative_ref: "runtime/comms/trace/reviewer_view.json".to_string(),
                visible_event_ids: vec![
                    "trace-0002".to_string(),
                    "trace-0003".to_string(),
                    "trace-0004".to_string(),
                ],
                visible_artifact_refs: visible_artifact_refs.clone(),
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
                narrative_ref: "runtime/comms/trace/public_view.json".to_string(),
                visible_event_ids: vec!["trace-0003".to_string(), "trace-0004".to_string()],
                visible_artifact_refs: vec!["runtime/comms/trace/public_summary.json".to_string()],
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
                narrative_ref: "runtime/comms/trace/observatory_view.json".to_string(),
                visible_event_ids: vec![
                    "trace-0001".to_string(),
                    "trace-0003".to_string(),
                    "trace-0004".to_string(),
                ],
                visible_artifact_refs: vec![
                    "runtime/comms/trace/observatory_summary.json".to_string(),
                    "runtime/comms/trace/redacted_payload_digest.json".to_string(),
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
            fixture_ref: "runtime/comms/fixtures/acip_invocation_fixture_set_v1.json".to_string(),
            fixture_case: match status {
                AcipInvocationStatusV1::Completed => "completed".to_string(),
                AcipInvocationStatusV1::Refused => "refused".to_string(),
                AcipInvocationStatusV1::Failed => "failed".to_string(),
                _ => unreachable!("trace fixture only supports terminal statuses"),
            },
            deterministic_event_order: true,
            deterministic_redaction_views: true,
            remote_provider_required: false,
        },
        evidence_packet_refs: vec![
            "runtime/comms/trace/message_anchor.json".to_string(),
            "runtime/comms/trace/gate_result.json".to_string(),
            match status {
                AcipInvocationStatusV1::Completed => {
                    "runtime/comms/invocation/evidence/completed_trace.json".to_string()
                }
                AcipInvocationStatusV1::Refused => {
                    "runtime/comms/invocation/evidence/refusal_trace.json".to_string()
                }
                AcipInvocationStatusV1::Failed => {
                    "runtime/comms/invocation/evidence/failure_trace.json".to_string()
                }
                _ => unreachable!("trace fixture only supports terminal statuses"),
            },
        ],
    }
}

fn validate_acip_trace_event_v1(
    event: &AcipTraceEventV1,
    expected_conversation_id: &str,
) -> Result<()> {
    validate_id(&event.event_id, "event_id")?;
    if event.conversation_id != expected_conversation_id {
        return Err(anyhow!(
            "trace event conversation_id must match the bundle conversation_id"
        ));
    }
    if let Some(invocation_id) = &event.invocation_id {
        validate_id(invocation_id, "invocation_id")?;
    }
    if let Some(source_message_id) = &event.source_message_id {
        validate_id(source_message_id, "source_message_id")?;
    }
    if let Some(contract_ref) = &event.contract_ref {
        validate_repo_relative_ref(contract_ref, "contract_ref")?;
    }
    if let Some(decision_event_ref) = &event.decision_event_ref {
        validate_gate_decision_ref(decision_event_ref, "decision_event_ref")?;
    }
    if event.output_refs.len() > MAX_LIST_LEN {
        return Err(anyhow!("output_refs exceeds bounded list length"));
    }
    for reference in &event.output_refs {
        validate_repo_relative_ref(reference, "output_refs[]")?;
    }
    if event.evidence_refs.len() > MAX_LIST_LEN {
        return Err(anyhow!("evidence_refs exceeds bounded list length"));
    }
    for reference in &event.evidence_refs {
        validate_repo_relative_ref(reference, "evidence_refs[]")?;
    }
    validate_non_empty(&event.summary, "summary")?;
    if event.summary.chars().count() > MAX_INLINE_SUMMARY_CHARS {
        return Err(anyhow!(
            "summary exceeds bounded inline posture of {MAX_INLINE_SUMMARY_CHARS} characters"
        ));
    }
    ensure_safe_trace_summary(&event.summary, "summary")?;

    match event.event_kind {
        AcipTraceEventKindV1::MessageCreated => {
            if event.source_message_id.is_none() {
                return Err(anyhow!(
                    "message_created trace event must carry source_message_id"
                ));
            }
            if event.invocation_id.is_some()
                || event.contract_ref.is_some()
                || event.decision_event_ref.is_some()
                || event.invocation_status.is_some()
            {
                return Err(anyhow!(
                    "message_created trace event must not carry post-message invocation fields"
                ));
            }
        }
        AcipTraceEventKindV1::InvocationContractDeclared => {
            if event.invocation_id.is_none() || event.contract_ref.is_none() {
                return Err(anyhow!(
                    "invocation_contract_declared trace event must carry invocation_id and contract_ref"
                ));
            }
            if event.decision_event_ref.is_none() {
                return Err(anyhow!(
                    "invocation_contract_declared trace event must carry decision_event_ref"
                ));
            }
        }
        AcipTraceEventKindV1::DecisionRecorded => {
            if event.invocation_id.is_none()
                || event.contract_ref.is_none()
                || event.decision_event_ref.is_none()
            {
                return Err(anyhow!(
                    "decision_recorded trace event must carry invocation_id, contract_ref, and decision_event_ref"
                ));
            }
        }
        AcipTraceEventKindV1::InvocationCompleted => {
            if event.invocation_status != Some(AcipInvocationStatusV1::Completed) {
                return Err(anyhow!(
                    "invocation_completed trace event must carry invocation_status 'completed'"
                ));
            }
            if event.invocation_id.is_none()
                || event.contract_ref.is_none()
                || event.output_refs.is_empty()
                || event.decision_event_ref.is_none()
            {
                return Err(anyhow!(
                    "invocation_completed trace event must carry invocation_id, contract_ref, decision_event_ref, and output_refs"
                ));
            }
        }
        AcipTraceEventKindV1::InvocationRefused => {
            if event.invocation_status != Some(AcipInvocationStatusV1::Refused) {
                return Err(anyhow!(
                    "invocation_refused trace event must carry invocation_status 'refused'"
                ));
            }
            if event.invocation_id.is_none()
                || event.contract_ref.is_none()
                || event.evidence_refs.is_empty()
                || event.decision_event_ref.is_none()
            {
                return Err(anyhow!(
                    "invocation_refused trace event must carry invocation_id, contract_ref, decision_event_ref, and evidence_refs"
                ));
            }
            if !event.output_refs.is_empty() {
                return Err(anyhow!(
                    "invocation_refused trace event must not carry output_refs"
                ));
            }
            if !event.requires_redaction {
                return Err(anyhow!(
                    "invocation_refused trace event must require redaction"
                ));
            }
        }
        AcipTraceEventKindV1::InvocationFailed => {
            if event.invocation_status != Some(AcipInvocationStatusV1::Failed) {
                return Err(anyhow!(
                    "invocation_failed trace event must carry invocation_status 'failed'"
                ));
            }
            if event.invocation_id.is_none()
                || event.contract_ref.is_none()
                || event.evidence_refs.is_empty()
                || event.decision_event_ref.is_none()
            {
                return Err(anyhow!(
                    "invocation_failed trace event must carry invocation_id, contract_ref, decision_event_ref, and evidence_refs"
                ));
            }
            if !event.output_refs.is_empty() {
                return Err(anyhow!(
                    "invocation_failed trace event must not carry output_refs"
                ));
            }
            if !event.requires_redaction {
                return Err(anyhow!(
                    "invocation_failed trace event must require redaction"
                ));
            }
        }
    }

    Ok(())
}

fn validate_acip_trace_audience_view_v1(
    view: &AcipTraceAudienceViewV1,
    known_event_ids: &BTreeSet<String>,
) -> Result<()> {
    validate_repo_relative_ref(&view.narrative_ref, "narrative_ref")?;
    if view.visible_event_ids.is_empty() {
        return Err(anyhow!("trace audience view must carry visible_event_ids"));
    }
    for event_id in &view.visible_event_ids {
        validate_id(event_id, "visible_event_ids[]")?;
        if !known_event_ids.contains(event_id) {
            return Err(anyhow!(
                "trace audience view references unknown event_id '{}'",
                event_id
            ));
        }
    }
    for reference in &view.visible_artifact_refs {
        validate_repo_relative_ref(reference, "visible_artifact_refs[]")?;
        if matches!(
            view.audience,
            AcipTraceAudienceV1::Reviewer
                | AcipTraceAudienceV1::Public
                | AcipTraceAudienceV1::Observatory
        ) {
            ensure_redacted_trace_ref(reference, "visible_artifact_refs[]")?;
        }
    }
    if view.redacted_elements.is_empty() {
        return Err(anyhow!(
            "trace audience view must declare redacted_elements"
        ));
    }
    for element in &view.redacted_elements {
        validate_id(element, "redacted_elements[]")?;
    }
    match view.audience {
        AcipTraceAudienceV1::Actor | AcipTraceAudienceV1::Operator => {}
        AcipTraceAudienceV1::Reviewer
        | AcipTraceAudienceV1::Public
        | AcipTraceAudienceV1::Observatory => {
            ensure_redacted_trace_ref(&view.narrative_ref, "narrative_ref")?;
            if view.allows_private_payload_refs {
                return Err(anyhow!(
                    "reviewer, public, and observatory views must not allow private payload refs"
                ));
            }
            if view.allows_raw_tool_args {
                return Err(anyhow!(
                    "reviewer, public, and observatory views must not allow raw tool args"
                ));
            }
            if view.allows_local_host_paths {
                return Err(anyhow!(
                    "reviewer, public, and observatory views must not allow local host paths"
                ));
            }
            if view.allows_rejected_alternative_details {
                return Err(anyhow!(
                    "reviewer, public, and observatory views must not allow rejected alternative details"
                ));
            }
        }
    }
    Ok(())
}

fn validate_acip_replay_contract_v1(contract: &AcipReplayContractV1) -> Result<()> {
    validate_repo_relative_ref(&contract.fixture_ref, "fixture_ref")?;
    validate_id(&contract.fixture_case, "fixture_case")?;
    if !contract.deterministic_event_order {
        return Err(anyhow!(
            "ACIP replay contract requires deterministic_event_order"
        ));
    }
    if !contract.deterministic_redaction_views {
        return Err(anyhow!(
            "ACIP replay contract requires deterministic_redaction_views"
        ));
    }
    if contract.remote_provider_required {
        return Err(anyhow!(
            "ACIP replay contract must remain fixture-backed and local for v1"
        ));
    }
    Ok(())
}

fn ensure_safe_trace_summary(value: &str, field: &str) -> Result<()> {
    let lowered = value.to_ascii_lowercase();
    for forbidden in [
        "secret",
        "token",
        "password",
        "prompt",
        "tool_args",
        "raw tool arguments",
        "raw tool args",
        "private_state",
        "private state",
        "rejected_alternative",
        "rejected alternative",
        "rejected-alternative",
        "alternatives rejected",
        "/users/",
        "/home/",
        "/tmp/",
        "/var/folders/",
        "c:\\",
    ] {
        if lowered.contains(forbidden) {
            return Err(anyhow!(
                "{field} must not leak protected trace content '{}'",
                forbidden
            ));
        }
    }
    Ok(())
}

fn ensure_redacted_trace_ref(value: &str, field: &str) -> Result<()> {
    let lowered = value.to_ascii_lowercase();
    for forbidden in [
        "private_state",
        "raw_args",
        "raw_result",
        "prompt",
        "secret",
        "rejected_alternative",
        "rejected alternative",
        "rejected-alternative",
        "alternatives rejected",
    ] {
        if lowered.contains(forbidden) {
            return Err(anyhow!(
                "{field} must not expose unredacted trace ref '{}'",
                forbidden
            ));
        }
    }
    Ok(())
}

fn trace_event_kind_str(kind: &AcipTraceEventKindV1) -> &'static str {
    match kind {
        AcipTraceEventKindV1::MessageCreated => "message_created",
        AcipTraceEventKindV1::InvocationContractDeclared => "invocation_contract_declared",
        AcipTraceEventKindV1::DecisionRecorded => "decision_recorded",
        AcipTraceEventKindV1::InvocationCompleted => "invocation_completed",
        AcipTraceEventKindV1::InvocationRefused => "invocation_refused",
        AcipTraceEventKindV1::InvocationFailed => "invocation_failed",
    }
}

fn trace_audience_str(audience: &AcipTraceAudienceV1) -> &'static str {
    match audience {
        AcipTraceAudienceV1::Actor => "actor",
        AcipTraceAudienceV1::Operator => "operator",
        AcipTraceAudienceV1::Reviewer => "reviewer",
        AcipTraceAudienceV1::Public => "public",
        AcipTraceAudienceV1::Observatory => "observatory",
    }
}

pub fn validate_acip_trace_bundle_v1(bundle: &AcipTraceBundleV1) -> Result<()> {
    if bundle.schema_version != ACIP_TRACE_BUNDLE_SCHEMA_VERSION {
        return Err(anyhow!(
            "ACIP trace bundle requires schema_version '{}'",
            ACIP_TRACE_BUNDLE_SCHEMA_VERSION
        ));
    }
    validate_id(&bundle.conversation_id, "conversation_id")?;
    if bundle.trace_events.is_empty() {
        return Err(anyhow!("ACIP trace bundle requires trace_events"));
    }
    if bundle.trace_events.len() > MAX_LIST_LEN {
        return Err(anyhow!("trace_events exceeds bounded list length"));
    }
    let mut seen_event_ids = BTreeSet::new();
    let mut seen_event_kinds = BTreeSet::new();
    let mut terminal_events = 0_usize;
    let mut canonical_invocation_id: Option<&str> = None;
    let mut canonical_contract_ref: Option<&str> = None;
    let mut canonical_decision_ref: Option<&str> = None;
    for (index, event) in bundle.trace_events.iter().enumerate() {
        validate_acip_trace_event_v1(event, &bundle.conversation_id)?;
        if !seen_event_ids.insert(event.event_id.clone()) {
            return Err(anyhow!(
                "trace bundle contains duplicate event_id '{}'",
                event.event_id
            ));
        }
        if !seen_event_kinds.insert(event.event_kind.clone()) {
            return Err(anyhow!(
                "trace bundle must not contain duplicate event kind '{}'",
                trace_event_kind_str(&event.event_kind)
            ));
        }
        match index {
            0 if event.event_kind != AcipTraceEventKindV1::MessageCreated => {
                return Err(anyhow!(
                    "trace bundle must preserve canonical event order: message_created first"
                ))
            }
            1 if event.event_kind != AcipTraceEventKindV1::InvocationContractDeclared => {
                return Err(anyhow!(
                    "trace bundle must preserve canonical event order: invocation_contract_declared second"
                ))
            }
            2 if event.event_kind != AcipTraceEventKindV1::DecisionRecorded => {
                return Err(anyhow!(
                    "trace bundle must preserve canonical event order: decision_recorded third"
                ))
            }
            _ => {}
        }
        if !matches!(event.event_kind, AcipTraceEventKindV1::MessageCreated) {
            let Some(invocation_id) = event.invocation_id.as_deref() else {
                return Err(anyhow!(
                    "trace bundle non-message events must carry invocation_id"
                ));
            };
            let Some(contract_ref) = event.contract_ref.as_deref() else {
                return Err(anyhow!(
                    "trace bundle non-message events must carry contract_ref"
                ));
            };
            let Some(decision_ref) = event.decision_event_ref.as_deref() else {
                return Err(anyhow!(
                    "trace bundle non-message events must carry decision_event_ref"
                ));
            };
            if let Some(expected) = canonical_invocation_id {
                if expected != invocation_id {
                    return Err(anyhow!(
                        "trace bundle must preserve one canonical invocation_id across non-message events"
                    ));
                }
            } else {
                canonical_invocation_id = Some(invocation_id);
            }
            if let Some(expected) = canonical_contract_ref {
                if expected != contract_ref {
                    return Err(anyhow!(
                        "trace bundle must preserve one canonical contract_ref across non-message events"
                    ));
                }
            } else {
                canonical_contract_ref = Some(contract_ref);
            }
            if let Some(expected) = canonical_decision_ref {
                if expected != decision_ref {
                    return Err(anyhow!(
                        "trace bundle must preserve one canonical decision_event_ref across non-message events"
                    ));
                }
            } else {
                canonical_decision_ref = Some(decision_ref);
            }
        }
        match event.event_kind {
            AcipTraceEventKindV1::InvocationCompleted
            | AcipTraceEventKindV1::InvocationRefused
            | AcipTraceEventKindV1::InvocationFailed => {
                terminal_events += 1;
            }
            _ => {}
        }
    }
    for required in [
        AcipTraceEventKindV1::MessageCreated,
        AcipTraceEventKindV1::InvocationContractDeclared,
        AcipTraceEventKindV1::DecisionRecorded,
    ] {
        if !seen_event_kinds.contains(&required) {
            return Err(anyhow!(
                "trace bundle missing required event kind '{}'",
                trace_event_kind_str(&required)
            ));
        }
    }
    if terminal_events != 1 {
        return Err(anyhow!(
            "trace bundle must contain exactly one terminal event kind"
        ));
    }
    let last_event = bundle
        .trace_events
        .last()
        .expect("trace bundle checked non-empty above");
    if !matches!(
        last_event.event_kind,
        AcipTraceEventKindV1::InvocationCompleted
            | AcipTraceEventKindV1::InvocationRefused
            | AcipTraceEventKindV1::InvocationFailed
    ) {
        return Err(anyhow!(
            "trace bundle must preserve canonical event order: terminal invocation event last"
        ));
    }
    validate_acip_replay_contract_v1(&bundle.replay_contract)?;
    if bundle.evidence_packet_refs.is_empty() {
        return Err(anyhow!("ACIP trace bundle requires evidence_packet_refs"));
    }
    for reference in &bundle.evidence_packet_refs {
        validate_repo_relative_ref(reference, "evidence_packet_refs[]")?;
    }
    if bundle.audience_views.len() != 5 {
        return Err(anyhow!(
            "ACIP trace bundle requires exactly five canonical audience_views"
        ));
    }
    let mut seen_audiences = BTreeSet::new();
    for view in &bundle.audience_views {
        validate_acip_trace_audience_view_v1(view, &seen_event_ids)?;
        if !seen_audiences.insert(view.audience.clone()) {
            return Err(anyhow!(
                "trace bundle contains duplicate audience view '{}'",
                trace_audience_str(&view.audience)
            ));
        }
    }
    for required in [
        AcipTraceAudienceV1::Actor,
        AcipTraceAudienceV1::Operator,
        AcipTraceAudienceV1::Reviewer,
        AcipTraceAudienceV1::Public,
        AcipTraceAudienceV1::Observatory,
    ] {
        if !seen_audiences.contains(&required) {
            return Err(anyhow!(
                "trace bundle missing required audience view '{}'",
                trace_audience_str(&required)
            ));
        }
    }
    Ok(())
}

pub fn validate_acip_trace_fixture_set_v1(fixtures: &AcipTraceFixtureSetV1) -> Result<()> {
    if fixtures.schema_version != ACIP_TRACE_FIXTURE_SCHEMA_VERSION {
        return Err(anyhow!(
            "ACIP trace fixture set requires schema_version '{}'",
            ACIP_TRACE_FIXTURE_SCHEMA_VERSION
        ));
    }
    if fixtures.negative_cases.is_empty() {
        return Err(anyhow!("ACIP trace fixture set requires negative_cases"));
    }
    validate_acip_trace_bundle_v1(&fixtures.valid_completed_bundle)?;
    validate_acip_trace_bundle_v1(&fixtures.valid_refused_bundle)?;
    validate_acip_trace_bundle_v1(&fixtures.valid_failed_bundle)?;
    for case in &fixtures.negative_cases {
        validate_negative_case_name(&case.name, "negative_cases[].name")?;
        validate_non_empty(
            &case.expected_error_substring,
            "negative_cases[].expected_error_substring",
        )?;
        validate_negative_result(
            validate_acip_trace_bundle_v1_value(&case.bundle).map(|_| ()),
            &case.expected_error_substring,
        )?;
    }
    Ok(())
}
