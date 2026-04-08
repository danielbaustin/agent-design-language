use anyhow::{anyhow, bail, Result};
use serde::{Deserialize, Serialize};
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
struct RuntimeReviewSurfaceSummary {
    review_surface_version: String,
    review_root: String,
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

#[derive(Debug, Deserialize)]
struct RuntimeReviewManifest {
    review_surface_version: String,
    milestone: String,
    demo_id: String,
    review_root: String,
    review_readme: String,
    primary_proof_surface: String,
    demo_packages: Vec<RuntimeReviewPackage>,
}

#[derive(Debug, Deserialize)]
struct RuntimeReviewPackage {
    demo_id: String,
    title: String,
    review_readme: String,
    primary_proof_surface: String,
    secondary_proof_surfaces: Vec<String>,
}

pub(super) fn real_review_runtime_surface(args: &[String]) -> Result<()> {
    let mut review_root: Option<PathBuf> = None;
    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--review-root" => {
                let Some(value) = args.get(i + 1) else {
                    bail!("missing --review-root");
                };
                review_root = Some(PathBuf::from(value));
                i += 1;
            }
            "--help" | "-h" => {
                println!("{}", tooling_usage());
                return Ok(());
            }
            other => bail!("unknown arg for tooling review-runtime-surface: {other}"),
        }
        i += 1;
    }

    let review_root = review_root.ok_or_else(|| anyhow!("missing --review-root"))?;
    let root = repo_root()?;
    let review_root_abs = absolutize(&review_root)?;
    let review_root_rel = repo_relative_display(&root, &review_root_abs)?;
    let manifest_path = review_root_abs.join("demo_manifest.json");
    let readme_path = review_root_abs.join("README.md");
    let manifest_text = fs::read_to_string(&manifest_path)?;
    let readme_text = fs::read_to_string(&readme_path)?;
    let manifest: RuntimeReviewManifest = serde_json::from_str(&manifest_text)?;

    let mut checks = Vec::new();

    let manifest_shape_valid = manifest.review_surface_version == "adl.runtime_review_surface.v1"
        && manifest.milestone == "v0.87.1"
        && manifest.demo_id == "D8"
        && manifest.review_root == review_root_rel;
    add_check(
        &mut checks,
        "RRS-STR-001",
        "structure",
        "high",
        manifest_shape_valid,
        "Runtime review manifest identity is normalized",
        vec![format!("path:{review_root_rel}/demo_manifest.json")],
        if manifest_shape_valid {
            "Manifest version, milestone, demo_id, and review_root are normalized."
        } else {
            "Manifest identity fields do not match the canonical runtime review surface contract."
        },
    );

    let readme_rel = repo_relative_display(&root, &readme_path)?;
    let readme_declared_valid = manifest.review_readme == readme_rel;
    add_check(
        &mut checks,
        "RRS-STR-002",
        "structure",
        "high",
        readme_declared_valid,
        "Manifest review_readme points at the canonical README",
        vec![format!("path:{readme_rel}")],
        if readme_declared_valid {
            "Manifest review_readme matches the assembled review README."
        } else {
            "Manifest review_readme does not match the assembled review README path."
        },
    );

    let root_paths_valid = is_repo_relative(&manifest.review_root)
        && is_repo_relative(&manifest.review_readme)
        && is_repo_relative(&manifest.primary_proof_surface);
    let package_paths_valid = !manifest.demo_packages.is_empty()
        && manifest.demo_packages.iter().all(|pkg| {
            is_repo_relative(&pkg.review_readme)
                && is_repo_relative(&pkg.primary_proof_surface)
                && pkg
                    .secondary_proof_surfaces
                    .iter()
                    .all(|path| is_repo_relative(path))
        });
    add_check(
        &mut checks,
        "RRS-ART-001",
        "artifacts",
        "high",
        root_paths_valid && package_paths_valid,
        "Manifest artifact pointers are explicit and repo-relative",
        vec![format!("path:{review_root_rel}/demo_manifest.json")],
        if root_paths_valid && package_paths_valid {
            "All manifest artifact pointers are repo-relative."
        } else {
            "One or more manifest artifact pointers are blank or not repo-relative."
        },
    );

    let primary_exists = root.join(&manifest.primary_proof_surface).is_file();
    let package_artifacts_exist = manifest.demo_packages.iter().all(|pkg| {
        root.join(&pkg.review_readme).is_file()
            && root.join(&pkg.primary_proof_surface).is_file()
            && pkg
                .secondary_proof_surfaces
                .iter()
                .all(|path| root.join(path).is_file())
    });
    add_check(
        &mut checks,
        "RRS-ART-002",
        "artifacts",
        "high",
        primary_exists && package_artifacts_exist,
        "Primary and referenced proof surfaces exist",
        vec![format!("path:{review_root_rel}")],
        if primary_exists && package_artifacts_exist {
            "Manifest and package references resolve to real proof surfaces."
        } else {
            "One or more referenced proof surfaces do not exist."
        },
    );

    let package_layout_valid = manifest.demo_packages.len() == 2
        && manifest.demo_packages[0].demo_id == "D6"
        && manifest.demo_packages[0].title == "Operator Invocation Surface"
        && manifest.demo_packages[1].demo_id == "D7"
        && manifest.demo_packages[1].title == "Runtime State / Persistence Discipline";
    add_check(
        &mut checks,
        "RRS-STR-003",
        "structure",
        "medium",
        package_layout_valid,
        "Runtime review package contains the canonical D6/D7 entry ordering",
        vec![format!("path:{review_root_rel}/demo_manifest.json")],
        if package_layout_valid {
            "Package ordering matches the canonical runtime reviewer walkthrough."
        } else {
            "Package ordering or package identity does not match the canonical D6/D7 walkthrough."
        },
    );

    let reviewer_guidance_present = readme_text
        .contains("Review D6 first for the canonical operator entrypoint.")
        && readme_text
            .contains("Then inspect D7 for persistence, pause-state, and continuity evidence.")
        && readme_text.contains("Primary proof surface:")
        && readme_text.contains("Secondary proof surfaces:");
    add_check(
        &mut checks,
        "RRS-DOC-001",
        "documentation",
        "medium",
        reviewer_guidance_present,
        "Review README gives a bounded reviewer walkthrough",
        vec![format!("path:{readme_rel}")],
        if reviewer_guidance_present {
            "README contains the canonical reviewer walkthrough and proof-surface guidance."
        } else {
            "README is missing bounded reviewer walkthrough guidance."
        },
    );

    let absolute_path_free = !contains_absolute_host_path_in_text(&manifest_text)
        && !contains_absolute_host_path_in_text(&readme_text);
    add_check(
        &mut checks,
        "RRS-SEC-001",
        "security_privacy",
        "high",
        absolute_path_free,
        "Runtime review surface contains no absolute host paths",
        vec![
            format!("path:{review_root_rel}/demo_manifest.json"),
            format!("path:{readme_rel}"),
        ],
        if absolute_path_free {
            "No absolute host paths detected in the review package."
        } else {
            "Absolute host path pattern detected in the review package."
        },
    );

    let summary = RuntimeReviewSurfaceSummary {
        review_surface_version: "adl.runtime_review_surface.v1".to_string(),
        review_root: review_root_rel,
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
