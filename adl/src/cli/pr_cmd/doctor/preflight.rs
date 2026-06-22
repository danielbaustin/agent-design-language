use std::path::Path;

use super::*;

pub(super) fn run_doctor_preflight(
    repo_root: &Path,
    repo: &str,
    version: &str,
    issue_ref: &IssueRef,
    branch: &str,
) -> Result<DoctorPreflightResult> {
    let source_path = resolve_doctor_issue_prompt_path(repo_root, issue_ref)?;
    let target_queue = resolve_issue_prompt_workflow_queue(&source_path)?;
    let unresolved =
        unresolved_milestone_pr_wave(repo, version, &target_queue.queue, Some(branch))?;
    let open_prs = unresolved
        .iter()
        .map(|pr| DoctorPreflightJsonPullRequest {
            number: pr.number,
            head_ref_name: pr.head_ref_name.clone(),
            state: if pr.is_draft { "draft" } else { "ready" },
            queue: pr.queue.clone(),
            url: pr.url.clone(),
        })
        .collect::<Vec<_>>();
    let card_run_readiness = preflight_card_run_readiness(repo_root, issue_ref);
    let (status, block_kind, guidance) =
        doctor_preflight_status(open_prs.is_empty(), card_run_readiness);
    if open_prs.is_empty() && card_run_readiness != Some("blocked") {
        Ok(DoctorPreflightResult {
            target_queue: target_queue.queue,
            target_queue_source: target_queue.source,
            open_pr_count: 0,
            open_prs,
            status,
            block_kind,
            guidance,
        })
    } else {
        Ok(DoctorPreflightResult {
            target_queue: target_queue.queue,
            target_queue_source: target_queue.source,
            open_pr_count: open_prs.len(),
            open_prs,
            status,
            block_kind,
            guidance,
        })
    }
}

pub(super) fn doctor_preflight_status(
    open_pr_wave_empty: bool,
    card_run_readiness: Option<&'static str>,
) -> (&'static str, &'static str, &'static str) {
    let card_blocked = card_run_readiness == Some("blocked");
    match (open_pr_wave_empty, card_blocked) {
        (true, false) => (
            "PASS",
            "none",
            "No preflight queue or card-readiness blockers detected.",
        ),
        (false, false) => (
            "BLOCK",
            "open_pr_wave",
            "Issue-local readiness may proceed only under an explicit queue override such as --allow-open-pr-wave after recording why the open PR wave is unrelated or intentionally sequenced.",
        ),
        (true, true) => (
            "BLOCK",
            "card_run_readiness",
            "Repair issue-local SIP/STP/SPP/VPP/SRP/SOR readiness before execution; do not override this as queue pressure.",
        ),
        (false, true) => (
            "BLOCK",
            "open_pr_wave_and_card_run_readiness",
            "Repair issue-local card readiness before execution; open PR queue pressure remains a separate scheduling gate.",
        ),
    }
}

pub(super) fn preflight_card_run_readiness(
    repo_root: &Path,
    issue_ref: &IssueRef,
) -> Option<&'static str> {
    let sip = issue_ref.task_bundle_input_path(repo_root);
    let stp = issue_ref.task_bundle_stp_path(repo_root);
    let spp = issue_ref.task_bundle_plan_path(repo_root);
    let vpp = issue_ref.task_bundle_validation_plan_path(repo_root);
    let srp = issue_ref.task_bundle_review_policy_path(repo_root);
    let sor = issue_ref.task_bundle_output_path(repo_root);
    if [&sip, &stp, &spp, &vpp, &srp, &sor]
        .iter()
        .any(|path| !path.is_file())
    {
        return Some("blocked");
    }
    Some(
        build_doctor_card_lifecycle(repo_root, &sip, &stp, &spp, &vpp, &srp, &sor).pr_run_readiness,
    )
}
