use anyhow::{anyhow, bail, Context, Result};
use serde::Deserialize;
use std::fs;
#[cfg(unix)]
use std::os::unix::fs as unix_fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use ::adl::control_plane::{
    card_input_path, card_output_path, resolve_cards_root, sanitize_slug, IssueRef,
};

const DEFAULT_VERSION: &str = "v0.86";
const DEFAULT_NEW_LABELS: &str = "track:roadmap,type:task,area:tools";

#[derive(Debug, Clone, PartialEq, Eq)]
struct InitArgs {
    issue: u32,
    slug: Option<String>,
    title_arg: Option<String>,
    no_fetch_issue: bool,
    version: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum CreateMode {
    Reconcile {
        issue: u32,
        stp_path: PathBuf,
    },
    Create {
        title: String,
        slug: Option<String>,
        body: Option<String>,
        body_file: Option<PathBuf>,
        labels: String,
        version: Option<String>,
        no_start: bool,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct StartArgs {
    issue: u32,
    prefix: String,
    slug: Option<String>,
    title_arg: Option<String>,
    no_fetch_issue: bool,
    version: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ReadyArgs {
    issue: u32,
    slug: Option<String>,
    version: Option<String>,
    no_fetch_issue: bool,
}

#[derive(Debug, Deserialize)]
struct IssuePromptFrontMatter {
    title: String,
    labels: Vec<String>,
    issue_number: u32,
}

#[derive(Debug)]
struct IssuePromptDoc {
    front_matter: IssuePromptFrontMatter,
    body: String,
}

pub(crate) fn real_pr(args: &[String]) -> Result<()> {
    let Some(subcommand) = args.first().map(|s| s.as_str()) else {
        bail!("pr requires a subcommand: init | create | start | ready");
    };

    match subcommand {
        "init" => real_pr_init(&args[1..]),
        "create" => real_pr_create(&args[1..]),
        "start" => real_pr_start(&args[1..]),
        "ready" => real_pr_ready(&args[1..]),
        other => bail!("unknown pr subcommand: {other}"),
    }
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
    let managed_root = std::env::var_os("ADL_WORKTREE_ROOT").map(PathBuf::from);
    let worktree_path = issue_ref.default_worktree_path(&repo_root, managed_root.as_deref());

    eprintln!("• Target branch: {branch}");
    eprintln!("• Target worktree: {}", worktree_path.display());

    fetch_origin_main_with_fallback()?;
    ensure_local_branch_exists(&branch)?;
    ensure_worktree_for_branch(&worktree_path, &branch)?;
    ensure_primary_checkout_on_main(&repo_root)?;

    let source_path =
        ensure_source_issue_prompt(&repo_root, &repo, &issue_ref, &title, None, &version)?;
    validate_issue_prompt_exists(&source_path)?;
    validate_bootstrap_stp(&repo_root, &source_path)?;

    let root_stp = ensure_task_bundle_stp(&repo_root, &issue_ref, &source_path)?;
    let worktree_source = ensure_local_issue_prompt_copy(&worktree_path, &issue_ref, &source_path)?;
    let worktree_stp = ensure_task_bundle_stp(&worktree_path, &issue_ref, &worktree_source)?;

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
                    .ok_or_else(|| {
                        anyhow!("ready: could not infer slug; pass --slug or run start first")
                    })?,
            )
        };

    let issue_ref = IssueRef::new(parsed.issue, version.clone(), slug.clone())?;
    let branch = issue_ref.branch_name("codex");
    let managed_root = std::env::var_os("ADL_WORKTREE_ROOT").map(PathBuf::from);
    let worktree_path = issue_ref.default_worktree_path(&repo_root, managed_root.as_deref());
    let source_path = issue_ref.issue_prompt_path(&repo_root);
    let root_stp = issue_ref.task_bundle_stp_path(&repo_root);
    let wt_stp = issue_ref.task_bundle_stp_path(&worktree_path);

    let root_bundle_input = issue_ref.task_bundle_input_path(&repo_root);
    let root_bundle_output = issue_ref.task_bundle_output_path(&repo_root);
    let wt_bundle_input = issue_ref.task_bundle_input_path(&worktree_path);
    let wt_bundle_output = issue_ref.task_bundle_output_path(&worktree_path);

    validate_issue_prompt_exists(&source_path)?;
    validate_bootstrap_stp(&repo_root, &source_path)?;
    if !root_stp.is_file() {
        bail!("ready: missing root stp: {}", root_stp.display());
    }
    validate_bootstrap_stp(&repo_root, &root_stp)?;
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
    validate_bootstrap_cards(
        &repo_root,
        parsed.issue,
        &branch,
        &root_bundle_input,
        &root_bundle_output,
    )?;
    validate_bootstrap_cards(
        &worktree_path,
        parsed.issue,
        &branch,
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

fn real_pr_init(args: &[String]) -> Result<()> {
    let parsed = parse_init_args(args)?;
    let repo_root = repo_root()?;
    let repo = default_repo(&repo_root)?;

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
        title = gh_issue_title(parsed.issue, &repo)?.unwrap_or_default();
    }
    if slug.is_empty() {
        if parsed.no_fetch_issue {
            bail!("init: --slug is required when --no-fetch-issue is set");
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
    let source_path =
        ensure_source_issue_prompt(&repo_root, &repo, &issue_ref, &title, None, &version)?;
    validate_issue_prompt_exists(&source_path)?;

    let stp_path = issue_ref.task_bundle_stp_path(&repo_root);
    let bundle_dir = issue_ref.task_bundle_dir_path(&repo_root);
    if stp_path.is_file() {
        eprintln!("• STP already exists: {}", stp_path.display());
    } else {
        eprintln!("• Initializing task bundle: {}", bundle_dir.display());
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
    }

    if bundle_dir.join("sip.md").exists() || bundle_dir.join("sor.md").exists() {
        eprintln!("• SIP/SOR already exist; pr init leaves them untouched.");
    }

    println!("• Initialized:");
    println!(
        "  STP      {}",
        path_relative_to_repo(&repo_root, &stp_path)
    );
    println!(
        "  BUNDLE   {}",
        path_relative_to_repo(&repo_root, &bundle_dir)
    );
    println!(
        "  SOURCE   {}",
        path_relative_to_repo(&repo_root, &source_path)
    );
    println!("  CONTRACT minimum v0.85 init = task-bundle directory + validated stp.md only");
    println!("  STATE    ISSUE_AND_STP_READY");
    eprintln!("• Done.");
    Ok(())
}

fn real_pr_create(args: &[String]) -> Result<()> {
    let mode = parse_create_args(args)?;
    let repo_root = repo_root()?;
    let repo = default_repo(&repo_root)?;

    match mode {
        CreateMode::Reconcile { issue, stp_path } => {
            let doc = load_issue_prompt(&stp_path)?;
            if doc.front_matter.issue_number != issue {
                bail!(
                    "create: STP issue_number ({}) does not match requested issue ({issue})",
                    doc.front_matter.issue_number
                );
            }
            reconcile_issue(issue, &repo, &doc)?;
            println!("ISSUE_NUM={issue}");
            println!("STP_PATH={}", path_relative_to_repo(&repo_root, &stp_path));
            println!("MODE=reconcile");
            Ok(())
        }
        CreateMode::Create {
            title,
            slug,
            body,
            body_file,
            labels,
            version,
            no_start,
        } => {
            let slug = sanitize_slug(slug.unwrap_or_else(|| title.clone()));
            if slug.is_empty() {
                bail!("new: slug is empty after sanitization");
            }

            let issue_body = resolve_issue_body(body, body_file.as_deref())?;
            if issue_body.contains("/Users/") || issue_body.contains("/home/") {
                bail!("new: issue body contains disallowed absolute host path");
            }

            let version = version
                .or_else(|| version_from_labels_csv(&labels))
                .or_else(|| version_from_title(&title))
                .unwrap_or_else(|| DEFAULT_VERSION.to_string());
            let labels_csv = normalize_labels_csv(&labels, &version);

            let issue_url = gh_issue_create(&title, &issue_body, &labels_csv)?;
            let issue_num = parse_issue_number_from_url(&issue_url)?;

            println!("ISSUE_URL={issue_url}");
            println!("ISSUE_NUM={issue_num}");
            println!("STATE=ISSUE_CREATED");

            let issue_ref = IssueRef::new(issue_num, version.clone(), slug.clone())?;
            let source_path = ensure_source_issue_prompt(
                &repo_root,
                &repo,
                &issue_ref,
                &title,
                Some(&labels_csv),
                &version,
            )?;
            println!(
                "SOURCE_PATH={}",
                path_relative_to_repo(&repo_root, &source_path)
            );

            if no_start {
                println!("START_STATE=SKIPPED");
                return Ok(());
            }

            let status = Command::new("bash")
                .arg("./adl/tools/pr.sh")
                .arg("start")
                .arg(issue_num.to_string())
                .arg("--slug")
                .arg(slug.clone())
                .arg("--title")
                .arg(title.clone())
                .arg("--version")
                .arg(version.clone())
                .current_dir(&repo_root)
                .status()
                .with_context(|| "failed to delegate create->start handoff to pr.sh")?;
            if !status.success() {
                println!("START_STATE=FAILED");
                bail!(
                    "create: issue created but start failed; issue #{} exists and source prompt is at {}",
                    issue_num,
                    path_relative_to_repo(&repo_root, &source_path)
                );
            }
            println!("START_STATE=STARTED");
            println!("BRANCH=codex/{}-{}", issue_num, slug);
            Ok(())
        }
    }
}

fn parse_init_args(args: &[String]) -> Result<InitArgs> {
    let issue_raw = args
        .first()
        .ok_or_else(|| anyhow!("init: missing <issue> number"))?;
    let issue = issue_raw
        .parse::<u32>()
        .with_context(|| format!("invalid issue number: {issue_raw}"))?;
    let mut parsed = InitArgs {
        issue,
        slug: None,
        title_arg: None,
        no_fetch_issue: false,
        version: None,
    };

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--slug" => {
                parsed.slug = Some(require_value(args, i, "init", "--slug")?);
                i += 1;
            }
            "--title" => {
                parsed.title_arg = Some(require_value(args, i, "init", "--title")?);
                i += 1;
            }
            "--no-fetch-issue" => parsed.no_fetch_issue = true,
            "--version" => {
                parsed.version = Some(require_value(args, i, "init", "--version")?);
                i += 1;
            }
            other => bail!("init: unknown arg: {other}"),
        }
        i += 1;
    }
    Ok(parsed)
}

fn parse_create_args(args: &[String]) -> Result<CreateMode> {
    if let Some(first) = args.first() {
        if let Ok(issue) = first.parse::<u32>() {
            let mut stp_path: Option<PathBuf> = None;
            let mut i = 1;
            while i < args.len() {
                match args[i].as_str() {
                    "--stp" => {
                        stp_path = Some(PathBuf::from(require_value(args, i, "create", "--stp")?));
                        i += 1;
                    }
                    other => bail!("create: unknown arg: {other}"),
                }
                i += 1;
            }
            let stp_path =
                stp_path.ok_or_else(|| anyhow!("create: --stp is required for reconcile mode"))?;
            return Ok(CreateMode::Reconcile { issue, stp_path });
        }
    }

    let mut title: Option<String> = None;
    let mut slug: Option<String> = None;
    let mut body: Option<String> = None;
    let mut body_file: Option<PathBuf> = None;
    let mut labels = DEFAULT_NEW_LABELS.to_string();
    let mut version: Option<String> = None;
    let mut no_start = false;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--title" => {
                title = Some(require_value(args, i, "new", "--title")?);
                i += 1;
            }
            "--slug" => {
                slug = Some(require_value(args, i, "new", "--slug")?);
                i += 1;
            }
            "--body" => {
                body = Some(require_value(args, i, "new", "--body")?);
                i += 1;
            }
            "--body-file" => {
                body_file = Some(PathBuf::from(require_value(args, i, "new", "--body-file")?));
                i += 1;
            }
            "--labels" => {
                labels = require_value(args, i, "new", "--labels")?;
                i += 1;
            }
            "--version" => {
                version = Some(require_value(args, i, "new", "--version")?);
                i += 1;
            }
            "--no-start" => no_start = true,
            other => bail!("new: unknown arg: {other}"),
        }
        i += 1;
    }

    let title = title.ok_or_else(|| anyhow!("new: --title is required"))?;
    if body.is_some() && body_file.is_some() {
        bail!("new: pass only one of --body or --body-file");
    }

    Ok(CreateMode::Create {
        title,
        slug,
        body,
        body_file,
        labels,
        version,
        no_start,
    })
}

