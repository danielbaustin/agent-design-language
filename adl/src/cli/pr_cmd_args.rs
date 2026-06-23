use anyhow::{anyhow, bail, Context, Result};
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct InitArgs {
    pub(crate) issue: u32,
    pub(crate) slug: Option<String>,
    pub(crate) title_arg: Option<String>,
    pub(crate) no_fetch_issue: bool,
    pub(crate) version: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RepairIssueBodyArgs {
    pub(crate) issue: u32,
    pub(crate) slug: Option<String>,
    pub(crate) title_arg: Option<String>,
    pub(crate) body: Option<String>,
    pub(crate) body_file: Option<PathBuf>,
    pub(crate) labels: Option<String>,
    pub(crate) version: Option<String>,
    pub(crate) force: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CreateArgs {
    pub(crate) slug: Option<String>,
    pub(crate) title_arg: Option<String>,
    pub(crate) body: Option<String>,
    pub(crate) body_file: Option<PathBuf>,
    pub(crate) labels: Option<String>,
    pub(crate) version: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct StartArgs {
    pub(crate) issue: u32,
    pub(crate) prefix: String,
    pub(crate) slug: Option<String>,
    pub(crate) title_arg: Option<String>,
    pub(crate) no_fetch_issue: bool,
    pub(crate) version: Option<String>,
    pub(crate) allow_open_pr_wave: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ReadyArgs {
    pub(crate) issue: u32,
    pub(crate) slug: Option<String>,
    pub(crate) version: Option<String>,
    pub(crate) no_fetch_issue: bool,
    pub(crate) json: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PreflightArgs {
    pub(crate) issue: u32,
    pub(crate) version: Option<String>,
    pub(crate) slug: Option<String>,
    pub(crate) no_fetch_issue: bool,
    pub(crate) json: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum DoctorMode {
    Full,
    Ready,
    Preflight,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct DoctorArgs {
    pub(crate) issue: u32,
    pub(crate) version: Option<String>,
    pub(crate) slug: Option<String>,
    pub(crate) no_fetch_issue: bool,
    pub(crate) mode: DoctorMode,
    pub(crate) json: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct FinishArgs {
    pub(crate) issue: u32,
    pub(crate) title: String,
    pub(crate) extra_body: Option<String>,
    pub(crate) paths: String,
    pub(crate) no_checks: bool,
    pub(crate) no_close: bool,
    pub(crate) ready: bool,
    pub(crate) allow_gitignore: bool,
    pub(crate) input_path: Option<PathBuf>,
    pub(crate) output_path: Option<PathBuf>,
    pub(crate) no_open: bool,
    pub(crate) merge_mode: bool,
    pub(crate) idempotent: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ValidationArgs {
    pub(crate) pr_ref: String,
    pub(crate) repo: Option<String>,
    pub(crate) watch: bool,
    pub(crate) json: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct WatchArgs {
    pub(crate) issue_ref: String,
    pub(crate) repo: Option<String>,
    pub(crate) slug: Option<String>,
    pub(crate) version: Option<String>,
    pub(crate) json: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ClosingLinkageArgs {
    pub(crate) event_name: Option<String>,
    pub(crate) event_path: Option<PathBuf>,
    pub(crate) head_ref: Option<String>,
    pub(crate) repo: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CloseoutArgs {
    pub(crate) issue: u32,
    pub(crate) slug: Option<String>,
    pub(crate) version: Option<String>,
    pub(crate) no_fetch_issue: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ProjectionMapArgs {
    pub(crate) json: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum IssueStateFilter {
    Open,
    Closed,
    All,
}

impl IssueStateFilter {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Closed => "closed",
            Self::All => "all",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct IssueListArgs {
    pub(crate) repo: Option<String>,
    pub(crate) state: IssueStateFilter,
    pub(crate) json: bool,
    pub(crate) limit: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct IssueSearchArgs {
    pub(crate) query: String,
    pub(crate) repo: Option<String>,
    pub(crate) state: IssueStateFilter,
    pub(crate) json: bool,
    pub(crate) limit: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct IssueViewArgs {
    pub(crate) issue_ref: String,
    pub(crate) repo: Option<String>,
    pub(crate) json: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct IssueCreateArgs {
    pub(crate) title: String,
    pub(crate) body: Option<String>,
    pub(crate) body_file: Option<PathBuf>,
    pub(crate) labels: Vec<String>,
    pub(crate) repo: Option<String>,
    pub(crate) json: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct IssueCommentArgs {
    pub(crate) issue_ref: String,
    pub(crate) body: Option<String>,
    pub(crate) body_file: Option<PathBuf>,
    pub(crate) repo: Option<String>,
    pub(crate) json: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct IssueEditArgs {
    pub(crate) issue_ref: String,
    pub(crate) title: Option<String>,
    pub(crate) body: Option<String>,
    pub(crate) body_file: Option<PathBuf>,
    pub(crate) labels: Vec<String>,
    pub(crate) repo: Option<String>,
    pub(crate) json: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum IssueCloseReason {
    Completed,
    NotPlanned,
}

impl IssueCloseReason {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            Self::Completed => "completed",
            Self::NotPlanned => "not_planned",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct IssueCloseArgs {
    pub(crate) issue_ref: String,
    pub(crate) reason: IssueCloseReason,
    pub(crate) repo: Option<String>,
    pub(crate) json: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum IssueArgs {
    List(IssueListArgs),
    Search(IssueSearchArgs),
    View(IssueViewArgs),
    Create(IssueCreateArgs),
    Comment(IssueCommentArgs),
    Edit(IssueEditArgs),
    Close(IssueCloseArgs),
}

pub(crate) fn parse_init_args(args: &[String]) -> Result<InitArgs> {
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

pub(crate) fn parse_create_args(args: &[String]) -> Result<CreateArgs> {
    let mut parsed = CreateArgs {
        slug: None,
        title_arg: None,
        body: None,
        body_file: None,
        labels: None,
        version: None,
    };

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--slug" => {
                parsed.slug = Some(require_value(args, i, "create", "--slug")?);
                i += 1;
            }
            "--title" => {
                parsed.title_arg = Some(require_value(args, i, "create", "--title")?);
                i += 1;
            }
            "--body" => {
                parsed.body = Some(require_value(args, i, "create", "--body")?);
                i += 1;
            }
            "--body-file" => {
                parsed.body_file = Some(PathBuf::from(require_value(
                    args,
                    i,
                    "create",
                    "--body-file",
                )?));
                i += 1;
            }
            "--labels" => {
                parsed.labels = Some(require_value(args, i, "create", "--labels")?);
                i += 1;
            }
            "--version" => {
                parsed.version = Some(require_value(args, i, "create", "--version")?);
                i += 1;
            }
            other => bail!("create: unknown arg: {other}"),
        }
        i += 1;
    }

    if parsed.title_arg.as_deref().unwrap_or("").trim().is_empty() {
        bail!("create: --title is required");
    }
    if parsed.body.is_some() && parsed.body_file.is_some() {
        bail!("create: pass only one of --body or --body-file");
    }

    Ok(parsed)
}

pub(crate) fn parse_repair_issue_body_args(args: &[String]) -> Result<RepairIssueBodyArgs> {
    let issue_raw = args
        .first()
        .ok_or_else(|| anyhow!("repair-issue-body: missing <issue> number"))?;
    let issue = issue_raw
        .parse::<u32>()
        .with_context(|| format!("invalid issue number: {issue_raw}"))?;
    let mut parsed = RepairIssueBodyArgs {
        issue,
        slug: None,
        title_arg: None,
        body: None,
        body_file: None,
        labels: None,
        version: None,
        force: false,
    };
    let mut i = 1;

    while i < args.len() {
        match args[i].as_str() {
            "--slug" => {
                parsed.slug = Some(require_value(args, i, "repair-issue-body", "--slug")?);
                i += 1;
            }
            "--title" => {
                parsed.title_arg = Some(require_value(args, i, "repair-issue-body", "--title")?);
                i += 1;
            }
            "--body" => {
                parsed.body = Some(require_value(args, i, "repair-issue-body", "--body")?);
                i += 1;
            }
            "--body-file" => {
                parsed.body_file = Some(PathBuf::from(require_value(
                    args,
                    i,
                    "repair-issue-body",
                    "--body-file",
                )?));
                i += 1;
            }
            "--labels" => {
                parsed.labels = Some(require_value(args, i, "repair-issue-body", "--labels")?);
                i += 1;
            }
            "--version" => {
                parsed.version = Some(require_value(args, i, "repair-issue-body", "--version")?);
                i += 1;
            }
            "--force" => parsed.force = true,
            other => bail!("repair-issue-body: unknown arg: {other}"),
        }
        i += 1;
    }

    if parsed.body.is_some() && parsed.body_file.is_some() {
        bail!("repair-issue-body: pass only one of --body or --body-file");
    }
    if parsed.body.is_none()
        && parsed.body_file.is_none()
        && parsed.title_arg.is_none()
        && parsed.labels.is_none()
        && parsed.slug.is_none()
        && parsed.version.is_none()
    {
        bail!(
            "repair-issue-body: pass at least one of --body, --body-file, --title, --labels, --slug, or --version"
        );
    }

    Ok(parsed)
}

pub(crate) fn parse_start_args(args: &[String]) -> Result<StartArgs> {
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
        allow_open_pr_wave: false,
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
            "--allow-open-pr-wave" => parsed.allow_open_pr_wave = true,
            other => bail!("start: unknown arg: {other}"),
        }
        i += 1;
    }
    Ok(parsed)
}

pub(crate) fn parse_preflight_args(args: &[String]) -> Result<PreflightArgs> {
    let issue_raw = args
        .first()
        .ok_or_else(|| anyhow!("preflight: missing <issue> number"))?;
    let issue = issue_raw
        .parse::<u32>()
        .with_context(|| format!("invalid issue number: {issue_raw}"))?;
    let mut parsed = PreflightArgs {
        issue,
        version: None,
        slug: None,
        no_fetch_issue: false,
        json: false,
    };
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--slug" => {
                parsed.slug = Some(require_value(args, i, "preflight", "--slug")?);
                i += 1;
            }
            "--version" => {
                parsed.version = Some(require_value(args, i, "preflight", "--version")?);
                i += 1;
            }
            "--no-fetch-issue" => parsed.no_fetch_issue = true,
            "--json" => parsed.json = true,
            other => bail!("preflight: unknown arg: {other}"),
        }
        i += 1;
    }
    Ok(parsed)
}

pub(crate) fn parse_doctor_args(args: &[String]) -> Result<DoctorArgs> {
    let issue_raw = args
        .first()
        .ok_or_else(|| anyhow!("doctor: missing <issue> number"))?;
    let issue = issue_raw
        .parse::<u32>()
        .with_context(|| format!("invalid issue number: {issue_raw}"))?;
    let mut parsed = DoctorArgs {
        issue,
        version: None,
        slug: None,
        no_fetch_issue: false,
        mode: DoctorMode::Full,
        json: false,
    };
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--slug" => {
                parsed.slug = Some(require_value(args, i, "doctor", "--slug")?);
                i += 1;
            }
            "--version" => {
                parsed.version = Some(require_value(args, i, "doctor", "--version")?);
                i += 1;
            }
            "--mode" => {
                let mode = require_value(args, i, "doctor", "--mode")?;
                parsed.mode = match mode.as_str() {
                    "full" => DoctorMode::Full,
                    "ready" => DoctorMode::Ready,
                    "preflight" => DoctorMode::Preflight,
                    other => bail!("doctor: unsupported mode: {other}"),
                };
                i += 1;
            }
            "--no-fetch-issue" => parsed.no_fetch_issue = true,
            "--json" => parsed.json = true,
            other => bail!("doctor: unknown arg: {other}"),
        }
        i += 1;
    }
    Ok(parsed)
}

pub(crate) fn parse_ready_args(args: &[String]) -> Result<ReadyArgs> {
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
        json: false,
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
            "--json" => parsed.json = true,
            other => bail!("ready: unknown arg: {other}"),
        }
        i += 1;
    }
    Ok(parsed)
}

pub(crate) fn parse_finish_args(args: &[String]) -> Result<FinishArgs> {
    let issue = args
        .first()
        .ok_or_else(|| anyhow!("finish: missing <issue> number"))?;
    let issue = issue
        .parse::<u32>()
        .with_context(|| format!("finish: invalid issue number: {issue}"))?;

    let mut parsed = FinishArgs {
        issue,
        title: String::new(),
        extra_body: None,
        paths: ".".to_string(),
        no_checks: false,
        no_close: false,
        ready: false,
        allow_gitignore: false,
        input_path: None,
        output_path: None,
        no_open: false,
        merge_mode: false,
        idempotent: false,
    };

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--title" => {
                parsed.title = require_value(args, i, "finish", "--title")?;
                i += 2;
            }
            "--body" => {
                parsed.extra_body = Some(require_value(args, i, "finish", "--body")?);
                i += 2;
            }
            "--paths" => {
                parsed.paths = require_value(args, i, "finish", "--paths")?;
                i += 2;
            }
            "--no-checks" => {
                parsed.no_checks = true;
                i += 1;
            }
            "--no-close" => {
                parsed.no_close = true;
                i += 1;
            }
            "--ready" => {
                parsed.ready = true;
                i += 1;
            }
            "--allow-gitignore" => {
                parsed.allow_gitignore = true;
                i += 1;
            }
            "-f" | "--file" | "--input" => {
                parsed.input_path = Some(PathBuf::from(require_value(
                    args,
                    i,
                    "finish",
                    args[i].as_str(),
                )?));
                i += 2;
            }
            "--output" | "--output-card" | "--output-card-file" => {
                parsed.output_path = Some(PathBuf::from(require_value(
                    args,
                    i,
                    "finish",
                    args[i].as_str(),
                )?));
                i += 2;
            }
            "--no-open" => {
                parsed.no_open = true;
                i += 1;
            }
            "--open" => {
                parsed.no_open = false;
                i += 1;
            }
            "--merge" | "--auto-merge" => {
                parsed.merge_mode = true;
                parsed.ready = true;
                i += 1;
            }
            "--idempotent" => {
                parsed.idempotent = true;
                i += 1;
            }
            other => bail!("finish: unknown arg: {other}"),
        }
    }

    if parsed.title.trim().is_empty() {
        bail!("finish: --title is required");
    }
    if parsed.merge_mode && parsed.no_checks {
        bail!("finish: --merge requires checks; remove --no-checks");
    }

    Ok(parsed)
}

pub(crate) fn parse_validation_args(args: &[String]) -> Result<ValidationArgs> {
    let pr_ref = args
        .first()
        .ok_or_else(|| anyhow!("validation: missing <pr> number or URL"))?
        .clone();
    let mut parsed = ValidationArgs {
        pr_ref,
        repo: None,
        watch: false,
        json: false,
    };
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-R" | "--repo" => {
                parsed.repo = Some(require_value(args, i, "validation", args[i].as_str())?);
                i += 1;
            }
            "--watch" | "--wait" => parsed.watch = true,
            "--json" => parsed.json = true,
            other => bail!("validation: unknown arg: {other}"),
        }
        i += 1;
    }
    Ok(parsed)
}

pub(crate) fn parse_watch_args(args: &[String]) -> Result<WatchArgs> {
    let issue_ref = args
        .first()
        .ok_or_else(|| anyhow!("watch: missing <issue-number-or-url>"))?
        .to_string();
    let mut parsed = WatchArgs {
        issue_ref,
        repo: None,
        slug: None,
        version: None,
        json: false,
    };
    let mut i = 1usize;
    while i < args.len() {
        match args[i].as_str() {
            "-R" | "--repo" => {
                parsed.repo = Some(require_value(args, i, "watch", args[i].as_str())?);
                i += 2;
            }
            "--slug" => {
                parsed.slug = Some(require_value(args, i, "watch", "--slug")?);
                i += 2;
            }
            "--version" => {
                parsed.version = Some(require_value(args, i, "watch", "--version")?);
                i += 2;
            }
            "--json" => {
                parsed.json = true;
                i += 1;
            }
            other => bail!("watch: unknown arg: {other}"),
        }
    }
    Ok(parsed)
}

pub(crate) fn parse_closing_linkage_args(args: &[String]) -> Result<ClosingLinkageArgs> {
    let mut parsed = ClosingLinkageArgs {
        event_name: None,
        event_path: None,
        head_ref: None,
        repo: None,
    };
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--event-name" => {
                parsed.event_name =
                    Some(require_value(args, i, "closing-linkage", "--event-name")?);
                i += 1;
            }
            "--event-path" => {
                parsed.event_path = Some(PathBuf::from(require_value(
                    args,
                    i,
                    "closing-linkage",
                    "--event-path",
                )?));
                i += 1;
            }
            "--head-ref" => {
                parsed.head_ref = Some(require_value(args, i, "closing-linkage", "--head-ref")?);
                i += 1;
            }
            "-R" | "--repo" => {
                parsed.repo = Some(require_value(args, i, "closing-linkage", args[i].as_str())?);
                i += 1;
            }
            other => bail!("closing-linkage: unknown arg: {other}"),
        }
        i += 1;
    }
    Ok(parsed)
}

pub(crate) fn parse_closeout_args(args: &[String]) -> Result<CloseoutArgs> {
    let issue_raw = args
        .first()
        .ok_or_else(|| anyhow!("closeout: missing <issue> number"))?;
    let issue = issue_raw
        .parse::<u32>()
        .with_context(|| format!("invalid issue number: {issue_raw}"))?;
    let mut parsed = CloseoutArgs {
        issue,
        slug: None,
        version: None,
        no_fetch_issue: false,
    };
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--slug" => {
                parsed.slug = Some(require_value(args, i, "closeout", "--slug")?);
                i += 1;
            }
            "--version" => {
                parsed.version = Some(require_value(args, i, "closeout", "--version")?);
                i += 1;
            }
            "--no-fetch-issue" => parsed.no_fetch_issue = true,
            other => bail!("closeout: unknown arg: {other}"),
        }
        i += 1;
    }
    Ok(parsed)
}

