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
    let mut in_fence = false;
    for line in text.lines() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("```") {
            in_fence = !in_fence;
            continue;
        }
        if !in_fence && line.trim_end() == format!("## {heading}") {
            return true;
        }
    }
    false
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
            .map(|value| value.trim().trim_matches('`').to_string())
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_dir(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "adl-markdown-{name}-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("clock")
                .as_nanos()
        ));
        fs::create_dir_all(&dir).expect("create temp dir");
        dir
    }

    #[test]
    fn split_front_matter_and_heading_helpers_cover_fenced_and_missing_cases() {
        let (frontmatter, body) = split_front_matter("---\nname: demo\n---\n## Heading\nBody\n")
            .expect("front matter should split");
        assert_eq!(frontmatter, "name: demo");
        assert_eq!(body, "## Heading\nBody");

        let err = split_front_matter("name: demo\n---\nBody\n").expect_err("missing opener");
        assert!(err.to_string().contains("missing YAML front matter opener"));

        let err = split_front_matter("---\nname: demo\nBody\n").expect_err("missing closer");
        assert!(err.to_string().contains("missing YAML front matter closer"));

        let text = "## Visible\n```md\n## Hidden\n```\n## Also Visible\n";
        assert!(markdown_has_heading(text, "Visible"));
        assert!(markdown_has_heading(text, "Also Visible"));
        assert!(!markdown_has_heading(text, "Hidden"));
        assert_eq!(markdown_headings(text), vec!["Visible", "Hidden", "Also Visible"]);
    }

    #[test]
    fn section_and_block_helpers_cover_section_block_and_field_extraction() {
        let text = "\
Task ID: issue-4329
Status: IN_PROGRESS

## Summary
Line one
Line two
## Validation
- Command: run-this
Execution:
- Actor: Codex
- Branch action: refresh
- Final PVF lane: `prompt_template`
Traces:
  note
";

        assert_eq!(
            markdown_section_body(text, "Summary").as_deref(),
            Some("Line one\nLine two")
        );
        assert_eq!(markdown_section_body(text, "Missing"), None);
        assert_eq!(markdown_field(text, "Task ID").as_deref(), Some("issue-4329"));
        assert_eq!(markdown_field(text, "Status").as_deref(), Some("IN_PROGRESS"));
        assert_eq!(
            markdown_block_field(text, "Execution", "Actor").as_deref(),
            Some("Codex")
        );
        assert_eq!(
            markdown_block_field(text, "Execution", "Final PVF lane").as_deref(),
            Some("prompt_template")
        );
        assert_eq!(
            markdown_named_block_body(text, "Execution").as_deref(),
            Some("- Actor: Codex\n- Branch action: refresh\n- Final PVF lane: `prompt_template`")
        );
        assert_eq!(
            markdown_named_block_body(text, "Validation").as_deref(),
            Some("- Command: run-this\nExecution:\n- Actor: Codex\n- Branch action: refresh\n- Final PVF lane: `prompt_template`\nTraces:\n  note")
        );
    }

    #[test]
    fn utility_helpers_cover_blank_trimming_headers_and_display_refs() {
        assert_eq!(
            trim_blank_edges(vec![
                "".to_string(),
                "  ".to_string(),
                "keep".to_string(),
                "".to_string(),
            ]),
            vec!["keep".to_string()]
        );
        assert!(trim_blank_edges(vec!["".to_string(), " ".to_string()]).is_empty());

        assert_eq!(section_id_to_header("goal"), Some("Goal"));
        assert_eq!(section_id_to_header("reviewer_checklist"), Some("Reviewer Checklist (machine-readable hints)"));
        assert_eq!(section_id_to_header("missing"), None);

        let root = temp_dir("display-card-ref");
        let tracked = root.join(".adl/v0.91.6/tasks/demo/sor.md");
        fs::create_dir_all(tracked.parent().expect("tracked parent")).expect("tracked dir");
        fs::write(&tracked, "demo").expect("tracked file");
        assert_eq!(
            display_card_ref(&tracked).expect("tracked ref"),
            ".adl/v0.91.6/tasks/demo/sor.md"
        );

        let legacy = root.join("issue-1234__input__v0.3.md");
        fs::write(&legacy, "legacy").expect("legacy file");
        assert_eq!(
            display_card_ref(&legacy).expect("legacy ref"),
            "issue-1234 (legacy input card)"
        );

        let plain = root.join("notes.md");
        fs::write(&plain, "notes").expect("plain file");
        assert_eq!(display_card_ref(&plain).expect("plain ref"), "notes.md");
    }
}