fn parse_start_args(args: &[String]) -> Result<StartArgs> {
    let issue_raw = args
        .first()
        .ok_or_else(|| anyhow!("start: missing <issue> number"))?;
    let issue = issue_raw
        .parse::<u32>()
        .with_context(|| format!("invalid issue number: {issue_raw}"))?;
    let mut parsed = StartArgs {
        issue,
        prefix: "codex".to_string(),
        slug: None,
        title_arg: None,
        no_fetch_issue: false,
        version: None,
    };
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--prefix" => {
                parsed.prefix = require_value(args, i, "start", "--prefix")?;
                i += 1;
            }
            "--slug" => {
                parsed.slug = Some(require_value(args, i, "start", "--slug")?);
                i += 1;
            }
            "--title" => {
                parsed.title_arg = Some(require_value(args, i, "start", "--title")?);
                i += 1;
            }
            "--version" => {
                parsed.version = Some(require_value(args, i, "start", "--version")?);
                i += 1;
            }
            "--no-fetch-issue" => parsed.no_fetch_issue = true,
            other => bail!("start: unknown arg: {other}"),
        }
        i += 1;
    }
    Ok(parsed)
}

fn parse_ready_args(args: &[String]) -> Result<ReadyArgs> {
    let issue_raw = args
        .first()
        .ok_or_else(|| anyhow!("ready: missing <issue> number"))?;
    let issue = issue_raw
        .parse::<u32>()
        .with_context(|| format!("invalid issue number: {issue_raw}"))?;
    let mut parsed = ReadyArgs {
        issue,
        slug: None,
        version: None,
        no_fetch_issue: false,
    };
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--slug" => {
                parsed.slug = Some(require_value(args, i, "ready", "--slug")?);
                i += 1;
            }
            "--version" => {
                parsed.version = Some(require_value(args, i, "ready", "--version")?);
                i += 1;
            }
            "--no-fetch-issue" => parsed.no_fetch_issue = true,
            other => bail!("ready: unknown arg: {other}"),
        }
        i += 1;
    }
    Ok(parsed)
}

