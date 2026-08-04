use anyhow::Result;

mod agent_cmd;
mod artifact_cmd;
mod commands;
mod csm_cmd;
mod csm_service_cmd;
mod csmctl_cmd;
mod demo_cmd;
#[allow(dead_code)]
mod github_token;
mod godel_cmd;
mod identity_cmd;
mod observability;
mod open;
mod process_cmd;
mod provider_cmd;
mod run;
pub(crate) mod run_artifacts;
#[allow(dead_code)]
mod run_artifacts_types;
mod runtime_v2_cmd;
mod runtime_v3_cmd;
mod scheduler_cmd;
mod session_cmd;
#[allow(dead_code)]
mod tokio_runtime;
mod usage;

use agent_cmd::real_agent;
use artifact_cmd::real_artifact;
use commands::{real_instrument, real_keygen, real_learn, real_sign, real_verify};
use csm_cmd::{real_csm, real_csm_standalone};
use csmctl_cmd::real_csmctl;
use demo_cmd::real_demo;
use godel_cmd::real_godel;
use identity_cmd::real_identity;
use process_cmd::real_process;
use provider_cmd::real_provider;
use run::{real_resume, run_workflow};
use runtime_v2_cmd::real_runtime_v2;
use runtime_v3_cmd::real_runtime_v3;
use scheduler_cmd::real_scheduler;
use session_cmd::real_session;

fn usage() -> &'static str {
    usage::usage()
}

fn resume_usage() -> &'static str {
    usage::resume_usage()
}

fn version_text() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

fn real_tooling(_args: &[String]) -> Result<()> {
    Err(anyhow::anyhow!(
        "the v1 tooling multiplexer was removed; use the independent C-SDLC v2 binaries"
    ))
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

#[allow(dead_code)]
pub fn run_runtime_main() {
    if let Err(err) = real_runtime_main() {
        print_error_chain(&err);
        std::process::exit(1);
    }
}

#[allow(dead_code)]
pub fn run_csm_main() {
    adl_runtime::supervision::install_csm_redacting_panic_hook();
    if let Err(err) = real_csm_main() {
        print_error_chain(&err);
        std::process::exit(1);
    }
}

#[allow(dead_code)]
pub fn run_csmctl_main() {
    if let Err(err) = real_csmctl_main() {
        print_error_chain(&err);
        std::process::exit(1);
    }
}

#[allow(dead_code)]
pub fn run_review_main() {
    if let Err(err) = real_review_main() {
        print_error_chain(&err);
        std::process::exit(1);
    }
}

#[allow(dead_code)]
#[cfg(not(test))]
pub fn run_csdlc_main() {
    run_csdlc_main_named("adl-csdlc");
}

#[allow(dead_code)]
#[cfg(not(test))]
pub fn run_csdlc_main_named(binary_name: &'static str) {
    if let Err(err) = real_csdlc_main(binary_name) {
        print_error_chain(&err);
        std::process::exit(1);
    }
}

fn real_main() -> Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    dispatch_args(&args)
}

#[allow(dead_code)]
fn real_runtime_main() -> Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    dispatch_runtime_args(&args)
}

#[allow(dead_code)]
fn real_csm_main() -> Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    dispatch_csm_args(&args)
}

#[allow(dead_code)]
fn real_csmctl_main() -> Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    dispatch_csmctl_args(&args)
}

#[allow(dead_code)]
fn real_review_main() -> Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    dispatch_review_args(&args)
}

#[allow(dead_code)]
#[cfg(not(test))]
fn real_csdlc_main(binary_name: &'static str) -> Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    dispatch_csdlc_args_for(binary_name, &args)
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

    observability::emit_event(
        "adl",
        "dispatch",
        "started",
        &[(
            "subcommand",
            args.first().map(String::as_str).unwrap_or("workflow"),
        )],
    );

    match args.first().map(|s| s.as_str()) {
        Some("artifact") => real_artifact(&args[1..]),
        Some("agent") => real_agent(&args[1..]),
        Some("csm") => real_csm(&args[1..]),
        Some("demo") => real_demo(&args[1..]),
        Some("godel") => real_godel(&args[1..]),
        Some("identity") => real_identity(&args[1..]),
        Some("process") => real_process(&args[1..]),
        Some("provider") => real_provider(&args[1..]),
        Some("runtime-v2") => real_runtime_v2(&args[1..]),
        Some("runtime-v3") => real_runtime_v3(&args[1..]),
        Some("scheduler") => real_scheduler(&args[1..]),
        Some("session") => real_session(&args[1..]),
        Some("pr") => Err(anyhow::anyhow!(
            "the v1 `adl pr` control plane was removed; use the independent C-SDLC v2 binaries"
        )),
        Some("keygen") => real_keygen(&args[1..]),
        Some("sign") => real_sign(&args[1..]),
        Some("instrument") => real_instrument(&args[1..]),
        Some("learn") => real_learn(&args[1..]),
        Some("tooling") => Err(anyhow::anyhow!(
            "the v1 tooling multiplexer was removed; use the independent C-SDLC v2 binaries"
        )),
        Some("verify") => real_verify(&args[1..]),
        Some("resume") => real_resume(&args[1..]),
        _ => run_workflow(args),
    }
}

