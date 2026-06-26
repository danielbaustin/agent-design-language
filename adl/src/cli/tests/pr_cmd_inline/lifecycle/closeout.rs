use super::*;

#[test]
fn real_pr_closeout_reconciles_closed_completed_issue_bundle() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-closeout-success");
    let origin = temp.join("origin.git");
    let repo = temp.join("repo");
    fs::create_dir_all(&repo).expect("repo dir");
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
    fs::write(repo.join("README.md"), "closeout success\n").expect("seed file");
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
    let issue_ref = IssueRef::new(
        1596,
        "v0.87.1",
        "v0-87-1-tools-make-closeout-automatic-after-merge-closure",
    )
    .expect("issue ref");
    write_authored_issue_prompt(
        &repo,
        &issue_ref,
        "[v0.87.1][tools] Make closeout automatic after merge/closure",
    );
    {
        let _github_fixture = install_issue_label_fixture(&repo);
        real_pr(&[
            "init".to_string(),
            "1596".to_string(),
            "--slug".to_string(),
            issue_ref.slug().to_string(),
            "--title".to_string(),
            "[v0.87.1][tools] Make closeout automatic after merge/closure".to_string(),
            "--no-fetch-issue".to_string(),
            "--version".to_string(),
            "v0.87.1".to_string(),
        ])
        .expect("real_pr init");
    }

    let sip_path = issue_ref.task_bundle_input_path(&repo);
    write_authored_sip(
        &sip_path,
        &issue_ref,
        "[v0.87.1][tools] Make closeout automatic after merge/closure",
        "codex/1596-v0-87-1-tools-make-closeout-automatic-after-merge-closure",
        &issue_ref.issue_prompt_path(&repo),
        &repo,
    );
    write_completed_srp_fixture(
        &issue_ref.task_bundle_review_policy_path(&repo),
        &issue_ref,
        "[v0.87.1][tools] Make closeout automatic after merge/closure",
        "codex/1596-v0-87-1-tools-make-closeout-automatic-after-merge-closure",
        &repo,
    );
    let sor_path = issue_ref.task_bundle_output_path(&repo);
    write_completed_sor_fixture(
        &sor_path,
        "codex/1596-v0-87-1-tools-make-closeout-automatic-after-merge-closure",
    );

    let worktree = issue_ref.default_worktree_path(&repo, None);
    assert!(Command::new("git")
        .args([
            "worktree",
            "add",
            "-q",
            "-b",
            "codex/1596-v0-87-1-tools-make-closeout-automatic-after-merge-closure",
            path_str(&worktree).expect("worktree path"),
            "origin/main",
        ])
        .current_dir(&repo)
        .status()
        .expect("git worktree add")
        .success());
    assert!(worktree.is_dir(), "closeout fixture worktree should exist");

    let fixture_dir = temp.join("github-fixtures");
    fs::create_dir_all(&fixture_dir).expect("fixture dir");
    let github_cli_fixture = fixture_dir.join("closed-completed-1596");
    write_executable(
        &github_cli_fixture,
        "#!/usr/bin/env bash\nset -euo pipefail\nif [[ \"$1 $2 $3 $4\" == \"issue view 1596 -R\" ]]; then\n  echo '{\"state\":\"CLOSED\",\"stateReason\":\"COMPLETED\"}'\n  exit 0\nfi\nexit 1\n",
    );
    let _github_fixture = GithubCliFixtureGuard::set(&github_cli_fixture);

    let closeout = real_pr(&[
        "closeout".to_string(),
        "1596".to_string(),
        "--slug".to_string(),
        issue_ref.slug().to_string(),
        "--no-fetch-issue".to_string(),
        "--version".to_string(),
        "v0.87.1".to_string(),
    ]);

    env::set_current_dir(prev_dir).expect("restore cwd");
    closeout.expect("closeout closed reconcile");

    let canonical_text = fs::read_to_string(&sor_path).expect("read canonical sor");
    assert!(canonical_text.contains("Status: DONE"));
    assert!(canonical_text.contains("- Integration state: merged"));
    assert!(canonical_text.contains("- Verification scope: main_repo"));
    assert!(canonical_text.contains("- Worktree-only paths remaining: none"));
    assert!(canonical_text.contains("- Worktree prune result: pruned: adl-wp-1596"));
    assert!(!worktree.exists());
}