pub(crate) fn parse_issue_args(args: &[String]) -> Result<IssueArgs> {
    let Some(subcommand) = args.first().map(|value| value.as_str()) else {
        bail!("issue: missing subcommand (list | search | view | create | comment | edit | close)");
    };

    match subcommand {
        "list" => parse_issue_list_args(&args[1..]).map(IssueArgs::List),
        "search" => parse_issue_search_args(&args[1..]).map(IssueArgs::Search),
        "view" => parse_issue_view_args(&args[1..]).map(IssueArgs::View),
        "create" => parse_issue_create_args(&args[1..]).map(IssueArgs::Create),
        "comment" => parse_issue_comment_args(&args[1..]).map(IssueArgs::Comment),
        "edit" => parse_issue_edit_args(&args[1..]).map(IssueArgs::Edit),
        "close" => parse_issue_close_args(&args[1..]).map(IssueArgs::Close),
        other => bail!("issue: unknown subcommand: {other}"),
    }
}

pub(crate) fn parse_projection_map_args(args: &[String]) -> Result<ProjectionMapArgs> {
    let mut parsed = ProjectionMapArgs { json: false };
    for arg in args {
        match arg.as_str() {
            "--json" => parsed.json = true,
            other => bail!("projection-map: unknown arg: {other}"),
        }
    }
    Ok(parsed)
}

