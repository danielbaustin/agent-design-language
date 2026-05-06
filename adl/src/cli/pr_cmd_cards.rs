use anyhow::{anyhow, bail, Context, Result};
use chrono::Utc;
use serde_yaml::Value;
use std::fs;
#[cfg(unix)]
use std::os::unix::fs as unix_fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use super::pr_cmd_prompt::{
    infer_required_outcome_type, infer_workflow_queue, infer_wp_from_title, normalize_labels_csv,
    render_generated_issue_prompt,
};
use super::pr_cmd_validate::{
    bootstrap_stub_reason, placeholder_issue_body_reason, validate_authored_prompt_surface,
    PromptSurfaceKind,
};
use ::adl::control_plane::{
    card_input_path, card_output_path, card_plan_path, card_review_policy_path, card_stp_path,
    resolve_cards_root, IssueRef,
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
    let bundle_plan = issue_ref.task_bundle_plan_path(root);
    let bundle_review_policy = issue_ref.task_bundle_review_policy_path(root);
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
    if !bundle_plan.is_file() {
        write_plan_card(root, &bundle_plan, issue_ref, title, branch)?;
    }
    if !bundle_review_policy.is_file() {
        write_review_policy_card(root, &bundle_review_policy, issue_ref, title, branch)?;
    }

    let cards_root = resolve_cards_root(root, None);
    let compat_stp = card_stp_path(&cards_root, issue_ref.issue_number());
    let compat_input = card_input_path(&cards_root, issue_ref.issue_number());
    let compat_output = card_output_path(&cards_root, issue_ref.issue_number());
    let compat_plan = card_plan_path(&cards_root, issue_ref.issue_number());
    let compat_review_policy = card_review_policy_path(&cards_root, issue_ref.issue_number());
    ensure_symlink(&compat_stp, &bundle_stp)?;
    ensure_symlink(&compat_input, &bundle_input)?;
    ensure_symlink(&compat_output, &bundle_output)?;
    ensure_symlink(&compat_plan, &bundle_plan)?;
    ensure_symlink(&compat_review_policy, &bundle_review_policy)?;

    validate_bootstrap_cards(
        root,
        issue_ref.issue_number(),
        issue_ref.slug(),
        branch,
        &bundle_input,
        &bundle_output,
    )?;
    validate_authored_prompt_surface("start", &bundle_input, PromptSurfaceKind::Sip)?;
    validate_structured_bundle_artifacts(root, &bundle_plan, &bundle_review_policy)?;
    Ok((bundle_stp, bundle_input, bundle_output))
}

fn validate_structured_bundle_artifacts(
    repo_root: &Path,
    bundle_plan: &Path,
    bundle_review_policy: &Path,
) -> Result<()> {
    let validator = repo_root.join("adl/tools/validate_structured_prompt.sh");
    for (kind, path) in [("spp", bundle_plan), ("srp", bundle_review_policy)] {
        let output = Command::new("bash")
            .args([
                path_str(&validator)?,
                "--type",
                kind,
                "--input",
                path_str(path)?,
            ])
            .output()
            .with_context(|| "failed to spawn 'bash'")?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
            let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
            let detail = if !stderr.is_empty() {
                stderr
            } else if !stdout.is_empty() {
                stdout
            } else {
                format!("bash failed with status {:?}", output.status.code())
            };
            bail!("{kind}: failed validation: {}: {detail}", path.display());
        }
    }
    Ok(())
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
        bail!(
            "create: issue body cannot satisfy source-prompt validation: {err}; provide an authored body or use {}",
            init_template
        );
    }
    if let Some(reason) = placeholder_issue_body_reason(body) {
        bail!(
            "create: issue body is still bootstrap stub content ({reason}); provide an authored body or use {}",
            init_template
        );
    }
    Ok(())
}

