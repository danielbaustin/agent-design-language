use super::*;

#[test]
fn closing_linkage_helpers_cover_reference_body_repair_and_error_paths() {
    let _guard = env_lock();
    let policy_env = clear_github_policy_env();
    let temp = unique_temp_dir("adl-github-closing-linkage");
    let bin_dir = temp.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let gh_log = temp.join("gh.log");
    let state_dir = temp.join("state");
    fs::create_dir_all(&state_dir).expect("state dir");

    let linked_ref = state_dir.join("linked_ref.txt");
    let linked_body = state_dir.join("linked_body.txt");
    let unlinked_ref = state_dir.join("unlinked_ref.txt");
    let unlinked_body = state_dir.join("unlinked_body.txt");
    let repair_ref = state_dir.join("repair_ref.txt");
    let repair_body = state_dir.join("repair_body.txt");
    fs::write(&linked_ref, "1153\n").expect("linked refs");
    fs::write(&linked_body, "Refs #1153\n").expect("linked body");
    fs::write(&unlinked_ref, "").expect("unlinked refs");
    fs::write(&unlinked_body, "Refs #9999\n").expect("unlinked body");
    fs::write(&repair_ref, "").expect("repair refs");
    fs::write(&repair_body, "Refs #1153\n").expect("repair body");

    let github_cli_fixture = bin_dir.join("github-cli-fixture");
    write_executable(
            &github_cli_fixture,
            &format!(
                "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\nif [ \"$1 $2\" = 'pr list' ]; then\n  printf 'https://github.com/owner/repo/pull/1159\\n'\n  exit 0\nfi\nif [ \"$1 $2\" = 'pr view' ]; then\n  pr_ref=''\n  for arg in \"$@\"; do\n    case \"$arg\" in\n      https://github.com/owner/repo/pull/1159|https://github.com/owner/repo/pull/1160|https://github.com/owner/repo/pull/1161)\n        pr_ref=\"$arg\"\n        ;;\n    esac\n  done\n  case \"$pr_ref\" in\n    https://github.com/owner/repo/pull/1159)\n      refs='{}'\n      body='{}'\n      ;;\n    https://github.com/owner/repo/pull/1160)\n      refs='{}'\n      body='{}'\n      ;;\n    https://github.com/owner/repo/pull/1161)\n      refs='{}'\n      body='{}'\n      ;;\n    *)\n      exit 13\n      ;;\n  esac\n  if printf '%s ' \"$@\" | grep -q 'closingIssuesReferences'; then\n    cat \"$refs\"\n    exit 0\n  fi\n  if printf '%s ' \"$@\" | grep -q ' --json body '; then\n    cat \"$body\"\n    exit 0\n  fi\n  exit 14\nfi\nif [ \"$1 $2\" = 'pr edit' ]; then\n  pr_ref=''\n  body_file=''\n  while [ $# -gt 0 ]; do\n    case \"$1\" in\n      https://github.com/owner/repo/pull/1161)\n        pr_ref=\"$1\"\n        shift\n        ;;\n      --body-file)\n        body_file=\"$2\"\n        shift 2\n        ;;\n      *)\n        shift\n        ;;\n    esac\n  done\n  [ \"$pr_ref\" = 'https://github.com/owner/repo/pull/1161' ] || exit 15\n  cp \"$body_file\" '{}'\n  printf '1153\\n' > '{}'\n  exit 0\nfi\nexit 16\n",
                gh_log.display(),
                linked_ref.display(),
                linked_body.display(),
                unlinked_ref.display(),
                unlinked_body.display(),
                repair_ref.display(),
                repair_body.display(),
                repair_body.display(),
                repair_ref.display()
            ),
        );

    let desired_body = temp.join("desired.md");
    fs::write(&desired_body, "Closes #1153\n\n## Summary\nrepaired\n").expect("desired body");

    let old_path = std::env::var("PATH").ok();
    let mut path_entries = vec![bin_dir.clone()];
    path_entries.extend(std::env::split_paths(old_path.as_deref().unwrap_or("")));
    unsafe {
        std::env::set_var("ADL_TEST_GITHUB_CLI_FIXTURE", &github_cli_fixture);
        std::env::set_var(
            "PATH",
            std::env::join_paths(path_entries).expect("join PATH"),
        );
    }

    assert_eq!(
        current_pr_url("owner/repo", "codex/1153-branch")
            .expect("current pr")
            .as_deref(),
        Some("https://github.com/owner/repo/pull/1159")
    );
    assert!(pr_has_closing_linkage(
        "owner/repo",
        "https://github.com/owner/repo/pull/1159",
        1153
    )
    .expect("linked ref"));
    assert!(!pr_has_closing_linkage(
        "owner/repo",
        "https://github.com/owner/repo/pull/1160",
        1153
    )
    .expect("unlinked"));
    ensure_pr_closing_linkage(
        "owner/repo",
        "https://github.com/owner/repo/pull/1159",
        1153,
        true,
    )
    .expect("no-close skip");
    let err = ensure_pr_closing_linkage(
        "owner/repo",
        "https://github.com/owner/repo/pull/1160",
        1153,
        false,
    )
    .expect_err("missing linkage should fail");
    assert!(err
        .to_string()
        .contains("missing closing linkage to issue #1153"));

    let repaired = ensure_or_repair_pr_closing_linkage(
        "owner/repo",
        "https://github.com/owner/repo/pull/1161",
        1153,
        false,
        &desired_body,
    )
    .expect("repair should succeed");
    assert!(repaired);
    assert!(pr_has_closing_linkage(
        "owner/repo",
        "https://github.com/owner/repo/pull/1161",
        1153
    )
    .expect("linked after repair"));

    restore_env("PATH", old_path);

    let gh_calls = fs::read_to_string(&gh_log).expect("gh log");
    assert!(gh_calls
        .contains("pr edit -R owner/repo https://github.com/owner/repo/pull/1161 --body-file"));
    restore_github_policy_env(policy_env);
}

