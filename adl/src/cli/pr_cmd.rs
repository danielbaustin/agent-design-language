use anyhow::{anyhow, bail, Context, Result};
use serde::Serialize;
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
#[cfg(test)]
use std::process::Command;
#[cfg(test)]
use std::sync::{Mutex, OnceLock};

#[cfg(test)]
use super::pr_cmd_args::parse_finish_args;
use super::pr_cmd_args::{
    parse_closeout_args, parse_create_args, parse_doctor_args, parse_init_args, parse_issue_args,
    parse_preflight_args, parse_ready_args, parse_repair_issue_body_args, parse_start_args,
    parse_validation_args, DoctorArgs, DoctorMode, IssueArgs,
};
use super::pr_cmd_cards::{
    branch_indicates_unbound_state, ensure_bootstrap_cards, ensure_local_issue_prompt_copy,
    ensure_pre_run_bootstrap_cards, ensure_source_issue_prompt, ensure_symlink,
    ensure_task_bundle_stp, ensure_worktree_task_bundle_materialized, field_line_value,
    mirror_docs_templates_into_worktree, path_relative_to_repo,
    sync_root_task_bundle_into_worktree, validate_bootstrap_stp, validate_initialized_cards,
    validate_issue_body_for_create, validate_ready_cards, write_source_issue_prompt,
};
#[cfg(test)]
use super::pr_cmd_prompt::load_issue_prompt;
use super::pr_cmd_prompt::{
    ensure_no_duplicate_issue_identities, infer_required_outcome_type, infer_workflow_queue,
    normalize_issue_title_for_version, normalize_labels_csv, parse_issue_number_from_url,
    render_generated_issue_body, resolve_issue_body, resolve_issue_prompt_path,
    resolve_issue_prompt_workflow_queue, resolve_issue_scope_and_slug_from_local_state,
    validate_issue_prompt_exists, version_from_labels_csv, version_from_title,
    WorkflowQueueResolution,
};
use super::pr_cmd_validate::{
    bootstrap_stub_reason, validate_authored_prompt_surface, PromptSurfaceKind,
};
use ::adl::control_plane::{
    card_output_path, resolve_cards_root, resolve_primary_checkout_root, sanitize_slug, IssueRef,
};

mod doctor;
mod finish_support;
mod git_support;
mod github;
pub(crate) mod github_client;
mod lifecycle;

pub(crate) use self::github::gh_issue_is_closed_completed;
pub(crate) use self::github::{gh_issue_body, gh_issue_label_names};

#[cfg(test)]
type CreatePostBootstrapHook = fn(&Path, &IssueRef) -> Result<()>;

#[cfg(test)]
static CREATE_POST_BOOTSTRAP_HOOK: OnceLock<Mutex<Option<CreatePostBootstrapHook>>> =
    OnceLock::new();

#[cfg(test)]
use self::finish_support::write_temp_markdown;
#[cfg(test)]
use self::finish_support::{
    ensure_issue_surfaces_are_local_only, ensure_output_card_is_started,
    extra_pr_body_looks_like_issue_template, extract_markdown_section, finish_changed_paths,
    finish_inputs_fingerprint, issue_bundle_issue_number_from_repo_relative,
    render_default_finish_validation, render_pr_body, run_finish_validation_rust,
    select_finish_validation_plan, stage_selected_paths_rust, staged_diff_is_empty,
    staged_gitignore_change_present, tracked_issue_surface_paths,
};
use self::finish_support::{ensure_nonempty_file_path, validate_completed_sor};
#[cfg(test)]
use self::git_support::commits_ahead_of_origin_main;
#[cfg(test)]
use self::git_support::has_uncommitted_changes;
#[cfg(test)]
use self::git_support::{branch_checked_out_worktree_path, infer_repo_from_remote};
use self::git_support::{
    current_branch, default_repo, ensure_git_metadata_writable, ensure_local_branch_exists,
    ensure_worktree_for_branch, fetch_origin_main_with_fallback,
    has_uncommitted_or_untracked_changes, issue_create_repo, path_str, primary_checkout_root,
    repo_root, run_capture, run_status, tracked_changes_status,
};
use self::github::{
    ensure_issue_metadata_parity, format_open_pr_wave, gh_issue_comment, gh_issue_create,
    gh_issue_edit_body, gh_issue_edit_title, gh_issue_set_labels, gh_issue_title, issue_version,
    unresolved_milestone_pr_wave, IssueRecord,
};

const DEFAULT_VERSION: &str = "v0.86";
const DEFAULT_NEW_LABELS: &str = "track:roadmap,type:task,area:tools";

fn resolve_version_for_create(
    explicit_version: Option<String>,
    labels_csv: Option<&str>,
    raw_title: &str,
) -> Result<String> {
    if let Some(version) = explicit_version {
        return Ok(version);
    }

    version_from_labels_csv(labels_csv.unwrap_or_default())
        .or_else(|| version_from_title(raw_title))
        .ok_or_else(|| {
            anyhow!(
                "create: could not infer version from title or labels; pass --version or include a version:vX.Y label / [vX.Y] title prefix"
            )
        })
}

fn fetched_issue_labels_csv(repo: &str, issue: u32) -> Result<String> {
    Ok(gh_issue_label_names(issue, repo)?
        .into_iter()
        .map(|label| label.trim().to_string())
        .filter(|label| !label.is_empty())
        .collect::<Vec<_>>()
        .join(","))
}

fn resolve_version_for_existing_issue(
    repo_root: &Path,
    repo: &str,
    issue: u32,
    explicit_version: Option<String>,
    no_fetch_issue: bool,
    command: &str,
) -> Result<String> {
    if let Some(version) = explicit_version {
        return Ok(version);
    }

    if no_fetch_issue {
        if let Some((version, _slug)) =
            resolve_issue_scope_and_slug_from_local_state(repo_root, issue)?
        {
            return Ok(version);
        }
        bail!(
            "{command}: --version is required when --no-fetch-issue is set and no canonical local bundle exists to infer the milestone band"
        );
    }

    issue_version(issue, repo)?.ok_or_else(|| {
        anyhow!(
            "{command}: could not infer version for issue #{issue}; pass --version or add a version:vX.Y label / [vX.Y] title prefix"
        )
    })
}

