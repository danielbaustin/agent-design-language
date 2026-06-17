extern crate adl;

#[allow(dead_code)]
#[path = "../cli/mod.rs"]
mod cli;
#[path = "../pr_dispatch_support.rs"]
mod pr_dispatch_support;

const USAGE: &str = "adl-pr-run - ADL direct PR lifecycle binary\n\n\
Usage:\n\
  adl-pr-run <issue> [--slug <slug>] [--title \"<title>\"] [--prefix codex] [--no-fetch-issue] [--version <v0.85|v0.87.1>] [--allow-open-pr-wave]\n\
  adl-pr-run --help\n\
  adl-pr-run --version";

fn main() {
    pr_dispatch_support::run_pr_subcommand_main("start", USAGE, cli::pr_cmd::real_pr);
}
