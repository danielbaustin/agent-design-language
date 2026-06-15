use super::*;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

pub(super) fn repo_root() -> Result<PathBuf> {
    let current_top = PathBuf::from(run_capture("git", &["rev-parse", "--show-toplevel"])?.trim());
    Ok(current_top)
}

pub(super) fn primary_checkout_root() -> Result<PathBuf> {
    let current_top = repo_root()?;
    let common_dir = run_capture_allow_failure("git", &["rev-parse", "--git-common-dir"])?;
    let common_dir = common_dir
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(PathBuf::from);
    Ok(resolve_primary_checkout_root(
        &current_top,
        common_dir.as_deref(),
    ))
}

pub(super) fn path_str(path: &Path) -> Result<&str> {
    path.to_str()
        .ok_or_else(|| anyhow!("path must be valid utf-8: {}", path.display()))
}

pub(super) fn default_repo(repo_root: &Path) -> Result<String> {
    let remote = run_capture_allow_failure("git", &["remote", "get-url", "origin"])?;
    if let Some(url) = remote {
        if let Some(inferred) = infer_repo_from_remote(&url) {
            return Ok(inferred);
        }
    }

    let base = repo_root
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| anyhow!("failed to derive local repo name"))?;
    Ok(format!("local/{base}"))
}

pub(super) fn issue_create_repo(repo_root: &Path) -> Result<String> {
    let remote = run_capture_allow_failure(
        "git",
        &["-C", path_str(repo_root)?, "remote", "get-url", "origin"],
    )?;
    if let Some(url) = remote {
        if let Some(inferred) = infer_repo_from_remote(&url) {
            return Ok(inferred);
        }
    }

    bail!(
        "create: refusing to infer the GitHub issue target from ambient gh context; configure git origin with a GitHub owner/repo remote before running create"
    )
}

pub(super) fn infer_repo_from_remote(url: &str) -> Option<String> {
    let trimmed = url.trim();
    let candidate = trimmed
        .strip_prefix("https://github.com/")
        .or_else(|| trimmed.strip_prefix("git@github.com:"))
        .or_else(|| trimmed.strip_prefix("ssh://git@github.com/"))?;
    let candidate = candidate.strip_suffix(".git").unwrap_or(candidate);
    let mut parts = candidate.split('/');
    let owner = parts.next()?;
    let repo = parts.next()?;
    if owner.is_empty() || repo.is_empty() {
        return None;
    }
    Some(format!("{owner}/{repo}"))
}

pub(super) fn current_branch(repo_root: &Path) -> Result<String> {
    Ok(run_capture(
        "git",
        &[
            "-C",
            path_str(repo_root)?,
            "rev-parse",
            "--abbrev-ref",
            "HEAD",
        ],
    )?
    .trim()
    .to_string())
}

pub(super) fn ensure_not_on_main_branch(repo_root: &Path) -> Result<()> {
    let branch = current_branch(repo_root)?;
    if branch == "main" {
        bail!("finish: refusing to run on main");
    }
    Ok(())
}

pub(super) fn has_uncommitted_changes(repo_root: &Path) -> Result<bool> {
    let unstaged =
        run_status_allow_failure("git", &["-C", path_str(repo_root)?, "diff", "--quiet"])?;
    let staged = run_status_allow_failure(
        "git",
        &["-C", path_str(repo_root)?, "diff", "--cached", "--quiet"],
    )?;
    Ok(!(unstaged && staged))
}

pub(super) fn has_uncommitted_or_untracked_changes(repo_root: &Path) -> Result<bool> {
    let status = run_capture(
        "git",
        &[
            "-C",
            path_str(repo_root)?,
            "status",
            "--porcelain",
            "--untracked-files=all",
        ],
    )?;
    Ok(!status.trim().is_empty())
}

pub(super) fn tracked_changes_status(repo_root: &Path) -> Result<String> {
    run_capture(
        "git",
        &[
            "-C",
            path_str(repo_root)?,
            "status",
            "--porcelain",
            "--untracked-files=no",
        ],
    )
}

