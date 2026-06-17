use anyhow::Result;

pub(crate) fn run_pr_subcommand_args(
    subcommand: &'static str,
    usage: &'static str,
    args: &[String],
    dispatch: fn(&[String]) -> Result<()>,
) -> Result<()> {
    if matches!(
        args.first().map(|s| s.as_str()),
        Some("--help" | "-h" | "help")
    ) {
        println!("{usage}");
        return Ok(());
    }
    if matches!(args.first().map(|s| s.as_str()), Some("--version" | "-V")) {
        println!("{}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }
    let mut pr_args = Vec::with_capacity(args.len() + 1);
    pr_args.push(subcommand.to_string());
    pr_args.extend(args.iter().cloned());
    dispatch(&pr_args)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Mutex, OnceLock};

    static CALLS: OnceLock<Mutex<Vec<Vec<String>>>> = OnceLock::new();

    fn record_dispatch(args: &[String]) -> Result<()> {
        CALLS
            .get_or_init(|| Mutex::new(Vec::new()))
            .lock()
            .expect("dispatch calls lock")
            .push(args.to_vec());
        Ok(())
    }

    #[test]
    fn pr_dispatch_support_forwards_subcommand_and_args() {
        let args = vec!["1234".to_string(), "--json".to_string()];
        run_pr_subcommand_args("doctor", "usage", &args, record_dispatch)
            .expect("dispatch should succeed");
        let calls = CALLS
            .get_or_init(|| Mutex::new(Vec::new()))
            .lock()
            .expect("dispatch calls lock");
        assert_eq!(
            calls.last().expect("recorded call"),
            &vec![
                "doctor".to_string(),
                "1234".to_string(),
                "--json".to_string()
            ]
        );
    }

    #[test]
    fn pr_dispatch_support_accepts_help_aliases() {
        for flag in ["--help", "-h", "help"] {
            run_pr_subcommand_args("doctor", "usage text", &[flag.to_string()], record_dispatch)
                .expect("help should succeed");
        }
    }

    #[test]
    fn pr_dispatch_support_accepts_version_aliases() {
        for flag in ["--version", "-V"] {
            run_pr_subcommand_args("doctor", "usage text", &[flag.to_string()], record_dispatch)
                .expect("version should succeed");
        }
    }
}
