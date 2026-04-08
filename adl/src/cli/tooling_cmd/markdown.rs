use anyhow::{ensure, Result};
use std::path::Path;

use super::common::absolutize;

pub(super) fn split_front_matter(text: &str) -> Result<(String, String)> {
    let first = text.lines().next().unwrap_or_default();
    ensure!(first.trim() == "---", "missing YAML front matter opener");
    let all = text.lines().collect::<Vec<_>>();
    let close = all
        .iter()
        .enumerate()
        .skip(1)
        .find(|(_, line)| line.trim() == "---")
        .map(|(idx, _)| idx)
        .ok_or_else(|| anyhow::anyhow!("missing YAML front matter closer"))?;
    let fm = all[1..close].join("\n");
    let body = all[close + 1..].join("\n");
    Ok((fm, body))
}

pub(super) fn markdown_has_heading(text: &str, heading: &str) -> bool {
    text.lines()
        .any(|line| line.trim_end() == format!("## {heading}"))
}

pub(super) fn markdown_headings(text: &str) -> Vec<&str> {
    text.lines()
        .filter_map(|line| line.strip_prefix("## "))
        .collect()
}

pub(super) fn markdown_section_body(text: &str, heading: &str) -> Option<String> {
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

pub(super) fn markdown_field(text: &str, key: &str) -> Option<String> {
    text.lines().find_map(|line| {
        line.strip_prefix(&format!("{key}:"))
            .map(|value| value.trim().to_string())
    })
}

pub(super) fn markdown_block_field(text: &str, block: &str, key: &str) -> Option<String> {
    let body = markdown_named_block_body(text, block)?;
    body.lines().find_map(|line| {
        line.trim_start()
            .strip_prefix(&format!("- {key}:"))
            .map(|value| value.trim().to_string())
    })
}

pub(super) fn markdown_named_block_body(text: &str, block: &str) -> Option<String> {
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

pub(super) fn trim_blank_edges(lines: Vec<String>) -> Vec<String> {
    let first = lines.iter().position(|line| !line.trim().is_empty());
    let last = lines.iter().rposition(|line| !line.trim().is_empty());
    match (first, last) {
        (Some(start), Some(end)) if start <= end => lines[start..=end].to_vec(),
        _ => Vec::new(),
    }
}

pub(super) fn section_id_to_header(id: &str) -> Option<&'static str> {
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

pub(super) fn display_card_ref(path: &Path) -> Result<String> {
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
