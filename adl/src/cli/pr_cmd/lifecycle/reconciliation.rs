use super::super::*;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

fn reconcile_closed_completed_issue_bundle_with_recovery_sources(
    repo_root: &Path,
    issue_ref: &IssueRef,
    canonical_output: &Path,
    recovery_bundles: &[PathBuf],
) -> Result<()> {
    let bundle_dir = issue_ref.task_bundle_dir_path(repo_root);
    if let Some(parent) = bundle_dir.parent() {
        fs::create_dir_all(parent).with_context(|| {
            format!(
                "doctor: failed to create canonical task bundle parent '{}'",
                parent.display()
            )
        })?;
    }

    let mut duplicates = Vec::new();
    for recovery_bundle in recovery_bundles {
        if recovery_bundle.is_dir() && !duplicates.iter().any(|path| path == recovery_bundle) {
            duplicates.push(recovery_bundle.clone());
        }
    }
    for duplicate in matching_task_bundle_dirs(repo_root, issue_ref)? {
        if duplicate.is_dir() && !duplicates.iter().any(|path| path == &duplicate) {
            duplicates.push(duplicate);
        }
    }
    if !bundle_dir.exists() {
        fs::create_dir_all(&bundle_dir).with_context(|| {
            format!(
                "doctor: failed to create canonical task bundle '{}'",
                bundle_dir.display()
            )
        })?;
    }

    for relative in ["stp.md", "sip.md", "spp.md", "srp.md", "sor.md"] {
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

pub(crate) fn closeout_closed_completed_issue_bundle(
    repo_root: &Path,
    primary_root: &Path,
    issue_ref: &IssueRef,
    canonical_output: &Path,
) -> Result<()> {
    let worktree_path = issue_ref.default_worktree_path(
        primary_root,
        std::env::var_os("ADL_WORKTREE_ROOT")
            .map(PathBuf::from)
            .as_deref(),
    );
    let recovery_bundles = [issue_ref.task_bundle_dir_path(&worktree_path)];
    reconcile_closed_completed_issue_bundle_with_recovery_sources(
        primary_root,
        issue_ref,
        canonical_output,
        &recovery_bundles,
    )?;
    if worktree_path.is_dir() {
        super::cleanup::scrub_noncanonical_issue_bundle_residue(&worktree_path, issue_ref)?;
    }
    if worktree_path.is_dir() && has_uncommitted_or_untracked_changes(&worktree_path)? {
        let name = super::cleanup::worktree_display_name(&worktree_path);
        super::cleanup::record_worktree_prune_result(
            canonical_output,
            &format!("blocked_dirty: retained {name}"),
        )?;
        super::cleanup::replace_worktree_only_paths_remaining(
            canonical_output,
            &format!("issue worktree retained: {name}"),
        )?;
        bail!(
            "closeout: refusing to prune dirty worktree '{}' because it contains staged, unstaged, or untracked changes",
            worktree_path.display()
        );
    }
    let prune_result = super::cleanup::prune_issue_worktree(repo_root, primary_root, issue_ref)?;
    super::cleanup::record_worktree_prune_result(canonical_output, &prune_result.card_value())?;
    ensure_closed_completed_issue_bundle_truth(primary_root, issue_ref, canonical_output)
        .with_context(|| {
            format!(
                "closeout: canonical closed-issue sor truth drift remains for issue #{}",
                issue_ref.issue_number()
            )
        })
}

pub(crate) fn ensure_closed_completed_issue_bundle_truth(
    repo_root: &Path,
    issue_ref: &IssueRef,
    canonical_output: &Path,
) -> Result<()> {
    let bundle_dir = issue_ref.task_bundle_dir_path(repo_root);
    let mut sor_integration_state: Option<String> = None;

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
        sor_integration_state = line_value_after_prefix(&text, "- Integration state:");
        let branch = line_value_after_prefix(&text, "Branch:");
        let retrospective_no_branch = branch.as_deref() == Some("retrospective-no-branch");
        check_required_field(&text, "Status:", "DONE", "SOR Status", &mut mismatches);
        check_required_field(
            &text,
            "- Verification scope:",
            "main_repo",
            "SOR Verification scope",
            &mut mismatches,
        );
        match sor_integration_state.as_deref() {
            Some("merged") => {
                if retrospective_no_branch {
                    mismatches.push(
                        "SOR Integration state is 'merged' but Branch is 'retrospective-no-branch'; use 'closed_no_pr'"
                            .to_string(),
                    );
                }
            }
            Some("closed_no_pr") => {
                check_required_field(
                    &text,
                    "Branch:",
                    "retrospective-no-branch",
                    "SOR Branch for closed_no_pr",
                    &mut mismatches,
                );
            }
            Some(value) => mismatches.push(format!(
                "SOR Integration state expected 'merged' or 'closed_no_pr' but found '{value}'"
            )),
            None => mismatches.push("SOR Integration state is missing".to_string()),
        }
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
        let expected_branch = issue_ref.branch_name("codex");
        match sor_integration_state.as_deref() {
            Some("merged") => check_required_field(
                &text,
                "Branch:",
                &expected_branch,
                "SIP Branch",
                &mut mismatches,
            ),
            Some("closed_no_pr") => {
                let branch = line_value_after_prefix(&text, "Branch:").unwrap_or_default();
                if branch != expected_branch && branch != "retrospective-no-branch" {
                    mismatches.push(format!(
                        "SIP Branch expected '{}' or 'retrospective-no-branch' but found '{}'",
                        expected_branch, branch
                    ));
                }
            }
            _ => check_required_field(
                &text,
                "Branch:",
                &expected_branch,
                "SIP Branch",
                &mut mismatches,
            ),
        }
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

fn line_value_after_prefix(text: &str, prefix: &str) -> Option<String> {
    text.lines().find_map(|line| {
        line.strip_prefix(prefix)
            .map(|value| value.trim().to_string())
    })
}

pub(crate) fn matching_task_bundle_dirs(
    repo_root: &Path,
    issue_ref: &IssueRef,
) -> Result<Vec<PathBuf>> {
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

pub(crate) fn normalize_closed_completed_output_card(path: &Path) -> Result<()> {
    let mut text = fs::read_to_string(path)?;
    replace_field_line_in_text(&mut text, "Status", "DONE");
    let branch = line_value_after_prefix(&text, "Branch:");
    if branch.as_deref() == Some("retrospective-no-branch") {
        replace_first_exact_line(
            &mut text,
            "- Integration state: worktree_only | pr_open | merged",
            "- Integration state: closed_no_pr",
        );
        replace_first_exact_line(
            &mut text,
            "- Integration state: worktree_only",
            "- Integration state: closed_no_pr",
        );
        replace_first_exact_line(
            &mut text,
            "- Integration state: pr_open",
            "- Integration state: closed_no_pr",
        );
        replace_first_exact_line(
            &mut text,
            "- Integration state: merged",
            "- Integration state: closed_no_pr",
        );
    } else {
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
    }
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

pub(crate) fn normalize_closed_completed_stp(path: &Path) -> Result<()> {
    let mut text = fs::read_to_string(path)?;
    replace_field_line_in_text(&mut text, "status", "\"complete\"");
    fs::write(path, text)?;
    Ok(())
}

pub(crate) fn normalize_closed_completed_sip(path: &Path, issue_ref: &IssueRef) -> Result<()> {
    let mut text = fs::read_to_string(path)?;
    let branch = line_value_after_prefix(&text, "Branch:");
    if branch.as_deref() != Some("retrospective-no-branch") {
        replace_field_line_in_text(&mut text, "Branch", &issue_ref.branch_name("codex"));
    }
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
pub(crate) fn ensure_canonical_output_is_local_only(
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

pub(crate) fn replace_first_prefixed_line(text: &mut String, prefix: &str, to: &str) {
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

pub(crate) fn replace_or_insert_after_prefixed_line(
    text: &mut String,
    prefix: &str,
    insert_prefix: &str,
    to: &str,
) {
    let mut out = Vec::new();
    let mut replaced = false;
    let mut inserted = false;
    for line in text.lines() {
        if line.starts_with(insert_prefix) {
            if !replaced {
                out.push(to.to_string());
                replaced = true;
            }
            continue;
        }
        out.push(line.to_string());
        if !inserted && line.starts_with(prefix) && !replaced {
            out.push(to.to_string());
            inserted = true;
            replaced = true;
        }
    }
    if !replaced {
        out.push(to.to_string());
    }
    *text = out.join("\n");
    if !text.ends_with('\n') {
        text.push('\n');
    }
}