#[test]
fn real_pr_closeout_reconciles_closed_no_pr_issue_bundle() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-closeout-no-pr");
    let origin = temp.join("origin.git");
    let repo = temp.join("repo");
    fs::create_dir_all(&repo).expect("repo dir");
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
    fs::write(repo.join("README.md"), "closeout no-pr\n").expect("seed file");
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
    let issue_ref =
        IssueRef::new(1597, "v0.87.1", "v0-87-1-tools-closeout-no-pr-truth").expect("issue ref");
    write_authored_issue_prompt(&repo, &issue_ref, "[v0.87.1][tools] Closeout no-pr truth");
    {
        let _github_fixture = install_issue_label_fixture(&repo);
        real_pr(&[
            "init".to_string(),
            "1597".to_string(),
            "--slug".to_string(),
            issue_ref.slug().to_string(),
            "--title".to_string(),
            "[v0.87.1][tools] Closeout no-pr truth".to_string(),
            "--no-fetch-issue".to_string(),
            "--version".to_string(),
            "v0.87.1".to_string(),
        ])
        .expect("real_pr init");
    }

    let sip_path = issue_ref.task_bundle_input_path(&repo);
    write_authored_sip(
        &sip_path,
        &issue_ref,
        "[v0.87.1][tools] Closeout no-pr truth",
        "codex/1597-v0-87-1-tools-closeout-no-pr-truth",
        &issue_ref.issue_prompt_path(&repo),
        &repo,
    );
    write_completed_srp_fixture(
        &issue_ref.task_bundle_review_policy_path(&repo),
        &issue_ref,
        "[v0.87.1][tools] Closeout no-pr truth",
        "retrospective-no-branch",
        &repo,
    );
    let sor_path = issue_ref.task_bundle_output_path(&repo);
    write_completed_sor_fixture(&sor_path, "codex/1597-v0-87-1-tools-closeout-no-pr-truth");
    let sor_text = fs::read_to_string(&sor_path).expect("read sor fixture");
    let sor_text = sor_text
        .replace(
            "- Integration state: pr_open",
            "- Integration state: closed_no_pr",
        )
        .replace(
            "Branch: codex/1597-v0-87-1-tools-closeout-no-pr-truth",
            "Branch: retrospective-no-branch",
        );
    fs::write(&sor_path, sor_text).expect("write closed no-pr sor");

    let worktree = issue_ref.default_worktree_path(&repo, None);
    assert!(Command::new("git")
        .args([
            "worktree",
            "add",
            "-q",
            "-b",
            "codex/1597-v0-87-1-tools-closeout-no-pr-truth",
            path_str(&worktree).expect("worktree path"),
            "origin/main",
        ])
        .current_dir(&repo)
        .status()
        .expect("git worktree add")
        .success());
    assert!(
        worktree.is_dir(),
        "closeout no-pr fixture worktree should exist"
    );

    let fixture_dir = temp.join("github-fixtures");
    fs::create_dir_all(&fixture_dir).expect("fixture dir");
    let github_cli_fixture = fixture_dir.join("closed-completed-1597");
    write_executable(
        &github_cli_fixture,
        "#!/usr/bin/env bash\nset -euo pipefail\nif [[ \"$1 $2 $3 $4\" == \"issue view 1597 -R\" ]]; then\n  echo '{\"state\":\"CLOSED\",\"stateReason\":\"COMPLETED\"}'\n  exit 0\nfi\nexit 1\n",
    );
    let _github_fixture = GithubCliFixtureGuard::set(&github_cli_fixture);

    let closeout = real_pr(&[
        "closeout".to_string(),
        "1597".to_string(),
        "--slug".to_string(),
        issue_ref.slug().to_string(),
        "--no-fetch-issue".to_string(),
        "--version".to_string(),
        "v0.87.1".to_string(),
    ]);

    env::set_current_dir(prev_dir).expect("restore cwd");
    closeout.expect("closeout closed no-pr issue");

    let canonical_text = fs::read_to_string(&sor_path).expect("read canonical sor");
    assert!(canonical_text.contains("Status: DONE"));
    assert!(canonical_text.contains("Branch: retrospective-no-branch"));
    assert!(canonical_text.contains("- Integration state: closed_no_pr"));
    assert!(canonical_text.contains("- Verification scope: main_repo"));
    assert!(canonical_text.contains("- Worktree-only paths remaining: none"));
    assert!(!worktree.exists());
}

