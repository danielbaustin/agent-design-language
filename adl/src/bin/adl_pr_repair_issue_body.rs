extern crate adl;

#[allow(dead_code)]
#[path = "../cli/mod.rs"]
mod cli;
#[path = "../pr_dispatch_support.rs"]
mod pr_dispatch_support;

const USAGE: &str = "adl-pr-repair-issue-body - ADL direct PR lifecycle binary\n\n\
Usage:\n\
  adl-pr-repair-issue-body <issue> [--slug <slug>] [--title \"<title>\"] [--body \"<markdown>\" | --body-file <path>] [--labels <csv>] [--version <v0.85|v0.87.1>] [--force]\n\
  adl-pr-repair-issue-body --help\n\
  adl-pr-repair-issue-body --version";

fn main() {
    pr_dispatch_support::run_pr_subcommand_main("repair-issue-body", USAGE, cli::pr_cmd::real_pr);
}
