use anyhow::{bail, Context, Result};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::thread;
use std::time::{Duration, Instant};

use super::git_support::{
    branch_checked_out_worktree_path, commits_ahead_of_origin_main, commits_behind_origin_main,
    current_branch, default_repo, ensure_not_on_main_branch, has_uncommitted_changes, path_str,
    primary_checkout_root, repo_root, run_capture, run_capture_allow_failure, run_status,
    run_status_allow_failure,
};
use super::github::{
    attach_post_merge_closeout, attach_pr_janitor, current_pr_url,
    ensure_or_repair_pr_closing_linkage, pr_create_finish, pr_edit_finish_existing,
    pr_merge_finish, pr_ready_finish_allow_failure, pr_ready_finish_merge_allow_failure,
    pr_view_base_ref_finish_existing, wait_for_pr_validation_finish,
};
use super::lifecycle;
use super::DEFAULT_VERSION;
use crate::cli::observability::ProgressHeartbeat;
use crate::cli::pr_cmd_args::parse_finish_args;
use crate::cli::pr_cmd_cards::{
    path_relative_to_repo, validate_bootstrap_stp, validate_structured_artifact,
};
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

    let inferred = resolve_finish_issue_scope_and_slug(&repo_root, &primary_root, parsed.issue)?;
    let issue_ref = IssueRef::new(parsed.issue, inferred.0.clone(), inferred.1.clone())?;
    let expected_branch = issue_ref.branch_name("codex");
    ensure_finish_uses_bound_checkout(&repo_root, &expected_branch)?;
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

    let source_path =
        resolve_finish_source_issue_prompt_path(&repo_root, &primary_root, &issue_ref)?;
    ensure_finish_task_bundle_surfaces(&repo_root, &issue_ref)?;
    let stp_path = issue_ref.task_bundle_stp_path(&repo_root);

    let input_path = parsed
        .input_path
        .clone()
        .unwrap_or_else(|| issue_ref.task_bundle_input_path(&repo_root));
    let output_path = parsed
        .output_path
        .clone()
        .unwrap_or_else(|| issue_ref.task_bundle_output_path(&repo_root));
    let plan_path = issue_ref.task_bundle_plan_path(&repo_root);
    let review_policy_path = issue_ref.task_bundle_review_policy_path(&repo_root);

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
    validate_structured_artifact(&repo_root, "finish", &plan_path, "spp")?;
    validate_structured_artifact(&repo_root, "finish", &review_policy_path, "srp")?;
    ensure_issue_surfaces_are_local_only(&repo_root, &primary_root, &issue_ref, &source_path)?;

    normalize_sor_enum_aliases_for_finish(&output_path)?;

    stage_selected_paths_rust(&repo_root, &parsed.paths)?;
    ensure_no_staged_issue_bundle_mutations(&repo_root, &issue_ref)?;
    let has_uncommitted = has_uncommitted_changes(&repo_root)?;
    run_status("git", &["fetch", "origin", "main"])
        .context("finish: failed to fetch origin/main before stale-base guard")?;
    let ahead = commits_ahead_of_origin_main(&repo_root)?;
    ensure_finish_branch_not_behind_origin_main(&repo_root)?;
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
    let validation_changed_paths = if changed_paths.is_empty() {
        finish_declared_paths_for_validation(&parsed.paths)
    } else {
        changed_paths.clone()
    };
    validate_milestone_doc_drift_for_finish(
        &repo_root,
        issue_ref.scope(),
        &validation_changed_paths,
    )?;
    let finish_validation_plan =
        select_finish_validation_plan_for_finish(&parsed.paths, &validation_changed_paths)?;
    if !parsed.no_checks {
        run_finish_validation_rust(&repo_root, &finish_validation_plan)?;
        record_docs_only_validation_evidence_for_finish(&output_path, &finish_validation_plan)?;
    }
    validate_completed_sor(&repo_root, &output_path)?;

    let canonical_output = lifecycle::sync_completed_output_surfaces(
        &repo_root,
        &primary_root,
        &issue_ref,
        &output_path,
    )?;
    restage_finish_output_truth_paths(
        &repo_root,
        &primary_root,
        &issue_ref,
        &[output_path.clone(), canonical_output.clone()],
    )?;
    ensure_no_staged_issue_bundle_mutations(&repo_root, &issue_ref)?;
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

    let close_line = if parsed.no_close {
        None
    } else {
        Some(format!("Closes #{}", parsed.issue))
    };
    let default_validation = if parsed.no_checks {
        None
    } else {
        Some(render_default_finish_validation(&finish_validation_plan))
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

    let has_uncommitted_after_finish_truth = has_uncommitted_changes(&repo_root)?;
    if has_uncommitted_after_finish_truth {
        run_status(
            "git",
            &["-C", path_str(&repo_root)?, "commit", "-m", &commit_msg],
        )?;
    }

    let _ = run_status_allow_failure(
        "git",
        &["-C", path_str(&repo_root)?, "push", "origin", &branch],
    )?;

    let pr_base = resolve_finish_pr_base(&repo_root, &branch)?;

    let pr_url = if let Some(existing) = current_pr_url(&repo, &branch)? {
        ensure_existing_pr_base_matches(&repo, &existing, &pr_base)?;
        pr_edit_finish_existing(&repo, &existing, &parsed.title, &pr_body_file)?;
        existing
    } else {
        let created =
            pr_create_finish(&repo, &parsed.title, &branch, &pr_base, &pr_body_file, true)?;
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
            let _ = pr_ready_finish_merge_allow_failure(&repo, &pr_url)?;
        }
        wait_for_pr_validation_finish(&repo, &pr_url)?;
        pr_merge_finish(&repo, &pr_url)?;
        lifecycle::wait_for_issue_closed_and_completed(parsed.issue, &repo)?;
        lifecycle::closeout_closed_completed_issue_bundle(
            &repo_root,
            &primary_root,
            &issue_ref,
            &canonical_output,
        )?;
        println!("{pr_url}");
        return Ok(());
    }

    if parsed.ready {
        let _ = pr_ready_finish_allow_failure(&repo, &pr_url)?;
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

    println!("{pr_url}");
    if !parsed.no_open {
        let open_result = open_pr_url_nonblocking("open", &pr_url);
        if !open_result.success {
            eprintln!("{}", open_result.warning);
        }
    }
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct OpenPrUrlResult {
    pub(super) success: bool,
    pub(super) warning: String,
}

const PR_URL_OPENER_TIMEOUT: Duration = Duration::from_secs(5);

pub(super) fn open_pr_url_nonblocking(opener: &str, pr_url: &str) -> OpenPrUrlResult {
    open_pr_url_nonblocking_with_timeout(opener, pr_url, PR_URL_OPENER_TIMEOUT)
}

pub(super) fn open_pr_url_nonblocking_with_timeout(
    opener: &str,
    pr_url: &str,
    timeout: Duration,
) -> OpenPrUrlResult {
    let mut child = match Command::new(opener)
        .arg(pr_url)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
    {
        Ok(child) => child,
        Err(err) => {
            return OpenPrUrlResult {
                success: false,
                warning: format!(
                    "warning: local PR URL opener could not start; PR publication already succeeded. Open manually: {pr_url} ({err})"
                ),
            }
        }
    };

    let started = Instant::now();
    loop {
        match child.try_wait() {
            Ok(Some(_status)) => break,
            Ok(None) if started.elapsed() < timeout => {
                thread::sleep(Duration::from_millis(50));
            }
            Ok(None) => {
                let _ = child.kill();
                let _ = child.wait();
                return OpenPrUrlResult {
                    success: false,
                    warning: format!(
                        "warning: local PR URL opener timed out after {}s; PR publication already succeeded. Open manually: {pr_url}",
                        timeout.as_secs()
                    ),
                };
            }
            Err(err) => {
                return OpenPrUrlResult {
                    success: false,
                    warning: format!(
                        "warning: local PR URL opener could not be checked; PR publication already succeeded. Open manually: {pr_url} ({err})"
                    ),
                };
            }
        }
    }

    match child.wait_with_output() {
        Ok(output) if output.status.success() => OpenPrUrlResult {
            success: true,
            warning: String::new(),
        },
        Ok(output) => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let detail = stderr.trim();
            let status = output
                .status
                .code()
                .map(|code| code.to_string())
                .unwrap_or_else(|| "terminated_by_signal".to_string());
            OpenPrUrlResult {
                success: false,
                warning: format!(
                    "warning: local PR URL opener failed with status {status}; PR publication already succeeded. Open manually: {pr_url}{}",
                    if detail.is_empty() {
                        String::new()
                    } else {
                        format!(" ({detail})")
                    }
                ),
            }
        }
        Err(err) => OpenPrUrlResult {
            success: false,
            warning: format!(
                "warning: local PR URL opener could not complete; PR publication already succeeded. Open manually: {pr_url} ({err})"
            ),
        },
    }
}

