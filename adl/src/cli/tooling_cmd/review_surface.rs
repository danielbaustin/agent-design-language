use anyhow::{anyhow, bail, Result};
use serde::Serialize;
use std::fs;
use std::path::PathBuf;

use super::common::{
    absolutize, contains_absolute_host_path_in_text, is_repo_relative, repo_relative_display,
    repo_root, ALLOWED_OUTPUT_STATUS, REQUIRED_OUTPUT_SECTIONS, REQUIRED_REVIEW_SURFACES,
};
use super::markdown::{
    markdown_block_field, markdown_field, markdown_has_heading, markdown_section_body,
};
use super::structured_prompt::prompt_spec_review_surfaces;
use super::tooling_usage;

#[derive(Serialize)]
struct ReviewSurfaceSummary {
    review_surface_version: String,
    review_target: ReviewTarget,
    decision: String,
    checks: Vec<ReviewCheck>,
}

#[derive(Serialize)]
struct ReviewTarget {
    input_card_path: String,
    output_card_path: String,
}

#[derive(Serialize)]
pub(super) struct ReviewCheck {
    pub(super) id: String,
    pub(super) domain: String,
    pub(super) severity: String,
    pub(super) status: String,
    pub(super) title: String,
    pub(super) evidence: Vec<String>,
    pub(super) notes: String,
}

