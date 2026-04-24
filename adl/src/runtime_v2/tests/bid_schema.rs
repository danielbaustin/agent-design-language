use super::*;

#[test]
fn runtime_v2_bid_schema_contract_is_stable() {
    let artifacts = runtime_v2_bid_schema_contract().expect("bid schema artifacts");
    artifacts.validate().expect("valid bid schema artifacts");

    assert_eq!(artifacts.valid_bids.len(), 2);
    assert_eq!(artifacts.valid_bids[0].demo_id, "D3");
    assert_eq!(artifacts.valid_bids[1].demo_id, "D3");
    assert_eq!(artifacts.negative_cases.required_negative_cases.len(), 7);
}

#[test]
fn runtime_v2_bid_schema_matches_golden_fixtures() {
    let artifacts = runtime_v2_bid_schema_contract().expect("bid schema artifacts");

    let alpha = String::from_utf8(
        artifacts.valid_bids[0]
            .pretty_json_bytes()
            .expect("alpha bid json"),
    )
    .expect("utf8 alpha");
    let bravo = String::from_utf8(
        artifacts.valid_bids[1]
            .pretty_json_bytes()
            .expect("bravo bid json"),
    )
    .expect("utf8 bravo");

    assert_eq!(
        alpha,
        include_str!("../../../tests/fixtures/runtime_v2/contract_market/bid_alpha.json")
            .trim_end()
    );
    assert_eq!(
        bravo,
        include_str!("../../../tests/fixtures/runtime_v2/contract_market/bid_bravo.json")
            .trim_end()
    );
}

#[test]
fn runtime_v2_bid_schema_negative_cases_match_golden_fixture() {
    let artifacts = runtime_v2_bid_schema_contract().expect("bid schema artifacts");
    let json = String::from_utf8(
        artifacts
            .negative_cases
            .pretty_json_bytes()
            .expect("negative cases json"),
    )
    .expect("utf8 negative cases");

    assert_eq!(
        json,
        include_str!("../../../tests/fixtures/runtime_v2/contract_market/bid_negative_cases.json")
            .trim_end()
    );
}

#[test]
fn runtime_v2_bid_schema_preserves_pricing_and_governed_tool_boundaries() {
    let artifacts = runtime_v2_bid_schema_contract().expect("bid schema artifacts");

    for bid in &artifacts.valid_bids {
        assert!(bid.extension_slots.iter().any(|slot| slot == "pricing"));
        assert!(bid
            .extension_slots
            .iter()
            .any(|slot| slot == "payment_rails"));
        assert!(bid.claim_boundary.contains("does not settle payment"));
        assert!(bid
            .claim_boundary
            .contains("governed-tool execution authority"));
        assert!(bid
            .expected_tool_usage
            .iter()
            .all(|usage| !usage.direct_execution_allowed));
    }
}

#[test]
fn runtime_v2_bid_schema_valid_bids_bind_to_parent_contract() {
    let artifacts = runtime_v2_bid_schema_contract().expect("bid schema artifacts");

    for bid in &artifacts.valid_bids {
        assert_eq!(bid.target_contract_ref, artifacts.contract.artifact_path);
        assert_eq!(bid.target_contract_id, artifacts.contract.contract_id);
        assert_eq!(
            bid.bidding_window_closes_at_utc,
            artifacts.contract.bidding_closes_at_utc
        );
    }
}

#[test]
fn runtime_v2_bid_schema_rejects_invalid_bid_fixtures_for_expected_reasons() {
    let artifacts = runtime_v2_bid_schema_contract().expect("bid schema artifacts");

    for case in &artifacts.negative_cases.required_negative_cases {
        let err = case
            .invalid_bid
            .validate_against(&artifacts.contract)
            .expect_err("invalid bid should fail");
        assert!(
            err.to_string().contains(&case.expected_error_fragment),
            "case {} failed with unexpected error {}",
            case.case_id,
            err
        );
    }
}

#[test]
fn runtime_v2_bid_schema_write_to_root_materializes_fixtures() {
    let artifacts = runtime_v2_bid_schema_contract().expect("bid schema artifacts");
    let root = common::unique_temp_path("bid-schema-write");

    artifacts
        .write_to_root(&root)
        .expect("write bid schema artifacts");

    for rel_path in [
        RUNTIME_V2_BID_ALPHA_PATH,
        RUNTIME_V2_BID_BRAVO_PATH,
        RUNTIME_V2_BID_NEGATIVE_CASES_PATH,
    ] {
        let text = std::fs::read_to_string(root.join(rel_path)).expect("artifact text");
        assert!(text.contains("D3"));
        assert!(text.contains("bid"));
        assert!(!text.contains(root.to_string_lossy().as_ref()));
    }

    std::fs::remove_dir_all(root).expect("cleanup bid schema temp root");
}
