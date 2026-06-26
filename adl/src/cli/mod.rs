use anyhow::Result;

mod agent_cmd;
mod artifact_cmd;
mod commands;
mod csm_cmd;
mod demo_cmd;
mod github_token;
mod godel_cmd;
mod identity_cmd;
mod observability;
mod open;
pub(crate) mod pr_cmd;
mod pr_cmd_args;
mod pr_cmd_cards;
mod pr_cmd_prompt;
mod pr_cmd_validate;
mod process_cmd;
mod provider_cmd;
mod run;
pub(crate) mod run_artifacts;
mod run_artifacts_types;
mod runtime_v2_cmd;
mod scheduler_cmd;
mod session_cmd;
#[cfg(test)]
mod tests;
mod tokio_runtime;
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
use process_cmd::real_process;
use provider_cmd::real_provider;
use run::{real_resume, run_workflow};
use runtime_v2_cmd::real_runtime_v2;
use scheduler_cmd::real_scheduler;
use session_cmd::real_session;
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

#[allow(dead_code)]
pub fn run_runtime_main() {
    if let Err(err) = real_runtime_main() {
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
    if let Err(err) = real_csdlc_main() {
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
fn real_review_main() -> Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    dispatch_review_args(&args)
}

#[allow(dead_code)]
#[cfg(not(test))]
fn real_csdlc_main() -> Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    dispatch_csdlc_args(&args)
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
        Some("scheduler") => real_scheduler(&args[1..]),
        Some("session") => real_session(&args[1..]),
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
  adl-runtime provider setup <family> [--out <dir>] [--force]\n\
  adl-runtime keygen --out-dir <dir>\n\
  adl-runtime sign <adl.yaml> --key <private_key_path> [--key-id <id>] [--out <signed_file>]\n\
  adl-runtime verify <adl.yaml> [--key <public_key_path>]\n\
  adl-runtime --help\n\
  adl-runtime --version\n\n\
Notes:\n\
  adl <adl.yaml> remains available as a compatibility shortcut during migration.\n\
  C-SDLC issue work belongs to adl/tools/pr.sh run <issue>; adl-runtime run expects an ADL workflow YAML path."
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
        Some("session") => Err(anyhow::anyhow!(
            "adl-runtime does not own polis/session coordination commands. Use adl session <status|claim|heartbeat|release>."
        )),
        Some("keygen") => real_keygen(&args[1..]),
        Some("sign") => real_sign(&args[1..]),
        Some("verify") => real_verify(&args[1..]),
        Some("pr") | Some("tooling") => Err(anyhow::anyhow!(
            "adl-runtime does not own C-SDLC workflow commands. Use adl/tools/pr.sh run <issue> for issue work or adl-csdlc for C-SDLC compatibility surfaces."
        )),
        Some(other) => Err(anyhow::anyhow!(
            "unknown adl-runtime command '{other}'. Expected run, resume, agent, artifact, scheduler, csm, demo, godel, identity, instrument, learn, provider, runtime-v2, keygen, sign, verify, help, or --version."
        )),
        None => Err(anyhow::anyhow!(
            "adl-runtime requires a command. Run `adl-runtime --help` for usage."
        )),
    }
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
            "adl-runtime run expects an ADL workflow YAML path, got issue id '{operand}'. C-SDLC issue work belongs to adl/tools/pr.sh run <issue>."
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
  adl tooling code-review and related review commands remain available as compatibility shims during migration.\n\
  C-SDLC issue work belongs to adl/tools/pr.sh run <issue>; runtime workflow YAML belongs to adl-runtime run <adl.yaml>."
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
            "adl-review owns review tooling only. Use adl/tools/pr.sh run <issue> or adl-csdlc for C-SDLC issue work."
        )),
        Some("run") | Some("resume") | Some("agent") | Some("artifact") | Some("csm")
        | Some("demo") | Some("godel") | Some("identity") | Some("instrument") | Some("learn")
        | Some("provider") | Some("runtime-v2") | Some("keygen") | Some("sign")
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
pub(crate) fn csdlc_usage() -> &'static str {
    "adl-csdlc - ADL C-SDLC compatibility binary\n\n\
Usage:\n\
  adl-csdlc pr <create|init|start|doctor|ready|preflight|finish|closeout> ...\n\
  adl-csdlc issue <create|init|run|doctor|finish|closeout> ...\n\
  adl-csdlc issue run <issue> [--slug <slug>] [--version <v>]\n\
  adl-csdlc tooling <card-prompt|csdlc-prompt-editor|generate-wp-issue-wave|lint-prompt-spec|prompt-template|srp-sor-update|validate-structured-prompt|...> ...\n\
  adl-csdlc --help\n\
  adl-csdlc --version\n\n\
Notes:\n\
  adl/tools/pr.sh remains the canonical agent-facing issue wrapper during migration.\n\
  GitHub issue/PR metadata interpretation is owned by the shared pr control-plane client layer.\n\
  adl-csdlc issue run expects a numeric issue id. Runtime workflow YAML belongs to adl-runtime run <adl.yaml>."
}

