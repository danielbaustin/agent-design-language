use super::super::{path_str, IssueRef};
use super::*;
use crate::cli::tests::env_lock;
use std::env;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn temp_dir(prefix: &str) -> PathBuf {
    let mut path = env::temp_dir();
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    path.push(format!("{prefix}-{}-{nanos}", std::process::id()));
    fs::create_dir_all(&path).expect("create temp dir");
    path
}

fn write_executable(path: &Path, body: &str) {
    fs::write(path, body).expect("write executable");
    let mut perms = fs::metadata(path).expect("metadata").permissions();
    perms.set_mode(0o755);
    fs::set_permissions(path, perms).expect("chmod");
}

fn init_repo_with_origin(repo: &Path, origin: &Path) {
    fs::create_dir_all(repo).expect("repo dir");
    assert!(Command::new("git")
        .args(["init", "-q"])
        .current_dir(repo)
        .status()
        .expect("git init")
        .success());
    assert!(Command::new("git")
        .args([
            "remote",
            "add",
            "origin",
            path_str(origin).expect("origin path")
        ])
        .current_dir(repo)
        .status()
        .expect("git remote add")
        .success());
    assert!(Command::new("git")
        .args(["config", "user.name", "Test User"])
        .current_dir(repo)
        .status()
        .expect("git config name")
        .success());
    assert!(Command::new("git")
        .args(["config", "user.email", "test@example.com"])
        .current_dir(repo)
        .status()
        .expect("git config email")
        .success());
    fs::write(repo.join("README.md"), "seed\n").expect("seed readme");
    assert!(Command::new("git")
        .args(["add", "-A"])
        .current_dir(repo)
        .status()
        .expect("git add")
        .success());
    assert!(Command::new("git")
        .args(["commit", "-q", "-m", "init"])
        .current_dir(repo)
        .status()
        .expect("git commit")
        .success());
    assert!(Command::new("git")
        .args(["branch", "-M", "main"])
        .current_dir(repo)
        .status()
        .expect("git branch")
        .success());
    assert!(Command::new("git")
        .args([
            "init",
            "--bare",
            "-q",
            path_str(origin).expect("origin path")
        ])
        .current_dir(repo)
        .status()
        .expect("git init bare")
        .success());
    assert!(Command::new("git")
        .args([
            "remote",
            "set-url",
            "origin",
            path_str(origin).expect("origin path"),
        ])
        .current_dir(repo)
        .status()
        .expect("git remote set-url")
        .success());
    assert!(Command::new("git")
        .args(["push", "-q", "-u", "origin", "main"])
        .current_dir(repo)
        .status()
        .expect("git push")
        .success());
}

#[test]
fn issue_is_closed_and_completed_parses_completed_state() {
    let _guard = env_lock();
    let temp = temp_dir("adl-pr-lifecycle-gh");
    let bin_dir = temp.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    write_executable(
            &bin_dir.join("gh"),
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf '{\"state\":\"CLOSED\",\"stateReason\":\"COMPLETED\"}\\n'\n",
        );
    let old_path = env::var("PATH").unwrap_or_default();
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
    }

    let result = issue_is_closed_and_completed(1410, "owner/repo").expect("completed state");

    unsafe {
        env::set_var("PATH", old_path);
    }
    assert!(result);
}

#[test]
fn issue_is_closed_and_completed_returns_false_for_empty_or_open_state() {
    let _guard = env_lock();
    let temp = temp_dir("adl-pr-lifecycle-gh-open");
    let bin_dir = temp.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    write_executable(
            &bin_dir.join("gh"),
            "#!/usr/bin/env bash\nset -euo pipefail\nif [[ \"${1:-}\" == \"issue\" ]]; then\n  printf '{\"state\":\"OPEN\",\"stateReason\":null}\\n'\nfi\n",
        );
    let old_path = env::var("PATH").unwrap_or_default();
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
    }

    let result = issue_is_closed_and_completed(1410, "owner/repo").expect("open state");

    unsafe {
        env::set_var("PATH", old_path);
    }
    assert!(!result);
}

