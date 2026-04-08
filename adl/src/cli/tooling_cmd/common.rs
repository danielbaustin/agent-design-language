use anyhow::{anyhow, bail, ensure, Result};
use serde_yaml::{Mapping, Value};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use adl::control_plane::{card_input_path, resolve_cards_root, resolve_primary_checkout_root};

pub(super) const REQUIRED_OUTPUT_SECTIONS: &[&str] = &[
    "Summary",
    "Artifacts produced",
    "Actions taken",
    "Main Repo Integration (REQUIRED)",
    "Validation",
    "Verification Summary",
    "Determinism Evidence",
    "Security / Privacy Checks",
    "Replay Artifacts",
    "Artifact Verification",
    "Decisions / Deviations",
    "Follow-ups / Deferred work",
];

pub(super) const ALLOWED_OUTPUT_STATUS: &[&str] = &["NOT_STARTED", "IN_PROGRESS", "DONE", "FAILED"];
pub(super) const REQUIRED_REVIEW_SURFACES: &[&str] = &[
    "card_review_checklist.v1",
    "card_review_output.v1",
    "card_reviewer_gpt.v1.1",
];
pub(super) const POINTER_PREFIX_ORDER: &[&str] = &["path:", "command:", "ci:", "artifact:"];
pub(super) const DECISIONS: &[&str] = &["PASS", "MINOR_FIXES", "MAJOR_ISSUES"];
pub(super) const EVIDENCE_STATES: &[&str] = &["contradicted", "not_evidenced", "not_applicable"];
pub(super) const VALIDATION_RESULTS: &[&str] = &["PASS", "FAIL", "PARTIAL"];
pub(super) const REQUIRED_REPO_REVIEW_HEADINGS: &[&str] = &[
    "Metadata",
    "Scope",
    "Findings",
    "System-Level Assessment",
    "Recommended Action Plan",
    "Follow-ups / Deferred Work",
    "Final Assessment",
];

pub(super) fn resolve_issue_or_input_arg(args: &[String]) -> Result<PathBuf> {
    let mut issue: Option<u32> = None;
    let mut input: Option<PathBuf> = None;
    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--issue" => {
                let Some(value) = args.get(i + 1) else {
                    bail!("missing value for --issue");
                };
                issue = Some(normalize_issue(value)?);
                i += 1;
            }
            "--input" => {
                let Some(value) = args.get(i + 1) else {
                    bail!("missing value for --input");
                };
                input = Some(PathBuf::from(value));
                i += 1;
            }
            "--help" | "-h" => return Ok(PathBuf::new()),
            other => bail!("unknown argument: {other}"),
        }
        i += 1;
    }
    if issue.is_some() && input.is_some() {
        bail!("use either --issue or --input, not both");
    }
    if let Some(issue) = issue {
        resolve_input_card_path(issue)
    } else if let Some(input) = input {
        Ok(input)
    } else {
        bail!("missing --issue or --input")
    }
}

pub(super) fn normalize_issue(raw: &str) -> Result<u32> {
    raw.parse::<u32>()
        .map_err(|_| anyhow!("invalid issue number: {raw}"))
}

pub(super) fn resolve_input_card_path(issue: u32) -> Result<PathBuf> {
    let current_top =
        git_output(&["rev-parse", "--show-toplevel"]).unwrap_or_else(|_| cwd_string());
    let current_top = PathBuf::from(current_top);
    let common = git_output(&["rev-parse", "--git-common-dir"])
        .ok()
        .map(PathBuf::from);
    let root = resolve_primary_checkout_root(&current_top, common.as_deref());
    let cards_root = resolve_cards_root(&root, None);
    Ok(card_input_path(&cards_root, issue))
}

pub(super) fn repo_root() -> Result<PathBuf> {
    let root = git_output(&["rev-parse", "--show-toplevel"])?;
    Ok(PathBuf::from(root))
}

