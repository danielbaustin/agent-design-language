use clap::Parser;
use csdlc_v2::github::{collect_pr_state, PrStateRequest};
use std::path::PathBuf;

#[derive(Parser)]
struct Cli {
    #[arg(long)]
    request: PathBuf,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let request: PrStateRequest = match std::fs::read(&cli.request)
        .ok()
        .and_then(|b| serde_json::from_slice(&b).ok())
    {
        Some(value) => value,
        None => {
            eprintln!("invalid PR-state request");
            std::process::exit(64);
        }
    };
    match collect_pr_state(&request).await {
        Ok(packet) => println!("{}", serde_json::to_string_pretty(&packet).expect("JSON")),
        Err(error) => {
            eprintln!("{}", error.message);
            std::process::exit(error.code.exit_code());
        }
    }
}
