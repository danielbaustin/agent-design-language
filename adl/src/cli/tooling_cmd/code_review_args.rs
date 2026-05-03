use anyhow::{anyhow, bail, ensure, Result};
use std::path::{Component, Path};

use super::code_review_types::{CodeReviewArgs, FixtureCase, MAX_REVIEW_EXCERPT_BYTES, ReviewerBackend, VisibilityMode};

pub(crate) fn parse_args(args: &[String]) -> Result<CodeReviewArgs> {
    let mut parsed = CodeReviewArgs {
        out: std::path::PathBuf::new(),
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
        max_diff_bytes: MAX_REVIEW_EXCERPT_BYTES,
        include_files: Vec::new(),
    };

    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--out" => {
                parsed.out = Path::new(value_arg(args, i, "--out")?).to_path_buf();
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
                parsed.reviewer_session = Some(value_arg(args, i, "--reviewer-session")?.to_string());
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
                parsed.include_files.push(validate_include_file(value_arg(args, i, "--file")?)?);
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
    ensure!((1..=900).contains(&parsed.timeout_secs), "--timeout-secs must be between 1 and 900");
    Ok(parsed)
}

fn value_arg<'a>(args: &'a [String], index: usize, flag: &str) -> Result<&'a str> {
    args.get(index + 1)
        .map(|value| value.as_str())
        .ok_or_else(|| anyhow!("missing value for {flag}"))
}

pub(crate) fn parse_backend(raw: &str) -> Result<ReviewerBackend> {
    match raw {
        "fixture" => Ok(ReviewerBackend::Fixture),
        "ollama" | "gemma4_local" => Ok(ReviewerBackend::Ollama),
        other => bail!("unsupported code-review backend '{other}'"),
    }
}

pub(crate) fn parse_visibility(raw: &str) -> Result<VisibilityMode> {
    match raw {
        "packet-only" | "packet_only" => Ok(VisibilityMode::PacketOnly),
        "read-only-repo" | "read_only_repo" => Ok(VisibilityMode::ReadOnlyRepo),
        other => bail!("unsupported visibility mode '{other}'"),
    }
}

pub(crate) fn parse_fixture_case(raw: &str) -> Result<FixtureCase> {
    match raw {
        "clean" => Ok(FixtureCase::Clean),
        "blocked" => Ok(FixtureCase::Blocked),
        other => bail!("unsupported fixture case '{other}'"),
    }
}

pub(crate) fn validate_include_file(raw: &str) -> Result<String> {
    let value = raw.trim();
    ensure!(!value.is_empty(), "--file must not be empty");
    let path = Path::new(value);
    ensure!(!path.is_absolute(), "--file must be repo-relative");
    ensure!(
        !path
            .components()
            .any(|component| matches!(component, Component::ParentDir | Component::RootDir | Component::Prefix(_))),
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

pub(crate) fn validate_git_ref(raw: &str, flag: &str) -> Result<String> {
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
