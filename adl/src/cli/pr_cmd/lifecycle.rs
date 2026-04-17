use super::*;
use serde::Deserialize;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
struct GithubIssueLifecycleState {
    state: String,
    #[serde(rename = "stateReason")]
    state_reason: Option<String>,
}

pub(super) fn issue_is_closed_and_completed(issue: u32, repo: &str) -> Result<bool> {
    let Some(raw) = run_capture_allow_failure(
        "gh",
        &[
            "issue",
            "view",
            &issue.to_string(),
            "-R",
            repo,
            "--json",
            "state,stateReason",
        ],
    )?
    else {
        return Ok(false);
    };
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Ok(false);
    }
    let state: GithubIssueLifecycleState =
        serde_json::from_str(trimmed).context("failed to parse gh issue state json")?;
    Ok(state.state == "CLOSED" && state.state_reason.as_deref() == Some("COMPLETED"))
}

pub(super) fn ensure_issue_closed_completed_for_closeout(issue: u32, repo: &str) -> Result<()> {
    if !issue_is_closed_and_completed(issue, repo)? {
        bail!(
            "closeout: issue #{} is not closed with COMPLETED state yet; refusing automatic closeout",
            issue
        );
    }
    Ok(())
}

pub(super) fn wait_for_issue_closed_and_completed(issue: u32, repo: &str) -> Result<()> {
    for _ in 0..10 {
        if issue_is_closed_and_completed(issue, repo)? {
            return Ok(());
        }
        thread::sleep(Duration::from_millis(500));
    }
    bail!(
        "finish: issue #{} did not reach CLOSED/COMPLETED state after merge; closeout cannot proceed automatically",
        issue
    );
}

pub(super) fn reconcile_closed_completed_issue_bundle(
    repo_root: &Path,
    issue_ref: &IssueRef,
    canonical_output: &Path,
) -> Result<()> {
    let bundle_dir = issue_ref.task_bundle_dir_path(repo_root);
    if let Some(parent) = bundle_dir.parent() {
        fs::create_dir_all(parent)?;
    }

    let duplicates = matching_task_bundle_dirs(repo_root, issue_ref)?;
    if !bundle_dir.exists() {
        fs::create_dir_all(&bundle_dir)?;
    }

    for relative in ["stp.md", "sip.md", "sor.md"] {
        let canonical_path = bundle_dir.join(relative);
        if ensure_nonempty_file_path(&canonical_path)? {
            continue;
        }
        for duplicate in &duplicates {
            if *duplicate == bundle_dir {
                continue;
            }
            let candidate = duplicate.join(relative);
            if ensure_nonempty_file_path(&candidate)? {
                fs::copy(&candidate, &canonical_path).with_context(|| {
                    format!(
                        "doctor: failed to restore canonical bundle file '{}' from duplicate '{}'",
                        canonical_path.display(),
                        candidate.display()
                    )
                })?;
                break;
            }
        }
    }

    if !ensure_nonempty_file_path(canonical_output)? {
        for duplicate in &duplicates {
            if *duplicate == bundle_dir {
                continue;
            }
            let candidate = duplicate.join("sor.md");
            if ensure_nonempty_file_path(&candidate)? {
                fs::copy(&candidate, canonical_output).with_context(|| {
                    format!(
                        "doctor: failed to restore canonical sor from duplicate '{}'",
                        candidate.display()
                    )
                })?;
                break;
            }
        }

        if !ensure_nonempty_file_path(canonical_output)? {
            let cards_root = resolve_cards_root(repo_root, None);
            let review_output = card_output_path(&cards_root, issue_ref.issue_number());
            if ensure_nonempty_file_path(&review_output)? {
                fs::copy(&review_output, canonical_output).with_context(|| {
                    format!(
                        "doctor: failed to restore canonical sor from review card '{}'",
                        review_output.display()
                    )
                })?;
            }
        }
    }

    if !ensure_nonempty_file_path(canonical_output)? {
        bail!(
            "doctor: closed issue is missing canonical sor: {}",
            canonical_output.display()
        );
    }

    ensure_canonical_output_is_local_only(
        repo_root,
        canonical_output,
        "doctor: canonical .adl output surfaces must remain local-only during closed-issue reconciliation",
    )?;
    normalize_closed_completed_stp(&issue_ref.task_bundle_stp_path(repo_root))?;
    normalize_closed_completed_sip(&issue_ref.task_bundle_input_path(repo_root), issue_ref)?;
    normalize_closed_completed_output_card(canonical_output)?;
    validate_closed_completed_stp(repo_root, &issue_ref.task_bundle_stp_path(repo_root))?;
    validate_closed_completed_sip(repo_root, &issue_ref.task_bundle_input_path(repo_root))?;
    validate_completed_sor(repo_root, canonical_output)?;

    let cards_root = resolve_cards_root(repo_root, None);
    let review_output = card_output_path(&cards_root, issue_ref.issue_number());
    ensure_symlink(&review_output, canonical_output)?;
    Ok(())
}