#[test]
fn ensure_issue_closed_completed_for_closeout_rejects_unfinished_issue() {
    let _guard = env_lock();
    let temp = temp_dir("adl-pr-lifecycle-closeout-guard");
    let bin_dir = temp.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    write_executable(
            &bin_dir.join("gh"),
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf '{\"state\":\"CLOSED\",\"stateReason\":\"NOT_PLANNED\"}\\n'\n",
        );
    let old_path = env::var("PATH").unwrap_or_default();
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
    }

    let err = ensure_issue_closed_completed_for_closeout(1410, "owner/repo")
        .expect_err("should reject unfinished closeout");

    unsafe {
        env::set_var("PATH", old_path);
    }
    assert!(err
        .to_string()
        .contains("is not closed with COMPLETED state yet"));
}

#[test]
fn wait_for_issue_closed_and_completed_succeeds_after_retry() {
    let _guard = env_lock();
    let temp = temp_dir("adl-pr-lifecycle-closeout-wait");
    let bin_dir = temp.join("bin");
    let counter = temp.join("counter.txt");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    write_executable(
            &bin_dir.join("gh"),
            &format!(
                "#!/usr/bin/env bash\nset -euo pipefail\ncounter=\"{}\"\ncount=0\nif [[ -f \"$counter\" ]]; then\n  count=$(cat \"$counter\")\nfi\ncount=$((count + 1))\nprintf '%s' \"$count\" > \"$counter\"\nif [[ \"$count\" -lt 2 ]]; then\n  printf '{{\"state\":\"OPEN\",\"stateReason\":null}}\\n'\nelse\n  printf '{{\"state\":\"CLOSED\",\"stateReason\":\"COMPLETED\"}}\\n'\nfi\n",
                counter.display()
            ),
        );
    let old_path = env::var("PATH").unwrap_or_default();
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
    }

    wait_for_issue_closed_and_completed(1410, "owner/repo").expect("wait succeeds");

    unsafe {
        env::set_var("PATH", old_path);
    }
    assert_eq!(fs::read_to_string(&counter).expect("counter"), "2");
}

#[test]
fn matching_task_bundle_dirs_returns_sorted_prefix_matches() {
    let repo = temp_dir("adl-pr-lifecycle-bundles");
    let issue_ref = IssueRef::new(1410, "v0.87", "canonical-slug").expect("issue ref");
    let tasks_dir = repo.join(".adl").join("v0.87").join("tasks");
    fs::create_dir_all(tasks_dir.join("issue-1410__z-slug")).expect("dir 1");
    fs::create_dir_all(tasks_dir.join("issue-1410__a-slug")).expect("dir 2");
    fs::create_dir_all(tasks_dir.join("issue-999__other")).expect("dir 3");

    let matches = matching_task_bundle_dirs(&repo, &issue_ref).expect("matches");
    let names = matches
        .iter()
        .map(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .unwrap()
                .to_string()
        })
        .collect::<Vec<_>>();

    assert_eq!(names, vec!["issue-1410__a-slug", "issue-1410__z-slug"]);
}

#[test]
fn normalize_closed_completed_output_card_rewrites_status_and_integration_fields() {
    let temp = temp_dir("adl-pr-lifecycle-output");
    let output = temp.join("sor.md");
    fs::write(
            &output,
            "Status: IN_PROGRESS\n- Integration state: worktree_only\n- Verification scope: worktree\n- Worktree-only paths remaining: adl/src/foo.rs\n",
        )
        .expect("write output");

    normalize_closed_completed_output_card(&output).expect("normalize");
    let text = fs::read_to_string(&output).expect("read output");

    assert!(text.contains("Status: DONE"));
    assert!(text.contains("- Integration state: merged"));
    assert!(text.contains("- Verification scope: main_repo"));
    assert!(text.contains("- Worktree-only paths remaining: none"));
}