pub(super) fn ensure_finish_branch_not_behind_origin_main(repo_root: &Path) -> Result<()> {
    let behind = commits_behind_origin_main(repo_root)?;
    if behind == 0 {
        return Ok(());
    }
    bail!(
        "finish: branch is behind origin/main by {behind} commit(s); rebase before publication to avoid stale-base validation and coverage-impact false positives. Suggested recovery: git fetch origin main && git rebase origin/main --autostash, then rerun pr finish from the bound issue worktree."
    )
}

pub(super) fn resolve_finish_issue_scope_and_slug(
    repo_root: &Path,
    primary_root: &Path,
    issue: u32,
) -> Result<(String, String)> {
    if let Some(identity) = resolve_issue_scope_and_slug_from_local_state(repo_root, issue)? {
        return Ok(identity);
    }
    if primary_root != repo_root {
        if let Some(identity) = resolve_issue_scope_and_slug_from_local_state(primary_root, issue)?
        {
            return Ok(identity);
        }
    }
    Ok((DEFAULT_VERSION.to_string(), format!("issue-{issue}")))
}

pub(super) fn resolve_finish_source_issue_prompt_path(
    repo_root: &Path,
    primary_root: &Path,
    issue_ref: &IssueRef,
) -> Result<PathBuf> {
    if let Ok(path) = resolve_issue_prompt_path(repo_root, issue_ref) {
        return Ok(path);
    }
    if primary_root != repo_root {
        return resolve_issue_prompt_path(primary_root, issue_ref);
    }
    resolve_issue_prompt_path(repo_root, issue_ref)
}

pub(super) fn ensure_finish_task_bundle_surfaces(
    repo_root: &Path,
    issue_ref: &IssueRef,
) -> Result<()> {
    let canonical_dir = issue_ref.task_bundle_dir_path(repo_root);
    let candidate_dirs = lifecycle::matching_task_bundle_dirs(repo_root, issue_ref)?;
    let required_files = ["stp.md", "sip.md", "sor.md", "spp.md", "srp.md"];
    let mut copied_any = false;

    for file_name in required_files {
        let canonical_path = canonical_dir.join(file_name);
        if ensure_nonempty_file_path(&canonical_path)? {
            continue;
        }
        let source_path = candidate_dirs
            .iter()
            .filter(|candidate| *candidate != &canonical_dir)
            .map(|candidate| candidate.join(file_name))
            .find(|candidate| ensure_nonempty_file_path(candidate).unwrap_or(false));
        if let Some(source_path) = source_path {
            fs::create_dir_all(&canonical_dir).with_context(|| {
                format!(
                    "finish: failed to create canonical task bundle {}",
                    canonical_dir.display()
                )
            })?;
            fs::copy(&source_path, &canonical_path).with_context(|| {
                format!(
                    "finish: failed to restore canonical {} from slug-drifted task bundle {}",
                    canonical_path.display(),
                    source_path.display()
                )
            })?;
            copied_any = true;
        }
    }

    if copied_any {
        eprintln!(
            "finish: restored missing canonical task-bundle cards for #{} from matching issue bundle",
            issue_ref.issue_number()
        );
    }
    Ok(())
}

fn ensure_finish_uses_bound_checkout(repo_root: &Path, branch: &str) -> Result<()> {
    let Some(bound_worktree) = branch_checked_out_worktree_path(branch)? else {
        return Ok(());
    };
    if same_checkout_root(repo_root, &bound_worktree)? {
        return Ok(());
    }
    bail!(
        "finish: mismatched_publication_surface: branch '{}' is bound to worktree '{}', but finish is running from '{}'. Rerun finish from the bound issue worktree instead of publishing from the primary checkout or another checkout.",
        branch,
        bound_worktree.display(),
        repo_root.display()
    );
}

