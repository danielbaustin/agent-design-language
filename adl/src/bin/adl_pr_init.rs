extern crate adl;

#[allow(dead_code)]
#[path = "../cli/mod.rs"]
mod cli;
#[path = "../pr_dispatch_support.rs"]
mod pr_dispatch_support;

const USAGE: &str = "adl-pr-init - ADL direct PR lifecycle binary\n\n\
Usage:\n\
  adl-pr-init <issue> [--slug <slug>] [--title \"<title>\"] [--no-fetch-issue] [--version <v0.85|v0.87.1>]\n\
  adl-pr-init --help\n\
  adl-pr-init --version";

fn main() {
    pr_dispatch_support::run_pr_subcommand_main("init", USAGE, cli::pr_cmd::real_pr);
}
