use anyhow::{anyhow, bail, Context, Result};
use serde::Serialize;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use super::common::{repo_relative_display, repo_root};

#[derive(Debug, Serialize, PartialEq, Eq)]
struct WaveDoc {
    schema: &'static str,
    version: String,
    sources: WaveSources,
    entries: Vec<WaveEntry>,
}

#[derive(Debug, Serialize, PartialEq, Eq)]
struct WaveSources {
    wbs: String,
    sprint: String,
}

#[derive(Debug, Serialize, PartialEq, Eq)]
struct WaveEntry {
    wp: String,
    issue_kind: String,
    title: String,
    slug: String,
    queue: String,
    labels: Vec<String>,
    milestone_sprint: String,
    sprint_id: String,
    dependencies: Vec<String>,
    dependency_notes: String,
    work_package: String,
    summary: String,
    deliverable: String,
    issue_column: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct WbsRow {
    wp: String,
    work_package: String,
    description: String,
    deliverable: String,
    dependencies: Vec<String>,
    dependency_notes: String,
    issue_column: String,
}

pub(crate) fn real_generate_wp_issue_wave(args: &[String]) -> Result<()> {
    if args
        .iter()
        .any(|arg| matches!(arg.as_str(), "--help" | "-h"))
    {
        println!(
            "adl tooling generate-wp-issue-wave --version <version> [--wbs <path>] [--sprint <path>] [--out <path>]"
        );
        return Ok(());
    }

    let parsed = parse_args(args)?;
    let repo = repo_root()?;
    let default_wbs = repo.join(format!("docs/milestones/{0}/WBS_{0}.md", parsed.version));
    let default_sprint = repo.join(format!("docs/milestones/{0}/SPRINT_{0}.md", parsed.version));
    let wbs_path = absolutize_from_repo(&repo, parsed.wbs.unwrap_or(default_wbs));
    let sprint_path = absolutize_from_repo(&repo, parsed.sprint.unwrap_or(default_sprint));

    let wbs_text = fs::read_to_string(&wbs_path)
        .with_context(|| format!("read WBS file: {}", wbs_path.display()))?;
    let sprint_text = fs::read_to_string(&sprint_path)
        .with_context(|| format!("read sprint file: {}", sprint_path.display()))?;

    let wave = generate_wave_doc(
        &parsed.version,
        &wbs_text,
        &sprint_text,
        &repo_relative_display(&repo, &wbs_path)?,
        &repo_relative_display(&repo, &sprint_path)?,
    )?;
    let rendered = serde_yaml::to_string(&wave)?;

    if let Some(out) = parsed.out {
        let out = absolutize_from_repo(&repo, out);
        if let Some(parent) = out.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&out, rendered)?;
    } else {
        print!("{rendered}");
    }

    Ok(())
}

#[derive(Debug, Default)]
struct GenerateArgs {
    version: String,
    wbs: Option<PathBuf>,
    sprint: Option<PathBuf>,
    out: Option<PathBuf>,
}

fn parse_args(args: &[String]) -> Result<GenerateArgs> {
    let mut parsed = GenerateArgs::default();
    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--version" => {
                parsed.version = require_value(args, i, "--version")?;
                i += 1;
            }
            "--wbs" => {
                parsed.wbs = Some(PathBuf::from(require_value(args, i, "--wbs")?));
                i += 1;
            }
            "--sprint" => {
                parsed.sprint = Some(PathBuf::from(require_value(args, i, "--sprint")?));
                i += 1;
            }
            "--out" => {
                parsed.out = Some(PathBuf::from(require_value(args, i, "--out")?));
                i += 1;
            }
            other => bail!("generate-wp-issue-wave: unknown arg: {other}"),
        }
        i += 1;
    }

    if parsed.version.trim().is_empty() {
        bail!("generate-wp-issue-wave: --version is required");
    }

    Ok(parsed)
}

fn require_value(args: &[String], index: usize, flag: &str) -> Result<String> {
    args.get(index + 1)
        .cloned()
        .ok_or_else(|| anyhow!("generate-wp-issue-wave: missing value for {flag}"))
}

fn absolutize_from_repo(repo_root: &Path, path: PathBuf) -> PathBuf {
    if path.is_absolute() {
        path
    } else {
        repo_root.join(path)
    }
}

