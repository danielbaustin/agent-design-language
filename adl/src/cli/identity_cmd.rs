use anyhow::{anyhow, Context, Result};
use chrono::Utc;
use serde_json::to_string_pretty;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use ::adl::chronosense::{
    default_identity_profile_path, load_identity_profile, write_identity_profile,
    ChronosenseFoundation, CommitmentDeadlineContract, ContinuitySemanticsContract,
    IdentityProfile, TemporalCausalityExplanationContract, TemporalContext,
    TemporalQueryRetrievalContract, TemporalSchemaContract,
};

pub(crate) fn real_identity(args: &[String]) -> Result<()> {
    let repo_root = repo_root()?;
    real_identity_in_repo(args, &repo_root)
}

fn real_identity_in_repo(args: &[String], repo_root: &Path) -> Result<()> {
    let Some(subcommand) = args.first().map(|arg| arg.as_str()) else {
        return Err(anyhow!(
            "identity requires a subcommand: init | show | now | foundation | schema | continuity | retrieval | commitments"
        ));
    };

    match subcommand {
        "init" => real_identity_init(repo_root, &args[1..]),
        "show" => real_identity_show(repo_root, &args[1..]),
        "now" => real_identity_now(repo_root, &args[1..]),
        "foundation" => real_identity_foundation(repo_root, &args[1..]),
        "schema" => real_identity_schema(repo_root, &args[1..]),
        "continuity" => real_identity_continuity(repo_root, &args[1..]),
        "retrieval" => real_identity_retrieval(repo_root, &args[1..]),
        "commitments" => real_identity_commitments(repo_root, &args[1..]),
        "causality" => real_identity_causality(repo_root, &args[1..]),
        "--help" | "-h" | "help" => {
            println!("{}", super::usage::usage());
            Ok(())
        }
        _ => Err(anyhow!(
            "unknown identity subcommand '{subcommand}' (expected init | show | now | foundation | schema | continuity | retrieval | commitments | causality)"
        )),
    }
}

fn real_identity_init(repo_root: &Path, args: &[String]) -> Result<()> {
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

fn real_identity_show(repo_root: &Path, args: &[String]) -> Result<()> {
    let profile_path = resolve_identity_path(repo_root, args)?;
    let profile = load_identity_profile(&profile_path)?;
    println!("{}", to_string_pretty(&profile)?);
    Ok(())
}

fn real_identity_now(repo_root: &Path, args: &[String]) -> Result<()> {
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

fn real_identity_foundation(repo_root: &Path, args: &[String]) -> Result<()> {
    let mut out_path: Option<PathBuf> = None;

    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--out" => {
                out_path = Some(PathBuf::from(required_value(args, i, "--out")?));
                i += 1;
            }
            "--help" | "-h" => {
                println!("{}", super::usage::usage());
                return Ok(());
            }
            other => return Err(anyhow!("unknown arg for identity foundation: {other}")),
        }
        i += 1;
    }

    let foundation = ChronosenseFoundation::bounded_v088();
    let json = to_string_pretty(&foundation)?;

    if let Some(out) = out_path {
        let resolved = if out.is_absolute() {
            out
        } else {
            repo_root.join(out)
        };
        let Some(parent) = resolved.parent() else {
            return Err(anyhow!(
                "identity foundation --out path must have a parent directory"
            ));
        };
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create output directory {}", parent.display()))?;
        fs::write(&resolved, json.as_bytes()).with_context(|| {
            format!(
                "failed to write chronosense foundation artifact to {}",
                resolved.display()
            )
        })?;
        println!("CHRONOSENSE_FOUNDATION_PATH={}", resolved.display());
    } else {
        println!("{json}");
    }

    Ok(())
}

