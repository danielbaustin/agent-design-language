# Output Contract

```yaml
status: done | blocked | failed
target:
  issue_number: <u32 or null>
  branch: <branch or null>
  worktree_path: <path or null>
finish_state:
  output_record_present: true | false
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
```