#[test]
fn normalize_closed_completed_output_card_rewrites_no_direct_pr_truth() {
    let temp = temp_dir("adl-pr-lifecycle-output-no-pr");
    let output = temp.join("sor.md");
    fs::write(
            &output,
            "Status: IN_PROGRESS\nBranch: retrospective-no-branch\n- Integration state: worktree_only\n- Verification scope: worktree\n- Worktree-only paths remaining: adl/src/foo.rs\n",
        )
        .expect("write output");

    normalize_closed_completed_output_card(&output).expect("normalize");
    let text = fs::read_to_string(&output).expect("read output");

    assert!(text.contains("Status: DONE"));
    assert!(text.contains("Branch: retrospective-no-branch"));
    assert!(text.contains("- Integration state: closed_no_pr"));
    assert!(text.contains("- Verification scope: main_repo"));
    assert!(text.contains("- Worktree-only paths remaining: none"));
}

#[test]
fn normalize_closed_completed_stp_marks_issue_complete() {
    let temp = temp_dir("adl-pr-lifecycle-stp");
    let stp = temp.join("stp.md");
    fs::write(
        &stp,
        "---\nstatus: \"draft\"\naction: \"edit\"\n---\n\n# Example\n",
    )
    .expect("write stp");

    normalize_closed_completed_stp(&stp).expect("normalize stp");
    let text = fs::read_to_string(&stp).expect("read stp");

    assert!(text.contains("status: \"complete\""));
    assert!(text.contains("action: \"edit\""));
}

#[test]
fn normalize_closed_completed_sip_rewrites_pre_run_lifecycle_truth() {
    let temp = temp_dir("adl-pr-lifecycle-sip");
    let issue_ref = IssueRef::new(1410, "v0.87", "canonical-slug").expect("issue ref");
    let sip = temp.join("sip.md");
    fs::write(
            &sip,
            "# ADL Input Card\n\nTask ID: issue-1410\nRun ID: issue-1410\nVersion: v0.87\nTitle: Example\nBranch: not bound yet\n\n## Agent Execution Rules\n- This issue is not started yet; do not assume a branch or worktree already exists.\n- Do not run `pr start`; use the current issue-mode `pr run` flow only if execution later becomes necessary.\n\n## Goal\n\nPrepare the linked issue prompt and review surfaces for truthful pre-run review before execution is bound.\n\n## Required Outcome\n\n- Preserve truthful lifecycle state until `pr run` binds the branch and worktree.\n\n## Acceptance Criteria\n\n- The card bundle does not imply a branch or worktree exists before `pr run`.\n",
        )
        .expect("write sip");

    normalize_closed_completed_sip(&sip, &issue_ref).expect("normalize sip");
    let text = fs::read_to_string(&sip).expect("read sip");

    assert!(text.contains("Branch: codex/1410-canonical-slug"));
    assert!(!text.contains("- PR: none"));
    assert!(text.contains("closed/completed"));
    assert!(!text.contains("This issue is not started yet"));
    assert!(!text.contains("before execution is bound"));
    assert!(!text.contains("until `pr run` binds the branch and worktree"));
}

