use anyhow::{anyhow, Result};

#[path = "tooling_cmd/card_prompt.rs"]
mod card_prompt;
#[path = "tooling_cmd/ci_log_archive.rs"]
mod ci_log_archive;
#[path = "tooling_cmd/code_review.rs"]
mod code_review;
#[path = "tooling_cmd/codex_usage_watch.rs"]
mod codex_usage_watch;
#[path = "tooling_cmd/common.rs"]
mod common;
#[path = "tooling_cmd/csdlc_prompt_editor.rs"]
mod csdlc_prompt_editor;
#[path = "tooling_cmd/github_release.rs"]
mod github_release;
#[path = "tooling_cmd/issue_resource_telemetry.rs"]
mod issue_resource_telemetry;
#[path = "tooling_cmd/markdown.rs"]
mod markdown;
#[path = "tooling_cmd/markdown_ast_edit.rs"]
mod markdown_ast_edit;
#[path = "tooling_cmd/portable_project_doctor.rs"]
mod portable_project_doctor;
#[path = "tooling_cmd/prompt_template.rs"]
mod prompt_template;
#[path = "tooling_cmd/public_prompt_packet.rs"]
mod public_prompt_packet;
#[path = "tooling_cmd/review_contract.rs"]
mod review_contract;
#[path = "tooling_cmd/review_surface.rs"]
mod review_surface;
#[path = "tooling_cmd/structured_prompt.rs"]
mod structured_prompt;
#[path = "tooling_cmd/wp_issue_wave.rs"]
mod wp_issue_wave;

use card_prompt::real_card_prompt;
use ci_log_archive::real_ci_log_archive;
use code_review::real_code_review;
use codex_usage_watch::real_codex_usage_watch;
use csdlc_prompt_editor::real_csdlc_prompt_editor;
use github_release::real_github_release;
use issue_resource_telemetry::real_issue_resource_telemetry;
use markdown_ast_edit::real_markdown_ast_edit;
use portable_project_doctor::real_portable_project_doctor;
use prompt_template::real_prompt_template;
use public_prompt_packet::real_public_prompt_packet;
use review_contract::{real_verify_repo_review_contract, real_verify_review_output_provenance};
use review_surface::{real_review_card_surface, real_review_runtime_surface};
use structured_prompt::{real_lint_prompt_spec, real_validate_structured_prompt};
use wp_issue_wave::real_generate_wp_issue_wave;

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
            "tooling requires a subcommand: card-prompt | ci-log-archive | code-review | codex-usage-watch | csdlc-prompt-editor | generate-wp-issue-wave | github-release | issue-resource-telemetry | lint-prompt-spec | prompt-template | public-prompt-packet | validate-structured-prompt | review-card-surface | review-runtime-surface | verify-review-output-provenance | verify-repo-review-contract"
        ));
    };

    match subcommand {
        "card-prompt" => real_card_prompt(&args[1..]),
        "ci-log-archive" => real_ci_log_archive(&args[1..]),
        "code-review" => real_code_review(&args[1..]),
        "codex-usage-watch" => real_codex_usage_watch(&args[1..]),
        "csdlc-prompt-editor" => real_csdlc_prompt_editor(&args[1..]),
        "generate-wp-issue-wave" => real_generate_wp_issue_wave(&args[1..]),
        "github-release" => real_github_release(&args[1..]),
        "issue-resource-telemetry" => real_issue_resource_telemetry(&args[1..]),
        "lint-prompt-spec" => real_lint_prompt_spec(&args[1..]),
        "markdown-ast-edit" => real_markdown_ast_edit(&args[1..]),
        "portable-project-doctor" => real_portable_project_doctor(&args[1..]),
        "prompt-template" => real_prompt_template(&args[1..]),
        "public-prompt-packet" => real_public_prompt_packet(&args[1..]),
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
            "unknown tooling subcommand '{subcommand}' (expected card-prompt | ci-log-archive | code-review | codex-usage-watch | csdlc-prompt-editor | generate-wp-issue-wave | github-release | issue-resource-telemetry | lint-prompt-spec | portable-project-doctor | prompt-template | public-prompt-packet | validate-structured-prompt | review-card-surface | review-runtime-surface | verify-review-output-provenance | verify-repo-review-contract)"
        )),
    }
}

