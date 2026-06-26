use anyhow::{bail, Context, Result};
use chrono::Utc;
use serde_json::Value;
use serde_yaml::Value as YamlValue;
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use super::super::pr_cmd_prompt::{
    infer_initial_pvf_lane, infer_initial_pvf_lane_source, infer_planned_pvf_lane,
    infer_planned_pvf_lane_source, NEEDS_PLANNING_PVF_LANE,
};
use super::super::pr_cmd_validate::validate_authored_prompt_surface;
use super::super::pr_cmd_validate::{bootstrap_stub_reason, PromptSurfaceKind};
use super::shared::{
    branch_indicates_unbound_state, copy_directory_contents, deduplicate_exact_line, default_repo,
    ensure_symlink, field_line_value, output_card_title_matches_slug, path_relative_to_repo,
    replace_exact_line, replace_field_line, replace_field_line_in_file,
};
use super::validation::{
    validate_bootstrap_cards, validate_bootstrap_stp, validate_initialized_cards,
    StructuredBundlePaths,
};
use ::adl::control_plane::{
    card_input_path, card_output_path, card_plan_path, card_review_policy_path, card_stp_path,
    card_validation_plan_path, resolve_cards_root, IssueRef,
};

pub(crate) fn ensure_task_bundle_stp(
    root: &Path,
    issue_ref: &IssueRef,
    source_path: &Path,
) -> Result<PathBuf> {
    let stp_path = issue_ref.task_bundle_stp_path(root);
    let stp_invalid = stp_path.is_file() && validate_bootstrap_stp(root, &stp_path).is_err();
    if stp_invalid || prompt_surface_needs_template_refresh(&stp_path)? {
        if let Some(parent) = stp_path.parent() {
            fs::create_dir_all(parent)?;
        }
        let mut text = read_prompt_template(root, "stp", &[])?;
        let timestamp = Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
        let prompt = fs::read_to_string(source_path).unwrap_or_default();
        let metadata = SourcePromptMetadata::from_prompt(&prompt);
        let source_rel = path_relative_to_repo(root, source_path);
        let title = metadata
            .title
            .clone()
            .unwrap_or_else(|| issue_ref.slug().replace('-', " "));
        let wp = metadata.wp.as_deref().unwrap_or("process").to_string();
        let required_outcome_type = metadata
            .required_outcome_type
            .first()
            .cloned()
            .unwrap_or_else(|| "code".to_string());
        let demo_required = metadata
            .demo_required
            .map(|value| value.to_string())
            .unwrap_or_else(|| "false".to_string());
        let summary = issue_prompt_section(&prompt, "Summary")
            .map(|value| one_line_summary(&value))
            .unwrap_or_else(|| format!("Issue-local task surface for {title}."));
        let goal = issue_prompt_section(&prompt, "Goal")
            .map(|value| one_line_summary(&value))
            .unwrap_or_else(|| "Refine the linked source issue prompt goal.".to_string());
        let required_outcome = issue_prompt_section(&prompt, "Required Outcome")
            .map(|value| one_line_summary(&value))
            .unwrap_or_else(|| {
                "Refine the linked source issue prompt required outcome.".to_string()
            });
        let deliverables = issue_prompt_section(&prompt, "Deliverables")
            .map(|value| one_line_summary(&value))
            .unwrap_or_else(|| "Refine source issue deliverables before execution.".to_string());
        let acceptance = issue_prompt_section(&prompt, "Acceptance Criteria")
            .map(|value| one_line_summary(&value))
            .unwrap_or_else(|| {
                "Refine source issue acceptance criteria before execution.".to_string()
            });
        let repo_inputs = issue_prompt_section(&prompt, "Repo Inputs")
            .map(|value| one_line_summary(&value))
            .unwrap_or_else(|| source_rel.clone());
        let dependencies = issue_prompt_section(&prompt, "Dependencies")
            .map(|value| one_line_summary(&value))
            .unwrap_or_else(|| "none recorded in source issue prompt".to_string());
        let target_surfaces = repo_inputs.clone();
        let validation_plan = issue_prompt_section(&prompt, "Validation Plan")
            .or_else(|| issue_prompt_section(&prompt, "Tooling Notes"))
            .map(|value| one_line_summary(&value))
            .unwrap_or_else(|| {
                "Run the smallest proving validation for the touched surface and record it in SOR."
                    .to_string()
            });
        let demo_requirements = issue_prompt_section(&prompt, "Demo Expectations")
            .map(|value| one_line_summary(&value))
            .unwrap_or_else(|| {
                "No demo required unless the source issue says otherwise.".to_string()
            });
        let non_goals = issue_prompt_section(&prompt, "Non-goals")
            .map(|value| one_line_summary(&value))
            .unwrap_or_else(|| {
                "Do not widen scope beyond the linked source issue prompt.".to_string()
            });
        let issue_graph_notes = issue_prompt_section(&prompt, "Issue-Graph Notes")
            .map(|value| one_line_summary(&value))
            .unwrap_or_else(|| {
                "Preserve issue graph truth from the linked source issue prompt.".to_string()
            });
        let notes_risks = issue_prompt_section(&prompt, "Notes")
            .map(|value| one_line_summary(&value))
            .unwrap_or_else(|| "Update this card if execution reality diverges.".to_string());
        apply_template_values(
            &mut text,
            &[
                ("<issue>", issue_ref.issue_number().to_string()),
                ("<issue_padded>", issue_ref.padded_issue_number().to_string()),
                ("<version>", issue_ref.scope().to_string()),
                ("<slug>", issue_ref.slug().to_string()),
                ("<title>", title.clone()),
                ("<branch>", "not bound yet".to_string()),
                ("<timestamp>", timestamp),
                ("<card_status>", "ready".to_string()),
                ("<source_issue_prompt>", source_rel.clone()),
                ("<wp>", wp),
                ("<required_outcome_type>", required_outcome_type),
                ("<demo_required>", demo_required),
                (
                    "<issue_graph_note>",
                    "Versioned C-SDLC prompt template applied; source issue prompt remains the design-time intent source."
                        .to_string(),
                ),
                ("<summary>", summary),
                ("<goal>", goal),
                ("<required_outcome>", required_outcome),
                ("<deliverables>", deliverables),
                ("<acceptance_criteria>", acceptance),
                ("<repo_inputs>", repo_inputs),
                ("<dependencies>", dependencies),
                ("<target_files_surfaces>", target_surfaces),
                ("<validation_plan>", validation_plan),
                ("<demo_proof_requirements>", demo_requirements),
                ("<non_goals>", non_goals),
                ("<issue_graph_notes>", issue_graph_notes),
                ("<notes_risks>", notes_risks),
                (
                    "<tooling_notes>",
                    format!(
                        "Generated from docs/templates/prompts/{}/.",
                        active_prompt_template_set_label(root)
                    ),
                ),
            ],
        );
        apply_stp_metadata_values(&mut text, &metadata, &source_rel);
        fs::write(&stp_path, text)?;
    }
    validate_bootstrap_stp(root, &stp_path)?;
    Ok(stp_path)
}

pub(crate) fn ensure_local_issue_prompt_copy(
    root: &Path,
    issue_ref: &IssueRef,
    canonical_source_path: &Path,
) -> Result<PathBuf> {
    let local_source_path = issue_ref.issue_prompt_path(root);
    if !local_source_path.is_file() {
        if let Some(parent) = local_source_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::copy(canonical_source_path, &local_source_path)?;
    }
    Ok(local_source_path)
}

fn file_exists_nonempty(path: &Path) -> bool {
    fs::metadata(path)
        .map(|metadata| metadata.is_file() && metadata.len() > 0)
        .unwrap_or(false)
}

fn plan_card_needs_design_time_refresh(path: &Path) -> Result<bool> {
    if !path.is_file() {
        return Ok(true);
    }
    let text = fs::read_to_string(path)
        .with_context(|| format!("failed to read SPP at {}", path.display()))?;
    let legacy_design_time_ready =
        text.contains("activation_state: \"design_time_ready\"") && !text.contains("card_status:");
    Ok(text.contains("Bootstrap-generated SPP")
        || text.contains("Bootstrap planning surface for this issue")
        || text.contains("Review the issue bundle and tighten the planned execution sequence.")
        || legacy_design_time_ready)
}

fn prompt_surface_needs_template_refresh(path: &Path) -> Result<bool> {
    if !path.is_file() {
        return Ok(true);
    }
    let text = fs::read_to_string(path)
        .with_context(|| format!("failed to read prompt surface at {}", path.display()))?;
    Ok(text.contains("[summary truncated]") || contains_prompt_template_placeholder(&text))
}

fn contains_prompt_template_placeholder(text: &str) -> bool {
    let mut chars = text.char_indices().peekable();
    while let Some((start, ch)) = chars.next() {
        if ch != '<' {
            continue;
        }
        let mut end = None;
        while let Some(&(idx, next)) = chars.peek() {
            chars.next();
            if next == '>' {
                end = Some(idx);
                break;
            }
        }
        let Some(end_idx) = end else {
            break;
        };
        let candidate = &text[start + 1..end_idx];
        if !candidate.is_empty()
            && candidate
                .chars()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
        {
            return true;
        }
    }
    false
}

#[derive(Default)]
struct SourcePromptMetadata {
    title: Option<String>,
    wp: Option<String>,
    labels: Vec<String>,
    depends_on: Vec<String>,
    required_outcome_type: Vec<String>,
    repo_inputs: Vec<String>,
    canonical_files: Vec<String>,
    demo_required: Option<bool>,
    initial_pvf_lane: Option<String>,
    initial_pvf_lane_source: Option<String>,
    estimate_elapsed_seconds: Option<String>,
    estimate_total_tokens: Option<String>,
    estimate_validation_seconds: Option<String>,
    issue_goal_token_budget: Option<String>,
    estimate_confidence: Option<String>,
    estimate_data_source: Option<String>,
    estimate_source_ref: Option<String>,
    goal_metrics_rollup_ref: Option<String>,
    validation_runtime_class: Option<String>,
    validation_resource_profile: Option<String>,
    validation_family: Option<String>,
    validation_size_split: Option<String>,
    expected_proof_cost: Option<String>,
    planned_validation_seconds: Option<String>,
    planned_validation_tokens: Option<String>,
}

