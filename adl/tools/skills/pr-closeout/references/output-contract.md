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
```
