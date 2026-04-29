use super::*;
use crate::cli::pr_cmd::github::{current_pr_url, OpenPullRequest};

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

#[test]
fn issue_version_prefers_labels_and_falls_back_to_title() {
    let _guard = env_lock();
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
fn unresolved_milestone_pr_wave_filters_by_version_queue_and_excluded_branch() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-unresolved-wave");
    let bin_dir = temp.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    write_executable(
        &bin_dir.join("gh"),
        "#!/usr/bin/env bash\nset -euo pipefail\nif [ \"$1 $2\" = 'pr list' ]; then\n  cat <<'EOF'\n[\n  {\n    \"number\": 101,\n    \"title\": \"[v0.90.4][docs] Quality gate, docs, and review convergence\",\n    \"url\": \"https://example.invalid/pr/101\",\n    \"headRefName\": \"codex/2435-v0-90-4-wp-15-quality-gate-docs-and-review-convergence\",\n    \"baseRefName\": \"main\",\n    \"isDraft\": true\n  },\n  {\n    \"number\": 102,\n    \"title\": \"[v0.90.4][WP-02] Runtime economics inheritance and authority audit\",\n    \"url\": \"https://example.invalid/pr/102\",\n    \"headRefName\": \"codex/2421-v0-90-4-wp-02-economics-inheritance-and-authority-audit\",\n    \"baseRefName\": \"main\",\n    \"isDraft\": false\n  },\n  {\n    \"number\": 103,\n    \"title\": \"[v0.90.3][WP-15] Older milestone tail\",\n    \"url\": \"https://example.invalid/pr/103\",\n    \"headRefName\": \"codex/old-tail\",\n    \"baseRefName\": \"main\",\n    \"isDraft\": false\n  },\n  {\n    \"number\": 104,\n    \"title\": \"[v0.90.4][WP-15] Wrong base\",\n    \"url\": \"https://example.invalid/pr/104\",\n    \"headRefName\": \"codex/wrong-base\",\n    \"baseRefName\": \"release\",\n    \"isDraft\": false\n  }\n]\nEOF\n  exit 0\nfi\nexit 1\n",
    );

    let old_path = env::var("PATH").unwrap_or_default();
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
    }

    let prs = unresolved_milestone_pr_wave(
        "danielbaustin/agent-design-language",
        "v0.90.4",
        "docs",
        Some("codex/2435-v0-90-4-wp-15-quality-gate-docs-and-review-convergence"),
    )
    .expect("wave");

    unsafe {
        env::set_var("PATH", old_path);
    }

    assert!(
        prs.is_empty(),
        "excluded docs branch should leave no matching PRs"
    );

    let old_path = env::var("PATH").unwrap_or_default();
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
    }

    let prs =
        unresolved_milestone_pr_wave("danielbaustin/agent-design-language", "v0.90.4", "wp", None)
            .expect("runtime wave");

    unsafe {
        env::set_var("PATH", old_path);
    }

    assert_eq!(prs.len(), 1);
    assert_eq!(prs[0].number, 102);
    assert_eq!(prs[0].queue.as_deref(), Some("wp"));
}

#[test]
fn unresolved_milestone_pr_wave_rejects_invalid_json() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-unresolved-wave-invalid-json");
    let bin_dir = temp.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    write_executable(
        &bin_dir.join("gh"),
        "#!/usr/bin/env bash\nset -euo pipefail\nif [ \"$1 $2\" = 'pr list' ]; then\n  printf 'not-json\\n'\n  exit 0\nfi\nexit 1\n",
    );

    let old_path = env::var("PATH").unwrap_or_default();
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
    }

    let err =
        unresolved_milestone_pr_wave("danielbaustin/agent-design-language", "v0.90.4", "wp", None)
            .expect_err("invalid json should be rejected");

    unsafe {
        env::set_var("PATH", old_path);
    }

    assert!(err.to_string().contains("failed to parse gh pr list json"));
}

#[test]
fn format_open_pr_wave_marks_draft_and_unknown_queue() {
    let mut pr = OpenPullRequest {
        number: 101,
        title: "[v0.90.4][WP-15] Quality gate, docs, and review convergence".to_string(),
        url: "https://example.invalid/pr/101".to_string(),
        head_ref_name: "codex/2435-v0-90-4-wp-15-quality-gate-docs-and-review-convergence"
            .to_string(),
        base_ref_name: "main".to_string(),
        is_draft: true,
        queue: Some("docs".to_string()),
    };
    let rendered = format_open_pr_wave(&[pr.clone()]);
    assert!(rendered.contains("#101 [draft] [queue=docs]"));
    assert!(rendered.contains("https://example.invalid/pr/101"));

    pr.queue = None;
    pr.is_draft = false;
    let rendered = format_open_pr_wave(&[pr]);
    assert!(rendered.contains("#101 [ready] [queue=unknown]"));
}