#[test]
fn helper_attach_commands_cover_disabled_success_failure_and_fallback_paths() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-github-attach-helpers");
    let repo = temp.join("repo");
    let tools_dir = repo.join("adl/tools");
    fs::create_dir_all(&tools_dir).expect("repo tools");

    let janitor_success = temp.join("janitor-success.log");
    let closeout_success = temp.join("closeout-success.log");

    write_executable(
        &tools_dir.join("attach_pr_janitor.sh"),
        &format!(
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\nprintf 'ctx ADL_GITHUB_CLIENT=%s GH_TOKEN_PRESENT=%s\\n' \"${{ADL_GITHUB_CLIENT:-missing}}\" \"${{GH_TOKEN:+present}}\" >> '{}'\n",
            janitor_success.display(),
            janitor_success.display()
        ),
    );
    write_executable(
        &tools_dir.join("attach_post_merge_closeout.sh"),
        &format!(
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\nprintf 'ctx ADL_GITHUB_CLIENT=%s GH_TOKEN_PRESENT=%s\\n' \"${{ADL_GITHUB_CLIENT:-missing}}\" \"${{GH_TOKEN:+present}}\" >> '{}'\n",
            closeout_success.display(),
            closeout_success.display()
        ),
    );
    let failing = temp.join("failing-helper.sh");
    write_executable(
            &failing,
            "#!/usr/bin/env bash\nset -euo pipefail\necho 'helper stdout'\necho 'helper stderr' >&2\nexit 9\n",
        );

    let old_janitor_disable = std::env::var("ADL_PR_JANITOR_DISABLE").ok();
    let old_janitor_cmd = std::env::var("ADL_PR_JANITOR_CMD").ok();
    let old_closeout_disable = std::env::var("ADL_POST_MERGE_CLOSEOUT_DISABLE").ok();
    let old_closeout_cmd = std::env::var("ADL_POST_MERGE_CLOSEOUT_CMD").ok();
    let old_github_client = std::env::var("ADL_GITHUB_CLIENT").ok();
    let old_gh_token = std::env::var("GH_TOKEN").ok();

    unsafe {
        std::env::set_var("ADL_GITHUB_CLIENT", "octocrab");
        std::env::set_var("GH_TOKEN", "gh-token-from-parent");
        std::env::set_var("ADL_PR_JANITOR_DISABLE", "1");
        std::env::remove_var("ADL_PR_JANITOR_CMD");
    }
    attach_pr_janitor(
        &repo,
        "owner/repo",
        1153,
        "codex/1153-branch",
        "https://github.com/owner/repo/pull/1159",
        "draft",
    )
    .expect("disabled janitor should skip");

    unsafe {
        std::env::set_var("ADL_PR_JANITOR_DISABLE", "0");
        std::env::remove_var("ADL_PR_JANITOR_CMD");
    }
    attach_pr_janitor(
        &repo,
        "owner/repo",
        1153,
        "codex/1153-branch",
        "https://github.com/owner/repo/pull/1159",
        "draft",
    )
    .expect("repo helper janitor");

    unsafe {
        std::env::set_var("ADL_PR_JANITOR_CMD", "   ");
    }
    attach_pr_janitor(
        &repo,
        "owner/repo",
        1153,
        "codex/1153-branch",
        "https://github.com/owner/repo/pull/1159",
        "ready",
    )
    .expect("blank override janitor fallback");

    unsafe {
        std::env::set_var("ADL_PR_JANITOR_CMD", &failing);
    }
    let err = attach_pr_janitor(
        &repo,
        "owner/repo",
        1153,
        "codex/1153-branch",
        "https://github.com/owner/repo/pull/1159",
        "draft",
    )
    .expect_err("failing janitor should bubble");
    assert!(err.to_string().contains("PR janitor auto-attach failed"));
    assert!(err.to_string().contains("helper stderr"));
    assert!(err.to_string().contains("stdout: helper stdout"));

    unsafe {
        std::env::set_var("ADL_POST_MERGE_CLOSEOUT_DISABLE", "1");
        std::env::remove_var("ADL_POST_MERGE_CLOSEOUT_CMD");
    }
    attach_post_merge_closeout(
        &repo,
        "owner/repo",
        1153,
        "codex/1153-branch",
        "https://github.com/owner/repo/pull/1159",
    )
    .expect("disabled closeout should skip");

    unsafe {
        std::env::set_var("ADL_POST_MERGE_CLOSEOUT_DISABLE", "0");
        std::env::set_var(
            "ADL_POST_MERGE_CLOSEOUT_CMD",
            tools_dir.join("attach_post_merge_closeout.sh"),
        );
    }
    attach_post_merge_closeout(
        &repo,
        "owner/repo",
        1153,
        "codex/1153-branch",
        "https://github.com/owner/repo/pull/1159",
    )
    .expect("repo helper closeout");

    unsafe {
        std::env::set_var("ADL_POST_MERGE_CLOSEOUT_CMD", "   ");
    }
    attach_post_merge_closeout(
        &repo,
        "owner/repo",
        1153,
        "codex/1153-branch",
        "https://github.com/owner/repo/pull/1159",
    )
    .expect("blank override closeout skip");

    unsafe {
        std::env::set_var("ADL_POST_MERGE_CLOSEOUT_CMD", &failing);
    }
    let err = attach_post_merge_closeout(
        &repo,
        "owner/repo",
        1153,
        "codex/1153-branch",
        "https://github.com/owner/repo/pull/1159",
    )
    .expect_err("failing closeout should bubble");
    assert!(err
        .to_string()
        .contains("post-merge closeout auto-attach failed"));
    assert!(err.to_string().contains("helper stderr"));
    assert!(err.to_string().contains("stdout: helper stdout"));

    restore_env("ADL_PR_JANITOR_DISABLE", old_janitor_disable);
    restore_env("ADL_PR_JANITOR_CMD", old_janitor_cmd);
    restore_env("ADL_POST_MERGE_CLOSEOUT_DISABLE", old_closeout_disable);
    restore_env("ADL_POST_MERGE_CLOSEOUT_CMD", old_closeout_cmd);

    let janitor_calls = fs::read_to_string(&janitor_success).expect("janitor success log");
    assert!(janitor_calls.contains("--expected-pr-state draft"));
    assert!(janitor_calls.contains("--expected-pr-state ready"));
    assert!(janitor_calls.contains("ctx ADL_GITHUB_CLIENT=octocrab GH_TOKEN_PRESENT=present"));
    let closeout_calls = fs::read_to_string(&closeout_success).expect("closeout success log");
    assert!(closeout_calls.contains("--pr-url https://github.com/owner/repo/pull/1159"));
    assert!(closeout_calls.contains("ctx ADL_GITHUB_CLIENT=octocrab GH_TOKEN_PRESENT=present"));

    restore_env("ADL_GITHUB_CLIENT", old_github_client);
    restore_env("GH_TOKEN", old_gh_token);
}