fn real_identity_schema(repo_root: &Path, args: &[String]) -> Result<()> {
    let mut out_path: Option<PathBuf> = None;

    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--out" => {
                out_path = Some(PathBuf::from(required_value(args, i, "--out")?));
                i += 1;
            }
            "--help" | "-h" => {
                println!("{}", super::usage::usage());
                return Ok(());
            }
            other => return Err(anyhow!("unknown arg for identity schema: {other}")),
        }
        i += 1;
    }

    let schema = TemporalSchemaContract::v01();
    let json = to_string_pretty(&schema)?;

    if let Some(out) = out_path {
        let resolved = if out.is_absolute() {
            out
        } else {
            repo_root.join(out)
        };
        let Some(parent) = resolved.parent() else {
            return Err(anyhow!(
                "identity schema --out path must have a parent directory"
            ));
        };
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create output directory {}", parent.display()))?;
        fs::write(&resolved, json.as_bytes()).with_context(|| {
            format!(
                "failed to write temporal schema artifact to {}",
                resolved.display()
            )
        })?;
        println!("TEMPORAL_SCHEMA_PATH={}", resolved.display());
    } else {
        println!("{json}");
    }

    Ok(())
}

fn real_identity_continuity(repo_root: &Path, args: &[String]) -> Result<()> {
    let mut out_path: Option<PathBuf> = None;

    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--out" => {
                out_path = Some(PathBuf::from(required_value(args, i, "--out")?));
                i += 1;
            }
            "--help" | "-h" => {
                println!("{}", super::usage::usage());
                return Ok(());
            }
            other => return Err(anyhow!("unknown arg for identity continuity: {other}")),
        }
        i += 1;
    }

    let contract = ContinuitySemanticsContract::v1();
    let json = to_string_pretty(&contract)?;

    if let Some(out) = out_path {
        let resolved = if out.is_absolute() {
            out
        } else {
            repo_root.join(out)
        };
        let Some(parent) = resolved.parent() else {
            return Err(anyhow!(
                "identity continuity --out path must have a parent directory"
            ));
        };
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create output directory {}", parent.display()))?;
        fs::write(&resolved, json.as_bytes()).with_context(|| {
            format!(
                "failed to write continuity semantics artifact to {}",
                resolved.display()
            )
        })?;
        println!("CONTINUITY_SEMANTICS_PATH={}", resolved.display());
    } else {
        println!("{json}");
    }

    Ok(())
}

fn real_identity_retrieval(repo_root: &Path, args: &[String]) -> Result<()> {
    let mut out_path: Option<PathBuf> = None;

    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--out" => {
                out_path = Some(PathBuf::from(required_value(args, i, "--out")?));
                i += 1;
            }
            "--help" | "-h" => {
                println!("{}", super::usage::usage());
                return Ok(());
            }
            other => return Err(anyhow!("unknown arg for identity retrieval: {other}")),
        }
        i += 1;
    }

    let contract = TemporalQueryRetrievalContract::v1();
    let json = to_string_pretty(&contract)?;

    if let Some(out) = out_path {
        let resolved = if out.is_absolute() {
            out
        } else {
            repo_root.join(out)
        };
        let Some(parent) = resolved.parent() else {
            return Err(anyhow!(
                "identity retrieval --out path must have a parent directory"
            ));
        };
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create output directory {}", parent.display()))?;
        fs::write(&resolved, json.as_bytes()).with_context(|| {
            format!(
                "failed to write temporal query retrieval artifact to {}",
                resolved.display()
            )
        })?;
        println!("TEMPORAL_QUERY_RETRIEVAL_PATH={}", resolved.display());
    } else {
        println!("{json}");
    }

    Ok(())
}

fn real_identity_commitments(repo_root: &Path, args: &[String]) -> Result<()> {
    let mut out_path: Option<PathBuf> = None;

    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--out" => {
                out_path = Some(PathBuf::from(required_value(args, i, "--out")?));
                i += 1;
            }
            "--help" | "-h" => {
                println!("{}", super::usage::usage());
                return Ok(());
            }
            other => return Err(anyhow!("unknown arg for identity commitments: {other}")),
        }
        i += 1;
    }

    let contract = CommitmentDeadlineContract::v1();
    let json = to_string_pretty(&contract)?;

    if let Some(out) = out_path {
        let resolved = if out.is_absolute() {
            out
        } else {
            repo_root.join(out)
        };
        let Some(parent) = resolved.parent() else {
            return Err(anyhow!(
                "identity commitments --out path must have a parent directory"
            ));
        };
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create output directory {}", parent.display()))?;
        fs::write(&resolved, json.as_bytes()).with_context(|| {
            format!(
                "failed to write commitment deadline artifact to {}",
                resolved.display()
            )
        })?;
        println!("COMMITMENT_DEADLINE_PATH={}", resolved.display());
    } else {
        println!("{json}");
    }

    Ok(())
}