impl SourcePromptMetadata {
    fn from_prompt(text: &str) -> Self {
        let mut out = Self::default();
        for value in yaml_front_matter_documents(text) {
            let Some(mapping) = value.as_mapping() else {
                continue;
            };
            if out.title.is_none() {
                out.title = yaml_string_field(mapping, "title");
            }
            if out.wp.is_none() {
                out.wp = yaml_string_field(mapping, "wp");
            }
            merge_strings(
                &mut out.labels,
                yaml_string_sequence_field(mapping, "labels"),
            );
            merge_strings(
                &mut out.depends_on,
                yaml_string_sequence_field(mapping, "depends_on"),
            );
            merge_strings(
                &mut out.required_outcome_type,
                yaml_string_sequence_field(mapping, "required_outcome_type"),
            );
            merge_strings(
                &mut out.repo_inputs,
                yaml_string_sequence_field(mapping, "repo_inputs"),
            );
            merge_strings(
                &mut out.canonical_files,
                yaml_string_sequence_field(mapping, "canonical_files"),
            );
            if out.demo_required.is_none() {
                out.demo_required = yaml_bool_field(mapping, "demo_required");
            }
            if out.initial_pvf_lane.is_none() {
                out.initial_pvf_lane = yaml_string_field(mapping, "initial_pvf_lane");
            }
            if out.initial_pvf_lane_source.is_none() {
                out.initial_pvf_lane_source = yaml_string_field(mapping, "initial_pvf_lane_source");
            }
            if out.estimate_elapsed_seconds.is_none() {
                out.estimate_elapsed_seconds =
                    yaml_string_field(mapping, "estimate_elapsed_seconds");
            }
            if out.estimate_total_tokens.is_none() {
                out.estimate_total_tokens = yaml_string_field(mapping, "estimate_total_tokens");
            }
            if out.estimate_validation_seconds.is_none() {
                out.estimate_validation_seconds =
                    yaml_string_field(mapping, "estimate_validation_seconds");
            }
            if out.issue_goal_token_budget.is_none() {
                out.issue_goal_token_budget = yaml_string_field(mapping, "issue_goal_token_budget");
            }
            if out.estimate_confidence.is_none() {
                out.estimate_confidence = yaml_string_field(mapping, "estimate_confidence");
            }
            if out.estimate_data_source.is_none() {
                out.estimate_data_source = yaml_string_field(mapping, "estimate_data_source");
            }
            if out.estimate_source_ref.is_none() {
                out.estimate_source_ref = yaml_string_field(mapping, "estimate_source_ref");
            }
            if out.goal_metrics_rollup_ref.is_none() {
                out.goal_metrics_rollup_ref = yaml_string_field(mapping, "goal_metrics_rollup_ref");
            }
            if out.validation_runtime_class.is_none() {
                out.validation_runtime_class =
                    yaml_string_field(mapping, "validation_runtime_class");
            }
            if out.validation_resource_profile.is_none() {
                out.validation_resource_profile =
                    yaml_string_field(mapping, "validation_resource_profile");
            }
            if out.validation_family.is_none() {
                out.validation_family = yaml_string_field(mapping, "validation_family");
            }
            if out.validation_size_split.is_none() {
                out.validation_size_split = yaml_string_field(mapping, "validation_size_split");
            }
            if out.expected_proof_cost.is_none() {
                out.expected_proof_cost = yaml_string_field(mapping, "expected_proof_cost");
            }
            if out.planned_validation_seconds.is_none() {
                out.planned_validation_seconds =
                    yaml_string_field(mapping, "planned_validation_seconds");
            }
            if out.planned_validation_tokens.is_none() {
                out.planned_validation_tokens =
                    yaml_string_field(mapping, "planned_validation_tokens");
            }
        }
        out
    }
}

fn metadata_value_or_unknown(value: &Option<String>) -> String {
    value.clone().unwrap_or_else(|| "unknown".to_string())
}

fn resolved_initial_pvf_lane(metadata: &SourcePromptMetadata, title: &str, prompt: &str) -> String {
    metadata
        .initial_pvf_lane
        .clone()
        .unwrap_or_else(|| infer_initial_pvf_lane(title, &metadata.labels.join(","), Some(prompt)))
}

fn resolved_initial_pvf_lane_source(
    metadata: &SourcePromptMetadata,
    title: &str,
    prompt: &str,
    initial_lane: &str,
) -> String {
    metadata.initial_pvf_lane_source.clone().unwrap_or_else(|| {
        infer_initial_pvf_lane_source(
            title,
            &metadata.labels.join(","),
            Some(prompt),
            initial_lane,
        )
    })
}

fn resolved_planned_pvf_lane(initial_lane: &str) -> String {
    infer_planned_pvf_lane(initial_lane)
}

fn resolved_planned_pvf_lane_source(initial_lane: &str, initial_source: &str) -> String {
    infer_planned_pvf_lane_source(initial_lane, initial_source)
}

fn apply_stp_metadata_values(text: &mut String, metadata: &SourcePromptMetadata, source_rel: &str) {
    if !metadata.labels.is_empty() {
        replace_top_level_yaml_field(
            text,
            "labels",
            &yaml_sequence_field("labels", &metadata.labels),
        );
    }
    if !metadata.depends_on.is_empty() {
        replace_top_level_yaml_field(
            text,
            "depends_on",
            &yaml_sequence_field("depends_on", &metadata.depends_on),
        );
    }
    if !metadata.required_outcome_type.is_empty() {
        replace_top_level_yaml_field(
            text,
            "required_outcome_type",
            &yaml_sequence_field("required_outcome_type", &metadata.required_outcome_type),
        );
    }
    let repo_inputs = if metadata.repo_inputs.is_empty() {
        vec![source_rel.to_string()]
    } else {
        metadata.repo_inputs.clone()
    };
    replace_top_level_yaml_field(
        text,
        "repo_inputs",
        &yaml_sequence_field("repo_inputs", &repo_inputs),
    );
    if !metadata.canonical_files.is_empty() {
        replace_top_level_yaml_field(
            text,
            "canonical_files",
            &yaml_sequence_field("canonical_files", &metadata.canonical_files),
        );
    }
}

fn yaml_front_matter_documents(text: &str) -> Vec<YamlValue> {
    let normalized = text.replace("\r\n", "\n");
    let mut docs = Vec::new();
    let mut rest = normalized.as_str();
    while let Some(start) = rest.find("---\n") {
        let after_start = &rest[start + 4..];
        let Some(end) = after_start.find("\n---\n") else {
            break;
        };
        let front_matter = &after_start[..end];
        if let Ok(value) = serde_yaml::from_str(front_matter) {
            docs.push(value);
        }
        rest = &after_start[end + 5..];
    }
    docs
}

fn yaml_string_field(mapping: &serde_yaml::Mapping, key: &str) -> Option<String> {
    mapping
        .get(YamlValue::String(key.to_string()))
        .and_then(yaml_scalar_to_string)
}

fn yaml_scalar_to_string(value: &YamlValue) -> Option<String> {
    match value {
        YamlValue::String(value) => Some(value.clone()),
        YamlValue::Number(value) => Some(value.to_string()),
        YamlValue::Bool(value) => Some(value.to_string()),
        _ => None,
    }
}

fn yaml_bool_field(mapping: &serde_yaml::Mapping, key: &str) -> Option<bool> {
    mapping
        .get(YamlValue::String(key.to_string()))
        .and_then(YamlValue::as_bool)
}

fn yaml_string_sequence_field(mapping: &serde_yaml::Mapping, key: &str) -> Vec<String> {
    let Some(value) = mapping.get(YamlValue::String(key.to_string())) else {
        return Vec::new();
    };
    if let Some(sequence) = value.as_sequence() {
        return sequence.iter().filter_map(yaml_scalar_to_string).collect();
    }
    yaml_scalar_to_string(value).into_iter().collect()
}

fn merge_strings(target: &mut Vec<String>, values: Vec<String>) {
    for value in values {
        if !target.contains(&value) {
            target.push(value);
        }
    }
}

fn yaml_sequence_field(key: &str, values: &[String]) -> String {
    if values.is_empty() {
        return format!("{key}: []");
    }
    let lines = values
        .iter()
        .map(|value| format!("  - \"{}\"", value.replace('"', "\\\"")))
        .collect::<Vec<_>>()
        .join("\n");
    format!("{key}:\n{lines}")
}

fn replace_top_level_yaml_field(text: &mut String, key: &str, replacement: &str) {
    let mut lines = text.lines().map(ToString::to_string).collect::<Vec<_>>();
    let Some(start) = lines
        .iter()
        .position(|line| line == &format!("{key}:") || line.starts_with(&format!("{key}: ")))
    else {
        return;
    };
    let mut end = start + 1;
    while end < lines.len() && (lines[end].starts_with("  ") || lines[end].trim().is_empty()) {
        end += 1;
    }
    let replacement_lines = replacement
        .lines()
        .map(ToString::to_string)
        .collect::<Vec<_>>();
    lines.splice(start..end, replacement_lines);
    *text = format!("{}\n", lines.join("\n"));
}

pub(crate) fn sync_root_task_bundle_into_worktree(
    primary_checkout_root: &Path,
    worktree_root: &Path,
    issue_ref: &IssueRef,
) -> Result<()> {
    let worktree_bundle_dir = issue_ref.worktree_task_bundle_dir_path(worktree_root);
    if let Some(parent) = worktree_bundle_dir.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::create_dir_all(&worktree_bundle_dir)?;

    let bundle_pairs = [
        (
            issue_ref.task_bundle_stp_path(primary_checkout_root),
            issue_ref.worktree_task_bundle_stp_path(worktree_root),
        ),
        (
            issue_ref.task_bundle_input_path(primary_checkout_root),
            issue_ref.worktree_task_bundle_input_path(worktree_root),
        ),
        (
            issue_ref.task_bundle_output_path(primary_checkout_root),
            issue_ref.worktree_task_bundle_output_path(worktree_root),
        ),
        (
            issue_ref.task_bundle_plan_path(primary_checkout_root),
            issue_ref.worktree_task_bundle_plan_path(worktree_root),
        ),
        (
            issue_ref.task_bundle_validation_plan_path(primary_checkout_root),
            issue_ref.worktree_task_bundle_validation_plan_path(worktree_root),
        ),
        (
            issue_ref.task_bundle_review_policy_path(primary_checkout_root),
            issue_ref.worktree_task_bundle_review_policy_path(worktree_root),
        ),
    ];

    for (root_path, worktree_path) in bundle_pairs {
        if file_exists_nonempty(&worktree_path) {
            continue;
        }
        if !file_exists_nonempty(&root_path) {
            bail!(
                "start: cannot materialize missing worktree bundle file '{}' because the canonical root file '{}' is absent",
                worktree_path.display(),
                root_path.display()
            );
        }
        if let Some(parent) = worktree_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::copy(&root_path, &worktree_path).with_context(|| {
            format!(
                "start: failed to sync canonical bundle file '{}' into worktree path '{}'",
                root_path.display(),
                worktree_path.display()
            )
        })?;
    }

    Ok(())
}

