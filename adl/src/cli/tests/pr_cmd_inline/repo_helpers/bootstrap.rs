use super::*;
use crate::cli::pr_cmd::github::current_pr_url;
use adl::session_ledger::{
    default_ledger_path, save_ledger, ClaimInput, ClaimMode, GithubRef, ResourceRef, SessionLedger,
    DEFAULT_TTL_SECS,
};

fn write_session_claim(
    repo: &std::path::Path,
    issue: u64,
    session_id: &str,
    branch: &str,
    worktree_path: &std::path::Path,
) {
    let mut ledger = SessionLedger::empty(chrono::Utc::now());
    ledger
        .claim(
            ClaimInput {
                session_id: session_id.to_string(),
                owner: "codex".to_string(),
                resource: ResourceRef {
                    kind: "csdlc_issue".to_string(),
                    id: issue.to_string(),
                },
                purpose: "test ownership".to_string(),
                mode: ClaimMode::Active,
                lifecycle_phase: Some("pr_run".to_string()),
                policy_ref: Some("AGENTS.md".to_string()),
                github: GithubRef {
                    issue: Some(issue),
                    pull_request: None,
                    repository: Some("owner/repo".to_string()),
                    last_state: Some("open".to_string()),
                },
                branch: Some(branch.to_string()),
                worktree_path: Some(
                    worktree_path
                        .strip_prefix(repo)
                        .unwrap_or(worktree_path)
                        .display()
                        .to_string(),
                ),
                do_not_touch_paths: Vec::new(),
                blockers: Vec::new(),
                ttl_secs: DEFAULT_TTL_SECS,
            },
            chrono::Utc::now(),
        )
        .expect("claim");
    save_ledger(&default_ledger_path(repo), &ledger).expect("save ledger");
}

#[test]
fn real_pr_start_requires_explicit_version_when_no_fetch_issue_has_no_local_bundle() {
    let _guard = env_lock();
    let repo = unique_temp_dir("adl-pr-start-no-fetch-missing-version");
    copy_bootstrap_support_files(&repo);
    init_git_repo(&repo);
    assert!(Command::new("git")
        .args([
            "remote",
            "set-url",
            "origin",
            "https://github.com/owner/repo.git",
        ])
        .current_dir(&repo)
        .status()
        .expect("git remote set-url")
        .success());

    let old_path = env::var("PATH").unwrap_or_default();
    let prev_dir = env::current_dir().expect("cwd");
    env::set_current_dir(&repo).expect("chdir");

    let err = real_pr(&[
        "start".to_string(),
        "1153".to_string(),
        "--slug".to_string(),
        "runtime-missing-version-no-fetch".to_string(),
        "--no-fetch-issue".to_string(),
    ])
    .expect_err("no-fetch start without local bundle or explicit version should fail");

    env::set_current_dir(prev_dir).expect("restore cwd");
    unsafe {
        env::set_var("PATH", old_path);
    }

    assert!(err.to_string().contains(
        "start: --version is required when --no-fetch-issue is set and no canonical local bundle exists to infer the milestone band"
    ));
}

#[test]
fn real_pr_start_blocks_before_worktree_when_design_time_cards_are_not_ready() {
    let _guard = env_lock();
    let repo = unique_temp_dir("adl-pr-start-design-time-card-gate");
    copy_bootstrap_support_files(&repo);
    init_git_repo(&repo);
    assert!(Command::new("git")
        .args(["config", "user.name", "Test User"])
        .current_dir(&repo)
        .status()
        .expect("git config")
        .success());
    assert!(Command::new("git")
        .args(["config", "user.email", "test@example.com"])
        .current_dir(&repo)
        .status()
        .expect("git config")
        .success());
    fs::write(repo.join("README.md"), "design-time gate fixture\n").expect("write readme");
    assert!(Command::new("git")
        .args(["add", "-A"])
        .current_dir(&repo)
        .status()
        .expect("git add")
        .success());
    assert!(Command::new("git")
        .args(["commit", "-q", "-m", "init"])
        .current_dir(&repo)
        .status()
        .expect("git commit")
        .success());
    assert!(Command::new("git")
        .args(["branch", "-M", "main"])
        .current_dir(&repo)
        .status()
        .expect("git branch")
        .success());
    let issue_ref = IssueRef::new(
        1154,
        "v0.86".to_string(),
        "v0-86-tools-design-time-card-gate".to_string(),
    )
    .expect("issue ref");
    write_authored_issue_prompt(&repo, &issue_ref, "[v0.86][tools] Design-time card gate");
    let root_spp_path = issue_ref.task_bundle_plan_path(&repo);
    fs::create_dir_all(root_spp_path.parent().expect("root spp parent"))
        .expect("create root spp parent");
    write_authored_spp(
        &root_spp_path,
        &issue_ref,
        "[v0.86][tools] Design-time card gate",
        "not bound yet",
        &repo,
    );
    let root_spp = fs::read_to_string(&root_spp_path).expect("read root spp");
    fs::write(
        &root_spp_path,
        root_spp
            .replace("status: \"reviewed\"", "status: \"draft\"")
            .replace(
                "activation_state: \"reviewed\"",
                "activation_state: \"draft\"",
            ),
    )
    .expect("write non-ready root spp");
    assert!(Command::new("git")
        .args([
            "remote",
            "set-url",
            "origin",
            "https://github.com/owner/repo.git",
        ])
        .current_dir(&repo)
        .status()
        .expect("git remote set-url")
        .success());
    write_session_claim(
        &repo,
        1154,
        "thread-self",
        "codex/1154-v0-86-tools-design-time-card-gate",
        &issue_ref.default_worktree_path(&repo, None),
    );

    let prev_dir = env::current_dir().expect("cwd");
    let old_session = env::var_os("CODEX_SESSION_ID");
    unsafe {
        env::set_var("CODEX_SESSION_ID", "thread-self");
    }
    env::set_current_dir(&repo).expect("chdir");

    let err = real_pr(&[
        "start".to_string(),
        "1154".to_string(),
        "--slug".to_string(),
        "v0-86-tools-design-time-card-gate".to_string(),
        "--title".to_string(),
        "[v0.86][tools] Design-time card gate".to_string(),
        "--no-fetch-issue".to_string(),
        "--version".to_string(),
        "v0.86".to_string(),
    ])
    .expect_err("start should block before worktree binding when root SPP is not ready");

    env::set_current_dir(prev_dir).expect("restore cwd");
    match old_session {
        Some(value) => unsafe { env::set_var("CODEX_SESSION_ID", value) },
        None => unsafe { env::remove_var("CODEX_SESSION_ID") },
    }

    let err_text = err.to_string();
    assert!(
        err_text.contains("design-time card completion gate failed"),
        "unexpected error: {err_text}"
    );
    assert!(err_text.contains("SPP"));
    assert!(
        !repo.join(".worktrees/adl-wp-1154").exists(),
        "design-time card gate should block before worktree creation"
    );
    let branch = "codex/1154-v0-86-tools-design-time-card-gate";
    let root_sip = fs::read_to_string(issue_ref.task_bundle_input_path(&repo)).expect("read sip");
    let root_sor = fs::read_to_string(issue_ref.task_bundle_output_path(&repo)).expect("read sor");
    let root_spp = fs::read_to_string(issue_ref.task_bundle_plan_path(&repo)).expect("read spp");
    let root_srp =
        fs::read_to_string(issue_ref.task_bundle_review_policy_path(&repo)).expect("read srp");
    assert!(root_sip.contains("Branch: not bound yet"));
    assert!(root_sor.contains("Branch: not bound yet"));
    assert!(root_sor.contains("Status: NOT_STARTED"));
    assert!(!root_sor.contains("Status: IN_PROGRESS"));
    assert!(!root_sip.contains(&format!("Branch: {branch}")));
    assert!(!root_sor.contains(&format!("Branch: {branch}")));
    assert!(!root_spp.contains(&format!("branch: \"{branch}\"")));
    assert!(!root_srp.contains(&format!("branch: \"{branch}\"")));
}