fn require_value(args: &[String], index: usize, cmd: &str, flag: &str) -> Result<String> {
    args.get(index + 1)
        .cloned()
        .ok_or_else(|| anyhow!("{cmd}: {flag} requires a value"))
}

fn repo_root() -> Result<PathBuf> {
    let out = run_capture("git", &["rev-parse", "--show-toplevel"])?;
    Ok(PathBuf::from(out.trim()))
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

fn resolve_issue_scope_and_slug_from_local_state(
    repo_root: &Path,
    issue: u32,
) -> Result<Option<(String, String)>> {
    let issue_dir = format!("issue-{:04}__", issue);
    let adl_root = repo_root.join(".adl");
    if !adl_root.is_dir() {
        return Ok(None);
    }
    let mut matches = Vec::new();
    for scope_entry in fs::read_dir(&adl_root)? {
        let scope_entry = scope_entry?;
        let tasks = scope_entry.path().join("tasks");
        if !tasks.is_dir() {
            continue;
        }
        for entry in fs::read_dir(&tasks)? {
            let entry = entry?;
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if let Some(slug) = name.strip_prefix(&issue_dir) {
                matches.push((
                    scope_entry.file_name().to_string_lossy().to_string(),
                    slug.to_string(),
                ));
            }
        }
    }
    matches.sort();
    Ok(matches.into_iter().next())
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

fn render_generated_issue_prompt(
    issue: u32,
    slug: &str,
    title: &str,
    labels_csv: &str,
    issue_url: &str,
) -> String {
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
        "---\nissue_card_schema: adl.issue.v1\nwp: \"{wp}\"\nslug: \"{slug}\"\ntitle: \"{title}\"\nlabels:\n{label_lines}\nissue_number: {issue}\nstatus: \"draft\"\naction: \"edit\"\ndepends_on: []\nmilestone_sprint: \"Pending sprint assignment\"\nrequired_outcome_type:\n  - \"{outcome_type}\"\nrepo_inputs: []\ncanonical_files: []\ndemo_required: false\ndemo_names: []\nissue_graph_notes:\n  - \"Bootstrap-generated from GitHub issue metadata because no canonical local issue prompt existed yet.\"\npr_start:\n  enabled: true\n  slug: \"{slug}\"\n---\n\n# {title}\n\n## Summary\n\nBootstrap-generated local source prompt for issue #{issue}.\n\n## Goal\n\nTranslate the GitHub issue into the canonical local STP/task-bundle flow and refine this prompt before execution as needed.\n\n## Required Outcome\n\nThis issue currently defaults to a required outcome type of `{outcome_type}`. Refine this if the issue requires a different outcome or a combination.\n\n## Deliverables\n\n- one bounded, reviewable outcome matching the issue scope\n- updated canonical docs/code/tests/demo artifacts as required by the issue\n\n## Acceptance Criteria\n\n- the issue title and labels are reflected in the local source prompt\n- the task can proceed through `pr init`, `pr start`, and card editing without manual bootstrap repair\n\n## Repo Inputs\n\n- {issue_url}\n\n## Dependencies\n\n- none recorded yet\n\n## Demo Expectations\n\n- No demo is required by default. Update this section if the issue requires a proof surface.\n\n## Non-goals\n\n- changing milestone scope without recording it explicitly\n- ad-hoc local workflow drift outside the tracked issue flow\n\n## Issue-Graph Notes\n\n- This prompt was generated automatically because the canonical local issue prompt was missing.\n- Review and refine it before substantive implementation work begins.\n\n## Notes\n\n- GitHub issue: {issue_url}\n\n## Tooling Notes\n\n- Generated by `pr.sh` bootstrap fallback.\n"
    )
}

fn infer_wp_from_title(title: &str) -> String {
    if let Some(start) = title.find("[WP-") {
        if let Some(end_rel) = title[start + 1..].find(']') {
            return title[start + 1..start + 1 + end_rel].to_string();
        }
    }
    "unassigned".to_string()
}

fn infer_required_outcome_type(labels_csv: &str, title: &str) -> &'static str {
    let lowered = format!("{} {}", labels_csv.to_lowercase(), title.to_lowercase());
    if lowered.contains("type:docs")
        || lowered.contains("area:docs")
        || lowered.contains("[docs]")
        || lowered.contains("type:design")
    {
        return "docs";
    }
    if lowered.contains("type:test") || lowered.contains("area:tests") || lowered.contains("[test]")
    {
        return "tests";
    }
    if lowered.contains("area:demo") || lowered.contains("[demo]") {
        return "demo";
    }
    "code"
}

