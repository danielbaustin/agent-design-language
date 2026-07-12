use clap::Parser;
use csdlc_v2::{bind_issue, BindRequest, Store};
use std::{fs, path::PathBuf};

#[derive(Parser)]
struct Cli {
    #[arg(long)]
    root: PathBuf,
    #[arg(long)]
    request: PathBuf,
}

fn main() {
    let cli = Cli::parse();
    let result = fs::read(&cli.request)
        .map_err(csdlc_v2::V2Error::from)
        .and_then(|bytes| {
            serde_json::from_slice::<BindRequest>(&bytes).map_err(csdlc_v2::V2Error::from)
        })
        .and_then(|request| bind_issue(&Store::new(cli.root), request));
    match result {
        Ok(value) => println!("{}", serde_json::to_string(&value).expect("JSON")),
        Err(error) => {
            eprintln!("csdlc-bind: {}", error);
            println!(
                "{}",
                serde_json::json!({"schema":"csdlc.error.v1","code":error.code,"message":error.message})
            );
            std::process::exit(error.code.exit_code());
        }
    }
}
