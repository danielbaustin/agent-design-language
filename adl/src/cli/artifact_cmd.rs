use anyhow::{anyhow, Result};
use std::path::PathBuf;

use super::run_artifacts::validate_control_path_artifact_set;

pub(crate) fn real_artifact(args: &[String]) -> Result<()> {
    let Some(subcommand) = args.first().map(|arg| arg.as_str()) else {
        return Err(anyhow!(
            "artifact requires a subcommand: validate-control-path"
        ));
    };

    match subcommand {
        "validate-control-path" => real_validate_control_path(&args[1..]),
        "--help" | "-h" | "help" => {
            println!("{}", super::usage::usage());
            Ok(())
        }
        _ => Err(anyhow!(
            "unknown artifact subcommand '{subcommand}' (expected validate-control-path)"
        )),
    }
}

fn real_validate_control_path(args: &[String]) -> Result<()> {
    let mut root: Option<PathBuf> = None;
    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--root" => {
                let Some(value) = args.get(i + 1) else {
                    return Err(anyhow!(
                        "artifact validate-control-path requires --root <dir>"
                    ));
                };
                root = Some(PathBuf::from(value));
                i += 1;
            }
            "--help" | "-h" => {
                println!("{}", super::usage::usage());
                return Ok(());
            }
            other => {
                return Err(anyhow!(
                    "unknown arg for artifact validate-control-path: {other}"
                ));
            }
        }
        i += 1;
    }

    let root =
        root.ok_or_else(|| anyhow!("artifact validate-control-path requires --root <dir>"))?;
    validate_control_path_artifact_set(&root)?;
    println!("CONTROL_PATH_VALIDATION=PASS");
    println!("CONTROL_PATH_ROOT={}", root.display());
    Ok(())
}
