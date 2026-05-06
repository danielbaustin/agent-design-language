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
                "negotiation",
                "coding_agent_handoff",
                "operator_request",
                "broadcast"
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
                "identity_drift",
                "missing_recipient",
                "hidden_invocation",
                "malformed_payload_refs",
                "unsupported_visibility",
                "raw_local_path_refs",
                "authority_escalation"
            ]
        );
    }

    #[test]
    fn agent_comms_root_helpers_validate_gate_and_repo_relative_refs() {
        validate_gate_decision_ref("gate.review-approved", "decision_event_ref")
            .expect("gate token should validate");
        validate_repo_relative_ref(
            "runtime/comms/trace/review_packet.json",
            "artifact_ref",
        )
        .expect("repo-relative path should validate");

        let gate_error = validate_gate_decision_ref("review-approved", "decision_event_ref")
            .expect_err("missing gate prefix should fail");
        assert!(gate_error
            .to_string()
            .contains("must link to a Freedom Gate decision token"));

        let path_error = validate_repo_relative_ref("/tmp/leak.json", "artifact_ref")
            .expect_err("absolute path should fail");
        assert!(path_error
            .to_string()
            .contains("must be a repository-relative path"));
    }

    #[test]
    fn agent_comms_root_helpers_cover_negative_result_and_intent_strings() {
        assert_eq!(AcipIntentV1::Conversation.as_str(), "conversation");
        assert_eq!(AcipIntentV1::Delegation.as_str(), "delegation");

        validate_negative_result(
            Err(anyhow!("stable test failure: expected substring")),
            "expected substring",
        )
        .expect("matching negative result should validate");

        let mismatch = validate_negative_result(Ok(()), "expected substring")
            .expect_err("unexpected success should fail");
        assert!(mismatch
            .to_string()
            .contains("negative case unexpectedly validated"));
    }

    #[test]
    fn agent_comms_root_helpers_cover_identifier_and_format_guards() {
        validate_negative_case_name("stable_case_id", "negative_cases[].name")
            .expect("stable case name should validate");
        validate_id("artifact_ref", "field").expect("stable id should validate");
        validate_non_empty("non-empty", "field").expect("non-empty value should validate");
        validate_timestamp("2026-04-29T06:17:13Z", "timestamp").expect("utc timestamp");
        validate_sha256(
            "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
            "sha",
        )
        .expect("sha256 should validate");

        assert!(validate_id("../bad", "field")
            .expect_err("path-like identifier should fail")
            .to_string()
            .contains("must be a stable identifier"));
        assert!(validate_non_empty("   ", "field")
            .expect_err("blank value should fail")
            .to_string()
            .contains("must not be empty"));
        assert!(validate_timestamp("2026-04-29T06:17:13+01:00", "timestamp")
            .expect_err("non-utc timestamp should fail")
            .to_string()
            .contains("must be a UTC timestamp ending in Z"));
        assert!(validate_sha256("abc", "sha")
            .expect_err("short hash should fail")
            .to_string()
            .contains("64-character hexadecimal sha256"));
        assert!(validate_gate_decision_ref("gate:", "decision_event_ref")
            .expect_err("missing decision id should fail")
            .to_string()
            .contains("must include a Freedom Gate decision identifier"));
        assert!(validate_gate_decision_ref("gate.bad token", "decision_event_ref")
            .expect_err("invalid gate token should fail")
            .to_string()
            .contains("must use a stable Freedom Gate decision token"));
        assert!(validate_repo_relative_ref("runtime/../escape.json", "artifact_ref")
            .expect_err("path traversal should fail")
            .to_string()
            .contains("without traversal"));
        assert!(validate_timestamp("not-a-timestamp", "timestamp")
            .expect_err("malformed timestamp should fail")
            .to_string()
            .contains("RFC3339-style UTC timestamp"));
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
    fn acip_conformance_report_schema_and_report_are_available() {
        let schema = acip_conformance_report_v1_schema_json().expect("conformance schema");
        assert!(schema.contains("\"valid_fixture_classes\""));
        assert!(schema.contains("\"negative_fixture_classes\""));

        let report = acip_conformance_report_v1();
        validate_acip_conformance_report_v1(&report).expect("conformance report should validate");
        assert!(report
            .valid_fixture_classes
            .iter()
            .any(|class| class.fixture_name == "coding_agent_handoff"));
        assert!(report
            .valid_fixture_classes
            .iter()
            .any(|class| class.fixture_name == "shared_conversation_thread"));
        assert!(report
            .valid_fixture_classes
            .iter()
            .any(|class| class.fixture_name == "governed_invocation_contract"));
        assert!(report
            .negative_fixture_classes
            .iter()
            .any(|class| class.case_name == "missing_gate_rejects_governed_invocation"));
        assert!(report
            .negative_fixture_classes
            .iter()
            .any(|class| class.case_name == "ambiguous_stop_policy_rejected"));
        assert!(report
            .negative_fixture_classes
            .iter()
            .any(|class| class.case_name == "unsafe_input_refs_rejected"));
        assert!(report
            .negative_fixture_classes
            .iter()
            .any(|class| class.case_name == "status_refusal_inconsistency_rejected"));
        assert!(report
            .negative_fixture_classes
            .iter()
            .any(|class| class.case_name == "output_contract_mismatch_rejected"));
    }

    #[test]
    fn acip_conformance_report_requires_full_cross_surface_matrix() {
        let mut report = acip_conformance_report_v1();
        report
            .valid_fixture_classes
            .retain(|class| class.fixture_name != "governed_invocation_contract");
        let valid_error = validate_acip_conformance_report_v1(&report)
            .expect_err("missing invocation proof fixture should fail");
        assert!(valid_error
            .to_string()
            .contains("missing required valid fixture 'governed_invocation_contract'"));

        let mut report = acip_conformance_report_v1();
        report
            .negative_fixture_classes
            .retain(|class| class.case_name != "output_contract_mismatch_rejected");
        let negative_error = validate_acip_conformance_report_v1(&report)
            .expect_err("missing invocation negative proof should fail");
        assert!(negative_error
            .to_string()
            .contains("missing required negative fixture 'output_contract_mismatch_rejected'"));
    }

    #[test]
    fn acip_a2a_adapter_schemas_and_fixtures_are_available() {
        let adapter_schema = acip_a2a_adapter_boundary_v1_schema_json().expect("adapter schema");
        let fixture_schema = acip_a2a_fixture_set_v1_schema_json().expect("fixture schema");
        assert!(adapter_schema.contains("\"trust_classification\""));
        assert!(adapter_schema.contains("\"required_entrypoint\""));
        assert!(fixture_schema.contains("\"valid_adapters\""));
        assert!(fixture_schema.contains("\"negative_cases\""));

        let fixtures = acip_a2a_fixture_set_v1();
        validate_acip_a2a_fixture_set_v1(&fixtures).expect("A2A fixtures should validate");
        assert_eq!(fixtures.valid_adapters.len(), 3);
        assert_eq!(fixtures.negative_cases.len(), 4);
    }

    #[test]
    fn acip_a2a_adapter_requires_all_trust_classifications() {
        let mut fixtures = acip_a2a_fixture_set_v1();
        fixtures
            .valid_adapters
            .retain(|adapter| adapter.trust_classification.classification != AcipA2aTrustClassV1::Citizen);
        let error = validate_acip_a2a_fixture_set_v1(&fixtures)
            .expect_err("missing trust classification coverage should fail");
        assert!(error
            .to_string()
            .contains("ACIP A2A fixture set must cover trust class"));
    }

    #[test]
    fn acip_a2a_adapter_rejects_parallel_authority_and_direct_execution() {
        let fixtures = acip_a2a_fixture_set_v1();
        for case_name in ["parallel_authority_model", "direct_execution_grant"] {
            let case = fixtures
                .negative_cases
                .iter()
                .find(|case| case.name == case_name)
                .expect("negative case");
            let error = validate_acip_a2a_adapter_boundary_v1_value(&case.value)
                .expect_err("negative case should fail");
            assert!(error.to_string().contains(&case.expected_error_substring));
        }
    }

    #[test]
    fn acip_a2a_adapter_rejects_missing_agent_invoke_and_external_transport() {
        let fixtures = acip_a2a_fixture_set_v1();
        for case_name in [
            "missing_agent_invoke_boundary",
            "external_transport_without_local_gate",
        ] {
            let case = fixtures
                .negative_cases
                .iter()
                .find(|case| case.name == case_name)
                .expect("negative case");
            let error = validate_acip_a2a_adapter_boundary_v1_value(&case.value)
                .expect_err("negative case should fail");
            assert!(error.to_string().contains(&case.expected_error_substring));
        }
    }

    #[test]
    fn acip_review_specialization_schemas_and_fixtures_are_available() {
        let contract_schema = acip_review_invocation_v1_schema_json().expect("review schema");
        let outcome_schema = acip_review_outcome_v1_schema_json().expect("outcome schema");
        let fixture_schema = acip_review_fixture_set_v1_schema_json().expect("fixture schema");
        assert!(contract_schema.contains("\"srp_ref\""));
        assert!(outcome_schema.contains("\"handoff_outcome\""));
        assert!(fixture_schema.contains("\"valid_blessed_outcome\""));

        let fixtures = acip_review_fixture_set_v1();
        validate_acip_review_fixture_set_v1(&fixtures).expect("review fixtures should validate");
        assert_eq!(fixtures.negative_cases.len(), 6);
    }

    #[test]
    fn acip_review_specialization_rejects_self_review_and_merge_authority_gaps() {
        let fixtures = acip_review_fixture_set_v1();

        let same_session = fixtures
            .negative_cases
            .iter()
            .find(|case| case.name == "same_session_self_review_rejected")
            .expect("same session case");
        let same_session_contract: AcipReviewInvocationContractV1 =
            serde_json::from_value(same_session.contract.clone()).expect("contract");
        let same_session_error =
            validate_acip_review_invocation_contract_v1(&same_session_contract)
                .expect_err("same-session review should fail");
        assert!(same_session_error
            .to_string()
            .contains("forbids same-session review"));

        let same_model = fixtures
            .negative_cases
            .iter()
            .find(|case| case.name == "same_model_self_blessing_rejected")
            .expect("same model case");
        let same_model_contract: AcipReviewInvocationContractV1 =
            serde_json::from_value(same_model.contract.clone()).expect("contract");
        let same_model_error = validate_acip_review_invocation_contract_v1(&same_model_contract)
            .expect_err("same-model review should fail");
        assert!(same_model_error
            .to_string()
            .contains("forbids same-model review"));

        let merge_gap = fixtures
            .negative_cases
            .iter()
            .find(|case| case.name == "merge_authority_gap_rejected")
            .expect("merge gap case");
        let merge_gap_contract: AcipReviewInvocationContractV1 =
            serde_json::from_value(merge_gap.contract.clone()).expect("contract");
        let merge_gap_error = validate_acip_review_invocation_contract_v1(&merge_gap_contract)
            .expect_err("missing merge prohibition should fail");
        assert!(merge_gap_error
            .to_string()
            .contains("requires constraints.prohibited_actions to include 'merge'"));
    }

    #[test]
    fn acip_review_specialization_rejects_invalid_outcome_contract_mappings() {
        let fixtures = acip_review_fixture_set_v1();

        let blocked_case = fixtures
            .negative_cases
            .iter()
            .find(|case| case.name == "blocked_outcome_requires_findings_ref")
            .expect("blocked outcome case");
        let contract: AcipReviewInvocationContractV1 =
            serde_json::from_value(blocked_case.contract.clone()).expect("contract");
        let outcome: AcipReviewOutcomeV1 =
            serde_json::from_value(blocked_case.outcome.clone().expect("outcome"))
                .expect("outcome");
        let blocked_error = validate_acip_review_outcome_v1(&contract, &outcome)
            .expect_err("blocked review without findings should fail");
        assert!(blocked_error
            .to_string()
            .contains("blocked review outcome must carry findings_ref"));

        let non_blessed_case = fixtures
            .negative_cases
            .iter()
            .find(|case| case.name == "non_blessed_outcome_cannot_allow_pr_finish")
            .expect("non blessed case");
        let contract: AcipReviewInvocationContractV1 =
            serde_json::from_value(non_blessed_case.contract.clone()).expect("contract");
        let outcome: AcipReviewOutcomeV1 =
            serde_json::from_value(non_blessed_case.outcome.clone().expect("outcome"))
                .expect("outcome");
        let handoff_error = validate_acip_review_outcome_v1(&contract, &outcome)
            .expect_err("non-blessed review should not allow PR finish");
        assert!(
            handoff_error
                .to_string()
                .contains("review outcome handoff must match the disposition contract mapping")
                || handoff_error
                    .to_string()
                    .contains("only blessed review outcomes may allow PR finish")
        );
    }

    #[test]
    fn acip_review_specialization_binds_wrapper_refs_to_invocation_contract() {
        let mut contract = sample_review_invocation_contract();
        contract.review_packet_ref = "runtime/comms/review/drifted_packet.json".to_string();
        let packet_error = validate_acip_review_invocation_contract_v1(&contract)
            .expect_err("drifted review packet ref should fail");
        assert!(packet_error
            .to_string()
            .contains("invocation.input_refs to include review_packet_ref"));

        let contract = sample_review_invocation_contract();
        let mut outcome = sample_review_outcome(
            &contract,
            AcipReviewDispositionV1::Blessed,
            AcipReviewHandoffOutcomeV1::AllowPrFinish,
            Some("runtime/comms/review/findings/clean_review.json".to_string()),
            vec!["runtime/comms/review/residual_risk/none.json".to_string()],
            true,
        );
        outcome.gate_result_ref = "runtime/comms/review/not_the_gate.json".to_string();
        let output_error = validate_acip_review_outcome_v1(&contract, &outcome)
            .expect_err("drifted gate result ref should fail");
        assert!(output_error
            .to_string()
            .contains("gate_result_ref must match the declared gate_result output contract"));
    }

    #[test]
    fn acip_coding_specialization_schemas_and_fixtures_are_available() {
        let contract_schema = acip_coding_invocation_v1_schema_json().expect("coding schema");
        let outcome_schema = acip_coding_outcome_v1_schema_json().expect("outcome schema");
        let fixture_schema = acip_coding_fixture_set_v1_schema_json().expect("fixture schema");
        assert!(contract_schema.contains("\"provider_lane\""));
        assert!(outcome_schema.contains("\"review_handoff_ref\""));
        assert!(fixture_schema.contains("\"valid_codex_outcome\""));

        let fixtures = acip_coding_fixture_set_v1();
        validate_acip_coding_fixture_set_v1(&fixtures).expect("coding fixtures should validate");
        assert_eq!(fixtures.negative_cases.len(), 12);
    }

    #[test]
    fn acip_coding_specialization_rejects_non_codex_worktree_and_review_bypass() {
        let fixtures = acip_coding_fixture_set_v1();

        let non_codex = fixtures
            .negative_cases
            .iter()
            .find(|case| case.name == "non_codex_worktree_edit_rejected")
            .expect("non codex case");
        let non_codex_contract: AcipCodingInvocationContractV1 =
            serde_json::from_value(non_codex.contract.clone()).expect("contract");
        let non_codex_error = validate_acip_coding_invocation_contract_v1(&non_codex_contract)
            .expect_err("non-codex worktree edit should fail");
        assert!(non_codex_error
            .to_string()
            .contains("only codex_issue_worktree lane may use execution_mode 'worktree_edit'"));

        let review_schema = fixtures
            .negative_cases
            .iter()
            .find(|case| case.name == "review_schema_drift_rejected")
            .expect("review schema case");
        let review_schema_contract: AcipCodingInvocationContractV1 =
            serde_json::from_value(review_schema.contract.clone()).expect("contract");
        let review_schema_error =
            validate_acip_coding_invocation_contract_v1(&review_schema_contract)
                .expect_err("review schema drift should fail");
        assert!(review_schema_error
            .to_string()
            .contains("must route to review schema 'acip.review.invocation.v1'"));

        let capability_case = fixtures
            .negative_cases
            .iter()
            .find(|case| case.name == "proposal_lane_code_edit_capability_rejected")
            .expect("capability case");
        let capability_contract: AcipCodingInvocationContractV1 =
            serde_json::from_value(capability_case.contract.clone()).expect("contract");
        let capability_error = validate_acip_coding_invocation_contract_v1(&capability_contract)
            .expect_err("proposal-only lane should not claim code_edit");
        assert!(capability_error
            .to_string()
            .contains("proposal-only coding lanes must not claim 'code_edit' capability"));

        let chatgpt_case = fixtures
            .negative_cases
            .iter()
            .find(|case| case.name == "chatgpt_structured_proposal_rejected")
            .expect("chatgpt case");
        let chatgpt_contract: AcipCodingInvocationContractV1 =
            serde_json::from_value(chatgpt_case.contract.clone()).expect("contract");
        let chatgpt_error = validate_acip_coding_invocation_contract_v1(&chatgpt_contract)
            .expect_err("chatgpt structured proposal should fail");
        assert!(chatgpt_error
            .to_string()
            .contains("chatgpt_api lane does not permit execution_mode 'structured_proposal'"));

        let worktree_flag_case = fixtures
            .negative_cases
            .iter()
            .find(|case| case.name == "proposal_lane_worktree_flag_rejected")
            .expect("worktree flag case");
        let worktree_flag_contract: AcipCodingInvocationContractV1 =
            serde_json::from_value(worktree_flag_case.contract.clone()).expect("contract");
        let worktree_flag_error =
            validate_acip_coding_invocation_contract_v1(&worktree_flag_contract)
                .expect_err("proposal-only lane should not request issue worktree");
        assert!(worktree_flag_error
            .to_string()
            .contains("proposal-only coding lanes must set issue_worktree_required to false"));
    }

    #[test]
    fn acip_coding_specialization_binds_output_contract_and_writer_identity() {
        let patch_contract = sample_coding_invocation_contract(
            AcipCodingProviderLaneV1::ChatgptApi,
            AcipCodingExecutionModeV1::UnappliedPatch,
        );
        let mut patch_outcome = sample_coding_outcome(&patch_contract);
        patch_outcome.primary_output_ref = "runtime/comms/coding/proposal.json".to_string();
        let patch_error = validate_acip_coding_outcome_v1(&patch_contract, &patch_outcome)
            .expect_err("patch output drift should fail");
        assert!(patch_error
            .to_string()
            .contains("must match the declared patch_diff output contract"));

        let worktree_contract = sample_coding_invocation_contract(
            AcipCodingProviderLaneV1::CodexIssueWorktree,
            AcipCodingExecutionModeV1::WorktreeEdit,
        );
        let mut worktree_outcome = sample_coding_outcome(&worktree_contract);
        worktree_outcome.writer_session_id = "drifted-session".to_string();
        let identity_error = validate_acip_coding_outcome_v1(&worktree_contract, &worktree_outcome)
            .expect_err("writer identity drift should fail");
        assert!(identity_error
            .to_string()
            .contains("must preserve the writer_session_id declared by the approval policy"));

        let mut validation_outcome = sample_coding_outcome(&worktree_contract);
        validation_outcome.validation_result_refs =
            vec!["runtime/comms/coding/not_validation.json".to_string()];
        let validation_error =
            validate_acip_coding_outcome_v1(&worktree_contract, &validation_outcome)
                .expect_err("validation summary drift should fail");
        assert!(validation_error.to_string().contains(
            "validation_result_refs must include the declared validation_summary output contract"
        ));

        let mut stop_contract = sample_coding_invocation_contract(
            AcipCodingProviderLaneV1::CodexIssueWorktree,
            AcipCodingExecutionModeV1::WorktreeEdit,
        );
        stop_contract.invocation.stop_policy.stop_on_failure = false;
        let stop_error = validate_acip_coding_invocation_contract_v1(&stop_contract)
            .expect_err("stop policy drift should fail");
        assert!(stop_error
            .to_string()
            .contains("requires stop_policy.stop_on_failure to be true"));
    }

    #[test]
    fn acip_coding_sample_contracts_cover_all_provider_lanes_and_output_modes() {
        let cases = vec![
            (
                AcipCodingProviderLaneV1::CodexIssueWorktree,
                AcipCodingExecutionModeV1::WorktreeEdit,
                "writer-session-codex",
                "gpt-5-codex",
                "runtime/comms/coding/patch_manifest.json",
                true,
            ),
            (
                AcipCodingProviderLaneV1::ChatgptApi,
                AcipCodingExecutionModeV1::UnappliedPatch,
                "writer-session-openai",
                "gpt-5-api",
                "runtime/comms/coding/proposed_changes.diff",
                false,
            ),
            (
                AcipCodingProviderLaneV1::ClaudeApi,
                AcipCodingExecutionModeV1::StructuredProposal,
                "writer-session-anthropic",
                "claude-api",
                "runtime/comms/coding/proposal.json",
                false,
            ),
            (
                AcipCodingProviderLaneV1::LocalOllama,
                AcipCodingExecutionModeV1::UnappliedPatch,
                "writer-session-ollama",
                "gemma3-local",
                "runtime/comms/coding/proposed_changes.diff",
                false,
            ),
            (
                AcipCodingProviderLaneV1::OtherProposalOnly,
                AcipCodingExecutionModeV1::StructuredProposal,
                "writer-session-other",
                "other-provider",
                "runtime/comms/coding/proposal.json",
                false,
            ),
        ];

        for (provider_lane, execution_mode, session_id, model_ref, output_ref, worktree_required) in
            cases
        {
            let contract =
                sample_coding_invocation_contract(provider_lane.clone(), execution_mode.clone());
            validate_acip_coding_invocation_contract_v1(&contract)
                .expect("sample coding contract should validate");
            assert_eq!(contract.approval_policy.writer_session_id, session_id);
            assert_eq!(contract.approval_policy.writer_model_ref, model_ref);
            assert_eq!(contract.issue_worktree_required, worktree_required);

            let outcome = sample_coding_outcome(&contract);
            validate_acip_coding_outcome_v1(&contract, &outcome)
                .expect("sample coding outcome should validate");
            assert_eq!(outcome.primary_output_ref, output_ref);
            assert_eq!(
                outcome.writer_session_id,
                contract.approval_policy.writer_session_id
            );
            assert_eq!(
                outcome.writer_model_ref,
                contract.approval_policy.writer_model_ref
            );
        }
    }

    #[test]
    fn acip_trace_sample_helpers_cover_review_and_coding_specialization_branches() {
        let review_contract = sample_review_invocation_contract();
        validate_acip_review_invocation_contract_v1(&review_contract)
            .expect("sample review contract should validate");
        let dispositions = &review_contract.disposition_contract.allowed_dispositions;
        assert_eq!(dispositions.len(), 4);
        assert!(dispositions.contains(&AcipReviewDispositionV1::Blessed));
        assert!(dispositions.contains(&AcipReviewDispositionV1::Blocked));
        assert!(dispositions.contains(&AcipReviewDispositionV1::NonProving));
        assert!(dispositions.contains(&AcipReviewDispositionV1::Skipped));

        let blocked = sample_review_outcome(
            &review_contract,
            AcipReviewDispositionV1::Blocked,
            AcipReviewHandoffOutcomeV1::FixFindingsAndRerunReview,
            Some("runtime/comms/review/findings/blocker.json".to_string()),
            vec!["runtime/comms/review/residual_risk/known_gap.json".to_string()],
            false,
        );
        validate_acip_review_outcome_v1(&review_contract, &blocked)
            .expect("blocked review outcome should validate");

        let skipped = sample_review_outcome(
            &review_contract,
            AcipReviewDispositionV1::Skipped,
            AcipReviewHandoffOutcomeV1::OperatorWaiverRequired,
            None,
            vec!["runtime/comms/review/residual_risk/provider_unavailable.json".to_string()],
            false,
        );
        validate_acip_review_outcome_v1(&review_contract, &skipped)
            .expect("skipped review outcome should validate");

        let failed_trace_bundle = sample_trace_bundle(AcipInvocationStatusV1::Failed);
        validate_acip_trace_bundle_v1(&failed_trace_bundle)
            .expect("failed trace bundle helper should validate");
    }

    #[test]
    fn acip_trace_bundle_schemas_and_fixtures_are_available() {
        let bundle_schema = acip_trace_bundle_v1_schema_json().expect("bundle schema");
        assert!(bundle_schema.contains("AcipTraceBundleV1"));
        let fixture_schema = acip_trace_fixture_set_v1_schema_json().expect("fixture schema");
        assert!(fixture_schema.contains("AcipTraceFixtureSetV1"));

        let fixtures = acip_trace_fixture_set_v1();
        validate_acip_trace_fixture_set_v1(&fixtures).expect("trace fixtures should validate");
        assert_eq!(fixtures.negative_cases.len(), 5);
    }

    #[test]
    fn acip_trace_bundle_requires_terminal_mapping_and_replay_posture() {
        let completed = sample_trace_bundle(AcipInvocationStatusV1::Completed);
        validate_acip_trace_bundle_v1(&completed).expect("completed bundle should validate");

        let refused = sample_trace_bundle(AcipInvocationStatusV1::Refused);
        validate_acip_trace_bundle_v1(&refused).expect("refused bundle should validate");

        let mut invalid = sample_trace_bundle(AcipInvocationStatusV1::Completed);
        invalid.trace_events[3].event_kind = AcipTraceEventKindV1::InvocationFailed;
        let terminal_error =
            validate_acip_trace_bundle_v1(&invalid).expect_err("terminal mismatch should fail");
        assert!(terminal_error
            .to_string()
            .contains("invocation_failed trace event must carry invocation_status 'failed'"));

        let mut duplicate_terminal = sample_trace_bundle(AcipInvocationStatusV1::Completed);
        duplicate_terminal.trace_events.push(AcipTraceEventV1 {
            event_id: "trace-0005".to_string(),
            conversation_id: duplicate_terminal.conversation_id.clone(),
            invocation_id: Some("invoke-0001".to_string()),
            event_kind: AcipTraceEventKindV1::InvocationCompleted,
            source_message_id: Some("msg-review-0001".to_string()),
            contract_ref: Some(
                "runtime/comms/invocation/contracts/review_request.json".to_string(),
            ),
            decision_event_ref: Some("gate.review-0001".to_string()),
            invocation_status: Some(AcipInvocationStatusV1::Completed),
            output_refs: vec!["runtime/comms/invocation/review_report.json".to_string()],
            evidence_refs: vec![
                "runtime/comms/invocation/evidence/completed_trace.json".to_string()
            ],
            summary: "Duplicate terminal event for regression coverage.".to_string(),
            requires_redaction: false,
        });
        let duplicate_error = validate_acip_trace_bundle_v1(&duplicate_terminal)
            .expect_err("duplicate terminal event should fail");
        assert!(duplicate_error
            .to_string()
            .contains("trace bundle must not contain duplicate event kind 'invocation_completed'"));

        let mut drift_bundle = sample_trace_bundle(AcipInvocationStatusV1::Completed);
        drift_bundle.trace_events[2].decision_event_ref = Some("gate.review-drifted".to_string());
        let drift_error =
            validate_acip_trace_bundle_v1(&drift_bundle).expect_err("decision drift should fail");
        assert!(drift_error.to_string().contains(
            "trace bundle must preserve one canonical decision_event_ref across non-message events"
        ));

        let mut missing_contract_bundle = sample_trace_bundle(AcipInvocationStatusV1::Completed);
        missing_contract_bundle.trace_events[2].contract_ref = None;
        let missing_contract_error = validate_acip_trace_bundle_v1(&missing_contract_bundle)
            .expect_err("missing contract ref should fail closed");
        assert!(missing_contract_error.to_string().contains(
            "decision_recorded trace event must carry invocation_id, contract_ref, and decision_event_ref"
        ));

        let mut order_bundle = sample_trace_bundle(AcipInvocationStatusV1::Completed);
        order_bundle.trace_events.swap(1, 2);
        let order_error = validate_acip_trace_bundle_v1(&order_bundle)
            .expect_err("out-of-order trace event should fail");
        assert!(order_error
            .to_string()
            .contains("trace bundle must preserve canonical event order"));

        let mut replay_invalid = sample_trace_bundle(AcipInvocationStatusV1::Completed);
        replay_invalid.replay_contract.deterministic_redaction_views = false;
        let replay_error = validate_acip_trace_bundle_v1(&replay_invalid)
            .expect_err("non-deterministic redaction view should fail");
        assert!(replay_error
            .to_string()
            .contains("ACIP replay contract requires deterministic_redaction_views"));

        let mut message_boundary_bundle = sample_trace_bundle(AcipInvocationStatusV1::Completed);
        message_boundary_bundle.trace_events[0].decision_event_ref =
            Some("gate.review-0001".to_string());
        let message_boundary_error = validate_acip_trace_bundle_v1(&message_boundary_bundle)
            .expect_err("message_created should reject post-message invocation metadata");
        assert!(message_boundary_error
            .to_string()
            .contains("message_created trace event must not carry post-message invocation fields"));
    }

    #[test]
    fn acip_trace_bundle_redaction_views_fail_closed_on_leakage() {
        let mut bundle = sample_trace_bundle(AcipInvocationStatusV1::Completed);
        bundle.audience_views[2]
            .visible_artifact_refs
            .push("runtime/comms/private_state/raw_args.json".to_string());
        let reviewer_error =
            validate_acip_trace_bundle_v1(&bundle).expect_err("reviewer leak should fail");
        assert!(reviewer_error.to_string().contains(
            "visible_artifact_refs[] must not expose unredacted trace ref 'private_state'"
        ));

        let mut narrative_bundle = sample_trace_bundle(AcipInvocationStatusV1::Completed);
        narrative_bundle.audience_views[2].narrative_ref =
            "runtime/comms/trace/private_state_dump.json".to_string();
        let narrative_error = validate_acip_trace_bundle_v1(&narrative_bundle)
            .expect_err("narrative leak should fail");
        assert!(narrative_error
            .to_string()
            .contains("narrative_ref must not expose unredacted trace ref 'private_state'"));

        let mut summary_bundle = sample_trace_bundle(AcipInvocationStatusV1::Failed);
        summary_bundle.trace_events[3].summary =
            "Failure packet contained secret token and raw operator prompt.".to_string();
        let summary_error = validate_acip_trace_bundle_v1(&summary_bundle)
            .expect_err("protected summary leak should fail");
        assert!(summary_error
            .to_string()
            .contains("summary must not leak protected trace content 'secret'"));

        let mut path_bundle = sample_trace_bundle(AcipInvocationStatusV1::Failed);
        path_bundle.trace_events[3].summary =
            "Failure packet copied from /var/folders/tmp and raw tool arguments for replay."
                .to_string();
        let path_error = validate_acip_trace_bundle_v1(&path_bundle)
            .expect_err("workstation path leak should fail");
        assert!(path_error
            .to_string()
            .contains("summary must not leak protected trace content 'raw tool arguments'"));

        let mut rejected_alt_ref_bundle = sample_trace_bundle(AcipInvocationStatusV1::Completed);
        rejected_alt_ref_bundle.audience_views[3]
            .visible_artifact_refs
            .push("runtime/comms/trace/rejected-alternative-notes.json".to_string());
        let rejected_alt_ref_error = validate_acip_trace_bundle_v1(&rejected_alt_ref_bundle)
            .expect_err("rejected alternative refs should fail closed");
        assert!(rejected_alt_ref_error.to_string().contains(
            "visible_artifact_refs[] must not expose unredacted trace ref 'rejected-alternative'"
        ));

        let mut rejected_alt_summary_bundle = sample_trace_bundle(AcipInvocationStatusV1::Failed);
        rejected_alt_summary_bundle.trace_events[3].summary =
            "Failure packet included rejected alternative reasoning for reviewers.".to_string();
        let rejected_alt_summary_error =
            validate_acip_trace_bundle_v1(&rejected_alt_summary_bundle)
                .expect_err("rejected alternative summaries should fail closed");
        assert!(rejected_alt_summary_error
            .to_string()
            .contains("summary must not leak protected trace content 'rejected alternative'"));
    }

    #[test]
    fn acip_trace_bundle_event_and_audience_guardrails_cover_remaining_branches() {
        let mut conversation_mismatch = sample_trace_bundle(AcipInvocationStatusV1::Completed);
        conversation_mismatch.trace_events[1].conversation_id = "conv-review-drift".to_string();
        let conversation_error = validate_acip_trace_bundle_v1(&conversation_mismatch)
            .expect_err("conversation drift should fail");
        assert!(conversation_error
            .to_string()
            .contains("trace event conversation_id must match the bundle conversation_id"));

        let mut missing_message_source = sample_trace_bundle(AcipInvocationStatusV1::Completed);
        missing_message_source.trace_events[0].source_message_id = None;
        let message_source_error = validate_acip_trace_bundle_v1(&missing_message_source)
            .expect_err("message_created without source_message_id should fail");
        assert!(message_source_error
            .to_string()
            .contains("message_created trace event must carry source_message_id"));

        let mut missing_declared_decision = sample_trace_bundle(AcipInvocationStatusV1::Completed);
        missing_declared_decision.trace_events[1].decision_event_ref = None;
        let declared_decision_error = validate_acip_trace_bundle_v1(&missing_declared_decision)
            .expect_err("invocation contract declaration without decision ref should fail");
        assert!(declared_decision_error.to_string().contains(
            "invocation_contract_declared trace event must carry decision_event_ref"
        ));

        let mut completed_missing_outputs = sample_trace_bundle(AcipInvocationStatusV1::Completed);
        completed_missing_outputs.trace_events[3].output_refs.clear();
        let completed_error = validate_acip_trace_bundle_v1(&completed_missing_outputs)
            .expect_err("completed terminal event without outputs should fail");
        assert!(completed_error.to_string().contains(
            "invocation_completed trace event must carry invocation_id, contract_ref, decision_event_ref, and output_refs"
        ));

        let mut refused_with_outputs = sample_trace_bundle(AcipInvocationStatusV1::Refused);
        refused_with_outputs.trace_events[3]
            .output_refs
            .push("runtime/comms/invocation/should_not_exist.json".to_string());
        let refused_output_error = validate_acip_trace_bundle_v1(&refused_with_outputs)
            .expect_err("refused terminal event must not expose outputs");
        assert!(refused_output_error
            .to_string()
            .contains("invocation_refused trace event must not carry output_refs"));

        let mut failed_with_outputs = sample_trace_bundle(AcipInvocationStatusV1::Failed);
        failed_with_outputs.trace_events[3]
            .output_refs
            .push("runtime/comms/invocation/should_not_exist.json".to_string());
        let failed_output_error = validate_acip_trace_bundle_v1(&failed_with_outputs)
            .expect_err("failed terminal event must not expose outputs");
        assert!(failed_output_error
            .to_string()
            .contains("invocation_failed trace event must not carry output_refs"));

        let mut missing_evidence_refs = sample_trace_bundle(AcipInvocationStatusV1::Completed);
        missing_evidence_refs.evidence_packet_refs.clear();
        let missing_evidence_error = validate_acip_trace_bundle_v1(&missing_evidence_refs)
            .expect_err("bundle must require evidence packet refs");
        assert!(missing_evidence_error
            .to_string()
            .contains("ACIP trace bundle requires evidence_packet_refs"));

        let mut empty_visible_events = sample_trace_bundle(AcipInvocationStatusV1::Completed);
        empty_visible_events.audience_views[0].visible_event_ids.clear();
        let visible_events_error = validate_acip_trace_bundle_v1(&empty_visible_events)
            .expect_err("audience views must expose visible events");
        assert!(visible_events_error
            .to_string()
            .contains("trace audience view must carry visible_event_ids"));

        let mut unknown_visible_event = sample_trace_bundle(AcipInvocationStatusV1::Completed);
        unknown_visible_event.audience_views[0]
            .visible_event_ids
            .push("trace-9999".to_string());
        let unknown_event_error = validate_acip_trace_bundle_v1(&unknown_visible_event)
            .expect_err("unknown visible event ids should fail");
        assert!(unknown_event_error
            .to_string()
            .contains("trace audience view references unknown event_id 'trace-9999'"));

        let mut missing_redactions = sample_trace_bundle(AcipInvocationStatusV1::Completed);
        missing_redactions.audience_views[2].redacted_elements.clear();
        let redaction_error = validate_acip_trace_bundle_v1(&missing_redactions)
            .expect_err("audience views must declare redactions");
        assert!(redaction_error
            .to_string()
            .contains("trace audience view must declare redacted_elements"));

        let mut local_path_allowed = sample_trace_bundle(AcipInvocationStatusV1::Completed);
        local_path_allowed.audience_views[4].allows_local_host_paths = true;
        let local_path_error = validate_acip_trace_bundle_v1(&local_path_allowed)
            .expect_err("review-safe audiences must not allow local host paths");
        assert!(local_path_error
            .to_string()
            .contains("reviewer, public, and observatory views must not allow local host paths"));

        let mut rejected_details_allowed = sample_trace_bundle(AcipInvocationStatusV1::Completed);
        rejected_details_allowed.audience_views[3].allows_rejected_alternative_details = true;
        let rejected_details_error = validate_acip_trace_bundle_v1(&rejected_details_allowed)
            .expect_err("review-safe audiences must not allow rejected alternatives");
        assert!(rejected_details_error.to_string().contains(
            "reviewer, public, and observatory views must not allow rejected alternative details"
        ));

        let mut nondeterministic_order = sample_trace_bundle(AcipInvocationStatusV1::Completed);
        nondeterministic_order
            .replay_contract
            .deterministic_event_order = false;
        let replay_order_error = validate_acip_trace_bundle_v1(&nondeterministic_order)
            .expect_err("replay contract must require deterministic event ordering");
        assert!(replay_order_error
            .to_string()
            .contains("ACIP replay contract requires deterministic_event_order"));
    }

    #[test]
    fn acip_trace_fixture_set_fails_closed_on_schema_and_negative_case_drift() {
        let mut wrong_schema = acip_trace_fixture_set_v1();
        wrong_schema.schema_version = "acip.trace.fixture-set.v0".to_string();
        let wrong_schema_error = validate_acip_trace_fixture_set_v1(&wrong_schema)
            .expect_err("fixture-set schema drift should fail");
        assert!(wrong_schema_error
            .to_string()
            .contains("ACIP trace fixture set requires schema_version"));

        let mut missing_negative_cases = acip_trace_fixture_set_v1();
        missing_negative_cases.negative_cases.clear();
        let negative_cases_error = validate_acip_trace_fixture_set_v1(&missing_negative_cases)
            .expect_err("fixture-set must require negative cases");
        assert!(negative_cases_error
            .to_string()
            .contains("ACIP trace fixture set requires negative_cases"));

        let mut malformed_negative_case = acip_trace_fixture_set_v1();
        malformed_negative_case.negative_cases[0].name = "../bad".to_string();
        let malformed_name_error = validate_acip_trace_fixture_set_v1(&malformed_negative_case)
            .expect_err("negative case names must remain machine-safe");
        assert!(malformed_name_error
            .to_string()
            .contains("negative_cases[].name"));

        let mut empty_expected_error = acip_trace_fixture_set_v1();
        empty_expected_error.negative_cases[0].expected_error_substring.clear();
        let expected_error = validate_acip_trace_fixture_set_v1(&empty_expected_error)
            .expect_err("negative cases must declare the expected error substring");
        assert!(expected_error
            .to_string()
            .contains("negative_cases[].expected_error_substring"));
    }

    #[test]
    fn acip_trace_event_and_audience_helpers_cover_remaining_validation_paths() {
        let mut duplicate_audience = sample_trace_bundle(AcipInvocationStatusV1::Completed);
        duplicate_audience.audience_views[4].audience = AcipTraceAudienceV1::Public;
        assert!(validate_acip_trace_bundle_v1(&duplicate_audience)
            .expect_err("duplicate audiences should fail")
            .to_string()
            .contains("trace bundle contains duplicate audience view 'public'"));

        let mut missing_required_event = sample_trace_bundle(AcipInvocationStatusV1::Completed);
        missing_required_event
            .trace_events
            .retain(|event| event.event_kind != AcipTraceEventKindV1::DecisionRecorded);
        assert!(validate_acip_trace_bundle_v1(&missing_required_event)
            .expect_err("missing required event kind should fail")
            .to_string()
            .contains("trace bundle must preserve canonical event order"));

        let mut duplicate_event_id = sample_trace_bundle(AcipInvocationStatusV1::Completed);
        duplicate_event_id.trace_events[3].event_id = "trace-0003".to_string();
        assert!(validate_acip_trace_bundle_v1(&duplicate_event_id)
            .expect_err("duplicate event id should fail")
            .to_string()
            .contains("trace bundle contains duplicate event_id 'trace-0003'"));

        let mut bad_replay_fixture = sample_trace_bundle(AcipInvocationStatusV1::Completed);
        bad_replay_fixture.replay_contract.fixture_ref = "/tmp/replay.json".to_string();
        assert!(validate_acip_trace_bundle_v1(&bad_replay_fixture)
            .expect_err("absolute replay fixture ref should fail")
            .to_string()
            .contains("fixture_ref"));

        let mut bad_replay_case = sample_trace_bundle(AcipInvocationStatusV1::Completed);
        bad_replay_case.replay_contract.fixture_case = "../bad".to_string();
        assert!(validate_acip_trace_bundle_v1(&bad_replay_case)
            .expect_err("unsafe replay fixture case should fail")
            .to_string()
            .contains("fixture_case"));

        let mut prompt_leak = sample_trace_bundle(AcipInvocationStatusV1::Completed);
        prompt_leak.trace_events[3].summary =
            "Terminal event included prompt fragments for debugging.".to_string();
        assert!(validate_acip_trace_bundle_v1(&prompt_leak)
            .expect_err("prompt leak should fail")
            .to_string()
            .contains("summary must not leak protected trace content 'prompt'"));

        let mut raw_args_ref = sample_trace_bundle(AcipInvocationStatusV1::Completed);
        raw_args_ref.audience_views[2]
            .visible_artifact_refs
            .push("runtime/comms/trace/raw_args.json".to_string());
        assert!(validate_acip_trace_bundle_v1(&raw_args_ref)
            .expect_err("raw args ref should fail")
            .to_string()
            .contains("visible_artifact_refs[] must not expose unredacted trace ref 'raw_args'"));

        let mut wrong_terminal_position = sample_trace_bundle(AcipInvocationStatusV1::Completed);
        let terminal = wrong_terminal_position.trace_events.pop().expect("terminal event");
        wrong_terminal_position.trace_events.insert(1, terminal);
        assert!(validate_acip_trace_bundle_v1(&wrong_terminal_position)
            .expect_err("terminal event must remain last")
            .to_string()
            .contains("trace bundle must preserve canonical event order"));
    }

    #[test]
    fn acip_trace_sample_invocation_helpers_cover_event_status_and_value_parsing() {
        let contract = sample_invocation_contract();
        let completed = sample_invocation_event(
            &contract,
            AcipInvocationStatusV1::Completed,
            vec!["runtime/comms/invocation/review_report.json".to_string()],
            "required outputs satisfied".to_string(),
            None,
            None,
            vec!["runtime/comms/invocation/evidence/completed_trace.json".to_string()],
        );
        assert_eq!(completed.status, AcipInvocationStatusV1::Completed);
        assert!(completed.refusal_code.is_none());
        assert!(completed.failure_code.is_none());

        let refused = sample_invocation_event(
            &contract,
            AcipInvocationStatusV1::Refused,
            Vec::new(),
            "governed refusal".to_string(),
            Some("policy_denied".to_string()),
            None,
            vec!["runtime/comms/invocation/evidence/refusal_trace.json".to_string()],
        );
        assert_eq!(refused.status, AcipInvocationStatusV1::Refused);
        assert_eq!(refused.refusal_code.as_deref(), Some("policy_denied"));

        let failed = sample_invocation_event(
            &contract,
            AcipInvocationStatusV1::Failed,
            Vec::new(),
            "execution failure".to_string(),
            None,
            Some("provider_error".to_string()),
            vec!["runtime/comms/invocation/evidence/failure_trace.json".to_string()],
        );
        assert_eq!(failed.status, AcipInvocationStatusV1::Failed);
        assert_eq!(failed.failure_code.as_deref(), Some("provider_error"));

        let completed_value =
            serde_json::to_value(sample_trace_bundle(AcipInvocationStatusV1::Completed))
                .expect("serialize bundle");
        validate_acip_trace_bundle_v1_value(&completed_value)
            .expect("valid json bundle should parse and validate");

        let parse_error = validate_acip_trace_bundle_v1_value(&serde_json::json!({
            "schema_version": ACIP_TRACE_BUNDLE_SCHEMA_VERSION,
            "conversation_id": 7
        }))
        .expect_err("malformed bundle json should fail");
        assert!(parse_error
            .to_string()
            .contains("parse ACIP trace bundle v1"));
    }

    #[test]
    fn acip_proof_demo_packet_schema_and_fixture_are_available() {
        let schema = acip_proof_demo_packet_v1_schema_json().expect("proof schema");
        assert!(schema.contains("AcipProofDemoPacketV1"));
        assert!(schema.contains("\"represented_modes\""));

        let packet = acip_proof_demo_packet_v1();
        validate_acip_proof_demo_packet_v1(&packet).expect("proof packet should validate");
        assert_eq!(
            packet.represented_modes,
            vec![
                AcipIntentV1::Consultation,
                AcipIntentV1::Negotiation,
                AcipIntentV1::CodingRequest
            ]
        );
    }

    #[test]
    fn acip_proof_demo_packet_requires_multi_mode_linkage_and_non_proving_guards() {
        let mut packet = acip_proof_demo_packet_v1();
        packet.represented_modes = vec![AcipIntentV1::CodingRequest];
        let mode_error = validate_acip_proof_demo_packet_v1(&packet)
            .expect_err("single-mode proof packet should fail");
        assert!(mode_error.to_string().contains(
            "must carry exactly consultation, negotiation, and coding_request represented_modes"
        ));

        let mut packet = acip_proof_demo_packet_v1();
        packet.conversation.messages[1].intent = AcipIntentV1::Conversation;
        packet.conversation.messages[1].authority_scope =
            Some(default_message_authority_scope(&AcipIntentV1::Conversation));
        let negotiation_error = validate_acip_proof_demo_packet_v1(&packet)
            .expect_err("canonical negotiation turn drift should fail");
        assert!(negotiation_error
            .to_string()
            .contains("must keep capability negotiation between consultation and coding_request"));

        let mut packet = acip_proof_demo_packet_v1();
        packet.coding_invocation.invocation.causal_message_id = "msg-other".to_string();
        let linkage_error = validate_acip_proof_demo_packet_v1(&packet)
            .expect_err("causal linkage drift should fail");
        assert!(linkage_error.to_string().contains(
            "must bind coding invocation causal_message_id to the final coding_request message"
        ));

        let mut packet = acip_proof_demo_packet_v1();
        packet.public_visible_artifact_refs =
            vec!["runtime/comms/trace/private_state_dump.json".to_string()];
        let public_error = validate_acip_proof_demo_packet_v1(&packet)
            .expect_err("public refs outside public audience view should fail");
        assert!(public_error.to_string().contains(
            "public_visible_artifact_refs must be present in the public trace audience view"
        ));

        let mut packet = acip_proof_demo_packet_v1();
        packet.validation_commands = vec!["cargo test --lib".to_string()];
        let command_error = validate_acip_proof_demo_packet_v1(&packet)
            .expect_err("validation command drift should fail");
        assert!(command_error
            .to_string()
            .contains("validation_commands must match coding invocation validation_commands"));

        let mut packet = acip_proof_demo_packet_v1();
        packet.reviewer_visible_artifact_refs =
            vec!["runtime/comms/proof/acip_demo_reviewer_trace.json".to_string()];
        let reviewer_error = validate_acip_proof_demo_packet_v1(&packet)
            .expect_err("reviewer artifact drift should fail");
        assert!(reviewer_error.to_string().contains(
            "reviewer_visible_artifact_refs must include reviewer narrative, coding primary output, and review handoff"
        ));

        let mut packet = acip_proof_demo_packet_v1();
        packet.non_proving_statements.pop();
        let non_proving_error = validate_acip_proof_demo_packet_v1(&packet)
            .expect_err("missing non-proving statements should fail");
        assert!(non_proving_error
            .to_string()
            .contains("must carry explicit non_proving_statements"));
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
    fn acip_invocation_event_accepts_exact_required_output_identity() {
        let contract = sample_invocation_contract();
        let event = sample_invocation_event(
            &contract,
            AcipInvocationStatusV1::Completed,
            vec!["runtime/comms/invocation/review_report.json".to_string()],
            "completed_output_contract".to_string(),
            None,
            None,
            vec!["runtime/comms/invocation/evidence/review_trace.json".to_string()],
        );
        validate_acip_invocation_event_against_contract(&contract, &event)
            .expect("exact required output identity should pass");
    }

    #[test]
    fn acip_invocation_event_rejects_incidental_required_output_substring_match() {
        let contract = sample_invocation_contract();
        let event = sample_invocation_event(
            &contract,
            AcipInvocationStatusV1::Completed,
            vec!["runtime/comms/invocation/review_report_shadow.json".to_string()],
            "completed_output_contract".to_string(),
            None,
            None,
            vec!["runtime/comms/invocation/evidence/review_trace.json".to_string()],
        );
        let error = validate_acip_invocation_event_against_contract(&contract, &event)
            .expect_err("incidental substring match should fail");
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

    #[test]
    fn acip_message_envelope_rejects_hidden_governed_authority_and_escalation() {
        let fixtures = acip_fixture_set_v1();
        let hidden = fixtures
            .invalid_messages
            .iter()
            .find(|case| case.name == "hidden_invocation")
            .expect("hidden invocation case");
        let hidden_error = validate_acip_message_envelope_v1_value(&hidden.value)
            .expect_err("hidden invocation should fail");
        assert!(hidden_error
            .to_string()
            .contains("message intent 'conversation' must not claim authority action 'invoke'"));

        let escalation = fixtures
            .invalid_messages
            .iter()
            .find(|case| case.name == "authority_escalation")
            .expect("authority escalation case");
        let escalation_error = validate_acip_message_envelope_v1_value(&escalation.value)
            .expect_err("authority escalation should fail");
        assert!(escalation_error
            .to_string()
            .contains("message intent 'consultation' must not claim authority action 'delegate'"));
    }
