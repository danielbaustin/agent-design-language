use std::process;

#[allow(dead_code)]
mod cli_observability {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/cli/observability.rs"
    ));
}

mod tooling_cmd {
    #[allow(dead_code)]
    pub(super) fn tooling_usage() -> &'static str {
        "adl-validate-structured-prompt --type <sip|stp|spp|vpp|srp|sor> --input <path> [--phase <phase>]\n\
\n\
Runs the structured-prompt validator directly without routing through the broad `adl tooling` dispatch surface.\n"
    }

    #[allow(dead_code)]
    pub mod common {
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/cli/tooling_cmd/common.rs"
        ));
    }
    #[allow(dead_code)]
    pub mod markdown {
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/cli/tooling_cmd/markdown.rs"
        ));
    }
    #[allow(dead_code)]
    pub mod structured_prompt {
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/cli/tooling_cmd/structured_prompt.rs"
        ));
    }

    pub(super) fn run_validate_structured_prompt(args: &[String]) -> anyhow::Result<()> {
        structured_prompt::real_validate_structured_prompt(args)
    }
}

fn print_error_chain(err: &anyhow::Error) {
    eprintln!("Error: {err}");
    let mut n = 0;
    let mut cur = err.source();
    while let Some(cause) = cur {
        eprintln!("  {n}: {cause}");
        n += 1;
        cur = cause.source();
    }
}

fn run(args: &[String]) -> anyhow::Result<()> {
    let prompt_type = args
        .windows(2)
        .find(|pair| pair[0] == "--type")
        .map(|pair| pair[1].as_str())
        .unwrap_or("");
    cli_observability::emit_event(
        "adl-validate-structured-prompt",
        "command",
        "started",
        &[("type", prompt_type)],
    );
    match tooling_cmd::run_validate_structured_prompt(args) {
        Ok(()) => {
            cli_observability::emit_event(
                "adl-validate-structured-prompt",
                "command",
                "completed",
                &[("type", prompt_type)],
            );
            Ok(())
        }
        Err(err) => {
            let reason = err.to_string();
            cli_observability::emit_event(
                "adl-validate-structured-prompt",
                "command",
                "failed",
                &[("type", prompt_type), ("reason", reason.as_str())],
            );
            Err(err)
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Err(err) = run(&args) {
        print_error_chain(&err);
        process::exit(1);
    }
}

#[cfg(test)]
fn binary_help_probe() -> String {
    tooling_cmd::tooling_usage().to_string()
}

#[cfg(test)]
mod tests {
    use super::run;
    use anyhow::Context;

    #[test]
    fn help_mentions_direct_validator_binary() {
        let output = super::binary_help_probe();
        assert!(output.contains("adl-validate-structured-prompt"));
        assert!(output.contains("without routing through the broad `adl tooling` dispatch surface"));
    }

    #[test]
    fn run_help_with_type_succeeds() {
        run(&[
            "--type".to_string(),
            "spp".to_string(),
            "--help".to_string(),
        ])
        .expect("help succeeds");
    }

    #[test]
    fn run_invalid_args_fail() {
        let err = run(&["--bogus".to_string()]).expect_err("invalid args fail");
        assert!(err.to_string().contains("unknown arg"));
    }

    #[test]
    fn print_error_chain_handles_nested_error() {
        let err = Err::<(), _>(anyhow::anyhow!("root cause"))
            .context("outer")
            .expect_err("error");
        super::print_error_chain(&err);
    }
}
