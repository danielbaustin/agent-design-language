use std::{
    fs,
    path::PathBuf,
    process::{Command, Output},
};

fn guardian(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_adl-runtime-guardian"))
        .args(args)
        .output()
        .expect("guardian binary should execute")
}

fn complete_args<'a>(kernel: &'a str, continuity_root: &'a str) -> Vec<&'a str> {
    vec![
        "--kernel",
        kernel,
        "--init",
        "runtime.toml",
        "--continuity-root",
        continuity_root,
        "--restart-budget",
        "0",
        "--backoff-base-ms",
        "1",
        "--backoff-cap-ms",
        "1",
        "--shutdown-grace-ms",
        "100",
    ]
}

fn portable_success_child(root: &std::path::Path) -> PathBuf {
    let source = root.join("success_child.rs");
    let executable = root.join(format!("success_child{}", std::env::consts::EXE_SUFFIX));
    fs::write(&source, "fn main() {}\n").unwrap();
    let status = Command::new("rustc")
        .arg(&source)
        .arg("-o")
        .arg(&executable)
        .status()
        .expect("the Rust test toolchain should resolve rustc");
    assert!(status.success(), "portable child compilation failed");
    executable
}

#[test]
fn guardian_cli_reports_successful_portable_child_as_json() {
    let continuity = tempfile::tempdir().unwrap();
    let child = portable_success_child(continuity.path());
    let output = guardian(&complete_args(
        child.to_str().unwrap(),
        continuity.path().to_str().unwrap(),
    ));

    assert!(
        output.status.success(),
        "stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );
    let payload: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(payload["schema"], "adl.runtime_v3.external_guardian.v2");
    assert_eq!(payload["terminal_state"], "exited_successfully");
    assert_eq!(payload["attempts"], 1);
}

#[test]
fn guardian_cli_reports_spawn_failure_without_restart() {
    let continuity = tempfile::tempdir().unwrap();
    let output = guardian(&complete_args(
        "/definitely/missing/adl-runtime-kernel",
        continuity.path().to_str().unwrap(),
    ));

    assert_eq!(output.status.code(), Some(70));
    let payload: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(payload["terminal_state"], "spawn_failed");
    assert_eq!(payload["attempts"], 1);
}

#[test]
fn guardian_cli_rejects_incomplete_unknown_and_invalid_numeric_arguments() {
    for args in [
        vec!["--kernel", "unused"],
        vec!["--unknown", "value"],
        vec![
            "--kernel",
            "unused",
            "--init",
            "runtime.toml",
            "--continuity-root",
            "continuity",
            "--restart-budget",
            "not-a-number",
        ],
    ] {
        let output = guardian(&args);
        assert_eq!(output.status.code(), Some(64));
        assert!(!output.stderr.is_empty());
        assert!(output.stdout.is_empty());
    }
}
