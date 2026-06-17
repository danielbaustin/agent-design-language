extern crate adl;

#[allow(dead_code)]
#[path = "../cli/mod.rs"]
mod cli;
#[path = "../pr_dispatch_support.rs"]
mod pr_dispatch_support;

const USAGE: &str = "adl-pr-preflight - ADL direct PR lifecycle binary\n\n\
Usage:\n\
  adl-pr-preflight <issue> [--slug <slug>] [--version <v>] [--no-fetch-issue] [--json]\n\
  adl-pr-preflight --help\n\
  adl-pr-preflight --version";

fn main() {
    pr_dispatch_support::run_pr_subcommand_main("preflight", USAGE, cli::pr_cmd::real_pr);
}
