use super::*;

pub(super) fn print_doctor_preflight_text(preflight: &DoctorPreflightResult) {
    for line in doctor_preflight_lines(preflight) {
        println!("{line}");
    }
}

pub(super) fn doctor_preflight_lines(preflight: &DoctorPreflightResult) -> Vec<String> {
    let mut lines = vec![
        format!("OPEN_PR_SCAN_STATUS={}", preflight.open_pr_scan_status),
        format!(
            "OPEN_PR_COUNT={}",
            preflight
                .open_pr_count
                .map(|count| count.to_string())
                .unwrap_or_else(|| "unknown".to_string())
        ),
    ];
    for pr in &preflight.open_prs {
        lines.push(format!(
            "OPEN_PR=#{}|{}|{}|{}|{}",
            pr.number,
            pr.head_ref_name,
            pr.state,
            pr.queue.as_deref().unwrap_or("unknown"),
            pr.url
        ));
    }
    lines.extend([
        format!("PREFLIGHT={}", preflight.status),
        format!("PREFLIGHT_BLOCK_KIND={}", preflight.block_kind),
        format!("PREFLIGHT_GUIDANCE={}", preflight.guidance),
        format!("SESSION_LEDGER={}", preflight.session_ledger.status),
        format!(
            "SESSION_LEDGER_BLOCK_KIND={}",
            preflight.session_ledger.block_kind
        ),
        format!(
            "SESSION_LEDGER_GUIDANCE={}",
            preflight.session_ledger.guidance
        ),
        format!(
            "SESSION_LEDGER_PATH={}",
            preflight.session_ledger.ledger_path
        ),
        format!(
            "SESSION_LEDGER_CURRENT_SESSION={}",
            preflight
                .session_ledger
                .current_session_id
                .as_deref()
                .unwrap_or("none")
        ),
        format!(
            "SESSION_LEDGER_RELEVANT_CLAIM_COUNT={}",
            preflight.session_ledger.relevant_claim_count
        ),
    ]);
    for claim in &preflight.session_ledger.relevant_claims {
        lines.push(format!(
            "SESSION_LEDGER_CLAIM={}|{}|{}|self={}|issue={}|branch={}|worktree={}|resource={}:{}",
            claim.claim_id,
            claim.classification,
            claim.mode,
            claim.self_claim,
            claim.matches_issue,
            claim.matches_branch,
            claim.matches_worktree,
            claim.resource_kind,
            claim.resource_id
        ));
    }
    lines
}

pub(super) fn print_doctor_ready_text(ready: &DoctorReadyResult) {
    for line in doctor_ready_lines(ready) {
        println!("{line}");
    }
}

pub(super) fn doctor_ready_lines(ready: &DoctorReadyResult) -> Vec<String> {
    let mut lines = vec![
        format!("LIFECYCLE_STATE={}", ready.lifecycle_state),
        format!("SOURCE={}", ready.source),
        format!("ROOT_STP={}", ready.root_stp),
        format!("ROOT_INPUT={}", ready.root_input),
        format!("ROOT_OUTPUT={}", ready.root_output),
    ];
    if let Some(worktree) = &ready.worktree {
        lines.insert(1, format!("WORKTREE={worktree}"));
    }
    if let Some(wt_stp) = &ready.wt_stp {
        lines.push(format!("WT_STP={wt_stp}"));
    }
    if let Some(wt_input) = &ready.wt_input {
        lines.push(format!("WT_INPUT={wt_input}"));
    }
    if let Some(wt_output) = &ready.wt_output {
        lines.push(format!("WT_OUTPUT={wt_output}"));
    }
    lines.push(format!("READY={}", ready.status));
    lines
}

pub(super) fn print_doctor_card_lifecycle_text(card_lifecycle: &DoctorCardLifecycleJson) {
    for line in doctor_card_lifecycle_lines(card_lifecycle) {
        println!("{line}");
    }
}

pub(super) fn doctor_card_lifecycle_lines(card_lifecycle: &DoctorCardLifecycleJson) -> Vec<String> {
    let mut lines = vec![
        format!("CARD_LIFECYCLE_ORDER={}", card_lifecycle.order.join("->")),
        format!(
            "CARD_LIFECYCLE_ACTIVE_STAGE={}",
            card_lifecycle.active_stage
        ),
        format!(
            "CARD_LIFECYCLE_NEXT_REQUIRED_STAGE={}",
            card_lifecycle.next_required_stage.unwrap_or("none")
        ),
        format!(
            "CARD_LIFECYCLE_PR_RUN_READINESS={}",
            card_lifecycle.pr_run_readiness
        ),
        format!(
            "CARD_LIFECYCLE_PR_FINISH_READINESS={}",
            card_lifecycle.pr_finish_readiness
        ),
    ];
    for stage in &card_lifecycle.stages {
        lines.push(format!(
            "CARD_STAGE={}|{}|complete={}|design_time={}|final={}|editor={}|{}",
            stage.stage,
            stage.state,
            stage.complete,
            stage.design_time_complete,
            stage.final_ready,
            stage.next_editor.unwrap_or("none"),
            stage.path
        ));
    }
    lines
}
