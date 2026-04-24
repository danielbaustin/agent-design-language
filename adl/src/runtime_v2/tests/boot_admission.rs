#[cfg(feature = "slow-proof-tests")]
use super::common::unique_temp_path;
use super::*;
#[cfg(feature = "slow-proof-tests")]
use std::fs;

#[test]
fn runtime_v2_csm_boot_admission_contract_is_stable() {
    let artifacts = runtime_v2_csm_boot_admission_contract().expect("boot admission artifacts");
    artifacts
        .validate()
        .expect("valid boot admission artifacts");

    assert_eq!(
        artifacts.boot_manifest.schema_version,
        RUNTIME_V2_CSM_BOOT_MANIFEST_SCHEMA
    );
    assert_eq!(artifacts.boot_manifest.demo_id, "D3");
    assert_eq!(artifacts.boot_manifest.manifold_id, "proto-csm-01");
    assert_eq!(
        artifacts.boot_manifest.artifact_path,
        "runtime_v2/csm_run/boot_manifest.json"
    );
    assert_eq!(artifacts.boot_manifest.admitted_citizens.len(), 2);
    assert!(
        artifacts
            .boot_manifest
            .admitted_citizens
            .iter()
            .any(|receipt| receipt.citizen_id == "proto-citizen-alpha"
                && receipt.can_execute_episodes)
    );
    assert!(
        artifacts
            .boot_manifest
            .admitted_citizens
            .iter()
            .any(|receipt| receipt.citizen_id == "proto-citizen-beta"
                && !receipt.can_execute_episodes)
    );
    assert_eq!(artifacts.citizen_roster.entries.len(), 2);
    assert_eq!(artifacts.admission_trace.len(), 4);
    assert!(artifacts
        .boot_manifest
        .claim_boundary
        .contains("not a true Godel-agent birthday"));
}

#[test]
fn runtime_v2_csm_boot_admission_contract_matches_golden_fixtures() {
    let artifacts = runtime_v2_csm_boot_admission_contract().expect("boot admission artifacts");
    let boot_manifest = String::from_utf8(
        artifacts
            .boot_manifest
            .to_pretty_json_bytes()
            .expect("json"),
    )
    .expect("utf8");
    let citizen_roster = String::from_utf8(
        artifacts
            .citizen_roster
            .to_pretty_json_bytes()
            .expect("json"),
    )
    .expect("utf8");
    let admission_trace = String::from_utf8(
        artifacts
            .admission_trace_jsonl_bytes()
            .expect("trace jsonl"),
    )
    .expect("utf8");

    assert_eq!(
        boot_manifest,
        include_str!("../../../tests/fixtures/runtime_v2/csm_run/boot_manifest.json").trim_end()
    );
    assert_eq!(
        citizen_roster,
        include_str!("../../../tests/fixtures/runtime_v2/csm_run/citizen_roster.json").trim_end()
    );
    assert_eq!(
        admission_trace,
        include_str!("../../../tests/fixtures/runtime_v2/csm_run/boot_admission_trace.jsonl")
    );
}

#[cfg(feature = "slow-proof-tests")]
#[test]
fn runtime_v2_csm_boot_admission_writes_without_path_leakage() {
    let temp_root = unique_temp_path("csm-boot-admission");
    let artifacts = runtime_v2_csm_boot_admission_contract().expect("boot admission artifacts");

    artifacts
        .write_to_root(&temp_root)
        .expect("write boot admission artifacts");

    let boot_path = temp_root.join(&artifacts.boot_manifest.artifact_path);
    let roster_path = temp_root.join(&artifacts.citizen_roster.artifact_path);
    let trace_path = temp_root.join(&artifacts.admission_trace_path);
    assert!(boot_path.is_file());
    assert!(roster_path.is_file());
    assert!(trace_path.is_file());
    let boot_text = fs::read_to_string(boot_path).expect("boot text");
    let roster_text = fs::read_to_string(roster_path).expect("roster text");
    let trace_text = fs::read_to_string(trace_path).expect("trace text");
    let temp_root_text = temp_root.to_string_lossy();
    assert!(!boot_text.contains(temp_root_text.as_ref()));
    assert!(!roster_text.contains(temp_root_text.as_ref()));
    assert!(!trace_text.contains(temp_root_text.as_ref()));
    assert!(boot_text.contains("\"schema_version\": \"runtime_v2.csm_boot_manifest.v1\""));
    assert_eq!(trace_text.lines().count(), 4);

    fs::remove_dir_all(temp_root).ok();
}

#[test]
fn runtime_v2_csm_boot_admission_validation_rejects_unsafe_or_ambiguous_state() {
    let mut artifacts = runtime_v2_csm_boot_admission_contract().expect("boot admission artifacts");
    artifacts.boot_manifest.artifact_path =
        "/tmp/runtime_v2/csm_run/boot_manifest.json".to_string();
    assert!(artifacts
        .validate()
        .expect_err("absolute boot path should fail")
        .to_string()
        .contains("repository-relative path"));

    let mut artifacts = runtime_v2_csm_boot_admission_contract().expect("boot admission artifacts");
    artifacts
        .boot_manifest
        .admitted_citizens
        .retain(|receipt| receipt.citizen_id != "proto-citizen-beta");
    assert!(artifacts
        .validate()
        .expect_err("missing beta should fail")
        .to_string()
        .contains("exactly two worker citizens"));

    let mut artifacts = runtime_v2_csm_boot_admission_contract().expect("boot admission artifacts");
    artifacts.admission_trace[1].event_sequence = 9;
    assert!(artifacts
        .validate()
        .expect_err("non-contiguous trace should fail")
        .to_string()
        .contains("contiguous"));

    let mut artifacts = runtime_v2_csm_boot_admission_contract().expect("boot admission artifacts");
    artifacts.boot_manifest.claim_boundary = "first true Godel-agent birthday".to_string();
    assert!(artifacts
        .validate()
        .expect_err("birthday overclaim should fail")
        .to_string()
        .contains("non-claims"));
}
