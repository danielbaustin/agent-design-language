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

fn runtime_test_command(executable: PathBuf) -> Command {
    let mut command = Command::new(executable);
    command
        .env("ADL_CSM_DISK_FLOOR_BYTES", "0")
        .env("ADL_CSM_TEST_AVAILABLE_BYTES", "1073741824");
    command
}

fn run_adl(args: &[&str]) -> std::process::Output {
    runtime_test_command(resolve_adl_exe())
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

fn run_csdlc(args: &[&str]) -> std::process::Output {
    Command::new(resolve_csdlc_exe())
        .args(args)
        .output()
        .expect("run csdlc binary")
}

fn run_adl_runtime(args: &[&str]) -> std::process::Output {
    runtime_test_command(resolve_adl_runtime_exe())
        .args(args)
        .output()
        .expect("run adl-runtime binary")
}

fn run_csm(args: &[&str]) -> std::process::Output {
    runtime_test_command(resolve_csm_exe())
        .args(args)
        .output()
        .expect("run csm binary")
}

fn run_csm_without_aws_credentials(args: &[&str]) -> std::process::Output {
    run_csm_with_env_without_aws_credentials(args, &[])
}

fn run_csm_with_env_without_aws_credentials(
    args: &[&str],
    envs: &[(&str, &str)],
) -> std::process::Output {
    const AWS_CREDENTIAL_ENV: &[&str] = &[
        "AWS_ACCESS_KEY_ID",
        "AWS_SECRET_ACCESS_KEY",
        "AWS_SESSION_TOKEN",
        "AWS_PROFILE",
        "AWS_DEFAULT_PROFILE",
        "AWS_REGION",
        "AWS_DEFAULT_REGION",
        "AWS_CONTAINER_CREDENTIALS_RELATIVE_URI",
        "AWS_CONTAINER_CREDENTIALS_FULL_URI",
        "AWS_WEB_IDENTITY_TOKEN_FILE",
        "AWS_ROLE_ARN",
        "AWS_CONFIG_FILE",
        "AWS_SHARED_CREDENTIALS_FILE",
        "AWS_SDK_LOAD_CONFIG",
        "ADL_AWS_PROFILE",
        "ADL_AWS_REGION",
        "ADL_AWS_SIGNAL_MODE",
        "ADL_AWS_SIGNAL_APPROVED",
        "ADL_AWS_HEARTBEAT_LOG_GROUP",
        "ADL_AWS_HEARTBEAT_LOG_STREAM",
        "ADL_AWS_SNS_TOPIC_ARN",
    ];
    let mut command = runtime_test_command(resolve_csm_exe());
    command.args(args).env("AWS_EC2_METADATA_DISABLED", "true");
    for (key, value) in envs {
        command.env(key, value);
    }
    for name in AWS_CREDENTIAL_ENV {
        command.env_remove(name);
    }
    command
        .output()
        .expect("run csm binary without AWS credentials")
}

fn run_csmctl(args: &[&str]) -> std::process::Output {
    Command::new(resolve_csmctl_exe())
        .args(args)
        .output()
        .expect("run csmctl binary")
}

fn run_adl_review(args: &[&str]) -> std::process::Output {
    Command::new(resolve_adl_review_exe())
        .args(args)
        .output()
        .expect("run adl-review binary")
}

fn run_adl_runtime_with_env(args: &[&str], envs: &[(&str, &str)]) -> std::process::Output {
    let mut cmd = runtime_test_command(resolve_adl_runtime_exe());
    cmd.args(args);
    for (k, v) in envs {
        cmd.env(k, v);
    }
    cmd.output().expect("run adl-runtime binary")
}

fn run_csm_with_env(args: &[&str], envs: &[(&str, &str)]) -> std::process::Output {
    let mut cmd = runtime_test_command(resolve_csm_exe());
    cmd.args(args);
    for (k, v) in envs {
        cmd.env(k, v);
    }
    cmd.output().expect("run csm binary")
}

fn run_adl_with_env(args: &[&str], envs: &[(&str, &str)]) -> std::process::Output {
    let mut cmd = runtime_test_command(resolve_adl_exe());
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

fn resolve_csdlc_exe() -> PathBuf {
    let raw = std::env::var("CARGO_BIN_EXE_csdlc")
        .unwrap_or_else(|_| env!("CARGO_BIN_EXE_csdlc").to_string());
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

fn resolve_csm_exe() -> PathBuf {
    let raw = std::env::var("CARGO_BIN_EXE_csm")
        .unwrap_or_else(|_| env!("CARGO_BIN_EXE_csm").to_string());
    let path = PathBuf::from(raw);
    if path.is_absolute() {
        path
    } else {
        Path::new(env!("CARGO_MANIFEST_DIR")).join(path)
    }
}

fn resolve_csmctl_exe() -> PathBuf {
    let raw = std::env::var("CARGO_BIN_EXE_csmctl")
        .unwrap_or_else(|_| env!("CARGO_BIN_EXE_csmctl").to_string());
    let path = PathBuf::from(raw);
    if path.is_absolute() {
        path
    } else {
        Path::new(env!("CARGO_MANIFEST_DIR")).join(path)
    }
}

fn resolve_adl_review_exe() -> PathBuf {
    let raw = std::env::var("CARGO_BIN_EXE_adl-review")
        .unwrap_or_else(|_| env!("CARGO_BIN_EXE_adl-review").to_string());
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
fn csdlc_cli_binary_help_and_version_smoke() {
    let help = run_csdlc(&["--help"]);
    assert!(
        help.status.success(),
        "stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&help.stdout),
        String::from_utf8_lossy(&help.stderr)
    );
    let help_stdout = String::from_utf8_lossy(&help.stdout);
    assert!(help_stdout.contains("csdlc - ADL C-SDLC workflow control-plane binary"));
    assert!(help_stdout.contains("csdlc issue run <issue>"));
    assert!(help_stdout.contains("adl-csdlc remains a compatibility alias"));

    let version = run_csdlc(&["--version"]);
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
fn adl_review_cli_binary_help_and_version_smoke() {
    let help = run_adl_review(&["--help"]);
    assert!(
        help.status.success(),
        "stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&help.stdout),
        String::from_utf8_lossy(&help.stderr)
    );
    let help_stdout = String::from_utf8_lossy(&help.stdout);
    assert!(help_stdout.contains("adl-review - ADL review tooling compatibility binary"));
    assert!(help_stdout.contains("adl-review code-review --out <dir>"));
    assert!(help_stdout.contains("verify-repo-contract"));

    let version = run_adl_review(&["--version"]);
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
fn adl_review_verify_repo_contract_matches_legacy_tooling_command() {
    let review = unique_test_temp_dir("adl-review-contract").join("review.md");
    fs::write(
        &review,
        "# Repository Review\n\n\
## Metadata\n\n\
- Review Type: fixture\n\
- Subject: adl-review compatibility\n\
- Reviewer: fixture\n\n\
## Scope\n\n\
- Reviewed: review compatibility surface\n\
- Not Reviewed: runtime behavior\n\
- Review Mode: fixture\n\
- Gate: non-blocking\n\n\
## Findings\n\n\
No material findings.\n\n\
## System-Level Assessment\n\n\
The review packet is structurally valid for compatibility smoke coverage.\n\n\
## Recommended Action Plan\n\n\
- Fix now: none\n\
- Fix before milestone closeout: none\n\
- Defer: none\n\n\
## Follow-ups / Deferred Work\n\n\
None.\n\n\
## Final Assessment\n\n\
Pass.\n",
    )
    .expect("write review fixture");

    let legacy = run_adl(&[
        "tooling",
        "verify-repo-review-contract",
        "--review",
        review.to_str().unwrap(),
    ]);
    let review_bin =
        run_adl_review(&["verify-repo-contract", "--review", review.to_str().unwrap()]);

    assert!(
        legacy.status.success() && review_bin.status.success(),
        "legacy stderr:\n{}\nreview stderr:\n{}",
        String::from_utf8_lossy(&legacy.stderr),
        String::from_utf8_lossy(&review_bin.stderr)
    );
    assert_eq!(
        legacy.stdout, review_bin.stdout,
        "adl-review verify-repo-contract should preserve legacy tooling output"
    );
}

#[test]
fn adl_review_rejects_issue_and_runtime_families() {
    let issue = run_adl_review(&["pr", "run", "3599"]);
    assert_failure_contains(&issue, "review tooling only");

    let runtime = run_adl_review(&["run", "workflow.adl.yaml"]);
    assert_failure_contains(&runtime, "does not run ADL runtime commands");
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

#[test]
fn runtime_v2_constructability_anchor_validator_processes_input_and_preserves_channels() {
    let repo = unique_test_temp_dir("constructability-validator-process");
    fs::create_dir_all(&repo).expect("create temporary repository root");

    let fixture = Command::new(resolve_adl_exe())
        .current_dir(&repo)
        .args([
            "runtime-v2",
            "constructability-anchor-validator",
            "--out",
            "candidate.json",
        ])
        .output()
        .expect("emit constructability fixture");
    assert!(
        fixture.status.success(),
        "stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&fixture.stdout),
        String::from_utf8_lossy(&fixture.stderr)
    );
    assert_eq!(
        String::from_utf8_lossy(&fixture.stdout),
        "RUNTIME_V2_CONSTRUCTABILITY_ANCHOR_VALIDATOR_PATH=candidate.json\n"
    );
    assert!(String::from_utf8_lossy(&fixture.stderr).contains("adl_event"));

    let validated = Command::new(resolve_adl_exe())
        .current_dir(&repo)
        .args([
            "runtime-v2",
            "constructability-anchor-validator",
            "--input",
            "candidate.json",
            "--out",
            "validated.json",
        ])
        .output()
        .expect("validate constructability input");
    assert!(
        validated.status.success(),
        "stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&validated.stdout),
        String::from_utf8_lossy(&validated.stderr)
    );
    assert_eq!(
        String::from_utf8_lossy(&validated.stdout),
        "RUNTIME_V2_CONSTRUCTABILITY_ANCHOR_VALIDATOR_PATH=validated.json\n"
    );
    assert!(String::from_utf8_lossy(&validated.stderr).contains("adl_event"));
    assert_eq!(
        fs::read(repo.join("candidate.json")).expect("candidate packet"),
        fs::read(repo.join("validated.json")).expect("validated packet")
    );

    let mut invalid: serde_json::Value = serde_json::from_slice(
        &fs::read(repo.join("candidate.json")).expect("read candidate packet"),
    )
    .expect("parse candidate packet");
    invalid["decisions"]
        .as_array_mut()
        .expect("decision array")
        .iter_mut()
        .find(|decision| decision["event_id"] == "event-unanchored-promotion-attempt")
        .expect("unanchored decision")["outcome"] = serde_json::json!("pass");
    fs::write(
        repo.join("invalid.json"),
        serde_json::to_vec_pretty(&invalid).expect("serialize invalid packet"),
    )
    .expect("write invalid packet");

    let rejected = Command::new(resolve_adl_exe())
        .current_dir(&repo)
        .args([
            "runtime-v2",
            "constructability-anchor-validator",
            "--input",
            "invalid.json",
            "--out",
            "validated.json",
        ])
        .output()
        .expect("reject invalid constructability input");
    assert!(!rejected.status.success());
    assert!(rejected.stdout.is_empty());
    assert!(String::from_utf8_lossy(&rejected.stderr).contains("must fail closed"));
    assert!(
        !repo.join("validated.json").exists(),
        "failed validation must remove a stale output from the prior successful run"
    );

    fs::remove_dir_all(repo).ok();
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
#[path = "cli_smoke/process_status.rs"]
mod process_status;
