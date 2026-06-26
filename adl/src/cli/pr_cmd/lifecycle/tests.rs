use super::super::{path_str, IssueRef};
use super::*;
use crate::cli::pr_cmd::lifecycle::cleanup::{
    record_worktree_prune_result, replace_worktree_only_paths_remaining,
};
use crate::cli::pr_cmd::{card_output_path, resolve_cards_root};
use crate::cli::pr_cmd_cards::{ensure_pre_run_bootstrap_cards, ensure_task_bundle_stp};
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

struct EnvVarGuard {
    key: String,
    old: Option<std::ffi::OsString>,
}

impl EnvVarGuard {
    fn set(key: &str, value: &str) -> Self {
        let old = env::var_os(key);
        unsafe {
            env::set_var(key, value);
        }
        Self {
            key: key.to_string(),
            old,
        }
    }
}

impl Drop for EnvVarGuard {
    fn drop(&mut self) {
        unsafe {
            match &self.old {
                Some(value) => env::set_var(&self.key, value),
                None => env::remove_var(&self.key),
            }
        }
    }
}

fn ensure_validate_structured_prompt_script(repo_root: &Path) {
    let validator = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tools")
        .join("validate_structured_prompt.sh");
    let destination_parent = repo_root.join("adl").join("tools");
    let destination = destination_parent.join("validate_structured_prompt.sh");
    if destination.exists() {
        return;
    }

    fs::create_dir_all(&destination_parent).expect("create tools dir");
    fs::copy(&validator, &destination).expect("copy validator script");
    let mut perms = fs::metadata(&destination)
        .expect("metadata validator")
        .permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&destination, perms).expect("chmod validator");
}

fn copy_prompt_templates(repo_root: &Path) {
    fn copy_dir(src: &Path, dst: &Path) {
        fs::create_dir_all(dst).expect("create prompt template dir");
        for entry in fs::read_dir(src).expect("read prompt template dir") {
            let entry = entry.expect("prompt template entry");
            let src_path = entry.path();
            let dst_path = dst.join(entry.file_name());
            if src_path.is_dir() {
                copy_dir(&src_path, &dst_path);
            } else {
                fs::copy(&src_path, &dst_path).expect("copy prompt template file");
            }
        }
    }

    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("manifest parent");
    copy_dir(
        &workspace_root.join("docs/templates/prompts"),
        &repo_root.join("docs/templates/prompts"),
    );
}

fn set_tooling_manifest_root_to_workspace() -> EnvVarGuard {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("manifest parent");
    EnvVarGuard::set(
        "ADL_TOOLING_MANIFEST_ROOT",
        workspace_root.to_string_lossy().as_ref(),
    )
}

