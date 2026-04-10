use super::*;
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};

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
        if let Some(existing) = duplicates.first() {
            fs::rename(existing, &bundle_dir).with_context(|| {
                format!(
                    "doctor: failed to reconcile duplicate task bundle '{}' into canonical '{}'",
                    existing.display(),
                    bundle_dir.display()
                )
            })?;
        } else {
            fs::create_dir_all(&bundle_dir)?;
        }
    }

    if !ensure_nonempty_file_path(canonical_output)? {
        for duplicate in matching_task_bundle_dirs(repo_root, issue_ref)? {
            if duplicate == bundle_dir {
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

    normalize_closed_completed_output_card(canonical_output)?;
    validate_completed_sor(repo_root, canonical_output)?;

    for duplicate in matching_task_bundle_dirs(repo_root, issue_ref)? {
        if duplicate != bundle_dir {
            fs::remove_dir_all(&duplicate).with_context(|| {
                format!(
                    "doctor: failed to remove duplicate closed task bundle '{}'",
                    duplicate.display()
                )
            })?;
        }
    }

    let cards_root = resolve_cards_root(repo_root, None);
    let review_output = card_output_path(&cards_root, issue_ref.issue_number());
    ensure_symlink(&review_output, canonical_output)?;
    Ok(())
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
        if let Some(parent) = canonical_root_output.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::copy(&normalized_output_path, &canonical_root_output).with_context(|| {
            format!(
                "finish: failed to sync completed output card '{}' to canonical root task bundle '{}'",
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
