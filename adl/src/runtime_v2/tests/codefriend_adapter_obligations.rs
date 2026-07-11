use super::*;

#[test]
fn runtime_v2_codefriend_adapter_obligations_preserves_pre_v092_boundary() {
    let packet = runtime_v2_codefriend_adapter_obligations_contract()
        .expect("CodeFriend/adapter obligations packet");

    assert_eq!(
        packet.schema_version,
        RUNTIME_V2_CODEFRIEND_ADAPTER_OBLIGATIONS_SCHEMA
    );
    assert_eq!(packet.milestone, "v0.91.7");
    assert_eq!(packet.wp, "WP-13");
    assert_eq!(packet.pre_v092_posture, "proof_planning_boundary_for_v0_92");
    assert_eq!(packet.smallest_codefriend_v1_proof.len(), 4);
    assert_eq!(packet.adapter_v2_dependencies.len(), 4);
    assert!(packet
        .adapter_v2_dependencies
        .iter()
        .all(|dependency| { dependency.owner_milestone == "v0.95" && !dependency.blocks_v092 }));
}

#[test]
fn runtime_v2_codefriend_adapter_obligations_names_smallest_proof() {
    let packet = runtime_v2_codefriend_adapter_obligations_contract()
        .expect("CodeFriend/adapter obligations packet");

    for required in [
        "repo-review-packet",
        "specialist-review-lanes",
        "redaction-publication-gate",
        "human-readable-report",
    ] {
        assert!(
            packet
                .smallest_codefriend_v1_proof
                .iter()
                .any(|surface| surface.surface_id == required),
            "missing proof surface {required}"
        );
    }
}

#[test]
fn runtime_v2_codefriend_adapter_obligations_rejects_product_overclaim() {
    let mut packet = runtime_v2_codefriend_adapter_obligations_contract()
        .expect("CodeFriend/adapter obligations packet");
    packet.claim_boundary = "CodeFriend v1 is product ready.".to_string();

    assert!(packet
        .validate()
        .expect_err("product overclaim should fail")
        .to_string()
        .contains("handoff-only posture"));
}

#[test]
fn runtime_v2_codefriend_adapter_obligations_rejects_adapter_v2_as_v092_blocker() {
    let mut packet = runtime_v2_codefriend_adapter_obligations_contract()
        .expect("CodeFriend/adapter obligations packet");
    packet.adapter_v2_dependencies[0].blocks_v092 = true;

    assert!(packet
        .validate()
        .expect_err("v0.92 blocker drift should fail")
        .to_string()
        .contains("must not block v0.92"));
}

#[test]
fn runtime_v2_codefriend_adapter_obligations_rejects_missing_proof_surface() {
    let mut packet = runtime_v2_codefriend_adapter_obligations_contract()
        .expect("CodeFriend/adapter obligations packet");
    packet.smallest_codefriend_v1_proof.pop();

    assert!(packet
        .validate()
        .expect_err("missing proof surface should fail")
        .to_string()
        .contains("exact four proof surfaces"));
}

#[test]
fn runtime_v2_codefriend_adapter_obligations_rejects_gate_drift() {
    let mut packet = runtime_v2_codefriend_adapter_obligations_contract()
        .expect("CodeFriend/adapter obligations packet");
    packet
        .required_promotion_gates
        .retain(|gate| gate != "redaction_publication_review");

    assert!(packet
        .validate()
        .expect_err("gate drift should fail")
        .to_string()
        .contains("full gate set"));
}

#[test]
fn runtime_v2_codefriend_adapter_obligations_rejects_duplicate_non_claims() {
    let mut packet = runtime_v2_codefriend_adapter_obligations_contract()
        .expect("CodeFriend/adapter obligations packet");
    packet.non_claims.push("adapter_v2_implemented".to_string());

    assert!(packet
        .validate()
        .expect_err("duplicate non-claim should fail")
        .to_string()
        .contains("duplicate entries"));
}

#[test]
fn runtime_v2_codefriend_adapter_obligations_json_is_stable_and_path_safe() {
    let packet = runtime_v2_codefriend_adapter_obligations_contract()
        .expect("CodeFriend/adapter obligations packet");
    let json =
        String::from_utf8(packet.pretty_json_bytes().expect("stable json")).expect("utf8 json");

    assert!(json.contains("\"artifact_path\": \"docs/milestones/v0.91.7/review/codefriend_adapter_obligations_4756/boundary_packet.json\""));
    assert!(json.contains("\"pre_v092_posture\": \"proof_planning_boundary_for_v0_92\""));
    assert!(json.contains("v0.95 CodeFriend external-repo proof packaging"));
    assert!(!json.contains("/Users/"));
}

#[test]
fn runtime_v2_codefriend_adapter_obligations_proof_route_paths_exist() {
    let repo_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("repo root");

    for proof_path in [
        "docs/planning/codefriend/CODEFRIEND_V1_BUILD_PLAN.md",
        "docs/milestones/v0.91.7/review/wp13_codefriend_adapter_obligations_4756.md",
        "docs/milestones/v0.91.2/features/CODEFRIEND_PRODUCTIZATION.md",
        "docs/adr/0025-codefriend-review-packet-product-boundary.md",
        "adl/src/runtime_v2/codefriend_adapter_obligations.rs",
        RUNTIME_V2_CODEFRIEND_ADAPTER_OBLIGATIONS_PATH,
    ] {
        assert!(
            repo_root.join(proof_path).exists(),
            "expected CodeFriend proof-route path to exist: {proof_path}"
        );
    }
}

#[test]
fn runtime_v2_codefriend_adapter_obligations_retained_json_matches_contract() {
    let packet = runtime_v2_codefriend_adapter_obligations_contract()
        .expect("CodeFriend/adapter obligations packet");
    let repo_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("repo root");
    let retained_json =
        std::fs::read_to_string(repo_root.join(RUNTIME_V2_CODEFRIEND_ADAPTER_OBLIGATIONS_PATH))
            .expect("read retained CodeFriend/adapter obligations JSON");
    let retained: serde_json::Value =
        serde_json::from_str(&retained_json).expect("parse retained JSON");
    let canonical: serde_json::Value =
        serde_json::from_slice(&packet.pretty_json_bytes().expect("canonical JSON"))
            .expect("parse canonical JSON");

    assert_eq!(retained, canonical);
}