pub(super) fn git_output(args: &[&str]) -> Result<String> {
    let output = Command::new("git").args(args).output()?;
    if !output.status.success() {
        bail!("git command failed: git {}", args.join(" "));
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn cwd_string() -> String {
    std::env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .to_string_lossy()
        .to_string()
}

pub(super) fn absolutize(path: &Path) -> Result<PathBuf> {
    if path.is_absolute() {
        Ok(path.to_path_buf())
    } else {
        Ok(std::env::current_dir()?.join(path))
    }
}

pub(super) fn repo_relative_display(root: &Path, path: &Path) -> Result<String> {
    Ok(path
        .strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/"))
}

pub(super) fn ensure_file(path: &Path, label: &str) -> Result<()> {
    let path = absolutize(path)?;
    if !path.is_file() {
        bail!("{label} not found: {}", path.display());
    }
    Ok(())
}

pub(super) fn ensure_no_disallowed_content(path: &Path, label: &str) -> Result<()> {
    let text = fs::read_to_string(path)?;
    if contains_absolute_host_path_in_text(&text) || contains_secret_like_token(&text) {
        bail!("{label} contains disallowed secret-like token or absolute host path");
    }
    Ok(())
}

pub(super) fn ensure_no_absolute_host_path(path: &Path, prompt_type: &str) -> Result<()> {
    let text = fs::read_to_string(path)?;
    if contains_absolute_host_path_in_text(&text) {
        bail!("{prompt_type} contains disallowed absolute host path");
    }
    Ok(())
}

pub(super) fn contains_absolute_host_path_in_text(text: &str) -> bool {
    ["/Users/", "/home/", "/tmp/", "/var/folders/"]
        .iter()
        .any(|needle| text.contains(needle))
        || text.contains(":\\")
}

pub(super) fn contains_secret_like_token(text: &str) -> bool {
    if text.contains("AKIA")
        || text.contains("ghp_")
        || text.contains("gho_")
        || text.contains("ghu_")
        || text.contains("ghs_")
        || text.contains("ghr_")
    {
        return true;
    }

    for (idx, _) in text.match_indices("sk-") {
        let before_ok = idx == 0
            || !text[..idx]
                .chars()
                .next_back()
                .is_some_and(|c| c.is_ascii_alphanumeric() || c == '_');
        if !before_ok {
            continue;
        }
        let suffix = &text[idx + 3..];
        let token_len = suffix
            .chars()
            .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
            .count();
        if token_len < 8 {
            continue;
        }
        let after = suffix.chars().nth(token_len);
        let after_ok = !after.is_some_and(|c| c.is_ascii_alphanumeric() || c == '_');
        if after_ok {
            return true;
        }
    }

    false
}

pub(super) fn ensure_sorted_pointers(items: &[String], label: &str) -> Result<()> {
    let mut sorted = items.to_vec();
    sorted.sort_by_key(|pointer| pointer_sort_key(pointer));
    ensure!(
        items == sorted,
        "{label} must use canonical evidence-pointer ordering"
    );
    Ok(())
}

pub(super) fn pointer_sort_key(pointer: &str) -> (usize, String) {
    let prefix = POINTER_PREFIX_ORDER
        .iter()
        .position(|candidate| pointer.starts_with(candidate))
        .unwrap_or(usize::MAX);
    (prefix, pointer.to_string())
}

pub(super) fn is_repo_relative(value: &str) -> bool {
    !value.is_empty() && !value.starts_with('/') && !value.contains("..") && !value.contains(":\\")
}

pub(super) fn valid_task_id(value: &str) -> bool {
    let parts = value.split('-').collect::<Vec<_>>();
    parts.len() == 2
        && parts[0] == "issue"
        && parts[1].len() == 4
        && parts[1].chars().all(|c| c.is_ascii_digit())
}

pub(super) fn valid_version(value: &str) -> bool {
    let Some(rest) = value.strip_prefix('v') else {
        return false;
    };
    rest.split('.')
        .all(|part| !part.is_empty() && part.chars().all(|c| c.is_ascii_digit()))
}

pub(super) fn valid_branch(value: &str) -> bool {
    value.starts_with("codex/")
        && value[6..]
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
}

pub(super) fn valid_github_issue_url(value: &str) -> bool {
    let parts = value.split('/').collect::<Vec<_>>();
    parts.len() >= 7
        && parts[0] == "https:"
        && parts[2] == "github.com"
        && parts[5] == "issues"
        && parts[6].chars().all(|c| c.is_ascii_digit())
}

pub(super) fn valid_github_pr_url(value: &str) -> bool {
    let parts = value.split('/').collect::<Vec<_>>();
    parts.len() >= 7
        && parts[0] == "https:"
        && parts[2] == "github.com"
        && parts[5] == "pull"
        && parts[6].chars().all(|c| c.is_ascii_digit())
}

pub(super) fn valid_reference(value: &str) -> bool {
    value.starts_with("http://")
        || value.starts_with("https://")
        || value
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '/' | '-'))
}

pub(super) fn valid_iso8601_datetime(value: &str) -> bool {
    chrono::DateTime::parse_from_rfc3339(value)
        .map(|dt| dt.to_rfc3339_opts(chrono::SecondsFormat::Secs, true) == value)
        .unwrap_or(false)
}

pub(super) fn is_normalized_slug(value: &str) -> bool {
    !value.is_empty()
        && value
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
        && !value.starts_with('-')
        && !value.ends_with('-')
        && !value.contains("--")
}

pub(super) fn is_repo_review_finding_title(line: &str) -> bool {
    let trimmed = line.trim_start();
    let Some((num, rest)) = trimmed.split_once(". ") else {
        return false;
    };
    if !num.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    matches!(
        rest.get(0..4),
        Some("[P1]") | Some("[P2]") | Some("[P3]") | Some("[P4]") | Some("[P5]")
    )
}

pub(super) fn repo_review_finding_sort_key(line: &str) -> (u8, String) {
    let trimmed = line.trim_start();
    let sev = trimmed
        .split("[P")
        .nth(1)
        .and_then(|rest| rest.chars().next())
        .and_then(|c| c.to_digit(10))
        .unwrap_or(9) as u8;
    (sev, trimmed.to_string())
}

pub(super) fn mapping_contains(mapping: &Mapping, key: &str) -> bool {
    mapping.contains_key(Value::String(key.to_string()))
}

pub(super) fn mapping_string(mapping: &Mapping, key: &str) -> Option<String> {
    mapping
        .get(Value::String(key.to_string()))
        .and_then(|value| match value {
            Value::String(v) => Some(v.clone()),
            Value::Number(v) => Some(v.to_string()),
            _ => None,
        })
}

pub(super) fn mapping_bool(mapping: &Mapping, key: &str) -> Option<bool> {
    mapping
        .get(Value::String(key.to_string()))
        .and_then(Value::as_bool)
}

pub(super) fn mapping_mapping<'a>(mapping: &'a Mapping, key: &str) -> Result<&'a Mapping> {
    mapping
        .get(Value::String(key.to_string()))
        .and_then(Value::as_mapping)
        .ok_or_else(|| anyhow!("missing required key: {key}"))
}

pub(super) fn mapping_seq_len(mapping: &Mapping, key: &str) -> usize {
    mapping
        .get(Value::String(key.to_string()))
        .and_then(Value::as_sequence)
        .map(Vec::len)
        .unwrap_or(0)
}

pub(super) fn ensure_bool(mapping: &Mapping, key: &str, message: &str) -> Result<bool> {
    mapping_bool(mapping, key).ok_or_else(|| anyhow!(message.to_string()))
}
