use clap::Parser;
use clap::Subcommand;
use csdlc_v2::github::{collect_pr_state, PrStateRequest};
use csdlc_v2::{classify_shepherd, ShepherdInput};
use std::{fs, path::PathBuf};

#[derive(Parser)]
#[command(about = "Classify read-only C-SDLC shepherd state from typed JSON")]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
    #[arg(long, conflicts_with_all = ["input", "example"], help = "Print the ShepherdInput JSON schema")]
    schema: bool,
    #[arg(long, conflicts_with_all = ["input", "schema"], value_name = "NAME", help = "Print a checked example (ready, waiting, retryable, repair_required, operator_required)")]
    example: Option<String>,
    #[arg(long, conflicts_with_all = ["schema", "example"], help = "Read a ShepherdInput JSON file")]
    input: Option<PathBuf>,
    #[arg(long)]
    pr_state_request: Option<PathBuf>,
}
#[derive(Subcommand)]
enum Command {
    #[command(name = "schema", about = "Print the ShepherdInput JSON schema")]
    Schema,
    #[command(name = "example", about = "Print a checked ShepherdInput example")]
    Example { name: String },
}

fn example(name: &str) -> Option<ShepherdInput> {
    let base = |validation,
                dependency_wait,
                retryable_failure,
                repair_needed,
                operator_decision_needed| {
        ShepherdInput {
            validation,
            dependency_wait,
            retryable_failure,
            repair_needed,
            operator_decision_needed,
        }
    };
    Some(match name {
        "ready" => base(
            Some(csdlc_v2::pvf::ValidationDisposition::LocalPass),
            false,
            false,
            false,
            false,
        ),
        "waiting" => base(None, true, false, false, false),
        "retryable" => base(
            Some(csdlc_v2::pvf::ValidationDisposition::Failed),
            false,
            true,
            false,
            false,
        ),
        "repair_required" => base(
            Some(csdlc_v2::pvf::ValidationDisposition::Failed),
            false,
            false,
            true,
            false,
        ),
        "operator_required" => base(
            Some(csdlc_v2::pvf::ValidationDisposition::Failed),
            false,
            false,
            false,
            true,
        ),
        _ => return None,
    })
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    if let Some(path) = cli.pr_state_request {
        let request: PrStateRequest = serde_json::from_slice(&fs::read(path).unwrap_or_default())
            .unwrap_or_else(|e| {
                eprintln!("{e}");
                std::process::exit(64)
            });
        match collect_pr_state(&request).await {
            Ok(packet) => println!("{}", serde_json::to_string_pretty(&packet).expect("JSON")),
            Err(error) => {
                eprintln!("{}", error.message);
                std::process::exit(error.code.exit_code());
            }
        }
        return;
    }
    if cli.schema || matches!(cli.command, Some(Command::Schema)) {
        println!(
            "{}",
            serde_json::to_string_pretty(&schemars::schema_for!(ShepherdInput)).expect("JSON")
        );
        return;
    }
    if let Some(name) = cli.example.or(match cli.command {
        Some(Command::Example { name }) => Some(name),
        _ => None,
    }) {
        let Some(input) = example(&name) else {
            eprintln!("unknown example {name}; expected ready, waiting, retryable, repair_required, or operator_required");
            std::process::exit(64);
        };
        println!("{}", serde_json::to_string_pretty(&input).expect("JSON"));
        return;
    }
    let Some(input) = cli.input else {
        eprintln!("provide --input <PATH>, --schema, or --example <NAME>");
        std::process::exit(64);
    };
    let bytes = fs::read(input).unwrap_or_else(|e| {
        eprintln!("{e}");
        std::process::exit(74)
    });
    let input: ShepherdInput = serde_json::from_slice(&bytes).unwrap_or_else(|e| {
        eprintln!("{e}");
        std::process::exit(64)
    });
    println!(
        "{}",
        serde_json::to_string(&classify_shepherd(&input)).expect("JSON")
    );
}
