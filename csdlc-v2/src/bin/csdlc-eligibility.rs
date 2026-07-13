use std::fs;
use std::path::PathBuf;

use clap::{Parser, Subcommand};
use csdlc_v2::{
    eligibility_schema_bundle, evaluate_deletion_eligibility, DeletionEligibilityRequest,
};

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Evaluate {
        #[arg(long)]
        repo: PathBuf,
        #[arg(long)]
        request: PathBuf,
    },
    Schema,
}

fn main() {
    let args = Args::parse();
    if matches!(args.command, Command::Schema) {
        println!(
            "{}",
            serde_json::to_string_pretty(&eligibility_schema_bundle()).expect("JSON")
        );
        return;
    }
    let Command::Evaluate { repo, request } = args.command else {
        unreachable!()
    };
    let result = fs::read(&request)
        .map_err(Into::into)
        .and_then(|bytes| {
            serde_json::from_slice::<DeletionEligibilityRequest>(&bytes).map_err(Into::into)
        })
        .and_then(|request| evaluate_deletion_eligibility(&repo, &request));
    match result {
        Ok(value) => println!("{}", serde_json::to_string_pretty(&value).expect("JSON")),
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(error.code.exit_code());
        }
    }
}