pub(crate) fn validate_bootstrap_stp(repo_root: &Path, path: &Path) -> Result<()> {
    let validator = repo_root.join("adl/tools/validate_structured_prompt.sh");
    let output = Command::new("bash")
        .args([
            path_str(&validator)?,
            "--type",
            "stp",
            "--input",
            path_str(path)?,
        ])
        .output()
        .with_context(|| "failed to spawn 'bash'")?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
        let detail = if !stderr.is_empty() {
            stderr
        } else if !stdout.is_empty() {
            stdout
        } else {
            format!("bash failed with status {:?}", output.status.code())
        };
        bail!(
            "init: stp failed validation: {}: {}",
            path.display(),
            detail
        );
    }
    Ok(())
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
    let text = render_bootstrap_output_card(repo_root, issue_ref, title, branch);
    fs::write(path, text)?;
    Ok(())
}

pub(crate) fn write_plan_card(
    repo_root: &Path,
    path: &Path,
    issue_ref: &IssueRef,
    title: &str,
    branch: &str,
) -> Result<()> {
    let text = render_bootstrap_plan_card(repo_root, issue_ref, title, branch);
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
    let text = render_bootstrap_review_policy_card(repo_root, issue_ref, title, branch);
    fs::write(path, text)?;
    Ok(())
}

