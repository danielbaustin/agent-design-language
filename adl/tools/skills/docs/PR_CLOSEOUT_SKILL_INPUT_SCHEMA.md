# PR Closeout Skill Input Schema

```yaml
skill_input_schema: pr_closeout.v1
mode: closeout_issue | closeout_pr | closeout_worktree
repo_root: /absolute/path
target:
  issue_number: <u32>
  pr_number: <u32 or null>
  branch: <string or null>
  worktree_path: <absolute path or null>
  root_stp_path: <path or null>
  root_sip_path: <path or null>
  root_sor_path: <path or null>
  wt_stp_path: <path or null>
  wt_sip_path: <path or null>
  wt_sor_path: <path or null>
policy:
  closure_outcome: merged | intentionally_closed | closed_no_pr | superseded | duplicate
  closure_references:
    - <issue_or_pr_url_or_identifier>
  sync_root_bundle: true | false
  prune_worktree: true
  delete_local_branch: true | false
  stop_after_closeout: true
```

Mode requirements:

- `closeout_issue`
  - requires `target.issue_number`
- `closeout_pr`
  - requires `target.issue_number`
  - requires `target.pr_number`
- `closeout_worktree`
  - requires `target.issue_number`
  - requires `target.worktree_path`