#[test]
fn real_pr_init_requires_explicit_or_inferable_version_for_issue() {
    let _guard = env_lock();
    let repo = unique_temp_dir("adl-pr-init-missing-version");
    copy_bootstrap_support_files(&repo);
    init_git_repo(&repo);
    assert!(Command::new("git")
        .args([
            "remote",
            "set-url",
            "origin",
            "https://github.com/owner/repo.git",
        ])
        .current_dir(&repo)
        .status()
        .expect("git remote set-url")
        .success());

    let bin_dir = repo.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    write_executable(
        &bin_dir.join("gh"),
        "#!/usr/bin/env bash\nset -euo pipefail\nif [[ \"$*\" == *\"issue view 1153 -R owner/repo --json title --jq .title\"* ]]; then\n  printf '[runtime] Missing version metadata\\n'\n  exit 0\nfi\nif [[ \"$*\" == *\"issue view 1153 -R owner/repo --json labels --jq .labels[].name\"* ]]; then\n  printf 'track:roadmap\\ntype:task\\narea:runtime\\n'\n  exit 0\nfi\nexit 1\n",
    );

    let old_path = env::var("PATH").unwrap_or_default();
    let prev_dir = env::current_dir().expect("cwd");
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
    }
    env::set_current_dir(&repo).expect("chdir");

    let err = real_pr(&[
        "init".to_string(),
        "1153".to_string(),
        "--slug".to_string(),
        "runtime-missing-version-metadata".to_string(),
    ])
    .expect_err("missing version metadata should fail init");

    env::set_current_dir(prev_dir).expect("restore cwd");
    unsafe {
        env::set_var("PATH", old_path);
    }

    assert!(err
        .to_string()
        .contains("init: could not infer version for issue #1153"));
}

#[test]
fn real_pr_init_fails_closed_on_conflicting_issue_version_metadata_before_remote_mutation() {
    let _guard = env_lock();
    let repo = unique_temp_dir("adl-pr-init-conflicting-version-metadata");
    copy_bootstrap_support_files(&repo);
    init_git_repo(&repo);
    assert!(Command::new("git")
        .args([
            "remote",
            "set-url",
            "origin",
            "https://github.com/owner/repo.git",
        ])
        .current_dir(&repo)
        .status()
        .expect("git remote set-url")
        .success());

    let bin_dir = repo.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let gh_log = repo.join("gh.log");
    write_executable(
        &bin_dir.join("gh"),
        &format!(
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\nif [[ \"$*\" == *\"issue view 1153 -R owner/repo --json title --jq .title\"* ]]; then\n  printf '[v0.91.6][runtime] Metadata conflict gate\\n'\n  exit 0\nfi\nif [[ \"$*\" == *\"issue view 1153 -R owner/repo --json labels --jq .labels[].name\"* ]]; then\n  printf 'track:roadmap\\ntype:task\\narea:runtime\\nversion:v0.91.6\\n'\n  exit 0\nfi\nif [[ \"$*\" == *\"issue view 1153 -R owner/repo --json body --jq .body\"* ]]; then\n  cat <<'EOF'\n## Summary\n\nConflicting issue metadata regression fixture.\n\nVersion: v0.91.7\nEOF\n  exit 0\nfi\nif [[ \"$*\" == *\"issue edit 1153 -R owner/repo\"* || \"$*\" == *\"issue create -R owner/repo\"* ]]; then\n  exit 97\nfi\nexit 1\n",
            gh_log.display(),
        ),
    );

    let old_path = env::var("PATH").unwrap_or_default();
    let prev_dir = env::current_dir().expect("cwd");
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
    }
    env::set_current_dir(&repo).expect("chdir");

    let err = real_pr(&[
        "init".to_string(),
        "1153".to_string(),
        "--slug".to_string(),
        "runtime-conflicting-version-metadata".to_string(),
    ])
    .expect_err("conflicting version metadata should fail init before mutation");

    env::set_current_dir(prev_dir).expect("restore cwd");
    unsafe {
        env::set_var("PATH", old_path);
    }

    let err_text = err.to_string();
    assert!(err_text.contains("conflicting version evidence for issue #1153"));
    let gh_log = fs::read_to_string(&gh_log).expect("gh log");
    assert!(gh_log.contains("issue view 1153 -R owner/repo --json title --jq .title"));
    assert!(gh_log.contains("issue view 1153 -R owner/repo --json labels --jq .labels[].name"));
    assert!(gh_log.contains("issue view 1153 -R owner/repo --json body --jq .body"));
    assert!(!gh_log.contains("issue edit 1153 -R owner/repo"));
    assert!(!gh_log.contains("issue create -R owner/repo"));
}

#[test]
fn current_pr_url_filters_empty_and_null_results() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-current-url");
    let bin_dir = temp.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    write_executable(
            &bin_dir.join("gh"),
            "#!/usr/bin/env bash\nset -euo pipefail\ncase \"${GH_PR_LIST_MODE:-url}\" in\n  null) printf 'null\\n' ;;\n  empty) printf '\\n' ;;\n  *) printf 'https://github.com/example/repo/pull/1\\n' ;;\nesac\n",
        );

    let old_path = env::var("PATH").unwrap_or_default();
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
        env::set_var("GH_PR_LIST_MODE", "url");
    }
    assert_eq!(
        current_pr_url("owner/repo", "codex/test").expect("url"),
        Some("https://github.com/example/repo/pull/1".to_string())
    );
    unsafe {
        env::set_var("GH_PR_LIST_MODE", "null");
    }
    assert_eq!(
        current_pr_url("owner/repo", "codex/test").expect("null"),
        None
    );
    unsafe {
        env::set_var("GH_PR_LIST_MODE", "empty");
    }
    assert_eq!(
        current_pr_url("owner/repo", "codex/test").expect("empty"),
        None
    );
    unsafe {
        env::set_var("PATH", old_path);
        env::remove_var("GH_PR_LIST_MODE");
    }
}

