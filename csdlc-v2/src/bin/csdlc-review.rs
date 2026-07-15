use clap::{Parser, Subcommand};
use csdlc_v2::{
    assign_review, evaluate_publication_review_in_repo, record_review, recover_review,
    ReviewAssignmentRequest, ReviewEvidence, ReviewRecordRequest, ReviewRecoveryRequest, Store,
};
use serde::Deserialize;
use std::{fs, path::PathBuf};
#[derive(Parser)]
struct Cli {
    #[arg(long, default_value = ".")]
    root: PathBuf,
    #[command(subcommand)]
    command: Command,
}
#[derive(Subcommand)]
enum Command {
    Assign {
        #[arg(long)]
        request: PathBuf,
    },
    Record {
        #[arg(long)]
        request: PathBuf,
    },
    Recover {
        #[arg(long)]
        request: PathBuf,
    },
    Guard {
        #[arg(long)]
        request: PathBuf,
    },
}
#[derive(Deserialize)]
struct GuardRequest {
    evidence: Option<ReviewEvidence>,
    scope: Vec<String>,
}
fn read<T: for<'a> Deserialize<'a>>(path: &PathBuf) -> csdlc_v2::Result<T> {
    Ok(serde_json::from_slice(&fs::read(path)?)?)
}
fn main() {
    let cli = Cli::parse();
    let store = Store::new(cli.root);
    let result = match cli.command {
        Command::Assign { request } => read::<ReviewAssignmentRequest>(&request)
            .and_then(|v| assign_review(&store, v))
            .and_then(json),
        Command::Record { request } => read::<ReviewRecordRequest>(&request)
            .and_then(|v| record_review(&store, v))
            .and_then(json),
        Command::Recover { request } => read::<ReviewRecoveryRequest>(&request)
            .and_then(|v| recover_review(&store, v))
            .and_then(json),
        Command::Guard { request } => read::<GuardRequest>(&request).and_then(|v| {
            let current = csdlc_v2::git::substantive_revision(store.root(), &v.scope)?;
            json(evaluate_publication_review_in_repo(
                store.root(),
                v.evidence.as_ref(),
                &current,
            ))
        }),
    };
    match result {
        Ok(v) => println!("{v}"),
        Err(e) => {
            println!(
                "{}",
                serde_json::json!({"schema":"csdlc.error.v1","code":e.code,"message":e.message})
            );
            std::process::exit(e.code.exit_code())
        }
    }
}
fn json<T: serde::Serialize>(value: T) -> csdlc_v2::Result<String> {
    Ok(serde_json::to_string(&value)?)
}
