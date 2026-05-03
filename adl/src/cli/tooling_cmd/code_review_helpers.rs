use anyhow::{ensure, Context, Result};
use serde::Serialize;
use std::io::Read;
use std::path::Path;
use std::process::{Command, Stdio};

pub(crate) fn safe_read_worktree_file(
    root: &Path,
    canonical_root: &Path,
    file: &str,
    max_bytes: usize,
) -> Result<String> {
    let candidate = root.join(file);
    let canonical_candidate = candidate
        .canonicalize()
        .with_context(|| format!("canonicalize review file '{}'", candidate.display()))?;
    ensure!(
        canonical_candidate.starts_with(canonical_root),
        "review file resolves outside repo root"
    );
    read_file_prefix(&canonical_candidate, max_bytes).context("read review file")
}

pub(crate) fn git_show_file_prefix(
    root: &Path,
    head_ref: &str,
    file: &str,
    max_bytes: usize,
) -> Result<String> {
    let spec = format!("{head_ref}:{file}");
    let mut child = Command::new("git")
        .args(["show", "--end-of-options", &spec])
        .current_dir(root)
        .stdout(Stdio::piped())
        .spawn()
        .with_context(|| format!("spawn git show for '{file}'"))?;
    let mut stdout = child.stdout.take().context("capture git show stdout")?;
    let mut bytes = Vec::new();
    stdout
        .by_ref()
        .take(max_bytes as u64)
        .read_to_end(&mut bytes)
        .context("read git show stdout")?;
    drop(stdout);
    let status = child.wait().context("wait for git show")?;
    ensure!(
        status.success() || !bytes.is_empty(),
        "git show failed for '{file}'"
    );
    Ok(String::from_utf8_lossy(&bytes).to_string())
}

pub(crate) fn read_file_prefix(path: &Path, max_bytes: usize) -> Result<String> {
    let file = std::fs::File::open(path)?;
    let mut bytes = Vec::new();
    file.take(max_bytes as u64).read_to_end(&mut bytes)?;
    Ok(String::from_utf8_lossy(&bytes).to_string())
}

pub(crate) fn truncate(text: &str, max_bytes: usize) -> (String, bool) {
    if text.len() <= max_bytes {
        return (text.to_string(), false);
    }
    let end = text
        .char_indices()
        .map(|(idx, _)| idx)
        .chain(std::iter::once(text.len()))
        .take_while(|idx| *idx <= max_bytes)
        .last()
        .unwrap_or(0);
    (text[..end].to_string(), true)
}

pub(crate) fn git_output(root: &Path, args: &[&str]) -> Result<String> {
    let output = Command::new("git").args(args).current_dir(root).output()?;
    if !output.status.success() {
        anyhow::bail!("git command failed: git {}", args.join(" "));
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

pub(crate) fn write_json(path: &Path, value: &impl Serialize) -> Result<()> {
    let bytes = serde_json::to_vec_pretty(value)?;
    std::fs::write(path, bytes)?;
    Ok(())
}
