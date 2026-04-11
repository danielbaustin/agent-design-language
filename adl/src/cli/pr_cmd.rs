use anyhow::{anyhow, bail, Context, Result};
use serde::Serialize;
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
#[cfg(test)]
use std::process::Command;

use super::pr_cmd_args::{
    parse_closeout_args, parse_create_args, parse_doctor_args, parse_finish_args, parse_init_args,
    parse_preflight_args, parse_ready_args, parse_start_args, DoctorArgs, DoctorMode,
};
use super::pr_cmd_cards::{
    branch_indicates_unbound_state, ensure_bootstrap_cards, ensure_local_issue_prompt_copy,
    ensure_source_issue_prompt, ensure_symlink, ensure_task_bundle_stp, field_line_value,
    mirror_docs_templates_into_worktree, path_relative_to_repo, validate_bootstrap_stp,
    validate_initialized_cards, validate_issue_body_for_create, validate_ready_cards,
    write_source_issue_prompt,
};
#[cfg(test)]
use super::pr_cmd_prompt::load_issue_prompt;
use super::pr_cmd_prompt::{
    ensure_no_duplicate_issue_identities, infer_required_outcome_type,
    normalize_issue_title_for_version, normalize_labels_csv, parse_issue_number_from_url,
    render_generated_issue_body, resolve_issue_body, resolve_issue_prompt_path,
    resolve_issue_scope_and_slug_from_local_state, validate_issue_prompt_exists,
    version_from_labels_csv, version_from_title,
};
use super::pr_cmd_validate::{
    validate_authored_prompt_surface, validate_milestone_doc_drift_for_finish, PromptSurfaceKind,
};
use ::adl::control_plane::{
    card_output_path, resolve_cards_root, resolve_primary_checkout_root, sanitize_slug, IssueRef,
};

mod doctor;
mod git_support;
mod github;
mod lifecycle;

#[cfg(test)]
use self::git_support::{branch_checked_out_worktree_path, infer_repo_from_remote};
use self::git_support::{
    commits_ahead_of_origin_main, current_branch, default_repo, ensure_git_metadata_writable,
    ensure_local_branch_exists, ensure_not_on_main_branch, ensure_worktree_for_branch,
    fetch_origin_main_with_fallback, has_uncommitted_changes, issue_create_repo, path_str,
    primary_checkout_root, repo_root, run_capture, run_capture_allow_failure, run_status,
    run_status_allow_failure,
};
#[cfg(test)]
use self::github::pr_has_closing_linkage;
use self::github::{
    attach_pr_janitor, current_pr_url, ensure_issue_metadata_parity, ensure_pr_closing_linkage,
    format_open_pr_wave, gh_issue_create, gh_issue_edit_body, gh_issue_title, issue_version,
    unresolved_milestone_pr_wave,
};

const DEFAULT_VERSION: &str = "v0.86";
const DEFAULT_NEW_LABELS: &str = "track:roadmap,type:task,area:tools";
pub(crate) fn real_pr(args: &[String]) -> Result<()> {
    let Some(subcommand) = args.first().map(|s| s.as_str()) else {
        bail!(
            "pr requires a subcommand: create | init | start | doctor | ready | preflight | finish | closeout"
        );
    };

    match subcommand {
        "create" => real_pr_create(&args[1..]),
        "init" => real_pr_init(&args[1..]),
        "start" => real_pr_start(&args[1..]),
        "doctor" => real_pr_doctor(&args[1..]),
        "ready" => real_pr_ready(&args[1..]),
        "preflight" => real_pr_preflight(&args[1..]),
        "finish" => real_pr_finish(&args[1..]),
        "closeout" => real_pr_closeout(&args[1..]),
        other => bail!("unknown pr subcommand: {other}"),
    }
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

    let version = if let Some(version) = parsed.version.clone() {
        version
    } else {
        parsed
            .labels
            .as_deref()
            .and_then(version_from_labels_csv)
            .or_else(|| version_from_title(&raw_title))
            .unwrap_or_else(|| DEFAULT_VERSION.to_string())
    };
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
        "  READ       {}",
        path_relative_to_repo(&repo_root, &bundle_input)
    );
    println!(
        "  WRITE      {}",
        path_relative_to_repo(&repo_root, &bundle_output)
    );
    println!(
        "  BUNDLE     {}",
        path_relative_to_repo(&repo_root, &bundle_dir)
    );
    println!("  NEXT       qualitative STP/SIP review, then adl/tools/pr.sh run {issue} --slug {slug} --version {version}");
    println!("  STATE      ISSUE_AND_BUNDLE_READY");
    eprintln!("• Done.");
    Ok(())
}