fn render_bootstrap_output_card(
    repo_root: &Path,
    issue_ref: &IssueRef,
    title: &str,
    branch: &str,
) -> String {
    let output_rel =
        path_relative_to_repo(repo_root, &issue_ref.task_bundle_output_path(repo_root));
    let timestamp = Utc::now().format("%Y-%m-%dT%H:%M:%SZ");
    format!(
        r#"# {slug}

Canonical Template Source: `adl/templates/cards/output_card_template.md`
Consumed by: `adl/tools/pr.sh` (`OUTPUT_TEMPLATE`) with legacy fallback support for `.adl/templates/output_card_template.md`.

Execution Record Requirements:
- The output card is a machine-auditable execution record.
- All sections must be fully populated. Empty sections, placeholders, or implicit claims are not allowed.
- Every command listed must include both what was run and what it verified.
- If something is not applicable, include a one-line justification.

Task ID: issue-{issue}
Run ID: issue-{issue}
Version: {version}
Title: {title}
Branch: {branch}
Status: IN_PROGRESS

Execution:
- Actor: issue-wave bootstrap
- Model: not_applicable
- Provider: not_applicable
- Start Time: {timestamp}
- End Time: {timestamp}

## Summary

Pre-run output scaffold initialized during issue-wave opening. No implementation has started yet.

## Artifacts produced
- Local ignored output-card scaffold at `{output_rel}`
- Tracked implementation artifacts: not_applicable until execution begins

## Actions taken
- Opened the local issue bundle and wrote a truthful pre-run output scaffold.
- Reserved the execution branch `{branch}` for later implementation.
- Deferred implementation, proof capture, and release integration to the execution lifecycle.

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: none
- Worktree-only paths remaining: no tracked implementation artifacts exist yet; execution-time proof surfaces will be established during implementation and PR publication
- Integration state: worktree_only
- Verification scope: main_repo
- Integration method used: direct write in main repo for the local ignored pre-run record; tracked implementation artifacts do not exist yet
- Verification performed:
  - `bash adl/tools/validate_structured_prompt.sh --type sor --phase bootstrap --input {output_rel}`
    Verified bootstrap SOR contract compliance for the local pre-run scaffold.
- Result: PASS

Rules:
- Final artifacts must exist in the main repository, not only in a worktree.
- Do not leave docs, code, or generated artifacts only under a `adl-wp-*` worktree.
- Prefer git-aware transfer into the main repo (`git checkout <branch> -- <path>` or commit + cherry-pick).
- If artifacts exist only in the worktree, the task is NOT complete.
- `Integration state` describes lifecycle state of the integrated artifact set, not where verification happened.
- `Verification scope` describes where the verification commands were run.
- `worktree_only` means at least one required path still exists only outside the main repository path.
- `pr_open` should pair with truthful `Worktree-only paths remaining` content; list those paths when they still exist only in the worktree or say `none` only when the branch contents are fully represented in the main repository path.
- If `Integration state` is `pr_open`, verify the actual proof artifacts rather than only the containing directory or card path.
- If `Integration method used` is `direct write in main repo`, `Verification scope` should normally be `main_repo` unless the deviation is explained.
- If `Verification scope` and `Integration method used` differ in a non-obvious way, explain the difference in one line.
- Completed output records must not leave `Status` as `NOT_STARTED`.
- By `pr finish`, `Status` should normally be `DONE` (or `FAILED` if the run failed and the record is documenting that failure).

## Validation
- Validation commands and their purpose:
  - `bash adl/tools/validate_structured_prompt.sh --type sor --phase bootstrap --input {output_rel}`
    Verified bootstrap SOR contract compliance for the local output scaffold.
- Results:
  - PASS

Validation command/path rules:
- Prefer repository-relative paths in recorded commands and artifact references.
- Do not record absolute host paths in output records unless they are explicitly required and justified.
- `absolute_path_leakage_detected: false` means the final recorded artifact does not contain unjustified absolute host paths.
- Do not list commands without describing their effect.

## Verification Summary

Rules:
- Replace the example values below with one actual final value per field.
- Do not leave pipe-delimited enum menus or placeholder text in a finished record.

```yaml
verification_summary:
  validation:
    status: PASS
    checks_run:
      - "bash adl/tools/validate_structured_prompt.sh --type sor --phase bootstrap --input {output_rel}"
  determinism:
    status: NOT_RUN
    replay_verified: unknown
    ordering_guarantees_verified: unknown
  security_privacy:
    status: PARTIAL
    secrets_leakage_detected: false
    prompt_or_tool_arg_leakage_detected: false
    absolute_path_leakage_detected: false
  artifacts:
    status: PASS
    required_artifacts_present: true
    schema_changes:
      present: false
      approved: not_applicable
```

## Determinism Evidence
- Determinism tests executed: not_run; bootstrap scaffold creation has not been replay-verified for this issue yet.
- Fixtures or scripts used: `adl/tools/pr.sh` issue-wave opening flow.
- Replay verification (same inputs -> same artifacts/order): not yet verified for this specific issue record.
- Ordering guarantees (sorting / tie-break rules used): not_applicable for a single-card bootstrap write.
- Artifact stability notes: repository-relative paths only; execution-time proof artifacts are not expected yet.

## Security / Privacy Checks
- Secret leakage scan performed: limited content review only; no secrets were intentionally recorded in the scaffold.
- Prompt / tool argument redaction verified: not_applicable for bootstrap scaffold generation.
- Absolute path leakage check: repository-relative paths only in the scaffold.
- Sandbox / policy invariants preserved: yes; local ignored issue-record path only.

## Replay Artifacts
- Trace bundle path(s): not_applicable until execution begins
- Run artifact root: not_applicable until execution begins
- Replay command used for verification: not_run
- Replay result: NOT_RUN

## Artifact Verification
- Primary proof surface: this local pre-run SOR scaffold and its bootstrap validation result
- Required artifacts present: local output card scaffold only; tracked implementation artifacts are not expected yet
- Artifact schema/version checks: bootstrap SOR validator passed
- Hash/byte-stability checks: not_run
- Missing/optional artifacts and rationale: execution proofs, demos, and tracked outputs are intentionally absent before implementation begins

## Decisions / Deviations
- Issue-wave opening emits a truthful pre-run SOR scaffold instead of leaving raw template residue for later cleanup.
- Integration state remains `worktree_only` until execution creates tracked artifacts or opens a PR.

## Follow-ups / Deferred work
- Update this record during execution with actual actions, validations, proof surfaces, and integration truth.
- Normalize this record to `pr_open`, `merged`, or `closed_no_pr` during finish/closeout as appropriate.
"#,
        slug = issue_ref.slug(),
        issue = issue_ref.padded_issue_number(),
        version = issue_ref.scope(),
        title = title,
        branch = branch,
        output_rel = output_rel,
        timestamp = timestamp,
    )
}