pub(crate) fn ensure_worktree_task_bundle_materialized(
    worktree_root: &Path,
    issue_ref: &IssueRef,
) -> Result<()> {
    let expected = [
        issue_ref.worktree_task_bundle_stp_path(worktree_root),
        issue_ref.worktree_task_bundle_input_path(worktree_root),
        issue_ref.worktree_task_bundle_output_path(worktree_root),
        issue_ref.worktree_task_bundle_plan_path(worktree_root),
        issue_ref.worktree_task_bundle_validation_plan_path(worktree_root),
        issue_ref.worktree_task_bundle_review_policy_path(worktree_root),
    ];
    let missing: Vec<String> = expected
        .iter()
        .filter(|path| !file_exists_nonempty(path))
        .map(|path| path.display().to_string())
        .collect();
    if !missing.is_empty() {
        bail!(
            "start: bound worktree is missing canonical task-bundle surfaces after materialization; refusing partial execution surface:\n{}",
            missing.join("\n")
        );
    }
    Ok(())
}

pub(crate) fn mirror_docs_templates_into_worktree(
    repo_root: &Path,
    worktree_root: &Path,
) -> Result<()> {
    let source_templates = repo_root.join("docs/templates");
    if !source_templates.is_dir() {
        return Ok(());
    }
    let target_templates = worktree_root.join("docs/templates");
    copy_directory_contents(&source_templates, &target_templates)
}

pub(crate) fn mirror_scope_sprints_into_worktree(
    repo_root: &Path,
    worktree_root: &Path,
    issue_ref: &IssueRef,
) -> Result<()> {
    let source_sprints = repo_root
        .join(".adl")
        .join(issue_ref.scope())
        .join("sprints");
    if !source_sprints.is_dir() {
        return Ok(());
    }
    let target_sprints = worktree_root
        .join(".adl")
        .join(issue_ref.scope())
        .join("sprints");
    crate::cli::pr_cmd_cards::copy_directory_contents_if_missing(&source_sprints, &target_sprints)
}

pub(crate) fn ensure_bootstrap_cards(
    root: &Path,
    issue_ref: &IssueRef,
    title: &str,
    branch: &str,
    source_path: &Path,
) -> Result<(PathBuf, PathBuf, PathBuf)> {
    ensure_bootstrap_cards_with_mode(root, issue_ref, title, branch, source_path, true)
}

#[allow(dead_code)]
pub(crate) fn ensure_pre_run_bootstrap_cards(
    root: &Path,
    issue_ref: &IssueRef,
    title: &str,
    source_path: &Path,
) -> Result<(PathBuf, PathBuf, PathBuf)> {
    ensure_bootstrap_cards_with_mode(root, issue_ref, title, "not bound yet", source_path, false)
}

fn ensure_bootstrap_cards_with_mode(
    root: &Path,
    issue_ref: &IssueRef,
    title: &str,
    branch: &str,
    source_path: &Path,
    bind_existing_cards: bool,
) -> Result<(PathBuf, PathBuf, PathBuf)> {
    let bundle_stp = issue_ref.task_bundle_stp_path(root);
    let bundle_input = issue_ref.task_bundle_input_path(root);
    let bundle_output = issue_ref.task_bundle_output_path(root);
    let bundle_plan = issue_ref.task_bundle_plan_path(root);
    let bundle_validation_plan = issue_ref.task_bundle_validation_plan_path(root);
    let bundle_review_policy = issue_ref.task_bundle_review_policy_path(root);
    let bundle_stp_created = !bundle_stp.is_file();
    if let Some(parent) = bundle_input.parent() {
        fs::create_dir_all(parent)?;
    }
    if bundle_stp_created {
        validate_authored_prompt_surface("start", &bundle_stp, PromptSurfaceKind::Stp)?;
    }
    if prompt_surface_needs_template_refresh(&bundle_input)?
        || prompt_surface_is_bootstrap_stub(&bundle_input, PromptSurfaceKind::Sip)?
    {
        write_input_card(
            root,
            &bundle_input,
            issue_ref,
            title,
            branch,
            source_path,
            &bundle_output,
        )?;
    } else if bind_existing_cards && field_line_value(&bundle_input, "Branch")?.trim() != branch {
        replace_field_line_in_file(&bundle_input, "Branch", branch)?;
    }
    if prompt_surface_needs_template_refresh(&bundle_output)?
        || !output_card_title_matches_slug(&bundle_output, issue_ref.slug())?
    {
        write_output_card(root, &bundle_output, issue_ref, title, branch)?;
    } else if bind_existing_cards && field_line_value(&bundle_output, "Branch")?.trim() != branch {
        replace_field_line_in_file(&bundle_output, "Branch", branch)?;
    }
    if prompt_surface_needs_template_refresh(&bundle_plan)?
        || plan_card_needs_design_time_refresh(&bundle_plan)?
    {
        write_plan_card(root, &bundle_plan, issue_ref, title, branch, source_path)?;
    } else if bind_existing_cards && field_line_value(&bundle_plan, "branch")?.trim() != branch {
        replace_field_line_in_file(&bundle_plan, "branch", &format!("\"{branch}\""))?;
    }
    if prompt_surface_needs_template_refresh(&bundle_validation_plan)? {
        write_validation_plan_card(
            root,
            &bundle_validation_plan,
            issue_ref,
            title,
            branch,
            source_path,
        )?;
    } else if bind_existing_cards
        && field_line_value(&bundle_validation_plan, "branch")?.trim() != format!("\"{branch}\"")
    {
        replace_field_line_in_file(&bundle_validation_plan, "branch", &format!("\"{branch}\""))?;
    }
    if prompt_surface_needs_template_refresh(&bundle_review_policy)? {
        write_review_policy_card(root, &bundle_review_policy, issue_ref, title, branch)?;
    } else if bind_existing_cards
        && field_line_value(&bundle_review_policy, "branch")?.trim() != branch
    {
        replace_field_line_in_file(&bundle_review_policy, "branch", &format!("\"{branch}\""))?;
    }

    let cards_root = resolve_cards_root(root, None);
    let compat_stp = card_stp_path(&cards_root, issue_ref.issue_number());
    let compat_input = card_input_path(&cards_root, issue_ref.issue_number());
    let compat_output = card_output_path(&cards_root, issue_ref.issue_number());
    let compat_plan = card_plan_path(&cards_root, issue_ref.issue_number());
    let compat_validation_plan = card_validation_plan_path(&cards_root, issue_ref.issue_number());
    let compat_review_policy = card_review_policy_path(&cards_root, issue_ref.issue_number());
    ensure_symlink(&compat_stp, &bundle_stp)?;
    ensure_symlink(&compat_input, &bundle_input)?;
    ensure_symlink(&compat_output, &bundle_output)?;
    ensure_symlink(&compat_plan, &bundle_plan)?;
    ensure_symlink(&compat_validation_plan, &bundle_validation_plan)?;
    ensure_symlink(&compat_review_policy, &bundle_review_policy)?;

    let structured_paths = StructuredBundlePaths {
        plan_path: &bundle_plan,
        validation_plan_path: &bundle_validation_plan,
        review_policy_path: &bundle_review_policy,
    };
    if bind_existing_cards {
        validate_bootstrap_cards(
            root,
            issue_ref.issue_number(),
            issue_ref.slug(),
            branch,
            &bundle_input,
            &bundle_output,
            structured_paths,
        )?;
    } else {
        validate_initialized_cards(
            issue_ref.issue_number(),
            issue_ref.slug(),
            &bundle_input,
            &bundle_output,
            root,
            structured_paths,
        )?;
    }
    validate_authored_prompt_surface("start", &bundle_input, PromptSurfaceKind::Sip)?;
    Ok((bundle_stp, bundle_input, bundle_output))
}

pub(crate) fn write_output_card(
    repo_root: &Path,
    path: &Path,
    issue_ref: &IssueRef,
    title: &str,
    branch: &str,
) -> Result<()> {
    let text = render_bootstrap_output_card(repo_root, issue_ref, title, branch)?;
    fs::write(path, text)?;
    Ok(())
}

pub(crate) fn write_plan_card(
    repo_root: &Path,
    path: &Path,
    issue_ref: &IssueRef,
    title: &str,
    branch: &str,
    source_path: &Path,
) -> Result<()> {
    let text = render_bootstrap_plan_card(repo_root, issue_ref, title, branch, source_path)?;
    fs::write(path, text)?;
    Ok(())
}

pub(crate) fn write_validation_plan_card(
    repo_root: &Path,
    path: &Path,
    issue_ref: &IssueRef,
    title: &str,
    branch: &str,
    source_path: &Path,
) -> Result<()> {
    let text =
        render_bootstrap_validation_plan_card(repo_root, issue_ref, title, branch, source_path)?;
    fs::write(path, text)?;
    Ok(())
}

pub(crate) fn write_review_policy_card(
    repo_root: &Path,
    path: &Path,
    issue_ref: &IssueRef,
    title: &str,
    branch: &str,
) -> Result<()> {
    let text = render_bootstrap_review_policy_card(repo_root, issue_ref, title, branch)?;
    fs::write(path, text)?;
    Ok(())
}

