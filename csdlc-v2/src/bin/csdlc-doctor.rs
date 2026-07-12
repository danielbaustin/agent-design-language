use std::path::PathBuf;

use clap::Parser;
use csdlc_v2::{diagnose, Store};

#[derive(Parser)]
#[command(name = "csdlc-doctor")]
struct Args {
    #[arg(long, default_value = ".")]
    repo: PathBuf,
    #[arg(long)]
    issue: u64,
}

fn main() {
    let args = Args::parse();
    let report = diagnose(&Store::new(args.repo), args.issue);
    match serde_json::to_string_pretty(&report) {
        Ok(json) => println!("{json}"),
        Err(error) => {
            eprintln!("csdlc-doctor: {error}");
            std::process::exit(70);
        }
    }
    let code = match report.status {
        csdlc_v2::doctor::DoctorStatus::Pass => 0,
        csdlc_v2::doctor::DoctorStatus::Block => 2,
        csdlc_v2::doctor::DoctorStatus::Corrupt => 3,
        csdlc_v2::doctor::DoctorStatus::Interrupted => 4,
    };
    std::process::exit(code);
}
