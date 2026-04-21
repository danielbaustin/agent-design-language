use super::common::unique_temp_path;
use super::*;
use std::fs;

#[test]
fn runtime_v2_citizen_lifecycle_contract_matches_manifold_refs() {
    let manifold = runtime_v2_manifold_contract().expect("manifold");
    let citizens = RuntimeV2CitizenLifecycleArtifacts::prototype(&manifold).expect("citizens");

    assert_eq!(citizens.active_index.manifold_id, manifold.manifold_id);
    assert_eq!(citizens.pending_index.manifold_id, manifold.manifold_id);
    assert_eq!(
        citizens.active_index.registry_root,
        manifold.citizen_registry_refs.registry_root
    );
    assert_eq!(
        citizens.active_index.index_path,
        manifold.citizen_registry_refs.active_index
    );
    assert_eq!(
        citizens.pending_index.index_path,
        manifold.citizen_registry_refs.pending_index
    );
    assert_eq!(citizens.records.len(), 2);
    assert_eq!(citizens.active_index.citizens.len(), 1);
    assert_eq!(citizens.pending_index.citizens.len(), 1);
    assert!(citizens
        .records
        .iter()
        .any(|record| record.lifecycle_state == "active" && record.can_execute_episodes));
    assert!(citizens
        .records
        .iter()
        .any(|record| record.lifecycle_state == "proposed" && !record.can_execute_episodes));
}

#[test]
fn runtime_v2_citizen_lifecycle_artifacts_match_golden_fixtures() {
    let citizens = runtime_v2_citizen_lifecycle_contract().expect("citizens");
    let alpha = citizens
        .records
        .iter()
        .find(|record| record.citizen_id == "proto-citizen-alpha")
        .expect("alpha");
    let beta = citizens
        .records
        .iter()
        .find(|record| record.citizen_id == "proto-citizen-beta")
        .expect("beta");
    let alpha_json = String::from_utf8(
        RuntimeV2CitizenLifecycleArtifacts::record_pretty_json_bytes(alpha).expect("alpha json"),
    )
    .expect("utf8 alpha");
    let beta_json = String::from_utf8(
        RuntimeV2CitizenLifecycleArtifacts::record_pretty_json_bytes(beta).expect("beta json"),
    )
    .expect("utf8 beta");
    let active_index_json = String::from_utf8(
        RuntimeV2CitizenLifecycleArtifacts::index_pretty_json_bytes(&citizens.active_index)
            .expect("active index json"),
    )
    .expect("utf8 active index");
    let pending_index_json = String::from_utf8(
        RuntimeV2CitizenLifecycleArtifacts::index_pretty_json_bytes(&citizens.pending_index)
            .expect("pending index json"),
    )
    .expect("utf8 pending index");

    assert_eq!(
        alpha_json,
        include_str!("../../../tests/fixtures/runtime_v2/citizens/proto-citizen-alpha.json")
            .trim_end()
    );
    assert_eq!(
        beta_json,
        include_str!("../../../tests/fixtures/runtime_v2/citizens/proto-citizen-beta.json")
            .trim_end()
    );
    assert_eq!(
        active_index_json,
        include_str!("../../../tests/fixtures/runtime_v2/citizens/active_index.json").trim_end()
    );
    assert_eq!(
        pending_index_json,
        include_str!("../../../tests/fixtures/runtime_v2/citizens/pending_index.json").trim_end()
    );
}

#[test]
fn runtime_v2_citizen_lifecycle_writes_artifacts_without_path_leakage() {
    let temp_root = unique_temp_path("citizens");
    let citizens = runtime_v2_citizen_lifecycle_contract().expect("citizens");

    citizens
        .write_to_root(&temp_root)
        .expect("write citizen artifacts");

    let alpha_path = temp_root.join(&citizens.records[0].record_path);
    let beta_path = temp_root.join(&citizens.records[1].record_path);
    let active_index_path = temp_root.join(&citizens.active_index.index_path);
    let pending_index_path = temp_root.join(&citizens.pending_index.index_path);
    assert!(alpha_path.is_file());
    assert!(beta_path.is_file());
    assert!(active_index_path.is_file());
    assert!(pending_index_path.is_file());

    let alpha = fs::read_to_string(alpha_path).expect("alpha text");
    let index = fs::read_to_string(active_index_path).expect("index text");
    let temp_root_text = temp_root.to_string_lossy();
    assert!(!alpha.contains(temp_root_text.as_ref()));
    assert!(!index.contains(temp_root_text.as_ref()));
    assert!(index.contains("\"index_kind\": \"active\""));
    assert!(index.contains("\"citizens\""));

    fs::remove_dir_all(temp_root).ok();
}

#[test]
fn runtime_v2_citizen_lifecycle_validation_rejects_unsafe_or_ambiguous_state() {
    let mut citizens = runtime_v2_citizen_lifecycle_contract().expect("citizens");
    citizens.records[1].citizen_id = "proto-citizen-alpha".to_string();
    assert!(citizens
        .validate()
        .expect_err("duplicate citizen should fail")
        .to_string()
        .contains("duplicate citizen"));

    let mut citizens = runtime_v2_citizen_lifecycle_contract().expect("citizens");
    citizens.records[1].lifecycle_state = "paused".to_string();
    citizens.records[1].can_execute_episodes = true;
    assert!(citizens
        .validate()
        .expect_err("inactive executor should fail")
        .to_string()
        .contains("true only for active citizens"));

    let mut citizens = runtime_v2_citizen_lifecycle_contract().expect("citizens");
    citizens.records[1].lifecycle_state = "waking".to_string();
    assert!(citizens
        .validate()
        .expect_err("waking without rehydration proof should fail")
        .to_string()
        .contains("rehydration validation"));

    let mut citizens = runtime_v2_citizen_lifecycle_contract().expect("citizens");
    citizens.records[1].resources_released = true;
    assert!(citizens
        .validate()
        .expect_err("resource release without termination proof should fail")
        .to_string()
        .contains("before termination is recorded"));
}