fn write_input_card(
    repo_root: &Path,
    path: &Path,
    issue_ref: &IssueRef,
    title: &str,
    branch: &str,
    source_path: &Path,
    output_path: &Path,
) -> Result<()> {
    let mut text = read_prompt_template(
        repo_root,
        "sip",
        &[
            "adl/templates/cards/input_card_template.md",
            ".adl/templates/input_card_template.md",
        ],
    )?;
    let issue_url = format!(
        "https://github.com/{}/issues/{}",
        default_repo(repo_root)?,
        issue_ref.issue_number()
    );
    let source_rel = path_relative_to_repo(repo_root, source_path);
    let output_rel = path_relative_to_repo(repo_root, output_path);
    apply_template_values(
        &mut text,
        &[
            ("<issue>", issue_ref.issue_number().to_string()),
            ("<issue_padded>", issue_ref.padded_issue_number().to_string()),
            ("<task_id>", format!("issue-{}", issue_ref.padded_issue_number())),
            ("<run_id>", format!("issue-{}", issue_ref.padded_issue_number())),
            ("<version>", issue_ref.scope().to_string()),
            ("<slug>", issue_ref.slug().to_string()),
            ("<title>", title.to_string()),
            ("<branch>", branch.to_string()),
            ("<timestamp>", Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()),
            ("<card_status>", "ready".to_string()),
            ("<issue_url>", issue_url.clone()),
            ("<source_issue_prompt>", source_rel.clone()),
            ("<docs_context>", "none".to_string()),
            ("<output_card>", output_rel.clone()),
            ("<required_outcome_type>", "combination".to_string()),
            ("<demo_required>", "false".to_string()),
            (
                "<goal>",
                "Execute the linked issue prompt in the bound issue worktree.".to_string(),
            ),
            (
                "<required_outcome>",
                "Ship the required outcome described by the linked source issue prompt."
                    .to_string(),
            ),
            (
                "<acceptance_criteria>",
                "Satisfy the acceptance criteria in the linked source issue prompt and record focused proof in SOR."
                    .to_string(),
            ),
            (
                "<inputs>",
                "Linked source issue prompt; root task bundle cards; current repository state."
                    .to_string(),
            ),
            (
                "<target_files_surfaces>",
                "Files, docs, tests, commands, schemas, and artifacts named by the linked source issue prompt."
                    .to_string(),
            ),
            (
                "<validation_plan>",
                "Run the smallest proving validation for the touched surface and record exact commands in SOR."
                    .to_string(),
            ),
            (
                "<demo_proof_requirements>",
                "Follow demo and proof requirements from the linked source issue prompt."
                    .to_string(),
            ),
            (
                "<non_goals>",
                "Do not widen scope beyond the linked source issue prompt.".to_string(),
            ),
            (
                "<notes_risks>",
                "Refine this card if the linked source issue prompt changes materially before execution begins."
                    .to_string(),
            ),
        ],
    );
    replace_field_line(
        &mut text,
        "Task ID",
        &format!("issue-{}", issue_ref.padded_issue_number()),
    );
    replace_field_line(
        &mut text,
        "Run ID",
        &format!("issue-{}", issue_ref.padded_issue_number()),
    );
    replace_field_line(&mut text, "Version", issue_ref.scope());
    replace_field_line(&mut text, "Title", title);
    replace_field_line(&mut text, "Branch", branch);
    replace_exact_line(&mut text, "- Issue:", &format!("- Issue: {issue_url}"));
    replace_exact_line(
        &mut text,
        "- Source Issue Prompt: <required repo-relative reference or URL>",
        &format!("- Source Issue Prompt: {}", source_rel),
    );
    replace_exact_line(
        &mut text,
        "- Docs: <required freeform value or 'none'>",
        "- Docs: none",
    );
    replace_exact_line(
        &mut text,
        "- Other: <optional note or 'none'>",
        "- Other: none",
    );
    replace_exact_line(
        &mut text,
        "  output_card: .adl/<scope>/tasks/<task-id>__<slug>/sor.md",
        &format!("  output_card: {}", output_rel),
    );
    apply_input_card_lifecycle(&mut text, branch);
    fs::write(path, text)?;
    Ok(())
}

fn read_prompt_template(repo_root: &Path, kind: &str, fallbacks: &[&str]) -> Result<String> {
    if let Some(path) = active_prompt_template_path(repo_root, kind)? {
        if path.is_file() {
            return fs::read_to_string(&path)
                .with_context(|| format!("failed to read prompt template {}", path.display()));
        }
    }
    let primary = repo_root
        .join("docs")
        .join("templates")
        .join("prompts")
        .join("1.0.0")
        .join(format!("{kind}.md"));
    if primary.is_file() {
        return fs::read_to_string(&primary)
            .with_context(|| format!("failed to read prompt template {}", primary.display()));
    }
    for fallback in fallbacks {
        let path = repo_root.join(fallback);
        if path.is_file() {
            return fs::read_to_string(&path)
                .with_context(|| format!("failed to read prompt template {}", path.display()));
        }
    }
    bail!("missing {kind} prompt template under docs/templates/prompts/current.json or docs/templates/prompts/1.0.0")
}

fn active_prompt_template_path(repo_root: &Path, kind: &str) -> Result<Option<PathBuf>> {
    let registry_path = repo_root.join("docs/templates/prompts/current.json");
    if !registry_path.is_file() {
        return Ok(None);
    }
    let registry_text = fs::read_to_string(&registry_path).with_context(|| {
        format!(
            "failed to read prompt template registry {}",
            registry_path.display()
        )
    })?;
    let registry: Value = serde_json::from_str(&registry_text).with_context(|| {
        format!(
            "failed to parse prompt template registry {}",
            registry_path.display()
        )
    })?;
    let Some(path) = registry
        .get("templates")
        .and_then(|templates| templates.get(kind))
        .and_then(|entry| entry.get("path"))
        .and_then(Value::as_str)
    else {
        return Ok(None);
    };
    Ok(Some(repo_root.join(path)))
}

fn active_prompt_template_set_label(repo_root: &Path) -> String {
    let registry_path = repo_root.join("docs/templates/prompts/current.json");
    let registry_text = match fs::read_to_string(&registry_path) {
        Ok(text) => text,
        Err(_) => return "1.0.0".to_string(),
    };
    let registry: Value = match serde_json::from_str(&registry_text) {
        Ok(value) => value,
        Err(_) => return "1.0.0".to_string(),
    };
    registry
        .get("semver")
        .and_then(Value::as_str)
        .or_else(|| {
            registry
                .get("csdlc_prompt_template_set")
                .and_then(Value::as_str)
        })
        .unwrap_or("1.0.0")
        .to_string()
}

fn apply_template_values(text: &mut String, values: &[(&str, String)]) {
    for (token, value) in values {
        *text = text.replace(token, value);
    }
}

fn apply_input_card_lifecycle(text: &mut String, branch: &str) {
    if branch_indicates_unbound_state(branch) {
        return;
    }
    replace_exact_line(
        text,
        "- This issue is not started yet; do not assume a branch or worktree already exists.",
        "- Do not run `pr start`; the branch and worktree already exist.",
    );
    replace_exact_line(
        text,
        "- Do not run `pr start`; use the current issue-mode `pr run` flow only if execution later becomes necessary.",
        "- Do not delete or recreate cards.",
    );
    deduplicate_exact_line(text, "- Do not delete or recreate cards.");
    replace_exact_line(
        text,
        "Prepare the linked issue prompt and review surfaces for truthful pre-run review before execution is bound.",
        "Execute the linked issue prompt in this started worktree without rerunning bootstrap commands.",
    );
    replace_exact_line(
        text,
        "- Keep the linked issue prompt, input card, and output record aligned for review.",
        "- Ship the required outcome type recorded in the linked issue prompt.",
    );
    replace_exact_line(
        text,
        "- Preserve truthful lifecycle state until `pr run` binds the branch and worktree.",
        "- Keep the linked issue prompt, repository changes, and output record aligned.",
    );
    replace_exact_line(
        text,
        "- The linked source issue prompt is reviewable and structurally valid.",
        "- The implementation satisfies the linked issue prompt.",
    );
    replace_exact_line(
        text,
        "- The card bundle does not imply a branch or worktree exists before `pr run`.",
        "- Validation and proof surfaces named below are completed or explicitly marked not applicable.",
    );
    replace_exact_line(
        text,
        "- root task bundle cards",
        "- root and worktree task bundle cards",
    );
    replace_exact_line(
        text,
        "- current repository state before execution binding",
        "- current repository state for this branch",
    );
    replace_exact_line(
        text,
        "- files, docs, tests, commands, schemas, and artifacts named by the linked issue prompt, once execution is bound",
        "- files, docs, tests, commands, schemas, and artifacts named by the linked issue prompt",
    );
    replace_exact_line(
        text,
        "- Commands to run before execution: structured prompt/card validation only, unless the source issue prompt explicitly requires a pre-run proof.",
        "- Commands to run: derive the exact command set from the linked issue prompt and repo state; record what actually ran in the output card.",
    );
    replace_exact_line(
        text,
        "- Commands to run during execution: derive the exact command set from the linked issue prompt and repo state after `pr run` binds the worktree.",
        "- Tests to run: execute the smallest proving test set for the required outcome.",
    );
    replace_exact_line(
        text,
        "- Tests to run: execute the smallest proving test set for the required outcome during execution.",
        "- Artifacts or traces: produce or update the proof surfaces required by the linked issue prompt.",
    );
    replace_exact_line(
        text,
        "- Artifacts or traces: produce or update the proof surfaces required by the linked issue prompt during execution.",
        "- Reviewer checks: capture any manual review or demo checks in the output card.",
    );
    replace_exact_line(
        text,
        "- Proof surfaces: use the proof surfaces named by the linked issue prompt and output card once execution is bound.",
        "- Proof surfaces: use the proof surfaces named by the linked issue prompt and output card.",
    );
    replace_exact_line(
        text,
        "- No-demo rationale: if no demo is required, explain why in the output card during execution.",
        "- No-demo rationale: if no demo is required, explain why in the output card.",
    );
    replace_exact_line(
        text,
        "- Refine this card if the linked issue prompt changes materially before execution begins.",
        "- Refine this card if the linked issue prompt changes materially before implementation begins.",
    );
    replace_exact_line(
        text,
        "- When execution is approved, run the repo-native issue-mode `pr run` flow and then perform the work described above.",
        "- Do the work described above.",
    );
    replace_exact_line(
        text,
        "- Write results to the paired output card file during execution.",
        "- Write results to the paired output card file.",
    );
}

fn prompt_surface_is_bootstrap_stub(path: &Path, kind: PromptSurfaceKind) -> Result<bool> {
    if !path.is_file() {
        return Ok(false);
    }
    let text = fs::read_to_string(path)?;
    Ok(bootstrap_stub_reason(&text, kind).is_some())
}

