use anyhow::Result;

mod agent_cmd;
mod artifact_cmd;
mod commands;
mod csm_cmd;
mod demo_cmd;
mod godel_cmd;
mod identity_cmd;
mod open;
mod pr_cmd;
mod pr_cmd_args;
mod pr_cmd_cards;
mod pr_cmd_prompt;
mod pr_cmd_validate;
mod provider_cmd;
mod run;
pub(crate) mod run_artifacts;
mod run_artifacts_types;
mod runtime_v2_cmd;
#[cfg(test)]
mod tests;
mod tooling_cmd;
mod usage;

use agent_cmd::real_agent;
use artifact_cmd::real_artifact;
use commands::{real_instrument, real_keygen, real_learn, real_sign, real_verify};
use csm_cmd::real_csm;
use demo_cmd::real_demo;
use godel_cmd::real_godel;
use identity_cmd::real_identity;
use pr_cmd::real_pr;
use provider_cmd::real_provider;
use run::{real_resume, run_workflow};
use runtime_v2_cmd::real_runtime_v2;
use tooling_cmd::real_tooling;

fn usage() -> &'static str {
    usage::usage()
}

fn resume_usage() -> &'static str {
    usage::resume_usage()
}

fn version_text() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

fn print_error_chain(err: &anyhow::Error) {
    eprintln!("Error: {err}");

    let mut n = 0;
    let mut cur = err.source();
    while let Some(cause) = cur {
        eprintln!("  {n}: {cause}");
        n += 1;
        cur = cause.source();
    }
}

pub fn run_main() {
    if let Err(err) = real_main() {
        print_error_chain(&err);
        std::process::exit(1);
    }
}

fn real_main() -> Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    dispatch_args(&args)
}

fn dispatch_args(args: &[String]) -> Result<()> {
    if matches!(args.first().map(|s| s.as_str()), Some("--help" | "-h")) {
        println!("{}", usage());
        return Ok(());
    }

    if matches!(args.first().map(|s| s.as_str()), Some("--version" | "-V")) {
        println!("{}", version_text());
        return Ok(());
    }

    match args.first().map(|s| s.as_str()) {
        Some("artifact") => real_artifact(&args[1..]),
        Some("agent") => real_agent(&args[1..]),
        Some("csm") => real_csm(&args[1..]),
        Some("demo") => real_demo(&args[1..]),
        Some("godel") => real_godel(&args[1..]),
        Some("identity") => real_identity(&args[1..]),
        Some("provider") => real_provider(&args[1..]),
        Some("runtime-v2") => real_runtime_v2(&args[1..]),
        Some("pr") => real_pr(&args[1..]),
        Some("keygen") => real_keygen(&args[1..]),
        Some("sign") => real_sign(&args[1..]),
        Some("instrument") => real_instrument(&args[1..]),
        Some("learn") => real_learn(&args[1..]),
        Some("tooling") => real_tooling(&args[1..]),
        Some("verify") => real_verify(&args[1..]),
        Some("resume") => real_resume(&args[1..]),
        _ => run_workflow(args),
    }
}