#[test]
fn branch_checked_out_worktree_path_returns_none_without_match() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-worktree-none");
    let bin_dir = temp.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let repo = temp.join("repo");
    fs::create_dir_all(repo.join(".git")).expect("repo git dir");
    write_executable(
            &bin_dir.join("git"),
            &format!(
                "#!/usr/bin/env bash\nset -euo pipefail\nif [ \"$1 $2\" = 'rev-parse --show-toplevel' ]; then\n  printf '%s\\n' '{0}'\n  exit 0\nfi\nif [ \"$1 $2\" = 'rev-parse --git-common-dir' ]; then\n  printf '%s\\n' '{1}'\n  exit 0\nfi\nif [ \"$1 $2 $3\" = 'worktree list --porcelain' ]; then\n  cat <<'EOF'\nworktree {0}\nHEAD deadbeef\nbranch refs/heads/main\nEOF\n  exit 0\nfi\nexit 1\n",
                repo.display(),
                repo.join(".git").display(),
            ),
        );

    let old_path = env::var("PATH").unwrap_or_default();
    let old_pwd = env::current_dir().expect("cwd");
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
    }
    env::set_current_dir(&repo).expect("chdir");
    assert_eq!(
        branch_checked_out_worktree_path("codex/missing").expect("none"),
        None
    );
    env::set_current_dir(old_pwd).expect("restore cwd");
    unsafe {
        env::set_var("PATH", old_path);
    }
}

#[test]
fn branch_checked_out_worktree_path_ignores_unrelated_noncanonical_worktrees() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-worktree-ignore-external");
    let bin_dir = temp.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let repo = temp.join("repo");
    fs::create_dir_all(repo.join(".git")).expect("repo git dir");
    write_executable(
            &bin_dir.join("git"),
            &format!(
                "#!/usr/bin/env bash\nset -euo pipefail\nif [ \"$1 $2\" = 'rev-parse --show-toplevel' ]; then\n  printf '%s\\n' '{0}'\n  exit 0\nfi\nif [ \"$1 $2\" = 'rev-parse --git-common-dir' ]; then\n  printf '%s\\n' '{1}'\n  exit 0\nfi\nif [ \"$1 $2 $3\" = 'worktree list --porcelain' ]; then\n  cat <<'EOF'\nworktree /Users/daniel/.codex/worktrees/abcd/agent-design-language\nHEAD deadbeef\nbranch refs/heads/codex/external\n\nworktree {0}/.worktrees/adl-wp-1153\nHEAD cafefood\nbranch refs/heads/codex/1153-test\nEOF\n  exit 0\nfi\nexit 1\n",
                repo.display(),
                repo.join(".git").display(),
            ),
        );

    let old_path = env::var("PATH").unwrap_or_default();
    let old_pwd = env::current_dir().expect("cwd");
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
    }
    env::set_current_dir(&repo).expect("chdir");
    let resolved =
        branch_checked_out_worktree_path("codex/1153-test").expect("canonical worktree resolved");
    env::set_current_dir(old_pwd).expect("restore cwd");
    unsafe {
        env::set_var("PATH", old_path);
    }
    assert_eq!(resolved, Some(repo.join(".worktrees").join("adl-wp-1153")));
}

#[test]
fn branch_checked_out_worktree_path_rejects_noncanonical_matching_branch() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-worktree-reject-external");
    let bin_dir = temp.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let repo = temp.join("repo");
    fs::create_dir_all(repo.join(".git")).expect("repo git dir");
    write_executable(
            &bin_dir.join("git"),
            &format!(
                "#!/usr/bin/env bash\nset -euo pipefail\nif [ \"$1 $2\" = 'rev-parse --show-toplevel' ]; then\n  printf '%s\\n' '{0}'\n  exit 0\nfi\nif [ \"$1 $2\" = 'rev-parse --git-common-dir' ]; then\n  printf '%s\\n' '{1}'\n  exit 0\nfi\nif [ \"$1 $2 $3\" = 'worktree list --porcelain' ]; then\n  cat <<'EOF'\nworktree /Users/daniel/.codex/worktrees/abcd/agent-design-language\nHEAD deadbeef\nbranch refs/heads/codex/1153-test\nEOF\n  exit 0\nfi\nexit 1\n",
                repo.display(),
                repo.join(".git").display()
            ),
        );

    let old_path = env::var("PATH").unwrap_or_default();
    let old_pwd = env::current_dir().expect("cwd");
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
    }
    env::set_current_dir(&repo).expect("chdir");
    let err = branch_checked_out_worktree_path("codex/1153-test")
        .expect_err("non-canonical matching branch should fail");
    env::set_current_dir(old_pwd).expect("restore cwd");
    unsafe {
        env::set_var("PATH", old_path);
    }
    assert!(err.to_string().contains("non-canonical worktree"));
    assert!(err
        .to_string()
        .contains("/Users/daniel/.codex/worktrees/abcd/agent-design-language"));
}

