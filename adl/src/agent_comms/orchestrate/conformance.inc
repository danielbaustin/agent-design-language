pub fn acip_conformance_report_v1_schema_json() -> Result<String> {
    serde_json::to_string_pretty(&schema_for!(AcipConformanceReportV1))
        .context("serialize ACIP conformance report v1 schema")
}


pub fn validate_acip_conformance_report_v1(report: &AcipConformanceReportV1) -> Result<()> {
    if report.schema_version != ACIP_CONFORMANCE_REPORT_SCHEMA_VERSION {
        return Err(anyhow!(
            "ACIP conformance report requires schema_version '{}'",
            ACIP_CONFORMANCE_REPORT_SCHEMA_VERSION
        ));
    }
    if report.valid_fixture_classes.is_empty() {
        return Err(anyhow!(
            "ACIP conformance report requires valid_fixture_classes"
        ));
    }
    if report.negative_fixture_classes.is_empty() {
        return Err(anyhow!(
            "ACIP conformance report requires negative_fixture_classes"
        ));
    }

    let mut seen_valid = BTreeSet::new();
    for class in &report.valid_fixture_classes {
        validate_id(&class.fixture_name, "valid_fixture_classes[].fixture_name")?;
        validate_id(&class.mode_label, "valid_fixture_classes[].mode_label")?;
        validate_non_empty(&class.proves, "valid_fixture_classes[].proves")?;
        validate_repo_relative_ref(
            &class.feature_doc_ref,
            "valid_fixture_classes[].feature_doc_ref",
        )?;
        if !seen_valid.insert(class.fixture_name.clone()) {
            return Err(anyhow!(
                "ACIP conformance report contains duplicate valid fixture '{}'",
                class.fixture_name
            ));
        }
    }

    let mut seen_negative = BTreeSet::new();
    for class in &report.negative_fixture_classes {
        validate_id(&class.case_name, "negative_fixture_classes[].case_name")?;
        validate_non_empty(&class.proves, "negative_fixture_classes[].proves")?;
        validate_non_empty(
            &class.expected_error_substring,
            "negative_fixture_classes[].expected_error_substring",
        )?;
        validate_repo_relative_ref(
            &class.feature_doc_ref,
            "negative_fixture_classes[].feature_doc_ref",
        )?;
        if !seen_negative.insert(class.case_name.clone()) {
            return Err(anyhow!(
                "ACIP conformance report contains duplicate negative fixture '{}'",
                class.case_name
            ));
        }
    }

    let required_valid = [
        "conversation",
        "consultation",
        "invocation_setup",
        "review_request",
        "coding_request",
        "coding_agent_handoff",
        "delegation",
        "negotiation",
        "operator_request",
        "broadcast",
        "shared_conversation_thread",
        "governed_invocation_contract",
    ];
    for required in required_valid {
        if !seen_valid.contains(required) {
            return Err(anyhow!(
                "ACIP conformance report missing required valid fixture '{}'",
                required
            ));
        }
    }

    let required_negative = [
        "identity_drift",
        "missing_recipient",
        "hidden_invocation",
        "malformed_payload_refs",
        "unsupported_visibility",
        "raw_local_path_refs",
        "authority_escalation",
        "stale_ordering",
        "missing_gate_rejects_governed_invocation",
        "ambiguous_stop_policy_rejected",
        "unsafe_input_refs_rejected",
        "status_refusal_inconsistency_rejected",
        "output_contract_mismatch_rejected",
    ];
    for required in required_negative {
        if !seen_negative.contains(required) {
            return Err(anyhow!(
                "ACIP conformance report missing required negative fixture '{}'",
                required
            ));
        }
    }

    Ok(())
}


