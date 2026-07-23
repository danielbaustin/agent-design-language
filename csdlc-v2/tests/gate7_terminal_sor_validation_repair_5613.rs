#[test]
fn public_contract_exposes_terminal_sor_validation_repair_request() {
    let bundle = csdlc_v2::public_schema_bundle();
    assert!(bundle
        .get("terminal_sor_validation_repair_request")
        .is_some());
}