#[test]
fn branch_checked_out_worktree_path_accepts_explicit_managed_worktree_root() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-worktree-managed-root");
    let bin_dir = temp.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let repo = temp.join("repo");
    let managed_root = temp.join("managed");
    fs::create_dir_all(repo.join(".git")).expect("repo git dir");
    fs::create_dir_all(&managed_root).expect("managed root");
    write_executable(
            &bin_dir.join("git"),
            &format!(
                "#!/usr/bin/env bash\nset -euo pipefail\nif [ \"$1 $2\" = 'rev-parse --show-toplevel' ]; then\n  printf '%s\\n' '{0}'\n  exit 0\nfi\nif [ \"$1 $2\" = 'rev-parse --git-common-dir' ]; then\n  printf '%s\\n' '{1}'\n  exit 0\nfi\nif [ \"$1 $2 $3\" = 'worktree list --porcelain' ]; then\n  cat <<'EOF'\nworktree {2}/adl-wp-1153\nHEAD deadbeef\nbranch refs/heads/codex/1153-test\nEOF\n  exit 0\nfi\nexit 1\n",
                repo.display(),
                repo.join(".git").display(),
                managed_root.display()
            ),
        );

    let old_path = env::var("PATH").unwrap_or_default();
    let old_pwd = env::current_dir().expect("cwd");
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
        env::set_var("ADL_WORKTREE_ROOT", &managed_root);
    }
    env::set_current_dir(&repo).expect("chdir");
    let resolved =
        branch_checked_out_worktree_path("codex/1153-test").expect("managed worktree resolved");
    env::set_current_dir(old_pwd).expect("restore cwd");
    unsafe {
        env::set_var("PATH", old_path);
        env::remove_var("ADL_WORKTREE_ROOT");
    }
    assert_eq!(resolved, Some(managed_root.join("adl-wp-1153")));
}

#[test]
fn ensure_worktree_for_branch_reuses_matching_path_and_creates_new_one() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-worktree-reuse-create");
    let bin_dir = temp.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let git_log = temp.join("git.log");
    let repo = temp.join("repo");
    fs::create_dir_all(repo.join(".git")).expect("repo git dir");
    write_executable(
            &bin_dir.join("git"),
            &format!(
                "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{0}'\nif [ \"$1 $2\" = 'rev-parse --show-toplevel' ]; then\n  printf '%s\\n' '{1}'\n  exit 0\nfi\nif [ \"$1 $2\" = 'rev-parse --git-common-dir' ]; then\n  printf '%s\\n' '{2}'\n  exit 0\nfi\nif [ \"$1 $2 $3\" = 'worktree list --porcelain' ]; then\n  if [ \"${{WT_MODE:-reuse}}\" = 'reuse' ]; then\n    cat <<'EOF'\nworktree {1}/.worktrees/reuse-me\nHEAD deadbeef\nbranch refs/heads/codex/reuse\nEOF\n    exit 0\n  fi\n  printf 'worktree {1}\\nHEAD deadbeef\\nbranch refs/heads/main\\n'\n  exit 0\nfi\nif [ \"$1 $2 $3\" = '-C {1}/.worktrees/reuse-me status' ]; then\n  exit 0\nfi\nif [ \"$1 $2 $3\" = 'worktree add {1}/.worktrees/create-me' ]; then\n  mkdir -p {1}/.worktrees/create-me\n  exit 0\nfi\nexit 1\n",
                git_log.display(),
                repo.display(),
                repo.join(".git").display()
            ),
        );

    let old_path = env::var("PATH").unwrap_or_default();
    let old_pwd = env::current_dir().expect("cwd");
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
        env::set_var("WT_MODE", "reuse");
    }
    env::set_current_dir(&repo).expect("chdir");
    ensure_worktree_for_branch(&repo.join(".worktrees").join("reuse-me"), "codex/reuse")
        .expect("reuse");

    unsafe {
        env::set_var("WT_MODE", "create");
    }
    let create_path = repo.join(".worktrees").join("create-me");
    let _ = fs::remove_dir_all(&create_path);
    ensure_worktree_for_branch(&create_path, "codex/create").expect("create");

    env::set_current_dir(old_pwd).expect("restore cwd");
    unsafe {
        env::set_var("PATH", old_path);
        env::remove_var("WT_MODE");
    }
    let log = fs::read_to_string(&git_log).expect("git log");
    assert!(log.contains(&format!(
        "worktree add {} codex/create",
        create_path.display()
    )));
}