#[allow(dead_code)]
pub(crate) fn runtime_usage() -> &'static str {
    "adl-runtime - ADL runtime compatibility binary\n\n\
Usage:\n\
  adl-runtime run <adl.yaml> [--print-plan] [--print-prompts] [--trace] [--run] [--resume <run.json>] [--steer <steering.json>] [--overlay <overlay.json>] [--out <dir>] [--quiet] [--open]\n\
  adl-runtime resume <run_id> --adl <path> [--steer <steering.json>]\n\
  adl-runtime agent <tick|run|status|inspect|stop> ...\n\
  adl-runtime scheduler plan --input <bundle.json> [--out <path>] [--json]\n\
  adl-runtime artifact validate-control-path --root <dir>\n\
  adl-runtime csm observatory --packet <visibility-packet.json> ...\n\
  adl-runtime demo <name> ...\n\
  adl-runtime godel <run|inspect|evaluate|affect-slice> ...\n\
  adl-runtime identity <init|show|now|foundation|...> ...\n\
  adl-runtime instrument <graph|replay|replay-bundle|diff-plan|diff-trace|trace-schema|validate-trace-v1|provider-substrate|provider-substrate-schema> ...\n\
  adl-runtime learn export --format <jsonl|bundle-v1|trace-bundle-v2> ...\n\
  adl-runtime provider setup <family> [--model <provider_model_id>] [--out <dir>] [--force]\n\
  adl-runtime runtime-v3 select [--runtime v2|v3] [--json]\n\
  adl-runtime keygen --out-dir <dir>\n\
  adl-runtime sign <adl.yaml> --key <private_key_path> [--key-id <id>] [--out <signed_file>]\n\
  adl-runtime verify <adl.yaml> [--key <public_key_path>]\n\
  adl-runtime --help\n\
  adl-runtime --version\n\n\
Notes:\n\
  adl <adl.yaml> remains available as a compatibility shortcut during migration.\n\
  C-SDLC issue work resolves through csdlc-install and the independent typed v2 binaries; adl-runtime run expects an ADL workflow YAML path."
}

#[allow(dead_code)]
fn dispatch_runtime_args(args: &[String]) -> Result<()> {
    if matches!(
        args.first().map(|s| s.as_str()),
        Some("--help" | "-h" | "help")
    ) {
        println!("{}", runtime_usage());
        return Ok(());
    }

    if matches!(args.first().map(|s| s.as_str()), Some("--version" | "-V")) {
        println!("{}", version_text());
        return Ok(());
    }

    observability::emit_event(
        "adl-runtime",
        "dispatch",
        "started",
        &[("subcommand", args.first().map(String::as_str).unwrap_or(""))],
    );

    match args.first().map(|s| s.as_str()) {
        Some("run") => real_runtime_run(&args[1..]),
        Some("resume") => real_resume(&args[1..]),
        Some("artifact") => real_artifact(&args[1..]),
        Some("agent") => real_agent(&args[1..]),
        Some("scheduler") => real_scheduler(&args[1..]),
        Some("csm") => real_csm(&args[1..]),
        Some("demo") => real_demo(&args[1..]),
        Some("godel") => real_godel(&args[1..]),
        Some("identity") => real_identity(&args[1..]),
        Some("instrument") => real_instrument(&args[1..]),
        Some("learn") => real_learn(&args[1..]),
        Some("provider") => real_provider(&args[1..]),
        Some("runtime-v2") => real_runtime_v2(&args[1..]),
        Some("runtime-v3") => real_runtime_v3(&args[1..]),
        Some("session") => Err(anyhow::anyhow!(
            "adl-runtime does not own polis/session coordination commands. Use adl session <status|claim|heartbeat|release>."
        )),
        Some("keygen") => real_keygen(&args[1..]),
        Some("sign") => real_sign(&args[1..]),
        Some("verify") => real_verify(&args[1..]),
        Some("pr") | Some("tooling") => Err(anyhow::anyhow!(
            "adl-runtime does not own C-SDLC workflow commands. Resolve the final generation with csdlc-install, then use the independent typed v2 binaries."
        )),
        Some(other) => Err(anyhow::anyhow!(
            "unknown adl-runtime command '{other}'. Expected run, resume, agent, artifact, scheduler, csm, demo, godel, identity, instrument, learn, provider, runtime-v2, runtime-v3, keygen, sign, verify, help, or --version."
        )),
        None => Err(anyhow::anyhow!(
            "adl-runtime requires a command. Run `adl-runtime --help` for usage."
        )),
    }
}

