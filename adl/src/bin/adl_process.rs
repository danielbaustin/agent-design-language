extern crate adl;

#[path = "../cli/process_cmd.rs"]
mod process_cmd;

fn run(args: &[String]) -> anyhow::Result<()> {
    process_cmd::real_process(args)
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
    fn adl_process_help_path_succeeds() {
        run(&["--help".to_string()]).expect("help should succeed");
    }

    #[test]
    fn adl_process_rejects_unknown_subcommand() {
        let err = run(&["nope".to_string()]).expect_err("unknown subcommand should fail");
        assert!(err.to_string().contains("unknown process command 'nope'"));
    }
}
