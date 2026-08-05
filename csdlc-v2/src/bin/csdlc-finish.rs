use std::fs;
use std::path::PathBuf;

use clap::Parser;
use csdlc_v2::finish::{execute_finish, FinishRequest, FinishResult};

#[derive(Parser)]
#[command(about = "Finish one C-SDLC v2 issue from exact live GitHub terminal truth")]
struct Cli {
    #[arg(long)]
    root: PathBuf,
    #[arg(long)]
    request: PathBuf,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    match run(cli).await {
        Ok(result) => println!("{}", serde_json::to_string_pretty(&result).expect("JSON")),
        Err(error) => {
            println!(
                "{}",
                serde_json::json!({"schema":"csdlc.error.v1","code":error.code.to_string(),"message":error.message})
            );
            std::process::exit(error.code.exit_code());
        }
    }
}

async fn run(cli: Cli) -> csdlc_v2::Result<FinishResult> {
    let request: FinishRequest = serde_json::from_slice(&fs::read(cli.request)?)?;
    execute_finish(&cli.root, &request).await
}
