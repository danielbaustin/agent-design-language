use anyhow::{anyhow, bail, Result};
use serde::{Deserialize, Serialize};
use serde_yaml::{Mapping, Value};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use ::adl::control_plane::{card_input_path, resolve_cards_root, resolve_primary_checkout_root};

const REQUIRED_OUTPUT_SECTIONS: &[&str] = &[
    "Summary",
    "Artifacts produced",
    "Actions taken",
    "Main Repo Integration (REQUIRED)",
    "Validation",
    "Verification Summary",
    "Determinism Evidence",
    "Security / Privacy Checks",
    "Replay Artifacts",
    "Artifact Verification",
    "Decisions / Deviations",
    "Follow-ups / Deferred work",
];

const ALLOWED_OUTPUT_STATUS: &[&str] = &["NOT_STARTED", "IN_PROGRESS", "DONE", "FAILED"];
const REQUIRED_REVIEW_SURFACES: &[&str] = &[
    "card_review_checklist.v1",
    "card_review_output.v1",
    "card_reviewer_gpt.v1.1",
];

const POINTER_PREFIX_ORDER: &[&str] = &["path:", "command:", "ci:", "artifact:"];
const DECISIONS: &[&str] = &["PASS", "MINOR_FIXES", "MAJOR_ISSUES"];
const EVIDENCE_STATES: &[&str] = &["contradicted", "not_evidenced", "not_applicable"];
const VALIDATION_RESULTS: &[&str] = &["PASS", "FAIL", "PARTIAL"];
const REQUIRED_REPO_REVIEW_HEADINGS: &[&str] = &[
    "Metadata",
    "Scope",
    "Findings",
    "System-Level Assessment",
    "Recommended Action Plan",
    "Follow-ups / Deferred Work",
    "Final Assessment",
];

macro_rules! ensure {
    ($cond:expr, $($arg:tt)*) => {
        if $cond {
            Ok::<(), anyhow::Error>(())
        } else {
            bail!($($arg)*)
        }
    };
}

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
struct ReviewCheck {
    id: String,
    domain: String,
    severity: String,
    status: String,
    title: String,
    evidence: Vec<String>,
    notes: String,
}

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

pub(crate) fn real_tooling(args: &[String]) -> Result<()> {
    let Some(subcommand) = args.first().map(|arg| arg.as_str()) else {
        return Err(anyhow!(
            "tooling requires a subcommand: card-prompt | lint-prompt-spec | validate-structured-prompt | review-card-surface | verify-review-output-provenance | verify-repo-review-contract"
        ));
    };

    match subcommand {
        "card-prompt" => real_card_prompt(&args[1..]),
        "lint-prompt-spec" => real_lint_prompt_spec(&args[1..]),
        "validate-structured-prompt" => real_validate_structured_prompt(&args[1..]),
        "review-card-surface" => real_review_card_surface(&args[1..]),
        "verify-review-output-provenance" => real_verify_review_output_provenance(&args[1..]),
        "verify-repo-review-contract" => real_verify_repo_review_contract(&args[1..]),
        "--help" | "-h" | "help" => {
            println!("{}", tooling_usage());
            Ok(())
        }
        _ => Err(anyhow!(
            "unknown tooling subcommand '{subcommand}' (expected card-prompt | lint-prompt-spec | validate-structured-prompt | review-card-surface | verify-review-output-provenance | verify-repo-review-contract)"
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
adl tooling verify-review-output-provenance --review <yaml>\n\
adl tooling verify-repo-review-contract --review <markdown>"
}

fn real_card_prompt(args: &[String]) -> Result<()> {
    let mut issue: Option<u32> = None;
    let mut input: Option<PathBuf> = None;
    let mut out: Option<PathBuf> = None;
    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--issue" => {
                let Some(value) = args.get(i + 1) else {
                    bail!("missing value for --issue");
                };
                issue = Some(normalize_issue(value)?);
                i += 1;
            }
            "--input" => {
                let Some(value) = args.get(i + 1) else {
                    bail!("missing value for --input");
                };
                input = Some(PathBuf::from(value));
                i += 1;
            }
            "--out" => {
                let Some(value) = args.get(i + 1) else {
                    bail!("missing value for --out");
                };
                out = Some(PathBuf::from(value));
                i += 1;
            }
            "--help" | "-h" => {
                println!("{}", tooling_usage());
                return Ok(());
            }
            other => bail!("unknown arg for tooling card-prompt: {other}"),
        }
        i += 1;
    }

    if issue.is_some() && input.is_some() {
        bail!("use either --issue or --input, not both");
    }
    if issue.is_none() && input.is_none() {
        bail!("missing --issue or --input");
    }

    let input = match (issue, input) {
        (Some(issue), None) => resolve_input_card_path(issue)?,
        (None, Some(input)) => input,
        _ => unreachable!(),
    };
    ensure_file(&input, "input card")?;
    ensure_no_disallowed_content(&input, "input card")?;

    let input_text = fs::read_to_string(&input)?;
    let task_id = markdown_field(&input_text, "Task ID").unwrap_or_default();
    let run_id = markdown_field(&input_text, "Run ID").unwrap_or_default();
    let version = markdown_field(&input_text, "Version").unwrap_or_default();
    let title = markdown_field(&input_text, "Title").unwrap_or_default();
    let branch = markdown_field(&input_text, "Branch").unwrap_or_default();

    let prompt_spec = extract_prompt_spec_yaml(&input_text)?;
    let mut section_ids = prompt_spec_sections(&prompt_spec);
    if section_ids.is_empty() {
        section_ids = vec![
            "goal".to_string(),
            "required_outcome".to_string(),
            "acceptance_criteria".to_string(),
            "inputs".to_string(),
            "target_files_surfaces".to_string(),
            "validation_plan".to_string(),
            "demo_proof_requirements".to_string(),
            "constraints_policies".to_string(),
            "system_invariants".to_string(),
            "reviewer_checklist".to_string(),
            "non_goals_out_of_scope".to_string(),
            "notes_risks".to_string(),
            "instructions_to_agent".to_string(),
        ];
    }

    let include_system_invariants =
        prompt_spec_bool(&prompt_spec, "include_system_invariants").unwrap_or(true);
    let include_reviewer_checklist =
        prompt_spec_bool(&prompt_spec, "include_reviewer_checklist").unwrap_or(true);

    if include_reviewer_checklist {
        let has_checklist = section_ids.iter().any(|id| id == "reviewer_checklist");
        if !has_checklist {
            section_ids.push("reviewer_checklist".to_string());
        }
    }

    let mut ordered = Vec::new();
    let mut seen = BTreeSet::new();
    for id in section_ids {
        if !id.is_empty() && seen.insert(id.clone()) {
            ordered.push(id);
        }
    }

    let mut rendered_sections = Vec::new();
    for id in ordered {
        if id == "system_invariants" && !include_system_invariants {
            continue;
        }
        if id == "reviewer_checklist" && !include_reviewer_checklist {
            continue;
        }
        if let Some(header) = section_id_to_header(&id) {
            let body = trim_blank_edges(
                markdown_section_body(&input_text, header)
                    .unwrap_or_default()
                    .lines()
                    .map(|line| line.to_string())
                    .collect(),
            )
            .join("\n");
            rendered_sections.push(format!(
                "{header}\n{}",
                if body.is_empty() {
                    "(not provided)".to_string()
                } else {
                    body
                }
            ));
        }
    }

    let prompt = format!(
        "Work Prompt — {}\n\nContext\n- Task ID: {}\n- Run ID: {}\n- Version: {}\n- Title: {}\n- Branch: {}\n- Input Card: {}\n\n{}\n\nExecution Rules\n- Keep changes deterministic and replay-safe.\n- Do not broaden scope beyond the card.\n- Run validation commands required by the card/repo.\n- Update the paired output card with concrete evidence.\n",
        if task_id.is_empty() { "issue-unknown" } else { &task_id },
        if task_id.is_empty() { "unknown" } else { &task_id },
        if run_id.is_empty() { "unknown" } else { &run_id },
        if version.is_empty() { "unknown" } else { &version },
        if title.is_empty() { "unknown" } else { &title },
        if branch.is_empty() { "unknown" } else { &branch },
        display_card_ref(&input)?,
        rendered_sections.join("\n\n")
    );

    if let Some(out) = out {
        if let Some(parent) = out.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(out, prompt)?;
    } else {
        print!("{prompt}");
    }
    Ok(())
}

