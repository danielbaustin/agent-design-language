use anyhow::{anyhow, bail, Context, Result};
use serde::Deserialize;
use std::fs;
#[cfg(unix)]
use std::os::unix::fs as unix_fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use super::pr_cmd_args::{
    parse_create_args, parse_finish_args, parse_init_args, parse_preflight_args, parse_ready_args,
    parse_start_args,
};
#[cfg(test)]
use super::pr_cmd_prompt::load_issue_prompt;
use super::pr_cmd_prompt::{
    infer_required_outcome_type, infer_wp_from_title, normalize_labels_csv,
    parse_issue_number_from_url, render_generated_issue_body, render_generated_issue_prompt,
    resolve_issue_body, resolve_issue_prompt_path, resolve_issue_scope_and_slug_from_local_state,
    validate_issue_prompt_exists, version_from_labels_csv, version_from_title,
};
use ::adl::control_plane::{
    card_input_path, card_output_path, resolve_cards_root, resolve_primary_checkout_root,
    sanitize_slug, IssueRef,
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

pub(crate) fn real_pr(args: &[String]) -> Result<()> {
    let Some(subcommand) = args.first().map(|s| s.as_str()) else {
        bail!("pr requires a subcommand: create | init | start | ready | preflight | finish");
    };

    match subcommand {
        "create" => real_pr_create(&args[1..]),
        "init" => real_pr_init(&args[1..]),
        "start" => real_pr_start(&args[1..]),
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
    let issue_url = gh_issue_create(&repo, &title, &create_body, &normalized_labels)?;
    let issue = parse_issue_number_from_url(&issue_url)?;
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
    if create_body != final_body {
        gh_issue_edit_body(&repo, issue, &final_body)?;
    }

    println!("• Created:");
    println!("  ISSUE_URL  {issue_url}");
    println!("  ISSUE_NUM  {issue}");
    println!("  VERSION    {version}");
    println!("  SLUG       {slug}");
    println!(
        "  SOURCE     {}",
        path_relative_to_repo(&repo_root, &source_path)
    );
    println!("  NEXT       adl/tools/pr.sh init {issue} --slug {slug} --version {version}");
    println!("  STATE      ISSUE_CREATED");
    eprintln!("• Done.");
    Ok(())
}

fn real_pr_start(args: &[String]) -> Result<()> {
    let parsed = parse_start_args(args)?;
    let repo_root = repo_root()?;
    let repo = default_repo(&repo_root)?;

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
    ensure_primary_checkout_on_main(&repo_root)?;

    let source_path =
        ensure_source_issue_prompt(&repo_root, &repo, &issue_ref, &title, None, &version)?;
    validate_issue_prompt_exists(&source_path)?;
    validate_bootstrap_stp(&repo_root, &source_path)?;
    validate_authored_prompt_surface("start", &source_path, PromptSurfaceKind::IssuePrompt)?;

    let root_stp = ensure_task_bundle_stp(&repo_root, &issue_ref, &source_path)?;
    let worktree_source = ensure_local_issue_prompt_copy(&worktree_path, &issue_ref, &source_path)?;
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
    println!("  READ   {}", worktree_paths.0.display());
    println!("  WRITE  {}", worktree_paths.1.display());
    println!("  ROOT_STP    {}", root_stp.display());
    println!("  ROOT_READ   {}", root_paths.0.display());
    println!("  ROOT_WRITE  {}", root_paths.1.display());
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
    let parsed = parse_ready_args(args)?;
    let repo_root = primary_checkout_root()?;
    let repo = default_repo(&repo_root)?;

    let (version, slug) =
        if let (Some(version), Some(slug)) = (parsed.version.clone(), parsed.slug.clone()) {
            (version, slug)
        } else {
            let inferred = resolve_issue_scope_and_slug_from_local_state(&repo_root, parsed.issue)?;
            (
                parsed
                    .version
                    .clone()
                    .or(inferred.as_ref().map(|x| x.0.clone()))
                    .unwrap_or_else(|| DEFAULT_VERSION.to_string()),
                parsed
                    .slug
                    .clone()
                    .or(inferred.map(|x| x.1))
                    .ok_or_else(|| {
                        anyhow!("ready: could not infer slug; pass --slug or run start first")
                    })?,
            )
        };

    let issue_ref = IssueRef::new(parsed.issue, version.clone(), slug.clone())?;
    let branch = issue_ref.branch_name("codex");
    let managed_root = std::env::var_os("ADL_WORKTREE_ROOT").map(PathBuf::from);
    let worktree_path = issue_ref.default_worktree_path(&repo_root, managed_root.as_deref());
    let source_path = resolve_issue_prompt_path(&repo_root, &issue_ref)?;
    let root_stp = issue_ref.task_bundle_stp_path(&repo_root);
    let wt_stp = issue_ref.task_bundle_stp_path(&worktree_path);

    let root_bundle_input = issue_ref.task_bundle_input_path(&repo_root);
    let root_bundle_output = issue_ref.task_bundle_output_path(&repo_root);
    let wt_bundle_input = issue_ref.task_bundle_input_path(&worktree_path);
    let wt_bundle_output = issue_ref.task_bundle_output_path(&worktree_path);

    validate_issue_prompt_exists(&source_path)?;
    validate_bootstrap_stp(&repo_root, &source_path)?;
    validate_authored_prompt_surface("ready", &source_path, PromptSurfaceKind::IssuePrompt)?;
    if !root_stp.is_file() {
        bail!("ready: missing root stp: {}", root_stp.display());
    }
    validate_bootstrap_stp(&repo_root, &root_stp)?;
    validate_authored_prompt_surface("ready", &root_stp, PromptSurfaceKind::Stp)?;
    if !worktree_path.is_dir() {
        bail!("ready: missing worktree: {}", worktree_path.display());
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
            "ready: worktree branch mismatch for {}",
            worktree_path.display()
        );
    }
    if !wt_stp.is_file() {
        bail!("ready: missing worktree stp: {}", wt_stp.display());
    }
    validate_bootstrap_stp(&worktree_path, &wt_stp)?;
    validate_authored_prompt_surface("ready", &wt_stp, PromptSurfaceKind::Stp)?;
    validate_ready_cards(
        &repo_root,
        parsed.issue,
        wt_branch.trim(),
        &root_bundle_input,
        &root_bundle_output,
    )?;
    validate_ready_cards(
        &worktree_path,
        parsed.issue,
        wt_branch.trim(),
        &wt_bundle_input,
        &wt_bundle_output,
    )?;

    println!("ISSUE={}", parsed.issue);
    println!("VERSION={version}");
    println!("SLUG={slug}");
    println!("BRANCH={branch}");
    println!("WORKTREE={}", worktree_path.display());
    println!("SOURCE={}", path_relative_to_repo(&repo_root, &source_path));
    println!("ROOT_STP={}", path_relative_to_repo(&repo_root, &root_stp));
    println!(
        "ROOT_INPUT={}",
        path_relative_to_repo(&repo_root, &root_bundle_input)
    );
    println!(
        "ROOT_OUTPUT={}",
        path_relative_to_repo(&repo_root, &root_bundle_output)
    );
    println!("WT_STP={}", path_relative_to_repo(&repo_root, &wt_stp));
    println!(
        "WT_INPUT={}",
        path_relative_to_repo(&repo_root, &wt_bundle_input)
    );
    println!(
        "WT_OUTPUT={}",
        path_relative_to_repo(&repo_root, &wt_bundle_output)
    );
    println!("READY=PASS");
    let _ = repo; // keep parity with init/create remote-based inference
    Ok(())
}

fn real_pr_preflight(args: &[String]) -> Result<()> {
    let parsed = parse_preflight_args(args)?;
    let repo_root = repo_root()?;
    let repo = default_repo(&repo_root)?;

    let (version, slug) =
        if let (Some(version), Some(slug)) = (parsed.version.clone(), parsed.slug.clone()) {
            (version, slug)
        } else {
            let inferred = resolve_issue_scope_and_slug_from_local_state(&repo_root, parsed.issue)?;
            (
                parsed
                    .version
                    .clone()
                    .or(inferred.as_ref().map(|x| x.0.clone()))
                    .unwrap_or_else(|| DEFAULT_VERSION.to_string()),
                parsed
                    .slug
                    .clone()
                    .or(inferred.map(|x| x.1))
                    .unwrap_or_else(|| format!("issue-{}", parsed.issue)),
            )
        };

    let issue_ref = IssueRef::new(parsed.issue, version.clone(), slug)?;
    let branch = issue_ref.branch_name("codex");
    let unresolved = unresolved_milestone_pr_wave(&repo, &version, Some(&branch))?;

    println!("ISSUE={}", parsed.issue);
    println!("VERSION={version}");
    println!("BRANCH={branch}");
    println!("OPEN_PR_COUNT={}", unresolved.len());
    for pr in &unresolved {
        println!(
            "OPEN_PR=#{}|{}|{}|{}",
            pr.number,
            pr.head_ref_name,
            if pr.is_draft { "draft" } else { "ready" },
            pr.url
        );
    }
    if unresolved.is_empty() {
        println!("PREFLIGHT=PASS");
    } else {
        println!("PREFLIGHT=BLOCK");
    }
    Ok(())
}

fn real_pr_finish(args: &[String]) -> Result<()> {
    let parsed = parse_finish_args(args)?;
    let repo_root = repo_root()?;
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
    let source_path = resolve_issue_prompt_path(&repo_root, &issue_ref)?;
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
    let source_path =
        ensure_source_issue_prompt(&repo_root, &repo, &issue_ref, &title, None, &version)?;
    validate_issue_prompt_exists(&source_path)?;

    let stp_path = issue_ref.task_bundle_stp_path(&repo_root);
    let bundle_dir = issue_ref.task_bundle_dir_path(&repo_root);
    let init_branch = issue_ref.branch_name("codex");
    eprintln!("• Initializing task bundle: {}", bundle_dir.display());
    if !stp_path.is_file() {
        if let Some(parent) = stp_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::copy(&source_path, &stp_path).with_context(|| {
            format!(
                "failed to seed task-bundle stp from '{}' to '{}'",
                source_path.display(),
                stp_path.display()
            )
        })?;
    } else {
        eprintln!("• STP already exists: {}", stp_path.display());
    }
    let (bundle_input, bundle_output) =
        ensure_bootstrap_cards(&repo_root, &issue_ref, &title, &init_branch, &source_path)?;

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

fn write_source_issue_prompt(
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

fn render_issue_prompt_from_body(
    issue: u32,
    slug: &str,
    title: &str,
    labels_csv: &str,
    _issue_url: &str,
    body: &str,
) -> String {
    let wp = infer_wp_from_title(title);
    let outcome_type = infer_required_outcome_type_for_create(labels_csv, title);
    let label_lines = labels_csv
        .split(',')
        .map(str::trim)
        .filter(|label| !label.is_empty())
        .map(|label| format!("  - \"{label}\""))
        .collect::<Vec<_>>()
        .join("\n");
    format!(
        "---\nissue_card_schema: adl.issue.v1\nwp: \"{wp}\"\nslug: \"{slug}\"\ntitle: \"{title}\"\nlabels:\n{label_lines}\nissue_number: {issue}\nstatus: \"draft\"\naction: \"edit\"\ndepends_on: []\nmilestone_sprint: \"Pending sprint assignment\"\nrequired_outcome_type:\n  - \"{outcome_type}\"\nrepo_inputs: []\ncanonical_files: []\ndemo_required: false\ndemo_names: []\nissue_graph_notes:\n  - \"Bootstrap-generated from GitHub issue metadata because no canonical local issue prompt existed yet.\"\npr_start:\n  enabled: true\n  slug: \"{slug}\"\n---\n\n{body}\n"
    )
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

fn ensure_primary_checkout_on_main(repo_root: &Path) -> Result<()> {
    let current = run_capture(
        "git",
        &[
            "-C",
            path_str(repo_root)?,
            "rev-parse",
            "--abbrev-ref",
            "HEAD",
        ],
    )?;
    let current = current.trim().to_string();
    let dirty = !run_status_allow_failure("git", &["-C", path_str(repo_root)?, "diff", "--quiet"])?
        || !run_status_allow_failure(
            "git",
            &["-C", path_str(repo_root)?, "diff", "--cached", "--quiet"],
        )?
        || !run_capture(
            "git",
            &["-C", path_str(repo_root)?, "status", "--porcelain"],
        )
        .unwrap_or_default()
        .trim()
        .is_empty();
    if current != "main" && dirty {
        bail!(
            "start: primary checkout ({}) is on '{}' with local changes. Remediation: commit/stash there, switch to main, then rerun.",
            repo_root.display(),
            current
        );
    }
    if current != "main" {
        run_status("git", &["-C", path_str(repo_root)?, "switch", "main"])?;
    }
    Ok(())
}

fn ensure_task_bundle_stp(
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

fn ensure_local_issue_prompt_copy(
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

fn ensure_bootstrap_cards(
    root: &Path,
    issue_ref: &IssueRef,
    title: &str,
    branch: &str,
    source_path: &Path,
) -> Result<(PathBuf, PathBuf)> {
    let bundle_input = issue_ref.task_bundle_input_path(root);
    let bundle_output = issue_ref.task_bundle_output_path(root);
    if let Some(parent) = bundle_input.parent() {
        fs::create_dir_all(parent)?;
    }
    if !bundle_input.is_file() {
        write_input_card(
            root,
            &bundle_input,
            issue_ref,
            title,
            branch,
            source_path,
            &bundle_output,
        )?;
    }
    if !bundle_output.is_file() {
        write_output_card(root, &bundle_output, issue_ref, title, branch)?;
    }

    let cards_root = resolve_cards_root(root, None);
    let compat_input = card_input_path(&cards_root, issue_ref.issue_number());
    let compat_output = card_output_path(&cards_root, issue_ref.issue_number());
    ensure_symlink(&compat_input, &bundle_input)?;
    ensure_symlink(&compat_output, &bundle_output)?;

    validate_bootstrap_cards(
        root,
        issue_ref.issue_number(),
        branch,
        &bundle_input,
        &bundle_output,
    )?;
    Ok((bundle_input, bundle_output))
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
    fs::write(path, text)?;
    Ok(())
}

fn write_output_card(
    repo_root: &Path,
    path: &Path,
    issue_ref: &IssueRef,
    title: &str,
    branch: &str,
) -> Result<()> {
    let mut text =
        fs::read_to_string(repo_root.join("adl/templates/cards/output_card_template.md"))?;
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

fn ensure_symlink(link_path: &Path, target: &Path) -> Result<()> {
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

fn validate_bootstrap_stp(repo_root: &Path, path: &Path) -> Result<()> {
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

fn validate_bootstrap_cards(
    repo_root: &Path,
    issue: u32,
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
    Ok(())
}

fn validate_ready_cards(
    _repo_root: &Path,
    issue: u32,
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
    validate_authored_prompt_surface("ready", input_path, PromptSurfaceKind::Sip)?;
    Ok(())
}

#[derive(Clone, Copy)]
enum PromptSurfaceKind {
    IssuePrompt,
    Stp,
    Sip,
}

impl PromptSurfaceKind {
    fn label(self) -> &'static str {
        match self {
            Self::IssuePrompt => "issue body/source prompt",
            Self::Stp => "stp",
            Self::Sip => "sip",
        }
    }
}

fn validate_authored_prompt_surface(
    phase: &str,
    path: &Path,
    kind: PromptSurfaceKind,
) -> Result<()> {
    let text = fs::read_to_string(path)?;
    if let Some(reason) = bootstrap_stub_reason(&text, kind) {
        bail!(
            "{phase}: {} is still bootstrap stub content ({reason}): {}",
            kind.label(),
            path.display()
        );
    }
    Ok(())
}

fn bootstrap_stub_reason(text: &str, kind: PromptSurfaceKind) -> Option<&'static str> {
    let normalized = text.replace("\r\n", "\n");
    if normalized.contains("## Goal\n-\n\n## Acceptance Criteria\n-") {
        return Some("placeholder goal/acceptance criteria stub");
    }
    if !section_has_authored_content(&normalized, "## Goal") {
        return Some("empty Goal section");
    }
    if !section_has_authored_content(&normalized, "## Acceptance Criteria") {
        return Some("empty Acceptance Criteria section");
    }

    match kind {
        PromptSurfaceKind::IssuePrompt | PromptSurfaceKind::Stp => {
            let issue_prompt_markers = [
                "Bootstrap-generated local source prompt for issue #",
                "Translate the GitHub issue into the canonical local STP/task-bundle flow and refine this prompt before execution as needed.",
                "Generated by `pr.sh` bootstrap fallback.",
                "This prompt was generated automatically because the canonical local issue prompt was missing.",
            ];
            if issue_prompt_markers
                .iter()
                .any(|marker| normalized.contains(marker))
            {
                return Some("bootstrap-generated issue prompt template text");
            }
        }
        PromptSurfaceKind::Sip => {
            let sip_markers = [
                "- State whether this issue must ship code, docs, tests, demo artifacts, or a combination.",
                "- Likely files, modules, docs, commands, schemas, or artifacts to modify or validate",
                "- Required commands:",
                "- Required tests:",
                "- Required artifacts / traces:",
                "- Required reviewer or demo checks:",
                "- Required demo(s):",
                "- Required proof surface(s):",
                "- If no demo is required, say why:",
                "- Determinism requirements:",
                "- Security / privacy requirements:",
                "- Resource limits (time/CPU/memory/network):",
            ];
            if normalized
                .lines()
                .map(str::trim)
                .any(|line| sip_markers.contains(&line))
            {
                return Some("unrefined SIP template guidance");
            }
        }
    }
    None
}

fn section_has_authored_content(text: &str, header: &str) -> bool {
    let mut in_section = false;
    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed == header {
            in_section = true;
            continue;
        }
        if in_section {
            if trimmed.starts_with("## ") {
                return false;
            }
            if !trimmed.is_empty() {
                return true;
            }
        }
    }
    false
}

fn field_line_value(path: &Path, label: &str) -> Result<String> {
    let prefix = format!("{label}:");
    let text = fs::read_to_string(path)?;
    for line in text.lines() {
        if let Some(rest) = line.strip_prefix(&prefix) {
            return Ok(rest.trim().to_string());
        }
    }
    Ok(String::new())
}

fn branch_matches_started_state(recorded: &str, actual_branch: &str) -> bool {
    let recorded = recorded.trim();
    if recorded == actual_branch {
        return true;
    }
    recorded.starts_with("TBD (run pr.sh start ")
}

fn ensure_source_issue_prompt(
    repo_root: &Path,
    repo: &str,
    issue_ref: &IssueRef,
    title: &str,
    labels_csv: Option<&str>,
    version: &str,
) -> Result<PathBuf> {
    let source_path = issue_ref.issue_prompt_path(repo_root);
    if source_path.is_file() {
        return Ok(source_path);
    }

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
            DEFAULT_NEW_LABELS.to_string()
        } else {
            fetched
        };
        normalize_labels_csv(&baseline, version)
    };

    let issue_url = format!(
        "https://github.com/{repo}/issues/{}",
        issue_ref.issue_number()
    );
    let content = render_generated_issue_prompt(
        issue_ref.issue_number(),
        issue_ref.slug(),
        title,
        &labels_csv,
        &issue_url,
    );
    if let Some(parent) = source_path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&source_path, content)?;
    Ok(source_path)
}

fn path_relative_to_repo(repo_root: &Path, path: &Path) -> String {
    path.strip_prefix(repo_root)
        .map(|relative| relative.display().to_string())
        .unwrap_or_else(|_| path.display().to_string())
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
mod tests {
    use super::*;
    use std::env;
    use std::sync::{Mutex, OnceLock};
    use std::time::{SystemTime, UNIX_EPOCH};

    fn unique_temp_dir(label: &str) -> PathBuf {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time")
            .as_nanos();
        let dir = std::env::temp_dir().join(format!("{label}-{now}-{}", std::process::id()));
        fs::create_dir_all(&dir).expect("create temp dir");
        dir
    }

    fn env_lock() -> &'static Mutex<()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(()))
    }

    fn write_executable(path: &Path, content: &str) {
        fs::write(path, content).expect("write executable");
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(path).expect("metadata").permissions();
            perms.set_mode(0o755);
            fs::set_permissions(path, perms).expect("chmod");
        }
    }

    fn init_git_repo(dir: &Path) {
        assert!(Command::new("git")
            .arg("init")
            .arg("-q")
            .current_dir(dir)
            .status()
            .expect("git init")
            .success());
        assert!(Command::new("git")
            .args([
                "remote",
                "add",
                "origin",
                "https://github.com/danielbaustin/agent-design-language.git"
            ])
            .current_dir(dir)
            .status()
            .expect("git remote add")
            .success());
    }

    fn copy_bootstrap_support_files(repo: &Path) {
        let workspace_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("workspace root")
            .to_path_buf();
        let tools_dir = repo.join("adl/tools");
        let templates_dir = repo.join("adl/templates/cards");
        let schemas_dir = repo.join("adl/schemas");
        fs::create_dir_all(&tools_dir).expect("tools dir");
        fs::create_dir_all(&templates_dir).expect("templates dir");
        fs::create_dir_all(&schemas_dir).expect("schemas dir");

        let files = [
            (
                workspace_root.join("adl/tools/card_paths.sh"),
                tools_dir.join("card_paths.sh"),
            ),
            (
                workspace_root.join("adl/tools/validate_structured_prompt.sh"),
                tools_dir.join("validate_structured_prompt.sh"),
            ),
            (
                workspace_root.join("adl/tools/lint_prompt_spec.sh"),
                tools_dir.join("lint_prompt_spec.sh"),
            ),
            (
                workspace_root.join("adl/templates/cards/input_card_template.md"),
                templates_dir.join("input_card_template.md"),
            ),
            (
                workspace_root.join("adl/templates/cards/output_card_template.md"),
                templates_dir.join("output_card_template.md"),
            ),
            (
                workspace_root.join("adl/schemas/structured_task_prompt.contract.yaml"),
                schemas_dir.join("structured_task_prompt.contract.yaml"),
            ),
            (
                workspace_root.join("adl/schemas/structured_implementation_prompt.contract.yaml"),
                schemas_dir.join("structured_implementation_prompt.contract.yaml"),
            ),
            (
                workspace_root.join("adl/schemas/structured_output_record.contract.yaml"),
                schemas_dir.join("structured_output_record.contract.yaml"),
            ),
        ];

        for (src, dst) in files {
            fs::copy(src, &dst).expect("copy support file");
            #[cfg(unix)]
            if dst.extension().is_none() || dst.to_string_lossy().ends_with(".sh") {
                use std::os::unix::fs::PermissionsExt;
                let mut perms = fs::metadata(&dst).expect("metadata").permissions();
                perms.set_mode(0o755);
                fs::set_permissions(&dst, perms).expect("chmod");
            }
        }
    }

    fn write_authored_issue_prompt(repo: &Path, issue_ref: &IssueRef, title: &str) {
        let path = issue_ref.issue_prompt_path(repo);
        fs::create_dir_all(path.parent().expect("issue prompt parent")).expect("create body dir");
        let content = format!(
            "---\nissue_card_schema: adl.issue.v1\nwp: \"unassigned\"\nslug: \"{slug}\"\ntitle: \"{title}\"\nlabels:\n  - \"track:roadmap\"\n  - \"area:tools\"\n  - \"type:task\"\n  - \"version:v0.86\"\nissue_number: {issue}\nstatus: \"active\"\naction: \"edit\"\ndepends_on: []\nmilestone_sprint: \"unplanned\"\nrequired_outcome_type:\n  - \"code\"\nrepo_inputs:\n  - \"https://github.com/example/repo/issues/{issue}\"\ncanonical_files: []\ndemo_required: false\ndemo_names: []\nissue_graph_notes:\n  - \"Authored for test coverage.\"\npr_start:\n  enabled: true\n  slug: \"{slug}\"\n---\n\n# {title}\n\n## Summary\n\nAuthored prompt for lifecycle validation tests.\n\n## Goal\n\nMake the issue prompt authored enough that lifecycle commands should accept it.\n\n## Required Outcome\n\nThis test issue ships code only.\n\n## Deliverables\n\n- authored issue prompt content\n\n## Acceptance Criteria\n\n- lifecycle validation accepts this source prompt\n\n## Repo Inputs\n\n- https://github.com/example/repo/issues/{issue}\n\n## Dependencies\n\n- none\n\n## Demo Expectations\n\n- none\n\n## Non-goals\n\n- bootstrap placeholder content\n\n## Issue-Graph Notes\n\n- test fixture\n\n## Notes\n\n- generated inside unit tests\n\n## Tooling Notes\n\n- authored fixture, not bootstrap fallback\n",
            slug = issue_ref.slug(),
            title = title,
            issue = issue_ref.issue_number()
        );
        fs::write(path, content).expect("write authored prompt");
    }

    fn write_authored_sip(
        path: &Path,
        issue_ref: &IssueRef,
        title: &str,
        branch: &str,
        source_prompt: &Path,
        repo_root: &Path,
    ) {
        let source_rel = path_relative_to_repo(repo_root, source_prompt);
        let content = format!(
            "# ADL Input Card\n\nTask ID: {task_id}\nRun ID: {task_id}\nVersion: v0.86\nTitle: {title}\nBranch: {branch}\n\nContext:\n- Issue: https://github.com/example/repo/issues/{issue}\n- PR: none\n- Source Issue Prompt: {source_rel}\n- Docs: none\n- Other: none\n\n## Agent Execution Rules\n- Do not run `pr start`; the branch and worktree already exist.\n- Only modify files required for the issue.\n\n## Prompt Spec\n```yaml\nprompt_schema: adl.v1\nactor:\n  role: execution_agent\n  name: codex\nmodel:\n  id: gpt-5-codex\n  determinism_mode: stable\ninputs:\n  sections:\n    - goal\n    - required_outcome\n    - acceptance_criteria\n    - inputs\n    - target_files_surfaces\n    - validation_plan\n    - demo_proof_requirements\n    - constraints_policies\n    - system_invariants\n    - reviewer_checklist\n    - non_goals_out_of_scope\n    - notes_risks\n    - instructions_to_agent\noutputs:\n  output_card: .adl/v0.86/tasks/{bundle}/sor.md\n  summary_style: concise_structured\nconstraints:\n  include_system_invariants: true\n  include_reviewer_checklist: true\n  disallow_secrets: true\n  disallow_absolute_host_paths: true\nautomation_hints:\n  source_issue_prompt_required: true\n  target_files_surfaces_recommended: true\n  validation_plan_required: true\n  required_outcome_type_supported: true\nreview_surfaces:\n  - card_review_checklist.v1\n  - card_review_output.v1\n  - card_reviewer_gpt.v1.1\n```\n\nExecution:\n- Agent: codex\n- Provider: openai\n- Tools allowed: git, cargo\n- Sandbox / approvals: workspace-write\n- Source issue-prompt slug: {slug}\n- Required outcome type: code\n- Demo required: false\n\n## Goal\n\nBlock lifecycle execution when prompts are still bootstrap stubs.\n\n## Required Outcome\n\n- This issue must ship code and tests.\n\n## Acceptance Criteria\n\n- lifecycle commands reject placeholder prompt content\n\n## Inputs\n- issue body\n- task bundle cards\n\n## Target Files / Surfaces\n- adl/src/cli/pr_cmd.rs\n- adl/tools/pr.sh\n\n## Validation Plan\n- Required commands: cargo test --manifest-path Cargo.toml pr_cmd -- --nocapture\n- Required tests: targeted lifecycle validation coverage\n- Required artifacts / traces: none\n- Required reviewer or demo checks: none\n\n## Demo / Proof Requirements\n- Required demo(s): none\n- Required proof surface(s): command failure behavior and tests\n- If no demo is required, say why: tooling guardrail only\n\n## Constraints / Policies\n- Determinism requirements: stable error messages for the same stub input\n- Security / privacy requirements: no secrets or absolute host paths\n- Resource limits (time/CPU/memory/network): standard local test limits\n\n## System Invariants (must remain true)\n- Deterministic execution for identical inputs.\n- No hidden state or undeclared side effects.\n- Artifacts remain replay-compatible with the replay runner.\n- Trace artifacts contain no secrets, prompts, tool arguments, or absolute host paths.\n- Artifact schema changes are explicit and approved.\n\n## Reviewer Checklist (machine-readable hints)\n```yaml\ndeterminism_required: true\nnetwork_allowed: false\nartifact_schema_change: false\nreplay_required: true\nsecurity_sensitive: true\nci_validation_required: true\n```\n\n## Card Automation Hooks (prompt generation)\n- Prompt source fields:\n  - Goal\n  - Required Outcome\n  - Acceptance Criteria\n- Generation requirements:\n  - Deterministic output for identical input card content\n  - Preserve traceability back to the source issue prompt\n\n## Non-goals / Out of scope\n- rewriting historical issues automatically\n\n## Notes / Risks\n- none\n\n## Instructions to the Agent\n- Read the linked source issue prompt before starting work.\n- Do the work described above.\n- Write results to the paired output card file.\n",
            task_id = issue_ref.task_issue_id(),
            title = title,
            branch = branch,
            issue = issue_ref.issue_number(),
            source_rel = source_rel,
            bundle = issue_ref.task_bundle_dir_name(),
            slug = issue_ref.slug(),
        );
        fs::write(path, content).expect("write authored sip");
    }

    fn write_completed_sor_fixture(path: &Path, branch: &str) {
        let body = format!(
            r#"# ADL Output Card

Canonical Template Source: `adl/templates/cards/output_card_template.md`
Consumed by: `adl/tools/pr.sh` (`OUTPUT_TEMPLATE`) with legacy fallback support for `.adl/templates/output_card_template.md`.

Execution Record Requirements:
- The output card is a machine-auditable execution record.
- All sections must be fully populated. Empty sections, placeholders, or implicit claims are not allowed.
- Every command listed must include both what was run and what it verified.
- If something is not applicable, include a one-line justification.

Task ID: issue-1153
Run ID: issue-1153
Version: v0.86
Title: rust-finish-test
Branch: {branch}
Status: DONE

Execution:
- Actor: Codex
- Model: GPT-5 Codex
- Provider: Test
- Start Time: 2026-03-29T20:19:06Z
- End Time: 2026-03-29T20:19:09Z

## Summary

Finish test summary.

## Artifacts produced
- Code:
  - `adl/src/cli/pr_cmd.rs`
- Generated runtime artifacts: not_applicable for this tooling task

## Actions taken
- Added Rust finish handling.

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: none
- Worktree-only paths remaining: none
- Integration state: pr_open
- Verification scope: worktree
- Integration method used: branch-local validation before draft PR publication
- Verification performed:
  - `cargo test --manifest-path adl/Cargo.toml pr_cmd`
    Verified Rust `pr` command tests.
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
  - `cargo test --manifest-path adl/Cargo.toml pr_cmd`
    Verified Rust `pr` command tests.
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
      - "cargo test --manifest-path adl/Cargo.toml pr_cmd"
  determinism:
    status: PASS
    replay_verified: true
    ordering_guarantees_verified: true
  security_privacy:
    status: PASS
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
- Determinism tests executed:
  - `cargo test --manifest-path adl/Cargo.toml pr_cmd`
- Fixtures or scripts used:
  - direct Rust unit coverage
- Replay verification (same inputs -> same artifacts/order):
  - PASS
- Ordering guarantees (sorting / tie-break rules used):
  - Stable section ordering.
- Artifact stability notes:
  - not_applicable beyond deterministic record rendering.

## Security / Privacy Checks
- Secret leakage scan performed:
  - Verified test output uses repo-relative paths only.
- Prompt / tool argument redaction verified:
  - Verified issue template text is not emitted in PR bodies.
- Absolute path leakage check:
  - PASS
- Sandbox / policy invariants preserved:
  - PASS

## Replay Artifacts
- Trace bundle path(s): not_applicable for this tooling task
- Run artifact root: not_applicable for this tooling task
- Replay command used for verification:
  - `cargo test --manifest-path adl/Cargo.toml pr_cmd`
- Replay result:
  - PASS

## Artifact Verification
- Primary proof surface:
  - `adl/src/cli/pr_cmd.rs`
- Required artifacts present:
  - yes
- Artifact schema/version checks:
  - none
- Hash/byte-stability checks:
  - not_applicable
- Missing/optional artifacts and rationale:
  - none

## Decisions / Deviations
- Kept the fixture minimal while satisfying completed-phase validation.

## Follow-ups / Deferred work
- none
"#
        );
        fs::write(path, body).expect("write completed sor fixture");
    }

    #[test]
    fn render_generated_issue_prompt_preserves_bootstrap_contract() {
        let content = render_generated_issue_prompt(
            1151,
            "v0-86-tools-implement-rust-owned-pr-init-and-pr-create-workflow-surfaces",
            "[v0.86][tools] Implement Rust-owned pr init and pr create workflow surfaces",
            "track:roadmap,type:task,area:tooling,version:v0.86",
            "https://github.com/example/repo/issues/1151",
        );
        assert!(content.contains("issue_number: 1151"));
        assert!(content.contains(
            "slug: \"v0-86-tools-implement-rust-owned-pr-init-and-pr-create-workflow-surfaces\""
        ));
        assert!(content.contains("required_outcome_type:\n  - \"code\""));
        assert!(content.contains(
            "Bootstrap-generated issue body created from the requested title and labels"
        ));
        assert!(content.contains(
            "This body should be concrete enough that `gh issue view` is usable immediately after creation."
        ));
    }

    #[test]
    fn load_issue_prompt_parses_front_matter_and_body() {
        let dir = unique_temp_dir("adl-pr-load-prompt");
        let path = dir.join("issue.md");
        fs::write(
            &path,
            "---\ntitle: \"Example\"\nlabels:\n  - \"track:roadmap\"\nissue_number: 42\n---\n\n# Heading\n\nBody\n",
        )
        .expect("write");

        let doc = load_issue_prompt(&path).expect("load");
        assert_eq!(doc.front_matter.title, "Example");
        assert_eq!(doc.front_matter.issue_number, 42);
        assert_eq!(doc.front_matter.labels, vec!["track:roadmap"]);
        assert!(doc.body.starts_with("# Heading"));
    }

    #[test]
    fn normalize_labels_csv_replaces_version_label() {
        let labels =
            normalize_labels_csv("track:roadmap,type:task,version:v0.3,area:tooling", "v0.86");
        assert_eq!(labels, "track:roadmap,type:task,area:tooling,version:v0.86");
    }

    #[test]
    fn infer_repo_from_remote_supports_https_and_ssh() {
        assert_eq!(
            infer_repo_from_remote("https://github.com/danielbaustin/agent-design-language.git"),
            Some("danielbaustin/agent-design-language".to_string())
        );
        assert_eq!(
            infer_repo_from_remote("git@github.com:danielbaustin/agent-design-language.git"),
            Some("danielbaustin/agent-design-language".to_string())
        );
        assert_eq!(
            infer_repo_from_remote("https://example.com/not-github.git"),
            None
        );
    }

    #[test]
    fn infer_wp_from_title_extracts_tag_or_defaults() {
        assert_eq!(
            infer_wp_from_title("[v0.86][WP-15] Implement local agent demo program"),
            "WP-15"
        );
        assert_eq!(infer_wp_from_title("No work package tag"), "unassigned");
    }

    #[test]
    fn infer_required_outcome_type_prefers_docs_tests_and_demo_signals() {
        assert_eq!(
            infer_required_outcome_type("track:roadmap,area:docs", "[v0.86][WP-01] Example"),
            "docs"
        );
        assert_eq!(
            infer_required_outcome_type("track:roadmap,type:test", "[v0.86][WP-01] Example"),
            "tests"
        );
        assert_eq!(
            infer_required_outcome_type("track:roadmap,area:demo", "[v0.86][WP-01] Example"),
            "demo"
        );
        assert_eq!(
            infer_required_outcome_type("track:roadmap,area:runtime", "[v0.86][WP-01] Example"),
            "code"
        );
    }

    #[test]
    fn version_can_be_inferred_from_labels_or_title() {
        assert_eq!(
            version_from_labels_csv("track:roadmap,version:v0.86,area:tools"),
            Some("v0.86".to_string())
        );
        assert_eq!(
            version_from_title("[v0.86][WP-15] Implement local agent demo program"),
            Some("v0.86".to_string())
        );
        assert_eq!(version_from_title("No version title"), None);
    }

    #[test]
    fn resolve_issue_body_uses_inline_text_default_and_file() {
        assert_eq!(
            resolve_issue_body(Some("custom body".to_string()), None).expect("body"),
            "custom body"
        );
        assert_eq!(resolve_issue_body(None, None).expect("default body"), "");

        let dir = unique_temp_dir("adl-pr-body-file");
        let path = dir.join("body.md");
        fs::write(&path, "body from file").expect("write body");
        assert_eq!(
            resolve_issue_body(None, Some(&path)).expect("file body"),
            "body from file"
        );
    }

    #[test]
    fn resolve_issue_body_rejects_stdin_and_missing_file() {
        let err = resolve_issue_body(None, Some(Path::new("-"))).expect_err("stdin unsupported");
        assert!(err.to_string().contains("--body-file - is not supported"));

        let missing = PathBuf::from("/definitely/missing/body.md");
        let err = resolve_issue_body(None, Some(&missing)).expect_err("missing file");
        assert!(err.to_string().contains("--body-file not found"));
    }

    #[test]
    fn parse_issue_number_from_url_accepts_issue_url_and_rejects_other_suffixes() {
        assert_eq!(
            parse_issue_number_from_url("https://github.com/example/repo/issues/1151")
                .expect("issue number"),
            1151
        );
        assert!(
            parse_issue_number_from_url("https://github.com/example/repo/issues/not-a-number")
                .is_err()
        );
    }

    #[test]
    fn path_relative_to_repo_returns_relative_or_absolute_when_outside_repo() {
        let repo_root = Path::new("/tmp/example-repo");
        let inside = Path::new("/tmp/example-repo/.adl/cards/1151/input_1151.md");
        let outside = Path::new("/var/tmp/elsewhere.md");
        assert_eq!(
            path_relative_to_repo(repo_root, inside),
            ".adl/cards/1151/input_1151.md"
        );
        assert_eq!(
            path_relative_to_repo(repo_root, outside),
            "/var/tmp/elsewhere.md"
        );
    }

    #[test]
    fn parse_init_args_accepts_bootstrap_flags() {
        let parsed = parse_init_args(&[
            "1151".to_string(),
            "--title".to_string(),
            "Example".to_string(),
            "--version".to_string(),
            "v0.86".to_string(),
        ])
        .expect("parse");
        assert_eq!(parsed.issue, 1151);
        assert_eq!(parsed.title_arg.as_deref(), Some("Example"));
        assert_eq!(parsed.version.as_deref(), Some("v0.86"));
    }

    #[test]
    fn parse_create_args_accepts_issue_creation_flags() {
        let parsed = parse_create_args(&[
            "--title".to_string(),
            "[v0.86][tools] New init path".to_string(),
            "--slug".to_string(),
            "new-init-path".to_string(),
            "--body".to_string(),
            "## Goal\n- test".to_string(),
            "--labels".to_string(),
            "track:roadmap,type:task,area:tools".to_string(),
            "--version".to_string(),
            "v0.86".to_string(),
        ])
        .expect("parse");
        assert_eq!(
            parsed.title_arg.as_deref(),
            Some("[v0.86][tools] New init path")
        );
        assert_eq!(parsed.slug.as_deref(), Some("new-init-path"));
        assert_eq!(parsed.body.as_deref(), Some("## Goal\n- test"));
        assert_eq!(
            parsed.labels.as_deref(),
            Some("track:roadmap,type:task,area:tools")
        );
        assert_eq!(parsed.version.as_deref(), Some("v0.86"));
    }

    #[test]
    fn parse_init_args_rejects_unknown_arg() {
        let err = parse_init_args(&["1151".to_string(), "--bogus".to_string()]).expect_err("err");
        assert!(err.to_string().contains("init: unknown arg"));
    }

    #[test]
    fn parse_create_args_rejects_missing_title_and_conflicting_body_inputs() {
        let err = parse_create_args(&[]).expect_err("missing title");
        assert!(err.to_string().contains("create: --title is required"));

        let err = parse_create_args(&[
            "--title".to_string(),
            "Example".to_string(),
            "--body".to_string(),
            "a".to_string(),
            "--body-file".to_string(),
            "body.md".to_string(),
        ])
        .expect_err("conflicting body inputs");
        assert!(err
            .to_string()
            .contains("create: pass only one of --body or --body-file"));
    }

    #[test]
    fn real_pr_dispatch_rejects_missing_and_unknown_subcommands() {
        let err = real_pr(&[]).expect_err("missing subcommand");
        assert!(err.to_string().contains(
            "pr requires a subcommand: create | init | start | ready | preflight | finish"
        ));

        let err = real_pr(&["bogus".to_string()]).expect_err("unknown subcommand");
        assert!(err.to_string().contains("unknown pr subcommand: bogus"));
    }

    #[test]
    fn parse_ready_args_accepts_flags_and_rejects_unknown_arg() {
        let parsed = parse_ready_args(&[
            "1152".to_string(),
            "--slug".to_string(),
            "ready-test".to_string(),
            "--version".to_string(),
            "v0.86".to_string(),
            "--no-fetch-issue".to_string(),
        ])
        .expect("parse ready");
        assert_eq!(parsed.issue, 1152);
        assert_eq!(parsed.slug.as_deref(), Some("ready-test"));
        assert_eq!(parsed.version.as_deref(), Some("v0.86"));
        assert!(parsed.no_fetch_issue);

        let err = parse_ready_args(&["1152".to_string(), "--bogus".to_string()]).expect_err("err");
        assert!(err.to_string().contains("ready: unknown arg"));
    }

    #[test]
    fn parse_preflight_args_accepts_flags_and_rejects_unknown_arg() {
        let parsed = parse_preflight_args(&[
            "1173".to_string(),
            "--slug".to_string(),
            "preflight-test".to_string(),
            "--version".to_string(),
            "v0.86".to_string(),
            "--no-fetch-issue".to_string(),
        ])
        .expect("parse preflight");
        assert_eq!(parsed.issue, 1173);
        assert_eq!(parsed.slug.as_deref(), Some("preflight-test"));
        assert_eq!(parsed.version.as_deref(), Some("v0.86"));
        assert!(parsed.no_fetch_issue);

        let err =
            parse_preflight_args(&["1173".to_string(), "--bogus".to_string()]).expect_err("err");
        assert!(err.to_string().contains("preflight: unknown arg"));
    }

    #[test]
    fn parse_start_args_accepts_prefix_and_rejects_unknown_arg() {
        let parsed = parse_start_args(&[
            "1152".to_string(),
            "--prefix".to_string(),
            "codex".to_string(),
            "--slug".to_string(),
            "start-test".to_string(),
            "--title".to_string(),
            "Start Test".to_string(),
            "--version".to_string(),
            "v0.86".to_string(),
            "--no-fetch-issue".to_string(),
            "--allow-open-pr-wave".to_string(),
        ])
        .expect("parse start");
        assert_eq!(parsed.issue, 1152);
        assert_eq!(parsed.prefix, "codex");
        assert_eq!(parsed.slug.as_deref(), Some("start-test"));
        assert_eq!(parsed.title_arg.as_deref(), Some("Start Test"));
        assert_eq!(parsed.version.as_deref(), Some("v0.86"));
        assert!(parsed.no_fetch_issue);
        assert!(parsed.allow_open_pr_wave);

        let err = parse_start_args(&["1152".to_string(), "--bogus".to_string()]).expect_err("err");
        assert!(err.to_string().contains("start: unknown arg"));
    }

    #[test]
    fn real_pr_init_seeds_stp_from_generated_source_prompt() {
        let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
        let repo = unique_temp_dir("adl-pr-real-init");
        init_git_repo(&repo);
        copy_bootstrap_support_files(&repo);
        let prev_dir = env::current_dir().expect("cwd");
        env::set_current_dir(&repo).expect("chdir");

        let result = real_pr(&[
            "init".to_string(),
            "1151".to_string(),
            "--slug".to_string(),
            "v0-86-tools-init-test".to_string(),
            "--title".to_string(),
            "[v0.86][tools] Init test".to_string(),
            "--no-fetch-issue".to_string(),
            "--version".to_string(),
            "v0.86".to_string(),
        ]);

        env::set_current_dir(prev_dir).expect("restore cwd");
        result.expect("real_pr init");

        let issue_ref = IssueRef::new(
            1151,
            "v0.86".to_string(),
            "v0-86-tools-init-test".to_string(),
        )
        .expect("issue ref");
        let stp_path = issue_ref.task_bundle_stp_path(&repo);
        let source_path = issue_ref.issue_prompt_path(&repo);
        let sip_path = issue_ref.task_bundle_input_path(&repo);
        let sor_path = issue_ref.task_bundle_output_path(&repo);
        assert!(stp_path.is_file());
        assert!(source_path.is_file());
        assert!(sip_path.is_file());
        assert!(sor_path.is_file());
        let stp = fs::read_to_string(&stp_path).expect("read stp");
        assert!(stp.contains("issue_number: 1151"));
        assert!(stp.contains("title: \"[v0.86][tools] Init test\""));
    }

    #[test]
    fn real_pr_init_existing_stp_is_left_untouched() {
        let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
        let repo = unique_temp_dir("adl-pr-real-init-existing");
        init_git_repo(&repo);
        copy_bootstrap_support_files(&repo);
        let issue_ref = IssueRef::new(
            1151,
            "v0.86".to_string(),
            "v0-86-tools-init-existing".to_string(),
        )
        .expect("issue ref");
        let stp_path = issue_ref.task_bundle_stp_path(&repo);
        let sip_path = issue_ref.task_bundle_input_path(&repo);
        let sor_path = issue_ref.task_bundle_output_path(&repo);
        fs::create_dir_all(stp_path.parent().expect("parent")).expect("bundle dir");
        fs::write(&stp_path, "sentinel\n").expect("write sentinel");

        let prev_dir = env::current_dir().expect("cwd");
        env::set_current_dir(&repo).expect("chdir");
        let result = real_pr(&[
            "init".to_string(),
            "1151".to_string(),
            "--slug".to_string(),
            "v0-86-tools-init-existing".to_string(),
            "--title".to_string(),
            "[v0.86][tools] Init existing".to_string(),
            "--no-fetch-issue".to_string(),
            "--version".to_string(),
            "v0.86".to_string(),
        ]);
        env::set_current_dir(prev_dir).expect("restore cwd");
        result.expect("real_pr init existing");
        assert_eq!(
            fs::read_to_string(&stp_path).expect("read stp"),
            "sentinel\n"
        );
        assert!(sip_path.is_file());
        assert!(sor_path.is_file());
    }

    #[test]
    fn real_pr_create_creates_issue_without_bootstrapping_bundle() {
        let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
        let repo = unique_temp_dir("adl-pr-real-create");
        init_git_repo(&repo);
        copy_bootstrap_support_files(&repo);

        let bin_dir = repo.join("bin");
        fs::create_dir_all(&bin_dir).expect("bin dir");
        let gh_log = repo.join("gh.log");
        let issue_body_log = repo.join("issue_body.log");
        let gh_path = bin_dir.join("gh");
        write_executable(
            &gh_path,
            &format!(
                "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\nif [[ \"$1 $2\" == \"issue create\" ]]; then\n  i=1\n  while [[ $i -le $# ]]; do\n    arg=\"${{@:$i:1}}\"\n    if [[ \"$arg\" == \"--body\" ]]; then\n      next=$((i+1))\n      printf '%s' \"${{@:$next:1}}\" > '{}'\n      break\n    fi\n    i=$((i+1))\n  done\n  printf 'https://github.com/example/repo/issues/1202\\n'\n  exit 0\nfi\nif [[ \"$1 $2\" == \"issue edit\" ]]; then\n  exit 0\nfi\nexit 1\n",
                gh_log.display(),
                issue_body_log.display()
            ),
        );

        let old_path = env::var("PATH").unwrap_or_default();
        let prev_dir = env::current_dir().expect("cwd");
        unsafe {
            env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
        }
        env::set_current_dir(&repo).expect("chdir");

        let result = real_pr(&[
            "create".to_string(),
            "--title".to_string(),
            "[v0.86][tools] Simplified init path".to_string(),
            "--slug".to_string(),
            "v0-86-tools-simplified-init-path".to_string(),
            "--body".to_string(),
            "## Goal\n- simplify init".to_string(),
            "--labels".to_string(),
            "track:roadmap,type:task,area:tools".to_string(),
            "--version".to_string(),
            "v0.86".to_string(),
        ]);

        env::set_current_dir(prev_dir).expect("restore cwd");
        unsafe {
            env::set_var("PATH", old_path);
        }
        result.expect("real_pr create");

        let gh_calls = fs::read_to_string(&gh_log).expect("gh log");
        assert!(gh_calls.contains("issue create"));
        assert!(gh_calls.contains("--label"));
        assert!(gh_calls.contains("version:v0.86"));
        let source = repo.join(".adl/v0.86/bodies/issue-1202-v0-86-tools-simplified-init-path.md");
        assert!(
            source.is_file(),
            "create should write the local source prompt"
        );
        let prompt = fs::read_to_string(&source).expect("read source prompt");
        assert!(prompt.contains("issue_number: 1202"));
        assert!(prompt.contains("## Goal\n- simplify init"));
        assert!(
            !repo.join(".adl/v0.86/tasks").exists(),
            "create should not bootstrap the local task bundle"
        );
        let issue_body = fs::read_to_string(&issue_body_log).expect("issue body");
        assert_eq!(issue_body, "## Goal\n- simplify init");
    }

    #[test]
    fn real_pr_create_generates_concrete_body_when_none_is_supplied() {
        let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
        let repo = unique_temp_dir("adl-pr-real-create-generated-body");
        init_git_repo(&repo);
        copy_bootstrap_support_files(&repo);

        let bin_dir = repo.join("bin");
        fs::create_dir_all(&bin_dir).expect("bin dir");
        let issue_body_log = repo.join("issue_body.log");
        let gh_path = bin_dir.join("gh");
        write_executable(
            &gh_path,
            &format!(
                "#!/usr/bin/env bash\nset -euo pipefail\nif [[ \"$1 $2\" == \"issue create\" ]]; then\n  i=1\n  while [[ $i -le $# ]]; do\n    arg=\"${{@:$i:1}}\"\n    if [[ \"$arg\" == \"--body\" ]]; then\n      next=$((i+1))\n      printf '%s' \"${{@:$next:1}}\" > '{}'\n      break\n    fi\n    i=$((i+1))\n  done\n  printf 'https://github.com/example/repo/issues/1203\\n'\n  exit 0\nfi\nif [[ \"$1 $2\" == \"issue edit\" ]]; then\n  exit 0\nfi\nexit 1\n",
                issue_body_log.display()
            ),
        );

        let old_path = env::var("PATH").unwrap_or_default();
        let prev_dir = env::current_dir().expect("cwd");
        unsafe {
            env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
        }
        env::set_current_dir(&repo).expect("chdir");

        let result = real_pr(&[
            "create".to_string(),
            "--title".to_string(),
            "[v0.86][tools] Generated issue body".to_string(),
            "--slug".to_string(),
            "v0-86-tools-generated-issue-body".to_string(),
            "--labels".to_string(),
            "track:roadmap,type:task,area:tools".to_string(),
            "--version".to_string(),
            "v0.86".to_string(),
        ]);

        env::set_current_dir(prev_dir).expect("restore cwd");
        unsafe {
            env::set_var("PATH", old_path);
        }
        result.expect("real_pr create");

        let issue_body = fs::read_to_string(&issue_body_log).expect("issue body");
        assert!(issue_body.contains("## Goal"));
        assert!(issue_body.contains("## Acceptance Criteria"));
        assert!(!issue_body.contains("## Goal\n-"));
        assert!(!issue_body.contains("## Acceptance Criteria\n-"));
        let source = repo.join(".adl/v0.86/bodies/issue-1203-v0-86-tools-generated-issue-body.md");
        let prompt = fs::read_to_string(&source).expect("read source prompt");
        assert!(prompt.contains("issue_number: 1203"));
        assert!(prompt.contains("## Goal"));
        assert!(!prompt.contains("## Goal\n-"));
    }

    #[test]
    fn real_pr_start_bootstraps_worktree_and_ready_passes() {
        let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
        let temp = unique_temp_dir("adl-pr-start-ready");
        let origin = temp.join("origin.git");
        let repo = temp.join("repo");
        fs::create_dir_all(&repo).expect("repo dir");
        copy_bootstrap_support_files(&repo);
        init_git_repo(&repo);
        assert!(Command::new("git")
            .args(["config", "user.name", "Test User"])
            .current_dir(&repo)
            .status()
            .expect("git config")
            .success());
        assert!(Command::new("git")
            .args(["config", "user.email", "test@example.com"])
            .current_dir(&repo)
            .status()
            .expect("git config")
            .success());
        fs::write(repo.join("README.md"), "ready branch placeholder\n").expect("write readme");
        assert!(Command::new("git")
            .args(["add", "-A"])
            .current_dir(&repo)
            .status()
            .expect("git add")
            .success());
        assert!(Command::new("git")
            .args(["commit", "-q", "-m", "init"])
            .current_dir(&repo)
            .status()
            .expect("git commit")
            .success());
        assert!(Command::new("git")
            .args(["branch", "-M", "main"])
            .current_dir(&repo)
            .status()
            .expect("git branch")
            .success());
        assert!(Command::new("git")
            .args([
                "init",
                "--bare",
                "-q",
                path_str(&origin).expect("origin path")
            ])
            .current_dir(&repo)
            .status()
            .expect("git init bare")
            .success());
        assert!(Command::new("git")
            .args([
                "remote",
                "set-url",
                "origin",
                path_str(&origin).expect("origin path"),
            ])
            .current_dir(&repo)
            .status()
            .expect("git remote set-url")
            .success());
        assert!(Command::new("git")
            .args(["push", "-q", "-u", "origin", "main"])
            .current_dir(&repo)
            .status()
            .expect("git push")
            .success());
        assert!(Command::new("git")
            .args(["fetch", "-q", "origin", "main"])
            .current_dir(&repo)
            .status()
            .expect("git fetch")
            .success());

        let prev_dir = env::current_dir().expect("cwd");
        env::set_current_dir(&repo).expect("chdir");
        let issue_ref = IssueRef::new(1152, "v0.86", "rust-start-ready-test").expect("issue ref");
        write_authored_issue_prompt(&repo, &issue_ref, "[v0.86][tools] Rust start ready test");

        real_pr(&[
            "start".to_string(),
            "1152".to_string(),
            "--slug".to_string(),
            "rust-start-ready-test".to_string(),
            "--title".to_string(),
            "[v0.86][tools] Rust start ready test".to_string(),
            "--no-fetch-issue".to_string(),
            "--version".to_string(),
            "v0.86".to_string(),
        ])
        .expect("real_pr start");

        let root_sip = issue_ref.task_bundle_input_path(&repo);
        let worktree = issue_ref.default_worktree_path(&repo, None);
        let wt_sip = issue_ref.task_bundle_input_path(&worktree);
        let source_path = issue_ref.issue_prompt_path(&repo);
        let branch = "codex/1152-rust-start-ready-test";
        write_authored_sip(
            &root_sip,
            &issue_ref,
            "[v0.86][tools] Rust start ready test",
            branch,
            &source_path,
            &repo,
        );
        write_authored_sip(
            &wt_sip,
            &issue_ref,
            "[v0.86][tools] Rust start ready test",
            branch,
            &issue_ref.issue_prompt_path(&worktree),
            &worktree,
        );

        let ready = real_pr(&[
            "ready".to_string(),
            "1152".to_string(),
            "--slug".to_string(),
            "rust-start-ready-test".to_string(),
            "--no-fetch-issue".to_string(),
            "--version".to_string(),
            "v0.86".to_string(),
        ]);

        env::set_current_dir(prev_dir).expect("restore cwd");
        ready.expect("real_pr ready");

        assert!(worktree.is_dir());
        assert_eq!(
            run_capture(
                "git",
                &[
                    "-C",
                    path_str(&worktree).expect("wt path"),
                    "rev-parse",
                    "--abbrev-ref",
                    "HEAD"
                ]
            )
            .expect("branch")
            .trim(),
            "codex/1152-rust-start-ready-test"
        );
        assert!(issue_ref.task_bundle_stp_path(&repo).is_file());
        assert!(issue_ref.task_bundle_input_path(&repo).is_file());
        assert!(issue_ref.task_bundle_output_path(&repo).is_file());
        assert!(issue_ref.task_bundle_stp_path(&worktree).is_file());
        assert!(issue_ref.task_bundle_input_path(&worktree).is_file());
        assert!(issue_ref.task_bundle_output_path(&worktree).is_file());
        let root_cards = resolve_cards_root(&repo, None);
        assert!(card_input_path(&root_cards, 1152)
            .symlink_metadata()
            .is_ok());
        assert!(card_output_path(&root_cards, 1152)
            .symlink_metadata()
            .is_ok());
    }

    #[test]
    fn real_pr_ready_succeeds_when_invoked_from_started_worktree() {
        let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
        let temp = unique_temp_dir("adl-pr-ready-worktree-cwd");
        let origin = temp.join("origin.git");
        let repo = temp.join("repo");
        fs::create_dir_all(&repo).expect("repo dir");
        copy_bootstrap_support_files(&repo);
        init_git_repo(&repo);
        assert!(Command::new("git")
            .args(["config", "user.name", "Test User"])
            .current_dir(&repo)
            .status()
            .expect("git config")
            .success());
        assert!(Command::new("git")
            .args(["config", "user.email", "test@example.com"])
            .current_dir(&repo)
            .status()
            .expect("git config")
            .success());
        fs::write(repo.join("README.md"), "ready from worktree\n").expect("seed file");
        assert!(Command::new("git")
            .args(["add", "-A"])
            .current_dir(&repo)
            .status()
            .expect("git add")
            .success());
        assert!(Command::new("git")
            .args(["commit", "-q", "-m", "init"])
            .current_dir(&repo)
            .status()
            .expect("git commit")
            .success());
        assert!(Command::new("git")
            .args(["branch", "-M", "main"])
            .current_dir(&repo)
            .status()
            .expect("git branch")
            .success());
        assert!(Command::new("git")
            .args([
                "init",
                "--bare",
                "-q",
                path_str(&origin).expect("origin path"),
            ])
            .current_dir(&repo)
            .status()
            .expect("git init bare")
            .success());
        assert!(Command::new("git")
            .args([
                "remote",
                "set-url",
                "origin",
                path_str(&origin).expect("origin path"),
            ])
            .current_dir(&repo)
            .status()
            .expect("git remote set-url")
            .success());
        assert!(Command::new("git")
            .args(["push", "-q", "-u", "origin", "main"])
            .current_dir(&repo)
            .status()
            .expect("git push")
            .success());
        assert!(Command::new("git")
            .args(["fetch", "-q", "origin", "main"])
            .current_dir(&repo)
            .status()
            .expect("git fetch")
            .success());

        let prev_dir = env::current_dir().expect("cwd");
        env::set_current_dir(&repo).expect("chdir");
        let issue_ref = IssueRef::new(1198, "v0.86", "ready-worktree-cwd").expect("issue ref");
        write_authored_issue_prompt(&repo, &issue_ref, "[v0.86][tools] Ready worktree cwd");
        real_pr(&[
            "start".to_string(),
            "1198".to_string(),
            "--slug".to_string(),
            "ready-worktree-cwd".to_string(),
            "--title".to_string(),
            "[v0.86][tools] Ready worktree cwd".to_string(),
            "--no-fetch-issue".to_string(),
            "--version".to_string(),
            "v0.86".to_string(),
        ])
        .expect("real_pr start");
        let worktree = issue_ref.default_worktree_path(&repo, None);
        let root_sip = issue_ref.task_bundle_input_path(&repo);
        let wt_sip = issue_ref.task_bundle_input_path(&worktree);
        write_authored_sip(
            &root_sip,
            &issue_ref,
            "[v0.86][tools] Ready worktree cwd",
            "codex/1198-ready-worktree-cwd",
            &issue_ref.issue_prompt_path(&repo),
            &repo,
        );
        write_authored_sip(
            &wt_sip,
            &issue_ref,
            "[v0.86][tools] Ready worktree cwd",
            "codex/1198-ready-worktree-cwd",
            &issue_ref.issue_prompt_path(&worktree),
            &worktree,
        );
        env::set_current_dir(&worktree).expect("chdir worktree");

        let ready = real_pr(&[
            "ready".to_string(),
            "1198".to_string(),
            "--slug".to_string(),
            "ready-worktree-cwd".to_string(),
            "--no-fetch-issue".to_string(),
            "--version".to_string(),
            "v0.86".to_string(),
        ]);

        env::set_current_dir(prev_dir).expect("restore cwd");
        ready.expect("ready from worktree");
    }

    #[test]
    fn real_pr_preflight_reports_open_milestone_prs() {
        let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
        let repo = unique_temp_dir("adl-pr-preflight");
        init_git_repo(&repo);
        let bin_dir = repo.join("bin");
        fs::create_dir_all(&bin_dir).expect("bin dir");
        let gh_path = bin_dir.join("gh");
        write_executable(
            &gh_path,
            "#!/usr/bin/env bash\nset -euo pipefail\nif [[ \"$1 $2\" == \"pr list\" ]]; then\n  cat <<'JSON'\n[{\"number\":1169,\"title\":\"[v0.86][runtime] Sprint 3A: Make WP-06 fast / slow paths drive real runtime behavior\",\"url\":\"https://example.test/pr/1169\",\"headRefName\":\"codex/1161-v0-86-runtime-sprint-3a-make-wp-06-fast-slow-paths-drive-real-runtime-behavior\",\"baseRefName\":\"main\",\"isDraft\":true}]\nJSON\n  exit 0\nfi\nexit 1\n",
        );

        let old_path = env::var("PATH").unwrap_or_default();
        let prev_dir = env::current_dir().expect("cwd");
        unsafe {
            env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
        }
        env::set_current_dir(&repo).expect("chdir");

        let result = real_pr(&[
            "preflight".to_string(),
            "1173".to_string(),
            "--slug".to_string(),
            "v0-86-tools-preflight".to_string(),
            "--version".to_string(),
            "v0.86".to_string(),
            "--no-fetch-issue".to_string(),
        ]);

        env::set_current_dir(prev_dir).expect("restore cwd");
        unsafe {
            env::set_var("PATH", old_path);
        }
        result.expect("preflight");
    }

    #[test]
    fn real_pr_start_blocks_when_open_milestone_pr_wave_exists() {
        let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
        let temp = unique_temp_dir("adl-pr-start-blocks-wave");
        let origin = temp.join("origin.git");
        let repo = temp.join("repo");
        fs::create_dir_all(&repo).expect("repo dir");
        copy_bootstrap_support_files(&repo);
        init_git_repo(&repo);
        assert!(Command::new("git")
            .args(["config", "user.name", "Test User"])
            .current_dir(&repo)
            .status()
            .expect("git config")
            .success());
        assert!(Command::new("git")
            .args(["config", "user.email", "test@example.com"])
            .current_dir(&repo)
            .status()
            .expect("git config")
            .success());
        fs::write(repo.join("README.md"), "ready branch placeholder\n").expect("write readme");
        assert!(Command::new("git")
            .args(["add", "-A"])
            .current_dir(&repo)
            .status()
            .expect("git add")
            .success());
        assert!(Command::new("git")
            .args(["commit", "-q", "-m", "init"])
            .current_dir(&repo)
            .status()
            .expect("git commit")
            .success());
        assert!(Command::new("git")
            .args(["branch", "-M", "main"])
            .current_dir(&repo)
            .status()
            .expect("git branch")
            .success());
        assert!(Command::new("git")
            .args([
                "init",
                "--bare",
                "-q",
                path_str(&origin).expect("origin path")
            ])
            .current_dir(&repo)
            .status()
            .expect("git init bare")
            .success());
        assert!(Command::new("git")
            .args([
                "remote",
                "set-url",
                "origin",
                path_str(&origin).expect("origin path"),
            ])
            .current_dir(&repo)
            .status()
            .expect("git remote set-url")
            .success());
        assert!(Command::new("git")
            .args(["push", "-q", "-u", "origin", "main"])
            .current_dir(&repo)
            .status()
            .expect("git push")
            .success());
        assert!(Command::new("git")
            .args(["fetch", "-q", "origin", "main"])
            .current_dir(&repo)
            .status()
            .expect("git fetch")
            .success());

        let bin_dir = repo.join("bin");
        fs::create_dir_all(&bin_dir).expect("bin dir");
        let gh_path = bin_dir.join("gh");
        write_executable(
            &gh_path,
            "#!/usr/bin/env bash\nset -euo pipefail\nif [[ \"$1 $2\" == \"pr list\" ]]; then\n  cat <<'JSON'\n[{\"number\":1169,\"title\":\"[v0.86][runtime] Sprint 3A: Make WP-06 fast / slow paths drive real runtime behavior\",\"url\":\"https://example.test/pr/1169\",\"headRefName\":\"codex/1161-v0-86-runtime-sprint-3a-make-wp-06-fast-slow-paths-drive-real-runtime-behavior\",\"baseRefName\":\"main\",\"isDraft\":true}]\nJSON\n  exit 0\nfi\nexit 1\n",
        );

        let old_path = env::var("PATH").unwrap_or_default();
        let prev_dir = env::current_dir().expect("cwd");
        unsafe {
            env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
        }
        env::set_current_dir(&repo).expect("chdir");

        let err = real_pr(&[
            "start".to_string(),
            "1173".to_string(),
            "--slug".to_string(),
            "v0-86-tools-preflight-guard".to_string(),
            "--title".to_string(),
            "[v0.86][tools] Preflight guard".to_string(),
            "--version".to_string(),
            "v0.86".to_string(),
            "--no-fetch-issue".to_string(),
        ])
        .expect_err("start should block on open PR wave");

        env::set_current_dir(prev_dir).expect("restore cwd");
        unsafe {
            env::set_var("PATH", old_path);
        }
        assert!(err
            .to_string()
            .contains("start: unresolved open PR wave detected for v0.86"));
        assert!(err.to_string().contains("#1169 [draft]"));
    }

    #[test]
    fn real_pr_ready_requires_slug_when_local_state_missing() {
        let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
        let repo = unique_temp_dir("adl-pr-ready-missing-slug");
        init_git_repo(&repo);
        let prev_dir = env::current_dir().expect("cwd");
        env::set_current_dir(&repo).expect("chdir");
        let err =
            real_pr(&["ready".to_string(), "1152".to_string()]).expect_err("ready should fail");
        env::set_current_dir(prev_dir).expect("restore cwd");
        assert!(err
            .to_string()
            .contains("ready: could not infer slug; pass --slug or run start first"));
    }

    #[test]
    fn parse_finish_args_requires_title_and_accepts_finish_flags() {
        let err = parse_finish_args(&["1153".to_string()]).expect_err("missing title");
        assert!(err.to_string().contains("--title is required"));

        let parsed = parse_finish_args(&[
            "1153".to_string(),
            "--title".to_string(),
            "Example".to_string(),
            "--paths".to_string(),
            "adl,docs".to_string(),
            "--no-checks".to_string(),
            "--ready".to_string(),
            "--no-open".to_string(),
        ])
        .expect("parse finish");
        assert_eq!(parsed.issue, 1153);
        assert_eq!(parsed.title, "Example");
        assert_eq!(parsed.paths, "adl,docs");
        assert!(parsed.no_checks);
        assert!(parsed.ready);
        assert!(parsed.no_open);
    }

    #[test]
    fn render_pr_body_uses_output_sections_and_rejects_issue_template_text() {
        let temp = unique_temp_dir("adl-pr-render-body");
        fs::create_dir_all(&temp).expect("temp dir");
        let input = temp.join("input.md");
        let output = temp.join("output.md");
        fs::write(&input, "# input\n").expect("write input");
        fs::write(
            &output,
            "# ADL Output Card\n\n## Summary\nsummary text\n\n## Artifacts produced\n- adl/src/cli/pr_cmd.rs\n\n## Validation\n- cargo test\n",
        )
        .expect("write output");

        let body = render_pr_body(
            Some("Closes #1153"),
            &input,
            &output,
            Some("extra notes"),
            false,
            "fp-123",
            &temp,
        )
        .expect("render body");
        assert!(body.contains("Closes #1153"));
        assert!(body.contains("## Summary"));
        assert!(body.contains("summary text"));
        assert!(body.contains("## Artifacts"));
        assert!(body.contains("adl/src/cli/pr_cmd.rs"));
        assert!(body.contains("## Validation"));
        assert!(body.contains("## Notes"));
        assert!(body.contains("Idempotency-Key: fp-123"));

        let err = render_pr_body(
            None,
            &input,
            &output,
            Some("issue_card_schema: adl.issue.v1"),
            false,
            "fp-123",
            &temp,
        )
        .expect_err("issue template text should be rejected");
        assert!(err.to_string().contains("issue-template/prompt text"));
    }

    #[test]
    fn real_pr_finish_creates_draft_pr_and_commits_branch_changes() {
        let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
        let temp = unique_temp_dir("adl-pr-finish-create");
        let origin = temp.join("origin.git");
        let repo = temp.join("repo");
        fs::create_dir_all(&repo).expect("repo dir");
        copy_bootstrap_support_files(&repo);
        init_git_repo(&repo);
        assert!(Command::new("git")
            .args(["config", "user.name", "Test User"])
            .current_dir(&repo)
            .status()
            .expect("git config")
            .success());
        assert!(Command::new("git")
            .args(["config", "user.email", "test@example.com"])
            .current_dir(&repo)
            .status()
            .expect("git config")
            .success());
        fs::create_dir_all(repo.join("adl/src")).expect("adl src");
        fs::write(repo.join("adl/src/lib.rs"), "pub fn placeholder() {}\n").expect("write source");
        assert!(Command::new("git")
            .args(["add", "-A"])
            .current_dir(&repo)
            .status()
            .expect("git add")
            .success());
        assert!(Command::new("git")
            .args(["commit", "-q", "-m", "init"])
            .current_dir(&repo)
            .status()
            .expect("git commit")
            .success());
        assert!(Command::new("git")
            .args(["branch", "-M", "main"])
            .current_dir(&repo)
            .status()
            .expect("git branch")
            .success());
        assert!(Command::new("git")
            .args([
                "init",
                "--bare",
                "-q",
                path_str(&origin).expect("origin path"),
            ])
            .current_dir(&repo)
            .status()
            .expect("git init bare")
            .success());
        assert!(Command::new("git")
            .args([
                "remote",
                "set-url",
                "origin",
                path_str(&origin).expect("origin path"),
            ])
            .current_dir(&repo)
            .status()
            .expect("git remote set-url")
            .success());
        assert!(Command::new("git")
            .args(["push", "-q", "-u", "origin", "main"])
            .current_dir(&repo)
            .status()
            .expect("git push")
            .success());
        assert!(Command::new("git")
            .args(["checkout", "-q", "-b", "codex/1153-rust-finish-test",])
            .current_dir(&repo)
            .status()
            .expect("git checkout")
            .success());

        let issue_ref =
            IssueRef::new(1153, "v0.86".to_string(), "rust-finish-test".to_string()).expect("ref");
        let bundle_dir = issue_ref.task_bundle_dir_path(&repo);
        fs::create_dir_all(&bundle_dir).expect("bundle dir");
        let stp = issue_ref.task_bundle_stp_path(&repo);
        let input = issue_ref.task_bundle_input_path(&repo);
        let output = issue_ref.task_bundle_output_path(&repo);
        write_authored_issue_prompt(&repo, &issue_ref, "[v0.86][tools] Rust finish test");
        fs::copy(issue_ref.issue_prompt_path(&repo), &stp).expect("seed stp");
        write_authored_sip(
            &input,
            &issue_ref,
            "[v0.86][tools] Rust finish test",
            "codex/1153-rust-finish-test",
            &issue_ref.issue_prompt_path(&repo),
            &repo,
        );
        write_completed_sor_fixture(&output, "codex/1153-rust-finish-test");

        fs::write(
            repo.join("adl/src/lib.rs"),
            "pub fn placeholder() {}\npub fn changed() {}\n",
        )
        .expect("write change");

        let bin_dir = temp.join("bin");
        fs::create_dir_all(&bin_dir).expect("bin dir");
        let gh_log = temp.join("gh.log");
        let gh_path = bin_dir.join("gh");
        write_executable(
            &gh_path,
            &format!(
                "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\nif [ \"$1 $2 $3\" = 'repo view --json' ]; then\n  printf 'danielbaustin/agent-design-language\\n'\n  exit 0\nfi\nif [ \"$1 $2\" = 'pr list' ]; then\n  exit 0\nfi\nif [ \"$1 $2\" = 'pr create' ]; then\n  printf 'https://github.com/danielbaustin/agent-design-language/pull/1159\\n'\n  exit 0\nfi\nif [ \"$1 $2\" = 'pr view' ]; then\n  if printf '%s ' \"$@\" | grep -q 'closingIssuesReferences'; then\n    printf '1153\\n'\n  else\n    printf 'Closes #1153\\n'\n  fi\n  exit 0\nfi\nexit 1\n",
                gh_log.display()
            ),
        );

        let old_path = env::var("PATH").unwrap_or_default();
        let prev_dir = env::current_dir().expect("cwd");
        unsafe {
            env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
        }
        env::set_current_dir(&repo).expect("chdir");

        let result = real_pr(&[
            "finish".to_string(),
            "1153".to_string(),
            "--title".to_string(),
            "[v0.86][tools] Rust finish test".to_string(),
            "--paths".to_string(),
            "adl".to_string(),
            "--input".to_string(),
            path_relative_to_repo(&repo, &input),
            "--output".to_string(),
            path_relative_to_repo(&repo, &output),
            "--no-checks".to_string(),
            "--no-open".to_string(),
        ]);

        env::set_current_dir(prev_dir).expect("restore cwd");
        unsafe {
            env::set_var("PATH", old_path);
        }
        result.expect("real_pr finish");

        let head_subject = run_capture(
            "git",
            &[
                "-C",
                path_str(&repo).expect("repo"),
                "log",
                "-1",
                "--format=%s",
            ],
        )
        .expect("head subject");
        assert!(head_subject.contains("[v0.86][tools] Rust finish test (Closes #1153)"));
        assert!(Command::new("git")
            .args([
                "--git-dir",
                path_str(&origin).expect("origin"),
                "rev-parse",
                "--verify",
                "refs/heads/codex/1153-rust-finish-test",
            ])
            .status()
            .expect("verify pushed branch")
            .success());
        let gh_calls = fs::read_to_string(&gh_log).expect("read gh log");
        assert!(gh_calls.contains("pr create"));
        assert!(gh_calls.contains("pr view -R danielbaustin/agent-design-language https://github.com/danielbaustin/agent-design-language/pull/1159 --json closingIssuesReferences --jq .closingIssuesReferences[]?.number"));
    }

    #[test]
    fn real_pr_finish_updates_existing_pr_and_marks_ready() {
        let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
        let temp = unique_temp_dir("adl-pr-finish-edit");
        let origin = temp.join("origin.git");
        let repo = temp.join("repo");
        fs::create_dir_all(&repo).expect("repo dir");
        copy_bootstrap_support_files(&repo);
        init_git_repo(&repo);
        assert!(Command::new("git")
            .args(["config", "user.name", "Test User"])
            .current_dir(&repo)
            .status()
            .expect("git config")
            .success());
        assert!(Command::new("git")
            .args(["config", "user.email", "test@example.com"])
            .current_dir(&repo)
            .status()
            .expect("git config")
            .success());
        fs::create_dir_all(repo.join("adl/src")).expect("adl src");
        fs::write(repo.join("adl/src/lib.rs"), "pub fn placeholder() {}\n").expect("write source");
        assert!(Command::new("git")
            .args(["add", "-A"])
            .current_dir(&repo)
            .status()
            .expect("git add")
            .success());
        assert!(Command::new("git")
            .args(["commit", "-q", "-m", "init"])
            .current_dir(&repo)
            .status()
            .expect("git commit")
            .success());
        assert!(Command::new("git")
            .args(["branch", "-M", "main"])
            .current_dir(&repo)
            .status()
            .expect("git branch")
            .success());
        assert!(Command::new("git")
            .args([
                "init",
                "--bare",
                "-q",
                path_str(&origin).expect("origin path"),
            ])
            .current_dir(&repo)
            .status()
            .expect("git init bare")
            .success());
        assert!(Command::new("git")
            .args([
                "remote",
                "set-url",
                "origin",
                path_str(&origin).expect("origin path"),
            ])
            .current_dir(&repo)
            .status()
            .expect("git remote set-url")
            .success());
        assert!(Command::new("git")
            .args(["push", "-q", "-u", "origin", "main"])
            .current_dir(&repo)
            .status()
            .expect("git push")
            .success());
        assert!(Command::new("git")
            .args(["checkout", "-q", "-b", "codex/1153-rust-finish-test-edit",])
            .current_dir(&repo)
            .status()
            .expect("git checkout")
            .success());

        let issue_ref = IssueRef::new(
            1153,
            "v0.86".to_string(),
            "rust-finish-test-edit".to_string(),
        )
        .expect("ref");
        let bundle_dir = issue_ref.task_bundle_dir_path(&repo);
        fs::create_dir_all(&bundle_dir).expect("bundle dir");
        let stp = issue_ref.task_bundle_stp_path(&repo);
        let input = issue_ref.task_bundle_input_path(&repo);
        let output = issue_ref.task_bundle_output_path(&repo);
        write_authored_issue_prompt(&repo, &issue_ref, "[v0.86][tools] Rust finish test edit");
        fs::copy(issue_ref.issue_prompt_path(&repo), &stp).expect("seed stp");
        write_authored_sip(
            &input,
            &issue_ref,
            "[v0.86][tools] Rust finish test edit",
            "codex/1153-rust-finish-test-edit",
            &issue_ref.issue_prompt_path(&repo),
            &repo,
        );
        write_completed_sor_fixture(&output, "codex/1153-rust-finish-test-edit");

        fs::write(repo.join("adl/src/lib.rs"), "pub fn placeholder() {}\n").expect("write change");
        assert!(Command::new("git")
            .args(["add", "-A"])
            .current_dir(&repo)
            .status()
            .expect("git add")
            .success());
        assert!(Command::new("git")
            .args(["commit", "-q", "-m", "existing branch commit"])
            .current_dir(&repo)
            .status()
            .expect("git commit")
            .success());

        let bin_dir = temp.join("bin");
        fs::create_dir_all(&bin_dir).expect("bin dir");
        let gh_log = temp.join("gh.log");
        let gh_path = bin_dir.join("gh");
        write_executable(
            &gh_path,
            &format!(
                "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\nif [ \"$1 $2 $3\" = 'repo view --json' ]; then\n  printf 'danielbaustin/agent-design-language\\n'\n  exit 0\nfi\nif [ \"$1 $2\" = 'pr list' ]; then\n  printf 'https://github.com/danielbaustin/agent-design-language/pull/1160\\n'\n  exit 0\nfi\nif [ \"$1 $2\" = 'pr edit' ]; then\n  exit 0\nfi\nif [ \"$1 $2\" = 'pr ready' ]; then\n  exit 0\nfi\nif [ \"$1 $2\" = 'pr view' ]; then\n  if printf '%s ' \"$@\" | grep -q 'closingIssuesReferences'; then\n    printf '1153\\n'\n  else\n    printf 'Closes #1153\\n'\n  fi\n  exit 0\nfi\nexit 1\n",
                gh_log.display()
            ),
        );

        let old_path = env::var("PATH").unwrap_or_default();
        let prev_dir = env::current_dir().expect("cwd");
        unsafe {
            env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
        }
        env::set_current_dir(&repo).expect("chdir");

        let result = real_pr(&[
            "finish".to_string(),
            "1153".to_string(),
            "--title".to_string(),
            "[v0.86][tools] Rust finish test edit".to_string(),
            "--paths".to_string(),
            "adl".to_string(),
            "--input".to_string(),
            path_relative_to_repo(&repo, &input),
            "--output".to_string(),
            path_relative_to_repo(&repo, &output),
            "--no-checks".to_string(),
            "--ready".to_string(),
            "--no-open".to_string(),
        ]);

        env::set_current_dir(prev_dir).expect("restore cwd");
        unsafe {
            env::set_var("PATH", old_path);
        }
        result.expect("real_pr finish edit");

        let gh_calls = fs::read_to_string(&gh_log).expect("read gh log");
        assert!(gh_calls.contains("pr edit -R danielbaustin/agent-design-language https://github.com/danielbaustin/agent-design-language/pull/1160 --title [v0.86][tools] Rust finish test edit --body-file"));
        assert!(gh_calls.contains("pr ready -R danielbaustin/agent-design-language https://github.com/danielbaustin/agent-design-language/pull/1160"));
    }

    #[test]
    fn finish_helper_paths_cover_nonempty_and_staged_checks() {
        let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
        let repo = unique_temp_dir("adl-pr-finish-helpers");
        init_git_repo(&repo);
        assert!(Command::new("git")
            .args(["config", "user.name", "Test User"])
            .current_dir(&repo)
            .status()
            .expect("git config")
            .success());
        assert!(Command::new("git")
            .args(["config", "user.email", "test@example.com"])
            .current_dir(&repo)
            .status()
            .expect("git config")
            .success());
        fs::write(repo.join("tracked.txt"), "base\n").expect("write base");
        assert!(Command::new("git")
            .args(["add", "tracked.txt"])
            .current_dir(&repo)
            .status()
            .expect("git add")
            .success());
        assert!(Command::new("git")
            .args(["commit", "-q", "-m", "init"])
            .current_dir(&repo)
            .status()
            .expect("git commit")
            .success());

        let missing = repo.join("missing.md");
        let empty = repo.join("empty.md");
        let filled = repo.join("filled.md");
        fs::write(&empty, " \n").expect("write empty");
        fs::write(&filled, "content\n").expect("write filled");
        assert!(!ensure_nonempty_file_path(&missing).expect("missing ok"));
        assert!(!ensure_nonempty_file_path(&empty).expect("empty ok"));
        assert!(ensure_nonempty_file_path(&filled).expect("filled ok"));

        assert!(!has_uncommitted_changes(&repo).expect("clean"));
        fs::write(repo.join("tracked.txt"), "changed\n").expect("modify tracked");
        assert!(has_uncommitted_changes(&repo).expect("dirty"));

        stage_selected_paths_rust(&repo, "tracked.txt").expect("stage");
        assert!(!staged_diff_is_empty(&repo).expect("staged diff"));
        assert!(!staged_gitignore_change_present(&repo).expect("no gitignore"));

        fs::write(repo.join(".gitignore"), "target\n").expect("write gitignore");
        stage_selected_paths_rust(&repo, ".gitignore").expect("stage gitignore");
        assert!(staged_gitignore_change_present(&repo).expect("gitignore change"));
    }

    #[test]
    fn finish_helper_paths_cover_ahead_count_and_batch_checks() {
        let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
        let temp = unique_temp_dir("adl-pr-finish-batch-checks");
        let origin = temp.join("origin.git");
        let repo = temp.join("repo");
        fs::create_dir_all(repo.join("adl")).expect("adl dir");
        fs::write(
            repo.join("adl/Cargo.toml"),
            "[package]\nname='adl'\nversion='0.1.0'\n",
        )
        .expect("cargo toml");
        init_git_repo(&repo);
        assert!(Command::new("git")
            .args(["config", "user.name", "Test User"])
            .current_dir(&repo)
            .status()
            .expect("git config")
            .success());
        assert!(Command::new("git")
            .args(["config", "user.email", "test@example.com"])
            .current_dir(&repo)
            .status()
            .expect("git config")
            .success());
        fs::write(repo.join("README.md"), "base\n").expect("readme");
        assert!(Command::new("git")
            .args(["add", "-A"])
            .current_dir(&repo)
            .status()
            .expect("git add")
            .success());
        assert!(Command::new("git")
            .args(["commit", "-q", "-m", "init"])
            .current_dir(&repo)
            .status()
            .expect("git commit")
            .success());
        assert!(Command::new("git")
            .args([
                "init",
                "--bare",
                "-q",
                path_str(&origin).expect("origin path"),
            ])
            .current_dir(&repo)
            .status()
            .expect("git init bare")
            .success());
        assert!(Command::new("git")
            .args([
                "remote",
                "set-url",
                "origin",
                path_str(&origin).expect("origin path"),
            ])
            .current_dir(&repo)
            .status()
            .expect("git remote set-url")
            .success());
        assert!(Command::new("git")
            .args(["branch", "-M", "main"])
            .current_dir(&repo)
            .status()
            .expect("git branch")
            .success());
        assert!(Command::new("git")
            .args(["push", "-q", "-u", "origin", "main"])
            .current_dir(&repo)
            .status()
            .expect("git push")
            .success());
        assert_eq!(commits_ahead_of_origin_main(&repo).expect("ahead count"), 0);

        fs::write(repo.join("README.md"), "ahead\n").expect("modify");
        assert!(Command::new("git")
            .args(["add", "README.md"])
            .current_dir(&repo)
            .status()
            .expect("git add")
            .success());
        assert!(Command::new("git")
            .args(["commit", "-q", "-m", "ahead"])
            .current_dir(&repo)
            .status()
            .expect("git commit")
            .success());
        assert_eq!(commits_ahead_of_origin_main(&repo).expect("ahead count"), 1);

        let bin_dir = temp.join("bin");
        fs::create_dir_all(&bin_dir).expect("bin dir");
        let cargo_log = temp.join("cargo.log");
        let cargo_path = bin_dir.join("cargo");
        write_executable(
            &cargo_path,
            &format!(
                "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\nexit 0\n",
                cargo_log.display()
            ),
        );
        let old_path = env::var("PATH").unwrap_or_default();
        unsafe {
            env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
        }
        run_batched_checks_rust(&repo).expect("batch checks");
        unsafe {
            env::set_var("PATH", old_path);
        }

        let cargo_calls = fs::read_to_string(&cargo_log).expect("cargo log");
        assert!(cargo_calls.contains("fmt --manifest-path"));
        assert!(cargo_calls.contains("clippy --manifest-path"));
        assert!(cargo_calls.contains("test --manifest-path"));
    }

    #[test]
    fn finish_helper_paths_cover_pr_lookup_and_closing_linkage() {
        let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
        let temp = unique_temp_dir("adl-pr-finish-gh-helpers");
        let bin_dir = temp.join("bin");
        fs::create_dir_all(&bin_dir).expect("bin dir");
        let gh_log = temp.join("gh.log");
        let gh_path = bin_dir.join("gh");
        write_executable(
            &gh_path,
            &format!(
                "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\nif [ \"$1 $2\" = 'pr list' ]; then\n  printf 'https://github.com/danielbaustin/agent-design-language/pull/1159\\n'\n  exit 0\nfi\nif [ \"$1 $2\" = 'pr view' ]; then\n  if printf '%s ' \"$@\" | grep -q 'closingIssuesReferences'; then\n    printf '1153\\n'\n  else\n    printf 'Closes #1153\\n'\n  fi\n  exit 0\nfi\nexit 1\n",
                gh_log.display()
            ),
        );
        let old_path = env::var("PATH").unwrap_or_default();
        unsafe {
            env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
        }

        let pr = current_pr_url(
            "danielbaustin/agent-design-language",
            "codex/1153-rust-finish-test",
        )
        .expect("pr url");
        assert_eq!(
            pr.as_deref(),
            Some("https://github.com/danielbaustin/agent-design-language/pull/1159")
        );
        assert!(pr_has_closing_linkage(
            "danielbaustin/agent-design-language",
            "https://github.com/danielbaustin/agent-design-language/pull/1159",
            1153
        )
        .expect("closing linkage"));
        ensure_pr_closing_linkage(
            "danielbaustin/agent-design-language",
            "https://github.com/danielbaustin/agent-design-language/pull/1159",
            1153,
            false,
        )
        .expect("ensure linkage");
        ensure_pr_closing_linkage(
            "danielbaustin/agent-design-language",
            "https://github.com/danielbaustin/agent-design-language/pull/1159",
            1153,
            true,
        )
        .expect("no close skips");

        unsafe {
            env::set_var("PATH", old_path);
        }
        let gh_calls = fs::read_to_string(&gh_log).expect("gh log");
        assert!(gh_calls.contains("pr list -R danielbaustin/agent-design-language --head codex/1153-rust-finish-test --state open --json url --jq .[0].url"));
    }

    #[test]
    fn real_pr_finish_rejects_main_and_no_changes_paths() {
        let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
        let temp = unique_temp_dir("adl-pr-finish-errors");
        let origin = temp.join("origin.git");
        let repo = temp.join("repo");
        fs::create_dir_all(&repo).expect("repo dir");
        copy_bootstrap_support_files(&repo);
        init_git_repo(&repo);
        assert!(Command::new("git")
            .args(["config", "user.name", "Test User"])
            .current_dir(&repo)
            .status()
            .expect("git config")
            .success());
        assert!(Command::new("git")
            .args(["config", "user.email", "test@example.com"])
            .current_dir(&repo)
            .status()
            .expect("git config")
            .success());
        assert!(Command::new("git")
            .args(["branch", "-M", "main"])
            .current_dir(&repo)
            .status()
            .expect("git branch")
            .success());
        fs::create_dir_all(repo.join("adl/src")).expect("adl src");
        fs::write(repo.join("adl/src/lib.rs"), "pub fn placeholder() {}\n").expect("write source");
        let issue_ref =
            IssueRef::new(1153, "v0.86".to_string(), "rust-finish-test".to_string()).expect("ref");
        let bundle_dir = issue_ref.task_bundle_dir_path(&repo);
        fs::create_dir_all(&bundle_dir).expect("bundle dir");
        fs::write(issue_ref.task_bundle_input_path(&repo), "# input\n").expect("input");
        write_completed_sor_fixture(
            &issue_ref.task_bundle_output_path(&repo),
            "codex/1153-rust-finish-test",
        );
        assert!(Command::new("git")
            .args(["add", "-A"])
            .current_dir(&repo)
            .status()
            .expect("git add")
            .success());
        assert!(Command::new("git")
            .args(["commit", "-q", "-m", "init on main"])
            .current_dir(&repo)
            .status()
            .expect("git commit")
            .success());
        assert!(Command::new("git")
            .args([
                "init",
                "--bare",
                "-q",
                path_str(&origin).expect("origin path"),
            ])
            .current_dir(&repo)
            .status()
            .expect("git init bare")
            .success());
        assert!(Command::new("git")
            .args([
                "remote",
                "set-url",
                "origin",
                path_str(&origin).expect("origin path"),
            ])
            .current_dir(&repo)
            .status()
            .expect("git remote set-url")
            .success());
        assert!(Command::new("git")
            .args(["push", "-q", "-u", "origin", "main"])
            .current_dir(&repo)
            .status()
            .expect("git push")
            .success());
        let issue_ref =
            IssueRef::new(1153, "v0.86".to_string(), "rust-finish-test".to_string()).expect("ref");
        let bundle_dir = issue_ref.task_bundle_dir_path(&repo);
        fs::create_dir_all(&bundle_dir).expect("bundle dir");
        let stp = issue_ref.task_bundle_stp_path(&repo);
        let input = issue_ref.task_bundle_input_path(&repo);
        let output = issue_ref.task_bundle_output_path(&repo);
        write_authored_issue_prompt(&repo, &issue_ref, "Example");
        fs::copy(issue_ref.issue_prompt_path(&repo), &stp).expect("seed stp");
        write_authored_sip(
            &input,
            &issue_ref,
            "Example",
            "codex/1153-rust-finish-test",
            &issue_ref.issue_prompt_path(&repo),
            &repo,
        );
        write_completed_sor_fixture(&output, "codex/1153-rust-finish-test");
        assert!(Command::new("git")
            .args(["add", "-A"])
            .current_dir(&repo)
            .status()
            .expect("git add")
            .success());
        assert!(Command::new("git")
            .args(["commit", "-q", "-m", "seed finish bundle"])
            .current_dir(&repo)
            .status()
            .expect("git commit")
            .success());
        assert!(Command::new("git")
            .args(["push", "-q", "origin", "main"])
            .current_dir(&repo)
            .status()
            .expect("git push")
            .success());

        let prev_dir = env::current_dir().expect("cwd");
        env::set_current_dir(&repo).expect("chdir");
        let main_err = real_pr(&[
            "finish".to_string(),
            "1153".to_string(),
            "--title".to_string(),
            "Example".to_string(),
            "--no-checks".to_string(),
            "--no-open".to_string(),
        ])
        .expect_err("main should be rejected");
        assert!(main_err.to_string().contains("refusing to run on main"));
        env::set_current_dir(prev_dir).expect("restore cwd");

        assert!(Command::new("git")
            .args(["checkout", "-q", "-b", "codex/1153-rust-finish-test"])
            .current_dir(&repo)
            .status()
            .expect("git checkout")
            .success());

        let prev_dir = env::current_dir().expect("cwd");
        env::set_current_dir(&repo).expect("chdir");
        let no_change_err = real_pr(&[
            "finish".to_string(),
            "1153".to_string(),
            "--title".to_string(),
            "Example".to_string(),
            "--input".to_string(),
            path_relative_to_repo(&repo, &input),
            "--output".to_string(),
            path_relative_to_repo(&repo, &output),
            "--no-checks".to_string(),
            "--no-open".to_string(),
        ])
        .expect_err("no changes should fail");
        env::set_current_dir(prev_dir).expect("restore cwd");
        assert!(no_change_err.to_string().contains("Nothing to PR."));
    }

    #[test]
    fn real_pr_finish_rejects_not_started_output_card_before_publication() {
        let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
        let temp = unique_temp_dir("adl-pr-finish-not-started");
        let origin = temp.join("origin.git");
        let repo = temp.join("repo");
        fs::create_dir_all(&repo).expect("repo dir");
        copy_bootstrap_support_files(&repo);
        init_git_repo(&repo);
        assert!(Command::new("git")
            .args(["config", "user.name", "Test User"])
            .current_dir(&repo)
            .status()
            .expect("git config")
            .success());
        assert!(Command::new("git")
            .args(["config", "user.email", "test@example.com"])
            .current_dir(&repo)
            .status()
            .expect("git config")
            .success());
        fs::create_dir_all(repo.join("adl/src")).expect("adl src");
        fs::write(repo.join("adl/src/lib.rs"), "pub fn placeholder() {}\n").expect("write source");
        assert!(Command::new("git")
            .args(["add", "-A"])
            .current_dir(&repo)
            .status()
            .expect("git add")
            .success());
        assert!(Command::new("git")
            .args(["commit", "-q", "-m", "init"])
            .current_dir(&repo)
            .status()
            .expect("git commit")
            .success());
        assert!(Command::new("git")
            .args(["branch", "-M", "main"])
            .current_dir(&repo)
            .status()
            .expect("git branch")
            .success());
        assert!(Command::new("git")
            .args([
                "init",
                "--bare",
                "-q",
                path_str(&origin).expect("origin path"),
            ])
            .current_dir(&repo)
            .status()
            .expect("git init bare")
            .success());
        assert!(Command::new("git")
            .args([
                "remote",
                "set-url",
                "origin",
                path_str(&origin).expect("origin path"),
            ])
            .current_dir(&repo)
            .status()
            .expect("git remote set-url")
            .success());
        assert!(Command::new("git")
            .args(["push", "-q", "-u", "origin", "main"])
            .current_dir(&repo)
            .status()
            .expect("git push")
            .success());
        assert!(Command::new("git")
            .args(["checkout", "-q", "-b", "codex/1156-output-card-guard"])
            .current_dir(&repo)
            .status()
            .expect("git checkout")
            .success());

        let issue_ref =
            IssueRef::new(1156, "v0.86".to_string(), "output-card-guard".to_string()).expect("ref");
        let bundle_dir = issue_ref.task_bundle_dir_path(&repo);
        fs::create_dir_all(&bundle_dir).expect("bundle dir");
        let stp = issue_ref.task_bundle_stp_path(&repo);
        let input = issue_ref.task_bundle_input_path(&repo);
        let output = issue_ref.task_bundle_output_path(&repo);
        write_authored_issue_prompt(&repo, &issue_ref, "[v0.86][tools] Output card guard");
        fs::copy(issue_ref.issue_prompt_path(&repo), &stp).expect("seed stp");
        write_authored_sip(
            &input,
            &issue_ref,
            "[v0.86][tools] Output card guard",
            "codex/1156-output-card-guard",
            &issue_ref.issue_prompt_path(&repo),
            &repo,
        );
        fs::write(
            &output,
            r#"# ADL Output Card

Task ID: issue-1156
Run ID: issue-1156
Version: v0.86
Title: output-card-guard
Branch: codex/1156-output-card-guard
Status: NOT_STARTED
"#,
        )
        .expect("write output");
        fs::write(
            repo.join("adl/src/lib.rs"),
            "pub fn placeholder() {}\npub fn changed() {}\n",
        )
        .expect("write change");

        let bin_dir = temp.join("bin");
        fs::create_dir_all(&bin_dir).expect("bin dir");
        let gh_log = temp.join("gh.log");
        let gh_path = bin_dir.join("gh");
        write_executable(
            &gh_path,
            &format!(
                "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\nexit 0\n",
                gh_log.display()
            ),
        );

        let old_path = env::var("PATH").unwrap_or_default();
        let prev_dir = env::current_dir().expect("cwd");
        unsafe {
            env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
        }
        env::set_current_dir(&repo).expect("chdir");

        let err = real_pr(&[
            "finish".to_string(),
            "1156".to_string(),
            "--title".to_string(),
            "[v0.86][tools] Output card guard".to_string(),
            "--paths".to_string(),
            "adl".to_string(),
            "--input".to_string(),
            path_relative_to_repo(&repo, &input),
            "--output".to_string(),
            path_relative_to_repo(&repo, &output),
            "--no-checks".to_string(),
            "--no-open".to_string(),
        ])
        .expect_err("NOT_STARTED output card should be rejected");

        env::set_current_dir(prev_dir).expect("restore cwd");
        unsafe {
            env::set_var("PATH", old_path);
        }

        assert!(err
            .to_string()
            .contains("output card is still bootstrap state (Status: NOT_STARTED)"));
        let gh_calls = fs::read_to_string(&gh_log).unwrap_or_default();
        assert!(
            !gh_calls.contains("pr create") && !gh_calls.contains("pr edit"),
            "finish should fail before any PR publication call"
        );
    }

    #[test]
    fn default_repo_falls_back_to_local_name_when_remote_and_gh_are_unavailable() {
        let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
        let repo = unique_temp_dir("adl-pr-default-repo-fallback");
        assert!(Command::new("git")
            .arg("init")
            .arg("-q")
            .current_dir(&repo)
            .status()
            .expect("git init")
            .success());

        let bin_dir = repo.join("bin");
        fs::create_dir_all(&bin_dir).expect("bin dir");
        write_executable(
            &bin_dir.join("gh"),
            "#!/usr/bin/env bash\nset -euo pipefail\nexit 1\n",
        );

        let old_path = env::var("PATH").unwrap_or_default();
        let prev_dir = env::current_dir().expect("cwd");
        unsafe {
            env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
        }
        env::set_current_dir(&repo).expect("chdir");

        let inferred = default_repo(&repo).expect("default repo");

        env::set_current_dir(prev_dir).expect("restore cwd");
        unsafe {
            env::set_var("PATH", old_path);
        }
        assert_eq!(
            inferred,
            format!("local/{}", repo.file_name().unwrap().to_string_lossy())
        );
    }

    #[test]
    fn default_repo_uses_gh_repo_when_remote_is_unparseable() {
        let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
        let repo = unique_temp_dir("adl-pr-default-repo-gh");
        assert!(Command::new("git")
            .arg("init")
            .arg("-q")
            .current_dir(&repo)
            .status()
            .expect("git init")
            .success());

        let bin_dir = repo.join("bin");
        fs::create_dir_all(&bin_dir).expect("bin dir");
        write_executable(
            &bin_dir.join("gh"),
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf 'owner/example\\n'\n",
        );

        let old_path = env::var("PATH").unwrap_or_default();
        let prev_dir = env::current_dir().expect("cwd");
        unsafe {
            env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
        }
        env::set_current_dir(&repo).expect("chdir");

        let inferred = default_repo(&repo).expect("default repo");

        env::set_current_dir(prev_dir).expect("restore cwd");
        unsafe {
            env::set_var("PATH", old_path);
        }
        assert_eq!(inferred, "owner/example");
    }

    #[test]
    fn fetch_origin_main_with_fallback_reuses_local_origin_main_and_errors_when_missing() {
        let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
        let temp = unique_temp_dir("adl-pr-fetch-fallback");
        let bin_dir = temp.join("bin");
        fs::create_dir_all(&bin_dir).expect("bin dir");
        write_executable(
            &bin_dir.join("git"),
            "#!/usr/bin/env bash\nset -euo pipefail\nif [ \"$1 $2 $3\" = 'fetch origin main' ]; then\n  exit 1\nfi\nif [ \"$1 $2 $3 $4\" = 'rev-parse --verify --quiet origin/main' ]; then\n  if [ \"${HAS_ORIGIN_MAIN:-0}\" = '1' ]; then\n    exit 0\n  fi\n  exit 1\nfi\nexit 1\n",
        );

        let old_path = env::var("PATH").unwrap_or_default();
        unsafe {
            env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
            env::set_var("HAS_ORIGIN_MAIN", "1");
        }
        fetch_origin_main_with_fallback().expect("should reuse local origin/main");

        unsafe {
            env::set_var("HAS_ORIGIN_MAIN", "0");
        }
        let err = fetch_origin_main_with_fallback().expect_err("missing origin/main should fail");
        unsafe {
            env::set_var("PATH", old_path);
            env::remove_var("HAS_ORIGIN_MAIN");
        }
        assert!(err
            .to_string()
            .contains("fetch origin main failed and origin/main is unavailable locally"));
    }

    #[test]
    fn ensure_worktree_for_branch_rejects_branch_checked_out_elsewhere() {
        let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
        let temp = unique_temp_dir("adl-pr-worktree-conflict");
        let bin_dir = temp.join("bin");
        fs::create_dir_all(&bin_dir).expect("bin dir");
        write_executable(
            &bin_dir.join("git"),
            "#!/usr/bin/env bash\nset -euo pipefail\nif [ \"$1 $2 $3\" = 'worktree list --porcelain' ]; then\n  cat <<'EOF'\nworktree /tmp/main\nHEAD deadbeef\nbranch refs/heads/main\n\nworktree /tmp/existing\nHEAD cafefood\nbranch refs/heads/codex/1153-test\nEOF\n  exit 0\nfi\nexit 1\n",
        );

        let old_path = env::var("PATH").unwrap_or_default();
        unsafe {
            env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
        }
        let err = ensure_worktree_for_branch(Path::new("/tmp/requested"), "codex/1153-test")
            .expect_err("conflicting worktree should fail");
        unsafe {
            env::set_var("PATH", old_path);
        }
        assert!(err.to_string().contains("already checked out in worktree"));
        assert!(err.to_string().contains("/tmp/existing"));
    }

    #[test]
    fn ensure_local_branch_exists_covers_existing_remote_and_new_branch_paths() {
        let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
        let temp = unique_temp_dir("adl-pr-ensure-branch");
        let bin_dir = temp.join("bin");
        fs::create_dir_all(&bin_dir).expect("bin dir");
        let git_log = temp.join("git.log");
        write_executable(
            &bin_dir.join("git"),
            &format!(
                "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\ncase \"$*\" in\n  'show-ref --verify --quiet refs/heads/codex/existing') exit 0 ;;\n  'show-ref --verify --quiet refs/heads/codex/remote') exit 1 ;;\n  'ls-remote --exit-code --heads origin codex/remote') exit 0 ;;\n  'branch --track codex/remote origin/codex/remote') exit 0 ;;\n  'show-ref --verify --quiet refs/heads/codex/new') exit 1 ;;\n  'ls-remote --exit-code --heads origin codex/new') exit 1 ;;\n  'branch codex/new origin/main') exit 0 ;;\n  *) exit 1 ;;\nesac\n",
                git_log.display()
            ),
        );

        let old_path = env::var("PATH").unwrap_or_default();
        unsafe {
            env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
        }

        ensure_local_branch_exists("codex/existing").expect("existing local branch");
        ensure_local_branch_exists("codex/remote").expect("remote tracking branch");
        ensure_local_branch_exists("codex/new").expect("new branch from origin/main");

        unsafe {
            env::set_var("PATH", old_path);
        }
        let log = fs::read_to_string(&git_log).expect("git log");
        assert!(log.contains("show-ref --verify --quiet refs/heads/codex/existing"));
        assert!(log.contains("branch --track codex/remote origin/codex/remote"));
        assert!(log.contains("branch codex/new origin/main"));
    }

    #[test]
    fn issue_version_prefers_labels_and_falls_back_to_title() {
        let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
        let temp = unique_temp_dir("adl-pr-issue-version");
        let bin_dir = temp.join("bin");
        fs::create_dir_all(&bin_dir).expect("bin dir");
        write_executable(
            &bin_dir.join("gh"),
            "#!/usr/bin/env bash\nset -euo pipefail\nif [ \"$1 $2 $3 $4\" = 'issue view 1153 -R' ]; then\n  case \"${GH_MODE:-labels}\" in\n    labels) printf 'track:roadmap\\nversion:v0.86\\n' ;;\n    title) printf '[v0.89][WP-15] Demo issue\\n' ;;\n    *) printf 'track:roadmap\\n' ;;\n  esac\n  exit 0\nfi\nexit 1\n",
        );

        let old_path = env::var("PATH").unwrap_or_default();
        unsafe {
            env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
            env::set_var("GH_MODE", "labels");
        }
        assert_eq!(
            issue_version(1153, "owner/repo").expect("labels"),
            Some("v0.86".to_string())
        );
        unsafe {
            env::set_var("GH_MODE", "title");
        }
        assert_eq!(
            issue_version(1153, "owner/repo").expect("title"),
            Some("v0.89".to_string())
        );
        unsafe {
            env::set_var("PATH", old_path);
            env::remove_var("GH_MODE");
        }
    }

    #[test]
    fn current_pr_url_filters_empty_and_null_results() {
        let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
        let temp = unique_temp_dir("adl-pr-current-url");
        let bin_dir = temp.join("bin");
        fs::create_dir_all(&bin_dir).expect("bin dir");
        write_executable(
            &bin_dir.join("gh"),
            "#!/usr/bin/env bash\nset -euo pipefail\ncase \"${GH_PR_LIST_MODE:-url}\" in\n  null) printf 'null\\n' ;;\n  empty) printf '\\n' ;;\n  *) printf 'https://github.com/example/repo/pull/1\\n' ;;\nesac\n",
        );

        let old_path = env::var("PATH").unwrap_or_default();
        unsafe {
            env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
            env::set_var("GH_PR_LIST_MODE", "url");
        }
        assert_eq!(
            current_pr_url("owner/repo", "codex/test").expect("url"),
            Some("https://github.com/example/repo/pull/1".to_string())
        );
        unsafe {
            env::set_var("GH_PR_LIST_MODE", "null");
        }
        assert_eq!(
            current_pr_url("owner/repo", "codex/test").expect("null"),
            None
        );
        unsafe {
            env::set_var("GH_PR_LIST_MODE", "empty");
        }
        assert_eq!(
            current_pr_url("owner/repo", "codex/test").expect("empty"),
            None
        );
        unsafe {
            env::set_var("PATH", old_path);
            env::remove_var("GH_PR_LIST_MODE");
        }
    }

    #[test]
    fn branch_checked_out_worktree_path_returns_none_without_match() {
        let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
        let temp = unique_temp_dir("adl-pr-worktree-none");
        let bin_dir = temp.join("bin");
        fs::create_dir_all(&bin_dir).expect("bin dir");
        write_executable(
            &bin_dir.join("git"),
            "#!/usr/bin/env bash\nset -euo pipefail\nif [ \"$1 $2 $3\" = 'worktree list --porcelain' ]; then\n  cat <<'EOF'\nworktree /tmp/main\nHEAD deadbeef\nbranch refs/heads/main\nEOF\n  exit 0\nfi\nexit 1\n",
        );

        let old_path = env::var("PATH").unwrap_or_default();
        unsafe {
            env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
        }
        assert_eq!(
            branch_checked_out_worktree_path("codex/missing").expect("none"),
            None
        );
        unsafe {
            env::set_var("PATH", old_path);
        }
    }

    #[test]
    fn ensure_worktree_for_branch_reuses_matching_path_and_creates_new_one() {
        let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
        let temp = unique_temp_dir("adl-pr-worktree-reuse-create");
        let bin_dir = temp.join("bin");
        fs::create_dir_all(&bin_dir).expect("bin dir");
        let git_log = temp.join("git.log");
        write_executable(
            &bin_dir.join("git"),
            &format!(
                "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\nif [ \"$1 $2 $3\" = 'worktree list --porcelain' ]; then\n  if [ \"${{WT_MODE:-reuse}}\" = 'reuse' ]; then\n    cat <<'EOF'\nworktree /tmp/reuse-me\nHEAD deadbeef\nbranch refs/heads/codex/reuse\nEOF\n    exit 0\n  fi\n  printf 'worktree /tmp/main\\nHEAD deadbeef\\nbranch refs/heads/main\\n'\n  exit 0\nfi\nif [ \"$1 $2 $3\" = 'worktree add /tmp/create-me' ]; then\n  mkdir -p /tmp/create-me\n  exit 0\nfi\nexit 1\n",
                git_log.display()
            ),
        );

        let old_path = env::var("PATH").unwrap_or_default();
        unsafe {
            env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
            env::set_var("WT_MODE", "reuse");
        }
        ensure_worktree_for_branch(Path::new("/tmp/reuse-me"), "codex/reuse").expect("reuse");

        unsafe {
            env::set_var("WT_MODE", "create");
        }
        let create_path = Path::new("/tmp/create-me");
        let _ = fs::remove_dir_all(create_path);
        ensure_worktree_for_branch(create_path, "codex/create").expect("create");

        unsafe {
            env::set_var("PATH", old_path);
            env::remove_var("WT_MODE");
        }
        let log = fs::read_to_string(&git_log).expect("git log");
        assert!(log.contains("worktree add /tmp/create-me codex/create"));
    }

    #[test]
    fn validate_issue_prompt_exists_rejects_missing_file() {
        let missing = unique_temp_dir("adl-pr-missing-prompt").join("missing.md");
        let err = validate_issue_prompt_exists(&missing).expect_err("missing prompt");
        assert!(err
            .to_string()
            .contains("missing canonical source issue prompt"));
    }

    #[test]
    fn resolve_issue_prompt_path_accepts_legacy_issue_bodies_location() {
        let repo = unique_temp_dir("adl-pr-legacy-prompt-path");
        let issue_ref = IssueRef::new(1197, "v0.86".to_string(), "legacy-ready-source".to_string())
            .expect("issue ref");
        let legacy = issue_ref.legacy_issue_prompt_path(&repo);
        fs::create_dir_all(legacy.parent().expect("legacy parent")).expect("legacy dir");
        fs::write(
            &legacy,
            "---\nissue_card_schema: adl.issue.v1\n---\n\n# x\n",
        )
        .expect("legacy");

        let resolved = resolve_issue_prompt_path(&repo, &issue_ref).expect("resolved");
        assert_eq!(resolved, legacy);
    }

    #[test]
    fn real_pr_start_rejects_missing_slug_or_empty_sanitized_title_in_no_fetch_mode() {
        let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
        let repo = unique_temp_dir("adl-pr-start-preconditions");
        init_git_repo(&repo);
        let prev_dir = env::current_dir().expect("cwd");
        env::set_current_dir(&repo).expect("chdir");

        let missing_slug = real_pr(&[
            "start".to_string(),
            "1152".to_string(),
            "--no-fetch-issue".to_string(),
        ])
        .expect_err("missing slug should fail");
        assert!(missing_slug
            .to_string()
            .contains("start: --slug is required when --no-fetch-issue is set"));

        let bad_title = real_pr(&[
            "start".to_string(),
            "1152".to_string(),
            "--title".to_string(),
            "!!!".to_string(),
            "--no-fetch-issue".to_string(),
        ])
        .expect_err("empty sanitized title should fail");
        env::set_current_dir(prev_dir).expect("restore cwd");
        assert!(bad_title
            .to_string()
            .contains("start: --title produced empty slug after sanitization"));
    }

    #[test]
    fn real_pr_ready_accepts_started_issue_when_output_branch_is_bootstrap_placeholder() {
        let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
        let repo = unique_temp_dir("adl-pr-ready-branch-placeholder");
        let origin = repo.join("origin.git");
        init_git_repo(&repo);
        copy_bootstrap_support_files(&repo);
        assert!(Command::new("git")
            .args(["config", "user.name", "Test User"])
            .current_dir(&repo)
            .status()
            .expect("git config")
            .success());
        assert!(Command::new("git")
            .args(["config", "user.email", "test@example.com"])
            .current_dir(&repo)
            .status()
            .expect("git config")
            .success());
        fs::write(repo.join("README.md"), "ready branch placeholder\n").expect("write readme");
        assert!(Command::new("git")
            .args(["add", "-A"])
            .current_dir(&repo)
            .status()
            .expect("git add")
            .success());
        assert!(Command::new("git")
            .args(["commit", "-q", "-m", "init"])
            .current_dir(&repo)
            .status()
            .expect("git commit")
            .success());
        assert!(Command::new("git")
            .args(["branch", "-M", "main"])
            .current_dir(&repo)
            .status()
            .expect("git branch")
            .success());
        assert!(Command::new("git")
            .args([
                "init",
                "--bare",
                "-q",
                path_str(&origin).expect("origin path"),
            ])
            .current_dir(&repo)
            .status()
            .expect("git init bare")
            .success());
        assert!(Command::new("git")
            .args([
                "remote",
                "set-url",
                "origin",
                path_str(&origin).expect("origin path"),
            ])
            .current_dir(&repo)
            .status()
            .expect("git remote set-url")
            .success());
        assert!(Command::new("git")
            .args(["push", "-q", "-u", "origin", "main"])
            .current_dir(&repo)
            .status()
            .expect("git push")
            .success());
        assert!(Command::new("git")
            .args(["fetch", "-q", "origin", "main"])
            .current_dir(&repo)
            .status()
            .expect("git fetch")
            .success());

        let prev_dir = env::current_dir().expect("cwd");
        env::set_current_dir(&repo).expect("chdir");
        let issue_ref =
            IssueRef::new(1198, "v0.86", "ready-branch-placeholder").expect("issue ref");
        write_authored_issue_prompt(&repo, &issue_ref, "[v0.86][tools] Ready branch placeholder");

        real_pr(&[
            "start".to_string(),
            "1198".to_string(),
            "--slug".to_string(),
            "ready-branch-placeholder".to_string(),
            "--title".to_string(),
            "[v0.86][tools] Ready branch placeholder".to_string(),
            "--no-fetch-issue".to_string(),
            "--version".to_string(),
            "v0.86".to_string(),
        ])
        .expect("real_pr start");

        let root_output = issue_ref.task_bundle_output_path(&repo);
        let worktree = issue_ref.default_worktree_path(&repo, None);
        let root_sip = issue_ref.task_bundle_input_path(&repo);
        let wt_sip = issue_ref.task_bundle_input_path(&worktree);
        write_authored_sip(
            &root_sip,
            &issue_ref,
            "[v0.86][tools] Ready branch placeholder",
            "codex/1198-ready-branch-placeholder",
            &issue_ref.issue_prompt_path(&repo),
            &repo,
        );
        write_authored_sip(
            &wt_sip,
            &issue_ref,
            "[v0.86][tools] Ready branch placeholder",
            "codex/1198-ready-branch-placeholder",
            &issue_ref.issue_prompt_path(&worktree),
            &worktree,
        );
        let wt_output = issue_ref.task_bundle_output_path(&worktree);
        for path in [&root_output, &wt_output] {
            let text = fs::read_to_string(path).expect("sor");
            fs::write(
                path,
                text.replace(
                    "Branch: codex/1198-ready-branch-placeholder",
                    "Branch: TBD (run pr.sh start 1198)",
                ),
            )
            .expect("rewrite sor");
        }

        let ready = real_pr(&[
            "ready".to_string(),
            "1198".to_string(),
            "--slug".to_string(),
            "ready-branch-placeholder".to_string(),
            "--no-fetch-issue".to_string(),
            "--version".to_string(),
            "v0.86".to_string(),
        ]);

        env::set_current_dir(prev_dir).expect("restore cwd");
        ready.expect("ready should accept placeholder output branch");
    }

    #[test]
    fn bootstrap_stub_reason_detects_issue_prompt_and_sip_templates() {
        let issue_prompt = "# x\n\n## Summary\n\nBootstrap-generated local source prompt for issue #1.\n\n## Goal\n\nTranslate the GitHub issue into the canonical local STP/task-bundle flow and refine this prompt before execution as needed.\n\n## Acceptance Criteria\n\n- something\n";
        assert_eq!(
            bootstrap_stub_reason(issue_prompt, PromptSurfaceKind::IssuePrompt),
            Some("bootstrap-generated issue prompt template text")
        );

        let sip = "# ADL Input Card\n\n## Goal\n\nReal goal\n\n## Acceptance Criteria\n\n- one\n\n## Required Outcome\n\n- State whether this issue must ship code, docs, tests, demo artifacts, or a combination.\n";
        assert_eq!(
            bootstrap_stub_reason(sip, PromptSurfaceKind::Sip),
            Some("unrefined SIP template guidance")
        );
    }

    #[cfg(unix)]
    #[test]
    fn ensure_git_metadata_writable_rejects_unwritable_git_dir() {
        use std::os::unix::fs::PermissionsExt;

        let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
        let repo = unique_temp_dir("adl-pr-git-metadata-write");
        init_git_repo(&repo);
        let prev_dir = env::current_dir().expect("cwd");
        env::set_current_dir(&repo).expect("chdir");

        let git_dir = repo.join(".git");
        let refs_dir = git_dir.join("refs");
        let heads_dir = refs_dir.join("heads");
        let git_mode = fs::metadata(&git_dir)
            .expect("git metadata")
            .permissions()
            .mode();
        let refs_mode = fs::metadata(&refs_dir)
            .expect("refs metadata")
            .permissions()
            .mode();
        let heads_mode = fs::metadata(&heads_dir)
            .expect("heads metadata")
            .permissions()
            .mode();

        fs::set_permissions(&git_dir, fs::Permissions::from_mode(0o555)).expect("chmod git");
        fs::set_permissions(&refs_dir, fs::Permissions::from_mode(0o555)).expect("chmod refs");
        fs::set_permissions(&heads_dir, fs::Permissions::from_mode(0o555)).expect("chmod heads");

        let err = ensure_git_metadata_writable().expect_err("unwritable git dir should fail");

        fs::set_permissions(&heads_dir, fs::Permissions::from_mode(heads_mode))
            .expect("restore heads");
        fs::set_permissions(&refs_dir, fs::Permissions::from_mode(refs_mode))
            .expect("restore refs");
        fs::set_permissions(&git_dir, fs::Permissions::from_mode(git_mode)).expect("restore git");
        env::set_current_dir(prev_dir).expect("restore cwd");

        assert!(err.to_string().contains("git metadata directory"));
        assert!(err
            .to_string()
            .contains("restore write access to git metadata before rerunning"));
    }

    #[test]
    fn ensure_primary_checkout_on_main_handles_dirty_and_clean_non_main_states() {
        let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
        let repo = unique_temp_dir("adl-pr-primary-main");
        init_git_repo(&repo);
        assert!(Command::new("git")
            .args(["config", "user.name", "Test User"])
            .current_dir(&repo)
            .status()
            .expect("git config")
            .success());
        assert!(Command::new("git")
            .args(["config", "user.email", "test@example.com"])
            .current_dir(&repo)
            .status()
            .expect("git config")
            .success());
        fs::write(repo.join("README.md"), "hello\n").expect("write readme");
        assert!(Command::new("git")
            .args(["add", "README.md"])
            .current_dir(&repo)
            .status()
            .expect("git add")
            .success());
        assert!(Command::new("git")
            .args(["commit", "-q", "-m", "init"])
            .current_dir(&repo)
            .status()
            .expect("git commit")
            .success());
        assert!(Command::new("git")
            .args(["branch", "-M", "main"])
            .current_dir(&repo)
            .status()
            .expect("git branch")
            .success());
        assert!(Command::new("git")
            .args(["checkout", "-q", "-b", "codex/1153-test"])
            .current_dir(&repo)
            .status()
            .expect("git checkout")
            .success());

        fs::write(repo.join("README.md"), "dirty\n").expect("dirty write");
        let err = ensure_primary_checkout_on_main(&repo).expect_err("dirty non-main should fail");
        assert!(err.to_string().contains("with local changes"));

        assert!(Command::new("git")
            .args(["restore", "README.md"])
            .current_dir(&repo)
            .status()
            .expect("git restore")
            .success());
        ensure_primary_checkout_on_main(&repo).expect("clean non-main should switch");
        let branch = current_branch(&repo).expect("branch");
        assert_eq!(branch, "main");
    }

    #[test]
    fn ensure_bootstrap_cards_creates_bundle_and_compat_links() {
        let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
        let repo = unique_temp_dir("adl-pr-bootstrap-cards");
        init_git_repo(&repo);
        copy_bootstrap_support_files(&repo);

        let issue_ref =
            IssueRef::new(1153, "v0.86".to_string(), "rust-finish-test".to_string()).expect("ref");
        let source_path = issue_ref.issue_prompt_path(&repo);
        fs::create_dir_all(source_path.parent().expect("source parent")).expect("mkdir");
        fs::write(
            &source_path,
            "---\ntitle: \"[v0.86][tools] Bootstrap cards\"\nlabels:\n  - \"track:roadmap\"\nissue_number: 1153\n---\n\n# Body\n",
        )
        .expect("write source");

        let (bundle_input, bundle_output) = ensure_bootstrap_cards(
            &repo,
            &issue_ref,
            "[v0.86][tools] Bootstrap cards",
            "codex/1153-rust-finish-test",
            &source_path,
        )
        .expect("bootstrap cards");

        assert!(bundle_input.is_file());
        assert!(bundle_output.is_file());
        let compat_input = card_input_path(&resolve_cards_root(&repo, None), 1153);
        let compat_output = card_output_path(&resolve_cards_root(&repo, None), 1153);
        assert!(compat_input.symlink_metadata().is_ok());
        assert!(compat_output.symlink_metadata().is_ok());
        assert_eq!(
            field_line_value(&bundle_input, "Branch").expect("input branch"),
            "codex/1153-rust-finish-test"
        );
        assert_eq!(
            field_line_value(&bundle_output, "Status").expect("output status"),
            "IN_PROGRESS"
        );
    }
}
