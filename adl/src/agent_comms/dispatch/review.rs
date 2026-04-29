pub fn acip_review_invocation_v1_schema_json() -> Result<String> {
    serde_json::to_string_pretty(&schema_for!(AcipReviewInvocationContractV1))
        .context("serialize ACIP review invocation contract v1 schema")
}

pub fn acip_review_outcome_v1_schema_json() -> Result<String> {
    serde_json::to_string_pretty(&schema_for!(AcipReviewOutcomeV1))
        .context("serialize ACIP review outcome v1 schema")
}

pub fn acip_review_fixture_set_v1_schema_json() -> Result<String> {
    serde_json::to_string_pretty(&schema_for!(AcipReviewFixtureSetV1))
        .context("serialize ACIP review fixture set v1 schema")
}

pub fn validate_acip_review_invocation_contract_v1_value(
    value: &JsonValue,
) -> Result<AcipReviewInvocationContractV1> {
    let contract: AcipReviewInvocationContractV1 = serde_json::from_value(value.clone())
        .context("parse ACIP review invocation contract v1")?;
    validate_acip_review_invocation_contract_v1(&contract)?;
    Ok(contract)
}

pub fn validate_acip_review_outcome_v1_value(
    contract: &AcipReviewInvocationContractV1,
    value: &JsonValue,
) -> Result<AcipReviewOutcomeV1> {
    let outcome: AcipReviewOutcomeV1 =
        serde_json::from_value(value.clone()).context("parse ACIP review outcome v1")?;
    validate_acip_review_outcome_v1(contract, &outcome)?;
    Ok(outcome)
}


pub fn validate_acip_review_invocation_contract_v1(
    contract: &AcipReviewInvocationContractV1,
) -> Result<()> {
    if contract.schema_version != ACIP_REVIEW_INVOCATION_SCHEMA_VERSION {
        return Err(anyhow!(
            "ACIP review invocation contract requires schema_version '{}'",
            ACIP_REVIEW_INVOCATION_SCHEMA_VERSION
        ));
    }
    validate_acip_invocation_contract_v1(&contract.invocation)?;
    if contract.invocation.intent != AcipIntentV1::ReviewRequest {
        return Err(anyhow!(
            "ACIP review invocation specialization requires invocation.intent 'review_request'"
        ));
    }
    validate_repo_relative_ref(&contract.srp_ref, "srp_ref")?;
    validate_repo_relative_ref(&contract.review_packet_ref, "review_packet_ref")?;
    if contract.evidence_packet_refs.is_empty() {
        return Err(anyhow!(
            "ACIP review invocation specialization requires evidence_packet_refs"
        ));
    }
    for reference in &contract.evidence_packet_refs {
        validate_repo_relative_ref(reference, "evidence_packet_refs[]")?;
    }
    if !contract
        .invocation
        .input_refs
        .iter()
        .any(|reference| reference == &contract.review_packet_ref)
    {
        return Err(anyhow!(
            "ACIP review invocation specialization requires invocation.input_refs to include review_packet_ref"
        ));
    }
    for reference in &contract.evidence_packet_refs {
        if !contract
            .invocation
            .input_refs
            .iter()
            .any(|input| input == reference)
        {
            return Err(anyhow!(
                "ACIP review invocation specialization requires invocation.input_refs to include every evidence_packet_refs entry"
            ));
        }
    }
    if !contract
        .invocation
        .constraints
        .policy_refs
        .iter()
        .any(|policy_ref| policy_ref == &contract.srp_ref)
    {
        return Err(anyhow!(
            "ACIP review invocation specialization requires constraints.policy_refs to include srp_ref"
        ));
    }
    if !contract
        .invocation
        .constraints
        .required_capabilities
        .iter()
        .any(|capability| capability == "review")
    {
        return Err(anyhow!(
            "ACIP review invocation specialization requires constraints.required_capabilities to include 'review'"
        ));
    }
    if !contract
        .invocation
        .constraints
        .prohibited_actions
        .iter()
        .any(|action| action == "merge")
    {
        return Err(anyhow!(
            "ACIP review invocation specialization requires constraints.prohibited_actions to include 'merge'"
        ));
    }
    if !contract
        .invocation
        .constraints
        .prohibited_actions
        .iter()
        .any(|action| action == "push")
    {
        return Err(anyhow!(
            "ACIP review invocation specialization requires constraints.prohibited_actions to include 'push'"
        ));
    }
    validate_review_independence_policy(&contract.independence_policy)?;
    validate_review_disposition_contract(&contract.disposition_contract)?;
    Ok(())
}

