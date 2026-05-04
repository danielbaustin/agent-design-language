use anyhow::{Context, Result};
use serde_json::Value;
use std::time::Duration;

use super::code_review_args::{validate_git_ref, validate_include_file};
use super::code_review_helpers::truncate;
use super::code_review_types::*;

pub(crate) fn run_reviewer(
    args: &CodeReviewArgs,
    packet: &ReviewPacket,
    packet_id: &str,
) -> Result<ReviewResult> {
    match args.backend {
        ReviewerBackend::Fixture => Ok(fixture_review(args, packet, packet_id)),
        ReviewerBackend::Ollama => ollama_review(args, packet, packet_id),
    }
}

pub(crate) fn fixture_review(
    args: &CodeReviewArgs,
    packet: &ReviewPacket,
    packet_id: &str,
) -> ReviewResult {
    let reviewer_session = args
        .reviewer_session
        .clone()
        .unwrap_or_else(|| "fixture-reviewer-session".to_string());
    let same_session = reviewer_session == args.writer_session;
    let mut findings = Vec::new();
    let mut disposition = ReviewDisposition::NonProving;
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
        ReviewResultPartsCompat {
            reviewer_session,
            reviewer_model: args
                .model
                .clone()
                .unwrap_or_else(|| "fixture-reviewer-v1".to_string()),
            same_session,
            disposition,
            findings,
            residual_risk: vec![
                "fixture backend proves artifact shape only and cannot bless PR publication"
                    .to_string(),
            ],
        },
    )
}

pub(crate) struct ReviewResultPartsCompat {
    pub(crate) reviewer_session: String,
    pub(crate) reviewer_model: String,
    pub(crate) same_session: bool,
    pub(crate) disposition: ReviewDisposition,
    pub(crate) findings: Vec<ReviewFinding>,
    pub(crate) residual_risk: Vec<String>,
}

#[derive(Debug)]
pub(crate) struct ParsedModelReview {
    pub(crate) disposition: ReviewDisposition,
    pub(crate) findings: Vec<ReviewFinding>,
    pub(crate) residual_risk: Vec<String>,
}

pub(crate) fn skipped_review_result(
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
        ReviewResultPartsCompat {
            same_session: reviewer_session == args.writer_session,
            reviewer_session,
            reviewer_model: model,
            disposition: ReviewDisposition::Skipped,
            findings: Vec::new(),
            residual_risk: vec![residual_risk],
        },
    )
}

pub(crate) fn non_proving_review_result(
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
        ReviewResultPartsCompat {
            same_session: reviewer_session == args.writer_session,
            reviewer_session,
            reviewer_model: model,
            disposition: ReviewDisposition::NonProving,
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
    if packet.redaction_status.secret_like_values_present {
        return Ok(non_proving_review_result(
            args,
            packet,
            packet_id,
            reviewer_session.clone(),
            model,
            "live model invocation suppressed because the review packet contains secret-like values".to_string(),
        ));
    }

    let prompt = reviewer_prompt(packet);
    let endpoint = ollama_generate_url(&args.ollama_url)?;
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(args.timeout_secs))
        .build()
        .context("build Ollama HTTP client")?;
    let response = client
        .post(endpoint)
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({
            "model": model,
            "prompt": prompt,
            "stream": false,
            "format": "json",
            "options": {
                "temperature": 0,
                "num_predict": 4096
            }
        }))
        .send();
    let text = match response {
        Ok(resp) if resp.status().is_success() => resp
            .json::<Value>()
            .context("parse Ollama response JSON")?
            .as_object()
            .map(|object| {
                let response = object
                    .get("response")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .trim();
                if !response.is_empty() {
                    response.to_string()
                } else {
                    object
                        .get("thinking")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .trim()
                        .to_string()
                }
            })
            .unwrap_or_default(),
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
        Some(model_review) => normalize_model_review(model_review, packet),
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
        ReviewResultPartsCompat {
            same_session: reviewer_session == args.writer_session,
            reviewer_session,
            reviewer_model: model,
            disposition,
            findings,
            residual_risk: residual,
        },
    ))
}

pub(crate) fn review_result(
    args: &CodeReviewArgs,
    packet: &ReviewPacket,
    packet_id: &str,
    parts: ReviewResultPartsCompat,
) -> ReviewResult {
    ReviewResult {
        schema_version: CODE_REVIEW_RESULT_SCHEMA.to_string(),
        review_id: format!("review-{}", packet_id),
        reviewer_backend: args.backend.as_str().to_string(),
        reviewer_model: parts.reviewer_model,
        reviewer_session: parts.reviewer_session,
        writer_session: args.writer_session.clone(),
        same_session_as_writer: parts.same_session,
        visibility_mode: args.visibility_mode,
        repo_access: RepoAccess {
            read_only: false,
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
            "reviewer received a bounded packet, not live repository read tools".to_string(),
        ],
    }
}