fn real_lint_prompt_spec(args: &[String]) -> Result<()> {
    let input = resolve_issue_or_input_arg(args)?;
    ensure_file(&input, "input card")?;
    ensure_no_disallowed_content(&input, "input card")?;
    let text = fs::read_to_string(&input)?;
    let spec = extract_prompt_spec_yaml(&text)?;
    validate_prompt_spec(&spec)?;
    println!("PASS: Prompt Spec is valid for {}", input.display());
    Ok(())
}

fn real_validate_structured_prompt(args: &[String]) -> Result<()> {
    let mut prompt_type: Option<String> = None;
    let mut input: Option<PathBuf> = None;
    let mut phase: Option<String> = None;
    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--type" => {
                let Some(value) = args.get(i + 1) else {
                    bail!("missing value for --type");
                };
                prompt_type = Some(value.clone());
                i += 1;
            }
            "--input" => {
                let Some(value) = args.get(i + 1) else {
                    bail!("missing value for --input");
                };
                input = Some(PathBuf::from(value));
                i += 1;
            }
            "--phase" => {
                let Some(value) = args.get(i + 1) else {
                    bail!("missing value for --phase");
                };
                phase = Some(value.clone());
                i += 1;
            }
            "--help" | "-h" => {
                println!("{}", tooling_usage());
                return Ok(());
            }
            other => bail!("unknown arg for tooling validate-structured-prompt: {other}"),
        }
        i += 1;
    }

    let prompt_type = prompt_type.ok_or_else(|| anyhow!("missing --type"))?;
    let input = input.ok_or_else(|| anyhow!("missing --input"))?;
    ensure_file(&input, "input")?;
    ensure_no_absolute_host_path(&input, &prompt_type)?;
    let text = fs::read_to_string(&input)?;
    match prompt_type.as_str() {
        "stp" => validate_stp_text(&text)?,
        "sip" => validate_sip_text(&text, &input)?,
        "sor" => validate_sor_text(&text, phase.as_deref())?,
        _ => bail!("unsupported --type: {}", prompt_type),
    }
    println!(
        "PASS: {} contract valid for {}",
        prompt_type,
        input.display()
    );
    Ok(())
}