pub(super) fn closeout_closed_completed_issue_bundle(
    repo_root: &Path,
    primary_root: &Path,
    issue_ref: &IssueRef,
    canonical_output: &Path,
) -> Result<()> {
    reconcile_closed_completed_issue_bundle(primary_root, issue_ref, canonical_output)?;
    ensure_closed_completed_issue_bundle_truth(primary_root, issue_ref, canonical_output)
        .with_context(|| {
            format!(
                "closeout: canonical closed-issue sor truth drift remains for issue #{}",
                issue_ref.issue_number()
            )
        })?;
    prune_issue_worktree(repo_root, primary_root, issue_ref)
}

pub(super) fn ensure_closed_completed_issue_bundle_truth(
    repo_root: &Path,
    issue_ref: &IssueRef,
    canonical_output: &Path,
) -> Result<()> {
    let bundle_dir = issue_ref.task_bundle_dir_path(repo_root);

    let mut mismatches = Vec::new();
    if !bundle_dir.is_dir() {
        mismatches.push(format!(
            "missing canonical task bundle directory: {}",
            bundle_dir.display()
        ));
    }
    if !ensure_nonempty_file_path(canonical_output)? {
        mismatches.push("missing canonical sor.md".to_string());
    } else {
        let text = fs::read_to_string(canonical_output)?;
        check_required_field(&text, "Status:", "DONE", "SOR Status", &mut mismatches);
        check_required_field(
            &text,
            "- Integration state:",
            "merged",
            "SOR Integration state",
            &mut mismatches,
        );
        check_required_field(
            &text,
            "- Verification scope:",
            "main_repo",
            "SOR Verification scope",
            &mut mismatches,
        );
        check_required_field(
            &text,
            "- Worktree-only paths remaining:",
            "none",
            "SOR Worktree-only paths remaining",
            &mut mismatches,
        );
    }

    let stp_path = issue_ref.task_bundle_stp_path(repo_root);
    if !ensure_nonempty_file_path(&stp_path)? {
        mismatches.push("missing canonical stp.md".to_string());
    } else {
        let text = fs::read_to_string(&stp_path)?;
        check_required_field(
            &text,
            "status:",
            "\"complete\"",
            "STP status",
            &mut mismatches,
        );
    }

    let sip_path = issue_ref.task_bundle_input_path(repo_root);
    if !ensure_nonempty_file_path(&sip_path)? {
        mismatches.push("missing canonical sip.md".to_string());
    } else {
        let text = fs::read_to_string(&sip_path)?;
        check_required_field(
            &text,
            "Branch:",
            &issue_ref.branch_name("codex"),
            "SIP Branch",
            &mut mismatches,
        );
        if text.contains("This issue is not started yet")
            || text.contains("before execution is bound")
            || text.contains("until `pr run` binds the branch and worktree")
        {
            mismatches.push("SIP still contains pre-run lifecycle wording".to_string());
        }
    }

    if !mismatches.is_empty() {
        bail!(
            "canonical closed-issue sor truth drift at {}: {}",
            canonical_output.display(),
            mismatches.join("; ")
        );
    }
    Ok(())
}

fn check_required_field(
    text: &str,
    prefix: &str,
    expected: &str,
    label: &str,
    mismatches: &mut Vec<String>,
) {
    match text
        .lines()
        .find(|line| line.starts_with(prefix))
        .map(|line| line[prefix.len()..].trim().to_string())
    {
        Some(actual) if actual == expected => {}
        Some(actual) => mismatches.push(format!(
            "{} expected '{}' but found '{}'",
            label, expected, actual
        )),
        None => mismatches.push(format!("{} is missing", label)),
    }
}

fn matching_task_bundle_dirs(repo_root: &Path, issue_ref: &IssueRef) -> Result<Vec<PathBuf>> {
    let tasks_dir = repo_root.join(".adl").join(issue_ref.scope()).join("tasks");
    if !tasks_dir.is_dir() {
        return Ok(Vec::new());
    }
    let prefix = format!("{}__", issue_ref.task_issue_id());
    let mut matches = Vec::new();
    for entry in fs::read_dir(&tasks_dir)? {
        let entry = entry?;
        if !entry.file_type()?.is_dir() {
            continue;
        }
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if name.starts_with(&prefix) {
            matches.push(entry.path());
        }
    }
    matches.sort();
    Ok(matches)
}