#[test]
fn helper_attach_failures_redact_token_like_output() {
    let _guard = env_lock();
    let policy_env = clear_github_policy_env();
    let temp = unique_temp_dir("adl-github-helper-redaction");
    let failing = temp.join("failing-helper.sh");
    write_executable(
            &failing,
            "#!/usr/bin/env bash\nset -euo pipefail\necho 'stdout ghp_stdout_secret {\"token\":\"ghp_json_secret\"}'\necho 'stderr token=ghp_stderr_secret github_pat_secret \"github_pat_quoted_secret\"' >&2\nexit 9\n",
        );

    let old_janitor_disable = std::env::var("ADL_PR_JANITOR_DISABLE").ok();
    let old_janitor_cmd = std::env::var("ADL_PR_JANITOR_CMD").ok();
    let old_closeout_disable = std::env::var("ADL_POST_MERGE_CLOSEOUT_DISABLE").ok();
    let old_closeout_cmd = std::env::var("ADL_POST_MERGE_CLOSEOUT_CMD").ok();
    let old_gh_token = std::env::var("GH_TOKEN").ok();

    unsafe {
        std::env::set_var("GH_TOKEN", "ghp_parent_secret");
        std::env::set_var("ADL_PR_JANITOR_CMD", &failing);
        std::env::set_var("ADL_PR_JANITOR_DISABLE", "0");
    }
    let err = attach_pr_janitor(
        temp.as_path(),
        "owner/repo",
        1153,
        "codex/1153-branch",
        "https://github.com/owner/repo/pull/1159",
        "draft",
    )
    .expect_err("failing janitor should bubble");
    let err = err.to_string();
    assert!(err.contains("<redacted>"));
    assert!(!err.contains("ghp_stdout_secret"));
    assert!(!err.contains("ghp_stderr_secret"));
    assert!(!err.contains("ghp_json_secret"));
    assert!(!err.contains("github_pat_secret"));
    assert!(!err.contains("github_pat_quoted_secret"));
    assert!(!err.contains("ghp_parent_secret"));

    unsafe {
        std::env::set_var("ADL_POST_MERGE_CLOSEOUT_CMD", &failing);
        std::env::set_var("ADL_POST_MERGE_CLOSEOUT_DISABLE", "0");
    }
    let err = attach_post_merge_closeout(
        temp.as_path(),
        "owner/repo",
        1153,
        "codex/1153-branch",
        "https://github.com/owner/repo/pull/1159",
    )
    .expect_err("failing closeout should bubble");
    let err = err.to_string();
    assert!(err.contains("<redacted>"));
    assert!(!err.contains("ghp_stdout_secret"));
    assert!(!err.contains("ghp_stderr_secret"));
    assert!(!err.contains("ghp_json_secret"));
    assert!(!err.contains("github_pat_secret"));
    assert!(!err.contains("github_pat_quoted_secret"));
    assert!(!err.contains("ghp_parent_secret"));

    restore_env("ADL_PR_JANITOR_DISABLE", old_janitor_disable);
    restore_env("ADL_PR_JANITOR_CMD", old_janitor_cmd);
    restore_env("ADL_POST_MERGE_CLOSEOUT_DISABLE", old_closeout_disable);
    restore_env("ADL_POST_MERGE_CLOSEOUT_CMD", old_closeout_cmd);
    restore_env("GH_TOKEN", old_gh_token);
    restore_github_policy_env(policy_env);
}