#[test]
fn ensure_source_issue_prompt_replaces_existing_bootstrap_stub_when_github_body_is_authored() {
    let _guard = env_lock();
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
    let _guard = env_lock();
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
fn real_pr_init_repairs_missing_version_metadata_on_github_issue() {
    let _guard = env_lock();
    let repo = unique_temp_dir("adl-pr-init-metadata-parity");
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
        .expect("git remote add")
        .success());

    let bin_dir = repo.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let gh_log = repo.join("gh.log");
    let title_state = repo.join("gh-title.txt");
    let labels_state = repo.join("gh-labels.txt");
    write_executable(
        &bin_dir.join("gh"),
        &format!(
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\nTITLE_FILE='{}'\nLABELS_FILE='{}'\nread_title() {{ if [[ -f \"$TITLE_FILE\" ]]; then cat \"$TITLE_FILE\"; else printf '[tools] Metadata parity\\n'; fi; }}\nread_labels() {{ if [[ -f \"$LABELS_FILE\" ]]; then cat \"$LABELS_FILE\"; else printf 'track:roadmap\\ntype:task\\narea:tools\\n'; fi; }}\nif [[ \"$*\" == *\"issue view 1153 -R owner/repo --json title -q .title\"* ]]; then\n  read_title\n  exit 0\nfi\nif [[ \"$*\" == *\"issue view 1153 -R owner/repo --json labels -q .labels[].name\"* ]]; then\n  read_labels\n  exit 0\nfi\nif [[ \"$*\" == *\"issue view 1153 -R owner/repo --json body -q .body\"* ]]; then\n  cat <<'EOF'\n## Summary\n\nRepair missing version metadata during init.\n\n## Goal\n\nKeep GitHub issue metadata aligned with the canonical local prompt.\n\n## Required Outcome\n\nThis issue ships tooling code and tests.\n\n## Deliverables\n\n- metadata parity enforcement\n\n## Acceptance Criteria\n\n- init repairs the missing version title prefix and version label\n\n## Repo Inputs\n\n- adl/src/cli/pr_cmd.rs\n\n## Dependencies\n\n- none\n\n## Demo Expectations\n\n- none\n\n## Non-goals\n\n- broader tracker redesign\n\n## Issue-Graph Notes\n\n- regression fixture\n\n## Notes\n\n- none\n\n## Tooling Notes\n\n- ensure bootstrap is truthful\nEOF\n  exit 0\nfi\nif [[ \"$*\" == *\"issue edit 1153 -R owner/repo --title [v0.87.1][tools] Metadata parity\"* ]]; then\n  printf '%s\\n' '[v0.87.1][tools] Metadata parity' > \"$TITLE_FILE\"\n  exit 0\nfi\nif [[ \"$*\" == *\"issue edit 1153 -R owner/repo\"* && \"$*\" == *\"--add-label\"* ]]; then\n  cat <<'EOF' > \"$LABELS_FILE\"\ntrack:roadmap\ntype:task\narea:tools\nversion:v0.87.1\nEOF\n  exit 0\nfi\nexit 1\n",
            gh_log.display(),
            title_state.display(),
            labels_state.display()
        ),
    );

    let old_path = env::var("PATH").unwrap_or_default();
    let prev_dir = env::current_dir().expect("cwd");
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
    }
    env::set_current_dir(&repo).expect("chdir");

    real_pr(&[
        "init".to_string(),
        "1153".to_string(),
        "--slug".to_string(),
        "v0-87-1-tools-metadata-parity".to_string(),
        "--version".to_string(),
        "v0.87.1".to_string(),
    ])
    .expect("real_pr init");

    env::set_current_dir(prev_dir).expect("restore cwd");
    unsafe {
        env::set_var("PATH", old_path);
    }

    assert_eq!(
        fs::read_to_string(&title_state).expect("title state"),
        "[v0.87.1][tools] Metadata parity\n"
    );
    let labels = fs::read_to_string(&labels_state).expect("labels state");
    assert!(labels.contains("version:v0.87.1"));

    let issue_ref = IssueRef::new(
        1153,
        "v0.87.1".to_string(),
        "v0-87-1-tools-metadata-parity".to_string(),
    )
    .expect("issue ref");
    let prompt = fs::read_to_string(issue_ref.issue_prompt_path(&repo)).expect("read prompt");
    assert!(prompt.contains("title: \"[v0.87.1][tools] Metadata parity\""));

    let gh_log = fs::read_to_string(&gh_log).expect("gh log");
    assert!(
        gh_log.contains("issue edit 1153 -R owner/repo --title [v0.87.1][tools] Metadata parity")
    );
    assert!(gh_log.contains("issue edit 1153 -R owner/repo --add-label version:v0.87.1"));
}

#[test]
fn ensure_issue_metadata_parity_errors_when_drift_remains_after_repair() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-metadata-parity-drift-remains");
    let bin_dir = temp.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let gh_log = temp.join("gh.log");
    let title_state = temp.join("gh-title.txt");
    let labels_state = temp.join("gh-labels.txt");
    fs::write(&title_state, "[tools] Metadata parity\n").expect("seed title");
    fs::write(
        &labels_state,
        "track:roadmap\ntype:task\narea:tools\nversion:v0.86\n",
    )
    .expect("seed labels");
    write_executable(
        &bin_dir.join("gh"),
        &format!(
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\nTITLE_FILE='{}'\nLABELS_FILE='{}'\nread_title() {{ cat \"$TITLE_FILE\"; }}\nread_labels() {{ cat \"$LABELS_FILE\"; }}\nif [[ \"$*\" == *\"issue view 1153 -R owner/repo --json title -q .title\"* ]]; then\n  read_title\n  exit 0\nfi\nif [[ \"$*\" == *\"issue view 1153 -R owner/repo --json labels -q .labels[].name\"* ]]; then\n  read_labels\n  exit 0\nfi\nif [[ \"$*\" == *\"issue edit 1153 -R owner/repo --title [v0.87.1][tools] Metadata parity\"* ]]; then\n  exit 0\nfi\nif [[ \"$*\" == *\"issue edit 1153 -R owner/repo\"* && \"$*\" == *\"--add-label version:v0.87.1\"* ]]; then\n  exit 0\nfi\nif [[ \"$*\" == *\"issue edit 1153 -R owner/repo\"* && \"$*\" == *\"--remove-label version:v0.86\"* ]]; then\n  exit 0\nfi\nexit 1\n",
            gh_log.display(),
            title_state.display(),
            labels_state.display()
        ),
    );

    let old_path = env::var("PATH").unwrap_or_default();
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
    }

    let err = ensure_issue_metadata_parity(
        "owner/repo",
        1153,
        "[v0.87.1][tools] Metadata parity",
        "track:roadmap,type:task,area:tools,version:v0.87.1",
    )
    .expect_err("drift should remain when gh edits are ineffective");

    unsafe {
        env::set_var("PATH", old_path);
    }

    let message = err.to_string();
    assert!(
        message.contains("title mismatch after metadata parity enforcement")
            || message.contains("metadata drift remains after parity enforcement")
    );
    let gh_log = fs::read_to_string(&gh_log).expect("gh log");
    assert!(
        gh_log.contains("issue edit 1153 -R owner/repo --title [v0.87.1][tools] Metadata parity")
    );
    assert!(gh_log.contains("issue edit 1153 -R owner/repo --add-label version:v0.87.1"));
    assert!(gh_log.contains("issue edit 1153 -R owner/repo --remove-label version:v0.86"));
}

#[test]
fn gh_issue_create_passes_labels_and_returns_created_url() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-gh-issue-create-success");
    let bin_dir = temp.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let gh_log = temp.join("gh.log");
    write_executable(
        &bin_dir.join("gh"),
        &format!(
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\nprintf 'https://github.com/owner/repo/issues/2625\\n'\n",
            gh_log.display()
        ),
    );

    let old_path = env::var("PATH").unwrap_or_default();
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
    }

    let created = gh_issue_create(
        "owner/repo",
        "[v0.90.5][tools] Coverage blocker repair",
        "Body text",
        "track:roadmap, type:task, area:tools, version:v0.90.5",
    )
    .expect("issue create");

    unsafe {
        env::set_var("PATH", old_path);
    }

    assert_eq!(created, "https://github.com/owner/repo/issues/2625");
    let gh_log = fs::read_to_string(&gh_log).expect("gh log");
    assert!(gh_log.contains("issue create -R owner/repo --title [v0.90.5][tools] Coverage blocker repair --body Body text"));
    assert!(gh_log.contains("--label track:roadmap"));
    assert!(gh_log.contains("--label type:task"));
    assert!(gh_log.contains("--label area:tools"));
    assert!(gh_log.contains("--label version:v0.90.5"));
}

#[test]
fn gh_issue_create_reports_stderr_and_empty_output_failures() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-gh-issue-create-failure");
    let bin_dir = temp.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let mode_file = temp.join("mode.txt");
    fs::write(&mode_file, "stderr\n").expect("seed mode");
    write_executable(
        &bin_dir.join("gh"),
        &format!(
            "#!/usr/bin/env bash\nset -euo pipefail\nmode=\"$(cat '{}')\"\ncase \"$mode\" in\n  stderr)\n    echo 'create failed' >&2\n    exit 1\n    ;;\n  empty)\n    exit 0\n    ;;\n  *)\n    printf 'https://github.com/owner/repo/issues/1\\n'\n    ;;\nesac\n",
            mode_file.display()
        ),
    );

    let old_path = env::var("PATH").unwrap_or_default();
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
    }

    let err = gh_issue_create("owner/repo", "x", "y", "track:roadmap")
        .expect_err("stderr failure should bubble");
    assert!(err.to_string().contains("gh issue create failed: create failed"));

    fs::write(&mode_file, "empty\n").expect("switch mode");
    let err = gh_issue_create("owner/repo", "x", "y", "track:roadmap")
        .expect_err("empty output should fail");
    assert!(err
        .to_string()
        .contains("gh issue create returned empty output"));

    unsafe {
        env::set_var("PATH", old_path);
    }
}

