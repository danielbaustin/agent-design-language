use anyhow::{anyhow, bail, ensure, Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::Digest;
use std::collections::BTreeSet;
use std::fs;
use std::io::Read;
use std::path::{Component, Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::Duration;

use super::common::{contains_secret_like_token, repo_root};
use super::tooling_usage;

const PACKET_SCHEMA: &str = "adl.pr_review_packet.v1";
const RESULT_SCHEMA: &str = "adl.pr_review_result.v1";
const GATE_SCHEMA: &str = "adl.pr_review_gate.v1";
const SUMMARY_SCHEMA: &str = "adl.pr_review_run_summary.v1";
const DEFAULT_REVIEW_EXCERPT_BYTES: usize = 12_000;
const MAX_REVIEW_EXCERPT_BYTES: usize = 100_000;
const MAX_REVIEW_DIFF_FILES: usize = 40;
const MAX_REVIEW_CONTEXT_FILES: usize = 24;

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
    timeout_secs: u64,
    include_working_tree: bool,
    fixture_case: FixtureCase,
    max_diff_bytes: usize,
    include_files: Vec<String>,
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
    file_contexts: Vec<FileContext>,
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
    max_diff_files: usize,
    max_context_files: usize,
    file_limit_truncated: bool,
    truncated_hunks: bool,
}

#[derive(Debug, Serialize)]
struct DiffHunk {
    file: String,
    diff_excerpt: String,
    truncated: bool,
}

#[derive(Debug, Serialize)]
struct FileContext {
    file: String,
    current_excerpt: String,
    truncated: bool,
    read_error: Option<String>,
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
        timeout_secs: 120,
        include_working_tree: false,
        fixture_case: FixtureCase::Clean,
        max_diff_bytes: DEFAULT_REVIEW_EXCERPT_BYTES,
        include_files: Vec::new(),
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
                parsed.base_ref = validate_git_ref(value_arg(args, i, "--base")?, "--base")?;
                i += 1;
            }
            "--head" => {
                parsed.head_ref = validate_git_ref(value_arg(args, i, "--head")?, "--head")?;
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
            "--include-working-tree" => parsed.include_working_tree = true,
            "--ollama-url" => {
                parsed.ollama_url = value_arg(args, i, "--ollama-url")?.to_string();
                i += 1;
            }
            "--timeout-secs" => {
                parsed.timeout_secs = value_arg(args, i, "--timeout-secs")?
                    .parse()
                    .map_err(|_| anyhow!("invalid --timeout-secs value"))?;
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
            "--file" => {
                parsed
                    .include_files
                    .push(validate_include_file(value_arg(args, i, "--file")?)?);
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
        (256..=MAX_REVIEW_EXCERPT_BYTES).contains(&parsed.max_diff_bytes),
        "--max-diff-bytes must be between 256 and {MAX_REVIEW_EXCERPT_BYTES}"
    );
    ensure!(
        (1..=900).contains(&parsed.timeout_secs),
        "--timeout-secs must be between 1 and 900"
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

fn validate_include_file(raw: &str) -> Result<String> {
    let value = raw.trim();
    ensure!(!value.is_empty(), "--file must not be empty");
    let path = Path::new(value);
    ensure!(!path.is_absolute(), "--file must be repo-relative");
    ensure!(
        !path.components().any(|component| matches!(
            component,
            Component::ParentDir | Component::RootDir | Component::Prefix(_)
        )),
        "--file must not contain traversal or absolute path components"
    );
    ensure!(
        !value.contains('\\'),
        "--file must use forward-slash repo-relative paths"
    );
    ensure!(
        value
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '/' | '.' | '_' | '-')),
        "--file contains unsupported path characters"
    );
    ensure!(
        !value.contains("..")
            && !value.contains("//")
            && !value.starts_with('-')
            && !value.ends_with('/')
            && !value.ends_with(".lock"),
        "--file contains unsupported path patterns"
    );
    ensure!(
        !is_sensitive_review_path(value),
        "--file points at a sensitive path pattern that must not be sent to a model reviewer"
    );
    Ok(value.to_string())
}

fn is_sensitive_review_path(path: &str) -> bool {
    let lower = path.to_ascii_lowercase();
    let file_name = lower.rsplit('/').next().unwrap_or("");
    let extension = file_name.rsplit_once('.').map(|(_, ext)| ext);
    file_name == ".env"
        || file_name.starts_with(".env.")
        || lower == ".ssh"
        || lower.starts_with(".ssh/")
        || lower.contains("/.ssh/")
        || matches!(extension, Some("pem" | "key" | "p12" | "pfx"))
}

fn validate_git_ref(raw: &str, flag: &str) -> Result<String> {
    let value = raw.trim();
    ensure!(!value.is_empty(), "{flag} must not be empty");
    ensure!(
        !value.starts_with('-'),
        "{flag} must be a revision, not an option"
    );
    ensure!(
        value
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '/' | '.' | '_' | '-' | '^' | '~')),
        "{flag} contains unsupported revision characters"
    );
    ensure!(
        !value.contains(':')
            && !value.contains("..")
            && !value.contains("//")
            && !value.contains("^^")
            && !value.contains("~~")
            && !value.contains("^~")
            && !value.contains("~^")
            && !value.ends_with('/')
            && !value.ends_with(".lock"),
        "{flag} contains unsupported revision characters"
    );
    ensure!(
        !value.ends_with('^') && !value.ends_with('~'),
        "{flag} contains incomplete parent or ancestor syntax"
    );
    Ok(value.to_string())
}

