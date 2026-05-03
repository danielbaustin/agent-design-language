use std::collections::BTreeSet;
use std::path::Path;

use anyhow::{ensure, bail, Context};
use super::code_review_args::validate_include_file;
use super::code_review_helpers::{git_show_file_prefix, safe_read_worktree_file, truncate};
use super::code_review_types::{
    CODE_REVIEW_PACKET_SCHEMA, CodeReviewArgs, DiffHunk, DiffSummary, FileContext,
    RepoSliceManifest, RedactionStatus, ReviewPacket, ValidationEvidence, VisibilityMode,
    MAX_REVIEW_CONTEXT_FILES, MAX_REVIEW_DIFF_FILES,
};

pub(crate) fn build_packet(root: &Path, args: &CodeReviewArgs) -> anyhow::Result<ReviewPacket> {
    let branch = super::code_review_helpers::git_output(root, &["rev-parse", "--abbrev-ref", "HEAD"])
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
        secret_like_values_present: super::super::common::contains_secret_like_token(&packet_text),
    };
    Ok(ReviewPacket {
        schema_version: CODE_REVIEW_PACKET_SCHEMA,
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

pub(crate) fn changed_files(root: &Path, args: &CodeReviewArgs) -> anyhow::Result<Vec<String>> {
    let mut files = BTreeSet::new();
    if let Ok(output) = super::code_review_helpers::git_output(
        root,
        &[
            "diff",
            "--name-only",
            "--end-of-options",
            &format!("{}...{}", args.base_ref, args.head_ref),
        ],
    ) {
        for line in output.lines().map(str::trim).filter(|line| !line.is_empty()) {
            files.insert(validate_include_file(line)?);
        }
    }
    if args.include_working_tree {
        let output = super::code_review_helpers::git_output(root, &["diff", "--name-only"])?;
        for line in output.lines().map(str::trim).filter(|line| !line.is_empty()) {
            files.insert(validate_include_file(line)?);
        }
    }
    if !args.include_files.is_empty() {
        for file in &args.include_files {
            ensure!(!files.contains(file), "--file '{file}' is not in the changed file set for the selected base/head or working tree");
        }
        return Ok(args.include_files.clone());
    }
    Ok(files.into_iter().collect())
}

pub(crate) fn diff_hunks(
    root: &Path,
    args: &CodeReviewArgs,
    files: &[String],
) -> anyhow::Result<Vec<DiffHunk>> {
    let mut hunks = Vec::new();
    for file in files.iter().take(MAX_REVIEW_DIFF_FILES) {
        let committed_diff = super::code_review_helpers::git_output(
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
            super::code_review_helpers::git_output(root, &["diff", "--cached", "--", file]).unwrap_or_default()
        } else {
            String::new()
        };
        let unstaged_diff = if args.include_working_tree {
            super::code_review_helpers::git_output(root, &["diff", "--", file]).unwrap_or_default()
        } else {
            String::new()
        };
        let mut diff_parts = Vec::new();
        if !committed_diff.trim().is_empty() {
            diff_parts.push(format!(
                "--- committed diff ({}...{}) ---\\n{}",
                args.base_ref, args.head_ref, committed_diff
            ));
        }
        if !staged_diff.trim().is_empty() {
            diff_parts.push(format!("--- staged working tree diff ---\\n{staged_diff}"));
        }
        if !unstaged_diff.trim().is_empty() {
            diff_parts.push(format!("--- unstaged working tree diff ---\\n{unstaged_diff}"));
        }
        let diff = diff_parts.join("\\n");
        let (diff_excerpt, truncated) = truncate(&diff, args.max_diff_bytes);
        hunks.push(DiffHunk {
            file: file.clone(),
            diff_excerpt,
            truncated,
        });
    }
    Ok(hunks)
}

pub(crate) fn file_contexts(
    root: &Path,
    args: &CodeReviewArgs,
    files: &[String],
) -> anyhow::Result<Vec<FileContext>> {
    let mut contexts = Vec::new();
    let max_read_bytes = args.max_diff_bytes.saturating_add(1);
    let canonical_root = std::fs::canonicalize(root).context("canonicalize repo root")?;
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
    let output = std::process::Command::new("git")
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

pub(crate) fn review_packet_contains_absolute_host_path(hunks: &[DiffHunk], contexts: &[FileContext]) -> bool {
    hunks.iter()
        .any(|hunk| contains_review_absolute_host_path(&hunk.diff_excerpt))
        || contexts
            .iter()
            .any(|context| contains_review_absolute_host_path(&context.current_excerpt))
}

pub(crate) fn contains_review_absolute_host_path(text: &str) -> bool {
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
