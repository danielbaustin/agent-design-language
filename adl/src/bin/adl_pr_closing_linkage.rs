extern crate adl;

#[allow(dead_code)]
#[path = "../cli/mod.rs"]
mod cli;
#[path = "../pr_dispatch_support.rs"]
mod pr_dispatch_support;

const USAGE: &str = "adl-pr-closing-linkage - ADL direct PR closing-linkage guard binary\n\n\
Usage:\n\
  adl-pr-closing-linkage [--event-name <event>] [--event-path <path>] [--head-ref <branch>] [-R owner/repo]\n\
  adl-pr-closing-linkage --help\n\
  adl-pr-closing-linkage --version";

fn run_with_dispatch(
    args: &[String],
    dispatch: fn(&[String]) -> anyhow::Result<()>,
) -> anyhow::Result<()> {
    pr_dispatch_support::run_pr_subcommand_args("closing-linkage", USAGE, args, dispatch)
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
    fn adl_pr_closing_linkage_help_and_version_paths_succeed() {
        for args in [vec!["--help".to_string()], vec!["--version".to_string()]] {
            run(&args).expect("wrapper path should succeed");
        }
    }

    #[test]
    fn adl_pr_closing_linkage_forwards_args_to_dispatch() {
        let args = vec![
            "--event-name".to_string(),
            "pull_request".to_string(),
            "--head-ref".to_string(),
            "codex/4286-demo".to_string(),
        ];
        run_with_dispatch(&args, record_dispatch).expect("dispatch should succeed");
        let calls = CALLS
            .get_or_init(|| Mutex::new(Vec::new()))
            .lock()
            .expect("dispatch calls lock");
        assert_eq!(
            calls.last().expect("recorded call"),
            &vec![
                "closing-linkage".to_string(),
                "--event-name".to_string(),
                "pull_request".to_string(),
                "--head-ref".to_string(),
                "codex/4286-demo".to_string(),
            ]
        );
    }
}