#[allow(dead_code)]
fn dispatch_csm_args(args: &[String]) -> Result<()> {
    if matches!(
        args.first().map(|s| s.as_str()),
        Some("--help" | "-h" | "help")
    ) {
        println!("{}", csm_cmd::csm_usage());
        return Ok(());
    }

    if matches!(args.first().map(|s| s.as_str()), Some("--version" | "-V")) {
        println!("{}", version_text());
        return Ok(());
    }

    observability::emit_event(
        "csm",
        "dispatch",
        "started",
        &[("subcommand", args.first().map(String::as_str).unwrap_or(""))],
    );

    real_csm_standalone(args)
}

#[allow(dead_code)]
fn dispatch_csmctl_args(args: &[String]) -> Result<()> {
    if matches!(
        args.first().map(|s| s.as_str()),
        Some("--help" | "-h" | "help")
    ) {
        println!("{}", csmctl_cmd::csmctl_usage());
        return Ok(());
    }

    if matches!(args.first().map(|s| s.as_str()), Some("--version" | "-V")) {
        println!("{}", version_text());
        return Ok(());
    }

    observability::emit_event(
        "csmctl",
        "dispatch",
        "started",
        &[("subcommand", args.first().map(String::as_str).unwrap_or(""))],
    );

    real_csmctl(args)
}

#[allow(dead_code)]
fn real_runtime_run(args: &[String]) -> Result<()> {
    if matches!(args.first().map(|s| s.as_str()), Some("--help" | "-h")) {
        println!("{}", runtime_usage());
        return Ok(());
    }
    let Some(operand) = args.first() else {
        return Err(anyhow::anyhow!(
            "adl-runtime run requires an ADL workflow YAML path."
        ));
    };
    if looks_like_issue_ref(operand) {
        return Err(anyhow::anyhow!(
            "adl-runtime run expects an ADL workflow YAML path, got issue id '{operand}'. C-SDLC issue work resolves through csdlc-install and the independent typed v2 binaries."
        ));
    }
    run_workflow(args)
}

#[allow(dead_code)]
pub(crate) fn review_usage() -> &'static str {
    "adl-review - ADL review tooling compatibility binary\n\n\
Usage:\n\
  adl-review code-review --out <dir> [--backend fixture|ollama] [--visibility packet-only|read-only-repo] ...\n\
  adl-review card-surface --input <input.md> --output <output.md>\n\
  adl-review runtime-surface --review-root <dir>\n\
  adl-review verify-output-provenance --review <review-output.yaml>\n\
  adl-review verify-repo-contract --review <review.md>\n\
  adl-review --help\n\
  adl-review --version\n\n\
Notes:\n\
  Legacy review/tooling multiplexers are removed; use the current direct owner binaries.\n\
  C-SDLC issue work resolves through csdlc-install and the independent typed v2 binaries; runtime workflow YAML belongs to adl-runtime run <adl.yaml>."
}

#[allow(dead_code)]
fn dispatch_review_args(args: &[String]) -> Result<()> {
    if matches!(
        args.first().map(|s| s.as_str()),
        Some("--help" | "-h" | "help")
    ) {
        println!("{}", review_usage());
        return Ok(());
    }

    if matches!(args.first().map(|s| s.as_str()), Some("--version" | "-V")) {
        println!("{}", version_text());
        return Ok(());
    }

    observability::emit_event(
        "adl-review",
        "dispatch",
        "started",
        &[("subcommand", args.first().map(String::as_str).unwrap_or(""))],
    );

    match args.first().map(|s| s.as_str()) {
        Some("code-review") => review_to_tooling_args("code-review", &args[1..])
            .and_then(|mapped| real_tooling(&mapped)),
        Some("card-surface") => review_to_tooling_args("review-card-surface", &args[1..])
            .and_then(|mapped| real_tooling(&mapped)),
        Some("runtime-surface") => review_to_tooling_args("review-runtime-surface", &args[1..])
            .and_then(|mapped| real_tooling(&mapped)),
        Some("verify-output-provenance") => {
            review_to_tooling_args("verify-review-output-provenance", &args[1..])
                .and_then(|mapped| real_tooling(&mapped))
        }
        Some("verify-repo-contract") => {
            review_to_tooling_args("verify-repo-review-contract", &args[1..])
                .and_then(|mapped| real_tooling(&mapped))
        }
        Some("pr") | Some("issue") | Some("tooling") => Err(anyhow::anyhow!(
            "adl-review owns review tooling only. Resolve the final generation with csdlc-install, then use the independent typed v2 binaries for C-SDLC issue work."
        )),
        Some("run") | Some("resume") | Some("agent") | Some("artifact") | Some("csm")
        | Some("demo") | Some("godel") | Some("identity") | Some("instrument") | Some("learn")
        | Some("provider") | Some("runtime-v2") | Some("runtime-v3") | Some("keygen") | Some("sign")
        | Some("verify") => Err(anyhow::anyhow!(
            "adl-review does not run ADL runtime commands. Use adl-runtime run <adl.yaml> for runtime workflows."
        )),
        Some(other) => Err(anyhow::anyhow!(
            "unknown adl-review command '{other}'. Expected code-review, card-surface, runtime-surface, verify-output-provenance, verify-repo-contract, help, or --version."
        )),
        None => Err(anyhow::anyhow!(
            "adl-review requires a command. Run `adl-review --help` for usage."
        )),
    }
}

