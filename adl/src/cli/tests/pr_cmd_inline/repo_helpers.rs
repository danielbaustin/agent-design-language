use super::*;

#[test]
fn default_repo_falls_back_to_local_name_when_remote_and_gh_are_unavailable() {
    let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
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
    let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
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
    let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
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
    let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
    let temp = unique_temp_dir("adl-pr-worktree-conflict");
    let bin_dir = temp.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    write_executable(
            &bin_dir.join("git"),
            "#!/usr/bin/env bash\nset -euo pipefail\nif [ \"$1 $2 $3\" = 'worktree list --porcelain' ]; then\n  cat <<'EOF'\nworktree /tmp/main\nHEAD deadbeef\nbranch refs/heads/main\n\nworktree /tmp/existing\nHEAD cafefood\nbranch refs/heads/codex/1153-test\nEOF\n  exit 0\nfi\nexit 1\n",
        );

    let old_path = env::var("PATH").unwrap_or_default();
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
    }
    let err = ensure_worktree_for_branch(Path::new("/tmp/requested"), "codex/1153-test")
        .expect_err("conflicting worktree should fail");
    unsafe {
        env::set_var("PATH", old_path);
    }
    assert!(err.to_string().contains("already checked out in worktree"));
    assert!(err.to_string().contains("/tmp/existing"));
}

#[test]
fn ensure_local_branch_exists_covers_existing_remote_and_new_branch_paths() {
    let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
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

#[test]
fn issue_version_prefers_labels_and_falls_back_to_title() {
    let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
    let temp = unique_temp_dir("adl-pr-issue-version");
    let bin_dir = temp.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    write_executable(
            &bin_dir.join("gh"),
            "#!/usr/bin/env bash\nset -euo pipefail\nif [ \"$1 $2 $3 $4\" = 'issue view 1153 -R' ]; then\n  case \"${GH_MODE:-labels}\" in\n    labels) printf 'track:roadmap\\nversion:v0.86\\n' ;;\n    title) printf '[v0.89][WP-15] Demo issue\\n' ;;\n    *) printf 'track:roadmap\\n' ;;\n  esac\n  exit 0\nfi\nexit 1\n",
        );

    let old_path = env::var("PATH").unwrap_or_default();
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
        env::set_var("GH_MODE", "labels");
    }
    assert_eq!(
        issue_version(1153, "owner/repo").expect("labels"),
        Some("v0.86".to_string())
    );
    unsafe {
        env::set_var("GH_MODE", "title");
    }
    assert_eq!(
        issue_version(1153, "owner/repo").expect("title"),
        Some("v0.89".to_string())
    );
    unsafe {
        env::set_var("PATH", old_path);
        env::remove_var("GH_MODE");
    }
}