fn parse_issue_list_args(args: &[String]) -> Result<IssueListArgs> {
    let mut parsed = IssueListArgs {
        repo: None,
        state: IssueStateFilter::Open,
        json: false,
        limit: 100,
    };
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "-R" | "--repo" => {
                parsed.repo = Some(require_value(args, i, "issue list", args[i].as_str())?);
                i += 1;
            }
            "--state" => {
                let value = require_value(args, i, "issue list", "--state")?;
                parsed.state = parse_issue_state_filter("issue list", &value)?;
                i += 1;
            }
            "--limit" => {
                parsed.limit = parse_positive_usize(
                    &require_value(args, i, "issue list", "--limit")?,
                    "issue list",
                    "--limit",
                )?;
                i += 1;
            }
            "--json" => parsed.json = true,
            other => bail!("issue list: unknown arg: {other}"),
        }
        i += 1;
    }
    Ok(parsed)
}

fn parse_issue_search_args(args: &[String]) -> Result<IssueSearchArgs> {
    let mut parsed = IssueSearchArgs {
        query: String::new(),
        repo: None,
        state: IssueStateFilter::Open,
        json: false,
        limit: 100,
    };
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--query" => {
                parsed.query = require_value(args, i, "issue search", "--query")?;
                i += 1;
            }
            "-R" | "--repo" => {
                parsed.repo = Some(require_value(args, i, "issue search", args[i].as_str())?);
                i += 1;
            }
            "--state" => {
                let value = require_value(args, i, "issue search", "--state")?;
                parsed.state = parse_issue_state_filter("issue search", &value)?;
                i += 1;
            }
            "--limit" => {
                parsed.limit = parse_positive_usize(
                    &require_value(args, i, "issue search", "--limit")?,
                    "issue search",
                    "--limit",
                )?;
                i += 1;
            }
            "--json" => parsed.json = true,
            other => bail!("issue search: unknown arg: {other}"),
        }
        i += 1;
    }
    if parsed.query.trim().is_empty() {
        bail!("issue search: --query is required");
    }
    if parsed.limit > 1000 {
        bail!("issue search: --limit must be 1000 or less");
    }
    Ok(parsed)
}

