use super::common::unique_temp_path;
use super::*;
use std::fs;

#[test]
fn runtime_v2_operator_control_report_records_bounded_controls() {
    let report = runtime_v2_operator_control_report_contract().expect("operator report");

    assert_eq!(
        report.schema_version,
        RUNTIME_V2_OPERATOR_CONTROL_REPORT_SCHEMA
    );
    assert_eq!(report.manifold_id, "proto-csm-01");
    assert_eq!(
        report.control_interface_service_id,
        "operator_control_interface"
    );
    assert_eq!(report.commands.len(), 7);
    assert_eq!(report.commands[0].command, "inspect_manifold");
    assert_eq!(report.commands[2].command, "pause_manifold");
    assert_eq!(report.commands[2].outcome, "deferred");
    assert_eq!(
        report.commands[2].post_state.manifold_lifecycle_state,
        "active"
    );
    assert_eq!(report.commands[3].command, "resume_manifold");
    assert_eq!(report.commands[3].outcome, "refused");
    assert_eq!(
        report.commands[4].post_state.latest_snapshot_id.as_deref(),
        None
    );
    assert_eq!(report.commands[4].outcome, "deferred");
    assert_eq!(report.commands[5].command, "inspect_last_failures");
    assert!(report.commands[5]
        .trace_event_ref
        .contains("violation-0001"));
    assert_eq!(
        report.commands[6].post_state.manifold_lifecycle_state,
        "active"
    );
    assert_eq!(report.commands[6].outcome, "deferred");
    assert_eq!(report.commands[6].post_state.active_citizen_count, 1);
}

#[test]
fn runtime_v2_operator_control_report_matches_golden_fixture() {
    let report = runtime_v2_operator_control_report_contract().expect("operator report");
    let generated = String::from_utf8(report.to_pretty_json_bytes().expect("json")).expect("utf8");

    assert_eq!(
        generated,
        include_str!("../../../tests/fixtures/runtime_v2/operator/control_report.json").trim_end()
    );
}

#[test]
fn runtime_v2_operator_control_report_writes_without_path_leakage() {
    let temp_root = unique_temp_path("operator-controls");
    let report = runtime_v2_operator_control_report_contract().expect("operator report");

    report
        .write_to_root(&temp_root)
        .expect("write operator report");

    let report_path = temp_root.join(&report.artifact_path);
    assert!(report_path.is_file());
    let text = fs::read_to_string(report_path).expect("operator report text");
    assert!(!text.contains(temp_root.to_string_lossy().as_ref()));
    assert!(text.contains("\"schema_version\": \"runtime_v2.operator_control_report.v1\""));
    assert!(text.contains("\"command\": \"pause_manifold\""));
    assert!(text.contains("\"command\": \"terminate_manifold\""));

    fs::remove_dir_all(temp_root).ok();
}

#[test]
fn runtime_v2_operator_control_validation_rejects_unsafe_or_ambiguous_state() {
    let mut report = runtime_v2_operator_control_report_contract().expect("operator report");
    report.artifact_path = "/tmp/operator/control_report.json".to_string();
    assert!(report
        .validate()
        .expect_err("absolute path should fail")
        .to_string()
        .contains("repository-relative path"));

    let mut report = runtime_v2_operator_control_report_contract().expect("operator report");
    report.commands[0].command = "inspect_citizens".to_string();
    assert!(report
        .validate()
        .expect_err("command order should fail")
        .to_string()
        .contains("deterministic command order"));

    let mut report = runtime_v2_operator_control_report_contract().expect("operator report");
    report.commands[2].post_state = report.commands[2].pre_state.clone();
    report.commands[2].outcome = "allowed".to_string();
    assert!(report
        .validate()
        .expect_err("mutating command with unchanged state should fail")
        .to_string()
        .contains("mutating control commands must change post_state"));

    let mut report = runtime_v2_operator_control_report_contract().expect("operator report");
    report.commands[6].post_state.manifold_lifecycle_state = "terminated".to_string();
    report.commands[6].post_state.active_citizen_count = 1;
    assert!(report
        .validate()
        .expect_err("terminated state with active citizens should fail")
        .to_string()
        .contains("terminated state must not retain active citizens"));
}
