use std::fs;
use std::path::PathBuf;
use std::process::Command;

mod helpers;
use helpers::unique_test_temp_dir;

fn run_swarm(args: &[&str]) -> std::process::Output {
    let exe = env!("CARGO_BIN_EXE_adl");
    Command::new(exe).args(args).output().unwrap()
}

fn run_swarm_with_ci(args: &[&str]) -> std::process::Output {
    let exe = env!("CARGO_BIN_EXE_adl");
    Command::new(exe)
        .env("CI", "1")
        .args(args)
        .output()
        .unwrap()
}

fn tmp_dir(prefix: &str) -> PathBuf {
    unique_test_temp_dir(prefix)
}

#[test]
fn demo_print_plan_works() {
    let out = run_swarm(&["demo", "demo-a-say-mcp", "--print-plan"]);
    assert!(
        out.status.success(),
        "expected success, stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("Demo: demo-a-say-mcp"), "stdout:\n{stdout}");
    assert!(stdout.contains("Steps: 4"), "stdout:\n{stdout}");
}

#[test]
fn demo_run_writes_required_artifacts() {
    let out_root = tmp_dir("demo-run");
    let out = run_swarm(&[
        "demo",
        "demo-a-say-mcp",
        "--run",
        "--trace",
        "--out",
        out_root.to_string_lossy().as_ref(),
    ]);
    assert!(
        out.status.success(),
        "expected success, stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );

    let run_out = out_root.join("demo-a-say-mcp");
    assert!(run_out.join("design.md").is_file());
    assert!(run_out.join("Cargo.toml").is_file());
    assert!(run_out.join("README.md").is_file());
    assert!(run_out.join("src/lib.rs").is_file());
    assert!(run_out.join("src/main.rs").is_file());
    assert!(run_out.join("tests/say_server_tests.rs").is_file());
    assert!(run_out.join("coverage.txt").is_file());
    assert!(run_out.join("index.html").is_file());
    assert!(run_out.join("trace.jsonl").is_file());

    // Ensure README includes canonical run instructions
    let readme = fs::read_to_string(run_out.join("README.md")).unwrap();
    assert!(
        readme.contains("cargo build"),
        "README missing 'cargo build':\n{readme}"
    );
    assert!(
        readme.contains("cargo test"),
        "README missing 'cargo test':\n{readme}"
    );
    assert!(
        readme.contains("cargo run"),
        "README missing 'cargo run':\n{readme}"
    );
    let trace = fs::read_to_string(run_out.join("trace.jsonl")).unwrap();
    assert!(
        trace.contains("TRACE run_id=demo-a-say-mcp"),
        "trace:\n{trace}"
    );
    assert!(trace.contains("RunFinished"), "trace:\n{trace}");
}

#[test]
fn demo_unknown_name_exits_with_code_2() {
    let out = run_swarm(&["demo", "nope", "--run"]);
    assert_eq!(
        out.status.code(),
        Some(2),
        "expected exit 2, got {:?}\nstderr:\n{}",
        out.status.code(),
        String::from_utf8_lossy(&out.stderr)
    );
}

#[test]
fn demo_b_print_plan_is_deterministic() {
    let out = run_swarm(&["demo", "demo-b-one-command", "--print-plan"]);
    assert!(
        out.status.success(),
        "expected success, stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("Demo: demo-b-one-command"),
        "stdout:\n{stdout}"
    );
    assert!(stdout.contains("Steps: 3"), "stdout:\n{stdout}");
    assert!(stdout.contains("0. plan"), "stdout:\n{stdout}");
    assert!(stdout.contains("1. build"), "stdout:\n{stdout}");
    assert!(stdout.contains("2. verify"), "stdout:\n{stdout}");
}

#[test]
fn demo_b_run_is_quiet_and_writes_artifacts() {
    let out_root = tmp_dir("demo-b-run");
    let out = run_swarm_with_ci(&[
        "demo",
        "demo-b-one-command",
        "--run",
        "--trace",
        "--out",
        out_root.to_string_lossy().as_ref(),
    ]);
    assert!(
        out.status.success(),
        "expected success, stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("DEMO OK run_id=demo-b-one-command"),
        "stdout:\n{stdout}"
    );

    let run_out = out_root.join("demo-b-one-command");
    assert!(run_out.join("design.md").is_file());
    assert!(run_out.join("README.md").is_file());
    assert!(run_out.join("coverage.txt").is_file());
    assert!(run_out.join("index.html").is_file());
    assert!(run_out.join("trace.jsonl").is_file());

    let readme = fs::read_to_string(run_out.join("README.md")).unwrap();
    assert!(
        readme.contains("cargo run -- demo demo-b-one-command --run --out <dir>"),
        "README missing run instruction:\n{readme}"
    );

    let trace = fs::read_to_string(run_out.join("trace.jsonl")).unwrap();
    assert!(trace.contains("RunFinished"), "trace:\n{trace}");

    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.trim().is_empty(),
        "expected empty stderr on success, got:\n{stderr}"
    );
}

#[test]
fn demo_c_print_plan_works() {
    let out = run_swarm(&["demo", "demo-c-godel-runtime", "--print-plan"]);
    assert!(
        out.status.success(),
        "expected success, stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("Demo: demo-c-godel-runtime"),
        "stdout:\n{stdout}"
    );
    assert!(stdout.contains("Steps: 3"), "stdout:\n{stdout}");
    assert!(stdout.contains("0. load"), "stdout:\n{stdout}");
    assert!(stdout.contains("1. verify"), "stdout:\n{stdout}");
    assert!(stdout.contains("2. emit"), "stdout:\n{stdout}");
}

#[test]
fn demo_c_run_writes_runtime_surface_artifacts() {
    let out_root = tmp_dir("demo-c-run");
    let out = run_swarm(&[
        "demo",
        "demo-c-godel-runtime",
        "--run",
        "--trace",
        "--out",
        out_root.to_string_lossy().as_ref(),
    ]);
    assert!(
        out.status.success(),
        "expected success, stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );

    let run_out = out_root.join("demo-c-godel-runtime");
    assert!(run_out.join("godel_runtime_surface_status.json").is_file());
    assert!(run_out.join("verification.txt").is_file());
    assert!(run_out.join("README.md").is_file());
    assert!(run_out.join("trace.jsonl").is_file());

    let status = fs::read_to_string(run_out.join("godel_runtime_surface_status.json")).unwrap();
    assert!(
        status.contains("\"status_version\": 1"),
        "status:\n{status}"
    );
    assert!(status.contains("\"failure\""), "status:\n{status}");
    assert!(status.contains("\"record\""), "status:\n{status}");
}

#[test]
fn demo_d_print_plan_works() {
    let out = run_swarm(&["demo", "demo-d-godel-obsmem-loop", "--print-plan"]);
    assert!(
        out.status.success(),
        "expected success, stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("Demo: demo-d-godel-obsmem-loop"),
        "stdout:\n{stdout}"
    );
    assert!(stdout.contains("Steps: 3"), "stdout:\n{stdout}");
}

#[test]
fn demo_d_run_writes_godel_obsmem_artifacts() {
    let out_root = tmp_dir("demo-d-run");
    let out = run_swarm(&[
        "demo",
        "demo-d-godel-obsmem-loop",
        "--run",
        "--trace",
        "--out",
        out_root.to_string_lossy().as_ref(),
    ]);
    assert!(
        out.status.success(),
        "expected success, stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let run_out = out_root.join("demo-d-godel-obsmem-loop");
    assert!(run_out.join("failure_signal.json").is_file());
    assert!(run_out.join("godel_obsmem_demo_summary.json").is_file());
    assert!(run_out
        .join("runs/demo-d-run-001/godel/canonical_evidence_view.v1.json")
        .is_file());
    assert!(run_out
        .join("runs/demo-d-run-001/godel/experiment_record.runtime.v1.json")
        .is_file());
    assert!(run_out
        .join("runs/demo-d-run-001/godel/obsmem_index_entry.runtime.v1.json")
        .is_file());
}

#[test]
fn demo_e_run_writes_card_pipeline_artifacts() {
    let out_root = tmp_dir("demo-e-run");
    let out = run_swarm(&[
        "demo",
        "demo-e-multi-agent-card-pipeline",
        "--run",
        "--trace",
        "--out",
        out_root.to_string_lossy().as_ref(),
    ]);
    assert!(
        out.status.success(),
        "expected success, stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let run_out = out_root.join("demo-e-multi-agent-card-pipeline");
    assert!(run_out.join("pipeline/input_card.md").is_file());
    assert!(run_out.join("pipeline/pipeline_manifest.json").is_file());
}

#[test]
fn demo_f_run_writes_retrieval_artifacts() {
    let out_root = tmp_dir("demo-f-run");
    let out = run_swarm(&[
        "demo",
        "demo-f-obsmem-retrieval",
        "--run",
        "--trace",
        "--out",
        out_root.to_string_lossy().as_ref(),
    ]);
    assert!(
        out.status.success(),
        "expected success, stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let run_out = out_root.join("demo-f-obsmem-retrieval");
    assert!(run_out.join("obsmem_retrieval_result.json").is_file());
    assert!(run_out
        .join("runs/demo-f-run-a/godel/obsmem_index_entry.runtime.v1.json")
        .is_file());
    assert!(run_out
        .join("runs/demo-f-run-b/godel/obsmem_index_entry.runtime.v1.json")
        .is_file());
}
