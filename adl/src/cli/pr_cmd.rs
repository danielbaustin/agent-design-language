use anyhow::{anyhow, bail, Context, Result};
use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};
#[cfg(test)]
use std::process::Command;
#[cfg(test)]
use std::sync::{Mutex, OnceLock};

#[cfg(test)]
use super::pr_cmd_args::parse_finish_args;
use super::pr_cmd_args::{
    parse_closeout_args, parse_create_args, parse_doctor_args, parse_init_args,
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
    ensure_no_duplicate_issue_identities, infer_required_outcome_type, infer_workflow_queue,
    normalize_issue_title_for_version, normalize_labels_csv, parse_issue_number_from_url,
    render_generated_issue_body, resolve_issue_body, resolve_issue_prompt_path,
    resolve_issue_prompt_workflow_queue, resolve_issue_scope_and_slug_from_local_state,
    validate_issue_prompt_exists, version_from_labels_csv, version_from_title,
    WorkflowQueueResolution,
};
use super::pr_cmd_validate::{validate_authored_prompt_surface, PromptSurfaceKind};
use ::adl::control_plane::{
    card_output_path, resolve_cards_root, resolve_primary_checkout_root, sanitize_slug, IssueRef,
};

mod doctor;
mod finish_support;
mod git_support;
mod github;
mod lifecycle;

#[cfg(test)]
type CreatePostBootstrapHook = fn(&Path, &IssueRef) -> Result<()>;

#[cfg(test)]
static CREATE_POST_BOOTSTRAP_HOOK: OnceLock<Mutex<Option<CreatePostBootstrapHook>>> =
    OnceLock::new();

#[cfg(test)]
use self::finish_support::{
    ensure_issue_surfaces_are_local_only, ensure_output_card_is_started,
    extra_pr_body_looks_like_issue_template, extract_markdown_section, finish_changed_paths,
    finish_inputs_fingerprint, issue_bundle_issue_number_from_repo_relative,
    render_default_finish_validation, render_pr_body, run_finish_validation_rust,
    select_finish_validation_mode, stage_selected_paths_rust, staged_diff_is_empty,
    staged_gitignore_change_present, tracked_issue_surface_paths, FinishValidationMode,
};
use self::finish_support::{
    ensure_nonempty_file_path, validate_completed_sor, write_temp_markdown,
};
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
    repo_root, run_capture, run_capture_allow_failure, run_status, tracked_changes_status,
};
#[cfg(test)]
use self::github::ensure_pr_closing_linkage;
#[cfg(test)]
use self::github::pr_has_closing_linkage;
#[cfg(test)]
use self::github::{current_pr_url, ensure_or_repair_pr_closing_linkage};
use self::github::{
    ensure_issue_metadata_parity, format_open_pr_wave, gh_issue_create, gh_issue_edit_body,
    gh_issue_title, issue_version, unresolved_milestone_pr_wave,
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
        "finish" => finish_support::real_pr_finish(&args[1..]),
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
    println!("  READY      {}", ready.status);
    println!("  LIFECYCLE  {}", ready.lifecycle_state);
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

    let version = resolve_version_for_existing_issue(
        &repo_root,
        &repo,
        parsed.issue,
        parsed.version.clone(),
        parsed.no_fetch_issue,
        "start",
    )?;
    title = normalize_issue_title_for_version(&title, &version);
    if parsed.slug.as_deref().unwrap_or_default().trim().is_empty() {
        slug = sanitize_slug(&title);
        if slug.is_empty() {
            bail!("start: title produced empty slug after normalization");
        }
    }
    ensure_issue_run_primary_checkout_safe(&repo_root)?;
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
    let target_queue = if issue_ref.issue_prompt_path(&repo_root).is_file() {
        resolve_issue_prompt_workflow_queue(&issue_ref.issue_prompt_path(&repo_root))?
    } else {
        WorkflowQueueResolution {
            queue: infer_workflow_queue(&title, &normalized_labels, None)
                .ok_or_else(|| {
                    anyhow!(
                        "start: missing or invalid workflow queue for issue #{}; add a canonical queue such as wp/tools/demo/docs/review/release before execution",
                        parsed.issue
                    )
                })?
                .to_string(),
            source: "inferred",
        }
    };
    let unresolved =
        unresolved_milestone_pr_wave(&repo, &version, &target_queue.queue, Some(&branch))?;
    if !parsed.allow_open_pr_wave && !unresolved.is_empty() {
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

    let version = resolve_version_for_existing_issue(
        &repo_root,
        &repo,
        issue,
        parsed.version.clone(),
        parsed.no_fetch_issue,
        "init",
    )?;
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