#[test]
fn real_pr_closeout_refuses_issue_that_is_not_completed() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-closeout-refuse");
    let origin = temp.join("origin.git");
    let repo = temp.join("repo");
    fs::create_dir_all(&repo).expect("repo dir");
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
    fs::write(repo.join("README.md"), "closeout refusal\n").expect("seed file");
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

    let prev_dir = env::current_dir().expect("cwd");
    env::set_current_dir(&repo).expect("chdir");
    let issue_ref = IssueRef::new(
        1596,
        "v0.87.1",
        "v0-87-1-tools-make-closeout-automatic-after-merge-closure",
    )
    .expect("issue ref");
    write_authored_issue_prompt(
        &repo,
        &issue_ref,
        "[v0.87.1][tools] Make closeout automatic after merge/closure",
    );
    {
        let _github_fixture = install_issue_label_fixture(&repo);
        real_pr(&[
            "init".to_string(),
            "1596".to_string(),
            "--slug".to_string(),
            issue_ref.slug().to_string(),
            "--title".to_string(),
            "[v0.87.1][tools] Make closeout automatic after merge/closure".to_string(),
            "--no-fetch-issue".to_string(),
            "--version".to_string(),
            "v0.87.1".to_string(),
        ])
        .expect("real_pr init");
    }

    let fixture_dir = temp.join("github-fixtures");
    fs::create_dir_all(&fixture_dir).expect("fixture dir");
    let github_cli_fixture = fixture_dir.join("open-reopened-1596");
    write_executable(
        &github_cli_fixture,
        "#!/usr/bin/env bash\nset -euo pipefail\nif [[ \"$1 $2 $3 $4\" == \"issue view 1596 -R\" ]]; then\n  echo '{\"state\":\"OPEN\",\"stateReason\":\"REOPENED\"}'\n  exit 0\nfi\nexit 1\n",
    );
    let _github_fixture = GithubCliFixtureGuard::set(&github_cli_fixture);

    let closeout = real_pr(&[
        "closeout".to_string(),
        "1596".to_string(),
        "--slug".to_string(),
        issue_ref.slug().to_string(),
        "--no-fetch-issue".to_string(),
        "--version".to_string(),
        "v0.87.1".to_string(),
    ]);

    env::set_current_dir(prev_dir).expect("restore cwd");

    let err = closeout.expect_err("closeout should refuse unfinished issue");
    assert!(err.to_string().contains("refusing automatic closeout"));
}

fn write_completed_srp_fixture(
    path: &Path,
    issue_ref: &IssueRef,
    title: &str,
    branch: &str,
    repo_root: &Path,
) {
    let stp_rel = path_relative_to_repo(repo_root, &issue_ref.task_bundle_stp_path(repo_root));
    let sip_rel = path_relative_to_repo(repo_root, &issue_ref.task_bundle_input_path(repo_root));
    let sor_rel = path_relative_to_repo(repo_root, &issue_ref.task_bundle_output_path(repo_root));
    let content = format!(
        r#"---
schema_version: "0.1"
artifact_type: "structured_review_prompt"
name: "{slug}-review-prompt"
issue: {issue}
task_id: "{task_id}"
version: v0.87.1
title: "{title}"
branch: "{branch}"
status: "approved"
source_refs:
  - kind: "issue"
    ref: "https://github.com/example/repo/issues/{issue}"
  - kind: "stp"
    ref: "{stp_rel}"
  - kind: "sip"
    ref: "{sip_rel}"
  - kind: "sor"
    ref: "{sor_rel}"
review_mode: "pre_pr_independent_review"
timing: "before_pr_open"
scope_basis:
  - "{stp_rel}"
  - "{sip_rel}"
in_scope_surfaces:
  - "tracked changes for this issue branch"
evidence_policy:
  - "Use repository evidence and issue-local validation only."
validation_inputs:
  - "Issue-local proofs recorded in the SOR."
allowed_dispositions:
  - "PASS"
  - "BLOCK"
  - "NEEDS_FOLLOWUP"
reviewer_constraints:
  - "Do not widen issue scope."
refusal_policy:
  - "Refuse claims that are unsupported by repository evidence."
follow_up_routing:
  - "Route actionable findings back to the issue branch."
non_claims:
  - "This prompt does not guarantee review quality by itself."
policy_refs:
  - "{stp_rel}"
review_results:
  findings_status: "no_findings"
  recommended_outcome: "pass"
notes: "fixture review completed"
---

# Structured Review Prompt

## Review Summary

fixture review completed

## Scope Basis

- {stp_rel}
- {sip_rel}

## In-Scope Surfaces

- tracked changes for this issue branch

## Evidence Rules

- Use repository evidence and issue-local validation only.

## Validation Inputs

- Issue-local proofs recorded in the SOR.

## Allowed Dispositions

- PASS
- BLOCK
- NEEDS_FOLLOWUP

## Reviewer Constraints

- Do not widen issue scope.

## Refusal Policy

- Refuse claims that are unsupported by repository evidence.

## Follow-up Routing

- Route actionable findings back to the issue branch.

## Non-Claims

- This prompt does not guarantee review quality by itself.

## Review Results

### Findings

- No findings.

### Dispositions

- None.

### Recommended Outcome

- PASS

## Notes

fixture review completed
"#,
        slug = issue_ref.slug(),
        issue = issue_ref.issue_number(),
        task_id = issue_ref.task_issue_id(),
        title = title,
        branch = branch,
        stp_rel = stp_rel,
        sip_rel = sip_rel,
        sor_rel = sor_rel,
    );
    fs::write(path, content).expect("write completed srp");
}
