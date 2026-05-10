mod cleanup;
mod reconciliation;
mod transitions;

#[allow(unused_imports)]
pub(crate) use cleanup::{
    prune_issue_worktree, same_filesystem_target, scrub_noncanonical_issue_bundle_residue,
    sync_completed_output_surfaces, IssueWorktreePruneResult,
};
#[allow(unused_imports)]
pub(crate) use reconciliation::{
    closeout_closed_completed_issue_bundle, ensure_canonical_output_is_local_only,
    ensure_closed_completed_issue_bundle_truth, matching_task_bundle_dirs,
    normalize_closed_completed_output_card, normalize_closed_completed_sip,
    normalize_closed_completed_stp, reconcile_closed_completed_issue_bundle,
};
#[allow(unused_imports)]
pub(crate) use transitions::{
    ensure_issue_closed_completed_for_closeout, issue_is_closed_and_completed,
    wait_for_issue_closed_and_completed,
};

#[cfg(test)]
mod tests;