fn write_executable(path: &Path, body: &str) {
    let body = if path.file_name().and_then(|name| name.to_str()) == Some("gh")
        && !body.contains("ADL_GITHUB_TEST_FIXTURE")
    {
        body.replacen(
            "#!/usr/bin/env bash\n",
            "#!/usr/bin/env bash\n# ADL_GITHUB_TEST_FIXTURE\n",
            1,
        )
    } else {
        body.to_string()
    };
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
    ensure_validate_structured_prompt_script(repo);
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
    let github_cli_fixture = bin_dir.join("github-cli-fixture");
    write_executable(
        &github_cli_fixture,
        "#!/usr/bin/env bash\nset -euo pipefail\nprintf '{\"state\":\"CLOSED\",\"stateReason\":\"COMPLETED\"}\\n'\n",
    );
    let old_path = env::var("PATH").unwrap_or_default();
    let _fixture_guard = EnvVarGuard::set(
        "ADL_TEST_GITHUB_CLI_FIXTURE",
        github_cli_fixture.to_string_lossy().as_ref(),
    );
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
    let github_cli_fixture = bin_dir.join("github-cli-fixture");
    write_executable(
        &github_cli_fixture,
        "#!/usr/bin/env bash\nset -euo pipefail\nif [[ \"${1:-}\" == \"issue\" ]]; then\n  printf '{\"state\":\"OPEN\",\"stateReason\":null}\\n'\nfi\n",
    );
    let old_path = env::var("PATH").unwrap_or_default();
    let _fixture_guard = EnvVarGuard::set(
        "ADL_TEST_GITHUB_CLI_FIXTURE",
        github_cli_fixture.to_string_lossy().as_ref(),
    );
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
    let github_cli_fixture = bin_dir.join("github-cli-fixture");
    write_executable(
        &github_cli_fixture,
        "#!/usr/bin/env bash\nset -euo pipefail\nprintf '{\"state\":\"CLOSED\",\"stateReason\":\"NOT_PLANNED\"}\\n'\n",
    );
    let old_path = env::var("PATH").unwrap_or_default();
    let _fixture_guard = EnvVarGuard::set(
        "ADL_TEST_GITHUB_CLI_FIXTURE",
        github_cli_fixture.to_string_lossy().as_ref(),
    );
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
    let github_cli_fixture = bin_dir.join("github-cli-fixture");
    write_executable(
        &github_cli_fixture,
        &format!(
            "#!/usr/bin/env bash\nset -euo pipefail\ncounter=\"{}\"\ncount=0\nif [[ -f \"$counter\" ]]; then\n  count=$(cat \"$counter\")\nfi\ncount=$((count + 1))\nprintf '%s' \"$count\" > \"$counter\"\nif [[ \"$count\" -lt 2 ]]; then\n  printf '{{\"state\":\"OPEN\",\"stateReason\":null}}\\n'\nelse\n  printf '{{\"state\":\"CLOSED\",\"stateReason\":\"COMPLETED\"}}\\n'\nfi\n",
            counter.display()
        ),
    );
    let old_path = env::var("PATH").unwrap_or_default();
    let _fixture_guard = EnvVarGuard::set(
        "ADL_TEST_GITHUB_CLI_FIXTURE",
        github_cli_fixture.to_string_lossy().as_ref(),
    );
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
    fs::write(
        canonical_dir.join("srp.md"),
        "---\nschema_version: \"0.1\"\nartifact_type: \"structured_review_prompt\"\nname: \"fixture-review\"\nissue: 1410\ntask_id: \"issue-1410\"\nversion: v0.87\ntitle: \"Fixture\"\nbranch: \"codex/1410-canonical-slug\"\nstatus: \"draft\"\nreview_results:\n  findings_status: \"not_run\"\n  recommended_outcome: \"not_run\"\n---\n\n# Structured Review Prompt\n\n## Review Summary\n\nPre-review scaffold.\n\n## Scope Basis\n\n- fixture\n\n## In-Scope Surfaces\n\n- tracked changes\n\n## Evidence Rules\n\n- repo only\n\n## Validation Inputs\n\n- issue local\n\n## Allowed Dispositions\n\n- PASS\n- BLOCK\n\n## Reviewer Constraints\n\n- keep scope narrow\n\n## Refusal Policy\n\n- refuse unsupported claims\n\n## Follow-up Routing\n\n- route back to issue\n\n## Non-Claims\n\n- not final review\n\n## Notes\n\nPre-review scaffold.\n",
    )
    .expect("write stale srp");

    let err = ensure_closed_completed_issue_bundle_truth(&temp, &issue_ref, &output)
        .expect_err("stale truth should fail");
    let rendered = err.to_string();
    assert!(rendered.contains("canonical closed-issue sor truth drift"));
    assert!(rendered.contains("SOR Status expected 'DONE' but found 'IN_PROGRESS'"));
    assert!(rendered
        .contains("SOR Integration state expected 'merged' or 'closed_no_pr' but found 'pr_open'"));
    assert!(rendered.contains("SOR Verification scope expected 'main_repo' but found 'worktree'"));
    assert!(rendered.contains(
        "SOR Worktree-only paths remaining expected 'none' or retained issue worktree but found 'adl/src/foo.rs'"
    ));
    assert!(rendered.contains("STP status expected '\"complete\"' but found '\"draft\"'"));
    assert!(rendered.contains("SIP Branch expected 'codex/1410-canonical-slug'"));
    assert!(rendered.contains("SIP still contains pre-run lifecycle wording"));
    assert!(rendered.contains(
        "SRP review_results must record final findings_status/recommended_outcome truth for closed issues"
    ));
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
    fs::write(
        canonical_dir.join("srp.md"),
        issue_ref_completed_srp_content(),
    )
    .expect("write normalized srp");
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
    fs::write(
        canonical_dir.join("srp.md"),
        issue_ref_completed_srp_content(),
    )
    .expect("write normalized srp");
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
        fs::write(dir.join("srp.md"), issue_ref_completed_srp_content())
            .expect("write normalized srp");
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

#[test]
fn sync_completed_output_surfaces_copies_completed_output_to_canonical_output_and_link() {
    let _guard = env_lock();
    let _manifest_guard = set_tooling_manifest_root_to_workspace();
    let temp = temp_dir("adl-pr-lifecycle-sync-completed-output-copy");
    let repo = temp.join("repo");
    let origin = temp.join("origin.git");
    init_repo_with_origin(&repo, &origin);
    let issue_ref = IssueRef::new(1410, "v0.87", "canonical-slug").expect("issue ref");

    let completed_output = temp.join("completed").join("worktree-output.md");
    fs::create_dir_all(completed_output.parent().expect("completed output parent"))
        .expect("create completed output parent");
    let expected_sor = issue_ref_sync_completed_output_content();
    fs::write(&completed_output, &expected_sor).expect("write completed output");

    let synced = sync_completed_output_surfaces(&repo, &repo, &issue_ref, &completed_output)
        .expect("sync should copy completed output");
    let expected_output = issue_ref.task_bundle_output_path(&repo);
    assert_eq!(synced, expected_output);
    assert_eq!(
        fs::read_to_string(&synced).expect("read synced output"),
        expected_sor
    );

    let cards_root = resolve_cards_root(&repo, None);
    let review_output = card_output_path(&cards_root, issue_ref.issue_number());
    assert!(fs::symlink_metadata(&review_output)
        .expect("read review output metadata")
        .file_type()
        .is_symlink());
    assert_eq!(
        fs::read_link(&review_output).expect("review output symlink target"),
        expected_output
    );
}

#[test]
fn sync_completed_output_surfaces_skips_copy_when_output_already_canonical() {
    let _guard = env_lock();
    let _manifest_guard = set_tooling_manifest_root_to_workspace();
    let temp = temp_dir("adl-pr-lifecycle-sync-completed-output-no-copy");
    let repo = temp.join("repo");
    let origin = temp.join("origin.git");
    init_repo_with_origin(&repo, &origin);
    let issue_ref = IssueRef::new(1410, "v0.87", "canonical-slug").expect("issue ref");

    let completed_output = issue_ref.task_bundle_output_path(&repo);
    fs::create_dir_all(completed_output.parent().expect("completed output parent"))
        .expect("create output parent");
    let expected_sor = issue_ref_sync_completed_output_content();
    fs::write(&completed_output, &expected_sor).expect("write completed output");

    let synced = sync_completed_output_surfaces(&repo, &repo, &issue_ref, &completed_output)
        .expect("sync should skip canonical copy");
    assert_eq!(synced, completed_output);
    assert_eq!(
        fs::read_to_string(&synced).expect("read canonical output"),
        expected_sor
    );
}

#[test]
fn closeout_closed_completed_issue_bundle_records_prune_result_on_canonical_output() {
    let _guard = env_lock();
    let _manifest_guard = set_tooling_manifest_root_to_workspace();
    let temp = temp_dir("adl-pr-lifecycle-closeout-canonical-output");
    let repo = temp.join("repo");
    let origin = temp.join("origin.git");
    init_repo_with_origin(&repo, &origin);
    let issue_ref = IssueRef::new(1410, "v0.87", "canonical-slug").expect("issue ref");
    let worktree = issue_ref.default_worktree_path(&repo, None);
    let sip_text = format!(
        "# ADL Input Card\n\nTask ID: {task_id}\nRun ID: {task_id}\nVersion: v0.87\nTitle: PR Command Sync Coverage\nBranch: codex/1410-canonical-slug\n\nContext:\n- Issue: https://github.com/example/repo/issues/{issue}\n- PR: https://github.com/example/repo/pull/{issue}\n- Source Issue Prompt: .adl/v0.87/bodies/issue-1410-canonical-slug.md\n- Docs: none\n- Other: none\n\n## Agent Execution Rules\n- Work only in the issue worktree until closeout reconciles the canonical bundle.\n- Keep closeout proof repo-relative and deterministic.\n\n## Prompt Spec\n```yaml\nprompt_schema: adl.v1\nactor:\n  role: execution_agent\n  name: adl\nmodel:\n  id: codl\n  determinism_mode: stable\ninputs:\n  sections:\n    - goal\n    - required_outcome\n    - acceptance_criteria\n    - inputs\n    - target_files_surfaces\n    - validation_plan\n    - demo_proof_requirements\n    - constraints_policies\n    - system_invariants\n    - reviewer_checklist\n    - non_goals_out_of_scope\n    - notes_risks\n    - instructions_to_agent\noutputs:\n  output_card: .adl/v0.87/tasks/{bundle}/sor.md\n  summary_style: concise_structured\nconstraints:\n  include_system_invariants: true\n  include_reviewer_checklist: true\n  disallow_secrets: true\n  disallow_absolute_host_paths: true\nautomation_hints:\n  source_issue_prompt_required: true\n  target_files_surfaces_recommended: true\n  validation_plan_required: true\n  required_outcome_type_supported: true\nreview_surfaces:\n  - card_review_checklist.v1\n  - card_review_output.v1\n  - card_reviewer_gpt.v1.1\n```\n\nReviewer protocol IDs are versioned and order-sensitive:\n1. checklist contract\n2. output artifact contract\n3. reviewer behavior contract\n\nPrompt Spec contract notes:\n- Supported section IDs and machine-readable field semantics are defined in `docs/tooling/prompt-spec.md`.\n- Missing required Prompt Spec keys or required boolean `automation_hints` fields should fail lint.\n- Prompt generation must preserve declared section order rather than heuristic extraction.\n\nExecution:\n- Agent: adl\n- Provider: local\n- Tools allowed: git, cargo, bash\n- Sandbox / approvals: workspace-write / on-failure\n- Source issue-prompt slug: canonical-slug\n- Required outcome type: code\n- Demo required: false\n\n## Goal\n\nRecord canonical closeout truth after pruning the clean issue worktree.\n\n## Required Outcome\n\n- closed completed closeout preserves canonical output truth and records the prune result\n\n## Acceptance Criteria\n\n- canonical output remains valid after closeout\n- worktree prune result is recorded on the canonical output card\n- the clean issue worktree is removed\n\n## Inputs\n- linked source issue prompt\n- canonical `sip.md`, `stp.md`, and `sor.md` surfaces for the closed issue\n- current repository state before closeout reconciliation\n\n## Target Files / Surfaces\n- `.adl/v0.87/tasks/{bundle}/sor.md`\n- `.adl/v0.87/tasks/{bundle}/sip.md`\n- `.adl/v0.87/tasks/{bundle}/stp.md`\n\n## Validation Plan\n- `cargo test --manifest-path adl/Cargo.toml closeout_closed_completed_issue_bundle_records_prune_result_on_canonical_output -- --nocapture`\n- `bash adl/tools/validate_structured_prompt.sh --type stp --input .adl/v0.87/tasks/{bundle}/stp.md`\n- `bash adl/tools/validate_structured_prompt.sh --type sip --input .adl/v0.87/tasks/{bundle}/sip.md`\n\n## Demo / Proof Requirements\n- Demo set: none\n- Proof surfaces: canonical `sor.md` plus prune-result recording\n- No-demo rationale: this is a lifecycle closeout test fixture only\n\n## Constraints / Policies\n- Determinism: keep closeout proof stable for identical inputs.\n- Security and privacy: do not introduce secrets, prompts, tool arguments, or absolute host paths.\n- Resource limits: use focused lifecycle validation only.\n\n## System Invariants (must remain true)\n- Deterministic execution for identical inputs.\n- No hidden state or undeclared side effects.\n- Canonical closeout truth remains replay-compatible.\n- Artifact paths remain repo-relative.\n\n## Reviewer Checklist (machine-readable hints)\n```yaml\ndeterminism_required: true\nnetwork_allowed: false\nartifact_schema_change: false\nreplay_required: false\nsecurity_sensitive: true\nci_validation_required: true\n```\n\n## Non-goals / Out of scope\n- PR publication or merge handling\n- unrelated repository repair\n\n## Notes / Risks\n- Keep the fixture self-contained so CI does not depend on ignored local card bundles.\n\n## Instructions to the Agent\n- Validate the canonical bundle surfaces before running closeout.\n- Reconcile closeout truth without widening into PR publication.\n- Record execution outcome truth only in the paired `sor.md` output.\n",
        task_id = issue_ref.task_issue_id(),
        issue = issue_ref.issue_number(),
        bundle = issue_ref.task_bundle_dir_name(),
    );
    let stp_text = "---\nissue_card_schema: adl.issue.v1\nwp: \"WP-15\"\nqueue: \"tools\"\nslug: \"canonical-slug\"\ntitle: \"PR Command Sync Coverage\"\nlabels:\n  - \"area:tools\"\nissue_number: 1410\nstatus: \"draft\"\naction: \"edit\"\ndepends_on: []\nmilestone_sprint: \"test fixture\"\nrequired_outcome_type:\n  - \"code\"\nrepo_inputs:\n  - \"adl/src/cli/pr_cmd/lifecycle/tests.rs\"\ncanonical_files:\n  - \".adl/v0.87/tasks/issue-1410__canonical-slug/sor.md\"\ndemo_required: false\ndemo_names: []\nissue_graph_notes:\n  - \"Self-contained lifecycle closeout fixture.\"\npr_start:\n  enabled: false\n  slug: \"canonical-slug\"\n---\n\n# PR Command Sync Coverage\n\n## Summary\n\nSelf-contained fixture for canonical closeout reconciliation.\n\n## Goal\n\nRecord canonical closeout truth after pruning a clean issue worktree.\n\n## Required Outcome\n\n- canonical closeout truth remains valid after pruning the issue worktree\n\n## Deliverables\n\n- canonical `sor.md` remains final-valid after closeout\n- worktree prune result is recorded on the canonical output card\n\n## Acceptance Criteria\n\n- closeout records the worktree prune result\n- the canonical output card remains final-valid after reconciliation\n- the clean issue worktree is removed\n\n## Repo Inputs\n\n- `adl/src/cli/pr_cmd/lifecycle/tests.rs`\n\n## Dependencies\n\n- none\n\n## Demo Expectations\n\n- none\n\n## Non-goals\n\n- PR publication or merge handling\n\n## Issue-Graph Notes\n\n- Self-contained lifecycle closeout fixture.\n\n## Notes\n\n- Keep the fixture portable so CI does not depend on ignored local card bundles.\n\n## Tooling Notes\n\n- Validate closeout behavior through focused lifecycle tests only.\n";

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

    let canonical_dir = issue_ref.task_bundle_dir_path(&repo);
    fs::create_dir_all(&canonical_dir).expect("canonical dir");
    fs::write(canonical_dir.join("stp.md"), stp_text).expect("write stp");
    fs::write(canonical_dir.join("sip.md"), sip_text).expect("write sip");
    fs::write(
        canonical_dir.join("srp.md"),
        issue_ref_completed_srp_content(),
    )
    .expect("write srp");
    let output = canonical_dir.join("sor.md");
    fs::write(&output, issue_ref_sync_completed_output_content()).expect("write sor");

    closeout_closed_completed_issue_bundle(&repo, &repo, &issue_ref, &output)
        .expect("closeout should preserve canonical output");

    let text = fs::read_to_string(&output).expect("read sor");
    assert!(text.contains("- Worktree prune result: pruned: adl-wp-1410"));
    assert!(!worktree.exists(), "worktree should be pruned");
    ensure_closed_completed_issue_bundle_truth(&repo, &issue_ref, &output)
        .expect("canonical truth remains valid");
}

#[test]
fn closeout_recovers_missing_primary_cards_from_bound_worktree_bundle() {
    let _guard = env_lock();
    let _manifest_guard = set_tooling_manifest_root_to_workspace();
    let temp = temp_dir("adl-pr-lifecycle-closeout-recovers-worktree-cards");
    let repo = temp.join("repo");
    let origin = temp.join("origin.git");
    init_repo_with_origin(&repo, &origin);
    copy_prompt_templates(&repo);
    fs::write(repo.join(".gitignore"), ".adl/\n").expect("write gitignore");
    assert!(Command::new("git")
        .args([
            "-C",
            path_str(&repo).expect("repo path"),
            "add",
            ".gitignore"
        ])
        .status()
        .expect("git add gitignore")
        .success());
    assert!(Command::new("git")
        .args([
            "-C",
            path_str(&repo).expect("repo path"),
            "commit",
            "-q",
            "-m",
            "ignore local adl state",
        ])
        .status()
        .expect("git commit gitignore")
        .success());

    let issue_ref = IssueRef::new(1410, "v0.87", "canonical-slug").expect("issue ref");
    let title = "PR Command Sync Coverage";
    let source_path = issue_ref.issue_prompt_path(&repo);
    fs::create_dir_all(source_path.parent().expect("source parent")).expect("source parent");
    fs::write(
        &source_path,
        "## Summary\n\nCloseout recovery fixture.\n\n## Goal\n\nRecover missing primary cards from the bound worktree.\n\n## Required Outcome\n\n- closeout restores missing primary cards\n\n## Deliverables\n\n- focused lifecycle regression test\n\n## Acceptance Criteria\n\n- primary STP and SIP are restored from the worktree\n- clean worktree is pruned\n\n## Repo Inputs\n\n- `adl/src/cli/pr_cmd/lifecycle/reconciliation.rs`\n\n## Dependencies\n\n- none\n\n## Demo Expectations\n\n- none\n\n## Non-goals\n\n- no broader closeout redesign\n\n## Issue-Graph Notes\n\n- regression fixture\n\n## Notes\n\n- local `.adl` state is ignored\n\n## Tooling Notes\n\n- focused lifecycle test\n",
    )
    .expect("write source");
    ensure_task_bundle_stp(&repo, &issue_ref, &source_path).expect("stp");
    ensure_pre_run_bootstrap_cards(&repo, &issue_ref, title, &source_path).expect("cards");
    let canonical_dir = issue_ref.task_bundle_dir_path(&repo);
    fs::write(
        canonical_dir.join("srp.md"),
        issue_ref_completed_srp_content(),
    )
    .expect("write completed srp");
    let output = issue_ref.task_bundle_output_path(&repo);
    fs::write(&output, issue_ref_sync_completed_output_content()).expect("write completed sor");

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
    let worktree_bundle = issue_ref.task_bundle_dir_path(&worktree);
    fs::create_dir_all(&worktree_bundle).expect("worktree bundle");
    for relative in ["stp.md", "sip.md", "spp.md", "srp.md", "sor.md"] {
        fs::copy(canonical_dir.join(relative), worktree_bundle.join(relative))
            .expect("copy card to worktree");
    }
    let stale_ref = IssueRef::new(1410, "v0.87", "stale-primary-duplicate").expect("stale ref");
    let stale_bundle = stale_ref.task_bundle_dir_path(&repo);
    fs::create_dir_all(&stale_bundle).expect("stale bundle");
    for relative in ["stp.md", "sip.md", "spp.md", "srp.md", "sor.md"] {
        fs::copy(canonical_dir.join(relative), stale_bundle.join(relative))
            .expect("copy stale primary duplicate card");
    }
    fs::write(
        stale_bundle.join("stp.md"),
        format!(
            "{}\n\n<!-- stale-primary-duplicate-marker -->\n",
            fs::read_to_string(stale_bundle.join("stp.md")).expect("read stale stp")
        ),
    )
    .expect("mark stale stp");

    for relative in ["stp.md", "sip.md", "spp.md", "srp.md"] {
        fs::remove_file(canonical_dir.join(relative)).expect("remove primary card");
    }

    closeout_closed_completed_issue_bundle(&repo, &repo, &issue_ref, &output)
        .expect("closeout should recover missing primary cards from worktree");

    for relative in ["stp.md", "sip.md", "spp.md", "srp.md"] {
        assert!(
            canonical_dir.join(relative).is_file(),
            "{relative} should be restored"
        );
    }
    assert!(
        !fs::read_to_string(canonical_dir.join("stp.md"))
            .expect("read restored stp")
            .contains("stale-primary-duplicate-marker"),
        "bound worktree recovery should win over stale primary duplicate bundles"
    );
    assert!(!worktree.exists(), "worktree should be pruned");
    ensure_closed_completed_issue_bundle_truth(&repo, &issue_ref, &output)
        .expect("canonical truth remains valid");
}

#[test]
fn closeout_recovers_stale_root_srp_and_sor_from_bound_worktree_bundle() {
    let _guard = env_lock();
    let _manifest_guard = set_tooling_manifest_root_to_workspace();
    let temp = temp_dir("adl-pr-lifecycle-closeout-recovers-stale-root-review-output");
    let repo = temp.join("repo");
    let origin = temp.join("origin.git");
    init_repo_with_origin(&repo, &origin);
    copy_prompt_templates(&repo);
    fs::write(repo.join(".gitignore"), ".adl/\n").expect("write gitignore");
    assert!(Command::new("git")
        .args([
            "-C",
            path_str(&repo).expect("repo path"),
            "add",
            ".gitignore"
        ])
        .status()
        .expect("git add gitignore")
        .success());
    assert!(Command::new("git")
        .args([
            "-C",
            path_str(&repo).expect("repo path"),
            "commit",
            "-q",
            "-m",
            "ignore local adl state",
        ])
        .status()
        .expect("git commit gitignore")
        .success());

    let issue_ref = IssueRef::new(1410, "v0.87", "canonical-slug").expect("issue ref");
    let title = "PR Command Sync Coverage";
    let source_path = issue_ref.issue_prompt_path(&repo);
    fs::create_dir_all(source_path.parent().expect("source parent")).expect("source parent");
    fs::write(
        &source_path,
        "## Summary\n\nCloseout stale-root recovery fixture.\n\n## Goal\n\nRecover stale root SRP and SOR truth from the bound worktree before prune.\n\n## Required Outcome\n\n- closeout replaces stale root review/output truth with the final worktree truth\n\n## Deliverables\n\n- focused lifecycle regression test\n\n## Acceptance Criteria\n\n- canonical SRP and SOR are refreshed from the worktree\n- clean worktree is pruned\n\n## Repo Inputs\n\n- `adl/src/cli/pr_cmd/lifecycle/reconciliation.rs`\n\n## Dependencies\n\n- none\n\n## Demo Expectations\n\n- none\n\n## Non-goals\n\n- no broader closeout redesign\n\n## Issue-Graph Notes\n\n- regression fixture\n\n## Notes\n\n- local `.adl` state is ignored\n\n## Tooling Notes\n\n- focused lifecycle test\n",
    )
    .expect("write source");
    ensure_task_bundle_stp(&repo, &issue_ref, &source_path).expect("stp");
    ensure_pre_run_bootstrap_cards(&repo, &issue_ref, title, &source_path).expect("cards");
    let canonical_dir = issue_ref.task_bundle_dir_path(&repo);
    let output = issue_ref.task_bundle_output_path(&repo);
    fs::write(
        &output,
        "Status: NOT_STARTED\n- Integration state: worktree_only\n- Verification scope: worktree\n- Worktree-only paths remaining: issue bundle still local\n",
    )
    .expect("write stale root sor");
    fs::write(
        canonical_dir.join("srp.md"),
        "---\nschema_version: \"0.1\"\nartifact_type: \"structured_review_prompt\"\nname: \"fixture-review\"\nissue: 1410\ntask_id: \"issue-1410\"\nversion: v0.87\ntitle: \"Fixture\"\nbranch: \"codex/1410-canonical-slug\"\nstatus: \"draft\"\nreview_results:\n  findings_status: \"not_run\"\n  recommended_outcome: \"not_run\"\n---\n\n# Structured Review Prompt\n\n## Review Summary\n\nStale root review scaffold.\n\n## Scope Basis\n\n- fixture\n\n## In-Scope Surfaces\n\n- tracked changes\n\n## Evidence Rules\n\n- repo only\n\n## Validation Inputs\n\n- issue local\n\n## Allowed Dispositions\n\n- PASS\n- BLOCK\n\n## Reviewer Constraints\n\n- keep scope narrow\n\n## Refusal Policy\n\n- refuse unsupported claims\n\n## Follow-up Routing\n\n- route back to issue\n\n## Non-Claims\n\n- stale scaffold\n\n## Notes\n\nStale root review scaffold.\n",
    )
    .expect("write stale root srp");

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
    let worktree_bundle = issue_ref.task_bundle_dir_path(&worktree);
    fs::create_dir_all(&worktree_bundle).expect("worktree bundle");
    fs::copy(canonical_dir.join("stp.md"), worktree_bundle.join("stp.md")).expect("copy stp");
    fs::copy(canonical_dir.join("sip.md"), worktree_bundle.join("sip.md")).expect("copy sip");
    fs::copy(canonical_dir.join("spp.md"), worktree_bundle.join("spp.md")).expect("copy spp");
    fs::write(
        worktree_bundle.join("srp.md"),
        issue_ref_completed_srp_content(),
    )
    .expect("write final worktree srp");
    fs::write(
        worktree_bundle.join("sor.md"),
        issue_ref_sync_completed_output_content(),
    )
    .expect("write final worktree sor");

    closeout_closed_completed_issue_bundle(&repo, &repo, &issue_ref, &output)
        .expect("closeout should recover stale root review/output truth from worktree");

    let root_srp_text =
        fs::read_to_string(canonical_dir.join("srp.md")).expect("read recovered root srp");
    assert!(root_srp_text.contains("findings_status: \"no_findings\""));
    assert!(root_srp_text.contains("recommended_outcome: \"pass\""));
    let root_sor_text = fs::read_to_string(&output).expect("read recovered root sor");
    assert!(root_sor_text.contains("Status: DONE"));
    assert!(root_sor_text.contains("- Integration state: merged"));
    assert!(!worktree.exists(), "worktree should be pruned");
    ensure_closed_completed_issue_bundle_truth(&repo, &issue_ref, &output)
        .expect("canonical truth remains valid after stale-root recovery");
}

#[test]
fn closeout_recovers_malformed_root_srp_from_bound_worktree_bundle() {
    let _guard = env_lock();
    let _manifest_guard = set_tooling_manifest_root_to_workspace();
    let temp = temp_dir("adl-pr-lifecycle-closeout-recovers-malformed-root-srp");
    let repo = temp.join("repo");
    let origin = temp.join("origin.git");
    init_repo_with_origin(&repo, &origin);
    copy_prompt_templates(&repo);
    fs::write(repo.join(".gitignore"), ".adl/\n").expect("write gitignore");
    assert!(Command::new("git")
        .args([
            "-C",
            path_str(&repo).expect("repo path"),
            "add",
            ".gitignore"
        ])
        .status()
        .expect("git add gitignore")
        .success());
    assert!(Command::new("git")
        .args([
            "-C",
            path_str(&repo).expect("repo path"),
            "commit",
            "-q",
            "-m",
            "ignore local adl state",
        ])
        .status()
        .expect("git commit gitignore")
        .success());

    let issue_ref = IssueRef::new(1410, "v0.87", "canonical-slug").expect("issue ref");
    let title = "PR Command Sync Coverage";
    let source_path = issue_ref.issue_prompt_path(&repo);
    fs::create_dir_all(source_path.parent().expect("source parent")).expect("source parent");
    fs::write(
        &source_path,
        "## Summary\n\nCloseout malformed-root recovery fixture.\n\n## Goal\n\nRecover malformed root SRP truth from the bound worktree before prune.\n\n## Required Outcome\n\n- closeout treats malformed root SRP as recoverable drift and restores final worktree truth\n\n## Deliverables\n\n- focused lifecycle regression test\n\n## Acceptance Criteria\n\n- malformed canonical SRP does not abort closeout recovery\n- canonical SRP is refreshed from the worktree\n- clean worktree is pruned\n\n## Repo Inputs\n\n- `adl/src/cli/pr_cmd/lifecycle/reconciliation.rs`\n\n## Dependencies\n\n- none\n\n## Demo Expectations\n\n- none\n\n## Non-goals\n\n- no broader closeout redesign\n\n## Issue-Graph Notes\n\n- regression fixture\n\n## Notes\n\n- local `.adl` state is ignored\n\n## Tooling Notes\n\n- focused lifecycle test\n",
    )
    .expect("write source");
    ensure_task_bundle_stp(&repo, &issue_ref, &source_path).expect("stp");
    ensure_pre_run_bootstrap_cards(&repo, &issue_ref, title, &source_path).expect("cards");
    let canonical_dir = issue_ref.task_bundle_dir_path(&repo);
    let output = issue_ref.task_bundle_output_path(&repo);
    fs::write(&output, issue_ref_sync_completed_output_content()).expect("write completed sor");
    fs::write(
        canonical_dir.join("srp.md"),
        "---\nreview_results: [this is not valid yaml\n",
    )
    .expect("write malformed root srp");

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
    let worktree_bundle = issue_ref.task_bundle_dir_path(&worktree);
    fs::create_dir_all(&worktree_bundle).expect("worktree bundle");
    for relative in ["stp.md", "sip.md", "spp.md"] {
        fs::copy(canonical_dir.join(relative), worktree_bundle.join(relative))
            .expect("copy card to worktree");
    }
    fs::write(
        worktree_bundle.join("srp.md"),
        issue_ref_completed_srp_content(),
    )
    .expect("write final worktree srp");
    fs::write(
        worktree_bundle.join("sor.md"),
        issue_ref_sync_completed_output_content(),
    )
    .expect("write final worktree sor");

    closeout_closed_completed_issue_bundle(&repo, &repo, &issue_ref, &output)
        .expect("closeout should recover malformed root srp from worktree");

    let root_srp_text =
        fs::read_to_string(canonical_dir.join("srp.md")).expect("read recovered root srp");
    assert!(root_srp_text.contains("findings_status: \"no_findings\""));
    assert!(root_srp_text.contains("recommended_outcome: \"pass\""));
    assert!(!worktree.exists(), "worktree should be pruned");
    ensure_closed_completed_issue_bundle_truth(&repo, &issue_ref, &output)
        .expect("canonical truth remains valid after malformed-root recovery");
}

#[test]
fn closeout_retains_dirty_stale_worktree_when_canonical_truth_is_complete() {
    let _guard = env_lock();
    let _manifest_guard = set_tooling_manifest_root_to_workspace();
    let temp = temp_dir("adl-pr-lifecycle-closeout-dirty-worktree");
    let repo = temp.join("repo");
    let origin = temp.join("origin.git");
    init_repo_with_origin(&repo, &origin);
    copy_prompt_templates(&repo);
    fs::write(repo.join(".gitignore"), ".adl/\n").expect("write gitignore");
    assert!(Command::new("git")
        .args([
            "-C",
            path_str(&repo).expect("repo path"),
            "add",
            ".gitignore"
        ])
        .status()
        .expect("git add gitignore")
        .success());
    assert!(Command::new("git")
        .args([
            "-C",
            path_str(&repo).expect("repo path"),
            "commit",
            "-q",
            "-m",
            "ignore local adl state",
        ])
        .status()
        .expect("git commit gitignore")
        .success());

    let issue_ref = IssueRef::new(1410, "v0.87", "canonical-slug").expect("issue ref");
    let title = "PR Command Dirty Closeout";
    let source_path = issue_ref.issue_prompt_path(&repo);
    fs::create_dir_all(source_path.parent().expect("source parent")).expect("source parent");
    fs::write(
        &source_path,
        "## Summary\n\nDirty worktree closeout fixture.\n\n## Goal\n\nRefuse to prune a dirty worktree while recording closeout truth.\n\n## Required Outcome\n\n- closeout fails closed when the bound worktree is dirty\n\n## Deliverables\n\n- focused lifecycle regression test\n\n## Acceptance Criteria\n\n- dirty worktree is retained\n- canonical SOR records blocked dirty prune result\n\n## Repo Inputs\n\n- `adl/src/cli/pr_cmd/lifecycle/reconciliation.rs`\n\n## Dependencies\n\n- none\n\n## Demo Expectations\n\n- none\n\n## Non-goals\n\n- no broader closeout redesign\n\n## Issue-Graph Notes\n\n- regression fixture\n\n## Notes\n\n- local `.adl` state is ignored\n\n## Tooling Notes\n\n- focused lifecycle test\n",
    )
    .expect("write source");
    ensure_task_bundle_stp(&repo, &issue_ref, &source_path).expect("stp");
    ensure_pre_run_bootstrap_cards(&repo, &issue_ref, title, &source_path).expect("cards");
    let canonical_dir = issue_ref.task_bundle_dir_path(&repo);
    fs::write(
        canonical_dir.join("srp.md"),
        issue_ref_completed_srp_content(),
    )
    .expect("write completed srp");
    let output = issue_ref.task_bundle_output_path(&repo);
    fs::write(&output, issue_ref_sync_completed_output_content()).expect("write completed sor");

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
    fs::write(worktree.join("DIRTY.txt"), "dirty\n").expect("dirty marker");

    closeout_closed_completed_issue_bundle(&repo, &repo, &issue_ref, &output)
        .expect("dirty stale worktree should be retained when root closeout truth is complete");

    assert!(worktree.is_dir(), "dirty worktree should be retained");
    let text = fs::read_to_string(&output).expect("read sor");
    assert!(text.contains(
        "- Worktree prune result: retained_with_reason: dirty stale worktree retained: adl-wp-1410"
    ));
    assert!(text.contains("- Worktree-only paths remaining: issue worktree retained: adl-wp-1410"));
    ensure_closed_completed_issue_bundle_truth(&repo, &issue_ref, &output)
        .expect("canonical truth accepts retained stale worktree");
}

#[test]
fn closeout_refuses_dirty_worktree_when_canonical_truth_needs_recovery() {
    let _guard = env_lock();
    let _manifest_guard = set_tooling_manifest_root_to_workspace();
    let temp = temp_dir("adl-pr-lifecycle-closeout-dirty-worktree-only-source");
    let repo = temp.join("repo");
    let origin = temp.join("origin.git");
    init_repo_with_origin(&repo, &origin);
    copy_prompt_templates(&repo);
    fs::write(repo.join(".gitignore"), ".adl/\n").expect("write gitignore");
    assert!(Command::new("git")
        .args([
            "-C",
            path_str(&repo).expect("repo path"),
            "add",
            ".gitignore"
        ])
        .status()
        .expect("git add gitignore")
        .success());
    assert!(Command::new("git")
        .args([
            "-C",
            path_str(&repo).expect("repo path"),
            "commit",
            "-q",
            "-m",
            "ignore local adl state",
        ])
        .status()
        .expect("git commit gitignore")
        .success());

    let issue_ref = IssueRef::new(1410, "v0.87", "canonical-slug").expect("issue ref");
    let title = "PR Command Dirty Recovery Closeout";
    let source_path = issue_ref.issue_prompt_path(&repo);
    fs::create_dir_all(source_path.parent().expect("source parent")).expect("source parent");
    fs::write(
        &source_path,
        "## Summary\n\nDirty worktree recovery fixture.\n\n## Goal\n\nDo not recover closeout truth from a dirty worktree.\n\n## Required Outcome\n\n- closeout fails when dirty worktree cards are the only recovery source\n\n## Deliverables\n\n- focused lifecycle regression test\n\n## Acceptance Criteria\n\n- dirty worktree is retained\n- root cards are not recovered from dirty worktree residue\n\n## Repo Inputs\n\n- `adl/src/cli/pr_cmd/lifecycle/reconciliation.rs`\n\n## Dependencies\n\n- none\n\n## Demo Expectations\n\n- none\n\n## Non-goals\n\n- no broader closeout redesign\n\n## Issue-Graph Notes\n\n- regression fixture\n\n## Notes\n\n- local `.adl` state is ignored\n\n## Tooling Notes\n\n- focused lifecycle test\n",
    )
    .expect("write source");
    ensure_task_bundle_stp(&repo, &issue_ref, &source_path).expect("stp");
    ensure_pre_run_bootstrap_cards(&repo, &issue_ref, title, &source_path).expect("cards");
    let canonical_dir = issue_ref.task_bundle_dir_path(&repo);
    let output = issue_ref.task_bundle_output_path(&repo);
    fs::write(&output, issue_ref_sync_completed_output_content()).expect("write completed sor");

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
    let worktree_bundle = issue_ref.task_bundle_dir_path(&worktree);
    fs::create_dir_all(&worktree_bundle).expect("worktree bundle");
    for relative in ["stp.md", "sip.md", "spp.md", "srp.md", "sor.md"] {
        fs::copy(canonical_dir.join(relative), worktree_bundle.join(relative))
            .expect("copy card to dirty worktree");
    }
    for relative in ["stp.md", "sip.md", "spp.md", "srp.md"] {
        fs::remove_file(canonical_dir.join(relative)).expect("remove root card");
    }
    fs::write(worktree.join("DIRTY.txt"), "dirty\n").expect("dirty marker");

    let err = closeout_closed_completed_issue_bundle(&repo, &repo, &issue_ref, &output)
        .expect_err("dirty worktree should not be used as a recovery source");

    assert!(
        err.to_string().contains("dirty worktree")
            || err.to_string().contains("failed to create")
            || err.to_string().contains("missing canonical")
            || err.to_string().contains("failed closed"),
        "unexpected closeout error: {err}"
    );
    assert!(worktree.is_dir(), "dirty worktree should be retained");
    assert!(
        !canonical_dir.join("stp.md").exists(),
        "root cards should not be recovered from dirty worktree residue"
    );
}

#[test]
fn record_worktree_prune_result_inserts_result_line_after_worktree_only_value() {
    let temp = temp_dir("adl-pr-lifecycle-record-worktree-prune-result");
    let output = temp.join("sor.md");
    fs::write(
        &output,
        "Task ID: issue-1410\n- Worktree-only paths remaining: adl/src/legacy.rs\n",
    )
    .expect("write output");

    record_worktree_prune_result(&output, "pruned: adl-wp-1410").expect("record prune result");

    let text = fs::read_to_string(&output).expect("read output");
    assert!(
        text.contains("Task ID: issue-1410"),
        "task id should remain"
    );
    assert!(
        text.contains("- Worktree prune result: pruned: adl-wp-1410"),
        "prune result should be added"
    );
}

#[test]
fn replace_worktree_only_paths_remaining_rewrites_existing_line() {
    let temp = temp_dir("adl-pr-lifecycle-replace-worktree-only");
    let output = temp.join("sor.md");
    fs::write(
        &output,
        "- Worktree-only paths remaining: adl/src/legacy.rs\nFollow-up: none\n",
    )
    .expect("write output");

    replace_worktree_only_paths_remaining(&output, "none").expect("replace worktree paths");

    assert_eq!(
        fs::read_to_string(&output).expect("read output"),
        "- Worktree-only paths remaining: none\nFollow-up: none\n"
    );
}

#[test]
fn same_filesystem_target_detects_exact_match() {
    let temp = temp_dir("adl-pr-lifecycle-same-target-exact");
    let path = temp.join("output.md");
    fs::write(&path, "content\n").expect("write output");

    assert!(same_filesystem_target(&path, &path).expect("compare same target"));
}

fn issue_ref_sync_completed_output_content() -> String {
    [
        "# issue-1410-canonical-slug",
        "",
        "Task ID: issue-1410",
        "Run ID: issue-1410",
        "Version: v0.87",
        "Title: PR Command Sync Coverage",
        "Branch: codex/1410-canonical-slug",
        "Status: DONE",
        "",
        "Execution:",
        "- Actor: adl",
        "- Model: codl",
        "- Provider: local",
        "- Start Time: 2026-01-01T00:00:00Z",
        "- End Time: 2026-01-01T00:00:10Z",
        "",
        "## Summary",
        "- Completed output sync surfaces successfully.",
        "",
        "## PVF Lane Truth",
        "- Lane: focused",
        "- Proof role: lifecycle closeout fixture",
        "- Determinism posture: deterministic",
        "- Release-gate impact: none",
        "",
        "## Issue Metrics Truth",
        "- Estimated elapsed seconds: unknown",
        "- Actual elapsed seconds: unknown",
        "- Estimated total tokens: unknown",
        "- Actual total tokens: unknown",
        "- Estimated validation seconds: unknown",
        "- Actual validation seconds: unknown",
        "- Goal metrics data source: unknown",
        "- Goal metrics source ref: unknown",
        "- Data-source confidence: unknown",
        "- Estimate error percent: unknown",
        "",
        "## Artifacts produced",
        "- `.adl/v0.87/tasks/issue-1410__canonical-slug/sor.md`",
        "",
        "## Actions taken",
        "- Synced output card from worktree path to canonical task bundle output.",
        "",
        "## Main Repo Integration (REQUIRED)",
        "- Main-repo paths updated: `.adl/v0.87/tasks/issue-1410__canonical-slug/sor.md`",
        "- Worktree-only paths remaining: none",
        "- Integration state: merged",
        "- Verification scope: main_repo",
        "- Integration method used:",
        "- Verification performed:",
        "  - `git status`",
        "- Result: PASS",
        "",
        "## Validation",
        "- Validation commands and their purpose:",
        "  - `bash adl/tools/validate_structured_prompt.sh --type sor --phase completed --input ...`",
        "Results:",
        "  - PASS",
        "",
        "## Verification Summary",
        "- Validation status: PASS",
        "",
        "## Determinism Evidence",
        "- Determinism: same inputs emit same output path format.",
        "",
        "## Security / Privacy Checks",
        "- Secret leakage scan: PASS",
        "",
        "## Replay Artifacts",
        "- Replay command: `git checkout main && cargo run -- adl ...`",
        "",
        "## Artifact Verification",
        "- Output artifact exists at `.adl/v0.87/tasks/issue-1410__canonical-slug/sor.md`.",
        "",
        "## Decisions / Deviations",
        "- No deviations.",
        "",
        "## Follow-ups / Deferred work",
        "- None.",
        "",
    ]
    .join("\n")
}

fn issue_ref_completed_srp_content() -> String {
    [
        "---",
        "schema_version: \"0.1\"",
        "artifact_type: \"structured_review_prompt\"",
        "name: \"fixture-review\"",
        "issue: 1410",
        "task_id: \"issue-1410\"",
        "version: v0.87",
        "title: \"PR Command Sync Coverage\"",
        "branch: \"codex/1410-canonical-slug\"",
        "status: \"approved\"",
        "review_results:",
        "  findings_status: \"no_findings\"",
        "  recommended_outcome: \"pass\"",
        "---",
        "",
        "# Structured Review Prompt",
        "",
        "## Review Summary",
        "",
        "Completed pre-PR review recorded.",
        "",
        "## Scope Basis",
        "",
        "- fixture",
        "",
        "## In-Scope Surfaces",
        "",
        "- tracked changes",
        "",
        "## Evidence Rules",
        "",
        "- repo only",
        "",
        "## Validation Inputs",
        "",
        "- issue local",
        "",
        "## Allowed Dispositions",
        "",
        "- PASS",
        "- BLOCK",
        "- NEEDS_FOLLOWUP",
        "",
        "## Reviewer Constraints",
        "",
        "- keep scope narrow",
        "",
        "## Refusal Policy",
        "",
        "- refuse unsupported claims",
        "",
        "## Follow-up Routing",
        "",
        "- route back to issue",
        "",
        "## Non-Claims",
        "",
        "- no non-claims beyond the fixture boundary",
        "",
        "## Notes",
        "",
        "Final review truth is recorded for closeout recovery.",
        "",
    ]
    .join("\n")
}
