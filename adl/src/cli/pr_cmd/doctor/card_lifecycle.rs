use std::fs;
use std::path::Path;

use super::*;
use crate::cli::pr_cmd_cards::StructuredBundlePaths;

pub(super) fn build_doctor_card_lifecycle(
    repo_root: &Path,
    sip_path: &Path,
    stp_path: &Path,
    spp_path: &Path,
    vpp_path: &Path,
    srp_path: &Path,
    sor_path: &Path,
) -> DoctorCardLifecycleJson {
    let stages = vec![
        classify_sip_stage(repo_root, sip_path),
        classify_stp_stage(repo_root, stp_path),
        classify_spp_stage(repo_root, spp_path),
        classify_vpp_stage(repo_root, vpp_path),
        classify_srp_stage(repo_root, srp_path),
        classify_sor_stage(repo_root, sor_path),
    ];
    let next_required_stage = stages
        .iter()
        .find(|stage| !stage.complete)
        .map(|stage| stage.stage);
    let active_stage = next_required_stage.unwrap_or("SOR");
    let pr_run_readiness = if stages
        .iter()
        .filter(|stage| ["SIP", "STP", "SPP", "VPP", "SRP"].contains(&stage.stage))
        .all(|stage| stage.design_time_complete)
    {
        "ready"
    } else {
        "blocked"
    };
    let pr_finish_readiness = match (
        stages.iter().find(|stage| stage.stage == "SRP"),
        stages.iter().find(|stage| stage.stage == "SOR"),
    ) {
        (Some(srp), Some(sor)) if srp.final_ready && (sor.complete || sor.final_ready) => "ready",
        (Some(srp), Some(sor)) if srp.state == "legacy_compatible" || sor.state == "scaffold" => {
            "blocked"
        }
        _ => "blocked",
    };

    DoctorCardLifecycleJson {
        order: vec!["SIP", "STP", "SPP", "VPP", "SRP", "SOR"],
        active_stage,
        next_required_stage,
        pr_run_readiness,
        pr_finish_readiness,
        stages,
    }
}

fn classify_vpp_stage(repo_root: &Path, path: &Path) -> DoctorCardStageJson {
    let Some(text) = read_card_text(path) else {
        return missing_stage(repo_root, "VPP", path, "vpp-editor");
    };
    let status = line_value_after_prefix(&text, "status:").unwrap_or_default();
    if !design_time_card_status_allows_execution(&text) {
        return card_stage(
            repo_root,
            "VPP",
            path,
            stage_truth(
                "active",
                false,
                false,
                Some("vpp-editor"),
                "VPP card_status must be ready or approved before execution binding.",
            ),
        );
    }
    if has_generic_vpp_design_time_scaffold(&text) {
        return card_stage(
            repo_root,
            "VPP",
            path,
            stage_truth(
                "scaffold",
                false,
                false,
                Some("vpp-editor"),
                "VPP is still generic validation-planning text and needs lane truth.",
            ),
        );
    }
    if ["ready", "reviewed", "approved"].contains(&status.trim_matches('"')) {
        return card_stage(
            repo_root,
            "VPP",
            path,
            stage_truth(
                "complete",
                true,
                false,
                None,
                "VPP has ready, reviewed, or approved validation planning state.",
            ),
        );
    }
    card_stage(
        repo_root,
        "VPP",
        path,
        stage_truth(
            "active",
            false,
            false,
            Some("vpp-editor"),
            "VPP is present but not yet marked ready, reviewed, or approved.",
        ),
    )
}

pub(super) fn doctor_ready_status_for(lifecycle: &DoctorCardLifecycleJson) -> &'static str {
    if lifecycle.pr_run_readiness == "ready" {
        "PASS"
    } else {
        "BLOCK"
    }
}

fn classify_sip_stage(repo_root: &Path, path: &Path) -> DoctorCardStageJson {
    let Some(text) = read_card_text(path) else {
        return missing_stage(repo_root, "SIP", path, "sip-editor");
    };
    if !design_time_card_status_allows_execution(&text) {
        return card_stage(
            repo_root,
            "SIP",
            path,
            stage_truth(
                "active",
                false,
                false,
                Some("sip-editor"),
                "SIP card_status must be ready or approved before execution binding.",
            ),
        );
    }
    if has_generic_sip_design_time_scaffold(&text) {
        return card_stage(
            repo_root,
            "SIP",
            path,
            stage_truth(
                "scaffold",
                false,
                false,
                Some("sip-editor"),
                "SIP is still generic bootstrap text and needs issue-specific design-time intent.",
            ),
        );
    }
    card_stage(
        repo_root,
        "SIP",
        path,
        stage_truth(
            "complete",
            true,
            false,
            None,
            "SIP has issue-specific design-time intent for execution planning.",
        ),
    )
}

