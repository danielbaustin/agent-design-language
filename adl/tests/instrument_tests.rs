use std::process::Command;

mod helpers;
use helpers::unique_test_temp_dir;

fn run_adl(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_adl"))
        .args(args)
        .output()
        .expect("run adl")
}

#[test]
fn instrument_trace_schema_rejects_extra_args() {
    let out = run_adl(&["instrument", "trace-schema", "extra"]);
    assert_eq!(
        out.status.code(),
        Some(2),
        "stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(String::from_utf8_lossy(&out.stderr)
        .contains("instrument trace-schema accepts no additional arguments"));
}

#[test]
fn instrument_validate_trace_v1_requires_path() {
    let out = run_adl(&["instrument", "validate-trace-v1"]);
    assert_eq!(
        out.status.code(),
        Some(2),
        "stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(String::from_utf8_lossy(&out.stderr)
        .contains("instrument validate-trace-v1 requires <trace-v1.json>"));
}

#[test]
fn instrument_validate_trace_v1_rejects_extra_args() {
    let dir = unique_test_temp_dir("instrument-validate-trace-v1-extra");
    let trace = dir.join("trace-v1.json");
    std::fs::write(
        &trace,
        serde_json::to_string_pretty(&serde_json::json!({
            "schema_version": "trace.v1",
            "events": []
        }))
        .expect("serialize trace fixture"),
    )
    .expect("write trace fixture");

    let out = run_adl(&[
        "instrument",
        "validate-trace-v1",
        trace.to_string_lossy().as_ref(),
        "extra",
    ]);
    assert_eq!(
        out.status.code(),
        Some(2),
        "stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(String::from_utf8_lossy(&out.stderr)
        .contains("instrument validate-trace-v1 accepts exactly <trace-v1.json>"));
}

#[test]
fn instrument_provider_substrate_requires_path() {
    let out = run_adl(&["instrument", "provider-substrate"]);
    assert_eq!(
        out.status.code(),
        Some(2),
        "stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(String::from_utf8_lossy(&out.stderr)
        .contains("instrument provider-substrate requires <adl.yaml>"));
}

#[test]
fn instrument_provider_substrate_rejects_extra_args() {
    let fixture = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("examples/v0-5-pattern-fork-join.adl.yaml");
    let out = run_adl(&[
        "instrument",
        "provider-substrate",
        fixture.to_string_lossy().as_ref(),
        "extra",
    ]);
    assert_eq!(
        out.status.code(),
        Some(2),
        "stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(String::from_utf8_lossy(&out.stderr)
        .contains("instrument provider-substrate accepts exactly one <adl.yaml>"));
}

#[test]
fn instrument_provider_substrate_schema_rejects_extra_args() {
    let out = run_adl(&["instrument", "provider-substrate-schema", "extra"]);
    assert_eq!(
        out.status.code(),
        Some(2),
        "stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(String::from_utf8_lossy(&out.stderr)
        .contains("instrument provider-substrate-schema accepts no extra args"));
}
