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

fn run(args: &[String]) -> anyhow::Result<()> {
    pr_dispatch_support::run_pr_subcommand_args("start", USAGE, args, cli::pr_cmd::real_pr)
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
    fn run_binary_help_and_version_paths_succeed() {
        for args in [vec!["--help".to_string()], vec!["--version".to_string()]] {
            run(&args).expect("wrapper path should succeed");
        }
    }
}
