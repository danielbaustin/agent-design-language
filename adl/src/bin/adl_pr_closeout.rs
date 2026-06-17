extern crate adl;

#[allow(dead_code)]
#[path = "../cli/mod.rs"]
mod cli;
#[path = "../pr_dispatch_support.rs"]
mod pr_dispatch_support;

const USAGE: &str = "adl-pr-closeout - ADL direct PR lifecycle binary\n\n\
Usage:\n\
  adl-pr-closeout <issue> [--slug <slug>] [--version <v>] [--no-fetch-issue]\n\
  adl-pr-closeout --help\n\
  adl-pr-closeout --version";

fn main() {
    pr_dispatch_support::run_pr_subcommand_main("closeout", USAGE, cli::pr_cmd::real_pr);
}