fn classify_stp_stage(repo_root: &Path, path: &Path) -> DoctorCardStageJson {
    let Some(text) = read_card_text(path) else {
        return missing_stage(repo_root, "STP", path, "stp-editor");
    };
    if !design_time_card_status_allows_execution(&text) {
        return card_stage(
            repo_root,
            "STP",
            path,
            stage_truth(
                "active",
                false,
                false,
                Some("stp-editor"),
                "STP card_status must be ready or approved before execution binding.",
            ),
        );
    }
    if has_complete_stp_design_time_surface(&text) {
        return card_stage(
            repo_root,
            "STP",
            path,
            stage_truth(
                "complete",
                true,
                false,
                None,
                "STP has the required issue-specific task intent, acceptance, and dependency surfaces.",
            ),
        );
    }
    card_stage(
        repo_root,
        "STP",
        path,
        stage_truth(
            "active",
            false,
            false,
            Some("stp-editor"),
            "STP exists but is not complete enough to anchor execution.",
        ),
    )
}

fn has_complete_stp_design_time_surface(text: &str) -> bool {
    const REQUIRED_HEADINGS: &[&str] = &[
        "Summary",
        "Goal",
        "Required Outcome",
        "Deliverables",
        "Acceptance Criteria",
        "Repo Inputs",
        "Dependencies",
        "Demo Expectations",
        "Non-goals",
        "Issue-Graph Notes",
        "Notes",
        "Tooling Notes",
    ];
    REQUIRED_HEADINGS
        .iter()
        .all(|heading| doctor_markdown_has_heading(text, heading))
}

fn doctor_markdown_has_heading(text: &str, heading: &str) -> bool {
    let mut in_fence = false;
    for line in text.lines() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("```") {
            in_fence = !in_fence;
            continue;
        }
        if !in_fence && line.trim_end() == format!("## {heading}") {
            return true;
        }
    }
    false
}

fn classify_spp_stage(repo_root: &Path, path: &Path) -> DoctorCardStageJson {
    let Some(text) = read_card_text(path) else {
        return missing_stage(repo_root, "SPP", path, "spp-editor");
    };
    let status = line_value_after_prefix(&text, "status:").unwrap_or_default();
    if !design_time_card_status_allows_execution(&text) {
        return card_stage(
            repo_root,
            "SPP",
            path,
            stage_truth(
                "active",
                false,
                false,
                Some("spp-editor"),
                "SPP card_status must be ready or approved before execution binding.",
            ),
        );
    }
    if has_generic_spp_design_time_scaffold(&text) {
        return card_stage(
            repo_root,
            "SPP",
            path,
            stage_truth(
                "scaffold",
                false,
                false,
                Some("spp-editor"),
                "SPP is still generic or truncated bootstrap planning text and needs issue-specific plan truth.",
            ),
        );
    }
    if ["ready", "reviewed", "approved"].contains(&status.trim_matches('"')) {
        return card_stage(
            repo_root,
            "SPP",
            path,
            stage_truth(
                "complete",
                true,
                false,
                None,
                "SPP has ready, reviewed, or approved design-time planning state.",
            ),
        );
    }
    card_stage(
        repo_root,
        "SPP",
        path,
        stage_truth(
            "active",
            false,
            false,
            Some("spp-editor"),
            "SPP is branch-bound but not yet marked reviewed or approved.",
        ),
    )
}

fn has_generic_sip_design_time_scaffold(text: &str) -> bool {
    const MARKERS: &[&str] = &[
        "Prepare the linked issue prompt and review surfaces for truthful pre-run review before execution is bound.",
        "Keep the linked issue prompt, SIP, and SOR aligned for review.",
        "The linked source issue prompt is reviewable and structurally valid.",
        "files, docs, tests, commands, schemas, and artifacts named by the linked source issue prompt",
        "derive the exact command set from the linked issue prompt",
    ];
    MARKERS.iter().any(|marker| text.contains(marker))
}

