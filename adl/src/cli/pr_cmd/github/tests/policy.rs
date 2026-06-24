use super::*;

#[test]
fn live_gh_policy_guard_blocks_disabled_fallback_before_spawn() {
    let _guard = env_lock();
    let policy_env = clear_github_policy_env();
    let temp = unique_temp_dir("adl-github-disabled-fallback");
    let old_home = std::env::var("HOME").ok();
    let bin_dir = temp.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let gh_log = temp.join("gh.log");
    write_executable(
            &bin_dir.join("gh"),
            &format!(
                "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\nprintf 'unexpected gh spawn\\n'\n",
                gh_log.display()
            ),
        );
    let old_path = std::env::var("PATH").ok();
    let mut path_entries = vec![bin_dir.clone()];
    path_entries.extend(std::env::split_paths(old_path.as_deref().unwrap_or("")));
    unsafe {
        std::env::set_var(
            "PATH",
            std::env::join_paths(path_entries).expect("join PATH"),
        );
        std::env::set_var("HOME", &temp);
        std::env::set_var("ADL_GITHUB_DISABLE_GH_FALLBACK", "1");
    }

    let err = current_pr_url("owner/repo", "codex/3672-branch")
        .expect_err("fallback-disabled current_pr_url should fail closed");
    let err_debug = format!("{err:?}");
    assert!(err_debug.contains("pr.list.current_branch"));
    assert!(err_debug.contains("github_client.fallback_disabled"));
    let err = gh_issue_edit_body("owner/repo", 3672, "body")
        .expect_err("fallback-disabled issue edit should fail closed");
    let err_debug = format!("{err:?}");
    assert!(err_debug.contains("issue.edit.body"));
    assert!(err_debug.contains("github_client.fallback_disabled"));
    assert!(
        !gh_log.exists(),
        "policy guard should reject before spawning gh"
    );

    restore_env("PATH", old_path);
    restore_env("HOME", old_home);
    restore_github_policy_env(policy_env);
}

#[test]
fn live_github_policy_blocks_explicit_gh_fallback_before_spawn() {
    let _guard = env_lock();
    let policy_env = clear_github_policy_env();
    let temp = unique_temp_dir("adl-github-explicit-gh-fallback");
    let bin_dir = temp.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let gh_log = temp.join("gh.log");
    write_executable(
            &bin_dir.join("gh"),
            &format!(
                "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\nprintf 'unexpected gh spawn\\n'\n",
                gh_log.display()
            ),
        );
    let old_path = std::env::var("PATH").ok();
    let mut path_entries = vec![bin_dir.clone()];
    path_entries.extend(std::env::split_paths(old_path.as_deref().unwrap_or("")));
    unsafe {
        std::env::set_var(
            "PATH",
            std::env::join_paths(path_entries).expect("join PATH"),
        );
        std::env::set_var("ADL_GITHUB_CLIENT", "gh");
        std::env::set_var("GITHUB_TOKEN", "test-token");
    }

    let err = current_pr_url("owner/repo", "codex/3672-branch")
        .expect_err("explicit gh fallback current_pr_url should fail closed");
    let err_debug = format!("{err:?}");
    assert!(err_debug.contains("pr.list.current_branch"));
    assert!(err_debug.contains("github_client.gh_fallback_removed"));
    assert!(err_debug.contains("credential_status=token_present"));
    assert!(err_debug.contains("source=GITHUB_TOKEN"));
    assert!(!err_debug.contains("test-token"));
    let err = gh_issue_edit_body("owner/repo", 3672, "body")
        .expect_err("explicit gh fallback issue edit should fail closed");
    let err_debug = format!("{err:?}");
    assert!(err_debug.contains("github_client.gh_fallback_removed"));
    assert!(err_debug.contains("credential_status=token_present"));
    assert!(!err_debug.contains("test-token"));
    assert!(
        !gh_log.exists(),
        "fallback removal should reject before spawning gh"
    );

    restore_env("PATH", old_path);
    restore_github_policy_env(policy_env);
}

#[test]
fn live_github_policy_explains_missing_token_before_spawn() {
    let _guard = env_lock();
    let policy_env = clear_github_policy_env();
    let temp = unique_temp_dir("adl-github-missing-token");
    let old_home = std::env::var("HOME").ok();
    unsafe {
        std::env::set_var("HOME", &temp);
        std::env::set_var("ADL_GITHUB_CLIENT", "gh");
    }

    let err = current_pr_url("owner/repo", "codex/3805-branch")
        .expect_err("explicit gh fallback without token should explain credential preflight");
    let err_debug = format!("{err:?}");
    assert!(err_debug.contains("github_client.gh_fallback_removed"));
    assert!(err_debug.contains("credential_status=missing_token"));
    assert!(err_debug.contains("GITHUB_TOKEN"));
    assert!(err_debug.contains("GH_TOKEN"));
    assert!(err_debug.contains("ADL_GITHUB_TOKEN_FILE"));
    assert!(err_debug.contains("ADL_GITHUB_TOKEN_KEYCHAIN_SERVICE"));
    assert!(err_debug.contains("operator-approved secret source"));
    assert!(err_debug.contains("do not fall back to direct gh commands"));
    assert!(err_debug.contains("credential values are never printed"));

    restore_env("HOME", old_home);
    restore_github_policy_env(policy_env);
}