fn render_bootstrap_plan_card(
    repo_root: &Path,
    issue_ref: &IssueRef,
    title: &str,
    branch: &str,
) -> String {
    let stp_rel = path_relative_to_repo(repo_root, &issue_ref.task_bundle_stp_path(repo_root));
    let sip_rel = path_relative_to_repo(repo_root, &issue_ref.task_bundle_input_path(repo_root));
    let spp_rel = path_relative_to_repo(repo_root, &issue_ref.task_bundle_plan_path(repo_root));
    format!(
        r#"---
schema_version: "0.1"
artifact_type: "structured_planning_prompt"
name: "{slug}-execution-plan"
issue: {issue}
task_id: "issue-{issue_padded}"
run_id: "issue-{issue_padded}"
version: "{version}"
title: "{title}"
branch: "{branch}"
status: "draft"
plan_revision: 1
source_refs:
  - kind: "issue"
    ref: "https://github.com/{repo}/issues/{issue}"
  - kind: "stp"
    ref: "{stp_rel}"
  - kind: "sip"
    ref: "{sip_rel}"
scope:
  files:
    - "{stp_rel}"
    - "{sip_rel}"
  components:
    - "{slug}"
  out_of_scope:
    - "implementation beyond the approved issue scope"
constraints:
  - "read_only_until_execution_is_approved"
  - "no_hidden_scope_expansion"
confidence: "medium"
plan_summary: "Bootstrap planning surface for {title}; revise it before tracked execution if this issue needs an explicit reviewed execution plan."
assumptions:
  - "The linked STP and SIP remain the canonical issue-intent and execution-context inputs."
proposed_steps:
  - id: "step-1"
    description: "Review the linked STP and SIP, then tighten the planned execution sequence."
    expected_output: "{spp_rel}"
    allowed_mode: "execution_after_approval"
codex_plan:
  - step: "Review the issue bundle and tighten the planned execution sequence."
    status: "pending"
affected_areas:
  - "{slug}"
invariants_to_preserve:
  - "Do not claim implementation work inside the plan."
  - "Do not expand touched files or validation beyond issue-local evidence without recording why."
risks_and_edge_cases:
  - "The planned files or validations may drift if the issue prompt changes materially before execution."
test_strategy:
  - "Review the proposed validation commands before tracked execution."
execution_handoff: "Use this artifact as the durable plan-of-record when planning is required before execution."
required_permissions:
  - "workspace-write after execution approval"
stop_conditions:
  - "Stop and re-plan if the touched-file set or proving commands change materially."
alternatives_considered:
  - description: "Rely only on transient chat planning."
    reason_not_chosen: "Chat-only planning is not durable or reviewable enough for this workflow surface."
review_hooks:
  - "Check scope truthfulness, touched-file truthfulness, validation sufficiency, and re-plan triggers."
notes: "Bootstrap-generated SPP; revise before use if planning review is required."
---

# Structured Plan Prompt

## Plan Summary

Bootstrap planning surface for this issue. Tighten the plan before tracked execution if plan review is required.

## Codex Plan

1. [pending] Review the issue bundle and tighten the planned execution sequence.

## Assumptions

- The linked STP and SIP remain the canonical issue-local inputs.

## Proposed Steps

1. Review the linked STP and SIP, then tighten the planned execution sequence.

## Affected Areas

- {slug}

## Invariants To Preserve

- Do not claim implementation work inside the plan.
- Do not expand touched files or validations without recording why.

## Risks And Edge Cases

- The planned files or validations may drift if the issue prompt changes materially before execution.

## Test Strategy

- Review the proposed validation commands before tracked execution.

## Execution Handoff

Use this artifact as the durable plan-of-record when planning is required before execution.

## Stop Conditions

- Stop and re-plan if the touched-file set or proving commands change materially.

## Notes

Bootstrap-generated SPP; revise before use if planning review is required.
"#,
        slug = issue_ref.slug(),
        issue = issue_ref.issue_number(),
        issue_padded = issue_ref.padded_issue_number(),
        version = issue_ref.scope(),
        title = title,
        branch = branch,
        repo = default_repo(repo_root)
            .unwrap_or_else(|_| "danielbaustin/agent-design-language".to_string()),
        stp_rel = stp_rel,
        sip_rel = sip_rel,
        spp_rel = spp_rel,
    )
}