pub fn acip_conformance_report_v1() -> AcipConformanceReportV1 {
    let fixture_set = acip_fixture_set_v1();
    let invocation_fixture_set = acip_invocation_fixture_set_v1();
    let feature_doc_ref = "docs/milestones/v0.90.5/features/AGENT_COMMS_v1.md".to_string();

    let mut valid_fixture_classes = fixture_set
        .valid_messages
        .iter()
        .map(|fixture| {
            let (surface, mode_label, proves) = match fixture.name.as_str() {
                "conversation" => (
                    AcipConformanceSurfaceV1::Message,
                    "conversation",
                    "ACIP supports bounded non-governed conversation without hidden invocation authority.",
                ),
                "consultation" => (
                    AcipConformanceSurfaceV1::Message,
                    "consultation",
                    "ACIP supports advisory consultation with explicit identity and share-only authority.",
                ),
                "invocation_setup" => (
                    AcipConformanceSurfaceV1::Message,
                    "invocation_setup",
                    "ACIP can stage governed invocation setup as a first-class mode before contract execution.",
                ),
                "review_request" => (
                    AcipConformanceSurfaceV1::Message,
                    "review_request",
                    "ACIP supports reviewer-facing governed requests without collapsing into generic chat.",
                ),
                "coding_request" => (
                    AcipConformanceSurfaceV1::Message,
                    "coding_request",
                    "ACIP supports coding-agent requests with bounded task-bundle payloads.",
                ),
                "coding_agent_handoff" => (
                    AcipConformanceSurfaceV1::Message,
                    "handoff",
                    "ACIP supports explicit agent-to-agent coding handoff without redefining the core transport.",
                ),
                "delegation" => (
                    AcipConformanceSurfaceV1::Message,
                    "delegation",
                    "ACIP supports explicit delegation requests with parent-accountability semantics.",
                ),
                "negotiation" => (
                    AcipConformanceSurfaceV1::Message,
                    "negotiation",
                    "ACIP supports negotiation as a first-class communication mode.",
                ),
                "operator_request" => (
                    AcipConformanceSurfaceV1::Message,
                    "operator_request",
                    "ACIP supports operator-group requests without requiring live-provider orchestration.",
                ),
                "broadcast" => (
                    AcipConformanceSurfaceV1::Message,
                    "broadcast",
                    "ACIP supports group broadcast without smuggling governed authority.",
                ),
                other => (
                    AcipConformanceSurfaceV1::Message,
                    other,
                    "ACIP preserves a deterministic valid message fixture.",
                ),
            };
            AcipConformanceFixtureClassV1 {
                fixture_name: fixture.name.clone(),
                surface,
                mode_label: mode_label.to_string(),
                proves: proves.to_string(),
                feature_doc_ref: feature_doc_ref.clone(),
            }
        })
        .collect::<Vec<_>>();

    valid_fixture_classes.push(AcipConformanceFixtureClassV1 {
        fixture_name: "shared_conversation_thread".to_string(),
        surface: AcipConformanceSurfaceV1::Conversation,
        mode_label: "conversation_thread".to_string(),
        proves: "ACIP preserves monotonic multi-message conversation sequencing across mixed conversation, consultation, and review-request turns.".to_string(),
        feature_doc_ref: feature_doc_ref.clone(),
    });
    valid_fixture_classes.push(AcipConformanceFixtureClassV1 {
        fixture_name: "governed_invocation_contract".to_string(),
        surface: AcipConformanceSurfaceV1::Invocation,
        mode_label: "invocation".to_string(),
        proves: "ACIP preserves governed invocation contract, refusal, failure, and completed-output semantics under explicit Freedom Gate linkage.".to_string(),
        feature_doc_ref: feature_doc_ref.clone(),
    });

    let mut negative_fixture_classes = fixture_set
        .invalid_messages
        .iter()
        .map(|case| AcipConformanceNegativeClassV1 {
            case_name: case.name.clone(),
            surface: AcipConformanceSurfaceV1::Message,
            proves: format!(
                "ACIP fails closed for '{}' without leaking host-local or authority-drift semantics.",
                case.name
            ),
            expected_error_substring: case.expected_error_substring.clone(),
            feature_doc_ref: feature_doc_ref.clone(),
        })
        .collect::<Vec<_>>();
    negative_fixture_classes.extend(fixture_set.invalid_conversations.iter().map(|case| {
        AcipConformanceNegativeClassV1 {
            case_name: case.name.clone(),
            surface: AcipConformanceSurfaceV1::Conversation,
            proves: format!(
                "ACIP conversation sequencing rejects '{}' deterministically.",
                case.name
            ),
            expected_error_substring: case.expected_error_substring.clone(),
            feature_doc_ref: feature_doc_ref.clone(),
        }
    }));
    negative_fixture_classes.extend(invocation_fixture_set.negative_cases.iter().map(|case| {
        AcipConformanceNegativeClassV1 {
            case_name: case.name.clone(),
            surface: AcipConformanceSurfaceV1::Invocation,
            proves: format!(
                "ACIP governed invocation rejects '{}' with a stable failure reason.",
                case.name
            ),
            expected_error_substring: case.expected_error_substring.clone(),
            feature_doc_ref: feature_doc_ref.clone(),
        }
    }));

    AcipConformanceReportV1 {
        schema_version: ACIP_CONFORMANCE_REPORT_SCHEMA_VERSION.to_string(),
        valid_fixture_classes,
        negative_fixture_classes,
    }
}

