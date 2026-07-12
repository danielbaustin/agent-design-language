use std::fs;
use std::path::Path;
use std::process::Command;

use crate::error::{ErrorCode, Result, V2Error};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitOutput {
    pub stdout: String,
    pub stderr: String,
}

pub fn run(root: &Path, args: &[&str]) -> Result<GitOutput> {
    let output = Command::new("git")
        .current_dir(root)
        .args(args)
        .output()
        .map_err(|error| V2Error::new(ErrorCode::GitFailure, error.to_string()))?;
    if !output.status.success() {
        return Err(V2Error::new(
            ErrorCode::GitFailure,
            format!(
                "git {:?} failed: {}",
                args,
                String::from_utf8_lossy(&output.stderr).trim()
            ),
        ));
    }
    Ok(GitOutput {
        stdout: String::from_utf8_lossy(&output.stdout).trim().into(),
        stderr: String::from_utf8_lossy(&output.stderr).trim().into(),
    })
}

pub fn current_branch(root: &Path) -> Result<String> {
    Ok(run(root, &["branch", "--show-current"])?.stdout)
}

pub fn worktrees(root: &Path) -> Result<Vec<(String, String)>> {
    let text = run(root, &["worktree", "list", "--porcelain"])?.stdout;
    let mut result = Vec::new();
    let mut path = None;
    for line in text.lines().chain(std::iter::once("")) {
        if let Some(value) = line.strip_prefix("worktree ") {
            path = Some(value.to_owned());
        }
        if let (Some(branch), Some(path)) = (line.strip_prefix("branch refs/heads/"), path.as_ref())
        {
            result.push((branch.to_owned(), path.clone()));
        }
    }
    Ok(result)
}

pub fn substantive_revision(root: &Path, scope: &[String]) -> Result<String> {
    if scope.is_empty() {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "revision scope is empty",
        ));
    }
    let head = run(root, &["rev-parse", "HEAD"])?.stdout;
    let mut hasher = blake3::Hasher::new();
    hasher.update(head.as_bytes());
    let mut diff = Command::new("git");
    diff.current_dir(root).args([
        "diff",
        "--no-ext-diff",
        "--binary",
        "HEAD",
        "--",
        ".",
        ":(exclude).csdlc/**",
    ]);
    let output = diff
        .output()
        .map_err(|e| V2Error::new(ErrorCode::GitFailure, e.to_string()))?;
    if !output.status.success() {
        return Err(V2Error::new(
            ErrorCode::GitFailure,
            String::from_utf8_lossy(&output.stderr),
        ));
    }
    hasher.update(&output.stdout);
    let mut others = Command::new("git");
    others.current_dir(root).args([
        "ls-files",
        "--others",
        "--exclude-standard",
        "--",
        ".",
        ":(exclude).csdlc/**",
    ]);
    let output = others
        .output()
        .map_err(|e| V2Error::new(ErrorCode::GitFailure, e.to_string()))?;
    if !output.status.success() {
        return Err(V2Error::new(
            ErrorCode::GitFailure,
            String::from_utf8_lossy(&output.stderr),
        ));
    }
    let mut paths: Vec<_> = String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(str::to_owned)
        .collect();
    paths.sort();
    for path in paths {
        hasher.update(path.as_bytes());
        hasher.update(&fs::read(root.join(path))?);
    }
    Ok(format!("git-blake3:{head}:{}", hasher.finalize().to_hex()))
}

pub fn clean_commit_revision(commit: &str) -> String {
    let mut hasher = blake3::Hasher::new();
    hasher.update(commit.as_bytes());
    format!("git-blake3:{commit}:{}", hasher.finalize().to_hex())
}

pub fn metadata_only_changed_paths(
    root: &Path,
    from_commit: &str,
    to_commit: &str,
) -> Result<Vec<String>> {
    let output = Command::new("git")
        .current_dir(root)
        .args([
            "diff",
            "--name-only",
            "--no-renames",
            from_commit,
            to_commit,
            "--",
        ])
        .output()
        .map_err(|e| V2Error::new(ErrorCode::GitFailure, e.to_string()))?;
    if !output.status.success() {
        return Err(V2Error::new(
            ErrorCode::GitFailure,
            String::from_utf8_lossy(&output.stderr),
        ));
    }
    let mut paths: Vec<_> = String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(str::to_owned)
        .collect();
    paths.sort();
    if paths.iter().any(|p| !safe_metadata_path(p)) {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "revision includes substantive paths",
        ));
    }
    Ok(paths)
}
fn safe_metadata_path(path: &str) -> bool {
    let value = Path::new(path);
    value
        .components()
        .all(|c| matches!(c, std::path::Component::Normal(_)))
        && (path.starts_with(".csdlc/review/") || path.starts_with(".csdlc/evidence/"))
}