fn version_from_labels_csv(labels_csv: &str) -> Option<String> {
    labels_csv
        .split(',')
        .map(str::trim)
        .find_map(|label| label.strip_prefix("version:").map(str::to_string))
}

fn version_from_title(title: &str) -> Option<String> {
    let start = title.find("[v")?;
    let rest = &title[start + 1..];
    let end = rest.find(']')?;
    Some(rest[..end].to_string())
}

fn validate_issue_prompt_exists(path: &Path) -> Result<()> {
    if !path.is_file() {
        bail!("missing canonical source issue prompt: {}", path.display());
    }
    Ok(())
}

fn load_issue_prompt(path: &Path) -> Result<IssuePromptDoc> {
    let text = fs::read_to_string(path)
        .with_context(|| format!("failed to read issue prompt '{}'", path.display()))?;
    let mut parts = text.splitn(3, "---");
    let _ = parts.next();
    let front_matter = parts
        .next()
        .ok_or_else(|| anyhow!("missing front matter in '{}'", path.display()))?;
    let body = parts
        .next()
        .ok_or_else(|| anyhow!("missing markdown body in '{}'", path.display()))?;
    let front_matter: IssuePromptFrontMatter =
        serde_yaml::from_str(front_matter).with_context(|| {
            format!(
                "failed to parse issue prompt front matter '{}'",
                path.display()
            )
        })?;
    Ok(IssuePromptDoc {
        front_matter,
        body: body.trim_start().to_string(),
    })
}

