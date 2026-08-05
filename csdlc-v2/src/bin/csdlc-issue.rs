use std::{fs, path::PathBuf};

use clap::{Parser, Subcommand};
use csdlc_v2::{create_issue_draft, IssueCreateRequest, Store};

#[derive(Parser)]
struct Cli {
    #[arg(long)]
    root: PathBuf,
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Create {
        #[arg(long)]
        request: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();
    let result = match cli.command {
        Command::Create { request } => fs::read(request)
            .map_err(csdlc_v2::V2Error::from)
            .and_then(|bytes| {
                serde_json::from_slice::<IssueCreateRequest>(&bytes)
                    .map_err(csdlc_v2::V2Error::from)
            })
            .and_then(|request| create_issue_draft(&Store::new(cli.root), request))
            .and_then(|value| serde_json::to_value(value).map_err(csdlc_v2::V2Error::from)),
    };
    emit(result, "csdlc-issue");
}

fn emit(result: csdlc_v2::Result<serde_json::Value>, command: &str) {
    match result {
        Ok(value) => println!("{}", serde_json::to_string(&value).expect("JSON")),
        Err(error) => {
            eprintln!("{command}: {error}");
            println!(
                "{}",
                serde_json::json!({"schema":"csdlc.error.v1","code":error.code,"message":error.message})
            );
            std::process::exit(error.code.exit_code());
        }
    }
}
