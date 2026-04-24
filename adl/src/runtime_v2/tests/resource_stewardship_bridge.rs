use super::*;

#[test]
fn runtime_v2_resource_stewardship_bridge_is_stable() {
    let artifact =
        runtime_v2_resource_stewardship_bridge().expect("resource stewardship bridge artifact");
    artifact
        .validate()
        .expect("valid resource stewardship bridge artifact");

    assert_eq!(
        artifact.schema_version,
        RUNTIME_V2_RESOURCE_STEWARDSHIP_BRIDGE_SCHEMA
    );
    assert_eq!(artifact.demo_id, "D8");
    assert_eq!(artifact.wp_id, "WP-10");
    assert_eq!(artifact.contract_resource_claims.len(), 7);
    assert_eq!(artifact.bid_resource_estimates.len(), 2);
}

#[test]
#[cfg(feature = "slow-proof-tests")]
fn runtime_v2_resource_stewardship_bridge_matches_golden_fixture() {
    let artifact =
        runtime_v2_resource_stewardship_bridge().expect("resource stewardship bridge artifact");
    let json = String::from_utf8(
        artifact
            .pretty_json_bytes()
            .expect("resource stewardship bridge json"),
    )
    .expect("utf8 resource stewardship bridge json");

    assert_eq!(
        json,
        include_str!(
            "../../../tests/fixtures/runtime_v2/contract_market/resource_stewardship_bridge.json"
        )
        .trim_end()
    );
}

#[test]
fn runtime_v2_resource_stewardship_bridge_preserves_policy_and_tool_boundaries() {
    let artifact =
        runtime_v2_resource_stewardship_bridge().expect("resource stewardship bridge artifact");

    let policy_domains = artifact
        .policy_bindings
        .iter()
        .map(|binding| binding.policy_domain.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(
        policy_domains,
        std::collections::BTreeSet::from([
            "standing",
            "access_control",
            "quarantine",
            "sanctuary",
            "challenge",
        ])
    );
    assert!(artifact
        .boundary_notes
        .iter()
        .any(|note| note.contains("Payment and pricing remain explicitly out of scope")));
    assert!(artifact
        .boundary_notes
        .iter()
        .any(|note| note.contains("v0.90.5 governed-tool authority")));
    assert!(artifact
        .bid_resource_estimates
        .iter()
        .flat_map(|estimate| &estimate.tool_resource_constraints)
        .all(|constraint| constraint.governed_authority_required));
    assert!(artifact
        .bid_resource_estimates
        .iter()
        .flat_map(|estimate| &estimate.tool_resource_constraints)
        .all(|constraint| !constraint.execution_authority_granted));
}

#[test]
fn runtime_v2_resource_stewardship_bridge_preserves_existing_bid_claims() {
    let artifact =
        runtime_v2_resource_stewardship_bridge().expect("resource stewardship bridge artifact");
    let bid_artifacts = runtime_v2_bid_schema_contract().expect("bid schema artifacts");

    for bid in &bid_artifacts.valid_bids {
        let estimate = artifact
            .bid_resource_estimates
            .iter()
            .find(|estimate| estimate.bid_id == bid.bid_id)
            .expect("matching bid estimate");
        for original_claim in &bid.resource_claims {
            let bridge_claim = estimate
                .claims
                .iter()
                .find(|claim| claim.resource_kind == original_claim.resource_kind)
                .expect("overlapping claim present");
            assert_eq!(bridge_claim.estimated_units, original_claim.estimated_units);
            assert_eq!(bridge_claim.unit_basis, original_claim.unit_basis);
        }
    }
}

#[test]
fn runtime_v2_resource_stewardship_bridge_rejects_policy_and_authority_drift() {
    let artifact =
        runtime_v2_resource_stewardship_bridge().expect("resource stewardship bridge artifact");
    let contract = runtime_v2_contract_schema_contract()
        .expect("contract schema artifacts")
        .contract;
    let selection =
        RuntimeV2EvaluationSelectionArtifacts::prototype().expect("selection artifacts");
    let delegation = runtime_v2_delegation_subcontract_model().expect("delegation artifacts");

    let mut missing_challenge = artifact.clone();
    missing_challenge
        .policy_bindings
        .retain(|binding| binding.policy_domain != "challenge");
    assert!(missing_challenge
        .validate_against(&contract, &selection, &delegation)
        .expect_err("missing challenge policy binding should fail")
        .to_string()
        .contains("must include standing, access_control, quarantine, sanctuary, and challenge"));

    let mut pricing_leak = artifact.clone();
    pricing_leak
        .boundary_notes
        .retain(|note| !note.contains("Payment and pricing remain explicitly out of scope"));
    assert!(pricing_leak
        .validate_against(&contract, &selection, &delegation)
        .expect_err("pricing note drift should fail")
        .to_string()
        .contains("must keep payment and pricing out of scope"));

    let mut tool_authority = artifact.clone();
    tool_authority.bid_resource_estimates[0].tool_resource_constraints[0]
        .execution_authority_granted = true;
    assert!(tool_authority
        .validate_against(&contract, &selection, &delegation)
        .expect_err("tool authority drift should fail")
        .to_string()
        .contains("execution_authority_granted must stay false"));

    let mut overlapping_claim_drift = artifact.clone();
    let alpha_claim = overlapping_claim_drift.bid_resource_estimates[0]
        .claims
        .iter_mut()
        .find(|claim| claim.resource_kind == "operator_review_minutes")
        .expect("alpha operator claim");
    alpha_claim.estimated_units = 99;
    assert!(overlapping_claim_drift
        .validate_against(&contract, &selection, &delegation)
        .expect_err("overlapping claim drift should fail")
        .to_string()
        .contains("must preserve overlapping bid resource claim"));
}

#[test]
#[cfg(feature = "slow-proof-tests")]
fn runtime_v2_resource_stewardship_bridge_write_to_root_materializes_fixture() {
    let artifact =
        runtime_v2_resource_stewardship_bridge().expect("resource stewardship bridge artifact");
    let fixture_refresh_root = std::env::var("ADL_RUNTIME_V2_WRITE_ROOT").ok();
    let root = fixture_refresh_root
        .as_ref()
        .map(PathBuf::from)
        .unwrap_or_else(|| common::unique_temp_path("resource-stewardship-bridge-write"));

    artifact
        .write_to_root(&root)
        .expect("write resource stewardship bridge artifact");

    let text = std::fs::read_to_string(root.join(RUNTIME_V2_RESOURCE_STEWARDSHIP_BRIDGE_PATH))
        .expect("resource stewardship bridge text");
    assert!(text.contains("D8"));
    assert!(text.contains("Payment and pricing remain explicitly out of scope"));
    assert!(!text.contains(root.to_string_lossy().as_ref()));

    if fixture_refresh_root.is_none() {
        std::fs::remove_dir_all(root).expect("cleanup resource stewardship bridge temp root");
    }
}