#[test]
fn ensure_canonical_output_is_local_only_rejects_tracked_canonical_output() {
    let _guard = env_lock();
    let temp = temp_dir("adl-pr-lifecycle-local-only");
    let repo = temp.join("repo");
    let origin = temp.join("origin.git");
    init_repo_with_origin(&repo, &origin);
    let issue_ref = IssueRef::new(1410, "v0.87", "canonical-slug").expect("issue ref");
    let output = issue_ref.task_bundle_output_path(&repo);
    fs::create_dir_all(output.parent().expect("output parent")).expect("create bundle dir");
    fs::write(
            &output,
            "Status: DONE\n- Integration state: merged\n- Verification scope: main_repo\n- Worktree-only paths remaining: none\n",
        )
        .expect("write tracked output");
    assert!(Command::new("git")
        .args(["add", path_str(&output).expect("output path")])
        .current_dir(&repo)
        .status()
        .expect("git add output")
        .success());

    let err = ensure_canonical_output_is_local_only(
        &repo,
        &output,
        "finish: canonical .adl output surfaces must remain local-only during output sync",
    )
    .expect_err("tracked canonical output should be rejected");

    assert!(err
        .to_string()
        .contains("canonical .adl output surfaces must remain local-only"));
    assert!(err
        .to_string()
        .contains(".adl/v0.87/tasks/issue-1410__canonical-slug/sor.md"));
}

#[test]
fn ensure_closed_completed_issue_bundle_truth_rejects_stale_fields() {
    let temp = temp_dir("adl-pr-lifecycle-truth-drift");
    let issue_ref = IssueRef::new(1410, "v0.87", "canonical-slug").expect("issue ref");
    let canonical_dir = issue_ref.task_bundle_dir_path(&temp);
    let duplicate_dir = temp
        .join(".adl")
        .join("v0.87")
        .join("tasks")
        .join("issue-1410__legacy-slug");
    fs::create_dir_all(&canonical_dir).expect("canonical dir");
    fs::create_dir_all(&duplicate_dir).expect("duplicate dir");
    let output = canonical_dir.join("sor.md");
    let stp = canonical_dir.join("stp.md");
    let sip = canonical_dir.join("sip.md");
    fs::write(
        &stp,
        "---\nstatus: \"draft\"\naction: \"edit\"\n---\n\n# Example\n",
    )
    .expect("write stale stp");
    fs::write(
            &sip,
            "# ADL Input Card\n\nBranch: not bound yet\n\n## Goal\n\nPrepare the linked issue prompt and review surfaces for truthful pre-run review before execution is bound.\n",
        )
        .expect("write stale sip");
    fs::write(
            &output,
            "Status: IN_PROGRESS\n- Integration state: pr_open\n- Verification scope: worktree\n- Worktree-only paths remaining: adl/src/foo.rs\n",
        )
        .expect("write stale output");

    let err = ensure_closed_completed_issue_bundle_truth(&temp, &issue_ref, &output)
        .expect_err("stale truth should fail");
    let rendered = err.to_string();
    assert!(rendered.contains("canonical closed-issue sor truth drift"));
    assert!(rendered.contains("SOR Status expected 'DONE' but found 'IN_PROGRESS'"));
    assert!(rendered
        .contains("SOR Integration state expected 'merged' or 'closed_no_pr' but found 'pr_open'"));
    assert!(rendered.contains("SOR Verification scope expected 'main_repo' but found 'worktree'"));
    assert!(rendered
        .contains("SOR Worktree-only paths remaining expected 'none' but found 'adl/src/foo.rs'"));
    assert!(rendered.contains("STP status expected '\"complete\"' but found '\"draft\"'"));
    assert!(rendered.contains("SIP Branch expected 'codex/1410-canonical-slug'"));
    assert!(rendered.contains("SIP still contains pre-run lifecycle wording"));
}

