use std::process::Command;

mod helpers;
use helpers::unique_test_temp_dir;

fn has_iso_prefix_and_elapsed(line: &str) -> bool {
    // Expected prefix: YYYY-MM-DDTHH:MM:SS.mmmZ (+Nms)
    if line.len() < 30 {
        return false;
    }
    let b = line.as_bytes();
    if !(b[4] == b'-'
        && b[7] == b'-'
        && b[10] == b'T'
        && b[13] == b':'
        && b[16] == b':'
        && b[19] == b'.'
        && b[23] == b'Z'
        && b[24] == b' '
        && b[25] == b'('
        && b[26] == b'+')
    {
        return false;
    }
    line.contains("ms)")
}

#[test]
fn cli_trace_flag_prints_trace_header() {
    // This verifies end-to-end CLI wiring produces trace output.
    // We generate a minimal ADL YAML in a temp file so the test does not depend
    // on any checked-in example files.
    let exe = env!("CARGO_BIN_EXE_swarm");

    // Minimal doc: empty agents/tasks, one empty step.
    // Keep this aligned with the structs in src/adl.rs.
    let yaml = r#"version: "0.1"
providers: {}
tools: {}
agents: {}
tasks: {}
run:
  name: "test-run"
  workflow:
    kind: sequential
    steps:
      - {}
"#;

    let path = unique_test_temp_dir("trace-flag").join("trace.yaml");

    std::fs::write(&path, yaml).expect("failed to write temp adl yaml");

    let out = Command::new(exe)
        .arg(path.to_string_lossy().as_ref())
        .arg("--trace")
        .output()
        .expect("failed to run swarm binary");

    // Best-effort cleanup.
    let _ = std::fs::remove_file(&path);

    assert!(
        out.status.success(),
        "expected success, got {:?}\nstderr:\n{}",
        out.status.code(),
        String::from_utf8_lossy(&out.stderr)
    );

    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("TRACE run_id=")
            && stdout.contains("workflow_id=")
            && stdout.contains("version="),
        "expected TRACE header, stdout was:\n{}",
        stdout
    );
}

#[test]
fn cli_trace_reports_run_failed_on_invalid_yaml() {
    let exe = env!("CARGO_BIN_EXE_swarm");

    let path = unique_test_temp_dir("trace-invalid-yaml").join("invalid.yaml");

    std::fs::write(&path, "version: [").expect("failed to write invalid yaml");

    let out = Command::new(exe)
        .arg(path.to_string_lossy().as_ref())
        .arg("--trace")
        .output()
        .expect("failed to run swarm binary");

    let _ = std::fs::remove_file(&path);

    assert!(
        !out.status.success(),
        "expected failure, got {:?}\nstderr:\n{}",
        out.status.code(),
        String::from_utf8_lossy(&out.stderr)
    );

    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("TRACE run_id=") && stdout.contains("RunFailed"),
        "expected trace with RunFailed, stdout was:\n{}",
        stdout
    );
}

#[test]
fn cli_trace_records_step_failure_on_missing_file() {
    let exe = env!("CARGO_BIN_EXE_swarm");

    let yaml = r#"version: "0.1"
providers: {}
tools: {}
agents: {}
tasks:
  t1:
    prompt:
      user: "Summarize: {{doc}}"
run:
  name: "trace-missing-file"
  workflow:
    kind: sequential
    steps:
      - task: "t1"
        inputs:
          doc: "@file:does-not-exist.txt"
"#;

    let path = unique_test_temp_dir("trace-missing-file").join("missing-file.yaml");

    std::fs::write(&path, yaml).expect("failed to write temp adl yaml");

    let out = Command::new(exe)
        .arg(path.to_string_lossy().as_ref())
        .arg("--run")
        .arg("--trace")
        .output()
        .expect("failed to run swarm binary");

    let _ = std::fs::remove_file(&path);

    assert!(
        !out.status.success(),
        "expected failure, got {:?}\nstderr:\n{}",
        out.status.code(),
        String::from_utf8_lossy(&out.stderr)
    );

    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("StepStarted")
            && stdout.contains("StepFinished")
            && stdout.contains("success=false")
            && stdout.contains("RunFailed"),
        "expected step failure in trace, stdout was:\n{}",
        stdout
    );
}

#[test]
fn cli_trace_v0_2_includes_human_timestamps_and_durations() {
    let exe = env!("CARGO_BIN_EXE_swarm");

    let yaml = r#"version: "0.2"
providers: {}
tools: {}
agents: {}
tasks:
  t1:
    prompt:
      user: "Hello {{name}}"
run:
  name: "trace-v0-2"
  workflow:
    kind: sequential
    steps:
      - id: "s1"
        task: "t1"
        inputs:
          name: "world"
"#;

    let path = unique_test_temp_dir("trace-v0-2").join("trace-v0-2.yaml");

    std::fs::write(&path, yaml).expect("failed to write temp adl yaml");

    let out = Command::new(exe)
        .arg(path.to_string_lossy().as_ref())
        .arg("--trace")
        .output()
        .expect("failed to run swarm binary");

    let _ = std::fs::remove_file(&path);

    assert!(
        out.status.success(),
        "expected success, got {:?}\nstderr:\n{}",
        out.status.code(),
        String::from_utf8_lossy(&out.stderr)
    );

    let stdout = String::from_utf8_lossy(&out.stdout);
    let step_started_line = stdout
        .lines()
        .find(|l| l.contains("StepStarted"))
        .unwrap_or_default();
    let step_finished_line = stdout
        .lines()
        .find(|l| l.contains("StepFinished"))
        .unwrap_or_default();
    assert!(
        stdout.contains("version=0.2") && stdout.contains("RunFinished"),
        "expected v0.2 trace enhancements, stdout was:\n{}",
        stdout
    );
    assert!(
        has_iso_prefix_and_elapsed(step_started_line),
        "expected ISO timestamp + elapsed prefix on StepStarted, got:\n{}",
        step_started_line
    );
    assert!(
        has_iso_prefix_and_elapsed(step_finished_line),
        "expected ISO timestamp + elapsed prefix on StepFinished, got:\n{}",
        step_finished_line
    );
    assert!(
        step_finished_line.contains("duration_ms="),
        "expected duration_ms on StepFinished line, got:\n{}",
        step_finished_line
    );
}

#[test]
fn cli_trace_v0_2_preserves_explicit_step_ids() {
    let exe = env!("CARGO_BIN_EXE_swarm");

    let out = Command::new(exe)
        .arg("examples/v0-2-multi-step-basic.adl.yaml")
        .arg("--trace")
        .output()
        .expect("failed to run swarm binary");

    assert!(
        out.status.success(),
        "expected success, got {:?}\nstderr:\n{}",
        out.status.code(),
        String::from_utf8_lossy(&out.stderr)
    );

    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("step=step-1") && stdout.contains("step=step-2"),
        "expected explicit step ids in trace output, stdout was:\n{}",
        stdout
    );
}