fn build_packet(root: &Path, args: &CodeReviewArgs) -> Result<ReviewPacket> {
    let branch = git_output(root, &["rev-parse", "--abbrev-ref", "HEAD"])
        .unwrap_or_else(|_| "unknown".to_string());
    let changed_files = changed_files(root, args)?;
    let focused_diff_hunks = diff_hunks(root, args, &changed_files)?;
    let file_contexts = file_contexts(root, args, &changed_files)?;
    let truncated_hunks = focused_diff_hunks.iter().any(|hunk| hunk.truncated);
    let file_limit_truncated = changed_files.len() > MAX_REVIEW_DIFF_FILES
        || changed_files.len() > MAX_REVIEW_CONTEXT_FILES;
    let static_analysis_evidence = static_evidence(root, args);
    let packet_text =
        serde_json::to_string(&(&focused_diff_hunks, &file_contexts)).unwrap_or_default();
    let redaction_status = RedactionStatus {
        absolute_host_paths_present: review_packet_contains_absolute_host_path(
            &focused_diff_hunks,
            &file_contexts,
        ),
        secret_like_values_present: contains_secret_like_token(&packet_text),
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
            max_diff_files: MAX_REVIEW_DIFF_FILES,
            max_context_files: MAX_REVIEW_CONTEXT_FILES,
            file_limit_truncated,
            truncated_hunks,
        },
        focused_diff_hunks,
        file_contexts,
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

fn changed_files(root: &Path, args: &CodeReviewArgs) -> Result<Vec<String>> {
    let mut files = BTreeSet::new();
    if let Ok(output) = git_output(
        root,
        &[
            "diff",
            "--name-only",
            "--end-of-options",
            &format!("{}...{}", args.base_ref, args.head_ref),
        ],
    ) {
        for line in output
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
        {
            files.insert(validate_include_file(line)?);
        }
    }
    if args.include_working_tree {
        let output = git_output(root, &["diff", "--name-only"])?;
        for line in output
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
        {
            files.insert(validate_include_file(line)?);
        }
    }
    if !args.include_files.is_empty() {
        for file in &args.include_files {
            ensure!(
                files.contains(file),
                "--file '{file}' is not in the changed file set for the selected base/head or working tree"
            );
        }
        return Ok(args.include_files.clone());
    }
    Ok(files.into_iter().collect())
}

fn diff_hunks(root: &Path, args: &CodeReviewArgs, files: &[String]) -> Result<Vec<DiffHunk>> {
    let mut hunks = Vec::new();
    for file in files.iter().take(MAX_REVIEW_DIFF_FILES) {
        let committed_diff = git_output(
            root,
            &[
                "diff",
                "--end-of-options",
                &format!("{}...{}", args.base_ref, args.head_ref),
                "--",
                file,
            ],
        )
        .unwrap_or_default();
        let staged_diff = if args.include_working_tree {
            git_output(root, &["diff", "--cached", "--", file]).unwrap_or_default()
        } else {
            String::new()
        };
        let unstaged_diff = if args.include_working_tree {
            git_output(root, &["diff", "--", file]).unwrap_or_default()
        } else {
            String::new()
        };
        let mut diff_parts = Vec::new();
        if !committed_diff.trim().is_empty() {
            diff_parts.push(format!(
                "--- committed diff ({}...{}) ---\n{}",
                args.base_ref, args.head_ref, committed_diff
            ));
        }
        if !staged_diff.trim().is_empty() {
            diff_parts.push(format!("--- staged working tree diff ---\n{staged_diff}"));
        }
        if !unstaged_diff.trim().is_empty() {
            diff_parts.push(format!(
                "--- unstaged working tree diff ---\n{unstaged_diff}"
            ));
        }
        let diff = diff_parts.join("\n");
        let (diff_excerpt, truncated) = truncate(&diff, args.max_diff_bytes);
        hunks.push(DiffHunk {
            file: file.clone(),
            diff_excerpt,
            truncated,
        });
    }
    Ok(hunks)
}

fn file_contexts(root: &Path, args: &CodeReviewArgs, files: &[String]) -> Result<Vec<FileContext>> {
    let mut contexts = Vec::new();
    let max_read_bytes = args.max_diff_bytes.saturating_add(1);
    let canonical_root = fs::canonicalize(root).context("canonicalize repo root")?;
    for file in files.iter().take(MAX_REVIEW_CONTEXT_FILES) {
        let content = if args.include_working_tree {
            safe_read_worktree_file(root, &canonical_root, file, max_read_bytes)
                .or_else(|_| git_show_file_prefix(root, &args.head_ref, file, max_read_bytes))
        } else {
            git_show_file_prefix(root, &args.head_ref, file, max_read_bytes)
                .or_else(|_| safe_read_worktree_file(root, &canonical_root, file, max_read_bytes))
        };
        match content {
            Ok(content) => {
                let (current_excerpt, truncated) = truncate(&content, args.max_diff_bytes);
                contexts.push(FileContext {
                    file: file.clone(),
                    current_excerpt,
                    truncated,
                    read_error: None,
                });
            }
            Err(err) if !args.include_files.is_empty() => {
                bail!("failed to read explicit --file context for '{file}': {err}");
            }
            Err(err) => contexts.push(FileContext {
                file: file.clone(),
                current_excerpt: String::new(),
                truncated: false,
                read_error: Some(truncate(&err.to_string(), 240).0),
            }),
        }
    }
    Ok(contexts)
}

fn safe_read_worktree_file(
    root: &Path,
    canonical_root: &Path,
    file: &str,
    max_bytes: usize,
) -> Result<String> {
    let candidate = root.join(file);
    let canonical_candidate = fs::canonicalize(&candidate)
        .with_context(|| format!("canonicalize review file '{}'", candidate.display()))?;
    ensure!(
        canonical_candidate.starts_with(canonical_root),
        "review file resolves outside repo root"
    );
    read_file_prefix(&canonical_candidate, max_bytes).context("read review file")
}

fn git_show_file_prefix(
    root: &Path,
    head_ref: &str,
    file: &str,
    max_bytes: usize,
) -> Result<String> {
    let spec = format!("{head_ref}:{file}");
    let mut child = Command::new("git")
        .args(["show", "--end-of-options", &spec])
        .current_dir(root)
        .stdout(Stdio::piped())
        .spawn()
        .with_context(|| format!("spawn git show for '{file}'"))?;
    let mut stdout = child.stdout.take().context("capture git show stdout")?;
    let mut bytes = Vec::new();
    stdout
        .by_ref()
        .take(max_bytes as u64)
        .read_to_end(&mut bytes)
        .context("read git show stdout")?;
    drop(stdout);
    let status = child.wait().context("wait for git show")?;
    ensure!(
        status.success() || !bytes.is_empty(),
        "git show failed for '{file}'"
    );
    Ok(String::from_utf8_lossy(&bytes).to_string())
}

fn read_file_prefix(path: &Path, max_bytes: usize) -> Result<String> {
    let file = fs::File::open(path)?;
    let mut bytes = Vec::new();
    file.take(max_bytes as u64).read_to_end(&mut bytes)?;
    Ok(String::from_utf8_lossy(&bytes).to_string())
}

fn static_evidence(root: &Path, args: &CodeReviewArgs) -> Vec<ValidationEvidence> {
    let mut evidence = Vec::new();
    if args.include_working_tree {
        evidence.push(command_evidence(
            root,
            &["diff", "--check"],
            "working tree whitespace check",
        ));
    }
    evidence.push(command_evidence(
        root,
        &[
            "diff",
            "--check",
            "--end-of-options",
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
                "fixture backend proves artifact shape only and cannot bless PR publication"
                    .to_string(),
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

fn non_proving_review_result(
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

fn evaluate_gate(result: &ReviewResult, packet: &ReviewPacket) -> GateResult {
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
        schema_version: GATE_SCHEMA,
        gate_disposition: gate_disposition.to_string(),
        pr_open_allowed,
        reasons,
    }
}

fn reviewer_prompt(packet: &ReviewPacket) -> String {
    let packet_json = redact_absolute_host_paths_for_prompt(
        &serde_json::to_string_pretty(packet).unwrap_or_default(),
    );
    format!(
        "/no_think\nYou are an ADL code reviewer. Return final JSON immediately, with no chain-of-thought, analysis transcript, markdown, or prose. Do not call tools, do not emit action/action_input JSON, and do not request web search or repository commands. Be skeptical, concrete, and adversarial-but-fair. Your job is to find defects before PR publication, not to approve work. Review this bounded packet for correctness, fail-open gates, missing tests, path traversal, packet completeness, unsafe reviewer/session handling, timeout/truncation behavior, misleading docs, and lifecycle drift. Return only compact JSON matching schema_version adl.pr_review_result.v1 with fields disposition (blessed|blocked|non_proving), findings array, and residual_risk array. The top-level JSON object must not contain keys named action, action_input, tool, tool_call, function_call, or arguments. Include at most 5 findings. Keep each title under 12 words, each body under 90 words, and each evidence string under 120 characters. Findings are only actionable risks, bugs, regressions, missing tests, security issues, or misleading documentation. Do not put praise, confirmations, or already-correct behavior in findings. A security finding must cite the exact missing guard or a concrete bypass input; do not report a hypothetical bypass if the packet shows an existing guard that blocks it. Any blocking P0/P1 security finding must include at least one evidence string starting with 'bypass:' followed by the concrete malicious input or state transition. If you include a finding, every finding must include a specific title, priority (P0|P1|P2|P3), file, body, evidence array, heuristic_ids array, confidence, blocking, and suggested_fix_scope. Do not include placeholder, empty, or non-actionable findings. If you find no actionable issues, set disposition to blessed only after checking every included diff hunk, file_context, file-limit flag, and read_error. Residual_risk must list concrete checked invariants or limitations with file/function references. Do not use approval language such as approved, correct, good, sound, aligns, or looks fine as residual rationale. If the packet is truncated or has read errors enough that you cannot review it fairly, set disposition to non_proving. Packet:\n{}",
        packet_json
    )
}

fn redact_absolute_host_paths_for_prompt(text: &str) -> String {
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
                redacted.push_str("[REDACTED_HOST_PATH]\\");
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

fn review_packet_contains_absolute_host_path(hunks: &[DiffHunk], contexts: &[FileContext]) -> bool {
    hunks
        .iter()
        .any(|hunk| contains_review_absolute_host_path(&hunk.diff_excerpt))
        || contexts
            .iter()
            .any(|context| contains_review_absolute_host_path(&context.current_excerpt))
}

fn contains_review_absolute_host_path(text: &str) -> bool {
    ["/Users/", "/home/", "/tmp/", "/var/folders/"]
        .iter()
        .any(|needle| text.contains(needle))
        || contains_windows_absolute_path(text)
}

fn contains_windows_absolute_path(text: &str) -> bool {
    let mut chars = text.char_indices().peekable();
    while let Some((idx, ch)) = chars.next() {
        if ch.is_ascii_alphabetic()
            && is_windows_drive_boundary(text, idx)
            && chars.peek().map(|(_, next)| next) == Some(&':')
        {
            chars.next();
            if chars.peek().map(|(_, next)| next) == Some(&'\\') {
                return true;
            }
        }
    }
    false
}

fn is_windows_drive_boundary(text: &str, idx: usize) -> bool {
    idx == 0
        || !text[..idx]
            .chars()
            .next_back()
            .is_some_and(|ch| ch.is_ascii_alphanumeric() || ch == '_')
}

struct ParsedModelReview {
    disposition: ReviewDisposition,
    findings: Vec<ReviewFinding>,
    residual_risk: Vec<String>,
}

fn parse_model_review_json(raw: &str) -> Option<ParsedModelReview> {
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

fn normalize_model_review(
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

#[cfg(test)]
mod tests {
    use super::*;

    fn test_args() -> CodeReviewArgs {
        CodeReviewArgs {
            out: PathBuf::from("artifacts/review"),
            backend: ReviewerBackend::Fixture,
            visibility_mode: VisibilityMode::PacketOnly,
            base_ref: "origin/main".to_string(),
            head_ref: "HEAD".to_string(),
            issue_number: Some(2603),
            writer_session: "writer".to_string(),
            reviewer_session: Some("reviewer".to_string()),
            model: None,
            allow_live_ollama: false,
            ollama_url: "http://127.0.0.1:11434".to_string(),
            timeout_secs: 120,
            include_working_tree: false,
            fixture_case: FixtureCase::Clean,
            max_diff_bytes: DEFAULT_REVIEW_EXCERPT_BYTES,
            include_files: Vec::new(),
        }
    }

    fn test_packet() -> ReviewPacket {
        ReviewPacket {
            schema_version: PACKET_SCHEMA,
            issue_number: Some(2603),
            branch: "codex/test".to_string(),
            base_ref: "origin/main".to_string(),
            head_ref: "HEAD".to_string(),
            visibility_mode: VisibilityMode::PacketOnly,
            changed_files: vec!["docs/example.md".to_string()],
            diff_summary: DiffSummary {
                files_changed: 1,
                max_diff_bytes: DEFAULT_REVIEW_EXCERPT_BYTES,
                max_diff_files: MAX_REVIEW_DIFF_FILES,
                max_context_files: MAX_REVIEW_CONTEXT_FILES,
                file_limit_truncated: false,
                truncated_hunks: false,
            },
            focused_diff_hunks: vec![DiffHunk {
                file: "docs/example.md".to_string(),
                diff_excerpt: "+example".to_string(),
                truncated: false,
            }],
            file_contexts: vec![FileContext {
                file: "docs/example.md".to_string(),
                current_excerpt: "example".to_string(),
                truncated: false,
                read_error: None,
            }],
            validation_evidence: Vec::new(),
            static_analysis_evidence: Vec::new(),
            repo_slice_manifest: RepoSliceManifest {
                read_only: false,
                write_allowed: false,
                tool_execution_allowed: false,
                files: vec!["docs/example.md".to_string()],
            },
            review_scope: "test review scope".to_string(),
            non_scope: vec!["do not edit files".to_string()],
            known_risks: Vec::new(),
            redaction_status: RedactionStatus {
                absolute_host_paths_present: false,
                secret_like_values_present: false,
            },
        }
    }

    #[test]
    fn parse_args_preserves_backend_after_out_and_accepts_timeout() {
        let args = vec![
            "--out".to_string(),
            "artifacts/review".to_string(),
            "--backend".to_string(),
            "ollama".to_string(),
            "--timeout-secs".to_string(),
            "240".to_string(),
            "--include-working-tree".to_string(),
            "--file".to_string(),
            "adl/src/cli/tooling_cmd/code_review.rs".to_string(),
        ];
        let parsed = parse_args(&args).expect("args should parse");
        assert_eq!(parsed.backend, ReviewerBackend::Ollama);
        assert_eq!(parsed.timeout_secs, 240);
        assert!(parsed.include_working_tree);
        assert_eq!(
            parsed.include_files,
            vec!["adl/src/cli/tooling_cmd/code_review.rs"]
        );
    }

    #[test]
    fn parse_args_excludes_working_tree_by_default() {
        let args = vec!["--out".to_string(), "artifacts/review".to_string()];
        let parsed = parse_args(&args).expect("args should parse");
        assert!(!parsed.include_working_tree);
    }

    #[test]
    fn parse_args_rejects_invalid_values_and_missing_required_out() {
        assert!(parse_args(&["--backend".to_string(), "bad".to_string()]).is_err());
        assert!(parse_args(&[
            "--out".to_string(),
            "artifacts/review".to_string(),
            "--visibility".to_string(),
            "bad".to_string(),
        ])
        .is_err());
        assert!(parse_args(&[
            "--out".to_string(),
            "artifacts/review".to_string(),
            "--fixture-case".to_string(),
            "bad".to_string(),
        ])
        .is_err());
        assert!(parse_args(&[
            "--out".to_string(),
            "artifacts/review".to_string(),
            "--timeout-secs".to_string(),
            "0".to_string(),
        ])
        .is_err());
        assert!(parse_args(&[
            "--out".to_string(),
            "artifacts/review".to_string(),
            "--max-diff-bytes".to_string(),
            "255".to_string(),
        ])
        .is_err());
        assert!(parse_args(&[
            "--out".to_string(),
            "artifacts/review".to_string(),
            "--max-diff-bytes".to_string(),
            (MAX_REVIEW_EXCERPT_BYTES + 1).to_string(),
        ])
        .is_err());
        assert!(parse_args(&[
            "--out".to_string(),
            "artifacts/review".to_string(),
            "--file".to_string(),
            "../secret.txt".to_string(),
        ])
        .is_err());
        assert!(parse_args(&[
            "--out".to_string(),
            "artifacts/review".to_string(),
            "--file".to_string(),
            "/tmp/secret.txt".to_string(),
        ])
        .is_err());
        assert!(parse_args(&[
            "--out".to_string(),
            "artifacts/review".to_string(),
            "--file".to_string(),
            "adl\\src\\lib.rs".to_string(),
        ])
        .is_err());
        assert!(parse_args(&[
            "--out".to_string(),
            "artifacts/review".to_string(),
            "--file".to_string(),
            "docs/file with spaces.md".to_string(),
        ])
        .is_err());
        assert!(parse_args(&[
            "--out".to_string(),
            "artifacts/review".to_string(),
            "--file".to_string(),
            "docs/ref:path.md".to_string(),
        ])
        .is_err());
        assert!(parse_args(&[
            "--out".to_string(),
            "artifacts/review".to_string(),
            "--file".to_string(),
            ".env".to_string(),
        ])
        .is_err());
        assert!(parse_args(&[
            "--out".to_string(),
            "artifacts/review".to_string(),
            "--file".to_string(),
            "config/private.pem".to_string(),
        ])
        .is_err());
        assert!(parse_args(&[
            "--out".to_string(),
            "artifacts/review".to_string(),
            "--file".to_string(),
            ".ssh/config".to_string(),
        ])
        .is_err());
        assert!(parse_args(&[
            "--out".to_string(),
            "artifacts/review".to_string(),
            "--head".to_string(),
            "--help".to_string(),
        ])
        .is_err());
        assert!(parse_args(&[
            "--out".to_string(),
            "artifacts/review".to_string(),
            "--head".to_string(),
            "HEAD:secret".to_string(),
        ])
        .is_err());
        assert!(parse_args(&[
            "--out".to_string(),
            "artifacts/review".to_string(),
            "--head".to_string(),
            "HEAD --cached".to_string(),
        ])
        .is_err());
        assert!(parse_args(&[
            "--out".to_string(),
            "artifacts/review".to_string(),
            "--base".to_string(),
            "origin/main..topic".to_string(),
        ])
        .is_err());
        assert!(parse_args(&[
            "--out".to_string(),
            "artifacts/review".to_string(),
            "--base".to_string(),
            "953e913f^".to_string(),
        ])
        .is_err());
        assert!(parse_args(&[
            "--out".to_string(),
            "artifacts/review".to_string(),
            "--base".to_string(),
            "953e913f^^1".to_string(),
        ])
        .is_err());
        assert!(parse_args(&["--writer-session".to_string(), "".to_string()]).is_err());
        assert!(parse_args(&["--out".to_string()]).is_err());
        assert!(parse_args(&["--unknown".to_string()]).is_err());
    }

    #[test]
    fn parse_args_accepts_safe_parent_and_ancestor_git_refs() {
        let parsed = parse_args(&[
            "--out".to_string(),
            "artifacts/review".to_string(),
            "--base".to_string(),
            "953e913f^1".to_string(),
            "--head".to_string(),
            "HEAD~1".to_string(),
        ])
        .expect("safe parent and ancestor refs should parse");
        assert_eq!(parsed.base_ref, "953e913f^1");
        assert_eq!(parsed.head_ref, "HEAD~1");
    }

    #[test]
    fn changed_files_rejects_file_filter_outside_changed_set() {
        let root = repo_root().expect("repo root");
        let mut args = test_args();
        args.base_ref = "HEAD".to_string();
        args.head_ref = "HEAD".to_string();
        args.include_working_tree = false;
        args.include_files = vec!["adl/src/cli/tooling_cmd/code_review.rs".to_string()];

        let err = changed_files(&root, &args).expect_err("unchanged file filter should fail");
        assert!(err.to_string().contains("not in the changed file set"));
    }

    #[test]
    fn read_file_prefix_bounds_file_context_memory() {
        let path =
            std::env::temp_dir().join(format!("adl-code-review-prefix-{}.txt", std::process::id()));
        fs::write(&path, "abcdef").expect("write temp");
        let text = read_file_prefix(&path, 3).expect("read prefix");
        fs::remove_file(&path).ok();
        assert_eq!(text, "abc");
    }

    #[test]
    fn fixture_review_covers_blocked_and_same_session_paths() {
        let packet = test_packet();
        let packet_id = "packet-id";

        let mut blocked_args = test_args();
        blocked_args.fixture_case = FixtureCase::Blocked;
        let blocked = fixture_review(&blocked_args, &packet, packet_id);
        assert_eq!(blocked.disposition, ReviewDisposition::Blocked);
        assert_eq!(blocked.findings.len(), 1);

        let clean = fixture_review(&test_args(), &packet, packet_id);
        assert_eq!(clean.disposition, ReviewDisposition::NonProving);
        let clean_gate = evaluate_gate(&clean, &packet);
        assert!(!clean_gate.pr_open_allowed);

        let mut same_session_args = test_args();
        same_session_args.reviewer_session = Some("writer".to_string());
        let same_session = fixture_review(&same_session_args, &packet, packet_id);
        assert_eq!(same_session.disposition, ReviewDisposition::NonProving);
        assert!(same_session.same_session_as_writer);
    }

    #[test]
    fn ollama_without_live_returns_skipped_blocker() {
        let mut args = test_args();
        args.backend = ReviewerBackend::Ollama;
        args.model = Some("gemma4:test".to_string());
        let packet = test_packet();
        let result = ollama_review(&args, &packet, "packet-id").expect("skipped review");
        assert_eq!(result.disposition, ReviewDisposition::Skipped);
        let gate = evaluate_gate(&result, &packet);
        assert!(!gate.pr_open_allowed);
        assert!(gate.reasons.iter().any(|reason| reason.contains("skipped")));
    }

    #[test]
    fn ollama_live_review_suppresses_redaction_failures() {
        let mut args = test_args();
        args.backend = ReviewerBackend::Ollama;
        args.allow_live_ollama = true;
        args.model = Some("gemma4:test".to_string());
        let mut packet = test_packet();
        packet.redaction_status.secret_like_values_present = true;

        let result =
            ollama_review(&args, &packet, "packet-id").expect("redaction failure should not call");
        assert_eq!(result.disposition, ReviewDisposition::NonProving);
        assert!(result
            .residual_risk
            .iter()
            .any(|risk| { risk.contains("live model invocation suppressed") }));
        let gate = evaluate_gate(&result, &packet);
        assert!(!gate.pr_open_allowed);
    }

    #[test]
    fn evaluate_gate_covers_static_failure_and_blocking_finding() {
        let mut packet = test_packet();
        packet.static_analysis_evidence.push(ValidationEvidence {
            command: "git diff --check".to_string(),
            status: "FAIL".to_string(),
            summary: "whitespace error".to_string(),
        });
        packet.redaction_status.absolute_host_paths_present = true;
        let mut result = review_result(
            &test_args(),
            &packet,
            "packet-id",
            ReviewResultParts {
                reviewer_session: "reviewer".to_string(),
                reviewer_model: "fixture".to_string(),
                same_session: false,
                disposition: ReviewDisposition::Blessed,
                findings: vec![ReviewFinding {
                    title: "Blocking issue".to_string(),
                    priority: "P2".to_string(),
                    file: "docs/example.md".to_string(),
                    line: Some(1),
                    body: "A concrete problem exists.".to_string(),
                    evidence: vec!["path:docs/example.md".to_string()],
                    heuristic_ids: vec!["T1".to_string()],
                    confidence: "high".to_string(),
                    blocking: true,
                    suggested_fix_scope: "issue_local".to_string(),
                }],
                residual_risk: Vec::new(),
            },
        );
        result.same_session_as_writer = true;
        let gate = evaluate_gate(&result, &packet);
        assert!(!gate.pr_open_allowed);
        assert!(gate
            .reasons
            .iter()
            .any(|reason| reason.contains("static analysis")));
        assert!(gate
            .reasons
            .iter()
            .any(|reason| reason.contains("absolute host paths")));
        assert!(gate
            .reasons
            .iter()
            .any(|reason| reason.contains("blocking P2 finding")));
        assert!(gate
            .reasons
            .iter()
            .any(|reason| reason.contains("matches writer session")));

        result.disposition = ReviewDisposition::NonProving;
        result.same_session_as_writer = false;
        packet.static_analysis_evidence.clear();
        let non_proving_gate = evaluate_gate(&result, &packet);
        assert!(!non_proving_gate.pr_open_allowed);
        assert!(non_proving_gate
            .reasons
            .iter()
            .any(|reason| reason.contains("non_proving")));
        assert!(!non_proving_gate
            .reasons
            .iter()
            .any(|reason| reason.contains("blocking P2 finding")));
    }

    #[test]
    fn parse_model_review_json_accepts_fenced_json_and_filters_string_arrays() {
        let raw = r#"```json
{
  "disposition": "blocked",
  "findings": [
    {
      "title": "Missing evidence",
      "priority": "P3",
      "file": "docs/example.md",
      "line": 12,
      "body": "The example references a fact without evidence.",
      "evidence": ["path:docs/example.md", 42],
      "heuristic_ids": ["D1", false],
      "confidence": "medium",
      "blocking": false,
      "suggested_fix_scope": "issue_local"
    }
  ]
}
```"#;
        let parsed = parse_model_review_json(raw).expect("parse review json");
        assert_eq!(parsed.disposition, ReviewDisposition::Blocked);
        assert_eq!(parsed.findings[0].evidence, vec!["path:docs/example.md"]);
        assert_eq!(parsed.findings[0].heuristic_ids, vec!["D1"]);
        assert_eq!(parsed.findings[0].line, Some(12));
        assert!(parse_model_review_json("{not-json").is_none());
        assert!(parse_model_review_json(r#"{"disposition":"unexpected"}"#).is_none());
    }

    #[test]
    fn helpers_cover_url_normalization_prompt_and_unicode_truncation() {
        assert_eq!(
            ollama_generate_url("http://127.0.0.1:11434").expect("url"),
            "http://127.0.0.1:11434/api/generate"
        );
        assert_eq!(
            ollama_generate_url("http://127.0.0.1:11434/api/generate").expect("url"),
            "http://127.0.0.1:11434/api/generate"
        );
        assert!(ollama_generate_url("not a url").is_err());

        let prompt = reviewer_prompt(&test_packet());
        assert!(prompt.contains("actionable risks"));
        assert!(prompt.contains("adl.pr_review_result.v1"));
        assert_eq!(
            redact_absolute_host_paths_for_prompt("/tmp/secret.txt /Users/alice/repo C:\\secret"),
            "[REDACTED_HOST_PATH]/secret.txt [REDACTED_HOST_PATH]/alice/repo [REDACTED_HOST_PATH]\\secret"
        );
        assert!(!contains_review_absolute_host_path("Expected signal:\\n"));
        assert!(contains_review_absolute_host_path("C:\\secret"));

        let (truncated, was_truncated) = truncate("éclair", 3);
        assert!(was_truncated);
        assert_eq!(truncated, "éc");
    }

    #[test]
    fn normalize_model_review_rejects_placeholder_findings() {
        let finding = ReviewFinding {
            title: "Untitled model finding".to_string(),
            priority: "P3".to_string(),
            file: "unknown".to_string(),
            line: None,
            body: String::new(),
            evidence: Vec::new(),
            heuristic_ids: Vec::new(),
            confidence: "medium".to_string(),
            blocking: false,
            suggested_fix_scope: "issue_local".to_string(),
        };

        let (disposition, findings, residual) = normalize_model_review(
            ParsedModelReview {
                disposition: ReviewDisposition::Blessed,
                findings: vec![finding],
                residual_risk: vec![
                    "Reviewed docs/example.md finding evidence and no packet truncation flags were present."
                        .to_string(),
                ],
            },
            &test_packet(),
        );
        assert_eq!(disposition, ReviewDisposition::NonProving);
        assert_eq!(findings.len(), 1);
        assert!(residual
            .iter()
            .any(|risk| risk.contains("malformed findings")));
        assert!(residual
            .iter()
            .any(|risk| risk.contains("missing concrete evidence")));
    }

    #[test]
    fn normalize_model_review_rejects_praise_as_findings() {
        let finding = ReviewFinding {
            title: "Correctly separates proposal and execution".to_string(),
            priority: "P3".to_string(),
            file: "docs/example.md".to_string(),
            line: None,
            body: "The change correctly preserves the intended safety boundary.".to_string(),
            evidence: vec!["diff excerpt".to_string()],
            heuristic_ids: vec!["ADL-REVIEW".to_string()],
            confidence: "high".to_string(),
            blocking: false,
            suggested_fix_scope: "None. This is already correct.".to_string(),
        };

        let (disposition, findings, residual) = normalize_model_review(
            ParsedModelReview {
                disposition: ReviewDisposition::Blessed,
                findings: vec![finding],
                residual_risk: Vec::new(),
            },
            &test_packet(),
        );
        assert_eq!(disposition, ReviewDisposition::NonProving);
        assert_eq!(findings.len(), 1);
        assert!(residual
            .iter()
            .any(|risk| risk.contains("non-actionable praise")));
    }

    #[test]
    fn normalize_model_review_accepts_specific_evidenced_finding() {
        let finding = ReviewFinding {
            title: "Missing reviewer evidence link".to_string(),
            priority: "P3".to_string(),
            file: "docs/example.md".to_string(),
            line: Some(12),
            body: "The review packet mentions a follow-up but does not link the evidence."
                .to_string(),
            evidence: vec!["path:docs/example.md".to_string()],
            heuristic_ids: vec!["C1".to_string()],
            confidence: "medium".to_string(),
            blocking: false,
            suggested_fix_scope: "issue_local".to_string(),
        };

        let (disposition, findings, residual) = normalize_model_review(
            ParsedModelReview {
                disposition: ReviewDisposition::Blessed,
                findings: vec![finding],
                residual_risk: vec![
                    "Reviewed docs/example.md finding evidence and no packet truncation flags were present."
                        .to_string(),
                ],
            },
            &test_packet(),
        );
        assert_eq!(disposition, ReviewDisposition::Blessed);
        assert_eq!(findings.len(), 1);
        assert_eq!(residual.len(), 1);
    }

    #[test]
    fn normalize_model_review_requires_bypass_for_blocking_security_findings() {
        let finding = ReviewFinding {
            title: "Possible path traversal".to_string(),
            priority: "P0".to_string(),
            file: "adl/src/cli/tooling_cmd/code_review.rs".to_string(),
            line: None,
            body: "The path might be exploitable without a concrete bypass.".to_string(),
            evidence: vec!["safe_read_worktree_file".to_string()],
            heuristic_ids: vec!["SEC-PATH-TRAVERSAL".to_string()],
            confidence: "medium".to_string(),
            blocking: true,
            suggested_fix_scope: "function_local".to_string(),
        };

        let (disposition, _, residual) = normalize_model_review(
            ParsedModelReview {
                disposition: ReviewDisposition::Blocked,
                findings: vec![finding],
                residual_risk: Vec::new(),
            },
            &test_packet(),
        );

        assert_eq!(disposition, ReviewDisposition::NonProving);
        assert!(residual
            .iter()
            .any(|risk| risk.contains("without concrete bypass evidence")));
    }

    #[test]
    fn normalize_model_review_rejects_bypass_rejected_by_cli_validators() {
        let finding = ReviewFinding {
            title: "Path traversal bypass".to_string(),
            priority: "P1".to_string(),
            file: "adl/src/cli/tooling_cmd/code_review.rs".to_string(),
            line: None,
            body: "The model claims --file traversal is accepted.".to_string(),
            evidence: vec!["bypass: --file ../../etc/passwd".to_string()],
            heuristic_ids: vec!["SEC-PATH-TRAVERSAL".to_string()],
            confidence: "high".to_string(),
            blocking: true,
            suggested_fix_scope: "function_local".to_string(),
        };

        let (disposition, _, residual) = normalize_model_review(
            ParsedModelReview {
                disposition: ReviewDisposition::Blocked,
                findings: vec![finding],
                residual_risk: Vec::new(),
            },
            &test_packet(),
        );

        assert_eq!(disposition, ReviewDisposition::NonProving);
        assert!(residual
            .iter()
            .any(|risk| risk.contains("rejected by the current CLI validators")));
    }

    #[test]
    fn normalize_model_review_rejects_empty_blessing_for_truncated_packet() {
        let mut packet = test_packet();
        packet.diff_summary.truncated_hunks = true;
        packet.focused_diff_hunks[0].truncated = true;
        packet.file_contexts[0].truncated = true;

        let (disposition, findings, residual) = normalize_model_review(
            ParsedModelReview {
                disposition: ReviewDisposition::Blessed,
                findings: Vec::new(),
                residual_risk: Vec::new(),
            },
            &packet,
        );

        assert_eq!(disposition, ReviewDisposition::NonProving);
        assert!(findings.is_empty());
        assert!(residual
            .iter()
            .any(|risk| risk.contains("incomplete packet")));
        assert!(residual
            .iter()
            .any(|risk| risk.contains("no residual-review rationale")));
    }

    #[test]
    fn normalize_model_review_accepts_empty_blessing_with_rationale_for_complete_packet() {
        let (disposition, findings, residual) = normalize_model_review(
            ParsedModelReview {
                disposition: ReviewDisposition::Blessed,
                findings: Vec::new(),
                residual_risk: vec![
                    "Reviewed complete docs/example.md diff, including file_contexts and focused_diff_hunks; no actionable defect evidence was present."
                        .to_string(),
                ],
            },
            &test_packet(),
        );

        assert_eq!(disposition, ReviewDisposition::Blessed);
        assert!(findings.is_empty());
        assert_eq!(residual.len(), 1);
    }

    #[test]
    fn normalize_model_review_rejects_blessed_incomplete_packet_with_findings() {
        let finding = ReviewFinding {
            title: "Missing reviewer evidence link".to_string(),
            priority: "P3".to_string(),
            file: "docs/example.md".to_string(),
            line: Some(12),
            body: "The review packet mentions a follow-up but does not link the evidence."
                .to_string(),
            evidence: vec!["path:docs/example.md".to_string()],
            heuristic_ids: vec!["C1".to_string()],
            confidence: "medium".to_string(),
            blocking: false,
            suggested_fix_scope: "issue_local".to_string(),
        };
        let mut packet = test_packet();
        packet.diff_summary.truncated_hunks = true;

        let (disposition, findings, residual) = normalize_model_review(
            ParsedModelReview {
                disposition: ReviewDisposition::Blessed,
                findings: vec![finding],
                residual_risk: vec![
                    "Reviewed docs/example.md finding evidence and packet limits.".to_string(),
                ],
            },
            &packet,
        );

        assert_eq!(disposition, ReviewDisposition::NonProving);
        assert_eq!(findings.len(), 1);
        assert!(residual
            .iter()
            .any(|risk| risk.contains("incomplete packet")));
    }

    #[test]
    fn normalize_model_review_rejects_empty_blessing_with_praise_rationale() {
        let (disposition, findings, residual) = normalize_model_review(
            ParsedModelReview {
                disposition: ReviewDisposition::Blessed,
                findings: Vec::new(),
                residual_risk: vec![
                    "The implementation is logically sound and correctly tested.".to_string(),
                ],
            },
            &test_packet(),
        );

        assert_eq!(disposition, ReviewDisposition::NonProving);
        assert!(findings.is_empty());
        assert!(residual
            .iter()
            .any(|risk| risk.contains("praise-like residual rationale")));
    }
}