#[test]
fn ensure_closed_completed_issue_bundle_truth_rejects_merged_with_retrospective_branch() {
    let temp = temp_dir("adl-pr-lifecycle-truth-no-pr");
    let issue_ref = IssueRef::new(1410, "v0.87", "canonical-slug").expect("issue ref");
    let canonical_dir = issue_ref.task_bundle_dir_path(&temp);
    fs::create_dir_all(&canonical_dir).expect("canonical dir");
    fs::write(
        canonical_dir.join("stp.md"),
        "---\nstatus: \"complete\"\naction: \"edit\"\n---\n\n# Example\n",
    )
    .expect("write normalized stp");
    fs::write(
            canonical_dir.join("sip.md"),
            "# ADL Input Card\n\nBranch: retrospective-no-branch\n\n## Goal\n\nPreserve the closed/completed issue prompt and local card truth after closeout.\n",
        )
        .expect("write normalized sip");
    let output = canonical_dir.join("sor.md");
    fs::write(
            &output,
            "Status: DONE\nBranch: retrospective-no-branch\n- Integration state: merged\n- Verification scope: main_repo\n- Worktree-only paths remaining: none\n",
        )
        .expect("write stale output");

    let err = ensure_closed_completed_issue_bundle_truth(&temp, &issue_ref, &output)
        .expect_err("merged with retrospective branch should fail");
    assert!(err
            .to_string()
            .contains(
                "SOR Integration state is 'merged' but Branch is 'retrospective-no-branch'; use 'closed_no_pr'"
            ));
}

#[test]
fn ensure_closed_completed_issue_bundle_truth_accepts_normalized_bundle() {
    let temp = temp_dir("adl-pr-lifecycle-truth-clean");
    let issue_ref = IssueRef::new(1410, "v0.87", "canonical-slug").expect("issue ref");
    let canonical_dir = issue_ref.task_bundle_dir_path(&temp);
    fs::create_dir_all(&canonical_dir).expect("canonical dir");
    fs::write(
        canonical_dir.join("stp.md"),
        "---\nstatus: \"complete\"\naction: \"edit\"\n---\n\n# Example\n",
    )
    .expect("write normalized stp");
    fs::write(
            canonical_dir.join("sip.md"),
            "# ADL Input Card\n\nBranch: codex/1410-canonical-slug\n\n## Goal\n\nPreserve the closed/completed issue prompt and local card truth after closeout.\n",
        )
        .expect("write normalized sip");
    let output = canonical_dir.join("sor.md");
    fs::write(
            &output,
            "Status: DONE\n- Integration state: merged\n- Verification scope: main_repo\n- Worktree-only paths remaining: none\n",
        )
        .expect("write normalized output");

    ensure_closed_completed_issue_bundle_truth(&temp, &issue_ref, &output)
        .expect("normalized truth should pass");
}

#[test]
fn ensure_closed_completed_issue_bundle_truth_accepts_retrospective_no_branch_closed_no_pr() {
    let temp = temp_dir("adl-pr-lifecycle-truth-no-pr-clean");
    let issue_ref = IssueRef::new(1410, "v0.87", "canonical-slug").expect("issue ref");
    let canonical_dir = issue_ref.task_bundle_dir_path(&temp);
    fs::create_dir_all(&canonical_dir).expect("canonical dir");
    fs::write(
        canonical_dir.join("stp.md"),
        "---\nstatus: \"complete\"\naction: \"edit\"\n---\n\n# Example\n",
    )
    .expect("write normalized stp");
    fs::write(
            canonical_dir.join("sip.md"),
            "# ADL Input Card\n\nBranch: retrospective-no-branch\n\n## Goal\n\nPreserve the closed/completed issue prompt and local card truth after closeout.\n",
        )
        .expect("write normalized sip");
    let output = canonical_dir.join("sor.md");
    fs::write(
            &output,
            "Status: DONE\nBranch: retrospective-no-branch\n- Integration state: closed_no_pr\n- Verification scope: main_repo\n- Worktree-only paths remaining: none\n",
        )
        .expect("write normalized output");

    ensure_closed_completed_issue_bundle_truth(&temp, &issue_ref, &output)
        .expect("normalized no-pr truth should pass");
}

