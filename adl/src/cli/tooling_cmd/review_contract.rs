use anyhow::{anyhow, bail, ensure, Result};
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

use super::common::{
    contains_absolute_host_path_in_text, ensure_file, ensure_sorted_pointers, is_repo_relative,
    is_repo_review_finding_title, repo_review_finding_sort_key, repo_root, DECISIONS,
    EVIDENCE_STATES, REQUIRED_REPO_REVIEW_HEADINGS, VALIDATION_RESULTS,
};
use super::markdown::{markdown_headings, markdown_section_body};
use super::tooling_usage;

#[derive(Debug, Deserialize)]
struct ReviewOutputArtifact {
    review_format_version: String,
    decision: String,
    review_target: ReviewOutputTarget,
    findings: Vec<ReviewFinding>,
    validation_checks: ValidationChecks,
    security_privacy_checks: SecurityPrivacyChecks,
}

#[derive(Debug, Deserialize)]
struct ReviewOutputTarget {
    input_card_path: String,
    output_card_path: String,
}

#[derive(Debug, Deserialize)]
struct ReviewFinding {
    evidence_state: String,
    evidence: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct ValidationChecks {
    validation_result: String,
    commands: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct SecurityPrivacyChecks {
    absolute_host_paths_present: bool,
}

pub(super) fn real_verify_review_output_provenance(args: &[String]) -> Result<()> {
    let review = parse_review_arg(args, "tooling verify-review-output-provenance")?;
    let root = repo_root()?;
    ensure_file(&review, "review artifact")?;
    let artifact: ReviewOutputArtifact = serde_yaml::from_str(&fs::read_to_string(&review)?)?;
    ensure!(
        artifact.review_format_version == "card_review_output.v1",
        "unexpected review_format_version"
    );
    ensure!(
        DECISIONS.contains(&artifact.decision.as_str()),
        "invalid decision enum"
    );
    ensure!(
        is_repo_relative(&artifact.review_target.input_card_path),
        "input_card_path must be repo-relative"
    );
    ensure!(
        is_repo_relative(&artifact.review_target.output_card_path),
        "output_card_path must be repo-relative"
    );
    ensure!(
        root.join(&artifact.review_target.input_card_path).is_file(),
        "input card path does not exist: {}",
        artifact.review_target.input_card_path
    );
    ensure!(
        root.join(&artifact.review_target.output_card_path)
            .is_file(),
        "output card path does not exist: {}",
        artifact.review_target.output_card_path
    );
    for finding in &artifact.findings {
        ensure!(
            EVIDENCE_STATES.contains(&finding.evidence_state.as_str()),
            "invalid evidence_state"
        );
        ensure!(
            !finding.evidence.is_empty(),
            "finding evidence must be a non-empty array"
        );
        ensure_sorted_pointers(&finding.evidence, "finding evidence")?;
        for pointer in &finding.evidence {
            if let Some(path_value) = pointer
                .strip_prefix("path:")
                .or_else(|| pointer.strip_prefix("artifact:"))
            {
                ensure!(
                    is_repo_relative(path_value),
                    "evidence pointer path must be repo-relative: {}",
                    pointer
                );
            }
        }
    }
    ensure!(
        VALIDATION_RESULTS.contains(&artifact.validation_checks.validation_result.as_str()),
        "invalid validation_result enum"
    );
    for command in &artifact.validation_checks.commands {
        ensure!(
            !contains_absolute_host_path_in_text(command),
            "validation command leaks absolute host path"
        );
    }
    if !artifact.security_privacy_checks.absolute_host_paths_present {
        let serialized = fs::read_to_string(&review)?;
        ensure!(
            !contains_absolute_host_path_in_text(&serialized),
            "review artifact contradicts absolute_host_paths_present=false"
        );
    }
    println!(
        "PASS: review output provenance valid for {}",
        review.display()
    );
    Ok(())
}

pub(super) fn real_verify_repo_review_contract(args: &[String]) -> Result<()> {
    let review = parse_review_arg(args, "tooling verify-repo-review-contract")?;
    ensure_file(&review, "review artifact")?;
    let text = fs::read_to_string(&review)?;
    ensure!(
        !contains_absolute_host_path_in_text(&text),
        "review artifact contains absolute host path"
    );
    let headings = markdown_headings(&text);
    ensure!(
        headings == REQUIRED_REPO_REVIEW_HEADINGS,
        "review headings must match canonical order exactly"
    );
    let metadata = markdown_section_body(&text, "Metadata").unwrap_or_default();
    let scope = markdown_section_body(&text, "Scope").unwrap_or_default();
    let findings = markdown_section_body(&text, "Findings").unwrap_or_default();
    let assessment = markdown_section_body(&text, "System-Level Assessment").unwrap_or_default();
    let action_plan = markdown_section_body(&text, "Recommended Action Plan").unwrap_or_default();
    let follow_ups = markdown_section_body(&text, "Follow-ups / Deferred Work").unwrap_or_default();
    let final_assessment = markdown_section_body(&text, "Final Assessment").unwrap_or_default();

    ensure!(
        metadata.contains("Review Type:"),
        "Metadata must include Review Type"
    );
    ensure!(
        metadata.contains("Subject:"),
        "Metadata must include Subject"
    );
    ensure!(
        metadata.contains("Reviewer:"),
        "Metadata must include Reviewer"
    );
    ensure!(scope.contains("Reviewed:"), "Scope must include Reviewed");
    ensure!(
        scope.contains("Not Reviewed:"),
        "Scope must include Not Reviewed"
    );
    ensure!(
        scope.contains("Review Mode:"),
        "Scope must include Review Mode"
    );
    ensure!(scope.contains("Gate:"), "Scope must include Gate");

    let findings_lines = findings.lines().map(str::trim_end).collect::<Vec<_>>();
    let has_explicit_no_findings = findings_lines
        .iter()
        .any(|line| line.trim() == "No material findings.");
    let finding_titles = findings_lines
        .iter()
        .filter(|line| is_repo_review_finding_title(line))
        .map(|line| (*line).to_string())
        .collect::<Vec<_>>();
    ensure!(
        has_explicit_no_findings || !finding_titles.is_empty(),
        "Findings must contain explicit findings or 'No material findings.'"
    );
    if !finding_titles.is_empty() {
        let mut expected = finding_titles.clone();
        expected.sort_by_key(|line| repo_review_finding_sort_key(line));
        ensure!(
            finding_titles == expected,
            "Findings must be ordered by severity and stable title ordering"
        );
    }

    ensure!(
        !assessment.trim().is_empty(),
        "System-Level Assessment must not be empty"
    );
    ensure!(
        action_plan.contains("Fix now:"),
        "Recommended Action Plan must include Fix now"
    );
    ensure!(
        action_plan.contains("Fix before milestone closeout:"),
        "Recommended Action Plan must include Fix before milestone closeout"
    );
    ensure!(
        action_plan.contains("Defer:"),
        "Recommended Action Plan must include Defer"
    );
    ensure!(
        !follow_ups.trim().is_empty(),
        "Follow-ups / Deferred Work must not be empty"
    );
    ensure!(
        !final_assessment.trim().is_empty(),
        "Final Assessment must not be empty"
    );

    println!("PASS: repo review contract valid for {}", review.display());
    Ok(())
}

fn parse_review_arg(args: &[String], label: &str) -> Result<PathBuf> {
    let mut review: Option<PathBuf> = None;
    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--review" => {
                let Some(value) = args.get(i + 1) else {
                    bail!("missing --review");
                };
                review = Some(PathBuf::from(value));
                i += 1;
            }
            "--help" | "-h" => {
                println!("{}", tooling_usage());
                return Ok(PathBuf::new());
            }
            other => bail!("unknown arg for {label}: {other}"),
        }
        i += 1;
    }
    review.ok_or_else(|| anyhow!("missing --review"))
}