fn real_identity_causality(repo_root: &Path, args: &[String]) -> Result<()> {
    let mut out_path: Option<PathBuf> = None;

    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--out" => {
                out_path = Some(PathBuf::from(required_value(args, i, "--out")?));
                i += 1;
            }
            "--help" | "-h" => {
                println!("{}", super::usage::usage());
                return Ok(());
            }
            other => return Err(anyhow!("unknown arg for identity causality: {other}")),
        }
        i += 1;
    }

    let contract = TemporalCausalityExplanationContract::v1();
    let json = to_string_pretty(&contract)?;

    if let Some(out) = out_path {
        let resolved = if out.is_absolute() {
            out
        } else {
            repo_root.join(out)
        };
        let Some(parent) = resolved.parent() else {
            return Err(anyhow!(
                "identity causality --out path must have a parent directory"
            ));
        };
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create output directory {}", parent.display()))?;
        fs::write(&resolved, json.as_bytes()).with_context(|| {
            format!(
                "failed to write temporal causality explanation artifact to {}",
                resolved.display()
            )
        })?;
        println!("TEMPORAL_CAUSALITY_EXPLANATION_PATH={}", resolved.display());
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

    fn system_git_bin() -> &'static str {
        if Path::new("/usr/bin/git").exists() {
            "/usr/bin/git"
        } else {
            "git"
        }
    }

    fn temp_repo(name: &str) -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock")
            .as_nanos();
        let repo = env::temp_dir().join(format!("adl-{name}-{unique}"));
        fs::create_dir_all(&repo).expect("create repo dir");
        Command::new(system_git_bin())
            .arg("init")
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
        let _guard = TEST_MUTEX
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let repo = temp_repo("identity-init-show");

        real_identity_in_repo(
            &[
                "init".to_string(),
                "--name".to_string(),
                "Codex".to_string(),
                "--birthday".to_string(),
                "2026-03-30T13:34:00-07:00".to_string(),
                "--timezone".to_string(),
                "America/Los_Angeles".to_string(),
                "--created-by".to_string(),
                "daniel".to_string(),
            ],
            &repo,
        )
        .expect("identity init");

        let profile_path = repo.join("adl/identity/identity_profile.v1.json");
        assert!(profile_path.is_file(), "profile should exist");

        let profile = load_identity_profile(&profile_path).expect("profile load");
        assert_eq!(profile.agent_id, "codex");
        assert_eq!(profile.birth_weekday_local, "Monday");

        real_identity_in_repo(&["show".to_string()], &repo).expect("identity show");
    }

    #[test]
    fn identity_now_requires_timezone_without_profile() {
        let _guard = TEST_MUTEX
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let repo = temp_repo("identity-now");

        let err = real_identity_in_repo(&["now".to_string()], &repo)
            .expect_err("should fail without timezone");
        assert!(err
            .to_string()
            .contains("identity now requires --timezone <IANA> when no profile exists"));
    }

    #[test]
    fn identity_now_writes_temporal_context_json() {
        let _guard = TEST_MUTEX
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let repo = temp_repo("identity-now-out");

        real_identity_in_repo(
            &[
                "init".to_string(),
                "--name".to_string(),
                "Codex".to_string(),
                "--birthday".to_string(),
                "2026-03-30T13:34:00-07:00".to_string(),
                "--timezone".to_string(),
                "America/Los_Angeles".to_string(),
            ],
            &repo,
        )
        .expect("identity init");

        let out_path = repo.join(".adl/state/temporal_context.v1.json");
        real_identity_in_repo(
            &[
                "now".to_string(),
                "--out".to_string(),
                out_path.display().to_string(),
            ],
            &repo,
        )
        .expect("identity now");

        let json: Value =
            serde_json::from_slice(&fs::read(&out_path).expect("read out")).expect("parse json");
        assert_eq!(json["schema_version"], TEMPORAL_CONTEXT_SCHEMA);
        assert_eq!(json["identity_agent_id"], "codex");
    }

    #[test]
    fn identity_foundation_writes_bounded_foundation_json() {
        let _guard = TEST_MUTEX
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let repo = temp_repo("identity-foundation");
        let out_path = repo.join(".adl/state/chronosense_foundation.v1.json");

        real_identity_in_repo(
            &[
                "foundation".to_string(),
                "--out".to_string(),
                ".adl/state/chronosense_foundation.v1.json".to_string(),
            ],
            &repo,
        )
        .expect("identity foundation");

        let json: Value =
            serde_json::from_slice(&fs::read(&out_path).expect("read out")).expect("parse json");
        assert_eq!(json["schema_version"], "chronosense_foundation.v1");
        assert_eq!(
            json["proof_hook_output_path"],
            ".adl/state/chronosense_foundation.v1.json"
        );
        assert!(json["owned_runtime_surfaces"]
            .as_array()
            .expect("array")
            .iter()
            .any(|value| value == "adl identity foundation"));
    }

    #[test]
    fn identity_requires_subcommand_and_rejects_unknown_subcommand() {
        let repo = temp_repo("identity-subcommands");

        let err = real_identity_in_repo(&[], &repo).expect_err("missing subcommand should fail");
        assert!(err
            .to_string()
            .contains("identity requires a subcommand: init | show | now | foundation | schema"));
        assert!(err.to_string().contains("continuity"));

        let err = real_identity_in_repo(&["nope".to_string()], &repo)
            .expect_err("unknown subcommand should fail");
        assert!(err
            .to_string()
            .contains("unknown identity subcommand 'nope'"));
    }

    #[test]
    fn identity_top_level_help_and_subcommand_help_succeed() {
        let repo = temp_repo("identity-help");

        real_identity_in_repo(&["help".to_string()], &repo).expect("top-level help");
        real_identity_in_repo(&["init".to_string(), "--help".to_string()], &repo)
            .expect("init help");
        real_identity_in_repo(&["now".to_string(), "--help".to_string()], &repo).expect("now help");
        real_identity_in_repo(&["foundation".to_string(), "--help".to_string()], &repo)
            .expect("foundation help");
        real_identity_in_repo(&["schema".to_string(), "--help".to_string()], &repo)
            .expect("schema help");
        real_identity_in_repo(&["continuity".to_string(), "--help".to_string()], &repo)
            .expect("continuity help");
    }

    #[test]
    fn identity_init_validates_required_and_unknown_args() {
        let repo = temp_repo("identity-init-errors");

        let err = real_identity_in_repo(
            &[
                "init".to_string(),
                "--birthday".to_string(),
                "2026-03-30T13:34:00-07:00".to_string(),
                "--timezone".to_string(),
                "America/Los_Angeles".to_string(),
            ],
            &repo,
        )
        .expect_err("missing name should fail");
        assert!(err
            .to_string()
            .contains("identity init requires --name <display-name>"));

        let err = real_identity_in_repo(
            &[
                "init".to_string(),
                "--name".to_string(),
                "Codex".to_string(),
                "--timezone".to_string(),
                "America/Los_Angeles".to_string(),
            ],
            &repo,
        )
        .expect_err("missing birthday should fail");
        assert!(err
            .to_string()
            .contains("identity init requires --birthday <RFC3339>"));

        let err = real_identity_in_repo(
            &[
                "init".to_string(),
                "--name".to_string(),
                "Codex".to_string(),
                "--birthday".to_string(),
                "2026-03-30T13:34:00-07:00".to_string(),
            ],
            &repo,
        )
        .expect_err("missing timezone should fail");
        assert!(err
            .to_string()
            .contains("identity init requires --timezone <IANA>"));

        let err = real_identity_in_repo(&["init".to_string(), "--bogus".to_string()], &repo)
            .expect_err("unknown arg should fail");
        assert!(err
            .to_string()
            .contains("unknown arg for identity init: --bogus"));
    }

    #[test]
    fn identity_init_supports_custom_path_agent_id_and_force() {
        let _guard = TEST_MUTEX
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let repo = temp_repo("identity-init-custom-path");
        let profile_path = repo.join(".adl/state/custom_identity_profile.v1.json");

        real_identity_in_repo(
            &[
                "init".to_string(),
                "--name".to_string(),
                "Codex".to_string(),
                "--agent-id".to_string(),
                "codex-local".to_string(),
                "--birthday".to_string(),
                "2026-03-30T13:34:00-07:00".to_string(),
                "--timezone".to_string(),
                "America/Los_Angeles".to_string(),
                "--path".to_string(),
                profile_path.display().to_string(),
            ],
            &repo,
        )
        .expect("custom path init");

        let profile = load_identity_profile(&profile_path).expect("profile load");
        assert_eq!(profile.agent_id, "codex-local");

        let err = real_identity_in_repo(
            &[
                "init".to_string(),
                "--name".to_string(),
                "Codex".to_string(),
                "--birthday".to_string(),
                "2026-03-30T13:34:00-07:00".to_string(),
                "--timezone".to_string(),
                "America/Los_Angeles".to_string(),
                "--path".to_string(),
                profile_path.display().to_string(),
            ],
            &repo,
        )
        .expect_err("existing profile without force should fail");
        assert!(err.to_string().contains("identity profile already exists"));

        real_identity_in_repo(
            &[
                "init".to_string(),
                "--name".to_string(),
                "Codex".to_string(),
                "--birthday".to_string(),
                "2026-03-30T13:34:00-07:00".to_string(),
                "--timezone".to_string(),
                "America/Los_Angeles".to_string(),
                "--path".to_string(),
                profile_path.display().to_string(),
                "--force".to_string(),
            ],
            &repo,
        )
        .expect("force overwrite");
    }

    #[test]
    fn identity_show_supports_custom_path_and_rejects_unknown_args() {
        let _guard = TEST_MUTEX
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let repo = temp_repo("identity-show-path");
        let profile_path = repo.join("identity/custom_profile.v1.json");

        real_identity_in_repo(
            &[
                "init".to_string(),
                "--name".to_string(),
                "Codex".to_string(),
                "--birthday".to_string(),
                "2026-03-30T13:34:00-07:00".to_string(),
                "--timezone".to_string(),
                "America/Los_Angeles".to_string(),
                "--path".to_string(),
                profile_path.display().to_string(),
            ],
            &repo,
        )
        .expect("seed profile");

        real_identity_in_repo(
            &[
                "show".to_string(),
                "--path".to_string(),
                profile_path.display().to_string(),
            ],
            &repo,
        )
        .expect("show custom path");

        let err = real_identity_in_repo(&["show".to_string(), "--bogus".to_string()], &repo)
            .expect_err("show unknown arg");
        assert!(err
            .to_string()
            .contains("unknown arg for identity show: --bogus"));
    }

    #[test]
    fn identity_now_validates_unknown_args_and_missing_out_value() {
        let repo = temp_repo("identity-now-errors");

        let err = real_identity_in_repo(
            &[
                "now".to_string(),
                "--timezone".to_string(),
                "America/Los_Angeles".to_string(),
                "--bogus".to_string(),
            ],
            &repo,
        )
        .expect_err("unknown arg should fail");
        assert!(err
            .to_string()
            .contains("unknown arg for identity now: --bogus"));

        let err = real_identity_in_repo(
            &[
                "now".to_string(),
                "--timezone".to_string(),
                "America/Los_Angeles".to_string(),
                "--out".to_string(),
            ],
            &repo,
        )
        .expect_err("out flag without value should fail");
        assert!(err.to_string().contains("--out requires a value"));
    }

    #[test]
    fn identity_foundation_validates_unknown_args_and_missing_out_value() {
        let repo = temp_repo("identity-foundation-errors");

        let err = real_identity_in_repo(&["foundation".to_string(), "--bogus".to_string()], &repo)
            .expect_err("unknown arg should fail");
        assert!(err
            .to_string()
            .contains("unknown arg for identity foundation: --bogus"));

        let err = real_identity_in_repo(&["foundation".to_string(), "--out".to_string()], &repo)
            .expect_err("out flag without value should fail");
        assert!(err.to_string().contains("--out requires a value"));
    }

    #[test]
    fn identity_schema_writes_temporal_schema_contract_json() {
        let _guard = TEST_MUTEX
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let repo = temp_repo("identity-schema");
        let out_path = repo.join(".adl/state/temporal_schema_v01.json");

        real_identity_in_repo(
            &[
                "schema".to_string(),
                "--out".to_string(),
                ".adl/state/temporal_schema_v01.json".to_string(),
            ],
            &repo,
        )
        .expect("identity schema");

        let json: Value =
            serde_json::from_slice(&fs::read(&out_path).expect("read out")).expect("parse json");
        assert_eq!(json["schema_version"], "temporal_schema.v0_1");
        assert_eq!(
            json["proof_hook_output_path"],
            ".adl/state/temporal_schema_v01.json"
        );
        assert!(json["execution_policy_trace_hooks"]
            .as_array()
            .expect("array")
            .iter()
            .any(|value| value == "run_state.v1.duration_ms"));
    }

    #[test]
    fn identity_schema_validates_unknown_args_and_missing_out_value() {
        let repo = temp_repo("identity-schema-errors");

        let err = real_identity_in_repo(&["schema".to_string(), "--bogus".to_string()], &repo)
            .expect_err("unknown arg should fail");
        assert!(err
            .to_string()
            .contains("unknown arg for identity schema: --bogus"));

        let err = real_identity_in_repo(&["schema".to_string(), "--out".to_string()], &repo)
            .expect_err("out flag without value should fail");
        assert!(err.to_string().contains("--out requires a value"));
    }

    #[test]
    fn identity_continuity_writes_continuity_semantics_contract_json() {
        let _guard = TEST_MUTEX
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let repo = temp_repo("identity-continuity");
        let out_path = repo.join(".adl/state/continuity_semantics_v1.json");

        real_identity_in_repo(
            &[
                "continuity".to_string(),
                "--out".to_string(),
                ".adl/state/continuity_semantics_v1.json".to_string(),
            ],
            &repo,
        )
        .expect("identity continuity");

        let json: Value =
            serde_json::from_slice(&fs::read(&out_path).expect("read out")).expect("parse json");
        assert_eq!(json["schema_version"], "continuity_semantics.v1");
        assert!(json["continuity_state_contract"]["continuity_status"]
            .as_array()
            .expect("array")
            .iter()
            .any(|value| value == "resume_ready"));
        assert_eq!(
            json["proof_hook_output_path"],
            ".adl/state/continuity_semantics_v1.json"
        );
    }

    #[test]
    fn identity_continuity_validates_unknown_args_and_missing_out_value() {
        let repo = temp_repo("identity-continuity-errors");

        let err = real_identity_in_repo(&["continuity".to_string(), "--bogus".to_string()], &repo)
            .expect_err("unknown arg should fail");
        assert!(err
            .to_string()
            .contains("unknown arg for identity continuity: --bogus"));

        let err = real_identity_in_repo(&["continuity".to_string(), "--out".to_string()], &repo)
            .expect_err("out flag without value should fail");
        assert!(err.to_string().contains("--out requires a value"));
    }

    #[test]
    fn identity_retrieval_writes_temporal_query_retrieval_contract_json() {
        let _guard = TEST_MUTEX
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let repo = temp_repo("identity-retrieval");
        let out_path = repo.join(".adl/state/temporal_query_retrieval_v1.json");

        real_identity_in_repo(
            &[
                "retrieval".to_string(),
                "--out".to_string(),
                ".adl/state/temporal_query_retrieval_v1.json".to_string(),
            ],
            &repo,
        )
        .expect("identity retrieval");

        let json: Value =
            serde_json::from_slice(&fs::read(&out_path).expect("read out")).expect("parse json");
        assert_eq!(json["schema_version"], "temporal_query_retrieval.v1");
        assert_eq!(
            json["proof_hook_output_path"],
            ".adl/state/temporal_query_retrieval_v1.json"
        );
        assert!(json["owned_runtime_surfaces"]
            .as_array()
            .expect("array")
            .iter()
            .any(|value| value == "adl identity retrieval"));
    }

    #[test]
    fn identity_retrieval_validates_unknown_args_and_missing_out_value() {
        let repo = temp_repo("identity-retrieval-errors");

        let err = real_identity_in_repo(&["retrieval".to_string(), "--bogus".to_string()], &repo)
            .expect_err("unknown arg should fail");
        assert!(err
            .to_string()
            .contains("unknown arg for identity retrieval: --bogus"));

        let err = real_identity_in_repo(&["retrieval".to_string(), "--out".to_string()], &repo)
            .expect_err("out flag without value should fail");
        assert!(err.to_string().contains("--out requires a value"));
    }

    #[test]
    fn identity_commitments_writes_commitment_deadline_contract_json() {
        let _guard = TEST_MUTEX
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let repo = temp_repo("identity-commitments");
        let out_path = repo.join(".adl/state/commitment_deadline_semantics_v1.json");

        real_identity_in_repo(
            &[
                "commitments".to_string(),
                "--out".to_string(),
                ".adl/state/commitment_deadline_semantics_v1.json".to_string(),
            ],
            &repo,
        )
        .expect("identity commitments");

        let json: Value =
            serde_json::from_slice(&fs::read(&out_path).expect("read out")).expect("parse json");
        assert_eq!(json["schema_version"], "commitment_deadline_semantics.v1");
        assert_eq!(
            json["proof_hook_output_path"],
            ".adl/state/commitment_deadline_semantics_v1.json"
        );
        assert!(json["owned_runtime_surfaces"]
            .as_array()
            .expect("array")
            .iter()
            .any(|value| value == "adl identity commitments"));
    }

    #[test]
    fn identity_commitments_validates_unknown_args_and_missing_out_value() {
        let repo = temp_repo("identity-commitments-errors");

        let err = real_identity_in_repo(&["commitments".to_string(), "--bogus".to_string()], &repo)
            .expect_err("unknown arg should fail");
        assert!(err
            .to_string()
            .contains("unknown arg for identity commitments: --bogus"));

        let err = real_identity_in_repo(&["commitments".to_string(), "--out".to_string()], &repo)
            .expect_err("out flag without value should fail");
        assert!(err.to_string().contains("--out requires a value"));
    }

    #[test]
    fn identity_causality_writes_temporal_causality_explanation_contract_json() {
        let _guard = TEST_MUTEX
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let repo = temp_repo("identity-causality");
        let out_path = repo.join(".adl/state/temporal_causality_explanation_v1.json");

        real_identity_in_repo(
            &[
                "causality".to_string(),
                "--out".to_string(),
                ".adl/state/temporal_causality_explanation_v1.json".to_string(),
            ],
            &repo,
        )
        .expect("identity causality");

        let json: Value =
            serde_json::from_slice(&fs::read(&out_path).expect("read out")).expect("parse json");
        assert_eq!(json["schema_version"], "temporal_causality_explanation.v1");
        assert_eq!(
            json["proof_hook_output_path"],
            ".adl/state/temporal_causality_explanation_v1.json"
        );
        assert!(json["owned_runtime_surfaces"]
            .as_array()
            .expect("array")
            .iter()
            .any(|value| value == "adl identity causality"));
    }

    #[test]
    fn identity_causality_validates_unknown_args_and_missing_out_value() {
        let repo = temp_repo("identity-causality-errors");

        let err = real_identity_in_repo(&["causality".to_string(), "--bogus".to_string()], &repo)
            .expect_err("unknown arg should fail");
        assert!(err
            .to_string()
            .contains("unknown arg for identity causality: --bogus"));

        let err = real_identity_in_repo(&["causality".to_string(), "--out".to_string()], &repo)
            .expect_err("out flag without value should fail");
        assert!(err.to_string().contains("--out requires a value"));
    }

    #[test]
    fn required_value_and_git_capture_report_errors() {
        let err = required_value(&["--name".to_string()], 0, "--name")
            .expect_err("missing flag value should fail");
        assert!(err.to_string().contains("--name requires a value"));

        let err = run_git_capture(&["definitely-not-a-real-subcommand"])
            .expect_err("invalid git command should fail");
        assert!(err
            .to_string()
            .contains("git definitely-not-a-real-subcommand failed with status"));
    }
}