#[test]
fn ensure_worktree_for_branch_rejects_dirty_matching_worktree() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-worktree-dirty-reuse");
    let bin_dir = temp.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let repo = temp.join("repo");
    fs::create_dir_all(repo.join(".git")).expect("repo git dir");
    fs::create_dir_all(repo.join(".worktrees/reuse-me")).expect("reuse worktree");
    write_executable(
        &bin_dir.join("git"),
        &format!(
            "#!/usr/bin/env bash\nset -euo pipefail\nif [ \"$1 $2\" = 'rev-parse --show-toplevel' ]; then\n  printf '%s\\n' '{0}'\n  exit 0\nfi\nif [ \"$1 $2\" = 'rev-parse --git-common-dir' ]; then\n  printf '%s\\n' '{1}'\n  exit 0\nfi\nif [ \"$1 $2 $3\" = 'worktree list --porcelain' ]; then\n  cat <<'EOF'\nworktree {0}/.worktrees/reuse-me\nHEAD deadbeef\nbranch refs/heads/codex/reuse\nEOF\n  exit 0\nfi\nif [ \"$1 $2 $3\" = '-C {0}/.worktrees/reuse-me status' ]; then\n  printf ' M README.md\\n?? scratch.txt\\n'\n  exit 0\nfi\nexit 1\n",
            repo.display(),
            repo.join(".git").display(),
        ),
    );

    let old_path = env::var("PATH").unwrap_or_default();
    let old_pwd = env::current_dir().expect("cwd");
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
    }
    env::set_current_dir(&repo).expect("chdir");
    let err = ensure_worktree_for_branch(&repo.join(".worktrees/reuse-me"), "codex/reuse")
        .expect_err("dirty matching worktree should fail closed");
    env::set_current_dir(old_pwd).expect("restore cwd");
    unsafe {
        env::set_var("PATH", old_path);
    }

    let err = err.to_string();
    assert!(err.contains("unsafe_existing_worktree_dirty"));
    assert!(err.contains("README.md"));
    assert!(err.contains("scratch.txt"));
    assert!(err.contains("do not reset or prune until the dirty state is accounted for"));
}

#[test]
fn ensure_worktree_for_branch_rejects_existing_path_without_matching_branch() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-worktree-stale-path");
    let bin_dir = temp.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let repo = temp.join("repo");
    fs::create_dir_all(repo.join(".git")).expect("repo git dir");
    fs::create_dir_all(repo.join(".worktrees/stale")).expect("stale worktree");
    write_executable(
        &bin_dir.join("git"),
        &format!(
            "#!/usr/bin/env bash\nset -euo pipefail\nif [ \"$1 $2\" = 'rev-parse --show-toplevel' ]; then\n  printf '%s\\n' '{0}'\n  exit 0\nfi\nif [ \"$1 $2\" = 'rev-parse --git-common-dir' ]; then\n  printf '%s\\n' '{1}'\n  exit 0\nfi\nif [ \"$1 $2 $3\" = 'worktree list --porcelain' ]; then\n  cat <<'EOF'\nworktree {0}\nHEAD deadbeef\nbranch refs/heads/main\nEOF\n  exit 0\nfi\nexit 1\n",
            repo.display(),
            repo.join(".git").display(),
        ),
    );

    let old_path = env::var("PATH").unwrap_or_default();
    let old_pwd = env::current_dir().expect("cwd");
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
    }
    env::set_current_dir(&repo).expect("chdir");
    let err = ensure_worktree_for_branch(&repo.join(".worktrees/stale"), "codex/current")
        .expect_err("existing stale path should fail closed");
    env::set_current_dir(old_pwd).expect("restore cwd");
    unsafe {
        env::set_var("PATH", old_path);
    }

    let err = err.to_string();
    assert!(err.contains("unsafe_existing_worktree_path"));
    assert!(err.contains(".worktrees/stale"));
    assert!(err.contains("codex/current"));
}

#[test]
fn validate_issue_prompt_exists_rejects_missing_file() {
    let missing = unique_temp_dir("adl-pr-missing-prompt").join("missing.md");
    let err = validate_issue_prompt_exists(&missing).expect_err("missing prompt");
    assert!(err
        .to_string()
        .contains("missing canonical source issue prompt"));
}

#[test]
fn resolve_issue_prompt_path_accepts_legacy_issue_bodies_location() {
    let repo = unique_temp_dir("adl-pr-legacy-prompt-path");
    let issue_ref = IssueRef::new(1197, "v0.86".to_string(), "legacy-ready-source".to_string())
        .expect("issue ref");
    let legacy = issue_ref.legacy_issue_prompt_path(&repo);
    fs::create_dir_all(legacy.parent().expect("legacy parent")).expect("legacy dir");
    fs::write(
        &legacy,
        "---\nissue_card_schema: adl.issue.v1\n---\n\n# x\n",
    )
    .expect("legacy");

    let resolved = resolve_issue_prompt_path(&repo, &issue_ref).expect("resolved");
    assert_eq!(resolved, legacy);
}

#[test]
fn resolve_issue_prompt_workflow_queue_rejects_missing_and_uninferrable_queue() {
    let repo = unique_temp_dir("adl-pr-missing-workflow-queue");
    let prompt = repo.join("issue.md");
    fs::write(
        &prompt,
        "---\nissue_card_schema: adl.issue.v1\nwp: \"unassigned\"\nslug: \"no-queue\"\ntitle: \"Plain issue title\"\nlabels:\n  - \"track:roadmap\"\n  - \"version:v0.88\"\nissue_number: 1\nstatus: \"draft\"\naction: \"edit\"\ndepends_on: []\nmilestone_sprint: \"Pending\"\nrequired_outcome_type:\n  - \"code\"\nrepo_inputs: []\ncanonical_files: []\ndemo_required: false\ndemo_names: []\nissue_graph_notes: []\npr_start:\n  enabled: false\n  slug: \"no-queue\"\n---\n\n# Plain issue title\n\n## Summary\nx\n## Goal\nx\n## Required Outcome\nx\n## Deliverables\nx\n## Acceptance Criteria\nx\n## Repo Inputs\nx\n## Dependencies\nx\n## Demo Expectations\nx\n## Non-goals\nx\n## Issue-Graph Notes\nx\n## Notes\nx\n## Tooling Notes\nx\n",
    )
    .expect("prompt");

    let err = resolve_issue_prompt_workflow_queue(&prompt).expect_err("missing queue should fail");
    assert!(err
        .to_string()
        .contains("missing or invalid workflow queue"));
}

