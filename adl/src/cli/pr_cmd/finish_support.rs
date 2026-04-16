use anyhow::{bail, Context, Result};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use super::git_support::{
    commits_ahead_of_origin_main, current_branch, default_repo, ensure_not_on_main_branch,
    has_uncommitted_changes, path_str, primary_checkout_root, repo_root, run_capture, run_status,
    run_status_allow_failure,
};
use super::github::{
    attach_post_merge_closeout, attach_pr_janitor, current_pr_url,
    ensure_or_repair_pr_closing_linkage,
};
use super::lifecycle;
use super::DEFAULT_VERSION;
use crate::cli::pr_cmd_args::parse_finish_args;
use crate::cli::pr_cmd_cards::{path_relative_to_repo, validate_bootstrap_stp};
use crate::cli::pr_cmd_prompt::{
    resolve_issue_prompt_path, resolve_issue_scope_and_slug_from_local_state,
    validate_issue_prompt_exists,
};
use crate::cli::pr_cmd_validate::{
    validate_authored_prompt_surface, validate_milestone_doc_drift_for_finish, PromptSurfaceKind,
};
use ::adl::control_plane::{sanitize_slug, IssueRef};

pub(super) fn real_pr_finish(args: &[String]) -> Result<()> {
    let parsed = parse_finish_args(args)?;
    let repo_root = repo_root()?;
    let primary_root = primary_checkout_root()?;
    let repo = default_repo(&repo_root)?;

    ensure_not_on_main_branch(&repo_root)?;

    let branch = current_branch(&repo_root)?;
    if !branch.contains(&format!("/{issue}-", issue = parsed.issue)) {
        bail!(
            "finish: current branch '{}' does not look like it matches issue #{} (expected */{}-<slug>)",
            branch,
            parsed.issue,
            parsed.issue
        );
    }

    let _ = run_status_allow_failure("git", &["fetch", "origin"]);

    let inferred = resolve_issue_scope_and_slug_from_local_state(&repo_root, parsed.issue)?
        .unwrap_or((
            DEFAULT_VERSION.to_string(),
            format!("issue-{}", parsed.issue),
        ));
    let issue_ref = IssueRef::new(parsed.issue, inferred.0.clone(), inferred.1.clone())?;
    let source_path = resolve_issue_prompt_path(&primary_root, &issue_ref)?;
    let stp_path = issue_ref.task_bundle_stp_path(&repo_root);

    let input_path = parsed
        .input_path
        .clone()
        .unwrap_or_else(|| issue_ref.task_bundle_input_path(&repo_root));
    let output_path = parsed
        .output_path
        .clone()
        .unwrap_or_else(|| issue_ref.task_bundle_output_path(&repo_root));

    if !ensure_nonempty_file_path(&input_path)? {
        bail!("finish: missing input card: {}", input_path.display());
    }
    if !ensure_nonempty_file_path(&output_path)? {
        if !output_path.is_file() {
            bail!("finish: missing output card: {}", output_path.display());
        }
        bail!("finish: output card is empty: {}", output_path.display());
    }
    validate_issue_prompt_exists(&source_path)?;
    validate_bootstrap_stp(&repo_root, &source_path)?;
    validate_bootstrap_stp(&repo_root, &stp_path)?;
    validate_authored_prompt_surface("finish", &source_path, PromptSurfaceKind::IssuePrompt)?;
    validate_authored_prompt_surface("finish", &stp_path, PromptSurfaceKind::Stp)?;
    validate_authored_prompt_surface("finish", &input_path, PromptSurfaceKind::Sip)?;
    validate_completed_sor(&repo_root, &output_path)?;
    ensure_issue_surfaces_are_local_only(&repo_root, &primary_root, &issue_ref, &source_path)?;

    let canonical_output = lifecycle::sync_completed_output_surfaces(
        &repo_root,
        &primary_root,
        &issue_ref,
        &output_path,
    )?;
    if lifecycle::issue_is_closed_and_completed(parsed.issue, &repo)? {
        lifecycle::ensure_closed_completed_issue_bundle_truth(
            &primary_root,
            &issue_ref,
            &canonical_output,
        )
        .with_context(|| {
            format!(
                "finish: closed issue #{} has stale canonical sor truth; run closeout normalization before publication",
                parsed.issue
            )
        })?;
    }

    let finish_validation_mode = select_finish_validation_mode(&parsed.paths)?;
    if !parsed.no_checks {
        run_finish_validation_rust(&repo_root, finish_validation_mode)?;
    }

    stage_selected_paths_rust(&repo_root, &parsed.paths)?;
    ensure_no_staged_issue_bundle_mutations(&repo_root, &issue_ref)?;
    let has_uncommitted = has_uncommitted_changes(&repo_root)?;
    let ahead = commits_ahead_of_origin_main(&repo_root)?;
    if staged_diff_is_empty(&repo_root)? {
        if !has_uncommitted && ahead == 0 {
            bail!("No changes detected and branch has no commits ahead of origin/main. Nothing to PR.");
        }
    } else if !parsed.allow_gitignore && staged_gitignore_change_present(&repo_root)? {
        bail!(
            "finish: staged .gitignore or adl/.gitignore changes detected. Revert them or re-run with --allow-gitignore. Canonical .adl issue bundles are local-only and must not be staged."
        );
    }

    let changed_paths = finish_changed_paths(&repo_root, has_uncommitted)?;
    validate_milestone_doc_drift_for_finish(&repo_root, issue_ref.scope(), &changed_paths)?;

    let close_line = if parsed.no_close {
        None
    } else {
        Some(format!("Closes #{}", parsed.issue))
    };
    let default_validation = if parsed.no_checks {
        None
    } else {
        Some(render_default_finish_validation(finish_validation_mode))
    };
    let fingerprint = finish_inputs_fingerprint(
        &parsed.title,
        &parsed.paths,
        &path_relative_to_repo(&repo_root, &input_path),
        &path_relative_to_repo(&repo_root, &output_path),
    );
    let pr_body = render_pr_body(
        close_line.as_deref(),
        &input_path,
        &output_path,
        parsed.extra_body.as_deref(),
        default_validation.as_deref(),
        &fingerprint,
        &repo_root,
    )?;
    let pr_body_file = write_temp_markdown("pr_body", &pr_body)?;

    let commit_msg = if let Some(close) = &close_line {
        format!("{} ({close})", parsed.title)
    } else {
        parsed.title.clone()
    };

    if has_uncommitted {
        run_status(
            "git",
            &["-C", path_str(&repo_root)?, "commit", "-m", &commit_msg],
        )?;
    }

    let _ = run_status_allow_failure(
        "git",
        &["-C", path_str(&repo_root)?, "push", "origin", &branch],
    )?;

    let pr_url = if let Some(existing) = current_pr_url(&repo, &branch)? {
        run_status(
            "gh",
            &[
                "pr",
                "edit",
                "-R",
                &repo,
                &existing,
                "--title",
                &parsed.title,
                "--body-file",
                path_str(&pr_body_file)?,
            ],
        )?;
        existing
    } else {
        let created = run_capture(
            "gh",
            &[
                "pr",
                "create",
                "-R",
                &repo,
                "--base",
                "main",
                "--head",
                &branch,
                "--title",
                &parsed.title,
                "--body-file",
                path_str(&pr_body_file)?,
                "--draft",
            ],
        )?;
        created.trim().to_string()
    };

    let _closing_linkage_repaired = ensure_or_repair_pr_closing_linkage(
        &repo,
        &pr_url,
        parsed.issue,
        parsed.no_close,
        &pr_body_file,
    )?;

    if parsed.merge_mode {
        if parsed.ready {
            let _ = run_status_allow_failure("gh", &["pr", "ready", "-R", &repo, &pr_url])?;
        }
        run_status(
            "gh",
            &[
                "pr",
                "merge",
                "-R",
                &repo,
                "--squash",
                "--delete-branch",
                &pr_url,
            ],
        )?;
        lifecycle::wait_for_issue_closed_and_completed(parsed.issue, &repo)?;
        lifecycle::closeout_closed_completed_issue_bundle(
            &repo_root,
            &primary_root,
            &issue_ref,
            &output_path,
        )?;
        println!("{pr_url}");
        return Ok(());
    }

    if parsed.ready {
        let _ = run_status_allow_failure("gh", &["pr", "ready", "-R", &repo, &pr_url])?;
    }

    attach_pr_janitor(
        &repo_root,
        &repo,
        parsed.issue,
        &branch,
        &pr_url,
        if parsed.ready { "ready" } else { "draft" },
    )?;
    attach_post_merge_closeout(&repo_root, &repo, parsed.issue, &branch, &pr_url)?;

    if !parsed.no_open {
        let _ = run_status_allow_failure("open", &[&pr_url])?;
    }

    println!("{pr_url}");
    Ok(())
}

