use anyhow::Result;

mod artifact_cmd;
mod commands;
mod demo_cmd;
mod godel_cmd;
mod identity_cmd;
mod open;
mod pr_cmd;
mod pr_cmd_args;
mod pr_cmd_prompt;
mod pr_cmd_validate;
mod run;
pub(crate) mod run_artifacts;
#[cfg(test)]
mod tests;
mod usage;

use artifact_cmd::real_artifact;
use commands::{real_instrument, real_keygen, real_learn, real_sign, real_verify};
use demo_cmd::real_demo;
use godel_cmd::real_godel;
use identity_cmd::real_identity;
use pr_cmd::real_pr;
use run::{real_resume, run_workflow};

fn usage() -> &'static str {
    usage::usage()
}

fn resume_usage() -> &'static str {
    usage::resume_usage()
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

    if matches!(args.first().map(|s| s.as_str()), Some("--help" | "-h")) {
        println!("{}", usage());
        return Ok(());
    }

    match args.first().map(|s| s.as_str()) {
        Some("artifact") => real_artifact(&args[1..]),
        Some("demo") => real_demo(&args[1..]),
        Some("godel") => real_godel(&args[1..]),
        Some("identity") => real_identity(&args[1..]),
        Some("pr") => real_pr(&args[1..]),
        Some("keygen") => real_keygen(&args[1..]),
        Some("sign") => real_sign(&args[1..]),
        Some("instrument") => real_instrument(&args[1..]),
        Some("learn") => real_learn(&args[1..]),
        Some("verify") => real_verify(&args[1..]),
        Some("resume") => real_resume(&args[1..]),
        _ => run_workflow(&args),
    }
}
