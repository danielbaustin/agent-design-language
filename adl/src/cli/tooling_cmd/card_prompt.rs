use anyhow::{bail, Result};
use std::collections::BTreeSet;
use std::fs;
use std::path::PathBuf;

use super::common::{
    ensure_file, ensure_no_disallowed_content, normalize_issue, resolve_input_card_path,
};
use super::markdown::{
    display_card_ref, markdown_field, markdown_section_body, section_id_to_header, trim_blank_edges,
};
use super::structured_prompt::{extract_prompt_spec_yaml, prompt_spec_bool, prompt_spec_sections};
use super::tooling_usage;

pub(super) fn real_card_prompt(args: &[String]) -> Result<()> {
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