pub(super) fn commits_ahead_of_origin_main(repo_root: &Path) -> Result<usize> {
    let local_origin_main = run_status_allow_failure(
        "git",
        &[
            "-C",
            path_str(repo_root)?,
            "rev-parse",
            "--verify",
            "origin/main",
        ],
    )?;
    if !local_origin_main {
        return Ok(0);
    }
    let count = run_capture(
        "git",
        &[
            "-C",
            path_str(repo_root)?,
            "rev-list",
            "--count",
            "origin/main..HEAD",
        ],
    )?;
    Ok(count.trim().parse::<usize>().unwrap_or(0))
}

pub(super) fn fetch_origin_main_with_fallback() -> Result<()> {
    eprintln!("• Fetching origin/main…");
    let output = Command::new("git")
        .args(["fetch", "origin", "main"])
        .output()
        .with_context(|| "failed to spawn git fetch origin main")?;
    if output.status.success() {
        return Ok(());
    }
    if run_status_allow_failure("git", &["rev-parse", "--verify", "--quiet", "origin/main"])? {
        eprintln!("• Warning: start: fetch origin main failed; reusing existing local origin/main");
        let stderr = String::from_utf8_lossy(&output.stderr);
        if !stderr.trim().is_empty() {
            eprintln!("{stderr}");
        }
        return Ok(());
    }
    bail!("start: fetch origin main failed and origin/main is unavailable locally");
}

pub(super) fn ensure_git_metadata_writable() -> Result<()> {
    let git_dir = run_capture("git", &["rev-parse", "--git-common-dir"])?;
    let git_dir = git_dir.trim();
    let probe_dir = Path::new(git_dir).join(format!(
        "adl-git-write-probe-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos()
    ));
    match fs::create_dir(&probe_dir) {
        Ok(()) => {
            let _ = fs::remove_dir(&probe_dir);
            Ok(())
        }
        Err(err) => bail!(
            "start: git metadata directory '{}' is not writable, so branch/worktree creation cannot proceed. Remediation: restore write access to git metadata before rerunning. ({err})",
            git_dir
        ),
    }
}

pub(super) fn ensure_local_branch_exists(branch: &str) -> Result<()> {
    if run_status_allow_failure(
        "git",
        &[
            "show-ref",
            "--verify",
            "--quiet",
            &format!("refs/heads/{branch}"),
        ],
    )? {
        eprintln!("• Local branch exists; reusing: {branch}");
        return Ok(());
    }
    if run_status_allow_failure(
        "git",
        &["ls-remote", "--exit-code", "--heads", "origin", branch],
    )? {
        eprintln!("• Branch exists on origin; creating local tracking branch…");
        run_status(
            "git",
            &["branch", "--track", branch, &format!("origin/{branch}")],
        )?;
        return Ok(());
    }
    eprintln!("• Creating local branch from origin/main…");
    run_status("git", &["branch", branch, "origin/main"])?;
    Ok(())
}

pub(super) fn branch_checked_out_worktree_path(branch: &str) -> Result<Option<PathBuf>> {
    let primary_root = primary_checkout_root()?;
    let normalized_primary_root = normalize_existing_worktree_path(&primary_root);
    let canonical_worktrees_root = primary_root.join(".worktrees");
    let normalized_canonical_worktrees_root =
        normalize_existing_worktree_path(&canonical_worktrees_root);
    let managed_worktree_root = std::env::var_os("ADL_WORKTREE_ROOT")
        .map(PathBuf::from)
        .map(|path| {
            if path.is_absolute() {
                path
            } else {
                primary_root.join(path)
            }
        });
    let out = run_capture_allow_failure("git", &["worktree", "list", "--porcelain"])?;
    let Some(out) = out else { return Ok(None) };
    let mut current_worktree: Option<PathBuf> = None;
    for line in out.lines() {
        if let Some(path) = line.strip_prefix("worktree ") {
            current_worktree = Some(PathBuf::from(path.trim()));
        } else if let Some(head_branch) = line.strip_prefix("branch refs/heads/") {
            if head_branch.trim() == branch {
                let Some(candidate) = current_worktree.clone() else {
                    continue;
                };
                let normalized_candidate = normalize_existing_worktree_path(&candidate);
                let managed_root_matches = managed_worktree_root.as_ref().is_some_and(|root| {
                    let normalized_root = normalize_existing_worktree_path(root);
                    candidate.starts_with(root)
                        || normalized_candidate.starts_with(&normalized_root)
                });
                if candidate == primary_root
                    || normalized_candidate == normalized_primary_root
                    || candidate.starts_with(&canonical_worktrees_root)
                    || normalized_candidate.starts_with(&normalized_canonical_worktrees_root)
                    || managed_root_matches
                {
                    return Ok(Some(candidate));
                }
                bail!(
                    "start: branch '{}' is already checked out in non-canonical worktree '{}'. ADL only permits the primary checkout and repo-local '.worktrees/' worktrees. Remediation: remove or migrate the external worktree before rerunning.",
                    branch,
                    candidate.display()
                );
            }
        }
    }
    Ok(None)
}

