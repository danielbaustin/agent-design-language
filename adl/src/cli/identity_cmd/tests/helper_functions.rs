use super::*;

#[test]
fn required_value_and_git_capture_report_errors() {
    let value = required_value(&["--name".to_string(), "Codex".to_string()], 0, "--name")
        .expect("present flag value should succeed");
    assert_eq!(value, "Codex");

    let err = required_value(&["--name".to_string()], 0, "--name")
        .expect_err("missing flag value should fail");
    assert!(err.to_string().contains("--name requires a value"));

    let git_version = run_git_capture(&["--version"]).expect("git version should succeed");
    assert!(git_version.starts_with("git version "));

    let err = run_git_capture(&["definitely-not-a-real-subcommand"])
        .expect_err("invalid git command should fail");
    assert!(err
        .to_string()
        .contains("git definitely-not-a-real-subcommand failed with status"));
}

#[test]
fn resolve_identity_path_defaults_to_repo_identity_profile_path() {
    let repo = temp_repo("identity-path-default");

    let resolved = resolve_identity_path(&repo, &[]).expect("default path should resolve");

    assert_eq!(resolved, default_identity_profile_path(&repo));
}

#[test]
fn resolve_identity_path_accepts_explicit_path_and_rejects_unknown_args() {
    let repo = temp_repo("identity-path-explicit");

    let resolved = resolve_identity_path(
        &repo,
        &[
            "--path".to_string(),
            "identity/custom_profile.v1.json".to_string(),
        ],
    )
    .expect("explicit path should resolve");
    assert_eq!(resolved, PathBuf::from("identity/custom_profile.v1.json"));

    let err = resolve_identity_path(&repo, &["--bogus".to_string()])
        .expect_err("unknown arg should fail");
    assert!(err
        .to_string()
        .contains("unknown arg for identity show: --bogus"));

    let err = resolve_identity_path(&repo, &["--path".to_string()])
        .expect_err("missing path value should fail");
    assert!(err.to_string().contains("--path requires a value"));
}

#[test]
fn repo_root_matches_git_toplevel() {
    let _guard = TEST_MUTEX
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let repo = temp_repo("identity-repo-root");

    let original_cwd = env::current_dir().expect("capture cwd");
    env::set_current_dir(&repo).expect("set cwd to temp repo");

    let expected = PathBuf::from(
        run_git_capture(&["rev-parse", "--show-toplevel"])
            .expect("git top level")
            .trim(),
    );
    let resolved = repo_root().expect("repo root should resolve");

    env::set_current_dir(original_cwd).expect("restore cwd");
    assert_eq!(resolved, expected);
}

#[test]
fn run_git_capture_rejects_non_utf8_stdout() {
    let _guard = TEST_MUTEX
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let repo = temp_repo("identity-git-non-utf8");

    let mut hash = Command::new(system_git_bin())
        .args(["hash-object", "-w", "--stdin"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .current_dir(&repo)
        .spawn()
        .expect("spawn hash-object");
    hash.stdin
        .as_mut()
        .expect("stdin")
        .write_all(&[0xff, 0xfe, 0xfd])
        .expect("write invalid bytes");
    let output = hash.wait_with_output().expect("hash-object output");
    assert!(output.status.success(), "hash-object should succeed");

    let oid = String::from_utf8(output.stdout)
        .expect("oid utf8")
        .trim()
        .to_string();
    assert!(!oid.is_empty(), "oid should be present");

    let original_cwd = env::current_dir().expect("capture cwd");
    env::set_current_dir(&repo).expect("set cwd to temp repo");
    let err = run_git_capture(&["cat-file", "blob", oid.as_str()])
        .expect_err("binary blob output should fail utf8 decode");
    env::set_current_dir(original_cwd).expect("restore cwd");
    assert!(err.to_string().contains("git output was not valid UTF-8"));
}
