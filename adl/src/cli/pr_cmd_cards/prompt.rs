use anyhow::Result;
use serde_yaml::Value;
use std::fs;
use std::path::Path;

use ::adl::control_plane::IssueRef;
use super::shared::{default_repo, run_capture_allow_failure, write_temp_markdown};
use super::super::pr_cmd_prompt::{
    infer_required_outcome_type, infer_workflow_queue, infer_wp_from_title, normalize_labels_csv,
    render_generated_issue_prompt,
};
use super::super::pr_cmd_validate::{bootstrap_stub_reason, PromptSurfaceKind};
use super::validation::validate_bootstrap_stp;

pub(crate) fn write_source_issue_prompt(
    repo_root: &Path,
    issue_ref: &IssueRef,
    title: &str,
    labels_csv: &str,
    issue_url: &str,
    body: &str,
) -> Result<std::path::PathBuf> {
    let source_path = issue_ref.issue_prompt_path(repo_root);
    if let Some(parent) = source_path.parent() {
        fs::create_dir_all(parent)?;
    }
    let prompt = render_issue_prompt_from_body(
        issue_ref.issue_number(),
        issue_ref.slug(),
        title,
        labels_csv,
        issue_url,
        body,
    );
    fs::write(&source_path, prompt)?;
    Ok(source_path)
}

pub(crate) fn ensure_source_issue_prompt(
    repo_root: &Path,
    repo: &str,
    issue_ref: &IssueRef,
    title: &str,
    labels_csv: Option<&str>,
    version: &str,
    default_new_labels: &str,
) -> Result<std::path::PathBuf> {
    let source_path = issue_ref.issue_prompt_path(repo_root);
    let labels_csv = if let Some(labels) = labels_csv {
        normalize_labels_csv(labels, version)
    } else {
        let fetched = run_capture_allow_failure(
            "gh",
            &[
                "issue",
                "view",
                &issue_ref.issue_number().to_string(),
                "-R",
                repo,
                "--json",
                "labels",
                "-q",
                ".labels[].name",
            ],
        )?
        .unwrap_or_default()
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join(",");
        let baseline = if fetched.trim().is_empty() {
            default_new_labels.to_string()
        } else {
            fetched
        };
        normalize_labels_csv(&baseline, version)
    };

    let issue_url = format!("https://github.com/{repo}/issues/{}", issue_ref.issue_number());
    let generated_prompt = render_generated_issue_prompt(
        issue_ref.issue_number(),
        issue_ref.slug(),
        title,
        &labels_csv,
        &issue_url,
    );

    if source_path.is_file() {
        let existing = fs::read_to_string(&source_path)?;
        if existing != generated_prompt {
            return Ok(source_path);
        }
    }

    if let Some(body) = fetch_issue_body(repo, issue_ref.issue_number())? {
        let prompt = render_issue_prompt_from_body(
            issue_ref.issue_number(),
            issue_ref.slug(),
            title,
            &labels_csv,
            &issue_url,
            &body,
        );
        if bootstrap_stub_reason(&prompt, PromptSurfaceKind::IssuePrompt).is_none() {
            if let Some(parent) = source_path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(&source_path, prompt)?;
            return Ok(source_path);
        }
    }

    if source_path.is_file() {
        return Ok(source_path);
    }

    if let Some(parent) = source_path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&source_path, generated_prompt)?;
    Ok(source_path)
}

pub(crate) fn validate_issue_body_for_create(
    repo_root: &Path,
    title: &str,
    labels_csv: &str,
    slug: &str,
    body: &str,
) -> Result<()> {
    let init_template =
        "docs/templates/PR_INIT_INVOCATION_TEMPLATE.md#canonical-authored-issue-body-scaffold or an authored issue body file";
    let probe_issue = 999_999;
    let probe_url = format!(
        "https://github.com/{}/issues/{probe_issue}",
        default_repo(repo_root)?
    );
    let prompt =
        render_issue_prompt_from_body(probe_issue, slug, title, labels_csv, &probe_url, body);
    let temp = write_temp_markdown("issue_body_probe", &prompt)?;
    if let Err(err) = validate_bootstrap_stp(repo_root, &temp) {
        anyhow::bail!(
            "create: issue body cannot satisfy source-prompt validation: {err}; provide an authored body or use {init_template}"
        );
    }
    if let Some(reason) = super::super::pr_cmd_validate::placeholder_issue_body_reason(body) {
        anyhow::bail!(
            "create: issue body is still bootstrap stub content ({reason}); provide an authored body or use {init_template}"
        );
    }
    Ok(())
}

pub(crate) fn render_issue_prompt_from_body(
    issue: u32,
    slug: &str,
    title: &str,
    labels_csv: &str,
    _issue_url: &str,
    body: &str,
) -> String {
    if let Some(prompt) = render_issue_prompt_from_authored_front_matter(issue, body) {
        return prompt;
    }

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
    format!(
        "---\nissue_card_schema: adl.issue.v1\nwp: \"{wp}\"\nqueue: \"{queue}\"\nslug: \"{slug}\"\ntitle: \"{title}\"\nlabels:\n{label_lines}\nissue_number: {issue}\nstatus: \"draft\"\naction: \"edit\"\ndepends_on: []\nmilestone_sprint: \"Pending sprint assignment\"\nrequired_outcome_type:\n  - \"{outcome_type}\"\nrepo_inputs: []\ncanonical_files: []\ndemo_required: false\ndemo_names: []\nissue_graph_notes:\n  - \"Mirrored from the authored GitHub issue body during bootstrap/init.\"\npr_start:\n  enabled: false\n  slug: \"{slug}\"\n---\n\n{body}\n"
    )
}

fn render_issue_prompt_from_authored_front_matter(issue: u32, body: &str) -> Option<String> {
    let normalized = body.replace("\r\n", "\n");
    let stripped = normalized.trim().strip_prefix("---\n")?;
    let (front_matter, markdown_body) = stripped.split_once("\n---\n")?;
    let mut value: Value = serde_yaml::from_str(front_matter).ok()?;
    let mapping = value.as_mapping_mut()?;
    if !mapping.contains_key(Value::String("issue_card_schema".to_string())) {
        return None;
    }

    mapping.insert(
        Value::String("issue_number".to_string()),
        serde_yaml::to_value(issue).ok()?,
    );

    let front_matter = serde_yaml::to_string(&value).ok()?;
    Some(format!(
        "---\n{front_matter}---\n\n{}\n",
        markdown_body.trim_start()
    ))
}

fn fetch_issue_body(repo: &str, issue: u32) -> Result<Option<String>> {
    let output = match std::process::Command::new("gh")
        .args([
            "issue",
            "view",
            &issue.to_string(),
            "-R",
            repo,
            "--json",
            "body",
            "-q",
            ".body",
        ])
        .output()
    {
        Ok(output) => output,
        Err(_) => return Ok(None),
    };
    if !output.status.success() {
        return Ok(None);
    }
    let body = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if body.is_empty() || body.eq_ignore_ascii_case("null") {
        Ok(None)
    } else {
        Ok(Some(body))
    }
}
