# Output Contract

```yaml
status: done | blocked | failed
target:
  issue_number: <u32 or null>
  branch: <branch or null>
  worktree_path: <path or null>
finish_state:
  output_record_present: true | false
  sor_finalized_before_pr_open: true | false
  tracked_review_surface_published: true | false
  staged_paths_validated: true | false
  pr_state: created | updated | blocked
findings:
  - severity: info | warning | blocking
    area: target | output_record | staging | validation | pr
    message: <short finding>
actions_taken:
  - <finish action>
validation_performed:
  - <command or check>
handoff_state:
  next_phase: pr_janitor | human_review | blocked
  ready_for_janitor: true | false
  issue_session_continues: true | false
  shepherding_goal:
    active: true | false
    target_pr_state: waiting_for_review | green_and_mergeable | merged_needs_closeout | blocked
    closeout_required_after_settlement: true | false
  lifecycle_shepherd:
    active: true | false
    state: pre_run | execution_bound | publication_ready | pr_waiting | janitor_active | merged_needs_closeout | closed_no_pr | settled | blocked
    owner_skill: workflow-conductor | pr-ready | pr-run | pr-finish | issue-watcher | pr-janitor | pr-closeout | human_review | none
    next_skill: pr-init | pr-ready | pr-run | pr-finish | issue-watcher | pr-janitor | pr-closeout | stp-editor | sip-editor | spp-editor | srp-editor | sor-editor | human_review | none
    closeout_required: true | false
    authority_boundary:
      merge_authority_human_only: true | false
      issue_close_authority_human_only: true | false
      review_authority_human_only: true | false
```

See `docs/tooling/ISSUE_LIFECYCLE_SHEPHERD_CONTRACT.md` for the canonical
shared state meanings.
