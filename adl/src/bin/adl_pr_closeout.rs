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

fn run(args: &[String]) -> anyhow::Result<()> {
    pr_dispatch_support::run_pr_subcommand_args("closeout", USAGE, args, cli::pr_cmd::real_pr)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Err(err) = run(&args) {
        eprintln!("Error: {err}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn closeout_binary_help_and_version_paths_succeed() {
        for args in [vec!["--help".to_string()], vec!["--version".to_string()]] {
            run(&args).expect("wrapper path should succeed");
        }
    }
}
