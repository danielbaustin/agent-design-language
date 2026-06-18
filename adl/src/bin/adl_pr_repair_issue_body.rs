extern crate adl;

#[allow(dead_code)]
#[path = "../cli/mod.rs"]
mod cli;
#[path = "../pr_dispatch_support.rs"]
mod pr_dispatch_support;
#[cfg(test)]
#[path = "../test_support.rs"]
mod test_support;

const USAGE: &str = "adl-pr-repair-issue-body - ADL direct PR lifecycle binary\n\n\
Usage:\n\
  adl-pr-repair-issue-body <issue> [--slug <slug>] [--title \"<title>\"] [--body \"<markdown>\" | --body-file <path>] [--labels <csv>] [--version <v0.85|v0.87.1>] [--force]\n\
  adl-pr-repair-issue-body --help\n\
  adl-pr-repair-issue-body --version";

fn run_with_dispatch(
    args: &[String],
    dispatch: fn(&[String]) -> anyhow::Result<()>,
) -> anyhow::Result<()> {
    pr_dispatch_support::run_pr_subcommand_args("repair-issue-body", USAGE, args, dispatch)
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
    fn adl_pr_repair_issue_body_help_and_version_paths_succeed() {
        for args in [vec!["--help".to_string()], vec!["--version".to_string()]] {
            run(&args).expect("wrapper path should succeed");
        }
    }

    #[test]
    fn adl_pr_repair_issue_body_forwards_args_to_dispatch() {
        let args = vec!["123".to_string(), "--force".to_string()];
        run_with_dispatch(&args, record_dispatch).expect("dispatch should succeed");
        let calls = CALLS
            .get_or_init(|| Mutex::new(Vec::new()))
            .lock()
            .expect("dispatch calls lock");
        assert_eq!(
            calls.last().expect("recorded call"),
            &vec![
                "repair-issue-body".to_string(),
                "123".to_string(),
                "--force".to_string()
            ]
        );
    }
}
