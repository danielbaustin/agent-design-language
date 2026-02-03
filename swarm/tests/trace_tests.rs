use std::process::Command;

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

    let mut path = std::env::temp_dir();
    let unique = format!(
        "adl-trace-test-{}-{}.yaml",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0)
    );
    path.push(unique);

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
        stdout.contains("TRACE run_id=") && stdout.contains("workflow_id="),
        "expected TRACE header, stdout was:\n{}",
        stdout
    );
}
