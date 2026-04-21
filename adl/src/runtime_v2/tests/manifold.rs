use super::common::unique_temp_path;
use super::*;
use std::fs;

#[test]
fn runtime_v2_manifold_root_contract_is_stable() {
    let root = runtime_v2_manifold_contract().expect("contract");
    root.validate().expect("valid manifold root");

    assert_eq!(root.schema_version, RUNTIME_V2_MANIFOLD_SCHEMA);
    assert_eq!(root.manifold_id, "proto-csm-01");
    assert_eq!(root.lifecycle_state, "initialized");
    assert_eq!(root.artifact_path, DEFAULT_MANIFOLD_ARTIFACT_PATH);
    assert_eq!(root.clock_anchor.monotonic_tick, 0);
    assert_eq!(root.trace_root.next_event_sequence, 1);
    assert_eq!(root.snapshot_root.latest_snapshot_id, None);
    assert!(root
        .invariant_policy_refs
        .blocking_invariants
        .contains(&"single_active_manifold_instance".to_string()));
    assert!(root
        .review_surface
        .downstream_boundaries
        .iter()
        .any(|boundary| boundary.contains("WP-06")));
}

#[test]
fn runtime_v2_manifold_root_round_trips_without_path_leakage() {
    let temp_root = unique_temp_path("roundtrip");
    let path = temp_root.join(DEFAULT_MANIFOLD_ARTIFACT_PATH);
    let root = runtime_v2_manifold_contract().expect("contract");

    root.write_to_path(&path).expect("write manifest");
    let loaded = RuntimeV2ManifoldRoot::read_from_path(&path).expect("read manifest");
    assert_eq!(loaded, root);

    let text = fs::read_to_string(&path).expect("manifest text");
    assert!(text.contains("\"schema_version\": \"runtime_v2.manifold.v1\""));
    assert!(text.contains("\"artifact_path\": \"runtime_v2/manifold.json\""));
    assert!(!text.contains(temp_root.to_string_lossy().as_ref()));

    fs::remove_dir_all(temp_root).ok();
}

#[test]
fn runtime_v2_manifold_root_matches_golden_manifest_fixture() {
    let root = runtime_v2_manifold_contract().expect("contract");
    let generated = String::from_utf8(root.to_pretty_json_bytes().expect("json")).expect("utf8");
    let expected = include_str!("../../../tests/fixtures/runtime_v2/manifold.json");

    assert_eq!(generated, expected.trim_end());
}

#[test]
fn runtime_v2_manifold_validation_rejects_unsafe_or_ambiguous_roots() {
    let mut root = runtime_v2_manifold_contract().expect("contract");
    root.manifold_id = " ".to_string();
    assert!(root
        .validate()
        .expect_err("empty id should fail")
        .to_string()
        .contains("manifold_id must not be empty"));

    let mut root = runtime_v2_manifold_contract().expect("contract");
    root.artifact_path = "/tmp/runtime_v2/manifold.json".to_string();
    assert!(root
        .validate()
        .expect_err("absolute path should fail")
        .to_string()
        .contains("artifact_path must be a repository-relative path"));

    let mut root = runtime_v2_manifold_contract().expect("contract");
    root.trace_root.next_event_sequence = 0;
    assert!(root
        .validate()
        .expect_err("zero sequence should fail")
        .to_string()
        .contains("trace_root.next_event_sequence must be positive"));
}

#[test]
fn runtime_v2_manifold_root_does_not_claim_later_wp_outputs() {
    let root = runtime_v2_manifold_contract().expect("contract");
    let json = String::from_utf8(root.to_pretty_json_bytes().expect("json")).expect("utf8");

    assert!(json.contains("WP-07 owns provisional citizen record materialization"));
    assert!(json.contains("WP-08 owns snapshot writing, sealing, and rehydration"));
    assert!(json.contains("no true Godel-agent birthday"));
    assert!(!json.contains("citizen_id"));
    assert!(!json.contains("snapshot_hash"));
    assert!(!json.contains("kernel_tick_completed"));
}
