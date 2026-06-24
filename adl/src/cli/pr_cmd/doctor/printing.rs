use super::*;

pub(super) fn print_doctor_preflight_text(preflight: &DoctorPreflightResult) {
    println!("OPEN_PR_COUNT={}", preflight.open_pr_count);
    for pr in &preflight.open_prs {
        println!(
            "OPEN_PR=#{}|{}|{}|{}|{}",
            pr.number,
            pr.head_ref_name,
            pr.state,
            pr.queue.as_deref().unwrap_or("unknown"),
            pr.url
        );
    }
    println!("PREFLIGHT={}", preflight.status);
    println!("PREFLIGHT_BLOCK_KIND={}", preflight.block_kind);
    println!("PREFLIGHT_GUIDANCE={}", preflight.guidance);
    println!("SESSION_LEDGER={}", preflight.session_ledger.status);
    println!(
        "SESSION_LEDGER_BLOCK_KIND={}",
        preflight.session_ledger.block_kind
    );
    println!(
        "SESSION_LEDGER_GUIDANCE={}",
        preflight.session_ledger.guidance
    );
    println!(
        "SESSION_LEDGER_PATH={}",
        preflight.session_ledger.ledger_path
    );
    println!(
        "SESSION_LEDGER_CURRENT_SESSION={}",
        preflight
            .session_ledger
            .current_session_id
            .as_deref()
            .unwrap_or("none")
    );
    println!(
        "SESSION_LEDGER_RELEVANT_CLAIM_COUNT={}",
        preflight.session_ledger.relevant_claim_count
    );
    for claim in &preflight.session_ledger.relevant_claims {
        println!(
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
        );
    }
}

pub(super) fn print_doctor_ready_text(ready: &DoctorReadyResult) {
    println!("LIFECYCLE_STATE={}", ready.lifecycle_state);
    if let Some(worktree) = &ready.worktree {
        println!("WORKTREE={worktree}");
    }
    println!("SOURCE={}", ready.source);
    println!("ROOT_STP={}", ready.root_stp);
    println!("ROOT_INPUT={}", ready.root_input);
    println!("ROOT_OUTPUT={}", ready.root_output);
    if let Some(wt_stp) = &ready.wt_stp {
        println!("WT_STP={wt_stp}");
    }
    if let Some(wt_input) = &ready.wt_input {
        println!("WT_INPUT={wt_input}");
    }
    if let Some(wt_output) = &ready.wt_output {
        println!("WT_OUTPUT={wt_output}");
    }
    println!("READY={}", ready.status);
}

pub(super) fn print_doctor_card_lifecycle_text(card_lifecycle: &DoctorCardLifecycleJson) {
    println!("CARD_LIFECYCLE_ORDER={}", card_lifecycle.order.join("->"));
    println!(
        "CARD_LIFECYCLE_ACTIVE_STAGE={}",
        card_lifecycle.active_stage
    );
    println!(
        "CARD_LIFECYCLE_NEXT_REQUIRED_STAGE={}",
        card_lifecycle.next_required_stage.unwrap_or("none")
    );
    println!(
        "CARD_LIFECYCLE_PR_RUN_READINESS={}",
        card_lifecycle.pr_run_readiness
    );
    println!(
        "CARD_LIFECYCLE_PR_FINISH_READINESS={}",
        card_lifecycle.pr_finish_readiness
    );
    for stage in &card_lifecycle.stages {
        println!(
            "CARD_STAGE={}|{}|complete={}|design_time={}|final={}|editor={}|{}",
            stage.stage,
            stage.state,
            stage.complete,
            stage.design_time_complete,
            stage.final_ready,
            stage.next_editor.unwrap_or("none"),
            stage.path
        );
    }
}
