# Output Contract

```yaml
status: done | blocked | failed
target:
  spp_path: <path>
edit_state:
  planning_state_normalized: true | false
  codex_plan_statuses_validated: true | false
  placeholders_removed: true | false
findings:
  - severity: info | warning | blocking
    area: planning | codex_plan | dependencies | structure | review_hooks
    message: <short finding>
actions_taken:
  - <edit or normalization>
handoff_state:
  next_phase: qualitative_review | pr_ready | pr_run | blocked
```