pub fn validate_acip_review_outcome_v1(
    contract: &AcipReviewInvocationContractV1,
    outcome: &AcipReviewOutcomeV1,
) -> Result<()> {
    validate_acip_review_invocation_contract_v1(contract)?;
    if outcome.schema_version != ACIP_REVIEW_OUTCOME_SCHEMA_VERSION {
        return Err(anyhow!(
            "ACIP review outcome requires schema_version '{}'",
            ACIP_REVIEW_OUTCOME_SCHEMA_VERSION
        ));
    }
    validate_id(&outcome.invocation_id, "invocation_id")?;
    validate_repo_relative_ref(&outcome.review_result_ref, "review_result_ref")?;
    validate_repo_relative_ref(&outcome.gate_result_ref, "gate_result_ref")?;
    validate_id(&outcome.reviewer_session_id, "reviewer_session_id")?;
    validate_id(&outcome.reviewer_model_ref, "reviewer_model_ref")?;
    if contract.disposition_contract.gate_result_required && outcome.gate_result_ref.is_empty() {
        return Err(anyhow!(
            "ACIP review outcome requires gate_result_ref when gate_result_required is true"
        ));
    }
    if contract.disposition_contract.findings_required_when_blocked
        && outcome.disposition == AcipReviewDispositionV1::Blocked
        && outcome.findings_ref.is_none()
    {
        return Err(anyhow!(
            "blocked review outcome must carry findings_ref under the disposition contract"
        ));
    }
    if let Some(findings_ref) = &outcome.findings_ref {
        validate_repo_relative_ref(findings_ref, "findings_ref")?;
    }
    for reference in &outcome.residual_risk_refs {
        validate_repo_relative_ref(reference, "residual_risk_refs[]")?;
    }
    if contract.invocation.invocation_id != outcome.invocation_id {
        return Err(anyhow!(
            "review outcome must match the invocation contract invocation_id"
        ));
    }
    if contract.independence_policy.reviewer_session_id != outcome.reviewer_session_id {
        return Err(anyhow!(
            "review outcome must preserve the reviewer_session_id declared by the invocation contract"
        ));
    }
    if contract.independence_policy.reviewer_model_ref != outcome.reviewer_model_ref {
        return Err(anyhow!(
            "review outcome must preserve the reviewer_model_ref declared by the invocation contract"
        ));
    }
    if !contract
        .invocation
        .expected_outputs
        .iter()
        .any(|output| output.required && output.output_kind == "review_result")
    {
        return Err(anyhow!(
            "review invocation specialization requires expected_outputs to declare required review_result output"
        ));
    }
    if !contract
        .invocation
        .expected_outputs
        .iter()
        .any(|output| output.required && output.output_kind == "gate_result")
    {
        return Err(anyhow!(
            "review invocation specialization requires expected_outputs to declare required gate_result output"
        ));
    }
    if !outcome.review_result_ref.ends_with("review_result.json") {
        return Err(anyhow!(
            "review outcome review_result_ref must match the declared review_result output contract"
        ));
    }
    if !outcome.gate_result_ref.ends_with("gate_result.json") {
        return Err(anyhow!(
            "review outcome gate_result_ref must match the declared gate_result output contract"
        ));
    }

    let expected_handoff = match outcome.disposition {
        AcipReviewDispositionV1::Blessed => &contract.disposition_contract.blessed_handoff,
        AcipReviewDispositionV1::Blocked => &contract.disposition_contract.blocked_handoff,
        AcipReviewDispositionV1::NonProving => &contract.disposition_contract.non_proving_handoff,
        AcipReviewDispositionV1::Skipped => &contract.disposition_contract.skipped_handoff,
    };
    if &outcome.handoff_outcome != expected_handoff {
        return Err(anyhow!(
            "review outcome handoff must match the disposition contract mapping"
        ));
    }
    if outcome.pr_open_allowed {
        if outcome.disposition != AcipReviewDispositionV1::Blessed {
            return Err(anyhow!("only blessed review outcomes may allow PR finish"));
        }
        if outcome.handoff_outcome != AcipReviewHandoffOutcomeV1::AllowPrFinish {
            return Err(anyhow!(
                "review outcome allowing PR finish must use handoff outcome 'allow_pr_finish'"
            ));
        }
    } else if outcome.disposition == AcipReviewDispositionV1::Blessed
        && outcome.handoff_outcome == AcipReviewHandoffOutcomeV1::AllowPrFinish
    {
        return Err(anyhow!(
            "blessed review outcome with allow_pr_finish handoff must set pr_open_allowed"
        ));
    }

    Ok(())
}

