use anyhow::{anyhow, Context};
use anyhow::{bail, Result};
#[cfg(test)]
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};

use ::adl::control_plane::IssueRef;

#[cfg(test)]
#[derive(Debug, Deserialize)]
pub(crate) struct IssuePromptFrontMatter {
    pub(crate) title: String,
    pub(crate) labels: Vec<String>,
    pub(crate) issue_number: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct WorkflowQueueResolution {
    pub(crate) queue: String,
    pub(crate) source: &'static str,
}

const VALID_WORKFLOW_QUEUES: &[&str] = &[
    "wp", "tools", "runtime", "demo", "docs", "review", "release",
];

#[cfg(test)]
#[derive(Debug)]
pub(crate) struct IssuePromptDoc {
    pub(crate) front_matter: IssuePromptFrontMatter,
    pub(crate) body: String,
}

pub(crate) fn resolve_issue_scope_and_slug_from_local_state(
    repo_root: &Path,
    issue: u32,
) -> Result<Option<(String, String)>> {
    let issue_dir = format!("issue-{:04}__", issue);
    let adl_root = repo_root.join(".adl");
    if !adl_root.is_dir() {
        return Ok(None);
    }
    let mut matches = Vec::new();
    for scope_entry in fs::read_dir(&adl_root)? {
        let scope_entry = scope_entry?;
        let tasks = scope_entry.path().join("tasks");
        if !tasks.is_dir() {
            continue;
        }
        for entry in fs::read_dir(&tasks)? {
            let entry = entry?;
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if let Some(slug) = name.strip_prefix(&issue_dir) {
                matches.push((
                    scope_entry.file_name().to_string_lossy().to_string(),
                    slug.to_string(),
                ));
            }
        }
    }
    matches.sort();
    if matches.len() > 1 {
        let candidates = matches
            .iter()
            .map(|(version, slug)| format!("{version}:{slug}"))
            .collect::<Vec<_>>()
            .join(", ");
        bail!(
            "duplicate local task-bundle identities detected for issue #{}: {}",
            issue,
            candidates
        );
    }
    Ok(matches.into_iter().next())
}

pub(crate) fn normalize_issue_title_for_version(title: &str, version: &str) -> String {
    let trimmed = title.trim();
    if trimmed.is_empty() {
        return trimmed.to_string();
    }
    let expected_prefix = format!("[{version}]");
    if trimmed.starts_with(&expected_prefix) {
        return trimmed.to_string();
    }
    if let Some(rest) = trimmed.strip_prefix("[v") {
        if let Some(end) = rest.find(']') {
            return format!("{expected_prefix}{}", &rest[end + 1..]);
        }
    }
    format!("{expected_prefix}{trimmed}")
}

pub(crate) fn ensure_no_duplicate_issue_identities(
    repo_root: &Path,
    issue_ref: &IssueRef,
) -> Result<()> {
    let adl_root = repo_root.join(".adl");
    if !adl_root.is_dir() {
        return Ok(());
    }

    let body_prefix = format!("issue-{:04}-", issue_ref.issue_number());
    let task_prefix = format!("issue-{:04}__", issue_ref.issue_number());
    let canonical_body = issue_ref.issue_prompt_path(repo_root);
    let canonical_bundle = issue_ref.task_bundle_dir_path(repo_root);
    let mut duplicates = Vec::new();

    for scope_entry in fs::read_dir(&adl_root)? {
        let scope_entry = scope_entry?;
        let scope_path = scope_entry.path();

        let bodies = scope_path.join("bodies");
        if bodies.is_dir() {
            for entry in fs::read_dir(&bodies)? {
                let entry = entry?;
                let path = entry.path();
                let name = entry.file_name().to_string_lossy().to_string();
                if name.starts_with(&body_prefix) && path != canonical_body {
                    duplicates.push(path);
                }
            }
        }

        let tasks = scope_path.join("tasks");
        if tasks.is_dir() {
            for entry in fs::read_dir(&tasks)? {
                let entry = entry?;
                let path = entry.path();
                let name = entry.file_name().to_string_lossy().to_string();
                if name.starts_with(&task_prefix) && path != canonical_bundle {
                    duplicates.push(path);
                }
            }
        }
    }

    duplicates.sort();
    duplicates.dedup();
    if duplicates.is_empty() {
        return Ok(());
    }

    let rendered = duplicates
        .iter()
        .map(|path| {
            path.strip_prefix(repo_root)
                .unwrap_or(path)
                .display()
                .to_string()
        })
        .collect::<Vec<_>>()
        .join(", ");
    bail!(
        "duplicate local issue identities detected for issue #{}; keep one canonical prompt/task bundle only: {}",
        issue_ref.issue_number(),
        rendered
    );
}

pub(crate) fn render_generated_issue_prompt(
    issue: u32,
    slug: &str,
    title: &str,
    labels_csv: &str,
    issue_url: &str,
) -> String {
    let wp = infer_wp_from_title(title);
    let queue = infer_workflow_queue(title, labels_csv, Some(&wp)).unwrap_or("wp");
    let outcome_type = infer_required_outcome_type(labels_csv, title);
    let label_lines = labels_csv
        .split(',')
        .map(str::trim)
        .filter(|label| !label.is_empty())
        .map(|label| format!("  - \"{label}\""))
        .collect::<Vec<_>>()
        .join("\n");
    let body = render_generated_issue_body(title, outcome_type, Some(issue_url));

    format!(
        "---\nissue_card_schema: adl.issue.v1\nwp: \"{wp}\"\nqueue: \"{queue}\"\nslug: \"{slug}\"\ntitle: \"{title}\"\nlabels:\n{label_lines}\nissue_number: {issue}\nstatus: \"draft\"\naction: \"edit\"\ndepends_on: []\nmilestone_sprint: \"Pending sprint assignment\"\nrequired_outcome_type:\n  - \"{outcome_type}\"\nrepo_inputs: []\ncanonical_files: []\ndemo_required: false\ndemo_names: []\nissue_graph_notes:\n  - \"Bootstrap-generated from GitHub issue metadata because no canonical local issue prompt existed yet.\"\npr_start:\n  enabled: false\n  slug: \"{slug}\"\n---\n\n{body}\n"
    )
}

pub(crate) fn render_generated_issue_body(
    title: &str,
    outcome_type: &str,
    issue_url: Option<&str>,
) -> String {
    if uses_workflow_skill_bootstrap_template(title) {
        return render_workflow_skill_issue_body(title, outcome_type, issue_url);
    }

    let repo_inputs = issue_url.map(|url| format!("- {url}")).unwrap_or_else(|| {
        "- GitHub issue URL will be available after issue creation.".to_string()
    });

    format!(
        "# {title}\n\n## Summary\n\nBootstrap-generated issue body created from the requested title and labels so the issue starts with a readable, reviewable task surface instead of a placeholder stub.\n\n## Goal\n\nShip one bounded, reviewable ADL task derived from the requested title and labels using the tracked issue/task-bundle workflow.\n\n## Required Outcome\n\nThe default required outcome type for this issue is `{outcome_type}` based on the current title and labels. Adjust it only if the actual task needs a different or broader outcome set.\n\n## Deliverables\n\n- one bounded outcome matching the current issue scope\n- updated canonical docs, code, tests, or demo artifacts only where the issue actually requires them\n\n## Acceptance Criteria\n\n- the issue title and labels are reflected in the authored issue body\n- the issue body is concrete enough to review before any manual refinement pass\n- the GitHub issue body and local source prompt stay aligned with the current workflow\n\n## Repo Inputs\n\n{repo_inputs}\n\n## Dependencies\n\n- none recorded yet\n\n## Demo Expectations\n\n- No demo is required by default. Update this section only if the issue requires a proof surface.\n\n## Non-goals\n\n- changing milestone scope without recording it explicitly\n- ad-hoc local workflow drift outside the tracked issue flow\n\n## Issue-Graph Notes\n\n- This issue body was generated automatically because no canonical local issue prompt existed yet.\n- Follow-up refinement may improve specificity, but the initial issue surface should already be reviewable as written.\n\n## Notes\n\n- Generated by the ADL PR control plane from issue metadata.\n\n## Tooling Notes\n\n- This body should be concrete enough that `gh issue view` is useful immediately after creation.\n- Default next steps should follow `pr-ready` and `pr-run`, not the older `pr start` path.\n"
    )
}

fn uses_workflow_skill_bootstrap_template(title: &str) -> bool {
    let lowered = title.to_lowercase();
    lowered.contains("[tools]")
        && (lowered.contains(" skill ")
            || lowered.contains("skill ")
            || lowered.contains(" workflow ")
            || lowered.contains("workflow "))
}

fn render_workflow_skill_issue_body(
    title: &str,
    outcome_type: &str,
    issue_url: Option<&str>,
) -> String {
    let repo_inputs = issue_url
        .map(|url| {
            format!(
                "- {url}\n- adl/tools/skills\n- adl/tools/skills/docs/OPERATIONAL_SKILLS_GUIDE.md"
            )
        })
        .unwrap_or_else(|| {
            "- GitHub issue URL will be available after issue creation.\n- adl/tools/skills\n- adl/tools/skills/docs/OPERATIONAL_SKILLS_GUIDE.md".to_string()
        });

    format!(
        "# {title}\n\n## Summary\n\nBootstrap-generated workflow-skill issue body created from the requested title and labels so the issue starts with a concrete first draft instead of a generic bootstrap stub.\n\n## Goal\n\nDefine one bounded workflow-skill or tooling-surface change in the tracked PR workflow substrate and make the resulting source prompt/STP concrete enough for qualitative review before execution.\n\n## Required Outcome\n\nThe default required outcome type for this issue is `{outcome_type}` based on the current title and labels. Workflow-skill issues should also name the tracked skill, contract, docs, and validation surfaces that need to move together.\n\n## Deliverables\n\n- the targeted workflow-skill or tooling-surface change under `adl/tools/skills` or the owning control-plane code\n- matching schema or operator-doc updates when invocation, lifecycle behavior, or closeout guidance changes\n- focused validation covering the changed workflow-skill surface\n\n## Acceptance Criteria\n\n- the generated prompt identifies this as a workflow-skill/tooling issue rather than a generic bootstrap task\n- the generated first draft names likely tracked surfaces, expected validation, and lifecycle boundaries concretely enough that only bounded refinement is normally needed before readiness review\n- bootstrap output remains deterministic, reviewable, and free of placeholder drift\n\n## Repo Inputs\n\n{repo_inputs}\n\n## Dependencies\n\n- none recorded yet\n\n## Demo Expectations\n\n- No demo is required by default. Update this section only if the workflow-skill change needs a proof surface.\n\n## Non-goals\n\n- silently widening the issue into a broad workflow redesign\n- introducing ad-hoc card or lifecycle drift outside the tracked skill flow\n\n## Issue-Graph Notes\n\n- This issue body was generated automatically because no canonical local issue prompt existed yet.\n- The workflow-skill bootstrap template should still be refined if the issue needs more specific acceptance criteria, but the starting draft should already be reviewable.\n\n## Notes\n\n- Generated by the ADL PR control plane from issue metadata using the workflow-skill bootstrap template.\n\n## Tooling Notes\n\n- This body should be concrete enough that `gh issue view` is useful immediately after creation.\n- Default next steps should follow `pr-ready`, the editor skills, and `pr-run`, not the older `pr start` path.\n"
    )
}

pub(crate) fn infer_wp_from_title(title: &str) -> String {
    if let Some(start) = title.find("[WP-") {
        if let Some(end_rel) = title[start + 1..].find(']') {
            return title[start + 1..start + 1 + end_rel].to_string();
        }
    }
    "unassigned".to_string()
}

fn normalize_workflow_queue(value: &str) -> Option<String> {
    let lowered = value.trim().to_lowercase();
    VALID_WORKFLOW_QUEUES
        .iter()
        .copied()
        .find(|candidate| *candidate == lowered)
        .map(str::to_string)
}

pub(crate) fn infer_workflow_queue(
    title: &str,
    labels_csv: &str,
    wp_hint: Option<&str>,
) -> Option<&'static str> {
    if title.to_lowercase().contains("[wp-") {
        return Some("wp");
    }
    if let Some(wp) = wp_hint.and_then(normalize_workflow_queue) {
        return VALID_WORKFLOW_QUEUES
            .iter()
            .copied()
            .find(|candidate| *candidate == wp);
    }
    let lowered = format!("{} {}", labels_csv.to_lowercase(), title.to_lowercase());
    if lowered.contains("area:tools") || lowered.contains("[tools]") {
        return Some("tools");
    }
    if lowered.contains("area:demo") || lowered.contains("[demo]") {
        return Some("demo");
    }
    if lowered.contains("area:docs")
        || lowered.contains("type:docs")
        || lowered.contains("[docs]")
        || lowered.contains("type:design")
    {
        return Some("docs");
    }
    if lowered.contains("area:review") || lowered.contains("[review]") {
        return Some("review");
    }
    if lowered.contains("area:release") || lowered.contains("[release]") {
        return Some("release");
    }
    if lowered.contains("area:runtime") || lowered.contains("[runtime]") {
        return Some("runtime");
    }
    None
}