#[test]
fn resolve_issue_prompt_workflow_queue_accepts_runtime_queue() {
    let repo = unique_temp_dir("adl-pr-runtime-workflow-queue");
    let prompt = repo.join("issue.md");
    fs::write(
        &prompt,
        "---\nissue_card_schema: adl.issue.v1\nwp: \"WP-05\"\nqueue: \"runtime\"\nslug: \"runtime-queue\"\ntitle: \"[v0.90.1][WP-05] Runtime v2 manifold contract\"\nlabels:\n  - \"track:roadmap\"\n  - \"area:runtime\"\n  - \"version:v0.90.1\"\nissue_number: 2145\nstatus: \"active\"\naction: \"edit\"\ndepends_on: []\nmilestone_sprint: \"v0.90.1\"\nrequired_outcome_type:\n  - \"code\"\nrepo_inputs: []\ncanonical_files: []\ndemo_required: false\ndemo_names: []\nissue_graph_notes: []\npr_start:\n  enabled: false\n  slug: \"runtime-queue\"\n---\n\n# Runtime queue\n\n## Summary\nx\n## Goal\nx\n## Required Outcome\nx\n## Deliverables\nx\n## Acceptance Criteria\nx\n## Repo Inputs\nx\n## Dependencies\nx\n## Demo Expectations\nx\n## Non-goals\nx\n## Issue-Graph Notes\nx\n## Notes\nx\n## Tooling Notes\nx\n",
    )
    .expect("prompt");

    let queue = resolve_issue_prompt_workflow_queue(&prompt).expect("runtime queue resolves");
    assert_eq!(queue.queue, "runtime");
    assert_eq!(queue.source, "explicit");
}

#[test]
fn real_pr_start_rejects_missing_slug_or_empty_sanitized_title_in_no_fetch_mode() {
    let _guard = env_lock();
    let repo = unique_temp_dir("adl-pr-start-preconditions");
    init_git_repo(&repo);
    let prev_dir = env::current_dir().expect("cwd");
    env::set_current_dir(&repo).expect("chdir");

    let missing_slug = real_pr(&[
        "start".to_string(),
        "1152".to_string(),
        "--no-fetch-issue".to_string(),
    ])
    .expect_err("missing slug should fail");
    assert!(missing_slug
        .to_string()
        .contains("start: --slug is required when --no-fetch-issue is set"));

    let bad_title = real_pr(&[
        "start".to_string(),
        "1152".to_string(),
        "--title".to_string(),
        "!!!".to_string(),
        "--no-fetch-issue".to_string(),
    ])
    .expect_err("empty sanitized title should fail");
    env::set_current_dir(prev_dir).expect("restore cwd");
    assert!(bad_title
        .to_string()
        .contains("start: --title produced empty slug after sanitization"));
}

#[test]
fn real_pr_ready_accepts_started_issue_when_output_branch_is_bootstrap_placeholder() {
    let _guard = env_lock();
    let repo = unique_temp_dir("adl-pr-ready-branch-placeholder");
    let origin = repo.join("origin.git");
    init_git_repo(&repo);
    copy_bootstrap_support_files(&repo);
    assert!(Command::new("git")
        .args(["config", "user.name", "Test User"])
        .current_dir(&repo)
        .status()
        .expect("git config")
        .success());
    assert!(Command::new("git")
        .args(["config", "user.email", "test@example.com"])
        .current_dir(&repo)
        .status()
        .expect("git config")
        .success());
    fs::write(repo.join("README.md"), "ready branch placeholder\n").expect("write readme");
    assert!(Command::new("git")
        .args(["add", "-A"])
        .current_dir(&repo)
        .status()
        .expect("git add")
        .success());
    assert!(Command::new("git")
        .args(["commit", "-q", "-m", "init"])
        .current_dir(&repo)
        .status()
        .expect("git commit")
        .success());
    assert!(Command::new("git")
        .args(["branch", "-M", "main"])
        .current_dir(&repo)
        .status()
        .expect("git branch")
        .success());
    assert!(Command::new("git")
        .args([
            "init",
            "--bare",
            "-q",
            path_str(&origin).expect("origin path"),
        ])
        .current_dir(&repo)
        .status()
        .expect("git init bare")
        .success());
    assert!(Command::new("git")
        .args([
            "remote",
            "set-url",
            "origin",
            path_str(&origin).expect("origin path"),
        ])
        .current_dir(&repo)
        .status()
        .expect("git remote set-url")
        .success());
    assert!(Command::new("git")
        .args(["push", "-q", "-u", "origin", "main"])
        .current_dir(&repo)
        .status()
        .expect("git push")
        .success());
    assert!(Command::new("git")
        .args(["fetch", "-q", "origin", "main"])
        .current_dir(&repo)
        .status()
        .expect("git fetch")
        .success());

    let prev_dir = env::current_dir().expect("cwd");
    env::set_current_dir(&repo).expect("chdir");
    let issue_ref = IssueRef::new(1198, "v0.86", "ready-branch-placeholder").expect("issue ref");
    write_authored_issue_prompt(&repo, &issue_ref, "[v0.86][tools] Ready branch placeholder");
    write_design_time_ready_cards(
        &repo,
        &issue_ref,
        "[v0.86][tools] Ready branch placeholder",
        "codex/1198-ready-branch-placeholder",
    );

    real_pr(&[
        "start".to_string(),
        "1198".to_string(),
        "--slug".to_string(),
        "ready-branch-placeholder".to_string(),
        "--title".to_string(),
        "[v0.86][tools] Ready branch placeholder".to_string(),
        "--no-fetch-issue".to_string(),
        "--version".to_string(),
        "v0.86".to_string(),
    ])
    .expect("real_pr start");

    let root_output = issue_ref.task_bundle_output_path(&repo);
    let worktree = issue_ref.default_worktree_path(&repo, None);
    let root_sip = issue_ref.task_bundle_input_path(&repo);
    let wt_sip = issue_ref.task_bundle_input_path(&worktree);
    write_authored_sip(
        &root_sip,
        &issue_ref,
        "[v0.86][tools] Ready branch placeholder",
        "codex/1198-ready-branch-placeholder",
        &issue_ref.issue_prompt_path(&repo),
        &repo,
    );
    write_authored_sip(
        &wt_sip,
        &issue_ref,
        "[v0.86][tools] Ready branch placeholder",
        "codex/1198-ready-branch-placeholder",
        &issue_ref.issue_prompt_path(&worktree),
        &worktree,
    );
    let wt_output = issue_ref.task_bundle_output_path(&worktree);
    for path in [&root_output, &wt_output] {
        let text = fs::read_to_string(path).expect("sor");
        fs::write(
            path,
            text.replace(
                "Branch: codex/1198-ready-branch-placeholder",
                "Branch: TBD (run pr.sh start 1198)",
            ),
        )
        .expect("rewrite sor");
    }

    let ready = real_pr(&[
        "ready".to_string(),
        "1198".to_string(),
        "--slug".to_string(),
        "ready-branch-placeholder".to_string(),
        "--no-fetch-issue".to_string(),
        "--version".to_string(),
        "v0.86".to_string(),
    ]);

    env::set_current_dir(prev_dir).expect("restore cwd");
    ready.expect("ready should accept placeholder output branch");
}

