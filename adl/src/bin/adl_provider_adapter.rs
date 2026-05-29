use adl::provider_adapter::execute_provider_invocation;
use adl::provider_communication::{ProviderInvocationRequestV1, ProviderRunLoggerV1};
use anyhow::{Context, Result};
use clap::Parser;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(name = "adl-provider-adapter")]
#[command(about = "Run one ADL provider invocation through the Rust provider adapter")]
struct Args {
    #[arg(long)]
    request: PathBuf,

    #[arg(long)]
    out: PathBuf,

    #[arg(long)]
    log: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let request_text = fs::read_to_string(&args.request)
        .with_context(|| format!("read request file {}", args.request.display()))?;
    let request: ProviderInvocationRequestV1 = serde_json::from_str(&request_text)
        .with_context(|| format!("parse request file {}", args.request.display()))?;
    let run_id = request
        .run_id
        .clone()
        .unwrap_or_else(|| format!("{}:{}", request.lane_ref, request.route.provider_model_id));
    let mut logger = ProviderRunLoggerV1::create(&args.log, run_id)
        .with_context(|| format!("open run log {}", args.log.display()))?;
    let result = execute_provider_invocation(request, &mut logger);
    fs::write(&args.out, serde_json::to_string_pretty(&result)? + "\n")
        .with_context(|| format!("write result file {}", args.out.display()))?;
    println!("result={}", args.out.display());
    println!("run_log={}", args.log.display());
    println!("watch=tail -f {}", args.log.display());
    Ok(())
}
