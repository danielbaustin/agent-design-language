use clap::{Parser, Subcommand};
use csdlc_v2::{
    build_and_install_binaries, resolve_operator_generation, verify_coexistence,
    CoexistenceInventory, Generation, SkillManifest,
};
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "csdlc-install")]
struct Args {
    #[command(subcommand)]
    command: Command,
}
#[derive(Subcommand)]
enum Command {
    Manifest,
    Install {
        #[arg(long)]
        repo: PathBuf,
        #[arg(long)]
        destination: PathBuf,
    },
    Verify {
        #[arg(long)]
        repo: PathBuf,
        #[arg(long)]
        bin_dir: PathBuf,
        #[arg(long)]
        inventory: PathBuf,
    },
    Resolve {
        #[arg(long)]
        repo: PathBuf,
        #[arg(long)]
        issue: u64,
        #[arg(long)]
        requested: Option<Generation>,
    },
}
fn main() {
    let result = match Args::parse().command {
        Command::Manifest => SkillManifest::load().and_then(json),
        Command::Install { repo, destination } => {
            build_and_install_binaries(&repo, &destination).and_then(json)
        }
        Command::Verify {
            repo,
            bin_dir,
            inventory,
        } => fs::read(inventory)
            .map_err(io_error)
            .and_then(|b| {
                serde_json::from_slice::<CoexistenceInventory>(&b).map_err(|e| {
                    csdlc_v2::V2Error::new(csdlc_v2::ErrorCode::CorruptRecord, e.to_string())
                })
            })
            .and_then(|v| verify_coexistence(&repo, &bin_dir, &v))
            .and_then(|r| {
                let pass = r.pass;
                json(r)?;
                if pass {
                    Ok(())
                } else {
                    Err(csdlc_v2::V2Error::new(
                        csdlc_v2::ErrorCode::ValidationFailed,
                        "coexistence proof failed",
                    ))
                }
            }),
        Command::Resolve {
            repo,
            issue,
            requested,
        } => resolve_operator_generation(&repo, issue, requested).and_then(json),
    };
    if let Err(error) = result {
        eprintln!("{error}");
        std::process::exit(2);
    }
}
fn json(value: impl serde::Serialize) -> csdlc_v2::Result<()> {
    println!(
        "{}",
        serde_json::to_string_pretty(&value)
            .map_err(|e| csdlc_v2::V2Error::new(csdlc_v2::ErrorCode::Io, e.to_string()))?
    );
    Ok(())
}
fn io_error(error: std::io::Error) -> csdlc_v2::V2Error {
    csdlc_v2::V2Error::new(csdlc_v2::ErrorCode::Io, error.to_string())
}