fn resolve_local_issue_identity(repo_root: &Path, issue: u32) -> Result<Option<(String, String)>> {
    resolve_issue_scope_and_slug_from_local_state(repo_root, issue)
}

pub(crate) fn real_pr(args: &[String]) -> Result<()> {
    let Some(subcommand) = args.first().map(|s| s.as_str()) else {
        bail!(
            "pr requires a subcommand: create | init | repair-issue-body | start | doctor | ready | preflight | finish | validation | issue | closeout"
        );
    };

    match subcommand {
        "create" => real_pr_create(&args[1..]),
        "init" => real_pr_init(&args[1..]),
        "repair-issue-body" => real_pr_repair_issue_body(&args[1..]),
        "start" => real_pr_start(&args[1..]),
        "doctor" => real_pr_doctor(&args[1..]),
        "ready" => real_pr_ready(&args[1..]),
        "preflight" => real_pr_preflight(&args[1..]),
        "finish" => finish_support::real_pr_finish(&args[1..]),
        "validation" => real_pr_validation(&args[1..]),
        "issue" => real_pr_issue(&args[1..]),
        "closeout" => real_pr_closeout(&args[1..]),
        other => bail!("unknown pr subcommand: {other}"),
    }
}

fn real_pr_validation(args: &[String]) -> Result<()> {
    let parsed = parse_validation_args(args)?;
    let repo_root = repo_root()?;
    let repo = parsed
        .repo
        .clone()
        .or_else(|| repo_from_pr_ref(&parsed.pr_ref))
        .unwrap_or(default_repo(&repo_root)?);
    let report = if parsed.watch {
        github::wait_for_pr_validation_report(&repo, &parsed.pr_ref)?
    } else {
        github::pr_validation_report(&repo, &parsed.pr_ref)?
    };
    if parsed.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&report)
                .context("validation: failed to serialize validation report")?
        );
    } else {
        println!(
            "PR #{} validation: {} (checks: {}, failed: {}, pending: {})",
            report.pr_number,
            report.disposition,
            report.checks.len(),
            report.failed_checks.len(),
            report.pending_checks.len()
        );
        for check in report
            .failed_checks
            .iter()
            .chain(report.pending_checks.iter())
        {
            println!(
                "- {} status={} conclusion={} job_run_id={}",
                check.name, check.status, check.conclusion, check.job_run_id
            );
        }
    }
    if validation_disposition_blocks_shell_success(&report.disposition) {
        bail!(
            "validation: PR #{} is {}",
            report.pr_number,
            report.disposition
        );
    }
    Ok(())
}

fn validation_disposition_blocks_shell_success(disposition: &str) -> bool {
    matches!(
        disposition,
        "pending" | "failed" | "cancelled" | "timed_out"
    )
}

fn real_pr_issue(args: &[String]) -> Result<()> {
    let parsed = parse_issue_args(args)?;
    let repo_root = repo_root()?;
    match parsed {
        IssueArgs::List(parsed) => {
            let repo = parsed.repo.unwrap_or(default_repo(&repo_root)?);
            let issues = github::gh_issue_list(&repo, parsed.state, parsed.limit)?;
            if parsed.json {
                print_json(&issues)?;
            } else {
                print_issue_rows(&issues);
            }
        }
        IssueArgs::Search(parsed) => {
            let repo = parsed.repo.unwrap_or(default_repo(&repo_root)?);
            let issues = github::gh_issue_search(&repo, &parsed.query, parsed.state, parsed.limit)?;
            if parsed.json {
                print_json(&issues)?;
            } else {
                print_issue_rows(&issues);
            }
        }
        IssueArgs::View(parsed) => {
            let repo = parsed
                .repo
                .or_else(|| repo_from_issue_ref(&parsed.issue_ref))
                .unwrap_or(default_repo(&repo_root)?);
            let issue = parse_issue_ref_number("issue view", &parsed.issue_ref)?;
            let record = github::gh_issue_view(&repo, issue)?;
            if parsed.json {
                print_json(&record)?;
            } else {
                print_issue_view(&record);
            }
        }
        IssueArgs::Create(parsed) => {
            let repo = parsed.repo.unwrap_or(default_repo(&repo_root)?);
            let body = resolve_issue_body(parsed.body, parsed.body_file.as_deref())?;
            let labels_csv = parsed.labels.join(",");
            let url = gh_issue_create(&repo, &parsed.title, &body, &labels_csv)?;
            if parsed.json {
                print_json(&IssueMutationResult {
                    status: "created",
                    issue: parse_issue_number_from_url(&url).ok(),
                    url: Some(url),
                })?;
            } else {
                println!("{url}");
            }
        }
        IssueArgs::Comment(parsed) => {
            let repo = parsed
                .repo
                .or_else(|| repo_from_issue_ref(&parsed.issue_ref))
                .unwrap_or(default_repo(&repo_root)?);
            let issue = parse_issue_ref_number("issue comment", &parsed.issue_ref)?;
            let body = resolve_issue_body(parsed.body, parsed.body_file.as_deref())?;
            gh_issue_comment(&repo, issue, &body)?;
            if parsed.json {
                print_json(&IssueMutationResult {
                    status: "commented",
                    issue: Some(issue),
                    url: None,
                })?;
            }
        }
        IssueArgs::Edit(parsed) => {
            let repo = parsed
                .repo
                .or_else(|| repo_from_issue_ref(&parsed.issue_ref))
                .unwrap_or(default_repo(&repo_root)?);
            let issue = parse_issue_ref_number("issue edit", &parsed.issue_ref)?;
            if let Some(title) = parsed.title {
                gh_issue_edit_title(&repo, issue, &title)?;
            }
            if parsed.body.is_some() || parsed.body_file.is_some() {
                let body = resolve_issue_body(parsed.body, parsed.body_file.as_deref())?;
                gh_issue_edit_body(&repo, issue, &body)?;
            }
            if !parsed.labels.is_empty() {
                gh_issue_set_labels(&repo, issue, &parsed.labels)?;
            }
            if parsed.json {
                print_json(&IssueMutationResult {
                    status: "edited",
                    issue: Some(issue),
                    url: None,
                })?;
            }
        }
    }
    Ok(())
}

