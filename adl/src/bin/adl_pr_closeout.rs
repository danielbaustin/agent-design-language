extern crate adl;

#[allow(dead_code)]
#[path = "../cli/mod.rs"]
mod cli;
#[path = "../pr_dispatch_support.rs"]
mod pr_dispatch_support;
#[cfg(test)]
#[path = "../test_support.rs"]
mod test_support;

const USAGE: &str = "adl-pr-closeout - ADL direct PR lifecycle binary\n\n\
Usage:\n\
  adl-pr-closeout <issue> [--slug <slug>] [--version <v>] [--no-fetch-issue]\n\
  adl-pr-closeout --help\n\
  adl-pr-closeout --version";

fn run_with_dispatch(
    args: &[String],
    dispatch: fn(&[String]) -> anyhow::Result<()>,
) -> anyhow::Result<()> {
    pr_dispatch_support::run_pr_subcommand_args("closeout", USAGE, args, dispatch)
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
    fn adl_pr_closeout_help_and_version_paths_succeed() {
        for args in [vec!["--help".to_string()], vec!["--version".to_string()]] {
            run(&args).expect("wrapper path should succeed");
        }
    }

    #[test]
    fn adl_pr_closeout_forwards_args_to_dispatch() {
        let args = vec!["123".to_string(), "--slug".to_string(), "demo".to_string()];
        run_with_dispatch(&args, record_dispatch).expect("dispatch should succeed");
        let calls = CALLS
            .get_or_init(|| Mutex::new(Vec::new()))
            .lock()
            .expect("dispatch calls lock");
        assert_eq!(
            calls.last().expect("recorded call"),
            &vec![
                "closeout".to_string(),
                "123".to_string(),
                "--slug".to_string(),
                "demo".to_string()
            ]
        );
    }
}
