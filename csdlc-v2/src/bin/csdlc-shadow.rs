use std::fs;
use std::path::PathBuf;

use clap::{Parser, Subcommand};
use csdlc_v2::{compare_shadow, generate_compatibility_view, NormalizedOutcome, Store};

#[derive(Parser)]
struct Cli {
    #[arg(long, default_value = ".")]
    root: PathBuf,
    #[command(subcommand)]
    command: Command,
}
#[derive(Subcommand)]
enum Command {
    Compare {
        #[arg(long)]
        issue: u64,
        #[arg(long)]
        legacy_observation: PathBuf,
    },
    GenerateView {
        #[arg(long)]
        issue: u64,
    },
    Schema,
}

fn main() {
    let cli = Cli::parse();
    let store = Store::new(&cli.root);
    let result: csdlc_v2::Result<serde_json::Value> = match cli.command {
        Command::Schema => Ok(csdlc_v2::public_schema_bundle()),
        Command::Compare {
            issue,
            legacy_observation,
        } => (|| {
            let legacy: NormalizedOutcome = serde_json::from_slice(&fs::read(legacy_observation)?)?;
            Ok(serde_json::to_value(compare_shadow(
                &legacy,
                &NormalizedOutcome::from_v2(&store, issue)?,
            ))?)
        })(),
        Command::GenerateView { issue } => (|| {
            let view = generate_compatibility_view(&store, issue)?;
            let output = csdlc_v2::write_compatibility_view_atomic(&store, issue, &view)?;
            Ok(
                serde_json::json!({"schema":"csdlc.compatibility_view_result.v1","issue":issue,"output":output}),
            )
        })(),
    };
    match result {
        Ok(value) => println!("{}", serde_json::to_string_pretty(&value).expect("JSON")),
        Err(error) => {
            println!(
                "{}",
                serde_json::json!({"schema":"csdlc.error.v1","code":error.code.to_string(),"message":error.message})
            );
            std::process::exit(error.code.exit_code());
        }
    }
}
