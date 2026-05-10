use super::*;

#[test]
fn issue_create_repo_requires_github_origin_remote() {
    let _guard = env_lock();
    let repo = unique_temp_dir("adl-pr-create-repo-guard");
    assert!(Command::new("git")
        .arg("init")
        .arg("-q")
        .current_dir(&repo)
        .status()
        .expect("git init")
        .success());

    let err = issue_create_repo(&repo).expect_err("missing origin should fail");
    assert!(err
        .to_string()
        .contains("refusing to infer the GitHub issue target from ambient gh context"));

    assert!(Command::new("git")
        .args([
            "remote",
            "add",
            "origin",
            "https://gitlab.example.com/example/repo.git"
        ])
        .current_dir(&repo)
        .status()
        .expect("git remote add")
        .success());

    let err = issue_create_repo(&repo).expect_err("non-github origin should fail");
    assert!(err
        .to_string()
        .contains("refusing to infer the GitHub issue target from ambient gh context"));

    assert!(Command::new("git")
        .args([
            "remote",
            "set-url",
            "origin",
            "https://github.com/example/repo.git"
        ])
        .current_dir(&repo)
        .status()
        .expect("git remote set-url")
        .success());

    assert_eq!(
        issue_create_repo(&repo).expect("github origin"),
        "example/repo"
    );
}

#[test]
fn default_repo_falls_back_to_local_name_when_remote_and_gh_are_unavailable() {
    let _guard = env_lock();
    let repo = unique_temp_dir("adl-pr-default-repo-fallback");
    assert!(Command::new("git")
        .arg("init")
        .arg("-q")
        .current_dir(&repo)
        .status()
        .expect("git init")
        .success());

    let bin_dir = repo.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    write_executable(
        &bin_dir.join("gh"),
        "#!/usr/bin/env bash\nset -euo pipefail\nexit 1\n",
    );

    let old_path = env::var("PATH").unwrap_or_default();
    let prev_dir = env::current_dir().expect("cwd");
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
    }
    env::set_current_dir(&repo).expect("chdir");

    let inferred = default_repo(&repo).expect("default repo");

    env::set_current_dir(prev_dir).expect("restore cwd");
    unsafe {
        env::set_var("PATH", old_path);
    }
    assert_eq!(
        inferred,
        format!("local/{}", repo.file_name().unwrap().to_string_lossy())
    );
}

#[test]
fn default_repo_uses_gh_repo_when_remote_is_unparseable() {
    let _guard = env_lock();
    let repo = unique_temp_dir("adl-pr-default-repo-gh");
    assert!(Command::new("git")
        .arg("init")
        .arg("-q")
        .current_dir(&repo)
        .status()
        .expect("git init")
        .success());

    let bin_dir = repo.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    write_executable(
        &bin_dir.join("gh"),
        "#!/usr/bin/env bash\nset -euo pipefail\nprintf 'owner/example\\n'\n",
    );

    let old_path = env::var("PATH").unwrap_or_default();
    let prev_dir = env::current_dir().expect("cwd");
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
    }
    env::set_current_dir(&repo).expect("chdir");

    let inferred = default_repo(&repo).expect("default repo");

    env::set_current_dir(prev_dir).expect("restore cwd");
    unsafe {
        env::set_var("PATH", old_path);
    }
    assert_eq!(inferred, "owner/example");
}

