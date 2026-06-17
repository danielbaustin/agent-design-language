extern crate adl;

#[allow(dead_code)]
#[path = "../cli/mod.rs"]
mod cli;
#[path = "../pr_dispatch_support.rs"]
mod pr_dispatch_support;

const USAGE: &str = "adl-pr-validation - ADL direct PR lifecycle binary\n\n\
Usage:\n\
  adl-pr-validation <pr-number-or-url> [-R owner/repo] [--watch] [--json]\n\
  adl-pr-validation --help\n\
  adl-pr-validation --version";

fn main() {
    pr_dispatch_support::run_pr_subcommand_main("validation", USAGE, cli::pr_cmd::real_pr);
}