#[allow(dead_code)]
fn review_to_tooling_args(subcommand: &str, args: &[String]) -> Result<Vec<String>> {
    if args
        .iter()
        .any(|arg| matches!(arg.as_str(), "help" | "--help" | "-h"))
    {
        return Ok(vec!["help".to_string()]);
    }
    let mut mapped = Vec::with_capacity(args.len() + 1);
    mapped.push(subcommand.to_string());
    mapped.extend(args.iter().cloned());
    Ok(mapped)
}

#[allow(dead_code)]
fn looks_like_issue_ref(value: &str) -> bool {
    let issue = value.strip_prefix('#').unwrap_or(value);
    !issue.is_empty() && issue.chars().all(|ch| ch.is_ascii_digit())
}

#[allow(dead_code)]
pub(crate) fn csdlc_usage_for(binary_name: &str) -> String {
    let title = if binary_name == "csdlc" {
        "csdlc - ADL C-SDLC workflow control-plane binary"
    } else {
        "adl-csdlc - ADL C-SDLC compatibility binary"
    };
    format!(
        "{title}\n\n\
Usage:\n\
  {binary_name} --help\n\
  {binary_name} --version\n\n\
Notes:\n\
  The v1 lifecycle surface is removed. Resolve the final generation with csdlc-install and use the independent typed v2 binaries.\n\
  This compatibility binary has no operational lifecycle or tooling commands. Runtime workflow YAML belongs to adl-runtime run <adl.yaml>."
    )
}

#[allow(dead_code)]
pub(crate) fn csdlc_usage() -> String {
    csdlc_usage_for("adl-csdlc")
}

#[allow(dead_code)]
fn dispatch_csdlc_args(args: &[String]) -> Result<()> {
    dispatch_csdlc_args_for("adl-csdlc", args)
}

#[allow(dead_code)]
fn dispatch_csdlc_args_for(binary_name: &'static str, args: &[String]) -> Result<()> {
    if matches!(
        args.first().map(|s| s.as_str()),
        Some("--help" | "-h" | "help")
    ) {
        println!("{}", csdlc_usage_for(binary_name));
        return Ok(());
    }

    if matches!(args.first().map(|s| s.as_str()), Some("--version" | "-V")) {
        println!("{}", version_text());
        return Ok(());
    }

    observability::emit_event(
        binary_name,
        "dispatch",
        "started",
        &[("subcommand", args.first().map(String::as_str).unwrap_or(""))],
    );

    match args.first().map(|s| s.as_str()) {
        Some("pr") | Some("issue") => Err(anyhow::anyhow!(
            "{binary_name} v1 lifecycle commands were removed; use the independent C-SDLC v2 binaries"
        )),
        Some("tooling") => Err(anyhow::anyhow!(
            "{binary_name} tooling was removed; use the current direct owner binaries"
        )),
        Some("run") => Err(anyhow::anyhow!(
            "{binary_name} does not run ADL workflow YAML. Use adl-runtime run <adl.yaml> for runtime workflows; resolve C-SDLC issue execution through csdlc-install and the typed v2 binaries."
        )),
        Some(other) => Err(anyhow::anyhow!(
            "unknown {binary_name} command '{other}'. Expected help or --version."
        )),
        None => Err(anyhow::anyhow!(
            "{binary_name} requires a command. Run `{binary_name} --help` for usage."
        )),
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Mutex, MutexGuard, OnceLock};

    pub fn env_lock() -> MutexGuard<'static, ()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(()))
            .lock()
            .expect("environment lock")
    }
}