fn has_generic_spp_design_time_scaffold(text: &str) -> bool {
    const MARKERS: &[&str] = &[
        "Bootstrap-generated SPP",
        "Design-time generated SPP; review before execution",
        "Review this SPP before execution; during runtime, update it before continuing if the actual execution sequence changes.",
        "generated from source issue prompt, STP/SIP surfaces",
        "Design-time execution plan for",
        "Use dependency truth from the linked source issue prompt",
        "Use repo inputs from the linked source issue prompt",
        "Use deliverables from the linked source issue prompt",
        "Satisfy the linked source issue prompt acceptance criteria",
        "Run focused proof gates for acceptance: Satisfy the linked source issue prompt acceptance criteria",
        "Record SRP review results and SOR outcome truth",
    ];
    MARKERS.iter().any(|marker| text.contains(marker)) || has_truncation_sentinel_line(text)
}

fn has_generic_vpp_design_time_scaffold(text: &str) -> bool {
    let unresolved_planned_lane = line_value_after_prefix(text, "planned_pvf_lane:")
        .as_deref()
        .map(|value| value == "needs_planning_lane_assignment")
        .unwrap_or_else(|| {
            text.contains("Planned PVF lane for execution: `needs_planning_lane_assignment`")
        });
    let unresolved_selected_lane = text.contains("- \"needs_planning_lane_assignment\"")
        || text.contains("- needs_planning_lane_assignment");
    let unresolved_failure_policy = line_value_after_prefix(text, "failure_policy:")
        .as_deref()
        .map(|value| value == "fail_closed_until_validation_lane_is_selected")
        .unwrap_or_else(|| text.contains("fail_closed_until_validation_lane_is_selected"));
    const MARKERS: &[&str] = &["selected_lanes_inline: needs_planning_lane_assignment"];
    unresolved_planned_lane
        || unresolved_selected_lane
        || unresolved_failure_policy
        || MARKERS.iter().any(|marker| text.contains(marker))
        || has_truncation_sentinel_line(text)
}

fn has_truncation_sentinel_line(text: &str) -> bool {
    text.lines()
        .map(str::trim)
        .any(|line| matches!(line, "..." | "- ..." | "* ..." | "<...>"))
}

fn card_status_value(text: &str) -> Option<String> {
    line_value_after_prefix(text, "card_status:")
        .or_else(|| line_value_after_prefix(text, "Card Status:"))
}

fn design_time_card_status_allows_execution(text: &str) -> bool {
    match card_status_value(text).as_deref() {
        Some("ready" | "approved") => true,
        Some(_) => false,
        None => true,
    }
}

fn classify_srp_stage(repo_root: &Path, path: &Path) -> DoctorCardStageJson {
    let Some(text) = read_card_text(path) else {
        return missing_stage(repo_root, "SRP", path, "srp-editor");
    };
    let has_review_results = srp_has_final_review_results(&text);
    let has_pre_review_absence_exception = text.contains("pre-execution review results are absent");
    let has_policy_exception = (text.contains("explicit policy exception")
        || text.contains("review_results_exception:")
        || text.contains("policy_exception:"))
        && !has_pre_review_absence_exception;
    let pre_execution_review_state = srp_looks_pre_execution_review_state(&text);
    let legacy_policy_only = text.contains("# Structured Review Policy")
        || text.contains("artifact_type: \"structured_review_policy\"");
    let structured_review_prompt = text.contains("# Structured Review Prompt")
        && text.contains("artifact_type: \"structured_review_prompt\"");
    if card_status_value(&text).as_deref() == Some("completed")
        && !(has_review_results || (has_policy_exception && !pre_execution_review_state))
    {
        return card_stage(
            repo_root,
            "SRP",
            path,
            stage_truth(
                "active",
                false,
                false,
                Some("srp-editor"),
                "SRP card_status completed requires review findings, dispositions, or an explicit final policy exception.",
            ),
        );
    }
    if has_review_results || (has_policy_exception && !pre_execution_review_state) {
        return card_stage(
            repo_root,
            "SRP",
            path,
            stage_truth(
                "final",
                true,
                true,
                None,
                "SRP contains review results or an explicit policy exception for final review truth.",
            ),
        );
    }
    if structured_review_prompt && !legacy_policy_only {
        return card_stage(
            repo_root,
            "SRP",
            path,
            stage_truth(
                "pre_review",
                true,
                false,
                None,
                "SRP is a Structured Review Prompt ready for review; final review results are not recorded yet.",
            ),
        );
    }
    if legacy_policy_only {
        return card_stage(
            repo_root,
            "SRP",
            path,
            stage_truth(
                "legacy_compatible",
                false,
                false,
                Some("srp-editor"),
                "SRP validates as the legacy review-policy scaffold but is not final Structured Review Prompt truth.",
            ),
        );
    }
    card_stage(
        repo_root,
        "SRP",
        path,
        stage_truth(
            "active",
            false,
            false,
            Some("srp-editor"),
            "SRP exists but still needs review results or an explicit policy exception.",
        ),
    )
}