#[test]
fn github_helpers_cover_fallback_and_spawn_failure_paths() {
    let _guard = env_lock();
    let policy_env = clear_github_policy_env();
    let temp = unique_temp_dir("adl-github-helper-fallbacks");
    let bin_dir = temp.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let body_ref = temp.join("body-ref.txt");
    let body_text = temp.join("body.txt");
    fs::write(&body_ref, "").expect("empty refs");
    fs::write(&body_text, "Closes #1153\n").expect("body text");
    let github_cli_fixture = bin_dir.join("github-cli-fixture");
    write_executable(
            &github_cli_fixture,
            &format!(
                "#!/usr/bin/env bash\nset -euo pipefail\nif [ \"$1 $2\" = 'pr view' ]; then\n  if printf '%s ' \"$@\" | grep -q 'closingIssuesReferences'; then\n    cat '{}'\n    exit 0\n  fi\n  if printf '%s ' \"$@\" | grep -q ' --json body '; then\n    cat '{}'\n    exit 0\n  fi\nfi\nif [ \"$1 $2\" = 'issue view' ]; then\n  if printf '%s ' \"$@\" | grep -q 'labels'; then\n    printf 'track:roadmap\\n'\n  else\n    printf 'Tracking issue without version\\n'\n  fi\n  exit 0\nfi\nexit 1\n",
                body_ref.display(),
                body_text.display()
            ),
        );

    let old_path = std::env::var("PATH").ok();
    let mut path_entries = vec![bin_dir.clone()];
    path_entries.extend(std::env::split_paths(old_path.as_deref().unwrap_or("")));
    unsafe {
        std::env::set_var("ADL_TEST_GITHUB_CLI_FIXTURE", &github_cli_fixture);
        std::env::set_var(
            "PATH",
            std::env::join_paths(path_entries).expect("join PATH"),
        );
    }

    assert!(pr_has_closing_linkage(
        "owner/repo",
        "https://github.com/owner/repo/pull/1159",
        1153
    )
    .expect("body fallback should count"));
    assert_eq!(
        issue_version(1153, "owner/repo").expect("no inferred version"),
        None
    );

    restore_env("PATH", old_path);

    let missing = temp.join("missing-helper.sh");
    unsafe {
        std::env::set_var("ADL_PR_JANITOR_CMD", &missing);
        std::env::set_var("ADL_PR_JANITOR_DISABLE", "0");
    }
    let err = attach_pr_janitor(
        temp.as_path(),
        "owner/repo",
        1153,
        "codex/1153-branch",
        "https://github.com/owner/repo/pull/1159",
        "draft",
    )
    .expect_err("missing janitor helper should surface spawn failure");
    assert!(err
        .to_string()
        .contains("failed to spawn PR janitor command"));

    unsafe {
        std::env::set_var("ADL_POST_MERGE_CLOSEOUT_CMD", &missing);
        std::env::set_var("ADL_POST_MERGE_CLOSEOUT_DISABLE", "0");
    }
    let err = attach_post_merge_closeout(
        temp.as_path(),
        "owner/repo",
        1153,
        "codex/1153-branch",
        "https://github.com/owner/repo/pull/1159",
    )
    .expect_err("missing closeout helper should surface spawn failure");
    assert!(err
        .to_string()
        .contains("failed to spawn post-merge closeout command"));

    unsafe {
        std::env::remove_var("ADL_PR_JANITOR_CMD");
        std::env::remove_var("ADL_PR_JANITOR_DISABLE");
        std::env::remove_var("ADL_POST_MERGE_CLOSEOUT_CMD");
        std::env::remove_var("ADL_POST_MERGE_CLOSEOUT_DISABLE");
    }
    restore_github_policy_env(policy_env);
}

