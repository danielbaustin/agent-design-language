use clap::{Parser, Subcommand};
use csdlc_v2::{execute, finalize, ExecutionRequest, FinalizeRequest, Store};
use std::{fs, path::PathBuf};
#[derive(Parser)]
struct Cli {
    #[arg(long, default_value = ".")]
    root: PathBuf,
    #[command(subcommand)]
    command: Option<Command>,
    #[arg(long)]
    request: Option<PathBuf>,
}
#[derive(Subcommand)]
enum Command {
    Issue {
        #[arg(long)]
        issue: u64,
    },
    Finalize {
        #[arg(long)]
        request: PathBuf,
    },
}
fn main() {
    let cli = Cli::parse();
    let result = match cli.command {
        Some(Command::Issue { issue }) => {
            csdlc_v2::doctor::diagnose_result(&Store::new(cli.root), issue)
                .and_then(|report| serde_json::to_value(report).map_err(Into::into))
        }
        Some(Command::Finalize { request }) => fs::read(request)
            .map_err(csdlc_v2::V2Error::from)
            .and_then(|b| serde_json::from_slice::<FinalizeRequest>(&b).map_err(Into::into))
            .and_then(|request| finalize(&Store::new(cli.root), request))
            .and_then(|record| serde_json::to_value(record).map_err(Into::into)),
        None => (|| {
            let path = cli.request.ok_or_else(|| {
                csdlc_v2::V2Error::new(
                    csdlc_v2::ErrorCode::InvalidInput,
                    "--request or finalize is required",
                )
            })?;
            let request: ExecutionRequest = serde_json::from_slice(&fs::read(path)?)?;
            Ok(serde_json::to_value(execute(request)?)?)
        })(),
    };
    match result {
        Ok(v) => println!("{}", serde_json::to_string(&v).expect("JSON")),
        Err(e) => {
            println!(
                "{}",
                serde_json::json!({"schema":"csdlc.error.v1","code":e.code,"message":e.message})
            );
            std::process::exit(e.code.exit_code())
        }
    }
}