fn srp_looks_pre_execution_review_state(text: &str) -> bool {
    text.contains("pre-execution review results are absent")
        || text.contains("Review results are intentionally absent before implementation exists")
        || text.contains("Run the bounded issue review after implementation.")
        || text.contains("- Not run yet.")
        || text.contains("Not applicable until review runs.")
}

pub(super) fn validate_closed_completed_ready_bundle(
    repo_root: &Path,
    issue_ref: &IssueRef,
    root_bundle_input: &Path,
    root_bundle_output: &Path,
    bundle_paths: StructuredBundlePaths<'_>,
) -> Result<()> {
    let canonical_bundle_dir = issue_ref.task_bundle_dir_path(repo_root);
    let canonical_bundle_missing = !canonical_bundle_dir.is_dir()
        || !ensure_nonempty_file_path(root_bundle_input)?
        || !ensure_nonempty_file_path(root_bundle_output)?
        || !ensure_nonempty_file_path(bundle_paths.plan_path)?
        || !ensure_nonempty_file_path(bundle_paths.validation_plan_path)?
        || !ensure_nonempty_file_path(bundle_paths.review_policy_path)?;
    if canonical_bundle_missing {
        bail!(
            "doctor: closed issue #{} is missing canonical closeout bundle surfaces; run the explicit closeout normalization path instead of doctor --mode ready",
            issue_ref.issue_number()
        );
    }
    crate::cli::pr_cmd::lifecycle::ensure_closed_completed_issue_bundle_truth(
        repo_root,
        issue_ref,
        root_bundle_output,
    )
        .with_context(|| {
            format!(
                "doctor: closed issue #{} has stale canonical closeout truth; run the explicit closeout normalization path instead of doctor --mode ready",
                issue_ref.issue_number()
            )
        })?;
    validate_initialized_cards(
        issue_ref.issue_number(),
        issue_ref.slug(),
        root_bundle_input,
        root_bundle_output,
        repo_root,
        bundle_paths,
    )
}

fn srp_has_final_review_results(text: &str) -> bool {
    let Some(front_matter) = markdown_front_matter(text) else {
        return false;
    };
    if !front_matter.contains("review_results:") {
        return false;
    }
    let findings_status = line_value_after_prefix(front_matter, "findings_status:");
    let recommended_outcome = line_value_after_prefix(front_matter, "recommended_outcome:");
    matches_final_findings_status(findings_status.as_deref())
        && matches_final_recommended_outcome(recommended_outcome.as_deref())
}

fn markdown_front_matter(text: &str) -> Option<&str> {
    let rest = text.strip_prefix("---\n")?;
    let end = rest.find("\n---")?;
    Some(&rest[..end])
}

fn normalized_review_value(value: Option<&str>) -> Option<String> {
    value.map(|value| {
        value
            .trim()
            .trim_matches('"')
            .trim_matches('\'')
            .to_ascii_lowercase()
    })
}

fn matches_final_findings_status(value: Option<&str>) -> bool {
    matches!(
        normalized_review_value(value).as_deref(),
        Some("no_findings" | "findings_present")
    )
}

fn matches_final_recommended_outcome(value: Option<&str>) -> bool {
    matches!(
        normalized_review_value(value).as_deref(),
        Some("pass" | "block" | "needs_followup")
    )
}

