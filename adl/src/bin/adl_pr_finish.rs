extern crate adl;

#[allow(dead_code)]
#[path = "../cli/mod.rs"]
mod cli;
#[path = "../pr_dispatch_support.rs"]
mod pr_dispatch_support;

const USAGE: &str = "adl-pr-finish - ADL direct PR lifecycle binary\n\n\
Usage:\n\
  adl-pr-finish <issue> --title \"<title>\" [-f <input_card.md>] [--output-card <output_card.md>] [--body \"<extra body>\"] [--paths \"<p1,p2,...>\"] [--no-checks] [--no-close] [--ready] [--allow-gitignore] [--no-open]\n\
  adl-pr-finish --help\n\
  adl-pr-finish --version";

fn main() {
    pr_dispatch_support::run_pr_subcommand_main("finish", USAGE, cli::pr_cmd::real_pr);
}
