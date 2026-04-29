use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn unique_temp_dir(prefix: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be valid")
        .as_nanos();
    std::env::temp_dir().join(format!("{prefix}-{nanos}"))
}

#[test]
fn demo_v0905_model_proposal_benchmark_runs_with_explicit_output_path() {
    let exe = env!("CARGO_BIN_EXE_demo_v0905_model_proposal_benchmark");
    let temp_dir = unique_temp_dir("model-proposal-benchmark-demo-explicit");
    let report_path = temp_dir.join("report.json");

    let output = Command::new(exe)
        .arg(&report_path)
        .output()
        .expect("run benchmark demo");

    assert!(
        output.status.success(),
        "expected success, stderr:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        String::from_utf8_lossy(&output.stdout).trim(),
        report_path.display().to_string()
    );
    let report_body = fs::read_to_string(&report_path).expect("read generated report");
    assert!(report_body.contains("model_proposal_benchmark.v1"));
    fs::remove_dir_all(&temp_dir).expect("remove temp dir");
}

#[test]
fn demo_v0905_model_proposal_benchmark_defaults_to_tracked_relative_output_path() {
    let exe = env!("CARGO_BIN_EXE_demo_v0905_model_proposal_benchmark");
    let temp_dir = unique_temp_dir("model-proposal-benchmark-demo-default");
    fs::create_dir_all(&temp_dir).expect("create temp dir");

    let output = Command::new(exe)
        .current_dir(&temp_dir)
        .output()
        .expect("run benchmark demo");

    assert!(
        output.status.success(),
        "expected success, stderr:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        String::from_utf8_lossy(&output.stdout).trim(),
        adl::model_proposal_benchmark::MODEL_PROPOSAL_BENCHMARK_REPORT_ARTIFACT_PATH
    );

    let report_path =
        temp_dir.join(adl::model_proposal_benchmark::MODEL_PROPOSAL_BENCHMARK_REPORT_ARTIFACT_PATH);
    let report_body = fs::read_to_string(&report_path).expect("read generated report");
    assert!(report_body.contains("model_proposal_benchmark.v1"));
    fs::remove_dir_all(&temp_dir).expect("remove temp dir");
}
