use super::*;

pub(crate) fn render_governed_tools_flagship_operator_report(
    proof_packet: &RuntimeV2GovernedToolsFlagshipProofPacket,
    cases: &[RuntimeV2GovernedToolsFlagshipCase],
) -> Result<String> {
    let mut lines = vec![
        "# D11 Governed Tools v1.0 Flagship Demo".to_string(),
        String::new(),
        format!(
            "Proof classification: `{}`",
            proof_packet.proof_classification
        ),
        String::new(),
        proof_packet.proof_summary.clone(),
        String::new(),
        "## Cases".to_string(),
    ];
    for case in cases {
        lines.push(format!(
            "- `{}`: compiler `{}`, gate `{}`, executor `{}`",
            case.case_id,
            case.compiler_decision,
            case.gate_decision
                .clone()
                .unwrap_or_else(|| "not_reached".to_string()),
            case.executor_outcome,
        ));
        lines.push(format!(
            "  reviewer view: {}",
            case.reviewer_visible_outcome
        ));
    }
    lines.extend([
        String::new(),
        "## Support Reports".to_string(),
        format!(
            "- model benchmark: `{}`",
            RUNTIME_V2_GOVERNED_TOOLS_FLAGSHIP_MODEL_BENCHMARK_REF
        ),
        format!(
            "- dangerous negative suite: `{}`",
            RUNTIME_V2_GOVERNED_TOOLS_FLAGSHIP_NEGATIVE_SUITE_REF
        ),
        String::new(),
        "## Review Command".to_string(),
        format!("`{}`", proof_packet.reviewer_command),
        String::new(),
        "## Non-claims".to_string(),
    ]);
    for non_claim in &proof_packet.non_claims {
        lines.push(format!("- {}", non_claim));
    }
    let report = lines.join("\n");
    validate_nonempty_text(&report, "governed_tools_flagship.operator_report_markdown")?;
    Ok(report)
}

pub(crate) fn render_governed_tools_flagship_public_report(
    proof_packet: &RuntimeV2GovernedToolsFlagshipProofPacket,
    cases: &[RuntimeV2GovernedToolsFlagshipCase],
) -> Result<String> {
    let mut lines = vec![
        "# D11 Governed Tools Public Proof Summary".to_string(),
        String::new(),
        "This public summary keeps the review story visible while preserving redaction boundaries."
            .to_string(),
        String::new(),
        "## Public Outcomes".to_string(),
    ];
    for case in cases {
        lines.push(format!(
            "- `{}`: {}",
            case.case_id, case.public_redaction_outcome
        ));
    }
    lines.extend([
        String::new(),
        "## Non-claims".to_string(),
        proof_packet
            .non_claims
            .iter()
            .map(|claim| format!("- {}", claim))
            .collect::<Vec<_>>()
            .join("\n"),
    ]);
    let report = lines.join("\n");
    validate_nonempty_text(&report, "governed_tools_flagship.public_report_markdown")?;
    Ok(report)
}

pub(crate) fn validate_governed_tools_flagship_operator_report(
    proof_packet: &RuntimeV2GovernedToolsFlagshipProofPacket,
    cases: &[RuntimeV2GovernedToolsFlagshipCase],
    report: &str,
) -> Result<()> {
    validate_nonempty_text(report, "governed_tools_flagship.operator_report")?;
    for required in [
        "D11 Governed Tools v1.0 Flagship Demo",
        proof_packet.proof_summary.as_str(),
        proof_packet.reviewer_command.as_str(),
        "Support Reports",
    ] {
        if !report.contains(required) {
            return Err(anyhow!(
                "governed-tools flagship operator report must contain {required}"
            ));
        }
    }
    for case in cases {
        for required in [
            case.case_id.as_str(),
            case.reviewer_visible_outcome.as_str(),
        ] {
            if !report.contains(required) {
                return Err(anyhow!(
                    "governed-tools flagship operator report must preserve {required}"
                ));
            }
        }
    }
    Ok(())
}

pub(crate) fn validate_governed_tools_flagship_public_report(
    proof_packet: &RuntimeV2GovernedToolsFlagshipProofPacket,
    cases: &[RuntimeV2GovernedToolsFlagshipCase],
    report: &str,
) -> Result<()> {
    validate_nonempty_text(report, "governed_tools_flagship.public_report")?;
    for forbidden in [
        "/Users/",
        "/home/",
        "/tmp/",
        "C:\\\\",
        "{{system prompt}}",
        "sk-live-dangerous-secret",
    ] {
        if report.contains(forbidden) {
            return Err(anyhow!(
                "governed-tools flagship public report must not contain {forbidden}"
            ));
        }
    }
    if !report.contains("Public Outcomes") || !report.contains("Non-claims") {
        return Err(anyhow!(
            "governed-tools flagship public report must preserve public outcomes and non-claims"
        ));
    }
    if !report.contains(&proof_packet.non_claims[0]) {
        return Err(anyhow!(
            "governed-tools flagship public report must preserve bounded non-claims"
        ));
    }
    for case in cases {
        for required in [
            case.case_id.as_str(),
            case.public_redaction_outcome.as_str(),
        ] {
            if !report.contains(required) {
                return Err(anyhow!(
                    "governed-tools flagship public report must preserve {required}"
                ));
            }
        }
    }
    Ok(())
}
