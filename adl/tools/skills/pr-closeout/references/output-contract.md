# Output Contract

```yaml
status: done | blocked | failed
target:
  issue_number: <u32 or null>
  pr_number: <u32 or null>
  branch: <branch or null>
  worktree_path: <path or null>
closeout_state:
  closure_outcome: merged | intentionally_closed | closed_no_pr | superseded | duplicate
  closure_references:
    - <issue_or_pr_url_or_identifier>
  issue_closed_verified: true | false
  shepherding_settled: true | false
  cards_finalized: true | false
  root_bundle_reconciled: true | false
  required_artifacts_in_canonical_repo_path: true | false
  worktree_pruned: true | false
findings:
  - severity: info | warning | blocking
    area: closure | cards | artifacts | worktree
    message: <short finding>
actions_taken:
  - <closeout action>
validation_performed:
  - <command or check>
handoff_state:
  next_phase: human_review | reporting_followup | blocked
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