fn generate_wave_doc(
    version: &str,
    wbs_text: &str,
    sprint_text: &str,
    wbs_rel: &str,
    sprint_rel: &str,
) -> Result<WaveDoc> {
    let sprint_map = parse_sprint_overview(sprint_text)?;
    let entries = parse_wbs_rows(wbs_text)?
        .into_iter()
        .filter(|row| row.issue_column.contains("to be seeded"))
        .map(|row| build_entry(version, &row, &sprint_map))
        .collect::<Result<Vec<_>>>()?;

    Ok(WaveDoc {
        schema: "adl.wp_issue_wave.v1",
        version: version.to_string(),
        sources: WaveSources {
            wbs: wbs_rel.to_string(),
            sprint: sprint_rel.to_string(),
        },
        entries,
    })
}

fn build_entry(
    version: &str,
    row: &WbsRow,
    sprint_map: &BTreeMap<String, (String, String)>,
) -> Result<WaveEntry> {
    let Some((sprint_id, sprint_label)) = sprint_map.get(&row.wp) else {
        bail!("no sprint mapping found for {}", row.wp);
    };
    let area = infer_area(&row.work_package, &row.issue_column);
    let slug = format!(
        "{}-{}-{}",
        slugify(version),
        slugify(&row.wp),
        slugify(&row.work_package)
    );
    Ok(WaveEntry {
        wp: row.wp.clone(),
        issue_kind: if row.issue_column.contains("closeout issue to be seeded") {
            "closeout".to_string()
        } else {
            "execution".to_string()
        },
        title: format!("[{version}][{}] {}", row.wp, row.work_package),
        slug,
        queue: "wp".to_string(),
        labels: vec![
            "track:roadmap".to_string(),
            "type:task".to_string(),
            format!("area:{area}"),
            format!("version:{version}"),
        ],
        milestone_sprint: sprint_label.clone(),
        sprint_id: sprint_id.clone(),
        dependencies: row.dependencies.clone(),
        dependency_notes: row.dependency_notes.clone(),
        work_package: row.work_package.clone(),
        summary: row.description.clone(),
        deliverable: row.deliverable.clone(),
        issue_column: row.issue_column.clone(),
    })
}

fn infer_area(work_package: &str, issue_column: &str) -> &'static str {
    let lowered = work_package.to_lowercase();
    if issue_column.contains("closeout issue to be seeded") {
        if lowered.contains("release") {
            "release"
        } else if lowered.contains("docs") || lowered.contains("next milestone planning") {
            "docs"
        } else if lowered.contains("quality") || lowered.contains("coverage") {
            "quality"
        } else {
            "review"
        }
    } else if lowered.contains("demo") || lowered.contains("paper sonata") {
        "demo"
    } else {
        "runtime"
    }
}

fn parse_wbs_rows(text: &str) -> Result<Vec<WbsRow>> {
    let mut rows = Vec::new();
    let table = extract_markdown_table(text, "## Work Packages")?;
    for line in table.into_iter().skip(2) {
        let cols = split_markdown_row(&line);
        if cols.len() != 6 {
            bail!(
                "unexpected WBS table shape: expected 6 columns, got {}",
                cols.len()
            );
        }
        rows.push(WbsRow {
            wp: cols[0].to_string(),
            work_package: cols[1].to_string(),
            description: cols[2].to_string(),
            deliverable: cols[3].to_string(),
            dependencies: extract_wp_refs(cols[4]),
            dependency_notes: cols[4].to_string(),
            issue_column: cols[5].to_string(),
        });
    }
    Ok(rows)
}

fn parse_sprint_overview(text: &str) -> Result<BTreeMap<String, (String, String)>> {
    let table = extract_markdown_table(text, "## Sprint Overview")?;
    let mut mapping = BTreeMap::new();
    for line in table.into_iter().skip(2) {
        let cols = split_markdown_row(&line);
        if cols.len() != 4 {
            bail!(
                "unexpected sprint overview table shape: expected 4 columns, got {}",
                cols.len()
            );
        }
        let sprint_id = cols[0].trim_matches('`').to_string();
        let sprint_label = sprint_display_label(&sprint_id);
        for wp in extract_wp_refs(cols[2]) {
            mapping.insert(wp, (sprint_id.clone(), sprint_label.clone()));
        }
    }
    Ok(mapping)
}

fn sprint_display_label(sprint_id: &str) -> String {
    if let Some((_, suffix)) = sprint_id.rsplit_once("-s") {
        if suffix.chars().all(|ch| ch.is_ascii_digit()) {
            return format!("Sprint {suffix}");
        }
    }
    sprint_id.to_string()
}

