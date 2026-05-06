use anyhow::{anyhow, bail, Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[cfg(unix)]
use std::os::unix::fs as unix_fs;

pub(crate) fn field_line_value(path: &Path, label: &str) -> Result<String> {
    let prefix = format!("{label}:");
    let text = fs::read_to_string(path)?;
    for line in text.lines() {
        if let Some(rest) = line.strip_prefix(&prefix) {
            return Ok(rest.trim().to_string());
        }
    }
    Ok(String::new())
}

pub(crate) fn branch_indicates_unbound_state(recorded: &str) -> bool {
    let recorded = recorded.trim();
    recorded.is_empty()
        || recorded.eq_ignore_ascii_case("not bound yet")
        || recorded.starts_with("TBD (run pr.sh start ")
        || recorded.starts_with("TBD (run pr.sh run ")
}

pub(crate) fn output_card_title_matches_slug(path: &Path, slug: &str) -> Result<bool> {
    let expected = format!("# {slug}");
    let text = fs::read_to_string(path)?;
    let header = text
        .lines()
        .find(|line| line.starts_with("# "))
        .unwrap_or_default()
        .trim()
        .to_string();
    Ok(header == expected)
}

pub(crate) fn path_relative_to_repo(repo_root: &Path, path: &Path) -> String {
    path.strip_prefix(repo_root)
        .map(|relative| relative.display().to_string())
        .unwrap_or_else(|_| path.display().to_string())
}

pub(crate) fn copy_directory_contents(source: &Path, target: &Path) -> Result<()> {
    fs::create_dir_all(target)?;
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let source_path = entry.path();
        let target_path = target.join(entry.file_name());
        let file_type = entry.file_type()?;
        if file_type.is_dir() {
            copy_directory_contents(&source_path, &target_path)?;
        } else if file_type.is_file() {
            if let Some(parent) = target_path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(&source_path, &target_path)?;
        }
    }
    Ok(())
}

pub(crate) fn ensure_symlink(link_path: &Path, target: &Path) -> Result<()> {
    if let Some(parent) = link_path.parent() {
        fs::create_dir_all(parent)?;
    }
    if link_path.exists() || link_path.symlink_metadata().is_ok() {
        let _ = fs::remove_file(link_path);
    }
    #[cfg(unix)]
    {
        unix_fs::symlink(target, link_path)?;
    }
    #[cfg(not(unix))]
    {
        fs::copy(target, link_path)?;
    }
    Ok(())
}

pub(crate) fn default_repo(repo_root: &Path) -> Result<String> {
    let origin_url = run_capture_allow_failure(
        "git",
        &["-C", path_str(repo_root)?, "remote", "get-url", "origin"],
    )?
    .unwrap_or_default();
    if let Some(inferred) = infer_repo_from_remote(origin_url.trim()) {
        return Ok(inferred);
    }

    let inferred = run_capture_allow_failure(
        "gh",
        &[
            "repo",
            "view",
            "--json",
            "nameWithOwner",
            "--jq",
            ".nameWithOwner",
        ],
    )?
    .unwrap_or_default();
    let inferred = inferred.trim();
    if !inferred.is_empty() {
        return Ok(inferred);
    }

    let base = repo_root
        .file_name()
        .map(|value| value.to_string_lossy().to_string())
        .unwrap_or_else(|| "repo".to_string());
    Ok(format!("local/{base}"))
}

fn infer_repo_from_remote(url: &str) -> Option<String> {
    let trimmed = url.trim().trim_end_matches(".git");
    let marker = "github.com";
    let idx = trimmed.find(marker)?;
    let suffix = &trimmed[idx + marker.len()..];
    let suffix = suffix.trim_start_matches(':').trim_start_matches('/');
    let mut parts = suffix.split('/');
    let owner = parts.next()?;
    let repo = parts.next()?;
    Some(format!("{owner}/{repo}"))
}

pub(crate) fn replace_field_line(text: &mut String, label: &str, value: &str) {
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

pub(crate) fn replace_exact_line(text: &mut String, from: &str, to: &str) {
    let mut out = Vec::new();
    for line in text.lines() {
        if line == from {
            out.push(to.to_string());
        } else {
            out.push(line.to_string());
        }
    }
    *text = out.join("\n");
    if !text.ends_with('\n') {
        text.push('\n');
    }
}

pub(crate) fn remove_exact_line(text: &mut String, target: &str) {
    let mut out = Vec::new();
    for line in text.lines() {
        if line != target {
            out.push(line.to_string());
        }
    }
    *text = out.join("\n");
    if !text.ends_with('\n') {
        text.push('\n');
    }
}

pub(crate) fn deduplicate_exact_line(text: &mut String, target: &str) {
    let mut seen = false;
    let mut out = Vec::new();
    for line in text.lines() {
        if line == target {
            if seen {
                continue;
            }
            seen = true;
        }
        out.push(line.to_string());
    }
    *text = out.join("\n");
    if !text.ends_with('\n') {
        text.push('\n');
    }
}

pub(crate) fn replace_field_line_in_file(path: &Path, label: &str, value: &str) -> Result<()> {
    let mut text = fs::read_to_string(path)?;
    replace_field_line(&mut text, label, value);
    fs::write(path, text)?;
    Ok(())
}

pub(crate) fn path_str(path: &Path) -> Result<&str> {
    path.to_str()
        .ok_or_else(|| anyhow!("non-utf8 path: {}", path.display()))
}

pub(crate) fn run_capture_allow_failure(program: &str, args: &[&str]) -> Result<Option<String>> {
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

pub(crate) fn run_status(program: &str, args: &[&str]) -> Result<()> {
    let status = Command::new(program)
        .args(args)
        .status()
        .with_context(|| format!("failed to spawn '{program}'"))?;
    if !status.success() {
        bail!("{program} failed with status {:?}", status.code());
    }
    Ok(())
}

pub(crate) fn run_capture(program: &str, args: &[&str]) -> Result<String> {
    let output = Command::new(program)
        .args(args)
        .output()
        .with_context(|| format!("failed to spawn '{program}'"))?;
    if !output.status.success() {
        bail!("{program} failed");
    }
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

pub(crate) fn write_temp_markdown(prefix: &str, body: &str) -> Result<PathBuf> {
    let unique = format!(
        "{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos()
    );
    let path = std::env::temp_dir().join(format!("{prefix}-{unique}.md"));
    fs::write(&path, body)?;
    Ok(path)
}
