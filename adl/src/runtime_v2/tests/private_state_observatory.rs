use super::*;

#[test]
fn runtime_v2_private_state_observatory_contract_is_stable() {
    let artifacts = runtime_v2_private_state_observatory_contract().expect("observatory artifacts");
    artifacts.validate().expect("valid observatory artifacts");

    assert_eq!(
        artifacts.redaction_policy.schema_version,
        RUNTIME_V2_PRIVATE_STATE_OBSERVATORY_POLICY_SCHEMA
    );
    assert_eq!(
        artifacts.projection_packet.schema_version,
        RUNTIME_V2_PRIVATE_STATE_OBSERVATORY_PACKET_SCHEMA
    );
    assert_eq!(artifacts.projection_packet.demo_id, "D9");
    assert_eq!(
        artifacts.projection_packet.projection_authority_status,
        "non_authoritative_review_projection"
    );
    assert_eq!(artifacts.projection_packet.projections.len(), 4);
}

#[test]
fn runtime_v2_private_state_observatory_policy_matches_golden_fixture() {
    let artifacts = runtime_v2_private_state_observatory_contract().expect("observatory artifacts");
    let json = String::from_utf8(
        artifacts
            .redaction_policy
            .pretty_json_bytes()
            .expect("policy json"),
    )
    .expect("utf8 policy");

    assert_eq!(
        json,
        include_str!(
            "../../../tests/fixtures/runtime_v2/observatory/private_state_redaction_policy.json"
        )
        .trim_end()
    );
}

#[test]
fn runtime_v2_private_state_observatory_packet_matches_golden_fixture() {
    let artifacts = runtime_v2_private_state_observatory_contract().expect("observatory artifacts");
    let json = String::from_utf8(
        artifacts
            .projection_packet
            .pretty_json_bytes()
            .expect("packet json"),
    )
    .expect("utf8 packet");

    assert_eq!(
        json,
        include_str!(
            "../../../tests/fixtures/runtime_v2/observatory/private_state_projection_packet.json"
        )
        .trim_end()
    );
}

#[test]
fn runtime_v2_private_state_observatory_report_matches_golden_fixture() {
    let artifacts = runtime_v2_private_state_observatory_contract().expect("observatory artifacts");

    assert_eq!(
        artifacts.operator_report_markdown.trim_end(),
        include_str!(
            "../../../tests/fixtures/runtime_v2/observatory/private_state_projection_report.md"
        )
        .trim_end()
    );
}

#[test]
fn runtime_v2_private_state_observatory_negative_cases_match_golden_fixture() {
    let artifacts = runtime_v2_private_state_observatory_contract().expect("observatory artifacts");
    let json = String::from_utf8(
        artifacts
            .negative_cases
            .pretty_json_bytes()
            .expect("negative cases json"),
    )
    .expect("utf8 negative cases");

    assert_eq!(
        json,
        include_str!(
            "../../../tests/fixtures/runtime_v2/observatory/private_state_projection_negative_cases.json"
        )
        .trim_end()
    );
}

#[test]
fn runtime_v2_private_state_observatory_redacts_all_audience_views() {
    let artifacts = runtime_v2_private_state_observatory_contract().expect("observatory artifacts");
    for projection in &artifacts.projection_packet.projections {
        assert!(!projection.raw_private_state_present);
        assert!(!projection.projection_authoritative);
        assert!(projection
            .redacted_fields
            .contains(&"raw_private_state".to_string()));
        assert!(projection
            .denied_actions
            .contains(&"inspect_raw_private_state".to_string()));
        assert_eq!(projection.allowed_actions, vec!["read_only_projection"]);
    }
    let public = artifacts
        .projection_packet
        .projections
        .iter()
        .find(|projection| projection.audience == "public")
        .expect("public projection");
    assert!(!public.visible_fields.contains(&"lineage_id".to_string()));
    assert!(!public
        .visible_fields
        .contains(&"source_state_hash".to_string()));
}

