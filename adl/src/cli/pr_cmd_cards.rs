use anyhow::{anyhow, bail, Context, Result};
use serde_yaml::Value;
use std::fs;
#[cfg(unix)]
use std::os::unix::fs as unix_fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use super::pr_cmd_prompt::{
    infer_required_outcome_type, infer_wp_from_title, normalize_labels_csv,
    render_generated_issue_prompt,
};
use super::pr_cmd_validate::{
    bootstrap_stub_reason, placeholder_issue_body_reason, validate_authored_prompt_surface,
    PromptSurfaceKind,
};
use ::adl::control_plane::{
    card_input_path, card_output_path, card_stp_path, resolve_cards_root, IssueRef,
};

pub(crate) fn write_source_issue_prompt(
    repo_root: &Path,
    issue_ref: &IssueRef,
    title: &str,
    labels_csv: &str,
    issue_url: &str,
    body: &str,
) -> Result<PathBuf> {
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

pub(crate) fn ensure_task_bundle_stp(
    root: &Path,
    issue_ref: &IssueRef,
    source_path: &Path,
) -> Result<PathBuf> {
    let stp_path = issue_ref.task_bundle_stp_path(root);
    if !stp_path.is_file() {
        if let Some(parent) = stp_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::copy(source_path, &stp_path)?;
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

pub(crate) fn ensure_bootstrap_cards(
    root: &Path,
    issue_ref: &IssueRef,
    title: &str,
    branch: &str,
    source_path: &Path,
) -> Result<(PathBuf, PathBuf, PathBuf)> {
    let bundle_stp = issue_ref.task_bundle_stp_path(root);
    let bundle_input = issue_ref.task_bundle_input_path(root);
    let bundle_output = issue_ref.task_bundle_output_path(root);
    let bundle_stp_created = !bundle_stp.is_file();
    if let Some(parent) = bundle_input.parent() {
        fs::create_dir_all(parent)?;
    }
    if bundle_stp_created {
        validate_authored_prompt_surface("start", &bundle_stp, PromptSurfaceKind::Stp)?;
    }
    if !bundle_input.is_file()
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
    } else if field_line_value(&bundle_input, "Branch")?.trim() != branch {
        replace_field_line_in_file(&bundle_input, "Branch", branch)?;
    }
    if !bundle_output.is_file()
        || !output_card_title_matches_slug(&bundle_output, issue_ref.slug())?
    {
        write_output_card(root, &bundle_output, issue_ref, title, branch)?;
    } else if field_line_value(&bundle_output, "Branch")?.trim() != branch {
        replace_field_line_in_file(&bundle_output, "Branch", branch)?;
    }

    let cards_root = resolve_cards_root(root, None);
    let compat_stp = card_stp_path(&cards_root, issue_ref.issue_number());
    let compat_input = card_input_path(&cards_root, issue_ref.issue_number());
    let compat_output = card_output_path(&cards_root, issue_ref.issue_number());
    ensure_symlink(&compat_stp, &bundle_stp)?;
    ensure_symlink(&compat_input, &bundle_input)?;
    ensure_symlink(&compat_output, &bundle_output)?;

    validate_bootstrap_cards(
        root,
        issue_ref.issue_number(),
        issue_ref.slug(),
        branch,
        &bundle_input,
        &bundle_output,
    )?;
    validate_authored_prompt_surface("start", &bundle_input, PromptSurfaceKind::Sip)?;
    Ok((bundle_stp, bundle_input, bundle_output))
}

pub(crate) fn validate_issue_body_for_create(
    repo_root: &Path,
    title: &str,
    labels_csv: &str,
    slug: &str,
    body: &str,
) -> Result<()> {
    let probe_issue = 999_999;
    let probe_url = format!(
        "https://github.com/{}/issues/{probe_issue}",
        default_repo(repo_root)?
    );
    let prompt =
        render_issue_prompt_from_body(probe_issue, slug, title, labels_csv, &probe_url, body);
    let temp = write_temp_markdown("issue_body_probe", &prompt)?;
    validate_bootstrap_stp(repo_root, &temp)
        .with_context(|| "create: issue body cannot satisfy source-prompt validation")?;
    if let Some(reason) = placeholder_issue_body_reason(body) {
        bail!("create: issue body is still bootstrap stub content ({reason})");
    }
    Ok(())
}

pub(crate) fn validate_bootstrap_stp(repo_root: &Path, path: &Path) -> Result<()> {
    let validator = repo_root.join("adl/tools/validate_structured_prompt.sh");
    run_status(
        "bash",
        &[
            path_str(&validator)?,
            "--type",
            "stp",
            "--input",
            path_str(path)?,
        ],
    )
    .with_context(|| format!("init: stp failed validation: {}", path.display()))
}

pub(crate) fn validate_ready_cards(
    _repo_root: &Path,
    issue: u32,
    slug: &str,
    actual_branch: &str,
    input_path: &Path,
    output_path: &Path,
) -> Result<()> {
    let expected = format!("issue-{:04}", issue);
    if field_line_value(input_path, "Task ID")? != expected {
        bail!("ready: input card Task ID mismatch");
    }
    if field_line_value(input_path, "Run ID")? != expected {
        bail!("ready: input card Run ID mismatch");
    }
    if field_line_value(output_path, "Task ID")? != expected {
        bail!("ready: output card Task ID mismatch");
    }
    if field_line_value(output_path, "Run ID")? != expected {
        bail!("ready: output card Run ID mismatch");
    }
    if !branch_matches_started_state(&field_line_value(input_path, "Branch")?, actual_branch) {
        bail!("ready: input card branch mismatch");
    }
    if !branch_matches_started_state(&field_line_value(output_path, "Branch")?, actual_branch) {
        bail!("ready: output card branch mismatch");
    }
    if !output_card_title_matches_slug(output_path, slug)? {
        bail!("ready: output card title mismatch");
    }
    validate_authored_prompt_surface("ready", input_path, PromptSurfaceKind::Sip)?;
    Ok(())
}

pub(crate) fn validate_initialized_cards(
    issue: u32,
    slug: &str,
    input_path: &Path,
    output_path: &Path,
) -> Result<()> {
    let expected = format!("issue-{:04}", issue);
    if field_line_value(input_path, "Task ID")? != expected {
        bail!("doctor: input card Task ID mismatch");
    }
    if field_line_value(input_path, "Run ID")? != expected {
        bail!("doctor: input card Run ID mismatch");
    }
    if field_line_value(output_path, "Task ID")? != expected {
        bail!("doctor: output card Task ID mismatch");
    }
    if field_line_value(output_path, "Run ID")? != expected {
        bail!("doctor: output card Run ID mismatch");
    }
    if !output_card_title_matches_slug(output_path, slug)? {
        bail!("doctor: output card title mismatch");
    }
    validate_authored_prompt_surface("doctor", input_path, PromptSurfaceKind::Sip)?;
    Ok(())
}

pub(crate) fn field_line_value(path: &Path, label: &str) -> Result<String> {
    let prefix = format!("{label}:");
    let text = fs::read_to_string(path)?;
    for line in text.lines() {
        if let Some(rest) = line.strip_prefix(&prefix) {
            return Ok(rest.trim().to_string());
        }
    }
    Ok(String::new())
}

pub(crate) fn branch_indicates_unbound_state(recorded: &str) -> bool {
    let recorded = recorded.trim();
    recorded.is_empty()
        || recorded.eq_ignore_ascii_case("not bound yet")
        || recorded.starts_with("TBD (run pr.sh start ")
        || recorded.starts_with("TBD (run pr.sh run ")
}

pub(crate) fn ensure_source_issue_prompt(
    repo_root: &Path,
    repo: &str,
    issue_ref: &IssueRef,
    title: &str,
    labels_csv: Option<&str>,
    version: &str,
    default_new_labels: &str,
) -> Result<PathBuf> {
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

fn fetch_issue_body(repo: &str, issue: u32) -> Result<Option<String>> {
    let output = match Command::new("gh")
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

pub(crate) fn path_relative_to_repo(repo_root: &Path, path: &Path) -> String {
    path.strip_prefix(repo_root)
        .map(|relative| relative.display().to_string())
        .unwrap_or_else(|_| path.display().to_string())
}

fn render_issue_prompt_from_body(
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
    let outcome_type = infer_required_outcome_type(labels_csv, title);
    let label_lines = labels_csv
        .split(',')
        .map(str::trim)
        .filter(|label| !label.is_empty())
        .map(|label| format!("  - \"{label}\""))
        .collect::<Vec<_>>()
        .join("\n");
    format!(
        "---\nissue_card_schema: adl.issue.v1\nwp: \"{wp}\"\nslug: \"{slug}\"\ntitle: \"{title}\"\nlabels:\n{label_lines}\nissue_number: {issue}\nstatus: \"draft\"\naction: \"edit\"\ndepends_on: []\nmilestone_sprint: \"Pending sprint assignment\"\nrequired_outcome_type:\n  - \"{outcome_type}\"\nrepo_inputs: []\ncanonical_files: []\ndemo_required: false\ndemo_names: []\nissue_graph_notes:\n  - \"Mirrored from the authored GitHub issue body during bootstrap/init.\"\npr_start:\n  enabled: false\n  slug: \"{slug}\"\n---\n\n{body}\n"
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

fn copy_directory_contents(source: &Path, target: &Path) -> Result<()> {
    fs::create_dir_all(target)?;
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let source_path = entry.path();
        let target_path = target.join(entry.file_name());
        let file_type = entry.file_type()?;
        if file_type.is_dir() {
            copy_directory_contents(&source_path, &target_path)?;
        } else if file_type.is_file() {
            if let Some(parent) = target_path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(&source_path, &target_path)?;
        }
    }
    Ok(())
}

fn prompt_surface_is_bootstrap_stub(path: &Path, kind: PromptSurfaceKind) -> Result<bool> {
    if !path.is_file() {
        return Ok(false);
    }
    let text = fs::read_to_string(path)?;
    Ok(bootstrap_stub_reason(&text, kind).is_some())
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
    let mut text =
        fs::read_to_string(repo_root.join("adl/templates/cards/input_card_template.md"))?;
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
    replace_exact_line(
        &mut text,
        "- Issue:",
        &format!(
            "- Issue: https://github.com/{}/issues/{}",
            default_repo(repo_root)?,
            issue_ref.issue_number()
        ),
    );
    replace_exact_line(
        &mut text,
        "- Source Issue Prompt: <required repo-relative reference or URL>",
        &format!(
            "- Source Issue Prompt: {}",
            path_relative_to_repo(repo_root, source_path)
        ),
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
        &format!(
            "  output_card: {}",
            path_relative_to_repo(repo_root, output_path)
        ),
    );
    apply_input_card_lifecycle(&mut text, branch);
    fs::write(path, text)?;
    Ok(())
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
        "- Ship the required outcome type recorded in the linked source issue prompt.",
    );
    replace_exact_line(
        text,
        "- Preserve truthful lifecycle state until `pr run` binds the branch and worktree.",
        "- Keep the linked issue prompt, repository changes, and output record aligned.",
    );
    replace_exact_line(
        text,
        "- The linked source issue prompt is reviewable and structurally valid.",
        "- The implementation satisfies the linked source issue prompt.",
    );
    replace_exact_line(
        text,
        "- The card bundle does not imply a branch or worktree exists before `pr run`.",
        "- Validation and proof surfaces named below are completed or explicitly marked not applicable.",
    );
    remove_exact_line(
        text,
        "- Validation and proof expectations are recorded or explicitly marked not applicable.",
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
        "- files, docs, tests, commands, schemas, and artifacts named by the linked source issue prompt, once execution is bound",
        "- files, docs, tests, commands, schemas, and artifacts named by the linked source issue prompt",
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
    remove_exact_line(
        text,
        "- Reviewer checks: capture any manual review or demo checks in the output card after execution.",
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
        "- Refine this card if the linked source issue prompt changes materially before execution begins.",
        "- Refine this card if the linked source issue prompt changes materially before implementation begins.",
    );
    remove_exact_line(
        text,
        "- Do not create a branch or worktree from this card alone.",
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

pub(crate) fn write_output_card(
    repo_root: &Path,
    path: &Path,
    issue_ref: &IssueRef,
    title: &str,
    branch: &str,
) -> Result<()> {
    let mut text =
        fs::read_to_string(repo_root.join("adl/templates/cards/output_card_template.md"))?;
    replace_markdown_h1(&mut text, issue_ref.slug());
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
    replace_field_line(&mut text, "Status", "IN_PROGRESS");
    replace_exact_line(
        &mut text,
        "- Integration state: worktree_only | pr_open | merged",
        "- Integration state: worktree_only",
    );
    replace_exact_line(
        &mut text,
        "- Verification scope: worktree | pr_branch | main_repo",
        "- Verification scope: worktree",
    );
    fs::write(path, text)?;
    Ok(())
}

fn replace_markdown_h1(text: &mut String, value: &str) {
    let mut replaced = false;
    let mut out = Vec::new();
    for line in text.lines() {
        if !replaced && line.starts_with("# ") {
            out.push(format!("# {value}"));
            replaced = true;
        } else {
            out.push(line.to_string());
        }
    }
    *text = out.join("\n");
    if !text.ends_with('\n') {
        text.push('\n');
    }
}

fn output_card_title_matches_slug(path: &Path, slug: &str) -> Result<bool> {
    let expected = format!("# {slug}");
    let text = fs::read_to_string(path)?;
    let header = text
        .lines()
        .find(|line| line.starts_with("# "))
        .unwrap_or_default()
        .trim()
        .to_string();
    Ok(header == expected)
}

fn replace_field_line(text: &mut String, label: &str, value: &str) {
    let prefix = format!("{label}:");
    let mut out = Vec::new();
    for line in text.lines() {
        if line.starts_with(&prefix) {
            out.push(format!("{prefix} {value}"));
        } else {
            out.push(line.to_string());
        }
    }
    *text = out.join("\n");
    if !text.ends_with('\n') {
        text.push('\n');
    }
}

fn replace_exact_line(text: &mut String, from: &str, to: &str) {
    let mut out = Vec::new();
    for line in text.lines() {
        if line == from {
            out.push(to.to_string());
        } else {
            out.push(line.to_string());
        }
    }
    *text = out.join("\n");
    if !text.ends_with('\n') {
        text.push('\n');
    }
}

fn remove_exact_line(text: &mut String, target: &str) {
    let mut out = Vec::new();
    for line in text.lines() {
        if line != target {
            out.push(line.to_string());
        }
    }
    *text = out.join("\n");
    if !text.ends_with('\n') {
        text.push('\n');
    }
}

fn deduplicate_exact_line(text: &mut String, target: &str) {
    let mut seen = false;
    let mut out = Vec::new();
    for line in text.lines() {
        if line == target {
            if seen {
                continue;
            }
            seen = true;
        }
        out.push(line.to_string());
    }
    *text = out.join("\n");
    if !text.ends_with('\n') {
        text.push('\n');
    }
}

fn replace_field_line_in_file(path: &Path, label: &str, value: &str) -> Result<()> {
    let mut text = fs::read_to_string(path)?;
    replace_field_line(&mut text, label, value);
    fs::write(path, text)?;
    Ok(())
}

pub(crate) fn ensure_symlink(link_path: &Path, target: &Path) -> Result<()> {
    if let Some(parent) = link_path.parent() {
        fs::create_dir_all(parent)?;
    }
    if link_path.exists() || link_path.symlink_metadata().is_ok() {
        let _ = fs::remove_file(link_path);
    }
    #[cfg(unix)]
    {
        unix_fs::symlink(target, link_path)?;
    }
    #[cfg(not(unix))]
    {
        fs::copy(target, link_path)?;
    }
    Ok(())
}

fn validate_bootstrap_cards(
    repo_root: &Path,
    issue: u32,
    slug: &str,
    branch: &str,
    input_path: &Path,
    output_path: &Path,
) -> Result<()> {
    let validator = repo_root.join("adl/tools/validate_structured_prompt.sh");
    run_status(
        "bash",
        &[
            path_str(&validator)?,
            "--type",
            "sip",
            "--phase",
            "bootstrap",
            "--input",
            path_str(input_path)?,
        ],
    )?;
    run_status(
        "bash",
        &[
            path_str(&validator)?,
            "--type",
            "sor",
            "--phase",
            "bootstrap",
            "--input",
            path_str(output_path)?,
        ],
    )?;
    let expected = format!("issue-{:04}", issue);
    if field_line_value(input_path, "Task ID")? != expected {
        bail!("start: input card Task ID mismatch");
    }
    if field_line_value(input_path, "Run ID")? != expected {
        bail!("start: input card Run ID mismatch");
    }
    if field_line_value(output_path, "Task ID")? != expected {
        bail!("start: output card Task ID mismatch");
    }
    if field_line_value(output_path, "Run ID")? != expected {
        bail!("start: output card Run ID mismatch");
    }
    if field_line_value(input_path, "Branch")? != branch {
        bail!("start: input card branch mismatch");
    }
    if field_line_value(output_path, "Branch")? != branch {
        bail!("start: output card branch mismatch");
    }
    if !output_card_title_matches_slug(output_path, slug)? {
        bail!("start: output card title mismatch");
    }
    Ok(())
}

fn branch_matches_started_state(recorded: &str, actual_branch: &str) -> bool {
    let recorded = recorded.trim();
    if recorded == actual_branch {
        return true;
    }
    recorded.starts_with("TBD (run pr.sh start ")
}

fn default_repo(repo_root: &Path) -> Result<String> {
    let origin_url = run_capture_allow_failure(
        "git",
        &["-C", path_str(repo_root)?, "remote", "get-url", "origin"],
    )?
    .unwrap_or_default();
    if let Some(inferred) = infer_repo_from_remote(origin_url.trim()) {
        return Ok(inferred);
    }

    let inferred = run_capture_allow_failure(
        "gh",
        &[
            "repo",
            "view",
            "--json",
            "nameWithOwner",
            "--jq",
            ".nameWithOwner",
        ],
    )?
    .unwrap_or_default();
    let inferred = inferred.trim();
    if !inferred.is_empty() {
        return Ok(inferred.to_string());
    }

    let base = repo_root
        .file_name()
        .map(|value| value.to_string_lossy().to_string())
        .unwrap_or_else(|| "repo".to_string());
    Ok(format!("local/{base}"))
}

fn infer_repo_from_remote(url: &str) -> Option<String> {
    let trimmed = url.trim().trim_end_matches(".git");
    let marker = "github.com";
    let idx = trimmed.find(marker)?;
    let suffix = &trimmed[idx + marker.len()..];
    let suffix = suffix.trim_start_matches(':').trim_start_matches('/');
    let mut parts = suffix.split('/');
    let owner = parts.next()?;
    let repo = parts.next()?;
    Some(format!("{owner}/{repo}"))
}

fn path_str(path: &Path) -> Result<&str> {
    path.to_str()
        .ok_or_else(|| anyhow!("non-utf8 path: {}", path.display()))
}

fn run_capture_allow_failure(program: &str, args: &[&str]) -> Result<Option<String>> {
    let output = Command::new(program)
        .args(args)
        .output()
        .with_context(|| format!("failed to spawn '{program}'"))?;
    if output.status.success() {
        Ok(Some(String::from_utf8_lossy(&output.stdout).to_string()))
    } else {
        Ok(None)
    }
}

fn run_status(program: &str, args: &[&str]) -> Result<()> {
    let status = Command::new(program)
        .args(args)
        .status()
        .with_context(|| format!("failed to spawn '{program}'"))?;
    if !status.success() {
        bail!("{program} failed with status {:?}", status.code());
    }
    Ok(())
}

fn write_temp_markdown(prefix: &str, body: &str) -> Result<PathBuf> {
    let unique = format!(
        "{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos()
    );
    let path = std::env::temp_dir().join(format!("{prefix}-{unique}.md"));
    fs::write(&path, body)?;
    Ok(path)
}