pub(super) fn finish_changed_paths(repo_root: &Path, has_uncommitted: bool) -> Result<Vec<String>> {
    let args = if has_uncommitted {
        vec![
            "-C",
            path_str(repo_root)?,
            "diff",
            "--cached",
            "--name-only",
            "--diff-filter=ACMR",
        ]
    } else {
        vec![
            "-C",
            path_str(repo_root)?,
            "diff",
            "--name-only",
            "--diff-filter=ACMR",
            "origin/main...HEAD",
        ]
    };
    let out = run_capture("git", &args)?;
    Ok(out
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(ToString::to_string)
        .collect())
}

pub(super) fn ensure_nonempty_file_path(path: &Path) -> Result<bool> {
    if !path.is_file() {
        return Ok(false);
    }
    let text = fs::read_to_string(path)?;
    Ok(!text.trim().is_empty())
}

pub(super) fn ensure_output_card_is_started(output_path: &Path) -> Result<()> {
    let text = fs::read_to_string(output_path)?;
    let normalized = text.replace("\r\n", "\n");
    for line in normalized.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("Status:") {
            let status = rest.trim();
            if status.eq_ignore_ascii_case("NOT_STARTED") {
                bail!(
                    "finish: output card is still bootstrap state (Status: NOT_STARTED): {}",
                    output_path.display()
                );
            }
            return Ok(());
        }
    }
    Ok(())
}

