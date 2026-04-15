use anyhow::{anyhow, bail, ensure, Result};
use serde_yaml::Value;
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use super::common::{
    ensure_bool, ensure_file, ensure_no_absolute_host_path, ensure_no_disallowed_content,
    is_normalized_slug, mapping_contains, mapping_mapping, mapping_seq_len, mapping_string,
    resolve_issue_or_input_arg, valid_branch, valid_github_issue_url, valid_github_pr_url,
    valid_iso8601_datetime, valid_reference, valid_task_id, valid_version, ALLOWED_OUTPUT_STATUS,
};
use super::markdown::{
    markdown_block_field, markdown_field, markdown_has_heading, markdown_section_body,
    split_front_matter,
};
use super::tooling_usage;

pub(super) fn real_lint_prompt_spec(args: &[String]) -> Result<()> {
    let input = resolve_issue_or_input_arg(args)?;
    ensure_file(&input, "input card")?;
    ensure_no_disallowed_content(&input, "input card")?;
    let text = fs::read_to_string(&input)?;
    let spec = extract_prompt_spec_yaml(&text)?;
    validate_prompt_spec(&spec)?;
    println!("PASS: Prompt Spec is valid for {}", input.display());
    Ok(())
}

pub(super) fn real_validate_structured_prompt(args: &[String]) -> Result<()> {
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
        "sip" => validate_sip_text(&text, &input, phase.as_deref())?,
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

pub(super) fn extract_prompt_spec_yaml(text: &str) -> Result<String> {
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

pub(super) fn prompt_spec_sections(spec: &str) -> Vec<String> {
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

pub(super) fn prompt_spec_bool(spec: &str, key: &str) -> Option<bool> {
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

pub(super) fn validate_prompt_spec(spec: &str) -> Result<()> {
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
        );
    }

    ensure!(
        mapping_string(mapping, "prompt_schema").as_deref() == Some("adl.v1"),
        "unsupported prompt_schema: {}",
        mapping_string(mapping, "prompt_schema").unwrap_or_else(|| "<empty>".to_string())
    );

    let inputs = mapping_mapping(mapping, "inputs")?;
    let sections = inputs
        .get(Value::String("sections".to_string()))
        .and_then(Value::as_sequence)
        .ok_or_else(|| anyhow!("Prompt Spec missing inputs.sections"))?;
    ensure!(
        !sections.is_empty(),
        "inputs.sections must include at least one section id"
    );
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
        );
    }

    let constraints = mapping_mapping(mapping, "constraints")?;
    let _ = ensure_bool(
        constraints,
        "include_system_invariants",
        "constraints.include_system_invariants must be true or false",
    )?;
    let _ = ensure_bool(
        constraints,
        "include_reviewer_checklist",
        "constraints.include_reviewer_checklist must be true or false",
    )?;
    let _ = ensure_bool(
        constraints,
        "disallow_secrets",
        "constraints.disallow_secrets must be true or false",
    )?;
    let _ = ensure_bool(
        constraints,
        "disallow_absolute_host_paths",
        "constraints.disallow_absolute_host_paths must be true or false",
    )?;

    let automation_hints = mapping_mapping(mapping, "automation_hints")?;
    let _ = ensure_bool(
        automation_hints,
        "source_issue_prompt_required",
        "automation_hints.source_issue_prompt_required must be true or false",
    )?;
    let _ = ensure_bool(
        automation_hints,
        "target_files_surfaces_recommended",
        "automation_hints.target_files_surfaces_recommended must be true or false",
    )?;
    let _ = ensure_bool(
        automation_hints,
        "validation_plan_required",
        "automation_hints.validation_plan_required must be true or false",
    )?;
    let _ = ensure_bool(
        automation_hints,
        "required_outcome_type_supported",
        "automation_hints.required_outcome_type_supported must be true or false",
    )?;

    let review_surfaces = mapping
        .get(Value::String("review_surfaces".to_string()))
        .and_then(Value::as_sequence)
        .ok_or_else(|| anyhow!("Prompt Spec missing review_surfaces"))?;
    let actual = review_surfaces
        .iter()
        .filter_map(|value| value.as_str())
        .collect::<Vec<_>>();
    ensure!(
        actual == super::common::REQUIRED_REVIEW_SURFACES,
        "review_surfaces must match canonical order exactly"
    );
    Ok(())
}