fn real_review_card_surface(args: &[String]) -> Result<()> {
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

fn real_verify_review_output_provenance(args: &[String]) -> Result<()> {
    let review = parse_review_arg(args, "tooling verify-review-output-provenance")?;
    let root = repo_root()?;
    ensure_file(&review, "review artifact")?;
    let artifact: ReviewOutputArtifact = serde_yaml::from_str(&fs::read_to_string(&review)?)?;
    ensure!(
        artifact.review_format_version == "card_review_output.v1",
        "unexpected review_format_version"
    )?;
    ensure!(
        DECISIONS.contains(&artifact.decision.as_str()),
        "invalid decision enum"
    )?;
    ensure!(
        is_repo_relative(&artifact.review_target.input_card_path),
        "input_card_path must be repo-relative"
    )?;
    ensure!(
        is_repo_relative(&artifact.review_target.output_card_path),
        "output_card_path must be repo-relative"
    )?;
    ensure!(
        root.join(&artifact.review_target.input_card_path).is_file(),
        "input card path does not exist: {}",
        artifact.review_target.input_card_path
    )?;
    ensure!(
        root.join(&artifact.review_target.output_card_path)
            .is_file(),
        "output card path does not exist: {}",
        artifact.review_target.output_card_path
    )?;
    for finding in &artifact.findings {
        ensure!(
            EVIDENCE_STATES.contains(&finding.evidence_state.as_str()),
            "invalid evidence_state"
        )?;
        ensure!(
            !finding.evidence.is_empty(),
            "finding evidence must be a non-empty array"
        )?;
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
                )?;
            }
        }
    }
    ensure!(
        VALIDATION_RESULTS.contains(&artifact.validation_checks.validation_result.as_str()),
        "invalid validation_result enum"
    )?;
    for command in &artifact.validation_checks.commands {
        ensure!(
            !contains_absolute_host_path_in_text(command),
            "validation command leaks absolute host path"
        )?;
    }
    if !artifact.security_privacy_checks.absolute_host_paths_present {
        let serialized = fs::read_to_string(&review)?;
        ensure!(
            !contains_absolute_host_path_in_text(&serialized),
            "review artifact contradicts absolute_host_paths_present=false"
        )?;
    }
    println!(
        "PASS: review output provenance valid for {}",
        review.display()
    );
    Ok(())
}

fn real_verify_repo_review_contract(args: &[String]) -> Result<()> {
    let review = parse_review_arg(args, "tooling verify-repo-review-contract")?;
    ensure_file(&review, "review artifact")?;
    let text = fs::read_to_string(&review)?;
    ensure!(
        !contains_absolute_host_path_in_text(&text),
        "review artifact contains absolute host path"
    )?;
    let headings = markdown_headings(&text);
    ensure!(
        headings == REQUIRED_REPO_REVIEW_HEADINGS,
        "review headings must match canonical order exactly"
    )?;
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
    )?;
    ensure!(
        metadata.contains("Subject:"),
        "Metadata must include Subject"
    )?;
    ensure!(
        metadata.contains("Reviewer:"),
        "Metadata must include Reviewer"
    )?;
    ensure!(scope.contains("Reviewed:"), "Scope must include Reviewed")?;
    ensure!(
        scope.contains("Not Reviewed:"),
        "Scope must include Not Reviewed"
    )?;
    ensure!(
        scope.contains("Review Mode:"),
        "Scope must include Review Mode"
    )?;
    ensure!(scope.contains("Gate:"), "Scope must include Gate")?;

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
    )?;
    if !finding_titles.is_empty() {
        let mut expected = finding_titles.clone();
        expected.sort_by_key(|line| repo_review_finding_sort_key(line));
        ensure!(
            finding_titles == expected,
            "Findings must be ordered by severity and stable title ordering"
        )?;
    }

    ensure!(
        !assessment.trim().is_empty(),
        "System-Level Assessment must not be empty"
    )?;
    ensure!(
        action_plan.contains("Fix now:"),
        "Recommended Action Plan must include Fix now"
    )?;
    ensure!(
        action_plan.contains("Fix before milestone closeout:"),
        "Recommended Action Plan must include Fix before milestone closeout"
    )?;
    ensure!(
        action_plan.contains("Defer:"),
        "Recommended Action Plan must include Defer"
    )?;
    ensure!(
        !follow_ups.trim().is_empty(),
        "Follow-ups / Deferred Work must not be empty"
    )?;
    ensure!(
        !final_assessment.trim().is_empty(),
        "Final Assessment must not be empty"
    )?;

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

fn resolve_issue_or_input_arg(args: &[String]) -> Result<PathBuf> {
    let mut issue: Option<u32> = None;
    let mut input: Option<PathBuf> = None;
    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--issue" => {
                let Some(value) = args.get(i + 1) else {
                    bail!("missing value for --issue");
                };
                issue = Some(normalize_issue(value)?);
                i += 1;
            }
            "--input" => {
                let Some(value) = args.get(i + 1) else {
                    bail!("missing value for --input");
                };
                input = Some(PathBuf::from(value));
                i += 1;
            }
            "--help" | "-h" => {
                println!("{}", tooling_usage());
                return Ok(PathBuf::new());
            }
            other => bail!("unknown argument: {other}"),
        }
        i += 1;
    }
    if issue.is_some() && input.is_some() {
        bail!("use either --issue or --input, not both");
    }
    if let Some(issue) = issue {
        resolve_input_card_path(issue)
    } else if let Some(input) = input {
        Ok(input)
    } else {
        bail!("missing --issue or --input")
    }
}

fn resolve_input_card_path(issue: u32) -> Result<PathBuf> {
    let current_top =
        git_output(&["rev-parse", "--show-toplevel"]).unwrap_or_else(|_| cwd_string());
    let current_top = PathBuf::from(current_top);
    let common = git_output(&["rev-parse", "--git-common-dir"])
        .ok()
        .map(PathBuf::from);
    let root = resolve_primary_checkout_root(&current_top, common.as_deref());
    let cards_root = resolve_cards_root(&root, None);
    Ok(card_input_path(&cards_root, issue))
}

fn repo_root() -> Result<PathBuf> {
    let root = git_output(&["rev-parse", "--show-toplevel"])?;
    Ok(PathBuf::from(root))
}

