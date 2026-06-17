extern crate adl;

#[allow(dead_code)]
#[path = "../cli/mod.rs"]
mod cli;
#[path = "../pr_dispatch_support.rs"]
mod pr_dispatch_support;

const USAGE: &str = "adl-pr-ready - ADL direct PR lifecycle binary\n\n\
Usage:\n\
  adl-pr-ready <issue> [--slug <slug>] [--version <v>] [--no-fetch-issue] [--json]\n\
  adl-pr-ready --help\n\
  adl-pr-ready --version";

fn main() {
    pr_dispatch_support::run_pr_subcommand_main("ready", USAGE, cli::pr_cmd::real_pr);
}