pub(super) fn validate_stp_text(text: &str) -> Result<()> {
    let (fm_text, body_text) = split_front_matter(text)?;
    let fm_yaml: Value = serde_yaml::from_str(&fm_text)?;
    let fm = fm_yaml
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
        );
    }

    ensure!(
        mapping_string(fm, "issue_card_schema").as_deref() == Some("adl.issue.v1"),
        "issue_card_schema must be one of: adl.issue.v1"
    );
    ensure!(
        !mapping_string(fm, "wp").unwrap_or_default().is_empty(),
        "missing required field: wp"
    );
    let slug = mapping_string(fm, "slug").unwrap_or_default();
    ensure!(!slug.is_empty(), "missing required field: slug");
    ensure!(is_normalized_slug(&slug), "slug must be a normalized slug");
    ensure!(
        !mapping_string(fm, "title").unwrap_or_default().is_empty(),
        "missing required field: title"
    );
    ensure!(
        mapping_seq_len(fm, "labels") >= 1,
        "labels must contain at least 1 item(s)"
    );
    ensure!(
        mapping_string(fm, "issue_number")
            .and_then(|v| v.parse::<u32>().ok())
            .is_some(),
        "issue_number must be an integer"
    );
    let status = mapping_string(fm, "status").unwrap_or_default();
    ensure!(
        ["draft", "active", "complete"].contains(&status.as_str()),
        "status must be one of: draft, active, complete"
    );
    let action = mapping_string(fm, "action").unwrap_or_default();
    ensure!(
        ["create", "edit", "close", "split", "supersede"].contains(&action.as_str()),
        "action must be one of: create, edit, close, split, supersede"
    );
    ensure!(
        mapping_contains(fm, "depends_on"),
        "missing required field: depends_on"
    );
    ensure!(
        !mapping_string(fm, "milestone_sprint")
            .unwrap_or_default()
            .is_empty(),
        "missing required field: milestone_sprint"
    );
    ensure!(
        mapping_seq_len(fm, "required_outcome_type") >= 1,
        "required_outcome_type must contain at least 1 item(s)"
    );
    ensure!(
        mapping_contains(fm, "repo_inputs"),
        "missing required field: repo_inputs"
    );
    ensure!(
        mapping_contains(fm, "canonical_files"),
        "missing required field: canonical_files"
    );
    let demo_required = super::common::mapping_bool(fm, "demo_required")
        .ok_or_else(|| anyhow!("demo_required must be true or false"))?;
    let _ = demo_required;
    ensure!(
        mapping_contains(fm, "demo_names"),
        "missing required field: demo_names"
    );
    ensure!(
        mapping_contains(fm, "issue_graph_notes"),
        "missing required field: issue_graph_notes"
    );
    let pr_start = mapping_mapping(fm, "pr_start")?;
    let _ = ensure_bool(
        pr_start,
        "enabled",
        "pr_start.enabled must be true or false",
    )?;
    let pr_start_slug = mapping_string(pr_start, "slug").unwrap_or_default();
    ensure!(
        !pr_start_slug.is_empty() && is_normalized_slug(&pr_start_slug),
        "pr_start.slug must be a normalized slug"
    );
    Ok(())
}

pub(super) fn validate_sip_text(text: &str, path: &Path, phase: Option<&str>) -> Result<()> {
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
        );
    }
    ensure!(
        valid_task_id(&markdown_field(text, "Task ID").unwrap_or_default()),
        "Task ID must match issue-0000"
    );
    ensure!(
        valid_task_id(&markdown_field(text, "Run ID").unwrap_or_default()),
        "Run ID must match issue-0000"
    );
    ensure!(
        valid_version(&markdown_field(text, "Version").unwrap_or_default()),
        "Version must match milestone version format (for example v0.85 or v0.87.1)"
    );
    ensure!(
        !markdown_field(text, "Title").unwrap_or_default().is_empty(),
        "missing required field: Title"
    );
    let branch = markdown_field(text, "Branch").unwrap_or_default();
    let branch_ok = if phase == Some("bootstrap") {
        valid_branch(&branch) || branch.eq_ignore_ascii_case("not bound yet")
    } else {
        valid_branch(&branch)
    };
    ensure!(
        branch_ok,
        "Branch must be a codex/ branch{}",
        if phase == Some("bootstrap") {
            " or `not bound yet` in bootstrap phase"
        } else {
            ""
        }
    );
    let issue = markdown_block_field(text, "Context", "Issue").unwrap_or_default();
    ensure!(
        valid_github_issue_url(&issue),
        "Context.Issue must be a GitHub issue URL"
    );
    let pr = markdown_block_field(text, "Context", "PR").unwrap_or_default();
    ensure!(
        pr.is_empty() || valid_github_pr_url(&pr),
        "Context.PR must be a GitHub PR URL"
    );
    let source = markdown_block_field(text, "Context", "Source Issue Prompt").unwrap_or_default();
    ensure!(
        !source.is_empty() && valid_reference(&source),
        "Context.Source Issue Prompt must be a repo-relative reference or URL"
    );
    ensure!(
        !markdown_block_field(text, "Context", "Docs")
            .unwrap_or_default()
            .is_empty(),
        "missing required field: Context.Docs"
    );
    ensure!(
        !markdown_block_field(text, "Context", "Other")
            .unwrap_or_default()
            .is_empty(),
        "missing required field: Context.Other"
    );
    let source_slug =
        markdown_block_field(text, "Execution", "Source issue-prompt slug").unwrap_or_default();
    ensure!(
        source_slug.is_empty() || is_normalized_slug(&source_slug),
        "Execution.Source issue-prompt slug must be a normalized slug"
    );
    let outcome =
        markdown_block_field(text, "Execution", "Required outcome type").unwrap_or_default();
    ensure!(
        outcome.is_empty()
            || ["code", "docs", "tests", "demo", "combination"].contains(&outcome.as_str()),
        "Execution.Required outcome type must be one of: code, docs, tests, demo, combination"
    );
    let demo_required =
        markdown_block_field(text, "Execution", "Demo required").unwrap_or_default();
    ensure!(
        demo_required.is_empty() || ["true", "false"].contains(&demo_required.as_str()),
        "Execution.Demo required must be true or false"
    );
    let _ = path;
    validate_prompt_spec(&extract_prompt_spec_yaml(text)?)?;
    Ok(())
}

