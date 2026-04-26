use anyhow::{anyhow, bail, ensure, Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::Digest;
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;

use super::common::{contains_absolute_host_path_in_text, repo_root};
use super::tooling_usage;

const PACKET_SCHEMA: &str = "adl.pr_review_packet.v1";
const RESULT_SCHEMA: &str = "adl.pr_review_result.v1";
const GATE_SCHEMA: &str = "adl.pr_review_gate.v1";
const SUMMARY_SCHEMA: &str = "adl.pr_review_run_summary.v1";

#[derive(Debug)]
struct CodeReviewArgs {
    out: PathBuf,
    backend: ReviewerBackend,
    visibility_mode: VisibilityMode,
    base_ref: String,
    head_ref: String,
    issue_number: Option<u32>,
    writer_session: String,
    reviewer_session: Option<String>,
    model: Option<String>,
    allow_live_ollama: bool,
    ollama_url: String,
    fixture_case: FixtureCase,
    max_diff_bytes: usize,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
enum ReviewerBackend {
    Fixture,
    Ollama,
}

impl ReviewerBackend {
    fn as_str(self) -> &'static str {
        match self {
            Self::Fixture => "fixture",
            Self::Ollama => "ollama",
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
enum VisibilityMode {
    PacketOnly,
    ReadOnlyRepo,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FixtureCase {
    Clean,
    Blocked,
}

#[derive(Debug, Serialize)]
struct ReviewPacket {
    schema_version: &'static str,
    issue_number: Option<u32>,
    branch: String,
    base_ref: String,
    head_ref: String,
    visibility_mode: VisibilityMode,
    changed_files: Vec<String>,
    diff_summary: DiffSummary,
    focused_diff_hunks: Vec<DiffHunk>,
    validation_evidence: Vec<ValidationEvidence>,
    static_analysis_evidence: Vec<ValidationEvidence>,
    repo_slice_manifest: RepoSliceManifest,
    review_scope: String,
    non_scope: Vec<String>,
    known_risks: Vec<String>,
    redaction_status: RedactionStatus,
}

#[derive(Debug, Serialize)]
struct DiffSummary {
    files_changed: usize,
    max_diff_bytes: usize,
    truncated_hunks: bool,
}

#[derive(Debug, Serialize)]
struct DiffHunk {
    file: String,
    diff_excerpt: String,
    truncated: bool,
}

#[derive(Debug, Serialize)]
struct ValidationEvidence {
    command: String,
    status: String,
    summary: String,
}

#[derive(Debug, Serialize)]
struct RepoSliceManifest {
    read_only: bool,
    write_allowed: bool,
    tool_execution_allowed: bool,
    files: Vec<String>,
}

#[derive(Debug, Serialize)]
struct RedactionStatus {
    absolute_host_paths_present: bool,
    secret_like_values_present: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct ReviewResult {
    schema_version: String,
    review_id: String,
    reviewer_backend: String,
    reviewer_model: String,
    reviewer_session: String,
    writer_session: String,
    same_session_as_writer: bool,
    visibility_mode: VisibilityMode,
    repo_access: RepoAccess,
    packet_id: String,
    static_analysis_summary: Vec<String>,
    findings: Vec<ReviewFinding>,
    disposition: ReviewDisposition,
    residual_risk: Vec<String>,
    validation_claims: Vec<String>,
    non_claims: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct RepoAccess {
    read_only: bool,
    write_allowed: bool,
    tool_execution_allowed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct ReviewFinding {
    title: String,
    priority: String,
    file: String,
    line: Option<u32>,
    body: String,
    evidence: Vec<String>,
    heuristic_ids: Vec<String>,
    confidence: String,
    blocking: bool,
    suggested_fix_scope: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
enum ReviewDisposition {
    Blessed,
    Blocked,
    NonProving,
    Skipped,
}

#[derive(Debug, Serialize)]
struct GateResult {
    schema_version: &'static str,
    gate_disposition: String,
    pr_open_allowed: bool,
    reasons: Vec<String>,
}

#[derive(Debug, Serialize)]
struct RunSummary {
    schema_version: &'static str,
    packet_path: String,
    result_path: String,
    gate_path: String,
    backend: String,
    visibility_mode: VisibilityMode,
    pr_open_allowed: bool,
}

pub(super) fn real_code_review(args: &[String]) -> Result<()> {
    if args
        .iter()
        .any(|arg| matches!(arg.as_str(), "help" | "--help" | "-h"))
    {
        println!("{}", tooling_usage());
        return Ok(());
    }

    let args = parse_args(args)?;
    fs::create_dir_all(&args.out).context("create code-review output directory")?;

    let root = repo_root()?;
    let packet = build_packet(&root, &args)?;
    let packet_id = packet_id(&packet);
    let result = run_reviewer(&args, &packet, &packet_id)?;
    let gate = evaluate_gate(&result, &packet);

    let packet_path = args.out.join("review_packet.json");
    let result_path = args.out.join("review_result.json");
    let gate_path = args.out.join("gate_result.json");
    let summary_path = args.out.join("run_summary.json");

    write_json(&packet_path, &packet)?;
    write_json(&result_path, &result)?;
    write_json(&gate_path, &gate)?;
    write_json(
        &summary_path,
        &RunSummary {
            schema_version: SUMMARY_SCHEMA,
            packet_path: "review_packet.json".to_string(),
            result_path: "review_result.json".to_string(),
            gate_path: "gate_result.json".to_string(),
            backend: args.backend.as_str().to_string(),
            visibility_mode: args.visibility_mode,
            pr_open_allowed: gate.pr_open_allowed,
        },
    )?;

    println!(
        "CODE_REVIEW_GATE={} OUT={}",
        gate.gate_disposition,
        args.out.display()
    );
    Ok(())
}

fn parse_args(args: &[String]) -> Result<CodeReviewArgs> {
    let mut parsed = CodeReviewArgs {
        out: PathBuf::new(),
        backend: ReviewerBackend::Fixture,
        visibility_mode: VisibilityMode::PacketOnly,
        base_ref: "origin/main".to_string(),
        head_ref: "HEAD".to_string(),
        issue_number: None,
        writer_session: std::env::var("CODEX_SESSION_ID")
            .unwrap_or_else(|_| "unknown-writer-session".to_string()),
        reviewer_session: None,
        model: None,
        allow_live_ollama: false,
        ollama_url: std::env::var("OLLAMA_HOST")
            .unwrap_or_else(|_| "http://127.0.0.1:11434".to_string()),
        fixture_case: FixtureCase::Clean,
        max_diff_bytes: 12_000,
    };

    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--out" => {
                parsed.out = PathBuf::from(value_arg(args, i, "--out")?);
                i += 1;
            }
            "--backend" => {
                parsed.backend = parse_backend(value_arg(args, i, "--backend")?)?;
                i += 1;
            }
            "--visibility" => {
                parsed.visibility_mode = parse_visibility(value_arg(args, i, "--visibility")?)?;
                i += 1;
            }
            "--base" => {
                parsed.base_ref = value_arg(args, i, "--base")?.to_string();
                i += 1;
            }
            "--head" => {
                parsed.head_ref = value_arg(args, i, "--head")?.to_string();
                i += 1;
            }
            "--issue" => {
                parsed.issue_number = Some(
                    value_arg(args, i, "--issue")?
                        .parse()
                        .map_err(|_| anyhow!("invalid --issue value"))?,
                );
                i += 1;
            }
            "--writer-session" => {
                parsed.writer_session = value_arg(args, i, "--writer-session")?.to_string();
                i += 1;
            }
            "--reviewer-session" => {
                parsed.reviewer_session =
                    Some(value_arg(args, i, "--reviewer-session")?.to_string());
                i += 1;
            }
            "--model" => {
                parsed.model = Some(value_arg(args, i, "--model")?.to_string());
                i += 1;
            }
            "--allow-live-ollama" => parsed.allow_live_ollama = true,
            "--ollama-url" => {
                parsed.ollama_url = value_arg(args, i, "--ollama-url")?.to_string();
                i += 1;
            }
            "--fixture-case" => {
                parsed.fixture_case = parse_fixture_case(value_arg(args, i, "--fixture-case")?)?;
                i += 1;
            }
            "--max-diff-bytes" => {
                parsed.max_diff_bytes = value_arg(args, i, "--max-diff-bytes")?
                    .parse()
                    .map_err(|_| anyhow!("invalid --max-diff-bytes value"))?;
                i += 1;
            }
            other => bail!("unknown arg for tooling code-review: {other}"),
        }
        i += 1;
    }

    ensure!(!parsed.out.as_os_str().is_empty(), "missing --out <dir>");
    ensure!(
        !parsed.writer_session.trim().is_empty(),
        "--writer-session must not be empty"
    );
    ensure!(
        parsed.max_diff_bytes >= 256,
        "--max-diff-bytes must be at least 256"
    );
    Ok(parsed)
}

fn value_arg<'a>(args: &'a [String], index: usize, flag: &str) -> Result<&'a str> {
    args.get(index + 1)
        .map(|value| value.as_str())
        .ok_or_else(|| anyhow!("missing value for {flag}"))
}

fn parse_backend(raw: &str) -> Result<ReviewerBackend> {
    match raw {
        "fixture" => Ok(ReviewerBackend::Fixture),
        "ollama" | "gemma4_local" => Ok(ReviewerBackend::Ollama),
        other => bail!("unsupported code-review backend '{other}'"),
    }
}

fn parse_visibility(raw: &str) -> Result<VisibilityMode> {
    match raw {
        "packet-only" | "packet_only" => Ok(VisibilityMode::PacketOnly),
        "read-only-repo" | "read_only_repo" => Ok(VisibilityMode::ReadOnlyRepo),
        other => bail!("unsupported visibility mode '{other}'"),
    }
}

fn parse_fixture_case(raw: &str) -> Result<FixtureCase> {
    match raw {
        "clean" => Ok(FixtureCase::Clean),
        "blocked" => Ok(FixtureCase::Blocked),
        other => bail!("unsupported fixture case '{other}'"),
    }
}

fn build_packet(root: &Path, args: &CodeReviewArgs) -> Result<ReviewPacket> {
    let branch = git_output(root, &["rev-parse", "--abbrev-ref", "HEAD"])
        .unwrap_or_else(|_| "unknown".to_string());
    let changed_files = changed_files(root, &args.base_ref, &args.head_ref)?;
    let focused_diff_hunks = diff_hunks(root, args, &changed_files)?;
    let truncated_hunks = focused_diff_hunks.iter().any(|hunk| hunk.truncated);
    let static_analysis_evidence = static_evidence(root, args);
    let packet_text = serde_json::to_string(&focused_diff_hunks).unwrap_or_default();
    let redaction_status = RedactionStatus {
        absolute_host_paths_present: contains_absolute_host_path_in_text(&packet_text),
        secret_like_values_present: false,
    };
    Ok(ReviewPacket {
        schema_version: PACKET_SCHEMA,
        issue_number: args.issue_number,
        branch,
        base_ref: args.base_ref.clone(),
        head_ref: args.head_ref.clone(),
        visibility_mode: args.visibility_mode,
        changed_files: changed_files.clone(),
        diff_summary: DiffSummary {
            files_changed: changed_files.len(),
            max_diff_bytes: args.max_diff_bytes,
            truncated_hunks,
        },
        focused_diff_hunks,
        validation_evidence: Vec::new(),
        static_analysis_evidence,
        repo_slice_manifest: RepoSliceManifest {
            read_only: args.visibility_mode == VisibilityMode::ReadOnlyRepo,
            write_allowed: false,
            tool_execution_allowed: false,
            files: changed_files,
        },
        review_scope: "review changed files, diff hunks, static evidence, and issue/card context for correctness, safety, tests, docs, and ADL lifecycle risks".to_string(),
        non_scope: vec![
            "do not edit files".to_string(),
            "do not claim merge authority".to_string(),
            "do not claim validation that is not present in the packet".to_string(),
        ],
        known_risks: vec![
            "packet-only mode may miss surrounding-code context not included in focused hunks".to_string(),
        ],
        redaction_status,
    })
}

fn changed_files(root: &Path, base: &str, head: &str) -> Result<Vec<String>> {
    let mut files = BTreeSet::new();
    if let Ok(output) = git_output(root, &["diff", "--name-only", &format!("{base}...{head}")]) {
        files.extend(
            output
                .lines()
                .map(str::trim)
                .filter(|line| !line.is_empty())
                .map(str::to_string),
        );
    }
    if let Ok(output) = git_output(root, &["diff", "--name-only"]) {
        files.extend(
            output
                .lines()
                .map(str::trim)
                .filter(|line| !line.is_empty())
                .map(str::to_string),
        );
    }
    Ok(files.into_iter().collect())
}

fn diff_hunks(root: &Path, args: &CodeReviewArgs, files: &[String]) -> Result<Vec<DiffHunk>> {
    let mut hunks = Vec::new();
    for file in files.iter().take(40) {
        let diff = git_output(root, &["diff", "--", file])
            .ok()
            .filter(|text| !text.trim().is_empty())
            .or_else(|| {
                git_output(
                    root,
                    &[
                        "diff",
                        &format!("{}...{}", args.base_ref, args.head_ref),
                        "--",
                        file,
                    ],
                )
                .ok()
            })
            .unwrap_or_default();
        let (diff_excerpt, truncated) = truncate(&diff, args.max_diff_bytes);
        hunks.push(DiffHunk {
            file: file.clone(),
            diff_excerpt,
            truncated,
        });
    }
    Ok(hunks)
}

fn static_evidence(root: &Path, args: &CodeReviewArgs) -> Vec<ValidationEvidence> {
    let mut evidence = Vec::new();
    evidence.push(command_evidence(
        root,
        &["diff", "--check"],
        "working tree whitespace check",
    ));
    evidence.push(command_evidence(
        root,
        &[
            "diff",
            "--check",
            &format!("{}...{}", args.base_ref, args.head_ref),
        ],
        "committed diff whitespace check",
    ));
    evidence
}

fn command_evidence(root: &Path, git_args: &[&str], label: &str) -> ValidationEvidence {
    let output = Command::new("git")
        .args(git_args)
        .current_dir(root)
        .output();
    match output {
        Ok(output) if output.status.success() => ValidationEvidence {
            command: format!("git {}", git_args.join(" ")),
            status: "PASS".to_string(),
            summary: label.to_string(),
        },
        Ok(output) => ValidationEvidence {
            command: format!("git {}", git_args.join(" ")),
            status: "FAIL".to_string(),
            summary: String::from_utf8_lossy(&output.stderr).trim().to_string(),
        },
        Err(err) => ValidationEvidence {
            command: format!("git {}", git_args.join(" ")),
            status: "SKIPPED".to_string(),
            summary: format!("{label}: {err}"),
        },
    }
}

fn run_reviewer(
    args: &CodeReviewArgs,
    packet: &ReviewPacket,
    packet_id: &str,
) -> Result<ReviewResult> {
    match args.backend {
        ReviewerBackend::Fixture => Ok(fixture_review(args, packet, packet_id)),
        ReviewerBackend::Ollama => ollama_review(args, packet, packet_id),
    }
}

fn fixture_review(args: &CodeReviewArgs, packet: &ReviewPacket, packet_id: &str) -> ReviewResult {
    let reviewer_session = args
        .reviewer_session
        .clone()
        .unwrap_or_else(|| "fixture-reviewer-session".to_string());
    let same_session = reviewer_session == args.writer_session;
    let mut findings = Vec::new();
    let mut disposition = ReviewDisposition::Blessed;
    if args.fixture_case == FixtureCase::Blocked {
        disposition = ReviewDisposition::Blocked;
        findings.push(ReviewFinding {
            title: "Fixture blocking finding".to_string(),
            priority: "P2".to_string(),
            file: packet
                .changed_files
                .first()
                .cloned()
                .unwrap_or_else(|| "unknown".to_string()),
            line: Some(1),
            body: "Fixture backend intentionally produced a blocking finding for gate testing."
                .to_string(),
            evidence: vec!["fixture:blocked-case".to_string()],
            heuristic_ids: vec!["C1".to_string(), "T7".to_string()],
            confidence: "high".to_string(),
            blocking: true,
            suggested_fix_scope: "fixture_only".to_string(),
        });
    }
    if same_session {
        disposition = ReviewDisposition::NonProving;
    }
    review_result(
        args,
        packet,
        packet_id,
        ReviewResultParts {
            reviewer_session,
            reviewer_model: args
                .model
                .clone()
                .unwrap_or_else(|| "fixture-reviewer-v1".to_string()),
            same_session,
            disposition,
            findings,
            residual_risk: vec![
                "fixture backend does not perform semantic model review".to_string()
            ],
        },
    )
}

struct ReviewResultParts {
    reviewer_session: String,
    reviewer_model: String,
    same_session: bool,
    disposition: ReviewDisposition,
    findings: Vec<ReviewFinding>,
    residual_risk: Vec<String>,
}

fn skipped_review_result(
    args: &CodeReviewArgs,
    packet: &ReviewPacket,
    packet_id: &str,
    reviewer_session: String,
    model: String,
    residual_risk: String,
) -> ReviewResult {
    review_result(
        args,
        packet,
        packet_id,
        ReviewResultParts {
            same_session: reviewer_session == args.writer_session,
            reviewer_session,
            reviewer_model: model,
            disposition: ReviewDisposition::Skipped,
            findings: Vec::new(),
            residual_risk: vec![residual_risk],
        },
    )
}

fn ollama_review(
    args: &CodeReviewArgs,
    packet: &ReviewPacket,
    packet_id: &str,
) -> Result<ReviewResult> {
    let reviewer_session = args
        .reviewer_session
        .clone()
        .unwrap_or_else(|| "ollama-reviewer-session".to_string());
    let model = args
        .model
        .clone()
        .unwrap_or_else(|| "gemma4:latest".to_string());
    if !args.allow_live_ollama {
        return Ok(skipped_review_result(
            args,
            packet,
            packet_id,
            reviewer_session.clone(),
            model,
            "live Ollama review requires --allow-live-ollama".to_string(),
        ));
    }

    let prompt = reviewer_prompt(packet);
    let endpoint = ollama_generate_url(&args.ollama_url)?;
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(120))
        .build()
        .context("build Ollama HTTP client")?;
    let response = client
        .post(endpoint)
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({
            "model": model,
            "prompt": prompt,
            "stream": false
        }))
        .send();
    let text = match response {
        Ok(resp) if resp.status().is_success() => resp
            .json::<Value>()
            .context("parse Ollama response JSON")?
            .get("response")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .trim()
            .to_string(),
        Ok(resp) => {
            return Ok(skipped_review_result(
                args,
                packet,
                packet_id,
                reviewer_session.clone(),
                model,
                format!("Ollama returned HTTP {}", resp.status()),
            ));
        }
        Err(err) => {
            return Ok(skipped_review_result(
                args,
                packet,
                packet_id,
                reviewer_session.clone(),
                model,
                format!("Ollama unavailable: {err}"),
            ));
        }
    };

    let parsed = parse_model_review_json(&text);
    let (disposition, findings, residual) = match parsed {
        Some((disposition, findings)) => (disposition, findings, Vec::new()),
        None => (
            ReviewDisposition::NonProving,
            Vec::new(),
            vec![
                "Ollama response was not valid normalized review JSON".to_string(),
                truncate(&text, 800).0,
            ],
        ),
    };
    Ok(review_result(
        args,
        packet,
        packet_id,
        ReviewResultParts {
            same_session: reviewer_session == args.writer_session,
            reviewer_session,
            reviewer_model: model,
            disposition,
            findings,
            residual_risk: residual,
        },
    ))
}

fn review_result(
    args: &CodeReviewArgs,
    packet: &ReviewPacket,
    packet_id: &str,
    parts: ReviewResultParts,
) -> ReviewResult {
    ReviewResult {
        schema_version: RESULT_SCHEMA.to_string(),
        review_id: format!("review-{}", packet_id),
        reviewer_backend: args.backend.as_str().to_string(),
        reviewer_model: parts.reviewer_model,
        reviewer_session: parts.reviewer_session,
        writer_session: args.writer_session.clone(),
        same_session_as_writer: parts.same_session,
        visibility_mode: args.visibility_mode,
        repo_access: RepoAccess {
            read_only: args.visibility_mode == VisibilityMode::ReadOnlyRepo,
            write_allowed: false,
            tool_execution_allowed: false,
        },
        packet_id: packet_id.to_string(),
        static_analysis_summary: packet
            .static_analysis_evidence
            .iter()
            .map(|item| format!("{}: {}", item.status, item.command))
            .collect(),
        findings: parts.findings,
        disposition: parts.disposition,
        residual_risk: parts.residual_risk,
        validation_claims: Vec::new(),
        non_claims: vec![
            "review result is not merge authority".to_string(),
            "reviewer did not execute repository writes".to_string(),
        ],
    }
}

fn evaluate_gate(result: &ReviewResult, packet: &ReviewPacket) -> GateResult {
    let mut reasons = Vec::new();
    if result.same_session_as_writer {
        reasons.push("reviewer session matches writer session".to_string());
    }
    if packet
        .static_analysis_evidence
        .iter()
        .any(|item| item.status == "FAIL")
    {
        reasons.push("static analysis evidence contains failures".to_string());
    }
    for finding in &result.findings {
        if finding.blocking && matches!(finding.priority.as_str(), "P0" | "P1" | "P2") {
            reasons.push(format!(
                "blocking {} finding: {}",
                finding.priority, finding.title
            ));
        }
    }
    if result.disposition == ReviewDisposition::Blocked {
        reasons.push("review disposition is blocked".to_string());
    }
    if result.disposition == ReviewDisposition::Skipped {
        reasons
            .push("review disposition is skipped; operator waiver is not implemented".to_string());
    }
    if result.disposition == ReviewDisposition::NonProving {
        reasons.push(
            "review disposition is non_proving; operator waiver is not implemented".to_string(),
        );
    }
    let pr_open_allowed = reasons.is_empty() && result.disposition == ReviewDisposition::Blessed;
    let gate_disposition = if pr_open_allowed {
        "allow_with_evidence"
    } else {
        "block_pr_open"
    };
    GateResult {
        schema_version: GATE_SCHEMA,
        gate_disposition: gate_disposition.to_string(),
        pr_open_allowed,
        reasons,
    }
}

fn reviewer_prompt(packet: &ReviewPacket) -> String {
    format!(
        "You are an ADL code reviewer. Review this bounded packet fairly and meticulously. Return only JSON matching schema_version adl.pr_review_result.v1 with fields disposition (blessed|blocked|non_proving), findings array, and residual_risk array. Packet:\n{}",
        serde_json::to_string_pretty(packet).unwrap_or_default()
    )
}

fn parse_model_review_json(raw: &str) -> Option<(ReviewDisposition, Vec<ReviewFinding>)> {
    let cleaned = raw
        .trim()
        .trim_start_matches("```json")
        .trim_start_matches("```")
        .trim_end_matches("```")
        .trim();
    let value: Value = serde_json::from_str(cleaned).ok()?;
    let disposition = match value.get("disposition")?.as_str()? {
        "blessed" => ReviewDisposition::Blessed,
        "blocked" => ReviewDisposition::Blocked,
        "non_proving" => ReviewDisposition::NonProving,
        "skipped" => ReviewDisposition::Skipped,
        _ => return None,
    };
    let findings = value
        .get("findings")
        .and_then(|v| v.as_array())
        .map(|items| {
            items
                .iter()
                .map(|item| ReviewFinding {
                    title: item
                        .get("title")
                        .and_then(|v| v.as_str())
                        .unwrap_or("Untitled model finding")
                        .to_string(),
                    priority: item
                        .get("priority")
                        .and_then(|v| v.as_str())
                        .unwrap_or("P3")
                        .to_string(),
                    file: item
                        .get("file")
                        .and_then(|v| v.as_str())
                        .unwrap_or("unknown")
                        .to_string(),
                    line: item.get("line").and_then(|v| v.as_u64()).map(|v| v as u32),
                    body: item
                        .get("body")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string(),
                    evidence: string_array(item.get("evidence")),
                    heuristic_ids: string_array(item.get("heuristic_ids")),
                    confidence: item
                        .get("confidence")
                        .and_then(|v| v.as_str())
                        .unwrap_or("medium")
                        .to_string(),
                    blocking: item
                        .get("blocking")
                        .and_then(|v| v.as_bool())
                        .unwrap_or(false),
                    suggested_fix_scope: item
                        .get("suggested_fix_scope")
                        .and_then(|v| v.as_str())
                        .unwrap_or("issue_local")
                        .to_string(),
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    Some((disposition, findings))
}

fn string_array(value: Option<&Value>) -> Vec<String> {
    value
        .and_then(|v| v.as_array())
        .map(|items| {
            items
                .iter()
                .filter_map(|item| item.as_str().map(ToString::to_string))
                .collect()
        })
        .unwrap_or_default()
}

fn ollama_generate_url(raw: &str) -> Result<String> {
    let mut url = reqwest::Url::parse(raw).context("parse Ollama URL")?;
    let path = url.path().trim_end_matches('/');
    if !path.ends_with("/api/generate") {
        url.set_path("/api/generate");
    }
    Ok(url.to_string())
}

fn packet_id(packet: &ReviewPacket) -> String {
    let seed = format!(
        "{}:{}:{}:{}",
        packet.issue_number.unwrap_or_default(),
        packet.branch,
        packet.changed_files.len(),
        packet.diff_summary.truncated_hunks
    );
    let digest = sha2::Sha256::digest(seed.as_bytes());
    format!("{:x}", digest)[..16].to_string()
}

fn truncate(text: &str, max_bytes: usize) -> (String, bool) {
    if text.len() <= max_bytes {
        return (text.to_string(), false);
    }
    let end = text
        .char_indices()
        .map(|(idx, _)| idx)
        .chain(std::iter::once(text.len()))
        .take_while(|idx| *idx <= max_bytes)
        .last()
        .unwrap_or(0);
    (text[..end].to_string(), true)
}

fn git_output(root: &Path, args: &[&str]) -> Result<String> {
    let output = Command::new("git").args(args).current_dir(root).output()?;
    if !output.status.success() {
        bail!("git command failed: git {}", args.join(" "));
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn write_json(path: &Path, value: &impl Serialize) -> Result<()> {
    let bytes = serde_json::to_vec_pretty(value)?;
    fs::write(path, bytes)?;
    Ok(())
}
