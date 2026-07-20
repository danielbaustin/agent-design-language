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
    let pathspec = scoped_pathspec(scope);
    let mut diff = Command::new("git");
    diff.current_dir(root)
        .args(["diff", "--no-ext-diff", "--binary", "HEAD", "--"])
        .args(&pathspec);
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
    others
        .current_dir(root)
        .args(["ls-files", "--others", "--exclude-standard", "--"])
        .args(&pathspec);
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

fn scoped_pathspec(scope: &[String]) -> Vec<String> {
    let mut pathspec = scope.to_vec();
    pathspec.push(":(exclude).csdlc/**".into());
    pathspec
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
    validate_card_projection_commits(root, from_commit, to_commit)?;
    Ok(paths)
}

fn validate_card_projection_commits(root: &Path, from_commit: &str, to_commit: &str) -> Result<()> {
    let output = Command::new("git")
        .current_dir(root)
        .args([
            "rev-list",
            "--reverse",
            &format!("{from_commit}..{to_commit}"),
        ])
        .output()
        .map_err(|e| V2Error::new(ErrorCode::GitFailure, e.to_string()))?;
    if !output.status.success() {
        return Err(V2Error::new(
            ErrorCode::GitFailure,
            String::from_utf8_lossy(&output.stderr),
        ));
    }
    for commit in String::from_utf8_lossy(&output.stdout).lines() {
        let paths = commit_changed_paths(root, commit)?;
        for path in paths.iter().filter(|path| is_card_markdown(path)) {
            let values = format!("{}.values.json", path.trim_end_matches(".md"));
            if !paths.iter().any(|candidate| candidate == &values) {
                return Err(V2Error::new(
                    ErrorCode::InvalidInput,
                    "card prose changed without its typed values projection",
                ));
            }
        }
    }
    Ok(())
}

fn commit_changed_paths(root: &Path, commit: &str) -> Result<Vec<String>> {
    let output = Command::new("git")
        .current_dir(root)
        .args([
            "diff-tree",
            "--no-commit-id",
            "--name-only",
            "-r",
            "-m",
            commit,
        ])
        .output()
        .map_err(|e| V2Error::new(ErrorCode::GitFailure, e.to_string()))?;
    if !output.status.success() {
        return Err(V2Error::new(
            ErrorCode::GitFailure,
            String::from_utf8_lossy(&output.stderr),
        ));
    }
    Ok(String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(str::to_owned)
        .collect())
}

fn is_card_markdown(path: &str) -> bool {
    let Some(name) = path.strip_prefix(".csdlc/issues/") else {
        return false;
    };
    let Some((issue, card)) = name.split_once("/cards/") else {
        return false;
    };
    issue.bytes().all(|byte| byte.is_ascii_digit())
        && matches!(
            card,
            "sip.md" | "stp.md" | "spp.md" | "vpp.md" | "srp.md" | "sor.md"
        )
}

fn safe_metadata_path(path: &str) -> bool {
    let value = Path::new(path);
    if !value
        .components()
        .all(|c| matches!(c, std::path::Component::Normal(_)))
    {
        return false;
    }
    if path.starts_with(".csdlc/review/") || path.starts_with(".csdlc/evidence/") {
        return true;
    }
    let parts: Vec<_> = path.split('/').collect();
    let issue_id = |value: &str| !value.is_empty() && value.bytes().all(|b| b.is_ascii_digit());
    match parts.as_slice() {
        [".csdlc", "issues", issue, file]
            if issue_id(issue) && matches!(*file, "index.json" | "audit.jsonl") =>
        {
            true
        }
        [".csdlc", "issues", issue, "cards", file] if issue_id(issue) => {
            let names = ["sip", "stp", "spp", "vpp", "srp", "sor"];
            names
                .iter()
                .any(|name| *file == format!("{name}.md") || *file == format!("{name}.values.json"))
        }
        [".csdlc", "prepared", "issues", issue, file]
            if issue_id(issue) && file.ends_with(".json") =>
        {
            true
        }
        [".csdlc", "requests", file]
            if file
                .split_once('-')
                .is_some_and(|(issue, suffix)| issue_id(issue) && suffix.ends_with(".json")) =>
        {
            true
        }
        [".csdlc", "publication", file]
            if file.strip_suffix(".intent.json").is_some_and(issue_id) =>
        {
            true
        }
        _ => false,
    }
}
