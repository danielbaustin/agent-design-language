use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

mod helpers;
use helpers::unique_test_temp_dir;

fn fixture_path(rel: &str) -> PathBuf {
    // Robust: works regardless of where tests are run from.
    Path::new(env!("CARGO_MANIFEST_DIR")).join(rel)
}

fn write_temp_adl_yaml() -> PathBuf {
    let yaml_path = fixture_path("tests/fixtures/cli_smoke.adl.yaml");
    let yaml = fs::read_to_string(&yaml_path).expect("read cli_smoke.adl.yaml fixture");

    let p = unique_test_temp_dir("cli-smoke").join("cli_smoke.adl.yaml");

    fs::write(&p, yaml).expect("write temp yaml");
    p
}

fn run_swarm(args: &[&str]) -> std::process::Output {
    // This env var is provided by Cargo for integration tests.
    let exe = env!("CARGO_BIN_EXE_swarm");
    Command::new(exe)
        .args(args)
        .output()
        .expect("run swarm binary")
}

#[test]
fn default_behavior_prints_plan() {
    let path = write_temp_adl_yaml();
    let out = run_swarm(&[path.to_str().unwrap()]);
    assert!(
        out.status.success(),
        "expected success, stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(
        !out.stdout.is_empty(),
        "expected some stdout, stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
}

#[test]
fn print_plan_flag_works() {
    let path = write_temp_adl_yaml();
    let out = run_swarm(&[path.to_str().unwrap(), "--print-plan"]);
    assert!(
        out.status.success(),
        "expected success, stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(!out.stdout.is_empty(), "expected stdout");
}

#[test]
fn print_prompts_flag_works() {
    let path = write_temp_adl_yaml();
    let out = run_swarm(&[path.to_str().unwrap(), "--print-prompts"]);
    assert!(
        out.status.success(),
        "expected success, stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(!out.stdout.is_empty(), "expected stdout");
}

#[test]
fn trace_flag_works() {
    let path = write_temp_adl_yaml();
    let out = run_swarm(&[path.to_str().unwrap(), "--trace"]);
    assert!(
        out.status.success(),
        "expected success, stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(!out.stdout.is_empty(), "expected stdout");
}

#[test]
fn print_plan_preserves_explicit_step_ids_v0_2() {
    let path = fixture_path("examples/v0-2-multi-step-basic.adl.yaml");
    let out = run_swarm(&[path.to_str().unwrap(), "--print-plan"]);
    assert!(
        out.status.success(),
        "expected success, stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );

    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("step-1") && stdout.contains("step-2"),
        "expected explicit step ids in plan output, stdout:\n{}",
        stdout
    );
}

#[test]
fn print_plan_v0_3_concurrency_fixture_works() {
    let path = fixture_path("examples/v0-3-concurrency-fork-join.adl.yaml");
    let out = run_swarm(&[path.to_str().unwrap(), "--print-plan"]);
    assert!(
        out.status.success(),
        "expected success, stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );

    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("fork.plan")
            && stdout.contains("fork.branch.alpha")
            && stdout.contains("fork.join"),
        "expected v0.3 fork/join steps in plan output, stdout:\n{}",
        stdout
    );
}

#[test]
fn print_plan_v0_3_remote_provider_demo_works() {
    let path = fixture_path("examples/v0-3-remote-http-provider.adl.yaml");
    let out = run_swarm(&[path.to_str().unwrap(), "--print-plan"]);
    assert!(
        out.status.success(),
        "expected success, stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );

    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("remote_summary") && stdout.contains("remote_http"),
        "expected remote demo step/provider in plan output, stdout:\n{}",
        stdout
    );
}

#[test]
fn print_plan_v0_5_primitives_fixture_works() {
    let path = fixture_path("examples/v0-5-primitives-minimal.adl.yaml");
    let out = run_swarm(&[path.to_str().unwrap(), "--print-plan"]);
    assert!(
        out.status.success(),
        "expected success, stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );

    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("summarize.topic")
            && stdout.contains("planner")
            && stdout.contains("local_ollama"),
        "expected v0.5 primitive refs in plan output, stdout:\n{}",
        stdout
    );
}

#[test]
fn print_plan_v0_5_include_fixture_works() {
    let path = fixture_path("examples/v0-5-composition-include-root.adl.yaml");
    let out = run_swarm(&[path.to_str().unwrap(), "--print-plan"]);
    assert!(
        out.status.success(),
        "expected include root to load/resolve, stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );

    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("wf_fragment") && stdout.contains("fragment.step"),
        "expected included workflow and step in plan output, stdout:\n{}",
        stdout
    );
}

#[test]
fn print_plan_v0_5_unicode_include_fixture_works() {
    let path = fixture_path("tests/fixtures/ユニコード/include-root.adl.yaml");
    let out = run_swarm(&[path.to_str().unwrap(), "--print-plan"]);
    assert!(
        out.status.success(),
        "expected unicode include fixture to load/resolve, stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );

    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("wf_unicode_fragment") && stdout.contains("unicode.fragment.step"),
        "expected unicode include workflow and step in plan output, stdout:\n{}",
        stdout
    );
}

#[test]
fn print_plan_v0_5_pattern_fixture_is_deterministic() {
    let path = fixture_path("examples/v0-5-pattern-fork-join.adl.yaml");
    let out1 = run_swarm(&[path.to_str().unwrap(), "--print-plan"]);
    let out2 = run_swarm(&[path.to_str().unwrap(), "--print-plan"]);
    assert!(
        out1.status.success() && out2.status.success(),
        "expected success, stderr1:\n{}\nstderr2:\n{}",
        String::from_utf8_lossy(&out1.stderr),
        String::from_utf8_lossy(&out2.stderr)
    );

    assert_eq!(
        out1.stdout, out2.stdout,
        "expected deterministic print-plan output across repeated runs"
    );

    let stdout = String::from_utf8_lossy(&out1.stdout);
    assert!(
        stdout.contains("p::p_fork::left::L1")
            && stdout.contains("p::p_fork::right::R1")
            && stdout.contains("p::p_fork::J"),
        "expected canonical pattern IDs in plan output, stdout:\n{}",
        stdout
    );
}

#[test]
fn unknown_arg_exits_with_code_2_and_prints_usage() {
    let path = write_temp_adl_yaml();
    let out = run_swarm(&[path.to_str().unwrap(), "--nope"]);
    assert_eq!(
        out.status.code(),
        Some(2),
        "expected exit 2, got {:?}\nstderr:\n{}",
        out.status.code(),
        String::from_utf8_lossy(&out.stderr)
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("Unknown arg"), "stderr:\n{stderr}");
    assert!(stderr.contains("Run 'swarm --help'"), "stderr:\n{stderr}");
    assert!(stderr.contains("Usage:"), "stderr:\n{stderr}");
}

#[test]
fn help_prints_examples() {
    let out = run_swarm(&["--help"]);
    assert!(
        out.status.success(),
        "expected success, stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("Examples:"), "stdout:\n{stdout}");
    assert!(
        stdout.contains("examples/v0-3-concurrency-fork-join.adl.yaml")
            && stdout.contains("examples/adl-0.1.yaml")
            && stdout.contains("legacy regression example"),
        "stdout:\n{stdout}"
    );
}

#[test]
fn missing_path_is_an_error() {
    let out = run_swarm(&[]);
    assert!(
        !out.status.success(),
        "expected failure but succeeded; stdout:\n{}",
        String::from_utf8_lossy(&out.stdout)
    );
    // Don't overfit the exact anyhow formatting; just check it's not empty.
    assert!(
        !out.stderr.is_empty(),
        "expected stderr to mention missing args"
    );
}
