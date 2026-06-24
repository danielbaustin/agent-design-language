use super::*;
use crate::cli::pr_cmd::github::{attach_issue_watcher, attach_pr_janitor};

#[test]
fn attach_pr_janitor_reports_failure_output() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-attach-janitor-failure");
    let repo = temp.join("repo");
    let tools_dir = repo.join("adl/tools");
    fs::create_dir_all(&tools_dir).expect("tools dir");
    write_executable(
        &tools_dir.join("attach_pr_janitor.sh"),
        "#!/usr/bin/env bash\nset -euo pipefail\necho 'janitor stdout'\necho 'janitor stderr' >&2\nexit 17\n",
    );

    let old_disable = env::var("ADL_PR_JANITOR_DISABLE").ok();
    let old_cmd = env::var("ADL_PR_JANITOR_CMD").ok();
    unsafe {
        env::remove_var("ADL_PR_JANITOR_CMD");
        env::set_var("ADL_PR_JANITOR_DISABLE", "0");
    }

    let err = attach_pr_janitor(
        &repo,
        "owner/repo",
        1153,
        "codex/1153-rust-finish-test",
        "https://github.com/owner/repo/pull/1159",
        "draft",
    )
    .expect_err("failing janitor helper should bubble up");

    unsafe {
        if let Some(value) = old_disable {
            env::set_var("ADL_PR_JANITOR_DISABLE", value);
        } else {
            env::remove_var("ADL_PR_JANITOR_DISABLE");
        }
        if let Some(value) = old_cmd {
            env::set_var("ADL_PR_JANITOR_CMD", value);
        } else {
            env::remove_var("ADL_PR_JANITOR_CMD");
        }
    }

    let message = err.to_string();
    assert!(message.contains("PR janitor auto-attach failed"));
    assert!(message.contains("janitor stderr"));
    assert!(message.contains("stdout: janitor stdout"));
}

#[test]
fn attach_pr_janitor_returns_early_when_disabled() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-attach-janitor-disabled");
    let repo = temp.join("repo");

    let old_disable = env::var("ADL_PR_JANITOR_DISABLE").ok();
    let old_cmd = env::var("ADL_PR_JANITOR_CMD").ok();
    unsafe {
        env::set_var("ADL_PR_JANITOR_DISABLE", "1");
        env::remove_var("ADL_PR_JANITOR_CMD");
    }

    attach_pr_janitor(
        &repo,
        "owner/repo",
        1153,
        "codex/1153-rust-finish-test",
        "https://github.com/owner/repo/pull/1159",
        "draft",
    )
    .expect("disabled janitor helper should be skipped");

    unsafe {
        if let Some(value) = old_disable {
            env::set_var("ADL_PR_JANITOR_DISABLE", value);
        } else {
            env::remove_var("ADL_PR_JANITOR_DISABLE");
        }
        if let Some(value) = old_cmd {
            env::set_var("ADL_PR_JANITOR_CMD", value);
        } else {
            env::remove_var("ADL_PR_JANITOR_CMD");
        }
    }
}

