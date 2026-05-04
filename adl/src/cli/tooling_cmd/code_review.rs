mod code_review_args;
mod code_review_build;
mod code_review_helpers;
mod code_review_reviewer;
mod code_review_types;

#[cfg(test)]
#[path = "tests/code_review.rs"]
mod tests;

use anyhow::Context;
use anyhow::Result;
use sha2::{Digest, Sha256};
use std::fs;

use super::common::repo_root;
use super::tooling_usage;

use code_review_build::build_packet;
use code_review_helpers::write_json;
use code_review_reviewer::{evaluate_gate, run_reviewer};
use code_review_types::{RunSummary, CODE_REVIEW_SUMMARY_SCHEMA};

#[cfg(test)]
pub(crate) use code_review_args::parse_args;
#[cfg(test)]
pub(crate) use code_review_build::{changed_files, contains_review_absolute_host_path};
#[cfg(test)]
pub(crate) use code_review_helpers::{read_file_prefix, truncate};
#[cfg(test)]
pub(crate) use code_review_reviewer::{
    fixture_review, normalize_model_review, ollama_generate_url, parse_model_review_json,
    redact_absolute_host_paths_for_prompt, review_result, reviewer_prompt, ParsedModelReview,
    ReviewResultPartsCompat,
};
#[cfg(test)]
pub(crate) use code_review_types::{
    CodeReviewArgs, DiffHunk, DiffSummary, FileContext, FixtureCase, RedactionStatus,
    RepoSliceManifest, ReviewDisposition, ReviewFinding, ReviewPacket, ReviewerBackend,
    ValidationEvidence, VisibilityMode, DEFAULT_REVIEW_EXCERPT_BYTES, MAX_REVIEW_CONTEXT_FILES,
    MAX_REVIEW_DIFF_FILES, MAX_REVIEW_EXCERPT_BYTES, PACKET_SCHEMA,
};

pub(super) fn real_code_review(args: &[String]) -> Result<()> {
    if args
        .iter()
        .any(|arg| matches!(arg.as_str(), "help" | "--help" | "-h"))
    {
        println!("{}", tooling_usage());
        return Ok(());
    }

    let args = code_review_args::parse_args(args)?;
    fs::create_dir_all(&args.out).context("create code-review output directory")?;

    let root = repo_root()?;
    run_code_review_for_root(&root, &args)
}

fn run_code_review_for_root(
    root: &std::path::Path,
    args: &code_review_types::CodeReviewArgs,
) -> Result<()> {
    let packet = build_packet(&root, &args)?;
    let packet_id = packet_id(&packet);
    let result = run_reviewer(&args, &packet, &packet_id)?;
    let gate = evaluate_gate(&result, &packet);

    let packet_path = args.out.join("review_packet.json");
    let result_path = args.out.join("review_result.json");
    let gate_path = args.out.join("gate_result.json");
    let summary_path = args.out.join("run_summary.json");

    write_json(&packet_path, &packet)?;
    write_json(&result_path, &result)?;
    write_json(&gate_path, &gate)?;
    write_json(
        &summary_path,
        &RunSummary {
            schema_version: CODE_REVIEW_SUMMARY_SCHEMA,
            packet_path: "review_packet.json".to_string(),
            result_path: "review_result.json".to_string(),
            gate_path: "gate_result.json".to_string(),
            backend: args.backend.as_str().to_string(),
            visibility_mode: args.visibility_mode,
            pr_open_allowed: gate.pr_open_allowed,
        },
    )?;

    println!(
        "CODE_REVIEW_GATE={} OUT={}",
        gate.gate_disposition,
        args.out.display()
    );
    Ok(())
}

#[cfg(test)]
pub(crate) fn real_code_review_for_root(root: &std::path::Path, args: &[String]) -> Result<()> {
    if args
        .iter()
        .any(|arg| matches!(arg.as_str(), "help" | "--help" | "-h"))
    {
        println!("{}", tooling_usage());
        return Ok(());
    }

    let args = code_review_args::parse_args(args)?;
    fs::create_dir_all(&args.out).context("create code-review output directory")?;
    run_code_review_for_root(root, &args)
}

fn packet_id(packet: &code_review_types::ReviewPacket) -> String {
    let seed = format!(
        "{}:{}:{}:{}",
        packet.issue_number.unwrap_or_default(),
        packet.branch,
        packet.changed_files.len(),
        packet.diff_summary.truncated_hunks
    );
    let digest = Sha256::digest(seed.as_bytes());
    format!("{:x}", digest)[..16].to_string()
}
