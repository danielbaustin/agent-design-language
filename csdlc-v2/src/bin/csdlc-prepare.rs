use std::{fs, path::PathBuf};

use clap::{Parser, Subcommand};
use csdlc_v2::{
    run_preparation, run_preparation_batch, seal_preparation, sync_preparation,
    PrepareBatchRequest, PrepareRunRequest, PrepareSealRequest, PrepareSyncRequest, Store,
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
    Sync {
        #[arg(long)]
        request: PathBuf,
    },
    Seal {
        #[arg(long)]
        request: PathBuf,
    },
    Run {
        #[arg(long)]
        request: PathBuf,
    },
    Batch {
        #[arg(long)]
        request: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();
    let store = Store::new(cli.root);
    let result = match cli.command {
        Command::Sync { request } => decode::<PrepareSyncRequest>(request)
            .and_then(|request| sync_preparation(&store, request))
            .and_then(to_value),
        Command::Seal { request } => decode::<PrepareSealRequest>(request)
            .and_then(|request| seal_preparation(&store, request))
            .and_then(to_value),
        Command::Run { request } => decode::<PrepareRunRequest>(request)
            .and_then(|request| run_preparation(&store, request))
            .and_then(to_value),
        Command::Batch { request } => decode::<PrepareBatchRequest>(request)
            .and_then(|request| run_preparation_batch(&store, request))
            .and_then(to_value),
    };
    emit(result);
}

fn decode<T: serde::de::DeserializeOwned>(path: PathBuf) -> csdlc_v2::Result<T> {
    fs::read(path)
        .map_err(csdlc_v2::V2Error::from)
        .and_then(|bytes| serde_json::from_slice(&bytes).map_err(csdlc_v2::V2Error::from))
}

fn to_value<T: serde::Serialize>(value: T) -> csdlc_v2::Result<serde_json::Value> {
    serde_json::to_value(value).map_err(csdlc_v2::V2Error::from)
}

fn emit(result: csdlc_v2::Result<serde_json::Value>) {
    match result {
        Ok(value) => println!("{}", serde_json::to_string(&value).expect("JSON")),
        Err(error) => {
            eprintln!("csdlc-prepare: {error}");
            println!(
                "{}",
                serde_json::json!({"schema":"csdlc.error.v1","code":error.code,"message":error.message})
            );
            std::process::exit(error.code.exit_code());
        }
    }
}
