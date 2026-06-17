extern crate adl;

#[allow(dead_code)]
#[path = "../cli/mod.rs"]
mod cli;
#[path = "../pr_dispatch_support.rs"]
mod pr_dispatch_support;

const USAGE: &str = "adl-pr-create - ADL direct PR lifecycle binary\n\n\
Usage:\n\
  adl-pr-create --title \"<title>\" [--slug <slug>] [--body \"<markdown>\" | --body-file <path>] [--labels <csv>] [--version <v0.85|v0.87.1>]\n\
  adl-pr-create --help\n\
  adl-pr-create --version";

fn main() {
    pr_dispatch_support::run_pr_subcommand_main("create", USAGE, cli::pr_cmd::real_pr);
}