pub(super) fn validate_sor_text(text: &str, phase: Option<&str>) -> Result<()> {
    for section in super::common::REQUIRED_OUTPUT_SECTIONS {
        ensure!(
            markdown_has_heading(text, section),
            "missing required section: {section}"
        );
    }
    ensure!(
        valid_task_id(&markdown_field(text, "Task ID").unwrap_or_default()),
        "Task ID must match issue-0000"
    );
    ensure!(
        valid_task_id(&markdown_field(text, "Run ID").unwrap_or_default()),
        "Run ID must match issue-0000"
    );
    ensure!(
        valid_version(&markdown_field(text, "Version").unwrap_or_default()),
        "Version must match milestone version format (for example v0.85 or v0.87.1)"
    );
    ensure!(
        !markdown_field(text, "Title").unwrap_or_default().is_empty(),
        "missing required field: Title"
    );
    let integration_state = markdown_block_field(
        text,
        "Main Repo Integration (REQUIRED)",
        "Integration state",
    )
    .unwrap_or_default();
    ensure!(
        integration_state.is_empty()
            || ["worktree_only", "pr_open", "merged", "closed_no_pr"]
                .contains(&integration_state.as_str()),
        "Main Repo Integration.Integration state must be one of: worktree_only, pr_open, merged, closed_no_pr"
    );
    let branch = markdown_field(text, "Branch").unwrap_or_default();
    let branch_ok = if phase == Some("bootstrap") {
        valid_branch(&branch) || branch.eq_ignore_ascii_case("not bound yet")
    } else if phase == Some("completed") && integration_state == "closed_no_pr" {
        branch.eq_ignore_ascii_case("retrospective-no-branch")
    } else {
        valid_branch(&branch)
    };
    ensure!(
        branch_ok,
        "Branch must be a codex/ branch{}",
        if phase == Some("bootstrap") {
            " or `not bound yet` in bootstrap phase"
        } else if phase == Some("completed") && integration_state == "closed_no_pr" {
            " or `retrospective-no-branch` when completed-phase Integration state is `closed_no_pr`"
        } else {
            ""
        }
    );
    let status = markdown_field(text, "Status").unwrap_or_default();
    ensure!(
        ALLOWED_OUTPUT_STATUS.contains(&status.as_str()),
        "Status must be one of: NOT_STARTED, IN_PROGRESS, DONE, FAILED"
    );
    let start = markdown_block_field(text, "Execution", "Start Time").unwrap_or_default();
    ensure!(
        start.is_empty() || valid_iso8601_datetime(&start),
        "Execution.Start Time must be UTC ISO 8601 / RFC3339 with trailing Z (YYYY-MM-DDTHH:MM:SSZ)"
    );
    let end = markdown_block_field(text, "Execution", "End Time").unwrap_or_default();
    ensure!(
        end.is_empty() || valid_iso8601_datetime(&end),
        "Execution.End Time must be UTC ISO 8601 / RFC3339 with trailing Z (YYYY-MM-DDTHH:MM:SSZ)"
    );
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
    );
    let result = markdown_block_field(text, "Main Repo Integration (REQUIRED)", "Result")
        .unwrap_or_default();
    ensure!(
        result.is_empty() || ["PASS", "FAIL"].contains(&result.as_str()),
        "Main Repo Integration.Result must be one of: PASS, FAIL"
    );

    if phase == Some("completed") {
        ensure!(
            ["DONE", "FAILED"].contains(&status.as_str()),
            "completed-phase SOR Status must be DONE or FAILED"
        );
        ensure!(
            !start.is_empty(),
            "completed-phase SOR requires Execution.Start Time"
        );
        ensure!(
            !end.is_empty(),
            "completed-phase SOR requires Execution.End Time"
        );
        ensure!(
            !markdown_section_body(text, "Summary")
                .unwrap_or_default()
                .trim()
                .is_empty(),
            "completed-phase SOR requires non-empty Summary content"
        );
        ensure!(
            !markdown_section_body(text, "Actions taken")
                .unwrap_or_default()
                .trim()
                .is_empty(),
            "completed-phase SOR requires non-empty Actions taken content"
        );
        ensure!(
            !markdown_section_body(text, "Validation")
                .unwrap_or_default()
                .trim()
                .is_empty(),
            "completed-phase SOR requires non-empty Validation content"
        );
        ensure!(
            !integration_state.is_empty(),
            "completed-phase SOR requires Main Repo Integration.Integration state"
        );
        ensure!(
            !result.is_empty(),
            "completed-phase SOR requires Main Repo Integration.Result"
        );
    }
    Ok(())
}

pub(super) fn prompt_spec_review_surfaces(text: &str) -> Vec<String> {
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
