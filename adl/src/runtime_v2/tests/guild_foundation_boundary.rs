use super::*;

#[test]
fn runtime_v2_guild_foundation_boundary_is_v092_handoff_only() {
    let packet =
        runtime_v2_guild_foundation_boundary_contract().expect("guild foundation boundary packet");

    assert_eq!(
        packet.schema_version,
        RUNTIME_V2_GUILD_FOUNDATION_BOUNDARY_SCHEMA
    );
    assert_eq!(
        packet.activation_posture,
        "foundation_proof_for_v0_92_governance_handoff"
    );
    assert!(packet
        .minimum_foundation_surfaces
        .contains(&"guild_identity_record".to_string()));
    assert!(packet
        .v092_consumption_allowlist
        .contains(&"birthday_governance_context".to_string()));
    assert!(packet
        .validation_commands
        .iter()
        .any(|command| command.contains(RUNTIME_V2_GUILD_FOUNDATION_BOUNDARY_TEST_MARKER)));
}

#[test]
fn runtime_v2_guild_foundation_boundary_preserves_required_non_claims() {
    let packet =
        runtime_v2_guild_foundation_boundary_contract().expect("guild foundation boundary packet");

    for required in [
        "constitutional_citizenship",
        "polis_governance_runtime",
        "delegated_governance_authority",
        "binding_collective_decision_making",
        "public_guild_product_readiness",
        "v0_92_governance_completion",
    ] {
        assert!(
            packet.non_claims.iter().any(|claim| claim == required),
            "missing non-claim {required}"
        );
    }
}

#[test]
fn runtime_v2_guild_foundation_boundary_rejects_foundation_surface_drift() {
    let mut packet =
        runtime_v2_guild_foundation_boundary_contract().expect("guild foundation boundary packet");
    packet
        .minimum_foundation_surfaces
        .retain(|surface| surface != "moderation_escalation_hook");

    assert!(packet
        .validate()
        .expect_err("missing moderation hook should fail")
        .to_string()
        .contains("exact MVP foundation set"));
}

#[test]
fn runtime_v2_guild_foundation_boundary_rejects_duplicate_foundation_surface() {
    let mut packet =
        runtime_v2_guild_foundation_boundary_contract().expect("guild foundation boundary packet");
    packet
        .minimum_foundation_surfaces
        .push("guild_identity_record".to_string());

    assert!(packet
        .validate()
        .expect_err("duplicate foundation surface should fail")
        .to_string()
        .contains("duplicate entry"));
}

#[test]
fn runtime_v2_guild_foundation_boundary_rejects_consumption_scope_expansion() {
    let mut packet =
        runtime_v2_guild_foundation_boundary_contract().expect("guild foundation boundary packet");
    packet
        .v092_consumption_allowlist
        .push("binding_guild_vote_execution".to_string());

    assert!(packet
        .validate()
        .expect_err("expanded allowlist should fail")
        .to_string()
        .contains("handoff-context allowlist"));
}

#[test]
fn runtime_v2_guild_foundation_boundary_rejects_duplicate_consumption_scope() {
    let mut packet =
        runtime_v2_guild_foundation_boundary_contract().expect("guild foundation boundary packet");
    packet
        .v092_consumption_allowlist
        .push("birthday_governance_context".to_string());

    assert!(packet
        .validate()
        .expect_err("duplicate consumption scope should fail")
        .to_string()
        .contains("duplicate entry"));
}

#[test]
fn runtime_v2_guild_foundation_boundary_rejects_governance_handoff_drift() {
    let mut packet =
        runtime_v2_guild_foundation_boundary_contract().expect("guild foundation boundary packet");
    packet
        .governance_handoff
        .iter_mut()
        .find(|surface| surface.surface_id == "constitutional-citizenship")
        .expect("constitutional citizenship surface")
        .target_milestone = "v0.92".to_string();

    assert!(packet
        .validate()
        .expect_err("constitutional citizenship milestone drift should fail")
        .to_string()
        .contains("constitutional_citizenship.target_milestone"));

    let mut packet =
        runtime_v2_guild_foundation_boundary_contract().expect("guild foundation boundary packet");
    packet
        .governance_handoff
        .iter_mut()
        .find(|surface| surface.surface_id == "polis-governance")
        .expect("polis governance surface")
        .v092_consequence = "Governance may launch in birthday scope.".to_string();

    assert!(packet
        .validate()
        .expect_err("polis consequence drift should fail")
        .to_string()
        .contains("polis governance must deny"));
}

#[test]
fn runtime_v2_guild_foundation_boundary_rejects_duplicate_governance_handoff() {
    let mut packet =
        runtime_v2_guild_foundation_boundary_contract().expect("guild foundation boundary packet");
    let duplicate = packet
        .governance_handoff
        .iter()
        .find(|surface| surface.surface_id == "constitutional-citizenship")
        .expect("constitutional citizenship surface")
        .clone();
    packet.governance_handoff.push(duplicate);

    assert!(packet
        .validate()
        .expect_err("duplicate handoff surface should fail")
        .to_string()
        .contains("duplicate entry"));
}

#[test]
fn runtime_v2_guild_foundation_boundary_rejects_promotion_gate_drift() {
    let mut packet =
        runtime_v2_guild_foundation_boundary_contract().expect("guild foundation boundary packet");
    packet
        .required_promotion_gates
        .retain(|gate| gate != "public_claim_review");

    assert!(packet
        .validate()
        .expect_err("missing public-claim review should fail")
        .to_string()
        .contains("full promotion gate set"));
}

#[test]
fn runtime_v2_guild_foundation_boundary_rejects_duplicate_promotion_gate() {
    let mut packet =
        runtime_v2_guild_foundation_boundary_contract().expect("guild foundation boundary packet");
    packet
        .required_promotion_gates
        .push("operator_approval".to_string());

    assert!(packet
        .validate()
        .expect_err("duplicate promotion gate should fail")
        .to_string()
        .contains("duplicate entry"));
}

#[test]
fn runtime_v2_guild_foundation_boundary_json_is_stable_and_path_safe() {
    let packet =
        runtime_v2_guild_foundation_boundary_contract().expect("guild foundation boundary packet");
    let json =
        String::from_utf8(packet.pretty_json_bytes().expect("stable json")).expect("json is UTF-8");

    assert!(json.contains(
        "\"artifact_path\": \"runtime_v2/guild_foundation_boundary/boundary_packet.json\""
    ));
    assert!(
        json.contains("\"activation_posture\": \"foundation_proof_for_v0_92_governance_handoff\"")
    );
    assert!(json.contains("\"surface_id\": \"constitutional-citizenship\""));
    assert!(!json.contains("/Users/"));
}