fn normalize_closed_completed_output_card(path: &Path) -> Result<()> {
    let mut text = fs::read_to_string(path)?;
    replace_field_line_in_text(&mut text, "Status", "DONE");
    replace_first_exact_line(
        &mut text,
        "- Integration state: worktree_only | pr_open | merged",
        "- Integration state: merged",
    );
    replace_first_exact_line(
        &mut text,
        "- Integration state: worktree_only",
        "- Integration state: merged",
    );
    replace_first_exact_line(
        &mut text,
        "- Integration state: pr_open",
        "- Integration state: merged",
    );
    replace_first_exact_line(
        &mut text,
        "- Verification scope: worktree | pr_branch | main_repo",
        "- Verification scope: main_repo",
    );
    replace_first_exact_line(
        &mut text,
        "- Verification scope: worktree",
        "- Verification scope: main_repo",
    );
    replace_first_exact_line(
        &mut text,
        "- Verification scope: pr_branch",
        "- Verification scope: main_repo",
    );
    replace_first_prefixed_line(
        &mut text,
        "- Worktree-only paths remaining:",
        "- Worktree-only paths remaining: none",
    );
    fs::write(path, text)?;
    Ok(())
}

fn normalize_closed_completed_stp(path: &Path) -> Result<()> {
    let mut text = fs::read_to_string(path)?;
    replace_field_line_in_text(&mut text, "status", "\"complete\"");
    fs::write(path, text)?;
    Ok(())
}

fn normalize_closed_completed_sip(path: &Path, issue_ref: &IssueRef) -> Result<()> {
    let mut text = fs::read_to_string(path)?;
    replace_field_line_in_text(&mut text, "Branch", &issue_ref.branch_name("codex"));
    replace_first_exact_line(&mut text, "- PR: none", "- PR:");
    replace_first_exact_line(
        &mut text,
        "- This issue is not started yet; do not assume a branch or worktree already exists.",
        "- This issue is closed/completed; implementation branch/worktree lifecycle is finished.",
    );
    replace_first_exact_line(
        &mut text,
        "- Do not run `pr start`; use the current issue-mode `pr run` flow only if execution later becomes necessary.",
        "- Do not run `pr start`; the issue has already completed its lifecycle.",
    );
    replace_first_exact_line(
        &mut text,
        "Prepare the linked issue prompt and review surfaces for truthful pre-run review before execution is bound.",
        "Preserve the closed/completed issue prompt and local card truth after closeout.",
    );
    replace_first_exact_line(
        &mut text,
        "- Preserve truthful lifecycle state until `pr run` binds the branch and worktree.",
        "- Preserve truthful closed/completed lifecycle state after merge and closeout.",
    );
    replace_first_exact_line(
        &mut text,
        "- The card bundle does not imply a branch or worktree exists before `pr run`.",
        "- The card bundle records the completed issue branch and no longer claims pre-run state.",
    );
    fs::write(path, text)?;
    Ok(())
}

fn validate_closed_completed_stp(repo_root: &Path, stp_path: &Path) -> Result<()> {
    let validator = repo_root.join("adl/tools/validate_structured_prompt.sh");
    run_status(
        "bash",
        &[
            path_str(&validator)?,
            "--type",
            "stp",
            "--input",
            path_str(stp_path)?,
        ],
    )
    .with_context(|| {
        format!(
            "closeout: stp failed closed/completed validation: {}",
            stp_path.display()
        )
    })
}

fn validate_closed_completed_sip(repo_root: &Path, sip_path: &Path) -> Result<()> {
    let validator = repo_root.join("adl/tools/validate_structured_prompt.sh");
    run_status(
        "bash",
        &[
            path_str(&validator)?,
            "--type",
            "sip",
            "--input",
            path_str(sip_path)?,
        ],
    )
    .with_context(|| {
        format!(
            "closeout: sip failed closed/completed validation: {}",
            sip_path.display()
        )
    })
}

fn ensure_canonical_output_is_local_only(
    repo_root: &Path,
    canonical_output: &Path,
    context: &str,
) -> Result<()> {
    let Ok(repo_relative) = canonical_output.strip_prefix(repo_root) else {
        return Ok(());
    };
    let repo_relative = repo_relative.to_string_lossy().into_owned();
    let status = Command::new("git")
        .args([
            "-C",
            path_str(repo_root)?,
            "ls-files",
            "--error-unmatch",
            "--",
            &repo_relative,
        ])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .context("failed to spawn 'git'")?;
    if status.success() {
        bail!(
            "{context}: '{}' is still tracked in git. Untrack canonical .adl issue surfaces before lifecycle normalization.",
            repo_relative
        );
    }
    Ok(())
}