fn reconcile_issue(issue: u32, repo: &str, doc: &IssuePromptDoc) -> Result<()> {
    let desired_labels = doc.front_matter.labels.clone();
    let current_labels = run_capture_allow_failure(
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
    .unwrap_or_default()
    .lines()
    .map(str::trim)
    .filter(|line| !line.is_empty())
    .map(str::to_string)
    .collect::<Vec<_>>();

    let body_file = std::env::temp_dir().join(format!("adl-pr-create-body-{issue}.md"));
    fs::write(&body_file, &doc.body)?;
    run_status(
        "gh",
        &[
            "issue",
            "edit",
            &issue.to_string(),
            "-R",
            repo,
            "--title",
            &doc.front_matter.title,
            "--body-file",
            body_file
                .to_str()
                .ok_or_else(|| anyhow!("body file path must be utf-8"))?,
        ],
    )?;

    for desired in desired_labels
        .iter()
        .filter(|label| !current_labels.contains(label))
    {
        run_status(
            "gh",
            &[
                "issue",
                "edit",
                &issue.to_string(),
                "-R",
                repo,
                "--add-label",
                desired,
            ],
        )?;
    }

    for current in current_labels
        .iter()
        .filter(|label| !desired_labels.contains(label))
    {
        run_status(
            "gh",
            &[
                "issue",
                "edit",
                &issue.to_string(),
                "-R",
                repo,
                "--remove-label",
                current,
            ],
        )?;
    }
    let _ = fs::remove_file(&body_file);
    Ok(())
}

fn resolve_issue_body(body: Option<String>, body_file: Option<&Path>) -> Result<String> {
    if let Some(path) = body_file {
        if path == Path::new("-") {
            bail!("new: --body-file - is not supported in Rust path yet");
        }
        return fs::read_to_string(path)
            .with_context(|| format!("new: --body-file not found: {}", path.display()));
    }
    Ok(body.unwrap_or_else(|| "## Goal\n-\n\n## Acceptance Criteria\n-".to_string()))
}

fn normalize_labels_csv(labels: &str, version: &str) -> String {
    let mut normalized = labels
        .split(',')
        .map(str::trim)
        .filter(|label| !label.is_empty() && !label.starts_with("version:"))
        .map(str::to_string)
        .collect::<Vec<_>>();
    normalized.push(format!("version:{version}"));
    normalized.join(",")
}

fn gh_issue_create(title: &str, body: &str, labels_csv: &str) -> Result<String> {
    let mut cmd = Command::new("gh");
    cmd.arg("issue")
        .arg("create")
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
        .with_context(|| "failed to run gh issue create")?;
    if !output.status.success() {
        bail!(
            "gh issue create failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn parse_issue_number_from_url(url: &str) -> Result<u32> {
    let issue = url
        .trim()
        .rsplit('/')
        .next()
        .ok_or_else(|| anyhow!("new: failed to parse issue number from URL: {url}"))?;
    issue
        .parse::<u32>()
        .with_context(|| format!("new: failed to parse issue number from URL: {url}"))
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
        assert!(content.contains("Generated by `pr.sh` bootstrap fallback."));
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
        assert!(resolve_issue_body(None, None)
            .expect("default body")
            .contains("## Goal"));

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
    fn parse_init_args_rejects_unknown_arg() {
        let err = parse_init_args(&["1151".to_string(), "--bogus".to_string()]).expect_err("err");
        assert!(err.to_string().contains("init: unknown arg"));
    }

    #[test]
    fn parse_create_args_supports_reconcile_mode() {
        match parse_create_args(&[
            "1151".to_string(),
            "--stp".to_string(),
            ".adl/v0.86/tasks/example/stp.md".to_string(),
        ])
        .expect("parse")
        {
            CreateMode::Reconcile { issue, stp_path } => {
                assert_eq!(issue, 1151);
                assert_eq!(stp_path, PathBuf::from(".adl/v0.86/tasks/example/stp.md"));
            }
            other => panic!("unexpected mode: {other:?}"),
        }
    }

    #[test]
    fn parse_create_args_requires_title_and_rejects_conflicting_body_flags() {
        let err = parse_create_args(&["--no-start".to_string()]).expect_err("missing title");
        assert!(err.to_string().contains("--title is required"));

        let err = parse_create_args(&[
            "--title".to_string(),
            "Example".to_string(),
            "--body".to_string(),
            "inline".to_string(),
            "--body-file".to_string(),
            "body.md".to_string(),
        ])
        .expect_err("conflicting body flags");
        assert!(err
            .to_string()
            .contains("only one of --body or --body-file"));
    }

    #[test]
    fn real_pr_init_seeds_stp_from_generated_source_prompt() {
        let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
        let repo = unique_temp_dir("adl-pr-real-init");
        init_git_repo(&repo);
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
        assert!(stp_path.is_file());
        assert!(source_path.is_file());
        let stp = fs::read_to_string(&stp_path).expect("read stp");
        assert!(stp.contains("issue_number: 1151"));
        assert!(stp.contains("title: \"[v0.86][tools] Init test\""));
    }

    #[test]
    fn real_pr_create_no_start_creates_issue_and_source_prompt() {
        let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
        let repo = unique_temp_dir("adl-pr-real-create");
        init_git_repo(&repo);
        let bin_dir = repo.join("bin");
        fs::create_dir_all(&bin_dir).expect("bin dir");
        let gh_log = repo.join("gh.log");
        let gh_path = bin_dir.join("gh");
        write_executable(
            &gh_path,
            &format!(
                "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\nif [ \"$1 $2\" = 'issue create' ]; then\n  printf 'https://github.com/danielbaustin/agent-design-language/issues/1158\\n'\n  exit 0\nfi\nexit 1\n",
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
            "create".to_string(),
            "--title".to_string(),
            "[v0.86][tools] Create test".to_string(),
            "--slug".to_string(),
            "v0-86-tools-create-test".to_string(),
            "--body".to_string(),
            "## Goal\n- test\n\n## Acceptance Criteria\n- works".to_string(),
            "--labels".to_string(),
            "track:roadmap,type:task,area:tools,version:v0.86".to_string(),
            "--no-start".to_string(),
        ]);

        env::set_current_dir(prev_dir).expect("restore cwd");
        unsafe {
            env::set_var("PATH", old_path);
        }
        result.expect("real_pr create");

        let issue_ref = IssueRef::new(
            1158,
            "v0.86".to_string(),
            "v0-86-tools-create-test".to_string(),
        )
        .expect("issue ref");
        let source_path = issue_ref.issue_prompt_path(&repo);
        assert!(source_path.is_file());
        let source = fs::read_to_string(&source_path).expect("read source");
        assert!(source.contains("issue_number: 1158"));
        assert!(source.contains("title: \"[v0.86][tools] Create test\""));
        let gh_calls = fs::read_to_string(&gh_log).expect("read gh log");
        assert!(gh_calls.contains("issue create"));
    }

    #[test]
    fn real_pr_create_reconcile_updates_issue_via_gh() {
        let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
        let repo = unique_temp_dir("adl-pr-reconcile");
        init_git_repo(&repo);
        let bin_dir = repo.join("bin");
        fs::create_dir_all(&bin_dir).expect("bin dir");
        let gh_log = repo.join("gh.log");
        let gh_path = bin_dir.join("gh");
        write_executable(
            &gh_path,
            &format!(
                "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\nif [ \"$1 $2 $5 $6\" = 'issue view --json labels -q' ]; then\n  printf 'track:roadmap\\nversion:v0.86\\n'\n  exit 0\nfi\nif [ \"$1 $2\" = 'issue edit' ]; then\n  exit 0\nfi\nexit 1\n",
                gh_log.display()
            ),
        );

        let stp_dir = repo.join(".adl/v0.86/tasks/issue-1151__example");
        fs::create_dir_all(&stp_dir).expect("stp dir");
        let stp_path = stp_dir.join("stp.md");
        fs::write(
            &stp_path,
            "---\ntitle: \"[v0.86][tools] Reconcile test\"\nlabels:\n  - \"track:roadmap\"\n  - \"type:task\"\n  - \"area:tools\"\n  - \"version:v0.86\"\nissue_number: 1151\n---\n\n# Body\n\nReconcile me.\n",
        )
        .expect("write stp");

        let old_path = env::var("PATH").unwrap_or_default();
        let prev_dir = env::current_dir().expect("cwd");
        unsafe {
            env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
        }
        env::set_current_dir(&repo).expect("chdir");

        let result = real_pr(&[
            "create".to_string(),
            "1151".to_string(),
            "--stp".to_string(),
            stp_path.display().to_string(),
        ]);

        env::set_current_dir(prev_dir).expect("restore cwd");
        unsafe {
            env::set_var("PATH", old_path);
        }
        result.expect("reconcile");

        let gh_calls = fs::read_to_string(&gh_log).expect("read gh log");
        assert!(gh_calls.contains("issue edit 1151 -R danielbaustin/agent-design-language --title [v0.86][tools] Reconcile test --body-file"));
        assert!(gh_calls.contains(
            "issue edit 1151 -R danielbaustin/agent-design-language --add-label type:task"
        ));
        assert!(gh_calls.contains(
            "issue edit 1151 -R danielbaustin/agent-design-language --add-label area:tools"
        ));
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

        let issue_ref = IssueRef::new(
            1152,
            "v0.86".to_string(),
            "rust-start-ready-test".to_string(),
        )
        .expect("issue ref");
        let worktree = issue_ref.default_worktree_path(&repo, None);
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
    fn parse_create_args_supports_new_issue_mode() {
        match parse_create_args(&[
            "--title".to_string(),
            "Example".to_string(),
            "--no-start".to_string(),
        ])
        .expect("parse")
        {
            CreateMode::Create {
                title, no_start, ..
            } => {
                assert_eq!(title, "Example");
                assert!(no_start);
            }
            other => panic!("unexpected mode: {other:?}"),
        }
    }
}