fn parse_issue_view_args(args: &[String]) -> Result<IssueViewArgs> {
    let issue_ref = args
        .first()
        .ok_or_else(|| anyhow!("issue view: missing <issue-number-or-url>"))?
        .clone();
    let mut parsed = IssueViewArgs {
        issue_ref,
        repo: None,
        json: false,
    };
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-R" | "--repo" => {
                parsed.repo = Some(require_value(args, i, "issue view", args[i].as_str())?);
                i += 1;
            }
            "--json" => parsed.json = true,
            other => bail!("issue view: unknown arg: {other}"),
        }
        i += 1;
    }
    Ok(parsed)
}

fn parse_issue_create_args(args: &[String]) -> Result<IssueCreateArgs> {
    let mut parsed = IssueCreateArgs {
        title: String::new(),
        body: None,
        body_file: None,
        labels: Vec::new(),
        repo: None,
        json: false,
    };
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--title" => {
                parsed.title = require_value(args, i, "issue create", "--title")?;
                i += 1;
            }
            "--body" => {
                parsed.body = Some(require_value(args, i, "issue create", "--body")?);
                i += 1;
            }
            "--body-file" => {
                parsed.body_file = Some(PathBuf::from(require_value(
                    args,
                    i,
                    "issue create",
                    "--body-file",
                )?));
                i += 1;
            }
            "--label" => {
                parsed
                    .labels
                    .push(require_value(args, i, "issue create", "--label")?);
                i += 1;
            }
            "--labels" => {
                parsed.labels.extend(split_labels(&require_value(
                    args,
                    i,
                    "issue create",
                    "--labels",
                )?));
                i += 1;
            }
            "-R" | "--repo" => {
                parsed.repo = Some(require_value(args, i, "issue create", args[i].as_str())?);
                i += 1;
            }
            "--json" => parsed.json = true,
            other => bail!("issue create: unknown arg: {other}"),
        }
        i += 1;
    }
    if parsed.title.trim().is_empty() {
        bail!("issue create: --title is required");
    }
    if parsed.body.is_some() && parsed.body_file.is_some() {
        bail!("issue create: pass only one of --body or --body-file");
    }
    Ok(parsed)
}