#[test]
fn fetch_origin_main_with_fallback_reuses_local_origin_main_and_errors_when_missing() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-fetch-fallback");
    let bin_dir = temp.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    write_executable(
            &bin_dir.join("git"),
            "#!/usr/bin/env bash\nset -euo pipefail\nif [ \"$1 $2 $3\" = 'fetch origin main' ]; then\n  exit 1\nfi\nif [ \"$1 $2 $3 $4\" = 'rev-parse --verify --quiet origin/main' ]; then\n  if [ \"${HAS_ORIGIN_MAIN:-0}\" = '1' ]; then\n    exit 0\n  fi\n  exit 1\nfi\nexit 1\n",
        );

    let old_path = env::var("PATH").unwrap_or_default();
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
        env::set_var("HAS_ORIGIN_MAIN", "1");
    }
    fetch_origin_main_with_fallback().expect("should reuse local origin/main");

    unsafe {
        env::set_var("HAS_ORIGIN_MAIN", "0");
    }
    let err = fetch_origin_main_with_fallback().expect_err("missing origin/main should fail");
    unsafe {
        env::set_var("PATH", old_path);
        env::remove_var("HAS_ORIGIN_MAIN");
    }
    assert!(err
        .to_string()
        .contains("fetch origin main failed and origin/main is unavailable locally"));
}

#[test]
fn ensure_worktree_for_branch_rejects_branch_checked_out_elsewhere() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-worktree-conflict");
    let bin_dir = temp.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let repo = temp.join("repo");
    fs::create_dir_all(repo.join(".git")).expect("repo git dir");
    write_executable(
            &bin_dir.join("git"),
            &format!(
                "#!/usr/bin/env bash\nset -euo pipefail\nif [ \"$1 $2\" = 'rev-parse --show-toplevel' ]; then\n  printf '%s\\n' '{0}'\n  exit 0\nfi\nif [ \"$1 $2\" = 'rev-parse --git-common-dir' ]; then\n  printf '%s\\n' '{1}'\n  exit 0\nfi\nif [ \"$1 $2 $3\" = 'worktree list --porcelain' ]; then\n  cat <<'EOF'\nworktree {0}\nHEAD deadbeef\nbranch refs/heads/main\n\nworktree {0}/.worktrees/existing\nHEAD cafefood\nbranch refs/heads/codex/1153-test\nEOF\n  exit 0\nfi\nexit 1\n",
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
    let err = ensure_worktree_for_branch(
        &repo.join(".worktrees").join("requested"),
        "codex/1153-test",
    )
    .expect_err("conflicting worktree should fail");
    env::set_current_dir(old_pwd).expect("restore cwd");
    unsafe {
        env::set_var("PATH", old_path);
    }
    assert!(err.to_string().contains("already checked out in worktree"));
    assert!(err.to_string().contains(
        &repo
            .join(".worktrees")
            .join("existing")
            .display()
            .to_string()
    ));
}

#[test]
fn ensure_local_branch_exists_covers_existing_remote_and_new_branch_paths() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-ensure-branch");
    let bin_dir = temp.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let git_log = temp.join("git.log");
    write_executable(
            &bin_dir.join("git"),
            &format!(
                "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\ncase \"$*\" in\n  'show-ref --verify --quiet refs/heads/codex/existing') exit 0 ;;\n  'show-ref --verify --quiet refs/heads/codex/remote') exit 1 ;;\n  'ls-remote --exit-code --heads origin codex/remote') exit 0 ;;\n  'branch --track codex/remote origin/codex/remote') exit 0 ;;\n  'show-ref --verify --quiet refs/heads/codex/new') exit 1 ;;\n  'ls-remote --exit-code --heads origin codex/new') exit 1 ;;\n  'branch codex/new origin/main') exit 0 ;;\n  *) exit 1 ;;\nesac\n",
                git_log.display()
            ),
        );

    let old_path = env::var("PATH").unwrap_or_default();
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
    }

    ensure_local_branch_exists("codex/existing").expect("existing local branch");
    ensure_local_branch_exists("codex/remote").expect("remote tracking branch");
    ensure_local_branch_exists("codex/new").expect("new branch from origin/main");

    unsafe {
        env::set_var("PATH", old_path);
    }
    let log = fs::read_to_string(&git_log).expect("git log");
    assert!(log.contains("show-ref --verify --quiet refs/heads/codex/existing"));
    assert!(log.contains("branch --track codex/remote origin/codex/remote"));
    assert!(log.contains("branch codex/new origin/main"));
}