fn real_pr_start(args: &[String]) -> Result<()> {
    let parsed = parse_start_args(args)?;
    let repo_root = repo_root()?;
    let repo = default_repo(&repo_root)?;

    eprintln!(
        "• Deprecated compatibility path: prefer `adl/tools/pr.sh run {}` for execution-context binding.",
        parsed.issue
    );

    let mut title = parsed.title_arg.clone().unwrap_or_default();
    let mut slug = parsed.slug.clone().unwrap_or_default();
    if slug.is_empty() && !title.is_empty() {
        slug = sanitize_slug(&title);
        if slug.is_empty() {
            bail!("start: --title produced empty slug after sanitization");
        }
    }
    if title.is_empty() && !parsed.no_fetch_issue {
        eprintln!("• Fetching issue title via gh…");
        title = gh_issue_title(parsed.issue, &repo)?.unwrap_or_default();
    }
    if slug.is_empty() {
        if parsed.no_fetch_issue {
            bail!("start: --slug is required when --no-fetch-issue is set");
        }
        if title.is_empty() {
            bail!(
                "Could not fetch issue #{} title. Pass --slug or check gh auth/repo.",
                parsed.issue
            );
        }
        slug = sanitize_slug(&title);
    }
    if title.is_empty() {
        title = slug.clone();
    }

    let version = if let Some(version) = parsed.version.clone() {
        version
    } else if parsed.no_fetch_issue {
        DEFAULT_VERSION.to_string()
    } else {
        issue_version(parsed.issue, &repo)?.unwrap_or_else(|| DEFAULT_VERSION.to_string())
    };
    title = normalize_issue_title_for_version(&title, &version);
    if parsed.slug.as_deref().unwrap_or_default().trim().is_empty() {
        slug = sanitize_slug(&title);
        if slug.is_empty() {
            bail!("start: title produced empty slug after normalization");
        }
    }
    let fetched_labels = if parsed.no_fetch_issue {
        String::new()
    } else {
        run_capture_allow_failure(
            "gh",
            &[
                "issue",
                "view",
                &parsed.issue.to_string(),
                "-R",
                &repo,
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
        .join(",")
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
    let unresolved = unresolved_milestone_pr_wave(&repo, &version, Some(&branch))?;
    if !parsed.allow_open_pr_wave && !unresolved.is_empty() {
        bail!(
            "start: unresolved open PR wave detected for {}. Resolve or merge these PRs first, or rerun with --allow-open-pr-wave if you are deliberately overriding the guard:\n{}",
            version,
            format_open_pr_wave(&unresolved)
        );
    }
    let managed_root = std::env::var_os("ADL_WORKTREE_ROOT").map(PathBuf::from);
    let worktree_path = issue_ref.default_worktree_path(&repo_root, managed_root.as_deref());

    eprintln!("• Target branch: {branch}");
    eprintln!("• Target worktree: {}", worktree_path.display());

    ensure_git_metadata_writable()?;
    fetch_origin_main_with_fallback()?;
    ensure_local_branch_exists(&branch)?;
    ensure_worktree_for_branch(&worktree_path, &branch)?;

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
    let worktree_source = ensure_local_issue_prompt_copy(&worktree_path, &issue_ref, &source_path)?;
    mirror_docs_templates_into_worktree(&repo_root, &worktree_path)?;
    let worktree_stp = ensure_task_bundle_stp(&worktree_path, &issue_ref, &worktree_source)?;
    validate_authored_prompt_surface("start", &root_stp, PromptSurfaceKind::Stp)?;
    validate_authored_prompt_surface("start", &worktree_stp, PromptSurfaceKind::Stp)?;

    let root_paths = ensure_bootstrap_cards(&repo_root, &issue_ref, &title, &branch, &source_path)?;
    let worktree_paths = ensure_bootstrap_cards(
        &worktree_path,
        &issue_ref,
        &title,
        &branch,
        &worktree_source,
    )?;

    println!("• Agent:");
    println!("  STP    {}", worktree_stp.display());
    println!("  READ   {}", worktree_paths.1.display());
    println!("  WRITE  {}", worktree_paths.2.display());
    println!("  ROOT_STP    {}", root_stp.display());
    println!("  ROOT_READ   {}", root_paths.1.display());
    println!("  ROOT_WRITE  {}", root_paths.2.display());
    println!("  WORKTREE {}", worktree_path.display());
    println!("  BRANCH {branch}");
    println!(
        "  OPEN   ./adl/tools/open_artifact.sh card {} output",
        parsed.issue
    );
    println!("  STATE  FULLY_STARTED");
    eprintln!("• Done.");
    Ok(())
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

fn real_pr_finish(args: &[String]) -> Result<()> {
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

    if !parsed.no_checks {
        run_batched_checks_rust(&repo_root)?;
    }

    let canonical_publish_paths = canonical_finish_bundle_paths(
        &repo_root,
        &source_path,
        &stp_path,
        &input_path,
        &output_path,
    )?;

    stage_selected_paths_rust(&repo_root, &parsed.paths, &canonical_publish_paths)?;
    let has_uncommitted = has_uncommitted_changes(&repo_root)?;
    let ahead = commits_ahead_of_origin_main(&repo_root)?;
    if staged_diff_is_empty(&repo_root)? {
        if !has_uncommitted && ahead == 0 {
            bail!("No changes detected and branch has no commits ahead of origin/main. Nothing to PR.");
        }
    } else if !parsed.allow_gitignore && staged_gitignore_change_present(&repo_root)? {
        bail!(
            "finish: staged .gitignore or adl/.gitignore changes detected. Revert them or re-run with --allow-gitignore. Canonical issue bundle files are staged automatically."
        );
    }

    let changed_paths = finish_changed_paths(&repo_root, has_uncommitted)?;
    validate_milestone_doc_drift_for_finish(&repo_root, issue_ref.scope(), &changed_paths)?;

    let close_line = if parsed.no_close {
        None
    } else {
        Some(format!("Closes #{}", parsed.issue))
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
        parsed.no_checks,
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

    ensure_pr_closing_linkage(&repo, &pr_url, parsed.issue, parsed.no_close)?;

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

    if !parsed.no_open {
        let _ = run_status_allow_failure("open", &[&pr_url])?;
    }

    println!("{pr_url}");
    Ok(())
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

fn finish_changed_paths(repo_root: &Path, has_uncommitted: bool) -> Result<Vec<String>> {
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

    let issue = parsed.issue;
    let mut title = parsed.title_arg.clone().unwrap_or_default();
    let mut slug = parsed.slug.clone().unwrap_or_default();
    if slug.is_empty() && !title.is_empty() {
        slug = sanitize_slug(&title);
        if slug.is_empty() {
            bail!("init: --title produced empty slug after sanitization");
        }
    }

    if title.is_empty() && !parsed.no_fetch_issue {
        eprintln!("• Fetching issue title via gh…");
        title = gh_issue_title(issue, &repo)?.unwrap_or_default();
    }
    if slug.is_empty() {
        if parsed.no_fetch_issue {
            bail!("init: --slug is required when --no-fetch-issue is set");
        }
        if title.is_empty() {
            bail!(
                "Could not fetch issue #{} title. Pass --slug or check gh auth/repo.",
                issue
            );
        }
        slug = sanitize_slug(&title);
    }
    if title.is_empty() {
        title = slug.clone();
    }

    let version = if let Some(version) = parsed.version.clone() {
        version
    } else if parsed.no_fetch_issue {
        DEFAULT_VERSION.to_string()
    } else {
        issue_version(issue, &repo)?.unwrap_or_else(|| DEFAULT_VERSION.to_string())
    };
    title = normalize_issue_title_for_version(&title, &version);
    if parsed.slug.as_deref().unwrap_or_default().trim().is_empty() {
        slug = sanitize_slug(&title);
        if slug.is_empty() {
            bail!("init: title produced empty slug after normalization");
        }
    }
    let fetched_labels = if parsed.no_fetch_issue {
        String::new()
    } else {
        run_capture_allow_failure(
            "gh",
            &[
                "issue",
                "view",
                &issue.to_string(),
                "-R",
                &repo,
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
        .join(",")
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
        "  READ     {}",
        path_relative_to_repo(&repo_root, &bundle_input)
    );
    println!(
        "  WRITE    {}",
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
    println!("  CONTRACT minimum v0.86 init = validated source prompt + root stp/sip/sor bundle");
    println!("  STATE    ISSUE_AND_BUNDLE_READY");
    eprintln!("• Done.");
    Ok(())
}

fn bootstrap_root_task_bundle(
    repo_root: &Path,
    issue_ref: &IssueRef,
    title: &str,
    source_path: &Path,
) -> Result<(PathBuf, PathBuf, PathBuf, PathBuf)> {
    let stp_path = issue_ref.task_bundle_stp_path(repo_root);
    let bundle_dir = issue_ref.task_bundle_dir_path(repo_root);
    let init_branch = "not bound yet";
    eprintln!("• Initializing task bundle: {}", bundle_dir.display());
    if !stp_path.is_file() {
        if let Some(parent) = stp_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::copy(source_path, &stp_path).with_context(|| {
            format!(
                "failed to seed task-bundle stp from '{}' to '{}'",
                source_path.display(),
                stp_path.display()
            )
        })?;
    } else {
        eprintln!("• STP already exists: {}", stp_path.display());
    }
    let (_, bundle_input, bundle_output) =
        ensure_bootstrap_cards(repo_root, issue_ref, title, init_branch, source_path)?;
    Ok((stp_path, bundle_input, bundle_output, bundle_dir))
}

fn ensure_nonempty_file_path(path: &Path) -> Result<bool> {
    if !path.is_file() {
        return Ok(false);
    }
    let text = fs::read_to_string(path)?;
    Ok(!text.trim().is_empty())
}

fn ensure_output_card_is_started(output_path: &Path) -> Result<()> {
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

fn validate_completed_sor(repo_root: &Path, output_path: &Path) -> Result<()> {
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

fn run_batched_checks_rust(repo_root: &Path) -> Result<()> {
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

fn canonical_finish_bundle_paths(
    repo_root: &Path,
    source_path: &Path,
    stp_path: &Path,
    input_path: &Path,
    output_path: &Path,
) -> Result<Vec<String>> {
    let mut out = BTreeSet::new();
    for path in [source_path, stp_path, input_path, output_path] {
        let normalized = if path.is_absolute() {
            path.to_path_buf()
        } else {
            repo_root.join(path)
        };
        if !normalized.exists() {
            continue;
        }
        if let Ok(relative) = normalized.strip_prefix(repo_root) {
            out.insert(relative.to_string_lossy().into_owned());
        }
    }
    Ok(out.into_iter().collect())
}

fn stage_selected_paths_rust(repo_root: &Path, csv: &str, force_paths: &[String]) -> Result<()> {
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

    if !force_paths.is_empty() {
        let mut force_args = vec!["-C", path_str(repo_root)?, "add", "-f", "--"];
        force_args.extend(force_paths.iter().map(String::as_str));
        run_status("git", &force_args)?;
    }

    Ok(())
}

fn staged_diff_is_empty(repo_root: &Path) -> Result<bool> {
    run_status_allow_failure(
        "git",
        &["-C", path_str(repo_root)?, "diff", "--cached", "--quiet"],
    )
}

fn staged_gitignore_change_present(repo_root: &Path) -> Result<bool> {
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

fn extract_markdown_section(path: &Path, heading: &str) -> Result<String> {
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

fn extra_pr_body_looks_like_issue_template(body: &str) -> bool {
    let lowered = body.to_lowercase();
    lowered.contains("issue_card_schema:")
        || lowered.contains("wp:")
        || lowered.contains("pr_start:")
        || lowered.contains("## goal")
        || lowered.contains("## deliverables")
        || lowered.contains("\n---\n")
}

fn render_pr_body(
    close_line: Option<&str>,
    input_path: &Path,
    output_path: &Path,
    extra_body: Option<&str>,
    no_checks: bool,
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
    } else if !no_checks {
        parts.push("## Validation".to_string());
        parts.push("- cargo fmt".to_string());
        parts.push("- cargo clippy --all-targets -- -D warnings".to_string());
        parts.push("- cargo test".to_string());
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

fn finish_inputs_fingerprint(
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

fn write_temp_markdown(prefix: &str, body: &str) -> Result<PathBuf> {
    let mut path = std::env::temp_dir();
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("time")
        .as_nanos();
    path.push(format!("{prefix}-{nanos}.md"));
    fs::write(&path, body)?;
    Ok(path)
}

fn infer_required_outcome_type_for_create(labels_csv: &str, title: &str) -> &'static str {
    infer_required_outcome_type(labels_csv, title)
}

#[cfg(test)]
#[path = "tests/pr_cmd_inline/mod.rs"]
mod tests;
