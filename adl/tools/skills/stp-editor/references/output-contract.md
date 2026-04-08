# Output Contract

```yaml
status: done | blocked | failed
target:
  stp_path: <path>
edit_state:
  placeholders_removed: true | false
  intent_preserved: true | false
  validation_plan_tightened: true | false
findings:
  - severity: info | warning | blocking
    area: structure | scope | validation | wording
    message: <short finding>
actions_taken:
  - <edit or normalization>
handoff_state:
  next_phase: qualitative_review | pr_ready | pr_run | blocked
```
