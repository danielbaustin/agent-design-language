use anyhow::{anyhow, bail, Context, Result};
use chrono::Utc;
use serde::Serialize;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::{Duration, SystemTime};

const DEFAULT_INTERVAL_SECONDS: f64 = 60.0;
const DEFAULT_ITERATIONS: u32 = 1;
const DEFAULT_HISTORY_ROOT: &str = ".adl/runs/codex_usage_watch";
const HISTORY_FILE: &str = "history.jsonl";
const DEFAULT_GUARD_CONTEXT_CONSERVE_PERCENT: f64 = 20.0;
const DEFAULT_GUARD_LIMIT_CONSERVE_PERCENT: f64 = 15.0;
const DEFAULT_GUARD_LIMIT_PAUSE_PERCENT: f64 = 5.0;
const DEFAULT_GUARD_LIMIT_RESET_READY_PERCENT: f64 = 1.0;

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub(crate) enum CodexUsageMode {
    Normal,
    Conserve,
    Emergency,
    ResetReady,
    InvokeReset,
    UsageUnknown,
}

impl CodexUsageMode {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            Self::Normal => "normal",
            Self::Conserve => "conserve",
            Self::Emergency => "emergency",
            Self::ResetReady => "reset_ready",
            Self::InvokeReset => "invoke_reset",
            Self::UsageUnknown => "usage_unknown",
        }
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub(crate) struct UsageDimension {
    pub percent_left: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used_tokens: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_tokens: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resets_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub(crate) struct CodexUsageReport {
    pub schema_version: String,
    pub mode: CodexUsageMode,
    pub parse_ok: bool,
    pub sampled_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<UsageDimension>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_5h: Option<UsageDimension>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_7d: Option<UsageDimension>,
    pub warnings: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub history_ref: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
struct ParsedStatus {
    context: UsageDimension,
    limit_5h: UsageDimension,
    limit_7d: UsageDimension,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub(crate) enum CodexUsageGuardAction {
    Continue,
    Conserve,
    Pause,
    ResetReady,
    Unknown,
}

impl CodexUsageGuardAction {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Continue => "continue",
            Self::Conserve => "conserve",
            Self::Pause => "pause",
            Self::ResetReady => "reset_ready",
            Self::Unknown => "unknown",
        }
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub(crate) struct CodexUsageGuardPolicy {
    pub conserve_context_percent: f64,
    pub conserve_limit_percent: f64,
    pub pause_limit_percent: f64,
    pub reset_ready_limit_percent: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_input_age_seconds: Option<u64>,
}

impl Default for CodexUsageGuardPolicy {
    fn default() -> Self {
        Self {
            conserve_context_percent: DEFAULT_GUARD_CONTEXT_CONSERVE_PERCENT,
            conserve_limit_percent: DEFAULT_GUARD_LIMIT_CONSERVE_PERCENT,
            pause_limit_percent: DEFAULT_GUARD_LIMIT_PAUSE_PERCENT,
            reset_ready_limit_percent: DEFAULT_GUARD_LIMIT_RESET_READY_PERCENT,
            max_input_age_seconds: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub(crate) struct CodexUsageGuardDecision {
    pub schema_version: String,
    pub action: CodexUsageGuardAction,
    pub mode: CodexUsageMode,
    pub sampled_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_ref: Option<String>,
    pub policy: CodexUsageGuardPolicy,
    pub usage: CodexUsageReport,
    pub warnings: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

enum CommandMode {
    Parse,
    Collect,
    Guard,
    Watch,
}

struct CommandArgs {
    mode: CommandMode,
    input: Option<PathBuf>,
    text: Option<String>,
    json_output: bool,
    interval_seconds: f64,
    iterations: u32,
    history_root: PathBuf,
    guard_policy: CodexUsageGuardPolicy,
}

pub(crate) fn real_codex_usage_watch(args: &[String]) -> Result<()> {
    if matches!(
        args.first().map(|arg| arg.as_str()),
        Some("--help" | "-h" | "help")
    ) {
        println!("{}", usage());
        return Ok(());
    }
    let args = parse_args(args)?;
    match args.mode {
        CommandMode::Parse => {
            let source = load_status_text(&args)?;
            let report = report_from_source(source.as_deref(), args.input.as_deref());
            print_report(&report, args.json_output)?;
            ensure_report_ready(&report)?;
        }
        CommandMode::Collect => {
            let source = load_status_text(&args)?;
            let report = report_from_collected_source(source.as_deref(), args.input.as_deref());
            print_report(&report, args.json_output)?;
            ensure_report_ready(&report)?;
        }
        CommandMode::Guard => {
            let decision = guard_decision(&args)?;
            print_guard_decision(&decision, args.json_output)?;
            ensure_guard_decision_ready(&decision)?;
        }
        CommandMode::Watch => {
            run_watch(&args)?;
        }
    }
    Ok(())
}

#[cfg(test)]
pub(crate) fn run_parse_status_text(text: &str) -> Result<CodexUsageReport> {
    Ok(report_from_source(Some(text), None))
}

#[cfg(test)]
pub(crate) fn run_collect_status_text(text: &str) -> CodexUsageReport {
    report_from_collected_source(Some(text), None)
}

#[cfg(test)]
pub(crate) fn run_guard_status_text(text: &str) -> Result<CodexUsageGuardDecision> {
    run_guard_status_text_with_policy(text, CodexUsageGuardPolicy::default())
}

#[cfg(test)]
pub(crate) fn run_guard_status_text_with_policy(
    text: &str,
    guard_policy: CodexUsageGuardPolicy,
) -> Result<CodexUsageGuardDecision> {
    let args = CommandArgs {
        mode: CommandMode::Guard,
        input: None,
        text: Some(text.to_string()),
        json_output: true,
        interval_seconds: DEFAULT_INTERVAL_SECONDS,
        iterations: DEFAULT_ITERATIONS,
        history_root: PathBuf::from(DEFAULT_HISTORY_ROOT),
        guard_policy,
    };
    guard_decision(&args)
}

#[cfg(test)]
pub(crate) fn run_guard_status_input_with_policy(
    input: PathBuf,
    guard_policy: CodexUsageGuardPolicy,
) -> Result<CodexUsageGuardDecision> {
    let args = CommandArgs {
        mode: CommandMode::Guard,
        input: Some(input),
        text: None,
        json_output: true,
        interval_seconds: DEFAULT_INTERVAL_SECONDS,
        iterations: DEFAULT_ITERATIONS,
        history_root: PathBuf::from(DEFAULT_HISTORY_ROOT),
        guard_policy,
    };
    guard_decision(&args)
}

fn parse_args(args: &[String]) -> Result<CommandArgs> {
    let Some(mode) = args.first().map(|arg| arg.as_str()) else {
        bail!("{}", usage());
    };
    let mode = match mode {
        "parse" => CommandMode::Parse,
        "collect" => CommandMode::Collect,
        "guard" => CommandMode::Guard,
        "watch" => CommandMode::Watch,
        "--help" | "-h" | "help" => {
            println!("{}", usage());
            return Err(anyhow!("help requested"));
        }
        other => bail!("unknown codex-usage-watch action '{other}'\n{}", usage()),
    };

    let mut input = None;
    let mut text = None;
    let mut json_output = false;
    let mut interval_seconds = DEFAULT_INTERVAL_SECONDS;
    let mut iterations = DEFAULT_ITERATIONS;
    let mut history_root = PathBuf::from(DEFAULT_HISTORY_ROOT);
    let mut guard_policy = CodexUsageGuardPolicy::default();

    let mut i = 1usize;
    while i < args.len() {
        match args[i].as_str() {
            "--input" => input = Some(PathBuf::from(require_value(args, &mut i, "--input")?)),
            "--text" => text = Some(require_value(args, &mut i, "--text")?),
            "--json" => json_output = true,
            "--interval-seconds" => {
                let value = require_value(args, &mut i, "--interval-seconds")?;
                interval_seconds = value
                    .parse::<f64>()
                    .with_context(|| format!("invalid --interval-seconds '{value}'"))?;
                if interval_seconds < 0.0 {
                    bail!("--interval-seconds must be non-negative");
                }
            }
            "--iterations" => {
                let value = require_value(args, &mut i, "--iterations")?;
                iterations = value
                    .parse::<u32>()
                    .with_context(|| format!("invalid --iterations '{value}'"))?;
                if iterations == 0 {
                    bail!("--iterations must be at least 1");
                }
            }
            "--history-root" => {
                history_root = PathBuf::from(require_value(args, &mut i, "--history-root")?)
            }
            "--conserve-context-percent" => {
                guard_policy.conserve_context_percent =
                    parse_percent_flag(args, &mut i, "--conserve-context-percent")?
            }
            "--conserve-limit-percent" => {
                guard_policy.conserve_limit_percent =
                    parse_percent_flag(args, &mut i, "--conserve-limit-percent")?
            }
            "--pause-limit-percent" => {
                guard_policy.pause_limit_percent =
                    parse_percent_flag(args, &mut i, "--pause-limit-percent")?
            }
            "--reset-ready-limit-percent" => {
                guard_policy.reset_ready_limit_percent =
                    parse_percent_flag(args, &mut i, "--reset-ready-limit-percent")?
            }
            "--max-input-age-seconds" => {
                let value = require_value(args, &mut i, "--max-input-age-seconds")?;
                guard_policy.max_input_age_seconds = Some(
                    value
                        .parse::<u64>()
                        .with_context(|| format!("invalid --max-input-age-seconds '{value}'"))?,
                );
            }
            "--help" | "-h" => {
                println!("{}", usage());
                return Err(anyhow!("help requested"));
            }
            other => bail!("unknown codex-usage-watch argument '{other}'"),
        }
        i += 1;
    }

    validate_guard_policy(&guard_policy)?;

    if input.is_some() && text.is_some() {
        bail!("pass only one of --input or --text");
    }

    match mode {
        CommandMode::Parse => {
            if input.is_none() && text.is_none() {
                bail!("codex-usage-watch parse requires --input <path> or --text <status>");
            }
        }
        CommandMode::Collect => {}
        CommandMode::Guard => {}
        CommandMode::Watch => {
            if input.is_none() {
                bail!("codex-usage-watch watch requires --input <path>");
            }
        }
    }

    Ok(CommandArgs {
        mode,
        input,
        text,
        json_output,
        interval_seconds,
        iterations,
        history_root,
        guard_policy,
    })
}

fn usage() -> &'static str {
    "adl tooling codex-usage-watch parse --input <status.txt> [--json]\n\
adl tooling codex-usage-watch parse --text \"Context: ...\" [--json]\n\
adl tooling codex-usage-watch collect [--input <status-panel.txt> | --text \"Status...\"] [--json]\n\
adl tooling codex-usage-watch guard [--input <status-panel.txt> | --text \"Status...\"] [--max-input-age-seconds <n>] [--conserve-context-percent <n>] [--conserve-limit-percent <n>] [--pause-limit-percent <n>] [--reset-ready-limit-percent <n>] [--json]\n\
adl tooling codex-usage-watch watch --input <status.txt> [--interval-seconds <n>] [--iterations <n>] [--history-root <dir>] [--json]"
}

fn require_value(args: &[String], i: &mut usize, flag: &str) -> Result<String> {
    *i += 1;
    args.get(*i)
        .filter(|value| !value.trim().is_empty())
        .cloned()
        .ok_or_else(|| anyhow!("{flag} requires a non-empty value"))
}

fn parse_percent_flag(args: &[String], i: &mut usize, flag: &str) -> Result<f64> {
    let value = require_value(args, i, flag)?;
    let parsed = value
        .parse::<f64>()
        .with_context(|| format!("invalid {flag} '{value}'"))?;
    if !(0.0..=100.0).contains(&parsed) {
        bail!("{flag} must be between 0 and 100");
    }
    Ok(parsed)
}

fn validate_guard_policy(policy: &CodexUsageGuardPolicy) -> Result<()> {
    if policy.reset_ready_limit_percent > policy.pause_limit_percent {
        bail!("--reset-ready-limit-percent must be <= --pause-limit-percent");
    }
    if policy.pause_limit_percent > policy.conserve_limit_percent {
        bail!("--pause-limit-percent must be <= --conserve-limit-percent");
    }
    Ok(())
}

fn run_watch(args: &CommandArgs) -> Result<()> {
    let history_file = args.history_root.join(HISTORY_FILE);
    if let Some(parent) = history_file.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create history dir '{}'", parent.display()))?;
    }

    for iteration in 0..args.iterations {
        let source = load_status_text(args)?;
        let mut report = report_from_source(source.as_deref(), args.input.as_deref());
        append_history(&history_file, &report)?;
        report.history_ref = Some(repo_relative_or_filename(&history_file));
        print_report(&report, args.json_output)?;
        ensure_report_ready(&report)?;

        if iteration + 1 < args.iterations && args.interval_seconds > 0.0 {
            thread::sleep(Duration::from_secs_f64(args.interval_seconds));
        }
    }
    Ok(())
}

fn guard_decision(args: &CommandArgs) -> Result<CodexUsageGuardDecision> {
    if let Some(stale_error) = stale_input_error(args)? {
        let sampled_at = Utc::now().to_rfc3339();
        let source_ref = args.input.as_deref().map(repo_relative_or_filename);
        let usage = unknown_report(sampled_at, source_ref, stale_error);
        return Ok(decision_from_report(usage, &args.guard_policy));
    }

    let source = load_status_text(args)?;
    let usage = report_from_collected_source(source.as_deref(), args.input.as_deref());
    Ok(decision_from_report(usage, &args.guard_policy))
}

fn stale_input_error(args: &CommandArgs) -> Result<Option<String>> {
    let Some(max_age_seconds) = args.guard_policy.max_input_age_seconds else {
        return Ok(None);
    };
    let Some(input) = args.input.as_ref() else {
        return Ok(None);
    };
    let metadata = match fs::metadata(input) {
        Ok(metadata) => metadata,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(err) => {
            return Err(anyhow!(
                "failed to stat status input '{}': {err}",
                input.display()
            ));
        }
    };
    let modified = metadata
        .modified()
        .with_context(|| format!("failed to read modification time for '{}'", input.display()))?;
    let age = SystemTime::now()
        .duration_since(modified)
        .unwrap_or_else(|_| Duration::from_secs(0));
    if age.as_secs() > max_age_seconds {
        return Ok(Some(format!(
            "status input stale: age={}s exceeds max_input_age_seconds={}s",
            age.as_secs(),
            max_age_seconds
        )));
    }
    Ok(None)
}

fn decision_from_report(
    usage: CodexUsageReport,
    policy: &CodexUsageGuardPolicy,
) -> CodexUsageGuardDecision {
    let action = classify_guard_action(&usage, policy);
    let mut warnings = usage.warnings.clone();
    match action {
        CodexUsageGuardAction::Continue => {}
        CodexUsageGuardAction::Conserve => warnings.push(
            "guard_conserve: avoid broad validation, long reviews, and speculative work"
                .to_string(),
        ),
        CodexUsageGuardAction::Pause => warnings.push(
            "guard_pause: pause expensive work until fresh capacity is available".to_string(),
        ),
        CodexUsageGuardAction::ResetReady => warnings.push(
            "guard_reset_ready: capacity is critically low; prepare manual reset or handoff"
                .to_string(),
        ),
        CodexUsageGuardAction::Unknown => warnings.push(
            "guard_unknown: usage state unavailable; fail closed and ask for /status".to_string(),
        ),
    }

    CodexUsageGuardDecision {
        schema_version: "adl.codex_usage_guard.v1".to_string(),
        action,
        mode: usage.mode.clone(),
        sampled_at: usage.sampled_at.clone(),
        source_ref: usage.source_ref.clone(),
        policy: policy.clone(),
        error: usage.error.clone(),
        usage,
        warnings,
    }
}

fn classify_guard_action(
    usage: &CodexUsageReport,
    policy: &CodexUsageGuardPolicy,
) -> CodexUsageGuardAction {
    if !usage.parse_ok {
        return CodexUsageGuardAction::Unknown;
    }
    let Some(context) = usage.context.as_ref() else {
        return CodexUsageGuardAction::Unknown;
    };
    let Some(limit_5h) = usage.limit_5h.as_ref() else {
        return CodexUsageGuardAction::Unknown;
    };
    let Some(limit_7d) = usage.limit_7d.as_ref() else {
        return CodexUsageGuardAction::Unknown;
    };

    let min_limit = limit_5h.percent_left.min(limit_7d.percent_left);
    if min_limit <= policy.reset_ready_limit_percent {
        CodexUsageGuardAction::ResetReady
    } else if min_limit <= policy.pause_limit_percent {
        CodexUsageGuardAction::Pause
    } else if min_limit <= policy.conserve_limit_percent
        || context.percent_left <= policy.conserve_context_percent
    {
        CodexUsageGuardAction::Conserve
    } else {
        CodexUsageGuardAction::Continue
    }
}

fn print_guard_decision(decision: &CodexUsageGuardDecision, json_output: bool) -> Result<()> {
    if json_output {
        println!("{}", serde_json::to_string_pretty(decision)?);
        return Ok(());
    }
    println!("action: {}", decision.action.as_str());
    println!("mode: {}", decision.mode.as_str());
    for warning in &decision.warnings {
        eprintln!("{warning}");
    }
    if let Some(error) = decision.error.as_ref() {
        println!("error: {error}");
    }
    Ok(())
}

fn ensure_guard_decision_ready(decision: &CodexUsageGuardDecision) -> Result<()> {
    if decision.action != CodexUsageGuardAction::Unknown {
        return Ok(());
    }
    bail!(
        "{}",
        decision
            .error
            .as_deref()
            .unwrap_or("codex usage guard could not determine usage state")
    )
}

fn load_status_text(args: &CommandArgs) -> Result<Option<String>> {
    if let Some(text) = args.text.as_ref() {
        return Ok(Some(text.clone()));
    }
    if let Some(path) = args.input.as_ref() {
        match fs::read_to_string(path) {
            Ok(text) => return Ok(Some(text)),
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(err) => {
                return Err(anyhow!(
                    "failed to read status input '{}': {err}",
                    path.display()
                ));
            }
        }
    }
    Ok(None)
}

fn report_from_source(source: Option<&str>, input_path: Option<&Path>) -> CodexUsageReport {
    let sampled_at = Utc::now().to_rfc3339();
    let source_ref = input_path.map(repo_relative_or_filename);
    match source {
        Some(text) => match parse_status_text(text) {
            Ok(parsed) => {
                let mode = classify_mode(&parsed);
                let warnings = warnings_for_mode(&mode, &parsed);
                for warning in &warnings {
                    eprintln!("{warning}");
                }
                CodexUsageReport {
                    schema_version: "adl.codex_usage_watch.v1".to_string(),
                    mode,
                    parse_ok: true,
                    sampled_at,
                    source_ref,
                    context: Some(parsed.context),
                    limit_5h: Some(parsed.limit_5h),
                    limit_7d: Some(parsed.limit_7d),
                    warnings,
                    error: None,
                    history_ref: None,
                }
            }
            Err(err) => unknown_report(
                sampled_at,
                source_ref,
                format!("failed to parse status text: {err}"),
            ),
        },
        None => unknown_report(
            sampled_at,
            source_ref,
            "status input missing; mode forced to usage_unknown".to_string(),
        ),
    }
}

fn report_from_collected_source(
    source: Option<&str>,
    input_path: Option<&Path>,
) -> CodexUsageReport {
    let sampled_at = Utc::now().to_rfc3339();
    let source_ref = input_path.map(repo_relative_or_filename);
    match source {
        Some(text) => match normalize_collected_status_text(text) {
            Ok(normalized) => match parse_status_text(&normalized) {
                Ok(parsed) => {
                    let mode = classify_mode(&parsed);
                    let warnings = warnings_for_mode(&mode, &parsed);
                    for warning in &warnings {
                        eprintln!("{warning}");
                    }
                    CodexUsageReport {
                        schema_version: "adl.codex_usage_watch.v1".to_string(),
                        mode,
                        parse_ok: true,
                        sampled_at,
                        source_ref,
                        context: Some(parsed.context),
                        limit_5h: Some(parsed.limit_5h),
                        limit_7d: Some(parsed.limit_7d),
                        warnings,
                        error: None,
                        history_ref: None,
                    }
                }
                Err(err) => unknown_report(
                    sampled_at,
                    source_ref,
                    format!("failed to parse collected status text: {err}"),
                ),
            },
            Err(err) => unknown_report(
                sampled_at,
                source_ref,
                format!("failed to collect Codex status text: {err}"),
            ),
        },
        None => unknown_report(
            sampled_at,
            source_ref,
            "Codex status collection input missing; live UI collection is not available in this command, pass --input or --text from the app /status panel".to_string(),
        ),
    }
}

fn unknown_report(
    sampled_at: String,
    source_ref: Option<String>,
    error: String,
) -> CodexUsageReport {
    let warnings = vec![format!("usage_unknown: {error}")];
    for warning in &warnings {
        eprintln!("{warning}");
    }
    CodexUsageReport {
        schema_version: "adl.codex_usage_watch.v1".to_string(),
        mode: CodexUsageMode::UsageUnknown,
        parse_ok: false,
        sampled_at,
        source_ref,
        context: None,
        limit_5h: None,
        limit_7d: None,
        warnings,
        error: Some(error),
        history_ref: None,
    }
}

fn print_report(report: &CodexUsageReport, json_output: bool) -> Result<()> {
    if json_output {
        println!("{}", serde_json::to_string_pretty(report)?);
        return Ok(());
    }

    println!("mode: {}", report.mode.as_str());
    if let Some(context) = report.context.as_ref() {
        println!(
            "context: {:.1}% left ({} / {})",
            context.percent_left,
            context.used_tokens.unwrap_or_default(),
            context.limit_tokens.unwrap_or_default()
        );
    }
    if let Some(limit_5h) = report.limit_5h.as_ref() {
        println!(
            "5h: {:.1}% left{}",
            limit_5h.percent_left,
            limit_5h
                .resets_at
                .as_deref()
                .map(|value| format!(" (resets {value})"))
                .unwrap_or_default()
        );
    }
    if let Some(limit_7d) = report.limit_7d.as_ref() {
        println!(
            "7d: {:.1}% left{}",
            limit_7d.percent_left,
            limit_7d
                .resets_at
                .as_deref()
                .map(|value| format!(" (resets {value})"))
                .unwrap_or_default()
        );
    }
    if let Some(error) = report.error.as_ref() {
        println!("error: {error}");
    }
    if let Some(history_ref) = report.history_ref.as_ref() {
        println!("history: {history_ref}");
    }
    Ok(())
}

fn ensure_report_ready(report: &CodexUsageReport) -> Result<()> {
    if report.parse_ok {
        return Ok(());
    }
    bail!(
        "{}",
        report
            .error
            .as_deref()
            .unwrap_or("codex usage watcher input was not parseable")
    )
}

fn append_history(path: &Path, report: &CodexUsageReport) -> Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .with_context(|| format!("failed to open history file '{}'", path.display()))?;
    writeln!(file, "{}", serde_json::to_string(report)?)
        .with_context(|| format!("failed to append history file '{}'", path.display()))?;
    Ok(())
}

fn parse_status_text(text: &str) -> Result<ParsedStatus> {
    let context_line = find_line(text, "Context:")
        .ok_or_else(|| anyhow!("missing 'Context:' line in status text"))?;
    let five_hour_line = find_line(text, "5h limit:")
        .ok_or_else(|| anyhow!("missing '5h limit:' line in status text"))?;
    let seven_day_line = find_line(text, "7d limit:")
        .ok_or_else(|| anyhow!("missing '7d limit:' line in status text"))?;

    Ok(ParsedStatus {
        context: parse_context_line(context_line)?,
        limit_5h: parse_limit_line(five_hour_line, "5h limit:")?,
        limit_7d: parse_limit_line(seven_day_line, "7d limit:")?,
    })
}

fn normalize_collected_status_text(text: &str) -> Result<String> {
    let context_line = find_line(text, "Context:")
        .ok_or_else(|| anyhow!("missing 'Context:' line in collected status text"))?;
    let five_hour_line = find_line(text, "5h limit:")
        .ok_or_else(|| anyhow!("missing '5h limit:' line in collected status text"))?;
    let seven_day_line = find_line(text, "7d limit:")
        .ok_or_else(|| anyhow!("missing '7d limit:' line in collected status text"))?;

    let normalized = [
        normalize_status_line(context_line),
        normalize_status_line(five_hour_line),
        normalize_status_line(seven_day_line),
    ]
    .join("\n");

    Ok(format!("{normalized}\n"))
}

fn normalize_status_line(line: &str) -> String {
    line.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn find_line<'a>(text: &'a str, prefix: &str) -> Option<&'a str> {
    text.lines()
        .find(|line| line.trim_start().starts_with(prefix))
}

fn parse_context_line(line: &str) -> Result<UsageDimension> {
    let percent = extract_percent(line)?;
    let open = line
        .find('(')
        .ok_or_else(|| anyhow!("context line missing '(' usage details"))?;
    let close = line[open + 1..]
        .find(')')
        .map(|idx| open + 1 + idx)
        .ok_or_else(|| anyhow!("context line missing ')' usage details"))?;
    let details = &line[open + 1..close];
    let (used_part, limit_part) = details
        .split_once(" used / ")
        .ok_or_else(|| anyhow!("context details must match '<used> used / <limit>'"))?;

    Ok(UsageDimension {
        percent_left: percent,
        used_tokens: Some(parse_token_amount(used_part)?),
        limit_tokens: Some(parse_token_amount(limit_part)?),
        resets_at: None,
    })
}

fn parse_limit_line(line: &str, prefix: &str) -> Result<UsageDimension> {
    let body = line
        .trim_start()
        .strip_prefix(prefix)
        .ok_or_else(|| anyhow!("status line must start with '{prefix}'"))?;
    let percent = extract_percent(body)?;
    let resets_at = if let Some(start) = body.find("(resets ") {
        let start_idx = start + "(resets ".len();
        let rest = &body[start_idx..];
        let end = rest
            .find(')')
            .ok_or_else(|| anyhow!("limit line missing closing ')' for reset"))?;
        Some(rest[..end].trim().to_string())
    } else {
        None
    };

    Ok(UsageDimension {
        percent_left: percent,
        used_tokens: None,
        limit_tokens: None,
        resets_at,
    })
}

fn extract_percent(text: &str) -> Result<f64> {
    let percent_idx = text
        .find('%')
        .ok_or_else(|| anyhow!("missing '%' percentage marker"))?;
    let left = &text[..percent_idx];
    let number = left
        .split_whitespace()
        .last()
        .ok_or_else(|| anyhow!("missing percentage number before '%'"))?;
    number
        .parse::<f64>()
        .with_context(|| format!("invalid percentage '{number}'"))
}

fn parse_token_amount(value: &str) -> Result<u64> {
    let trimmed = value.trim().replace(',', "");
    if trimmed.ends_with('K') || trimmed.ends_with('k') {
        let number = &trimmed[..trimmed.len() - 1];
        let parsed = number
            .parse::<f64>()
            .with_context(|| format!("invalid token amount '{value}'"))?;
        return Ok((parsed * 1000.0).round() as u64);
    }
    trimmed
        .parse::<u64>()
        .with_context(|| format!("invalid token amount '{value}'"))
}

fn classify_mode(parsed: &ParsedStatus) -> CodexUsageMode {
    let five_hour = parsed.limit_5h.percent_left;
    let seven_day = parsed.limit_7d.percent_left;
    let context = parsed.context.percent_left;

    if five_hour <= 0.5 || seven_day <= 0.5 {
        CodexUsageMode::InvokeReset
    } else if five_hour <= 1.0 || seven_day <= 1.0 {
        CodexUsageMode::ResetReady
    } else if five_hour <= 5.0 || seven_day <= 5.0 {
        CodexUsageMode::Emergency
    } else if five_hour <= 15.0 || seven_day <= 15.0 || context <= 20.0 {
        CodexUsageMode::Conserve
    } else {
        CodexUsageMode::Normal
    }
}

fn warnings_for_mode(mode: &CodexUsageMode, parsed: &ParsedStatus) -> Vec<String> {
    match mode {
        CodexUsageMode::Normal => Vec::new(),
        CodexUsageMode::Conserve => vec![format!(
            "usage_conserve: context={:.1}% 5h={:.1}% 7d={:.1}%",
            parsed.context.percent_left, parsed.limit_5h.percent_left, parsed.limit_7d.percent_left
        )],
        CodexUsageMode::Emergency => vec![format!(
            "usage_emergency: 5h={:.1}% 7d={:.1}% left",
            parsed.limit_5h.percent_left, parsed.limit_7d.percent_left
        )],
        CodexUsageMode::ResetReady => vec![format!(
            "usage_reset_ready: 5h={:.1}% 7d={:.1}% left; reset may be near",
            parsed.limit_5h.percent_left, parsed.limit_7d.percent_left
        )],
        CodexUsageMode::InvokeReset => vec![format!(
            "usage_invoke_reset: 5h={:.1}% 7d={:.1}% left; manual reset is recommended",
            parsed.limit_5h.percent_left, parsed.limit_7d.percent_left
        )],
        CodexUsageMode::UsageUnknown => Vec::new(),
    }
}

fn repo_relative_or_filename(path: &Path) -> String {
    if !path.is_absolute() {
        return path.to_string_lossy().replace('\\', "/");
    }

    if let Ok(cwd) = std::env::current_dir() {
        if let Ok(rel) = path.strip_prefix(&cwd) {
            return rel.to_string_lossy().replace('\\', "/");
        }
        if let Some(parent) = cwd.parent() {
            if let Ok(rel) = path.strip_prefix(parent) {
                return rel.to_string_lossy().replace('\\', "/");
            }
        }
    }

    path.file_name()
        .map(|name| format!("<external>/{}", name.to_string_lossy()))
        .unwrap_or_else(|| "<external>".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "Context: 37% left (161,634 used / 258K)\n5h limit: 4% left (resets 4:04 PM)\n7d limit: 3% left (resets Jun 24)\n";

    #[test]
    fn parses_sample_status_shape() {
        let report = run_parse_status_text(SAMPLE).expect("sample should parse");
        assert!(report.parse_ok);
        assert_eq!(report.mode, CodexUsageMode::Emergency);
        assert_eq!(report.context.as_ref().unwrap().used_tokens, Some(161_634));
        assert_eq!(report.context.as_ref().unwrap().limit_tokens, Some(258_000));
        assert_eq!(
            report.limit_5h.as_ref().unwrap().resets_at.as_deref(),
            Some("4:04 PM")
        );
        assert_eq!(
            report.limit_7d.as_ref().unwrap().resets_at.as_deref(),
            Some("Jun 24")
        );
    }

    #[test]
    fn mode_policy_thresholds_match_issue_contract() {
        let normal = parse_status_text(
            "Context: 85% left (10,000 used / 258K)\n5h limit: 55% left (resets 4:04 PM)\n7d limit: 42% left (resets Jun 24)\n",
        )
        .expect("normal parse");
        assert_eq!(classify_mode(&normal), CodexUsageMode::Normal);

        let conserve = parse_status_text(
            "Context: 20% left (200,000 used / 250K)\n5h limit: 15% left (resets 4:04 PM)\n7d limit: 40% left (resets Jun 24)\n",
        )
        .expect("conserve parse");
        assert_eq!(classify_mode(&conserve), CodexUsageMode::Conserve);

        let emergency = parse_status_text(
            "Context: 50% left (100,000 used / 250K)\n5h limit: 5% left (resets 4:04 PM)\n7d limit: 20% left (resets Jun 24)\n",
        )
        .expect("emergency parse");
        assert_eq!(classify_mode(&emergency), CodexUsageMode::Emergency);

        let reset_ready = parse_status_text(
            "Context: 50% left (100,000 used / 250K)\n5h limit: 1% left (resets 4:04 PM)\n7d limit: 20% left (resets Jun 24)\n",
        )
        .expect("reset ready parse");
        assert_eq!(classify_mode(&reset_ready), CodexUsageMode::ResetReady);

        let invoke_reset = parse_status_text(
            "Context: 50% left (100,000 used / 250K)\n5h limit: 0.5% left (resets 4:04 PM)\n7d limit: 20% left (resets Jun 24)\n",
        )
        .expect("invoke reset parse");
        assert_eq!(classify_mode(&invoke_reset), CodexUsageMode::InvokeReset);
    }

    #[test]
    fn parser_tolerates_decimal_k_suffixes_and_missing_input_fails_closed() {
        assert_eq!(parse_token_amount("258K").expect("K suffix"), 258_000);
        assert_eq!(parse_token_amount("1.5K").expect("decimal K suffix"), 1_500);
        let report = report_from_source(Some("invalid"), Some(Path::new("status.txt")));
        assert_eq!(report.mode, CodexUsageMode::UsageUnknown);
        assert!(!report.parse_ok);
        assert!(report.error.unwrap().contains("failed to parse"));
    }

    #[test]
    fn source_refs_prefer_repo_relative_or_bounded_external_labels() {
        let cwd = std::env::current_dir().expect("cwd");
        let repo_root = cwd.parent().expect("repo root from adl crate dir");
        let repo_path = repo_root.join(".tmp/tooling_cmd_tests/status.txt");
        let external_path = Path::new("/private/tmp/status.txt");

        assert_eq!(
            repo_relative_or_filename(&repo_path),
            ".tmp/tooling_cmd_tests/status.txt"
        );
        assert_eq!(
            repo_relative_or_filename(external_path),
            "<external>/status.txt"
        );
    }
}