#[test]
fn ensure_closed_completed_issue_bundle_truth_accepts_normalized_bundle_with_duplicate() {
    let temp = temp_dir("adl-pr-lifecycle-truth-clean-duplicate");
    let issue_ref = IssueRef::new(1410, "v0.87", "canonical-slug").expect("issue ref");
    let canonical_dir = issue_ref.task_bundle_dir_path(&temp);
    let duplicate_dir = temp
        .join(".adl")
        .join("v0.87")
        .join("tasks")
        .join("issue-1410__legacy-slug");
    fs::create_dir_all(&canonical_dir).expect("canonical dir");
    fs::create_dir_all(&duplicate_dir).expect("duplicate dir");
    for dir in [&canonical_dir, &duplicate_dir] {
        fs::write(
            dir.join("stp.md"),
            "---\nstatus: \"complete\"\naction: \"edit\"\n---\n\n# Example\n",
        )
        .expect("write normalized stp");
        fs::write(
                dir.join("sip.md"),
                "# ADL Input Card\n\nBranch: codex/1410-canonical-slug\n\n## Goal\n\nPreserve the closed/completed issue prompt and local card truth after closeout.\n",
            )
            .expect("write normalized sip");
    }
    let output = canonical_dir.join("sor.md");
    fs::write(
            &output,
            "Status: DONE\n- Integration state: merged\n- Verification scope: main_repo\n- Worktree-only paths remaining: none\n",
        )
        .expect("write normalized output");
    fs::write(
            duplicate_dir.join("sor.md"),
            "Status: DONE\n- Integration state: merged\n- Verification scope: main_repo\n- Worktree-only paths remaining: none\n",
        )
        .expect("write duplicate output");

    ensure_closed_completed_issue_bundle_truth(&temp, &issue_ref, &output)
        .expect("normalized truth should pass with preserved duplicate");
}

#[test]
fn same_filesystem_target_detects_equivalent_paths() {
    let temp = temp_dir("adl-pr-lifecycle-same-target");
    let left = temp.join("left.txt");
    let right = temp.join("right.txt");
    fs::write(&left, "hello\n").expect("write left");
    std::os::unix::fs::symlink(&left, &right).expect("symlink");

    assert!(same_filesystem_target(&left, &left).expect("same path"));
    assert!(same_filesystem_target(&left, &right).expect("same target"));
    assert!(!same_filesystem_target(&left, &temp.join("missing.txt")).expect("missing"));
}

#[test]
fn prune_issue_worktree_noops_when_worktree_is_missing() {
    let _guard = env_lock();
    let temp = temp_dir("adl-pr-lifecycle-prune-missing");
    let repo = temp.join("repo");
    let origin = temp.join("origin.git");
    init_repo_with_origin(&repo, &origin);
    let issue_ref = IssueRef::new(1410, "v0.87", "canonical-slug").expect("issue ref");

    let result = prune_issue_worktree(&repo, &repo, &issue_ref).expect("missing worktree is fine");
    assert_eq!(
        result,
        IssueWorktreePruneResult::Missing("adl-wp-1410".to_string())
    );
}

#[test]
fn prune_issue_worktree_rejects_dirty_worktree() {
    let _guard = env_lock();
    let temp = temp_dir("adl-pr-lifecycle-prune-dirty");
    let repo = temp.join("repo");
    let origin = temp.join("origin.git");
    init_repo_with_origin(&repo, &origin);
    let issue_ref = IssueRef::new(1410, "v0.87", "canonical-slug").expect("issue ref");
    let worktree = issue_ref.default_worktree_path(&repo, None);

    assert!(Command::new("git")
        .args([
            "-C",
            path_str(&repo).expect("repo path"),
            "worktree",
            "add",
            path_str(&worktree).expect("worktree path"),
            "-b",
            "codex/1410-canonical-slug",
            "main",
        ])
        .status()
        .expect("git worktree add")
        .success());
    fs::write(worktree.join("DIRTY.txt"), "dirty\n").expect("dirty file");

    prune_issue_worktree(&repo, &repo, &issue_ref).expect_err("dirty worktree rejected");
    assert!(worktree.is_dir());
}

