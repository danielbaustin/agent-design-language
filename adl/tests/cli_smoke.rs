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
