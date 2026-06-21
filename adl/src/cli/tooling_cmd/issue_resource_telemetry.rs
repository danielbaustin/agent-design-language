use anyhow::{anyhow, bail, Context, Result};
use chrono::{DateTime, SecondsFormat, Utc};
use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use super::common::{contains_absolute_host_path_in_text, repo_relative_display};

const SCHEMA_VERSION: &str = "adl.issue_resource_telemetry.v1";
const ARCHIVE_MANIFEST_SCHEMA_VERSION: &str = "adl.issue_resource_telemetry_archive_manifest.v1";
const DEFAULT_CAPTURE_STAGES: &[&str] = &[
    "issue_start",
    "pre_validation",
    "post_validation",
    "review_handoff",
];
const APPROVED_LOCAL_HOST_LABEL: &str = "wuji";
const APPROVED_CSM_HOST_LABELS: &[&str] = &["opticon", "nessus"];
const REDACTED_HOST_LABEL: &str = "redacted_host";

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(untagged)]
enum MetricField<T> {
    Value(T),
    NotAvailable(String),
}

impl<T> MetricField<T> {
    fn not_available() -> Self {
        Self::NotAvailable("not_available".to_string())
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
struct IssueResourceTelemetryRow {
    schema_version: String,
    issue_number: u32,
    issue_slug: String,
    captured_at: String,
    capture_stage: String,
    host: HostRecord,
    data_source: DataSourceRecord,
    cpu: MetricField<CpuRecord>,
    memory: MetricField<MemoryRecord>,
    disk: MetricField<Vec<DiskRecord>>,
    gpu: MetricField<Vec<GpuRecord>>,
    process_summary: MetricField<ProcessSummaryRecord>,
    archive: ArchiveRecord,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
struct HostRecord {
    label: String,
    classification: String,
    approval_state: String,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
struct DataSourceRecord {
    collector: String,
    sampling_scope: String,
    sampling_mode: String,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
struct CpuRecord {
    logical_cores: u32,
    load_avg_1m: f64,
    utilization_pct: f64,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
struct MemoryRecord {
    total_bytes: u64,
    available_bytes: u64,
    used_bytes: u64,
    pressure_state: String,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
struct DiskRecord {
    mount_label: String,
    filesystem_class: String,
    total_bytes: u64,
    available_bytes: u64,
    used_bytes: u64,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
struct GpuRecord {
    device_class: String,
    vendor: String,
    memory_total_bytes: u64,
    memory_used_bytes: u64,
    utilization_pct: f64,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
struct ProcessSummaryRecord {
    tracked_process_count: u32,
    heavy_processes: Vec<TrackedProcessRecord>,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
struct TrackedProcessRecord {
    role: String,
    executable_basename: String,
    pid: u32,
    cpu_pct: f64,
    rss_bytes: u64,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
struct ArchiveRecord {
    redaction_status: String,
    local_retention: String,
    private_archive_ref: Option<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
struct IssueResourceTelemetryArchiveManifest {
    schema_version: String,
    generated_at: String,
    source: ArchiveSourceRecord,
    host: HostRecord,
    archive: ArchiveManifestArchiveRecord,
    consumption_policy: ArchiveConsumptionPolicy,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
struct ArchiveSourceRecord {
    issue_number: u32,
    issue_slug: String,
    captured_at: String,
    input_ref: String,
    repo: String,
    row_count: usize,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
struct ArchiveManifestArchiveRecord {
    backend: String,
    raw_telemetry_ref: String,
    manifest_ref: Option<String>,
    upload_status: String,
    manifest_upload_status: String,
    redaction_status: String,
    local_retention: String,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
struct ArchiveConsumptionPolicy {
    raw_telemetry_is_private_evidence: bool,
    review_safe_summary_is_separate: bool,
    fail_closed_on_upload_or_redaction_error: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum TelemetryAction {
    Collect,
    Archive,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum TrackedProcessSpec {
    Pid { role: String, pid: u32 },
    PidFile { role: String, path: PathBuf },
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TelemetryArgs {
    action: TelemetryAction,
    issue_number: u32,
    issue_slug: String,
    capture_stage: Option<String>,
    host_label: String,
    repo_root: PathBuf,
    out: Option<PathBuf>,
    input: Option<PathBuf>,
    manifest_out: Option<PathBuf>,
    s3_prefix: Option<String>,
    repo: Option<String>,
    captured_at: Option<String>,
    processes: Vec<TrackedProcessSpec>,
    upload: bool,
    upload_manifest: bool,
    redaction_status: String,
    json_output: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct HostPolicy {
    label: String,
    classification: String,
    approval_state: String,
    archive_host_ref: String,
}

trait CommandRunner {
    fn run(&self, program: &str, args: &[&str]) -> Result<String>;
}

trait ArchiveUploader {
    fn upload(&self, source: &Path, dest: &str) -> Result<()>;
}

struct SystemCommandRunner;
struct AwsCliUploader;

impl CommandRunner for SystemCommandRunner {
    fn run(&self, program: &str, args: &[&str]) -> Result<String> {
        let output = Command::new(program)
            .args(args)
            .output()
            .with_context(|| format!("failed running '{program}'"))?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
            bail!(
                "command '{}' failed with status {}{}",
                program,
                output.status,
                if stderr.is_empty() {
                    String::new()
                } else {
                    format!(": {stderr}")
                }
            );
        }
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }
}

impl ArchiveUploader for AwsCliUploader {
    fn upload(&self, source: &Path, dest: &str) -> Result<()> {
        let status = Command::new("aws")
            .args(["s3", "cp"])
            .arg(source)
            .arg(dest)
            .status()
            .context("failed to invoke aws s3 cp for issue-resource-telemetry upload")?;
        if !status.success() {
            bail!("aws s3 cp failed for issue-resource-telemetry upload");
        }
        Ok(())
    }
}

pub(super) fn real_issue_resource_telemetry(args: &[String]) -> Result<()> {
    if matches!(
        args.first().map(|arg| arg.as_str()),
        Some("--help" | "-h" | "help")
    ) {
        println!("{}", usage());
        return Ok(());
    }
    let args = parse_args(args)?;
    let runner = SystemCommandRunner;
    match args.action {
        TelemetryAction::Collect => {
            let row = build_row(&args, &runner)?;
            let output_path = args
                .out
                .clone()
                .unwrap_or_else(|| default_output_path(&args.repo_root, args.issue_number));
            write_row(&output_path, &row)?;
            if args.json_output {
                println!("{}", serde_json::to_string_pretty(&row)?);
            } else {
                let display = safe_output_ref(&args.repo_root, &output_path);
                println!("issue_resource_telemetry={display}");
            }
        }
        TelemetryAction::Archive => {
            let uploader = AwsCliUploader;
            let manifest = build_archive_manifest(&args, &uploader)?;
            let manifest_out = args
                .manifest_out
                .clone()
                .unwrap_or_else(|| default_manifest_path(&args.repo_root, args.issue_number));
            write_archive_manifest(&manifest_out, &manifest, &uploader)?;
            if args.json_output {
                println!("{}", serde_json::to_string_pretty(&manifest)?);
            } else {
                let display = safe_output_ref(&args.repo_root, &manifest_out);
                println!("issue_resource_telemetry_manifest={display}");
            }
        }
    }
    Ok(())
}

fn parse_args(args: &[String]) -> Result<TelemetryArgs> {
    let Some(action) = args.first().map(String::as_str) else {
        bail!("{}", usage());
    };
    let action = match action {
        "collect" => TelemetryAction::Collect,
        "archive" => TelemetryAction::Archive,
        "--help" | "-h" | "help" => {
            println!("{}", usage());
            return Err(anyhow!("help requested"));
        }
        other => bail!(
            "unknown issue-resource-telemetry action '{other}'\n{}",
            usage()
        ),
    };

    let mut issue_number = None;
    let mut issue_slug = None;
    let mut capture_stage = None;
    let mut host_label = Some(APPROVED_LOCAL_HOST_LABEL.to_string());
    let mut repo_root = Some(std::env::current_dir()?);
    let mut out = None;
    let mut input = None;
    let mut manifest_out = None;
    let mut s3_prefix = None;
    let mut repo = None;
    let mut captured_at = None;
    let mut processes = Vec::new();
    let mut upload = false;
    let mut upload_manifest = false;
    let mut redaction_status = "not_redacted_private_archive_manifest_only".to_string();
    let mut json_output = false;

    let mut i = 1usize;
    while i < args.len() {
        match args[i].as_str() {
            "--issue" => {
                let value = require_value(args, &mut i, "--issue")?;
                issue_number = Some(parse_issue_number(&value)?);
            }
            "--issue-slug" => issue_slug = Some(require_value(args, &mut i, "--issue-slug")?),
            "--capture-stage" => {
                capture_stage = Some(require_value(args, &mut i, "--capture-stage")?)
            }
            "--host-label" => host_label = Some(require_value(args, &mut i, "--host-label")?),
            "--repo-root" => {
                repo_root = Some(PathBuf::from(require_value(args, &mut i, "--repo-root")?))
            }
            "--out" => out = Some(PathBuf::from(require_value(args, &mut i, "--out")?)),
            "--input" => input = Some(PathBuf::from(require_value(args, &mut i, "--input")?)),
            "--manifest-out" => {
                manifest_out = Some(PathBuf::from(require_value(
                    args,
                    &mut i,
                    "--manifest-out",
                )?))
            }
            "--s3-prefix" => s3_prefix = Some(require_value(args, &mut i, "--s3-prefix")?),
            "--repo" => repo = Some(require_value(args, &mut i, "--repo")?),
            "--captured-at" => captured_at = Some(require_value(args, &mut i, "--captured-at")?),
            "--process" => processes.push(parse_process_spec(&require_value(
                args,
                &mut i,
                "--process",
            )?)?),
            "--pid-file-process" => processes.push(parse_pid_file_process_spec(&require_value(
                args,
                &mut i,
                "--pid-file-process",
            )?)?),
            "--upload" => upload = true,
            "--upload-manifest" => upload_manifest = true,
            "--redaction-status" => {
                redaction_status = require_value(args, &mut i, "--redaction-status")?;
                validate_redaction_status(&redaction_status)?;
            }
            "--json" => json_output = true,
            other => bail!("unknown issue-resource-telemetry argument '{other}'"),
        }
        i += 1;
    }

    let issue_number =
        issue_number.ok_or_else(|| anyhow!("issue-resource-telemetry collect requires --issue"))?;
    let issue_slug = issue_slug
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| anyhow!("issue-resource-telemetry collect requires --issue-slug"))?;
    let capture_stage = capture_stage.filter(|value| !value.trim().is_empty());
    if matches!(action, TelemetryAction::Collect) {
        let value = capture_stage
            .as_deref()
            .ok_or_else(|| anyhow!("issue-resource-telemetry collect requires --capture-stage"))?;
        validate_capture_stage(value)?;
    }
    let host_label = host_label
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| anyhow!("issue-resource-telemetry collect requires --host-label"))?;
    validate_host_label(&host_label)?;
    let repo_root = repo_root.expect("repo_root defaulted");
    if !repo_root.exists() {
        bail!("--repo-root does not exist: {}", repo_root.display());
    }
    if let Some(ref value) = captured_at {
        parse_captured_at(value)?;
    }
    if let Some(ref value) = s3_prefix {
        if !value.starts_with("s3://") {
            bail!("--s3-prefix must start with s3://");
        }
    }
    if upload_manifest && !upload {
        bail!("--upload-manifest requires --upload");
    }
    validate_redaction_status(&redaction_status)?;
    if matches!(action, TelemetryAction::Archive) {
        if input.is_none() {
            bail!("issue-resource-telemetry archive requires --input");
        }
        if manifest_out.is_none() {
            bail!("issue-resource-telemetry archive requires --manifest-out");
        }
        if s3_prefix.is_none() {
            bail!("issue-resource-telemetry archive requires --s3-prefix s3://bucket/prefix");
        }
        if captured_at.is_none() {
            bail!("issue-resource-telemetry archive requires --captured-at");
        }
        if !processes.is_empty() {
            bail!("issue-resource-telemetry archive does not accept process inputs");
        }
    }

    Ok(TelemetryArgs {
        action,
        issue_number,
        issue_slug,
        capture_stage,
        host_label,
        repo_root,
        out,
        input,
        manifest_out,
        s3_prefix,
        repo,
        captured_at,
        processes,
        upload,
        upload_manifest,
        redaction_status,
        json_output,
    })
}

fn usage() -> &'static str {
    "adl tooling issue-resource-telemetry collect --issue <number> --issue-slug <slug> --capture-stage <issue_start|pre_validation|post_validation|review_handoff|custom_stage> [--host-label wuji] [--process <role:pid>] [--pid-file-process <role:path>] [--captured-at <rfc3339>] [--repo-root <path>] [--out <path>] [--json]\n\
adl tooling issue-resource-telemetry archive --issue <number> --issue-slug <slug> --input <telemetry.jsonl> --manifest-out <manifest.json> --s3-prefix s3://bucket/prefix [--repo owner/repo] [--host-label <label>] --captured-at <rfc3339> [--redaction-status <status>] [--upload] [--upload-manifest] [--repo-root <path>] [--json]"
}

fn require_value(args: &[String], i: &mut usize, flag: &str) -> Result<String> {
    *i += 1;
    args.get(*i)
        .filter(|value| !value.trim().is_empty())
        .cloned()
        .ok_or_else(|| anyhow!("{flag} requires a non-empty value"))
}

fn parse_issue_number(value: &str) -> Result<u32> {
    value
        .parse::<u32>()
        .with_context(|| format!("invalid issue number '{value}'"))
}

fn validate_capture_stage(value: &str) -> Result<()> {
    if value
        .chars()
        .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '_')
    {
        return Ok(());
    }
    bail!(
        "--capture-stage must be snake_case like {} or similar; got '{value}'",
        DEFAULT_CAPTURE_STAGES.join("|")
    )
}

fn validate_host_label(value: &str) -> Result<()> {
    let normalized = canonical_host_label(value)?;
    if normalized.is_empty() {
        bail!("--host-label cannot be empty");
    }
    Ok(())
}

fn validate_redaction_status(value: &str) -> Result<()> {
    match value {
        "not_redacted_private_archive_manifest_only"
        | "redacted_private_archive"
        | "redacted_review_safe_summary" => Ok(()),
        other => bail!(
            "unsupported --redaction-status '{other}' (expected not_redacted_private_archive_manifest_only | redacted_private_archive | redacted_review_safe_summary)"
        ),
    }
}

fn parse_captured_at(value: &str) -> Result<DateTime<Utc>> {
    Ok(DateTime::parse_from_rfc3339(value)
        .with_context(|| format!("invalid --captured-at '{value}'"))?
        .with_timezone(&Utc))
}

fn archive_capture_token(value: &DateTime<Utc>) -> String {
    value.to_rfc3339_opts(SecondsFormat::Secs, true)
}

fn parse_process_spec(value: &str) -> Result<TrackedProcessSpec> {
    let (role, pid_text) = value
        .split_once(':')
        .ok_or_else(|| anyhow!("--process must use role:pid"))?;
    let role = normalize_role(role)?;
    let pid = pid_text
        .parse::<u32>()
        .with_context(|| format!("invalid pid in --process '{value}'"))?;
    if pid == 0 {
        bail!("--process pid must be greater than zero");
    }
    Ok(TrackedProcessSpec::Pid { role, pid })
}

fn parse_pid_file_process_spec(value: &str) -> Result<TrackedProcessSpec> {
    let (role, path) = value
        .split_once(':')
        .ok_or_else(|| anyhow!("--pid-file-process must use role:path"))?;
    let role = normalize_role(role)?;
    let path = path.trim();
    if path.is_empty() {
        bail!("--pid-file-process path cannot be empty");
    }
    Ok(TrackedProcessSpec::PidFile {
        role,
        path: PathBuf::from(path),
    })
}

fn normalize_role(value: &str) -> Result<String> {
    let value = value.trim();
    if value.is_empty() {
        bail!("process role cannot be empty");
    }
    if !value
        .chars()
        .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '_')
    {
        bail!("process role must be snake_case");
    }
    Ok(value.to_string())
}

fn default_output_path(repo_root: &Path, issue_number: u32) -> PathBuf {
    repo_root
        .join(".adl/runs/issues")
        .join(format!("issue-{issue_number}"))
        .join("telemetry/issue_resource_telemetry.v1.jsonl")
}

fn default_manifest_path(repo_root: &Path, issue_number: u32) -> PathBuf {
    repo_root
        .join(".adl/runs/issues")
        .join(format!("issue-{issue_number}"))
        .join("telemetry/issue_resource_telemetry_manifest.v1.json")
}

fn safe_output_ref(repo_root: &Path, output_path: &Path) -> String {
    match output_path.strip_prefix(repo_root) {
        Ok(_) => repo_relative_display(repo_root, output_path)
            .unwrap_or_else(|_| "issue_resource_telemetry.v1.jsonl".to_string()),
        Err(_) => {
            let basename = output_path
                .file_name()
                .and_then(|value| value.to_str())
                .unwrap_or("issue_resource_telemetry.v1.jsonl");
            format!("<external>/{basename}")
        }
    }
}

fn canonical_host_label(value: &str) -> Result<String> {
    let trimmed = value.trim().to_ascii_lowercase();
    let normalized = trimmed
        .strip_suffix(".agent-logic.ai")
        .or_else(|| trimmed.strip_suffix(".local"))
        .unwrap_or(trimmed.as_str())
        .trim();
    if normalized.is_empty() {
        bail!("host label cannot be empty");
    }
    if !normalized
        .chars()
        .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '_' || ch == '-')
    {
        bail!("host label must use lowercase letters, digits, '-' or '_'");
    }
    Ok(normalized.to_string())
}

fn resolve_host_policy(label: &str) -> Result<HostPolicy> {
    let normalized = canonical_host_label(label)?;
    if normalized == APPROVED_LOCAL_HOST_LABEL {
        return Ok(HostPolicy {
            label: normalized.clone(),
            classification: "operator_named_local_host".to_string(),
            approval_state: "approved_label".to_string(),
            archive_host_ref: normalized,
        });
    }
    if APPROVED_CSM_HOST_LABELS.contains(&normalized.as_str()) {
        return Ok(HostPolicy {
            label: normalized.clone(),
            classification: "operator_named_csm_host".to_string(),
            approval_state: "approved_label".to_string(),
            archive_host_ref: normalized,
        });
    }
    Ok(HostPolicy {
        label: REDACTED_HOST_LABEL.to_string(),
        classification: "redacted_remote_host".to_string(),
        approval_state: "redacted_label".to_string(),
        archive_host_ref: REDACTED_HOST_LABEL.to_string(),
    })
}

fn build_row(
    args: &TelemetryArgs,
    runner: &dyn CommandRunner,
) -> Result<IssueResourceTelemetryRow> {
    match args.action {
        TelemetryAction::Collect => {}
        TelemetryAction::Archive => {
            bail!("build_row only supports the collect action");
        }
    }
    let captured_at = args
        .captured_at
        .as_deref()
        .map(parse_captured_at)
        .transpose()?
        .unwrap_or_else(Utc::now)
        .to_rfc3339();
    let cpu = collect_cpu(runner).unwrap_or_else(|_| MetricField::not_available());
    let memory = collect_memory(runner).unwrap_or_else(|_| MetricField::not_available());
    let disk = collect_disk(runner).unwrap_or_else(|_| MetricField::not_available());
    let process_summary = collect_process_summary(runner, &args.processes)
        .unwrap_or_else(|_| MetricField::not_available());
    let host_policy = resolve_host_policy(&args.host_label)?;
    let row = IssueResourceTelemetryRow {
        schema_version: SCHEMA_VERSION.to_string(),
        issue_number: args.issue_number,
        issue_slug: args.issue_slug.clone(),
        captured_at,
        capture_stage: args
            .capture_stage
            .clone()
            .expect("collect validated capture_stage"),
        host: HostRecord {
            label: host_policy.label,
            classification: host_policy.classification,
            approval_state: host_policy.approval_state,
        },
        data_source: DataSourceRecord {
            collector: "bounded_local_sampler_v1".to_string(),
            sampling_scope: "issue_execution".to_string(),
            sampling_mode: "point_in_time".to_string(),
        },
        cpu,
        memory,
        disk,
        gpu: MetricField::not_available(),
        process_summary,
        archive: ArchiveRecord {
            redaction_status: "not_redacted_private_archive_manifest_only".to_string(),
            local_retention: "local_ignored_until_private_archive".to_string(),
            private_archive_ref: None,
        },
    };
    let rendered = serde_json::to_string(&row)?;
    if contains_absolute_host_path_in_text(&rendered) {
        bail!("issue resource telemetry row contains an absolute host path");
    }
    Ok(row)
}

fn build_archive_manifest(
    args: &TelemetryArgs,
    uploader: &dyn ArchiveUploader,
) -> Result<IssueResourceTelemetryArchiveManifest> {
    let input = args
        .input
        .as_ref()
        .expect("archive validated input")
        .to_path_buf();
    if !input.is_file() {
        bail!("--input is not a file: {}", input.display());
    }
    let captured_at = args
        .captured_at
        .as_deref()
        .expect("archive validated captured_at");
    let captured_at = parse_captured_at(captured_at)?;
    let archive_capture_token = archive_capture_token(&captured_at);
    let captured_at = captured_at.to_rfc3339_opts(SecondsFormat::Secs, true);
    let repo = resolve_repo_slug(args)?;
    let host_policy = resolve_host_policy(&args.host_label)?;
    let row_count = validate_telemetry_jsonl(&input)?;
    let raw_telemetry_ref = archive_raw_telemetry_ref(
        args.s3_prefix
            .as_deref()
            .expect("archive validated s3_prefix"),
        &repo,
        args.issue_number,
        &host_policy.archive_host_ref,
        &archive_capture_token,
    );
    let manifest_ref = if args.upload_manifest {
        Some(archive_manifest_ref(
            args.s3_prefix
                .as_deref()
                .expect("archive validated s3_prefix"),
            &repo,
            args.issue_number,
            &host_policy.archive_host_ref,
            &archive_capture_token,
        ))
    } else {
        None
    };
    let upload_status = if args.upload {
        uploader.upload(&input, &raw_telemetry_ref)?;
        "uploaded".to_string()
    } else {
        "upload_not_run".to_string()
    };
    let manifest_upload_status = if args.upload_manifest {
        "pending_manifest_write".to_string()
    } else {
        "upload_not_run".to_string()
    };
    let manifest = IssueResourceTelemetryArchiveManifest {
        schema_version: ARCHIVE_MANIFEST_SCHEMA_VERSION.to_string(),
        generated_at: Utc::now().to_rfc3339(),
        source: ArchiveSourceRecord {
            issue_number: args.issue_number,
            issue_slug: args.issue_slug.clone(),
            captured_at,
            input_ref: safe_output_ref(&args.repo_root, &input),
            repo,
            row_count,
        },
        host: HostRecord {
            label: host_policy.label,
            classification: host_policy.classification,
            approval_state: host_policy.approval_state,
        },
        archive: ArchiveManifestArchiveRecord {
            backend: "s3".to_string(),
            raw_telemetry_ref,
            manifest_ref,
            upload_status,
            manifest_upload_status,
            redaction_status: args.redaction_status.clone(),
            local_retention: "local_ignored_until_private_archive".to_string(),
        },
        consumption_policy: ArchiveConsumptionPolicy {
            raw_telemetry_is_private_evidence: true,
            review_safe_summary_is_separate: true,
            fail_closed_on_upload_or_redaction_error: true,
        },
    };
    let rendered = serde_json::to_string(&manifest)?;
    if contains_absolute_host_path_in_text(&rendered) {
        bail!("issue resource telemetry archive manifest contains an absolute host path");
    }
    Ok(manifest)
}

fn write_archive_manifest(
    path: &Path,
    manifest: &IssueResourceTelemetryArchiveManifest,
    uploader: &dyn ArchiveUploader,
) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).with_context(|| {
            format!(
                "failed creating telemetry manifest dir '{}'",
                parent.display()
            )
        })?;
    }
    let manifest = manifest.clone();
    let rendered = serde_json::to_string_pretty(&manifest)? + "\n";
    fs::write(path, &rendered).with_context(|| format!("failed writing '{}'", path.display()))?;
    if let Some(manifest_ref) = manifest.archive.manifest_ref.clone() {
        if manifest.archive.upload_status != "uploaded" {
            bail!("manifest upload requires raw telemetry upload to have completed");
        }
        let mut uploaded_manifest = manifest;
        uploaded_manifest.archive.manifest_upload_status = "uploaded".to_string();
        let uploaded_rendered = serde_json::to_string_pretty(&uploaded_manifest)? + "\n";
        let upload_path = path.with_extension("upload.json");
        fs::write(&upload_path, &uploaded_rendered).with_context(|| {
            format!(
                "failed writing upload manifest staging file '{}'",
                upload_path.display()
            )
        })?;
        uploader.upload(&upload_path, &manifest_ref)?;
        fs::write(path, uploaded_rendered)
            .with_context(|| format!("failed writing uploaded manifest '{}'", path.display()))?;
        let _ = fs::remove_file(&upload_path);
    }
    Ok(())
}

fn validate_telemetry_jsonl(path: &Path) -> Result<usize> {
    let raw = fs::read_to_string(path)
        .with_context(|| format!("failed reading telemetry jsonl '{}'", path.display()))?;
    let mut row_count = 0usize;
    for line in raw.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let value: serde_json::Value = serde_json::from_str(trimmed)
            .with_context(|| format!("invalid telemetry jsonl row in '{}'", path.display()))?;
        if value.get("schema_version").and_then(|v| v.as_str()) != Some(SCHEMA_VERSION) {
            bail!("telemetry jsonl row has unexpected schema_version");
        }
        if contains_absolute_host_path_in_text(trimmed) {
            bail!("telemetry jsonl row contains an absolute host path");
        }
        row_count += 1;
    }
    if row_count == 0 {
        bail!("telemetry jsonl input must contain at least one row");
    }
    Ok(row_count)
}

fn resolve_repo_slug(args: &TelemetryArgs) -> Result<String> {
    if let Some(repo) = args.repo.as_deref() {
        return Ok(sanitize_key_component(repo));
    }
    let output = Command::new("git")
        .args(["remote", "get-url", "origin"])
        .current_dir(&args.repo_root)
        .output()
        .context("failed to invoke git remote get-url origin")?;
    if !output.status.success() {
        bail!("git remote get-url origin failed while resolving repo slug");
    }
    let raw = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let without_suffix = raw.trim_end_matches(".git");
    let repo = without_suffix
        .rsplit_once(':')
        .map(|(_, right)| right)
        .or_else(|| without_suffix.rsplit_once('/').map(|(_, right)| right))
        .unwrap_or(without_suffix);
    if raw.contains("github.com/") {
        let right = without_suffix.split("github.com/").nth(1).unwrap_or(repo);
        return Ok(sanitize_key_component(right));
    }
    Ok(sanitize_key_component(repo))
}

fn archive_raw_telemetry_ref(
    s3_prefix: &str,
    repo: &str,
    issue_number: u32,
    host_ref: &str,
    captured_at: &str,
) -> String {
    format!(
        "{}/{}/issues/issue-{}/host-{}/capture-{}/issue_resource_telemetry.v1.jsonl",
        s3_prefix.trim_end_matches('/'),
        repo,
        issue_number,
        sanitize_key_component(host_ref),
        sanitize_key_component(captured_at),
    )
}

fn archive_manifest_ref(
    s3_prefix: &str,
    repo: &str,
    issue_number: u32,
    host_ref: &str,
    captured_at: &str,
) -> String {
    format!(
        "{}/{}/issues/issue-{}/host-{}/capture-{}/issue_resource_telemetry_manifest.v1.json",
        s3_prefix.trim_end_matches('/'),
        repo,
        issue_number,
        sanitize_key_component(host_ref),
        sanitize_key_component(captured_at),
    )
}

fn sanitize_key_component(value: &str) -> String {
    value
        .chars()
        .map(|ch| match ch {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' | '.' => ch,
            _ => '-',
        })
        .collect()
}

fn write_row(path: &Path, row: &IssueResourceTelemetryRow) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed creating telemetry dir '{}'", parent.display()))?;
    }
    let mut existing = if path.is_file() {
        fs::read_to_string(path).with_context(|| format!("failed reading '{}'", path.display()))?
    } else {
        String::new()
    };
    existing.push_str(&serde_json::to_string(row)?);
    existing.push('\n');
    fs::write(path, existing).with_context(|| format!("failed writing '{}'", path.display()))?;
    Ok(())
}

fn collect_cpu(runner: &dyn CommandRunner) -> Result<MetricField<CpuRecord>> {
    let logical_cores = runner
        .run("sysctl", &["-n", "hw.logicalcpu"])?
        .trim()
        .parse::<u32>()
        .context("parse hw.logicalcpu")?;
    let load_avg_1m = parse_first_float(&runner.run("sysctl", &["-n", "vm.loadavg"])?)
        .context("parse vm.loadavg")?;
    let utilization_pct = parse_top_cpu_usage(&runner.run("top", &["-l", "1", "-n", "0"])?)
        .context("parse top cpu usage")?;
    Ok(MetricField::Value(CpuRecord {
        logical_cores,
        load_avg_1m: round3(load_avg_1m),
        utilization_pct: round3(utilization_pct),
    }))
}

fn collect_memory(runner: &dyn CommandRunner) -> Result<MetricField<MemoryRecord>> {
    let total_bytes = runner
        .run("sysctl", &["-n", "hw.memsize"])?
        .trim()
        .parse::<u64>()
        .context("parse hw.memsize")?;
    let vm_stat = runner.run("vm_stat", &[])?;
    let (available_bytes, used_bytes) = parse_vm_stat_memory(&vm_stat, total_bytes)?;
    let pressure_state = classify_pressure_state(total_bytes, available_bytes);
    Ok(MetricField::Value(MemoryRecord {
        total_bytes,
        available_bytes,
        used_bytes,
        pressure_state,
    }))
}

fn collect_disk(runner: &dyn CommandRunner) -> Result<MetricField<Vec<DiskRecord>>> {
    let df = runner.run("df", &["-k", "/"])?;
    Ok(MetricField::Value(parse_df_k_output(&df)?))
}

fn collect_process_summary(
    runner: &dyn CommandRunner,
    specs: &[TrackedProcessSpec],
) -> Result<MetricField<ProcessSummaryRecord>> {
    let mut resolved = if specs.is_empty() {
        vec![TrackedProcessSpec::Pid {
            role: "collector".to_string(),
            pid: std::process::id(),
        }]
    } else {
        specs.to_vec()
    };
    let mut heavy_processes = Vec::new();
    for spec in resolved.drain(..) {
        if let Ok((role, pid)) = resolve_process_spec(spec) {
            if let Ok(record) = collect_process_record(runner, &role, pid) {
                heavy_processes.push(record);
            }
        }
    }
    if heavy_processes.is_empty() {
        return Ok(MetricField::not_available());
    }
    heavy_processes.sort_by(|left, right| {
        right
            .rss_bytes
            .cmp(&left.rss_bytes)
            .then_with(|| {
                right
                    .cpu_pct
                    .partial_cmp(&left.cpu_pct)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .then_with(|| left.pid.cmp(&right.pid))
    });
    Ok(MetricField::Value(ProcessSummaryRecord {
        tracked_process_count: heavy_processes.len() as u32,
        heavy_processes,
    }))
}

fn resolve_process_spec(spec: TrackedProcessSpec) -> Result<(String, u32)> {
    match spec {
        TrackedProcessSpec::Pid { role, pid } => Ok((role, pid)),
        TrackedProcessSpec::PidFile { role, path } => {
            let raw = fs::read_to_string(&path)
                .with_context(|| format!("failed reading pid file '{}'", path.display()))?;
            let pid = raw.trim().parse::<u32>().with_context(|| {
                format!(
                    "pid file '{}' does not contain a positive integer",
                    path.display()
                )
            })?;
            if pid == 0 {
                bail!(
                    "pid file '{}' must contain a pid greater than zero",
                    path.display()
                );
            }
            Ok((role, pid))
        }
    }
}

fn collect_process_record(
    runner: &dyn CommandRunner,
    role: &str,
    pid: u32,
) -> Result<TrackedProcessRecord> {
    let pid_text = pid.to_string();
    let output = runner.run(
        "ps",
        &[
            "-p",
            pid_text.as_str(),
            "-o",
            "%cpu=",
            "-o",
            "rss=",
            "-o",
            "comm=",
        ],
    )?;
    let line = output
        .lines()
        .find(|line| !line.trim().is_empty())
        .ok_or_else(|| anyhow!("ps returned no row for pid {pid}"))?;
    parse_process_row(role, pid, line)
}

fn parse_process_row(role: &str, pid: u32, line: &str) -> Result<TrackedProcessRecord> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() < 3 {
        bail!("process row must contain cpu, rss, and command fields");
    }
    let cpu_pct = parts[0]
        .parse::<f64>()
        .with_context(|| format!("invalid cpu pct '{}'", parts[0]))?;
    let rss_kb = parts[1]
        .parse::<u64>()
        .with_context(|| format!("invalid rss '{}'", parts[1]))?;
    let executable = Path::new(parts[2])
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or(parts[2])
        .to_string();
    Ok(TrackedProcessRecord {
        role: role.to_string(),
        executable_basename: executable,
        pid,
        cpu_pct: round3(cpu_pct),
        rss_bytes: rss_kb.saturating_mul(1024),
    })
}

fn parse_first_float(text: &str) -> Result<f64> {
    numeric_tokens(text)
        .into_iter()
        .find_map(|token| token.parse::<f64>().ok())
        .ok_or_else(|| anyhow!("no float found"))
}

fn parse_top_cpu_usage(text: &str) -> Result<f64> {
    let line = text
        .lines()
        .find(|line| line.contains("CPU usage:"))
        .ok_or_else(|| anyhow!("missing CPU usage line"))?;
    let user = extract_percent_before(line, "% user");
    let sys = extract_percent_before(line, "% sys");
    if let (Some(user), Some(sys)) = (user, sys) {
        return Ok(user + sys);
    }
    let idle = extract_percent_before(line, "% idle")
        .ok_or_else(|| anyhow!("missing idle percentage in CPU usage line"))?;
    Ok(100.0 - idle)
}

fn extract_percent_before(text: &str, marker: &str) -> Option<f64> {
    let index = text.find(marker)?;
    let prefix = &text[..index];
    let token = prefix
        .split(|ch: char| ch == ',' || ch.is_whitespace())
        .rfind(|value| !value.is_empty())?;
    token.trim_end_matches('%').parse::<f64>().ok()
}

fn parse_vm_stat_memory(text: &str, total_bytes: u64) -> Result<(u64, u64)> {
    let page_size = text
        .lines()
        .find(|line| line.contains("page size of"))
        .and_then(|line| {
            numeric_tokens(line)
                .into_iter()
                .find_map(|value| value.parse::<u64>().ok())
        })
        .ok_or_else(|| anyhow!("vm_stat output missing page size"))?;
    let free_pages = parse_named_pages(text, "Pages free")?;
    let inactive_pages = parse_named_pages(text, "Pages inactive").unwrap_or(0);
    let speculative_pages = parse_named_pages(text, "Pages speculative").unwrap_or(0);
    let available_bytes = free_pages
        .saturating_add(inactive_pages)
        .saturating_add(speculative_pages)
        .saturating_mul(page_size);
    let available_bytes = available_bytes.min(total_bytes);
    let used_bytes = total_bytes.saturating_sub(available_bytes);
    Ok((available_bytes, used_bytes))
}

fn parse_named_pages(text: &str, label: &str) -> Result<u64> {
    let line = text
        .lines()
        .find(|line| line.trim_start().starts_with(label))
        .ok_or_else(|| anyhow!("vm_stat output missing '{label}'"))?;
    numeric_tokens(line)
        .into_iter()
        .find_map(|value| value.trim_end_matches('.').parse::<u64>().ok())
        .ok_or_else(|| anyhow!("vm_stat output missing page count for '{label}'"))
}

fn classify_pressure_state(total_bytes: u64, available_bytes: u64) -> String {
    if total_bytes == 0 {
        return "unknown".to_string();
    }
    let available_ratio = available_bytes as f64 / total_bytes as f64;
    if available_ratio >= 0.20 {
        "normal".to_string()
    } else if available_ratio >= 0.10 {
        "warning".to_string()
    } else {
        "critical".to_string()
    }
}

fn parse_df_k_output(text: &str) -> Result<Vec<DiskRecord>> {
    let line = text
        .lines()
        .find(|line| {
            let trimmed = line.trim();
            !trimmed.is_empty() && !trimmed.starts_with("Filesystem")
        })
        .ok_or_else(|| anyhow!("df output missing data line"))?;
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() < 4 {
        bail!("df output data line was too short");
    }
    let total_bytes = parts[1]
        .parse::<u64>()
        .context("parse df total blocks")?
        .saturating_mul(1024);
    let used_bytes = parts[2]
        .parse::<u64>()
        .context("parse df used blocks")?
        .saturating_mul(1024);
    let available_bytes = parts[3]
        .parse::<u64>()
        .context("parse df available blocks")?
        .saturating_mul(1024);
    Ok(vec![DiskRecord {
        mount_label: "system".to_string(),
        filesystem_class: "system_volume".to_string(),
        total_bytes,
        available_bytes,
        used_bytes,
    }])
}

fn numeric_tokens(text: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    for ch in text.chars() {
        if ch.is_ascii_digit() || ch == '.' {
            current.push(ch);
        } else if !current.is_empty() {
            tokens.push(current.clone());
            current.clear();
        }
    }
    if !current.is_empty() {
        tokens.push(current);
    }
    tokens
}

fn round3(value: f64) -> f64 {
    (value * 1000.0).round() / 1000.0
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::sync::Mutex;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_repo_root(label: &str) -> PathBuf {
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock before unix epoch")
            .as_nanos();
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("adl crate lives under repo root")
            .join(".tmp/tooling_cmd_tests")
            .join(format!("{label}-{stamp}"));
        fs::create_dir_all(&path).expect("create temp repo root");
        path
    }

    struct FakeCommandRunner {
        outputs: HashMap<String, Result<String>>,
    }

    #[derive(Debug, Clone)]
    struct UploadRecord {
        source: PathBuf,
        dest: String,
        content: String,
    }

    struct FakeArchiveUploader {
        fail: bool,
        uploads: Mutex<Vec<UploadRecord>>,
    }

    impl FakeCommandRunner {
        fn new(outputs: HashMap<String, Result<String>>) -> Self {
            Self { outputs }
        }

        fn key(program: &str, args: &[&str]) -> String {
            format!("{program}\u{1f}{}", args.join("\u{1f}"))
        }
    }

    impl CommandRunner for FakeCommandRunner {
        fn run(&self, program: &str, args: &[&str]) -> Result<String> {
            match self.outputs.get(&Self::key(program, args)) {
                Some(Ok(value)) => Ok(value.clone()),
                Some(Err(err)) => Err(anyhow!(err.to_string())),
                None => Err(anyhow!("missing fixture for {program} {:?}", args)),
            }
        }
    }

    impl FakeArchiveUploader {
        fn succeed() -> Self {
            Self {
                fail: false,
                uploads: Mutex::new(Vec::new()),
            }
        }

        fn fail() -> Self {
            Self {
                fail: true,
                uploads: Mutex::new(Vec::new()),
            }
        }
    }

    impl ArchiveUploader for FakeArchiveUploader {
        fn upload(&self, source: &Path, dest: &str) -> Result<()> {
            if self.fail {
                bail!("simulated upload failure");
            }
            let content = fs::read_to_string(source)
                .with_context(|| format!("read '{}'", source.display()))?;
            self.uploads
                .lock()
                .expect("lock uploads")
                .push(UploadRecord {
                    source: source.to_path_buf(),
                    dest: dest.to_string(),
                    content,
                });
            Ok(())
        }
    }

    fn fake_runner_with_supported_metrics() -> FakeCommandRunner {
        let mut outputs = HashMap::new();
        outputs.insert(
            FakeCommandRunner::key("sysctl", &["-n", "hw.logicalcpu"]),
            Ok("10".to_string()),
        );
        outputs.insert(
            FakeCommandRunner::key("sysctl", &["-n", "vm.loadavg"]),
            Ok("{ 1.24 1.57 1.49 }".to_string()),
        );
        outputs.insert(
            FakeCommandRunner::key("top", &["-l", "1", "-n", "0"]),
            Ok(
                "Processes: 100 total\nCPU usage: 11.20% user, 8.80% sys, 80.00% idle\n"
                    .to_string(),
            ),
        );
        outputs.insert(
            FakeCommandRunner::key("sysctl", &["-n", "hw.memsize"]),
            Ok("32000000000".to_string()),
        );
        outputs.insert(
            FakeCommandRunner::key("vm_stat", &[]),
            Ok(
                "Mach Virtual Memory Statistics: (page size of 4096 bytes)\nPages free: 500000.\nPages inactive: 250000.\nPages speculative: 100000.\n".to_string(),
            ),
        );
        outputs.insert(
            FakeCommandRunner::key("df", &["-k", "/"]),
            Ok(
                "Filesystem 1024-blocks Used Available Capacity iused ifree %iused Mounted on\n/dev/disk3s1 976490576 476490576 500000000 49% 1 1 1% /\n".to_string(),
            ),
        );
        outputs.insert(
            FakeCommandRunner::key(
                "ps",
                &["-p", "4242", "-o", "%cpu=", "-o", "rss=", "-o", "comm="],
            ),
            Ok("18.5 716800 /usr/bin/cargo\n".to_string()),
        );
        FakeCommandRunner::new(outputs)
    }

    #[test]
    fn issue_resource_telemetry_collect_builds_supported_row_and_writes_jsonl() {
        let repo_root = temp_repo_root("issue-resource-telemetry");
        let output = repo_root
            .join(".adl/runs/issues/issue-4298/telemetry/issue_resource_telemetry.v1.jsonl");
        let args = TelemetryArgs {
            action: TelemetryAction::Collect,
            issue_number: 4298,
            issue_slug:
                "v0-91-6-observability-telemetry-implement-wuji-issue-resource-telemetry-collector"
                    .to_string(),
            capture_stage: Some("issue_start".to_string()),
            host_label: APPROVED_LOCAL_HOST_LABEL.to_string(),
            repo_root: repo_root.clone(),
            out: Some(output.clone()),
            input: None,
            manifest_out: None,
            s3_prefix: None,
            repo: None,
            captured_at: Some("2026-06-20T09:30:00Z".to_string()),
            processes: vec![TrackedProcessSpec::Pid {
                role: "validation".to_string(),
                pid: 4242,
            }],
            upload: false,
            upload_manifest: false,
            redaction_status: "not_redacted_private_archive_manifest_only".to_string(),
            json_output: false,
        };

        let row = build_row(&args, &fake_runner_with_supported_metrics()).expect("row");
        assert_eq!(row.schema_version, SCHEMA_VERSION);
        assert_eq!(row.host.label, "wuji");
        assert_eq!(row.archive.private_archive_ref, None);
        assert_eq!(
            row.gpu,
            MetricField::NotAvailable("not_available".to_string())
        );
        assert!(matches!(row.cpu, MetricField::Value(_)));
        assert!(matches!(row.memory, MetricField::Value(_)));
        assert!(matches!(row.disk, MetricField::Value(_)));
        assert!(matches!(row.process_summary, MetricField::Value(_)));

        write_row(&output, &row).expect("write row");
        let raw = fs::read_to_string(&output).expect("telemetry jsonl");
        let line = raw.lines().next().expect("first jsonl line");
        let value: serde_json::Value = serde_json::from_str(line).expect("json line");
        assert_eq!(value["schema_version"], SCHEMA_VERSION);
        assert_eq!(value["host"]["label"], "wuji");
        assert_eq!(value["gpu"], "not_available");
        assert!(!contains_absolute_host_path_in_text(line));
        assert!(!line.contains("/usr/bin/cargo"));
        let _ = fs::remove_dir_all(repo_root);
    }

    #[test]
    fn issue_resource_telemetry_collect_falls_back_to_not_available_for_missing_metric_families() {
        let repo_root = temp_repo_root("issue-resource-telemetry-not-available");
        let args = TelemetryArgs {
            action: TelemetryAction::Collect,
            issue_number: 4298,
            issue_slug: "collector".to_string(),
            capture_stage: Some("pre_validation".to_string()),
            host_label: APPROVED_LOCAL_HOST_LABEL.to_string(),
            repo_root: repo_root.clone(),
            out: None,
            input: None,
            manifest_out: None,
            s3_prefix: None,
            repo: None,
            captured_at: Some("2026-06-20T09:30:00Z".to_string()),
            processes: vec![],
            upload: false,
            upload_manifest: false,
            redaction_status: "not_redacted_private_archive_manifest_only".to_string(),
            json_output: false,
        };

        let row = build_row(&args, &FakeCommandRunner::new(HashMap::new())).expect("row");
        assert_eq!(
            row.cpu,
            MetricField::NotAvailable("not_available".to_string())
        );
        assert_eq!(
            row.memory,
            MetricField::NotAvailable("not_available".to_string())
        );
        assert_eq!(
            row.disk,
            MetricField::NotAvailable("not_available".to_string())
        );
        assert_eq!(
            row.process_summary,
            MetricField::NotAvailable("not_available".to_string())
        );
        assert_eq!(
            row.gpu,
            MetricField::NotAvailable("not_available".to_string())
        );
        let _ = fs::remove_dir_all(repo_root);
    }

    #[test]
    fn issue_resource_telemetry_dispatch_help_and_parse_rules_are_stable() {
        real_issue_resource_telemetry(&["--help".to_string()]).expect("help");

        let err = real_issue_resource_telemetry(&["collect".to_string()])
            .expect_err("missing required args should fail");
        assert!(err.to_string().contains("--issue"));

        let args = parse_args(&[
            "collect".to_string(),
            "--issue".to_string(),
            "4298".to_string(),
            "--issue-slug".to_string(),
            "collector".to_string(),
            "--capture-stage".to_string(),
            "issue_start".to_string(),
            "--host-label".to_string(),
            "other-host".to_string(),
        ])
        .expect("unknown host should redact instead of fail");
        let row = build_row(&args, &FakeCommandRunner::new(HashMap::new())).expect("row");
        assert_eq!(row.host.label, REDACTED_HOST_LABEL);
        assert_eq!(row.host.approval_state, "redacted_label");
    }

    #[test]
    fn issue_resource_telemetry_collect_supports_approved_csm_host_labels() {
        let repo_root = temp_repo_root("issue-resource-telemetry-approved-csm");
        let args = TelemetryArgs {
            action: TelemetryAction::Collect,
            issue_number: 4299,
            issue_slug: "archive".to_string(),
            capture_stage: Some("review_handoff".to_string()),
            host_label: "opticon.local".to_string(),
            repo_root: repo_root.clone(),
            out: None,
            input: None,
            manifest_out: None,
            s3_prefix: None,
            repo: None,
            captured_at: Some("2026-06-20T09:30:00Z".to_string()),
            processes: vec![],
            upload: false,
            upload_manifest: false,
            redaction_status: "not_redacted_private_archive_manifest_only".to_string(),
            json_output: false,
        };
        let row = build_row(&args, &FakeCommandRunner::new(HashMap::new())).expect("row");
        assert_eq!(row.host.label, "opticon");
        assert_eq!(row.host.classification, "operator_named_csm_host");
        assert_eq!(row.host.approval_state, "approved_label");
        let _ = fs::remove_dir_all(repo_root);
    }

    #[test]
    fn issue_resource_telemetry_safe_output_ref_redacts_external_paths() {
        let repo_root = PathBuf::from("/repo");
        let repo_path = repo_root
            .join(".adl/runs/issues/issue-4298/telemetry/issue_resource_telemetry.v1.jsonl");
        assert_eq!(
            safe_output_ref(&repo_root, &repo_path),
            ".adl/runs/issues/issue-4298/telemetry/issue_resource_telemetry.v1.jsonl"
        );

        let external = PathBuf::from("/tmp/private/telemetry.jsonl");
        assert_eq!(
            safe_output_ref(&repo_root, &external),
            "<external>/telemetry.jsonl"
        );
    }

    #[test]
    fn issue_resource_telemetry_extract_percent_before_uses_nearest_token() {
        let line = "CPU usage: 11.20% user, 8.80% sys, 80.00% idle";
        assert_eq!(extract_percent_before(line, "% user"), Some(11.20));
        assert_eq!(extract_percent_before(line, "% sys"), Some(8.80));
        assert_eq!(extract_percent_before(line, "% idle"), Some(80.00));
    }

    #[test]
    fn issue_resource_telemetry_parse_top_cpu_usage_falls_back_to_idle() {
        let line = "Processes: 100 total\nCPU usage: 80.00% idle\n";
        assert_eq!(parse_top_cpu_usage(line).expect("cpu usage"), 20.0);
    }

    #[test]
    fn issue_resource_telemetry_parse_vm_stat_memory_and_pressure_state() {
        let vm_stat = "Mach Virtual Memory Statistics: (page size of 4096 bytes)\nPages free: 500000.\nPages inactive: 250000.\nPages speculative: 100000.\n";
        let (available_bytes, used_bytes) =
            parse_vm_stat_memory(vm_stat, 32_000_000_000).expect("vm_stat");
        assert_eq!(available_bytes, 3_481_600_000);
        assert_eq!(used_bytes, 28_518_400_000);
        assert_eq!(
            classify_pressure_state(32_000_000_000, available_bytes),
            "warning"
        );
    }

    #[test]
    fn issue_resource_telemetry_parse_df_k_output_and_process_row_are_reviewable() {
        let df = "Filesystem 1024-blocks Used Available Capacity iused ifree %iused Mounted on\n/dev/disk3s1 976490576 476490576 500000000 49% 1 1 1% /\n";
        let disks = parse_df_k_output(df).expect("df");
        assert_eq!(disks.len(), 1);
        assert_eq!(disks[0].mount_label, "system");
        assert_eq!(disks[0].filesystem_class, "system_volume");
        assert_eq!(disks[0].total_bytes, 999_926_349_824);
        assert_eq!(disks[0].used_bytes, 487_926_349_824);

        let process = parse_process_row("validation", 4242, "18.5 716800 /usr/bin/cargo")
            .expect("process row");
        assert_eq!(process.role, "validation");
        assert_eq!(process.executable_basename, "cargo");
        assert_eq!(process.pid, 4242);
        assert_eq!(process.cpu_pct, 18.5);
        assert_eq!(process.rss_bytes, 734_003_200);
    }

    #[test]
    fn issue_resource_telemetry_parse_args_accepts_optional_fields_and_process_specs() {
        let repo_root = temp_repo_root("issue-resource-telemetry-parse-args");
        let out = repo_root.join("telemetry.jsonl");
        let args = parse_args(&[
            "collect".to_string(),
            "--issue".to_string(),
            "4298".to_string(),
            "--issue-slug".to_string(),
            "collector".to_string(),
            "--capture-stage".to_string(),
            "custom_stage".to_string(),
            "--host-label".to_string(),
            "wuji".to_string(),
            "--repo-root".to_string(),
            repo_root.display().to_string(),
            "--out".to_string(),
            out.display().to_string(),
            "--captured-at".to_string(),
            "2026-06-20T09:30:00Z".to_string(),
            "--process".to_string(),
            "validation:4242".to_string(),
            "--pid-file-process".to_string(),
            "control_plane:tmp/control.pid".to_string(),
            "--json".to_string(),
        ])
        .expect("args");

        assert_eq!(args.action, TelemetryAction::Collect);
        assert_eq!(args.issue_number, 4298);
        assert_eq!(args.issue_slug, "collector");
        assert_eq!(args.capture_stage.as_deref(), Some("custom_stage"));
        assert_eq!(args.host_label, "wuji");
        assert_eq!(args.repo_root, repo_root);
        assert_eq!(args.out, Some(out));
        assert_eq!(args.captured_at.as_deref(), Some("2026-06-20T09:30:00Z"));
        assert!(args.json_output);
        assert_eq!(
            args.processes,
            vec![
                TrackedProcessSpec::Pid {
                    role: "validation".to_string(),
                    pid: 4242,
                },
                TrackedProcessSpec::PidFile {
                    role: "control_plane".to_string(),
                    path: PathBuf::from("tmp/control.pid"),
                },
            ]
        );
    }

    #[test]
    fn issue_resource_telemetry_helper_validation_errors_are_reviewable() {
        let invalid_issue = parse_issue_number("not-a-number").expect_err("invalid issue");
        assert!(invalid_issue.to_string().contains("invalid issue number"));

        let invalid_stage =
            validate_capture_stage("Bad-Stage").expect_err("invalid capture stage should fail");
        assert!(invalid_stage.to_string().contains("snake_case"));

        let invalid_timestamp =
            parse_captured_at("not-a-timestamp").expect_err("invalid timestamp should fail");
        assert!(invalid_timestamp
            .to_string()
            .contains("invalid --captured-at"));

        let missing_process_value = require_value(&["--process".to_string()], &mut 0, "--process")
            .expect_err("missing process value should fail");
        assert!(missing_process_value
            .to_string()
            .contains("--process requires a non-empty value"));

        let invalid_process = parse_process_spec("Validation:not-a-pid")
            .expect_err("invalid process spec should fail");
        assert!(invalid_process
            .to_string()
            .contains("process role must be snake_case"));

        let invalid_pid_file = parse_pid_file_process_spec("control_plane:")
            .expect_err("empty pid file path should fail");
        assert!(invalid_pid_file
            .to_string()
            .contains("--pid-file-process path cannot be empty"));
    }

    #[test]
    fn issue_resource_telemetry_write_row_appends_existing_jsonl_content() {
        let repo_root = temp_repo_root("issue-resource-telemetry-append");
        let output = repo_root.join("telemetry/issue_resource_telemetry.v1.jsonl");
        let first = build_row(
            &TelemetryArgs {
                action: TelemetryAction::Collect,
                issue_number: 4298,
                issue_slug: "collector".to_string(),
                capture_stage: Some("issue_start".to_string()),
                host_label: APPROVED_LOCAL_HOST_LABEL.to_string(),
                repo_root: repo_root.clone(),
                out: Some(output.clone()),
                input: None,
                manifest_out: None,
                s3_prefix: None,
                repo: None,
                captured_at: Some("2026-06-20T09:30:00Z".to_string()),
                processes: vec![],
                upload: false,
                upload_manifest: false,
                redaction_status: "not_redacted_private_archive_manifest_only".to_string(),
                json_output: false,
            },
            &FakeCommandRunner::new(HashMap::new()),
        )
        .expect("first row");
        let second = build_row(
            &TelemetryArgs {
                action: TelemetryAction::Collect,
                issue_number: 4298,
                issue_slug: "collector".to_string(),
                capture_stage: Some("post_validation".to_string()),
                host_label: APPROVED_LOCAL_HOST_LABEL.to_string(),
                repo_root: repo_root.clone(),
                out: Some(output.clone()),
                input: None,
                manifest_out: None,
                s3_prefix: None,
                repo: None,
                captured_at: Some("2026-06-20T09:45:00Z".to_string()),
                processes: vec![],
                upload: false,
                upload_manifest: false,
                redaction_status: "not_redacted_private_archive_manifest_only".to_string(),
                json_output: false,
            },
            &FakeCommandRunner::new(HashMap::new()),
        )
        .expect("second row");

        write_row(&output, &first).expect("write first row");
        write_row(&output, &second).expect("append second row");

        let lines: Vec<_> = fs::read_to_string(&output)
            .expect("telemetry jsonl")
            .lines()
            .map(str::to_string)
            .collect();
        assert_eq!(lines.len(), 2);

        let first_value: serde_json::Value =
            serde_json::from_str(&lines[0]).expect("first json line");
        let second_value: serde_json::Value =
            serde_json::from_str(&lines[1]).expect("second json line");
        assert_eq!(first_value["capture_stage"], "issue_start");
        assert_eq!(second_value["capture_stage"], "post_validation");
    }

    #[test]
    fn issue_resource_telemetry_process_summary_keeps_valid_rows_when_one_pid_source_is_bad() {
        let repo_root = temp_repo_root("issue-resource-telemetry-process-mixed");
        let bad_pid_file = repo_root.join("bad.pid");
        fs::write(&bad_pid_file, "not-a-pid\n").expect("bad pid file");
        let summary = collect_process_summary(
            &fake_runner_with_supported_metrics(),
            &[
                TrackedProcessSpec::Pid {
                    role: "validation".to_string(),
                    pid: 4242,
                },
                TrackedProcessSpec::PidFile {
                    role: "control_plane".to_string(),
                    path: bad_pid_file.clone(),
                },
            ],
        )
        .expect("process summary");
        match summary {
            MetricField::Value(value) => {
                assert_eq!(value.tracked_process_count, 1);
                assert_eq!(value.heavy_processes[0].role, "validation");
            }
            MetricField::NotAvailable(_) => panic!("expected valid process summary"),
        }
        let _ = fs::remove_dir_all(repo_root);
    }

    #[test]
    fn issue_resource_telemetry_archive_builds_manifest_and_s3_refs() {
        let repo_root = temp_repo_root("issue-resource-telemetry-archive");
        let input = repo_root
            .join(".adl/runs/issues/issue-4299/telemetry/issue_resource_telemetry.v1.jsonl");
        fs::create_dir_all(input.parent().expect("parent")).expect("create input dir");
        fs::write(
            &input,
            "{\"schema_version\":\"adl.issue_resource_telemetry.v1\",\"issue_number\":4299}\n",
        )
        .expect("write input");
        let args = TelemetryArgs {
            action: TelemetryAction::Archive,
            issue_number: 4299,
            issue_slug: "archive".to_string(),
            capture_stage: None,
            host_label: "nessus.local".to_string(),
            repo_root: repo_root.clone(),
            out: None,
            input: Some(input.clone()),
            manifest_out: Some(default_manifest_path(&repo_root, 4299)),
            s3_prefix: Some("s3://adl-issue-telemetry/v0.91.6".to_string()),
            repo: Some("danielbaustin/agent-design-language".to_string()),
            captured_at: Some("2026-06-20T09:30:00Z".to_string()),
            processes: vec![],
            upload: false,
            upload_manifest: false,
            redaction_status: "not_redacted_private_archive_manifest_only".to_string(),
            json_output: false,
        };

        let manifest =
            build_archive_manifest(&args, &FakeArchiveUploader::succeed()).expect("manifest");
        assert_eq!(manifest.schema_version, ARCHIVE_MANIFEST_SCHEMA_VERSION);
        assert_eq!(manifest.host.label, "nessus");
        assert_eq!(manifest.host.classification, "operator_named_csm_host");
        assert_eq!(manifest.archive.upload_status, "upload_not_run");
        assert_eq!(
            manifest.archive.raw_telemetry_ref,
            "s3://adl-issue-telemetry/v0.91.6/danielbaustin-agent-design-language/issues/issue-4299/host-nessus/capture-2026-06-20T09-30-00Z/issue_resource_telemetry.v1.jsonl"
        );
        assert!(manifest
            .source
            .input_ref
            .contains(".adl/runs/issues/issue-4299/telemetry/issue_resource_telemetry.v1.jsonl"));
        let _ = fs::remove_dir_all(repo_root);
    }

    #[test]
    fn issue_resource_telemetry_archive_upload_manifest_persists_truthful_uploaded_status() {
        let repo_root = temp_repo_root("issue-resource-telemetry-archive-manifest-upload");
        let input = repo_root
            .join(".adl/runs/issues/issue-4299/telemetry/issue_resource_telemetry.v1.jsonl");
        fs::create_dir_all(input.parent().expect("parent")).expect("create input dir");
        fs::write(
            &input,
            "{\"schema_version\":\"adl.issue_resource_telemetry.v1\",\"issue_number\":4299}\n",
        )
        .expect("write input");
        let manifest_out = default_manifest_path(&repo_root, 4299);
        let args = TelemetryArgs {
            action: TelemetryAction::Archive,
            issue_number: 4299,
            issue_slug: "archive".to_string(),
            capture_stage: None,
            host_label: "wuji".to_string(),
            repo_root: repo_root.clone(),
            out: None,
            input: Some(input.clone()),
            manifest_out: Some(manifest_out.clone()),
            s3_prefix: Some("s3://adl-issue-telemetry/v0.91.6".to_string()),
            repo: Some("danielbaustin/agent-design-language".to_string()),
            captured_at: Some("2026-06-20T09:30:00Z".to_string()),
            processes: vec![],
            upload: true,
            upload_manifest: true,
            redaction_status: "not_redacted_private_archive_manifest_only".to_string(),
            json_output: false,
        };

        let uploader = FakeArchiveUploader::succeed();
        let manifest = build_archive_manifest(&args, &uploader).expect("manifest");
        assert_eq!(manifest.archive.upload_status, "uploaded");
        assert_eq!(
            manifest.archive.manifest_upload_status,
            "pending_manifest_write"
        );

        write_archive_manifest(&manifest_out, &manifest, &uploader).expect("write manifest");

        let uploads = uploader.uploads.lock().expect("lock uploads");
        assert_eq!(uploads.len(), 2);
        assert_eq!(
            uploads[0].dest,
            "s3://adl-issue-telemetry/v0.91.6/danielbaustin-agent-design-language/issues/issue-4299/host-wuji/capture-2026-06-20T09-30-00Z/issue_resource_telemetry.v1.jsonl"
        );
        assert_eq!(
            uploads[1].dest,
            "s3://adl-issue-telemetry/v0.91.6/danielbaustin-agent-design-language/issues/issue-4299/host-wuji/capture-2026-06-20T09-30-00Z/issue_resource_telemetry_manifest.v1.json"
        );
        assert!(uploads[1]
            .content
            .contains("\"manifest_upload_status\": \"uploaded\""));
        assert!(uploads[1].source.ends_with(Path::new(
            "issue_resource_telemetry_manifest.v1.upload.json"
        )));
        drop(uploads);

        let persisted = fs::read_to_string(&manifest_out).expect("persisted manifest");
        assert!(persisted.contains("\"manifest_upload_status\": \"uploaded\""));
        assert!(!manifest_out.with_extension("upload.json").exists());
        let _ = fs::remove_dir_all(repo_root);
    }

    #[test]
    fn issue_resource_telemetry_archive_fails_closed_on_upload_error() {
        let repo_root = temp_repo_root("issue-resource-telemetry-archive-upload");
        let input = repo_root.join("telemetry.jsonl");
        fs::write(
            &input,
            "{\"schema_version\":\"adl.issue_resource_telemetry.v1\",\"issue_number\":4299}\n",
        )
        .expect("write input");
        let args = TelemetryArgs {
            action: TelemetryAction::Archive,
            issue_number: 4299,
            issue_slug: "archive".to_string(),
            capture_stage: None,
            host_label: "wuji".to_string(),
            repo_root: repo_root.clone(),
            out: None,
            input: Some(input),
            manifest_out: Some(default_manifest_path(&repo_root, 4299)),
            s3_prefix: Some("s3://adl-issue-telemetry/v0.91.6".to_string()),
            repo: Some("danielbaustin/agent-design-language".to_string()),
            captured_at: Some("2026-06-20T09:30:00Z".to_string()),
            processes: vec![],
            upload: true,
            upload_manifest: false,
            redaction_status: "not_redacted_private_archive_manifest_only".to_string(),
            json_output: false,
        };
        let err = build_archive_manifest(&args, &FakeArchiveUploader::fail())
            .expect_err("upload failure must fail closed");
        assert!(err.to_string().contains("simulated upload failure"));
        let _ = fs::remove_dir_all(repo_root);
    }

    #[test]
    fn issue_resource_telemetry_archive_rejects_unknown_redaction_status() {
        let err = parse_args(&[
            "archive".to_string(),
            "--issue".to_string(),
            "4299".to_string(),
            "--issue-slug".to_string(),
            "archive".to_string(),
            "--input".to_string(),
            "telemetry.jsonl".to_string(),
            "--manifest-out".to_string(),
            "manifest.json".to_string(),
            "--s3-prefix".to_string(),
            "s3://adl-issue-telemetry/v0.91.6".to_string(),
            "--captured-at".to_string(),
            "2026-06-20T09:30:00Z".to_string(),
            "--redaction-status".to_string(),
            "mystery".to_string(),
        ])
        .expect_err("unknown redaction status must fail");
        assert!(err
            .to_string()
            .contains("unsupported --redaction-status 'mystery'"));
    }
}
