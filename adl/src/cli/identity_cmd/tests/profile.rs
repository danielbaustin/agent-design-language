use super::*;

#[test]
fn identity_init_writes_default_profile_and_show_reads_it() {
    let _guard = TEST_MUTEX
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let repo = temp_repo("identity-init-show");

    real_identity_in_repo(
        &[
            "init".to_string(),
            "--name".to_string(),
            "Codex".to_string(),
            "--birthday".to_string(),
            "2026-03-30T13:34:00-07:00".to_string(),
            "--timezone".to_string(),
            "America/Los_Angeles".to_string(),
            "--created-by".to_string(),
            "daniel".to_string(),
        ],
        &repo,
    )
    .expect("identity init");

    let profile_path = repo.join("adl/identity/identity_profile.v1.json");
    assert!(profile_path.is_file(), "profile should exist");

    let profile = load_identity_profile(&profile_path).expect("profile load");
    assert_eq!(profile.agent_id, "codex");
    assert_eq!(profile.birth_weekday_local, "Monday");

    real_identity_in_repo(&["show".to_string()], &repo).expect("identity show");
}

#[test]
fn identity_now_requires_timezone_without_profile() {
    let _guard = TEST_MUTEX
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let repo = temp_repo("identity-now");

    let err = real_identity_in_repo(&["now".to_string()], &repo)
        .expect_err("should fail without timezone");
    assert!(err
        .to_string()
        .contains("identity now requires --timezone <IANA> when no profile exists"));
}

#[test]
fn identity_now_writes_temporal_context_json() {
    let _guard = TEST_MUTEX
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let repo = temp_repo("identity-now-out");

    real_identity_in_repo(
        &[
            "init".to_string(),
            "--name".to_string(),
            "Codex".to_string(),
            "--birthday".to_string(),
            "2026-03-30T13:34:00-07:00".to_string(),
            "--timezone".to_string(),
            "America/Los_Angeles".to_string(),
        ],
        &repo,
    )
    .expect("identity init");

    let out_path = repo.join(".adl/state/temporal_context.v1.json");
    real_identity_in_repo(
        &[
            "now".to_string(),
            "--out".to_string(),
            out_path.display().to_string(),
        ],
        &repo,
    )
    .expect("identity now");

    let json: Value =
        serde_json::from_slice(&fs::read(&out_path).expect("read out")).expect("parse json");
    assert_eq!(json["schema_version"], TEMPORAL_CONTEXT_SCHEMA);
    assert_eq!(json["identity_agent_id"], "codex");
}

#[test]
fn identity_init_validates_required_and_unknown_args() {
    let repo = temp_repo("identity-init-errors");

    let err = real_identity_in_repo(
        &[
            "init".to_string(),
            "--birthday".to_string(),
            "2026-03-30T13:34:00-07:00".to_string(),
            "--timezone".to_string(),
            "America/Los_Angeles".to_string(),
        ],
        &repo,
    )
    .expect_err("missing name should fail");
    assert!(err
        .to_string()
        .contains("identity init requires --name <display-name>"));

    let err = real_identity_in_repo(
        &[
            "init".to_string(),
            "--name".to_string(),
            "Codex".to_string(),
            "--timezone".to_string(),
            "America/Los_Angeles".to_string(),
        ],
        &repo,
    )
    .expect_err("missing birthday should fail");
    assert!(err
        .to_string()
        .contains("identity init requires --birthday <RFC3339>"));

    let err = real_identity_in_repo(
        &[
            "init".to_string(),
            "--name".to_string(),
            "Codex".to_string(),
            "--birthday".to_string(),
            "2026-03-30T13:34:00-07:00".to_string(),
        ],
        &repo,
    )
    .expect_err("missing timezone should fail");
    assert!(err
        .to_string()
        .contains("identity init requires --timezone <IANA>"));

    let err = real_identity_in_repo(&["init".to_string(), "--bogus".to_string()], &repo)
        .expect_err("unknown arg should fail");
    assert!(err
        .to_string()
        .contains("unknown arg for identity init: --bogus"));
}