fn replace_field_line_in_text(text: &mut String, label: &str, value: &str) {
    let prefix = format!("{label}:");
    let mut out = Vec::new();
    for line in text.lines() {
        if line.starts_with(&prefix) {
            out.push(format!("{prefix} {value}"));
        } else {
            out.push(line.to_string());
        }
    }
    *text = out.join("\n");
    if !text.ends_with('\n') {
        text.push('\n');
    }
}

fn replace_first_exact_line(text: &mut String, from: &str, to: &str) {
    let mut out = Vec::new();
    let mut replaced = false;
    for line in text.lines() {
        if !replaced && line == from {
            out.push(to.to_string());
            replaced = true;
        } else {
            out.push(line.to_string());
        }
    }
    *text = out.join("\n");
    if !text.ends_with('\n') {
        text.push('\n');
    }
}

fn replace_first_prefixed_line(text: &mut String, prefix: &str, to: &str) {
    let mut out = Vec::new();
    let mut replaced = false;
    for line in text.lines() {
        if !replaced && line.starts_with(prefix) {
            out.push(to.to_string());
            replaced = true;
        } else {
            out.push(line.to_string());
        }
    }
    *text = out.join("\n");
    if !text.ends_with('\n') {
        text.push('\n');
    }
}

pub(super) fn sync_completed_output_surfaces(
    repo_root: &Path,
    primary_root: &Path,
    issue_ref: &IssueRef,
    completed_output_path: &Path,
) -> Result<PathBuf> {
    let normalized_output_path = if completed_output_path.is_absolute() {
        completed_output_path.to_path_buf()
    } else {
        repo_root.join(completed_output_path)
    };
    let canonical_root_output = issue_ref.task_bundle_output_path(primary_root);
    let copied_to_root =
        !(same_filesystem_target(&normalized_output_path, &canonical_root_output)?);
    if copied_to_root {
        ensure_canonical_output_is_local_only(
            primary_root,
            &canonical_root_output,
            "finish: canonical .adl output surfaces must remain local-only during output sync",
        )?;
        if let Some(parent) = canonical_root_output.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::copy(&normalized_output_path, &canonical_root_output).with_context(|| {
            format!(
                "finish: failed to sync completed output card '{}' to canonical local task bundle '{}'",
                normalized_output_path.display(),
                canonical_root_output.display()
            )
        })?;
        validate_completed_sor(repo_root, &canonical_root_output)?;
    }

    let cards_root = resolve_cards_root(primary_root, None);
    let review_output = card_output_path(&cards_root, issue_ref.issue_number());
    ensure_symlink(&review_output, &canonical_root_output)?;
    Ok(canonical_root_output)
}

fn same_filesystem_target(left: &Path, right: &Path) -> Result<bool> {
    if left == right {
        return Ok(true);
    }
    if left.exists() && right.exists() {
        let left_canonical = fs::canonicalize(left)?;
        let right_canonical = fs::canonicalize(right)?;
        return Ok(left_canonical == right_canonical);
    }
    Ok(false)
}

fn prune_issue_worktree(repo_root: &Path, primary_root: &Path, issue_ref: &IssueRef) -> Result<()> {
    let worktree_path = issue_ref.default_worktree_path(
        primary_root,
        std::env::var_os("ADL_WORKTREE_ROOT")
            .map(PathBuf::from)
            .as_deref(),
    );
    if !worktree_path.is_dir() {
        return Ok(());
    }
    if has_uncommitted_changes(&worktree_path)? {
        bail!(
            "closeout: refusing to prune dirty worktree '{}'",
            worktree_path.display()
        );
    }
    let current_dir = env::current_dir().context("closeout: determine current directory")?;
    if current_dir.starts_with(&worktree_path) {
        env::set_current_dir(primary_root).with_context(|| {
            format!(
                "closeout: failed to leave worktree '{}' before pruning",
                worktree_path.display()
            )
        })?;
    }
    run_status(
        "git",
        &[
            "-C",
            path_str(repo_root)?,
            "worktree",
            "remove",
            path_str(&worktree_path)?,
        ],
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::tests::env_lock;
    use std::env;
    use std::os::unix::fs::PermissionsExt;
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
        assert!(rendered.contains("SOR Integration state expected 'merged' but found 'pr_open'"));
        assert!(
            rendered.contains("SOR Verification scope expected 'main_repo' but found 'worktree'")
        );
        assert!(rendered.contains(
            "SOR Worktree-only paths remaining expected 'none' but found 'adl/src/foo.rs'"
        ));
        assert!(rendered.contains("STP status expected '\"complete\"' but found '\"draft\"'"));
        assert!(rendered.contains("SIP Branch expected 'codex/1410-canonical-slug'"));
        assert!(rendered.contains("SIP still contains pre-run lifecycle wording"));
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

        prune_issue_worktree(&repo, &repo, &issue_ref).expect("missing worktree is fine");
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
}
