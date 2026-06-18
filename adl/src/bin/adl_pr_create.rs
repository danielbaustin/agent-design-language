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

fn run_with_dispatch(
    args: &[String],
    dispatch: fn(&[String]) -> anyhow::Result<()>,
) -> anyhow::Result<()> {
    pr_dispatch_support::run_pr_subcommand_args("create", USAGE, args, dispatch)
}

fn run(args: &[String]) -> anyhow::Result<()> {
    run_with_dispatch(args, cli::pr_cmd::real_pr)
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
    use super::{run, run_with_dispatch};
    use std::sync::{Mutex, OnceLock};

    static CALLS: OnceLock<Mutex<Vec<Vec<String>>>> = OnceLock::new();

    fn record_dispatch(args: &[String]) -> anyhow::Result<()> {
        CALLS
            .get_or_init(|| Mutex::new(Vec::new()))
            .lock()
            .expect("dispatch calls lock")
            .push(args.to_vec());
        Ok(())
    }

    #[test]
    fn adl_pr_create_help_and_version_paths_succeed() {
        for args in [vec!["--help".to_string()], vec!["--version".to_string()]] {
            run(&args).expect("wrapper path should succeed");
        }
    }

    #[test]
    fn adl_pr_create_forwards_args_to_dispatch() {
        let args = vec!["--title".to_string(), "demo".to_string()];
        run_with_dispatch(&args, record_dispatch).expect("dispatch should succeed");
        let calls = CALLS
            .get_or_init(|| Mutex::new(Vec::new()))
            .lock()
            .expect("dispatch calls lock");
        assert_eq!(
            calls.last().expect("recorded call"),
            &vec![
                "create".to_string(),
                "--title".to_string(),
                "demo".to_string()
            ]
        );
    }
}
