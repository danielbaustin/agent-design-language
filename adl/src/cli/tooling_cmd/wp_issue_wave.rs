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
    outcome: String,
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
    let sprint_map = parse_sprint_overview(version, sprint_text)?;
    let entries = parse_wbs_rows(wbs_text)?
        .into_iter()
        .filter(|row| row.wp != "WP-01" && issue_column_is_trackable(&row.issue_column))
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
    let is_closeout = is_closeout_row(&row.work_package, &row.issue_column);
    let area = infer_area(&row.work_package, is_closeout);
    let queue = infer_queue(&row.work_package, area);
    let outcome = infer_outcome(&row.work_package, area);
    let slug = format!(
        "{}-{}-{}",
        slugify(version),
        slugify(&row.wp),
        slugify(&row.work_package)
    );
    Ok(WaveEntry {
        wp: row.wp.clone(),
        issue_kind: if is_closeout {
            "closeout".to_string()
        } else {
            "execution".to_string()
        },
        title: format!("[{version}][{}] {}", row.wp, row.work_package),
        slug,
        queue: queue.to_string(),
        labels: vec![
            "track:roadmap".to_string(),
            "type:task".to_string(),
            format!("area:{area}"),
            format!("version:{version}"),
        ],
        outcome: outcome.to_string(),
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

fn issue_column_is_trackable(issue_column: &str) -> bool {
    issue_column.contains("to be seeded") || issue_column.contains('#')
}

fn is_closeout_row(work_package: &str, issue_column: &str) -> bool {
    if issue_column.contains("closeout issue to be seeded") {
        return true;
    }
    let lowered = work_package.to_lowercase();
    lowered.contains("quality")
        || lowered.contains("coverage")
        || lowered.contains("docs + review")
        || lowered == "internal review"
        || lowered.contains("3rd-party review")
        || lowered.contains("review findings remediation")
        || lowered.contains("review remediation")
        || lowered.contains("next milestone planning")
        || lowered.contains("release readiness")
        || lowered.contains("release-evidence")
        || lowered.contains("release evidence")
        || lowered.contains("release ceremony")
}

fn infer_area(work_package: &str, is_closeout: bool) -> &'static str {
    let lowered = work_package.to_lowercase();
    if is_closeout {
        if lowered.contains("release") {
            "release"
        } else if lowered.contains("docs") || lowered.contains("next milestone planning") {
            "docs"
        } else if lowered.contains("quality") || lowered.contains("coverage") {
            "quality"
        } else {
            "review"
        }
    } else if lowered.contains("issue-wave")
        || lowered.contains("issue wave")
        || lowered.contains("worktree")
    {
        "tools"
    } else if lowered.contains("demo") || lowered.contains("paper sonata") {
        "demo"
    } else if lowered.contains("docs")
        || lowered.contains("documentation")
        || lowered.contains("handoff")
        || lowered.contains("execution policy")
    {
        "docs"
    } else {
        "runtime"
    }
}

fn infer_queue(work_package: &str, area: &'static str) -> &'static str {
    let lowered = work_package.to_lowercase();
    if lowered.contains("quality") || lowered.contains("coverage") {
        "wp"
    } else {
        area
    }
}

fn infer_outcome(work_package: &str, area: &str) -> &'static str {
    let lowered = work_package.to_lowercase();
    if lowered.contains("release ceremony") {
        "release"
    } else if lowered.contains("release readiness")
        || lowered.contains("release-evidence")
        || lowered.contains("release evidence")
    {
        "docs"
    } else if lowered.contains("quality") || lowered.contains("coverage") {
        "tests"
    } else if lowered.contains("demo") {
        "demo"
    } else if lowered.contains("review") && !lowered.contains("remediation") {
        "review"
    } else if lowered.contains("docs")
        || lowered.contains("documentation")
        || lowered.contains("handoff")
        || lowered.contains("issue-wave")
        || lowered.contains("issue wave")
        || lowered.contains("execution policy")
        || area == "docs"
    {
        "docs"
    } else {
        "code"
    }
}

fn parse_wbs_rows(text: &str) -> Result<Vec<WbsRow>> {
    let mut rows = Vec::new();
    let table = extract_wbs_table(text)?;
    let header_cols = split_markdown_row(&table[0]);
    let issue_second_shape = header_cols
        .get(1)
        .is_some_and(|value| value.eq_ignore_ascii_case("Issue"));
    for line in table.into_iter().skip(2) {
        let cols = split_markdown_row(&line);
        if cols.len() != 5 && cols.len() != 6 {
            bail!(
                "unexpected WBS table shape: expected 5 or 6 columns, got {}",
                cols.len()
            );
        }
        let (work_package, description, deliverable, dependencies, issue_column) =
            if cols.len() == 6 && issue_second_shape {
                (
                    cols[2].to_string(),
                    cols[3].to_string(),
                    cols[4].to_string(),
                    cols[5].to_string(),
                    cols[1].to_string(),
                )
            } else if cols.len() == 6 {
                (
                    cols[1].to_string(),
                    cols[2].to_string(),
                    cols[3].to_string(),
                    cols[4].to_string(),
                    cols[5].to_string(),
                )
            } else {
                (
                    cols[1].to_string(),
                    cols[2].to_string(),
                    cols[3].to_string(),
                    cols[4].to_string(),
                    "issue to be seeded".to_string(),
                )
            };
        let dependency_refs = if issue_second_shape && cols.len() == 6 {
            extract_wp_refs(&dependencies)
        } else {
            extract_wp_refs(cols[4])
        };
        rows.push(WbsRow {
            wp: cols[0].to_string(),
            work_package,
            description,
            deliverable,
            dependencies: dependency_refs,
            dependency_notes: dependencies,
            issue_column,
        });
    }
    Ok(rows)
}

fn extract_wbs_table(text: &str) -> Result<Vec<String>> {
    extract_markdown_table(text, "## Work Packages")
        .or_else(|_| extract_markdown_table(text, "## Work Package Shape"))
}

fn parse_sprint_overview(version: &str, text: &str) -> Result<BTreeMap<String, (String, String)>> {
    if let Ok(table) = extract_markdown_table(text, "## Sprint Overview") {
        return parse_sprint_overview_table(table);
    }

    parse_sprint_sections(version, text)
}

fn parse_sprint_overview_table(table: Vec<String>) -> Result<BTreeMap<String, (String, String)>> {
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

fn parse_sprint_sections(version: &str, text: &str) -> Result<BTreeMap<String, (String, String)>> {
    let mut mapping = BTreeMap::new();
    let mut current: Option<(String, String)> = None;
    for raw_line in text.lines() {
        let line = raw_line.trim();
        if let Some(rest) = line.strip_prefix("## Sprint ") {
            let number = rest
                .chars()
                .take_while(|ch| ch.is_ascii_digit())
                .collect::<String>();
            if !number.is_empty() {
                current = Some((format!("{version}-s{number}"), format!("Sprint {number}")));
                continue;
            }
        }
        if line == "## Release Tail" {
            current = Some((
                format!("{version}-release-tail"),
                "Release Tail".to_string(),
            ));
            continue;
        }
        if line.starts_with("## ") {
            current = None;
            continue;
        }
        if !line.starts_with("- ") {
            continue;
        }
        let Some((sprint_id, sprint_label)) = &current else {
            continue;
        };
        for wp in extract_wp_refs(line) {
            mapping.insert(wp, (sprint_id.clone(), sprint_label.clone()));
        }
    }

    if mapping.is_empty() {
        bail!(
            "unable to find markdown table under heading '## Sprint Overview' or sprint sections"
        );
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
            vec!["WP-02", "WP-03", "WP-04", "WP-05", "WP-06", "WP-07", "WP-08"]
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

    #[test]
    fn generate_wave_doc_accepts_v0901_five_column_wbs_and_sprint_sections() {
        let wbs = include_str!("../../../../docs/milestones/v0.90.1/WBS_v0.90.1.md");
        let sprint = include_str!("../../../../docs/milestones/v0.90.1/SPRINT_v0.90.1.md");

        let wave = generate_wave_doc(
            "v0.90.1",
            wbs,
            sprint,
            "docs/milestones/v0.90.1/WBS_v0.90.1.md",
            "docs/milestones/v0.90.1/SPRINT_v0.90.1.md",
        )
        .expect("generate v0.90.1 wave");

        assert_eq!(wave.entries.len(), 19);

        let wp02 = &wave.entries[0];
        assert_eq!(wp02.wp, "WP-02");
        assert_eq!(
            wp02.title,
            "[v0.90.1][WP-02] Issue-wave template and generator alignment"
        );
        assert_eq!(wp02.queue, "tools");
        assert_eq!(wp02.outcome, "docs");
        assert_eq!(wp02.milestone_sprint, "Sprint 1");
        assert_eq!(wp02.sprint_id, "v0.90.1-s1");
        assert_eq!(wp02.dependencies, vec!["WP-01"]);

        let wp03 = &wave.entries[1];
        assert_eq!(wp03.wp, "WP-03");
        assert_eq!(wp03.queue, "tools");
        assert_eq!(wp03.outcome, "code");

        let wp04 = &wave.entries[2];
        assert_eq!(wp04.wp, "WP-04");
        assert_eq!(wp04.queue, "docs");
        assert_eq!(wp04.outcome, "docs");

        let wp12 = wave
            .entries
            .iter()
            .find(|entry| entry.wp == "WP-12")
            .expect("wp12 present");
        assert_eq!(wp12.queue, "demo");
        assert_eq!(wp12.outcome, "demo");

        let wp17 = wave
            .entries
            .iter()
            .find(|entry| entry.wp == "WP-17")
            .expect("wp17 present");
        assert_eq!(wp17.queue, "release");
        assert_eq!(wp17.outcome, "docs");
        assert_eq!(wp17.milestone_sprint, "Release Tail");
        assert_eq!(wp17.sprint_id, "v0.90.1-release-tail");

        let wp16 = wave
            .entries
            .iter()
            .find(|entry| entry.wp == "WP-16")
            .expect("wp16 present");
        assert_eq!(wp16.queue, "review");
        assert_eq!(wp16.outcome, "code");
    }
}
