use std::{fs, path::PathBuf};

use clap::{Parser, Subcommand};
use csdlc_v2::{
    github::{collect_pr_state, PrStateRequest},
    public_schema_bundle, ErrorCode, GithubAction, GithubActionRequest, V2Error,
};

#[derive(Parser)]
#[command(name = "csdlc-github-pr")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    State {
        #[arg(long)]
        request: PathBuf,
    },
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
        Command::State { request } => state(&request).await,
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

async fn state(path: &PathBuf) -> csdlc_v2::Result<serde_json::Value> {
    let request: PrStateRequest = serde_json::from_slice(&fs::read(path)?)?;
    serde_json::to_value(collect_pr_state(&request).await?).map_err(Into::into)
}

async fn run(path: &PathBuf) -> csdlc_v2::Result<serde_json::Value> {
    let request: GithubActionRequest = serde_json::from_slice(&fs::read(path)?)?;
    if !matches!(request.action, GithubAction::PrState) {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "csdlc-github-pr only accepts pr_state actions; use csdlc-github-issue for issue actions",
        ));
    }
    let pr_request = PrStateRequest::try_from(&request)?;
    serde_json::to_value(collect_pr_state(&pr_request).await?).map_err(Into::into)
}