pub fn validate_acip_review_fixture_set_v1(fixtures: &AcipReviewFixtureSetV1) -> Result<()> {
    if fixtures.schema_version != ACIP_REVIEW_FIXTURE_SCHEMA_VERSION {
        return Err(anyhow!(
            "ACIP review fixture set requires schema_version '{}'",
            ACIP_REVIEW_FIXTURE_SCHEMA_VERSION
        ));
    }
    if fixtures.negative_cases.is_empty() {
        return Err(anyhow!("ACIP review fixture set requires negative_cases"));
    }
    validate_acip_review_invocation_contract_v1(&fixtures.valid_contract)?;
    validate_acip_review_outcome_v1(&fixtures.valid_contract, &fixtures.valid_blessed_outcome)?;
    validate_acip_review_outcome_v1(&fixtures.valid_contract, &fixtures.valid_blocked_outcome)?;
    for case in &fixtures.negative_cases {
        validate_negative_case_name(&case.name, "negative_cases[].name")?;
        validate_non_empty(
            &case.expected_error_substring,
            "negative_cases[].expected_error_substring",
        )?;
        let contract_result = validate_acip_review_invocation_contract_v1_value(&case.contract);
        match &case.outcome {
            Some(outcome_value) => match contract_result {
                Ok(contract) => validate_negative_result(
                    validate_acip_review_outcome_v1_value(&contract, outcome_value).map(|_| ()),
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


pub fn acip_review_fixture_set_v1() -> AcipReviewFixtureSetV1 {
    let valid_contract = sample_review_invocation_contract();
    AcipReviewFixtureSetV1 {
        schema_version: ACIP_REVIEW_FIXTURE_SCHEMA_VERSION.to_string(),
        valid_blessed_outcome: sample_review_outcome(
            &valid_contract,
            AcipReviewDispositionV1::Blessed,
            AcipReviewHandoffOutcomeV1::AllowPrFinish,
            Some("runtime/comms/review/findings/clean_review.json".to_string()),
            vec!["runtime/comms/review/residual_risk/none.json".to_string()],
            true,
        ),
        valid_blocked_outcome: sample_review_outcome(
            &valid_contract,
            AcipReviewDispositionV1::Blocked,
            AcipReviewHandoffOutcomeV1::FixFindingsAndRerunReview,
            Some("runtime/comms/review/findings/blocking_findings.json".to_string()),
            vec!["runtime/comms/review/residual_risk/needs_fix.json".to_string()],
            false,
        ),
        negative_cases: vec![
            AcipReviewNegativeCaseV1 {
                name: "missing_srp_policy_link_rejected".to_string(),
                expected_error_substring: "requires constraints.policy_refs to include srp_ref"
                    .to_string(),
                contract: {
                    let mut value =
                        serde_json::to_value(sample_review_invocation_contract()).expect("json");
                    value["invocation"]["constraints"]["policy_refs"] =
                        json!(["runtime/comms/policy/review_policy.json"]);
                    value
                },
                outcome: None,
            },
            AcipReviewNegativeCaseV1 {
                name: "same_session_self_review_rejected".to_string(),
                expected_error_substring:
                    "review invocation independence policy forbids same-session review".to_string(),
                contract: {
                    let mut value =
                        serde_json::to_value(sample_review_invocation_contract()).expect("json");
                    value["independence_policy"]["reviewer_session_id"] =
                        json!("writer-session-codex");
                    value
                },
                outcome: None,
            },
            AcipReviewNegativeCaseV1 {
                name: "same_model_self_blessing_rejected".to_string(),
                expected_error_substring:
                    "review invocation independence policy forbids same-model review".to_string(),
                contract: {
                    let mut value =
                        serde_json::to_value(sample_review_invocation_contract()).expect("json");
                    value["independence_policy"]["reviewer_model_ref"] = json!("gpt-5-codex");
                    value
                },
                outcome: None,
            },
            AcipReviewNegativeCaseV1 {
                name: "merge_authority_gap_rejected".to_string(),
                expected_error_substring:
                    "requires constraints.prohibited_actions to include 'merge'".to_string(),
                contract: {
                    let mut value =
                        serde_json::to_value(sample_review_invocation_contract()).expect("json");
                    value["invocation"]["constraints"]["prohibited_actions"] =
                        json!(["push", "destructive_edit"]);
                    value
                },
                outcome: None,
            },
            AcipReviewNegativeCaseV1 {
                name: "blocked_outcome_requires_findings_ref".to_string(),
                expected_error_substring: "blocked review outcome must carry findings_ref"
                    .to_string(),
                contract: serde_json::to_value(sample_review_invocation_contract()).expect("json"),
                outcome: Some({
                    let mut value = serde_json::to_value(sample_review_outcome(
                        &valid_contract,
                        AcipReviewDispositionV1::Blocked,
                        AcipReviewHandoffOutcomeV1::FixFindingsAndRerunReview,
                        Some("runtime/comms/review/findings/blocking_findings.json".to_string()),
                        vec!["runtime/comms/review/residual_risk/needs_fix.json".to_string()],
                        false,
                    ))
                    .expect("json");
                    value["findings_ref"] = JsonValue::Null;
                    value
                }),
            },
            AcipReviewNegativeCaseV1 {
                name: "non_blessed_outcome_cannot_allow_pr_finish".to_string(),
                expected_error_substring: "only blessed review outcomes may allow PR finish"
                    .to_string(),
                contract: serde_json::to_value(sample_review_invocation_contract()).expect("json"),
                outcome: Some(
                    serde_json::to_value(sample_review_outcome(
                        &valid_contract,
                        AcipReviewDispositionV1::Blocked,
                        AcipReviewHandoffOutcomeV1::FixFindingsAndRerunReview,
                        Some("runtime/comms/review/findings/blocking_findings.json".to_string()),
                        vec!["runtime/comms/review/residual_risk/needs_fix.json".to_string()],
                        true,
                    ))
                    .expect("json"),
                ),
            },
        ],
        valid_contract,
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


fn validate_review_independence_policy(policy: &AcipReviewIndependencePolicyV1) -> Result<()> {
    validate_id(
        &policy.writer_session_id,
        "independence_policy.writer_session_id",
    )?;
    validate_id(
        &policy.writer_model_ref,
        "independence_policy.writer_model_ref",
    )?;
    validate_id(
        &policy.reviewer_session_id,
        "independence_policy.reviewer_session_id",
    )?;
    validate_id(
        &policy.reviewer_model_ref,
        "independence_policy.reviewer_model_ref",
    )?;
    if policy.forbid_same_session && policy.writer_session_id == policy.reviewer_session_id {
        return Err(anyhow!(
            "review invocation independence policy forbids same-session review"
        ));
    }
    if policy.forbid_same_model_ref && policy.writer_model_ref == policy.reviewer_model_ref {
        return Err(anyhow!(
            "review invocation independence policy forbids same-model review"
        ));
    }
    Ok(())
}

fn validate_review_disposition_contract(contract: &AcipReviewDispositionContractV1) -> Result<()> {
    if contract.allowed_dispositions.is_empty() {
        return Err(anyhow!(
            "review disposition contract requires allowed_dispositions"
        ));
    }
    let mut seen = BTreeSet::new();
    for disposition in &contract.allowed_dispositions {
        if !seen.insert(disposition.clone()) {
            return Err(anyhow!(
                "review disposition contract contains duplicate allowed_dispositions entries"
            ));
        }
    }
    for required in [
        AcipReviewDispositionV1::Blessed,
        AcipReviewDispositionV1::Blocked,
        AcipReviewDispositionV1::NonProving,
        AcipReviewDispositionV1::Skipped,
    ] {
        if !seen.contains(&required) {
            return Err(anyhow!(
                "review disposition contract must include every canonical review disposition"
            ));
        }
    }
    if contract.blessed_handoff != AcipReviewHandoffOutcomeV1::AllowPrFinish {
        return Err(anyhow!(
            "review disposition contract must map blessed review to allow_pr_finish"
        ));
    }
    if contract.blocked_handoff == AcipReviewHandoffOutcomeV1::AllowPrFinish
        || contract.non_proving_handoff == AcipReviewHandoffOutcomeV1::AllowPrFinish
        || contract.skipped_handoff == AcipReviewHandoffOutcomeV1::AllowPrFinish
    {
        return Err(anyhow!(
            "only blessed review may map to allow_pr_finish in the disposition contract"
        ));
    }
    Ok(())
}
