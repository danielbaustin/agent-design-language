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
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PreflightArgs {
    pub(crate) issue: u32,
    pub(crate) version: Option<String>,
    pub(crate) slug: Option<String>,
    pub(crate) no_fetch_issue: bool,
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

fn require_value(args: &[String], index: usize, cmd: &str, flag: &str) -> Result<String> {
    args.get(index + 1)
        .cloned()
        .ok_or_else(|| anyhow!("{cmd}: {flag} requires a value"))
}