pub(super) fn validate_completed_sor(repo_root: &Path, output_path: &Path) -> Result<()> {
    ensure_output_card_is_started(output_path)?;
    let validator = repo_root.join("adl/tools/validate_structured_prompt.sh");
    run_status(
        "bash",
        &[
            path_str(&validator)?,
            "--type",
            "sor",
            "--phase",
            "completed",
            "--input",
            path_str(output_path)?,
        ],
    )
    .with_context(|| {
        format!(
            "finish: output card failed completed-phase validation: {}",
            output_path.display()
        )
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum FinishValidationMode {
    DocsOnly,
    FullRust,
}

fn finish_validation_guard(repo_root: &Path) -> Result<()> {
    let tracked_residue_guard =
        repo_root.join("adl/tools/check_no_tracked_adl_issue_record_residue.sh");
    run_status("bash", &[path_str(&tracked_residue_guard)?])
}

pub(super) fn select_finish_validation_mode(paths_csv: &str) -> Result<FinishValidationMode> {
    let paths = paths_csv
        .split(',')
        .map(str::trim)
        .filter(|path| !path.is_empty())
        .collect::<Vec<_>>();
    if paths.is_empty() {
        bail!("finish: --paths resolved to empty");
    }
    if paths.iter().all(|path| finish_path_is_docs_only(path)) {
        return Ok(FinishValidationMode::DocsOnly);
    }
    Ok(FinishValidationMode::FullRust)
}

fn finish_path_is_docs_only(path: &str) -> bool {
    let trimmed = path.trim().trim_matches('/');
    if trimmed.is_empty() {
        return false;
    }
    if trimmed == "docs" || trimmed.starts_with("docs/") {
        return true;
    }
    !trimmed.contains('/')
        && Path::new(trimmed)
            .extension()
            .and_then(|ext| ext.to_str())
            .is_some_and(|ext| ext.eq_ignore_ascii_case("md"))
}

pub(super) fn run_finish_validation_rust(
    repo_root: &Path,
    mode: FinishValidationMode,
) -> Result<()> {
    finish_validation_guard(repo_root)?;
    if mode == FinishValidationMode::DocsOnly {
        run_status("git", &["-C", path_str(repo_root)?, "diff", "--check"])?;
        return Ok(());
    }

    let manifest = repo_root.join("adl/Cargo.toml");
    run_status(
        "cargo",
        &[
            "fmt",
            "--manifest-path",
            path_str(&manifest)?,
            "--all",
            "--check",
        ],
    )?;
    run_status(
        "cargo",
        &[
            "clippy",
            "--manifest-path",
            path_str(&manifest)?,
            "--all-targets",
            "--",
            "-D",
            "warnings",
        ],
    )?;
    run_status("cargo", &["test", "--manifest-path", path_str(&manifest)?])?;
    Ok(())
}

pub(super) fn render_default_finish_validation(mode: FinishValidationMode) -> String {
    match mode {
        FinishValidationMode::DocsOnly => [
            "- bash adl/tools/check_no_tracked_adl_issue_record_residue.sh",
            "- git diff --check",
        ]
        .join("\n"),
        FinishValidationMode::FullRust => [
            "- bash adl/tools/check_no_tracked_adl_issue_record_residue.sh",
            "- cargo fmt",
            "- cargo clippy --all-targets -- -D warnings",
            "- cargo test",
        ]
        .join("\n"),
    }
}

pub(super) fn ensure_issue_surfaces_are_local_only(
    repo_root: &Path,
    primary_root: &Path,
    issue_ref: &IssueRef,
    source_path: &Path,
) -> Result<()> {
    let tracked = tracked_issue_surface_paths(repo_root, primary_root, issue_ref, source_path)?;
    if tracked.is_empty() {
        return Ok(());
    }
    bail!(
        "finish: canonical .adl issue surfaces must remain local-only; untrack these paths before publication: {}",
        tracked.join(", ")
    );
}

pub(super) fn tracked_issue_surface_paths(
    repo_root: &Path,
    primary_root: &Path,
    issue_ref: &IssueRef,
    source_path: &Path,
) -> Result<Vec<String>> {
    let mut tracked = BTreeSet::new();
    let root_stp = issue_ref.task_bundle_stp_path(primary_root);
    let root_input = issue_ref.task_bundle_input_path(primary_root);
    let root_output = issue_ref.task_bundle_output_path(primary_root);
    for path in [source_path, &root_stp, &root_input, &root_output] {
        let Some(repo_relative) = path
            .strip_prefix(primary_root)
            .ok()
            .map(|value| value.to_string_lossy().into_owned())
        else {
            continue;
        };
        if run_status_allow_failure(
            "git",
            &[
                "-C",
                path_str(repo_root)?,
                "ls-files",
                "--error-unmatch",
                "--",
                &repo_relative,
            ],
        )? {
            tracked.insert(repo_relative);
        }
    }
    Ok(tracked.into_iter().collect())
}

pub(super) fn stage_selected_paths_rust(repo_root: &Path, csv: &str) -> Result<()> {
    let paths = csv
        .split(',')
        .map(str::trim)
        .filter(|x| !x.is_empty())
        .collect::<Vec<_>>();
    if paths.is_empty() {
        bail!("finish: --paths resolved to empty");
    }
    let mut args = vec!["-C", path_str(repo_root)?, "add", "--"];
    args.extend(paths);
    run_status("git", &args)?;
    Ok(())
}

pub(super) fn ensure_no_staged_issue_bundle_mutations(
    repo_root: &Path,
    issue_ref: &IssueRef,
) -> Result<()> {
    let staged = run_capture(
        "git",
        &[
            "-C",
            path_str(repo_root)?,
            "diff",
            "--cached",
            "--name-status",
            "--find-renames",
            "--",
            ".adl",
        ],
    )?;
    let mut active_issue_paths = Vec::new();
    let mut foreign_issue_paths = Vec::new();
    for line in staged
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
    {
        let mut fields = line.split('\t');
        let status = fields.next().unwrap_or("");
        for path in fields {
            let Some(issue_number) = issue_bundle_issue_number_from_repo_relative(path) else {
                continue;
            };
            if status.starts_with('D') && issue_number != issue_ref.issue_number() {
                continue;
            }
            if issue_number == issue_ref.issue_number() {
                active_issue_paths.push(path.to_string());
            } else {
                foreign_issue_paths.push(path.to_string());
            }
        }
    }

    if !foreign_issue_paths.is_empty() {
        foreign_issue_paths.sort();
        foreign_issue_paths.dedup();
        bail!(
            "finish: staged .adl task-bundle changes for non-active issues detected: {}. Keep .adl bundles local-only and unstage them before publication.",
            foreign_issue_paths.join(", ")
        );
    }
    if !active_issue_paths.is_empty() {
        active_issue_paths.sort();
        active_issue_paths.dedup();
        bail!(
            "finish: staged canonical .adl issue-bundle paths detected: {}. Keep .adl bundles local-only and unstage them before publication.",
            active_issue_paths.join(", ")
        );
    }

    Ok(())
}

pub(super) fn issue_bundle_issue_number_from_repo_relative(path: &str) -> Option<u32> {
    let marker = "/tasks/issue-";
    let start = path.find(marker)? + marker.len();
    let digits = path[start..]
        .chars()
        .take_while(|ch| ch.is_ascii_digit())
        .collect::<String>();
    if digits.is_empty() {
        return None;
    }
    digits.parse().ok()
}

pub(super) fn staged_diff_is_empty(repo_root: &Path) -> Result<bool> {
    run_status_allow_failure(
        "git",
        &["-C", path_str(repo_root)?, "diff", "--cached", "--quiet"],
    )
}

pub(super) fn staged_gitignore_change_present(repo_root: &Path) -> Result<bool> {
    Ok(!run_status_allow_failure(
        "git",
        &[
            "-C",
            path_str(repo_root)?,
            "diff",
            "--cached",
            "--quiet",
            "--",
            ".gitignore",
            "adl/.gitignore",
        ],
    )?)
}

pub(super) fn extract_markdown_section(path: &Path, heading: &str) -> Result<String> {
    let text = fs::read_to_string(path)?;
    let marker = format!("## {heading}");
    let mut in_section = false;
    let mut out = Vec::new();
    for line in text.lines() {
        if line == marker {
            in_section = true;
            continue;
        }
        if in_section && line.starts_with("## ") {
            break;
        }
        if in_section {
            out.push(line);
        }
    }
    Ok(out.join("\n").trim().to_string())
}

pub(super) fn extra_pr_body_looks_like_issue_template(body: &str) -> bool {
    let lowered = body.to_lowercase();
    lowered.contains("issue_card_schema:")
        || lowered.contains("wp:")
        || lowered.contains("pr_start:")
        || lowered.contains("## goal")
        || lowered.contains("## deliverables")
        || lowered.contains("\n---\n")
}

pub(super) fn render_pr_body(
    close_line: Option<&str>,
    input_path: &Path,
    output_path: &Path,
    extra_body: Option<&str>,
    default_validation: Option<&str>,
    fingerprint: &str,
    repo_root: &Path,
) -> Result<String> {
    if let Some(extra) = extra_body {
        if extra_pr_body_looks_like_issue_template(extra) {
            bail!("finish: --body looks like issue-template/prompt text; use the output card as the PR summary source instead");
        }
    }

    let summary = extract_markdown_section(output_path, "Summary")?;
    let artifacts = extract_markdown_section(output_path, "Artifacts produced")?;
    let validation = extract_markdown_section(output_path, "Validation")?;
    let input_ref = path_relative_to_repo(repo_root, input_path);
    let output_ref = path_relative_to_repo(repo_root, output_path);

    let mut parts = Vec::new();
    if let Some(close) = close_line {
        parts.push(close.to_string());
        parts.push(String::new());
    }
    if !summary.is_empty() {
        parts.push("## Summary".to_string());
        parts.push(summary);
        parts.push(String::new());
    }
    if !artifacts.is_empty() {
        parts.push("## Artifacts".to_string());
        parts.push(artifacts);
        parts.push(String::new());
    }
    if !validation.is_empty() {
        parts.push("## Validation".to_string());
        parts.push(validation);
        parts.push(String::new());
    } else if let Some(default_validation) = default_validation {
        parts.push("## Validation".to_string());
        parts.push(default_validation.to_string());
        parts.push(String::new());
    }
    if let Some(extra) = extra_body {
        if !extra.trim().is_empty() {
            parts.push("## Notes".to_string());
            parts.push(extra.trim().to_string());
            parts.push(String::new());
        }
    }
    parts.push("## Local Artifacts".to_string());
    parts.push(format!("- Input card:  {input_ref}"));
    parts.push(format!("- Output card: {output_ref}"));
    parts.push(format!("- Idempotency-Key: {fingerprint}"));
    Ok(parts.join("\n"))
}

pub(super) fn finish_inputs_fingerprint(
    title: &str,
    paths: &str,
    input_ref: &str,
    output_ref: &str,
) -> String {
    let mut raw = String::new();
    raw.push_str(title);
    raw.push('|');
    raw.push_str(paths);
    raw.push('|');
    raw.push_str(input_ref);
    raw.push('|');
    raw.push_str(output_ref);
    sanitize_slug(&raw)
}

pub(super) fn write_temp_markdown(prefix: &str, body: &str) -> Result<PathBuf> {
    let mut path = std::env::temp_dir();
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("time")
        .as_nanos();
    path.push(format!("{prefix}-{nanos}.md"));
    fs::write(&path, body)?;
    Ok(path)
}