fn git_output(args: &[&str]) -> Result<String> {
    let output = Command::new("git").args(args).output()?;
    if !output.status.success() {
        bail!("git command failed: git {}", args.join(" "));
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn cwd_string() -> String {
    std::env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .to_string_lossy()
        .to_string()
}

fn normalize_issue(raw: &str) -> Result<u32> {
    raw.parse::<u32>()
        .map_err(|_| anyhow!("invalid issue number: {raw}"))
}

fn absolutize(path: &Path) -> Result<PathBuf> {
    if path.is_absolute() {
        Ok(path.to_path_buf())
    } else {
        Ok(std::env::current_dir()?.join(path))
    }
}

fn repo_relative_display(root: &Path, path: &Path) -> Result<String> {
    Ok(path
        .strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/"))
}

fn ensure_file(path: &Path, label: &str) -> Result<()> {
    let path = absolutize(path)?;
    if !path.is_file() {
        bail!("{label} not found: {}", path.display());
    }
    Ok(())
}

fn ensure_no_disallowed_content(path: &Path, label: &str) -> Result<()> {
    let text = fs::read_to_string(path)?;
    if contains_absolute_host_path_in_text(&text) || contains_secret_like_token(&text) {
        bail!("{label} contains disallowed secret-like token or absolute host path");
    }
    Ok(())
}

fn ensure_no_absolute_host_path(path: &Path, prompt_type: &str) -> Result<()> {
    let text = fs::read_to_string(path)?;
    if contains_absolute_host_path_in_text(&text) {
        bail!("{prompt_type} contains disallowed absolute host path");
    }
    Ok(())
}

fn contains_absolute_host_path_in_text(text: &str) -> bool {
    ["/Users/", "/home/", "/tmp/", "/var/folders/"]
        .iter()
        .any(|needle| text.contains(needle))
        || text.contains(":\\")
}

fn contains_secret_like_token(text: &str) -> bool {
    if text.contains("AKIA")
        || text.contains("ghp_")
        || text.contains("gho_")
        || text.contains("ghu_")
        || text.contains("ghs_")
        || text.contains("ghr_")
    {
        return true;
    }

    for (idx, _) in text.match_indices("sk-") {
        let before_ok = idx == 0
            || !text[..idx]
                .chars()
                .next_back()
                .is_some_and(|c| c.is_ascii_alphanumeric() || c == '_');
        if !before_ok {
            continue;
        }
        let suffix = &text[idx + 3..];
        let token_len = suffix
            .chars()
            .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
            .count();
        if token_len < 8 {
            continue;
        }
        let after = suffix.chars().nth(token_len);
        let after_ok = !after.is_some_and(|c| c.is_ascii_alphanumeric() || c == '_');
        if after_ok {
            return true;
        }
    }

    false
}

fn extract_prompt_spec_yaml(text: &str) -> Result<String> {
    let prompt_block = markdown_section_body(text, "Prompt Spec")
        .ok_or_else(|| anyhow!("missing Prompt Spec YAML block"))?;
    let mut in_yaml = false;
    let mut lines = Vec::new();
    for line in prompt_block.lines() {
        let trimmed = line.trim_end();
        if trimmed == "```yaml" {
            in_yaml = true;
            continue;
        }
        if in_yaml && trimmed == "```" {
            return Ok(lines.join("\n"));
        }
        if in_yaml {
            lines.push(trimmed.to_string());
        }
    }
    bail!("missing Prompt Spec YAML block")
}

fn prompt_spec_sections(spec: &str) -> Vec<String> {
    let mut in_inputs = false;
    let mut in_sections = false;
    let mut out = Vec::new();
    for line in spec.lines() {
        if line.trim() == "inputs:" {
            in_inputs = true;
            continue;
        }
        if in_inputs && line.trim() == "outputs:" {
            break;
        }
        if in_inputs && line.trim() == "sections:" {
            in_sections = true;
            continue;
        }
        if in_sections {
            if let Some(rest) = line.trim_start().strip_prefix("- ") {
                out.push(rest.trim().to_string());
            } else if !line.starts_with("    ") && !line.trim().is_empty() {
                in_sections = false;
            }
        }
    }
    out
}

fn prompt_spec_bool(spec: &str, key: &str) -> Option<bool> {
    for line in spec.lines() {
        let trimmed = line.trim();
        if let Some(value) = trimmed.strip_prefix(&format!("{key}:")) {
            let value = value.trim().to_ascii_lowercase();
            return match value.as_str() {
                "true" => Some(true),
                "false" => Some(false),
                _ => None,
            };
        }
    }
    None
}

fn validate_prompt_spec(spec: &str) -> Result<()> {
    let yaml: Value = serde_yaml::from_str(spec)?;
    let mapping = yaml
        .as_mapping()
        .ok_or_else(|| anyhow!("Prompt Spec must be a YAML mapping"))?;

    for key in [
        "prompt_schema",
        "actor",
        "model",
        "inputs",
        "outputs",
        "constraints",
        "automation_hints",
        "review_surfaces",
    ] {
        ensure!(
            mapping_contains(mapping, key),
            "Prompt Spec missing required key: {key}"
        )?;
    }

    ensure!(
        mapping_string(mapping, "prompt_schema").as_deref() == Some("adl.v1"),
        "unsupported prompt_schema: {}",
        mapping_string(mapping, "prompt_schema").unwrap_or_else(|| "<empty>".to_string())
    )?;

    let inputs = mapping_mapping(mapping, "inputs")?;
    let sections = inputs
        .get(Value::String("sections".to_string()))
        .and_then(Value::as_sequence)
        .ok_or_else(|| anyhow!("Prompt Spec missing inputs.sections"))?;
    ensure!(
        !sections.is_empty(),
        "inputs.sections must include at least one section id"
    )?;
    let supported_sections = BTreeSet::from([
        "goal",
        "required_outcome",
        "acceptance_criteria",
        "inputs",
        "target_files_surfaces",
        "validation_plan",
        "demo_proof_requirements",
        "constraints_policies",
        "system_invariants",
        "reviewer_checklist",
        "non_goals_out_of_scope",
        "notes_risks",
        "instructions_to_agent",
    ]);
    for id in sections {
        let id = id
            .as_str()
            .ok_or_else(|| anyhow!("inputs.sections entries must be strings"))?;
        ensure!(
            supported_sections.contains(id),
            "unsupported inputs.sections entry: {id}"
        )?;
    }

    let constraints = mapping_mapping(mapping, "constraints")?;
    for key in [
        "include_system_invariants",
        "include_reviewer_checklist",
        "disallow_secrets",
        "disallow_absolute_host_paths",
    ] {
        ensure_bool(
            constraints,
            key,
            &format!("Prompt Spec missing constraints.{key}"),
        )?;
    }

    let hints = mapping_mapping(mapping, "automation_hints")?;
    for key in [
        "source_issue_prompt_required",
        "target_files_surfaces_recommended",
        "validation_plan_required",
        "required_outcome_type_supported",
    ] {
        ensure_bool(
            hints,
            key,
            &format!("Prompt Spec missing automation_hints.{key}"),
        )?;
    }

    let surfaces = mapping
        .get(Value::String("review_surfaces".to_string()))
        .and_then(Value::as_sequence)
        .ok_or_else(|| anyhow!("review_surfaces must include at least one protocol id"))?;
    ensure!(
        !surfaces.is_empty(),
        "review_surfaces must include at least one protocol id"
    )?;
    let mut has_checklist = false;
    let mut has_output = false;
    let mut has_reviewer = false;
    for surface in surfaces {
        match surface.as_str().unwrap_or_default() {
            "card_review_checklist.v1" => has_checklist = true,
            "card_review_output.v1" => has_output = true,
            "card_reviewer_gpt.v1.1" => has_reviewer = true,
            _ => {}
        }
    }
    ensure!(
        has_checklist && has_output && has_reviewer,
        "review_surfaces must include card_review_checklist.v1, card_review_output.v1, and card_reviewer_gpt.v1.1"
    )?;
    Ok(())
}

fn validate_stp_text(text: &str) -> Result<()> {
    let (fm_text, body_text) = split_front_matter(text)?;
    let fm: Value = serde_yaml::from_str(&fm_text)?;
    let fm = fm
        .as_mapping()
        .ok_or_else(|| anyhow!("front matter must be a YAML mapping"))?;

    for section in [
        "Summary",
        "Goal",
        "Required Outcome",
        "Deliverables",
        "Acceptance Criteria",
        "Repo Inputs",
        "Dependencies",
        "Demo Expectations",
        "Non-goals",
        "Issue-Graph Notes",
        "Notes",
        "Tooling Notes",
    ] {
        ensure!(
            markdown_has_heading(&body_text, section),
            "missing required section: {section}"
        )?;
    }

    ensure!(
        mapping_string(fm, "issue_card_schema").as_deref() == Some("adl.issue.v1"),
        "issue_card_schema must be one of: adl.issue.v1"
    )?;
    ensure!(
        !mapping_string(fm, "wp").unwrap_or_default().is_empty(),
        "missing required field: wp"
    )?;
    let slug = mapping_string(fm, "slug").unwrap_or_default();
    ensure!(!slug.is_empty(), "missing required field: slug")?;
    ensure!(is_normalized_slug(&slug), "slug must be a normalized slug")?;
    ensure!(
        !mapping_string(fm, "title").unwrap_or_default().is_empty(),
        "missing required field: title"
    )?;
    ensure!(
        mapping_seq_len(fm, "labels") >= 1,
        "labels must contain at least 1 item(s)"
    )?;
    ensure!(
        mapping_string(fm, "issue_number")
            .and_then(|v| v.parse::<u32>().ok())
            .is_some(),
        "issue_number must be an integer"
    )?;
    let status = mapping_string(fm, "status").unwrap_or_default();
    ensure!(
        ["draft", "active", "complete"].contains(&status.as_str()),
        "status must be one of: draft, active, complete"
    )?;
    let action = mapping_string(fm, "action").unwrap_or_default();
    ensure!(
        ["create", "edit", "close", "split", "supersede"].contains(&action.as_str()),
        "action must be one of: create, edit, close, split, supersede"
    )?;
    ensure!(
        mapping_contains(fm, "depends_on"),
        "missing required field: depends_on"
    )?;
    ensure!(
        !mapping_string(fm, "milestone_sprint")
            .unwrap_or_default()
            .is_empty(),
        "missing required field: milestone_sprint"
    )?;
    ensure!(
        mapping_seq_len(fm, "required_outcome_type") >= 1,
        "required_outcome_type must contain at least 1 item(s)"
    )?;
    ensure!(
        mapping_contains(fm, "repo_inputs"),
        "missing required field: repo_inputs"
    )?;
    ensure!(
        mapping_contains(fm, "canonical_files"),
        "missing required field: canonical_files"
    )?;
    let demo_required = mapping_bool(fm, "demo_required")
        .ok_or_else(|| anyhow!("demo_required must be true or false"))?;
    let _ = demo_required;
    ensure!(
        mapping_contains(fm, "demo_names"),
        "missing required field: demo_names"
    )?;
    ensure!(
        mapping_contains(fm, "issue_graph_notes"),
        "missing required field: issue_graph_notes"
    )?;
    let pr_start = mapping_mapping(fm, "pr_start")?;
    ensure_bool(
        pr_start,
        "enabled",
        "pr_start.enabled must be true or false",
    )?;
    let pr_start_slug = mapping_string(pr_start, "slug").unwrap_or_default();
    ensure!(
        !pr_start_slug.is_empty() && is_normalized_slug(&pr_start_slug),
        "pr_start.slug must be a normalized slug"
    )?;
    Ok(())
}

fn validate_sip_text(text: &str, path: &Path) -> Result<()> {
    for section in [
        "Goal",
        "Required Outcome",
        "Acceptance Criteria",
        "Inputs",
        "Target Files / Surfaces",
        "Validation Plan",
        "Demo / Proof Requirements",
        "Constraints / Policies",
        "System Invariants (must remain true)",
        "Reviewer Checklist (machine-readable hints)",
        "Non-goals / Out of scope",
        "Notes / Risks",
        "Instructions to the Agent",
    ] {
        ensure!(
            markdown_has_heading(text, section),
            "missing required section: {section}"
        )?;
    }
    ensure!(
        valid_task_id(&markdown_field(text, "Task ID").unwrap_or_default()),
        "Task ID must match issue-0000"
    )?;
    ensure!(
        valid_task_id(&markdown_field(text, "Run ID").unwrap_or_default()),
        "Run ID must match issue-0000"
    )?;
    ensure!(
        valid_version(&markdown_field(text, "Version").unwrap_or_default()),
        "Version must match milestone version format (for example v0.85 or v0.87.1)"
    )?;
    ensure!(
        !markdown_field(text, "Title").unwrap_or_default().is_empty(),
        "missing required field: Title"
    )?;
    ensure!(
        valid_branch(&markdown_field(text, "Branch").unwrap_or_default()),
        "Branch must be a codex/ branch"
    )?;
    let issue = markdown_block_field(text, "Context", "Issue").unwrap_or_default();
    ensure!(
        valid_github_issue_url(&issue),
        "Context.Issue must be a GitHub issue URL"
    )?;
    let pr = markdown_block_field(text, "Context", "PR").unwrap_or_default();
    ensure!(
        pr.is_empty() || valid_github_pr_url(&pr),
        "Context.PR must be a GitHub PR URL"
    )?;
    let source = markdown_block_field(text, "Context", "Source Issue Prompt").unwrap_or_default();
    ensure!(
        !source.is_empty() && valid_reference(&source),
        "Context.Source Issue Prompt must be a repo-relative reference or URL"
    )?;
    ensure!(
        !markdown_block_field(text, "Context", "Docs")
            .unwrap_or_default()
            .is_empty(),
        "missing required field: Context.Docs"
    )?;
    ensure!(
        !markdown_block_field(text, "Context", "Other")
            .unwrap_or_default()
            .is_empty(),
        "missing required field: Context.Other"
    )?;
    let source_slug =
        markdown_block_field(text, "Execution", "Source issue-prompt slug").unwrap_or_default();
    ensure!(
        source_slug.is_empty() || is_normalized_slug(&source_slug),
        "Execution.Source issue-prompt slug must be a normalized slug"
    )?;
    let outcome =
        markdown_block_field(text, "Execution", "Required outcome type").unwrap_or_default();
    ensure!(
        outcome.is_empty()
            || ["code", "docs", "tests", "demo", "combination"].contains(&outcome.as_str()),
        "Execution.Required outcome type must be one of: code, docs, tests, demo, combination"
    )?;
    let demo_required =
        markdown_block_field(text, "Execution", "Demo required").unwrap_or_default();
    ensure!(
        demo_required.is_empty() || ["true", "false"].contains(&demo_required.as_str()),
        "Execution.Demo required must be true or false"
    )?;
    let _ = path;
    validate_prompt_spec(&extract_prompt_spec_yaml(text)?)?;
    Ok(())
}

fn validate_sor_text(text: &str, phase: Option<&str>) -> Result<()> {
    for section in REQUIRED_OUTPUT_SECTIONS {
        ensure!(
            markdown_has_heading(text, section),
            "missing required section: {section}"
        )?;
    }
    ensure!(
        valid_task_id(&markdown_field(text, "Task ID").unwrap_or_default()),
        "Task ID must match issue-0000"
    )?;
    ensure!(
        valid_task_id(&markdown_field(text, "Run ID").unwrap_or_default()),
        "Run ID must match issue-0000"
    )?;
    ensure!(
        valid_version(&markdown_field(text, "Version").unwrap_or_default()),
        "Version must match milestone version format (for example v0.85 or v0.87.1)"
    )?;
    ensure!(
        !markdown_field(text, "Title").unwrap_or_default().is_empty(),
        "missing required field: Title"
    )?;
    ensure!(
        valid_branch(&markdown_field(text, "Branch").unwrap_or_default()),
        "Branch must be a codex/ branch"
    )?;
    let status = markdown_field(text, "Status").unwrap_or_default();
    ensure!(
        ALLOWED_OUTPUT_STATUS.contains(&status.as_str()),
        "Status must be one of: NOT_STARTED, IN_PROGRESS, DONE, FAILED"
    )?;
    let start = markdown_block_field(text, "Execution", "Start Time").unwrap_or_default();
    ensure!(
        start.is_empty() || valid_iso8601_datetime(&start),
        "Execution.Start Time must be UTC ISO 8601 / RFC3339 with trailing Z (YYYY-MM-DDTHH:MM:SSZ)"
    )?;
    let end = markdown_block_field(text, "Execution", "End Time").unwrap_or_default();
    ensure!(
        end.is_empty() || valid_iso8601_datetime(&end),
        "Execution.End Time must be UTC ISO 8601 / RFC3339 with trailing Z (YYYY-MM-DDTHH:MM:SSZ)"
    )?;
    let integration_state = markdown_block_field(
        text,
        "Main Repo Integration (REQUIRED)",
        "Integration state",
    )
    .unwrap_or_default();
    ensure!(
        integration_state.is_empty()
            || ["worktree_only", "pr_open", "merged"].contains(&integration_state.as_str()),
        "Main Repo Integration.Integration state must be one of: worktree_only, pr_open, merged"
    )?;
    let verification_scope = markdown_block_field(
        text,
        "Main Repo Integration (REQUIRED)",
        "Verification scope",
    )
    .unwrap_or_default();
    ensure!(
        verification_scope.is_empty()
            || ["worktree", "pr_branch", "main_repo"].contains(&verification_scope.as_str()),
        "Main Repo Integration.Verification scope must be one of: worktree, pr_branch, main_repo"
    )?;
    let result = markdown_block_field(text, "Main Repo Integration (REQUIRED)", "Result")
        .unwrap_or_default();
    ensure!(
        result.is_empty() || ["PASS", "FAIL"].contains(&result.as_str()),
        "Main Repo Integration.Result must be one of: PASS, FAIL"
    )?;

    if phase == Some("completed") {
        ensure!(
            ["DONE", "FAILED"].contains(&status.as_str()),
            "completed-phase SOR Status must be DONE or FAILED"
        )?;
        ensure!(
            !start.is_empty(),
            "completed-phase SOR requires Execution.Start Time"
        )?;
        ensure!(
            !end.is_empty(),
            "completed-phase SOR requires Execution.End Time"
        )?;
        ensure!(
            !markdown_section_body(text, "Summary")
                .unwrap_or_default()
                .trim()
                .is_empty(),
            "completed-phase SOR requires non-empty Summary content"
        )?;
        ensure!(
            !markdown_section_body(text, "Actions taken")
                .unwrap_or_default()
                .trim()
                .is_empty(),
            "completed-phase SOR requires non-empty Actions taken content"
        )?;
        ensure!(
            !markdown_section_body(text, "Validation")
                .unwrap_or_default()
                .trim()
                .is_empty(),
            "completed-phase SOR requires non-empty Validation content"
        )?;
        ensure!(
            !integration_state.is_empty(),
            "completed-phase SOR requires Main Repo Integration.Integration state"
        )?;
        ensure!(
            !result.is_empty(),
            "completed-phase SOR requires Main Repo Integration.Result"
        )?;
    }
    Ok(())
}

fn split_front_matter(text: &str) -> Result<(String, String)> {
    let mut lines = text.lines();
    let first = lines.next().unwrap_or_default();
    ensure!(first.trim() == "---", "missing YAML front matter opener")?;
    let all = text.lines().collect::<Vec<_>>();
    let close = all
        .iter()
        .enumerate()
        .skip(1)
        .find(|(_, line)| line.trim() == "---")
        .map(|(idx, _)| idx)
        .ok_or_else(|| anyhow!("missing YAML front matter closer"))?;
    let fm = all[1..close].join("\n");
    let body = all[close + 1..].join("\n");
    Ok((fm, body))
}

fn markdown_has_heading(text: &str, heading: &str) -> bool {
    text.lines()
        .any(|line| line.trim_end() == format!("## {heading}"))
}

fn markdown_headings(text: &str) -> Vec<&str> {
    text.lines()
        .filter_map(|line| line.strip_prefix("## "))
        .collect()
}

fn markdown_section_body(text: &str, heading: &str) -> Option<String> {
    let mut lines = text.lines();
    let mut in_section = false;
    let mut out = Vec::new();
    for line in lines.by_ref() {
        if !in_section {
            if line.trim_end() == format!("## {heading}") {
                in_section = true;
            }
            continue;
        }
        if line.starts_with("## ") {
            break;
        }
        out.push(line.to_string());
    }
    if in_section {
        Some(out.join("\n"))
    } else {
        None
    }
}

fn markdown_field(text: &str, key: &str) -> Option<String> {
    text.lines().find_map(|line| {
        line.strip_prefix(&format!("{key}:"))
            .map(|value| value.trim().to_string())
    })
}

fn markdown_block_field(text: &str, block: &str, key: &str) -> Option<String> {
    let body = markdown_named_block_body(text, block)?;
    body.lines().find_map(|line| {
        line.trim_start()
            .strip_prefix(&format!("- {key}:"))
            .map(|value| value.trim().to_string())
    })
}

fn markdown_named_block_body(text: &str, block: &str) -> Option<String> {
    if let Some(body) = markdown_section_body(text, block) {
        return Some(body);
    }

    let mut in_block = false;
    let mut out = Vec::new();
    for line in text.lines() {
        if !in_block {
            if line.trim_end() == format!("{block}:") {
                in_block = true;
            }
            continue;
        }
        if line.starts_with("## ") {
            break;
        }
        if !line.trim().is_empty()
            && !line.starts_with("- ")
            && !line.starts_with("  ")
            && line.ends_with(':')
        {
            break;
        }
        out.push(line.to_string());
    }

    if in_block {
        Some(out.join("\n"))
    } else {
        None
    }
}

fn trim_blank_edges(lines: Vec<String>) -> Vec<String> {
    let first = lines.iter().position(|line| !line.trim().is_empty());
    let last = lines.iter().rposition(|line| !line.trim().is_empty());
    match (first, last) {
        (Some(start), Some(end)) if start <= end => lines[start..=end].to_vec(),
        _ => Vec::new(),
    }
}

fn section_id_to_header(id: &str) -> Option<&'static str> {
    match id {
        "goal" => Some("Goal"),
        "required_outcome" => Some("Required Outcome"),
        "acceptance_criteria" => Some("Acceptance Criteria"),
        "inputs" => Some("Inputs"),
        "target_files_surfaces" => Some("Target Files / Surfaces"),
        "validation_plan" => Some("Validation Plan"),
        "demo_proof_requirements" => Some("Demo / Proof Requirements"),
        "constraints_policies" => Some("Constraints / Policies"),
        "system_invariants" => Some("System Invariants (must remain true)"),
        "reviewer_checklist" => Some("Reviewer Checklist (machine-readable hints)"),
        "non_goals_out_of_scope" => Some("Non-goals / Out of scope"),
        "notes_risks" => Some("Notes / Risks"),
        "instructions_to_agent" => Some("Instructions to the Agent"),
        _ => None,
    }
}

fn display_card_ref(path: &Path) -> Result<String> {
    let path = absolutize(path)?;
    let normalized = path.to_string_lossy().replace('\\', "/");
    if let Some(idx) = normalized.find(".adl/") {
        return Ok(normalized[idx..].to_string());
    }
    if let Some(issue) = normalized
        .rsplit('/')
        .next()
        .and_then(|name| name.strip_prefix("issue-"))
        .and_then(|rest| rest.split("__input__v").next())
    {
        if issue.len() == 4 && issue.chars().all(|c| c.is_ascii_digit()) {
            return Ok(format!("issue-{issue} (legacy input card)"));
        }
    }
    Ok(path
        .file_name()
        .map(|name| name.to_string_lossy().to_string())
        .unwrap_or(normalized))
}

fn prompt_spec_review_surfaces(text: &str) -> Vec<String> {
    extract_prompt_spec_yaml(text)
        .ok()
        .and_then(|spec| serde_yaml::from_str::<Value>(&spec).ok())
        .and_then(|value| value.as_mapping().cloned())
        .and_then(|mapping| {
            mapping
                .get(Value::String("review_surfaces".to_string()))
                .and_then(Value::as_sequence)
                .map(|seq| {
                    seq.iter()
                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                        .collect()
                })
        })
        .unwrap_or_default()
}

fn decision_for(checks: &[ReviewCheck]) -> String {
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

fn ensure_sorted_pointers(items: &[String], label: &str) -> Result<()> {
    let mut sorted = items.to_vec();
    sorted.sort_by_key(|pointer| pointer_sort_key(pointer));
    ensure!(
        items == sorted,
        "{label} must use canonical evidence-pointer ordering"
    )?;
    Ok(())
}

fn pointer_sort_key(pointer: &str) -> (usize, String) {
    let prefix = POINTER_PREFIX_ORDER
        .iter()
        .position(|candidate| pointer.starts_with(candidate))
        .unwrap_or(usize::MAX);
    (prefix, pointer.to_string())
}

fn is_repo_relative(value: &str) -> bool {
    !value.is_empty() && !value.starts_with('/') && !value.contains("..") && !value.contains(":\\")
}

fn valid_task_id(value: &str) -> bool {
    let parts = value.split('-').collect::<Vec<_>>();
    parts.len() == 2
        && parts[0] == "issue"
        && parts[1].len() == 4
        && parts[1].chars().all(|c| c.is_ascii_digit())
}

fn valid_version(value: &str) -> bool {
    let Some(rest) = value.strip_prefix('v') else {
        return false;
    };
    rest.split('.')
        .all(|part| !part.is_empty() && part.chars().all(|c| c.is_ascii_digit()))
}

fn valid_branch(value: &str) -> bool {
    value.starts_with("codex/")
        && value[6..]
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
}

fn valid_github_issue_url(value: &str) -> bool {
    let parts = value.split('/').collect::<Vec<_>>();
    parts.len() >= 7
        && parts[0] == "https:"
        && parts[2] == "github.com"
        && parts[5] == "issues"
        && parts[6].chars().all(|c| c.is_ascii_digit())
}

fn valid_github_pr_url(value: &str) -> bool {
    let parts = value.split('/').collect::<Vec<_>>();
    parts.len() >= 7
        && parts[0] == "https:"
        && parts[2] == "github.com"
        && parts[5] == "pull"
        && parts[6].chars().all(|c| c.is_ascii_digit())
}

fn valid_reference(value: &str) -> bool {
    value.starts_with("http://")
        || value.starts_with("https://")
        || value
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '/' | '-'))
}

fn valid_iso8601_datetime(value: &str) -> bool {
    chrono::DateTime::parse_from_rfc3339(value)
        .map(|dt| dt.to_rfc3339_opts(chrono::SecondsFormat::Secs, true) == value)
        .unwrap_or(false)
}

fn is_normalized_slug(value: &str) -> bool {
    !value.is_empty()
        && value
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
        && !value.starts_with('-')
        && !value.ends_with('-')
        && !value.contains("--")
}

fn is_repo_review_finding_title(line: &str) -> bool {
    let trimmed = line.trim_start();
    let Some((num, rest)) = trimmed.split_once(". ") else {
        return false;
    };
    if !num.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    matches!(
        rest.get(0..4),
        Some("[P1]") | Some("[P2]") | Some("[P3]") | Some("[P4]") | Some("[P5]")
    )
}

fn repo_review_finding_sort_key(line: &str) -> (u8, String) {
    let trimmed = line.trim_start();
    let sev = trimmed
        .split("[P")
        .nth(1)
        .and_then(|rest| rest.chars().next())
        .and_then(|c| c.to_digit(10))
        .unwrap_or(9) as u8;
    (sev, trimmed.to_string())
}

fn mapping_contains(mapping: &Mapping, key: &str) -> bool {
    mapping.contains_key(Value::String(key.to_string()))
}

fn mapping_string(mapping: &Mapping, key: &str) -> Option<String> {
    mapping
        .get(Value::String(key.to_string()))
        .and_then(|value| match value {
            Value::String(v) => Some(v.clone()),
            Value::Number(v) => Some(v.to_string()),
            _ => None,
        })
}

fn mapping_bool(mapping: &Mapping, key: &str) -> Option<bool> {
    mapping
        .get(Value::String(key.to_string()))
        .and_then(Value::as_bool)
}

fn mapping_mapping<'a>(mapping: &'a Mapping, key: &str) -> Result<&'a Mapping> {
    mapping
        .get(Value::String(key.to_string()))
        .and_then(Value::as_mapping)
        .ok_or_else(|| anyhow!("missing required key: {key}"))
}

fn mapping_seq_len(mapping: &Mapping, key: &str) -> usize {
    mapping
        .get(Value::String(key.to_string()))
        .and_then(Value::as_sequence)
        .map(Vec::len)
        .unwrap_or(0)
}

fn ensure_bool(mapping: &Mapping, key: &str, message: &str) -> Result<bool> {
    mapping_bool(mapping, key).ok_or_else(|| anyhow!(message.to_string()))
}
