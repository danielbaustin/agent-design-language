use std::fs;
use std::path::PathBuf;

use clap::Parser;
use csdlc_v2::{import_legacy, LegacyImportRequest};

#[derive(Parser)]
struct Cli {
    #[arg(long)]
    request: PathBuf,
}

fn main() {
    let cli = Cli::parse();
    let result = fs::read(&cli.request)
        .map_err(Into::into)
        .and_then(|bytes| serde_json::from_slice::<LegacyImportRequest>(&bytes).map_err(Into::into))
        .and_then(import_legacy);
    match result {
        Ok(report) => {
            println!("{}", serde_json::to_string_pretty(&report).expect("JSON"));
            if report.status == csdlc_v2::migration::ImportStatus::Unsupported {
                std::process::exit(76);
            }
        }
        Err(error) => {
            println!(
                "{}",
                serde_json::json!({"schema":"csdlc.error.v1","code":error.code.to_string(),"message":error.message})
            );
            std::process::exit(error.code.exit_code());
        }
    }
}