fn classify_sor_stage(repo_root: &Path, path: &Path) -> DoctorCardStageJson {
    let Some(text) = read_card_text(path) else {
        return missing_stage(repo_root, "SOR", path, "sor-editor");
    };
    let status = line_value_after_prefix(&text, "Status:").unwrap_or_default();
    let integration_state =
        line_value_after_prefix(&text, "- Integration state:").unwrap_or_default();
    let result = line_value_after_prefix(&text, "- Result:").unwrap_or_default();
    let worktree_only =
        line_value_after_prefix(&text, "- Worktree-only paths remaining:").unwrap_or_default();
    let worktree_prune_result =
        line_value_after_prefix(&text, "- Worktree prune result:").unwrap_or_default();
    let terminal_worktree_truth = worktree_only == "none"
        || (worktree_only.starts_with("issue worktree retained: ")
            && worktree_prune_result.starts_with("retained_with_reason: "));
    let terminal_closeout = ["merged", "closed_no_pr"].contains(&integration_state.as_str())
        && matches!(
            (status.as_str(), result.as_str()),
            ("DONE", "PASS") | ("FAILED", "FAIL")
        )
        && terminal_worktree_truth
        && text.contains("## Validation");
    if card_status_value(&text).as_deref() == Some("completed") && !terminal_closeout {
        return card_stage(
            repo_root,
            "SOR",
            path,
            stage_truth(
                "active",
                false,
                false,
                Some("sor-editor"),
                "SOR card_status completed requires terminal closeout truth.",
            ),
        );
    }
    if status == "NOT_STARTED" || text.contains("No implementation has started yet") {
        return card_stage(
            repo_root,
            "SOR",
            path,
            stage_truth(
                "scaffold",
                false,
                false,
                Some("sor-editor"),
                "SOR is still the pre-execution output scaffold.",
            ),
        );
    }
    if terminal_closeout {
        return card_stage(
            repo_root,
            "SOR",
            path,
            stage_truth(
                "final",
                true,
                true,
                None,
                "SOR records terminal integration, validation, closeout, and artifact truth.",
            ),
        );
    }
    if integration_state == "pr_open" && status == "DONE" && result == "PASS" {
        return card_stage(
            repo_root,
            "SOR",
            path,
            stage_truth(
                "complete",
                true,
                false,
                Some("sor-editor"),
                "SOR is complete enough for PR publication but is not terminal closeout truth.",
            ),
        );
    }
    card_stage(
        repo_root,
        "SOR",
        path,
        stage_truth(
            "active",
            false,
            false,
            Some("sor-editor"),
            "SOR exists but does not yet satisfy PR publication or terminal closeout readiness.",
        ),
    )
}

fn missing_stage(
    repo_root: &Path,
    stage: &'static str,
    path: &Path,
    editor: &'static str,
) -> DoctorCardStageJson {
    card_stage(
        repo_root,
        stage,
        path,
        stage_truth(
            "missing",
            false,
            false,
            Some(editor),
            "Required lifecycle card is missing.",
        ),
    )
}

struct DoctorCardStageTruth {
    state: &'static str,
    complete: bool,
    final_ready: bool,
    next_editor: Option<&'static str>,
    detail: &'static str,
}

fn stage_truth(
    state: &'static str,
    complete: bool,
    final_ready: bool,
    next_editor: Option<&'static str>,
    detail: &'static str,
) -> DoctorCardStageTruth {
    DoctorCardStageTruth {
        state,
        complete,
        final_ready,
        next_editor,
        detail,
    }
}

fn card_stage(
    repo_root: &Path,
    stage: &'static str,
    path: &Path,
    truth: DoctorCardStageTruth,
) -> DoctorCardStageJson {
    DoctorCardStageJson {
        stage,
        path: path_relative_to_repo(repo_root, path),
        state: truth.state,
        complete: truth.complete,
        design_time_complete: matches!(stage, "SIP" | "STP" | "SPP" | "VPP" | "SRP")
            && truth.complete,
        final_ready: truth.final_ready,
        next_editor: truth.next_editor,
        detail: truth.detail.to_string(),
    }
}

fn read_card_text(path: &Path) -> Option<String> {
    fs::read_to_string(path).ok()
}

fn line_value_after_prefix(text: &str, prefix: &str) -> Option<String> {
    text.lines()
        .find_map(|line| line.trim().strip_prefix(prefix))
        .map(|value| value.trim().trim_matches('"').to_string())
}
