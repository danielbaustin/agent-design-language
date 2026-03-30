use anyhow::{anyhow, Context, Result};
use chrono::Utc;
use serde_json::to_string_pretty;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use ::adl::chronosense::{
    default_identity_profile_path, load_identity_profile, write_identity_profile, IdentityProfile,
    TemporalContext,
};

pub(crate) fn real_identity(args: &[String]) -> Result<()> {
    let Some(subcommand) = args.first().map(|arg| arg.as_str()) else {
        return Err(anyhow!("identity requires a subcommand: init | show | now"));
    };

    match subcommand {
        "init" => real_identity_init(&args[1..]),
        "show" => real_identity_show(&args[1..]),
        "now" => real_identity_now(&args[1..]),
        "--help" | "-h" | "help" => {
            println!("{}", super::usage::usage());
            Ok(())
        }
        _ => Err(anyhow!(
            "unknown identity subcommand '{subcommand}' (expected init | show | now)"
        )),
    }
}

fn real_identity_init(args: &[String]) -> Result<()> {
    let repo_root = repo_root()?;
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
                println!("{}", super::usage::usage());
                return Ok(());
            }
            other => return Err(anyhow!("unknown arg for identity init: {other}")),
        }
        i += 1;
    }

    let profile_path = path.unwrap_or_else(|| default_identity_profile_path(&repo_root));
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

fn real_identity_show(args: &[String]) -> Result<()> {
    let repo_root = repo_root()?;
    let profile_path = resolve_identity_path(&repo_root, args)?;
    let profile = load_identity_profile(&profile_path)?;
    println!("{}", to_string_pretty(&profile)?);
    Ok(())
}

fn real_identity_now(args: &[String]) -> Result<()> {
    let repo_root = repo_root()?;
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
                println!("{}", super::usage::usage());
                return Ok(());
            }
            other => return Err(anyhow!("unknown arg for identity now: {other}")),
        }
        i += 1;
    }

    let profile_path = path.unwrap_or_else(|| default_identity_profile_path(&repo_root));
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

fn required_value(args: &[String], index: usize, flag: &str) -> Result<String> {
    args.get(index + 1)
        .cloned()
        .ok_or_else(|| anyhow!("{flag} requires a value"))
}

fn resolve_identity_path(repo_root: &Path, args: &[String]) -> Result<PathBuf> {
    let mut path: Option<PathBuf> = None;
    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--path" => {
                path = Some(PathBuf::from(required_value(args, i, "--path")?));
                i += 1;
            }
            "--help" | "-h" => {
                println!("{}", super::usage::usage());
                std::process::exit(0);
            }
            other => return Err(anyhow!("unknown arg for identity show: {other}")),
        }
        i += 1;
    }
    Ok(path.unwrap_or_else(|| default_identity_profile_path(repo_root)))
}

fn run_git_capture(args: &[&str]) -> Result<String> {
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

fn repo_root() -> Result<PathBuf> {
    Ok(PathBuf::from(
        run_git_capture(&["rev-parse", "--show-toplevel"])?
            .trim()
            .to_string(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::adl::chronosense::TEMPORAL_CONTEXT_SCHEMA;
    use once_cell::sync::Lazy;
    use serde_json::Value;
    use std::env;
    use std::sync::Mutex;
    use std::time::{SystemTime, UNIX_EPOCH};

    static TEST_MUTEX: Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));

    fn temp_repo(name: &str) -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock")
            .as_nanos();
        let repo = env::temp_dir().join(format!("adl-{name}-{unique}"));
        fs::create_dir_all(&repo).expect("create repo dir");
        Command::new("git")
            .arg("init")
            .arg("-b")
            .arg("main")
            .current_dir(&repo)
            .status()
            .expect("git init")
            .success()
            .then_some(())
            .expect("git init should succeed");
        repo
    }

    #[test]
    fn identity_init_writes_default_profile_and_show_reads_it() {
        let _guard = TEST_MUTEX.lock().expect("test mutex");
        let repo = temp_repo("identity-init-show");
        let prev_dir = env::current_dir().expect("cwd");
        env::set_current_dir(&repo).expect("chdir");

        real_identity(&[
            "init".to_string(),
            "--name".to_string(),
            "Codex".to_string(),
            "--birthday".to_string(),
            "2026-03-30T13:34:00-07:00".to_string(),
            "--timezone".to_string(),
            "America/Los_Angeles".to_string(),
            "--created-by".to_string(),
            "daniel".to_string(),
        ])
        .expect("identity init");

        let profile_path = repo.join("identity/identity_profile.v1.json");
        assert!(profile_path.is_file(), "profile should exist");

        let profile = load_identity_profile(&profile_path).expect("profile load");
        assert_eq!(profile.agent_id, "codex");
        assert_eq!(profile.birth_weekday_local, "Monday");

        real_identity(&["show".to_string()]).expect("identity show");

        env::set_current_dir(prev_dir).expect("restore cwd");
    }

    #[test]
    fn identity_now_requires_timezone_without_profile() {
        let _guard = TEST_MUTEX.lock().expect("test mutex");
        let repo = temp_repo("identity-now");
        let prev_dir = env::current_dir().expect("cwd");
        env::set_current_dir(&repo).expect("chdir");

        let err = real_identity(&["now".to_string()]).expect_err("should fail without timezone");
        assert!(err
            .to_string()
            .contains("identity now requires --timezone <IANA> when no profile exists"));

        env::set_current_dir(prev_dir).expect("restore cwd");
    }

    #[test]
    fn identity_now_writes_temporal_context_json() {
        let _guard = TEST_MUTEX.lock().expect("test mutex");
        let repo = temp_repo("identity-now-out");
        let prev_dir = env::current_dir().expect("cwd");
        env::set_current_dir(&repo).expect("chdir");

        real_identity(&[
            "init".to_string(),
            "--name".to_string(),
            "Codex".to_string(),
            "--birthday".to_string(),
            "2026-03-30T13:34:00-07:00".to_string(),
            "--timezone".to_string(),
            "America/Los_Angeles".to_string(),
        ])
        .expect("identity init");

        let out_path = repo.join(".adl/state/temporal_context.v1.json");
        real_identity(&[
            "now".to_string(),
            "--out".to_string(),
            out_path.display().to_string(),
        ])
        .expect("identity now");

        let json: Value =
            serde_json::from_slice(&fs::read(&out_path).expect("read out")).expect("parse json");
        assert_eq!(json["schema_version"], TEMPORAL_CONTEXT_SCHEMA);
        assert_eq!(json["identity_agent_id"], "codex");

        env::set_current_dir(prev_dir).expect("restore cwd");
    }
}
