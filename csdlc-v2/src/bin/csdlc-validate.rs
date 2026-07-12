use clap::Parser;
use csdlc_v2::{execute, ExecutionRequest};
use std::{fs, path::PathBuf};
#[derive(Parser)]
struct Cli {
    #[arg(long)]
    request: PathBuf,
}
fn main() {
    let cli = Cli::parse();
    let result = fs::read(cli.request)
        .map_err(csdlc_v2::V2Error::from)
        .and_then(|b| {
            serde_json::from_slice::<ExecutionRequest>(&b).map_err(csdlc_v2::V2Error::from)
        })
        .and_then(execute);
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
