use clap::Parser;
use csdlc_v2::{classify_schedule, ScheduleInput};
use std::{fs, path::PathBuf};
#[derive(Parser)]
struct Cli {
    #[arg(long)]
    input: PathBuf,
}
fn main() {
    let cli = Cli::parse();
    let bytes = fs::read(cli.input).unwrap_or_else(|e| {
        eprintln!("{e}");
        std::process::exit(74)
    });
    let input: ScheduleInput = serde_json::from_slice(&bytes).unwrap_or_else(|e| {
        eprintln!("{e}");
        std::process::exit(64)
    });
    println!(
        "{}",
        serde_json::to_string(&classify_schedule(&input)).expect("JSON")
    );
}