fn render_bootstrap_review_policy_card(
    repo_root: &Path,
    issue_ref: &IssueRef,
    title: &str,
    branch: &str,
) -> String {
    let stp_rel = path_relative_to_repo(repo_root, &issue_ref.task_bundle_stp_path(repo_root));
    let sip_rel = path_relative_to_repo(repo_root, &issue_ref.task_bundle_input_path(repo_root));
    let sor_rel = path_relative_to_repo(repo_root, &issue_ref.task_bundle_output_path(repo_root));
    format!(
        r#"---
schema_version: "0.1"
artifact_type: "structured_review_policy"
name: "{slug}-review-policy"
issue: {issue}
task_id: "issue-{issue_padded}"
version: "{version}"
title: "{title}"
branch: "{branch}"
status: "draft"
source_refs:
  - kind: "issue"
    ref: "https://github.com/{repo}/issues/{issue}"
  - kind: "stp"
    ref: "{stp_rel}"
  - kind: "sip"
    ref: "{sip_rel}"
  - kind: "sor"
    ref: "{sor_rel}"
review_mode: "pre_pr_independent_review"
timing: "before_pr_open"
scope_basis:
  - "{stp_rel}"
  - "{sip_rel}"
in_scope_surfaces:
  - "tracked changes for this issue branch"
evidence_policy:
  - "Use repository evidence, targeted validation output, and linked issue-bundle artifacts only."
validation_inputs:
  - "Issue-local proofs recorded in the SOR."
allowed_dispositions:
  - "PASS"
  - "BLOCK"
  - "NEEDS_FOLLOWUP"
reviewer_constraints:
  - "Do not widen issue scope."
  - "Do not merge, publish, or close the issue."
refusal_policy:
  - "Refuse claims that are unsupported by repository evidence."
  - "Refuse approving behavior outside the recorded issue scope."
follow_up_routing:
  - "Route actionable defects back to the issue branch before PR publication."
non_claims:
  - "This policy does not guarantee review quality by itself."
policy_refs:
  - "{stp_rel}"
  - "{sip_rel}"
notes: "Bootstrap-generated SRP; revise before use if the review policy needs issue-specific constraints."
---

# Structured Review Policy

## Review Summary

Use this policy to govern the independent pre-PR review for this issue.

## Scope Basis

- {stp_rel}
- {sip_rel}

## In-Scope Surfaces

- tracked changes for this issue branch

## Evidence Rules

- Use repository evidence, targeted validation output, and linked issue-bundle artifacts only.

## Validation Inputs

- Issue-local proofs recorded in the SOR.

## Allowed Dispositions

- PASS
- BLOCK
- NEEDS_FOLLOWUP

## Reviewer Constraints

- Do not widen issue scope.
- Do not merge, publish, or close the issue.

## Refusal Policy

- Refuse claims that are unsupported by repository evidence.
- Refuse approving behavior outside the recorded issue scope.

## Follow-up Routing

- Route actionable defects back to the issue branch before PR publication.

## Non-Claims

- This policy does not guarantee review quality by itself.

## Notes

Bootstrap-generated SRP; revise before use if the review policy needs issue-specific constraints.
"#,
        slug = issue_ref.slug(),
        issue = issue_ref.issue_number(),
        issue_padded = issue_ref.padded_issue_number(),
        version = issue_ref.scope(),
        title = title,
        branch = branch,
        repo = default_repo(repo_root)
            .unwrap_or_else(|_| "danielbaustin/agent-design-language".to_string()),
        stp_rel = stp_rel,
        sip_rel = sip_rel,
        sor_rel = sor_rel,
    )
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

pub(crate) fn validate_bootstrap_cards(
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
    )
    .with_context(|| {
        format!(
            "doctor: output card failed bootstrap validation: {}",
            output_path.display()
        )
    })?;
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

pub(crate) fn validate_bootstrap_output_card(
    repo_root: &Path,
    issue: u32,
    slug: &str,
    branch: &str,
    output_path: &Path,
) -> Result<()> {
    let validator = repo_root.join("adl/tools/validate_structured_prompt.sh");
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
    if field_line_value(output_path, "Task ID")? != expected {
        bail!("doctor: output card Task ID mismatch");
    }
    if field_line_value(output_path, "Run ID")? != expected {
        bail!("doctor: output card Run ID mismatch");
    }
    if field_line_value(output_path, "Branch")? != branch {
        bail!("doctor: output card branch mismatch");
    }
    if !output_card_title_matches_slug(output_path, slug)? {
        bail!("doctor: output card title mismatch");
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