fn render_bootstrap_output_card(
    repo_root: &Path,
    issue_ref: &IssueRef,
    title: &str,
    branch: &str,
) -> Result<String> {
    let output_rel =
        path_relative_to_repo(repo_root, &issue_ref.task_bundle_output_path(repo_root));
    let vpp_rel = path_relative_to_repo(
        repo_root,
        &issue_ref.task_bundle_validation_plan_path(repo_root),
    );
    let timestamp = Utc::now().format("%Y-%m-%dT%H:%M:%SZ");
    let pre_run_unbound = branch_indicates_unbound_state(branch);
    let status = if pre_run_unbound {
        "NOT_STARTED"
    } else {
        "IN_PROGRESS"
    };
    let branch_action = if pre_run_unbound {
        "Preserved pre-run branch truth; no execution branch or worktree is bound yet."
    } else {
        "Reserved the execution branch for later implementation."
    };
    let source_path = issue_ref.issue_prompt_path(repo_root);
    let prompt = fs::read_to_string(&source_path).unwrap_or_default();
    let metadata = SourcePromptMetadata::from_prompt(&prompt);
    let initial_pvf_lane = resolved_initial_pvf_lane(&metadata, title, &prompt);
    let planned_pvf_lane = resolved_planned_pvf_lane(&initial_pvf_lane);
    let summary =
        "Pre-run output scaffold initialized during issue-wave opening. No implementation has started yet.";
    let tracked_implementation_artifacts = "not_applicable until execution begins";
    let additional_proof_artifacts = "not_applicable until execution begins";
    let worktree_only_paths_remaining = "no tracked implementation artifacts exist yet; execution-time proof surfaces will be established during implementation and PR publication";
    let integration_method_used =
        "local ignored card-bundle scaffold write under the active checkout; tracked implementation artifacts do not exist yet";
    let bootstrap_validation_command =
        format!("bash adl/tools/validate_structured_prompt.sh --type sor --phase bootstrap --input {output_rel}");
    let mut text = read_prompt_template(repo_root, "sor", &[])?;
    apply_template_values(
        &mut text,
        &[
            ("<issue>", issue_ref.issue_number().to_string()),
            (
                "<issue_padded>",
                issue_ref.padded_issue_number().to_string(),
            ),
            (
                "<task_id>",
                format!("issue-{}", issue_ref.padded_issue_number()),
            ),
            (
                "<run_id>",
                format!("issue-{}", issue_ref.padded_issue_number()),
            ),
            ("<version>", issue_ref.scope().to_string()),
            ("<slug>", issue_ref.slug().to_string()),
            ("<title>", title.to_string()),
            ("<branch>", branch.to_string()),
            (
                "<card_status>",
                if pre_run_unbound { "draft" } else { "ready" }.to_string(),
            ),
            ("<status>", status.to_string()),
            ("<timestamp>", timestamp.to_string()),
            ("<output_card>", output_rel),
            ("<execution_actor>", "issue-wave bootstrap".to_string()),
            ("<model>", "not_applicable".to_string()),
            ("<provider>", "not_applicable".to_string()),
            ("<start_time>", timestamp.to_string()),
            ("<end_time>", timestamp.to_string()),
            ("<summary>", summary.to_string()),
            ("<initial_pvf_lane>", initial_pvf_lane.clone()),
            ("<planned_pvf_lane>", planned_pvf_lane.clone()),
            ("<final_pvf_lane>", "not_recorded_yet".to_string()),
            ("<lane_change_reason>", "not_recorded_yet".to_string()),
            (
                "<expected_runtime_class>",
                metadata_value_or_unknown(&metadata.validation_runtime_class),
            ),
            (
                "<estimate_elapsed_seconds>",
                metadata_value_or_unknown(&metadata.estimate_elapsed_seconds),
            ),
            (
                "<estimated_elapsed_seconds>",
                metadata_value_or_unknown(&metadata.estimate_elapsed_seconds),
            ),
            ("<actual_elapsed_seconds>", "unknown".to_string()),
            ("<actual_active_work_seconds>", "unknown".to_string()),
            (
                "<estimate_total_tokens>",
                metadata_value_or_unknown(&metadata.estimate_total_tokens),
            ),
            (
                "<estimated_total_tokens>",
                metadata_value_or_unknown(&metadata.estimate_total_tokens),
            ),
            ("<actual_total_tokens>", "unknown".to_string()),
            (
                "<estimate_validation_seconds>",
                metadata_value_or_unknown(&metadata.estimate_validation_seconds),
            ),
            (
                "<estimated_validation_seconds>",
                metadata_value_or_unknown(&metadata.estimate_validation_seconds),
            ),
            ("<actual_validation_seconds>", "unknown".to_string()),
            (
                "<budget_source>",
                metadata_value_or_unknown(&metadata.estimate_data_source),
            ),
            ("<actual_pr_wait_seconds>", "unknown".to_string()),
            ("<actual_ci_wait_seconds>", "unknown".to_string()),
            (
                "<actual_metrics_data_source>",
                metadata_value_or_unknown(&metadata.estimate_data_source),
            ),
            (
                "<actual_metrics_source_ref>",
                metadata_value_or_unknown(&metadata.estimate_source_ref),
            ),
            (
                "<actual_metrics_confidence>",
                metadata_value_or_unknown(&metadata.estimate_confidence),
            ),
            ("<estimate_error_percent>", "unknown".to_string()),
            ("<completion_state>", "unknown".to_string()),
            ("<issue_goal_ref>", format!("issue-{}", issue_ref.issue_number())),
            ("<sprint_goal_ref>", "unknown".to_string()),
            (
                "<goal_metrics_rollup_ref>",
                metadata_value_or_unknown(&metadata.goal_metrics_rollup_ref),
            ),
            ("<vpp_card>", vpp_rel),
            ("<variance_analysis_required>", "not_applicable".to_string()),
            ("<variance_analysis_completed>", "not_applicable".to_string()),
            ("<variance_category>", "not_applicable".to_string()),
            (
                "<variance_note>",
                "Bootstrap scaffold records unknown issue metrics only; variance analysis is deferred until execution produces authoritative estimates and actuals."
                    .to_string(),
            ),
            (
                "<tracked_implementation_artifacts>",
                tracked_implementation_artifacts.to_string(),
            ),
            (
                "<additional_proof_artifacts>",
                additional_proof_artifacts.to_string(),
            ),
            (
                "<actions_taken_line_1>",
                "Opened the local issue bundle and wrote a truthful pre-run output scaffold."
                    .to_string(),
            ),
            ("<branch_action>", branch_action.to_string()),
            ("<actions_taken_line_2>", branch_action.to_string()),
            (
                "<actions_taken_line_3>",
                "Deferred implementation, proof capture, and release integration to the execution lifecycle and PR publication."
                    .to_string(),
            ),
            ("<main_repo_paths_updated>", "none".to_string()),
            (
                "<worktree_only_paths_remaining>",
                worktree_only_paths_remaining.to_string(),
            ),
            ("<integration_state>", "worktree_only".to_string()),
            ("<verification_scope>", "main_repo".to_string()),
            (
                "<integration_method_used>",
                integration_method_used.to_string(),
            ),
            (
                "<integration_verification_command>",
                bootstrap_validation_command.clone(),
            ),
            (
                "<integration_verification_effect>",
                "Verified bootstrap SOR contract compliance for the local pre-run scaffold."
                    .to_string(),
            ),
            ("<integration_result>", "PASS".to_string()),
            (
                "<validation_command>",
                bootstrap_validation_command.clone(),
            ),
            (
                "<validation_effect>",
                "Verified bootstrap SOR contract compliance for the local output scaffold."
                    .to_string(),
            ),
            ("<validation_result>", "PASS".to_string()),
            ("<verification_validation_status>", "PASS".to_string()),
            (
                "<verification_check_1>",
                bootstrap_validation_command.clone(),
            ),
            ("<verification_determinism_status>", "NOT_RUN".to_string()),
            ("<verification_replay_verified>", "unknown".to_string()),
            (
                "<verification_ordering_guarantees_verified>",
                "unknown".to_string(),
            ),
            (
                "<verification_security_privacy_status>",
                "PARTIAL".to_string(),
            ),
            (
                "<verification_secrets_leakage_detected>",
                "false".to_string(),
            ),
            (
                "<verification_prompt_or_tool_arg_leakage_detected>",
                "false".to_string(),
            ),
            (
                "<verification_absolute_path_leakage_detected>",
                "false".to_string(),
            ),
            ("<verification_artifacts_status>", "PASS".to_string()),
            (
                "<verification_required_artifacts_present>",
                "true".to_string(),
            ),
            (
                "<verification_schema_changes_present>",
                "false".to_string(),
            ),
            (
                "<verification_schema_changes_approved>",
                "not_applicable".to_string(),
            ),
            (
                "<determinism_tests_executed>",
                "not_run; bootstrap scaffold creation has not been replay-verified for this issue yet."
                    .to_string(),
            ),
            (
                "<fixtures_or_scripts_used>",
                "`adl/tools/pr.sh` issue-wave opening flow.".to_string(),
            ),
            (
                "<replay_verification>",
                "not yet verified for this specific issue record.".to_string(),
            ),
            (
                "<ordering_guarantees>",
                "not_applicable for a single-card bootstrap write.".to_string(),
            ),
            (
                "<artifact_stability_notes>",
                "repository-relative paths only; execution-time proof artifacts are not expected yet."
                    .to_string(),
            ),
            (
                "<secret_leakage_scan_performed>",
                "limited content review only; no secrets were intentionally recorded in the scaffold."
                    .to_string(),
            ),
            (
                "<prompt_tool_arg_redaction_verified>",
                "not_applicable for bootstrap scaffold generation.".to_string(),
            ),
            (
                "<absolute_path_leakage_check>",
                "repository-relative paths only in the scaffold.".to_string(),
            ),
            (
                "<sandbox_policy_invariants_preserved>",
                "yes; local ignored issue-record path only.".to_string(),
            ),
            (
                "<trace_bundle_paths>",
                "not_applicable until execution begins".to_string(),
            ),
            (
                "<run_artifact_root>",
                "not_applicable until execution begins".to_string(),
            ),
            ("<replay_command>", "not_run".to_string()),
            ("<replay_result>", "NOT_RUN".to_string()),
            (
                "<primary_proof_surface>",
                "this local pre-run SOR scaffold and its bootstrap validation result"
                    .to_string(),
            ),
            (
                "<required_artifacts_present>",
                "local output card scaffold only; tracked implementation artifacts are not expected yet"
                    .to_string(),
            ),
            (
                "<artifact_schema_checks>",
                "bootstrap SOR validator passed".to_string(),
            ),
            ("<hash_byte_stability_checks>", "not_run".to_string()),
            (
                "<missing_optional_artifacts_rationale>",
                "execution proofs, demos, and tracked outputs are intentionally absent before implementation begins"
                    .to_string(),
            ),
            (
                "<decision_or_deviation_1>",
                "Issue-wave opening emits a truthful pre-run SOR scaffold instead of leaving raw template residue for later cleanup."
                    .to_string(),
            ),
            (
                "<decision_or_deviation_2>",
                "Integration state remains `worktree_only` until execution creates tracked artifacts or opens a PR."
                    .to_string(),
            ),
            (
                "<follow_up_1>",
                "Update this record during execution with actual actions, validations, proof surfaces, and integration truth."
                    .to_string(),
            ),
            (
                "<follow_up_2>",
                "Normalize this record to `pr_open`, `merged`, or `closed_no_pr` during finish/closeout as appropriate."
                    .to_string(),
            ),
        ],
    );
    replace_exact_line(
        &mut text,
        "- Main-repo paths updated: `none`",
        "- Main-repo paths updated: none",
    );
    replace_exact_line(
        &mut text,
        &format!("- Worktree-only paths remaining: `{worktree_only_paths_remaining}`"),
        &format!("- Worktree-only paths remaining: {worktree_only_paths_remaining}"),
    );
    replace_exact_line(
        &mut text,
        "- Integration state: `worktree_only`",
        "- Integration state: worktree_only",
    );
    replace_exact_line(
        &mut text,
        "- Verification scope: `main_repo`",
        "- Verification scope: main_repo",
    );
    replace_exact_line(
        &mut text,
        &format!("- Integration method used: `{integration_method_used}`"),
        &format!("- Integration method used: {integration_method_used}"),
    );
    replace_exact_line(&mut text, "- Result: `PASS`", "- Result: PASS");
    replace_exact_line(
        &mut text,
        "- Replay result: `NOT_RUN`",
        "- Replay result: NOT_RUN",
    );
    Ok(text)
}

