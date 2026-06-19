use anyhow::{anyhow, bail, Context, Result};
use chrono::{DateTime, Utc};
use serde::Serialize;
use serde_json::json;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, Clone, PartialEq, Eq)]
enum ArchiveAction {
    Summarize,
}

#[derive(Debug, Clone)]
struct ArchiveArgs {
    action: ArchiveAction,
    logs_dir: PathBuf,
    out: PathBuf,
    s3_prefix: String,
    repo: Option<String>,
    pr_number: Option<u64>,
    run_id: Option<String>,
    commit: Option<String>,
    raw_zip: Option<PathBuf>,
    upload: bool,
    threshold_seconds: f64,
    redaction_status: String,
}

#[derive(Debug, Clone, Serialize)]
struct TimingEntry {
    kind: String,
    name: String,
    duration_seconds: f64,
    lane_class: String,
    source_ref: String,
}

pub(super) fn real_ci_log_archive(args: &[String]) -> Result<()> {
    let args = parse_args(args)?;
    let manifest = run_archive(&args)?;
    if let Some(parent) = args.out.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create output dir '{}'", parent.display()))?;
    }
    fs::write(&args.out, serde_json::to_string_pretty(&manifest)? + "\n")
        .with_context(|| format!("failed to write manifest '{}'", args.out.display()))?;
    println!("ci_log_archive_manifest={}", args.out.display());
    Ok(())
}

fn parse_args(args: &[String]) -> Result<ArchiveArgs> {
    let Some(action) = args.first().map(String::as_str) else {
        bail!("{}", usage());
    };
    let action = match action {
        "summarize" => ArchiveAction::Summarize,
        "--help" | "-h" | "help" => {
            println!("{}", usage());
            return Err(anyhow!("help requested"));
        }
        other => bail!("unknown ci-log-archive action '{other}'\n{}", usage()),
    };

    let mut logs_dir = None;
    let mut out = None;
    let mut s3_prefix = None;
    let mut repo = None;
    let mut pr_number = None;
    let mut run_id = None;
    let mut commit = None;
    let mut raw_zip = None;
    let mut upload = false;
    let mut threshold_seconds = 60.0;
    let mut redaction_status = "not_redacted_private_archive_manifest_only".to_string();

    let mut i = 1usize;
    while i < args.len() {
        match args[i].as_str() {
            "--logs-dir" => {
                logs_dir = Some(PathBuf::from(require_value(args, &mut i, "--logs-dir")?))
            }
            "--out" => out = Some(PathBuf::from(require_value(args, &mut i, "--out")?)),
            "--s3-prefix" => s3_prefix = Some(require_value(args, &mut i, "--s3-prefix")?),
            "--repo" => repo = Some(require_value(args, &mut i, "--repo")?),
            "--pr" => {
                let value = require_value(args, &mut i, "--pr")?;
                pr_number = Some(
                    value
                        .parse::<u64>()
                        .with_context(|| format!("invalid --pr '{value}'"))?,
                );
            }
            "--run-id" => run_id = Some(require_value(args, &mut i, "--run-id")?),
            "--commit" => commit = Some(require_value(args, &mut i, "--commit")?),
            "--raw-zip" => raw_zip = Some(PathBuf::from(require_value(args, &mut i, "--raw-zip")?)),
            "--upload" => upload = true,
            "--threshold-seconds" => {
                let value = require_value(args, &mut i, "--threshold-seconds")?;
                threshold_seconds = value
                    .parse::<f64>()
                    .with_context(|| format!("invalid --threshold-seconds '{value}'"))?;
                if threshold_seconds <= 0.0 {
                    bail!("--threshold-seconds must be positive");
                }
            }
            "--redaction-status" => {
                redaction_status = require_value(args, &mut i, "--redaction-status")?;
                validate_redaction_status(&redaction_status)?;
            }
            other => bail!("unknown ci-log-archive argument '{other}'"),
        }
        i += 1;
    }

    let logs_dir =
        logs_dir.ok_or_else(|| anyhow!("ci-log-archive summarize requires --logs-dir"))?;
    let out = out.ok_or_else(|| anyhow!("ci-log-archive summarize requires --out"))?;
    let s3_prefix = s3_prefix.ok_or_else(|| {
        anyhow!("ci-log-archive summarize requires --s3-prefix s3://bucket/prefix")
    })?;
    if !s3_prefix.starts_with("s3://") {
        bail!("--s3-prefix must start with s3://");
    }
    if !logs_dir.is_dir() {
        bail!("--logs-dir is not a directory: {}", logs_dir.display());
    }
    if let Some(raw_zip) = raw_zip.as_ref() {
        if !raw_zip.is_file() {
            bail!("--raw-zip is not a file: {}", raw_zip.display());
        }
    }

    validate_redaction_status(&redaction_status)?;

    Ok(ArchiveArgs {
        action,
        logs_dir,
        out,
        s3_prefix,
        repo,
        pr_number,
        run_id,
        commit,
        raw_zip,
        upload,
        threshold_seconds,
        redaction_status,
    })
}