#[test]
fn attach_pr_janitor_invokes_helper_successfully() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-attach-janitor-success");
    let repo = temp.join("repo");
    let tools_dir = repo.join("adl/tools");
    let argv_log = temp.join("janitor-args.log");
    fs::create_dir_all(&tools_dir).expect("tools dir");
    write_executable(
        &tools_dir.join("attach_pr_janitor.sh"),
        &format!(
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" > '{}'\n",
            argv_log.display()
        ),
    );

    let old_disable = env::var("ADL_PR_JANITOR_DISABLE").ok();
    let old_cmd = env::var("ADL_PR_JANITOR_CMD").ok();
    unsafe {
        env::set_var("ADL_PR_JANITOR_DISABLE", "0");
        env::remove_var("ADL_PR_JANITOR_CMD");
    }

    attach_pr_janitor(
        &repo,
        "owner/repo",
        1153,
        "codex/1153-rust-finish-test",
        "https://github.com/owner/repo/pull/1159",
        "draft",
    )
    .expect("janitor helper should succeed");

    unsafe {
        if let Some(value) = old_disable {
            env::set_var("ADL_PR_JANITOR_DISABLE", value);
        } else {
            env::remove_var("ADL_PR_JANITOR_DISABLE");
        }
        if let Some(value) = old_cmd {
            env::set_var("ADL_PR_JANITOR_CMD", value);
        } else {
            env::remove_var("ADL_PR_JANITOR_CMD");
        }
    }

    let argv = fs::read_to_string(&argv_log).expect("janitor args");
    assert!(argv.contains("--repo owner/repo"));
    assert!(argv.contains("--issue 1153"));
    assert!(argv.contains("--branch codex/1153-rust-finish-test"));
    assert!(argv.contains("--pr-url https://github.com/owner/repo/pull/1159"));
    assert!(argv.contains("--expected-pr-state draft"));
}

#[test]
fn attach_pr_janitor_falls_back_when_command_override_is_blank() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-attach-janitor-blank-override");
    let repo = temp.join("repo");
    let tools_dir = repo.join("adl/tools");
    let argv_log = temp.join("janitor-args.log");
    fs::create_dir_all(&tools_dir).expect("tools dir");
    write_executable(
        &tools_dir.join("attach_pr_janitor.sh"),
        &format!(
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" > '{}'\n",
            argv_log.display()
        ),
    );

    let old_disable = env::var("ADL_PR_JANITOR_DISABLE").ok();
    let old_cmd = env::var("ADL_PR_JANITOR_CMD").ok();
    unsafe {
        env::set_var("ADL_PR_JANITOR_DISABLE", "0");
        env::set_var("ADL_PR_JANITOR_CMD", "   ");
    }

    attach_pr_janitor(
        &repo,
        "owner/repo",
        1153,
        "codex/1153-rust-finish-test",
        "https://github.com/owner/repo/pull/1159",
        "draft",
    )
    .expect("blank command override should fall back to helper path");

    unsafe {
        if let Some(value) = old_disable {
            env::set_var("ADL_PR_JANITOR_DISABLE", value);
        } else {
            env::remove_var("ADL_PR_JANITOR_DISABLE");
        }
        if let Some(value) = old_cmd {
            env::set_var("ADL_PR_JANITOR_CMD", value);
        } else {
            env::remove_var("ADL_PR_JANITOR_CMD");
        }
    }

    let argv = fs::read_to_string(&argv_log).expect("janitor args");
    assert!(argv.contains("--repo owner/repo"));
}

#[test]
fn attach_issue_watcher_reports_failure_output() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-attach-watcher-failure");
    let repo = temp.join("repo");
    let watcher_path = temp.join("watcher");
    write_executable(
        &watcher_path,
        "#!/usr/bin/env bash\nset -euo pipefail\necho 'watcher stdout'\necho 'watcher stderr' >&2\nexit 17\n",
    );

    let old_disable = env::var("ADL_ISSUE_WATCHER_DISABLE").ok();
    let old_cmd = env::var("ADL_ISSUE_WATCHER_CMD").ok();
    unsafe {
        env::set_var("ADL_ISSUE_WATCHER_DISABLE", "0");
        env::set_var("ADL_ISSUE_WATCHER_CMD", &watcher_path);
    }

    let err = attach_issue_watcher(
        &repo,
        "owner/repo",
        1153,
        "codex/1153-rust-finish-test",
        "https://github.com/owner/repo/pull/1159",
        "draft",
        "pr_open",
        "issue-watcher",
        "watcher_owned_pr_open",
    )
    .expect_err("failing watcher helper should bubble up");

    unsafe {
        if let Some(value) = old_disable {
            env::set_var("ADL_ISSUE_WATCHER_DISABLE", value);
        } else {
            env::remove_var("ADL_ISSUE_WATCHER_DISABLE");
        }
        if let Some(value) = old_cmd {
            env::set_var("ADL_ISSUE_WATCHER_CMD", value);
        } else {
            env::remove_var("ADL_ISSUE_WATCHER_CMD");
        }
    }

    let message = err.to_string();
    assert!(message.contains("issue watcher auto-attach failed"));
    assert!(message.contains("watcher stderr"));
    assert!(message.contains("stdout: watcher stdout"));
}