pub(super) fn real_review_card_surface(args: &[String]) -> Result<()> {
    let mut input: Option<PathBuf> = None;
    let mut output: Option<PathBuf> = None;
    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--input" => {
                let Some(value) = args.get(i + 1) else {
                    bail!("missing --input");
                };
                input = Some(PathBuf::from(value));
                i += 1;
            }
            "--output" => {
                let Some(value) = args.get(i + 1) else {
                    bail!("missing --output");
                };
                output = Some(PathBuf::from(value));
                i += 1;
            }
            "--help" | "-h" => {
                println!("{}", tooling_usage());
                return Ok(());
            }
            other => bail!("unknown arg for tooling review-card-surface: {other}"),
        }
        i += 1;
    }

    let input = input.ok_or_else(|| anyhow!("missing --input"))?;
    let output = output.ok_or_else(|| anyhow!("missing --output"))?;
    let root = repo_root()?;
    let input_abs = absolutize(&input)?;
    let output_abs = absolutize(&output)?;
    let input_rel = repo_relative_display(&root, &input_abs)?;
    let output_rel = repo_relative_display(&root, &output_abs)?;
    let input_text = fs::read_to_string(&input_abs)?;
    let output_text = fs::read_to_string(&output_abs)?;

    let mut checks = Vec::new();
    add_check(
        &mut checks,
        "RVS-STR-001",
        "structure",
        "high",
        REQUIRED_OUTPUT_SECTIONS
            .iter()
            .all(|heading| markdown_has_heading(&output_text, heading)),
        "Required output-card sections are present",
        vec![format!("path:{output_rel}")],
        if REQUIRED_OUTPUT_SECTIONS
            .iter()
            .all(|heading| markdown_has_heading(&output_text, heading))
        {
            "All required sections were found."
        } else {
            "One or more required sections are missing."
        },
    );

    let status = markdown_field(&output_text, "Status").unwrap_or_default();
    let status_valid = ALLOWED_OUTPUT_STATUS.contains(&status.as_str());
    add_check(
        &mut checks,
        "RVS-STR-002",
        "structure",
        "medium",
        status_valid,
        "Output-card status is normalized",
        vec![format!("path:{output_rel}")],
        if status_valid {
            format!("Status={status}")
        } else {
            format!(
                "Status must be one of {}.",
                ALLOWED_OUTPUT_STATUS.join(", ")
            )
        },
    );

    let execution_complete = [
        markdown_block_field(&output_text, "Execution", "Actor"),
        markdown_block_field(&output_text, "Execution", "Model"),
        markdown_block_field(&output_text, "Execution", "Provider"),
        markdown_block_field(&output_text, "Execution", "Start Time"),
        markdown_block_field(&output_text, "Execution", "End Time"),
    ]
    .into_iter()
    .all(|v| v.is_some_and(|s| !s.is_empty()));
    add_check(
        &mut checks,
        "RVS-STR-003",
        "structure",
        "medium",
        execution_complete,
        "Execution metadata is present",
        vec![format!("path:{output_rel}")],
        if execution_complete {
            "Actor/model/provider/start/end fields are populated.".to_string()
        } else {
            "Execution metadata contains blank fields.".to_string()
        },
    );

    let artifacts = markdown_section_body(&output_text, "Artifacts produced")
        .unwrap_or_default()
        .lines()
        .filter_map(|line| {
            line.trim()
                .strip_prefix("- `")
                .and_then(|rest| rest.strip_suffix('`'))
                .map(|s| s.to_string())
        })
        .collect::<Vec<_>>();
    let artifact_paths_valid =
        !artifacts.is_empty() && artifacts.iter().all(|path| is_repo_relative(path));
    add_check(
        &mut checks,
        "RVS-ART-001",
        "artifacts",
        "high",
        artifact_paths_valid,
        "Artifact paths are explicit and repo-relative",
        vec![format!("path:{output_rel}")],
        if artifact_paths_valid {
            "Artifact list is populated with repo-relative paths.".to_string()
        } else {
            "Artifact list is blank or contains non-repo-relative paths.".to_string()
        },
    );

    let absolute_path_free = !contains_absolute_host_path_in_text(&output_text);
    add_check(
        &mut checks,
        "RVS-SEC-001",
        "security_privacy",
        "high",
        absolute_path_free,
        "Output card contains no absolute host paths",
        vec![format!("path:{output_rel}")],
        if absolute_path_free {
            "No absolute host paths detected.".to_string()
        } else {
            "Absolute host path pattern detected.".to_string()
        },
    );

    let surfaces = prompt_spec_review_surfaces(&input_text);
    let surfaces_valid = surfaces.is_empty() || surfaces == REQUIRED_REVIEW_SURFACES;
    add_check(
        &mut checks,
        "RVS-PRM-001",
        "validation",
        "medium",
        surfaces_valid,
        "Prompt-spec review surfaces are in canonical order when present",
        vec![format!("path:{input_rel}")],
        if surfaces.is_empty() {
            "No prompt-spec review surfaces were declared.".to_string()
        } else {
            format!(
                "Review surface ordering is {}.",
                if surfaces_valid { "valid" } else { "invalid" }
            )
        },
    );

    let summary = ReviewSurfaceSummary {
        review_surface_version: "adl.review_surface.v1".to_string(),
        review_target: ReviewTarget {
            input_card_path: input_rel,
            output_card_path: output_rel,
        },
        decision: decision_for(&checks),
        checks,
    };
    print!("---\n{}", serde_yaml::to_string(&summary)?);
    Ok(())
}

pub(super) fn decision_for(checks: &[ReviewCheck]) -> String {
    let severities = checks
        .iter()
        .filter(|check| check.status != "PASS")
        .map(|check| check.severity.as_str())
        .collect::<Vec<_>>();
    if severities.contains(&"high") {
        "MAJOR_ISSUES".to_string()
    } else if severities.contains(&"medium") {
        "MINOR_FIXES".to_string()
    } else {
        "PASS".to_string()
    }
}

#[allow(clippy::too_many_arguments)]
fn add_check(
    checks: &mut Vec<ReviewCheck>,
    id: &str,
    domain: &str,
    severity: &str,
    passed: bool,
    title: &str,
    evidence: Vec<String>,
    notes: impl Into<String>,
) {
    checks.push(ReviewCheck {
        id: id.to_string(),
        domain: domain.to_string(),
        severity: severity.to_string(),
        status: if passed { "PASS" } else { "FAIL" }.to_string(),
        title: title.to_string(),
        evidence,
        notes: notes.into(),
    });
}
