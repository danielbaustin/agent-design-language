use std::process;
#[cfg(test)]
#[path = "../test_support.rs"]
mod test_support;

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
        "adl-lint-prompt-spec --issue <number>\n\
adl-lint-prompt-spec --input <path>\n\
\n\
Runs Prompt Spec lint directly without routing through the broad `adl tooling` dispatch surface.\n"
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

    pub(super) fn run_lint_prompt_spec(args: &[String]) -> anyhow::Result<()> {
        structured_prompt::real_lint_prompt_spec(args)
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
    cli_observability::emit_event("adl-lint-prompt-spec", "command", "started", &[]);
    match tooling_cmd::run_lint_prompt_spec(args) {
        Ok(()) => {
            cli_observability::emit_event("adl-lint-prompt-spec", "command", "completed", &[]);
            Ok(())
        }
        Err(err) => {
            let reason = err.to_string();
            cli_observability::emit_event(
                "adl-lint-prompt-spec",
                "command",
                "failed",
                &[("reason", reason.as_str())],
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
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_log_path(name: &str) -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time ok")
            .as_nanos();
        std::env::temp_dir().join(format!("adl-{name}-{unique}.md"))
    }

    fn valid_prompt_spec_card() -> String {
        r#"# Prompt Spec Fixture

## Prompt Spec
```yaml
prompt_schema: adl.v1
actor:
  role: execution_agent
  name: codex
model:
  id: gpt-5-codex
  determinism_mode: stable
inputs:
  sections:
    - goal
outputs:
  output_card: .adl/cards/1374/output_1374.md
  summary_style: concise_structured
constraints:
  include_system_invariants: true
  include_reviewer_checklist: true
  disallow_secrets: true
  disallow_absolute_host_paths: true
automation_hints:
  source_issue_prompt_required: true
  target_files_surfaces_recommended: true
  validation_plan_required: true
  required_outcome_type_supported: true
review_surfaces:
  - card_review_checklist.v1
  - card_review_output.v1
  - card_reviewer_gpt.v1.1
```
"#
        .to_string()
    }

    #[test]
    fn help_mentions_direct_lint_binary() {
        let output = super::binary_help_probe();
        assert!(output.contains("adl-lint-prompt-spec"));
        assert!(output.contains("without routing through the broad `adl tooling` dispatch surface"));
    }

    #[test]
    fn run_valid_input_succeeds() {
        let input = temp_log_path("lint-card");
        fs::write(&input, valid_prompt_spec_card()).expect("write card");
        run(&["--input".to_string(), input.display().to_string()]).expect("lint succeeds");
    }

    #[test]
    fn run_invalid_args_fail() {
        let err = run(&["--bogus".to_string()]).expect_err("invalid args fail");
        assert!(err.to_string().contains("unknown arg"));
    }
}