#[test]
fn bootstrap_stub_reason_detects_issue_prompt_and_sip_templates() {
    let issue_prompt = "# x\n\n## Summary\n\nBootstrap-generated local source prompt for issue #1.\n\n## Goal\n\nTranslate the GitHub issue into the canonical local STP/task-bundle flow and refine this prompt before execution as needed.\n\n## Acceptance Criteria\n\n- something\n";
    assert_eq!(
        bootstrap_stub_reason(issue_prompt, PromptSurfaceKind::IssuePrompt),
        Some("bootstrap-generated issue prompt template text")
    );

    let workflow_skill_prompt = "# x\n\n## Summary\n\nBootstrap-generated workflow-skill issue body created from the requested title and labels so the issue starts with a concrete first draft instead of a generic bootstrap stub.\n\n## Goal\n\nDefine one bounded workflow-skill or tooling-surface change in the tracked PR workflow substrate and make the resulting source prompt/STP concrete enough for qualitative review before execution.\n\n## Acceptance Criteria\n\n- the generated prompt identifies this as a workflow-skill/tooling issue rather than a generic bootstrap task\n";
    assert_eq!(
        bootstrap_stub_reason(workflow_skill_prompt, PromptSurfaceKind::IssuePrompt),
        Some("bootstrap-generated issue prompt template text")
    );

    let sip = "# ADL Input Card\n\n## Goal\n\nReal goal\n\n## Acceptance Criteria\n\n- one\n\n## Required Outcome\n\n- State whether this issue must ship code, docs, tests, demo artifacts, or a combination.\n";
    assert_eq!(
        bootstrap_stub_reason(sip, PromptSurfaceKind::Sip),
        Some("unrefined SIP template guidance")
    );
}

#[cfg(unix)]
#[test]
fn ensure_git_metadata_writable_rejects_unwritable_git_dir() {
    use std::os::unix::fs::PermissionsExt;

    let _guard = env_lock();
    let repo = unique_temp_dir("adl-pr-git-metadata-write");
    init_git_repo(&repo);
    let prev_dir = env::current_dir().expect("cwd");
    env::set_current_dir(&repo).expect("chdir");

    let git_dir = repo.join(".git");
    let refs_dir = git_dir.join("refs");
    let heads_dir = refs_dir.join("heads");
    let git_mode = fs::metadata(&git_dir)
        .expect("git metadata")
        .permissions()
        .mode();
    let refs_mode = fs::metadata(&refs_dir)
        .expect("refs metadata")
        .permissions()
        .mode();
    let heads_mode = fs::metadata(&heads_dir)
        .expect("heads metadata")
        .permissions()
        .mode();

    fs::set_permissions(&git_dir, fs::Permissions::from_mode(0o555)).expect("chmod git");
    fs::set_permissions(&refs_dir, fs::Permissions::from_mode(0o555)).expect("chmod refs");
    fs::set_permissions(&heads_dir, fs::Permissions::from_mode(0o555)).expect("chmod heads");

    let err = ensure_git_metadata_writable().expect_err("unwritable git dir should fail");

    fs::set_permissions(&heads_dir, fs::Permissions::from_mode(heads_mode)).expect("restore heads");
    fs::set_permissions(&refs_dir, fs::Permissions::from_mode(refs_mode)).expect("restore refs");
    fs::set_permissions(&git_dir, fs::Permissions::from_mode(git_mode)).expect("restore git");
    env::set_current_dir(prev_dir).expect("restore cwd");

    assert!(err.to_string().contains("git metadata directory"));
    assert!(err
        .to_string()
        .contains("restore write access to git metadata before rerunning"));
}

