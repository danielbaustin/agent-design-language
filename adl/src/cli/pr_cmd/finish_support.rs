use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use serde_yaml::{Mapping, Value};
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
use ::adl::control_plane::{resolve_cards_root, sanitize_slug, IssueRef};

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
    let finish_validation_plan = select_finish_validation_plan_for_finish(
        parsed.issue,
        &parsed.paths,
        &validation_changed_paths,
    )?;
    let finish_validation_profile = if parsed.no_checks {
        None
    } else {
        Some(load_finish_validation_profile(
            &repo_root,
            &validation_changed_paths,
        )?)
    };
    if !parsed.no_checks {
        run_finish_validation_rust(&repo_root, &finish_validation_plan)?;
        record_docs_only_validation_evidence_for_finish(&output_path, &finish_validation_plan)?;
    }
    record_sor_emitted_facts_for_finish(
        &output_path,
        &review_policy_path,
        &changed_paths,
        &finish_validation_plan,
        SorFactEmissionContext {
            validation_status: if parsed.no_checks { "NOT_RUN" } else { "PASS" },
            pr_url: None,
            integration_state: "worktree_only",
            closing_linkage_repaired: false,
        },
    )?;
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
        Some(non_closing_lifecycle_line(parsed.issue))
    } else {
        Some(format!("Closes #{}", parsed.issue))
    };
    let default_validation = if parsed.no_checks {
        None
    } else {
        Some(render_default_finish_validation(
            &finish_validation_plan,
            finish_validation_profile.as_ref(),
        ))
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

    let commit_msg = if !parsed.no_close {
        let close = close_line
            .as_ref()
            .expect("closing finish should have a close line");
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
    record_sor_emitted_facts_for_finish(
        &output_path,
        &review_policy_path,
        &changed_paths,
        &finish_validation_plan,
        SorFactEmissionContext {
            validation_status: if parsed.no_checks { "NOT_RUN" } else { "PASS" },
            pr_url: Some(pr_url.as_str()),
            integration_state: "pr_open",
            closing_linkage_repaired: _closing_linkage_repaired,
        },
    )?;
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct SorFacts {
    schema_version: &'static str,
    changed_paths: Vec<String>,
    validation: SorFactValidation,
    review: SorFactReview,
    finish: SorFactFinish,
    integration: SorFactIntegration,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct SorFactValidation {
    status: String,
    commands: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct SorFactReview {
    findings_status: String,
    recommended_outcome: String,
    findings: Vec<String>,
    fixes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct SorFactFinish {
    pr_url: Option<String>,
    blocking_notes: Vec<String>,
    fix_notes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct SorFactIntegration {
    state: String,
    main_repo_paths: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SorReviewEvidence {
    findings_status: String,
    recommended_outcome: String,
    findings: Vec<String>,
    fixes: Vec<String>,
}

#[derive(Debug, Clone, Copy)]
pub(super) struct SorFactEmissionContext<'a> {
    pub validation_status: &'a str,
    pub pr_url: Option<&'a str>,
    pub integration_state: &'a str,
    pub closing_linkage_repaired: bool,
}

fn record_sor_emitted_facts_for_finish(
    output_path: &Path,
    review_policy_path: &Path,
    changed_paths: &[String],
    plan: &FinishValidationPlan,
    context: SorFactEmissionContext<'_>,
) -> Result<()> {
    let original = fs::read_to_string(output_path).with_context(|| {
        format!(
            "finish: failed to read output card {}",
            output_path.display()
        )
    })?;
    let review = read_sor_review_evidence(review_policy_path)?;
    let normalized = normalize_sor_emitted_facts_text(
        &original,
        &build_sor_facts(
            changed_paths,
            plan,
            context.validation_status,
            &review,
            context.pr_url,
            context.integration_state,
            context.closing_linkage_repaired,
        ),
    )?;
    if normalized != original {
        fs::write(output_path, normalized).with_context(|| {
            format!(
                "finish: failed to write emitted SOR facts into output card {}",
                output_path.display()
            )
        })?;
    }
    Ok(())
}

fn build_sor_facts(
    changed_paths: &[String],
    plan: &FinishValidationPlan,
    validation_status: &str,
    review: &SorReviewEvidence,
    pr_url: Option<&str>,
    integration_state: &str,
    closing_linkage_repaired: bool,
) -> SorFacts {
    let validation_commands = if validation_status == "NOT_RUN" {
        Vec::new()
    } else {
        plan.commands
            .iter()
            .map(|command| sanitize_validation_profile_command(command))
            .collect()
    };
    let fix_notes = if closing_linkage_repaired {
        vec!["repaired missing PR closing linkage".to_string()]
    } else {
        vec!["none".to_string()]
    };
    SorFacts {
        schema_version: "adl.sor_facts.v1",
        changed_paths: changed_paths.to_vec(),
        validation: SorFactValidation {
            status: validation_status.to_string(),
            commands: validation_commands,
        },
        review: SorFactReview {
            findings_status: review.findings_status.clone(),
            recommended_outcome: review.recommended_outcome.clone(),
            findings: review.findings.clone(),
            fixes: review.fixes.clone(),
        },
        finish: SorFactFinish {
            pr_url: pr_url.map(str::to_string),
            blocking_notes: vec!["none".to_string()],
            fix_notes,
        },
        integration: SorFactIntegration {
            state: integration_state.to_string(),
            main_repo_paths: changed_paths.to_vec(),
        },
    }
}

fn read_sor_review_evidence(review_policy_path: &Path) -> Result<SorReviewEvidence> {
    let text = fs::read_to_string(review_policy_path).with_context(|| {
        format!(
            "finish: failed to read review policy card {}",
            review_policy_path.display()
        )
    })?;
    Ok(parse_sor_review_evidence(&text))
}

fn parse_sor_review_evidence(text: &str) -> SorReviewEvidence {
    let (findings_status, recommended_outcome) = review_results_from_front_matter(text)
        .unwrap_or_else(|| ("not_run".to_string(), "not_run".to_string()));
    let findings = bullet_lines_from_markdown_section(text, "Findings");
    let fixes = bullet_lines_from_markdown_section(text, "Dispositions");
    SorReviewEvidence {
        findings_status,
        recommended_outcome,
        findings: if findings.is_empty() {
            vec!["not_recorded".to_string()]
        } else {
            findings
        },
        fixes: if fixes.is_empty() {
            vec!["not_recorded".to_string()]
        } else {
            fixes
        },
    }
}

fn review_results_from_front_matter(text: &str) -> Option<(String, String)> {
    let front_matter = extract_yaml_front_matter(text)?;
    let yaml: Value = serde_yaml::from_str(front_matter).ok()?;
    let mapping = yaml.as_mapping()?;
    let review_results = mapping.get(Value::String("review_results".to_string()))?;
    let review_mapping = review_results.as_mapping()?;
    let findings_status = yaml_mapping_string(review_mapping, "findings_status")
        .unwrap_or_else(|| "not_run".to_string());
    let recommended_outcome = yaml_mapping_string(review_mapping, "recommended_outcome")
        .unwrap_or_else(|| "not_run".to_string());
    Some((findings_status, recommended_outcome))
}

fn extract_yaml_front_matter(text: &str) -> Option<&str> {
    let mut cursor = 0usize;
    let mut lines = text.split_inclusive('\n');
    let first = lines.next()?;
    if first.trim_end_matches(['\r', '\n']).trim() != "---" {
        return None;
    }
    let content_start = first.len();
    cursor += first.len();
    for line in lines {
        let trimmed = line.trim_end_matches(['\r', '\n']).trim();
        if trimmed == "---" {
            return text.get(content_start..cursor);
        }
        cursor += line.len();
    }
    None
}

fn yaml_mapping_string(mapping: &Mapping, key: &str) -> Option<String> {
    mapping
        .get(Value::String(key.to_string()))
        .and_then(Value::as_str)
        .map(str::to_string)
}

fn bullet_lines_from_markdown_section(text: &str, heading: &str) -> Vec<String> {
    markdown_section_body_local(text, heading)
        .map(|body| {
            body.lines()
                .map(str::trim)
                .filter_map(|line| line.strip_prefix("- "))
                .map(str::trim)
                .filter(|line| !line.is_empty())
                .map(str::to_string)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default()
}

fn normalize_sor_emitted_facts_text(text: &str, facts: &SorFacts) -> Result<String> {
    let Some(summary_body) = markdown_section_body_local(text, "Verification Summary") else {
        return Ok(text.to_string());
    };
    let updated_summary = update_verification_summary_yaml(&summary_body, facts)?;
    Ok(
        replace_markdown_section_body(text, "Verification Summary", &updated_summary)
            .unwrap_or_else(|_| text.to_string()),
    )
}

fn update_verification_summary_yaml(body: &str, facts: &SorFacts) -> Result<String> {
    let facts_value = serde_yaml::to_value(facts)
        .context("finish: failed to serialize emitted SOR facts into YAML")?;
    let Some(yaml) = extract_fenced_yaml_block(body) else {
        return append_standalone_sor_facts_block(body, facts_value);
    };
    let mut value: Value = match serde_yaml::from_str(&yaml) {
        Ok(value) => value,
        Err(_) => return append_standalone_sor_facts_block(body, facts_value),
    };
    let Some(mapping) = value.as_mapping_mut() else {
        return append_standalone_sor_facts_block(body, facts_value);
    };
    mapping.insert(Value::String("sor_facts".to_string()), facts_value);
    render_yaml_fence(&value)
}

fn extract_fenced_yaml_block(body: &str) -> Option<String> {
    let mut lines = body.lines();
    let first = lines.next()?.trim();
    if !first.starts_with("```") {
        return None;
    }
    let mut yaml_lines = Vec::new();
    for line in lines {
        if line.trim() == "```" {
            return Some(yaml_lines.join("\n"));
        }
        yaml_lines.push(line.to_string());
    }
    None
}

fn append_standalone_sor_facts_block(body: &str, facts_value: Value) -> Result<String> {
    let mut sor_facts = Mapping::new();
    sor_facts.insert(Value::String("sor_facts".to_string()), facts_value);
    let fenced = render_yaml_fence(&Value::Mapping(sor_facts))?;
    let mut out = strip_existing_standalone_sor_facts_block(body)
        .trim_end()
        .to_string();
    if !out.is_empty() {
        out.push_str("\n\n");
    }
    out.push_str("Machine-readable SOR facts:\n\n");
    out.push_str(&fenced);
    Ok(out)
}

fn strip_existing_standalone_sor_facts_block(body: &str) -> &str {
    if let Some(idx) = body.find("Machine-readable SOR facts:\n") {
        &body[..idx]
    } else {
        body
    }
}

fn render_yaml_fence(value: &Value) -> Result<String> {
    let mut rendered = serde_yaml::to_string(value)
        .context("finish: failed to render Verification Summary YAML with emitted SOR facts")?;
    if let Some(stripped) = rendered.strip_prefix("---\n") {
        rendered = stripped.to_string();
    }
    Ok(format!("```yaml\n{rendered}```"))
}

#[cfg(test)]
pub(super) fn normalize_sor_emitted_facts_fixture(
    text: &str,
    changed_paths: &[String],
    commands: &[String],
    review_text: &str,
    context: SorFactEmissionContext<'_>,
) -> Result<String> {
    let plan = FinishValidationPlan {
        mode: FinishValidationMode::LargerBinaryFocused,
        commands: commands.to_vec(),
    };
    let review = parse_sor_review_evidence(review_text);
    let facts = build_sor_facts(
        changed_paths,
        &plan,
        context.validation_status,
        &review,
        context.pr_url,
        context.integration_state,
        context.closing_linkage_repaired,
    );
    normalize_sor_emitted_facts_text(text, &facts)
}

pub(super) fn restage_finish_output_truth_paths(
    repo_root: &Path,
    primary_root: &Path,
    issue_ref: &IssueRef,
    paths: &[PathBuf],
) -> Result<()> {
    let worktree_bundle_dir = issue_ref.task_bundle_dir_path(repo_root);
    let primary_bundle_dir = issue_ref.task_bundle_dir_path(primary_root);
    let worktree_cards_root = resolve_cards_root(repo_root, None);
    let primary_cards_root = resolve_cards_root(primary_root, None);
    let mut relpaths = BTreeSet::new();

    for path in paths {
        let resolved = if path.is_absolute() {
            path.clone()
        } else {
            repo_root.join(path)
        };
        let tracked_cards_relpath =
            tracked_local_cards_relpath(repo_root, &resolved, &worktree_cards_root, repo_root)?.or(
                tracked_local_cards_relpath(
                    primary_root,
                    &resolved,
                    &primary_cards_root,
                    primary_root,
                )?,
            );
        if let Some(relpath) = tracked_cards_relpath {
            bail!(
                "finish: compatibility cards path '{}' is tracked under .adl/cards; keep local cards ignored-only before finish publication",
                relpath
            );
        }
        let is_local_ignored_cards_path = path_is_ignored_local_cards_path(
            repo_root,
            &resolved,
            &worktree_cards_root,
            repo_root,
        )? || path_is_ignored_local_cards_path(
            primary_root,
            &resolved,
            &primary_cards_root,
            primary_root,
        )?;
        if resolved.starts_with(&worktree_bundle_dir)
            || resolved.starts_with(&primary_bundle_dir)
            || is_local_ignored_cards_path
        {
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

fn tracked_local_cards_relpath(
    query_root: &Path,
    resolved: &Path,
    cards_root: &Path,
    rel_root: &Path,
) -> Result<Option<String>> {
    if !resolved.starts_with(cards_root) {
        return Ok(None);
    }
    let Ok(relpath) = resolved.strip_prefix(rel_root) else {
        return Ok(None);
    };
    let relpath = relpath.to_string_lossy().into_owned();
    if path_is_tracked_in_git(query_root, &relpath)? {
        return Ok(Some(relpath));
    }
    Ok(None)
}

fn path_is_ignored_local_cards_path(
    query_root: &Path,
    resolved: &Path,
    cards_root: &Path,
    rel_root: &Path,
) -> Result<bool> {
    if !resolved.starts_with(cards_root) {
        return Ok(false);
    }
    let Ok(relpath) = resolved.strip_prefix(rel_root) else {
        return Ok(false);
    };
    let relpath = relpath.to_string_lossy().into_owned();
    if path_is_tracked_in_git(query_root, &relpath)? {
        return Ok(false);
    }
    path_is_git_ignored(query_root, &relpath)
}

fn path_is_tracked_in_git(repo_root: &Path, relpath: &str) -> Result<bool> {
    Ok(run_capture_allow_failure(
        "git",
        &[
            "-C",
            path_str(repo_root)?,
            "ls-files",
            "--error-unmatch",
            "--",
            relpath,
        ],
    )?
    .is_some())
}

fn path_is_git_ignored(repo_root: &Path, relpath: &str) -> Result<bool> {
    run_status_allow_failure(
        "git",
        &[
            "-C",
            path_str(repo_root)?,
            "check-ignore",
            "-q",
            "--no-index",
            "--",
            relpath,
        ],
    )
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

#[derive(Debug, Deserialize)]
pub(super) struct FinishValidationProfile {
    pub selected_profile: String,
    pub status: String,
    pub pr_publication_sufficient: bool,
    #[serde(default)]
    pub run: Vec<FinishValidationProfileRunItem>,
    #[serde(default)]
    pub not_run: Vec<FinishValidationProfileSurfaceItem>,
    #[serde(default)]
    pub deferred: Vec<FinishValidationProfileSurfaceItem>,
    pub escalation: FinishValidationProfileEscalation,
}

#[derive(Debug, Deserialize)]
pub(super) struct FinishValidationProfileRunItem {
    pub lane_id: String,
    pub command: String,
    pub reason: String,
    #[allow(dead_code)]
    #[serde(default)]
    pub matched_paths: Vec<String>,
    #[serde(default)]
    pub vpp_record: Option<FinishValidationVppRecord>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub(super) struct FinishValidationVppRecord {
    pub contract_version: String,
    pub artifacts: Vec<String>,
    pub expected_runtime_class: String,
    pub parallel_group: String,
    pub cache_equivalence_group: String,
    pub failure_semantics: String,
}

#[derive(Debug, Deserialize)]
pub(super) struct FinishValidationProfileSurfaceItem {
    pub surface: String,
    pub reason: String,
}

#[derive(Debug, Deserialize)]
pub(super) struct FinishValidationProfileEscalation {
    pub required: bool,
    #[serde(default)]
    pub reasons: Vec<FinishValidationProfileEscalationReason>,
}

#[derive(Debug, Deserialize)]
pub(super) struct FinishValidationProfileEscalationReason {
    pub lane_id: String,
    pub status: String,
    pub reason: String,
    #[serde(default)]
    pub matched_paths: Vec<String>,
    #[serde(default)]
    pub manifest_rule: Option<String>,
    #[serde(default)]
    pub remediation_hint: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FinishPathOwnerBinary {
    Adl,
    Csdlc,
    OwnerValidationLane,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FinishPathProofRole {
    OwnerBinaryRustSlice,
    OwnerLaneContract,
    CsdlcOwnerLane,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct FinishPathOwnershipRule {
    owner_binary: FinishPathOwnerBinary,
    validation_lane: FinishValidationMode,
    proof_role: FinishPathProofRole,
    publication_sufficient: bool,
    exact_paths: &'static [&'static str],
    prefix_paths: &'static [&'static str],
}

const FINISH_PATH_OWNERSHIP_RULES: &[FinishPathOwnershipRule] = &[
    FinishPathOwnershipRule {
        owner_binary: FinishPathOwnerBinary::Adl,
        validation_lane: FinishValidationMode::LargerBinaryFocused,
        proof_role: FinishPathProofRole::OwnerBinaryRustSlice,
        publication_sufficient: true,
        exact_paths: &[
            "adl/src/control_plane.rs",
            "adl/src/lib.rs",
            "adl/src/session_ledger.rs",
            "adl/src/cli/session_cmd.rs",
            "adl/src/cli/tests.rs",
            "adl/src/csdlc_prompt_editor.rs",
            "adl/src/cli/run_artifacts_types.rs",
        ],
        prefix_paths: &[
            "adl/src/csdlc_prompt_editor/",
            "adl/src/cli/run_artifacts_types/",
        ],
    },
    FinishPathOwnershipRule {
        owner_binary: FinishPathOwnerBinary::OwnerValidationLane,
        validation_lane: FinishValidationMode::LargerBinaryFocused,
        proof_role: FinishPathProofRole::OwnerLaneContract,
        publication_sufficient: true,
        exact_paths: &[
            "adl/tools/run_owner_validation_lane.sh",
            "adl/tools/test_owner_validation_lane.sh",
            "adl/tools/test_control_plane_observability.sh",
            "docs/milestones/v0.91.5/VALIDATION_LANE_SPLIT_3610.md",
            "docs/milestones/v0.91.5/LOCAL_VS_CI_VALIDATION_POLICY_3607.md",
        ],
        prefix_paths: &[],
    },
    FinishPathOwnershipRule {
        owner_binary: FinishPathOwnerBinary::Csdlc,
        validation_lane: FinishValidationMode::LargerBinaryFocused,
        proof_role: FinishPathProofRole::CsdlcOwnerLane,
        publication_sufficient: true,
        exact_paths: &[
            "adl/tools/run_owner_validation_lane.sh",
            "adl/tools/test_owner_validation_lane.sh",
            "adl/tools/test_cli_wrapper_migration_contract.sh",
            "adl/tools/test_pr_run_ambiguity_policy.sh",
            "adl/tools/test_control_plane_observability.sh",
            "docs/milestones/v0.91.5/VALIDATION_LANE_SPLIT_3610.md",
            "docs/milestones/v0.91.5/LOCAL_VS_CI_VALIDATION_POLICY_3607.md",
        ],
        prefix_paths: &[],
    },
];

fn finish_path_matches_ownership_rule(path: &str, rule: &FinishPathOwnershipRule) -> bool {
    let trimmed = path.trim().trim_matches('/');
    !trimmed.is_empty()
        && (rule.exact_paths.contains(&trimmed)
            || rule
                .prefix_paths
                .iter()
                .any(|prefix| trimmed.starts_with(prefix)))
}

fn finish_path_matches_registry_proof_role(path: &str, role: FinishPathProofRole) -> bool {
    FINISH_PATH_OWNERSHIP_RULES.iter().any(|rule| {
        finish_path_matches_ownership_rule(path, rule)
            && rule.proof_role == role
            && rule.publication_sufficient
    })
}

fn finish_path_matches_registry_lane(path: &str, lane: FinishValidationMode) -> bool {
    FINISH_PATH_OWNERSHIP_RULES.iter().any(|rule| {
        finish_path_matches_ownership_rule(path, rule)
            && rule.validation_lane == lane
            && rule.publication_sufficient
    })
}

fn finish_path_matches_registry_owner(path: &str, owner: FinishPathOwnerBinary) -> bool {
    FINISH_PATH_OWNERSHIP_RULES.iter().any(|rule| {
        finish_path_matches_ownership_rule(path, rule)
            && rule.owner_binary == owner
            && rule.publication_sufficient
    })
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
    if paths.iter().all(|path| finish_path_is_docs_only(path))
        && !paths
            .iter()
            .any(|path| finish_path_needs_prompt_template_focused_validation(path))
    {
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
            .any(|path| finish_path_needs_process_status_focused_validation(path))
        {
            mode = FinishValidationMode::LargerBinaryFocused;
            push_finish_validation_command(
                &mut commands,
                "cargo fmt --manifest-path adl/Cargo.toml --all --check",
            );
            push_finish_validation_command(
                &mut commands,
                "cargo test --manifest-path adl/Cargo.toml --test cli_smoke process_status -- --nocapture",
            );
        }
        if paths
            .iter()
            .any(|path| finish_path_needs_scheduler_focused_validation(path))
        {
            mode = FinishValidationMode::LargerBinaryFocused;
            push_finish_validation_command(
                &mut commands,
                "cargo fmt --manifest-path adl/Cargo.toml --all --check",
            );
            push_finish_validation_command(
                &mut commands,
                "cargo test --manifest-path adl/Cargo.toml --lib scheduler_economics",
            );
        }
        if paths
            .iter()
            .any(|path| finish_path_needs_github_release_focused_validation(path))
        {
            mode = FinishValidationMode::LargerBinaryFocused;
            push_finish_validation_command(
                &mut commands,
                "cargo fmt --manifest-path adl/Cargo.toml --all --check",
            );
            push_finish_validation_command(
                &mut commands,
                "cargo test --manifest-path adl/Cargo.toml --bin adl github_release_",
            );
        }
        if paths
            .iter()
            .any(|path| finish_path_needs_ci_log_archive_focused_validation(path))
        {
            mode = FinishValidationMode::LargerBinaryFocused;
            push_finish_validation_command(
                &mut commands,
                "cargo fmt --manifest-path adl/Cargo.toml --all --check",
            );
            push_finish_validation_command(
                &mut commands,
                "cargo test --manifest-path adl/Cargo.toml ci_log_archive -- --nocapture",
            );
            push_finish_validation_command(
                &mut commands,
                "cargo test --manifest-path adl/Cargo.toml tooling_cmd_dispatch_and_help_paths_cover_public_entrypoint -- --nocapture",
            );
        }
        if paths
            .iter()
            .any(|path| finish_path_needs_issue_resource_telemetry_focused_validation(path))
        {
            mode = FinishValidationMode::LargerBinaryFocused;
            push_finish_validation_command(
                &mut commands,
                "cargo fmt --manifest-path adl/Cargo.toml --all --check",
            );
            push_finish_validation_command(
                &mut commands,
                "cargo test --manifest-path adl/Cargo.toml issue_resource_telemetry -- --nocapture",
            );
            push_finish_validation_command(
                &mut commands,
                "cargo test --manifest-path adl/Cargo.toml tooling_cmd_dispatch_and_help_paths_cover_public_entrypoint -- --nocapture",
            );
        }
        if paths
            .iter()
            .any(|path| finish_path_needs_prompt_template_focused_validation(path))
        {
            mode = FinishValidationMode::LargerBinaryFocused;
            push_finish_validation_command(
                &mut commands,
                "cargo fmt --manifest-path adl/Cargo.toml --all --check",
            );
            push_finish_validation_command(
                &mut commands,
                "cargo test --manifest-path adl/Cargo.toml --bin adl prompt_template_ -- --nocapture",
            );
            push_finish_validation_command(
                &mut commands,
                "cargo test --manifest-path adl/Cargo.toml --bin adl structured_prompt_ -- --nocapture",
            );
        }
        if paths
            .iter()
            .any(|path| finish_path_needs_long_lived_agent_tokio_validation(path))
        {
            mode = FinishValidationMode::LargerBinaryFocused;
            push_finish_validation_command(
                &mut commands,
                "cargo fmt --manifest-path adl/Cargo.toml --all --check",
            );
            push_finish_validation_command(
                &mut commands,
                "cargo test --manifest-path adl/Cargo.toml long_lived_agent",
            );
        }
        if paths
            .iter()
            .any(|path| finish_path_needs_tokio_bootstrap_helper_validation(path))
        {
            mode = FinishValidationMode::LargerBinaryFocused;
            push_finish_validation_command(
                &mut commands,
                "cargo fmt --manifest-path adl/Cargo.toml --all --check",
            );
            push_finish_validation_command(
                &mut commands,
                "cargo test --manifest-path adl/Cargo.toml pr_cmd::github",
            );
            push_finish_validation_command(
                &mut commands,
                "cargo test --manifest-path adl/Cargo.toml --bin adl github_release_",
            );
            push_finish_validation_command(
                &mut commands,
                "cargo test --manifest-path adl/Cargo.toml --bin adl octocrab_transport_",
            );
        }
        if paths
            .iter()
            .any(|path| finish_path_needs_remote_exec_tokio_validation(path))
        {
            mode = FinishValidationMode::LargerBinaryFocused;
            push_finish_validation_command(
                &mut commands,
                "cargo fmt --manifest-path adl/Cargo.toml --all --check",
            );
            push_finish_validation_command(
                &mut commands,
                "cargo test --manifest-path adl/Cargo.toml build_remote_execute_request_preserves_conversation_as_audit_metadata",
            );
            push_finish_validation_command(
                &mut commands,
                "cargo test --manifest-path adl/Cargo.toml execute_step_with_retry_does_not_retry_remote_schema_violation",
            );
            push_finish_validation_command(
                &mut commands,
                "cargo test --manifest-path adl/Cargo.toml security_envelope_rejects_tampered_signed_conversation_metadata",
            );
            push_finish_validation_command(
                &mut commands,
                "cargo test --manifest-path adl/Cargo.toml remote_exec::",
            );
        }
        if paths
            .iter()
            .any(|path| finish_path_needs_cav_tokio_validation(path))
        {
            mode = FinishValidationMode::LargerBinaryFocused;
            push_finish_validation_command(
                &mut commands,
                "cargo fmt --manifest-path adl/Cargo.toml --all --check",
            );
            push_finish_validation_command(
                &mut commands,
                "cargo test --manifest-path adl/Cargo.toml continuous_verification_contract_covers_cadence_lifecycle_and_artifacts",
            );
            push_finish_validation_command(
                &mut commands,
                "cargo test --manifest-path adl/Cargo.toml self_attack_contract_is_policy_bounded_and_reviewable",
            );
            push_finish_validation_command(
                &mut commands,
                "cargo test --manifest-path adl/Cargo.toml identity_continuous_verification_writes_contract_json",
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
            commands.push("bash adl/tools/test_run_pr_fast_test_lane.sh".to_string());
        }
        if paths
            .iter()
            .any(|path| finish_path_needs_ci_runtime_contract_validation(path))
        {
            mode = FinishValidationMode::LargerBinaryFocused;
            commands.push("bash adl/tools/test_ci_runtime_contracts.sh".to_string());
        }
        if paths
            .iter()
            .any(|path| finish_path_needs_validation_selector_focused_validation(path))
        {
            mode = FinishValidationMode::LargerBinaryFocused;
            commands.push("bash adl/tools/test_select_validation_lanes.sh".to_string());
        }
        if paths
            .iter()
            .any(|path| finish_path_needs_slow_proof_family_focused_validation(path))
        {
            mode = FinishValidationMode::LargerBinaryFocused;
            push_finish_validation_command(
                &mut commands,
                "cargo fmt --manifest-path adl/Cargo.toml --all --check",
            );
            commands.push("bash adl/tools/test_slow_proof_lane_contract.sh".to_string());
        }
        if paths
            .iter()
            .any(|path| finish_path_needs_validation_manager_focused_validation(path))
        {
            commands.push("bash adl/tools/test_validation_manager.sh".to_string());
        }
        if paths
            .iter()
            .any(|path| finish_path_needs_validation_inventory_focused_validation(path))
        {
            commands.push("bash adl/tools/test_validation_inventory.sh".to_string());
        }
        if paths
            .iter()
            .any(|path| finish_path_needs_sprint_conductor_helper_validation(path))
        {
            mode = FinishValidationMode::LargerBinaryFocused;
            commands.push("bash adl/tools/test_sprint_conductor_helpers.sh".to_string());
        }
        if paths
            .iter()
            .any(|path| finish_path_needs_unity_observatory_baseline_validation(0, path))
        {
            commands.push("bash adl/tools/test_v0916_unity_observatory_baseline.sh".to_string());
        }
        if paths
            .iter()
            .any(|path| finish_path_needs_small_binary_delegation_validation(path))
        {
            commands.push("bash adl/tools/test_pr_small_binary_delegation.sh".to_string());
        }
        if paths
            .iter()
            .any(|path| finish_path_needs_locked_cargo_fallback_validation(path))
        {
            commands.push(
                "bash adl/tools/test_pr_run_locked_cargo_fallback_refuses_cleanly.sh".to_string(),
            );
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
            .any(|path| finish_path_needs_repo_quality_staleness_validation(path))
        {
            mode = FinishValidationMode::LargerBinaryFocused;
            commands.push("bash adl/tools/test_check_repo_quality_staleness.sh".to_string());
        }
        if paths
            .iter()
            .any(|path| finish_path_needs_deepseek_suitability_validation(path))
        {
            mode = FinishValidationMode::LargerBinaryFocused;
            commands.push("bash adl/tools/test_v0916_deepseek_suitability.sh".to_string());
        }
        if paths
            .iter()
            .any(|path| finish_path_needs_private_endpoint_fixture_sanitation_validation(path))
        {
            mode = FinishValidationMode::LargerBinaryFocused;
            commands
                .push("bash adl/tools/test_demo_codex_ollama_operational_skills.sh".to_string());
            commands.push("bash adl/tools/test_demo_codex_ollama_semantic_fallback.sh".to_string());
            commands.push("bash adl/tools/test_demo_v089_gemma4_issue_clerk.sh".to_string());
            push_finish_validation_command(
                &mut commands,
                "cargo test --manifest-path adl/Cargo.toml --lib provider_substrate_uses_http_transport_for_ollama_with_endpoint",
            );
            commands.push(
                "python3 adl/tools/validate_v0915_remote_gemma_watcher_probe.py docs/milestones/v0.91.5/review/remote_gemma_watcher"
                    .to_string(),
            );
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
                .any(|path| finish_path_needs_agent_comms_focused_validation(path))
            {
                push_finish_validation_command(
                    &mut commands,
                    "cargo test --manifest-path adl/Cargo.toml agent_comms --lib -- --nocapture",
                );
            }
            if paths
                .iter()
                .any(|path| finish_path_needs_provider_adapter_focused_validation(path))
            {
                push_finish_validation_command(
                    &mut commands,
                    "cargo test --manifest-path adl/Cargo.toml --lib provider_adapter",
                );
            }
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

pub(super) fn load_finish_validation_profile(
    repo_root: &Path,
    changed_paths: &[String],
) -> Result<FinishValidationProfile> {
    load_finish_validation_profile_with_retention(repo_root, changed_paths, false)
}

pub(super) fn load_finish_validation_profile_for_execution(
    repo_root: &Path,
    changed_paths: &[String],
) -> Result<FinishValidationProfile> {
    load_finish_validation_profile_with_retention(repo_root, changed_paths, true)
}

fn load_finish_validation_profile_with_retention(
    repo_root: &Path,
    changed_paths: &[String],
    retain_changed_file_for_execution: bool,
) -> Result<FinishValidationProfile> {
    let changed_file_body = changed_paths
        .iter()
        .map(|path| format!("M\t{path}"))
        .collect::<Vec<_>>()
        .join("\n");
    let changed_file_path =
        write_temp_text("finish-validation-profile", "txt", &changed_file_body)?;
    let changed_file_path_str = path_str(&changed_file_path)?.to_string();
    let manager = repo_root.join("adl/tools/validation_manager.py");
    let output = run_capture(
        "python3",
        &[
            path_str(&manager)?,
            "--changed-files",
            path_str(&changed_file_path)?,
            "--json",
        ],
    );
    let output = match output {
        Ok(output) => output,
        Err(err) => {
            let _ = fs::remove_file(&changed_file_path);
            return Err(err).context("finish: failed to load validation manager profile");
        }
    };
    let mut profile = match serde_json::from_str::<FinishValidationProfile>(&output) {
        Ok(profile) => profile,
        Err(err) => {
            let _ = fs::remove_file(&changed_file_path);
            return Err(err).context("finish: validation manager returned invalid profile JSON");
        }
    };
    if retain_changed_file_for_execution {
        if let Err(err) =
            validate_manager_backed_retained_changed_file(&profile, &changed_file_path_str)
        {
            let _ = fs::remove_file(&changed_file_path);
            return Err(err);
        }
    }
    if !retain_changed_file_for_execution {
        for item in &mut profile.run {
            item.command = sanitize_validation_profile_command(&item.command);
        }
    }
    let profile_needs_changed_file = profile
        .run
        .iter()
        .any(|item| item.command.contains(&changed_file_path_str));
    if !retain_changed_file_for_execution || !profile_needs_changed_file {
        let _ = fs::remove_file(&changed_file_path);
    }
    Ok(profile)
}

fn validate_manager_backed_retained_changed_file(
    profile: &FinishValidationProfile,
    expected_changed_file: &str,
) -> Result<()> {
    for item in &profile.run {
        if item
            .command
            .starts_with("bash adl/tools/run_pr_fast_test_lane.sh --changed-files ")
        {
            let changed_file = manager_backed_pr_fast_changed_files_arg(&item.command)?;
            let changed_file_cmp =
                fs::canonicalize(&changed_file).unwrap_or_else(|_| PathBuf::from(&changed_file));
            let expected_changed_file_cmp = fs::canonicalize(expected_changed_file)
                .unwrap_or_else(|_| PathBuf::from(expected_changed_file));
            if changed_file_cmp != expected_changed_file_cmp {
                bail!(
                    "finish: validation manager returned unsupported changed-files manifest '{}'; expected ADL-created retained manifest '{}'",
                    changed_file,
                    expected_changed_file
                );
            }
        }
    }
    Ok(())
}

pub(super) fn select_finish_validation_plan_for_finish(
    issue_number: u32,
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
    if finish_paths_are_version_metadata_update(changed_paths) {
        return Ok(build_version_metadata_validation_plan());
    }
    if finish_issue_needs_tokio_manifest_runtime_validation(issue_number, changed_paths) {
        return Ok(build_tokio_manifest_runtime_validation_plan());
    }
    if finish_issue_needs_issue_small_binary_validation(issue_number, changed_paths) {
        return Ok(build_issue_small_binary_validation_plan());
    }
    if finish_issue_needs_session_ledger_issue_validation(issue_number, changed_paths) {
        return Ok(build_session_ledger_issue_validation_plan());
    }
    if finish_issue_needs_closing_linkage_small_binary_validation(issue_number, changed_paths) {
        return Ok(build_closing_linkage_small_binary_validation_plan());
    }
    if finish_issue_needs_wuji_ddns_validation(issue_number, changed_paths) {
        return Ok(build_wuji_ddns_validation_plan());
    }
    if finish_issue_needs_wuji_ddns_installer_validation(issue_number, changed_paths) {
        return Ok(build_wuji_ddns_installer_validation_plan());
    }
    if finish_issue_needs_locked_cargo_fallback_validation(issue_number, changed_paths) {
        return Ok(build_locked_cargo_fallback_validation_plan());
    }
    if finish_issue_needs_native_gws_runtime_validation(issue_number, changed_paths) {
        return Ok(build_native_gws_runtime_validation_plan());
    }
    if finish_paths_need_github_projection_watch_validation(changed_paths) {
        return Ok(build_github_projection_watch_validation_plan());
    }
    if finish_issue_needs_unity_observatory_scaffold_validation(issue_number, changed_paths) {
        return Ok(build_unity_observatory_scaffold_validation_plan(
            changed_paths,
        ));
    }
    if finish_issue_needs_unity_observatory_contract_validation(issue_number, changed_paths) {
        return Ok(build_unity_observatory_contract_validation_plan(
            changed_paths,
        ));
    }
    if finish_issue_needs_html_observatory_validation(issue_number, changed_paths) {
        return Ok(build_html_observatory_validation_plan(changed_paths));
    }
    let finish_profile =
        load_finish_validation_profile_for_execution(&repo_root()?, changed_paths)?;
    if let Some(plan) = profile_backed_finish_validation_plan(&finish_profile) {
        return Ok(plan);
    }
    match select_finish_validation_plan(&changed_paths.join(",")) {
        Ok(plan) => Ok(plan),
        Err(legacy_err) => {
            ensure_finish_validation_profile_is_runnable(&finish_profile, changed_paths)?;
            Err(legacy_err)
        }
    }
}

fn profile_backed_finish_validation_plan(
    profile: &FinishValidationProfile,
) -> Option<FinishValidationPlan> {
    if profile.status != "ready_to_run"
        || !profile.pr_publication_sufficient
        || profile.escalation.required
    {
        return None;
    }
    let mut commands = vec![
        "bash adl/tools/check_no_tracked_adl_issue_record_residue.sh".to_string(),
        "git diff --check".to_string(),
    ];
    for item in &profile.run {
        push_finish_validation_command(&mut commands, &item.command);
    }
    let mode = profile_backed_finish_validation_mode(profile);
    Some(FinishValidationPlan { mode, commands })
}

fn profile_backed_finish_validation_mode(
    profile: &FinishValidationProfile,
) -> FinishValidationMode {
    if profile.run.len() == 1 && profile.run[0].lane_id == "docs_diff_check" {
        return FinishValidationMode::DocsOnly;
    }
    if profile.run.iter().any(|item| {
        item.lane_id == "rust_pr_fast"
            || item.lane_id.contains("owner_lane")
            || item.command.contains("run_owner_validation_lane.sh")
            || item.command.contains("run_pr_fast_test_lane.sh")
            || item
                .command
                .contains("cargo test --manifest-path adl/Cargo.toml")
    }) {
        return FinishValidationMode::LargerBinaryFocused;
    }
    FinishValidationMode::SmallBinaryFocused
}

fn ensure_finish_validation_profile_is_runnable(
    profile: &FinishValidationProfile,
    changed_paths: &[String],
) -> Result<()> {
    if profile.status == "ready_to_run"
        && profile.pr_publication_sufficient
        && !profile.escalation.required
    {
        return Ok(());
    }
    let changed = if changed_paths.is_empty() {
        "<none>".to_string()
    } else {
        changed_paths.join(", ")
    };
    let mut details = vec![format!(
        "finish: validation manager reported a non-runnable profile for changed paths: {}",
        changed
    )];
    details.push(format!(
        "profile={} status={} pr_publication_sufficient={}",
        profile.selected_profile, profile.status, profile.pr_publication_sufficient
    ));
    if profile.escalation.required {
        for reason in &profile.escalation.reasons {
            let mut line = format!(
                "lane={} status={} reason={}",
                reason.lane_id, reason.status, reason.reason
            );
            if !reason.matched_paths.is_empty() {
                line.push_str(&format!(
                    " matched_paths={}",
                    reason.matched_paths.join(",")
                ));
            }
            if let Some(rule) = &reason.manifest_rule {
                line.push_str(&format!(" manifest_rule={rule}"));
            }
            if let Some(hint) = &reason.remediation_hint {
                line.push_str(&format!(" remediation_hint={hint}"));
            }
            details.push(line);
        }
    } else if profile.run.is_empty() {
        details.push("validation manager selected no runnable lanes".to_string());
    }
    bail!("{}", details.join("; "))
}

fn finish_paths_are_version_metadata_update(changed_paths: &[String]) -> bool {
    let mut has_manifest = false;
    let mut has_lockfile = false;
    let mut has_current_docs = false;
    for path in changed_paths {
        match path.trim().trim_matches('/') {
            "README.md" => has_current_docs = true,
            "adl/Cargo.toml" => has_manifest = true,
            "adl/Cargo.lock" => has_lockfile = true,
            _ => return false,
        }
    }
    has_manifest && has_lockfile && has_current_docs
}

fn build_version_metadata_validation_plan() -> FinishValidationPlan {
    FinishValidationPlan {
        mode: FinishValidationMode::SmallBinaryFocused,
        commands: vec![
            "bash adl/tools/check_no_tracked_adl_issue_record_residue.sh".to_string(),
            "git diff --check".to_string(),
            "cargo metadata --manifest-path adl/Cargo.toml --no-deps --format-version 1"
                .to_string(),
            "cargo metadata --manifest-path adl/Cargo.toml --locked --no-deps --format-version 1"
                .to_string(),
        ],
    }
}

fn finish_paths_need_github_projection_watch_validation(changed_paths: &[String]) -> bool {
    let mut has_github_projection_surface = false;
    !changed_paths.is_empty()
        && changed_paths.iter().all(|path| {
            let trimmed = path.trim().trim_matches('/');
            if matches!(
                trimmed,
                "adl/src/cli/pr_cmd/github.rs"
                    | "adl/src/cli/pr_cmd/github/transport.rs"
                    | "adl/src/cli/pr_cmd/github/tests/watch.rs"
                    | "adl/src/cli/pr_cmd/github/tests/validation.rs"
            ) || trimmed.starts_with("adl/src/cli/pr_cmd/github/")
            {
                has_github_projection_surface = true;
                return true;
            }
            matches!(
                trimmed,
                "adl/src/cli/pr_cmd.rs" | "adl/src/cli/tests/pr_cmd_inline/basics.rs"
            ) || finish_path_is_docs_only(trimmed)
        })
        && has_github_projection_surface
}

fn build_github_projection_watch_validation_plan() -> FinishValidationPlan {
    FinishValidationPlan {
        mode: FinishValidationMode::LargerBinaryFocused,
        commands: vec![
            "bash adl/tools/check_no_tracked_adl_issue_record_residue.sh".to_string(),
            "git diff --check".to_string(),
            "cargo fmt --manifest-path adl/Cargo.toml --all --check".to_string(),
            "cargo test --manifest-path adl/Cargo.toml --bin adl cli::pr_cmd::github::tests -- --nocapture".to_string(),
            "cargo test --manifest-path adl/Cargo.toml --bin adl validation_disposition_blocks_pending_and_terminal_failures -- --nocapture".to_string(),
        ],
    }
}

fn finish_issue_needs_unity_observatory_scaffold_validation(
    issue_number: u32,
    changed_paths: &[String],
) -> bool {
    issue_number == 4031
        && !changed_paths.is_empty()
        && changed_paths.iter().all(|path| {
            let trimmed = path.trim().trim_matches('/');
            finish_path_is_docs_only(trimmed)
                || finish_path_is_small_binary_focused(trimmed)
                || finish_path_is_larger_binary_focused(trimmed)
                || parsed_issue_is_4031_unity_observatory_scaffold_path(issue_number, trimmed)
        })
}

fn build_unity_observatory_scaffold_validation_plan(
    changed_paths: &[String],
) -> FinishValidationPlan {
    let mut commands = vec![
        "bash adl/tools/check_no_tracked_adl_issue_record_residue.sh".to_string(),
        "git diff --check".to_string(),
        "bash adl/tools/test_v0916_unity_observatory_baseline.sh".to_string(),
    ];
    if changed_paths
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
    FinishValidationPlan {
        mode: FinishValidationMode::SmallBinaryFocused,
        commands,
    }
}

fn finish_issue_needs_unity_observatory_contract_validation(
    issue_number: u32,
    changed_paths: &[String],
) -> bool {
    matches!(issue_number, 4032..=4034 | 4416)
        && !changed_paths.is_empty()
        && changed_paths.iter().all(|path| {
            let trimmed = path.trim().trim_matches('/');
            finish_path_is_docs_only(trimmed)
                || finish_path_is_small_binary_focused(trimmed)
                || finish_path_is_larger_binary_focused(trimmed)
                || parsed_issue_is_unity_observatory_contract_path(issue_number, trimmed)
        })
}

fn build_unity_observatory_contract_validation_plan(
    changed_paths: &[String],
) -> FinishValidationPlan {
    let mut commands = vec![
        "bash adl/tools/check_no_tracked_adl_issue_record_residue.sh".to_string(),
        "git diff --check".to_string(),
        "bash adl/tools/test_v0916_unity_observatory_contract.sh".to_string(),
        "cargo fmt --manifest-path adl/Cargo.toml --all --check".to_string(),
        "cargo test --manifest-path adl/Cargo.toml --test cli_smoke csm_observatory_cli_writes_unity_contract_bundle_and_matches_seeded_resource -- --nocapture".to_string(),
        "cargo test --manifest-path adl/Cargo.toml --bin adl-pr-finish cli::pr_cmd::tests::finish::arg_render::finish_validation_profile_classifies_unity_observatory_contract_slice_as_small_binary_focused -- --nocapture".to_string(),
    ];
    if changed_paths
        .iter()
        .any(|path| finish_path_needs_pr_finish_rust_focused_validation(path))
    {
        push_finish_validation_command(
            &mut commands,
            "cargo test --manifest-path adl/Cargo.toml --bin adl-pr-finish cli::pr_cmd::tests::finish::arg_render::finish_validation",
        );
        push_finish_validation_command(
            &mut commands,
            "cargo test --manifest-path adl/Cargo.toml --bin adl-pr-finish cli::pr_cmd::tests::finish::arg_render::finish_helper_paths_run_focused_local_ci_gated_validation",
        );
    }
    FinishValidationPlan {
        mode: FinishValidationMode::SmallBinaryFocused,
        commands,
    }
}

fn finish_issue_needs_html_observatory_validation(
    issue_number: u32,
    changed_paths: &[String],
) -> bool {
    issue_number == 4341
        && !changed_paths.is_empty()
        && changed_paths.iter().all(|path| {
            let trimmed = path.trim().trim_matches('/');
            finish_path_is_docs_only(trimmed)
                || finish_path_is_small_binary_focused(trimmed)
                || finish_path_is_larger_binary_focused(trimmed)
                || parsed_issue_is_html_observatory_path(issue_number, trimmed)
        })
}

fn build_html_observatory_validation_plan(changed_paths: &[String]) -> FinishValidationPlan {
    let mut commands = vec![
        "bash adl/tools/check_no_tracked_adl_issue_record_residue.sh".to_string(),
        "git diff --check".to_string(),
        "bash adl/tools/test_demo_v0904_csm_observatory_governed_prototype.sh".to_string(),
    ];
    if changed_paths.iter().any(|path| {
        matches!(
            path.trim().trim_matches('/'),
            "adl/config/validation_lane_selector.v0.91.6.json"
                | "adl/tools/test_select_validation_lanes.sh"
        )
    }) {
        push_finish_validation_command(
            &mut commands,
            "bash adl/tools/test_select_validation_lanes.sh",
        );
    }
    if changed_paths
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
        push_finish_validation_command(
            &mut commands,
            "cargo test --manifest-path adl/Cargo.toml --bin adl-pr-finish cli::pr_cmd::tests::finish::arg_render::finish_validation_profile_classifies_html_mobile_observatory_slice_as_small_binary_focused -- --nocapture",
        );
    }
    FinishValidationPlan {
        mode: FinishValidationMode::SmallBinaryFocused,
        commands,
    }
}

fn finish_path_is_docs_only(path: &str) -> bool {
    let trimmed = path.trim().trim_matches('/');
    if trimmed.is_empty() {
        return false;
    }
    if trimmed == "adl/tools/README.md" {
        return true;
    }
    if trimmed == "docs" || trimmed.starts_with("docs/") {
        return true;
    }
    if trimmed.starts_with("demos/") && trimmed.ends_with(".md") {
        return true;
    }
    if trimmed.starts_with("adl/tools/skills/docs/") {
        return finish_path_has_docs_artifact_extension(trimmed);
    }
    if trimmed.starts_with("adl/tools/skills/") && trimmed.contains("/docs/") {
        return finish_path_has_docs_artifact_extension(trimmed);
    }
    if trimmed.starts_with("adl/tools/skills/")
        && (trimmed.ends_with("/SKILL.md") || trimmed.contains("/references/"))
    {
        return finish_path_has_docs_artifact_extension(trimmed);
    }
    if trimmed.starts_with("adl/tools/skills/") && trimmed.ends_with("/agents/openai.yaml") {
        return true;
    }
    if trimmed.starts_with("adl/tools/skills/")
        && (trimmed.ends_with("/adl-skill.yaml") || trimmed.contains("/scripts/"))
    {
        return finish_path_has_docs_artifact_extension(trimmed) || trimmed.ends_with(".py");
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
    if finish_path_matches_registry_lane(trimmed, FinishValidationMode::SmallBinaryFocused) {
        return true;
    }
    matches!(
        trimmed,
        "adl/tools/pr.sh"
            | "adl/tools/observability.sh"
            | "adl/tools/polis_status_for_ssm_windows.ps1"
            | "adl/tools/test_pr_delegate_cargo_fallback_liveness.sh"
            | "adl/tools/test_pr_delegate_prefers_primary_checkout_binary.sh"
            | "adl/tools/test_pr_small_binary_delegation.sh"
            | "adl/tools/test_pr_run_locked_cargo_fallback_refuses_cleanly.sh"
            | "adl/tools/validate_polis_status_for_ssm_windows.py"
            | "adl/tools/validation_manager.py"
            | "adl/tools/validation_manager.sh"
            | "adl/tools/test_validation_manager.sh"
            | "adl/tools/validation_inventory.py"
            | "adl/tools/validation_inventory.sh"
            | "adl/tools/test_validation_inventory.sh"
            | "adl/tools/test_install_adl_operational_skills.sh"
            | "adl/tools/test_sprint_conductor_helpers.sh"
            | "adl/tools/test_v0916_unity_observatory_baseline.sh"
            | "adl/tools/test_v0916_unity_observatory_contract.sh"
    ) || finish_path_needs_pr_finish_rust_focused_validation(trimmed)
}

fn finish_path_is_larger_binary_focused(path: &str) -> bool {
    let trimmed = path.trim().trim_matches('/');
    if finish_path_matches_registry_lane(trimmed, FinishValidationMode::LargerBinaryFocused) {
        return true;
    }
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
            | "adl/src/cli/pr_cmd_args.rs"
            | "adl/src/cli/pr_cmd_prompt.rs"
            | "adl/src/cli/mod.rs"
            | "adl/src/cli/process_cmd.rs"
            | "adl/src/cli/session_cmd.rs"
            | "adl/src/cli/tests.rs"
            | "adl/src/cli/usage.rs"
            | "adl/src/cli/tokio_runtime.rs"
            | "adl/src/cli/github_token.rs"
            | "adl/src/control_plane.rs"
            | "adl/src/lib.rs"
            | "adl/src/session_ledger.rs"
            | "adl/src/scheduler.rs"
            | "adl/src/agent_comms.rs"
            | "adl/src/provider_adapter.rs"
            | "adl/src/provider_communication.rs"
            | "adl/src/resilience.rs"
            | "adl/src/long_lived_agent.rs"
            | "adl/src/long_lived_agent/inspection.rs"
            | "adl/src/long_lived_agent/schema.rs"
            | "adl/src/long_lived_agent/storage.rs"
            | "adl/src/long_lived_agent/tests.rs"
            | "adl/src/runtime_aws_signal.rs"
            | "adl/src/demo/stock_league/model.rs"
            | "adl/src/execute/runner.rs"
            | "adl/src/execute/tests.rs"
            | "adl/src/remote_exec.rs"
            | "adl/src/remote_exec/signing_support.rs"
            | "adl/src/remote_exec/types.rs"
            | "adl/src/bin/run_v0916_integrated_runtime_soak.rs"
            | "adl/src/continuous_verification_self_attack.rs"
            | "adl/src/cli/identity_cmd/tests/adversarial_contracts.rs"
            | "adl/src/cli/tests/pr_cmd_inline/basics.rs"
            | "adl/src/cli/tests/pr_cmd_inline/versioned_bootstrap.rs"
            | "adl/tests/cli_smoke.rs"
            | "adl/tests/cli_smoke/agent.rs"
            | "adl/tests/cli_smoke/process_status.rs"
            | "adl/tests/demo_tests.rs"
            | "adl/src/cli/tests/pr_cmd_inline/repo_helpers/metadata.rs"
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
            | "adl/tools/test_ci_runtime_contracts.sh"
            | "adl/tools/run_authoritative_coverage_lane.sh"
            | "adl/tools/test_run_authoritative_coverage_lane.sh"
            | "adl/tools/run_pr_fast_test_lane.sh"
            | "adl/tools/test_run_pr_fast_test_lane.sh"
            | "adl/config/validation_lane_selector.v0.91.6.json"
            | "adl/tools/select_validation_lanes.py"
            | "adl/tools/select_validation_lanes.sh"
            | "adl/tools/test_select_validation_lanes.sh"
            | "adl/tools/run_owner_validation_lane.sh"
            | "adl/tools/test_owner_validation_lane.sh"
            | "adl/tools/test_control_plane_observability.sh"
            | "adl/tools/test_five_command_regression_suite.sh"
            | "adl/tools/test_cli_wrapper_migration_contract.sh"
            | "adl/tools/test_pr_run_ambiguity_policy.sh"
            | "adl/tools/test_pr_run_issue_mode.sh"
            | "adl/tools/test_pr_small_binary_delegation.sh"
            | "adl/tools/test_adl_runtime_compatibility.sh"
            | "adl/tools/test_adl_review_compatibility.sh"
            | "adl/tools/polis_status_for_ssm_qts.sh"
            | "adl/tools/validate_polis_status_for_ssm_qts.py"
            | "adl/tools/run_slow_proof_family.sh"
            | "adl/tools/test_slow_proof_lane_contract.sh"
            | "adl/config/slow_proof_families.v0.91.6.json"
            | "adl/tools/check_repo_quality_staleness.py"
            | "adl/tools/test_check_repo_quality_staleness.sh"
            | "adl/tools/run_v0916_agent_suitability_panel.py"
            | "adl/tools/run_v0916_deepseek_suitability.py"
            | "adl/tools/validate_v0916_agent_suitability_panel.py"
            | "adl/tools/validate_v0916_deepseek_suitability.py"
            | "adl/tools/test_v0916_deepseek_suitability.sh"
            | "adl/tools/suitability_specs/deepseek_csdlc_panel_4096.json"
            | "docs/milestones/v0.91.5/VALIDATION_LANE_SPLIT_3610.md"
            | "docs/milestones/v0.91.5/LOCAL_VS_CI_VALIDATION_POLICY_3607.md"
            | "adl/src/cli/tooling_cmd.rs"
            | "adl/src/cli/tooling_cmd/common.rs"
            | "adl/src/cli/tooling_cmd/markdown.rs"
            | "adl/src/cli/tooling_cmd/ci_log_archive.rs"
            | "adl/src/cli/tooling_cmd/issue_resource_telemetry.rs"
            | "adl/src/cli/tooling_cmd/prompt_template.rs"
            | "adl/src/cli/tooling_cmd/structured_prompt.rs"
            | "adl/src/cli/tooling_cmd/tests/prompt_template.rs"
            | "adl/src/cli/tooling_cmd/tests/structured_prompt.rs"
            | "adl/src/cli/tooling_cmd/tests/support.rs"
            | "adl/src/cli/tooling_cmd/tests/tooling_dispatch.rs"
            | "adl/src/cli/tooling_cmd/github_release.rs"
            | "adl/src/cli/tooling_cmd/public_prompt_packet.rs"
            | "adl/src/cli/tooling_cmd/tests/public_prompt_packet.rs"
            | "adl/src/bin/adl_prompt_template.rs"
            | "adl/src/bin/adl_validate_structured_prompt.rs"
            | "adl/tools/demo_codex_ollama_operational_skills.sh"
            | "adl/tools/demo_v089_gemma4_issue_clerk.sh"
            | "adl/tools/test_prompt_template_structure_schemas.py"
            | "adl/tools/test_prompt_template_workflow_integration.sh"
            | "adl/tools/test_demo_codex_ollama_operational_skills.sh"
            | "adl/tools/test_demo_codex_ollama_semantic_fallback.sh"
            | "adl/tools/test_demo_v089_gemma4_issue_clerk.sh"
            | "adl/src/provider_substrate.rs"
            | "adl/tools/validate_v0915_remote_gemma_watcher_probe.py"
            | "demos/v0.87.1/codex_ollama_operational_skills_demo.md"
            | "demos/v0.89/gemma4_issue_clerk_demo.md"
            | "adl/Cargo.toml"
    ) || trimmed.starts_with("adl/src/cli/pr_cmd/")
        || trimmed.starts_with("adl/src/cli/pr_cmd_cards/")
        || trimmed.starts_with("adl/src/cli/tests/pr_cmd_inline/lifecycle/")
        || trimmed.starts_with("adl/src/agent_comms/")
        || trimmed.starts_with("adl/src/csdlc_prompt_editor/")
        || trimmed.starts_with("adl/src/cli/run_artifacts_types/")
        || trimmed.starts_with("adl/tools/skills/sprint-conductor/scripts/")
        || trimmed == "adl/src/runtime_v2/tests.rs"
        || trimmed.starts_with("adl/src/runtime_v2/tests/")
        || trimmed.starts_with("adl/tests/fixtures/scheduler/")
        || trimmed.starts_with("docs/milestones/v0.91.4/review/merge_readiness/")
        || trimmed.starts_with("adl/src/cli/tests/pr_cmd_inline/finish/")
        || trimmed == "docs/templates/prompts/current.json"
        || trimmed.starts_with("docs/templates/prompts/")
        || trimmed.starts_with("adl/tools/suitability_specs/")
        || trimmed.starts_with("docs/milestones/v0.91.6/review/provider/openrouter_current_models/")
}

fn finish_path_needs_pr_finish_rust_focused_validation(path: &str) -> bool {
    let trimmed = path.trim().trim_matches('/');
    trimmed == "adl/src/cli/pr_cmd/finish_support.rs"
        || trimmed.starts_with("adl/src/cli/tests/pr_cmd_inline/finish/")
}

fn finish_path_needs_pr_cmd_lifecycle_focused_validation(path: &str) -> bool {
    let trimmed = path.trim().trim_matches('/');
    trimmed == "adl/src/cli/pr_cmd.rs"
        || trimmed == "adl/src/cli/pr_cmd_args.rs"
        || trimmed == "adl/src/cli/pr_cmd_prompt.rs"
        || trimmed == "adl/src/cli/tests/pr_cmd_inline/basics.rs"
        || trimmed == "adl/src/cli/tests/pr_cmd_inline/versioned_bootstrap.rs"
        || trimmed == "adl/src/cli/tests/pr_cmd_inline/repo_helpers/metadata.rs"
        || trimmed == "adl/src/cli/tests/pr_cmd_inline/support.rs"
        || trimmed.starts_with("adl/src/cli/tests/pr_cmd_inline/lifecycle/")
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
    finish_path_matches_registry_proof_role(path, FinishPathProofRole::OwnerBinaryRustSlice)
}

fn finish_path_needs_process_status_focused_validation(path: &str) -> bool {
    let trimmed = path.trim().trim_matches('/');
    matches!(
        trimmed,
        "adl/src/cli/process_cmd.rs"
            | "adl/src/cli/usage.rs"
            | "adl/tests/cli_smoke.rs"
            | "adl/tests/cli_smoke/process_status.rs"
    )
}

fn finish_path_needs_scheduler_focused_validation(path: &str) -> bool {
    let trimmed = path.trim().trim_matches('/');
    trimmed == "adl/src/scheduler.rs" || trimmed.starts_with("adl/tests/fixtures/scheduler/")
}

fn finish_path_needs_github_release_focused_validation(path: &str) -> bool {
    let trimmed = path.trim().trim_matches('/');
    trimmed == "adl/src/cli/tooling_cmd/github_release.rs"
}

fn finish_path_needs_ci_log_archive_focused_validation(path: &str) -> bool {
    let trimmed = path.trim().trim_matches('/');
    matches!(
        trimmed,
        "adl/src/cli/tooling_cmd.rs"
            | "adl/src/cli/tooling_cmd/ci_log_archive.rs"
            | "adl/src/cli/tooling_cmd/tests/tooling_dispatch.rs"
    )
}

fn finish_path_needs_issue_resource_telemetry_focused_validation(path: &str) -> bool {
    let trimmed = path.trim().trim_matches('/');
    matches!(
        trimmed,
        "adl/src/cli/tooling_cmd.rs"
            | "adl/src/cli/tooling_cmd/issue_resource_telemetry.rs"
            | "adl/src/cli/tooling_cmd/tests/tooling_dispatch.rs"
    )
}

fn finish_path_needs_prompt_template_focused_validation(path: &str) -> bool {
    let trimmed = path.trim().trim_matches('/');
    matches!(
        trimmed,
        "adl/src/cli/tooling_cmd/common.rs"
            | "adl/src/cli/tooling_cmd/prompt_template.rs"
            | "adl/src/cli/tooling_cmd/structured_prompt.rs"
            | "adl/src/cli/tooling_cmd/tests/prompt_template.rs"
            | "adl/src/cli/tooling_cmd/tests/structured_prompt.rs"
            | "adl/src/cli/tooling_cmd/tests/support.rs"
            | "adl/src/bin/adl_prompt_template.rs"
            | "adl/src/bin/adl_validate_structured_prompt.rs"
            | "adl/tools/test_prompt_template_structure_schemas.py"
            | "adl/tools/test_prompt_template_workflow_integration.sh"
            | "docs/templates/prompts/current.json"
    ) || trimmed.starts_with("docs/templates/prompts/")
}

fn finish_path_needs_long_lived_agent_tokio_validation(path: &str) -> bool {
    let trimmed = path.trim().trim_matches('/');
    matches!(
        trimmed,
        "adl/src/long_lived_agent.rs"
            | "adl/src/long_lived_agent/inspection.rs"
            | "adl/src/long_lived_agent/schema.rs"
            | "adl/src/long_lived_agent/storage.rs"
            | "adl/src/long_lived_agent/tests.rs"
            | "adl/src/runtime_aws_signal.rs"
            | "adl/src/demo/stock_league/model.rs"
            | "adl/tests/cli_smoke/agent.rs"
            | "adl/tests/demo_tests.rs"
            | "adl/src/bin/run_v0916_integrated_runtime_soak.rs"
    )
}

fn finish_path_needs_tokio_bootstrap_helper_validation(path: &str) -> bool {
    let trimmed = path.trim().trim_matches('/');
    trimmed == "adl/src/cli/tokio_runtime.rs"
}

fn finish_path_needs_remote_exec_tokio_validation(path: &str) -> bool {
    let trimmed = path.trim().trim_matches('/');
    matches!(
        trimmed,
        "adl/src/execute/runner.rs"
            | "adl/src/execute/tests.rs"
            | "adl/src/remote_exec.rs"
            | "adl/src/remote_exec/signing_support.rs"
            | "adl/src/remote_exec/types.rs"
            | "adl/src/bin/run_v0916_integrated_runtime_soak.rs"
    )
}

fn finish_path_needs_cav_tokio_validation(path: &str) -> bool {
    let trimmed = path.trim().trim_matches('/');
    matches!(
        trimmed,
        "adl/src/continuous_verification_self_attack.rs"
            | "adl/src/cli/identity_cmd/tests/adversarial_contracts.rs"
    )
}

fn finish_issue_needs_tokio_manifest_runtime_validation(
    issue_number: u32,
    changed_paths: &[String],
) -> bool {
    if issue_number != 4178 {
        return false;
    }
    !changed_paths.is_empty()
        && changed_paths.iter().all(|path| {
            matches!(
                path.trim().trim_matches('/'),
                "adl/Cargo.toml" | "adl/Cargo.lock"
            )
        })
}

fn finish_issue_needs_issue_small_binary_validation(
    issue_number: u32,
    changed_paths: &[String],
) -> bool {
    if issue_number != 4216 {
        return false;
    }
    !changed_paths.is_empty()
        && changed_paths.iter().all(|path| {
            matches!(
                path.trim().trim_matches('/'),
                "adl/Cargo.toml"
                    | "adl/src/bin/adl_issue.rs"
                    | "adl/src/cli/pr_cmd/finish_support.rs"
                    | "adl/src/cli/tests/pr_cmd_inline/finish/arg_render.rs"
                    | "adl/tools/pr.sh"
                    | "adl/tools/test_ci_path_policy.sh"
                    | "adl/tools/test_pr_small_binary_delegation.sh"
            )
        })
}

fn finish_issue_needs_session_ledger_issue_validation(
    issue_number: u32,
    changed_paths: &[String],
) -> bool {
    if issue_number != 4419 {
        return false;
    }
    !changed_paths.is_empty()
        && changed_paths.iter().all(|path| {
            matches!(
                path.trim().trim_matches('/'),
                "adl/src/cli/pr_cmd.rs"
                    | "adl/src/cli/pr_cmd/doctor.rs"
                    | "adl/src/cli/pr_cmd/doctor/preflight.rs"
                    | "adl/src/cli/pr_cmd/doctor/printing.rs"
                    | "adl/src/cli/pr_cmd/doctor/tests.rs"
                    | "adl/src/cli/pr_cmd/doctor/types.rs"
                    | "adl/src/cli/pr_cmd/finish_support.rs"
                    | "adl/src/cli/tests/pr_cmd_inline/finish/arg_render.rs"
                    | "adl/src/cli/tests/pr_cmd_inline/lifecycle/start_ready.rs"
                    | "adl/src/session_ledger.rs"
            )
        })
}

fn finish_issue_needs_closing_linkage_small_binary_validation(
    issue_number: u32,
    changed_paths: &[String],
) -> bool {
    if issue_number != 4286 {
        return false;
    }
    !changed_paths.is_empty()
        && changed_paths.iter().all(|path| {
            matches!(
                path.trim().trim_matches('/'),
                "adl/Cargo.toml"
                    | "adl/src/bin/adl_pr_closing_linkage.rs"
                    | "adl/src/cli/pr_cmd.rs"
                    | "adl/src/cli/pr_cmd/github.rs"
                    | "adl/src/cli/pr_cmd/github/tests.rs"
                    | "adl/src/cli/pr_cmd/github/tests/closing_linkage.rs"
                    | "adl/src/cli/pr_cmd_args.rs"
                    | "adl/tools/check_pr_closing_linkage.sh"
                    | "adl/tools/pr.sh"
                    | "adl/tools/run_owner_validation_lane.sh"
                    | "adl/tools/test_pr_closing_linkage.sh"
                    | "adl/tools/test_pr_small_binary_delegation.sh"
                    | "docs/milestones/v0.91.6/review/github_projection/GITHUB_CSDLC_PROJECTION_MAP_4047.md"
                    | "adl/src/cli/pr_cmd/finish_support.rs"
                    | "adl/src/cli/tests/pr_cmd_inline/finish/arg_render.rs"
                    | ".adl/v0.91.6/tasks/issue-4286__typed-pr-closing-linkage-rust-pvf/spp.md"
                    | ".adl/v0.91.6/tasks/issue-4286__typed-pr-closing-linkage-rust-pvf/srp.md"
                    | ".adl/v0.91.6/tasks/issue-4286__typed-pr-closing-linkage-rust-pvf/sor.md"
            )
        })
}

fn finish_issue_needs_locked_cargo_fallback_validation(
    issue_number: u32,
    changed_paths: &[String],
) -> bool {
    if issue_number != 4306 {
        return false;
    }
    !changed_paths.is_empty()
        && changed_paths.iter().all(|path| {
            matches!(
                path.trim().trim_matches('/'),
                "adl/Cargo.lock"
                    | "adl/config/validation_lane_selector.v0.91.6.json"
                    | "adl/src/cli/pr_cmd/finish_support.rs"
                    | "adl/src/cli/tests/pr_cmd_inline/finish/arg_render.rs"
                    | "adl/tools/check_coverage_impact.sh"
                    | "adl/tools/pr.sh"
                    | "adl/tools/run_pr_fast_test_lane.sh"
                    | "adl/tools/run_owner_validation_lane.sh"
                    | "adl/tools/test_check_coverage_impact.sh"
                    | "adl/tools/test_control_plane_observability.sh"
                    | "adl/tools/test_five_command_regression_suite.sh"
                    | "adl/tools/test_pr_run_locked_cargo_fallback_refuses_cleanly.sh"
                    | "adl/tools/test_run_pr_fast_test_lane.sh"
            )
        })
}

fn finish_issue_needs_native_gws_runtime_validation(
    issue_number: u32,
    changed_paths: &[String],
) -> bool {
    if issue_number != 4406 {
        return false;
    }
    !changed_paths.is_empty()
        && changed_paths.iter().all(|path| {
            matches!(
                path.trim().trim_matches('/'),
                "adl/Cargo.toml"
                    | "adl/Cargo.lock"
                    | "adl/src/lib.rs"
                    | "adl/src/adl_gws_native.rs"
                    | "adl/src/adl_gws_drive_sync.rs"
                    | "adl/src/adl_gws_context_mirror.rs"
                    | "adl/src/bin/demo_adl_gws_native_drive_sync.rs"
                    | "adl/src/bin/demo_adl_gws_context_mirror.rs"
                    | "adl/src/cli/pr_cmd/finish_support.rs"
                    | "adl/src/cli/tests/pr_cmd_inline/finish/arg_render.rs"
            )
        })
}

fn finish_issue_needs_wuji_ddns_validation(issue_number: u32, changed_paths: &[String]) -> bool {
    if issue_number != 4284 {
        return false;
    }
    !changed_paths.is_empty()
        && changed_paths.iter().all(|path| {
            let trimmed = path.trim().trim_matches('/');
            trimmed == ".gitignore"
                || trimmed == "adl/src/cli/pr_cmd/finish_support.rs"
                || trimmed == "adl/src/cli/tests/pr_cmd_inline/finish/arg_render.rs"
                || trimmed.starts_with("infra/ddns/")
        })
}

fn finish_issue_needs_wuji_ddns_installer_validation(
    issue_number: u32,
    changed_paths: &[String],
) -> bool {
    if issue_number != 4330 {
        return false;
    }
    !changed_paths.is_empty()
        && changed_paths.iter().all(|path| {
            matches!(
                path.trim().trim_matches('/'),
                "adl/src/cli/pr_cmd/finish_support.rs"
                    | "adl/src/cli/tests/pr_cmd_inline/finish/arg_render.rs"
                    | "infra/ddns/README.md"
                    | "infra/ddns/client/install_wuji_ddns_launchd.sh"
            )
        })
}

fn build_tokio_manifest_runtime_validation_plan() -> FinishValidationPlan {
    let mut commands = Vec::new();
    push_finish_validation_command(
        &mut commands,
        "cargo fmt --manifest-path adl/Cargo.toml --all --check",
    );
    push_finish_validation_command(
        &mut commands,
        "cargo test --manifest-path adl/Cargo.toml pr_cmd::github",
    );
    push_finish_validation_command(
        &mut commands,
        "cargo test --manifest-path adl/Cargo.toml --bin adl github_release_",
    );
    push_finish_validation_command(
        &mut commands,
        "cargo test --manifest-path adl/Cargo.toml long_lived_agent",
    );
    FinishValidationPlan {
        mode: FinishValidationMode::LargerBinaryFocused,
        commands,
    }
}

fn build_locked_cargo_fallback_validation_plan() -> FinishValidationPlan {
    let mut commands = vec![
        "bash adl/tools/check_no_tracked_adl_issue_record_residue.sh".to_string(),
        "git diff --check".to_string(),
    ];
    push_finish_validation_command(
        &mut commands,
        "cargo fmt --manifest-path adl/Cargo.toml --all --check",
    );
    push_finish_validation_command(
        &mut commands,
        "cargo test --manifest-path adl/Cargo.toml --bin adl-pr-finish cli::pr_cmd::tests::finish::arg_render::finish_validation",
    );
    commands.push("bash adl/tools/test_ci_path_policy.sh".to_string());
    commands.push("bash adl/tools/run_owner_validation_lane.sh csdlc".to_string());
    FinishValidationPlan {
        mode: FinishValidationMode::LargerBinaryFocused,
        commands,
    }
}

fn build_native_gws_runtime_validation_plan() -> FinishValidationPlan {
    let mut commands = vec![
        "bash adl/tools/check_no_tracked_adl_issue_record_residue.sh".to_string(),
        "git diff --check".to_string(),
    ];
    push_finish_validation_command(
        &mut commands,
        "cargo fmt --manifest-path adl/Cargo.toml --all --check",
    );
    push_finish_validation_command(
        &mut commands,
        "cargo test --manifest-path adl/Cargo.toml adl_gws -- --nocapture",
    );
    push_finish_validation_command(
        &mut commands,
        "cargo test --manifest-path adl/Cargo.toml --bin adl-pr-finish cli::pr_cmd::tests::finish::arg_render::finish_validation_profile_classifies_native_gws_runtime_slice -- --exact --nocapture",
    );
    commands.push("bash adl/tools/run_owner_validation_lane.sh runtime --build".to_string());
    FinishValidationPlan {
        mode: FinishValidationMode::LargerBinaryFocused,
        commands,
    }
}

fn build_issue_small_binary_validation_plan() -> FinishValidationPlan {
    let mut commands = vec![
        "bash adl/tools/check_no_tracked_adl_issue_record_residue.sh".to_string(),
        "git diff --check".to_string(),
    ];
    push_finish_validation_command(
        &mut commands,
        "cargo fmt --manifest-path adl/Cargo.toml --all --check",
    );
    push_finish_validation_command(
        &mut commands,
        "cargo test --manifest-path adl/Cargo.toml --bin adl-issue tests::adl_issue_forwards_args_to_dispatch -- --exact --nocapture",
    );
    push_finish_validation_command(
        &mut commands,
        "cargo test --manifest-path adl/Cargo.toml --bin adl-pr-finish cli::pr_cmd::tests::finish::arg_render::finish_validation_profile_classifies_issue_small_binary_slice -- --exact --nocapture",
    );
    push_finish_validation_command(
        &mut commands,
        "bash adl/tools/test_pr_small_binary_delegation.sh",
    );
    FinishValidationPlan {
        mode: FinishValidationMode::SmallBinaryFocused,
        commands,
    }
}

fn build_session_ledger_issue_validation_plan() -> FinishValidationPlan {
    let mut commands = vec![
        "bash adl/tools/check_no_tracked_adl_issue_record_residue.sh".to_string(),
        "git diff --check".to_string(),
    ];
    push_finish_validation_command(
        &mut commands,
        "cargo fmt --manifest-path adl/Cargo.toml --all --check",
    );
    push_finish_validation_command(
        &mut commands,
        "cargo test --manifest-path adl/Cargo.toml target_claim_assessment_ -- --nocapture",
    );
    push_finish_validation_command(
        &mut commands,
        "cargo test --manifest-path adl/Cargo.toml doctor_preflight_ -- --nocapture",
    );
    push_finish_validation_command(
        &mut commands,
        "cargo test --manifest-path adl/Cargo.toml real_pr_start_blocks_when_another_session_claims_the_issue -- --exact --nocapture",
    );
    push_finish_validation_command(
        &mut commands,
        "cargo test --manifest-path adl/Cargo.toml real_pr_start_allows_current_session_claim_and_stale_claim_history -- --exact --nocapture",
    );
    push_finish_validation_command(
        &mut commands,
        "cargo test --manifest-path adl/Cargo.toml --bin adl-pr-finish cli::pr_cmd::tests::finish::arg_render::load_finish_validation_profile_cleans_tempfile_when_profile_only_needs_rendering -- --exact --nocapture",
    );
    push_finish_validation_command(
        &mut commands,
        "cargo test --manifest-path adl/Cargo.toml --bin adl-pr-finish cli::pr_cmd::tests::finish::arg_render::render_default_finish_validation_includes_profile_truth_and_sanitizes_changed_files -- --exact --nocapture",
    );
    push_finish_validation_command(
        &mut commands,
        "cargo test --manifest-path adl/Cargo.toml --bin adl-pr-finish cli::pr_cmd::tests::finish::arg_render::finish_validation_profile_classifies_session_ledger_issue_4419_slice -- --exact --nocapture",
    );
    FinishValidationPlan {
        mode: FinishValidationMode::LargerBinaryFocused,
        commands,
    }
}

fn build_closing_linkage_small_binary_validation_plan() -> FinishValidationPlan {
    let mut commands = vec![
        "bash adl/tools/check_no_tracked_adl_issue_record_residue.sh".to_string(),
        "git diff --check".to_string(),
    ];
    push_finish_validation_command(
        &mut commands,
        "cargo fmt --manifest-path adl/Cargo.toml --all --check",
    );
    push_finish_validation_command(
        &mut commands,
        "cargo test --manifest-path adl/Cargo.toml --bin adl-pr-closing-linkage closing_linkage -- --nocapture",
    );
    push_finish_validation_command(
        &mut commands,
        "cargo test --manifest-path adl/Cargo.toml --bin adl-pr-finish cli::pr_cmd::tests::finish::arg_render::finish_validation_profile_classifies_closing_linkage_small_binary_slice -- --exact --nocapture",
    );
    push_finish_validation_command(&mut commands, "bash adl/tools/test_pr_closing_linkage.sh");
    push_finish_validation_command(
        &mut commands,
        "bash adl/tools/test_pr_small_binary_delegation.sh",
    );
    FinishValidationPlan {
        mode: FinishValidationMode::SmallBinaryFocused,
        commands,
    }
}

fn build_wuji_ddns_validation_plan() -> FinishValidationPlan {
    FinishValidationPlan {
        mode: FinishValidationMode::LargerBinaryFocused,
        commands: vec![
            "bash adl/tools/check_no_tracked_adl_issue_record_residue.sh".to_string(),
            "git diff --check".to_string(),
            "cargo test --manifest-path adl/Cargo.toml --bin adl-pr-finish wuji_ddns_slice -- --nocapture".to_string(),
            "python3 -m unittest infra/ddns/tests/test_handler.py".to_string(),
            "sh -n infra/ddns/client/wuji_ddns_update.sh".to_string(),
            "terraform -chdir=infra/ddns fmt -check".to_string(),
            "terraform -chdir=infra/ddns init -backend=false".to_string(),
            "terraform -chdir=infra/ddns validate".to_string(),
        ],
    }
}

fn build_wuji_ddns_installer_validation_plan() -> FinishValidationPlan {
    FinishValidationPlan {
        mode: FinishValidationMode::LargerBinaryFocused,
        commands: vec![
            "bash adl/tools/check_no_tracked_adl_issue_record_residue.sh".to_string(),
            "git diff --check".to_string(),
            "cargo test --manifest-path adl/Cargo.toml --bin adl-pr-finish wuji_ddns_installer_slice -- --nocapture".to_string(),
            "sh -n infra/ddns/client/install_wuji_ddns_launchd.sh".to_string(),
            "sh -n infra/ddns/client/wuji_ddns_update.sh".to_string(),
        ],
    }
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
            | "adl/tools/run_pr_fast_test_lane.sh"
            | "adl/tools/test_run_pr_fast_test_lane.sh"
    )
}

fn finish_path_needs_ci_runtime_contract_validation(path: &str) -> bool {
    let trimmed = path.trim().trim_matches('/');
    matches!(
        trimmed,
        ".github/workflows/ci.yaml"
            | "adl/tools/test_ci_runtime_contracts.sh"
            | "adl/tools/run_authoritative_coverage_lane.sh"
            | "adl/tools/test_run_authoritative_coverage_lane.sh"
    )
}

fn finish_path_needs_validation_selector_focused_validation(path: &str) -> bool {
    let trimmed = path.trim().trim_matches('/');
    matches!(
        trimmed,
        "adl/config/validation_lane_selector.v0.91.6.json"
            | "adl/tools/select_validation_lanes.py"
            | "adl/tools/select_validation_lanes.sh"
            | "adl/tools/test_select_validation_lanes.sh"
    )
}

fn finish_path_needs_slow_proof_family_focused_validation(path: &str) -> bool {
    let trimmed = path.trim().trim_matches('/');
    matches!(
        trimmed,
        "adl/Cargo.toml"
            | "adl/config/slow_proof_families.v0.91.6.json"
            | "adl/tools/run_slow_proof_family.sh"
            | "adl/tools/test_slow_proof_lane_contract.sh"
    ) || trimmed == "adl/src/runtime_v2/tests.rs"
        || trimmed.starts_with("adl/src/runtime_v2/tests/")
}

fn finish_path_needs_validation_manager_focused_validation(path: &str) -> bool {
    let trimmed = path.trim().trim_matches('/');
    matches!(
        trimmed,
        "adl/tools/validation_manager.py"
            | "adl/tools/validation_manager.sh"
            | "adl/tools/test_validation_manager.sh"
    )
}

fn finish_path_needs_validation_inventory_focused_validation(path: &str) -> bool {
    let trimmed = path.trim().trim_matches('/');
    matches!(
        trimmed,
        "adl/tools/validation_inventory.py"
            | "adl/tools/validation_inventory.sh"
            | "adl/tools/test_validation_inventory.sh"
    )
}

fn finish_path_needs_sprint_conductor_helper_validation(path: &str) -> bool {
    let trimmed = path.trim().trim_matches('/');
    trimmed.starts_with("adl/tools/skills/sprint-conductor/scripts/")
}

fn parsed_issue_is_4031_unity_observatory_scaffold_path(issue: u32, path: &str) -> bool {
    if issue != 4031 {
        return false;
    }
    let trimmed = path.trim().trim_matches('/');
    matches!(
        trimmed,
        "demos/v0.91.6/unity-observatory/PROOF_PACKET.md"
            | "demos/v0.91.6/unity-observatory/Assets/Scenes/UnityObservatory.unity"
            | "demos/v0.91.6/unity-observatory/Assets/Scenes/UnityObservatory.unity.meta"
            | "demos/v0.91.6/unity-observatory/Assets/Scripts/UnityObservatoryBootstrap.cs"
            | "demos/v0.91.6/unity-observatory/Assets/Scripts/UnityObservatoryBootstrap.cs.meta"
            | "demos/v0.91.6/unity-observatory/Assets/Scripts/UnityObservatoryShellController.cs"
            | "demos/v0.91.6/unity-observatory/Assets/Scripts/UnityObservatoryShellController.cs.meta"
            | "demos/v0.91.6/unity-observatory/Assets/UI/ObservatoryShell.uxml"
            | "demos/v0.91.6/unity-observatory/Assets/UI/ObservatoryShell.uxml.meta"
            | "demos/v0.91.6/unity-observatory/Assets/UI/ObservatoryShell.uss"
            | "demos/v0.91.6/unity-observatory/Assets/UI/ObservatoryShell.uss.meta"
            | "demos/v0.91.6/unity-observatory/Packages/manifest.json"
            | "demos/v0.91.6/unity-observatory/ProjectSettings/EditorBuildSettings.asset"
            | "demos/v0.91.6/unity-observatory/ProjectSettings/ProjectVersion.txt"
    )
}

fn finish_path_needs_unity_observatory_baseline_validation(issue: u32, path: &str) -> bool {
    let trimmed = path.trim().trim_matches('/');
    trimmed == "adl/tools/test_v0916_unity_observatory_baseline.sh"
        || trimmed == "demos/v0.91.6/unity-observatory/README.md"
        || trimmed == "demos/v0.91.6/unity-observatory/PROOF_PACKET.md"
        || parsed_issue_is_4031_unity_observatory_scaffold_path(issue, trimmed)
}

fn parsed_issue_is_unity_observatory_contract_path(issue: u32, path: &str) -> bool {
    if !matches!(issue, 4032..=4034 | 4416) {
        return false;
    }
    let trimmed = path.trim().trim_matches('/');
    matches!(
        trimmed,
        "demos/v0.91.6/unity-observatory/README.md"
            | "demos/v0.91.6/unity-observatory/PROOF_PACKET.md"
            | "demos/v0.91.6/unity-observatory/Assets/Resources.meta"
            | "demos/v0.91.6/unity-observatory/Assets/Resources/observatory_contract.json"
            | "demos/v0.91.6/unity-observatory/Assets/Resources/observatory_contract.json.meta"
            | "demos/v0.91.6/unity-observatory/Assets/Scripts/UnityObservatoryBootstrap.cs"
            | "demos/v0.91.6/unity-observatory/Assets/Scripts/UnityObservatoryShellController.cs"
            | "demos/v0.91.6/unity-observatory/Assets/UI/ObservatoryShell.uxml"
            | "adl/src/csm_observatory.rs"
            | "adl/tests/cli_smoke/instrument_and_cli.rs"
            | "adl/tools/test_v0916_unity_observatory_baseline.sh"
            | "adl/tools/test_v0916_unity_observatory_contract.sh"
    )
}

fn parsed_issue_is_html_observatory_path(issue: u32, path: &str) -> bool {
    if issue != 4341 {
        return false;
    }
    let trimmed = path.trim().trim_matches('/');
    matches!(
        trimmed,
        "adl/tools/test_demo_v0904_csm_observatory_governed_prototype.sh"
            | "adl/tools/validate_csm_governed_observatory.py"
            | "demos/fixtures/csm_observatory/proto-csm-02-governed-observatory-packet.json"
            | "demos/v0.90.4/csm_observatory_governed_prototype.html"
            | "demos/v0.90.4/csm_observatory_governed_prototype.css"
            | "demos/v0.90.4/csm_observatory_governed_prototype.js"
            | "demos/v0.90.4/csm_observatory_governed_prototype.md"
            | "docs/milestones/v0.91.6/review/observatory/HTML_MOBILE_GOVERNED_OBSERVATORY_PROOF_4341.md"
    )
}

fn finish_path_needs_small_binary_delegation_validation(path: &str) -> bool {
    let trimmed = path.trim().trim_matches('/');
    matches!(
        trimmed,
        "adl/tools/observability.sh"
            | "adl/tools/test_pr_delegate_cargo_fallback_liveness.sh"
            | "adl/tools/test_pr_delegate_prefers_primary_checkout_binary.sh"
            | "adl/tools/test_pr_small_binary_delegation.sh"
    )
}

fn finish_path_needs_locked_cargo_fallback_validation(path: &str) -> bool {
    let trimmed = path.trim().trim_matches('/');
    matches!(
        trimmed,
        "adl/Cargo.lock"
            | "adl/tools/pr.sh"
            | "adl/tools/test_five_command_regression_suite.sh"
            | "adl/tools/test_pr_run_locked_cargo_fallback_refuses_cleanly.sh"
    )
}

fn finish_path_needs_owner_lane_contract_validation(path: &str) -> bool {
    finish_path_matches_registry_proof_role(path, FinishPathProofRole::OwnerLaneContract)
}

fn finish_path_needs_repo_quality_staleness_validation(path: &str) -> bool {
    let trimmed = path.trim().trim_matches('/');
    matches!(
        trimmed,
        "adl/tools/check_repo_quality_staleness.py"
            | "adl/tools/test_check_repo_quality_staleness.sh"
    )
}

fn finish_path_needs_deepseek_suitability_validation(path: &str) -> bool {
    let trimmed = path.trim().trim_matches('/');
    matches!(
        trimmed,
        "adl/tools/run_v0916_agent_suitability_panel.py"
            | "adl/tools/run_v0916_deepseek_suitability.py"
            | "adl/tools/validate_v0916_agent_suitability_panel.py"
            | "adl/tools/validate_v0916_deepseek_suitability.py"
            | "adl/tools/test_v0916_deepseek_suitability.sh"
            | "adl/tools/suitability_specs/deepseek_csdlc_panel_4096.json"
    ) || trimmed.starts_with("adl/tools/suitability_specs/")
        || trimmed.starts_with("docs/milestones/v0.91.6/review/provider/openrouter_current_models/")
        || trimmed.starts_with("docs/milestones/v0.91.6/review/provider/openrouter_next_tranche_")
}

fn finish_path_needs_private_endpoint_fixture_sanitation_validation(path: &str) -> bool {
    let trimmed = path.trim().trim_matches('/');
    matches!(
        trimmed,
        "adl/tools/demo_codex_ollama_operational_skills.sh"
            | "adl/tools/demo_v089_gemma4_issue_clerk.sh"
            | "adl/tools/test_demo_codex_ollama_operational_skills.sh"
            | "adl/tools/test_demo_codex_ollama_semantic_fallback.sh"
            | "adl/tools/test_demo_v089_gemma4_issue_clerk.sh"
            | "adl/src/provider_substrate.rs"
            | "adl/tools/validate_v0915_remote_gemma_watcher_probe.py"
            | "demos/v0.87.1/codex_ollama_operational_skills_demo.md"
            | "demos/v0.89/gemma4_issue_clerk_demo.md"
    )
}

fn finish_path_needs_csdlc_owner_lane_validation(path: &str) -> bool {
    finish_path_matches_registry_proof_role(path, FinishPathProofRole::CsdlcOwnerLane)
        || finish_path_matches_registry_owner(path, FinishPathOwnerBinary::Csdlc)
}

fn finish_path_needs_runtime_owner_lane_validation(path: &str) -> bool {
    let trimmed = path.trim().trim_matches('/');
    matches!(
        trimmed,
        "adl/tools/test_adl_runtime_compatibility.sh"
            | "adl/src/agent_comms.rs"
            | "adl/src/provider_adapter.rs"
            | "adl/src/provider_communication.rs"
            | "adl/src/resilience.rs"
            | "adl/src/bin/run_v0916_integrated_runtime_soak.rs"
    ) || trimmed.starts_with("adl/src/agent_comms/")
}

fn finish_path_needs_agent_comms_focused_validation(path: &str) -> bool {
    let trimmed = path.trim().trim_matches('/');
    trimmed == "adl/src/agent_comms.rs" || trimmed.starts_with("adl/src/agent_comms/")
}

fn finish_path_needs_provider_adapter_focused_validation(path: &str) -> bool {
    let trimmed = path.trim().trim_matches('/');
    trimmed == "adl/src/provider_adapter.rs"
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
                "cargo test --manifest-path adl/Cargo.toml --bin adl cli::pr_cmd::github::tests -- --nocapture" => {
                    run_finish_validation_status(
                        "cargo",
                        &[
                            "test",
                            "--manifest-path",
                            path_str(&manifest)?,
                            "--bin",
                            "adl",
                            "cli::pr_cmd::github::tests",
                            "--",
                            "--nocapture",
                        ],
                    )?;
                }
                "cargo test --manifest-path adl/Cargo.toml --bin adl validation_disposition_blocks_pending_and_terminal_failures -- --nocapture" => {
                    run_finish_validation_status(
                        "cargo",
                        &[
                            "test",
                            "--manifest-path",
                            path_str(&manifest)?,
                            "--bin",
                            "adl",
                            "validation_disposition_blocks_pending_and_terminal_failures",
                            "--",
                            "--nocapture",
                        ],
                    )?;
                }
                "cargo test --manifest-path adl/Cargo.toml --bin adl" => {
                    run_finish_validation_status(
                        "cargo",
                        &["test", "--manifest-path", path_str(&manifest)?, "--bin", "adl"],
                    )?;
                }
                "cargo test --manifest-path adl/Cargo.toml --bin adl github_release_" => {
                    run_finish_validation_status(
                        "cargo",
                        &[
                            "test",
                            "--manifest-path",
                            path_str(&manifest)?,
                            "--bin",
                            "adl",
                            "github_release_",
                        ],
                    )?;
                }
                "cargo test --manifest-path adl/Cargo.toml ci_log_archive -- --nocapture" => {
                    run_finish_validation_status(
                        "cargo",
                        &[
                            "test",
                            "--manifest-path",
                            path_str(&manifest)?,
                            "ci_log_archive",
                            "--",
                            "--nocapture",
                        ],
                    )?;
                }
                "cargo test --manifest-path adl/Cargo.toml issue_resource_telemetry -- --nocapture" => {
                    run_finish_validation_status(
                        "cargo",
                        &[
                            "test",
                            "--manifest-path",
                            path_str(&manifest)?,
                            "issue_resource_telemetry",
                            "--",
                            "--nocapture",
                        ],
                    )?;
                }
                "cargo test --manifest-path adl/Cargo.toml tooling_cmd_dispatch_and_help_paths_cover_public_entrypoint -- --nocapture" => {
                    run_finish_validation_status(
                        "cargo",
                        &[
                            "test",
                            "--manifest-path",
                            path_str(&manifest)?,
                            "tooling_cmd_dispatch_and_help_paths_cover_public_entrypoint",
                            "--",
                            "--nocapture",
                        ],
                    )?;
                }
                "cargo test --manifest-path adl/Cargo.toml pr_cmd::github" => {
                    run_finish_validation_status(
                        "cargo",
                        &["test", "--manifest-path", path_str(&manifest)?, "pr_cmd::github"],
                    )?;
                }
                "cargo test --manifest-path adl/Cargo.toml long_lived_agent" => {
                    run_finish_validation_status(
                        "cargo",
                        &["test", "--manifest-path", path_str(&manifest)?, "long_lived_agent"],
                    )?;
                }
                "cargo test --manifest-path adl/Cargo.toml --bin adl octocrab_transport_" => {
                    run_finish_validation_status(
                        "cargo",
                        &[
                            "test",
                            "--manifest-path",
                            path_str(&manifest)?,
                            "--bin",
                            "adl",
                            "octocrab_transport_",
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
                "cargo test --manifest-path adl/Cargo.toml build_remote_execute_request_preserves_conversation_as_audit_metadata" => {
                    run_finish_validation_status(
                        "cargo",
                        &[
                            "test",
                            "--manifest-path",
                            path_str(&manifest)?,
                            "build_remote_execute_request_preserves_conversation_as_audit_metadata",
                        ],
                    )?;
                }
                "cargo test --manifest-path adl/Cargo.toml execute_step_with_retry_does_not_retry_remote_schema_violation" => {
                    run_finish_validation_status(
                        "cargo",
                        &[
                            "test",
                            "--manifest-path",
                            path_str(&manifest)?,
                            "execute_step_with_retry_does_not_retry_remote_schema_violation",
                        ],
                    )?;
                }
                "cargo test --manifest-path adl/Cargo.toml security_envelope_rejects_tampered_signed_conversation_metadata" => {
                    run_finish_validation_status(
                        "cargo",
                        &[
                            "test",
                            "--manifest-path",
                            path_str(&manifest)?,
                            "security_envelope_rejects_tampered_signed_conversation_metadata",
                        ],
                    )?;
                }
                "cargo test --manifest-path adl/Cargo.toml remote_exec::" => {
                    run_finish_validation_status(
                        "cargo",
                        &["test", "--manifest-path", path_str(&manifest)?, "remote_exec::"],
                    )?;
                }
                "cargo test --manifest-path adl/Cargo.toml --lib scheduler_economics" => {
                    run_finish_validation_status(
                        "cargo",
                        &[
                            "test",
                            "--manifest-path",
                            path_str(&manifest)?,
                            "--lib",
                            "scheduler_economics",
                        ],
                    )?;
                }
                "cargo test --manifest-path adl/Cargo.toml continuous_verification_contract_covers_cadence_lifecycle_and_artifacts" => {
                    run_finish_validation_status(
                        "cargo",
                        &[
                            "test",
                            "--manifest-path",
                            path_str(&manifest)?,
                            "continuous_verification_contract_covers_cadence_lifecycle_and_artifacts",
                        ],
                    )?;
                }
                "cargo test --manifest-path adl/Cargo.toml self_attack_contract_is_policy_bounded_and_reviewable" => {
                    run_finish_validation_status(
                        "cargo",
                        &[
                            "test",
                            "--manifest-path",
                            path_str(&manifest)?,
                            "self_attack_contract_is_policy_bounded_and_reviewable",
                        ],
                    )?;
                }
                "cargo test --manifest-path adl/Cargo.toml identity_continuous_verification_writes_contract_json" => {
                    run_finish_validation_status(
                        "cargo",
                        &[
                            "test",
                            "--manifest-path",
                            path_str(&manifest)?,
                            "identity_continuous_verification_writes_contract_json",
                        ],
                    )?;
                }
                "cargo test --manifest-path adl/Cargo.toml --test cli_smoke process_status -- --nocapture" => {
                    run_finish_validation_status(
                        "cargo",
                        &[
                            "test",
                            "--manifest-path",
                            path_str(&manifest)?,
                            "--test",
                            "cli_smoke",
                            "process_status",
                            "--",
                            "--nocapture",
                        ],
                    )?;
                }
                "cargo test --manifest-path adl/Cargo.toml --test cli_smoke csm_observatory_cli_writes_unity_contract_bundle_and_matches_seeded_resource -- --nocapture" => {
                    run_finish_validation_status(
                        "cargo",
                        &[
                            "test",
                            "--manifest-path",
                            path_str(&manifest)?,
                            "--test",
                            "cli_smoke",
                            "csm_observatory_cli_writes_unity_contract_bundle_and_matches_seeded_resource",
                            "--",
                            "--nocapture",
                        ],
                    )?;
                }
                "cargo test --manifest-path adl/Cargo.toml unity_observatory_contract_ -- --nocapture" => {
                    run_finish_validation_status(
                        "cargo",
                        &[
                            "test",
                            "--manifest-path",
                            path_str(&manifest)?,
                            "unity_observatory_contract_",
                            "--",
                            "--nocapture",
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
                "cargo test --manifest-path adl/Cargo.toml --bin adl-pr-finish cli::pr_cmd::tests::finish::arg_render::finish_validation_profile_classifies_unity_observatory_contract_slice_as_small_binary_focused -- --nocapture" => {
                    run_finish_validation_status(
                        "cargo",
                        &[
                            "test",
                            "--manifest-path",
                            path_str(&manifest)?,
                            "--bin",
                            "adl-pr-finish",
                            "cli::pr_cmd::tests::finish::arg_render::finish_validation_profile_classifies_unity_observatory_contract_slice_as_small_binary_focused",
                            "--",
                            "--nocapture",
                        ],
                    )?;
                }
                "cargo test --manifest-path adl/Cargo.toml --bin adl-pr-finish cli::pr_cmd::tests::finish::arg_render::finish_validation_profile_classifies_html_mobile_observatory_slice_as_small_binary_focused -- --nocapture" => {
                    run_finish_validation_status(
                        "cargo",
                        &[
                            "test",
                            "--manifest-path",
                            path_str(&manifest)?,
                            "--bin",
                            "adl-pr-finish",
                            "cli::pr_cmd::tests::finish::arg_render::finish_validation_profile_classifies_html_mobile_observatory_slice_as_small_binary_focused",
                            "--",
                            "--nocapture",
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
                "cargo metadata --manifest-path adl/Cargo.toml --no-deps --format-version 1" => {
                    run_finish_validation_status(
                        "cargo",
                        &[
                            "metadata",
                            "--manifest-path",
                            path_str(&manifest)?,
                            "--no-deps",
                            "--format-version",
                            "1",
                        ],
                    )?;
                }
                "cargo metadata --manifest-path adl/Cargo.toml --locked --no-deps --format-version 1" => {
                    run_finish_validation_status(
                        "cargo",
                        &[
                            "metadata",
                            "--manifest-path",
                            path_str(&manifest)?,
                            "--locked",
                            "--no-deps",
                            "--format-version",
                            "1",
                        ],
                    )?;
                }
                "cargo test --manifest-path adl/Cargo.toml --bin adl-issue tests::adl_issue_forwards_args_to_dispatch -- --exact --nocapture" => {
                    run_finish_validation_status(
                        "cargo",
                        &[
                            "test",
                            "--manifest-path",
                            path_str(&manifest)?,
                            "--bin",
                            "adl-issue",
                            "tests::adl_issue_forwards_args_to_dispatch",
                            "--",
                            "--exact",
                            "--nocapture",
                        ],
                    )?;
                }
                "cargo test --manifest-path adl/Cargo.toml --bin adl-pr-finish cli::pr_cmd::tests::finish::arg_render::finish_validation_profile_classifies_issue_small_binary_slice -- --exact --nocapture" => {
                    run_finish_validation_status(
                        "cargo",
                        &[
                            "test",
                            "--manifest-path",
                            path_str(&manifest)?,
                            "--bin",
                            "adl-pr-finish",
                            "cli::pr_cmd::tests::finish::arg_render::finish_validation_profile_classifies_issue_small_binary_slice",
                            "--",
                            "--exact",
                            "--nocapture",
                        ],
                    )?;
                }
                "cargo test --manifest-path adl/Cargo.toml target_claim_assessment_ -- --nocapture" => {
                    run_finish_validation_status(
                        "cargo",
                        &[
                            "test",
                            "--manifest-path",
                            path_str(&manifest)?,
                            "target_claim_assessment_",
                            "--",
                            "--nocapture",
                        ],
                    )?;
                }
                "cargo test --manifest-path adl/Cargo.toml doctor_preflight_ -- --nocapture" => {
                    run_finish_validation_status(
                        "cargo",
                        &[
                            "test",
                            "--manifest-path",
                            path_str(&manifest)?,
                            "doctor_preflight_",
                            "--",
                            "--nocapture",
                        ],
                    )?;
                }
                "cargo test --manifest-path adl/Cargo.toml real_pr_start_blocks_when_another_session_claims_the_issue -- --exact --nocapture" => {
                    run_finish_validation_status(
                        "cargo",
                        &[
                            "test",
                            "--manifest-path",
                            path_str(&manifest)?,
                            "real_pr_start_blocks_when_another_session_claims_the_issue",
                            "--",
                            "--exact",
                            "--nocapture",
                        ],
                    )?;
                }
                "cargo test --manifest-path adl/Cargo.toml real_pr_start_allows_current_session_claim_and_stale_claim_history -- --exact --nocapture" => {
                    run_finish_validation_status(
                        "cargo",
                        &[
                            "test",
                            "--manifest-path",
                            path_str(&manifest)?,
                            "real_pr_start_allows_current_session_claim_and_stale_claim_history",
                            "--",
                            "--exact",
                            "--nocapture",
                        ],
                    )?;
                }
                "cargo test --manifest-path adl/Cargo.toml --bin adl-pr-finish cli::pr_cmd::tests::finish::arg_render::load_finish_validation_profile_cleans_tempfile_when_profile_only_needs_rendering -- --exact --nocapture" => {
                    run_finish_validation_status(
                        "cargo",
                        &[
                            "test",
                            "--manifest-path",
                            path_str(&manifest)?,
                            "--bin",
                            "adl-pr-finish",
                            "cli::pr_cmd::tests::finish::arg_render::load_finish_validation_profile_cleans_tempfile_when_profile_only_needs_rendering",
                            "--",
                            "--exact",
                            "--nocapture",
                        ],
                    )?;
                }
                "cargo test --manifest-path adl/Cargo.toml --bin adl-pr-finish cli::pr_cmd::tests::finish::arg_render::render_default_finish_validation_includes_profile_truth_and_sanitizes_changed_files -- --exact --nocapture" => {
                    run_finish_validation_status(
                        "cargo",
                        &[
                            "test",
                            "--manifest-path",
                            path_str(&manifest)?,
                            "--bin",
                            "adl-pr-finish",
                            "cli::pr_cmd::tests::finish::arg_render::render_default_finish_validation_includes_profile_truth_and_sanitizes_changed_files",
                            "--",
                            "--exact",
                            "--nocapture",
                        ],
                    )?;
                }
                "cargo test --manifest-path adl/Cargo.toml --bin adl-pr-finish cli::pr_cmd::tests::finish::arg_render::finish_validation_profile_classifies_session_ledger_issue_4419_slice -- --exact --nocapture" => {
                    run_finish_validation_status(
                        "cargo",
                        &[
                            "test",
                            "--manifest-path",
                            path_str(&manifest)?,
                            "--bin",
                            "adl-pr-finish",
                            "cli::pr_cmd::tests::finish::arg_render::finish_validation_profile_classifies_session_ledger_issue_4419_slice",
                            "--",
                            "--exact",
                            "--nocapture",
                        ],
                    )?;
                }
                "cargo test --manifest-path adl/Cargo.toml --bin adl-pr-closing-linkage closing_linkage -- --nocapture" => {
                    run_finish_validation_status(
                        "cargo",
                        &[
                            "test",
                            "--manifest-path",
                            path_str(&manifest)?,
                            "--bin",
                            "adl-pr-closing-linkage",
                            "closing_linkage",
                            "--",
                            "--nocapture",
                        ],
                    )?;
                }
                "cargo test --manifest-path adl/Cargo.toml --bin adl-pr-finish cli::pr_cmd::tests::finish::arg_render::finish_validation_profile_classifies_closing_linkage_small_binary_slice -- --exact --nocapture" => {
                    run_finish_validation_status(
                        "cargo",
                        &[
                            "test",
                            "--manifest-path",
                            path_str(&manifest)?,
                            "--bin",
                            "adl-pr-finish",
                            "cli::pr_cmd::tests::finish::arg_render::finish_validation_profile_classifies_closing_linkage_small_binary_slice",
                            "--",
                            "--exact",
                            "--nocapture",
                        ],
                    )?;
                }
                "cargo test --manifest-path adl/Cargo.toml --bin adl-pr-finish wuji_ddns_slice -- --nocapture" => {
                    run_finish_validation_status(
                        "cargo",
                        &[
                            "test",
                            "--manifest-path",
                            path_str(&manifest)?,
                            "--bin",
                            "adl-pr-finish",
                            "wuji_ddns_slice",
                            "--",
                            "--nocapture",
                        ],
                    )?;
                }
                "cargo test --manifest-path adl/Cargo.toml --bin adl-pr-finish wuji_ddns_installer_slice -- --nocapture" => {
                    run_finish_validation_status(
                        "cargo",
                        &[
                            "test",
                            "--manifest-path",
                            path_str(&manifest)?,
                            "--bin",
                            "adl-pr-finish",
                            "wuji_ddns_installer_slice",
                            "--",
                            "--nocapture",
                        ],
                    )?;
                }
                "python3 -m unittest infra/ddns/tests/test_handler.py" => {
                    run_finish_validation_status(
                        "python3",
                        &["-m", "unittest", "infra/ddns/tests/test_handler.py"],
                    )?;
                }
                "sh -n infra/ddns/client/wuji_ddns_update.sh" => {
                    run_finish_validation_status("sh", &["-n", "infra/ddns/client/wuji_ddns_update.sh"])?;
                }
                "sh -n infra/ddns/client/install_wuji_ddns_launchd.sh" => {
                    run_finish_validation_status(
                        "sh",
                        &["-n", "infra/ddns/client/install_wuji_ddns_launchd.sh"],
                    )?;
                }
                "terraform -chdir=infra/ddns fmt -check" => {
                    run_finish_validation_status(
                        "terraform",
                        &["-chdir=infra/ddns", "fmt", "-check"],
                    )?;
                }
                "terraform -chdir=infra/ddns init -backend=false" => {
                    run_finish_validation_status(
                        "terraform",
                        &["-chdir=infra/ddns", "init", "-backend=false"],
                    )?;
                }
                "terraform -chdir=infra/ddns validate" => {
                    run_finish_validation_status(
                        "terraform",
                        &["-chdir=infra/ddns", "validate"],
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
                "bash adl/tools/test_ci_runtime_contracts.sh" => {
                    let script = repo_root.join("adl/tools/test_ci_runtime_contracts.sh");
                    run_finish_validation_status("bash", &[path_str(&script)?])?;
                }
                "bash adl/tools/test_run_pr_fast_test_lane.sh" => {
                    let script = repo_root.join("adl/tools/test_run_pr_fast_test_lane.sh");
                    run_finish_validation_status("bash", &[path_str(&script)?])?;
                }
                "bash adl/tools/test_select_validation_lanes.sh" => {
                    let script = repo_root.join("adl/tools/test_select_validation_lanes.sh");
                    run_finish_validation_status("bash", &[path_str(&script)?])?;
                }
                "bash adl/tools/test_demo_v0904_csm_observatory_governed_prototype.sh" => {
                    let script =
                        repo_root.join("adl/tools/test_demo_v0904_csm_observatory_governed_prototype.sh");
                    run_finish_validation_status("bash", &[path_str(&script)?])?;
                }
                "bash adl/tools/test_validation_manager.sh" => {
                    let script = repo_root.join("adl/tools/test_validation_manager.sh");
                    run_finish_validation_status("bash", &[path_str(&script)?])?;
                }
                "bash adl/tools/test_ci_path_policy.sh && bash adl/tools/test_select_validation_lanes.sh && bash adl/tools/test_validation_manager.sh" => {
                    let ci_path_policy = repo_root.join("adl/tools/test_ci_path_policy.sh");
                    run_finish_validation_status("bash", &[path_str(&ci_path_policy)?])?;
                    let select_validation_lanes =
                        repo_root.join("adl/tools/test_select_validation_lanes.sh");
                    run_finish_validation_status("bash", &[path_str(&select_validation_lanes)?])?;
                    let validation_manager =
                        repo_root.join("adl/tools/test_validation_manager.sh");
                    run_finish_validation_status("bash", &[path_str(&validation_manager)?])?;
                }
                "bash adl/tools/test_validation_inventory.sh" => {
                    let script = repo_root.join("adl/tools/test_validation_inventory.sh");
                    run_finish_validation_status("bash", &[path_str(&script)?])?;
                }
                "bash adl/tools/test_v0916_unity_observatory_baseline.sh" => {
                    let script = repo_root.join("adl/tools/test_v0916_unity_observatory_baseline.sh");
                    run_finish_validation_status("bash", &[path_str(&script)?])?;
                }
                "bash adl/tools/test_v0916_unity_observatory_contract.sh" => {
                    let script = repo_root.join("adl/tools/test_v0916_unity_observatory_contract.sh");
                    run_finish_validation_status("bash", &[path_str(&script)?])?;
                }
                "bash adl/tools/test_slow_proof_lane_contract.sh" => {
                    let script = repo_root.join("adl/tools/test_slow_proof_lane_contract.sh");
                    run_finish_validation_status("bash", &[path_str(&script)?])?;
                }
                "bash adl/tools/test_pr_small_binary_delegation.sh" => {
                    let script = repo_root.join("adl/tools/test_pr_small_binary_delegation.sh");
                    run_finish_validation_status("bash", &[path_str(&script)?])?;
                }
                "bash adl/tools/test_pr_closing_linkage.sh" => {
                    let script = repo_root.join("adl/tools/test_pr_closing_linkage.sh");
                    run_finish_validation_status("bash", &[path_str(&script)?])?;
                }
                "bash adl/tools/test_pr_run_locked_cargo_fallback_refuses_cleanly.sh" => {
                    let script =
                        repo_root.join("adl/tools/test_pr_run_locked_cargo_fallback_refuses_cleanly.sh");
                    run_finish_validation_status("bash", &[path_str(&script)?])?;
                }
                "bash adl/tools/test_owner_validation_lane.sh" => {
                    let script = repo_root.join("adl/tools/test_owner_validation_lane.sh");
                    run_finish_validation_status("bash", &[path_str(&script)?])?;
                }
                "bash adl/tools/test_check_repo_quality_staleness.sh" => {
                    let script = repo_root.join("adl/tools/test_check_repo_quality_staleness.sh");
                    run_finish_validation_status("bash", &[path_str(&script)?])?;
                }
                "bash adl/tools/test_v0916_deepseek_suitability.sh" => {
                    let script = repo_root.join("adl/tools/test_v0916_deepseek_suitability.sh");
                    run_finish_validation_status("bash", &[path_str(&script)?])?;
                }
                "bash adl/tools/test_demo_codex_ollama_operational_skills.sh" => {
                    let script = repo_root.join("adl/tools/test_demo_codex_ollama_operational_skills.sh");
                    run_finish_validation_status("bash", &[path_str(&script)?])?;
                }
                "bash adl/tools/test_demo_codex_ollama_semantic_fallback.sh" => {
                    let script = repo_root.join("adl/tools/test_demo_codex_ollama_semantic_fallback.sh");
                    run_finish_validation_status("bash", &[path_str(&script)?])?;
                }
                "bash adl/tools/test_demo_v089_gemma4_issue_clerk.sh" => {
                    let script = repo_root.join("adl/tools/test_demo_v089_gemma4_issue_clerk.sh");
                    run_finish_validation_status("bash", &[path_str(&script)?])?;
                }
                "cargo test --manifest-path adl/Cargo.toml --lib provider_substrate_uses_http_transport_for_ollama_with_endpoint" => {
                    run_finish_validation_status(
                        "cargo",
                        &[
                            "test",
                            "--manifest-path",
                            path_str(&manifest)?,
                            "--lib",
                            "provider_substrate_uses_http_transport_for_ollama_with_endpoint",
                        ],
                    )?;
                }
                "python3 adl/tools/validate_v0915_remote_gemma_watcher_probe.py docs/milestones/v0.91.5/review/remote_gemma_watcher" => {
                    let script =
                        repo_root.join("adl/tools/validate_v0915_remote_gemma_watcher_probe.py");
                    let review_root =
                        repo_root.join("docs/milestones/v0.91.5/review/remote_gemma_watcher");
                    run_finish_validation_status(
                        "python3",
                        &[path_str(&script)?, path_str(&review_root)?],
                    )?;
                }
                "bash adl/tools/run_owner_validation_lane.sh csdlc" => {
                    let script = repo_root.join("adl/tools/run_owner_validation_lane.sh");
                    run_finish_validation_status("bash", &[path_str(&script)?, "csdlc"])?;
                }
                "bash adl/tools/run_owner_validation_lane.sh runtime --build" => {
                    let script = repo_root.join("adl/tools/run_owner_validation_lane.sh");
                    run_finish_validation_status("bash", &[path_str(&script)?, "runtime", "--build"])?;
                }
                other if other.starts_with(
                    "bash adl/tools/run_pr_fast_test_lane.sh --changed-files ",
                ) => {
                    let changed_files = manager_backed_pr_fast_changed_files_arg(other)?;
                    let script = repo_root.join("adl/tools/run_pr_fast_test_lane.sh");
                    let result = run_finish_validation_status(
                        "bash",
                        &[path_str(&script)?, "--changed-files", &changed_files],
                    );
                    let _ = fs::remove_file(&changed_files);
                    result?;
                }
                "cargo test --manifest-path adl/Cargo.toml agent_comms --lib -- --nocapture" => {
                    run_finish_validation_status(
                        "cargo",
                        &[
                            "test",
                            "--manifest-path",
                            path_str(&manifest)?,
                            "agent_comms",
                            "--lib",
                            "--",
                            "--nocapture",
                        ],
                    )?;
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
                "cargo test --manifest-path adl/Cargo.toml --lib provider_adapter" => {
                    run_finish_validation_status(
                        "cargo",
                        &[
                            "test",
                            "--manifest-path",
                            path_str(&manifest)?,
                            "--lib",
                            "provider_adapter",
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
                "cargo test --manifest-path adl/Cargo.toml --bin adl prompt_template_ -- --nocapture" => {
                    run_finish_validation_status(
                        "cargo",
                        &[
                            "test",
                            "--manifest-path",
                            path_str(&manifest)?,
                            "--bin",
                            "adl",
                            "prompt_template_",
                            "--",
                            "--nocapture",
                        ],
                    )?;
                }
                "cargo test --manifest-path adl/Cargo.toml --bin adl structured_prompt_ -- --nocapture" => {
                    run_finish_validation_status(
                        "cargo",
                        &[
                            "test",
                            "--manifest-path",
                            path_str(&manifest)?,
                            "--bin",
                            "adl",
                            "structured_prompt_",
                            "--",
                            "--nocapture",
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
                "bash adl/tools/test_sprint_conductor_helpers.sh" => {
                    let script = repo_root.join("adl/tools/test_sprint_conductor_helpers.sh");
                    run_finish_validation_status("bash", &[path_str(&script)?])?;
                }
                other => bail!("finish: unsupported focused validation command '{other}'"),
            }
        }
        return Ok(());
    }
    bail!("finish: unsupported validation mode")
}

fn manager_backed_pr_fast_changed_files_arg(command: &str) -> Result<String> {
    let Some(changed_files) =
        command.strip_prefix("bash adl/tools/run_pr_fast_test_lane.sh --changed-files ")
    else {
        bail!("finish: unsupported focused validation command '{command}'");
    };
    let changed_files = changed_files.trim();
    if changed_files.is_empty() {
        bail!("finish: unsupported focused validation command '{command}'");
    }
    let changed_files = changed_files.trim_matches('\'');
    let file_name = Path::new(changed_files)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or_default();
    if !file_name.starts_with("finish-validation-profile-") || !file_name.ends_with(".txt") {
        bail!(
            "finish: validation manager returned unsupported changed-files manifest '{}'; expected ADL-created finish-validation-profile-*.txt",
            changed_files
        );
    }
    Ok(changed_files.to_string())
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

pub(super) fn run_finish_validation_status(program: &str, args: &[&str]) -> Result<()> {
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

pub(super) fn render_default_finish_validation(
    plan: &FinishValidationPlan,
    profile: Option<&FinishValidationProfile>,
) -> String {
    let mut lines = plan
        .commands
        .iter()
        .map(|command| format!("- {command}"))
        .collect::<Vec<_>>();
    if let Some(profile) = profile {
        lines.push(format!(
            "- Selected validation profile: `{}` (`status={}`, `pr_publication_sufficient={}`)",
            profile.selected_profile, profile.status, profile.pr_publication_sufficient
        ));
        if profile.run.is_empty() {
            lines.push("- Profile-selected run lanes: none".to_string());
        } else {
            lines.push("- Profile-selected run lanes:".to_string());
            for item in &profile.run {
                lines.push(format!(
                    "  - `{}` via `{}`",
                    item.lane_id,
                    sanitize_validation_profile_command(&item.command)
                ));
                lines.push(format!("    reason: {}", item.reason));
                if let Some(vpp) = &item.vpp_record {
                    lines.push(format!(
                        "    vpp: contract={} runtime_class={} parallel_group={} cache_equivalence_group={} failure_semantics={}",
                        vpp.contract_version,
                        vpp.expected_runtime_class,
                        vpp.parallel_group,
                        vpp.cache_equivalence_group,
                        vpp.failure_semantics
                    ));
                    lines.push(format!("    artifacts: {}", vpp.artifacts.join(", ")));
                }
            }
        }
        if profile.not_run.is_empty() {
            lines.push("- Profile-skipped proof surfaces: none".to_string());
        } else {
            lines.push("- Profile-skipped proof surfaces:".to_string());
            for item in &profile.not_run {
                lines.push(format!("  - `{}`: {}", item.surface, item.reason));
            }
        }
        if profile.deferred.is_empty() {
            lines.push("- Deferred proof: none".to_string());
        } else {
            lines.push("- Deferred proof:".to_string());
            for item in &profile.deferred {
                lines.push(format!("  - `{}`: {}", item.surface, item.reason));
            }
        }
        if profile.escalation.required {
            lines.push("- Escalation: required".to_string());
            for item in &profile.escalation.reasons {
                lines.push(format!(
                    "  - `{}` (`{}`): {}",
                    item.lane_id, item.status, item.reason
                ));
            }
        } else {
            lines.push("- Escalation: not required".to_string());
        }
    }
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

fn sanitize_validation_profile_command(command: &str) -> String {
    let mut sanitized = Vec::new();
    let mut replace_next_changed_files = false;
    for token in command.split_whitespace() {
        if replace_next_changed_files {
            sanitized.push("<changed-files>".to_string());
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
    reject_local_issue_bundle_paths_in_finish_paths(repo_root, &paths)?;
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

pub(super) fn reject_local_issue_bundle_paths_in_finish_paths(
    repo_root: &Path,
    paths: &[&str],
) -> Result<()> {
    let mut local_issue_surfaces = paths
        .iter()
        .filter_map(|path| finish_local_issue_bundle_card_display(repo_root, path))
        .collect::<Vec<_>>();
    if local_issue_surfaces.is_empty() {
        return Ok(());
    }

    local_issue_surfaces.sort_unstable();
    local_issue_surfaces.dedup();
    bail!(
        "finish: --paths includes local-only .adl task-bundle card paths: {}. Do not pass SIP/STP/SPP/SRP/SOR task-bundle files via --paths; use --output-card for the SOR truth surface and pass only tracked repo publication inputs such as docs/... or adl/src/.... Canonical .adl bundles are validated and synchronized separately and must remain local-only.",
        local_issue_surfaces.join(", ")
    )
}

fn finish_local_issue_bundle_card_display(repo_root: &Path, path: &str) -> Option<String> {
    let path_value = Path::new(path);
    let relpath = if path_value.is_absolute() {
        path_value.strip_prefix(repo_root).ok()?.to_path_buf()
    } else {
        path_value
            .components()
            .filter(|component| !matches!(component, std::path::Component::CurDir))
            .collect::<PathBuf>()
    };
    let relpath = relpath.to_string_lossy().replace('\\', "/");
    if !relpath.starts_with(".adl/") {
        return None;
    }
    issue_bundle_issue_number_from_repo_relative(&relpath)?;
    match Path::new(&relpath)
        .file_name()
        .and_then(|name| name.to_str())
    {
        Some("sip.md" | "stp.md" | "spp.md" | "srp.md" | "sor.md") => Some(relpath),
        _ => None,
    }
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

pub(super) fn non_closing_lifecycle_line(issue: u32) -> String {
    format!("Non-closing lifecycle PR: issue #{issue} remains open.")
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
    write_temp_text(prefix, "md", body)
}

fn write_temp_text(prefix: &str, extension: &str, body: &str) -> Result<PathBuf> {
    let mut path = std::env::temp_dir();
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("time")
        .as_nanos();
    path.push(format!("{prefix}-{nanos}.{extension}"));
    fs::write(&path, body)?;
    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::{
        ensure_no_staged_issue_bundle_mutations, extra_pr_body_looks_like_issue_template,
        extract_markdown_section, finish_inputs_fingerprint,
        issue_bundle_issue_number_from_repo_relative,
        reject_local_issue_bundle_paths_in_finish_paths, restage_finish_output_truth_paths,
        run_finish_validation_status,
    };
    use crate::cli::observability::test_env_lock as shared_env_lock;
    use ::adl::control_plane::{card_output_path, resolve_cards_root, IssueRef};
    use std::fs;
    use std::os::unix::fs::PermissionsExt;
    use std::path::PathBuf;
    use std::process::Command;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::sync::MutexGuard;

    static TEMP_SEQ: AtomicU64 = AtomicU64::new(0);

    fn env_lock() -> MutexGuard<'static, ()> {
        shared_env_lock()
    }

    struct ObservabilityEnvGuard;

    impl ObservabilityEnvGuard {
        fn install(log: &PathBuf) -> Self {
            unsafe {
                std::env::set_var("ADL_OBSERVABILITY_STDERR", "0");
                std::env::set_var("ADL_OBSERVABILITY_LOG", log);
                std::env::set_var("ADL_OBSERVABILITY_HEARTBEAT_MS", "25");
            }
            Self
        }
    }

    impl Drop for ObservabilityEnvGuard {
        fn drop(&mut self) {
            unsafe {
                std::env::remove_var("ADL_OBSERVABILITY_STDERR");
                std::env::remove_var("ADL_OBSERVABILITY_LOG");
                std::env::remove_var("ADL_OBSERVABILITY_HEARTBEAT_MS");
            }
        }
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

    fn read_log_until(log: &PathBuf, needle: &str) -> String {
        for _ in 0..10 {
            let contents = fs::read_to_string(log).unwrap_or_default();
            if contents.contains(needle) {
                return contents;
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
        fs::read_to_string(log).unwrap_or_default()
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

        let _env = ObservabilityEnvGuard::install(&log);

        run_finish_validation_status("bash", &[script.to_str().expect("script path")])
            .expect("validation command");

        let contents = read_log_until(&log, "result=completed");
        assert!(contents.contains("command=finish"));
        assert!(contents.contains("stage=validation_subprocess"));
        assert!(contents.contains("program=bash"));
        assert!(contents.contains("subprocess_class=shell_validation"));
        assert!(contents.contains("result=heartbeat"));
        assert!(contents.contains("result=completed"));
    }

    #[test]
    fn finish_validation_emits_failed_terminal_event_on_spawn_error() {
        let _guard = env_lock();
        let temp = temp_dir("spawn-error");
        let log = temp.join("observability.log");

        let _env = ObservabilityEnvGuard::install(&log);

        let err =
            run_finish_validation_status("definitely-not-a-real-finish-subprocess", &["--version"])
                .expect_err("spawn should fail");
        assert!(err.to_string().contains("failed to spawn"));

        let contents = read_log_until(&log, "result=failed");
        assert!(contents.contains("result=started"));
        assert!(contents.contains("result=failed"));
        assert!(contents.contains("reason_code=validation_subprocess_spawn_failed"));
        assert!(contents.contains("next_action_hint=check_subprocess_path_and_permissions"));
    }

    #[test]
    fn restage_finish_output_truth_paths_skips_local_cards_root_but_stages_tracked_files() {
        let _guard = env_lock();
        let repo = temp_dir("restage-cards-root");
        let issue_ref = IssueRef::new(4262, "v0.91.6".to_string(), "finish-cards-root".to_string())
            .expect("issue ref");
        let tracked = repo.join("README.md");
        let cards_root = resolve_cards_root(&repo, None);
        let output_card = card_output_path(&cards_root, issue_ref.issue_number());

        fs::create_dir_all(output_card.parent().expect("output parent")).expect("cards root");
        fs::create_dir_all(issue_ref.task_bundle_dir_path(&repo)).expect("task bundle dir");
        fs::write(repo.join(".gitignore"), ".adl/\n").expect("gitignore");
        fs::write(&tracked, "changed\n").expect("tracked file");
        fs::write(&output_card, "local output truth\n").expect("output card");

        assert!(Command::new("git")
            .args(["init", "-q"])
            .current_dir(&repo)
            .status()
            .expect("git init")
            .success());
        assert!(Command::new("git")
            .args(["config", "user.name", "Test User"])
            .current_dir(&repo)
            .status()
            .expect("git config name")
            .success());
        assert!(Command::new("git")
            .args(["config", "user.email", "test@example.com"])
            .current_dir(&repo)
            .status()
            .expect("git config email")
            .success());
        assert!(Command::new("git")
            .args(["add", ".gitignore", "README.md"])
            .current_dir(&repo)
            .status()
            .expect("git add initial")
            .success());
        assert!(Command::new("git")
            .args(["commit", "-q", "-m", "init"])
            .current_dir(&repo)
            .status()
            .expect("git commit")
            .success());

        fs::write(&tracked, "changed twice\n").expect("update tracked file");

        restage_finish_output_truth_paths(
            &repo,
            &repo,
            &issue_ref,
            &[tracked.clone(), output_card.clone()],
        )
        .expect("restage should skip ignored local cards root");

        let staged = Command::new("git")
            .args(["diff", "--cached", "--name-only"])
            .current_dir(&repo)
            .output()
            .expect("git diff --cached");
        assert!(staged.status.success(), "git diff --cached should succeed");
        let staged_text = String::from_utf8_lossy(&staged.stdout);
        assert!(staged_text.contains("README.md"));
        assert!(!staged_text.contains(".adl/cards/4262/output_4262.md"));
    }

    #[test]
    fn restage_finish_output_truth_paths_rejects_tracked_cards_root_paths() {
        let _guard = env_lock();
        let repo = temp_dir("restage-tracked-cards-root");
        let issue_ref = IssueRef::new(
            4263,
            "v0.91.6".to_string(),
            "tracked-cards-root".to_string(),
        )
        .expect("issue ref");
        let cards_root = resolve_cards_root(&repo, None);
        let output_card = card_output_path(&cards_root, issue_ref.issue_number());

        fs::create_dir_all(output_card.parent().expect("output parent")).expect("cards root");
        fs::create_dir_all(issue_ref.task_bundle_dir_path(&repo)).expect("task bundle dir");
        fs::write(repo.join(".gitignore"), ".adl/\n").expect("gitignore");
        fs::write(&output_card, "tracked output truth\n").expect("output card");

        assert!(Command::new("git")
            .args(["init", "-q"])
            .current_dir(&repo)
            .status()
            .expect("git init")
            .success());
        assert!(Command::new("git")
            .args(["config", "user.name", "Test User"])
            .current_dir(&repo)
            .status()
            .expect("git config name")
            .success());
        assert!(Command::new("git")
            .args(["config", "user.email", "test@example.com"])
            .current_dir(&repo)
            .status()
            .expect("git config email")
            .success());
        assert!(Command::new("git")
            .args(["add", ".gitignore"])
            .current_dir(&repo)
            .status()
            .expect("git add gitignore")
            .success());
        assert!(Command::new("git")
            .args(["add", "-f", ".adl/cards/4263/output_4263.md"])
            .current_dir(&repo)
            .status()
            .expect("git add forced output")
            .success());
        assert!(Command::new("git")
            .args(["commit", "-q", "-m", "init"])
            .current_dir(&repo)
            .status()
            .expect("git commit")
            .success());

        fs::write(&output_card, "tracked output truth updated\n").expect("update output card");

        let err = restage_finish_output_truth_paths(
            &repo,
            &repo,
            &issue_ref,
            std::slice::from_ref(&output_card),
        )
        .expect_err("tracked cards-root path should fail closed");
        assert!(err.to_string().contains("compatibility cards path"));
        assert!(err.to_string().contains(".adl/cards/4263/output_4263.md"));
    }

    #[test]
    fn restage_finish_output_truth_paths_rejects_tracked_primary_cards_root_paths() {
        let _guard = env_lock();
        let primary = temp_dir("restage-primary-tracked-cards-root-primary");
        let worktree = temp_dir("restage-primary-tracked-cards-root-worktree");
        let issue_ref = IssueRef::new(
            4264,
            "v0.91.6".to_string(),
            "primary-tracked-cards-root".to_string(),
        )
        .expect("issue ref");
        let primary_cards_root = resolve_cards_root(&primary, None);
        let primary_output = card_output_path(&primary_cards_root, issue_ref.issue_number());

        fs::create_dir_all(primary_output.parent().expect("output parent")).expect("cards root");
        fs::write(primary.join(".gitignore"), ".adl/\n").expect("gitignore");
        fs::write(&primary_output, "tracked output truth\n").expect("output card");

        assert!(Command::new("git")
            .args(["init", "-q"])
            .current_dir(&primary)
            .status()
            .expect("git init")
            .success());
        assert!(Command::new("git")
            .args(["config", "user.name", "Test User"])
            .current_dir(&primary)
            .status()
            .expect("git config name")
            .success());
        assert!(Command::new("git")
            .args(["config", "user.email", "test@example.com"])
            .current_dir(&primary)
            .status()
            .expect("git config email")
            .success());
        assert!(Command::new("git")
            .args(["add", ".gitignore"])
            .current_dir(&primary)
            .status()
            .expect("git add gitignore")
            .success());
        assert!(Command::new("git")
            .args(["add", "-f", ".adl/cards/4264/output_4264.md"])
            .current_dir(&primary)
            .status()
            .expect("git add forced output")
            .success());
        assert!(Command::new("git")
            .args(["commit", "-q", "-m", "init"])
            .current_dir(&primary)
            .status()
            .expect("git commit")
            .success());

        fs::create_dir_all(worktree.join(".git")).expect("worktree git dir placeholder");
        fs::write(&primary_output, "tracked output truth updated\n").expect("update output card");

        let err = restage_finish_output_truth_paths(
            &worktree,
            &primary,
            &issue_ref,
            std::slice::from_ref(&primary_output),
        )
        .expect_err("tracked primary cards-root path should fail closed");
        assert!(err.to_string().contains("compatibility cards path"));
        assert!(err.to_string().contains(".adl/cards/4264/output_4264.md"));
    }

    #[test]
    fn reject_local_issue_bundle_paths_in_finish_paths_flags_local_cards() {
        let _guard = env_lock();
        let repo = temp_dir("reject-local-issue-bundle-paths");
        let issue_ref = IssueRef::new(
            4265,
            "v0.91.6".to_string(),
            "reject-local-paths".to_string(),
        )
        .expect("issue ref");
        let local_sip = issue_ref.task_bundle_input_path(&repo);
        let local_sor = issue_ref.task_bundle_output_path(&repo);
        fs::create_dir_all(local_sip.parent().expect("sip parent")).expect("bundle dir");
        fs::write(&local_sip, "sip\n").expect("write sip");
        fs::write(&local_sor, "sor\n").expect("write sor");

        let err = reject_local_issue_bundle_paths_in_finish_paths(
            &repo,
            &[
                "docs/notes.md",
                local_sip.to_str().expect("sip path"),
                local_sor.to_str().expect("sor path"),
            ],
        )
        .expect_err("local issue bundle paths should fail closed");

        assert!(err
            .to_string()
            .contains("local-only .adl task-bundle card paths"));
        assert!(err
            .to_string()
            .contains(".adl/v0.91.6/tasks/issue-4265__reject-local-paths/sip.md"));
        assert!(err
            .to_string()
            .contains(".adl/v0.91.6/tasks/issue-4265__reject-local-paths/sor.md"));
    }

    #[test]
    fn ensure_no_staged_issue_bundle_mutations_rejects_foreign_issue_paths() {
        let _guard = env_lock();
        let repo = temp_dir("reject-foreign-issue-bundle-stage");
        let active_issue =
            IssueRef::new(4266, "v0.91.6".to_string(), "active".to_string()).expect("active issue");
        let foreign_issue = IssueRef::new(4267, "v0.91.6".to_string(), "foreign".to_string())
            .expect("foreign issue");
        let foreign_sor = foreign_issue.task_bundle_output_path(&repo);

        fs::create_dir_all(foreign_sor.parent().expect("output parent")).expect("bundle dir");
        fs::write(repo.join(".gitignore"), ".adl/\n").expect("gitignore");
        fs::write(&foreign_sor, "foreign sor\n").expect("foreign sor");

        assert!(Command::new("git")
            .args(["init", "-q"])
            .current_dir(&repo)
            .status()
            .expect("git init")
            .success());
        assert!(Command::new("git")
            .args(["config", "user.name", "Test User"])
            .current_dir(&repo)
            .status()
            .expect("git config name")
            .success());
        assert!(Command::new("git")
            .args(["config", "user.email", "test@example.com"])
            .current_dir(&repo)
            .status()
            .expect("git config email")
            .success());
        assert!(Command::new("git")
            .args(["add", ".gitignore"])
            .current_dir(&repo)
            .status()
            .expect("git add gitignore")
            .success());
        assert!(Command::new("git")
            .args(["commit", "-q", "-m", "init"])
            .current_dir(&repo)
            .status()
            .expect("git commit")
            .success());
        assert!(Command::new("git")
            .args(["add", "-f", foreign_sor.to_str().expect("foreign sor path")])
            .current_dir(&repo)
            .status()
            .expect("git add foreign sor")
            .success());

        let err = ensure_no_staged_issue_bundle_mutations(&repo, &active_issue)
            .expect_err("foreign staged bundle paths should fail");
        assert!(err
            .to_string()
            .contains("staged .adl task-bundle changes for non-active issues detected"));
        assert!(err
            .to_string()
            .contains(".adl/v0.91.6/tasks/issue-4267__foreign/sor.md"));
    }

    #[test]
    fn finish_support_helper_functions_cover_markdown_and_fingerprint_surfaces() {
        let _guard = env_lock();
        let repo = temp_dir("finish-support-helper-surfaces");
        let markdown = repo.join("output.md");
        fs::write(
            &markdown,
            "## Summary\nsummary\n\n## Validation\n- ok\n\n## Tail\nignored\n",
        )
        .expect("write markdown");

        assert_eq!(
            extract_markdown_section(&markdown, "Summary").expect("summary section"),
            "summary"
        );
        assert_eq!(
            extract_markdown_section(&markdown, "Validation").expect("validation section"),
            "- ok"
        );
        assert_eq!(
            issue_bundle_issue_number_from_repo_relative(
                ".adl/v0.91.6/tasks/issue-4268__helper/sor.md"
            ),
            Some(4268)
        );
        assert_eq!(
            issue_bundle_issue_number_from_repo_relative("docs/milestones/v0.91.6/README.md"),
            None
        );
        assert!(extra_pr_body_looks_like_issue_template(
            "issue_card_schema: adl.issue.v1"
        ));
        assert!(extra_pr_body_looks_like_issue_template(
            "## Goal\nstuff\n---\nmore"
        ));
        assert!(!extra_pr_body_looks_like_issue_template(
            "regular reviewer notes"
        ));
        assert_eq!(
            finish_inputs_fingerprint(
                "[v0.91.6][tools] Example",
                "adl/src/lib.rs,docs/notes.md",
                ".adl/v0.91.6/tasks/issue-4268__helper/sip.md",
                ".adl/v0.91.6/tasks/issue-4268__helper/sor.md",
            ),
            "v0-91-6-tools-example-adl-src-lib-rs-docs-notes-md-adl-v0-91-6-tasks-issue-4268-helper-sip-md-adl-v0-91-6-tasks-issue-4268-helper-sor-md"
        );
    }
}
