use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

mod helpers;
use helpers::unique_test_temp_dir;

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
#[path = "cli_smoke/process_status.rs"]
mod process_status;

fn fixture_path(relative: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(relative)
}

fn repo_root() -> PathBuf {
    fixture_path("..")
}

fn write_temp_adl_yaml() -> PathBuf {
    let source = fixture_path("tests/fixtures/cli_smoke.adl.yaml");
    let destination = unique_test_temp_dir("cli-smoke").join("cli_smoke.adl.yaml");
    fs::copy(source, &destination).expect("copy cli smoke fixture");
    destination
}

fn runtime_test_command(executable: PathBuf) -> Command {
    let mut command = Command::new(executable);
    command
        .env("ADL_CSM_DISK_FLOOR_BYTES", "0")
        .env("ADL_CSM_TEST_AVAILABLE_BYTES", "1073741824");
    command
}

fn resolve_adl_exe() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_adl"))
}

fn resolve_csm_exe() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_csm"))
}

fn resolve_csmctl_exe() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_csmctl"))
}

fn run_adl(args: &[&str]) -> Output {
    run_adl_with_env(args, &[])
}

fn run_adl_with_env(args: &[&str], envs: &[(&str, &str)]) -> Output {
    let mut command = runtime_test_command(resolve_adl_exe());
    command.args(args);
    for (key, value) in envs {
        command.env(key, value);
    }
    command.output().expect("run adl binary")
}

fn run_csm(args: &[&str]) -> Output {
    run_csm_with_env(args, &[])
}

fn run_csm_with_env(args: &[&str], envs: &[(&str, &str)]) -> Output {
    let mut command = runtime_test_command(resolve_csm_exe());
    command.args(args);
    for (key, value) in envs {
        command.env(key, value);
    }
    command.output().expect("run csm binary")
}

fn run_csm_with_env_without_aws_credentials(args: &[&str], envs: &[(&str, &str)]) -> Output {
    run_csm_with_env(args, envs)
}

fn run_csmctl(args: &[&str]) -> Output {
    runtime_test_command(resolve_csmctl_exe())
        .args(args)
        .output()
        .expect("run csmctl binary")
}

fn assert_failure_contains(output: &Output, expected: &str) {
    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        !output.status.success(),
        "unexpected success: stdout={stdout} stderr={stderr}"
    );
    assert!(
        stderr.contains(expected) || stdout.contains(expected),
        "expected {expected:?} in stdout={stdout} stderr={stderr}"
    );
}
