use super::*;

#[test]
fn runtime_v2_economics_civilization_boundary_is_context_only_for_v092() {
    let packet = runtime_v2_economics_civilization_boundary_contract()
        .expect("economics/civilization boundary packet");

    assert_eq!(
        packet.schema_version,
        RUNTIME_V2_ECONOMICS_CIVILIZATION_BOUNDARY_SCHEMA
    );
    assert_eq!(packet.activation_posture, "context_only_for_v0_92");
    assert!(packet.promoted_activation_tests.is_empty());
    assert!(packet
        .allowed_v092_consumption
        .contains(&"scheduler_and_resource_stewardship_context".to_string()));
    assert!(packet
        .validation_commands
        .iter()
        .any(|command| command.contains(RUNTIME_V2_ECONOMICS_CIVILIZATION_BOUNDARY_TEST_MARKER)));
}

#[test]
fn runtime_v2_economics_civilization_boundary_preserves_required_non_claims() {
    let packet = runtime_v2_economics_civilization_boundary_contract()
        .expect("economics/civilization boundary packet");

    for required in [
        "payments_implementation",
        "settlement_implementation",
        "market_mechanism_proof",
        "civilization_runtime",
        "autonomous_economy",
        "runtime_economic_optimizer",
        "v0_92_product_readiness",
    ] {
        assert!(
            packet.non_claims.iter().any(|claim| claim == required),
            "missing non-claim {required}"
        );
    }
}

#[test]
fn runtime_v2_economics_civilization_boundary_rejects_promoted_tests_without_issue() {
    let mut packet = runtime_v2_economics_civilization_boundary_contract()
        .expect("economics/civilization boundary packet");
    packet
        .promoted_activation_tests
        .push("market-simulation-smoke".to_string());

    assert!(packet
        .validate()
        .expect_err("promoted tests should fail without separate approval")
        .to_string()
        .contains("operator-approved issue"));
}

#[test]
fn runtime_v2_economics_civilization_boundary_rejects_activation_claim_drift() {
    let mut packet = runtime_v2_economics_civilization_boundary_contract()
        .expect("economics/civilization boundary packet");
    packet.activation_posture = "activation_required_for_v0_92".to_string();
    assert!(packet
        .validate()
        .expect_err("activation posture drift should fail")
        .to_string()
        .contains("activation_posture"));

    let mut packet = runtime_v2_economics_civilization_boundary_contract()
        .expect("economics/civilization boundary packet");
    packet
        .allowed_v092_consumption
        .push("payment_execution".to_string());
    assert!(packet
        .validate()
        .expect_err("expanded allowlist should fail")
        .to_string()
        .contains("context-only allowlist"));
}

#[test]
fn runtime_v2_economics_civilization_boundary_rejects_postponed_surface_drift() {
    let mut packet = runtime_v2_economics_civilization_boundary_contract()
        .expect("economics/civilization boundary packet");
    packet
        .postponed_surfaces
        .iter_mut()
        .find(|surface| surface.surface_id == "payments-settlement")
        .expect("payments surface")
        .target_milestone = "v0.92".to_string();
    assert!(packet
        .validate()
        .expect_err("payment target milestone drift should fail")
        .to_string()
        .contains("payments_settlement.target_milestone"));

    let mut packet = runtime_v2_economics_civilization_boundary_contract()
        .expect("economics/civilization boundary packet");
    packet
        .postponed_surfaces
        .iter_mut()
        .find(|surface| surface.surface_id == "civilization-economics")
        .expect("civilization surface")
        .v092_consequence = "No immediate concern; revisit later.".to_string();
    assert!(packet
        .validate()
        .expect_err("civilization consequence drift should fail")
        .to_string()
        .contains("civilization economics must deny"));
}

#[test]
fn runtime_v2_economics_civilization_boundary_rejects_promotion_gate_drift() {
    let mut packet = runtime_v2_economics_civilization_boundary_contract()
        .expect("economics/civilization boundary packet");
    packet
        .required_promotion_gates
        .retain(|gate| gate != "security_governance_review");
    assert!(packet
        .validate()
        .expect_err("missing promotion gate should fail")
        .to_string()
        .contains("full promotion gate set"));
}

#[test]
fn runtime_v2_economics_civilization_boundary_rejects_duplicate_policy_rows() {
    let mut packet = runtime_v2_economics_civilization_boundary_contract()
        .expect("economics/civilization boundary packet");
    packet
        .allowed_v092_consumption
        .push(packet.allowed_v092_consumption[0].clone());
    assert!(packet
        .validate()
        .expect_err("duplicate allowed consumption should fail")
        .to_string()
        .contains("allowed_v092_consumption must not contain duplicates"));

    let mut packet = runtime_v2_economics_civilization_boundary_contract()
        .expect("economics/civilization boundary packet");
    packet
        .postponed_surfaces
        .push(packet.postponed_surfaces[0].clone());
    assert!(packet
        .validate()
        .expect_err("duplicate postponed surface should fail")
        .to_string()
        .contains("postponed_surfaces must not contain duplicate surface ids"));

    let mut packet = runtime_v2_economics_civilization_boundary_contract()
        .expect("economics/civilization boundary packet");
    packet
        .required_promotion_gates
        .push(packet.required_promotion_gates[0].clone());
    assert!(packet
        .validate()
        .expect_err("duplicate promotion gate should fail")
        .to_string()
        .contains("required_promotion_gates must not contain duplicates"));
}

#[test]
fn runtime_v2_economics_civilization_boundary_json_is_stable_and_path_safe() {
    let packet = runtime_v2_economics_civilization_boundary_contract()
        .expect("economics/civilization boundary packet");
    let json =
        String::from_utf8(packet.pretty_json_bytes().expect("stable json")).expect("json is UTF-8");

    assert!(json.contains(
        "\"artifact_path\": \"runtime_v2/economics_civilization_boundary/boundary_packet.json\""
    ));
    assert!(json.contains("\"activation_posture\": \"context_only_for_v0_92\""));
    assert!(!json.contains("/Users/"));
}