#[test]
fn scrub_noncanonical_issue_bundle_residue_keeps_only_canonical_issue_bundle() {
    let _guard = env_lock();
    let temp = temp_dir("adl-pr-lifecycle-scrub-foreign-bundles");
    let repo = temp.join("repo");
    let origin = temp.join("origin.git");
    init_repo_with_origin(&repo, &origin);
    let issue_ref = IssueRef::new(1410, "v0.87", "canonical-slug").expect("issue ref");
    let foreign_ref = IssueRef::new(1411, "v0.87", "foreign-slug").expect("foreign issue ref");
    let drift_ref = IssueRef::new(1410, "v0.87", "stale-drift-slug").expect("same issue drift ref");
    let worktree = issue_ref.default_worktree_path(&repo, None);
    fs::create_dir_all(worktree.join(".adl").join("v0.87").join("bodies")).expect("bodies");
    fs::create_dir_all(worktree.join(".adl").join("v0.87").join("tasks")).expect("tasks");

    let canonical_body = issue_ref.issue_prompt_path(&worktree);
    let canonical_bundle = issue_ref.task_bundle_dir_path(&worktree);
    fs::create_dir_all(canonical_bundle.parent().expect("canonical bundle parent"))
        .expect("canonical bundle parent mkdir");
    fs::create_dir_all(&canonical_bundle).expect("canonical bundle");
    fs::write(&canonical_body, "canonical body\n").expect("canonical body");
    fs::write(canonical_bundle.join("stp.md"), "canonical stp\n").expect("canonical stp");

    let foreign_body = foreign_ref.issue_prompt_path(&worktree);
    let foreign_bundle = foreign_ref.task_bundle_dir_path(&worktree);
    fs::create_dir_all(foreign_bundle.parent().expect("foreign bundle parent"))
        .expect("foreign bundle parent mkdir");
    fs::create_dir_all(&foreign_bundle).expect("foreign bundle");
    fs::write(&foreign_body, "foreign body\n").expect("foreign body");
    fs::write(foreign_bundle.join("stp.md"), "foreign stp\n").expect("foreign stp");

    let drift_body = drift_ref.issue_prompt_path(&worktree);
    let drift_bundle = drift_ref.task_bundle_dir_path(&worktree);
    fs::create_dir_all(drift_bundle.parent().expect("drift bundle parent"))
        .expect("drift bundle parent mkdir");
    fs::create_dir_all(&drift_bundle).expect("drift bundle");
    fs::write(&drift_body, "drift body\n").expect("drift body");
    fs::write(drift_bundle.join("stp.md"), "drift stp\n").expect("drift stp");

    scrub_noncanonical_issue_bundle_residue(&worktree, &issue_ref).expect("scrub");

    assert!(canonical_body.is_file());
    assert!(canonical_bundle.is_dir());
    assert!(!foreign_body.exists());
    assert!(!foreign_bundle.exists());
    assert!(!drift_body.exists());
    assert!(!drift_bundle.exists());
}

#[test]
fn prune_issue_worktree_removes_clean_issue_worktree() {
    let _guard = env_lock();
    let temp = temp_dir("adl-pr-lifecycle-prune-clean");
    let repo = temp.join("repo");
    let origin = temp.join("origin.git");
    init_repo_with_origin(&repo, &origin);
    let issue_ref = IssueRef::new(1410, "v0.87", "canonical-slug").expect("issue ref");
    let worktree = issue_ref.default_worktree_path(&repo, None);

    assert!(Command::new("git")
        .args([
            "-C",
            path_str(&repo).expect("repo path"),
            "worktree",
            "add",
            path_str(&worktree).expect("worktree path"),
            "-b",
            "codex/1410-canonical-slug",
            "main",
        ])
        .status()
        .expect("git worktree add")
        .success());
    assert!(worktree.is_dir());

    let result = prune_issue_worktree(&repo, &repo, &issue_ref).expect("clean worktree pruned");
    assert_eq!(
        result,
        IssueWorktreePruneResult::Pruned("adl-wp-1410".to_string())
    );
    assert!(!worktree.exists());
}
