use std::{fs, path::PathBuf};

use clap::{Parser, Subcommand};
use csdlc_v2::{execute_github_action, public_schema_bundle, GithubActionRequest};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Run {
        #[arg(long)]
        request: PathBuf,
    },
    Schema,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let result = match cli.command {
        Command::Run { request } => run(&request).await,
        Command::Schema => Ok(public_schema_bundle()),
    };
    match result {
        Ok(value) => println!("{}", serde_json::to_string_pretty(&value).expect("JSON")),
        Err(error) => {
            println!(
                "{}",
                serde_json::json!({
                    "schema": "csdlc.error.v1",
                    "code": error.code.to_string(),
                    "message": error.message
                })
            );
            std::process::exit(error.code.exit_code());
        }
    }
}

async fn run(path: &PathBuf) -> csdlc_v2::Result<serde_json::Value> {
    let request: GithubActionRequest = serde_json::from_slice(&fs::read(path)?)?;
    serde_json::to_value(execute_github_action(&request).await?).map_err(Into::into)
}