#[test]
fn ensure_source_issue_prompt_replaces_existing_bootstrap_stub_when_github_body_is_authored() {
    let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
    let repo = unique_temp_dir("adl-pr-replace-bootstrap-stub");
    init_git_repo(&repo);

    let bin_dir = repo.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let gh_log = repo.join("gh.log");
    write_executable(
        &bin_dir.join("gh"),
        &format!(
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\nif [[ \"$*\" == *\"issue view 1153 -R owner/repo --json body -q .body\"* ]]; then\n  cat <<'EOF'\n---\nissue_card_schema: adl.issue.v1\nwp: \"unassigned\"\nslug: \"v0-86-tools-replace-bootstrap-stub\"\ntitle: \"[v0.86][tools] Replace bootstrap stub\"\nlabels:\n  - \"track:roadmap\"\n  - \"type:task\"\n  - \"area:tools\"\n  - \"version:v0.86\"\nstatus: \"active\"\naction: \"edit\"\ndepends_on: []\nmilestone_sprint: \"Pending sprint assignment\"\nrequired_outcome_type:\n  - \"code\"\nrepo_inputs:\n  - \"adl/src/cli/pr_cmd.rs\"\ncanonical_files:\n  - \"adl/src/cli/pr_cmd.rs\"\ndemo_required: false\ndemo_names: []\nissue_graph_notes:\n  - \"GitHub-authored body should replace the local stub.\"\npr_start:\n  enabled: false\n  slug: \"v0-86-tools-replace-bootstrap-stub\"\n---\n\n# [v0.86][tools] Replace bootstrap stub\n\n## Summary\n\nAuthored GitHub issue body should win over the local bootstrap stub.\n\n## Goal\n\nPreserve the authored issue body locally.\n\n## Acceptance Criteria\n\n- replace the bootstrap stub\n- keep the authored body intact\nEOF\n  exit 0\nfi\nexit 1\n",
            gh_log.display()
        ),
    );

    let issue_ref = IssueRef::new(
        1153,
        "v0.86".to_string(),
        "v0-86-tools-replace-bootstrap-stub".to_string(),
    )
    .expect("issue ref");
    let source_path = issue_ref.issue_prompt_path(&repo);
    if let Some(parent) = source_path.parent() {
        fs::create_dir_all(parent).expect("source parent");
    }
    fs::write(
        &source_path,
        render_generated_issue_prompt(
            1153,
            "v0-86-tools-replace-bootstrap-stub",
            "[v0.86][tools] Replace bootstrap stub",
            "track:roadmap,type:task,area:tools,version:v0.86",
            "https://github.com/owner/repo/issues/1153",
        ),
    )
    .expect("write stub prompt");

    let old_path = env::var("PATH").unwrap_or_default();
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
    }

    let source = ensure_source_issue_prompt(
        &repo,
        "owner/repo",
        &issue_ref,
        "[v0.86][tools] Replace bootstrap stub",
        Some("track:roadmap,type:task,area:tools"),
        "v0.86",
        "track:roadmap,type:task,area:tools",
    )
    .expect("ensure source prompt");

    unsafe {
        env::set_var("PATH", old_path);
    }

    let prompt = fs::read_to_string(&source).expect("read prompt");
    assert!(prompt.contains("issue_number: 1153"));
    assert!(prompt.contains("Authored GitHub issue body should win over the local bootstrap stub."));
    assert!(prompt.contains("Preserve the authored issue body locally."));
    assert!(prompt.contains("replace the bootstrap stub"));
    assert!(!prompt
        .contains("Bootstrap-generated issue body created from the requested title and labels"));
    let gh_log = fs::read_to_string(&gh_log).expect("gh log");
    assert!(gh_log.contains("issue view 1153 -R owner/repo --json body -q .body"));
}

#[test]
fn ensure_source_issue_prompt_preserves_authored_front_matter_from_github_body() {
    let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
    let repo = unique_temp_dir("adl-pr-preserve-authored-front-matter");
    init_git_repo(&repo);

    let bin_dir = repo.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    write_executable(
        &bin_dir.join("gh"),
        "#!/usr/bin/env bash\nset -euo pipefail\nif [[ \"$*\" == *\"issue view 1152 -R owner/repo --json body -q .body\"* ]]; then\n  cat <<'EOF'\n---\nissue_card_schema: adl.issue.v1\nwp: \"unassigned\"\nslug: \"v0-86-tools-preserve-authored-front-matter\"\ntitle: \"[v0.86][tools] Preserve authored front matter\"\nlabels:\n  - \"track:roadmap\"\n  - \"type:task\"\n  - \"area:tools\"\n  - \"version:v0.86\"\nstatus: \"active\"\naction: \"edit\"\ndepends_on: []\nmilestone_sprint: \"Pending sprint assignment\"\nrequired_outcome_type:\n  - \"code\"\nrepo_inputs:\n  - \"adl/src/cli/pr_cmd.rs\"\ncanonical_files:\n  - \"adl/src/cli/pr_cmd.rs\"\ndemo_required: false\ndemo_names: []\nissue_graph_notes:\n  - \"Authored on GitHub first.\"\npr_start:\n  enabled: false\n  slug: \"v0-86-tools-preserve-authored-front-matter\"\n---\n\n# [v0.86][tools] Preserve authored front matter\n\n## Summary\n\nAuthored issue body with front matter from GitHub.\n\n## Goal\n\nKeep this authored structure during bootstrap.\n\n## Acceptance Criteria\n\n- preserve authored front matter\n- inject issue number locally\nEOF\n  exit 0\nfi\nexit 1\n",
    );

    let old_path = env::var("PATH").unwrap_or_default();
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
    }

    let issue_ref = IssueRef::new(
        1152,
        "v0.86".to_string(),
        "v0-86-tools-preserve-authored-front-matter".to_string(),
    )
    .expect("issue ref");
    let source = ensure_source_issue_prompt(
        &repo,
        "owner/repo",
        &issue_ref,
        "[v0.86][tools] Preserve authored front matter",
        Some("track:roadmap,type:task,area:tools"),
        "v0.86",
        "track:roadmap,type:task,area:tools",
    )
    .expect("ensure source prompt");

    unsafe {
        env::set_var("PATH", old_path);
    }

    let prompt = fs::read_to_string(&source).expect("read prompt");
    assert!(prompt.contains("issue_number: 1152"));
    assert!(prompt.contains("status: active"));
    assert!(prompt.contains("Authored issue body with front matter from GitHub."));
    assert!(prompt.contains("Keep this authored structure during bootstrap."));
    assert!(prompt.contains("preserve authored front matter"));
    assert!(!prompt
        .contains("Bootstrap-generated issue body created from the requested title and labels"));
}

