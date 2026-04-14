use anyhow::{anyhow, Context, Result};
use std::path::{Path, PathBuf};
use std::process::Command;

use ::adl::chronosense::default_identity_profile_path;

pub(super) fn required_value(args: &[String], index: usize, flag: &str) -> Result<String> {
    args.get(index + 1)
        .cloned()
        .ok_or_else(|| anyhow!("{flag} requires a value"))
}

pub(super) fn resolve_identity_path(repo_root: &Path, args: &[String]) -> Result<PathBuf> {
    let mut path: Option<PathBuf> = None;
    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--path" => {
                path = Some(PathBuf::from(required_value(args, i, "--path")?));
                i += 1;
            }
            "--help" | "-h" => {
                println!("{}", super::super::usage::usage());
                std::process::exit(0);
            }
            other => return Err(anyhow!("unknown arg for identity show: {other}")),
        }
        i += 1;
    }
    Ok(path.unwrap_or_else(|| default_identity_profile_path(repo_root)))
}

pub(super) fn run_git_capture(args: &[&str]) -> Result<String> {
    let output = Command::new("git")
        .args(args)
        .output()
        .with_context(|| format!("failed to invoke git {}", args.join(" ")))?;
    if !output.status.success() {
        return Err(anyhow!(
            "git {} failed with status {}",
            args.join(" "),
            output.status
        ));
    }
    String::from_utf8(output.stdout).context("git output was not valid UTF-8")
}

pub(super) fn repo_root() -> Result<PathBuf> {
    Ok(PathBuf::from(
        run_git_capture(&["rev-parse", "--show-toplevel"])?
            .trim()
            .to_string(),
    ))
}