fn render_bootstrap_plan_card(
    repo_root: &Path,
    issue_ref: &IssueRef,
    title: &str,
    branch: &str,
    source_path: &Path,
) -> Result<String> {
    let stp_rel = path_relative_to_repo(repo_root, &issue_ref.task_bundle_stp_path(repo_root));
    let sip_rel = path_relative_to_repo(repo_root, &issue_ref.task_bundle_input_path(repo_root));
    let spp_rel = path_relative_to_repo(repo_root, &issue_ref.task_bundle_plan_path(repo_root));
    let vpp_rel = path_relative_to_repo(
        repo_root,
        &issue_ref.task_bundle_validation_plan_path(repo_root),
    );
    let source_rel = path_relative_to_repo(repo_root, source_path);
    let timestamp = Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
    let prompt = fs::read_to_string(source_path).unwrap_or_default();
    let metadata = SourcePromptMetadata::from_prompt(&prompt);
    let dependencies = issue_prompt_section(&prompt, "Dependencies")
        .map(|value| one_line_summary(&value))
        .unwrap_or_else(|| "Review the source issue prompt for dependency truth.".to_string());
    let deliverables = issue_prompt_section(&prompt, "Deliverables")
        .map(|value| one_line_summary(&value))
        .unwrap_or_else(|| {
            "Produce the deliverables named by the source issue prompt.".to_string()
        });
    let acceptance = issue_prompt_section(&prompt, "Acceptance Criteria")
        .map(|value| one_line_summary(&value))
        .unwrap_or_else(|| {
            "Satisfy the acceptance criteria named by the source issue prompt.".to_string()
        });
    let repo_inputs = issue_prompt_section(&prompt, "Repo Inputs")
        .map(|value| one_line_summary(&value))
        .unwrap_or_else(|| "Use the repo inputs named by the source issue prompt.".to_string());
    let non_goals = issue_prompt_section(&prompt, "Non-goals")
        .map(|value| one_line_summary(&value))
        .unwrap_or_else(|| "Preserve the non-goals named by the source issue prompt.".to_string());
    let validation_strategy = issue_prompt_section(&prompt, "Tooling Notes")
        .or_else(|| issue_prompt_section(&prompt, "Acceptance Criteria"))
        .map(|value| one_line_summary(&value))
        .unwrap_or_else(|| {
            "Run the smallest proving validation for the touched surface.".to_string()
        });
    let dependency_step = yaml_inline(&dependencies);
    let deliverable_step = yaml_inline(&deliverables);
    let acceptance_step = yaml_inline(&acceptance);
    let repo_inputs_step = yaml_inline(&repo_inputs);
    let non_goals_step = yaml_inline(&non_goals);
    let validation_step = yaml_inline(&validation_strategy);
    let initial_pvf_lane = resolved_initial_pvf_lane(&metadata, title, &prompt);
    let initial_pvf_lane_source =
        resolved_initial_pvf_lane_source(&metadata, title, &prompt, &initial_pvf_lane);
    let planned_pvf_lane = resolved_planned_pvf_lane(&initial_pvf_lane);
    let planned_pvf_lane_source =
        resolved_planned_pvf_lane_source(&initial_pvf_lane, &initial_pvf_lane_source);
    let mut text = read_prompt_template(repo_root, "spp", &[])?;
    let issue_url = format!(
        "https://github.com/{}/issues/{}",
        default_repo(repo_root)
            .unwrap_or_else(|_| "danielbaustin/agent-design-language".to_string()),
        issue_ref.issue_number()
    );
    apply_template_values(
        &mut text,
        &[
            ("<issue>", issue_ref.issue_number().to_string()),
            ("<issue_padded>", issue_ref.padded_issue_number().to_string()),
            ("<task_id>", format!("issue-{}", issue_ref.padded_issue_number())),
            ("<run_id>", format!("issue-{}", issue_ref.padded_issue_number())),
            ("<version>", issue_ref.scope().to_string()),
            ("<slug>", issue_ref.slug().to_string()),
            ("<title>", title.to_string()),
            ("<branch>", branch.to_string()),
            ("<timestamp>", timestamp),
            ("<card_status>", "ready".to_string()),
            ("<status>", "ready".to_string()),
            ("<activation_state>", "ready".to_string()),
            ("<issue_url>", issue_url),
            ("<source_issue_prompt>", source_rel),
            ("<stp_card>", stp_rel),
            ("<sip_card>", sip_rel),
            ("<spp_card>", spp_rel),
            ("<vpp_card>", vpp_rel),
            ("<target_files_surfaces_inline>", repo_inputs_step.clone()),
            ("<non_goals_inline>", non_goals_step),
            ("<plan_summary>", format!("Issue-local execution plan for {title}.")),
            ("<dependencies_inline>", dependency_step),
            ("<repo_inputs_inline>", repo_inputs_step),
            ("<deliverables_inline>", deliverable_step),
            ("<acceptance_criteria_inline>", acceptance_step),
            (
                "<risks_inline>",
                "Generated card may need editor tightening if the source issue prompt is underspecified."
                    .to_string(),
            ),
            ("<validation_plan_inline>", validation_step),
            (
                "<notes_risks_inline>",
                format!(
                    "Generated from {} template; update before continuing if execution diverges.",
                    active_prompt_template_set_label(repo_root)
                ),
            ),
            ("<initial_pvf_lane>", initial_pvf_lane.clone()),
            ("<planned_pvf_lane>", planned_pvf_lane.clone()),
            ("<planned_pvf_lane_source>", planned_pvf_lane_source.clone()),
            ("<expected_runtime_class>", "unknown".to_string()),
            (
                "<estimate_elapsed_seconds>",
                metadata_value_or_unknown(&metadata.estimate_elapsed_seconds),
            ),
            (
                "<estimated_elapsed_seconds>",
                metadata_value_or_unknown(&metadata.estimate_elapsed_seconds),
            ),
            (
                "<estimate_total_tokens>",
                metadata_value_or_unknown(&metadata.estimate_total_tokens),
            ),
            (
                "<estimated_total_tokens>",
                metadata_value_or_unknown(&metadata.estimate_total_tokens),
            ),
            (
                "<estimate_validation_seconds>",
                metadata_value_or_unknown(&metadata.estimate_validation_seconds),
            ),
            (
                "<estimated_validation_seconds>",
                metadata_value_or_unknown(&metadata.estimate_validation_seconds),
            ),
            (
                "<issue_goal_token_budget>",
                metadata_value_or_unknown(&metadata.issue_goal_token_budget),
            ),
            ("<variance_threshold_percent>", "10".to_string()),
            (
                "<estimate_confidence>",
                metadata_value_or_unknown(&metadata.estimate_confidence),
            ),
            (
                "<estimate_data_source>",
                metadata_value_or_unknown(&metadata.estimate_data_source),
            ),
            (
                "<estimate_source_ref>",
                metadata_value_or_unknown(&metadata.estimate_source_ref),
            ),
            ("<issue_goal_ref>", format!("issue-{}", issue_ref.issue_number())),
            ("<sprint_goal_ref>", "unknown".to_string()),
            (
                "<goal_metrics_rollup_ref>",
                metadata_value_or_unknown(&metadata.goal_metrics_rollup_ref),
            ),
        ],
    );
    Ok(text)
}

