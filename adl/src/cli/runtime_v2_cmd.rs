use anyhow::{anyhow, Context, Result};
use serde_json::to_string_pretty;
use std::fs;
use std::path::{Path, PathBuf};

use ::adl::runtime_v2::runtime_v2_operator_control_report_contract;

pub(crate) fn real_runtime_v2(args: &[String]) -> Result<()> {
    let repo_root = std::env::current_dir().context("resolve current working directory")?;
    real_runtime_v2_in_repo(args, &repo_root)
}

fn real_runtime_v2_in_repo(args: &[String], repo_root: &Path) -> Result<()> {
    let Some(subcommand) = args.first().map(|arg| arg.as_str()) else {
        return Err(anyhow!(
            "runtime-v2 requires a subcommand: operator-controls"
        ));
    };

    match subcommand {
        "operator-controls" => real_runtime_v2_operator_controls(repo_root, &args[1..]),
        "--help" | "-h" | "help" => {
            println!("{}", super::usage::usage());
            Ok(())
        }
        _ => Err(anyhow!(
            "unknown runtime-v2 subcommand '{subcommand}' (expected operator-controls)"
        )),
    }
}

fn real_runtime_v2_operator_controls(repo_root: &Path, args: &[String]) -> Result<()> {
    let mut out_path: Option<PathBuf> = None;
    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--out" => {
                let Some(value) = args.get(i + 1) else {
                    return Err(anyhow!(
                        "runtime-v2 operator-controls requires --out <path>"
                    ));
                };
                out_path = Some(PathBuf::from(value));
                i += 1;
            }
            "--help" | "-h" => {
                println!("{}", super::usage::usage());
                return Ok(());
            }
            other => {
                return Err(anyhow!(
                    "unknown arg for runtime-v2 operator-controls: {other}"
                ))
            }
        }
        i += 1;
    }

    let report = runtime_v2_operator_control_report_contract()?;
    let json = to_string_pretty(&report)?;
    let Some(out_path) = out_path else {
        println!("{json}");
        return Ok(());
    };
    let resolved = if out_path.is_absolute() {
        out_path
    } else {
        repo_root.join(out_path)
    };
    let Some(parent) = resolved.parent() else {
        return Err(anyhow!(
            "runtime-v2 operator-controls --out path must have a parent directory"
        ));
    };
    fs::create_dir_all(parent)
        .with_context(|| format!("failed to create output directory {}", parent.display()))?;
    fs::write(&resolved, json.as_bytes()).with_context(|| {
        format!(
            "failed to write Runtime v2 operator control report to {}",
            resolved.display()
        )
    })?;
    println!(
        "RUNTIME_V2_OPERATOR_CONTROL_REPORT_PATH={}",
        resolved.display()
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_repo(label: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time went backwards")
            .as_nanos();
        std::env::temp_dir().join(format!(
            "adl-runtime-v2-cli-{label}-{}-{nanos}",
            std::process::id()
        ))
    }

    #[test]
    fn runtime_v2_operator_controls_writes_report_json() {
        let repo = temp_repo("operator-controls");
        let out_path = repo.join("out/operator_report.json");

        real_runtime_v2_in_repo(
            &[
                "operator-controls".to_string(),
                "--out".to_string(),
                "out/operator_report.json".to_string(),
            ],
            &repo,
        )
        .expect("operator controls");

        let json: serde_json::Value = serde_json::from_slice(
            &fs::read(&out_path).expect("operator controls report should be written"),
        )
        .expect("valid json");
        assert_eq!(
            json["schema_version"],
            "runtime_v2.operator_control_report.v1"
        );
        assert_eq!(json["commands"].as_array().expect("commands").len(), 7);

        fs::remove_dir_all(repo).ok();
    }

    #[test]
    fn runtime_v2_operator_controls_validates_unknown_args_and_missing_out_value() {
        let repo = temp_repo("operator-controls-errors");

        let err = real_runtime_v2_in_repo(
            &["operator-controls".to_string(), "--bogus".to_string()],
            &repo,
        )
        .expect_err("unknown arg should fail");
        assert!(err
            .to_string()
            .contains("unknown arg for runtime-v2 operator-controls: --bogus"));

        let err = real_runtime_v2_in_repo(
            &["operator-controls".to_string(), "--out".to_string()],
            &repo,
        )
        .expect_err("missing out value should fail");
        assert!(err
            .to_string()
            .contains("runtime-v2 operator-controls requires --out <path>"));
    }

    #[test]
    fn runtime_v2_dispatch_covers_help_and_subcommand_errors() {
        let repo = temp_repo("dispatch");

        real_runtime_v2_in_repo(&["--help".to_string()], &repo).expect("top-level help");
        real_runtime_v2_in_repo(&["help".to_string()], &repo).expect("help alias");

        let err = real_runtime_v2_in_repo(&[], &repo).expect_err("missing subcommand should fail");
        assert!(err
            .to_string()
            .contains("runtime-v2 requires a subcommand: operator-controls"));

        let err = real_runtime_v2_in_repo(&["bogus".to_string()], &repo)
            .expect_err("unknown subcommand should fail");
        assert!(err
            .to_string()
            .contains("unknown runtime-v2 subcommand 'bogus'"));
    }

    #[test]
    fn runtime_v2_operator_controls_covers_stdout_help_and_absolute_output() {
        let repo = temp_repo("operator-controls-branches");
        let out_path = repo.join("absolute/operator_report.json");

        real_runtime_v2_in_repo(&["operator-controls".to_string()], &repo).expect("stdout report");
        real_runtime_v2_in_repo(
            &["operator-controls".to_string(), "--help".to_string()],
            &repo,
        )
        .expect("operator controls help");
        real_runtime_v2_in_repo(
            &[
                "operator-controls".to_string(),
                "--out".to_string(),
                out_path.to_string_lossy().to_string(),
            ],
            &repo,
        )
        .expect("absolute output path");

        assert!(out_path.is_file());

        fs::remove_dir_all(repo).ok();
    }

    #[test]
    fn runtime_v2_public_dispatch_uses_current_directory() {
        real_runtime_v2(&["operator-controls".to_string()]).expect("public dispatch stdout");
    }
}
