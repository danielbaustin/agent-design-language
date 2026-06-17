extern crate adl;

#[allow(dead_code)]
#[path = "../cli/mod.rs"]
mod cli;
#[path = "../pr_dispatch_support.rs"]
mod pr_dispatch_support;

const USAGE: &str = "adl-pr-doctor - ADL direct PR lifecycle binary\n\n\
Usage:\n\
  adl-pr-doctor <issue> [--slug <slug>] [--no-fetch-issue] [--version <v0.2>] [--mode full|ready|preflight] [--json]\n\
  adl-pr-doctor --help\n\
  adl-pr-doctor --version";

fn main() {
    pr_dispatch_support::run_pr_subcommand_main("doctor", USAGE, cli::pr_cmd::real_pr);
}