fn same_checkout_root(left: &Path, right: &Path) -> Result<bool> {
    if left == right {
        return Ok(true);
    }
    let left = fs::canonicalize(left).with_context(|| {
        format!(
            "finish: failed to canonicalize checkout '{}'",
            left.display()
        )
    })?;
    let right = fs::canonicalize(right).with_context(|| {
        format!(
            "finish: failed to canonicalize checkout '{}'",
            right.display()
        )
    })?;
    Ok(left == right)
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

pub(super) fn finish_declared_paths_for_validation(paths: &str) -> Vec<String> {
    paths
        .split(',')
        .map(str::trim)
        .filter(|path| !path.is_empty())
        .map(ToString::to_string)
        .collect()
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

fn normalize_sor_enum_aliases_for_finish(output_path: &Path) -> Result<()> {
    let original = fs::read_to_string(output_path).with_context(|| {
        format!(
            "finish: failed to read output card {}",
            output_path.display()
        )
    })?;
    let normalized = normalize_docs_only_sor_aliases(&original);
    if normalized != original {
        fs::write(output_path, normalized).with_context(|| {
            format!(
                "finish: failed to write normalized docs-only output card {}",
                output_path.display()
            )
        })?;
    }
    Ok(())
}

fn record_docs_only_validation_evidence_for_finish(
    output_path: &Path,
    plan: &FinishValidationPlan,
) -> Result<()> {
    if plan.mode != FinishValidationMode::DocsOnly {
        return Ok(());
    }
    let original = fs::read_to_string(output_path).with_context(|| {
        format!(
            "finish: failed to read output card {}",
            output_path.display()
        )
    })?;
    let normalized = normalize_docs_only_validation_evidence(&original, &plan.commands);
    if normalized != original {
        fs::write(output_path, normalized).with_context(|| {
            format!(
                "finish: failed to write docs-only validation evidence into output card {}",
                output_path.display()
            )
        })?;
    }
    Ok(())
}

fn restage_finish_output_truth_paths(
    repo_root: &Path,
    primary_root: &Path,
    issue_ref: &IssueRef,
    paths: &[PathBuf],
) -> Result<()> {
    let worktree_bundle_dir = issue_ref.task_bundle_dir_path(repo_root);
    let primary_bundle_dir = issue_ref.task_bundle_dir_path(primary_root);
    let mut relpaths = BTreeSet::new();

    for path in paths {
        let resolved = if path.is_absolute() {
            path.clone()
        } else {
            repo_root.join(path)
        };
        if resolved.starts_with(&worktree_bundle_dir) || resolved.starts_with(&primary_bundle_dir) {
            continue;
        }
        if !resolved.exists() {
            continue;
        }
        if let Ok(relpath) = resolved.strip_prefix(repo_root) {
            relpaths.insert(relpath.to_string_lossy().into_owned());
        }
    }

    for relpath in relpaths {
        run_status("git", &["-C", path_str(repo_root)?, "add", "--", &relpath]).with_context(
            || format!("finish: failed to re-stage finish-written output path '{relpath}'"),
        )?;
    }

    Ok(())
}

#[cfg(test)]
pub(super) fn normalize_docs_only_sor_text(text: &str, commands: &[String]) -> String {
    let mut updated = normalize_docs_only_sor_aliases(text);
    updated = normalize_docs_only_validation_evidence(&updated, commands);
    updated
}

fn normalize_docs_only_sor_aliases(text: &str) -> String {
    let mut updated = text.to_string();
    updated = normalize_sor_enum_line(
        &updated,
        "- Integration state:",
        normalize_integration_state_alias,
    );
    updated = normalize_sor_enum_line(
        &updated,
        "- Verification scope:",
        normalize_verification_scope_alias,
    );
    updated
}

fn normalize_docs_only_validation_evidence(text: &str, commands: &[String]) -> String {
    let mut updated = text.to_string();
    updated = ensure_docs_only_validation_entries(&updated, commands);
    updated = ensure_docs_only_checks_run_entries(&updated, commands);
    updated
}

fn normalize_sor_enum_line(
    text: &str,
    prefix: &str,
    normalizer: fn(&str) -> Option<&'static str>,
) -> String {
    let mut changed = false;
    let lines = text
        .lines()
        .map(|line| {
            let trimmed = line.trim_start();
            if let Some(value) = trimmed.strip_prefix(prefix) {
                if let Some(normalized) = normalizer(value.trim()) {
                    let indent_len = line.len() - trimmed.len();
                    let indent = &line[..indent_len];
                    let rewritten = format!("{indent}{prefix} {normalized}");
                    if rewritten != line {
                        changed = true;
                        return rewritten;
                    }
                }
            }
            line.to_string()
        })
        .collect::<Vec<_>>();
    if changed {
        format!("{}\n", lines.join("\n"))
    } else {
        text.to_string()
    }
}

fn normalize_integration_state_alias(value: &str) -> Option<&'static str> {
    match value.trim().to_ascii_lowercase().as_str() {
        "worktree-only" | "worktree only" | "worktree" => Some("worktree_only"),
        "pr" | "open_pr" | "open-pr" | "pr-ready" | "pr_ready" => Some("pr_open"),
        "closed-no-pr" | "closed_no_pr" | "no-pr" | "no_pr" => Some("closed_no_pr"),
        "merged-pr" | "merged_pr" => Some("merged"),
        _ => None,
    }
}

fn normalize_verification_scope_alias(value: &str) -> Option<&'static str> {
    match value.trim().to_ascii_lowercase().as_str() {
        "main" | "main repo" | "main-repo" | "repo" => Some("main_repo"),
        "pr" | "pr branch" | "pr-branch" | "branch" => Some("pr_branch"),
        "worktree-only" | "worktree_only" => Some("worktree"),
        _ => None,
    }
}

fn ensure_docs_only_validation_entries(text: &str, commands: &[String]) -> String {
    let Some(validation_body) = markdown_section_body_local(text, "Validation") else {
        return text.to_string();
    };
    let replacement = ensure_command_entries_before_marker(
        &validation_body,
        commands,
        "- Results:",
        render_docs_only_validation_entry,
    );
    replace_markdown_section_body(text, "Validation", &replacement)
        .unwrap_or_else(|_| text.to_string())
}

fn ensure_docs_only_checks_run_entries(text: &str, commands: &[String]) -> String {
    let Some(summary_body) = markdown_section_body_local(text, "Verification Summary") else {
        return text.to_string();
    };
    let replacement = ensure_yaml_checks_run_entries(&summary_body, commands);
    replace_markdown_section_body(text, "Verification Summary", &replacement)
        .unwrap_or_else(|_| text.to_string())
}

fn ensure_command_entries_before_marker(
    body: &str,
    commands: &[String],
    marker: &str,
    render_entry: fn(&str) -> String,
) -> String {
    let mut lines = body.lines().map(str::to_string).collect::<Vec<_>>();
    let insert_at = lines
        .iter()
        .position(|line| line.trim_start() == marker)
        .unwrap_or(lines.len());
    let mut insertion = Vec::new();
    for command in commands {
        let needle = format!("`{command}`");
        if body.contains(&needle) {
            continue;
        }
        insertion.extend(render_entry(command).lines().map(str::to_string));
    }
    if insertion.is_empty() {
        return body.to_string();
    }
    if insert_at > 0 && !lines[insert_at - 1].trim().is_empty() {
        insertion.insert(0, String::new());
    }
    lines.splice(insert_at..insert_at, insertion);
    trim_trailing_blank_lines(lines).join("\n")
}

fn render_docs_only_validation_entry(command: &str) -> String {
    format!(
        "  - `{command}`\n    {}",
        docs_only_command_description(command)
    )
}

fn docs_only_command_description(command: &str) -> &'static str {
    match command {
        "bash adl/tools/check_no_tracked_adl_issue_record_residue.sh" => {
            "Verified no tracked local-only ADL issue-record residue remained staged for publication."
        }
        "git diff --check" => {
            "Verified whitespace and patch hygiene on the docs-only changed surfaces."
        }
        _ => "Verified the docs-only finish validation command completed successfully.",
    }
}

fn ensure_yaml_checks_run_entries(body: &str, commands: &[String]) -> String {
    let mut lines = body.lines().map(str::to_string).collect::<Vec<_>>();
    let Some(checks_run_idx) = lines.iter().position(|line| line.trim() == "checks_run:") else {
        return body.to_string();
    };
    let list_end = lines
        .iter()
        .enumerate()
        .skip(checks_run_idx + 1)
        .find(|(_, line)| line.starts_with("  ") && !line.starts_with("      - "))
        .map(|(idx, _)| idx)
        .unwrap_or(lines.len());
    let mut insert_at = list_end;
    for command in commands {
        let rendered = format!("      - \"{command}\"");
        if lines.iter().any(|line| line.trim() == rendered.trim()) {
            continue;
        }
        lines.insert(insert_at, rendered);
        insert_at += 1;
    }
    trim_trailing_blank_lines(lines).join("\n")
}

fn replace_markdown_section_body(input: &str, heading: &str, replacement: &str) -> Result<String> {
    let lines = input.lines().map(str::to_string).collect::<Vec<_>>();
    let mut in_fence = false;
    let mut start = None;
    let mut target_depth = None;
    let mut end = lines.len();

    for (idx, line) in lines.iter().enumerate() {
        if line.trim_start().starts_with("```") {
            in_fence = !in_fence;
            continue;
        }
        if in_fence {
            continue;
        }
        if let Some((depth, text)) = parse_heading_line(line) {
            if start.is_none() && text == heading.trim() {
                start = Some(idx);
                target_depth = Some(depth);
                continue;
            }
            if start.is_some() && depth <= target_depth.expect("target depth") {
                end = idx;
                break;
            }
        }
    }

    let start = start.ok_or_else(|| anyhow::anyhow!("heading '{heading}' not found"))?;
    let mut out = Vec::new();
    out.extend_from_slice(&lines[..=start]);
    out.push(String::new());
    if !replacement.trim().is_empty() {
        out.extend(replacement.trim().lines().map(str::to_string));
        out.push(String::new());
    }
    out.extend_from_slice(&lines[end..]);
    Ok(format!("{}\n", trim_trailing_blank_lines(out).join("\n")))
}

