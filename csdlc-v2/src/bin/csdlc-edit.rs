use std::fs;
use std::path::PathBuf;

use clap::{Parser, Subcommand};
use csdlc_v2::{
    approve_design, edit_issue, initialize_native_json, public_schema_bundle, ApproveDesignRequest,
    EditRequest, ErrorCode, Store,
};
use serde::Serialize;

#[derive(Parser)]
#[command(name = "csdlc-edit")]
struct Args {
    #[arg(long, default_value = ".")]
    repo: PathBuf,
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Bootstrap {
        #[arg(long)]
        request: PathBuf,
    },
    Apply {
        #[arg(long)]
        request: PathBuf,
    },
    ApproveDesign {
        #[arg(long)]
        request: PathBuf,
    },
    Schema,
}

#[derive(Serialize)]
struct ErrorOutput<'a> {
    schema: &'static str,
    code: ErrorCode,
    message: &'a str,
}

fn main() {
    let args = Args::parse();
    let store = Store::new(args.repo);
    if matches!(&args.command, Command::Schema) {
        println!(
            "{}",
            serde_json::to_string_pretty(&public_schema_bundle()).expect("schema JSON")
        );
        return;
    }
    let result = match args.command {
        Command::Bootstrap { request } => fs::read(request)
            .map_err(Into::into)
            .and_then(|bytes| initialize_native_json(&store, &bytes)),
        Command::Apply { request } => {
            read::<EditRequest>(&request).and_then(|request| edit_issue(&store, request))
        }
        Command::ApproveDesign { request } => read::<ApproveDesignRequest>(&request)
            .and_then(|request| approve_design(&store, request)),
        Command::Schema => unreachable!("handled above"),
    };
    match result {
        Ok(record) => println!(
            "{}",
            serde_json::to_string_pretty(&record).expect("record JSON")
        ),
        Err(error) => {
            eprintln!("csdlc-edit: {}", error.message);
            println!(
                "{}",
                serde_json::to_string(&ErrorOutput {
                    schema: "csdlc.error.v1",
                    code: error.code,
                    message: &error.message,
                })
                .expect("error JSON")
            );
            std::process::exit(error.code.exit_code());
        }
    }
}

fn read<T: for<'de> serde::Deserialize<'de>>(path: &PathBuf) -> csdlc_v2::Result<T> {
    let bytes = fs::read(path)?;
    Ok(serde_json::from_slice(&bytes)?)
}