#[test]
fn current_pr_url_filters_empty_and_null_results() {
    let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
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
    let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
    let temp = unique_temp_dir("adl-pr-worktree-none");
    let bin_dir = temp.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    write_executable(
            &bin_dir.join("git"),
            "#!/usr/bin/env bash\nset -euo pipefail\nif [ \"$1 $2 $3\" = 'worktree list --porcelain' ]; then\n  cat <<'EOF'\nworktree /tmp/main\nHEAD deadbeef\nbranch refs/heads/main\nEOF\n  exit 0\nfi\nexit 1\n",
        );

    let old_path = env::var("PATH").unwrap_or_default();
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
    }
    assert_eq!(
        branch_checked_out_worktree_path("codex/missing").expect("none"),
        None
    );
    unsafe {
        env::set_var("PATH", old_path);
    }
}

#[test]
fn ensure_worktree_for_branch_reuses_matching_path_and_creates_new_one() {
    let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
    let temp = unique_temp_dir("adl-pr-worktree-reuse-create");
    let bin_dir = temp.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let git_log = temp.join("git.log");
    write_executable(
            &bin_dir.join("git"),
            &format!(
                "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\nif [ \"$1 $2 $3\" = 'worktree list --porcelain' ]; then\n  if [ \"${{WT_MODE:-reuse}}\" = 'reuse' ]; then\n    cat <<'EOF'\nworktree /tmp/reuse-me\nHEAD deadbeef\nbranch refs/heads/codex/reuse\nEOF\n    exit 0\n  fi\n  printf 'worktree /tmp/main\\nHEAD deadbeef\\nbranch refs/heads/main\\n'\n  exit 0\nfi\nif [ \"$1 $2 $3\" = 'worktree add /tmp/create-me' ]; then\n  mkdir -p /tmp/create-me\n  exit 0\nfi\nexit 1\n",
                git_log.display()
            ),
        );

    let old_path = env::var("PATH").unwrap_or_default();
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
        env::set_var("WT_MODE", "reuse");
    }
    ensure_worktree_for_branch(Path::new("/tmp/reuse-me"), "codex/reuse").expect("reuse");

    unsafe {
        env::set_var("WT_MODE", "create");
    }
    let create_path = Path::new("/tmp/create-me");
    let _ = fs::remove_dir_all(create_path);
    ensure_worktree_for_branch(create_path, "codex/create").expect("create");

    unsafe {
        env::set_var("PATH", old_path);
        env::remove_var("WT_MODE");
    }
    let log = fs::read_to_string(&git_log).expect("git log");
    assert!(log.contains("worktree add /tmp/create-me codex/create"));
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
fn real_pr_start_rejects_missing_slug_or_empty_sanitized_title_in_no_fetch_mode() {
    let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
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
    let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
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

    let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
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
fn ensure_primary_checkout_on_main_handles_dirty_and_clean_non_main_states() {
    let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
    let repo = unique_temp_dir("adl-pr-primary-main");
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
    fs::write(repo.join("README.md"), "hello\n").expect("write readme");
    assert!(Command::new("git")
        .args(["add", "README.md"])
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
        .args(["checkout", "-q", "-b", "codex/1153-test"])
        .current_dir(&repo)
        .status()
        .expect("git checkout")
        .success());

    fs::write(repo.join("README.md"), "dirty\n").expect("dirty write");
    let err = ensure_primary_checkout_on_main(&repo).expect_err("dirty non-main should fail");
    assert!(err.to_string().contains("with local changes"));

    assert!(Command::new("git")
        .args(["restore", "README.md"])
        .current_dir(&repo)
        .status()
        .expect("git restore")
        .success());
    ensure_primary_checkout_on_main(&repo).expect("clean non-main should switch");
    let branch = current_branch(&repo).expect("branch");
    assert_eq!(branch, "main");
}

#[test]
fn ensure_bootstrap_cards_creates_bundle_and_compat_links() {
    let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
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
    assert!(compat_stp.symlink_metadata().is_ok());
    assert!(compat_input.symlink_metadata().is_ok());
    assert!(compat_output.symlink_metadata().is_ok());
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
    let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
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