fn markdown_section_body_local(text: &str, heading: &str) -> Option<String> {
    let lines = text.lines().collect::<Vec<_>>();
    let mut in_fence = false;
    let mut start = None;
    let mut target_depth = None;
    let mut end = lines.len();

    for (idx, line) in lines.iter().enumerate() {
        if line.trim_start().starts_with("```") {
            in_fence = !in_fence;
            continue;
        }
        if in_fence {
            continue;
        }
        if let Some((depth, text)) = parse_heading_line(line) {
            if start.is_none() && text == heading.trim() {
                start = Some(idx + 1);
                target_depth = Some(depth);
                continue;
            }
            if start.is_some() && depth <= target_depth.expect("target depth") {
                end = idx;
                break;
            }
        }
    }

    let start = start?;
    Some(lines[start..end].join("\n").trim().to_string())
}

fn parse_heading_line(line: &str) -> Option<(usize, String)> {
    let trimmed = line.trim_start();
    let hashes = trimmed.chars().take_while(|ch| *ch == '#').count();
    if !(1..=6).contains(&hashes) {
        return None;
    }
    let rest = trimmed.get(hashes..)?;
    if !rest.starts_with(' ') {
        return None;
    }
    Some((hashes, rest.trim().to_string()))
}

fn trim_trailing_blank_lines(mut lines: Vec<String>) -> Vec<String> {
    while lines.last().is_some_and(|line| line.trim().is_empty()) {
        lines.pop();
    }
    lines
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum FinishValidationMode {
    DocsOnly,
    SmallBinaryFocused,
    LargerBinaryFocused,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct FinishValidationPlan {
    pub mode: FinishValidationMode,
    pub commands: Vec<String>,
}

fn finish_validation_guard(repo_root: &Path) -> Result<()> {
    let tracked_residue_guard =
        repo_root.join("adl/tools/check_no_tracked_adl_issue_record_residue.sh");
    run_status("bash", &[path_str(&tracked_residue_guard)?])
}

pub(super) fn select_finish_validation_plan(paths_csv: &str) -> Result<FinishValidationPlan> {
    let paths = paths_csv
        .split(',')
        .map(str::trim)
        .filter(|path| !path.is_empty())
        .collect::<Vec<_>>();
    if paths.is_empty() {
        bail!("finish: --paths resolved to empty");
    }
    if paths.iter().all(|path| finish_path_is_docs_only(path)) {
        return Ok(FinishValidationPlan {
            mode: FinishValidationMode::DocsOnly,
            commands: vec![
                "bash adl/tools/check_no_tracked_adl_issue_record_residue.sh".to_string(),
                "git diff --check".to_string(),
            ],
        });
    }
    let unsupported_paths = paths
        .iter()
        .copied()
        .filter(|path| {
            !finish_path_is_docs_only(path)
                && !finish_path_is_small_binary_focused(path)
                && !finish_path_is_larger_binary_focused(path)
        })
        .collect::<Vec<_>>();
    if !unsupported_paths.is_empty() {
        bail!(
            "finish: changed paths are not classified into docs-only, small-binary focused, or larger-binary focused validation lanes: {}",
            unsupported_paths.join(", ")
        );
    }
    if paths.iter().all(|path| {
        finish_path_is_docs_only(path)
            || finish_path_is_small_binary_focused(path)
            || finish_path_is_larger_binary_focused(path)
    }) {
        let mut commands = vec![
            "bash adl/tools/check_no_tracked_adl_issue_record_residue.sh".to_string(),
            "git diff --check".to_string(),
        ];
        let mut mode = FinishValidationMode::SmallBinaryFocused;
        if paths
            .iter()
            .any(|path| finish_path_needs_pr_finish_rust_focused_validation(path))
        {
            push_finish_validation_command(
                &mut commands,
                "cargo fmt --manifest-path adl/Cargo.toml --all --check",
            );
            push_finish_validation_command(
                &mut commands,
                "cargo test --manifest-path adl/Cargo.toml --bin adl-pr-finish cli::pr_cmd::tests::finish::arg_render::finish_validation",
            );
            push_finish_validation_command(
                &mut commands,
                "cargo test --manifest-path adl/Cargo.toml --bin adl-pr-finish cli::pr_cmd::tests::finish::arg_render::finish_helper_paths_run_focused_local_ci_gated_validation",
            );
        }
        if paths
            .iter()
            .any(|path| finish_path_needs_pr_cmd_lifecycle_focused_validation(path))
        {
            mode = FinishValidationMode::LargerBinaryFocused;
            push_finish_validation_command(
                &mut commands,
                "cargo fmt --manifest-path adl/Cargo.toml --all --check",
            );
            push_finish_validation_command(
                &mut commands,
                "cargo test --manifest-path adl/Cargo.toml --bin adl cli::pr_cmd",
            );
        }
        if paths
            .iter()
            .any(|path| finish_path_needs_github_token_focused_validation(path))
        {
            mode = FinishValidationMode::LargerBinaryFocused;
            push_finish_validation_command(
                &mut commands,
                "cargo fmt --manifest-path adl/Cargo.toml --all --check",
            );
            push_finish_validation_command(
                &mut commands,
                "cargo test --manifest-path adl/Cargo.toml --bin adl github_token",
            );
            push_finish_validation_command(
                &mut commands,
                "cargo test --manifest-path adl/Cargo.toml --bin adl github_client",
            );
            push_finish_validation_command(
                &mut commands,
                "cargo test --manifest-path adl/Cargo.toml --bin adl github_release_octocrab_covers_absent_draft_present_publish",
            );
        }
        if paths
            .iter()
            .any(|path| finish_path_needs_owner_binary_rust_slice_validation(path))
        {
            mode = FinishValidationMode::LargerBinaryFocused;
            push_finish_validation_command(
                &mut commands,
                "cargo fmt --manifest-path adl/Cargo.toml --all --check",
            );
            push_finish_validation_command(
                &mut commands,
                "cargo test --manifest-path adl/Cargo.toml --bin adl",
            );
        }
        if paths
            .iter()
            .any(|path| finish_path_needs_public_prompt_packet_focused_validation(path))
        {
            mode = FinishValidationMode::LargerBinaryFocused;
            push_finish_validation_command(
                &mut commands,
                "cargo fmt --manifest-path adl/Cargo.toml --all --check",
            );
            push_finish_validation_command(
                &mut commands,
                "cargo test --manifest-path adl/Cargo.toml --bin adl-csdlc public_prompt_packet",
            );
        }
        if paths
            .iter()
            .any(|path| finish_path_needs_coverage_tooling_focused_validation(path))
        {
            mode = FinishValidationMode::LargerBinaryFocused;
            commands.push("bash adl/tools/test_check_coverage_impact.sh".to_string());
        }
        if paths
            .iter()
            .any(|path| finish_path_needs_ci_policy_focused_validation(path))
        {
            mode = FinishValidationMode::LargerBinaryFocused;
            commands.push("bash adl/tools/test_ci_path_policy.sh".to_string());
        }
        if paths
            .iter()
            .any(|path| finish_path_needs_small_binary_delegation_validation(path))
        {
            commands.push("bash adl/tools/test_pr_small_binary_delegation.sh".to_string());
        }
        if paths
            .iter()
            .any(|path| finish_path_needs_owner_lane_contract_validation(path))
        {
            mode = FinishValidationMode::LargerBinaryFocused;
            commands.push("bash adl/tools/test_owner_validation_lane.sh".to_string());
        }
        if paths
            .iter()
            .any(|path| finish_path_needs_csdlc_owner_lane_validation(path))
        {
            mode = FinishValidationMode::LargerBinaryFocused;
            commands.push("bash adl/tools/run_owner_validation_lane.sh csdlc".to_string());
        }
        if paths
            .iter()
            .any(|path| finish_path_needs_runtime_owner_lane_validation(path))
        {
            mode = FinishValidationMode::LargerBinaryFocused;
            if paths
                .iter()
                .any(|path| finish_path_needs_provider_communication_focused_validation(path))
            {
                push_finish_validation_command(
                    &mut commands,
                    "cargo test --manifest-path adl/Cargo.toml --lib provider_communication",
                );
            }
            if paths
                .iter()
                .any(|path| finish_path_needs_resilience_focused_validation(path))
            {
                push_finish_validation_command(
                    &mut commands,
                    "cargo test --manifest-path adl/Cargo.toml --lib resilience",
                );
            }
            commands
                .push("bash adl/tools/run_owner_validation_lane.sh runtime --build".to_string());
        }
        if paths
            .iter()
            .any(|path| finish_path_needs_review_owner_lane_validation(path))
        {
            mode = FinishValidationMode::LargerBinaryFocused;
            commands.push("bash adl/tools/run_owner_validation_lane.sh review --build".to_string());
        }
        if paths
            .iter()
            .any(|path| finish_path_needs_all_owner_lane_validation(path))
        {
            mode = FinishValidationMode::LargerBinaryFocused;
            commands.push("bash adl/tools/run_owner_validation_lane.sh all --build".to_string());
        }
        return Ok(FinishValidationPlan { mode, commands });
    }
    bail!("finish: internal error selecting validation lane")
}

fn push_finish_validation_command(commands: &mut Vec<String>, command: &str) {
    if !commands.iter().any(|existing| existing == command) {
        commands.push(command.to_string());
    }
}

pub(super) fn select_finish_validation_plan_for_finish(
    requested_paths_csv: &str,
    changed_paths: &[String],
) -> Result<FinishValidationPlan> {
    let requested_paths = requested_paths_csv
        .split(',')
        .map(str::trim)
        .filter(|path| !path.is_empty())
        .collect::<Vec<_>>();
    if requested_paths.is_empty() {
        bail!("finish: --paths resolved to empty");
    }
    if changed_paths.is_empty() {
        bail!("finish: no changed tracked paths available for validation profile selection");
    }
    select_finish_validation_plan(&changed_paths.join(","))
}

fn finish_path_is_docs_only(path: &str) -> bool {
    let trimmed = path.trim().trim_matches('/');
    if trimmed.is_empty() {
        return false;
    }
    if trimmed == "docs" || trimmed.starts_with("docs/") {
        return true;
    }
    if trimmed.starts_with("adl/tools/skills/docs/") {
        return finish_path_has_docs_artifact_extension(trimmed);
    }
    if trimmed.starts_with("adl/tools/skills/")
        && (trimmed.ends_with("/SKILL.md") || trimmed.contains("/references/"))
    {
        return finish_path_has_docs_artifact_extension(trimmed);
    }
    !trimmed.contains('/')
        && Path::new(trimmed)
            .extension()
            .and_then(|ext| ext.to_str())
            .is_some_and(|ext| ext.eq_ignore_ascii_case("md"))
}

fn finish_path_has_docs_artifact_extension(path: &str) -> bool {
    Path::new(path)
        .extension()
        .and_then(|ext| ext.to_str())
        .is_some_and(|ext| {
            matches!(
                ext.to_ascii_lowercase().as_str(),
                "md" | "yaml" | "yml" | "json"
            )
        })
}

fn finish_path_is_small_binary_focused(path: &str) -> bool {
    let trimmed = path.trim().trim_matches('/');
    matches!(
        trimmed,
        "adl/tools/pr.sh" | "adl/tools/test_pr_small_binary_delegation.sh"
    ) || finish_path_needs_pr_finish_rust_focused_validation(trimmed)
}

fn finish_path_is_larger_binary_focused(path: &str) -> bool {
    let trimmed = path.trim().trim_matches('/');
    matches!(
        trimmed,
        ".github/workflows/ci.yaml"
            | "docs/default_workflow.md"
            | "docs/milestones/v0.90/milestone_compression/FINISH_VALIDATION_PROFILES_v0.90.md"
            | "docs/tooling/merge_readiness_gate_policy_v0.91.4.md"
            | "docs/milestones/v0.91.4/features/MERGE_READINESS_AND_PR_GATE_HARDENING.md"
            | "docs/milestones/v0.91.4/DEMO_MATRIX_v0.91.4.md"
            | "docs/milestones/v0.91.4/FEATURE_PROOF_COVERAGE_v0.91.4.md"
            | "adl/src/cli/pr_cmd.rs"
            | "adl/src/cli/mod.rs"
            | "adl/src/cli/github_token.rs"
            | "adl/src/lib.rs"
            | "adl/src/provider_communication.rs"
            | "adl/src/resilience.rs"
            | "adl/src/cli/tests/pr_cmd_inline/basics.rs"
            | "adl/src/cli/tests/pr_cmd_inline/support.rs"
            | "adl/src/csdlc_prompt_editor.rs"
            | "adl/src/cli/run_artifacts_types.rs"
            | "adl/schemas/structured_implementation_prompt.contract.yaml"
            | "adl/schemas/structured_output_record.contract.yaml"
            | "adl/schemas/structured_task_prompt.contract.yaml"
            | "adl/templates/cards/input_card_template.md"
            | "adl/templates/cards/output_card_template.md"
            | "adl/tools/attach_post_merge_closeout.sh"
            | "adl/tools/card_paths.sh"
            | "adl/tools/check_no_tracked_adl_issue_record_residue.sh"
            | "adl/tools/lint_prompt_spec.sh"
            | "adl/tools/validate_structured_prompt.sh"
            | "adl/tools/check_coverage_impact.sh"
            | "adl/tools/test_check_coverage_impact.sh"
            | "adl/tools/ci_path_policy.sh"
            | "adl/tools/test_ci_path_policy.sh"
            | "adl/tools/run_owner_validation_lane.sh"
            | "adl/tools/test_owner_validation_lane.sh"
            | "adl/tools/test_cli_wrapper_migration_contract.sh"
            | "adl/tools/test_pr_run_ambiguity_policy.sh"
            | "adl/tools/test_pr_small_binary_delegation.sh"
            | "adl/tools/test_control_plane_observability.sh"
            | "adl/tools/test_adl_runtime_compatibility.sh"
            | "adl/tools/test_adl_review_compatibility.sh"
            | "docs/milestones/v0.91.5/VALIDATION_LANE_SPLIT_3610.md"
            | "docs/milestones/v0.91.5/LOCAL_VS_CI_VALIDATION_POLICY_3607.md"
            | "adl/src/cli/tooling_cmd/public_prompt_packet.rs"
            | "adl/src/cli/tooling_cmd/github_release.rs"
            | "adl/src/cli/tooling_cmd/tests/public_prompt_packet.rs"
    ) || trimmed.starts_with("adl/src/cli/pr_cmd/")
        || trimmed.starts_with("adl/src/cli/pr_cmd_cards/")
        || trimmed.starts_with("adl/src/csdlc_prompt_editor/")
        || trimmed.starts_with("adl/src/cli/run_artifacts_types/")
        || trimmed.starts_with("docs/milestones/v0.91.4/review/merge_readiness/")
        || trimmed.starts_with("adl/src/cli/tests/pr_cmd_inline/finish/")
}

fn finish_path_needs_pr_finish_rust_focused_validation(path: &str) -> bool {
    let trimmed = path.trim().trim_matches('/');
    trimmed == "adl/src/cli/pr_cmd/finish_support.rs"
        || trimmed.starts_with("adl/src/cli/tests/pr_cmd_inline/finish/")
}

fn finish_path_needs_pr_cmd_lifecycle_focused_validation(path: &str) -> bool {
    let trimmed = path.trim().trim_matches('/');
    trimmed == "adl/src/cli/pr_cmd.rs"
        || trimmed == "adl/src/cli/tests/pr_cmd_inline/basics.rs"
        || trimmed == "adl/src/cli/tests/pr_cmd_inline/support.rs"
        || trimmed.starts_with("adl/src/cli/pr_cmd_cards/")
        || (trimmed.starts_with("adl/src/cli/pr_cmd/")
            && trimmed != "adl/src/cli/pr_cmd/finish_support.rs")
}

fn finish_path_needs_github_token_focused_validation(path: &str) -> bool {
    let trimmed = path.trim().trim_matches('/');
    matches!(
        trimmed,
        "adl/src/cli/mod.rs"
            | "adl/src/cli/github_token.rs"
            | "adl/src/cli/tooling_cmd/github_release.rs"
    )
}

fn finish_path_needs_owner_binary_rust_slice_validation(path: &str) -> bool {
    let trimmed = path.trim().trim_matches('/');
    trimmed == "adl/src/lib.rs"
        || trimmed == "adl/src/csdlc_prompt_editor.rs"
        || trimmed.starts_with("adl/src/csdlc_prompt_editor/")
        || trimmed == "adl/src/cli/run_artifacts_types.rs"
        || trimmed.starts_with("adl/src/cli/run_artifacts_types/")
}

fn finish_path_needs_coverage_tooling_focused_validation(path: &str) -> bool {
    let trimmed = path.trim().trim_matches('/');
    matches!(
        trimmed,
        ".github/workflows/ci.yaml"
            | "adl/tools/check_coverage_impact.sh"
            | "adl/tools/test_check_coverage_impact.sh"
    )
}

fn finish_path_needs_public_prompt_packet_focused_validation(path: &str) -> bool {
    let trimmed = path.trim().trim_matches('/');
    matches!(
        trimmed,
        "adl/src/cli/tooling_cmd/public_prompt_packet.rs"
            | "adl/src/cli/tooling_cmd/tests/public_prompt_packet.rs"
    )
}

fn finish_path_needs_ci_policy_focused_validation(path: &str) -> bool {
    let trimmed = path.trim().trim_matches('/');
    matches!(
        trimmed,
        ".github/workflows/ci.yaml"
            | "adl/tools/ci_path_policy.sh"
            | "adl/tools/test_ci_path_policy.sh"
    )
}

fn finish_path_needs_small_binary_delegation_validation(path: &str) -> bool {
    let trimmed = path.trim().trim_matches('/');
    matches!(trimmed, "adl/tools/test_pr_small_binary_delegation.sh")
}

fn finish_path_needs_owner_lane_contract_validation(path: &str) -> bool {
    let trimmed = path.trim().trim_matches('/');
    matches!(
        trimmed,
        "adl/tools/run_owner_validation_lane.sh"
            | "adl/tools/test_owner_validation_lane.sh"
            | "docs/milestones/v0.91.5/VALIDATION_LANE_SPLIT_3610.md"
            | "docs/milestones/v0.91.5/LOCAL_VS_CI_VALIDATION_POLICY_3607.md"
    )
}

fn finish_path_needs_csdlc_owner_lane_validation(path: &str) -> bool {
    let trimmed = path.trim().trim_matches('/');
    matches!(
        trimmed,
        "adl/tools/run_owner_validation_lane.sh"
            | "adl/tools/test_owner_validation_lane.sh"
            | "adl/tools/test_cli_wrapper_migration_contract.sh"
            | "adl/tools/test_pr_run_ambiguity_policy.sh"
            | "adl/tools/test_control_plane_observability.sh"
            | "docs/milestones/v0.91.5/VALIDATION_LANE_SPLIT_3610.md"
            | "docs/milestones/v0.91.5/LOCAL_VS_CI_VALIDATION_POLICY_3607.md"
    )
}

fn finish_path_needs_runtime_owner_lane_validation(path: &str) -> bool {
    let trimmed = path.trim().trim_matches('/');
    matches!(
        trimmed,
        "adl/tools/test_adl_runtime_compatibility.sh"
            | "adl/src/provider_communication.rs"
            | "adl/src/resilience.rs"
    )
}

fn finish_path_needs_provider_communication_focused_validation(path: &str) -> bool {
    let trimmed = path.trim().trim_matches('/');
    trimmed == "adl/src/provider_communication.rs"
}

fn finish_path_needs_resilience_focused_validation(path: &str) -> bool {
    let trimmed = path.trim().trim_matches('/');
    trimmed == "adl/src/resilience.rs"
}

fn finish_path_needs_review_owner_lane_validation(path: &str) -> bool {
    let trimmed = path.trim().trim_matches('/');
    matches!(trimmed, "adl/tools/test_adl_review_compatibility.sh")
}

fn finish_path_needs_all_owner_lane_validation(path: &str) -> bool {
    let trimmed = path.trim().trim_matches('/');
    matches!(trimmed, "adl/tools/run_owner_validation_lane.sh")
}

pub(super) fn run_finish_validation_rust(
    repo_root: &Path,
    plan: &FinishValidationPlan,
) -> Result<()> {
    finish_validation_guard(repo_root)?;
    if plan.mode == FinishValidationMode::DocsOnly {
        run_finish_validation_status("git", &["-C", path_str(repo_root)?, "diff", "--check"])?;
        return Ok(());
    }

    if matches!(
        plan.mode,
        FinishValidationMode::SmallBinaryFocused | FinishValidationMode::LargerBinaryFocused
    ) {
        run_finish_validation_status("git", &["-C", path_str(repo_root)?, "diff", "--check"])?;
        let manifest = repo_root.join("adl/Cargo.toml");
        for command in &plan.commands {
            match command.as_str() {
                "bash adl/tools/check_no_tracked_adl_issue_record_residue.sh" => {}
                "git diff --check" => {}
                "cargo fmt --manifest-path adl/Cargo.toml --all --check" => {
                    run_finish_validation_status(
                        "cargo",
                        &[
                            "fmt",
                            "--manifest-path",
                            path_str(&manifest)?,
                            "--all",
                            "--check",
                        ],
                    )?;
                }
                "cargo test --manifest-path adl/Cargo.toml --bin adl cli::pr_cmd" => {
                    run_finish_validation_status(
                        "cargo",
                        &[
                            "test",
                            "--manifest-path",
                            path_str(&manifest)?,
                            "--bin",
                            "adl",
                            "cli::pr_cmd",
                        ],
                    )?;
                }
                "cargo test --manifest-path adl/Cargo.toml --bin adl" => {
                    run_finish_validation_status(
                        "cargo",
                        &[
                            "test",
                            "--manifest-path",
                            path_str(&manifest)?,
                            "--bin",
                            "adl",
                        ],
                    )?;
                }
                "cargo test --manifest-path adl/Cargo.toml --bin adl github_token" => {
                    run_finish_validation_status(
                        "cargo",
                        &[
                            "test",
                            "--manifest-path",
                            path_str(&manifest)?,
                            "--bin",
                            "adl",
                            "github_token",
                        ],
                    )?;
                }
                "cargo test --manifest-path adl/Cargo.toml --bin adl github_client" => {
                    run_finish_validation_status(
                        "cargo",
                        &[
                            "test",
                            "--manifest-path",
                            path_str(&manifest)?,
                            "--bin",
                            "adl",
                            "github_client",
                        ],
                    )?;
                }
                "cargo test --manifest-path adl/Cargo.toml --bin adl github_release_octocrab_covers_absent_draft_present_publish" => {
                    run_finish_validation_status(
                        "cargo",
                        &[
                            "test",
                            "--manifest-path",
                            path_str(&manifest)?,
                            "--bin",
                            "adl",
                            "github_release_octocrab_covers_absent_draft_present_publish",
                        ],
                    )?;
                }
                "cargo test --manifest-path adl/Cargo.toml --bin adl-pr-finish cli::pr_cmd::tests::finish::arg_render::finish_validation" => {
                    run_finish_validation_status(
                        "cargo",
                        &[
                            "test",
                            "--manifest-path",
                            path_str(&manifest)?,
                            "--bin",
                            "adl-pr-finish",
                            "cli::pr_cmd::tests::finish::arg_render::finish_validation",
                        ],
                    )?;
                }
                "cargo test --manifest-path adl/Cargo.toml --bin adl-pr-finish cli::pr_cmd::tests::finish::arg_render::finish_helper_paths_run_focused_local_ci_gated_validation" => {
                    run_finish_validation_status(
                        "cargo",
                        &[
                            "test",
                            "--manifest-path",
                            path_str(&manifest)?,
                            "--bin",
                            "adl-pr-finish",
                            "cli::pr_cmd::tests::finish::arg_render::finish_helper_paths_run_focused_local_ci_gated_validation",
                        ],
                    )?;
                }
                "cargo test --manifest-path adl/Cargo.toml --bin adl-csdlc public_prompt_packet" => {
                    run_finish_validation_status(
                        "cargo",
                        &[
                            "test",
                            "--manifest-path",
                            path_str(&manifest)?,
                            "--bin",
                            "adl-csdlc",
                            "public_prompt_packet",
                        ],
                    )?;
                }
                "bash adl/tools/test_check_coverage_impact.sh" => {
                    let script = repo_root.join("adl/tools/test_check_coverage_impact.sh");
                    run_finish_validation_status("bash", &[path_str(&script)?])?;
                }
                "bash adl/tools/test_ci_path_policy.sh" => {
                    let script = repo_root.join("adl/tools/test_ci_path_policy.sh");
                    run_finish_validation_status("bash", &[path_str(&script)?])?;
                }
                "bash adl/tools/test_pr_small_binary_delegation.sh" => {
                    let script = repo_root.join("adl/tools/test_pr_small_binary_delegation.sh");
                    run_finish_validation_status("bash", &[path_str(&script)?])?;
                }
                "bash adl/tools/test_owner_validation_lane.sh" => {
                    let script = repo_root.join("adl/tools/test_owner_validation_lane.sh");
                    run_finish_validation_status("bash", &[path_str(&script)?])?;
                }
                "bash adl/tools/run_owner_validation_lane.sh csdlc" => {
                    let script = repo_root.join("adl/tools/run_owner_validation_lane.sh");
                    run_finish_validation_status("bash", &[path_str(&script)?, "csdlc"])?;
                }
                "bash adl/tools/run_owner_validation_lane.sh runtime --build" => {
                    let script = repo_root.join("adl/tools/run_owner_validation_lane.sh");
                    run_finish_validation_status("bash", &[path_str(&script)?, "runtime", "--build"])?;
                }
                "cargo test --manifest-path adl/Cargo.toml --lib provider_communication" => {
                    run_finish_validation_status(
                        "cargo",
                        &[
                            "test",
                            "--manifest-path",
                            path_str(&manifest)?,
                            "--lib",
                            "provider_communication",
                        ],
                    )?;
                }
                "cargo test --manifest-path adl/Cargo.toml --lib resilience" => {
                    run_finish_validation_status(
                        "cargo",
                        &[
                            "test",
                            "--manifest-path",
                            path_str(&manifest)?,
                            "--lib",
                            "resilience",
                        ],
                    )?;
                }
                "bash adl/tools/run_owner_validation_lane.sh review --build" => {
                    let script = repo_root.join("adl/tools/run_owner_validation_lane.sh");
                    run_finish_validation_status("bash", &[path_str(&script)?, "review", "--build"])?;
                }
                "bash adl/tools/run_owner_validation_lane.sh all --build" => {
                    let script = repo_root.join("adl/tools/run_owner_validation_lane.sh");
                    run_finish_validation_status("bash", &[path_str(&script)?, "all", "--build"])?;
                }
                other => bail!("finish: unsupported focused validation command '{other}'"),
            }
        }
        return Ok(());
    }
    bail!("finish: unsupported validation mode")
}

const FINISH_VALIDATION_SANITIZED_ENVS: &[&str] = &[
    "ADL_GITHUB_CLIENT",
    "ADL_GITHUB_DISABLE_GH_FALLBACK",
    "ADL_GITHUB_OCTOCRAB_BASE_URI",
    "GITHUB_TOKEN",
    "GH_TOKEN",
    "ADL_GITHUB_TOKEN_FILE",
    "ADL_GITHUB_TOKEN_KEYCHAIN_SERVICE",
    "ADL_GITHUB_TOKEN_KEYCHAIN_ACCOUNT",
];

fn run_finish_validation_status(program: &str, args: &[&str]) -> Result<()> {
    let class = classify_finish_subprocess(program, args);
    let excerpt = format_subprocess_excerpt(program, args);
    let heartbeat = ProgressHeartbeat::start(
        "finish",
        "validation_subprocess",
        &[
            ("program", program),
            ("subprocess_class", class),
            ("argv_excerpt", &excerpt),
        ],
    );
    let mut command = Command::new(program);
    command.args(args);
    for key in FINISH_VALIDATION_SANITIZED_ENVS {
        command.env_remove(key);
    }
    let status = match command.status() {
        Ok(status) => status,
        Err(err) => {
            heartbeat.failed(&[
                ("reason_code", "validation_subprocess_spawn_failed"),
                ("next_action_hint", "check_subprocess_path_and_permissions"),
            ]);
            return Err(err).with_context(|| format!("failed to spawn '{program}'"));
        }
    };
    if !status.success() {
        let exit_code = status
            .code()
            .map(|value| value.to_string())
            .unwrap_or_else(|| "signal".to_string());
        heartbeat.failed(&[
            ("reason_code", "validation_subprocess_failed"),
            ("next_action_hint", "inspect_subprocess_output"),
            ("exit_code", &exit_code),
        ]);
        bail!("{program} failed with status {:?}", status.code());
    }
    let exit_code = status
        .code()
        .map(|value| value.to_string())
        .unwrap_or_else(|| "0".to_string());
    heartbeat.completed(&[("exit_code", &exit_code)]);
    Ok(())
}

fn classify_finish_subprocess(program: &str, args: &[&str]) -> &'static str {
    match program {
        "cargo" => "rust_validation",
        "git" => "git_hygiene",
        "bash" => {
            if args
                .iter()
                .any(|arg| arg.contains("run_owner_validation_lane"))
            {
                "owner_validation_lane"
            } else if args
                .iter()
                .any(|arg| arg.contains("coverage") || arg.contains("llvm-cov"))
            {
                "coverage_validation"
            } else {
                "shell_validation"
            }
        }
        _ => "validation_subprocess",
    }
}

fn format_subprocess_excerpt(program: &str, args: &[&str]) -> String {
    let mut parts = Vec::with_capacity(args.len() + 1);
    parts.push(program.to_string());
    parts.extend(args.iter().take(4).map(|value| value.to_string()));
    let mut joined = parts.join(" ");
    if args.len() > 4 {
        joined.push_str(" ...");
    }
    joined
}

fn resolve_finish_pr_base(repo_root: &Path, branch: &str) -> Result<String> {
    for key in ["ADL_PR_BASE", "ADL_PR_BASE_BRANCH", "GH_PR_BASE"] {
        if let Some(value) = std::env::var_os(key) {
            let value = value.to_string_lossy().trim().to_string();
            if !value.is_empty() {
                if value == branch {
                    bail!("finish: {key} must not match the current branch '{branch}'");
                }
                return Ok(value);
            }
        }
    }

    let config_key = format!("branch.{branch}.gh-merge-base");
    if let Some(value) = run_capture_allow_failure(
        "git",
        &["-C", path_str(repo_root)?, "config", "--get", &config_key],
    )? {
        let value = value.trim().to_string();
        if !value.is_empty() {
            if value == branch {
                bail!("finish: {config_key} must not match the current branch '{branch}'");
            }
            return Ok(value);
        }
    }

    Ok("main".to_string())
}

fn ensure_existing_pr_base_matches(repo: &str, pr_url: &str, expected_base: &str) -> Result<()> {
    let actual_base = pr_view_base_ref_finish_existing(repo, pr_url)?;
    let actual_base = actual_base.trim();
    if actual_base != expected_base {
        bail!(
            "finish: existing PR base '{}' does not match expected base '{}'; update the PR base or rerun with the intended ADL_PR_BASE",
            actual_base,
            expected_base
        );
    }
    Ok(())
}

pub(super) fn render_default_finish_validation(plan: &FinishValidationPlan) -> String {
    let mut lines = plan
        .commands
        .iter()
        .map(|command| format!("- {command}"))
        .collect::<Vec<_>>();
    match plan.mode {
        FinishValidationMode::DocsOnly => {
            lines.push(
                "- CI integration proof: deferred to GitHub checks for merge-context validation."
                    .to_string(),
            );
        }
        FinishValidationMode::SmallBinaryFocused => {
            lines.push(
                "- Local preflight profile: small-binary focused build/test only; this is not a broader local Rust sweep."
                    .to_string(),
            );
            lines.push(
                "- CI integration proof: deferred to GitHub checks for merge-context validation."
                    .to_string(),
            );
        }
        FinishValidationMode::LargerBinaryFocused => {
            lines.push(
                "- Local preflight profile: larger owner-binary focused build/test only; this is not a repo-wide local Rust sweep."
                    .to_string(),
            );
            lines.push(
                "- CI integration proof: deferred to GitHub checks for merge-context validation."
                    .to_string(),
            );
        }
    }
    lines.join("\n")
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
    let root_spp = issue_ref.task_bundle_plan_path(primary_root);
    let root_srp = issue_ref.task_bundle_review_policy_path(primary_root);
    let root_input = issue_ref.task_bundle_input_path(primary_root);
    let root_output = issue_ref.task_bundle_output_path(primary_root);
    for path in [
        source_path,
        &root_stp,
        &root_spp,
        &root_srp,
        &root_input,
        &root_output,
    ] {
        let Some(repo_relative) = path
            .strip_prefix(primary_root)
            .ok()
            .map(|value| value.to_string_lossy().into_owned())
        else {
            continue;
        };
        if run_capture_allow_failure(
            "git",
            &[
                "-C",
                path_str(repo_root)?,
                "ls-files",
                "--error-unmatch",
                "--",
                &repo_relative,
            ],
        )?
        .is_some()
        {
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
    let mut stageable = Vec::new();
    for path in paths {
        if path.starts_with(".adl/") {
            let tracked = run_status_allow_failure(
                "git",
                &[
                    "-C",
                    path_str(repo_root)?,
                    "ls-files",
                    "--error-unmatch",
                    "--",
                    path,
                ],
            )?;
            if !tracked {
                continue;
            }
        }
        stageable.push(path);
    }
    if stageable.is_empty() {
        bail!("finish: --paths resolved only to local-only .adl issue surfaces; no tracked repo paths remain to stage");
    }
    let mut args = vec!["-C", path_str(repo_root)?, "add", "--"];
    args.extend(stageable);
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

#[cfg(test)]
mod tests {
    use super::run_finish_validation_status;
    use crate::test_support::env_lock as shared_env_lock;
    use std::fs;
    use std::os::unix::fs::PermissionsExt;
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::sync::MutexGuard;

    static TEMP_SEQ: AtomicU64 = AtomicU64::new(0);

    fn env_lock() -> MutexGuard<'static, ()> {
        shared_env_lock()
    }

    fn temp_dir(prefix: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "adl-finish-validation-{prefix}-{}-{}",
            std::process::id(),
            TEMP_SEQ.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir_all(&dir).expect("create temp dir");
        dir
    }

    #[test]
    fn finish_validation_emits_subprocess_heartbeat_and_classification() {
        let _guard = env_lock();
        let temp = temp_dir("heartbeat");
        let script = temp.join("slow-validation.sh");
        let log = temp.join("observability.log");
        fs::write(&script, "#!/bin/sh\nsleep 0.08\nexit 0\n").expect("write script");
        let mut perms = fs::metadata(&script).expect("metadata").permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&script, perms).expect("chmod");

        unsafe {
            std::env::set_var("ADL_OBSERVABILITY_STDERR", "0");
            std::env::set_var("ADL_OBSERVABILITY_LOG", &log);
            std::env::set_var("ADL_OBSERVABILITY_HEARTBEAT_MS", "25");
        }

        run_finish_validation_status("bash", &[script.to_str().expect("script path")])
            .expect("validation command");

        let contents = fs::read_to_string(&log).expect("read log");
        assert!(contents.contains("command=finish"));
        assert!(contents.contains("stage=validation_subprocess"));
        assert!(contents.contains("program=bash"));
        assert!(contents.contains("subprocess_class=shell_validation"));
        assert!(contents.contains("result=heartbeat"));
        assert!(contents.contains("result=completed"));

        unsafe {
            std::env::remove_var("ADL_OBSERVABILITY_STDERR");
            std::env::remove_var("ADL_OBSERVABILITY_LOG");
            std::env::remove_var("ADL_OBSERVABILITY_HEARTBEAT_MS");
        }
    }

    #[test]
    fn finish_validation_emits_failed_terminal_event_on_spawn_error() {
        let _guard = env_lock();
        let temp = temp_dir("spawn-error");
        let log = temp.join("observability.log");

        unsafe {
            std::env::set_var("ADL_OBSERVABILITY_STDERR", "0");
            std::env::set_var("ADL_OBSERVABILITY_LOG", &log);
            std::env::set_var("ADL_OBSERVABILITY_HEARTBEAT_MS", "25");
        }

        let err =
            run_finish_validation_status("definitely-not-a-real-finish-subprocess", &["--version"])
                .expect_err("spawn should fail");
        assert!(err.to_string().contains("failed to spawn"));

        let contents = fs::read_to_string(&log).expect("read log");
        assert!(contents.contains("result=started"));
        assert!(contents.contains("result=failed"));
        assert!(contents.contains("reason_code=validation_subprocess_spawn_failed"));
        assert!(contents.contains("next_action_hint=check_subprocess_path_and_permissions"));

        unsafe {
            std::env::remove_var("ADL_OBSERVABILITY_STDERR");
            std::env::remove_var("ADL_OBSERVABILITY_LOG");
            std::env::remove_var("ADL_OBSERVABILITY_HEARTBEAT_MS");
        }
    }
}
