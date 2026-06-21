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
        "adl-prompt-template render --kind <sip|stp|spp|vpp|srp|sor> --values <values.yaml> --out <card.md> [--repo-root <path>]\n\
adl-prompt-template render-all --values-dir <dir> --out-dir <dir> [--repo-root <path>] [--template-set <semver>]\n\
adl-prompt-template edit-values --kind <sip|stp|spp|vpp|srp|sor> --values <values.yaml> --set <field=value> [--set <field=value> ...] [--out <values.yaml>] [--repo-root <path>]\n\
adl-prompt-template edit-rendered --kind <sip|stp|spp|vpp|srp|sor> --input <card.md> --set <field=value> [--set <field=value> ...] --out <card.md> [--values-out <values.yaml>] [--repo-root <path>] [--template-set <semver>]\n\
adl-prompt-template import-values --kind <sip|stp|spp|vpp|srp|sor> --input <card.md> --out <values.yaml> [--normalized-out <card.md>] [--repo-root <path>] [--template-set <semver>]\n\
adl-prompt-template validate-values --kind <sip|stp|spp|vpp|srp|sor> --values <values.yaml> [--repo-root <path>]\n\
adl-prompt-template validate-structure --kind <sip|stp|spp|vpp|srp|sor> --input <card.md> [--repo-root <path>] [--template-set <semver>]\n\
adl-prompt-template validate-schemas [--repo-root <path>] [--template-set <semver>]\n\
adl-prompt-template write-sample-values --out-dir <dir> [--template-set <semver>]\n\
adl-prompt-template write-structure-schemas --out-dir <dir> [--repo-root <path>] [--template-set <semver>]\n\
\n\
Runs prompt-template editor and renderer operations directly without routing through the broad `adl tooling` dispatch surface.\n"
    }

    #[allow(dead_code)]
    pub mod prompt_template {
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/cli/tooling_cmd/prompt_template.rs"
        ));
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
    let subcommand = args.first().map(String::as_str).unwrap_or("");
    cli_observability::emit_event(
        "adl-prompt-template",
        "command",
        "started",
        &[("subcommand", subcommand)],
    );
    match tooling_cmd::prompt_template::real_prompt_template(args) {
        Ok(()) => {
            cli_observability::emit_event(
                "adl-prompt-template",
                "command",
                "completed",
                &[("subcommand", subcommand)],
            );
            Ok(())
        }
        Err(err) => {
            let reason = err.to_string();
            cli_observability::emit_event(
                "adl-prompt-template",
                "command",
                "failed",
                &[("subcommand", subcommand), ("reason", reason.as_str())],
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
    use std::path::Path;

    fn repo_root() -> &'static Path {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("repo root")
    }

    #[test]
    fn prompt_template_binary_help_mentions_direct_prompt_template_binary() {
        let output = super::binary_help_probe();
        assert!(output.contains("adl-prompt-template render"));
        assert!(output.contains("without routing through the broad `adl tooling` dispatch surface"));
    }

    #[test]
    fn prompt_template_binary_run_validate_schemas_succeeds() {
        run(&[
            "validate-schemas".to_string(),
            "--repo-root".to_string(),
            repo_root().display().to_string(),
        ])
        .expect("validate-schemas succeeds");
    }

    #[test]
    fn prompt_template_binary_run_invalid_args_fail() {
        let err = run(&["bogus".to_string()]).expect_err("invalid args fail");
        assert!(err
            .to_string()
            .contains("unknown prompt-template subcommand"));
    }

    #[test]
    fn prompt_template_binary_print_error_chain_handles_nested_error() {
        let err = Err::<(), _>(anyhow::anyhow!("root cause"))
            .context("outer")
            .expect_err("error");
        super::print_error_chain(&err);
    }
}