fn parse_issue_comment_args(args: &[String]) -> Result<IssueCommentArgs> {
    let issue_ref = args
        .first()
        .ok_or_else(|| anyhow!("issue comment: missing <issue-number-or-url>"))?
        .clone();
    let mut parsed = IssueCommentArgs {
        issue_ref,
        body: None,
        body_file: None,
        repo: None,
        json: false,
    };
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--body" => {
                parsed.body = Some(require_value(args, i, "issue comment", "--body")?);
                i += 1;
            }
            "--body-file" => {
                parsed.body_file = Some(PathBuf::from(require_value(
                    args,
                    i,
                    "issue comment",
                    "--body-file",
                )?));
                i += 1;
            }
            "-R" | "--repo" => {
                parsed.repo = Some(require_value(args, i, "issue comment", args[i].as_str())?);
                i += 1;
            }
            "--json" => parsed.json = true,
            other => bail!("issue comment: unknown arg: {other}"),
        }
        i += 1;
    }
    if parsed.body.is_some() && parsed.body_file.is_some() {
        bail!("issue comment: pass only one of --body or --body-file");
    }
    if parsed.body.is_none() && parsed.body_file.is_none() {
        bail!("issue comment: --body or --body-file is required");
    }
    Ok(parsed)
}

fn parse_issue_edit_args(args: &[String]) -> Result<IssueEditArgs> {
    let issue_ref = args
        .first()
        .ok_or_else(|| anyhow!("issue edit: missing <issue-number-or-url>"))?
        .clone();
    let mut parsed = IssueEditArgs {
        issue_ref,
        title: None,
        body: None,
        body_file: None,
        labels: Vec::new(),
        repo: None,
        json: false,
    };
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--title" => {
                parsed.title = Some(require_value(args, i, "issue edit", "--title")?);
                i += 1;
            }
            "--body" => {
                parsed.body = Some(require_value(args, i, "issue edit", "--body")?);
                i += 1;
            }
            "--body-file" => {
                parsed.body_file = Some(PathBuf::from(require_value(
                    args,
                    i,
                    "issue edit",
                    "--body-file",
                )?));
                i += 1;
            }
            "--label" => {
                parsed
                    .labels
                    .push(require_value(args, i, "issue edit", "--label")?);
                i += 1;
            }
            "--labels" => {
                parsed.labels.extend(split_labels(&require_value(
                    args,
                    i,
                    "issue edit",
                    "--labels",
                )?));
                i += 1;
            }
            "-R" | "--repo" => {
                parsed.repo = Some(require_value(args, i, "issue edit", args[i].as_str())?);
                i += 1;
            }
            "--json" => parsed.json = true,
            other => bail!("issue edit: unknown arg: {other}"),
        }
        i += 1;
    }
    if parsed.body.is_some() && parsed.body_file.is_some() {
        bail!("issue edit: pass only one of --body or --body-file");
    }
    if parsed.title.is_none()
        && parsed.body.is_none()
        && parsed.body_file.is_none()
        && parsed.labels.is_empty()
    {
        bail!(
            "issue edit: pass at least one of --title, --body, --body-file, --label, or --labels"
        );
    }
    Ok(parsed)
}