#[test]
fn identity_init_supports_custom_path_agent_id_and_force() {
    let _guard = TEST_MUTEX
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let repo = temp_repo("identity-init-custom-path");
    let profile_path = repo.join(".adl/state/custom_identity_profile.v1.json");

    real_identity_in_repo(
        &[
            "init".to_string(),
            "--name".to_string(),
            "Codex".to_string(),
            "--agent-id".to_string(),
            "codex-local".to_string(),
            "--birthday".to_string(),
            "2026-03-30T13:34:00-07:00".to_string(),
            "--timezone".to_string(),
            "America/Los_Angeles".to_string(),
            "--path".to_string(),
            profile_path.display().to_string(),
        ],
        &repo,
    )
    .expect("custom path init");

    let profile = load_identity_profile(&profile_path).expect("profile load");
    assert_eq!(profile.agent_id, "codex-local");

    let err = real_identity_in_repo(
        &[
            "init".to_string(),
            "--name".to_string(),
            "Codex".to_string(),
            "--birthday".to_string(),
            "2026-03-30T13:34:00-07:00".to_string(),
            "--timezone".to_string(),
            "America/Los_Angeles".to_string(),
            "--path".to_string(),
            profile_path.display().to_string(),
        ],
        &repo,
    )
    .expect_err("existing profile without force should fail");
    assert!(err.to_string().contains("identity profile already exists"));

    real_identity_in_repo(
        &[
            "init".to_string(),
            "--name".to_string(),
            "Codex".to_string(),
            "--birthday".to_string(),
            "2026-03-30T13:34:00-07:00".to_string(),
            "--timezone".to_string(),
            "America/Los_Angeles".to_string(),
            "--path".to_string(),
            profile_path.display().to_string(),
            "--force".to_string(),
        ],
        &repo,
    )
    .expect("force overwrite");
}

#[test]
fn identity_show_supports_custom_path_and_rejects_unknown_args() {
    let _guard = TEST_MUTEX
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let repo = temp_repo("identity-show-path");
    let profile_path = repo.join("identity/custom_profile.v1.json");

    real_identity_in_repo(
        &[
            "init".to_string(),
            "--name".to_string(),
            "Codex".to_string(),
            "--birthday".to_string(),
            "2026-03-30T13:34:00-07:00".to_string(),
            "--timezone".to_string(),
            "America/Los_Angeles".to_string(),
            "--path".to_string(),
            profile_path.display().to_string(),
        ],
        &repo,
    )
    .expect("seed profile");

    real_identity_in_repo(
        &[
            "show".to_string(),
            "--path".to_string(),
            profile_path.display().to_string(),
        ],
        &repo,
    )
    .expect("show custom path");

    let err = real_identity_in_repo(&["show".to_string(), "--bogus".to_string()], &repo)
        .expect_err("show unknown arg");
    assert!(err
        .to_string()
        .contains("unknown arg for identity show: --bogus"));
}

#[test]
fn identity_now_validates_unknown_args_and_missing_out_value() {
    let repo = temp_repo("identity-now-errors");

    let err = real_identity_in_repo(
        &[
            "now".to_string(),
            "--timezone".to_string(),
            "America/Los_Angeles".to_string(),
            "--bogus".to_string(),
        ],
        &repo,
    )
    .expect_err("unknown arg should fail");
    assert!(err
        .to_string()
        .contains("unknown arg for identity now: --bogus"));

    let err = real_identity_in_repo(
        &[
            "now".to_string(),
            "--timezone".to_string(),
            "America/Los_Angeles".to_string(),
            "--out".to_string(),
        ],
        &repo,
    )
    .expect_err("out flag without value should fail");
    assert!(err.to_string().contains("--out requires a value"));
}