pub(crate) fn evaluate_gate(result: &ReviewResult, packet: &ReviewPacket) -> GateResult {
    let mut reasons = Vec::new();
    if result.same_session_as_writer {
        reasons.push("reviewer session matches writer session".to_string());
    }
    if packet.redaction_status.absolute_host_paths_present {
        reasons.push(
            "review packet contained absolute host paths; live prompt redaction was required"
                .to_string(),
        );
    }
    if packet.redaction_status.secret_like_values_present {
        reasons.push("review packet contained secret-like values".to_string());
    }
    if packet
        .static_analysis_evidence
        .iter()
        .any(|item| item.status == "FAIL")
    {
        reasons.push("static analysis evidence contains failures".to_string());
    }
    if result.disposition != ReviewDisposition::NonProving {
        for finding in &result.findings {
            if finding.blocking && matches!(finding.priority.as_str(), "P0" | "P1" | "P2") {
                reasons.push(format!(
                    "blocking {} finding: {}",
                    finding.priority, finding.title
                ));
            }
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
        schema_version: CODE_REVIEW_GATE_SCHEMA,
        gate_disposition: gate_disposition.to_string(),
        pr_open_allowed,
        reasons,
    }
}

pub(crate) fn reviewer_prompt(packet: &ReviewPacket) -> String {
    let packet_json = redact_absolute_host_paths_for_prompt(
        &serde_json::to_string_pretty(packet).unwrap_or_default(),
    );
    format!(
        "/no_think\\nYou are an ADL code reviewer. Return final JSON immediately, with no chain-of-thought, analysis transcript, markdown, or prose. Do not call tools, do not emit action/action_input JSON, and do not request web search or repository commands. Be skeptical, concrete, and adversarial-but-fair. Your job is to find defects before PR publication, not to approve work. Review this bounded packet for correctness, fail-open gates, missing tests, path traversal, packet completeness, unsafe reviewer/session handling, timeout/truncation behavior, misleading docs, and lifecycle drift. Return only compact JSON matching schema_version adl.pr_review_result.v1 with fields disposition (blessed|blocked|non_proving), findings array, and residual_risk array. The top-level JSON object must not contain keys named action, action_input, tool, tool_call, function_call, or arguments. Include at most 5 findings. Keep each title under 12 words, each body under 90 words, and each evidence string under 120 characters. Findings are only actionable risks, bugs, regressions, missing tests, security issues, or misleading documentation. Do not put praise, confirmations, or already-correct behavior in findings. A security finding must cite the exact missing guard or a concrete bypass input; do not report a hypothetical bypass if the packet shows an existing guard that blocks it. Any blocking P0/P1 security finding must include at least one evidence string starting with 'bypass:' followed by the concrete malicious input or state transition. If you include a finding, every finding must include a specific title, priority (P0|P1|P2|P3), file, body, evidence array, heuristic_ids array, confidence, blocking, and suggested_fix_scope. Do not include placeholder, empty, or non-actionable findings. If you find no actionable issues, set disposition to blessed only after checking every included diff hunk, file_context, file-limit flag, and read_error. Residual_risk must list concrete checked invariants or limitations with file/function references. Do not use approval language such as approved, correct, sound, aligns, or looks fine as residual rationale. If the packet is truncated or has read errors enough that you cannot review it fairly, set disposition to non_proving. Packet:\\n{}",
        packet_json
    )
}

pub(crate) fn redact_absolute_host_paths_for_prompt(text: &str) -> String {
    redact_windows_absolute_paths(
        &text
            .replace("/Users/", "[REDACTED_HOST_PATH]/")
            .replace("/home/", "[REDACTED_HOST_PATH]/")
            .replace("/tmp/", "[REDACTED_HOST_PATH]/")
            .replace("/var/folders/", "[REDACTED_HOST_PATH]/"),
    )
}

fn redact_windows_absolute_paths(text: &str) -> String {
    let mut redacted = String::with_capacity(text.len());
    let mut chars = text.char_indices().peekable();
    while let Some((idx, ch)) = chars.next() {
        if ch.is_ascii_alphabetic()
            && is_windows_drive_boundary(text, idx)
            && chars.peek().map(|(_, next)| next) == Some(&':')
        {
            chars.next();
            if chars.peek().map(|(_, next)| next) == Some(&'\\') {
                redacted.push_str("[REDACTED_HOST_PATH]\\\\");
                chars.next();
                continue;
            }
            redacted.push(ch);
            redacted.push(':');
            continue;
        }
        redacted.push(ch);
    }
    redacted
}

pub(crate) fn parse_model_review_json(raw: &str) -> Option<ParsedModelReview> {
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
    let residual_risk = string_array(value.get("residual_risk"));
    Some(ParsedModelReview {
        disposition,
        findings,
        residual_risk,
    })
}

pub(crate) fn normalize_model_review(
    model_review: ParsedModelReview,
    packet: &ReviewPacket,
) -> (ReviewDisposition, Vec<ReviewFinding>, Vec<String>) {
    let disposition = model_review.disposition;
    let findings = model_review.findings;
    let mut malformed = Vec::new();
    for (idx, finding) in findings.iter().enumerate() {
        let label = format!("finding {}", idx + 1);
        if finding.title.trim().is_empty() || finding.title == "Untitled model finding" {
            malformed.push(format!("{label} is missing a specific title"));
        }
        if !matches!(finding.priority.as_str(), "P0" | "P1" | "P2" | "P3") {
            malformed.push(format!(
                "{label} has unsupported priority '{}'",
                finding.priority
            ));
        }
        if finding.file.trim().is_empty() || finding.file == "unknown" {
            malformed.push(format!("{label} is missing a specific file"));
        }
        if finding.body.trim().is_empty() {
            malformed.push(format!("{label} is missing an explanatory body"));
        }
        if finding.evidence.is_empty()
            || finding
                .evidence
                .iter()
                .any(|evidence| evidence.trim().is_empty())
        {
            malformed.push(format!("{label} is missing concrete evidence"));
        }
        if finding.confidence.trim().is_empty() {
            malformed.push(format!("{label} is missing confidence"));
        }
        if finding.suggested_fix_scope.trim().is_empty() {
            malformed.push(format!("{label} is missing suggested_fix_scope"));
        }
        if is_non_actionable_fix_scope(&finding.suggested_fix_scope) {
            malformed.push(format!(
                "{label} is non-actionable praise or confirmation, not a review finding"
            ));
        }
        if finding.blocking
            && matches!(finding.priority.as_str(), "P0" | "P1")
            && finding
                .heuristic_ids
                .iter()
                .any(|id| id.to_ascii_uppercase().starts_with("SEC"))
            && !finding
                .evidence
                .iter()
                .any(|evidence| evidence.trim_start().starts_with("bypass:"))
        {
            malformed.push(format!(
                "{label} is a blocking high-severity security finding without concrete bypass evidence"
            ));
        }
        if finding.evidence.iter().any(|evidence| {
            evidence
                .trim_start()
                .strip_prefix("bypass:")
                .and_then(classify_rejected_cli_bypass)
                .is_some()
        }) {
            malformed.push(format!(
                "{label} uses bypass evidence that is rejected by the current CLI validators"
            ));
        }
    }

    if disposition == ReviewDisposition::Blessed {
        if packet_is_incomplete(packet) {
            malformed.push(
                "model blessed an incomplete packet; review is not useful enough".to_string(),
            );
        }
        if model_review.residual_risk.is_empty() {
            malformed.push("model blessed with no residual-review rationale".to_string());
        }
        if model_review
            .residual_risk
            .iter()
            .any(|risk| is_praise_like_rationale(risk))
        {
            malformed.push(
                "model blessed with praise-like residual rationale instead of review evidence"
                    .to_string(),
            );
        }
    }

    if malformed.is_empty() {
        return (disposition, findings, model_review.residual_risk);
    }

    let mut residual_risk = vec!["model review output contained malformed findings".to_string()];
    residual_risk.extend(malformed);
    (ReviewDisposition::NonProving, findings, residual_risk)
}

fn packet_is_incomplete(packet: &ReviewPacket) -> bool {
    let truncated_context = packet.file_contexts.iter().any(|context| context.truncated);
    let missing_context = packet
        .file_contexts
        .iter()
        .any(|context| context.read_error.is_some());
    packet.diff_summary.truncated_hunks
        || packet.diff_summary.file_limit_truncated
        || truncated_context
        || missing_context
}

fn is_praise_like_rationale(raw: &str) -> bool {
    let normalized = raw.trim().to_ascii_lowercase();
    [
        "approved",
        "correct",
        "good",
        "sound",
        "aligns",
        "looks fine",
        "well implemented",
        "properly",
    ]
    .iter()
    .any(|needle| normalized.contains(needle))
}

fn is_non_actionable_fix_scope(raw: &str) -> bool {
    let normalized = raw.trim().to_ascii_lowercase();
    normalized == "none"
        || normalized == "n/a"
        || normalized == "not applicable"
        || normalized.starts_with("none.")
        || normalized.starts_with("no fix")
        || normalized.starts_with("no change")
        || normalized.starts_with("no action")
}

fn classify_rejected_cli_bypass(raw: &str) -> Option<&'static str> {
    let mut parts = raw.split_whitespace();
    while let Some(part) = parts.next() {
        match part {
            "--file" => {
                let value = parts.next()?;
                return validate_include_file(trim_shell_quotes(value))
                    .is_err()
                    .then_some("--file");
            }
            "--base" | "--head" => {
                let value = parts.next()?;
                return validate_git_ref(trim_shell_quotes(value), part)
                    .is_err()
                    .then_some("git-ref");
            }
            _ => {}
        }
    }
    None
}

fn trim_shell_quotes(raw: &str) -> &str {
    raw.trim_matches(|ch| matches!(ch, '\'' | '"' | '`'))
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

pub(crate) fn ollama_generate_url(raw: &str) -> Result<String> {
    let mut url = reqwest::Url::parse(raw).context("parse Ollama URL")?;
    let path = url.path().trim_end_matches('/');
    if !path.ends_with("/api/generate") {
        url.set_path("/api/generate");
    }
    Ok(url.to_string())
}

fn is_windows_drive_boundary(text: &str, idx: usize) -> bool {
    idx == 0
        || !text[..idx]
            .chars()
            .next_back()
            .is_some_and(|ch| ch.is_ascii_alphanumeric() || ch == '_')
}