fn parse_issue_close_args(args: &[String]) -> Result<IssueCloseArgs> {
    let issue_ref = args
        .first()
        .ok_or_else(|| anyhow!("issue close: missing <issue-number-or-url>"))?
        .clone();
    let mut parsed = IssueCloseArgs {
        issue_ref,
        reason: IssueCloseReason::Completed,
        repo: None,
        json: false,
    };
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--reason" | "--state-reason" => {
                parsed.reason = parse_issue_close_reason(&require_value(
                    args,
                    i,
                    "issue close",
                    args[i].as_str(),
                )?)?;
                i += 1;
            }
            "-R" | "--repo" => {
                parsed.repo = Some(require_value(args, i, "issue close", args[i].as_str())?);
                i += 1;
            }
            "--json" => parsed.json = true,
            other => bail!("issue close: unknown arg: {other}"),
        }
        i += 1;
    }
    Ok(parsed)
}

fn split_labels(raw: &str) -> Vec<String> {
    raw.split(',')
        .map(str::trim)
        .filter(|label| !label.is_empty())
        .map(ToString::to_string)
        .collect()
}

fn parse_issue_state_filter(cmd: &str, value: &str) -> Result<IssueStateFilter> {
    match value {
        "open" => Ok(IssueStateFilter::Open),
        "closed" => Ok(IssueStateFilter::Closed),
        "all" => Ok(IssueStateFilter::All),
        other => bail!("{cmd}: unsupported --state value: {other}"),
    }
}

fn parse_issue_close_reason(value: &str) -> Result<IssueCloseReason> {
    match value {
        "completed" => Ok(IssueCloseReason::Completed),
        "not_planned" | "not-planned" => Ok(IssueCloseReason::NotPlanned),
        other => {
            bail!("issue close: unsupported --reason '{other}'; expected completed or not_planned")
        }
    }
}

fn parse_positive_usize(value: &str, cmd: &str, flag: &str) -> Result<usize> {
    let parsed = value
        .parse::<usize>()
        .with_context(|| format!("{cmd}: invalid value for {flag}: {value}"))?;
    if parsed == 0 {
        bail!("{cmd}: {flag} must be greater than zero");
    }
    Ok(parsed)
}

fn require_value(args: &[String], index: usize, cmd: &str, flag: &str) -> Result<String> {
    args.get(index + 1)
        .cloned()
        .ok_or_else(|| anyhow!("{cmd}: {flag} requires a value"))
}
