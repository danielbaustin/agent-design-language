use anyhow::{anyhow, Result};

#[path = "tooling_cmd/card_prompt.rs"]
mod card_prompt;
#[path = "tooling_cmd/common.rs"]
mod common;
#[path = "tooling_cmd/markdown.rs"]
mod markdown;
#[path = "tooling_cmd/review_contract.rs"]
mod review_contract;
#[path = "tooling_cmd/review_surface.rs"]
mod review_surface;
#[path = "tooling_cmd/structured_prompt.rs"]
mod structured_prompt;

use card_prompt::real_card_prompt;
use review_contract::{real_verify_repo_review_contract, real_verify_review_output_provenance};
use review_surface::{real_review_card_surface, real_review_runtime_surface};
use structured_prompt::{real_lint_prompt_spec, real_validate_structured_prompt};

#[cfg(test)]
use common::{
    absolutize, contains_absolute_host_path_in_text, contains_secret_like_token, ensure_file,
    ensure_no_absolute_host_path, ensure_no_disallowed_content, ensure_sorted_pointers,
    is_normalized_slug, is_repo_relative, normalize_issue, pointer_sort_key, repo_relative_display,
    valid_branch, valid_github_issue_url, valid_github_pr_url, valid_iso8601_datetime,
    valid_reference, valid_task_id, valid_version,
};
#[cfg(test)]
use markdown::{
    display_card_ref, markdown_block_field, markdown_field, markdown_has_heading,
    markdown_section_body, split_front_matter,
};
#[cfg(test)]
use review_surface::{decision_for, ReviewCheck};
#[cfg(test)]
use structured_prompt::{
    extract_prompt_spec_yaml, prompt_spec_bool, prompt_spec_sections, validate_prompt_spec,
    validate_sip_text, validate_sor_text, validate_stp_text,
};

pub(crate) fn real_tooling(args: &[String]) -> Result<()> {
    let Some(subcommand) = args.first().map(|arg| arg.as_str()) else {
        return Err(anyhow!(
            "tooling requires a subcommand: card-prompt | lint-prompt-spec | validate-structured-prompt | review-card-surface | review-runtime-surface | verify-review-output-provenance | verify-repo-review-contract"
        ));
    };

    match subcommand {
        "card-prompt" => real_card_prompt(&args[1..]),
        "lint-prompt-spec" => real_lint_prompt_spec(&args[1..]),
        "validate-structured-prompt" => real_validate_structured_prompt(&args[1..]),
        "review-card-surface" => real_review_card_surface(&args[1..]),
        "review-runtime-surface" => real_review_runtime_surface(&args[1..]),
        "verify-review-output-provenance" => real_verify_review_output_provenance(&args[1..]),
        "verify-repo-review-contract" => real_verify_repo_review_contract(&args[1..]),
        "--help" | "-h" | "help" => {
            println!("{}", tooling_usage());
            Ok(())
        }
        _ => Err(anyhow!(
            "unknown tooling subcommand '{subcommand}' (expected card-prompt | lint-prompt-spec | validate-structured-prompt | review-card-surface | review-runtime-surface | verify-review-output-provenance | verify-repo-review-contract)"
        )),
    }
}

fn tooling_usage() -> &'static str {
    "adl tooling card-prompt --issue <number> [--out <path>]\n\
adl tooling card-prompt --input <path> [--out <path>]\n\
adl tooling lint-prompt-spec --issue <number>\n\
adl tooling lint-prompt-spec --input <path>\n\
adl tooling validate-structured-prompt --type <stp|sip|sor> --input <path> [--phase <phase>]\n\
adl tooling review-card-surface --input <input.md> --output <output.md>\n\
adl tooling review-runtime-surface --review-root <dir>\n\
adl tooling verify-review-output-provenance --review <yaml>\n\
adl tooling verify-repo-review-contract --review <markdown>"
}

#[cfg(test)]
mod tests;
