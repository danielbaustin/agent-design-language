use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

mod helpers;
use helpers::unique_test_temp_dir;

fn fixture_path(rel: &str) -> PathBuf {
    // Robust: works regardless of where tests are run from.
    Path::new(env!("CARGO_MANIFEST_DIR")).join(rel)
}

fn repo_root() -> PathBuf {
    fixture_path("..")
}

fn write_temp_adl_yaml() -> PathBuf {
    let yaml_path = fixture_path("tests/fixtures/cli_smoke.adl.yaml");
    let yaml = fs::read_to_string(&yaml_path).expect("read cli_smoke.adl.yaml fixture");

    let p = unique_test_temp_dir("cli-smoke").join("cli_smoke.adl.yaml");

    fs::write(&p, yaml).expect("write temp yaml");
    p
}

fn run_adl(args: &[&str]) -> std::process::Output {
    Command::new(resolve_adl_exe())
        .args(args)
        .output()
        .expect("run adl binary")
}

fn run_adl_csdlc(args: &[&str]) -> std::process::Output {
    Command::new(resolve_adl_csdlc_exe())
        .args(args)
        .output()
        .expect("run adl-csdlc binary")
}

fn run_adl_runtime(args: &[&str]) -> std::process::Output {
    Command::new(resolve_adl_runtime_exe())
        .args(args)
        .output()
        .expect("run adl-runtime binary")
}

fn run_adl_runtime_with_env(args: &[&str], envs: &[(&str, &str)]) -> std::process::Output {
    let mut cmd = Command::new(resolve_adl_runtime_exe());
    cmd.args(args);
    for (k, v) in envs {
        cmd.env(k, v);
    }
    cmd.output().expect("run adl-runtime binary")
}

fn run_adl_with_env(args: &[&str], envs: &[(&str, &str)]) -> std::process::Output {
    let mut cmd = Command::new(resolve_adl_exe());
    cmd.args(args);
    for (k, v) in envs {
        cmd.env(k, v);
    }
    cmd.output().expect("run adl binary")
}

fn resolve_adl_exe() -> PathBuf {
    let raw = std::env::var("CARGO_BIN_EXE_adl")
        .unwrap_or_else(|_| env!("CARGO_BIN_EXE_adl").to_string());
    let path = PathBuf::from(raw);
    if path.is_absolute() {
        path
    } else {
        Path::new(env!("CARGO_MANIFEST_DIR")).join(path)
    }
}

fn resolve_adl_csdlc_exe() -> PathBuf {
    let raw = std::env::var("CARGO_BIN_EXE_adl-csdlc")
        .unwrap_or_else(|_| env!("CARGO_BIN_EXE_adl-csdlc").to_string());
    let path = PathBuf::from(raw);
    if path.is_absolute() {
        path
    } else {
        Path::new(env!("CARGO_MANIFEST_DIR")).join(path)
    }
}

fn resolve_adl_runtime_exe() -> PathBuf {
    let raw = std::env::var("CARGO_BIN_EXE_adl-runtime")
        .unwrap_or_else(|_| env!("CARGO_BIN_EXE_adl-runtime").to_string());
    let path = PathBuf::from(raw);
    if path.is_absolute() {
        path
    } else {
        Path::new(env!("CARGO_MANIFEST_DIR")).join(path)
    }
}