#[test]
fn issue_version_prefers_consistent_label_title_and_body_evidence_and_fails_closed_on_conflict() {
    let _guard = env_lock();
    let policy_env = clear_github_policy_env();
    let temp = unique_temp_dir("adl-github-issue-version-evidence");
    let bin_dir = temp.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let title_file = temp.join("title.txt");
    let labels_file = temp.join("labels.txt");
    let body_file = temp.join("body.md");
    let github_cli_fixture = bin_dir.join("github-cli-fixture");

    write_executable(
        &github_cli_fixture,
        &format!(
            r#"#!/usr/bin/env python3
import pathlib
import sys

title = pathlib.Path({title:?})
labels = pathlib.Path({labels:?})
body = pathlib.Path({body:?})
args = sys.argv[1:]

if args[:2] == ["issue", "view"]:
    if "labels" in args:
        print(labels.read_text(encoding="utf-8"), end="")
        sys.exit(0)
    if "title" in args:
        print(title.read_text(encoding="utf-8"), end="")
        sys.exit(0)
    if "body" in args:
        print(body.read_text(encoding="utf-8"), end="")
        sys.exit(0)
sys.exit(9)
"#,
            title = title_file.display().to_string(),
            labels = labels_file.display().to_string(),
            body = body_file.display().to_string(),
        ),
    );

    let old_path = std::env::var("PATH").ok();
    let mut path_entries = vec![bin_dir.clone()];
    path_entries.extend(std::env::split_paths(old_path.as_deref().unwrap_or("")));
    unsafe {
        std::env::set_var("ADL_TEST_GITHUB_CLI_FIXTURE", &github_cli_fixture);
        std::env::set_var(
            "PATH",
            std::env::join_paths(path_entries).expect("join PATH"),
        );
    }

    fs::write(&labels_file, "track:roadmap\nversion:v0.91.6\n").expect("labels");
    fs::write(&title_file, "[v0.91.6][adr] Create and route v0.91.6 ADR candidates\n")
        .expect("title");
    fs::write(
        &body_file,
        "## Summary\nObserved repair failure: `pr init 4383` inferred `v0.91.7` for a `v0.91.6` issue.\n\nVersion: v0.91.6\n",
    )
    .expect("body");

    assert_eq!(
        issue_version(4383, "owner/repo").expect("consistent inferred version"),
        Some("v0.91.6".to_string())
    );

    fs::write(
        &body_file,
        "## Summary\nObserved repair failure: `pr init 4383` inferred `v0.91.7` for a `v0.91.6` issue.\n\nVersion: v0.91.7\n",
    )
    .expect("conflicting body");

    let err = issue_version(4383, "owner/repo").expect_err("conflicting metadata should fail");
    assert!(err
        .to_string()
        .contains("conflicting version evidence for issue #4383"));

    fs::write(
        &body_file,
        "## Summary\nObserved repair failure cites [v0.91.7][tools] but the issue remains in the v0.91.6 wave.\n",
    )
    .expect("non-authoritative body");
    assert_eq!(
        issue_version(4383, "owner/repo").expect("non-authoritative body should not conflict"),
        Some("v0.91.6".to_string())
    );

    restore_env("PATH", old_path);
    restore_github_policy_env(policy_env);
}

