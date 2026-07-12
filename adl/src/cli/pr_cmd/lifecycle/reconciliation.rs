use super::super::*;
use chrono::{DateTime, FixedOffset, Utc};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct CloseoutCompletionMarker {
    schema: String,
    issue: u32,
    scope: String,
    slug: String,
    canonical_sor: String,
    status: String,
    #[serde(default)]
    validated_at: Option<String>,
}

const CLOSEOUT_MARKER_ROLLOUT_CUTOFF_RFC3339: &str = "2026-07-06T00:00:00Z";

fn closeout_completion_marker_path(repo_root: &Path, issue: u32) -> PathBuf {
    repo_root
        .join(".adl")
        .join("logs")
        .join("closeout-complete")
        .join(format!("issue-{issue}.json"))
}

fn repo_relative_path(repo_root: &Path, path: &Path) -> String {
    path.strip_prefix(repo_root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

fn write_validated_closeout_marker(
    repo_root: &Path,
    issue_ref: &IssueRef,
    canonical_output: &Path,
) -> Result<()> {
    let marker_path = closeout_completion_marker_path(repo_root, issue_ref.issue_number());
    if let Some(parent) = marker_path.parent() {
        fs::create_dir_all(parent).with_context(|| {
            format!(
                "closeout: failed to create closeout marker directory '{}'",
                parent.display()
            )
        })?;
    }
    let marker = CloseoutCompletionMarker {
        schema: "adl.closeout.complete.v1".to_string(),
        issue: issue_ref.issue_number(),
        scope: issue_ref.scope().to_string(),
        slug: issue_ref.slug().to_string(),
        canonical_sor: repo_relative_path(repo_root, canonical_output),
        status: "validated".to_string(),
        validated_at: Some(Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true)),
    };
    let json = serde_json::to_string_pretty(&marker)?;
    fs::write(&marker_path, format!("{json}\n")).with_context(|| {
        format!(
            "closeout: failed to write closeout marker '{}'",
            marker_path.display()
        )
    })?;
    Ok(())
}

pub(crate) fn validated_closeout_marker_is_current(
    repo_root: &Path,
    issue_ref: &IssueRef,
) -> Result<bool> {
    let marker_path = closeout_completion_marker_path(repo_root, issue_ref.issue_number());
    if !marker_path.is_file() {
        return Ok(false);
    }
    let marker: CloseoutCompletionMarker = serde_json::from_str(
        &fs::read_to_string(&marker_path)
            .with_context(|| format!("watch: failed to read '{}'", marker_path.display()))?,
    )
    .with_context(|| {
        format!(
            "watch: failed to parse closeout marker '{}'",
            marker_path.display()
        )
    })?;
    if marker.schema != "adl.closeout.complete.v1"
        || marker.status != "validated"
        || marker.issue != issue_ref.issue_number()
        || marker.scope != issue_ref.scope()
        || marker.slug != issue_ref.slug()
    {
        return Ok(false);
    }
    let canonical_output = repo_root.join(&marker.canonical_sor);
    ensure_closed_completed_issue_bundle_truth(repo_root, issue_ref, &canonical_output)?;
    closeout_worktree_settlement_is_current(repo_root, issue_ref, &canonical_output)
}

pub(crate) fn validated_closeout_state_is_current(
    repo_root: &Path,
    issue_ref: &IssueRef,
) -> Result<bool> {
    if validated_closeout_marker_is_current(repo_root, issue_ref).unwrap_or(false) {
        return Ok(true);
    }
    let canonical_output = issue_ref.task_bundle_output_path(repo_root);
    ensure_closed_completed_issue_bundle_truth(repo_root, issue_ref, &canonical_output)?;
    closeout_worktree_settlement_is_current(repo_root, issue_ref, &canonical_output)
}

pub(crate) fn validated_closeout_state_matches_linked_pr(
    repo_root: &Path,
    issue_ref: &IssueRef,
    linked_pr_url: &str,
    linked_pr_number: u32,
    issue_closed_at: Option<&str>,
) -> Result<bool> {
    if !validated_closeout_state_is_current(repo_root, issue_ref)? {
        return Ok(false);
    }
    let canonical_output = issue_ref.task_bundle_output_path(repo_root);
    let text = fs::read_to_string(&canonical_output).with_context(|| {
        format!(
            "watch: failed to read canonical SOR '{}'",
            canonical_output.display()
        )
    })?;
    let pr_matches = closeout_sor_names_linked_pr(&text, linked_pr_url, linked_pr_number);
    if !pr_matches {
        return Ok(false);
    }
    closeout_marker_timestamp_covers_issue_closed_at(repo_root, issue_ref, &text, issue_closed_at)
}

fn closeout_sor_names_linked_pr(text: &str, linked_pr_url: &str, _linked_pr_number: u32) -> bool {
    let expected_url = linked_pr_url.trim_end_matches('/');
    text.lines().any(|line| {
        let line = line.trim().trim_start_matches("- ").trim();
        let Some((key, value)) = line.split_once(':') else {
            return false;
        };
        let key = key.trim().trim_matches('`');
        if !matches!(key, "pr_url" | "PR URL" | "Pull Request URL") {
            return false;
        }
        let value = value.trim().trim_matches('`').trim_matches('"');
        value.trim_end_matches('/') == expected_url
    })
}

fn closeout_worktree_settlement_is_current(
    repo_root: &Path,
    issue_ref: &IssueRef,
    canonical_output: &Path,
) -> Result<bool> {
    let worktree_path = issue_ref.default_worktree_path(
        repo_root,
        std::env::var_os("ADL_WORKTREE_ROOT")
            .map(PathBuf::from)
            .as_deref(),
    );
    if !worktree_path.is_dir() {
        return Ok(true);
    }

    let text = fs::read_to_string(canonical_output).with_context(|| {
        format!(
            "watch: failed to read canonical SOR '{}'",
            canonical_output.display()
        )
    })?;
    let worktree_prune_result = line_value_after_prefix(&text, "- Worktree prune result:");
    let worktree_only_paths = line_value_after_prefix(&text, "- Worktree-only paths remaining:");
    let retained_is_recorded = worktree_prune_result
        .as_deref()
        .unwrap_or_default()
        .starts_with("retained_with_reason:")
        || worktree_prune_result
            .as_deref()
            .unwrap_or_default()
            .starts_with("blocked_dirty:");
    let remaining_paths_record_retention = worktree_only_paths
        .as_deref()
        .unwrap_or_default()
        .starts_with("issue worktree retained:");

    Ok(retained_is_recorded && remaining_paths_record_retention)
}

fn closeout_sor_pr_url(text: &str) -> Option<String> {
    for line in text.lines() {
        let line = line.trim().trim_start_matches("- ").trim();
        let Some((key, value)) = line.split_once(':') else {
            continue;
        };
        let key = key.trim().trim_matches('`');
        if !matches!(key, "pr_url" | "PR URL" | "Pull Request URL") {
            continue;
        }
        let value = value.trim().trim_matches('`').trim_matches('"');
        if !value.is_empty() {
            return Some(value.to_string());
        }
    }
    None
}

fn ensure_terminal_watcher_for_closeout(
    repo_root: &Path,
    issue_ref: &IssueRef,
    canonical_output: &Path,
) -> Result<()> {
    let sor_text = fs::read_to_string(canonical_output).with_context(|| {
        format!(
            "closeout: failed to read canonical SOR for watcher terminal disposition '{}'",
            canonical_output.display()
        )
    })?;
    let pr_url = closeout_sor_pr_url(&sor_text);
    let branch = line_value_after_prefix(&sor_text, "Branch:");
    let terminal_disposition =
        match line_value_after_prefix(&sor_text, "- Integration state:").as_deref() {
            Some("merged") => "green_merged",
            _ => "closed_without_merge",
        };
    super::super::github::ensure_issue_watcher_terminal_disposition(
        repo_root,
        issue_ref.issue_number(),
        pr_url.as_deref(),
        terminal_disposition,
        branch.as_deref(),
    )
}

fn closeout_marker_timestamp_covers_issue_closed_at(
    repo_root: &Path,
    issue_ref: &IssueRef,
    sor_text: &str,
    issue_closed_at: Option<&str>,
) -> Result<bool> {
    let marker_path = closeout_completion_marker_path(repo_root, issue_ref.issue_number());
    let Some(issue_closed_at) = issue_closed_at else {
        return Ok(false);
    };
    let issue_closed_at = parse_rfc3339_markdown_value(issue_closed_at)?;
    if !marker_path.is_file() {
        return legacy_or_sor_timestamp_covers_issue_closed_at(sor_text, issue_closed_at);
    }
    let marker: CloseoutCompletionMarker = serde_json::from_str(
        &fs::read_to_string(&marker_path)
            .with_context(|| format!("watch: failed to read '{}'", marker_path.display()))?,
    )
    .with_context(|| {
        format!(
            "watch: failed to parse closeout marker '{}'",
            marker_path.display()
        )
    })?;
    let Some(validated_at) = marker.validated_at.as_deref() else {
        return legacy_or_sor_timestamp_covers_issue_closed_at(sor_text, issue_closed_at);
    };
    let validated_at = parse_rfc3339_markdown_value(validated_at)?;
    Ok(validated_at >= issue_closed_at)
}

fn legacy_or_sor_timestamp_covers_issue_closed_at(
    sor_text: &str,
    issue_closed_at: DateTime<FixedOffset>,
) -> Result<bool> {
    let marker_rollout_cutoff =
        parse_rfc3339_markdown_value(CLOSEOUT_MARKER_ROLLOUT_CUTOFF_RFC3339)?;
    if issue_closed_at < marker_rollout_cutoff {
        return Ok(true);
    }
    if let Some(sor_timestamp) = closeout_sor_terminal_timestamp(sor_text)? {
        return Ok(sor_timestamp >= issue_closed_at);
    }
    Ok(false)
}

fn closeout_sor_terminal_timestamp(text: &str) -> Result<Option<DateTime<FixedOffset>>> {
    for line in text.lines() {
        let line = line.trim().trim_start_matches("- ").trim();
        let Some((key, value)) = line.split_once(':') else {
            continue;
        };
        let key = key.trim();
        if matches!(key, "End Time" | "Generated") {
            return Ok(Some(parse_rfc3339_markdown_value(value)?));
        }
    }
    Ok(None)
}

fn parse_rfc3339_markdown_value(value: &str) -> Result<DateTime<FixedOffset>> {
    let value = value.trim().trim_matches('`');
    DateTime::parse_from_rfc3339(value)
        .with_context(|| format!("watch: failed to parse RFC3339 timestamp '{value}'"))
}

fn reconcile_closed_completed_issue_bundle_with_recovery_sources(
    repo_root: &Path,
    issue_ref: &IssueRef,
    canonical_output: &Path,
    recovery_bundles: &[PathBuf],
) -> Result<()> {
    let bundle_dir = issue_ref.task_bundle_dir_path(repo_root);
    if let Some(parent) = bundle_dir.parent() {
        fs::create_dir_all(parent).with_context(|| {
            format!(
                "doctor: failed to create canonical task bundle parent '{}'",
                parent.display()
            )
        })?;
    }

    let mut duplicates = Vec::new();
    for recovery_bundle in recovery_bundles {
        if recovery_bundle.is_dir() && !duplicates.iter().any(|path| path == recovery_bundle) {
            duplicates.push(recovery_bundle.clone());
        }
    }
    for duplicate in matching_task_bundle_dirs(repo_root, issue_ref)? {
        if duplicate.is_dir() && !duplicates.iter().any(|path| path == &duplicate) {
            duplicates.push(duplicate);
        }
    }
    if !bundle_dir.exists() {
        fs::create_dir_all(&bundle_dir).with_context(|| {
            format!(
                "doctor: failed to create canonical task bundle '{}'",
                bundle_dir.display()
            )
        })?;
    }

    for relative in ["stp.md", "sip.md", "spp.md", "srp.md", "sor.md"] {
        let canonical_path = bundle_dir.join(relative);
        if !canonical_bundle_path_needs_recovery(&canonical_path, relative)? {
            continue;
        }
        for duplicate in &duplicates {
            if *duplicate == bundle_dir {
                continue;
            }
            let candidate = duplicate.join(relative);
            if recovery_candidate_is_usable(&candidate, relative)? {
                fs::copy(&candidate, &canonical_path).with_context(|| {
                    format!(
                        "doctor: failed to restore canonical bundle file '{}' from duplicate '{}'",
                        canonical_path.display(),
                        candidate.display()
                    )
                })?;
                break;
            }
        }
    }

    if !ensure_nonempty_file_path(canonical_output)? {
        for duplicate in &duplicates {
            if *duplicate == bundle_dir {
                continue;
            }
            let candidate = duplicate.join("sor.md");
            if ensure_nonempty_file_path(&candidate)? {
                fs::copy(&candidate, canonical_output).with_context(|| {
                    format!(
                        "doctor: failed to restore canonical sor from duplicate '{}'",
                        candidate.display()
                    )
                })?;
                break;
            }
        }

        if !ensure_nonempty_file_path(canonical_output)? {
            let cards_root = resolve_cards_root(repo_root, None);
            let review_output = card_output_path(&cards_root, issue_ref.issue_number());
            if ensure_nonempty_file_path(&review_output)? {
                fs::copy(&review_output, canonical_output).with_context(|| {
                    format!(
                        "doctor: failed to restore canonical sor from review card '{}'",
                        review_output.display()
                    )
                })?;
            }
        }
    }

    if !ensure_nonempty_file_path(canonical_output)? {
        bail!(
            "doctor: closed issue is missing canonical sor: {}",
            canonical_output.display()
        );
    }

    ensure_canonical_output_is_local_only(
        repo_root,
        canonical_output,
        "doctor: canonical .adl output surfaces must remain local-only during closed-issue reconciliation",
    )?;
    normalize_closed_completed_stp(&issue_ref.task_bundle_stp_path(repo_root))?;
    normalize_closed_completed_sip(&issue_ref.task_bundle_input_path(repo_root), issue_ref)?;
    normalize_closed_completed_output_card(canonical_output)?;
    validate_closed_completed_stp(repo_root, &issue_ref.task_bundle_stp_path(repo_root))?;
    validate_closed_completed_sip(repo_root, &issue_ref.task_bundle_input_path(repo_root))?;
    validate_completed_sor(repo_root, canonical_output)?;

    let cards_root = resolve_cards_root(repo_root, None);
    let review_output = card_output_path(&cards_root, issue_ref.issue_number());
    ensure_symlink(&review_output, canonical_output)?;
    Ok(())
}

pub(crate) fn closeout_closed_completed_issue_bundle(
    repo_root: &Path,
    primary_root: &Path,
    issue_ref: &IssueRef,
    canonical_output: &Path,
) -> Result<()> {
    let worktree_path = issue_ref.default_worktree_path(
        primary_root,
        std::env::var_os("ADL_WORKTREE_ROOT")
            .map(PathBuf::from)
            .as_deref(),
    );
    let worktree_is_dirty =
        worktree_path.is_dir() && has_uncommitted_or_untracked_changes(&worktree_path)?;
    let recovery_bundles = if worktree_is_dirty {
        Vec::new()
    } else {
        vec![issue_ref.task_bundle_dir_path(&worktree_path)]
    };
    if let Err(error) = reconcile_closed_completed_issue_bundle_with_recovery_sources(
        primary_root,
        issue_ref,
        canonical_output,
        &recovery_bundles,
    ) {
        if worktree_is_dirty {
            if canonical_output.is_file() {
                let name = super::cleanup::worktree_display_name(&worktree_path);
                super::cleanup::record_worktree_prune_result(
                    canonical_output,
                    &format!("blocked_dirty: retained {name}"),
                )?;
                super::cleanup::replace_worktree_only_paths_remaining(
                    canonical_output,
                    &format!("issue worktree retained: {name}"),
                )?;
            }
            return Err(error).with_context(|| {
                format!(
                    "closeout: dirty worktree '{}' was retained and was not used as a recovery source",
                    worktree_path.display()
                )
            });
        }
        return Err(error);
    }
    if worktree_path.is_dir() {
        super::cleanup::scrub_noncanonical_issue_bundle_residue(&worktree_path, issue_ref)?;
    }
    if worktree_is_dirty {
        let retained_result = super::cleanup::IssueWorktreePruneResult::RetainedWithReason(
            super::cleanup::worktree_display_name(&worktree_path),
        );
        super::cleanup::record_worktree_prune_result(
            canonical_output,
            &retained_result.card_value(),
        )?;
        super::cleanup::replace_worktree_only_paths_remaining(
            canonical_output,
            &format!(
                "issue worktree retained: {}",
                super::cleanup::worktree_display_name(&worktree_path)
            ),
        )?;
        ensure_closed_completed_issue_bundle_truth(primary_root, issue_ref, canonical_output)
            .with_context(|| {
                format!(
                    "closeout: canonical closed-issue sor truth drift remains for issue #{} after retaining dirty stale worktree",
                    issue_ref.issue_number()
                )
            })?;
        ensure_terminal_watcher_for_closeout(primary_root, issue_ref, canonical_output)?;
        write_validated_closeout_marker(primary_root, issue_ref, canonical_output)?;
        return Ok(());
    }
    let prune_result = super::cleanup::prune_issue_worktree(repo_root, primary_root, issue_ref)?;
    super::cleanup::record_worktree_prune_result(canonical_output, &prune_result.card_value())?;
    ensure_closed_completed_issue_bundle_truth(primary_root, issue_ref, canonical_output)
        .with_context(|| {
            format!(
                "closeout: canonical closed-issue sor truth drift remains for issue #{}",
                issue_ref.issue_number()
            )
        })?;
    ensure_terminal_watcher_for_closeout(primary_root, issue_ref, canonical_output)?;
    write_validated_closeout_marker(primary_root, issue_ref, canonical_output)?;
    Ok(())
}

pub(crate) fn ensure_closed_completed_issue_bundle_truth(
    repo_root: &Path,
    issue_ref: &IssueRef,
    canonical_output: &Path,
) -> Result<()> {
    let bundle_dir = issue_ref.task_bundle_dir_path(repo_root);
    let mut sor_integration_state: Option<String> = None;

    let mut mismatches = Vec::new();
    if !bundle_dir.is_dir() {
        mismatches.push(format!(
            "missing canonical task bundle directory: {}",
            bundle_dir.display()
        ));
    }
    if !ensure_nonempty_file_path(canonical_output)? {
        mismatches.push("missing canonical sor.md".to_string());
    } else {
        let text = fs::read_to_string(canonical_output)?;
        sor_integration_state = line_value_after_prefix(&text, "- Integration state:");
        let worktree_only_paths =
            line_value_after_prefix(&text, "- Worktree-only paths remaining:");
        let worktree_prune_result = line_value_after_prefix(&text, "- Worktree prune result:");
        let branch = line_value_after_prefix(&text, "Branch:");
        let retrospective_no_branch = branch.as_deref() == Some("retrospective-no-branch");
        check_required_field(&text, "Status:", "DONE", "SOR Status", &mut mismatches);
        check_required_field(
            &text,
            "- Verification scope:",
            "main_repo",
            "SOR Verification scope",
            &mut mismatches,
        );
        match sor_integration_state.as_deref() {
            Some("merged") => {
                if retrospective_no_branch {
                    mismatches.push(
                        "SOR Integration state is 'merged' but Branch is 'retrospective-no-branch'; use 'closed_no_pr'"
                            .to_string(),
                    );
                }
            }
            Some("closed_no_pr") => {
                check_required_field(
                    &text,
                    "Branch:",
                    "retrospective-no-branch",
                    "SOR Branch for closed_no_pr",
                    &mut mismatches,
                );
            }
            Some(value) => mismatches.push(format!(
                "SOR Integration state expected 'merged' or 'closed_no_pr' but found '{value}'"
            )),
            None => mismatches.push("SOR Integration state is missing".to_string()),
        }
        match worktree_only_paths.as_deref() {
            Some("none") => {}
            Some(value) if value.starts_with("issue worktree retained: ") => {
                match worktree_prune_result.as_deref() {
                    Some(result) if result.starts_with("retained_with_reason: ") => {}
                    Some(result) => mismatches.push(format!(
                        "SOR Worktree prune result expected retained_with_reason for retained worktree but found '{result}'"
                    )),
                    None => mismatches.push(
                        "SOR Worktree prune result is missing for retained worktree".to_string(),
                    ),
                }
            }
            Some(value) => mismatches.push(format!(
                "SOR Worktree-only paths remaining expected 'none' or retained issue worktree but found '{value}'"
            )),
            None => mismatches.push("SOR Worktree-only paths remaining is missing".to_string()),
        }
    }

    let stp_path = issue_ref.task_bundle_stp_path(repo_root);
    if !ensure_nonempty_file_path(&stp_path)? {
        mismatches.push("missing canonical stp.md".to_string());
    } else {
        let text = fs::read_to_string(&stp_path)?;
        check_required_field(
            &text,
            "status:",
            "\"complete\"",
            "STP status",
            &mut mismatches,
        );
    }

    let sip_path = issue_ref.task_bundle_input_path(repo_root);
    if !ensure_nonempty_file_path(&sip_path)? {
        mismatches.push("missing canonical sip.md".to_string());
    } else {
        let text = fs::read_to_string(&sip_path)?;
        let expected_branch = issue_ref.branch_name("codex");
        match sor_integration_state.as_deref() {
            Some("merged") => check_required_field(
                &text,
                "Branch:",
                &expected_branch,
                "SIP Branch",
                &mut mismatches,
            ),
            Some("closed_no_pr") => {
                let branch = line_value_after_prefix(&text, "Branch:").unwrap_or_default();
                if branch != expected_branch && branch != "retrospective-no-branch" {
                    mismatches.push(format!(
                        "SIP Branch expected '{}' or 'retrospective-no-branch' but found '{}'",
                        expected_branch, branch
                    ));
                }
            }
            _ => check_required_field(
                &text,
                "Branch:",
                &expected_branch,
                "SIP Branch",
                &mut mismatches,
            ),
        }
        if text.contains("This issue is not started yet")
            || text.contains("before execution is bound")
            || text.contains("until `pr run` binds the branch and worktree")
        {
            mismatches.push("SIP still contains pre-run lifecycle wording".to_string());
        }
    }

    let srp_path = issue_ref.task_bundle_review_policy_path(repo_root);
    if !ensure_nonempty_file_path(&srp_path)? {
        mismatches.push("missing canonical srp.md".to_string());
    } else if !srp_has_final_review_results_text(&fs::read_to_string(&srp_path)?)? {
        mismatches.push(
            "SRP review_results must record final findings_status/recommended_outcome truth for closed issues"
                .to_string(),
        );
    }

    if !mismatches.is_empty() {
        bail!(
            "canonical closed-issue sor truth drift at {}: {}",
            canonical_output.display(),
            mismatches.join("; ")
        );
    }
    Ok(())
}

fn canonical_bundle_path_needs_recovery(path: &Path, relative: &str) -> Result<bool> {
    if !ensure_nonempty_file_path(path)? {
        return Ok(true);
    }
    match relative {
        "srp.md" => Ok(!srp_has_final_review_results_text(&fs::read_to_string(
            path,
        )?)?),
        "sor.md" => Ok(!sor_has_terminal_closeout_truth_text(&fs::read_to_string(
            path,
        )?)),
        _ => Ok(false),
    }
}

fn recovery_candidate_is_usable(path: &Path, relative: &str) -> Result<bool> {
    if !ensure_nonempty_file_path(path)? {
        return Ok(false);
    }
    match relative {
        "srp.md" => srp_has_final_review_results_text(&fs::read_to_string(path)?),
        "sor.md" => Ok(sor_has_terminal_closeout_truth_text(&fs::read_to_string(
            path,
        )?)),
        _ => Ok(true),
    }
}

pub(super) fn srp_has_final_review_results_text(text: &str) -> Result<bool> {
    let Some(front_matter) = markdown_front_matter_local(text) else {
        return Ok(false);
    };
    let yaml: serde_yaml::Value = match serde_yaml::from_str(front_matter) {
        Ok(value) => value,
        Err(_) => return Ok(false),
    };
    let Some(mapping) = yaml.as_mapping() else {
        return Ok(false);
    };
    let Some(review_results) = mapping
        .get(serde_yaml::Value::String("review_results".to_string()))
        .and_then(serde_yaml::Value::as_mapping)
    else {
        return Ok(false);
    };
    let findings_status = yaml_mapping_string_local(review_results, "findings_status");
    let recommended_outcome = yaml_mapping_string_local(review_results, "recommended_outcome");
    Ok(matches!(
        findings_status.as_deref(),
        Some(
            "no_findings"
                | "findings_present"
                | "review_unavailable"
                | "review_timeout"
                | "review_cancelled"
                | "review_failed"
        )
    ) && matches!(
        recommended_outcome.as_deref(),
        Some("pass" | "block" | "needs_followup")
    ))
}

fn markdown_front_matter_local(text: &str) -> Option<&str> {
    let rest = text.strip_prefix("---\n")?;
    let end = rest.find("\n---")?;
    Some(&rest[..end])
}

fn yaml_mapping_string_local(mapping: &serde_yaml::Mapping, key: &str) -> Option<String> {
    mapping
        .get(serde_yaml::Value::String(key.to_string()))
        .and_then(serde_yaml::Value::as_str)
        .map(|value| {
            value
                .trim()
                .trim_matches('"')
                .trim_matches('\'')
                .to_ascii_lowercase()
        })
}

fn sor_has_terminal_closeout_truth_text(text: &str) -> bool {
    let status = line_value_after_prefix(text, "Status:").unwrap_or_default();
    let integration_state =
        line_value_after_prefix(text, "- Integration state:").unwrap_or_default();
    let result = line_value_after_prefix(text, "- Result:").unwrap_or_default();
    let worktree_only =
        line_value_after_prefix(text, "- Worktree-only paths remaining:").unwrap_or_default();
    let worktree_prune_result =
        line_value_after_prefix(text, "- Worktree prune result:").unwrap_or_default();
    let terminal_worktree_truth = worktree_only == "none"
        || (worktree_only.starts_with("issue worktree retained: ")
            && worktree_prune_result.starts_with("retained_with_reason: "));
    ["merged", "closed_no_pr"].contains(&integration_state.as_str())
        && matches!(
            (status.as_str(), result.as_str()),
            ("DONE", "PASS") | ("FAILED", "FAIL")
        )
        && terminal_worktree_truth
        && text.contains("## Validation")
}

fn check_required_field(
    text: &str,
    prefix: &str,
    expected: &str,
    label: &str,
    mismatches: &mut Vec<String>,
) {
    match text
        .lines()
        .find(|line| line.starts_with(prefix))
        .map(|line| line[prefix.len()..].trim().to_string())
    {
        Some(actual) if actual == expected => {}
        Some(actual) => mismatches.push(format!(
            "{} expected '{}' but found '{}'",
            label, expected, actual
        )),
        None => mismatches.push(format!("{} is missing", label)),
    }
}

fn line_value_after_prefix(text: &str, prefix: &str) -> Option<String> {
    text.lines().find_map(|line| {
        line.strip_prefix(prefix)
            .map(|value| value.trim().to_string())
    })
}

pub(crate) fn matching_task_bundle_dirs(
    repo_root: &Path,
    issue_ref: &IssueRef,
) -> Result<Vec<PathBuf>> {
    let tasks_dir = repo_root.join(".adl").join(issue_ref.scope()).join("tasks");
    if !tasks_dir.is_dir() {
        return Ok(Vec::new());
    }
    let prefix = format!("{}__", issue_ref.task_issue_id());
    let mut matches = Vec::new();
    for entry in fs::read_dir(&tasks_dir)? {
        let entry = entry?;
        if !entry.file_type()?.is_dir() {
            continue;
        }
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if name.starts_with(&prefix) {
            matches.push(entry.path());
        }
    }
    matches.sort();
    Ok(matches)
}

pub(crate) fn normalize_closed_completed_output_card(path: &Path) -> Result<()> {
    let mut text = fs::read_to_string(path)?;
    replace_field_line_in_text(&mut text, "Status", "DONE");
    let branch = line_value_after_prefix(&text, "Branch:");
    if branch.as_deref() == Some("retrospective-no-branch") {
        replace_first_exact_line(
            &mut text,
            "- Integration state: worktree_only | pr_open | merged",
            "- Integration state: closed_no_pr",
        );
        replace_first_exact_line(
            &mut text,
            "- Integration state: worktree_only",
            "- Integration state: closed_no_pr",
        );
        replace_first_exact_line(
            &mut text,
            "- Integration state: pr_open",
            "- Integration state: closed_no_pr",
        );
        replace_first_exact_line(
            &mut text,
            "- Integration state: merged",
            "- Integration state: closed_no_pr",
        );
    } else {
        replace_first_exact_line(
            &mut text,
            "- Integration state: worktree_only | pr_open | merged",
            "- Integration state: merged",
        );
        replace_first_exact_line(
            &mut text,
            "- Integration state: worktree_only",
            "- Integration state: merged",
        );
        replace_first_exact_line(
            &mut text,
            "- Integration state: pr_open",
            "- Integration state: merged",
        );
    }
    replace_first_exact_line(
        &mut text,
        "- Verification scope: worktree | pr_branch | main_repo",
        "- Verification scope: main_repo",
    );
    replace_first_exact_line(
        &mut text,
        "- Verification scope: worktree",
        "- Verification scope: main_repo",
    );
    replace_first_exact_line(
        &mut text,
        "- Verification scope: pr_branch",
        "- Verification scope: main_repo",
    );
    replace_first_prefixed_line(
        &mut text,
        "- Worktree-only paths remaining:",
        "- Worktree-only paths remaining: none",
    );
    fs::write(path, text)?;
    Ok(())
}

pub(crate) fn normalize_closed_completed_stp(path: &Path) -> Result<()> {
    let mut text = fs::read_to_string(path)?;
    replace_field_line_in_text(&mut text, "status", "\"complete\"");
    fs::write(path, text)?;
    Ok(())
}

pub(crate) fn normalize_closed_completed_sip(path: &Path, issue_ref: &IssueRef) -> Result<()> {
    let mut text = fs::read_to_string(path)?;
    let branch = line_value_after_prefix(&text, "Branch:");
    if branch.as_deref() != Some("retrospective-no-branch") {
        replace_field_line_in_text(&mut text, "Branch", &issue_ref.branch_name("codex"));
    }
    replace_first_exact_line(&mut text, "- PR: none", "- PR:");
    replace_first_exact_line(
        &mut text,
        "- This issue is not started yet; do not assume a branch or worktree already exists.",
        "- This issue is closed/completed; implementation branch/worktree lifecycle is finished.",
    );
    replace_first_exact_line(
        &mut text,
        "- Do not run `pr start`; use the current issue-mode `pr run` flow only if execution later becomes necessary.",
        "- Do not run `pr start`; the issue has already completed its lifecycle.",
    );
    replace_first_exact_line(
        &mut text,
        "Prepare the linked issue prompt and review surfaces for truthful pre-run review before execution is bound.",
        "Preserve the closed/completed issue prompt and local card truth after closeout.",
    );
    replace_first_exact_line(
        &mut text,
        "- Preserve truthful lifecycle state until `pr run` binds the branch and worktree.",
        "- Preserve truthful closed/completed lifecycle state after merge and closeout.",
    );
    replace_first_exact_line(
        &mut text,
        "- The card bundle does not imply a branch or worktree exists before `pr run`.",
        "- The card bundle records the completed issue branch and no longer claims pre-run state.",
    );
    fs::write(path, text)?;
    Ok(())
}

fn validate_closed_completed_stp(repo_root: &Path, stp_path: &Path) -> Result<()> {
    let validator = repo_root.join("adl/tools/validate_structured_prompt.sh");
    run_status(
        "bash",
        &[
            path_str(&validator)?,
            "--type",
            "stp",
            "--input",
            path_str(stp_path)?,
        ],
    )
    .with_context(|| {
        format!(
            "closeout: stp failed closed/completed validation: {}",
            stp_path.display()
        )
    })
}

fn validate_closed_completed_sip(repo_root: &Path, sip_path: &Path) -> Result<()> {
    let validator = repo_root.join("adl/tools/validate_structured_prompt.sh");
    run_status(
        "bash",
        &[
            path_str(&validator)?,
            "--type",
            "sip",
            "--input",
            path_str(sip_path)?,
        ],
    )
    .with_context(|| {
        format!(
            "closeout: sip failed closed/completed validation: {}",
            sip_path.display()
        )
    })
}
pub(crate) fn ensure_canonical_output_is_local_only(
    repo_root: &Path,
    canonical_output: &Path,
    context: &str,
) -> Result<()> {
    let Ok(repo_relative) = canonical_output.strip_prefix(repo_root) else {
        return Ok(());
    };
    let repo_relative = repo_relative.to_string_lossy().into_owned();
    let status = Command::new("git")
        .args([
            "-C",
            path_str(repo_root)?,
            "ls-files",
            "--error-unmatch",
            "--",
            &repo_relative,
        ])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .context("failed to spawn 'git'")?;
    if status.success() {
        bail!(
            "{context}: '{}' is still tracked in git. Untrack canonical .adl issue surfaces before lifecycle normalization.",
            repo_relative
        );
    }
    Ok(())
}

fn replace_field_line_in_text(text: &mut String, label: &str, value: &str) {
    let prefix = format!("{label}:");
    let mut out = Vec::new();
    for line in text.lines() {
        if line.starts_with(&prefix) {
            out.push(format!("{prefix} {value}"));
        } else {
            out.push(line.to_string());
        }
    }
    *text = out.join("\n");
    if !text.ends_with('\n') {
        text.push('\n');
    }
}

fn replace_first_exact_line(text: &mut String, from: &str, to: &str) {
    let mut out = Vec::new();
    let mut replaced = false;
    for line in text.lines() {
        if !replaced && line == from {
            out.push(to.to_string());
            replaced = true;
        } else {
            out.push(line.to_string());
        }
    }
    *text = out.join("\n");
    if !text.ends_with('\n') {
        text.push('\n');
    }
}

pub(crate) fn replace_first_prefixed_line(text: &mut String, prefix: &str, to: &str) {
    let mut out = Vec::new();
    let mut replaced = false;
    for line in text.lines() {
        if !replaced && line.starts_with(prefix) {
            out.push(to.to_string());
            replaced = true;
        } else {
            out.push(line.to_string());
        }
    }
    *text = out.join("\n");
    if !text.ends_with('\n') {
        text.push('\n');
    }
}

pub(crate) fn replace_or_insert_after_prefixed_line(
    text: &mut String,
    prefix: &str,
    insert_prefix: &str,
    to: &str,
) {
    let mut out = Vec::new();
    let mut replaced = false;
    let mut inserted = false;
    for line in text.lines() {
        if line.starts_with(insert_prefix) {
            if !replaced {
                out.push(to.to_string());
                replaced = true;
            }
            continue;
        }
        out.push(line.to_string());
        if !inserted && line.starts_with(prefix) && !replaced {
            out.push(to.to_string());
            inserted = true;
            replaced = true;
        }
    }
    if !replaced {
        out.push(to.to_string());
    }
    *text = out.join("\n");
    if !text.ends_with('\n') {
        text.push('\n');
    }
}