#[test]
fn ensure_bootstrap_cards_creates_bundle_and_compat_links() {
    let _guard = env_lock();
    let repo = unique_temp_dir("adl-pr-bootstrap-cards");
    init_git_repo(&repo);
    copy_bootstrap_support_files(&repo);

    let issue_ref =
        IssueRef::new(1153, "v0.86".to_string(), "rust-finish-test".to_string()).expect("ref");
    let source_path = issue_ref.issue_prompt_path(&repo);
    fs::create_dir_all(source_path.parent().expect("source parent")).expect("mkdir");
    fs::write(
            &source_path,
            "---\ntitle: \"[v0.86][tools] Bootstrap cards\"\nlabels:\n  - \"track:roadmap\"\nissue_number: 1153\n---\n\n# Body\n",
        )
        .expect("write source");
    let stp_path = issue_ref.task_bundle_stp_path(&repo);
    fs::create_dir_all(stp_path.parent().expect("stp parent")).expect("mkdir");
    fs::write(
        &stp_path,
        "---\nissue_card_schema: adl.issue.v1\nwp: \"tools\"\nslug: \"rust-finish-test\"\ntitle: \"[v0.86][tools] Bootstrap cards\"\nlabels:\n  - \"track:roadmap\"\n  - \"version:v0.86\"\nissue_number: 1153\nstatus: \"draft\"\naction: \"edit\"\ndepends_on: []\nmilestone_sprint: \"Sprint Test\"\nrequired_outcome_type:\n  - \"code\"\nrepo_inputs: []\ncanonical_files: []\ndemo_required: false\ndemo_names: []\nissue_graph_notes: []\npr_start:\n  enabled: true\n  slug: \"rust-finish-test\"\n---\n\n# Bootstrap cards\n\n## Summary\nx\n## Goal\nx\n## Required Outcome\nx\n## Deliverables\nx\n## Acceptance Criteria\nx\n## Repo Inputs\nx\n## Dependencies\nx\n## Demo Expectations\nx\n## Non-goals\nx\n## Issue-Graph Notes\nx\n## Notes\nx\n## Tooling Notes\nx\n",
    )
    .expect("write stp");

    let (bundle_stp, bundle_input, bundle_output) = ensure_bootstrap_cards(
        &repo,
        &issue_ref,
        "[v0.86][tools] Bootstrap cards",
        "codex/1153-rust-finish-test",
        &source_path,
    )
    .expect("bootstrap cards");

    assert!(bundle_stp.is_file());
    assert!(bundle_input.is_file());
    assert!(bundle_output.is_file());
    let cards_root = resolve_cards_root(&repo, None);
    let compat_stp = card_stp_path(&cards_root, 1153);
    let compat_input = card_input_path(&cards_root, 1153);
    let compat_output = card_output_path(&cards_root, 1153);
    let compat_plan = card_plan_path(&cards_root, 1153);
    let compat_review_policy = card_review_policy_path(&cards_root, 1153);
    assert!(compat_stp.symlink_metadata().is_ok());
    assert!(compat_input.symlink_metadata().is_ok());
    assert!(compat_output.symlink_metadata().is_ok());
    assert!(issue_ref.task_bundle_plan_path(&repo).is_file());
    assert!(issue_ref.task_bundle_review_policy_path(&repo).is_file());
    assert!(compat_plan.symlink_metadata().is_ok());
    assert!(compat_review_policy.symlink_metadata().is_ok());
    let review_prompt_text =
        fs::read_to_string(issue_ref.task_bundle_review_policy_path(&repo)).expect("review prompt");
    assert!(review_prompt_text.contains("artifact_type: \"structured_review_prompt\""));
    assert!(review_prompt_text.contains("# Structured Review Prompt"));
    assert!(!review_prompt_text.contains("review_results_exception:"));
    assert!(review_prompt_text.contains("Review results are intentionally absent"));
    assert!(!review_prompt_text.contains("# Structured Review Policy"));
    assert_eq!(
        field_line_value(&bundle_input, "Branch").expect("input branch"),
        "codex/1153-rust-finish-test"
    );
    assert_eq!(
        field_line_value(&bundle_output, "Status").expect("output status"),
        "IN_PROGRESS"
    );
    let bundle_input_text = fs::read_to_string(&bundle_input).expect("bundle input");
    assert_eq!(
        bootstrap_stub_reason(&bundle_input_text, PromptSurfaceKind::Sip),
        None
    );
}

#[test]
fn ensure_bootstrap_cards_rewrites_existing_bootstrap_stub_input_card() {
    let _guard = env_lock();
    let repo = unique_temp_dir("adl-pr-bootstrap-cards-rewrite");
    init_git_repo(&repo);
    copy_bootstrap_support_files(&repo);

    let issue_ref = IssueRef::new(
        1154,
        "v0.86".to_string(),
        "rewrite-bootstrap-sip".to_string(),
    )
    .expect("ref");
    let source_path = issue_ref.issue_prompt_path(&repo);
    fs::create_dir_all(source_path.parent().expect("source parent")).expect("mkdir");
    fs::write(
            &source_path,
            "---\ntitle: \"[v0.86][tools] Rewrite bootstrap SIP\"\nlabels:\n  - \"track:roadmap\"\nissue_number: 1154\n---\n\n## Summary\nx\n## Goal\nx\n## Required Outcome\nx\n## Deliverables\nx\n## Acceptance Criteria\nx\n## Repo Inputs\nx\n## Dependencies\nx\n## Demo Expectations\nx\n## Non-goals\nx\n## Issue-Graph Notes\nx\n## Notes\nx\n## Tooling Notes\nx\n",
        )
        .expect("write source");
    let stp_path = issue_ref.task_bundle_stp_path(&repo);
    fs::create_dir_all(stp_path.parent().expect("stp parent")).expect("mkdir");
    fs::write(
        &stp_path,
        "---\nissue_card_schema: adl.issue.v1\nwp: \"tools\"\nslug: \"rewrite-bootstrap-sip\"\ntitle: \"[v0.86][tools] Rewrite bootstrap SIP\"\nlabels:\n  - \"track:roadmap\"\n  - \"version:v0.86\"\nissue_number: 1154\nstatus: \"draft\"\naction: \"edit\"\ndepends_on: []\nmilestone_sprint: \"Sprint Test\"\nrequired_outcome_type:\n  - \"code\"\nrepo_inputs: []\ncanonical_files: []\ndemo_required: false\ndemo_names: []\nissue_graph_notes: []\npr_start:\n  enabled: true\n  slug: \"rewrite-bootstrap-sip\"\n---\n\n# Rewrite bootstrap SIP\n\n## Summary\nx\n## Goal\nx\n## Required Outcome\nx\n## Deliverables\nx\n## Acceptance Criteria\nx\n## Repo Inputs\nx\n## Dependencies\nx\n## Demo Expectations\nx\n## Non-goals\nx\n## Issue-Graph Notes\nx\n## Notes\nx\n## Tooling Notes\nx\n",
    )
    .expect("write stp");

    let bundle_input = issue_ref.task_bundle_input_path(&repo);
    fs::create_dir_all(bundle_input.parent().expect("input parent")).expect("mkdir");
    fs::write(
            &bundle_input,
            "# ADL Input Card\n\n## Goal\n\n\n## Required Outcome\n\n- State whether this issue must ship code, docs, tests, demo artifacts, or a combination.\n\n## Acceptance Criteria\n\n\n",
        )
        .expect("write stub input");

    let (_, repaired_input, _) = ensure_bootstrap_cards(
        &repo,
        &issue_ref,
        "[v0.86][tools] Rewrite bootstrap SIP",
        "codex/1154-rewrite-bootstrap-sip",
        &source_path,
    )
    .expect("bootstrap cards");

    let repaired_text = fs::read_to_string(repaired_input).expect("read repaired input");
    assert_eq!(
        bootstrap_stub_reason(&repaired_text, PromptSurfaceKind::Sip),
        None
    );
}