fn require_value(args: &[String], i: &mut usize, flag: &str) -> Result<String> {
    *i += 1;
    args.get(*i)
        .filter(|value| !value.trim().is_empty())
        .cloned()
        .ok_or_else(|| anyhow!("{flag} requires a non-empty value"))
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

fn usage() -> &'static str {
    "adl tooling ci-log-archive summarize --logs-dir <dir> --out <manifest.json> --s3-prefix s3://bucket/prefix [--repo owner/repo] [--pr <n>] [--run-id <id>] [--commit <sha>] [--raw-zip <logs.zip>] [--upload] [--threshold-seconds 60] [--redaction-status <status>]"
}

fn run_archive(args: &ArchiveArgs) -> Result<serde_json::Value> {
    match args.action {
        ArchiveAction::Summarize => {}
    }
    let mut entries = analyze_logs(&args.logs_dir, args.threshold_seconds)?;
    entries.sort_by(|a, b| {
        b.duration_seconds
            .partial_cmp(&a.duration_seconds)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| a.source_ref.cmp(&b.source_ref))
    });
    let over_threshold = entries
        .iter()
        .filter(|entry| entry.duration_seconds > args.threshold_seconds)
        .count();
    let raw_log_ref = raw_log_s3_ref(args);
    let upload_status = if args.upload {
        upload_raw_logs(args, &raw_log_ref)?
    } else {
        "upload_not_run".to_string()
    };

    Ok(json!({
        "schema_version": "adl.ci_log_archive_manifest.v1",
        "generated_at": Utc::now().to_rfc3339(),
        "source": {
            "repo": args.repo,
            "pr_number": args.pr_number,
            "run_id": args.run_id,
            "commit": args.commit
        },
        "archive": {
            "backend": "s3",
            "raw_log_ref": raw_log_ref,
            "upload_status": upload_status,
            "redaction_status": args.redaction_status,
            "local_retention": "temporary_input_only"
        },
        "timing_summary": {
            "threshold_seconds": args.threshold_seconds,
            "entry_count": entries.len(),
            "over_threshold_count": over_threshold,
            "a_small_count": entries.len().saturating_sub(over_threshold),
            "b_large_count": over_threshold
        },
        "timing_entries": entries,
        "consumption_policy": {
            "raw_logs_are_evidence": true,
            "manifest_and_summaries_are_memory": true,
            "obsmem_ingests_raw_logs": false,
            "obsmem_ingests_manifest": true,
            "validation_lane_index_consumes_timing_summary": true,
            "fail_closed_on_upload_or_redaction_error": true
        }
    }))
}

