extern crate adl;

#[allow(dead_code)]
#[path = "../cli/mod.rs"]
mod cli;
#[path = "../pr_dispatch_support.rs"]
mod pr_dispatch_support;

const USAGE: &str = "adl-issue - ADL direct issue operation binary\n\n\
Usage:\n\
  adl-issue list [-R owner/repo] [--state open|closed|all] [--limit <n>] [--json]\n\
  adl-issue search --query \"<text>\" [-R owner/repo] [--state open|closed|all] [--limit <n>] [--json]\n\
  adl-issue view <issue-number-or-url> [-R owner/repo] [--json]\n\
  adl-issue create --title \"<title>\" [--body \"<markdown>\" | --body-file <path>] [--label <label>]... [--labels <csv>] [-R owner/repo] [--json]\n\
  adl-issue comment <issue-number-or-url> [--body \"<markdown>\" | --body-file <path>] [-R owner/repo] [--json]\n\
  adl-issue edit <issue-number-or-url> [--title \"<title>\"] [--body \"<markdown>\" | --body-file <path>] [--label <label>]... [--labels <csv>] [-R owner/repo] [--json]\n\
  adl-issue --help\n\
  adl-issue --version";

fn run_with_dispatch(
    args: &[String],
    dispatch: fn(&[String]) -> anyhow::Result<()>,
) -> anyhow::Result<()> {
    pr_dispatch_support::run_pr_subcommand_args("issue", USAGE, args, dispatch)
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
    fn adl_issue_help_and_version_paths_succeed() {
        for args in [vec!["--help".to_string()], vec!["--version".to_string()]] {
            run(&args).expect("wrapper path should succeed");
        }
    }

    #[test]
    fn adl_issue_forwards_args_to_dispatch() {
        let args = vec!["list".to_string(), "--json".to_string()];
        run_with_dispatch(&args, record_dispatch).expect("dispatch should succeed");
        let calls = CALLS
            .get_or_init(|| Mutex::new(Vec::new()))
            .lock()
            .expect("dispatch calls lock");
        assert_eq!(
            calls.last().expect("recorded call"),
            &vec![
                "issue".to_string(),
                "list".to_string(),
                "--json".to_string()
            ]
        );
    }
}