pub(crate) fn resolve_issue_prompt_workflow_queue(path: &Path) -> Result<WorkflowQueueResolution> {
    let text = fs::read_to_string(path)
        .with_context(|| format!("queue: failed to read issue prompt: {}", path.display()))?;
    let normalized = text.replace("\r\n", "\n");
    let stripped = normalized.trim().strip_prefix("---\n").ok_or_else(|| {
        anyhow!(
            "queue: missing YAML front matter opener: {}",
            path.display()
        )
    })?;
    let (front_matter, _) = stripped.split_once("\n---\n").ok_or_else(|| {
        anyhow!(
            "queue: missing YAML front matter closer: {}",
            path.display()
        )
    })?;
    let value: serde_yaml::Value = serde_yaml::from_str(front_matter).with_context(|| {
        format!(
            "queue: failed to parse YAML front matter for issue prompt: {}",
            path.display()
        )
    })?;
    let mapping = value.as_mapping().ok_or_else(|| {
        anyhow!(
            "queue: issue prompt front matter is not a mapping: {}",
            path.display()
        )
    })?;
    let queue = mapping
        .get(serde_yaml::Value::String("queue".to_string()))
        .and_then(|value| value.as_str())
        .and_then(normalize_workflow_queue);
    if let Some(queue) = queue {
        return Ok(WorkflowQueueResolution {
            queue,
            source: "explicit",
        });
    }
    let title = mapping
        .get(serde_yaml::Value::String("title".to_string()))
        .and_then(|value| value.as_str())
        .ok_or_else(|| anyhow!("queue: missing title in issue prompt: {}", path.display()))?;
    let wp = mapping
        .get(serde_yaml::Value::String("wp".to_string()))
        .and_then(|value| value.as_str());
    let labels_csv = mapping
        .get(serde_yaml::Value::String("labels".to_string()))
        .and_then(|value| value.as_sequence())
        .map(|labels| {
            labels
                .iter()
                .filter_map(|value| value.as_str())
                .collect::<Vec<_>>()
                .join(",")
        })
        .unwrap_or_default();
    let inferred = infer_workflow_queue(title, &labels_csv, wp).ok_or_else(|| {
        anyhow!(
            "queue: missing or invalid workflow queue in {} and could not infer one from title/labels",
            path.display()
        )
    })?;
    Ok(WorkflowQueueResolution {
        queue: inferred.to_string(),
        source: "inferred",
    })
}