#[test]
fn gh_issue_edit_body_and_metadata_parity_cover_command_shapes() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-gh-issue-helper-shapes");
    let bin_dir = temp.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let gh_log = temp.join("gh.log");
    let title_state = temp.join("title.txt");
    let labels_state = temp.join("labels.txt");
    fs::write(&title_state, "[tools] Old title\n").expect("seed title");
    fs::write(
        &labels_state,
        "track:roadmap\narea:tools\nversion:v0.86\nqueue:old\n",
    )
    .expect("seed labels");
    write_executable(
        &bin_dir.join("gh"),
        &format!(
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\nTITLE_FILE='{}'\nLABELS_FILE='{}'\nread_title() {{ cat \"$TITLE_FILE\"; }}\nread_labels() {{ cat \"$LABELS_FILE\"; }}\nif [[ \"$*\" == *\"issue view 1153 -R owner/repo --json title -q .title\"* ]]; then\n  read_title\n  exit 0\nfi\nif [[ \"$*\" == *\"issue view 1153 -R owner/repo --json labels -q .labels[].name\"* ]]; then\n  read_labels\n  exit 0\nfi\nif [[ \"$*\" == *\"issue edit 1153 -R owner/repo --title [v0.90.5][tools] Coverage blocker repair\"* ]]; then\n  printf '[v0.90.5][tools] Coverage blocker repair\\n' > \"$TITLE_FILE\"\n  exit 0\nfi\nif [[ \"$*\" == *\"issue edit 1153 -R owner/repo\"* && \"$*\" == *\"--add-label queue:tools\"* && \"$*\" == *\"--add-label version:v0.90.5\"* ]]; then\n  printf 'track:roadmap\\narea:tools\\nversion:v0.86\\nqueue:old\\nversion:v0.90.5\\nqueue:tools\\n' > \"$LABELS_FILE\"\n  exit 0\nfi\nif [[ \"$*\" == *\"issue edit 1153 -R owner/repo\"* && \"$*\" == *\"--remove-label version:v0.86\"* ]]; then\n  printf 'track:roadmap\\narea:tools\\nqueue:old\\nversion:v0.90.5\\nqueue:tools\\n' > \"$LABELS_FILE\"\n  exit 0\nfi\nexit 0\n",
            gh_log.display(),
            title_state.display(),
            labels_state.display()
        ),
    );

    let old_path = env::var("PATH").unwrap_or_default();
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
    }

    gh_issue_edit_body("owner/repo", 1153, "Updated body").expect("edit body");
    ensure_issue_metadata_parity(
        "owner/repo",
        1153,
        "[v0.90.5][tools] Coverage blocker repair",
        "track:roadmap,area:tools,version:v0.90.5,queue:tools",
    )
    .expect("metadata parity");

    unsafe {
        env::set_var("PATH", old_path);
    }

    let gh_log = fs::read_to_string(&gh_log).expect("gh log");
    assert!(gh_log.contains("issue view 1153 -R owner/repo --json labels -q .labels[].name"));
    assert!(gh_log.contains("issue edit 1153 -R owner/repo --body-file"));
    assert!(gh_log.contains("issue edit 1153 -R owner/repo --title [v0.90.5][tools] Coverage blocker repair"));
    assert!(gh_log.contains("issue edit 1153 -R owner/repo --add-label"));
    assert!(gh_log.contains("--add-label queue:tools"));
    assert!(gh_log.contains("--add-label version:v0.90.5"));
    assert!(gh_log.contains("issue edit 1153 -R owner/repo --remove-label version:v0.86"));
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
        "#!/usr/bin/env bash\nset -euo pipefail\nif [[ \"$*\" == *\"issue view 1153 -R owner/repo --json title -q .title\"* ]]; then\n  printf '[runtime] Missing version metadata\\n'\n  exit 0\nfi\nif [[ \"$*\" == *\"issue view 1153 -R owner/repo --json labels -q .labels[].name\"* ]]; then\n  printf 'track:roadmap\\ntype:task\\narea:runtime\\n'\n  exit 0\nfi\nexit 1\n",
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
    let _guard = env_lock();
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
