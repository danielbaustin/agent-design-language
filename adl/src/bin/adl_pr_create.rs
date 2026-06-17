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

fn run(args: &[String]) -> anyhow::Result<()> {
    pr_dispatch_support::run_pr_subcommand_args("create", USAGE, args, cli::pr_cmd::real_pr)
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
    fn create_binary_help_and_version_paths_succeed() {
        for args in [vec!["--help".to_string()], vec!["--version".to_string()]] {
            run(&args).expect("wrapper path should succeed");
        }
    }
}