fn analyze_logs(logs_dir: &Path, threshold_seconds: f64) -> Result<Vec<TimingEntry>> {
    let mut files = Vec::new();
    collect_files(logs_dir, logs_dir, &mut files)?;
    let mut entries = Vec::new();
    for (path, rel) in files {
        let text = fs::read_to_string(&path)
            .with_context(|| format!("failed to read log '{}'", path.display()))?;
        if let Some(duration) = github_timestamp_duration_seconds(&text) {
            entries.push(TimingEntry {
                kind: "github_step_log".to_string(),
                name: rel.clone(),
                duration_seconds: round3(duration),
                lane_class: lane_class(duration, threshold_seconds),
                source_ref: rel.clone(),
            });
        }
        for (idx, duration) in rust_finished_durations(&text).into_iter().enumerate() {
            entries.push(TimingEntry {
                kind: "rust_test_summary".to_string(),
                name: format!("{rel}#finished_in_{}", idx + 1),
                duration_seconds: round3(duration),
                lane_class: lane_class(duration, threshold_seconds),
                source_ref: rel.clone(),
            });
        }
    }
    Ok(entries)
}

fn collect_files(root: &Path, dir: &Path, files: &mut Vec<(PathBuf, String)>) -> Result<()> {
    for entry in
        fs::read_dir(dir).with_context(|| format!("failed to read dir '{}'", dir.display()))?
    {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            collect_files(root, &path, files)?;
        } else if path.is_file() && is_log_file(&path) {
            let rel = path
                .strip_prefix(root)
                .unwrap_or(&path)
                .to_string_lossy()
                .replace('\\', "/");
            files.push((path, rel));
        }
    }
    Ok(())
}

fn is_log_file(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .is_some_and(|ext| matches!(ext, "txt" | "log" | "jsonl"))
}

fn github_timestamp_duration_seconds(text: &str) -> Option<f64> {
    let mut first: Option<DateTime<Utc>> = None;
    let mut last: Option<DateTime<Utc>> = None;
    for line in text.lines() {
        let Some(stamp) = line.get(..30).and_then(extract_rfc3339_prefix) else {
            continue;
        };
        first.get_or_insert(stamp);
        last = Some(stamp);
    }
    match (first, last) {
        (Some(first), Some(last)) if last >= first => {
            Some((last - first).num_milliseconds() as f64 / 1000.0)
        }
        _ => None,
    }
}

fn extract_rfc3339_prefix(line_prefix: &str) -> Option<DateTime<Utc>> {
    let end = line_prefix.find(' ')?;
    let candidate = &line_prefix[..end];
    DateTime::parse_from_rfc3339(candidate)
        .ok()
        .map(|dt| dt.with_timezone(&Utc))
}

fn rust_finished_durations(text: &str) -> Vec<f64> {
    let mut durations = Vec::new();
    for line in text.lines() {
        let Some(idx) = line.find("finished in ") else {
            continue;
        };
        let rest = &line[idx + "finished in ".len()..];
        if let Some(duration) = parse_duration_token(rest.split_whitespace().next().unwrap_or("")) {
            durations.push(duration);
        }
    }
    durations
}

fn parse_duration_token(token: &str) -> Option<f64> {
    if let Some(raw) = token.strip_suffix("ms") {
        raw.parse::<f64>().ok().map(|value| value / 1000.0)
    } else if let Some(raw) = token.strip_suffix('s') {
        raw.parse::<f64>().ok()
    } else if let Some(raw) = token.strip_suffix('m') {
        raw.parse::<f64>().ok().map(|value| value * 60.0)
    } else {
        None
    }
}

fn lane_class(duration: f64, threshold_seconds: f64) -> String {
    if duration > threshold_seconds {
        "B_large".to_string()
    } else {
        "A_small".to_string()
    }
}

fn round3(value: f64) -> f64 {
    (value * 1000.0).round() / 1000.0
}