#[test]
fn runtime_v2_private_state_observatory_rejects_raw_state_leakage_and_authority() {
    let artifacts = runtime_v2_private_state_observatory_contract().expect("observatory artifacts");

    let mut leaked = artifacts.projection_packet.clone();
    let leak_token = artifacts.redaction_policy.leakage_probe_tokens[1].clone();
    leaked.projections[0].visible_summary.push(leak_token);
    leaked.packet_hash = leaked.computed_hash().expect("leaked packet hash");
    assert!(leaked
        .validate_against(
            &artifacts.redaction_policy,
            &artifacts.private_state_artifacts,
            &artifacts.witness_artifacts,
            &artifacts.sanctuary_artifacts,
        )
        .expect_err("raw private token leak should fail")
        .to_string()
        .contains("raw private-state token leak"));

    let mut authoritative = artifacts.projection_packet.clone();
    authoritative.projections[0].projection_authoritative = true;
    authoritative.packet_hash = authoritative
        .computed_hash()
        .expect("authoritative packet hash");
    assert!(authoritative
        .validate_against(
            &artifacts.redaction_policy,
            &artifacts.private_state_artifacts,
            &artifacts.witness_artifacts,
            &artifacts.sanctuary_artifacts,
        )
        .expect_err("authoritative projection should fail")
        .to_string()
        .contains("raw state or authority"));

    let mut debug_policy = artifacts.redaction_policy.clone();
    debug_policy.audiences[3].raw_private_state_allowed = true;
    assert!(debug_policy
        .validate_against(&artifacts.private_state_artifacts)
        .expect_err("debug raw-state allowance should fail")
        .to_string()
        .contains("raw state or authority"));
}

#[test]
fn runtime_v2_private_state_observatory_rejects_public_overexposure_and_report_drift() {
    let artifacts = runtime_v2_private_state_observatory_contract().expect("observatory artifacts");

    let mut public_overexposed = artifacts.projection_packet.clone();
    let mut overexposed_policy = artifacts.redaction_policy.clone();
    let public_policy = overexposed_policy
        .audiences
        .iter_mut()
        .find(|projection| projection.audience == "public")
        .expect("public policy");
    public_policy.allowed_fields.push("lineage_id".to_string());
    let public = public_overexposed
        .projections
        .iter_mut()
        .find(|projection| projection.audience == "public")
        .expect("public projection");
    public.visible_fields.push("lineage_id".to_string());
    public_overexposed.packet_hash = public_overexposed
        .computed_hash()
        .expect("public overexposed packet hash");
    assert!(public_overexposed
        .validate_against(
            &overexposed_policy,
            &artifacts.private_state_artifacts,
            &artifacts.witness_artifacts,
            &artifacts.sanctuary_artifacts,
        )
        .expect_err("public overexposure should fail")
        .to_string()
        .contains("public Observatory projection must stay minimal"));

    let report = format!(
        "{}\nraw private-state inspection is allowed\n",
        artifacts.operator_report_markdown
    );
    assert!(validate_private_state_observatory_report_for_test_only(
        &artifacts.projection_packet,
        &report
    )
    .expect_err("report drift should fail")
    .to_string()
    .contains("must not claim raw private-state inspection"));
}

#[cfg(feature = "slow-proof-tests")]
#[test]
fn runtime_v2_private_state_observatory_write_to_root_materializes_fixtures() {
    let artifacts = runtime_v2_private_state_observatory_contract().expect("observatory artifacts");
    let root = common::unique_temp_path("private-state-observatory-write");

    artifacts
        .write_to_root(&root)
        .expect("write observatory artifacts");

    for rel_path in [
        RUNTIME_V2_PRIVATE_STATE_OBSERVATORY_POLICY_PATH,
        RUNTIME_V2_PRIVATE_STATE_OBSERVATORY_PACKET_PATH,
        RUNTIME_V2_PRIVATE_STATE_OBSERVATORY_REPORT_PATH,
        RUNTIME_V2_PRIVATE_STATE_OBSERVATORY_PROOF_PATH,
    ] {
        let text = std::fs::read_to_string(root.join(rel_path)).expect("artifact text");
        assert!(text.contains("D9") || text.contains("Private-State Observatory"));
        assert!(text.contains("first true Godel-agent birth"));
        assert!(!text.contains(root.to_string_lossy().as_ref()));
    }

    std::fs::remove_dir_all(root).expect("cleanup observatory temp root");
}
