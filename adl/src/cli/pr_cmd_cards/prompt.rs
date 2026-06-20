use anyhow::Result;
use serde_yaml::Value;
use std::fs;
use std::path::Path;

use super::super::pr_cmd::{gh_issue_body, gh_issue_label_names};
use super::super::pr_cmd_prompt::{
    infer_initial_pvf_lane, infer_initial_pvf_lane_source, infer_required_outcome_type,
    infer_workflow_queue, infer_wp_from_title, normalize_labels_csv, render_generated_issue_prompt,
};
use super::super::pr_cmd_validate::{bootstrap_stub_reason, PromptSurfaceKind};
use super::shared::{default_repo, write_temp_markdown};
use super::validation::validate_bootstrap_stp;
use ::adl::control_plane::IssueRef;

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
        let fetched = gh_issue_label_names(issue_ref.issue_number(), repo)?
            .into_iter()
            .map(|label| label.trim().to_string())
            .filter(|label| !label.is_empty())
            .collect::<Vec<_>>()
            .join(",");
        let baseline = if fetched.trim().is_empty() {
            default_new_labels.to_string()
        } else {
            fetched
        };
        normalize_labels_csv(&baseline, version)
    };

    let issue_url = format!(
        "https://github.com/{repo}/issues/{}",
        issue_ref.issue_number()
    );
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
    let normalized_body = ensure_issue_body_has_notes_section(body);

    let wp = infer_wp_from_title(title);
    let queue = infer_workflow_queue(title, labels_csv, Some(&wp)).unwrap_or("wp");
    let outcome_type = infer_required_outcome_type(labels_csv, title);
    let initial_pvf_lane = infer_initial_pvf_lane(title, labels_csv, Some(body));
    let initial_pvf_lane_source =
        infer_initial_pvf_lane_source(title, labels_csv, Some(body), initial_pvf_lane);
    let label_lines = labels_csv
        .split(',')
        .map(str::trim)
        .filter(|label| !label.is_empty())
        .map(|label| format!("  - \"{label}\""))
        .collect::<Vec<_>>()
        .join("\n");
    format!(
        "---\nissue_card_schema: adl.issue.v1\nwp: \"{wp}\"\nqueue: \"{queue}\"\nslug: \"{slug}\"\ntitle: \"{title}\"\nlabels:\n{label_lines}\nissue_number: {issue}\nstatus: \"draft\"\naction: \"edit\"\ndepends_on: []\nmilestone_sprint: \"Pending sprint assignment\"\nrequired_outcome_type:\n  - \"{outcome_type}\"\nrepo_inputs: []\ncanonical_files: []\ndemo_required: false\ndemo_names: []\nissue_graph_notes:\n  - \"Mirrored from the authored GitHub issue body during bootstrap/init.\"\ninitial_pvf_lane: \"{initial_pvf_lane}\"\ninitial_pvf_lane_source: \"{initial_pvf_lane_source}\"\npr_start:\n  enabled: false\n  slug: \"{slug}\"\n---\n\n{normalized_body}\n"
    )
}

fn ensure_issue_body_has_notes_section(body: &str) -> String {
    let normalized = body.replace("\r\n", "\n").trim_end().to_string();
    if normalized.lines().any(|line| line.trim_end() == "## Notes") {
        return normalized;
    }
    format!("{normalized}\n\n## Notes\n\n- No additional notes.")
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
    let title = mapping
        .get(Value::String("title".to_string()))
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string();
    let labels_csv = mapping
        .get(Value::String("labels".to_string()))
        .and_then(Value::as_sequence)
        .map(|labels| {
            labels
                .iter()
                .filter_map(Value::as_str)
                .collect::<Vec<_>>()
                .join(",")
        })
        .unwrap_or_default();
    if !mapping.contains_key(Value::String("initial_pvf_lane".to_string())) {
        let lane = infer_initial_pvf_lane(&title, &labels_csv, Some(markdown_body));
        mapping.insert(
            Value::String("initial_pvf_lane".to_string()),
            serde_yaml::to_value(lane).ok()?,
        );
    }
    if !mapping.contains_key(Value::String("initial_pvf_lane_source".to_string())) {
        let lane = mapping
            .get(Value::String("initial_pvf_lane".to_string()))
            .and_then(Value::as_str)
            .unwrap_or(super::super::pr_cmd_prompt::NEEDS_PLANNING_PVF_LANE);
        mapping.insert(
            Value::String("initial_pvf_lane_source".to_string()),
            serde_yaml::to_value(infer_initial_pvf_lane_source(
                &title,
                &labels_csv,
                Some(markdown_body),
                lane,
            ))
            .ok()?,
        );
    }

    let front_matter = serde_yaml::to_string(&value).ok()?;
    Some(format!(
        "---\n{front_matter}---\n\n{}\n",
        markdown_body.trim_start()
    ))
}