pub(crate) fn infer_required_outcome_type(labels_csv: &str, title: &str) -> &'static str {
    let lowered = format!("{} {}", labels_csv.to_lowercase(), title.to_lowercase());
    if lowered.contains("type:docs")
        || lowered.contains("area:docs")
        || lowered.contains("[docs]")
        || lowered.contains("type:design")
    {
        return "docs";
    }
    if lowered.contains("type:test") || lowered.contains("area:tests") || lowered.contains("[test]")
    {
        return "tests";
    }
    if lowered.contains("area:demo") || lowered.contains("[demo]") {
        return "demo";
    }
    "code"
}

pub(crate) fn version_from_labels_csv(labels_csv: &str) -> Option<String> {
    labels_csv
        .split(',')
        .map(str::trim)
        .find_map(|label| label.strip_prefix("version:").map(str::to_string))
}

pub(crate) fn version_from_title(title: &str) -> Option<String> {
    let start = title.find("[v")?;
    let rest = &title[start + 1..];
    let end = rest.find(']')?;
    Some(rest[..end].to_string())
}

pub(crate) fn validate_issue_prompt_exists(path: &Path) -> Result<()> {
    if !path.is_file() {
        bail!("missing canonical source issue prompt: {}", path.display());
    }
    Ok(())
}