fn assert_failure_contains(out: &std::process::Output, needle: &str) {
    assert!(
        !out.status.success(),
        "expected failure, stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains(needle), "stderr:\n{stderr}");
}

#[test]
fn adl_csdlc_cli_binary_help_and_version_smoke() {
    let help = run_adl_csdlc(&["--help"]);
    assert!(
        help.status.success(),
        "stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&help.stdout),
        String::from_utf8_lossy(&help.stderr)
    );
    let help_stdout = String::from_utf8_lossy(&help.stdout);
    assert!(help_stdout.contains("adl-csdlc - ADL C-SDLC compatibility binary"));
    assert!(help_stdout.contains("adl-csdlc issue run <issue>"));

    let version = run_adl_csdlc(&["--version"]);
    assert!(
        version.status.success(),
        "stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&version.stdout),
        String::from_utf8_lossy(&version.stderr)
    );
    assert_eq!(
        String::from_utf8_lossy(&version.stdout).trim(),
        env!("CARGO_PKG_VERSION")
    );
}

#[test]
fn adl_runtime_cli_binary_help_and_version_smoke() {
    let help = run_adl_runtime(&["--help"]);
    assert!(
        help.status.success(),
        "stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&help.stdout),
        String::from_utf8_lossy(&help.stderr)
    );
    let help_stdout = String::from_utf8_lossy(&help.stdout);
    assert!(help_stdout.contains("adl-runtime - ADL runtime compatibility binary"));
    assert!(help_stdout.contains("adl-runtime run <adl.yaml>"));
    assert!(help_stdout.contains("adl-runtime resume <run_id>"));

    let run_help = run_adl_runtime(&["run", "--help"]);
    assert!(
        run_help.status.success(),
        "stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&run_help.stdout),
        String::from_utf8_lossy(&run_help.stderr)
    );
    assert!(String::from_utf8_lossy(&run_help.stdout).contains("adl-runtime run <adl.yaml>"));

    let version = run_adl_runtime(&["--version"]);
    assert!(
        version.status.success(),
        "stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&version.stdout),
        String::from_utf8_lossy(&version.stderr)
    );
    assert_eq!(
        String::from_utf8_lossy(&version.stdout).trim(),
        env!("CARGO_PKG_VERSION")
    );
}

#[test]
fn adl_runtime_run_matches_adl_yaml_shortcut_for_print_plan() {
    let path = fixture_path("examples/v0-3-concurrency-fork-join.adl.yaml");
    let legacy = run_adl(&[path.to_str().unwrap(), "--print-plan"]);
    let runtime = run_adl_runtime(&["run", path.to_str().unwrap(), "--print-plan"]);

    assert!(
        legacy.status.success() && runtime.status.success(),
        "legacy stderr:\n{}\nruntime stderr:\n{}",
        String::from_utf8_lossy(&legacy.stderr),
        String::from_utf8_lossy(&runtime.stderr)
    );
    assert_eq!(
        legacy.stdout, runtime.stdout,
        "adl-runtime run should preserve legacy YAML shortcut semantics"
    );
}

#[test]
fn adl_runtime_run_executes_fixture_with_mock_provider_and_writes_outputs() {
    let out_dir = unique_test_temp_dir("adl-runtime-run-mock").join("out");
    let runs_root = unique_test_temp_dir("adl-runtime-run-mock-runs");
    let fixture = fixture_path("examples/v0-6-hitl-no-pause.adl.yaml");
    let mock = fixture_path("tools/mock_ollama_v0_4.sh");
    let out = run_adl_runtime_with_env(
        &[
            "run",
            fixture.to_str().unwrap(),
            "--run",
            "--allow-unsigned",
            "--out",
            out_dir.to_str().unwrap(),
        ],
        &[
            ("ADL_OLLAMA_BIN", mock.to_str().unwrap()),
            ("ADL_RUNS_ROOT", runs_root.to_str().unwrap()),
        ],
    );
    assert!(
        out.status.success(),
        "stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(out_dir.join("s1.txt").is_file(), "missing s1.txt");
    assert!(out_dir.join("s2.txt").is_file(), "missing s2.txt");
    assert!(out_dir.join("s3.txt").is_file(), "missing s3.txt");
}

#[test]
fn adl_runtime_run_fails_closed_for_issue_ids() {
    let out = run_adl_runtime(&["run", "3598"]);
    assert_failure_contains(
        &out,
        "C-SDLC issue work belongs to adl/tools/pr.sh run <issue>",
    );

    let hash_out = run_adl_runtime(&["run", "#3598"]);
    assert_failure_contains(
        &hash_out,
        "C-SDLC issue work belongs to adl/tools/pr.sh run <issue>",
    );
}

#[path = "cli_smoke/agent.rs"]
mod agent;
#[path = "cli_smoke/basics.rs"]
mod basics;
#[path = "cli_smoke/exports_and_remote.rs"]
mod exports_and_remote;
#[path = "cli_smoke/godel.rs"]
mod godel;
#[path = "cli_smoke/instrument_and_cli.rs"]
mod instrument_and_cli;