fn fetch_issue_body(repo: &str, issue: u32) -> Result<Option<String>> {
    gh_issue_body(issue, repo)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::tests::env_lock;
    use std::path::{Path, PathBuf};

    fn unique_temp_dir(name: &str) -> PathBuf {
        let path = std::env::temp_dir().join(format!(
            "{name}-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("clock")
                .as_nanos()
        ));
        fs::create_dir_all(&path).expect("temp dir");
        path
    }

    fn write_executable(path: &Path, body: &str) {
        let body = if path.file_name().and_then(|name| name.to_str()) == Some("gh")
            && !body.contains("ADL_GITHUB_TEST_FIXTURE")
        {
            body.replacen(
                "#!/usr/bin/env bash\n",
                "#!/usr/bin/env bash\n# ADL_GITHUB_TEST_FIXTURE\n",
                1,
            )
        } else {
            body.to_string()
        };
        fs::write(path, body).expect("script");
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(path).expect("metadata").permissions();
            perms.set_mode(0o755);
            fs::set_permissions(path, perms).expect("chmod");
        }
    }

    fn restore_env(key: &str, value: Option<String>) {
        unsafe {
            if let Some(value) = value {
                std::env::set_var(key, value);
            } else {
                std::env::remove_var(key);
            }
        }
    }

    #[test]
    fn fetch_issue_body_respects_github_fallback_policy() {
        let _guard = env_lock();
        let temp = unique_temp_dir("adl-fetch-issue-body-policy");
        let bin_dir = temp.join("bin");
        fs::create_dir_all(&bin_dir).expect("bin dir");
        let gh_log = temp.join("gh.log");
        write_executable(
            &bin_dir.join("gh"),
            &format!(
                "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\nprintf 'Issue body from gh\\n'\n",
                gh_log.display()
            ),
        );

        let old_path = std::env::var("PATH").ok();
        let old_client = std::env::var("ADL_GITHUB_CLIENT").ok();
        let old_disable = std::env::var("ADL_GITHUB_DISABLE_GH_FALLBACK").ok();
        let old_github_token = std::env::var("GITHUB_TOKEN").ok();
        let old_gh_token = std::env::var("GH_TOKEN").ok();
        let old_token_file = std::env::var("ADL_GITHUB_TOKEN_FILE").ok();
        let old_keychain_service = std::env::var("ADL_GITHUB_TOKEN_KEYCHAIN_SERVICE").ok();
        let old_keychain_account = std::env::var("ADL_GITHUB_TOKEN_KEYCHAIN_ACCOUNT").ok();
        let mut path_entries = vec![bin_dir.clone()];
        path_entries.extend(std::env::split_paths(old_path.as_deref().unwrap_or("")));
        unsafe {
            std::env::set_var(
                "PATH",
                std::env::join_paths(path_entries).expect("join PATH"),
            );
            std::env::remove_var("ADL_GITHUB_CLIENT");
            std::env::remove_var("ADL_GITHUB_DISABLE_GH_FALLBACK");
            std::env::remove_var("GITHUB_TOKEN");
            std::env::remove_var("GH_TOKEN");
            std::env::remove_var("ADL_GITHUB_TOKEN_FILE");
            std::env::remove_var("ADL_GITHUB_TOKEN_KEYCHAIN_SERVICE");
            std::env::remove_var("ADL_GITHUB_TOKEN_KEYCHAIN_ACCOUNT");
        }

        assert_eq!(
            fetch_issue_body("owner/repo", 3672).expect("fetch body"),
            Some("Issue body from gh".to_string())
        );
        fs::remove_file(&gh_log).expect("clear gh log");

        unsafe {
            std::env::set_var("ADL_GITHUB_DISABLE_GH_FALLBACK", "1");
        }
        let err = fetch_issue_body("owner/repo", 3672)
            .expect_err("fallback-disabled body fetch should fail closed");
        let err_debug = format!("{err:?}");
        assert!(err_debug.contains("issue.view.body"));
        assert!(err_debug.contains("github_client.fallback_disabled"));
        assert!(
            !gh_log.exists(),
            "policy guard should reject before spawning gh"
        );

        restore_env("PATH", old_path);
        restore_env("ADL_GITHUB_CLIENT", old_client);
        restore_env("ADL_GITHUB_DISABLE_GH_FALLBACK", old_disable);
        restore_env("GITHUB_TOKEN", old_github_token);
        restore_env("GH_TOKEN", old_gh_token);
        restore_env("ADL_GITHUB_TOKEN_FILE", old_token_file);
        restore_env("ADL_GITHUB_TOKEN_KEYCHAIN_SERVICE", old_keychain_service);
        restore_env("ADL_GITHUB_TOKEN_KEYCHAIN_ACCOUNT", old_keychain_account);
    }

    #[test]
    fn render_issue_prompt_from_authored_front_matter_infers_missing_pvf_lane_fields() {
        let body = r#"---
issue_card_schema: adl.issue.v1
wp: "WP-01"
queue: "wp"
slug: "v0-91-6-tools-example"
title: "[v0.91.6][tools] Prompt-template lane"
labels:
  - "track:roadmap"
  - "area:tools"
status: "draft"
action: "edit"
depends_on: []
milestone_sprint: "Pending sprint assignment"
required_outcome_type:
  - "code"
repo_inputs:
  - "docs/templates/prompts/1.0.0/spp.md"
canonical_files: []
demo_required: false
demo_names: []
issue_graph_notes:
  - "Authored front matter fixture"
pr_start:
  enabled: false
  slug: "v0-91-6-tools-example"
---

## Summary

Authored body.
"#;

        let rendered = render_issue_prompt_from_body(
            4277,
            "v0-91-6-tools-example",
            "[v0.91.6][tools] Prompt-template lane",
            "track:roadmap,area:tools",
            "https://github.com/example/repo/issues/4277",
            body,
        );

        assert!(rendered.contains("issue_number: 4277"));
        assert!(rendered.contains("initial_pvf_lane: prompt_template"));
        assert!(rendered.contains("initial_pvf_lane_source: title_labels_inference"));
    }

    #[test]
    fn render_issue_prompt_from_authored_front_matter_records_body_assisted_pvf_lane_source() {
        let body = r#"---
issue_card_schema: adl.issue.v1
wp: "WP-01"
queue: "wp"
slug: "v0-91-6-tools-example"
title: "[v0.91.6][tools] Generic lane title"
labels:
  - "track:roadmap"
  - "area:tools"
issue_number: 1
status: "draft"
action: "edit"
depends_on: []
milestone_sprint: "Pending sprint assignment"
required_outcome_type:
  - "code"
repo_inputs: []
canonical_files: []
demo_required: false
demo_names: []
issue_graph_notes:
  - "Authored front matter fixture"
pr_start:
  enabled: false
  slug: "v0-91-6-tools-example"
---

## Repo Inputs

- docs/templates/prompts/1.0.0/spp.md

## Summary

Authored body.
"#;

        let rendered = render_issue_prompt_from_body(
            4277,
            "v0-91-6-tools-example",
            "[v0.91.6][tools] Generic lane title",
            "track:roadmap,area:tools",
            "https://github.com/example/repo/issues/4277",
            body,
        );

        assert!(rendered.contains("initial_pvf_lane: prompt_template"));
        assert!(rendered.contains("initial_pvf_lane_source: title_labels_and_body_inference"));
    }

    #[test]
    fn render_issue_prompt_from_authored_front_matter_preserves_explicit_pvf_lane_fields() {
        let body = r#"---
issue_card_schema: adl.issue.v1
wp: "WP-01"
queue: "wp"
slug: "v0-91-6-tools-example"
title: "[v0.91.6][tools] Explicit PVF lane"
labels:
  - "track:roadmap"
  - "area:tools"
issue_number: 1
initial_pvf_lane: "runtime"
initial_pvf_lane_source: "manual_override"
status: "draft"
action: "edit"
depends_on: []
milestone_sprint: "Pending sprint assignment"
required_outcome_type:
  - "code"
repo_inputs: []
canonical_files: []
demo_required: false
demo_names: []
issue_graph_notes:
  - "Authored front matter fixture"
pr_start:
  enabled: false
  slug: "v0-91-6-tools-example"
---

## Summary

Authored body.
"#;

        let rendered = render_issue_prompt_from_body(
            4277,
            "v0-91-6-tools-example",
            "[v0.91.6][tools] Explicit PVF lane",
            "track:roadmap,area:tools",
            "https://github.com/example/repo/issues/4277",
            body,
        );

        assert!(rendered.contains("issue_number: 4277"));
        assert!(rendered.contains("initial_pvf_lane: runtime"));
        assert!(rendered.contains("initial_pvf_lane_source: manual_override"));
    }
}