#[derive(Serialize)]
struct IssueMutationResult {
    status: &'static str,
    issue: Option<u32>,
    url: Option<String>,
}

fn parse_issue_ref_number(command: &str, issue_ref: &str) -> Result<u32> {
    if issue_ref.starts_with("http://") || issue_ref.starts_with("https://") {
        parse_issue_number_from_url(issue_ref)
    } else {
        issue_ref
            .parse::<u32>()
            .with_context(|| format!("{command}: invalid issue number: {issue_ref}"))
    }
}

fn repo_from_pr_ref(pr_ref: &str) -> Option<String> {
    let trimmed = pr_ref.trim();
    let marker = "github.com/";
    let (_, tail) = trimmed.split_once(marker)?;
    let path = tail.split(['?', '#']).next()?;
    let mut parts = path.split('/');
    let owner = parts.next()?.trim();
    let repo = parts.next()?.trim();
    let pull_marker = parts.next()?.trim();
    if owner.is_empty() || repo.is_empty() || pull_marker != "pull" {
        return None;
    }
    Some(format!("{owner}/{repo}"))
}

fn repo_from_issue_ref(issue_ref: &str) -> Option<String> {
    let trimmed = issue_ref.trim();
    let marker = "github.com/";
    let (_, tail) = trimmed.split_once(marker)?;
    let path = tail.split(['?', '#']).next()?;
    let mut parts = path.split('/');
    let owner = parts.next()?.trim();
    let repo = parts.next()?.trim();
    let issue_marker = parts.next()?.trim();
    if owner.is_empty() || repo.is_empty() || issue_marker != "issues" {
        return None;
    }
    let issue_number = parts.next()?.trim();
    if issue_number.parse::<u32>().is_err() {
        return None;
    }
    Some(format!("{owner}/{repo}"))
}

fn print_issue_rows(issues: &[IssueRecord]) {
    let rendered = format_issue_rows(issues);
    if !rendered.is_empty() {
        println!("{rendered}");
    }
}

