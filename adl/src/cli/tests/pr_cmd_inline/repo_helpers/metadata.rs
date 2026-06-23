use super::*;
use crate::cli::pr_cmd::github::{
    ensure_issue_metadata_parity, ensure_or_repair_pr_closing_linkage, ensure_pr_closing_linkage,
    gh_issue_create, gh_issue_edit_body, gh_issue_title, pr_has_closing_linkage, OpenPullRequest,
};
use crate::cli::pr_cmd_prompt::render_generated_issue_prompt;

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
        "#!/usr/bin/env bash\nset -euo pipefail\nif [ \"$1 $2\" = 'pr list' ]; then\n  cat <<'EOF'\n[\n  {\n    \"number\": 101,\n    \"title\": \"[v0.90.4][docs] Quality gate, docs, and review convergence\",\n    \"url\": \"https://example.invalid/pr/101\",\n    \"headRefName\": \"codex/2435-v0-90-4-wp-15-quality-gate-docs-and-review-convergence\",\n    \"baseRefName\": \"main\",\n    \"isDraft\": true\n  },\n  {\n    \"number\": 102,\n    \"title\": \"[v0.90.4][WP-02] Runtime economics inheritance and authority audit\",\n    \"url\": \"https://example.invalid/pr/102\",\n    \"headRefName\": \"codex/2421-v0-90-4-wp-02-economics-inheritance-and-authority-audit\",\n    \"baseRefName\": \"main\",\n    \"isDraft\": false\n  },\n  {\n    \"number\": 103,\n    \"title\": \"[v0.90.3][WP-15] Older milestone tail\",\n    \"url\": \"https://example.invalid/pr/103\",\n    \"headRefName\": \"codex/old-tail\",\n    \"baseRefName\": \"main\",\n    \"isDraft\": false\n  },\n  {\n    \"number\": 104,\n    \"title\": \"[v0.90.4][WP-15] Wrong base\",\n    \"url\": \"https://example.invalid/pr/104\",\n    \"headRefName\": \"codex/wrong-base\",\n    \"baseRefName\": \"release\",\n    \"isDraft\": false\n  }\n]\nEOF\n  exit 0\nfi\nif [ \"$1 $2\" = 'pr view' ]; then\n  printf '2421\\n'\n  exit 0\nfi\nexit 1\n",
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

    assert!(err
        .to_string()
        .contains("failed to parse GitHub PR list JSON"));
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
        state: "OPEN".to_string(),
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
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\nif [[ \"$*\" == *\"issue view 1153 -R owner/repo --json body --jq .body\"* ]]; then\n  cat <<'EOF'\n---\nissue_card_schema: adl.issue.v1\nwp: \"unassigned\"\nslug: \"v0-86-tools-replace-bootstrap-stub\"\ntitle: \"[v0.86][tools] Replace bootstrap stub\"\nlabels:\n  - \"track:roadmap\"\n  - \"type:task\"\n  - \"area:tools\"\n  - \"version:v0.86\"\nstatus: \"active\"\naction: \"edit\"\ndepends_on: []\nmilestone_sprint: \"Pending sprint assignment\"\nrequired_outcome_type:\n  - \"code\"\nrepo_inputs:\n  - \"adl/src/cli/pr_cmd.rs\"\ncanonical_files:\n  - \"adl/src/cli/pr_cmd.rs\"\ndemo_required: false\ndemo_names: []\nissue_graph_notes:\n  - \"GitHub-authored body should replace the local stub.\"\npr_start:\n  enabled: false\n  slug: \"v0-86-tools-replace-bootstrap-stub\"\n---\n\n# [v0.86][tools] Replace bootstrap stub\n\n## Summary\n\nAuthored GitHub issue body should win over the local bootstrap stub.\n\n## Goal\n\nPreserve the authored issue body locally.\n\n## Acceptance Criteria\n\n- replace the bootstrap stub\n- keep the authored body intact\nEOF\n  exit 0\nfi\nexit 1\n",
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
    assert!(gh_log.contains("issue view 1153 -R owner/repo --json body --jq .body"));
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
        "#!/usr/bin/env bash\nset -euo pipefail\nif [[ \"$*\" == *\"issue view 1152 -R owner/repo --json body --jq .body\"* ]]; then\n  cat <<'EOF'\n---\nissue_card_schema: adl.issue.v1\nwp: \"unassigned\"\nslug: \"v0-86-tools-preserve-authored-front-matter\"\ntitle: \"[v0.86][tools] Preserve authored front matter\"\nlabels:\n  - \"track:roadmap\"\n  - \"type:task\"\n  - \"area:tools\"\n  - \"version:v0.86\"\nstatus: \"active\"\naction: \"edit\"\ndepends_on: []\nmilestone_sprint: \"Pending sprint assignment\"\nrequired_outcome_type:\n  - \"code\"\nrepo_inputs:\n  - \"adl/src/cli/pr_cmd.rs\"\ncanonical_files:\n  - \"adl/src/cli/pr_cmd.rs\"\ndemo_required: false\ndemo_names: []\nissue_graph_notes:\n  - \"Authored on GitHub first.\"\npr_start:\n  enabled: false\n  slug: \"v0-86-tools-preserve-authored-front-matter\"\n---\n\n# [v0.86][tools] Preserve authored front matter\n\n## Summary\n\nAuthored issue body with front matter from GitHub.\n\n## Goal\n\nKeep this authored structure during bootstrap.\n\n## Acceptance Criteria\n\n- preserve authored front matter\n- inject issue number locally\nEOF\n  exit 0\nfi\nexit 1\n",
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
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\nTITLE_FILE='{}'\nLABELS_FILE='{}'\nread_title() {{ if [[ -f \"$TITLE_FILE\" ]]; then cat \"$TITLE_FILE\"; else printf '[tools] Metadata parity\\n'; fi; }}\nread_labels() {{ if [[ -f \"$LABELS_FILE\" ]]; then cat \"$LABELS_FILE\"; else printf 'track:roadmap\\ntype:task\\narea:tools\\n'; fi; }}\nif [[ \"$1 $2\" == \"label list\" ]]; then\n  printf 'track:roadmap\\ntype:task\\narea:tools\\nversion:v0.87.1\\n'\n  exit 0\nfi\nif [[ \"$*\" == *\"issue view 1153 -R owner/repo --json title --jq .title\"* ]]; then\n  read_title\n  exit 0\nfi\nif [[ \"$*\" == *\"issue view 1153 -R owner/repo --json labels --jq .labels[].name\"* ]]; then\n  read_labels\n  exit 0\nfi\nif [[ \"$*\" == *\"issue view 1153 -R owner/repo --json body --jq .body\"* ]]; then\n  cat <<'EOF'\n## Summary\n\nRepair missing version metadata during init.\n\n## Goal\n\nKeep GitHub issue metadata aligned with the canonical local prompt.\n\n## Required Outcome\n\nThis issue ships tooling code and tests.\n\n## Deliverables\n\n- metadata parity enforcement\n\n## Acceptance Criteria\n\n- init repairs the missing version title prefix and version label\n\n## Repo Inputs\n\n- adl/src/cli/pr_cmd.rs\n\n## Dependencies\n\n- none\n\n## Demo Expectations\n\n- none\n\n## Non-goals\n\n- broader tracker redesign\n\n## Issue-Graph Notes\n\n- regression fixture\n\n## Notes\n\n- none\n\n## Tooling Notes\n\n- ensure bootstrap is truthful\nEOF\n  exit 0\nfi\nif [[ \"$*\" == *\"issue edit 1153 -R owner/repo --title [v0.87.1][tools] Metadata parity\"* ]]; then\n  printf '%s\\n' '[v0.87.1][tools] Metadata parity' > \"$TITLE_FILE\"\n  exit 0\nfi\nif [[ \"$*\" == *\"issue edit 1153 -R owner/repo\"* && \"$*\" == *\"--add-label area:tools,track:roadmap,type:task,version:v0.87.1\"* ]]; then\n  cat <<'EOF' > \"$LABELS_FILE\"\narea:tools\ntrack:roadmap\ntype:task\nversion:v0.87.1\nEOF\n  exit 0\nfi\nexit 1\n",
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
    assert!(gh_log.contains(
        "issue edit 1153 -R owner/repo --add-label area:tools,track:roadmap,type:task,version:v0.87.1"
    ));
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
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\nTITLE_FILE='{}'\nLABELS_FILE='{}'\nread_title() {{ cat \"$TITLE_FILE\"; }}\nread_labels() {{ cat \"$LABELS_FILE\"; }}\nif [[ \"$1 $2\" == \"label list\" ]]; then\n  printf 'track:roadmap\\ntype:task\\narea:tools\\nversion:v0.87.1\\n'\n  exit 0\nfi\nif [[ \"$*\" == *\"issue view 1153 -R owner/repo --json title --jq .title\"* ]]; then\n  read_title\n  exit 0\nfi\nif [[ \"$*\" == *\"issue view 1153 -R owner/repo --json labels --jq .labels[].name\"* ]]; then\n  read_labels\n  exit 0\nfi\nif [[ \"$*\" == *\"issue edit 1153 -R owner/repo --title [v0.87.1][tools] Metadata parity\"* ]]; then\n  exit 0\nfi\nif [[ \"$*\" == *\"issue edit 1153 -R owner/repo\"* && \"$*\" == *\"--add-label area:tools,track:roadmap,type:task,version:v0.87.1\"* ]]; then\n  exit 0\nfi\nexit 1\n",
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
    assert!(gh_log.contains(
        "issue edit 1153 -R owner/repo --add-label area:tools,track:roadmap,type:task,version:v0.87.1"
    ));
}

#[test]
fn github_issue_create_and_metadata_helpers_cover_direct_success_paths() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-github-helper-success");
    let bin_dir = temp.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let gh_log = temp.join("gh.log");
    let title_state = temp.join("gh-title.txt");
    let labels_state = temp.join("gh-labels.txt");
    let body_state = temp.join("gh-body.txt");
    fs::write(&title_state, "[tools] Metadata parity\n").expect("seed title");
    fs::write(
        &labels_state,
        "track:roadmap\ntype:task\narea:tools\nversion:v0.86\n",
    )
    .expect("seed labels");
    fs::write(&body_state, "initial body\n").expect("seed body");
    write_executable(
        &bin_dir.join("gh"),
        &format!(
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\nTITLE_FILE='{}'\nLABELS_FILE='{}'\nBODY_FILE='{}'\nif [[ \"$1 $2\" == \"label list\" ]]; then\n  printf 'track:roadmap\\ntype:task\\narea:tools\\nversion:v0.87.1\\n'\n  exit 0\nfi\nif [[ \"$*\" == *\"issue create -R owner/repo --title [v0.87.1][tools] Metadata parity --body Seed body\"* ]]; then\n  printf 'https://github.com/owner/repo/issues/1153\\n'\n  exit 0\nfi\nif [[ \"$*\" == *\"issue view 1153 -R owner/repo --json title --jq .title\"* ]]; then\n  cat \"$TITLE_FILE\"\n  exit 0\nfi\nif [[ \"$*\" == *\"issue view 1153 -R owner/repo --json labels --jq .labels[].name\"* ]]; then\n  cat \"$LABELS_FILE\"\n  exit 0\nfi\nif [[ \"$*\" == *\"issue edit 1153 -R owner/repo --title [v0.87.1][tools] Metadata parity\"* ]]; then\n  printf '[v0.87.1][tools] Metadata parity\\n' > \"$TITLE_FILE\"\n  exit 0\nfi\nif [[ \"$*\" == *\"issue edit 1153 -R owner/repo\"* && \"$*\" == *\"--add-label area:tools,track:roadmap,type:task,version:v0.87.1\"* ]]; then\n  printf 'area:tools\\ntrack:roadmap\\ntype:task\\nversion:v0.87.1\\n' > \"$LABELS_FILE\"\n  exit 0\nfi\nif [[ \"$*\" == *\"issue edit 1153 -R owner/repo --body Refined issue body\"* ]]; then\n  printf 'Refined issue body' > \"$BODY_FILE\"\n  exit 0\nfi\nexit 1\n",
            gh_log.display(),
            title_state.display(),
            labels_state.display(),
            body_state.display()
        ),
    );

    let old_path = env::var("PATH").unwrap_or_default();
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
    }

    let created = gh_issue_create(
        "owner/repo",
        "[v0.87.1][tools] Metadata parity",
        "Seed body",
        "track:roadmap,type:task,area:tools,version:v0.87.1",
    )
    .expect("issue create");
    assert_eq!(created, "https://github.com/owner/repo/issues/1153");

    ensure_issue_metadata_parity(
        "owner/repo",
        1153,
        "[v0.87.1][tools] Metadata parity",
        "track:roadmap,type:task,area:tools,version:v0.87.1",
    )
    .expect("metadata parity");
    gh_issue_edit_body("owner/repo", 1153, "Refined issue body").expect("edit body");
    assert_eq!(
        gh_issue_title(1153, "owner/repo").expect("title"),
        Some("[v0.87.1][tools] Metadata parity".to_string())
    );

    unsafe {
        env::set_var("PATH", old_path);
    }

    let gh_log = fs::read_to_string(&gh_log).expect("gh log");
    assert!(gh_log.contains(
        "issue create -R owner/repo --title [v0.87.1][tools] Metadata parity --body Seed body"
    ));
    assert!(
        gh_log.contains("issue edit 1153 -R owner/repo --title [v0.87.1][tools] Metadata parity")
    );
    assert!(gh_log.contains("--add-label area:tools,track:roadmap,type:task,version:v0.87.1"));
    assert!(!gh_log.contains("--remove-label version:v0.86"));
    assert_eq!(
        fs::read_to_string(&body_state).expect("body state"),
        "Refined issue body"
    );
}

#[test]
fn github_closing_linkage_helpers_cover_body_fallback_and_repair_paths() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-closing-linkage-helpers");
    let bin_dir = temp.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let gh_log = temp.join("gh.log");
    let linkage_state = temp.join("closing-state.txt");
    fs::write(&linkage_state, "body_only\n").expect("seed linkage state");
    write_executable(
        &bin_dir.join("gh"),
        &format!(
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\nSTATE='{}'\nif [[ \"$*\" == *\"pr view -R owner/repo https://github.com/owner/repo/pull/1 --json closingIssuesReferences --jq .closingIssuesReferences[]?.number\"* ]]; then\n  if grep -q linked \"$STATE\"; then\n    printf '1153\\n'\n  fi\n  exit 0\nfi\nif [[ \"$*\" == *\"pr view -R owner/repo https://github.com/owner/repo/pull/1 --json body --jq .body\"* ]]; then\n  if grep -q body_only \"$STATE\"; then\n    printf 'Closes #1153\\n'\n  else\n    printf 'No close keyword present\\n'\n  fi\n  exit 0\nfi\nif [[ \"$*\" == *\"pr edit -R owner/repo https://github.com/owner/repo/pull/1 --body-file \"* ]]; then\n  printf 'linked\\n' > \"$STATE\"\n  exit 0\nfi\nexit 1\n",
            gh_log.display(),
            linkage_state.display()
        ),
    );

    let old_path = env::var("PATH").unwrap_or_default();
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
    }

    assert!(
        pr_has_closing_linkage("owner/repo", "https://github.com/owner/repo/pull/1", 1153)
            .expect("body fallback linkage")
    );
    ensure_pr_closing_linkage(
        "owner/repo",
        "https://github.com/owner/repo/pull/1",
        1153,
        false,
    )
    .expect("body fallback should satisfy close linkage");

    fs::write(&linkage_state, "repair_needed\n").expect("reset linkage state");
    let desired_body = temp.join("desired-body.md");
    fs::write(&desired_body, "Closes #1153\n").expect("desired body");
    assert!(ensure_or_repair_pr_closing_linkage(
        "owner/repo",
        "https://github.com/owner/repo/pull/1",
        1153,
        false,
        &desired_body,
    )
    .expect("repair linkage"));

    unsafe {
        env::set_var("PATH", old_path);
    }

    let gh_log = fs::read_to_string(&gh_log).expect("gh log");
    assert!(gh_log.contains("pr view -R owner/repo https://github.com/owner/repo/pull/1 --json closingIssuesReferences --jq .closingIssuesReferences[]?.number"));
    assert!(gh_log.contains(
        "pr view -R owner/repo https://github.com/owner/repo/pull/1 --json body --jq .body"
    ));
    assert!(
        gh_log.contains("pr edit -R owner/repo https://github.com/owner/repo/pull/1 --body-file")
    );
}
