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