fn normalize_existing_worktree_path(path: &Path) -> PathBuf {
    fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf())
}

pub(super) fn ensure_worktree_for_branch(worktree_path: &Path, branch: &str) -> Result<()> {
    if let Some(existing) = branch_checked_out_worktree_path(branch)? {
        if existing == worktree_path {
            ensure_reusable_worktree_is_clean(&existing, branch)?;
            eprintln!(
                "• Reusing existing worktree for branch: {}",
                worktree_path.display()
            );
            return Ok(());
        }
        bail!(
            "start: branch '{}' is already checked out in worktree '{}'. Remediation: run commands there or remove it with 'git worktree remove \"{}\"'.",
            branch,
            existing.display(),
            existing.display()
        );
    }
    if !worktree_path.exists() {
        eprintln!("• Creating worktree: {}", worktree_path.display());
        if let Some(parent) = worktree_path.parent() {
            fs::create_dir_all(parent)?;
        }
        run_status(
            "git",
            &["worktree", "add", path_str(worktree_path)?, branch],
        )?;
        return Ok(());
    }
    bail!(
        "start: unsafe_existing_worktree_path: target worktree path '{}' already exists but is not the clean checked-out worktree for branch '{}'. Remediation: preserve any local changes, then prune/remove the stale path before rerunning.",
        worktree_path.display(),
        branch
    )
}

fn ensure_reusable_worktree_is_clean(worktree_path: &Path, branch: &str) -> Result<()> {
    let status = run_capture(
        "git",
        &[
            "-C",
            path_str(worktree_path)?,
            "status",
            "--porcelain",
            "--untracked-files=all",
        ],
    )?;
    if status.trim().is_empty() {
        return Ok(());
    }

    let preview = status
        .lines()
        .take(8)
        .map(str::trim_end)
        .collect::<Vec<_>>()
        .join("\\n");
    bail!(
        "start: unsafe_existing_worktree_dirty: refusing to reuse worktree '{}' for branch '{}' because it has staged, unstaged, or untracked changes. Remediation: preserve or resolve those changes, then rerun; do not reset or prune until the dirty state is accounted for. Dirty status:\\n{}",
        worktree_path.display(),
        branch,
        preview
    )
}

pub(super) fn run_capture(program: &str, args: &[&str]) -> Result<String> {
    let output = Command::new(program)
        .args(args)
        .output()
        .with_context(|| format!("failed to spawn '{program}'"))?;
    if !output.status.success() {
        bail!(
            "{program} failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

pub(super) fn run_capture_allow_failure(program: &str, args: &[&str]) -> Result<Option<String>> {
    let output = Command::new(program)
        .args(args)
        .output()
        .with_context(|| format!("failed to spawn '{program}'"))?;
    if output.status.success() {
        Ok(Some(String::from_utf8_lossy(&output.stdout).to_string()))
    } else {
        Ok(None)
    }
}

pub(super) fn run_status(program: &str, args: &[&str]) -> Result<()> {
    let status = Command::new(program)
        .args(args)
        .status()
        .with_context(|| format!("failed to spawn '{program}'"))?;
    if !status.success() {
        bail!("{program} failed with status {:?}", status.code());
    }
    Ok(())
}

pub(super) fn run_status_allow_failure(program: &str, args: &[&str]) -> Result<bool> {
    let status = Command::new(program)
        .args(args)
        .status()
        .with_context(|| format!("failed to spawn '{program}'"))?;
    Ok(status.success())
}