pub(crate) fn resolve_issue_prompt_path(repo_root: &Path, issue_ref: &IssueRef) -> Result<PathBuf> {
    let preferred = issue_ref.issue_prompt_path(repo_root);
    if preferred.is_file() {
        return Ok(preferred);
    }

    let legacy = issue_ref.legacy_issue_prompt_path(repo_root);
    if legacy.is_file() {
        return Ok(legacy);
    }

    bail!(
        "missing canonical source issue prompt: {}",
        preferred.display()
    )
}

#[cfg(test)]
pub(crate) fn load_issue_prompt(path: &Path) -> Result<IssuePromptDoc> {
    let text = fs::read_to_string(path)
        .with_context(|| format!("failed to read issue prompt '{}'", path.display()))?;
    let mut parts = text.splitn(3, "---");
    let _ = parts.next();
    let front_matter = parts
        .next()
        .ok_or_else(|| anyhow!("missing front matter in '{}'", path.display()))?;
    let body = parts
        .next()
        .ok_or_else(|| anyhow!("missing markdown body in '{}'", path.display()))?;
    let front_matter: IssuePromptFrontMatter =
        serde_yaml::from_str(front_matter).with_context(|| {
            format!(
                "failed to parse issue prompt front matter '{}'",
                path.display()
            )
        })?;
    Ok(IssuePromptDoc {
        front_matter,
        body: body.trim_start().to_string(),
    })
}

pub(crate) fn resolve_issue_body(body: Option<String>, body_file: Option<&Path>) -> Result<String> {
    if let Some(path) = body_file {
        if path == Path::new("-") {
            bail!("new: --body-file - is not supported in Rust path yet");
        }
        return fs::read_to_string(path)
            .with_context(|| format!("new: --body-file not found: {}", path.display()));
    }
    Ok(body.unwrap_or_default())
}

pub(crate) fn normalize_labels_csv(labels: &str, version: &str) -> String {
    let mut normalized = labels
        .split(',')
        .map(str::trim)
        .filter(|label| !label.is_empty() && !label.starts_with("version:"))
        .map(str::to_string)
        .collect::<Vec<_>>();
    normalized.push(format!("version:{version}"));
    normalized.join(",")
}

pub(crate) fn parse_issue_number_from_url(url: &str) -> Result<u32> {
    let issue = url
        .trim()
        .rsplit('/')
        .next()
        .ok_or_else(|| anyhow!("new: failed to parse issue number from URL: {url}"))?;
    issue
        .parse::<u32>()
        .with_context(|| format!("new: failed to parse issue number from URL: {url}"))
}
