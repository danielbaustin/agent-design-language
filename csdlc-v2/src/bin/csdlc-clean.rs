use std::fs;
use std::path::PathBuf;

use clap::{Parser, Subcommand};
use csdlc_v2::cleanup::{
    build_legacy_terminal_index, execute_cleanup, materialize_terminal, validate_terminal_census,
    CleanupRequest, LegacyTerminalIndexRequest, TerminalMaterializeRequest,
};

#[derive(Parser)]
#[command(
    about = "Safely classify/remove exact C-SDLC worktrees and validate legacy terminal compatibility"
)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Cleanup {
        #[arg(long)]
        root: PathBuf,
        #[arg(long)]
        request: PathBuf,
    },
    CompatibilityIndex {
        #[arg(long)]
        root: PathBuf,
        #[arg(long)]
        request: PathBuf,
    },
    MaterializeTerminal {
        #[arg(long)]
        root: PathBuf,
        #[arg(long)]
        request: PathBuf,
    },
    ValidateCensus {
        #[arg(long)]
        root: PathBuf,
        #[arg(long)]
        audit: PathBuf,
    },
    Schema,
}

fn main() {
    let args = Args::parse();
    let result = match args.command {
        Command::Cleanup { root, request } => read::<CleanupRequest>(&request)
            .and_then(|request| execute_cleanup(&root, &request))
            .and_then(json_value),
        Command::CompatibilityIndex { root, request } => {
            read::<LegacyTerminalIndexRequest>(&request)
                .and_then(|request| build_legacy_terminal_index(&root, &request))
                .and_then(json_value)
        }
        Command::MaterializeTerminal { root, request } => {
            read::<TerminalMaterializeRequest>(&request)
                .and_then(|request| materialize_terminal(&root, &request))
                .and_then(json_value)
        }
        Command::ValidateCensus { root, audit } => {
            validate_terminal_census(&root, &audit).and_then(json_value)
        }
        Command::Schema => Ok(csdlc_v2::cleanup_schema_bundle()),
    };
    match result {
        Ok(value) => println!("{}", serde_json::to_string_pretty(&value).expect("JSON")),
        Err(error) => {
            println!(
                "{}",
                serde_json::json!({"schema":"csdlc.error.v1","code":error.code.to_string(),"message":error.message})
            );
            std::process::exit(error.code.exit_code());
        }
    }
}

fn read<T: serde::de::DeserializeOwned>(path: &PathBuf) -> csdlc_v2::Result<T> {
    Ok(serde_json::from_slice(&fs::read(path)?)?)
}

fn json_value<T: serde::Serialize>(value: T) -> csdlc_v2::Result<serde_json::Value> {
    Ok(serde_json::to_value(value)?)
}