#[test]
fn issue_metadata_helpers_preserve_create_body_title_and_label_parity() {
    let _guard = env_lock();
    let policy_env = clear_github_policy_env();
    let temp = unique_temp_dir("adl-github-issue-metadata");
    let bin_dir = temp.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let title_file = temp.join("title.txt");
    let labels_file = temp.join("labels.txt");
    let body_file = temp.join("body.md");
    let log_file = temp.join("gh.log");
    fs::write(&title_file, "[v0.91.4][tools] Old title\n").expect("title");
    fs::write(&labels_file, "track:roadmap\nversion:v0.91.4\n").expect("labels");

    let github_cli_fixture = bin_dir.join("github-cli-fixture");
    write_executable(
        &github_cli_fixture,
        &format!(
            r#"#!/usr/bin/env python3
import pathlib
import shutil
import sys

title = pathlib.Path({title:?})
labels = pathlib.Path({labels:?})
body = pathlib.Path({body:?})
log = pathlib.Path({log:?})
args = sys.argv[1:]
with log.open("a", encoding="utf-8") as fh:
    fh.write(repr(args) + "\n")

if args[:2] == ["label", "list"]:
    print("track:roadmap\narea:tools\ntype:task\nversion:v0.91.5")
    sys.exit(0)

if args[:2] == ["issue", "create"]:
    print("https://github.com/owner/repo/issues/77")
    sys.exit(0)

if args[:2] == ["issue", "view"]:
    if "labels" in args:
        print(labels.read_text(encoding="utf-8"), end="")
        sys.exit(0)
    if "title" in args:
        print(title.read_text(encoding="utf-8"), end="")
        sys.exit(0)
    sys.exit(2)

if args[:2] == ["issue", "edit"]:
    current_labels = [
        line.strip()
        for line in labels.read_text(encoding="utf-8").splitlines()
        if line.strip()
    ]
    i = 2
    while i < len(args):
        if args[i] == "--title":
            title.write_text(args[i + 1] + "\n", encoding="utf-8")
            i += 2
        elif args[i] == "--add-label":
            requested_labels = [
                label.strip()
                for label in args[i + 1].split(",")
                if label.strip()
            ]
            if "," in args[i + 1]:
                current_labels = []
            for label in requested_labels:
                label = label.strip()
                if label and label not in current_labels:
                    current_labels.append(label)
            i += 2
        elif args[i] == "--remove-label":
            current_labels = [label for label in current_labels if label != args[i + 1]]
            i += 2
        elif args[i] == "--body":
            body.write_text(args[i + 1], encoding="utf-8")
            i += 2
        elif args[i] == "--body-file":
            shutil.copyfile(args[i + 1], body)
            i += 2
        else:
            i += 1
    labels.write_text("\n".join(current_labels) + "\n", encoding="utf-8")
    sys.exit(0)

sys.exit(9)
"#,
            title = title_file.display().to_string(),
            labels = labels_file.display().to_string(),
            body = body_file.display().to_string(),
            log = log_file.display().to_string(),
        ),
    );

    let old_path = std::env::var("PATH").ok();
    let mut path_entries = vec![bin_dir.clone()];
    path_entries.extend(std::env::split_paths(old_path.as_deref().unwrap_or("")));
    unsafe {
        std::env::set_var("ADL_TEST_GITHUB_CLI_FIXTURE", &github_cli_fixture);
        std::env::set_var(
            "PATH",
            std::env::join_paths(path_entries).expect("join PATH"),
        );
    }

    let created = gh_issue_create(
        "owner/repo",
        "[v0.91.5][tools] New title",
        "issue body",
        " version:v0.91.5, area:tools,,type:task ",
    )
    .expect("create issue");
    assert_eq!(created, "https://github.com/owner/repo/issues/77");

    gh_issue_edit_body("owner/repo", 77, "updated body").expect("edit body");
    assert_eq!(
        fs::read_to_string(&body_file).expect("body file"),
        "updated body"
    );

    ensure_issue_metadata_parity(
        "owner/repo",
        77,
        "[v0.91.5][tools] New title",
        "track:roadmap,area:tools,version:v0.91.5",
    )
    .expect("metadata parity");

    assert_eq!(
        fs::read_to_string(&title_file).expect("title"),
        "[v0.91.5][tools] New title\n"
    );
    assert_eq!(
        fs::read_to_string(&labels_file).expect("labels"),
        "area:tools\ntrack:roadmap\nversion:v0.91.5\n"
    );

    restore_env("PATH", old_path);

    let calls = fs::read_to_string(&log_file).expect("gh log");
    assert!(calls.contains("'--label', ' version:v0.91.5, area:tools,,type:task '"));
    assert!(calls.contains("'--title', '[v0.91.5][tools] New title'"));
    assert!(calls.contains("'--add-label', 'area:tools,track:roadmap,version:v0.91.5'"));
    restore_github_policy_env(policy_env);
}