fn render_bootstrap_validation_plan_card(
    repo_root: &Path,
    issue_ref: &IssueRef,
    title: &str,
    branch: &str,
    source_path: &Path,
) -> Result<String> {
    let stp_rel = path_relative_to_repo(repo_root, &issue_ref.task_bundle_stp_path(repo_root));
    let sip_rel = path_relative_to_repo(repo_root, &issue_ref.task_bundle_input_path(repo_root));
    let spp_rel = path_relative_to_repo(repo_root, &issue_ref.task_bundle_plan_path(repo_root));
    let source_rel = path_relative_to_repo(repo_root, source_path);
    let timestamp = Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
    let prompt = fs::read_to_string(source_path).unwrap_or_default();
    let metadata = SourcePromptMetadata::from_prompt(&prompt);
    let initial_pvf_lane = resolved_initial_pvf_lane(&metadata, title, &prompt);
    let initial_pvf_lane_source =
        resolved_initial_pvf_lane_source(&metadata, title, &prompt, &initial_pvf_lane);
    let planned_pvf_lane = resolved_planned_pvf_lane(&initial_pvf_lane);
    let planned_pvf_lane_source =
        resolved_planned_pvf_lane_source(&initial_pvf_lane, &initial_pvf_lane_source);
    let fallback_failure_policy = if planned_pvf_lane == NEEDS_PLANNING_PVF_LANE {
        "fail_closed_until_validation_lane_is_selected".to_string()
    } else {
        "fail_closed".to_string()
    };
    let fallback_validation_strategy = issue_prompt_section(&prompt, "Validation Plan")
        .or_else(|| issue_prompt_section(&prompt, "Tooling Notes"))
        .map(|value| one_line_summary(&value))
        .unwrap_or_else(|| {
            "Select the smallest proving validation lane before execution proceeds.".to_string()
        });
    let fallback_validation_command = yaml_inline(&fallback_validation_strategy);
    let generated_vpp =
        generate_vpp_plan_from_prompt(repo_root, &prompt, &planned_pvf_lane).unwrap_or_else(
            |err| GeneratedVppPlan {
                selected_lanes_inline: planned_pvf_lane.clone(),
                parallel_groups_inline: "unknown".to_string(),
                validation_runtime_class: "unknown".to_string(),
                validation_resource_profile: "unknown".to_string(),
                validation_family: "unknown".to_string(),
                validation_size_split: "unknown".to_string(),
                expected_proof_cost: "unknown".to_string(),
                planned_validation_seconds: "unknown".to_string(),
                planned_validation_tokens: "unknown".to_string(),
                validation_commands_inline: fallback_validation_command.clone(),
                failure_policy: fallback_failure_policy.clone(),
                notes_risks_inline: format!(
                    "Generated from {} template; lane source: {planned_pvf_lane_source}. Validation-profile derivation fallback used: {}",
                    active_prompt_template_set_label(repo_root),
                    one_line_summary(&err.to_string())
                ),
            },
        );
    let generated_vpp = GeneratedVppPlan {
        validation_runtime_class: metadata
            .validation_runtime_class
            .clone()
            .unwrap_or(generated_vpp.validation_runtime_class),
        validation_resource_profile: metadata
            .validation_resource_profile
            .clone()
            .unwrap_or(generated_vpp.validation_resource_profile),
        validation_family: metadata
            .validation_family
            .clone()
            .unwrap_or(generated_vpp.validation_family),
        validation_size_split: metadata
            .validation_size_split
            .clone()
            .unwrap_or(generated_vpp.validation_size_split),
        expected_proof_cost: metadata
            .expected_proof_cost
            .clone()
            .unwrap_or(generated_vpp.expected_proof_cost),
        planned_validation_seconds: metadata
            .planned_validation_seconds
            .clone()
            .unwrap_or(generated_vpp.planned_validation_seconds),
        planned_validation_tokens: metadata
            .planned_validation_tokens
            .clone()
            .unwrap_or(generated_vpp.planned_validation_tokens),
        ..generated_vpp
    };
    let mut text = read_prompt_template(repo_root, "vpp", &[])?;
    let issue_url = format!(
        "https://github.com/{}/issues/{}",
        default_repo(repo_root)
            .unwrap_or_else(|_| "danielbaustin/agent-design-language".to_string()),
        issue_ref.issue_number()
    );
    apply_template_values(
        &mut text,
        &[
            ("<issue>", issue_ref.issue_number().to_string()),
            (
                "<issue_padded>",
                issue_ref.padded_issue_number().to_string(),
            ),
            (
                "<task_id>",
                format!("issue-{}", issue_ref.padded_issue_number()),
            ),
            (
                "<run_id>",
                format!("issue-{}", issue_ref.padded_issue_number()),
            ),
            ("<version>", issue_ref.scope().to_string()),
            ("<slug>", issue_ref.slug().to_string()),
            ("<title>", title.to_string()),
            ("<branch>", branch.to_string()),
            ("<timestamp>", timestamp),
            ("<card_status>", "ready".to_string()),
            ("<status>", "ready".to_string()),
            ("<issue_url>", issue_url),
            ("<stp_card>", stp_rel),
            ("<sip_card>", sip_rel),
            ("<spp_card>", spp_rel),
            ("<initial_pvf_lane>", initial_pvf_lane),
            ("<planned_pvf_lane>", planned_pvf_lane.clone()),
            (
                "<lane_registry_path>",
                "docs/validation/pvf_lanes.json".to_string(),
            ),
            ("<lane_registry_template_set>", "vpp.lane.v1".to_string()),
            (
                "<validation_runtime_class>",
                generated_vpp.validation_runtime_class,
            ),
            (
                "<validation_resource_profile>",
                generated_vpp.validation_resource_profile,
            ),
            ("<validation_family>", generated_vpp.validation_family),
            (
                "<validation_size_split>",
                generated_vpp.validation_size_split,
            ),
            ("<expected_proof_cost>", generated_vpp.expected_proof_cost),
            (
                "<planned_validation_seconds>",
                generated_vpp.planned_validation_seconds,
            ),
            (
                "<planned_validation_tokens>",
                generated_vpp.planned_validation_tokens,
            ),
            (
                "<issue_goal_ref>",
                format!("issue-{}", issue_ref.issue_number()),
            ),
            ("<sprint_goal_ref>", "unknown".to_string()),
            (
                "<goal_metrics_rollup_ref>",
                metadata_value_or_unknown(&metadata.goal_metrics_rollup_ref),
            ),
            (
                "<selected_lanes_inline>",
                generated_vpp.selected_lanes_inline,
            ),
            (
                "<parallel_groups_inline>",
                generated_vpp.parallel_groups_inline,
            ),
            (
                "<validation_commands_inline>",
                generated_vpp.validation_commands_inline,
            ),
            ("<failure_policy>", generated_vpp.failure_policy),
            ("<notes_risks_inline>", generated_vpp.notes_risks_inline),
            (
                "<plan_summary>",
                format!(
                    "Validation planning prompt for {title}; source issue prompt: {source_rel}."
                ),
            ),
        ],
    );
    Ok(text)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct GeneratedVppPlan {
    selected_lanes_inline: String,
    parallel_groups_inline: String,
    validation_runtime_class: String,
    validation_resource_profile: String,
    validation_family: String,
    validation_size_split: String,
    expected_proof_cost: String,
    planned_validation_seconds: String,
    planned_validation_tokens: String,
    validation_commands_inline: String,
    failure_policy: String,
    notes_risks_inline: String,
}

fn generate_vpp_plan_from_prompt(
    repo_root: &Path,
    prompt: &str,
    planned_pvf_lane: &str,
) -> Result<GeneratedVppPlan> {
    let declared_paths = extract_declared_repo_paths(repo_root, prompt);
    if declared_paths.is_empty() {
        return Ok(GeneratedVppPlan {
            selected_lanes_inline: planned_pvf_lane.to_string(),
            parallel_groups_inline: "unknown".to_string(),
            validation_runtime_class: "unknown".to_string(),
            validation_resource_profile: "unknown".to_string(),
            validation_family: "unknown".to_string(),
            validation_size_split: "unknown".to_string(),
            expected_proof_cost: "unknown".to_string(),
            planned_validation_seconds: "unknown".to_string(),
            planned_validation_tokens: "unknown".to_string(),
            validation_commands_inline: yaml_inline(
                "Select the smallest proving validation lane before execution proceeds.",
            ),
            failure_policy: "fail_closed".to_string(),
            notes_risks_inline: "Generated from prompt metadata only; declare repo-relative target surfaces to derive a fact-backed VPP.".to_string(),
        });
    }

    let temp_file = std::env::temp_dir().join(format!(
        "adl-vpp-paths-{}-{}.txt",
        std::process::id(),
        Utc::now().timestamp_nanos_opt().unwrap_or_default()
    ));
    fs::write(&temp_file, declared_paths.join("\n") + "\n").with_context(|| {
        format!(
            "failed to write temporary changed-files list at {}",
            temp_file.display()
        )
    })?;
    let output = Command::new("python3")
        .arg(repo_root.join("adl/tools/validation_manager.py"))
        .arg("--changed-files")
        .arg(&temp_file)
        .arg("--json")
        .current_dir(repo_root)
        .output()
        .with_context(|| {
            format!(
                "failed to invoke validation_manager.py for {}",
                temp_file.display()
            )
        });
    let _ = fs::remove_file(&temp_file);
    let output = output?;
    if !output.status.success() {
        bail!(
            "validation manager returned non-zero status: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }
    let profile: Value = serde_json::from_slice(&output.stdout)
        .context("validation manager returned invalid JSON for VPP generation")?;
    Ok(generated_vpp_plan_from_profile_json(&profile))
}

fn generated_vpp_plan_from_profile_json(profile: &Value) -> GeneratedVppPlan {
    let selected_profile = profile
        .get("selected_profile")
        .and_then(Value::as_str)
        .unwrap_or("unknown_profile");
    let status = profile
        .get("status")
        .and_then(Value::as_str)
        .unwrap_or("unknown");
    let publication_sufficient = profile
        .get("pr_publication_sufficient")
        .and_then(Value::as_bool)
        .unwrap_or(false);

    let run_items = profile
        .get("run")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    let not_run_items = profile
        .get("not_run")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    let escalation_reasons = profile
        .get("escalation")
        .and_then(|value| value.get("reasons"))
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();

    let selected_lanes = run_items
        .iter()
        .filter_map(|item| item.get("lane_id").and_then(Value::as_str))
        .map(ToOwned::to_owned)
        .collect::<Vec<_>>();
    let selected_lanes_inline = if selected_lanes.is_empty() {
        "none_selected".to_string()
    } else {
        selected_lanes.join(", ")
    };

    let mut parallel_groups = BTreeSet::new();
    for item in &run_items {
        if let Some(group) = item
            .get("vpp_record")
            .and_then(|value| value.get("parallel_group"))
            .and_then(Value::as_str)
        {
            if !group.trim().is_empty() {
                parallel_groups.insert(group.trim().to_string());
            }
        }
    }
    if parallel_groups.is_empty() {
        parallel_groups.insert("local".to_string());
    }

    let mut command_items = run_items
        .iter()
        .filter_map(|item| item.get("command").and_then(Value::as_str))
        .map(sanitize_vpp_validation_command)
        .collect::<Vec<_>>();
    if command_items.is_empty() {
        command_items.push(format!(
            "validation profile `{selected_profile}` is not runnable; review escalation before execution"
        ));
    }
    command_items.extend(not_run_items.iter().take(4).filter_map(|item| {
        let surface = item.get("surface").and_then(Value::as_str)?;
        let reason = item.get("reason").and_then(Value::as_str)?;
        Some(format!("deferred {surface}: {reason}"))
    }));
    command_items.extend(escalation_reasons.iter().filter_map(|item| {
        let lane_id = item.get("lane_id").and_then(Value::as_str)?;
        let reason = item.get("reason").and_then(Value::as_str)?;
        Some(format!("escalation {lane_id}: {reason}"))
    }));

    let selected_count = run_items.len();
    let size_split = match selected_count {
        0 => "not_applicable",
        1 => "small_only",
        _ => "mixed",
    };
    let runtime_class = profile
        .get("estimated_cost")
        .and_then(|value| value.get("runtime_class"))
        .and_then(Value::as_str)
        .unwrap_or("unknown");
    let proof_cost = profile
        .get("estimated_cost")
        .and_then(|value| value.get("token_review_cost"))
        .and_then(Value::as_str)
        .unwrap_or("unknown");
    let notes = if escalation_reasons.is_empty() {
        format!(
            "Generated from validation profile {selected_profile} (status={status}, pr_publication_sufficient={publication_sufficient})."
        )
    } else {
        format!(
            "Generated from validation profile {selected_profile} (status={status}, pr_publication_sufficient={publication_sufficient}); escalation remains explicit in this VPP."
        )
    };

    GeneratedVppPlan {
        selected_lanes_inline,
        parallel_groups_inline: parallel_groups.into_iter().collect::<Vec<_>>().join(", "),
        validation_runtime_class: runtime_class.to_string(),
        validation_resource_profile: "local".to_string(),
        validation_family: selected_profile.to_string(),
        validation_size_split: size_split.to_string(),
        expected_proof_cost: proof_cost.to_string(),
        planned_validation_seconds: "unknown".to_string(),
        planned_validation_tokens: "unknown".to_string(),
        validation_commands_inline: yaml_inline(&command_items.join("; ")),
        failure_policy: "fail_closed".to_string(),
        notes_risks_inline: notes,
    }
}

fn extract_declared_repo_paths(repo_root: &Path, prompt: &str) -> Vec<String> {
    let mut paths = BTreeSet::new();
    for heading in ["Repo Inputs", "Target Files / Surfaces"] {
        let Some(section) = issue_prompt_section(prompt, heading) else {
            continue;
        };
        for candidate in extract_paths_from_section(repo_root, &section) {
            paths.insert(candidate);
        }
    }
    paths.into_iter().collect()
}

fn extract_paths_from_section(repo_root: &Path, section: &str) -> Vec<String> {
    let mut paths = BTreeSet::new();
    for line in section.lines() {
        let trimmed = line.trim().trim_start_matches("- ").trim();
        for candidate in extract_backtick_paths(trimmed) {
            if let Some(path) = normalize_declared_repo_path(repo_root, &candidate) {
                paths.insert(path);
            }
        }
        for token in trimmed
            .split(|ch: char| ch.is_whitespace() || matches!(ch, ';' | ',' | '(' | ')' | '[' | ']'))
        {
            if let Some(path) = normalize_declared_repo_path(repo_root, token) {
                paths.insert(path);
            }
        }
    }
    paths.into_iter().collect()
}

fn extract_backtick_paths(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut remainder = text;
    while let Some(start) = remainder.find('`') {
        let after_start = &remainder[start + 1..];
        let Some(end) = after_start.find('`') else {
            break;
        };
        out.push(after_start[..end].to_string());
        remainder = &after_start[end + 1..];
    }
    out
}

fn normalize_declared_repo_path(repo_root: &Path, raw: &str) -> Option<String> {
    let candidate = raw
        .trim()
        .trim_matches(|ch: char| matches!(ch, '`' | '"' | '\'' | ':'));
    if candidate.is_empty()
        || candidate.starts_with("http://")
        || candidate.starts_with("https://")
        || candidate.starts_with('#')
        || candidate.starts_with(".adl/")
    {
        return None;
    }
    let supported_root_file = matches!(
        candidate,
        "AGENTS.md" | "README.md" | "Cargo.toml" | "Cargo.lock"
    );
    let supported_prefix = candidate.starts_with("adl/")
        || candidate.starts_with("docs/")
        || candidate.starts_with(".github/");
    if !(supported_root_file || supported_prefix) {
        return None;
    }
    let path = repo_root.join(candidate);
    if path.exists() {
        Some(candidate.to_string())
    } else {
        None
    }
}

fn issue_prompt_section(text: &str, heading: &str) -> Option<String> {
    let wanted = format!("## {heading}");
    let mut in_section = false;
    let mut lines = Vec::new();
    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed == wanted {
            in_section = true;
            continue;
        }
        if in_section && trimmed.starts_with("## ") {
            break;
        }
        if in_section {
            lines.push(line);
        }
    }
    let value = lines.join("\n").trim().to_string();
    if value.is_empty() {
        None
    } else {
        Some(value)
    }
}

fn one_line_summary(text: &str) -> String {
    let summary = text
        .lines()
        .map(|line| line.trim().trim_start_matches("- ").trim())
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("; ");
    sanitize_prompt_template_residue_literals(&summary)
}

fn sanitize_prompt_template_residue_literals(text: &str) -> String {
    let text = text.replace("[summary truncated]", "[summary-truncated marker]");
    escape_prompt_template_placeholder_literals(&text)
}

fn escape_prompt_template_placeholder_literals(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut cursor = 0;
    while let Some(relative_start) = text[cursor..].find('<') {
        let start = cursor + relative_start;
        let Some(relative_end) = text[start + 1..].find('>') else {
            break;
        };
        let end = start + 1 + relative_end;
        let candidate = &text[start + 1..end];
        if !candidate.is_empty()
            && candidate
                .chars()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
        {
            out.push_str(&text[cursor..start]);
            out.push_str("&lt;");
            out.push_str(candidate);
            out.push_str("&gt;");
            cursor = end + 1;
        } else {
            out.push_str(&text[cursor..=start]);
            cursor = start + 1;
        }
    }
    out.push_str(&text[cursor..]);
    out
}

fn yaml_inline(text: &str) -> String {
    text.replace('\\', "\\\\").replace('"', "'")
}

fn sanitize_vpp_validation_command(command: &str) -> String {
    let mut sanitized = Vec::new();
    let mut replace_next_changed_files = false;
    for token in command.split_whitespace() {
        if replace_next_changed_files {
            sanitized.push(".adl/generated-vpp-changed-files.txt".to_string());
            replace_next_changed_files = false;
            continue;
        }
        if token == "--changed-files" {
            sanitized.push(token.to_string());
            replace_next_changed_files = true;
            continue;
        }
        if token.starts_with('/') {
            sanitized.push("<path>".to_string());
        } else {
            sanitized.push(token.to_string());
        }
    }
    sanitized.join(" ")
}

fn render_bootstrap_review_policy_card(
    repo_root: &Path,
    issue_ref: &IssueRef,
    title: &str,
    branch: &str,
) -> Result<String> {
    let stp_rel = path_relative_to_repo(repo_root, &issue_ref.task_bundle_stp_path(repo_root));
    let sip_rel = path_relative_to_repo(repo_root, &issue_ref.task_bundle_input_path(repo_root));
    let spp_rel = path_relative_to_repo(repo_root, &issue_ref.task_bundle_plan_path(repo_root));
    let vpp_rel = path_relative_to_repo(
        repo_root,
        &issue_ref.task_bundle_validation_plan_path(repo_root),
    );
    let srp_rel = path_relative_to_repo(
        repo_root,
        &issue_ref.task_bundle_review_policy_path(repo_root),
    );
    let sor_rel = path_relative_to_repo(repo_root, &issue_ref.task_bundle_output_path(repo_root));
    let timestamp = Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
    let mut text = read_prompt_template(repo_root, "srp", &[])?;
    let issue_url = format!(
        "https://github.com/{}/issues/{}",
        default_repo(repo_root)
            .unwrap_or_else(|_| "danielbaustin/agent-design-language".to_string()),
        issue_ref.issue_number()
    );
    apply_template_values(
        &mut text,
        &[
            ("<issue>", issue_ref.issue_number().to_string()),
            (
                "<issue_padded>",
                issue_ref.padded_issue_number().to_string(),
            ),
            (
                "<task_id>",
                format!("issue-{}", issue_ref.padded_issue_number()),
            ),
            ("<version>", issue_ref.scope().to_string()),
            ("<slug>", issue_ref.slug().to_string()),
            ("<title>", title.to_string()),
            ("<branch>", branch.to_string()),
            ("<timestamp>", timestamp),
            ("<card_status>", "ready".to_string()),
            ("<issue_url>", issue_url),
            ("<stp_card>", stp_rel),
            ("<sip_card>", sip_rel),
            ("<spp_card>", spp_rel),
            ("<vpp_card>", vpp_rel),
            ("<srp_card>", srp_rel),
            ("<sor_card>", sor_rel),
            ("<findings_status>", "not_run".to_string()),
            ("<recommended_outcome>", "not_run".to_string()),
        ],
    );
    Ok(text)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn extract_declared_repo_paths_ignores_local_adl_and_urls() {
        let repo_root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("adl crate should live under the repo root");
        let prompt = r#"
## Repo Inputs

- https://github.com/example/repo/issues/4425
- `.adl/v0.91.6/tasks/issue-4425__/`
- `adl/src/cli/pr_cmd_cards/cards.rs`
- `docs/tooling/FINISH_VALIDATION_PATH_OWNERSHIP_REGISTRY.md`
- `.github/workflows/ci.yaml`

## Target Files / Surfaces

- `AGENTS.md`
- adl/tools/validation_manager.py
- not/a/real/path.txt
"#;

        let paths = extract_declared_repo_paths(repo_root, prompt);
        assert!(paths.contains(&"AGENTS.md".to_string()));
        assert!(paths.contains(&"adl/src/cli/pr_cmd_cards/cards.rs".to_string()));
        assert!(paths.contains(&"adl/tools/validation_manager.py".to_string()));
        assert!(paths.contains(&".github/workflows/ci.yaml".to_string()));
        assert!(paths
            .contains(&"docs/tooling/FINISH_VALIDATION_PATH_OWNERSHIP_REGISTRY.md".to_string()));
        assert!(!paths.iter().any(|path| path.starts_with(".adl/")));
        assert!(!paths.iter().any(|path| path.starts_with("http")));
    }

    #[test]
    fn generated_vpp_plan_from_profile_json_carries_run_and_skip_rationale() {
        let profile = json!({
            "selected_profile": "selected_3_lane_profile",
            "status": "ready_to_run",
            "pr_publication_sufficient": true,
            "estimated_cost": {
                "runtime_class": "normal",
                "token_review_cost": "medium"
            },
            "run": [
                {
                    "lane_id": "docs_diff_check",
                    "command": "git diff --check",
                    "vpp_record": {
                        "parallel_group": "docs_hygiene"
                    }
                },
                {
                    "lane_id": "csdlc_owner_lane",
                    "command": "bash adl/tools/run_owner_validation_lane.sh csdlc"
                },
                {
                    "lane_id": "rust_pr_fast",
                    "command": "bash adl/tools/run_pr_fast_test_lane.sh --changed-files /private/tmp/changed-files.txt"
                }
            ],
            "not_run": [
                {
                    "surface": "full_workspace_nextest",
                    "reason": "not selected by validation profile"
                }
            ],
            "escalation": {
                "reasons": []
            }
        });

        let plan = generated_vpp_plan_from_profile_json(&profile);
        assert_eq!(
            plan.selected_lanes_inline,
            "docs_diff_check, csdlc_owner_lane, rust_pr_fast"
        );
        assert_eq!(plan.parallel_groups_inline, "docs_hygiene");
        assert_eq!(plan.validation_runtime_class, "normal");
        assert_eq!(plan.validation_resource_profile, "local");
        assert_eq!(plan.validation_family, "selected_3_lane_profile");
        assert_eq!(plan.validation_size_split, "mixed");
        assert_eq!(plan.expected_proof_cost, "medium");
        assert!(plan.validation_commands_inline.contains("git diff --check"));
        assert!(plan
            .validation_commands_inline
            .contains("bash adl/tools/run_pr_fast_test_lane.sh --changed-files .adl/generated-vpp-changed-files.txt"));
        assert!(!plan
            .validation_commands_inline
            .contains("/private/tmp/changed-files.txt"));
        assert!(plan
            .validation_commands_inline
            .contains("deferred full_workspace_nextest: not selected by validation profile"));
    }

    #[test]
    fn generated_vpp_plan_from_profile_json_records_escalation_when_not_runnable() {
        let profile = json!({
            "selected_profile": "escalated_profile",
            "status": "not_runnable",
            "pr_publication_sufficient": false,
            "estimated_cost": {
                "runtime_class": "escalated",
                "token_review_cost": "high"
            },
            "run": [],
            "not_run": [],
            "escalation": {
                "reasons": [
                    {
                        "lane_id": "release_gate_review",
                        "reason": "release-gate disposition required"
                    }
                ]
            }
        });

        let plan = generated_vpp_plan_from_profile_json(&profile);
        assert_eq!(plan.selected_lanes_inline, "none_selected");
        assert!(plan
            .validation_commands_inline
            .contains("validation profile `escalated_profile` is not runnable"));
        assert!(plan
            .validation_commands_inline
            .contains("escalation release_gate_review: release-gate disposition required"));
        assert!(plan
            .notes_risks_inline
            .contains("escalation remains explicit"));
    }
}