fn format_issue_rows(issues: &[IssueRecord]) -> String {
    issues
        .iter()
        .map(|issue| {
            let milestone = issue
                .milestone
                .as_deref()
                .map(|value| format!(" milestone={value}"))
                .unwrap_or_default();
            format!(
                "#{} {} {}{} {}",
                issue.number,
                issue.state.to_uppercase(),
                issue.title,
                milestone,
                issue.url
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn print_issue_view(issue: &IssueRecord) {
    println!("{}", format_issue_view(issue));
}

fn format_issue_view(issue: &IssueRecord) -> String {
    let mut lines = vec![
        format!("#{} {}", issue.number, issue.title),
        format!("state: {}", issue.state),
        format!("url: {}", issue.url),
    ];
    if let Some(closed_at) = issue.closed_at.as_deref() {
        lines.push(format!("closed_at: {closed_at}"));
    }
    if let Some(milestone) = issue.milestone.as_deref() {
        lines.push(format!("milestone: {milestone}"));
    }
    if issue.labels.is_empty() {
        lines.push("labels:".to_string());
    } else {
        lines.push(format!("labels: {}", issue.labels.join(", ")));
    }
    if let Some(body) = issue.body.as_deref() {
        lines.push(String::new());
        lines.push(body.to_string());
    }
    lines.join("\n")
}

fn real_pr_repair_issue_body(args: &[String]) -> Result<()> {
    let parsed = parse_repair_issue_body_args(args)?;
    let repo_root = repo_root()?;
    let repo = default_repo(&repo_root)?;
    let local_identity = resolve_local_issue_identity(&repo_root, parsed.issue)?;
    if let Some((local_version, local_slug)) = local_identity.as_ref() {
        if let Some(requested_slug) = parsed.slug.as_deref() {
            let requested_slug = sanitize_slug(requested_slug);
            if !requested_slug.is_empty() && requested_slug != *local_slug {
                bail!(
                    "repair-issue-body: slug/local identity change is not supported for issue #{}; current canonical local identity is {}:{} and requested slug is '{}'. Keep the canonical slug or migrate the local prompt/task bundle manually before rerunning.",
                    parsed.issue,
                    local_version,
                    local_slug,
                    requested_slug
                );
            }
        }
        if let Some(requested_version) = parsed.version.as_deref() {
            if requested_version != local_version {
                bail!(
                    "repair-issue-body: version/local identity change is not supported for issue #{}; current canonical local identity is {}:{} and requested version is '{}'. Keep the canonical version or migrate the local prompt/task bundle manually before rerunning.",
                    parsed.issue,
                    local_version,
                    local_slug,
                    requested_version
                );
            }
        }
    }
    let raw_title = if let Some(title) = parsed.title_arg.clone() {
        title
    } else {
        gh_issue_title(parsed.issue, &repo)?
            .ok_or_else(|| anyhow!("repair-issue-body: could not fetch issue #{} title; pass --title or check GitHub token/repo configuration", parsed.issue))?
    };
    let version = if let Some(version) = parsed.version.clone() {
        version
    } else if let Some((local_version, _)) = local_identity.as_ref() {
        local_version.clone()
    } else {
        resolve_version_for_existing_issue(
            &repo_root,
            &repo,
            parsed.issue,
            None,
            false,
            "repair-issue-body",
        )?
    };
    let title = normalize_issue_title_for_version(&raw_title, &version);
    let slug = parsed
        .slug
        .clone()
        .or_else(|| local_identity.as_ref().map(|(_, slug)| slug.clone()))
        .unwrap_or_else(|| sanitize_slug(&title));
    let slug = sanitize_slug(&slug);
    if slug.is_empty() {
        bail!("repair-issue-body: slug is empty after sanitization");
    }
    let fetched_labels = if parsed.labels.is_some() {
        String::new()
    } else {
        fetched_issue_labels_csv(&repo, parsed.issue)?
    };
    let label_source = parsed.labels.as_deref().unwrap_or_else(|| {
        if fetched_labels.trim().is_empty() {
            DEFAULT_NEW_LABELS
        } else {
            &fetched_labels
        }
    });
    let normalized_labels = normalize_labels_csv(label_source, &version);
    let body_requested = parsed.body.is_some() || parsed.body_file.is_some();
    let body = if body_requested {
        resolve_issue_body(parsed.body.clone(), parsed.body_file.as_deref())?
    } else {
        gh_issue_body(parsed.issue, &repo)?.ok_or_else(|| {
            anyhow!(
                "repair-issue-body: metadata-only repair requires the current GitHub issue body; pass --body or --body-file if the issue body is empty or cannot be fetched"
            )
        })?
    };
    validate_issue_body_for_create(&repo_root, &title, &normalized_labels, &slug, &body)?;

    let issue_ref = IssueRef::new(parsed.issue, version.clone(), slug.clone())?;
    ensure_no_duplicate_issue_identities(&repo_root, &issue_ref)?;
    let source_path = issue_ref.issue_prompt_path(&repo_root);
    if source_path.is_file() && !parsed.force {
        let current = fs::read_to_string(&source_path)
            .with_context(|| format!("failed to read {}", source_path.display()))?;
        if bootstrap_stub_reason(&current, PromptSurfaceKind::IssuePrompt).is_none() {
            bail!(
                "repair-issue-body: refusing to overwrite authored source prompt without --force: {}",
                source_path.display()
            );
        }
    }

    let issue_url = format!("https://github.com/{repo}/issues/{}", parsed.issue);
    let current_title = gh_issue_title(parsed.issue, &repo)?.unwrap_or_default();
    if title != current_title {
        gh_issue_edit_title(&repo, parsed.issue, &title)?;
    }
    let current_labels = gh_issue_label_names(parsed.issue, &repo)?
        .into_iter()
        .map(|label| label.trim().to_string())
        .filter(|label| !label.is_empty())
        .collect::<BTreeSet<_>>();
    let expected_labels = normalized_labels
        .split(',')
        .map(str::trim)
        .filter(|label| !label.is_empty())
        .map(str::to_string)
        .collect::<BTreeSet<_>>();
    if current_labels != expected_labels {
        let ordered_labels = normalized_labels
            .split(',')
            .map(str::trim)
            .filter(|label| !label.is_empty())
            .map(str::to_string)
            .collect::<Vec<_>>();
        gh_issue_set_labels(&repo, parsed.issue, &ordered_labels)?;
    }
    if body_requested {
        gh_issue_edit_body(&repo, parsed.issue, &body)?;
    }
    let source_path = write_source_issue_prompt(
        &repo_root,
        &issue_ref,
        &title,
        &normalized_labels,
        &issue_url,
        &body,
    )?;
    validate_bootstrap_stp(&repo_root, &source_path)?;
    validate_authored_prompt_surface(
        "repair-issue-body",
        &source_path,
        PromptSurfaceKind::IssuePrompt,
    )?;

    let bundle_dir = issue_ref.task_bundle_dir_path(&repo_root);
    if bundle_dir.is_dir() {
        fs::remove_dir_all(&bundle_dir)
            .with_context(|| format!("failed to remove stale bundle {}", bundle_dir.display()))?;
    }
    let (stp_path, bundle_input, bundle_output, bundle_dir) =
        bootstrap_root_task_bundle(&repo_root, &issue_ref, &title, &source_path)?;

    println!("• Repaired issue metadata/source prompt:");
    println!("  ISSUE     #{}", parsed.issue);
    println!("  VERSION   {version}");
    println!("  SLUG      {slug}");
    println!(
        "  SOURCE    {}",
        path_relative_to_repo(&repo_root, &source_path)
    );
    println!(
        "  STP       {}",
        path_relative_to_repo(&repo_root, &stp_path)
    );
    println!(
        "  SIP       {}",
        path_relative_to_repo(&repo_root, &bundle_input)
    );
    println!(
        "  SOR       {}",
        path_relative_to_repo(&repo_root, &bundle_output)
    );
    println!(
        "  BUNDLE    {}",
        path_relative_to_repo(&repo_root, &bundle_dir)
    );
    println!("  STATE     ISSUE_BODY_AND_BUNDLE_REPAIRED");
    println!("• Done.");
    Ok(())
}

fn real_pr_create(args: &[String]) -> Result<()> {
    let parsed = parse_create_args(args)?;
    let repo_root = repo_root()?;
    let repo = issue_create_repo(&repo_root)?;

    let raw_title = parsed.title_arg.clone().unwrap_or_default();
    let mut slug = parsed.slug.clone().unwrap_or_default();
    if slug.is_empty() {
        slug = sanitize_slug(&raw_title);
    } else {
        slug = sanitize_slug(&slug);
    }
    if slug.is_empty() {
        bail!("create: slug is empty after sanitization");
    }

    let version =
        resolve_version_for_create(parsed.version.clone(), parsed.labels.as_deref(), &raw_title)?;
    let title = normalize_issue_title_for_version(&raw_title, &version);
    if parsed.slug.as_deref().unwrap_or_default().trim().is_empty() {
        slug = sanitize_slug(&title);
        if slug.is_empty() {
            bail!("create: title produced empty slug after normalization");
        }
    }

    let normalized_labels = normalize_labels_csv(
        parsed.labels.as_deref().unwrap_or(DEFAULT_NEW_LABELS),
        &version,
    );
    let body = resolve_issue_body(parsed.body.clone(), parsed.body_file.as_deref())?;
    let create_body = if body.trim().is_empty() {
        render_generated_issue_body(
            &title,
            infer_required_outcome_type_for_create(&normalized_labels, &title),
            None,
        )
    } else {
        body.clone()
    };
    validate_issue_body_for_create(&repo_root, &title, &normalized_labels, &slug, &create_body)?;
    let issue_url = gh_issue_create(&repo, &title, &create_body, &normalized_labels)?;
    let issue = parse_issue_number_from_url(&issue_url)?;
    ensure_issue_metadata_parity(&repo, issue, &title, &normalized_labels)?;
    let issue_ref = IssueRef::new(issue, version.clone(), slug.clone())?;
    ensure_no_duplicate_issue_identities(&repo_root, &issue_ref)?;
    let final_body = if body.trim().is_empty() {
        render_generated_issue_body(
            &title,
            infer_required_outcome_type_for_create(&normalized_labels, &title),
            Some(&issue_url),
        )
    } else {
        body
    };
    let source_path = write_source_issue_prompt(
        &repo_root,
        &issue_ref,
        &title,
        &normalized_labels,
        &issue_url,
        &final_body,
    )?;
    validate_bootstrap_stp(&repo_root, &source_path)?;
    if create_body != final_body {
        gh_issue_edit_body(&repo, issue, &final_body)?;
    }
    let (stp_path, bundle_input, bundle_output, bundle_dir) =
        bootstrap_root_task_bundle(&repo_root, &issue_ref, &title, &source_path)?;
    run_create_post_bootstrap_test_hook(&repo_root, &issue_ref)?;
    let ready = doctor::run_doctor_ready(
        &repo_root,
        &repo,
        &issue_ref,
        &issue_ref.branch_name("codex"),
    )
    .with_context(|| {
        format!(
            "create: issue #{} failed immediate ready-state validation",
            issue_ref.issue_number()
        )
    })?;

    println!("• Created:");
    println!("  ISSUE_URL  {issue_url}");
    println!("  ISSUE_NUM  {issue}");
    println!("  VERSION    {version}");
    println!("  SLUG       {slug}");
    println!(
        "  SOURCE     {}",
        path_relative_to_repo(&repo_root, &source_path)
    );
    println!(
        "  STP        {}",
        path_relative_to_repo(&repo_root, &stp_path)
    );
    println!(
        "  SPP        {}",
        path_relative_to_repo(&repo_root, &issue_ref.task_bundle_plan_path(&repo_root))
    );
    println!(
        "  SRP        {}",
        path_relative_to_repo(
            &repo_root,
            &issue_ref.task_bundle_review_policy_path(&repo_root),
        )
    );
    println!(
        "  SIP        {}",
        path_relative_to_repo(&repo_root, &bundle_input)
    );
    println!(
        "  SOR        {}",
        path_relative_to_repo(&repo_root, &bundle_output)
    );
    println!(
        "  BUNDLE     {}",
        path_relative_to_repo(&repo_root, &bundle_dir)
    );
    println!(
        "  READY      {}",
        bootstrap_ready_status_label(ready.status)
    );
    println!("  LIFECYCLE  {}", ready.lifecycle_state);
    println!("  NEXT       qualitative SIP/STP/SPP/SRP design-time review, then adl/tools/pr.sh run {issue} --slug {slug} --version {version}");
    println!("  STATE      ISSUE_AND_BUNDLE_READY");
    println!("• Done.");
    Ok(())
}

fn real_pr_start(args: &[String]) -> Result<()> {
    let parsed = parse_start_args(args)?;
    let repo_root = repo_root()?;
    let primary_root = primary_checkout_root()?;
    let repo = default_repo(&repo_root)?;
    let local_identity = resolve_local_issue_identity(&repo_root, parsed.issue)?;

    if std::env::var("ADL_PR_SUPPRESS_START_COMPAT_NOTE").as_deref() != Ok("1") {
        eprintln!(
            "• Deprecated compatibility path: prefer `adl/tools/pr.sh run {}` for execution-context binding.",
            parsed.issue
        );
    }

    let mut title = parsed.title_arg.clone().unwrap_or_default();
    let mut slug = parsed.slug.clone().unwrap_or_default();
    let mut slug_from_local_identity = false;
    if title.is_empty() && !parsed.no_fetch_issue {
        eprintln!("• Fetching issue title via gh…");
        title = gh_issue_title(parsed.issue, &repo)?.unwrap_or_default();
    }
    if slug.is_empty() {
        if let Some((_, local_slug)) = local_identity.as_ref() {
            slug = local_slug.clone();
            slug_from_local_identity = true;
        } else if !title.is_empty() {
            slug = sanitize_slug(&title);
            if slug.is_empty() {
                bail!("start: --title produced empty slug after sanitization");
            }
        } else if parsed.no_fetch_issue {
            bail!("start: --slug is required when --no-fetch-issue is set");
        }
        if slug.is_empty() && title.is_empty() {
            bail!(
                "Could not fetch issue #{} title. Pass --slug or check GitHub token/repo configuration.",
                parsed.issue
            );
        }
        if slug.is_empty() {
            slug = sanitize_slug(&title);
        }
    }
    if !same_checkout_root(&repo_root, &primary_root)? {
        bail!(
            "start: issue-mode run must be invoked from the primary checkout, not from an existing issue worktree. Current checkout: '{}'. Primary checkout: '{}'. Remediation: continue working in the current issue worktree if it already matches your target issue, or rerun `adl/tools/pr.sh run {}` from the primary checkout.",
            repo_root.display(),
            primary_root.display(),
            parsed.issue
        );
    }
    if title.is_empty() {
        title = slug.clone();
    }

    let version = if let Some(version) = parsed.version.clone() {
        version
    } else if let Some((local_version, _)) = local_identity.as_ref() {
        local_version.clone()
    } else {
        resolve_version_for_existing_issue(
            &repo_root,
            &repo,
            parsed.issue,
            None,
            parsed.no_fetch_issue,
            "start",
        )?
    };
    title = normalize_issue_title_for_version(&title, &version);
    if parsed.slug.as_deref().unwrap_or_default().trim().is_empty() && !slug_from_local_identity {
        slug = sanitize_slug(&title);
        if slug.is_empty() {
            bail!("start: title produced empty slug after normalization");
        }
    }
    ensure_issue_run_primary_checkout_safe(&repo_root)?;
    let fetched_labels = if parsed.no_fetch_issue {
        String::new()
    } else {
        fetched_issue_labels_csv(&repo, parsed.issue)?
    };
    let normalized_labels = normalize_labels_csv(
        if fetched_labels.trim().is_empty() {
            DEFAULT_NEW_LABELS
        } else {
            &fetched_labels
        },
        &version,
    );

    let issue_ref = IssueRef::new(parsed.issue, version.clone(), slug.clone())?;
    if !parsed.no_fetch_issue {
        ensure_issue_metadata_parity(&repo, parsed.issue, &title, &normalized_labels)?;
    }
    ensure_no_duplicate_issue_identities(&repo_root, &issue_ref)?;
    let branch = issue_ref.branch_name(&parsed.prefix);
    let target_queue = if issue_ref.issue_prompt_path(&repo_root).is_file() {
        resolve_issue_prompt_workflow_queue(&issue_ref.issue_prompt_path(&repo_root))?
    } else {
        WorkflowQueueResolution {
            queue: infer_workflow_queue(&title, &normalized_labels, None)
                .ok_or_else(|| {
                    anyhow!(
                        "start: missing or invalid workflow queue for issue #{}; add a canonical queue such as wp/tools/runtime/demo/docs/review/release before execution",
                        parsed.issue
                    )
                })?
                .to_string(),
            source: "inferred",
        }
    };
    let unresolved =
        unresolved_milestone_pr_wave(&repo, &version, &target_queue.queue, Some(&branch))?;
    let sprint_wave_override = std::env::var("ADL_SPRINT_ALLOW_OPEN_PR_WAVE")
        .map(|value| {
            matches!(
                value.trim().to_ascii_lowercase().as_str(),
                "1" | "true" | "yes" | "on"
            )
        })
        .unwrap_or(false);
    if !(parsed.allow_open_pr_wave || sprint_wave_override || unresolved.is_empty()) {
        bail!(
            "start: unresolved open PR queue detected for {} [{}:{}]. Resolve or merge these PRs first, or rerun with --allow-open-pr-wave if you are deliberately overriding the guard:\n{}",
            version,
            target_queue.queue,
            target_queue.source,
            format_open_pr_wave(&unresolved)
        );
    }
    let managed_root = std::env::var_os("ADL_WORKTREE_ROOT").map(PathBuf::from);
    let worktree_path = issue_ref.default_worktree_path(&repo_root, managed_root.as_deref());

    eprintln!("• Target branch: {branch}");
    eprintln!("• Target worktree: {}", worktree_path.display());

    let source_path = ensure_source_issue_prompt(
        &repo_root,
        &repo,
        &issue_ref,
        &title,
        None,
        &version,
        DEFAULT_NEW_LABELS,
    )?;
    validate_issue_prompt_exists(&source_path)?;
    validate_bootstrap_stp(&repo_root, &source_path)?;
    validate_authored_prompt_surface("start", &source_path, PromptSurfaceKind::IssuePrompt)?;

    let root_stp = ensure_task_bundle_stp(&repo_root, &issue_ref, &source_path)?;
    validate_authored_prompt_surface("start", &root_stp, PromptSurfaceKind::Stp)?;

    ensure_pre_run_bootstrap_cards(&repo_root, &issue_ref, &title, &source_path)?;
    doctor::ensure_pr_run_design_time_ready(&repo_root, &issue_ref, &branch)?;
    let root_paths = ensure_bootstrap_cards(&repo_root, &issue_ref, &title, &branch, &source_path)?;

    ensure_git_metadata_writable()?;
    fetch_origin_main_with_fallback()?;
    ensure_local_branch_exists(&branch)?;
    ensure_worktree_for_branch(&worktree_path, &branch)?;

    let worktree_source = ensure_local_issue_prompt_copy(&worktree_path, &issue_ref, &source_path)?;
    mirror_docs_templates_into_worktree(&repo_root, &worktree_path)?;
    let worktree_stp = ensure_task_bundle_stp(&worktree_path, &issue_ref, &worktree_source)?;
    validate_authored_prompt_surface("start", &worktree_stp, PromptSurfaceKind::Stp)?;

    sync_root_task_bundle_into_worktree(&repo_root, &worktree_path, &issue_ref)?;
    let worktree_paths = ensure_bootstrap_cards(
        &worktree_path,
        &issue_ref,
        &title,
        &branch,
        &worktree_source,
    )?;
    ensure_worktree_task_bundle_materialized(&worktree_path, &issue_ref)?;

    println!("• Agent:");
    println!("  STP    {}", worktree_stp.display());
    println!(
        "  SPP    {}",
        issue_ref.task_bundle_plan_path(&worktree_path).display()
    );
    println!(
        "  SRP    {}",
        issue_ref
            .task_bundle_review_policy_path(&worktree_path)
            .display()
    );
    println!("  SIP    {}", worktree_paths.1.display());
    println!("  SOR    {}", worktree_paths.2.display());
    println!("  ROOT_STP    {}", root_stp.display());
    println!(
        "  ROOT_SPP    {}",
        issue_ref.task_bundle_plan_path(&repo_root).display()
    );
    println!(
        "  ROOT_SRP    {}",
        issue_ref
            .task_bundle_review_policy_path(&repo_root)
            .display()
    );
    println!("  ROOT_SIP    {}", root_paths.1.display());
    println!("  ROOT_SOR    {}", root_paths.2.display());
    println!("  WORKTREE {}", worktree_path.display());
    println!("  BRANCH {branch}");
    println!(
        "  OPEN   ./adl/tools/open_artifact.sh card {} output",
        parsed.issue
    );
    println!("  STATE  FULLY_STARTED");
    println!("• Done.");
    Ok(())
}

fn ensure_issue_run_primary_checkout_safe(repo_root: &Path) -> Result<()> {
    let primary_root = primary_checkout_root()?;
    if !same_checkout_root(repo_root, &primary_root)? {
        return Ok(());
    }
    if current_branch(repo_root)? != "main" {
        return Ok(());
    }
    let tracked_status = tracked_changes_status(repo_root)?;
    if tracked_status.trim().is_empty() {
        return Ok(());
    }
    bail!(
        "run: unsafe_root_checkout_execution: the primary checkout is on main with tracked changes. Issue-mode run may bind or reuse an issue worktree only from a tracked-clean main checkout. Move tracked edits into the issue worktree or clear them before rerunning; ignored local .adl notes may remain.\n{}",
        tracked_status.trim()
    );
}

fn same_checkout_root(left: &Path, right: &Path) -> Result<bool> {
    if left == right {
        return Ok(true);
    }
    let left = fs::canonicalize(left)
        .with_context(|| format!("failed to canonicalize checkout path '{}'", left.display()))?;
    let right = fs::canonicalize(right)
        .with_context(|| format!("failed to canonicalize checkout path '{}'", right.display()))?;
    Ok(left == right)
}

fn real_pr_ready(args: &[String]) -> Result<()> {
    eprintln!(
        "• Deprecated compatibility path: prefer `adl/tools/pr.sh doctor {} --mode ready ...`.",
        args.first()
            .cloned()
            .unwrap_or_else(|| "<issue>".to_string())
    );
    let parsed = parse_ready_args(args)?;
    doctor::run_doctor(
        DoctorArgs {
            issue: parsed.issue,
            version: parsed.version,
            slug: parsed.slug,
            no_fetch_issue: parsed.no_fetch_issue,
            mode: DoctorMode::Ready,
            json: parsed.json,
        },
        "ready",
    )
}

fn real_pr_preflight(args: &[String]) -> Result<()> {
    eprintln!(
        "• Deprecated compatibility path: prefer `adl/tools/pr.sh doctor {} --mode preflight ...`.",
        args.first()
            .cloned()
            .unwrap_or_else(|| "<issue>".to_string())
    );
    let parsed = parse_preflight_args(args)?;
    doctor::run_doctor(
        DoctorArgs {
            issue: parsed.issue,
            version: parsed.version,
            slug: parsed.slug,
            no_fetch_issue: parsed.no_fetch_issue,
            mode: DoctorMode::Preflight,
            json: parsed.json,
        },
        "preflight",
    )
}

fn real_pr_doctor(args: &[String]) -> Result<()> {
    let parsed = parse_doctor_args(args)?;
    doctor::run_doctor(parsed, "doctor")
}

fn real_pr_closeout(args: &[String]) -> Result<()> {
    let parsed = parse_closeout_args(args)?;
    let repo_root = repo_root()?;
    let primary_root = primary_checkout_root()?;
    let repo = default_repo(&primary_root)?;
    let inferred = resolve_issue_scope_and_slug_from_local_state(&primary_root, parsed.issue)?
        .unwrap_or((
            parsed
                .version
                .clone()
                .unwrap_or_else(|| DEFAULT_VERSION.to_string()),
            parsed
                .slug
                .clone()
                .unwrap_or_else(|| format!("issue-{}", parsed.issue)),
        ));
    let issue_ref = IssueRef::new(parsed.issue, inferred.0, inferred.1)?;
    let output_path = issue_ref.task_bundle_output_path(&primary_root);
    lifecycle::ensure_issue_closed_completed_for_closeout(parsed.issue, &repo)?;
    lifecycle::closeout_closed_completed_issue_bundle(
        &repo_root,
        &primary_root,
        &issue_ref,
        &output_path,
    )?;
    println!(
        "{}",
        path_relative_to_repo(
            &primary_root,
            &issue_ref.task_bundle_dir_path(&primary_root)
        )
    );
    Ok(())
}

fn print_json<T: Serialize>(value: &T) -> Result<()> {
    println!(
        "{}",
        serde_json::to_string_pretty(value).context("failed to serialize pr command json")?
    );
    Ok(())
}

fn real_pr_init(args: &[String]) -> Result<()> {
    let parsed = parse_init_args(args)?;
    let repo_root = repo_root()?;
    let repo = default_repo(&repo_root)?;
    let local_identity = resolve_local_issue_identity(&repo_root, parsed.issue)?;

    let issue = parsed.issue;
    let mut title = parsed.title_arg.clone().unwrap_or_default();
    let mut slug = parsed.slug.clone().unwrap_or_default();
    let mut slug_from_local_identity = false;

    if title.is_empty() && !parsed.no_fetch_issue {
        eprintln!("• Fetching issue title via gh…");
        title = gh_issue_title(issue, &repo)?.unwrap_or_default();
    }
    if slug.is_empty() {
        if let Some((_, local_slug)) = local_identity.as_ref() {
            slug = local_slug.clone();
            slug_from_local_identity = true;
        } else if !title.is_empty() {
            slug = sanitize_slug(&title);
            if slug.is_empty() {
                bail!("init: --title produced empty slug after sanitization");
            }
        } else if parsed.no_fetch_issue {
            bail!("init: --slug is required when --no-fetch-issue is set");
        }
        if slug.is_empty() && title.is_empty() {
            bail!(
                "Could not fetch issue #{} title. Pass --slug or check GitHub token/repo configuration.",
                issue
            );
        }
        if slug.is_empty() {
            slug = sanitize_slug(&title);
        }
    }
    if title.is_empty() {
        title = slug.clone();
    }

    let version = if let Some(version) = parsed.version.clone() {
        version
    } else if let Some((local_version, _)) = local_identity.as_ref() {
        local_version.clone()
    } else {
        resolve_version_for_existing_issue(
            &repo_root,
            &repo,
            issue,
            None,
            parsed.no_fetch_issue,
            "init",
        )?
    };
    title = normalize_issue_title_for_version(&title, &version);
    if parsed.slug.as_deref().unwrap_or_default().trim().is_empty() && !slug_from_local_identity {
        slug = sanitize_slug(&title);
        if slug.is_empty() {
            bail!("init: title produced empty slug after normalization");
        }
    }
    let fetched_labels = if parsed.no_fetch_issue {
        String::new()
    } else {
        fetched_issue_labels_csv(&repo, issue)?
    };
    let normalized_labels = normalize_labels_csv(
        if fetched_labels.trim().is_empty() {
            DEFAULT_NEW_LABELS
        } else {
            &fetched_labels
        },
        &version,
    );
    let issue_ref = IssueRef::new(issue, version.clone(), slug.clone())?;
    if !parsed.no_fetch_issue {
        ensure_issue_metadata_parity(&repo, issue, &title, &normalized_labels)?;
    }
    ensure_no_duplicate_issue_identities(&repo_root, &issue_ref)?;
    let source_path = ensure_source_issue_prompt(
        &repo_root,
        &repo,
        &issue_ref,
        &title,
        None,
        &version,
        DEFAULT_NEW_LABELS,
    )?;
    validate_issue_prompt_exists(&source_path)?;
    let (stp_path, bundle_input, bundle_output, bundle_dir) =
        bootstrap_root_task_bundle(&repo_root, &issue_ref, &title, &source_path)?;

    println!("• Initialized:");
    println!(
        "  STP      {}",
        path_relative_to_repo(&repo_root, &stp_path)
    );
    println!(
        "  SPP      {}",
        path_relative_to_repo(&repo_root, &issue_ref.task_bundle_plan_path(&repo_root))
    );
    println!(
        "  SRP      {}",
        path_relative_to_repo(
            &repo_root,
            &issue_ref.task_bundle_review_policy_path(&repo_root),
        )
    );
    println!(
        "  SIP      {}",
        path_relative_to_repo(&repo_root, &bundle_input)
    );
    println!(
        "  SOR      {}",
        path_relative_to_repo(&repo_root, &bundle_output)
    );
    println!(
        "  BUNDLE   {}",
        path_relative_to_repo(&repo_root, &bundle_dir)
    );
    println!(
        "  SOURCE   {}",
        path_relative_to_repo(&repo_root, &source_path)
    );
    println!("  ISSUE    #{issue}");
    println!("  CONTRACT validated source prompt + root SIP/STP/SPP/SRP/SOR task bundle");
    println!("  STATE    ISSUE_AND_BUNDLE_READY");
    println!("• Done.");
    Ok(())
}

fn bootstrap_ready_status_label(status: &str) -> &str {
    match status {
        "BLOCK" => "BLOCKED_PENDING_CARD_REVIEW",
        other => other,
    }
}

#[cfg(test)]
mod bootstrap_output_tests {
    use super::bootstrap_ready_status_label;

    #[test]
    fn bootstrap_ready_status_explains_card_review_blockers() {
        assert_eq!(
            bootstrap_ready_status_label("BLOCK"),
            "BLOCKED_PENDING_CARD_REVIEW"
        );
        assert_eq!(bootstrap_ready_status_label("PASS"), "PASS");
    }
}

fn bootstrap_root_task_bundle(
    repo_root: &Path,
    issue_ref: &IssueRef,
    title: &str,
    source_path: &Path,
) -> Result<(PathBuf, PathBuf, PathBuf, PathBuf)> {
    let bundle_dir = issue_ref.task_bundle_dir_path(repo_root);
    let init_branch = "not bound yet";
    eprintln!("• Initializing task bundle: {}", bundle_dir.display());
    let stp_path = ensure_task_bundle_stp(repo_root, issue_ref, source_path)?;
    let (_, bundle_input, bundle_output) =
        ensure_bootstrap_cards(repo_root, issue_ref, title, init_branch, source_path)?;
    Ok((stp_path, bundle_input, bundle_output, bundle_dir))
}

fn infer_required_outcome_type_for_create(labels_csv: &str, title: &str) -> &'static str {
    infer_required_outcome_type(labels_csv, title)
}

#[cfg(test)]
fn run_create_post_bootstrap_test_hook(repo_root: &Path, issue_ref: &IssueRef) -> Result<()> {
    if let Some(hook) = CREATE_POST_BOOTSTRAP_HOOK
        .get_or_init(|| Mutex::new(None))
        .lock()
        .expect("lock create post-bootstrap hook")
        .as_ref()
        .copied()
    {
        hook(repo_root, issue_ref)?;
    }
    Ok(())
}

#[cfg(not(test))]
fn run_create_post_bootstrap_test_hook(_repo_root: &Path, _issue_ref: &IssueRef) -> Result<()> {
    Ok(())
}

#[cfg(test)]
fn set_create_post_bootstrap_test_hook(
    hook: Option<CreatePostBootstrapHook>,
) -> Option<CreatePostBootstrapHook> {
    let mut guard = CREATE_POST_BOOTSTRAP_HOOK
        .get_or_init(|| Mutex::new(None))
        .lock()
        .expect("lock create post-bootstrap hook");
    std::mem::replace(&mut *guard, hook)
}

#[cfg(test)]
#[path = "tests/pr_cmd_inline/mod.rs"]
mod tests;
