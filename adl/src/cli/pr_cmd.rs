use anyhow::{anyhow, bail, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use super::pr_cmd_args::{
    parse_create_args, parse_doctor_args, parse_finish_args, parse_init_args, parse_preflight_args,
    parse_ready_args, parse_start_args, DoctorArgs, DoctorMode,
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
    infer_required_outcome_type, normalize_labels_csv, parse_issue_number_from_url,
    render_generated_issue_body, resolve_issue_body, resolve_issue_prompt_path,
    resolve_issue_scope_and_slug_from_local_state, validate_issue_prompt_exists,
    version_from_labels_csv, version_from_title,
};
use super::pr_cmd_validate::{validate_authored_prompt_surface, PromptSurfaceKind};
use ::adl::control_plane::{
    card_output_path, resolve_cards_root, resolve_primary_checkout_root, sanitize_slug, IssueRef,
};

const DEFAULT_VERSION: &str = "v0.86";
const DEFAULT_NEW_LABELS: &str = "track:roadmap,type:task,area:tools";
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
struct OpenPullRequest {
    number: u32,
    title: String,
    url: String,
    #[serde(rename = "headRefName")]
    head_ref_name: String,
    #[serde(rename = "baseRefName")]
    base_ref_name: String,
    #[serde(rename = "isDraft")]
    is_draft: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
struct GithubIssueLifecycleState {
    state: String,
    #[serde(rename = "stateReason")]
    state_reason: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct DoctorPreflightJsonPullRequest {
    number: u32,
    head_ref_name: String,
    state: &'static str,
    url: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DoctorPreflightResult {
    open_pr_count: usize,
    open_prs: Vec<DoctorPreflightJsonPullRequest>,
    status: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DoctorReadyResult {
    lifecycle_state: &'static str,
    worktree: Option<String>,
    source: String,
    root_stp: String,
    root_input: String,
    root_output: String,
    wt_stp: Option<String>,
    wt_input: Option<String>,
    wt_output: Option<String>,
    status: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct DoctorJsonOutput {
    schema: &'static str,
    issue: u32,
    version: String,
    slug: String,
    branch: String,
    mode: &'static str,
    preflight_status: &'static str,
    open_pr_count: usize,
    open_prs: Vec<DoctorPreflightJsonPullRequest>,
    lifecycle_state: Option<&'static str>,
    ready_status: Option<&'static str>,
    worktree: Option<String>,
    source: Option<String>,
    root_stp: Option<String>,
    root_input: Option<String>,
    root_output: Option<String>,
    wt_stp: Option<String>,
    wt_input: Option<String>,
    wt_output: Option<String>,
    doctor_status: &'static str,
}

pub(crate) fn real_pr(args: &[String]) -> Result<()> {
    let Some(subcommand) = args.first().map(|s| s.as_str()) else {
        bail!(
            "pr requires a subcommand: create | init | start | doctor | ready | preflight | finish"
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
        other => bail!("unknown pr subcommand: {other}"),
    }
}

fn real_pr_create(args: &[String]) -> Result<()> {
    let parsed = parse_create_args(args)?;
    let repo_root = repo_root()?;
    let repo = default_repo(&repo_root)?;

    let title = parsed.title_arg.clone().unwrap_or_default();
    let mut slug = parsed.slug.clone().unwrap_or_default();
    if slug.is_empty() {
        slug = sanitize_slug(&title);
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
            .or_else(|| version_from_title(&title))
            .unwrap_or_else(|| DEFAULT_VERSION.to_string())
    };

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
    ensure_issue_labels(&repo, issue, &normalized_labels)?;
    let issue_ref = IssueRef::new(issue, version.clone(), slug.clone())?;
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

    let issue_ref = IssueRef::new(parsed.issue, version.clone(), slug.clone())?;
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
    run_doctor(
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
    run_doctor(
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
    run_doctor(parsed, "doctor")
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
    sync_completed_output_surfaces(&repo_root, &primary_root, &issue_ref, &output_path)?;

    let ahead = commits_ahead_of_origin_main(&repo_root)?;
    let has_uncommitted = has_uncommitted_changes(&repo_root)?;
    if !has_uncommitted && ahead == 0 {
        bail!("No changes detected and branch has no commits ahead of origin/main. Nothing to PR.");
    }

    if !parsed.no_checks {
        run_batched_checks_rust(&repo_root)?;
    }

    if has_uncommitted {
        stage_selected_paths_rust(&repo_root, &parsed.paths)?;
        if staged_diff_is_empty(&repo_root)? {
            bail!(
                "finish: nothing staged after 'git add' for paths '{}'",
                parsed.paths
            );
        }
        if !parsed.allow_gitignore && staged_gitignore_change_present(&repo_root)? {
            bail!("finish: .gitignore changes detected. Revert them or re-run with --allow-gitignore.");
        }
    }

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
        println!("{pr_url}");
        return Ok(());
    }

    if parsed.ready {
        let _ = run_status_allow_failure("gh", &["pr", "ready", "-R", &repo, &pr_url])?;
    }

    if !parsed.no_open {
        let _ = run_status_allow_failure("open", &[&pr_url])?;
    }

    println!("{pr_url}");
    Ok(())
}

fn run_doctor(parsed: DoctorArgs, label: &str) -> Result<()> {
    let repo_root = primary_checkout_root()?;
    let repo = default_repo(&repo_root)?;
    let (version, slug) = resolve_doctor_scope_and_slug(&repo_root, &parsed, label)?;
    let issue_ref = IssueRef::new(parsed.issue, version.clone(), slug.clone())?;
    let branch = issue_ref.branch_name("codex");

    let preflight = run_doctor_preflight(&repo, &version, &branch)?;
    let ready = match parsed.mode {
        DoctorMode::Preflight => None,
        DoctorMode::Ready | DoctorMode::Full => {
            Some(run_doctor_ready(&repo_root, &repo, &issue_ref, &branch)?)
        }
    };
    let mode = doctor_mode_name(&parsed.mode);
    let doctor_status = match parsed.mode {
        DoctorMode::Preflight => preflight.status,
        DoctorMode::Ready => ready.as_ref().map(|x| x.status).unwrap_or("BLOCK"),
        DoctorMode::Full => {
            if preflight.status == "PASS" && ready.as_ref().map(|x| x.status) == Some("PASS") {
                "PASS"
            } else {
                "BLOCK"
            }
        }
    };

    if parsed.json {
        print_json(&DoctorJsonOutput {
            schema: "adl.pr.doctor.v1",
            issue: parsed.issue,
            version,
            slug,
            branch,
            mode,
            preflight_status: preflight.status,
            open_pr_count: preflight.open_pr_count,
            open_prs: preflight.open_prs,
            lifecycle_state: ready.as_ref().map(|x| x.lifecycle_state),
            ready_status: ready.as_ref().map(|x| x.status),
            worktree: ready.as_ref().and_then(|x| x.worktree.clone()),
            source: ready.as_ref().map(|x| x.source.clone()),
            root_stp: ready.as_ref().map(|x| x.root_stp.clone()),
            root_input: ready.as_ref().map(|x| x.root_input.clone()),
            root_output: ready.as_ref().map(|x| x.root_output.clone()),
            wt_stp: ready.as_ref().and_then(|x| x.wt_stp.clone()),
            wt_input: ready.as_ref().and_then(|x| x.wt_input.clone()),
            wt_output: ready.as_ref().and_then(|x| x.wt_output.clone()),
            doctor_status,
        })?;
    } else {
        println!("ISSUE={}", parsed.issue);
        println!("VERSION={version}");
        println!("SLUG={slug}");
        println!("BRANCH={branch}");
        print_doctor_preflight_text(&preflight);
        if let Some(ready) = &ready {
            print_doctor_ready_text(ready);
        }
        println!("DOCTOR_MODE={mode}");
        println!("DOCTOR_STATUS={doctor_status}");
    }
    Ok(())
}

fn resolve_doctor_scope_and_slug(
    repo_root: &Path,
    parsed: &DoctorArgs,
    label: &str,
) -> Result<(String, String)> {
    if let (Some(version), Some(slug)) = (parsed.version.clone(), parsed.slug.clone()) {
        return Ok((version, slug));
    }
    let inferred = resolve_issue_scope_and_slug_from_local_state(repo_root, parsed.issue)?;
    let version = parsed
        .version
        .clone()
        .or(inferred.as_ref().map(|x| x.0.clone()))
        .unwrap_or_else(|| DEFAULT_VERSION.to_string());
    let slug = match parsed.mode {
        DoctorMode::Preflight => parsed
            .slug
            .clone()
            .or(inferred.map(|x| x.1))
            .unwrap_or_else(|| format!("issue-{}", parsed.issue)),
        DoctorMode::Ready | DoctorMode::Full => parsed.slug.clone().or(inferred.map(|x| x.1)).ok_or_else(|| {
            if label == "ready" {
                anyhow!("ready: could not infer slug; pass --slug or run start first")
            } else {
                anyhow!("doctor: could not infer slug for readiness check; pass --slug or create the execution context first")
            }
        })?,
    };
    Ok((version, slug))
}

fn run_doctor_preflight(repo: &str, version: &str, branch: &str) -> Result<DoctorPreflightResult> {
    let unresolved = unresolved_milestone_pr_wave(repo, version, Some(branch))?;
    let open_prs = unresolved
        .iter()
        .map(|pr| DoctorPreflightJsonPullRequest {
            number: pr.number,
            head_ref_name: pr.head_ref_name.clone(),
            state: if pr.is_draft { "draft" } else { "ready" },
            url: pr.url.clone(),
        })
        .collect::<Vec<_>>();
    if open_prs.is_empty() {
        Ok(DoctorPreflightResult {
            open_pr_count: 0,
            open_prs,
            status: "PASS",
        })
    } else {
        Ok(DoctorPreflightResult {
            open_pr_count: open_prs.len(),
            open_prs,
            status: "BLOCK",
        })
    }
}

fn run_doctor_ready(
    repo_root: &Path,
    repo: &str,
    issue_ref: &IssueRef,
    branch: &str,
) -> Result<DoctorReadyResult> {
    let worktree_path = issue_ref.default_worktree_path(
        repo_root,
        std::env::var_os("ADL_WORKTREE_ROOT")
            .map(PathBuf::from)
            .as_deref(),
    );
    let source_path = resolve_issue_prompt_path(repo_root, issue_ref)?;
    let root_stp = issue_ref.task_bundle_stp_path(repo_root);
    let wt_stp = issue_ref.task_bundle_stp_path(&worktree_path);
    let root_bundle_input = issue_ref.task_bundle_input_path(repo_root);
    let root_bundle_output = issue_ref.task_bundle_output_path(repo_root);
    let wt_bundle_input = issue_ref.task_bundle_input_path(&worktree_path);
    let wt_bundle_output = issue_ref.task_bundle_output_path(&worktree_path);
    let closed_completed = issue_is_closed_and_completed(issue_ref.issue_number(), repo)?;

    validate_issue_prompt_exists(&source_path)?;
    validate_bootstrap_stp(repo_root, &source_path)?;
    validate_authored_prompt_surface("doctor", &source_path, PromptSurfaceKind::IssuePrompt)?;
    if closed_completed {
        reconcile_closed_completed_issue_bundle(repo_root, issue_ref, &root_bundle_output)?;
    }
    if !root_stp.is_file() {
        bail!("doctor: missing root stp: {}", root_stp.display());
    }
    validate_bootstrap_stp(repo_root, &root_stp)?;
    validate_authored_prompt_surface("doctor", &root_stp, PromptSurfaceKind::Stp)?;
    if closed_completed {
        validate_initialized_cards(
            issue_ref.issue_number(),
            issue_ref.slug(),
            &root_bundle_input,
            &root_bundle_output,
        )?;
        return Ok(DoctorReadyResult {
            lifecycle_state: "closed",
            worktree: None,
            source: path_relative_to_repo(repo_root, &source_path),
            root_stp: path_relative_to_repo(repo_root, &root_stp),
            root_input: path_relative_to_repo(repo_root, &root_bundle_input),
            root_output: path_relative_to_repo(repo_root, &root_bundle_output),
            wt_stp: None,
            wt_input: None,
            wt_output: None,
            status: "PASS",
        });
    }
    validate_initialized_cards(
        issue_ref.issue_number(),
        issue_ref.slug(),
        &root_bundle_input,
        &root_bundle_output,
    )?;
    let root_input_body = fs::read_to_string(&root_bundle_input).with_context(|| {
        format!(
            "doctor: read root input card: {}",
            root_bundle_input.display()
        )
    })?;
    let root_branch = field_line_value(&root_bundle_input, "Branch")?;
    let root_indicates_pre_run = branch_indicates_unbound_state(&root_branch)
        || root_input_body.contains(
            "This issue is not started yet; do not assume a branch or worktree already exists.",
        )
        || root_input_body
            .contains("Do not assume a branch or worktree already exists before `pr run`.");
    if !worktree_path.is_dir() {
        if root_indicates_pre_run {
            return Ok(DoctorReadyResult {
                lifecycle_state: "pre_run",
                worktree: None,
                source: path_relative_to_repo(repo_root, &source_path),
                root_stp: path_relative_to_repo(repo_root, &root_stp),
                root_input: path_relative_to_repo(repo_root, &root_bundle_input),
                root_output: path_relative_to_repo(repo_root, &root_bundle_output),
                wt_stp: None,
                wt_input: None,
                wt_output: None,
                status: "PASS",
            });
        }
        bail!("doctor: missing worktree: {}", worktree_path.display());
    }
    if root_indicates_pre_run
        && (!wt_stp.is_file() || !wt_bundle_input.is_file() || !wt_bundle_output.is_file())
    {
        return Ok(DoctorReadyResult {
            lifecycle_state: "pre_run",
            worktree: None,
            source: path_relative_to_repo(repo_root, &source_path),
            root_stp: path_relative_to_repo(repo_root, &root_stp),
            root_input: path_relative_to_repo(repo_root, &root_bundle_input),
            root_output: path_relative_to_repo(repo_root, &root_bundle_output),
            wt_stp: None,
            wt_input: None,
            wt_output: None,
            status: "PASS",
        });
    }
    let wt_branch = run_capture(
        "git",
        &[
            "-C",
            path_str(&worktree_path)?,
            "rev-parse",
            "--abbrev-ref",
            "HEAD",
        ],
    )?;
    if wt_branch.trim() != branch {
        bail!(
            "doctor: worktree branch mismatch for {}",
            worktree_path.display()
        );
    }
    if !wt_stp.is_file() {
        bail!("doctor: missing worktree stp: {}", wt_stp.display());
    }
    validate_bootstrap_stp(&worktree_path, &wt_stp)?;
    validate_authored_prompt_surface("doctor", &wt_stp, PromptSurfaceKind::Stp)?;
    validate_ready_cards(
        repo_root,
        issue_ref.issue_number(),
        issue_ref.slug(),
        wt_branch.trim(),
        &root_bundle_input,
        &root_bundle_output,
    )?;
    validate_ready_cards(
        &worktree_path,
        issue_ref.issue_number(),
        issue_ref.slug(),
        wt_branch.trim(),
        &wt_bundle_input,
        &wt_bundle_output,
    )?;

    Ok(DoctorReadyResult {
        lifecycle_state: "run_bound",
        worktree: Some(path_relative_to_repo(repo_root, &worktree_path)),
        source: path_relative_to_repo(repo_root, &source_path),
        root_stp: path_relative_to_repo(repo_root, &root_stp),
        root_input: path_relative_to_repo(repo_root, &root_bundle_input),
        root_output: path_relative_to_repo(repo_root, &root_bundle_output),
        wt_stp: Some(path_relative_to_repo(repo_root, &wt_stp)),
        wt_input: Some(path_relative_to_repo(repo_root, &wt_bundle_input)),
        wt_output: Some(path_relative_to_repo(repo_root, &wt_bundle_output)),
        status: "PASS",
    })
}

fn issue_is_closed_and_completed(issue: u32, repo: &str) -> Result<bool> {
    let Some(raw) = run_capture_allow_failure(
        "gh",
        &[
            "issue",
            "view",
            &issue.to_string(),
            "-R",
            repo,
            "--json",
            "state,stateReason",
        ],
    )?
    else {
        return Ok(false);
    };
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Ok(false);
    }
    let state: GithubIssueLifecycleState =
        serde_json::from_str(trimmed).context("failed to parse gh issue state json")?;
    Ok(state.state == "CLOSED" && state.state_reason.as_deref() == Some("COMPLETED"))
}

fn reconcile_closed_completed_issue_bundle(
    repo_root: &Path,
    issue_ref: &IssueRef,
    canonical_output: &Path,
) -> Result<()> {
    let bundle_dir = issue_ref.task_bundle_dir_path(repo_root);
    if let Some(parent) = bundle_dir.parent() {
        fs::create_dir_all(parent)?;
    }

    let duplicates = matching_task_bundle_dirs(repo_root, issue_ref)?;
    if !bundle_dir.exists() {
        if let Some(existing) = duplicates.first() {
            fs::rename(existing, &bundle_dir).with_context(|| {
                format!(
                    "doctor: failed to reconcile duplicate task bundle '{}' into canonical '{}'",
                    existing.display(),
                    bundle_dir.display()
                )
            })?;
        } else {
            fs::create_dir_all(&bundle_dir)?;
        }
    }

    if !ensure_nonempty_file_path(canonical_output)? {
        for duplicate in matching_task_bundle_dirs(repo_root, issue_ref)? {
            if duplicate == bundle_dir {
                continue;
            }
            let candidate = duplicate.join("sor.md");
            if ensure_nonempty_file_path(&candidate)? {
                fs::copy(&candidate, canonical_output).with_context(|| {
                    format!(
                        "doctor: failed to restore canonical sor from duplicate '{}'",
                        candidate.display()
                    )
                })?;
                break;
            }
        }

        if !ensure_nonempty_file_path(canonical_output)? {
            let cards_root = resolve_cards_root(repo_root, None);
            let review_output = card_output_path(&cards_root, issue_ref.issue_number());
            if ensure_nonempty_file_path(&review_output)? {
                fs::copy(&review_output, canonical_output).with_context(|| {
                    format!(
                        "doctor: failed to restore canonical sor from review card '{}'",
                        review_output.display()
                    )
                })?;
            }
        }
    }

    if !ensure_nonempty_file_path(canonical_output)? {
        bail!(
            "doctor: closed issue is missing canonical sor: {}",
            canonical_output.display()
        );
    }

    normalize_closed_completed_output_card(canonical_output)?;
    validate_completed_sor(repo_root, canonical_output)?;

    for duplicate in matching_task_bundle_dirs(repo_root, issue_ref)? {
        if duplicate != bundle_dir {
            fs::remove_dir_all(&duplicate).with_context(|| {
                format!(
                    "doctor: failed to remove duplicate closed task bundle '{}'",
                    duplicate.display()
                )
            })?;
        }
    }

    let cards_root = resolve_cards_root(repo_root, None);
    let review_output = card_output_path(&cards_root, issue_ref.issue_number());
    ensure_symlink(&review_output, canonical_output)?;
    Ok(())
}

fn matching_task_bundle_dirs(repo_root: &Path, issue_ref: &IssueRef) -> Result<Vec<PathBuf>> {
    let tasks_dir = repo_root.join(".adl").join(issue_ref.scope()).join("tasks");
    if !tasks_dir.is_dir() {
        return Ok(Vec::new());
    }
    let prefix = format!("{}__", issue_ref.task_issue_id());
    let mut matches = Vec::new();
    for entry in fs::read_dir(&tasks_dir)? {
        let entry = entry?;
        if !entry.file_type()?.is_dir() {
            continue;
        }
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if name.starts_with(&prefix) {
            matches.push(entry.path());
        }
    }
    matches.sort();
    Ok(matches)
}

fn normalize_closed_completed_output_card(path: &Path) -> Result<()> {
    let mut text = fs::read_to_string(path)?;
    replace_field_line_in_text(&mut text, "Status", "DONE");
    replace_first_exact_line(
        &mut text,
        "- Integration state: worktree_only | pr_open | merged",
        "- Integration state: merged",
    );
    replace_first_exact_line(
        &mut text,
        "- Integration state: worktree_only",
        "- Integration state: merged",
    );
    replace_first_exact_line(
        &mut text,
        "- Integration state: pr_open",
        "- Integration state: merged",
    );
    replace_first_exact_line(
        &mut text,
        "- Verification scope: worktree | pr_branch | main_repo",
        "- Verification scope: main_repo",
    );
    replace_first_exact_line(
        &mut text,
        "- Verification scope: worktree",
        "- Verification scope: main_repo",
    );
    replace_first_exact_line(
        &mut text,
        "- Verification scope: pr_branch",
        "- Verification scope: main_repo",
    );
    replace_first_prefixed_line(
        &mut text,
        "- Worktree-only paths remaining:",
        "- Worktree-only paths remaining: none",
    );
    fs::write(path, text)?;
    Ok(())
}

fn replace_field_line_in_text(text: &mut String, label: &str, value: &str) {
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

fn replace_first_exact_line(text: &mut String, from: &str, to: &str) {
    let mut out = Vec::new();
    let mut replaced = false;
    for line in text.lines() {
        if !replaced && line == from {
            out.push(to.to_string());
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

fn replace_first_prefixed_line(text: &mut String, prefix: &str, to: &str) {
    let mut out = Vec::new();
    let mut replaced = false;
    for line in text.lines() {
        if !replaced && line.starts_with(prefix) {
            out.push(to.to_string());
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

fn doctor_mode_name(mode: &DoctorMode) -> &'static str {
    match mode {
        DoctorMode::Full => "full",
        DoctorMode::Ready => "ready",
        DoctorMode::Preflight => "preflight",
    }
}

fn print_doctor_preflight_text(preflight: &DoctorPreflightResult) {
    println!("OPEN_PR_COUNT={}", preflight.open_pr_count);
    for pr in &preflight.open_prs {
        println!(
            "OPEN_PR=#{}|{}|{}|{}",
            pr.number, pr.head_ref_name, pr.state, pr.url
        );
    }
    println!("PREFLIGHT={}", preflight.status);
}

fn print_doctor_ready_text(ready: &DoctorReadyResult) {
    println!("LIFECYCLE_STATE={}", ready.lifecycle_state);
    if let Some(worktree) = &ready.worktree {
        println!("WORKTREE={worktree}");
    }
    println!("SOURCE={}", ready.source);
    println!("ROOT_STP={}", ready.root_stp);
    println!("ROOT_INPUT={}", ready.root_input);
    println!("ROOT_OUTPUT={}", ready.root_output);
    if let Some(wt_stp) = &ready.wt_stp {
        println!("WT_STP={wt_stp}");
    }
    if let Some(wt_input) = &ready.wt_input {
        println!("WT_INPUT={wt_input}");
    }
    if let Some(wt_output) = &ready.wt_output {
        println!("WT_OUTPUT={wt_output}");
    }
    println!("READY={}", ready.status);
}

fn print_json<T: Serialize>(value: &T) -> Result<()> {
    println!(
        "{}",
        serde_json::to_string_pretty(value).context("failed to serialize pr command json")?
    );
    Ok(())
}

fn sync_completed_output_surfaces(
    repo_root: &Path,
    primary_root: &Path,
    issue_ref: &IssueRef,
    completed_output_path: &Path,
) -> Result<PathBuf> {
    let normalized_output_path = if completed_output_path.is_absolute() {
        completed_output_path.to_path_buf()
    } else {
        repo_root.join(completed_output_path)
    };
    let canonical_root_output = issue_ref.task_bundle_output_path(primary_root);
    let copied_to_root =
        !(same_filesystem_target(&normalized_output_path, &canonical_root_output)?);
    if copied_to_root {
        if let Some(parent) = canonical_root_output.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::copy(&normalized_output_path, &canonical_root_output).with_context(|| {
            format!(
                "finish: failed to sync completed output card '{}' to canonical root task bundle '{}'",
                normalized_output_path.display(),
                canonical_root_output.display()
            )
        })?;
        validate_completed_sor(repo_root, &canonical_root_output)?;
    }

    let cards_root = resolve_cards_root(primary_root, None);
    let review_output = card_output_path(&cards_root, issue_ref.issue_number());
    ensure_symlink(&review_output, &canonical_root_output)?;
    Ok(canonical_root_output)
}

fn same_filesystem_target(left: &Path, right: &Path) -> Result<bool> {
    if left == right {
        return Ok(true);
    }
    if left.exists() && right.exists() {
        let left_canonical = fs::canonicalize(left)?;
        let right_canonical = fs::canonicalize(right)?;
        return Ok(left_canonical == right_canonical);
    }
    Ok(false)
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
    let issue_ref = IssueRef::new(issue, version.clone(), slug.clone())?;
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

fn repo_root() -> Result<PathBuf> {
    let current_top = PathBuf::from(run_capture("git", &["rev-parse", "--show-toplevel"])?.trim());
    Ok(current_top)
}

fn primary_checkout_root() -> Result<PathBuf> {
    let current_top = repo_root()?;
    let common_dir = run_capture_allow_failure("git", &["rev-parse", "--git-common-dir"])?;
    let common_dir = common_dir
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(PathBuf::from);
    Ok(resolve_primary_checkout_root(
        &current_top,
        common_dir.as_deref(),
    ))
}

fn path_str(path: &Path) -> Result<&str> {
    path.to_str()
        .ok_or_else(|| anyhow!("path must be valid utf-8: {}", path.display()))
}

fn default_repo(repo_root: &Path) -> Result<String> {
    let remote = run_capture_allow_failure("git", &["remote", "get-url", "origin"])?;
    if let Some(url) = remote {
        if let Some(inferred) = infer_repo_from_remote(&url) {
            return Ok(inferred);
        }
    }

    let gh_repo = run_capture_allow_failure(
        "gh",
        &[
            "repo",
            "view",
            "--json",
            "nameWithOwner",
            "--jq",
            ".nameWithOwner",
        ],
    )?;
    if let Some(repo) = gh_repo {
        let trimmed = repo.trim();
        if !trimmed.is_empty() {
            return Ok(trimmed.to_string());
        }
    }

    let base = repo_root
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| anyhow!("failed to derive local repo name"))?;
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

fn current_branch(repo_root: &Path) -> Result<String> {
    Ok(run_capture(
        "git",
        &[
            "-C",
            path_str(repo_root)?,
            "rev-parse",
            "--abbrev-ref",
            "HEAD",
        ],
    )?
    .trim()
    .to_string())
}

fn ensure_not_on_main_branch(repo_root: &Path) -> Result<()> {
    let branch = current_branch(repo_root)?;
    if branch == "main" {
        bail!("finish: refusing to run on main");
    }
    Ok(())
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

fn has_uncommitted_changes(repo_root: &Path) -> Result<bool> {
    let unstaged =
        run_status_allow_failure("git", &["-C", path_str(repo_root)?, "diff", "--quiet"])?;
    let staged = run_status_allow_failure(
        "git",
        &["-C", path_str(repo_root)?, "diff", "--cached", "--quiet"],
    )?;
    Ok(!(unstaged && staged))
}

fn commits_ahead_of_origin_main(repo_root: &Path) -> Result<usize> {
    let local_origin_main = run_status_allow_failure(
        "git",
        &[
            "-C",
            path_str(repo_root)?,
            "rev-parse",
            "--verify",
            "origin/main",
        ],
    )?;
    if !local_origin_main {
        return Ok(0);
    }
    let count = run_capture(
        "git",
        &[
            "-C",
            path_str(repo_root)?,
            "rev-list",
            "--count",
            "origin/main..HEAD",
        ],
    )?;
    Ok(count.trim().parse::<usize>().unwrap_or(0))
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

fn stage_selected_paths_rust(repo_root: &Path, csv: &str) -> Result<()> {
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
    run_status("git", &args)
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

fn current_pr_url(repo: &str, branch: &str) -> Result<Option<String>> {
    let out = run_capture_allow_failure(
        "gh",
        &[
            "pr", "list", "-R", repo, "--head", branch, "--state", "open", "--json", "url", "--jq",
            ".[0].url",
        ],
    )?;
    Ok(out
        .map(|x| x.trim().to_string())
        .filter(|x| !x.is_empty() && x != "null"))
}

fn unresolved_milestone_pr_wave(
    repo: &str,
    version: &str,
    current_branch: Option<&str>,
) -> Result<Vec<OpenPullRequest>> {
    let out = run_capture_allow_failure(
        "gh",
        &[
            "pr",
            "list",
            "-R",
            repo,
            "--state",
            "open",
            "--json",
            "number,title,url,headRefName,baseRefName,isDraft",
        ],
    )?
    .unwrap_or_else(|| "[]".to_string());
    let prs: Vec<OpenPullRequest> =
        serde_json::from_str(&out).with_context(|| "failed to parse gh pr list json")?;
    let version_tag = format!("[{version}]");
    Ok(prs
        .into_iter()
        .filter(|pr| pr.base_ref_name == "main")
        .filter(|pr| pr.title.contains(&version_tag))
        .filter(|pr| {
            current_branch
                .map(|branch| pr.head_ref_name != branch)
                .unwrap_or(true)
        })
        .collect())
}

fn format_open_pr_wave(prs: &[OpenPullRequest]) -> String {
    prs.iter()
        .map(|pr| {
            format!(
                "- #{} [{}] {} ({})",
                pr.number,
                if pr.is_draft { "draft" } else { "ready" },
                pr.title,
                pr.url
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn pr_has_closing_linkage(repo: &str, pr_ref: &str, issue: u32) -> Result<bool> {
    let linked = run_capture_allow_failure(
        "gh",
        &[
            "pr",
            "view",
            "-R",
            repo,
            pr_ref,
            "--json",
            "closingIssuesReferences",
            "--jq",
            ".closingIssuesReferences[]?.number",
        ],
    )?;
    if linked
        .as_deref()
        .unwrap_or_default()
        .lines()
        .any(|line| line.trim() == issue.to_string())
    {
        return Ok(true);
    }
    let body = run_capture_allow_failure(
        "gh",
        &[
            "pr", "view", "-R", repo, pr_ref, "--json", "body", "--jq", ".body",
        ],
    )?
    .unwrap_or_default();
    Ok(body.contains(&format!("Closes #{issue}")))
}

fn ensure_pr_closing_linkage(repo: &str, pr_ref: &str, issue: u32, no_close: bool) -> Result<()> {
    if no_close {
        return Ok(());
    }
    if !pr_has_closing_linkage(repo, pr_ref, issue)? {
        bail!(
            "finish: PR is missing GitHub closing linkage for issue #{}",
            issue
        );
    }
    Ok(())
}

fn issue_version(issue: u32, repo: &str) -> Result<Option<String>> {
    let labels = run_capture_allow_failure(
        "gh",
        &[
            "issue",
            "view",
            &issue.to_string(),
            "-R",
            repo,
            "--json",
            "labels",
            "-q",
            ".labels[].name",
        ],
    )?;
    if let Some(labels) = labels {
        for line in labels.lines() {
            if let Some(version) = line.strip_prefix("version:") {
                return Ok(Some(version.trim().to_string()));
            }
        }
    }

    let title = gh_issue_title(issue, repo)?;
    Ok(title.and_then(|title| version_from_title(&title)))
}

fn gh_issue_create(repo: &str, title: &str, body: &str, labels_csv: &str) -> Result<String> {
    let mut cmd = Command::new("gh");
    cmd.arg("issue")
        .arg("create")
        .arg("-R")
        .arg(repo)
        .arg("--title")
        .arg(title)
        .arg("--body")
        .arg(body);
    for label in labels_csv
        .split(',')
        .map(str::trim)
        .filter(|label| !label.is_empty())
    {
        cmd.arg("--label").arg(label);
    }
    let output = cmd
        .output()
        .with_context(|| "failed to spawn gh issue create")?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!(
            "init: gh issue create failed{}",
            if stderr.trim().is_empty() {
                String::new()
            } else {
                format!(": {}", stderr.trim())
            }
        );
    }
    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if stdout.is_empty() {
        bail!("init: gh issue create returned empty output");
    }
    Ok(stdout)
}

fn issue_label_names(issue: u32, repo: &str) -> Result<Vec<String>> {
    let labels = run_capture_allow_failure(
        "gh",
        &[
            "issue",
            "view",
            &issue.to_string(),
            "-R",
            repo,
            "--json",
            "labels",
            "-q",
            ".labels[].name",
        ],
    )?
    .unwrap_or_default();
    Ok(labels
        .lines()
        .map(str::trim)
        .filter(|label| !label.is_empty())
        .map(ToString::to_string)
        .collect())
}

fn ensure_issue_labels(repo: &str, issue: u32, labels_csv: &str) -> Result<()> {
    let expected: BTreeSet<String> = labels_csv
        .split(',')
        .map(str::trim)
        .filter(|label| !label.is_empty())
        .map(ToString::to_string)
        .collect();
    if expected.is_empty() {
        bail!("create: expected at least one label for tracked issue creation");
    }

    let actual: BTreeSet<String> = issue_label_names(issue, repo)?.into_iter().collect();
    let missing: Vec<String> = expected.difference(&actual).cloned().collect();
    if !missing.is_empty() {
        bail!(
            "create: issue #{} is missing expected labels after gh issue create: {}",
            issue,
            missing.join(", ")
        );
    }
    Ok(())
}

fn gh_issue_edit_body(repo: &str, issue: u32, body: &str) -> Result<()> {
    let body_file = write_temp_markdown("issue_body", body)?;
    run_status(
        "gh",
        &[
            "issue",
            "edit",
            &issue.to_string(),
            "-R",
            repo,
            "--body-file",
            path_str(&body_file)?,
        ],
    )
    .with_context(|| format!("create: gh issue edit failed for issue #{issue}"))
}

fn infer_required_outcome_type_for_create(labels_csv: &str, title: &str) -> &'static str {
    infer_required_outcome_type(labels_csv, title)
}

fn gh_issue_title(issue: u32, repo: &str) -> Result<Option<String>> {
    let out = run_capture_allow_failure(
        "gh",
        &[
            "issue",
            "view",
            &issue.to_string(),
            "-R",
            repo,
            "--json",
            "title",
            "-q",
            ".title",
        ],
    )?;
    Ok(out
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty()))
}

fn fetch_origin_main_with_fallback() -> Result<()> {
    eprintln!("• Fetching origin/main…");
    let output = Command::new("git")
        .args(["fetch", "origin", "main"])
        .output()
        .with_context(|| "failed to spawn git fetch origin main")?;
    if output.status.success() {
        return Ok(());
    }
    if run_status_allow_failure("git", &["rev-parse", "--verify", "--quiet", "origin/main"])? {
        eprintln!("• Warning: start: fetch origin main failed; reusing existing local origin/main");
        let stderr = String::from_utf8_lossy(&output.stderr);
        if !stderr.trim().is_empty() {
            eprintln!("{stderr}");
        }
        return Ok(());
    }
    bail!("start: fetch origin main failed and origin/main is unavailable locally");
}

fn ensure_git_metadata_writable() -> Result<()> {
    let git_dir = run_capture("git", &["rev-parse", "--git-common-dir"])?;
    let git_dir = git_dir.trim();
    let probe_dir = Path::new(git_dir).join(format!(
        "adl-git-write-probe-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos()
    ));
    match fs::create_dir(&probe_dir) {
        Ok(()) => {
            let _ = fs::remove_dir(&probe_dir);
            Ok(())
        }
        Err(err) => bail!(
            "start: git metadata directory '{}' is not writable, so branch/worktree creation cannot proceed. Remediation: restore write access to git metadata before rerunning. ({err})",
            git_dir
        ),
    }
}

fn ensure_local_branch_exists(branch: &str) -> Result<()> {
    if run_status_allow_failure(
        "git",
        &[
            "show-ref",
            "--verify",
            "--quiet",
            &format!("refs/heads/{branch}"),
        ],
    )? {
        eprintln!("• Local branch exists; reusing: {branch}");
        return Ok(());
    }
    if run_status_allow_failure(
        "git",
        &["ls-remote", "--exit-code", "--heads", "origin", branch],
    )? {
        eprintln!("• Branch exists on origin; creating local tracking branch…");
        run_status(
            "git",
            &["branch", "--track", branch, &format!("origin/{branch}")],
        )?;
        return Ok(());
    }
    eprintln!("• Creating local branch from origin/main…");
    run_status("git", &["branch", branch, "origin/main"])?;
    Ok(())
}

fn branch_checked_out_worktree_path(branch: &str) -> Result<Option<PathBuf>> {
    let out = run_capture_allow_failure("git", &["worktree", "list", "--porcelain"])?;
    let Some(out) = out else { return Ok(None) };
    let mut current_worktree: Option<PathBuf> = None;
    for line in out.lines() {
        if let Some(path) = line.strip_prefix("worktree ") {
            current_worktree = Some(PathBuf::from(path.trim()));
        } else if let Some(head_branch) = line.strip_prefix("branch refs/heads/") {
            if head_branch.trim() == branch {
                return Ok(current_worktree);
            }
        }
    }
    Ok(None)
}

fn ensure_worktree_for_branch(worktree_path: &Path, branch: &str) -> Result<()> {
    if let Some(existing) = branch_checked_out_worktree_path(branch)? {
        if existing == worktree_path {
            eprintln!(
                "• Reusing existing worktree for branch: {}",
                worktree_path.display()
            );
            return Ok(());
        }
        bail!(
            "start: branch '{}' is already checked out in worktree '{}'. Remediation: run commands there or remove it with 'git worktree remove \"{}\"'.",
            branch,
            existing.display(),
            existing.display()
        );
    }
    if !worktree_path.exists() {
        eprintln!("• Creating worktree: {}", worktree_path.display());
        if let Some(parent) = worktree_path.parent() {
            fs::create_dir_all(parent)?;
        }
        run_status(
            "git",
            &["worktree", "add", path_str(worktree_path)?, branch],
        )?;
        return Ok(());
    }
    eprintln!(
        "• Reusing existing worktree path: {}",
        worktree_path.display()
    );
    Ok(())
}

fn run_capture(program: &str, args: &[&str]) -> Result<String> {
    let output = Command::new(program)
        .args(args)
        .output()
        .with_context(|| format!("failed to spawn '{program}'"))?;
    if !output.status.success() {
        bail!(
            "{program} failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
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

fn run_status_allow_failure(program: &str, args: &[&str]) -> Result<bool> {
    let status = Command::new(program)
        .args(args)
        .status()
        .with_context(|| format!("failed to spawn '{program}'"))?;
    Ok(status.success())
}

#[cfg(test)]
#[path = "tests/pr_cmd_inline/mod.rs"]
mod tests;
