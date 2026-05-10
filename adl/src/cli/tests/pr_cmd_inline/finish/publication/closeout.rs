use super::*;
use crate::cli::pr_cmd::github::attach_post_merge_closeout;

#[test]
fn attach_post_merge_closeout_reports_failure_output() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-attach-closeout-failure");
    let repo = temp.join("repo");
    let tools_dir = repo.join("adl/tools");
    fs::create_dir_all(&tools_dir).expect("tools dir");
    write_executable(
        &tools_dir.join("attach_post_merge_closeout.sh"),
        "#!/usr/bin/env bash\nset -euo pipefail\necho 'closeout stdout'\necho 'closeout stderr' >&2\nexit 18\n",
    );

    let old_disable = env::var("ADL_POST_MERGE_CLOSEOUT_DISABLE").ok();
    let old_cmd = env::var("ADL_POST_MERGE_CLOSEOUT_CMD").ok();
    unsafe {
        env::remove_var("ADL_POST_MERGE_CLOSEOUT_CMD");
        env::set_var("ADL_POST_MERGE_CLOSEOUT_DISABLE", "0");
    }

    let err = attach_post_merge_closeout(
        &repo,
        "owner/repo",
        1153,
        "codex/1153-rust-finish-test",
        "https://github.com/owner/repo/pull/1159",
    )
    .expect_err("failing closeout helper should bubble up");

    unsafe {
        if let Some(value) = old_disable {
            env::set_var("ADL_POST_MERGE_CLOSEOUT_DISABLE", value);
        } else {
            env::remove_var("ADL_POST_MERGE_CLOSEOUT_DISABLE");
        }
        if let Some(value) = old_cmd {
            env::set_var("ADL_POST_MERGE_CLOSEOUT_CMD", value);
        } else {
            env::remove_var("ADL_POST_MERGE_CLOSEOUT_CMD");
        }
    }

    let message = err.to_string();
    assert!(message.contains("post-merge closeout auto-attach failed"));
    assert!(message.contains("closeout stderr"));
    assert!(message.contains("stdout: closeout stdout"));
}

#[test]
fn attach_post_merge_closeout_returns_early_when_disabled() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-attach-closeout-disabled");
    let repo = temp.join("repo");

    let old_disable = env::var("ADL_POST_MERGE_CLOSEOUT_DISABLE").ok();
    let old_cmd = env::var("ADL_POST_MERGE_CLOSEOUT_CMD").ok();
    unsafe {
        env::set_var("ADL_POST_MERGE_CLOSEOUT_DISABLE", "1");
        env::remove_var("ADL_POST_MERGE_CLOSEOUT_CMD");
    }

    attach_post_merge_closeout(
        &repo,
        "owner/repo",
        1153,
        "codex/1153-rust-finish-test",
        "https://github.com/owner/repo/pull/1159",
    )
    .expect("disabled closeout helper should be skipped");

    unsafe {
        if let Some(value) = old_disable {
            env::set_var("ADL_POST_MERGE_CLOSEOUT_DISABLE", value);
        } else {
            env::remove_var("ADL_POST_MERGE_CLOSEOUT_DISABLE");
        }
        if let Some(value) = old_cmd {
            env::set_var("ADL_POST_MERGE_CLOSEOUT_CMD", value);
        } else {
            env::remove_var("ADL_POST_MERGE_CLOSEOUT_CMD");
        }
    }
}

#[test]
fn attach_post_merge_closeout_invokes_helper_successfully() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-attach-closeout-success");
    let repo = temp.join("repo");
    let tools_dir = repo.join("adl/tools");
    let argv_log = temp.join("closeout-args.log");
    fs::create_dir_all(&tools_dir).expect("tools dir");
    write_executable(
        &tools_dir.join("attach_post_merge_closeout.sh"),
        &format!(
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" > '{}'\n",
            argv_log.display()
        ),
    );

    let old_disable = env::var("ADL_POST_MERGE_CLOSEOUT_DISABLE").ok();
    let old_cmd = env::var("ADL_POST_MERGE_CLOSEOUT_CMD").ok();
    unsafe {
        env::set_var("ADL_POST_MERGE_CLOSEOUT_DISABLE", "0");
        env::remove_var("ADL_POST_MERGE_CLOSEOUT_CMD");
    }

    attach_post_merge_closeout(
        &repo,
        "owner/repo",
        1153,
        "codex/1153-rust-finish-test",
        "https://github.com/owner/repo/pull/1159",
    )
    .expect("closeout helper should succeed");

    unsafe {
        if let Some(value) = old_disable {
            env::set_var("ADL_POST_MERGE_CLOSEOUT_DISABLE", value);
        } else {
            env::remove_var("ADL_POST_MERGE_CLOSEOUT_DISABLE");
        }
        if let Some(value) = old_cmd {
            env::set_var("ADL_POST_MERGE_CLOSEOUT_CMD", value);
        } else {
            env::remove_var("ADL_POST_MERGE_CLOSEOUT_CMD");
        }
    }

    let argv = fs::read_to_string(&argv_log).expect("closeout args");
    assert!(argv.contains("--repo owner/repo"));
    assert!(argv.contains("--issue 1153"));
    assert!(argv.contains("--branch codex/1153-rust-finish-test"));
    assert!(argv.contains("--pr-url https://github.com/owner/repo/pull/1159"));
}

#[test]
fn attach_post_merge_closeout_falls_back_when_command_override_is_blank() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-attach-closeout-blank-override");
    let repo = temp.join("repo");
    let tools_dir = repo.join("adl/tools");
    let argv_log = temp.join("closeout-args.log");
    fs::create_dir_all(&tools_dir).expect("tools dir");
    write_executable(
        &tools_dir.join("attach_post_merge_closeout.sh"),
        &format!(
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" > '{}'\n",
            argv_log.display()
        ),
    );

    let old_disable = env::var("ADL_POST_MERGE_CLOSEOUT_DISABLE").ok();
    let old_cmd = env::var("ADL_POST_MERGE_CLOSEOUT_CMD").ok();
    unsafe {
        env::set_var("ADL_POST_MERGE_CLOSEOUT_DISABLE", "0");
        env::set_var("ADL_POST_MERGE_CLOSEOUT_CMD", "   ");
    }

    attach_post_merge_closeout(
        &repo,
        "owner/repo",
        1153,
        "codex/1153-rust-finish-test",
        "https://github.com/owner/repo/pull/1159",
    )
    .expect("blank command override should fall back to helper path");

    unsafe {
        if let Some(value) = old_disable {
            env::set_var("ADL_POST_MERGE_CLOSEOUT_DISABLE", value);
        } else {
            env::remove_var("ADL_POST_MERGE_CLOSEOUT_DISABLE");
        }
        if let Some(value) = old_cmd {
            env::set_var("ADL_POST_MERGE_CLOSEOUT_CMD", value);
        } else {
            env::remove_var("ADL_POST_MERGE_CLOSEOUT_CMD");
        }
    }

    let argv = fs::read_to_string(&argv_log).expect("closeout args");
    assert!(argv.contains("--repo owner/repo"));
}