fn extract_markdown_table(text: &str, heading: &str) -> Result<Vec<String>> {
    let mut in_section = false;
    let mut rows = Vec::new();
    for raw_line in text.lines() {
        let line = raw_line.trim_end();
        if line.trim() == heading {
            in_section = true;
            continue;
        }
        if !in_section {
            continue;
        }
        if line.starts_with("## ") && !rows.is_empty() {
            break;
        }
        if line.trim_start().starts_with('|') {
            rows.push(line.trim().to_string());
        } else if !rows.is_empty() && !line.trim().is_empty() {
            break;
        }
    }

    if rows.len() < 3 {
        bail!("unable to find markdown table under heading '{heading}'");
    }

    Ok(rows)
}

fn split_markdown_row(line: &str) -> Vec<&str> {
    line.trim()
        .trim_matches('|')
        .split('|')
        .map(|part| part.trim())
        .collect()
}

fn extract_wp_refs(text: &str) -> Vec<String> {
    let bytes = text.as_bytes();
    let mut refs = Vec::new();
    let mut idx = 0usize;
    while idx + 5 <= bytes.len() {
        if &bytes[idx..idx + 3] == b"WP-" {
            let mut end = idx + 3;
            while end < bytes.len() && bytes[end].is_ascii_digit() {
                end += 1;
            }
            if end > idx + 3 {
                refs.push(text[idx..end].to_string());
                idx = end;
                continue;
            }
        }
        idx += 1;
    }
    if text.contains("through") && refs.len() == 2 {
        if let (Some(start), Some(end)) = (parse_wp_number(&refs[0]), parse_wp_number(&refs[1])) {
            if start <= end {
                return (start..=end)
                    .map(|value| format!("WP-{value:02}"))
                    .collect();
            }
        }
    }
    refs
}

fn parse_wp_number(value: &str) -> Option<u32> {
    value.strip_prefix("WP-")?.parse::<u32>().ok()
}

fn slugify(value: &str) -> String {
    let mut out = String::new();
    let mut prev_dash = false;
    for ch in value.chars() {
        let mapped = match ch {
            'A'..='Z' => ch.to_ascii_lowercase(),
            'a'..='z' | '0'..='9' => ch,
            _ => '-',
        };
        if mapped == '-' {
            if !out.is_empty() && !prev_dash {
                out.push('-');
            }
            prev_dash = true;
        } else {
            out.push(mapped);
            prev_dash = false;
        }
    }
    out.trim_matches('-').to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_wp_refs_handles_lists_and_ranges() {
        assert_eq!(
            extract_wp_refs("`WP-02` through `WP-08`"),
            vec!["WP-02", "WP-08"]
        );
        assert_eq!(
            extract_wp_refs("`WP-09`, `WP-10`, `WP-11`"),
            vec!["WP-09", "WP-10", "WP-11"]
        );
    }

    #[test]
    fn generate_wave_doc_from_v088_surfaces_is_deterministic_and_complete() {
        let wbs = include_str!("../../../../docs/milestones/v0.88/WBS_v0.88.md");
        let sprint = include_str!("../../../../docs/milestones/v0.88/SPRINT_v0.88.md");

        let first = generate_wave_doc(
            "v0.88",
            wbs,
            sprint,
            "docs/milestones/v0.88/WBS_v0.88.md",
            "docs/milestones/v0.88/SPRINT_v0.88.md",
        )
        .expect("generate first");
        let second = generate_wave_doc(
            "v0.88",
            wbs,
            sprint,
            "docs/milestones/v0.88/WBS_v0.88.md",
            "docs/milestones/v0.88/SPRINT_v0.88.md",
        )
        .expect("generate second");

        assert_eq!(first, second);
        assert_eq!(first.entries.len(), 19);
        assert_eq!(first.entries.first().unwrap().wp, "WP-02");
        assert_eq!(
            first.entries.first().unwrap().title,
            "[v0.88][WP-02] Chronosense foundation"
        );
        assert_eq!(
            first.entries.first().unwrap().labels,
            vec![
                "track:roadmap",
                "type:task",
                "area:runtime",
                "version:v0.88"
            ]
        );
        assert_eq!(first.entries.first().unwrap().milestone_sprint, "Sprint 1");
        assert_eq!(first.entries.first().unwrap().dependencies, vec!["WP-01"]);
        assert_eq!(first.entries[10].wp, "WP-12");
        assert_eq!(first.entries[10].labels[2], "area:demo");
        assert_eq!(first.entries.last().unwrap().wp, "WP-20");
        assert_eq!(first.entries.last().unwrap().issue_kind, "closeout");
        assert_eq!(first.entries.last().unwrap().labels[2], "area:release");
    }
}
