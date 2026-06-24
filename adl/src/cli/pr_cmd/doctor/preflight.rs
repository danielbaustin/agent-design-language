use std::path::Path;

use super::*;
use adl::session_ledger::{load_target_claim_assessment, ClaimClassification, ClaimMode};

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
    let session_assessment = load_target_claim_assessment(
        repo_root,
        issue_ref.issue_number().into(),
        branch,
        &issue_ref.default_worktree_path(
            repo_root,
            std::env::var_os("ADL_WORKTREE_ROOT")
                .map(std::path::PathBuf::from)
                .as_deref(),
        ),
        std::env::var("CODEX_SESSION_ID").ok().as_deref(),
        chrono::Utc::now(),
    )?;
    let session_ledger = DoctorSessionLedgerJson {
        status: session_assessment.status,
        block_kind: session_assessment.block_kind,
        guidance: session_assessment.guidance,
        ledger_path: session_assessment.ledger_path,
        current_session_id: session_assessment.current_session_id,
        relevant_claim_count: session_assessment.relevant_claims.len(),
        relevant_claims: session_assessment
            .relevant_claims
            .into_iter()
            .map(|claim| DoctorSessionLedgerClaimJson {
                claim_id: claim.claim_id,
                session_id: claim.session_id,
                owner: claim.owner,
                resource_kind: claim.resource.kind,
                resource_id: claim.resource.id,
                mode: claim_mode_name(claim.mode),
                classification: claim_classification_name(claim.classification),
                issue: claim.issue,
                branch: claim.branch,
                worktree_path: claim.worktree_path,
                matches_issue: claim.matches_issue,
                matches_branch: claim.matches_branch,
                matches_worktree: claim.matches_worktree,
                self_claim: claim.self_claim,
            })
            .collect(),
    };
    let (status, block_kind, guidance) = doctor_preflight_status(
        open_prs.is_empty(),
        card_run_readiness,
        session_ledger.status,
    );
    if open_prs.is_empty() && card_run_readiness != Some("blocked") {
        Ok(DoctorPreflightResult {
            target_queue: target_queue.queue,
            target_queue_source: target_queue.source,
            open_pr_count: 0,
            open_prs,
            status,
            block_kind,
            guidance,
            session_ledger,
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
            session_ledger,
        })
    }
}

pub(super) fn doctor_preflight_status(
    open_pr_wave_empty: bool,
    card_run_readiness: Option<&'static str>,
    session_status: &'static str,
) -> (&'static str, &'static str, &'static str) {
    let card_blocked = card_run_readiness == Some("blocked");
    match (open_pr_wave_empty, card_blocked, session_status) {
        (true, false, "BLOCK") => (
            "BLOCK",
            "session_active_conflict",
            "Session-ledger ownership is actively claimed by another session. Resolve the claim before execution binding.",
        ),
        (true, false, "WARN") => (
            "WARN",
            "session_manual_inspection",
            "Session-ledger history needs manual inspection before execution, but there is no active ownership conflict.",
        ),
        (true, false, _) => (
            "PASS",
            "none",
            "No preflight queue or card-readiness blockers detected.",
        ),
        (false, false, _) => (
            "BLOCK",
            "open_pr_wave",
            "Issue-local readiness may proceed only under an explicit queue override such as --allow-open-pr-wave after recording why the open PR wave is unrelated or intentionally sequenced.",
        ),
        (true, true, _) => (
            "BLOCK",
            "card_run_readiness",
            "Repair issue-local SIP/STP/SPP/VPP/SRP/SOR readiness before execution; do not override this as queue pressure.",
        ),
        (false, true, _) => (
            "BLOCK",
            "open_pr_wave_and_card_run_readiness",
            "Repair issue-local card readiness before execution; open PR queue pressure remains a separate scheduling gate.",
        ),
    }
}

pub(super) fn claim_mode_name(mode: ClaimMode) -> &'static str {
    match mode {
        ClaimMode::Active => "active",
        ClaimMode::Watching => "watching",
        ClaimMode::Paused => "paused",
        ClaimMode::Stale => "stale",
        ClaimMode::Released => "released",
    }
}

pub(super) fn claim_classification_name(classification: ClaimClassification) -> &'static str {
    match classification {
        ClaimClassification::Active => "active",
        ClaimClassification::Watching => "watching",
        ClaimClassification::Paused => "paused",
        ClaimClassification::Stale => "stale",
        ClaimClassification::Released => "released",
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
