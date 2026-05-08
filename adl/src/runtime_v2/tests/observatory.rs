#[cfg(feature = "slow-proof-tests")]
use super::common::unique_temp_path;
use super::*;
#[cfg(feature = "slow-proof-tests")]
use std::fs;

#[test]
fn runtime_v2_csm_observatory_contract_is_stable() {
    let artifacts = runtime_v2_csm_observatory_contract().expect("observatory artifacts");
    artifacts.validate().expect("valid observatory artifacts");

    assert_eq!(
        artifacts.visibility_packet["schema"],
        crate::csm_observatory::VISIBILITY_PACKET_SCHEMA
    );
    assert_eq!(
        artifacts.visibility_packet["packet_id"],
        "runtime-v2-csm-observatory-active-packet-0001"
    );
    assert_eq!(
        artifacts.visibility_packet["active_surface"]["surface_id"],
        "wp04-observatory-active-surface-0001"
    );
    assert_eq!(
        artifacts.visibility_packet["active_surface"]["lifecycle_contract_ref"],
        "runtime_v2/agent_lifecycle/state_contract.json"
    );
    assert_eq!(
        artifacts.visibility_packet["review"]["demo_classification"],
        "fixture_backed"
    );
    assert_eq!(
        artifacts.visibility_packet["kernel"]["pulse"]["completed_through_event_sequence"],
        9
    );
    assert_eq!(
        artifacts.visibility_packet["freedom_gate"]["refuse_count"],
        1
    );
    assert_eq!(
        artifacts.visibility_packet_path,
        "runtime_v2/observatory/visibility_packet.json"
    );
    assert_eq!(
        artifacts.operator_report_path,
        "runtime_v2/observatory/operator_report.md"
    );
    assert!(artifacts
        .operator_report_markdown
        .contains("Counts: allow 1, defer 0, refuse 1."));
    assert!(artifacts
        .operator_report_markdown
        .contains("wp04-observatory-active-surface-0001"));
    assert!(artifacts
        .operator_report_markdown
        .contains("runtime_v2/agent_lifecycle/state_contract.json"));
}

#[cfg(feature = "slow-proof-tests")]
#[test]
fn runtime_v2_csm_observatory_writes_without_path_leakage() {
    let temp_root = unique_temp_path("csm-observatory");
    let artifacts = runtime_v2_csm_observatory_contract().expect("observatory artifacts");

    artifacts
        .write_to_root(&temp_root)
        .expect("write observatory artifacts");

    let packet_path = temp_root.join(&artifacts.visibility_packet_path);
    let report_path = temp_root.join(&artifacts.operator_report_path);
    assert!(packet_path.is_file());
    assert!(report_path.is_file());
    let packet_text = fs::read_to_string(packet_path).expect("packet text");
    let report_text = fs::read_to_string(report_path).expect("report text");
    let temp_root_text = temp_root.to_string_lossy();
    assert!(!packet_text.contains(temp_root_text.as_ref()));
    assert!(!report_text.contains(temp_root_text.as_ref()));
    assert!(packet_text.contains("\"schema\": \"adl.csm_visibility_packet.v1\""));
    assert!(packet_text.contains("runtime_v2/csm_run/wake_continuity_proof.json"));
    assert!(report_text.contains("runtime-v2-csm-observatory-active-packet-0001"));

    fs::remove_dir_all(temp_root).ok();
}

#[test]
fn runtime_v2_csm_observatory_contract_matches_golden_fixtures() {
    let artifacts = runtime_v2_csm_observatory_contract().expect("observatory artifacts");
    let visibility_packet = String::from_utf8(
        artifacts
            .visibility_packet_pretty_json_bytes()
            .expect("packet json"),
    )
    .expect("utf8 packet");

    assert_eq!(
        visibility_packet,
        include_str!("../../../tests/fixtures/runtime_v2/observatory/visibility_packet.json")
            .trim_end()
    );
    assert_eq!(
        artifacts.operator_report_markdown.trim_end(),
        include_str!("../../../tests/fixtures/runtime_v2/observatory/operator_report.md")
            .trim_end()
    );
}

#[test]
fn runtime_v2_csm_observatory_validation_rejects_unsafe_or_ambiguous_state() {
    let mut artifacts = runtime_v2_csm_observatory_contract().expect("observatory artifacts");
    artifacts.visibility_packet_path =
        "/tmp/runtime_v2/observatory/visibility_packet.json".to_string();
    assert!(artifacts
        .validate()
        .expect_err("absolute packet path should fail")
        .to_string()
        .contains("repository-relative path"));

    let mut artifacts = runtime_v2_csm_observatory_contract().expect("observatory artifacts");
    artifacts.visibility_packet["freedom_gate"]["allow_count"] = serde_json::json!(7);
    assert!(artifacts
        .validate()
        .expect_err("count mismatch should fail")
        .to_string()
        .contains("allow_count"));

    let mut artifacts = runtime_v2_csm_observatory_contract().expect("observatory artifacts");
    artifacts.visibility_packet["source"]["source_refs"] =
        serde_json::json!(["runtime_v2/csm_run/run_packet_contract.json"]);
    assert!(artifacts
        .validate()
        .expect_err("missing wake proof source should fail")
        .to_string()
        .contains("source ref"));

    let mut artifacts = runtime_v2_csm_observatory_contract().expect("observatory artifacts");
    artifacts.visibility_packet["active_surface"]["lifecycle_contract_ref"] =
        serde_json::json!("runtime_v2/agent_lifecycle/state_contract.v0.json");
    assert!(artifacts
        .validate()
        .expect_err("lifecycle linkage drift should fail")
        .to_string()
        .contains("WP-03 lifecycle contract"));

    let mut artifacts = runtime_v2_csm_observatory_contract().expect("observatory artifacts");
    artifacts.operator_report_markdown = artifacts.operator_report_markdown.replace(
        "Counts: allow 1, defer 0, refuse 1.",
        "Counts: allow 0, defer 0, refuse 0.",
    );
    assert!(artifacts
        .validate()
        .expect_err("report drift should fail")
        .to_string()
        .contains("operator report"));

    let run_packet = runtime_v2_csm_run_packet_contract().expect("run packet");
    let boot_admission = runtime_v2_csm_boot_admission_contract().expect("boot admission");
    let mut wake_continuity = runtime_v2_csm_wake_continuity_contract().expect("wake continuity");
    let lifecycle = runtime_v2_agent_lifecycle_state_contract().expect("lifecycle");
    wake_continuity.first_run_trace_path =
        "runtime_v2/csm_run/alternate_first_run_trace.jsonl".to_string();
    wake_continuity.wake_continuity_proof.source_trace_ref =
        wake_continuity.first_run_trace_path.clone();
    assert!(RuntimeV2CsmObservatoryArtifacts::from_contracts(
        &run_packet,
        &boot_admission,
        &wake_continuity,
        &lifecycle
    )
    .expect_err("lifecycle linkage drift should fail")
    .to_string()
    .contains("ACTIVE lifecycle linkage"));
}
