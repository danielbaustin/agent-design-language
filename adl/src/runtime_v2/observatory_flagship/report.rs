use super::*;

pub(super) fn render_observatory_flagship_operator_report(
    proof: &RuntimeV2ObservatoryFlagshipProofPacket,
    challenge: &RuntimeV2ContinuityChallengeArtifacts,
) -> Result<String> {
    proof.validate_shape()?;
    challenge.validate()?;
    let mut report = format!(
        concat!(
            "# D12 Inhabited CSM Observatory Flagship\n\n",
            "Proof classification: `{}`\n\n",
            "Primary proof packet: `{}`\n\n",
            "Reviewer command: `{}`\n\n",
            "Citizen continuity basis:\n",
            "- witness set: `{}`\n",
            "- citizen receipt set: `{}`\n",
            "- redacted projection: `{}`\n",
            "- continuity challenge: `{}`\n",
            "- sanctuary/quarantine: `{}`\n\n",
            "Sprint 3 runtime/comms bindings:\n",
            "- lifecycle state contract: `{}`\n",
            "- lifecycle transition matrix: `{}`\n",
            "- ACIP hardening packet: `{}`\n",
            "- A2A adapter boundary packet: `{}`\n",
            "- runtime inhabitant integration packet: `{}`\n\n",
            "Operator-facing result: the Observatory can explain why the citizen-state scenario is reviewable, which authority paths are refused, and which ambiguous continuity transition is frozen without exposing canonical private state.\n\n",
            "Non-claims: personhood, first true Godel-agent birthday, raw private-state inspection, and unbounded live Runtime v2 execution remain outside this proof.\n"
        ),
        proof.proof_classification,
        proof.artifact_path,
        proof.reviewer_command,
        challenge.access_control_artifacts.observatory_artifacts.witness_artifacts.witness_set.artifact_path,
        challenge.access_control_artifacts.observatory_artifacts.witness_artifacts.receipt_set.artifact_path,
        challenge.access_control_artifacts.observatory_artifacts.projection_packet.artifact_path,
        challenge.challenge.artifact_path,
        challenge.sanctuary_artifacts.quarantine_artifact.artifact_path,
        proof.lifecycle_refs[0],
        proof.lifecycle_refs[1],
        proof.communication_boundary_refs[0],
        proof.communication_boundary_refs[1],
        proof.runtime_inhabitant_refs[0],
    );
    report.push_str("\nFeature demo coverage:\n");
    for feature in &proof.feature_demo_coverage {
        report.push_str(&format!(
            "- `{}` {} [{} / {}]\n  surfaces: {}\n  summary: {}\n",
            feature.feature_id,
            feature.feature_name,
            feature.owning_wp,
            feature.demo_mode,
            feature.demo_surface_refs.join(", "),
            feature.coverage_summary
        ));
    }
    Ok(report)
}

pub(super) fn validate_flagship_operator_report(
    proof: &RuntimeV2ObservatoryFlagshipProofPacket,
    report: &str,
) -> Result<()> {
    proof.validate_shape()?;
    validate_nonempty_text(report, "observatory_flagship.operator_report")?;
    for required in [
        "D12 Inhabited CSM Observatory Flagship",
        proof.artifact_path.as_str(),
        proof.reviewer_command.as_str(),
        "witness set",
        "citizen receipt set",
        "redacted projection",
        "continuity challenge",
        "sanctuary/quarantine",
        "lifecycle state contract",
        "lifecycle transition matrix",
        "ACIP hardening packet",
        "A2A adapter boundary packet",
        "runtime inhabitant integration packet",
        "Feature demo coverage",
        "Non-claims",
    ] {
        if !report.contains(required) {
            return Err(anyhow!(
                "observatory flagship operator report missing required text '{required}'"
            ));
        }
    }
    for forbidden in [
        "private_payload_b64",
        "sealed_payload_b64",
        "section_digests",
    ] {
        if report.contains(forbidden) {
            return Err(anyhow!(
                "observatory flagship operator report leaked forbidden private-state token"
            ));
        }
    }
    Ok(())
}
