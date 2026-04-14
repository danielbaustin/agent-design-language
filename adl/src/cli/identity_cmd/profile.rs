use anyhow::{anyhow, Context, Result};
use chrono::Utc;
use serde_json::to_string_pretty;
use std::fs;
use std::path::{Path, PathBuf};

use ::adl::chronosense::{
    default_identity_profile_path, load_identity_profile, write_identity_profile, IdentityProfile,
    TemporalContext,
};

use super::helpers::{required_value, resolve_identity_path};

pub(super) fn real_identity_init(repo_root: &Path, args: &[String]) -> Result<()> {
    let mut name: Option<String> = None;
    let mut agent_id = "codex".to_string();
    let mut birthday: Option<String> = None;
    let mut timezone: Option<String> = None;
    let mut created_by = "daniel".to_string();
    let mut path: Option<PathBuf> = None;
    let mut force = false;

    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--name" => {
                name = Some(required_value(args, i, "--name")?);
                i += 1;
            }
            "--agent-id" => {
                agent_id = required_value(args, i, "--agent-id")?;
                i += 1;
            }
            "--birthday" => {
                birthday = Some(required_value(args, i, "--birthday")?);
                i += 1;
            }
            "--timezone" => {
                timezone = Some(required_value(args, i, "--timezone")?);
                i += 1;
            }
            "--created-by" => {
                created_by = required_value(args, i, "--created-by")?;
                i += 1;
            }
            "--path" => {
                path = Some(PathBuf::from(required_value(args, i, "--path")?));
                i += 1;
            }
            "--force" => force = true,
            "--help" | "-h" => {
                println!("{}", super::super::usage::usage());
                return Ok(());
            }
            other => return Err(anyhow!("unknown arg for identity init: {other}")),
        }
        i += 1;
    }

    let profile_path = path.unwrap_or_else(|| default_identity_profile_path(repo_root));
    if profile_path.exists() && !force {
        return Err(anyhow!(
            "identity profile already exists at {} (pass --force to overwrite)",
            profile_path.display()
        ));
    }

    let profile = IdentityProfile::from_birthday(
        agent_id,
        name.ok_or_else(|| anyhow!("identity init requires --name <display-name>"))?,
        &birthday.ok_or_else(|| anyhow!("identity init requires --birthday <RFC3339>"))?,
        &timezone.ok_or_else(|| anyhow!("identity init requires --timezone <IANA>"))?,
        created_by,
    )?;
    write_identity_profile(&profile_path, &profile)?;

    println!("IDENTITY_PATH={}", profile_path.display());
    println!("IDENTITY_AGENT_ID={}", profile.agent_id);
    println!("IDENTITY_NAME={}", profile.display_name);
    println!("BIRTHDAY={}", profile.birthday_rfc3339);
    Ok(())
}

pub(super) fn real_identity_show(repo_root: &Path, args: &[String]) -> Result<()> {
    let profile_path = resolve_identity_path(repo_root, args)?;
    let profile = load_identity_profile(&profile_path)?;
    println!("{}", to_string_pretty(&profile)?);
    Ok(())
}

pub(super) fn real_identity_now(repo_root: &Path, args: &[String]) -> Result<()> {
    let mut timezone: Option<String> = None;
    let mut path: Option<PathBuf> = None;
    let mut out_path: Option<PathBuf> = None;

    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--timezone" => {
                timezone = Some(required_value(args, i, "--timezone")?);
                i += 1;
            }
            "--path" => {
                path = Some(PathBuf::from(required_value(args, i, "--path")?));
                i += 1;
            }
            "--out" => {
                out_path = Some(PathBuf::from(required_value(args, i, "--out")?));
                i += 1;
            }
            "--help" | "-h" => {
                println!("{}", super::super::usage::usage());
                return Ok(());
            }
            other => return Err(anyhow!("unknown arg for identity now: {other}")),
        }
        i += 1;
    }

    let profile_path = path.unwrap_or_else(|| default_identity_profile_path(repo_root));
    let identity = if profile_path.exists() {
        Some(load_identity_profile(&profile_path)?)
    } else {
        None
    };
    let timezone_name = timezone
        .or_else(|| {
            identity
                .as_ref()
                .map(|profile| profile.birth_timezone.clone())
        })
        .ok_or_else(|| anyhow!("identity now requires --timezone <IANA> when no profile exists"))?;
    let context = TemporalContext::from_now(Utc::now(), &timezone_name, identity.as_ref())?;
    let json = to_string_pretty(&context)?;

    if let Some(out) = out_path {
        let Some(parent) = out.parent() else {
            return Err(anyhow!(
                "identity now --out path must have a parent directory"
            ));
        };
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create output directory {}", parent.display()))?;
        fs::write(&out, json.as_bytes())
            .with_context(|| format!("failed to write temporal context to {}", out.display()))?;
        println!("TEMPORAL_CONTEXT_PATH={}", out.display());
    } else {
        println!("{json}");
    }

    Ok(())
}
