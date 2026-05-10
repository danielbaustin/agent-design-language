use super::super::*;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

pub(crate) fn sync_completed_output_surfaces(
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
        super::reconciliation::ensure_canonical_output_is_local_only(
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
        super::super::validate_completed_sor(repo_root, &canonical_root_output)?;
    }

    let cards_root = resolve_cards_root(primary_root, None);
    let review_output = card_output_path(&cards_root, issue_ref.issue_number());
    ensure_symlink(&review_output, &canonical_root_output)?;
    Ok(canonical_root_output)
}

pub(crate) fn same_filesystem_target(left: &Path, right: &Path) -> Result<bool> {
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum IssueWorktreePruneResult {
    Missing(String),
    Pruned(String),
}

impl IssueWorktreePruneResult {
    pub(crate) fn card_value(&self) -> String {
        match self {
            Self::Missing(name) => format!("skipped_missing: {name}"),
            Self::Pruned(name) => format!("pruned: {name}"),
        }
    }
}

pub(crate) fn worktree_display_name(path: &Path) -> String {
    path.file_name()
        .map(|name| name.to_string_lossy().to_string())
        .filter(|name| !name.trim().is_empty())
        .unwrap_or_else(|| "issue worktree".to_string())
}

pub(crate) fn record_worktree_prune_result(path: &Path, result: &str) -> Result<()> {
    let mut text = fs::read_to_string(path)?;
    super::reconciliation::replace_or_insert_after_prefixed_line(
        &mut text,
        "- Worktree-only paths remaining:",
        "- Worktree prune result:",
        &format!("- Worktree prune result: {result}"),
    );
    fs::write(path, text)?;
    Ok(())
}

pub(crate) fn replace_worktree_only_paths_remaining(path: &Path, value: &str) -> Result<()> {
    let mut text = fs::read_to_string(path)?;
    super::reconciliation::replace_first_prefixed_line(
        &mut text,
        "- Worktree-only paths remaining:",
        &format!("- Worktree-only paths remaining: {value}"),
    );
    fs::write(path, text)?;
    Ok(())
}

pub(crate) fn scrub_noncanonical_issue_bundle_residue(
    worktree_root: &Path,
    issue_ref: &IssueRef,
) -> Result<()> {
    let adl_root = worktree_root.join(".adl");
    if !adl_root.is_dir() {
        return Ok(());
    }

    let canonical_body = issue_ref.issue_prompt_path(worktree_root);
    let canonical_bundle = issue_ref.task_bundle_dir_path(worktree_root);
    let body_prefix = format!("issue-{:04}-", issue_ref.issue_number());
    let task_prefix = format!("issue-{:04}__", issue_ref.issue_number());

    for scope_entry in fs::read_dir(&adl_root)? {
        let scope_entry = scope_entry?;
        let scope_path = scope_entry.path();

        let bodies = scope_path.join("bodies");
        if bodies.is_dir() {
            for entry in fs::read_dir(&bodies)? {
                let entry = entry?;
                let path = entry.path();
                let name = entry.file_name().to_string_lossy().to_string();
                if name.starts_with("issue-")
                    && (name.starts_with(&body_prefix) && path != canonical_body
                        || !name.starts_with(&body_prefix))
                {
                    fs::remove_file(&path).with_context(|| {
                        format!(
                            "closeout: failed to scrub noncanonical local issue prompt residue '{}'",
                            path.display()
                        )
                    })?;
                }
            }
        }

        let tasks = scope_path.join("tasks");
        if tasks.is_dir() {
            for entry in fs::read_dir(&tasks)? {
                let entry = entry?;
                let path = entry.path();
                let name = entry.file_name().to_string_lossy().to_string();
                if name.starts_with("issue-")
                    && (name.starts_with(&task_prefix) && path != canonical_bundle
                        || !name.starts_with(&task_prefix))
                {
                    fs::remove_dir_all(&path).with_context(|| {
                        format!(
                            "closeout: failed to scrub noncanonical local task-bundle residue '{}'",
                            path.display()
                        )
                    })?;
                }
            }
        }
    }

    Ok(())
}

pub(crate) fn prune_issue_worktree(
    repo_root: &Path,
    primary_root: &Path,
    issue_ref: &IssueRef,
) -> Result<IssueWorktreePruneResult> {
    let worktree_path = issue_ref.default_worktree_path(
        primary_root,
        std::env::var_os("ADL_WORKTREE_ROOT")
            .map(PathBuf::from)
            .as_deref(),
    );
    if !worktree_path.is_dir() {
        println!(
            "closeout: issue worktree already absent; prune not needed: {}",
            worktree_path.display()
        );
        return Ok(IssueWorktreePruneResult::Missing(worktree_display_name(
            &worktree_path,
        )));
    }
    if has_uncommitted_or_untracked_changes(&worktree_path)? {
        bail!(
            "closeout: refusing to prune dirty worktree '{}' because it contains staged, unstaged, or untracked changes",
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
    println!(
        "closeout: pruned issue worktree: {}",
        worktree_path.display()
    );
    Ok(IssueWorktreePruneResult::Pruned(worktree_display_name(
        &worktree_path,
    )))
}