fn tooling_usage() -> &'static str {
    "adl tooling card-prompt --issue <number> [--out <path>]\n\
adl tooling card-prompt --input <path> [--out <path>]\n\
adl tooling ci-log-archive summarize --logs-dir <dir> --out <manifest.json> --s3-prefix s3://bucket/prefix [--repo owner/repo] [--pr <n>] [--run-id <id>] [--commit <sha>] [--raw-zip <logs.zip>] [--upload] [--threshold-seconds 60] [--redaction-status <status>]\n\
adl tooling code-review --out <dir> [--backend fixture|ollama] [--visibility packet-only|read-only-repo] [--base <ref>] [--head <ref>] [--issue <number>] [--writer-session <id>] [--reviewer-session <id>] [--model <name>] [--allow-live-ollama] [--ollama-url <url>] [--timeout-secs <n>] [--include-working-tree] [--file <path> ...] [--fixture-case clean|blocked]\n\
adl tooling codex-usage-watch parse --input <status.txt> [--json]\n\
adl tooling codex-usage-watch parse --text \"Context: ...\" [--json]\n\
adl tooling codex-usage-watch watch --input <status.txt> [--interval-seconds <n>] [--iterations <n>] [--history-root <dir>] [--json]\n\
adl tooling csdlc-prompt-editor [--repo-root <path>] [--emit-model-js <path>] [--render-samples <dir>]\n\
adl tooling generate-wp-issue-wave --version <version> [--wbs <path>] [--sprint <path>] [--out <path>]\n\
adl tooling github-release ensure-absent|ensure-present|draft|publish --repo <owner/repo> --tag <tag> [--name <name>] [--notes-file <path>] [--target <branch>]\n\
adl tooling issue-resource-telemetry collect --issue <number> --issue-slug <slug> --capture-stage <issue_start|pre_validation|post_validation|review_handoff|custom_stage> [--host-label wuji] [--process <role:pid>] [--pid-file-process <role:path>] [--captured-at <rfc3339>] [--repo-root <path>] [--out <path>] [--json]\n\
adl tooling lint-prompt-spec --issue <number>\n\
adl tooling lint-prompt-spec --input <path>\n\
adl tooling markdown-ast-edit replace-section --input <path> --heading <heading> --replacement <path> --out <path> [--repair-note-out <path>]\n\
adl tooling portable-project-doctor --project <adl_project.json> [--adl-home <path>] [--json]\n\
adl tooling prompt-template render --kind <sip|stp|spp|vpp|srp|sor> --values <values.yaml> --out <card.md> [--repo-root <path>]\n\
adl tooling prompt-template render-all --values-dir <dir> --out-dir <dir> [--repo-root <path>] [--template-set <semver>]\n\
adl tooling prompt-template edit-values --kind <sip|stp|spp|vpp|srp|sor> --values <values.yaml> --set <field=value> [--set <field=value> ...] [--out <values.yaml>] [--repo-root <path>]\n\
adl tooling prompt-template edit-rendered --kind <sip|stp|spp|vpp|srp|sor> --input <card.md> --set <field=value> [--set <field=value> ...] --out <card.md> [--values-out <values.yaml>] [--repo-root <path>] [--template-set <semver>]\n\
adl tooling prompt-template import-values --kind <sip|stp|spp|vpp|srp|sor> --input <card.md> --out <values.yaml> [--normalized-out <card.md>] [--repo-root <path>] [--template-set <semver>]\n\
adl tooling prompt-template validate-values --kind <sip|stp|spp|vpp|srp|sor> --values <values.yaml> [--repo-root <path>]\n\
adl tooling prompt-template validate-structure --kind <sip|stp|spp|vpp|srp|sor> --input <card.md> [--repo-root <path>] [--template-set <semver>]\n\
adl tooling prompt-template validate-schemas [--repo-root <path>] [--template-set <semver>]\n\
adl tooling prompt-template write-sample-values --out-dir <dir> [--template-set <semver>]\n\
adl tooling prompt-template write-structure-schemas --out-dir <dir> [--repo-root <path>] [--template-set <semver>]\n\
adl tooling public-prompt-packet export --issue <number> --slug <slug> --version <version> [--source <dir>] [--out-root <dir>] [--tracker-url <url>] [--repo-root <path>]\n\
adl tooling validate-structured-prompt --type <sip|stp|spp|vpp|srp|sor> --input <path> [--phase <phase>]\n\
adl tooling review-card-surface --input <input.md> --output <output.md>\n\
adl tooling review-runtime-surface --review-root <dir>\n\
adl tooling verify-review-output-provenance --review <yaml>\n\
adl tooling verify-repo-review-contract --review <markdown>"
}

#[cfg(test)]
mod tests;