#[allow(dead_code)]
fn dispatch_csdlc_args(args: &[String]) -> Result<()> {
    if matches!(
        args.first().map(|s| s.as_str()),
        Some("--help" | "-h" | "help")
    ) {
        println!("{}", csdlc_usage());
        return Ok(());
    }

    if matches!(args.first().map(|s| s.as_str()), Some("--version" | "-V")) {
        println!("{}", version_text());
        return Ok(());
    }

    observability::emit_event(
        "adl-csdlc",
        "dispatch",
        "started",
        &[("subcommand", args.first().map(String::as_str).unwrap_or(""))],
    );

    match args.first().map(|s| s.as_str()) {
        Some("pr") => {
            reject_csdlc_runtime_run("adl-csdlc pr", &args[1..])?;
            real_pr(&args[1..])
        }
        Some("issue") => real_csdlc_issue(&args[1..]),
        Some("tooling") => real_tooling(&args[1..]),
        Some("run") => Err(anyhow::anyhow!(
            "adl-csdlc does not run ADL workflow YAML. Use adl-runtime run <adl.yaml> for runtime workflows or adl-csdlc issue run <issue> for C-SDLC issue execution."
        )),
        Some(other) => Err(anyhow::anyhow!(
            "unknown adl-csdlc command '{other}'. Expected pr, issue, tooling, help, or --version."
        )),
        None => Err(anyhow::anyhow!(
            "adl-csdlc requires a command. Run `adl-csdlc --help` for usage."
        )),
    }
}

#[allow(dead_code)]
fn real_csdlc_issue(args: &[String]) -> Result<()> {
    let pr_args = csdlc_issue_to_pr_args(args)?;
    real_pr(&pr_args)
}

#[allow(dead_code)]
fn csdlc_issue_to_pr_args(args: &[String]) -> Result<Vec<String>> {
    reject_csdlc_runtime_run("adl-csdlc issue", args)?;
    let Some(subcommand) = args.first().map(|s| s.as_str()) else {
        return Err(anyhow::anyhow!(
            "adl-csdlc issue requires a pr-compatible subcommand such as run, doctor, finish, or closeout."
        ));
    };
    if subcommand == "run" {
        let Some(issue) = args.get(1) else {
            return Err(anyhow::anyhow!(
                "adl-csdlc issue run requires a numeric issue id."
            ));
        };
        if !issue.chars().all(|ch| ch.is_ascii_digit()) {
            return Err(anyhow::anyhow!(
                "adl-csdlc issue run expects a numeric issue id, got '{issue}'. Runtime workflow YAML belongs to adl-runtime run <adl.yaml>."
            ));
        }
        let mut mapped = Vec::with_capacity(args.len());
        mapped.push("start".to_string());
        mapped.extend(args[1..].iter().cloned());
        return Ok(mapped);
    }
    Ok(args.to_vec())
}

#[allow(dead_code)]
fn reject_csdlc_runtime_run(context: &str, args: &[String]) -> Result<()> {
    if args.first().map(|s| s.as_str()) != Some("run") {
        return Ok(());
    }
    let Some(operand) = args.get(1) else {
        return Ok(());
    };
    if looks_like_adl_workflow_path(operand) {
        return Err(anyhow::anyhow!(
            "{context} run cannot execute ADL workflow YAML '{operand}'. Use adl-runtime run <adl.yaml> for runtime workflows."
        ));
    }
    Ok(())
}

#[allow(dead_code)]
fn looks_like_adl_workflow_path(value: &str) -> bool {
    value.ends_with(".adl.yaml") || value.ends_with(".adl.yml")
}
