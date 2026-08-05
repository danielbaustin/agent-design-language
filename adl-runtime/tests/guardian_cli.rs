use std::{
    fs,
    path::PathBuf,
    process::{Command, Output},
};

fn guardian(args: &[String]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_adl-runtime-guardian"))
        .args(args)
        .output()
        .expect("guardian binary should execute")
}

fn complete_args(kernel: &str, root: &std::path::Path) -> Vec<String> {
    let init = root.join("runtime-init.toml");
    fs::write(
        &init,
        format!(
            r#"
[binaries]
kernel_path = "{}"

[shutdown]
checkpoint_deadline_millis = 100
kernel_grace_millis = 100
api_drain_millis = 100
guardian_margin_millis = 100

[guardian]
restart_budget = 0
backoff_base_millis = 1
backoff_cap_millis = 1
healthy_window_millis = 100
lease_auth_timeout_millis = 100
lease_auth_attempts = 1
capture_max_bytes = 65536
capture_drain_grace_millis = 100
configuration_exit_codes = [64]
"#,
            kernel.replace('\\', "\\\\").replace('"', "\\\"")
        ),
    )
    .unwrap();
    vec!["--init".to_owned(), init.to_string_lossy().into_owned()]
}

fn test_root(name: &str) -> tempfile::TempDir {
    let parent = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join(".csdlc")
        .join("evidence")
        .join("5344")
        .join("work")
        .join("guardian-cli-tests");
    fs::create_dir_all(&parent).unwrap();
    tempfile::Builder::new()
        .prefix(name)
        .tempdir_in(parent)
        .unwrap()
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
    let continuity = test_root("success");
    let child = portable_success_child(continuity.path());
    let output = guardian(&complete_args(child.to_str().unwrap(), continuity.path()));

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
fn guardian_cli_rejects_missing_kernel_before_launch() {
    let continuity = test_root("spawn-failure");
    let output = guardian(&complete_args(
        "/definitely/missing/adl-runtime-kernel",
        continuity.path(),
    ));

    assert_eq!(output.status.code(), Some(64));
    assert!(String::from_utf8_lossy(&output.stderr)
        .contains("binaries.kernel_path must be an absolute existing file"));
    assert!(output.stdout.is_empty());
}

#[test]
fn guardian_cli_rejects_incomplete_unknown_and_invalid_numeric_arguments() {
    for args in [
        Vec::new(),
        vec!["--unknown".to_owned(), "value".to_owned()],
        vec!["--init".to_owned(), "relative.toml".to_owned()],
    ] {
        let output = guardian(&args);
        assert_eq!(output.status.code(), Some(64));
        assert!(!output.stderr.is_empty());
        assert!(output.stdout.is_empty());
    }
}