#[test]
fn attach_issue_watcher_returns_early_when_disabled() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-attach-watcher-disabled");
    let repo = temp.join("repo");

    let old_disable = env::var("ADL_ISSUE_WATCHER_DISABLE").ok();
    let old_cmd = env::var("ADL_ISSUE_WATCHER_CMD").ok();
    unsafe {
        env::set_var("ADL_ISSUE_WATCHER_DISABLE", "1");
        env::remove_var("ADL_ISSUE_WATCHER_CMD");
    }

    attach_issue_watcher(
        &repo,
        "owner/repo",
        1153,
        "codex/1153-rust-finish-test",
        "https://github.com/owner/repo/pull/1159",
        "draft",
        "pr_open",
        "issue-watcher",
        "watcher_owned_pr_open",
    )
    .expect("disabled watcher helper should be skipped");

    unsafe {
        if let Some(value) = old_disable {
            env::set_var("ADL_ISSUE_WATCHER_DISABLE", value);
        } else {
            env::remove_var("ADL_ISSUE_WATCHER_DISABLE");
        }
        if let Some(value) = old_cmd {
            env::set_var("ADL_ISSUE_WATCHER_CMD", value);
        } else {
            env::remove_var("ADL_ISSUE_WATCHER_CMD");
        }
    }
}

#[test]
fn attach_issue_watcher_invokes_helper_successfully() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-attach-watcher-success");
    let repo = temp.join("repo");
    let argv_log = temp.join("watcher-args.log");
    let watcher_path = temp.join("watcher");
    write_executable(
        &watcher_path,
        &format!(
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" > '{}'\n",
            argv_log.display()
        ),
    );

    let old_disable = env::var("ADL_ISSUE_WATCHER_DISABLE").ok();
    let old_cmd = env::var("ADL_ISSUE_WATCHER_CMD").ok();
    unsafe {
        env::set_var("ADL_ISSUE_WATCHER_DISABLE", "0");
        env::set_var("ADL_ISSUE_WATCHER_CMD", &watcher_path);
    }

    attach_issue_watcher(
        &repo,
        "owner/repo",
        1153,
        "codex/1153-rust-finish-test",
        "https://github.com/owner/repo/pull/1159",
        "draft",
        "pr_open",
        "issue-watcher",
        "watcher_owned_pr_open",
    )
    .expect("watcher helper should succeed");

    unsafe {
        if let Some(value) = old_disable {
            env::set_var("ADL_ISSUE_WATCHER_DISABLE", value);
        } else {
            env::remove_var("ADL_ISSUE_WATCHER_DISABLE");
        }
        if let Some(value) = old_cmd {
            env::set_var("ADL_ISSUE_WATCHER_CMD", value);
        } else {
            env::remove_var("ADL_ISSUE_WATCHER_CMD");
        }
    }

    let argv = fs::read_to_string(&argv_log).expect("watcher args");
    assert!(argv.contains("--repo owner/repo"));
    assert!(argv.contains("--issue 1153"));
    assert!(argv.contains("--branch codex/1153-rust-finish-test"));
    assert!(argv.contains("--pr-url https://github.com/owner/repo/pull/1159"));
    assert!(argv.contains("--expected-pr-state draft"));
    assert!(argv.contains("--classification pr_open"));
    assert!(argv.contains("--tail-owner issue-watcher"));
    assert!(argv.contains("--shepherd-state watcher_owned_pr_open"));
}