fn raw_log_s3_ref(args: &ArchiveArgs) -> String {
    let mut prefix = args.s3_prefix.trim_end_matches('/').to_string();
    if let Some(repo) = args.repo.as_deref() {
        prefix.push('/');
        prefix.push_str(&sanitize_key_component(repo));
    }
    if let Some(pr) = args.pr_number {
        prefix.push_str(&format!("/pr-{pr}"));
    }
    if let Some(run_id) = args.run_id.as_deref() {
        prefix.push_str("/run-");
        prefix.push_str(&sanitize_key_component(run_id));
    }
    prefix.push_str("/github-actions-logs.zip");
    prefix
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

fn upload_raw_logs(args: &ArchiveArgs, raw_log_ref: &str) -> Result<String> {
    let Some(raw_zip) = args.raw_zip.as_ref() else {
        bail!("--upload requires --raw-zip so the uploaded payload is explicit");
    };
    let status = Command::new("aws")
        .args(["s3", "cp"])
        .arg(raw_zip)
        .arg(raw_log_ref)
        .status()
        .context("failed to invoke aws s3 cp for ci-log-archive upload")?;
    if !status.success() {
        bail!("aws s3 cp failed for ci-log-archive upload");
    }
    Ok("uploaded".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_dir(label: &str) -> PathBuf {
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock")
            .as_nanos();
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("repo root")
            .join(".tmp/ci_log_archive_tests")
            .join(format!("{label}-{stamp}"));
        fs::create_dir_all(&path).expect("temp dir");
        path
    }

    #[test]
    fn summarize_classifies_github_step_and_rust_finished_durations() {
        let root = temp_dir("summary");
        let logs = root.join("logs");
        fs::create_dir_all(&logs).expect("logs");
        fs::write(
            logs.join("slow-step.txt"),
            "2026-06-19T18:00:00.0000000Z start\n2026-06-19T18:02:10.0000000Z end\n",
        )
        .expect("slow log");
        fs::write(
            logs.join("rust.txt"),
            "test result: ok. 12 passed; finished in 0.05s\ntest result: ok. 1 passed; finished in 61.2s\n",
        )
        .expect("rust log");
        let out = root.join("manifest.json");
        let args = ArchiveArgs {
            action: ArchiveAction::Summarize,
            logs_dir: logs,
            out,
            s3_prefix: "s3://adl-ci-logs/v0.91.6".to_string(),
            repo: Some("danielbaustin/agent-design-language".to_string()),
            pr_number: Some(4152),
            run_id: Some("27840922589".to_string()),
            commit: Some("abc123".to_string()),
            raw_zip: None,
            upload: false,
            threshold_seconds: 60.0,
            redaction_status: "not_redacted_private_archive_manifest_only".to_string(),
        };
        let manifest = run_archive(&args).expect("manifest");
        assert_eq!(manifest["schema_version"], "adl.ci_log_archive_manifest.v1");
        assert_eq!(manifest["timing_summary"]["over_threshold_count"], 2);
        assert_eq!(manifest["timing_summary"]["a_small_count"], 1);
        assert_eq!(manifest["archive"]["upload_status"], "upload_not_run");
        assert_eq!(
            manifest["archive"]["raw_log_ref"],
            "s3://adl-ci-logs/v0.91.6/danielbaustin-agent-design-language/pr-4152/run-27840922589/github-actions-logs.zip"
        );
        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn rejects_unrecognized_redaction_status() {
        let args = vec![
            "summarize".to_string(),
            "--logs-dir".to_string(),
            ".".to_string(),
            "--out".to_string(),
            "manifest.json".to_string(),
            "--s3-prefix".to_string(),
            "s3://adl-ci-logs/v0.91.6".to_string(),
            "--redaction-status".to_string(),
            "public_safe_trust_me".to_string(),
        ];
        let err = parse_args(&args).expect_err("unknown redaction status must fail");
        assert!(err.to_string().contains("unsupported --redaction-status"));
    }

    #[test]
    fn upload_requires_explicit_raw_zip() {
        let root = temp_dir("upload-requires-zip");
        let logs = root.join("logs");
        fs::create_dir_all(&logs).expect("logs");
        let args = ArchiveArgs {
            action: ArchiveAction::Summarize,
            logs_dir: logs,
            out: root.join("manifest.json"),
            s3_prefix: "s3://adl-ci-logs/v0.91.6".to_string(),
            repo: None,
            pr_number: None,
            run_id: None,
            commit: None,
            raw_zip: None,
            upload: true,
            threshold_seconds: 60.0,
            redaction_status: "not_redacted_private_archive_manifest_only".to_string(),
        };
        let err = run_archive(&args).expect_err("upload without zip must fail");
        assert!(err.to_string().contains("--upload requires --raw-zip"));
        let _ = fs::remove_dir_all(root);
    }
}
