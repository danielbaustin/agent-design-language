use clap::{Parser, Subcommand};
use csdlc_v2::{initialize_native_json, Store};
use std::{fs, path::PathBuf};

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
            .and_then(|bytes| initialize_native_json(&Store::new(cli.root), &bytes)),
    };
    match result {
        Ok(record) => println!("{}", serde_json::to_string(&record).expect("JSON")),
        Err(error) => {
            eprintln!("csdlc-issue: {}", error);
            println!(
                "{}",
                serde_json::json!({"schema":"csdlc.error.v1","code":error.code,"message":error.message})
            );
            std::process::exit(error.code.exit_code());
        }
    }
}
