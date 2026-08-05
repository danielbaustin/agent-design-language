use std::{fs, path::PathBuf};

use clap::{Parser, Subcommand};
use csdlc_v2::{
    migrate_legacy_preparation, repair_legacy_preparation, LegacyPreparationMigrationRequest,
    LegacyPreparationRepairRequest, Store,
};

#[derive(Parser)]
struct Cli {
    #[arg(long)]
    root: PathBuf,
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Preparation {
        #[arg(long)]
        request: PathBuf,
    },
    Repair {
        #[arg(long)]
        request: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();
    let result = match cli.command {
        Command::Preparation { request } => fs::read(request)
            .map_err(csdlc_v2::V2Error::from)
            .and_then(|bytes| {
                serde_json::from_slice::<LegacyPreparationMigrationRequest>(&bytes)
                    .map_err(csdlc_v2::V2Error::from)
            })
            .and_then(|request| migrate_legacy_preparation(&Store::new(cli.root), request))
            .and_then(|value| serde_json::to_value(value).map_err(csdlc_v2::V2Error::from)),
        Command::Repair { request } => fs::read(request)
            .map_err(csdlc_v2::V2Error::from)
            .and_then(|bytes| {
                serde_json::from_slice::<LegacyPreparationRepairRequest>(&bytes)
                    .map_err(csdlc_v2::V2Error::from)
            })
            .and_then(|request| repair_legacy_preparation(&Store::new(cli.root), request))
            .and_then(|value| serde_json::to_value(value).map_err(csdlc_v2::V2Error::from)),
    };
    match result {
        Ok(value) => println!("{}", serde_json::to_string(&value).expect("JSON")),
        Err(error) => {
            eprintln!("csdlc-migrate: {error}");
            println!(
                "{}",
                serde_json::json!({"schema":"csdlc.error.v1","code":error.code,"message":error.message})
            );
            std::process::exit(error.code.exit_code());
        }
    }
}
