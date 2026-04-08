# Output Contract

```yaml
status: done | blocked | failed
target:
  sip_path: <path>
edit_state:
  lifecycle_state_normalized: true | false
  placeholders_removed: true | false
  target_surfaces_tightened: true | false
findings:
  - severity: info | warning | blocking
    area: lifecycle | structure | validation | targets
    message: <short finding>
actions_taken:
  - <edit or normalization>
handoff_state:
  next_phase: qualitative_review | pr_ready | pr_run | blocked
```
